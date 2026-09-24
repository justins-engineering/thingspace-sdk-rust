# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- The examples no longer log or print the access token, the session token, the browser example's
  token cookies, or raw callback bodies, which carry the listener password in clear

### Added

- examples/native.rs

### Changed

- Replaced ureq with reqwest
- Renamed linux api to native
- Updated native samples
- Moved cf-worker example to examples/cf-worker
- Updated dependencies
- The library builds as an rlib only; the cdylib is the example worker's, so a dependent no longer
  also builds a stray `thingspace_sdk.wasm`

### Fixed

- The crate documentation named `ureq`, which the crate no longer uses
- The native examples for `devices_list` and the three callback listener calls, which no longer
  compiled against the API

### Removed

- console_error_panic_hook crate
- main.rs

[unreleased]: https://github.com/justins-engineering/thingspace-sdk-rust/compare/v0.1.0...master
