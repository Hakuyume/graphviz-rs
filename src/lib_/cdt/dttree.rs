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
unsafe extern "C" fn dttree(
    mut dt: *mut Dt_t,
    mut obj: *mut libc::c_void,
    mut type_0: libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut root: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut cmp: libc::c_int = 0;
    let mut lk: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut ky: libc::c_int = 0;
    let mut o: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut l: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut r: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut me: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut link: Dtlink_t = Dtlink_t {
        right: 0 as *mut Dtlink_t,
        hl: C2RustUnnamed { _hash: 0 },
    };
    let mut n: libc::c_int = 0;
    let mut minp: libc::c_int = 0;
    let mut turn: [libc::c_int; 62] = [0; 62];
    let mut cmpf: Dtcompar_f = None;
    let mut disc: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        dtrestore(dt, 0 as *mut Dtlink_t);
    } else {};
    disc = (*dt).disc;
    ky = (*disc).key;
    sz = (*disc).size;
    lk = (*disc).link;
    cmpf = (*disc).comparf;
    (*dt).type_0 &= !(0o100000 as libc::c_int);
    root = (*(*dt).data).here;
    if obj.is_null() {
        if root.is_null()
            || type_0
                & (0o100 as libc::c_int | 0o200 as libc::c_int | 0o400 as libc::c_int)
                == 0
        {
            return 0 as *mut libc::c_void;
        }
        if type_0 & 0o100 as libc::c_int != 0 {
            if ((*disc).freef).is_some() || (*disc).link < 0 as libc::c_int {
                loop {
                    loop {
                        t = (*root).hl._left;
                        if t.is_null() {
                            break;
                        }
                        let ref mut fresh0 = (*root).hl._left;
                        *fresh0 = (*t).right;
                        let ref mut fresh1 = (*t).right;
                        *fresh1 = root;
                        root = t;
                    }
                    t = (*root).right;
                    if ((*disc).freef).is_some() {
                        ((*disc).freef)
                            .expect(
                                "non-null function pointer",
                            )(
                            dt,
                            if lk < 0 as libc::c_int {
                                (*(root as *mut Dthold_t)).obj
                            } else {
                                (root as *mut libc::c_char).offset(-(lk as isize))
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
                            root as *mut libc::c_void,
                            0 as libc::c_int as size_t,
                            disc,
                        );
                    }
                    root = t;
                    if root.is_null() {
                        break;
                    }
                }
            }
            (*(*dt).data).size = 0 as libc::c_int;
            let ref mut fresh2 = (*(*dt).data).here;
            *fresh2 = 0 as *mut Dtlink_t;
            return 0 as *mut libc::c_void;
        } else {
            if type_0 & 0o400 as libc::c_int != 0 {
                loop {
                    t = (*root).right;
                    if t.is_null() {
                        break;
                    }
                    let ref mut fresh3 = (*root).right;
                    *fresh3 = (*t).hl._left;
                    let ref mut fresh4 = (*t).hl._left;
                    *fresh4 = root;
                    root = t;
                }
            } else {
                loop {
                    t = (*root).hl._left;
                    if t.is_null() {
                        break;
                    }
                    let ref mut fresh5 = (*root).hl._left;
                    *fresh5 = (*t).right;
                    let ref mut fresh6 = (*t).right;
                    *fresh6 = root;
                    root = t;
                }
            }
            let ref mut fresh7 = (*(*dt).data).here;
            *fresh7 = root;
            return if lk < 0 as libc::c_int {
                (*(root as *mut Dthold_t)).obj
            } else {
                (root as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
            };
        }
    }
    r = &mut link;
    l = r;
    if (*(*dt).meth).type_0 == 0o10 as libc::c_int
        && type_0 & (0o2 as libc::c_int | 0o10000 as libc::c_int) != 0
    {
        key = (if sz < 0 as libc::c_int {
            *((obj as *mut libc::c_char).offset(ky as isize) as *mut *mut libc::c_char)
        } else {
            (obj as *mut libc::c_char).offset(ky as isize)
        }) as *mut libc::c_void;
        o = (Some(((*dt).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(dt, obj, 0o4 as libc::c_int);
        loop {
            if o.is_null() {
                current_block = 10399321362245223758;
                break;
            }
            k = (if sz < 0 as libc::c_int {
                *((o as *mut libc::c_char).offset(ky as isize) as *mut *mut libc::c_char)
            } else {
                (o as *mut libc::c_char).offset(ky as isize)
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
            }) != 0 as libc::c_int
            {
                current_block = 10399321362245223758;
                break;
            }
            if o == obj {
                root = (*(*dt).data).here;
                let ref mut fresh8 = (*l).right;
                *fresh8 = (*root).hl._left;
                let ref mut fresh9 = (*r).hl._left;
                *fresh9 = (*root).right;
                current_block = 3519113796920557345;
                break;
            } else {
                o = (Some(((*dt).searchf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(dt, o, 0o10 as libc::c_int);
            }
        }
    } else {
        current_block = 10399321362245223758;
    }
    match current_block {
        10399321362245223758 => {
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
                if !root.is_null() {
                    current_block = 7826724679303511971;
                } else {
                    current_block = 12608488225262500095;
                }
            } else if type_0 & 0o40 as libc::c_int != 0 {
                me = obj as *mut Dtlink_t;
                obj = if lk < 0 as libc::c_int {
                    (*(me as *mut Dthold_t)).obj
                } else {
                    (me as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                };
                key = (if sz < 0 as libc::c_int {
                    *((obj as *mut libc::c_char).offset(ky as isize)
                        as *mut *mut libc::c_char)
                } else {
                    (obj as *mut libc::c_char).offset(ky as isize)
                }) as *mut libc::c_void;
                if !root.is_null() {
                    current_block = 7826724679303511971;
                } else {
                    current_block = 12608488225262500095;
                }
            } else if !root.is_null()
                    && (if lk < 0 as libc::c_int {
                        (*(root as *mut Dthold_t)).obj
                    } else {
                        (root as *mut libc::c_char).offset(-(lk as isize))
                            as *mut libc::c_void
                    }) != obj
                {
                key = (if sz < 0 as libc::c_int {
                    *((obj as *mut libc::c_char).offset(ky as isize)
                        as *mut *mut libc::c_char)
                } else {
                    (obj as *mut libc::c_char).offset(ky as isize)
                }) as *mut libc::c_void;
                current_block = 7826724679303511971;
            } else {
                current_block = 12608488225262500095;
            }
            match current_block {
                7826724679303511971 => {
                    if (*(*dt).meth).type_0 == 0o4 as libc::c_int
                        && {
                            minp = (*(*dt).data).minp;
                            minp != 0 as libc::c_int
                        } && type_0 & (0o1000 as libc::c_int | 0o4 as libc::c_int) != 0
                    {
                        t = root;
                        n = 0 as libc::c_int;
                        while n < minp {
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
                            cmp = (if cmpf.is_some() {
                                (Some(cmpf.expect("non-null function pointer")))
                                    .expect("non-null function pointer")(dt, key, k, disc)
                            } else {
                                (if sz <= 0 as libc::c_int {
                                    strcmp(key as *const libc::c_char, k as *const libc::c_char)
                                } else {
                                    memcmp(key, k, sz as size_t)
                                })
                            });
                            if cmp == 0 as libc::c_int {
                                return if lk < 0 as libc::c_int {
                                    (*(t as *mut Dthold_t)).obj
                                } else {
                                    (t as *mut libc::c_char).offset(-(lk as isize))
                                        as *mut libc::c_void
                                }
                            } else {
                                turn[n as usize] = cmp;
                                t = if cmp < 0 as libc::c_int {
                                    (*t).hl._left
                                } else {
                                    (*t).right
                                };
                                if t.is_null() {
                                    return 0 as *mut libc::c_void;
                                }
                            }
                            n += 1;
                        }
                        n = 0 as libc::c_int;
                        while n < minp {
                            if turn[n as usize] < 0 as libc::c_int {
                                t = (*root).hl._left;
                                if turn[(n + 1 as libc::c_int) as usize] < 0 as libc::c_int
                                {
                                    let ref mut fresh10 = (*root).hl._left;
                                    *fresh10 = (*t).right;
                                    let ref mut fresh11 = (*t).right;
                                    *fresh11 = root;
                                    let ref mut fresh12 = (*r).hl._left;
                                    *fresh12 = t;
                                    r = *fresh12;
                                    root = (*t).hl._left;
                                } else {
                                    let ref mut fresh13 = (*l).right;
                                    *fresh13 = t;
                                    l = *fresh13;
                                    let ref mut fresh14 = (*r).hl._left;
                                    *fresh14 = root;
                                    r = *fresh14;
                                    root = (*t).right;
                                }
                            } else {
                                t = (*root).right;
                                if turn[(n + 1 as libc::c_int) as usize] > 0 as libc::c_int
                                {
                                    let ref mut fresh15 = (*root).right;
                                    *fresh15 = (*t).hl._left;
                                    let ref mut fresh16 = (*t).hl._left;
                                    *fresh16 = root;
                                    let ref mut fresh17 = (*l).right;
                                    *fresh17 = t;
                                    l = *fresh17;
                                    root = (*t).right;
                                } else {
                                    let ref mut fresh18 = (*r).hl._left;
                                    *fresh18 = t;
                                    r = *fresh18;
                                    let ref mut fresh19 = (*l).right;
                                    *fresh19 = root;
                                    l = *fresh19;
                                    root = (*t).hl._left;
                                }
                            }
                            n += 2 as libc::c_int;
                        }
                    }
                    loop {
                        k = if lk < 0 as libc::c_int {
                            (*(root as *mut Dthold_t)).obj
                        } else {
                            (root as *mut libc::c_char).offset(-(lk as isize))
                                as *mut libc::c_void
                        };
                        k = (if sz < 0 as libc::c_int {
                            *((k as *mut libc::c_char).offset(ky as isize)
                                as *mut *mut libc::c_char)
                        } else {
                            (k as *mut libc::c_char).offset(ky as isize)
                        }) as *mut libc::c_void;
                        cmp = (if cmpf.is_some() {
                            (Some(cmpf.expect("non-null function pointer")))
                                .expect("non-null function pointer")(dt, key, k, disc)
                        } else {
                            (if sz <= 0 as libc::c_int {
                                strcmp(key as *const libc::c_char, k as *const libc::c_char)
                            } else {
                                memcmp(key, k, sz as size_t)
                            })
                        });
                        if cmp == 0 as libc::c_int {
                            break;
                        }
                        if cmp < 0 as libc::c_int {
                            t = (*root).hl._left;
                            if !t.is_null() {
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
                                cmp = (if cmpf.is_some() {
                                    (Some(cmpf.expect("non-null function pointer")))
                                        .expect("non-null function pointer")(dt, key, k, disc)
                                } else {
                                    (if sz <= 0 as libc::c_int {
                                        strcmp(key as *const libc::c_char, k as *const libc::c_char)
                                    } else {
                                        memcmp(key, k, sz as size_t)
                                    })
                                });
                                if cmp < 0 as libc::c_int {
                                    let ref mut fresh20 = (*root).hl._left;
                                    *fresh20 = (*t).right;
                                    let ref mut fresh21 = (*t).right;
                                    *fresh21 = root;
                                    let ref mut fresh22 = (*r).hl._left;
                                    *fresh22 = t;
                                    r = *fresh22;
                                    root = (*t).hl._left;
                                    if root.is_null() {
                                        break;
                                    }
                                } else if cmp == 0 as libc::c_int {
                                    let ref mut fresh23 = (*r).hl._left;
                                    *fresh23 = root;
                                    r = *fresh23;
                                    root = t;
                                    break;
                                } else {
                                    let ref mut fresh24 = (*l).right;
                                    *fresh24 = t;
                                    l = *fresh24;
                                    let ref mut fresh25 = (*r).hl._left;
                                    *fresh25 = root;
                                    r = *fresh25;
                                    root = (*t).right;
                                    if root.is_null() {
                                        break;
                                    }
                                }
                            } else {
                                let ref mut fresh26 = (*r).hl._left;
                                *fresh26 = root;
                                r = *fresh26;
                                root = 0 as *mut Dtlink_t;
                                break;
                            }
                        } else {
                            t = (*root).right;
                            if !t.is_null() {
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
                                cmp = (if cmpf.is_some() {
                                    (Some(cmpf.expect("non-null function pointer")))
                                        .expect("non-null function pointer")(dt, key, k, disc)
                                } else {
                                    (if sz <= 0 as libc::c_int {
                                        strcmp(key as *const libc::c_char, k as *const libc::c_char)
                                    } else {
                                        memcmp(key, k, sz as size_t)
                                    })
                                });
                                if cmp > 0 as libc::c_int {
                                    let ref mut fresh27 = (*root).right;
                                    *fresh27 = (*t).hl._left;
                                    let ref mut fresh28 = (*t).hl._left;
                                    *fresh28 = root;
                                    let ref mut fresh29 = (*l).right;
                                    *fresh29 = t;
                                    l = *fresh29;
                                    root = (*t).right;
                                    if root.is_null() {
                                        break;
                                    }
                                } else if cmp == 0 as libc::c_int {
                                    let ref mut fresh30 = (*l).right;
                                    *fresh30 = root;
                                    l = *fresh30;
                                    root = t;
                                    break;
                                } else {
                                    let ref mut fresh31 = (*r).hl._left;
                                    *fresh31 = t;
                                    r = *fresh31;
                                    let ref mut fresh32 = (*l).right;
                                    *fresh32 = root;
                                    l = *fresh32;
                                    root = (*t).hl._left;
                                    if root.is_null() {
                                        break;
                                    }
                                }
                            } else {
                                let ref mut fresh33 = (*l).right;
                                *fresh33 = root;
                                l = *fresh33;
                                root = 0 as *mut Dtlink_t;
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            if !root.is_null() {
                (*dt).type_0 |= 0o100000 as libc::c_int;
                let ref mut fresh34 = (*l).right;
                *fresh34 = (*root).hl._left;
                let ref mut fresh35 = (*r).hl._left;
                *fresh35 = (*root).right;
                if type_0 & (0o4 as libc::c_int | 0o1000 as libc::c_int) != 0 {
                    current_block = 3991888782355776856;
                } else if type_0 & 0o10 as libc::c_int != 0 {
                    let ref mut fresh44 = (*root).hl._left;
                    *fresh44 = link.right;
                    let ref mut fresh45 = (*root).right;
                    *fresh45 = 0 as *mut Dtlink_t;
                    link.right = root;
                    current_block = 15912224468364440450;
                } else if type_0 & 0o20 as libc::c_int != 0 {
                    let ref mut fresh48 = (*root).right;
                    *fresh48 = link.hl._left;
                    let ref mut fresh49 = (*root).hl._left;
                    *fresh49 = 0 as *mut Dtlink_t;
                    link.hl._left = root;
                    current_block = 2224934281058824896;
                } else if type_0 & (0o2 as libc::c_int | 0o10000 as libc::c_int) != 0 {
                    current_block = 3519113796920557345;
                } else if type_0 & (0o1 as libc::c_int | 0o4000 as libc::c_int) != 0 {
                    if (*(*dt).meth).type_0 & 0o4 as libc::c_int != 0 {
                        current_block = 3991888782355776856;
                    } else {
                        let ref mut fresh53 = (*root).hl._left;
                        *fresh53 = 0 as *mut Dtlink_t;
                        let ref mut fresh54 = (*root).right;
                        *fresh54 = link.hl._left;
                        link.hl._left = root;
                        current_block = 402273287568158252;
                    }
                } else {
                    if type_0 & 0o40 as libc::c_int != 0 {
                        if (*(*dt).meth).type_0 & 0o4 as libc::c_int != 0 {
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
                                    me as *mut libc::c_void,
                                    0 as libc::c_int as size_t,
                                    disc,
                                );
                            }
                        } else {
                            let ref mut fresh55 = (*me).hl._left;
                            *fresh55 = 0 as *mut Dtlink_t;
                            let ref mut fresh56 = (*me).right;
                            *fresh56 = link.hl._left;
                            link.hl._left = me;
                            (*(*dt).data).size += 1 as libc::c_int;
                        }
                    } else {
                        return 0 as *mut libc::c_void
                    }
                    current_block = 3991888782355776856;
                }
            } else {
                let ref mut fresh57 = (*r).hl._left;
                *fresh57 = 0 as *mut Dtlink_t;
                let ref mut fresh58 = (*l).right;
                *fresh58 = 0 as *mut Dtlink_t;
                if type_0 & 0o10 as libc::c_int != 0 {
                    current_block = 15912224468364440450;
                } else if type_0 & 0o20 as libc::c_int != 0 {
                    current_block = 2224934281058824896;
                } else if type_0 & (0o4 as libc::c_int | 0o1000 as libc::c_int) != 0 {
                    current_block = 17209826222385471788;
                } else if type_0 & (0o1 as libc::c_int | 0o4000 as libc::c_int) != 0 {
                    current_block = 402273287568158252;
                } else if type_0 & 0o40 as libc::c_int != 0 {
                    root = me;
                    (*(*dt).data).size += 1 as libc::c_int;
                    current_block = 3991888782355776856;
                } else {
                    obj = 0 as *mut libc::c_void;
                    current_block = 17209826222385471788;
                }
            }
            match current_block {
                17209826222385471788 => {}
                3519113796920557345 => {}
                _ => {
                    match current_block {
                        2224934281058824896 => {
                            root = link.right;
                            if !root.is_null() {
                                loop {
                                    t = (*root).right;
                                    if t.is_null() {
                                        break;
                                    }
                                    let ref mut fresh50 = (*root).right;
                                    *fresh50 = (*t).hl._left;
                                    let ref mut fresh51 = (*t).hl._left;
                                    *fresh51 = root;
                                    root = t;
                                }
                                link.right = (*root).hl._left;
                                current_block = 3991888782355776856;
                            } else {
                                current_block = 17209826222385471788;
                            }
                        }
                        15912224468364440450 => {
                            root = link.hl._left;
                            if !root.is_null() {
                                loop {
                                    t = (*root).hl._left;
                                    if t.is_null() {
                                        break;
                                    }
                                    let ref mut fresh46 = (*root).hl._left;
                                    *fresh46 = (*t).right;
                                    let ref mut fresh47 = (*t).right;
                                    *fresh47 = root;
                                    root = t;
                                }
                                link.hl._left = (*root).right;
                                current_block = 3991888782355776856;
                            } else {
                                current_block = 17209826222385471788;
                            }
                        }
                        402273287568158252 => {
                            if ((*disc).makef).is_some()
                                && type_0 & 0o1 as libc::c_int != 0
                            {
                                obj = ((*disc).makef)
                                    .expect("non-null function pointer")(dt, obj, disc);
                            }
                            if !obj.is_null() {
                                if lk >= 0 as libc::c_int {
                                    root = (obj as *mut libc::c_char).offset(lk as isize)
                                        as *mut Dtlink_t;
                                } else {
                                    root = ((*dt).memoryf)
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dt,
                                        0 as *mut libc::c_void,
                                        ::std::mem::size_of::<Dthold_t>() as libc::c_ulong,
                                        disc,
                                    ) as *mut Dtlink_t;
                                    if !root.is_null() {
                                        let ref mut fresh61 = (*(root as *mut Dthold_t)).obj;
                                        *fresh61 = obj;
                                    } else if ((*disc).makef).is_some()
                                            && ((*disc).freef).is_some()
                                            && type_0 & 0o1 as libc::c_int != 0
                                        {
                                        ((*disc).freef)
                                            .expect("non-null function pointer")(dt, obj, disc);
                                    }
                                }
                            }
                            if !root.is_null() {
                                if (*(*dt).data).size >= 0 as libc::c_int {
                                    (*(*dt).data).size += 1 as libc::c_int;
                                }
                                current_block = 3991888782355776856;
                            } else {
                                current_block = 17209826222385471788;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        17209826222385471788 => {}
                        _ => {
                            let ref mut fresh36 = (*root).hl._left;
                            *fresh36 = link.right;
                            let ref mut fresh37 = (*root).right;
                            *fresh37 = link.hl._left;
                            if (*(*dt).meth).type_0 & 0o10 as libc::c_int != 0
                                && type_0 & (0o4 as libc::c_int | 0o1000 as libc::c_int)
                                    != 0
                            {
                                key = if lk < 0 as libc::c_int {
                                    (*(root as *mut Dthold_t)).obj
                                } else {
                                    (root as *mut libc::c_char).offset(-(lk as isize))
                                        as *mut libc::c_void
                                };
                                key = (if sz < 0 as libc::c_int {
                                    *((key as *mut libc::c_char).offset(ky as isize)
                                        as *mut *mut libc::c_char)
                                } else {
                                    (key as *mut libc::c_char).offset(ky as isize)
                                }) as *mut libc::c_void;
                                loop {
                                    t = (*root).hl._left;
                                    if t.is_null() {
                                        break;
                                    }
                                    loop {
                                        r = (*t).right;
                                        if r.is_null() {
                                            break;
                                        }
                                        let ref mut fresh38 = (*t).right;
                                        *fresh38 = (*r).hl._left;
                                        let ref mut fresh39 = (*r).hl._left;
                                        *fresh39 = t;
                                        t = r;
                                    }
                                    let ref mut fresh40 = (*root).hl._left;
                                    *fresh40 = t;
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
                                    }) != 0 as libc::c_int
                                    {
                                        break;
                                    }
                                    let ref mut fresh41 = (*root).hl._left;
                                    *fresh41 = (*t).right;
                                    let ref mut fresh42 = (*t).right;
                                    *fresh42 = root;
                                    root = t;
                                }
                            }
                            let ref mut fresh43 = (*(*dt).data).here;
                            *fresh43 = root;
                            return if lk < 0 as libc::c_int {
                                (*(root as *mut Dthold_t)).obj
                            } else {
                                (root as *mut libc::c_char).offset(-(lk as isize))
                                    as *mut libc::c_void
                            };
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        3519113796920557345 => {
            obj = if lk < 0 as libc::c_int {
                (*(root as *mut Dthold_t)).obj
            } else {
                (root as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
            };
            if ((*disc).freef).is_some() && type_0 & 0o2 as libc::c_int != 0 {
                ((*disc).freef).expect("non-null function pointer")(dt, obj, disc);
            }
            if (*disc).link < 0 as libc::c_int {
                ((*dt).memoryf)
                    .expect(
                        "non-null function pointer",
                    )(dt, root as *mut libc::c_void, 0 as libc::c_int as size_t, disc);
            }
            let ref mut fresh52 = (*(*dt).data).size;
            *fresh52 -= 1 as libc::c_int;
            if *fresh52 < 0 as libc::c_int {
                (*(*dt).data).size = -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    loop {
        t = (*r).hl._left;
        if t.is_null() {
            break;
        }
        r = t;
    }
    let ref mut fresh59 = (*r).hl._left;
    *fresh59 = link.right;
    let ref mut fresh60 = (*(*dt).data).here;
    *fresh60 = link.hl._left;
    return if type_0 & 0o2 as libc::c_int != 0 { obj } else { 0 as *mut libc::c_void };
}
static mut _Dtoset: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dttree
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o4 as libc::c_int,
        };
        init
    }
};
static mut _Dtobag: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dttree
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o10 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut Dtoset: *mut Dtmethod_t = unsafe {
    &_Dtoset as *const Dtmethod_t as *mut Dtmethod_t
};
#[no_mangle]
pub static mut Dtobag: *mut Dtmethod_t = unsafe {
    &_Dtobag as *const Dtmethod_t as *mut Dtmethod_t
};
#[no_mangle]
pub static mut _Dttree: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dttree
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o4 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut Dtorder: *mut Dtmethod_t = unsafe {
    &_Dttree as *const Dtmethod_t as *mut Dtmethod_t
};
#[no_mangle]
pub static mut Dttree: *mut Dtmethod_t = unsafe {
    &_Dttree as *const Dtmethod_t as *mut Dtmethod_t
};
