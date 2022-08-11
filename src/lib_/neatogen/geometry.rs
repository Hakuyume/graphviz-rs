#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Point {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[no_mangle]
pub static mut origin: Point = {
    let mut init = Point {
        x: 0 as libc::c_int as libc::c_double,
        y: 0 as libc::c_int as libc::c_double,
    };
    init
};
#[no_mangle]
pub static mut xmin: libc::c_double = 0.;
#[no_mangle]
pub static mut xmax: libc::c_double = 0.;
#[no_mangle]
pub static mut ymin: libc::c_double = 0.;
#[no_mangle]
pub static mut ymax: libc::c_double = 0.;
#[no_mangle]
pub static mut deltax: libc::c_double = 0.;
#[no_mangle]
pub static mut deltay: libc::c_double = 0.;
#[no_mangle]
pub static mut nsites: libc::c_int = 0;
#[no_mangle]
pub static mut sqrt_nsites: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn geominit() {
    let mut sn: libc::c_double = 0.;
    sn = (nsites + 4 as libc::c_int) as libc::c_double;
    sqrt_nsites = sqrt(sn) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dist_2(
    mut pp: *mut Point,
    mut qp: *mut Point,
) -> libc::c_double {
    let mut dx: libc::c_double = (*pp).x - (*qp).x;
    let mut dy: libc::c_double = (*pp).y - (*qp).y;
    return dx * dx + dy * dy;
}
#[no_mangle]
pub unsafe extern "C" fn subpt(mut a: *mut Point, mut b: Point, mut c: Point) {
    (*a).x = b.x - c.x;
    (*a).y = b.y - c.y;
}
#[no_mangle]
pub unsafe extern "C" fn addpt(mut c: *mut Point, mut a: Point, mut b: Point) {
    (*c).x = a.x + b.x;
    (*c).y = a.y + b.y;
}
#[no_mangle]
pub unsafe extern "C" fn area_2(
    mut a: Point,
    mut b: Point,
    mut c: Point,
) -> libc::c_double {
    return (a.y - b.y) * (c.x - b.x) - (c.y - b.y) * (a.x - b.x);
}
#[no_mangle]
pub unsafe extern "C" fn leftOf(
    mut a: Point,
    mut b: Point,
    mut c: Point,
) -> libc::c_int {
    return (area_2(a, b, c) > 0 as libc::c_int as libc::c_double) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn intersection(
    mut a: Point,
    mut b: Point,
    mut c: Point,
    mut d: Point,
    mut p: *mut Point,
) -> libc::c_int {
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut denom: libc::c_double = 0.;
    denom = a.x * (d.y - c.y) + b.x * (c.y - d.y) + d.x * (b.y - a.y)
        + c.x * (a.y - b.y);
    if denom == 0.0f64 {
        return 0 as libc::c_int;
    }
    s = (a.x * (d.y - c.y) + c.x * (a.y - d.y) + d.x * (c.y - a.y)) / denom;
    t = -(a.x * (c.y - b.y) + b.x * (a.y - c.y) + c.x * (b.y - a.y)) / denom;
    (*p).x = a.x + s * (b.x - a.x);
    (*p).y = a.y + s * (b.y - a.y);
    if 0.0f64 <= s && s <= 1.0f64 && 0.0f64 <= t && t <= 1.0f64 {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
