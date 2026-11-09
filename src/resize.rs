use tao::window::Window;

pub const MIN_WIDTH: f64 = 700.0;
pub const MIN_HEIGHT: f64 = 480.0;

#[cfg(any(target_os = "macos", test))]
#[derive(Clone, Copy)]
struct DragStart {
    pointer: (f64, f64),
    origin: (f64, f64),
    size: (f64, f64),
}

#[cfg(any(target_os = "macos", test))]
impl DragStart {
    fn frame_at(self, pointer: (f64, f64)) -> (f64, f64, f64, f64) {
        // AppKit screen coordinates point up. Preserve the top-left corner.
        let width = (self.size.0 + pointer.0 - self.pointer.0).clamp(MIN_WIDTH, 2000.0);
        let height = (self.size.1 + self.pointer.1 - pointer.1).clamp(MIN_HEIGHT, 1600.0);
        (
            self.origin.0,
            self.origin.1 + self.size.1 - height,
            width,
            height,
        )
    }
}

#[derive(Default)]
pub struct Controller {
    #[cfg(target_os = "macos")]
    start: Option<DragStart>,
}

impl Controller {
    pub fn begin(&mut self, window: &Window) {
        #[cfg(target_os = "macos")]
        {
            use objc2_app_kit::{NSEvent, NSWindow};
            use tao::platform::macos::WindowExtMacOS;
            if NSEvent::pressedMouseButtons() & 1 == 0 {
                return;
            }
            let point = NSEvent::mouseLocation();
            // SAFETY: Tao owns this live NSWindow, on the main thread.
            let frame = unsafe { (&*window.ns_window().cast::<NSWindow>()).frame() };
            self.start = Some(DragStart {
                pointer: (point.x, point.y),
                origin: (frame.origin.x, frame.origin.y),
                size: (frame.size.width, frame.size.height),
            });
        }
        #[cfg(target_os = "windows")]
        {
            let _ = window.drag_resize_window(tao::window::ResizeDirection::SouthEast);
        }
    }

    pub fn active(&self) -> bool {
        #[cfg(target_os = "macos")]
        return self.start.is_some();
        #[cfg(target_os = "windows")]
        false
    }

    pub fn tick(&mut self, window: &Window) -> bool {
        #[cfg(target_os = "macos")]
        {
            use objc2_app_kit::NSEvent;
            let Some(start) = self.start else {
                return false;
            };
            // This is only polled during a drag explicitly started on our grip.
            // No input hooks, Accessibility permission, or game interaction.
            if NSEvent::pressedMouseButtons() & 1 == 0 {
                self.start = None;
                return false;
            }
            let point = NSEvent::mouseLocation();
            let (x, y, width, height) = start.frame_at((point.x, point.y));
            set_frame(window, (x, y, width, height))
        }
        #[cfg(target_os = "windows")]
        {
            let _ = window;
            false
        }
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn set_frame(window: &Window, rect: (f64, f64, f64, f64)) -> bool {
    use objc2_app_kit::NSWindow;
    use tao::platform::macos::WindowExtMacOS;
    // SAFETY: Tao owns this live NSWindow; all calls are on its main thread.
    unsafe {
        let native = &*window.ns_window().cast::<NSWindow>();
        let mut frame = native.frame();
        if frame.size.width == rect.2 && frame.size.height == rect.3 {
            return false;
        }
        frame.origin.x = rect.0;
        frame.origin.y = rect.1;
        frame.size.width = rect.2;
        frame.size.height = rect.3;
        native.setFrame_display(frame, true);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn corner_drag_preserves_top_left_and_clamps_size() {
        let start = DragStart {
            pointer: (900.0, 300.0),
            origin: (80.0, 280.0),
            size: (860.0, 640.0),
        };
        let (x, y, w, h) = start.frame_at((1020.0, 200.0));
        assert_eq!((x, y, w, h), (80.0, 180.0, 980.0, 740.0));
        assert_eq!(y + h, start.origin.1 + start.size.1);
        let (_, y, w, h) = start.frame_at((-1000.0, 5000.0));
        assert_eq!((w, h), (MIN_WIDTH, MIN_HEIGHT));
        assert_eq!(y + h, start.origin.1 + start.size.1);
    }
}
