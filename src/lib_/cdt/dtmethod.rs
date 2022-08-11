#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
    fn dtstrhash(_: libc::c_uint, _: *mut libc::c_void, _: libc::c_int) -> libc::c_uint;
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
pub struct _dthold_s {
    pub hdr: Dtlink_t,
    pub obj: *mut libc::c_void,
}
pub type Dthold_t = _dthold_s;
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
pub unsafe extern "C" fn dtmethod(
    mut dt: *mut Dt_t,
    mut meth: *mut Dtmethod_t,
) -> *mut Dtmethod_t {
    let mut list: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut r: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut disc: *mut Dtdisc_t = (*dt).disc;
    let mut oldmeth: *mut Dtmethod_t = (*dt).meth;
    if meth.is_null() || (*meth).type_0 == (*oldmeth).type_0 {
        return oldmeth;
    }
    if ((*disc).eventf).is_some()
        && ((*disc).eventf)
            .expect(
                "non-null function pointer",
            )(dt, 4 as libc::c_int, meth as *mut libc::c_void, disc) < 0 as libc::c_int
    {
        return 0 as *mut Dtmethod_t;
    }
    (*(*dt).data).minp = 0 as libc::c_int;
    list = dtflatten(dt);
    if (*(*dt).data).type_0
        & (0o20 as libc::c_int | 0o40 as libc::c_int | 0o100 as libc::c_int) != 0
    {
        let ref mut fresh0 = (*(*dt).data).hh._head;
        *fresh0 = 0 as *mut Dtlink_t;
    } else if (*(*dt).data).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
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
        (*(*dt).data).ntab = 0 as libc::c_int;
        let ref mut fresh1 = (*(*dt).data).hh._htab;
        *fresh1 = 0 as *mut *mut Dtlink_t;
    }
    let ref mut fresh2 = (*(*dt).data).here;
    *fresh2 = 0 as *mut Dtlink_t;
    (*(*dt).data)
        .type_0 = (*(*dt).data).type_0 & !(0o377 as libc::c_int | 0o10000 as libc::c_int)
        | (*meth).type_0;
    let ref mut fresh3 = (*dt).meth;
    *fresh3 = meth;
    if (*dt).searchf == (*oldmeth).searchf {
        let ref mut fresh4 = (*dt).searchf;
        *fresh4 = (*meth).searchf;
    }
    if (*meth).type_0
        & (0o20 as libc::c_int | 0o40 as libc::c_int | 0o100 as libc::c_int) != 0
    {
        if (*oldmeth).type_0
            & (0o20 as libc::c_int | 0o40 as libc::c_int | 0o100 as libc::c_int) == 0
        {
            r = list;
            if !r.is_null() {
                let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
                t = (*r).right;
                while !t.is_null() {
                    let ref mut fresh5 = (*t).hl._left;
                    *fresh5 = r;
                    r = t;
                    t = (*t).right;
                }
                let ref mut fresh6 = (*list).hl._left;
                *fresh6 = r;
            }
        }
        let ref mut fresh7 = (*(*dt).data).hh._head;
        *fresh7 = list;
    } else if (*meth).type_0 & (0o4 as libc::c_int | 0o10 as libc::c_int) != 0 {
        (*(*dt).data).size = 0 as libc::c_int;
        while !list.is_null() {
            r = (*list).right;
            ((*meth).searchf)
                .expect(
                    "non-null function pointer",
                )(dt, list as *mut libc::c_void, 0o40 as libc::c_int);
            list = r;
        }
    } else if !((*meth).type_0 & 0o2 as libc::c_int != 0
            && (*oldmeth).type_0 & 0o1 as libc::c_int != 0)
        {
        let mut rehash: libc::c_int = 0;
        if (*meth).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0
            && (*oldmeth).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) == 0
        {
            rehash = 1 as libc::c_int;
        } else {
            rehash = 0 as libc::c_int;
        }
        let ref mut fresh8 = (*(*dt).data).loop_0;
        *fresh8 = 0 as libc::c_int;
        (*(*dt).data).size = *fresh8;
        while !list.is_null() {
            r = (*list).right;
            if rehash != 0 {
                let mut key: *mut libc::c_void = if (*disc).link < 0 as libc::c_int {
                    (*(list as *mut Dthold_t)).obj
                } else {
                    (list as *mut libc::c_char).offset(-((*disc).link as isize))
                        as *mut libc::c_void
                };
                key = (if (*disc).size < 0 as libc::c_int {
                    *((key as *mut libc::c_char).offset((*disc).key as isize)
                        as *mut *mut libc::c_char)
                } else {
                    (key as *mut libc::c_char).offset((*disc).key as isize)
                }) as *mut libc::c_void;
                (*list)
                    .hl
                    ._hash = if ((*disc).hashf).is_some() {
                    (Some(((*disc).hashf).expect("non-null function pointer")))
                        .expect("non-null function pointer")(dt, key, disc)
                } else {
                    dtstrhash(0 as libc::c_int as libc::c_uint, key, (*disc).size)
                };
            }
            ((*meth).searchf)
                .expect(
                    "non-null function pointer",
                )(dt, list as *mut libc::c_void, 0o40 as libc::c_int);
            list = r;
        }
    }
    return oldmeth;
}
