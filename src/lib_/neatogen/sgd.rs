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
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn log(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn hypotf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    fn initial_positions(_: *mut graph_t, _: libc::c_int);
    static mut Verbose: libc::c_uchar;
    static mut Epsilon: libc::c_double;
    static mut MaxIter: libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn dijkstra_sgd(_: *mut graph_sgd, _: libc::c_int, _: *mut term_sgd) -> libc::c_int;
    fn rk_seed(seed: libc::c_ulong, state: *mut rk_state);
    fn rk_interval(max: libc::c_ulong, state: *mut rk_state) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
pub struct bitarray_t {
    pub c2rust_unnamed: C2RustUnnamed,
    pub size_bits: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub block: [uint8_t; 8],
    pub base: *mut uint8_t,
}
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
pub type gvevent_key_callback_t = Option<unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int>;
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
    pub refresh: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub button_press: Option<unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> ()>,
    pub button_release: Option<unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> ()>,
    pub motion: Option<unsafe extern "C" fn(*mut GVJ_t, pointf) -> ()>,
    pub modify:
        Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> ()>,
    pub del: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub read:
        Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> ()>,
    pub layout: Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> ()>,
    pub render:
        Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, *const libc::c_char) -> ()>,
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
    pub u: C2RustUnnamed_1,
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
    pub explicit_tooltip_explicit_tailtooltip_explicit_headtooltip_explicit_labeltooltip_explicit_tailtarget_explicit_headtarget_explicit_edgetarget_explicit_tailurl_explicit_headurl_labeledgealigned:
        [u8; 2],
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
    pub u: C2RustUnnamed_0,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub union C2RustUnnamed_1 {
    pub g: *mut graph_t,
    pub sg: *mut graph_t,
    pub n: *mut node_t,
    pub e: *mut edge_t,
}
pub type edge_t = Agedge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agedge_s {
    pub base: Agobj_t,
    pub id_link: Dtlink_t,
    pub seq_link: Dtlink_t,
    pub node: *mut Agnode_t,
}
pub type Agnode_t = Agnode_s;
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
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
pub type Agraph_t = Agraph_s;
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
pub type Dtsearch_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void>;
pub type Dtmemory_f = Option<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, size_t, *mut Dtdisc_t) -> *mut libc::c_void,
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
pub type Dtevent_f = Option<
    unsafe extern "C" fn(*mut Dt_t, libc::c_int, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_int,
>;
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
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed_3,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
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
    pub graph: C2RustUnnamed_4,
    pub node: C2RustUnnamed_4,
    pub edge: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ins: agobjfn_t,
    pub mod_0: agobjupdfn_t,
    pub del: agobjfn_t,
}
pub type agobjfn_t =
    Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> ()>;
