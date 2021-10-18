#![no_std]
//! `EitherFuture` is a `no_std` implementation of `Future<Output = Either<Left, Right>>` for [`Either<LeftFuture, RightFuture>`].
//!
//! It is both implemented for [`futures::Future`] (0.1) and [`core::future::Future`].
//!
//! The minimum supported rust version (MSRV) is 1.15.0 if default features are disabled and only `futures01` is enabled.
//! See the different features for their respecitive MSRV.
//!
//! ## Features
//! * `futures01`: Implement [`futures::Future`] with version 0.1 of the [`futures`] library
//!     * MSRV: 1.15.0 (MSRV of [`futures`] `0.1`)
//! * `futures03`: Implement Conversions to and from [`futures_util::future::Either`]
//!     * MSRV: 1.41.0 (MSRV of [futures_util`] `0.3`)
//! * `std_future`: Implement [`core::future::Future`], enabled by default
//!     * MSRV: 1.36.0 (where [`core::future::Future`] was introduced to the standard library)

extern crate either;

use core::ops::{Deref, DerefMut};
use either::Either;

#[cfg(feature = "std_future")]
mod future;
#[cfg(feature = "futures01")]
mod futures01;
#[cfg(feature = "futures03")]
mod futures03;

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

#[cfg(not(feature = "futures03"))]
impl<Left, Right> Into<Either<Left, Right>> for EitherFuture<Left, Right> {
	fn into(self) -> Either<Left, Right> {
		self.0
	}
}

#[cfg(feature = "futures03")]
impl<Left, Right> From<EitherFuture<Left, Right>> for Either<Left, Right> {
	fn from(either_future: EitherFuture<Left, Right>) -> Either<Left, Right> {
		either_future.0
	}
}
