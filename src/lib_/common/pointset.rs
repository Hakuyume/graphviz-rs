#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn free(_: *mut libc::c_void);
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type Dtlink_t = _dtlink_s;
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
pub type Dict_t = _dt_s;
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
pub type Dt_t = _dt_s;
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
pub type Dtdisc_t = _dtdisc_s;
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
pub type PointSet = Dict_t;
pub type PointMap = Dict_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair {
    pub link: Dtlink_t,
    pub id: point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MPairDisc {
    pub disc: Dtdisc_t,
    pub flist: *mut mpair,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpair {
    pub link: Dtlink_t,
    pub id: point,
    pub v: libc::c_int,
}
unsafe extern "C" fn mkPair(mut p: point) -> *mut pair {
    let mut pp: *mut pair = 0 as *mut pair;
    pp = zmalloc(::std::mem::size_of::<pair>() as libc::c_ulong) as *mut pair;
    (*pp).id = p;
    return pp;
}
unsafe extern "C" fn freePair(
    mut d: *mut Dt_t,
    mut pp: *mut pair,
    mut disc: *mut Dtdisc_t,
) {
    free(pp as *mut libc::c_void);
}
unsafe extern "C" fn cmppair(
    mut d: *mut Dt_t,
    mut key1: *mut point,
    mut key2: *mut point,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if (*key1).x > (*key2).x {
        return 1 as libc::c_int
    } else if (*key1).x < (*key2).x {
        return -(1 as libc::c_int)
    } else if (*key1).y > (*key2).y {
        return 1 as libc::c_int
    } else if (*key1).y < (*key2).y {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
static mut intPairDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<point>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut pair, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freePair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut pair,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut point,
                        *mut point,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    cmppair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut point,
                            *mut point,
                            *mut Dtdisc_t,
                        ) -> libc::c_int,
                ),
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn newPS() -> *mut PointSet {
    return dtopen(&mut intPairDisc, Dtoset);
}
#[no_mangle]
pub unsafe extern "C" fn freePS(mut ps: *mut PointSet) {
    dtclose(ps);
}
#[no_mangle]
pub unsafe extern "C" fn insertPS(mut ps: *mut PointSet, mut pt: point) {
    let mut pp: *mut pair = 0 as *mut pair;
    pp = mkPair(pt);
    if (Some(((*(ps as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ps, pp as *mut libc::c_void, 0o1 as libc::c_int) != pp as *mut libc::c_void
    {
        free(pp as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn addPS(
    mut ps: *mut PointSet,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut pt: point = point { x: 0, y: 0 };
    let mut pp: *mut pair = 0 as *mut pair;
    pt.x = x;
    pt.y = y;
    pp = mkPair(pt);
    if (Some(((*(ps as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ps, pp as *mut libc::c_void, 0o1 as libc::c_int) != pp as *mut libc::c_void
    {
        free(pp as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn inPS(mut ps: *mut PointSet, mut pt: point) -> libc::c_int {
    let mut p: pair = pair {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id: point { x: 0, y: 0 },
    };
    p.id = pt;
    return if !((Some(
        ((*(ps as *mut Dt_t)).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(ps, &mut p as *mut pair as *mut libc::c_void, 0o4 as libc::c_int))
        .is_null()
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn isInPS(
    mut ps: *mut PointSet,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    let mut p: pair = pair {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id: point { x: 0, y: 0 },
    };
    p.id.x = x;
    p.id.y = y;
    return if !((Some(
        ((*(ps as *mut Dt_t)).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(ps, &mut p as *mut pair as *mut libc::c_void, 0o4 as libc::c_int))
        .is_null()
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sizeOf(mut ps: *mut PointSet) -> libc::c_int {
    return dtsize(ps);
}
#[no_mangle]
pub unsafe extern "C" fn pointsOf(mut ps: *mut PointSet) -> *mut point {
    let mut n: libc::c_int = dtsize(ps);
    let mut pts: *mut point = gcalloc(
        n as size_t,
        ::std::mem::size_of::<point>() as libc::c_ulong,
    ) as *mut point;
    let mut p: *mut pair = 0 as *mut pair;
    let mut pp: *mut point = pts;
    p = dtflatten(ps) as *mut pair;
    while !p.is_null() {
        let fresh0 = pp;
        pp = pp.offset(1);
        *fresh0 = (*p).id;
        p = (*(p as *mut Dtlink_t)).right as *mut pair;
    }
    return pts;
}
unsafe extern "C" fn mkMPair(
    mut d: *mut Dt_t,
    mut obj: *mut mpair,
    mut disc: *mut MPairDisc,
) -> *mut mpair {
    let mut ap: *mut mpair = 0 as *mut mpair;
    if !((*disc).flist).is_null() {
        ap = (*disc).flist;
        let ref mut fresh1 = (*disc).flist;
        *fresh1 = (*ap).link.right as *mut mpair;
    } else {
        ap = gmalloc(::std::mem::size_of::<mpair>() as libc::c_ulong) as *mut mpair;
    }
    (*ap).id = (*obj).id;
    (*ap).v = (*obj).v;
    return ap;
}
unsafe extern "C" fn freeMPair(
    mut d: *mut Dt_t,
    mut ap: *mut mpair,
    mut disc: *mut MPairDisc,
) {
    let ref mut fresh2 = (*ap).link.right;
    *fresh2 = (*disc).flist as *mut Dtlink_t;
    let ref mut fresh3 = (*disc).flist;
    *fresh3 = ap;
}
static mut intMPairDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<point>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut mpair,
                        *mut MPairDisc,
                    ) -> *mut mpair,
                >,
                Dtmake_f,
            >(
                Some(
                    mkMPair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut mpair,
                            *mut MPairDisc,
                        ) -> *mut mpair,
                ),
            ),
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut mpair, *mut MPairDisc) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freeMPair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut mpair,
                            *mut MPairDisc,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut point,
                        *mut point,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    cmppair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut point,
                            *mut point,
                            *mut Dtdisc_t,
                        ) -> libc::c_int,
                ),
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn newPM() -> *mut PointMap {
    let mut dp: *mut MPairDisc = gmalloc(
        ::std::mem::size_of::<MPairDisc>() as libc::c_ulong,
    ) as *mut MPairDisc;
    (*dp).disc = intMPairDisc;
    let ref mut fresh4 = (*dp).flist;
    *fresh4 = 0 as *mut mpair;
    return dtopen(&mut (*dp).disc, Dtoset);
}
#[no_mangle]
pub unsafe extern "C" fn clearPM(mut ps: *mut PointMap) {
    (Some(((*(ps as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ps, 0 as *mut libc::c_void, 0o100 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn freePM(mut ps: *mut PointMap) {
    let mut dp: *mut MPairDisc = (*ps).disc as *mut MPairDisc;
    let mut p: *mut mpair = 0 as *mut mpair;
    let mut next: *mut mpair = 0 as *mut mpair;
    dtclose(ps);
    p = (*dp).flist;
    while !p.is_null() {
        next = (*p).link.right as *mut mpair;
        free(p as *mut libc::c_void);
        p = next;
    }
    free(dp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn insertPM(
    mut pm: *mut PointMap,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut v: libc::c_int,
) -> libc::c_int {
    let mut p: *mut mpair = 0 as *mut mpair;
    let mut dummy: mpair = mpair {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id: point { x: 0, y: 0 },
        v: 0,
    };
    dummy.id.x = x;
    dummy.id.y = y;
    dummy.v = v;
    p = (Some(((*(pm as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(pm, &mut dummy as *mut mpair as *mut libc::c_void, 0o1 as libc::c_int)
        as *mut mpair;
    return (*p).v;
}
