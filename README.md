either-future
=============

`EitherFuture` is a	`no_std` implementation of `Future` for `Either<impl Future, impl Future>`.

It is both implemented for `futures::Future` (0.1) and `core::future::Future`.

The minimum supported rust version is 1.15.0 if default features are disabled and only `futures01` is enabled.
Once `std_futue` is enabled, the minimum version is 1.36.0 which is where `core::future::Future` was introduced.

## Features
`futures01`: Implement `futures::Future` with version 0.1 of the `futures` library
`std_future`: Implement `core::future::Future`, enabled by default
