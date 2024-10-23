use std::borrow::Cow;

use include_directory::include_directory;
use log::info;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{http::Response, WebViewBuilder};

static PROJECT_DIR: include_directory::Dir = include_directory!("$CARGO_MANIFEST_DIR/ui/dist");

pub fn gui(project: crate::core::project::Project) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("conduct")
        .build(&event_loop)
        .unwrap();

    let builder = WebViewBuilder::new()
        .with_url("conduct://home")
        .with_custom_protocol("conduct".to_string(), |a, b| {
            let mut path = b.uri().path();
            if path.eq("/") {
                path = "index.html"
            }
            info!("Requesting file: {}", path);

            for entry in PROJECT_DIR.entries().iter() {
                info!("Has entry: {}", entry.path().to_str().unwrap());
            }

            if PROJECT_DIR.contains(path) {
                let data = PROJECT_DIR.get_file(path);
                let str = data.unwrap().contents_utf8().unwrap();
                Response::builder()
                    .status(200)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(Cow::Owned(str.into()))
                    .unwrap()
            } else {
                Response::builder()
                    .status(404)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(Cow::Owned("Not found".into()))
                    .unwrap()
            }
        });

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
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
