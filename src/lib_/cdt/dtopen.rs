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
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn dtdisc(dt: *mut Dt_t, _: *mut Dtdisc_t, _: libc::c_int) -> *mut Dtdisc_t;
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
pub unsafe extern "C" fn dtopen(mut disc: *mut Dtdisc_t, mut meth: *mut Dtmethod_t) -> *mut Dt_t {
    let mut current_block: u64;
    let mut dt: *mut Dt_t = 0 as *mut Dt_t;
    let mut e: libc::c_int = 0;
    let mut data: *mut Dtdata_t = 0 as *mut Dtdata_t;
    if disc.is_null() || meth.is_null() {
        return 0 as *mut Dt_t;
    }
    dt = malloc(::std::mem::size_of::<Dt_t>() as libc::c_ulong) as *mut Dt_t;
    if dt.is_null() {
        return 0 as *mut Dt_t;
    }
    let ref mut fresh0 = (*dt).searchf;
    *fresh0 = None;
    let ref mut fresh1 = (*dt).meth;
    *fresh1 = 0 as *mut Dtmethod_t;
    let ref mut fresh2 = (*dt).disc;
    *fresh2 = 0 as *mut Dtdisc_t;
    dtdisc(dt, disc, 0 as libc::c_int);
    (*dt).type_0 = 0 as libc::c_int;
    (*dt).nview = 0 as libc::c_int;
    let ref mut fresh3 = (*dt).walk;
    *fresh3 = 0 as *mut Dt_t;
    let ref mut fresh4 = (*dt).view;
    *fresh4 = *fresh3;
    let ref mut fresh5 = (*dt).user;
    *fresh5 = 0 as *mut libc::c_void;
    if ((*disc).eventf).is_some() {
        data = 0 as *mut Dtdata_t;
        e = ((*disc).eventf).expect("non-null function pointer")(
            dt,
            1 as libc::c_int,
            &mut data as *mut *mut Dtdata_t as *mut libc::c_void,
            disc,
        );
        if e < 0 as libc::c_int {
            current_block = 9989543335252588407;
        } else if e > 0 as libc::c_int {
            if !data.is_null() {
                if (*data).type_0 & (*meth).type_0 != 0 {
                    current_block = 8186394046759647133;
                } else {
                    current_block = 9989543335252588407;
                }
            } else if ((*disc).memoryf).is_none() {
                current_block = 9989543335252588407;
            } else {
                free(dt as *mut libc::c_void);
                dt = ((*disc).memoryf).expect("non-null function pointer")(
                    0 as *mut Dt_t,
                    0 as *mut libc::c_void,
                    ::std::mem::size_of::<Dt_t>() as libc::c_ulong,
                    disc,
                ) as *mut Dt_t;
                if dt.is_null() {
                    return 0 as *mut Dt_t;
                }
                let ref mut fresh6 = (*dt).searchf;
                *fresh6 = None;
                let ref mut fresh7 = (*dt).meth;
                *fresh7 = 0 as *mut Dtmethod_t;
                let ref mut fresh8 = (*dt).disc;
                *fresh8 = 0 as *mut Dtdisc_t;
                dtdisc(dt, disc, 0 as libc::c_int);
                (*dt).type_0 = 1 as libc::c_int;
                (*dt).nview = 0 as libc::c_int;
                let ref mut fresh9 = (*dt).walk;
                *fresh9 = 0 as *mut Dt_t;
                let ref mut fresh10 = (*dt).view;
                *fresh10 = *fresh9;
                current_block = 14359455889292382949;
            }
        } else {
            current_block = 14359455889292382949;
        }
    } else {
        current_block = 14359455889292382949;
    }
    match current_block {
        14359455889292382949 => {
            data = ((*dt).memoryf).expect("non-null function pointer")(
                dt,
                0 as *mut libc::c_void,
                ::std::mem::size_of::<Dtdata_t>() as libc::c_ulong,
                disc,
            ) as *mut Dtdata_t;
            if data.is_null() {
                current_block = 9989543335252588407;
            } else {
                (*data).type_0 = (*meth).type_0;
                let ref mut fresh11 = (*data).here;
                *fresh11 = 0 as *mut Dtlink_t;
                let ref mut fresh12 = (*data).hh._htab;
                *fresh12 = 0 as *mut *mut Dtlink_t;
                let ref mut fresh13 = (*data).loop_0;
                *fresh13 = 0 as libc::c_int;
                let ref mut fresh14 = (*data).size;
                *fresh14 = *fresh13;
                (*data).ntab = *fresh14;
                (*data).minp = 0 as libc::c_int;
                current_block = 8186394046759647133;
            }
        }
        _ => {}
    }
    match current_block {
        8186394046759647133 => {
            let ref mut fresh15 = (*dt).data;
            *fresh15 = data;
            let ref mut fresh16 = (*dt).searchf;
            *fresh16 = (*meth).searchf;
            let ref mut fresh17 = (*dt).meth;
            *fresh17 = meth;
            if ((*disc).eventf).is_some() {
                (Some(((*disc).eventf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    dt,
                    5 as libc::c_int,
                    dt as *mut libc::c_void,
                    disc,
                );
            }
            return dt;
        }
        _ => {
            free(dt as *mut libc::c_void);
            return 0 as *mut Dt_t;
        }
    };
}
