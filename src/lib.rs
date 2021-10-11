/*
 * either-future
 *
 * Copyright (C) 2019 Max Bruckner (FSMaxB)
 *
 * Permission to use, copy, modify, and/or distribute this software for any purpose with or without
 * fee is hereby granted, provided that the above copyright notice and this permission notice appear
 * in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
 * SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
 * AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
 * NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

use either::Either;
#[cfg(feature = "futures_future")]
use futures::Async;
use std::ops::{Deref, DerefMut};
#[cfg(feature = "std_future")]
use std::pin::Pin;
#[cfg(feature = "std_future")]
use std::task::{Context, Poll};

pub struct EitherFuture<LeftFuture, RightFuture>(Either<LeftFuture, RightFuture>);

impl<LeftFuture, RightFuture> EitherFuture<LeftFuture, RightFuture> {
	pub fn left(left_future: LeftFuture) -> Self {
		Self(Either::Left(left_future))
	}

	pub fn right(right_future: RightFuture) -> Self {
		Self(Either::Right(right_future))
	}
}

impl<LeftFuture, RightFuture> Deref for EitherFuture<LeftFuture, RightFuture> {
	type Target = Either<LeftFuture, RightFuture>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<LeftFuture, RightFuture> DerefMut for EitherFuture<LeftFuture, RightFuture> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<LeftFuture, RightFuture> From<Either<LeftFuture, RightFuture>> for EitherFuture<LeftFuture, RightFuture> {
	fn from(either: Either<LeftFuture, RightFuture>) -> Self {
		EitherFuture(either)
	}
}

#[cfg(feature = "futures_future")]
impl<Left, Right, ErrorType, LeftFuture, RightFuture> futures::Future for EitherFuture<LeftFuture, RightFuture>
where
	LeftFuture: futures::Future<Item = Left, Error = ErrorType>,
	RightFuture: futures::Future<Item = Right, Error = ErrorType>,
{
	type Item = Either<Left, Right>;
	type Error = ErrorType;

	fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
		let either = match self.0.as_mut() {
			Either::Left(left_future) => match left_future.poll()? {
				Async::Ready(left) => Either::Left(left),
				Async::NotReady => return Ok(Async::NotReady),
			},
			Either::Right(right_future) => match right_future.poll()? {
				Async::Ready(right) => Either::Right(right),
				Async::NotReady => return Ok(Async::NotReady),
			},
		};
		return Ok(Async::Ready(either));
	}
}

#[cfg(feature = "std_future")]
impl<Left, Right, LeftFuture, RightFuture> std::future::Future for EitherFuture<LeftFuture, RightFuture>
where
	LeftFuture: std::future::Future<Output = Left>,
	RightFuture: std::future::Future<Output = Right>,
{
	type Output = Either<Left, Right>;

	fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
		// This use of unsafe is required in order to transform Pin<Either<LeftFuture,RightFuture>>
		// to Either<Pin<LeftFuture>,Pin<RightFuture>> essentially. This is safe to do because nothing
		// is moved or changed in this method, and LeftFuture/RightFuture are properly pinned
		// again before being polled.
		// Future could be implemented for EitherFuture without the use of unsafe, but then it would
		// only work for Futures that are Unpin. Using unsafe makes this work for !Unpin as well.
		unsafe {
			match Pin::get_unchecked_mut(self).0.as_mut() {
				Either::Left(left_future) => {
					let left_future = Pin::new_unchecked(left_future);
					match left_future.poll(context) {
						Poll::Ready(left) => Poll::Ready(Either::Left(left)),
						Poll::Pending => Poll::Pending,
					}
				}
				Either::Right(right_future) => {
					let right_future = Pin::new_unchecked(right_future);
					match right_future.poll(context) {
						Poll::Ready(right) => Poll::Ready(Either::Right(right)),
						Poll::Pending => Poll::Pending,
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use either::Either;
	#[cfg(feature = "std_future")]
	use std::marker::PhantomPinned;

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
	#[cfg(feature = "std-future")]
	fn should_work_with_unpin() {
		let either = Either::<_, NotPinFuture>::Left(ValueFuture::new(42));
		let either_future = EitherFuture::from(either);

		let runtime = tokio02::runtime::Runtime::new().expect("Failed to create runtime.");
		assert_eq!(Either::Left(42), runtime.block_on(either_future));
	}
}
