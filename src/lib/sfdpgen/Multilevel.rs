#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn free(_: *mut libc::c_void);
    fn SparseMatrix_decompose_to_supervariables(
        A: SparseMatrix,
        ncluster: *mut libc::c_int,
        cluster: *mut *mut libc::c_int,
        clusterp: *mut *mut libc::c_int,
    );
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn random_permutation(n: libc::c_int) -> *mut libc::c_int;
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
    fn SparseMatrix_multiply(A: SparseMatrix, B: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_multiply3(
        A: SparseMatrix,
        B: SparseMatrix,
        C: SparseMatrix,
    ) -> SparseMatrix;
    fn SparseMatrix_is_symmetric(
        A: SparseMatrix,
        test_pattern_symmetry_only: bool,
    ) -> libc::c_int;
    fn SparseMatrix_transpose(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_symmetrize_nodiag(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_multiply_vector(
        A: SparseMatrix,
        v: *mut libc::c_double,
        res: *mut *mut libc::c_double,
    );
    fn SparseMatrix_remove_diagonal(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_divide_row_by_degree(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(
        A: SparseMatrix,
    ) -> SparseMatrix;
    fn PriorityQueue_new(n: libc::c_int, ngain: libc::c_int) -> PriorityQueue;
    fn PriorityQueue_delete(q: PriorityQueue);
    fn PriorityQueue_push(
        q: PriorityQueue,
        i: libc::c_int,
        gain: libc::c_int,
    ) -> PriorityQueue;
    fn PriorityQueue_pop(
        q: PriorityQueue,
        i: *mut libc::c_int,
        gain: *mut libc::c_int,
    ) -> libc::c_int;
    fn PriorityQueue_remove(q: PriorityQueue, i: libc::c_int) -> libc::c_int;
    fn PriorityQueue_get_gain(q: PriorityQueue, i: libc::c_int) -> libc::c_int;
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
pub const MATRIX_UNDIRECTED: C2RustUnnamed = 16;
pub const MATRIX_HERMITIAN: C2RustUnnamed = 8;
pub const MATRIX_SKEW: C2RustUnnamed = 4;
pub const MATRIX_SYMMETRIC: C2RustUnnamed = 2;
pub const MATRIX_PATTERN_SYMMETRIC: C2RustUnnamed = 1;
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
pub type Multilevel = *mut Multilevel_struct;
pub type C2RustUnnamed_1 = libc::c_int;
pub const MAX_IND_VTX_SET_C: C2RustUnnamed_1 = 0;
pub const MAX_IND_VTX_SET_F: C2RustUnnamed_1 = -1;
pub const MAX_IND_VTX_SET_U: C2RustUnnamed_1 = -100;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MAX_CLUSTER_SIZE: C2RustUnnamed_2 = 4;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const COARSEN_HYBRID: C2RustUnnamed_3 = 12;
pub const VERTEX_BASED_STO: C2RustUnnamed_3 = 11;
pub const COARSEN_INDEPENDENT_VERTEX_SET_RS: C2RustUnnamed_3 = 10;
pub const COARSEN_INDEPENDENT_VERTEX_SET: C2RustUnnamed_3 = 9;
pub const VERTEX_BASED_STA: C2RustUnnamed_3 = 8;
pub const EDGE_BASED_STO: C2RustUnnamed_3 = 7;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_CLUSTER_PERNODE_LEAVES_FIRST: C2RustUnnamed_3 = 6;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_DEGREE_SCALED: C2RustUnnamed_3 = 5;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_SUPERNODES_FIRST: C2RustUnnamed_3 = 4;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_LEAVES_FIRST: C2RustUnnamed_3 = 3;
pub const COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE: C2RustUnnamed_3 = 2;
pub const COARSEN_INDEPENDENT_EDGE_SET: C2RustUnnamed_3 = 1;
pub const EDGE_BASED_STA: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const COARSEN_MODE_FORCEFUL: C2RustUnnamed_4 = 1;
pub const COARSEN_MODE_GENTLE: C2RustUnnamed_4 = 0;
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
pub type Multilevel_control = *mut Multilevel_control_struct;
pub type PriorityQueue = *mut PriorityQueue_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PriorityQueue_struct {
    pub count: libc::c_int,
    pub n: libc::c_int,
    pub ngain: libc::c_int,
    pub gain_max: libc::c_int,
    pub buckets: *mut DoubleLinkedList,
    pub where_0: *mut DoubleLinkedList,
    pub gain: *mut libc::c_int,
}
pub type DoubleLinkedList = *mut DoubleLinkedList_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DoubleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: DoubleLinkedList,
    pub prev: DoubleLinkedList,
}
pub const MATCHED: C2RustUnnamed_5 = -1;
pub type C2RustUnnamed_5 = libc::c_int;
pub const UNMATCHED: C2RustUnnamed_5 = -2;
pub const MATCHED_0: C2RustUnnamed_6 = -1;
pub type C2RustUnnamed_6 = libc::c_int;
pub const UNMATCHED_0: C2RustUnnamed_6 = -2;
pub const MATCHED_1: C2RustUnnamed_7 = -1;
pub type C2RustUnnamed_7 = libc::c_int;
pub const UNMATCHED_1: C2RustUnnamed_7 = -2;
#[no_mangle]
pub unsafe extern "C" fn Multilevel_control_new(
    mut scheme: libc::c_int,
    mut mode: libc::c_int,
) -> Multilevel_control {
    let mut ctrl: Multilevel_control = 0 as *mut Multilevel_control_struct;
    ctrl = gmalloc(::std::mem::size_of::<Multilevel_control_struct>() as libc::c_ulong)
        as *mut Multilevel_control_struct;
    (*ctrl).minsize = 4 as libc::c_int;
    (*ctrl).min_coarsen_factor = 0.75f64;
    (*ctrl).maxlevel = (1 as libc::c_int) << 30 as libc::c_int;
    (*ctrl).randomize = (0 as libc::c_int == 0) as libc::c_int;
    (*ctrl).coarsen_scheme = scheme;
    (*ctrl).coarsen_mode = mode;
    return ctrl;
}
#[no_mangle]
pub unsafe extern "C" fn Multilevel_control_delete(mut ctrl: Multilevel_control) {
    free(ctrl as *mut libc::c_void);
}
unsafe extern "C" fn Multilevel_init(
    mut A: SparseMatrix,
    mut D: SparseMatrix,
    mut node_weights: *mut libc::c_double,
) -> Multilevel {
    let mut grid: Multilevel = 0 as *mut Multilevel_struct;
    if A.is_null() {
        return 0 as Multilevel;
    }
    if (*A).m == (*A).n {} else {
        __assert_fail(
            b"A->m == A->n\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"Multilevel Multilevel_init(SparseMatrix, SparseMatrix, double *)\0"))
                .as_ptr(),
        );
    }
    grid = gmalloc(::std::mem::size_of::<Multilevel_struct>() as libc::c_ulong)
        as *mut Multilevel_struct;
    (*grid).level = 0 as libc::c_int;
    (*grid).n = (*A).n;
    let ref mut fresh0 = (*grid).A;
    *fresh0 = A;
    let ref mut fresh1 = (*grid).D;
    *fresh1 = D;
    let ref mut fresh2 = (*grid).P;
    *fresh2 = 0 as SparseMatrix;
    let ref mut fresh3 = (*grid).R;
    *fresh3 = 0 as SparseMatrix;
    let ref mut fresh4 = (*grid).node_weights;
    *fresh4 = node_weights;
    let ref mut fresh5 = (*grid).next;
    *fresh5 = 0 as Multilevel;
    let ref mut fresh6 = (*grid).prev;
    *fresh6 = 0 as Multilevel;
    (*grid).delete_top_level_A = 0 as libc::c_int;
    return grid;
}
#[no_mangle]
pub unsafe extern "C" fn Multilevel_delete(mut grid: Multilevel) {
    if grid.is_null() {
        return;
    }
    if !((*grid).A).is_null() {
        if (*grid).level == 0 as libc::c_int {
            if (*grid).delete_top_level_A != 0 {
                SparseMatrix_delete((*grid).A);
                if !((*grid).D).is_null() {
                    SparseMatrix_delete((*grid).D);
                }
            }
        } else {
            SparseMatrix_delete((*grid).A);
            if !((*grid).D).is_null() {
                SparseMatrix_delete((*grid).D);
            }
        }
    }
    SparseMatrix_delete((*grid).P);
    SparseMatrix_delete((*grid).R);
    if !((*grid).node_weights).is_null() && (*grid).level > 0 as libc::c_int {
        free((*grid).node_weights as *mut libc::c_void);
    }
    Multilevel_delete((*grid).next);
    free(grid as *mut libc::c_void);
}
unsafe extern "C" fn maximal_independent_vertex_set(
    mut A: SparseMatrix,
    mut randomize: libc::c_int,
    mut vset: *mut *mut libc::c_int,
    mut nvset: *mut libc::c_int,
    mut nzc: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"void maximal_independent_vertex_set(SparseMatrix, int, int **, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"void maximal_independent_vertex_set(SparseMatrix, int, int **, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"void maximal_independent_vertex_set(SparseMatrix, int, int **, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *vset = gcalloc(m as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *(*vset).offset(i as isize) = MAX_IND_VTX_SET_U as libc::c_int;
        i += 1;
    }
    *nvset = 0 as libc::c_int;
    *nzc = 0 as libc::c_int;
    if randomize == 0 {
        i = 0 as libc::c_int;
        while i < m {
            if *(*vset).offset(i as isize) == MAX_IND_VTX_SET_U as libc::c_int {
                let fresh7 = *nvset;
                *nvset = *nvset + 1;
                *(*vset).offset(i as isize) = fresh7;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        *(*vset)
                            .offset(
                                *ja.offset(j as isize) as isize,
                            ) = MAX_IND_VTX_SET_F as libc::c_int;
                        *nzc += 1;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
    } else {
        p = random_permutation(m);
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            if *(*vset).offset(i as isize) == MAX_IND_VTX_SET_U as libc::c_int {
                let fresh8 = *nvset;
                *nvset = *nvset + 1;
                *(*vset).offset(i as isize) = fresh8;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        *(*vset)
                            .offset(
                                *ja.offset(j as isize) as isize,
                            ) = MAX_IND_VTX_SET_F as libc::c_int;
                        *nzc += 1;
                    }
                    j += 1;
                }
            }
            ii += 1;
        }
        free(p as *mut libc::c_void);
    }
    *nzc += *nvset;
}
unsafe extern "C" fn maximal_independent_vertex_set_RS(
    mut A: SparseMatrix,
    mut randomize: libc::c_int,
    mut vset: *mut *mut libc::c_int,
    mut nvset: *mut libc::c_int,
    mut nzc: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut gain: libc::c_int = 0;
    let mut removed: libc::c_int = 0;
    let mut nf: libc::c_int = 0 as libc::c_int;
    let mut q: PriorityQueue = 0 as *mut PriorityQueue_struct;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void maximal_independent_vertex_set_RS(SparseMatrix, int, int **, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void maximal_independent_vertex_set_RS(SparseMatrix, int, int **, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void maximal_independent_vertex_set_RS(SparseMatrix, int, int **, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *vset = gcalloc(m as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *(*vset).offset(i as isize) = MAX_IND_VTX_SET_U as libc::c_int;
        i += 1;
    }
    *nvset = 0 as libc::c_int;
    *nzc = 0 as libc::c_int;
    q = PriorityQueue_new(m, 2 as libc::c_int * (m - 1 as libc::c_int));
    if randomize == 0 {
        i = 0 as libc::c_int;
        while i < m {
            PriorityQueue_push(
                q,
                i,
                *ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize),
            );
            i += 1;
        }
    } else {
        p = random_permutation(m);
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            PriorityQueue_push(
                q,
                i,
                *ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize),
            );
            ii += 1;
        }
        free(p as *mut libc::c_void);
    }
    while PriorityQueue_pop(q, &mut i, &mut gain) != 0 {
        if *(*vset).offset(i as isize) == MAX_IND_VTX_SET_U as libc::c_int {} else {
            __assert_fail(
                b"(*vset)[i] == MAX_IND_VTX_SET_U\0" as *const u8 as *const libc::c_char,
                b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                171 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"void maximal_independent_vertex_set_RS(SparseMatrix, int, int **, int *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        let fresh9 = *nvset;
        *nvset = *nvset + 1;
        *(*vset).offset(i as isize) = fresh9;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            if *(*vset).offset(jj as isize) == MAX_IND_VTX_SET_U as libc::c_int
                || *(*vset).offset(jj as isize) == MAX_IND_VTX_SET_F as libc::c_int
            {} else {
                __assert_fail(
                    b"(*vset)[jj] == MAX_IND_VTX_SET_U || (*vset)[jj] == MAX_IND_VTX_SET_F\0"
                        as *const u8 as *const libc::c_char,
                    b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                    175 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 80],
                        &[libc::c_char; 80],
                    >(
                        b"void maximal_independent_vertex_set_RS(SparseMatrix, int, int **, int *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if !(i == jj) {
                if *(*vset).offset(jj as isize) == MAX_IND_VTX_SET_U as libc::c_int {
                    removed = PriorityQueue_remove(q, jj);
                    if removed != 0 {} else {
                        __assert_fail(
                            b"removed\0" as *const u8 as *const libc::c_char,
                            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                            180 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 80],
                                &[libc::c_char; 80],
                            >(
                                b"void maximal_independent_vertex_set_RS(SparseMatrix, int, int **, int *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *(*vset).offset(jj as isize) = MAX_IND_VTX_SET_F as libc::c_int;
                    nf += 1;
                    k = *ia.offset(jj as isize);
                    while k < *ia.offset((jj + 1 as libc::c_int) as isize) {
                        if !(jj == *ja.offset(k as isize)) {
                            if *(*vset).offset(*ja.offset(k as isize) as isize)
                                == MAX_IND_VTX_SET_U as libc::c_int
                            {
                                gain = PriorityQueue_get_gain(q, *ja.offset(k as isize));
                                if gain >= 0 as libc::c_int {} else {
                                    __assert_fail(
                                        b"gain >= 0\0" as *const u8 as *const libc::c_char,
                                        b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                                        188 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 80],
                                            &[libc::c_char; 80],
                                        >(
                                            b"void maximal_independent_vertex_set_RS(SparseMatrix, int, int **, int *, int *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                PriorityQueue_push(
                                    q,
                                    *ja.offset(k as isize),
                                    gain + 1 as libc::c_int,
                                );
                            }
                        }
                        k += 1;
                    }
                }
                *nzc += 1;
            }
            j += 1;
        }
    }
    *nzc += *nvset;
    PriorityQueue_delete(q);
}
unsafe extern "C" fn maximal_independent_edge_set(
    mut A: SparseMatrix,
    mut randomize: libc::c_int,
    mut matching: *mut *mut libc::c_int,
    mut nmatch: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void maximal_independent_edge_set(SparseMatrix, int, int **, int *)\0"))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void maximal_independent_edge_set(SparseMatrix, int, int **, int *)\0"))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void maximal_independent_edge_set(SparseMatrix, int, int **, int *)\0"))
                .as_ptr(),
        );
    }
    *matching = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *(*matching).offset(i as isize) = i;
        i += 1;
    }
    *nmatch = n;
    if randomize == 0 {
        i = 0 as libc::c_int;
        while i < m {
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if !(i == *ja.offset(j as isize)) {
                    if *(*matching).offset(*ja.offset(j as isize) as isize)
                        == *ja.offset(j as isize) && *(*matching).offset(i as isize) == i
                    {
                        *(*matching).offset(*ja.offset(j as isize) as isize) = i;
                        *(*matching).offset(i as isize) = *ja.offset(j as isize);
                        *nmatch -= 1;
                    }
                }
                j += 1;
            }
            i += 1;
        }
    } else {
        p = random_permutation(m);
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if !(i == *ja.offset(j as isize)) {
                    if *(*matching).offset(*ja.offset(j as isize) as isize)
                        == *ja.offset(j as isize) && *(*matching).offset(i as isize) == i
                    {
                        *(*matching).offset(*ja.offset(j as isize) as isize) = i;
                        *(*matching).offset(i as isize) = *ja.offset(j as isize);
                        *nmatch -= 1;
                    }
                }
                j += 1;
            }
            ii += 1;
        }
        free(p as *mut libc::c_void);
    };
}
unsafe extern "C" fn maximal_independent_edge_set_heavest_edge_pernode(
    mut A: SparseMatrix,
    mut randomize: libc::c_int,
    mut matching: *mut *mut libc::c_int,
    mut nmatch: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut amax: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut first: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    let mut jamax: libc::c_int = 0 as libc::c_int;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *matching = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *(*matching).offset(i as isize) = i;
        i += 1;
    }
    *nmatch = n;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    a = (*A).a as *mut libc::c_double;
    if randomize == 0 {
        i = 0 as libc::c_int;
        while i < m {
            first = (0 as libc::c_int == 0) as libc::c_int;
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if !(i == *ja.offset(j as isize)) {
                    if *(*matching).offset(*ja.offset(j as isize) as isize)
                        == *ja.offset(j as isize) && *(*matching).offset(i as isize) == i
                    {
                        if first != 0 {
                            amax = *a.offset(j as isize);
                            jamax = *ja.offset(j as isize);
                            first = 0 as libc::c_int;
                        } else if *a.offset(j as isize) > amax {
                            amax = *a.offset(j as isize);
                            jamax = *ja.offset(j as isize);
                        }
                    }
                }
                j += 1;
            }
            if first == 0 {
                *(*matching).offset(jamax as isize) = i;
                *(*matching).offset(i as isize) = jamax;
                *nmatch -= 1;
            }
            i += 1;
        }
    } else {
        p = random_permutation(m);
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            if !(*(*matching).offset(i as isize) != i) {
                first = (0 as libc::c_int == 0) as libc::c_int;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        if *(*matching).offset(*ja.offset(j as isize) as isize)
                            == *ja.offset(j as isize)
                            && *(*matching).offset(i as isize) == i
                        {
                            if first != 0 {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                                first = 0 as libc::c_int;
                            } else if *a.offset(j as isize) > amax {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                            }
                        }
                    }
                    j += 1;
                }
                if first == 0 {
                    *(*matching).offset(jamax as isize) = i;
                    *(*matching).offset(i as isize) = jamax;
                    *nmatch -= 1;
                }
            }
            ii += 1;
        }
        free(p as *mut libc::c_void);
    };
}
unsafe extern "C" fn maximal_independent_edge_set_heavest_edge_pernode_leaves_first(
    mut A: SparseMatrix,
    mut randomize: libc::c_int,
    mut cluster: *mut *mut libc::c_int,
    mut clusterp: *mut *mut libc::c_int,
    mut ncluster: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut q: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut amax: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut first: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    let mut jamax: libc::c_int = 0 as libc::c_int;
    let mut matched: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut ncmax: libc::c_int = 0 as libc::c_int;
    let mut nz0: libc::c_int = 0;
    let mut nzz: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *cluster = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *clusterp = gcalloc(
        (m + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    matched = gcalloc(m as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *matched.offset(i as isize) = i;
        i += 1;
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *ncluster = 0 as libc::c_int;
    *(*clusterp).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    nz = 0 as libc::c_int;
    a = (*A).a as *mut libc::c_double;
    if randomize == 0 {
        i = 0 as libc::c_int;
        while i < m {
            if !(*matched.offset(i as isize) == MATCHED_1 as libc::c_int
                || *ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize)
                    != 1 as libc::c_int)
            {
                q = *ja.offset(*ia.offset(i as isize) as isize);
                if *matched.offset(q as isize) != MATCHED_1 as libc::c_int {} else {
                    __assert_fail(
                        b"matched[q] != MATCHED\0" as *const u8 as *const libc::c_char,
                        b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                        359 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 110],
                            &[libc::c_char; 110],
                        >(
                            b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *matched.offset(q as isize) = MATCHED_1 as libc::c_int;
                let fresh10 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh10 as isize) = q;
                j = *ia.offset(q as isize);
                while j < *ia.offset((q + 1 as libc::c_int) as isize) {
                    if !(q == *ja.offset(j as isize)) {
                        if *ia
                            .offset((*ja.offset(j as isize) + 1 as libc::c_int) as isize)
                            - *ia.offset(*ja.offset(j as isize) as isize)
                            == 1 as libc::c_int
                        {
                            *matched
                                .offset(
                                    *ja.offset(j as isize) as isize,
                                ) = MATCHED_1 as libc::c_int;
                            let fresh11 = nz;
                            nz = nz + 1;
                            *(*cluster)
                                .offset(fresh11 as isize) = *ja.offset(j as isize);
                        }
                    }
                    j += 1;
                }
                ncmax = if ncmax > nz - *(*clusterp).offset(*ncluster as isize) {
                    ncmax
                } else {
                    nz - *(*clusterp).offset(*ncluster as isize)
                };
                nz0 = *(*clusterp).offset(*ncluster as isize);
                if nz - nz0 <= MAX_CLUSTER_SIZE as libc::c_int {
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz;
                } else {
                    nz0 += 1;
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz0;
                    nzz = nz0;
                    k = nz0;
                    while k < nz && nzz < nz {
                        nzz += MAX_CLUSTER_SIZE as libc::c_int - 1 as libc::c_int;
                        nzz = if nz < nzz { nz } else { nzz };
                        *ncluster += 1;
                        *(*clusterp).offset(*ncluster as isize) = nzz;
                        k += 1;
                    }
                }
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < m {
            first = (0 as libc::c_int == 0) as libc::c_int;
            if !(*matched.offset(i as isize) == MATCHED_1 as libc::c_int) {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        if *matched.offset(*ja.offset(j as isize) as isize)
                            != MATCHED_1 as libc::c_int
                            && *matched.offset(i as isize) != MATCHED_1 as libc::c_int
                        {
                            if first != 0 {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                                first = 0 as libc::c_int;
                            } else if *a.offset(j as isize) > amax {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                            }
                        }
                    }
                    j += 1;
                }
                if first == 0 {
                    *matched.offset(jamax as isize) = MATCHED_1 as libc::c_int;
                    *matched.offset(i as isize) = MATCHED_1 as libc::c_int;
                    let fresh12 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh12 as isize) = i;
                    let fresh13 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh13 as isize) = jamax;
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz;
                }
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < m {
            if *matched.offset(i as isize) == i {
                let fresh14 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh14 as isize) = i;
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz;
            }
            i += 1;
        }
        if nz == n {} else {
            __assert_fail(
                b"nz == n\0" as *const u8 as *const libc::c_char,
                b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                422 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        p = random_permutation(m);
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            if !(*matched.offset(i as isize) == MATCHED_1 as libc::c_int
                || *ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize)
                    != 1 as libc::c_int)
            {
                q = *ja.offset(*ia.offset(i as isize) as isize);
                if *matched.offset(q as isize) != MATCHED_1 as libc::c_int {} else {
                    __assert_fail(
                        b"matched[q] != MATCHED\0" as *const u8 as *const libc::c_char,
                        b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                        430 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 110],
                            &[libc::c_char; 110],
                        >(
                            b"void maximal_independent_edge_set_heavest_edge_pernode_leaves_first(SparseMatrix, int, int **, int **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *matched.offset(q as isize) = MATCHED_1 as libc::c_int;
                let fresh15 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh15 as isize) = q;
                j = *ia.offset(q as isize);
                while j < *ia.offset((q + 1 as libc::c_int) as isize) {
                    if !(q == *ja.offset(j as isize)) {
                        if *ia
                            .offset((*ja.offset(j as isize) + 1 as libc::c_int) as isize)
                            - *ia.offset(*ja.offset(j as isize) as isize)
                            == 1 as libc::c_int
                        {
                            *matched
                                .offset(
                                    *ja.offset(j as isize) as isize,
                                ) = MATCHED_1 as libc::c_int;
                            let fresh16 = nz;
                            nz = nz + 1;
                            *(*cluster)
                                .offset(fresh16 as isize) = *ja.offset(j as isize);
                        }
                    }
                    j += 1;
                }
                ncmax = if ncmax > nz - *(*clusterp).offset(*ncluster as isize) {
                    ncmax
                } else {
                    nz - *(*clusterp).offset(*ncluster as isize)
                };
                nz0 = *(*clusterp).offset(*ncluster as isize);
                if nz - nz0 <= MAX_CLUSTER_SIZE as libc::c_int {
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz;
                } else {
                    nz0 += 1;
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz0;
                    nzz = nz0;
                    k = nz0;
                    while k < nz && nzz < nz {
                        nzz += MAX_CLUSTER_SIZE as libc::c_int - 1 as libc::c_int;
                        nzz = if nz < nzz { nz } else { nzz };
                        *ncluster += 1;
                        *(*clusterp).offset(*ncluster as isize) = nzz;
                        k += 1;
                    }
                }
            }
            ii += 1;
        }
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            first = (0 as libc::c_int == 0) as libc::c_int;
            if !(*matched.offset(i as isize) == MATCHED_1 as libc::c_int) {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        if *matched.offset(*ja.offset(j as isize) as isize)
                            != MATCHED_1 as libc::c_int
                            && *matched.offset(i as isize) != MATCHED_1 as libc::c_int
                        {
                            if first != 0 {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                                first = 0 as libc::c_int;
                            } else if *a.offset(j as isize) > amax {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                            }
                        }
                    }
                    j += 1;
                }
                if first == 0 {
                    *matched.offset(jamax as isize) = MATCHED_1 as libc::c_int;
                    *matched.offset(i as isize) = MATCHED_1 as libc::c_int;
                    let fresh17 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh17 as isize) = i;
                    let fresh18 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh18 as isize) = jamax;
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz;
                }
            }
            ii += 1;
        }
        i = 0 as libc::c_int;
        while i < m {
            if *matched.offset(i as isize) == i {
                let fresh19 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh19 as isize) = i;
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz;
            }
            i += 1;
        }
        free(p as *mut libc::c_void);
    }
    free(matched as *mut libc::c_void);
}
unsafe extern "C" fn maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(
    mut A: SparseMatrix,
    mut randomize: libc::c_int,
    mut cluster: *mut *mut libc::c_int,
    mut clusterp: *mut *mut libc::c_int,
    mut ncluster: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut amax: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut first: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    let mut jamax: libc::c_int = 0 as libc::c_int;
    let mut matched: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut nz0: libc::c_int = 0;
    let mut nsuper: libc::c_int = 0;
    let mut super_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut superp: *mut libc::c_int = 0 as *mut libc::c_int;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            512 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            518 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *cluster = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *clusterp = gcalloc(
        (m + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    matched = gcalloc(m as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *matched.offset(i as isize) = i;
        i += 1;
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            525 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 114],
                &[libc::c_char; 114],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(SparseMatrix, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    SparseMatrix_decompose_to_supervariables(A, &mut nsuper, &mut super_0, &mut superp);
    *ncluster = 0 as libc::c_int;
    *(*clusterp).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    nz = 0 as libc::c_int;
    a = (*A).a as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < nsuper {
        if !(*superp.offset((i + 1 as libc::c_int) as isize) - *superp.offset(i as isize)
            <= 1 as libc::c_int)
        {
            nz0 = *(*clusterp).offset(*ncluster as isize);
            j = *superp.offset(i as isize);
            while j < *superp.offset((i + 1 as libc::c_int) as isize) {
                *matched
                    .offset(
                        *super_0.offset(j as isize) as isize,
                    ) = MATCHED_0 as libc::c_int;
                let fresh20 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh20 as isize) = *super_0.offset(j as isize);
                if nz - nz0 >= MAX_CLUSTER_SIZE as libc::c_int {
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz;
                    nz0 = nz;
                }
                j += 1;
            }
            if nz > nz0 {
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz;
            }
        }
        i += 1;
    }
    if randomize == 0 {
        i = 0 as libc::c_int;
        while i < m {
            first = (0 as libc::c_int == 0) as libc::c_int;
            if !(*matched.offset(i as isize) == MATCHED_0 as libc::c_int) {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        if *matched.offset(*ja.offset(j as isize) as isize)
                            != MATCHED_0 as libc::c_int
                            && *matched.offset(i as isize) != MATCHED_0 as libc::c_int
                        {
                            if first != 0 {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                                first = 0 as libc::c_int;
                            } else if *a.offset(j as isize) > amax {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                            }
                        }
                    }
                    j += 1;
                }
                if first == 0 {
                    *matched.offset(jamax as isize) = MATCHED_0 as libc::c_int;
                    *matched.offset(i as isize) = MATCHED_0 as libc::c_int;
                    let fresh21 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh21 as isize) = i;
                    let fresh22 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh22 as isize) = jamax;
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz;
                }
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < m {
            if *matched.offset(i as isize) == i {
                let fresh23 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh23 as isize) = i;
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz;
            }
            i += 1;
        }
        if nz == n {} else {
            __assert_fail(
                b"nz == n\0" as *const u8 as *const libc::c_char,
                b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                584 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 114],
                    &[libc::c_char; 114],
                >(
                    b"void maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(SparseMatrix, int, int **, int **, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        p = random_permutation(m);
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            first = (0 as libc::c_int == 0) as libc::c_int;
            if !(*matched.offset(i as isize) == MATCHED_0 as libc::c_int) {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        if *matched.offset(*ja.offset(j as isize) as isize)
                            != MATCHED_0 as libc::c_int
                            && *matched.offset(i as isize) != MATCHED_0 as libc::c_int
                        {
                            if first != 0 {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                                first = 0 as libc::c_int;
                            } else if *a.offset(j as isize) > amax {
                                amax = *a.offset(j as isize);
                                jamax = *ja.offset(j as isize);
                            }
                        }
                    }
                    j += 1;
                }
                if first == 0 {
                    *matched.offset(jamax as isize) = MATCHED_0 as libc::c_int;
                    *matched.offset(i as isize) = MATCHED_0 as libc::c_int;
                    let fresh24 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh24 as isize) = i;
                    let fresh25 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh25 as isize) = jamax;
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nz;
                }
            }
            ii += 1;
        }
        i = 0 as libc::c_int;
        while i < m {
            if *matched.offset(i as isize) == i {
                let fresh26 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh26 as isize) = i;
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz;
            }
            i += 1;
        }
        free(p as *mut libc::c_void);
    }
    free(super_0 as *mut libc::c_void);
    free(superp as *mut libc::c_void);
    free(matched as *mut libc::c_void);
}
unsafe extern "C" fn scomp(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    let mut ss1: *const libc::c_double = s1 as *const libc::c_double;
    let mut ss2: *const libc::c_double = s2 as *const libc::c_double;
    if *ss1.offset(1 as libc::c_int as isize) > *ss2.offset(1 as libc::c_int as isize) {
        return -(1 as libc::c_int)
    } else {
        if *ss1.offset(1 as libc::c_int as isize)
            < *ss2.offset(1 as libc::c_int as isize)
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(
    mut A: SparseMatrix,
    mut csize: libc::c_int,
    mut randomize: libc::c_int,
    mut cluster: *mut *mut libc::c_int,
    mut clusterp: *mut *mut libc::c_int,
    mut ncluster: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut q: libc::c_int = 0;
    let mut iv: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut matched: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut nz0: libc::c_int = 0;
    let mut nzz: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut vlist: *mut libc::c_double = 0 as *mut libc::c_double;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            655 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(SparseMatrix, int, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            656 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(SparseMatrix, int, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            661 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(SparseMatrix, int, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *cluster = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *clusterp = gcalloc(
        (m + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    matched = gcalloc(m as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    vlist = gcalloc(
        (2 as libc::c_int * m) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        *matched.offset(i as isize) = i;
        i += 1;
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            669 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(SparseMatrix, int, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            670 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"void maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(SparseMatrix, int, int, int **, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *ncluster = 0 as libc::c_int;
    *(*clusterp).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    nz = 0 as libc::c_int;
    a = (*A).a as *mut libc::c_double;
    p = random_permutation(m);
    ii = 0 as libc::c_int;
    while ii < m {
        i = *p.offset(ii as isize);
        if !(*matched.offset(i as isize) == MATCHED as libc::c_int
            || *ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize)
                != 1 as libc::c_int)
        {
            q = *ja.offset(*ia.offset(i as isize) as isize);
            if *matched.offset(q as isize) != MATCHED as libc::c_int {} else {
                __assert_fail(
                    b"matched[q] != MATCHED\0" as *const u8 as *const libc::c_char,
                    b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                    682 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 118],
                        &[libc::c_char; 118],
                    >(
                        b"void maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(SparseMatrix, int, int, int **, int **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            *matched.offset(q as isize) = MATCHED as libc::c_int;
            let fresh27 = nz;
            nz = nz + 1;
            *(*cluster).offset(fresh27 as isize) = q;
            j = *ia.offset(q as isize);
            while j < *ia.offset((q + 1 as libc::c_int) as isize) {
                if !(q == *ja.offset(j as isize)) {
                    if *ia.offset((*ja.offset(j as isize) + 1 as libc::c_int) as isize)
                        - *ia.offset(*ja.offset(j as isize) as isize) == 1 as libc::c_int
                    {
                        *matched
                            .offset(
                                *ja.offset(j as isize) as isize,
                            ) = MATCHED as libc::c_int;
                        let fresh28 = nz;
                        nz = nz + 1;
                        *(*cluster).offset(fresh28 as isize) = *ja.offset(j as isize);
                    }
                }
                j += 1;
            }
            nz0 = *(*clusterp).offset(*ncluster as isize);
            if nz - nz0 <= MAX_CLUSTER_SIZE as libc::c_int {
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz;
            } else {
                nz0 += 1;
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz0;
                nzz = nz0;
                k = nz0;
                while k < nz && nzz < nz {
                    nzz += MAX_CLUSTER_SIZE as libc::c_int - 1 as libc::c_int;
                    nzz = if nz < nzz { nz } else { nzz };
                    *ncluster += 1;
                    *(*clusterp).offset(*ncluster as isize) = nzz;
                    k += 1;
                }
            }
        }
        ii += 1;
    }
    ii = 0 as libc::c_int;
    while ii < m {
        i = *p.offset(ii as isize);
        if !(*matched.offset(i as isize) == MATCHED as libc::c_int) {
            nv = 0 as libc::c_int;
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if !(i == *ja.offset(j as isize)) {
                    if *matched.offset(*ja.offset(j as isize) as isize)
                        != MATCHED as libc::c_int
                        && *matched.offset(i as isize) != MATCHED as libc::c_int
                    {
                        *vlist
                            .offset(
                                (2 as libc::c_int * nv) as isize,
                            ) = *ja.offset(j as isize) as libc::c_double;
                        *vlist
                            .offset(
                                (2 as libc::c_int * nv + 1 as libc::c_int) as isize,
                            ) = *a.offset(j as isize);
                        nv += 1;
                    }
                }
                j += 1;
            }
            if nv > 0 as libc::c_int {
                qsort(
                    vlist as *mut libc::c_void,
                    nv as size_t,
                    (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    Some(
                        scomp
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                j = 0 as libc::c_int;
                while j
                    < (if (csize - 1 as libc::c_int) < nv {
                        csize - 1 as libc::c_int
                    } else {
                        nv
                    })
                {
                    iv = *vlist.offset((2 as libc::c_int * j) as isize) as libc::c_int;
                    *matched.offset(iv as isize) = MATCHED as libc::c_int;
                    let fresh29 = nz;
                    nz = nz + 1;
                    *(*cluster).offset(fresh29 as isize) = iv;
                    j += 1;
                }
                *matched.offset(i as isize) = MATCHED as libc::c_int;
                let fresh30 = nz;
                nz = nz + 1;
                *(*cluster).offset(fresh30 as isize) = i;
                *ncluster += 1;
                *(*clusterp).offset(*ncluster as isize) = nz;
            }
        }
        ii += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        if *matched.offset(i as isize) == i {
            let fresh31 = nz;
            nz = nz + 1;
            *(*cluster).offset(fresh31 as isize) = i;
            *ncluster += 1;
            *(*clusterp).offset(*ncluster as isize) = nz;
        }
        i += 1;
    }
    free(p as *mut libc::c_void);
    free(matched as *mut libc::c_void);
}
unsafe extern "C" fn maximal_independent_edge_set_heavest_edge_pernode_scaled(
    mut A: SparseMatrix,
    mut randomize: libc::c_int,
    mut matching: *mut *mut libc::c_int,
    mut nmatch: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut amax: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut first: libc::c_int = (0 as libc::c_int == 0) as libc::c_int;
    let mut jamax: libc::c_int = 0 as libc::c_int;
    if !A.is_null() {} else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            748 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_scaled(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0 {} else {
        __assert_fail(
            b"SparseMatrix_known_strucural_symmetric(A)\0" as *const u8
                as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            749 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_scaled(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    n = (*A).n;
    if n == m {} else {
        __assert_fail(
            b"n == m\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            754 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_scaled(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *matching = gcalloc(
        m as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *(*matching).offset(i as isize) = i;
        i += 1;
    }
    *nmatch = n;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            759 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_scaled(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            760 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void maximal_independent_edge_set_heavest_edge_pernode_scaled(SparseMatrix, int, int **, int *)\0",
            ))
                .as_ptr(),
        );
    }
    a = (*A).a as *mut libc::c_double;
    if randomize == 0 {
        i = 0 as libc::c_int;
        while i < m {
            first = (0 as libc::c_int == 0) as libc::c_int;
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if !(i == *ja.offset(j as isize)) {
                    if *(*matching).offset(*ja.offset(j as isize) as isize)
                        == *ja.offset(j as isize) && *(*matching).offset(i as isize) == i
                    {
                        if first != 0 {
                            amax = *a.offset(j as isize)
                                / (*ia.offset((i + 1 as libc::c_int) as isize)
                                    - *ia.offset(i as isize)) as libc::c_double
                                / (*ia
                                    .offset(
                                        (*ja.offset(j as isize) + 1 as libc::c_int) as isize,
                                    ) - *ia.offset(*ja.offset(j as isize) as isize))
                                    as libc::c_double;
                            jamax = *ja.offset(j as isize);
                            first = 0 as libc::c_int;
                        } else if *a.offset(j as isize)
                                / (*ia.offset((i + 1 as libc::c_int) as isize)
                                    - *ia.offset(i as isize)) as libc::c_double
                                / (*ia
                                    .offset(
                                        (*ja.offset(j as isize) + 1 as libc::c_int) as isize,
                                    ) - *ia.offset(*ja.offset(j as isize) as isize))
                                    as libc::c_double > amax
                            {
                            amax = *a.offset(j as isize)
                                / (*ia.offset((i + 1 as libc::c_int) as isize)
                                    - *ia.offset(i as isize)) as libc::c_double
                                / (*ia
                                    .offset(
                                        (*ja.offset(j as isize) + 1 as libc::c_int) as isize,
                                    ) - *ia.offset(*ja.offset(j as isize) as isize))
                                    as libc::c_double;
                            jamax = *ja.offset(j as isize);
                        }
                    }
                }
                j += 1;
            }
            if first == 0 {
                *(*matching).offset(jamax as isize) = i;
                *(*matching).offset(i as isize) = jamax;
                *nmatch -= 1;
            }
            i += 1;
        }
    } else {
        p = random_permutation(m);
        ii = 0 as libc::c_int;
        while ii < m {
            i = *p.offset(ii as isize);
            if !(*(*matching).offset(i as isize) != i) {
                first = (0 as libc::c_int == 0) as libc::c_int;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !(i == *ja.offset(j as isize)) {
                        if *(*matching).offset(*ja.offset(j as isize) as isize)
                            == *ja.offset(j as isize)
                            && *(*matching).offset(i as isize) == i
                        {
                            if first != 0 {
                                amax = *a.offset(j as isize)
                                    / (*ia.offset((i + 1 as libc::c_int) as isize)
                                        - *ia.offset(i as isize)) as libc::c_double
                                    / (*ia
                                        .offset(
                                            (*ja.offset(j as isize) + 1 as libc::c_int) as isize,
                                        ) - *ia.offset(*ja.offset(j as isize) as isize))
                                        as libc::c_double;
                                jamax = *ja.offset(j as isize);
                                first = 0 as libc::c_int;
                            } else if *a.offset(j as isize)
                                    / (*ia.offset((i + 1 as libc::c_int) as isize)
                                        - *ia.offset(i as isize)) as libc::c_double
                                    / (*ia
                                        .offset(
                                            (*ja.offset(j as isize) + 1 as libc::c_int) as isize,
                                        ) - *ia.offset(*ja.offset(j as isize) as isize))
                                        as libc::c_double > amax
                                {
                                amax = *a.offset(j as isize)
                                    / (*ia.offset((i + 1 as libc::c_int) as isize)
                                        - *ia.offset(i as isize)) as libc::c_double
                                    / (*ia
                                        .offset(
                                            (*ja.offset(j as isize) + 1 as libc::c_int) as isize,
                                        ) - *ia.offset(*ja.offset(j as isize) as isize))
                                        as libc::c_double;
                                jamax = *ja.offset(j as isize);
                            }
                        }
                    }
                    j += 1;
                }
                if first == 0 {
                    *(*matching).offset(jamax as isize) = i;
                    *(*matching).offset(i as isize) = jamax;
                    *nmatch -= 1;
                }
            }
            ii += 1;
        }
        free(p as *mut libc::c_void);
    };
}
unsafe extern "C" fn DistanceMatrix_restrict_filtering(
    mut mask: *mut libc::c_int,
    mut is_C: libc::c_int,
    mut is_F: libc::c_int,
    mut D: SparseMatrix,
) -> SparseMatrix {
    if D.is_null() {
        return 0 as SparseMatrix;
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"Multilevel.c\0" as *const u8 as *const libc::c_char,
        821 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 78],
            &[libc::c_char; 78],
        >(
            b"SparseMatrix DistanceMatrix_restrict_filtering(int *, int, int, SparseMatrix)\0",
        ))
            .as_ptr(),
    );
    return 0 as SparseMatrix;
}
unsafe extern "C" fn Multilevel_coarsen_internal(
    mut A: SparseMatrix,
    mut cA: *mut SparseMatrix,
    mut D: SparseMatrix,
    mut cD: *mut SparseMatrix,
    mut node_wgt: *mut libc::c_double,
    mut cnode_wgt: *mut *mut libc::c_double,
    mut P: *mut SparseMatrix,
    mut R: *mut SparseMatrix,
    mut ctrl: Multilevel_control,
    mut coarsen_scheme_used: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut matching: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmatch: libc::c_int = 0 as libc::c_int;
    let mut nc: libc::c_int = 0;
    let mut nzc: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut irn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jcn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut B: SparseMatrix = 0 as SparseMatrix;
    let mut vset: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvset: libc::c_int = 0;
    let mut ncov: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cluster: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut clusterp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncluster: libc::c_int = 0;
    if (*A).m == (*A).n {} else {
        __assert_fail(
            b"A->m == A->n\0" as *const u8 as *const libc::c_char,
            b"Multilevel.c\0" as *const u8 as *const libc::c_char,
            835 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 173],
                &[libc::c_char; 173],
            >(
                b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *cA = 0 as SparseMatrix;
    *cD = 0 as SparseMatrix;
    *P = 0 as SparseMatrix;
    *R = 0 as SparseMatrix;
    n = (*A).m;
    *coarsen_scheme_used = (*ctrl).coarsen_scheme;
    match (*ctrl).coarsen_scheme {
        12 => {
            let ref mut fresh32 = (*ctrl).coarsen_scheme;
            *fresh32 = COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_LEAVES_FIRST
                as libc::c_int;
            *coarsen_scheme_used = *fresh32;
            Multilevel_coarsen_internal(
                A,
                cA,
                D,
                cD,
                node_wgt,
                cnode_wgt,
                P,
                R,
                ctrl,
                coarsen_scheme_used,
            );
            if (*cA).is_null() {
                let ref mut fresh33 = (*ctrl).coarsen_scheme;
                *fresh33 = COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_SUPERNODES_FIRST
                    as libc::c_int;
                *coarsen_scheme_used = *fresh33;
                Multilevel_coarsen_internal(
                    A,
                    cA,
                    D,
                    cD,
                    node_wgt,
                    cnode_wgt,
                    P,
                    R,
                    ctrl,
                    coarsen_scheme_used,
                );
            }
            if (*cA).is_null() {
                let ref mut fresh34 = (*ctrl).coarsen_scheme;
                *fresh34 = COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_CLUSTER_PERNODE_LEAVES_FIRST
                    as libc::c_int;
                *coarsen_scheme_used = *fresh34;
                Multilevel_coarsen_internal(
                    A,
                    cA,
                    D,
                    cD,
                    node_wgt,
                    cnode_wgt,
                    P,
                    R,
                    ctrl,
                    coarsen_scheme_used,
                );
            }
            if (*cA).is_null() {
                let ref mut fresh35 = (*ctrl).coarsen_scheme;
                *fresh35 = COARSEN_INDEPENDENT_VERTEX_SET as libc::c_int;
                *coarsen_scheme_used = *fresh35;
                Multilevel_coarsen_internal(
                    A,
                    cA,
                    D,
                    cD,
                    node_wgt,
                    cnode_wgt,
                    P,
                    R,
                    ctrl,
                    coarsen_scheme_used,
                );
            }
            if (*cA).is_null() {
                let ref mut fresh36 = (*ctrl).coarsen_scheme;
                *fresh36 = COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE
                    as libc::c_int;
                *coarsen_scheme_used = *fresh36;
                Multilevel_coarsen_internal(
                    A,
                    cA,
                    D,
                    cD,
                    node_wgt,
                    cnode_wgt,
                    P,
                    R,
                    ctrl,
                    coarsen_scheme_used,
                );
            }
            (*ctrl).coarsen_scheme = COARSEN_HYBRID as libc::c_int;
            current_block = 13133327569201511773;
        }
        4 | 6 | 3 => {
            if (*ctrl).coarsen_scheme
                == COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_LEAVES_FIRST
                    as libc::c_int
            {
                maximal_independent_edge_set_heavest_edge_pernode_leaves_first(
                    A,
                    (*ctrl).randomize,
                    &mut cluster,
                    &mut clusterp,
                    &mut ncluster,
                );
            } else if (*ctrl).coarsen_scheme
                    == COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_SUPERNODES_FIRST
                        as libc::c_int
                {
                maximal_independent_edge_set_heavest_edge_pernode_supernodes_first(
                    A,
                    (*ctrl).randomize,
                    &mut cluster,
                    &mut clusterp,
                    &mut ncluster,
                );
            } else {
                maximal_independent_edge_set_heavest_cluster_pernode_leaves_first(
                    A,
                    4 as libc::c_int,
                    (*ctrl).randomize,
                    &mut cluster,
                    &mut clusterp,
                    &mut ncluster,
                );
            }
            if ncluster <= n {} else {
                __assert_fail(
                    b"ncluster <= n\0" as *const u8 as *const libc::c_char,
                    b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                    901 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 173],
                        &[libc::c_char; 173],
                    >(
                        b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            nc = ncluster;
            if (*ctrl).coarsen_mode == COARSEN_MODE_GENTLE as libc::c_int
                && nc as libc::c_double
                    > (*ctrl).min_coarsen_factor * n as libc::c_double || nc == n
                || nc < (*ctrl).minsize
            {
                current_block = 13133327569201511773;
            } else {
                irn = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                jcn = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                val = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                ) as *mut libc::c_double;
                nzc = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < ncluster {
                    j = *clusterp.offset(i as isize);
                    while j < *clusterp.offset((i + 1 as libc::c_int) as isize) {
                        if *clusterp.offset((i + 1 as libc::c_int) as isize)
                            > *clusterp.offset(i as isize)
                        {} else {
                            __assert_fail(
                                b"clusterp[i+1] > clusterp[i]\0" as *const u8
                                    as *const libc::c_char,
                                b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                                916 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 173],
                                    &[libc::c_char; 173],
                                >(
                                    b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        *irn.offset(nzc as isize) = *cluster.offset(j as isize);
                        *jcn.offset(nzc as isize) = i;
                        let fresh37 = nzc;
                        nzc = nzc + 1;
                        *val.offset(fresh37 as isize) = 1.0f64;
                        j += 1;
                    }
                    i += 1;
                }
                if nzc == n {} else {
                    __assert_fail(
                        b"nzc == n\0" as *const u8 as *const libc::c_char,
                        b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                        922 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 173],
                            &[libc::c_char; 173],
                        >(
                            b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *P = SparseMatrix_from_coordinate_arrays(
                    nzc,
                    n,
                    nc,
                    irn,
                    jcn,
                    val as *mut libc::c_void,
                    MATRIX_TYPE_REAL as libc::c_int,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                *R = SparseMatrix_transpose(*P);
                *cD = 0 as SparseMatrix;
                *cA = SparseMatrix_multiply3(*R, A, *P);
                if (*cA).is_null() {
                    current_block = 13133327569201511773;
                } else {
                    SparseMatrix_multiply_vector(*R, node_wgt, cnode_wgt);
                    *R = SparseMatrix_divide_row_by_degree(*R);
                    (**cA).property = (**cA).property | MATRIX_SYMMETRIC as libc::c_int;
                    (**cA)
                        .property = (**cA).property
                        | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
                    *cA = SparseMatrix_remove_diagonal(*cA);
                    current_block = 13133327569201511773;
                }
            }
        }
        1 => {
            maximal_independent_edge_set(
                A,
                (*ctrl).randomize,
                &mut matching,
                &mut nmatch,
            );
            current_block = 15612923873456120061;
        }
        2 => {
            current_block = 15612923873456120061;
        }
        5 => {
            current_block = 3825971548398956300;
        }
        9 | 10 => {
            if (*ctrl).coarsen_scheme == COARSEN_INDEPENDENT_VERTEX_SET as libc::c_int {
                maximal_independent_vertex_set(
                    A,
                    (*ctrl).randomize,
                    &mut vset,
                    &mut nvset,
                    &mut nzc,
                );
            } else {
                maximal_independent_vertex_set_RS(
                    A,
                    (*ctrl).randomize,
                    &mut vset,
                    &mut nvset,
                    &mut nzc,
                );
            }
            ia = (*A).ia;
            ja = (*A).ja;
            nc = nvset;
            if (*ctrl).coarsen_mode == COARSEN_MODE_GENTLE as libc::c_int
                && nc as libc::c_double
                    > (*ctrl).min_coarsen_factor * n as libc::c_double || nc == n
                || nc < (*ctrl).minsize
            {
                current_block = 13133327569201511773;
            } else {
                irn = gcalloc(
                    nzc as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                jcn = gcalloc(
                    nzc as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                val = gcalloc(
                    nzc as size_t,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                ) as *mut libc::c_double;
                nzc = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n {
                    if *vset.offset(i as isize) == MAX_IND_VTX_SET_F as libc::c_int {
                        ncov = 0 as libc::c_int;
                        j = *ia.offset(i as isize);
                        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                            if *vset.offset(*ja.offset(j as isize) as isize)
                                >= MAX_IND_VTX_SET_C as libc::c_int
                            {
                                ncov += 1;
                            }
                            j += 1;
                        }
                        if ncov > 0 as libc::c_int {} else {
                            __assert_fail(
                                b"ncov > 0\0" as *const u8 as *const libc::c_char,
                                b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                                1037 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 173],
                                    &[libc::c_char; 173],
                                >(
                                    b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        j = *ia.offset(i as isize);
                        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                            if *vset.offset(*ja.offset(j as isize) as isize)
                                >= MAX_IND_VTX_SET_C as libc::c_int
                            {
                                *irn.offset(nzc as isize) = i;
                                *jcn
                                    .offset(
                                        nzc as isize,
                                    ) = *vset.offset(*ja.offset(j as isize) as isize);
                                let fresh41 = nzc;
                                nzc = nzc + 1;
                                *val
                                    .offset(fresh41 as isize) = 1.0f64 / ncov as libc::c_double;
                            }
                            j += 1;
                        }
                    } else {
                        if *vset.offset(i as isize) >= MAX_IND_VTX_SET_C as libc::c_int
                        {} else {
                            __assert_fail(
                                b"vset[i] >= MAX_IND_VTX_SET_C\0" as *const u8
                                    as *const libc::c_char,
                                b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                                1046 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 173],
                                    &[libc::c_char; 173],
                                >(
                                    b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        *irn.offset(nzc as isize) = i;
                        *jcn.offset(nzc as isize) = *vset.offset(i as isize);
                        let fresh42 = nzc;
                        nzc = nzc + 1;
                        *val.offset(fresh42 as isize) = 1.0f64;
                    }
                    i += 1;
                }
                *P = SparseMatrix_from_coordinate_arrays(
                    nzc,
                    n,
                    nc,
                    irn,
                    jcn,
                    val as *mut libc::c_void,
                    MATRIX_TYPE_REAL as libc::c_int,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                *R = SparseMatrix_transpose(*P);
                *cA = SparseMatrix_multiply3(*R, A, *P);
                if (*cA).is_null() {
                    current_block = 13133327569201511773;
                } else {
                    SparseMatrix_multiply_vector(*R, node_wgt, cnode_wgt);
                    (**cA).property = (**cA).property | MATRIX_SYMMETRIC as libc::c_int;
                    (**cA)
                        .property = (**cA).property
                        | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
                    *cA = SparseMatrix_remove_diagonal(*cA);
                    *cD = DistanceMatrix_restrict_filtering(
                        vset,
                        MAX_IND_VTX_SET_C as libc::c_int,
                        MAX_IND_VTX_SET_F as libc::c_int,
                        D,
                    );
                    current_block = 13133327569201511773;
                }
            }
        }
        _ => {
            current_block = 13133327569201511773;
        }
    }
    match current_block {
        15612923873456120061 => {
            if (*ctrl).coarsen_scheme
                == COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE as libc::c_int
            {
                maximal_independent_edge_set_heavest_edge_pernode(
                    A,
                    (*ctrl).randomize,
                    &mut matching,
                    &mut nmatch,
                );
            }
            current_block = 3825971548398956300;
        }
        _ => {}
    }
    match current_block {
        3825971548398956300 => {
            if (*ctrl).coarsen_scheme
                == COARSEN_INDEPENDENT_EDGE_SET_HEAVEST_EDGE_PERNODE_DEGREE_SCALED
                    as libc::c_int
            {
                maximal_independent_edge_set_heavest_edge_pernode_scaled(
                    A,
                    (*ctrl).randomize,
                    &mut matching,
                    &mut nmatch,
                );
            }
            nc = nmatch;
            if !((*ctrl).coarsen_mode == COARSEN_MODE_GENTLE as libc::c_int
                && nc as libc::c_double
                    > (*ctrl).min_coarsen_factor * n as libc::c_double || nc == n
                || nc < (*ctrl).minsize)
            {
                irn = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                jcn = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                val = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                ) as *mut libc::c_double;
                nzc = 0 as libc::c_int;
                nc = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n {
                    if *matching.offset(i as isize) >= 0 as libc::c_int {
                        if *matching.offset(i as isize) == i {
                            *irn.offset(nzc as isize) = i;
                            *jcn.offset(nzc as isize) = nc;
                            let fresh38 = nzc;
                            nzc = nzc + 1;
                            *val.offset(fresh38 as isize) = 1.0f64;
                        } else {
                            *irn.offset(nzc as isize) = i;
                            *jcn.offset(nzc as isize) = nc;
                            let fresh39 = nzc;
                            nzc = nzc + 1;
                            *val
                                .offset(
                                    fresh39 as isize,
                                ) = 1 as libc::c_int as libc::c_double;
                            *irn.offset(nzc as isize) = *matching.offset(i as isize);
                            *jcn.offset(nzc as isize) = nc;
                            let fresh40 = nzc;
                            nzc = nzc + 1;
                            *val
                                .offset(
                                    fresh40 as isize,
                                ) = 1 as libc::c_int as libc::c_double;
                            *matching
                                .offset(
                                    *matching.offset(i as isize) as isize,
                                ) = -(1 as libc::c_int);
                        }
                        nc += 1;
                        *matching.offset(i as isize) = -(1 as libc::c_int);
                    }
                    i += 1;
                }
                if nc == nmatch {} else {
                    __assert_fail(
                        b"nc == nmatch\0" as *const u8 as *const libc::c_char,
                        b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                        986 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 173],
                            &[libc::c_char; 173],
                        >(
                            b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if nzc == n {} else {
                    __assert_fail(
                        b"nzc == n\0" as *const u8 as *const libc::c_char,
                        b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                        987 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 173],
                            &[libc::c_char; 173],
                        >(
                            b"void Multilevel_coarsen_internal(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *P = SparseMatrix_from_coordinate_arrays(
                    nzc,
                    n,
                    nc,
                    irn,
                    jcn,
                    val as *mut libc::c_void,
                    MATRIX_TYPE_REAL as libc::c_int,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                *R = SparseMatrix_transpose(*P);
                *cA = SparseMatrix_multiply3(*R, A, *P);
                if !(*cA).is_null() {
                    SparseMatrix_multiply_vector(*R, node_wgt, cnode_wgt);
                    *R = SparseMatrix_divide_row_by_degree(*R);
                    (**cA).property = (**cA).property | MATRIX_SYMMETRIC as libc::c_int;
                    (**cA)
                        .property = (**cA).property
                        | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
                    *cA = SparseMatrix_remove_diagonal(*cA);
                    *cD = 0 as SparseMatrix;
                }
            }
        }
        _ => {}
    }
    free(matching as *mut libc::c_void);
    free(vset as *mut libc::c_void);
    free(irn as *mut libc::c_void);
    free(jcn as *mut libc::c_void);
    free(val as *mut libc::c_void);
    if !B.is_null() {
        SparseMatrix_delete(B);
    }
    free(cluster as *mut libc::c_void);
    free(clusterp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Multilevel_coarsen(
    mut A: SparseMatrix,
    mut cA: *mut SparseMatrix,
    mut D: SparseMatrix,
    mut cD: *mut SparseMatrix,
    mut node_wgt: *mut libc::c_double,
    mut cnode_wgt: *mut *mut libc::c_double,
    mut P: *mut SparseMatrix,
    mut R: *mut SparseMatrix,
    mut ctrl: Multilevel_control,
    mut coarsen_scheme_used: *mut libc::c_int,
) {
    let mut cA0: SparseMatrix = A;
    let mut cD0: SparseMatrix = 0 as SparseMatrix;
    let mut P0: SparseMatrix = 0 as SparseMatrix;
    let mut R0: SparseMatrix = 0 as SparseMatrix;
    let mut M: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut cnode_wgt0: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nc: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    *P = 0 as SparseMatrix;
    *R = 0 as SparseMatrix;
    *cA = 0 as SparseMatrix;
    *cnode_wgt = 0 as *mut libc::c_double;
    *cD = 0 as SparseMatrix;
    n = (*A).n;
    loop {
        node_wgt = cnode_wgt0;
        Multilevel_coarsen_internal(
            A,
            &mut cA0,
            D,
            &mut cD0,
            node_wgt,
            &mut cnode_wgt0,
            &mut P0,
            &mut R0,
            ctrl,
            coarsen_scheme_used,
        );
        if cA0.is_null() {
            return;
        }
        nc = (*cA0).n;
        if !(*P).is_null() {
            if !(*R).is_null() {} else {
                __assert_fail(
                    b"*R\0" as *const u8 as *const libc::c_char,
                    b"Multilevel.c\0" as *const u8 as *const libc::c_char,
                    1099 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 164],
                        &[libc::c_char; 164],
                    >(
                        b"void Multilevel_coarsen(SparseMatrix, SparseMatrix *, SparseMatrix, SparseMatrix *, double *, double **, SparseMatrix *, SparseMatrix *, Multilevel_control, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            M = SparseMatrix_multiply(*P, P0);
            SparseMatrix_delete(*P);
            SparseMatrix_delete(P0);
            *P = M;
            M = SparseMatrix_multiply(R0, *R);
            SparseMatrix_delete(*R);
            SparseMatrix_delete(R0);
            *R = M;
        } else {
            *P = P0;
            *R = R0;
        }
        if !(*cA).is_null() {
            SparseMatrix_delete(*cA);
        }
        *cA = cA0;
        if !(*cD).is_null() {
            SparseMatrix_delete(*cD);
        }
        *cD = cD0;
        if !(*cnode_wgt).is_null() {
            free(*cnode_wgt as *mut libc::c_void);
        }
        *cnode_wgt = cnode_wgt0;
        A = cA0;
        D = cD0;
        node_wgt = cnode_wgt0;
        cnode_wgt0 = 0 as *mut libc::c_double;
        if !(nc as libc::c_double > (*ctrl).min_coarsen_factor * n as libc::c_double
            && (*ctrl).coarsen_mode == COARSEN_MODE_FORCEFUL as libc::c_int)
        {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_padding(mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        fputs(b" \0" as *const u8 as *const libc::c_char, stderr);
        i += 1;
    }
}
unsafe extern "C" fn Multilevel_establish(
    mut grid: Multilevel,
    mut ctrl: Multilevel_control,
) -> Multilevel {
    let mut cgrid: Multilevel = 0 as *mut Multilevel_struct;
    let mut coarsen_scheme_used: libc::c_int = 0;
    let mut cnode_weights: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut P: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut R: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut cA: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut D: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut cD: SparseMatrix = 0 as *mut SparseMatrix_struct;
    A = (*grid).A;
    D = (*grid).D;
    if (*grid).level >= (*ctrl).maxlevel - 1 as libc::c_int {
        return grid;
    }
    Multilevel_coarsen(
        A,
        &mut cA,
        D,
        &mut cD,
        (*grid).node_weights,
        &mut cnode_weights,
        &mut P,
        &mut R,
        ctrl,
        &mut coarsen_scheme_used,
    );
    if cA.is_null() {
        return grid;
    }
    cgrid = Multilevel_init(cA, cD, cnode_weights);
    let ref mut fresh43 = (*grid).next;
    *fresh43 = cgrid;
    (*cgrid).coarsen_scheme_used = coarsen_scheme_used;
    (*cgrid).level = (*grid).level + 1 as libc::c_int;
    (*cgrid).n = (*cA).m;
    let ref mut fresh44 = (*cgrid).A;
    *fresh44 = cA;
    let ref mut fresh45 = (*cgrid).D;
    *fresh45 = cD;
    let ref mut fresh46 = (*cgrid).P;
    *fresh46 = P;
    let ref mut fresh47 = (*grid).R;
    *fresh47 = R;
    let ref mut fresh48 = (*cgrid).prev;
    *fresh48 = grid;
    cgrid = Multilevel_establish(cgrid, ctrl);
    return grid;
}
#[no_mangle]
pub unsafe extern "C" fn Multilevel_new(
    mut A0: SparseMatrix,
    mut D0: SparseMatrix,
    mut ctrl: Multilevel_control,
) -> Multilevel {
    let mut grid: Multilevel = 0 as *mut Multilevel_struct;
    let mut A: SparseMatrix = A0;
    let mut D: SparseMatrix = D0;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) == 0
        || (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int
    {
        A = SparseMatrix_get_real_adjacency_matrix_symmetrized(A);
    }
    if !D.is_null()
        && (SparseMatrix_is_symmetric(D, 0 as libc::c_int != 0) == 0
            || (*D).type_0 != MATRIX_TYPE_REAL as libc::c_int)
    {
        D = SparseMatrix_symmetrize_nodiag(D);
    }
    grid = Multilevel_init(A, D, 0 as *mut libc::c_double);
    grid = Multilevel_establish(grid, ctrl);
    if A != A0 {
        (*grid).delete_top_level_A = (0 as libc::c_int == 0) as libc::c_int;
    }
    return grid;
}
#[no_mangle]
pub unsafe extern "C" fn Multilevel_get_coarsest(mut grid: Multilevel) -> Multilevel {
    while !((*grid).next).is_null() {
        grid = (*grid).next;
    }
    return grid;
}
