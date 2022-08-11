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
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtview(_: *mut Dt_t, _: *mut Dt_t) -> *mut Dt_t;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agisstrict(g: *mut Agraph_t) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agsubedge(g: *mut Agraph_t, e: *mut Agedge_t, createflag: libc::c_int) -> *mut Agedge_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agidnode(g: *mut Agraph_t, id: IDTYPE, createflag: libc::c_int) -> *mut Agnode_t;
    fn aghtmlstr(_: *const libc::c_char) -> libc::c_int;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn xml_escape(
        s: *const libc::c_char,
        flags: xml_flags_t,
        cb: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
        state: *mut libc::c_void,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn agdatadict(g: *mut Agraph_t, cflag: libc::c_int) -> *mut Agdatadict_t;
    fn agattrrec(obj: *mut libc::c_void) -> *mut Agattr_t;
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
pub type agobjfn_t =
    Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> ()>;
pub type Agraph_t = Agraph_s;
pub type agobjupdfn_t = Option<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void, *mut Agsym_t) -> (),
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
    pub afread: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub putstr: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type Agiddisc_t = Agiddisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiddisc_s {
    pub open: Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agdisc_t) -> *mut libc::c_void>,
    pub map: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_char,
            *mut IDTYPE,
            libc::c_int,
        ) -> libc::c_long,
    >,
    pub alloc: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> libc::c_long>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> ()>,
    pub print:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> *mut libc::c_char>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub idregister:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, *mut libc::c_void) -> ()>,
}
pub type Agmemdisc_t = Agmemdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agmemdisc_s {
    pub open: Option<unsafe extern "C" fn(*mut Agdisc_t) -> *mut libc::c_void>,
    pub alloc: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>,
    pub resize: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct xml_flags_t {
    #[bitfield(name = "raw", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "dash", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "nbsp", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "utf8", ty = "libc::c_uint", bits = "3..=3")]
    pub raw_dash_nbsp_utf8: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gxlstate_t {
    pub nodeMap: *mut Dt_t,
    pub graphMap: *mut Dt_t,
    pub synNodeMap: *mut Dt_t,
    pub idList: *mut Dt_t,
    pub root: *mut Agraph_t,
    pub attrsNotWritten: libc::c_char,
    pub directed: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct idv_t {
    pub link: Dtlink_t,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct namev_t {
    pub link: Dtlink_t,
    pub name: *mut libc::c_char,
    pub unique_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Local_Agnodeinfo_t {
    pub h: Agrec_t,
    pub written: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_calloc(mut nmemb: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong
        && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_alloc(mut size: size_t) -> *mut libc::c_void {
    return gv_calloc(1 as libc::c_int as size_t, size);
}
#[inline]
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
static mut Level: libc::c_int = 0;
static mut Tailport: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
static mut Headport: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
unsafe extern "C" fn make_nitem(
    mut d: *mut Dt_t,
    mut objp: *mut namev_t,
    mut disc: *mut Dtdisc_t,
) -> *mut namev_t {
    let mut np: *mut namev_t =
        gv_alloc(::std::mem::size_of::<namev_t>() as libc::c_ulong) as *mut namev_t;
    let ref mut fresh0 = (*np).name;
    *fresh0 = (*objp).name;
    let ref mut fresh1 = (*np).unique_name;
    *fresh1 = 0 as *mut libc::c_char;
    return np;
}
unsafe extern "C" fn free_nitem(mut d: *mut Dt_t, mut np: *mut namev_t, mut disc: *mut Dtdisc_t) {
    free(np as *mut libc::c_void);
}
static mut nameDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> *mut namev_t,
                >,
                Dtmake_f,
            >(Some(
                make_nitem
                    as unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> *mut namev_t,
            )),
            freef: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> ()>,
                Dtfree_f,
            >(Some(
                free_nitem as unsafe extern "C" fn(*mut Dt_t, *mut namev_t, *mut Dtdisc_t) -> (),
            )),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn free_iditem(mut d: *mut Dt_t, mut idp: *mut idv_t, mut disc: *mut Dtdisc_t) {
    free((*idp).name as *mut libc::c_void);
    free(idp as *mut libc::c_void);
}
static mut idDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Dt_t, *mut idv_t, *mut Dtdisc_t) -> ()>,
                Dtfree_f,
            >(Some(
                free_iditem as unsafe extern "C" fn(*mut Dt_t, *mut idv_t, *mut Dtdisc_t) -> (),
            )),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn tabover(mut gxlFile: *mut FILE) {
    let mut temp: libc::c_int = 0;
    temp = Level;
    loop {
        let fresh2 = temp;
        temp = temp - 1;
        if !(fresh2 != 0) {
            break;
        }
        putc('\t' as i32, gxlFile);
    }
}
unsafe extern "C" fn legalGXLName(mut id: *mut libc::c_char) -> libc::c_int {
    let fresh3 = id;
    id = id.offset(1);
    let mut c: libc::c_char = *fresh3;
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        && c as libc::c_int != '_' as i32
        && c as libc::c_int != ':' as i32
    {
        return 0 as libc::c_int;
    }
    loop {
        let fresh4 = id;
        id = id.offset(1);
        c = *fresh4;
        if !(c != 0) {
            break;
        }
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0
            && c as libc::c_int != '_' as i32
            && c as libc::c_int != ':' as i32
            && c as libc::c_int != '-' as i32
            && c as libc::c_int != '.' as i32
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn put(mut stream: *mut libc::c_void, mut s: *const libc::c_char) -> libc::c_int {
    return fputs(s, stream as *mut FILE);
}
#[inline]
unsafe extern "C" fn xml_puts(mut stream: *mut FILE, mut s: *const libc::c_char) -> libc::c_int {
    let flags: xml_flags_t = {
        let mut init = xml_flags_t {
            raw_dash_nbsp_utf8: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_raw(0);
        init.set_dash(1 as libc::c_int as libc::c_uint);
        init.set_nbsp(1 as libc::c_int as libc::c_uint);
        init.set_utf8(0);
        init
    };
    return xml_escape(
        s,
        flags,
        Some(put as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int),
        stream as *mut libc::c_void,
    );
}
unsafe extern "C" fn xml_url_puts(mut f: *mut FILE, mut s: *const libc::c_char) -> libc::c_int {
    let flags: xml_flags_t = {
        let mut init = xml_flags_t {
            raw_dash_nbsp_utf8: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_raw(0 as libc::c_int as libc::c_uint);
        init.set_dash(0);
        init.set_nbsp(0);
        init.set_utf8(0);
        init
    };
    return xml_escape(
        s,
        flags,
        Some(put as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int),
        f as *mut libc::c_void,
    );
}
unsafe extern "C" fn isGxlGrammar(mut name: *mut libc::c_char) -> libc::c_int {
    return (strncmp(
        name,
        b"_gxl_\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn isLocatorType(mut name: *mut libc::c_char) -> libc::c_int {
    return (strncmp(
        name,
        b"_gxl_locator_\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn idexists(mut ids: *mut Dt_t, mut id: *mut libc::c_char) -> *mut libc::c_void {
    return (Some(((*ids).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        ids, id as *mut libc::c_void, 0o1000 as libc::c_int
    );
}
unsafe extern "C" fn addid(mut ids: *mut Dt_t, mut id: *mut libc::c_char) -> *mut libc::c_char {
    let mut idp: *mut idv_t =
        gv_alloc(::std::mem::size_of::<idv_t>() as libc::c_ulong) as *mut idv_t;
    let ref mut fresh5 = (*idp).name;
    *fresh5 = gv_strdup(id);
    (Some(((*ids).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(ids, idp as *mut libc::c_void, 0o1 as libc::c_int);
    return (*idp).name;
}
unsafe extern "C" fn createGraphId(mut ids: *mut Dt_t) -> *mut libc::c_char {
    static mut graphIdCounter: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 128] = [0; 128];
    loop {
        let fresh6 = graphIdCounter;
        graphIdCounter = graphIdCounter + 1;
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"G_%d\0" as *const u8 as *const libc::c_char,
            fresh6,
        );
        if (idexists(ids, buf.as_mut_ptr())).is_null() {
            break;
        }
    }
    return addid(ids, buf.as_mut_ptr());
}
unsafe extern "C" fn createNodeId(mut ids: *mut Dt_t) -> *mut libc::c_char {
    static mut nodeIdCounter: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 128] = [0; 128];
    loop {
        let fresh7 = nodeIdCounter;
        nodeIdCounter = nodeIdCounter + 1;
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"N_%d\0" as *const u8 as *const libc::c_char,
            fresh7,
        );
        if (idexists(ids, buf.as_mut_ptr())).is_null() {
            break;
        }
    }
    return addid(ids, buf.as_mut_ptr());
}
unsafe extern "C" fn mapLookup(
    mut nm: *mut Dt_t,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut objp: *mut namev_t = (Some(((*nm).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        nm, name as *mut libc::c_void, 0o1000 as libc::c_int
    ) as *mut namev_t;
    if !objp.is_null() {
        return (*objp).unique_name;
    } else {
        return 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn nodeID(mut stp: *mut gxlstate_t, mut n: *mut Agnode_t) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uniqueName: *mut libc::c_char = 0 as *mut libc::c_char;
    name = agnameof(n as *mut libc::c_void);
    uniqueName = mapLookup((*stp).nodeMap, name);
    if !uniqueName.is_null() {
    } else {
        __assert_fail(
            b"uniqueName\0" as *const u8 as *const libc::c_char,
            b"gv2gxl.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"char *nodeID(gxlstate_t *, Agnode_t *)\0",
            ))
            .as_ptr(),
        );
    }
    return uniqueName;
}
unsafe extern "C" fn createEdgeId(
    mut stp: *mut gxlstate_t,
    mut e: *mut Agedge_t,
) -> *mut libc::c_char {
    let mut edgeIdCounter: libc::c_int = 1 as libc::c_int;
    let mut hname: *mut libc::c_char = nodeID(
        stp,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node,
    );
    let mut tname: *mut libc::c_char = nodeID(
        stp,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node,
    );
    let mut baselen: size_t = (strlen(hname))
        .wrapping_add(strlen(tname))
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong);
    let mut len: size_t = baselen.wrapping_add(32 as libc::c_int as libc::c_ulong);
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: *mut libc::c_char =
        gv_calloc(len, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong) as *mut libc::c_char;
    endp = bp.offset(baselen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    sprintf(
        bp,
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        tname,
        b"--\0" as *const u8 as *const libc::c_char,
        hname,
    );
    while !(idexists((*stp).idList, bp)).is_null() {
        let fresh8 = edgeIdCounter;
        edgeIdCounter = edgeIdCounter + 1;
        sprintf(endp, b":%d\0" as *const u8 as *const libc::c_char, fresh8);
    }
    rv = addid((*stp).idList, bp);
    free(bp as *mut libc::c_void);
    return rv;
}
unsafe extern "C" fn addToMap(
    mut map: *mut Dt_t,
    mut name: *mut libc::c_char,
    mut uniqueName: *mut libc::c_char,
) {
    let mut obj: namev_t = namev_t {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        name: 0 as *mut libc::c_char,
        unique_name: 0 as *mut libc::c_char,
    };
    let mut objp: *mut namev_t = 0 as *mut namev_t;
    obj.name = name;
    objp = (Some(((*map).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        map,
        &mut obj as *mut namev_t as *mut libc::c_void,
        0o1 as libc::c_int,
    ) as *mut namev_t;
    if ((*objp).unique_name).is_null() {
    } else {
        __assert_fail(
            b"objp->unique_name == 0\0" as *const u8 as *const libc::c_char,
            b"gv2gxl.c\0" as *const u8 as *const libc::c_char,
            276 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void addToMap(Dt_t *, char *, char *)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh9 = (*objp).unique_name;
    *fresh9 = uniqueName;
}
unsafe extern "C" fn graphAttrs(mut gxlFile: *mut FILE, mut g: *mut Agraph_t) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    val = agget(
        g as *mut libc::c_void,
        b"_gxl_role\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(val.is_null() || *val as libc::c_int == '\0' as i32) {
        fprintf(gxlFile, b" role=\"\0" as *const u8 as *const libc::c_char);
        xml_puts(gxlFile, val);
        fprintf(gxlFile, b"\"\0" as *const u8 as *const libc::c_char);
    }
    val = agget(
        g as *mut libc::c_void,
        b"_gxl_hypergraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(val.is_null() || *val as libc::c_int == '\0' as i32) {
        fprintf(
            gxlFile,
            b" hypergraph=\"\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, val);
        fprintf(gxlFile, b"\"\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn edgeAttrs(mut gxlFile: *mut FILE, mut e: *mut Agedge_t) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    val = agget(
        e as *mut libc::c_void,
        b"_gxl_id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(val.is_null() || *val as libc::c_int == '\0' as i32) {
        fprintf(gxlFile, b" id=\"\0" as *const u8 as *const libc::c_char);
        xml_puts(gxlFile, val);
        fprintf(gxlFile, b"\"\0" as *const u8 as *const libc::c_char);
    }
    val = agget(
        e as *mut libc::c_void,
        b"_gxl_fromorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(val.is_null() || *val as libc::c_int == '\0' as i32) {
        fprintf(
            gxlFile,
            b" fromorder=\"\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, val);
        fprintf(gxlFile, b"\"\0" as *const u8 as *const libc::c_char);
    }
    val = agget(
        e as *mut libc::c_void,
        b"_gxl_toorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(val.is_null() || *val as libc::c_int == '\0' as i32) {
        fprintf(
            gxlFile,
            b" toorder=\"\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, val);
        fprintf(gxlFile, b"\"\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn printHref(mut gxlFile: *mut FILE, mut n: *mut libc::c_void) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    val = agget(
        n,
        b"_gxl_type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(val.is_null() || *val as libc::c_int == '\0' as i32) {
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t<type xlink:href=\"\0" as *const u8 as *const libc::c_char,
        );
        xml_url_puts(gxlFile, val);
        fprintf(gxlFile, b"\">\n\0" as *const u8 as *const libc::c_char);
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t</type>\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn writeDict(
    mut g: *mut Agraph_t,
    mut gxlFile: *mut FILE,
    mut name: *mut libc::c_char,
    mut dict: *mut Dict_t,
    mut isGraph: libc::c_int,
) {
    let mut view: *mut Dict_t = 0 as *mut Dict_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut psym: *mut Agsym_t = 0 as *mut Agsym_t;
    view = dtview(dict, 0 as *mut Dt_t);
    let mut current_block_55: u64;
    sym = (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        dict, 0 as *mut libc::c_void, 0o200 as libc::c_int
    ) as *mut Agsym_t;
    while !sym.is_null() {
        if isGxlGrammar((*sym).name) == 0 {
            if ((*sym).defval).is_null() || *(*sym).defval as libc::c_int == '\0' as i32 {
                if view.is_null() {
                    current_block_55 = 10680521327981672866;
                } else {
                    psym = (Some(
                        ((*(view as *mut Dt_t)).searchf).expect("non-null function pointer"),
                    ))
                    .expect("non-null function pointer")(
                        view,
                        sym as *mut libc::c_void,
                        0o4 as libc::c_int,
                    ) as *mut Agsym_t;
                    if ((*psym).defval).is_null() || *(*psym).defval as libc::c_int == '\0' as i32 {
                        current_block_55 = 10680521327981672866;
                    } else {
                        current_block_55 = 13109137661213826276;
                    }
                }
            } else {
                current_block_55 = 13109137661213826276;
            }
            match current_block_55 {
                10680521327981672866 => {}
                _ => {
                    if isLocatorType((*sym).defval) != 0 {
                        let mut locatorVal: *mut libc::c_char = 0 as *mut libc::c_char;
                        locatorVal = (*sym).defval;
                        locatorVal = locatorVal.offset(13 as libc::c_int as isize);
                        tabover(gxlFile);
                        fprintf(
                            gxlFile,
                            b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
                        );
                        xml_puts(gxlFile, (*sym).name);
                        fprintf(gxlFile, b"\">\n\0" as *const u8 as *const libc::c_char);
                        tabover(gxlFile);
                        fprintf(
                            gxlFile,
                            b"\t\t<locator xlink:href=\"\0" as *const u8 as *const libc::c_char,
                        );
                        xml_url_puts(gxlFile, locatorVal);
                        fprintf(gxlFile, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                        tabover(gxlFile);
                        fprintf(
                            gxlFile,
                            b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        tabover(gxlFile);
                        if isGraph != 0 {
                            fprintf(
                                gxlFile,
                                b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
                            );
                            xml_puts(gxlFile, (*sym).name);
                            fprintf(gxlFile, b"\" \0" as *const u8 as *const libc::c_char);
                            fprintf(gxlFile, b"kind=\"\0" as *const u8 as *const libc::c_char);
                            xml_puts(gxlFile, name);
                            fprintf(gxlFile, b"\">\n\0" as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(
                                gxlFile,
                                b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
                            );
                            xml_puts(gxlFile, name);
                            fprintf(gxlFile, b":\0" as *const u8 as *const libc::c_char);
                            xml_puts(gxlFile, (*sym).name);
                            fprintf(gxlFile, b"\" kind=\"\0" as *const u8 as *const libc::c_char);
                            xml_puts(gxlFile, name);
                            fprintf(gxlFile, b"\">\n\0" as *const u8 as *const libc::c_char);
                        }
                        tabover(gxlFile);
                        fprintf(
                            gxlFile,
                            b"\t\t<string>\0" as *const u8 as *const libc::c_char,
                        );
                        xml_puts(gxlFile, (*sym).defval);
                        fprintf(
                            gxlFile,
                            b"</string>\n\0" as *const u8 as *const libc::c_char,
                        );
                        tabover(gxlFile);
                        fprintf(
                            gxlFile,
                            b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        } else if strncmp(
            (*sym).name,
            b"_gxl_composite_\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            if ((*sym).defval).is_null() || *(*sym).defval as libc::c_int == '\0' as i32 {
                if view.is_null() {
                    current_block_55 = 10680521327981672866;
                } else {
                    psym = (Some(
                        ((*(view as *mut Dt_t)).searchf).expect("non-null function pointer"),
                    ))
                    .expect("non-null function pointer")(
                        view,
                        sym as *mut libc::c_void,
                        0o4 as libc::c_int,
                    ) as *mut Agsym_t;
                    if ((*psym).defval).is_null() || *(*psym).defval as libc::c_int == '\0' as i32 {
                        current_block_55 = 10680521327981672866;
                    } else {
                        current_block_55 = 6476622998065200121;
                    }
                }
            } else {
                current_block_55 = 6476622998065200121;
            }
            match current_block_55 {
                10680521327981672866 => {}
                _ => {
                    tabover(gxlFile);
                    fprintf(
                        gxlFile,
                        b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
                    );
                    xml_puts(
                        gxlFile,
                        ((*sym).name).offset(
                            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ),
                    );
                    fprintf(gxlFile, b"\" \0" as *const u8 as *const libc::c_char);
                    fprintf(gxlFile, b"kind=\"\0" as *const u8 as *const libc::c_char);
                    xml_puts(gxlFile, name);
                    fprintf(gxlFile, b"\">\n\0" as *const u8 as *const libc::c_char);
                    tabover(gxlFile);
                    fprintf(gxlFile, b"\t\t\0" as *const u8 as *const libc::c_char);
                    xml_puts(gxlFile, (*sym).defval);
                    fprintf(gxlFile, b"\n\0" as *const u8 as *const libc::c_char);
                    tabover(gxlFile);
                    fprintf(
                        gxlFile,
                        b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        sym = (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            dict,
            sym as *mut libc::c_void,
            0o10 as libc::c_int,
        ) as *mut Agsym_t;
    }
    dtview(dict, view);
}
unsafe extern "C" fn writeDicts(mut g: *mut Agraph_t, mut gxlFile: *mut FILE) {
    let mut def: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    def = agdatadict(g, 0 as libc::c_int);
    if !def.is_null() {
        writeDict(
            g,
            gxlFile,
            b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*def).dict.g,
            1 as libc::c_int,
        );
        writeDict(
            g,
            gxlFile,
            b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*def).dict.n,
            0 as libc::c_int,
        );
        writeDict(
            g,
            gxlFile,
            b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*def).dict.e,
            0 as libc::c_int,
        );
    }
}
unsafe extern "C" fn writeHdr(
    mut stp: *mut gxlstate_t,
    mut g: *mut Agraph_t,
    mut gxlFile: *mut FILE,
    mut top: libc::c_int,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kind: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uniqueName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    Level += 1;
    (*stp).attrsNotWritten = ((*(g as *mut Agobj_t)).tag).attrwf() as libc::c_char;
    name = agnameof(g as *mut libc::c_void);
    if ((*g).desc).directed() != 0 {
        kind = b"directed\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        kind = b"undirected\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if top == 0 && !(agparent(g)).is_null() {
        len = (strlen(name))
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong);
        let mut bp: *mut libc::c_char =
            gv_calloc(len, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as *mut libc::c_char;
        sprintf(bp, b"N_%s\0" as *const u8 as *const libc::c_char, name);
        if !(idexists((*stp).idList, bp)).is_null() || legalGXLName(bp) == 0 {
            bp = createNodeId((*stp).idList);
        } else {
            bp = addid((*stp).idList, bp);
        }
        addToMap((*stp).synNodeMap, name, bp);
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"<node id=\"%s\">\n\0" as *const u8 as *const libc::c_char,
            bp,
        );
        free(bp as *mut libc::c_void);
        Level += 1;
    } else {
        Tailport = agattr(
            g,
            2 as libc::c_int,
            b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
        Headport = agattr(
            g,
            2 as libc::c_int,
            b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
    }
    uniqueName = mapLookup((*stp).graphMap, name);
    tabover(gxlFile);
    fprintf(
        gxlFile,
        b"<graph id=\"%s\" edgeids=\"true\" edgemode=\"%s\"\0" as *const u8 as *const libc::c_char,
        uniqueName,
        kind,
    );
    graphAttrs(gxlFile, g);
    fprintf(gxlFile, b">\n\0" as *const u8 as *const libc::c_char);
    if !uniqueName.is_null() && strcmp(name, uniqueName) != 0 as libc::c_int {
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t<attr name=\"name\">\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t\t<string>\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, name);
        fprintf(
            gxlFile,
            b"</string>\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if agisstrict(g) != 0 {
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t<attr name=\"strict\">\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t\t<string>true</string>\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    writeDicts(g, gxlFile);
    printHref(gxlFile, g as *mut libc::c_void);
    let ref mut fresh10 = (*(g as *mut Agobj_t)).tag;
    (*fresh10)
        .set_attrwf((((*(g as *mut Agobj_t)).tag).attrwf() == 0) as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn writeTrl(mut g: *mut Agraph_t, mut gxlFile: *mut FILE, mut top: libc::c_int) {
    tabover(gxlFile);
    fprintf(gxlFile, b"</graph>\n\0" as *const u8 as *const libc::c_char);
    Level -= 1;
    if top == 0 && !(agparent(g)).is_null() {
        tabover(gxlFile);
        fprintf(gxlFile, b"</node>\n\0" as *const u8 as *const libc::c_char);
        Level -= 1;
    }
}
unsafe extern "C" fn writeSubgs(
    mut stp: *mut gxlstate_t,
    mut g: *mut Agraph_t,
    mut gxlFile: *mut FILE,
) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        writeHdr(stp, subg, gxlFile, 0 as libc::c_int);
        writeBody(stp, subg, gxlFile);
        writeTrl(subg, gxlFile, 0 as libc::c_int);
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn writeEdgeName(mut e: *mut Agedge_t, mut gxlFile: *mut FILE) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = agnameof(e as *mut libc::c_void);
    if !(p.is_null() || *p as libc::c_int == '\0' as i32) {
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t<attr name=\"key\">\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t\t<string>\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, p);
        fprintf(
            gxlFile,
            b"</string>\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
        );
        rv = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        rv = 0 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn writeNondefaultAttr(
    mut obj: *mut libc::c_void,
    mut gxlFile: *mut FILE,
    mut defdict: *mut Dict_t,
) {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    if ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
        || ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
    {
        if writeEdgeName(obj as *mut Agedge_t, gxlFile) != 0 {
            cnt += 1;
        }
    }
    let mut data: *mut Agattr_t = agattrrec(obj);
    if !data.is_null() {
        let mut current_block_48: u64;
        sym = (Some(((*(defdict as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            defdict,
            0 as *mut libc::c_void,
            0o200 as libc::c_int,
        ) as *mut Agsym_t;
        while !sym.is_null() {
            if isGxlGrammar((*sym).name) == 0 {
                if ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
                    || ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
                {
                    if !Tailport.is_null() && (*sym).id == (*Tailport).id {
                        current_block_48 = 11006700562992250127;
                    } else if !Headport.is_null() && (*sym).id == (*Headport).id {
                        current_block_48 = 11006700562992250127;
                    } else {
                        current_block_48 = 13586036798005543211;
                    }
                } else {
                    current_block_48 = 13586036798005543211;
                }
                match current_block_48 {
                    11006700562992250127 => {}
                    _ => {
                        if *((*data).str_0).offset((*sym).id as isize) != (*sym).defval {
                            if !(strcmp(
                                *((*data).str_0).offset((*sym).id as isize),
                                b"\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int)
                            {
                                if isLocatorType(*((*data).str_0).offset((*sym).id as isize)) != 0 {
                                    let mut locatorVal: *mut libc::c_char = 0 as *mut libc::c_char;
                                    locatorVal = *((*data).str_0).offset((*sym).id as isize);
                                    locatorVal = locatorVal.offset(13 as libc::c_int as isize);
                                    tabover(gxlFile);
                                    fprintf(
                                        gxlFile,
                                        b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
                                    );
                                    xml_puts(gxlFile, (*sym).name);
                                    fprintf(
                                        gxlFile,
                                        b"\">\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    tabover(gxlFile);
                                    fprintf(
                                        gxlFile,
                                        b"\t\t<locator xlink:href=\"\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    xml_url_puts(gxlFile, locatorVal);
                                    fprintf(
                                        gxlFile,
                                        b"\"/>\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    tabover(gxlFile);
                                    fprintf(
                                        gxlFile,
                                        b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    tabover(gxlFile);
                                    fprintf(
                                        gxlFile,
                                        b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
                                    );
                                    xml_puts(gxlFile, (*sym).name);
                                    fprintf(gxlFile, b"\"\0" as *const u8 as *const libc::c_char);
                                    if aghtmlstr(*((*data).str_0).offset((*sym).id as isize)) != 0 {
                                        fprintf(
                                            gxlFile,
                                            b" kind=\"HTML-like string\"\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    fprintf(gxlFile, b">\n\0" as *const u8 as *const libc::c_char);
                                    tabover(gxlFile);
                                    fprintf(
                                        gxlFile,
                                        b"\t\t<string>\0" as *const u8 as *const libc::c_char,
                                    );
                                    xml_puts(gxlFile, *((*data).str_0).offset((*sym).id as isize));
                                    fprintf(
                                        gxlFile,
                                        b"</string>\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    tabover(gxlFile);
                                    fprintf(
                                        gxlFile,
                                        b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                            }
                        }
                    }
                }
            } else if strncmp(
                (*sym).name,
                b"_gxl_composite_\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            {
                if *((*data).str_0).offset((*sym).id as isize) != (*sym).defval {
                    tabover(gxlFile);
                    fprintf(
                        gxlFile,
                        b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
                    );
                    xml_puts(
                        gxlFile,
                        ((*sym).name).offset(
                            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ),
                    );
                    fprintf(gxlFile, b"\">\n\0" as *const u8 as *const libc::c_char);
                    tabover(gxlFile);
                    fprintf(gxlFile, b"\t\t\0" as *const u8 as *const libc::c_char);
                    xml_puts(gxlFile, *((*data).str_0).offset((*sym).id as isize));
                    fprintf(gxlFile, b"\n\0" as *const u8 as *const libc::c_char);
                    tabover(gxlFile);
                    fprintf(
                        gxlFile,
                        b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            sym = (Some(((*(defdict as *mut Dt_t)).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                defdict,
                sym as *mut libc::c_void,
                0o10 as libc::c_int,
            ) as *mut Agsym_t;
        }
    }
    let ref mut fresh11 = (*(obj as *mut Agobj_t)).tag;
    (*fresh11)
        .set_attrwf((((*(obj as *mut Agobj_t)).tag).attrwf() == 0) as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn attrs_written(
    mut stp: *mut gxlstate_t,
    mut obj: *mut libc::c_void,
) -> libc::c_int {
    return !(((*(obj as *mut Agobj_t)).tag).attrwf() as libc::c_int
        == (*stp).attrsNotWritten as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn writeNode(
    mut stp: *mut gxlstate_t,
    mut n: *mut Agnode_t,
    mut gxlFile: *mut FILE,
    mut d: *mut Dict_t,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uniqueName: *mut libc::c_char = 0 as *mut libc::c_char;
    name = agnameof(n as *mut libc::c_void);
    uniqueName = nodeID(stp, n);
    Level += 1;
    tabover(gxlFile);
    fprintf(
        gxlFile,
        b"<node id=\"%s\">\n\0" as *const u8 as *const libc::c_char,
        uniqueName,
    );
    printHref(gxlFile, n as *mut libc::c_void);
    if strcmp(name, uniqueName) != 0 {
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t<attr name=\"name\">\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t\t<string>\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, name);
        fprintf(
            gxlFile,
            b"</string>\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if attrs_written(stp, n as *mut libc::c_void) == 0 {
        writeNondefaultAttr(n as *mut libc::c_void, gxlFile, d);
    }
    tabover(gxlFile);
    fprintf(gxlFile, b"</node>\n\0" as *const u8 as *const libc::c_char);
    Level -= 1;
}
unsafe extern "C" fn writePort(
    mut e: *mut Agedge_t,
    mut gxlFile: *mut FILE,
    mut name: *mut libc::c_char,
) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    val = agget(e as *mut libc::c_void, name);
    if !val.is_null() && *val.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t<attr name=\"\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, name);
        fprintf(gxlFile, b"\">\n\0" as *const u8 as *const libc::c_char);
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t\t<string>\0" as *const u8 as *const libc::c_char,
        );
        xml_puts(gxlFile, val);
        fprintf(
            gxlFile,
            b"</string>\n\0" as *const u8 as *const libc::c_char,
        );
        tabover(gxlFile);
        fprintf(
            gxlFile,
            b"\t</attr>\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn writeEdgeTest(mut g: *mut Agraph_t, mut e: *mut Agedge_t) -> libc::c_int {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        if !(agsubedge(subg, e, 0 as libc::c_int)).is_null() {
            return 0 as libc::c_int;
        }
        subg = agnxtsubg(subg);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn writeEdge(
    mut stp: *mut gxlstate_t,
    mut e: *mut Agedge_t,
    mut gxlFile: *mut FILE,
    mut d: *mut Dict_t,
) {
    let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut edge_id: *mut libc::c_char = 0 as *mut libc::c_char;
    t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
    .node;
    h = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
    .node;
    Level += 1;
    tabover(gxlFile);
    fprintf(
        gxlFile,
        b"<edge from=\"%s\" \0" as *const u8 as *const libc::c_char,
        nodeID(stp, t),
    );
    fprintf(
        gxlFile,
        b"to=\"%s\"\0" as *const u8 as *const libc::c_char,
        nodeID(stp, h),
    );
    edgeAttrs(gxlFile, e);
    if (*stp).directed != 0 {
        fprintf(
            gxlFile,
            b" isdirected=\"true\"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            gxlFile,
            b" isdirected=\"false\"\0" as *const u8 as *const libc::c_char,
        );
    }
    edge_id = agget(
        e as *mut libc::c_void,
        b"_gxl_id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(edge_id.is_null() || *edge_id as libc::c_int == '\0' as i32) {
        fprintf(gxlFile, b">\n\0" as *const u8 as *const libc::c_char);
    } else {
        bp = createEdgeId(stp, e);
        fprintf(
            gxlFile,
            b" id=\"%s\">\n\0" as *const u8 as *const libc::c_char,
            bp,
        );
    }
    printHref(gxlFile, e as *mut libc::c_void);
    writePort(
        e,
        gxlFile,
        b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    writePort(
        e,
        gxlFile,
        b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if attrs_written(stp, e as *mut libc::c_void) == 0 {
        writeNondefaultAttr(e as *mut libc::c_void, gxlFile, d);
    } else {
        writeEdgeName(e, gxlFile);
    }
    tabover(gxlFile);
    fprintf(gxlFile, b"</edge>\n\0" as *const u8 as *const libc::c_char);
    Level -= 1;
}
unsafe extern "C" fn writeBody(
    mut stp: *mut gxlstate_t,
    mut g: *mut Agraph_t,
    mut gxlFile: *mut FILE,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut realn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    writeSubgs(stp, g, gxlFile);
    let mut dd: *mut Agdatadict_t = agdatadict(g, 0 as libc::c_int);
    n = agfstnode(g);
    while !n.is_null() {
        realn = agidnode((*stp).root, (*(n as *mut Agobj_t)).tag.id, 0 as libc::c_int);
        if (*((*realn).base.data as *mut Local_Agnodeinfo_t)).written == 0 {
            (*((*realn).base.data as *mut Local_Agnodeinfo_t)).written = 1 as libc::c_int;
            writeNode(stp, n, gxlFile, (*dd).dict.n);
        }
        e = agfstout(g, n);
        while !e.is_null() {
            if writeEdgeTest(g, e) != 0 {
                writeEdge(stp, e, gxlFile, (*dd).dict.e);
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn iterateHdr(mut stp: *mut gxlstate_t, mut g: *mut Agraph_t) {
    let mut gxlId: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = agnameof(g as *mut libc::c_void);
    gxlId = agget(
        g as *mut libc::c_void,
        b"_gxl_id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if gxlId.is_null() || *gxlId as libc::c_int == '\0' as i32 {
        gxlId = name;
    }
    if !(idexists((*stp).idList, gxlId)).is_null() || legalGXLName(gxlId) == 0 {
        gxlId = createGraphId((*stp).idList);
    } else {
        gxlId = addid((*stp).idList, gxlId);
    }
    addToMap((*stp).graphMap, name, gxlId);
}
unsafe extern "C" fn iterate_subgs(mut stp: *mut gxlstate_t, mut g: *mut Agraph_t) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        iterateHdr(stp, subg);
        iterateBody(stp, subg);
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn iterateBody(mut stp: *mut gxlstate_t, mut g: *mut Agraph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    iterate_subgs(stp, g);
    n = agfstnode(g);
    while !n.is_null() {
        let mut gxlId: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nodename: *mut libc::c_char = agnameof(n as *mut libc::c_void);
        if (mapLookup((*stp).nodeMap, nodename)).is_null() {
            gxlId = agget(
                n as *mut libc::c_void,
                b"_gxl_id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if gxlId.is_null() || *gxlId as libc::c_int == '\0' as i32 {
                gxlId = nodename;
            }
            if !(idexists((*stp).idList, gxlId)).is_null() || legalGXLName(gxlId) == 0 {
                gxlId = createNodeId((*stp).idList);
            } else {
                gxlId = addid((*stp).idList, gxlId);
            }
            addToMap((*stp).nodeMap, nodename, gxlId);
        }
        e = agfstout(g, n);
        while !e.is_null() {
            if writeEdgeTest(g, e) != 0 {
                let mut edge_id: *mut libc::c_char = agget(
                    e as *mut libc::c_void,
                    b"_gxl_id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if !(edge_id.is_null() || *edge_id as libc::c_int == '\0' as i32) {
                    addid((*stp).idList, edge_id);
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn initState(mut g: *mut Agraph_t) -> *mut gxlstate_t {
    let mut stp: *mut gxlstate_t =
        gv_alloc(::std::mem::size_of::<gxlstate_t>() as libc::c_ulong) as *mut gxlstate_t;
    let ref mut fresh12 = (*stp).nodeMap;
    *fresh12 = dtopen(&mut nameDisc, Dtoset);
    let ref mut fresh13 = (*stp).graphMap;
    *fresh13 = dtopen(&mut nameDisc, Dtoset);
    let ref mut fresh14 = (*stp).synNodeMap;
    *fresh14 = dtopen(&mut nameDisc, Dtoset);
    let ref mut fresh15 = (*stp).idList;
    *fresh15 = dtopen(&mut idDisc, Dtoset);
    (*stp).attrsNotWritten = 0 as libc::c_int as libc::c_char;
    let ref mut fresh16 = (*stp).root;
    *fresh16 = g;
    (*stp).directed = (agisdirected(g) != 0 as libc::c_int) as libc::c_int as libc::c_char;
    return stp;
}
unsafe extern "C" fn freeState(mut stp: *mut gxlstate_t) {
    dtclose((*stp).nodeMap);
    dtclose((*stp).graphMap);
    dtclose((*stp).synNodeMap);
    dtclose((*stp).idList);
    free(stp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gv_to_gxl(mut g: *mut Agraph_t, mut gxlFile: *mut FILE) {
    let mut stp: *mut gxlstate_t = initState(g);
    aginit(
        g,
        1 as libc::c_int,
        b"node\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Local_Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    iterateHdr(stp, g);
    iterateBody(stp, g);
    Level = 0 as libc::c_int;
    fprintf(
        gxlFile,
        b"<?xml version=\"1.0\" encoding=\"iso-8859-1\"?>\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(gxlFile, b"<gxl>\n\0" as *const u8 as *const libc::c_char);
    writeHdr(stp, g, gxlFile, (0 as libc::c_int == 0) as libc::c_int);
    writeBody(stp, g, gxlFile);
    writeTrl(g, gxlFile, (0 as libc::c_int == 0) as libc::c_int);
    fprintf(gxlFile, b"</gxl>\n\0" as *const u8 as *const libc::c_char);
    freeState(stp);
}
