//! Support for serialization (planned) and deserialization for database artifacts.
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
