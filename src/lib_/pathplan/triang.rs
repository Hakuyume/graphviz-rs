#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
unsafe extern "C" fn dpd_ccw(
    mut p1: *mut Ppoint_t,
    mut p2: *mut Ppoint_t,
    mut p3: *mut Ppoint_t,
) -> libc::c_int {
    let mut d: libc::c_double = ((*p1).y - (*p2).y) * ((*p3).x - (*p2).x)
        - ((*p3).y - (*p2).y) * ((*p1).x - (*p2).x);
    return if d > 0 as libc::c_int as libc::c_double {
        2 as libc::c_int
    } else if d < 0 as libc::c_int as libc::c_double {
        1 as libc::c_int
    } else {
        3 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn Ptriangulate(
    mut polygon: *mut Ppoly_t,
    mut fn_0: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut Ppoint_t) -> ()>,
    mut vc: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pointn: libc::c_int = 0;
    let mut pointp: *mut *mut Ppoint_t = 0 as *mut *mut Ppoint_t;
    pointn = (*polygon).pn;
    pointp = calloc(
        pointn as libc::c_ulong,
        ::std::mem::size_of::<*mut Ppoint_t>() as libc::c_ulong,
    ) as *mut *mut Ppoint_t;
    i = 0 as libc::c_int;
    while i < pointn {
        let ref mut fresh0 = *pointp.offset(i as isize);
        *fresh0 = &mut *((*polygon).ps).offset(i as isize) as *mut Ppoint_t;
        i += 1;
    }
    if triangulate(pointp, pointn, fn_0, vc) != 0 as libc::c_int {
        free(pointp as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    free(pointp as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn triangulate(
    mut pointp: *mut *mut Ppoint_t,
    mut pointn: libc::c_int,
    mut fn_0: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut Ppoint_t) -> ()>,
    mut vc: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ip1: libc::c_int = 0;
    let mut ip2: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut A: [Ppoint_t; 3] = [Ppoint_t { x: 0., y: 0. }; 3];
    if pointn > 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < pointn {
            ip1 = (i + 1 as libc::c_int) % pointn;
            ip2 = (i + 2 as libc::c_int) % pointn;
            if dpd_isdiagonal(i, ip2, pointp, pointn) {
                A[0 as libc::c_int as usize] = **pointp.offset(i as isize);
                A[1 as libc::c_int as usize] = **pointp.offset(ip1 as isize);
                A[2 as libc::c_int as usize] = **pointp.offset(ip2 as isize);
                fn_0.expect("non-null function pointer")(vc, A.as_mut_ptr());
                j = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < pointn {
                    if i != ip1 {
                        let fresh1 = j;
                        j = j + 1;
                        let ref mut fresh2 = *pointp.offset(fresh1 as isize);
                        *fresh2 = *pointp.offset(i as isize);
                    }
                    i += 1;
                }
                return triangulate(pointp, pointn - 1 as libc::c_int, fn_0, vc);
            }
            i += 1;
        }
        return -(1 as libc::c_int);
    } else {
        A[0 as libc::c_int as usize] = **pointp.offset(0 as libc::c_int as isize);
        A[1 as libc::c_int as usize] = **pointp.offset(1 as libc::c_int as isize);
        A[2 as libc::c_int as usize] = **pointp.offset(2 as libc::c_int as isize);
        fn_0.expect("non-null function pointer")(vc, A.as_mut_ptr());
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dpd_isdiagonal(
    mut i: libc::c_int,
    mut ip2: libc::c_int,
    mut pointp: *mut *mut Ppoint_t,
    mut pointn: libc::c_int,
) -> bool {
    let mut ip1: libc::c_int = 0;
    let mut im1: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jp1: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    ip1 = (i + 1 as libc::c_int) % pointn;
    im1 = (i + pointn - 1 as libc::c_int) % pointn;
    if dpd_ccw(
        *pointp.offset(im1 as isize),
        *pointp.offset(i as isize),
        *pointp.offset(ip1 as isize),
    ) == 1 as libc::c_int
    {
        res = (dpd_ccw(
            *pointp.offset(i as isize),
            *pointp.offset(ip2 as isize),
            *pointp.offset(im1 as isize),
        ) == 1 as libc::c_int
            && dpd_ccw(
                *pointp.offset(ip2 as isize),
                *pointp.offset(i as isize),
                *pointp.offset(ip1 as isize),
            ) == 1 as libc::c_int) as libc::c_int;
    } else {
        res = (dpd_ccw(
            *pointp.offset(i as isize),
            *pointp.offset(ip2 as isize),
            *pointp.offset(ip1 as isize),
        ) == 2 as libc::c_int) as libc::c_int;
    }
    if res == 0 {
        return 0 as libc::c_int != 0;
    }
    j = 0 as libc::c_int;
    while j < pointn {
        jp1 = (j + 1 as libc::c_int) % pointn;
        if !(j == i || jp1 == i || j == ip2 || jp1 == ip2) {
            if dpd_intersects(
                *pointp.offset(i as isize),
                *pointp.offset(ip2 as isize),
                *pointp.offset(j as isize),
                *pointp.offset(jp1 as isize),
            ) {
                return 0 as libc::c_int != 0;
            }
        }
        j += 1;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn dpd_intersects(
    mut pa: *mut Ppoint_t,
    mut pb: *mut Ppoint_t,
    mut pc: *mut Ppoint_t,
    mut pd: *mut Ppoint_t,
) -> bool {
    let mut ccw1: libc::c_int = 0;
    let mut ccw2: libc::c_int = 0;
    let mut ccw3: libc::c_int = 0;
    let mut ccw4: libc::c_int = 0;
    if dpd_ccw(pa, pb, pc) == 3 as libc::c_int || dpd_ccw(pa, pb, pd) == 3 as libc::c_int
        || dpd_ccw(pc, pd, pa) == 3 as libc::c_int
        || dpd_ccw(pc, pd, pb) == 3 as libc::c_int
    {
        if dpd_between(pa, pb, pc) as libc::c_int != 0
            || dpd_between(pa, pb, pd) as libc::c_int != 0
            || dpd_between(pc, pd, pa) as libc::c_int != 0
            || dpd_between(pc, pd, pb) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
    } else {
        ccw1 = if dpd_ccw(pa, pb, pc) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        ccw2 = if dpd_ccw(pa, pb, pd) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        ccw3 = if dpd_ccw(pc, pd, pa) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        ccw4 = if dpd_ccw(pc, pd, pb) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        return ccw1 ^ ccw2 != 0 && ccw3 ^ ccw4 != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn dpd_between(
    mut pa: *mut Ppoint_t,
    mut pb: *mut Ppoint_t,
    mut pc: *mut Ppoint_t,
) -> bool {
    let mut pba: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut pca: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    pba.x = (*pb).x - (*pa).x;
    pba.y = (*pb).y - (*pa).y;
    pca.x = (*pc).x - (*pa).x;
    pca.y = (*pc).y - (*pa).y;
    if dpd_ccw(pa, pb, pc) != 3 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return pca.x * pba.x + pca.y * pba.y >= 0 as libc::c_int as libc::c_double
        && pca.x * pca.x + pca.y * pca.y <= pba.x * pba.x + pba.y * pba.y;
}
