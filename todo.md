# Liferay Workspace Updater (LWU) - Improvement Plan

## 1. CI/CD & Build Stability (High Priority)

- [x] **Fix GitHub Action Runners**: Update `.github/workflows/release.yml` to use `macos-latest` for all macOS builds and update actions to `v4`.
- [x] **Stabilize Cross-Compilation**: Switch `reqwest` from `native-tls` to `rustls-tls` in `Cargo.toml` to eliminate system OpenSSL dependencies.
- [x] **Clean Up Dependencies**: Remove unused `edit-xml` dependency.

## 2. Feature: Product Version Management

- [x] **Fetch Latest Product Version**: Implement a utility to fetch the latest Liferay Portal/DXP versions from Liferay's metadata.
- [x] **Update `gradle.properties`**:
  - [x] Create `src/utils/properties.rs` to handle reading/writing `.properties` files.
  - [x] Implement logic to update `liferay.workspace.product`.
- [x] **Integrate into CLI**: Add a flag or subcommand (e.g., `lwu update --product`) to trigger this.

## 3. Feature: Workspace Health Check (`lwu doctor`)

- [x] **Gradle Wrapper Validation**: Check `gradle/wrapper/gradle-wrapper.properties` and recommend updates to 8.5+.
- [x] **Java Compatibility Check**: Verify `sourceCompatibility` in `build.gradle` or `gradle.properties` against Liferay 7.4 standards.
- [x] **Infrastructure Settings**: Check for `liferay.workspace.bundle.download.read.timeout` and suggest adding it if missing.

## 4. Refactoring & Testing

- [x] **Support `latest.release`**: Update the workspace plugin logic to support setting the version to `latest.release` in `settings.gradle`.
- [x] **Expand Test Suite**:
  - [x] Add unit tests for `gradle.properties` manipulation.
  - [x] Add unit tests for Gradle wrapper version detection.
- [x] **Documentation**: Update `README.md` with new command usage.
