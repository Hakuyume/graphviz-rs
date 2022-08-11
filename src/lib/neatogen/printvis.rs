#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pxy_t {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type Ppoint_t = Pxy_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vconfig_s {
    pub Npoly: libc::c_int,
    pub N: libc::c_int,
    pub P: *mut Ppoint_t,
    pub start: *mut libc::c_int,
    pub next: *mut libc::c_int,
    pub prev: *mut libc::c_int,
    pub vis: array2,
}
pub type array2 = *mut *mut COORD;
pub type COORD = libc::c_double;
pub type vconfig_t = vconfig_s;
pub type point = Ppoint_t;
#[no_mangle]
pub unsafe extern "C" fn printvis(mut cp: *mut vconfig_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut next: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pts: *mut point = 0 as *mut point;
    let mut arr: array2 = 0 as *mut *mut COORD;
    next = (*cp).next;
    prev = (*cp).prev;
    pts = (*cp).P;
    arr = (*cp).vis;
    printf(b"this next prev point\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*cp).N {
        printf(
            b"%3d  %3d  %3d    (%f,%f)\n\0" as *const u8 as *const libc::c_char,
            i,
            *next.offset(i as isize),
            *prev.offset(i as isize),
            (*pts.offset(i as isize)).x,
            (*pts.offset(i as isize)).y,
        );
        i += 1;
    }
    printf(b"\n\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*cp).N {
        j = 0 as libc::c_int;
        while j < (*cp).N {
            printf(
                b"%4.1f \0" as *const u8 as *const libc::c_char,
                *(*arr.offset(i as isize)).offset(j as isize),
            );
            j += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
