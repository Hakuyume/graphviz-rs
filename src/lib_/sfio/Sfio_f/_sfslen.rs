#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut _Sfi: ssize_t;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[no_mangle]
pub unsafe extern "C" fn sfslen() -> ssize_t {
    return _Sfi;
}
