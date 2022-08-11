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
#![feature(extern_types, register_tool)]
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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn late_string(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
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
    pub u: C2RustUnnamed_0,
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
    pub u: C2RustUnnamed,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub union C2RustUnnamed_0 {
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
    pub hl: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
    pub hh: C2RustUnnamed_2,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
    pub graph: C2RustUnnamed_3,
    pub node: C2RustUnnamed_3,
    pub edge: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct rdata {
    pub nStepsToLeaf: uint64_t,
    pub subtreeSize: uint64_t,
    pub nChildren: uint64_t,
    pub nStepsToCenter: uint64_t,
    pub parent: *mut node_t,
    pub span: libc::c_double,
    pub theta: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue {
    pub head: *mut item_t,
    pub tail: *mut item_t,
}
pub type item_t = item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item_s {
    pub p: *mut libc::c_void,
    pub s: *mut item_s,
}
pub const _ISspace: C2RustUnnamed_8 = 8192;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_8 = 8;
pub const _ISpunct: C2RustUnnamed_8 = 4;
pub const _IScntrl: C2RustUnnamed_8 = 2;
pub const _ISblank: C2RustUnnamed_8 = 1;
pub const _ISgraph: C2RustUnnamed_8 = 32768;
pub const _ISprint: C2RustUnnamed_8 = 16384;
pub const _ISxdigit: C2RustUnnamed_8 = 4096;
pub const _ISdigit: C2RustUnnamed_8 = 2048;
pub const _ISalpha: C2RustUnnamed_8 = 1024;
pub const _ISlower: C2RustUnnamed_8 = 512;
pub const _ISupper: C2RustUnnamed_8 = 256;
unsafe extern "C" fn setNStepsToLeaf(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut prev: *mut Agnode_t,
) {
    let mut next: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut nsteps: uint64_t =
        ((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).nStepsToLeaf)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut ep: *mut Agedge_t = agfstedge(g, n);
    while !ep.is_null() {
        next = (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            ep
        } else {
            ep.offset(1 as libc::c_int as isize)
        }))
        .node;
        if next == n {
            next =
                (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                })
                .node;
        }
        if !(prev == next) {
            if nsteps
                < (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                    .nStepsToLeaf
            {
                (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                    .nStepsToLeaf = nsteps;
                setNStepsToLeaf(g, next, n);
            }
        }
        ep = agnxtedge(g, ep, n);
    }
}
unsafe extern "C" fn isLeaf(mut g: *mut Agraph_t, mut n: *mut Agnode_t) -> bool {
    let mut neighp: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ep: *mut Agedge_t = agfstedge(g, n);
    while !ep.is_null() {
        np = (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            ep
        } else {
            ep.offset(1 as libc::c_int as isize)
        }))
        .node;
        if np == n {
            np = (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                ep
            } else {
                ep.offset(-(1 as libc::c_int as isize))
            })
            .node;
        }
        if !(n == np) {
            if !neighp.is_null() {
                if neighp != np {
                    return 0 as libc::c_int != 0;
                }
            } else {
                neighp = np;
            }
        }
        ep = agnxtedge(g, ep, n);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn initLayout(mut g: *mut Agraph_t) {
    let mut nnodes: libc::c_int = agnnodes(g);
    let mut INF: libc::c_int = nnodes * nnodes;
    let mut n: *mut Agnode_t = agfstnode(g);
    while !n.is_null() {
        (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
            .nStepsToCenter = INF as uint64_t;
        (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).theta =
            10.00f64;
        if isLeaf(g, n) {
            (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                .nStepsToLeaf = 0 as libc::c_int as uint64_t;
        } else {
            (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                .nStepsToLeaf = INF as uint64_t;
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn findCenterNode(mut g: *mut Agraph_t) -> *mut Agnode_t {
    let mut center: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut maxNStepsToLeaf: uint64_t = 0 as libc::c_int as uint64_t;
    if agnnodes(g) <= 2 as libc::c_int {
        return agfstnode(g);
    }
    let mut n: *mut Agnode_t = agfstnode(g);
    while !n.is_null() {
        if (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).nStepsToLeaf
            == 0 as libc::c_int as libc::c_ulong
        {
            setNStepsToLeaf(g, n, 0 as *mut Agnode_t);
        }
        n = agnxtnode(g, n);
    }
    let mut n_0: *mut Agnode_t = agfstnode(g);
    while !n_0.is_null() {
        if (*((*((*(n_0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
            .nStepsToLeaf
            > maxNStepsToLeaf
        {
            maxNStepsToLeaf = (*((*((*(n_0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                as *mut rdata))
                .nStepsToLeaf;
            center = n_0;
        }
        n_0 = agnxtnode(g, n_0);
    }
    return center;
}
unsafe extern "C" fn push(mut q: *mut queue, mut p: *mut libc::c_void) {
    let mut ip: *mut item_t =
        zmalloc(::std::mem::size_of::<item_t>() as libc::c_ulong) as *mut item_t;
    let ref mut fresh0 = (*ip).p;
    *fresh0 = p;
    if !((*q).tail).is_null() {
        let ref mut fresh1 = (*(*q).tail).s;
        *fresh1 = ip;
        let ref mut fresh2 = (*q).tail;
        *fresh2 = ip;
    } else {
        let ref mut fresh3 = (*q).head;
        *fresh3 = ip;
        let ref mut fresh4 = (*q).tail;
        *fresh4 = *fresh3;
    };
}
unsafe extern "C" fn pull(mut q: *mut queue) -> *mut libc::c_void {
    let mut ip: *mut item_t = 0 as *mut item_t;
    ip = (*q).head;
    if !ip.is_null() {
        let mut p: *mut libc::c_void = (*ip).p;
        let ref mut fresh5 = (*q).head;
        *fresh5 = (*ip).s;
        free(ip as *mut libc::c_void);
        if ((*q).head).is_null() {
            let ref mut fresh6 = (*q).tail;
            *fresh6 = 0 as *mut item_t;
        }
        return p;
    } else {
        return 0 as *mut libc::c_void;
    };
}
unsafe extern "C" fn setNStepsToCenter(mut g: *mut Agraph_t, mut n: *mut Agnode_t) {
    let mut next: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut wt: *mut Agsym_t = agattr(
        g,
        2 as libc::c_int,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let mut qd: queue = {
        let mut init = queue {
            head: 0 as *mut item_t,
            tail: 0 as *mut item_t,
        };
        init
    };
    let mut q: *mut queue = &mut qd;
    push(q, n as *mut libc::c_void);
    loop {
        n = pull(q) as *mut Agnode_t;
        if n.is_null() {
            break;
        }
        let mut nsteps: uint64_t = ((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
            as *mut rdata))
            .nStepsToCenter)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut ep: *mut Agedge_t = agfstedge(g, n);
        while !ep.is_null() {
            if !(!wt.is_null()
                && strcmp(
                    agxget(ep as *mut libc::c_void, wt),
                    b"0\0" as *const u8 as *const libc::c_char,
                ) == 0)
            {
                next = (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(1 as libc::c_int as isize)
                }))
                .node;
                if next == n {
                    next = (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(-(1 as libc::c_int as isize))
                    })
                    .node;
                }
                if nsteps
                    < (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                        as *mut rdata))
                        .nStepsToCenter
                {
                    (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                        as *mut rdata))
                        .nStepsToCenter = nsteps;
                    let ref mut fresh7 =
                        (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                            as *mut rdata))
                            .parent;
                    *fresh7 = n;
                    let ref mut fresh8 = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .alg as *mut rdata))
                        .nChildren;
                    *fresh8 = (*fresh8).wrapping_add(1);
                    push(q, next as *mut libc::c_void);
                }
            }
            ep = agnxtedge(g, ep, n);
        }
    }
}
unsafe extern "C" fn setParentNodes(mut sg: *mut Agraph_t, mut center: *mut Agnode_t) -> uint64_t {
    let mut maxn: uint64_t = 0 as libc::c_int as uint64_t;
    let mut unset: uint64_t = (*((*((*(center as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
        as *mut rdata))
        .nStepsToCenter;
    (*((*((*(center as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
        .nStepsToCenter = 0 as libc::c_int as uint64_t;
    let ref mut fresh9 =
        (*((*((*(center as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).parent;
    *fresh9 = 0 as *mut node_t;
    setNStepsToCenter(sg, center);
    let mut n: *mut Agnode_t = agfstnode(sg);
    while !n.is_null() {
        if (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
            .nStepsToCenter
            == unset
        {
            return 18446744073709551615 as libc::c_ulong;
        } else {
            if (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                .nStepsToCenter
                > maxn
            {
                maxn = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                    .nStepsToCenter;
            }
        }
        n = agnxtnode(sg, n);
    }
    return maxn;
}
unsafe extern "C" fn setSubtreeSize(mut g: *mut Agraph_t) {
    let mut n: *mut Agnode_t = agfstnode(g);
    while !n.is_null() {
        if !((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).nChildren
            > 0 as libc::c_int as libc::c_ulong)
        {
            let ref mut fresh10 = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                as *mut rdata))
                .subtreeSize;
            *fresh10 = (*fresh10).wrapping_add(1);
            let mut parent: *mut Agnode_t =
                (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).parent;
            while !parent.is_null() {
                let ref mut fresh11 = (*((*((*(parent as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .alg as *mut rdata))
                    .subtreeSize;
                *fresh11 = (*fresh11).wrapping_add(1);
                parent = (*((*((*(parent as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                    as *mut rdata))
                    .parent;
            }
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn setChildSubtreeSpans(mut g: *mut Agraph_t, mut n: *mut Agnode_t) {
    let mut next: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ratio: libc::c_double =
        (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).span
            / (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                .subtreeSize as libc::c_double;
    let mut ep: *mut Agedge_t = agfstedge(g, n);
    while !ep.is_null() {
        next = (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            ep
        } else {
            ep.offset(1 as libc::c_int as isize)
        }))
        .node;
        if next == n {
            next =
                (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                })
                .node;
        }
        if !((*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).parent
            != n)
        {
            if !((*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                .span
                != 0.0f64)
            {
                (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                    .span = ratio
                    * (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                        as *mut rdata))
                        .subtreeSize as libc::c_double;
                if (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                    .nChildren
                    > 0 as libc::c_int as libc::c_ulong
                {
                    setChildSubtreeSpans(g, next);
                }
            }
        }
        ep = agnxtedge(g, ep, n);
    }
}
unsafe extern "C" fn setSubtreeSpans(mut sg: *mut Agraph_t, mut center: *mut Agnode_t) {
    (*((*((*(center as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).span =
        2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
    setChildSubtreeSpans(sg, center);
}
unsafe extern "C" fn setChildPositions(mut sg: *mut Agraph_t, mut n: *mut Agnode_t) {
    let mut next: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut theta: libc::c_double = 0.;
    if ((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).parent)
        .is_null()
    {
        theta = 0 as libc::c_int as libc::c_double;
    } else {
        theta = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).theta
            - (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).span
                / 2 as libc::c_int as libc::c_double;
    }
    let mut ep: *mut Agedge_t = agfstedge(sg, n);
    while !ep.is_null() {
        next = (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            ep
        } else {
            ep.offset(1 as libc::c_int as isize)
        }))
        .node;
        if next == n {
            next =
                (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                })
                .node;
        }
        if !((*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).parent
            != n)
        {
            if !((*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                .theta
                != 10.00f64)
            {
                (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                    .theta = theta
                    + (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                        as *mut rdata))
                        .span
                        / 2.0f64;
                theta += (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                    as *mut rdata))
                    .span;
                if (*((*((*(next as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                    .nChildren
                    > 0 as libc::c_int as libc::c_ulong
                {
                    setChildPositions(sg, next);
                }
            }
        }
        ep = agnxtedge(sg, ep, n);
    }
}
unsafe extern "C" fn setPositions(mut sg: *mut Agraph_t, mut center: *mut Agnode_t) {
    (*((*((*(center as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).theta =
        0 as libc::c_int as libc::c_double;
    setChildPositions(sg, center);
}
unsafe extern "C" fn getRankseps(
    mut g: *mut Agraph_t,
    mut maxrank: uint64_t,
) -> *mut libc::c_double {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut rk: uint64_t = 1 as libc::c_int as uint64_t;
    let mut ranks: *mut libc::c_double = gcalloc(
        maxrank.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut xf: libc::c_double = 0.0f64;
    let mut delx: libc::c_double = 0.0f64;
    let mut d: libc::c_double = 0.;
    p = late_string(
        g as *mut libc::c_void,
        agattr(
            (*g).root,
            0 as libc::c_int,
            b"ranksep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        0 as *mut libc::c_char,
    );
    if !p.is_null() {
        while rk <= maxrank && {
            d = strtod(p, &mut endp);
            d > 0 as libc::c_int as libc::c_double
        } {
            delx = fmax(d, 0.02f64);
            xf += delx;
            let fresh12 = rk;
            rk = rk.wrapping_add(1);
            *ranks.offset(fresh12 as isize) = xf;
            p = endp;
            loop {
                c = *p;
                if !(c as libc::c_int != 0
                    && (*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                        || c as libc::c_int == ':' as i32))
                {
                    break;
                }
                p = p.offset(1);
            }
        }
    } else {
        delx = 1.00f64;
    }
    let mut i: uint64_t = rk;
    while i <= maxrank {
        xf += delx;
        *ranks.offset(i as isize) = xf;
        i = i.wrapping_add(1);
    }
    return ranks;
}
unsafe extern "C" fn setAbsolutePos(mut g: *mut Agraph_t, mut maxrank: uint64_t) {
    let mut ranksep: *mut libc::c_double = getRankseps(g, maxrank);
    if Verbose != 0 {
        fputs(
            b"Rank separation = \0" as *const u8 as *const libc::c_char,
            stderr,
        );
        let mut i: uint64_t = 0 as libc::c_int as uint64_t;
        while i <= maxrank {
            fprintf(
                stderr,
                b"%.03lf \0" as *const u8 as *const libc::c_char,
                *ranksep.offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
        fputs(b"\n\0" as *const u8 as *const libc::c_char, stderr);
    }
    let mut n: *mut Agnode_t = agfstnode(g);
    while !n.is_null() {
        let mut hyp: libc::c_double = *ranksep.offset(
            (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata))
                .nStepsToCenter as isize,
        );
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) = hyp
            * cos(
                (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).theta,
            );
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) = hyp
            * sin(
                (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut rdata)).theta,
            );
        n = agnxtnode(g, n);
    }
    free(ranksep as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn circleLayout(
    mut sg: *mut Agraph_t,
    mut center: *mut Agnode_t,
) -> *mut Agnode_t {
    if agnnodes(sg) == 1 as libc::c_int {
        let mut n: *mut Agnode_t = agfstnode(sg);
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        return center;
    }
    initLayout(sg);
    if center.is_null() {
        center = findCenterNode(sg);
    }
    let mut maxNStepsToCenter: uint64_t = setParentNodes(sg, center);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"root = %s max steps to root = %lu\n\0" as *const u8 as *const libc::c_char,
            agnameof(center as *mut libc::c_void),
            maxNStepsToCenter,
        );
    }
    if maxNStepsToCenter == 18446744073709551615 as libc::c_ulong {
        agerr(
            AGERR,
            b"twopi: use of weight=0 creates disconnected component.\n\0" as *const u8
                as *const libc::c_char,
        );
        return center;
    }
    setSubtreeSize(sg);
    setSubtreeSpans(sg, center);
    setPositions(sg, center);
    setAbsolutePos(sg, maxNStepsToCenter);
    return center;
}
