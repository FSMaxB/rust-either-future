#![cfg(feature = "std_future")]

use either::Either;
use either_future::EitherFuture;
use futures_lite::future::block_on;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ValueFuture<Type>(Option<Type>)
where
	Type: Unpin;

impl<Type> ValueFuture<Type>
where
	Type: Unpin,
{
	fn new(value: Type) -> ValueFuture<Type> {
		ValueFuture(Some(value))
	}
}

impl<Type> std::future::Future for ValueFuture<Type>
where
	Type: Unpin,
{
	type Output = Type;

	fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
		Poll::Ready(
			Pin::into_inner(self)
				.0
				.take()
				.expect("ValueFuture has already resolved."),
		)
	}
}

struct NotPinFuture {
	_phantom: PhantomPinned,
}

#[cfg(feature = "std_future")]
impl std::future::Future for NotPinFuture {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
		Poll::Ready(())
	}
}

#[test]
fn should_run_left_future() {
	let either = Either::<_, ValueFuture<()>>::Left(ValueFuture::new(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Left(42), block_on(either_future));
}

#[test]
fn should_run_right_future() {
	let either = Either::<ValueFuture<()>, _>::Right(ValueFuture::new(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Right(42), block_on(either_future));
}

#[test]
fn should_work_with_unpin() {
	let either = Either::<_, NotPinFuture>::Left(ValueFuture::new(42));
	let either_future = EitherFuture::from(either);

	assert_eq!(Either::Left(42), block_on(either_future));
}
