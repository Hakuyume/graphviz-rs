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
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub type queue = Dt_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nsitem {
    pub link: Dtlink_t,
    pub np: *mut libc::c_void,
}
unsafe extern "C" fn makef(
    mut d: *mut Dt_t,
    mut obj: *mut nsitem,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut p: *mut nsitem = 0 as *mut nsitem;
    p = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<nsitem>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut nsitem
    } else {
        malloc(
            (::std::mem::size_of::<nsitem>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut nsitem
    };
    let ref mut fresh0 = (*p).np;
    *fresh0 = (*obj).np;
    return p as *mut libc::c_void;
}
unsafe extern "C" fn freef(mut d: *mut Dt_t, mut obj: *mut nsitem, mut disc: *mut Dtdisc_t) {
    free(obj as *mut libc::c_void);
}
static mut ndisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut nsitem,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
                >,
                Dtmake_f,
            >(Some(
                makef
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut nsitem,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
            )),
            freef: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Dt_t, *mut nsitem, *mut Dtdisc_t) -> ()>,
                Dtfree_f,
            >(Some(
                freef as unsafe extern "C" fn(*mut Dt_t, *mut nsitem, *mut Dtdisc_t) -> (),
            )),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn mkQ(mut meth: *mut Dtmethod_t) -> *mut queue {
    let mut nq: *mut queue = 0 as *mut queue;
    nq = dtopen(&mut ndisc, meth);
    return nq;
}
#[no_mangle]
pub unsafe extern "C" fn push(mut nq: *mut queue, mut n: *mut libc::c_void) {
    let mut obj: nsitem = nsitem {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        np: 0 as *mut libc::c_void,
    };
    obj.np = n;
    (Some(((*(nq as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        nq,
        &mut obj as *mut nsitem as *mut libc::c_void,
        0o1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pop(mut nq: *mut queue, mut delete: libc::c_int) -> *mut libc::c_void {
    let mut obj: *mut nsitem = 0 as *mut nsitem;
    let mut n: *mut libc::c_void = 0 as *mut libc::c_void;
    obj = (Some(((*(nq as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        nq, 0 as *mut libc::c_void, 0o200 as libc::c_int
    ) as *mut nsitem;
    if !obj.is_null() {
        n = (*obj).np;
        if delete != 0 {
            (Some(((*(nq as *mut Dt_t)).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                nq,
                0 as *mut libc::c_void,
                0o2 as libc::c_int,
            );
        }
        return n;
    } else {
        return 0 as *mut libc::c_void;
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeQ(mut nq: *mut queue) {
    dtclose(nq);
}
