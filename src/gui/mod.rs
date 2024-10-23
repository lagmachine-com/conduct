mod api;
mod embedded_files;
mod router;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wry::WebViewBuilder;

fn get_url() -> String {
    #[cfg(any(target_os = "windows", target_os = "android"))]
    return "http://conduct.base".to_string();

    #[cfg(not(any(target_os = "windows", target_os = "android")))]
    return "conduct://base".to_string();
}

pub fn gui(_project: crate::core::project::Project) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("conduct")
        .build(&event_loop)
        .unwrap();

    let builder = WebViewBuilder::new()
        .with_url(get_url())
        .with_custom_protocol("conduct".to_string(), router::route);

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
