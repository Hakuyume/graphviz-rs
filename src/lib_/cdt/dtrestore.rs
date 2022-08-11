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
pub unsafe extern "C" fn dtrestore(mut dt: *mut Dt_t, mut list: *mut Dtlink_t) -> libc::c_int {
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut s: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut ends: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut type_0: libc::c_int = 0;
    let mut searchf: Dtsearch_f = (*(*dt).meth).searchf;
    type_0 = (*(*dt).data).type_0 & 0o10000 as libc::c_int;
    if list.is_null() {
        if type_0 == 0 {
            return -(1 as libc::c_int);
        }
        list = (*(*dt).data).here;
    } else {
        if (*(*dt).data).size != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        type_0 = 0 as libc::c_int;
    }
    (*(*dt).data).type_0 &= !(0o10000 as libc::c_int);
    if (*(*dt).data).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
        let ref mut fresh0 = (*(*dt).data).here;
        *fresh0 = 0 as *mut Dtlink_t;
        if type_0 != 0 {
            s = (*(*dt).data).hh._htab;
            ends = s.offset((*(*dt).data).ntab as isize);
            while s < ends {
                t = *s;
                if !t.is_null() {
                    *s = list;
                    list = (*t).right;
                    let ref mut fresh1 = (*t).right;
                    *fresh1 = 0 as *mut Dtlink_t;
                }
                s = s.offset(1);
            }
        } else {
            (*(*dt).data).size = 0 as libc::c_int;
            while !list.is_null() {
                t = (*list).right;
                searchf.expect("non-null function pointer")(
                    dt,
                    list as *mut libc::c_void,
                    0o40 as libc::c_int,
                );
                list = t;
            }
        }
    } else {
        if (*(*dt).data).type_0 & (0o4 as libc::c_int | 0o10 as libc::c_int) != 0 {
            let ref mut fresh2 = (*(*dt).data).here;
            *fresh2 = list;
        } else {
            let ref mut fresh3 = (*(*dt).data).here;
            *fresh3 = 0 as *mut Dtlink_t;
            let ref mut fresh4 = (*(*dt).data).hh._head;
            *fresh4 = list;
        }
        if type_0 == 0 {
            (*(*dt).data).size = -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
