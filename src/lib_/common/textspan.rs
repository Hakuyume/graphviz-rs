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
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn emit_once(message: *mut libc::c_char) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn gvtextlayout(
        gvc: *mut GVC_t,
        span: *mut textspan_t,
        fontpath: *mut *mut libc::c_char,
    ) -> bool;
    fn estimate_text_width_1pt(
        font_name: *const libc::c_char,
        text: *const libc::c_char,
        bold: bool,
        italic: bool,
    ) -> libc::c_double;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obj_state_s {
    pub parent: *mut obj_state_t,
    pub type_0: obj_type,
    pub u: C2RustUnnamed_2,
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
    pub u: C2RustUnnamed_1,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub union C2RustUnnamed_2 {
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
pub type agobjfn_t = Option::<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> (),
>;
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
pub type IDTYPE = uint64_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
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
unsafe extern "C" fn estimate_textspan_size(
    mut span: *mut textspan_t,
    mut fontpath: *mut *mut libc::c_char,
) {
    let mut fontsize: libc::c_double = 0.;
    let mut flags: libc::c_int = (*(*span).font).flags() as libc::c_int;
    let mut bold: bool = flags & (1 as libc::c_int) << 0 as libc::c_int
        != 0 as libc::c_int;
    let mut italic: bool = flags & (1 as libc::c_int) << 1 as libc::c_int
        != 0 as libc::c_int;
    fontsize = (*(*span).font).size;
    (*span).size.x = 0.0f64;
    (*span).size.y = fontsize * 1.20f64;
    (*span).yoffset_layout = 0.0f64;
    (*span).yoffset_centerline = 0.1f64 * fontsize;
    let ref mut fresh0 = (*span).layout;
    *fresh0 = 0 as *mut libc::c_void;
    let ref mut fresh1 = (*span).free_layout;
    *fresh1 = None;
    (*span)
        .size
        .x = fontsize
        * estimate_text_width_1pt((*(*span).font).name, (*span).str_0, bold, italic);
    if !fontpath.is_null() {
        *fontpath = b"[internal hard-coded]\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
}
static mut postscript_alias: [PostscriptAlias; 35] = [
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-Book\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"book\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 4 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-BookOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"book\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 5 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-Demi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 6 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-DemiOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 7 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-Demi\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 10 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-DemiItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 11 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-Light\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"light\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 8 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-LightItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"light\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 9 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 12 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 14 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier-BoldOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 15 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier-Oblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 13 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 16 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 18 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-BoldOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 19 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 20 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 22 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow-BoldOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 23 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow-Oblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 21 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Oblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 17 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 26 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-BoldItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 27 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-Italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 25 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-Roman\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 24 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 30 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-BoldItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 31 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-Italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 29 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-Roman\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 28 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Symbol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Symbol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 32 as libc::c_int,
            svg_font_family: b"fantasy\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 2 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-BoldItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 3 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-Italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 1 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-Roman\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 0 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"ZapfChancery-MediumItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Chancery L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"medium\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 33 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"ZapfDingbats\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Dingbats\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 34 as libc::c_int,
            svg_font_family: b"fantasy\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn fontcmpf(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return strcasecmp(
        (*(a as *const PostscriptAlias)).name,
        (*(b as *const PostscriptAlias)).name,
    );
}
unsafe extern "C" fn translate_postscript_fontname(
    mut fontname: *mut libc::c_char,
) -> *mut PostscriptAlias {
    static mut key: PostscriptAlias = PostscriptAlias {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        family: 0 as *const libc::c_char as *mut libc::c_char,
        weight: 0 as *const libc::c_char as *mut libc::c_char,
        stretch: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0 as *const libc::c_char as *mut libc::c_char,
        xfig_code: 0,
        svg_font_family: 0 as *const libc::c_char as *mut libc::c_char,
        svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
        svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
    };
    static mut result: *mut PostscriptAlias = 0 as *const PostscriptAlias
        as *mut PostscriptAlias;
    if (key.name).is_null() || strcasecmp(key.name, fontname) != 0 {
        free(key.name as *mut libc::c_void);
        key.name = strdup(fontname);
        result = bsearch(
            &mut key as *mut PostscriptAlias as *const libc::c_void,
            postscript_alias.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[PostscriptAlias; 35]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<PostscriptAlias>() as libc::c_ulong),
            ::std::mem::size_of::<PostscriptAlias>() as libc::c_ulong,
            Some(
                fontcmpf
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut PostscriptAlias;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn textspan_size(
    mut gvc: *mut GVC_t,
    mut span: *mut textspan_t,
) -> pointf {
    let mut fpp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut fontpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut font: *mut textfont_t = 0 as *mut textfont_t;
    if !((*span).font).is_null() {} else {
        __assert_fail(
            b"span->font\0" as *const u8 as *const libc::c_char,
            b"textspan.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"pointf textspan_size(GVC_t *, textspan_t *)\0"))
                .as_ptr(),
        );
    }
    font = (*span).font;
    if !((*font).name).is_null() {} else {
        __assert_fail(
            b"font->name\0" as *const u8 as *const libc::c_char,
            b"textspan.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"pointf textspan_size(GVC_t *, textspan_t *)\0"))
                .as_ptr(),
        );
    }
    if ((*font).postscript_alias).is_null() {
        let ref mut fresh2 = (*font).postscript_alias;
        *fresh2 = translate_postscript_fontname((*font).name);
    }
    if Verbose as libc::c_int != 0 && emit_once((*font).name) != 0 {
        fpp = &mut fontpath;
    }
    if !gvtextlayout(gvc, span, fpp) {
        estimate_textspan_size(span, fpp);
    }
    if !fpp.is_null() {
        if !fontpath.is_null() {
            fprintf(
                stderr,
                b"fontname: \"%s\" resolved to: %s\n\0" as *const u8
                    as *const libc::c_char,
                (*font).name,
                fontpath,
            );
        } else {
            fprintf(
                stderr,
                b"fontname: unable to resolve \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                (*font).name,
            );
        }
    }
    return (*span).size;
}
unsafe extern "C" fn textfont_makef(
    mut dt: *mut Dt_t,
    mut obj: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut f1: *mut textfont_t = obj as *mut textfont_t;
    let mut f2: *mut textfont_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<textfont_t>() as libc::c_ulong,
    ) as *mut textfont_t;
    if !((*f1).name).is_null() {
        let ref mut fresh3 = (*f2).name;
        *fresh3 = strdup((*f1).name);
    }
    if !((*f1).color).is_null() {
        let ref mut fresh4 = (*f2).color;
        *fresh4 = strdup((*f1).color);
    }
    (*f2).set_flags((*f1).flags());
    (*f2).size = (*f1).size;
    let ref mut fresh5 = (*f2).postscript_alias;
    *fresh5 = (*f1).postscript_alias;
    return f2 as *mut libc::c_void;
}
unsafe extern "C" fn textfont_freef(
    mut dt: *mut Dt_t,
    mut obj: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) {
    let mut f: *mut textfont_t = obj as *mut textfont_t;
    free((*f).name as *mut libc::c_void);
    free((*f).color as *mut libc::c_void);
    free(f as *mut libc::c_void);
}
unsafe extern "C" fn textfont_comparf(
    mut dt: *mut Dt_t,
    mut key1: *mut libc::c_void,
    mut key2: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut f1: *mut textfont_t = key1 as *mut textfont_t;
    let mut f2: *mut textfont_t = key2 as *mut textfont_t;
    if !((*f1).name).is_null() || !((*f2).name).is_null() {
        if ((*f1).name).is_null() {
            return -(1 as libc::c_int);
        }
        if ((*f2).name).is_null() {
            return 1 as libc::c_int;
        }
        rc = strcmp((*f1).name, (*f2).name);
        if rc != 0 {
            return rc;
        }
    }
    if !((*f1).color).is_null() || !((*f2).color).is_null() {
        if ((*f1).color).is_null() {
            return -(1 as libc::c_int);
        }
        if ((*f2).color).is_null() {
            return 1 as libc::c_int;
        }
        rc = strcmp((*f1).color, (*f2).color);
        if rc != 0 {
            return rc;
        }
    }
    rc = (*f1).flags() as libc::c_int - (*f2).flags() as libc::c_int;
    if rc != 0 {
        return rc;
    }
    if (*f1).size < (*f2).size {
        return -(1 as libc::c_int);
    }
    if (*f1).size > (*f2).size {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn textfont_dict_open(mut gvc: *mut GVC_t) -> *mut Dt_t {
    (*gvc).textfont_disc.key = 0 as libc::c_int;
    (*gvc)
        .textfont_disc
        .size = ::std::mem::size_of::<textfont_t>() as libc::c_ulong as libc::c_int;
    (*gvc).textfont_disc.link = -(1 as libc::c_int);
    let ref mut fresh6 = (*gvc).textfont_disc.makef;
    *fresh6 = Some(
        textfont_makef
            as unsafe extern "C" fn(
                *mut Dt_t,
                *mut libc::c_void,
                *mut Dtdisc_t,
            ) -> *mut libc::c_void,
    );
    let ref mut fresh7 = (*gvc).textfont_disc.freef;
    *fresh7 = Some(
        textfont_freef
            as unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
    );
    let ref mut fresh8 = (*gvc).textfont_disc.comparf;
    *fresh8 = Some(
        textfont_comparf
            as unsafe extern "C" fn(
                *mut Dt_t,
                *mut libc::c_void,
                *mut libc::c_void,
                *mut Dtdisc_t,
            ) -> libc::c_int,
    );
    let ref mut fresh9 = (*gvc).textfont_disc.hashf;
    *fresh9 = None;
    let ref mut fresh10 = (*gvc).textfont_disc.memoryf;
    *fresh10 = None;
    let ref mut fresh11 = (*gvc).textfont_disc.eventf;
    *fresh11 = None;
    let ref mut fresh12 = (*gvc).textfont_dt;
    *fresh12 = dtopen(&mut (*gvc).textfont_disc, Dtoset);
    return (*gvc).textfont_dt;
}
#[no_mangle]
pub unsafe extern "C" fn textfont_dict_close(mut gvc: *mut GVC_t) {
    dtclose((*gvc).textfont_dt);
}
