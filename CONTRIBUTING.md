# Contributing

Small, focused fixes are welcome. Start with an issue for a larger change and
include your OS, app version and expected behaviour. Use fictional chats and
redacted screenshots in public reports.

1. Fork and clone the repository.
2. Install stable Rust and your platform's prerequisites from the README.
3. Make the change and run:

   ```sh
   cargo fmt --all -- --check
   cargo clippy --locked --all-targets -- -D warnings
   cargo test --locked
   python3 -m unittest discover -s scripts/tests
   python3 scripts/version.py --check
   ```

4. For native macOS window changes, also run
   `cargo run --locked --example check_window`. It uses an invisible offline
   window and does not touch the game or a WhatsApp account.
5. Open a PR explaining the visible behaviour and what you verified. The CI
   builds on Apple Silicon, Intel macOS and Windows x64.

Keep the official-origin restriction, separate offline frame, and fixed command
handler intact. Do not add message scraping, game hooks, automated messages,
telemetry or credentials. Never commit a WebView profile, chat export, live QR
code or screenshot of a real conversation.

Original code contributions are under the project's MIT license. Preserve
third-party notices; do not label Blizzard assets as MIT. New artwork must have
clear provenance and licensing. The demo must stay fictional and clearly labelled.

See [development notes](docs/DEVELOPMENT.md) for UI tooling and
[releasing](docs/RELEASING.md) for the version/build workflow.
