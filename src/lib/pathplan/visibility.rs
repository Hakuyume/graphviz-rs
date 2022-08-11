#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn in_poly(argpoly: Ppoly_t, q: Ppoint_t) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
unsafe extern "C" fn allocArray(mut V: libc::c_int, mut extra: libc::c_int) -> array2 {
    let mut i: libc::c_int = 0;
    let mut arr: array2 = 0 as *mut *mut COORD;
    let mut p: *mut COORD = 0 as *mut COORD;
    arr = malloc(
        ((V + extra) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut COORD>() as libc::c_ulong),
    ) as array2;
    p = calloc((V * V) as libc::c_ulong, ::std::mem::size_of::<COORD>() as libc::c_ulong)
        as *mut COORD;
    i = 0 as libc::c_int;
    while i < V {
        let ref mut fresh0 = *arr.offset(i as isize);
        *fresh0 = p;
        p = p.offset(V as isize);
        i += 1;
    }
    i = V;
    while i < V + extra {
        let ref mut fresh1 = *arr.offset(i as isize);
        *fresh1 = 0 as *mut COORD;
        i += 1;
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C" fn area2(
    mut a: Ppoint_t,
    mut b: Ppoint_t,
    mut c: Ppoint_t,
) -> COORD {
    return (a.y - b.y) * (c.x - b.x) - (c.y - b.y) * (a.x - b.x);
}
#[no_mangle]
pub unsafe extern "C" fn wind(
    mut a: Ppoint_t,
    mut b: Ppoint_t,
    mut c: Ppoint_t,
) -> libc::c_int {
    let mut w: COORD = 0.;
    w = (a.y - b.y) * (c.x - b.x) - (c.y - b.y) * (a.x - b.x);
    return if w > 0.0001f64 {
        1 as libc::c_int
    } else if w < -0.0001f64 {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn inBetween(
    mut a: Ppoint_t,
    mut b: Ppoint_t,
    mut c: Ppoint_t,
) -> bool {
    if a.x != b.x {
        return a.x < c.x && c.x < b.x || b.x < c.x && c.x < a.x
    } else {
        return a.y < c.y && c.y < b.y || b.y < c.y && c.y < a.y
    };
}
unsafe extern "C" fn intersect(
    mut a: Ppoint_t,
    mut b: Ppoint_t,
    mut c: Ppoint_t,
    mut d: Ppoint_t,
) -> bool {
    let mut a_abc: libc::c_int = 0;
    let mut a_abd: libc::c_int = 0;
    let mut a_cda: libc::c_int = 0;
    let mut a_cdb: libc::c_int = 0;
    a_abc = wind(a, b, c);
    if a_abc == 0 as libc::c_int && inBetween(a, b, c) as libc::c_int != 0 {
        return 1 as libc::c_int != 0;
    }
    a_abd = wind(a, b, d);
    if a_abd == 0 as libc::c_int && inBetween(a, b, d) as libc::c_int != 0 {
        return 1 as libc::c_int != 0;
    }
    a_cda = wind(c, d, a);
    a_cdb = wind(c, d, b);
    return a_abc * a_abd < 0 as libc::c_int && a_cda * a_cdb < 0 as libc::c_int;
}
unsafe extern "C" fn in_cone(
    mut a0: Ppoint_t,
    mut a1: Ppoint_t,
    mut a2: Ppoint_t,
    mut b: Ppoint_t,
) -> bool {
    let mut m: libc::c_int = wind(b, a0, a1);
    let mut p: libc::c_int = wind(b, a1, a2);
    if wind(a0, a1, a2) > 0 as libc::c_int {
        return m >= 0 as libc::c_int && p >= 0 as libc::c_int
    } else {
        return m >= 0 as libc::c_int || p >= 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn dist2(mut a: Ppoint_t, mut b: Ppoint_t) -> COORD {
    let mut delx: COORD = a.x - b.x;
    let mut dely: COORD = a.y - b.y;
    return delx * delx + dely * dely;
}
unsafe extern "C" fn dist(mut a: Ppoint_t, mut b: Ppoint_t) -> COORD {
    return sqrt(dist2(a, b));
}
unsafe extern "C" fn inCone(
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut pts: *mut Ppoint_t,
    mut nextPt: *mut libc::c_int,
    mut prevPt: *mut libc::c_int,
) -> bool {
    return in_cone(
        *pts.offset(*prevPt.offset(i as isize) as isize),
        *pts.offset(i as isize),
        *pts.offset(*nextPt.offset(i as isize) as isize),
        *pts.offset(j as isize),
    );
}
unsafe extern "C" fn clear(
    mut pti: Ppoint_t,
    mut ptj: Ppoint_t,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut V: libc::c_int,
    mut pts: *mut Ppoint_t,
    mut nextPt: *mut libc::c_int,
    mut prevPt: *mut libc::c_int,
) -> bool {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < start {
        if intersect(
            pti,
            ptj,
            *pts.offset(k as isize),
            *pts.offset(*nextPt.offset(k as isize) as isize),
        ) {
            return 0 as libc::c_int != 0;
        }
        k += 1;
    }
    k = end;
    while k < V {
        if intersect(
            pti,
            ptj,
            *pts.offset(k as isize),
            *pts.offset(*nextPt.offset(k as isize) as isize),
        ) {
            return 0 as libc::c_int != 0;
        }
        k += 1;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn compVis(mut conf: *mut vconfig_t, mut start: libc::c_int) {
    let mut V: libc::c_int = (*conf).N;
    let mut pts: *mut Ppoint_t = (*conf).P;
    let mut nextPt: *mut libc::c_int = (*conf).next;
    let mut prevPt: *mut libc::c_int = (*conf).prev;
    let mut wadj: array2 = (*conf).vis;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut previ: libc::c_int = 0;
    let mut d: COORD = 0.;
    i = start;
    while i < V {
        previ = *prevPt.offset(i as isize);
        d = dist(*pts.offset(i as isize), *pts.offset(previ as isize));
        *(*wadj.offset(i as isize)).offset(previ as isize) = d;
        *(*wadj.offset(previ as isize)).offset(i as isize) = d;
        if previ == i - 1 as libc::c_int {
            j = i - 2 as libc::c_int;
        } else {
            j = i - 1 as libc::c_int;
        }
        while j >= 0 as libc::c_int {
            if inCone(i, j, pts, nextPt, prevPt) as libc::c_int != 0
                && inCone(j, i, pts, nextPt, prevPt) as libc::c_int != 0
                && clear(
                    *pts.offset(i as isize),
                    *pts.offset(j as isize),
                    V,
                    V,
                    V,
                    pts,
                    nextPt,
                    prevPt,
                ) as libc::c_int != 0
            {
                d = dist(*pts.offset(i as isize), *pts.offset(j as isize));
                *(*wadj.offset(i as isize)).offset(j as isize) = d;
                *(*wadj.offset(j as isize)).offset(i as isize) = d;
            }
            j -= 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn visibility(mut conf: *mut vconfig_t) {
    let ref mut fresh2 = (*conf).vis;
    *fresh2 = allocArray((*conf).N, 2 as libc::c_int);
    compVis(conf, 0 as libc::c_int);
}
unsafe extern "C" fn polyhit(mut conf: *mut vconfig_t, mut p: Ppoint_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut poly: Ppoly_t = Ppoly_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    i = 0 as libc::c_int;
    while i < (*conf).Npoly {
        poly
            .ps = &mut *((*conf).P).offset(*((*conf).start).offset(i as isize) as isize)
            as *mut Ppoint_t;
        poly
            .pn = *((*conf).start).offset((i + 1 as libc::c_int) as isize)
            - *((*conf).start).offset(i as isize);
        if in_poly(poly, p) != 0 {
            return i;
        }
        i += 1;
    }
    return -(1111 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ptVis(
    mut conf: *mut vconfig_t,
    mut pp: libc::c_int,
    mut p: Ppoint_t,
) -> *mut COORD {
    let mut V: libc::c_int = (*conf).N;
    let mut pts: *mut Ppoint_t = (*conf).P;
    let mut nextPt: *mut libc::c_int = (*conf).next;
    let mut prevPt: *mut libc::c_int = (*conf).prev;
    let mut k: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut vadj: *mut COORD = 0 as *mut COORD;
    let mut pk: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut d: COORD = 0.;
    vadj = malloc(
        ((V + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<COORD>() as libc::c_ulong),
    ) as *mut COORD;
    if pp == -(2222 as libc::c_int) {
        pp = polyhit(conf, p);
    }
    if pp >= 0 as libc::c_int {
        start = *((*conf).start).offset(pp as isize);
        end = *((*conf).start).offset((pp + 1 as libc::c_int) as isize);
    } else {
        start = V;
        end = V;
    }
    k = 0 as libc::c_int;
    while k < start {
        pk = *pts.offset(k as isize);
        if in_cone(
            *pts.offset(*prevPt.offset(k as isize) as isize),
            pk,
            *pts.offset(*nextPt.offset(k as isize) as isize),
            p,
        ) as libc::c_int != 0
            && clear(p, pk, start, end, V, pts, nextPt, prevPt) as libc::c_int != 0
        {
            d = dist(p, pk);
            *vadj.offset(k as isize) = d;
        } else {
            *vadj.offset(k as isize) = 0 as libc::c_int as COORD;
        }
        k += 1;
    }
    k = start;
    while k < end {
        *vadj.offset(k as isize) = 0 as libc::c_int as COORD;
        k += 1;
    }
    k = end;
    while k < V {
        pk = *pts.offset(k as isize);
        if in_cone(
            *pts.offset(*prevPt.offset(k as isize) as isize),
            pk,
            *pts.offset(*nextPt.offset(k as isize) as isize),
            p,
        ) as libc::c_int != 0
            && clear(p, pk, start, end, V, pts, nextPt, prevPt) as libc::c_int != 0
        {
            d = dist(p, pk);
            *vadj.offset(k as isize) = d;
        } else {
            *vadj.offset(k as isize) = 0 as libc::c_int as COORD;
        }
        k += 1;
    }
    *vadj.offset(V as isize) = 0 as libc::c_int as COORD;
    *vadj.offset((V + 1 as libc::c_int) as isize) = 0 as libc::c_int as COORD;
    return vadj;
}
#[no_mangle]
pub unsafe extern "C" fn directVis(
    mut p: Ppoint_t,
    mut pp: libc::c_int,
    mut q: Ppoint_t,
    mut qp: libc::c_int,
    mut conf: *mut vconfig_t,
) -> bool {
    let mut V: libc::c_int = (*conf).N;
    let mut pts: *mut Ppoint_t = (*conf).P;
    let mut nextPt: *mut libc::c_int = (*conf).next;
    let mut k: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut e1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut e2: libc::c_int = 0;
    if pp < 0 as libc::c_int {
        s1 = 0 as libc::c_int;
        e1 = 0 as libc::c_int;
        if qp < 0 as libc::c_int {
            s2 = 0 as libc::c_int;
            e2 = 0 as libc::c_int;
        } else {
            s2 = *((*conf).start).offset(qp as isize);
            e2 = *((*conf).start).offset((qp + 1 as libc::c_int) as isize);
        }
    } else if qp < 0 as libc::c_int {
        s1 = 0 as libc::c_int;
        e1 = 0 as libc::c_int;
        s2 = *((*conf).start).offset(pp as isize);
        e2 = *((*conf).start).offset((pp + 1 as libc::c_int) as isize);
    } else if pp <= qp {
        s1 = *((*conf).start).offset(pp as isize);
        e1 = *((*conf).start).offset((pp + 1 as libc::c_int) as isize);
        s2 = *((*conf).start).offset(qp as isize);
        e2 = *((*conf).start).offset((qp + 1 as libc::c_int) as isize);
    } else {
        s1 = *((*conf).start).offset(qp as isize);
        e1 = *((*conf).start).offset((qp + 1 as libc::c_int) as isize);
        s2 = *((*conf).start).offset(pp as isize);
        e2 = *((*conf).start).offset((pp + 1 as libc::c_int) as isize);
    }
    k = 0 as libc::c_int;
    while k < s1 {
        if intersect(
            p,
            q,
            *pts.offset(k as isize),
            *pts.offset(*nextPt.offset(k as isize) as isize),
        ) {
            return 0 as libc::c_int != 0;
        }
        k += 1;
    }
    k = e1;
    while k < s2 {
        if intersect(
            p,
            q,
            *pts.offset(k as isize),
            *pts.offset(*nextPt.offset(k as isize) as isize),
        ) {
            return 0 as libc::c_int != 0;
        }
        k += 1;
    }
    k = e2;
    while k < V {
        if intersect(
            p,
            q,
            *pts.offset(k as isize),
            *pts.offset(*nextPt.offset(k as isize) as isize),
        ) {
            return 0 as libc::c_int != 0;
        }
        k += 1;
    }
    return 1 as libc::c_int != 0;
}
