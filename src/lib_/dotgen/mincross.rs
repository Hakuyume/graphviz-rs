#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    pub type htmllabel_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn agdegree(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        in_0: libc::c_int,
        out: libc::c_int,
    ) -> libc::c_int;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
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
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agcontains(_: *mut Agraph_t, _: *mut libc::c_void) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelnode(g: *mut Agraph_t, arg_n: *mut Agnode_t) -> libc::c_int;
    fn agdeledge(g: *mut Agraph_t, arg_e: *mut Agedge_t) -> libc::c_int;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agwarningf(fmt: *const libc::c_char, _: ...);
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Agstrictdirected: Agdesc_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut Verbose: libc::c_uchar;
    fn new_virtual_edge(
        _: *mut Agnode_t,
        _: *mut Agnode_t,
        _: *mut Agedge_t,
    ) -> *mut Agedge_t;
    fn merge_oneway(_: *mut Agedge_t, _: *mut Agedge_t);
    fn mark_lowclusters(_: *mut Agraph_t);
    fn is_cluster(_: *mut Agraph_t) -> bool;
    fn flat_edge(_: *mut Agraph_t, _: *mut Agedge_t);
    fn find_flat_edge(_: *mut Agnode_t, _: *mut Agnode_t) -> *mut Agedge_t;
    fn expand_cluster(_: *mut Agraph_t);
    fn delete_flat_edge(_: *mut Agedge_t);
    fn decompose(_: *mut Agraph_t, _: libc::c_int);
    fn class2(_: *mut Agraph_t);
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn new_queue(_: libc::c_int) -> *mut nodequeue;
    fn free_queue(_: *mut nodequeue);
    fn enqueue(_: *mut nodequeue, _: *mut Agnode_t);
    fn dequeue(_: *mut nodequeue) -> *mut Agnode_t;
    fn late_string(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn install_cluster(
        _: *mut Agraph_t,
        _: *mut Agnode_t,
        _: libc::c_int,
        _: *mut nodequeue,
    );
    fn dot_root(_: *mut libc::c_void) -> *mut Agraph_t;
    static mut MaxIter: libc::c_int;
    static mut G_ordering: *mut Agsym_t;
    static mut N_ordering: *mut Agsym_t;
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
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVJ_s {
    pub gvc: *mut GVC_t,
    pub next: *mut GVJ_t,
    pub next_active: *mut GVJ_t,
    pub common: *mut GVCOMMON_t,
    pub obj: *mut obj_state_t,
    pub input_filename: *mut libc::c_char,
    pub graph_index: libc::c_int,
    pub layout_type: *const libc::c_char,
    pub output_filename: *const libc::c_char,
    pub output_file: *mut FILE,
    pub output_data: *mut libc::c_char,
    pub output_data_allocated: libc::c_uint,
    pub output_data_position: libc::c_uint,
    pub output_langname: *const libc::c_char,
    pub output_lang: libc::c_int,
    pub render: gvplugin_active_render_t,
    pub device: gvplugin_active_device_t,
    pub loadimage: gvplugin_active_loadimage_t,
    pub callbacks: *mut gvdevice_callbacks_t,
    pub device_dpi: pointf,
    pub device_sets_dpi: bool,
    pub display: *mut libc::c_void,
    pub screen: libc::c_int,
    pub context: *mut libc::c_void,
    pub external_context: bool,
    pub imagedata: *mut libc::c_char,
    pub flags: libc::c_int,
    pub numLayers: libc::c_int,
    pub layerNum: libc::c_int,
    pub pagesArraySize: point,
    pub pagesArrayFirst: point,
    pub pagesArrayMajor: point,
    pub pagesArrayMinor: point,
    pub pagesArrayElem: point,
    pub numPages: libc::c_int,
    pub bb: boxf,
    pub pad: pointf,
    pub clip: boxf,
    pub pageBox: boxf,
    pub pageSize: pointf,
    pub focus: pointf,
    pub zoom: libc::c_double,
    pub rotation: libc::c_int,
    pub view: pointf,
    pub canvasBox: boxf,
    pub margin: pointf,
    pub dpi: pointf,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub pageBoundingBox: box_0,
    pub boundingBox: box_0,
    pub scale: pointf,
    pub translation: pointf,
    pub devscale: pointf,
    pub fit_mode: bool,
    pub needs_refresh: bool,
    pub click: bool,
    pub has_grown: bool,
    pub has_been_rendered: bool,
    pub button: libc::c_uchar,
    pub pointer: pointf,
    pub oldpointer: pointf,
    pub current_obj: *mut libc::c_void,
    pub selected_obj: *mut libc::c_void,
    pub active_tooltip: *mut libc::c_char,
    pub selected_href: *mut libc::c_char,
    pub selected_obj_type_name: gv_argvlist_t,
    pub selected_obj_attributes: gv_argvlist_t,
    pub window: *mut libc::c_void,
    pub keybindings: *mut gvevent_key_binding_t,
    pub numkeys: libc::c_int,
    pub keycodes: *mut libc::c_void,
}
pub type gvevent_key_binding_t = gvevent_key_binding_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvevent_key_binding_s {
    pub keystring: *mut libc::c_char,
    pub callback: gvevent_key_callback_t,
}
pub type gvevent_key_callback_t = Option::<
    unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int,
>;
pub type GVJ_t = GVJ_s;
pub type gv_argvlist_t = gv_argvlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_argvlist_s {
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
    pub alloc: libc::c_int,
}
pub type gvdevice_callbacks_t = gvdevice_callbacks_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_callbacks_s {
    pub refresh: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub button_press: Option::<
        unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> (),
    >,
    pub button_release: Option::<
        unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> (),
    >,
    pub motion: Option::<unsafe extern "C" fn(*mut GVJ_t, pointf) -> ()>,
    pub modify: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> (),
    >,
    pub del: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub read: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> (),
    >,
    pub layout: Option::<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> ()>,
    pub render: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_loadimage_t {
    pub engine: *mut gvloadimage_engine_t,
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
}
pub type gvloadimage_engine_t = gvloadimage_engine_s;
pub type gvplugin_active_device_t = gvplugin_active_device_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_device_s {
    pub engine: *mut gvdevice_engine_t,
    pub id: libc::c_int,
    pub features: *mut gvdevice_features_t,
    pub type_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_features_t {
    pub flags: libc::c_int,
    pub default_margin: pointf,
    pub default_pagesize: pointf,
    pub default_dpi: pointf,
}
pub type gvdevice_engine_t = gvdevice_engine_s;
pub type gvplugin_active_render_t = gvplugin_active_render_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_render_s {
    pub engine: *mut gvrender_engine_t,
    pub id: libc::c_int,
    pub features: *mut gvrender_features_t,
    pub type_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvrender_features_t {
    pub flags: libc::c_int,
    pub default_pad: libc::c_double,
    pub knowncolors: *mut *mut libc::c_char,
    pub sz_knowncolors: libc::c_int,
    pub color_type: color_type_t,
}
pub type color_type_t = libc::c_uint;
pub const COLOR_INDEX: color_type_t = 6;
pub const COLOR_STRING: color_type_t = 5;
pub const RGBA_DOUBLE: color_type_t = 4;
pub const CMYK_BYTE: color_type_t = 3;
pub const RGBA_WORD: color_type_t = 2;
pub const RGBA_BYTE: color_type_t = 1;
pub const HSVA_DOUBLE: color_type_t = 0;
pub type gvrender_engine_t = gvrender_engine_s;
pub type obj_state_t = obj_state_s;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct obj_state_s {
    pub parent: *mut obj_state_t,
    pub type_0: obj_type,
    pub u: C2RustUnnamed_3,
    pub emit_state: emit_state_t,
    pub pencolor: gvcolor_t,
    pub fillcolor: gvcolor_t,
    pub stopcolor: gvcolor_t,
    pub gradient_angle: libc::c_int,
    pub gradient_frac: libc::c_float,
    pub pen: pen_type,
    pub fill: fill_type,
    pub penwidth: libc::c_double,
    pub rawstyle: *mut *mut libc::c_char,
    pub z: libc::c_double,
    pub tail_z: libc::c_double,
    pub head_z: libc::c_double,
    pub label: *mut libc::c_char,
    pub xlabel: *mut libc::c_char,
    pub taillabel: *mut libc::c_char,
    pub headlabel: *mut libc::c_char,
    pub url: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub labelurl: *mut libc::c_char,
    pub tailurl: *mut libc::c_char,
    pub headurl: *mut libc::c_char,
    pub tooltip: *mut libc::c_char,
    pub labeltooltip: *mut libc::c_char,
    pub tailtooltip: *mut libc::c_char,
    pub headtooltip: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub labeltarget: *mut libc::c_char,
    pub tailtarget: *mut libc::c_char,
    pub headtarget: *mut libc::c_char,
    #[bitfield(name = "explicit_tooltip", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "explicit_tailtooltip", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "explicit_headtooltip", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "explicit_labeltooltip", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "explicit_tailtarget", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "explicit_headtarget", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "explicit_edgetarget", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "explicit_tailurl", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "explicit_headurl", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "labeledgealigned", ty = "libc::c_uint", bits = "9..=9")]
    pub explicit_tooltip_explicit_tailtooltip_explicit_headtooltip_explicit_labeltooltip_explicit_tailtarget_explicit_headtarget_explicit_edgetarget_explicit_tailurl_explicit_headurl_labeledgealigned: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub url_map_shape: map_shape_t,
    pub url_map_n: libc::c_int,
    pub url_map_p: *mut pointf,
    pub url_bsplinemap_poly_n: libc::c_int,
    pub url_bsplinemap_n: *mut libc::c_int,
    pub url_bsplinemap_p: *mut pointf,
    pub tailendurl_map_n: libc::c_int,
    pub tailendurl_map_p: *mut pointf,
    pub headendurl_map_n: libc::c_int,
    pub headendurl_map_p: *mut pointf,
}
pub type map_shape_t = libc::c_uint;
pub const MAP_POLYGON: map_shape_t = 2;
pub const MAP_CIRCLE: map_shape_t = 1;
pub const MAP_RECTANGLE: map_shape_t = 0;
pub type fill_type = libc::c_uint;
pub const FILL_RADIAL: fill_type = 3;
pub const FILL_LINEAR: fill_type = 2;
pub const FILL_SOLID: fill_type = 1;
pub const FILL_NONE: fill_type = 0;
pub type pen_type = libc::c_uint;
pub const PEN_SOLID: pen_type = 3;
pub const PEN_DOTTED: pen_type = 2;
pub const PEN_DASHED: pen_type = 1;
pub const PEN_NONE: pen_type = 0;
pub type gvcolor_t = color_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_s {
    pub u: C2RustUnnamed_2,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub RGBA: [libc::c_double; 4],
    pub HSVA: [libc::c_double; 4],
    pub rgba: [libc::c_uchar; 4],
    pub cmyk: [libc::c_uchar; 4],
    pub rrggbbaa: [libc::c_int; 4],
    pub string: *mut libc::c_char,
    pub index: libc::c_int,
}
pub type emit_state_t = libc::c_uint;
pub const EMIT_ELABEL: emit_state_t = 11;
pub const EMIT_NLABEL: emit_state_t = 10;
pub const EMIT_EDRAW: emit_state_t = 9;
pub const EMIT_NDRAW: emit_state_t = 8;
pub const EMIT_HLABEL: emit_state_t = 7;
pub const EMIT_TLABEL: emit_state_t = 6;
pub const EMIT_CLABEL: emit_state_t = 5;
pub const EMIT_GLABEL: emit_state_t = 4;
pub const EMIT_HDRAW: emit_state_t = 3;
pub const EMIT_TDRAW: emit_state_t = 2;
pub const EMIT_CDRAW: emit_state_t = 1;
pub const EMIT_GDRAW: emit_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub g: *mut graph_t,
    pub sg: *mut graph_t,
    pub n: *mut node_t,
    pub e: *mut edge_t,
}
pub type edge_t = Agedge_s;
pub type node_t = Agnode_s;
pub type graph_t = Agraph_s;
pub type obj_type = libc::c_uint;
pub const EDGE_OBJTYPE: obj_type = 3;
pub const NODE_OBJTYPE: obj_type = 2;
pub const CLUSTER_OBJTYPE: obj_type = 1;
pub const ROOTGRAPH_OBJTYPE: obj_type = 0;
pub type GVCOMMON_t = GVCOMMON_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVCOMMON_s {
    pub info: *mut *mut libc::c_char,
    pub cmdname: *mut libc::c_char,
    pub verbose: libc::c_int,
    pub config: bool,
    pub auto_outfile_names: bool,
    pub errorfn: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub show_boxes: *mut *const libc::c_char,
    pub lib: *mut *const libc::c_char,
    pub viewNum: libc::c_int,
    pub builtins: *const lt_symlist_t,
    pub demand_loading: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_symlist_t {
    pub name: *const libc::c_char,
    pub address: *mut libc::c_void,
}
pub type GVC_t = GVC_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVC_s {
    pub common: GVCOMMON_t,
    pub config_path: *mut libc::c_char,
    pub config_found: bool,
    pub input_filenames: *mut *mut libc::c_char,
    pub gvgs: *mut GVG_t,
    pub gvg: *mut GVG_t,
    pub apis: [*mut gvplugin_available_t; 5],
    pub api: [*mut gvplugin_available_t; 5],
    pub packages: *mut gvplugin_package_t,
    pub write_fn: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, size_t) -> size_t,
    >,
    pub textfont_disc: Dtdisc_t,
    pub textfont_dt: *mut Dt_t,
    pub textlayout: gvplugin_active_textlayout_t,
    pub jobs: *mut GVJ_t,
    pub job: *mut GVJ_t,
    pub g: *mut graph_t,
    pub layout: gvplugin_active_layout_t,
    pub graphname: *mut libc::c_char,
    pub active_jobs: *mut GVJ_t,
    pub pagedir: *mut libc::c_char,
    pub margin: pointf,
    pub pad: pointf,
    pub pageSize: pointf,
    pub pb: point,
    pub bb: boxf,
    pub rotation: libc::c_int,
    pub graph_sets_pad: bool,
    pub graph_sets_margin: bool,
    pub graph_sets_pageSize: bool,
    pub layerDelims: *mut libc::c_char,
    pub layerListDelims: *mut libc::c_char,
    pub layers: *mut libc::c_char,
    pub layerIDs: *mut *mut libc::c_char,
    pub numLayers: libc::c_int,
    pub layerlist: *mut libc::c_int,
    pub defaultfontname: *mut libc::c_char,
    pub defaultfontsize: libc::c_double,
    pub defaultlinestyle: *mut *mut libc::c_char,
    pub bgcolor: gvcolor_t,
    pub fontrenaming: libc::c_int,
}
pub type gvplugin_active_layout_t = gvplugin_active_layout_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_layout_s {
    pub engine: *mut gvlayout_engine_t,
    pub id: libc::c_int,
    pub features: *mut gvlayout_features_t,
    pub type_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvlayout_features_t {
    pub flags: libc::c_int,
}
pub type gvlayout_engine_t = gvlayout_engine_s;
pub type gvplugin_active_textlayout_t = gvplugin_active_textlayout_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_active_textlayout_s {
    pub engine: *mut gvtextlayout_engine_t,
    pub id: libc::c_int,
    pub type_0: *mut libc::c_char,
}
pub type gvtextlayout_engine_t = gvtextlayout_engine_s;
pub type gvplugin_package_t = gvplugin_package_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_package_s {
    pub next: *mut gvplugin_package_t,
    pub path: *mut libc::c_char,
    pub name: *mut libc::c_char,
}
pub type gvplugin_available_t = gvplugin_available_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_available_s {
    pub next: *mut gvplugin_available_t,
    pub typestr: *mut libc::c_char,
    pub quality: libc::c_int,
    pub package: *mut gvplugin_package_t,
    pub typeptr: *mut gvplugin_installed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type GVG_t = GVG_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVG_s {
    pub gvc: *mut GVC_t,
    pub next: *mut GVG_t,
    pub input_filename: *mut libc::c_char,
    pub graph_index: libc::c_int,
    pub g: *mut graph_t,
}
pub type Ppoint_t = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ppoly_t {
    pub ps: *mut Ppoint_t,
    pub pn: libc::c_int,
}
pub type Ppolyline_t = Ppoly_t;
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
    pub free_layout: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub yoffset_layout: libc::c_double,
    pub yoffset_centerline: libc::c_double,
    pub size: pointf,
    pub just: libc::c_char,
}
pub type qsort_cmpf = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_5,
    pub s: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub struct bezier {
    pub list: *mut pointf,
    pub size: libc::c_int,
    pub sflag: libc::c_int,
    pub eflag: libc::c_int,
    pub sp: pointf,
    pub ep: pointf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct splines {
    pub list: *mut bezier,
    pub size: libc::c_int,
    pub bb: boxf,
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
    pub u: C2RustUnnamed_6,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub txt: C2RustUnnamed_7,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub initfn: Option::<unsafe extern "C" fn(*mut node_t) -> ()>,
    pub freefn: Option::<unsafe extern "C" fn(*mut node_t) -> ()>,
    pub portfn: Option::<
        unsafe extern "C" fn(*mut node_t, *mut libc::c_char, *mut libc::c_char) -> port,
    >,
    pub insidefn: Option::<unsafe extern "C" fn(*mut inside_t, pointf) -> bool>,
    pub pboxfn: Option::<
        unsafe extern "C" fn(
            *mut node_t,
            *mut port,
            libc::c_int,
            *mut boxf,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub codefn: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> ()>,
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
pub struct nodequeue {
    pub store: *mut *mut node_t,
    pub limit: *mut *mut node_t,
    pub head: *mut *mut node_t,
    pub tail: *mut *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct adjmatrix_t {
    pub nrows: libc::c_int,
    pub ncols: libc::c_int,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rank_t {
    pub n: libc::c_int,
    pub v: *mut *mut node_t,
    pub an: libc::c_int,
    pub av: *mut *mut node_t,
    pub ht1: libc::c_double,
    pub ht2: libc::c_double,
    pub pht1: libc::c_double,
    pub pht2: libc::c_double,
    pub candidate: bool,
    pub valid: bool,
    pub cache_nc: libc::c_int,
    pub flat: *mut adjmatrix_t,
}
pub type ratio_t = libc::c_uint;
pub const R_EXPAND: ratio_t = 5;
pub const R_AUTO: ratio_t = 4;
pub const R_COMPRESS: ratio_t = 3;
pub const R_FILL: ratio_t = 2;
pub const R_VALUE: ratio_t = 1;
pub const R_NONE: ratio_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout_t {
    pub quantum: libc::c_double,
    pub scale: libc::c_double,
    pub ratio: libc::c_double,
    pub dpi: libc::c_double,
    pub margin: pointf,
    pub page: pointf,
    pub size: pointf,
    pub filled: bool,
    pub landscape: bool,
    pub centered: bool,
    pub ratio_kind: ratio_t,
    pub xdots: *mut libc::c_void,
    pub id: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlist_t {
    pub list: *mut *mut node_t,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elist {
    pub list: *mut *mut edge_t,
    pub size: libc::c_int,
}
pub type fontname_kind = libc::c_uint;
pub const SVGFONTS: fontname_kind = 2;
pub const PSFONTS: fontname_kind = 1;
pub const NATIVEFONTS: fontname_kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agraphinfo_t {
    pub hdr: Agrec_t,
    pub drawing: *mut layout_t,
    pub label: *mut textlabel_t,
    pub bb: boxf,
    pub border: [pointf; 4],
    pub gui_state: libc::c_uchar,
    pub has_labels: libc::c_uchar,
    pub has_images: bool,
    pub charset: libc::c_uchar,
    pub rankdir: libc::c_int,
    pub ht1: libc::c_double,
    pub ht2: libc::c_double,
    pub flags: libc::c_ushort,
    pub alg: *mut libc::c_void,
    pub gvc: *mut GVC_t,
    pub cleanup: Option::<unsafe extern "C" fn(*mut graph_t) -> ()>,
    pub neato_nlist: *mut *mut node_t,
    pub move_0: libc::c_int,
    pub dist: *mut *mut libc::c_double,
    pub spring: *mut *mut libc::c_double,
    pub sum_t: *mut *mut libc::c_double,
    pub t: *mut *mut *mut libc::c_double,
    pub ndim: libc::c_ushort,
    pub odim: libc::c_ushort,
    pub n_cluster: libc::c_int,
    pub clust: *mut *mut graph_t,
    pub dotroot: *mut graph_t,
    pub nlist: *mut node_t,
    pub rank: *mut rank_t,
    pub parent: *mut graph_t,
    pub level: libc::c_int,
    pub minrep: *mut node_t,
    pub maxrep: *mut node_t,
    pub comp: nlist_t,
    pub minset: *mut node_t,
    pub maxset: *mut node_t,
    pub n_nodes: libc::c_long,
    pub minrank: libc::c_int,
    pub maxrank: libc::c_int,
    pub has_flat_edges: bool,
    pub has_sourcerank: bool,
    pub has_sinkrank: bool,
    pub showboxes: libc::c_uchar,
    pub fontnames: fontname_kind,
    pub nodesep: libc::c_int,
    pub ranksep: libc::c_int,
    pub ln: *mut node_t,
    pub rn: *mut node_t,
    pub leader: *mut node_t,
    pub rankleader: *mut *mut node_t,
    pub expanded: bool,
    pub installed: libc::c_char,
    pub set_type: libc::c_char,
    pub label_pos: libc::c_char,
    pub exact_ranksep: bool,
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
pub struct Agedgeinfo_t {
    pub hdr: Agrec_t,
    pub spl: *mut splines,
    pub tail_port: port,
    pub head_port: port,
    pub label: *mut textlabel_t,
    pub head_label: *mut textlabel_t,
    pub tail_label: *mut textlabel_t,
    pub xlabel: *mut textlabel_t,
    pub edge_type: libc::c_char,
    pub compound: libc::c_char,
    pub adjacent: libc::c_char,
    pub label_ontop: libc::c_char,
    pub gui_state: libc::c_uchar,
    pub to_orig: *mut edge_t,
    pub alg: *mut libc::c_void,
    pub factor: libc::c_double,
    pub dist: libc::c_double,
    pub path: Ppolyline_t,
    pub showboxes: libc::c_uchar,
    pub conc_opp_flag: bool,
    pub xpenalty: libc::c_short,
    pub weight: libc::c_int,
    pub cutvalue: libc::c_int,
    pub tree_index: libc::c_int,
    pub count: libc::c_short,
    pub minlen: libc::c_ushort,
    pub to_virt: *mut edge_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct info_t {
    pub h: Agrec_t,
    pub x: libc::c_int,
    pub lo: libc::c_int,
    pub hi: libc::c_int,
    pub np: *mut Agnode_t,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut MinQuit: libc::c_int = 0;
static mut Convergence: libc::c_double = 0.;
static mut Root: *mut graph_t = 0 as *const graph_t as *mut graph_t;
static mut GlobalMinRank: libc::c_int = 0;
static mut GlobalMaxRank: libc::c_int = 0;
static mut TE_list: *mut *mut edge_t = 0 as *const *mut edge_t as *mut *mut edge_t;
static mut TI_list: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut ReMincross: bool = false;
unsafe extern "C" fn emptyComp(mut sg: *mut graph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut nxt: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfstnode(sg);
    while !n.is_null() {
        nxt = agnxtnode(sg, n);
        agdelnode(sg, n);
        n = nxt;
    }
}
unsafe extern "C" fn findSource(
    mut g: *mut Agraph_t,
    mut sg: *mut Agraph_t,
) -> *mut Agnode_t {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfstnode(sg);
    while !n.is_null() {
        if agdegree(g, n, 1 as libc::c_int, 0 as libc::c_int) == 0 as libc::c_int {
            return n;
        }
        n = agnxtnode(sg, n);
    }
    return 0 as *mut Agnode_t;
}
unsafe extern "C" fn topsort(
    mut g: *mut Agraph_t,
    mut sg: *mut Agraph_t,
    mut arr: *mut *mut Agnode_t,
) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut nxte: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    loop {
        n = findSource(g, sg);
        if n.is_null() {
            break;
        }
        let fresh0 = cnt;
        cnt = cnt + 1;
        let ref mut fresh1 = *arr.offset(fresh0 as isize);
        *fresh1 = (*((*(n as *mut Agobj_t)).data as *mut info_t)).np;
        agdelnode(sg, n);
        e = agfstout(g, n);
        while !e.is_null() {
            nxte = agnxtout(g, e);
            agdeledge(g, e);
            e = nxte;
        }
    }
    return cnt;
}
unsafe extern "C" fn getComp(
    mut g: *mut graph_t,
    mut n: *mut node_t,
    mut comp: *mut graph_t,
    mut indices: *mut libc::c_int,
) -> libc::c_int {
    let mut backedge: libc::c_int = 0 as libc::c_int;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    (*((*(n as *mut Agobj_t)).data as *mut info_t)).x = 1 as libc::c_int;
    *indices
        .offset(
            agnnodes(comp) as isize,
        ) = (*((*((*((*(n as *mut Agobj_t)).data as *mut info_t)).np as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .order;
    agsubnode(comp, n, 1 as libc::c_int);
    e = agfstout(g, n);
    while !e.is_null() {
        if (*((*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut info_t))
            .np as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .order
            > (*((*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut info_t))
                .np as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order
        {
            backedge += 1;
        }
        if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut info_t))
            .x == 0
        {
            backedge
                += getComp(
                    g,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    comp,
                    indices,
                );
        }
        e = agnxtout(g, e);
    }
    e = agfstin(g, n);
    while !e.is_null() {
        if (*((*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut info_t))
            .np as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .order
            > (*((*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut info_t))
                .np as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order
        {
            backedge += 1;
        }
        if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut info_t))
            .x == 0
        {
            backedge
                += getComp(
                    g,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node,
                    comp,
                    indices,
                );
        }
        e = agnxtin(g, e);
    }
    return backedge;
}
unsafe extern "C" fn fixLabelOrder(mut g: *mut graph_t, mut rk: *mut rank_t) {
    let mut cnt: libc::c_int = 0;
    let mut haveBackedge: bool = 0 as libc::c_int != 0;
    let mut arr: *mut *mut Agnode_t = 0 as *mut *mut Agnode_t;
    let mut indices: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut nxtp: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut v: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfstnode(g);
    while !n.is_null() {
        nxtp = agnxtnode(g, n);
        v = nxtp;
        while !v.is_null() {
            if (*((*(v as *mut Agobj_t)).data as *mut info_t)).hi
                <= (*((*(n as *mut Agobj_t)).data as *mut info_t)).lo
            {
                haveBackedge = 1 as libc::c_int != 0;
                agedge(g, v, n, 0 as *mut libc::c_char, 1 as libc::c_int);
            } else if (*((*(n as *mut Agobj_t)).data as *mut info_t)).hi
                    <= (*((*(v as *mut Agobj_t)).data as *mut info_t)).lo
                {
                agedge(g, n, v, 0 as *mut libc::c_char, 1 as libc::c_int);
            }
            v = agnxtnode(g, v);
        }
        n = nxtp;
    }
    if !haveBackedge {
        return;
    }
    sg = agsubg(
        g,
        b"comp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    arr = gcalloc(
        agnnodes(g) as size_t,
        ::std::mem::size_of::<*mut Agnode_t>() as libc::c_ulong,
    ) as *mut *mut Agnode_t;
    indices = gcalloc(
        agnnodes(g) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        if !((*((*(n as *mut Agobj_t)).data as *mut info_t)).x != 0
            || agdegree(g, n, 1 as libc::c_int, 1 as libc::c_int) == 0 as libc::c_int)
        {
            if getComp(g, n, sg, indices) != 0 {
                let mut i: libc::c_int = 0;
                let mut sz: libc::c_int = agnnodes(sg);
                cnt = topsort(g, sg, arr);
                if cnt == sz {} else {
                    __assert_fail(
                        b"cnt == sz\0" as *const u8 as *const libc::c_char,
                        b"mincross.c\0" as *const u8 as *const libc::c_char,
                        270 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 40],
                            &[libc::c_char; 40],
                        >(b"void fixLabelOrder(graph_t *, rank_t *)\0"))
                            .as_ptr(),
                    );
                }
                qsort(
                    indices as *mut libc::c_void,
                    cnt as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_int,
                                *mut libc::c_int,
                            ) -> libc::c_int,
                        >,
                        qsort_cmpf,
                    >(
                        Some(
                            ordercmpf
                                as unsafe extern "C" fn(
                                    *mut libc::c_int,
                                    *mut libc::c_int,
                                ) -> libc::c_int,
                        ),
                    ),
                );
                i = 0 as libc::c_int;
                while i < sz {
                    (*((*(*arr.offset(i as isize) as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .order = *indices.offset(i as isize);
                    let ref mut fresh2 = *((*rk).v)
                        .offset(*indices.offset(i as isize) as isize);
                    *fresh2 = *arr.offset(i as isize);
                    i += 1;
                }
            }
            emptyComp(sg);
        }
        n = agnxtnode(g, n);
    }
    free(arr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn checkLabelOrder(mut g: *mut graph_t) {
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut lg: *mut graph_t = 0 as *mut graph_t;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut rk: *mut rank_t = 0 as *mut rank_t;
    let mut u: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        rk = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize);
        j = 0 as libc::c_int;
        while j < (*rk).n {
            u = *((*rk).v).offset(j as isize);
            e = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut edge_t;
            if !e.is_null() {
                if lg.is_null() {
                    lg = agopen(
                        b"lg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        Agstrictdirected,
                        0 as *mut Agdisc_t,
                    );
                }
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    j,
                );
                n = agnode(lg, buf.as_mut_ptr(), 1 as libc::c_int);
                agbindrec(
                    n as *mut libc::c_void,
                    b"info\0" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<info_t>() as libc::c_ulong as libc::c_uint,
                    1 as libc::c_int,
                );
                lo = (*((*((*if ((*(*((*((*(u as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                    .tag)
                    .objtype() as libc::c_int == 2 as libc::c_int
                {
                    *((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                        .offset(0 as libc::c_int as isize)
                } else {
                    (*((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                        .offset(0 as libc::c_int as isize))
                        .offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order;
                hi = (*((*((*if ((*(*((*((*(u as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .offset(1 as libc::c_int as isize) as *mut Agobj_t))
                    .tag)
                    .objtype() as libc::c_int == 2 as libc::c_int
                {
                    *((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                        .offset(1 as libc::c_int as isize)
                } else {
                    (*((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                        .offset(1 as libc::c_int as isize))
                        .offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order;
                if lo > hi {
                    let mut tmp: libc::c_int = 0;
                    tmp = lo;
                    lo = hi;
                    hi = tmp;
                }
                (*((*(n as *mut Agobj_t)).data as *mut info_t)).lo = lo;
                (*((*(n as *mut Agobj_t)).data as *mut info_t)).hi = hi;
                let ref mut fresh3 = (*((*(n as *mut Agobj_t)).data as *mut info_t)).np;
                *fresh3 = u;
            }
            j += 1;
        }
        if !lg.is_null() {
            if agnnodes(lg) > 1 as libc::c_int {
                fixLabelOrder(lg, rk);
            }
            agclose(lg);
            lg = 0 as *mut graph_t;
        }
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dot_mincross(mut g: *mut graph_t, mut doBalance: libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    i = 1 as libc::c_int as size_t;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster as size_t
    {
        if (agfstnode(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(i as isize),
        ))
            .is_null()
        {
            agwarningf(
                b"removing empty cluster\n\0" as *const u8 as *const libc::c_char,
            );
            memmove(
                &mut *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                    .offset(i as isize) as *mut *mut graph_t as *mut libc::c_void,
                &mut *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as *mut *mut graph_t as *const libc::c_void,
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster
                    as size_t)
                    .wrapping_sub(i)
                    .wrapping_mul(::std::mem::size_of::<*mut graph_t>() as libc::c_ulong),
            );
            let ref mut fresh4 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .n_cluster;
            *fresh4 -= 1;
        } else {
            i = i.wrapping_add(1);
        }
    }
    init_mincross(g);
    c = 0 as libc::c_int;
    nc = c;
    while c < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size {
        init_mccomp(g, c);
        nc += mincross(g, 0 as libc::c_int, 2 as libc::c_int, doBalance);
        c += 1;
    }
    merge2(g);
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        nc
            += mincross_clust(
                *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                    .offset(c as isize),
                doBalance,
            );
        c += 1;
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster > 0 as libc::c_int
        && {
            s = agget(
                g as *mut libc::c_void,
                b"remincross\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            s.is_null() || mapbool(s) as libc::c_int != 0
        }
    {
        mark_lowclusters(g);
        ReMincross = 1 as libc::c_int != 0;
        nc = mincross(g, 2 as libc::c_int, 2 as libc::c_int, doBalance);
    }
    cleanup2(g, nc);
}
unsafe extern "C" fn new_matrix(
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> *mut adjmatrix_t {
    let mut rv: *mut adjmatrix_t = zmalloc(
        ::std::mem::size_of::<adjmatrix_t>() as libc::c_ulong,
    ) as *mut adjmatrix_t;
    (*rv).nrows = i;
    (*rv).ncols = j;
    let ref mut fresh5 = (*rv).data;
    *fresh5 = gcalloc(
        (i * j) as size_t,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    return rv;
}
unsafe extern "C" fn free_matrix(mut p: *mut adjmatrix_t) {
    if !p.is_null() {
        free((*p).data as *mut libc::c_void);
        free(p as *mut libc::c_void);
    }
}
unsafe extern "C" fn init_mccomp(mut g: *mut graph_t, mut c: libc::c_int) {
    let mut r: libc::c_int = 0;
    let ref mut fresh6 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    *fresh6 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.list)
        .offset(c as isize);
    if c > 0 as libc::c_int {
        r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
            let ref mut fresh7 = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .v;
            *fresh7 = ((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(
                    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .n as isize,
                );
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n = 0 as libc::c_int;
            r += 1;
        }
    }
}
unsafe extern "C" fn betweenclust(mut e: *mut edge_t) -> libc::c_int {
    while !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig).is_null() {
        e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    return ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .clust
        != (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .clust) as libc::c_int;
}
unsafe extern "C" fn do_ordering_node(
    mut g: *mut graph_t,
    mut n: *mut node_t,
    mut outflag: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ne: libc::c_int = 0;
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    let mut fe: *mut edge_t = 0 as *mut edge_t;
    let mut sortlist: *mut *mut edge_t = TE_list;
    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null() {
        return;
    }
    if outflag != 0 {
        ne = 0 as libc::c_int;
        i = ne;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            if betweenclust(e) == 0 {
                let fresh8 = ne;
                ne = ne + 1;
                let ref mut fresh9 = *sortlist.offset(fresh8 as isize);
                *fresh9 = e;
            }
            i += 1;
        }
    } else {
        ne = 0 as libc::c_int;
        i = ne;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            if betweenclust(e) == 0 {
                let fresh10 = ne;
                ne = ne + 1;
                let ref mut fresh11 = *sortlist.offset(fresh10 as isize);
                *fresh11 = e;
            }
            i += 1;
        }
    }
    if ne <= 1 as libc::c_int {
        return;
    }
    let ref mut fresh12 = *sortlist.offset(ne as isize);
    *fresh12 = 0 as *mut edge_t;
    qsort(
        sortlist as *mut libc::c_void,
        ne as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut *mut edge_t, *mut *mut edge_t) -> libc::c_int,
            >,
            qsort_cmpf,
        >(
            Some(
                edgeidcmpf
                    as unsafe extern "C" fn(
                        *mut *mut edge_t,
                        *mut *mut edge_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    ne = 1 as libc::c_int;
    loop {
        f = *sortlist.offset(ne as isize);
        if f.is_null() {
            break;
        }
        e = *sortlist.offset((ne - 1 as libc::c_int) as isize);
        if outflag != 0 {
            u = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
            v = (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                f
            } else {
                f.offset(-(1 as libc::c_int as isize))
            })
                .node;
        } else {
            u = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node;
            v = (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                f
            } else {
                f.offset(1 as libc::c_int as isize)
            })
                .node;
        }
        if !(find_flat_edge(u, v)).is_null() {
            return;
        }
        fe = new_virtual_edge(u, v, 0 as *mut Agedge_t);
        (*((*(fe as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .edge_type = 4 as libc::c_int as libc::c_char;
        flat_edge(g, fe);
        ne += 1;
    };
}
unsafe extern "C" fn do_ordering(mut g: *mut graph_t, mut outflag: libc::c_int) {
    let mut n: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    while !n.is_null() {
        do_ordering_node(g, n, outflag);
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn do_ordering_for_nodes(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut ordering: *const libc::c_char = 0 as *const libc::c_char;
    n = agfstnode(g);
    while !n.is_null() {
        ordering = late_string(
            n as *mut libc::c_void,
            N_ordering,
            0 as *mut libc::c_char,
        );
        if !ordering.is_null() {
            if strcmp(ordering, b"out\0" as *const u8 as *const libc::c_char) == 0 {
                do_ordering_node(g, n, (0 as libc::c_int == 0) as libc::c_int);
            } else if strcmp(ordering, b"in\0" as *const u8 as *const libc::c_char) == 0
                {
                do_ordering_node(g, n, 0 as libc::c_int);
            } else if *ordering.offset(0 as libc::c_int as isize) != 0 {
                agerr(
                    AGERR,
                    b"ordering '%s' not recognized for node '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    ordering,
                    agnameof(n as *mut libc::c_void),
                );
            }
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn ordered_edges(mut g: *mut graph_t) {
    let mut ordering: *mut libc::c_char = 0 as *mut libc::c_char;
    if G_ordering.is_null() && N_ordering.is_null() {
        return;
    }
    ordering = late_string(g as *mut libc::c_void, G_ordering, 0 as *mut libc::c_char);
    if !ordering.is_null() {
        if strcmp(ordering, b"out\0" as *const u8 as *const libc::c_char) == 0 {
            do_ordering(g, (0 as libc::c_int == 0) as libc::c_int);
        } else if strcmp(ordering, b"in\0" as *const u8 as *const libc::c_char) == 0 {
            do_ordering(g, 0 as libc::c_int);
        } else if *ordering.offset(0 as libc::c_int as isize) != 0 {
            agerr(
                AGERR,
                b"ordering '%s' not recognized.\n\0" as *const u8 as *const libc::c_char,
                ordering,
            );
        }
    } else {
        let mut subg: *mut graph_t = 0 as *mut graph_t;
        subg = agfstsubg(g);
        while !subg.is_null() {
            if !is_cluster(subg) {
                ordered_edges(subg);
            }
            subg = agnxtsubg(subg);
        }
        if !N_ordering.is_null() {
            do_ordering_for_nodes(g);
        }
    };
}
unsafe extern "C" fn mincross_clust(
    mut g: *mut graph_t,
    mut doBalance: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    expand_cluster(g);
    ordered_edges(g);
    flat_breakcycles(g);
    flat_reorder(g);
    nc = mincross(g, 2 as libc::c_int, 2 as libc::c_int, doBalance);
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        nc
            += mincross_clust(
                *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                    .offset(c as isize),
                doBalance,
            );
        c += 1;
    }
    save_vlist(g);
    return nc;
}
unsafe extern "C" fn left2right(
    mut g: *mut graph_t,
    mut v: *mut node_t,
    mut w: *mut node_t,
) -> libc::c_int {
    let mut M: *mut adjmatrix_t = 0 as *mut adjmatrix_t;
    let mut rv: libc::c_int = 0;
    if !ReMincross {
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
            != (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
            && !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null()
            && !((*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null()
        {
            if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ranktype
                as libc::c_int == 7 as libc::c_int
                && (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 1 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            if (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ranktype
                as libc::c_int == 7 as libc::c_int
                && (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 1 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            return (0 as libc::c_int == 0) as libc::c_int;
        }
    } else if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
            != (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
        {
        return (0 as libc::c_int == 0) as libc::c_int
    }
    M = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize))
        .flat;
    if M.is_null() {
        rv = 0 as libc::c_int;
    } else {
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int & 1 as libc::c_int != 0
        {
            let mut t: *mut node_t = v;
            v = w;
            w = t;
        }
        rv = *((*M).data)
            .offset(
                ((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low * (*M).ncols
                    + (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low) as isize,
            ) as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn in_cross(mut v: *mut node_t, mut w: *mut node_t) -> libc::c_int {
    let mut e1: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut e2: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut inv: libc::c_int = 0;
    let mut cross: libc::c_int = 0 as libc::c_int;
    let mut t: libc::c_int = 0;
    e2 = (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list;
    while !(*e2).is_null() {
        let mut cnt: libc::c_int = (*((*(*e2 as *mut Agobj_t)).data
            as *mut Agedgeinfo_t))
            .xpenalty as libc::c_int;
        inv = (*((*((*if ((*(*e2 as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            *e2
        } else {
            (*e2).offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .order;
        e1 = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list;
        while !(*e1).is_null() {
            t = (*((*((*(if ((*(*e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                *e1
            } else {
                (*e1).offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order - inv;
            if t > 0 as libc::c_int
                || t == 0 as libc::c_int
                    && (*((*(*e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .tail_port
                        .p
                        .x
                        > (*((*(*e2 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .tail_port
                            .p
                            .x
            {
                cross
                    += (*((*(*e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xpenalty
                        as libc::c_int * cnt;
            }
            e1 = e1.offset(1);
        }
        e2 = e2.offset(1);
    }
    return cross;
}
unsafe extern "C" fn out_cross(mut v: *mut node_t, mut w: *mut node_t) -> libc::c_int {
    let mut e1: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut e2: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut inv: libc::c_int = 0;
    let mut cross: libc::c_int = 0 as libc::c_int;
    let mut t: libc::c_int = 0;
    e2 = (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list;
    while !(*e2).is_null() {
        let mut cnt: libc::c_int = (*((*(*e2 as *mut Agobj_t)).data
            as *mut Agedgeinfo_t))
            .xpenalty as libc::c_int;
        inv = (*((*((*if ((*(*e2 as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            *e2
        } else {
            (*e2).offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .order;
        e1 = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list;
        while !(*e1).is_null() {
            t = (*((*((*(if ((*(*e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                *e1
            } else {
                (*e1).offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order - inv;
            if t > 0 as libc::c_int
                || t == 0 as libc::c_int
                    && (*((*(*e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .head_port
                        .p
                        .x
                        > (*((*(*e2 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .head_port
                            .p
                            .x
            {
                cross
                    += (*((*(*e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xpenalty
                        as libc::c_int * cnt;
            }
            e1 = e1.offset(1);
        }
        e2 = e2.offset(1);
    }
    return cross;
}
unsafe extern "C" fn exchange(mut v: *mut node_t, mut w: *mut node_t) {
    let mut vi: libc::c_int = 0;
    let mut wi: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    r = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
    vi = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order;
    wi = (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order = wi;
    let ref mut fresh13 = *((*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rank)
        .offset(r as isize))
        .v)
        .offset(wi as isize);
    *fresh13 = v;
    (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order = vi;
    let ref mut fresh14 = *((*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rank)
        .offset(r as isize))
        .v)
        .offset(vi as isize);
    *fresh14 = w;
}
unsafe extern "C" fn balanceNodes(
    mut g: *mut graph_t,
    mut r: libc::c_int,
    mut v: *mut node_t,
    mut w: *mut node_t,
) {
    let mut s: *mut node_t = 0 as *mut node_t;
    let mut sepIndex: libc::c_int = 0 as libc::c_int;
    let mut nullType: libc::c_int = 0;
    let mut cntDummy: libc::c_int = 0 as libc::c_int;
    let mut cntOri: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut k1: libc::c_int = 0 as libc::c_int;
    let mut m1: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
    {
        return;
    }
    i = 0 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n
    {
        if (*((*(*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .node_type as libc::c_int == 0 as libc::c_int
        {
            cntOri += 1;
        } else {
            cntDummy += 1;
        }
        i += 1;
    }
    if cntOri < cntDummy {
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 0 as libc::c_int
        {
            s = v;
        } else {
            s = w;
        }
    } else if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
            as libc::c_int == 0 as libc::c_int
        {
        s = w;
    } else {
        s = v;
    }
    i = 0 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n
    {
        if *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize) == s
        {
            sepIndex = i;
        }
        i += 1;
    }
    nullType = if (*((*(s as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
        as libc::c_int == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    i = sepIndex - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !((*((*(*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .node_type as libc::c_int == nullType)
        {
            break;
        }
        k += 1;
        i -= 1;
    }
    i = sepIndex + 1 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n
    {
        if !((*((*(*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .node_type as libc::c_int == nullType)
        {
            break;
        }
        m += 1;
        i += 1;
    }
    exchange(v, w);
    i = 0 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n
    {
        if *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize) == s
        {
            sepIndex = i;
        }
        i += 1;
    }
    i = sepIndex - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if !((*((*(*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .node_type as libc::c_int == nullType)
        {
            break;
        }
        k1 += 1;
        i -= 1;
    }
    i = sepIndex + 1 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n
    {
        if !((*((*(*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .node_type as libc::c_int == nullType)
        {
            break;
        }
        m1 += 1;
        i += 1;
    }
    if abs(k1 - m1) > abs(k - m) {
        exchange(v, w);
    }
}
unsafe extern "C" fn balance(mut g: *mut graph_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut w: *mut node_t = 0 as *mut node_t;
    let mut r: libc::c_int = 0;
    rv = 0 as libc::c_int;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank;
    while r >= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank {
        (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .candidate = 0 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i
            < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n - 1 as libc::c_int
        {
            v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(i as isize);
            w = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset((i + 1 as libc::c_int) as isize);
            if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                < (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            {} else {
                __assert_fail(
                    b"ND_order(v) < ND_order(w)\0" as *const u8 as *const libc::c_char,
                    b"mincross.c\0" as *const u8 as *const libc::c_char,
                    735 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int balance(graph_t *)\0"))
                        .as_ptr(),
                );
            }
            if !(left2right(g, v, w) != 0) {
                c1 = 0 as libc::c_int;
                c0 = c1;
                if r > 0 as libc::c_int {
                    c0 += in_cross(v, w);
                    c1 += in_cross(w, v);
                }
                if (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset((r + 1 as libc::c_int) as isize))
                    .n > 0 as libc::c_int
                {
                    c0 += out_cross(v, w);
                    c1 += out_cross(w, v);
                }
                if c1 <= c0 {
                    balanceNodes(g, r, v, w);
                }
            }
            i += 1;
        }
        r -= 1;
    }
    return rv;
}
unsafe extern "C" fn transpose_step(
    mut g: *mut graph_t,
    mut r: libc::c_int,
    mut reverse: bool,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut w: *mut node_t = 0 as *mut node_t;
    rv = 0 as libc::c_int;
    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank).offset(r as isize))
        .candidate = 0 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n - 1 as libc::c_int
    {
        v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(i as isize);
        w = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset((i + 1 as libc::c_int) as isize);
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            < (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
        {} else {
            __assert_fail(
                b"ND_order(v) < ND_order(w)\0" as *const u8 as *const libc::c_char,
                b"mincross.c\0" as *const u8 as *const libc::c_char,
                767 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"int transpose_step(graph_t *, int, _Bool)\0"))
                    .as_ptr(),
            );
        }
        if !(left2right(g, v, w) != 0) {
            c1 = 0 as libc::c_int;
            c0 = c1;
            if r > 0 as libc::c_int {
                c0 += in_cross(v, w);
                c1 += in_cross(w, v);
            }
            if (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset((r + 1 as libc::c_int) as isize))
                .n > 0 as libc::c_int
            {
                c0 += out_cross(v, w);
                c1 += out_cross(w, v);
            }
            if c1 < c0
                || c0 > 0 as libc::c_int && reverse as libc::c_int != 0 && c1 == c0
            {
                exchange(v, w);
                rv += c0 - c1;
                (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .valid = 0 as libc::c_int != 0;
                (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .candidate = 1 as libc::c_int != 0;
                if r > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank {
                    (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset((r - 1 as libc::c_int) as isize))
                        .valid = 0 as libc::c_int != 0;
                    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset((r - 1 as libc::c_int) as isize))
                        .candidate = 1 as libc::c_int != 0;
                }
                if r < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
                    (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset((r + 1 as libc::c_int) as isize))
                        .valid = 0 as libc::c_int != 0;
                    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset((r + 1 as libc::c_int) as isize))
                        .candidate = 1 as libc::c_int != 0;
                }
            }
        }
        i += 1;
    }
    return rv;
}
unsafe extern "C" fn transpose(mut g: *mut graph_t, mut reverse: bool) {
    let mut r: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .candidate = 1 as libc::c_int != 0;
        r += 1;
    }
    loop {
        delta = 0 as libc::c_int;
        r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
            if (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .candidate
            {
                delta += transpose_step(g, r, reverse);
            }
            r += 1;
        }
        if !(delta >= 1 as libc::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn mincross(
    mut g: *mut graph_t,
    mut startpass: libc::c_int,
    mut endpass: libc::c_int,
    mut doBalance: libc::c_int,
) -> libc::c_int {
    let mut maxthispass: libc::c_int = 0 as libc::c_int;
    let mut iter: libc::c_int = 0;
    let mut trying: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut cur_cross: libc::c_int = 0;
    let mut best_cross: libc::c_int = 0;
    if startpass > 1 as libc::c_int {
        best_cross = ncross(g);
        cur_cross = best_cross;
        save_best(g);
    } else {
        best_cross = 2147483647 as libc::c_int;
        cur_cross = best_cross;
    }
    pass = startpass;
    while pass <= endpass {
        if pass <= 1 as libc::c_int {
            maxthispass = if (4 as libc::c_int) < MaxIter {
                4 as libc::c_int
            } else {
                MaxIter
            };
            if g == dot_root(g as *mut libc::c_void) {
                build_ranks(g, pass);
            }
            if pass == 0 as libc::c_int {
                flat_breakcycles(g);
            }
            flat_reorder(g);
            cur_cross = ncross(g);
            if cur_cross <= best_cross {
                save_best(g);
                best_cross = cur_cross;
            }
        } else {
            maxthispass = MaxIter;
            if cur_cross > best_cross {
                restore_best(g);
            }
            cur_cross = best_cross;
        }
        trying = 0 as libc::c_int;
        iter = 0 as libc::c_int;
        while iter < maxthispass {
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"mincross: pass %d iter %d trying %d cur_cross %d best_cross %d\n\0"
                        as *const u8 as *const libc::c_char,
                    pass,
                    iter,
                    trying,
                    cur_cross,
                    best_cross,
                );
            }
            let fresh15 = trying;
            trying = trying + 1;
            if fresh15 >= MinQuit {
                break;
            }
            if cur_cross == 0 as libc::c_int {
                break;
            }
            mincross_step(g, iter);
            cur_cross = ncross(g);
            if cur_cross <= best_cross {
                save_best(g);
                if (cur_cross as libc::c_double)
                    < Convergence * best_cross as libc::c_double
                {
                    trying = 0 as libc::c_int;
                }
                best_cross = cur_cross;
            }
            iter += 1;
        }
        if cur_cross == 0 as libc::c_int {
            break;
        }
        pass += 1;
    }
    if cur_cross > best_cross {
        restore_best(g);
    }
    if best_cross > 0 as libc::c_int {
        transpose(g, 0 as libc::c_int != 0);
        best_cross = ncross(g);
    }
    if doBalance != 0 {
        iter = 0 as libc::c_int;
        while iter < maxthispass {
            balance(g);
            iter += 1;
        }
    }
    return best_cross;
}
unsafe extern "C" fn restore_best(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        i = 0 as libc::c_int;
        while i
            < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n
        {
            n = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(i as isize);
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .order = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                as libc::c_int;
            i += 1;
        }
        r += 1;
    }
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .valid = 0 as libc::c_int != 0;
        qsort(
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v as *mut libc::c_void,
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n as size_t,
            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut *mut node_t,
                        *mut *mut node_t,
                    ) -> libc::c_int,
                >,
                qsort_cmpf,
            >(
                Some(
                    nodeposcmpf
                        as unsafe extern "C" fn(
                            *mut *mut node_t,
                            *mut *mut node_t,
                        ) -> libc::c_int,
                ),
            ),
        );
        r += 1;
    }
}
unsafe extern "C" fn save_best(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        i = 0 as libc::c_int;
        while i
            < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n
        {
            n = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(i as isize);
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                as libc::c_double;
            i += 1;
        }
        r += 1;
    }
}
unsafe extern "C" fn merge_components(mut g: *mut graph_t) {
    let mut c: libc::c_int = 0;
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size
        <= 1 as libc::c_int
    {
        return;
    }
    u = 0 as *mut node_t;
    c = 0 as libc::c_int;
    while c < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size {
        v = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.list)
            .offset(c as isize);
        if !u.is_null() {
            let ref mut fresh16 = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .next;
            *fresh16 = v;
        }
        let ref mut fresh17 = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).prev;
        *fresh17 = u;
        while !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next).is_null() {
            v = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
        }
        u = v;
        c += 1;
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size = 1 as libc::c_int;
    let ref mut fresh18 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    *fresh18 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.list)
        .offset(0 as libc::c_int as isize);
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank = GlobalMinRank;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank = GlobalMaxRank;
}
unsafe extern "C" fn merge2(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    merge_components(g);
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .an;
        let ref mut fresh19 = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rank)
            .offset(r as isize))
            .v;
        *fresh19 = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .av;
        i = 0 as libc::c_int;
        while i
            < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n
        {
            v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(i as isize);
            if v.is_null() {
                if Verbose != 0 {
                    fprintf(
                        stderr,
                        b"merge2: graph %s, rank %d has only %d < %d nodes\n\0"
                            as *const u8 as *const libc::c_char,
                        agnameof(g as *mut libc::c_void),
                        r,
                        i,
                        (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                            .offset(r as isize))
                            .n,
                    );
                }
                (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .n = i;
                break;
            } else {
                (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order = i;
                i += 1;
            }
        }
        r += 1;
    }
}
unsafe extern "C" fn cleanup2(mut g: *mut graph_t, mut nc: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if !TI_list.is_null() {
        free(TI_list as *mut libc::c_void);
        TI_list = 0 as *mut libc::c_int;
    }
    if !TE_list.is_null() {
        free(TE_list as *mut libc::c_void);
        TE_list = 0 as *mut *mut edge_t;
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        rec_reset_vlists(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
        );
        c += 1;
    }
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        i = 0 as libc::c_int;
        while i
            < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n
        {
            v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(i as isize);
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order = i;
            if !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.list)
                .is_null()
            {
                j = 0 as libc::c_int;
                loop {
                    e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .flat_out
                        .list)
                        .offset(j as isize);
                    if e.is_null() {
                        break;
                    }
                    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
                        as libc::c_int == 4 as libc::c_int
                    {
                        delete_flat_edge(e);
                        free((*e).base.data as *mut libc::c_void);
                        free(e as *mut libc::c_void);
                        j -= 1;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        free_matrix(
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .flat,
        );
        r += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"mincross %s: %d crossings, %.2f secs.\n\0" as *const u8
                as *const libc::c_char,
            agnameof(g as *mut libc::c_void),
            nc,
            elapsed_sec(),
        );
    }
}
unsafe extern "C" fn neighbor(mut v: *mut node_t, mut dir: libc::c_int) -> *mut node_t {
    let mut rv: *mut node_t = 0 as *mut node_t;
    rv = 0 as *mut node_t;
    if !v.is_null() {} else {
        __assert_fail(
            b"v\0" as *const u8 as *const libc::c_char,
            b"mincross.c\0" as *const u8 as *const libc::c_char,
            1006 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"node_t *neighbor(node_t *, int)\0"))
                .as_ptr(),
        );
    }
    if dir < 0 as libc::c_int {
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order > 0 as libc::c_int
        {
            rv = *((*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(
                    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
                ))
                .v)
                .offset(
                    ((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                        - 1 as libc::c_int) as isize,
                );
        }
    } else {
        rv = *((*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize))
            .v)
            .offset(
                ((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                    + 1 as libc::c_int) as isize,
            );
    }
    if rv.is_null()
        || ((*((*(rv as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            - (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order) * dir
            > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"(rv == 0) || (ND_order(rv)-ND_order(v))*dir > 0\0" as *const u8
                as *const libc::c_char,
            b"mincross.c\0" as *const u8 as *const libc::c_char,
            1012 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"node_t *neighbor(node_t *, int)\0"))
                .as_ptr(),
        );
    }
    return rv;
}
unsafe extern "C" fn is_a_normal_node_of(
    mut g: *mut graph_t,
    mut v: *mut node_t,
) -> libc::c_int {
    return ((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
        as libc::c_int == 0 as libc::c_int && agcontains(g, v as *mut libc::c_void) != 0)
        as libc::c_int;
}
unsafe extern "C" fn is_a_vnode_of_an_edge_of(
    mut g: *mut graph_t,
    mut v: *mut node_t,
) -> libc::c_int {
    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 1 as libc::c_int
        && (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
            == 1 as libc::c_int
        && (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
            == 1 as libc::c_int
    {
        let mut e: *mut edge_t = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list)
            .offset(0 as libc::c_int as isize);
        while (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
            as libc::c_int != 0 as libc::c_int
        {
            e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
        }
        if agcontains(g, e as *mut libc::c_void) != 0 {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn inside_cluster(
    mut g: *mut graph_t,
    mut v: *mut node_t,
) -> libc::c_int {
    return is_a_normal_node_of(g, v) | is_a_vnode_of_an_edge_of(g, v);
}
unsafe extern "C" fn furthestnode(
    mut g: *mut graph_t,
    mut v: *mut node_t,
    mut dir: libc::c_int,
) -> *mut node_t {
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut rv: *mut node_t = 0 as *mut node_t;
    u = v;
    rv = u;
    loop {
        u = neighbor(u, dir);
        if u.is_null() {
            break;
        }
        if is_a_normal_node_of(g, u) != 0 {
            rv = u;
        } else if is_a_vnode_of_an_edge_of(g, u) != 0 {
            rv = u;
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn save_vlist(mut g: *mut graph_t) {
    let mut r: libc::c_int = 0;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankleader).is_null() {
        r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
            let ref mut fresh20 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .rankleader)
                .offset(r as isize);
            *fresh20 = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(0 as libc::c_int as isize);
            r += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn rec_save_vlists(mut g: *mut graph_t) {
    let mut c: libc::c_int = 0;
    save_vlist(g);
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        rec_save_vlists(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
        );
        c += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rec_reset_vlists(mut g: *mut graph_t) {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut w: *mut node_t = 0 as *mut node_t;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        rec_reset_vlists(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
        );
        c += 1;
    }
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankleader).is_null() {
        r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
            v = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankleader)
                .offset(r as isize);
            u = furthestnode(g, v, -(1 as libc::c_int));
            w = furthestnode(g, v, 1 as libc::c_int);
            let ref mut fresh21 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .rankleader)
                .offset(r as isize);
            *fresh21 = u;
            let ref mut fresh22 = (*((*((*(g as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .v;
            *fresh22 = ((*((*((*(dot_root(g as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .v)
                .offset(
                    (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order as isize,
                );
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n = (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                - (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                + 1 as libc::c_int;
            r += 1;
        }
    }
}
unsafe extern "C" fn realFillRanks(
    mut g: *mut Agraph_t,
    mut rnks: *mut libc::c_int,
    mut rnks_sz: libc::c_int,
    mut sg: *mut Agraph_t,
) -> *mut Agraph_t {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        sg = realFillRanks(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
            rnks,
            rnks_sz,
            sg,
        );
        c += 1;
    }
    if dot_root(g as *mut libc::c_void) == g {
        return sg;
    }
    memset(
        rnks as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(rnks_sz as libc::c_ulong),
    );
    n = agfstnode(g);
    while !n.is_null() {
        *rnks
            .offset(
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
            ) = 1 as libc::c_int;
        e = agfstout(g, n);
        while !e.is_null() {
            i = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                + 1 as libc::c_int;
            while i
                <= (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rank
            {
                *rnks.offset(i as isize) = 1 as libc::c_int;
                i += 1;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    i = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        if *rnks.offset(i as isize) == 0 as libc::c_int {
            if sg.is_null() {
                sg = agsubg(
                    dot_root(g as *mut libc::c_void),
                    b"_new_rank\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    1 as libc::c_int,
                );
            }
            n = agnode(sg, 0 as *mut libc::c_char, 1 as libc::c_int);
            agbindrec(
                n as *mut libc::c_void,
                b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank = i;
            let ref mut fresh23 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .rw;
            *fresh23 = 0.5f64;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw = *fresh23;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .ht = 1 as libc::c_int as libc::c_double;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .UF_size = 1 as libc::c_int;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .size = 0 as libc::c_int;
            let ref mut fresh24 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .list;
            *fresh24 = gcalloc(
                (4 as libc::c_int + 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
            ) as *mut *mut edge_t;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .size = 0 as libc::c_int;
            let ref mut fresh25 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list;
            *fresh25 = gcalloc(
                (4 as libc::c_int + 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
            ) as *mut *mut edge_t;
            agsubnode(g, n, 1 as libc::c_int);
        }
        i += 1;
    }
    return sg;
}
unsafe extern "C" fn fillRanks(mut g: *mut Agraph_t) {
    let mut rnks_sz: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .maxrank + 2 as libc::c_int;
    let mut rnks: *mut libc::c_int = gcalloc(
        rnks_sz as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    realFillRanks(g, rnks, rnks_sz, 0 as *mut Agraph_t);
    free(rnks as *mut libc::c_void);
}
unsafe extern "C" fn init_mincross(mut g: *mut graph_t) {
    let mut size: libc::c_int = 0;
    if Verbose != 0 {
        start_timer();
    }
    ReMincross = 0 as libc::c_int != 0;
    Root = g;
    size = agnedges(dot_root(g as *mut libc::c_void)) + 1 as libc::c_int;
    TE_list = gcalloc(
        size as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
    ) as *mut *mut edge_t;
    TI_list = gcalloc(
        size as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    mincross_options(g);
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags as libc::c_int
        & (1 as libc::c_int) << 4 as libc::c_int != 0
    {
        fillRanks(g);
    }
    class2(g);
    decompose(g, 1 as libc::c_int);
    allocate_ranks(g);
    ordered_edges(g);
    GlobalMinRank = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    GlobalMaxRank = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank;
}
unsafe extern "C" fn flat_rev(mut g: *mut Agraph_t, mut e: *mut Agedge_t) {
    let mut j: libc::c_int = 0;
    let mut rev: *mut Agedge_t = 0 as *mut Agedge_t;
    if ((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .flat_out
        .list)
        .is_null()
    {
        rev = 0 as *mut Agedge_t;
    } else {
        j = 0 as libc::c_int;
        loop {
            rev = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .flat_out
                .list)
                .offset(j as isize);
            if rev.is_null() {
                break;
            }
            if (*(if ((*(rev as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                rev
            } else {
                rev.offset(-(1 as libc::c_int as isize))
            }))
                .node
                == (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node
            {
                break;
            }
            j += 1;
        }
    }
    if !rev.is_null() {
        merge_oneway(e, rev);
        if (*((*(rev as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
            as libc::c_int == 4 as libc::c_int
            && ((*((*(rev as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig)
                .is_null()
        {
            let ref mut fresh26 = (*((*(rev as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .to_orig;
            *fresh26 = e;
        }
        let ref mut fresh27 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .other
            .list;
        *fresh27 = if !((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .other
            .list)
            .is_null()
        {
            grealloc(
                (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .other
                    .list as *mut libc::c_void,
                (((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .other
                    .size + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
            ) as *mut *mut edge_t
        } else {
            gmalloc(
                (((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .other
                    .size + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
            ) as *mut *mut edge_t
        };
        let ref mut fresh28 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .other
            .size;
        let fresh29 = *fresh28;
        *fresh28 = *fresh28 + 1;
        let ref mut fresh30 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .other
            .list)
            .offset(fresh29 as isize);
        *fresh30 = e;
        let ref mut fresh31 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .other
            .list)
            .offset(
                (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .other
                    .size as isize,
            );
        *fresh31 = 0 as *mut edge_t;
    } else {
        rev = new_virtual_edge(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node,
            e,
        );
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type as libc::c_int
            == 4 as libc::c_int
        {
            (*((*(rev as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .edge_type = 4 as libc::c_int as libc::c_char;
        } else {
            (*((*(rev as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .edge_type = 3 as libc::c_int as libc::c_char;
        }
        let ref mut fresh32 = (*((*(rev as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .label;
        *fresh32 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
        flat_edge(g, rev);
    };
}
unsafe extern "C" fn flat_search(mut g: *mut graph_t, mut v: *mut node_t) {
    let mut i: libc::c_int = 0;
    let mut hascl: bool = false;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut M: *mut adjmatrix_t = (*((*((*(g as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .rank)
        .offset((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize))
        .flat;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .onstack = 1 as libc::c_int as libc::c_char;
    hascl = (*((*(dot_root(g as *mut libc::c_void) as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .n_cluster > 0 as libc::c_int;
    if !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.list).is_null() {
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            if !(hascl as libc::c_int != 0
                && !(agcontains(
                    g,
                    (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut libc::c_void,
                ) != 0
                    && agcontains(
                        g,
                        (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut libc::c_void,
                    ) != 0))
            {
                if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
                    == 0 as libc::c_int)
                {
                    if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .onstack != 0
                    {
                        if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .low < (*M).nrows
                        {} else {
                            __assert_fail(
                                b"flatindex(aghead(e)) < M->nrows\0" as *const u8
                                    as *const libc::c_char,
                                b"mincross.c\0" as *const u8 as *const libc::c_char,
                                1227 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 38],
                                    &[libc::c_char; 38],
                                >(b"void flat_search(graph_t *, node_t *)\0"))
                                    .as_ptr(),
                            );
                        }
                        if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .low < (*M).ncols
                        {} else {
                            __assert_fail(
                                b"flatindex(agtail(e)) < M->ncols\0" as *const u8
                                    as *const libc::c_char,
                                b"mincross.c\0" as *const u8 as *const libc::c_char,
                                1228 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 38],
                                    &[libc::c_char; 38],
                                >(b"void flat_search(graph_t *, node_t *)\0"))
                                    .as_ptr(),
                            );
                        }
                        *((*M).data)
                            .offset(
                                ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                    as libc::c_int == 2 as libc::c_int
                                {
                                    e
                                } else {
                                    e.offset(-(1 as libc::c_int as isize))
                                }))
                                    .node as *mut Agobj_t))
                                    .data as *mut Agnodeinfo_t))
                                    .low * (*M).ncols
                                    + (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                        as libc::c_int == 3 as libc::c_int
                                    {
                                        e
                                    } else {
                                        e.offset(1 as libc::c_int as isize)
                                    }))
                                        .node as *mut Agobj_t))
                                        .data as *mut Agnodeinfo_t))
                                        .low) as isize,
                            ) = 1 as libc::c_int as libc::c_char;
                        delete_flat_edge(e);
                        i -= 1;
                        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .edge_type as libc::c_int == 4 as libc::c_int)
                        {
                            flat_rev(g, e);
                        }
                    } else {
                        if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .low < (*M).nrows
                        {} else {
                            __assert_fail(
                                b"flatindex(aghead(e)) < M->nrows\0" as *const u8
                                    as *const libc::c_char,
                                b"mincross.c\0" as *const u8 as *const libc::c_char,
                                1236 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 38],
                                    &[libc::c_char; 38],
                                >(b"void flat_search(graph_t *, node_t *)\0"))
                                    .as_ptr(),
                            );
                        }
                        if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .low < (*M).ncols
                        {} else {
                            __assert_fail(
                                b"flatindex(agtail(e)) < M->ncols\0" as *const u8
                                    as *const libc::c_char,
                                b"mincross.c\0" as *const u8 as *const libc::c_char,
                                1237 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 38],
                                    &[libc::c_char; 38],
                                >(b"void flat_search(graph_t *, node_t *)\0"))
                                    .as_ptr(),
                            );
                        }
                        *((*M).data)
                            .offset(
                                ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                    as libc::c_int == 3 as libc::c_int
                                {
                                    e
                                } else {
                                    e.offset(1 as libc::c_int as isize)
                                }))
                                    .node as *mut Agobj_t))
                                    .data as *mut Agnodeinfo_t))
                                    .low * (*M).ncols
                                    + (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                        as libc::c_int == 2 as libc::c_int
                                    {
                                        e
                                    } else {
                                        e.offset(-(1 as libc::c_int as isize))
                                    }))
                                        .node as *mut Agobj_t))
                                        .data as *mut Agnodeinfo_t))
                                        .low) as isize,
                            ) = 1 as libc::c_int as libc::c_char;
                        if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        })
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .mark == 0
                        {
                            flat_search(
                                g,
                                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                    == 2 as libc::c_int
                                {
                                    e
                                } else {
                                    e.offset(-(1 as libc::c_int as isize))
                                })
                                    .node,
                            );
                        }
                    }
                }
            }
            i += 1;
        }
    }
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .onstack = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn flat_breakcycles(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut flat: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        flat = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i
            < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n
        {
            v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(i as isize);
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .mark = 0 as libc::c_int as size_t;
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .onstack = 0 as libc::c_int as libc::c_char;
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low = i;
            if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.size
                > 0 as libc::c_int && flat == 0 as libc::c_int
            {
                let ref mut fresh33 = (*((*((*(g as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .rank)
                    .offset(r as isize))
                    .flat;
                *fresh33 = new_matrix(
                    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .n,
                    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .n,
                );
                flat = 1 as libc::c_int;
            }
            i += 1;
        }
        if flat != 0 {
            i = 0 as libc::c_int;
            while i
                < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .n
            {
                v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .v)
                    .offset(i as isize);
                if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark == 0 {
                    flat_search(g, v);
                }
                i += 1;
            }
        }
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn allocate_ranks(mut g: *mut graph_t) {
    let mut r: libc::c_int = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut cn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    cn = gcalloc(
        ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            + 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        let ref mut fresh34 = *cn
            .offset((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize);
        *fresh34 += 1;
        e = agfstout(g, n);
        while !e.is_null() {
            low = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .rank;
            high = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .rank;
            if low > high {
                let mut t: libc::c_int = low;
                low = high;
                high = t;
            }
            r = low + 1 as libc::c_int;
            while r < high {
                let ref mut fresh35 = *cn.offset(r as isize);
                *fresh35 += 1;
                r += 1;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    let ref mut fresh36 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank;
    *fresh36 = gcalloc(
        ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            + 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<rank_t>() as libc::c_ulong,
    ) as *mut rank_t;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        let ref mut fresh37 = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rank)
            .offset(r as isize))
            .n;
        *fresh37 = *cn.offset(r as isize);
        (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .an = *fresh37;
        let ref mut fresh38 = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rank)
            .offset(r as isize))
            .v;
        *fresh38 = gcalloc(
            (*cn.offset(r as isize) + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
        ) as *mut *mut node_t;
        let ref mut fresh39 = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rank)
            .offset(r as isize))
            .av;
        *fresh39 = *fresh38;
        r += 1;
    }
    free(cn as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn install_in_rank(mut g: *mut graph_t, mut n: *mut node_t) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    r = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
    i = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset(r as isize))
        .n;
    if (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank).offset(r as isize))
        .an <= 0 as libc::c_int
    {
        agerr(
            AGERR,
            b"install_in_rank, line %d: %s %s rank %d i = %d an = 0\n\0" as *const u8
                as *const libc::c_char,
            1316 as libc::c_int,
            agnameof(g as *mut libc::c_void),
            agnameof(n as *mut libc::c_void),
            r,
            i,
        );
        return;
    }
    let ref mut fresh40 = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rank)
        .offset(r as isize))
        .v)
        .offset(i as isize);
    *fresh40 = n;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order = i;
    let ref mut fresh41 = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset(r as isize))
        .n;
    *fresh41 += 1;
    if (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank).offset(r as isize))
        .n
        <= (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .an
    {} else {
        __assert_fail(
            b"GD_rank(g)[r].n <= GD_rank(g)[r].an\0" as *const u8 as *const libc::c_char,
            b"mincross.c\0" as *const u8 as *const libc::c_char,
            1323 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void install_in_rank(graph_t *, node_t *)\0"))
                .as_ptr(),
        );
    }
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
        > (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .an
    {
        agerr(
            AGERR,
            b"install_in_rank, line %d: ND_order(%s) [%d] > GD_rank(Root)[%d].an [%d]\n\0"
                as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
            agnameof(n as *mut libc::c_void),
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order,
            r,
            (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .an,
        );
        return;
    }
    if r < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
        || r > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
    {
        agerr(
            AGERR,
            b"install_in_rank, line %d: rank %d not in rank range [%d,%d]\n\0"
                as *const u8 as *const libc::c_char,
            1341 as libc::c_int,
            r,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank,
        );
        return;
    }
    if ((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset(r as isize))
        .v)
        .offset((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order as isize)
        > ((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .av)
            .offset(
                (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .an as isize,
            )
    {
        agerr(
            AGERR,
            b"install_in_rank, line %d: GD_rank(g)[%d].v + ND_order(%s) [%d] > GD_rank(g)[%d].av + GD_rank(Root)[%d].an [%d]\n\0"
                as *const u8 as *const libc::c_char,
            1347 as libc::c_int,
            r,
            agnameof(n as *mut libc::c_void),
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order,
            r,
            r,
            (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .an,
        );
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn build_ranks(mut g: *mut graph_t, mut pass: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut n0: *mut node_t = 0 as *mut node_t;
    let mut otheredges: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut q: *mut nodequeue = 0 as *mut nodequeue;
    q = new_queue(
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_nodes as libc::c_int,
    );
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .mark = 0 as libc::c_int as size_t;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    i = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(i as isize))
            .n = 0 as libc::c_int;
        i += 1;
    }
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        otheredges = if pass == 0 as libc::c_int {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list
        } else {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list
        };
        if (*otheredges.offset(0 as libc::c_int as isize)).is_null() {
            if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark == 0 {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
                enqueue(q, n);
                loop {
                    n0 = dequeue(q);
                    if n0.is_null() {
                        break;
                    }
                    if (*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ranktype
                        as libc::c_int != 7 as libc::c_int
                    {
                        install_in_rank(g, n0);
                        enqueue_neighbors(q, n0, pass);
                    } else {
                        install_cluster(g, n0, pass, q);
                    }
                }
            }
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    if !(dequeue(q)).is_null() {
        agerr(AGERR, b"surprise\n\0" as *const u8 as *const libc::c_char);
    }
    i = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(i as isize))
            .valid = 0 as libc::c_int != 0;
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int & 1 as libc::c_int != 0
            && (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(i as isize))
                .n > 0 as libc::c_int
        {
            let mut vlist: *mut *mut node_t = (*((*((*(g as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(i as isize))
                .v;
            let mut num_nodes_1: libc::c_int = (*((*((*(g as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(i as isize))
                .n - 1 as libc::c_int;
            let mut half_num_nodes_1: libc::c_int = num_nodes_1 / 2 as libc::c_int;
            j = 0 as libc::c_int;
            while j <= half_num_nodes_1 {
                exchange(
                    *vlist.offset(j as isize),
                    *vlist.offset((num_nodes_1 - j) as isize),
                );
                j += 1;
            }
        }
        i += 1;
    }
    if g == dot_root(g as *mut libc::c_void) && ncross(g) > 0 as libc::c_int {
        transpose(g, 0 as libc::c_int != 0);
    }
    free_queue(q);
}
#[no_mangle]
pub unsafe extern "C" fn enqueue_neighbors(
    mut q: *mut nodequeue,
    mut n0: *mut node_t,
    mut pass: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if pass == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size {
            e = *((*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(i as isize);
            if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .mark == 0
            {
                (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
                enqueue(
                    q,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                );
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size {
            e = *((*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(i as isize);
            if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .mark == 0
            {
                (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
                enqueue(
                    q,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node,
                );
            }
            i += 1;
        }
    };
}
unsafe extern "C" fn constraining_flat_edge(
    mut g: *mut Agraph_t,
    mut v: *mut Agnode_t,
    mut e: *mut Agedge_t,
) -> libc::c_int {
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if inside_cluster(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if inside_cluster(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn postorder(
    mut g: *mut graph_t,
    mut v: *mut node_t,
    mut list: *mut *mut node_t,
    mut r: libc::c_int,
) -> libc::c_int {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.size
        > 0 as libc::c_int
    {
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            if !(constraining_flat_edge(g, v, e) == 0) {
                if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .mark == 0
                {
                    cnt
                        += postorder(
                            g,
                            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                == 2 as libc::c_int
                            {
                                e
                            } else {
                                e.offset(-(1 as libc::c_int as isize))
                            })
                                .node,
                            list.offset(cnt as isize),
                            r,
                        );
                }
            }
            i += 1;
        }
    }
    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank == r {} else {
        __assert_fail(
            b"ND_rank(v) == r\0" as *const u8 as *const libc::c_char,
            b"mincross.c\0" as *const u8 as *const libc::c_char,
            1466 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"int postorder(graph_t *, node_t *, node_t **, int)\0"))
                .as_ptr(),
        );
    }
    let fresh42 = cnt;
    cnt = cnt + 1;
    let ref mut fresh43 = *list.offset(fresh42 as isize);
    *fresh43 = v;
    return cnt;
}
unsafe extern "C" fn flat_reorder(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut n_search: libc::c_int = 0;
    let mut local_in_cnt: libc::c_int = 0;
    let mut local_out_cnt: libc::c_int = 0;
    let mut base_order: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut left: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut right: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut t: *mut node_t = 0 as *mut node_t;
    let mut temprank: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut flat_e: *mut edge_t = 0 as *mut edge_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if !(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_flat_edges {
        return;
    }
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        if !((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n == 0 as libc::c_int)
        {
            base_order = (*((*(*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .v)
                .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order;
            i = 0 as libc::c_int;
            while i
                < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .n
            {
                (*((*(*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .v)
                    .offset(i as isize) as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .mark = 0 as libc::c_int as size_t;
                i += 1;
            }
            temprank = if !temprank.is_null() {
                grealloc(
                    temprank as *mut libc::c_void,
                    ((i + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut node_t
            } else {
                gmalloc(
                    ((i + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut node_t
            };
            pos = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i
                < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .n
            {
                if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                    & 0x3 as libc::c_int & 1 as libc::c_int != 0
                {
                    v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .v)
                        .offset(i as isize);
                } else {
                    v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .v)
                        .offset(
                            ((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                                .rank)
                                .offset(r as isize))
                                .n - i - 1 as libc::c_int) as isize,
                        );
                }
                local_out_cnt = 0 as libc::c_int;
                local_in_cnt = local_out_cnt;
                j = 0 as libc::c_int;
                while j
                    < (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_in.size
                {
                    flat_e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .flat_in
                        .list)
                        .offset(j as isize);
                    if constraining_flat_edge(g, v, flat_e) != 0 {
                        local_in_cnt += 1;
                    }
                    j += 1;
                }
                j = 0 as libc::c_int;
                while j
                    < (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.size
                {
                    flat_e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .flat_out
                        .list)
                        .offset(j as isize);
                    if constraining_flat_edge(g, v, flat_e) != 0 {
                        local_out_cnt += 1;
                    }
                    j += 1;
                }
                if local_in_cnt == 0 as libc::c_int && local_out_cnt == 0 as libc::c_int
                {
                    let fresh44 = pos;
                    pos = pos + 1;
                    let ref mut fresh45 = *temprank.offset(fresh44 as isize);
                    *fresh45 = v;
                } else if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark == 0
                        && local_in_cnt == 0 as libc::c_int
                    {
                    left = temprank.offset(pos as isize);
                    n_search = postorder(g, v, left, r);
                    pos += n_search;
                }
                i += 1;
            }
            if pos != 0 {
                if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                    & 0x3 as libc::c_int & 1 as libc::c_int == 0
                {
                    left = temprank;
                    right = temprank
                        .offset(pos as isize)
                        .offset(-(1 as libc::c_int as isize));
                    while left < right {
                        t = *left;
                        *left = *right;
                        *right = t;
                        left = left.offset(1);
                        right = right.offset(-1);
                    }
                }
                i = 0 as libc::c_int;
                while i
                    < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .n
                {
                    let ref mut fresh46 = *((*((*((*(g as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .rank)
                        .offset(r as isize))
                        .v)
                        .offset(i as isize);
                    *fresh46 = *temprank.offset(i as isize);
                    v = *fresh46;
                    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .order = i + base_order;
                    i += 1;
                }
                i = 0 as libc::c_int;
                while i
                    < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .n
                {
                    v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .v)
                        .offset(i as isize);
                    if !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .flat_out
                        .list)
                        .is_null()
                    {
                        j = 0 as libc::c_int;
                        loop {
                            e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .flat_out
                                .list)
                                .offset(j as isize);
                            if e.is_null() {
                                break;
                            }
                            if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int == 0
                                && (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                    as libc::c_int == 2 as libc::c_int
                                {
                                    e
                                } else {
                                    e.offset(-(1 as libc::c_int as isize))
                                }))
                                    .node as *mut Agobj_t))
                                    .data as *mut Agnodeinfo_t))
                                    .order
                                    < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                        as libc::c_int == 3 as libc::c_int
                                    {
                                        e
                                    } else {
                                        e.offset(1 as libc::c_int as isize)
                                    }))
                                        .node as *mut Agobj_t))
                                        .data as *mut Agnodeinfo_t))
                                        .order
                                || (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                                    .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
                                    && (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                        as libc::c_int == 2 as libc::c_int
                                    {
                                        e
                                    } else {
                                        e.offset(-(1 as libc::c_int as isize))
                                    }))
                                        .node as *mut Agobj_t))
                                        .data as *mut Agnodeinfo_t))
                                        .order
                                        > (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                                            as libc::c_int == 3 as libc::c_int
                                        {
                                            e
                                        } else {
                                            e.offset(1 as libc::c_int as isize)
                                        }))
                                            .node as *mut Agobj_t))
                                            .data as *mut Agnodeinfo_t))
                                            .order
                            {
                                if constraining_flat_edge(g, v, e) == 0 {} else {
                                    __assert_fail(
                                        b"!constraining_flat_edge(g,v,e)\0" as *const u8
                                            as *const libc::c_char,
                                        b"mincross.c\0" as *const u8 as *const libc::c_char,
                                        1537 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 29],
                                            &[libc::c_char; 29],
                                        >(b"void flat_reorder(graph_t *)\0"))
                                            .as_ptr(),
                                    );
                                }
                                delete_flat_edge(e);
                                j -= 1;
                                flat_rev(g, e);
                            }
                            j += 1;
                        }
                    }
                    i += 1;
                }
            }
            (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .valid = 0 as libc::c_int != 0;
        }
        r += 1;
    }
    if !temprank.is_null() {
        free(temprank as *mut libc::c_void);
    }
}
unsafe extern "C" fn reorder(
    mut g: *mut graph_t,
    mut r: libc::c_int,
    mut reverse: bool,
    mut hasfixed: bool,
) {
    let mut changed: libc::c_int = 0 as libc::c_int;
    let mut nelt: libc::c_int = 0;
    let mut vlist: *mut *mut node_t = (*((*((*(g as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .rank)
        .offset(r as isize))
        .v;
    let mut lp: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut rp: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut ep: *mut *mut node_t = vlist
        .offset(
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n as isize,
        );
    nelt = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset(r as isize))
        .n - 1 as libc::c_int;
    while nelt >= 0 as libc::c_int {
        lp = vlist;
        while lp < ep {
            while lp < ep
                && (*((*(*lp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval
                    < 0 as libc::c_int as libc::c_double
            {
                lp = lp.offset(1);
            }
            if lp >= ep {
                break;
            }
            let mut sawclust: bool = 0 as libc::c_int != 0;
            let mut muststay: bool = 0 as libc::c_int != 0;
            rp = lp.offset(1 as libc::c_int as isize);
            while rp < ep {
                if !(sawclust as libc::c_int != 0
                    && !((*((*(*rp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust)
                        .is_null())
                {
                    if left2right(g, *lp, *rp) != 0 {
                        muststay = 1 as libc::c_int != 0;
                        break;
                    } else {
                        if (*((*(*rp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval
                            >= 0 as libc::c_int as libc::c_double
                        {
                            break;
                        }
                        if !((*((*(*rp as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .clust)
                            .is_null()
                        {
                            sawclust = 1 as libc::c_int != 0;
                        }
                    }
                }
                rp = rp.offset(1);
            }
            if rp >= ep {
                break;
            }
            if !muststay {
                let mut p1: libc::c_int = (*((*(*lp as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .mval as libc::c_int;
                let mut p2: libc::c_int = (*((*(*rp as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .mval as libc::c_int;
                if p1 > p2 || p1 == p2 && reverse as libc::c_int != 0 {
                    exchange(*lp, *rp);
                    changed += 1;
                }
            }
            lp = rp;
        }
        if !hasfixed && !reverse {
            ep = ep.offset(-1);
        }
        nelt -= 1;
    }
    if changed != 0 {
        (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .valid = 0 as libc::c_int != 0;
        if r > 0 as libc::c_int {
            (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset((r - 1 as libc::c_int) as isize))
                .valid = 0 as libc::c_int != 0;
        }
    }
}
unsafe extern "C" fn mincross_step(mut g: *mut graph_t, mut pass: libc::c_int) {
    let mut r: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut reverse: bool = (pass % 4 as libc::c_int) < 2 as libc::c_int;
    if pass % 2 as libc::c_int == 0 as libc::c_int {
        first = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            + 1 as libc::c_int;
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            > (*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
        {
            first -= 1;
        }
        last = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank;
        dir = 1 as libc::c_int;
    } else {
        first = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            - 1 as libc::c_int;
        last = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            < (*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
        {
            first += 1;
        }
        dir = -(1 as libc::c_int);
    }
    r = first;
    while r != last + dir {
        other = r - dir;
        let mut hasfixed: bool = medians(g, r, other);
        reorder(g, r, reverse, hasfixed);
        r += dir;
    }
    transpose(g, !reverse);
}
unsafe extern "C" fn local_cross(mut l: elist, mut dir: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut is_out: libc::c_int = 0;
    let mut cross: libc::c_int = 0 as libc::c_int;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    if dir > 0 as libc::c_int {
        is_out = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        is_out = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    loop {
        e = *(l.list).offset(i as isize);
        if e.is_null() {
            break;
        }
        if is_out != 0 {
            j = i + 1 as libc::c_int;
            loop {
                f = *(l.list).offset(j as isize);
                if f.is_null() {
                    break;
                }
                if ((*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    f
                } else {
                    f.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order
                    - (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .order) as libc::c_double
                    * ((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .tail_port
                        .p
                        .x
                        - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .tail_port
                            .p
                            .x) < 0 as libc::c_int as libc::c_double
                {
                    cross
                        += (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xpenalty
                            as libc::c_int
                            * (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .xpenalty as libc::c_int;
                }
                j += 1;
            }
        } else {
            j = i + 1 as libc::c_int;
            loop {
                f = *(l.list).offset(j as isize);
                if f.is_null() {
                    break;
                }
                if ((*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    f
                } else {
                    f.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order
                    - (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .order) as libc::c_double
                    * ((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .head_port
                        .p
                        .x
                        - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .head_port
                            .p
                            .x) < 0 as libc::c_int as libc::c_double
                {
                    cross
                        += (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xpenalty
                            as libc::c_int
                            * (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .xpenalty as libc::c_int;
                }
                j += 1;
            }
        }
        i += 1;
    }
    return cross;
}
unsafe extern "C" fn rcross(mut g: *mut graph_t, mut r: libc::c_int) -> libc::c_int {
    static mut Count: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
    static mut C: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut bot: libc::c_int = 0;
    let mut cross: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut rtop: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    cross = 0 as libc::c_int;
    max = 0 as libc::c_int;
    rtop = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset(r as isize))
        .v;
    if C
        <= (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((r + 1 as libc::c_int) as isize))
            .n
    {
        C = (*((*((*(Root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((r + 1 as libc::c_int) as isize))
            .n + 1 as libc::c_int;
        Count = if !Count.is_null() {
            grealloc(
                Count as *mut libc::c_void,
                (C as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int
        } else {
            gmalloc(
                (C as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int
        };
    }
    i = 0 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((r + 1 as libc::c_int) as isize))
            .n
    {
        *Count.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    top = 0 as libc::c_int;
    while top
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n
    {
        let mut e: *mut edge_t = 0 as *mut edge_t;
        if max > 0 as libc::c_int {
            i = 0 as libc::c_int;
            loop {
                e = *((*((*(*rtop.offset(top as isize) as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .offset(i as isize);
                if e.is_null() {
                    break;
                }
                k = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order + 1 as libc::c_int;
                while k <= max {
                    cross
                        += *Count.offset(k as isize)
                            * (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .xpenalty as libc::c_int;
                    k += 1;
                }
                i += 1;
            }
        }
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(*rtop.offset(top as isize) as *mut Agobj_t)).data
                as *mut Agnodeinfo_t))
                .out
                .list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            let mut inv: libc::c_int = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order;
            if inv > max {
                max = inv;
            }
            *Count.offset(inv as isize)
                += (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xpenalty
                    as libc::c_int;
            i += 1;
        }
        top += 1;
    }
    top = 0 as libc::c_int;
    while top
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n
    {
        v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(top as isize);
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).has_port {
            cross
                += local_cross(
                    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out,
                    1 as libc::c_int,
                );
        }
        top += 1;
    }
    bot = 0 as libc::c_int;
    while bot
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((r + 1 as libc::c_int) as isize))
            .n
    {
        v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((r + 1 as libc::c_int) as isize))
            .v)
            .offset(bot as isize);
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).has_port {
            cross
                += local_cross(
                    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0,
                    -(1 as libc::c_int),
                );
        }
        bot += 1;
    }
    return cross;
}
#[no_mangle]
pub unsafe extern "C" fn ncross(mut g: *mut graph_t) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    g = Root;
    count = 0 as libc::c_int;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        if (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .valid
        {
            count
                += (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .cache_nc;
        } else {
            let ref mut fresh47 = (*((*((*(g as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .cache_nc;
            *fresh47 = rcross(g, r);
            nc = *fresh47;
            count += nc;
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .valid = 1 as libc::c_int != 0;
        }
        r += 1;
    }
    return count;
}
unsafe extern "C" fn ordercmpf(
    mut i0: *mut libc::c_int,
    mut i1: *mut libc::c_int,
) -> libc::c_int {
    return *i0 - *i1;
}
unsafe extern "C" fn flat_mval(mut n: *mut node_t) -> bool {
    let mut i: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut fl: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut nn: *mut node_t = 0 as *mut node_t;
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_in.size
        > 0 as libc::c_int
    {
        fl = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_in.list;
        nn = (*if ((*(*fl.offset(0 as libc::c_int as isize) as *mut Agobj_t)).tag)
            .objtype() as libc::c_int == 3 as libc::c_int
        {
            *fl.offset(0 as libc::c_int as isize)
        } else {
            (*fl.offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
        })
            .node;
        i = 1 as libc::c_int;
        loop {
            e = *fl.offset(i as isize);
            if e.is_null() {
                break;
            }
            if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order > (*((*(nn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            {
                nn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node;
            }
            i += 1;
        }
        if (*((*(nn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval
            >= 0 as libc::c_int as libc::c_double
        {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .mval = (*((*(nn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval
                + 1 as libc::c_int as libc::c_double;
            return 0 as libc::c_int != 0;
        }
    } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.size
            > 0 as libc::c_int
        {
        fl = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.list;
        nn = (*if ((*(*fl.offset(0 as libc::c_int as isize) as *mut Agobj_t)).tag)
            .objtype() as libc::c_int == 2 as libc::c_int
        {
            *fl.offset(0 as libc::c_int as isize)
        } else {
            (*fl.offset(0 as libc::c_int as isize)).offset(-(1 as libc::c_int as isize))
        })
            .node;
        i = 1 as libc::c_int;
        loop {
            e = *fl.offset(i as isize);
            if e.is_null() {
                break;
            }
            if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order < (*((*(nn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            {
                nn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
            }
            i += 1;
        }
        if (*((*(nn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval
            > 0 as libc::c_int as libc::c_double
        {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .mval = (*((*(nn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval
                - 1 as libc::c_int as libc::c_double;
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn medians(
    mut g: *mut graph_t,
    mut r0: libc::c_int,
    mut r1: libc::c_int,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut j0: libc::c_int = 0;
    let mut lspan: libc::c_int = 0;
    let mut rspan: libc::c_int = 0;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut v: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut hasfixed: bool = 0 as libc::c_int != 0;
    list = TI_list;
    v = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset(r0 as isize))
        .v;
    i = 0 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r0 as isize))
            .n
    {
        n = *v.offset(i as isize);
        let mut j: size_t = 0 as libc::c_int as size_t;
        if r1 > r0 {
            j0 = 0 as libc::c_int;
            loop {
                e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                    .offset(j0 as isize);
                if e.is_null() {
                    break;
                }
                if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xpenalty
                    as libc::c_int > 0 as libc::c_int
                {
                    let fresh48 = j;
                    j = j.wrapping_add(1);
                    *list
                        .offset(
                            fresh48 as isize,
                        ) = 256 as libc::c_int
                        * (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .order
                        + (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .head_port
                            .order as libc::c_int;
                }
                j0 += 1;
            }
        } else {
            j0 = 0 as libc::c_int;
            loop {
                e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                    .offset(j0 as isize);
                if e.is_null() {
                    break;
                }
                if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xpenalty
                    as libc::c_int > 0 as libc::c_int
                {
                    let fresh49 = j;
                    j = j.wrapping_add(1);
                    *list
                        .offset(
                            fresh49 as isize,
                        ) = 256 as libc::c_int
                        * (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .order
                        + (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .tail_port
                            .order as libc::c_int;
                }
                j0 += 1;
            }
        }
        match j {
            0 => {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .mval = -(1 as libc::c_int) as libc::c_double;
            }
            1 => {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .mval = *list.offset(0 as libc::c_int as isize) as libc::c_double;
            }
            2 => {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .mval = ((*list.offset(0 as libc::c_int as isize)
                    + *list.offset(1 as libc::c_int as isize)) / 2 as libc::c_int)
                    as libc::c_double;
            }
            _ => {
                qsort(
                    list as *mut libc::c_void,
                    j,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_int,
                                *mut libc::c_int,
                            ) -> libc::c_int,
                        >,
                        qsort_cmpf,
                    >(
                        Some(
                            ordercmpf
                                as unsafe extern "C" fn(
                                    *mut libc::c_int,
                                    *mut libc::c_int,
                                ) -> libc::c_int,
                        ),
                    ),
                );
                if j.wrapping_rem(2 as libc::c_int as libc::c_ulong) != 0 {
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .mval = *list
                        .offset(
                            j.wrapping_div(2 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_double;
                } else {
                    let mut rm: size_t = j
                        .wrapping_div(2 as libc::c_int as libc::c_ulong);
                    let mut lm: size_t = rm
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    rspan = *list
                        .offset(
                            j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) - *list.offset(rm as isize);
                    lspan = *list.offset(lm as isize)
                        - *list.offset(0 as libc::c_int as isize);
                    if lspan == rspan {
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .mval = ((*list.offset(lm as isize)
                            + *list.offset(rm as isize)) / 2 as libc::c_int)
                            as libc::c_double;
                    } else {
                        let mut w: libc::c_double = *list.offset(lm as isize)
                            as libc::c_double * rspan as libc::c_double
                            + *list.offset(rm as isize) as libc::c_double
                                * lspan as libc::c_double;
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .mval = w / (lspan + rspan) as libc::c_double;
                    }
                }
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i
        < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r0 as isize))
            .n
    {
        n = *v.offset(i as isize);
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
            == 0 as libc::c_int
            && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                == 0 as libc::c_int
        {
            hasfixed = (hasfixed as libc::c_int | flat_mval(n) as libc::c_int) as bool;
        }
        i += 1;
    }
    return hasfixed;
}
unsafe extern "C" fn nodeposcmpf(
    mut n0: *mut *mut node_t,
    mut n1: *mut *mut node_t,
) -> libc::c_int {
    return (*((*(*n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
        - (*((*(*n1 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order;
}
unsafe extern "C" fn edgeidcmpf(
    mut e0: *mut *mut edge_t,
    mut e1: *mut *mut edge_t,
) -> libc::c_int {
    return ((*(*e0 as *mut Agobj_t)).tag).seq() as libc::c_int
        - ((*(*e1 as *mut Agobj_t)).tag).seq() as libc::c_int;
}
static mut table: [[libc::c_int; 3]; 3] = [
    [1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int],
    [1 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int],
    [1 as libc::c_int, 2 as libc::c_int, 4 as libc::c_int],
];
unsafe extern "C" fn endpoint_class(mut n: *mut node_t) -> libc::c_int {
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 1 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).weight_class as libc::c_int
        <= 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn virtual_weight(mut e: *mut edge_t) {
    let mut t: libc::c_int = 0;
    t = table[endpoint_class(
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node,
    )
        as usize][endpoint_class(
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
    ) as usize];
    if t >= 0 as libc::c_int {} else {
        __assert_fail(
            b"t >= 0\0" as *const u8 as *const libc::c_char,
            b"mincross.c\0" as *const u8 as *const libc::c_char,
            1870 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void virtual_weight(edge_t *)\0"))
                .as_ptr(),
        );
    }
    if 2147483647 as libc::c_int / t
        < (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
    {
        agerr(
            AGERR,
            b"overflow when calculating virtual weight of edge\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight *= t;
}
unsafe extern "C" fn mincross_options(mut g: *mut graph_t) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: libc::c_double = 0.;
    MinQuit = 8 as libc::c_int;
    MaxIter = 24 as libc::c_int;
    Convergence = 0.995f64;
    p = agget(
        g as *mut libc::c_void,
        b"mclimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null()
        && {
            f = atof(p);
            f > 0.0f64
        }
    {
        MinQuit = (if 1 as libc::c_int as libc::c_double > MinQuit as libc::c_double * f
        {
            1 as libc::c_int as libc::c_double
        } else {
            MinQuit as libc::c_double * f
        }) as libc::c_int;
        MaxIter = (if 1 as libc::c_int as libc::c_double > MaxIter as libc::c_double * f
        {
            1 as libc::c_int as libc::c_double
        } else {
            MaxIter as libc::c_double * f
        }) as libc::c_int;
    }
}
