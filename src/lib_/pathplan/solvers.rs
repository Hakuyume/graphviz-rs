#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn cbrt(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn solve3(
    mut coeff: *mut libc::c_double,
    mut roots: *mut libc::c_double,
) -> libc::c_int {
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut rootn: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut disc: libc::c_double = 0.;
    let mut b_over_3a: libc::c_double = 0.;
    let mut c_over_a: libc::c_double = 0.;
    let mut d_over_a: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    a = *coeff.offset(3 as libc::c_int as isize);
    b = *coeff.offset(2 as libc::c_int as isize);
    c = *coeff.offset(1 as libc::c_int as isize);
    d = *coeff.offset(0 as libc::c_int as isize);
    if a < 1E-7f64 && a > -1E-7f64 {
        return solve2(coeff, roots);
    }
    b_over_3a = b / (3 as libc::c_int as libc::c_double * a);
    c_over_a = c / a;
    d_over_a = d / a;
    p = b_over_3a * b_over_3a;
    q = 2 as libc::c_int as libc::c_double * b_over_3a * p - b_over_3a * c_over_a
        + d_over_a;
    p = c_over_a / 3 as libc::c_int as libc::c_double - p;
    disc = q * q + 4 as libc::c_int as libc::c_double * p * p * p;
    if disc < 0 as libc::c_int as libc::c_double {
        r = 0.5f64 * sqrt(-disc + q * q);
        theta = atan2(sqrt(-disc), -q);
        temp = 2 as libc::c_int as libc::c_double * cbrt(r);
        *roots
            .offset(
                0 as libc::c_int as isize,
            ) = temp * cos(theta / 3 as libc::c_int as libc::c_double);
        *roots
            .offset(
                1 as libc::c_int as isize,
            ) = temp
            * cos(
                (theta + 3.14159265358979323846f64 + 3.14159265358979323846f64)
                    / 3 as libc::c_int as libc::c_double,
            );
        *roots
            .offset(
                2 as libc::c_int as isize,
            ) = temp
            * cos(
                (theta - 3.14159265358979323846f64 - 3.14159265358979323846f64)
                    / 3 as libc::c_int as libc::c_double,
            );
        rootn = 3 as libc::c_int;
    } else {
        alpha = 0.5f64 * (sqrt(disc) - q);
        beta = -q - alpha;
        *roots.offset(0 as libc::c_int as isize) = cbrt(alpha) + cbrt(beta);
        if disc > 0 as libc::c_int as libc::c_double {
            rootn = 1 as libc::c_int;
        } else {
            let ref mut fresh0 = *roots.offset(2 as libc::c_int as isize);
            *fresh0 = -0.5f64 * *roots.offset(0 as libc::c_int as isize);
            *roots.offset(1 as libc::c_int as isize) = *fresh0;
            rootn = 3 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while i < rootn {
        *roots.offset(i as isize) -= b_over_3a;
        i += 1;
    }
    return rootn;
}
unsafe extern "C" fn solve2(
    mut coeff: *mut libc::c_double,
    mut roots: *mut libc::c_double,
) -> libc::c_int {
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut disc: libc::c_double = 0.;
    let mut b_over_2a: libc::c_double = 0.;
    let mut c_over_a: libc::c_double = 0.;
    a = *coeff.offset(2 as libc::c_int as isize);
    b = *coeff.offset(1 as libc::c_int as isize);
    c = *coeff.offset(0 as libc::c_int as isize);
    if a < 1E-7f64 && a > -1E-7f64 {
        return solve1(coeff, roots);
    }
    b_over_2a = b / (2 as libc::c_int as libc::c_double * a);
    c_over_a = c / a;
    disc = b_over_2a * b_over_2a - c_over_a;
    if disc < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int
    } else {
        if disc > 0 as libc::c_int as libc::c_double {
            *roots.offset(0 as libc::c_int as isize) = -b_over_2a + sqrt(disc);
            *roots
                .offset(
                    1 as libc::c_int as isize,
                ) = -(2 as libc::c_int) as libc::c_double * b_over_2a
                - *roots.offset(0 as libc::c_int as isize);
            return 2 as libc::c_int;
        }
    }
    *roots.offset(0 as libc::c_int as isize) = -b_over_2a;
    return 1 as libc::c_int;
}
unsafe extern "C" fn solve1(
    mut coeff: *mut libc::c_double,
    mut roots: *mut libc::c_double,
) -> libc::c_int {
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    a = *coeff.offset(1 as libc::c_int as isize);
    b = *coeff.offset(0 as libc::c_int as isize);
    if a < 1E-7f64 && a > -1E-7f64 {
        if b < 1E-7f64 && b > -1E-7f64 {
            return 4 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    }
    *roots.offset(0 as libc::c_int as isize) = -b / a;
    return 1 as libc::c_int;
}
