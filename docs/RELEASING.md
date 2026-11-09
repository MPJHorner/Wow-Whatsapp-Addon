# Releases

Versions follow [Semantic Versioning](https://semver.org/): patch for compatible
fixes, minor for features, major for incompatible changes. Before 1.0, interfaces
and behaviour may still change. Only stable `vMAJOR.MINOR.PATCH` tags are currently
published by the workflow; prerelease tags are rejected by the version check.

## One-click version bump and release

Use **Actions → Release → Run workflow** on `main`, choosing patch, minor or major.
The workflow:

1. Validates and updates the app version in `Cargo.toml`, the app entry in
   `Cargo.lock`, and `CFBundleShortVersionString` in `packaging/Info.plist`.
2. Increments the macOS integer build number.
3. Commits those changes and atomically pushes the commit and annotated tag.
4. Calls the reusable Build workflow at that exact tag.
5. Tests and builds macOS arm64, macOS x64 and Windows x64 on native runners.
6. Publishes the three ZIPs, SHA-256 checksums and generated release notes only
   after every platform passes.

No extra secret or PAT is needed. The version and publishing jobs have
`contents: write`; ordinary build jobs have read access. This intentionally calls
Build directly because [a push made with GITHUB_TOKEN does not start another push workflow](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow).
The workflow runs only on `main`. If branch protection later disallows bot pushes,
use a version-bump PR and the manual tag procedure instead of weakening protection.

## Manual tag / first release

After changing a version (if needed), validate and commit it before tagging:

```sh
python3 scripts/version.py --bump patch
python3 scripts/version.py --check
# Review and commit Cargo.toml, Cargo.lock and packaging/Info.plist.
git tag -a v0.3.1 -m 'WoW Whisper v0.3.1'
git push --atomic origin main v0.3.1
```

Substitute the version printed by the script. For the initial `0.3.0` release,
no bump is necessary: validate `--check --tag v0.3.0`, commit the project and push
that tag. A human-pushed tag triggers Build and publication automatically.

## Failure recovery

If a build or upload fails, the version commit/tag remains. Fix transient issues
and use **Re-run failed jobs** on the existing run; do not run Release again just
to retry, since that creates another version. An existing GitHub Release can have
its matching assets uploaded again by the same workflow.

A source fix should normally receive a new version/tag, rather than moving a
published tag. Never force-push a published release. The workflow checks that the
tag matches all package versions before publication.

## Artifacts and signatures

Branch/PR builds upload ZIPs as Actions artifacts (14-day retention); release
assets are attached to GitHub Releases for public download without an Actions
artifact login. Asset names are stable so `/releases/latest/download/…` links work.

The Mac app is ad-hoc signed. No Developer ID/notarization or Windows signing
certificate is configured. Do not describe these builds as notarized, audited or
vendor-approved. ZIP checksums provide integrity checks, not an identity signature.
