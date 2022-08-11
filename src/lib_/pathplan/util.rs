#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub type Ppolyline_t = Ppoly_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pedge_t {
    pub a: Ppoint_t,
    pub b: Ppoint_t,
}
#[no_mangle]
pub unsafe extern "C" fn freePath(mut p: *mut Ppolyline_t) {
    free((*p).ps as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Ppolybarriers(
    mut polys: *mut *mut Ppoly_t,
    mut npolys: libc::c_int,
    mut barriers: *mut *mut Pedge_t,
    mut n_barriers: *mut libc::c_int,
) -> libc::c_int {
    let mut pp: Ppoly_t = Ppoly_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut bar: *mut Pedge_t = 0 as *mut Pedge_t;
    n = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < npolys {
        n = n + (**polys.offset(i as isize)).pn;
        i += 1;
    }
    bar = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Pedge_t>() as libc::c_ulong),
    ) as *mut Pedge_t;
    b = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < npolys {
        pp = **polys.offset(i as isize);
        j = 0 as libc::c_int;
        while j < pp.pn {
            k = j + 1 as libc::c_int;
            if k >= pp.pn {
                k = 0 as libc::c_int;
            }
            (*bar.offset(b as isize)).a = *(pp.ps).offset(j as isize);
            (*bar.offset(b as isize)).b = *(pp.ps).offset(k as isize);
            b += 1;
            j += 1;
        }
        i += 1;
    }
    if b == n {} else {
        __assert_fail(
            b"b == n\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int Ppolybarriers(Ppoly_t **, int, Pedge_t **, int *)\0"))
                .as_ptr(),
        );
    }
    *barriers = bar;
    *n_barriers = n;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn make_polyline(
    mut line: Ppolyline_t,
    mut sline: *mut Ppolyline_t,
) {
    static mut isz: libc::c_int = 0 as libc::c_int;
    static mut ispline: *mut Ppoint_t = 0 as *const Ppoint_t as *mut Ppoint_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut npts: libc::c_int = 4 as libc::c_int
        + 3 as libc::c_int * (line.pn - 2 as libc::c_int);
    if npts > isz {
        ispline = realloc(
            ispline as *mut libc::c_void,
            (npts as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<Ppoint_t>() as libc::c_ulong),
        ) as *mut Ppoint_t;
        isz = npts;
    }
    i = 0 as libc::c_int;
    j = i;
    let ref mut fresh0 = *ispline.offset(j as isize);
    *fresh0 = *(line.ps).offset(i as isize);
    *ispline.offset((j + 1 as libc::c_int) as isize) = *fresh0;
    j += 2 as libc::c_int;
    i += 1;
    while i < line.pn - 1 as libc::c_int {
        let ref mut fresh1 = *ispline.offset(j as isize);
        *fresh1 = *(line.ps).offset(i as isize);
        let ref mut fresh2 = *ispline.offset((j + 1 as libc::c_int) as isize);
        *fresh2 = *fresh1;
        *ispline.offset((j + 2 as libc::c_int) as isize) = *fresh2;
        j += 3 as libc::c_int;
        i += 1;
    }
    let ref mut fresh3 = *ispline.offset(j as isize);
    *fresh3 = *(line.ps).offset(i as isize);
    *ispline.offset((j + 1 as libc::c_int) as isize) = *fresh3;
    (*sline).pn = npts;
    let ref mut fresh4 = (*sline).ps;
    *fresh4 = ispline;
}
