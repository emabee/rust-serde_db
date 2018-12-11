# serde_db

Support for flexibly deserializing database resultsets and rows into rust types,
and for serializing rust types into database parameter values.

Being based on serde, this crate can facilitate the data exchange between applications
and a database driver. It is meant to be consumed by the implementors of database drivers,
who then can expose a more comfortable driver API.

## Usage

Add hdbconnect to the dependencies section in your project's `Cargo.toml`, with

```toml
[dependencies]
serde_db = "0.6"
```

and add this to your crate root:

```rust
extern crate serde_db;
```

## Examples for Deserialization

The below examples assume the DB driver exposes on its
resultset type a function

    fn into_typed<'de, T: serde::de::Deserialize<'de>>(self) -> Result<T, E>

which is implemented using `serde_db`.

The application then can directly assign the database results to appropriate rust variables!

### Convert a n×m resultset into a Vec of structs

```rust
#[macro_use]
extern crate serde_derive;
...
#[derive(Deserialize)]
struct MyStruct {...}
...
let resultset = ...;
let data: Vec<MyStruct> = resultset.into_typed()?;
```

Note that `MyStruct` has to implement `serde::de::Deserialize`.

### Assign a n×1 resultset to a Vec of fields

```rust
let vec_s: Vec<String> = resultset.into_typed()?;
```

### Assign a 1×1 resultset to a single field

```rust
let s: String = resultset.into_typed()?;
```

### Assign rows to tuples or structs

For better streaming of large resultsets, you might want to iterate over the rows, like in

```rust
for row in resultset {
    let t: (String, NaiveDateTime, i32, Option<i32>) = row.into_typed()?;
}
```

or

```rust
for row in resultset {
    let data: MyStruct = row.into_typed()?;
}
```

## Examples for Serialization

`serde_db` also when a DB driver needs to translate rust values into DB types.

A Prepared Statement for example might have a generic function

```rust
fn add_batch<T>(&mut self, input: &T) -> HdbResult<()>
    where T: serde::ser::Serialize
```

If it  is implemented with `serde_db`, then the application can hand over
a _tuple_ of rust values that correspond to the parameters of the prepared statement,
or they can hand over an appropriate _struct_ that implements `serde::ser::Serialize`.

In both cases they do not need to differentiate between nullable and non-nullable
database values (except that they cannot convert an `Option::None` into a non-nullable database
value).

In its implementation of [`DbvFactory`](trait.DbvFactory.html),
the DB driver can decide to make their life even easier by converting flexibly between
different number types (an example can be found in the tests of this crate).

The implementation of `add_batch()` converts `input` into
a Vec of the driver's database values that can subsequently be sent to the DB server:

```rust
    let db_values: Vec<DBValue> = serde_db::ser::to_params(&input, input_metadata)?;
```

It is required that the prepared statement has metadata about the needed input parameters,
which implement [`DbvFactory`](trait.DbvFactory.html).
