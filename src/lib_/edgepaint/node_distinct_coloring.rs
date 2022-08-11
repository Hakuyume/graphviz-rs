#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn free(_: *mut libc::c_void);
    fn srand(__seed: libc::c_uint);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn drand() -> libc::c_double;
    fn irand(n: libc::c_int) -> libc::c_int;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Verbose: libc::c_uchar;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_symmetrize(
        A: SparseMatrix,
        pattern_symmetric_only: bool,
    ) -> SparseMatrix;
    fn SparseMatrix_weakly_connected_components(
        A0: SparseMatrix,
        ncomp: *mut libc::c_int,
        comps: *mut *mut libc::c_int,
        comps_ptr: *mut *mut libc::c_int,
    );
    fn SparseMatrix_get_submatrix(
        A: SparseMatrix,
        nrow: libc::c_int,
        ncol: libc::c_int,
        rindices: *mut libc::c_int,
        cindices: *mut libc::c_int,
    ) -> SparseMatrix;
    fn QuadTree_new_from_point_list(
        dim: libc::c_int,
        n: libc::c_int,
        max_level: libc::c_int,
        coord: *mut libc::c_double,
    ) -> QuadTree;
    fn QuadTree_get_nearest(
        qt: QuadTree,
        x: *mut libc::c_double,
        ymin: *mut libc::c_double,
        imin: *mut libc::c_int,
        min: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    fn RGB2LAB(color: color_rgb) -> color_lab;
    fn LAB2RGB_real_01(color: *mut libc::c_double);
    fn LAB2RGB(color: color_lab) -> color_rgb;
    fn color_lab_init(
        l: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> color_lab;
    fn lab_gamut_quadtree(
        lightness: *const libc::c_char,
        max_qtree_level: libc::c_int,
    ) -> QuadTree;
    fn color_blend_rgb2lab(
        color_list: *mut libc::c_char,
        maxpoints: libc::c_int,
        colors: *mut *mut libc::c_double,
    );
    fn furtherest_point(
        k: libc::c_int,
        dim: libc::c_int,
        wgt: *mut libc::c_double,
        pts: *mut libc::c_double,
        center: *mut libc::c_double,
        width: libc::c_double,
        max_level: libc::c_int,
        dist_max: *mut libc::c_double,
        argmax: *mut *mut libc::c_double,
    );
    fn furtherest_point_in_list(
        k: libc::c_int,
        dim: libc::c_int,
        wgt: *mut libc::c_double,
        pts: *mut libc::c_double,
        qt: QuadTree,
        max_level: libc::c_int,
        dist_max: *mut libc::c_double,
        argmax: *mut *mut libc::c_double,
    );
    fn color_palettes_get(color_palette_name: *mut libc::c_char) -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SingleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: SingleLinkedList,
}
pub type SingleLinkedList = *mut SingleLinkedList_struct;
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
pub type QuadTree = *mut QuadTree_struct;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COLOR_LAB: C2RustUnnamed_0 = 2;
pub const COLOR_GRAY: C2RustUnnamed_0 = 1;
pub const COLOR_RGB: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_int;
pub const ERROR_BAD_COLOR_SCHEME: C2RustUnnamed_1 = -9;
pub type color_rgb = rgb_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgb_struct {
    pub r: libc::c_double,
    pub g: libc::c_double,
    pub b: libc::c_double,
}
pub type color_lab = lab_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lab_struct {
    pub l: libc::c_schar,
    pub a: libc::c_schar,
    pub b: libc::c_schar,
}
unsafe extern "C" fn node_distinct_coloring_internal2(
    mut scheme: libc::c_int,
    mut qt: QuadTree,
    mut weightedQ: bool,
    mut A: SparseMatrix,
    mut cdim: libc::c_int,
    mut accuracy: libc::c_double,
    mut seed: libc::c_int,
    mut colors: *mut libc::c_double,
    mut color_diff0: *mut libc::c_double,
    mut color_diff_sum0: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut max_level: libc::c_int = 0;
    let mut center: [libc::c_double; 3] = [0.; 3];
    let mut width: libc::c_double = 0.;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist_max: libc::c_double = 0.;
    let mut color_diff: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut color_diff_old: libc::c_double = 0.;
    let mut color_diff_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut color_diff_sum_old: libc::c_double = 0.;
    let mut cc: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut iter: libc::c_int = 0 as libc::c_int;
    static mut iter_max: libc::c_int = 100 as libc::c_int;
    let mut cspace_size: libc::c_double = 0.7f64;
    let mut red: [libc::c_double; 3] = [0.; 3];
    let mut black: [libc::c_double; 3] = [0.; 3];
    let mut min: libc::c_double = 0.;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut imin: libc::c_int = 0;
    let mut wgt: *mut libc::c_double = 0 as *mut libc::c_double;
    if accuracy > 0 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"accuracy > 0\0" as *const u8 as *const libc::c_char,
            b"node_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 122],
                &[libc::c_char; 122],
            >(
                b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    max_level = (if 1 as libc::c_int as libc::c_double > -log(accuracy) / log(2.0f64) {
        1 as libc::c_int as libc::c_double
    } else {
        -log(accuracy) / log(2.0f64)
    }) as libc::c_int;
    max_level = if (30 as libc::c_int) < max_level {
        30 as libc::c_int
    } else {
        max_level
    };
    let mut rgb: color_rgb = {
        let mut init = rgb_struct {
            r: 255 as libc::c_int as libc::c_double * 0.5f64,
            g: 0 as libc::c_int as libc::c_double,
            b: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut lab: color_lab = RGB2LAB(rgb);
    red[0 as libc::c_int as usize] = lab.l as libc::c_double;
    red[1 as libc::c_int as usize] = lab.a as libc::c_double;
    red[2 as libc::c_int as usize] = lab.b as libc::c_double;
    n = (*A).m;
    if n == 1 as libc::c_int {
        if scheme == COLOR_LAB as libc::c_int {
            if !qt.is_null() {} else {
                __assert_fail(
                    b"qt\0" as *const u8 as *const libc::c_char,
                    b"node_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
                    56 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
            QuadTree_get_nearest(
                qt,
                black.as_mut_ptr(),
                colors,
                &mut imin,
                &mut min,
                &mut flag,
            );
            if flag == 0 {} else {
                __assert_fail(
                    b"!flag\0" as *const u8 as *const libc::c_char,
                    b"node_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
                    58 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
            LAB2RGB_real_01(colors);
            *color_diff0 = 1000 as libc::c_int as libc::c_double;
            *color_diff_sum0 = 1000 as libc::c_int as libc::c_double;
        } else {
            i = 0 as libc::c_int;
            while i < cdim {
                *colors.offset(i as isize) = 0 as libc::c_int as libc::c_double;
                i += 1;
            }
            *color_diff0 = sqrt(cdim as libc::c_double);
            *color_diff_sum0 = sqrt(cdim as libc::c_double);
        }
        return;
    } else {
        if n == 2 as libc::c_int {
            if scheme == COLOR_LAB as libc::c_int {
                if !qt.is_null() {} else {
                    __assert_fail(
                        b"qt\0" as *const u8 as *const libc::c_char,
                        b"node_distinct_coloring.c\0" as *const u8
                            as *const libc::c_char,
                        68 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 122],
                            &[libc::c_char; 122],
                        >(
                            b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                QuadTree_get_nearest(
                    qt,
                    black.as_mut_ptr(),
                    colors,
                    &mut imin,
                    &mut min,
                    &mut flag,
                );
                if flag == 0 {} else {
                    __assert_fail(
                        b"!flag\0" as *const u8 as *const libc::c_char,
                        b"node_distinct_coloring.c\0" as *const u8
                            as *const libc::c_char,
                        70 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 122],
                            &[libc::c_char; 122],
                        >(
                            b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                LAB2RGB_real_01(colors);
                QuadTree_get_nearest(
                    qt,
                    red.as_mut_ptr(),
                    colors.offset(cdim as isize),
                    &mut imin,
                    &mut min,
                    &mut flag,
                );
                if flag == 0 {} else {
                    __assert_fail(
                        b"!flag\0" as *const u8 as *const libc::c_char,
                        b"node_distinct_coloring.c\0" as *const u8
                            as *const libc::c_char,
                        74 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 122],
                            &[libc::c_char; 122],
                        >(
                            b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                LAB2RGB_real_01(colors.offset(cdim as isize));
                *color_diff0 = 1000 as libc::c_int as libc::c_double;
                *color_diff_sum0 = 1000 as libc::c_int as libc::c_double;
            } else {
                i = 0 as libc::c_int;
                while i < cdim {
                    *colors.offset(i as isize) = 0 as libc::c_int as libc::c_double;
                    i += 1;
                }
                i = 0 as libc::c_int;
                while i < cdim {
                    *colors
                        .offset(
                            (cdim + i) as isize,
                        ) = 0 as libc::c_int as libc::c_double;
                    i += 1;
                }
                *colors.offset(cdim as isize) = 0.5f64;
                *color_diff0 = sqrt(cdim as libc::c_double);
                *color_diff_sum0 = sqrt(cdim as libc::c_double);
            }
            return;
        }
    }
    if n == (*A).m {} else {
        __assert_fail(
            b"n == A->m\0" as *const u8 as *const libc::c_char,
            b"node_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 122],
                &[libc::c_char; 122],
            >(
                b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int && !((*A).a).is_null() {
        a = (*A).a as *mut libc::c_double;
    }
    center[2 as libc::c_int as usize] = cspace_size * 0.5f64;
    center[1 as libc::c_int as usize] = center[2 as libc::c_int as usize];
    center[0 as libc::c_int as usize] = center[1 as libc::c_int as usize];
    width = cspace_size * 0.5f64;
    srand(seed as libc::c_uint);
    i = 0 as libc::c_int;
    while i < n * cdim {
        *colors.offset(i as isize) = cspace_size * drand();
        i += 1;
    }
    x = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(cdim as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    if weightedQ {
        wgt = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    color_diff = 0 as libc::c_int as libc::c_double;
    color_diff_old = -(1 as libc::c_int) as libc::c_double;
    color_diff_sum = 0 as libc::c_int as libc::c_double;
    color_diff_sum_old = -(1 as libc::c_int) as libc::c_double;
    loop {
        let fresh0 = iter;
        iter = iter + 1;
        if !(fresh0 < iter_max
            && (color_diff > color_diff_old
                || color_diff == color_diff_old && color_diff_sum > color_diff_sum_old))
        {
            break;
        }
        color_diff_old = color_diff;
        color_diff_sum_old = color_diff_sum;
        i = 0 as libc::c_int;
        while i < n {
            k = 0 as libc::c_int;
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if !(*ja.offset(j as isize) == i) {
                    memcpy(
                        &mut *x.offset((k * cdim) as isize) as *mut libc::c_double
                            as *mut libc::c_void,
                        &mut *colors.offset((*ja.offset(j as isize) * cdim) as isize)
                            as *mut libc::c_double as *const libc::c_void,
                        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                            .wrapping_mul(cdim as libc::c_ulong),
                    );
                    if !wgt.is_null() && !a.is_null() {
                        *wgt.offset(k as isize) = *a.offset(j as isize);
                    }
                    k += 1;
                }
                j += 1;
            }
            cc = &mut *colors.offset((i * cdim) as isize) as *mut libc::c_double;
            if scheme == COLOR_LAB as libc::c_int {
                furtherest_point_in_list(
                    k,
                    cdim,
                    wgt,
                    x,
                    qt,
                    max_level,
                    &mut dist_max,
                    &mut cc,
                );
            } else if scheme == COLOR_RGB as libc::c_int
                    || scheme == COLOR_GRAY as libc::c_int
                {
                furtherest_point(
                    k,
                    cdim,
                    wgt,
                    x,
                    center.as_mut_ptr(),
                    width,
                    max_level,
                    &mut dist_max,
                    &mut cc,
                );
            } else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"node_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
                    124 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"void node_distinct_coloring_internal2(int, QuadTree, _Bool, SparseMatrix, int, double, int, double *, double *, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if i == 0 as libc::c_int {
                color_diff = dist_max;
                color_diff_sum = dist_max;
            } else {
                color_diff = if dist_max < color_diff { dist_max } else { color_diff };
                color_diff_sum += dist_max;
            }
            i += 1;
        }
        if Verbose != 0 {
            fprintf(
                stderr,
                b"iter ---- %d ---, color_diff = %f, color_diff_sum = %f\n\0"
                    as *const u8 as *const libc::c_char,
                iter,
                color_diff,
                color_diff_sum,
            );
        }
    }
    if scheme == COLOR_LAB as libc::c_int {
        let mut rgb_0: color_rgb = color_rgb { r: 0., g: 0., b: 0. };
        let mut lab_0: color_lab = color_lab { l: 0, a: 0, b: 0 };
        i = 0 as libc::c_int;
        while i < n {
            lab_0 = color_lab_init(
                *colors.offset((i * cdim) as isize),
                *colors.offset((i * cdim + 1 as libc::c_int) as isize),
                *colors.offset((i * cdim + 2 as libc::c_int) as isize),
            );
            rgb_0 = LAB2RGB(lab_0);
            *colors
                .offset(
                    (i * cdim) as isize,
                ) = rgb_0.r / 255 as libc::c_int as libc::c_double;
            *colors
                .offset(
                    (i * cdim + 1 as libc::c_int) as isize,
                ) = rgb_0.g / 255 as libc::c_int as libc::c_double;
            *colors
                .offset(
                    (i * cdim + 2 as libc::c_int) as isize,
                ) = rgb_0.b / 255 as libc::c_int as libc::c_double;
            i += 1;
        }
    }
    *color_diff0 = color_diff;
    *color_diff_sum0 = color_diff_sum;
    free(x as *mut libc::c_void);
}
unsafe extern "C" fn node_distinct_coloring_internal(
    mut scheme: libc::c_int,
    mut qt: QuadTree,
    mut weightedQ: bool,
    mut A: SparseMatrix,
    mut cdim: libc::c_int,
    mut accuracy: libc::c_double,
    mut seed: libc::c_int,
    mut colors: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut color_diff: libc::c_double = 0.;
    let mut color_diff_sum: libc::c_double = 0.;
    if seed < 0 as libc::c_int {
        let mut iter: libc::c_int = 0;
        let mut seed_max: libc::c_int = -(1 as libc::c_int);
        let mut color_diff_max: libc::c_double = -(1 as libc::c_int) as libc::c_double;
        srand(123 as libc::c_int as libc::c_uint);
        iter = -seed;
        i = 0 as libc::c_int;
        while i < iter {
            seed = irand(100000 as libc::c_int);
            node_distinct_coloring_internal2(
                scheme,
                qt,
                weightedQ,
                A,
                cdim,
                accuracy,
                seed,
                colors,
                &mut color_diff,
                &mut color_diff_sum,
            );
            if color_diff_max < color_diff {
                seed_max = seed;
                color_diff_max = color_diff;
            }
            i += 1;
        }
        seed = seed_max;
    }
    node_distinct_coloring_internal2(
        scheme,
        qt,
        weightedQ,
        A,
        cdim,
        accuracy,
        seed,
        colors,
        &mut color_diff,
        &mut color_diff_sum,
    );
}
#[no_mangle]
pub unsafe extern "C" fn node_distinct_coloring(
    mut color_scheme: *mut libc::c_char,
    mut lightness: *mut libc::c_char,
    mut weightedQ: bool,
    mut A0: SparseMatrix,
    mut accuracy: libc::c_double,
    mut seed: libc::c_int,
    mut cdim0: *mut libc::c_int,
    mut colors: *mut *mut libc::c_double,
) -> libc::c_int {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut A: SparseMatrix = A0;
    let mut ncomps: libc::c_int = 0;
    let mut comps: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut comps_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nn: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ctmp: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut qt: QuadTree = 0 as QuadTree;
    let mut cdim: libc::c_int = 0;
    let mut scheme: libc::c_int = COLOR_LAB as libc::c_int;
    let mut maxcolors: libc::c_int = 10000 as libc::c_int;
    let mut max_qtree_level: libc::c_int = 10 as libc::c_int;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut color_list: *mut libc::c_char = 0 as *mut libc::c_char;
    color_list = color_palettes_get(color_scheme);
    if !color_list.is_null() {
        color_scheme = color_list;
    }
    *cdim0 = 3 as libc::c_int;
    cdim = *cdim0;
    if strcmp(color_scheme, b"lab\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if Verbose != 0 {
            fprintf(stderr, b"lab\n\0" as *const u8 as *const libc::c_char);
        }
        scheme = COLOR_LAB as libc::c_int;
        qt = lab_gamut_quadtree(lightness, max_qtree_level);
        if qt.is_null() {
            fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    } else if strcmp(color_scheme, b"rgb\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
        if Verbose != 0 {
            fprintf(stderr, b"rgb\n\0" as *const u8 as *const libc::c_char);
        }
        scheme = COLOR_RGB as libc::c_int;
    } else if strcmp(color_scheme, b"gray\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
        scheme = COLOR_GRAY as libc::c_int;
        *cdim0 = 1 as libc::c_int;
        cdim = *cdim0;
    } else if sscanf(
            color_scheme,
            b"#%02X%02X%02X\0" as *const u8 as *const libc::c_char,
            &mut r as *mut libc::c_int,
            &mut g as *mut libc::c_int,
            &mut b as *mut libc::c_int,
        ) == 3 as libc::c_int
        {
        let mut colors_0: *mut libc::c_double = 0 as *mut libc::c_double;
        scheme = COLOR_LAB as libc::c_int;
        color_blend_rgb2lab(color_scheme, maxcolors, &mut colors_0);
        if !colors_0.is_null() {} else {
            __assert_fail(
                b"colors\0" as *const u8 as *const libc::c_char,
                b"node_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
                232 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"int node_distinct_coloring(char *, char *, _Bool, SparseMatrix, double, int, int *, double **)\0",
                ))
                    .as_ptr(),
            );
        }
        qt = QuadTree_new_from_point_list(cdim, maxcolors, max_qtree_level, colors_0);
        if !qt.is_null() {} else {
            __assert_fail(
                b"qt\0" as *const u8 as *const libc::c_char,
                b"node_distinct_coloring.c\0" as *const u8 as *const libc::c_char,
                234 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"int node_distinct_coloring(char *, char *, _Bool, SparseMatrix, double, int, int *, double **)\0",
                ))
                    .as_ptr(),
            );
        }
    } else {
        return ERROR_BAD_COLOR_SCHEME as libc::c_int
    }
    if accuracy <= 0 as libc::c_int as libc::c_double {
        accuracy = 0.0001f64;
    }
    n = (*A).m;
    if n != (*A).n {
        return -(1 as libc::c_int);
    }
    if (*colors).is_null() {
        *colors = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(cdim as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    ctmp = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(cdim as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    B = SparseMatrix_symmetrize(A, 0 as libc::c_int != 0);
    A = B;
    SparseMatrix_weakly_connected_components(A, &mut ncomps, &mut comps, &mut comps_ptr);
    i = 0 as libc::c_int;
    while i < ncomps {
        nn = *comps_ptr.offset((i + 1 as libc::c_int) as isize)
            - *comps_ptr.offset(i as isize);
        B = SparseMatrix_get_submatrix(
            A,
            nn,
            nn,
            &mut *comps.offset(*comps_ptr.offset(i as isize) as isize),
            &mut *comps.offset(*comps_ptr.offset(i as isize) as isize),
        );
        node_distinct_coloring_internal(
            scheme,
            qt,
            weightedQ,
            B,
            cdim,
            accuracy,
            seed,
            ctmp,
        );
        j = *comps_ptr.offset(i as isize);
        while j < *comps_ptr.offset((i + 1 as libc::c_int) as isize) {
            jj = j - *comps_ptr.offset(i as isize);
            memcpy(
                &mut *(*colors).offset((*comps.offset(j as isize) * cdim) as isize)
                    as *mut libc::c_double as *mut libc::c_void,
                &mut *ctmp.offset((jj * cdim) as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (cdim as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            j += 1;
        }
        SparseMatrix_delete(B);
        i += 1;
    }
    free(ctmp as *mut libc::c_void);
    if A != A0 {
        SparseMatrix_delete(A);
    }
    return 0 as libc::c_int;
}
