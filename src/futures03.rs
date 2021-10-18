use crate::EitherFuture;
use futures_util::future::Either;

impl<LeftFuture, RightFuture> From<Either<LeftFuture, RightFuture>> for EitherFuture<LeftFuture, RightFuture> {
	fn from(either: Either<LeftFuture, RightFuture>) -> Self {
		match either {
			Either::Left(left) => EitherFuture::left(left),
			Either::Right(right) => EitherFuture::right(right),
		}
	}
}

impl<LeftFuture, RightFuture> From<EitherFuture<LeftFuture, RightFuture>> for Either<LeftFuture, RightFuture> {
	fn from(either_future: EitherFuture<LeftFuture, RightFuture>) -> Self {
		match either_future.0 {
			either::Either::Left(left) => Either::Left(left),
			either::Either::Right(right) => Either::Right(right),
		}
	}
}
