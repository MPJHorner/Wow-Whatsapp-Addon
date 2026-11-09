use tao::window::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foreground {
    Game,
    Panel,
    Other,
}

pub fn should_show(manual_hidden: bool, foreground: Foreground) -> bool {
    !manual_hidden && matches!(foreground, Foreground::Game | Foreground::Panel)
}

#[cfg(target_os = "macos")]
pub fn foreground() -> Foreground {
    use objc2_app_kit::NSWorkspace;
    let Some(app) = NSWorkspace::sharedWorkspace().frontmostApplication() else {
        return Foreground::Other;
    };
    if app.processIdentifier() as u32 == std::process::id() {
        return Foreground::Panel;
    }
    let id = app
        .bundleIdentifier()
        .map(|id| id.to_string().to_ascii_lowercase());
    if id.as_deref().is_some_and(|id| {
        id == "com.blizzard.worldofwarcraft" || id.starts_with("com.blizzard.worldofwarcraft.")
    }) {
        Foreground::Game
    } else {
        Foreground::Other
    }
}

#[cfg(target_os = "windows")]
pub fn foreground() -> Foreground {
    use windows_sys::Win32::{
        Foundation::CloseHandle,
        System::Threading::{
            OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
        },
        UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
    };
    // Read only the foreground application's process identity. No game memory,
    // input hooks, process injection, or elevated access is involved.
    unsafe {
        let mut pid = 0;
        GetWindowThreadProcessId(GetForegroundWindow(), &mut pid);
        if pid == std::process::id() {
            return Foreground::Panel;
        }
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if process.is_null() {
            return Foreground::Other;
        }
        let mut path = [0_u16; 32768];
        let mut length = path.len() as u32;
        let ok = QueryFullProcessImageNameW(process, 0, path.as_mut_ptr(), &mut length);
        CloseHandle(process);
        if ok == 0 {
            return Foreground::Other;
        }
        let path = String::from_utf16_lossy(&path[..length as usize]);
        let name = path.rsplit('\\').next().unwrap_or("").to_ascii_lowercase();
        if matches!(
            name.as_str(),
            "wow.exe" | "wowclassic.exe" | "wowt.exe" | "wowb.exe" | "wowclassic_t.exe"
        ) {
            Foreground::Game
        } else {
            Foreground::Other
        }
    }
}

#[cfg(target_os = "macos")]
pub fn show_without_focus(window: &Window) {
    use objc2_app_kit::NSWindow;
    use tao::platform::macos::WindowExtMacOS;
    // SAFETY: Tao owns this live NSWindow; this runs on its main thread.
    // Tao's set_visible(true) makes the window key, so use orderFrontRegardless
    // specifically to leave the player's keyboard focus in WoW.
    unsafe {
        (&*window.ns_window().cast::<NSWindow>()).orderFrontRegardless();
    }
}

#[cfg(target_os = "windows")]
pub fn show_without_focus(window: &Window) {
    use tao::platform::windows::WindowExtWindows;
    use windows_sys::Win32::UI::WindowsAndMessaging::{SW_SHOWNOACTIVATE, ShowWindow};
    // SAFETY: Tao owns this live HWND on the current thread.
    unsafe {
        ShowWindow(window.hwnd() as _, SW_SHOWNOACTIVATE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn visibility_follows_the_game_but_respects_manual_hide() {
        assert!(should_show(false, Foreground::Game));
        assert!(should_show(false, Foreground::Panel));
        assert!(!should_show(false, Foreground::Other));
        for context in [Foreground::Game, Foreground::Panel, Foreground::Other] {
            assert!(!should_show(true, context));
        }
    }
}
