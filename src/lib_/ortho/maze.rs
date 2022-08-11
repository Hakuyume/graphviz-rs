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
    pub type GVJ_s;
    pub type htmllabel_t;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    static mut odb_flags: libc::c_int;
    fn createSEdge(
        g: *mut sgraph,
        v0: *mut snode,
        v1: *mut snode,
        wt: libc::c_double,
    ) -> *mut sedge;
    fn createSNode(_: *mut sgraph) -> *mut snode;
    fn initSEdges(g: *mut sgraph, maxdeg: libc::c_int);
    fn freeSGraph(_: *mut sgraph);
    fn createSGraph(_: libc::c_int) -> *mut sgraph;
    fn gsave(_: *mut sgraph);
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn partition(_: *mut cell, _: libc::c_int, _: *mut libc::c_int, _: boxf) -> *mut boxf;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_0 {
    pub LL: point,
    pub UR: point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
}
pub type GVJ_t = GVJ_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PostscriptAlias {
    pub name: *mut libc::c_char,
    pub family: *mut libc::c_char,
    pub weight: *mut libc::c_char,
    pub stretch: *mut libc::c_char,
    pub style: *mut libc::c_char,
    pub xfig_code: libc::c_int,
    pub svg_font_family: *mut libc::c_char,
    pub svg_font_weight: *mut libc::c_char,
    pub svg_font_style: *mut libc::c_char,
}
pub type PostscriptAlias = _PostscriptAlias;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct textfont_t {
    pub name: *mut libc::c_char,
    pub color: *mut libc::c_char,
    pub postscript_alias: *mut PostscriptAlias,
    pub size: libc::c_double,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=6")]
    #[bitfield(name = "cnt", ty = "libc::c_uint", bits = "7..=31")]
    pub flags_cnt: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textspan_t {
    pub str_0: *mut libc::c_char,
    pub font: *mut textfont_t,
    pub layout: *mut libc::c_void,
    pub free_layout: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub yoffset_layout: libc::c_double,
    pub yoffset_centerline: libc::c_double,
    pub size: pointf,
    pub just: libc::c_char,
}
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
pub type graph_t = Agraph_s;
pub type node_t = Agnode_s;
pub type edge_t = Agedge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_3,
    pub s: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub p: *mut pointf,
    pub r: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct port {
    pub p: pointf,
    pub theta: libc::c_double,
    pub bp: *mut boxf,
    pub defined: bool,
    pub constrained: bool,
    pub clip: bool,
    pub dyna: bool,
    pub order: libc::c_uchar,
    pub side: libc::c_uchar,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textlabel_t {
    pub text: *mut libc::c_char,
    pub fontname: *mut libc::c_char,
    pub fontcolor: *mut libc::c_char,
    pub charset: libc::c_int,
    pub fontsize: libc::c_double,
    pub dimen: pointf,
    pub space: pointf,
    pub pos: pointf,
    pub u: C2RustUnnamed_4,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub txt: C2RustUnnamed_5,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub span: *mut textspan_t,
    pub nspans: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polygon_t {
    pub regular: libc::c_int,
    pub peripheries: libc::c_int,
    pub sides: libc::c_int,
    pub orientation: libc::c_double,
    pub distortion: libc::c_double,
    pub skew: libc::c_double,
    pub option: libc::c_int,
    pub vertices: *mut pointf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shape_functions {
    pub initfn: Option<unsafe extern "C" fn(*mut node_t) -> ()>,
    pub freefn: Option<unsafe extern "C" fn(*mut node_t) -> ()>,
    pub portfn:
        Option<unsafe extern "C" fn(*mut node_t, *mut libc::c_char, *mut libc::c_char) -> port>,
    pub insidefn: Option<unsafe extern "C" fn(*mut inside_t, pointf) -> bool>,
    pub pboxfn: Option<
        unsafe extern "C" fn(
            *mut node_t,
            *mut port,
            libc::c_int,
            *mut boxf,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub codefn: Option<unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shape_desc {
    pub name: *mut libc::c_char,
    pub fns: *mut shape_functions,
    pub polygon: *mut polygon_t,
    pub usershape: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elist {
    pub list: *mut *mut edge_t,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnodeinfo_t {
    pub hdr: Agrec_t,
    pub shape: *mut shape_desc,
    pub shape_info: *mut libc::c_void,
    pub coord: pointf,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub bb: boxf,
    pub ht: libc::c_double,
    pub lw: libc::c_double,
    pub rw: libc::c_double,
    pub label: *mut textlabel_t,
    pub xlabel: *mut textlabel_t,
    pub alg: *mut libc::c_void,
    pub state: libc::c_char,
    pub gui_state: libc::c_uchar,
    pub clustnode: bool,
    pub pinned: libc::c_uchar,
    pub id: libc::c_int,
    pub heapindex: libc::c_int,
    pub hops: libc::c_int,
    pub pos: *mut libc::c_double,
    pub dist: libc::c_double,
    pub showboxes: libc::c_uchar,
    pub has_port: bool,
    pub rep: *mut node_t,
    pub set: *mut node_t,
    pub node_type: libc::c_char,
    pub mark: size_t,
    pub onstack: libc::c_char,
    pub ranktype: libc::c_char,
    pub weight_class: libc::c_char,
    pub next: *mut node_t,
    pub prev: *mut node_t,
    pub in_0: elist,
    pub out: elist,
    pub flat_out: elist,
    pub flat_in: elist,
    pub other: elist,
    pub clust: *mut graph_t,
    pub UF_size: libc::c_int,
    pub UF_parent: *mut node_t,
    pub inleaf: *mut node_t,
    pub outleaf: *mut node_t,
    pub rank: libc::c_int,
    pub order: libc::c_int,
    pub mval: libc::c_double,
    pub save_in: elist,
    pub save_out: elist,
    pub tree_in: elist,
    pub tree_out: elist,
    pub par: *mut edge_t,
    pub low: libc::c_int,
    pub lim: libc::c_int,
    pub priority: libc::c_int,
    pub pad: [libc::c_double; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub flags: libc::c_int,
    pub nedges: libc::c_int,
    pub edges: [*mut sedge; 6],
    pub nsides: libc::c_int,
    pub sides: *mut *mut snode,
    pub bb: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snode {
    pub n_val: libc::c_int,
    pub n_idx: libc::c_int,
    pub n_dad: *mut snode,
    pub n_edge: *mut sedge,
    pub n_adj: libc::c_short,
    pub save_n_adj: libc::c_short,
    pub cells: [*mut cell; 2],
    pub adj_edge_list: *mut libc::c_int,
    pub index: libc::c_int,
    pub isVert: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sedge {
    pub weight: libc::c_double,
    pub cnt: libc::c_int,
    pub v1: libc::c_int,
    pub v2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sgraph {
    pub nnodes: libc::c_int,
    pub nedges: libc::c_int,
    pub save_nnodes: libc::c_int,
    pub save_nedges: libc::c_int,
    pub nodes: *mut snode,
    pub edges: *mut sedge,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const M_BOTTOM: C2RustUnnamed_6 = 3;
pub const M_LEFT: C2RustUnnamed_6 = 2;
pub const M_TOP: C2RustUnnamed_6 = 1;
pub const M_RIGHT: C2RustUnnamed_6 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maze {
    pub ncells: libc::c_int,
    pub ngcells: libc::c_int,
    pub cells: *mut cell,
    pub gcells: *mut cell,
    pub sg: *mut sgraph,
    pub hchans: *mut Dt_t,
    pub vchans: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snodeitem {
    pub np: *mut snode,
    pub p: pointf,
    pub link: Dtlink_t,
}
#[inline]
unsafe extern "C" fn dfp_cmp(mut f1: libc::c_double, mut f2: libc::c_double) -> libc::c_int {
    let mut d: libc::c_double = f1 - f2;
    if d < -1.0e-7f64 {
        return -(1 as libc::c_int);
    }
    if d > 1.0e-7f64 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut pre: *mut libc::c_char = b"%!PS-Adobe-2.0\n/node {\n  /Y exch def\n  /X exch def\n  /y exch def\n  /x exch def\n  newpath\n  x y moveto\n  x Y lineto\n  X Y lineto\n  X y lineto\n  closepath fill\n} def\n/cell {\n  /Y exch def\n  /X exch def\n  /y exch def\n  /x exch def\n  newpath\n  x y moveto\n  x Y lineto\n  X Y lineto\n  X y lineto\n  closepath stroke\n} def\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut post: *mut libc::c_char =
    b"showpage\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn psdump(
    mut gcells: *mut cell,
    mut n_gcells: libc::c_int,
    mut BB: boxf,
    mut rects: *mut boxf,
    mut nrect: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut absbb: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    absbb.LL.x = 10 as libc::c_int;
    absbb.LL.y = absbb.LL.x;
    absbb.UR.x = (absbb.LL.x as libc::c_double + BB.UR.x - BB.LL.x) as libc::c_int;
    absbb.UR.y = (absbb.LL.y as libc::c_double + BB.UR.y - BB.LL.y) as libc::c_int;
    fputs(pre, stderr);
    fprintf(
        stderr,
        b"%%%%Page: 1 1\n%%%%PageBoundingBox: %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
        absbb.LL.x,
        absbb.LL.y,
        absbb.UR.x,
        absbb.UR.y,
    );
    fprintf(
        stderr,
        b"%f %f translate\n\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int as libc::c_double - BB.LL.x,
        10 as libc::c_int as libc::c_double - BB.LL.y,
    );
    fputs(
        b"0 0 1 setrgbcolor\n\0" as *const u8 as *const libc::c_char,
        stderr,
    );
    i = 0 as libc::c_int;
    while i < n_gcells {
        bb = (*gcells.offset(i as isize)).bb;
        fprintf(
            stderr,
            b"%f %f %f %f node\n\0" as *const u8 as *const libc::c_char,
            bb.LL.x,
            bb.LL.y,
            bb.UR.x,
            bb.UR.y,
        );
        i += 1;
    }
    fputs(
        b"0 0 0 setrgbcolor\n\0" as *const u8 as *const libc::c_char,
        stderr,
    );
    i = 0 as libc::c_int;
    while i < nrect {
        bb = *rects.offset(i as isize);
        fprintf(
            stderr,
            b"%f %f %f %f cell\n\0" as *const u8 as *const libc::c_char,
            bb.LL.x,
            bb.LL.y,
            bb.UR.x,
            bb.UR.y,
        );
        i += 1;
    }
    fputs(
        b"1 0 0 setrgbcolor\n\0" as *const u8 as *const libc::c_char,
        stderr,
    );
    fprintf(
        stderr,
        b"%f %f %f %f cell\n\0" as *const u8 as *const libc::c_char,
        BB.LL.x,
        BB.LL.y,
        BB.UR.x,
        BB.UR.y,
    );
    fputs(post, stderr);
}
unsafe extern "C" fn vcmpid(
    mut d: *mut Dt_t,
    mut key1: *mut pointf,
    mut key2: *mut pointf,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut dx: libc::c_int = dfp_cmp((*key1).x, (*key2).x);
    if dx != 0 as libc::c_int {
        return dx;
    }
    return dfp_cmp((*key1).y, (*key2).y);
}
unsafe extern "C" fn hcmpid(
    mut d: *mut Dt_t,
    mut key1: *mut pointf,
    mut key2: *mut pointf,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut dy: libc::c_int = dfp_cmp((*key1).y, (*key2).y);
    if dy != 0 as libc::c_int {
        return dy;
    }
    return dfp_cmp((*key1).x, (*key2).x);
}
static mut vdictDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 8 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<pointf>() as libc::c_ulong as libc::c_int,
            link: 24 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut pointf,
                        *mut pointf,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(Some(
                vcmpid
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut pointf,
                        *mut pointf,
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
static mut hdictDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 8 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<pointf>() as libc::c_ulong as libc::c_int,
            link: 24 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut pointf,
                        *mut pointf,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(Some(
                hcmpid
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut pointf,
                        *mut pointf,
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
unsafe extern "C" fn updateWt(mut ep: *mut sedge, mut sz: libc::c_int) {
    let ref mut fresh0 = (*ep).cnt;
    *fresh0 += 1;
    if (*ep).cnt > sz {
        (*ep).cnt = 0 as libc::c_int;
        (*ep).weight += 16384 as libc::c_int as libc::c_double;
    }
}
#[no_mangle]
pub unsafe extern "C" fn updateWts(mut g: *mut sgraph, mut cp: *mut cell, mut ep: *mut sedge) {
    let mut i: libc::c_int = 0;
    let mut e: *mut sedge = 0 as *mut sedge;
    let mut isBend: libc::c_int = ((*((*g).nodes).offset((*ep).v1 as isize)).isVert as libc::c_int
        != (*((*g).nodes).offset((*ep).v2 as isize)).isVert as libc::c_int)
        as libc::c_int;
    let mut hsz: libc::c_int =
        (((*cp).bb.UR.y - (*cp).bb.LL.y - 3 as libc::c_int as libc::c_double)
            / 2 as libc::c_int as libc::c_double) as libc::c_int;
    let mut vsz: libc::c_int =
        (((*cp).bb.UR.x - (*cp).bb.LL.x - 3 as libc::c_int as libc::c_double)
            / 2 as libc::c_int as libc::c_double) as libc::c_int;
    let mut minsz: libc::c_int = if hsz < vsz { hsz } else { vsz };
    i = 0 as libc::c_int;
    while i < (*cp).nedges {
        e = (*cp).edges[i as usize];
        if !((*((*g).nodes).offset((*e).v1 as isize)).isVert as libc::c_int
            != (*((*g).nodes).offset((*e).v2 as isize)).isVert as libc::c_int)
        {
            break;
        }
        updateWt(e, minsz);
        i += 1;
    }
    while i < (*cp).nedges {
        e = (*cp).edges[i as usize];
        if isBend != 0 || e == ep {
            updateWt(
                e,
                if (*((*g).nodes).offset((*e).v1 as isize)).isVert as libc::c_int != 0 {
                    hsz
                } else {
                    vsz
                },
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn markSmall(mut cp: *mut cell) {
    let mut i: libc::c_int = 0;
    let mut onp: *mut snode = 0 as *mut snode;
    let mut ocp: *mut cell = 0 as *mut cell;
    if (((*cp).bb.UR.y - (*cp).bb.LL.y - 3 as libc::c_int as libc::c_double)
        / 2 as libc::c_int as libc::c_double)
        < 2 as libc::c_int as libc::c_double
    {
        i = 0 as libc::c_int;
        while i < (*cp).nsides {
            onp = *((*cp).sides).offset(i as isize);
            if (*onp).isVert {
                if (*onp).cells[0 as libc::c_int as usize] == cp {
                    ocp = (*onp).cells[1 as libc::c_int as usize];
                    (*ocp).flags |= 8 as libc::c_int;
                    loop {
                        onp = *((*ocp).sides).offset(M_RIGHT as libc::c_int as isize);
                        if !(!onp.is_null()
                            && (*(*onp).cells[1 as libc::c_int as usize]).flags & 1 as libc::c_int
                                == 0)
                        {
                            break;
                        }
                        ocp = (*onp).cells[1 as libc::c_int as usize];
                        (*ocp).flags |= 8 as libc::c_int;
                    }
                } else {
                    ocp = (*onp).cells[0 as libc::c_int as usize];
                    (*ocp).flags |= 8 as libc::c_int;
                    loop {
                        onp = *((*ocp).sides).offset(M_LEFT as libc::c_int as isize);
                        if !(!onp.is_null()
                            && (*(*onp).cells[0 as libc::c_int as usize]).flags & 1 as libc::c_int
                                == 0)
                        {
                            break;
                        }
                        ocp = (*onp).cells[0 as libc::c_int as usize];
                        (*ocp).flags |= 8 as libc::c_int;
                    }
                }
            }
            i += 1;
        }
    }
    if (((*cp).bb.UR.x - (*cp).bb.LL.x - 3 as libc::c_int as libc::c_double)
        / 2 as libc::c_int as libc::c_double)
        < 2 as libc::c_int as libc::c_double
    {
        i = 0 as libc::c_int;
        while i < (*cp).nsides {
            onp = *((*cp).sides).offset(i as isize);
            if !(*onp).isVert {
                if (*onp).cells[0 as libc::c_int as usize] == cp {
                    ocp = (*onp).cells[1 as libc::c_int as usize];
                    (*ocp).flags |= 16 as libc::c_int;
                    loop {
                        onp = *((*ocp).sides).offset(M_TOP as libc::c_int as isize);
                        if !(!onp.is_null()
                            && (*(*onp).cells[1 as libc::c_int as usize]).flags & 1 as libc::c_int
                                == 0)
                        {
                            break;
                        }
                        ocp = (*onp).cells[1 as libc::c_int as usize];
                        (*ocp).flags |= 16 as libc::c_int;
                    }
                } else {
                    ocp = (*onp).cells[0 as libc::c_int as usize];
                    (*ocp).flags |= 16 as libc::c_int;
                    loop {
                        onp = *((*ocp).sides).offset(M_BOTTOM as libc::c_int as isize);
                        if !(!onp.is_null()
                            && (*(*onp).cells[0 as libc::c_int as usize]).flags & 1 as libc::c_int
                                == 0)
                        {
                            break;
                        }
                        ocp = (*onp).cells[0 as libc::c_int as usize];
                        (*ocp).flags |= 16 as libc::c_int;
                    }
                }
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn createSEdges(mut cp: *mut cell, mut g: *mut sgraph) {
    let mut bb: boxf = (*cp).bb;
    let mut hwt: libc::c_double = 1 as libc::c_int as libc::c_double * (bb.UR.x - bb.LL.x);
    let mut vwt: libc::c_double = 1 as libc::c_int as libc::c_double * (bb.UR.y - bb.LL.y);
    let mut wt: libc::c_double = (hwt + vwt) / 2.0f64 + 500 as libc::c_int as libc::c_double;
    if ((bb.UR.y - bb.LL.y - 3 as libc::c_int as libc::c_double)
        / 2 as libc::c_int as libc::c_double)
        < 2 as libc::c_int as libc::c_double
        && (*cp).flags & 8 as libc::c_int == 0
    {
        hwt = 16384 as libc::c_int as libc::c_double;
        wt = 16384 as libc::c_int as libc::c_double;
    }
    if ((bb.UR.x - bb.LL.x - 3 as libc::c_int as libc::c_double)
        / 2 as libc::c_int as libc::c_double)
        < 2 as libc::c_int as libc::c_double
        && (*cp).flags & 16 as libc::c_int == 0
    {
        vwt = 16384 as libc::c_int as libc::c_double;
        wt = 16384 as libc::c_int as libc::c_double;
    }
    if !(*((*cp).sides).offset(M_LEFT as libc::c_int as isize)).is_null()
        && !(*((*cp).sides).offset(M_TOP as libc::c_int as isize)).is_null()
    {
        let ref mut fresh1 = (*cp).nedges;
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        let ref mut fresh3 = (*cp).edges[fresh2 as usize];
        *fresh3 = createSEdge(
            g,
            *((*cp).sides).offset(M_LEFT as libc::c_int as isize),
            *((*cp).sides).offset(M_TOP as libc::c_int as isize),
            wt,
        );
    }
    if !(*((*cp).sides).offset(M_TOP as libc::c_int as isize)).is_null()
        && !(*((*cp).sides).offset(M_RIGHT as libc::c_int as isize)).is_null()
    {
        let ref mut fresh4 = (*cp).nedges;
        let fresh5 = *fresh4;
        *fresh4 = *fresh4 + 1;
        let ref mut fresh6 = (*cp).edges[fresh5 as usize];
        *fresh6 = createSEdge(
            g,
            *((*cp).sides).offset(M_TOP as libc::c_int as isize),
            *((*cp).sides).offset(M_RIGHT as libc::c_int as isize),
            wt,
        );
    }
    if !(*((*cp).sides).offset(M_LEFT as libc::c_int as isize)).is_null()
        && !(*((*cp).sides).offset(M_BOTTOM as libc::c_int as isize)).is_null()
    {
        let ref mut fresh7 = (*cp).nedges;
        let fresh8 = *fresh7;
        *fresh7 = *fresh7 + 1;
        let ref mut fresh9 = (*cp).edges[fresh8 as usize];
        *fresh9 = createSEdge(
            g,
            *((*cp).sides).offset(M_LEFT as libc::c_int as isize),
            *((*cp).sides).offset(M_BOTTOM as libc::c_int as isize),
            wt,
        );
    }
    if !(*((*cp).sides).offset(M_BOTTOM as libc::c_int as isize)).is_null()
        && !(*((*cp).sides).offset(M_RIGHT as libc::c_int as isize)).is_null()
    {
        let ref mut fresh10 = (*cp).nedges;
        let fresh11 = *fresh10;
        *fresh10 = *fresh10 + 1;
        let ref mut fresh12 = (*cp).edges[fresh11 as usize];
        *fresh12 = createSEdge(
            g,
            *((*cp).sides).offset(M_BOTTOM as libc::c_int as isize),
            *((*cp).sides).offset(M_RIGHT as libc::c_int as isize),
            wt,
        );
    }
    if !(*((*cp).sides).offset(M_TOP as libc::c_int as isize)).is_null()
        && !(*((*cp).sides).offset(M_BOTTOM as libc::c_int as isize)).is_null()
    {
        let ref mut fresh13 = (*cp).nedges;
        let fresh14 = *fresh13;
        *fresh13 = *fresh13 + 1;
        let ref mut fresh15 = (*cp).edges[fresh14 as usize];
        *fresh15 = createSEdge(
            g,
            *((*cp).sides).offset(M_TOP as libc::c_int as isize),
            *((*cp).sides).offset(M_BOTTOM as libc::c_int as isize),
            vwt,
        );
    }
    if !(*((*cp).sides).offset(M_LEFT as libc::c_int as isize)).is_null()
        && !(*((*cp).sides).offset(M_RIGHT as libc::c_int as isize)).is_null()
    {
        let ref mut fresh16 = (*cp).nedges;
        let fresh17 = *fresh16;
        *fresh16 = *fresh16 + 1;
        let ref mut fresh18 = (*cp).edges[fresh17 as usize];
        *fresh18 = createSEdge(
            g,
            *((*cp).sides).offset(M_LEFT as libc::c_int as isize),
            *((*cp).sides).offset(M_RIGHT as libc::c_int as isize),
            hwt,
        );
    }
}
unsafe extern "C" fn findSVert(
    mut g: *mut sgraph,
    mut cdt: *mut Dt_t,
    mut p: pointf,
    mut ditems: *mut snodeitem,
    mut isVert: bool,
) -> *mut snode {
    let mut n: *mut snodeitem = (Some(((*cdt).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        cdt,
        &mut p as *mut pointf as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut snodeitem;
    if n.is_null() {
        let mut np: *mut snode = createSNode(g);
        if !ditems.is_null() {
        } else {
            __assert_fail(
                b"ditems\0" as *const u8 as *const libc::c_char,
                b"maze.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"snode *findSVert(sgraph *, Dt_t *, pointf, snodeitem *, _Bool)\0",
                ))
                .as_ptr(),
            );
        }
        n = ditems.offset((*np).index as isize);
        (*n).p = p;
        let ref mut fresh19 = (*n).np;
        *fresh19 = np;
        (*np).isVert = isVert;
        (Some(((*cdt).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            cdt, n as *mut libc::c_void, 0o1 as libc::c_int
        );
    }
    return (*n).np;
}
unsafe extern "C" fn chkSgraph(mut g: *mut sgraph) {
    let mut i: libc::c_int = 0;
    let mut np: *mut snode = 0 as *mut snode;
    i = 0 as libc::c_int;
    while i < (*g).nnodes {
        np = ((*g).nodes).offset(i as isize);
        if ((*np).cells[0 as libc::c_int as usize]).is_null() {
            fprintf(
                stderr,
                b"failed at node %d[0]\n\0" as *const u8 as *const libc::c_char,
                i,
            );
        }
        if !((*np).cells[0 as libc::c_int as usize]).is_null() {
        } else {
            __assert_fail(
                b"np->cells[0]\0" as *const u8 as *const libc::c_char,
                b"maze.c\0" as *const u8 as *const libc::c_char,
                328 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"void chkSgraph(sgraph *)\0",
                ))
                .as_ptr(),
            );
        }
        if ((*np).cells[1 as libc::c_int as usize]).is_null() {
            fprintf(
                stderr,
                b"failed at node %d[1]\n\0" as *const u8 as *const libc::c_char,
                i,
            );
        }
        if !((*np).cells[1 as libc::c_int as usize]).is_null() {
        } else {
            __assert_fail(
                b"np->cells[1]\0" as *const u8 as *const libc::c_char,
                b"maze.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"void chkSgraph(sgraph *)\0",
                ))
                .as_ptr(),
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn mkMazeGraph(mut mp: *mut maze, mut bb: boxf) -> *mut sgraph {
    let mut nsides: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut maxdeg: libc::c_int = 0;
    let mut bound: libc::c_int = 4 as libc::c_int * (*mp).ncells;
    let mut g: *mut sgraph = createSGraph(bound + 2 as libc::c_int);
    let mut vdict: *mut Dt_t = dtopen(&mut vdictDisc, Dtoset);
    let mut hdict: *mut Dt_t = dtopen(&mut hdictDisc, Dtoset);
    let mut ditems: *mut snodeitem = gcalloc(
        bound as size_t,
        ::std::mem::size_of::<snodeitem>() as libc::c_ulong,
    ) as *mut snodeitem;
    let mut sides: *mut *mut snode = 0 as *mut *mut snode;
    sides = gcalloc(
        (4 as libc::c_int * (*mp).ncells) as size_t,
        ::std::mem::size_of::<*mut snode>() as libc::c_ulong,
    ) as *mut *mut snode;
    i = 0 as libc::c_int;
    while i < (*mp).ncells {
        let mut cp: *mut cell = ((*mp).cells).offset(i as isize);
        let mut np: *mut snode = 0 as *mut snode;
        let mut pt: pointf = pointf { x: 0., y: 0. };
        (*cp).nsides = 4 as libc::c_int;
        let ref mut fresh20 = (*cp).sides;
        *fresh20 = sides.offset((4 as libc::c_int * i) as isize);
        if (*cp).bb.UR.x < bb.UR.x {
            pt.x = (*cp).bb.UR.x;
            pt.y = (*cp).bb.LL.y;
            np = findSVert(g, vdict, pt, ditems, 1 as libc::c_int != 0);
            let ref mut fresh21 = (*np).cells[0 as libc::c_int as usize];
            *fresh21 = cp;
            let ref mut fresh22 = *((*cp).sides).offset(M_RIGHT as libc::c_int as isize);
            *fresh22 = np;
        }
        if (*cp).bb.UR.y < bb.UR.y {
            pt.x = (*cp).bb.LL.x;
            pt.y = (*cp).bb.UR.y;
            np = findSVert(g, hdict, pt, ditems, 0 as libc::c_int != 0);
            let ref mut fresh23 = (*np).cells[0 as libc::c_int as usize];
            *fresh23 = cp;
            let ref mut fresh24 = *((*cp).sides).offset(M_TOP as libc::c_int as isize);
            *fresh24 = np;
        }
        if (*cp).bb.LL.x > bb.LL.x {
            np = findSVert(g, vdict, (*cp).bb.LL, ditems, 1 as libc::c_int != 0);
            let ref mut fresh25 = (*np).cells[1 as libc::c_int as usize];
            *fresh25 = cp;
            let ref mut fresh26 = *((*cp).sides).offset(M_LEFT as libc::c_int as isize);
            *fresh26 = np;
        }
        if (*cp).bb.LL.y > bb.LL.y {
            np = findSVert(g, hdict, (*cp).bb.LL, ditems, 0 as libc::c_int != 0);
            let ref mut fresh27 = (*np).cells[1 as libc::c_int as usize];
            *fresh27 = cp;
            let ref mut fresh28 = *((*cp).sides).offset(M_BOTTOM as libc::c_int as isize);
            *fresh28 = np;
        }
        i += 1;
    }
    maxdeg = 0 as libc::c_int;
    sides = gcalloc(
        (*g).nnodes as size_t,
        ::std::mem::size_of::<*mut snode>() as libc::c_ulong,
    ) as *mut *mut snode;
    nsides = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*mp).ngcells {
        let mut cp_0: *mut cell = ((*mp).gcells).offset(i as isize);
        let mut pt_0: pointf = pointf { x: 0., y: 0. };
        let mut np_0: *mut snodeitem = 0 as *mut snodeitem;
        let ref mut fresh29 = (*cp_0).sides;
        *fresh29 = sides.offset(nsides as isize);
        pt_0 = (*cp_0).bb.LL;
        np_0 = (Some(((*hdict).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            hdict,
            &mut pt_0 as *mut pointf as *mut libc::c_void,
            0o1000 as libc::c_int,
        ) as *mut snodeitem;
        while !np_0.is_null() && (*np_0).p.x < (*cp_0).bb.UR.x {
            let ref mut fresh30 = (*cp_0).nsides;
            let fresh31 = *fresh30;
            *fresh30 = *fresh30 + 1;
            let ref mut fresh32 = *((*cp_0).sides).offset(fresh31 as isize);
            *fresh32 = (*np_0).np;
            let ref mut fresh33 = (*(*np_0).np).cells[1 as libc::c_int as usize];
            *fresh33 = cp_0;
            np_0 = (Some(((*hdict).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                hdict,
                np_0 as *mut libc::c_void,
                0o10 as libc::c_int,
            ) as *mut snodeitem;
        }
        np_0 = (Some(((*vdict).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            vdict,
            &mut pt_0 as *mut pointf as *mut libc::c_void,
            0o1000 as libc::c_int,
        ) as *mut snodeitem;
        while !np_0.is_null() && (*np_0).p.y < (*cp_0).bb.UR.y {
            let ref mut fresh34 = (*cp_0).nsides;
            let fresh35 = *fresh34;
            *fresh34 = *fresh34 + 1;
            let ref mut fresh36 = *((*cp_0).sides).offset(fresh35 as isize);
            *fresh36 = (*np_0).np;
            let ref mut fresh37 = (*(*np_0).np).cells[1 as libc::c_int as usize];
            *fresh37 = cp_0;
            np_0 = (Some(((*vdict).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                vdict,
                np_0 as *mut libc::c_void,
                0o10 as libc::c_int,
            ) as *mut snodeitem;
        }
        pt_0.y = (*cp_0).bb.UR.y;
        np_0 = (Some(((*hdict).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            hdict,
            &mut pt_0 as *mut pointf as *mut libc::c_void,
            0o1000 as libc::c_int,
        ) as *mut snodeitem;
        while !np_0.is_null() && (*np_0).p.x < (*cp_0).bb.UR.x {
            let ref mut fresh38 = (*cp_0).nsides;
            let fresh39 = *fresh38;
            *fresh38 = *fresh38 + 1;
            let ref mut fresh40 = *((*cp_0).sides).offset(fresh39 as isize);
            *fresh40 = (*np_0).np;
            let ref mut fresh41 = (*(*np_0).np).cells[0 as libc::c_int as usize];
            *fresh41 = cp_0;
            np_0 = (Some(((*hdict).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                hdict,
                np_0 as *mut libc::c_void,
                0o10 as libc::c_int,
            ) as *mut snodeitem;
        }
        pt_0.x = (*cp_0).bb.UR.x;
        pt_0.y = (*cp_0).bb.LL.y;
        np_0 = (Some(((*vdict).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            vdict,
            &mut pt_0 as *mut pointf as *mut libc::c_void,
            0o1000 as libc::c_int,
        ) as *mut snodeitem;
        while !np_0.is_null() && (*np_0).p.y < (*cp_0).bb.UR.y {
            let ref mut fresh42 = (*cp_0).nsides;
            let fresh43 = *fresh42;
            *fresh42 = *fresh42 + 1;
            let ref mut fresh44 = *((*cp_0).sides).offset(fresh43 as isize);
            *fresh44 = (*np_0).np;
            let ref mut fresh45 = (*(*np_0).np).cells[0 as libc::c_int as usize];
            *fresh45 = cp_0;
            np_0 = (Some(((*vdict).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                vdict,
                np_0 as *mut libc::c_void,
                0o10 as libc::c_int,
            ) as *mut snodeitem;
        }
        nsides += (*cp_0).nsides;
        if (*cp_0).nsides > maxdeg {
            maxdeg = (*cp_0).nsides;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*mp).ngcells {
        let mut cp_1: *mut cell = ((*mp).gcells).offset(i as isize);
        markSmall(cp_1);
        i += 1;
    }
    (*((*g).nodes).offset((*g).nnodes as isize)).index = (*g).nnodes;
    (*((*g).nodes).offset(((*g).nnodes + 1 as libc::c_int) as isize)).index =
        (*g).nnodes + 1 as libc::c_int;
    initSEdges(g, maxdeg);
    i = 0 as libc::c_int;
    while i < (*mp).ncells {
        let mut cp_2: *mut cell = ((*mp).cells).offset(i as isize);
        createSEdges(cp_2, g);
        i += 1;
    }
    dtclose(vdict);
    dtclose(hdict);
    free(ditems as *mut libc::c_void);
    chkSgraph(g);
    gsave(g);
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn mkMaze(mut g: *mut graph_t) -> *mut maze {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut mp: *mut maze = zmalloc(::std::mem::size_of::<maze>() as libc::c_ulong) as *mut maze;
    let mut rects: *mut boxf = 0 as *mut boxf;
    let mut i: libc::c_int = 0;
    let mut nrect: libc::c_int = 0;
    let mut cp: *mut cell = 0 as *mut cell;
    let mut w2: libc::c_double = 0.;
    let mut h2: libc::c_double = 0.;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut BB: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    (*mp).ngcells = agnnodes(g);
    let ref mut fresh46 = (*mp).gcells;
    *fresh46 = gcalloc(
        (*mp).ngcells as size_t,
        ::std::mem::size_of::<cell>() as libc::c_ulong,
    ) as *mut cell;
    cp = *fresh46;
    BB.LL.y = 1.7976931348623157e+308f64;
    BB.LL.x = BB.LL.y;
    BB.UR.y = -1.7976931348623157e+308f64;
    BB.UR.x = BB.UR.y;
    n = agfstnode(g);
    while !n.is_null() {
        w2 = fmax(
            1 as libc::c_int as libc::c_double,
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
                / 2.0f64,
        );
        h2 = fmax(
            1 as libc::c_int as libc::c_double,
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64,
        );
        bb.LL.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .coord
            .x
            - w2;
        bb.UR.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .coord
            .x
            + w2;
        bb.LL.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .coord
            .y
            - h2;
        bb.UR.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .coord
            .y
            + h2;
        BB.LL.x = fmin(BB.LL.x, bb.LL.x);
        BB.LL.y = fmin(BB.LL.y, bb.LL.y);
        BB.UR.x = fmax(BB.UR.x, bb.UR.x);
        BB.UR.y = fmax(BB.UR.y, bb.UR.y);
        (*cp).bb = bb;
        (*cp).flags |= 1 as libc::c_int;
        let ref mut fresh47 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg;
        *fresh47 = cp as *mut libc::c_void;
        cp = cp.offset(1);
        n = agnxtnode(g, n);
    }
    BB.LL.x -= 36 as libc::c_int as libc::c_double;
    BB.LL.y -= 36 as libc::c_int as libc::c_double;
    BB.UR.x += 36 as libc::c_int as libc::c_double;
    BB.UR.y += 36 as libc::c_int as libc::c_double;
    rects = partition((*mp).gcells, (*mp).ngcells, &mut nrect, BB);
    if odb_flags & 1 as libc::c_int != 0 {
        psdump((*mp).gcells, (*mp).ngcells, BB, rects, nrect);
    }
    let ref mut fresh48 = (*mp).cells;
    *fresh48 = gcalloc(
        nrect as size_t,
        ::std::mem::size_of::<cell>() as libc::c_ulong,
    ) as *mut cell;
    (*mp).ncells = nrect;
    i = 0 as libc::c_int;
    while i < nrect {
        (*((*mp).cells).offset(i as isize)).bb = *rects.offset(i as isize);
        i += 1;
    }
    free(rects as *mut libc::c_void);
    let ref mut fresh49 = (*mp).sg;
    *fresh49 = mkMazeGraph(mp, BB);
    return mp;
}
#[no_mangle]
pub unsafe extern "C" fn freeMaze(mut mp: *mut maze) {
    free((*((*mp).cells).offset(0 as libc::c_int as isize)).sides as *mut libc::c_void);
    free((*((*mp).gcells).offset(0 as libc::c_int as isize)).sides as *mut libc::c_void);
    free((*mp).cells as *mut libc::c_void);
    free((*mp).gcells as *mut libc::c_void);
    freeSGraph((*mp).sg);
    dtclose((*mp).hchans);
    dtclose((*mp).vchans);
    free(mp as *mut libc::c_void);
}
