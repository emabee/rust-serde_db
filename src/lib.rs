//! Support for deserializing database resultsets and rows into rust types,
//! and for serializing rust types into database parameter values.
//!
//! Being based on serde, this crate can facilitate the data exchange
//! between applications and a database driver.
//! It is meant to be used by the implementors of database drivers,
//! who then can expose a more comfortable  driver API.
//!
//! See the module descriptions for more details and usage examples.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

pub mod de;
pub mod ser;
