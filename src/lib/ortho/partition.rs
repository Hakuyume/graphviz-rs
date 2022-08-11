#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn construct_trapezoids(
        _: libc::c_int,
        _: *mut segment_t,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: *mut trap_t,
    ) -> libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn drand48() -> libc::c_double;
    fn srand48(__seedval: libc::c_long);
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
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub flags: libc::c_int,
    pub nedges: libc::c_int,
    pub edges: [*mut sedge; 6],
    pub nsides: libc::c_int,
    pub sides: *mut *mut snode,
    pub bb: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snode {
    pub n_val: libc::c_int,
    pub n_idx: libc::c_int,
    pub n_dad: *mut snode,
    pub n_edge: *mut sedge,
    pub n_adj: libc::c_short,
    pub save_n_adj: libc::c_short,
    pub cells: [*mut cell; 2],
    pub adj_edge_list: *mut libc::c_int,
    pub index: libc::c_int,
    pub isVert: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sedge {
    pub weight: libc::c_double,
    pub cnt: libc::c_int,
    pub v1: libc::c_int,
    pub v2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trap_t {
    pub lseg: libc::c_int,
    pub rseg: libc::c_int,
    pub hi: pointf,
    pub lo: pointf,
    pub u0: libc::c_int,
    pub u1: libc::c_int,
    pub d0: libc::c_int,
    pub d1: libc::c_int,
    pub sink: libc::c_int,
    pub usave: libc::c_int,
    pub uside: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment_t {
    pub v0: pointf,
    pub v1: pointf,
    pub is_inserted: bool,
    pub root0: libc::c_int,
    pub root1: libc::c_int,
    pub next: libc::c_int,
    pub prev: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vertexchain_t {
    pub pt: pointf,
    pub vnext: [libc::c_int; 4],
    pub vpos: [libc::c_int; 4],
    pub nextfree: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monchain_t {
    pub vnum: libc::c_int,
    pub next: libc::c_int,
    pub prev: libc::c_int,
    pub marked: libc::c_int,
}
static mut chain_idx: libc::c_int = 0;
static mut mon_idx: libc::c_int = 0;
static mut mchain: *mut monchain_t = 0 as *const monchain_t as *mut monchain_t;
static mut vert: *mut vertexchain_t = 0 as *const vertexchain_t as *mut vertexchain_t;
static mut mon: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
unsafe extern "C" fn convert(
    mut bb: boxf,
    mut flip: libc::c_int,
    mut ccw: libc::c_int,
    mut pts: *mut pointf,
) {
    *pts.offset(0 as libc::c_int as isize) = bb.LL;
    *pts.offset(2 as libc::c_int as isize) = bb.UR;
    if ccw != 0 {
        (*pts.offset(1 as libc::c_int as isize)).x = bb.UR.x;
        (*pts.offset(1 as libc::c_int as isize)).y = bb.LL.y;
        (*pts.offset(3 as libc::c_int as isize)).x = bb.LL.x;
        (*pts.offset(3 as libc::c_int as isize)).y = bb.UR.y;
    } else {
        (*pts.offset(1 as libc::c_int as isize)).x = bb.LL.x;
        (*pts.offset(1 as libc::c_int as isize)).y = bb.UR.y;
        (*pts.offset(3 as libc::c_int as isize)).x = bb.UR.x;
        (*pts.offset(3 as libc::c_int as isize)).y = bb.LL.y;
    }
    if flip != 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut tmp: libc::c_double = (*pts.offset(i as isize)).y;
            (*pts.offset(i as isize)).y = (*pts.offset(i as isize)).x;
            (*pts.offset(i as isize)).x = -tmp;
            i += 1;
        }
    }
}
unsafe extern "C" fn store(
    mut seg: *mut segment_t,
    mut first: libc::c_int,
    mut pts: *mut pointf,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = first + 4 as libc::c_int - 1 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    i = first;
    while i <= last {
        if i == first {
            (*seg.offset(i as isize)).next = first + 1 as libc::c_int;
            (*seg.offset(i as isize)).prev = last;
        } else if i == last {
            (*seg.offset(i as isize)).next = first;
            (*seg.offset(i as isize)).prev = last - 1 as libc::c_int;
        } else {
            (*seg.offset(i as isize)).next = i + 1 as libc::c_int;
            (*seg.offset(i as isize)).prev = i - 1 as libc::c_int;
        }
        (*seg.offset(i as isize)).is_inserted = 0 as libc::c_int != 0;
        let ref mut fresh0 = (*seg.offset(i as isize)).v0;
        *fresh0 = *pts.offset(j as isize);
        (*seg.offset((*seg.offset(i as isize)).prev as isize)).v1 = *fresh0;
        i += 1;
        j += 1;
    }
    return last + 1 as libc::c_int;
}
unsafe extern "C" fn genSegments(
    mut cells: *mut cell,
    mut ncells: libc::c_int,
    mut bb: boxf,
    mut seg: *mut segment_t,
    mut flip: libc::c_int,
) {
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut pts: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    convert(bb, flip, 1 as libc::c_int, pts.as_mut_ptr());
    i = store(seg, i, pts.as_mut_ptr());
    j = 0 as libc::c_int;
    while j < ncells {
        convert(
            (*cells.offset(j as isize)).bb,
            flip,
            0 as libc::c_int,
            pts.as_mut_ptr(),
        );
        i = store(seg, i, pts.as_mut_ptr());
        j += 1;
    }
}
unsafe extern "C" fn generateRandomOrdering(
    mut n: libc::c_int,
    mut permute: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= n {
        *permute.offset(i as isize) = i;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i <= n {
        j = (i as libc::c_double
            + drand48() * (n + 1 as libc::c_int - i) as libc::c_double) as libc::c_int;
        if j != i {
            tmp = *permute.offset(i as isize);
            *permute.offset(i as isize) = *permute.offset(j as isize);
            *permute.offset(j as isize) = tmp;
        }
        i += 1;
    }
}
unsafe extern "C" fn inside_polygon(
    mut t: *mut trap_t,
    mut seg: *mut segment_t,
) -> bool {
    let mut rseg: libc::c_int = (*t).rseg;
    if (*t).state == 2 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (*t).lseg <= 0 as libc::c_int || (*t).rseg <= 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (*t).u0 <= 0 as libc::c_int && (*t).u1 <= 0 as libc::c_int
        || (*t).d0 <= 0 as libc::c_int && (*t).d1 <= 0 as libc::c_int
    {
        return if (*seg.offset(rseg as isize)).v1.y
            > (*seg.offset(rseg as isize)).v0.y + 1.0e-7f64
        {
            1 as libc::c_int
        } else if (*seg.offset(rseg as isize)).v1.y
                < (*seg.offset(rseg as isize)).v0.y - 1.0e-7f64
            {
            0 as libc::c_int
        } else {
            ((*seg.offset(rseg as isize)).v1.x > (*seg.offset(rseg as isize)).v0.x)
                as libc::c_int
        } != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn get_angle(
    mut vp0: *mut pointf,
    mut vpnext: *mut pointf,
    mut vp1: *mut pointf,
) -> libc::c_double {
    let mut v0: pointf = pointf { x: 0., y: 0. };
    let mut v1: pointf = pointf { x: 0., y: 0. };
    v0.x = (*vpnext).x - (*vp0).x;
    v0.y = (*vpnext).y - (*vp0).y;
    v1.x = (*vp1).x - (*vp0).x;
    v1.y = (*vp1).y - (*vp0).y;
    if v0.x * v1.y - v1.x * v0.y >= 0 as libc::c_int as libc::c_double {
        return (v0.x * v1.x + v0.y * v1.y) / hypot(v0.x, v0.y) / hypot(v1.x, v1.y)
    } else {
        return -1.0f64 * (v0.x * v1.x + v0.y * v1.y) / hypot(v0.x, v0.y)
            / hypot(v1.x, v1.y) - 2 as libc::c_int as libc::c_double
    };
}
unsafe extern "C" fn get_vertex_positions(
    mut v0: libc::c_int,
    mut v1: libc::c_int,
    mut ip: *mut libc::c_int,
    mut iq: *mut libc::c_int,
) {
    let mut vp0: *mut vertexchain_t = 0 as *mut vertexchain_t;
    let mut vp1: *mut vertexchain_t = 0 as *mut vertexchain_t;
    let mut i: libc::c_int = 0;
    let mut angle: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut tp: libc::c_int = 0 as libc::c_int;
    let mut tq: libc::c_int = 0 as libc::c_int;
    vp0 = &mut *vert.offset(v0 as isize) as *mut vertexchain_t;
    vp1 = &mut *vert.offset(v1 as isize) as *mut vertexchain_t;
    angle = -4.0f64;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !((*vp0).vnext[i as usize] <= 0 as libc::c_int) {
            temp = get_angle(
                &mut (*vp0).pt,
                &mut (*vert
                    .offset(*((*vp0).vnext).as_mut_ptr().offset(i as isize) as isize))
                    .pt,
                &mut (*vp1).pt,
            );
            if temp > angle {
                angle = temp;
                tp = i;
            }
        }
        i += 1;
    }
    *ip = tp;
    angle = -4.0f64;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !((*vp1).vnext[i as usize] <= 0 as libc::c_int) {
            temp = get_angle(
                &mut (*vp1).pt,
                &mut (*vert
                    .offset(*((*vp1).vnext).as_mut_ptr().offset(i as isize) as isize))
                    .pt,
                &mut (*vp0).pt,
            );
            if temp > angle {
                angle = temp;
                tq = i;
            }
        }
        i += 1;
    }
    *iq = tq;
}
unsafe extern "C" fn make_new_monotone_poly(
    mut mcur: libc::c_int,
    mut v0: libc::c_int,
    mut v1: libc::c_int,
) -> libc::c_int {
    let mut p: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut iq: libc::c_int = 0;
    mon_idx += 1;
    let mut mnew: libc::c_int = mon_idx;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nf0: libc::c_int = 0;
    let mut nf1: libc::c_int = 0;
    let mut vp0: *mut vertexchain_t = 0 as *mut vertexchain_t;
    let mut vp1: *mut vertexchain_t = 0 as *mut vertexchain_t;
    vp0 = &mut *vert.offset(v0 as isize) as *mut vertexchain_t;
    vp1 = &mut *vert.offset(v1 as isize) as *mut vertexchain_t;
    get_vertex_positions(v0, v1, &mut ip, &mut iq);
    p = (*vp0).vpos[ip as usize];
    q = (*vp1).vpos[iq as usize];
    chain_idx += 1;
    i = chain_idx;
    chain_idx += 1;
    j = chain_idx;
    (*mchain.offset(i as isize)).vnum = v0;
    (*mchain.offset(j as isize)).vnum = v1;
    (*mchain.offset(i as isize)).next = (*mchain.offset(p as isize)).next;
    (*mchain.offset((*mchain.offset(p as isize)).next as isize)).prev = i;
    (*mchain.offset(i as isize)).prev = j;
    (*mchain.offset(j as isize)).next = i;
    (*mchain.offset(j as isize)).prev = (*mchain.offset(q as isize)).prev;
    (*mchain.offset((*mchain.offset(q as isize)).prev as isize)).next = j;
    (*mchain.offset(p as isize)).next = q;
    (*mchain.offset(q as isize)).prev = p;
    nf0 = (*vp0).nextfree;
    nf1 = (*vp1).nextfree;
    (*vp0).vnext[ip as usize] = v1;
    (*vp0).vpos[nf0 as usize] = i;
    (*vp0)
        .vnext[nf0
        as usize] = (*mchain.offset((*mchain.offset(i as isize)).next as isize)).vnum;
    (*vp1).vpos[nf1 as usize] = j;
    (*vp1).vnext[nf1 as usize] = v0;
    let ref mut fresh1 = (*vp0).nextfree;
    *fresh1 += 1;
    let ref mut fresh2 = (*vp1).nextfree;
    *fresh2 += 1;
    *mon.offset(mcur as isize) = p;
    *mon.offset(mnew as isize) = i;
    return mnew;
}
unsafe extern "C" fn traverse_polygon(
    mut visited: *mut libc::c_int,
    mut decomp: *mut boxf,
    mut size: libc::c_int,
    mut seg: *mut segment_t,
    mut tr: *mut trap_t,
    mut mcur: libc::c_int,
    mut trnum: libc::c_int,
    mut from: libc::c_int,
    mut flip: libc::c_int,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut t: *mut trap_t = 0 as *mut trap_t;
    let mut mnew: libc::c_int = 0;
    let mut v0: libc::c_int = 0;
    let mut v1: libc::c_int = 0;
    if trnum <= 0 as libc::c_int || *visited.offset(trnum as isize) != 0 {
        return size;
    }
    t = &mut *tr.offset(trnum as isize) as *mut trap_t;
    *visited.offset(trnum as isize) = (0 as libc::c_int == 0) as libc::c_int;
    if (*t).hi.y > (*t).lo.y + 1.0e-7f64
        && fabs(
            (*seg.offset((*t).lseg as isize)).v0.x
                - (*seg.offset((*t).lseg as isize)).v1.x,
        ) <= 1.0e-7f64
        && fabs(
            (*seg.offset((*t).rseg as isize)).v0.x
                - (*seg.offset((*t).rseg as isize)).v1.x,
        ) <= 1.0e-7f64
    {
        if flip != 0 {
            (*decomp.offset(size as isize)).LL.x = (*t).lo.y;
            (*decomp.offset(size as isize))
                .LL
                .y = -(*seg.offset((*t).rseg as isize)).v0.x;
            (*decomp.offset(size as isize)).UR.x = (*t).hi.y;
            (*decomp.offset(size as isize))
                .UR
                .y = -(*seg.offset((*t).lseg as isize)).v0.x;
        } else {
            (*decomp.offset(size as isize))
                .LL
                .x = (*seg.offset((*t).lseg as isize)).v0.x;
            (*decomp.offset(size as isize)).LL.y = (*t).lo.y;
            (*decomp.offset(size as isize))
                .UR
                .x = (*seg.offset((*t).rseg as isize)).v0.x;
            (*decomp.offset(size as isize)).UR.y = (*t).hi.y;
        }
        size += 1;
    }
    if (*t).u0 <= 0 as libc::c_int && (*t).u1 <= 0 as libc::c_int {
        if (*t).d0 > 0 as libc::c_int && (*t).d1 > 0 as libc::c_int {
            v0 = (*tr.offset((*t).d1 as isize)).lseg;
            v1 = (*t).lseg;
            if from == (*t).d1 {
                mnew = make_new_monotone_poly(mcur, v1, v0);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
            } else {
                mnew = make_new_monotone_poly(mcur, v0, v1);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
            }
        } else {
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).u0,
                trnum,
                flip,
                2 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).u1,
                trnum,
                flip,
                2 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).d0,
                trnum,
                flip,
                1 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).d1,
                trnum,
                flip,
                1 as libc::c_int,
            );
        }
    } else if (*t).d0 <= 0 as libc::c_int && (*t).d1 <= 0 as libc::c_int {
        if (*t).u0 > 0 as libc::c_int && (*t).u1 > 0 as libc::c_int {
            v0 = (*t).rseg;
            v1 = (*tr.offset((*t).u0 as isize)).rseg;
            if from == (*t).u1 {
                mnew = make_new_monotone_poly(mcur, v1, v0);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
            } else {
                mnew = make_new_monotone_poly(mcur, v0, v1);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
            }
        } else {
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).u0,
                trnum,
                flip,
                2 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).u1,
                trnum,
                flip,
                2 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).d0,
                trnum,
                flip,
                1 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).d1,
                trnum,
                flip,
                1 as libc::c_int,
            );
        }
    } else if (*t).u0 > 0 as libc::c_int && (*t).u1 > 0 as libc::c_int {
        if (*t).d0 > 0 as libc::c_int && (*t).d1 > 0 as libc::c_int {
            v0 = (*tr.offset((*t).d1 as isize)).lseg;
            v1 = (*tr.offset((*t).u0 as isize)).rseg;
            if dir == 2 as libc::c_int && (*t).d1 == from
                || dir == 1 as libc::c_int && (*t).u1 == from
            {
                mnew = make_new_monotone_poly(mcur, v1, v0);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
            } else {
                mnew = make_new_monotone_poly(mcur, v0, v1);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
            }
        } else if fabs((*t).lo.y - (*seg.offset((*t).lseg as isize)).v1.y) <= 1.0e-7f64
                && fabs((*t).lo.x - (*seg.offset((*t).lseg as isize)).v1.x) <= 1.0e-7f64
            {
            v0 = (*tr.offset((*t).u0 as isize)).rseg;
            v1 = (*seg.offset((*t).lseg as isize)).next;
            if dir == 1 as libc::c_int && (*t).u0 == from {
                mnew = make_new_monotone_poly(mcur, v1, v0);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
            } else {
                mnew = make_new_monotone_poly(mcur, v0, v1);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
            }
        } else {
            v0 = (*t).rseg;
            v1 = (*tr.offset((*t).u0 as isize)).rseg;
            if dir == 1 as libc::c_int && (*t).u1 == from {
                mnew = make_new_monotone_poly(mcur, v1, v0);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
            } else {
                mnew = make_new_monotone_poly(mcur, v0, v1);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
            }
        }
    } else if (*t).u0 > 0 as libc::c_int || (*t).u1 > 0 as libc::c_int {
        if (*t).d0 > 0 as libc::c_int && (*t).d1 > 0 as libc::c_int {
            if fabs((*t).hi.y - (*seg.offset((*t).lseg as isize)).v0.y) <= 1.0e-7f64
                && fabs((*t).hi.x - (*seg.offset((*t).lseg as isize)).v0.x) <= 1.0e-7f64
            {
                v0 = (*tr.offset((*t).d1 as isize)).lseg;
                v1 = (*t).lseg;
                if !(dir == 2 as libc::c_int && (*t).d0 == from) {
                    mnew = make_new_monotone_poly(mcur, v1, v0);
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).u1,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).d1,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).u0,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).d0,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                } else {
                    mnew = make_new_monotone_poly(mcur, v0, v1);
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).d0,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).u0,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).u1,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).d1,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                }
            } else {
                v0 = (*tr.offset((*t).d1 as isize)).lseg;
                v1 = (*seg.offset((*t).rseg as isize)).next;
                if dir == 2 as libc::c_int && (*t).d1 == from {
                    mnew = make_new_monotone_poly(mcur, v1, v0);
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).d1,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).u1,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).u0,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).d0,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                } else {
                    mnew = make_new_monotone_poly(mcur, v0, v1);
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).u0,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).d0,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mcur,
                        (*t).u1,
                        trnum,
                        flip,
                        2 as libc::c_int,
                    );
                    size = traverse_polygon(
                        visited,
                        decomp,
                        size,
                        seg,
                        tr,
                        mnew,
                        (*t).d1,
                        trnum,
                        flip,
                        1 as libc::c_int,
                    );
                }
            }
        } else if fabs((*t).hi.y - (*seg.offset((*t).lseg as isize)).v0.y) <= 1.0e-7f64
                && fabs((*t).hi.x - (*seg.offset((*t).lseg as isize)).v0.x) <= 1.0e-7f64
                && (fabs((*t).lo.y - (*seg.offset((*t).rseg as isize)).v0.y) <= 1.0e-7f64
                    && fabs((*t).lo.x - (*seg.offset((*t).rseg as isize)).v0.x)
                        <= 1.0e-7f64)
            {
            v0 = (*t).rseg;
            v1 = (*t).lseg;
            if dir == 1 as libc::c_int {
                mnew = make_new_monotone_poly(mcur, v1, v0);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
            } else {
                mnew = make_new_monotone_poly(mcur, v0, v1);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
            }
        } else if fabs((*t).hi.y - (*seg.offset((*t).rseg as isize)).v1.y) <= 1.0e-7f64
                && fabs((*t).hi.x - (*seg.offset((*t).rseg as isize)).v1.x) <= 1.0e-7f64
                && (fabs((*t).lo.y - (*seg.offset((*t).lseg as isize)).v1.y) <= 1.0e-7f64
                    && fabs((*t).lo.x - (*seg.offset((*t).lseg as isize)).v1.x)
                        <= 1.0e-7f64)
            {
            v0 = (*seg.offset((*t).rseg as isize)).next;
            v1 = (*seg.offset((*t).lseg as isize)).next;
            if dir == 1 as libc::c_int {
                mnew = make_new_monotone_poly(mcur, v1, v0);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
            } else {
                mnew = make_new_monotone_poly(mcur, v0, v1);
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d1,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mcur,
                    (*t).d0,
                    trnum,
                    flip,
                    1 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u0,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
                size = traverse_polygon(
                    visited,
                    decomp,
                    size,
                    seg,
                    tr,
                    mnew,
                    (*t).u1,
                    trnum,
                    flip,
                    2 as libc::c_int,
                );
            }
        } else {
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).u0,
                trnum,
                flip,
                2 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).d0,
                trnum,
                flip,
                1 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).u1,
                trnum,
                flip,
                2 as libc::c_int,
            );
            size = traverse_polygon(
                visited,
                decomp,
                size,
                seg,
                tr,
                mcur,
                (*t).d1,
                trnum,
                flip,
                1 as libc::c_int,
            );
        }
    }
    return size;
}
unsafe extern "C" fn monotonate_trapezoids(
    mut nsegs: libc::c_int,
    mut seg: *mut segment_t,
    mut tr: *mut trap_t,
    mut flip: libc::c_int,
    mut decomp: *mut boxf,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tr_start: libc::c_int = 0;
    let mut tr_size: libc::c_int = 5 as libc::c_int * nsegs + 1 as libc::c_int;
    let mut visited: *mut libc::c_int = gcalloc(
        tr_size as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    mchain = gcalloc(
        tr_size as size_t,
        ::std::mem::size_of::<monchain_t>() as libc::c_ulong,
    ) as *mut monchain_t;
    vert = gcalloc(
        (nsegs + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<vertexchain_t>() as libc::c_ulong,
    ) as *mut vertexchain_t;
    mon = gcalloc(nsegs as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int * nsegs + 1 as libc::c_int {
        if inside_polygon(&mut *tr.offset(i as isize), seg) {
            break;
        }
        i += 1;
    }
    tr_start = i;
    i = 1 as libc::c_int;
    while i <= nsegs {
        (*mchain.offset(i as isize)).prev = (*seg.offset(i as isize)).prev;
        (*mchain.offset(i as isize)).next = (*seg.offset(i as isize)).next;
        (*mchain.offset(i as isize)).vnum = i;
        (*vert.offset(i as isize)).pt = (*seg.offset(i as isize)).v0;
        (*vert.offset(i as isize))
            .vnext[0 as libc::c_int as usize] = (*seg.offset(i as isize)).next;
        (*vert.offset(i as isize)).vpos[0 as libc::c_int as usize] = i;
        (*vert.offset(i as isize)).nextfree = 1 as libc::c_int;
        i += 1;
    }
    chain_idx = nsegs;
    mon_idx = 0 as libc::c_int;
    *mon.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
    if (*tr.offset(tr_start as isize)).u0 > 0 as libc::c_int {
        size = traverse_polygon(
            visited,
            decomp,
            0 as libc::c_int,
            seg,
            tr,
            0 as libc::c_int,
            tr_start,
            (*tr.offset(tr_start as isize)).u0,
            flip,
            1 as libc::c_int,
        );
    } else if (*tr.offset(tr_start as isize)).d0 > 0 as libc::c_int {
        size = traverse_polygon(
            visited,
            decomp,
            0 as libc::c_int,
            seg,
            tr,
            0 as libc::c_int,
            tr_start,
            (*tr.offset(tr_start as isize)).d0,
            flip,
            2 as libc::c_int,
        );
    } else {
        size = 0 as libc::c_int;
    }
    free(visited as *mut libc::c_void);
    free(mchain as *mut libc::c_void);
    free(vert as *mut libc::c_void);
    free(mon as *mut libc::c_void);
    return size;
}
unsafe extern "C" fn rectIntersect(
    mut d: *mut boxf,
    mut r0: *const boxf,
    mut r1: *const boxf,
) -> bool {
    let mut t: libc::c_double = fmax((*r0).LL.x, (*r1).LL.x);
    (*d).UR.x = fmin((*r0).UR.x, (*r1).UR.x);
    (*d).LL.x = t;
    t = fmax((*r0).LL.y, (*r1).LL.y);
    (*d).UR.y = fmin((*r0).UR.y, (*r1).UR.y);
    (*d).LL.y = t;
    return !((*d).LL.x >= (*d).UR.x || (*d).LL.y >= (*d).UR.y);
}
#[no_mangle]
pub unsafe extern "C" fn partition(
    mut cells: *mut cell,
    mut ncells: libc::c_int,
    mut nrects: *mut libc::c_int,
    mut bb: boxf,
) -> *mut boxf {
    let mut nsegs: libc::c_int = 4 as libc::c_int * (ncells + 1 as libc::c_int);
    let mut segs: *mut segment_t = gcalloc(
        (nsegs + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<segment_t>() as libc::c_ulong,
    ) as *mut segment_t;
    let mut permute: *mut libc::c_int = gcalloc(
        (nsegs + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut hd_size: libc::c_int = 0;
    let mut vd_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut rs: *mut boxf = 0 as *mut boxf;
    let mut ntraps: libc::c_int = 5 as libc::c_int * nsegs + 1 as libc::c_int;
    let mut trs: *mut trap_t = gcalloc(
        ntraps as size_t,
        ::std::mem::size_of::<trap_t>() as libc::c_ulong,
    ) as *mut trap_t;
    let mut hor_decomp: *mut boxf = gcalloc(
        ntraps as size_t,
        ::std::mem::size_of::<boxf>() as libc::c_ulong,
    ) as *mut boxf;
    let mut vert_decomp: *mut boxf = gcalloc(
        ntraps as size_t,
        ::std::mem::size_of::<boxf>() as libc::c_ulong,
    ) as *mut boxf;
    let mut nt: libc::c_int = 0;
    genSegments(cells, ncells, bb, segs, 0 as libc::c_int);
    srand48(173 as libc::c_int as libc::c_long);
    generateRandomOrdering(nsegs, permute);
    nt = construct_trapezoids(nsegs, segs, permute, ntraps, trs);
    hd_size = monotonate_trapezoids(nsegs, segs, trs, 0 as libc::c_int, hor_decomp);
    genSegments(cells, ncells, bb, segs, 1 as libc::c_int);
    generateRandomOrdering(nsegs, permute);
    nt = construct_trapezoids(nsegs, segs, permute, ntraps, trs);
    vd_size = monotonate_trapezoids(nsegs, segs, trs, 1 as libc::c_int, vert_decomp);
    rs = gcalloc(
        (hd_size * vd_size) as size_t,
        ::std::mem::size_of::<boxf>() as libc::c_ulong,
    ) as *mut boxf;
    i = 0 as libc::c_int;
    while i < vd_size {
        j = 0 as libc::c_int;
        while j < hd_size {
            if rectIntersect(
                &mut *rs.offset(cnt as isize),
                &mut *vert_decomp.offset(i as isize),
                &mut *hor_decomp.offset(j as isize),
            ) {
                cnt += 1;
            }
            j += 1;
        }
        i += 1;
    }
    rs = grealloc(
        rs as *mut libc::c_void,
        (cnt as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<boxf>() as libc::c_ulong),
    ) as *mut boxf;
    free(segs as *mut libc::c_void);
    free(permute as *mut libc::c_void);
    free(trs as *mut libc::c_void);
    free(hor_decomp as *mut libc::c_void);
    free(vert_decomp as *mut libc::c_void);
    *nrects = cnt;
    return rs;
}
