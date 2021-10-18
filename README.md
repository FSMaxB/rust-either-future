either-future
=============

`EitherFuture` is a	`no_std` implementation of `Future` for `Either<impl Future, impl Future>`.

It is both implemented for `futures::Future` (0.1) and `core::future::Future`.

The minimum supported rust version (MSRV) is 1.15.0 if default features are disabled and only `futures01` is enabled.
See the different features for their respecitive MSRV.

## Features
* `futures01`: Implement `futures::Future` with version 0.1 of the `futures` library
    * MSRV: 1.15.0 (MSRV of `futures` `0.1`)
* `futures03`: Implement Conversions to and from `futures_util::future::Either`
    * MSRV: 1.41.0 (minimum supported version by `futures_util` `0.3`)
* `std_future`: Implement `core::future::Future`, enabled by default
    * MSRV: 1.36.0 (where `core::future::Future` was introduced to the standard library)
