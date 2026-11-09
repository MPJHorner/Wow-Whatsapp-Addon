//! Checks the real Cocoa/WebKit resize path without displaying or focusing a
//! window, registering a hotkey, accessing an account, or generating OS input.
#[allow(dead_code)]
#[path = "../src/resize.rs"]
mod resize;
#[allow(dead_code)]
#[path = "../src/theme.rs"]
mod theme;
#[allow(dead_code)]
#[path = "../src/ui.rs"]
mod ui;

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("Native window check requires macOS.");
}

#[cfg(target_os = "macos")]
fn main() -> anyhow::Result<()> {
    use objc2_app_kit::NSWindow;
    use tao::{
        dpi::LogicalSize,
        event::Event,
        event_loop::{ControlFlow, EventLoopBuilder},
        platform::macos::{ActivationPolicy, EventLoopExtMacOS, WindowExtMacOS},
        window::WindowBuilder,
    };
    use wry::WebViewBuilder;
    let mut event_loop = EventLoopBuilder::<String>::with_user_event().build();
    event_loop.set_activation_policy(ActivationPolicy::Accessory);
    event_loop.set_dock_visibility(false);
    event_loop.set_activate_ignoring_other_apps(false);
    let window = WindowBuilder::new()
        .with_title("Whisper hidden UI check")
        .with_visible(false)
        .with_decorations(false)
        .with_inner_size(LogicalSize::new(860.0, 640.0))
        .build(&event_loop)?;
    let proxy = event_loop.create_proxy();
    let html = ui::html("Ctrl+Shift+W").replace(
        "</body>",
        "<script>document.fonts.ready.then(() => window.ipc.postMessage('ready'))</script></body>",
    );
    let chrome = WebViewBuilder::new()
        .with_html(html)
        .with_focused(false)
        .with_bounds(ui::frame_bounds(&window))
        .with_incognito(true)
        .with_ipc_handler(move |request| {
            let _ = proxy.send_event(request.body().clone());
        })
        .build_as_child(&window)?;
    let child = WebViewBuilder::new()
        .with_html("<!doctype html><title>Offline fixture</title>")
        .with_focused(false)
        .with_incognito(true)
        .with_bounds(ui::chat_bounds(&window))
        .build_as_child(&window)?;
    let result_proxy = event_loop.create_proxy();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::WaitUntil(deadline);
        assert!(std::time::Instant::now() < deadline, "Hidden WebKit check timed out");
        if let Event::UserEvent(value) = event {
            if value == "ready" {
                // SAFETY: Main-thread, live NSWindow owned by Tao.
                let original = unsafe { (&*window.ns_window().cast::<NSWindow>()).frame() };
                assert!(resize::set_frame(&window, (original.origin.x, original.origin.y + original.size.height - 720.0, 980.0, 720.0)));
                let size = window.inner_size().to_logical::<f64>(window.scale_factor());
                assert_eq!((size.width, size.height), (980.0, 720.0));
                let actual = unsafe { (&*window.ns_window().cast::<NSWindow>()).frame() };
                assert_eq!(actual.origin.y + actual.size.height, original.origin.y + original.size.height);
                child.set_bounds(ui::chat_bounds(&window)).unwrap();
                chrome.set_bounds(ui::frame_bounds(&window)).unwrap();
                let child_size = child.bounds().unwrap().size.to_logical::<f64>(window.scale_factor());
                assert_eq!((child_size.width, child_size.height), (940.0, 636.0));
                let proxy = result_proxy.clone();
                chrome.evaluate_script_with_callback(r#"JSON.stringify({width:innerWidth,height:innerHeight,font:[...document.fonts].some(f=>f.family.replaceAll('"','')==='WoW UI'&&f.status==='loaded'),grip:(()=>{let r=document.getElementById('resize').getBoundingClientRect();let e=document.elementFromPoint(r.x+r.width/2,r.y+r.height/2);return !!e.closest('#resize')})()})"#, move |json| { let _ = proxy.send_event(json); }).unwrap();
            } else {
                let serialized: String = serde_json::from_str(&value).unwrap();
                let result: serde_json::Value = serde_json::from_str(&serialized).unwrap();
                assert_eq!(result["width"], 980);
                assert_eq!(result["height"], 720);
                assert_eq!(result["font"], true);
                assert_eq!(result["grip"], true);
                println!("Hidden Cocoa/WebKit check passed: native resize, fixed top-left, both view sizes, embedded font, reachable grip.");
                *flow = ControlFlow::Exit;
            }
        }
    })
}
