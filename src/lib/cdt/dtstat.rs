#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn dtrestore(_: *mut Dt_t, _: *mut Dtlink_t) -> libc::c_int;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
pub type Dtlink_t = _dtlink_s;
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
pub type Dtdisc_t = _dtdisc_s;
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
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed_0,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtstat_s {
    pub dt_meth: libc::c_int,
    pub dt_size: libc::c_int,
    pub dt_n: libc::c_int,
    pub dt_max: libc::c_int,
    pub dt_count: *mut libc::c_int,
}
pub type Dtstat_t = _dtstat_s;
unsafe extern "C" fn dttstat(
    mut ds: *mut Dtstat_t,
    mut root: *mut Dtlink_t,
    mut depth: libc::c_int,
    mut level: *mut libc::c_int,
) {
    if !((*root).hl._left).is_null() {
        dttstat(ds, (*root).hl._left, depth + 1 as libc::c_int, level);
    }
    if !((*root).right).is_null() {
        dttstat(ds, (*root).right, depth + 1 as libc::c_int, level);
    }
    if depth > (*ds).dt_n {
        (*ds).dt_n = depth;
    }
    if !level.is_null() {
        *level.offset(depth as isize) += 1 as libc::c_int;
    }
}
unsafe extern "C" fn dthstat(
    mut data: *mut Dtdata_t,
    mut ds: *mut Dtstat_t,
    mut count: *mut libc::c_int,
) {
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut n: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    h = (*data).ntab - 1 as libc::c_int;
    while h >= 0 as libc::c_int {
        n = 0 as libc::c_int;
        t = *((*data).hh._htab).offset(h as isize);
        while !t.is_null() {
            n += 1 as libc::c_int;
            t = (*t).right;
        }
        if !count.is_null() {
            *count.offset(n as isize) += 1 as libc::c_int;
        } else if n > 0 as libc::c_int {
            (*ds).dt_n += 1 as libc::c_int;
            if n > (*ds).dt_max {
                (*ds).dt_max = n;
            }
        }
        h -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dtstat(
    mut dt: *mut Dt_t,
    mut ds: *mut Dtstat_t,
    mut all: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    static mut Count: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
    static mut Size: libc::c_int = 0;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        dtrestore(dt, 0 as *mut Dtlink_t);
    } else {};
    let ref mut fresh0 = (*ds).dt_max;
    *fresh0 = 0 as libc::c_int;
    (*ds).dt_n = *fresh0;
    let ref mut fresh1 = (*ds).dt_count;
    *fresh1 = 0 as *mut libc::c_int;
    (*ds).dt_size = dtsize(dt);
    (*ds).dt_meth = (*(*dt).data).type_0 & 0o377 as libc::c_int;
    if all == 0 {
        return 0 as libc::c_int;
    }
    if (*(*dt).data).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
        dthstat((*dt).data, ds, 0 as *mut libc::c_int);
        if (*ds).dt_max + 1 as libc::c_int > Size {
            if Size > 0 as libc::c_int {
                free(Count as *mut libc::c_void);
            }
            Count = malloc(
                (((*ds).dt_max + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            if Count.is_null() {
                return -(1 as libc::c_int);
            }
            Size = (*ds).dt_max + 1 as libc::c_int;
        }
        i = (*ds).dt_max;
        while i >= 0 as libc::c_int {
            *Count.offset(i as isize) = 0 as libc::c_int;
            i -= 1;
        }
        dthstat((*dt).data, ds, Count);
    } else if (*(*dt).data).type_0 & (0o4 as libc::c_int | 0o10 as libc::c_int) != 0 {
        if !((*(*dt).data).here).is_null() {
            dttstat(ds, (*(*dt).data).here, 0 as libc::c_int, 0 as *mut libc::c_int);
            if (*ds).dt_n + 1 as libc::c_int > Size {
                if Size > 0 as libc::c_int {
                    free(Count as *mut libc::c_void);
                }
                Count = malloc(
                    (((*ds).dt_n + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_int;
                if Count.is_null() {
                    return -(1 as libc::c_int);
                }
                Size = (*ds).dt_n + 1 as libc::c_int;
            }
            i = (*ds).dt_n;
            while i >= 0 as libc::c_int {
                *Count.offset(i as isize) = 0 as libc::c_int;
                i -= 1;
            }
            dttstat(ds, (*(*dt).data).here, 0 as libc::c_int, Count);
            i = (*ds).dt_n;
            while i >= 0 as libc::c_int {
                if *Count.offset(i as isize) > (*ds).dt_max {
                    (*ds).dt_max = *Count.offset(i as isize);
                }
                i -= 1;
            }
        }
    }
    let ref mut fresh2 = (*ds).dt_count;
    *fresh2 = Count;
    return 0 as libc::c_int;
}
