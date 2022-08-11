#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut Dttree: *mut Dtmethod_t;
    fn abort() -> !;
    fn agdelcb(g: *mut Agraph_t, obj: *mut libc::c_void, disc: *mut Agcbstack_t);
    fn agupdcb(
        g: *mut Agraph_t,
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        disc: *mut Agcbstack_t,
    );
    fn aginitcb(g: *mut Agraph_t, obj: *mut libc::c_void, disc: *mut Agcbstack_t);
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agdtopen(
        g: *mut Agraph_t,
        disc: *mut Dtdisc_t,
        method: *mut Dtmethod_t,
    ) -> *mut Dict_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
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
pub type Dict_t = _dt_s;
pub type IDTYPE = uint64_t;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct Agtag_s {
    #[bitfield(name = "objtype", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "mtflock", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "attrwf", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "seq", ty = "libc::c_uint", bits = "4..=31")]
    pub objtype_mtflock_attrwf_seq: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub id: IDTYPE,
}
pub type Agtag_t = Agtag_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agobj_s {
    pub tag: Agtag_t,
    pub data: *mut Agrec_t,
}
pub type Agrec_t = Agrec_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agrec_s {
    pub name: *mut libc::c_char,
    pub next: *mut Agrec_t,
}
pub type Agobj_t = Agobj_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agraph_s {
    pub base: Agobj_t,
    pub desc: Agdesc_t,
    pub link: Dtlink_t,
    pub n_seq: *mut Dict_t,
    pub n_id: *mut Dict_t,
    pub e_seq: *mut Dict_t,
    pub e_id: *mut Dict_t,
    pub g_dict: *mut Dict_t,
    pub parent: *mut Agraph_t,
    pub root: *mut Agraph_t,
    pub clos: *mut Agclos_t,
}
pub type Agclos_t = Agclos_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agclos_s {
    pub disc: Agdisc_t,
    pub state: Agdstate_t,
    pub strdict: *mut Dict_t,
    pub seq: [uint64_t; 3],
    pub cb: *mut Agcbstack_t,
    pub callbacks_enabled: libc::c_uchar,
    pub lookup_by_name: [*mut Dict_t; 3],
    pub lookup_by_id: [*mut Dict_t; 3],
}
pub type Agcbstack_t = Agcbstack_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agcbstack_s {
    pub f: *mut Agcbdisc_t,
    pub state: *mut libc::c_void,
    pub prev: *mut Agcbstack_t,
}
pub type Agcbdisc_t = Agcbdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agcbdisc_s {
    pub graph: C2RustUnnamed_1,
    pub node: C2RustUnnamed_1,
    pub edge: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ins: agobjfn_t,
    pub mod_0: agobjupdfn_t,
    pub del: agobjfn_t,
}
pub type agobjfn_t = Option::<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> (),
>;
pub type Agraph_t = Agraph_s;
pub type agobjupdfn_t = Option::<
    unsafe extern "C" fn(
        *mut Agraph_t,
        *mut Agobj_t,
        *mut libc::c_void,
        *mut Agsym_t,
    ) -> (),
