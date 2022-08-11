#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn free(_: *mut libc::c_void);
    fn srand(__seed: libc::c_uint);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn drand() -> libc::c_double;
    fn distance(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn distance_cropped(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
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
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_is_symmetric(
        A: SparseMatrix,
        test_pattern_symmetry_only: bool,
    ) -> libc::c_int;
    fn SparseMatrix_symmetrize(
        A: SparseMatrix,
        pattern_symmetric_only: bool,
    ) -> SparseMatrix;
    fn SparseMatrix_symmetrize_nodiag(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_remove_diagonal(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(
        A: SparseMatrix,
    ) -> SparseMatrix;
    fn SparseMatrix_multiply_dense(
        A: SparseMatrix,
        v: *mut libc::c_double,
        res: *mut *mut libc::c_double,
        dim: libc::c_int,
    );
    fn SparseMatrix_has_diagonal(A: SparseMatrix) -> libc::c_int;
    fn QuadTree_new_from_point_list(
        dim: libc::c_int,
        n: libc::c_int,
        max_level: libc::c_int,
        coord: *mut libc::c_double,
    ) -> QuadTree;
    fn QuadTree_delete(q: QuadTree);
    fn QuadTree_get_supernodes(
        qt: QuadTree,
        bh: libc::c_double,
        point: *mut libc::c_double,
        nodeid: libc::c_int,
        nsuper: *mut libc::c_int,
        nsupermax: *mut libc::c_int,
        center: *mut *mut libc::c_double,
        supernode_wgts: *mut *mut libc::c_double,
        distances: *mut *mut libc::c_double,
        counts: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    fn QuadTree_get_repulsive_force(
        qt: QuadTree,
        force: *mut libc::c_double,
        x: *mut libc::c_double,
        bh: libc::c_double,
        p: libc::c_double,
        KP: libc::c_double,
        counts: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    fn Multilevel_control_new(
        scheme: libc::c_int,
        mode: libc::c_int,
    ) -> Multilevel_control;
    fn Multilevel_control_delete(ctrl: Multilevel_control);
    fn Multilevel_delete(grid: Multilevel);
    fn Multilevel_new(
        A: SparseMatrix,
        D: SparseMatrix,
        ctrl: Multilevel_control,
    ) -> Multilevel;
    fn Multilevel_get_coarsest(grid: Multilevel) -> Multilevel;
    fn post_process_smoothing(
        dim: libc::c_int,
        A: SparseMatrix,
        ctrl: spring_electrical_control,
        x: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    fn remove_overlap(
        dim: libc::c_int,
        A: SparseMatrix,
        x: *mut libc::c_double,
        label_sizes: *mut libc::c_double,
        ntry: libc::c_int,
        initial_scaling: libc::c_double,
        edge_labeling_scheme: libc::c_int,
        n_constr_nodes: libc::c_int,
        constr_nodes: *mut libc::c_int,
        A_constr: SparseMatrix,
        doShrink: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type C2RustUnnamed_1 = libc::c_int;
pub const ERROR_NOT_SQUARE_MATRIX: C2RustUnnamed_1 = -100;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SMOOTHING_RNG: C2RustUnnamed_2 = 6;
pub const SMOOTHING_TRIANGLE: C2RustUnnamed_2 = 5;
pub const SMOOTHING_SPRING: C2RustUnnamed_2 = 4;
pub const SMOOTHING_STRESS_MAJORIZATION_POWER_DIST: C2RustUnnamed_2 = 3;
pub const SMOOTHING_STRESS_MAJORIZATION_AVG_DIST: C2RustUnnamed_2 = 2;
pub const SMOOTHING_STRESS_MAJORIZATION_GRAPH_DIST: C2RustUnnamed_2 = 1;
pub const SMOOTHING_NONE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const QUAD_TREE_HYBRID_SIZE: C2RustUnnamed_3 = 10000;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const QUAD_TREE_HYBRID: C2RustUnnamed_4 = 3;
pub const QUAD_TREE_FAST: C2RustUnnamed_4 = 2;
pub const QUAD_TREE_NORMAL: C2RustUnnamed_4 = 1;
pub const QUAD_TREE_NONE: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_int;
pub const METHOD_STO: C2RustUnnamed_5 = 8;
pub const METHOD_NONE: C2RustUnnamed_5 = 7;
pub const METHOD_FULL_STRESS: C2RustUnnamed_5 = 6;
pub const METHOD_UNIFORM_STRESS: C2RustUnnamed_5 = 5;
pub const METHOD_STRESS: C2RustUnnamed_5 = 4;
pub const METHOD_STRESS_APPROX: C2RustUnnamed_5 = 3;
pub const METHOD_STRESS_MAXENT: C2RustUnnamed_5 = 2;
pub const METHOD_SPRING_MAXENT: C2RustUnnamed_5 = 1;
pub const METHOD_SPRING_ELECTRICAL: C2RustUnnamed_5 = 0;
pub const METHOD_STA: C2RustUnnamed_5 = -1;
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
pub const COARSEN_MODE_FORCEFUL: C2RustUnnamed_8 = 1;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_SUPERNODES_FIRST: C2RustUnnamed_7 = 4;
pub type oned_optimizer = *mut oned_optimizer_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oned_optimizer_struct {
    pub i: libc::c_int,
    pub work: [libc::c_double; 21],
    pub direction: libc::c_int,
}
pub const OPT_UP: C2RustUnnamed_6 = 1;
pub const MAX_I: C2RustUnnamed_6 = 20;
pub const OPT_DOWN: C2RustUnnamed_6 = -1;
pub const OPT_INIT: C2RustUnnamed_6 = 0;
pub type QuadTree = *mut QuadTree_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QuadTree_struct {
    pub n: libc::c_int,
    pub total_weight: libc::c_double,
    pub dim: libc::c_int,
    pub center: *mut libc::c_double,
    pub width: libc::c_double,
    pub average: *mut libc::c_double,
    pub qts: *mut QuadTree,
    pub l: SingleLinkedList,
    pub max_level: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type SingleLinkedList = *mut SingleLinkedList_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SingleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: SingleLinkedList,
}
pub type Multilevel = *mut Multilevel_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Multilevel_struct {
    pub level: libc::c_int,
    pub n: libc::c_int,
    pub A: SparseMatrix,
    pub D: SparseMatrix,
    pub P: SparseMatrix,
    pub R: SparseMatrix,
    pub node_weights: *mut libc::c_double,
    pub next: Multilevel,
    pub prev: Multilevel,
    pub delete_top_level_A: libc::c_int,
    pub coarsen_scheme_used: libc::c_int,
}
pub type Multilevel_control = *mut Multilevel_control_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Multilevel_control_struct {
    pub minsize: libc::c_int,
    pub min_coarsen_factor: libc::c_double,
    pub maxlevel: libc::c_int,
    pub randomize: libc::c_int,
    pub coarsen_scheme: libc::c_int,
    pub coarsen_mode: libc::c_int,
}
pub const VERTEX_BASED_STO: C2RustUnnamed_7 = 11;
pub const VERTEX_BASED_STA: C2RustUnnamed_7 = 8;
pub const EDGE_BASED_STO: C2RustUnnamed_7 = 7;
pub const EDGE_BASED_STA: C2RustUnnamed_7 = 0;
pub const ELSCHEME_STRAIGHTLINE_PENALTY2: C2RustUnnamed_9 = 4;
pub const ELSCHEME_STRAIGHTLINE_PENALTY: C2RustUnnamed_9 = 3;
pub type C2RustUnnamed_6 = libc::c_int;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const COARSEN_HYBRID: C2RustUnnamed_7 = 12;
pub const COARSEN_INDEPENDENT_VERTEX_SET_RS: C2RustUnnamed_7 = 10;
pub const COARSEN_INDEPENDENT_VERTEX_SET: C2RustUnnamed_7 = 9;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_CLUSTER_PERNODE_LEAVES_FIRST: C2RustUnnamed_7 = 6;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_DEGREE_SCALED: C2RustUnnamed_7 = 5;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_LEAVES_FIRST: C2RustUnnamed_7 = 3;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE: C2RustUnnamed_7 = 2;
pub const COARSEN_INDEPENDENT_EDGE_SET: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const COARSEN_MODE_GENTLE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const ELSCHEME_PENALTY2: C2RustUnnamed_9 = 2;
pub const ELSCHEME_PENALTY: C2RustUnnamed_9 = 1;
pub const ELSCHEME_NONE: C2RustUnnamed_9 = 0;
#[no_mangle]
pub unsafe extern "C" fn spring_electrical_control_new() -> spring_electrical_control {
    let mut ctrl: spring_electrical_control = 0 as *mut spring_electrical_control_struct;
    ctrl = gmalloc(
        ::std::mem::size_of::<spring_electrical_control_struct>() as libc::c_ulong,
    ) as spring_electrical_control;
    (*ctrl).p = -1.0001234f64;
    (*ctrl).q = 1 as libc::c_int as libc::c_double;
    (*ctrl).random_start = (0 as libc::c_int == 0) as libc::c_int;
    (*ctrl).K = -(1 as libc::c_int) as libc::c_double;
    (*ctrl).C = 0.2f64;
    (*ctrl).multilevels = 0 as libc::c_int;
    (*ctrl)
        .multilevel_coarsen_scheme = COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_SUPERNODES_FIRST
        as libc::c_int;
    (*ctrl).multilevel_coarsen_mode = COARSEN_MODE_FORCEFUL as libc::c_int;
    (*ctrl).quadtree_size = 45 as libc::c_int;
    (*ctrl).max_qtree_level = 10 as libc::c_int;
    (*ctrl).bh = 0.6f64;
    (*ctrl).tol = 0.001f64;
    (*ctrl).maxiter = 500 as libc::c_int;
    (*ctrl).cool = 0.90f64;
    (*ctrl).step = 0.1f64;
    (*ctrl).adaptive_cooling = (0 as libc::c_int == 0) as libc::c_int;
    (*ctrl).random_seed = 123 as libc::c_int;
    (*ctrl).beautify_leaves = 0 as libc::c_int;
    (*ctrl).smoothing = SMOOTHING_NONE as libc::c_int;
    (*ctrl).overlap = 0 as libc::c_int;
    (*ctrl).do_shrinking = 1 as libc::c_int;
    (*ctrl).tscheme = QUAD_TREE_HYBRID as libc::c_int;
    (*ctrl).method = METHOD_SPRING_ELECTRICAL as libc::c_int;
    (*ctrl).initial_scaling = -(4 as libc::c_int) as libc::c_double;
    (*ctrl).rotation = 0.0f64;
    (*ctrl).edge_labeling_scheme = 0 as libc::c_int;
    return ctrl;
}
#[no_mangle]
pub unsafe extern "C" fn spring_electrical_control_delete(
    mut ctrl: spring_electrical_control,
) {
    free(ctrl as *mut libc::c_void);
}
static mut smoothings: [*mut libc::c_char; 7] = [
    b"NONE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STRESS_MAJORIZATION_GRAPH_DIST\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"STRESS_MAJORIZATION_AVG_DIST\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"STRESS_MAJORIZATION_POWER_DIST\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"SPRING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TRIANGLE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"RNG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut tschemes: [*mut libc::c_char; 4] = [
    b"NONE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"NORMAL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"FAST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"HYBRID\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut methods: [*mut libc::c_char; 8] = [
    b"SPRING_ELECTRICAL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SPRING_MAXENT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STRESS_MAXENT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STRESS_APPROX\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STRESS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"UNIFORM_STRESS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"FULL_STRESS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"NONE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn spring_electrical_control_print(
    mut ctrl: spring_electrical_control,
) {
    fprintf(
        stderr,
        b"spring_electrical_control:\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"  repulsive and attractive exponents: %.03f %.03f\n\0" as *const u8
            as *const libc::c_char,
        (*ctrl).p,
        (*ctrl).q,
    );
    fprintf(
        stderr,
        b"  random start %d seed %d\n\0" as *const u8 as *const libc::c_char,
        (*ctrl).random_start,
        (*ctrl).random_seed,
    );
    fprintf(
        stderr,
        b"  K : %.03f C : %.03f\n\0" as *const u8 as *const libc::c_char,
        (*ctrl).K,
        (*ctrl).C,
    );
    fprintf(
        stderr,
        b"  max levels %d coarsen_scheme %d coarsen_node %d\n\0" as *const u8
            as *const libc::c_char,
        (*ctrl).multilevels,
        (*ctrl).multilevel_coarsen_scheme,
        (*ctrl).multilevel_coarsen_mode,
    );
    fprintf(
        stderr,
        b"  quadtree size %d max_level %d\n\0" as *const u8 as *const libc::c_char,
        (*ctrl).quadtree_size,
        (*ctrl).max_qtree_level,
    );
    fprintf(
        stderr,
        b"  Barnes-Hutt constant %.03f tolerance  %.03f maxiter %d\n\0" as *const u8
            as *const libc::c_char,
        (*ctrl).bh,
        (*ctrl).tol,
        (*ctrl).maxiter,
    );
    fprintf(
        stderr,
        b"  cooling %.03f step size  %.03f adaptive %d\n\0" as *const u8
            as *const libc::c_char,
        (*ctrl).cool,
        (*ctrl).step,
        (*ctrl).adaptive_cooling,
    );
    fprintf(
        stderr,
        b"  beautify_leaves %d node weights %d rotation %.03f\n\0" as *const u8
            as *const libc::c_char,
        (*ctrl).beautify_leaves,
        0 as libc::c_int,
        (*ctrl).rotation,
    );
    fprintf(
        stderr,
        b"  smoothing %s overlap %d initial_scaling %.03f do_shrinking %d\n\0"
            as *const u8 as *const libc::c_char,
        smoothings[(*ctrl).smoothing as usize],
        (*ctrl).overlap,
        (*ctrl).initial_scaling,
        (*ctrl).do_shrinking,
    );
    fprintf(
        stderr,
        b"  octree scheme %s method %s\n\0" as *const u8 as *const libc::c_char,
        tschemes[(*ctrl).tscheme as usize],
        methods[(*ctrl).method as usize],
    );
    fprintf(
        stderr,
        b"  edge_labeling_scheme %d\n\0" as *const u8 as *const libc::c_char,
        (*ctrl).edge_labeling_scheme,
    );
}
#[no_mangle]
pub unsafe extern "C" fn oned_optimizer_delete(mut opt: oned_optimizer) {
    free(opt as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn oned_optimizer_new(mut i: libc::c_int) -> oned_optimizer {
    let mut opt: oned_optimizer = 0 as *mut oned_optimizer_struct;
    opt = gmalloc(::std::mem::size_of::<oned_optimizer_struct>() as libc::c_ulong)
        as oned_optimizer;
    (*opt).i = i;
    (*opt).direction = OPT_INIT as libc::c_int;
    return opt;
}
#[no_mangle]
pub unsafe extern "C" fn oned_optimizer_train(
    mut opt: oned_optimizer,
    mut work: libc::c_double,
) {
    let mut i: libc::c_int = (*opt).i;
    if i >= 0 as libc::c_int {} else {
        __assert_fail(
            b"i >= 0\0" as *const u8 as *const libc::c_char,
            b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void oned_optimizer_train(oned_optimizer, double)\0"))
                .as_ptr(),
        );
    }
    (*opt).work[i as usize] = work;
    if (*opt).direction == OPT_INIT as libc::c_int {
        if (*opt).i == MAX_I as libc::c_int {
            (*opt).direction = OPT_DOWN as libc::c_int;
            (*opt).i = (*opt).i - 1 as libc::c_int;
        } else {
            (*opt).direction = OPT_UP as libc::c_int;
            (*opt)
                .i = if (MAX_I as libc::c_int) < (*opt).i + 1 as libc::c_int {
                MAX_I as libc::c_int
            } else {
                (*opt).i + 1 as libc::c_int
            };
        }
    } else if (*opt).direction == OPT_UP as libc::c_int {
        if i >= 1 as libc::c_int {} else {
            __assert_fail(
                b"i >= 1\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void oned_optimizer_train(oned_optimizer, double)\0"))
                    .as_ptr(),
            );
        }
        if (*opt).work[i as usize] < (*opt).work[(i - 1 as libc::c_int) as usize]
            && (*opt).i < MAX_I as libc::c_int
        {
            (*opt)
                .i = if (MAX_I as libc::c_int) < (*opt).i + 1 as libc::c_int {
                MAX_I as libc::c_int
            } else {
                (*opt).i + 1 as libc::c_int
            };
        } else {
            let ref mut fresh0 = (*opt).i;
            *fresh0 -= 1;
            (*opt).direction = OPT_DOWN as libc::c_int;
        }
    } else {
        if i < MAX_I as libc::c_int {} else {
            __assert_fail(
                b"i < MAX_I\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void oned_optimizer_train(oned_optimizer, double)\0"))
                    .as_ptr(),
            );
        }
        if (*opt).work[i as usize] < (*opt).work[(i + 1 as libc::c_int) as usize]
            && (*opt).i > 0 as libc::c_int
        {
            (*opt)
                .i = if 0 as libc::c_int > (*opt).i - 1 as libc::c_int {
                0 as libc::c_int
            } else {
                (*opt).i - 1 as libc::c_int
            };
        } else {
            let ref mut fresh1 = (*opt).i;
            *fresh1 += 1;
            (*opt).direction = OPT_UP as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn oned_optimizer_get(mut opt: oned_optimizer) -> libc::c_int {
    return (*opt).i;
}
#[no_mangle]
pub unsafe extern "C" fn average_edge_length(
    mut A: SparseMatrix,
    mut dim: libc::c_int,
    mut coord: *mut libc::c_double,
) -> libc::c_double {
    let mut dist: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut d: libc::c_double = 0.;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if SparseMatrix_is_symmetric(A, 1 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, true)\0" as *const u8 as *const libc::c_char,
            b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"double average_edge_length(SparseMatrix, int, double *)\0"))
                .as_ptr(),
        );
    }
    if *ia.offset((*A).m as isize) == 0 as libc::c_int {
        return 1 as libc::c_int as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < (*A).m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            d = 0 as libc::c_int as libc::c_double;
            k = 0 as libc::c_int;
            while k < dim {
                d
                    += (*coord.offset((dim * i + k) as isize)
                        - *coord.offset((dim * *ja.offset(j as isize)) as isize))
                        * (*coord.offset((dim * i + k) as isize)
                            - *coord.offset((dim * *ja.offset(j as isize)) as isize));
                k += 1;
            }
            dist += sqrt(d);
            j += 1;
        }
        i += 1;
    }
    return dist / *ia.offset((*A).m as isize) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn export_embedding(
    mut fp: *mut FILE,
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
    mut width: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut ne: libc::c_int = 0 as libc::c_int;
    let mut xsize: libc::c_double = 0.;
    let mut ysize: libc::c_double = 0.;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    xmin = *x.offset(0 as libc::c_int as isize);
    xmax = xmin;
    ymin = *x.offset(1 as libc::c_int as isize);
    ymax = ymin;
    i = 0 as libc::c_int;
    while i < (*A).m {
        xmax = if xmax > *x.offset((i * dim) as isize) {
            xmax
        } else {
            *x.offset((i * dim) as isize)
        };
        xmin = if xmin < *x.offset((i * dim) as isize) {
            xmin
        } else {
            *x.offset((i * dim) as isize)
        };
        ymax = if ymax > *x.offset((i * dim + 1 as libc::c_int) as isize) {
            ymax
        } else {
            *x.offset((i * dim + 1 as libc::c_int) as isize)
        };
        ymin = if ymin < *x.offset((i * dim + 1 as libc::c_int) as isize) {
            ymin
        } else {
            *x.offset((i * dim + 1 as libc::c_int) as isize)
        };
        i += 1;
    }
    xsize = xmax - xmin;
    ysize = ymax - ymin;
    xsize = if xsize > ysize { xsize } else { ysize };
    if dim == 2 as libc::c_int {
        fprintf(
            fp,
            b"Graphics[{GrayLevel[0.5],Line[{\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            fp,
            b"Graphics3D[{GrayLevel[0.5],Line[{\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < (*A).m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(*ja.offset(j as isize) == i) {
                ne += 1;
                if ne > 1 as libc::c_int {
                    fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
                }
                fprintf(fp, b"{{\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < dim {
                    if k > 0 as libc::c_int {
                        fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
                    }
                    fprintf(
                        fp,
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *x.offset((i * dim + k) as isize),
                    );
                    k += 1;
                }
                fprintf(fp, b"},{\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < dim {
                    if k > 0 as libc::c_int {
                        fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
                    }
                    fprintf(
                        fp,
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *x.offset((*ja.offset(j as isize) * dim + k) as isize),
                    );
                    k += 1;
                }
                fprintf(fp, b"}}\0" as *const u8 as *const libc::c_char);
            }
            j += 1;
        }
        i += 1;
    }
    fprintf(fp, b"}],Hue[%f]\0" as *const u8 as *const libc::c_char, 1.0f64);
    if !width.is_null() && dim == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*A).m {
            if i >= 0 as libc::c_int {
                fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                fp,
                b"(*width={%f,%f}, x = {%f,%f}*){GrayLevel[.5,.5],Rectangle[{%f,%f},{%f,%f}]}\0"
                    as *const u8 as *const libc::c_char,
                *width.offset((i * dim) as isize),
                *width.offset((i * dim + 1 as libc::c_int) as isize),
                *x.offset((i * dim) as isize),
                *x.offset((i * dim + 1 as libc::c_int) as isize),
                *x.offset((i * dim) as isize) - *width.offset((i * dim) as isize),
                *x.offset((i * dim + 1 as libc::c_int) as isize)
                    - *width.offset((i * dim + 1 as libc::c_int) as isize),
                *x.offset((i * dim) as isize) + *width.offset((i * dim) as isize),
                *x.offset((i * dim + 1 as libc::c_int) as isize)
                    + *width.offset((i * dim + 1 as libc::c_int) as isize),
            );
            i += 1;
        }
    }
    if (*A).m < 100 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*A).m {
            if i >= 0 as libc::c_int {
                fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                fp,
                b"Text[%d,{\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
            k = 0 as libc::c_int;
            while k < dim {
                if k > 0 as libc::c_int {
                    fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
                }
                fprintf(
                    fp,
                    b"%f\0" as *const u8 as *const libc::c_char,
                    *x.offset((i * dim + k) as isize),
                );
                k += 1;
            }
            fprintf(fp, b"}]\0" as *const u8 as *const libc::c_char);
            i += 1;
        }
    } else if (*A).m < 500000 as libc::c_int {
        fprintf(fp, b", Point[{\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*A).m {
            if i > 0 as libc::c_int {
                fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(fp, b"{\0" as *const u8 as *const libc::c_char);
            k = 0 as libc::c_int;
            while k < dim {
                if k > 0 as libc::c_int {
                    fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
                }
                fprintf(
                    fp,
                    b"%f\0" as *const u8 as *const libc::c_char,
                    *x.offset((i * dim + k) as isize),
                );
                k += 1;
            }
            fprintf(fp, b"}\0" as *const u8 as *const libc::c_char);
            i += 1;
        }
        fprintf(fp, b"}]\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(fp, b"{}\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        fp,
        b"},ImageSize->%f]\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_double * xsize / 2 as libc::c_int as libc::c_double,
    );
}
unsafe extern "C" fn update_step(
    mut adaptive_cooling: libc::c_int,
    mut step: libc::c_double,
    mut Fnorm: libc::c_double,
    mut Fnorm0: libc::c_double,
    mut cool: libc::c_double,
) -> libc::c_double {
    if adaptive_cooling == 0 {
        return cool * step;
    }
    if Fnorm >= Fnorm0 {
        step = cool * step;
    } else if !(Fnorm > 0.95f64 * Fnorm0) {
        step = 0.99f64 * step / cool;
    }
    return step;
}
unsafe extern "C" fn check_real_array_size(
    mut a: *mut *mut libc::c_double,
    mut len: libc::c_int,
    mut lenmax: *mut libc::c_int,
) {
    if len >= *lenmax {
        *lenmax = len
            + (if 0.2f64 as libc::c_int * len > 10 as libc::c_int {
                0.2f64 as libc::c_int * len
            } else {
                10 as libc::c_int
            });
        *a = grealloc(
            *a as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(*lenmax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
}
unsafe extern "C" fn check_int_array_size(
    mut a: *mut *mut libc::c_int,
    mut len: libc::c_int,
    mut lenmax: *mut libc::c_int,
) {
    if len >= *lenmax {
        *lenmax = len
            + (if 0.2f64 as libc::c_int * len > 10 as libc::c_int {
                0.2f64 as libc::c_int * len
            } else {
                10 as libc::c_int
            });
        *a = grealloc(
            *a as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(*lenmax as libc::c_ulong),
        ) as *mut libc::c_int;
    }
}
unsafe extern "C" fn get_angle(
    mut x: *mut libc::c_double,
    mut dim: libc::c_int,
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut k: libc::c_int = 0;
    let mut y: [libc::c_double; 2] = [0.; 2];
    let mut res: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.00001f64;
    k = 0 as libc::c_int;
    while k < 2 as libc::c_int {
        y[k
            as usize] = *x.offset((j * dim + k) as isize)
            - *x.offset((i * dim + k) as isize);
        k += 1;
    }
    if fabs(y[0 as libc::c_int as usize]) <= fabs(y[1 as libc::c_int as usize]) * eps {
        if y[1 as libc::c_int as usize] > 0 as libc::c_int as libc::c_double {
            return 0.5f64 * 3.14159265358979323846f64;
        }
        return 1.5f64 * 3.14159265358979323846f64;
    }
    res = atan(y[1 as libc::c_int as usize] / y[0 as libc::c_int as usize]);
    if y[0 as libc::c_int as usize] > 0 as libc::c_int as libc::c_double {
        if y[1 as libc::c_int as usize] < 0 as libc::c_int as libc::c_double {
            res = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64 + res;
        }
    } else if y[0 as libc::c_int as usize] < 0 as libc::c_int as libc::c_double {
        res = res + 3.14159265358979323846f64;
    }
    return res;
}
unsafe extern "C" fn comp_real(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    let mut xx: *const libc::c_double = x as *const libc::c_double;
    let mut yy: *const libc::c_double = y as *const libc::c_double;
    if *xx > *yy {
        return 1 as libc::c_int
    } else {
        if *xx < *yy {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sort_real(mut n: libc::c_int, mut a: *mut libc::c_double) {
    qsort(
        a as *mut libc::c_void,
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        Some(
            comp_real
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn set_leaves(
    mut x: *mut libc::c_double,
    mut dim: libc::c_int,
    mut dist: libc::c_double,
    mut ang: libc::c_double,
    mut i: libc::c_int,
    mut j: libc::c_int,
) {
    *x.offset((dim * j) as isize) = cos(ang) * dist + *x.offset((dim * i) as isize);
    *x
        .offset(
            (dim * j + 1 as libc::c_int) as isize,
        ) = sin(ang) * dist + *x.offset((dim * i + 1 as libc::c_int) as isize);
}
unsafe extern "C" fn beautify_leaves(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*A).m;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut dist: libc::c_double = 0.;
    let mut nleaves: libc::c_int = 0;
    let mut nleaves_max: libc::c_int = 10 as libc::c_int;
    let mut angles: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut maxang: libc::c_double = 0.;
    let mut ang1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ang2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut pad: libc::c_double = 0.;
    let mut step: libc::c_double = 0.;
    let mut leaves: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nangles_max: libc::c_int = 10 as libc::c_int;
    let mut nangles: libc::c_int = 0;
    if SparseMatrix_has_diagonal(A) == 0 {} else {
        __assert_fail(
            b"!SparseMatrix_has_diagonal(A)\0" as *const u8 as *const libc::c_char,
            b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
            382 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void beautify_leaves(int, SparseMatrix, double *)\0"))
                .as_ptr(),
        );
    }
    let mut checked: *mut bool = gcalloc(
        ::std::mem::size_of::<bool>() as libc::c_ulong,
        m as size_t,
    ) as *mut bool;
    angles = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nangles_max as libc::c_ulong),
    ) as *mut libc::c_double;
    leaves = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nleaves_max as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        if !(*ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize)
            != 1 as libc::c_int)
        {
            if !*checked.offset(i as isize) {
                p = *ja.offset(*ia.offset(i as isize) as isize);
                if !*checked.offset(p as isize) {
                    *checked.offset(p as isize) = 1 as libc::c_int != 0;
                    dist = 0 as libc::c_int as libc::c_double;
                    nleaves = 0 as libc::c_int;
                    nangles = 0 as libc::c_int;
                    j = *ia.offset(p as isize);
                    while j < *ia.offset((p + 1 as libc::c_int) as isize) {
                        if *ia
                            .offset((*ja.offset(j as isize) + 1 as libc::c_int) as isize)
                            - *ia.offset(*ja.offset(j as isize) as isize)
                            == 1 as libc::c_int
                        {
                            *checked
                                .offset(
                                    *ja.offset(j as isize) as isize,
                                ) = 0 as libc::c_int == 0;
                            check_int_array_size(&mut leaves, nleaves, &mut nleaves_max);
                            dist += distance(x, dim, p, *ja.offset(j as isize));
                            *leaves.offset(nleaves as isize) = *ja.offset(j as isize);
                            nleaves += 1;
                        } else {
                            check_real_array_size(
                                &mut angles,
                                nangles,
                                &mut nangles_max,
                            );
                            let fresh2 = nangles;
                            nangles = nangles + 1;
                            *angles
                                .offset(
                                    fresh2 as isize,
                                ) = get_angle(x, dim, p, *ja.offset(j as isize));
                        }
                        j += 1;
                    }
                    if nleaves > 0 as libc::c_int {} else {
                        __assert_fail(
                            b"nleaves > 0\0" as *const u8 as *const libc::c_char,
                            b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                            408 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 50],
                                &[libc::c_char; 50],
                            >(b"void beautify_leaves(int, SparseMatrix, double *)\0"))
                                .as_ptr(),
                        );
                    }
                    dist /= nleaves as libc::c_double;
                    if nangles > 0 as libc::c_int {
                        sort_real(nangles, angles);
                        maxang = 0 as libc::c_int as libc::c_double;
                        k = 0 as libc::c_int;
                        while k < nangles - 1 as libc::c_int {
                            if *angles.offset((k + 1 as libc::c_int) as isize)
                                - *angles.offset(k as isize) > maxang
                            {
                                maxang = *angles.offset((k + 1 as libc::c_int) as isize)
                                    - *angles.offset(k as isize);
                                ang1 = *angles.offset(k as isize);
                                ang2 = *angles.offset((k + 1 as libc::c_int) as isize);
                            }
                            k += 1;
                        }
                        if 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                            + *angles.offset(0 as libc::c_int as isize)
                            - *angles.offset((nangles - 1 as libc::c_int) as isize)
                            > maxang
                        {
                            maxang = 2 as libc::c_int as libc::c_double
                                * 3.14159265358979323846f64
                                + *angles.offset(0 as libc::c_int as isize)
                                - *angles.offset((nangles - 1 as libc::c_int) as isize);
                            ang1 = *angles.offset((nangles - 1 as libc::c_int) as isize);
                            ang2 = 2 as libc::c_int as libc::c_double
                                * 3.14159265358979323846f64
                                + *angles.offset(0 as libc::c_int as isize);
                        }
                    } else {
                        ang1 = 0 as libc::c_int as libc::c_double;
                        ang2 = 2 as libc::c_int as libc::c_double
                            * 3.14159265358979323846f64;
                        maxang = 2 as libc::c_int as libc::c_double
                            * 3.14159265358979323846f64;
                    }
                    pad = (if maxang
                        - 3.14159265358979323846f64 * 0.166667f64
                            * (nleaves - 1 as libc::c_int) as libc::c_double
                        > 0 as libc::c_int as libc::c_double
                    {
                        maxang
                            - 3.14159265358979323846f64 * 0.166667f64
                                * (nleaves - 1 as libc::c_int) as libc::c_double
                    } else {
                        0 as libc::c_int as libc::c_double
                    }) * 0.5f64;
                    ang1 += pad * 0.95f64;
                    ang2 -= pad * 0.95f64;
                    ang1 = 0 as libc::c_int as libc::c_double;
                    ang2 = 2 as libc::c_int as libc::c_double
                        * 3.14159265358979323846f64;
                    maxang = 2 as libc::c_int as libc::c_double
                        * 3.14159265358979323846f64;
                    if ang2 >= ang1 {} else {
                        __assert_fail(
                            b"ang2 >= ang1\0" as *const u8 as *const libc::c_char,
                            b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                            431 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 50],
                                &[libc::c_char; 50],
                            >(b"void beautify_leaves(int, SparseMatrix, double *)\0"))
                                .as_ptr(),
                        );
                    }
                    step = 0.0f64;
                    if nleaves > 1 as libc::c_int {
                        step = (ang2 - ang1)
                            / (nleaves - 1 as libc::c_int) as libc::c_double;
                    }
                    i = 0 as libc::c_int;
                    while i < nleaves {
                        set_leaves(x, dim, dist, ang1, p, *leaves.offset(i as isize));
                        ang1 += step;
                        i += 1;
                    }
                }
            }
        }
        i += 1;
    }
    free(checked as *mut libc::c_void);
    free(angles as *mut libc::c_void);
    free(leaves as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn force_print(
    mut fp: *mut FILE,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut force: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    fprintf(fp, b"Graphics[{\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n {
        if i > 0 as libc::c_int {
            fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
        }
        fprintf(fp, b"Arrow[{{\0" as *const u8 as *const libc::c_char);
        k = 0 as libc::c_int;
        while k < dim {
            if k > 0 as libc::c_int {
                fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                fp,
                b"%f\0" as *const u8 as *const libc::c_char,
                *x.offset((i * dim + k) as isize),
            );
            k += 1;
        }
        fprintf(fp, b"},{\0" as *const u8 as *const libc::c_char);
        k = 0 as libc::c_int;
        while k < dim {
            if k > 0 as libc::c_int {
                fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                fp,
                b"%f\0" as *const u8 as *const libc::c_char,
                *x.offset((i * dim + k) as isize)
                    + 0.5f64 * *force.offset((i * dim + k) as isize),
            );
            k += 1;
        }
        fprintf(fp, b"}}]\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n {
        if i > 0 as libc::c_int {
            fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
        }
        fprintf(fp, b"Tooltip[Point[{\0" as *const u8 as *const libc::c_char);
        k = 0 as libc::c_int;
        while k < dim {
            if k > 0 as libc::c_int {
                fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                fp,
                b"%f\0" as *const u8 as *const libc::c_char,
                *x.offset((i * dim + k) as isize),
            );
            k += 1;
        }
        fprintf(fp, b"}],%d]\0" as *const u8 as *const libc::c_char, i);
        i += 1;
    }
    fprintf(fp, b"}]\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn spring_electrical_embedding_fast(
    mut dim: libc::c_int,
    mut A0: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut x: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut A: SparseMatrix = A0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_double = (*ctrl).p;
    let mut K: libc::c_double = (*ctrl).K;
    let mut C: libc::c_double = (*ctrl).C;
    let mut CRK: libc::c_double = 0.;
    let mut tol: libc::c_double = (*ctrl).tol;
    let mut maxiter: libc::c_double = (*ctrl).maxiter as libc::c_double;
    let mut cool: libc::c_double = (*ctrl).cool;
    let mut step: libc::c_double = (*ctrl).step;
    let mut KP: libc::c_double = 0.;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xold: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut F: libc::c_double = 0.;
    let mut Fnorm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut Fnorm0: libc::c_double = 0.;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut adaptive_cooling: libc::c_int = (*ctrl).adaptive_cooling;
    let mut qt: QuadTree = 0 as QuadTree;
    let mut counts: [libc::c_double; 4] = [0.; 4];
    let mut force: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut max_qtree_level: libc::c_int = (*ctrl).max_qtree_level;
    let mut qtree_level_optimizer: oned_optimizer = 0 as oned_optimizer;
    if A.is_null() || maxiter <= 0 as libc::c_int as libc::c_double {
        return;
    }
    m = (*A).m;
    n = (*A).n;
    if n <= 0 as libc::c_int || dim <= 0 as libc::c_int {
        return;
    }
    qtree_level_optimizer = oned_optimizer_new(max_qtree_level);
    *flag = 0 as libc::c_int;
    if m != n {
        *flag = ERROR_NOT_SQUARE_MATRIX as libc::c_int;
    } else {
        if (*A).format == FORMAT_CSR as libc::c_int {} else {
            __assert_fail(
                b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                518 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"void spring_electrical_embedding_fast(int, SparseMatrix, spring_electrical_control, double *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        A = SparseMatrix_symmetrize(A, 1 as libc::c_int != 0);
        ia = (*A).ia;
        ja = (*A).ja;
        if (*ctrl).random_start != 0 {
            srand((*ctrl).random_seed as libc::c_uint);
            i = 0 as libc::c_int;
            while i < dim * n {
                *x.offset(i as isize) = drand();
                i += 1;
            }
        }
        if K < 0 as libc::c_int as libc::c_double {
            K = average_edge_length(A, dim, x);
            (*ctrl).K = K;
        }
        if C < 0 as libc::c_int as libc::c_double {
            C = 0.2f64;
            (*ctrl).C = C;
        }
        if p >= 0 as libc::c_int as libc::c_double {
            p = -(1 as libc::c_int) as libc::c_double;
            (*ctrl).p = p;
        }
        KP = pow(K, 1 as libc::c_int as libc::c_double - p);
        CRK = pow(C, (2.0f64 - p) / 3.0f64) / K;
        xold = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
        force = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
        loop {
            iter += 1;
            memcpy(
                xold as *mut libc::c_void,
                x as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(dim as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
            Fnorm0 = Fnorm;
            Fnorm = 0.0f64;
            max_qtree_level = oned_optimizer_get(qtree_level_optimizer);
            qt = QuadTree_new_from_point_list(dim, n, max_qtree_level, x);
            QuadTree_get_repulsive_force(
                qt,
                force,
                x,
                (*ctrl).bh,
                p,
                KP,
                counts.as_mut_ptr(),
                flag,
            );
            if *flag == 0 {} else {
                __assert_fail(
                    b"!(*flag)\0" as *const u8 as *const libc::c_char,
                    b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                    566 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"void spring_electrical_embedding_fast(int, SparseMatrix, spring_electrical_control, double *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            i = 0 as libc::c_int;
            while i < n {
                f = &mut *force.offset((i * dim) as isize) as *mut libc::c_double;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(*ja.offset(j as isize) == i) {
                        dist = distance(x, dim, i, *ja.offset(j as isize));
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                -= CRK
                                    * (*x.offset((i * dim + k) as isize)
                                        - *x.offset((*ja.offset(j as isize) * dim + k) as isize))
                                    * dist;
                            k += 1;
                        }
                    }
                    j += 1;
                }
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < n {
                f = &mut *force.offset((i * dim) as isize) as *mut libc::c_double;
                F = 0.0f64;
                k = 0 as libc::c_int;
                while k < dim {
                    F += *f.offset(k as isize) * *f.offset(k as isize);
                    k += 1;
                }
                F = sqrt(F);
                Fnorm += F;
                if F > 0 as libc::c_int as libc::c_double {
                    k = 0 as libc::c_int;
                    while k < dim {
                        *f.offset(k as isize) /= F;
                        k += 1;
                    }
                }
                k = 0 as libc::c_int;
                while k < dim {
                    *x.offset((i * dim + k) as isize) += step * *f.offset(k as isize);
                    k += 1;
                }
                i += 1;
            }
            if !qt.is_null() {
                QuadTree_delete(qt);
                oned_optimizer_train(
                    qtree_level_optimizer,
                    counts[0 as libc::c_int as usize]
                        + 0.85f64 * counts[1 as libc::c_int as usize]
                        + 3.3f64 * counts[2 as libc::c_int as usize],
                );
            } else if Verbose != 0 {
                fprintf(
                    stderr,
                    b"\r                iter = %d, step = %f Fnorm = %f nz = %d  K = %f                                  \0"
                        as *const u8 as *const libc::c_char,
                    iter,
                    step,
                    Fnorm,
                    (*A).nz,
                    K,
                );
            }
            step = update_step(adaptive_cooling, step, Fnorm, Fnorm0, cool);
            if !(step > tol && (iter as libc::c_double) < maxiter) {
                break;
            }
        }
        if (*ctrl).beautify_leaves != 0 {
            beautify_leaves(dim, A, x);
        }
    }
    oned_optimizer_delete(qtree_level_optimizer);
    (*ctrl).max_qtree_level = max_qtree_level;
    free(xold as *mut libc::c_void);
    if A != A0 {
        SparseMatrix_delete(A);
    }
    free(force as *mut libc::c_void);
}
unsafe extern "C" fn spring_electrical_embedding_slow(
    mut dim: libc::c_int,
    mut A0: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut x: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut A: SparseMatrix = A0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_double = (*ctrl).p;
    let mut K: libc::c_double = (*ctrl).K;
    let mut C: libc::c_double = (*ctrl).C;
    let mut CRK: libc::c_double = 0.;
    let mut tol: libc::c_double = (*ctrl).tol;
    let mut maxiter: libc::c_double = (*ctrl).maxiter as libc::c_double;
    let mut cool: libc::c_double = (*ctrl).cool;
    let mut step: libc::c_double = (*ctrl).step;
    let mut KP: libc::c_double = 0.;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xold: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut F: libc::c_double = 0.;
    let mut Fnorm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut Fnorm0: libc::c_double = 0.;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut adaptive_cooling: libc::c_int = (*ctrl).adaptive_cooling;
    let mut qt: QuadTree = 0 as QuadTree;
    let mut USE_QT: libc::c_int = 0 as libc::c_int;
    let mut nsuper: libc::c_int = 0 as libc::c_int;
    let mut nsupermax: libc::c_int = 10 as libc::c_int;
    let mut center: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut supernode_wgts: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut distances: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nsuper_avg: libc::c_double = 0.;
    let mut counts: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut counts_avg: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut force: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut max_qtree_level: libc::c_int = (*ctrl).max_qtree_level;
    let mut qtree_level_optimizer: oned_optimizer = 0 as oned_optimizer;
    fprintf(
        stderr,
        b"spring_electrical_embedding_slow\0" as *const u8 as *const libc::c_char,
    );
    if A.is_null() || maxiter <= 0 as libc::c_int as libc::c_double {
        return;
    }
    m = (*A).m;
    n = (*A).n;
    if n <= 0 as libc::c_int || dim <= 0 as libc::c_int {
        return;
    }
    force = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    ) as *mut libc::c_double;
    if n >= (*ctrl).quadtree_size {
        USE_QT = (0 as libc::c_int == 0) as libc::c_int;
        qtree_level_optimizer = oned_optimizer_new(max_qtree_level);
        center = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        supernode_wgts = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
        distances = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    USE_QT = 0 as libc::c_int;
    *flag = 0 as libc::c_int;
    if m != n {
        *flag = ERROR_NOT_SQUARE_MATRIX as libc::c_int;
    } else {
        if (*A).format == FORMAT_CSR as libc::c_int {} else {
            __assert_fail(
                b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                708 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 101],
                    &[libc::c_char; 101],
                >(
                    b"void spring_electrical_embedding_slow(int, SparseMatrix, spring_electrical_control, double *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        A = SparseMatrix_symmetrize(A, 1 as libc::c_int != 0);
        ia = (*A).ia;
        ja = (*A).ja;
        if (*ctrl).random_start != 0 {
            srand((*ctrl).random_seed as libc::c_uint);
            i = 0 as libc::c_int;
            while i < dim * n {
                *x.offset(i as isize) = drand();
                i += 1;
            }
        }
        if K < 0 as libc::c_int as libc::c_double {
            K = average_edge_length(A, dim, x);
            (*ctrl).K = K;
        }
        if C < 0 as libc::c_int as libc::c_double {
            C = 0.2f64;
            (*ctrl).C = C;
        }
        if p >= 0 as libc::c_int as libc::c_double {
            p = -(1 as libc::c_int) as libc::c_double;
            (*ctrl).p = p;
        }
        KP = pow(K, 1 as libc::c_int as libc::c_double - p);
        CRK = pow(C, (2.0f64 - p) / 3.0f64) / K;
        f = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        xold = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
        's_225: loop {
            i = 0 as libc::c_int;
            while i < dim * n {
                *force.offset(i as isize) = 0 as libc::c_int as libc::c_double;
                i += 1;
            }
            iter += 1;
            memcpy(
                xold as *mut libc::c_void,
                x as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(dim as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
            Fnorm0 = Fnorm;
            Fnorm = 0.0f64;
            nsuper_avg = 0 as libc::c_int as libc::c_double;
            if USE_QT != 0 {
                max_qtree_level = oned_optimizer_get(qtree_level_optimizer);
                qt = QuadTree_new_from_point_list(dim, n, max_qtree_level, x);
            }
            i = 0 as libc::c_int;
            while i < n {
                k = 0 as libc::c_int;
                while k < dim {
                    *f.offset(k as isize) = 0.0f64;
                    k += 1;
                }
                if USE_QT != 0 {
                    QuadTree_get_supernodes(
                        qt,
                        (*ctrl).bh,
                        &mut *x.offset((dim * i) as isize),
                        i,
                        &mut nsuper,
                        &mut nsupermax,
                        &mut center,
                        &mut supernode_wgts,
                        &mut distances,
                        &mut counts,
                        flag,
                    );
                    counts_avg += counts;
                    nsuper_avg += nsuper as libc::c_double;
                    if *flag != 0 {
                        current_block = 12780144205826894671;
                        break 's_225;
                    }
                    j = 0 as libc::c_int;
                    while j < nsuper {
                        dist = if *distances.offset(j as isize) > 1.0e-15f64 {
                            *distances.offset(j as isize)
                        } else {
                            1.0e-15f64
                        };
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                += *supernode_wgts.offset(j as isize) * KP
                                    * (*x.offset((i * dim + k) as isize)
                                        - *center.offset((j * dim + k) as isize))
                                    / pow(dist, 1.0f64 - p);
                            k += 1;
                        }
                        j += 1;
                    }
                } else {
                    j = 0 as libc::c_int;
                    while j < n {
                        if !(j == i) {
                            dist = distance_cropped(x, dim, i, j);
                            k = 0 as libc::c_int;
                            while k < dim {
                                *f.offset(k as isize)
                                    += KP
                                        * (*x.offset((i * dim + k) as isize)
                                            - *x.offset((j * dim + k) as isize))
                                        / pow(dist, 1.0f64 - p);
                                k += 1;
                            }
                        }
                        j += 1;
                    }
                }
                k = 0 as libc::c_int;
                while k < dim {
                    *force.offset((i * dim + k) as isize) += *f.offset(k as isize);
                    k += 1;
                }
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < n {
                k = 0 as libc::c_int;
                while k < dim {
                    *f.offset(k as isize) = 0.0f64;
                    k += 1;
                }
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(*ja.offset(j as isize) == i) {
                        dist = distance(x, dim, i, *ja.offset(j as isize));
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                -= CRK
                                    * (*x.offset((i * dim + k) as isize)
                                        - *x.offset((*ja.offset(j as isize) * dim + k) as isize))
                                    * dist;
                            k += 1;
                        }
                    }
                    j += 1;
                }
                k = 0 as libc::c_int;
                while k < dim {
                    *force.offset((i * dim + k) as isize) += *f.offset(k as isize);
                    k += 1;
                }
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < n {
                k = 0 as libc::c_int;
                while k < dim {
                    *f.offset(k as isize) = *force.offset((i * dim + k) as isize);
                    k += 1;
                }
                F = 0.0f64;
                k = 0 as libc::c_int;
                while k < dim {
                    F += *f.offset(k as isize) * *f.offset(k as isize);
                    k += 1;
                }
                F = sqrt(F);
                Fnorm += F;
                if F > 0 as libc::c_int as libc::c_double {
                    k = 0 as libc::c_int;
                    while k < dim {
                        *f.offset(k as isize) /= F;
                        k += 1;
                    }
                }
                k = 0 as libc::c_int;
                while k < dim {
                    *x.offset((i * dim + k) as isize) += step * *f.offset(k as isize);
                    k += 1;
                }
                i += 1;
            }
            if !qt.is_null() {
                QuadTree_delete(qt);
                nsuper_avg /= n as libc::c_double;
                counts_avg /= n as libc::c_double;
                if Verbose as libc::c_int != 0 && 0 as libc::c_int != 0 {
                    fprintf(
                        stderr,
                        b"nsuper_avg=%f, counts_avg = %f 2*nsuper+counts=%f\n\0"
                            as *const u8 as *const libc::c_char,
                        nsuper_avg,
                        counts_avg,
                        2 as libc::c_int as libc::c_double * nsuper_avg + counts_avg,
                    );
                }
                oned_optimizer_train(
                    qtree_level_optimizer,
                    5 as libc::c_int as libc::c_double * nsuper_avg + counts_avg,
                );
            }
            step = update_step(adaptive_cooling, step, Fnorm, Fnorm0, cool);
            if !(step > tol && (iter as libc::c_double) < maxiter) {
                current_block = 17294538769010429813;
                break;
            }
        }
        match current_block {
            12780144205826894671 => {}
            _ => {
                if (*ctrl).beautify_leaves != 0 {
                    beautify_leaves(dim, A, x);
                }
            }
        }
    }
    if USE_QT != 0 {
        oned_optimizer_delete(qtree_level_optimizer);
        (*ctrl).max_qtree_level = max_qtree_level;
    }
    free(xold as *mut libc::c_void);
    if A != A0 {
        SparseMatrix_delete(A);
    }
    free(f as *mut libc::c_void);
    free(center as *mut libc::c_void);
    free(supernode_wgts as *mut libc::c_void);
    free(distances as *mut libc::c_void);
    free(force as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spring_electrical_embedding(
    mut dim: libc::c_int,
    mut A0: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut x: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut A: SparseMatrix = A0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_double = (*ctrl).p;
    let mut K: libc::c_double = (*ctrl).K;
    let mut C: libc::c_double = (*ctrl).C;
    let mut CRK: libc::c_double = 0.;
    let mut tol: libc::c_double = (*ctrl).tol;
    let mut maxiter: libc::c_double = (*ctrl).maxiter as libc::c_double;
    let mut cool: libc::c_double = (*ctrl).cool;
    let mut step: libc::c_double = (*ctrl).step;
    let mut KP: libc::c_double = 0.;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xold: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut F: libc::c_double = 0.;
    let mut Fnorm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut Fnorm0: libc::c_double = 0.;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut adaptive_cooling: libc::c_int = (*ctrl).adaptive_cooling;
    let mut qt: QuadTree = 0 as QuadTree;
    let mut USE_QT: libc::c_int = 0 as libc::c_int;
    let mut nsuper: libc::c_int = 0 as libc::c_int;
    let mut nsupermax: libc::c_int = 10 as libc::c_int;
    let mut center: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut supernode_wgts: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut distances: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nsuper_avg: libc::c_double = 0.;
    let mut counts: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut counts_avg: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut max_qtree_level: libc::c_int = (*ctrl).max_qtree_level;
    let mut qtree_level_optimizer: oned_optimizer = 0 as oned_optimizer;
    if A.is_null() || maxiter <= 0 as libc::c_int as libc::c_double {
        return;
    }
    m = (*A).m;
    n = (*A).n;
    if n <= 0 as libc::c_int || dim <= 0 as libc::c_int {
        return;
    }
    if n >= (*ctrl).quadtree_size {
        USE_QT = (0 as libc::c_int == 0) as libc::c_int;
        qtree_level_optimizer = oned_optimizer_new(max_qtree_level);
        center = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        supernode_wgts = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
        distances = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    *flag = 0 as libc::c_int;
    if m != n {
        *flag = ERROR_NOT_SQUARE_MATRIX as libc::c_int;
    } else {
        if (*A).format == FORMAT_CSR as libc::c_int {} else {
            __assert_fail(
                b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                938 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void spring_electrical_embedding(int, SparseMatrix, spring_electrical_control, double *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        A = SparseMatrix_symmetrize(A, 1 as libc::c_int != 0);
        ia = (*A).ia;
        ja = (*A).ja;
        if (*ctrl).random_start != 0 {
            srand((*ctrl).random_seed as libc::c_uint);
            i = 0 as libc::c_int;
            while i < dim * n {
                *x.offset(i as isize) = drand();
                i += 1;
            }
        }
        if K < 0 as libc::c_int as libc::c_double {
            K = average_edge_length(A, dim, x);
            (*ctrl).K = K;
        }
        if C < 0 as libc::c_int as libc::c_double {
            C = 0.2f64;
            (*ctrl).C = C;
        }
        if p >= 0 as libc::c_int as libc::c_double {
            p = -(1 as libc::c_int) as libc::c_double;
            (*ctrl).p = p;
        }
        KP = pow(K, 1 as libc::c_int as libc::c_double - p);
        CRK = pow(C, (2.0f64 - p) / 3.0f64) / K;
        f = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        xold = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
        's_210: loop {
            iter += 1;
            memcpy(
                xold as *mut libc::c_void,
                x as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(dim as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
            Fnorm0 = Fnorm;
            Fnorm = 0.0f64;
            nsuper_avg = 0 as libc::c_int as libc::c_double;
            counts_avg = 0 as libc::c_int as libc::c_double;
            if USE_QT != 0 {
                max_qtree_level = oned_optimizer_get(qtree_level_optimizer);
                qt = QuadTree_new_from_point_list(dim, n, max_qtree_level, x);
            }
            i = 0 as libc::c_int;
            while i < n {
                k = 0 as libc::c_int;
                while k < dim {
                    *f.offset(k as isize) = 0.0f64;
                    k += 1;
                }
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(*ja.offset(j as isize) == i) {
                        dist = distance(x, dim, i, *ja.offset(j as isize));
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                -= CRK
                                    * (*x.offset((i * dim + k) as isize)
                                        - *x.offset((*ja.offset(j as isize) * dim + k) as isize))
                                    * dist;
                            k += 1;
                        }
                    }
                    j += 1;
                }
                if USE_QT != 0 {
                    QuadTree_get_supernodes(
                        qt,
                        (*ctrl).bh,
                        &mut *x.offset((dim * i) as isize),
                        i,
                        &mut nsuper,
                        &mut nsupermax,
                        &mut center,
                        &mut supernode_wgts,
                        &mut distances,
                        &mut counts,
                        flag,
                    );
                    counts_avg += counts;
                    nsuper_avg += nsuper as libc::c_double;
                    if *flag != 0 {
                        current_block = 2494009981989147942;
                        break 's_210;
                    }
                    j = 0 as libc::c_int;
                    while j < nsuper {
                        dist = if *distances.offset(j as isize) > 1.0e-15f64 {
                            *distances.offset(j as isize)
                        } else {
                            1.0e-15f64
                        };
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                += *supernode_wgts.offset(j as isize) * KP
                                    * (*x.offset((i * dim + k) as isize)
                                        - *center.offset((j * dim + k) as isize))
                                    / pow(dist, 1.0f64 - p);
                            k += 1;
                        }
                        j += 1;
                    }
                } else {
                    j = 0 as libc::c_int;
                    while j < n {
                        if !(j == i) {
                            dist = distance_cropped(x, dim, i, j);
                            k = 0 as libc::c_int;
                            while k < dim {
                                *f.offset(k as isize)
                                    += KP
                                        * (*x.offset((i * dim + k) as isize)
                                            - *x.offset((j * dim + k) as isize))
                                        / pow(dist, 1.0f64 - p);
                                k += 1;
                            }
                        }
                        j += 1;
                    }
                }
                F = 0.0f64;
                k = 0 as libc::c_int;
                while k < dim {
                    F += *f.offset(k as isize) * *f.offset(k as isize);
                    k += 1;
                }
                F = sqrt(F);
                Fnorm += F;
                if F > 0 as libc::c_int as libc::c_double {
                    k = 0 as libc::c_int;
                    while k < dim {
                        *f.offset(k as isize) /= F;
                        k += 1;
                    }
                }
                k = 0 as libc::c_int;
                while k < dim {
                    *x.offset((i * dim + k) as isize) += step * *f.offset(k as isize);
                    k += 1;
                }
                i += 1;
            }
            if !qt.is_null() {
                QuadTree_delete(qt);
                nsuper_avg /= n as libc::c_double;
                counts_avg /= n as libc::c_double;
                if Verbose as libc::c_int & 0 as libc::c_int != 0 {
                    fprintf(
                        stderr,
                        b"nsuper_avg=%f, counts_avg = %f 2*nsuper+counts=%f\n\0"
                            as *const u8 as *const libc::c_char,
                        nsuper_avg,
                        counts_avg,
                        2 as libc::c_int as libc::c_double * nsuper_avg + counts_avg,
                    );
                }
                oned_optimizer_train(
                    qtree_level_optimizer,
                    5 as libc::c_int as libc::c_double * nsuper_avg + counts_avg,
                );
            }
            step = update_step(adaptive_cooling, step, Fnorm, Fnorm0, cool);
            if !(step > tol && (iter as libc::c_double) < maxiter) {
                current_block = 3575278370434307847;
                break;
            }
        }
        match current_block {
            2494009981989147942 => {}
            _ => {
                if (*ctrl).beautify_leaves != 0 {
                    beautify_leaves(dim, A, x);
                }
            }
        }
    }
    if USE_QT != 0 {
        oned_optimizer_delete(qtree_level_optimizer);
        (*ctrl).max_qtree_level = max_qtree_level;
    }
    free(xold as *mut libc::c_void);
    if A != A0 {
        SparseMatrix_delete(A);
    }
    free(f as *mut libc::c_void);
    free(center as *mut libc::c_void);
    free(supernode_wgts as *mut libc::c_void);
    free(distances as *mut libc::c_void);
}
unsafe extern "C" fn scale_coord(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut id: *mut libc::c_int,
    mut jd: *mut libc::c_int,
    mut d: *mut libc::c_double,
    mut dj: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut w_ij: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut s: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut stop: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sbot: libc::c_double = 0.0f64;
    let mut nz: libc::c_double = 0 as libc::c_int as libc::c_double;
    if dj == 0.0f64 {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        j = *id.offset(i as isize);
        while j < *id.offset((i + 1 as libc::c_int) as isize) {
            if !(*jd.offset(j as isize) == i) {
                dist = distance_cropped(x, dim, i, *jd.offset(j as isize));
                if !d.is_null() {
                    dj = *d.offset(j as isize);
                }
                if dj > 0 as libc::c_int as libc::c_double {} else {
                    __assert_fail(
                        b"dj > 0\0" as *const u8 as *const libc::c_char,
                        b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                        1139 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 69],
                            &[libc::c_char; 69],
                        >(
                            b"void scale_coord(int, int, double *, int *, int *, double *, double)\0",
                        ))
                            .as_ptr(),
                    );
                }
                w_ij = 1.0f64 / (dj * dj);
                k = 0 as libc::c_int;
                while k < dim {
                    stop += w_ij * dj * dist;
                    sbot += w_ij * dist * dist;
                    k += 1;
                }
                s += dist;
                nz += 1.;
            }
            j += 1;
        }
        i += 1;
    }
    s = stop / sbot;
    i = 0 as libc::c_int;
    while i < n * dim {
        *x.offset(i as isize) *= s;
        i += 1;
    }
    fprintf(stderr, b"scaling factor = %f\n\0" as *const u8 as *const libc::c_char, s);
}
unsafe extern "C" fn dmean_get(
    mut n: libc::c_int,
    mut id: *mut libc::c_int,
    mut d: *mut libc::c_double,
) -> libc::c_double {
    let mut dmean: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if d.is_null() {
        return 1.0f64;
    }
    i = 0 as libc::c_int;
    while i < n {
        j = *id.offset(i as isize);
        while j < *id.offset((i + 1 as libc::c_int) as isize) {
            dmean += *d.offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    return dmean / *id.offset(n as isize) as libc::c_double;
}
unsafe extern "C" fn spring_maxent_embedding(
    mut dim: libc::c_int,
    mut A0: SparseMatrix,
    mut D: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut x: *mut libc::c_double,
    mut rho: libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut A: SparseMatrix = A0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_double = (*ctrl).p;
    let mut C: libc::c_double = (*ctrl).C;
    let mut tol: libc::c_double = (*ctrl).tol;
    let mut maxiter: libc::c_double = (*ctrl).maxiter as libc::c_double;
    let mut cool: libc::c_double = (*ctrl).cool;
    let mut step: libc::c_double = (*ctrl).step;
    let mut w_ij: libc::c_double = 0.;
    let mut dj: libc::c_double = 1.0f64;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dmean: libc::c_double = 0.;
    let mut xold: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut F: libc::c_double = 0.;
    let mut Fnorm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut Fnorm0: libc::c_double = 0.;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut adaptive_cooling: libc::c_int = (*ctrl).adaptive_cooling;
    let mut qt: QuadTree = 0 as QuadTree;
    let mut USE_QT: libc::c_int = 0 as libc::c_int;
    let mut nsuper: libc::c_int = 0 as libc::c_int;
    let mut nsupermax: libc::c_int = 10 as libc::c_int;
    let mut center: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut supernode_wgts: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut distances: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nsuper_avg: libc::c_double = 0.;
    let mut counts: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut max_qtree_level: libc::c_int = 10 as libc::c_int;
    if A.is_null() || maxiter <= 0 as libc::c_int as libc::c_double {
        return;
    }
    m = (*A).m;
    n = (*A).n;
    if n <= 0 as libc::c_int || dim <= 0 as libc::c_int {
        return;
    }
    if (*ctrl).tscheme != QUAD_TREE_NONE as libc::c_int && n >= (*ctrl).quadtree_size {
        USE_QT = (0 as libc::c_int == 0) as libc::c_int;
        center = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        supernode_wgts = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
        distances = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    *flag = 0 as libc::c_int;
    if m != n {
        *flag = ERROR_NOT_SQUARE_MATRIX as libc::c_int;
    } else {
        if (*A).format == FORMAT_CSR as libc::c_int {} else {
            __assert_fail(
                b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                1225 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 114],
                    &[libc::c_char; 114],
                >(
                    b"void spring_maxent_embedding(int, SparseMatrix, SparseMatrix, spring_electrical_control, double *, double, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        A = SparseMatrix_symmetrize(A, 1 as libc::c_int != 0);
        ia = (*A).ia;
        ja = (*A).ja;
        if !D.is_null() {
            id = (*D).ia;
            jd = (*D).ja;
            d = (*D).a as *mut libc::c_double;
        } else {
            id = ia;
            jd = ja;
            d = 0 as *mut libc::c_double;
        }
        if rho < 0 as libc::c_int as libc::c_double {
            dmean = dmean_get(n, id, d);
            rho = rho
                * (*id.offset(n as isize) as libc::c_double
                    / (n as libc::c_double * n as libc::c_double
                        - *id.offset(n as isize) as libc::c_double))
                / pow(dmean, p + 1 as libc::c_int as libc::c_double);
            fprintf(
                stderr,
                b"dmean = %f, rho = %f\n\0" as *const u8 as *const libc::c_char,
                dmean,
                rho,
            );
        }
        if (*ctrl).random_start != 0 {
            fprintf(
                stderr,
                b"send random coordinates\n\0" as *const u8 as *const libc::c_char,
            );
            srand((*ctrl).random_seed as libc::c_uint);
            i = 0 as libc::c_int;
            while i < dim * n {
                *x.offset(i as isize) = drand();
                i += 1;
            }
        }
        scale_coord(n, dim, x, id, jd, d, dj);
        if C < 0 as libc::c_int as libc::c_double {
            C = 0.2f64;
            (*ctrl).C = C;
        }
        if p >= 0 as libc::c_int as libc::c_double {
            p = -(1 as libc::c_int) as libc::c_double;
            (*ctrl).p = p;
        }
        f = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        xold = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
        's_256: loop {
            iter += 1;
            memcpy(
                xold as *mut libc::c_void,
                x as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(dim as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
            Fnorm0 = Fnorm;
            Fnorm = 0.0f64;
            nsuper_avg = 0 as libc::c_int as libc::c_double;
            if USE_QT != 0 {
                qt = QuadTree_new_from_point_list(dim, n, max_qtree_level, x);
            }
            i = 0 as libc::c_int;
            while i < n {
                k = 0 as libc::c_int;
                while k < dim {
                    *f.offset(k as isize) = 0.0f64;
                    k += 1;
                }
                j = *id.offset(i as isize);
                while j < *id.offset((i + 1 as libc::c_int) as isize) {
                    if !(*jd.offset(j as isize) == i) {
                        dist = distance_cropped(x, dim, i, *jd.offset(j as isize));
                        if !d.is_null() {
                            dj = *d.offset(j as isize);
                        }
                        if dj > 0 as libc::c_int as libc::c_double {} else {
                            __assert_fail(
                                b"dj > 0\0" as *const u8 as *const libc::c_char,
                                b"spring_electrical.c\0" as *const u8
                                    as *const libc::c_char,
                                1303 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 114],
                                    &[libc::c_char; 114],
                                >(
                                    b"void spring_maxent_embedding(int, SparseMatrix, SparseMatrix, spring_electrical_control, double *, double, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        w_ij = 1.0f64
                            / pow(dj, (*ctrl).q + 1 as libc::c_int as libc::c_double);
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                += -w_ij
                                    * (*x.offset((i * dim + k) as isize)
                                        - *x.offset((*jd.offset(j as isize) * dim + k) as isize))
                                    * pow(dist - dj, (*ctrl).q) / dist;
                            k += 1;
                        }
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                -= rho
                                    * (*x.offset((i * dim + k) as isize)
                                        - *x.offset((*jd.offset(j as isize) * dim + k) as isize))
                                    / pow(dist, 1.0f64 - p);
                            k += 1;
                        }
                    }
                    j += 1;
                }
                if USE_QT != 0 {
                    QuadTree_get_supernodes(
                        qt,
                        (*ctrl).bh,
                        &mut *x.offset((dim * i) as isize),
                        i,
                        &mut nsuper,
                        &mut nsupermax,
                        &mut center,
                        &mut supernode_wgts,
                        &mut distances,
                        &mut counts,
                        flag,
                    );
                    nsuper_avg += nsuper as libc::c_double;
                    if *flag != 0 {
                        current_block = 15714839002739240143;
                        break 's_256;
                    }
                    j = 0 as libc::c_int;
                    while j < nsuper {
                        dist = if *distances.offset(j as isize) > 1.0e-15f64 {
                            *distances.offset(j as isize)
                        } else {
                            1.0e-15f64
                        };
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                += rho * *supernode_wgts.offset(j as isize)
                                    * (*x.offset((i * dim + k) as isize)
                                        - *center.offset((j * dim + k) as isize))
                                    / pow(dist, 1.0f64 - p);
                            k += 1;
                        }
                        j += 1;
                    }
                } else {
                    j = 0 as libc::c_int;
                    while j < n {
                        if !(j == i) {
                            dist = distance_cropped(x, dim, i, j);
                            k = 0 as libc::c_int;
                            while k < dim {
                                *f.offset(k as isize)
                                    += rho
                                        * (*x.offset((i * dim + k) as isize)
                                            - *x.offset((j * dim + k) as isize))
                                        / pow(dist, 1.0f64 - p);
                                k += 1;
                            }
                        }
                        j += 1;
                    }
                }
                F = 0.0f64;
                k = 0 as libc::c_int;
                while k < dim {
                    F += *f.offset(k as isize) * *f.offset(k as isize);
                    k += 1;
                }
                F = sqrt(F);
                Fnorm += F;
                if F > 0 as libc::c_int as libc::c_double {
                    k = 0 as libc::c_int;
                    while k < dim {
                        *f.offset(k as isize) /= F;
                        k += 1;
                    }
                }
                k = 0 as libc::c_int;
                while k < dim {
                    *x.offset((i * dim + k) as isize) += step * *f.offset(k as isize);
                    k += 1;
                }
                i += 1;
            }
            if !qt.is_null() {
                QuadTree_delete(qt);
            }
            nsuper_avg /= n as libc::c_double;
            step = update_step(adaptive_cooling, step, Fnorm, Fnorm0, cool);
            if !(step > tol && (iter as libc::c_double) < maxiter) {
                current_block = 10248984122780841972;
                break;
            }
        }
        match current_block {
            15714839002739240143 => {}
            _ => {
                if (*ctrl).beautify_leaves != 0 {
                    beautify_leaves(dim, A, x);
                }
            }
        }
    }
    free(xold as *mut libc::c_void);
    if A != A0 {
        SparseMatrix_delete(A);
    }
    free(f as *mut libc::c_void);
    free(center as *mut libc::c_void);
    free(supernode_wgts as *mut libc::c_void);
    free(distances as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spring_electrical_spring_embedding(
    mut dim: libc::c_int,
    mut A0: SparseMatrix,
    mut D: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut x: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut A: SparseMatrix = A0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_double = (*ctrl).p;
    let mut K: libc::c_double = (*ctrl).K;
    let mut C: libc::c_double = (*ctrl).C;
    let mut CRK: libc::c_double = 0.;
    let mut tol: libc::c_double = (*ctrl).tol;
    let mut maxiter: libc::c_double = (*ctrl).maxiter as libc::c_double;
    let mut cool: libc::c_double = (*ctrl).cool;
    let mut step: libc::c_double = (*ctrl).step;
    let mut KP: libc::c_double = 0.;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xold: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut F: libc::c_double = 0.;
    let mut Fnorm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut Fnorm0: libc::c_double = 0.;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut adaptive_cooling: libc::c_int = (*ctrl).adaptive_cooling;
    let mut qt: QuadTree = 0 as QuadTree;
    let mut USE_QT: libc::c_int = 0 as libc::c_int;
    let mut nsuper: libc::c_int = 0 as libc::c_int;
    let mut nsupermax: libc::c_int = 10 as libc::c_int;
    let mut center: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut supernode_wgts: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut distances: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nsuper_avg: libc::c_double = 0.;
    let mut counts: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut max_qtree_level: libc::c_int = 10 as libc::c_int;
    if A.is_null() || maxiter <= 0 as libc::c_int as libc::c_double {
        return;
    }
    m = (*A).m;
    n = (*A).n;
    if n <= 0 as libc::c_int || dim <= 0 as libc::c_int {
        return;
    }
    if n >= (*ctrl).quadtree_size {
        USE_QT = (0 as libc::c_int == 0) as libc::c_int;
        center = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        supernode_wgts = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
        distances = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    *flag = 0 as libc::c_int;
    if m != n {
        *flag = ERROR_NOT_SQUARE_MATRIX as libc::c_int;
    } else {
        if (*A).format == FORMAT_CSR as libc::c_int {} else {
            __assert_fail(
                b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                1428 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 117],
                    &[libc::c_char; 117],
                >(
                    b"void spring_electrical_spring_embedding(int, SparseMatrix, SparseMatrix, spring_electrical_control, double *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        A = SparseMatrix_symmetrize(A, 1 as libc::c_int != 0);
        ia = (*A).ia;
        ja = (*A).ja;
        id = (*D).ia;
        jd = (*D).ja;
        d = (*D).a as *mut libc::c_double;
        if (*ctrl).random_start != 0 {
            srand((*ctrl).random_seed as libc::c_uint);
            i = 0 as libc::c_int;
            while i < dim * n {
                *x.offset(i as isize) = drand();
                i += 1;
            }
        }
        if K < 0 as libc::c_int as libc::c_double {
            K = average_edge_length(A, dim, x);
            (*ctrl).K = K;
        }
        if C < 0 as libc::c_int as libc::c_double {
            C = 0.2f64;
            (*ctrl).C = C;
        }
        if p >= 0 as libc::c_int as libc::c_double {
            p = -(1 as libc::c_int) as libc::c_double;
            (*ctrl).p = p;
        }
        KP = pow(K, 1 as libc::c_int as libc::c_double - p);
        CRK = pow(C, (2.0f64 - p) / 3.0f64) / K;
        f = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        xold = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
        's_221: loop {
            iter += 1;
            memcpy(
                xold as *mut libc::c_void,
                x as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(dim as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
            Fnorm0 = Fnorm;
            Fnorm = 0.0f64;
            nsuper_avg = 0 as libc::c_int as libc::c_double;
            if USE_QT != 0 {
                qt = QuadTree_new_from_point_list(dim, n, max_qtree_level, x);
            }
            i = 0 as libc::c_int;
            while i < n {
                k = 0 as libc::c_int;
                while k < dim {
                    *f.offset(k as isize) = 0.0f64;
                    k += 1;
                }
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(*ja.offset(j as isize) == i) {
                        dist = distance(x, dim, i, *ja.offset(j as isize));
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                -= CRK
                                    * (*x.offset((i * dim + k) as isize)
                                        - *x.offset((*ja.offset(j as isize) * dim + k) as isize))
                                    * dist;
                            k += 1;
                        }
                    }
                    j += 1;
                }
                j = *id.offset(i as isize);
                while j < *id.offset((i + 1 as libc::c_int) as isize) {
                    if !(*jd.offset(j as isize) == i) {
                        dist = distance_cropped(x, dim, i, *jd.offset(j as isize));
                        k = 0 as libc::c_int;
                        while k < dim {
                            if dist < *d.offset(j as isize) {
                                *f.offset(k as isize)
                                    += 0.2f64 * CRK
                                        * (*x.offset((i * dim + k) as isize)
                                            - *x.offset((*jd.offset(j as isize) * dim + k) as isize))
                                        * (dist - *d.offset(j as isize))
                                        * (dist - *d.offset(j as isize)) / dist;
                            } else {
                                *f.offset(k as isize)
                                    -= 0.2f64 * CRK
                                        * (*x.offset((i * dim + k) as isize)
                                            - *x.offset((*jd.offset(j as isize) * dim + k) as isize))
                                        * (dist - *d.offset(j as isize))
                                        * (dist - *d.offset(j as isize)) / dist;
                            }
                            k += 1;
                        }
                    }
                    j += 1;
                }
                if USE_QT != 0 {
                    QuadTree_get_supernodes(
                        qt,
                        (*ctrl).bh,
                        &mut *x.offset((dim * i) as isize),
                        i,
                        &mut nsuper,
                        &mut nsupermax,
                        &mut center,
                        &mut supernode_wgts,
                        &mut distances,
                        &mut counts,
                        flag,
                    );
                    nsuper_avg += nsuper as libc::c_double;
                    if *flag != 0 {
                        current_block = 2460967513993438346;
                        break 's_221;
                    }
                    j = 0 as libc::c_int;
                    while j < nsuper {
                        dist = if *distances.offset(j as isize) > 1.0e-15f64 {
                            *distances.offset(j as isize)
                        } else {
                            1.0e-15f64
                        };
                        k = 0 as libc::c_int;
                        while k < dim {
                            *f.offset(k as isize)
                                += *supernode_wgts.offset(j as isize) * KP
                                    * (*x.offset((i * dim + k) as isize)
                                        - *center.offset((j * dim + k) as isize))
                                    / pow(dist, 1.0f64 - p);
                            k += 1;
                        }
                        j += 1;
                    }
                } else {
                    j = 0 as libc::c_int;
                    while j < n {
                        if !(j == i) {
                            dist = distance_cropped(x, dim, i, j);
                            k = 0 as libc::c_int;
                            while k < dim {
                                *f.offset(k as isize)
                                    += KP
                                        * (*x.offset((i * dim + k) as isize)
                                            - *x.offset((j * dim + k) as isize))
                                        / pow(dist, 1.0f64 - p);
                                k += 1;
                            }
                        }
                        j += 1;
                    }
                }
                F = 0.0f64;
                k = 0 as libc::c_int;
                while k < dim {
                    F += *f.offset(k as isize) * *f.offset(k as isize);
                    k += 1;
                }
                F = sqrt(F);
                Fnorm += F;
                if F > 0 as libc::c_int as libc::c_double {
                    k = 0 as libc::c_int;
                    while k < dim {
                        *f.offset(k as isize) /= F;
                        k += 1;
                    }
                }
                k = 0 as libc::c_int;
                while k < dim {
                    *x.offset((i * dim + k) as isize) += step * *f.offset(k as isize);
                    k += 1;
                }
                i += 1;
            }
            if !qt.is_null() {
                QuadTree_delete(qt);
            }
            nsuper_avg /= n as libc::c_double;
            step = update_step(adaptive_cooling, step, Fnorm, Fnorm0, cool);
            if !(step > tol && (iter as libc::c_double) < maxiter) {
                current_block = 13740693533991687037;
                break;
            }
        }
        match current_block {
            2460967513993438346 => {}
            _ => {
                if (*ctrl).beautify_leaves != 0 {
                    beautify_leaves(dim, A, x);
                }
            }
        }
    }
    free(xold as *mut libc::c_void);
    if A != A0 {
        SparseMatrix_delete(A);
    }
    free(f as *mut libc::c_void);
    free(center as *mut libc::c_void);
    free(supernode_wgts as *mut libc::c_void);
    free(distances as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn print_matrix(
    mut x: *mut libc::c_double,
    mut n: libc::c_int,
    mut dim: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    printf(b"{\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n {
        if i != 0 as libc::c_int {
            printf(b",\0" as *const u8 as *const libc::c_char);
        }
        printf(b"{\0" as *const u8 as *const libc::c_char);
        k = 0 as libc::c_int;
        while k < dim {
            if k != 0 as libc::c_int {
                printf(b",\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"%f\0" as *const u8 as *const libc::c_char,
                *x.offset((i * dim + k) as isize),
            );
            k += 1;
        }
        printf(b"}\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn interpolate_coord(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut x: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut nz: libc::c_int = 0;
    let mut alpha: libc::c_double = 0.5f64;
    let mut beta: libc::c_double = 0.;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    y = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < (*A).m {
        k = 0 as libc::c_int;
        while k < dim {
            *y.offset(k as isize) = 0 as libc::c_int as libc::c_double;
            k += 1;
        }
        nz = 0 as libc::c_int;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(*ja.offset(j as isize) == i) {
                nz += 1;
                k = 0 as libc::c_int;
                while k < dim {
                    *y.offset(k as isize)
                        += *x.offset((*ja.offset(j as isize) * dim + k) as isize);
                    k += 1;
                }
            }
            j += 1;
        }
        if nz > 0 as libc::c_int {
            beta = (1 as libc::c_int as libc::c_double - alpha) / nz as libc::c_double;
            k = 0 as libc::c_int;
            while k < dim {
                *x
                    .offset(
                        (i * dim + k) as isize,
                    ) = alpha * *x.offset((i * dim + k) as isize)
                    + beta * *y.offset(k as isize);
                k += 1;
            }
        }
        i += 1;
    }
    free(y as *mut libc::c_void);
}
unsafe extern "C" fn prolongate(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut P: SparseMatrix,
    mut R: SparseMatrix,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut coarsen_scheme_used: libc::c_int,
    mut delta: libc::c_double,
) {
    let mut nc: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    SparseMatrix_multiply_dense(P, x, &mut y, dim);
    if coarsen_scheme_used > EDGE_BASED_STA as libc::c_int
        && coarsen_scheme_used < EDGE_BASED_STO as libc::c_int
    {
        interpolate_coord(dim, A, y);
        nc = (*R).m;
        ia = (*R).ia;
        ja = (*R).ja;
        i = 0 as libc::c_int;
        while i < nc {
            j = *ia.offset(i as isize) + 1 as libc::c_int;
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                k = 0 as libc::c_int;
                while k < dim {
                    *y.offset((*ja.offset(j as isize) * dim + k) as isize)
                        += delta * (drand() - 0.5f64);
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn power_law_graph(mut A: SparseMatrix) -> libc::c_int {
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut j: libc::c_int = 0;
    let mut deg: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    m = (*A).m;
    mask = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((m + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m + 1 as libc::c_int {
        *mask.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        deg = 0 as libc::c_int;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(i == *ja.offset(j as isize)) {
                deg += 1;
            }
            j += 1;
        }
        let ref mut fresh3 = *mask.offset(deg as isize);
        *fresh3 += 1;
        max = if max > *mask.offset(deg as isize) {
            max
        } else {
            *mask.offset(deg as isize)
        };
        i += 1;
    }
    if *mask.offset(1 as libc::c_int as isize) as libc::c_double
        > 0.8f64 * max as libc::c_double
        && *mask.offset(1 as libc::c_int as isize) as libc::c_double
            > 0.3f64 * m as libc::c_double
    {
        res = (0 as libc::c_int == 0) as libc::c_int;
    }
    free(mask as *mut libc::c_void);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn pcp_rotate(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut y: [libc::c_double; 4] = [0.; 4];
    let mut axis: [libc::c_double; 2] = [0.; 2];
    let mut center: [libc::c_double; 2] = [0.; 2];
    let mut dist: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    if dim == 2 as libc::c_int {} else {
        __assert_fail(
            b"dim == 2\0" as *const u8 as *const libc::c_char,
            b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
            1666 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void pcp_rotate(int, int, double *)\0"))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < dim * dim {
        y[i as usize] = 0 as libc::c_int as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        center[i as usize] = 0 as libc::c_int as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            center[k as usize] += *x.offset((i * dim + k) as isize);
            k += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        center[i as usize] /= n as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            *x
                .offset(
                    (dim * i + k) as isize,
                ) = *x.offset((dim * i + k) as isize) - center[k as usize];
            k += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            l = 0 as libc::c_int;
            while l < dim {
                y[(dim * k + l) as usize]
                    += *x.offset((i * dim + k) as isize)
                        * *x.offset((i * dim + l) as isize);
                l += 1;
            }
            k += 1;
        }
        i += 1;
    }
    if y[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double {
        axis[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        axis[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    } else {
        axis[0 as libc::c_int
            as usize] = -(-y[0 as libc::c_int as usize] + y[3 as libc::c_int as usize]
            - sqrt(
                y[0 as libc::c_int as usize] * y[0 as libc::c_int as usize]
                    + 4 as libc::c_int as libc::c_double * y[1 as libc::c_int as usize]
                        * y[1 as libc::c_int as usize]
                    - 2 as libc::c_int as libc::c_double * y[0 as libc::c_int as usize]
                        * y[3 as libc::c_int as usize]
                    + y[3 as libc::c_int as usize] * y[3 as libc::c_int as usize],
            )) / (2 as libc::c_int as libc::c_double * y[1 as libc::c_int as usize]);
        axis[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    }
    dist = sqrt(
        1 as libc::c_int as libc::c_double
            + axis[0 as libc::c_int as usize] * axis[0 as libc::c_int as usize],
    );
    axis[0 as libc::c_int as usize] = axis[0 as libc::c_int as usize] / dist;
    axis[1 as libc::c_int as usize] = axis[1 as libc::c_int as usize] / dist;
    i = 0 as libc::c_int;
    while i < n {
        x0 = *x.offset((dim * i) as isize) * axis[0 as libc::c_int as usize]
            + *x.offset((dim * i + 1 as libc::c_int) as isize)
                * axis[1 as libc::c_int as usize];
        x1 = -*x.offset((dim * i) as isize) * axis[1 as libc::c_int as usize]
            + *x.offset((dim * i + 1 as libc::c_int) as isize)
                * axis[0 as libc::c_int as usize];
        *x.offset((dim * i) as isize) = x0;
        *x.offset((dim * i + 1 as libc::c_int) as isize) = x1;
        i += 1;
    }
}
unsafe extern "C" fn rotate(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut angle: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut axis: [libc::c_double; 2] = [0.; 2];
    let mut center: [libc::c_double; 2] = [0.; 2];
    let mut x0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut radian: libc::c_double = 3.14159f64 / 180 as libc::c_int as libc::c_double;
    if dim == 2 as libc::c_int {} else {
        __assert_fail(
            b"dim == 2\0" as *const u8 as *const libc::c_char,
            b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
            1719 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void rotate(int, int, double *, double)\0"))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < dim {
        center[i as usize] = 0 as libc::c_int as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            center[k as usize] += *x.offset((i * dim + k) as isize);
            k += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        center[i as usize] /= n as libc::c_double;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            *x
                .offset(
                    (dim * i + k) as isize,
                ) = *x.offset((dim * i + k) as isize) - center[k as usize];
            k += 1;
        }
        i += 1;
    }
    axis[0 as libc::c_int as usize] = cos(-angle * radian);
    axis[1 as libc::c_int as usize] = sin(-angle * radian);
    i = 0 as libc::c_int;
    while i < n {
        x0 = *x.offset((dim * i) as isize) * axis[0 as libc::c_int as usize]
            + *x.offset((dim * i + 1 as libc::c_int) as isize)
                * axis[1 as libc::c_int as usize];
        x1 = -*x.offset((dim * i) as isize) * axis[1 as libc::c_int as usize]
            + *x.offset((dim * i + 1 as libc::c_int) as isize)
                * axis[0 as libc::c_int as usize];
        *x.offset((dim * i) as isize) = x0;
        *x.offset((dim * i + 1 as libc::c_int) as isize) = x1;
        i += 1;
    }
}
unsafe extern "C" fn attach_edge_label_coordinates(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut n_edge_label_nodes: libc::c_int,
    mut edge_label_nodes: *mut libc::c_int,
    mut x: *mut libc::c_double,
    mut x2: *mut libc::c_double,
) {
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nnodes: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_double = 0.;
    mask = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*A).m as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*A).m {
        *mask.offset(i as isize) = 1 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n_edge_label_nodes {
        if *edge_label_nodes.offset(i as isize) >= 0 as libc::c_int
            && *edge_label_nodes.offset(i as isize) < (*A).m
        {
            *mask
                .offset(
                    *edge_label_nodes.offset(i as isize) as isize,
                ) = -(1 as libc::c_int);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*A).m {
        if *mask.offset(i as isize) >= 0 as libc::c_int {
            let fresh4 = nnodes;
            nnodes = nnodes + 1;
            *mask.offset(i as isize) = fresh4;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*A).m {
        if *mask.offset(i as isize) >= 0 as libc::c_int {
            k = 0 as libc::c_int;
            while k < dim {
                *x
                    .offset(
                        (i * dim + k) as isize,
                    ) = *x2.offset((*mask.offset(i as isize) * dim + k) as isize);
                k += 1;
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n_edge_label_nodes {
        ii = *edge_label_nodes.offset(i as isize);
        len = (*((*A).ia).offset((ii + 1 as libc::c_int) as isize)
            - *((*A).ia).offset(ii as isize)) as libc::c_double;
        if len >= 2 as libc::c_int as libc::c_double {} else {
            __assert_fail(
                b"len >= 2\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                1771 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void attach_edge_label_coordinates(int, SparseMatrix, int, int *, double *, double *)\0",
                ))
                    .as_ptr(),
            );
        }
        if *mask.offset(ii as isize) < 0 as libc::c_int {} else {
            __assert_fail(
                b"mask[ii] < 0\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                1772 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void attach_edge_label_coordinates(int, SparseMatrix, int, int *, double *, double *)\0",
                ))
                    .as_ptr(),
            );
        }
        k = 0 as libc::c_int;
        while k < dim {
            *x.offset((ii * dim + k) as isize) = 0 as libc::c_int as libc::c_double;
            k += 1;
        }
        j = *((*A).ia).offset(ii as isize);
        while j < *((*A).ia).offset((ii + 1 as libc::c_int) as isize) {
            k = 0 as libc::c_int;
            while k < dim {
                *x.offset((ii * dim + k) as isize)
                    += *x.offset((*((*A).ja).offset(j as isize) * dim + k) as isize);
                k += 1;
            }
            j += 1;
        }
        k = 0 as libc::c_int;
        while k < dim {
            *x.offset((ii * dim + k) as isize) /= len;
            k += 1;
        }
        i += 1;
    }
    free(mask as *mut libc::c_void);
}
unsafe extern "C" fn shorting_edge_label_nodes(
    mut A: SparseMatrix,
    mut n_edge_label_nodes: libc::c_int,
    mut edge_label_nodes: *mut libc::c_int,
) -> SparseMatrix {
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut id: libc::c_int = 0 as libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut irn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jcn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    mask = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*A).m as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*A).m {
        *mask.offset(i as isize) = 1 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n_edge_label_nodes {
        *mask
            .offset(*edge_label_nodes.offset(i as isize) as isize) = -(1 as libc::c_int);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*A).m {
        if *mask.offset(i as isize) > 0 as libc::c_int {
            let fresh5 = id;
            id = id + 1;
            *mask.offset(i as isize) = fresh5;
        }
        i += 1;
    }
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*A).m {
        if !(*mask.offset(i as isize) < 0 as libc::c_int) {
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if *mask.offset(*ja.offset(j as isize) as isize) >= 0 as libc::c_int {
                    nz += 1;
                } else {
                    ii = *ja.offset(j as isize);
                    jj = *ia.offset(ii as isize);
                    while jj < *ia.offset((ii + 1 as libc::c_int) as isize) {
                        if *ja.offset(jj as isize) != i
                            && *mask.offset(*ja.offset(jj as isize) as isize)
                                >= 0 as libc::c_int
                        {
                            nz += 1;
                        }
                        jj += 1;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    if nz > 0 as libc::c_int {
        irn = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nz as libc::c_ulong),
        ) as *mut libc::c_int;
        jcn = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nz as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*A).m {
        if !(*mask.offset(i as isize) < 0 as libc::c_int) {
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if *mask.offset(*ja.offset(j as isize) as isize) >= 0 as libc::c_int {
                    *irn.offset(nz as isize) = *mask.offset(i as isize);
                    let fresh6 = nz;
                    nz = nz + 1;
                    *jcn
                        .offset(
                            fresh6 as isize,
                        ) = *mask.offset(*ja.offset(j as isize) as isize);
                } else {
                    ii = *ja.offset(j as isize);
                    jj = *ia.offset(ii as isize);
                    while jj < *ia.offset((ii + 1 as libc::c_int) as isize) {
                        if *ja.offset(jj as isize) != i
                            && *mask.offset(*ja.offset(jj as isize) as isize)
                                >= 0 as libc::c_int
                        {
                            *irn.offset(nz as isize) = *mask.offset(i as isize);
                            let fresh7 = nz;
                            nz = nz + 1;
                            *jcn
                                .offset(
                                    fresh7 as isize,
                                ) = *mask.offset(*ja.offset(jj as isize) as isize);
                            if *mask.offset(i as isize) == 68 as libc::c_int
                                || *mask.offset(*ja.offset(jj as isize) as isize)
                                    == 68 as libc::c_int
                            {
                                fprintf(
                                    stderr,
                                    b"%d %d\n\0" as *const u8 as *const libc::c_char,
                                    *mask.offset(i as isize),
                                    *mask.offset(*ja.offset(jj as isize) as isize),
                                );
                                *mask.offset(i as isize) = *mask.offset(i as isize);
                            }
                        }
                        jj += 1;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    B = SparseMatrix_from_coordinate_arrays(
        nz,
        id,
        id,
        irn,
        jcn,
        0 as *mut libc::c_void,
        MATRIX_TYPE_PATTERN as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    free(irn as *mut libc::c_void);
    free(jcn as *mut libc::c_void);
    free(mask as *mut libc::c_void);
    return B;
}
unsafe extern "C" fn multilevel_spring_electrical_embedding_core(
    mut dim: libc::c_int,
    mut A0: SparseMatrix,
    mut D0: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut label_sizes: *mut libc::c_double,
    mut x: *mut libc::c_double,
    mut n_edge_label_nodes: libc::c_int,
    mut edge_label_nodes: *mut libc::c_int,
    mut flag: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut mctrl: Multilevel_control = 0 as Multilevel_control;
    let mut n: libc::c_int = 0;
    let mut plg: libc::c_int = 0;
    let mut coarsen_scheme_used: libc::c_int = 0;
    let mut A: SparseMatrix = A0;
    let mut D: SparseMatrix = D0;
    let mut P: SparseMatrix = 0 as SparseMatrix;
    let mut grid: Multilevel = 0 as *mut Multilevel_struct;
    let mut grid0: Multilevel = 0 as *mut Multilevel_struct;
    let mut xc: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xf: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ctrl0: spring_electrical_control_struct = spring_electrical_control_struct {
        p: 0.,
        q: 0.,
        random_start: 0,
        K: 0.,
        C: 0.,
        multilevels: 0,
        multilevel_coarsen_scheme: 0,
        multilevel_coarsen_mode: 0,
        quadtree_size: 0,
        max_qtree_level: 0,
        bh: 0.,
        tol: 0.,
        maxiter: 0,
        cool: 0.,
        step: 0.,
        adaptive_cooling: 0,
        random_seed: 0,
        beautify_leaves: 0,
        smoothing: 0,
        overlap: 0,
        do_shrinking: 0,
        tscheme: 0,
        method: 0,
        initial_scaling: 0.,
        rotation: 0.,
        edge_labeling_scheme: 0,
    };
    ctrl0 = *ctrl;
    *flag = 0 as libc::c_int;
    if A.is_null() {
        return;
    }
    n = (*A).n;
    if n <= 0 as libc::c_int || dim <= 0 as libc::c_int {
        return;
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) == 0
        || (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int
    {
        if (*ctrl).method == METHOD_SPRING_MAXENT as libc::c_int {
            A = SparseMatrix_symmetrize_nodiag(A);
            if !D0.is_null() {} else {
                __assert_fail(
                    b"D0\0" as *const u8 as *const libc::c_char,
                    b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                    1887 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 148],
                        &[libc::c_char; 148],
                    >(
                        b"void multilevel_spring_electrical_embedding_core(int, SparseMatrix, SparseMatrix, spring_electrical_control, double *, double *, int, int *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            D = SparseMatrix_symmetrize_nodiag(D);
        } else {
            A = SparseMatrix_get_real_adjacency_matrix_symmetrized(A);
        }
    } else {
        if (*ctrl).method == METHOD_SPRING_MAXENT as libc::c_int {
            if !D0.is_null() {} else {
                __assert_fail(
                    b"D0\0" as *const u8 as *const libc::c_char,
                    b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                    1894 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 148],
                        &[libc::c_char; 148],
                    >(
                        b"void multilevel_spring_electrical_embedding_core(int, SparseMatrix, SparseMatrix, spring_electrical_control, double *, double *, int, int *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            D = SparseMatrix_remove_diagonal(D);
        }
        A = SparseMatrix_remove_diagonal(A);
    }
    if ((*ctrl).edge_labeling_scheme == ELSCHEME_STRAIGHTLINE_PENALTY as libc::c_int
        || (*ctrl).edge_labeling_scheme == ELSCHEME_STRAIGHTLINE_PENALTY2 as libc::c_int)
        && n_edge_label_nodes > 0 as libc::c_int
    {
        let mut A2: SparseMatrix = 0 as *mut SparseMatrix_struct;
        let mut x2: *mut libc::c_double = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*A).m as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        A2 = shorting_edge_label_nodes(A, n_edge_label_nodes, edge_label_nodes);
        multilevel_spring_electrical_embedding(
            dim,
            A2,
            0 as SparseMatrix,
            ctrl,
            0 as *mut libc::c_double,
            x2,
            0 as libc::c_int,
            0 as *mut libc::c_int,
            flag,
        );
        if *flag == 0 {} else {
            __assert_fail(
                b"!(*flag)\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                1909 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 148],
                    &[libc::c_char; 148],
                >(
                    b"void multilevel_spring_electrical_embedding_core(int, SparseMatrix, SparseMatrix, spring_electrical_control, double *, double *, int, int *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        attach_edge_label_coordinates(
            dim,
            A,
            n_edge_label_nodes,
            edge_label_nodes,
            x,
            x2,
        );
        remove_overlap(
            dim,
            A,
            x,
            label_sizes,
            (*ctrl).overlap,
            (*ctrl).initial_scaling,
            (*ctrl).edge_labeling_scheme,
            n_edge_label_nodes,
            edge_label_nodes,
            A,
            (*ctrl).do_shrinking,
        );
        SparseMatrix_delete(A2);
        free(x2 as *mut libc::c_void);
        if A != A0 {
            SparseMatrix_delete(A);
        }
        return;
    }
    mctrl = Multilevel_control_new(
        (*ctrl).multilevel_coarsen_scheme,
        (*ctrl).multilevel_coarsen_mode,
    );
    (*mctrl).maxlevel = (*ctrl).multilevels;
    grid0 = Multilevel_new(A, D, mctrl);
    grid = Multilevel_get_coarsest(grid0);
    if ((*grid).prev).is_null() {
        xc = x;
    } else {
        xc = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*grid).n as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    plg = power_law_graph(A);
    if (*ctrl).p == -1.0001234f64 {
        (*ctrl).p = -(1 as libc::c_int) as libc::c_double;
        if plg != 0 {
            (*ctrl).p = -1.8f64;
        }
    }
    loop {
        if (*ctrl).method == METHOD_SPRING_ELECTRICAL as libc::c_int {
            if (*ctrl).tscheme == QUAD_TREE_NONE as libc::c_int {
                spring_electrical_embedding_slow(dim, (*grid).A, ctrl, xc, flag);
            } else if (*ctrl).tscheme == QUAD_TREE_FAST as libc::c_int
                    || (*ctrl).tscheme == QUAD_TREE_HYBRID as libc::c_int
                        && (*(*grid).A).m > QUAD_TREE_HYBRID_SIZE as libc::c_int
                {
                if (*ctrl).tscheme == QUAD_TREE_HYBRID as libc::c_int
                    && (*(*grid).A).m > 10 as libc::c_int && Verbose as libc::c_int != 0
                {
                    fprintf(
                        stderr,
                        b"QUAD_TREE_HYBRID, size larger than %d, switch to fast quadtree\0"
                            as *const u8 as *const libc::c_char,
                        QUAD_TREE_HYBRID_SIZE as libc::c_int,
                    );
                }
                spring_electrical_embedding_fast(dim, (*grid).A, ctrl, xc, flag);
            } else {
                spring_electrical_embedding(dim, (*grid).A, ctrl, xc, flag);
            }
        } else if (*ctrl).method == METHOD_SPRING_MAXENT as libc::c_int {
            let mut rho: libc::c_double = 0.05f64;
            (*ctrl).step = 1 as libc::c_int as libc::c_double;
            (*ctrl).adaptive_cooling = (0 as libc::c_int == 0) as libc::c_int;
            if ((*grid).next).is_null() {
                (*ctrl).maxiter = 500 as libc::c_int;
                rho = 0.5f64;
            } else {
                (*ctrl).maxiter = 100 as libc::c_int;
            }
            if ((*grid).prev).is_null() {
                spring_maxent_embedding(dim, (*grid).A, (*grid).D, ctrl, xc, rho, flag);
                (*ctrl).random_start = 0 as libc::c_int;
                (*ctrl).step = 0.05f64;
                (*ctrl).adaptive_cooling = 0 as libc::c_int;
                spring_maxent_embedding(
                    dim,
                    (*grid).A,
                    (*grid).D,
                    ctrl,
                    xc,
                    rho / 2 as libc::c_int as libc::c_double,
                    flag,
                );
                spring_maxent_embedding(
                    dim,
                    (*grid).A,
                    (*grid).D,
                    ctrl,
                    xc,
                    rho / 8 as libc::c_int as libc::c_double,
                    flag,
                );
                spring_maxent_embedding(
                    dim,
                    (*grid).A,
                    (*grid).D,
                    ctrl,
                    xc,
                    rho / 32 as libc::c_int as libc::c_double,
                    flag,
                );
            } else {
                spring_maxent_embedding(dim, (*grid).A, (*grid).D, ctrl, xc, rho, flag);
            }
        } else {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"spring_electrical.c\0" as *const u8 as *const libc::c_char,
                1983 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 148],
                    &[libc::c_char; 148],
                >(
                    b"void multilevel_spring_electrical_embedding_core(int, SparseMatrix, SparseMatrix, spring_electrical_control, double *, double *, int, int *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        if ((*grid).prev).is_null() {
            current_block = 15622658527355336244;
            break;
        }
        if *flag != 0 {
            free(xc as *mut libc::c_void);
            current_block = 13749659893526635823;
            break;
        } else {
            P = (*grid).P;
            coarsen_scheme_used = (*grid).coarsen_scheme_used;
            grid = (*grid).prev;
            if ((*grid).prev).is_null() {
                xf = x;
            } else {
                xf = gmalloc(
                    (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                        .wrapping_mul((*grid).n as libc::c_ulong)
                        .wrapping_mul(dim as libc::c_ulong),
                ) as *mut libc::c_double;
            }
            prolongate(
                dim,
                (*grid).A,
                P,
                (*grid).R,
                xc,
                xf,
                coarsen_scheme_used,
                (*ctrl).K * 0.001f64,
            );
            free(xc as *mut libc::c_void);
            xc = xf;
            (*ctrl).random_start = 0 as libc::c_int;
            (*ctrl).K = (*ctrl).K * 0.75f64;
            (*ctrl).adaptive_cooling = 0 as libc::c_int;
            if (*(*grid).next).coarsen_scheme_used > VERTEX_BASED_STA as libc::c_int
                && (*(*grid).next).coarsen_scheme_used < VERTEX_BASED_STO as libc::c_int
            {
                (*ctrl).step = 1 as libc::c_int as libc::c_double;
            } else {
                (*ctrl).step = 0.1f64;
            }
            if grid.is_null() {
                current_block = 15622658527355336244;
                break;
            }
        }
    }
    match current_block {
        15622658527355336244 => {
            post_process_smoothing(dim, A, ctrl, x, flag);
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"ctrl->overlap=%d\n\0" as *const u8 as *const libc::c_char,
                    (*ctrl).overlap,
                );
            }
            if dim == 2 as libc::c_int {
                pcp_rotate(n, dim, x);
            }
            if (*ctrl).rotation != 0 as libc::c_int as libc::c_double {
                rotate(n, dim, x, (*ctrl).rotation);
            }
            remove_overlap(
                dim,
                A,
                x,
                label_sizes,
                (*ctrl).overlap,
                (*ctrl).initial_scaling,
                (*ctrl).edge_labeling_scheme,
                n_edge_label_nodes,
                edge_label_nodes,
                A,
                (*ctrl).do_shrinking,
            );
        }
        _ => {}
    }
    *ctrl = ctrl0;
    if A != A0 {
        SparseMatrix_delete(A);
    }
    if !D.is_null() && D != D0 {
        SparseMatrix_delete(D);
    }
    Multilevel_control_delete(mctrl);
    Multilevel_delete(grid0);
}
#[no_mangle]
pub unsafe extern "C" fn multilevel_spring_electrical_embedding(
    mut dim: libc::c_int,
    mut A: SparseMatrix,
    mut D: SparseMatrix,
    mut ctrl: spring_electrical_control,
    mut label_sizes: *mut libc::c_double,
    mut x: *mut libc::c_double,
    mut n_edge_label_nodes: libc::c_int,
    mut edge_label_nodes: *mut libc::c_int,
    mut flag: *mut libc::c_int,
) {
    multilevel_spring_electrical_embedding_core(
        dim,
        A,
        D,
        ctrl,
        label_sizes,
        x,
        n_edge_label_nodes,
        edge_label_nodes,
        flag,
    );
}
