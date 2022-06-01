#!/usr/bin/env bash

CI_VERSION="$(grep "FORC_VERSION:" ../workflows/ci.yml | cut -d " " -f4)"
BADGE_LINK_VERSION="$(grep "https://crates.io/crates/forc/" ../../README.md | cut -d "/" -f6 | cut -d "\"" -f1)"
BADGE_VERSION="$(grep "https://img.shields.io/badge/forc-v" ../../README.md | cut -d "/" -f5 | cut -d "-" -f2  | cut -d "v" -f2)"

if [ "$BADGE_LINK_VERSION" != "$CI_VERSION" ]; then
    echo "CI forc version is out of sync with the README.md version"
    exit 1
elif [ "$BADGE_LINK_VERSION" != "$BADGE_VERSION" ]; then
    echo "Badge link version is out of sync with the badge img"
    exit 1
fi
