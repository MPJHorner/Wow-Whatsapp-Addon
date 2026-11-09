#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
compile_error!("WoW Whisper currently targets macOS and Windows.");

mod options;
mod overlay;
mod policy;
mod resize;
mod snapshot;
mod theme;
mod ui;

use anyhow::{Context, Result};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use muda::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use std::time::{Duration, Instant};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Window, WindowBuilder},
};
use wry::{BackgroundThrottlingPolicy, NewWindowResponse, WebContext, WebView, WebViewBuilder};

const WHATSAPP_URL: &str = "https://web.whatsapp.com/";

#[derive(Debug)]
enum AppEvent {
    Toggle,
    Menu(MenuEvent),
    Control(ui::Action),
    Status(&'static str),
    PageLoaded,
    Inspection(String),
}

fn main() -> Result<()> {
    let Some(options) = options::Options::parse(std::env::args().skip(1))? else {
        return Ok(());
    };
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    #[cfg(target_os = "macos")]
    let event_loop = {
        use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
        let mut event_loop = event_loop;
        event_loop.set_activation_policy(ActivationPolicy::Accessory);
        event_loop.set_dock_visibility(false);
        event_loop.set_activate_ignoring_other_apps(!options.background);
        event_loop
    };
    let window_builder = WindowBuilder::new()
        .with_title("WoW Whisper")
        .with_inner_size(LogicalSize::new(860.0, 640.0))
        .with_min_inner_size(LogicalSize::new(resize::MIN_WIDTH, resize::MIN_HEIGHT))
        .with_resizable(true)
        .with_decorations(false)
        .with_visible(!options.background)
        .with_always_on_top(true)
        .with_visible_on_all_workspaces(true);
    #[cfg(target_os = "windows")]
    let window_builder = {
        use tao::platform::windows::WindowBuilderExtWindows;
        window_builder.with_skip_taskbar(true)
    };
    let window = window_builder
        .build(&event_loop)
        .context("Could not create the chat window")?;
    let menu = AppMenu::new(&window)?;
    let menu_proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = menu_proxy.send_event(AppEvent::Menu(event));
    }));

    let hotkey_proxy = event_loop.create_proxy();
    let hotkey_id = options.hotkey.id();
    GlobalHotKeyEvent::set_event_handler(Some(move |event: GlobalHotKeyEvent| {
        if event.id == hotkey_id && event.state == HotKeyState::Pressed {
            let _ = hotkey_proxy.send_event(AppEvent::Toggle);
        }
    }));
    let hotkey_manager = GlobalHotKeyManager::new().context("Could not create hotkey manager")?;
    hotkey_manager.register(options.hotkey).with_context(|| {
        format!("Could not register {}. Check whether WoW Whisper is already running, or use --hotkey Ctrl+Alt+W.", options.hotkey_label)
    })?;

    let control_proxy = event_loop.create_proxy();
    let chrome = WebViewBuilder::new()
        .with_html(ui::html(&options.hotkey_label))
        .with_bounds(ui::frame_bounds(&window))
        .with_focused(false)
        .with_accept_first_mouse(true)
        .with_incognito(true)
        .with_background_color((17, 19, 18, 255))
        .with_devtools(false)
        .with_navigation_handler(|url| url == "about:blank")
        .with_new_window_req_handler(|_, _| NewWindowResponse::Deny)
        .with_permission_handler(|_| wry::PermissionResponse::Deny)
        .with_download_started_handler(|_, _| false)
        .with_ipc_handler(move |request| {
            // This view only loads bundled, static HTML. Only these window actions
            // are accepted. The WhatsApp view below has no IPC handler at all.
            if let Some(action) = ui::Action::parse(request.body()) {
                let _ = control_proxy.send_event(AppEvent::Control(action));
            }
        })
        .build_as_child(&window)
        .context("Could not create the window frame")?;

    let profile = dirs::data_local_dir()
        .context("Could not find local application data")?
        .join("WoWWhisper")
        .join("webview");
    std::fs::create_dir_all(&profile).context("Could not create local browser profile")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&profile, std::fs::Permissions::from_mode(0o700))?;
    }
    let mut web_context = WebContext::new(Some(profile));
    let navigation_proxy = event_loop.create_proxy();
    let popup_proxy = event_loop.create_proxy();
    let download_proxy = event_loop.create_proxy();
    let load_proxy = event_loop.create_proxy();
    let builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_url(WHATSAPP_URL)
        .with_focused(false)
        .with_bounds(ui::chat_bounds(&window))
        .with_background_color((17, 27, 33, 255))
        .with_background_throttling(BackgroundThrottlingPolicy::Disabled)
        .with_initialization_script_for_main_only(theme::initialization_script(), true)
        .with_devtools(false)
        .with_general_autofill_enabled(false)
        .with_permission_handler(policy::permission_response)
        .with_navigation_handler(move |url| {
            let allowed = policy::is_whatsapp_navigation(&url);
            if !allowed {
                let _ = navigation_proxy
                    .send_event(AppEvent::Status("Link blocked · copy it into your browser"));
            }
            allowed
        })
        .with_new_window_req_handler(move |_, _| {
            let _ = popup_proxy
                .send_event(AppEvent::Status("Link blocked · copy it into your browser"));
            NewWindowResponse::Deny
        })
        .with_download_started_handler(move |_, _| {
            let _ = download_proxy.send_event(AppEvent::Status(
                "Download files using WhatsApp in your browser",
            ));
            false
        })
        .with_on_page_load_handler(move |event, _| {
            if matches!(event, wry::PageLoadEvent::Finished) {
                let _ = load_proxy.send_event(AppEvent::PageLoaded);
            }
        });
    #[cfg(target_os = "macos")]
    let builder = {
        use wry::WebViewBuilderExtDarwin;
        builder
            .with_data_store_identifier(*b"WoWWhisperData01")
            .with_allow_link_preview(false)
            // WKWebView uses WebKit but omits the Safari product token by default.
            .with_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15")
    };
    // The local presentation script installs the font/theme and labels existing
    // sender/time metadata. No IPC, custom protocols, or message-body extraction.
    let webview = builder.build_as_child(&window).context(
        "Could not create WhatsApp view. Windows requires Microsoft Edge WebView2 Runtime.",
    )?;
    webview.zoom(options.zoom)?;
    let mut zoom = options.zoom;
    let mut pinned = true;
    let mut manual_hidden = false;
    let mut next_focus_check = Instant::now() + Duration::from_secs(2);
    let mut resizer = resize::Controller::default();
    let mut status_expires = None;
    let mut diagnostics_path = options.ui_diagnostics.clone();
    let mut snapshot_path = options.ui_snapshot.clone();
    let mut inspection_due = None;
    let inspection_proxy = event_loop.create_proxy();
    let inspection_deadline = Instant::now() + Duration::from_secs(90);
    let mut theme_modified = None;
    let mut next_theme_check = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let _resources = (&hotkey_manager, &web_context, &menu.root);
        if Instant::now() >= next_theme_check {
            next_theme_check = Instant::now() + Duration::from_secs(1);
            if let Some(path) = &options.theme_file
                && let Ok(modified) =
                    std::fs::metadata(path).and_then(|metadata| metadata.modified())
                && theme_modified != Some(modified)
                && let Ok(css) = std::fs::read_to_string(path)
            {
                report(webview.evaluate_script(&theme::stylesheet_update(&css)));
                theme_modified = Some(modified);
                diagnostics_path = options.ui_diagnostics.clone();
                snapshot_path = options.ui_snapshot.clone();
                inspection_due = Some(Instant::now() + Duration::from_secs(4));
            }
        }
        if inspection_due.is_some_and(|deadline| Instant::now() >= deadline) {
            inspection_due = None;
            let proxy = inspection_proxy.clone();
            report(webview.evaluate_script_with_callback(
                include_str!("../assets/inspect-ui.js"),
                move |data| {
                    let _ = proxy.send_event(AppEvent::Inspection(data));
                },
            ));
        }
        if resizer.tick(&window) {
            report(chrome.set_bounds(ui::frame_bounds(&window)));
            report(webview.set_bounds(ui::chat_bounds(&window)));
        }
        if status_expires.is_some_and(|deadline| Instant::now() >= deadline) {
            ui::set_status(&chrome, "");
            status_expires = None;
        }
        if Instant::now() >= next_focus_check {
            let should_show = overlay::should_show(manual_hidden, overlay::foreground());
            if should_show && !window.is_visible() {
                overlay::show_without_focus(&window);
            } else if !should_show && window.is_visible() {
                window.set_visible(false);
            }
            next_focus_check = Instant::now() + Duration::from_millis(400);
        }
        *control_flow = ControlFlow::WaitUntil(if resizer.active() {
            next_focus_check.min(Instant::now() + Duration::from_millis(16))
        } else {
            next_focus_check
        });
        let action = match event {
            Event::UserEvent(AppEvent::Toggle) => Some(ui::Action::Toggle),
            Event::UserEvent(AppEvent::Control(action)) => Some(action),
            Event::UserEvent(AppEvent::Menu(event)) => ui::Action::parse(event.id.as_ref()),
            Event::UserEvent(AppEvent::Status(status)) => {
                ui::set_status(&chrome, status);
                status_expires = Some(Instant::now() + Duration::from_secs(5));
                None
            }
            Event::UserEvent(AppEvent::PageLoaded) => {
                ui::set_status(&chrome, "");
                if diagnostics_path.is_some() || snapshot_path.is_some() {
                    inspection_due = Some(Instant::now() + Duration::from_secs(8));
                }
                None
            }
            Event::UserEvent(AppEvent::Inspection(data)) => {
                let value = serde_json::from_str::<serde_json::Value>(&data).unwrap_or_default();
                if value["ready"] != true && Instant::now() < inspection_deadline {
                    inspection_due = Some(Instant::now() + Duration::from_secs(2));
                } else {
                    if let Some(path) = diagnostics_path.take()
                        && let Ok(json) = serde_json::to_vec_pretty(&value)
                    {
                        let _ = std::fs::write(path, json);
                    }
                    if let Some(directory) = snapshot_path.take()
                        && std::fs::create_dir_all(&directory).is_ok()
                    {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let _ = std::fs::set_permissions(
                                &directory,
                                std::fs::Permissions::from_mode(0o700),
                            );
                        }
                        snapshot::capture(&chrome, directory.join("frame.png"));
                        snapshot::capture(&webview, directory.join("chat.png"));
                    }
                }
                None
            }
            Event::Reopen { .. } => {
                manual_hidden = false;
                show(&window, &webview);
                None
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => Some(ui::Action::Quit),
            Event::WindowEvent {
                event: WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. },
                ..
            } => {
                report(chrome.set_bounds(ui::frame_bounds(&window)));
                report(webview.set_bounds(ui::chat_bounds(&window)));
                None
            }
            _ => None,
        };
        match action {
            Some(ui::Action::Toggle) => {
                if window.is_visible() && !window.is_minimized() {
                    manual_hidden = true;
                    window.set_visible(false);
                } else {
                    manual_hidden = false;
                    show(&window, &webview);
                }
            }
            Some(ui::Action::Pin) => {
                pinned = !pinned;
                window.set_always_on_top(pinned);
                menu.pin.set_checked(pinned);
                ui::set_pinned(&chrome, pinned);
            }
            Some(ui::Action::Reload) => {
                ui::set_status(&chrome, "Reconnecting to WhatsApp Web…");
                report(webview.reload());
            }
            Some(ui::Action::ZoomIn) => {
                zoom = (zoom + 0.1).min(1.5);
                report(webview.zoom(zoom));
            }
            Some(ui::Action::ZoomOut) => {
                zoom = (zoom - 0.1).max(0.6);
                report(webview.zoom(zoom));
            }
            Some(ui::Action::Drag) => {
                let _ = window.drag_window();
            }
            Some(ui::Action::StartResize) => {
                resizer.begin(&window);
                *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16));
            }
            Some(ui::Action::Quit) => *control_flow = ControlFlow::Exit,
            None => {}
        }
    })
}

