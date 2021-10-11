#![cfg(feature = "std_future")]
use either::Either;
use either_future::EitherFuture;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

mod block_on;

use block_on::block_on;

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

fn ready<T>(value: T) -> Ready<T> {
	Ready(Some(value))
}

struct Ready<T>(Option<T>);

impl<T: Unpin> Future for Ready<T> {
	type Output = T;

	fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
		let value = self.get_mut().0.take().expect("Future is already finished.");
		Poll::Ready(value)
	}
}

struct NotPinFuture {
	_phantom: PhantomPinned,
}

impl Future for NotPinFuture {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
		Poll::Ready(())
	}
}
