use either::Either;
use either_future::EitherFuture;

#[test]
#[cfg(feature = "futures_future")]
fn should_run_left_futures_future() {
	let either = Either::<_, futures::future::FutureResult<(), ()>>::Left(futures::future::ok::<_, ()>(42));
	let either_future = EitherFuture::from(either);

	let mut runtime = tokio01::runtime::Runtime::new().expect("Failed to create runtime.");
	assert_eq!(Either::Left(42), runtime.block_on(either_future).unwrap());
}

#[test]
#[cfg(feature = "futures_future")]
fn should_run_right_futures_future() {
	let either = Either::<futures::future::FutureResult<(), ()>, _>::Right(futures::future::ok::<_, ()>(42));
	let either_future = EitherFuture::from(either);

	let mut runtime = tokio01::runtime::Runtime::new().expect("Failed to create runtime.");
	assert_eq!(Either::Right(42), runtime.block_on(either_future).unwrap());
}
