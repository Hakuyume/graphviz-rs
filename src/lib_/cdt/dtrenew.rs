#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn dtstrhash(_: libc::c_uint, _: *mut libc::c_void, _: libc::c_int) -> libc::c_uint;
    fn dtrestore(_: *mut Dt_t, _: *mut Dtlink_t) -> libc::c_int;
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
pub unsafe extern "C" fn dtrenew(
    mut dt: *mut Dt_t,
    mut obj: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut e: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut s: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut disc: *mut Dtdisc_t = (*dt).disc;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        dtrestore(dt, 0 as *mut Dtlink_t);
    } else {};
    e = (*(*dt).data).here;
    if e.is_null()
        || (if (*disc).link < 0 as libc::c_int {
            (*(e as *mut Dthold_t)).obj
        } else {
            (e as *mut libc::c_char).offset(-((*disc).link as isize))
                as *mut libc::c_void
        }) != obj
    {
        return 0 as *mut libc::c_void;
    }
    if (*(*dt).data).type_0
        & (0o40 as libc::c_int | 0o100 as libc::c_int | 0o20 as libc::c_int) != 0
    {
        return obj
    } else {
        if (*(*dt).data).type_0 & (0o4 as libc::c_int | 0o10 as libc::c_int) != 0 {
            if ((*e).right).is_null() {
                let ref mut fresh0 = (*(*dt).data).here;
                *fresh0 = (*e).hl._left;
            } else {
                let ref mut fresh1 = (*(*dt).data).here;
                *fresh1 = (*e).right;
                if !((*e).hl._left).is_null() {
                    t = (*e).right;
                    while !((*t).hl._left).is_null() {
                        t = (*t).hl._left;
                    }
                    let ref mut fresh2 = (*t).hl._left;
                    *fresh2 = (*e).hl._left;
                }
            }
        } else {
            s = ((*(*dt).data).hh._htab)
                .offset(
                    ((*e).hl._hash
                        & ((*(*dt).data).ntab - 1 as libc::c_int) as libc::c_uint)
                        as isize,
                );
            t = *s;
            if t == e {
                *s = (*e).right;
            } else {
                while (*t).right != e {
                    t = (*t).right;
                }
                let ref mut fresh3 = (*t).right;
                *fresh3 = (*e).right;
            }
            key = (if (*disc).size < 0 as libc::c_int {
                *((obj as *mut libc::c_char).offset((*disc).key as isize)
                    as *mut *mut libc::c_char)
            } else {
                (obj as *mut libc::c_char).offset((*disc).key as isize)
            }) as *mut libc::c_void;
            (*e)
                .hl
                ._hash = if ((*disc).hashf).is_some() {
                (Some(((*disc).hashf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(dt, key, disc)
            } else {
                dtstrhash(0 as libc::c_int as libc::c_uint, key, (*disc).size)
            };
            let ref mut fresh4 = (*(*dt).data).here;
            *fresh4 = 0 as *mut Dtlink_t;
        }
    }
    (*(*dt).data).size -= 1 as libc::c_int;
    return if !(((*(*dt).meth).searchf)
        .expect(
            "non-null function pointer",
        )(dt, e as *mut libc::c_void, 0o40 as libc::c_int))
        .is_null()
    {
        obj
    } else {
        0 as *mut libc::c_void
    };
}
