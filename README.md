# Liferay Workspace Updater (`lwu`)

[![Rust CI](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml)
[![Latest Release](https://img.shields.io/github/v/tag/peterrichards-lr/liferay-workspace-updater?label=version)](https://github.com/peterrichards-lr/liferay-workspace-updater/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, cross-platform Rust CLI utility to automatically update Liferay Workspace components to their latest versions. It fetches the most recent metadata from Liferay's Nexus and Release CDNs and applies it to your `settings.gradle` and `gradle.properties` files.

## Features

- **Automated Plugin Updates:** Compares your local Workspace Plugin version with the latest available on Liferay's Nexus and applies updates to `settings.gradle`.
- **Product Version Management:** Automatically updates `liferay.workspace.product` in `gradle.properties` to the latest recommended DXP or Portal release.
- **Workspace Doctor:** Runs health checks on your workspace, validating Gradle Wrapper versions, Java compatibility, and infrastructure settings.
- **Interactive:** Prompts for confirmation before applying updates by default to ensure safety.
- **Scriptable:** Includes a non-interactive mode (`--yes`) and a version-only output mode for seamless integration into CI/CD pipelines.
- **Cross-Platform:** Native binaries for macOS (Intel/ARM), Linux, and Windows.
- **Safe:** Performs minimal, targeted edits to your configuration files, preserving your custom formatting.

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

### Manual Download

Download the pre-compiled executable for your OS from the [GitHub Releases](https://github.com/peterrichards-lr/liferay-workspace-updater/releases) page.

## Usage

### Update Components

Check for and apply Workspace Plugin updates:

```bash
lwu update
```

Update the Liferay Product version (`liferay.workspace.product`) in `gradle.properties`:

```bash
lwu update --product
```

Update everything without prompting:

```bash
lwu update --plugin --product --yes
```

### Workspace Health Check

Run the "Doctor" to identify issues and get recommendations:

```bash
lwu doctor
```

### Version Information

```bash
# Display local and remote versions for all components
lwu version

# Display only the latest remote versions
lwu version --remote
```

### Options:

- `-y, --yes`: Apply updates without prompting for confirmation
- `--plugin`: Update the Liferay Workspace Gradle plugin (default)
- `--product`: Update the `liferay.workspace.product` in `gradle.properties`
- `-p, --path <PATH>`: Specify the path to the Liferay workspace (defaults to current directory)
- `-r, --remote`: Display the latest version available online
- `-l, --local`: Display the version currently in your configuration files

## Common Patterns

Update all workspaces in a directory recursively:

```bash
for d in ./*/ ; do (cd "$d" && lwu update --yes); done
```

Check the health of a specific workspace:

```bash
lwu doctor --path /path/to/my/workspace
```

## Disclaimer

**lwu** is provided "as-is" without warranty of any kind. While it is designed to be safe, always ensure your workspace is under version control before running any update tool.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
