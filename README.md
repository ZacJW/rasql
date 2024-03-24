# Rasql

Statically analyse SQL schemas and queries to generate Rust type definitions.

Rasql doesn't require you to define your schema and queries in its own syntax like an ORM might.
Instead you write in the most mature language for defining and querying databases, SQL. Rasql
statically analyses your SQL, type checking as it goes, and emits compatible Rust types to prevent
runtime errors like this:

```text
cannot convert between the Rust type `i32` and the Postgres type `int8`
```

By writing in SQL, Rasql doesn't lock you in to using Rust for all of your database access.
If another language you work in has a similar type generator, it and Rasql can happily coexist
without even needing to know the other is there.

## `rasql-core`

`rasql-core` holds the entire static analysis engine as well as the type generator. If you're
looking to extend this project, or just track down a bug, this is where you should probably look.
If you just want to use this project, look at the other crates in this repo.

## `rasql-build`

`rasql-build` provides an API designed to be used from a build script. If you have a crate that you
would like to hold type definitions, you can use this from your crate's build script to achieve it.

## `rasql-query`

`rasql-query` provides procedural macros so you can write fully type-checked queries with
automatically typed row output to prevent runtime type errors. This depends on you already using
`rasql-build`.

## Acknowledgements

Rasql builds upon the work of the [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs)
team, without which this project would not exist.
