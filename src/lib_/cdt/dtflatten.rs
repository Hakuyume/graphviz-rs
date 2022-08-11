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
#![feature(register_tool)]
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
pub type Dtevent_f = Option<
    unsafe extern "C" fn(*mut Dt_t, libc::c_int, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_int,
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
pub type Dtsearch_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void>;
pub type Dtmemory_f = Option<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, size_t, *mut Dtdisc_t) -> *mut libc::c_void,
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
pub type Dthash_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_uint>;
pub type Dtcompar_f = Option<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dtfree_f = Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> ()>;
pub type Dtmake_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> *mut libc::c_void>;
#[no_mangle]
pub unsafe extern "C" fn dtflatten(mut dt: *mut Dt_t) -> *mut Dtlink_t {
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut r: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut list: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut last: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut s: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut ends: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        return (*(*dt).data).here;
    }
    last = 0 as *mut Dtlink_t;
    list = last;
    if (*(*dt).data).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
        s = (*(*dt).data).hh._htab;
        ends = s.offset((*(*dt).data).ntab as isize);
        while s < ends {
            t = *s;
            if !t.is_null() {
                if !last.is_null() {
                    let ref mut fresh0 = (*last).right;
                    *fresh0 = t;
                } else {
                    last = t;
                    list = last;
                }
                while !((*last).right).is_null() {
                    last = (*last).right;
                }
                *s = last;
            }
            s = s.offset(1);
        }
    } else if (*(*dt).data).type_0
        & (0o20 as libc::c_int | 0o40 as libc::c_int | 0o100 as libc::c_int)
        != 0
    {
        list = (*(*dt).data).hh._head;
    } else {
        r = (*(*dt).data).here;
        if !r.is_null() {
            loop {
                t = (*r).hl._left;
                if t.is_null() {
                    break;
                }
                let ref mut fresh1 = (*r).hl._left;
                *fresh1 = (*t).right;
                let ref mut fresh2 = (*t).right;
                *fresh2 = r;
                r = t;
            }
            last = r;
            list = last;
            r = (*r).right;
            while !r.is_null() {
                t = (*r).hl._left;
                if !t.is_null() {
                    loop {
                        let ref mut fresh3 = (*r).hl._left;
                        *fresh3 = (*t).right;
                        let ref mut fresh4 = (*t).right;
                        *fresh4 = r;
                        r = t;
                        t = (*r).hl._left;
                        if t.is_null() {
                            break;
                        }
                    }
                    let ref mut fresh5 = (*last).right;
                    *fresh5 = r;
                }
                last = r;
                r = (*r).right;
            }
        }
    }
    let ref mut fresh6 = (*(*dt).data).here;
    *fresh6 = list;
    (*(*dt).data).type_0 |= 0o10000 as libc::c_int;
    return list;
}
