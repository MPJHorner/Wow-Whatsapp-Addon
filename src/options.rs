use anyhow::{Context, Result, bail};
use global_hotkey::hotkey::HotKey;

pub struct Options {
    pub hotkey: HotKey,
    pub hotkey_label: String,
    pub zoom: f64,
    pub background: bool,
    pub ui_diagnostics: Option<std::path::PathBuf>,
    pub ui_snapshot: Option<std::path::PathBuf>,
    pub theme_file: Option<std::path::PathBuf>,
}

impl Options {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Self>> {
        let mut hotkey_label = String::from("Ctrl+Shift+W");
        let mut zoom = 0.85;
        let mut background = false;
        let mut ui_diagnostics = None;
        let mut ui_snapshot = None;
        let mut theme_file = None;
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    println!(
                        "WoW Whisper {}\n\nA frameless WhatsApp Web panel for macOS and Windows.\n\nUsage: wow-chat [--hotkey Ctrl+Shift+W] [--zoom 0.85]\n\nKeep WoW windowed/borderless for live messages. The panel hides when you\nswitch away from WoW. Drag its header to move; use the corner grip to resize.\nThe close button quits; the hotkey hides or shows the panel.\n",
                        env!("CARGO_PKG_VERSION")
                    );
                    return Ok(None);
                }
                "--version" | "-V" => {
                    println!("WoW Whisper {}", env!("CARGO_PKG_VERSION"));
                    return Ok(None);
                }
                "--hotkey" => {
                    hotkey_label = args.next().context("--hotkey requires a key combination")?
                }
                "--background" => background = true,
                "--inspect-ui" => {
                    ui_diagnostics = Some(
                        args.next()
                            .context("--inspect-ui requires an output file")?
                            .into(),
                    );
                }
                "--snapshot-ui" => {
                    ui_snapshot = Some(
                        args.next()
                            .context("--snapshot-ui requires an output directory")?
                            .into(),
                    );
                }
                "--theme-file" => {
                    theme_file = Some(
                        args.next()
                            .context("--theme-file requires a local CSS file")?
                            .into(),
                    );
                }
                "--zoom" => {
                    zoom = args
                        .next()
                        .context("--zoom requires a number")?
                        .parse::<f64>()
                        .context("--zoom must be a number between 0.6 and 1.5")?;
                    if !(0.6..=1.5).contains(&zoom) {
                        bail!("--zoom must be between 0.6 and 1.5");
                    }
                }
                _ => bail!("Unknown option: {arg}. Use --help."),
            }
        }
        let hotkey = hotkey_label
            .parse::<HotKey>()
            .context("Invalid hotkey; try Ctrl+Shift+W")?;
        Ok(Some(Self {
            hotkey,
            hotkey_label,
            zoom,
            background,
            ui_diagnostics,
            ui_snapshot,
            theme_file,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_nonfinite_and_out_of_bounds_zoom() {
        for value in ["NaN", "inf", "-inf", "0", "2"] {
            assert!(Options::parse(["--zoom".into(), value.into()]).is_err());
        }
    }
}
