# lwu

[![GitHub Release](https://img.shields.io/github/v/release/peterrichards-lr/liferay-workspace-updater)](https://github.com/peterrichards-lr/liferay-workspace-updater/releases)
[![License](https://img.shields.io/github/license/peterrichards-lr/liferay-workspace-updater)](https://github.com/peterrichards-lr/liferay-workspace-updater/LICENSE)
[![Rust CI](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml)

**lwu** (Liferay Workspace Updater) is a CLI tool designed to automatically update the Liferay Workspace Gradle plugin to its latest version by fetching the most recent metadata from Liferay's Nexus CDN.

## Features

- **Automated Version Checking:** Compares your local `settings.gradle` version with the latest available on Liferay's Nexus.
- **Interactive & Safe:** Prompts for confirmation before applying updates by default.
- **Scriptable:** Includes a non-interactive mode (`--yes`) and a version-only output mode for piping to other tools.
- **Cross-Platform:** Works seamlessly on macOS, Linux, and Windows.
- **Safe & Secure:** Performs minimal, targeted edits to your configuration files.

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap peterrichards-lr/homebrew-tap && brew install lwu
```

### Scoop (Windows)

```bash
scoop bucket add lwu-bucket https://github.com/peterrichards-lr/scoop-bucket && scoop install lwu
```

### WSL

Install via Homebrew within WSL:

```bash
brew install lwu
```

### Manual Download

Download the latest binary for your platform from the [GitHub Releases](https://github.com/peterrichards-lr/liferay-workspace-updater/releases) page.

### Build from Source

```bash
cargo install --path .
```

## Usage

Check for updates and prompt to apply:

```bash
lwu update
```

Update without prompting:

```bash
lwu update --yes
```

Output the latest version only:

```bash
lwu version --remote
```

Output the current local version only:

```bash
lwu version --local
```

### Common Patterns

Update all workspaces in a directory:

```bash
for d in ./*/ ; do (cd "$d" && lwu update --yes); done
```

## Disclaimer

**lwu** is provided "as-is" without warranty of any kind. While it is designed to be safe, always ensure your `settings.gradle` is under version control before running any update tool.

## License

MIT
