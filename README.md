either-future
=============

`EitherFuture` is an implementation of `Future` for `Either<impl Future, impl Future>`.

It is both implemented for `futures::Future` (0.1) and `core::future::Future`.

`futues::Future` is behind the feature flag `futures01`
