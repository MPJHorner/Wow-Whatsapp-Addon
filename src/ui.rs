use tao::{
    dpi::{LogicalPosition, LogicalSize},
    window::Window,
};
use wry::{Rect, WebView};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Toggle,
    Pin,
    Reload,
    ZoomIn,
    ZoomOut,
    Quit,
    Drag,
    StartResize,
}

impl Action {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "toggle" => Some(Self::Toggle),
            "pin" => Some(Self::Pin),
            "reload" => Some(Self::Reload),
            "zoom-in" => Some(Self::ZoomIn),
            "zoom-out" => Some(Self::ZoomOut),
            "quit" => Some(Self::Quit),
            "drag" => Some(Self::Drag),
            "resize-start" => Some(Self::StartResize),
            _ => None,
        }
    }
}

pub fn html(hotkey: &str) -> String {
    let escaped = hotkey
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;");
    crate::theme::with_textures(include_str!("../assets/frame.html"))
        .replace("{{HOTKEY}}", &escaped)
}

pub fn chat_bounds(window: &Window) -> Rect {
    let size = window.inner_size().to_logical::<f64>(window.scale_factor());
    Rect {
        position: LogicalPosition::new(20.0, 54.0).into(),
        size: LogicalSize::new((size.width - 40.0).max(1.0), (size.height - 84.0).max(1.0)).into(),
    }
}

pub fn frame_bounds(window: &Window) -> Rect {
    Rect {
        position: LogicalPosition::new(0.0, 0.0).into(),
        size: window
            .inner_size()
            .to_logical::<f64>(window.scale_factor())
            .into(),
    }
}

pub fn set_status(chrome: &WebView, status: &'static str) {
    // Status strings are fixed, compile-time application messages, never chat data.
    let text = status.replace('\\', "\\\\").replace('\'', "\\'");
    let _ = chrome.evaluate_script(&format!(
        "document.getElementById('status').textContent = '{text}'"
    ));
}

pub fn set_pinned(chrome: &WebView, pinned: bool) {
    let _ = chrome.evaluate_script(&format!(
        "document.getElementById('pin').setAttribute('aria-pressed', '{pinned}')"
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_commands_reject_unknown_actions_and_invalid_dimensions() {
        for command in [
            "execute:hello",
            "open:https://example.com",
            "resize:NaN,640",
            "resize:740,inf",
            "resize:740",
        ] {
            assert!(Action::parse(command).is_none(), "{command}");
        }
        assert!(Action::parse("resize:1,999999").is_none());
        assert!(matches!(
            Action::parse("resize-start"),
            Some(Action::StartResize)
        ));
    }
}
