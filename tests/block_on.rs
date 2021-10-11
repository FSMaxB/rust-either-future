#![cfg(feature = "std_future")]
//! KISS implementation of block_on to run a future in an infinite loop until it is finished.
//! This is required because dev-dependencies cannot be conditional based on features and
//! older rust versions do not have std::future::Future, so including a dependency that
//! contains a small future executor for standard futures won't work on older rust versions.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub fn block_on<F, O>(mut future: F) -> O
where
	F: Future<Output = O>,
{
	let raw_waker = RawWaker::new(0 as *const (), &NOOP_WAKER_VTABLE);
	let waker = unsafe { Waker::from_raw(raw_waker) };
	let mut context = Context::from_waker(&waker);

	loop {
		if let Poll::Ready(value) = unsafe { Pin::new_unchecked(&mut future) }.poll(&mut context) {
			break value;
		}
	}
}

const NOOP_WAKER_VTABLE: RawWakerVTable =
	RawWakerVTable::new(clone_noop_waker, std::mem::drop, std::mem::drop, std::mem::drop);

fn clone_noop_waker(pointer: *const ()) -> RawWaker {
	RawWaker::new(pointer, &NOOP_WAKER_VTABLE)
}
