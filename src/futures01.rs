#![cfg(feature = "futures01")]
use crate::EitherFuture;
use either::Either;
use futures::Async;

impl<Left, Right, ErrorType, LeftFuture, RightFuture> futures::Future for EitherFuture<LeftFuture, RightFuture>
where
	LeftFuture: futures::Future<Item = Left, Error = ErrorType>,
	RightFuture: futures::Future<Item = Right, Error = ErrorType>,
{
	type Item = Either<Left, Right>;
	type Error = ErrorType;

	fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
		use Async::*;
		use Either::*;

		let either = match self.0.as_mut() {
			Left(left_future) => match left_future.poll()? {
				Ready(left) => Left(left),
				NotReady => return Ok(NotReady),
			},
			Right(right_future) => match right_future.poll()? {
				Ready(right) => Right(right),
				NotReady => return Ok(NotReady),
			},
		};
		Ok(Ready(either))
	}
}
