use either::Either;
use either_future::EitherFuture;
#[cfg(feature = "std_future")]
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(feature = "std_future")]
struct ValueFuture<Type>(Option<Type>)
where
	Type: Unpin;

#[cfg(feature = "std_future")]
impl<Type> ValueFuture<Type>
where
	Type: Unpin,
{
	fn new(value: Type) -> ValueFuture<Type> {
		ValueFuture(Some(value))
	}
}

#[cfg(feature = "std_future")]
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

#[cfg(feature = "std_future")]
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
#[cfg(feature = "std_future")]
fn should_run_left_std_future() {
	let either = Either::<_, ValueFuture<()>>::Left(ValueFuture::new(42));
	let either_future = EitherFuture::from(either);

	let mut runtime = tokio02::runtime::Runtime::new().expect("Failed to create runtime.");
	assert_eq!(Either::Left(42), runtime.block_on(either_future));
}

#[test]
#[cfg(feature = "std_future")]
fn should_run_right_std_future() {
	let either = Either::<ValueFuture<()>, _>::Right(ValueFuture::new(42));
	let either_future = EitherFuture::from(either);

	let mut runtime = tokio02::runtime::Runtime::new().expect("Failed to create runtime.");
	assert_eq!(Either::Right(42), runtime.block_on(either_future));
}

#[test]
#[cfg(feature = "std_future")]
fn should_work_with_unpin() {
	let either = Either::<_, NotPinFuture>::Left(ValueFuture::new(42));
	let either_future = EitherFuture::from(either);

	let mut runtime = tokio02::runtime::Runtime::new().expect("Failed to create runtime.");
	assert_eq!(Either::Left(42), runtime.block_on(either_future));
}