fn show(window: &Window, webview: &WebView) {
    window.set_minimized(false);
    window.set_visible(true);
    window.set_focus();
    report(webview.focus());
}

fn report(result: wry::Result<()>) {
    if result.is_err() {
        // Do not log URLs, message content, or session information.
        eprintln!("The browser could not complete the requested window action.");
    }
}

struct AppMenu {
    root: Menu,
    pin: CheckMenuItem,
}

impl AppMenu {
    fn new(window: &Window) -> Result<Self> {
        let root = Menu::new();
        let app = Submenu::new("WoW Whisper", true);
        let pin = CheckMenuItem::with_id("pin", "Keep above other windows", true, true, None);
        app.append_items(&[
            &MenuItem::with_id("toggle", "Show / Hide", true, None),
            &pin,
            &MenuItem::with_id(
                "reload",
                "Reload WhatsApp",
                true,
                shortcut(muda::accelerator::Code::KeyR),
            ),
            &PredefinedMenuItem::separator(),
            &MenuItem::with_id(
                "quit",
                "Quit WoW Whisper",
                true,
                shortcut(muda::accelerator::Code::KeyQ),
            ),
        ])?;
        let edit = Submenu::with_items(
            "Edit",
            true,
            &[
                &PredefinedMenuItem::undo(None),
                &PredefinedMenuItem::redo(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::cut(None),
                &PredefinedMenuItem::copy(None),
                &PredefinedMenuItem::paste(None),
                &PredefinedMenuItem::select_all(None),
            ],
        )?;
        root.append_items(&[&app, &edit])?;
        #[cfg(target_os = "macos")]
        {
            let _ = window;
            root.init_for_nsapp();
        }
        #[cfg(target_os = "windows")]
        {
            // All window controls live in the custom frame on Windows; no menu bar.
            let _ = window;
        }
        Ok(Self { root, pin })
    }
}

fn shortcut(code: muda::accelerator::Code) -> Option<muda::accelerator::Accelerator> {
    #[cfg(target_os = "macos")]
    {
        Some(muda::accelerator::Accelerator::new(
            muda::accelerator::Modifiers::META,
            code,
        ))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = code;
        None
    }
}
