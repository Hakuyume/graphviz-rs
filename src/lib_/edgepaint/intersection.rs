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
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub const dim: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
pub const dim_0: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
unsafe extern "C" fn cross(
    mut u: *mut libc::c_double,
    mut v: *mut libc::c_double,
) -> libc::c_double {
    return *u.offset(0 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
        - *u.offset(1 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize);
}
unsafe extern "C" fn dist(
    mut dim_1: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> libc::c_double {
    let mut k: libc::c_int = 0;
    let mut d: libc::c_double = 0 as libc::c_int as libc::c_double;
    k = 0 as libc::c_int;
    while k < dim_1 {
        d += (*x.offset(k as isize) - *y.offset(k as isize))
            * (*x.offset(k as isize) - *y.offset(k as isize));
        k += 1;
    }
    return sqrt(d);
}
unsafe extern "C" fn point_line_distance(
    mut p: *mut libc::c_double,
    mut q: *mut libc::c_double,
    mut r: *mut libc::c_double,
) -> libc::c_double {
    let mut t: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < dim as libc::c_int {
        t += (*p.offset(i as isize) - *q.offset(i as isize))
            * (*r.offset(i as isize) - *q.offset(i as isize));
        b += (*r.offset(i as isize) - *q.offset(i as isize))
            * (*r.offset(i as isize) - *q.offset(i as isize));
        i += 1;
    }
    if b <= 1.0e-16f64 {
        return dist(dim as libc::c_int, p, q);
    }
    t = t / b;
    if t >= 0 as libc::c_int as libc::c_double && t <= 1 as libc::c_int as libc::c_double {
        b = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < dim as libc::c_int {
            tmp = *p.offset(i as isize)
                - (*q.offset(i as isize) + t * (*r.offset(i as isize) - *q.offset(i as isize)));
            b += tmp * tmp;
            i += 1;
        }
        return sqrt(b);
    }
    t = dist(dim as libc::c_int, p, q);
    b = dist(dim as libc::c_int, p, r);
    return if t < b { t } else { b };
}
unsafe extern "C" fn line_segments_distance(
    mut p1: *mut libc::c_double,
    mut p2: *mut libc::c_double,
    mut q1: *mut libc::c_double,
    mut q2: *mut libc::c_double,
) -> libc::c_double {
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut t3: libc::c_double = 0.;
    let mut t4: libc::c_double = 0.;
    t1 = point_line_distance(p1, q1, q2);
    t2 = point_line_distance(p2, q1, q2);
    t3 = point_line_distance(q1, p1, p2);
    t4 = point_line_distance(q2, p1, p2);
    t1 = if t1 < t2 { t1 } else { t2 };
    t3 = if t3 < t4 { t3 } else { t4 };
    return if t1 < t3 { t1 } else { t3 };
}
#[no_mangle]
pub unsafe extern "C" fn intersection_angle(
    mut p1: *mut libc::c_double,
    mut p2: *mut libc::c_double,
    mut q1: *mut libc::c_double,
    mut q2: *mut libc::c_double,
) -> libc::c_double {
    let mut r: [libc::c_double; 2] = [0.; 2];
    let mut s: [libc::c_double; 2] = [0.; 2];
    let mut qp: [libc::c_double; 2] = [0.; 2];
    let mut rnorm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut snorm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut b: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut epsilon: libc::c_double = sin(1 as libc::c_int as libc::c_double / 180.0f64);
    let mut close: libc::c_double = 0.01f64;
    let mut line_dist_close: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut res: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < dim_0 as libc::c_int {
        r[i as usize] = *p2.offset(i as isize) - *p1.offset(i as isize);
        rnorm += r[i as usize] * r[i as usize];
        i += 1;
    }
    rnorm = sqrt(rnorm);
    i = 0 as libc::c_int;
    while i < dim_0 as libc::c_int {
        s[i as usize] = *q2.offset(i as isize) - *q1.offset(i as isize);
        snorm += s[i as usize] * s[i as usize];
        i += 1;
    }
    snorm = sqrt(snorm);
    b = cross(r.as_mut_ptr(), s.as_mut_ptr());
    line_dist_close = (line_segments_distance(p1, p2, q1, q2)
        <= close * (if rnorm > snorm { rnorm } else { snorm }))
        as libc::c_int;
    if fabs(b) <= epsilon * snorm * rnorm {
        if line_dist_close != 0 {
            return 1 as libc::c_int as libc::c_double;
        }
        return -(2 as libc::c_int) as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < dim_0 as libc::c_int {
        qp[i as usize] = *q1.offset(i as isize) - *p1.offset(i as isize);
        i += 1;
    }
    t = cross(qp.as_mut_ptr(), s.as_mut_ptr()) / b;
    u = cross(qp.as_mut_ptr(), r.as_mut_ptr()) / b;
    if t >= 0 as libc::c_int as libc::c_double
        && t <= 1 as libc::c_int as libc::c_double
        && u >= 0 as libc::c_int as libc::c_double
        && u <= 1 as libc::c_int as libc::c_double
        || line_dist_close != 0
    {
        let mut rs: libc::c_double = 0 as libc::c_int as libc::c_double;
        if rnorm * snorm < 1.0e-16f64 {
            return 0 as libc::c_int as libc::c_double;
        }
        i = 0 as libc::c_int;
        while i < dim_0 as libc::c_int {
            rs += r[i as usize] * s[i as usize];
            i += 1;
        }
        res = rs / (rnorm * snorm);
        if *p1.offset(0 as libc::c_int as isize) == *q1.offset(0 as libc::c_int as isize)
            && *p1.offset(1 as libc::c_int as isize) == *q1.offset(1 as libc::c_int as isize)
        {
            return res;
        } else {
            if *p1.offset(0 as libc::c_int as isize) == *q2.offset(0 as libc::c_int as isize)
                && *p1.offset(1 as libc::c_int as isize) == *q2.offset(1 as libc::c_int as isize)
            {
                return -res;
            } else {
                if *p2.offset(0 as libc::c_int as isize) == *q1.offset(0 as libc::c_int as isize)
                    && *p2.offset(1 as libc::c_int as isize)
                        == *q1.offset(1 as libc::c_int as isize)
                {
                    return -res;
                } else {
                    if *p2.offset(0 as libc::c_int as isize)
                        == *q2.offset(0 as libc::c_int as isize)
                        && *p2.offset(1 as libc::c_int as isize)
                            == *q2.offset(1 as libc::c_int as isize)
                    {
                        return res;
                    }
                }
            }
        }
        return fabs(res);
    }
    return -(2 as libc::c_int) as libc::c_double;
}
