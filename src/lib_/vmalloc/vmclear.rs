#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
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
pub unsafe extern "C" fn vmclear(mut vm: *mut Vmalloc_t) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*vm).size {
        free(*((*vm).allocated).offset(i as isize));
        i = i.wrapping_add(1);
    }
    free((*vm).allocated as *mut libc::c_void);
    let ref mut fresh0 = (*vm).allocated;
    *fresh0 = 0 as *mut *mut libc::c_void;
    let ref mut fresh1 = (*vm).capacity;
    *fresh1 = 0 as libc::c_int as size_t;
    (*vm).size = *fresh1;
    return 0 as libc::c_int;
}
