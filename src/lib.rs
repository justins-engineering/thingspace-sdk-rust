// #![warn(missing_docs)]
//! A Verizon ThingSpace API client over `worker::Fetch` on Cloudflare Workers (the `worker`
//! feature, the default) or `reqwest` natively (the `reqwest` feature).
//!
//! This library currently only covers the NBIoT related API endpoints.
pub mod api;
pub mod models;
