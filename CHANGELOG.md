# Changelog

## [0.10.0]  2021-12-08

Rename `into_typed` into the more idiomatic `try_into`.

Factor out dependency to log into an optional feature.

Make it an edition 2021.

## [0.9.2]  2020-06-16

Improve errors.

## [0.9.0]  2020-01-31

Change return value types that are based on &String to &str.

Fix some clippies.

Simplify the mock database structs that are used in the tests and as showcase.

## [0.8.3]  2019-08-22

Bring code up-to-date (e.g. for rustc 1.37).

## [0.8.2]  2019-03-29

Update versions of dependencies.

## [0.8.1]  2019-02-17

Remove requirement that DbValue implements Clone.

## [0.8.0]  2019-02-07

Let serde_db::ser::to_params() consume an iterator over input-parameter-descriptors,
rather than a slice of parameter-descriptors.

## [0.7.1]  2019-01-30

Map serialization of () to from_none().

## [0.7.0]  2019-01-25

Make the traits more idiomatic (and easier to implement).

## [0.6.0]  2018-12-11

Use Strings in error variants, rather than &str  (-> version bump).
Fix some clippies.
Migrate tests from rust_decimal to bigdecimal.
Move to edition 2018.

## [0.5.1] 2018-10-27

Support sequence in RowDeserializer.

## [0.5.0]  2018-09-27

Add error type for Parse-errors.

...
