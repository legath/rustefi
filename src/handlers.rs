
#[no_mangle]
pub unsafe extern "C" fn svc_handler() {}

#[no_mangle]
pub unsafe extern "C" fn prefetch_abort() {}

#[no_mangle]
pub unsafe extern "C" fn undef_entry() {}

/// User defined function for custom actions on Data Abort events
/// Function cannot panic
#[no_mangle]
pub unsafe extern "C" fn custom_dabort() { }

#[no_mangle]
pub unsafe extern "C" fn phantom_interrupt() {
    static mut PHANTOM_INT_COUNTER:u32 = 0;
    PHANTOM_INT_COUNTER += 1;
}
