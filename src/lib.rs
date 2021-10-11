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
