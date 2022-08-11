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
#![feature(label_break_value, register_tool)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn srand(__seed: libc::c_uint);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn drand() -> libc::c_double;
    fn scale_to_box(
        xmin: libc::c_double,
        ymin: libc::c_double,
        xmax: libc::c_double,
        ymax: libc::c_double,
        n: libc::c_int,
        dim: libc::c_int,
        x: *mut libc::c_double,
    );
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_is_symmetric(A: SparseMatrix, test_pattern_symmetry_only: bool) -> libc::c_int;
    fn SparseMatrix_symmetrize(A: SparseMatrix, pattern_symmetric_only: bool) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(A: SparseMatrix) -> SparseMatrix;
    fn StressMajorizationSmoother_delete(sm: StressMajorizationSmoother);
    fn StressMajorizationSmoother_smooth(
        sm: StressMajorizationSmoother,
        dim: libc::c_int,
        x: *mut libc::c_double,
        maxit: libc::c_int,
        tol: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const FORMAT_COORD: C2RustUnnamed = 2;
pub const FORMAT_CSR: C2RustUnnamed = 1;
pub const FORMAT_CSC: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_0 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_0 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_0 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_0 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SM_SCHEME_STRESS: C2RustUnnamed_1 = 5;
pub const SM_SCHEME_STRESS_APPROX: C2RustUnnamed_1 = 4;
pub const SM_SCHEME_MAXENT: C2RustUnnamed_1 = 3;
pub const SM_SCHEME_UNIFORM_STRESS: C2RustUnnamed_1 = 2;
pub const SM_SCHEME_NORMAL_ELABEL: C2RustUnnamed_1 = 1;
pub const SM_SCHEME_NORMAL: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StressMajorizationSmoother_struct {
    pub D: SparseMatrix,
    pub Lw: SparseMatrix,
    pub Lwd: SparseMatrix,
    pub lambda: *mut libc::c_double,
    pub data_deallocator: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub data: *mut libc::c_void,
    pub scheme: libc::c_int,
    pub scaling: libc::c_double,
    pub tol_cg: libc::c_double,
    pub maxit_cg: libc::c_int,
}
pub type StressMajorizationSmoother = *mut StressMajorizationSmoother_struct;
pub type UniformStressSmoother = StressMajorizationSmoother;
#[no_mangle]
pub unsafe extern "C" fn UniformStressSmoother_new(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
    mut alpha: libc::c_double,
    mut M: libc::c_double,
    mut flag: *mut libc::c_int,
) -> UniformStressSmoother {
    let mut sm: UniformStressSmoother = 0 as *mut StressMajorizationSmoother_struct;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut iw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
    let mut diag_d: libc::c_double = 0.;
    let mut diag_w: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut epsilon: libc::c_double = 0.01f64;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"uniform_stress.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 100],
                &[libc::c_char; 100],
            >(
                b"UniformStressSmoother UniformStressSmoother_new(int, SparseMatrix, double *, double, double, int *)\0",
            ))
                .as_ptr(),
        );
    }
    sm = gmalloc(::std::mem::size_of::<StressMajorizationSmoother_struct>() as libc::c_ulong)
        as UniformStressSmoother;
    let ref mut fresh0 = (*sm).data;
    *fresh0 = 0 as *mut libc::c_void;
    (*sm).scheme = SM_SCHEME_UNIFORM_STRESS as libc::c_int;
    let ref mut fresh1 = (*sm).lambda;
    *fresh1 = 0 as *mut libc::c_double;
    let ref mut fresh2 = (*sm).data;
    *fresh2 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    );
    *((*sm).data as *mut libc::c_double).offset(0 as libc::c_int as isize) = alpha;
    *((*sm).data as *mut libc::c_double).offset(1 as libc::c_int as isize) = M;
    let ref mut fresh3 = (*sm).data_deallocator;
    *fresh3 = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*sm).tol_cg = 0.01f64;
    (*sm).maxit_cg = sqrt((*A).m as libc::c_double) as libc::c_int;
    let ref mut fresh4 = (*sm).Lw;
    *fresh4 = SparseMatrix_new(
        m,
        m,
        (*A).nz + m,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    let ref mut fresh5 = (*sm).Lwd;
    *fresh5 = SparseMatrix_new(
        m,
        m,
        (*A).nz + m,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    if ((*sm).Lw).is_null() || ((*sm).Lwd).is_null() {
        StressMajorizationSmoother_delete(sm);
        return 0 as UniformStressSmoother;
    }
    iw = (*(*sm).Lw).ia;
    jw = (*(*sm).Lw).ja;
    id = (*(*sm).Lwd).ia;
    jd = (*(*sm).Lwd).ja;
    w = (*(*sm).Lw).a as *mut libc::c_double;
    d = (*(*sm).Lwd).a as *mut libc::c_double;
    let ref mut fresh6 = *id.offset(0 as libc::c_int as isize);
    *fresh6 = 0 as libc::c_int;
    *iw.offset(0 as libc::c_int as isize) = *fresh6;
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        diag_w = 0 as libc::c_int as libc::c_double;
        diag_d = diag_w;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            if k != i {
                dist = if fabs(*a.offset(j as isize)) > epsilon {
                    fabs(*a.offset(j as isize))
                } else {
                    epsilon
                };
                let ref mut fresh7 = *jw.offset(nz as isize);
                *fresh7 = k;
                *jd.offset(nz as isize) = *fresh7;
                *w.offset(nz as isize) = -(1 as libc::c_int) as libc::c_double / (dist * dist);
                *w.offset(nz as isize) = -1.0f64;
                *d.offset(nz as isize) = *w.offset(nz as isize) * dist;
                diag_w += *w.offset(nz as isize);
                diag_d += *d.offset(nz as isize);
                nz += 1;
            }
            j += 1;
        }
        let ref mut fresh8 = *jw.offset(nz as isize);
        *fresh8 = i;
        *jd.offset(nz as isize) = *fresh8;
        *w.offset(nz as isize) = -diag_w;
        *d.offset(nz as isize) = -diag_d;
        nz += 1;
        *iw.offset((i + 1 as libc::c_int) as isize) = nz;
        *id.offset((i + 1 as libc::c_int) as isize) = nz;
        i += 1;
    }
    (*(*sm).Lw).nz = nz;
    (*(*sm).Lwd).nz = nz;
    return sm;
}
#[no_mangle]
pub unsafe extern "C" fn UniformStressSmoother_delete(mut sm: UniformStressSmoother) {
    StressMajorizationSmoother_delete(sm);
}
unsafe extern "C" fn UniformStressSmoother_smooth(
    mut sm: UniformStressSmoother,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut maxit_sm: libc::c_int,
) -> libc::c_double {
    return StressMajorizationSmoother_smooth(sm, dim, x, maxit_sm, 0.001f64);
}
unsafe extern "C" fn get_distance_matrix(
    mut A: SparseMatrix,
    mut scaling: libc::c_double,
) -> SparseMatrix {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {
        B = SparseMatrix_symmetrize(A, 0 as libc::c_int != 0);
    } else {
        B = SparseMatrix_get_real_adjacency_matrix_symmetrized(A);
    }
    val = (*B).a as *mut libc::c_double;
    if scaling != 1 as libc::c_int as libc::c_double {
        i = 0 as libc::c_int;
        while i < (*B).nz {
            *val.offset(i as isize) *= scaling;
            i += 1;
        }
    }
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn uniform_stress(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut sm: UniformStressSmoother = 0 as *mut StressMajorizationSmoother_struct;
    let mut lambda0: libc::c_double = 10.1f64;
    let mut M: libc::c_double = 100 as libc::c_int as libc::c_double;
    let mut scaling: libc::c_double = 1.0f64;
    let mut maxit: libc::c_int = 300 as libc::c_int;
    let mut samepoint: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = (*A).m;
    let mut B: SparseMatrix = 0 as SparseMatrix;
    *flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < dim * n {
        *x.offset(i as isize) = M * drand();
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            if fabs(
                *x.offset((0 as libc::c_int * dim + k) as isize)
                    - *x.offset((i * dim + k) as isize),
            ) > 1.0e-16f64
            {
                samepoint = 0 as libc::c_int;
                i = n;
                break;
            } else {
                k += 1;
            }
        }
        i += 1;
    }
    if samepoint != 0 {
        srand(1 as libc::c_int as libc::c_uint);
        i = 0 as libc::c_int;
        while i < dim * n {
            *x.offset(i as isize) = M * drand();
            i += 1;
        }
    }
    B = get_distance_matrix(A, scaling);
    if SparseMatrix_is_symmetric(B, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(B, false)\0" as *const u8 as *const libc::c_char,
            b"uniform_stress.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"void uniform_stress(int, SparseMatrix, double *, int *)\0",
            ))
            .as_ptr(),
        );
    }
    sm = UniformStressSmoother_new(
        dim,
        B,
        x,
        1000000 as libc::c_int as libc::c_double * lambda0,
        M,
        flag,
    );
    UniformStressSmoother_smooth(sm, dim, x, maxit);
    UniformStressSmoother_delete(sm);
    sm = UniformStressSmoother_new(
        dim,
        B,
        x,
        10000 as libc::c_int as libc::c_double * lambda0,
        M,
        flag,
    );
    UniformStressSmoother_smooth(sm, dim, x, maxit);
    UniformStressSmoother_delete(sm);
    sm = UniformStressSmoother_new(
        dim,
        B,
        x,
        100 as libc::c_int as libc::c_double * lambda0,
        M,
        flag,
    );
    UniformStressSmoother_smooth(sm, dim, x, maxit);
    UniformStressSmoother_delete(sm);
    sm = UniformStressSmoother_new(dim, B, x, lambda0, M, flag);
    UniformStressSmoother_smooth(sm, dim, x, maxit);
    UniformStressSmoother_delete(sm);
    scale_to_box(
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        (7 as libc::c_int * 70 as libc::c_int) as libc::c_double,
        (10 as libc::c_int * 70 as libc::c_int) as libc::c_double,
        (*A).m,
        dim,
        x,
    );
    SparseMatrix_delete(B);
}
