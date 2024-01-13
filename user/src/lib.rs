//! lib.rs 作为 bin 目录下的源程序所依赖的用户库，等价于其他编程语言提供的标准库。
#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

/// 用户库的入口点
/// Rust 的宏将 _start 这段代码编译后的汇编代码中放在一个名为 .text.entry 的代码段中
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

/// #![feature(linkage)]: 我们使用 Rust 的宏将其函数符号 [`main`] 标志为弱链接。这样在最后链接的时候，
/// 虽然在 lib.rs 和 bin 目录下的某个应用程序都有 [`main`] 符号，但由于 lib.rs 中的 [`main`] 符号是
/// 弱链接，链接器会使用 bin 目录下的应用主逻辑作为 [`main`] 。这里我们主要是进行某种程度上的保护，如果在 
/// bin 目录下找不到任何 [`main`] ，那么编译也能够通过，但会在运行时报错。
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

// 手动清空需要零初始化的 .bss 段
fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
