#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn srand(__seed: libc::c_uint);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn vector_product(
        n: libc::c_int,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    ) -> libc::c_double;
    fn drand() -> libc::c_double;
    fn SparseMatrix_multiply_vector(
        A: SparseMatrix,
        v: *mut libc::c_double,
        res: *mut *mut libc::c_double,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type SparseMatrix = *mut SparseMatrix_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseMatrix_struct {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nz: libc::c_int,
    pub nzmax: libc::c_int,
    pub type_0: libc::c_int,
    pub ia: *mut libc::c_int,
    pub ja: *mut libc::c_int,
    pub a: *mut libc::c_void,
    pub format: libc::c_int,
    pub property: libc::c_int,
    pub size: libc::c_int,
}
#[inline]
unsafe extern "C" fn gv_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut maxit: libc::c_int = 100 as libc::c_int;
static mut tolerance: libc::c_double = 0.00001f64;
#[no_mangle]
pub unsafe extern "C" fn power_method(
    mut A: *mut libc::c_void,
    mut n: libc::c_int,
    mut K: libc::c_int,
    mut random_seed: libc::c_int,
    mut eigv: *mut *mut libc::c_double,
    mut eigs: *mut libc::c_double,
) {
    let mut v: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vv: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_double = 0.;
    let mut unorm: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut uij: libc::c_double = 0.;
    K = if 0 as libc::c_int > (if n < K { n } else { K }) {
        0 as libc::c_int
    } else if n < K {
        n
    } else {
        K
    };
    if K <= n && K > 0 as libc::c_int {} else {
        __assert_fail(
            b"K <= n && K > 0\0" as *const u8 as *const libc::c_char,
            b"power.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void power_method(void *, int, int, int, double **, double *)\0"))
                .as_ptr(),
        );
    }
    if (*eigv).is_null() {
        *eigv = gv_calloc(
            (n * K) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
    }
    v = gv_calloc(
        K as size_t,
        ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut libc::c_double;
    vv = gv_calloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    u = gv_calloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    srand(random_seed as libc::c_uint);
    k = 0 as libc::c_int;
    while k < K {
        let ref mut fresh0 = *v.offset(k as isize);
        *fresh0 = &mut *(*eigv).offset((k * n) as isize) as *mut libc::c_double;
        i = 0 as libc::c_int;
        while i < n {
            *u.offset(i as isize) = drand();
            i += 1;
        }
        res = sqrt(vector_product(n, u, u));
        if res > 0 as libc::c_int as libc::c_double {
            res = 1 as libc::c_int as libc::c_double / res;
        }
        i = 0 as libc::c_int;
        while i < n {
            *u.offset(i as isize) = *u.offset(i as isize) * res;
            *(*v.offset(k as isize)).offset(i as isize) = *u.offset(i as isize);
            i += 1;
        }
        iter = 0 as libc::c_int;
        loop {
            j = 0 as libc::c_int;
            while j < k {
                uij = vector_product(n, u, *v.offset(j as isize));
                i = 0 as libc::c_int;
                while i < n {
                    *u
                        .offset(
                            i as isize,
                        ) = *u.offset(i as isize)
                        - uij * *(*v.offset(j as isize)).offset(i as isize);
                    i += 1;
                }
                j += 1;
            }
            SparseMatrix_multiply_vector(A as SparseMatrix, u, &mut vv);
            unorm = vector_product(n, vv, vv);
            unorm = sqrt(unorm);
            *eigs.offset(k as isize) = unorm;
            if unorm > 0 as libc::c_int as libc::c_double {
                unorm = 1 as libc::c_int as libc::c_double / unorm;
            } else {
                i = 0 as libc::c_int;
                while i < n {
                    *vv.offset(i as isize) = *u.offset(i as isize);
                    i += 1;
                }
                unorm = sqrt(vector_product(n, vv, vv));
                if unorm > 0 as libc::c_int as libc::c_double {
                    unorm = 1 as libc::c_int as libc::c_double / unorm;
                }
            }
            res = 0.0f64;
            i = 0 as libc::c_int;
            while i < n {
                *u.offset(i as isize) = *vv.offset(i as isize) * unorm;
                res = res
                    + *u.offset(i as isize)
                        * *(*v.offset(k as isize)).offset(i as isize);
                *(*v.offset(k as isize)).offset(i as isize) = *u.offset(i as isize);
                i += 1;
            }
            if !(res < 1 as libc::c_int as libc::c_double - tolerance
                && {
                    let fresh1 = iter;
                    iter = iter + 1;
                    fresh1 < maxit
                })
            {
                break;
            }
        }
        k += 1;
    }
    free(u as *mut libc::c_void);
    free(vv as *mut libc::c_void);
}
