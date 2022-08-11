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
#![feature(label_break_value, register_tool)]
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn Overlap(_: *mut Rect_t, _: *mut Rect_t) -> libc::c_int;
    fn NodeCover(_: *mut Node_t) -> Rect_t;
    fn PickBranch(_: *mut Rect_t, _: *mut Node_t) -> libc::c_int;
    fn AddBranch(
        _: *mut RTree_t,
        _: *mut Branch_t,
        _: *mut Node_t,
        _: *mut *mut Node_t,
    ) -> libc::c_int;
    fn DisconBranch(_: *mut Node_t, _: libc::c_int);
    fn RTreeNewNode() -> *mut Node_t;
    fn CombineRect(_: *mut Rect_t, _: *mut Rect_t) -> Rect_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
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
pub type RTree_t = RTree;
pub type Branch_t = Branch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Leaf {
    pub rect: Rect_t,
    pub data: *mut libc::c_void,
}
pub type Leaf_t = Leaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LeafList {
    pub next: *mut LeafList,
    pub leaf: *mut Leaf_t,
}
pub type LeafList_t = LeafList;
#[no_mangle]
pub unsafe extern "C" fn RTreeNewLeafList(mut lp: *mut Leaf_t) -> *mut LeafList_t {
    let mut llp: *mut LeafList_t = 0 as *mut LeafList_t;
    llp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<LeafList_t>() as libc::c_ulong,
    ) as *mut LeafList_t;
    if !llp.is_null() {
        let ref mut fresh0 = (*llp).leaf;
        *fresh0 = lp;
        let ref mut fresh1 = (*llp).next;
        *fresh1 = 0 as *mut LeafList;
    }
    return llp;
}
#[no_mangle]
pub unsafe extern "C" fn RTreeLeafListAdd(
    mut llp: *mut LeafList_t,
    mut lp: *mut Leaf_t,
) -> *mut LeafList_t {
    if lp.is_null() {
        return llp;
    }
    let mut nlp: *mut LeafList_t = RTreeNewLeafList(lp);
    let ref mut fresh2 = (*nlp).next;
    *fresh2 = llp;
    return nlp;
}
#[no_mangle]
pub unsafe extern "C" fn RTreeLeafListFree(mut llp: *mut LeafList_t) {
    while !((*llp).next).is_null() {
        let mut tlp: *mut LeafList_t = (*llp).next;
        free(llp as *mut libc::c_void);
        llp = tlp;
    }
    free(llp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RTreeOpen() -> *mut RTree_t {
    let mut rtp: *mut RTree_t = 0 as *mut RTree_t;
    rtp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<RTree_t>() as libc::c_ulong,
    ) as *mut RTree_t;
    if !rtp.is_null() {
        let ref mut fresh3 = (*rtp).root;
        *fresh3 = RTreeNewIndex();
    }
    return rtp;
}
#[no_mangle]
pub unsafe extern "C" fn RTreeNewIndex() -> *mut Node_t {
    let mut x: *mut Node_t = RTreeNewNode();
    (*x).level = 0 as libc::c_int;
    return x;
}
unsafe extern "C" fn RTreeClose2(mut rtp: *mut RTree_t, mut n: *mut Node_t) -> libc::c_int {
    if (*n).level > 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if !((*n).branch[i as usize].child).is_null() {
                if RTreeClose2(rtp, (*n).branch[i as usize].child) == 0 {
                    free((*n).branch[i as usize].child as *mut libc::c_void);
                    DisconBranch(n, i);
                }
            }
            i += 1;
        }
    } else {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 64 as libc::c_int {
            if !((*n).branch[i_0 as usize].child).is_null() {
                DisconBranch(n, i_0);
            }
            i_0 += 1;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RTreeClose(mut rtp: *mut RTree_t) -> libc::c_int {
    RTreeClose2(rtp, (*rtp).root);
    free((*rtp).root as *mut libc::c_void);
    free(rtp as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RTreeSearch(
    mut rtp: *mut RTree_t,
    mut n: *mut Node_t,
    mut r: *mut Rect_t,
) -> *mut LeafList_t {
    let mut llp: *mut LeafList_t = 0 as *mut LeafList_t;
    if !n.is_null() {
    } else {
        __assert_fail(
            b"n\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"LeafList_t *RTreeSearch(RTree_t *, Node_t *, Rect_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*n).level >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"n->level >= 0\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"LeafList_t *RTreeSearch(RTree_t *, Node_t *, Rect_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !r.is_null() {
    } else {
        __assert_fail(
            b"r\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"LeafList_t *RTreeSearch(RTree_t *, Node_t *, Rect_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*n).level > 0 as libc::c_int {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 64 as libc::c_int as libc::c_ulong {
            if !((*n).branch[i as usize].child).is_null()
                && Overlap(
                    r,
                    &mut (*((*n).branch).as_mut_ptr().offset(i as isize)).rect,
                ) != 0
            {
                let mut tlp: *mut LeafList_t = RTreeSearch(rtp, (*n).branch[i as usize].child, r);
                if !llp.is_null() {
                    let mut xlp: *mut LeafList_t = llp;
                    while !((*xlp).next).is_null() {
                        xlp = (*xlp).next;
                    }
                    let ref mut fresh4 = (*xlp).next;
                    *fresh4 = tlp;
                } else {
                    llp = tlp;
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        while i_0 < 64 as libc::c_int as libc::c_ulong {
            if !((*n).branch[i_0 as usize].child).is_null()
                && Overlap(
                    r,
                    &mut (*((*n).branch).as_mut_ptr().offset(i_0 as isize)).rect,
                ) != 0
            {
                llp = RTreeLeafListAdd(
                    llp,
                    &mut *((*n).branch).as_mut_ptr().offset(i_0 as isize) as *mut Branch
                        as *mut Leaf_t,
                );
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    return llp;
}
#[no_mangle]
pub unsafe extern "C" fn RTreeInsert(
    mut rtp: *mut RTree_t,
    mut r: *mut Rect_t,
    mut data: *mut libc::c_void,
    mut n: *mut *mut Node_t,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut newnode: *mut Node_t = 0 as *mut Node_t;
    let mut b: Branch_t = Branch_t {
        rect: Rect_t { boundary: [0; 4] },
        child: 0 as *mut Node,
    };
    let mut result: libc::c_int = 0 as libc::c_int;
    if !r.is_null() && !n.is_null() {
    } else {
        __assert_fail(
            b"r && n\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"int RTreeInsert(RTree_t *, Rect_t *, void *, Node_t **, int)\0",
            ))
            .as_ptr(),
        );
    }
    if level >= 0 as libc::c_int && level <= (**n).level {
    } else {
        __assert_fail(
            b"level >= 0 && level <= (*n)->level\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"int RTreeInsert(RTree_t *, Rect_t *, void *, Node_t **, int)\0",
            ))
            .as_ptr(),
        );
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        if (*r).boundary[i as usize]
            <= (*r).boundary[(2 as libc::c_int as libc::c_ulong).wrapping_add(i) as usize]
        {
        } else {
            __assert_fail(
                b"r->boundary[i] <= r->boundary[NUMDIMS + i]\0" as *const u8 as *const libc::c_char,
                b"index.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                    b"int RTreeInsert(RTree_t *, Rect_t *, void *, Node_t **, int)\0",
                ))
                .as_ptr(),
            );
        }
        i = i.wrapping_add(1);
    }
    if RTreeInsert2(rtp, r, data, *n, &mut newnode, level) != 0 {
        let mut newroot: *mut Node_t = RTreeNewNode();
        (*newroot).level = (**n).level + 1 as libc::c_int;
        b.rect = NodeCover(*n);
        b.child = *n;
        AddBranch(rtp, &mut b, newroot, 0 as *mut *mut Node_t);
        b.rect = NodeCover(newnode);
        b.child = newnode;
        AddBranch(rtp, &mut b, newroot, 0 as *mut *mut Node_t);
        *n = newroot;
        result = 1 as libc::c_int;
    }
    return result;
}
unsafe extern "C" fn RTreeInsert2(
    mut rtp: *mut RTree_t,
    mut r: *mut Rect_t,
    mut data: *mut libc::c_void,
    mut n: *mut Node_t,
    mut new: *mut *mut Node_t,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut b: Branch_t = Branch_t {
        rect: Rect_t { boundary: [0; 4] },
        child: 0 as *mut Node,
    };
    let mut n2: *mut Node_t = 0 as *mut Node_t;
    if !r.is_null() && !n.is_null() && !new.is_null() {
    } else {
        __assert_fail(
            b"r && n && new\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 72], &[libc::c_char; 72]>(
                b"int RTreeInsert2(RTree_t *, Rect_t *, void *, Node_t *, Node_t **, int)\0",
            ))
            .as_ptr(),
        );
    }
    if level >= 0 as libc::c_int && level <= (*n).level {
    } else {
        __assert_fail(
            b"level >= 0 && level <= n->level\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 72], &[libc::c_char; 72]>(
                b"int RTreeInsert2(RTree_t *, Rect_t *, void *, Node_t *, Node_t **, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*n).level > level {
        let mut i: libc::c_int = PickBranch(r, n);
        if RTreeInsert2(rtp, r, data, (*n).branch[i as usize].child, &mut n2, level) == 0 {
            (*n).branch[i as usize].rect = CombineRect(
                r,
                &mut (*((*n).branch).as_mut_ptr().offset(i as isize)).rect,
            );
            return 0 as libc::c_int;
        } else {
            (*n).branch[i as usize].rect = NodeCover((*n).branch[i as usize].child);
            b.child = n2;
            b.rect = NodeCover(n2);
            return AddBranch(rtp, &mut b, n, new);
        }
    } else if (*n).level == level {
        b.rect = *r;
        b.child = data as *mut Node_t;
        return AddBranch(rtp, &mut b, n, new);
    } else {
        __assert_fail(
            b"FALSE\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 72], &[libc::c_char; 72]>(
                b"int RTreeInsert2(RTree_t *, Rect_t *, void *, Node_t *, Node_t **, int)\0",
            ))
            .as_ptr(),
        );
        return 0 as libc::c_int;
    };
}
