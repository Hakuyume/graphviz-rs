#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sfnew(
        _: *mut Sfio_t,
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut Sfio_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn vmstrdup(_: *mut Vmalloc_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn _err_msg(_: libc::c_int, _: *const libc::c_char, _: ...);
}
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfio_s {
    pub next: *mut libc::c_uchar,
    pub endw: *mut libc::c_uchar,
    pub endr: *mut libc::c_uchar,
    pub endb: *mut libc::c_uchar,
    pub push: *mut Sfio_t,
    pub flags: libc::c_ushort,
    pub file: libc::c_short,
    pub data: *mut libc::c_uchar,
    pub size: ssize_t,
    pub val: ssize_t,
}
pub type Sfio_t = _sfio_s;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vmalloc_s {
    pub allocated: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub type Vmalloc_t = _vmalloc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union EX_STYPE {
    pub expr: *mut Exnode_s,
    pub floating: libc::c_double,
    pub reference: *mut Exref_s,
    pub id: *mut Exid_s,
    pub integer: libc::c_longlong,
    pub op: libc::c_int,
    pub string: *mut libc::c_char,
    pub user: *mut libc::c_void,
    pub buffer: *mut Exbuf_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exbuf_s {
    pub size: uint64_t,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exid_s {
    pub link: Dtlink_t,
    pub lex: libc::c_long,
    pub index: libc::c_long,
    pub type_0: libc::c_long,
    pub index_type: libc::c_long,
    pub flags: libc::c_long,
    pub value: *mut Exnode_t,
    pub local: Exlocal_t,
    pub isstatic: libc::c_long,
    pub name: [libc::c_char; 32],
}
pub type Exlocal_t = Exlocal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exlocal_s {
    pub number: libc::c_longlong,
    pub pointer: *mut libc::c_char,
}
pub type Exnode_t = Exnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exnode_s {
    pub type_0: libc::c_int,
    pub op: libc::c_int,
    pub binary: libc::c_int,
    pub local: Exlocal_t,
    pub compiled: C2RustUnnamed_6,
    pub data: Exdata_t,
}
pub type Exdata_t = Exdata_u;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Exdata_u {
    pub constant: C2RustUnnamed_5,
    pub operand: C2RustUnnamed_4,
    pub select: C2RustUnnamed_3,
    pub variable: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub symbol: *mut Exid_t,
    pub reference: *mut Exref_t,
    pub index: *mut Exnode_t,
    pub dyna: *mut Exnode_t,
}
pub type Exref_t = Exref_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exref_s {
    pub next: *mut Exref_t,
    pub symbol: *mut Exid_t,
    pub index: *mut Exnode_t,
}
pub type Exid_t = Exid_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub statement: *mut Exnode_t,
    pub next: *mut Exnode_t,
    pub constant: *mut *mut Extype_t,
}
pub type Extype_t = EX_STYPE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub left: *mut Exnode_t,
    pub right: *mut Exnode_t,
    pub last: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub value: Extype_t,
    pub reference: *mut Exid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub floating: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_double,
    >,
    pub integer: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_longlong,
    >,
    pub string: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char) -> *mut libc::c_char,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exdisc_s {
    pub version: uint64_t,
    pub flags: uint64_t,
    pub symbols: *mut Exid_t,
    pub data: *mut *mut libc::c_char,
    pub lib: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
    pub castf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *const libc::c_char,
            libc::c_int,
            *mut Exid_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub convertf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exid_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub binaryf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exnode_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub typename: Option::<
        unsafe extern "C" fn(*mut Expr_t, libc::c_int) -> *mut libc::c_char,
    >,
    pub stringof: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub keyf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            Extype_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> Extype_t,
    >,
    pub errorf: Exerror_f,
    pub getf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exid_t,
            *mut Exref_t,
            *mut libc::c_void,
            libc::c_int,
            *mut Exdisc_t,
        ) -> Extype_t,
    >,
    pub reff: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exid_t,
            *mut Exref_t,
            *mut libc::c_char,
            libc::c_int,
            *mut Exdisc_t,
        ) -> Extype_t,
    >,
    pub setf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exid_t,
            *mut Exref_t,
            *mut libc::c_void,
            libc::c_int,
            Extype_t,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub matchf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *const libc::c_char,
            *mut Exnode_t,
            *const libc::c_char,
            *mut libc::c_void,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub exitf: Exexit_f,
    pub types: *mut libc::c_int,
    pub user: *mut libc::c_void,
}
pub type Exexit_f = Option::<
    unsafe extern "C" fn(*mut Expr_t, *mut Exdisc_t, libc::c_int) -> (),
