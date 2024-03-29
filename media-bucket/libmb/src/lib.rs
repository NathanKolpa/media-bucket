#![/*god*/forbid(unsafe_code)]
#![allow(dead_code)] // TODO remove me
#![allow(unused_variables)] // TODO remove me

pub use bucket::{Bucket, BucketError, SyncMatchStategy};

mod bucket;
pub mod data_source;
pub mod http_client;

#[cfg(feature = "http-server")]
pub mod http_server;

#[cfg(feature = "local")]
pub mod local;

pub mod model;

#[cfg(feature = "local")]
mod media_import;

mod http_models;
