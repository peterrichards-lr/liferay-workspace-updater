# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_Nothing yet._

## [1.0.4] - 2026-11-11

### Changed

- Label Dependabot pull requests `no-issue-needed`, alongside the default `dependencies` and ecosystem labels, so bot PRs are marked as intentionally having no linked issue.
- Update Rust dependencies:
  - `anyhow` 1.0.102 → 1.0.104
  - `serde` 1.0.228 → 1.0.229
  - `regex` 1.12.3 → 1.13.1
  - `reqwest` 0.11.27 → 0.12.28
  - `chrono` 0.4.44 → 0.4.45
- Update GitHub Actions:
  - `dtolnay/rust-toolchain` (pinned commit updated)
  - `actions/download-artifact` 8.0.1 → 8.0.2
  - `actions/upload-artifact` 7.0.1 → 7.0.2

## [1.0.3] - 2026-11-11

### Added

- Add `--version` to the CLI, and show the tool version in `lwu version`.
- Add `CHANGELOG.md`, `scripts/release.py` (release note extraction and annotated tagging) and `scripts/doctor.py` (template placeholder check, run from the pre-commit hook).
- Add `.github/dependabot.yml` for weekly GitHub Actions and Cargo updates.
- Add `formula.rb.example` and `scoop.json.example` distribution templates.
- Add `repository` and `license` to `Cargo.toml`.
- Document installation, usage and the release process in `README.md`.

### Changed

- Replace the release workflow with the template's: it derives binary and asset names from `Cargo.toml`, rejects tags that aren't annotated or don't match the `Cargo.toml` version, and publishes release notes from `CHANGELOG.md`.
- Pin GitHub Actions to commit SHAs.
- Enforce `Cargo.lock` synchronization in the pre-commit hook and CI (`--locked`).
- Name the Homebrew formula and Scoop manifest after the binary (`lwu`) rather than the repository in `update-distribution-channels.md`.
- Bump `rand` to 0.8.8.

### Fixed

- Fix `lwu update` panicking in debug builds because `--plugin` and `--path` both claimed `-p`; `--plugin` no longer has a short flag (`-p` is `--path`, as documented).
- Correct inaccurate `version` and `update` option descriptions in `README.md`, and document `doctor --fix`.

## [1.0.2] - 2026-03-18

### Changed

- Bump version to 1.0.2.
- Ignore local distribution artifacts (`lwu.json`, `*_sha.txt`) in `.gitignore`.

### Removed

- Remove the completed `todo.md`.

## [1.0.1] - 2026-03-18

### Added

- Add `lwu update --product` to update `liferay.workspace.product` in `gradle.properties` from Liferay's `releases.json`.
- Add `lwu doctor` with `--fix` for workspace health checks (plugin and product versions, Gradle Wrapper, download timeout, Java `sourceCompatibility`).
- Support `latest.release` as the workspace plugin version.
- Add the Intel macOS (`x86_64-apple-darwin`) release target.

### Changed

- Switch `reqwest` to `rustls-tls`.
- Update repository URLs to `liferay-workspace-updater` and refresh the README layout.

### Fixed

- Fix GitHub Actions failures caused by TLS dependencies and incorrect build paths.

## [1.0.0] - 2026-03-18

### Added

- Initial release of `lwu`: fetch the latest Liferay Workspace Gradle plugin version from Liferay's Nexus, compare it with `settings.gradle`, and update it after an interactive prompt (or non-interactively with `--yes`).
- Add version-only output modes for scripting (`lwu version --remote` / `--local`).
- Add release and CI workflows.

<!-- markdownlint-disable MD049 -->

---

_Last Updated: 2026-11-11_ | _Last Reviewed: 2026-11-11_
