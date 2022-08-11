#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn free(_: *mut libc::c_void);
    fn SparseMatrix_multiply_vector(
        A: SparseMatrix,
        v: *mut libc::c_double,
        res: *mut *mut libc::c_double,
    );
    fn vector_saxpy2(
        n: libc::c_int,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        beta: libc::c_double,
    ) -> *mut libc::c_double;
    fn vector_saxpy(
        n: libc::c_int,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        beta: libc::c_double,
    ) -> *mut libc::c_double;
    fn vector_product(
        n: libc::c_int,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    ) -> libc::c_double;
    fn vector_subtract_to(
        n: libc::c_int,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    ) -> *mut libc::c_double;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
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
pub type SparseMatrix = *mut SparseMatrix_struct;
pub type C2RustUnnamed = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SOLVE_METHOD_JACOBI: C2RustUnnamed_0 = 1;
pub const SOLVE_METHOD_CG: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Operator_struct {
    pub data: *mut libc::c_void,
    pub Operator_apply: Option::<
        unsafe extern "C" fn(
            Operator,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> *mut libc::c_double,
    >,
}
pub type Operator = *mut Operator_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uniform_stress_matmul_data {
    pub alpha: libc::c_double,
    pub A: SparseMatrix,
}
unsafe extern "C" fn Operator_uniform_stress_matmul_apply(
    mut o: Operator,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> *mut libc::c_double {
    let mut d: *mut uniform_stress_matmul_data = (*o).data
        as *mut uniform_stress_matmul_data;
    let mut A: SparseMatrix = (*d).A;
    let mut alpha: libc::c_double = (*d).alpha;
    let mut xsum: libc::c_double = 0.0f64;
    let mut m: libc::c_int = (*A).m;
    let mut i: libc::c_int = 0;
    SparseMatrix_multiply_vector(A, x, &mut y);
    i = 0 as libc::c_int;
    while i < m {
        xsum += *x.offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        *y.offset(i as isize)
            += alpha * (m as libc::c_double * *x.offset(i as isize) - xsum);
        i += 1;
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn Operator_uniform_stress_matmul(
    mut A: SparseMatrix,
    mut alpha: libc::c_double,
) -> Operator {
    let mut o: Operator = 0 as *mut Operator_struct;
    let mut d: *mut uniform_stress_matmul_data = 0 as *mut uniform_stress_matmul_data;
    o = gmalloc(::std::mem::size_of::<Operator_struct>() as libc::c_ulong) as Operator;
    d = gmalloc(::std::mem::size_of::<uniform_stress_matmul_data>() as libc::c_ulong)
        as *mut uniform_stress_matmul_data;
    let ref mut fresh0 = (*o).data;
    *fresh0 = d as *mut libc::c_void;
    (*d).alpha = alpha;
    let ref mut fresh1 = (*d).A;
    *fresh1 = A;
    let ref mut fresh2 = (*o).Operator_apply;
    *fresh2 = Some(
        Operator_uniform_stress_matmul_apply
            as unsafe extern "C" fn(
                Operator,
                *mut libc::c_double,
                *mut libc::c_double,
            ) -> *mut libc::c_double,
    );
    return o;
}
unsafe extern "C" fn Operator_matmul_apply(
    mut o: Operator,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> *mut libc::c_double {
    let mut A: SparseMatrix = (*o).data as SparseMatrix;
    SparseMatrix_multiply_vector(A, x, &mut y);
    return y;
}
unsafe extern "C" fn Operator_matmul_new(mut A: SparseMatrix) -> Operator {
    let mut o: Operator = 0 as *mut Operator_struct;
    o = gmalloc(::std::mem::size_of::<Operator_struct>() as libc::c_ulong)
        as *mut Operator_struct;
    let ref mut fresh3 = (*o).data;
    *fresh3 = A as *mut libc::c_void;
    let ref mut fresh4 = (*o).Operator_apply;
    *fresh4 = Some(
        Operator_matmul_apply
            as unsafe extern "C" fn(
                Operator,
                *mut libc::c_double,
                *mut libc::c_double,
            ) -> *mut libc::c_double,
    );
    return o;
}
unsafe extern "C" fn Operator_matmul_delete(mut o: Operator) {
    free(o as *mut libc::c_void);
}
unsafe extern "C" fn Operator_diag_precon_apply(
    mut o: Operator,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> *mut libc::c_double {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut diag: *mut libc::c_double = (*o).data as *mut libc::c_double;
    m = *diag.offset(0 as libc::c_int as isize) as libc::c_int;
    diag = diag.offset(1);
    i = 0 as libc::c_int;
    while i < m {
        *y.offset(i as isize) = *x.offset(i as isize) * *diag.offset(i as isize);
        i += 1;
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn Operator_uniform_stress_diag_precon_new(
    mut A: SparseMatrix,
    mut alpha: libc::c_double,
) -> Operator {
    let mut o: Operator = 0 as *mut Operator_struct;
    let mut diag: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"sparse_solve.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"Operator Operator_uniform_stress_diag_precon_new(SparseMatrix, double)\0",
            ))
                .as_ptr(),
        );
    }
    if !a.is_null() {} else {
        __assert_fail(
            b"a\0" as *const u8 as *const libc::c_char,
            b"sparse_solve.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"Operator Operator_uniform_stress_diag_precon_new(SparseMatrix, double)\0",
            ))
                .as_ptr(),
        );
    }
    o = gmalloc(::std::mem::size_of::<Operator_struct>() as libc::c_ulong) as Operator;
    let ref mut fresh5 = (*o).data;
    *fresh5 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((m + 1 as libc::c_int) as libc::c_ulong),
    );
    diag = (*o).data as *mut libc::c_double;
    *diag.offset(0 as libc::c_int as isize) = m as libc::c_double;
    diag = diag.offset(1);
    i = 0 as libc::c_int;
    while i < m {
        *diag.offset(i as isize) = 1.0f64 / (m - 1 as libc::c_int) as libc::c_double;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if i == *ja.offset(j as isize)
                && fabs(*a.offset(j as isize)) > 0 as libc::c_int as libc::c_double
            {
                *diag
                    .offset(
                        i as isize,
                    ) = 1.0f64
                    / ((m - 1 as libc::c_int) as libc::c_double * alpha
                        + *a.offset(j as isize));
            }
            j += 1;
        }
        i += 1;
    }
    let ref mut fresh6 = (*o).Operator_apply;
    *fresh6 = Some(
        Operator_diag_precon_apply
            as unsafe extern "C" fn(
                Operator,
                *mut libc::c_double,
                *mut libc::c_double,
            ) -> *mut libc::c_double,
    );
    return o;
}
unsafe extern "C" fn Operator_diag_precon_new(mut A: SparseMatrix) -> Operator {
    let mut o: Operator = 0 as *mut Operator_struct;
    let mut diag: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"sparse_solve.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"Operator Operator_diag_precon_new(SparseMatrix)\0"))
                .as_ptr(),
        );
    }
    if !a.is_null() {} else {
        __assert_fail(
            b"a\0" as *const u8 as *const libc::c_char,
            b"sparse_solve.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"Operator Operator_diag_precon_new(SparseMatrix)\0"))
                .as_ptr(),
        );
    }
    o = gcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<Operator_struct>() as libc::c_ulong,
    ) as *mut Operator_struct;
    let ref mut fresh7 = (*o).data;
    *fresh7 = gcalloc(
        ((*A).m + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double as *mut libc::c_void;
    diag = (*o).data as *mut libc::c_double;
    *diag.offset(0 as libc::c_int as isize) = m as libc::c_double;
    diag = diag.offset(1);
    i = 0 as libc::c_int;
    while i < m {
        *diag.offset(i as isize) = 1.0f64;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if i == *ja.offset(j as isize)
                && fabs(*a.offset(j as isize)) > 0 as libc::c_int as libc::c_double
            {
                *diag.offset(i as isize) = 1.0f64 / *a.offset(j as isize);
            }
            j += 1;
        }
        i += 1;
    }
    let ref mut fresh8 = (*o).Operator_apply;
    *fresh8 = Some(
        Operator_diag_precon_apply
            as unsafe extern "C" fn(
                Operator,
                *mut libc::c_double,
                *mut libc::c_double,
            ) -> *mut libc::c_double,
    );
    return o;
}
unsafe extern "C" fn Operator_diag_precon_delete(mut o: Operator) {
    free((*o).data);
    free(o as *mut libc::c_void);
}
unsafe extern "C" fn conjugate_gradient(
    mut A: Operator,
    mut precon: Operator,
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
    mut rhs: *mut libc::c_double,
    mut tol: libc::c_double,
    mut maxit: libc::c_int,
) -> libc::c_double {
    let mut z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut r: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut q: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut res: libc::c_double = 10 as libc::c_int as libc::c_double * tol;
    let mut alpha: libc::c_double = 0.;
    let mut rho: libc::c_double = 1.0e20f64;
    let mut rho_old: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut res0: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut Ax: Option::<
        unsafe extern "C" fn(
            Operator,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> *mut libc::c_double,
    > = (*A).Operator_apply;
    let mut Minvx: Option::<
        unsafe extern "C" fn(
            Operator,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> *mut libc::c_double,
    > = (*precon).Operator_apply;
    let mut iter: libc::c_int = 0 as libc::c_int;
    z = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    r = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    p = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    q = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    r = Ax.expect("non-null function pointer")(A, x, r);
    r = vector_subtract_to(n, rhs, r);
    res = sqrt(vector_product(n, r, r)) / n as libc::c_double;
    res0 = res;
    loop {
        let fresh9 = iter;
        iter = iter + 1;
        if !(fresh9 < maxit && res > tol * res0) {
            break;
        }
        z = Minvx.expect("non-null function pointer")(precon, r, z);
        rho = vector_product(n, r, z);
        if iter > 1 as libc::c_int {
            beta = rho / rho_old;
            p = vector_saxpy(n, z, p, beta);
        } else {
            memcpy(
                p as *mut libc::c_void,
                z as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
        }
        q = Ax.expect("non-null function pointer")(A, p, q);
        alpha = rho / vector_product(n, p, q);
        x = vector_saxpy2(n, x, p, alpha);
        r = vector_saxpy2(n, r, q, -alpha);
        res = sqrt(vector_product(n, r, r)) / n as libc::c_double;
        rho_old = rho;
    }
    free(z as *mut libc::c_void);
    free(r as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(q as *mut libc::c_void);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn cg(
    mut Ax: Operator,
    mut precond: Operator,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x0: *mut libc::c_double,
    mut rhs: *mut libc::c_double,
    mut tol: libc::c_double,
    mut maxit: libc::c_int,
) -> libc::c_double {
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut b: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    x = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    b = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    k = 0 as libc::c_int;
    while k < dim {
        i = 0 as libc::c_int;
        while i < n {
            *x.offset(i as isize) = *x0.offset((i * dim + k) as isize);
            *b.offset(i as isize) = *rhs.offset((i * dim + k) as isize);
            i += 1;
        }
        res += conjugate_gradient(Ax, precond, n, x, b, tol, maxit);
        i = 0 as libc::c_int;
        while i < n {
            *rhs.offset((i * dim + k) as isize) = *x.offset(i as isize);
            i += 1;
        }
        k += 1;
    }
    free(x as *mut libc::c_void);
    free(b as *mut libc::c_void);
    return res;
}
unsafe extern "C" fn jacobi(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut x0: *mut libc::c_double,
    mut rhs: *mut libc::c_double,
    mut maxit: libc::c_int,
    mut flag: *mut libc::c_int,
) -> *mut libc::c_double {
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut b: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut sum: libc::c_double = 0.;
    let mut diag: libc::c_double = 0.;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = (*A).n;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut iter: libc::c_int = 0;
    x = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    y = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    b = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"sparse_solve.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"double *jacobi(SparseMatrix, int, double *, double *, int, int *)\0"))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    a = (*A).a as *mut libc::c_double;
    k = 0 as libc::c_int;
    while k < dim {
        i = 0 as libc::c_int;
        while i < n {
            *x.offset(i as isize) = *x0.offset((i * dim + k) as isize);
            *b.offset(i as isize) = *rhs.offset((i * dim + k) as isize);
            i += 1;
        }
        iter = 0 as libc::c_int;
        while iter < maxit {
            i = 0 as libc::c_int;
            while i < n {
                sum = 0 as libc::c_int as libc::c_double;
                diag = 0 as libc::c_int as libc::c_double;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) != i {
                        sum
                            += *a.offset(j as isize)
                                * *x.offset(*ja.offset(j as isize) as isize);
                    } else {
                        diag = *a.offset(j as isize);
                    }
                    j += 1;
                }
                if sum == 0 as libc::c_int as libc::c_double {
                    fprintf(
                        stderr,
                        b"neighb=%d\n\0" as *const u8 as *const libc::c_char,
                        *ia.offset((i + 1 as libc::c_int) as isize)
                            - *ia.offset(i as isize),
                    );
                }
                if diag != 0 as libc::c_int as libc::c_double {} else {
                    __assert_fail(
                        b"diag != 0\0" as *const u8 as *const libc::c_char,
                        b"sparse_solve.c\0" as *const u8 as *const libc::c_char,
                        269 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 66],
                            &[libc::c_char; 66],
                        >(
                            b"double *jacobi(SparseMatrix, int, double *, double *, int, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *y.offset(i as isize) = (*b.offset(i as isize) - sum) / diag;
                i += 1;
            }
            memcpy(
                x as *mut libc::c_void,
                y as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
            iter += 1;
        }
        i = 0 as libc::c_int;
        while i < n {
            *rhs.offset((i * dim + k) as isize) = *x.offset(i as isize);
            i += 1;
        }
        k += 1;
    }
    free(x as *mut libc::c_void);
    free(y as *mut libc::c_void);
    free(b as *mut libc::c_void);
    return rhs;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_solve(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut x0: *mut libc::c_double,
    mut rhs: *mut libc::c_double,
    mut tol: libc::c_double,
    mut maxit: libc::c_int,
    mut method: libc::c_int,
    mut flag: *mut libc::c_int,
) -> libc::c_double {
    let mut Ax: Operator = 0 as *mut Operator_struct;
    let mut precond: Operator = 0 as *mut Operator_struct;
    let mut n: libc::c_int = (*A).m;
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    *flag = 0 as libc::c_int;
    match method {
        0 => {
            Ax = Operator_matmul_new(A);
            precond = Operator_diag_precon_new(A);
            res = cg(Ax, precond, n, dim, x0, rhs, tol, maxit);
            Operator_matmul_delete(Ax);
            Operator_diag_precon_delete(precond);
        }
        1 => {
            jacobi(A, dim, x0, rhs, maxit, flag);
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"sparse_solve.c\0" as *const u8 as *const libc::c_char,
                307 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"double SparseMatrix_solve(SparseMatrix, int, double *, double *, double, int, int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    return res;
}
