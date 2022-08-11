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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
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
unsafe extern "C" fn dtlist(
    mut dt: *mut Dt_t,
    mut obj: *mut libc::c_void,
    mut type_0: libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut lk: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut ky: libc::c_int = 0;
    let mut cmpf: Dtcompar_f = None;
    let mut disc: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
    let mut r: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut t: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        dtrestore(dt, 0 as *mut Dtlink_t);
    } else {
    };
    disc = (*dt).disc;
    ky = (*disc).key;
    sz = (*disc).size;
    lk = (*disc).link;
    cmpf = (*disc).comparf;
    (*dt).type_0 &= !(0o100000 as libc::c_int);
    if obj.is_null() {
        if type_0 & (0o400 as libc::c_int | 0o200 as libc::c_int) != 0 {
            r = (*(*dt).data).hh._head;
            if !r.is_null() {
                if type_0 & 0o400 as libc::c_int != 0 {
                    r = (*r).hl._left;
                }
                let ref mut fresh0 = (*(*dt).data).here;
                *fresh0 = r;
            }
            return if !r.is_null() {
                if lk < 0 as libc::c_int {
                    (*(r as *mut Dthold_t)).obj
                } else {
                    (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                }
            } else {
                0 as *mut libc::c_void
            };
        } else if type_0 & (0o2 as libc::c_int | 0o10000 as libc::c_int) != 0 {
            if (*(*dt).data).type_0 & (0o20 as libc::c_int | 0o200 as libc::c_int) != 0 || {
                r = (*(*dt).data).hh._head;
                r.is_null()
            } {
                return 0 as *mut libc::c_void;
            }
        } else if type_0 & 0o100 as libc::c_int != 0 {
            if ((*disc).freef).is_some() || (*disc).link < 0 as libc::c_int {
                r = (*(*dt).data).hh._head;
                while !r.is_null() {
                    t = (*r).right;
                    if ((*disc).freef).is_some() {
                        ((*disc).freef).expect("non-null function pointer")(
                            dt,
                            if lk < 0 as libc::c_int {
                                (*(r as *mut Dthold_t)).obj
                            } else {
                                (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                            },
                            disc,
                        );
                    }
                    if (*disc).link < 0 as libc::c_int {
                        ((*dt).memoryf).expect("non-null function pointer")(
                            dt,
                            r as *mut libc::c_void,
                            0 as libc::c_int as size_t,
                            disc,
                        );
                    }
                    r = t;
                }
            }
            let ref mut fresh1 = (*(*dt).data).here;
            *fresh1 = 0 as *mut Dtlink_t;
            let ref mut fresh2 = (*(*dt).data).hh._head;
            *fresh2 = *fresh1;
            (*(*dt).data).size = 0 as libc::c_int;
            return 0 as *mut libc::c_void;
        } else {
            return 0 as *mut libc::c_void;
        }
    } else if type_0 & (0o1 as libc::c_int | 0o4000 as libc::c_int) != 0 {
        if ((*disc).makef).is_some() && type_0 & 0o1 as libc::c_int != 0 && {
            obj = ((*disc).makef).expect("non-null function pointer")(dt, obj, disc);
            obj.is_null()
        } {
            return 0 as *mut libc::c_void;
        }
        if lk >= 0 as libc::c_int {
            r = (obj as *mut libc::c_char).offset(lk as isize) as *mut Dtlink_t;
        } else {
            r = ((*dt).memoryf).expect("non-null function pointer")(
                dt,
                0 as *mut libc::c_void,
                ::std::mem::size_of::<Dthold_t>() as libc::c_ulong,
                disc,
            ) as *mut Dtlink_t;
            if !r.is_null() {
                let ref mut fresh3 = (*(r as *mut Dthold_t)).obj;
                *fresh3 = obj;
            } else {
                if ((*disc).makef).is_some()
                    && ((*disc).freef).is_some()
                    && type_0 & 0o1 as libc::c_int != 0
                {
                    ((*disc).freef).expect("non-null function pointer")(dt, obj, disc);
                }
                return 0 as *mut libc::c_void;
            }
        }
        if (*(*dt).data).type_0 & 0o200 as libc::c_int != 0 {
            if type_0 & 0o20000 as libc::c_int != 0 {
                current_block = 10109068454326310419;
            } else {
                current_block = 11184253766269282829;
            }
        } else if (*(*dt).data).type_0 & 0o20 as libc::c_int != 0 {
            if type_0 & 0o20000 as libc::c_int != 0 {
                t = (*(*dt).data).here;
                if t.is_null() || ((*t).right).is_null() {
                    current_block = 10109068454326310419;
                } else {
                    let ref mut fresh4 = (*r).right;
                    *fresh4 = (*t).right;
                    let ref mut fresh5 = (*(*r).right).hl._left;
                    *fresh5 = r;
                    let ref mut fresh6 = (*r).hl._left;
                    *fresh6 = t;
                    let ref mut fresh7 = (*(*r).hl._left).right;
                    *fresh7 = r;
                    current_block = 919954187481050311;
                }
            } else {
                t = (*(*dt).data).here;
                if t.is_null() || t == (*(*dt).data).hh._head {
                    current_block = 11184253766269282829;
                } else {
                    let ref mut fresh8 = (*r).hl._left;
                    *fresh8 = (*t).hl._left;
                    let ref mut fresh9 = (*(*r).hl._left).right;
                    *fresh9 = r;
                    let ref mut fresh10 = (*r).right;
                    *fresh10 = t;
                    let ref mut fresh11 = (*(*r).right).hl._left;
                    *fresh11 = r;
                    current_block = 919954187481050311;
                }
            }
        } else if (*(*dt).data).type_0 & 0o40 as libc::c_int != 0 {
            current_block = 11184253766269282829;
        } else {
            current_block = 10109068454326310419;
        }
        match current_block {
            10109068454326310419 => {
                t = (*(*dt).data).hh._head;
                if !t.is_null() {
                    let ref mut fresh17 = (*(*t).hl._left).right;
                    *fresh17 = r;
                    let ref mut fresh18 = (*r).hl._left;
                    *fresh18 = (*t).hl._left;
                    let ref mut fresh19 = (*t).hl._left;
                    *fresh19 = r;
                } else {
                    let ref mut fresh20 = (*(*dt).data).hh._head;
                    *fresh20 = r;
                    let ref mut fresh21 = (*r).hl._left;
                    *fresh21 = r;
                }
                let ref mut fresh22 = (*r).right;
                *fresh22 = 0 as *mut Dtlink_t;
            }
            11184253766269282829 => {
                t = (*(*dt).data).hh._head;
                let ref mut fresh12 = (*r).right;
                *fresh12 = t;
                if !t.is_null() {
                    let ref mut fresh13 = (*r).hl._left;
                    *fresh13 = (*t).hl._left;
                    let ref mut fresh14 = (*t).hl._left;
                    *fresh14 = r;
                } else {
                    let ref mut fresh15 = (*r).hl._left;
                    *fresh15 = r;
                }
                let ref mut fresh16 = (*(*dt).data).hh._head;
                *fresh16 = r;
            }
            _ => {}
        }
        if (*(*dt).data).size >= 0 as libc::c_int {
            (*(*dt).data).size += 1 as libc::c_int;
        }
        let ref mut fresh23 = (*(*dt).data).here;
        *fresh23 = r;
        return if lk < 0 as libc::c_int {
            (*(r as *mut Dthold_t)).obj
        } else {
            (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
        };
    } else {
        if type_0 & 0o1000 as libc::c_int != 0
            || {
                r = (*(*dt).data).here;
                r.is_null()
            }
            || (if lk < 0 as libc::c_int {
                (*(r as *mut Dthold_t)).obj
            } else {
                (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
            }) != obj
        {
            key = if type_0 & 0o1000 as libc::c_int != 0 {
                obj
            } else {
                (if sz < 0 as libc::c_int {
                    *((obj as *mut libc::c_char).offset(ky as isize) as *mut *mut libc::c_char)
                } else {
                    (obj as *mut libc::c_char).offset(ky as isize)
                }) as *mut libc::c_void
            };
            r = (*(*dt).data).hh._head;
            while !r.is_null() {
                k = if lk < 0 as libc::c_int {
                    (*(r as *mut Dthold_t)).obj
                } else {
                    (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                };
                k = (if sz < 0 as libc::c_int {
                    *((k as *mut libc::c_char).offset(ky as isize) as *mut *mut libc::c_char)
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
                r = (*r).right;
            }
        }
        if r.is_null() {
            return 0 as *mut libc::c_void;
        }
        (*dt).type_0 |= 0o100000 as libc::c_int;
        if !(type_0 & (0o2 as libc::c_int | 0o10000 as libc::c_int) != 0) {
            if type_0 & 0o10 as libc::c_int != 0 {
                r = (*r).right;
            } else if type_0 & 0o20 as libc::c_int != 0 {
                r = if r == (*(*dt).data).hh._head {
                    0 as *mut Dtlink_t
                } else {
                    (*r).hl._left
                };
            }
            let ref mut fresh30 = (*(*dt).data).here;
            *fresh30 = r;
            return if !r.is_null() {
                if lk < 0 as libc::c_int {
                    (*(r as *mut Dthold_t)).obj
                } else {
                    (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
                }
            } else {
                0 as *mut libc::c_void
            };
        }
    }
    if !((*r).right).is_null() {
        let ref mut fresh24 = (*(*r).right).hl._left;
        *fresh24 = (*r).hl._left;
    }
    t = (*(*dt).data).hh._head;
    if r == t {
        let ref mut fresh25 = (*(*dt).data).hh._head;
        *fresh25 = (*r).right;
        if !((*(*dt).data).hh._head).is_null() {
            let ref mut fresh26 = (*(*(*dt).data).hh._head).hl._left;
            *fresh26 = (*t).hl._left;
        }
    } else {
        let ref mut fresh27 = (*(*r).hl._left).right;
        *fresh27 = (*r).right;
        if r == (*t).hl._left {
            let ref mut fresh28 = (*t).hl._left;
            *fresh28 = (*r).hl._left;
        }
    }
    let ref mut fresh29 = (*(*dt).data).here;
    *fresh29 = if r == (*(*dt).data).here {
        (*r).right
    } else {
        0 as *mut Dtlink_t
    };
    (*(*dt).data).size -= 1 as libc::c_int;
    obj = if lk < 0 as libc::c_int {
        (*(r as *mut Dthold_t)).obj
    } else {
        (r as *mut libc::c_char).offset(-(lk as isize)) as *mut libc::c_void
    };
    if ((*disc).freef).is_some() && type_0 & 0o2 as libc::c_int != 0 {
        ((*disc).freef).expect("non-null function pointer")(dt, obj, disc);
    }
    if (*disc).link < 0 as libc::c_int {
        ((*dt).memoryf).expect("non-null function pointer")(
            dt,
            r as *mut libc::c_void,
            0 as libc::c_int as size_t,
            disc,
        );
    }
    return obj;
}
#[no_mangle]
pub static mut _Dtlist: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dtlist
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o20 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut _Dtdeque: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dtlist
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o200 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut _Dtstack: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dtlist
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o40 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut _Dtqueue: Dtmethod_t = unsafe {
    {
        let mut init = _dtmethod_s {
            searchf: Some(
                dtlist
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            type_0: 0o100 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub static mut Dtlist: *mut Dtmethod_t =
    unsafe { &_Dtlist as *const Dtmethod_t as *mut Dtmethod_t };
#[no_mangle]
pub static mut Dtdeque: *mut Dtmethod_t =
    unsafe { &_Dtdeque as *const Dtmethod_t as *mut Dtmethod_t };
#[no_mangle]
pub static mut Dtstack: *mut Dtmethod_t =
    unsafe { &_Dtstack as *const Dtmethod_t as *mut Dtmethod_t };
#[no_mangle]
pub static mut Dtqueue: *mut Dtmethod_t =
    unsafe { &_Dtqueue as *const Dtmethod_t as *mut Dtmethod_t };
