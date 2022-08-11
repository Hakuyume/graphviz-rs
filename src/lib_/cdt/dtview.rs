#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn dtrestore(_: *mut Dt_t, _: *mut Dtlink_t) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
unsafe extern "C" fn dtvsearch(
    mut dt: *mut Dt_t,
    mut obj: *mut libc::c_void,
    mut type_0: libc::c_int,
) -> *mut libc::c_void {
    let mut d: *mut Dt_t = 0 as *mut Dt_t;
    let mut p: *mut Dt_t = 0 as *mut Dt_t;
    let mut o: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ok: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nk: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cmp: libc::c_int = 0;
    let mut lk: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut ky: libc::c_int = 0;
    let mut cmpf: Dtcompar_f = None;
    if type_0
        & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o100 as libc::c_int
            | 0o40 as libc::c_int) != 0
    {
        return (Some(((*(*dt).meth).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(dt, obj, type_0);
    }
    if type_0 & (0o1000 as libc::c_int | 0o4 as libc::c_int) != 0
        || type_0 & (0o200 as libc::c_int | 0o400 as libc::c_int) != 0
            && (*(*dt).meth).type_0 & (0o10 as libc::c_int | 0o4 as libc::c_int) == 0
    {
        d = dt;
        while !d.is_null() {
            o = ((*(*d).meth).searchf)
                .expect("non-null function pointer")(d, obj, type_0);
            if !o.is_null() {
                break;
            }
            d = (*d).view;
        }
        let ref mut fresh0 = (*dt).walk;
        *fresh0 = d;
        return o;
    }
    if (*(*dt).meth).type_0 & (0o10 as libc::c_int | 0o4 as libc::c_int) != 0 {
        if type_0
            & (0o200 as libc::c_int | 0o400 as libc::c_int | 0o10 as libc::c_int
                | 0o20 as libc::c_int) == 0
        {
            return 0 as *mut libc::c_void;
        }
        nk = 0 as *mut libc::c_void;
        n = nk;
        p = 0 as *mut Dt_t;
        d = dt;
        while !d.is_null() {
            o = ((*(*d).meth).searchf)
                .expect("non-null function pointer")(d, obj, type_0);
            if !o.is_null() {
                ky = (*(*d).disc).key;
                sz = (*(*d).disc).size;
                lk = (*(*d).disc).link;
                cmpf = (*(*d).disc).comparf;
                ok = (if sz < 0 as libc::c_int {
                    *((o as *mut libc::c_char).offset(ky as isize)
                        as *mut *mut libc::c_char)
                } else {
                    (o as *mut libc::c_char).offset(ky as isize)
                }) as *mut libc::c_void;
                let mut current_block_19: u64;
                if !n.is_null() {
                    cmp = if cmpf.is_some() {
                        (Some(cmpf.expect("non-null function pointer")))
                            .expect("non-null function pointer")(d, ok, nk, (*d).disc)
                    } else if sz <= 0 as libc::c_int {
                        strcmp(ok as *const libc::c_char, nk as *const libc::c_char)
                    } else {
                        memcmp(ok, nk, sz as size_t)
                    };
                    if type_0 & (0o10 as libc::c_int | 0o200 as libc::c_int) != 0
                        && cmp < 0 as libc::c_int
                        || type_0 & (0o20 as libc::c_int | 0o400 as libc::c_int) != 0
                            && cmp > 0 as libc::c_int
                    {
                        current_block_19 = 18413085401307447182;
                    } else {
                        current_block_19 = 15125582407903384992;
                    }
                } else {
                    current_block_19 = 18413085401307447182;
                }
                match current_block_19 {
                    18413085401307447182 => {
                        p = d;
                        n = o;
                        nk = ok;
                    }
                    _ => {}
                }
            }
            d = (*d).view;
        }
        let ref mut fresh1 = (*dt).walk;
        *fresh1 = p;
        return n;
    }
    if type_0 & (0o10 as libc::c_int | 0o20 as libc::c_int) == 0 {
        return 0 as *mut libc::c_void;
    }
    if ((*dt).walk).is_null()
        || obj
            != (if (*(*(*dt).walk).disc).link < 0 as libc::c_int {
                (*((*(*(*dt).walk).data).here as *mut Dthold_t)).obj
            } else {
                ((*(*(*dt).walk).data).here as *mut libc::c_char)
                    .offset(-((*(*(*dt).walk).disc).link as isize)) as *mut libc::c_void
            })
    {
        d = dt;
        while !d.is_null() {
            o = ((*(*d).meth).searchf)
                .expect("non-null function pointer")(d, obj, 0o4 as libc::c_int);
            if !o.is_null() {
                break;
            }
            d = (*d).view;
        }
        let ref mut fresh2 = (*dt).walk;
        *fresh2 = d;
        obj = o;
        if obj.is_null() {
            return 0 as *mut libc::c_void;
        }
    }
    d = (*dt).walk;
    obj = ((*(*d).meth).searchf).expect("non-null function pointer")(d, obj, type_0);
    loop {
        while !obj.is_null() {
            p = dt;
            loop {
                if p == d {
                    return obj;
                }
                if !(((*(*p).meth).searchf)
                    .expect("non-null function pointer")(p, obj, 0o4 as libc::c_int))
                    .is_null()
                {
                    break;
                }
                p = (*p).view;
            }
            obj = ((*(*d).meth).searchf)
                .expect("non-null function pointer")(d, obj, type_0);
        }
        let ref mut fresh3 = (*dt).walk;
        *fresh3 = (*d).view;
        d = *fresh3;
        if d.is_null() {
            return 0 as *mut libc::c_void
        } else {
            if type_0 & 0o10 as libc::c_int != 0 {
                obj = ((*(*d).meth).searchf)
                    .expect(
                        "non-null function pointer",
                    )(d, 0 as *mut libc::c_void, 0o200 as libc::c_int);
            } else {
                obj = ((*(*d).meth).searchf)
                    .expect(
                        "non-null function pointer",
                    )(d, 0 as *mut libc::c_void, 0o400 as libc::c_int);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dtview(mut dt: *mut Dt_t, mut view: *mut Dt_t) -> *mut Dt_t {
    let mut d: *mut Dt_t = 0 as *mut Dt_t;
    if (*(*dt).data).type_0 & 0o10000 as libc::c_int != 0 {
        dtrestore(dt, 0 as *mut Dtlink_t);
    } else {};
    if !view.is_null() {
        if (*(*view).data).type_0 & 0o10000 as libc::c_int != 0 {
            dtrestore(view, 0 as *mut Dtlink_t);
        } else {};
        if (*view).meth != (*dt).meth {
            return 0 as *mut Dt_t;
        }
    }
    d = view;
    while !d.is_null() {
        if d == dt {
            return 0 as *mut Dt_t;
        }
        d = (*d).view;
    }
    d = (*dt).view;
    if !d.is_null() {
        (*d).nview -= 1 as libc::c_int;
    }
    let ref mut fresh4 = (*dt).walk;
    *fresh4 = 0 as *mut Dt_t;
    let ref mut fresh5 = (*dt).view;
    *fresh5 = *fresh4;
    if view.is_null() {
        let ref mut fresh6 = (*dt).searchf;
        *fresh6 = (*(*dt).meth).searchf;
        return d;
    }
    let ref mut fresh7 = (*dt).view;
    *fresh7 = view;
    let ref mut fresh8 = (*dt).searchf;
    *fresh8 = Some(
        dtvsearch
            as unsafe extern "C" fn(
                *mut Dt_t,
                *mut libc::c_void,
                libc::c_int,
            ) -> *mut libc::c_void,
    );
    (*view).nview += 1 as libc::c_int;
    return view;
}
