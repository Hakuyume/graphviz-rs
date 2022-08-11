#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn SparseMatrix_symmetrize(
        A: SparseMatrix,
        pattern_symmetric_only: bool,
    ) -> SparseMatrix;
    fn SparseMatrix_is_symmetric(
        A: SparseMatrix,
        test_pattern_symmetry_only: bool,
    ) -> libc::c_int;
    fn SparseMatrix_coordinate_form_add_entry(
        A: SparseMatrix,
        irn: libc::c_int,
        jcn: libc::c_int,
        val: *mut libc::c_void,
    ) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_from_coordinate_format(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Verbose: libc::c_uchar;
    fn vector_ordering(n: libc::c_int, v: *mut libc::c_double, p: *mut *mut libc::c_int);
    fn power_method(
        A: *mut libc::c_void,
        n: libc::c_int,
        K: libc::c_int,
        random_seed: libc::c_int,
        eigv: *mut *mut libc::c_double,
        eigs: *mut libc::c_double,
    );
    fn clock() -> clock_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
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
unsafe extern "C" fn get_local_12_norm(
    mut n: libc::c_int,
    mut i: libc::c_int,
    mut ia: *const libc::c_int,
    mut ja: *const libc::c_int,
    mut p: *const libc::c_int,
    mut norm: *mut libc::c_double,
) {
    let mut j: libc::c_int = 0;
    let mut nz: libc::c_int = 0 as libc::c_int;
    *norm.offset(0 as libc::c_int as isize) = n as libc::c_double;
    *norm.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    j = *ia.offset(i as isize);
    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
        if !(*ja.offset(j as isize) == i) {
            *norm
                .offset(
                    0 as libc::c_int as isize,
                ) = if *norm.offset(0 as libc::c_int as isize)
                < abs(*p.offset(i as isize) - *p.offset(*ja.offset(j as isize) as isize))
                    as libc::c_double
            {
                *norm.offset(0 as libc::c_int as isize)
            } else {
                abs(*p.offset(i as isize) - *p.offset(*ja.offset(j as isize) as isize))
                    as libc::c_double
            };
            nz += 1;
            *norm.offset(1 as libc::c_int as isize)
                += abs(
                    *p.offset(i as isize) - *p.offset(*ja.offset(j as isize) as isize),
                ) as libc::c_double;
        }
        j += 1;
    }
    if nz > 0 as libc::c_int {
        *norm.offset(1 as libc::c_int as isize) /= nz as libc::c_double;
    }
}
unsafe extern "C" fn get_12_norm(
    mut n: libc::c_int,
    mut ia: *mut libc::c_int,
    mut ja: *mut libc::c_int,
    mut p: *mut libc::c_int,
    mut norm: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_double = 0.;
    *norm.offset(0 as libc::c_int as isize) = n as libc::c_double;
    *norm.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    *norm.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        tmp = n as libc::c_double;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(*ja.offset(j as isize) == i) {
                *norm
                    .offset(
                        0 as libc::c_int as isize,
                    ) = if *norm.offset(0 as libc::c_int as isize)
                    < abs(
                        *p.offset(i as isize)
                            - *p.offset(*ja.offset(j as isize) as isize),
                    ) as libc::c_double
                {
                    *norm.offset(0 as libc::c_int as isize)
                } else {
                    abs(
                        *p.offset(i as isize)
                            - *p.offset(*ja.offset(j as isize) as isize),
                    ) as libc::c_double
                };
                *norm.offset(1 as libc::c_int as isize)
                    += abs(
                        *p.offset(i as isize)
                            - *p.offset(*ja.offset(j as isize) as isize),
                    ) as libc::c_double;
                tmp = if tmp
                    < abs(
                        *p.offset(i as isize)
                            - *p.offset(*ja.offset(j as isize) as isize),
                    ) as libc::c_double
                {
                    tmp
                } else {
                    abs(
                        *p.offset(i as isize)
                            - *p.offset(*ja.offset(j as isize) as isize),
                    ) as libc::c_double
                };
                nz += 1;
            }
            j += 1;
        }
        *norm.offset(2 as libc::c_int as isize) += tmp;
        i += 1;
    }
    *norm.offset(2 as libc::c_int as isize) /= n as libc::c_double;
    *norm.offset(1 as libc::c_int as isize) /= nz as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn improve_antibandwidth_by_swapping(
    mut A: SparseMatrix,
    mut p: *mut libc::c_int,
) {
    let mut improved: bool = 1 as libc::c_int != 0;
    let mut cnt: libc::c_int = 1 as libc::c_int;
    let mut n: libc::c_int = (*A).m;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut norm1: [libc::c_double; 3] = [0.; 3];
    let mut norm2: [libc::c_double; 3] = [0.; 3];
    let mut norm11: [libc::c_double; 3] = [0.; 3];
    let mut norm22: [libc::c_double; 3] = [0.; 3];
    let mut pi: libc::c_double = 0.;
    let mut pj: libc::c_double = 0.;
    let mut start: clock_t = clock();
    let mut fp: *mut FILE = 0 as *mut FILE;
    if Verbose != 0 {
        fprintf(
            stderr,
            b"saving timing vs antiband data to timing_greedy\n\0" as *const u8
                as *const libc::c_char,
        );
        fp = fopen(
            b"timing_greedy\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
    }
    if SparseMatrix_is_symmetric(A, 1 as libc::c_int != 0) != 0 {} else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, true)\0" as *const u8 as *const libc::c_char,
            b"country_graph_coloring.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void improve_antibandwidth_by_swapping(SparseMatrix, int *)\0"))
                .as_ptr(),
        );
    }
    while improved {
        improved = 0 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i < n {
            get_local_12_norm(n, i, ia, ja, p, norm1.as_mut_ptr());
            j = 0 as libc::c_int;
            while j < n {
                if !(j == i) {
                    get_local_12_norm(n, j, ia, ja, p, norm2.as_mut_ptr());
                    pi = *p.offset(i as isize) as libc::c_double;
                    pj = *p.offset(j as isize) as libc::c_double;
                    *p.offset(i as isize) = pj as libc::c_int;
                    *p.offset(j as isize) = pi as libc::c_int;
                    get_local_12_norm(n, i, ia, ja, p, norm11.as_mut_ptr());
                    get_local_12_norm(n, j, ia, ja, p, norm22.as_mut_ptr());
                    if fmin(
                        norm11[0 as libc::c_int as usize],
                        norm22[0 as libc::c_int as usize],
                    )
                        > fmin(
                            norm1[0 as libc::c_int as usize],
                            norm2[0 as libc::c_int as usize],
                        )
                    {
                        improved = 1 as libc::c_int != 0;
                        norm1[0 as libc::c_int
                            as usize] = norm11[0 as libc::c_int as usize];
                        norm1[1 as libc::c_int
                            as usize] = norm11[1 as libc::c_int as usize];
                    } else {
                        *p.offset(i as isize) = pi as libc::c_int;
                        *p.offset(j as isize) = pj as libc::c_int;
                    }
                }
                j += 1;
            }
            if i % 100 as libc::c_int == 0 as libc::c_int && Verbose as libc::c_int != 0
            {
                get_12_norm(n, ia, ja, p, norm1.as_mut_ptr());
                fprintf(
                    fp,
                    b"%f %f %f\n\0" as *const u8 as *const libc::c_char,
                    (clock() - start) as libc::c_double
                        / 1000000 as libc::c_int as __clock_t as libc::c_double,
                    norm1[0 as libc::c_int as usize],
                    norm1[2 as libc::c_int as usize],
                );
            }
            i += 1;
        }
        if Verbose != 0 {
            get_12_norm(n, ia, ja, p, norm1.as_mut_ptr());
            let fresh0 = cnt;
            cnt = cnt + 1;
            fprintf(
                stderr,
                b"[%d] aband = %f, aband_avg = %f\n\0" as *const u8
                    as *const libc::c_char,
                fresh0,
                norm1[0 as libc::c_int as usize],
                norm1[2 as libc::c_int as usize],
            );
            fprintf(
                fp,
                b"%f %f %f\n\0" as *const u8 as *const libc::c_char,
                (clock() - start) as libc::c_double
                    / 1000000 as libc::c_int as __clock_t as libc::c_double,
                norm1[0 as libc::c_int as usize],
                norm1[2 as libc::c_int as usize],
            );
        }
    }
    if !fp.is_null() {
        fclose(fp);
    }
}
unsafe extern "C" fn country_graph_coloring_internal(
    mut seed: libc::c_int,
    mut A: SparseMatrix,
    mut p: *mut *mut libc::c_int,
) {
    let mut n: libc::c_int = (*A).m;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut L: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut A2: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut a: libc::c_int = -(1 as libc::c_int);
    let mut nrow: libc::c_double = 0.;
    let mut v: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut norm1: [libc::c_double; 3] = [0.; 3];
    let mut start: clock_t = 0;
    let mut start2: clock_t = 0;
    start = clock();
    if (*A).m == (*A).n {} else {
        __assert_fail(
            b"A->m == A->n\0" as *const u8 as *const libc::c_char,
            b"country_graph_coloring.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void country_graph_coloring_internal(int, SparseMatrix, int **)\0"))
                .as_ptr(),
        );
    }
    A2 = SparseMatrix_symmetrize(A, 1 as libc::c_int != 0);
    ia = (*A2).ia;
    ja = (*A2).ja;
    L = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < n {
        nrow = 0.0f64;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            if jj != i {
                nrow += 1.;
                L = SparseMatrix_coordinate_form_add_entry(
                    L,
                    i,
                    jj,
                    &mut a as *mut libc::c_int as *mut libc::c_void,
                );
            }
            j += 1;
        }
        L = SparseMatrix_coordinate_form_add_entry(
            L,
            i,
            i,
            &mut nrow as *mut libc::c_double as *mut libc::c_void,
        );
        i += 1;
    }
    L = SparseMatrix_from_coordinate_format(L);
    let mut eig: libc::c_double = 0.;
    power_method(
        L as *mut libc::c_void,
        (*L).n,
        1 as libc::c_int,
        seed,
        &mut v,
        &mut eig,
    );
    vector_ordering(n, v, p);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"cpu time for spectral ordering (before greedy) = %f\n\0" as *const u8
                as *const libc::c_char,
            (clock() - start) as libc::c_double
                / 1000000 as libc::c_int as __clock_t as libc::c_double,
        );
    }
    start2 = clock();
    improve_antibandwidth_by_swapping(A2, *p);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"cpu time for greedy refinement = %f\n\0" as *const u8
                as *const libc::c_char,
            (clock() - start2) as libc::c_double
                / 1000000 as libc::c_int as __clock_t as libc::c_double,
        );
        fprintf(
            stderr,
            b"cpu time for spectral + greedy = %f\n\0" as *const u8
                as *const libc::c_char,
            (clock() - start) as libc::c_double
                / 1000000 as libc::c_int as __clock_t as libc::c_double,
        );
    }
    get_12_norm(n, ia, ja, *p, norm1.as_mut_ptr());
    if A2 != A {
        SparseMatrix_delete(A2);
    }
    SparseMatrix_delete(L);
}
#[no_mangle]
pub unsafe extern "C" fn country_graph_coloring(
    mut seed: libc::c_int,
    mut A: SparseMatrix,
    mut p: *mut *mut libc::c_int,
) {
    country_graph_coloring_internal(seed, A, p);
}
