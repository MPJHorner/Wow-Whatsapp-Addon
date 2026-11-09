use base64::{Engine, engine::general_purpose::STANDARD};

pub fn with_textures(template: &str) -> String {
    let mut output = template.replace(
        "{{UI_FONT}}",
        &format!(
            "data:font/woff2;base64,{}",
            STANDARD.encode(include_bytes!("../assets/wow/BlizzQuadrata.woff2"))
        ),
    );
    for (key, bytes) in [
        (
            "TOOLTIP_BORDER",
            include_bytes!("../assets/wow/UI-Tooltip-Border.PNG").as_slice(),
        ),
        (
            "DIALOG_BORDER",
            include_bytes!("../assets/wow/UI-DialogBox-Border.PNG").as_slice(),
        ),
        (
            "DIALOG_HEADER",
            include_bytes!("../assets/wow/UI-DialogBox-Header.PNG").as_slice(),
        ),
        (
            "MARBLE",
            include_bytes!("../assets/wow/UI-Background-Marble.PNG").as_slice(),
        ),
        (
            "ROCK",
            include_bytes!("../assets/wow/UI-Background-Rock.PNG").as_slice(),
        ),
        (
            "BUTTON_UP",
            include_bytes!("../assets/wow/UI-Panel-Button-Up.PNG").as_slice(),
        ),
        (
            "BUTTON_DOWN",
            include_bytes!("../assets/wow/UI-Panel-Button-Down.PNG").as_slice(),
        ),
        (
            "DIALOG_BACKGROUND",
            include_bytes!("../assets/wow/UI-DialogBox-Background.PNG").as_slice(),
        ),
        (
            "CLOSE_UP",
            include_bytes!("../assets/wow/UI-Panel-MinimizeButton-Up.PNG").as_slice(),
        ),
        (
            "CLOSE_DOWN",
            include_bytes!("../assets/wow/UI-Panel-MinimizeButton-Down.PNG").as_slice(),
        ),
    ] {
        let mut data = format!("data:image/png;base64,{}", STANDARD.encode(bytes));
        if key == "BUTTON_UP" || key == "BUTTON_DOWN" {
            // UIPanelButton uses only the 80 x 22 region of this 128 x 32 atlas.
            // An SVG viewport preserves the original pixels and gives CSS
            // border-image the same texture coordinates as the game.
            let svg = format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 22" width="80" height="22"><image width="128" height="32" href="{data}"/></svg>"#
            );
            data = format!("data:image/svg+xml;base64,{}", STANDARD.encode(svg));
        } else if key == "DIALOG_HEADER" {
            let svg = format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="120 6 272 72" width="272" height="72"><image width="512" height="128" href="{data}"/></svg>"#
            );
            data = format!("data:image/svg+xml;base64,{}", STANDARD.encode(svg));
        }
        output = output.replace(&format!("{{{{{key}}}}}"), &data);
    }
    output
}

pub fn initialization_script() -> String {
    let css = with_textures(include_str!("../assets/whatsapp-theme.css"));
    // JSON encoding avoids interpreting any CSS text as JavaScript.
    let css = serde_json::to_string(&css).expect("serializing a string cannot fail");
    include_str!("../assets/install-theme.js")
        .replace("__WHISPER_CSS_JSON__", &css)
        .replace(
            "__WHISPER_FONT_BASE64__",
            &STANDARD.encode(include_bytes!("../assets/wow/BlizzQuadrata.woff2")),
        )
}

pub fn stylesheet_update(css: &str) -> String {
    let css = serde_json::to_string(&with_textures(css)).expect("serializing a string cannot fail");
    format!(
        r#"(() => {{ if (location.origin !== 'https://web.whatsapp.com') return; const style = document.getElementById('wow-whisper-theme'); if (style) style.textContent = {css}; }})();"#
    )
}
