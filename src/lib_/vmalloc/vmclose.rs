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
    fn vmclear(_: *mut Vmalloc_t) -> libc::c_int;
    fn free(_: *mut libc::c_void);
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
pub unsafe extern "C" fn vmclose(mut vm: *mut Vmalloc_t) -> libc::c_int {
    let mut r: libc::c_int = vmclear(vm);
    if r != 0 as libc::c_int {
        return r;
    }
    free(vm as *mut libc::c_void);
    return 0 as libc::c_int;
}
