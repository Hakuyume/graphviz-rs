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
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn free(_: *mut libc::c_void);
    fn zmalloc(_: size_t) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intitem {
    pub id: libc::c_int,
    pub link: Dtlink_t,
}
unsafe extern "C" fn mkIntItem(
    mut d: *mut Dt_t,
    mut obj: *mut intitem,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut np: *mut intitem =
        zmalloc(::std::mem::size_of::<intitem>() as libc::c_ulong) as *mut intitem;
    (*np).id = (*obj).id;
    return np as *mut libc::c_void;
}
unsafe extern "C" fn freeIntItem(mut d: *mut Dt_t, mut obj: *mut intitem, mut disc: *mut Dtdisc_t) {
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn cmpid(
    mut d: *mut Dt_t,
    mut key1: *mut libc::c_int,
    mut key2: *mut libc::c_int,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if *key1 > *key2 {
        return 1 as libc::c_int;
    } else if *key1 < *key2 {
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int;
    };
}
static mut intSetDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            link: 8 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut intitem,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
                >,
                Dtmake_f,
            >(Some(
                mkIntItem
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut intitem,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
            )),
            freef: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Dt_t, *mut intitem, *mut Dtdisc_t) -> ()>,
                Dtfree_f,
            >(Some(
                freeIntItem as unsafe extern "C" fn(*mut Dt_t, *mut intitem, *mut Dtdisc_t) -> (),
            )),
            comparf: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(Some(
                cmpid
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
            )),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn openIntSet() -> *mut Dt_t {
    return dtopen(&mut intSetDisc, Dtoset);
}
#[no_mangle]
pub unsafe extern "C" fn addIntSet(mut is: *mut Dt_t, mut v: libc::c_int) {
    let mut obj: intitem = intitem {
        id: 0,
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
    };
    obj.id = v;
    (Some(((*is).searchf).expect("non-null function pointer"))).expect("non-null function pointer")(
        is,
        &mut obj as *mut intitem as *mut libc::c_void,
        0o1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn inIntSet(mut is: *mut Dt_t, mut v: libc::c_int) -> libc::c_int {
    return ((Some(((*is).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        is,
        &mut v as *mut libc::c_int as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) != 0 as *mut libc::c_void) as libc::c_int;
}
