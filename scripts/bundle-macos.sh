#!/bin/sh
set -eu

cd "$(dirname "$0")/.."
python3 scripts/version.py --check
if command -v cargo >/dev/null 2>&1; then
    whisper_cargo="$(command -v cargo)"
else
    whisper_cargo="$HOME/.cargo/bin/cargo"
fi

export MACOSX_DEPLOYMENT_TARGET=14.0
"$whisper_cargo" build --release --locked --jobs "${WHISPER_BUILD_JOBS:-2}"
whisper_bundle="dist/WoW Whisper.app"
mkdir -p "$whisper_bundle/Contents/MacOS" "$whisper_bundle/Contents/Resources"
cp target/release/wow-chat "$whisper_bundle/Contents/MacOS/wow-chat.next"
# Replace the inode so a running copy keeps its existing executable intact.
mv -f "$whisper_bundle/Contents/MacOS/wow-chat.next" "$whisper_bundle/Contents/MacOS/wow-chat"
cp packaging/Info.plist "$whisper_bundle/Contents/Info.plist"
cp LICENSE THIRD_PARTY_NOTICES.md "$whisper_bundle/Contents/Resources/"
cp assets/wow/NOTICE.md "$whisper_bundle/Contents/Resources/Blizzard-Assets-Notice.md"
codesign --force --deep --sign - "$whisper_bundle"
echo "Built: $whisper_bundle"
