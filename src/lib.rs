#![no_std]
//! `EitherFuture` is a `no_std` implementation of `Future<Output = Either<Left, Right>>` for [`Either<LeftFuture, RightFuture>`].
//!
//! The minimum supported rust version (MSRV) is 1.36.0 (the version where `core::future::Future` was stabilized).

extern crate either;

use core::ops::{Deref, DerefMut};
use either::Either;

mod future;

pub struct EitherFuture<Left, Right>(pub Either<Left, Right>);

impl<Left, Right> EitherFuture<Left, Right> {
	pub fn left(left_future: Left) -> Self {
		EitherFuture(Either::Left(left_future))
	}

	pub fn right(right_future: Right) -> Self {
		EitherFuture(Either::Right(right_future))
	}
}

impl<Left, Right> Deref for EitherFuture<Left, Right> {
	type Target = Either<Left, Right>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<Left, Right> DerefMut for EitherFuture<Left, Right> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<Left, Right> From<Either<Left, Right>> for EitherFuture<Left, Right> {
	fn from(either: Either<Left, Right>) -> Self {
		EitherFuture(either)
	}
}

impl<Left, Right> From<EitherFuture<Left, Right>> for Either<Left, Right> {
	fn from(either_future: EitherFuture<Left, Right>) -> Either<Left, Right> {
		either_future.0
	}
}
