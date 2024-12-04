#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

use core::ptr::null;

#[cfg(feature = "axstd")]
use axstd::println;

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
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_terminate() {
    println!("[ABI:Terminate] Terminate Apps!");
    exit(0);
}


const PLASH_START: usize = 0xffff_ffc0_2200_0000;
const APP_NUM: usize = 2;
const BATCH_SIZE: usize = 1024 * 1024; // 1MB 暂不支持大于1MB的应用
pub struct ImageHeader {
    files_size: [u8; APP_NUM],
}
// 第一个块固定用来存ImageHeader，后面每个块存放一个应用
// 要配合对应指令把应用写到对应的位置，才能正常运行
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {

    let image_header = unsafe { core::slice::from_raw_parts(PLASH_START as *const u8, APP_NUM)};
    let image_header = image_header.as_ptr() as *const ImageHeader;

    let apps_start = PLASH_START + BATCH_SIZE;
    let mut load_codes: [&[u8]; APP_NUM] = [&[]; APP_NUM];
    let mut files_size = 2; // 文件最后要有两个零字节比较好
    println!("Load payload ...");
    let mut cur_offset = apps_start;
    for i in 0..APP_NUM {
        let file_size = unsafe { (*image_header).files_size[i] as usize }; 
        load_codes[i] = unsafe { core::slice::from_raw_parts(cur_offset as *const u8, file_size - 2)}; //bin文件最后两个字节不要了 
        cur_offset += BATCH_SIZE;
        println!("app index: {:?}: ", i);
        println!("app size: {:?}: ", file_size);
        println!("content: {:?}: ", load_codes[i]);
        files_size += file_size - 2;
    }
    println!("Load payload ok!");

    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // va_pa_offset: 0xffff_ffc0_0000_0000
    const RUN_START: usize = 0xffff_ffc0_8010_0000;
    let run_code = unsafe {
        core::slice::from_raw_parts_mut(RUN_START as *mut u8, files_size)
    };
    let mut index = 0;
    for i in 0..APP_NUM {
        run_code[index..index + load_codes[i].len()].copy_from_slice(load_codes[i]);
        index += load_codes[i].len();
    }
    println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
    println!("Load payload ok!");


    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

	println!("Execute app ...");
    let arg0: u8 = b'A';


    // // execute app
    // unsafe { core::arch::asm!("
    //     li      t2, {run_start}
    //     jalr    t2
    //     j       .",
    //     run_start = const RUN_START,
    // )}

    // execute app
    // jalr 是跳转并链接指令 会把下一指令的地址保存到 ra 寄存器，函数跳转的时候就要用这个，否则函数结束会跳转到错误的地址
    // 函数编译出来时编译器会在函数的最前和最后插入一些汇编指令，以保证结束时跳转回ra寄存器指向的指令地址
    unsafe { core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        la      t1, {abi_table}
        add     t1, t1, t0
        ld      t1, (t1)
        jalr    t1 
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
        //abi_num = const SYS_HELLO,
        // abi_num = const SYS_PUTCHAR,
        abi_num = const SYS_TERMINATE,
        in("a0") arg0,
    )}
}