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
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn abort() -> !;
    fn free(_: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn getsplinepoints(e: *mut edge_t) -> *mut splines;
    fn edgeMidpoint(g: *mut graph_t, e: *mut edge_t) -> pointf;
    static mut Verbose: libc::c_uchar;
    static mut Show_boxes: *mut *mut libc::c_char;
    static mut Concentrate: libc::c_uchar;
    static mut State: libc::c_int;
    static mut EdgeLabelsDone: libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn ccwrotatepf(p: pointf, ccwrot: libc::c_int) -> pointf;
    fn late_bool(_: *mut libc::c_void, _: *mut Agsym_t, _: bool) -> bool;
    fn updateBB(g: *mut graph_t, lp: *mut textlabel_t);
    fn gv_nodesize(n: *mut Agnode_t, flip: bool);
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn placeLabels(
        objs: *mut object_t,
        n_objs: libc::c_int,
        lbls: *mut xlabel_t,
        n_lbls: libc::c_int,
        params: *mut label_params_t,
    ) -> libc::c_int;
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
pub struct xlabel_t {
    pub sz: pointf,
    pub pos: pointf,
    pub lbl: *mut libc::c_void,
    pub set: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_t {
    pub pos: pointf,
    pub sz: pointf,
    pub lbl: *mut xlabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct label_params_t {
    pub bb: boxf,
    pub force: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cinfo_t {
    pub bb: boxf,
    pub objp: *mut object_t,
}
#[inline]
unsafe extern "C" fn pointfof(mut x: libc::c_double, mut y: libc::c_double) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = x;
    r.y = y;
    return r;
}
static mut Rankdir: libc::c_int = 0;
static mut Flip: bool = false;
static mut Offset: pointf = pointf { x: 0., y: 0. };
unsafe extern "C" fn map_point(mut p: pointf) -> pointf {
    p = ccwrotatepf(p, Rankdir * 90 as libc::c_int);
    p.x -= Offset.x;
    p.y -= Offset.y;
    return p;
}
unsafe extern "C" fn map_edge(mut e: *mut edge_t) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    if ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
        if Concentrate == 0
            && (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type as libc::c_int
                != 6 as libc::c_int
        {
            agerr(
                AGERR,
                b"lost %s %s edge\n\0" as *const u8 as *const libc::c_char,
                agnameof(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                    .node as *mut libc::c_void,
                ),
                agnameof(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                    .node as *mut libc::c_void,
                ),
            );
        }
        return;
    }
    j = 0 as libc::c_int;
    while j < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
        bz =
            *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list).offset(j as isize);
        k = 0 as libc::c_int;
        while k < bz.size {
            *(bz.list).offset(k as isize) = map_point(*(bz.list).offset(k as isize));
            k += 1;
        }
        if bz.sflag != 0 {
            (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                .offset(j as isize))
            .sp = map_point(
                (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                    .offset(j as isize))
                .sp,
            );
        }
        if bz.eflag != 0 {
            (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                .offset(j as isize))
            .ep = map_point(
                (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                    .offset(j as isize))
                .ep,
            );
        }
        j += 1;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos =
            map_point((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos);
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).pos =
            map_point((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).pos);
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).pos =
            map_point((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).pos);
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).pos =
            map_point((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).pos);
    }
}
#[no_mangle]
pub unsafe extern "C" fn translate_bb(mut g: *mut graph_t, mut rankdir: libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut new_bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    bb = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb;
    if rankdir == 1 as libc::c_int || rankdir == 2 as libc::c_int {
        new_bb.LL = map_point(pointfof(bb.LL.x, bb.UR.y));
        new_bb.UR = map_point(pointfof(bb.UR.x, bb.LL.y));
    } else {
        new_bb.LL = map_point(pointfof(bb.LL.x, bb.LL.y));
        new_bb.UR = map_point(pointfof(bb.UR.x, bb.UR.y));
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb = new_bb;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null() {
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos =
            map_point((*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos);
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        translate_bb(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize),
            rankdir,
        );
        c += 1;
    }
}
unsafe extern "C" fn translate_drawing(mut g: *mut graph_t) {
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut shift: bool = Offset.x != 0. || Offset.y != 0.;
    if !shift && Rankdir == 0 {
        return;
    }
    v = agfstnode(g);
    while !v.is_null() {
        if Rankdir != 0 {
            gv_nodesize(v, 0 as libc::c_int != 0);
        }
        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord =
            map_point((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord);
        if !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null() {
            (*(*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).pos =
                map_point((*(*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).pos);
        }
        if State == 1 as libc::c_int {
            e = agfstout(g, v);
            while !e.is_null() {
                map_edge(e);
                e = agnxtout(g, e);
            }
        }
        v = agnxtnode(g, v);
    }
    translate_bb(
        g,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir & 0x3 as libc::c_int,
    );
}
unsafe extern "C" fn place_root_label(mut g: *mut graph_t, mut d: pointf) {
    let mut p: pointf = pointf { x: 0., y: 0. };
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
        & 4 as libc::c_int
        != 0
    {
        p.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .bb
            .UR
            .x
            - d.x / 2 as libc::c_int as libc::c_double;
    } else if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
        & 2 as libc::c_int
        != 0
    {
        p.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .bb
            .LL
            .x
            + d.x / 2 as libc::c_int as libc::c_double;
    } else {
        p.x = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .bb
            .LL
            .x
            + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .x)
            / 2 as libc::c_int as libc::c_double;
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
        & 1 as libc::c_int
        != 0
    {
        p.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .bb
            .UR
            .y
            - d.y / 2 as libc::c_int as libc::c_double;
    } else {
        p.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .bb
            .LL
            .y
            + d.y / 2 as libc::c_int as libc::c_double;
    }
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos = p;
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set = 0 as libc::c_int == 0;
}
unsafe extern "C" fn centerPt(mut xlp: *mut xlabel_t) -> pointf {
    let mut p: pointf = pointf { x: 0., y: 0. };
    p = (*xlp).pos;
    p.x += (*xlp).sz.x / 2.0f64;
    p.y += (*xlp).sz.y / 2.0f64;
    return p;
}
unsafe extern "C" fn printData(
    mut objs: *mut object_t,
    mut n_objs: libc::c_int,
    mut lbls: *mut xlabel_t,
    mut n_lbls: libc::c_int,
    mut params: *mut label_params_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut xp: *mut xlabel_t = 0 as *mut xlabel_t;
    fprintf(
        stderr,
        b"%d objs %d xlabels force=%d bb=(%.02f,%.02f) (%.02f,%.02f)\n\0" as *const u8
            as *const libc::c_char,
        n_objs,
        n_lbls,
        (*params).force as libc::c_int,
        (*params).bb.LL.x,
        (*params).bb.LL.y,
        (*params).bb.UR.x,
        (*params).bb.UR.y,
    );
    if (Verbose as libc::c_int) < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    fprintf(stderr, b"objects\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n_objs {
        xp = (*objs).lbl;
        fprintf(
            stderr,
            b" [%d] (%.02f,%.02f) (%.02f,%.02f) %p \"%s\"\n\0" as *const u8 as *const libc::c_char,
            i,
            (*objs).pos.x,
            (*objs).pos.y,
            (*objs).sz.x,
            (*objs).sz.y,
            (*objs).lbl,
            if !xp.is_null() {
                (*((*xp).lbl as *mut textlabel_t)).text as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        objs = objs.offset(1);
        i += 1;
    }
    fprintf(stderr, b"xlabels\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n_lbls {
        fprintf(
            stderr,
            b" [%d] %p set %d (%.02f,%.02f) (%.02f,%.02f) %s\n\0" as *const u8
                as *const libc::c_char,
            i,
            lbls,
            (*lbls).set as libc::c_int,
            (*lbls).pos.x,
            (*lbls).pos.y,
            (*lbls).sz.x,
            (*lbls).sz.y,
            (*((*lbls).lbl as *mut textlabel_t)).text,
        );
        lbls = lbls.offset(1);
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn edgeTailpoint(mut e: *mut Agedge_t) -> pointf {
    let mut spl: *mut splines = 0 as *mut splines;
    let mut bez: *mut bezier = 0 as *mut bezier;
    spl = getsplinepoints(e);
    if spl.is_null() {
        let mut p: pointf = pointf { x: 0., y: 0. };
        p.y = 0 as libc::c_int as libc::c_double;
        p.x = p.y;
        return p;
    }
    bez = &mut *((*spl).list).offset(0 as libc::c_int as isize) as *mut bezier;
    if (*bez).sflag != 0 {
        return (*bez).sp;
    } else {
        return *((*bez).list).offset(0 as libc::c_int as isize);
    };
}
unsafe extern "C" fn edgeHeadpoint(mut e: *mut Agedge_t) -> pointf {
    let mut spl: *mut splines = 0 as *mut splines;
    let mut bez: *mut bezier = 0 as *mut bezier;
    spl = getsplinepoints(e);
    if spl.is_null() {
        let mut p: pointf = pointf { x: 0., y: 0. };
        p.y = 0 as libc::c_int as libc::c_double;
        p.x = p.y;
        return p;
    }
    bez = &mut *((*spl).list).offset(((*spl).size - 1 as libc::c_int) as isize) as *mut bezier;
    if (*bez).eflag != 0 {
        return (*bez).ep;
    } else {
        return *((*bez).list).offset(((*bez).size - 1 as libc::c_int) as isize);
    };
}
unsafe extern "C" fn adjustBB(mut objp: *mut object_t, mut bb: boxf) -> boxf {
    let mut ur: pointf = pointf { x: 0., y: 0. };
    bb.LL.x = if bb.LL.x < (*objp).pos.x {
        bb.LL.x
    } else {
        (*objp).pos.x
    };
    bb.LL.y = if bb.LL.y < (*objp).pos.y {
        bb.LL.y
    } else {
        (*objp).pos.y
    };
    ur.x = (*objp).pos.x + (*objp).sz.x;
    ur.y = (*objp).pos.y + (*objp).sz.y;
    bb.UR.x = if bb.UR.x > ur.x { bb.UR.x } else { ur.x };
    bb.UR.y = if bb.UR.y > ur.y { bb.UR.y } else { ur.y };
    return bb;
}
unsafe extern "C" fn addXLabel(
    mut lp: *mut textlabel_t,
    mut objp: *mut object_t,
    mut xlp: *mut xlabel_t,
    mut initObj: libc::c_int,
    mut pos: pointf,
) {
    if initObj != 0 {
        (*objp).sz.x = 0 as libc::c_int as libc::c_double;
        (*objp).sz.y = 0 as libc::c_int as libc::c_double;
        (*objp).pos = pos;
    }
    if Flip {
        (*xlp).sz.x = (*lp).dimen.y;
        (*xlp).sz.y = (*lp).dimen.x;
    } else {
        (*xlp).sz = (*lp).dimen;
    }
    let ref mut fresh0 = (*xlp).lbl;
    *fresh0 = lp as *mut libc::c_void;
    (*xlp).set = 0 as libc::c_int as libc::c_uchar;
    let ref mut fresh1 = (*objp).lbl;
    *fresh1 = xlp;
}
unsafe extern "C" fn addLabelObj(
    mut lp: *mut textlabel_t,
    mut objp: *mut object_t,
    mut bb: boxf,
) -> boxf {
    if Flip {
        (*objp).sz.x = (*lp).dimen.y;
        (*objp).sz.y = (*lp).dimen.x;
    } else {
        (*objp).sz.x = (*lp).dimen.x;
        (*objp).sz.y = (*lp).dimen.y;
    }
    (*objp).pos = (*lp).pos;
    (*objp).pos.x -= (*objp).sz.x / 2.0f64;
    (*objp).pos.y -= (*objp).sz.y / 2.0f64;
    return adjustBB(objp, bb);
}
unsafe extern "C" fn addNodeObj(
    mut np: *mut node_t,
    mut objp: *mut object_t,
    mut bb: boxf,
) -> boxf {
    if Flip {
        (*objp).sz.x = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double;
        (*objp).sz.y = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double;
    } else {
        (*objp).sz.x = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double;
        (*objp).sz.y = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double;
    }
    (*objp).pos = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    (*objp).pos.x -= (*objp).sz.x / 2.0f64;
    (*objp).pos.y -= (*objp).sz.y / 2.0f64;
    return adjustBB(objp, bb);
}
unsafe extern "C" fn addClusterObj(mut g: *mut Agraph_t, mut info: cinfo_t) -> cinfo_t {
    let mut c: libc::c_int = 0;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        info = addClusterObj(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize),
            info,
        );
        c += 1;
    }
    if g != agroot(g as *mut libc::c_void)
        && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set as libc::c_int != 0
    {
        let mut objp: *mut object_t = info.objp;
        info.bb = addLabelObj(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label,
            objp,
            info.bb,
        );
        info.objp = (info.objp).offset(1);
    }
    return info;
}
unsafe extern "C" fn countClusterLabels(mut g: *mut Agraph_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    if g != agroot(g as *mut libc::c_void)
        && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set as libc::c_int != 0
    {
        i += 1;
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        i += countClusterLabels(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize),
        );
        c += 1;
    }
    return i;
}
unsafe extern "C" fn addXLabels(mut gp: *mut Agraph_t) {
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ep: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n_objs: libc::c_int = 0;
    let mut n_lbls: libc::c_int = 0;
    let mut n_nlbls: libc::c_int = 0 as libc::c_int;
    let mut n_elbls: libc::c_int = 0 as libc::c_int;
    let mut n_set_lbls: libc::c_int = 0 as libc::c_int;
    let mut n_clbls: libc::c_int = 0 as libc::c_int;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut lp: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut params: label_params_t = label_params_t {
        bb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        force: 0,
    };
    let mut objs: *mut object_t = 0 as *mut object_t;
    let mut lbls: *mut xlabel_t = 0 as *mut xlabel_t;
    let mut objp: *mut object_t = 0 as *mut object_t;
    let mut xlp: *mut xlabel_t = 0 as *mut xlabel_t;
    let mut force: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut et: libc::c_int = (*((*(gp as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags
        as libc::c_int
        & (7 as libc::c_int) << 1 as libc::c_int;
    if (*((*(gp as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 4 as libc::c_int
        == 0
        && (*((*(gp as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
            & (1 as libc::c_int) << 5 as libc::c_int
            == 0
        && (*((*(gp as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int
            == 0
        && (*((*(gp as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int
            == 0
        && ((*((*(gp as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
            & (1 as libc::c_int) << 0 as libc::c_int
            == 0
            || EdgeLabelsDone != 0)
    {
        return;
    }
    np = agfstnode(gp);
    while !np.is_null() {
        if !((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null() {
            if (*(*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).set {
                n_set_lbls += 1;
            } else {
                n_nlbls += 1;
            }
        }
        ep = agfstout(gp, np);
        while !ep.is_null() {
            if !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).is_null() {
                if (*(*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).set {
                    n_set_lbls += 1;
                } else if et != (0 as libc::c_int) << 1 as libc::c_int
                    && !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
                {
                    n_elbls += 1;
                }
            }
            if !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).is_null() {
                if (*(*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).set {
                    n_set_lbls += 1;
                } else if et != (0 as libc::c_int) << 1 as libc::c_int
                    && !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
                {
                    n_elbls += 1;
                }
            }
            if !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).is_null() {
                if (*(*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).set {
                    n_set_lbls += 1;
                } else if et != (0 as libc::c_int) << 1 as libc::c_int
                    && !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
                {
                    n_elbls += 1;
                }
            }
            if !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
                if (*(*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).set {
                    n_set_lbls += 1;
                } else if et != (0 as libc::c_int) << 1 as libc::c_int
                    && !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
                {
                    n_elbls += 1;
                }
            }
            ep = agnxtout(gp, ep);
        }
        np = agnxtnode(gp, np);
    }
    if (*((*(gp as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 3 as libc::c_int
        != 0
    {
        n_clbls = countClusterLabels(gp);
    }
    n_lbls = n_nlbls + n_elbls;
    if n_lbls == 0 as libc::c_int {
        return;
    }
    n_objs = agnnodes(gp) + n_set_lbls + n_clbls + n_elbls;
    objs = gcalloc(
        n_objs as size_t,
        ::std::mem::size_of::<object_t>() as libc::c_ulong,
    ) as *mut object_t;
    objp = objs;
    lbls = gcalloc(
        n_lbls as size_t,
        ::std::mem::size_of::<xlabel_t>() as libc::c_ulong,
    ) as *mut xlabel_t;
    xlp = lbls;
    bb.LL = pointfof(
        2147483647 as libc::c_int as libc::c_double,
        2147483647 as libc::c_int as libc::c_double,
    );
    bb.UR = pointfof(
        -(2147483647 as libc::c_int) as libc::c_double,
        -(2147483647 as libc::c_int) as libc::c_double,
    );
    np = agfstnode(gp);
    while !np.is_null() {
        bb = addNodeObj(np, objp, bb);
        lp = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel;
        if !lp.is_null() {
            if (*lp).set {
                objp = objp.offset(1);
                bb = addLabelObj(lp, objp, bb);
            } else {
                let mut ignored: pointf = {
                    let mut init = pointf_s {
                        x: 0.0f64,
                        y: 0.0f64,
                    };
                    init
                };
                addXLabel(lp, objp, xlp, 0 as libc::c_int, ignored);
                xlp = xlp.offset(1);
            }
        }
        objp = objp.offset(1);
        let mut current_block_87: u64;
        ep = agfstout(gp, np);
        while !ep.is_null() {
            lp = (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
            if !lp.is_null() {
                if (*lp).set {
                    bb = addLabelObj(lp, objp, bb);
                    current_block_87 = 17020603795727957434;
                } else if et != (0 as libc::c_int) << 1 as libc::c_int
                    && !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
                {
                    addXLabel(lp, objp, xlp, 1 as libc::c_int, edgeMidpoint(gp, ep));
                    xlp = xlp.offset(1);
                    current_block_87 = 17020603795727957434;
                } else {
                    agerr(
                        AGWARN,
                        b"no position for edge with label %s\n\0" as *const u8
                            as *const libc::c_char,
                        (*(*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).text,
                    );
                    current_block_87 = 7494008139977416618;
                }
                match current_block_87 {
                    7494008139977416618 => {}
                    _ => {
                        objp = objp.offset(1);
                        current_block_87 = 5658374378798827547;
                    }
                }
            } else {
                current_block_87 = 5658374378798827547;
            }
            match current_block_87 {
                5658374378798827547 => {
                    lp = (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label;
                    if !lp.is_null() {
                        if (*lp).set {
                            bb = addLabelObj(lp, objp, bb);
                            current_block_87 = 15237655884915618618;
                        } else if et != (0 as libc::c_int) << 1 as libc::c_int
                            && !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
                                .is_null()
                        {
                            addXLabel(lp, objp, xlp, 1 as libc::c_int, edgeTailpoint(ep));
                            xlp = xlp.offset(1);
                            current_block_87 = 15237655884915618618;
                        } else {
                            agerr(
                                AGWARN,
                                b"no position for edge with tail label %s\n\0" as *const u8
                                    as *const libc::c_char,
                                (*(*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                    .tail_label)
                                    .text,
                            );
                            current_block_87 = 7494008139977416618;
                        }
                        match current_block_87 {
                            7494008139977416618 => {}
                            _ => {
                                objp = objp.offset(1);
                                current_block_87 = 15864857819291709765;
                            }
                        }
                    } else {
                        current_block_87 = 15864857819291709765;
                    }
                    match current_block_87 {
                        7494008139977416618 => {}
                        _ => {
                            lp = (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label;
                            if !lp.is_null() {
                                if (*lp).set {
                                    bb = addLabelObj(lp, objp, bb);
                                    current_block_87 = 18425699056680496821;
                                } else if et != (0 as libc::c_int) << 1 as libc::c_int
                                    && !((*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
                                        .is_null()
                                {
                                    addXLabel(lp, objp, xlp, 1 as libc::c_int, edgeHeadpoint(ep));
                                    xlp = xlp.offset(1);
                                    current_block_87 = 18425699056680496821;
                                } else {
                                    agerr(
                                        AGWARN,
                                        b"no position for edge with head label %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*(*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                            .head_label)
                                            .text,
                                    );
                                    current_block_87 = 7494008139977416618;
                                }
                                match current_block_87 {
                                    7494008139977416618 => {}
                                    _ => {
                                        objp = objp.offset(1);
                                        current_block_87 = 8937240710477387595;
                                    }
                                }
                            } else {
                                current_block_87 = 8937240710477387595;
                            }
                            match current_block_87 {
                                7494008139977416618 => {}
                                _ => {
                                    lp = (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                        .xlabel;
                                    if !lp.is_null() {
                                        if (*lp).set {
                                            bb = addLabelObj(lp, objp, bb);
                                            current_block_87 = 4804377075063615140;
                                        } else if et != (0 as libc::c_int) << 1 as libc::c_int
                                            && !((*((*(ep as *mut Agobj_t)).data
                                                as *mut Agedgeinfo_t))
                                                .spl)
                                                .is_null()
                                        {
                                            addXLabel(
                                                lp,
                                                objp,
                                                xlp,
                                                1 as libc::c_int,
                                                edgeMidpoint(gp, ep),
                                            );
                                            xlp = xlp.offset(1);
                                            current_block_87 = 4804377075063615140;
                                        } else {
                                            agerr(
                                                AGWARN,
                                                b"no position for edge with xlabel %s\n\0"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                (*(*((*(ep as *mut Agobj_t)).data
                                                    as *mut Agedgeinfo_t))
                                                    .xlabel)
                                                    .text,
                                            );
                                            current_block_87 = 7494008139977416618;
                                        }
                                        match current_block_87 {
                                            7494008139977416618 => {}
                                            _ => {
                                                objp = objp.offset(1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            ep = agnxtout(gp, ep);
        }
        np = agnxtnode(gp, np);
    }
    if n_clbls != 0 {
        let mut info: cinfo_t = cinfo_t {
            bb: boxf {
                LL: pointf { x: 0., y: 0. },
                UR: pointf { x: 0., y: 0. },
            },
            objp: 0 as *mut object_t,
        };
        info.bb = bb;
        info.objp = objp;
        info = addClusterObj(gp, info);
        bb = info.bb;
    }
    force = agattr(
        gp,
        0 as libc::c_int,
        b"forcelabels\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    params.force =
        late_bool(gp as *mut libc::c_void, force, 1 as libc::c_int != 0) as libc::c_uchar;
    params.bb = bb;
    placeLabels(objs, n_objs, lbls, n_lbls, &mut params);
    if Verbose != 0 {
        printData(objs, n_objs, lbls, n_lbls, &mut params);
    }
    xlp = lbls;
    cnt = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n_lbls {
        if (*xlp).set != 0 {
            cnt += 1;
            lp = (*xlp).lbl as *mut textlabel_t;
            (*lp).set = 1 as libc::c_int != 0;
            (*lp).pos = centerPt(xlp);
            updateBB(gp, lp);
        }
        xlp = xlp.offset(1);
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"%d out of %d labels positioned.\n\0" as *const u8 as *const libc::c_char,
            cnt,
            n_lbls,
        );
    } else if cnt != n_lbls {
        agerr(
            AGWARN,
            b"%d out of %d exterior labels positioned.\n\0" as *const u8 as *const libc::c_char,
            cnt,
            n_lbls,
        );
    }
    free(objs as *mut libc::c_void);
    free(lbls as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gv_postprocess(mut g: *mut Agraph_t, mut allowTranslation: libc::c_int) {
    let mut diff: libc::c_double = 0.;
    let mut dimen: pointf = {
        let mut init = pointf_s {
            x: 0.0f64,
            y: 0.0f64,
        };
        init
    };
    Rankdir = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir & 0x3 as libc::c_int;
    Flip = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
        & 0x3 as libc::c_int
        & 1 as libc::c_int
        != 0;
    if Flip {
        place_flip_graph_label(g);
    } else {
        place_graph_label(g);
    }
    addXLabels(g);
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && !(*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set
    {
        dimen = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).dimen;
        dimen.x += (4 as libc::c_int * 4 as libc::c_int) as libc::c_double;
        dimen.y += (2 as libc::c_int * 4 as libc::c_int) as libc::c_double;
        if Flip {
            if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
                & 1 as libc::c_int
                != 0
            {
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x += dimen.y;
            } else {
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .LL
                    .x -= dimen.y;
            }
            if dimen.x
                > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .y
                    - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .y
            {
                diff = dimen.x
                    - ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .y
                        - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                            .bb
                            .LL
                            .y);
                diff = diff / 2.0f64;
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .LL
                    .y -= diff;
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .y += diff;
            }
        } else {
            if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
                & 1 as libc::c_int
                != 0
            {
                if Rankdir == 0 as libc::c_int {
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .y += dimen.y;
                } else {
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .y -= dimen.y;
                }
            } else if Rankdir == 0 as libc::c_int {
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .LL
                    .y -= dimen.y;
            } else {
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .y += dimen.y;
            }
            if dimen.x
                > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x
                    - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .x
            {
                diff = dimen.x
                    - ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .x
                        - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                            .bb
                            .LL
                            .x);
                diff = diff / 2.0f64;
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .LL
                    .x -= diff;
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x += diff;
            }
        }
    }
    if allowTranslation != 0 {
        match Rankdir {
            0 => {
                Offset = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL;
            }
            1 => {
                Offset = pointfof(
                    -(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .y,
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .x,
                );
            }
            2 => {
                Offset = pointfof(
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .x,
                    -(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .y,
                );
            }
            3 => {
                Offset = pointfof(
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .y,
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .x,
                );
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"postproc.c\0" as *const u8 as *const libc::c_char,
                    674 as libc::c_int,
                );
                abort();
            }
        }
        translate_drawing(g);
    }
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && !(*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set
    {
        place_root_label(g, dimen);
    }
    if !Show_boxes.is_null() {
        let mut buf: [libc::c_char; 8192] = [0; 8192];
        if Flip {
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                b"/pathbox {\n    /X exch neg %.5g sub def\n    /Y exch %.5g sub def\n    /x exch neg %.5g sub def\n    /y exch %.5g sub def\n    newpath x y moveto\n    X y lineto\n    X Y lineto\n    x Y lineto\n    closepath stroke\n} def\n\0"
                    as *const u8 as *const libc::c_char,
                Offset.x,
                Offset.y,
                Offset.x,
                Offset.y,
            );
        } else {
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                b"/pathbox {\n    /Y exch %.5g sub def\n    /X exch %.5g sub def\n    /y exch %.5g sub def\n    /x exch %.5g sub def\n    newpath x y moveto\n    X y lineto\n    X Y lineto\n    x Y lineto\n    closepath stroke\n } def\n/dbgstart { gsave %.5g %.5g translate } def\n/arrowlength 10 def\n/arrowwidth arrowlength 2 div def\n/arrowhead {\n    gsave\n    rotate\n    currentpoint\n    newpath\n    moveto\n    arrowlength arrowwidth 2 div rlineto\n    0 arrowwidth neg rlineto\n    closepath fill\n    grestore\n} bind def\n/makearrow {\n    currentpoint exch pop sub exch currentpoint pop sub atan\n    arrowhead\n} bind def\n/point {    newpath    2 0 360 arc fill} def/makevec {\n    /Y exch def\n    /X exch def\n    /y exch def\n    /x exch def\n    newpath x y moveto\n    X Y lineto stroke\n    X Y moveto\n    x y makearrow\n} def\n\0"
                    as *const u8 as *const libc::c_char,
                Offset.y,
                Offset.x,
                Offset.y,
                Offset.x,
                -Offset.x,
                -Offset.y,
            );
        }
        let ref mut fresh2 = *Show_boxes.offset(0 as libc::c_int as isize);
        *fresh2 = strdup(buf.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn dotneato_postprocess(mut g: *mut Agraph_t) {
    gv_postprocess(g, 1 as libc::c_int);
}
unsafe extern "C" fn place_flip_graph_label(mut g: *mut graph_t) {
    let mut c: libc::c_int = 0;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut d: pointf = pointf { x: 0., y: 0. };
    if g != agroot(g as *mut libc::c_void)
        && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && !(*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set
    {
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
            & 1 as libc::c_int
            != 0
        {
            d = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).border
                [1 as libc::c_int as usize];
            p.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .x
                - d.x / 2 as libc::c_int as libc::c_double;
        } else {
            d = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).border
                [3 as libc::c_int as usize];
            p.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .LL
                .x
                + d.x / 2 as libc::c_int as libc::c_double;
        }
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
            & 4 as libc::c_int
            != 0
        {
            p.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .LL
                .y
                + d.y / 2 as libc::c_int as libc::c_double;
        } else if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
            & 2 as libc::c_int
            != 0
        {
            p.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .y
                - d.y / 2 as libc::c_int as libc::c_double;
        } else {
            p.y = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .LL
                .y
                + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .y)
                / 2 as libc::c_int as libc::c_double;
        }
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos = p;
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set = 0 as libc::c_int == 0;
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        place_flip_graph_label(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize),
        );
        c += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn place_graph_label(mut g: *mut graph_t) {
    let mut c: libc::c_int = 0;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut d: pointf = pointf { x: 0., y: 0. };
    if g != agroot(g as *mut libc::c_void)
        && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && !(*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set
    {
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
            & 1 as libc::c_int
            != 0
        {
            d = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).border
                [2 as libc::c_int as usize];
            p.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .y
                - d.y / 2 as libc::c_int as libc::c_double;
        } else {
            d = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).border
                [0 as libc::c_int as usize];
            p.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .LL
                .y
                + d.y / 2 as libc::c_int as libc::c_double;
        }
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
            & 4 as libc::c_int
            != 0
        {
            p.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .x
                - d.x / 2 as libc::c_int as libc::c_double;
        } else if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos as libc::c_int
            & 2 as libc::c_int
            != 0
        {
            p.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .LL
                .x
                + d.x / 2 as libc::c_int as libc::c_double;
        } else {
            p.x = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .LL
                .x
                + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x)
                / 2 as libc::c_int as libc::c_double;
        }
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos = p;
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set = 1 as libc::c_int != 0;
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        place_graph_label(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize),
        );
        c += 1;
    }
}
