use super::TaskId;
use alloc::boxed::Box;
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    pub id: TaskId,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Task {
            future: Box::pin(future),
            id: TaskId::new(),
        }
    }

    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
