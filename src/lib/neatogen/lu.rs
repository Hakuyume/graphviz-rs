#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn free_array(rv: *mut *mut libc::c_double);
    fn new_array(
        i: libc::c_int,
        j: libc::c_int,
        val: libc::c_double,
    ) -> *mut *mut libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
static mut scales: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
static mut lu: *mut *mut libc::c_double = 0 as *const *mut libc::c_double
    as *mut *mut libc::c_double;
static mut ps: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn lu_decompose(
    mut a: *mut *mut libc::c_double,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pivotindex: libc::c_int = 0 as libc::c_int;
    let mut pivot: libc::c_double = 0.;
    let mut biggest: libc::c_double = 0.;
    let mut mult: libc::c_double = 0.;
    let mut tempf: libc::c_double = 0.;
    if !lu.is_null() {
        free_array(lu);
    }
    lu = new_array(n, n, 0.0f64);
    free(ps as *mut libc::c_void);
    ps = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    free(scales as *mut libc::c_void);
    scales = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        biggest = 0.0f64;
        j = 0 as libc::c_int;
        while j < n {
            let ref mut fresh0 = *(*lu.offset(i as isize)).offset(j as isize);
            *fresh0 = *(*a.offset(i as isize)).offset(j as isize);
            biggest = fmax(biggest, fabs(*fresh0));
            j += 1;
        }
        if biggest > 0.0f64 {
            *scales.offset(i as isize) = 1.0f64 / biggest;
        } else {
            *scales.offset(i as isize) = 0.0f64;
            return 0 as libc::c_int;
        }
        *ps.offset(i as isize) = i;
        i += 1;
    }
    k = 0 as libc::c_int;
    while k < n - 1 as libc::c_int {
        biggest = 0.0f64;
        i = k;
        while i < n {
            tempf = fabs(
                *(*lu.offset(*ps.offset(i as isize) as isize)).offset(k as isize),
            ) * *scales.offset(*ps.offset(i as isize) as isize);
            if biggest < tempf {
                biggest = tempf;
                pivotindex = i;
            }
            i += 1;
        }
        if biggest <= 0.0f64 {
            return 0 as libc::c_int;
        }
        if pivotindex != k {
            j = *ps.offset(k as isize);
            *ps.offset(k as isize) = *ps.offset(pivotindex as isize);
            *ps.offset(pivotindex as isize) = j;
        }
        pivot = *(*lu.offset(*ps.offset(k as isize) as isize)).offset(k as isize);
        i = k + 1 as libc::c_int;
        while i < n {
            mult = *(*lu.offset(*ps.offset(i as isize) as isize)).offset(k as isize)
                / pivot;
            *(*lu.offset(*ps.offset(i as isize) as isize)).offset(k as isize) = mult;
            j = k + 1 as libc::c_int;
            while j < n {
                *(*lu.offset(*ps.offset(i as isize) as isize)).offset(j as isize)
                    -= mult
                        * *(*lu.offset(*ps.offset(k as isize) as isize))
                            .offset(j as isize);
                j += 1;
            }
            i += 1;
        }
        k += 1;
    }
    if *(*lu.offset(*ps.offset((n - 1 as libc::c_int) as isize) as isize))
        .offset((n - 1 as libc::c_int) as isize) == 0.0f64
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lu_solve(
    mut x: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dot: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < n {
        dot = 0.0f64;
        j = 0 as libc::c_int;
        while j < i {
            dot
                += *(*lu.offset(*ps.offset(i as isize) as isize)).offset(j as isize)
                    * *x.offset(j as isize);
            j += 1;
        }
        *x.offset(i as isize) = *b.offset(*ps.offset(i as isize) as isize) - dot;
        i += 1;
    }
    i = n - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        dot = 0.0f64;
        j = i + 1 as libc::c_int;
        while j < n {
            dot
                += *(*lu.offset(*ps.offset(i as isize) as isize)).offset(j as isize)
                    * *x.offset(j as isize);
            j += 1;
        }
        *x
            .offset(
                i as isize,
            ) = (*x.offset(i as isize) - dot)
            / *(*lu.offset(*ps.offset(i as isize) as isize)).offset(i as isize);
        i -= 1;
    }
}
