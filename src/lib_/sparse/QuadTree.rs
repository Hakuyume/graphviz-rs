#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn point_distance(
        p1: *mut libc::c_double,
        p2: *mut libc::c_double,
        dim: libc::c_int,
    ) -> libc::c_double;
    fn distance_cropped(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn SingleLinkedList_new(data: *mut libc::c_void) -> SingleLinkedList;
    fn SingleLinkedList_delete(
        head: SingleLinkedList,
        linklist_deallocator: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn SingleLinkedList_prepend(
        l: SingleLinkedList,
        data: *mut libc::c_void,
    ) -> SingleLinkedList;
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
pub type node_data = *mut node_data_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_data_struct {
    pub node_weight: libc::c_double,
    pub coord: *mut libc::c_double,
    pub id: libc::c_double,
    pub data: *mut libc::c_void,
}
unsafe extern "C" fn node_data_new(
    mut dim: libc::c_int,
    mut weight: libc::c_double,
    mut coord: *mut libc::c_double,
    mut id: libc::c_int,
) -> node_data {
    let mut nd: node_data = 0 as *mut node_data_struct;
    let mut i: libc::c_int = 0;
    nd = gmalloc(::std::mem::size_of::<node_data_struct>() as libc::c_ulong)
        as node_data;
    (*nd).node_weight = weight;
    let ref mut fresh0 = (*nd).coord;
    *fresh0 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    ) as *mut libc::c_double;
    (*nd).id = id as libc::c_double;
    i = 0 as libc::c_int;
    while i < dim {
        *((*nd).coord).offset(i as isize) = *coord.offset(i as isize);
        i += 1;
    }
    let ref mut fresh1 = (*nd).data;
    *fresh1 = 0 as *mut libc::c_void;
    return nd;
}
unsafe extern "C" fn node_data_delete(mut d: *mut libc::c_void) {
    let mut nd: node_data = d as node_data;
    free((*nd).coord as *mut libc::c_void);
    free(nd as *mut libc::c_void);
}
unsafe extern "C" fn node_data_get_weight(mut d: *mut libc::c_void) -> libc::c_double {
    let mut nd: node_data = d as node_data;
    return (*nd).node_weight;
}
unsafe extern "C" fn node_data_get_coord(
    mut d: *mut libc::c_void,
) -> *mut libc::c_double {
    let mut nd: node_data = d as node_data;
    return (*nd).coord;
}
unsafe extern "C" fn node_data_get_id(mut d: *mut libc::c_void) -> libc::c_int {
    let mut nd: node_data = d as node_data;
    return (*nd).id as libc::c_int;
}
unsafe extern "C" fn check_or_realloc_arrays(
    mut dim: libc::c_int,
    mut nsuper: *mut libc::c_int,
    mut nsupermax: *mut libc::c_int,
    mut center: *mut *mut libc::c_double,
    mut supernode_wgts: *mut *mut libc::c_double,
    mut distances: *mut *mut libc::c_double,
) {
    if *nsuper >= *nsupermax {
        *nsupermax = *nsuper
            + (if 10 as libc::c_int > 0.2f64 as libc::c_int * *nsuper {
                10 as libc::c_int
            } else {
                0.2f64 as libc::c_int * *nsuper
            });
        *center = grealloc(
            *center as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(*nsupermax as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        *supernode_wgts = grealloc(
            *supernode_wgts as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(*nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
        *distances = grealloc(
            *distances as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(*nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
}
unsafe extern "C" fn QuadTree_get_supernodes_internal(
    mut qt: QuadTree,
    mut bh: libc::c_double,
    mut point: *mut libc::c_double,
    mut nodeid: libc::c_int,
    mut nsuper: *mut libc::c_int,
    mut nsupermax: *mut libc::c_int,
    mut center: *mut *mut libc::c_double,
    mut supernode_wgts: *mut *mut libc::c_double,
    mut distances: *mut *mut libc::c_double,
    mut counts: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut l: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    let mut coord: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut dim: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    *counts += 1.;
    if qt.is_null() {
        return;
    }
    dim = (*qt).dim;
    l = (*qt).l;
    if !l.is_null() {
        while !l.is_null() {
            check_or_realloc_arrays(
                dim,
                nsuper,
                nsupermax,
                center,
                supernode_wgts,
                distances,
            );
            if node_data_get_id(SingleLinkedList_get_data(l)) != nodeid {
                coord = node_data_get_coord(SingleLinkedList_get_data(l));
                i = 0 as libc::c_int;
                while i < dim {
                    *(*center)
                        .offset(
                            (dim * *nsuper + i) as isize,
                        ) = *coord.offset(i as isize);
                    i += 1;
                }
                *(*supernode_wgts)
                    .offset(
                        *nsuper as isize,
                    ) = node_data_get_weight(SingleLinkedList_get_data(l));
                *(*distances)
                    .offset(*nsuper as isize) = point_distance(point, coord, dim);
                *nsuper += 1;
            }
            l = SingleLinkedList_get_next(l);
        }
    }
    if !((*qt).qts).is_null() {
        dist = point_distance((*qt).center, point, dim);
        if (*qt).width < bh * dist {
            check_or_realloc_arrays(
                dim,
                nsuper,
                nsupermax,
                center,
                supernode_wgts,
                distances,
            );
            i = 0 as libc::c_int;
            while i < dim {
                *(*center)
                    .offset(
                        (dim * *nsuper + i) as isize,
                    ) = *((*qt).average).offset(i as isize);
                i += 1;
            }
            *(*supernode_wgts).offset(*nsuper as isize) = (*qt).total_weight;
            *(*distances)
                .offset(*nsuper as isize) = point_distance((*qt).average, point, dim);
            *nsuper += 1;
        } else {
            i = 0 as libc::c_int;
            while i < (1 as libc::c_int) << dim {
                QuadTree_get_supernodes_internal(
                    *((*qt).qts).offset(i as isize),
                    bh,
                    point,
                    nodeid,
                    nsuper,
                    nsupermax,
                    center,
                    supernode_wgts,
                    distances,
                    counts,
                    flag,
                );
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_get_supernodes(
    mut qt: QuadTree,
    mut bh: libc::c_double,
    mut point: *mut libc::c_double,
    mut nodeid: libc::c_int,
    mut nsuper: *mut libc::c_int,
    mut nsupermax: *mut libc::c_int,
    mut center: *mut *mut libc::c_double,
    mut supernode_wgts: *mut *mut libc::c_double,
    mut distances: *mut *mut libc::c_double,
    mut counts: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut dim: libc::c_int = (*qt).dim;
    *counts = 0 as libc::c_int as libc::c_double;
    *nsuper = 0 as libc::c_int;
    *flag = 0 as libc::c_int;
    *nsupermax = 10 as libc::c_int;
    if (*center).is_null() {
        *center = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(*nsupermax as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    if (*supernode_wgts).is_null() {
        *supernode_wgts = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(*nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    if (*distances).is_null() {
        *distances = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(*nsupermax as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    QuadTree_get_supernodes_internal(
        qt,
        bh,
        point,
        nodeid,
        nsuper,
        nsupermax,
        center,
        supernode_wgts,
        distances,
        counts,
        flag,
    );
}
unsafe extern "C" fn get_or_assign_node_force(
    mut force: *mut libc::c_double,
    mut i: libc::c_int,
    mut l: SingleLinkedList,
    mut dim: libc::c_int,
) -> *mut libc::c_double {
    let mut f: *mut libc::c_double = (*(SingleLinkedList_get_data(l) as node_data)).data
        as *mut libc::c_double;
    if f.is_null() {
        let ref mut fresh2 = (*(SingleLinkedList_get_data(l) as node_data)).data;
        *fresh2 = &mut *force.offset((i * dim) as isize) as *mut libc::c_double
            as *mut libc::c_void;
        f = (*(SingleLinkedList_get_data(l) as node_data)).data as *mut libc::c_double;
    }
    return f;
}
unsafe extern "C" fn get_or_alloc_force_qt(
    mut qt: QuadTree,
    mut dim: libc::c_int,
) -> *mut libc::c_double {
    let mut i: libc::c_int = 0;
    let mut force: *mut libc::c_double = (*qt).data as *mut libc::c_double;
    if force.is_null() {
        let ref mut fresh3 = (*qt).data;
        *fresh3 = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        );
        force = (*qt).data as *mut libc::c_double;
        i = 0 as libc::c_int;
        while i < dim {
            *force.offset(i as isize) = 0.0f64;
            i += 1;
        }
    }
    return force;
}
unsafe extern "C" fn QuadTree_repulsive_force_interact(
    mut qt1: QuadTree,
    mut qt2: QuadTree,
    mut x: *mut libc::c_double,
    mut force: *mut libc::c_double,
    mut bh: libc::c_double,
    mut p: libc::c_double,
    mut KP: libc::c_double,
    mut counts: *mut libc::c_double,
) {
    let mut l1: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    let mut l2: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    let mut x1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut wgt1: libc::c_double = 0.;
    let mut wgt2: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut f1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w1: libc::c_double = 0.;
    let mut w2: libc::c_double = 0.;
    let mut dim: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut qt11: QuadTree = 0 as *mut QuadTree_struct;
    let mut qt12: QuadTree = 0 as *mut QuadTree_struct;
    if qt1.is_null() || qt2.is_null() {
        return;
    }
    if (*qt1).n > 0 as libc::c_int && (*qt2).n > 0 as libc::c_int {} else {
        __assert_fail(
            b"qt1->n > 0 && qt2->n > 0\0" as *const u8 as *const libc::c_char,
            b"QuadTree.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void QuadTree_repulsive_force_interact(QuadTree, QuadTree, double *, double *, double, double, double, double *)\0",
            ))
                .as_ptr(),
        );
    }
    dim = (*qt1).dim;
    l1 = (*qt1).l;
    l2 = (*qt2).l;
    dist = point_distance((*qt1).average, (*qt2).average, dim);
    if (*qt1).width + (*qt2).width < bh * dist {
        let ref mut fresh4 = *counts.offset(0 as libc::c_int as isize);
        *fresh4 += 1.;
        x1 = (*qt1).average;
        w1 = (*qt1).total_weight;
        f1 = get_or_alloc_force_qt(qt1, dim);
        x2 = (*qt2).average;
        w2 = (*qt2).total_weight;
        f2 = get_or_alloc_force_qt(qt2, dim);
        if dist > 0 as libc::c_int as libc::c_double {} else {
            __assert_fail(
                b"dist > 0\0" as *const u8 as *const libc::c_char,
                b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                188 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void QuadTree_repulsive_force_interact(QuadTree, QuadTree, double *, double *, double, double, double, double *)\0",
                ))
                    .as_ptr(),
            );
        }
        k = 0 as libc::c_int;
        while k < dim {
            if p == -(1 as libc::c_int) as libc::c_double {
                f = w1 * w2 * KP * (*x1.offset(k as isize) - *x2.offset(k as isize))
                    / (dist * dist);
            } else {
                f = w1 * w2 * KP * (*x1.offset(k as isize) - *x2.offset(k as isize))
                    / pow(dist, 1.0f64 - p);
            }
            *f1.offset(k as isize) += f;
            *f2.offset(k as isize) -= f;
            k += 1;
        }
        return;
    }
    if !l1.is_null() && !l2.is_null() {
        while !l1.is_null() {
            x1 = node_data_get_coord(SingleLinkedList_get_data(l1));
            wgt1 = node_data_get_weight(SingleLinkedList_get_data(l1));
            i1 = node_data_get_id(SingleLinkedList_get_data(l1));
            f1 = get_or_assign_node_force(force, i1, l1, dim);
            l2 = (*qt2).l;
            while !l2.is_null() {
                x2 = node_data_get_coord(SingleLinkedList_get_data(l2));
                wgt2 = node_data_get_weight(SingleLinkedList_get_data(l2));
                i2 = node_data_get_id(SingleLinkedList_get_data(l2));
                f2 = get_or_assign_node_force(force, i2, l2, dim);
                if qt1 == qt2 && i2 < i1 || i1 == i2 {
                    l2 = SingleLinkedList_get_next(l2);
                } else {
                    let ref mut fresh5 = *counts.offset(1 as libc::c_int as isize);
                    *fresh5 += 1.;
                    dist = distance_cropped(x, dim, i1, i2);
                    k = 0 as libc::c_int;
                    while k < dim {
                        if p == -(1 as libc::c_int) as libc::c_double {
                            f = wgt1 * wgt2 * KP
                                * (*x1.offset(k as isize) - *x2.offset(k as isize))
                                / (dist * dist);
                        } else {
                            f = wgt1 * wgt2 * KP
                                * (*x1.offset(k as isize) - *x2.offset(k as isize))
                                / pow(dist, 1.0f64 - p);
                        }
                        *f1.offset(k as isize) += f;
                        *f2.offset(k as isize) -= f;
                        k += 1;
                    }
                    l2 = SingleLinkedList_get_next(l2);
                }
            }
            l1 = SingleLinkedList_get_next(l1);
        }
        return;
    }
    if qt1 == qt2 {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << dim {
            qt11 = *((*qt1).qts).offset(i as isize);
            j = i;
            while j < (1 as libc::c_int) << dim {
                qt12 = *((*qt1).qts).offset(j as isize);
                QuadTree_repulsive_force_interact(
                    qt11,
                    qt12,
                    x,
                    force,
                    bh,
                    p,
                    KP,
                    counts,
                );
                j += 1;
            }
            i += 1;
        }
    } else if (*qt1).width > (*qt2).width && l1.is_null() {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << dim {
            qt11 = *((*qt1).qts).offset(i as isize);
            QuadTree_repulsive_force_interact(qt11, qt2, x, force, bh, p, KP, counts);
            i += 1;
        }
    } else if (*qt2).width > (*qt1).width && l2.is_null() {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << dim {
            qt11 = *((*qt2).qts).offset(i as isize);
            QuadTree_repulsive_force_interact(qt11, qt1, x, force, bh, p, KP, counts);
            i += 1;
        }
    } else if l1.is_null() {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << dim {
            qt11 = *((*qt1).qts).offset(i as isize);
            QuadTree_repulsive_force_interact(qt11, qt2, x, force, bh, p, KP, counts);
            i += 1;
        }
    } else if l2.is_null() {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << dim {
            qt11 = *((*qt2).qts).offset(i as isize);
            QuadTree_repulsive_force_interact(qt11, qt1, x, force, bh, p, KP, counts);
            i += 1;
        }
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"QuadTree.c\0" as *const u8 as *const libc::c_char,
            270 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void QuadTree_repulsive_force_interact(QuadTree, QuadTree, double *, double *, double, double, double, double *)\0",
            ))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn QuadTree_repulsive_force_accumulate(
    mut qt: QuadTree,
    mut force: *mut libc::c_double,
    mut counts: *mut libc::c_double,
) {
    let mut wgt: libc::c_double = 0.;
    let mut wgt2: libc::c_double = 0.;
    let mut f: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut f2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut l: SingleLinkedList = (*qt).l;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut qt2: QuadTree = 0 as *mut QuadTree_struct;
    dim = (*qt).dim;
    wgt = (*qt).total_weight;
    f = get_or_alloc_force_qt(qt, dim);
    if wgt > 0 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"wgt > 0\0" as *const u8 as *const libc::c_char,
            b"QuadTree.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"void QuadTree_repulsive_force_accumulate(QuadTree, double *, double *)\0",
            ))
                .as_ptr(),
        );
    }
    let ref mut fresh6 = *counts.offset(2 as libc::c_int as isize);
    *fresh6 += 1.;
    if !l.is_null() {
        while !l.is_null() {
            i = node_data_get_id(SingleLinkedList_get_data(l));
            f2 = get_or_assign_node_force(force, i, l, dim);
            wgt2 = node_data_get_weight(SingleLinkedList_get_data(l));
            wgt2 = wgt2 / wgt;
            k = 0 as libc::c_int;
            while k < dim {
                *f2.offset(k as isize) += wgt2 * *f.offset(k as isize);
                k += 1;
            }
            l = SingleLinkedList_get_next(l);
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << dim {
        qt2 = *((*qt).qts).offset(i as isize);
        if !qt2.is_null() {
            if (*qt2).n > 0 as libc::c_int {} else {
                __assert_fail(
                    b"qt2->n > 0\0" as *const u8 as *const libc::c_char,
                    b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                    304 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 71],
                        &[libc::c_char; 71],
                    >(
                        b"void QuadTree_repulsive_force_accumulate(QuadTree, double *, double *)\0",
                    ))
                        .as_ptr(),
                );
            }
            f2 = get_or_alloc_force_qt(qt2, dim);
            wgt2 = (*qt2).total_weight;
            wgt2 = wgt2 / wgt;
            k = 0 as libc::c_int;
            while k < dim {
                *f2.offset(k as isize) += wgt2 * *f.offset(k as isize);
                k += 1;
            }
            QuadTree_repulsive_force_accumulate(qt2, force, counts);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_get_repulsive_force(
    mut qt: QuadTree,
    mut force: *mut libc::c_double,
    mut x: *mut libc::c_double,
    mut bh: libc::c_double,
    mut p: libc::c_double,
    mut KP: libc::c_double,
    mut counts: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    let mut n: libc::c_int = (*qt).n;
    let mut dim: libc::c_int = (*qt).dim;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *counts.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i += 1;
    }
    *flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < dim * n {
        *force.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i += 1;
    }
    QuadTree_repulsive_force_interact(qt, qt, x, force, bh, p, KP, counts);
    QuadTree_repulsive_force_accumulate(qt, force, counts);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *counts.offset(i as isize) /= n as libc::c_double;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_new_from_point_list(
    mut dim: libc::c_int,
    mut n: libc::c_int,
    mut max_level: libc::c_int,
    mut coord: *mut libc::c_double,
) -> QuadTree {
    let mut xmin: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xmax: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut center: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut width: libc::c_double = 0.;
    let mut qt: QuadTree = 0 as QuadTree;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    xmin = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    ) as *mut libc::c_double;
    xmax = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    ) as *mut libc::c_double;
    center = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    ) as *mut libc::c_double;
    if xmin.is_null() || xmax.is_null() || center.is_null() {
        free(xmin as *mut libc::c_void);
        free(xmax as *mut libc::c_void);
        free(center as *mut libc::c_void);
        return 0 as QuadTree;
    }
    i = 0 as libc::c_int;
    while i < dim {
        *xmin.offset(i as isize) = *coord.offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim {
        *xmax.offset(i as isize) = *coord.offset(i as isize);
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < n {
        k = 0 as libc::c_int;
        while k < dim {
            *xmin
                .offset(
                    k as isize,
                ) = if *xmin.offset(k as isize) < *coord.offset((i * dim + k) as isize) {
                *xmin.offset(k as isize)
            } else {
                *coord.offset((i * dim + k) as isize)
            };
            *xmax
                .offset(
                    k as isize,
                ) = if *xmax.offset(k as isize) > *coord.offset((i * dim + k) as isize) {
                *xmax.offset(k as isize)
            } else {
                *coord.offset((i * dim + k) as isize)
            };
            k += 1;
        }
        i += 1;
    }
    width = *xmax.offset(0 as libc::c_int as isize)
        - *xmin.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < dim {
        *center
            .offset(
                i as isize,
            ) = (*xmin.offset(i as isize) + *xmax.offset(i as isize)) * 0.5f64;
        width = if width > *xmax.offset(i as isize) - *xmin.offset(i as isize) {
            width
        } else {
            *xmax.offset(i as isize) - *xmin.offset(i as isize)
        };
        i += 1;
    }
    if width == 0 as libc::c_int as libc::c_double {
        width = 0.00001f64;
    }
    width *= 0.52f64;
    qt = QuadTree_new(dim, center, width, max_level);
    i = 0 as libc::c_int;
    while i < n {
        qt = QuadTree_add(
            qt,
            &mut *coord.offset((i * dim) as isize),
            1 as libc::c_int as libc::c_double,
            i,
        );
        i += 1;
    }
    free(xmin as *mut libc::c_void);
    free(xmax as *mut libc::c_void);
    free(center as *mut libc::c_void);
    return qt;
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_new(
    mut dim: libc::c_int,
    mut center: *mut libc::c_double,
    mut width: libc::c_double,
    mut max_level: libc::c_int,
) -> QuadTree {
    let mut q: QuadTree = 0 as *mut QuadTree_struct;
    let mut i: libc::c_int = 0;
    q = gmalloc(::std::mem::size_of::<QuadTree_struct>() as libc::c_ulong) as QuadTree;
    (*q).dim = dim;
    (*q).n = 0 as libc::c_int;
    let ref mut fresh7 = (*q).center;
    *fresh7 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong),
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < dim {
        *((*q).center).offset(i as isize) = *center.offset(i as isize);
        i += 1;
    }
    if width > 0 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"width > 0\0" as *const u8 as *const libc::c_char,
            b"QuadTree.c\0" as *const u8 as *const libc::c_char,
            400 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"QuadTree QuadTree_new(int, double *, double, int)\0"))
                .as_ptr(),
        );
    }
    (*q).width = width;
    (*q).total_weight = 0 as libc::c_int as libc::c_double;
    let ref mut fresh8 = (*q).average;
    *fresh8 = 0 as *mut libc::c_double;
    let ref mut fresh9 = (*q).qts;
    *fresh9 = 0 as *mut QuadTree;
    let ref mut fresh10 = (*q).l;
    *fresh10 = 0 as SingleLinkedList;
    (*q).max_level = max_level;
    let ref mut fresh11 = (*q).data;
    *fresh11 = 0 as *mut libc::c_void;
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_delete(mut q: QuadTree) {
    let mut i: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    if q.is_null() {
        return;
    }
    dim = (*q).dim;
    free((*q).center as *mut libc::c_void);
    free((*q).average as *mut libc::c_void);
    free((*q).data);
    if !((*q).qts).is_null() {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << dim {
            QuadTree_delete(*((*q).qts).offset(i as isize));
            i += 1;
        }
        free((*q).qts as *mut libc::c_void);
    }
    SingleLinkedList_delete(
        (*q).l,
        Some(node_data_delete as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    free(q as *mut libc::c_void);
}
unsafe extern "C" fn QuadTree_get_quadrant(
    mut dim: libc::c_int,
    mut center: *mut libc::c_double,
    mut coord: *mut libc::c_double,
) -> libc::c_int {
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = dim - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *coord.offset(i as isize) - *center.offset(i as isize)
            < 0 as libc::c_int as libc::c_double
        {
            d = 2 as libc::c_int * d;
        } else {
            d = 2 as libc::c_int * d + 1 as libc::c_int;
        }
        i -= 1;
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_new_in_quadrant(
    mut dim: libc::c_int,
    mut center: *mut libc::c_double,
    mut width: libc::c_double,
    mut max_level: libc::c_int,
    mut i: libc::c_int,
) -> QuadTree {
    let mut qt: QuadTree = 0 as *mut QuadTree_struct;
    let mut k: libc::c_int = 0;
    qt = QuadTree_new(dim, center, width, max_level);
    center = (*qt).center;
    k = 0 as libc::c_int;
    while k < dim {
        if i % 2 as libc::c_int == 0 as libc::c_int {
            *center.offset(k as isize) -= width;
        } else {
            *center.offset(k as isize) += width;
        }
        i = (i - i % 2 as libc::c_int) / 2 as libc::c_int;
        k += 1;
    }
    return qt;
}
unsafe extern "C" fn QuadTree_add_internal(
    mut q: QuadTree,
    mut coord: *mut libc::c_double,
    mut weight: libc::c_double,
    mut id: libc::c_int,
    mut level: libc::c_int,
) -> QuadTree {
    let mut i: libc::c_int = 0;
    let mut dim: libc::c_int = (*q).dim;
    let mut ii: libc::c_int = 0;
    let mut nd: node_data = 0 as node_data;
    let mut max_level: libc::c_int = (*q).max_level;
    let mut idd: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*q).dim {
        *coord.offset(i as isize)
            < *((*q).center).offset(i as isize) - (*q).width
                - 1.0e5f64 * 1.0e-16f64 * (*q).width
            || *coord.offset(i as isize)
                > *((*q).center).offset(i as isize) + (*q).width
                    + 1.0e5f64 * 1.0e-16f64 * (*q).width;
        i += 1;
    }
    if (*q).n == 0 as libc::c_int {
        (*q).n = 1 as libc::c_int;
        (*q).total_weight = weight;
        let ref mut fresh12 = (*q).average;
        *fresh12 = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong),
        ) as *mut libc::c_double;
        i = 0 as libc::c_int;
        while i < (*q).dim {
            *((*q).average).offset(i as isize) = *coord.offset(i as isize);
            i += 1;
        }
        nd = node_data_new((*q).dim, weight, coord, id);
        if ((*q).l).is_null() {} else {
            __assert_fail(
                b"!(q->l)\0" as *const u8 as *const libc::c_char,
                b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh13 = (*q).l;
        *fresh13 = SingleLinkedList_new(nd as *mut libc::c_void);
    } else if level < max_level {
        (*q).total_weight += weight;
        i = 0 as libc::c_int;
        while i < (*q).dim {
            *((*q).average)
                .offset(
                    i as isize,
                ) = (*((*q).average).offset(i as isize) * (*q).n as libc::c_double
                + *coord.offset(i as isize))
                / ((*q).n + 1 as libc::c_int) as libc::c_double;
            i += 1;
        }
        if ((*q).qts).is_null() {
            let ref mut fresh14 = (*q).qts;
            *fresh14 = gmalloc(
                (::std::mem::size_of::<QuadTree>() as libc::c_ulong)
                    .wrapping_mul(((1 as libc::c_int) << dim) as libc::c_ulong),
            ) as *mut QuadTree;
            i = 0 as libc::c_int;
            while i < (1 as libc::c_int) << dim {
                let ref mut fresh15 = *((*q).qts).offset(i as isize);
                *fresh15 = 0 as QuadTree;
                i += 1;
            }
        }
        ii = QuadTree_get_quadrant(dim, (*q).center, coord);
        if ii < (1 as libc::c_int) << dim && ii >= 0 as libc::c_int {} else {
            __assert_fail(
                b"ii < 1<<dim && ii >= 0\0" as *const u8 as *const libc::c_char,
                b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                505 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*((*q).qts).offset(ii as isize)).is_null() {
            let ref mut fresh16 = *((*q).qts).offset(ii as isize);
            *fresh16 = QuadTree_new_in_quadrant(
                (*q).dim,
                (*q).center,
                (*q).width / 2 as libc::c_int as libc::c_double,
                max_level,
                ii,
            );
        }
        let ref mut fresh17 = *((*q).qts).offset(ii as isize);
        *fresh17 = QuadTree_add_internal(
            *((*q).qts).offset(ii as isize),
            coord,
            weight,
            id,
            level + 1 as libc::c_int,
        );
        if !(*((*q).qts).offset(ii as isize)).is_null() {} else {
            __assert_fail(
                b"q->qts[ii]\0" as *const u8 as *const libc::c_char,
                b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                509 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        if !((*q).l).is_null() {
            idd = node_data_get_id(SingleLinkedList_get_data((*q).l));
            if (*q).n == 1 as libc::c_int {} else {
                __assert_fail(
                    b"q->n == 1\0" as *const u8 as *const libc::c_char,
                    b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                    513 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            coord = node_data_get_coord(SingleLinkedList_get_data((*q).l));
            weight = node_data_get_weight(SingleLinkedList_get_data((*q).l));
            ii = QuadTree_get_quadrant(dim, (*q).center, coord);
            if ii < (1 as libc::c_int) << dim && ii >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"ii < 1<<dim && ii >= 0\0" as *const u8 as *const libc::c_char,
                    b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                    517 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            if (*((*q).qts).offset(ii as isize)).is_null() {
                let ref mut fresh18 = *((*q).qts).offset(ii as isize);
                *fresh18 = QuadTree_new_in_quadrant(
                    (*q).dim,
                    (*q).center,
                    (*q).width / 2 as libc::c_int as libc::c_double,
                    max_level,
                    ii,
                );
            }
            let ref mut fresh19 = *((*q).qts).offset(ii as isize);
            *fresh19 = QuadTree_add_internal(
                *((*q).qts).offset(ii as isize),
                coord,
                weight,
                idd,
                level + 1 as libc::c_int,
            );
            if !(*((*q).qts).offset(ii as isize)).is_null() {} else {
                __assert_fail(
                    b"q->qts[ii]\0" as *const u8 as *const libc::c_char,
                    b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                    522 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            SingleLinkedList_delete(
                (*q).l,
                Some(node_data_delete as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            let ref mut fresh20 = (*q).l;
            *fresh20 = 0 as SingleLinkedList;
        }
        let ref mut fresh21 = (*q).n;
        *fresh21 += 1;
    } else {
        if ((*q).qts).is_null() {} else {
            __assert_fail(
                b"!(q->qts)\0" as *const u8 as *const libc::c_char,
                b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                531 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh22 = (*q).n;
        *fresh22 += 1;
        (*q).total_weight += weight;
        i = 0 as libc::c_int;
        while i < (*q).dim {
            *((*q).average)
                .offset(
                    i as isize,
                ) = (*((*q).average).offset(i as isize) * (*q).n as libc::c_double
                + *coord.offset(i as isize))
                / ((*q).n + 1 as libc::c_int) as libc::c_double;
            i += 1;
        }
        nd = node_data_new((*q).dim, weight, coord, id);
        if !((*q).l).is_null() {} else {
            __assert_fail(
                b"q->l\0" as *const u8 as *const libc::c_char,
                b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                537 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"QuadTree QuadTree_add_internal(QuadTree, double *, double, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh23 = (*q).l;
        *fresh23 = SingleLinkedList_prepend((*q).l, nd as *mut libc::c_void);
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_add(
    mut q: QuadTree,
    mut coord: *mut libc::c_double,
    mut weight: libc::c_double,
    mut id: libc::c_int,
) -> QuadTree {
    if q.is_null() {
        return q;
    }
    return QuadTree_add_internal(q, coord, weight, id, 0 as libc::c_int);
}
unsafe extern "C" fn draw_polygon(
    mut fp: *mut FILE,
    mut dim: libc::c_int,
    mut center: *mut libc::c_double,
    mut width: libc::c_double,
) {
    if dim < 2 as libc::c_int || dim > 3 as libc::c_int {
        return;
    }
    fprintf(fp, b"(*in c*){Line[{\0" as *const u8 as *const libc::c_char);
    if dim == 2 as libc::c_int {
        fprintf(
            fp,
            b"{%f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
        );
        fprintf(
            fp,
            b",{%f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) + width,
        );
        fprintf(
            fp,
            b",{%f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
        );
    } else if dim == 3 as libc::c_int {
        fprintf(fp, b"{\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp,
            b"{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(fp, b"},\0" as *const u8 as *const libc::c_char);
        fprintf(fp, b"{\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp,
            b"{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(fp, b"},\0" as *const u8 as *const libc::c_char);
        fprintf(fp, b"{\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp,
            b"{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(fp, b"},\0" as *const u8 as *const libc::c_char);
        fprintf(fp, b"{\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp,
            b"{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) + width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(fp, b"},\0" as *const u8 as *const libc::c_char);
        fprintf(fp, b"{\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp,
            b"{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) + width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(fp, b"},\0" as *const u8 as *const libc::c_char);
        fprintf(fp, b"{\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp,
            b"{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) - width,
        );
        fprintf(
            fp,
            b",{%f, %f, %f}\0" as *const u8 as *const libc::c_char,
            *center.offset(0 as libc::c_int as isize) - width,
            *center.offset(1 as libc::c_int as isize) - width,
            *center.offset(2 as libc::c_int as isize) + width,
        );
        fprintf(fp, b"}\0" as *const u8 as *const libc::c_char);
    }
    fprintf(fp, b"}]}(*end C*)\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn QuadTree_print_internal(
    mut fp: *mut FILE,
    mut q: QuadTree,
    mut level: libc::c_int,
) {
    let mut l: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    let mut l0: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    let mut coord: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    if q.is_null() {
        return;
    }
    draw_polygon(fp, (*q).dim, (*q).center, (*q).width);
    dim = (*q).dim;
    l = (*q).l;
    l0 = l;
    if !l.is_null() {
        printf(b",(*a*) {Red,\0" as *const u8 as *const libc::c_char);
        while !l.is_null() {
            if l != l0 {
                printf(b",\0" as *const u8 as *const libc::c_char);
            }
            coord = node_data_get_coord(SingleLinkedList_get_data(l));
            fprintf(
                fp,
                b"(*node %d*) Point[{\0" as *const u8 as *const libc::c_char,
                node_data_get_id(SingleLinkedList_get_data(l)),
            );
            i = 0 as libc::c_int;
            while i < dim {
                if i != 0 as libc::c_int {
                    printf(b",\0" as *const u8 as *const libc::c_char);
                }
                fprintf(
                    fp,
                    b"%f\0" as *const u8 as *const libc::c_char,
                    *coord.offset(i as isize),
                );
                i += 1;
            }
            fprintf(fp, b"}]\0" as *const u8 as *const libc::c_char);
            l = SingleLinkedList_get_next(l);
        }
        fprintf(fp, b"}\0" as *const u8 as *const libc::c_char);
    }
    if !((*q).qts).is_null() {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << dim {
            fprintf(fp, b",(*b*){\0" as *const u8 as *const libc::c_char);
            QuadTree_print_internal(
                fp,
                *((*q).qts).offset(i as isize),
                level + 1 as libc::c_int,
            );
            fprintf(fp, b"}\0" as *const u8 as *const libc::c_char);
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_print(mut fp: *mut FILE, mut q: QuadTree) {
    if fp.is_null() {
        return;
    }
    if (*q).dim == 2 as libc::c_int {
        fprintf(fp, b"Graphics[{\0" as *const u8 as *const libc::c_char);
    } else if (*q).dim == 3 as libc::c_int {
        fprintf(fp, b"Graphics3D[{\0" as *const u8 as *const libc::c_char);
    } else {
        return
    }
    QuadTree_print_internal(fp, q, 0 as libc::c_int);
    if (*q).dim == 2 as libc::c_int {
        fprintf(
            fp,
            b"}, PlotRange -> All, Frame -> True, FrameTicks -> True]\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        fprintf(fp, b"}, PlotRange -> All]\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn QuadTree_get_nearest_internal(
    mut qt: QuadTree,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut min: *mut libc::c_double,
    mut imin: *mut libc::c_int,
    mut tentative: libc::c_int,
    mut flag: *mut libc::c_int,
) {
    let mut l: SingleLinkedList = 0 as *mut SingleLinkedList_struct;
    let mut coord: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut dim: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut iq: libc::c_int = -(1 as libc::c_int);
    let mut qmin: libc::c_double = 0.;
    let mut point: *mut libc::c_double = x;
    *flag = 0 as libc::c_int;
    if qt.is_null() {
        return;
    }
    dim = (*qt).dim;
    l = (*qt).l;
    if !l.is_null() {
        while !l.is_null() {
            coord = node_data_get_coord(SingleLinkedList_get_data(l));
            dist = point_distance(point, coord, dim);
            if *min < 0 as libc::c_int as libc::c_double || dist < *min {
                *min = dist;
                *imin = node_data_get_id(SingleLinkedList_get_data(l));
                i = 0 as libc::c_int;
                while i < dim {
                    *y.offset(i as isize) = *coord.offset(i as isize);
                    i += 1;
                }
            }
            l = SingleLinkedList_get_next(l);
        }
    }
    if !((*qt).qts).is_null() {
        dist = point_distance((*qt).center, point, dim);
        if *min >= 0 as libc::c_int as libc::c_double
            && dist - sqrt(dim as libc::c_double) * (*qt).width > *min
        {
            return
        } else {
            if tentative != 0 {
                qmin = -(1 as libc::c_int) as libc::c_double;
                i = 0 as libc::c_int;
                while i < (1 as libc::c_int) << dim {
                    if !(*((*qt).qts).offset(i as isize)).is_null() {
                        dist = point_distance(
                            (**((*qt).qts).offset(i as isize)).average,
                            point,
                            dim,
                        );
                        if dist < qmin || qmin < 0 as libc::c_int as libc::c_double {
                            qmin = dist;
                            iq = i;
                        }
                    }
                    i += 1;
                }
                if iq >= 0 as libc::c_int {} else {
                    __assert_fail(
                        b"iq >= 0\0" as *const u8 as *const libc::c_char,
                        b"QuadTree.c\0" as *const u8 as *const libc::c_char,
                        704 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"void QuadTree_get_nearest_internal(QuadTree, double *, double *, double *, int *, int, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                QuadTree_get_nearest_internal(
                    *((*qt).qts).offset(iq as isize),
                    x,
                    y,
                    min,
                    imin,
                    tentative,
                    flag,
                );
            } else {
                i = 0 as libc::c_int;
                while i < (1 as libc::c_int) << dim {
                    QuadTree_get_nearest_internal(
                        *((*qt).qts).offset(i as isize),
                        x,
                        y,
                        min,
                        imin,
                        tentative,
                        flag,
                    );
                    i += 1;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn QuadTree_get_nearest(
    mut qt: QuadTree,
    mut x: *mut libc::c_double,
    mut ymin: *mut libc::c_double,
    mut imin: *mut libc::c_int,
    mut min: *mut libc::c_double,
    mut flag: *mut libc::c_int,
) {
    *flag = 0 as libc::c_int;
    *min = -(1 as libc::c_int) as libc::c_double;
    QuadTree_get_nearest_internal(
        qt,
        x,
        ymin,
        min,
        imin,
        (0 as libc::c_int == 0) as libc::c_int,
        flag,
    );
    QuadTree_get_nearest_internal(qt, x, ymin, min, imin, 0 as libc::c_int, flag);
}
