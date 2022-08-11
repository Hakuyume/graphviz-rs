#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn free(_: *mut libc::c_void);
    fn dtview(_: *mut Dt_t, _: *mut Dt_t) -> *mut Dt_t;
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
#[no_mangle]
pub unsafe extern "C" fn dtclose(mut dt: *mut Dt_t) -> libc::c_int {
    let mut disc: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
    let mut ev: libc::c_int = 0 as libc::c_int;
    if dt.is_null() || (*dt).nview > 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    disc = (*dt).disc;
    if ((*disc).eventf).is_some()
        && {
            ev = ((*disc).eventf)
                .expect(
                    "non-null function pointer",
                )(dt, 2 as libc::c_int, 0 as *mut libc::c_void, disc);
            ev < 0 as libc::c_int
        }
    {
        return -(1 as libc::c_int);
    }
    if !((*dt).view).is_null() {
        dtview(dt, 0 as *mut Dt_t);
    }
    if ev == 0 as libc::c_int {
        ((*(*dt).meth).searchf)
            .expect(
                "non-null function pointer",
            )(dt, 0 as *mut libc::c_void, 0o100 as libc::c_int);
        if dtsize(dt) > 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if (*(*dt).data).ntab > 0 as libc::c_int {
            ((*dt).memoryf)
                .expect(
                    "non-null function pointer",
                )(
                dt,
                (*(*dt).data).hh._htab as *mut libc::c_void,
                0 as libc::c_int as size_t,
                disc,
            );
        }
        ((*dt).memoryf)
            .expect(
                "non-null function pointer",
            )(dt, (*dt).data as *mut libc::c_void, 0 as libc::c_int as size_t, disc);
    }
    if (*dt).type_0 == 0 as libc::c_int {
        free(dt as *mut libc::c_void);
    } else if ev == 0 as libc::c_int && (*dt).type_0 == 1 as libc::c_int {
        ((*dt).memoryf)
            .expect(
                "non-null function pointer",
            )(dt, dt as *mut libc::c_void, 0 as libc::c_int as size_t, disc);
    }
    if ((*disc).eventf).is_some() {
        ((*disc).eventf)
            .expect(
                "non-null function pointer",
            )(dt, 6 as libc::c_int, 0 as *mut libc::c_void, disc);
    }
    return 0 as libc::c_int;
}
