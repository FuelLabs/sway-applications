#!/usr/bin/env bash


check() 
{
    BADGE_LINK_VERSION="$(grep "https://crates.io/crates/forc/" $1/README.md | cut -d "/" -f6 | cut -d "\"" -f1)"
    BADGE_VERSION="$(grep "https://img.shields.io/badge/forc-v" $1/README.md | cut -d "/" -f5 | cut -d "-" -f2  | cut -d "v" -f2)"
    CI_VERSION="$(grep "$2:" ../workflows/ci.yml | cut -d " " -f4)"

    if [ "$BADGE_LINK_VERSION" != "$CI_VERSION" ]; then
        echo "[${1}] CI forc version is out of sync with the README.md version"
        exit 1
    elif [ "$BADGE_LINK_VERSION" != "$BADGE_VERSION" ]; then
        echo "[${1}] Badge link version is out of sync with the badge img"
        exit 1
    fi
}

check ../../escrow "ESCROW_FORC_VERSION";
check ../../multisig-wallet "MULTISIG_FORC_VERSION";
