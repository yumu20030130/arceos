#![feature(asm_const)]
#![no_std]
#![no_main]

use core::mem;
const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: usize = 0;

unsafe fn syscall(abi_num: usize, arg0: usize) -> isize {
    type FunctionType = fn(usize);
    unsafe {
        // 将 usize 转换为函数指针并调用
        // 记得做取值操作：   对应 ld  t1, (t1)
        let func_ptr: FunctionType = mem::transmute(*((ABI_TABLE + abi_num * 8) as *const usize));
        func_ptr(arg0)
    }
    // core::arch::asm!("
    //     slli    t0, t0, 3
    //     add     t1, a7, t0
    //     ld      t1, (t1)
    //     jalr    t1
    //     ",
    //     in("a7") ABI_TABLE,
    //     in("t0") abi_num,
    //     in("a0") arg0,
    // );
    0
}

fn hello() -> isize {
    unsafe { syscall(SYS_HELLO, 0) }
}

fn putchar(c: char) -> isize {
    unsafe { syscall(SYS_PUTCHAR, c as usize) }
}

fn terminate() -> isize {
    unsafe { syscall(SYS_TERMINATE, 0) }
}

fn puts(s: &str) -> isize {
    for c in s.chars() {
        putchar(c);
    }
    putchar('\n');
    0
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "mv {0}, a7",    // 将 a7 寄存器的值移到ABI_TABLE
            out(reg) ABI_TABLE,  
        );
    }
    hello();
    puts("puts");
    terminate();
    loop { }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
