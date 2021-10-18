#![no_std]

extern crate either;

use core::ops::{Deref, DerefMut};
use either::Either;

#[cfg(feature = "std_future")]
mod future;
#[cfg(feature = "futures01")]
mod futures01;
#[cfg(feature = "futures03")]
mod futures03;

pub struct EitherFuture<LeftFuture, RightFuture>(Either<LeftFuture, RightFuture>);

impl<LeftFuture, RightFuture> EitherFuture<LeftFuture, RightFuture> {
	pub fn left(left_future: LeftFuture) -> Self {
		EitherFuture(Either::Left(left_future))
	}

	pub fn right(right_future: RightFuture) -> Self {
		EitherFuture(Either::Right(right_future))
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
