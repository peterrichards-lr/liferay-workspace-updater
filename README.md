# Liferay Workspace Updater (`lwu`)

[![Rust CI](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml)
[![Latest Release](https://img.shields.io/github/v/tag/peterrichards-lr/liferay-workspace-updater?label=version)](https://github.com/peterrichards-lr/liferay-workspace-updater/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, cross-platform Rust CLI utility to automatically update the Liferay Workspace Gradle plugin to its latest version. It fetches the most recent metadata from Liferay's Nexus CDN and applies it to your `settings.gradle` file.

## Features

- **Automated:** Compares your local `settings.gradle` version with the latest available on Liferay's Nexus and applies updates.
- **Interactive:** Prompts for confirmation before applying updates by default to ensure safety.
- **Scriptable:** Includes a non-interactive mode (`--yes`) and a version-only output mode for seamless integration into CI/CD pipelines or other tools.
- **Cross-Platform:** Native binaries for macOS (Intel/ARM), Linux, and Windows.
- **Safe:** Performs minimal, targeted regex-based edits to your configuration files, avoiding complex file parsing that could break custom formatting.

## Installation

### macOS / Linux (Homebrew)

```bash
brew tap peterrichards-lr/homebrew-tap
brew install lwu
```

### Windows (Scoop)

```powershell
scoop bucket add peterrichards-lr https://github.com/peterrichards-lr/scoop-bucket
scoop install lwu
```

### Windows Subsystem for Linux (WSL)

The tool works perfectly in WSL! Install via Homebrew within your WSL distribution:

```bash
brew install lwu
```

### Manual Download

Download the pre-compiled executable for your OS from the [GitHub Releases](https://github.com/peterrichards-lr/liferay-workspace-updater/releases) page.

### From Source

If you have Rust installed, you can build from source:

```bash
cargo install --path .
```

## Usage

Check for updates and prompt to apply:

```bash
lwu update
```

Update without prompting (ideal for scripts):

```bash
lwu update --yes
```

Output version information:

```bash
# Output the latest remote version only
lwu version --remote

# Output the current local version only
lwu version --local

# Output both
lwu version
```

### Options:

- `-y, --yes`: Apply updates without prompting for confirmation
- `-p, --path <PATH>`: Specify the path to the Liferay workspace (defaults to current directory)
- `-r, --remote`: Display the latest version available on Nexus
- `-l, --local`: Display the version currently in `settings.gradle`

## Common Patterns

Update all workspaces in a directory recursively:

```bash
for d in ./*/ ; do (cd "$d" && lwu update --yes); done
```

Check the version as part of a script:

```bash
CURRENT_V=$(lwu version --local)
LATEST_V=$(lwu version --remote)
echo "Current: $CURRENT_V, Latest: $LATEST_V"
```

## Disclaimer

**lwu** is provided "as-is" without warranty of any kind. While it is designed to be safe, always ensure your `settings.gradle` is under version control before running any update tool.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
