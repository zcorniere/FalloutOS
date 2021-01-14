pub mod allocator;
pub mod locked;

mod frame_allocator;
pub use self::frame_allocator::BootInfoFrameAllocator;

use x86_64::{
    registers::control::Cr3,
    structures::paging::{OffsetPageTable, PageTable},
    VirtAddr,
};

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init(physical_mem_off: VirtAddr) -> OffsetPageTable<'static> {
    let active_lvl4_table = active_lvl4_table(physical_mem_off);
    OffsetPageTable::new(active_lvl4_table, physical_mem_off)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_lvl4_table(physical_mem_off: VirtAddr) -> &'static mut PageTable {
    let (lvl4, _) = Cr3::read();
    let phys = lvl4.start_address();
    let virt = physical_mem_off + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}