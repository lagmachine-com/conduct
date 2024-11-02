mod api_result;
mod embedded_files;
mod router;
mod routes;

use std::sync::{Arc, RwLock};

use log::{debug, info};
use router::{ApiEntry, RequestContext};
use routes::register_routes;
use tao::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

use wry::WebViewBuilder;

use crate::core::commands::DialogOptions;

enum UserWindowEvent {
    Exit,
}

fn get_custom_protocol_url() -> String {
    #[cfg(any(target_os = "windows", target_os = "android"))]
    return "http://conduct.base".to_string();

    #[cfg(not(any(target_os = "windows", target_os = "android")))]
    return "conduct://base".to_string();
}

#[allow(unreachable_code)]
fn get_homepage_url() -> String {
    #[cfg(debug_assertions)]
    return "http://localhost:3000".to_string();

    return get_custom_protocol_url();
}

fn get_init_script() -> String {
    let str = include_str!("../../ui/api.js").to_string();
    let base = get_custom_protocol_url();

    let result = str.replace("${BASE_PATH}", &base);
    return result;
}

pub fn gui(project: crate::core::project::Project, dialog_options: Option<DialogOptions>) {
    let event_loop: EventLoop<UserWindowEvent> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let project = Arc::new(RwLock::new(project.clone()));

    let router = Arc::new(RwLock::new(matchit::Router::<ApiEntry>::new()));
    register_routes(router.write().as_mut().unwrap());

    let mut page = get_homepage_url();
    let mut title = "conduct".to_string();

    let mut size = LogicalSize::<f64>::new(1280.0, 720.0);

    if let Some(options) = dialog_options {
        page += options.path.as_str();
        title = options.title;
        size.height = options.height;
        size.width = options.width;
    }

    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(Size::Logical(size))
        .build(&event_loop)
        .unwrap();

    info!("Starting ui with page: {}", page);

    let builder = WebViewBuilder::new()
        .with_url(page)
        .with_devtools(true)
        .with_initialization_script(get_init_script().as_str())
        .with_asynchronous_custom_protocol(
            "conduct".into(),
            move |_webview_id, request, responder| {
                let project = project.clone();

                let context = RequestContext {
                    project: project.clone(),
                };

                let result = router::route(request, router.clone(), context, responder);

                if let Some(result) = result {
                    debug!("Received control flow result from api: {:?}", result);
                    match result {
                        router::ApiControlFlowResult::Close => {
                            let _ = proxy.send_event(UserWindowEvent::Exit);
                        }
                    };
                };
            },
        );

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window).unwrap();
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            Event::UserEvent(event) => match event {
                UserWindowEvent::Exit => {
                    *control_flow = ControlFlow::Exit;
                }
            },
            _ => (),
        }
    });
}
