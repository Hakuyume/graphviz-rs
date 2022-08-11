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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn solve3(_: *mut libc::c_double, _: *mut libc::c_double) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pxy_t {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type Ppoint_t = Pxy_t;
pub type Pvector_t = Pxy_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tna_t {
    pub t: libc::c_double,
    pub a: [Ppoint_t; 2],
}
static mut ops: *mut Ppoint_t = 0 as *const Ppoint_t as *mut Ppoint_t;
static mut opn: libc::c_int = 0;
static mut opl: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Proutespline(
    mut edges: *mut Pedge_t,
    mut edgen: libc::c_int,
    mut input: Ppolyline_t,
    mut evs: *mut Ppoint_t,
    mut output: *mut Ppolyline_t,
) -> libc::c_int {
    let mut inps: *mut Ppoint_t = 0 as *mut Ppoint_t;
    let mut inpn: libc::c_int = 0;
    inps = input.ps;
    inpn = input.pn;
    *evs.offset(0 as libc::c_int as isize) = normv(*evs.offset(0 as libc::c_int as isize));
    *evs.offset(1 as libc::c_int as isize) = normv(*evs.offset(1 as libc::c_int as isize));
    opl = 0 as libc::c_int;
    if growops(4 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let fresh0 = opl;
    opl = opl + 1;
    *ops.offset(fresh0 as isize) = *inps.offset(0 as libc::c_int as isize);
    if reallyroutespline(
        edges,
        edgen,
        inps,
        inpn,
        *evs.offset(0 as libc::c_int as isize),
        *evs.offset(1 as libc::c_int as isize),
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    (*output).pn = opl;
    let ref mut fresh1 = (*output).ps;
    *fresh1 = ops;
    return 0 as libc::c_int;
}
unsafe extern "C" fn reallyroutespline(
    mut edges: *mut Pedge_t,
    mut edgen: libc::c_int,
    mut inps: *mut Ppoint_t,
    mut inpn: libc::c_int,
    mut ev0: Ppoint_t,
    mut ev1: Ppoint_t,
) -> libc::c_int {
    let mut p1: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut p2: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut cp1: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut cp2: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut p: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut v1: Pvector_t = Ppoint_t { x: 0., y: 0. };
    let mut v2: Pvector_t = Ppoint_t { x: 0., y: 0. };
    let mut splitv: Pvector_t = Ppoint_t { x: 0., y: 0. };
    let mut splitv1: Pvector_t = Ppoint_t { x: 0., y: 0. };
    let mut splitv2: Pvector_t = Ppoint_t { x: 0., y: 0. };
    let mut maxd: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut maxi: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut spliti: libc::c_int = 0;
    static mut tnas: *mut tna_t = 0 as *const tna_t as *mut tna_t;
    static mut tnan: libc::c_int = 0;
    if tnan < inpn {
        tnas = realloc(
            tnas as *mut libc::c_void,
            (::std::mem::size_of::<tna_t>() as libc::c_ulong).wrapping_mul(inpn as size_t),
        ) as *mut tna_t;
        if tnas.is_null() {
            return -(1 as libc::c_int);
        }
        tnan = inpn;
    }
    (*tnas.offset(0 as libc::c_int as isize)).t = 0 as libc::c_int as libc::c_double;
    i = 1 as libc::c_int;
    while i < inpn {
        (*tnas.offset(i as isize)).t = (*tnas.offset((i - 1 as libc::c_int) as isize)).t
            + dist(
                *inps.offset(i as isize),
                *inps.offset((i - 1 as libc::c_int) as isize),
            );
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < inpn {
        (*tnas.offset(i as isize)).t /= (*tnas.offset((inpn - 1 as libc::c_int) as isize)).t;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < inpn {
        (*tnas.offset(i as isize)).a[0 as libc::c_int as usize] =
            scale(ev0, B1((*tnas.offset(i as isize)).t));
        (*tnas.offset(i as isize)).a[1 as libc::c_int as usize] =
            scale(ev1, B2((*tnas.offset(i as isize)).t));
        i += 1;
    }
    if mkspline(
        inps, inpn, tnas, ev0, ev1, &mut p1, &mut v1, &mut p2, &mut v2,
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    let mut fit: libc::c_int = splinefits(edges, edgen, p1, v1, p2, v2, inps, inpn);
    if fit > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if fit < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    cp1 = add(p1, scale(v1, 1 as libc::c_int as libc::c_double / 3.0f64));
    cp2 = sub(p2, scale(v2, 1 as libc::c_int as libc::c_double / 3.0f64));
    maxd = -(1 as libc::c_int) as libc::c_double;
    maxi = -(1 as libc::c_int);
    i = 1 as libc::c_int;
    while i < inpn - 1 as libc::c_int {
        t = (*tnas.offset(i as isize)).t;
        p.x = B0(t) * p1.x + B1(t) * cp1.x + B2(t) * cp2.x + B3(t) * p2.x;
        p.y = B0(t) * p1.y + B1(t) * cp1.y + B2(t) * cp2.y + B3(t) * p2.y;
        d = dist(p, *inps.offset(i as isize));
        if d > maxd {
            maxd = d;
            maxi = i;
        }
        i += 1;
    }
    spliti = maxi;
    splitv1 = normv(sub(
        *inps.offset(spliti as isize),
        *inps.offset((spliti - 1 as libc::c_int) as isize),
    ));
    splitv2 = normv(sub(
        *inps.offset((spliti + 1 as libc::c_int) as isize),
        *inps.offset(spliti as isize),
    ));
    splitv = normv(add(splitv1, splitv2));
    if reallyroutespline(edges, edgen, inps, spliti + 1 as libc::c_int, ev0, splitv)
        < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if reallyroutespline(
        edges,
        edgen,
        &mut *inps.offset(spliti as isize),
        inpn - spliti,
        splitv,
        ev1,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mkspline(
    mut inps: *mut Ppoint_t,
    mut inpn: libc::c_int,
    mut tnas: *mut tna_t,
    mut ev0: Ppoint_t,
    mut ev1: Ppoint_t,
    mut sp0: *mut Ppoint_t,
    mut sv0: *mut Ppoint_t,
    mut sp1: *mut Ppoint_t,
    mut sv1: *mut Ppoint_t,
) -> libc::c_int {
    let mut tmp: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut c: [[libc::c_double; 2]; 2] = [[0.; 2]; 2];
    let mut x: [libc::c_double; 2] = [0.; 2];
    let mut det01: libc::c_double = 0.;
    let mut det0X: libc::c_double = 0.;
    let mut detX1: libc::c_double = 0.;
    let mut d01: libc::c_double = 0.;
    let mut scale0: libc::c_double = 0.;
    let mut scale3: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    scale3 = 0.0f64;
    scale0 = scale3;
    c[1 as libc::c_int as usize][1 as libc::c_int as usize] = 0.0f64;
    c[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        c[1 as libc::c_int as usize][1 as libc::c_int as usize];
    c[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        c[1 as libc::c_int as usize][0 as libc::c_int as usize];
    c[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        c[0 as libc::c_int as usize][1 as libc::c_int as usize];
    x[1 as libc::c_int as usize] = 0.0f64;
    x[0 as libc::c_int as usize] = x[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < inpn {
        c[0 as libc::c_int as usize][0 as libc::c_int as usize] += dot(
            (*tnas.offset(i as isize)).a[0 as libc::c_int as usize],
            (*tnas.offset(i as isize)).a[0 as libc::c_int as usize],
        );
        c[0 as libc::c_int as usize][1 as libc::c_int as usize] += dot(
            (*tnas.offset(i as isize)).a[0 as libc::c_int as usize],
            (*tnas.offset(i as isize)).a[1 as libc::c_int as usize],
        );
        c[1 as libc::c_int as usize][0 as libc::c_int as usize] =
            c[0 as libc::c_int as usize][1 as libc::c_int as usize];
        c[1 as libc::c_int as usize][1 as libc::c_int as usize] += dot(
            (*tnas.offset(i as isize)).a[1 as libc::c_int as usize],
            (*tnas.offset(i as isize)).a[1 as libc::c_int as usize],
        );
        tmp = sub(
            *inps.offset(i as isize),
            add(
                scale(
                    *inps.offset(0 as libc::c_int as isize),
                    B01((*tnas.offset(i as isize)).t),
                ),
                scale(
                    *inps.offset((inpn - 1 as libc::c_int) as isize),
                    B23((*tnas.offset(i as isize)).t),
                ),
            ),
        );
        x[0 as libc::c_int as usize] +=
            dot((*tnas.offset(i as isize)).a[0 as libc::c_int as usize], tmp);
        x[1 as libc::c_int as usize] +=
            dot((*tnas.offset(i as isize)).a[1 as libc::c_int as usize], tmp);
        i += 1;
    }
    det01 = c[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * c[1 as libc::c_int as usize][1 as libc::c_int as usize]
        - c[1 as libc::c_int as usize][0 as libc::c_int as usize]
            * c[0 as libc::c_int as usize][1 as libc::c_int as usize];
    det0X = c[0 as libc::c_int as usize][0 as libc::c_int as usize] * x[1 as libc::c_int as usize]
        - c[0 as libc::c_int as usize][1 as libc::c_int as usize] * x[0 as libc::c_int as usize];
    detX1 = x[0 as libc::c_int as usize] * c[1 as libc::c_int as usize][1 as libc::c_int as usize]
        - x[1 as libc::c_int as usize] * c[0 as libc::c_int as usize][1 as libc::c_int as usize];
    if fabs(det01) >= 1e-6f64 {
        scale0 = detX1 / det01;
        scale3 = det0X / det01;
    }
    if fabs(det01) < 1e-6f64 || scale0 <= 0.0f64 || scale3 <= 0.0f64 {
        d01 = dist(
            *inps.offset(0 as libc::c_int as isize),
            *inps.offset((inpn - 1 as libc::c_int) as isize),
        ) / 3.0f64;
        scale0 = d01;
        scale3 = d01;
    }
    *sp0 = *inps.offset(0 as libc::c_int as isize);
    *sv0 = scale(ev0, scale0);
    *sp1 = *inps.offset((inpn - 1 as libc::c_int) as isize);
    *sv1 = scale(ev1, scale3);
    return 0 as libc::c_int;
}
unsafe extern "C" fn dist_n(mut p: *mut Ppoint_t, mut n: libc::c_int) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_double = 0.;
    rv = 0.0f64;
    i = 1 as libc::c_int;
    while i < n {
        rv += hypot(
            (*p.offset(i as isize)).x - (*p.offset((i - 1 as libc::c_int) as isize)).x,
            (*p.offset(i as isize)).y - (*p.offset((i - 1 as libc::c_int) as isize)).y,
        );
        i += 1;
    }
    return rv;
}
unsafe extern "C" fn splinefits(
    mut edges: *mut Pedge_t,
    mut edgen: libc::c_int,
    mut pa: Ppoint_t,
    mut va: Pvector_t,
    mut pb: Ppoint_t,
    mut vb: Pvector_t,
    mut inps: *mut Ppoint_t,
    mut inpn: libc::c_int,
) -> libc::c_int {
    let mut sps: [Ppoint_t; 4] = [Ppoint_t { x: 0., y: 0. }; 4];
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut pi: libc::c_int = 0;
    let mut forceflag: libc::c_int = 0;
    let mut first: libc::c_int = 1 as libc::c_int;
    forceflag = if inpn == 2 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    b = 4 as libc::c_int as libc::c_double;
    a = b;
    loop {
        sps[0 as libc::c_int as usize].x = pa.x;
        sps[0 as libc::c_int as usize].y = pa.y;
        sps[1 as libc::c_int as usize].x = pa.x + a * va.x / 3.0f64;
        sps[1 as libc::c_int as usize].y = pa.y + a * va.y / 3.0f64;
        sps[2 as libc::c_int as usize].x = pb.x - b * vb.x / 3.0f64;
        sps[2 as libc::c_int as usize].y = pb.y - b * vb.y / 3.0f64;
        sps[3 as libc::c_int as usize].x = pb.x;
        sps[3 as libc::c_int as usize].y = pb.y;
        if first != 0 && dist_n(sps.as_mut_ptr(), 4 as libc::c_int) < dist_n(inps, inpn) - 1E-3f64 {
            return 0 as libc::c_int;
        }
        first = 0 as libc::c_int;
        if splineisinside(
            edges,
            edgen,
            &mut *sps.as_mut_ptr().offset(0 as libc::c_int as isize),
        ) != 0
        {
            if growops(opl + 4 as libc::c_int) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            pi = 1 as libc::c_int;
            while pi < 4 as libc::c_int {
                (*ops.offset(opl as isize)).x = sps[pi as usize].x;
                let fresh2 = opl;
                opl = opl + 1;
                (*ops.offset(fresh2 as isize)).y = sps[pi as usize].y;
                pi += 1;
            }
            return 1 as libc::c_int;
        }
        if a == 0 as libc::c_int as libc::c_double && b == 0 as libc::c_int as libc::c_double {
            if forceflag != 0 {
                if growops(opl + 4 as libc::c_int) < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                pi = 1 as libc::c_int;
                while pi < 4 as libc::c_int {
                    (*ops.offset(opl as isize)).x = sps[pi as usize].x;
                    let fresh3 = opl;
                    opl = opl + 1;
                    (*ops.offset(fresh3 as isize)).y = sps[pi as usize].y;
                    pi += 1;
                }
                return 1 as libc::c_int;
            }
            break;
        } else if a > 0.01f64 {
            a /= 2 as libc::c_int as libc::c_double;
            b /= 2 as libc::c_int as libc::c_double;
        } else {
            b = 0 as libc::c_int as libc::c_double;
            a = b;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn splineisinside(
    mut edges: *mut Pedge_t,
    mut edgen: libc::c_int,
    mut sps: *mut Ppoint_t,
) -> libc::c_int {
    let mut roots: [libc::c_double; 4] = [0.; 4];
    let mut rooti: libc::c_int = 0;
    let mut rootn: libc::c_int = 0;
    let mut ei: libc::c_int = 0;
    let mut lps: [Ppoint_t; 2] = [Ppoint_t { x: 0., y: 0. }; 2];
    let mut ip: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut t: libc::c_double = 0.;
    let mut ta: libc::c_double = 0.;
    let mut tb: libc::c_double = 0.;
    let mut tc: libc::c_double = 0.;
    let mut td: libc::c_double = 0.;
    ei = 0 as libc::c_int;
    while ei < edgen {
        lps[0 as libc::c_int as usize] = (*edges.offset(ei as isize)).a;
        lps[1 as libc::c_int as usize] = (*edges.offset(ei as isize)).b;
        rootn = splineintersectsline(sps, lps.as_mut_ptr(), roots.as_mut_ptr());
        if !(rootn == 4 as libc::c_int) {
            rooti = 0 as libc::c_int;
            while rooti < rootn {
                if !(roots[rooti as usize] < 1E-6f64
                    || roots[rooti as usize] > 1 as libc::c_int as libc::c_double - 1E-6f64)
                {
                    t = roots[rooti as usize];
                    td = t * t * t;
                    tc = 3 as libc::c_int as libc::c_double
                        * t
                        * t
                        * (1 as libc::c_int as libc::c_double - t);
                    tb = 3 as libc::c_int as libc::c_double
                        * t
                        * (1 as libc::c_int as libc::c_double - t)
                        * (1 as libc::c_int as libc::c_double - t);
                    ta = (1 as libc::c_int as libc::c_double - t)
                        * (1 as libc::c_int as libc::c_double - t)
                        * (1 as libc::c_int as libc::c_double - t);
                    ip.x = ta * (*sps.offset(0 as libc::c_int as isize)).x
                        + tb * (*sps.offset(1 as libc::c_int as isize)).x
                        + tc * (*sps.offset(2 as libc::c_int as isize)).x
                        + td * (*sps.offset(3 as libc::c_int as isize)).x;
                    ip.y = ta * (*sps.offset(0 as libc::c_int as isize)).y
                        + tb * (*sps.offset(1 as libc::c_int as isize)).y
                        + tc * (*sps.offset(2 as libc::c_int as isize)).y
                        + td * (*sps.offset(3 as libc::c_int as isize)).y;
                    if !((ip.x - lps[0 as libc::c_int as usize].x)
                        * (ip.x - lps[0 as libc::c_int as usize].x)
                        + (ip.y - lps[0 as libc::c_int as usize].y)
                            * (ip.y - lps[0 as libc::c_int as usize].y)
                        < 1E-3f64
                        || (ip.x - lps[1 as libc::c_int as usize].x)
                            * (ip.x - lps[1 as libc::c_int as usize].x)
                            + (ip.y - lps[1 as libc::c_int as usize].y)
                                * (ip.y - lps[1 as libc::c_int as usize].y)
                            < 1E-3f64)
                    {
                        return 0 as libc::c_int;
                    }
                }
                rooti += 1;
            }
        }
        ei += 1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn splineintersectsline(
    mut sps: *mut Ppoint_t,
    mut lps: *mut Ppoint_t,
    mut roots: *mut libc::c_double,
) -> libc::c_int {
    let mut scoeff: [libc::c_double; 4] = [0.; 4];
    let mut xcoeff: [libc::c_double; 2] = [0.; 2];
    let mut ycoeff: [libc::c_double; 2] = [0.; 2];
    let mut xroots: [libc::c_double; 3] = [0.; 3];
    let mut yroots: [libc::c_double; 3] = [0.; 3];
    let mut tv: libc::c_double = 0.;
    let mut sv: libc::c_double = 0.;
    let mut rat: libc::c_double = 0.;
    let mut rootn: libc::c_int = 0;
    let mut xrootn: libc::c_int = 0;
    let mut yrootn: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    xcoeff[0 as libc::c_int as usize] = (*lps.offset(0 as libc::c_int as isize)).x;
    xcoeff[1 as libc::c_int as usize] =
        (*lps.offset(1 as libc::c_int as isize)).x - (*lps.offset(0 as libc::c_int as isize)).x;
    ycoeff[0 as libc::c_int as usize] = (*lps.offset(0 as libc::c_int as isize)).y;
    ycoeff[1 as libc::c_int as usize] =
        (*lps.offset(1 as libc::c_int as isize)).y - (*lps.offset(0 as libc::c_int as isize)).y;
    rootn = 0 as libc::c_int;
    if xcoeff[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double {
        if ycoeff[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double {
            points2coeff(
                (*sps.offset(0 as libc::c_int as isize)).x,
                (*sps.offset(1 as libc::c_int as isize)).x,
                (*sps.offset(2 as libc::c_int as isize)).x,
                (*sps.offset(3 as libc::c_int as isize)).x,
                scoeff.as_mut_ptr(),
            );
            scoeff[0 as libc::c_int as usize] -= xcoeff[0 as libc::c_int as usize];
            xrootn = solve3(scoeff.as_mut_ptr(), xroots.as_mut_ptr());
            points2coeff(
                (*sps.offset(0 as libc::c_int as isize)).y,
                (*sps.offset(1 as libc::c_int as isize)).y,
                (*sps.offset(2 as libc::c_int as isize)).y,
                (*sps.offset(3 as libc::c_int as isize)).y,
                scoeff.as_mut_ptr(),
            );
            scoeff[0 as libc::c_int as usize] -= ycoeff[0 as libc::c_int as usize];
            yrootn = solve3(scoeff.as_mut_ptr(), yroots.as_mut_ptr());
            if xrootn == 4 as libc::c_int {
                if yrootn == 4 as libc::c_int {
                    return 4 as libc::c_int;
                } else {
                    j = 0 as libc::c_int;
                    while j < yrootn {
                        addroot(yroots[j as usize], roots, &mut rootn);
                        j += 1;
                    }
                }
            } else if yrootn == 4 as libc::c_int {
                i = 0 as libc::c_int;
                while i < xrootn {
                    addroot(xroots[i as usize], roots, &mut rootn);
                    i += 1;
                }
            } else {
                i = 0 as libc::c_int;
                while i < xrootn {
                    j = 0 as libc::c_int;
                    while j < yrootn {
                        if xroots[i as usize] == yroots[j as usize] {
                            addroot(xroots[i as usize], roots, &mut rootn);
                        }
                        j += 1;
                    }
                    i += 1;
                }
            }
            return rootn;
        } else {
            points2coeff(
                (*sps.offset(0 as libc::c_int as isize)).x,
                (*sps.offset(1 as libc::c_int as isize)).x,
                (*sps.offset(2 as libc::c_int as isize)).x,
                (*sps.offset(3 as libc::c_int as isize)).x,
                scoeff.as_mut_ptr(),
            );
            scoeff[0 as libc::c_int as usize] -= xcoeff[0 as libc::c_int as usize];
            xrootn = solve3(scoeff.as_mut_ptr(), xroots.as_mut_ptr());
            if xrootn == 4 as libc::c_int {
                return 4 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < xrootn {
                tv = xroots[i as usize];
                if tv >= 0 as libc::c_int as libc::c_double
                    && tv <= 1 as libc::c_int as libc::c_double
                {
                    points2coeff(
                        (*sps.offset(0 as libc::c_int as isize)).y,
                        (*sps.offset(1 as libc::c_int as isize)).y,
                        (*sps.offset(2 as libc::c_int as isize)).y,
                        (*sps.offset(3 as libc::c_int as isize)).y,
                        scoeff.as_mut_ptr(),
                    );
                    sv = scoeff[0 as libc::c_int as usize]
                        + tv * (scoeff[1 as libc::c_int as usize]
                            + tv * (scoeff[2 as libc::c_int as usize]
                                + tv * scoeff[3 as libc::c_int as usize]));
                    sv = (sv - ycoeff[0 as libc::c_int as usize])
                        / ycoeff[1 as libc::c_int as usize];
                    if 0 as libc::c_int as libc::c_double <= sv
                        && sv <= 1 as libc::c_int as libc::c_double
                    {
                        addroot(tv, roots, &mut rootn);
                    }
                }
                i += 1;
            }
            return rootn;
        }
    } else {
        rat = ycoeff[1 as libc::c_int as usize] / xcoeff[1 as libc::c_int as usize];
        points2coeff(
            (*sps.offset(0 as libc::c_int as isize)).y
                - rat * (*sps.offset(0 as libc::c_int as isize)).x,
            (*sps.offset(1 as libc::c_int as isize)).y
                - rat * (*sps.offset(1 as libc::c_int as isize)).x,
            (*sps.offset(2 as libc::c_int as isize)).y
                - rat * (*sps.offset(2 as libc::c_int as isize)).x,
            (*sps.offset(3 as libc::c_int as isize)).y
                - rat * (*sps.offset(3 as libc::c_int as isize)).x,
            scoeff.as_mut_ptr(),
        );
        scoeff[0 as libc::c_int as usize] +=
            rat * xcoeff[0 as libc::c_int as usize] - ycoeff[0 as libc::c_int as usize];
        xrootn = solve3(scoeff.as_mut_ptr(), xroots.as_mut_ptr());
        if xrootn == 4 as libc::c_int {
            return 4 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < xrootn {
            tv = xroots[i as usize];
            if tv >= 0 as libc::c_int as libc::c_double && tv <= 1 as libc::c_int as libc::c_double
            {
                points2coeff(
                    (*sps.offset(0 as libc::c_int as isize)).x,
                    (*sps.offset(1 as libc::c_int as isize)).x,
                    (*sps.offset(2 as libc::c_int as isize)).x,
                    (*sps.offset(3 as libc::c_int as isize)).x,
                    scoeff.as_mut_ptr(),
                );
                sv = scoeff[0 as libc::c_int as usize]
                    + tv * (scoeff[1 as libc::c_int as usize]
                        + tv * (scoeff[2 as libc::c_int as usize]
                            + tv * scoeff[3 as libc::c_int as usize]));
                sv = (sv - xcoeff[0 as libc::c_int as usize]) / xcoeff[1 as libc::c_int as usize];
                if 0 as libc::c_int as libc::c_double <= sv
                    && sv <= 1 as libc::c_int as libc::c_double
                {
                    addroot(tv, roots, &mut rootn);
                }
            }
            i += 1;
        }
        return rootn;
    };
}
unsafe extern "C" fn points2coeff(
    mut v0: libc::c_double,
    mut v1: libc::c_double,
    mut v2: libc::c_double,
    mut v3: libc::c_double,
    mut coeff: *mut libc::c_double,
) {
    *coeff.offset(3 as libc::c_int as isize) = v3 + 3 as libc::c_int as libc::c_double * v1
        - (v0 + 3 as libc::c_int as libc::c_double * v2);
    *coeff.offset(2 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double * v0
        + 3 as libc::c_int as libc::c_double * v2
        - 6 as libc::c_int as libc::c_double * v1;
    *coeff.offset(1 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double * (v1 - v0);
    *coeff.offset(0 as libc::c_int as isize) = v0;
}
unsafe extern "C" fn addroot(
    mut root: libc::c_double,
    mut roots: *mut libc::c_double,
    mut rootnp: *mut libc::c_int,
) {
    if root >= 0 as libc::c_int as libc::c_double && root <= 1 as libc::c_int as libc::c_double {
        *roots.offset(*rootnp as isize) = root;
        *rootnp += 1;
    }
}
unsafe extern "C" fn normv(mut v: Pvector_t) -> Pvector_t {
    let mut d: libc::c_double = 0.;
    d = v.x * v.x + v.y * v.y;
    if d > 1e-6f64 {
        d = sqrt(d);
        v.x /= d;
        v.y /= d;
    }
    return v;
}
unsafe extern "C" fn growops(mut newopn: libc::c_int) -> libc::c_int {
    if newopn <= opn {
        return 0 as libc::c_int;
    }
    ops = realloc(
        ops as *mut libc::c_void,
        (::std::mem::size_of::<Ppoint_t>() as libc::c_ulong).wrapping_mul(newopn as size_t),
    ) as *mut Ppoint_t;
    if ops.is_null() {
        return -(1 as libc::c_int);
    }
    opn = newopn;
    return 0 as libc::c_int;
}
unsafe extern "C" fn add(mut p1: Ppoint_t, mut p2: Ppoint_t) -> Ppoint_t {
    p1.x += p2.x;
    p1.y += p2.y;
    return p1;
}
unsafe extern "C" fn sub(mut p1: Ppoint_t, mut p2: Ppoint_t) -> Ppoint_t {
    p1.x -= p2.x;
    p1.y -= p2.y;
    return p1;
}
unsafe extern "C" fn dist(mut p1: Ppoint_t, mut p2: Ppoint_t) -> libc::c_double {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    dx = p2.x - p1.x;
    dy = p2.y - p1.y;
    return hypot(dx, dy);
}
unsafe extern "C" fn scale(mut p: Ppoint_t, mut c: libc::c_double) -> Ppoint_t {
    p.x *= c;
    p.y *= c;
    return p;
}
unsafe extern "C" fn dot(mut p1: Ppoint_t, mut p2: Ppoint_t) -> libc::c_double {
    return p1.x * p2.x + p1.y * p2.y;
}
unsafe extern "C" fn B0(mut t: libc::c_double) -> libc::c_double {
    let mut tmp: libc::c_double = 1.0f64 - t;
    return tmp * tmp * tmp;
}
unsafe extern "C" fn B1(mut t: libc::c_double) -> libc::c_double {
    let mut tmp: libc::c_double = 1.0f64 - t;
    return 3 as libc::c_int as libc::c_double * t * tmp * tmp;
}
unsafe extern "C" fn B2(mut t: libc::c_double) -> libc::c_double {
    let mut tmp: libc::c_double = 1.0f64 - t;
    return 3 as libc::c_int as libc::c_double * t * t * tmp;
}
unsafe extern "C" fn B3(mut t: libc::c_double) -> libc::c_double {
    return t * t * t;
}
unsafe extern "C" fn B01(mut t: libc::c_double) -> libc::c_double {
    let mut tmp: libc::c_double = 1.0f64 - t;
    return tmp * tmp * (tmp + 3 as libc::c_int as libc::c_double * t);
}
unsafe extern "C" fn B23(mut t: libc::c_double) -> libc::c_double {
    let mut tmp: libc::c_double = 1.0f64 - t;
    return t * t * (3 as libc::c_int as libc::c_double * tmp + t);
}
