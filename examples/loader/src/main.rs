#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

use core::ptr::null;

#[cfg(feature = "axstd")]
use axstd::println;

#[cfg(feature = "axstd")]
use axstd::print;

#[cfg(feature = "axstd")]
use axstd::process::exit;


const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;
static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("Hello, Apps!");
}

fn abi_putchar(c: char) {
    print!("{c}");
}

fn abi_terminate() {
    println!("[ABI:Terminate] Terminate Apps!");
    exit(0);
}


const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let load_start = PLASH_START as *const u8;
    let load_size = 32000; // Dangerous!!! We need to get accurate size of apps.

    println!("Load payload ...");

    let load_code = unsafe { core::slice::from_raw_parts(load_start, load_size) };
    // println!("load code {:?}; address [{:?}]", load_code, load_code.as_ptr());

    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // va_pa_offset: 0xffff_ffc0_0000_0000
    const RUN_START: usize = 0xffff_ffc0_8010_0000;

    let run_code = unsafe {
        core::slice::from_raw_parts_mut(RUN_START as *mut u8, load_size)
    };
    for i in 0..load_size {
        run_code[i] = 0;
    }
    run_code.copy_from_slice(load_code);
    println!("run code address [{:?}]", run_code.as_ptr());

    println!("Load payload ok!");
    println!("ABI Table: {:?}", unsafe {ABI_TABLE.as_ptr() });

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);


    // // execute app
    // unsafe { core::arch::asm!("
    //     li      t2, {run_start}
    //     jalr    t2
    //     j       .",
    //     run_start = const RUN_START,
    // )}

    println!("Execute app ...");

    // execute app
    unsafe { core::arch::asm!("
        la      a7, {abi_table}
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START + 0x1004,
        abi_table = sym ABI_TABLE,
    )}
}