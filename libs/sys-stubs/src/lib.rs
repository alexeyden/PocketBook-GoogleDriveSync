use std::ffi::*;

#[no_mangle]
pub unsafe extern "C" fn accept4(
    fd: c_int,
    addr: *mut libc::sockaddr,
    len: *mut libc::socklen_t,
    _flg: c_int
) -> c_int {
    libc::accept(fd, addr, len)
}

#[no_mangle]
pub unsafe extern "C" fn pipe2(fds: *mut c_int, _flags: c_int) -> c_int {
    libc::pipe(fds)
}

#[no_mangle]
pub unsafe extern "C" fn pthread_setname_np(
    _thread: libc::pthread_t,
    _name: *const c_char
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn getauxval(_type_: c_ulong) -> c_ulong {
    0
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr1() {}

#[no_mangle]
pub unsafe extern "C" fn __gnu_unwind_frame() {
}

#[no_mangle]
pub unsafe extern "C" fn _Unwind_GetRegionStart() {}
#[no_mangle]
pub unsafe extern "C" fn _Unwind_Backtrace() {}

#[no_mangle] pub unsafe extern "C" fn _Unwind_VRS_Set() {}
#[no_mangle] pub unsafe extern "C" fn _Unwind_VRS_Get() {}
#[no_mangle] pub unsafe extern "C" fn _Unwind_Resume() {}
#[no_mangle] pub unsafe extern "C" fn _Unwind_GetTextRelBase() {}
#[no_mangle] pub unsafe extern "C" fn _Unwind_GetDataRelBase() {}
#[no_mangle] pub unsafe extern "C" fn _Unwind_GetLanguageSpecificData() {}
