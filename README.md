# Liferay Workspace Updater (`lwu`)

[![GitHub Release](https://img.shields.io/github/v/release/peterrichards-lr/liferay-workspace-updater)](https://github.com/peterrichards-lr/liferay-workspace-updater/releases)
[![License](https://img.shields.io/github/license/peterrichards-lr/liferay-workspace-updater)](LICENSE)
[![Rust CI](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/liferay-workspace-updater/actions/workflows/rust.yml)

`lwu` is a cross-platform Rust CLI that keeps Liferay Workspaces up to date. It fetches the latest Liferay Workspace Gradle plugin version from Liferay's Nexus and the latest product releases from Liferay's release CDN, then applies them to your `settings.gradle` and `gradle.properties` files.

## Features

- **Automated Plugin Updates:** Compares your local Workspace Plugin version with the latest available on Liferay's Nexus and applies updates to `settings.gradle`.
- **Product Version Management:** Updates `liferay.workspace.product` in `gradle.properties` to the latest recommended DXP or Portal release.
- **Workspace Doctor:** Runs health checks on your workspace (plugin and product versions, Gradle Wrapper version, bundle download timeout, Java `sourceCompatibility`) and can fix some issues automatically.
- **Interactive:** Prompts for confirmation before applying updates by default.
- **Scriptable:** A non-interactive mode (`--yes`) and version-only output modes for CI/CD pipelines.
- **Cross-Platform:** Runs on macOS (Intel/ARM), Linux, and Windows.
- **Safe:** Makes minimal, targeted edits to your configuration files, preserving your custom formatting.

## Installation

### Homebrew (macOS / Linux)

```bash
brew tap peterrichards-lr/tap
brew install lwu
```

### Scoop (Windows)

```powershell
scoop bucket add peterrichards-lr https://github.com/peterrichards-lr/scoop-bucket
scoop install lwu
```

### WSL

Install [Homebrew on Linux](https://docs.brew.sh/Homebrew-on-Linux) inside your WSL distribution, then use the Homebrew commands above.

### Manual Download

Download the pre-compiled executable for your OS from the [GitHub Releases](https://github.com/peterrichards-lr/liferay-workspace-updater/releases) page.

### Build from Source

Requires a Rust toolchain (`cargo`).

```bash
git clone https://github.com/peterrichards-lr/liferay-workspace-updater.git
cd liferay-workspace-updater
cargo install --path .
```

## Usage

```bash
lwu --help
lwu --version
```

`lwu` finds the workspace root by searching upwards from the current directory (or from `--path`) for `settings.gradle`.

### Update Components

Check for and apply Workspace Plugin updates (the default when no component flag is given):

```bash
lwu update
```

Update the Liferay Product version (`liferay.workspace.product`) in `gradle.properties`:

```bash
lwu update --product
```

Update both without prompting:

```bash
lwu update --plugin --product --yes
```

Options:

- `-y, --yes`: Apply updates without prompting for confirmation
- `--plugin`: Update the Liferay Workspace Gradle plugin (default)
- `--product`: Update the `liferay.workspace.product` in `gradle.properties`
- `-p, --path <PATH>`: Specify the path to the Liferay workspace (defaults to current directory)

### Workspace Health Check

Run the doctor to identify issues and get recommendations:

```bash
lwu doctor
```

Options:

- `--fix`: Automatically fix issues where possible (updates an outdated workspace plugin or product version, adds `liferay.workspace.bundle.download.read.timeout=120000`, and sets `sourceCompatibility=17` if it is missing or unsupported). The Gradle Wrapper is reported but not updated.
- `-p, --path <PATH>`: Specify the path to the Liferay workspace (defaults to current directory)

### Version Information

```bash
# Display the lwu version, plus local and remote workspace plugin and product versions
lwu version

# Display only the latest workspace plugin version available on Nexus
lwu version --remote

# Display only the workspace plugin version in settings.gradle
lwu version --local
```

Passing both `--remote` and `--local` prints `local: <version>, remote: <version>`.

Options:

- `-r, --remote`: Display the latest workspace plugin version available on Nexus
- `-l, --local`: Display the workspace plugin version currently in `settings.gradle`
- `-p, --path <PATH>`: Specify the path to the Liferay workspace (defaults to current directory)

### Common Patterns

Update every workspace in the immediate subdirectories of the current directory:

```bash
for d in ./*/ ; do (cd "$d" && lwu update --yes); done
```

Check the health of a specific workspace:

```bash
lwu doctor --path /path/to/my/workspace
```

## Releasing

1. Bump `version` in `Cargo.toml` and run `cargo build` to update `Cargo.lock`.
2. Move the `[Unreleased]` entries in `CHANGELOG.md` into a new `## [x.y.z] - YYYY-MM-DD` section.
3. Commit, then create an annotated tag and push it:

   ```bash
   python3 scripts/release.py --tag vX.Y.Z
   git push origin vX.Y.Z
   ```

   The release workflow fails if the tag isn't annotated, doesn't match `Cargo.toml`, or has no `CHANGELOG.md` section.

4. The Homebrew tap and Scoop bucket pick up the new release automatically within a couple of hours. `.gemini/prompts/update-distribution-channels.md` is only needed for a tool's first release.

## Disclaimer

**`lwu` is provided "as-is" without warranty of any kind. It modifies `settings.gradle` and `gradle.properties` in your workspace. Always make sure your workspace is under version control before running it, and review the changes it makes. You use it at your own risk.**

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
