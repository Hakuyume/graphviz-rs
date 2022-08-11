#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
unsafe extern "C" fn dthtab(mut dt: *mut Dt_t) {
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut r: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut p: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut s: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut hs: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut is: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut olds: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*(*dt).data).minp > 0 as libc::c_int && (*(*dt).data).ntab > 0 as libc::c_int {
        return;
    }
    (*(*dt).data).minp = 0 as libc::c_int;
    n = (*(*dt).data).ntab;
    if !((*dt).disc).is_null() && ((*(*dt).disc).eventf).is_some()
        && ((*(*dt).disc).eventf)
            .expect(
                "non-null function pointer",
            )(
            dt,
            7 as libc::c_int,
            &mut n as *mut libc::c_int as *mut libc::c_void,
            (*dt).disc,
        ) > 0 as libc::c_int
    {
        if n < 0 as libc::c_int {
            (*(*dt).data).minp = 1 as libc::c_int;
            if (*(*dt).data).ntab > 0 as libc::c_int {
                return;
            }
        } else {
            k = 2 as libc::c_int;
            while k < n {
                k *= 2 as libc::c_int;
            }
            n = k;
        }
    } else {
        n = 0 as libc::c_int;
    }
    if n <= 0 as libc::c_int {
        n = (*(*dt).data).ntab;
        if n == 0 as libc::c_int {
            n = 256 as libc::c_int;
        }
        while (*(*dt).data).size > n << 1 as libc::c_int {
            n = n << 1 as libc::c_int;
        }
    }
    if n == (*(*dt).data).ntab {
        return;
    }
    olds = if (*(*dt).data).ntab == 0 as libc::c_int {
        0 as *mut *mut Dtlink_t
    } else {
        (*(*dt).data).hh._htab
    };
    s = ((*dt).memoryf)
        .expect(
            "non-null function pointer",
        )(
        dt,
        olds as *mut libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut Dtlink_t>() as libc::c_ulong),
        (*dt).disc,
    ) as *mut *mut Dtlink_t;
    if s.is_null() {
        return;
    }
    olds = s.offset((*(*dt).data).ntab as isize);
    let ref mut fresh0 = (*(*dt).data).hh._htab;
    *fresh0 = s;
    (*(*dt).data).ntab = n;
    hs = s.offset(n as isize).offset(-(1 as libc::c_int as isize));
    while hs >= olds {
        *hs = 0 as *mut Dtlink_t;
        hs = hs.offset(-1);
    }
    hs = s;
    while hs < olds {
        p = 0 as *mut Dtlink_t;
        t = *hs;
        while !t.is_null() {
            r = (*t).right;
            is = s
                .offset(
                    ((*t).hl._hash & (n - 1 as libc::c_int) as libc::c_uint) as isize,
                );
            if is == hs {
                p = t;
            } else {
                if !p.is_null() {
                    let ref mut fresh1 = (*p).right;
                    *fresh1 = r;
                } else {
                    *hs = r;
                }
                let ref mut fresh2 = (*t).right;
                *fresh2 = *is;
                *is = t;
            }
            t = r;
        }
        hs = hs.offset(1);
    }
}
unsafe extern "C" fn dthash(
    mut dt: *mut Dt_t,
    mut obj: *mut libc::c_void,
    mut type_0: libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut r: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut p: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut k: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut hsh: libc::c_uint = 0;
    let mut lk: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut ky: libc::c_int = 0;
    let mut cmpf: Dtcompar_f = None;
    let mut disc: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
    let mut s: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    let mut ends: *mut *mut Dtlink_t = 0 as *mut *mut Dtlink_t;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        dtrestore(dt, 0 as *mut Dtlink_t);
    } else {};
    disc = (*dt).disc;
    ky = (*disc).key;
    sz = (*disc).size;
    lk = (*disc).link;
    cmpf = (*disc).comparf;
    (*dt).type_0 &= !(0o100000 as libc::c_int);
    if obj.is_null() {
        if !(type_0 & (0o10 as libc::c_int | 0o20 as libc::c_int) != 0) {
            if (*(*dt).data).size <= 0 as libc::c_int
                || type_0
                    & (0o100 as libc::c_int | 0o200 as libc::c_int
                        | 0o400 as libc::c_int) == 0
            {
                return 0 as *mut libc::c_void;
            }
            s = (*(*dt).data).hh._htab;
            ends = s.offset((*(*dt).data).ntab as isize);
            if type_0 & 0o100 as libc::c_int != 0 {
                while s < ends {
                    t = *s;
                    *s = 0 as *mut Dtlink_t;
                    if !(((*disc).freef).is_none() && (*disc).link >= 0 as libc::c_int) {
                        while !t.is_null() {
                            r = (*t).right;
                            if ((*disc).freef).is_some() {
                                ((*disc).freef)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dt,
                                    if lk < 0 as libc::c_int {
                                        (*(t as *mut Dthold_t)).obj
                                    } else {
                                        (t as *mut libc::c_char).offset(-(lk as isize))
                                            as *mut libc::c_void
                                    },
                                    disc,
                                );
                            }
                            if (*disc).link < 0 as libc::c_int {
                                ((*dt).memoryf)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dt,
                                    t as *mut libc::c_void,
                                    0 as libc::c_int as size_t,
                                    disc,
                                );
                            }
                            t = r;
                        }
                    }
                    s = s.offset(1);
                }
                let ref mut fresh3 = (*(*dt).data).here;
                *fresh3 = 0 as *mut Dtlink_t;
                (*(*dt).data).size = 0 as libc::c_int;
                (*(*dt).data).loop_0 = 0 as libc::c_int;
                return 0 as *mut libc::c_void;
            } else {
                t = 0 as *mut Dtlink_t;
                while s < ends && t.is_null() {
                    t = if type_0 & 0o400 as libc::c_int != 0 {
                        ends = ends.offset(-1);
                        *ends
                    } else {
                        let fresh4 = s;
                        s = s.offset(1);
                        *fresh4
                    };
                }
                if !t.is_null() && type_0 & 0o400 as libc::c_int != 0 {
                    while !((*t).right).is_null() {
                        t = (*t).right;
                    }
                }
                (*(*dt).data).loop_0 += 1 as libc::c_int;
                let ref mut fresh5 = (*(*dt).data).here;
                *fresh5 = t;
                return if !t.is_null() {
                    if lk < 0 as libc::c_int {
                        (*(t as *mut Dthold_t)).obj
                    } else {
                        (t as *mut libc::c_char).offset(-(lk as isize))
                            as *mut libc::c_void
                    }
                } else {
                    0 as *mut libc::c_void
                };
            }
        }
    } else {
        if (*(*dt).meth).type_0 == 0o2 as libc::c_int
            && type_0 & (0o2 as libc::c_int | 0o10000 as libc::c_int) != 0
        {
            if ((Some(((*dt).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(dt, obj, 0o4 as libc::c_int))
                .is_null()
            {
                return 0 as *mut libc::c_void;
            }
            s = ((*(*dt).data).hh._htab)
                .offset(
                    ((*(*(*dt).data).here).hl._hash
                        & ((*(*dt).data).ntab - 1 as libc::c_int) as libc::c_uint)
                        as isize,
                );
            r = 0 as *mut Dtlink_t;
            p = 0 as *mut Dtlink_t;
            t = *s;
            loop {
                if t.is_null() {
                    current_block = 2989495919056355252;
                    break;
                }
                if (if lk < 0 as libc::c_int {
                    (*(t as *mut Dthold_t)).obj
                } else {
                    (t as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                }) == obj
                {
                    current_block = 6658210076141074664;
                    break;
                } else {
                    if t == (*(*dt).data).here {
                        r = p;
                    }
                    p = t;
                    t = (*t).right;
                }
            }
            match current_block {
                6658210076141074664 => {}
                _ => {
                    p = r;
                    t = (*(*dt).data).here;
                    current_block = 6658210076141074664;
                }
            }
        } else {
            if type_0
                & (0o1000 as libc::c_int | 0o4 as libc::c_int | 0o1 as libc::c_int
                    | 0o4000 as libc::c_int) != 0
            {
                key = if type_0 & 0o1000 as libc::c_int != 0 {
                    obj
                } else {
                    (if sz < 0 as libc::c_int {
                        *((obj as *mut libc::c_char).offset(ky as isize)
                            as *mut *mut libc::c_char)
                    } else {
                        (obj as *mut libc::c_char).offset(ky as isize)
                    }) as *mut libc::c_void
                };
                hsh = if ((*disc).hashf).is_some() {
                    (Some(((*disc).hashf).expect("non-null function pointer")))
                        .expect("non-null function pointer")(dt, key, disc)
                } else {
                    dtstrhash(0 as libc::c_int as libc::c_uint, key, sz)
                };
                current_block = 13246848547199022064;
            } else if type_0 & (0o40 as libc::c_int | 0o2000 as libc::c_int) != 0 {
                r = obj as *mut Dtlink_t;
                obj = if lk < 0 as libc::c_int {
                    (*(r as *mut Dthold_t)).obj
                } else {
                    (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                };
                key = (if sz < 0 as libc::c_int {
                    *((obj as *mut libc::c_char).offset(ky as isize)
                        as *mut *mut libc::c_char)
                } else {
                    (obj as *mut libc::c_char).offset(ky as isize)
                }) as *mut libc::c_void;
                hsh = (*r).hl._hash;
                current_block = 13246848547199022064;
            } else {
                t = (*(*dt).data).here;
                if !t.is_null()
                    && (if lk < 0 as libc::c_int {
                        (*(t as *mut Dthold_t)).obj
                    } else {
                        (t as *mut libc::c_char).offset(-(lk as isize))
                            as *mut libc::c_void
                    }) == obj
                {
                    hsh = (*t).hl._hash;
                    s = ((*(*dt).data).hh._htab)
                        .offset(
                            (hsh
                                & ((*(*dt).data).ntab - 1 as libc::c_int) as libc::c_uint)
                                as isize,
                        );
                    p = 0 as *mut Dtlink_t;
                    current_block = 3736434875406665187;
                } else {
                    key = (if sz < 0 as libc::c_int {
                        *((obj as *mut libc::c_char).offset(ky as isize)
                            as *mut *mut libc::c_char)
                    } else {
                        (obj as *mut libc::c_char).offset(ky as isize)
                    }) as *mut libc::c_void;
                    hsh = if ((*disc).hashf).is_some() {
                        (Some(((*disc).hashf).expect("non-null function pointer")))
                            .expect("non-null function pointer")(dt, key, disc)
                    } else {
                        dtstrhash(0 as libc::c_int as libc::c_uint, key, sz)
                    };
                    current_block = 13246848547199022064;
                }
            }
            match current_block {
                13246848547199022064 => {
                    t = if (*(*dt).data).ntab <= 0 as libc::c_int {
                        0 as *mut Dtlink_t
                    } else {
                        s = ((*(*dt).data).hh._htab)
                            .offset(
                                (hsh
                                    & ((*(*dt).data).ntab - 1 as libc::c_int) as libc::c_uint)
                                    as isize,
                            );
                        *s
                    };
                    p = 0 as *mut Dtlink_t;
                    while !t.is_null() {
                        if hsh == (*t).hl._hash {
                            k = if lk < 0 as libc::c_int {
                                (*(t as *mut Dthold_t)).obj
                            } else {
                                (t as *mut libc::c_char).offset(-(lk as isize))
                                    as *mut libc::c_void
                            };
                            k = (if sz < 0 as libc::c_int {
                                *((k as *mut libc::c_char).offset(ky as isize)
                                    as *mut *mut libc::c_char)
                            } else {
                                (k as *mut libc::c_char).offset(ky as isize)
                            }) as *mut libc::c_void;
                            if (if cmpf.is_some() {
                                (Some(cmpf.expect("non-null function pointer")))
                                    .expect("non-null function pointer")(dt, key, k, disc)
                            } else {
                                (if sz <= 0 as libc::c_int {
                                    strcmp(key as *const libc::c_char, k as *const libc::c_char)
                                } else {
                                    memcmp(key, k, sz as size_t)
                                })
                            }) == 0 as libc::c_int
                            {
                                break;
                            }
                        }
                        p = t;
                        t = (*t).right;
                    }
                }
                _ => {}
            }
            if !t.is_null() {
                (*dt).type_0 |= 0o100000 as libc::c_int;
            }
            if type_0
                & (0o1000 as libc::c_int | 0o4 as libc::c_int | 0o2000 as libc::c_int)
                != 0
            {
                if t.is_null() {
                    return 0 as *mut libc::c_void;
                }
                if !p.is_null() && (*(*dt).data).type_0 & 0o1 as libc::c_int != 0
                    && (*(*dt).data).loop_0 <= 0 as libc::c_int
                {
                    let ref mut fresh6 = (*p).right;
                    *fresh6 = (*t).right;
                    let ref mut fresh7 = (*t).right;
                    *fresh7 = *s;
                    *s = t;
                }
                let ref mut fresh8 = (*(*dt).data).here;
                *fresh8 = t;
                return if lk < 0 as libc::c_int {
                    (*(t as *mut Dthold_t)).obj
                } else {
                    (t as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                };
            } else {
                if type_0 & (0o1 as libc::c_int | 0o4000 as libc::c_int) != 0 {
                    if !t.is_null() && (*(*dt).data).type_0 & 0o1 as libc::c_int != 0 {
                        let ref mut fresh9 = (*(*dt).data).here;
                        *fresh9 = t;
                        return if lk < 0 as libc::c_int {
                            (*(t as *mut Dthold_t)).obj
                        } else {
                            (t as *mut libc::c_char).offset(-(lk as isize))
                                as *mut libc::c_void
                        };
                    }
                    if ((*disc).makef).is_some() && type_0 & 0o1 as libc::c_int != 0
                        && {
                            obj = ((*disc).makef)
                                .expect("non-null function pointer")(dt, obj, disc);
                            obj.is_null()
                        }
                    {
                        return 0 as *mut libc::c_void;
                    }
                    if lk >= 0 as libc::c_int {
                        r = (obj as *mut libc::c_char).offset(lk as isize)
                            as *mut Dtlink_t;
                    } else {
                        r = ((*dt).memoryf)
                            .expect(
                                "non-null function pointer",
                            )(
                            dt,
                            0 as *mut libc::c_void,
                            ::std::mem::size_of::<Dthold_t>() as libc::c_ulong,
                            disc,
                        ) as *mut Dtlink_t;
                        if !r.is_null() {
                            let ref mut fresh10 = (*(r as *mut Dthold_t)).obj;
                            *fresh10 = obj;
                        } else {
                            if ((*disc).makef).is_some() && ((*disc).freef).is_some()
                                && type_0 & 0o1 as libc::c_int != 0
                            {
                                ((*disc).freef)
                                    .expect("non-null function pointer")(dt, obj, disc);
                            }
                            return 0 as *mut libc::c_void;
                        }
                    }
                    (*r).hl._hash = hsh;
                    current_block = 6082913985297018206;
                } else {
                    if type_0 & 0o10 as libc::c_int != 0 {
                        if !t.is_null()
                            && {
                                p = (*t).right;
                                p.is_null()
                            }
                        {
                            ends = ((*(*dt).data).hh._htab)
                                .offset((*(*dt).data).ntab as isize);
                            s = s.offset(1 as libc::c_int as isize);
                            while s < ends {
                                p = *s;
                                if !p.is_null() {
                                    break;
                                }
                                s = s.offset(1);
                            }
                        }
                        current_block = 16908778556176415536;
                    } else if type_0 & 0o20 as libc::c_int != 0 {
                        if !t.is_null() && p.is_null() {
                            p = *s;
                            if p != t {
                                while (*p).right != t {
                                    p = (*p).right;
                                }
                            } else {
                                p = 0 as *mut Dtlink_t;
                                s = s.offset(-(1 as libc::c_int as isize));
                                ends = (*(*dt).data).hh._htab;
                                while s >= ends {
                                    p = *s;
                                    if !p.is_null() {
                                        while !((*p).right).is_null() {
                                            p = (*p).right;
                                        }
                                        break;
                                    } else {
                                        s = s.offset(-1);
                                    }
                                }
                            }
                        }
                        current_block = 16908778556176415536;
                    } else if type_0 & 0o40 as libc::c_int != 0 {
                        if t.is_null() || (*(*dt).data).type_0 & 0o2 as libc::c_int != 0
                        {
                            current_block = 6082913985297018206;
                        } else {
                            if ((*disc).freef).is_some() {
                                ((*disc).freef)
                                    .expect("non-null function pointer")(dt, obj, disc);
                            }
                            if (*disc).link < 0 as libc::c_int {
                                ((*dt).memoryf)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dt,
                                    r as *mut libc::c_void,
                                    0 as libc::c_int as size_t,
                                    disc,
                                );
                            }
                            return if !t.is_null() {
                                if lk < 0 as libc::c_int {
                                    (*(t as *mut Dthold_t)).obj
                                } else {
                                    (t as *mut libc::c_char).offset(-(lk as isize))
                                        as *mut libc::c_void
                                }
                            } else {
                                0 as *mut libc::c_void
                            };
                        }
                    } else {
                        current_block = 6658210076141074664;
                    }
                    match current_block {
                        6658210076141074664 => {}
                        6082913985297018206 => {}
                        _ => {
                            let ref mut fresh16 = (*(*dt).data).here;
                            *fresh16 = p;
                            if (*fresh16).is_null() {
                                current_block = 8164104897091980844;
                            } else {
                                (*(*dt).data).type_0 |= 0o20000 as libc::c_int;
                                return if lk < 0 as libc::c_int {
                                    (*(p as *mut Dthold_t)).obj
                                } else {
                                    (p as *mut libc::c_char).offset(-(lk as isize))
                                        as *mut libc::c_void
                                };
                            }
                        }
                    }
                }
                match current_block {
                    6658210076141074664 => {}
                    8164104897091980844 => {}
                    _ => {
                        let ref mut fresh11 = (*(*dt).data).size;
                        *fresh11 += 1 as libc::c_int;
                        if *fresh11 > (*(*dt).data).ntab << 1 as libc::c_int
                            && (*(*dt).data).loop_0 <= 0 as libc::c_int
                        {
                            dthtab(dt);
                        }
                        if (*(*dt).data).ntab == 0 as libc::c_int {
                            (*(*dt).data).size -= 1 as libc::c_int;
                            if ((*disc).freef).is_some()
                                && type_0 & 0o1 as libc::c_int != 0
                            {
                                ((*disc).freef)
                                    .expect("non-null function pointer")(dt, obj, disc);
                            }
                            if (*disc).link < 0 as libc::c_int {
                                ((*disc).memoryf)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dt,
                                    r as *mut libc::c_void,
                                    0 as libc::c_int as size_t,
                                    disc,
                                );
                            }
                            return 0 as *mut libc::c_void;
                        }
                        s = ((*(*dt).data).hh._htab)
                            .offset(
                                (hsh
                                    & ((*(*dt).data).ntab - 1 as libc::c_int) as libc::c_uint)
                                    as isize,
                            );
                        if !t.is_null() {
                            let ref mut fresh12 = (*r).right;
                            *fresh12 = (*t).right;
                            let ref mut fresh13 = (*t).right;
                            *fresh13 = r;
                        } else {
                            let ref mut fresh14 = (*r).right;
                            *fresh14 = *s;
                            *s = r;
                        }
                        let ref mut fresh15 = (*(*dt).data).here;
                        *fresh15 = r;
                        return obj;
                    }
                }
            }
        }
        match current_block {
            8164104897091980844 => {}
            _ => {
                if t.is_null() {
                    return 0 as *mut libc::c_void
                } else {
                    if !p.is_null() {
                        let ref mut fresh18 = (*p).right;
                        *fresh18 = (*t).right;
                    } else {
                        p = *s;
                        if p == t {
                            *s = (*t).right;
                            p = *s;
                        } else {
                            while (*p).right != t {
                                p = (*p).right;
                            }
                            let ref mut fresh19 = (*p).right;
                            *fresh19 = (*t).right;
                        }
                    }
                }
                obj = if lk < 0 as libc::c_int {
                    (*(t as *mut Dthold_t)).obj
                } else {
                    (t as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                };
                (*(*dt).data).size -= 1 as libc::c_int;
                let ref mut fresh20 = (*(*dt).data).here;
                *fresh20 = p;
                if ((*disc).freef).is_some() && type_0 & 0o2 as libc::c_int != 0 {
                    ((*disc).freef).expect("non-null function pointer")(dt, obj, disc);
                }
                if (*disc).link < 0 as libc::c_int {
                    ((*dt).memoryf)
                        .expect(
                            "non-null function pointer",
                        )(dt, t as *mut libc::c_void, 0 as libc::c_int as size_t, disc);
                }
                return obj;
            }
        }
    }
    let ref mut fresh17 = (*(*dt).data).loop_0;
    *fresh17 -= 1 as libc::c_int;
    if *fresh17 < 0 as libc::c_int {
        (*(*dt).data).loop_0 = 0 as libc::c_int;
    }
    if (*(*dt).data).size > (*(*dt).data).ntab << 1 as libc::c_int
        && (*(*dt).data).loop_0 <= 0 as libc::c_int
    {
        dthtab(dt);
    }
    return 0 as *mut libc::c_void;
}
static mut _Dtset: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dthash
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o1 as libc::c_int,
        };
        init
    }
};
static mut _Dtbag: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dthash
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o2 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut Dtset: *mut Dtmethod_t = unsafe {
    &_Dtset as *const Dtmethod_t as *mut Dtmethod_t
};
#[no_mangle]
pub static mut Dtbag: *mut Dtmethod_t = unsafe {
    &_Dtbag as *const Dtmethod_t as *mut Dtmethod_t
};
#[no_mangle]
pub static mut _Dthash: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dthash
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o1 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut Dthash: *mut Dtmethod_t = unsafe {
    &_Dthash as *const Dtmethod_t as *mut Dtmethod_t
};
