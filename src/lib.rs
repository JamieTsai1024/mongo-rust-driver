#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![allow(
    clippy::unreadable_literal,
    clippy::cognitive_complexity,
    clippy::float_cmp,
    clippy::match_like_matches_macro,
    clippy::derive_partial_eq_without_eq
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(test, type_length_limit = "80000000")]
#![doc(html_root_url = "https://docs.rs/mongodb/3.2.3")]

#[macro_use]
pub mod options;

#[cfg(feature = "in-use-encryption")]
pub use ::mongocrypt;

pub mod action;
pub(crate) mod bson_compat;
mod bson_util;
pub mod change_stream;
pub(crate) mod checked;
mod client;
mod cmap;
mod coll;
mod collation;
mod compression;
mod concern;
mod cursor;
mod db;
pub mod error;
pub mod event;
pub mod gridfs;
mod hello;
pub(crate) mod id_set;
mod index;
mod operation;
pub mod results;
pub(crate) mod runtime;
mod sdam;
mod search_index;
mod selection_criteria;
mod serde_util;
mod srv;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(test)]
mod test;
#[cfg(feature = "tracing-unstable")]
mod trace;
pub(crate) mod tracking_arc;

#[cfg(not(any(feature = "bson-2", feature = "bson-3")))]
compile_error!("One of the bson-2 and bson-3 features must be enabled.");

#[cfg(all(feature = "bson-2", not(feature = "bson-3")))]
pub use bson2 as bson;

#[cfg(feature = "bson-3")]
pub use bson3 as bson;

#[cfg(feature = "in-use-encryption")]
pub use crate::client::csfle::client_encryption;
pub use crate::{
    client::{session::ClientSession, Client},
    coll::Collection,
    cursor::{
        session::{SessionCursor, SessionCursorStream},
        Cursor,
    },
    db::Database,
};

pub use client::session::ClusterTime;
pub use coll::Namespace;
pub use index::IndexModel;
pub use sdam::public::*;
pub use search_index::{SearchIndexModel, SearchIndexType};

/// A boxed future.
pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[cfg(not(feature = "compat-3-3-0"))]
compile_error!(
    "The feature 'compat-3-3-0' must be enabled to ensure forward compatibility with future \
     versions of this crate."
);
