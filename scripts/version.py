#!/usr/bin/env python3
"""Validate or bump the app's SemVer without changing dependency versions."""
import argparse
import os
from pathlib import Path
import plistlib
import re

ROOT = Path(__file__).resolve().parents[1]
SEMVER = r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)"
MANIFEST = re.compile(r'(?m)^(version = ")([^"\n]+)(")$')
LOCK = re.compile(r'(\[\[package\]\]\nname = "wow-chat"\nversion = ")([^"\n]+)(")')


def read_state(root):
    manifest = (root / 'Cargo.toml').read_text()
    lock = (root / 'Cargo.lock').read_text()
    info_path = root / 'packaging/Info.plist'
    info = plistlib.loads(info_path.read_bytes())
    versions = MANIFEST.findall(manifest)
    locked = LOCK.findall(lock)
    if len(versions) != 1 or len(locked) != 1:
        raise ValueError('Expected exactly one app version in each Cargo file')
    version = versions[0][1]
    if not re.fullmatch(SEMVER, version):
        raise ValueError('Expected a stable MAJOR.MINOR.PATCH app version')
    if locked[0][1] != version or info['CFBundleShortVersionString'] != version:
        raise ValueError('Cargo.toml, Cargo.lock and Info.plist versions disagree')
    if not str(info['CFBundleVersion']).isdigit():
        raise ValueError('CFBundleVersion must be an integer build number')
    return version, manifest, lock, info


def next_version(version, kind):
    values = list(map(int, version.split('.')))
    index = {'major': 0, 'minor': 1, 'patch': 2}[kind]
    values[index] += 1
    for lower in range(index + 1, 3):
        values[lower] = 0
    return '.'.join(map(str, values))


def bump(root, kind, dry_run=False):
    version, manifest, lock, info = read_state(root)
    updated = next_version(version, kind)
    if not dry_run:
        manifest = MANIFEST.sub(lambda m: m[1] + updated + m[3], manifest)
        lock = LOCK.sub(lambda m: m[1] + updated + m[3], lock)
        info['CFBundleShortVersionString'] = updated
        info['CFBundleVersion'] = str(int(info['CFBundleVersion']) + 1)
        (root / 'Cargo.toml').write_text(manifest)
        (root / 'Cargo.lock').write_text(lock)
        (root / 'packaging/Info.plist').write_bytes(plistlib.dumps(info, sort_keys=False))
        read_state(root)
    return updated


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--check', action='store_true')
    mode.add_argument('--bump', choices=('patch', 'minor', 'major'))
    parser.add_argument('--tag', help='Require this exact vMAJOR.MINOR.PATCH tag')
    parser.add_argument('--dry-run', action='store_true')
    args = parser.parse_args()
    if args.tag and not args.check:
        parser.error('--tag requires --check')
    if args.dry_run and not args.bump:
        parser.error('--dry-run requires --bump')
    try:
        version = bump(ROOT, args.bump, args.dry_run) if args.bump else read_state(ROOT)[0]
        if args.tag and args.tag != 'v' + version:
            raise ValueError('Tag must match app version: v' + version)
    except (ValueError, KeyError) as error:
        parser.exit(1, str(error) + '\n')
    print(version)
    if os.environ.get('GITHUB_OUTPUT'):
        with open(os.environ['GITHUB_OUTPUT'], 'a') as output:
            output.write(f'version={version}\ntag=v{version}\n')


if __name__ == '__main__':
    main()
