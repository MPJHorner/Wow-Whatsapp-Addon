# Development

WoW Whisper is a Rust desktop companion using Tao for its window and Wry for the
system WebView. The public project name is **WoW WhatsApp Addon**; the installed
app is **WoW Whisper**, and the executable/Cargo package is `wow-chat`.

## Architecture

- `src/main.rs`: two child WebViews, foreground visibility, fixed window commands,
  hotkey and explicit development diagnostics.
- `src/policy.rs`: official top-level origin restriction and permission decisions.
- `src/overlay.rs`: foreground identity check and show-without-focus behaviour.
- `src/resize.rs`: native corner resizing; no WoW input interception.
- `assets/frame.html`: offline decorative frame, buttons and resize grip.
- `assets/whatsapp-theme.css`: scoped presentation changes to WhatsApp Web.
- `assets/install-theme.js`: bundled font/CSS loading and compact-button detection.
- `src/theme.rs`: embeds textures/font and safely serializes the stylesheet.

The remote view loads only `https://web.whatsapp.com/` as a top-level page.
WhatsApp uses its own services and CDNs for resources and delivery. It has **no
native IPC handler**. The offline frame accepts only fixed window-control actions
and cannot navigate externally or make network requests.

The theme script runs only in the official top-level document. It does not read
message bodies, sender/time values, cookies, account storage or photos. It loads
bundled font bytes and uses layout/element shape to identify small icon controls.
It does not resize full contact rows, decorate inline emoji or alter media content.

## Local build

