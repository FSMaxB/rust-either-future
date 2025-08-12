either-future
=============

`EitherFuture` is a	`no_std` implementation of `Future<Output = Either<Left, Right>` for `Either<LeftFuture, RightFuture>`.

The minimum supported rust version (MSRV) is 1.36.0 (the version where `core::future::Future` was stabilized).
