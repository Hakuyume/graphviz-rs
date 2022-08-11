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
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
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
pub struct qnode_t {
    pub nodetype: libc::c_int,
    pub segnum: libc::c_int,
    pub yval: pointf,
    pub trnum: libc::c_int,
    pub parent: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
}
static mut q_idx: libc::c_int = 0;
static mut tr_idx: libc::c_int = 0;
static mut QSIZE: libc::c_int = 0;
static mut TRSIZE: libc::c_int = 0;
unsafe extern "C" fn newnode() -> libc::c_int {
    if q_idx < QSIZE {
        let fresh0 = q_idx;
        q_idx = q_idx + 1;
        return fresh0;
    } else {
        fprintf(
            stderr,
            b"newnode: Query-table overflow\n\0" as *const u8 as *const libc::c_char,
        );
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"trapezoid.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"int newnode(void)\0"))
                .as_ptr(),
        );
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn newtrap(mut tr: *mut trap_t) -> libc::c_int {
    if tr_idx < TRSIZE {
        (*tr.offset(tr_idx as isize)).lseg = -(1 as libc::c_int);
        (*tr.offset(tr_idx as isize)).rseg = -(1 as libc::c_int);
        (*tr.offset(tr_idx as isize)).state = 1 as libc::c_int;
        let fresh1 = tr_idx;
        tr_idx = tr_idx + 1;
        return fresh1;
    } else {
        fprintf(
            stderr,
            b"newtrap: Trapezoid-table overflow %d\n\0" as *const u8 as *const libc::c_char,
            tr_idx,
        );
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"trapezoid.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"int newtrap(trap_t *)\0"))
                .as_ptr(),
        );
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn _max(mut yval: *mut pointf, mut v0: *mut pointf, mut v1: *mut pointf) {
    if (*v0).y > (*v1).y + 1.0e-7f64 {
        *yval = *v0;
    } else if fabs((*v0).y - (*v1).y) <= 1.0e-7f64 {
        if (*v0).x > (*v1).x + 1.0e-7f64 {
            *yval = *v0;
        } else {
            *yval = *v1;
        }
    } else {
        *yval = *v1;
    };
}
unsafe extern "C" fn _min(mut yval: *mut pointf, mut v0: *mut pointf, mut v1: *mut pointf) {
    if (*v0).y < (*v1).y - 1.0e-7f64 {
        *yval = *v0;
    } else if fabs((*v0).y - (*v1).y) <= 1.0e-7f64 {
        if (*v0).x < (*v1).x {
            *yval = *v0;
        } else {
            *yval = *v1;
        }
    } else {
        *yval = *v1;
    };
}
unsafe extern "C" fn _greater_than_equal_to(mut v0: *mut pointf, mut v1: *mut pointf) -> bool {
    if (*v0).y > (*v1).y + 1.0e-7f64 {
        return 0 as libc::c_int == 0;
    } else if (*v0).y < (*v1).y - 1.0e-7f64 {
        return 0 as libc::c_int != 0;
    } else {
        return (*v0).x >= (*v1).x;
    };
}
unsafe extern "C" fn _less_than(mut v0: *mut pointf, mut v1: *mut pointf) -> bool {
    return !_greater_than_equal_to(v0, v1);
}
unsafe extern "C" fn init_query_structure(
    mut segnum: libc::c_int,
    mut seg: *mut segment_t,
    mut tr: *mut trap_t,
    mut qs: *mut qnode_t,
) -> libc::c_int {
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut i3: libc::c_int = 0;
    let mut i4: libc::c_int = 0;
    let mut i5: libc::c_int = 0;
    let mut i6: libc::c_int = 0;
    let mut i7: libc::c_int = 0;
    let mut root: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut s: *mut segment_t = &mut *seg.offset(segnum as isize) as *mut segment_t;
    i1 = newnode();
    (*qs.offset(i1 as isize)).nodetype = 2 as libc::c_int;
    _max(
        &mut (*qs.offset(i1 as isize)).yval,
        &mut (*s).v0,
        &mut (*s).v1,
    );
    root = i1;
    i2 = newnode();
    (*qs.offset(i1 as isize)).right = i2;
    (*qs.offset(i2 as isize)).nodetype = 3 as libc::c_int;
    (*qs.offset(i2 as isize)).parent = i1;
    i3 = newnode();
    (*qs.offset(i1 as isize)).left = i3;
    (*qs.offset(i3 as isize)).nodetype = 2 as libc::c_int;
    _min(
        &mut (*qs.offset(i3 as isize)).yval,
        &mut (*s).v0,
        &mut (*s).v1,
    );
    (*qs.offset(i3 as isize)).parent = i1;
    i4 = newnode();
    (*qs.offset(i3 as isize)).left = i4;
    (*qs.offset(i4 as isize)).nodetype = 3 as libc::c_int;
    (*qs.offset(i4 as isize)).parent = i3;
    i5 = newnode();
    (*qs.offset(i3 as isize)).right = i5;
    (*qs.offset(i5 as isize)).nodetype = 1 as libc::c_int;
    (*qs.offset(i5 as isize)).segnum = segnum;
    (*qs.offset(i5 as isize)).parent = i3;
    i6 = newnode();
    (*qs.offset(i5 as isize)).left = i6;
    (*qs.offset(i6 as isize)).nodetype = 3 as libc::c_int;
    (*qs.offset(i6 as isize)).parent = i5;
    i7 = newnode();
    (*qs.offset(i5 as isize)).right = i7;
    (*qs.offset(i7 as isize)).nodetype = 3 as libc::c_int;
    (*qs.offset(i7 as isize)).parent = i5;
    t1 = newtrap(tr);
    t2 = newtrap(tr);
    t3 = newtrap(tr);
    t4 = newtrap(tr);
    let ref mut fresh2 = (*tr.offset(t4 as isize)).lo;
    *fresh2 = (*qs.offset(i1 as isize)).yval;
    let ref mut fresh3 = (*tr.offset(t2 as isize)).hi;
    *fresh3 = *fresh2;
    (*tr.offset(t1 as isize)).hi = *fresh3;
    let ref mut fresh4 = (*tr.offset(t3 as isize)).hi;
    *fresh4 = (*qs.offset(i3 as isize)).yval;
    let ref mut fresh5 = (*tr.offset(t2 as isize)).lo;
    *fresh5 = *fresh4;
    (*tr.offset(t1 as isize)).lo = *fresh5;
    (*tr.offset(t4 as isize)).hi.y = ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_double;
    (*tr.offset(t4 as isize)).hi.x = ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_double;
    (*tr.offset(t3 as isize)).lo.y = -(1 as libc::c_int) as libc::c_double
        * ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_double;
    (*tr.offset(t3 as isize)).lo.x = -(1 as libc::c_int) as libc::c_double
        * ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_double;
    let ref mut fresh6 = (*tr.offset(t2 as isize)).lseg;
    *fresh6 = segnum;
    (*tr.offset(t1 as isize)).rseg = *fresh6;
    let ref mut fresh7 = (*tr.offset(t2 as isize)).u0;
    *fresh7 = t4;
    (*tr.offset(t1 as isize)).u0 = *fresh7;
    let ref mut fresh8 = (*tr.offset(t2 as isize)).d0;
    *fresh8 = t3;
    (*tr.offset(t1 as isize)).d0 = *fresh8;
    let ref mut fresh9 = (*tr.offset(t3 as isize)).u0;
    *fresh9 = t1;
    (*tr.offset(t4 as isize)).d0 = *fresh9;
    let ref mut fresh10 = (*tr.offset(t3 as isize)).u1;
    *fresh10 = t2;
    (*tr.offset(t4 as isize)).d1 = *fresh10;
    (*tr.offset(t1 as isize)).sink = i6;
    (*tr.offset(t2 as isize)).sink = i7;
    (*tr.offset(t3 as isize)).sink = i4;
    (*tr.offset(t4 as isize)).sink = i2;
    let ref mut fresh11 = (*tr.offset(t2 as isize)).state;
    *fresh11 = 1 as libc::c_int;
    (*tr.offset(t1 as isize)).state = *fresh11;
    let ref mut fresh12 = (*tr.offset(t4 as isize)).state;
    *fresh12 = 1 as libc::c_int;
    (*tr.offset(t3 as isize)).state = *fresh12;
    (*qs.offset(i2 as isize)).trnum = t4;
    (*qs.offset(i4 as isize)).trnum = t3;
    (*qs.offset(i6 as isize)).trnum = t1;
    (*qs.offset(i7 as isize)).trnum = t2;
    (*s).is_inserted = 1 as libc::c_int != 0;
    return root;
}
unsafe extern "C" fn is_left_of(
    mut segnum: libc::c_int,
    mut seg: *mut segment_t,
    mut v: *mut pointf,
) -> bool {
    let mut s: *mut segment_t = &mut *seg.offset(segnum as isize) as *mut segment_t;
    let mut area: libc::c_double = 0.;
    if if (*s).v1.y > (*s).v0.y + 1.0e-7f64 {
        1 as libc::c_int
    } else if (*s).v1.y < (*s).v0.y - 1.0e-7f64 {
        0 as libc::c_int
    } else {
        ((*s).v1.x > (*s).v0.x) as libc::c_int
    } != 0
    {
        if fabs((*s).v1.y - (*v).y) <= 1.0e-7f64 {
            if (*v).x < (*s).v1.x {
                area = 1.0f64;
            } else {
                area = -1.0f64;
            }
        } else if fabs((*s).v0.y - (*v).y) <= 1.0e-7f64 {
            if (*v).x < (*s).v0.x {
                area = 1.0f64;
            } else {
                area = -1.0f64;
            }
        } else {
            area = ((*s).v1.x - (*s).v0.x) * ((*v).y - (*s).v0.y)
                - ((*s).v1.y - (*s).v0.y) * ((*v).x - (*s).v0.x);
        }
    } else if fabs((*s).v1.y - (*v).y) <= 1.0e-7f64 {
        if (*v).x < (*s).v1.x {
            area = 1.0f64;
        } else {
            area = -1.0f64;
        }
    } else if fabs((*s).v0.y - (*v).y) <= 1.0e-7f64 {
        if (*v).x < (*s).v0.x {
            area = 1.0f64;
        } else {
            area = -1.0f64;
        }
    } else {
        area = ((*s).v0.x - (*s).v1.x) * ((*v).y - (*s).v1.y)
            - ((*s).v0.y - (*s).v1.y) * ((*v).x - (*s).v1.x);
    }
    return area > 0.0f64;
}
unsafe extern "C" fn inserted(
    mut segnum: libc::c_int,
    mut seg: *mut segment_t,
    mut whichpt: libc::c_int,
) -> bool {
    if whichpt == 1 as libc::c_int {
        return (*seg.offset((*seg.offset(segnum as isize)).prev as isize)).is_inserted;
    } else {
        return (*seg.offset((*seg.offset(segnum as isize)).next as isize)).is_inserted;
    };
}
unsafe extern "C" fn locate_endpoint(
    mut v: *mut pointf,
    mut vo: *mut pointf,
    mut r: libc::c_int,
    mut seg: *mut segment_t,
    mut qs: *mut qnode_t,
) -> libc::c_int {
    let mut rptr: *mut qnode_t = &mut *qs.offset(r as isize) as *mut qnode_t;
    match (*rptr).nodetype {
        3 => return (*rptr).trnum,
        2 => {
            if if (*v).y > (*rptr).yval.y + 1.0e-7f64 {
                1 as libc::c_int
            } else if (*v).y < (*rptr).yval.y - 1.0e-7f64 {
                0 as libc::c_int
            } else {
                ((*v).x > (*rptr).yval.x) as libc::c_int
            } != 0
            {
                return locate_endpoint(v, vo, (*rptr).right, seg, qs);
            } else if fabs((*v).y - (*rptr).yval.y) <= 1.0e-7f64
                && fabs((*v).x - (*rptr).yval.x) <= 1.0e-7f64
            {
                if if (*vo).y > (*rptr).yval.y + 1.0e-7f64 {
                    1 as libc::c_int
                } else if (*vo).y < (*rptr).yval.y - 1.0e-7f64 {
                    0 as libc::c_int
                } else {
                    ((*vo).x > (*rptr).yval.x) as libc::c_int
                } != 0
                {
                    return locate_endpoint(v, vo, (*rptr).right, seg, qs);
                } else {
                    return locate_endpoint(v, vo, (*rptr).left, seg, qs);
                }
            } else {
                return locate_endpoint(v, vo, (*rptr).left, seg, qs);
            }
        }
        1 => {
            if fabs((*v).y - (*seg.offset((*rptr).segnum as isize)).v0.y) <= 1.0e-7f64
                && fabs((*v).x - (*seg.offset((*rptr).segnum as isize)).v0.x) <= 1.0e-7f64
                || fabs((*v).y - (*seg.offset((*rptr).segnum as isize)).v1.y) <= 1.0e-7f64
                    && fabs((*v).x - (*seg.offset((*rptr).segnum as isize)).v1.x) <= 1.0e-7f64
            {
                if fabs((*v).y - (*vo).y) <= 1.0e-7f64 {
                    if (*vo).x < (*v).x {
                        return locate_endpoint(v, vo, (*rptr).left, seg, qs);
                    } else {
                        return locate_endpoint(v, vo, (*rptr).right, seg, qs);
                    }
                } else if is_left_of((*rptr).segnum, seg, vo) {
                    return locate_endpoint(v, vo, (*rptr).left, seg, qs);
                } else {
                    return locate_endpoint(v, vo, (*rptr).right, seg, qs);
                }
            } else if is_left_of((*rptr).segnum, seg, v) {
                return locate_endpoint(v, vo, (*rptr).left, seg, qs);
            } else {
                return locate_endpoint(v, vo, (*rptr).right, seg, qs);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"unexpected case in locate_endpoint\n\0" as *const u8 as *const libc::c_char,
            );
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"trapezoid.c\0" as *const u8 as *const libc::c_char,
                336 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"int locate_endpoint(pointf *, pointf *, int, segment_t *, qnode_t *)\0",
                ))
                .as_ptr(),
            );
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn merge_trapezoids(
    mut segnum: libc::c_int,
    mut tfirst: libc::c_int,
    mut tlast: libc::c_int,
    mut side: libc::c_int,
    mut tr: *mut trap_t,
    mut qs: *mut qnode_t,
) {
    let mut t: libc::c_int = 0;
    t = tfirst;
    while t > 0 as libc::c_int
        && _greater_than_equal_to(
            &mut (*tr.offset(t as isize)).lo,
            &mut (*tr.offset(tlast as isize)).lo,
        ) as libc::c_int
            != 0
    {
        let mut tnext: libc::c_int = 0;
        let mut ptnext: libc::c_int = 0;
        let mut cond: bool = false;
        if side == 1 as libc::c_int {
            tnext = (*tr.offset(t as isize)).d0;
            cond = tnext > 0 as libc::c_int && (*tr.offset(tnext as isize)).rseg == segnum || {
                tnext = (*tr.offset(t as isize)).d1;
                tnext > 0 as libc::c_int && (*tr.offset(tnext as isize)).rseg == segnum
            };
        } else {
            tnext = (*tr.offset(t as isize)).d0;
            cond = tnext > 0 as libc::c_int && (*tr.offset(tnext as isize)).lseg == segnum || {
                tnext = (*tr.offset(t as isize)).d1;
                tnext > 0 as libc::c_int && (*tr.offset(tnext as isize)).lseg == segnum
            };
        }
        if cond {
            if (*tr.offset(t as isize)).lseg == (*tr.offset(tnext as isize)).lseg
                && (*tr.offset(t as isize)).rseg == (*tr.offset(tnext as isize)).rseg
            {
                ptnext = (*qs.offset((*tr.offset(tnext as isize)).sink as isize)).parent;
                if (*qs.offset(ptnext as isize)).left == (*tr.offset(tnext as isize)).sink {
                    (*qs.offset(ptnext as isize)).left = (*tr.offset(t as isize)).sink;
                } else {
                    (*qs.offset(ptnext as isize)).right = (*tr.offset(t as isize)).sink;
                }
                let ref mut fresh13 = (*tr.offset(t as isize)).d0;
                *fresh13 = (*tr.offset(tnext as isize)).d0;
                if *fresh13 > 0 as libc::c_int {
                    if (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 == tnext {
                        (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 = t;
                    } else if (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1 == tnext {
                        (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1 = t;
                    }
                }
                let ref mut fresh14 = (*tr.offset(t as isize)).d1;
                *fresh14 = (*tr.offset(tnext as isize)).d1;
                if *fresh14 > 0 as libc::c_int {
                    if (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 == tnext {
                        (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 = t;
                    } else if (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1 == tnext {
                        (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1 = t;
                    }
                }
                (*tr.offset(t as isize)).lo = (*tr.offset(tnext as isize)).lo;
                (*tr.offset(tnext as isize)).state = 2 as libc::c_int;
            } else {
                t = tnext;
            }
        } else {
            t = tnext;
        }
    }
}
unsafe extern "C" fn add_segment(
    mut segnum: libc::c_int,
    mut seg: *mut segment_t,
    mut tr: *mut trap_t,
    mut qs: *mut qnode_t,
) -> libc::c_int {
    let mut s: segment_t = segment_t {
        v0: pointf { x: 0., y: 0. },
        v1: pointf { x: 0., y: 0. },
        is_inserted: false,
        root0: 0,
        root1: 0,
        next: 0,
        prev: 0,
    };
    let mut tu: libc::c_int = 0;
    let mut tl: libc::c_int = 0;
    let mut sk: libc::c_int = 0;
    let mut tfirst: libc::c_int = 0;
    let mut tlast: libc::c_int = 0;
    let mut tfirstr: libc::c_int = 0 as libc::c_int;
    let mut tlastr: libc::c_int = 0 as libc::c_int;
    let mut tfirstl: libc::c_int = 0 as libc::c_int;
    let mut tlastl: libc::c_int = 0 as libc::c_int;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut tn: libc::c_int = 0;
    let mut tpt: pointf = pointf { x: 0., y: 0. };
    let mut tribot: libc::c_int = 0 as libc::c_int;
    let mut is_swapped: libc::c_int = 0;
    let mut tmptriseg: libc::c_int = 0;
    s = *seg.offset(segnum as isize);
    if if s.v1.y > s.v0.y + 1.0e-7f64 {
        1 as libc::c_int
    } else if s.v1.y < s.v0.y - 1.0e-7f64 {
        0 as libc::c_int
    } else {
        (s.v1.x > s.v0.x) as libc::c_int
    } != 0
    {
        let mut tmp: libc::c_int = 0;
        tpt = s.v0;
        s.v0 = s.v1;
        s.v1 = tpt;
        tmp = s.root0;
        s.root0 = s.root1;
        s.root1 = tmp;
        is_swapped = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        is_swapped = 0 as libc::c_int;
    }
    if !inserted(
        segnum,
        seg,
        if is_swapped != 0 {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        },
    ) {
        let mut tmp_d: libc::c_int = 0;
        tu = locate_endpoint(&mut s.v0, &mut s.v1, s.root0, seg, qs);
        tl = newtrap(tr);
        (*tr.offset(tl as isize)).state = 1 as libc::c_int;
        *tr.offset(tl as isize) = *tr.offset(tu as isize);
        let ref mut fresh15 = (*tr.offset(tl as isize)).hi.y;
        *fresh15 = s.v0.y;
        (*tr.offset(tu as isize)).lo.y = *fresh15;
        let ref mut fresh16 = (*tr.offset(tl as isize)).hi.x;
        *fresh16 = s.v0.x;
        (*tr.offset(tu as isize)).lo.x = *fresh16;
        (*tr.offset(tu as isize)).d0 = tl;
        (*tr.offset(tu as isize)).d1 = 0 as libc::c_int;
        (*tr.offset(tl as isize)).u0 = tu;
        (*tr.offset(tl as isize)).u1 = 0 as libc::c_int;
        tmp_d = (*tr.offset(tl as isize)).d0;
        if tmp_d > 0 as libc::c_int && (*tr.offset(tmp_d as isize)).u0 == tu {
            (*tr.offset(tmp_d as isize)).u0 = tl;
        }
        tmp_d = (*tr.offset(tl as isize)).d0;
        if tmp_d > 0 as libc::c_int && (*tr.offset(tmp_d as isize)).u1 == tu {
            (*tr.offset(tmp_d as isize)).u1 = tl;
        }
        tmp_d = (*tr.offset(tl as isize)).d1;
        if tmp_d > 0 as libc::c_int && (*tr.offset(tmp_d as isize)).u0 == tu {
            (*tr.offset(tmp_d as isize)).u0 = tl;
        }
        tmp_d = (*tr.offset(tl as isize)).d1;
        if tmp_d > 0 as libc::c_int && (*tr.offset(tmp_d as isize)).u1 == tu {
            (*tr.offset(tmp_d as isize)).u1 = tl;
        }
        i1 = newnode();
        i2 = newnode();
        sk = (*tr.offset(tu as isize)).sink;
        (*qs.offset(sk as isize)).nodetype = 2 as libc::c_int;
        (*qs.offset(sk as isize)).yval = s.v0;
        (*qs.offset(sk as isize)).segnum = segnum;
        (*qs.offset(sk as isize)).left = i2;
        (*qs.offset(sk as isize)).right = i1;
        (*qs.offset(i1 as isize)).nodetype = 3 as libc::c_int;
        (*qs.offset(i1 as isize)).trnum = tu;
        (*qs.offset(i1 as isize)).parent = sk;
        (*qs.offset(i2 as isize)).nodetype = 3 as libc::c_int;
        (*qs.offset(i2 as isize)).trnum = tl;
        (*qs.offset(i2 as isize)).parent = sk;
        (*tr.offset(tu as isize)).sink = i1;
        (*tr.offset(tl as isize)).sink = i2;
        tfirst = tl;
    } else {
        tfirst = locate_endpoint(&mut s.v0, &mut s.v1, s.root0, seg, qs);
    }
    if !inserted(
        segnum,
        seg,
        if is_swapped != 0 {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        },
    ) {
        let mut tmp_d_0: libc::c_int = 0;
        tu = locate_endpoint(&mut s.v1, &mut s.v0, s.root1, seg, qs);
        tl = newtrap(tr);
        (*tr.offset(tl as isize)).state = 1 as libc::c_int;
        *tr.offset(tl as isize) = *tr.offset(tu as isize);
        let ref mut fresh17 = (*tr.offset(tl as isize)).hi.y;
        *fresh17 = s.v1.y;
        (*tr.offset(tu as isize)).lo.y = *fresh17;
        let ref mut fresh18 = (*tr.offset(tl as isize)).hi.x;
        *fresh18 = s.v1.x;
        (*tr.offset(tu as isize)).lo.x = *fresh18;
        (*tr.offset(tu as isize)).d0 = tl;
        (*tr.offset(tu as isize)).d1 = 0 as libc::c_int;
        (*tr.offset(tl as isize)).u0 = tu;
        (*tr.offset(tl as isize)).u1 = 0 as libc::c_int;
        tmp_d_0 = (*tr.offset(tl as isize)).d0;
        if tmp_d_0 > 0 as libc::c_int && (*tr.offset(tmp_d_0 as isize)).u0 == tu {
            (*tr.offset(tmp_d_0 as isize)).u0 = tl;
        }
        tmp_d_0 = (*tr.offset(tl as isize)).d0;
        if tmp_d_0 > 0 as libc::c_int && (*tr.offset(tmp_d_0 as isize)).u1 == tu {
            (*tr.offset(tmp_d_0 as isize)).u1 = tl;
        }
        tmp_d_0 = (*tr.offset(tl as isize)).d1;
        if tmp_d_0 > 0 as libc::c_int && (*tr.offset(tmp_d_0 as isize)).u0 == tu {
            (*tr.offset(tmp_d_0 as isize)).u0 = tl;
        }
        tmp_d_0 = (*tr.offset(tl as isize)).d1;
        if tmp_d_0 > 0 as libc::c_int && (*tr.offset(tmp_d_0 as isize)).u1 == tu {
            (*tr.offset(tmp_d_0 as isize)).u1 = tl;
        }
        i1 = newnode();
        i2 = newnode();
        sk = (*tr.offset(tu as isize)).sink;
        (*qs.offset(sk as isize)).nodetype = 2 as libc::c_int;
        (*qs.offset(sk as isize)).yval = s.v1;
        (*qs.offset(sk as isize)).segnum = segnum;
        (*qs.offset(sk as isize)).left = i2;
        (*qs.offset(sk as isize)).right = i1;
        (*qs.offset(i1 as isize)).nodetype = 3 as libc::c_int;
        (*qs.offset(i1 as isize)).trnum = tu;
        (*qs.offset(i1 as isize)).parent = sk;
        (*qs.offset(i2 as isize)).nodetype = 3 as libc::c_int;
        (*qs.offset(i2 as isize)).trnum = tl;
        (*qs.offset(i2 as isize)).parent = sk;
        (*tr.offset(tu as isize)).sink = i1;
        (*tr.offset(tl as isize)).sink = i2;
        tlast = tu;
    } else {
        tlast = locate_endpoint(&mut s.v1, &mut s.v0, s.root1, seg, qs);
        tribot = 1 as libc::c_int;
    }
    t = tfirst;
    while t > 0 as libc::c_int
        && _greater_than_equal_to(
            &mut (*tr.offset(t as isize)).lo,
            &mut (*tr.offset(tlast as isize)).lo,
        ) as libc::c_int
            != 0
    {
        let mut t_sav: libc::c_int = 0;
        let mut tn_sav: libc::c_int = 0;
        sk = (*tr.offset(t as isize)).sink;
        i1 = newnode();
        i2 = newnode();
        (*qs.offset(sk as isize)).nodetype = 1 as libc::c_int;
        (*qs.offset(sk as isize)).segnum = segnum;
        (*qs.offset(sk as isize)).left = i1;
        (*qs.offset(sk as isize)).right = i2;
        (*qs.offset(i1 as isize)).nodetype = 3 as libc::c_int;
        (*qs.offset(i1 as isize)).trnum = t;
        (*qs.offset(i1 as isize)).parent = sk;
        (*qs.offset(i2 as isize)).nodetype = 3 as libc::c_int;
        tn = newtrap(tr);
        (*qs.offset(i2 as isize)).trnum = tn;
        (*tr.offset(tn as isize)).state = 1 as libc::c_int;
        (*qs.offset(i2 as isize)).parent = sk;
        if t == tfirst {
            tfirstr = tn;
        }
        if fabs((*tr.offset(t as isize)).lo.y - (*tr.offset(tlast as isize)).lo.y) <= 1.0e-7f64
            && fabs((*tr.offset(t as isize)).lo.x - (*tr.offset(tlast as isize)).lo.x) <= 1.0e-7f64
        {
            tlastr = tn;
        }
        *tr.offset(tn as isize) = *tr.offset(t as isize);
        (*tr.offset(t as isize)).sink = i1;
        (*tr.offset(tn as isize)).sink = i2;
        t_sav = t;
        tn_sav = tn;
        if (*tr.offset(t as isize)).d0 <= 0 as libc::c_int
            && (*tr.offset(t as isize)).d1 <= 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"add_segment: error\n\0" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            if (*tr.offset(t as isize)).d0 > 0 as libc::c_int
                && (*tr.offset(t as isize)).d1 <= 0 as libc::c_int
            {
                if (*tr.offset(t as isize)).u0 > 0 as libc::c_int
                    && (*tr.offset(t as isize)).u1 > 0 as libc::c_int
                {
                    if (*tr.offset(t as isize)).usave > 0 as libc::c_int {
                        if (*tr.offset(t as isize)).uside == 1 as libc::c_int {
                            (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                            (*tr.offset(t as isize)).u1 = -(1 as libc::c_int);
                            (*tr.offset(tn as isize)).u1 = (*tr.offset(t as isize)).usave;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                            (*tr.offset((*tr.offset(tn as isize)).u1 as isize)).d0 = tn;
                        } else {
                            (*tr.offset(tn as isize)).u1 = -(1 as libc::c_int);
                            (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                            (*tr.offset(t as isize)).u1 = (*tr.offset(t as isize)).u0;
                            (*tr.offset(t as isize)).u0 = (*tr.offset(t as isize)).usave;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(t as isize)).u1 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                        }
                        let ref mut fresh19 = (*tr.offset(tn as isize)).usave;
                        *fresh19 = 0 as libc::c_int;
                        (*tr.offset(t as isize)).usave = *fresh19;
                    } else {
                        (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                        let ref mut fresh20 = (*tr.offset(tn as isize)).u1;
                        *fresh20 = -(1 as libc::c_int);
                        (*tr.offset(t as isize)).u1 = *fresh20;
                        (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                    }
                } else {
                    let mut tmp_u: libc::c_int = (*tr.offset(t as isize)).u0;
                    let mut td0: libc::c_int = 0;
                    let mut td1: libc::c_int = 0;
                    td0 = (*tr.offset(tmp_u as isize)).d0;
                    if td0 > 0 as libc::c_int && {
                        td1 = (*tr.offset(tmp_u as isize)).d1;
                        td1 > 0 as libc::c_int
                    } {
                        if (*tr.offset(td0 as isize)).rseg > 0 as libc::c_int
                            && !is_left_of((*tr.offset(td0 as isize)).rseg, seg, &mut s.v1)
                        {
                            let ref mut fresh21 = (*tr.offset(tn as isize)).u1;
                            *fresh21 = -(1 as libc::c_int);
                            let ref mut fresh22 = (*tr.offset(t as isize)).u1;
                            *fresh22 = *fresh21;
                            (*tr.offset(t as isize)).u0 = *fresh22;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d1 = tn;
                        } else {
                            let ref mut fresh23 = (*tr.offset(t as isize)).u1;
                            *fresh23 = -(1 as libc::c_int);
                            let ref mut fresh24 = (*tr.offset(tn as isize)).u1;
                            *fresh24 = *fresh23;
                            (*tr.offset(tn as isize)).u0 = *fresh24;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                        }
                    } else {
                        (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                        (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d1 = tn;
                    }
                }
                if fabs((*tr.offset(t as isize)).lo.y - (*tr.offset(tlast as isize)).lo.y)
                    <= 1.0e-7f64
                    && fabs((*tr.offset(t as isize)).lo.x - (*tr.offset(tlast as isize)).lo.x)
                        <= 1.0e-7f64
                    && tribot != 0
                {
                    if is_swapped != 0 {
                        tmptriseg = (*seg.offset(segnum as isize)).prev;
                    } else {
                        tmptriseg = (*seg.offset(segnum as isize)).next;
                    }
                    if tmptriseg > 0 as libc::c_int
                        && is_left_of(tmptriseg, seg, &mut s.v0) as libc::c_int != 0
                    {
                        (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 = t;
                        let ref mut fresh25 = (*tr.offset(tn as isize)).d1;
                        *fresh25 = -(1 as libc::c_int);
                        (*tr.offset(tn as isize)).d0 = *fresh25;
                    } else {
                        (*tr.offset((*tr.offset(tn as isize)).d0 as isize)).u1 = tn;
                        let ref mut fresh26 = (*tr.offset(t as isize)).d1;
                        *fresh26 = -(1 as libc::c_int);
                        (*tr.offset(t as isize)).d0 = *fresh26;
                    }
                } else {
                    if (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 > 0 as libc::c_int
                        && (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1 > 0 as libc::c_int
                    {
                        if (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 == t {
                            (*tr.offset((*tr.offset(t as isize)).d0 as isize)).usave =
                                (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1;
                            (*tr.offset((*tr.offset(t as isize)).d0 as isize)).uside =
                                1 as libc::c_int;
                        } else {
                            (*tr.offset((*tr.offset(t as isize)).d0 as isize)).usave =
                                (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0;
                            (*tr.offset((*tr.offset(t as isize)).d0 as isize)).uside =
                                2 as libc::c_int;
                        }
                    }
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 = t;
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1 = tn;
                }
                t = (*tr.offset(t as isize)).d0;
            } else if (*tr.offset(t as isize)).d0 <= 0 as libc::c_int
                && (*tr.offset(t as isize)).d1 > 0 as libc::c_int
            {
                if (*tr.offset(t as isize)).u0 > 0 as libc::c_int
                    && (*tr.offset(t as isize)).u1 > 0 as libc::c_int
                {
                    if (*tr.offset(t as isize)).usave > 0 as libc::c_int {
                        if (*tr.offset(t as isize)).uside == 1 as libc::c_int {
                            (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                            (*tr.offset(t as isize)).u1 = -(1 as libc::c_int);
                            (*tr.offset(tn as isize)).u1 = (*tr.offset(t as isize)).usave;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                            (*tr.offset((*tr.offset(tn as isize)).u1 as isize)).d0 = tn;
                        } else {
                            (*tr.offset(tn as isize)).u1 = -(1 as libc::c_int);
                            (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                            (*tr.offset(t as isize)).u1 = (*tr.offset(t as isize)).u0;
                            (*tr.offset(t as isize)).u0 = (*tr.offset(t as isize)).usave;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(t as isize)).u1 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                        }
                        let ref mut fresh27 = (*tr.offset(tn as isize)).usave;
                        *fresh27 = 0 as libc::c_int;
                        (*tr.offset(t as isize)).usave = *fresh27;
                    } else {
                        (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                        let ref mut fresh28 = (*tr.offset(tn as isize)).u1;
                        *fresh28 = -(1 as libc::c_int);
                        (*tr.offset(t as isize)).u1 = *fresh28;
                        (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                    }
                } else {
                    let mut tmp_u_0: libc::c_int = (*tr.offset(t as isize)).u0;
                    let mut td0_0: libc::c_int = 0;
                    let mut td1_0: libc::c_int = 0;
                    td0_0 = (*tr.offset(tmp_u_0 as isize)).d0;
                    if td0_0 > 0 as libc::c_int && {
                        td1_0 = (*tr.offset(tmp_u_0 as isize)).d1;
                        td1_0 > 0 as libc::c_int
                    } {
                        if (*tr.offset(td0_0 as isize)).rseg > 0 as libc::c_int
                            && !is_left_of((*tr.offset(td0_0 as isize)).rseg, seg, &mut s.v1)
                        {
                            let ref mut fresh29 = (*tr.offset(tn as isize)).u1;
                            *fresh29 = -(1 as libc::c_int);
                            let ref mut fresh30 = (*tr.offset(t as isize)).u1;
                            *fresh30 = *fresh29;
                            (*tr.offset(t as isize)).u0 = *fresh30;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d1 = tn;
                        } else {
                            let ref mut fresh31 = (*tr.offset(t as isize)).u1;
                            *fresh31 = -(1 as libc::c_int);
                            let ref mut fresh32 = (*tr.offset(tn as isize)).u1;
                            *fresh32 = *fresh31;
                            (*tr.offset(tn as isize)).u0 = *fresh32;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                        }
                    } else {
                        (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                        (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d1 = tn;
                    }
                }
                if fabs((*tr.offset(t as isize)).lo.y - (*tr.offset(tlast as isize)).lo.y)
                    <= 1.0e-7f64
                    && fabs((*tr.offset(t as isize)).lo.x - (*tr.offset(tlast as isize)).lo.x)
                        <= 1.0e-7f64
                    && tribot != 0
                {
                    if is_swapped != 0 {
                        tmptriseg = (*seg.offset(segnum as isize)).prev;
                    } else {
                        tmptriseg = (*seg.offset(segnum as isize)).next;
                    }
                    if tmptriseg > 0 as libc::c_int
                        && is_left_of(tmptriseg, seg, &mut s.v0) as libc::c_int != 0
                    {
                        (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 = t;
                        let ref mut fresh33 = (*tr.offset(tn as isize)).d1;
                        *fresh33 = -(1 as libc::c_int);
                        (*tr.offset(tn as isize)).d0 = *fresh33;
                    } else {
                        (*tr.offset((*tr.offset(tn as isize)).d1 as isize)).u1 = tn;
                        let ref mut fresh34 = (*tr.offset(t as isize)).d1;
                        *fresh34 = -(1 as libc::c_int);
                        (*tr.offset(t as isize)).d0 = *fresh34;
                    }
                } else {
                    if (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 > 0 as libc::c_int
                        && (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1 > 0 as libc::c_int
                    {
                        if (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 == t {
                            (*tr.offset((*tr.offset(t as isize)).d1 as isize)).usave =
                                (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1;
                            (*tr.offset((*tr.offset(t as isize)).d1 as isize)).uside =
                                1 as libc::c_int;
                        } else {
                            (*tr.offset((*tr.offset(t as isize)).d1 as isize)).usave =
                                (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0;
                            (*tr.offset((*tr.offset(t as isize)).d1 as isize)).uside =
                                2 as libc::c_int;
                        }
                    }
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 = t;
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1 = tn;
                }
                t = (*tr.offset(t as isize)).d1;
            } else {
                let mut y0: libc::c_double = 0.;
                let mut yt: libc::c_double = 0.;
                let mut tmppt: pointf = pointf { x: 0., y: 0. };
                let mut tnext: libc::c_int = 0;
                let mut i_d0: libc::c_int = 0;
                let mut i_d1: libc::c_int = 0;
                i_d1 = 0 as libc::c_int;
                i_d0 = i_d1;
                if fabs((*tr.offset(t as isize)).lo.y - s.v0.y) <= 1.0e-7f64 {
                    if (*tr.offset(t as isize)).lo.x > s.v0.x {
                        i_d0 = (0 as libc::c_int == 0) as libc::c_int;
                    } else {
                        i_d1 = (0 as libc::c_int == 0) as libc::c_int;
                    }
                } else {
                    y0 = (*tr.offset(t as isize)).lo.y;
                    tmppt.y = y0;
                    yt = (y0 - s.v0.y) / (s.v1.y - s.v0.y);
                    tmppt.x = s.v0.x + yt * (s.v1.x - s.v0.x);
                    if _less_than(&mut tmppt, &mut (*tr.offset(t as isize)).lo) {
                        i_d0 = (0 as libc::c_int == 0) as libc::c_int;
                    } else {
                        i_d1 = (0 as libc::c_int == 0) as libc::c_int;
                    }
                }
                if (*tr.offset(t as isize)).u0 > 0 as libc::c_int
                    && (*tr.offset(t as isize)).u1 > 0 as libc::c_int
                {
                    if (*tr.offset(t as isize)).usave > 0 as libc::c_int {
                        if (*tr.offset(t as isize)).uside == 1 as libc::c_int {
                            (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                            (*tr.offset(t as isize)).u1 = -(1 as libc::c_int);
                            (*tr.offset(tn as isize)).u1 = (*tr.offset(t as isize)).usave;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                            (*tr.offset((*tr.offset(tn as isize)).u1 as isize)).d0 = tn;
                        } else {
                            (*tr.offset(tn as isize)).u1 = -(1 as libc::c_int);
                            (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                            (*tr.offset(t as isize)).u1 = (*tr.offset(t as isize)).u0;
                            (*tr.offset(t as isize)).u0 = (*tr.offset(t as isize)).usave;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(t as isize)).u1 as isize)).d0 = t;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                        }
                        let ref mut fresh35 = (*tr.offset(tn as isize)).usave;
                        *fresh35 = 0 as libc::c_int;
                        (*tr.offset(t as isize)).usave = *fresh35;
                    } else {
                        (*tr.offset(tn as isize)).u0 = (*tr.offset(t as isize)).u1;
                        (*tr.offset(tn as isize)).u1 = -(1 as libc::c_int);
                        (*tr.offset(t as isize)).u1 = -(1 as libc::c_int);
                        (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d0 = tn;
                    }
                } else {
                    let mut tmp_u_1: libc::c_int = (*tr.offset(t as isize)).u0;
                    let mut td0_1: libc::c_int = 0;
                    let mut td1_1: libc::c_int = 0;
                    td0_1 = (*tr.offset(tmp_u_1 as isize)).d0;
                    if td0_1 > 0 as libc::c_int && {
                        td1_1 = (*tr.offset(tmp_u_1 as isize)).d1;
                        td1_1 > 0 as libc::c_int
                    } {
                        if (*tr.offset(td0_1 as isize)).rseg > 0 as libc::c_int
                            && !is_left_of((*tr.offset(td0_1 as isize)).rseg, seg, &mut s.v1)
                        {
                            let ref mut fresh36 = (*tr.offset(tn as isize)).u1;
                            *fresh36 = -(1 as libc::c_int);
                            let ref mut fresh37 = (*tr.offset(t as isize)).u1;
                            *fresh37 = *fresh36;
                            (*tr.offset(t as isize)).u0 = *fresh37;
                            (*tr.offset((*tr.offset(tn as isize)).u0 as isize)).d1 = tn;
                        } else {
                            let ref mut fresh38 = (*tr.offset(t as isize)).u1;
                            *fresh38 = -(1 as libc::c_int);
                            let ref mut fresh39 = (*tr.offset(tn as isize)).u1;
                            *fresh39 = *fresh38;
                            (*tr.offset(tn as isize)).u0 = *fresh39;
                            (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                        }
                    } else {
                        (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d0 = t;
                        (*tr.offset((*tr.offset(t as isize)).u0 as isize)).d1 = tn;
                    }
                }
                if fabs((*tr.offset(t as isize)).lo.y - (*tr.offset(tlast as isize)).lo.y)
                    <= 1.0e-7f64
                    && fabs((*tr.offset(t as isize)).lo.x - (*tr.offset(tlast as isize)).lo.x)
                        <= 1.0e-7f64
                    && tribot != 0
                {
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 = t;
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1 = -(1 as libc::c_int);
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 = tn;
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1 = -(1 as libc::c_int);
                    (*tr.offset(tn as isize)).d0 = (*tr.offset(t as isize)).d1;
                    let ref mut fresh40 = (*tr.offset(tn as isize)).d1;
                    *fresh40 = -(1 as libc::c_int);
                    (*tr.offset(t as isize)).d1 = *fresh40;
                    tnext = (*tr.offset(t as isize)).d1;
                } else if i_d0 != 0 {
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 = t;
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1 = tn;
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 = tn;
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1 = -(1 as libc::c_int);
                    (*tr.offset(t as isize)).d1 = -(1 as libc::c_int);
                    tnext = (*tr.offset(t as isize)).d0;
                } else {
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u0 = t;
                    (*tr.offset((*tr.offset(t as isize)).d0 as isize)).u1 = -(1 as libc::c_int);
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u0 = t;
                    (*tr.offset((*tr.offset(t as isize)).d1 as isize)).u1 = tn;
                    (*tr.offset(tn as isize)).d0 = (*tr.offset(t as isize)).d1;
                    (*tr.offset(tn as isize)).d1 = -(1 as libc::c_int);
                    tnext = (*tr.offset(t as isize)).d1;
                }
                t = tnext;
            }
            let ref mut fresh41 = (*tr.offset(tn_sav as isize)).lseg;
            *fresh41 = segnum;
            (*tr.offset(t_sav as isize)).rseg = *fresh41;
        }
    }
    tfirstl = tfirst;
    tlastl = tlast;
    merge_trapezoids(segnum, tfirstl, tlastl, 1 as libc::c_int, tr, qs);
    merge_trapezoids(segnum, tfirstr, tlastr, 2 as libc::c_int, tr, qs);
    (*seg.offset(segnum as isize)).is_inserted = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_new_roots(
    mut segnum: libc::c_int,
    mut seg: *mut segment_t,
    mut tr: *mut trap_t,
    mut qs: *mut qnode_t,
) {
    let mut s: *mut segment_t = &mut *seg.offset(segnum as isize) as *mut segment_t;
    if (*s).is_inserted {
        return;
    }
    (*s).root0 = locate_endpoint(&mut (*s).v0, &mut (*s).v1, (*s).root0, seg, qs);
    (*s).root0 = (*tr.offset((*s).root0 as isize)).sink;
    (*s).root1 = locate_endpoint(&mut (*s).v1, &mut (*s).v0, (*s).root1, seg, qs);
    (*s).root1 = (*tr.offset((*s).root1 as isize)).sink;
}
unsafe extern "C" fn math_logstar_n(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_double = 0.;
    i = 0 as libc::c_int;
    v = n as libc::c_double;
    while v >= 1 as libc::c_int as libc::c_double {
        v = log2(v);
        i += 1;
    }
    return i - 1 as libc::c_int;
}
unsafe extern "C" fn math_N(mut n: libc::c_int, mut h: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_double = 0.;
    i = 0 as libc::c_int;
    v = n as libc::c_double;
    while i < h {
        v = log2(v);
        i += 1;
    }
    return ceil(1.0f64 * n as libc::c_double / v) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn construct_trapezoids(
    mut nseg: libc::c_int,
    mut seg: *mut segment_t,
    mut permute: *mut libc::c_int,
    mut ntraps: libc::c_int,
    mut tr: *mut trap_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut root: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut segi: libc::c_int = 1 as libc::c_int;
    let mut qs: *mut qnode_t = 0 as *mut qnode_t;
    QSIZE = 2 as libc::c_int * ntraps;
    TRSIZE = ntraps;
    qs = gcalloc(
        (2 as libc::c_int * ntraps) as size_t,
        ::std::mem::size_of::<qnode_t>() as libc::c_ulong,
    ) as *mut qnode_t;
    tr_idx = 1 as libc::c_int;
    q_idx = tr_idx;
    memset(
        tr as *mut libc::c_void,
        0 as libc::c_int,
        (ntraps as libc::c_ulong).wrapping_mul(::std::mem::size_of::<trap_t>() as libc::c_ulong),
    );
    let fresh42 = segi;
    segi = segi + 1;
    root = init_query_structure(*permute.offset(fresh42 as isize), seg, tr, qs);
    i = 1 as libc::c_int;
    while i <= nseg {
        let ref mut fresh43 = (*seg.offset(i as isize)).root1;
        *fresh43 = root;
        (*seg.offset(i as isize)).root0 = *fresh43;
        i += 1;
    }
    h = 1 as libc::c_int;
    while h <= math_logstar_n(nseg) {
        i = math_N(nseg, h - 1 as libc::c_int) + 1 as libc::c_int;
        while i <= math_N(nseg, h) {
            let fresh44 = segi;
            segi = segi + 1;
            add_segment(*permute.offset(fresh44 as isize), seg, tr, qs);
            i += 1;
        }
        i = 1 as libc::c_int;
        while i <= nseg {
            find_new_roots(i, seg, tr, qs);
            i += 1;
        }
        h += 1;
    }
    i = math_N(nseg, math_logstar_n(nseg)) + 1 as libc::c_int;
    while i <= nseg {
        let fresh45 = segi;
        segi = segi + 1;
        add_segment(*permute.offset(fresh45 as isize), seg, tr, qs);
        i += 1;
    }
    free(qs as *mut libc::c_void);
    return tr_idx;
}
