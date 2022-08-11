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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut Verbose: libc::c_uchar;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn SparseMatrix_multiply(A: SparseMatrix, B: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_coordinate_form_add_entry(
        A: SparseMatrix,
        irn: libc::c_int,
        jcn: libc::c_int,
        val: *mut libc::c_void,
    ) -> SparseMatrix;
    fn SparseMatrix_is_symmetric(A: SparseMatrix, test_pattern_symmetry_only: bool) -> libc::c_int;
    fn SparseMatrix_transpose(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_symmetrize(A: SparseMatrix, pattern_symmetric_only: bool) -> SparseMatrix;
    fn SparseMatrix_remove_diagonal(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_copy(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_set_entries_to_real_one(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_from_coordinate_format(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_multiply_vector(
        A: SparseMatrix,
        v: *mut libc::c_double,
        res: *mut *mut libc::c_double,
    );
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SingleLinkedList_new_int(i: libc::c_int) -> SingleLinkedList;
    fn SingleLinkedList_delete(
        head: SingleLinkedList,
        linklist_deallocator: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn SingleLinkedList_prepend_int(l: SingleLinkedList, i: libc::c_int) -> SingleLinkedList;
    fn SingleLinkedList_get_data(l: SingleLinkedList) -> *mut libc::c_void;
    fn SingleLinkedList_get_next(l: SingleLinkedList) -> SingleLinkedList;
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
pub type C2RustUnnamed = libc::c_int;
pub const UNMATCHED: C2RustUnnamed = -1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FORMAT_COORD: C2RustUnnamed_0 = 2;
pub const FORMAT_CSR: C2RustUnnamed_0 = 1;
pub const FORMAT_CSC: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_1 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_1 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_1 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_1 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Multilevel_MQ_Clustering_struct {
    pub level: libc::c_int,
    pub n: libc::c_int,
    pub A: SparseMatrix,
    pub P: SparseMatrix,
    pub R: SparseMatrix,
    pub next: Multilevel_MQ_Clustering,
    pub prev: Multilevel_MQ_Clustering,
    pub delete_top_level_A: libc::c_int,
    pub matching: *mut libc::c_int,
    pub mq: libc::c_double,
    pub mq_in: libc::c_double,
    pub mq_out: libc::c_double,
    pub ncluster: libc::c_int,
    pub deg_intra: *mut libc::c_double,
    pub dout: *mut libc::c_double,
    pub wgt: *mut libc::c_double,
}
pub type Multilevel_MQ_Clustering = *mut Multilevel_MQ_Clustering_struct;
pub type SingleLinkedList = *mut SingleLinkedList_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SingleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: SingleLinkedList,
}
unsafe extern "C" fn get_mq(
    mut A: SparseMatrix,
    mut assignment: *mut libc::c_int,
    mut ncluster0: *mut libc::c_int,
    mut mq_in0: *mut libc::c_double,
    mut mq_out0: *mut libc::c_double,
    mut dout0: *mut *mut libc::c_double,
) -> libc::c_double {
    let mut ncluster: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = (*A).m;
    let mut test_pattern_symmetry_only: bool = 0 as libc::c_int != 0;
    let mut counts: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut mq_in: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mq_out: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut Vi: libc::c_double = 0.;
    let mut Vj: libc::c_double = 0.;
    let mut c: libc::c_int = 0;
    let mut dout: *mut libc::c_double = 0 as *mut libc::c_double;
    if SparseMatrix_is_symmetric(A, test_pattern_symmetry_only) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, test_pattern_symmetry_only)\0" as *const u8
                as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                b"double get_mq(SparseMatrix, int *, int *, double *, double *, double **)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).n == n {
    } else {
        __assert_fail(
            b"A->n == n\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                b"double get_mq(SparseMatrix, int *, int *, double *, double *, double **)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {
        a = (*A).a as *mut libc::c_double;
    }
    counts = calloc(
        n as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if *assignment.offset(i as isize) >= 0 as libc::c_int && *assignment.offset(i as isize) < n
        {
        } else {
            __assert_fail(
                b"assignment[i] >= 0 && assignment[i] < n\0" as *const u8 as *const libc::c_char,
                b"mq.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                    b"double get_mq(SparseMatrix, int *, int *, double *, double *, double **)\0",
                ))
                .as_ptr(),
            );
        }
        if *counts.offset(*assignment.offset(i as isize) as isize) == 0 as libc::c_int {
            ncluster += 1;
        }
        let ref mut fresh0 = *counts.offset(*assignment.offset(i as isize) as isize);
        *fresh0 += 1;
        i += 1;
    }
    k = ncluster;
    if ncluster <= n {
    } else {
        __assert_fail(
            b"ncluster <= n\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                b"double get_mq(SparseMatrix, int *, int *, double *, double *, double **)\0",
            ))
            .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < n {
        if *assignment.offset(i as isize) < ncluster {
        } else {
            __assert_fail(
                b"assignment[i] < ncluster\0" as *const u8 as *const libc::c_char,
                b"mq.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 73], &[libc::c_char; 73]>(
                    b"double get_mq(SparseMatrix, int *, int *, double *, double *, double **)\0",
                ))
                .as_ptr(),
            );
        }
        c = *assignment.offset(i as isize);
        Vi = *counts.offset(c as isize) as libc::c_double;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            if !(jj >= i) {
                if *assignment.offset(jj as isize) < ncluster {
                } else {
                    __assert_fail(
                        b"assignment[jj] < ncluster\0" as *const u8
                            as *const libc::c_char,
                        b"mq.c\0" as *const u8 as *const libc::c_char,
                        105 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 73],
                            &[libc::c_char; 73],
                        >(
                            b"double get_mq(SparseMatrix, int *, int *, double *, double *, double **)\0",
                        ))
                            .as_ptr(),
                    );
                }
                Vj = *counts.offset(*assignment.offset(jj as isize) as isize) as libc::c_double;
                if *assignment.offset(jj as isize) == c {
                    if !a.is_null() {
                        mq_in += *a.offset(j as isize) / (Vi * Vi);
                    } else {
                        mq_in += 1.0f64 / (Vi * Vi);
                    }
                } else if !a.is_null() {
                    mq_out += *a.offset(j as isize) / (Vi * Vj);
                } else {
                    mq_out += 1.0f64 / (Vi * Vj);
                }
            }
            j += 1;
        }
        i += 1;
    }
    dout = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        *dout.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            if !(jj == i) {
                if !a.is_null() {
                    *dout.offset(i as isize) += *a.offset(j as isize)
                        / *counts.offset(*assignment.offset(jj as isize) as isize)
                            as libc::c_double;
                } else {
                    *dout.offset(i as isize) += 1.0f64
                        / *counts.offset(*assignment.offset(jj as isize) as isize)
                            as libc::c_double;
                }
            }
            j += 1;
        }
        i += 1;
    }
    *ncluster0 = k;
    *mq_in0 = mq_in;
    *mq_out0 = mq_out;
    *dout0 = dout;
    free(counts as *mut libc::c_void);
    if k > 1 as libc::c_int {
        return 2 as libc::c_int as libc::c_double
            * (mq_in / k as libc::c_double
                - mq_out / (k * (k - 1 as libc::c_int)) as libc::c_double);
    } else {
        return 2 as libc::c_int as libc::c_double * mq_in;
    };
}
unsafe extern "C" fn Multilevel_MQ_Clustering_init(
    mut A: SparseMatrix,
    mut level: libc::c_int,
) -> Multilevel_MQ_Clustering {
    let mut grid: Multilevel_MQ_Clustering = 0 as *mut Multilevel_MQ_Clustering_struct;
    let mut n: libc::c_int = (*A).n;
    let mut i: libc::c_int = 0;
    let mut matching: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {
    } else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"Multilevel_MQ_Clustering Multilevel_MQ_Clustering_init(SparseMatrix, int)\0",
            ))
            .as_ptr(),
        );
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"Multilevel_MQ_Clustering Multilevel_MQ_Clustering_init(SparseMatrix, int)\0",
            ))
            .as_ptr(),
        );
    }
    if A.is_null() {
        return 0 as Multilevel_MQ_Clustering;
    }
    if (*A).m == n {
    } else {
        __assert_fail(
            b"A->m == n\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"Multilevel_MQ_Clustering Multilevel_MQ_Clustering_init(SparseMatrix, int)\0",
            ))
            .as_ptr(),
        );
    }
    grid = malloc(::std::mem::size_of::<Multilevel_MQ_Clustering_struct>() as libc::c_ulong)
        as Multilevel_MQ_Clustering;
    (*grid).level = level;
    (*grid).n = n;
    let ref mut fresh1 = (*grid).A;
    *fresh1 = A;
    let ref mut fresh2 = (*grid).P;
    *fresh2 = 0 as SparseMatrix;
    let ref mut fresh3 = (*grid).R;
    *fresh3 = 0 as SparseMatrix;
    let ref mut fresh4 = (*grid).next;
    *fresh4 = 0 as Multilevel_MQ_Clustering;
    let ref mut fresh5 = (*grid).prev;
    *fresh5 = 0 as Multilevel_MQ_Clustering;
    (*grid).delete_top_level_A = 0 as libc::c_int;
    let ref mut fresh6 = (*grid).matching;
    *fresh6 = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_int;
    matching = *fresh6;
    let ref mut fresh7 = (*grid).deg_intra;
    *fresh7 = 0 as *mut libc::c_double;
    let ref mut fresh8 = (*grid).dout;
    *fresh8 = 0 as *mut libc::c_double;
    let ref mut fresh9 = (*grid).wgt;
    *fresh9 = 0 as *mut libc::c_double;
    if level == 0 as libc::c_int {
        let mut mq: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut mq_in: libc::c_double = 0.;
        let mut mq_out: libc::c_double = 0.;
        let mut n_0: libc::c_int = (*A).n;
        let mut ncluster: libc::c_int = 0;
        let mut deg_intra: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut wgt: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut dout: *mut libc::c_double = 0 as *mut libc::c_double;
        let ref mut fresh10 = (*grid).deg_intra;
        *fresh10 = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(n_0 as libc::c_ulong),
        ) as *mut libc::c_double;
        deg_intra = (*grid).deg_intra;
        let ref mut fresh11 = (*grid).wgt;
        *fresh11 = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(n_0 as libc::c_ulong),
        ) as *mut libc::c_double;
        wgt = (*grid).wgt;
        i = 0 as libc::c_int;
        while i < n_0 {
            *deg_intra.offset(i as isize) = 0 as libc::c_int as libc::c_double;
            *wgt.offset(i as isize) = 1.0f64;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < n_0 {
            *matching.offset(i as isize) = i;
            i += 1;
        }
        mq = get_mq(
            A,
            matching,
            &mut ncluster,
            &mut mq_in,
            &mut mq_out,
            &mut dout,
        );
        fprintf(
            stderr,
            b"ncluster = %d, mq = %f\n\0" as *const u8 as *const libc::c_char,
            ncluster,
            mq,
        );
        (*grid).mq = mq;
        (*grid).mq_in = mq_in;
        (*grid).mq_out = mq_out;
        let ref mut fresh12 = (*grid).dout;
        *fresh12 = dout;
        (*grid).ncluster = ncluster;
    }
    return grid;
}
unsafe extern "C" fn Multilevel_MQ_Clustering_delete(mut grid: Multilevel_MQ_Clustering) {
    if grid.is_null() {
        return;
    }
    if !((*grid).A).is_null() {
        if (*grid).level == 0 as libc::c_int {
            if (*grid).delete_top_level_A != 0 {
                SparseMatrix_delete((*grid).A);
            }
        } else {
            SparseMatrix_delete((*grid).A);
        }
    }
    SparseMatrix_delete((*grid).P);
    SparseMatrix_delete((*grid).R);
    free((*grid).matching as *mut libc::c_void);
    free((*grid).deg_intra as *mut libc::c_void);
    free((*grid).dout as *mut libc::c_void);
    free((*grid).wgt as *mut libc::c_void);
    Multilevel_MQ_Clustering_delete((*grid).next);
    free(grid as *mut libc::c_void);
}
unsafe extern "C" fn Multilevel_MQ_Clustering_establish(
    mut grid: Multilevel_MQ_Clustering,
    mut maxcluster: libc::c_int,
) -> Multilevel_MQ_Clustering {
    let mut matching: *mut libc::c_int = (*grid).matching;
    let mut A: SparseMatrix = (*grid).A;
    let mut n: libc::c_int = (*grid).n;
    let mut level: libc::c_int = (*grid).level;
    let mut nc: libc::c_int = 0 as libc::c_int;
    let mut nclusters: libc::c_int = n;
    let mut mq: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mq_in: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mq_out: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mq_new: libc::c_double = 0.;
    let mut mq_in_new: libc::c_double = 0.;
    let mut mq_out_new: libc::c_double = 0.;
    let mut mq_max: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mq_in_max: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mq_out_max: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut amax: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut deg_intra: *mut libc::c_double = (*grid).deg_intra;
    let mut wgt: *mut libc::c_double = (*grid).wgt;
    let mut deg_intra_new: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut wgt_new: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut jc: libc::c_int = 0;
    let mut jmax: libc::c_int = 0;
    let mut deg_inter: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gain: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dout: *mut libc::c_double = (*grid).dout;
    let mut dout_new: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut deg_in_i: libc::c_double = 0.;
    let mut deg_in_j: libc::c_double = 0.;
    let mut wgt_i: libc::c_double = 0.;
    let mut wgt_j: libc::c_double = 0.;
    let mut a_ij: libc::c_double = 0.;
    let mut dout_i: libc::c_double = 0.;
    let mut dout_j: libc::c_double = 0.;
    let mut dout_max: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut wgt_jmax: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut maxgain: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut total_gain: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut neighbors: *mut SingleLinkedList = 0 as *mut SingleLinkedList;
    let mut lst: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    neighbors = malloc(
        (::std::mem::size_of::<SingleLinkedList>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut SingleLinkedList;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh13 = *neighbors.offset(i as isize);
        *fresh13 = 0 as SingleLinkedList;
        i += 1;
    }
    mq = (*grid).mq;
    mq_in = (*grid).mq_in;
    mq_out = (*grid).mq_out;
    deg_intra_new = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    wgt_new = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    deg_inter = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    mask = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_int;
    dout_new = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    if n == (*A).n {
    } else {
        __assert_fail(
            b"n == A->n\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"Multilevel_MQ_Clustering Multilevel_MQ_Clustering_establish(Multilevel_MQ_Clustering, int)\0",
            ))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < n {
        *matching.offset(i as isize) = UNMATCHED as libc::c_int;
        i += 1;
    }
    a = (*A).a as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        if !(*matching.offset(i as isize) != UNMATCHED as libc::c_int) {
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                jj = *ja.offset(j as isize);
                if !(jj == i) {
                    jc = *matching.offset(jj as isize);
                    if jc != UNMATCHED as libc::c_int {
                        if *mask.offset(jc as isize) != i {
                            *mask.offset(jc as isize) = i;
                            *deg_inter.offset(jc as isize) = *a.offset(j as isize);
                        } else {
                            *deg_inter.offset(jc as isize) += *a.offset(j as isize);
                        }
                    }
                }
                j += 1;
            }
            deg_in_i = *deg_intra.offset(i as isize);
            wgt_i = *wgt.offset(i as isize);
            dout_i = *dout.offset(i as isize);
            maxgain = 0 as libc::c_int as libc::c_double;
            jmax = -(1 as libc::c_int);
            let mut current_block_70: u64;
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                jj = *ja.offset(j as isize);
                if !(jj == i) {
                    jc = *matching.offset(jj as isize);
                    if jc == UNMATCHED as libc::c_int {
                        a_ij = *a.offset(j as isize);
                        wgt_j = *wgt.offset(jj as isize);
                        deg_in_j = *deg_intra.offset(jj as isize);
                        dout_j = *dout.offset(jj as isize);
                        current_block_70 = 3392087639489470149;
                    } else if *deg_inter.offset(jc as isize) < 0 as libc::c_int as libc::c_double {
                        current_block_70 = 1345366029464561491;
                    } else {
                        a_ij = *deg_inter.offset(jc as isize);
                        wgt_j = *wgt_new.offset(jc as isize);
                        *deg_inter.offset(jc as isize) = -(1 as libc::c_int) as libc::c_double;
                        deg_in_j = *deg_intra_new.offset(jc as isize);
                        dout_j = *dout_new.offset(jc as isize);
                        current_block_70 = 3392087639489470149;
                    }
                    match current_block_70 {
                        1345366029464561491 => {}
                        _ => {
                            mq_in_new = mq_in
                                - deg_in_i / pow(wgt_i, 2 as libc::c_int as libc::c_double)
                                - deg_in_j / pow(wgt_j, 2 as libc::c_int as libc::c_double)
                                + (deg_in_i + deg_in_j + a_ij)
                                    / pow(wgt_i + wgt_j, 2 as libc::c_int as libc::c_double);
                            mq_out_new = mq_out - dout_i / wgt_i - dout_j / wgt_j
                                + (dout_i + dout_j) / (wgt_i + wgt_j);
                            if nclusters > 2 as libc::c_int {
                                mq_new = 2 as libc::c_int as libc::c_double
                                    * (mq_in_new
                                        / (nclusters - 1 as libc::c_int) as libc::c_double
                                        - mq_out_new
                                            / ((nclusters - 1 as libc::c_int)
                                                * (nclusters - 2 as libc::c_int))
                                                as libc::c_double);
                            } else {
                                mq_new = 2 as libc::c_int as libc::c_double * mq_in_new
                                    / (nclusters - 1 as libc::c_int) as libc::c_double;
                            }
                            gain = mq_new - mq;
                            if Verbose != 0 {
                                fprintf(
                                    stderr,
                                    b"gain in merging node %d with node %d = %f-%f = %f\n\0"
                                        as *const u8
                                        as *const libc::c_char,
                                    i,
                                    jj,
                                    mq,
                                    mq_new,
                                    gain,
                                );
                            }
                            if j == *ia.offset(i as isize) || gain > maxgain {
                                maxgain = gain;
                                jmax = jj;
                                amax = a_ij;
                                dout_max = dout_j;
                                wgt_jmax = wgt_j;
                                mq_max = mq_new;
                                mq_in_max = mq_in_new;
                                mq_out_max = mq_out_new;
                            }
                        }
                    }
                }
                j += 1;
            }
            if maxgain > 0 as libc::c_int as libc::c_double
                || nc >= 1 as libc::c_int && nc > maxcluster
            {
                total_gain += maxgain;
                jc = *matching.offset(jmax as isize);
                if jc == UNMATCHED as libc::c_int {
                    fprintf(
                        stderr,
                        b"maxgain=%f, merge %d, %d\n\0" as *const u8 as *const libc::c_char,
                        maxgain,
                        i,
                        jmax,
                    );
                    let ref mut fresh14 = *neighbors.offset(nc as isize);
                    *fresh14 = SingleLinkedList_new_int(jmax);
                    let ref mut fresh15 = *neighbors.offset(nc as isize);
                    *fresh15 = SingleLinkedList_prepend_int(*neighbors.offset(nc as isize), i);
                    *dout_new.offset(nc as isize) = dout_i + dout_max;
                    let ref mut fresh16 = *matching.offset(jmax as isize);
                    *fresh16 = nc;
                    *matching.offset(i as isize) = *fresh16;
                    *wgt_new.offset(nc as isize) =
                        *wgt.offset(i as isize) + *wgt.offset(jmax as isize);
                    *deg_intra_new.offset(nc as isize) =
                        *deg_intra.offset(i as isize) + *deg_intra.offset(jmax as isize) + amax;
                    nc += 1;
                } else {
                    fprintf(
                        stderr,
                        b"maxgain=%f, merge with existing cluster %d, %d\n\0" as *const u8
                            as *const libc::c_char,
                        maxgain,
                        i,
                        jc,
                    );
                    let ref mut fresh17 = *neighbors.offset(jc as isize);
                    *fresh17 = SingleLinkedList_prepend_int(*neighbors.offset(jc as isize), i);
                    *dout_new.offset(jc as isize) = dout_i + dout_max;
                    *wgt_new.offset(jc as isize) += *wgt.offset(i as isize);
                    *matching.offset(i as isize) = jc;
                    *deg_intra_new.offset(jc as isize) += *deg_intra.offset(i as isize) + amax;
                }
                mq = mq_max;
                mq_in = mq_in_max;
                mq_out = mq_out_max;
                nclusters -= 1;
            } else {
                fprintf(
                    stderr,
                    b"gain: %f -- no gain, skip merging node %d\n\0" as *const u8
                        as *const libc::c_char,
                    maxgain,
                    i,
                );
                if maxgain <= 0 as libc::c_int as libc::c_double {
                } else {
                    __assert_fail(
                        b"maxgain <= 0\0" as *const u8 as *const libc::c_char,
                        b"mq.c\0" as *const u8 as *const libc::c_char,
                        399 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 91],
                            &[libc::c_char; 91],
                        >(
                            b"Multilevel_MQ_Clustering Multilevel_MQ_Clustering_establish(Multilevel_MQ_Clustering, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                let ref mut fresh18 = *neighbors.offset(nc as isize);
                *fresh18 = SingleLinkedList_new_int(i);
                *matching.offset(i as isize) = nc;
                *deg_intra_new.offset(nc as isize) = *deg_intra.offset(i as isize);
                *wgt_new.offset(nc as isize) = *wgt.offset(i as isize);
                nc += 1;
            }
            jc = *matching.offset(i as isize);
            lst = *neighbors.offset(jc as isize);
            loop {
                *mask.offset(*(SingleLinkedList_get_data(lst) as *mut libc::c_int) as isize) =
                    n + i;
                lst = SingleLinkedList_get_next(lst);
                if lst.is_null() {
                    break;
                }
            }
            lst = *neighbors.offset(jc as isize);
            loop {
                k = *(SingleLinkedList_get_data(lst) as *mut libc::c_int);
                j = *ia.offset(k as isize);
                while j < *ia.offset((k + 1 as libc::c_int) as isize) {
                    jj = *ja.offset(j as isize);
                    if !(*mask.offset(jj as isize) == n + i) {
                        jc = *matching.offset(jj as isize);
                        if jc == UNMATCHED as libc::c_int {
                            if k == i {
                                *dout.offset(jj as isize) += -*a.offset(j as isize) / wgt_i
                                    + *a.offset(j as isize) / (wgt_i + wgt_jmax);
                            } else {
                                *dout.offset(jj as isize) += -*a.offset(j as isize) / wgt_jmax
                                    + *a.offset(j as isize) / (wgt_i + wgt_jmax);
                            }
                        } else if k == i {
                            *dout_new.offset(jc as isize) += -*a.offset(j as isize) / wgt_i
                                + *a.offset(j as isize) / (wgt_i + wgt_jmax);
                        } else {
                            *dout_new.offset(jc as isize) += -*a.offset(j as isize) / wgt_jmax
                                + *a.offset(j as isize) / (wgt_i + wgt_jmax);
                        }
                    }
                    j += 1;
                }
                lst = SingleLinkedList_get_next(lst);
                if lst.is_null() {
                    break;
                }
            }
        }
        i += 1;
    }
    fprintf(
        stderr,
        b"verbose=%d\n\0" as *const u8 as *const libc::c_char,
        Verbose as libc::c_int,
    );
    if Verbose != 0 {
        fprintf(
            stderr,
            b"mq = %f new mq = %f level = %d, n = %d, nc = %d, gain = %g, mq_in = %f, mq_out = %f\n\0"
                as *const u8 as *const libc::c_char,
            mq,
            mq + total_gain,
            level,
            n,
            nc,
            total_gain,
            mq_in,
            mq_out,
        );
    }
    if nc >= 1 as libc::c_int && (total_gain > 0 as libc::c_int as libc::c_double || nc < n) {
        let mut P: SparseMatrix = 0 as *mut SparseMatrix_struct;
        let mut R: SparseMatrix = 0 as *mut SparseMatrix_struct;
        let mut R0: SparseMatrix = 0 as *mut SparseMatrix_struct;
        let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
        let mut cA: SparseMatrix = 0 as *mut SparseMatrix_struct;
        let mut one: libc::c_double = 1.0f64;
        let mut cgrid: Multilevel_MQ_Clustering = 0 as *mut Multilevel_MQ_Clustering_struct;
        R0 = SparseMatrix_new(
            nc,
            n,
            1 as libc::c_int,
            MATRIX_TYPE_REAL as libc::c_int,
            FORMAT_COORD as libc::c_int,
        );
        i = 0 as libc::c_int;
        while i < n {
            jj = *matching.offset(i as isize);
            SparseMatrix_coordinate_form_add_entry(
                R0,
                jj,
                i,
                &mut one as *mut libc::c_double as *mut libc::c_void,
            );
            i += 1;
        }
        R = SparseMatrix_from_coordinate_format(R0);
        SparseMatrix_delete(R0);
        P = SparseMatrix_transpose(R);
        B = SparseMatrix_multiply(R, A);
        if B.is_null() {
            free(deg_intra_new as *mut libc::c_void);
            free(wgt_new as *mut libc::c_void);
            free(dout_new as *mut libc::c_void);
        } else {
            cA = SparseMatrix_multiply(B, P);
            if cA.is_null() {
                free(deg_intra_new as *mut libc::c_void);
                free(wgt_new as *mut libc::c_void);
                free(dout_new as *mut libc::c_void);
            } else {
                SparseMatrix_delete(B);
                let ref mut fresh19 = (*grid).P;
                *fresh19 = P;
                let ref mut fresh20 = (*grid).R;
                *fresh20 = R;
                level += 1;
                cgrid = Multilevel_MQ_Clustering_init(cA, level);
                deg_intra_new = realloc(
                    deg_intra_new as *mut libc::c_void,
                    (nc as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
                ) as *mut libc::c_double;
                wgt_new = realloc(
                    wgt_new as *mut libc::c_void,
                    (nc as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
                ) as *mut libc::c_double;
                let ref mut fresh21 = (*cgrid).deg_intra;
                *fresh21 = deg_intra_new;
                (*cgrid).mq = (*grid).mq + total_gain;
                let ref mut fresh22 = (*cgrid).wgt;
                *fresh22 = wgt_new;
                dout_new = realloc(
                    dout_new as *mut libc::c_void,
                    (nc as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
                ) as *mut libc::c_double;
                let ref mut fresh23 = (*cgrid).dout;
                *fresh23 = dout_new;
                cgrid = Multilevel_MQ_Clustering_establish(cgrid, maxcluster);
                let ref mut fresh24 = (*grid).next;
                *fresh24 = cgrid;
                let ref mut fresh25 = (*cgrid).prev;
                *fresh25 = grid;
            }
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            *matching.offset(i as isize) = i;
            i += 1;
        }
        free(deg_intra_new as *mut libc::c_void);
        free(wgt_new as *mut libc::c_void);
        free(dout_new as *mut libc::c_void);
    }
    i = 0 as libc::c_int;
    while i < n {
        SingleLinkedList_delete(
            *neighbors.offset(i as isize),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        i += 1;
    }
    free(neighbors as *mut libc::c_void);
    free(deg_inter as *mut libc::c_void);
    free(mask as *mut libc::c_void);
    return grid;
}
unsafe extern "C" fn Multilevel_MQ_Clustering_new(
    mut A0: SparseMatrix,
    mut maxcluster: libc::c_int,
) -> Multilevel_MQ_Clustering {
    let mut grid: Multilevel_MQ_Clustering = 0 as *mut Multilevel_MQ_Clustering_struct;
    let mut A: SparseMatrix = A0;
    if maxcluster <= 0 as libc::c_int {
        maxcluster = (*A).m;
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) == 0
        || (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int
    {
        A = SparseMatrix_get_real_adjacency_matrix_symmetrized(A);
    }
    grid = Multilevel_MQ_Clustering_init(A, 0 as libc::c_int);
    grid = Multilevel_MQ_Clustering_establish(grid, maxcluster);
    if A != A0 {
        (*grid).delete_top_level_A = 1 as libc::c_int;
    }
    return grid;
}
unsafe extern "C" fn hierachical_mq_clustering(
    mut A: SparseMatrix,
    mut maxcluster: libc::c_int,
    mut nclusters: *mut libc::c_int,
    mut assignment: *mut *mut libc::c_int,
    mut mq: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut grid: Multilevel_MQ_Clustering = 0 as *mut Multilevel_MQ_Clustering_struct;
    let mut cgrid: Multilevel_MQ_Clustering = 0 as *mut Multilevel_MQ_Clustering_struct;
    let mut matching: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut P: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    if (*A).m == (*A).n {
    } else {
        __assert_fail(
            b"A->m == A->n\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            551 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void hierachical_mq_clustering(SparseMatrix, int, int *, int **, double *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *mq = 0.0f64;
    *flag = 0 as libc::c_int;
    grid = Multilevel_MQ_Clustering_new(A, maxcluster);
    cgrid = grid;
    while !((*cgrid).next).is_null() {
        cgrid = (*cgrid).next;
    }
    u = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*cgrid).n as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < (*cgrid).n {
        *u.offset(i as isize) = *((*cgrid).matching).offset(i as isize) as libc::c_double;
        i += 1;
    }
    *nclusters = (*cgrid).n;
    *mq = (*cgrid).mq;
    while !((*cgrid).prev).is_null() {
        let mut v: *mut libc::c_double = 0 as *mut libc::c_double;
        P = (*(*cgrid).prev).P;
        SparseMatrix_multiply_vector(P, u, &mut v);
        free(u as *mut libc::c_void);
        u = v;
        cgrid = (*cgrid).prev;
    }
    if !(*assignment).is_null() {
        matching = *assignment;
    } else {
        matching = malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((*grid).n as libc::c_ulong),
        ) as *mut libc::c_int;
        *assignment = matching;
    }
    i = 0 as libc::c_int;
    while i < (*grid).n {
        *matching.offset(i as isize) = *u.offset(i as isize) as libc::c_int;
        i += 1;
    }
    free(u as *mut libc::c_void);
    Multilevel_MQ_Clustering_delete(grid);
}
#[no_mangle]
pub unsafe extern "C" fn mq_clustering(
    mut A: SparseMatrix,
    mut inplace: libc::c_int,
    mut maxcluster: libc::c_int,
    mut use_value: libc::c_int,
    mut nclusters: *mut libc::c_int,
    mut assignment: *mut *mut libc::c_int,
    mut mq: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    *flag = 0 as libc::c_int;
    if (*A).m == (*A).n {
    } else {
        __assert_fail(
            b"A->m == A->n\0" as *const u8 as *const libc::c_char,
            b"mq.c\0" as *const u8 as *const libc::c_char,
            608 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"void mq_clustering(SparseMatrix, int, int, int, int *, int **, double *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    B = SparseMatrix_symmetrize(A, 0 as libc::c_int != 0);
    if inplace == 0 && B == A {
        B = SparseMatrix_copy(A);
    }
    B = SparseMatrix_remove_diagonal(B);
    if (*B).type_0 != MATRIX_TYPE_REAL as libc::c_int || use_value == 0 {
        B = SparseMatrix_set_entries_to_real_one(B);
    }
    hierachical_mq_clustering(B, maxcluster, nclusters, assignment, mq, flag);
    if B != A {
        SparseMatrix_delete(B);
    }
}
