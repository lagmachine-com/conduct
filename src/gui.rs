mod api_result;
mod embedded_files;
mod router;
mod routes;

use std::{
    clone,
    sync::{Arc, RwLock},
};

use log::{debug, info, trace, warn, Log};
use router::{ApiEntry, RequestContext};
use routes::register_routes;
use serde_json::json;
use tao::{
    dpi::{LogicalSize, PhysicalPosition, Position, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

use wry::{WebView, WebViewBuilder};

use crate::{core::commands::DialogOptions, utils};

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
    let str = include_str!("../ui/api.js").to_string();
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

    let mut always_on_top = false;
    let mut closable = true;
    let mut minimizable = true;
    let mut maximizable = true;

    let mut webview_ref: Arc<RwLock<Option<WebView>>> = Arc::new(RwLock::new(None));

    if let Some(options) = dialog_options {
        page += options.path.as_str();
        title = options.title;
        size.height = options.height;
        size.width = options.width;
        always_on_top = true;
        closable = false;
        minimizable = false;
        maximizable = false
    }

    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(Size::Logical(size))
        .with_always_on_top(always_on_top)
        .with_closable(closable)
        .with_minimizable(minimizable)
        .with_maximizable(maximizable)
        .build(&event_loop)
        .unwrap();

    info!("Starting ui with page: {}", page);

    let size = window.current_monitor().unwrap().size();
    let window_size = window.outer_size();

    window.set_outer_position(Position::Physical(PhysicalPosition::new(
        i32::try_from((size.width / 2) - (window_size.width / 2)).unwrap(),
        i32::try_from((size.height / 2) - (window_size.height / 2) - (size.height / 16)).unwrap(),
    )));

    let cloned = webview_ref.clone();

    let builder = WebViewBuilder::new()
        .with_url(page)
        .with_devtools(true)
        .with_initialization_script(get_init_script().as_str())
        .with_drag_drop_handler(move |e| drag_drop_handler(webview_ref.clone(), e))
        .with_asynchronous_custom_protocol(
            "conduct".into(),
            move |_webview_id, request, responder| {
                let project = project.clone();

                let context = RequestContext {
                    project: project.clone(),
                };

                let result = router::route(request, router.clone(), context, responder);

                if let Some(result) = result {
                    trace!("Received control flow result from api: {:?}", result);
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

    {
        let mut view = cloned.write().unwrap();
        *view = Some(_webview);
        drop(view);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                event => {}
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

fn drag_drop_handler(webview_ref: Arc<RwLock<Option<WebView>>>, event: wry::DragDropEvent) -> bool {
    match event {
        wry::DragDropEvent::Drop { paths, position } => {
            info!("Received drop event");

            match webview_ref.clone().read() {
                Ok(webview) => {
                    info!("Handling drop!");
                    if webview.is_some() {
                        info!("Posting event");

                        let mut result = vec![];

                        for path in paths.iter() {
                            let mime = utils::mime::mime_from_file_path(path);

                            result.push(json!({
                                "path": path,
                                "mime": mime
                            }));
                        }

                        let _ = webview.as_ref().unwrap().evaluate_script(
                            &format!(
                                "window.postMessage({});",
                                json!({
                                    "type": "drag_drop_dropped",
                                    "data": result
                                })
                            )
                            .to_string(),
                        );
                    } else {
                        info!("Failed to get webview")
                    }

                    info!("Done handling drop");
                }
                Err(_) => warn!("Failed to read"),
            }

            info!("Done handling drop event");
            return true;
        }
        wry::DragDropEvent::Enter { paths, position } => match webview_ref.clone().read() {
            Ok(webview) => {
                info!("Handling Drag Drop Enter!");
                if webview.is_some() {
                    let _ = webview.as_ref().unwrap().evaluate_script(
                        &format!(
                            "window.postMessage({});",
                            json!({
                                "type": "drag_drop_enter",
                                "data": paths
                            })
                        )
                        .to_string(),
                    );
                } else {
                    warn!("Failed to get webview")
                }
            }
            Err(_) => warn!("Failed to read"),
        },
        wry::DragDropEvent::Leave => match webview_ref.clone().read() {
            Ok(webview) => {
                info!("Handling Drag Drop Leave!");
                if webview.is_some() {
                    let _ = webview.as_ref().unwrap().evaluate_script(
                        &format!(
                            "window.postMessage({});",
                            json!({
                                "type": "drag_drop_leave"
                            })
                        )
                        .to_string(),
                    );
                } else {
                    warn!("Failed to get webview")
                }
            }
            Err(_) => warn!("Failed to read"),
        },
        wry::DragDropEvent::Over { position } => (),
        _ => {
            info!("Some other event :o");
        }
    }
    true
}
