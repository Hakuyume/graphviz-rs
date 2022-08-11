#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut Dttree: *mut Dtmethod_t;
    fn dtview(_: *mut Dt_t, _: *mut Dt_t) -> *mut Dt_t;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn aghtmlstr(_: *const libc::c_char) -> libc::c_int;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    static mut Ag_G_global: *mut Agraph_t;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    fn agdtopen(
        g: *mut Agraph_t,
        disc: *mut Dtdisc_t,
        method: *mut Dtmethod_t,
    ) -> *mut Dict_t;
    fn agapply(
        g: *mut Agraph_t,
        obj: *mut Agobj_t,
        fn_0: agobjfn_t,
        arg: *mut libc::c_void,
        preorder: libc::c_int,
    ) -> libc::c_int;
    fn abort() -> !;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agmarkhtmlstr(s: *mut libc::c_char);
    fn agdelrec(obj: *mut libc::c_void, name: *const libc::c_char) -> libc::c_int;
    fn agmethod_upd(g: *mut Agraph_t, obj: *mut libc::c_void, sym: *mut Agsym_t);
    fn agdtclose(g: *mut Agraph_t, dict: *mut Dict_t) -> libc::c_int;
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
#[derive(Copy, Clone, BitfieldStruct)]
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
#[derive(Copy, Clone, BitfieldStruct)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnode_s {
    pub base: Agobj_t,
    pub root: *mut Agraph_t,
    pub mainsub: Agsubnode_t,
}
pub type Agsubnode_t = Agsubnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agsubnode_s {
    pub seq_link: Dtlink_t,
    pub id_link: Dtlink_t,
    pub node: *mut Agnode_t,
    pub in_id: *mut Dtlink_t,
    pub out_id: *mut Dtlink_t,
    pub in_seq: *mut Dtlink_t,
    pub out_seq: *mut Dtlink_t,
}
pub type Agnode_t = Agnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agedge_s {
    pub base: Agobj_t,
    pub id_link: Dtlink_t,
    pub seq_link: Dtlink_t,
    pub node: *mut Agnode_t,
}
pub type Agedge_t = Agedge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agattr_s {
    pub h: Agrec_t,
    pub dict: *mut Dict_t,
    pub str_0: *mut *mut libc::c_char,
}
pub type Agattr_t = Agattr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdatadict_s {
    pub h: Agrec_t,
    pub dict: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub n: *mut Dict_t,
    pub e: *mut Dict_t,
    pub g: *mut Dict_t,
}
pub type Agdatadict_t = Agdatadict_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
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
#[inline]
unsafe extern "C" fn streq(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> bool {
    return strcmp(a, b) == 0 as libc::c_int;
}
#[no_mangle]
pub static mut AgDataDictDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: Some(
                freesym
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
static mut DataDictName: [libc::c_char; 13] = unsafe {
    *::std::mem::transmute::<&[u8; 13], &mut [libc::c_char; 13]>(b"_AG_datadict\0")
};
static mut ProtoDesc: Agdesc_t = Agdesc_t {
    directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
    c2rust_padding: [0; 3],
};
static mut ProtoGraph: *mut Agraph_t = 0 as *const Agraph_t as *mut Agraph_t;
#[no_mangle]
pub unsafe extern "C" fn agdatadict(
    mut g: *mut Agraph_t,
    mut cflag: libc::c_int,
) -> *mut Agdatadict_t {
    let mut rv: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    rv = aggetrec(g as *mut libc::c_void, DataDictName.as_mut_ptr(), 0 as libc::c_int)
        as *mut Agdatadict_t;
    if !rv.is_null() || cflag == 0 {
        return rv;
    }
    init_all_attrs(g);
    rv = aggetrec(g as *mut libc::c_void, DataDictName.as_mut_ptr(), 0 as libc::c_int)
        as *mut Agdatadict_t;
    return rv;
}
unsafe extern "C" fn agdictof(
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
) -> *mut Dict_t {
    let mut dd: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    let mut dict: *mut Dict_t = 0 as *mut Dict_t;
    dd = agdatadict(g, 0 as libc::c_int);
    if !dd.is_null() {
        match kind {
            0 => {
                dict = (*dd).dict.g;
            }
            1 => {
                dict = (*dd).dict.n;
            }
            3 | 2 => {
                dict = (*dd).dict.e;
            }
            _ => {
                agerr(
                    AGERR,
                    b"agdictof: unknown kind %d\n\0" as *const u8 as *const libc::c_char,
                    kind,
                );
                dict = 0 as *mut Dict_t;
            }
        }
    } else {
        dict = 0 as *mut Dict_t;
    }
    return dict;
}
unsafe extern "C" fn agnewsym(
    mut g: *mut Agraph_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut id: libc::c_int,
    mut kind: libc::c_int,
) -> *mut Agsym_t {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    sym = agalloc(g, ::std::mem::size_of::<Agsym_t>() as libc::c_ulong) as *mut Agsym_t;
    (*sym).kind = kind as libc::c_uchar;
    let ref mut fresh0 = (*sym).name;
    *fresh0 = agstrdup(g, name);
    let ref mut fresh1 = (*sym).defval;
    *fresh1 = agstrdup(g, value);
    (*sym).id = id;
    return sym;
}
unsafe extern "C" fn agcopydict(
    mut src: *mut Dict_t,
    mut dest: *mut Dict_t,
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
) {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut newsym: *mut Agsym_t = 0 as *mut Agsym_t;
    if dtsize(dest) == 0 as libc::c_int {} else {
        __assert_fail(
            b"dtsize(dest) == 0\0" as *const u8 as *const libc::c_char,
            b"attr.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void agcopydict(Dict_t *, Dict_t *, Agraph_t *, int)\0"))
                .as_ptr(),
        );
    }
    sym = (Some(((*(src as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(src, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agsym_t;
    while !sym.is_null() {
        newsym = agnewsym(g, (*sym).name, (*sym).defval, (*sym).id, kind);
        (*newsym).print = (*sym).print;
        (*newsym).fixed = (*sym).fixed;
        (Some(((*(dest as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(dest, newsym as *mut libc::c_void, 0o1 as libc::c_int);
        sym = (Some(((*(src as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(src, sym as *mut libc::c_void, 0o10 as libc::c_int) as *mut Agsym_t;
    }
}
unsafe extern "C" fn agmakedatadict(mut g: *mut Agraph_t) -> *mut Agdatadict_t {
    let mut par: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut parent_dd: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    let mut dd: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    dd = agbindrec(
        g as *mut libc::c_void,
        DataDictName.as_mut_ptr(),
        ::std::mem::size_of::<Agdatadict_t>() as libc::c_ulong as libc::c_uint,
        0 as libc::c_int,
    ) as *mut Agdatadict_t;
    let ref mut fresh2 = (*dd).dict.n;
    *fresh2 = agdtopen(g, &mut AgDataDictDisc, Dttree);
    let ref mut fresh3 = (*dd).dict.e;
    *fresh3 = agdtopen(g, &mut AgDataDictDisc, Dttree);
    let ref mut fresh4 = (*dd).dict.g;
    *fresh4 = agdtopen(g, &mut AgDataDictDisc, Dttree);
    par = agparent(g);
    if !par.is_null() {
        parent_dd = agdatadict(par, 0 as libc::c_int);
        if dd != parent_dd {} else {
            __assert_fail(
                b"dd != parent_dd\0" as *const u8 as *const libc::c_char,
                b"attr.c\0" as *const u8 as *const libc::c_char,
                116 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"Agdatadict_t *agmakedatadict(Agraph_t *)\0"))
                    .as_ptr(),
            );
        }
        dtview((*dd).dict.n, (*parent_dd).dict.n);
        dtview((*dd).dict.e, (*parent_dd).dict.e);
        dtview((*dd).dict.g, (*parent_dd).dict.g);
    } else if !ProtoGraph.is_null() && g != ProtoGraph {
        parent_dd = agdatadict(ProtoGraph, 0 as libc::c_int);
        agcopydict((*parent_dd).dict.n, (*dd).dict.n, g, 1 as libc::c_int);
        agcopydict((*parent_dd).dict.e, (*dd).dict.e, g, 2 as libc::c_int);
        agcopydict((*parent_dd).dict.g, (*dd).dict.g, g, 0 as libc::c_int);
    }
    return dd;
}
unsafe extern "C" fn agdictsym(
    mut dict: *mut Dict_t,
    mut name: *mut libc::c_char,
) -> *mut Agsym_t {
    let mut key: Agsym_t = Agsym_t {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        name: 0 as *mut libc::c_char,
        defval: 0 as *mut libc::c_char,
        id: 0,
        kind: 0,
        fixed: 0,
        print: 0,
    };
    key.name = name;
    return (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(dict, &mut key as *mut Agsym_t as *mut libc::c_void, 0o4 as libc::c_int)
        as *mut Agsym_t;
}
unsafe extern "C" fn aglocaldictsym(
    mut dict: *mut Dict_t,
    mut name: *mut libc::c_char,
) -> *mut Agsym_t {
    let mut rv: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut view: *mut Dict_t = 0 as *mut Dict_t;
    view = dtview(dict, 0 as *mut Dt_t);
    rv = agdictsym(dict, name);
    dtview(dict, view);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agattrsym(
    mut obj: *mut libc::c_void,
    mut name: *mut libc::c_char,
) -> *mut Agsym_t {
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut rv: *mut Agsym_t = 0 as *mut Agsym_t;
    data = agattrrec(obj);
    if !data.is_null() {
        rv = agdictsym((*data).dict, name);
    } else {
        rv = 0 as *mut Agsym_t;
    }
    return rv;
}
#[no_mangle]
pub static mut AgDataRecName: *mut libc::c_char = b"_AG_strdata\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn topdictsize(mut obj: *mut Agobj_t) -> libc::c_int {
    let mut d: *mut Dict_t = 0 as *mut Dict_t;
    d = agdictof(
        agroot(agraphof(obj as *mut libc::c_void) as *mut libc::c_void),
        ((*obj).tag).objtype() as libc::c_int,
    );
    return if !d.is_null() { dtsize(d) } else { 0 as libc::c_int };
}
unsafe extern "C" fn agmakeattrs(
    mut context: *mut Agraph_t,
    mut obj: *mut libc::c_void,
) -> *mut Agrec_t {
    let mut sz: libc::c_int = 0;
    let mut rec: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut datadict: *mut Dict_t = 0 as *mut Dict_t;
    rec = agbindrec(
        obj,
        AgDataRecName,
        ::std::mem::size_of::<Agattr_t>() as libc::c_ulong as libc::c_uint,
        0 as libc::c_int,
    ) as *mut Agattr_t;
    datadict = agdictof(
        context,
        ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int,
    );
    if !datadict.is_null() {} else {
        __assert_fail(
            b"datadict\0" as *const u8 as *const libc::c_char,
            b"attr.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"Agrec_t *agmakeattrs(Agraph_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if ((*rec).dict).is_null() {
        let ref mut fresh5 = (*rec).dict;
        *fresh5 = agdictof(
            agroot(context as *mut libc::c_void),
            ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int,
        );
        sz = topdictsize(obj as *mut Agobj_t);
        if sz < 4 as libc::c_int {
            sz = 4 as libc::c_int;
        }
        let ref mut fresh6 = (*rec).str_0;
        *fresh6 = agalloc(
            agraphof(obj),
            (sz as size_t)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        sym = (Some(
            ((*(datadict as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(datadict, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agsym_t;
        while !sym.is_null() {
            let ref mut fresh7 = *((*rec).str_0).offset((*sym).id as isize);
            *fresh7 = agstrdup(agraphof(obj), (*sym).defval);
            sym = (Some(
                ((*(datadict as *mut Dt_t)).searchf).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(datadict, sym as *mut libc::c_void, 0o10 as libc::c_int)
                as *mut Agsym_t;
        }
    } else if (*rec).dict == datadict {} else {
        __assert_fail(
            b"rec->dict == datadict\0" as *const u8 as *const libc::c_char,
            b"attr.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"Agrec_t *agmakeattrs(Agraph_t *, void *)\0"))
                .as_ptr(),
        );
    }
    return rec as *mut Agrec_t;
}
unsafe extern "C" fn freeattr(mut obj: *mut Agobj_t, mut attr: *mut Agattr_t) {
    let mut i: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    g = agraphof(obj as *mut libc::c_void);
    sz = topdictsize(obj);
    i = 0 as libc::c_int;
    while i < sz {
        agstrfree(g, *((*attr).str_0).offset(i as isize));
        i += 1;
    }
    agfree(g, (*attr).str_0 as *mut libc::c_void);
}
unsafe extern "C" fn freesym(
    mut d: *mut Dict_t,
    mut obj: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    sym = obj as *mut Agsym_t;
    agstrfree(Ag_G_global, (*sym).name);
    agstrfree(Ag_G_global, (*sym).defval);
    agfree(Ag_G_global, sym as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn agattrrec(mut obj: *mut libc::c_void) -> *mut Agattr_t {
    return aggetrec(obj, AgDataRecName, 0 as libc::c_int) as *mut Agattr_t;
}
unsafe extern "C" fn addattr(
    mut g: *mut Agraph_t,
    mut obj: *mut Agobj_t,
    mut sym: *mut Agsym_t,
) {
    let mut attr: *mut Agattr_t = agattrrec(obj as *mut libc::c_void);
    if !attr.is_null() {} else {
        __assert_fail(
            b"attr != NULL\0" as *const u8 as *const libc::c_char,
            b"attr.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void addattr(Agraph_t *, Agobj_t *, Agsym_t *)\0"))
                .as_ptr(),
        );
    }
    if (*sym).id >= 4 as libc::c_int {
        let ref mut fresh8 = (*attr).str_0;
        *fresh8 = ((*(*(*g).clos).disc.mem).resize)
            .expect(
                "non-null function pointer",
            )(
            (*(*g).clos).state.mem,
            (*attr).str_0 as *mut libc::c_void,
            ((*sym).id as size_t)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
            ((*sym).id as size_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
    }
    let ref mut fresh9 = *((*attr).str_0).offset((*sym).id as isize);
    *fresh9 = agstrdup(g, (*sym).defval);
}
unsafe extern "C" fn getattr(
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
    mut name: *mut libc::c_char,
) -> *mut Agsym_t {
    let mut rv: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut dict: *mut Dict_t = 0 as *mut Dict_t;
    dict = agdictof(g, kind);
    if !dict.is_null() {
        rv = agdictsym(dict, name);
    }
    return rv;
}
unsafe extern "C" fn unviewsubgraphsattr(
    mut parent: *mut Agraph_t,
    mut name: *mut libc::c_char,
) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut psym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut lsym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut ldict: *mut Dict_t = 0 as *mut Dict_t;
    psym = getattr(parent, 0 as libc::c_int, name);
    if psym.is_null() {
        return;
    }
    subg = agfstsubg(parent);
    while !subg.is_null() {
        ldict = (*agdatadict(subg, (0 as libc::c_int == 0) as libc::c_int)).dict.g;
        lsym = aglocaldictsym(ldict, name);
        if lsym.is_null() {
            lsym = agnewsym(
                agroot(subg as *mut libc::c_void),
                name,
                agxget(subg as *mut libc::c_void, psym),
                (*psym).id,
                0 as libc::c_int,
            );
            (Some(((*(ldict as *mut Dt_t)).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(ldict, lsym as *mut libc::c_void, 0o1 as libc::c_int);
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn setattr(
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
    mut name: *mut libc::c_char,
    mut value: *const libc::c_char,
) -> *mut Agsym_t {
    let mut ldict: *mut Dict_t = 0 as *mut Dict_t;
    let mut rdict: *mut Dict_t = 0 as *mut Dict_t;
    let mut lsym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut psym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut rsym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut rv: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut root: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    if !value.is_null() {} else {
        __assert_fail(
            b"value\0" as *const u8 as *const libc::c_char,
            b"attr.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"Agsym_t *setattr(Agraph_t *, int, char *, const char *)\0"))
                .as_ptr(),
        );
    }
    root = agroot(g as *mut libc::c_void);
    agdatadict(g, (0 as libc::c_int == 0) as libc::c_int);
    ldict = agdictof(g, kind);
    lsym = aglocaldictsym(ldict, name);
    if !lsym.is_null() {
        if g != root
            && streq(name, b"layout\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0
        {
            agerr(
                AGWARN,
                b"layout attribute is invalid except on the root graph\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if kind == 0 as libc::c_int {
            unviewsubgraphsattr(g, name);
        }
        agstrfree(g, (*lsym).defval);
        let ref mut fresh10 = (*lsym).defval;
        *fresh10 = agstrdup(g, value);
        rv = lsym;
    } else {
        psym = agdictsym(ldict, name);
        if !psym.is_null() {
            lsym = agnewsym(g, name, value, (*psym).id, kind);
            (Some(((*(ldict as *mut Dt_t)).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(ldict, lsym as *mut libc::c_void, 0o1 as libc::c_int);
            rv = lsym;
        } else {
            rdict = agdictof(root, kind);
            rsym = agnewsym(g, name, value, dtsize(rdict), kind);
            (Some(((*(rdict as *mut Dt_t)).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(rdict, rsym as *mut libc::c_void, 0o1 as libc::c_int);
            match kind {
                0 => {
                    agapply(
                        root,
                        root as *mut Agobj_t,
                        ::std::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    *mut Agraph_t,
                                    *mut Agobj_t,
                                    *mut Agsym_t,
                                ) -> (),
                            >,
                            agobjfn_t,
                        >(
                            Some(
                                addattr
                                    as unsafe extern "C" fn(
                                        *mut Agraph_t,
                                        *mut Agobj_t,
                                        *mut Agsym_t,
                                    ) -> (),
                            ),
                        ),
                        rsym as *mut libc::c_void,
                        (0 as libc::c_int == 0) as libc::c_int,
                    );
                }
                1 => {
                    n = agfstnode(root);
                    while !n.is_null() {
                        addattr(g, n as *mut Agobj_t, rsym);
                        n = agnxtnode(root, n);
                    }
                }
                3 | 2 => {
                    n = agfstnode(root);
                    while !n.is_null() {
                        e = agfstout(root, n);
                        while !e.is_null() {
                            addattr(g, e as *mut Agobj_t, rsym);
                            e = agnxtout(root, e);
                        }
                        n = agnxtnode(root, n);
                    }
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s:%d: claimed unreachable code was reached\0" as *const u8
                            as *const libc::c_char,
                        b"attr.c\0" as *const u8 as *const libc::c_char,
                        326 as libc::c_int,
                    );
                    abort();
                }
            }
            rv = rsym;
        }
    }
    if !rv.is_null() && kind == 0 as libc::c_int {
        agxset(g as *mut libc::c_void, rv, value);
    }
    agmethod_upd(g, g as *mut libc::c_void, rv);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agattr(
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
    mut name: *mut libc::c_char,
    mut value: *const libc::c_char,
) -> *mut Agsym_t {
    let mut rv: *mut Agsym_t = 0 as *mut Agsym_t;
    if g.is_null() {
        if ProtoGraph.is_null() {
            ProtoGraph = agopen(0 as *mut libc::c_char, ProtoDesc, 0 as *mut Agdisc_t);
        }
        g = ProtoGraph;
    }
    if !value.is_null() {
        rv = setattr(g, kind, name, value);
    } else {
        rv = getattr(g, kind, name);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agnxtattr(
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
    mut attr: *mut Agsym_t,
) -> *mut Agsym_t {
    let mut d: *mut Dict_t = 0 as *mut Dict_t;
    let mut rv: *mut Agsym_t = 0 as *mut Agsym_t;
    d = agdictof(g, kind);
    if !d.is_null() {
        if !attr.is_null() {
            rv = (Some(
                ((*(d as *mut Dt_t)).searchf).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(d, attr as *mut libc::c_void, 0o10 as libc::c_int) as *mut Agsym_t;
        } else {
            rv = (Some(
                ((*(d as *mut Dt_t)).searchf).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(d, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agsym_t;
        }
    } else {
        rv = 0 as *mut Agsym_t;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agraphattr_init(mut g: *mut Agraph_t) {
    let mut context: *mut Agraph_t = 0 as *mut Agraph_t;
    let ref mut fresh11 = (*g).desc;
    (*fresh11).set_has_attrs(1 as libc::c_int as libc::c_uint);
    agmakedatadict(g);
    context = agparent(g);
    if context.is_null() {
        context = g;
    }
    agmakeattrs(context, g as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn agraphattr_delete(mut g: *mut Agraph_t) -> libc::c_int {
    let mut dd: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    let mut attr: *mut Agattr_t = 0 as *mut Agattr_t;
    Ag_G_global = g;
    attr = agattrrec(g as *mut libc::c_void);
    if !attr.is_null() {
        freeattr(g as *mut Agobj_t, attr);
        agdelrec(g as *mut libc::c_void, (*attr).h.name);
    }
    dd = agdatadict(g, 0 as libc::c_int);
    if !dd.is_null() {
        if agdtclose(g, (*dd).dict.n) != 0 {
            return 1 as libc::c_int;
        }
        if agdtclose(g, (*dd).dict.e) != 0 {
            return 1 as libc::c_int;
        }
        if agdtclose(g, (*dd).dict.g) != 0 {
            return 1 as libc::c_int;
        }
        agdelrec(g as *mut libc::c_void, (*dd).h.name);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agnodeattr_init(mut g: *mut Agraph_t, mut n: *mut Agnode_t) {
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    data = agattrrec(n as *mut libc::c_void);
    if data.is_null() || ((*data).dict).is_null() {
        agmakeattrs(g, n as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agnodeattr_delete(mut n: *mut Agnode_t) {
    let mut rec: *mut Agattr_t = 0 as *mut Agattr_t;
    rec = agattrrec(n as *mut libc::c_void);
    if !rec.is_null() {
        freeattr(n as *mut Agobj_t, rec);
        agdelrec(n as *mut libc::c_void, AgDataRecName);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agedgeattr_init(mut g: *mut Agraph_t, mut e: *mut Agedge_t) {
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    data = agattrrec(e as *mut libc::c_void);
    if data.is_null() || ((*data).dict).is_null() {
        agmakeattrs(g, e as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agedgeattr_delete(mut e: *mut Agedge_t) {
    let mut rec: *mut Agattr_t = 0 as *mut Agattr_t;
    rec = agattrrec(e as *mut libc::c_void);
    if !rec.is_null() {
        freeattr(e as *mut Agobj_t, rec);
        agdelrec(e as *mut libc::c_void, AgDataRecName);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agget(
    mut obj: *mut libc::c_void,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    sym = agattrsym(obj, name);
    if sym.is_null() {
        rv = 0 as *mut libc::c_char;
    } else {
        data = agattrrec(obj);
        rv = *((*data).str_0).offset((*sym).id as isize);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agxget(
    mut obj: *mut libc::c_void,
    mut sym: *mut Agsym_t,
) -> *mut libc::c_char {
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    data = agattrrec(obj);
    if (*sym).id >= 0 as libc::c_int && (*sym).id < topdictsize(obj as *mut Agobj_t)
    {} else {
        __assert_fail(
            b"sym->id >= 0 && sym->id < topdictsize(obj)\0" as *const u8
                as *const libc::c_char,
            b"attr.c\0" as *const u8 as *const libc::c_char,
            466 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"char *agxget(void *, Agsym_t *)\0"))
                .as_ptr(),
        );
    }
    rv = *((*data).str_0).offset((*sym).id as isize);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agset(
    mut obj: *mut libc::c_void,
    mut name: *mut libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut rv: libc::c_int = 0;
    sym = agattrsym(obj, name);
    if sym.is_null() {
        rv = -(1 as libc::c_int);
    } else {
        rv = agxset(obj, sym, value);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agxset(
    mut obj: *mut libc::c_void,
    mut sym: *mut Agsym_t,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut hdr: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut lsym: *mut Agsym_t = 0 as *mut Agsym_t;
    g = agraphof(obj);
    hdr = obj as *mut Agobj_t;
    data = agattrrec(hdr as *mut libc::c_void);
    if (*sym).id >= 0 as libc::c_int && (*sym).id < topdictsize(obj as *mut Agobj_t)
    {} else {
        __assert_fail(
            b"sym->id >= 0 && sym->id < topdictsize(obj)\0" as *const u8
                as *const libc::c_char,
            b"attr.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int agxset(void *, Agsym_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    agstrfree(g, *((*data).str_0).offset((*sym).id as isize));
    let ref mut fresh12 = *((*data).str_0).offset((*sym).id as isize);
    *fresh12 = agstrdup(g, value);
    if ((*hdr).tag).objtype() as libc::c_int == 0 as libc::c_int {
        let mut dict: *mut Dict_t = 0 as *mut Dict_t;
        dict = (*agdatadict(g, 0 as libc::c_int)).dict.g;
        lsym = aglocaldictsym(dict, (*sym).name);
        if !lsym.is_null() {
            agstrfree(g, (*lsym).defval);
            let ref mut fresh13 = (*lsym).defval;
            *fresh13 = agstrdup(g, value);
        } else {
            lsym = agnewsym(
                g,
                (*sym).name,
                value,
                (*sym).id,
                ((*hdr).tag).objtype() as libc::c_int,
            );
            (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(dict, lsym as *mut libc::c_void, 0o1 as libc::c_int);
        }
    }
    agmethod_upd(g, obj, sym);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agsafeset(
    mut obj: *mut libc::c_void,
    mut name: *mut libc::c_char,
    mut value: *const libc::c_char,
    mut def: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut Agsym_t = 0 as *mut Agsym_t;
    a = agattr(
        agraphof(obj),
        ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int,
        name,
        0 as *const libc::c_char,
    );
    if a.is_null() {
        a = agattr(
            agraphof(obj),
            ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int,
            name,
            def,
        );
    }
    return agxset(obj, a, value);
}
unsafe extern "C" fn init_all_attrs(mut g: *mut Agraph_t) {
    let mut root: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    root = agroot(g as *mut libc::c_void);
    agapply(
        root,
        root as *mut Agobj_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Agraph_t) -> ()>,
            agobjfn_t,
        >(Some(agraphattr_init as unsafe extern "C" fn(*mut Agraph_t) -> ())),
        0 as *mut libc::c_void,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    n = agfstnode(root);
    while !n.is_null() {
        agnodeattr_init(g, n);
        e = agfstout(root, n);
        while !e.is_null() {
            agedgeattr_init(g, e);
            e = agnxtout(root, e);
        }
        n = agnxtnode(root, n);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agcopyattr(
    mut oldobj: *mut libc::c_void,
    mut newobj: *mut libc::c_void,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut newsym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 1 as libc::c_int;
    g = agraphof(oldobj);
    if ((*(oldobj as *mut Agobj_t)).tag).objtype() as libc::c_int
        != ((*(newobj as *mut Agobj_t)).tag).objtype() as libc::c_int
    {
        return 1 as libc::c_int;
    }
    sym = 0 as *mut Agsym_t;
    loop {
        sym = agnxtattr(
            g,
            ((*(oldobj as *mut Agobj_t)).tag).objtype() as libc::c_int,
            sym,
        );
        if sym.is_null() {
            break;
        }
        newsym = agattrsym(newobj, (*sym).name);
        if newsym.is_null() {
            return 1 as libc::c_int;
        }
        val = agxget(oldobj, sym);
        r = agxset(newobj, newsym, val);
        if aghtmlstr(val) != 0 {
            nval = agxget(newobj, newsym);
            agmarkhtmlstr(nval);
        }
    }
    return r;
}
unsafe extern "C" fn run_static_initializers() {
    ProtoDesc = {
        let mut init = Agdesc_s {
            directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_directed(1 as libc::c_int as libc::c_uint);
        init.set_strict(0 as libc::c_int as libc::c_uint);
        init.set_no_loop(1 as libc::c_int as libc::c_uint);
        init.set_maingraph(0 as libc::c_int as libc::c_uint);
        init.set_flatlock(1 as libc::c_int as libc::c_uint);
        init.set_no_write(1 as libc::c_int as libc::c_uint);
        init.set_has_attrs(0 as libc::c_int as libc::c_uint);
        init.set_has_cmpnd(0 as libc::c_int as libc::c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
