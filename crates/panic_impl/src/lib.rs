#![no_std]
//#![feature(core_intrinsics)]

#[cfg(not(test))]
#[panic_handler]
pub extern "C" fn panic(_info: &core::panic::PanicInfo) -> ! {
    //unsafe {
        //core::intrinsics::abort()
        loop {

        }
    //}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }