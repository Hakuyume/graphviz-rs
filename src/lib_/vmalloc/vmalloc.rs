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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
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
unsafe extern "C" fn make_space(mut vm: *mut Vmalloc_t) -> bool {
    if (*vm).size == (*vm).capacity {
        let mut c: size_t = if (*vm).capacity == 0 as libc::c_int as libc::c_ulong {
            1 as libc::c_int as libc::c_ulong
        } else {
            ((*vm).capacity).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        };
        let mut p: *mut *mut libc::c_void = realloc(
            (*vm).allocated as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong).wrapping_mul(c),
        ) as *mut *mut libc::c_void;
        if p.is_null() {
            return 0 as libc::c_int != 0;
        }
        let ref mut fresh0 = (*vm).allocated;
        *fresh0 = p;
        (*vm).capacity = c;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn vmalloc(mut vm: *mut Vmalloc_t, mut size: size_t) -> *mut libc::c_void {
    if !make_space(vm) {
        return 0 as *mut libc::c_void;
    }
    let mut p: *mut libc::c_void = malloc(size);
    if p.is_null() {
        return 0 as *mut libc::c_void;
    }
    let ref mut fresh1 = *((*vm).allocated).offset((*vm).size as isize);
    *fresh1 = p;
    let ref mut fresh2 = (*vm).size;
    *fresh2 = (*fresh2).wrapping_add(1);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn vmfree(mut vm: *mut Vmalloc_t, mut data: *mut libc::c_void) {
    if data.is_null() {
        return;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*vm).size {
        if *((*vm).allocated).offset(i as isize) == data {
            let mut extent: size_t = (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(
                    ((*vm).size)
                        .wrapping_sub(i)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            memmove(
                ((*vm).allocated).offset(i as isize) as *mut libc::c_void,
                ((*vm).allocated)
                    .offset(i as isize)
                    .offset(1 as libc::c_int as isize) as *const libc::c_void,
                extent,
            );
            let ref mut fresh3 = (*vm).size;
            *fresh3 = (*fresh3).wrapping_sub(1);
            free(data);
            return;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vmresize(
    mut vm: *mut Vmalloc_t,
    mut data: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    if data.is_null() {
        return vmalloc(vm, size);
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*vm).size {
        if *((*vm).allocated).offset(i as isize) == data {
            let mut p: *mut libc::c_void = realloc(data, size);
            if p.is_null() {
                return p;
            }
            let ref mut fresh4 = *((*vm).allocated).offset(i as isize);
            *fresh4 = p;
            return p;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut libc::c_void;
}
