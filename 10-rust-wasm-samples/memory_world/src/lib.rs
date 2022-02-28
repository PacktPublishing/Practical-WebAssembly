// #![no_std]

// use core::panic::PanicInfo;
// use core::slice::from_raw_parts_mut;

// #[no_mangle]
// fn memory_to_js() {
//     let obj: &mut [u8];

//     unsafe {
//         obj = from_raw_parts_mut::<u8>(0 as *mut u8, 1);
//     }

//     obj[0] = 13;
// }

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> !{
//     loop{}
// }
use std::alloc::{alloc, dealloc,  Layout};
use std::mem;

#[no_mangle]
fn accumulate(data: *mut u8, len: usize) -> i32 {
    let y = unsafe { std::slice::from_raw_parts(data as *const u8, len) };
    let mut sum = 0;
    for i in 0..len {
        sum = sum + y[i];
    }
    sum as i32
}

#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    return ptr
                }
            } else {
                return align as *mut u8
            }
        }
    }
    std::process::abort
}
