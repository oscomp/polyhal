use core::ptr::NonNull;

use fdt_parser::{Fdt, FdtError};

use crate::{common::CPU_NUM, info::BOOT_INFO};

/// Get Flattened Device Tree
pub fn get_fdt() -> Result<Fdt<'static>, FdtError<'static>> {
    if !BOOT_INFO.dtb_ptr.is_none() {
        return Err(FdtError::BadPtr);
    }
    unsafe {
        Fdt::from_ptr(NonNull::new_unchecked(
            BOOT_INFO.dtb_ptr.unwrap().0.get_mut_ptr(),
        ))
    }
}

/// Allocate Memory From [MEM_AREA]
///
/// # Safety
///
/// - Ensure call this function in the primary core when booting
/// - Ensure no alignment required
pub unsafe fn alloc(alloc_size: usize) -> *mut u8 {
    BOOT_INFO.get_mut().alloc(alloc_size)
}

/// Parse Information from the device tree binary or Multiboot
///
/// Display information when booting
/// Initialize the variables and memory from device tree
#[inline]
pub fn parse_system_info() {
    display_info!();
    println!(include_str!("./banner.txt"));
    display_info!("Platform Arch", "{}", env!("HAL_ENV_ARCH"));
    if let Ok(fdt) = get_fdt() {
        display_info!("Boot HART ID", "{}", fdt.boot_cpuid_phys());
        display_info!("Boot HART Count", "{}", fdt.find_nodes("/cpus/cpu").count());
        CPU_NUM.init_once(fdt.find_nodes("/cpus/cpu").count());
        fdt.chosen().inspect(|chosen| {
            display_info!("Boot Args", "{}", chosen.bootargs().unwrap_or(""));
        });
        fdt.memory().flat_map(|x| x.regions()).for_each(|mm| {
            display_info!(
                "Platform Memory Region",
                "{:#p} - {:#018x}",
                mm.address,
                mm.address as usize + mm.size
            );
        });
    }
    get_mem_areas().for_each(|(address, size)| {
        display_info!(
            "Platform Memory Available",
            "{:#018x} - {:#018x}",
            address,
            address + size
        );
    });
}

/// Retrieves an iterator over the registered memory areas.
///
/// # Returns
///
/// An iterator yielding references to tuples `(start, end)`, where:
/// - `start` is the starting address of a memory area.
/// - `end` is the ending address of a memory area.
///
/// # Safety
///
/// - The caller must ensure that `MEM_AREA` is properly initialized before calling this function.
/// - Since this function returns an iterator over a static memory region, concurrent modification  
///   of `MEM_AREA` while iterating may lead to undefined behavior.
pub fn get_mem_areas<'a>() -> impl Iterator<Item = &'a (usize, usize)> {
    BOOT_INFO.available.iter()
}
