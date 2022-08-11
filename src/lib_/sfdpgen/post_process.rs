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
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn cg(
        Ax: Operator,
        precond: Operator,
        n: libc::c_int,
        dim: libc::c_int,
        x0: *mut libc::c_double,
        rhs: *mut libc::c_double,
        tol: libc::c_double,
        maxit: libc::c_int,
    ) -> libc::c_double;
    fn SparseMatrix_solve(
        A: SparseMatrix,
        dim: libc::c_int,
        x0: *mut libc::c_double,
        rhs: *mut libc::c_double,
        tol: libc::c_double,
        maxit: libc::c_int,
        method: libc::c_int,
        flag: *mut libc::c_int,
    ) -> libc::c_double;
    fn Operator_uniform_stress_matmul(A: SparseMatrix, alpha: libc::c_double) -> Operator;
    fn Operator_uniform_stress_diag_precon_new(A: SparseMatrix, alpha: libc::c_double) -> Operator;
    fn SparseMatrix_add(A: SparseMatrix, B: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_from_coordinate_arrays(
        nz: libc::c_int,
        m: libc::c_int,
        n: libc::c_int,
        irn: *mut libc::c_int,
        jcn: *mut libc::c_int,
        val: *mut libc::c_void,
        type_0: libc::c_int,
        sz: size_t,
    ) -> SparseMatrix;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn distance_cropped(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn distance(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn drand() -> libc::c_double;
    fn vector_product(
        n: libc::c_int,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    ) -> libc::c_double;
    fn SparseMatrix_copy(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_is_symmetric(A: SparseMatrix, test_pattern_symmetry_only: bool) -> libc::c_int;
    fn SparseMatrix_multiply_dense(
        A: SparseMatrix,
        v: *mut libc::c_double,
        res: *mut *mut libc::c_double,
        dim: libc::c_int,
    );
    fn spring_electrical_control_new() -> spring_electrical_control;
    fn spring_electrical_control_delete(ctrl: spring_electrical_control);
    fn spring_electrical_spring_embedding(
        dim: libc::c_int,
        A: SparseMatrix,
        D: SparseMatrix,
        ctrl: spring_electrical_control,
        x: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    fn call_tri(n: libc::c_int, dim: libc::c_int, x: *mut libc::c_double) -> SparseMatrix;
    fn call_tri2(n: libc::c_int, dim: libc::c_int, x: *mut libc::c_double) -> SparseMatrix;
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
pub const SOLVE_METHOD_JACOBI: C2RustUnnamed_1 = 1;
pub const SOLVE_METHOD_CG: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Operator_struct {
    pub data: *mut libc::c_void,
    pub Operator_apply: Option<
        unsafe extern "C" fn(
            Operator,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> *mut libc::c_double,
    >,
}
pub type Operator = *mut Operator_struct;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SMOOTHING_RNG: C2RustUnnamed_2 = 6;
pub const SMOOTHING_TRIANGLE: C2RustUnnamed_2 = 5;
pub const SMOOTHING_SPRING: C2RustUnnamed_2 = 4;
pub const SMOOTHING_STRESS_MAJORIZATION_POWER_DIST: C2RustUnnamed_2 = 3;
pub const SMOOTHING_STRESS_MAJORIZATION_AVG_DIST: C2RustUnnamed_2 = 2;
pub const SMOOTHING_STRESS_MAJORIZATION_GRAPH_DIST: C2RustUnnamed_2 = 1;
pub const SMOOTHING_NONE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spring_electrical_control_struct {
    pub p: libc::c_double,
    pub q: libc::c_double,
    pub random_start: libc::c_int,
    pub K: libc::c_double,
    pub C: libc::c_double,
    pub multilevels: libc::c_int,
    pub multilevel_coarsen_scheme: libc::c_int,
    pub multilevel_coarsen_mode: libc::c_int,
    pub quadtree_size: libc::c_int,
    pub max_qtree_level: libc::c_int,
    pub bh: libc::c_double,
    pub tol: libc::c_double,
    pub maxiter: libc::c_int,
    pub cool: libc::c_double,
    pub step: libc::c_double,
    pub adaptive_cooling: libc::c_int,
    pub random_seed: libc::c_int,
    pub beautify_leaves: libc::c_int,
    pub smoothing: libc::c_int,
    pub overlap: libc::c_int,
    pub do_shrinking: libc::c_int,
    pub tscheme: libc::c_int,
    pub method: libc::c_int,
    pub initial_scaling: libc::c_double,
    pub rotation: libc::c_double,
    pub edge_labeling_scheme: libc::c_int,
}
pub type spring_electrical_control = *mut spring_electrical_control_struct;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SM_SCHEME_STRESS: C2RustUnnamed_3 = 5;
pub const SM_SCHEME_STRESS_APPROX: C2RustUnnamed_3 = 4;
pub const SM_SCHEME_MAXENT: C2RustUnnamed_3 = 3;
pub const SM_SCHEME_UNIFORM_STRESS: C2RustUnnamed_3 = 2;
pub const SM_SCHEME_NORMAL_ELABEL: C2RustUnnamed_3 = 1;
pub const SM_SCHEME_NORMAL: C2RustUnnamed_3 = 0;
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const IDEAL_POWER_DIST: C2RustUnnamed_4 = 2;
pub const IDEAL_AVG_DIST: C2RustUnnamed_4 = 1;
pub const IDEAL_GRAPH_DIST: C2RustUnnamed_4 = 0;
pub type relative_position_constraints = *mut relative_position_constraints_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relative_position_constraints_struct {
    pub constr_penalty: libc::c_double,
    pub edge_labeling_scheme: libc::c_int,
    pub n_constr_nodes: libc::c_int,
    pub constr_nodes: *mut libc::c_int,
    pub irn: *mut libc::c_int,
    pub jcn: *mut libc::c_int,
    pub val: *mut libc::c_double,
    pub A_constr: SparseMatrix,
}
pub const ELSCHEME_STRAIGHTLINE_PENALTY2: C2RustUnnamed_6 = 4;
pub const ELSCHEME_PENALTY2: C2RustUnnamed_6 = 2;
pub const ELSCHEME_STRAIGHTLINE_PENALTY: C2RustUnnamed_6 = 3;
pub const ELSCHEME_PENALTY: C2RustUnnamed_6 = 1;
pub type TriangleSmoother = StressMajorizationSmoother;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpringSmoother_struct {
    pub D: SparseMatrix,
    pub ctrl: spring_electrical_control,
}
pub type SpringSmoother = *mut SpringSmoother_struct;
pub type SparseStressMajorizationSmoother = StressMajorizationSmoother;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const WEIGHTING_SCHEME_SQR_DIST: C2RustUnnamed_5 = 2;
pub const WEIGHTING_SCHEME_INV_DIST: C2RustUnnamed_5 = 1;
pub const WEIGHTING_SCHEME_NONE: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const ELSCHEME_NONE: C2RustUnnamed_6 = 0;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
unsafe extern "C" fn ideal_distance_matrix(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) -> SparseMatrix {
    let mut D: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut len: libc::c_double = 0.;
    let mut di: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut sumd: libc::c_double = 0.;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"post_process.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"SparseMatrix ideal_distance_matrix(SparseMatrix, int, double *)\0",
            ))
            .as_ptr(),
        );
    }
    D = SparseMatrix_copy(A);
    ia = (*D).ia;
    ja = (*D).ja;
    if (*D).type_0 != MATRIX_TYPE_REAL as libc::c_int {
        free((*D).a);
        (*D).type_0 = MATRIX_TYPE_REAL as libc::c_int;
        let ref mut fresh0 = (*D).a;
        *fresh0 = gcalloc(
            (*D).nz as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double as *mut libc::c_void;
    }
    d = (*D).a as *mut libc::c_double;
    mask = gcalloc(
        (*D).m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*D).m {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*D).m {
        di = (*ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize))
            as libc::c_double;
        *mask.offset(i as isize) = i;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *ja.offset(j as isize)) {
                *mask.offset(*ja.offset(j as isize) as isize) = i;
            }
            j += 1;
        }
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            if !(i == k) {
                len = di
                    + (*ia.offset((k + 1 as libc::c_int) as isize) - *ia.offset(k as isize))
                        as libc::c_double;
                l = *ia.offset(k as isize);
                while l < *ia.offset((k + 1 as libc::c_int) as isize) {
                    if *mask.offset(*ja.offset(l as isize) as isize) == i {
                        len -= 1.;
                    }
                    l += 1;
                }
                *d.offset(j as isize) = len;
                if len > 0 as libc::c_int as libc::c_double {
                } else {
                    __assert_fail(
                        b"len > 0\0" as *const u8 as *const libc::c_char,
                        b"post_process.c\0" as *const u8 as *const libc::c_char,
                        72 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                            b"SparseMatrix ideal_distance_matrix(SparseMatrix, int, double *)\0",
                        ))
                        .as_ptr(),
                    );
                }
            }
            j += 1;
        }
        i += 1;
    }
    sum = 0 as libc::c_int as libc::c_double;
    sumd = 0 as libc::c_int as libc::c_double;
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*D).m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *ja.offset(j as isize)) {
                nz += 1;
                sum += distance(x, dim, i, *ja.offset(j as isize));
                sumd += *d.offset(j as isize);
            }
            j += 1;
        }
        i += 1;
    }
    sum /= nz as libc::c_double;
    sumd /= nz as libc::c_double;
    sum = sum / sumd;
    i = 0 as libc::c_int;
    while i < (*D).m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *ja.offset(j as isize)) {
                *d.offset(j as isize) = sum * *d.offset(j as isize);
            }
            j += 1;
        }
        i += 1;
    }
    return D;
}
#[no_mangle]
pub unsafe extern "C" fn StressMajorizationSmoother2_new(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut lambda0: libc::c_double,
    mut x: *mut libc::c_double,
    mut ideal_dist_scheme: libc::c_int,
) -> StressMajorizationSmoother {
    let mut sm: StressMajorizationSmoother = 0 as *mut StressMajorizationSmoother_struct;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut iw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lambda: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut avg_dist: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut diag_d: libc::c_double = 0.;
    let mut diag_w: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut s: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut stop: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sbot: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ID: SparseMatrix = 0 as *mut SparseMatrix_struct;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"post_process.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"StressMajorizationSmoother StressMajorizationSmoother2_new(SparseMatrix, int, double, double *, int)\0",
            ))
                .as_ptr(),
        );
    }
    ID = ideal_distance_matrix(A, dim, x);
    sm = gmalloc(::std::mem::size_of::<StressMajorizationSmoother_struct>() as libc::c_ulong)
        as *mut StressMajorizationSmoother_struct;
    (*sm).scaling = 1.0f64;
    let ref mut fresh1 = (*sm).data;
    *fresh1 = 0 as *mut libc::c_void;
    (*sm).scheme = SM_SCHEME_NORMAL as libc::c_int;
    (*sm).tol_cg = 0.01f64;
    (*sm).maxit_cg = sqrt((*A).m as libc::c_double) as libc::c_int;
    let ref mut fresh2 = (*sm).lambda;
    *fresh2 = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    lambda = *fresh2;
    i = 0 as libc::c_int;
    while i < m {
        *((*sm).lambda).offset(i as isize) = lambda0;
        i += 1;
    }
    mask = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    avg_dist = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        *avg_dist.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        nz = 0 as libc::c_int;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *ja.offset(j as isize)) {
                *avg_dist.offset(i as isize) += distance(x, dim, i, *ja.offset(j as isize));
                nz += 1;
            }
            j += 1;
        }
        if nz > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"nz > 0\0" as *const u8 as *const libc::c_char,
                b"post_process.c\0" as *const u8 as *const libc::c_char,
                139 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"StressMajorizationSmoother StressMajorizationSmoother2_new(SparseMatrix, int, double, double *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        *avg_dist.offset(i as isize) /= nz as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *mask.offset(i as isize) = i;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            if *mask.offset(k as isize) != i {
                *mask.offset(k as isize) = i;
                nz += 1;
            }
            j += 1;
        }
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            l = *ia.offset(k as isize);
            while l < *ia.offset((k + 1 as libc::c_int) as isize) {
                if *mask.offset(*ja.offset(l as isize) as isize) != i {
                    *mask.offset(*ja.offset(l as isize) as isize) = i;
                    nz += 1;
                }
                l += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let ref mut fresh3 = (*sm).Lw;
    *fresh3 = SparseMatrix_new(
        m,
        m,
        nz + m,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    let ref mut fresh4 = (*sm).Lwd;
    *fresh4 = SparseMatrix_new(
        m,
        m,
        nz + m,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    if ((*sm).Lw).is_null() || ((*sm).Lwd).is_null() {
        StressMajorizationSmoother_delete(sm);
        return 0 as StressMajorizationSmoother;
    }
    iw = (*(*sm).Lw).ia;
    jw = (*(*sm).Lw).ja;
    w = (*(*sm).Lw).a as *mut libc::c_double;
    d = (*(*sm).Lwd).a as *mut libc::c_double;
    id = (*(*sm).Lwd).ia;
    jd = (*(*sm).Lwd).ja;
    let ref mut fresh5 = *id.offset(0 as libc::c_int as isize);
    *fresh5 = 0 as libc::c_int;
    *iw.offset(0 as libc::c_int as isize) = *fresh5;
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *mask.offset(i as isize) = i + m;
        diag_w = 0 as libc::c_int as libc::c_double;
        diag_d = diag_w;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            if *mask.offset(k as isize) != i + m {
                *mask.offset(k as isize) = i + m;
                *jw.offset(nz as isize) = k;
                if ideal_dist_scheme == IDEAL_GRAPH_DIST as libc::c_int {
                    dist = 1 as libc::c_int as libc::c_double;
                } else if ideal_dist_scheme == IDEAL_AVG_DIST as libc::c_int {
                    dist = (*avg_dist.offset(i as isize) + *avg_dist.offset(k as isize)) * 0.5f64;
                } else if ideal_dist_scheme == IDEAL_POWER_DIST as libc::c_int {
                    dist = pow(distance_cropped(x, dim, i, k), 0.4f64);
                } else {
                    fprintf(
                        stderr,
                        b"ideal_dist_scheme value wrong\0" as *const u8 as *const libc::c_char,
                    );
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"post_process.c\0" as *const u8 as *const libc::c_char,
                        200 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 101],
                            &[libc::c_char; 101],
                        >(
                            b"StressMajorizationSmoother StressMajorizationSmoother2_new(SparseMatrix, int, double, double *, int)\0",
                        ))
                            .as_ptr(),
                    );
                    graphviz_exit(1 as libc::c_int);
                }
                *w.offset(nz as isize) = -(1 as libc::c_int) as libc::c_double / (dist * dist);
                diag_w += *w.offset(nz as isize);
                *jd.offset(nz as isize) = k;
                *d.offset(nz as isize) = *w.offset(nz as isize) * dist;
                stop += *d.offset(nz as isize) * distance(x, dim, i, k);
                sbot += *d.offset(nz as isize) * dist;
                diag_d += *d.offset(nz as isize);
                nz += 1;
            }
            j += 1;
        }
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            l = *ia.offset(k as isize);
            while l < *ia.offset((k + 1 as libc::c_int) as isize) {
                if *mask.offset(*ja.offset(l as isize) as isize) != i + m {
                    *mask.offset(*ja.offset(l as isize) as isize) = i + m;
                    if ideal_dist_scheme == IDEAL_GRAPH_DIST as libc::c_int {
                        dist = 2 as libc::c_int as libc::c_double;
                    } else if ideal_dist_scheme == IDEAL_AVG_DIST as libc::c_int {
                        dist = (*avg_dist.offset(i as isize)
                            + 2 as libc::c_int as libc::c_double * *avg_dist.offset(k as isize)
                            + *avg_dist.offset(*ja.offset(l as isize) as isize))
                            * 0.5f64;
                    } else if ideal_dist_scheme == IDEAL_POWER_DIST as libc::c_int {
                        dist = pow(distance_cropped(x, dim, i, *ja.offset(l as isize)), 0.4f64);
                    } else {
                        fprintf(
                            stderr,
                            b"ideal_dist_scheme value wrong\0" as *const u8 as *const libc::c_char,
                        );
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"post_process.c\0" as *const u8 as *const libc::c_char,
                            242 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 101],
                                &[libc::c_char; 101],
                            >(
                                b"StressMajorizationSmoother StressMajorizationSmoother2_new(SparseMatrix, int, double, double *, int)\0",
                            ))
                                .as_ptr(),
                        );
                        graphviz_exit(1 as libc::c_int);
                    }
                    *jw.offset(nz as isize) = *ja.offset(l as isize);
                    *w.offset(nz as isize) = -(1 as libc::c_int) as libc::c_double / (dist * dist);
                    diag_w += *w.offset(nz as isize);
                    *jd.offset(nz as isize) = *ja.offset(l as isize);
                    *d.offset(nz as isize) = *w.offset(nz as isize) * dist;
                    stop += *d.offset(nz as isize) * distance(x, dim, *ja.offset(l as isize), k);
                    sbot += *d.offset(nz as isize) * dist;
                    diag_d += *d.offset(nz as isize);
                    nz += 1;
                }
                l += 1;
            }
            j += 1;
        }
        *jw.offset(nz as isize) = i;
        *lambda.offset(i as isize) *= -diag_w;
        *w.offset(nz as isize) = -diag_w + *lambda.offset(i as isize);
        *jd.offset(nz as isize) = i;
        *d.offset(nz as isize) = -diag_d;
        nz += 1;
        *iw.offset((i + 1 as libc::c_int) as isize) = nz;
        *id.offset((i + 1 as libc::c_int) as isize) = nz;
        i += 1;
    }
    s = stop / sbot;
    i = 0 as libc::c_int;
    while i < nz {
        *d.offset(i as isize) *= s;
        i += 1;
    }
    (*sm).scaling = s;
    (*(*sm).Lw).nz = nz;
    (*(*sm).Lwd).nz = nz;
    free(mask as *mut libc::c_void);
    free(avg_dist as *mut libc::c_void);
    SparseMatrix_delete(ID);
    return sm;
}
#[no_mangle]
pub unsafe extern "C" fn SparseStressMajorizationSmoother_new(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut lambda0: libc::c_double,
    mut x: *mut libc::c_double,
    mut weighting_scheme: libc::c_int,
    mut scale_initial_coord: libc::c_int,
) -> SparseStressMajorizationSmoother {
    let mut sm: StressMajorizationSmoother = 0 as *mut StressMajorizationSmoother_struct;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut iw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lambda: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut diag_d: libc::c_double = 0.;
    let mut diag_w: libc::c_double = 0.;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut s: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut stop: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sbot: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xdot: libc::c_double = 0 as libc::c_int as libc::c_double;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0
        && (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int
    {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false) && A->type == MATRIX_TYPE_REAL\0"
                as *const u8 as *const libc::c_char,
            b"post_process.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 117],
                &[libc::c_char; 117],
            >(
                b"SparseStressMajorizationSmoother SparseStressMajorizationSmoother_new(SparseMatrix, int, double, double *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < m * dim {
        xdot += *x.offset(i as isize) * *x.offset(i as isize);
        i += 1;
    }
    if xdot == 0 as libc::c_int as libc::c_double {
        i = 0 as libc::c_int;
        while i < m * dim {
            *x.offset(i as isize) = 72 as libc::c_int as libc::c_double * drand();
            i += 1;
        }
    }
    ia = (*A).ia;
    ja = (*A).ja;
    a = (*A).a as *mut libc::c_double;
    sm = gmalloc(::std::mem::size_of::<StressMajorizationSmoother_struct>() as libc::c_ulong)
        as StressMajorizationSmoother;
    (*sm).scaling = 1.0f64;
    let ref mut fresh6 = (*sm).data;
    *fresh6 = 0 as *mut libc::c_void;
    (*sm).scheme = SM_SCHEME_NORMAL as libc::c_int;
    let ref mut fresh7 = (*sm).D;
    *fresh7 = A;
    (*sm).tol_cg = 0.01f64;
    (*sm).maxit_cg = sqrt((*A).m as libc::c_double) as libc::c_int;
    let ref mut fresh8 = (*sm).lambda;
    *fresh8 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(m as libc::c_ulong),
    ) as *mut libc::c_double;
    lambda = *fresh8;
    i = 0 as libc::c_int;
    while i < m {
        *((*sm).lambda).offset(i as isize) = lambda0;
        i += 1;
    }
    nz = (*A).nz;
    let ref mut fresh9 = (*sm).Lw;
    *fresh9 = SparseMatrix_new(
        m,
        m,
        nz + m,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    let ref mut fresh10 = (*sm).Lwd;
    *fresh10 = SparseMatrix_new(
        m,
        m,
        nz + m,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    if ((*sm).Lw).is_null() || ((*sm).Lwd).is_null() {
        StressMajorizationSmoother_delete(sm);
        return 0 as SparseStressMajorizationSmoother;
    }
    iw = (*(*sm).Lw).ia;
    jw = (*(*sm).Lw).ja;
    id = (*(*sm).Lwd).ia;
    jd = (*(*sm).Lwd).ja;
    w = (*(*sm).Lw).a as *mut libc::c_double;
    d = (*(*sm).Lwd).a as *mut libc::c_double;
    let ref mut fresh11 = *id.offset(0 as libc::c_int as isize);
    *fresh11 = 0 as libc::c_int;
    *iw.offset(0 as libc::c_int as isize) = *fresh11;
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        diag_w = 0 as libc::c_int as libc::c_double;
        diag_d = diag_w;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            if k != i {
                *jw.offset(nz as isize) = k;
                dist = *a.offset(j as isize);
                match weighting_scheme {
                    2 => {
                        if dist * dist == 0 as libc::c_int as libc::c_double {
                            *w.offset(nz as isize) = -(100000 as libc::c_int) as libc::c_double;
                        } else {
                            *w.offset(nz as isize) =
                                -(1 as libc::c_int) as libc::c_double / (dist * dist);
                        }
                    }
                    1 => {
                        if dist * dist == 0 as libc::c_int as libc::c_double {
                            *w.offset(nz as isize) = -(100000 as libc::c_int) as libc::c_double;
                        } else {
                            *w.offset(nz as isize) = -(1 as libc::c_int) as libc::c_double / dist;
                        }
                    }
                    0 => {
                        *w.offset(nz as isize) = -(1 as libc::c_int) as libc::c_double;
                    }
                    _ => {
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"post_process.c\0" as *const u8 as *const libc::c_char,
                            374 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 117],
                                &[libc::c_char; 117],
                            >(
                                b"SparseStressMajorizationSmoother SparseStressMajorizationSmoother_new(SparseMatrix, int, double, double *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                        return 0 as SparseStressMajorizationSmoother;
                    }
                }
                diag_w += *w.offset(nz as isize);
                *jd.offset(nz as isize) = k;
                *d.offset(nz as isize) = *w.offset(nz as isize) * dist;
                stop += *d.offset(nz as isize) * distance(x, dim, i, k);
                sbot += *d.offset(nz as isize) * dist;
                diag_d += *d.offset(nz as isize);
                nz += 1;
            }
            j += 1;
        }
        *jw.offset(nz as isize) = i;
        *lambda.offset(i as isize) *= -diag_w;
        *w.offset(nz as isize) = -diag_w + *lambda.offset(i as isize);
        *jd.offset(nz as isize) = i;
        *d.offset(nz as isize) = -diag_d;
        nz += 1;
        *iw.offset((i + 1 as libc::c_int) as isize) = nz;
        *id.offset((i + 1 as libc::c_int) as isize) = nz;
        i += 1;
    }
    if scale_initial_coord != 0 {
        s = stop / sbot;
    } else {
        s = 1.0f64;
    }
    if s == 0 as libc::c_int as libc::c_double {
        return 0 as SparseStressMajorizationSmoother;
    }
    i = 0 as libc::c_int;
    while i < nz {
        *d.offset(i as isize) *= s;
        i += 1;
    }
    (*sm).scaling = s;
    (*(*sm).Lw).nz = nz;
    (*(*sm).Lwd).nz = nz;
    return sm;
}
unsafe extern "C" fn total_distance(
    mut m: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> libc::c_double {
    let mut total: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dist: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < m {
        dist = 0.0f64;
        j = 0 as libc::c_int;
        while j < dim {
            dist += (*y.offset((i * dim + j) as isize) - *x.offset((i * dim + j) as isize))
                * (*y.offset((i * dim + j) as isize) - *x.offset((i * dim + j) as isize));
            j += 1;
        }
        total += sqrt(dist);
        i += 1;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn SparseStressMajorizationSmoother_delete(
    mut sm: SparseStressMajorizationSmoother,
) {
    StressMajorizationSmoother_delete(sm);
}
#[no_mangle]
pub unsafe extern "C" fn SparseStressMajorizationSmoother_smooth(
    mut sm: SparseStressMajorizationSmoother,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut maxit_sm: libc::c_int,
    mut tol: libc::c_double,
) -> libc::c_double {
    return StressMajorizationSmoother_smooth(sm, dim, x, maxit_sm, tol);
}
unsafe extern "C" fn get_edge_label_matrix(
    mut data: relative_position_constraints,
    mut m: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut LL: *mut SparseMatrix,
    mut rhs: *mut *mut libc::c_double,
) {
    let mut edge_labeling_scheme: libc::c_int = (*data).edge_labeling_scheme;
    let mut n_constr_nodes: libc::c_int = (*data).n_constr_nodes;
    let mut constr_nodes: *mut libc::c_int = (*data).constr_nodes;
    let mut A_constr: SparseMatrix = (*data).A_constr;
    let mut ia: *mut libc::c_int = (*A_constr).ia;
    let mut ja: *mut libc::c_int = (*A_constr).ja;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut irn: *mut libc::c_int = (*data).irn;
    let mut jcn: *mut libc::c_int = (*data).jcn;
    let mut val: *mut libc::c_double = (*data).val;
    let mut dist: libc::c_double = 0.;
    let mut kk: libc::c_double = 0.;
    let mut k: libc::c_double = 0.;
    let mut x00: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut Lc: SparseMatrix = 0 as SparseMatrix;
    let mut constr_penalty: libc::c_double = (*data).constr_penalty;
    if edge_labeling_scheme == ELSCHEME_PENALTY as libc::c_int
        || edge_labeling_scheme == ELSCHEME_STRAIGHTLINE_PENALTY as libc::c_int
    {
        if irn.is_null() {
            if jcn.is_null() && val.is_null() {
            } else {
                __assert_fail(
                    b"(!jcn) && (!val)\0" as *const u8 as *const libc::c_char,
                    b"post_process.c\0" as *const u8 as *const libc::c_char,
                    475 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 105],
                        &[libc::c_char; 105],
                    >(
                        b"void get_edge_label_matrix(relative_position_constraints, int, int, double *, SparseMatrix *, double **)\0",
                    ))
                        .as_ptr(),
                );
            }
            nz = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < n_constr_nodes {
                ii = *constr_nodes.offset(i as isize);
                k = (*ia.offset((ii + 1 as libc::c_int) as isize) - *ia.offset(ii as isize))
                    as libc::c_double;
                nz += ((k + 1 as libc::c_int as libc::c_double)
                    * (k + 1 as libc::c_int as libc::c_double))
                    as libc::c_int;
                i += 1;
            }
            let ref mut fresh12 = (*data).irn;
            *fresh12 = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(nz as libc::c_ulong),
            ) as *mut libc::c_int;
            irn = *fresh12;
            let ref mut fresh13 = (*data).jcn;
            *fresh13 = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(nz as libc::c_ulong),
            ) as *mut libc::c_int;
            jcn = *fresh13;
            let ref mut fresh14 = (*data).val;
            *fresh14 = gmalloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(nz as libc::c_ulong),
            ) as *mut libc::c_double;
            val = *fresh14;
        }
        nz = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < n_constr_nodes {
            ii = *constr_nodes.offset(i as isize);
            jj = *ja.offset(*ia.offset(ii as isize) as isize);
            ll = *ja.offset((*ia.offset(ii as isize) + 1 as libc::c_int) as isize);
            if !(jj == ll) {
                dist = distance_cropped(x, dim, jj, ll);
                dist *= dist;
                k = (*ia.offset((ii + 1 as libc::c_int) as isize) - *ia.offset(ii as isize))
                    as libc::c_double;
                kk = k * k;
                *irn.offset(nz as isize) = ii;
                *jcn.offset(nz as isize) = ii;
                let fresh15 = nz;
                nz = nz + 1;
                *val.offset(fresh15 as isize) = constr_penalty / dist;
                k = constr_penalty / (k * dist);
                kk = constr_penalty / (kk * dist);
                j = *ia.offset(ii as isize);
                while j < *ia.offset((ii + 1 as libc::c_int) as isize) {
                    *irn.offset(nz as isize) = ii;
                    *jcn.offset(nz as isize) = *ja.offset(j as isize);
                    let fresh16 = nz;
                    nz = nz + 1;
                    *val.offset(fresh16 as isize) = -k;
                    j += 1;
                }
                j = *ia.offset(ii as isize);
                while j < *ia.offset((ii + 1 as libc::c_int) as isize) {
                    jj = *ja.offset(j as isize);
                    *irn.offset(nz as isize) = jj;
                    *jcn.offset(nz as isize) = ii;
                    let fresh17 = nz;
                    nz = nz + 1;
                    *val.offset(fresh17 as isize) = -k;
                    l = *ia.offset(ii as isize);
                    while l < *ia.offset((ii + 1 as libc::c_int) as isize) {
                        ll = *ja.offset(l as isize);
                        *irn.offset(nz as isize) = jj;
                        *jcn.offset(nz as isize) = ll;
                        let fresh18 = nz;
                        nz = nz + 1;
                        *val.offset(fresh18 as isize) = kk;
                        l += 1;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        Lc = SparseMatrix_from_coordinate_arrays(
            nz,
            m,
            m,
            irn,
            jcn,
            val as *mut libc::c_void,
            MATRIX_TYPE_REAL as libc::c_int,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        );
    } else if edge_labeling_scheme == ELSCHEME_PENALTY2 as libc::c_int
        || edge_labeling_scheme == ELSCHEME_STRAIGHTLINE_PENALTY2 as libc::c_int
    {
        if irn.is_null() {
            if jcn.is_null() && val.is_null() {
            } else {
                __assert_fail(
                    b"(!jcn) && (!val)\0" as *const u8 as *const libc::c_char,
                    b"post_process.c\0" as *const u8 as *const libc::c_char,
                    517 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 105],
                        &[libc::c_char; 105],
                    >(
                        b"void get_edge_label_matrix(relative_position_constraints, int, int, double *, SparseMatrix *, double **)\0",
                    ))
                        .as_ptr(),
                );
            }
            nz = n_constr_nodes;
            let ref mut fresh19 = (*data).irn;
            *fresh19 = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(nz as libc::c_ulong),
            ) as *mut libc::c_int;
            irn = *fresh19;
            let ref mut fresh20 = (*data).jcn;
            *fresh20 = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(nz as libc::c_ulong),
            ) as *mut libc::c_int;
            jcn = *fresh20;
            let ref mut fresh21 = (*data).val;
            *fresh21 = gmalloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(nz as libc::c_ulong),
            ) as *mut libc::c_double;
            val = *fresh21;
        }
        x00 = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(m as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        i = 0 as libc::c_int;
        while i < m * dim {
            *x00.offset(i as isize) = 0 as libc::c_int as libc::c_double;
            i += 1;
        }
        nz = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < n_constr_nodes {
            ii = *constr_nodes.offset(i as isize);
            jj = *ja.offset(*ia.offset(ii as isize) as isize);
            ll = *ja.offset((*ia.offset(ii as isize) + 1 as libc::c_int) as isize);
            dist = distance_cropped(x, dim, jj, ll);
            *irn.offset(nz as isize) = ii;
            *jcn.offset(nz as isize) = ii;
            let fresh22 = nz;
            nz = nz + 1;
            *val.offset(fresh22 as isize) = constr_penalty / dist;
            j = *ia.offset(ii as isize);
            while j < *ia.offset((ii + 1 as libc::c_int) as isize) {
                jj = *ja.offset(j as isize);
                l = 0 as libc::c_int;
                while l < dim {
                    *x00.offset((ii * dim + l) as isize) += *x.offset((jj * dim + l) as isize);
                    l += 1;
                }
                j += 1;
            }
            l = 0 as libc::c_int;
            while l < dim {
                *x00.offset((ii * dim + l) as isize) *= constr_penalty
                    / dist
                    / (*ia.offset((ii + 1 as libc::c_int) as isize) - *ia.offset(ii as isize))
                        as libc::c_double;
                l += 1;
            }
            i += 1;
        }
        Lc = SparseMatrix_from_coordinate_arrays(
            nz,
            m,
            m,
            irn,
            jcn,
            val as *mut libc::c_void,
            MATRIX_TYPE_REAL as libc::c_int,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        );
    }
    *LL = Lc;
    *rhs = x00;
}
#[no_mangle]
pub unsafe extern "C" fn get_stress(
    mut m: libc::c_int,
    mut dim: libc::c_int,
    mut iw: *mut libc::c_int,
    mut jw: *mut libc::c_int,
    mut w: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut x: *mut libc::c_double,
    mut scaling: libc::c_double,
    mut data: *mut libc::c_void,
    mut weighted: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut res: libc::c_double = 0.0f64;
    let mut dist: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < m {
        j = *iw.offset(i as isize);
        while j < *iw.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *jw.offset(j as isize)) {
                dist = *d.offset(j as isize) / *w.offset(j as isize);
                if weighted != 0 {
                    res += -*w.offset(j as isize)
                        * (dist - distance(x, dim, i, *jw.offset(j as isize)))
                        * (dist - distance(x, dim, i, *jw.offset(j as isize)));
                } else {
                    res += (dist - distance(x, dim, i, *jw.offset(j as isize)))
                        * (dist - distance(x, dim, i, *jw.offset(j as isize)));
                }
            }
            j += 1;
        }
        i += 1;
    }
    return 0.5f64 * res / scaling / scaling;
}
unsafe extern "C" fn uniform_stress_augment_rhs(
    mut m: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut alpha: libc::c_double,
    mut M: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dist: libc::c_double = 0.;
    let mut distij: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < m {
        j = i + 1 as libc::c_int;
        while j < m {
            dist = distance_cropped(x, dim, i, j);
            k = 0 as libc::c_int;
            while k < dim {
                distij =
                    (*x.offset((i * dim + k) as isize) - *x.offset((j * dim + k) as isize)) / dist;
                *y.offset((i * dim + k) as isize) += alpha * M * distij;
                *y.offset((j * dim + k) as isize) += alpha * M * -distij;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn uniform_stress_solve(
    mut Lw: SparseMatrix,
    mut alpha: libc::c_double,
    mut dim: libc::c_int,
    mut x0: *mut libc::c_double,
    mut rhs: *mut libc::c_double,
    mut tol: libc::c_double,
    mut maxit: libc::c_int,
) -> libc::c_double {
    let mut Ax: Operator = 0 as *mut Operator_struct;
    let mut Precon: Operator = 0 as *mut Operator_struct;
    Ax = Operator_uniform_stress_matmul(Lw, alpha);
    Precon = Operator_uniform_stress_diag_precon_new(Lw, alpha);
    return cg(Ax, Precon, (*Lw).m, dim, x0, rhs, tol, maxit);
}
#[no_mangle]
pub unsafe extern "C" fn StressMajorizationSmoother_smooth(
    mut sm: StressMajorizationSmoother,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut maxit_sm: libc::c_int,
    mut tol: libc::c_double,
) -> libc::c_double {
    let mut Lw: SparseMatrix = (*sm).Lw;
    let mut Lwd: SparseMatrix = (*sm).Lwd;
    let mut Lwdd: SparseMatrix = 0 as SparseMatrix;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut iw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut idiag: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dd: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x0: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x00: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut diag: libc::c_double = 0.;
    let mut diff: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut lambda: *mut libc::c_double = (*sm).lambda;
    let mut alpha: libc::c_double = 0.0f64;
    let mut M: libc::c_double = 0.0f64;
    let mut Lc: SparseMatrix = 0 as SparseMatrix;
    let mut dij: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    Lwdd = SparseMatrix_copy(Lwd);
    m = (*Lw).m;
    x0 = calloc(
        (dim * m) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if !x0.is_null() {
        memcpy(
            x0 as *mut libc::c_void,
            x as *const libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(m as libc::c_ulong),
        );
        y = calloc(
            (dim * m) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        if !y.is_null() {
            id = (*Lwd).ia;
            jd = (*Lwd).ja;
            d = (*Lwd).a as *mut libc::c_double;
            dd = (*Lwdd).a as *mut libc::c_double;
            w = (*Lw).a as *mut libc::c_double;
            iw = (*Lw).ia;
            jw = (*Lw).ja;
            if (*sm).scheme == SM_SCHEME_NORMAL_ELABEL as libc::c_int {
                get_edge_label_matrix(
                    (*sm).data as relative_position_constraints,
                    m,
                    dim,
                    x,
                    &mut Lc,
                    &mut x00,
                );
                if !Lc.is_null() {
                    Lw = SparseMatrix_add(Lw, Lc);
                }
            } else if (*sm).scheme == SM_SCHEME_UNIFORM_STRESS as libc::c_int {
                alpha = *((*sm).data as *mut libc::c_double).offset(0 as libc::c_int as isize);
                M = *((*sm).data as *mut libc::c_double).offset(1 as libc::c_int as isize);
            }
            loop {
                let fresh23 = iter;
                iter = iter + 1;
                if !(fresh23 < maxit_sm && diff > tol) {
                    break;
                }
                if (*sm).scheme != SM_SCHEME_STRESS_APPROX as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < m {
                        idiag = -(1 as libc::c_int);
                        diag = 0.0f64;
                        j = *id.offset(i as isize);
                        while j < *id.offset((i + 1 as libc::c_int) as isize) {
                            if i == *jd.offset(j as isize) {
                                idiag = j;
                            } else {
                                dist = distance(x, dim, i, *jd.offset(j as isize));
                                if *d.offset(j as isize) == 0 as libc::c_int as libc::c_double {
                                    *dd.offset(j as isize) = 0 as libc::c_int as libc::c_double;
                                } else {
                                    if dist == 0 as libc::c_int as libc::c_double {
                                        dij = *d.offset(j as isize) / *w.offset(j as isize);
                                        k = 0 as libc::c_int;
                                        while k < dim {
                                            *x.offset(
                                                (*jd.offset(j as isize) * dim + k) as isize,
                                            ) += 0.0001f64 * (drand() + 0.0001f64) * dij;
                                            k += 1;
                                        }
                                        dist = distance(x, dim, i, *jd.offset(j as isize));
                                    }
                                    *dd.offset(j as isize) = *d.offset(j as isize) / dist;
                                }
                                diag += *dd.offset(j as isize);
                            }
                            j += 1;
                        }
                        if idiag >= 0 as libc::c_int {
                        } else {
                            __assert_fail(
                                b"idiag >= 0\0" as *const u8 as *const libc::c_char,
                                b"post_process.c\0" as *const u8 as *const libc::c_char,
                                660 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 97],
                                    &[libc::c_char; 97],
                                >(
                                    b"double StressMajorizationSmoother_smooth(StressMajorizationSmoother, int, double *, int, double)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        *dd.offset(idiag as isize) = -diag;
                        i += 1;
                    }
                    SparseMatrix_multiply_dense(Lwdd, x, &mut y, dim);
                } else {
                    i = 0 as libc::c_int;
                    while i < m {
                        j = 0 as libc::c_int;
                        while j < dim {
                            *y.offset((i * dim + j) as isize) = 0 as libc::c_int as libc::c_double;
                            j += 1;
                        }
                        i += 1;
                    }
                }
                if !lambda.is_null() {
                    i = 0 as libc::c_int;
                    while i < m {
                        j = 0 as libc::c_int;
                        while j < dim {
                            *y.offset((i * dim + j) as isize) +=
                                *lambda.offset(i as isize) * *x0.offset((i * dim + j) as isize);
                            j += 1;
                        }
                        i += 1;
                    }
                }
                match (*sm).scheme {
                    1 => {
                        i = 0 as libc::c_int;
                        while i < m {
                            j = 0 as libc::c_int;
                            while j < dim {
                                *y.offset((i * dim + j) as isize) +=
                                    *x00.offset((i * dim + j) as isize);
                                j += 1;
                            }
                            i += 1;
                        }
                    }
                    2 => {
                        uniform_stress_augment_rhs(m, dim, x, y, alpha, M);
                    }
                    _ => {}
                }
                if (*sm).scheme == SM_SCHEME_UNIFORM_STRESS as libc::c_int {
                    uniform_stress_solve(Lw, alpha, dim, x, y, (*sm).tol_cg, (*sm).maxit_cg);
                } else {
                    SparseMatrix_solve(
                        Lw,
                        dim,
                        x,
                        y,
                        (*sm).tol_cg,
                        (*sm).maxit_cg,
                        SOLVE_METHOD_CG as libc::c_int,
                        &mut flag,
                    );
                }
                if flag != 0 {
                    break;
                }
                diff = total_distance(m, dim, x, y) / sqrt(vector_product(m * dim, x, x));
                memcpy(
                    x as *mut libc::c_void,
                    y as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                        .wrapping_mul(m as libc::c_ulong)
                        .wrapping_mul(dim as libc::c_ulong),
                );
            }
        }
    }
    SparseMatrix_delete(Lwdd);
    if !Lc.is_null() {
        SparseMatrix_delete(Lc);
        SparseMatrix_delete(Lw);
    }
    free(x0 as *mut libc::c_void);
    free(y as *mut libc::c_void);
    free(x00 as *mut libc::c_void);
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn StressMajorizationSmoother_delete(mut sm: StressMajorizationSmoother) {
    if sm.is_null() {
        return;
    }
    if !((*sm).Lw).is_null() {
        SparseMatrix_delete((*sm).Lw);
    }
    if !((*sm).Lwd).is_null() {
        SparseMatrix_delete((*sm).Lwd);
    }
    free((*sm).lambda as *mut libc::c_void);
    if !((*sm).data).is_null() {
        ((*sm).data_deallocator).expect("non-null function pointer")((*sm).data);
    }
    free(sm as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn TriangleSmoother_new(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut lambda0: libc::c_double,
    mut x: *mut libc::c_double,
    mut use_triangularization: libc::c_int,
) -> TriangleSmoother {
    let mut sm: TriangleSmoother = 0 as *mut StressMajorizationSmoother_struct;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut iw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jdiag: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut avg_dist: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lambda: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut diag_d: libc::c_double = 0.;
    let mut diag_w: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut s: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut stop: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sbot: libc::c_double = 0 as libc::c_int as libc::c_double;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"post_process.c\0" as *const u8 as *const libc::c_char,
            767 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"TriangleSmoother TriangleSmoother_new(SparseMatrix, int, double, double *, int)\0",
            ))
                .as_ptr(),
        );
    }
    avg_dist = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        *avg_dist.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        nz = 0 as libc::c_int;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *ja.offset(j as isize)) {
                *avg_dist.offset(i as isize) += distance(x, dim, i, *ja.offset(j as isize));
                nz += 1;
            }
            j += 1;
        }
        if nz > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"nz > 0\0" as *const u8 as *const libc::c_char,
                b"post_process.c\0" as *const u8 as *const libc::c_char,
                779 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"TriangleSmoother TriangleSmoother_new(SparseMatrix, int, double, double *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        *avg_dist.offset(i as isize) /= nz as libc::c_double;
        i += 1;
    }
    sm = gcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<StressMajorizationSmoother_struct>() as libc::c_ulong,
    ) as *mut StressMajorizationSmoother_struct;
    (*sm).scaling = 1 as libc::c_int as libc::c_double;
    let ref mut fresh24 = (*sm).data;
    *fresh24 = 0 as *mut libc::c_void;
    (*sm).scheme = SM_SCHEME_NORMAL as libc::c_int;
    (*sm).tol_cg = 0.01f64;
    (*sm).maxit_cg = sqrt((*A).m as libc::c_double) as libc::c_int;
    let ref mut fresh25 = (*sm).lambda;
    *fresh25 = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    lambda = *fresh25;
    i = 0 as libc::c_int;
    while i < m {
        *((*sm).lambda).offset(i as isize) = lambda0;
        i += 1;
    }
    if m > 2 as libc::c_int {
        if use_triangularization != 0 {
            B = call_tri(m, dim, x);
        } else {
            B = call_tri2(m, dim, x);
        }
    } else {
        B = SparseMatrix_copy(A);
    }
    let ref mut fresh26 = (*sm).Lw;
    *fresh26 = SparseMatrix_add(A, B);
    SparseMatrix_delete(B);
    let ref mut fresh27 = (*sm).Lwd;
    *fresh27 = SparseMatrix_copy((*sm).Lw);
    if ((*sm).Lw).is_null() || ((*sm).Lwd).is_null() {
        TriangleSmoother_delete(sm);
        return 0 as TriangleSmoother;
    }
    iw = (*(*sm).Lw).ia;
    jw = (*(*sm).Lw).ja;
    w = (*(*sm).Lw).a as *mut libc::c_double;
    d = (*(*sm).Lwd).a as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        diag_w = 0 as libc::c_int as libc::c_double;
        diag_d = diag_w;
        jdiag = -(1 as libc::c_int);
        j = *iw.offset(i as isize);
        while j < *iw.offset((i + 1 as libc::c_int) as isize) {
            k = *jw.offset(j as isize);
            if k == i {
                jdiag = j;
            } else {
                dist = pow(distance_cropped(x, dim, i, k), 0.6f64);
                *w.offset(j as isize) = 1 as libc::c_int as libc::c_double / (dist * dist);
                diag_w += *w.offset(j as isize);
                *d.offset(j as isize) = *w.offset(j as isize) * dist;
                stop += *d.offset(j as isize) * distance(x, dim, i, k);
                sbot += *d.offset(j as isize) * dist;
                diag_d += *d.offset(j as isize);
            }
            j += 1;
        }
        *lambda.offset(i as isize) *= -diag_w;
        if jdiag >= 0 as libc::c_int {
        } else {
            __assert_fail(
                b"jdiag >= 0\0" as *const u8 as *const libc::c_char,
                b"post_process.c\0" as *const u8 as *const libc::c_char,
                846 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"TriangleSmoother TriangleSmoother_new(SparseMatrix, int, double, double *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        *w.offset(jdiag as isize) = -diag_w + *lambda.offset(i as isize);
        *d.offset(jdiag as isize) = -diag_d;
        i += 1;
    }
    s = stop / sbot;
    i = 0 as libc::c_int;
    while i < *iw.offset(m as isize) {
        *d.offset(i as isize) *= s;
        i += 1;
    }
    (*sm).scaling = s;
    free(avg_dist as *mut libc::c_void);
    return sm;
}
#[no_mangle]
pub unsafe extern "C" fn TriangleSmoother_delete(mut sm: TriangleSmoother) {
    StressMajorizationSmoother_delete(sm);
}
#[no_mangle]
pub unsafe extern "C" fn TriangleSmoother_smooth(
    mut sm: TriangleSmoother,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) {
    StressMajorizationSmoother_smooth(sm, dim, x, 50 as libc::c_int, 0.001f64);
}
#[no_mangle]
pub unsafe extern "C" fn SpringSmoother_new(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut ctrl: spring_electrical_control,
    mut x: *mut libc::c_double,
) -> SpringSmoother {
    let mut sm: SpringSmoother = 0 as *mut SpringSmoother_struct;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dd: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut avg_dist: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ID: SparseMatrix = 0 as SparseMatrix;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"post_process.c\0" as *const u8 as *const libc::c_char,
            883 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"SpringSmoother SpringSmoother_new(SparseMatrix, int, spring_electrical_control, double *)\0",
            ))
                .as_ptr(),
        );
    }
    ID = ideal_distance_matrix(A, dim, x);
    dd = (*ID).a as *mut libc::c_double;
    sm = gcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<SpringSmoother_struct>() as libc::c_ulong,
    ) as *mut SpringSmoother_struct;
    mask = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    avg_dist = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        *avg_dist.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        nz = 0 as libc::c_int;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *ja.offset(j as isize)) {
                *avg_dist.offset(i as isize) += distance(x, dim, i, *ja.offset(j as isize));
                nz += 1;
            }
            j += 1;
        }
        if nz > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"nz > 0\0" as *const u8 as *const libc::c_char,
                b"post_process.c\0" as *const u8 as *const libc::c_char,
                901 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"SpringSmoother SpringSmoother_new(SparseMatrix, int, spring_electrical_control, double *)\0",
                ))
                    .as_ptr(),
            );
        }
        *avg_dist.offset(i as isize) /= nz as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *mask.offset(i as isize) = i;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            if *mask.offset(k as isize) != i {
                *mask.offset(k as isize) = i;
                nz += 1;
            }
            j += 1;
        }
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            l = *ia.offset(k as isize);
            while l < *ia.offset((k + 1 as libc::c_int) as isize) {
                if *mask.offset(*ja.offset(l as isize) as isize) != i {
                    *mask.offset(*ja.offset(l as isize) as isize) = i;
                    nz += 1;
                }
                l += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let ref mut fresh28 = (*sm).D;
    *fresh28 = SparseMatrix_new(
        m,
        m,
        nz,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    if ((*sm).D).is_null() {
        SpringSmoother_delete(sm);
        return 0 as SpringSmoother;
    }
    id = (*(*sm).D).ia;
    jd = (*(*sm).D).ja;
    d = (*(*sm).D).a as *mut libc::c_double;
    *id.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *mask.offset(i as isize) = i + m;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            if *mask.offset(k as isize) != i + m {
                *mask.offset(k as isize) = i + m;
                *jd.offset(nz as isize) = k;
                *d.offset(nz as isize) =
                    (*avg_dist.offset(i as isize) + *avg_dist.offset(k as isize)) * 0.5f64;
                *d.offset(nz as isize) = *dd.offset(j as isize);
                nz += 1;
            }
            j += 1;
        }
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = *ja.offset(j as isize);
            l = *ia.offset(k as isize);
            while l < *ia.offset((k + 1 as libc::c_int) as isize) {
                if *mask.offset(*ja.offset(l as isize) as isize) != i + m {
                    *mask.offset(*ja.offset(l as isize) as isize) = i + m;
                    *jd.offset(nz as isize) = *ja.offset(l as isize);
                    *d.offset(nz as isize) = (*avg_dist.offset(i as isize)
                        + 2 as libc::c_int as libc::c_double * *avg_dist.offset(k as isize)
                        + *avg_dist.offset(*ja.offset(l as isize) as isize))
                        * 0.5f64;
                    *d.offset(nz as isize) = *dd.offset(j as isize) + *dd.offset(l as isize);
                    nz += 1;
                }
                l += 1;
            }
            j += 1;
        }
        *id.offset((i + 1 as libc::c_int) as isize) = nz;
        i += 1;
    }
    (*(*sm).D).nz = nz;
    let ref mut fresh29 = (*sm).ctrl;
    *fresh29 = spring_electrical_control_new();
    *(*sm).ctrl = *ctrl;
    (*(*sm).ctrl).random_start = 0 as libc::c_int;
    (*(*sm).ctrl).multilevels = 1 as libc::c_int;
    (*(*sm).ctrl).step /= 2 as libc::c_int as libc::c_double;
    (*(*sm).ctrl).maxiter = 20 as libc::c_int;
    free(mask as *mut libc::c_void);
    free(avg_dist as *mut libc::c_void);
    SparseMatrix_delete(ID);
    return sm;
}
#[no_mangle]
pub unsafe extern "C" fn SpringSmoother_delete(mut sm: SpringSmoother) {
    if sm.is_null() {
        return;
    }
    if !((*sm).D).is_null() {
        SparseMatrix_delete((*sm).D);
    }
    if !((*sm).ctrl).is_null() {
        spring_electrical_control_delete((*sm).ctrl);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SpringSmoother_smooth(
    mut sm: SpringSmoother,
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) {
    let mut flag: libc::c_int = 0 as libc::c_int;
    spring_electrical_spring_embedding(dim, A, (*sm).D, (*sm).ctrl, x, &mut flag);
    if flag == 0 {
    } else {
        __assert_fail(
            b"!flag\0" as *const u8 as *const libc::c_char,
            b"post_process.c\0" as *const u8 as *const libc::c_char,
            996 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 72], &[libc::c_char; 72]>(
                b"void SpringSmoother_smooth(SpringSmoother, SparseMatrix, int, double *)\0",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn post_process_smoothing(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut x: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    *flag = 0 as libc::c_int;
    match (*ctrl).smoothing {
        6 | 5 => {
            let mut sm: TriangleSmoother = 0 as *mut StressMajorizationSmoother_struct;
            if (*A).m > 2 as libc::c_int {
                if (*ctrl).smoothing == SMOOTHING_RNG as libc::c_int {
                    sm = TriangleSmoother_new(
                        A,
                        dim,
                        0 as libc::c_int as libc::c_double,
                        x,
                        0 as libc::c_int,
                    );
                } else {
                    sm = TriangleSmoother_new(
                        A,
                        dim,
                        0 as libc::c_int as libc::c_double,
                        x,
                        (0 as libc::c_int == 0) as libc::c_int,
                    );
                }
                TriangleSmoother_smooth(sm, dim, x);
                TriangleSmoother_delete(sm);
            }
        }
        1 | 3 | 2 => {
            let mut sm_0: StressMajorizationSmoother = 0 as *mut StressMajorizationSmoother_struct;
            let mut k: libc::c_int = 0;
            let mut dist_scheme: libc::c_int = IDEAL_AVG_DIST as libc::c_int;
            if (*ctrl).smoothing == SMOOTHING_STRESS_MAJORIZATION_GRAPH_DIST as libc::c_int {
                dist_scheme = IDEAL_GRAPH_DIST as libc::c_int;
            } else if (*ctrl).smoothing == SMOOTHING_STRESS_MAJORIZATION_AVG_DIST as libc::c_int {
                dist_scheme = IDEAL_AVG_DIST as libc::c_int;
            } else if (*ctrl).smoothing == SMOOTHING_STRESS_MAJORIZATION_POWER_DIST as libc::c_int {
                dist_scheme = IDEAL_POWER_DIST as libc::c_int;
            }
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                sm_0 = StressMajorizationSmoother2_new(A, dim, 0.05f64, x, dist_scheme);
                StressMajorizationSmoother_smooth(sm_0, dim, x, 50 as libc::c_int, 0.001f64);
                StressMajorizationSmoother_delete(sm_0);
                k += 1;
            }
        }
        4 => {
            let mut sm_1: SpringSmoother = 0 as *mut SpringSmoother_struct;
            let mut k_0: libc::c_int = 0;
            k_0 = 0 as libc::c_int;
            while k_0 < 1 as libc::c_int {
                sm_1 = SpringSmoother_new(A, dim, ctrl, x);
                SpringSmoother_smooth(sm_1, A, dim, x);
                SpringSmoother_delete(sm_1);
                k_0 += 1;
            }
        }
        _ => {}
    };
}
