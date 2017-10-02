//! Support for deserializing database resultsets, or individual rows, or individual values,
//! into rust types.
#![warn(missing_docs)]

extern crate chrono;
#[macro_use]
extern crate log;
extern crate serde;

pub mod de;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
