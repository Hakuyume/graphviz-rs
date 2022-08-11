#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn directVis(
        _: Ppoint_t,
        _: libc::c_int,
        _: Ppoint_t,
        _: libc::c_int,
        _: *mut vconfig_t,
    ) -> bool;
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
static mut unseen: COORD = 2147483647 as libc::c_int as libc::c_double;
unsafe extern "C" fn shortestPath(
    mut root: libc::c_int,
    mut target: libc::c_int,
    mut V: libc::c_int,
    mut wadj: array2,
) -> *mut libc::c_int {
    let mut dad: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vl: *mut COORD = 0 as *mut COORD;
    let mut val: *mut COORD = 0 as *mut COORD;
    let mut min: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    dad = malloc(
        (V as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    vl = malloc(
        ((V + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<COORD>() as libc::c_ulong),
    ) as *mut COORD;
    val = vl.offset(1 as libc::c_int as isize);
    k = 0 as libc::c_int;
    while k < V {
        *dad.offset(k as isize) = -(1 as libc::c_int);
        *val.offset(k as isize) = -unseen;
        k += 1;
    }
    *val.offset(-(1 as libc::c_int) as isize) = -(unseen + 1 as libc::c_int as COORD);
    min = root;
    while min != target {
        k = min;
        let ref mut fresh0 = *val.offset(k as isize);
        *fresh0 *= -(1 as libc::c_int) as libc::c_double;
        min = -(1 as libc::c_int);
        if *val.offset(k as isize) == unseen {
            *val.offset(k as isize) = 0 as libc::c_int as COORD;
        }
        t = 0 as libc::c_int;
        while t < V {
            if *val.offset(t as isize) < 0 as libc::c_int as libc::c_double {
                let mut newpri: COORD = 0.;
                let mut wkt: COORD = 0.;
                if k >= t {
                    wkt = *(*wadj.offset(k as isize)).offset(t as isize);
                } else {
                    wkt = *(*wadj.offset(t as isize)).offset(k as isize);
                }
                newpri = -(*val.offset(k as isize) + wkt);
                if wkt != 0 as libc::c_int as libc::c_double
                    && *val.offset(t as isize) < newpri
                {
                    *val.offset(t as isize) = newpri;
                    *dad.offset(t as isize) = k;
                }
                if *val.offset(t as isize) > *val.offset(min as isize) {
                    min = t;
                }
            }
            t += 1;
        }
    }
    free(vl as *mut libc::c_void);
    return dad;
}
#[no_mangle]
pub unsafe extern "C" fn makePath(
    mut p: Ppoint_t,
    mut pp: libc::c_int,
    mut pvis: *mut COORD,
    mut q: Ppoint_t,
    mut qp: libc::c_int,
    mut qvis: *mut COORD,
    mut conf: *mut vconfig_t,
) -> *mut libc::c_int {
    let mut V: libc::c_int = (*conf).N;
    if directVis(p, pp, q, qp, conf) {
        let mut dad: *mut libc::c_int = malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((V + 2 as libc::c_int) as libc::c_ulong),
        ) as *mut libc::c_int;
        *dad.offset(V as isize) = V + 1 as libc::c_int;
        *dad.offset((V + 1 as libc::c_int) as isize) = -(1 as libc::c_int);
        return dad;
    } else {
        let mut wadj: array2 = (*conf).vis;
        let ref mut fresh1 = *wadj.offset(V as isize);
        *fresh1 = qvis;
        let ref mut fresh2 = *wadj.offset((V + 1 as libc::c_int) as isize);
        *fresh2 = pvis;
        return shortestPath(V + 1 as libc::c_int, V, V + 2 as libc::c_int, wadj);
    };
}
