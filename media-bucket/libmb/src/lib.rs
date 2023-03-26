#![/*god*/forbid(unsafe_code)]
#![allow(dead_code)] // TODO remove me
#![allow(unused_variables)] // TODO remove me
#![feature(async_fn_in_trait)]

//! The documentation was largely written by an OpenAI Assistant (https://openai.com/blog/openai-assistant/).

pub use bucket::{Bucket, BucketError};

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