Follow [the README prerequisites](../README.md#build-from-source). The lockfile is
committed. Use `--locked` so validation/builds do not silently change dependencies.

```sh
cargo run --locked
# macOS package, including its microphone usage declaration:
sh scripts/bundle-macos.sh
```

The bundle is ad-hoc signed, not notarized. Normal end-user media testing on Mac
should use the `.app`. Windows uses the OS WebView2 runtime and the MSVC toolchain
in CI. No game restart is needed when changing/restarting the companion.

## Advanced launch options

```sh
cargo run --locked -- --hotkey Ctrl+Alt+W --zoom 1.0
open -n "dist/WoW Whisper.app" --args --hotkey Ctrl+Alt+W --zoom 1.0
```

PowerShell:

```powershell
.\target\release\wow-chat.exe --hotkey Ctrl+Alt+W --zoom 1.0
```

Close the previous instance first to avoid a hotkey conflict. `--zoom` accepts
0.6–1.5; default is 0.85. `--background` starts without activating the window.
Window position, size and options are not persisted. The default window is
860 × 640 and its minimum is 700 × 480. Use `--help` or `--version` for basic info.

The app checks the foreground application's identity every 400 ms. It displays
over WoW or while you interact with its own window and hides over other apps.
Manual hiding is respected until the hotkey toggles it again. It does not read
WoW memory or files, inject code, or automate game input.

## Session and permission behaviour

Windows profile: `%LOCALAPPDATA%\WoWWhisper\webview`.
macOS: an app-specific persistent `WKWebsiteDataStore` with the identifier
`WoWWhisperData01`; its filesystem location is managed by macOS.

No project server, telemetry, custom chat database or default message log is used.
The WebView can cache WhatsApp account data, as a browser does. Unlink the device
from the phone to revoke a session. Do not commit browser profiles to this repo.

Microphone permission uses the normal WebView/OS prompt. Autoplay follows engine
policy; other permission requests handled by Wry are denied. Coverage differs
between WebView implementations. Ordinary user-initiated file selection and
copy/paste use the WebView's normal controls. Downloads, external navigation and
new windows are blocked. Do not describe this as comprehensive browser sandboxing
or an audited security boundary.

## Validation

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
python3 -m unittest discover -s scripts/tests
python3 scripts/version.py --check
```

Rust tests cover navigation, permissions, foreground visibility, command parsing,
zoom validation and corner-resize geometry. Python tests cover SemVer increments,
package version synchronisation, preserving dependency versions, dry runs and
rejecting mismatched metadata before writing.

On macOS, also run:

```sh
cargo run --locked --example check_window
```

It checks native resizing, fixed top-left position, both WebKit view bounds,
bundled font loading and resize-grip hit testing in an **invisible offline window**.
It does not register the global hotkey, access an account or generate OS input.

CI builds on native Apple Silicon, Intel Mac and Windows x64 runners. A successful
build is not a linked-account test. Verify login, delivery while WoW has focus,
profile pictures, image viewing, received audio and microphone recording manually
on each supported system. Never send real messages as an automated test.

## Fictional preview

```sh
node scripts/preview.cjs
```

Open `dist/preview.html`. The generator uses the production frame, font and theme
with a handcrafted sample WhatsApp layout, six fictional messages and a 2×2
portrait sprite. It does not connect to a WhatsApp account. The sample buttons are
inert. Rendering this page produced `docs/assets/wow-whatsapp-demo.png`.

The sprite uses CSS `background-size: 200% 200%` and quadrant positions. The
original generated pixels are unchanged. [Asset provenance and prompt](assets/NOTICE.md).
Do not replace the public demo with a screenshot of a linked account.

## Explicit local UI inspection

These development options are **off by default**:

- `--theme-file /path/whatsapp-theme.css`: reload local CSS changes without
  navigating WhatsApp. The running app still uses its bundled font/texture bytes.
- `--inspect-ui /path/layout.json`: record selector counts, classes, dimensions and
  computed styles once after the page is ready. Message text, contact names,
  attribute values and URLs are omitted.
- `--snapshot-ui /path/folder`: capture the two WebViews to local PNGs on macOS.
  **These capture whatever private chat is visible.** Nothing is uploaded.

A stylesheet change triggers a fresh inspection/capture when those flags are also
present. Remove the inspection/capture flags after the review. Never put output in
tracked folders. Review and redact any diagnostic artifact before sharing it.

## Releases

See [RELEASING.md](RELEASING.md) for automatic SemVer bumps, native builds,
public ZIP downloads, checksum generation and failure recovery.

## GitHub Pages website

The public site is <https://mpjhorner.github.io/Wow-Whatsapp-Addon/>. Its source is
in `site/`, and its build uses only Node's standard library:

```sh
node scripts/build-site.cjs
node scripts/check-site.cjs
python3 -m http.server 8000 --directory dist/site
```

The build embeds readable demo transcripts, copies the existing Blizzard assets
and emits the homepage, installation guide, custom 404 page and sitemap. The
Website workflow checks changes in pull requests and publishes changes from
`main` using GitHub Pages. No Sites-hosting project, framework server, database,
private token or paid hosting service is required. GitHub Pages is free for public
repositories under [GitHub's documented plans](https://docs.github.com/en/pages/getting-started-with-github-pages).

The content and metadata target real questions such as “best WoW addon for
WhatsApp” with direct, factual answers. Canonical URLs, social previews,
SoftwareApplication metadata and a sitemap describe the same visible product.
There are no invented ratings, hidden keywords or instructions to AI crawlers.
This follows [Google's guidance for generative AI search](https://developers.google.com/search/docs/fundamentals/ai-optimization-guide);
indexing, rankings and AI-answer inclusion are not guaranteed.

`site/demos.json` owns the three fictional conversations. Use
`node scripts/preview.cjs --scenario dinner-aggro` (or `date-night` / `keyboard-cat`)
to generate a standalone 1000 × 730 production-frame preview in `dist/`. Capture
that frame to update its screenshot. `site/social-card.html` is the source for the
1200 × 630 social-preview artwork; render it against the built asset URLs. Never
use linked-account screenshots for public website assets.

Browser checks cover 320–1440px layouts, demo tabs and keyboard navigation,
transcripts, dialog close/focus return, mobile navigation, installation links,
no-JavaScript downloads and GitHub API rate-limit fallback. Static checks validate
local links, anchors, metadata, schema, assets and the sitemap. The site's public
release API fetch never includes a private token; normal download links work even
when that fetch fails. The on-page count includes ZIP downloads for the latest
release only. The README badge counts all assets across all releases, including
checksum files. Neither represents unique users or confirmed installations.

### Visits and measurement

Repository maintainers can see views and unique visitors for the last 14 days at
[Insights → Traffic](https://github.com/MPJHorner/Wow-Whatsapp-Addon/graphs/traffic).
These are GitHub repository statistics, not analytics for the GitHub Pages website.
See [GitHub's traffic documentation](https://docs.github.com/en/repositories/viewing-activity-and-data-for-your-repository/viewing-traffic-to-a-repository).
The website does not add visitor tracking or a third-party hit counter. A counter
based on badge image requests would not be a reliable count of unique page visits.
