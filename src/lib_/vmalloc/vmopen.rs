#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
pub unsafe extern "C" fn vmopen() -> *mut Vmalloc_t {
    let mut vm: *mut Vmalloc_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Vmalloc_t>() as libc::c_ulong,
    ) as *mut Vmalloc_t;
    if vm.is_null() {
        return 0 as *mut Vmalloc_t;
    }
    return vm;
}
