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
    fn orthog1(n: libc::c_int, vec: *mut libc::c_double);
    fn right_mult_with_vector(
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
    );
    fn right_mult_with_vector_f(
        _: *mut *mut libc::c_float,
        _: libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
    );
    fn vectors_subtraction(
        _: libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
    );
    fn vectors_addition(
        _: libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
    );
    fn vectors_scalar_mult(
        _: libc::c_int,
        _: *mut libc::c_double,
        _: libc::c_double,
        _: *mut libc::c_double,
    );
    fn copy_vector(n: libc::c_int, source: *mut libc::c_double, dest: *mut libc::c_double);
    fn vectors_inner_product(
        n: libc::c_int,
        vector1: *mut libc::c_double,
        vector2: *mut libc::c_double,
    ) -> libc::c_double;
    fn max_abs(n: libc::c_int, vector: *mut libc::c_double) -> libc::c_double;
    fn orthog1f(n: libc::c_int, vec: *mut libc::c_float);
    fn right_mult_with_vector_ff(
        _: *mut libc::c_float,
        _: libc::c_int,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
    );
    fn vectors_substractionf(
        _: libc::c_int,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
    );
    fn vectors_mult_additionf(
        n: libc::c_int,
        vector1: *mut libc::c_float,
        alpha: libc::c_float,
        vector2: *mut libc::c_float,
    );
    fn copy_vectorf(n: libc::c_int, source: *mut libc::c_float, dest: *mut libc::c_float);
    fn vectors_inner_productf(
        n: libc::c_int,
        vector1: *mut libc::c_float,
        vector2: *mut libc::c_float,
    ) -> libc::c_double;
    fn max_absf(n: libc::c_int, vector: *mut libc::c_float) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vtx_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
    pub eweights: *mut libc::c_float,
    pub edists: *mut libc::c_float,
}
pub type size_t = libc::c_ulong;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[no_mangle]
pub unsafe extern "C" fn conjugate_gradient(
    mut A: *mut vtx_data,
    mut x: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut n: libc::c_int,
    mut tol: libc::c_double,
    mut max_iterations: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut r_r: libc::c_double = 0.;
    let mut r_r_new: libc::c_double = 0.;
    let mut p_Ap: libc::c_double = 0.;
    let mut r: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut p: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Ap: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Ax: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut alphap: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut orth_b: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    copy_vector(n, b, orth_b);
    orthog1(n, orth_b);
    orthog1(n, x);
    right_mult_with_vector(A, n, x, Ax);
    vectors_subtraction(n, orth_b, Ax, r);
    copy_vector(n, r, p);
    r_r = vectors_inner_product(n, r, r);
    i = 0 as libc::c_int;
    while i < max_iterations && max_abs(n, r) > tol {
        right_mult_with_vector(A, n, p, Ap);
        p_Ap = vectors_inner_product(n, p, Ap);
        if p_Ap == 0 as libc::c_int as libc::c_double {
            break;
        }
        alpha = r_r / p_Ap;
        vectors_scalar_mult(n, p, alpha, alphap);
        vectors_addition(n, x, alphap, x);
        if i < max_iterations - 1 as libc::c_int {
            vectors_scalar_mult(n, Ap, alpha, Ap);
            vectors_subtraction(n, r, Ap, r);
            r_r_new = vectors_inner_product(n, r, r);
            if r_r == 0 as libc::c_int as libc::c_double {
                agerr(
                    AGERR,
                    b"conjugate_gradient: unexpected length 0 vector\n\0" as *const u8
                        as *const libc::c_char,
                );
                rv = 1 as libc::c_int;
                break;
            } else {
                beta = r_r_new / r_r;
                r_r = r_r_new;
                vectors_scalar_mult(n, p, beta, p);
                vectors_addition(n, r, p, p);
            }
        }
        i += 1;
    }
    free(r as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(Ap as *mut libc::c_void);
    free(Ax as *mut libc::c_void);
    free(alphap as *mut libc::c_void);
    free(orth_b as *mut libc::c_void);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn conjugate_gradient_f(
    mut A: *mut *mut libc::c_float,
    mut x: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut n: libc::c_int,
    mut tol: libc::c_double,
    mut max_iterations: libc::c_int,
    mut ortho1: bool,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut r_r: libc::c_double = 0.;
    let mut r_r_new: libc::c_double = 0.;
    let mut p_Ap: libc::c_double = 0.;
    let mut r: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut p: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Ap: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Ax: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut alphap: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut orth_b: *mut libc::c_double = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    copy_vector(n, b, orth_b);
    if ortho1 {
        orthog1(n, orth_b);
        orthog1(n, x);
    }
    right_mult_with_vector_f(A, n, x, Ax);
    vectors_subtraction(n, orth_b, Ax, r);
    copy_vector(n, r, p);
    r_r = vectors_inner_product(n, r, r);
    i = 0 as libc::c_int;
    while i < max_iterations && max_abs(n, r) > tol {
        right_mult_with_vector_f(A, n, p, Ap);
        p_Ap = vectors_inner_product(n, p, Ap);
        if p_Ap == 0 as libc::c_int as libc::c_double {
            break;
        }
        alpha = r_r / p_Ap;
        vectors_scalar_mult(n, p, alpha, alphap);
        vectors_addition(n, x, alphap, x);
        if i < max_iterations - 1 as libc::c_int {
            vectors_scalar_mult(n, Ap, alpha, Ap);
            vectors_subtraction(n, r, Ap, r);
            r_r_new = vectors_inner_product(n, r, r);
            if r_r == 0 as libc::c_int as libc::c_double {
                rv = 1 as libc::c_int;
                agerr(
                    AGERR,
                    b"conjugate_gradient: unexpected length 0 vector\n\0" as *const u8
                        as *const libc::c_char,
                );
                break;
            } else {
                beta = r_r_new / r_r;
                r_r = r_r_new;
                vectors_scalar_mult(n, p, beta, p);
                vectors_addition(n, r, p, p);
            }
        }
        i += 1;
    }
    free(r as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(Ap as *mut libc::c_void);
    free(Ax as *mut libc::c_void);
    free(alphap as *mut libc::c_void);
    free(orth_b as *mut libc::c_void);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn conjugate_gradient_mkernel(
    mut A: *mut libc::c_float,
    mut x: *mut libc::c_float,
    mut b: *mut libc::c_float,
    mut n: libc::c_int,
    mut tol: libc::c_double,
    mut max_iterations: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut r_r: libc::c_double = 0.;
    let mut r_r_new: libc::c_double = 0.;
    let mut p_Ap: libc::c_double = 0.;
    let mut r: *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut p: *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut Ap: *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut Ax: *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    orthog1f(n, x);
    orthog1f(n, b);
    right_mult_with_vector_ff(A, n, x, Ax);
    orthog1f(n, Ax);
    vectors_substractionf(n, b, Ax, r);
    copy_vectorf(n, r, p);
    r_r = vectors_inner_productf(n, r, r);
    i = 0 as libc::c_int;
    while i < max_iterations && max_absf(n, r) > tol {
        orthog1f(n, p);
        orthog1f(n, x);
        orthog1f(n, r);
        right_mult_with_vector_ff(A, n, p, Ap);
        orthog1f(n, Ap);
        p_Ap = vectors_inner_productf(n, p, Ap);
        if p_Ap == 0 as libc::c_int as libc::c_double {
            break;
        }
        alpha = r_r / p_Ap;
        vectors_mult_additionf(n, x, alpha as libc::c_float, p);
        if i < max_iterations - 1 as libc::c_int {
            vectors_mult_additionf(n, r, -alpha as libc::c_float, Ap);
            r_r_new = vectors_inner_productf(n, r, r);
            if r_r == 0 as libc::c_int as libc::c_double {
                rv = 1 as libc::c_int;
                agerr(
                    AGERR,
                    b"conjugate_gradient: unexpected length 0 vector\n\0" as *const u8
                        as *const libc::c_char,
                );
                break;
            } else {
                beta = r_r_new / r_r;
                r_r = r_r_new;
                let mut j: size_t = 0 as libc::c_int as size_t;
                while j < n as size_t {
                    *p.offset(j as isize) =
                        beta as libc::c_float * *p.offset(j as isize) + *r.offset(j as isize);
                    j = j.wrapping_add(1);
                }
            }
        }
        i += 1;
    }
    free(r as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(Ap as *mut libc::c_void);
    free(Ax as *mut libc::c_void);
    return rv;
}
