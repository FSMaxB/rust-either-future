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

impl<LeftFuture, RightFuture> From<Either<LeftFuture, RightFuture>>
    for EitherFuture<LeftFuture, RightFuture>
{
    fn from(either: Either<LeftFuture, RightFuture>) -> Self {
        EitherFuture(either)
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

    use super::*;
    use either::Either;

    struct ValueFuture<Type>(Option<Type>)
    where
        Type: Unpin;

    impl<Type> ValueFuture<Type>
    where
        Type: Unpin,
    {
        fn new(value: Type) -> ValueFuture<Type> {
            ValueFuture(Some(value))
        }
    }

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

    #[test]
    fn should_run_left_futures_future() {
        let either = Either::<_, futures::future::FutureResult<(), ()>>::Left(
            futures::future::ok::<_, ()>(42),
        );
        let either_future = EitherFuture::from(either);

        let mut runtime = tokio01::runtime::Runtime::new().expect("Failed to create runtime.");
        assert_eq!(Either::Left(42), runtime.block_on(either_future).unwrap());
    }

    #[test]
    fn should_run_right_futures_future() {
        let either = Either::<futures::future::FutureResult<(), ()>, _>::Right(
            futures::future::ok::<_, ()>(42),
        );
        let either_future = EitherFuture::from(either);

        let mut runtime = tokio01::runtime::Runtime::new().expect("Failed to create runtime.");
        assert_eq!(Either::Right(42), runtime.block_on(either_future).unwrap());
    }

    #[test]
    fn should_run_left_std_future() {
        let either = Either::<_, ValueFuture<()>>::Left(ValueFuture::new(42));
        let either_future = EitherFuture::from(either);

        let runtime = tokio02::runtime::Runtime::new().expect("Failed to create runtime.");
        assert_eq!(Either::Left(42), runtime.block_on(either_future));
    }

    #[test]
    fn should_run_right_std_future() {
        let either = Either::<ValueFuture<()>, _>::Right(ValueFuture::new(42));
        let either_future = EitherFuture::from(either);

        let runtime = tokio02::runtime::Runtime::new().expect("Failed to create runtime.");
        assert_eq!(Either::Right(42), runtime.block_on(either_future));
    }
}