#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn RTreeInsert(
        _: *mut RTree_t,
        _: *mut Rect_t,
        _: *mut libc::c_void,
        _: *mut *mut Node_t,
        _: libc::c_int,
    ) -> libc::c_int;
    fn RTreeClose(rtp: *mut RTree_t) -> libc::c_int;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn RTreeOpen() -> *mut RTree_t;
    static mut Dtobag: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn RTreeSearch(_: *mut RTree_t, _: *mut Node_t, _: *mut Rect_t) -> *mut LeafList_t;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn RTreeLeafListFree(llp: *mut LeafList_t);
    fn zmalloc(_: size_t) -> *mut libc::c_void;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
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
pub struct xlabel_t {
    pub sz: pointf,
    pub pos: pointf,
    pub lbl: *mut libc::c_void,
    pub set: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_t {
    pub pos: pointf,
    pub sz: pointf,
    pub lbl: *mut xlabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct label_params_t {
    pub bb: boxf,
    pub force: libc::c_uchar,
}
pub type XLabels_t = XLabels_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XLabels_s {
    pub objs: *mut object_t,
    pub n_objs: libc::c_int,
    pub lbls: *mut xlabel_t,
    pub n_lbls: libc::c_int,
    pub params: *mut label_params_t,
    pub hdx: *mut Dt_t,
    pub spdx: *mut RTree_t,
}
pub type RTree_t = RTree;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RTree {
    pub root: *mut Node_t,
    pub split: SplitQ_t,
    pub MinFill: libc::c_int,
}
pub type SplitQ_t = split_q_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct split_q_s {
    pub BranchBuf: [Branch; 65],
    pub CoverSplit: Rect,
    pub CoverSplitArea: libc::c_uint,
    pub Partitions: [PartitionVars; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionVars {
    pub partition: [libc::c_int; 65],
    pub taken: [libc::c_int; 65],
    pub count: [libc::c_int; 2],
    pub cover: [Rect; 2],
    pub area: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub boundary: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Branch {
    pub rect: Rect_t,
    pub child: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub count: libc::c_int,
    pub level: libc::c_int,
    pub branch: [Branch; 64],
}
pub type Rect_t = Rect;
pub type Node_t = Node;
pub type Dt_t = _dt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dt_s {
    pub searchf: Dtsearch_f,
    pub disc: *mut Dtdisc_t,
    pub data: *mut Dtdata_t,
    pub memoryf: Dtmemory_f,
    pub meth: *mut Dtmethod_t,
    pub type_0: libc::c_int,
    pub nview: libc::c_int,
    pub view: *mut Dt_t,
    pub walk: *mut Dt_t,
    pub user: *mut libc::c_void,
}
pub type Dtmethod_t = _dtmethod_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtmethod_s {
    pub searchf: Dtsearch_f,
    pub type_0: libc::c_int,
}
pub type Dtsearch_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void,
>;
pub type Dtmemory_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        size_t,
        *mut Dtdisc_t,
    ) -> *mut libc::c_void,
>;
pub type Dtdisc_t = _dtdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdisc_s {
    pub key: libc::c_int,
    pub size: libc::c_int,
    pub link: libc::c_int,
    pub makef: Dtmake_f,
    pub freef: Dtfree_f,
    pub comparf: Dtcompar_f,
    pub hashf: Dthash_f,
    pub memoryf: Dtmemory_f,
    pub eventf: Dtevent_f,
}
pub type Dtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dthash_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_uint,
>;
pub type Dtcompar_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dtfree_f = Option::<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
>;
pub type Dtmake_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> *mut libc::c_void,
>;
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
}
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
pub type BestPos_t = best_p_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct best_p_s {
    pub n: libc::c_int,
    pub area: libc::c_double,
    pub pos: pointf,
}
pub type LeafList_t = LeafList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LeafList {
    pub next: *mut LeafList,
    pub leaf: *mut Leaf_t,
}
pub type Leaf_t = Leaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Leaf {
    pub rect: Rect_t,
    pub data: *mut libc::c_void,
}
pub type Dthold_t = _dthold_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dthold_s {
    pub hdr: Dtlink_t,
    pub obj: *mut libc::c_void,
}
pub type HDict_t = obyh;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obyh {
    pub link: Dtlink_t,
    pub key: libc::c_int,
    pub d: Leaf_t,
}
#[no_mangle]
pub static mut Hdisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            link: -(1 as libc::c_int),
            makef: None,
            freef: None,
            comparf: Some(
                icompare
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn icompare(
    mut dt: *mut Dt_t,
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut k1: libc::c_int = *(v1 as *mut libc::c_int);
    let mut k2: libc::c_int = *(v2 as *mut libc::c_int);
    if k1 < k2 {
        return -(1 as libc::c_int);
    }
    if k1 > k2 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xlnew(
    mut objs: *mut object_t,
    mut n_objs: libc::c_int,
    mut lbls: *mut xlabel_t,
    mut n_lbls: libc::c_int,
    mut params: *mut label_params_t,
) -> *mut XLabels_t {
    let mut xlp: *mut XLabels_t = zmalloc(
        ::std::mem::size_of::<XLabels_t>() as libc::c_ulong,
    ) as *mut XLabels_t;
    let ref mut fresh0 = (*xlp).hdx;
    *fresh0 = dtopen(&mut Hdisc, Dtobag);
    if (*fresh0).is_null() {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
    } else {
        let ref mut fresh1 = (*xlp).spdx;
        *fresh1 = RTreeOpen();
        if (*fresh1).is_null() {
            fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        } else {
            let ref mut fresh2 = (*xlp).objs;
            *fresh2 = objs;
            (*xlp).n_objs = n_objs;
            let ref mut fresh3 = (*xlp).lbls;
            *fresh3 = lbls;
            (*xlp).n_lbls = n_lbls;
            let ref mut fresh4 = (*xlp).params;
            *fresh4 = params;
            return xlp;
        }
    }
    if !((*xlp).hdx).is_null() {
        dtclose((*xlp).hdx);
    }
    if !((*xlp).spdx).is_null() {
        RTreeClose((*xlp).spdx);
    }
    free(xlp as *mut libc::c_void);
    return 0 as *mut XLabels_t;
}
unsafe extern "C" fn xlfree(mut xlp: *mut XLabels_t) {
    RTreeClose((*xlp).spdx);
    free(xlp as *mut libc::c_void);
}
unsafe extern "C" fn xlhorder(mut xlp: *mut XLabels_t) -> libc::c_uint {
    let mut maxx: libc::c_double = (*(*xlp).params).bb.UR.x;
    let mut maxy: libc::c_double = (*(*xlp).params).bb.UR.y;
    return (floor(log2(round(fmax(maxx, maxy)))) as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn hd_hil_s_from_xy(mut p: point, mut n: libc::c_int) -> libc::c_uint {
    let mut x: libc::c_int = p.x;
    let mut y: libc::c_int = p.y;
    let mut s: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = n - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut xi: libc::c_int = x >> i & 1 as libc::c_int;
        let mut yi: libc::c_int = y >> i & 1 as libc::c_int;
        s = (4 as libc::c_int as libc::c_uint)
            .wrapping_mul(s)
            .wrapping_add((2 as libc::c_int * xi) as libc::c_uint)
            .wrapping_add((xi ^ yi) as libc::c_uint);
        x = x ^ y;
        y = y ^ x & yi - 1 as libc::c_int;
        x = x ^ y;
        x = x ^ -xi & yi - 1 as libc::c_int;
        y = y ^ -xi & yi - 1 as libc::c_int;
        i -= 1;
    }
    return s;
}
unsafe extern "C" fn aabbaabb(mut r: *mut Rect_t, mut s: *mut Rect_t) -> libc::c_double {
    if (*r).boundary[2 as libc::c_int as usize]
        < (*s).boundary[0 as libc::c_int as usize]
        || (*r).boundary[0 as libc::c_int as usize]
            > (*s).boundary[2 as libc::c_int as usize]
    {
        return 0 as libc::c_int as libc::c_double;
    }
    if (*r).boundary[3 as libc::c_int as usize]
        < (*s).boundary[1 as libc::c_int as usize]
        || (*r).boundary[1 as libc::c_int as usize]
            > (*s).boundary[3 as libc::c_int as usize]
    {
        return 0 as libc::c_int as libc::c_double;
    }
    let mut iminx: libc::c_double = (if (*r).boundary[0 as libc::c_int as usize]
        > (*s).boundary[0 as libc::c_int as usize]
    {
        (*r).boundary[0 as libc::c_int as usize]
    } else {
        (*s).boundary[0 as libc::c_int as usize]
    }) as libc::c_double;
    let mut iminy: libc::c_double = (if (*r).boundary[1 as libc::c_int as usize]
        > (*s).boundary[1 as libc::c_int as usize]
    {
        (*r).boundary[1 as libc::c_int as usize]
    } else {
        (*s).boundary[1 as libc::c_int as usize]
    }) as libc::c_double;
    let mut imaxx: libc::c_double = (if (*r).boundary[2 as libc::c_int as usize]
        < (*s).boundary[2 as libc::c_int as usize]
    {
        (*r).boundary[2 as libc::c_int as usize]
    } else {
        (*s).boundary[2 as libc::c_int as usize]
    }) as libc::c_double;
    let mut imaxy: libc::c_double = (if (*r).boundary[3 as libc::c_int as usize]
        < (*s).boundary[3 as libc::c_int as usize]
    {
        (*r).boundary[3 as libc::c_int as usize]
    } else {
        (*s).boundary[3 as libc::c_int as usize]
    }) as libc::c_double;
    return (imaxx - iminx) * (imaxy - iminy);
}
unsafe extern "C" fn lblenclosing(
    mut objp: *mut object_t,
    mut objp1: *mut object_t,
) -> libc::c_int {
    let mut xlp: *mut xlabel_t = (*objp).lbl;
    if (*objp1).sz.x == 0 as libc::c_int as libc::c_double
        && (*objp1).sz.y == 0 as libc::c_int as libc::c_double
    {} else {
        __assert_fail(
            b"objp1->sz.x == 0 && objp1->sz.y == 0\0" as *const u8
                as *const libc::c_char,
            b"xlabels.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int lblenclosing(object_t *, object_t *)\0"))
                .as_ptr(),
        );
    }
    if xlp.is_null() {
        return 0 as libc::c_int;
    }
    return ((*objp1).pos.x > (*xlp).pos.x && (*objp1).pos.x < (*xlp).pos.x + (*xlp).sz.x
        && (*objp1).pos.y > (*xlp).pos.y && (*objp1).pos.y < (*xlp).pos.y + (*xlp).sz.y)
        as libc::c_int;
}
unsafe extern "C" fn objp2rect(mut op: *mut object_t, mut r: *mut Rect_t) {
    (*r).boundary[0 as libc::c_int as usize] = (*op).pos.x as libc::c_int;
    (*r).boundary[1 as libc::c_int as usize] = (*op).pos.y as libc::c_int;
    (*r).boundary[2 as libc::c_int as usize] = ((*op).pos.x + (*op).sz.x) as libc::c_int;
    (*r).boundary[3 as libc::c_int as usize] = ((*op).pos.y + (*op).sz.y) as libc::c_int;
}
unsafe extern "C" fn objplp2rect(mut objp: *mut object_t, mut r: *mut Rect_t) {
    let mut lp: *mut xlabel_t = (*objp).lbl;
    (*r).boundary[0 as libc::c_int as usize] = (*lp).pos.x as libc::c_int;
    (*r).boundary[1 as libc::c_int as usize] = (*lp).pos.y as libc::c_int;
    (*r).boundary[2 as libc::c_int as usize] = ((*lp).pos.x + (*lp).sz.x) as libc::c_int;
    (*r).boundary[3 as libc::c_int as usize] = ((*lp).pos.y + (*lp).sz.y) as libc::c_int;
}
unsafe extern "C" fn objplpmks(mut objp: *mut object_t) -> Rect_t {
    let mut rect: Rect_t = Rect_t { boundary: [0; 4] };
    let mut p: pointf = pointf { x: 0., y: 0. };
    p.y = 0.0f64;
    p.x = p.y;
    if !((*objp).lbl).is_null() {
        p = (*(*objp).lbl).sz;
    }
    rect.boundary[0 as libc::c_int as usize] = floor((*objp).pos.x - p.x) as libc::c_int;
    rect.boundary[1 as libc::c_int as usize] = floor((*objp).pos.y - p.y) as libc::c_int;
    rect
        .boundary[2 as libc::c_int
        as usize] = ceil((*objp).pos.x + (*objp).sz.x + p.x) as libc::c_int;
    if rect.boundary[2 as libc::c_int as usize] < 2147483647 as libc::c_int {} else {
        __assert_fail(
            b"rect.boundary[2] < INT_MAX\0" as *const u8 as *const libc::c_char,
            b"xlabels.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"Rect_t objplpmks(object_t *)\0"))
                .as_ptr(),
        );
    }
    rect
        .boundary[3 as libc::c_int
        as usize] = ceil((*objp).pos.y + (*objp).sz.y + p.y) as libc::c_int;
    if rect.boundary[3 as libc::c_int as usize] < 2147483647 as libc::c_int {} else {
        __assert_fail(
            b"rect.boundary[3] < INT_MAX\0" as *const u8 as *const libc::c_char,
            b"xlabels.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"Rect_t objplpmks(object_t *)\0"))
                .as_ptr(),
        );
    }
    return rect;
}
unsafe extern "C" fn getintrsxi(
    mut op: *mut object_t,
    mut cp: *mut object_t,
) -> libc::c_int {
    let mut i: libc::c_int = -(1 as libc::c_int);
    let mut lp: *mut xlabel_t = (*op).lbl;
    let mut clp: *mut xlabel_t = (*cp).lbl;
    if lp != clp {} else {
        __assert_fail(
            b"lp != clp\0" as *const u8 as *const libc::c_char,
            b"xlabels.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int getintrsxi(object_t *, object_t *)\0"))
                .as_ptr(),
        );
    }
    if (*lp).set as libc::c_int == 0 as libc::c_int
        || (*clp).set as libc::c_int == 0 as libc::c_int
    {
        return i;
    }
    if (*op).pos.x == 0.0f64 && (*op).pos.y == 0.0f64
        || (*cp).pos.x == 0.0f64 && (*cp).pos.y == 0.0f64
    {
        return i;
    }
    if (*cp).pos.y < (*op).pos.y {
        if (*cp).pos.x < (*op).pos.x {
            i = 0 as libc::c_int;
        } else if (*cp).pos.x > (*op).pos.x {
            i = 2 as libc::c_int;
        } else {
            i = 1 as libc::c_int;
        }
    } else if (*cp).pos.y > (*op).pos.y {
        if (*cp).pos.x < (*op).pos.x {
            i = 6 as libc::c_int;
        } else if (*cp).pos.x > (*op).pos.x {
            i = 8 as libc::c_int;
        } else {
            i = 7 as libc::c_int;
        }
    } else if (*cp).pos.x < (*op).pos.x {
        i = 3 as libc::c_int;
    } else if (*cp).pos.x > (*op).pos.x {
        i = 5 as libc::c_int;
    }
    return i;
}
unsafe extern "C" fn recordointrsx(
    mut op: *mut object_t,
    mut cp: *mut object_t,
    mut rp: *mut Rect_t,
    mut a: libc::c_double,
    mut intrsx: *mut *mut object_t,
) -> libc::c_double {
    let mut i: libc::c_int = getintrsxi(op, cp);
    if i < 0 as libc::c_int {
        i = 5 as libc::c_int;
    }
    if !(*intrsx.offset(i as isize)).is_null() {
        let mut sa: libc::c_double = 0.;
        let mut maxa: libc::c_double = 0.0f64;
        let mut srect: Rect_t = Rect_t { boundary: [0; 4] };
        objp2rect(*intrsx.offset(i as isize), &mut srect);
        sa = aabbaabb(rp, &mut srect);
        if sa > a {
            maxa = sa;
        }
        if !((**intrsx.offset(i as isize)).lbl).is_null() {
            objplp2rect(*intrsx.offset(i as isize), &mut srect);
            sa = aabbaabb(rp, &mut srect);
            if sa > a {
                maxa = fmax(sa, maxa);
            }
        }
        if maxa > 0.0f64 {
            return maxa;
        }
        let ref mut fresh5 = *intrsx.offset(i as isize);
        *fresh5 = cp;
        return a;
    }
    let ref mut fresh6 = *intrsx.offset(i as isize);
    *fresh6 = cp;
    return a;
}
unsafe extern "C" fn recordlintrsx(
    mut op: *mut object_t,
    mut cp: *mut object_t,
    mut rp: *mut Rect_t,
    mut a: libc::c_double,
    mut intrsx: *mut *mut object_t,
) -> libc::c_double {
    let mut i: libc::c_int = getintrsxi(op, cp);
    if i < 0 as libc::c_int {
        i = 5 as libc::c_int;
    }
    if !(*intrsx.offset(i as isize)).is_null() {
        let mut sa: libc::c_double = 0.;
        let mut maxa: libc::c_double = 0.0f64;
        let mut srect: Rect_t = Rect_t { boundary: [0; 4] };
        objp2rect(*intrsx.offset(i as isize), &mut srect);
        sa = aabbaabb(rp, &mut srect);
        if sa > a {
            maxa = sa;
        }
        if !((**intrsx.offset(i as isize)).lbl).is_null() {
            objplp2rect(*intrsx.offset(i as isize), &mut srect);
            sa = aabbaabb(rp, &mut srect);
            if sa > a {
                maxa = fmax(sa, maxa);
            }
        }
        if maxa > 0.0f64 {
            return maxa;
        }
        let ref mut fresh7 = *intrsx.offset(i as isize);
        *fresh7 = cp;
        return a;
    }
    let ref mut fresh8 = *intrsx.offset(i as isize);
    *fresh8 = cp;
    return a;
}
unsafe extern "C" fn xlintersections(
    mut xlp: *mut XLabels_t,
    mut objp: *mut object_t,
    mut intrsx: *mut *mut object_t,
) -> BestPos_t {
    let mut rect: Rect_t = Rect_t { boundary: [0; 4] };
    let mut srect: Rect_t = Rect_t { boundary: [0; 4] };
    let mut bp: BestPos_t = BestPos_t {
        n: 0,
        area: 0.,
        pos: pointf { x: 0., y: 0. },
    };
    if !((*objp).lbl).is_null() {} else {
        __assert_fail(
            b"objp->lbl\0" as *const u8 as *const libc::c_char,
            b"xlabels.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"BestPos_t xlintersections(XLabels_t *, object_t *, object_t **)\0"))
                .as_ptr(),
        );
    }
    bp.n = 0 as libc::c_int;
    bp.area = 0.0f64;
    bp.pos = (*(*objp).lbl).pos;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*xlp).n_objs {
        if !(objp == &mut *((*xlp).objs).offset(i as isize) as *mut object_t) {
            if !((*((*xlp).objs).offset(i as isize)).sz.x
                > 0 as libc::c_int as libc::c_double
                && (*((*xlp).objs).offset(i as isize)).sz.y
                    > 0 as libc::c_int as libc::c_double)
            {
                if lblenclosing(objp, &mut *((*xlp).objs).offset(i as isize)) != 0 {
                    bp.n += 1;
                }
            }
        }
        i += 1;
    }
    objplp2rect(objp, &mut rect);
    let mut llp: *mut LeafList_t = RTreeSearch(
        (*xlp).spdx,
        (*(*xlp).spdx).root,
        &mut rect,
    );
    if llp.is_null() {
        return bp;
    }
    let mut ilp: *mut LeafList_t = llp;
    while !ilp.is_null() {
        let mut a: libc::c_double = 0.;
        let mut ra: libc::c_double = 0.;
        let mut cp: *mut object_t = (*(*ilp).leaf).data as *mut object_t;
        if !(cp == objp) {
            objp2rect(cp, &mut srect);
            a = aabbaabb(&mut rect, &mut srect);
            if a > 0.0f64 {
                ra = recordointrsx(objp, cp, &mut rect, a, intrsx);
                bp.n += 1;
                bp.area += ra;
            }
            if !(((*cp).lbl).is_null() || (*(*cp).lbl).set == 0) {
                objplp2rect(cp, &mut srect);
                a = aabbaabb(&mut rect, &mut srect);
                if a > 0.0f64 {
                    ra = recordlintrsx(objp, cp, &mut rect, a, intrsx);
                    bp.n += 1;
                    bp.area += ra;
                }
            }
        }
        ilp = (*ilp).next;
    }
    RTreeLeafListFree(llp);
    return bp;
}
unsafe extern "C" fn xladjust(
    mut xlp: *mut XLabels_t,
    mut objp: *mut object_t,
) -> BestPos_t {
    let mut lp: *mut xlabel_t = (*objp).lbl;
    let mut xincr: libc::c_double = (2 as libc::c_int as libc::c_double * (*lp).sz.x
        + (*objp).sz.x) / 8 as libc::c_int as libc::c_double;
    let mut yincr: libc::c_double = (2 as libc::c_int as libc::c_double * (*lp).sz.y
        + (*objp).sz.y) / 2 as libc::c_int as libc::c_double;
    let mut intrsx: [*mut object_t; 9] = [
        0 as *mut object_t,
        0 as *mut object_t,
        0 as *mut object_t,
        0 as *mut object_t,
        0 as *mut object_t,
        0 as *mut object_t,
        0 as *mut object_t,
        0 as *mut object_t,
        0 as *mut object_t,
    ];
    let mut bp: BestPos_t = BestPos_t {
        n: 0,
        area: 0.,
        pos: pointf { x: 0., y: 0. },
    };
    let mut nbp: BestPos_t = BestPos_t {
        n: 0,
        area: 0.,
        pos: pointf { x: 0., y: 0. },
    };
    if !((*objp).lbl).is_null() {} else {
        __assert_fail(
            b"objp->lbl\0" as *const u8 as *const libc::c_char,
            b"xlabels.c\0" as *const u8 as *const libc::c_char,
            410 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"BestPos_t xladjust(XLabels_t *, object_t *)\0"))
                .as_ptr(),
        );
    }
    (*lp).pos.x = (*objp).pos.x - (*lp).sz.x;
    (*lp).pos.y = (*objp).pos.y + (*objp).sz.y;
    bp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if bp.n == 0 as libc::c_int {
        return bp;
    }
    (*lp).pos.y = (*objp).pos.y;
    nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if nbp.n == 0 as libc::c_int {
        return nbp;
    }
    if nbp.area < bp.area {
        bp = nbp;
    }
    (*lp).pos.y = (*objp).pos.y - (*lp).sz.y;
    nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if nbp.n == 0 as libc::c_int {
        return nbp;
    }
    if nbp.area < bp.area {
        bp = nbp;
    }
    (*lp).pos.x = (*objp).pos.x;
    (*lp).pos.y = (*objp).pos.y + (*objp).sz.y;
    nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if nbp.n == 0 as libc::c_int {
        return nbp;
    }
    if nbp.area < bp.area {
        bp = nbp;
    }
    (*lp).pos.y = (*objp).pos.y - (*lp).sz.y;
    nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if nbp.n == 0 as libc::c_int {
        return nbp;
    }
    if nbp.area < bp.area {
        bp = nbp;
    }
    (*lp).pos.x = (*objp).pos.x + (*objp).sz.x;
    (*lp).pos.y = (*objp).pos.y + (*objp).sz.y;
    nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if nbp.n == 0 as libc::c_int {
        return nbp;
    }
    if nbp.area < bp.area {
        bp = nbp;
    }
    (*lp).pos.y = (*objp).pos.y;
    nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if nbp.n == 0 as libc::c_int {
        return nbp;
    }
    if nbp.area < bp.area {
        bp = nbp;
    }
    (*lp).pos.y = (*objp).pos.y - (*lp).sz.y;
    nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
    if nbp.n == 0 as libc::c_int {
        return nbp;
    }
    if nbp.area < bp.area {
        bp = nbp;
    }
    if !(intrsx[6 as libc::c_int as usize]).is_null()
        || !(intrsx[7 as libc::c_int as usize]).is_null()
        || !(intrsx[8 as libc::c_int as usize]).is_null()
        || !(intrsx[3 as libc::c_int as usize]).is_null()
        || !(intrsx[0 as libc::c_int as usize]).is_null()
    {
        if (intrsx[7 as libc::c_int as usize]).is_null()
            && (intrsx[8 as libc::c_int as usize]).is_null()
        {
            (*lp).pos.x = (*objp).pos.x - (*lp).sz.x;
            (*lp).pos.y = (*objp).pos.y + (*objp).sz.y;
            while (*lp).pos.x <= (*objp).pos.x + (*objp).sz.x {
                nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
                if nbp.n == 0 as libc::c_int {
                    return nbp;
                }
                if nbp.area < bp.area {
                    bp = nbp;
                }
                (*lp).pos.x += xincr;
            }
        }
        if (intrsx[3 as libc::c_int as usize]).is_null()
            && (intrsx[0 as libc::c_int as usize]).is_null()
        {
            (*lp).pos.x = (*objp).pos.x - (*lp).sz.x;
            (*lp).pos.y = (*objp).pos.y + (*objp).sz.y;
            while (*lp).pos.y >= (*objp).pos.y - (*lp).sz.y {
                nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
                if nbp.n == 0 as libc::c_int {
                    return nbp;
                }
                if nbp.area < bp.area {
                    bp = nbp;
                }
                (*lp).pos.y -= yincr;
            }
        }
    }
    (*lp).pos.x = (*objp).pos.x + (*objp).sz.x;
    (*lp).pos.y = (*objp).pos.y - (*lp).sz.y;
    if !(intrsx[2 as libc::c_int as usize]).is_null()
        || !(intrsx[1 as libc::c_int as usize]).is_null()
        || !(intrsx[0 as libc::c_int as usize]).is_null()
        || !(intrsx[5 as libc::c_int as usize]).is_null()
        || !(intrsx[8 as libc::c_int as usize]).is_null()
    {
        if (intrsx[1 as libc::c_int as usize]).is_null()
            && (intrsx[0 as libc::c_int as usize]).is_null()
        {
            (*lp).pos.x = (*objp).pos.x + (*objp).sz.x;
            (*lp).pos.y = (*objp).pos.y - (*lp).sz.y;
            while (*lp).pos.x >= (*objp).pos.x - (*lp).sz.x {
                nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
                if nbp.n == 0 as libc::c_int {
                    return nbp;
                }
                if nbp.area < bp.area {
                    bp = nbp;
                }
                (*lp).pos.x -= xincr;
            }
        }
        if (intrsx[5 as libc::c_int as usize]).is_null()
            && (intrsx[8 as libc::c_int as usize]).is_null()
        {
            (*lp).pos.x = (*objp).pos.x + (*objp).sz.x;
            (*lp).pos.y = (*objp).pos.y - (*lp).sz.y;
            while (*lp).pos.y <= (*objp).pos.y + (*objp).sz.y {
                nbp = xlintersections(xlp, objp, intrsx.as_mut_ptr());
                if nbp.n == 0 as libc::c_int {
                    return nbp;
                }
                if nbp.area < bp.area {
                    bp = nbp;
                }
                (*lp).pos.y += yincr;
            }
        }
    }
    return bp;
}
unsafe extern "C" fn xlhdxload(mut xlp: *mut XLabels_t) -> libc::c_int {
    let mut order: libc::c_int = xlhorder(xlp) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*xlp).n_objs {
        let mut hp: *mut HDict_t = 0 as *mut HDict_t;
        let mut pi: point = point { x: 0, y: 0 };
        hp = zmalloc(::std::mem::size_of::<HDict_t>() as libc::c_ulong) as *mut HDict_t;
        let ref mut fresh9 = (*hp).d.data;
        *fresh9 = &mut *((*xlp).objs).offset(i as isize) as *mut object_t
            as *mut libc::c_void;
        (*hp).d.rect = objplpmks(&mut *((*xlp).objs).offset(i as isize));
        pi
            .x = (*hp).d.rect.boundary[0 as libc::c_int as usize]
            + ((*hp).d.rect.boundary[2 as libc::c_int as usize]
                - (*hp).d.rect.boundary[0 as libc::c_int as usize]) / 2 as libc::c_int;
        pi
            .y = (*hp).d.rect.boundary[1 as libc::c_int as usize]
            + ((*hp).d.rect.boundary[3 as libc::c_int as usize]
                - (*hp).d.rect.boundary[1 as libc::c_int as usize]) / 2 as libc::c_int;
        (*hp).key = hd_hil_s_from_xy(pi, order) as libc::c_int;
        if ((Some(((*(*xlp).hdx).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*xlp).hdx, hp as *mut libc::c_void, 0o1 as libc::c_int))
            .is_null()
        {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xlhdxunload(mut xlp: *mut XLabels_t) {
    let mut size: libc::c_int = dtsize((*xlp).hdx);
    let mut freed: libc::c_int = 0 as libc::c_int;
    while dtsize((*xlp).hdx) != 0 {
        let mut vp: *mut libc::c_void = if !((*(*(*xlp).hdx).data).here).is_null() {
            if (*(*(*xlp).hdx).disc).link < 0 as libc::c_int {
                (*((*(*(*xlp).hdx).data).here as *mut Dthold_t)).obj
            } else {
                ((*(*(*xlp).hdx).data).here as *mut libc::c_char)
                    .offset(-((*(*(*xlp).hdx).disc).link as isize)) as *mut libc::c_void
            }
        } else {
            0 as *mut libc::c_void
        };
        if !vp.is_null() {} else {
            __assert_fail(
                b"vp\0" as *const u8 as *const libc::c_char,
                b"xlabels.c\0" as *const u8 as *const libc::c_char,
                572 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void xlhdxunload(XLabels_t *)\0"))
                    .as_ptr(),
            );
        }
        if !vp.is_null() {
            (Some(((*(*xlp).hdx).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*xlp).hdx, vp, 0o10000 as libc::c_int);
            free(vp);
            freed += 1;
        }
    }
    if size == freed {} else {
        __assert_fail(
            b"size==freed\0" as *const u8 as *const libc::c_char,
            b"xlabels.c\0" as *const u8 as *const libc::c_char,
            579 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void xlhdxunload(XLabels_t *)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn xlspdxload(mut xlp: *mut XLabels_t) -> libc::c_int {
    let mut op: *mut HDict_t = (Some(
        ((*(*xlp).hdx).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )((*xlp).hdx, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut HDict_t;
    while !op.is_null() {
        RTreeInsert(
            (*xlp).spdx,
            &mut (*op).d.rect,
            (*op).d.data,
            &mut (*(*xlp).spdx).root,
            0 as libc::c_int,
        );
        op = (Some(((*(*xlp).hdx).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*xlp).hdx, op as *mut libc::c_void, 0o10 as libc::c_int) as *mut HDict_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xlinitialize(mut xlp: *mut XLabels_t) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    r = xlhdxload(xlp);
    if r < 0 as libc::c_int {
        return r;
    }
    r = xlspdxload(xlp);
    if r < 0 as libc::c_int {
        return r;
    }
    xlhdxunload(xlp);
    return dtclose((*xlp).hdx);
}
#[no_mangle]
pub unsafe extern "C" fn placeLabels(
    mut objs: *mut object_t,
    mut n_objs: libc::c_int,
    mut lbls: *mut xlabel_t,
    mut n_lbls: libc::c_int,
    mut params: *mut label_params_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut bp: BestPos_t = BestPos_t {
        n: 0,
        area: 0.,
        pos: pointf { x: 0., y: 0. },
    };
    let mut xlp: *mut XLabels_t = xlnew(objs, n_objs, lbls, n_lbls, params);
    r = xlinitialize(xlp);
    if r < 0 as libc::c_int {
        return r;
    }
    r = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_objs {
        if !((*objs.offset(i as isize)).lbl).is_null() {
            bp = xladjust(xlp, &mut *objs.offset(i as isize));
            if bp.n == 0 as libc::c_int {
                (*(*objs.offset(i as isize)).lbl)
                    .set = 1 as libc::c_int as libc::c_uchar;
            } else if bp.area == 0 as libc::c_int as libc::c_double {
                (*(*objs.offset(i as isize)).lbl).pos.x = bp.pos.x;
                (*(*objs.offset(i as isize)).lbl).pos.y = bp.pos.y;
                (*(*objs.offset(i as isize)).lbl)
                    .set = 1 as libc::c_int as libc::c_uchar;
            } else if (*params).force as libc::c_int == 1 as libc::c_int {
                (*(*objs.offset(i as isize)).lbl).pos.x = bp.pos.x;
                (*(*objs.offset(i as isize)).lbl).pos.y = bp.pos.y;
                (*(*objs.offset(i as isize)).lbl)
                    .set = 1 as libc::c_int as libc::c_uchar;
            } else {
                r = 1 as libc::c_int;
            }
        }
        i += 1;
    }
    xlfree(xlp);
    return r;
}
