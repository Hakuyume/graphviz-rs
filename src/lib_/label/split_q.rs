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
    fn InitNode(_: *mut Node_t);
    fn AddBranch(
        _: *mut RTree_t,
        _: *mut Branch_t,
        _: *mut Node_t,
        _: *mut *mut Node_t,
    ) -> libc::c_int;
    fn RectArea(_: *mut Rect_t) -> libc::c_uint;
    fn CombineRect(_: *mut Rect_t, _: *mut Rect_t) -> Rect_t;
    fn NullRect() -> Rect_t;
    fn RTreeNewNode() -> *mut Node_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
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
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn SplitNode(
    mut rtp: *mut RTree_t,
    mut n: *mut Node_t,
    mut b: *mut Branch_t,
    mut nn: *mut *mut Node_t,
) {
    if !n.is_null() {
    } else {
        __assert_fail(
            b"n\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void SplitNode(RTree_t *, Node_t *, Branch_t *, Node_t **)\0",
            ))
            .as_ptr(),
        );
    }
    if !b.is_null() {
    } else {
        __assert_fail(
            b"b\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void SplitNode(RTree_t *, Node_t *, Branch_t *, Node_t **)\0",
            ))
            .as_ptr(),
        );
    }
    let mut level: libc::c_int = (*n).level;
    GetBranches(rtp, n, b);
    let mut p: *mut PartitionVars = &mut *((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        as *mut PartitionVars;
    MethodZero(rtp);
    *nn = RTreeNewNode();
    let ref mut fresh0 = (*n).level;
    *fresh0 = level;
    (**nn).level = *fresh0;
    LoadNodes(rtp, n, *nn, p);
    if (*n).count + (**nn).count == 64 as libc::c_int + 1 as libc::c_int {
    } else {
        __assert_fail(
            b"n->count + (*nn)->count == NODECARD + 1\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void SplitNode(RTree_t *, Node_t *, Branch_t *, Node_t **)\0",
            ))
            .as_ptr(),
        );
    };
}
unsafe extern "C" fn GetBranches(mut rtp: *mut RTree_t, mut n: *mut Node_t, mut b: *mut Branch_t) {
    if !n.is_null() {
    } else {
        __assert_fail(
            b"n\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void GetBranches(RTree_t *, Node_t *, Branch_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !b.is_null() {
    } else {
        __assert_fail(
            b"b\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void GetBranches(RTree_t *, Node_t *, Branch_t *)\0",
            ))
            .as_ptr(),
        );
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 64 as libc::c_int as libc::c_ulong {
        if !((*n).branch[i as usize].child).is_null() {
        } else {
            __assert_fail(
                b"n->branch[i].child\0" as *const u8 as *const libc::c_char,
                b"split.q.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"void GetBranches(RTree_t *, Node_t *, Branch_t *)\0",
                ))
                .as_ptr(),
            );
        }
        (*rtp).split.BranchBuf[i as usize] = (*n).branch[i as usize];
        i = i.wrapping_add(1);
    }
    (*rtp).split.BranchBuf[64 as libc::c_int as usize] = *b;
    (*rtp).split.CoverSplit = (*rtp).split.BranchBuf[0 as libc::c_int as usize].rect;
    let mut i_0: size_t = 1 as libc::c_int as size_t;
    while i_0 < (64 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        (*rtp).split.CoverSplit = CombineRect(
            &mut (*rtp).split.CoverSplit,
            &mut (*((*rtp).split.BranchBuf).as_mut_ptr().offset(i_0 as isize)).rect,
        );
        i_0 = i_0.wrapping_add(1);
    }
    (*rtp).split.CoverSplitArea = RectArea(&mut (*rtp).split.CoverSplit);
    InitNode(n);
}
unsafe extern "C" fn MethodZero(mut rtp: *mut RTree_t) {
    let mut r: *mut Rect_t = 0 as *mut Rect_t;
    let mut growth0: libc::c_int = 0;
    let mut growth1: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut biggestDiff: libc::c_int = 0;
    let mut group: libc::c_int = 0;
    let mut chosen: libc::c_int = 0 as libc::c_int;
    let mut betterGroup: libc::c_int = 0 as libc::c_int;
    InitPVars(rtp);
    PickSeeds(rtp);
    while (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[0 as libc::c_int as usize]
        + (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .count[1 as libc::c_int as usize]
        < 64 as libc::c_int + 1 as libc::c_int
        && (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .count[0 as libc::c_int as usize]
            < 64 as libc::c_int + 1 as libc::c_int - (*rtp).MinFill
        && (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .count[1 as libc::c_int as usize]
            < 64 as libc::c_int + 1 as libc::c_int - (*rtp).MinFill
    {
        biggestDiff = -(1 as libc::c_int);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 64 as libc::c_int + 1 as libc::c_int {
            if (*((*rtp).split.Partitions)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .taken[i as usize]
                == 0
            {
                let mut rect: Rect_t = Rect_t { boundary: [0; 4] };
                r = &mut (*((*rtp).split.BranchBuf).as_mut_ptr().offset(i as isize)).rect;
                rect = CombineRect(
                    r,
                    &mut *((*((*rtp).split.Partitions)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .cover)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize),
                );
                growth0 = (RectArea(&mut rect)).wrapping_sub(
                    (*((*rtp).split.Partitions)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .area[0 as libc::c_int as usize] as libc::c_uint,
                ) as libc::c_int;
                rect = CombineRect(
                    r,
                    &mut *((*((*rtp).split.Partitions)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .cover)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize),
                );
                growth1 = (RectArea(&mut rect)).wrapping_sub(
                    (*((*rtp).split.Partitions)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .area[1 as libc::c_int as usize] as libc::c_uint,
                ) as libc::c_int;
                diff = growth1 - growth0;
                if diff >= 0 as libc::c_int {
                    group = 0 as libc::c_int;
                } else {
                    group = 1 as libc::c_int;
                    diff = -diff;
                }
                if diff > biggestDiff {
                    biggestDiff = diff;
                    chosen = i;
                    betterGroup = group;
                } else if diff == biggestDiff
                    && (*((*rtp).split.Partitions)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .count[group as usize]
                        < (*((*rtp).split.Partitions)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .count[betterGroup as usize]
                {
                    chosen = i;
                    betterGroup = group;
                }
            }
            i += 1;
        }
        Classify(rtp, chosen, betterGroup);
    }
    if (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[0 as libc::c_int as usize]
        + (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .count[1 as libc::c_int as usize]
        < 64 as libc::c_int + 1 as libc::c_int
    {
        group = 0 as libc::c_int;
        if (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .count[0 as libc::c_int as usize]
            >= 64 as libc::c_int + 1 as libc::c_int - (*rtp).MinFill
        {
            group = 1 as libc::c_int;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 64 as libc::c_int + 1 as libc::c_int {
            if (*((*rtp).split.Partitions)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .taken[i_0 as usize]
                == 0
            {
                Classify(rtp, i_0, group);
            }
            i_0 += 1;
        }
    }
    if (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[0 as libc::c_int as usize]
        + (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .count[1 as libc::c_int as usize]
        == 64 as libc::c_int + 1 as libc::c_int
    {
    } else {
        __assert_fail(
            b"rtp->split.Partitions[0].count[0] + rtp->split.Partitions[0].count[1] == NODECARD + 1\0"
                as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void MethodZero(RTree_t *)\0"))
                .as_ptr(),
        );
    }
    if (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[0 as libc::c_int as usize]
        >= (*rtp).MinFill
        && (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .count[1 as libc::c_int as usize]
            >= (*rtp).MinFill
    {
    } else {
        __assert_fail(
            b"rtp->split.Partitions[0].count[0] >= rtp->MinFill && rtp->split.Partitions[0].count[1] >= rtp->MinFill\0"
                as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void MethodZero(RTree_t *)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn PickSeeds(mut rtp: *mut RTree_t) {
    let mut seed0: libc::c_int = 0 as libc::c_int;
    let mut seed1: libc::c_int = 0 as libc::c_int;
    let mut area: [libc::c_uint; 65] = [0; 65];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 64 as libc::c_int + 1 as libc::c_int {
        area[i as usize] =
            RectArea(&mut (*((*rtp).split.BranchBuf).as_mut_ptr().offset(i as isize)).rect);
        i += 1;
    }
    let mut worst: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 64 as libc::c_int {
        let mut j: libc::c_int = i_0 + 1 as libc::c_int;
        while j < 64 as libc::c_int + 1 as libc::c_int {
            let mut rect: Rect_t = Rect_t { boundary: [0; 4] };
            rect = CombineRect(
                &mut (*((*rtp).split.BranchBuf).as_mut_ptr().offset(i_0 as isize)).rect,
                &mut (*((*rtp).split.BranchBuf).as_mut_ptr().offset(j as isize)).rect,
            );
            let mut waste: libc::c_uint = (RectArea(&mut rect))
                .wrapping_sub(area[i_0 as usize])
                .wrapping_sub(area[j as usize]);
            if waste > worst {
                worst = waste;
                seed0 = i_0;
                seed1 = j;
            }
            j += 1;
        }
        i_0 += 1;
    }
    Classify(rtp, seed0, 0 as libc::c_int);
    Classify(rtp, seed1, 1 as libc::c_int);
}
unsafe extern "C" fn Classify(mut rtp: *mut RTree_t, mut i: libc::c_int, mut group: libc::c_int) {
    if (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .taken[i as usize]
        == 0
    {
    } else {
        __assert_fail(
            b"!rtp->split.Partitions[0].taken[i]\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"void Classify(RTree_t *, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .partition[i as usize] = group;
    (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .taken[i as usize] = (0 as libc::c_int == 0) as libc::c_int;
    if (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[group as usize]
        == 0 as libc::c_int
    {
        (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .cover[group as usize] = (*rtp).split.BranchBuf[i as usize].rect;
    } else {
        (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .cover[group as usize] = CombineRect(
            &mut (*((*rtp).split.BranchBuf).as_mut_ptr().offset(i as isize)).rect,
            &mut *((*((*rtp).split.Partitions)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .cover)
                .as_mut_ptr()
                .offset(group as isize),
        );
    }
    (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .area[group as usize] = RectArea(
        &mut *((*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .cover)
            .as_mut_ptr()
            .offset(group as isize),
    ) as libc::c_int;
    let ref mut fresh1 = (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[group as usize];
    *fresh1 += 1;
}
unsafe extern "C" fn LoadNodes(
    mut rtp: *mut RTree_t,
    mut n: *mut Node_t,
    mut q: *mut Node_t,
    mut p: *mut PartitionVars,
) {
    if !n.is_null() {
    } else {
        __assert_fail(
            b"n\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 70], &[libc::c_char; 70]>(
                b"void LoadNodes(RTree_t *, Node_t *, Node_t *, struct PartitionVars *)\0",
            ))
            .as_ptr(),
        );
    }
    if !q.is_null() {
    } else {
        __assert_fail(
            b"q\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 70], &[libc::c_char; 70]>(
                b"void LoadNodes(RTree_t *, Node_t *, Node_t *, struct PartitionVars *)\0",
            ))
            .as_ptr(),
        );
    }
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\0" as *const u8 as *const libc::c_char,
            b"split.q.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 70], &[libc::c_char; 70]>(
                b"void LoadNodes(RTree_t *, Node_t *, Node_t *, struct PartitionVars *)\0",
            ))
            .as_ptr(),
        );
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (64 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        if (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .partition[i as usize]
            == 0 as libc::c_int
            || (*((*rtp).split.Partitions)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .partition[i as usize]
                == 1 as libc::c_int
        {
        } else {
            __assert_fail(
                b"rtp->split.Partitions[0].partition[i] == 0 || rtp->split.Partitions[0].partition[i] == 1\0"
                    as *const u8 as *const libc::c_char,
                b"split.q.c\0" as *const u8 as *const libc::c_char,
                267 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"void LoadNodes(RTree_t *, Node_t *, Node_t *, struct PartitionVars *)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .partition[i as usize]
            == 0 as libc::c_int
        {
            AddBranch(
                rtp,
                &mut *((*rtp).split.BranchBuf).as_mut_ptr().offset(i as isize),
                n,
                0 as *mut *mut Node_t,
            );
        } else if (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .partition[i as usize]
            == 1 as libc::c_int
        {
            AddBranch(
                rtp,
                &mut *((*rtp).split.BranchBuf).as_mut_ptr().offset(i as isize),
                q,
                0 as *mut *mut Node_t,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn InitPVars(mut rtp: *mut RTree_t) {
    let ref mut fresh2 = (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[1 as libc::c_int as usize];
    *fresh2 = 0 as libc::c_int;
    (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .count[0 as libc::c_int as usize] = *fresh2;
    let ref mut fresh3 = (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .cover[1 as libc::c_int as usize];
    *fresh3 = NullRect();
    (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .cover[0 as libc::c_int as usize] = *fresh3;
    let ref mut fresh4 = (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .area[1 as libc::c_int as usize];
    *fresh4 = 0 as libc::c_int;
    (*((*rtp).split.Partitions)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .area[0 as libc::c_int as usize] = *fresh4;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (64 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .taken[i as usize] = 0 as libc::c_int;
        (*((*rtp).split.Partitions)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .partition[i as usize] = -(1 as libc::c_int);
        i = i.wrapping_add(1);
    }
}
