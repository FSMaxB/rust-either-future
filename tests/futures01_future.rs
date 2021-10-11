#![cfg(feature = "futures01")]

extern crate either;
extern crate either_future;
extern crate futures;

use self::either::Either;
use self::either_future::EitherFuture;
use self::futures::executor::spawn;

#[test]
fn should_run_left_futures_future() {
	let either: Either<_, futures::future::FutureResult<(), ()>> = Either::Left(futures::future::ok::<_, ()>(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Left(42), spawn(either_future).wait_future().unwrap());
}

#[test]
fn should_run_right_futures_future() {
	let either: Either<futures::future::FutureResult<(), ()>, _> = Either::Right(futures::future::ok::<_, ()>(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Right(42), spawn(either_future).wait_future().unwrap());
}
