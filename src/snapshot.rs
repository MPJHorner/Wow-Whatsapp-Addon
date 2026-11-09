//! Explicit, local UI capture for visual verification. Never enabled by default.
#[cfg(target_os = "macos")]
pub fn capture(view: &wry::WebView, path: std::path::PathBuf) {
    use block2::RcBlock;
    use objc2::AnyThread;
    use objc2_app_kit::{NSBitmapImageFileType, NSBitmapImageRep, NSImage};
    use objc2_foundation::{NSDictionary, NSError};
    use std::os::unix::fs::PermissionsExt;
    use wry::WebViewExtMacOS;
    let callback = RcBlock::new(move |image: *mut NSImage, error: *mut NSError| {
        if image.is_null() || !error.is_null() {
            return;
        }
        // SAFETY: WebKit provides valid objects for this main-thread callback.
        unsafe {
            let Some(tiff) = (&*image).TIFFRepresentation() else {
                return;
            };
            let Some(bitmap) = NSBitmapImageRep::initWithData(NSBitmapImageRep::alloc(), &tiff)
            else {
                return;
            };
            let Some(png) = bitmap.representationUsingType_properties(
                NSBitmapImageFileType::PNG,
                &NSDictionary::new(),
            ) else {
                return;
            };
            if std::fs::write(&path, png.to_vec()).is_ok() {
                let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
            }
        }
    });
    // Captures only this application's web view, without screen recording or
    // input injection. WebKit retains the block for asynchronous completion.
    unsafe {
        view.webview()
            .takeSnapshotWithConfiguration_completionHandler(None, &callback);
    }
}

#[cfg(target_os = "windows")]
pub fn capture(_: &wry::WebView, _: std::path::PathBuf) {
    eprintln!("Local web-view snapshots currently require macOS.");
}
