#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn SparseMatrix_from_coordinate_format(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_multiply(A: SparseMatrix, B: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_coordinate_form_add_entry(
        A: SparseMatrix,
        irn: libc::c_int,
        jcn: libc::c_int,
        val: *mut libc::c_void,
    ) -> SparseMatrix;
    fn SparseMatrix_is_symmetric(
        A: SparseMatrix,
        test_pattern_symmetry_only: bool,
    ) -> libc::c_int;
    fn SparseMatrix_transpose(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_symmetrize(
        A: SparseMatrix,
        pattern_symmetric_only: bool,
    ) -> SparseMatrix;
    fn SparseMatrix_multiply_vector(
        A: SparseMatrix,
        v: *mut libc::c_double,
        res: *mut *mut libc::c_double,
    );
    fn SparseMatrix_remove_diagonal(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(
        A: SparseMatrix,
    ) -> SparseMatrix;
    fn SparseMatrix_copy(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_set_entries_to_real_one(A: SparseMatrix) -> SparseMatrix;
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
pub struct Multilevel_Modularity_Clustering_struct {
    pub level: libc::c_int,
    pub n: libc::c_int,
    pub A: SparseMatrix,
    pub P: SparseMatrix,
    pub R: SparseMatrix,
    pub next: Multilevel_Modularity_Clustering,
    pub prev: Multilevel_Modularity_Clustering,
    pub delete_top_level_A: libc::c_int,
    pub matching: *mut libc::c_int,
    pub modularity: libc::c_double,
    pub deg_total: libc::c_double,
    pub deg: *mut libc::c_double,
    pub agglomerate_regardless: libc::c_int,
}
pub type Multilevel_Modularity_Clustering = *mut Multilevel_Modularity_Clustering_struct;
unsafe extern "C" fn Multilevel_Modularity_Clustering_init(
    mut A: SparseMatrix,
    mut level: libc::c_int,
) -> Multilevel_Modularity_Clustering {
    let mut grid: Multilevel_Modularity_Clustering = 0
        as *mut Multilevel_Modularity_Clustering_struct;
    let mut n: libc::c_int = (*A).n;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {} else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"clustering.c\0" as *const u8 as *const libc::c_char,
            22 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"Multilevel_Modularity_Clustering Multilevel_Modularity_Clustering_init(SparseMatrix, int)\0",
            ))
                .as_ptr(),
        );
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, false)\0" as *const u8 as *const libc::c_char,
            b"clustering.c\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"Multilevel_Modularity_Clustering Multilevel_Modularity_Clustering_init(SparseMatrix, int)\0",
            ))
                .as_ptr(),
        );
    }
    if A.is_null() {
        return 0 as Multilevel_Modularity_Clustering;
    }
    if (*A).m == n {} else {
        __assert_fail(
            b"A->m == n\0" as *const u8 as *const libc::c_char,
            b"clustering.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"Multilevel_Modularity_Clustering Multilevel_Modularity_Clustering_init(SparseMatrix, int)\0",
            ))
                .as_ptr(),
        );
    }
    grid = malloc(
        ::std::mem::size_of::<Multilevel_Modularity_Clustering_struct>() as libc::c_ulong,
    ) as Multilevel_Modularity_Clustering;
    (*grid).level = level;
    (*grid).n = n;
    let ref mut fresh0 = (*grid).A;
    *fresh0 = A;
    let ref mut fresh1 = (*grid).P;
    *fresh1 = 0 as SparseMatrix;
    let ref mut fresh2 = (*grid).R;
    *fresh2 = 0 as SparseMatrix;
    let ref mut fresh3 = (*grid).next;
    *fresh3 = 0 as Multilevel_Modularity_Clustering;
    let ref mut fresh4 = (*grid).prev;
    *fresh4 = 0 as Multilevel_Modularity_Clustering;
    (*grid).delete_top_level_A = 0 as libc::c_int;
    let ref mut fresh5 = (*grid).matching;
    *fresh5 = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_int;
    let ref mut fresh6 = (*grid).deg;
    *fresh6 = 0 as *mut libc::c_double;
    (*grid).agglomerate_regardless = 0 as libc::c_int;
    if level == 0 as libc::c_int {
        let mut modularity: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut ia: *mut libc::c_int = (*A).ia;
        let mut ja: *mut libc::c_int = (*A).ja;
        let mut n_0: libc::c_int = (*A).n;
        let mut deg_total: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut deg: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
        let mut indeg: *mut libc::c_double = 0 as *mut libc::c_double;
        (*grid).deg_total = 0.0f64;
        let ref mut fresh7 = (*grid).deg;
        *fresh7 = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(n_0 as libc::c_ulong),
        ) as *mut libc::c_double;
        deg = (*grid).deg;
        indeg = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(n_0 as libc::c_ulong),
        ) as *mut libc::c_double;
        i = 0 as libc::c_int;
        while i < n_0 {
            *deg.offset(i as isize) = 0 as libc::c_int as libc::c_double;
            *indeg.offset(i as isize) = 0.0f64;
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                *deg.offset(i as isize) += *a.offset(j as isize);
                if *ja.offset(j as isize) == i {
                    *indeg.offset(i as isize) = *a.offset(j as isize);
                }
                j += 1;
            }
            deg_total += *deg.offset(i as isize);
            i += 1;
        }
        if deg_total == 0 as libc::c_int as libc::c_double {
            deg_total = 1 as libc::c_int as libc::c_double;
        }
        i = 0 as libc::c_int;
        while i < n_0 {
            modularity
                += (*indeg.offset(i as isize)
                    - *deg.offset(i as isize) * *deg.offset(i as isize) / deg_total)
                    / deg_total;
            i += 1;
        }
        (*grid).deg_total = deg_total;
        let ref mut fresh8 = (*grid).deg;
        *fresh8 = deg;
        (*grid).modularity = modularity;
        free(indeg as *mut libc::c_void);
    }
    return grid;
}
unsafe extern "C" fn Multilevel_Modularity_Clustering_delete(
    mut grid: Multilevel_Modularity_Clustering,
) {
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
    free((*grid).deg as *mut libc::c_void);
    Multilevel_Modularity_Clustering_delete((*grid).next);
    free(grid as *mut libc::c_void);
}
unsafe extern "C" fn Multilevel_Modularity_Clustering_establish(
    mut grid: Multilevel_Modularity_Clustering,
    mut ncluster_target: libc::c_int,
) -> Multilevel_Modularity_Clustering {
    let mut current_block: u64;
    let mut matching: *mut libc::c_int = (*grid).matching;
    let mut A: SparseMatrix = (*grid).A;
    let mut n: libc::c_int = (*grid).n;
    let mut level: libc::c_int = (*grid).level;
    let mut nc: libc::c_int = 0 as libc::c_int;
    let mut modularity: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut deg: *mut libc::c_double = (*grid).deg;
    let mut deg_new: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut jc: libc::c_int = 0;
    let mut jmax: libc::c_int = 0;
    let mut inv_deg_total: libc::c_double = 1.0f64 / (*grid).deg_total;
    let mut deg_inter: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gain: libc::c_double = 0.;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut maxgain: libc::c_double = 0.;
    let mut total_gain: libc::c_double = 0 as libc::c_int as libc::c_double;
    modularity = (*grid).modularity;
    deg_new = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    deg_inter = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    mask = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    if n == (*A).n {} else {
        __assert_fail(
            b"n == A->n\0" as *const u8 as *const libc::c_char,
            b"clustering.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 115],
                &[libc::c_char; 115],
            >(
                b"Multilevel_Modularity_Clustering Multilevel_Modularity_Clustering_establish(Multilevel_Modularity_Clustering, int)\0",
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
            maxgain = 0 as libc::c_int as libc::c_double;
            jmax = -(1 as libc::c_int);
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                jj = *ja.offset(j as isize);
                if !(jj == i) {
                    jc = *matching.offset(jj as isize);
                    if jc == UNMATCHED as libc::c_int {
                        gain = (2 as libc::c_int as libc::c_double
                            * *a.offset(j as isize)
                            - 2 as libc::c_int as libc::c_double
                                * *deg.offset(i as isize) * *deg.offset(jj as isize)
                                * inv_deg_total) * inv_deg_total;
                    } else if *deg_inter.offset(jc as isize)
                            > 0 as libc::c_int as libc::c_double
                        {
                        gain = (2 as libc::c_int as libc::c_double
                            * *deg_inter.offset(jc as isize)
                            - 2 as libc::c_int as libc::c_double
                                * *deg.offset(i as isize) * *deg_new.offset(jc as isize)
                                * inv_deg_total) * inv_deg_total;
                        *deg_inter
                            .offset(jc as isize) = -(1 as libc::c_int) as libc::c_double;
                    } else {
                        gain = -(1 as libc::c_int) as libc::c_double;
                    }
                    if jmax < 0 as libc::c_int || gain > maxgain {
                        maxgain = gain;
                        jmax = jj;
                    }
                }
                j += 1;
            }
            if maxgain > 0 as libc::c_int as libc::c_double
                || (*grid).agglomerate_regardless != 0
            {
                total_gain += maxgain;
                jc = *matching.offset(jmax as isize);
                if jc == UNMATCHED as libc::c_int {
                    let ref mut fresh9 = *matching.offset(jmax as isize);
                    *fresh9 = nc;
                    *matching.offset(i as isize) = *fresh9;
                    *deg_new
                        .offset(
                            nc as isize,
                        ) = *deg.offset(i as isize) + *deg.offset(jmax as isize);
                    nc += 1;
                } else {
                    *deg_new.offset(jc as isize) += *deg.offset(i as isize);
                    *matching.offset(i as isize) = jc;
                }
            } else {
                if maxgain <= 0 as libc::c_int as libc::c_double {} else {
                    __assert_fail(
                        b"maxgain <= 0\0" as *const u8 as *const libc::c_char,
                        b"clustering.c\0" as *const u8 as *const libc::c_char,
                        178 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 115],
                            &[libc::c_char; 115],
                        >(
                            b"Multilevel_Modularity_Clustering Multilevel_Modularity_Clustering_establish(Multilevel_Modularity_Clustering, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *matching.offset(i as isize) = nc;
                *deg_new.offset(nc as isize) = *deg.offset(i as isize);
                nc += 1;
            }
        }
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"modularity = %f new modularity = %f level = %d, n = %d, nc = %d, gain = %g\n\0"
                as *const u8 as *const libc::c_char,
            modularity,
            modularity + total_gain,
            level,
            n,
            nc,
            total_gain,
        );
    }
    if ncluster_target > 0 as libc::c_int {
        if nc <= ncluster_target && n >= ncluster_target {
            if n - ncluster_target > ncluster_target - nc {
                current_block = 10261677128829721533;
            } else if n - ncluster_target <= ncluster_target - nc {
                fprintf(
                    stderr,
                    b"ncluster_target = %d, close to n=%d\n\0" as *const u8
                        as *const libc::c_char,
                    ncluster_target,
                    n,
                );
                i = 0 as libc::c_int;
                while i < n {
                    *matching.offset(i as isize) = i;
                    i += 1;
                }
                free(deg_new as *mut libc::c_void);
                current_block = 6421900725611249331;
            } else {
                current_block = 10261677128829721533;
            }
        } else if n < ncluster_target {
            fprintf(stderr, b"n < target\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < n {
                *matching.offset(i as isize) = i;
                i += 1;
            }
            free(deg_new as *mut libc::c_void);
            current_block = 6421900725611249331;
        } else {
            current_block = 10261677128829721533;
        }
    } else {
        current_block = 10261677128829721533;
    }
    match current_block {
        10261677128829721533 => {
            if nc >= 1 as libc::c_int
                && (total_gain > 0 as libc::c_int as libc::c_double || nc < n)
            {
                let mut P: SparseMatrix = 0 as *mut SparseMatrix_struct;
                let mut R: SparseMatrix = 0 as *mut SparseMatrix_struct;
                let mut R0: SparseMatrix = 0 as *mut SparseMatrix_struct;
                let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
                let mut cA: SparseMatrix = 0 as *mut SparseMatrix_struct;
                let mut one: libc::c_double = 1.0f64;
                let mut cgrid: Multilevel_Modularity_Clustering = 0
                    as *mut Multilevel_Modularity_Clustering_struct;
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
                    free(deg_new as *mut libc::c_void);
                } else {
                    cA = SparseMatrix_multiply(B, P);
                    if cA.is_null() {
                        free(deg_new as *mut libc::c_void);
                    } else {
                        SparseMatrix_delete(B);
                        let ref mut fresh10 = (*grid).P;
                        *fresh10 = P;
                        let ref mut fresh11 = (*grid).R;
                        *fresh11 = R;
                        level += 1;
                        cgrid = Multilevel_Modularity_Clustering_init(cA, level);
                        deg_new = realloc(
                            deg_new as *mut libc::c_void,
                            (nc as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                ),
                        ) as *mut libc::c_double;
                        let ref mut fresh12 = (*cgrid).deg;
                        *fresh12 = deg_new;
                        (*cgrid).modularity = (*grid).modularity + total_gain;
                        (*cgrid).deg_total = (*grid).deg_total;
                        cgrid = Multilevel_Modularity_Clustering_establish(
                            cgrid,
                            ncluster_target,
                        );
                        let ref mut fresh13 = (*grid).next;
                        *fresh13 = cgrid;
                        let ref mut fresh14 = (*cgrid).prev;
                        *fresh14 = grid;
                    }
                }
            } else {
                if ncluster_target > 0 as libc::c_int && nc > ncluster_target
                    && (*grid).agglomerate_regardless == 0
                {
                    (*grid).agglomerate_regardless = 1 as libc::c_int;
                    free(deg_inter as *mut libc::c_void);
                    free(mask as *mut libc::c_void);
                    free(deg_new as *mut libc::c_void);
                    return Multilevel_Modularity_Clustering_establish(
                        grid,
                        ncluster_target,
                    );
                }
                i = 0 as libc::c_int;
                while i < n {
                    *matching.offset(i as isize) = i;
                    i += 1;
                }
                free(deg_new as *mut libc::c_void);
            }
        }
        _ => {}
    }
    free(deg_inter as *mut libc::c_void);
    free(mask as *mut libc::c_void);
    return grid;
}
unsafe extern "C" fn Multilevel_Modularity_Clustering_new(
    mut A0: SparseMatrix,
    mut ncluster_target: libc::c_int,
) -> Multilevel_Modularity_Clustering {
    let mut grid: Multilevel_Modularity_Clustering = 0
        as *mut Multilevel_Modularity_Clustering_struct;
    let mut A: SparseMatrix = A0;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) == 0
        || (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int
    {
        A = SparseMatrix_get_real_adjacency_matrix_symmetrized(A);
    }
    grid = Multilevel_Modularity_Clustering_init(A, 0 as libc::c_int);
    grid = Multilevel_Modularity_Clustering_establish(grid, ncluster_target);
    if A != A0 {
        (*grid).delete_top_level_A = 1 as libc::c_int;
    }
    return grid;
}
unsafe extern "C" fn hierachical_modularity_clustering(
    mut A: SparseMatrix,
    mut ncluster_target: libc::c_int,
    mut nclusters: *mut libc::c_int,
    mut assignment: *mut *mut libc::c_int,
    mut modularity: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut grid: Multilevel_Modularity_Clustering = 0
        as *mut Multilevel_Modularity_Clustering_struct;
    let mut cgrid: Multilevel_Modularity_Clustering = 0
        as *mut Multilevel_Modularity_Clustering_struct;
    let mut matching: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut P: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    if (*A).m == (*A).n {} else {
        __assert_fail(
            b"A->m == A->n\0" as *const u8 as *const libc::c_char,
            b"clustering.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void hierachical_modularity_clustering(SparseMatrix, int, int *, int **, double *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    *modularity = 0.0f64;
    *flag = 0 as libc::c_int;
    grid = Multilevel_Modularity_Clustering_new(A, ncluster_target);
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
        *u
            .offset(
                i as isize,
            ) = *((*cgrid).matching).offset(i as isize) as libc::c_double;
        i += 1;
    }
    *nclusters = (*cgrid).n;
    *modularity = (*cgrid).modularity;
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
    Multilevel_Modularity_Clustering_delete(grid);
}
#[no_mangle]
pub unsafe extern "C" fn modularity_clustering(
    mut A: SparseMatrix,
    mut inplace: libc::c_int,
    mut ncluster_target: libc::c_int,
    mut use_value: libc::c_int,
    mut nclusters: *mut libc::c_int,
    mut assignment: *mut *mut libc::c_int,
    mut modularity: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    *flag = 0 as libc::c_int;
    if (*A).m == (*A).n {} else {
        __assert_fail(
            b"A->m == A->n\0" as *const u8 as *const libc::c_char,
            b"clustering.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"void modularity_clustering(SparseMatrix, int, int, int, int *, int **, double *, int *)\0",
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
    hierachical_modularity_clustering(
        B,
        ncluster_target,
        nclusters,
        assignment,
        modularity,
        flag,
    );
    if B != A {
        SparseMatrix_delete(B);
    }
}
