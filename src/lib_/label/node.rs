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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn InitRect(r: *mut Rect_t);
    fn RectArea(_: *mut Rect_t) -> libc::c_uint;
    fn CombineRect(_: *mut Rect_t, _: *mut Rect_t) -> Rect_t;
    fn SplitNode(_: *mut RTree_t, _: *mut Node_t, _: *mut Branch_t, _: *mut *mut Node_t);
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
#[no_mangle]
pub unsafe extern "C" fn RTreeNewNode() -> *mut Node_t {
    let mut n: *mut Node_t =
        malloc(::std::mem::size_of::<Node_t>() as libc::c_ulong) as *mut Node_t;
    InitNode(n);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn InitNode(mut n: *mut Node_t) {
    (*n).count = 0 as libc::c_int;
    (*n).level = -(1 as libc::c_int);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 64 as libc::c_int as libc::c_ulong {
        InitBranch(&mut *((*n).branch).as_mut_ptr().offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitBranch(mut b: *mut Branch_t) {
    InitRect(&mut (*b).rect);
    let ref mut fresh0 = (*b).child;
    *fresh0 = 0 as *mut Node;
}
#[no_mangle]
pub unsafe extern "C" fn NodeCover(mut n: *mut Node_t) -> Rect_t {
    let mut r: Rect_t = Rect_t { boundary: [0; 4] };
    if !n.is_null() {
    } else {
        __assert_fail(
            b"n\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"Rect_t NodeCover(Node_t *)\0",
            ))
            .as_ptr(),
        );
    }
    InitRect(&mut r);
    let mut flag: bool = 1 as libc::c_int != 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 64 as libc::c_int as libc::c_ulong {
        if !((*n).branch[i as usize].child).is_null() {
            if flag {
                r = (*n).branch[i as usize].rect;
                flag = 0 as libc::c_int != 0;
            } else {
                r = CombineRect(
                    &mut r,
                    &mut (*((*n).branch).as_mut_ptr().offset(i as isize)).rect,
                );
            }
        }
        i = i.wrapping_add(1);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn PickBranch(mut r: *mut Rect_t, mut n: *mut Node_t) -> libc::c_int {
    let mut rr: *mut Rect_t = 0 as *mut Rect_t;
    let mut flag: libc::c_int = 1 as libc::c_int;
    let mut increase: libc::c_int = 0 as libc::c_int;
    let mut bestIncr: libc::c_int = 0 as libc::c_int;
    let mut area: libc::c_int = 0 as libc::c_int;
    let mut bestArea: libc::c_int = 0 as libc::c_int;
    let mut best: libc::c_int = 0 as libc::c_int;
    if !r.is_null() && !n.is_null() {
    } else {
        __assert_fail(
            b"r && n\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"int PickBranch(Rect_t *, Node_t *)\0",
            ))
            .as_ptr(),
        );
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !((*n).branch[i as usize].child).is_null() {
            let mut rect: Rect_t = Rect_t { boundary: [0; 4] };
            rr = &mut (*((*n).branch).as_mut_ptr().offset(i as isize)).rect;
            area = RectArea(rr) as libc::c_int;
            rect = CombineRect(r, rr);
            increase = (RectArea(&mut rect)).wrapping_sub(area as libc::c_uint) as libc::c_int;
            if increase < bestIncr || flag != 0 {
                best = i;
                bestArea = area;
                bestIncr = increase;
                flag = 0 as libc::c_int;
            } else if increase == bestIncr && area < bestArea {
                best = i;
                bestArea = area;
                bestIncr = increase;
            }
        }
        i += 1;
    }
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn AddBranch(
    mut rtp: *mut RTree_t,
    mut b: *mut Branch_t,
    mut n: *mut Node_t,
    mut new: *mut *mut Node_t,
) -> libc::c_int {
    if !b.is_null() {
    } else {
        __assert_fail(
            b"b\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"int AddBranch(RTree_t *, Branch_t *, Node_t *, Node_t **)\0",
            ))
            .as_ptr(),
        );
    }
    if !n.is_null() {
    } else {
        __assert_fail(
            b"n\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"int AddBranch(RTree_t *, Branch_t *, Node_t *, Node_t **)\0",
            ))
            .as_ptr(),
        );
    }
    if (*n).count < 64 as libc::c_int {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < 64 as libc::c_int as libc::c_ulong {
            if ((*n).branch[i as usize].child).is_null() {
                (*n).branch[i as usize] = *b;
                let ref mut fresh1 = (*n).count;
                *fresh1 += 1;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i < 64 as libc::c_int as libc::c_ulong {
        } else {
            __assert_fail(
                b"i < NODECARD\0" as *const u8 as *const libc::c_char,
                b"node.c\0" as *const u8 as *const libc::c_char,
                160 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"int AddBranch(RTree_t *, Branch_t *, Node_t *, Node_t **)\0",
                ))
                .as_ptr(),
            );
        }
        return 0 as libc::c_int;
    } else {
        if !new.is_null() {
        } else {
            __assert_fail(
                b"new\0" as *const u8 as *const libc::c_char,
                b"node.c\0" as *const u8 as *const libc::c_char,
                163 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"int AddBranch(RTree_t *, Branch_t *, Node_t *, Node_t **)\0",
                ))
                .as_ptr(),
            );
        }
        SplitNode(rtp, n, b, new);
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DisconBranch(mut n: *mut Node_t, mut i: libc::c_int) {
    if !n.is_null() && i >= 0 as libc::c_int && i < 64 as libc::c_int {
    } else {
        __assert_fail(
            b"n && i >= 0 && i < NODECARD\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"void DisconBranch(Node_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*n).branch[i as usize].child).is_null() {
    } else {
        __assert_fail(
            b"n->branch[i].child\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"void DisconBranch(Node_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    InitBranch(&mut *((*n).branch).as_mut_ptr().offset(i as isize));
    let ref mut fresh2 = (*n).count;
    *fresh2 -= 1;
}
