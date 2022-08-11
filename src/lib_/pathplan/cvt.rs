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
#![feature(label_break_value, register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn visibility(_: *mut vconfig_t);
    fn makePath(
        p: Ppoint_t,
        pp: libc::c_int,
        pvis: *mut COORD,
        q: Ppoint_t,
        qp: libc::c_int,
        qvis: *mut COORD,
        conf: *mut vconfig_t,
    ) -> *mut libc::c_int;
    fn ptVis(_: *mut vconfig_t, _: libc::c_int, _: Ppoint_t) -> *mut COORD;
}
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn mymalloc(mut newsize: size_t) -> *mut libc::c_void {
    let mut rv: *mut libc::c_void = 0 as *mut libc::c_void;
    if newsize > 0 as libc::c_int as libc::c_ulong {
        rv = malloc(newsize);
    } else {
        rv = 0 as *mut libc::c_void;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn Pobsopen(
    mut obs: *mut *mut Ppoly_t,
    mut n_obs: libc::c_int,
) -> *mut vconfig_t {
    let mut rv: *mut vconfig_t = 0 as *mut vconfig_t;
    let mut poly_i: libc::c_int = 0;
    let mut pt_i: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    rv = malloc(::std::mem::size_of::<vconfig_t>() as libc::c_ulong) as *mut vconfig_t;
    if rv.is_null() {
        return 0 as *mut vconfig_t;
    }
    n = 0 as libc::c_int;
    poly_i = 0 as libc::c_int;
    while poly_i < n_obs {
        n += (**obs.offset(poly_i as isize)).pn;
        poly_i += 1;
    }
    let ref mut fresh0 = (*rv).P;
    *fresh0 = mymalloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Ppoint_t>() as libc::c_ulong),
    ) as *mut Ppoint_t;
    let ref mut fresh1 = (*rv).start;
    *fresh1 = mymalloc(
        ((n_obs + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let ref mut fresh2 = (*rv).next;
    *fresh2 = mymalloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let ref mut fresh3 = (*rv).prev;
    *fresh3 = mymalloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*rv).N = n;
    (*rv).Npoly = n_obs;
    i = 0 as libc::c_int;
    poly_i = 0 as libc::c_int;
    while poly_i < n_obs {
        start = i;
        *((*rv).start).offset(poly_i as isize) = start;
        end = start + (**obs.offset(poly_i as isize)).pn - 1 as libc::c_int;
        pt_i = 0 as libc::c_int;
        while pt_i < (**obs.offset(poly_i as isize)).pn {
            *((*rv).P).offset(i as isize) =
                *((**obs.offset(poly_i as isize)).ps).offset(pt_i as isize);
            *((*rv).next).offset(i as isize) = i + 1 as libc::c_int;
            *((*rv).prev).offset(i as isize) = i - 1 as libc::c_int;
            i += 1;
            pt_i += 1;
        }
        *((*rv).next).offset(end as isize) = start;
        *((*rv).prev).offset(start as isize) = end;
        poly_i += 1;
    }
    *((*rv).start).offset(poly_i as isize) = i;
    visibility(rv);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn Pobsclose(mut config: *mut vconfig_t) {
    free((*config).P as *mut libc::c_void);
    free((*config).start as *mut libc::c_void);
    free((*config).next as *mut libc::c_void);
    free((*config).prev as *mut libc::c_void);
    if !((*config).vis).is_null() {
        free(*((*config).vis).offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free((*config).vis as *mut libc::c_void);
    }
    free(config as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Pobspath(
    mut config: *mut vconfig_t,
    mut p0: Ppoint_t,
    mut poly0: libc::c_int,
    mut p1: Ppoint_t,
    mut poly1: libc::c_int,
    mut output_route: *mut Ppolyline_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dad: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut opn: size_t = 0;
    let mut ops: *mut Ppoint_t = 0 as *mut Ppoint_t;
    let mut ptvis0: *mut COORD = 0 as *mut COORD;
    let mut ptvis1: *mut COORD = 0 as *mut COORD;
    ptvis0 = ptVis(config, poly0, p0);
    ptvis1 = ptVis(config, poly1, p1);
    dad = makePath(p0, poly0, ptvis0, p1, poly1, ptvis1, config);
    opn = 1 as libc::c_int as size_t;
    i = *dad.offset((*config).N as isize);
    while i != (*config).N + 1 as libc::c_int {
        opn = opn.wrapping_add(1);
        i = *dad.offset(i as isize);
    }
    opn = opn.wrapping_add(1);
    ops = malloc(opn.wrapping_mul(::std::mem::size_of::<Ppoint_t>() as libc::c_ulong))
        as *mut Ppoint_t;
    let mut j: size_t = opn.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let fresh4 = j;
    j = j.wrapping_sub(1);
    *ops.offset(fresh4 as isize) = p1;
    i = *dad.offset((*config).N as isize);
    while i != (*config).N + 1 as libc::c_int {
        let fresh5 = j;
        j = j.wrapping_sub(1);
        *ops.offset(fresh5 as isize) = *((*config).P).offset(i as isize);
        i = *dad.offset(i as isize);
    }
    *ops.offset(j as isize) = p0;
    if j == 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"j == 0\0" as *const u8 as *const libc::c_char,
            b"cvt.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"int Pobspath(vconfig_t *, Ppoint_t, int, Ppoint_t, int, Ppolyline_t *)\0",
            ))
            .as_ptr(),
        );
    }
    free(ptvis0 as *mut libc::c_void);
    free(ptvis1 as *mut libc::c_void);
    (*output_route).pn = opn as libc::c_int;
    let ref mut fresh6 = (*output_route).ps;
    *fresh6 = ops;
    free(dad as *mut libc::c_void);
    return (0 as libc::c_int == 0) as libc::c_int;
}
