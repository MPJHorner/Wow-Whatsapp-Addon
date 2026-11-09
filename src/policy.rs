use url::Url;
use wry::{PermissionKind, PermissionResponse};

/// Only the exact official HTTPS origin can become a page. WhatsApp's own
/// subresource requests to its media/CDN and messaging servers still work.
pub fn is_whatsapp_navigation(value: &str) -> bool {
    let Ok(url) = Url::parse(value) else {
        return false;
    };
    url.scheme() == "https"
        && url.host_str() == Some("web.whatsapp.com")
        && url.port_or_known_default() == Some(443)
        && url.username().is_empty()
        && url.password().is_none()
}

pub fn permission_response(kind: PermissionKind) -> PermissionResponse {
    match kind {
        // Defer microphone access to the WebView/OS prompt. Never grant it silently.
        PermissionKind::Microphone | PermissionKind::Autoplay => PermissionResponse::Default,
        _ => PermissionResponse::Deny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_the_official_https_origin() {
        for url in [
            "https://web.whatsapp.com/",
            "https://web.whatsapp.com:443/?lang=en#chat",
        ] {
            assert!(is_whatsapp_navigation(url), "{url}");
        }
        for url in [
            "http://web.whatsapp.com/",
            "https://web.whatsapp.com.evil.example/",
            "https://web.whatsapp.com@evil.example/",
            "https://evil.example@web.whatsapp.com/",
            "https://web.whatsapp.com:8443/",
            "https://whatsapp.com/",
            "https://web.whatsapp.com./",
            "file:///etc/passwd",
            "javascript:alert(1)",
            "data:text/html,hello",
            "whatsapp://send?text=hello",
            "about:blank",
            "not a URL",
        ] {
            assert!(!is_whatsapp_navigation(url), "{url}");
        }
    }

    #[test]
    fn sensitive_permissions_are_denied() {
        for permission in [
            PermissionKind::Camera,
            PermissionKind::DisplayCapture,
            PermissionKind::ClipboardRead,
            PermissionKind::Geolocation,
            PermissionKind::FileSystemAccess,
            PermissionKind::Notifications,
            PermissionKind::Other,
        ] {
            assert_eq!(permission_response(permission), PermissionResponse::Deny);
        }
    }

    #[test]
    fn microphone_uses_the_permission_prompt_instead_of_silent_access() {
        assert_eq!(
            permission_response(PermissionKind::Microphone),
            PermissionResponse::Default
        );
    }
}
