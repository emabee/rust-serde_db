//! Support for deserializing database resultsets and rows into rust types,
//! and for serializing rust types into database parameter values.
//!
//! Being based on serde, this crate can facilitate the data exchange
//! between applications and a database driver.
//! It is meant to be consumed by the implementors of database drivers,
//! who then can expose a more comfortable  driver API.
//!

#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate serde;

pub mod de;
pub mod ser;
