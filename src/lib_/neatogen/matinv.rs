#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn lu_decompose(a: *mut *mut libc::c_double, n: libc::c_int) -> libc::c_int;
    fn lu_solve(x: *mut libc::c_double, b: *mut libc::c_double, n: libc::c_int);
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn matinv(
    mut A: *mut *mut libc::c_double,
    mut Ainv: *mut *mut libc::c_double,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut temp: libc::c_double = 0.;
    if lu_decompose(A, n) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    b = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < n {
            *b.offset(j as isize) = 0.0f64;
            j += 1;
        }
        *b.offset(i as isize) = 1.0f64;
        lu_solve(*Ainv.offset(i as isize), b, n);
        i += 1;
    }
    free(b as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < i {
            temp = *(*Ainv.offset(i as isize)).offset(j as isize);
            *(*Ainv.offset(i as isize))
                .offset(j as isize) = *(*Ainv.offset(j as isize)).offset(i as isize);
            *(*Ainv.offset(j as isize)).offset(i as isize) = temp;
            j += 1;
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
