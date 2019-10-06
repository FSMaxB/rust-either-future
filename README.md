either-future
=============

`EitherFuture` is an implementation of `Future` for `Either<impl Future, impl Future>`.

It is both implemented for `futures::Future` (0.1) and `std::future::Future`, both enabled by default but behind feature flags so you can pick and chose.

Feature flags: `std_future`, `futures_future`
