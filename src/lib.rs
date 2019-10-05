use either::Either;
use futures::Async;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct EitherFuture<LeftFuture, RightFuture>(Either<LeftFuture, RightFuture>);

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

impl<Left, Right, ErrorType, LeftFuture, RightFuture> futures::Future
    for EitherFuture<LeftFuture, RightFuture>
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

impl<Left, Right, LeftFuture, RightFuture> std::future::Future
    for EitherFuture<LeftFuture, RightFuture>
where
    LeftFuture: std::future::Future<Output = Left> + Unpin,
    RightFuture: std::future::Future<Output = Right> + Unpin,
{
    type Output = Either<Left, Right>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::into_inner(self).0.as_mut() {
            Either::Left(left_future) => {
                let left_future = Pin::new(left_future);
                match left_future.poll(context) {
                    Poll::Ready(left) => Poll::Ready(Either::Left(left)),
                    Poll::Pending => Poll::Pending,
                }
            }
            Either::Right(right_future) => {
                let right_future = Pin::new(right_future);
                match right_future.poll(context) {
                    Poll::Ready(right) => Poll::Ready(Either::Right(right)),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
