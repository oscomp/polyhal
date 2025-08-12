#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(cfg_version)]
#![feature(decl_macro)]
#![feature(used_with_arg)]
#![cfg_attr(target_arch = "riscv64", feature(riscv_ext_intrinsics))]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

// extern crate alloc;
extern crate log;

#[macro_use]
pub mod ctor;
#[macro_use]
pub mod debug_console;
#[macro_use]
pub mod utils;
pub mod info;

pub mod arch;
pub use arch::consts;
pub mod barrier;
pub mod common;
pub mod instruction;
pub mod irq;
pub mod kcontext;
pub mod mem;
pub mod multicore;
pub mod pagetable;
pub mod percpu;
pub mod timer;

pub use utils::addr::{PhysAddr, VirtAddr};

#[cfg(feature = "boot")]
pub use polyhal_macro::arch_entry;
#[cfg(feature = "trap")]
pub use polyhal_macro::arch_interrupt;
pub use polyhal_macro::percpu;

// Re export the Module like Structure.
pub use pagetable::{MappingFlags, MappingSize, PageTable, PageTableWrapper};
