#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn vmalloc(vm: *mut Vmalloc_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vmalloc_s {
    pub allocated: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub type Vmalloc_t = _vmalloc_s;
#[no_mangle]
pub unsafe extern "C" fn vmstrdup(
    mut v: *mut Vmalloc_t,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut t: *mut libc::c_char = vmalloc(v, len) as *mut libc::c_char;
    if t.is_null() {
        return 0 as *mut libc::c_char;
    }
    memcpy(t as *mut libc::c_void, s as *const libc::c_void, len);
    return t;
}