pub type Agobj_t = Agobj_s;
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
pub type Agtag_t = Agtag_s;
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
pub type IDTYPE = uint64_t;
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
    pub errorfn: Option<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
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
    pub write_fn: Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char, size_t) -> size_t>,
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
    pub free_layout: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub yoffset_layout: libc::c_double,
    pub yoffset_centerline: libc::c_double,
    pub size: pointf,
    pub just: libc::c_char,
}
pub type Agedge_t = Agedge_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_6,
    pub s: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub u: C2RustUnnamed_7,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub txt: C2RustUnnamed_8,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub cleanup: Option<unsafe extern "C" fn(*mut graph_t) -> ()>,
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
pub struct term_sgd {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub d: libc::c_float,
    pub w: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct graph_sgd {
    pub n: size_t,
    pub sources: *mut size_t,
    pub pinneds: bitarray_t,
    pub targets: *mut size_t,
    pub weights: *mut libc::c_float,
}
pub type rk_state = rk_state_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rk_state_ {
    pub key: [libc::c_ulong; 624],
    pub pos: libc::c_int,
}
#[inline]
unsafe extern "C" fn bitarray_new(
    mut self_0: *mut bitarray_t,
    mut size_bits: size_t,
) -> libc::c_int {
    if !self_0.is_null() {
    } else {
        __assert_fail(
            b"self != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int bitarray_new(bitarray_t *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if (*self_0).size_bits == 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"self->size_bits == 0\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int bitarray_new(bitarray_t *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*self_0).c2rust_unnamed.block).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
        );
    } else {
        let mut capacity: size_t = size_bits
            .wrapping_div(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if size_bits.wrapping_rem(8 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as libc::c_ulong,
            );
        let mut base: *mut uint8_t =
            calloc(capacity, ::std::mem::size_of::<uint8_t>() as libc::c_ulong) as *mut uint8_t;
        if (base == 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int as libc::c_long != 0 {
            return 12 as libc::c_int;
        }
        let ref mut fresh0 = (*self_0).c2rust_unnamed.base;
        *fresh0 = base;
    }
    (*self_0).size_bits = size_bits;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bitarray_new_or_exit(mut size_bits: size_t) -> bitarray_t {
    let mut ba: bitarray_t = bitarray_t {
        c2rust_unnamed: C2RustUnnamed { block: [0; 8] },
        size_bits: 0,
    };
    memset(
        &mut ba as *mut bitarray_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bitarray_t>() as libc::c_ulong,
    );
    let mut error: libc::c_int = bitarray_new(&mut ba, size_bits);
    if (error != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return ba;
}
#[inline]
unsafe extern "C" fn bitarray_get(mut self_0: bitarray_t, mut index: size_t) -> bool {
    if index < self_0.size_bits
        && !(b"out of bounds access\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"index < self.size_bits && \"out of bounds access\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"_Bool bitarray_get(bitarray_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    let mut base: *const uint8_t = 0 as *const uint8_t;
    if self_0.size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        base = (self_0.c2rust_unnamed.block).as_mut_ptr();
    } else {
        base = self_0.c2rust_unnamed.base;
    }
    return *base.offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int
        >> index.wrapping_rem(8 as libc::c_int as libc::c_ulong)
        & 1 as libc::c_int
        != 0;
}
#[inline]
unsafe extern "C" fn bitarray_set(mut self_0: *mut bitarray_t, mut index: size_t, mut value: bool) {
    if index < (*self_0).size_bits
        && !(b"out of bounds access\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"index < self->size_bits && \"out of bounds access\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"void bitarray_set(bitarray_t *, size_t, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    let mut base: *mut uint8_t = 0 as *mut uint8_t;
    if (*self_0).size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        base = ((*self_0).c2rust_unnamed.block).as_mut_ptr();
    } else {
        base = (*self_0).c2rust_unnamed.base;
    }
    if value {
        let ref mut fresh1 =
            *base.offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
        *fresh1 = (*fresh1 as libc::c_int
            | ((1 as libc::c_int) << index.wrapping_rem(8 as libc::c_int as libc::c_ulong))
                as uint8_t as libc::c_int) as uint8_t;
    } else {
        let ref mut fresh2 =
            *base.offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
        *fresh2 = (*fresh2 as libc::c_int
            & !((1 as libc::c_int) << index.wrapping_rem(8 as libc::c_int as libc::c_ulong))
                as uint8_t as libc::c_int) as uint8_t;
    };
}
#[inline]
unsafe extern "C" fn bitarray_reset(mut self_0: *mut bitarray_t) {
    if !self_0.is_null() {
    } else {
        __assert_fail(
            b"self != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void bitarray_reset(bitarray_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*self_0).size_bits
        > (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        free((*self_0).c2rust_unnamed.base as *mut libc::c_void);
    }
    memset(
        self_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bitarray_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn calculate_stress(
    mut pos: *mut libc::c_float,
    mut terms: *mut term_sgd,
    mut n_terms: libc::c_int,
) -> libc::c_float {
    let mut stress: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut ij: libc::c_int = 0;
    ij = 0 as libc::c_int;
    while ij < n_terms {
        let mut dx: libc::c_float = *pos
            .offset((2 as libc::c_int * (*terms.offset(ij as isize)).i) as isize)
            - *pos.offset((2 as libc::c_int * (*terms.offset(ij as isize)).j) as isize);
        let mut dy: libc::c_float = *pos.offset(
            (2 as libc::c_int * (*terms.offset(ij as isize)).i + 1 as libc::c_int) as isize,
        ) - *pos.offset(
            (2 as libc::c_int * (*terms.offset(ij as isize)).j + 1 as libc::c_int) as isize,
        );
        let mut r: libc::c_float = hypotf(dx, dy) - (*terms.offset(ij as isize)).d;
        stress += (*terms.offset(ij as isize)).w * (r * r);
        ij += 1;
    }
    return stress;
}
static mut rstate: rk_state = rk_state {
    key: [0; 624],
    pos: 0,
};
unsafe extern "C" fn fisheryates_shuffle(mut terms: *mut term_sgd, mut n_terms: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = n_terms - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut j: libc::c_int = rk_interval(i as libc::c_ulong, &mut rstate) as libc::c_int;
        let mut temp: term_sgd = *terms.offset(i as isize);
        *terms.offset(i as isize) = *terms.offset(j as isize);
        *terms.offset(j as isize) = temp;
        i -= 1;
    }
}
unsafe extern "C" fn extract_adjacency(
    mut G: *mut graph_t,
    mut model: libc::c_int,
) -> *mut graph_sgd {
    let mut np: *mut node_t = 0 as *mut node_t;
    let mut ep: *mut edge_t = 0 as *mut edge_t;
    let mut n_nodes: size_t = 0 as libc::c_int as size_t;
    let mut n_edges: size_t = 0 as libc::c_int as size_t;
    np = agfstnode(G);
    while !np.is_null() {
        if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id as libc::c_ulong == n_nodes {
        } else {
            __assert_fail(
                b"ND_id(np) == n_nodes\0" as *const u8 as *const libc::c_char,
                b"sgd.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"graph_sgd *extract_adjacency(graph_t *, int)\0",
                ))
                .as_ptr(),
            );
        }
        n_nodes = n_nodes.wrapping_add(1);
        ep = agfstedge(G, np);
        while !ep.is_null() {
            if (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                ep
            } else {
                ep.offset(1 as libc::c_int as isize)
            }))
            .node
                != (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                }))
                .node
            {
                n_edges = n_edges.wrapping_add(1);
            }
            ep = agnxtedge(G, ep, np);
        }
        np = agnxtnode(G, np);
    }
    let mut graph: *mut graph_sgd = gcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<graph_sgd>() as libc::c_ulong,
    ) as *mut graph_sgd;
    let ref mut fresh3 = (*graph).sources;
    *fresh3 = gcalloc(
        n_nodes.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    ) as *mut size_t;
    (*graph).pinneds = bitarray_new_or_exit(n_nodes);
    let ref mut fresh4 = (*graph).targets;
    *fresh4 = gcalloc(n_edges, ::std::mem::size_of::<size_t>() as libc::c_ulong) as *mut size_t;
    let ref mut fresh5 = (*graph).weights;
    *fresh5 = gcalloc(
        n_edges,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    (*graph).n = n_nodes;
    if n_edges <= 2147483647 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"n_edges <= INT_MAX\0" as *const u8 as *const libc::c_char,
            b"sgd.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"graph_sgd *extract_adjacency(graph_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    *((*graph).sources).offset((*graph).n as isize) = n_edges;
    n_nodes = 0 as libc::c_int as size_t;
    n_edges = 0 as libc::c_int as size_t;
    np = agfstnode(G);
    while !np.is_null() {
        if n_edges <= 2147483647 as libc::c_int as libc::c_ulong {
        } else {
            __assert_fail(
                b"n_edges <= INT_MAX\0" as *const u8 as *const libc::c_char,
                b"sgd.c\0" as *const u8 as *const libc::c_char,
                65 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"graph_sgd *extract_adjacency(graph_t *, int)\0",
                ))
                .as_ptr(),
            );
        }
        *((*graph).sources).offset(n_nodes as isize) = n_edges;
        bitarray_set(
            &mut (*graph).pinneds,
            n_nodes,
            (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned as libc::c_int
                > 1 as libc::c_int,
        );
        ep = agfstedge(G, np);
        while !ep.is_null() {
            if !((*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                ep
            } else {
                ep.offset(1 as libc::c_int as isize)
            }))
            .node
                == (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                }))
                .node)
            {
                let mut target: *mut node_t = if (*(if ((*(ep as *mut Agobj_t)).tag).objtype()
                    as libc::c_int
                    == 3 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(1 as libc::c_int as isize)
                }))
                .node
                    == np
                {
                    (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(-(1 as libc::c_int as isize))
                    })
                    .node
                } else {
                    (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(1 as libc::c_int as isize)
                    })
                    .node
                };
                *((*graph).targets).offset(n_edges as isize) =
                    (*((*(target as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id as size_t;
                *((*graph).weights).offset(n_edges as isize) =
                    (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist as libc::c_float;
                if *((*graph).weights).offset(n_edges as isize) > 0 as libc::c_int as libc::c_float
                {
                } else {
                    __assert_fail(
                        b"graph->weights[n_edges] > 0\0" as *const u8 as *const libc::c_char,
                        b"sgd.c\0" as *const u8 as *const libc::c_char,
                        75 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"graph_sgd *extract_adjacency(graph_t *, int)\0",
                        ))
                        .as_ptr(),
                    );
                }
                n_edges = n_edges.wrapping_add(1);
            }
            ep = agnxtedge(G, ep, np);
        }
        n_nodes = n_nodes.wrapping_add(1);
        np = agnxtnode(G, np);
    }
    if n_nodes == (*graph).n {
    } else {
        __assert_fail(
            b"n_nodes == graph->n\0" as *const u8 as *const libc::c_char,
            b"sgd.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"graph_sgd *extract_adjacency(graph_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if n_edges <= 2147483647 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"n_edges <= INT_MAX\0" as *const u8 as *const libc::c_char,
            b"sgd.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"graph_sgd *extract_adjacency(graph_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if n_edges == *((*graph).sources).offset((*graph).n as isize) {
    } else {
        __assert_fail(
            b"n_edges == graph->sources[graph->n]\0" as *const u8 as *const libc::c_char,
            b"sgd.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"graph_sgd *extract_adjacency(graph_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    *((*graph).sources).offset(n_nodes as isize) = n_edges;
    if !(model == 0 as libc::c_int) {
        if model == 2 as libc::c_int {
            let mut neighbours_i: bitarray_t = bitarray_new_or_exit((*graph).n);
            let mut neighbours_j: bitarray_t = bitarray_new_or_exit((*graph).n);
            let mut i: size_t = 0 as libc::c_int as size_t;
            while i < (*graph).n {
                let mut deg_i: libc::c_int = 0 as libc::c_int;
                let mut x: size_t = *((*graph).sources).offset(i as isize);
                while x < *((*graph).sources)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                {
                    let mut j: size_t = *((*graph).targets).offset(x as isize);
                    if !bitarray_get(neighbours_i, j) {
                        bitarray_set(&mut neighbours_i, j, 1 as libc::c_int != 0);
                        deg_i += 1;
                    }
                    x = x.wrapping_add(1);
                }
                let mut x_0: size_t = *((*graph).sources).offset(i as isize);
                while x_0
                    < *((*graph).sources)
                        .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                {
                    let mut j_0: size_t = *((*graph).targets).offset(x_0 as isize);
                    let mut intersect: libc::c_int = 0 as libc::c_int;
                    let mut deg_j: libc::c_int = 0 as libc::c_int;
                    let mut y: size_t = *((*graph).sources).offset(j_0 as isize);
                    while y < *((*graph).sources)
                        .offset(j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    {
                        let mut k: size_t = *((*graph).targets).offset(y as isize);
                        if !bitarray_get(neighbours_j, k) {
                            bitarray_set(&mut neighbours_j, k, 1 as libc::c_int != 0);
                            deg_j += 1;
                            if bitarray_get(neighbours_i, k) {
                                intersect += 1;
                            }
                        }
                        y = y.wrapping_add(1);
                    }
                    *((*graph).weights).offset(x_0 as isize) =
                        (deg_i + deg_j - 2 as libc::c_int * intersect) as libc::c_float;
                    if *((*graph).weights).offset(x_0 as isize) > 0 as libc::c_int as libc::c_float
                    {
                    } else {
                        __assert_fail(
                            b"graph->weights[x] > 0\0" as *const u8 as *const libc::c_char,
                            b"sgd.c\0" as *const u8 as *const libc::c_char,
                            117 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                                b"graph_sgd *extract_adjacency(graph_t *, int)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                    let mut y_0: size_t = *((*graph).sources).offset(j_0 as isize);
                    while y_0
                        < *((*graph).sources)
                            .offset(j_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    {
                        let mut k_0: size_t = *((*graph).targets).offset(y_0 as isize);
                        bitarray_set(&mut neighbours_j, k_0, 0 as libc::c_int != 0);
                        y_0 = y_0.wrapping_add(1);
                    }
                    x_0 = x_0.wrapping_add(1);
                }
                let mut x_1: size_t = *((*graph).sources).offset(i as isize);
                while x_1
                    < *((*graph).sources)
                        .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                {
                    let mut j_1: size_t = *((*graph).targets).offset(x_1 as isize);
                    bitarray_set(&mut neighbours_i, j_1, 0 as libc::c_int != 0);
                    x_1 = x_1.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            bitarray_reset(&mut neighbours_i);
            bitarray_reset(&mut neighbours_j);
        } else {
            __assert_fail(
                b"false\0" as *const u8 as *const libc::c_char,
                b"sgd.c\0" as *const u8 as *const libc::c_char,
                133 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"graph_sgd *extract_adjacency(graph_t *, int)\0",
                ))
                .as_ptr(),
            );
        }
    }
    return graph;
}
unsafe extern "C" fn free_adjacency(mut graph: *mut graph_sgd) {
    free((*graph).sources as *mut libc::c_void);
    bitarray_reset(&mut (*graph).pinneds);
    free((*graph).targets as *mut libc::c_void);
    free((*graph).weights as *mut libc::c_void);
    free(graph as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sgd(mut G: *mut graph_t, mut model: libc::c_int) {
    if model == 1 as libc::c_int {
        agerr(
            AGWARN,
            b"circuit model not yet supported in Gmode=sgd, reverting to shortpath model\n\0"
                as *const u8 as *const libc::c_char,
        );
        model = 0 as libc::c_int;
    }
    if model == 3 as libc::c_int {
        agerr(
            AGWARN,
            b"mds model not yet supported in Gmode=sgd, reverting to shortpath model\n\0"
                as *const u8 as *const libc::c_char,
        );
        model = 0 as libc::c_int;
    }
    let mut n: libc::c_int = agnnodes(G);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"calculating shortest paths and setting up stress terms:\0" as *const u8
                as *const libc::c_char,
        );
        start_timer();
    }
    let mut i: libc::c_int = 0;
    let mut n_fixed: libc::c_int = 0 as libc::c_int;
    let mut n_terms: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if !((*((*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .pinned as libc::c_int
            > 1 as libc::c_int)
        {
            n_fixed += 1;
            n_terms += n - n_fixed;
        }
        i += 1;
    }
    let mut terms: *mut term_sgd = gcalloc(
        n_terms as size_t,
        ::std::mem::size_of::<term_sgd>() as libc::c_ulong,
    ) as *mut term_sgd;
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut graph: *mut graph_sgd = extract_adjacency(G, model);
    i = 0 as libc::c_int;
    while i < n {
        if !((*((*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .pinned as libc::c_int
            > 1 as libc::c_int)
        {
            offset += dijkstra_sgd(graph, i, terms.offset(offset as isize));
        }
        i += 1;
    }
    if offset == n_terms {
    } else {
        __assert_fail(
            b"offset == n_terms\0" as *const u8 as *const libc::c_char,
            b"sgd.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void sgd(graph_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    free_adjacency(graph);
    if Verbose != 0 {
        fprintf(
            stderr,
            b" %.2f sec\n\0" as *const u8 as *const libc::c_char,
            elapsed_sec(),
        );
    }
    let mut w_min: libc::c_float = (*terms.offset(0 as libc::c_int as isize)).w;
    let mut w_max: libc::c_float = (*terms.offset(0 as libc::c_int as isize)).w;
    let mut ij: libc::c_int = 0;
    ij = 1 as libc::c_int;
    while ij < n_terms {
        if (*terms.offset(ij as isize)).w < w_min {
            w_min = (*terms.offset(ij as isize)).w;
        }
        if (*terms.offset(ij as isize)).w > w_max {
            w_max = (*terms.offset(ij as isize)).w;
        }
        ij += 1;
    }
    let mut eta_max: libc::c_float = 1 as libc::c_int as libc::c_float / w_min;
    let mut eta_min: libc::c_float = (Epsilon / w_max as libc::c_double) as libc::c_float;
    let mut lambda: libc::c_float = (log((eta_max / eta_min) as libc::c_double)
        / (MaxIter - 1 as libc::c_int) as libc::c_double)
        as libc::c_float;
    initial_positions(G, n);
    let mut pos: *mut libc::c_float = gcalloc(
        (2 as libc::c_int * n) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut unfixed: *mut bool =
        gcalloc(n as size_t, ::std::mem::size_of::<bool>() as libc::c_ulong) as *mut bool;
    i = 0 as libc::c_int;
    while i < n {
        let mut node: *mut node_t =
            *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(i as isize);
        *pos.offset((2 as libc::c_int * i) as isize) =
            *((*((*(node as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) as libc::c_float;
        *pos.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) =
            *((*((*(node as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) as libc::c_float;
        *unfixed.offset(i as isize) = !((*((*(node as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .pinned as libc::c_int
            > 1 as libc::c_int);
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"solving model:\0" as *const u8 as *const libc::c_char,
        );
        start_timer();
    }
    let mut t: libc::c_int = 0;
    rk_seed(0 as libc::c_int as libc::c_ulong, &mut rstate);
    t = 0 as libc::c_int;
    while t < MaxIter {
        fisheryates_shuffle(terms, n_terms);
        let mut eta: libc::c_float = (eta_max as libc::c_double
            * exp((-lambda * t as libc::c_float) as libc::c_double))
            as libc::c_float;
        ij = 0 as libc::c_int;
        while ij < n_terms {
            let mut mu: libc::c_float = eta * (*terms.offset(ij as isize)).w;
            if mu > 1 as libc::c_int as libc::c_float {
                mu = 1 as libc::c_int as libc::c_float;
            }
            let mut dx: libc::c_float = *pos
                .offset((2 as libc::c_int * (*terms.offset(ij as isize)).i) as isize)
                - *pos.offset((2 as libc::c_int * (*terms.offset(ij as isize)).j) as isize);
            let mut dy: libc::c_float = *pos.offset(
                (2 as libc::c_int * (*terms.offset(ij as isize)).i + 1 as libc::c_int) as isize,
            ) - *pos.offset(
                (2 as libc::c_int * (*terms.offset(ij as isize)).j + 1 as libc::c_int) as isize,
            );
            let mut mag: libc::c_float = hypotf(dx, dy);
            let mut r: libc::c_float = mu * (mag - (*terms.offset(ij as isize)).d)
                / (2 as libc::c_int as libc::c_float * mag);
            let mut r_x: libc::c_float = r * dx;
            let mut r_y: libc::c_float = r * dy;
            if *unfixed.offset((*terms.offset(ij as isize)).i as isize) {
                *pos.offset((2 as libc::c_int * (*terms.offset(ij as isize)).i) as isize) -= r_x;
                *pos.offset(
                    (2 as libc::c_int * (*terms.offset(ij as isize)).i + 1 as libc::c_int) as isize,
                ) -= r_y;
            }
            if *unfixed.offset((*terms.offset(ij as isize)).j as isize) {
                *pos.offset((2 as libc::c_int * (*terms.offset(ij as isize)).j) as isize) += r_x;
                *pos.offset(
                    (2 as libc::c_int * (*terms.offset(ij as isize)).j + 1 as libc::c_int) as isize,
                ) += r_y;
            }
            ij += 1;
        }
        if Verbose != 0 {
            fprintf(
                stderr,
                b" %.3f\0" as *const u8 as *const libc::c_char,
                calculate_stress(pos, terms, n_terms) as libc::c_double,
            );
        }
        t += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"\nfinished in %.2f sec\n\0" as *const u8 as *const libc::c_char,
            elapsed_sec(),
        );
    }
    free(terms as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < n {
        let mut node_0: *mut node_t =
            *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(i as isize);
        *((*((*(node_0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) =
            *pos.offset((2 as libc::c_int * i) as isize) as libc::c_double;
        *((*((*(node_0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) =
            *pos.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) as libc::c_double;
        i += 1;
    }
    free(pos as *mut libc::c_void);
    free(unfixed as *mut libc::c_void);
}
