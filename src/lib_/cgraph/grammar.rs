#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut AgDefaultDisc: Agdisc_t;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    static mut Ag_G_global: *mut Agraph_t;
    fn aglexbad();
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abort() -> !;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agsubnode(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn aglexeof();
    fn aaglex() -> libc::c_int;
    fn aglexinit(disc: *mut Agdisc_t, ifile: *mut libc::c_void);
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn aginternalmapclearlocalnames(g: *mut Agraph_t);
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn aagerror(_: *const libc::c_char);
    static mut aagin: *mut FILE;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type FILE = _IO_FILE;
pub type uint64_t = __uint64_t;
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
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub union AAGSTYPE {
    pub i: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub n: *mut Agnode_s,
}
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
pub type gstack_t = gstack_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gstack_s {
    pub g: *mut Agraph_t,
    pub subg: *mut Agraph_t,
    pub nodelist: list_t,
    pub edgelist: list_t,
    pub attrlist: list_t,
    pub down: *mut gstack_s,
}
pub type list_t = list_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_s {
    pub first: *mut item,
    pub last: *mut item,
}
pub type item = item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item_s {
    pub tag: libc::c_int,
    pub u: val_t,
    pub str_0: *mut libc::c_char,
    pub next: *mut item_s,
}
pub type val_t = s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union s {
    pub n: *mut Agnode_t,
    pub subg: *mut Agraph_t,
    pub e: *mut Agedge_t,
    pub asym: *mut Agsym_t,
    pub name: *mut libc::c_char,
    pub list: *mut item_s,
}
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: AAGSTYPE,
}
#[inline]
unsafe extern "C" fn streq(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> bool {
    return strcmp(a, b) == 0 as libc::c_int;
}
static mut Key: [libc::c_char; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"key\0")
};
static mut SubgraphDepth: libc::c_int = 0 as libc::c_int;
static mut G: *mut Agraph_t = 0 as *const Agraph_t as *mut Agraph_t;
static mut Disc: *mut Agdisc_t = 0 as *const Agdisc_t as *mut Agdisc_t;
static mut S: *mut gstack_t = 0 as *const gstack_t as *mut gstack_t;
unsafe extern "C" fn newitem(
    mut tag: libc::c_int,
    mut p0: *mut libc::c_void,
    mut p1: *mut libc::c_char,
) -> *mut item {
    let mut rv: *mut item = agalloc(G, ::std::mem::size_of::<item>() as libc::c_ulong)
        as *mut item;
    (*rv).tag = tag;
    let ref mut fresh0 = (*rv).u.name;
    *fresh0 = p0 as *mut libc::c_char;
    let ref mut fresh1 = (*rv).str_0;
    *fresh1 = p1;
    return rv;
}
unsafe extern "C" fn cons_node(
    mut n: *mut Agnode_t,
    mut port: *mut libc::c_char,
) -> *mut item {
    return newitem(259 as libc::c_int, n as *mut libc::c_void, port);
}
unsafe extern "C" fn cons_attr(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> *mut item {
    return newitem(267 as libc::c_int, name as *mut libc::c_void, value);
}
unsafe extern "C" fn cons_list(mut list: *mut item) -> *mut item {
    return newitem(
        265 as libc::c_int,
        list as *mut libc::c_void,
        0 as *mut libc::c_char,
    );
}
unsafe extern "C" fn cons_subg(mut subg: *mut Agraph_t) -> *mut item {
    return newitem(
        262 as libc::c_int,
        subg as *mut libc::c_void,
        0 as *mut libc::c_char,
    );
}
unsafe extern "C" fn push(
    mut s: *mut gstack_t,
    mut subg: *mut Agraph_t,
) -> *mut gstack_t {
    let mut rv: *mut gstack_t = 0 as *mut gstack_t;
    rv = agalloc(G, ::std::mem::size_of::<gstack_t>() as libc::c_ulong) as *mut gstack_t;
    let ref mut fresh2 = (*rv).down;
    *fresh2 = s;
    let ref mut fresh3 = (*rv).g;
    *fresh3 = subg;
    return rv;
}
unsafe extern "C" fn pop(mut s: *mut gstack_t) -> *mut gstack_t {
    let mut rv: *mut gstack_t = 0 as *mut gstack_t;
    rv = (*s).down;
    agfree(G, s as *mut libc::c_void);
    return rv;
}
unsafe extern "C" fn delete_items(mut ilist: *mut item) {
    let mut p: *mut item = 0 as *mut item;
    let mut pn: *mut item = 0 as *mut item;
    p = ilist;
    while !p.is_null() {
        pn = (*p).next;
        match (*p).tag {
            265 => {
                delete_items((*p).u.list);
            }
            267 | 266 => {
                agstrfree(G, (*p).str_0);
            }
            _ => {}
        }
        agfree(G, p as *mut libc::c_void);
        p = pn;
    }
}
unsafe extern "C" fn deletelist(mut list: *mut list_t) {
    delete_items((*list).first);
    let ref mut fresh4 = (*list).last;
    *fresh4 = 0 as *mut item;
    let ref mut fresh5 = (*list).first;
    *fresh5 = *fresh4;
}
unsafe extern "C" fn listapp(mut list: *mut list_t, mut v: *mut item) {
    if !((*list).last).is_null() {
        let ref mut fresh6 = (*(*list).last).next;
        *fresh6 = v;
    }
    let ref mut fresh7 = (*list).last;
    *fresh7 = v;
    if ((*list).first).is_null() {
        let ref mut fresh8 = (*list).first;
        *fresh8 = v;
    }
}
unsafe extern "C" fn appendattr(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    let mut v: *mut item = 0 as *mut item;
    if !value.is_null() {} else {
        __assert_fail(
            b"value != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/grammar.y\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void appendattr(char *, char *)\0"))
                .as_ptr(),
        );
    }
    v = cons_attr(name, value);
    listapp(&mut (*S).attrlist, v);
}
unsafe extern "C" fn bindattrs(mut kind: libc::c_int) {
    let mut aptr: *mut item = 0 as *mut item;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    aptr = (*S).attrlist.first;
    while !aptr.is_null() {
        if (*aptr).tag == 267 as libc::c_int {} else {
            __assert_fail(
                b"aptr->tag == T_atom\0" as *const u8 as *const libc::c_char,
                b"../../lib/cgraph/grammar.y\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"void bindattrs(int)\0"))
                    .as_ptr(),
            );
        }
        name = (*aptr).u.name;
        if !(kind == 2 as libc::c_int
            && streq(name, Key.as_mut_ptr()) as libc::c_int != 0)
        {
            let ref mut fresh9 = (*aptr).u.asym;
            *fresh9 = agattr((*S).g, kind, name, 0 as *const libc::c_char);
            if (*fresh9).is_null() {
                let ref mut fresh10 = (*aptr).u.asym;
                *fresh10 = agattr(
                    (*S).g,
                    kind,
                    name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            (*aptr).tag = 266 as libc::c_int;
            agstrfree(G, name);
        }
        aptr = (*aptr).next;
    }
}
unsafe extern "C" fn applyattrs(mut obj: *mut libc::c_void) {
    let mut aptr: *mut item = 0 as *mut item;
    aptr = (*S).attrlist.first;
    while !aptr.is_null() {
        if (*aptr).tag == 266 as libc::c_int {
            if !((*aptr).u.asym).is_null() {
                agxset(obj, (*aptr).u.asym, (*aptr).str_0);
            }
        } else {
            if ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
                || ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
            {} else {
                __assert_fail(
                    b"AGTYPE(obj) == AGINEDGE || AGTYPE(obj) == AGOUTEDGE\0" as *const u8
                        as *const libc::c_char,
                    b"../../lib/cgraph/grammar.y\0" as *const u8 as *const libc::c_char,
                    301 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"void applyattrs(void *)\0"))
                        .as_ptr(),
                );
            }
            if (*aptr).tag == 267 as libc::c_int {} else {
                __assert_fail(
                    b"aptr->tag == T_atom\0" as *const u8 as *const libc::c_char,
                    b"../../lib/cgraph/grammar.y\0" as *const u8 as *const libc::c_char,
                    302 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"void applyattrs(void *)\0"))
                        .as_ptr(),
                );
            }
            if streq((*aptr).u.name, Key.as_mut_ptr()) {} else {
                __assert_fail(
                    b"streq(aptr->u.name,Key)\0" as *const u8 as *const libc::c_char,
                    b"../../lib/cgraph/grammar.y\0" as *const u8 as *const libc::c_char,
                    303 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"void applyattrs(void *)\0"))
                        .as_ptr(),
                );
            }
        }
        aptr = (*aptr).next;
    }
}
unsafe extern "C" fn nomacros() {
    agerr(
        AGWARN,
        b"attribute macros not implemented\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn attrstmt(mut tkind: libc::c_int, mut macroname: *mut libc::c_char) {
    let mut aptr: *mut item = 0 as *mut item;
    let mut kind: libc::c_int = 0 as libc::c_int;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    if !macroname.is_null() {
        nomacros();
    }
    aptr = (*S).attrlist.first;
    while !aptr.is_null() {
        if ((*aptr).str_0).is_null() {
            nomacros();
        }
        aptr = (*aptr).next;
    }
    match tkind {
        258 => {
            kind = 0 as libc::c_int;
        }
        259 => {
            kind = 1 as libc::c_int;
        }
        260 => {
            kind = 2 as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"../../lib/cgraph/grammar.y\0" as *const u8 as *const libc::c_char,
                333 as libc::c_int,
            );
            abort();
        }
    }
    bindattrs(kind);
    aptr = (*S).attrlist.first;
    while !aptr.is_null() {
        if !((*aptr).tag == 267 as libc::c_int) {
            if (*(*aptr).u.asym).fixed == 0 || (*S).g != G {
                sym = agattr((*S).g, kind, (*(*aptr).u.asym).name, (*aptr).str_0);
            } else {
                sym = (*aptr).u.asym;
            }
            if (*S).g == G {
                (*sym).print = (0 as libc::c_int == 0) as libc::c_int as libc::c_uchar;
            }
        }
        aptr = (*aptr).next;
    }
    deletelist(&mut (*S).attrlist);
}
unsafe extern "C" fn appendnode(
    mut name: *mut libc::c_char,
    mut port: *mut libc::c_char,
    mut sport: *mut libc::c_char,
) {
    let mut elt: *mut item = 0 as *mut item;
    if !sport.is_null() {
        port = concatPort(port, sport);
    }
    elt = cons_node(agnode((*S).g, name, (0 as libc::c_int == 0) as libc::c_int), port);
    listapp(&mut (*S).nodelist, elt);
    agstrfree(G, name);
}
unsafe extern "C" fn endnode() {
    let mut ptr: *mut item = 0 as *mut item;
    bindattrs(1 as libc::c_int);
    ptr = (*S).nodelist.first;
    while !ptr.is_null() {
        applyattrs((*ptr).u.n as *mut libc::c_void);
        ptr = (*ptr).next;
    }
    deletelist(&mut (*S).nodelist);
    deletelist(&mut (*S).attrlist);
    deletelist(&mut (*S).edgelist);
    let ref mut fresh11 = (*S).subg;
    *fresh11 = 0 as *mut Agraph_t;
}
unsafe extern "C" fn getedgeitems() {
    let mut v: *mut item = 0 as *mut item;
    if !((*S).nodelist.first).is_null() {
        v = cons_list((*S).nodelist.first);
        let ref mut fresh12 = (*S).nodelist.last;
        *fresh12 = 0 as *mut item;
        let ref mut fresh13 = (*S).nodelist.first;
        *fresh13 = *fresh12;
    } else {
        if !((*S).subg).is_null() {
            v = cons_subg((*S).subg);
        }
        let ref mut fresh14 = (*S).subg;
        *fresh14 = 0 as *mut Agraph_t;
    }
    if !v.is_null() {
        listapp(&mut (*S).edgelist, v);
    }
}
unsafe extern "C" fn endedge() {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aptr: *mut item = 0 as *mut item;
    let mut tptr: *mut item = 0 as *mut item;
    let mut p: *mut item = 0 as *mut item;
    let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    bindattrs(2 as libc::c_int);
    key = 0 as *mut libc::c_char;
    aptr = (*S).attrlist.first;
    while !aptr.is_null() {
        if (*aptr).tag == 267 as libc::c_int
            && streq((*aptr).u.name, Key.as_mut_ptr()) as libc::c_int != 0
        {
            key = (*aptr).str_0;
        }
        aptr = (*aptr).next;
    }
    p = (*S).edgelist.first;
    while !((*p).next).is_null() {
        if (*p).tag == 262 as libc::c_int {
            subg = (*p).u.subg;
            t = agfstnode(subg);
            while !t.is_null() {
                edgerhs(
                    agsubnode((*S).g, t, 0 as libc::c_int),
                    0 as *mut libc::c_char,
                    (*p).next,
                    key,
                );
                t = agnxtnode(subg, t);
            }
        } else {
            tptr = (*p).u.list;
            while !tptr.is_null() {
                edgerhs((*tptr).u.n, (*tptr).str_0, (*p).next, key);
                tptr = (*tptr).next;
            }
        }
        p = (*p).next;
    }
    deletelist(&mut (*S).nodelist);
    deletelist(&mut (*S).edgelist);
    deletelist(&mut (*S).attrlist);
    let ref mut fresh15 = (*S).subg;
    *fresh15 = 0 as *mut Agraph_t;
}
unsafe extern "C" fn concat(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut sym: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = (strlen(s1))
        .wrapping_add(strlen(s2))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if len <= 8192 as libc::c_int as libc::c_ulong {
        sym = buf.as_mut_ptr();
    } else {
        sym = malloc(len) as *mut libc::c_char;
    }
    strcpy(sym, s1);
    strcat(sym, s2);
    s = agstrdup(G, sym);
    agstrfree(G, s1);
    agstrfree(G, s2);
    if sym != buf.as_mut_ptr() {
        free(sym as *mut libc::c_void);
    }
    return s;
}
unsafe extern "C" fn concatPort(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut sym: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = (strlen(s1))
        .wrapping_add(strlen(s2))
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    if len <= 8192 as libc::c_int as libc::c_ulong {
        sym = buf.as_mut_ptr();
    } else {
        sym = malloc(len) as *mut libc::c_char;
    }
    sprintf(sym, b"%s:%s\0" as *const u8 as *const libc::c_char, s1, s2);
    s = agstrdup(G, sym);
    agstrfree(G, s1);
    agstrfree(G, s2);
    if sym != buf.as_mut_ptr() {
        free(sym as *mut libc::c_void);
    }
    return s;
}
unsafe extern "C" fn edgerhs(
    mut tail: *mut Agnode_t,
    mut tport: *mut libc::c_char,
    mut hlist: *mut item,
    mut key: *mut libc::c_char,
) {
    let mut head: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut hptr: *mut item = 0 as *mut item;
    if (*hlist).tag == 262 as libc::c_int {
        subg = (*hlist).u.subg;
        head = agfstnode(subg);
        while !head.is_null() {
            newedge(
                tail,
                tport,
                agsubnode((*S).g, head, 0 as libc::c_int),
                0 as *mut libc::c_char,
                key,
            );
            head = agnxtnode(subg, head);
        }
    } else {
        hptr = (*hlist).u.list;
        while !hptr.is_null() {
            newedge(
                tail,
                tport,
                agsubnode((*S).g, (*hptr).u.n, 0 as libc::c_int),
                (*hptr).str_0,
                key,
            );
            hptr = (*hptr).next;
        }
    };
}
unsafe extern "C" fn mkport(
    mut e: *mut Agedge_t,
    mut name: *mut libc::c_char,
    mut val: *mut libc::c_char,
) {
    let mut attr: *mut Agsym_t = 0 as *mut Agsym_t;
    if !val.is_null() {
        attr = agattr((*S).g, 2 as libc::c_int, name, 0 as *const libc::c_char);
        if attr.is_null() {
            attr = agattr(
                (*S).g,
                2 as libc::c_int,
                name,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        agxset(e as *mut libc::c_void, attr, val);
    }
}
unsafe extern "C" fn newedge(
    mut t: *mut Agnode_t,
    mut tport: *mut libc::c_char,
    mut h: *mut Agnode_t,
    mut hport: *mut libc::c_char,
    mut key: *mut libc::c_char,
) {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    e = agedge((*S).g, t, h, key, (0 as libc::c_int == 0) as libc::c_int);
    if !e.is_null() {
        let mut tp: *mut libc::c_char = tport;
        let mut hp: *mut libc::c_char = hport;
        if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node
            != (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node
            && (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node == t
        {
            let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
            temp = tp;
            tp = hp;
            hp = temp;
        }
        mkport(
            e,
            b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            tp,
        );
        mkport(
            e,
            b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            hp,
        );
        applyattrs(e as *mut libc::c_void);
    }
}
static mut yytranslate: [yytype_uint8; 269] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn startgraph(
    mut name: *mut libc::c_char,
    mut directed: bool,
    mut strict: bool,
) {
    if G.is_null() {
        SubgraphDepth = 0 as libc::c_int;
        let mut req: Agdesc_t = {
            let mut init = Agdesc_s {
                directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
                c2rust_padding: [0; 3],
            };
            init.set_directed(directed as libc::c_uint);
            init.set_strict(strict as libc::c_uint);
            init.set_no_loop(0);
            init.set_maingraph(1 as libc::c_int as libc::c_uint);
            init.set_flatlock(0);
            init.set_no_write(0);
            init.set_has_attrs(0);
            init.set_has_cmpnd(0);
            init
        };
        G = agopen(name, req, Disc);
        Ag_G_global = G;
    } else {
        Ag_G_global = G;
    }
    S = push(S, G);
    agstrfree(0 as *mut Agraph_t, name);
}
unsafe extern "C" fn endgraph() {
    aglexeof();
    aginternalmapclearlocalnames(G);
}
unsafe extern "C" fn opensubg(mut name: *mut libc::c_char) {
    SubgraphDepth += 1;
    if SubgraphDepth >= 10000 as libc::c_int / 2 as libc::c_int {
        agerr(
            AGERR,
            b"subgraphs nested more than %d deep\0" as *const u8 as *const libc::c_char,
            10000 as libc::c_int,
        );
    }
    S = push(S, agsubg((*S).g, name, (0 as libc::c_int == 0) as libc::c_int));
    agstrfree(G, name);
}
unsafe extern "C" fn closesubg() {
    let mut subg: *mut Agraph_t = (*S).g;
    SubgraphDepth -= 1;
    S = pop(S);
    let ref mut fresh16 = (*S).subg;
    *fresh16 = subg;
    if !subg.is_null() {} else {
        __assert_fail(
            b"subg\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/grammar.y\0" as *const u8 as *const libc::c_char,
            557 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void closesubg()\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn freestack() {
    while !S.is_null() {
        deletelist(&mut (*S).nodelist);
        deletelist(&mut (*S).attrlist);
        deletelist(&mut (*S).edgelist);
        S = pop(S);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agconcat(
    mut g: *mut Agraph_t,
    mut chan: *mut libc::c_void,
    mut disc: *mut Agdisc_t,
) -> *mut Agraph_t {
    aagin = chan as *mut FILE;
    G = g;
    Ag_G_global = 0 as *mut Agraph_t;
    Disc = if !disc.is_null() { disc } else { &mut AgDefaultDisc };
    aglexinit(Disc, chan);
    aagparse();
    if Ag_G_global.is_null() {
        aglexbad();
    }
    return Ag_G_global;
}
#[no_mangle]
pub unsafe extern "C" fn agread(
    mut fp: *mut libc::c_void,
    mut disc: *mut Agdisc_t,
) -> *mut Agraph_t {
    return agconcat(0 as *mut Agraph_t, fp, disc);
}
static mut yypact: [yytype_int8; 80] = [
    18 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    20 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(2 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    10 as libc::c_int as yytype_int8,
    -(2 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    19 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    19 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    11 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    22 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    26 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    -(5 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    16 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    30 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
];
static mut yydefact: [yytype_uint8; 80] = [
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int8; 35] = [
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    0 as libc::c_int as yytype_int8,
    -(17 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    12 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    8 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(8 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(11 as libc::c_int) as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 35] = [
    -(1 as libc::c_int) as yytype_int8,
    3 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 60] = [
    34 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    -(55 as libc::c_int) as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    -(4 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    -(10 as libc::c_int) as yytype_int8,
    15 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    -(10 as libc::c_int) as yytype_int8,
    36 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    -(40 as libc::c_int) as yytype_int8,
    68 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_uint8; 60] = [
    11 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
];
static mut yystos: [yytype_uint8; 80] = [
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 63] = [
    0 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
];
static mut yyr2: [yytype_uint8; 63] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut AAGSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut aagchar: libc::c_int = 0;
#[no_mangle]
pub static mut aaglval: AAGSTYPE = AAGSTYPE { i: 0 };
#[no_mangle]
pub static mut aagnerrs: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn aagparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: libc::c_int = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyvsa: [AAGSTYPE; 200] = [AAGSTYPE { i: 0 }; 200];
    let mut yyvs: *mut AAGSTYPE = 0 as *mut AAGSTYPE;
    let mut yyvsp: *mut AAGSTYPE = 0 as *mut AAGSTYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: AAGSTYPE = AAGSTYPE { i: 0 };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    aagnerrs = 0 as libc::c_int;
    aagchar = -(2 as libc::c_int);
    'c_1813: loop {
        *yyssp = yystate as yytype_int16;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_ulong = (yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong;
            if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                current_block = 2098600215101093122;
                break;
            }
            yystacksize = yystacksize.wrapping_mul(2 as libc::c_int as libc::c_ulong);
            if (10000 as libc::c_int as libc::c_ulong) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_ulong;
            }
            let mut yyss1: *mut yytype_int16 = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                yystacksize
                    .wrapping_mul(
                        (::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<AAGSTYPE>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 2098600215101093122;
                break;
            }
            let mut yynewbytes: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yytype_int16 as *mut libc::c_void,
                yyss as *const libc::c_void,
                yysize
                    .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            let mut yynewbytes_0: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut AAGSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<AAGSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                .wrapping_mul(::std::mem::size_of::<AAGSTYPE>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes_0
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 15133352111665004258;
                break;
            }
        }
        if yystate == 6 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 15404550697806340966;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(18 as libc::c_int) {
                current_block = 8478121261908513268;
            } else {
                if aagchar == -(2 as libc::c_int) {
                    aagchar = aaglex();
                }
                if aagchar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    aagchar = yytoken;
                } else {
                    yytoken = if aagchar as libc::c_uint
                        <= 268 as libc::c_int as libc::c_uint
                    {
                        yytranslate[aagchar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (59 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 8478121261908513268;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 8594975549263610941;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                        }
                        aagchar = -(2 as libc::c_int);
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = aaglval;
                        current_block = 1510118611213224565;
                    }
                }
            }
            match current_block {
                8478121261908513268 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if aagchar == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if aagchar as libc::c_uint
                                <= 268 as libc::c_int as libc::c_uint
                            {
                            yytranslate[aagchar as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            aagnerrs += 1;
                            aagerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if aagchar <= 0 as libc::c_int {
                                if aagchar == 0 as libc::c_int {
                                    current_block = 15133352111665004258;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut aaglval,
                                );
                                aagchar = -(2 as libc::c_int);
                            }
                        }
                        yyerrstatus = 3 as libc::c_int;
                        loop {
                            yyn = yypact[yystate as usize] as libc::c_int;
                            if !(yyn == -(18 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn && yyn <= 59 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 15133352111665004258;
                                break 'c_1813;
                            }
                            yydestruct(
                                b"Error: popping\0" as *const u8 as *const libc::c_char,
                                yystos[yystate as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            yystate = *yyssp as libc::c_int;
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = aaglval;
                        yystate = yyn;
                        current_block = 1510118611213224565;
                    } else {
                        current_block = 8594975549263610941;
                    }
                }
                _ => {}
            }
            match current_block {
                8594975549263610941 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            freestack();
                            endgraph();
                        }
                        3 => {
                            if !G.is_null() {
                                freestack();
                                endgraph();
                                agclose(G);
                                Ag_G_global = 0 as *mut Agraph_t;
                                G = Ag_G_global;
                            }
                        }
                        6 => {
                            startgraph(
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).i
                                    != 0 as libc::c_int,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).i
                                    != 0 as libc::c_int,
                            );
                        }
                        7 => {
                            yyval
                                .str_0 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        8 => {
                            yyval.str_0 = 0 as *mut libc::c_char;
                        }
                        9 => {
                            yyval.i = 1 as libc::c_int;
                        }
                        10 => {
                            yyval.i = 0 as libc::c_int;
                        }
                        11 => {
                            yyval.i = 0 as libc::c_int;
                        }
                        12 => {
                            yyval.i = 1 as libc::c_int;
                        }
                        21 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i != 0 {
                                endedge();
                            } else {
                                endnode();
                            }
                        }
                        24 => {
                            getedgeitems();
                        }
                        25 => {
                            getedgeitems();
                        }
                        26 => {
                            yyval.i = 1 as libc::c_int;
                        }
                        27 => {
                            yyval.i = 0 as libc::c_int;
                        }
                        30 => {
                            appendnode(
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                            );
                        }
                        31 => {
                            appendnode(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).str_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut libc::c_char,
                            );
                        }
                        32 => {
                            appendnode(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).str_0,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).str_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                            );
                        }
                        33 => {
                            attrstmt(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).i,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).str_0,
                            );
                        }
                        34 => {
                            attrstmt(258 as libc::c_int, 0 as *mut libc::c_char);
                        }
                        35 => {
                            yyval.i = 258 as libc::c_int;
                        }
                        36 => {
                            yyval.i = 259 as libc::c_int;
                        }
                        37 => {
                            yyval.i = 260 as libc::c_int;
                        }
                        38 => {
                            yyval
                                .str_0 = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .str_0;
                        }
                        39 => {
                            yyval.str_0 = 0 as *mut libc::c_char;
                        }
                        48 => {
                            appendattr(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).str_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                            );
                        }
                        49 => {
                            appendattr(
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut libc::c_char,
                            );
                        }
                        51 => {
                            opensubg((*yyvsp.offset(0 as libc::c_int as isize)).str_0);
                        }
                        52 => {
                            closesubg();
                        }
                        53 => {
                            yyval
                                .str_0 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        54 => {
                            yyval.str_0 = 0 as *mut libc::c_char;
                        }
                        55 => {
                            yyval.str_0 = 0 as *mut libc::c_char;
                        }
                        59 => {
                            yyval
                                .str_0 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        60 => {
                            yyval
                                .str_0 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        61 => {
                            yyval
                                .str_0 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        62 => {
                            yyval
                                .str_0 = concat(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).str_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                            );
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    yyn = yyr1[yyn as usize] as libc::c_int;
                    yystate = yypgoto[(yyn - 24 as libc::c_int) as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    if 0 as libc::c_int <= yystate && yystate <= 59 as libc::c_int
                        && yycheck[yystate as usize] as libc::c_int
                            == *yyssp as libc::c_int
                    {
                        yystate = yytable[yystate as usize] as libc::c_int;
                    } else {
                        yystate = yydefgoto[(yyn - 24 as libc::c_int) as usize]
                            as libc::c_int;
                    }
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
        }
    }
    match current_block {
        2098600215101093122 => {
            aagerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        15133352111665004258 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if aagchar != -(2 as libc::c_int) {
        yytoken = if aagchar as libc::c_uint <= 268 as libc::c_int as libc::c_uint {
            yytranslate[aagchar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut aaglval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
