use alloc::alloc::GlobalAlloc;
use alloc::alloc::Layout;

use crate::allocator::align_up;
use crate::locked::Locked;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Create a new empty bump allocator
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initializes the bumb allocator with the given heap bounds.
    ///
    /// This method is unsafe bevause the caller must ensure that the given
    /// memory range is unused. Also this methd must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(e) => e,
            None => return core::ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            core::ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut bump = self.lock();
        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
