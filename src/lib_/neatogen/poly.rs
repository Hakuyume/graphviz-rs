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
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn shapeOf(_: *mut node_t) -> shape_kind;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut origin: Point;
    fn intersection(a: Point, b: Point, c: Point, d: Point, p: *mut Point) -> libc::c_int;
    fn subpt(a: *mut Point, b: Point, c: Point);
    fn addpt(a: *mut Point, b: Point, c: Point);
    fn area_2(a: Point, b: Point, c: Point) -> libc::c_double;
    fn leftOf(a: Point, b: Point, c: Point) -> libc::c_int;
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
pub type shape_kind = libc::c_uint;
pub const SH_EPSF: shape_kind = 4;
pub const SH_POINT: shape_kind = 3;
pub const SH_RECORD: shape_kind = 2;
pub const SH_POLY: shape_kind = 1;
pub const SH_UNSET: shape_kind = 0;
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
pub struct field_t {
    pub size: pointf,
    pub b: boxf,
    pub n_flds: libc::c_int,
    pub lp: *mut textlabel_t,
    pub fld: *mut *mut field_t,
    pub id: *mut libc::c_char,
    pub LR: libc::c_uchar,
    pub sides: libc::c_uchar,
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
pub type Point = pointf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Poly {
    pub origin: Point,
    pub corner: Point,
    pub nverts: libc::c_int,
    pub verts: *mut Point,
    pub kind: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut maxcnt: libc::c_int = 0 as libc::c_int;
static mut tp1: *mut Point = 0 as *const Point as *mut Point;
static mut tp2: *mut Point = 0 as *const Point as *mut Point;
static mut tp3: *mut Point = 0 as *const Point as *mut Point;
#[no_mangle]
pub unsafe extern "C" fn polyFree() {
    maxcnt = 0 as libc::c_int;
    free(tp1 as *mut libc::c_void);
    free(tp2 as *mut libc::c_void);
    free(tp3 as *mut libc::c_void);
    tp1 = 0 as *mut Point;
    tp2 = 0 as *mut Point;
    tp3 = 0 as *mut Point;
}
#[no_mangle]
pub unsafe extern "C" fn breakPoly(mut pp: *mut Poly) {
    free((*pp).verts as *mut libc::c_void);
}
unsafe extern "C" fn bbox(
    mut verts: *mut Point,
    mut cnt: libc::c_int,
    mut o: *mut Point,
    mut c: *mut Point,
) {
    let mut xmin: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    xmax = (*verts).x;
    xmin = xmax;
    ymax = (*verts).y;
    ymin = ymax;
    i = 1 as libc::c_int;
    while i < cnt {
        verts = verts.offset(1);
        if (*verts).x < xmin {
            xmin = (*verts).x;
        }
        if (*verts).y < ymin {
            ymin = (*verts).y;
        }
        if (*verts).x > xmax {
            xmax = (*verts).x;
        }
        if (*verts).y > ymax {
            ymax = (*verts).y;
        }
        i += 1;
    }
    (*o).x = xmin;
    (*o).y = ymin;
    (*c).x = xmax;
    (*c).y = ymax;
}
unsafe extern "C" fn inflatePts(
    mut verts: *mut Point,
    mut cnt: libc::c_int,
    mut xmargin: libc::c_float,
    mut ymargin: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut cur: *mut Point = 0 as *mut Point;
    cur = &mut *verts.offset(0 as libc::c_int as isize) as *mut Point;
    i = 0 as libc::c_int;
    while i < cnt {
        (*cur).x *= xmargin as libc::c_double;
        (*cur).y *= ymargin as libc::c_double;
        cur = cur.offset(1);
        i += 1;
    }
}
unsafe extern "C" fn isBox(mut verts: *mut Point, mut cnt: libc::c_int) -> libc::c_int {
    if cnt != 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*verts.offset(0 as libc::c_int as isize)).y == (*verts.offset(1 as libc::c_int as isize)).y
    {
        return ((*verts.offset(2 as libc::c_int as isize)).y
            == (*verts.offset(3 as libc::c_int as isize)).y
            && (*verts.offset(0 as libc::c_int as isize)).x
                == (*verts.offset(3 as libc::c_int as isize)).x
            && (*verts.offset(1 as libc::c_int as isize)).x
                == (*verts.offset(2 as libc::c_int as isize)).x) as libc::c_int;
    } else {
        return ((*verts.offset(0 as libc::c_int as isize)).x
            == (*verts.offset(1 as libc::c_int as isize)).x
            && (*verts.offset(2 as libc::c_int as isize)).x
                == (*verts.offset(3 as libc::c_int as isize)).x
            && (*verts.offset(0 as libc::c_int as isize)).y
                == (*verts.offset(3 as libc::c_int as isize)).y
            && (*verts.offset(1 as libc::c_int as isize)).y
                == (*verts.offset(2 as libc::c_int as isize)).y) as libc::c_int;
    };
}
unsafe extern "C" fn makeScaledTransPoint(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut dx: libc::c_float,
    mut dy: libc::c_float,
) -> Point {
    let mut rv: Point = Point { x: 0., y: 0. };
    rv.x = x as libc::c_double / 72 as libc::c_int as libc::c_double + dx as libc::c_double;
    rv.y = y as libc::c_double / 72 as libc::c_int as libc::c_double + dy as libc::c_double;
    return rv;
}
unsafe extern "C" fn makeScaledPoint(mut x: libc::c_double, mut y: libc::c_double) -> Point {
    let mut rv: Point = Point { x: 0., y: 0. };
    rv.x = x / 72 as libc::c_int as libc::c_double;
    rv.y = y / 72 as libc::c_int as libc::c_double;
    return rv;
}
unsafe extern "C" fn genRound(
    mut n: *mut Agnode_t,
    mut sidep: *mut libc::c_int,
    mut xm: libc::c_float,
    mut ym: libc::c_float,
) -> *mut Point {
    let mut sides: libc::c_int = 0 as libc::c_int;
    let mut verts: *mut Point = 0 as *mut Point;
    let mut p: *mut libc::c_char = agget(
        n as *mut libc::c_void,
        b"samplepoints\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut i: libc::c_int = 0;
    if !p.is_null() {
        sides = atoi(p);
    }
    if sides < 3 as libc::c_int {
        sides = 20 as libc::c_int;
    }
    verts = gcalloc(
        sides as size_t,
        ::std::mem::size_of::<Point>() as libc::c_ulong,
    ) as *mut Point;
    i = 0 as libc::c_int;
    while i < sides {
        (*verts.offset(i as isize)).x =
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64
                + xm as libc::c_double)
                * cos(i as libc::c_double / sides as libc::c_double
                    * 3.14159265358979323846f64
                    * 2.0f64);
        (*verts.offset(i as isize)).y =
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64
                + ym as libc::c_double)
                * sin(i as libc::c_double / sides as libc::c_double
                    * 3.14159265358979323846f64
                    * 2.0f64);
        i += 1;
    }
    *sidep = sides;
    return verts;
}
#[no_mangle]
pub unsafe extern "C" fn makeAddPoly(
    mut pp: *mut Poly,
    mut n: *mut Agnode_t,
    mut xmargin: libc::c_float,
    mut ymargin: libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut verts: *mut Point = 0 as *mut Point;
    let mut poly: *mut polygon_t = 0 as *mut polygon_t;
    let mut b: boxf = boxf {
        LL: Point { x: 0., y: 0. },
        UR: Point { x: 0., y: 0. },
    };
    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null() {
        let mut b_0: Point = Point { x: 0., y: 0. };
        sides = 4 as libc::c_int;
        b_0.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64
            + xmargin as libc::c_double;
        b_0.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64
            + ymargin as libc::c_double;
        (*pp).kind = 1 as libc::c_int;
        verts = gcalloc(
            sides as size_t,
            ::std::mem::size_of::<Point>() as libc::c_ulong,
        ) as *mut Point;
        (*verts.offset(0 as libc::c_int as isize)).x = b_0.x;
        (*verts.offset(0 as libc::c_int as isize)).y = b_0.y;
        (*verts.offset(1 as libc::c_int as isize)).x = -b_0.x;
        (*verts.offset(1 as libc::c_int as isize)).y = b_0.y;
        (*verts.offset(2 as libc::c_int as isize)).x = -b_0.x;
        (*verts.offset(2 as libc::c_int as isize)).y = -b_0.y;
        (*verts.offset(3 as libc::c_int as isize)).x = b_0.x;
        (*verts.offset(3 as libc::c_int as isize)).y = -b_0.y;
    } else {
        match shapeOf(n) as libc::c_uint {
            1 => {
                poly = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
                    as *mut polygon_t;
                sides = (*poly).sides;
                if strcmp(
                    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
                    b"box\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    (*pp).kind = 1 as libc::c_int;
                } else if strcmp(
                    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
                    b"polygon\0" as *const u8 as *const libc::c_char,
                ) == 0
                    && isBox((*poly).vertices, sides) != 0
                {
                    (*pp).kind = 1 as libc::c_int;
                } else if (*poly).sides < 3 as libc::c_int && (*poly).regular != 0 {
                    (*pp).kind = 2 as libc::c_int;
                } else {
                    (*pp).kind = 0 as libc::c_int;
                }
                if sides >= 3 as libc::c_int {
                    verts = gcalloc(
                        sides as size_t,
                        ::std::mem::size_of::<Point>() as libc::c_ulong,
                    ) as *mut Point;
                    if (*pp).kind == 1 as libc::c_int {
                        (*verts.offset(0 as libc::c_int as isize)).x =
                            (*((*poly).vertices).offset(0 as libc::c_int as isize)).x
                                / 72 as libc::c_int as libc::c_double
                                + xmargin as libc::c_double;
                        (*verts.offset(0 as libc::c_int as isize)).y =
                            (*((*poly).vertices).offset(0 as libc::c_int as isize)).y
                                / 72 as libc::c_int as libc::c_double
                                + ymargin as libc::c_double;
                        (*verts.offset(1 as libc::c_int as isize)).x =
                            (*((*poly).vertices).offset(1 as libc::c_int as isize)).x
                                / 72 as libc::c_int as libc::c_double
                                - xmargin as libc::c_double;
                        (*verts.offset(1 as libc::c_int as isize)).y =
                            (*((*poly).vertices).offset(1 as libc::c_int as isize)).y
                                / 72 as libc::c_int as libc::c_double
                                + ymargin as libc::c_double;
                        (*verts.offset(2 as libc::c_int as isize)).x =
                            (*((*poly).vertices).offset(2 as libc::c_int as isize)).x
                                / 72 as libc::c_int as libc::c_double
                                - xmargin as libc::c_double;
                        (*verts.offset(2 as libc::c_int as isize)).y =
                            (*((*poly).vertices).offset(2 as libc::c_int as isize)).y
                                / 72 as libc::c_int as libc::c_double
                                - ymargin as libc::c_double;
                        (*verts.offset(3 as libc::c_int as isize)).x =
                            (*((*poly).vertices).offset(3 as libc::c_int as isize)).x
                                / 72 as libc::c_int as libc::c_double
                                + xmargin as libc::c_double;
                        (*verts.offset(3 as libc::c_int as isize)).y =
                            (*((*poly).vertices).offset(3 as libc::c_int as isize)).y
                                / 72 as libc::c_int as libc::c_double
                                - ymargin as libc::c_double;
                    } else {
                        i = 0 as libc::c_int;
                        while i < sides {
                            let mut h: libc::c_double = sqrt(
                                (*((*poly).vertices).offset(i as isize)).x
                                    * (*((*poly).vertices).offset(i as isize)).x
                                    + (*((*poly).vertices).offset(i as isize)).y
                                        * (*((*poly).vertices).offset(i as isize)).y,
                            );
                            (*verts.offset(i as isize)).x =
                                (*((*poly).vertices).offset(i as isize)).x
                                    * (1.0f64 + xmargin as libc::c_double / h);
                            (*verts.offset(i as isize)).y =
                                (*((*poly).vertices).offset(i as isize)).y
                                    * (1.0f64 + ymargin as libc::c_double / h);
                            (*verts.offset(i as isize)).x =
                                (*verts.offset(i as isize)).x / 72 as libc::c_int as libc::c_double;
                            (*verts.offset(i as isize)).y =
                                (*verts.offset(i as isize)).y / 72 as libc::c_int as libc::c_double;
                            i += 1;
                        }
                    }
                } else {
                    verts = genRound(n, &mut sides, xmargin, ymargin);
                }
            }
            2 => {
                sides = 4 as libc::c_int;
                verts = gcalloc(
                    sides as size_t,
                    ::std::mem::size_of::<Point>() as libc::c_ulong,
                ) as *mut Point;
                b = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
                    as *mut field_t))
                    .b;
                *verts.offset(0 as libc::c_int as isize) = makeScaledTransPoint(
                    b.LL.x as libc::c_int,
                    b.LL.y as libc::c_int,
                    -xmargin,
                    -ymargin,
                );
                *verts.offset(1 as libc::c_int as isize) = makeScaledTransPoint(
                    b.UR.x as libc::c_int,
                    b.LL.y as libc::c_int,
                    xmargin,
                    -ymargin,
                );
                *verts.offset(2 as libc::c_int as isize) = makeScaledTransPoint(
                    b.UR.x as libc::c_int,
                    b.UR.y as libc::c_int,
                    xmargin,
                    ymargin,
                );
                *verts.offset(3 as libc::c_int as isize) = makeScaledTransPoint(
                    b.LL.x as libc::c_int,
                    b.UR.y as libc::c_int,
                    -xmargin,
                    ymargin,
                );
                (*pp).kind = 1 as libc::c_int;
            }
            3 => {
                (*pp).kind = 2 as libc::c_int;
                verts = genRound(n, &mut sides, xmargin, ymargin);
            }
            _ => {
                agerr(
                    AGERR,
                    b"makeAddPoly: unknown shape type %s\n\0" as *const u8 as *const libc::c_char,
                    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
                );
                return 1 as libc::c_int;
            }
        }
    }
    let ref mut fresh0 = (*pp).verts;
    *fresh0 = verts;
    (*pp).nverts = sides;
    bbox(verts, sides, &mut (*pp).origin, &mut (*pp).corner);
    if sides > maxcnt {
        maxcnt = sides;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn makePoly(
    mut pp: *mut Poly,
    mut n: *mut Agnode_t,
    mut xmargin: libc::c_float,
    mut ymargin: libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut verts: *mut Point = 0 as *mut Point;
    let mut poly: *mut polygon_t = 0 as *mut polygon_t;
    let mut b: boxf = boxf {
        LL: Point { x: 0., y: 0. },
        UR: Point { x: 0., y: 0. },
    };
    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null() {
        let mut b_0: Point = Point { x: 0., y: 0. };
        sides = 4 as libc::c_int;
        b_0.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64;
        b_0.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64;
        (*pp).kind = 1 as libc::c_int;
        verts = gcalloc(
            sides as size_t,
            ::std::mem::size_of::<Point>() as libc::c_ulong,
        ) as *mut Point;
        (*verts.offset(0 as libc::c_int as isize)).x = b_0.x;
        (*verts.offset(0 as libc::c_int as isize)).y = b_0.y;
        (*verts.offset(1 as libc::c_int as isize)).x = -b_0.x;
        (*verts.offset(1 as libc::c_int as isize)).y = b_0.y;
        (*verts.offset(2 as libc::c_int as isize)).x = -b_0.x;
        (*verts.offset(2 as libc::c_int as isize)).y = -b_0.y;
        (*verts.offset(3 as libc::c_int as isize)).x = b_0.x;
        (*verts.offset(3 as libc::c_int as isize)).y = -b_0.y;
    } else {
        match shapeOf(n) as libc::c_uint {
            1 => {
                poly = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
                    as *mut polygon_t;
                sides = (*poly).sides;
                if sides >= 3 as libc::c_int {
                    verts = gcalloc(
                        sides as size_t,
                        ::std::mem::size_of::<Point>() as libc::c_ulong,
                    ) as *mut Point;
                    i = 0 as libc::c_int;
                    while i < sides {
                        (*verts.offset(i as isize)).x = (*((*poly).vertices).offset(i as isize)).x
                            / 72 as libc::c_int as libc::c_double;
                        (*verts.offset(i as isize)).y = (*((*poly).vertices).offset(i as isize)).y
                            / 72 as libc::c_int as libc::c_double;
                        i += 1;
                    }
                } else {
                    verts = genRound(
                        n,
                        &mut sides,
                        0 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
                if strcmp(
                    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
                    b"box\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    (*pp).kind = 1 as libc::c_int;
                } else if strcmp(
                    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
                    b"polygon\0" as *const u8 as *const libc::c_char,
                ) == 0
                    && isBox(verts, sides) != 0
                {
                    (*pp).kind = 1 as libc::c_int;
                } else if (*poly).sides < 3 as libc::c_int && (*poly).regular != 0 {
                    (*pp).kind = 2 as libc::c_int;
                } else {
                    (*pp).kind = 0 as libc::c_int;
                }
            }
            2 => {
                sides = 4 as libc::c_int;
                verts = gcalloc(
                    sides as size_t,
                    ::std::mem::size_of::<Point>() as libc::c_ulong,
                ) as *mut Point;
                b = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
                    as *mut field_t))
                    .b;
                *verts.offset(0 as libc::c_int as isize) = makeScaledPoint(b.LL.x, b.LL.y);
                *verts.offset(1 as libc::c_int as isize) = makeScaledPoint(b.UR.x, b.LL.y);
                *verts.offset(2 as libc::c_int as isize) = makeScaledPoint(b.UR.x, b.UR.y);
                *verts.offset(3 as libc::c_int as isize) = makeScaledPoint(b.LL.x, b.UR.y);
                (*pp).kind = 1 as libc::c_int;
            }
            3 => {
                (*pp).kind = 2 as libc::c_int;
                verts = genRound(
                    n,
                    &mut sides,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
            _ => {
                agerr(
                    AGERR,
                    b"makePoly: unknown shape type %s\n\0" as *const u8 as *const libc::c_char,
                    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
                );
                return 1 as libc::c_int;
            }
        }
    }
    if xmargin as libc::c_double != 1.0f64 || ymargin as libc::c_double != 1.0f64 {
        inflatePts(verts, sides, xmargin, ymargin);
    }
    let ref mut fresh1 = (*pp).verts;
    *fresh1 = verts;
    (*pp).nverts = sides;
    bbox(verts, sides, &mut (*pp).origin, &mut (*pp).corner);
    if sides > maxcnt {
        maxcnt = sides;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pintersect(
    mut originp: Point,
    mut cornerp: Point,
    mut originq: Point,
    mut cornerq: Point,
) -> libc::c_int {
    return (originp.x <= cornerq.x
        && originq.x <= cornerp.x
        && originp.y <= cornerq.y
        && originq.y <= cornerp.y) as libc::c_int;
}
unsafe extern "C" fn edgesIntersect(
    mut P: *mut Point,
    mut Q: *mut Point,
    mut n: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut aa: libc::c_int = 0 as libc::c_int;
    let mut ba: libc::c_int = 0 as libc::c_int;
    let mut a1: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut A: Point = Point { x: 0., y: 0. };
    let mut B: Point = Point { x: 0., y: 0. };
    let mut cross: libc::c_double = 0.;
    let mut bHA: libc::c_int = 0;
    let mut aHB: libc::c_int = 0;
    let mut p: Point = Point { x: 0., y: 0. };
    let mut inflag: libc::c_int = 0 as libc::c_int;
    loop {
        a1 = (a + n - 1 as libc::c_int) % n;
        b1 = (b + m - 1 as libc::c_int) % m;
        subpt(&mut A, *P.offset(a as isize), *P.offset(a1 as isize));
        subpt(&mut B, *Q.offset(b as isize), *Q.offset(b1 as isize));
        cross = area_2(origin, A, B);
        bHA = leftOf(
            *P.offset(a1 as isize),
            *P.offset(a as isize),
            *Q.offset(b as isize),
        );
        aHB = leftOf(
            *Q.offset(b1 as isize),
            *Q.offset(b as isize),
            *P.offset(a as isize),
        );
        if intersection(
            *P.offset(a1 as isize),
            *P.offset(a as isize),
            *Q.offset(b1 as isize),
            *Q.offset(b as isize),
            &mut p,
        ) != 0
        {
            return 1 as libc::c_int;
        }
        if cross == 0 as libc::c_int as libc::c_double && bHA == 0 && aHB == 0 {
            if inflag == 1 as libc::c_int {
                ba += 1;
                b = (b + 1 as libc::c_int) % m;
            } else {
                aa += 1;
                a = (a + 1 as libc::c_int) % n;
            }
        } else if cross >= 0 as libc::c_int as libc::c_double {
            if bHA != 0 {
                aa += 1;
                a = (a + 1 as libc::c_int) % n;
            } else {
                ba += 1;
                b = (b + 1 as libc::c_int) % m;
            }
        } else if aHB != 0 {
            ba += 1;
            b = (b + 1 as libc::c_int) % m;
        } else {
            aa += 1;
            a = (a + 1 as libc::c_int) % n;
        }
        if !((aa < n || ba < m) && aa < 2 as libc::c_int * n && ba < 2 as libc::c_int * m) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn inPoly(
    mut vertex: *mut Point,
    mut n: libc::c_int,
    mut q: Point,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    let mut crossings: libc::c_double = 0 as libc::c_int as libc::c_double;
    if tp3.is_null() {
        tp3 = gcalloc(
            maxcnt as size_t,
            ::std::mem::size_of::<Point>() as libc::c_ulong,
        ) as *mut Point;
    }
    i = 0 as libc::c_int;
    while i < n {
        (*tp3.offset(i as isize)).x = (*vertex.offset(i as isize)).x - q.x;
        (*tp3.offset(i as isize)).y = (*vertex.offset(i as isize)).y - q.y;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        i1 = (i + n - 1 as libc::c_int) % n;
        if (*tp3.offset(i as isize)).y == 0 as libc::c_int as libc::c_double
            && (*tp3.offset(i1 as isize)).y == 0 as libc::c_int as libc::c_double
        {
            if (*tp3.offset(i as isize)).x * (*tp3.offset(i1 as isize)).x
                < 0 as libc::c_int as libc::c_double
            {
                return 1 as libc::c_int;
            }
        } else if (*tp3.offset(i as isize)).y >= 0 as libc::c_int as libc::c_double
            && (*tp3.offset(i1 as isize)).y <= 0 as libc::c_int as libc::c_double
            || (*tp3.offset(i1 as isize)).y >= 0 as libc::c_int as libc::c_double
                && (*tp3.offset(i as isize)).y <= 0 as libc::c_int as libc::c_double
        {
            x = ((*tp3.offset(i as isize)).x * (*tp3.offset(i1 as isize)).y
                - (*tp3.offset(i1 as isize)).x * (*tp3.offset(i as isize)).y)
                / ((*tp3.offset(i1 as isize)).y - (*tp3.offset(i as isize)).y);
            if x == 0 as libc::c_int as libc::c_double {
                return 1 as libc::c_int;
            }
            if x > 0 as libc::c_int as libc::c_double {
                if (*tp3.offset(i as isize)).y == 0 as libc::c_int as libc::c_double
                    || (*tp3.offset(i1 as isize)).y == 0 as libc::c_int as libc::c_double
                {
                    crossings += 0.5f64;
                } else {
                    crossings += 1.0f64;
                }
            }
        }
        i += 1;
    }
    if crossings as libc::c_int % 2 as libc::c_int == 1 as libc::c_int {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn inBox(mut p: Point, mut origin_0: Point, mut corner: Point) -> libc::c_int {
    return (p.x <= corner.x && p.x >= origin_0.x && p.y <= corner.y && p.y >= origin_0.y)
        as libc::c_int;
}
unsafe extern "C" fn transCopy(
    mut inp: *mut Point,
    mut cnt: libc::c_int,
    mut off: Point,
    mut outp: *mut Point,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < cnt {
        (*outp).x = (*inp).x + off.x;
        (*outp).y = (*inp).y + off.y;
        inp = inp.offset(1);
        outp = outp.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn polyOverlap(
    mut p: Point,
    mut pp: *mut Poly,
    mut q: Point,
    mut qp: *mut Poly,
) -> libc::c_int {
    let mut op: Point = Point { x: 0., y: 0. };
    let mut cp: Point = Point { x: 0., y: 0. };
    let mut oq: Point = Point { x: 0., y: 0. };
    let mut cq: Point = Point { x: 0., y: 0. };
    addpt(&mut op, p, (*pp).origin);
    addpt(&mut cp, p, (*pp).corner);
    addpt(&mut oq, q, (*qp).origin);
    addpt(&mut cq, q, (*qp).corner);
    if pintersect(op, cp, oq, cq) == 0 {
        return 0 as libc::c_int;
    }
    if (*pp).kind & 1 as libc::c_int != 0 && (*qp).kind & 1 as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    if (*pp).kind & 2 as libc::c_int != 0 && (*qp).kind & 2 as libc::c_int != 0 {
        let mut d: libc::c_double =
            (*pp).corner.x - (*pp).origin.x + (*qp).corner.x - (*qp).origin.x;
        let mut dx: libc::c_double = p.x - q.x;
        let mut dy: libc::c_double = p.y - q.y;
        if dx * dx + dy * dy > d * d / 4.0f64 {
            return 0 as libc::c_int;
        } else {
            return 1 as libc::c_int;
        }
    }
    if tp1.is_null() {
        tp1 = gcalloc(
            maxcnt as size_t,
            ::std::mem::size_of::<Point>() as libc::c_ulong,
        ) as *mut Point;
        tp2 = gcalloc(
            maxcnt as size_t,
            ::std::mem::size_of::<Point>() as libc::c_ulong,
        ) as *mut Point;
    }
    transCopy((*pp).verts, (*pp).nverts, p, tp1);
    transCopy((*qp).verts, (*qp).nverts, q, tp2);
    return (edgesIntersect(tp1, tp2, (*pp).nverts, (*qp).nverts) != 0
        || inBox(*tp1, oq, cq) != 0 && inPoly(tp2, (*qp).nverts, *tp1) != 0
        || inBox(*tp2, op, cp) != 0 && inPoly(tp1, (*pp).nverts, *tp2) != 0)
        as libc::c_int;
}
