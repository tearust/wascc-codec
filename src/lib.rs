#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]
// Copyright 2015-2020 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # wascc-codec
//!
//! This library provides the core set of types and associated functions used to facilitate actor
//! and host runtime communication for waSCC.

#![feature(generic_associated_types)]
#![feature(min_specialization)]

/// The version of the codec as seen on crates.io
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The string used for the originator of messages dispatched by the host runtime
pub const SYSTEM_ACTOR: &str = "system";

#[macro_use]
extern crate serde_derive;
extern crate log;

pub use tea_codec::{deserialize, serialize};

pub trait Sample {
	fn sample() -> Self;
}

pub mod blobstore;
pub mod capabilities;
pub mod core;
pub mod error;
pub mod eventstreams;
pub mod extras;
pub mod http;
pub mod keyvalue;
pub mod logging;
pub mod messaging;
