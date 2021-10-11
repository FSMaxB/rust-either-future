use crate::EitherFuture;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use either::Either;

impl<Left, Right, LeftFuture, RightFuture> Future for EitherFuture<LeftFuture, RightFuture>
where
	LeftFuture: Future<Output = Left>,
	RightFuture: Future<Output = Right>,
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
