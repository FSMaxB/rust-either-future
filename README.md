either-future
=============

`EitherFuture` is an implementation of `Future` for `Either<impl Future, impl Future>`.

It is both implemented for `futures::Future` (0.1) and `core::future::Future`.

## Features
`futures01`: Implement `futues::Future`
`std_future`: Implement `core::future::Future`, enabled by default
