#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    static mut stderr: *mut FILE;
    fn free(_: *mut libc::c_void);
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
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn QuadTree_new(
        dim: libc::c_int,
        center: *mut libc::c_double,
        width: libc::c_double,
        max_level: libc::c_int,
    ) -> QuadTree;
    fn QuadTree_delete(q: QuadTree);
    fn QuadTree_print(fp: *mut FILE, q: QuadTree);
    fn QuadTree_new_in_quadrant(
        dim: libc::c_int,
        center: *mut libc::c_double,
        width: libc::c_double,
        max_level: libc::c_int,
        i: libc::c_int,
    ) -> QuadTree;
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
unsafe extern "C" fn dist(
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> libc::c_double {
    let mut k: libc::c_int = 0;
    let mut d: libc::c_double = 0 as libc::c_int as libc::c_double;
    k = 0 as libc::c_int;
    while k < dim {
        d
            += (*x.offset(k as isize) - *y.offset(k as isize))
                * (*x.offset(k as isize) - *y.offset(k as isize));
        k += 1;
    }
    return sqrt(d);
}
unsafe extern "C" fn distance_to_group(
    mut k: libc::c_int,
    mut dim: libc::c_int,
    mut wgt: *mut libc::c_double,
    mut pts: *mut libc::c_double,
    mut center: *mut libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut distance: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    let mut dist_min: libc::c_double = 0 as libc::c_int as libc::c_double;
    if wgt.is_null() {
        i = 0 as libc::c_int;
        while i < k {
            distance = dist(dim, &mut *pts.offset((i * dim) as isize), center);
            if i == 0 as libc::c_int {
                dist_min = distance;
            } else {
                dist_min = if dist_min < distance { dist_min } else { distance };
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < k {
            distance = dist(dim, &mut *pts.offset((i * dim) as isize), center);
            if i == 0 as libc::c_int {
                dist_min = *wgt.offset(i as isize) * distance;
            } else {
                dist_min = if dist_min < *wgt.offset(i as isize) * distance {
                    dist_min
                } else {
                    *wgt.offset(i as isize) * distance
                };
            }
            i += 1;
        }
    }
    return dist_min;
}
#[no_mangle]
pub unsafe extern "C" fn furtherest_point(
    mut k: libc::c_int,
    mut dim: libc::c_int,
    mut wgt: *mut libc::c_double,
    mut pts: *mut libc::c_double,
    mut center: *mut libc::c_double,
    mut width: libc::c_double,
    mut max_level: libc::c_int,
    mut dist_max: *mut libc::c_double,
    mut argmax: *mut *mut libc::c_double,
) {
    let mut qt: QuadTree = 0 as *mut QuadTree_struct;
    let mut qt0: QuadTree = 0 as *mut QuadTree_struct;
    let mut ncandidates: libc::c_int = 10 as libc::c_int;
    let mut ncandidates_max: libc::c_int = 10 as libc::c_int;
    let mut ntmp: libc::c_int = 0;
    let mut candidates: *mut QuadTree = 0 as *mut QuadTree;
    let mut ctmp: *mut QuadTree = 0 as *mut QuadTree;
    let mut ncandidates2: libc::c_int = 10 as libc::c_int;
    let mut ncandidates2_max: libc::c_int = 10 as libc::c_int;
    let mut candidates2: *mut QuadTree = 0 as *mut QuadTree;
    let mut distance: libc::c_double = 0.;
    let mut level: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pruned: libc::c_int = 0;
    let mut wmax: libc::c_double = 0 as libc::c_int as libc::c_double;
    if !wgt.is_null() {
        i = 0 as libc::c_int;
        while i < k {
            wmax = if *wgt.offset(i as isize) > wmax {
                *wgt.offset(i as isize)
            } else {
                wmax
            };
            i += 1;
        }
    } else {
        wmax = 1.0f64;
    }
    qt = QuadTree_new(dim, center, width, max_level);
    qt0 = qt;
    *dist_max = distance_to_group(k, dim, wgt, pts, center);
    (*qt).total_weight = *dist_max;
    if (*argmax).is_null() {
        *argmax = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    memcpy(
        *argmax as *mut libc::c_void,
        center as *const libc::c_void,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    );
    candidates = gmalloc(
        (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
            .wrapping_mul(ncandidates_max as libc::c_ulong),
    ) as *mut QuadTree;
    candidates2 = gmalloc(
        (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
            .wrapping_mul(ncandidates2_max as libc::c_ulong),
    ) as *mut QuadTree;
    let ref mut fresh0 = *candidates.offset(0 as libc::c_int as isize);
    *fresh0 = qt;
    ncandidates = 1 as libc::c_int;
    loop {
        let fresh1 = level;
        level = level + 1;
        if !(fresh1 < max_level) {
            break;
        }
        if Verbose as libc::c_int > 10 as libc::c_int {
            fprintf(
                stderr,
                b"level=%d=================\n\0" as *const u8 as *const libc::c_char,
                level,
            );
        }
        ncandidates2 = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ncandidates {
            qt = *candidates.offset(i as isize);
            if ((*qt).qts).is_null() {} else {
                __assert_fail(
                    b"!(qt->qts)\0" as *const u8 as *const libc::c_char,
                    b"furtherest_point.c\0" as *const u8 as *const libc::c_char,
                    108 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void furtherest_point(int, int, double *, double *, double *, double, int, double *, double **)\0",
                    ))
                        .as_ptr(),
                );
            }
            if Verbose as libc::c_int > 10 as libc::c_int {
                fprintf(
                    stderr,
                    b"candidate %d at {\0" as *const u8 as *const libc::c_char,
                    i,
                );
                j = 0 as libc::c_int;
                while j < dim {
                    fprintf(
                        stderr,
                        b"%f, \0" as *const u8 as *const libc::c_char,
                        *((*qt).center).offset(j as isize),
                    );
                    j += 1;
                }
                fprintf(
                    stderr,
                    b"}, width = %f, dist = %f\n\0" as *const u8 as *const libc::c_char,
                    (*qt).width,
                    (*qt).total_weight,
                );
            }
            distance = (*qt).total_weight;
            if !(distance + wmax * sqrt(dim as libc::c_double) * (*qt).width < *dist_max)
            {
                let ref mut fresh2 = (*qt).qts;
                *fresh2 = gmalloc(
                    (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
                        .wrapping_mul(((1 as libc::c_int) << dim) as libc::c_ulong),
                ) as *mut QuadTree;
                ii = 0 as libc::c_int;
                while ii < (1 as libc::c_int) << dim {
                    let ref mut fresh3 = *((*qt).qts).offset(ii as isize);
                    *fresh3 = QuadTree_new_in_quadrant(
                        (*qt).dim,
                        (*qt).center,
                        (*qt).width / 2 as libc::c_int as libc::c_double,
                        max_level,
                        ii,
                    );
                    distance = distance_to_group(
                        k,
                        dim,
                        wgt,
                        pts,
                        (**((*qt).qts).offset(ii as isize)).center,
                    );
                    (**((*qt).qts).offset(ii as isize)).total_weight = distance;
                    pruned = 0 as libc::c_int;
                    if distance > *dist_max {
                        *dist_max = distance;
                        if Verbose as libc::c_int > 10 as libc::c_int {
                            fprintf(
                                stderr,
                                b"new distmax=%f, pt={\0" as *const u8
                                    as *const libc::c_char,
                                *dist_max,
                            );
                            j = 0 as libc::c_int;
                            while j < dim {
                                fprintf(
                                    stderr,
                                    b"%f, \0" as *const u8 as *const libc::c_char,
                                    *((**((*qt).qts).offset(ii as isize)).center)
                                        .offset(j as isize),
                                );
                                j += 1;
                            }
                            fprintf(
                                stderr,
                                b"}\n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        memcpy(
                            *argmax as *mut libc::c_void,
                            (**((*qt).qts).offset(ii as isize)).center
                                as *const libc::c_void,
                            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                                .wrapping_mul(dim as libc::c_ulong),
                        );
                    } else if (distance
                            + wmax * sqrt(dim as libc::c_double) * (*qt).width
                                / 2 as libc::c_int as libc::c_double) < *dist_max
                        {
                        pruned = (0 as libc::c_int == 0) as libc::c_int;
                    }
                    if pruned == 0 {
                        if ncandidates2 >= ncandidates2_max {
                            ncandidates2_max
                                += (if 0.2f64 * ncandidates2_max as libc::c_double
                                    > 10 as libc::c_int as libc::c_double
                                {
                                    0.2f64 * ncandidates2_max as libc::c_double
                                } else {
                                    10 as libc::c_int as libc::c_double
                                }) as libc::c_int;
                            candidates2 = grealloc(
                                candidates2 as *mut libc::c_void,
                                (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
                                    .wrapping_mul(ncandidates2_max as libc::c_ulong),
                            ) as *mut QuadTree;
                        }
                        let fresh4 = ncandidates2;
                        ncandidates2 = ncandidates2 + 1;
                        let ref mut fresh5 = *candidates2.offset(fresh4 as isize);
                        *fresh5 = *((*qt).qts).offset(ii as isize);
                    }
                    ii += 1;
                }
            }
            i += 1;
        }
        ntmp = ncandidates;
        ncandidates = ncandidates2;
        ncandidates2 = ntmp;
        ntmp = ncandidates_max;
        ncandidates_max = ncandidates2_max;
        ncandidates2_max = ntmp;
        ctmp = candidates;
        candidates = candidates2;
        candidates2 = ctmp;
    }
    if Verbose as libc::c_int > 10 as libc::c_int {
        let mut fp: *mut FILE = 0 as *mut FILE;
        fp = fopen(
            b"/tmp/1.m\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        QuadTree_print(fp, qt0);
    }
    QuadTree_delete(qt0);
    free(candidates as *mut libc::c_void);
    free(candidates2 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn furtherest_point_in_list(
    mut k: libc::c_int,
    mut dim: libc::c_int,
    mut wgt: *mut libc::c_double,
    mut pts: *mut libc::c_double,
    mut qt: QuadTree,
    mut max_level: libc::c_int,
    mut dist_max: *mut libc::c_double,
    mut argmax: *mut *mut libc::c_double,
) {
    let mut ncandidates: libc::c_int = 10 as libc::c_int;
    let mut ncandidates_max: libc::c_int = 10 as libc::c_int;
    let mut ntmp: libc::c_int = 0;
    let mut candidates: *mut QuadTree = 0 as *mut QuadTree;
    let mut ctmp: *mut QuadTree = 0 as *mut QuadTree;
    let mut ncandidates2: libc::c_int = 10 as libc::c_int;
    let mut ncandidates2_max: libc::c_int = 10 as libc::c_int;
    let mut candidates2: *mut QuadTree = 0 as *mut QuadTree;
    let mut distance: libc::c_double = 0.;
    let mut level: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pruned: libc::c_int = 0;
    let mut average: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut wmax: libc::c_double = 0.0f64;
    if !wgt.is_null() {
        i = 0 as libc::c_int;
        while i < k {
            wmax = if *wgt.offset(i as isize) > wmax {
                *wgt.offset(i as isize)
            } else {
                wmax
            };
            i += 1;
        }
    } else {
        wmax = 1.0f64;
    }
    average = (*qt).average;
    *dist_max = distance_to_group(k, dim, wgt, pts, average);
    (*qt).total_weight = *dist_max;
    if (*argmax).is_null() {
        *argmax = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    memcpy(
        *argmax as *mut libc::c_void,
        average as *const libc::c_void,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    );
    candidates = gmalloc(
        (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
            .wrapping_mul(ncandidates_max as libc::c_ulong),
    ) as *mut QuadTree;
    candidates2 = gmalloc(
        (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
            .wrapping_mul(ncandidates2_max as libc::c_ulong),
    ) as *mut QuadTree;
    let ref mut fresh6 = *candidates.offset(0 as libc::c_int as isize);
    *fresh6 = qt;
    ncandidates = 1 as libc::c_int;
    loop {
        let fresh7 = level;
        level = level + 1;
        if !(fresh7 < max_level) {
            break;
        }
        if Verbose as libc::c_int > 10 as libc::c_int {
            fprintf(
                stderr,
                b"level=%d=================\n\0" as *const u8 as *const libc::c_char,
                level,
            );
        }
        ncandidates2 = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ncandidates {
            qt = *candidates.offset(i as isize);
            if Verbose as libc::c_int > 10 as libc::c_int {
                fprintf(
                    stderr,
                    b"candidate %d at {\0" as *const u8 as *const libc::c_char,
                    i,
                );
                j = 0 as libc::c_int;
                while j < dim {
                    fprintf(
                        stderr,
                        b"%f, \0" as *const u8 as *const libc::c_char,
                        *((*qt).center).offset(j as isize),
                    );
                    j += 1;
                }
                fprintf(
                    stderr,
                    b"}, width = %f, dist = %f\n\0" as *const u8 as *const libc::c_char,
                    (*qt).width,
                    (*qt).total_weight,
                );
            }
            distance = (*qt).total_weight;
            if !((*qt).n == 1 as libc::c_int
                || distance
                    + wmax * 2 as libc::c_int as libc::c_double
                        * sqrt(dim as libc::c_double) * (*qt).width < *dist_max)
            {
                if !((*qt).qts).is_null() {
                    ii = 0 as libc::c_int;
                    while ii < (1 as libc::c_int) << dim {
                        if !(*((*qt).qts).offset(ii as isize)).is_null() {
                            distance = distance_to_group(
                                k,
                                dim,
                                wgt,
                                pts,
                                (**((*qt).qts).offset(ii as isize)).average,
                            );
                            (**((*qt).qts).offset(ii as isize)).total_weight = distance;
                            pruned = 0 as libc::c_int;
                            if distance > *dist_max {
                                *dist_max = distance;
                                if Verbose as libc::c_int > 10 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"new distmax=%f, pt={\0" as *const u8
                                            as *const libc::c_char,
                                        *dist_max,
                                    );
                                    j = 0 as libc::c_int;
                                    while j < dim {
                                        fprintf(
                                            stderr,
                                            b"%f, \0" as *const u8 as *const libc::c_char,
                                            *((**((*qt).qts).offset(ii as isize)).average)
                                                .offset(j as isize),
                                        );
                                        j += 1;
                                    }
                                    fprintf(
                                        stderr,
                                        b"}\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                memcpy(
                                    *argmax as *mut libc::c_void,
                                    (**((*qt).qts).offset(ii as isize)).average
                                        as *const libc::c_void,
                                    (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                                        .wrapping_mul(dim as libc::c_ulong),
                                );
                            } else if distance
                                    + wmax * sqrt(dim as libc::c_double) * (*qt).width
                                    < *dist_max
                                {
                                pruned = (0 as libc::c_int == 0) as libc::c_int;
                            }
                            if pruned == 0 {
                                if ncandidates2 >= ncandidates2_max {
                                    ncandidates2_max
                                        += (if 0.2f64 * ncandidates2_max as libc::c_double
                                            > 10 as libc::c_int as libc::c_double
                                        {
                                            0.2f64 * ncandidates2_max as libc::c_double
                                        } else {
                                            10 as libc::c_int as libc::c_double
                                        }) as libc::c_int;
                                    candidates2 = grealloc(
                                        candidates2 as *mut libc::c_void,
                                        (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
                                            .wrapping_mul(ncandidates2_max as libc::c_ulong),
                                    ) as *mut QuadTree;
                                }
                                let fresh8 = ncandidates2;
                                ncandidates2 = ncandidates2 + 1;
                                let ref mut fresh9 = *candidates2.offset(fresh8 as isize);
                                *fresh9 = *((*qt).qts).offset(ii as isize);
                            }
                        }
                        ii += 1;
                    }
                }
            }
            i += 1;
        }
        ntmp = ncandidates;
        ncandidates = ncandidates2;
        ncandidates2 = ntmp;
        ntmp = ncandidates_max;
        ncandidates_max = ncandidates2_max;
        ncandidates2_max = ntmp;
        ctmp = candidates;
        candidates = candidates2;
        candidates2 = ctmp;
    }
    free(candidates as *mut libc::c_void);
    free(candidates2 as *mut libc::c_void);
}
