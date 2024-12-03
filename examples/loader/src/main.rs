#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;
const APP_NUM: usize = 2;
const BATCH_SIZE: usize = 1024 * 1024; // 1MB 暂不支持大于1MB的应用
pub struct ImageHeader {
    files_size: [u8; APP_NUM],
}
// 第一个块固定用来存ImageHeader，后面一个块存放一个应用
// 要配合对应指令把应用写到对应的位置，才能正常运行
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {

    let image_header = unsafe { core::slice::from_raw_parts(PLASH_START as *const u8, APP_NUM)};
    let image_header = image_header.as_ptr() as *const ImageHeader;

    let apps_start = PLASH_START + BATCH_SIZE;

    println!("Load payload ...");
    let mut cur_offset = apps_start;
    for i in 0..APP_NUM {
        let file_size = unsafe { (*image_header).files_size[i] as usize };
        let code = unsafe { core::slice::from_raw_parts(cur_offset as *const u8, file_size)};
        cur_offset += BATCH_SIZE;
        println!("app index: {:?}: ", i);
        println!("app size: {:?}: ", file_size);
        println!("content: {:?}: ", code);
    }

    println!("Load payload ok!");
}