>;
pub type Exdisc_t = Exdisc_s;
pub type Expr_t = Expr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr_s {
    pub id: *const libc::c_char,
    pub symbols: *mut Dt_t,
    pub more: *const libc::c_char,
    pub file: [*mut Sfio_t; 10],
    pub vm: *mut Vmalloc_t,
}
pub type Exerror_f = Option::<
    unsafe extern "C" fn(
        *mut Expr_t,
        *mut Exdisc_t,
        libc::c_int,
        *const libc::c_char,
        ...
    ) -> libc::c_int,
>;
pub type gvpruserfn = Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvprbinding {
    pub name: *mut libc::c_char,
    pub fn_0: gvpruserfn,
}
pub type trav_type = libc::c_uint;
pub const TV_prepostrev: trav_type = 12;
pub const TV_prepostfwd: trav_type = 11;
pub const TV_prepostdfs: trav_type = 10;
pub const TV_postrev: trav_type = 9;
pub const TV_postfwd: trav_type = 8;
pub const TV_postdfs: trav_type = 7;
pub const TV_rev: trav_type = 6;
pub const TV_fwd: trav_type = 5;
pub const TV_dfs: trav_type = 4;
pub const TV_bfs: trav_type = 3;
pub const TV_en: trav_type = 2;
pub const TV_ne: trav_type = 1;
pub const TV_flat: trav_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gpr_t {
    pub curgraph: *mut Agraph_t,
    pub nextgraph: *mut Agraph_t,
    pub target: *mut Agraph_t,
    pub outgraph: *mut Agraph_t,
    pub curobj: *mut Agobj_t,
    pub tmp: *mut Sfio_t,
    pub dp: *mut Exdisc_t,
    pub errf: Exerror_f,
    pub exitf: Exexit_f,
    pub tgtname: *mut libc::c_char,
    pub infname: *mut libc::c_char,
    pub outFile: *mut Sfio_t,
    pub dfltIO: *mut Agiodisc_t,
    pub tvt: trav_type,
    pub tvroot: *mut Agnode_t,
    pub tvnext: *mut Agnode_t,
    pub tvedge: *mut Agedge_t,
    pub name_used: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub flags: libc::c_int,
    pub bindings: *mut gvprbinding,
    pub n_bindings: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gpr_info {
    pub outFile: *mut Sfio_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub errf: Exerror_f,
    pub exitf: Exexit_f,
    pub flags: libc::c_int,
}
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
static mut name_used: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn validTVT(mut c: libc::c_int) -> libc::c_int {
    return (TV_flat as libc::c_int <= c && c <= TV_prepostrev as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn initGPRState(mut state: *mut Gpr_t, mut vm: *mut Vmalloc_t) {
    let ref mut fresh0 = (*state).tgtname;
    *fresh0 = vmstrdup(vm, b"gvpr_result\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn openGPRState(mut info: *mut gpr_info) -> *mut Gpr_t {
    let mut state: *mut Gpr_t = 0 as *mut Gpr_t;
    state = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<Gpr_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Gpr_t
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<Gpr_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Gpr_t
    };
    if state.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"Could not create gvpr state: out of memory\0" as *const u8
                as *const libc::c_char,
        );
        return state;
    }
    let ref mut fresh1 = (*state).tmp;
    *fresh1 = sfnew(
        0 as *mut Sfio_t,
        0 as *mut libc::c_char as *mut libc::c_void,
        -(1 as libc::c_int) as size_t,
        -(1 as libc::c_int),
        0o2 as libc::c_int | 0o4 as libc::c_int,
    );
    if (*fresh1).is_null() {
        _err_msg(
            2 as libc::c_int,
            b"Could not create state tmpfile\0" as *const u8 as *const libc::c_char,
        );
        free(state as *mut libc::c_void);
        return 0 as *mut Gpr_t;
    }
    (*state).tvt = TV_flat;
    (*state).name_used = name_used;
    let ref mut fresh2 = (*state).tvroot;
    *fresh2 = 0 as *mut Agnode_t;
    let ref mut fresh3 = (*state).tvnext;
    *fresh3 = 0 as *mut Agnode_t;
    let ref mut fresh4 = (*state).tvedge;
    *fresh4 = 0 as *mut Agedge_t;
    let ref mut fresh5 = (*state).outFile;
    *fresh5 = (*info).outFile;
    (*state).argc = (*info).argc;
    let ref mut fresh6 = (*state).argv;
    *fresh6 = (*info).argv;
    let ref mut fresh7 = (*state).errf;
    *fresh7 = (*info).errf;
    (*state).flags = (*info).flags;
    return state;
}
unsafe extern "C" fn bindingcmpf(
    mut key: *const libc::c_void,
    mut ip: *const libc::c_void,
) -> libc::c_int {
    return strcmp(
        (*(key as *const gvprbinding)).name,
        (*(ip as *const gvprbinding)).name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn findBinding(
    mut state: *mut Gpr_t,
    mut fname: *mut libc::c_char,
) -> *mut gvprbinding {
    let mut key: gvprbinding = gvprbinding {
        name: 0 as *mut libc::c_char,
        fn_0: None,
    };
    let mut bp: *mut gvprbinding = 0 as *mut gvprbinding;
    if ((*state).bindings).is_null() {
        _err_msg(
            2 as libc::c_int,
            b"call(\"%s\") failed: no bindings\0" as *const u8 as *const libc::c_char,
            fname,
        );
        return 0 as *mut gvprbinding;
    }
    if fname.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"NULL function name for call()\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut gvprbinding;
    }
    key.name = fname;
    bp = bsearch(
        &mut key as *mut gvprbinding as *const libc::c_void,
        (*state).bindings as *const libc::c_void,
        (*state).n_bindings as size_t,
        ::std::mem::size_of::<gvprbinding>() as libc::c_ulong,
        Some(
            bindingcmpf
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut gvprbinding;
    if bp.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"No binding for \"%s\" in call()\0" as *const u8 as *const libc::c_char,
            fname,
        );
    }
    return bp;
}
#[no_mangle]
pub unsafe extern "C" fn addBindings(
    mut state: *mut Gpr_t,
    mut bindings: *mut gvprbinding,
) {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut bp: *mut gvprbinding = bindings;
    let mut buf: *mut gvprbinding = 0 as *mut gvprbinding;
    let mut bufp: *mut gvprbinding = 0 as *mut gvprbinding;
    while !bp.is_null() && !((*bp).name).is_null() {
        if ((*bp).fn_0).is_some() {
            n += 1;
        }
        bp = bp.offset(1);
    }
    if n == 0 as libc::c_int {
        return;
    }
    buf = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<gvprbinding>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut gvprbinding
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<gvprbinding>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut gvprbinding
    };
    bufp = buf;
    bp = bindings;
    while !((*bp).name).is_null() {
        if ((*bp).fn_0).is_some() {
            *bufp = *bp;
            bufp = bufp.offset(1);
        }
        bp = bp.offset(1);
    }
    qsort(
        buf as *mut libc::c_void,
        n as size_t,
        ::std::mem::size_of::<gvprbinding>() as libc::c_ulong,
        Some(
            bindingcmpf
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let ref mut fresh8 = (*state).bindings;
    *fresh8 = buf;
    (*state).n_bindings = n;
}
#[no_mangle]
pub unsafe extern "C" fn closeGPRState(mut state: *mut Gpr_t) {
    if state.is_null() {
        return;
    }
    name_used = (*state).name_used;
    if !((*state).tmp).is_null() {
        sfclose((*state).tmp);
    }
    free((*state).dp as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
