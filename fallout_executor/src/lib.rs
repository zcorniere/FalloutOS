#![no_std]
#![feature(wake_trait)]

extern crate alloc;

mod executor;
pub mod keyboard;
pub mod simple_executor;
pub mod task;
pub use self::executor::Executor;
pub use self::task::Task;

use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
