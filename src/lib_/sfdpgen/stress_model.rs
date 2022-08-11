#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn srand(__seed: libc::c_uint);
    fn drand() -> libc::c_double;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_is_symmetric(
        A: SparseMatrix,
        test_pattern_symmetry_only: bool,
    ) -> libc::c_int;
    fn SparseMatrix_symmetrize(
        A: SparseMatrix,
        pattern_symmetric_only: bool,
    ) -> SparseMatrix;
    fn SparseMatrix_remove_diagonal(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(
        A: SparseMatrix,
    ) -> SparseMatrix;
    fn SparseStressMajorizationSmoother_delete(sm: SparseStressMajorizationSmoother);
    fn SparseStressMajorizationSmoother_new(
        A: SparseMatrix,
        dim: libc::c_int,
        lambda: libc::c_double,
        x: *mut libc::c_double,
        weighting_scheme: libc::c_int,
        scale_initial_coord: libc::c_int,
    ) -> SparseStressMajorizationSmoother;
    fn SparseStressMajorizationSmoother_smooth(
        sm: SparseStressMajorizationSmoother,
        dim: libc::c_int,
        x: *mut libc::c_double,
        maxit_sm: libc::c_int,
        tol: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
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
pub const SM_SCHEME_STRESS: C2RustUnnamed_0 = 5;
pub const SM_SCHEME_STRESS_APPROX: C2RustUnnamed_0 = 4;
pub const SM_SCHEME_MAXENT: C2RustUnnamed_0 = 3;
pub const SM_SCHEME_UNIFORM_STRESS: C2RustUnnamed_0 = 2;
pub const SM_SCHEME_NORMAL_ELABEL: C2RustUnnamed_0 = 1;
pub const SM_SCHEME_NORMAL: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StressMajorizationSmoother_struct {
    pub D: SparseMatrix,
    pub Lw: SparseMatrix,
    pub Lwd: SparseMatrix,
    pub lambda: *mut libc::c_double,
    pub data_deallocator: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub data: *mut libc::c_void,
    pub scheme: libc::c_int,
    pub scaling: libc::c_double,
    pub tol_cg: libc::c_double,
    pub maxit_cg: libc::c_int,
}
pub type StressMajorizationSmoother = *mut StressMajorizationSmoother_struct;
pub type SparseStressMajorizationSmoother = StressMajorizationSmoother;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const WEIGHTING_SCHEME_SQR_DIST: C2RustUnnamed_1 = 2;
pub const WEIGHTING_SCHEME_INV_DIST: C2RustUnnamed_1 = 1;
pub const WEIGHTING_SCHEME_NONE: C2RustUnnamed_1 = 0;
unsafe extern "C" fn stress_model_core(
    mut dim: libc::c_int,
    mut B: SparseMatrix,
    mut x: *mut *mut libc::c_double,
    mut edge_len_weighted: libc::c_int,
    mut maxit_sm: libc::c_int,
    mut tol: libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut m: libc::c_int = 0;
    let mut sm: SparseStressMajorizationSmoother = 0
        as *mut StressMajorizationSmoother_struct;
    let mut lambda: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut A: SparseMatrix = B;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) == 0
        || (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int
    {
        if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {
            A = SparseMatrix_symmetrize(A, 0 as libc::c_int != 0);
            A = SparseMatrix_remove_diagonal(A);
        } else {
            A = SparseMatrix_get_real_adjacency_matrix_symmetrized(A);
        }
    }
    A = SparseMatrix_remove_diagonal(A);
    *flag = 0 as libc::c_int;
    m = (*A).m;
    if x.is_null() {
        *x = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(m as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        srand(123 as libc::c_int as libc::c_uint);
        i = 0 as libc::c_int;
        while i < dim * m {
            *(*x).offset(i as isize) = drand();
            i += 1;
        }
    }
    if edge_len_weighted != 0 {
        sm = SparseStressMajorizationSmoother_new(
            A,
            dim,
            lambda,
            *x,
            WEIGHTING_SCHEME_SQR_DIST as libc::c_int,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    } else {
        sm = SparseStressMajorizationSmoother_new(
            A,
            dim,
            lambda,
            *x,
            WEIGHTING_SCHEME_NONE as libc::c_int,
            (0 as libc::c_int == 0) as libc::c_int,
        );
    }
    if sm.is_null() {
        *flag = -(1 as libc::c_int);
    } else {
        (*sm).tol_cg = 0.1f64;
        (*sm).scheme = SM_SCHEME_STRESS as libc::c_int;
        SparseStressMajorizationSmoother_smooth(sm, dim, *x, maxit_sm, tol);
        i = 0 as libc::c_int;
        while i < dim * m {
            *(*x).offset(i as isize) /= (*sm).scaling;
            i += 1;
        }
        SparseStressMajorizationSmoother_delete(sm);
    }
    if A != B {
        SparseMatrix_delete(A);
    }
}
#[no_mangle]
pub unsafe extern "C" fn stress_model(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut D: SparseMatrix,
    mut x: *mut *mut libc::c_double,
    mut edge_len_weighted: libc::c_int,
    mut maxit_sm: libc::c_int,
    mut tol: libc::c_double,
    mut flag: *mut libc::c_int,
) {
    stress_model_core(dim, D, x, edge_len_weighted, maxit_sm, tol, flag);
}
