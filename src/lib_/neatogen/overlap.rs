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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn distance(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn SparseMatrix_from_coordinate_format(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_add(A: SparseMatrix, B: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_coordinate_form_add_entry(
        A: SparseMatrix,
        irn: libc::c_int,
        jcn: libc::c_int,
        val: *mut libc::c_void,
    ) -> SparseMatrix;
    fn SparseMatrix_is_symmetric(A: SparseMatrix, test_pattern_symmetry_only: bool) -> libc::c_int;
    fn SparseMatrix_symmetrize(A: SparseMatrix, pattern_symmetric_only: bool) -> SparseMatrix;
    fn SparseMatrix_copy(A: SparseMatrix) -> SparseMatrix;
    fn average_edge_length(
        A: SparseMatrix,
        dim: libc::c_int,
        coord: *mut libc::c_double,
    ) -> libc::c_double;
    fn StressMajorizationSmoother_delete(sm: StressMajorizationSmoother);
    fn StressMajorizationSmoother_smooth(
        sm: StressMajorizationSmoother,
        dim: libc::c_int,
        x: *mut libc::c_double,
        maxit: libc::c_int,
        tol: libc::c_double,
    ) -> libc::c_double;
    fn call_tri(n: libc::c_int, dim: libc::c_int, x: *mut libc::c_double) -> SparseMatrix;
    fn RBTreeCreate(
        CompFunc: Option<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
        DestFunc: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        InfoDestFunc: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        PrintFunc: Option<unsafe extern "C" fn(*const libc::c_void) -> ()>,
        PrintInfo: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> *mut rb_red_blk_tree;
    fn RBTreeInsert(
        _: *mut rb_red_blk_tree,
        key: *mut libc::c_void,
        info: *mut libc::c_void,
    ) -> *mut rb_red_blk_node;
    fn RBDelete(_: *mut rb_red_blk_tree, _: *mut rb_red_blk_node);
    fn RBTreeDestroy(_: *mut rb_red_blk_tree);
    fn TreePredecessor(_: *mut rb_red_blk_tree, _: *mut rb_red_blk_node) -> *mut rb_red_blk_node;
    fn RBExactQuery(_: *mut rb_red_blk_tree, _: *mut libc::c_void) -> *mut rb_red_blk_node;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
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
pub type OverlapSmoother = StressMajorizationSmoother;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_red_blk_tree {
    pub Compare:
        Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    pub DestroyKey: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub DestroyInfo: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub PrintKey: Option<unsafe extern "C" fn(*const libc::c_void) -> ()>,
    pub PrintInfo: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub root: *mut rb_red_blk_node,
    pub nil: *mut rb_red_blk_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rb_red_blk_node {
    pub key: *mut libc::c_void,
    pub info: *mut libc::c_void,
    pub red: libc::c_int,
    pub left: *mut rb_red_blk_node,
    pub right: *mut rb_red_blk_node,
    pub parent: *mut rb_red_blk_node,
}
pub type scan_point = scan_point_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scan_point_struct {
    pub node: libc::c_int,
    pub x: libc::c_double,
    pub status: libc::c_int,
}
pub const INTV_OPEN: C2RustUnnamed_3 = 0;
pub const INTV_CLOSE: C2RustUnnamed_3 = 1;
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
pub const ELSCHEME_NONE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ELSCHEME_STRAIGHTLINE_PENALTY2: C2RustUnnamed_2 = 4;
pub const ELSCHEME_STRAIGHTLINE_PENALTY: C2RustUnnamed_2 = 3;
pub const ELSCHEME_PENALTY2: C2RustUnnamed_2 = 2;
pub const ELSCHEME_PENALTY: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
unsafe extern "C" fn ideal_distance_avoid_overlap(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut ideal_distance: *mut libc::c_double,
    mut tmax: *mut libc::c_double,
    mut tmin: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut dist: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut wx: libc::c_double = 0.;
    let mut wy: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut expandmax: libc::c_double = 1.5f64;
    let mut expandmin: libc::c_double = 1 as libc::c_int as libc::c_double;
    *tmax = 0 as libc::c_int as libc::c_double;
    *tmin = 1.0e10f64;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"overlap.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 103],
                &[libc::c_char; 103],
            >(
                b"void ideal_distance_avoid_overlap(int, SparseMatrix, double *, double *, double *, double *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < (*A).m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            if !(jj == i) {
                dist = distance(x, dim, i, jj);
                dx = fabs(*x.offset((i * dim) as isize) - *x.offset((jj * dim) as isize));
                dy = fabs(
                    *x.offset((i * dim + 1 as libc::c_int) as isize)
                        - *x.offset((jj * dim + 1 as libc::c_int) as isize),
                );
                wx = *width.offset((i * dim) as isize) + *width.offset((jj * dim) as isize);
                wy = *width.offset((i * dim + 1 as libc::c_int) as isize)
                    + *width.offset((jj * dim + 1 as libc::c_int) as isize);
                if dx < 1.0e-16f64 * wx && dy < 1.0e-16f64 * wy {
                    *ideal_distance.offset(j as isize) = hypot(wx, wy);
                    *tmax = 2 as libc::c_int as libc::c_double;
                } else {
                    if dx < 1.0e-16f64 * wx {
                        t = wy / dy;
                    } else if dy < 1.0e-16f64 * wy {
                        t = wx / dx;
                    } else {
                        t = if wx / dx < wy / dy { wx / dx } else { wy / dy };
                    }
                    if t > 1 as libc::c_int as libc::c_double {
                        t = if t > 1.001f64 { t } else { 1.001f64 };
                    }
                    *tmax = if *tmax > t { *tmax } else { t };
                    *tmin = if *tmin < t { *tmin } else { t };
                    t = if expandmax < t { expandmax } else { t };
                    t = if expandmin > t { expandmin } else { t };
                    if t > 1 as libc::c_int as libc::c_double {
                        *ideal_distance.offset(j as isize) = t * dist;
                    } else {
                        *ideal_distance.offset(j as isize) = -t * dist;
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn comp_scan_points(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut pp: *const scan_point = p as *const scan_point;
    let mut qq: *const scan_point = q as *const scan_point;
    if (*pp).x > (*qq).x {
        return 1 as libc::c_int;
    } else if (*pp).x < (*qq).x {
        return -(1 as libc::c_int);
    } else {
        if (*pp).node > (*qq).node {
            return 1 as libc::c_int;
        } else {
            if (*pp).node < (*qq).node {
                return -(1 as libc::c_int);
            }
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn NodeDest(mut a: *mut libc::c_void) {}
unsafe extern "C" fn NodeComp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return comp_scan_points(a, b);
}
unsafe extern "C" fn NodePrint(mut a: *const libc::c_void) {
    let mut aa: *const scan_point = a as *const scan_point;
    fprintf(
        stderr,
        b"node {%d, %f, %d}\n\0" as *const u8 as *const libc::c_char,
        (*aa).node,
        (*aa).x,
        (*aa).status,
    );
}
unsafe extern "C" fn InfoPrint(mut a: *mut libc::c_void) {}
unsafe extern "C" fn InfoDest(mut a: *mut libc::c_void) {}
unsafe extern "C" fn get_overlap_graph(
    mut dim: libc::c_int,
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut check_overlap_only: libc::c_int,
) -> SparseMatrix {
    let mut scanpointsx: *mut scan_point = 0 as *mut scan_point;
    let mut scanpointsy: *mut scan_point = 0 as *mut scan_point;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut B: SparseMatrix = 0 as SparseMatrix;
    let mut newNode: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut newNode0: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut newNode2: *mut rb_red_blk_node = 0 as *mut rb_red_blk_node;
    let mut treey: *mut rb_red_blk_tree = 0 as *mut rb_red_blk_tree;
    let mut one: libc::c_double = 1 as libc::c_int as libc::c_double;
    A = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    scanpointsx = gcalloc(
        (2 as libc::c_int * n) as size_t,
        ::std::mem::size_of::<scan_point>() as libc::c_ulong,
    ) as *mut scan_point;
    i = 0 as libc::c_int;
    while i < n {
        (*scanpointsx.offset((2 as libc::c_int * i) as isize)).node = i;
        (*scanpointsx.offset((2 as libc::c_int * i) as isize)).x =
            *x.offset((i * dim) as isize) - *width.offset((i * dim) as isize);
        (*scanpointsx.offset((2 as libc::c_int * i) as isize)).status = INTV_OPEN as libc::c_int;
        (*scanpointsx.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)).node = i + n;
        (*scanpointsx.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)).x =
            *x.offset((i * dim) as isize) + *width.offset((i * dim) as isize);
        (*scanpointsx.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)).status =
            INTV_CLOSE as libc::c_int;
        i += 1;
    }
    qsort(
        scanpointsx as *mut libc::c_void,
        (2 as libc::c_int * n) as size_t,
        ::std::mem::size_of::<scan_point>() as libc::c_ulong,
        Some(
            comp_scan_points
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    scanpointsy = gcalloc(
        (2 as libc::c_int * n) as size_t,
        ::std::mem::size_of::<scan_point>() as libc::c_ulong,
    ) as *mut scan_point;
    i = 0 as libc::c_int;
    while i < n {
        (*scanpointsy.offset(i as isize)).node = i;
        (*scanpointsy.offset(i as isize)).x = *x.offset((i * dim + 1 as libc::c_int) as isize)
            - *width.offset((i * dim + 1 as libc::c_int) as isize);
        (*scanpointsy.offset(i as isize)).status = INTV_OPEN as libc::c_int;
        (*scanpointsy.offset((i + n) as isize)).node = i;
        (*scanpointsy.offset((i + n) as isize)).x = *x
            .offset((i * dim + 1 as libc::c_int) as isize)
            + *width.offset((i * dim + 1 as libc::c_int) as isize);
        (*scanpointsy.offset((i + n) as isize)).status = INTV_CLOSE as libc::c_int;
        i += 1;
    }
    treey = RBTreeCreate(
        Some(
            NodeComp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
        Some(NodeDest as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(InfoDest as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(NodePrint as unsafe extern "C" fn(*const libc::c_void) -> ()),
        Some(InfoPrint as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    i = 0 as libc::c_int;
    's_121: while i < 2 as libc::c_int * n {
        k = (*scanpointsx.offset(i as isize)).node % n;
        if (*scanpointsx.offset(i as isize)).status == INTV_OPEN as libc::c_int {
            RBTreeInsert(
                treey,
                &mut *scanpointsy.offset(k as isize) as *mut scan_point as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
            RBTreeInsert(
                treey,
                &mut *scanpointsy.offset((k + n) as isize) as *mut scan_point as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        } else {
            let mut bsta: libc::c_double = 0.;
            let mut bbsta: libc::c_double = 0.;
            let mut bsto: libc::c_double = 0.;
            let mut bbsto: libc::c_double = 0.;
            let mut ii: libc::c_int = 0;
            if (*scanpointsx.offset(i as isize)).node >= n {
            } else {
                __assert_fail(
                    b"scanpointsx[i].node >= n\0" as *const u8 as *const libc::c_char,
                    b"overlap.c\0" as *const u8 as *const libc::c_char,
                    186 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                        b"SparseMatrix get_overlap_graph(int, int, double *, double *, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            newNode0 = RBExactQuery(
                treey,
                &mut *scanpointsy.offset((k + n) as isize) as *mut scan_point as *mut libc::c_void,
            );
            newNode = newNode0;
            ii = (*((*newNode).key as *mut scan_point)).node;
            if ii < n {
            } else {
                __assert_fail(
                    b"ii < n\0" as *const u8 as *const libc::c_char,
                    b"overlap.c\0" as *const u8 as *const libc::c_char,
                    190 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                        b"SparseMatrix get_overlap_graph(int, int, double *, double *, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            bsta = (*scanpointsy.offset(ii as isize)).x;
            bsto = (*scanpointsy.offset((ii + n) as isize)).x;
            if (*treey).nil != newNode {
            } else {
                __assert_fail(
                    b"treey->nil != newNode\0" as *const u8 as *const libc::c_char,
                    b"overlap.c\0" as *const u8 as *const libc::c_char,
                    198 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                        b"SparseMatrix get_overlap_graph(int, int, double *, double *, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            while !newNode.is_null() && {
                newNode = TreePredecessor(treey, newNode);
                newNode != (*treey).nil
            } {
                neighbor = (*((*newNode).key as *mut scan_point)).node % n;
                bbsta = (*scanpointsy.offset(neighbor as isize)).x;
                bbsto = (*scanpointsy.offset((neighbor + n) as isize)).x;
                if neighbor != k {
                    if !(fabs(0.5f64 * (bsta + bsto) - 0.5f64 * (bbsta + bbsto))
                        < 0.5f64 * (bsto - bsta) + 0.5f64 * (bbsto - bbsta))
                    {
                        continue;
                    }
                    A = SparseMatrix_coordinate_form_add_entry(
                        A,
                        neighbor,
                        k,
                        &mut one as *mut libc::c_double as *mut libc::c_void,
                    );
                    if check_overlap_only != 0 {
                        break 's_121;
                    }
                } else {
                    newNode2 = newNode;
                }
            }
            if !newNode0.is_null() {
                RBDelete(treey, newNode0);
            }
            if !newNode2.is_null() && newNode2 != (*treey).nil && newNode2 != newNode0 {
                if !newNode0.is_null() {
                    RBDelete(treey, newNode2);
                }
            }
        }
        i += 1;
    }
    free(scanpointsx as *mut libc::c_void);
    free(scanpointsy as *mut libc::c_void);
    RBTreeDestroy(treey);
    B = SparseMatrix_from_coordinate_format(A);
    SparseMatrix_delete(A);
    A = SparseMatrix_symmetrize(B, 0 as libc::c_int != 0);
    SparseMatrix_delete(B);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"found %d clashes\n\0" as *const u8 as *const libc::c_char,
            (*A).nz,
        );
    }
    return A;
}
unsafe extern "C" fn relative_position_constraints_delete(mut d: *mut libc::c_void) {
    let mut data: relative_position_constraints = 0 as *mut relative_position_constraints_struct;
    if d.is_null() {
        return;
    }
    data = d as relative_position_constraints;
    free((*data).irn as *mut libc::c_void);
    free((*data).jcn as *mut libc::c_void);
    free((*data).val as *mut libc::c_void);
    free(d);
}
unsafe extern "C" fn relative_position_constraints_new(
    mut A_constr: SparseMatrix,
    mut edge_labeling_scheme: libc::c_int,
    mut n_constr_nodes: libc::c_int,
    mut constr_nodes: *mut libc::c_int,
) -> relative_position_constraints {
    let mut data: relative_position_constraints = 0 as *mut relative_position_constraints_struct;
    if !A_constr.is_null() {
    } else {
        __assert_fail(
            b"A_constr\0" as *const u8 as *const libc::c_char,
            b"overlap.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"relative_position_constraints relative_position_constraints_new(SparseMatrix, int, int, int *)\0",
            ))
                .as_ptr(),
        );
    }
    data = gmalloc(::std::mem::size_of::<relative_position_constraints_struct>() as libc::c_ulong)
        as relative_position_constraints;
    (*data).constr_penalty = 1 as libc::c_int as libc::c_double;
    (*data).edge_labeling_scheme = edge_labeling_scheme;
    (*data).n_constr_nodes = n_constr_nodes;
    let ref mut fresh0 = (*data).constr_nodes;
    *fresh0 = constr_nodes;
    let ref mut fresh1 = (*data).A_constr;
    *fresh1 = A_constr;
    let ref mut fresh2 = (*data).irn;
    *fresh2 = 0 as *mut libc::c_int;
    let ref mut fresh3 = (*data).jcn;
    *fresh3 = 0 as *mut libc::c_int;
    let ref mut fresh4 = (*data).val;
    *fresh4 = 0 as *mut libc::c_double;
    return data;
}
unsafe extern "C" fn scale_coord(
    mut dim: libc::c_int,
    mut m: libc::c_int,
    mut x: *mut libc::c_double,
    mut scale: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < dim * m {
        *x.offset(i as isize) *= scale;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn overlap_scaling(
    mut dim: libc::c_int,
    mut m: libc::c_int,
    mut x: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut scale_sta: libc::c_double,
    mut scale_sto: libc::c_double,
    mut epsilon: libc::c_double,
    mut maxiter: libc::c_int,
) -> libc::c_double {
    let mut scale: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    let mut scale_best: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    let mut C: SparseMatrix = 0 as SparseMatrix;
    let mut check_overlap_only: libc::c_int = 1 as libc::c_int;
    let mut overlap: libc::c_int = 0 as libc::c_int;
    let mut two: libc::c_double = 2 as libc::c_int as libc::c_double;
    let mut iter: libc::c_int = 0 as libc::c_int;
    if epsilon > 0 as libc::c_int as libc::c_double {
    } else {
        __assert_fail(
            b"epsilon > 0\0" as *const u8 as *const libc::c_char,
            b"overlap.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"double overlap_scaling(int, int, double *, double *, double, double, double, int)\0",
            ))
                .as_ptr(),
        );
    }
    if scale_sta <= 0 as libc::c_int as libc::c_double {
        scale_sta = 0 as libc::c_int as libc::c_double;
    } else {
        scale_coord(dim, m, x, scale_sta);
        C = get_overlap_graph(dim, m, x, width, check_overlap_only);
        if C.is_null() || (*C).nz == 0 as libc::c_int {
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b" shrinking with %f works\n\0" as *const u8 as *const libc::c_char,
                    scale_sta,
                );
            }
            SparseMatrix_delete(C);
            return scale_sta;
        }
        scale_coord(dim, m, x, 1.0f64 / scale_sta);
        SparseMatrix_delete(C);
    }
    if scale_sto < 0 as libc::c_int as libc::c_double {
        if scale_sta == 0 as libc::c_int as libc::c_double {
            scale_sto = epsilon;
        } else {
            scale_sto = scale_sta;
        }
        scale_coord(dim, m, x, scale_sto);
        loop {
            scale_sto *= two;
            scale_coord(dim, m, x, two);
            C = get_overlap_graph(dim, m, x, width, check_overlap_only);
            overlap = (!C.is_null() && (*C).nz > 0 as libc::c_int) as libc::c_int;
            SparseMatrix_delete(C);
            if !(overlap != 0) {
                break;
            }
        }
        scale_coord(dim, m, x, 1 as libc::c_int as libc::c_double / scale_sto);
    }
    scale_best = scale_sto;
    loop {
        let fresh5 = iter;
        iter = iter + 1;
        if !(fresh5 < maxiter && scale_sto - scale_sta > epsilon) {
            break;
        }
        if Verbose != 0 {
            fprintf(
                stderr,
                b"in overlap_scaling iter=%d, maxiter=%d, scaling bracket: {%f,%f}\n\0" as *const u8
                    as *const libc::c_char,
                iter,
                maxiter,
                scale_sta,
                scale_sto,
            );
        }
        scale = 0.5f64 * (scale_sta + scale_sto);
        scale_coord(dim, m, x, scale);
        C = get_overlap_graph(dim, m, x, width, check_overlap_only);
        scale_coord(dim, m, x, 1.0f64 / scale);
        overlap = (!C.is_null() && (*C).nz > 0 as libc::c_int) as libc::c_int;
        SparseMatrix_delete(C);
        if overlap != 0 {
            scale_sta = scale;
        } else {
            scale_sto = scale;
            scale_best = scale_sto;
        }
    }
    scale_coord(dim, m, x, scale_best);
    return scale_best;
}
#[no_mangle]
pub unsafe extern "C" fn OverlapSmoother_new(
    mut A: SparseMatrix,
    mut m: libc::c_int,
    mut dim: libc::c_int,
    mut lambda0: libc::c_double,
    mut x: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut include_original_graph: libc::c_int,
    mut neighborhood_only: libc::c_int,
    mut max_overlap: *mut libc::c_double,
    mut min_overlap: *mut libc::c_double,
    mut edge_labeling_scheme: libc::c_int,
    mut n_constr_nodes: libc::c_int,
    mut constr_nodes: *mut libc::c_int,
    mut A_constr: SparseMatrix,
    mut shrink: libc::c_int,
) -> OverlapSmoother {
    let mut sm: OverlapSmoother = 0 as *mut StressMajorizationSmoother_struct;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut iw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jw: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jdiag: libc::c_int = 0;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut lambda: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut diag_d: libc::c_double = 0.;
    let mut diag_w: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    if A.is_null() || SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"(!A) || SparseMatrix_is_symmetric(A, false)\0" as *const u8
                as *const libc::c_char,
            b"overlap.c\0" as *const u8 as *const libc::c_char,
            375 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 154],
                &[libc::c_char; 154],
            >(
                b"OverlapSmoother OverlapSmoother_new(SparseMatrix, int, int, double, double *, double *, int, int, double *, double *, int, int, int *, SparseMatrix, int)\0",
            ))
                .as_ptr(),
        );
    }
    sm = gmalloc(::std::mem::size_of::<StressMajorizationSmoother_struct>() as libc::c_ulong)
        as *mut StressMajorizationSmoother_struct;
    (*sm).scheme = SM_SCHEME_NORMAL as libc::c_int;
    if !constr_nodes.is_null()
        && n_constr_nodes > 0 as libc::c_int
        && edge_labeling_scheme != ELSCHEME_NONE as libc::c_int
    {
        (*sm).scheme = SM_SCHEME_NORMAL_ELABEL as libc::c_int;
        let ref mut fresh6 = (*sm).data;
        *fresh6 = relative_position_constraints_new(
            A_constr,
            edge_labeling_scheme,
            n_constr_nodes,
            constr_nodes,
        ) as *mut libc::c_void;
        let ref mut fresh7 = (*sm).data_deallocator;
        *fresh7 = Some(
            relative_position_constraints_delete as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
    } else {
        let ref mut fresh8 = (*sm).data;
        *fresh8 = 0 as *mut libc::c_void;
    }
    (*sm).tol_cg = 0.01f64;
    (*sm).maxit_cg = sqrt((*A).m as libc::c_double) as libc::c_int;
    let ref mut fresh9 = (*sm).lambda;
    *fresh9 = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    lambda = *fresh9;
    i = 0 as libc::c_int;
    while i < m {
        *((*sm).lambda).offset(i as isize) = lambda0;
        i += 1;
    }
    B = call_tri(m, dim, x);
    if neighborhood_only == 0 {
        let mut C: SparseMatrix = 0 as *mut SparseMatrix_struct;
        let mut D: SparseMatrix = 0 as *mut SparseMatrix_struct;
        C = get_overlap_graph(dim, m, x, width, 0 as libc::c_int);
        D = SparseMatrix_add(B, C);
        SparseMatrix_delete(B);
        SparseMatrix_delete(C);
        B = D;
    }
    if include_original_graph != 0 {
        let ref mut fresh10 = (*sm).Lw;
        *fresh10 = SparseMatrix_add(A, B);
        SparseMatrix_delete(B);
    } else {
        let ref mut fresh11 = (*sm).Lw;
        *fresh11 = B;
    }
    let ref mut fresh12 = (*sm).Lwd;
    *fresh12 = SparseMatrix_copy((*sm).Lw);
    if ((*sm).Lw).is_null() || ((*sm).Lwd).is_null() {
        OverlapSmoother_delete(sm);
        return 0 as OverlapSmoother;
    }
    if (*(*sm).Lwd).type_0 == MATRIX_TYPE_REAL as libc::c_int {
    } else {
        __assert_fail(
            b"(sm->Lwd)->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"overlap.c\0" as *const u8 as *const libc::c_char,
            425 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 154],
                &[libc::c_char; 154],
            >(
                b"OverlapSmoother OverlapSmoother_new(SparseMatrix, int, int, double, double *, double *, int, int, double *, double *, int, int, int *, SparseMatrix, int)\0",
            ))
                .as_ptr(),
        );
    }
    ideal_distance_avoid_overlap(
        dim,
        (*sm).Lwd,
        x,
        width,
        (*(*sm).Lwd).a as *mut libc::c_double,
        max_overlap,
        min_overlap,
    );
    if *max_overlap < 1 as libc::c_int as libc::c_double && shrink != 0 {
        let mut scale_sta: libc::c_double =
            if (1 as libc::c_int as libc::c_double) < *max_overlap * 1.0001f64 {
                1 as libc::c_int as libc::c_double
            } else {
                *max_overlap * 1.0001f64
            };
        let mut scale_sto: libc::c_double = 1 as libc::c_int as libc::c_double;
        if Verbose != 0 {
            fprintf(
                stderr,
                b" no overlap (overlap = %f), rescale to shrink\n\0" as *const u8
                    as *const libc::c_char,
                *max_overlap - 1 as libc::c_int as libc::c_double,
            );
        }
        scale_sta = overlap_scaling(
            dim,
            m,
            x,
            width,
            scale_sta,
            scale_sto,
            0.0001f64,
            15 as libc::c_int,
        );
        *max_overlap = 1 as libc::c_int as libc::c_double;
    } else {
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
                    if *d.offset(j as isize) > 0 as libc::c_int as libc::c_double {
                        *w.offset(j as isize) = -(100 as libc::c_int) as libc::c_double
                            / *d.offset(j as isize)
                            / *d.offset(j as isize);
                    } else {
                        *w.offset(j as isize) = -(1 as libc::c_int) as libc::c_double
                            / *d.offset(j as isize)
                            / *d.offset(j as isize);
                        *d.offset(j as isize) = -*d.offset(j as isize);
                    }
                    dist = *d.offset(j as isize);
                    diag_w += *w.offset(j as isize);
                    *d.offset(j as isize) = *w.offset(j as isize) * dist;
                    diag_d += *d.offset(j as isize);
                }
                j += 1;
            }
            *lambda.offset(i as isize) *= -diag_w;
            if jdiag >= 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"jdiag >= 0\0" as *const u8 as *const libc::c_char,
                    b"overlap.c\0" as *const u8 as *const libc::c_char,
                    468 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 154],
                        &[libc::c_char; 154],
                    >(
                        b"OverlapSmoother OverlapSmoother_new(SparseMatrix, int, int, double, double *, double *, int, int, double *, double *, int, int, int *, SparseMatrix, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            *w.offset(jdiag as isize) = -diag_w + *lambda.offset(i as isize);
            *d.offset(jdiag as isize) = -diag_d;
            i += 1;
        }
    }
    return sm;
}
#[no_mangle]
pub unsafe extern "C" fn OverlapSmoother_delete(mut sm: OverlapSmoother) {
    StressMajorizationSmoother_delete(sm);
}
#[no_mangle]
pub unsafe extern "C" fn OverlapSmoother_smooth(
    mut sm: OverlapSmoother,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) -> libc::c_double {
    let mut maxit_sm: libc::c_int = 1 as libc::c_int;
    let mut res: libc::c_double = StressMajorizationSmoother_smooth(sm, dim, x, maxit_sm, 0.001f64);
    return res;
}
unsafe extern "C" fn scale_to_edge_length(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
    mut avg_label_size: libc::c_double,
) {
    let mut dist: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if A.is_null() {
        return;
    }
    dist = average_edge_length(A, dim, x);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"avg edge len=%f avg_label-size= %f\n\0" as *const u8 as *const libc::c_char,
            dist,
            avg_label_size,
        );
    }
    dist = avg_label_size / (if dist > 1.0e-16f64 { dist } else { 1.0e-16f64 });
    i = 0 as libc::c_int;
    while i < dim * (*A).m {
        *x.offset(i as isize) *= dist;
        i += 1;
    }
}
unsafe extern "C" fn print_bounding_box(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) {
    let mut xmin: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xmax: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    xmin = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    xmax = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < dim {
        let ref mut fresh13 = *xmax.offset(i as isize);
        *fresh13 = *x.offset(i as isize);
        *xmin.offset(i as isize) = *fresh13;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            *xmin.offset(k as isize) =
                if *xmin.offset(k as isize) < *x.offset((i * dim + k) as isize) {
                    *xmin.offset(k as isize)
                } else {
                    *x.offset((i * dim + k) as isize)
                };
            *xmax.offset(k as isize) =
                if *xmax.offset(k as isize) > *x.offset((i * dim + k) as isize) {
                    *xmax.offset(k as isize)
                } else {
                    *x.offset((i * dim + k) as isize)
                };
            k += 1;
        }
        i += 1;
    }
    fprintf(
        stderr,
        b"bounding box = \n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < dim {
        fprintf(
            stderr,
            b"{%f,%f}, \0" as *const u8 as *const libc::c_char,
            *xmin.offset(i as isize),
            *xmax.offset(i as isize),
        );
        i += 1;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    free(xmin as *mut libc::c_void);
    free(xmax as *mut libc::c_void);
}
unsafe extern "C" fn check_convergence(
    mut max_overlap: libc::c_double,
    mut res: libc::c_double,
    mut has_penalty_terms: libc::c_int,
    mut epsilon: libc::c_double,
) -> libc::c_int {
    if has_penalty_terms == 0 {
        return (max_overlap <= 1 as libc::c_int as libc::c_double) as libc::c_int;
    }
    return (res < epsilon) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remove_overlap(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
    mut label_sizes: *mut libc::c_double,
    mut ntry: libc::c_int,
    mut initial_scaling: libc::c_double,
    mut edge_labeling_scheme: libc::c_int,
    mut n_constr_nodes: libc::c_int,
    mut constr_nodes: *mut libc::c_int,
    mut A_constr: SparseMatrix,
    mut do_shrinking: libc::c_int,
) {
    let mut lambda: libc::c_double = 0.00f64;
    let mut sm: OverlapSmoother = 0 as *mut StressMajorizationSmoother_struct;
    let mut include_original_graph: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut LARGE: libc::c_double = 100000 as libc::c_int as libc::c_double;
    let mut avg_label_size: libc::c_double = 0.;
    let mut res: libc::c_double = LARGE;
    let mut max_overlap: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut min_overlap: libc::c_double = 999 as libc::c_int as libc::c_double;
    let mut neighborhood_only: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    let mut has_penalty_terms: libc::c_int = 0 as libc::c_int;
    let mut epsilon: libc::c_double = 0.005f64;
    let mut shrink: libc::c_int = 0 as libc::c_int;
    if label_sizes.is_null() {
        return;
    }
    if initial_scaling < 0 as libc::c_int as libc::c_double {
        avg_label_size = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < (*A).m {
            avg_label_size += *label_sizes.offset((i * dim) as isize)
                + *label_sizes.offset((i * dim + 1 as libc::c_int) as isize);
            i += 1;
        }
        avg_label_size /= (*A).m as libc::c_double;
        scale_to_edge_length(dim, A, x, -initial_scaling * avg_label_size);
    } else if initial_scaling > 0 as libc::c_int as libc::c_double {
        scale_to_edge_length(dim, A, x, initial_scaling);
    }
    if ntry == 0 {
        return;
    }
    has_penalty_terms = (edge_labeling_scheme != ELSCHEME_NONE as libc::c_int
        && n_constr_nodes > 0 as libc::c_int) as libc::c_int;
    i = 0 as libc::c_int;
    while i < ntry {
        if Verbose != 0 {
            print_bounding_box((*A).m, dim, x);
        }
        sm = OverlapSmoother_new(
            A,
            (*A).m,
            dim,
            lambda,
            x,
            label_sizes,
            include_original_graph,
            neighborhood_only,
            &mut max_overlap,
            &mut min_overlap,
            edge_labeling_scheme,
            n_constr_nodes,
            constr_nodes,
            A_constr,
            shrink,
        );
        if Verbose != 0 {
            fprintf(
                stderr,
                b"overlap removal neighbors only?= %d iter -- %d, overlap factor = %g underlap factor = %g\n\0"
                    as *const u8 as *const libc::c_char,
                neighborhood_only,
                i,
                max_overlap - 1 as libc::c_int as libc::c_double,
                min_overlap,
            );
        }
        if check_convergence(max_overlap, res, has_penalty_terms, epsilon) != 0 {
            OverlapSmoother_delete(sm);
            if neighborhood_only == 0 {
                break;
            }
            res = LARGE;
            neighborhood_only = 0 as libc::c_int;
            if do_shrinking != 0 {
                shrink = 1 as libc::c_int;
            }
        } else {
            res = OverlapSmoother_smooth(sm, dim, x);
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"res = %f\n\0" as *const u8 as *const libc::c_char,
                    res,
                );
            }
            OverlapSmoother_delete(sm);
        }
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"overlap removal neighbors only?= %d iter -- %d, overlap factor = %g underlap factor = %g\n\0"
                as *const u8 as *const libc::c_char,
            neighborhood_only,
            i,
            max_overlap - 1 as libc::c_int as libc::c_double,
            min_overlap,
        );
    }
    if has_penalty_terms != 0 {
        remove_overlap(
            dim,
            A,
            x,
            label_sizes,
            ntry,
            0.0f64,
            ELSCHEME_NONE as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_int,
            0 as SparseMatrix,
            do_shrinking,
        );
    }
}
