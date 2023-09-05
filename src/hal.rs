use crate::{GuestPageTableTrait, HostPageNum, HostPhysAddr, HostVirtAddr, memory::PAGE_SIZE_4K};

/// The interfaces which the underlginh software(kernel or hypervisor) must implement.
pub trait HyperCraftHal: Sized {
    const PAGE_SIZE: usize = PAGE_SIZE_4K;

    /// Allocates a 4K-sized contiguous physical page, returns its physical address.
    fn alloc_page() -> Option<HostPhysAddr> {
        Self::alloc_pages(1)
    }
    /// Deallocates the given physical page.
    fn dealloc_page(pa: HostPhysAddr) {
        Self::dealloc_pages(pa, 1)
    }
    /// Allocates a 16K-sized & 16K-align physical page, uesd in root page table.
    #[cfg(target_arch = "riscv64")]
    fn alloc_16_page() -> Option<HostPageNum> {
        Self::alloc_pages(4)
    }
    /// Deallocates the given 16K-sized physical page.
    #[cfg(target_arch = "riscv64")]
    fn dealloc_16_page(ppn: HostPageNum) {
        Self::dealloc_pages(ppn, 4)
    }
    /// Allocates contiguous pages, returns its physical address.
    fn alloc_pages(num_pages: usize) -> Option<HostPhysAddr>;
    /// Gives back the allocated pages starts from `pa` to the page allocator.
    fn dealloc_pages(pa: HostPhysAddr, num_pages: usize);
    // /// VM-Exit handler
    // fn vmexit_handler(vcpu: &mut crate::VCpu<Self>, vm_exit_info: VmExitInfo);

    /// Convert a host physical address to host virtual address.
    #[cfg(target_arch = "x86_64")]
    fn phys_to_virt(pa: HostPhysAddr) -> HostVirtAddr;
    /// Convert a host virtual address to host physical address.
    #[cfg(target_arch = "x86_64")]
    fn virt_to_phys(va: HostVirtAddr) -> HostPhysAddr;
}
