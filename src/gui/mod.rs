mod api_result;
mod embedded_files;
mod router;
mod routes;

use std::sync::{Arc, Mutex};

use log::info;
use router::RequestContext;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wry::WebViewBuilder;

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

pub fn gui(project: crate::core::project::Project) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("conduct")
        .build(&event_loop)
        .unwrap();

    let project = Arc::new(Mutex::new(project.clone()));

    let builder = WebViewBuilder::new()
        .with_url(get_homepage_url())
        .with_devtools(true)
        .with_initialization_script(get_init_script().as_str())
        .with_asynchronous_custom_protocol(
            "conduct".into(),
            move |_webview_id, request, responder| {
                // here you can use a tokio task, thread pool or anything
                // to do heavy computation to resolve your request
                // e.g. downloading files, opening the camera...

                let project = project.clone();

                std::thread::spawn(move || {
                    let context = RequestContext {
                        project: project.clone(),
                    };

                    let response = router::route(request, context);
                    responder.respond(response);
                });
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

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
