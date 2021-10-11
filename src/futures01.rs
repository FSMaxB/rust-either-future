#![cfg(feature = "futures01")]
extern crate either;
extern crate futures;

use self::either::Either;
use self::futures::Async;
use super::EitherFuture;

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
		Ok(Async::Ready(either))
	}
}
