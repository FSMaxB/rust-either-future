use either::Either;
use either_future::EitherFuture;
use futures_lite::future::block_on;
use std::future::{ready, Ready};
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

struct NotPinFuture {
	_phantom: PhantomPinned,
}

impl std::future::Future for NotPinFuture {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
		Poll::Ready(())
	}
}

#[test]
fn should_run_left_future() {
	let either = Either::<_, Ready<i32>>::Left(ready(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Left(42), block_on(either_future));
}

#[test]
fn should_run_right_future() {
	let either = Either::<Ready<i32>, _>::Right(ready(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Right(42), block_on(either_future));
}

#[test]
fn should_work_with_unpin() {
	let either = Either::<_, NotPinFuture>::Left(ready(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Left(42), block_on(either_future));
}