>;
pub type Agsym_t = Agsym_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agsym_s {
    pub link: Dtlink_t,
    pub name: *mut libc::c_char,
    pub defval: *mut libc::c_char,
    pub id: libc::c_int,
    pub kind: libc::c_uchar,
    pub fixed: libc::c_uchar,
    pub print: libc::c_uchar,
}
pub type Agdstate_t = Agdstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdstate_s {
    pub mem: *mut libc::c_void,
    pub id: *mut libc::c_void,
}
pub type Agdisc_t = Agdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdisc_s {
    pub mem: *mut Agmemdisc_t,
    pub id: *mut Agiddisc_t,
    pub io: *mut Agiodisc_t,
}
pub type Agiodisc_t = Agiodisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiodisc_s {
    pub afread: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub putstr: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type Agiddisc_t = Agiddisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiddisc_s {
    pub open: Option::<
        unsafe extern "C" fn(*mut Agraph_t, *mut Agdisc_t) -> *mut libc::c_void,
    >,
    pub map: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_char,
            *mut IDTYPE,
            libc::c_int,
        ) -> libc::c_long,
    >,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> libc::c_long,
    >,
    pub free: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> (),
    >,
    pub print: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> *mut libc::c_char,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub idregister: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, *mut libc::c_void) -> (),
    >,
}
pub type Agmemdisc_t = Agmemdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agmemdisc_s {
    pub open: Option::<unsafe extern "C" fn(*mut Agdisc_t) -> *mut libc::c_void>,
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub resize: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type Agdesc_t = Agdesc_s;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct Agdesc_s {
    #[bitfield(name = "directed", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "strict", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "no_loop", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "maingraph", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "flatlock", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "no_write", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "has_attrs", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "has_cmpnd", ty = "libc::c_uint", bits = "7..=7")]
    pub directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type cb_t = libc::c_uint;
pub const CB_DELETION: cb_t = 2;
pub const CB_UPDATE: cb_t = 1;
pub const CB_INITIALIZE: cb_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub g: *mut Dict_t,
    pub n: *mut Dict_t,
    pub e: *mut Dict_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pendingset_t {
    pub h: Agrec_t,
    pub ins: C2RustUnnamed_2,
    pub mod_0: C2RustUnnamed_2,
    pub del: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pending_cb_t {
    pub link: Dtlink_t,
    pub key: IDTYPE,
    pub g: *mut Agraph_t,
    pub obj: *mut Agobj_t,
    pub symlist: *mut symlist_t,
}
pub type symlist_t = symlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symlist_s {
    pub sym: *mut Agsym_t,
    pub link: *mut symlist_s,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
static mut DRName: [libc::c_char; 12] = unsafe {
    *::std::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"_AG_pending\0")
};
unsafe extern "C" fn free_symlist(mut pcb: *mut pending_cb_t) {
    let mut s: *mut symlist_t = 0 as *mut symlist_t;
    let mut t: *mut symlist_t = 0 as *mut symlist_t;
    s = (*pcb).symlist;
    while !s.is_null() {
        t = (*s).link;
        agfree((*pcb).g, s as *mut libc::c_void);
        s = t;
    }
}
unsafe extern "C" fn freef(
    mut dict: *mut Dict_t,
    mut ptr: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) {
    let mut pcb: *mut pending_cb_t = 0 as *mut pending_cb_t;
    pcb = ptr as *mut pending_cb_t;
    free_symlist(pcb);
    agfree((*pcb).g, pcb as *mut libc::c_void);
}
static mut Disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<uint64_t>() as libc::c_ulong as libc::c_int,
            link: 0,
            makef: None,
            freef: Some(
                freef
                    as unsafe extern "C" fn(
                        *mut Dict_t,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> (),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn dictof(
    mut ds: *mut pendingset_t,
    mut obj: *mut Agobj_t,
    mut kind: cb_t,
) -> *mut Dict_t {
    let mut dict_ref: *mut *mut Dict_t = 0 as *mut *mut Dict_t;
    dict_ref = 0 as *mut *mut Dict_t;
    match ((*obj).tag).objtype() as libc::c_int {
        0 => {
            match kind as libc::c_uint {
                0 => {
                    dict_ref = &mut (*ds).ins.g;
                }
                1 => {
                    dict_ref = &mut (*ds).mod_0.g;
                }
                2 => {
                    dict_ref = &mut (*ds).del.g;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s:%d: claimed unreachable code was reached\0" as *const u8
                            as *const libc::c_char,
                        b"pend.c\0" as *const u8 as *const libc::c_char,
                        84 as libc::c_int,
                    );
                    abort();
                }
            }
        }
        1 => {
            match kind as libc::c_uint {
                0 => {
                    dict_ref = &mut (*ds).ins.n;
                }
                1 => {
                    dict_ref = &mut (*ds).mod_0.n;
                }
                2 => {
                    dict_ref = &mut (*ds).del.n;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s:%d: claimed unreachable code was reached\0" as *const u8
                            as *const libc::c_char,
                        b"pend.c\0" as *const u8 as *const libc::c_char,
                        99 as libc::c_int,
                    );
                    abort();
                }
            }
        }
        2 => {
            match kind as libc::c_uint {
                0 => {
                    dict_ref = &mut (*ds).ins.e;
                }
                1 => {
                    dict_ref = &mut (*ds).mod_0.e;
                }
                2 => {
                    dict_ref = &mut (*ds).del.e;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s:%d: claimed unreachable code was reached\0" as *const u8
                            as *const libc::c_char,
                        b"pend.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                    );
                    abort();
                }
            }
        }
        _ => {}
    }
    if dict_ref.is_null() {
        agerr(AGERR, b"pend dictof a bad object\0" as *const u8 as *const libc::c_char);
    }
    if (*dict_ref).is_null() {
        *dict_ref = agdtopen(agraphof(obj as *mut libc::c_void), &mut Disc, Dttree);
    }
    return *dict_ref;
}
unsafe extern "C" fn genkey(mut obj: *mut Agobj_t) -> IDTYPE {
    return (*obj).tag.id;
}
unsafe extern "C" fn lookup(
    mut dict: *mut Dict_t,
    mut obj: *mut Agobj_t,
) -> *mut pending_cb_t {
    let mut key: pending_cb_t = pending_cb_t {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        key: 0,
        g: 0 as *mut Agraph_t,
        obj: 0 as *mut Agobj_t,
        symlist: 0 as *mut symlist_t,
    };
    let mut rv: *mut pending_cb_t = 0 as *mut pending_cb_t;
    key.key = genkey(obj);
    rv = (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(dict, &mut key as *mut pending_cb_t as *mut libc::c_void, 0o4 as libc::c_int)
        as *mut pending_cb_t;
    return rv;
}
unsafe extern "C" fn record_sym(
    mut obj: *mut Agobj_t,
    mut handle: *mut pending_cb_t,
    mut optsym: *mut Agsym_t,
) {
    let mut sym: *mut symlist_t = 0 as *mut symlist_t;
    let mut nsym: *mut symlist_t = 0 as *mut symlist_t;
    let mut psym: *mut symlist_t = 0 as *mut symlist_t;
    psym = 0 as *mut symlist_t;
    sym = (*handle).symlist;
    while !sym.is_null() {
        if (*sym).sym == optsym {
            break;
        }
        if sym.is_null() {
            nsym = agalloc(
                agraphof(obj as *mut libc::c_void),
                ::std::mem::size_of::<symlist_t>() as libc::c_ulong,
            ) as *mut symlist_t;
            let ref mut fresh0 = (*nsym).sym;
            *fresh0 = optsym;
            if !psym.is_null() {
                let ref mut fresh1 = (*psym).link;
                *fresh1 = nsym;
            } else {
                let ref mut fresh2 = (*handle).symlist;
                *fresh2 = nsym;
            }
        }
        psym = sym;
        sym = (*sym).link;
    }
}
unsafe extern "C" fn insert(
    mut dict: *mut Dict_t,
    mut g: *mut Agraph_t,
    mut obj: *mut Agobj_t,
    mut optsym: *mut Agsym_t,
) -> *mut pending_cb_t {
    let mut handle: *mut pending_cb_t = 0 as *mut pending_cb_t;
    handle = agalloc(
        agraphof(obj as *mut libc::c_void),
        ::std::mem::size_of::<pending_cb_t>() as libc::c_ulong,
    ) as *mut pending_cb_t;
    let ref mut fresh3 = (*handle).obj;
    *fresh3 = obj;
    (*handle).key = genkey(obj);
    let ref mut fresh4 = (*handle).g;
    *fresh4 = g;
    if !optsym.is_null() {
        let ref mut fresh5 = (*handle).symlist;
        *fresh5 = agalloc(
            (*handle).g,
            ::std::mem::size_of::<symlist_t>() as libc::c_ulong,
        ) as *mut symlist_t;
        let ref mut fresh6 = (*(*handle).symlist).sym;
        *fresh6 = optsym;
    }
    (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(dict, handle as *mut libc::c_void, 0o1 as libc::c_int);
    return handle;
}
unsafe extern "C" fn purge(mut dict: *mut Dict_t, mut obj: *mut Agobj_t) {
    let mut handle: *mut pending_cb_t = 0 as *mut pending_cb_t;
    handle = lookup(dict, obj);
    if !handle.is_null() {
        (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(dict, handle as *mut libc::c_void, 0o2 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agrecord_callback(
    mut g: *mut Agraph_t,
    mut obj: *mut Agobj_t,
    mut kind: cb_t,
    mut optsym: *mut Agsym_t,
) {
    let mut pending: *mut pendingset_t = 0 as *mut pendingset_t;
    let mut dict: *mut Dict_t = 0 as *mut Dict_t;
    let mut handle: *mut pending_cb_t = 0 as *mut pending_cb_t;
    pending = agbindrec(
        g as *mut libc::c_void,
        DRName.as_mut_ptr(),
        ::std::mem::size_of::<pendingset_t>() as libc::c_ulong as libc::c_uint,
        0 as libc::c_int,
    ) as *mut pendingset_t;
    match kind as libc::c_uint {
        0 => {
            if (lookup(dictof(pending, obj, CB_UPDATE), obj)).is_null() {} else {
                __assert_fail(
                    b"lookup(dictof(pending, obj, CB_UPDATE), obj) == 0\0" as *const u8
                        as *const libc::c_char,
                    b"pend.c\0" as *const u8 as *const libc::c_char,
                    198 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 63],
                        &[libc::c_char; 63],
                    >(
                        b"void agrecord_callback(Agraph_t *, Agobj_t *, cb_t, Agsym_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if (lookup(dictof(pending, obj, CB_DELETION), obj)).is_null() {} else {
                __assert_fail(
                    b"lookup(dictof(pending, obj, CB_DELETION), obj) == 0\0" as *const u8
                        as *const libc::c_char,
                    b"pend.c\0" as *const u8 as *const libc::c_char,
                    199 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 63],
                        &[libc::c_char; 63],
                    >(
                        b"void agrecord_callback(Agraph_t *, Agobj_t *, cb_t, Agsym_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            dict = dictof(pending, obj, CB_INITIALIZE);
            handle = lookup(dict, obj);
            if handle.is_null() {
                handle = insert(dict, g, obj, optsym);
            }
        }
        1 => {
            if (lookup(dictof(pending, obj, CB_INITIALIZE), obj)).is_null() {
                if (lookup(dictof(pending, obj, CB_DELETION), obj)).is_null() {
                    dict = dictof(pending, obj, CB_UPDATE);
                    handle = lookup(dict, obj);
                    if handle.is_null() {
                        handle = insert(dict, g, obj, optsym);
                    }
                    record_sym(obj, handle, optsym);
                }
            }
        }
        2 => {
            purge(dictof(pending, obj, CB_INITIALIZE), obj);
            purge(dictof(pending, obj, CB_UPDATE), obj);
            dict = dictof(pending, obj, CB_DELETION);
            handle = lookup(dict, obj);
            if handle.is_null() {
                handle = insert(dict, g, obj, optsym);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"pend.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int,
            );
            abort();
        }
    };
}
unsafe extern "C" fn cb(mut dict: *mut Dict_t, mut callback_kind: cb_t) {
    let mut pcb: *mut pending_cb_t = 0 as *mut pending_cb_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut psym: *mut symlist_t = 0 as *mut symlist_t;
    let mut stack: *mut Agcbstack_t = 0 as *mut Agcbstack_t;
    if !dict.is_null() {
        loop {
            pcb = (Some(
                ((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(dict, 0 as *mut libc::c_void, 0o200 as libc::c_int)
                as *mut pending_cb_t;
            if pcb.is_null() {
                break;
            }
            g = (*pcb).g;
            stack = (*(*g).clos).cb;
            match callback_kind as libc::c_uint {
                0 => {
                    aginitcb(g, (*pcb).obj as *mut libc::c_void, stack);
                }
                1 => {
                    psym = (*pcb).symlist;
                    while !psym.is_null() {
                        agupdcb(g, (*pcb).obj as *mut libc::c_void, (*psym).sym, stack);
                        psym = (*psym).link;
                    }
                }
                2 => {
                    agdelcb(g, (*pcb).obj as *mut libc::c_void, stack);
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s:%d: claimed unreachable code was reached\0" as *const u8
                            as *const libc::c_char,
                        b"pend.c\0" as *const u8 as *const libc::c_char,
                        252 as libc::c_int,
                    );
                    abort();
                }
            }
            (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(dict, pcb as *mut libc::c_void, 0o2 as libc::c_int);
        }
    }
}
unsafe extern "C" fn agrelease_callbacks(mut g: *mut Agraph_t) {
    let mut pending: *mut pendingset_t = 0 as *mut pendingset_t;
    if (*(*g).clos).callbacks_enabled == 0 {
        (*(*g).clos)
            .callbacks_enabled = (0 as libc::c_int == 0) as libc::c_int as libc::c_uchar;
        pending = agbindrec(
            g as *mut libc::c_void,
            DRName.as_mut_ptr(),
            ::std::mem::size_of::<pendingset_t>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        ) as *mut pendingset_t;
        cb((*pending).ins.g, CB_INITIALIZE);
        cb((*pending).ins.n, CB_INITIALIZE);
        cb((*pending).ins.e, CB_INITIALIZE);
        cb((*pending).mod_0.g, CB_UPDATE);
        cb((*pending).mod_0.n, CB_UPDATE);
        cb((*pending).mod_0.e, CB_UPDATE);
        cb((*pending).del.e, CB_DELETION);
        cb((*pending).del.n, CB_DELETION);
        cb((*pending).del.g, CB_DELETION);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agcallbacks(
    mut g: *mut Agraph_t,
    mut flag: libc::c_int,
) -> libc::c_int {
    if flag != 0 && (*(*g).clos).callbacks_enabled == 0 {
        agrelease_callbacks(g);
    }
    if (*(*g).clos).callbacks_enabled != 0 {
        (*(*g).clos)
            .callbacks_enabled = (flag != 0 as libc::c_int) as libc::c_int
            as libc::c_uchar;
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    (*(*g).clos)
        .callbacks_enabled = (flag != 0 as libc::c_int) as libc::c_int as libc::c_uchar;
    return 0 as libc::c_int;
}
