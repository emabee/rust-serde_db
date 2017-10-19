//! Support for the serialization of rust types into parameters for database commands.

mod dbv_factory;
mod serialization_error;
mod serializer;

pub use self::dbv_factory::DbvFactory;
pub use self::serialization_error::SerializationError;


use serde;
use self::serializer::Serializer;

/// Translates the input into a Vec of database values.
pub fn to_params<T: ?Sized, DF: DbvFactory>(value: &T, metadata: Vec<DF>)
                                            -> Result<Vec<DF::DBV>, SerializationError>
    where T: serde::ser::Serialize
{
    trace!("serde_db::to_params()");
    let mut serializer = Serializer::new(metadata);
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}
