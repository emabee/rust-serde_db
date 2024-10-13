# serde_db

Support for flexibly deserializing database result sets and rows into rust types,
and for serializing rust types into database parameter values.

Being based on serde, this crate can facilitate the data exchange between applications
and a database driver. It is meant to be used by the implementors of database drivers,
who then can expose a more comfortable driver API.

## Usage

Add `serde_db` to the dependencies section in your project's `Cargo.toml`, with

```toml
[dependencies]
serde_db = "0.11"
```

## Examples for Deserialization

The below examples assume the DB driver exposes on its
result set type a function

```rust
    fn try_into<'de, T: serde::Deserialize<'de>>(self) -> Result<T, E>
```

which is implemented using `serde_db`.

The application then can directly assign the database results to appropriate rust variables!

### Convert a n×m result set into a Vec of structs

```rust
#[macro_use]
extern crate serde_derive;
...
#[derive(Deserialize)]
struct MyStruct {...}
...
let result_set = ...;
let data: Vec<MyStruct> = result_set.try_into()?;
```

Note that `MyStruct` has to implement `serde::Deserialize`.

### Assign a n×1 result set to a Vec of fields

```rust
let vec_s: Vec<String> = result_set.try_into()?;
```

### Assign a 1×1 result set to a single field

```rust
let s: String = result_set.try_into()?;
```

### Assign rows to tuples or structs

For better streaming of large result sets, you might want to iterate over the rows, like in

```rust
for row in result_set {
    let t: (String, NaiveDateTime, i32, Option<i32>) = row.try_into()?;
}
```

or

```rust
for row in result_set {
    let data: MyStruct = row.try_into()?;
}
```

## Examples for Serialization

`serde_db` also when a DB driver needs to translate rust values into DB types.

A Prepared Statement for example might have a generic function

```rust
fn add_batch<T>(&mut self, input: &T) -> DbResult<()>
    where T: serde::Serialize
```

If it  is implemented with `serde_db`, then the application can hand over
a _tuple_ of rust values that correspond to the parameters of the prepared statement,
or they can hand over an appropriate _struct_ that implements `serde::Serialize`.

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

## Cargo Features

### `trace` (no default)

Adds trace output (using the `log` macros).
