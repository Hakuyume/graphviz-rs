#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn dtrestore(_: *mut Dt_t, _: *mut Dtlink_t) -> libc::c_int;
    fn dtstrhash(_: libc::c_uint, _: *mut libc::c_void, _: libc::c_int) -> libc::c_uint;
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
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
unsafe extern "C" fn dtmemory(
    mut dt: *mut Dt_t,
    mut addr: *mut libc::c_void,
    mut size: size_t,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    if !addr.is_null() {
        if size == 0 as libc::c_int as libc::c_ulong {
            free(addr);
            return 0 as *mut libc::c_void;
        } else {
            return realloc(addr, size)
        }
    } else {
        return if size > 0 as libc::c_int as libc::c_ulong {
            malloc(size)
        } else {
            0 as *mut libc::c_void
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dtdisc(
    mut dt: *mut Dt_t,
    mut disc: *mut Dtdisc_t,
    mut type_0: libc::c_int,
) -> *mut Dtdisc_t {
    let mut current_block: u64;
    let mut searchf: Dtsearch_f = None;
    let mut r: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
    old = (*dt).disc;
    if old.is_null() {
        let ref mut fresh0 = (*dt).disc;
        *fresh0 = disc;
        let ref mut fresh1 = (*dt).memoryf;
        *fresh1 = (*disc).memoryf;
        if (*fresh1).is_none() {
            let ref mut fresh2 = (*dt).memoryf;
            *fresh2 = Some(
                dtmemory
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        size_t,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
            );
        }
        return disc;
    }
    if disc.is_null() {
        return old;
    }
    searchf = (*(*dt).meth).searchf;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        dtrestore(dt, 0 as *mut Dtlink_t);
    } else {};
    if ((*old).eventf).is_some()
        && ((*old).eventf)
            .expect(
                "non-null function pointer",
            )(dt, 3 as libc::c_int, disc as *mut libc::c_void, old) < 0 as libc::c_int
    {
        return 0 as *mut Dtdisc_t;
    }
    let ref mut fresh3 = (*dt).disc;
    *fresh3 = disc;
    let ref mut fresh4 = (*dt).memoryf;
    *fresh4 = (*disc).memoryf;
    if (*fresh4).is_none() {
        let ref mut fresh5 = (*dt).memoryf;
        *fresh5 = Some(
            dtmemory
                as unsafe extern "C" fn(
                    *mut Dt_t,
                    *mut libc::c_void,
                    size_t,
                    *mut Dtdisc_t,
                ) -> *mut libc::c_void,
        );
    }
    if !((*(*dt).data).type_0
        & (0o40 as libc::c_int | 0o100 as libc::c_int | 0o20 as libc::c_int) != 0)
    {
        if (*(*dt).data).type_0 & 0o2 as libc::c_int != 0 {
            if type_0 & 0o2 as libc::c_int != 0 {
                current_block = 3174334154968278585;
            } else {
                current_block = 2566379501317479181;
            }
        } else if (*(*dt).data).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            if type_0 & 0o2 as libc::c_int != 0 && type_0 & 0o1 as libc::c_int != 0 {
                current_block = 3174334154968278585;
            } else {
                current_block = 2566379501317479181;
            }
        } else if type_0 & 0o1 as libc::c_int != 0 {
            current_block = 3174334154968278585;
        } else {
            current_block = 2566379501317479181;
        }
        match current_block {
            3174334154968278585 => {}
            _ => {
                r = dtflatten(dt);
                (*(*dt).data).type_0 &= !(0o10000 as libc::c_int);
                let ref mut fresh6 = (*(*dt).data).here;
                *fresh6 = 0 as *mut Dtlink_t;
                (*(*dt).data).size = 0 as libc::c_int;
                if (*(*dt).data).type_0 & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0
                {
                    let mut s: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
                    let mut ends: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
                    s = (*(*dt).data).hh._htab;
                    ends = s.offset((*(*dt).data).ntab as isize);
                    while s < ends {
                        let fresh7 = s;
                        s = s.offset(1);
                        *fresh7 = 0 as *mut Dtlink_t;
                    }
                }
                while !r.is_null() {
                    t = (*r).right;
                    if type_0 & 0o2 as libc::c_int == 0 {
                        k = (if (*disc).link < 0 as libc::c_int {
                            (*(r as *mut Dthold_t)).obj
                        } else {
                            (r as *mut libc::c_char).offset(-((*disc).link as isize))
                                as *mut libc::c_void
                        }) as *mut libc::c_char;
                        k = (if (*disc).size < 0 as libc::c_int {
                            *(k.offset((*disc).key as isize) as *mut *mut libc::c_char)
                        } else {
                            k.offset((*disc).key as isize)
                        }) as *mut libc::c_void as *mut libc::c_char;
                        (*r)
                            .hl
                            ._hash = if ((*disc).hashf).is_some() {
                            (Some(((*disc).hashf).expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(dt, k as *mut libc::c_void, disc)
                        } else {
                            dtstrhash(
                                0 as libc::c_int as libc::c_uint,
                                k as *mut libc::c_void,
                                (*disc).size,
                            )
                        };
                    }
                    searchf
                        .expect(
                            "non-null function pointer",
                        )(dt, r as *mut libc::c_void, 0o40 as libc::c_int);
                    r = t;
                }
            }
        }
    }
    return old;
}
