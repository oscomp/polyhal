use core::{ptr::NonNull, sync::atomic::AtomicBool};

use arrayvec::ArrayVec;
use fdt_parser::{Fdt, FdtError};

use crate::{arch::MEM_VECTOR_CAPACITY, consts::VIRT_ADDR_START, utils::BootLock, PhysAddr};

/// Boot Information
pub static BOOT_INFO: BootLock<BootInfo> = BootLock::new(BootInfo::new());

/// Memory Region Information
pub struct BootInfo {
    /// available memory regions
    pub available: ArrayVec<(usize, usize), MEM_VECTOR_CAPACITY>,
    /// reserved memory regions
    pub reserved: ArrayVec<(usize, usize), MEM_VECTOR_CAPACITY>,
    /// Device Tree Binary Pointer and its size
    pub dtb_ptr: Option<(PhysAddr, usize)>,
    /// The count of cpu core
    pub cpu_num: usize,
}

impl BootInfo {
    /// Create a new [BootInfo]
    pub const fn new() -> Self {
        Self {
            available: ArrayVec::new_const(),
            reserved: ArrayVec::new_const(),
            dtb_ptr: None,
            cpu_num: 1,
        }
    }

    pub fn init_once(&mut self) {
        static INITED: AtomicBool = AtomicBool::new(false);
        if INITED.swap(true, core::sync::atomic::Ordering::SeqCst) {
            return;
        }
        extern "C" {
            fn _skernel();
            fn _end();
        }
        self.reserved.push((
            _skernel as usize - VIRT_ADDR_START,
            _end as usize - VIRT_ADDR_START,
        ));
    }

    pub fn parse_dtb(&mut self, dtb_ptr: PhysAddr) -> Result<(), FdtError<'static>> {
        self.init_once();
        let ptr = NonNull::new(dtb_ptr.get_mut_ptr()).unwrap();
        let fdt = Fdt::from_ptr(ptr)?;
        self.dtb_ptr = Some((dtb_ptr, fdt.total_size()));
        self.reserved
            .push((dtb_ptr.raw(), dtb_ptr.raw() + fdt.total_size()));

        // Parse Memory Information From dtb
        fdt.memory()
            .flat_map(|x| x.regions())
            .for_each(|mm| unsafe {
                #[cfg(not(target_arch = "riscv64"))]
                self.add_memory_region(mm.address as _, mm.address as usize + mm.size);
                #[cfg(target_arch = "riscv64")]
                {
                    let mut start = mm.address as _;
                    let end = mm.address as usize + mm.size;

                    // TODO: using dynamic to skip memory
                    start += 0x200_000;

                    self.add_memory_region(start, end);
                }
            });
        self.cpu_num = fdt.find_nodes("/cpus/cpu").count();
        Ok(())
    }

    /// Adds a memory region.
    ///
    /// # Parameters
    /// - `start` - The starting address of the memory region.
    /// - `end` - The ending address of the memory region.
    ///
    /// # Safety
    ///
    /// - This function must be called from a single thread; concurrent access is **not** safe.
    /// - The caller must ensure that [MEM_VECTOR_CAPACITY] is sufficient to accommodate the memory region,  
    ///   otherwise, this function may result in out-of-bounds memory access or undefined behavior.
    pub unsafe fn add_memory_region(&mut self, start: usize, end: usize) {
        if end - start == 0 {
            return;
        }
        for &(rstart, rend) in self.reserved.iter() {
            // return if the whole region was reserved
            if rstart <= start && end <= rend {
                return;
            }
            // split if the partial region was reserved
            if start <= rstart && rend <= end {
                self.add_memory_region(start, rstart);
                self.add_memory_region(rend, end);
                return;
            }
        }
        self.available.push((start, end - start));
    }

    /// Allocate Memory From [BootInfo]
    ///
    /// # Safety
    ///
    /// - Ensure call this function in the primary core when booting
    /// - Ensure no alignment required
    pub fn alloc(&mut self, alloc_size: usize) -> *mut u8 {
        for (start, size) in self.available.iter_mut() {
            if *size > alloc_size {
                let ptr = *start;
                *start += alloc_size;
                *size -= alloc_size;
                return ptr as _;
            }
        }
        unreachable!()
    }
}
