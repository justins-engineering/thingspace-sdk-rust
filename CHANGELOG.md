# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- The examples no longer log or print the access token, the session token, the browser example's
  token cookies, or raw callback bodies, which carry the listener password in clear
- The example worker's unauthenticated `/api/*` routes are behind its `api` feature, no longer on
  by default; the default build serves only the callback receiver and the websocket echo
- `Debug` for `LoginResponse`, `Session`, `SessionRequestBody` and `CallbackListener` redacts the
  token or password

### Added

- `NiddMessage::validate`, `MAX_NIDD_BYTES`, `NIDD_DELIVERY_TIME_SECS` and
  `Error::NiddMessage(NiddMessageError)`: `send_nidd` refuses a message over 1358 decoded bytes or
  a delivery time outside 2..=2592000 s before sending anything
- examples/native.rs

### Changed

- `send_nidd` takes `&NiddMessage`; it never needed `&mut`
- `Error::Credential` and `Error::ThingSpace` (and their `CredentialError` and `ThingSpaceError`
  bodies) are replaced by `Error::Api { status, code, message }`, which keeps the HTTP status and
  reads Verizon's gateway fault, M2M and OAuth error bodies alike; any other body keeps the status
  alone instead of surfacing as a parse error
- `LoginResponse` requires only `access_token`: `scope` and `token_type` default to empty and
  `expires_in` to 3600, since Verizon does not document the token response
- Replaced ureq with reqwest
- Renamed linux api to native
- Updated native samples
- Moved cf-worker example to examples/cf-worker
- Updated dependencies
- The library builds as an rlib only; the cdylib is the example worker's, so a dependent no longer
  also builds a stray `thingspace_sdk.wasm`

### Fixed

- `get_access_token` no longer panics when the key pair is longer than a fixed 96-byte buffer;
  the `Basic` value is allocated at the size the keys need
- The crate documentation named `ureq`, which the crate no longer uses
- The native examples for `devices_list` and the three callback listener calls, which no longer
  compiled against the API

### Removed

- `Display` for `LoginResponse` and `Session`, which printed the access and session tokens
- console_error_panic_hook crate
- main.rs

[unreleased]: https://github.com/justins-engineering/thingspace-sdk-rust/compare/v0.1.0...master
