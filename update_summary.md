# Documentation update summary

## Why this update was needed

The repository described a low-maintenance application as updated to the
latest release even though its toolchain, like nearly every other application
here, is pinned to a 2024 nightly. The stable Sway Book links to these examples,
so readers could reasonably mistake old syntax, SDK usage, transaction
construction, or security patterns for current guidance.

## What changed

- Added a prominent legacy, version-pinned warning to the root README.
- Removed the unsupported claim that the low-maintenance application is
  updated to the latest release.
- Added a status matrix for all 16 root applications.
- Recorded each application's exact channel, Forc version, Fuel Core version,
  toolchain-file update date, and verification status.
- Explained that public workflow history no longer provides a durable
  per-application last-passing date.
- Added cautions about restoring old dated nightlies and about replacing them
  with Fuelup's `latest` mainnet alias.
- Added migration guidance to preserve bytecode and contract IDs and retest
  against an explicitly selected target stack.
- Corrected the contributing guide so it no longer presents the historical
  project layout as the current Rust SDK structure.

## Validation

- Every matrix row was checked against the corresponding committed
  `fuel-toolchain.toml` and its Git history.
- The contributing mdBook built successfully.
- All committed changes pass `git diff --check`.
