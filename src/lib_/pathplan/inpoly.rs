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
    fn wind(a: Ppoint_t, b: Ppoint_t, c: Ppoint_t) -> libc::c_int;
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
pub struct Ppoly_t {
    pub ps: *mut Ppoint_t,
    pub pn: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn in_poly(mut poly: Ppoly_t, mut q: Ppoint_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut P: *mut Ppoint_t = 0 as *mut Ppoint_t;
    P = poly.ps;
    n = poly.pn;
    i = 0 as libc::c_int;
    while i < n {
        i1 = (i + n - 1 as libc::c_int) % n;
        if wind(*P.offset(i1 as isize), *P.offset(i as isize), q) == 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
