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
    pub type XML_ParserStruct;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn scanEntity(t: *mut libc::c_char, xb: *mut agxbuf) -> *mut libc::c_char;
    fn charsetToStr(c: libc::c_int) -> *mut libc::c_char;
    static mut htmllval: HTMLSTYPE;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut libc::c_void);
    fn XML_Parse(
        parser: XML_Parser,
        s: *const libc::c_char,
        len: libc::c_int,
        isFinal: libc::c_int,
    ) -> XML_Status;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
}
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
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
pub struct htmllabel_t {
    pub u: C2RustUnnamed_4,
    pub kind: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub tbl: *mut htmltbl_t,
    pub txt: *mut htmltxt_t,
    pub img: *mut htmlimg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlimg_t {
    pub box_0: boxf,
    pub src: *mut libc::c_char,
    pub scale: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmltxt_t {
    pub spans: *mut htextspan_t,
    pub nspans: libc::c_short,
    pub simple: libc::c_char,
    pub box_0: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htextspan_t {
    pub items: *mut textspan_t,
    pub nitems: libc::c_short,
    pub just: libc::c_char,
    pub size: libc::c_double,
    pub lfsize: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmltbl_t {
    pub data: htmldata_t,
    pub u: C2RustUnnamed_5,
    pub cb: libc::c_schar,
    pub heights: *mut libc::c_int,
    pub widths: *mut libc::c_int,
    pub rc: libc::c_int,
    pub cc: libc::c_int,
    pub font: *mut textfont_t,
    pub flags: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub n: C2RustUnnamed_7,
    pub p: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub prev: *mut htmltbl_t,
    pub rows: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub parent: *mut htmlcell_t,
    pub cells: *mut *mut htmlcell_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlcell_t {
    pub data: htmldata_t,
    pub cspan: libc::c_ushort,
    pub rspan: libc::c_ushort,
    pub col: libc::c_ushort,
    pub row: libc::c_ushort,
    pub child: htmllabel_t,
    pub parent: *mut htmltbl_t,
    pub ruled: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmldata_t {
    pub href: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub bgcolor: *mut libc::c_char,
    pub pencolor: *mut libc::c_char,
    pub gradientangle: libc::c_int,
    pub space: libc::c_schar,
    pub border: libc::c_uchar,
    pub pad: libc::c_uchar,
    pub sides: libc::c_uchar,
    pub flags: libc::c_ushort,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub style: libc::c_ushort,
    pub box_0: boxf,
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
    pub u: C2RustUnnamed_8,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub txt: C2RustUnnamed_9,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub span: *mut textspan_t,
    pub nspans: libc::c_short,
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
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pitem {
    pub link: Dtlink_t,
    pub u: C2RustUnnamed_10,
    pub ruled: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub rp: *mut Dt_t,
    pub cp: *mut htmlcell_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlenv_t {
    pub pos: pointf,
    pub finfo: textfont_t,
    pub obj: *mut libc::c_void,
    pub g: *mut graph_t,
    pub imgscale: *mut libc::c_char,
    pub objid: *mut libc::c_char,
    pub objid_set: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union HTMLSTYPE {
    pub i: libc::c_int,
    pub txt: *mut htmltxt_t,
    pub cell: *mut htmlcell_t,
    pub tbl: *mut htmltbl_t,
    pub font: *mut textfont_t,
    pub img: *mut htmlimg_t,
    pub p: *mut pitem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lexstate_t {
    pub parser: XML_Parser,
    pub ptr: *mut libc::c_char,
    pub tok: libc::c_int,
    pub xb: *mut agxbuf,
    pub lb: agxbuf,
    pub warn: libc::c_int,
    pub error: libc::c_int,
    pub inCell: libc::c_char,
    pub mode: libc::c_char,
    pub currtok: *mut libc::c_char,
    pub prevtok: *mut libc::c_char,
    pub currtoklen: size_t,
    pub prevtoklen: size_t,
}
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_CharacterDataHandler =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const XML_Char, libc::c_int) -> ()>;
pub type XML_Char = libc::c_char;
pub type XML_Size = libc::c_ulong;
pub type XML_StartElementHandler =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const XML_Char, *mut *const XML_Char) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_item {
    pub name: *mut libc::c_char,
    pub action: attrFn,
}
pub type attrFn = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char) -> libc::c_int>;
pub type bcmpfn =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub const FLAGS_MAX: C2RustUnnamed_11 = 127;
pub type C2RustUnnamed_11 = libc::c_uint;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const XML_Char) -> ()>;
pub type XML_LChar = libc::c_char;
pub type XML_Error = libc::c_uint;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_NONE: XML_Error = 0;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub type XML_Status = libc::c_uint;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_OK: XML_Status = 1;
#[inline]
unsafe extern "C" fn gv_recalloc(
    mut ptr: *mut libc::c_void,
    mut old_nmemb: size_t,
    mut new_nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size > 0 as libc::c_int as libc::c_ulong
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size)) as libc::c_int
        as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(
        ptr,
        old_nmemb.wrapping_mul(size),
        new_nmemb.wrapping_mul(size),
    );
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
        __idx = __l
            .wrapping_add(__u)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void;
        }
    }
    return 0 as *mut libc::c_void;
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
unsafe extern "C" fn gv_realloc(
    mut ptr: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, new_size);
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    if new_size > old_size {
        memset(
            (p as *mut libc::c_char).offset(old_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            new_size.wrapping_sub(old_size),
        );
    }
    return p;
}
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh0 = (*xb).buf;
        *fresh0 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh1 = (*xb).buf;
        *fresh1 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh2 = (*xb).eptr;
    *fresh2 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh3 = (*xb).ptr;
    *fresh3 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn agxbfree(mut xb: *mut agxbuf) {
    if (*xb).dyna != 0 {
        free((*xb).buf as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn agxbmore(mut xb: *mut agxbuf, mut ssz: size_t) {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut nsize: size_t = 0 as libc::c_int as size_t;
    let mut nbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    size = ((*xb).eptr).offset_from((*xb).buf) as libc::c_long as size_t;
    nsize = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    if size.wrapping_add(ssz) > nsize {
        nsize = size.wrapping_add(ssz);
    }
    cnt = ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
    if (*xb).dyna != 0 {
        nbuf = gv_recalloc(
            (*xb).buf as *mut libc::c_void,
            size,
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        nbuf = gv_calloc(
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            nbuf as *mut libc::c_void,
            (*xb).buf as *const libc::c_void,
            cnt,
        );
        (*xb).dyna = 1 as libc::c_int;
    }
    let ref mut fresh4 = (*xb).buf;
    *fresh4 = nbuf;
    let ref mut fresh5 = (*xb).ptr;
    *fresh5 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh6 = (*xb).eptr;
    *fresh6 = ((*xb).buf).offset(nsize as isize);
}
#[inline]
unsafe extern "C" fn agxbput_n(
    mut xb: *mut agxbuf,
    mut s: *const libc::c_char,
    mut ssz: size_t,
) -> size_t {
    if ((*xb).ptr).offset(ssz as isize) > (*xb).eptr {
        agxbmore(xb, ssz);
    }
    memcpy(
        (*xb).ptr as *mut libc::c_void,
        s as *const libc::c_void,
        ssz,
    );
    let ref mut fresh7 = (*xb).ptr;
    *fresh7 = (*fresh7).offset(ssz as isize);
    return ssz;
}
#[inline]
unsafe extern "C" fn agxbput(mut xb: *mut agxbuf, mut s: *const libc::c_char) -> size_t {
    let mut ssz: size_t = strlen(s);
    return agxbput_n(xb, s, ssz);
}
#[inline]
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh8 = (*xb).ptr;
    let fresh9 = *fresh8;
    *fresh8 = (*fresh8).offset(1);
    *fresh9 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn agxblen(mut xb: *const agxbuf) -> size_t {
    return ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn agxbclear(mut xb: *mut agxbuf) {
    let ref mut fresh11 = (*xb).ptr;
    *fresh11 = (*xb).buf;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut state: lexstate_t = lexstate_t {
    parser: 0 as *const XML_ParserStruct as *mut XML_ParserStruct,
    ptr: 0 as *const libc::c_char as *mut libc::c_char,
    tok: 0,
    xb: 0 as *const agxbuf as *mut agxbuf,
    lb: agxbuf {
        buf: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        eptr: 0 as *const libc::c_char as *mut libc::c_char,
        dyna: 0,
    },
    warn: 0,
    error: 0,
    inCell: 0,
    mode: 0,
    currtok: 0 as *const libc::c_char as *mut libc::c_char,
    prevtok: 0 as *const libc::c_char as *mut libc::c_char,
    currtoklen: 0,
    prevtoklen: 0,
};
unsafe extern "C" fn error_context() {
    agxbclear(state.xb);
    if state.prevtoklen > 0 as libc::c_int as libc::c_ulong {
        agxbput_n(state.xb, state.prevtok, state.prevtoklen);
    }
    agxbput_n(state.xb, state.currtok, state.currtoklen);
    agerr(
        AGPREV,
        b"... %s ...\n\0" as *const u8 as *const libc::c_char,
        agxbuse(state.xb),
    );
}
#[no_mangle]
pub unsafe extern "C" fn htmlerror(mut msg: *const libc::c_char) {
    if state.error != 0 {
        return;
    }
    state.error = 1 as libc::c_int;
    agerr(
        AGERR,
        b"%s in line %d \n\0" as *const u8 as *const libc::c_char,
        msg,
        htmllineno(),
    );
    error_context();
}
unsafe extern "C" fn lexerror(mut name: *const libc::c_char) {
    state.tok = 268 as libc::c_int;
    state.error = 1 as libc::c_int;
    agerr(
        AGERR,
        b"Unknown HTML element <%s> on line %d \n\0" as *const u8 as *const libc::c_char,
        name,
        htmllineno(),
    );
}
unsafe extern "C" fn icmp(mut i: *mut attr_item, mut j: *mut attr_item) -> libc::c_int {
    return strcasecmp((*i).name, (*j).name);
}
unsafe extern "C" fn bgcolorfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh12 = (*p).bgcolor;
    *fresh12 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn pencolorfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh13 = (*p).pencolor;
    *fresh13 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn hreffn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh14 = (*p).href;
    *fresh14 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn sidesfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut flags: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut c: libc::c_char = 0;
    loop {
        let fresh15 = v;
        v = v.offset(1);
        c = *fresh15;
        if !(c != 0) {
            break;
        }
        match ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = c as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(c as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(c as libc::c_int as isize);
            }
            __res
        }) {
            108 => {
                flags = (flags as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int)
                    as libc::c_ushort;
            }
            116 => {
                flags = (flags as libc::c_int | (1 as libc::c_int) << 11 as libc::c_int)
                    as libc::c_ushort;
            }
            114 => {
                flags = (flags as libc::c_int | (1 as libc::c_int) << 12 as libc::c_int)
                    as libc::c_ushort;
            }
            98 => {
                flags = (flags as libc::c_int | (1 as libc::c_int) << 13 as libc::c_int)
                    as libc::c_ushort;
            }
            _ => {
                agerr(
                    AGWARN,
                    b"Unrecognized character '%c' (%d) in sides attribute\n\0" as *const u8
                        as *const libc::c_char,
                    c as libc::c_int,
                    c as libc::c_int,
                );
            }
        }
    }
    if flags as libc::c_int
        != (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int
            | (1 as libc::c_int) << 13 as libc::c_int
    {
        let ref mut fresh16 = (*p).flags;
        *fresh16 = (*fresh16 as libc::c_int | flags as libc::c_int) as libc::c_ushort;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn titlefn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh17 = (*p).title;
    *fresh17 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn portfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh18 = (*p).port;
    *fresh18 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn stylefn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut tk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = strdup(v);
    tk = strtok(buf, b" ,\0" as *const u8 as *const libc::c_char);
    while !tk.is_null() {
        if strcasecmp(tk, b"ROUNDED\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh19 = (*p).style;
            *fresh19 = (*fresh19 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
                as libc::c_ushort;
        } else if strcasecmp(tk, b"RADIAL\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh20 = (*p).style;
            *fresh20 = (*fresh20 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_ushort;
        } else if strcasecmp(tk, b"SOLID\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh21 = (*p).style;
            *fresh21 = (*fresh21 as libc::c_int
                & !((1 as libc::c_int) << 7 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int)
                    as libc::c_ushort as libc::c_int) as libc::c_ushort;
        } else if strcasecmp(tk, b"INVISIBLE\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(tk, b"INVIS\0" as *const u8 as *const libc::c_char) == 0
        {
            let ref mut fresh22 = (*p).style;
            *fresh22 = (*fresh22 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int)
                as libc::c_ushort;
        } else if strcasecmp(tk, b"DOTTED\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh23 = (*p).style;
            *fresh23 = (*fresh23 as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int)
                as libc::c_ushort;
        } else if strcasecmp(tk, b"DASHED\0" as *const u8 as *const libc::c_char) == 0 {
            let ref mut fresh24 = (*p).style;
            *fresh24 = (*fresh24 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int)
                as libc::c_ushort;
        } else {
            agerr(
                AGWARN,
                b"Illegal value %s for STYLE - ignored\n\0" as *const u8 as *const libc::c_char,
                tk,
            );
            rv = 1 as libc::c_int;
        }
        tk = strtok(
            0 as *mut libc::c_char,
            b" ,\0" as *const u8 as *const libc::c_char,
        );
    }
    free(buf as *mut libc::c_void);
    return rv;
}
unsafe extern "C" fn targetfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh25 = (*p).target;
    *fresh25 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn idfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh26 = (*p).id;
    *fresh26 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn doInt(
    mut v: *mut libc::c_char,
    mut s: *mut libc::c_char,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut ul: *mut libc::c_long,
) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: libc::c_long = strtol(v, &mut ep, 10 as libc::c_int);
    if ep == v {
        agerr(
            AGWARN,
            b"Improper %s value %s - ignored\0" as *const u8 as *const libc::c_char,
            s,
            v,
        );
        rv = 1 as libc::c_int;
    } else if b > max as libc::c_long {
        agerr(
            AGWARN,
            b"%s value %s > %d - too large - ignored\0" as *const u8 as *const libc::c_char,
            s,
            v,
            max,
        );
        rv = 1 as libc::c_int;
    } else if b < min as libc::c_long {
        agerr(
            AGWARN,
            b"%s value %s < %d - too small - ignored\0" as *const u8 as *const libc::c_char,
            s,
            v,
            min,
        );
        rv = 1 as libc::c_int;
    } else {
        *ul = b;
    }
    return rv;
}
unsafe extern "C" fn gradientanglefn(
    mut p: *mut htmldata_t,
    mut v: *mut libc::c_char,
) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"GRADIENTANGLE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        360 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).gradientangle = u as libc::c_ushort as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn borderfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"BORDER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).border = u as libc::c_uchar;
    let ref mut fresh27 = (*p).flags;
    *fresh27 = (*fresh27 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int) as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cellpaddingfn(
    mut p: *mut htmldata_t,
    mut v: *mut libc::c_char,
) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"CELLPADDING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).pad = u as libc::c_uchar;
    let ref mut fresh28 = (*p).flags;
    *fresh28 = (*fresh28 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cellspacingfn(
    mut p: *mut htmldata_t,
    mut v: *mut libc::c_char,
) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"CELLSPACING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        -(127 as libc::c_int) - 1 as libc::c_int,
        127 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).space = u as libc::c_schar;
    let ref mut fresh29 = (*p).flags;
    *fresh29 = (*fresh29 as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int) as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cellborderfn(mut p: *mut htmltbl_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"CELLSBORDER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        127 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).cb = u as libc::c_schar;
    return 0 as libc::c_int;
}
unsafe extern "C" fn columnsfn(mut p: *mut htmltbl_t, mut v: *mut libc::c_char) -> libc::c_int {
    if *v as libc::c_int != '*' as i32 {
        agerr(
            AGWARN,
            b"Unknown value %s for COLUMNS - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
        return 1 as libc::c_int;
    }
    let ref mut fresh30 = (*p).flags;
    *fresh30 = (*fresh30 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
    return 0 as libc::c_int;
}
unsafe extern "C" fn rowsfn(mut p: *mut htmltbl_t, mut v: *mut libc::c_char) -> libc::c_int {
    if *v as libc::c_int != '*' as i32 {
        agerr(
            AGWARN,
            b"Unknown value %s for ROWS - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
        return 1 as libc::c_int;
    }
    let ref mut fresh31 = (*p).flags;
    *fresh31 = (*fresh31 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fixedsizefn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    if strcasecmp(v, b"TRUE\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh32 = (*p).flags;
        *fresh32 = (*fresh32 as libc::c_int | 1 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"FALSE\0" as *const u8 as *const libc::c_char) != 0 {
        agerr(
            AGWARN,
            b"Illegal value %s for FIXEDSIZE - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
        rv = 1 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn valignfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    if strcasecmp(v, b"BOTTOM\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh33 = (*p).flags;
        *fresh33 =
            (*fresh33 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"TOP\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh34 = (*p).flags;
        *fresh34 =
            (*fresh34 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"MIDDLE\0" as *const u8 as *const libc::c_char) != 0 {
        agerr(
            AGWARN,
            b"Illegal value %s for VALIGN - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
        rv = 1 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn halignfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    if strcasecmp(v, b"LEFT\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh35 = (*p).flags;
        *fresh35 =
            (*fresh35 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"RIGHT\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh36 = (*p).flags;
        *fresh36 =
            (*fresh36 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"CENTER\0" as *const u8 as *const libc::c_char) != 0 {
        agerr(
            AGWARN,
            b"Illegal value %s for ALIGN - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
        rv = 1 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn cell_halignfn(
    mut p: *mut htmldata_t,
    mut v: *mut libc::c_char,
) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    if strcasecmp(v, b"LEFT\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh37 = (*p).flags;
        *fresh37 =
            (*fresh37 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"RIGHT\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh38 = (*p).flags;
        *fresh38 =
            (*fresh38 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"TEXT\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh39 = (*p).flags;
        *fresh39 = (*fresh39 as libc::c_int
            | ((1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int))
            as libc::c_ushort;
    } else if strcasecmp(v, b"CENTER\0" as *const u8 as *const libc::c_char) != 0 {
        rv = 1 as libc::c_int;
    }
    if rv != 0 {
        agerr(
            AGWARN,
            b"Illegal value %s for ALIGN in TD - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
    }
    return rv;
}
unsafe extern "C" fn balignfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    if strcasecmp(v, b"LEFT\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh40 = (*p).flags;
        *fresh40 =
            (*fresh40 as libc::c_int | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"RIGHT\0" as *const u8 as *const libc::c_char) == 0 {
        let ref mut fresh41 = (*p).flags;
        *fresh41 =
            (*fresh41 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int) as libc::c_ushort;
    } else if strcasecmp(v, b"CENTER\0" as *const u8 as *const libc::c_char) != 0 {
        rv = 1 as libc::c_int;
    }
    if rv != 0 {
        agerr(
            AGWARN,
            b"Illegal value %s for BALIGN in TD - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
    }
    return rv;
}
unsafe extern "C" fn heightfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"HEIGHT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).height = u as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn widthfn(mut p: *mut htmldata_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"WIDTH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).width = u as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn rowspanfn(mut p: *mut htmlcell_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"ROWSPAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if u == 0 as libc::c_int as libc::c_long {
        agerr(
            AGWARN,
            b"ROWSPAN value cannot be 0 - ignored\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*p).rspan = u as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn colspanfn(mut p: *mut htmlcell_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"COLSPAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if u == 0 as libc::c_int as libc::c_long {
        agerr(
            AGWARN,
            b"COLSPAN value cannot be 0 - ignored\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*p).cspan = u as libc::c_ushort;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fontcolorfn(mut p: *mut textfont_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh42 = (*p).color;
    *fresh42 = v;
    return 0 as libc::c_int;
}
unsafe extern "C" fn facefn(mut p: *mut textfont_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh43 = (*p).name;
    *fresh43 = v;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ptsizefn(mut p: *mut textfont_t, mut v: *mut libc::c_char) -> libc::c_int {
    let mut u: libc::c_long = 0;
    if doInt(
        v,
        b"POINT-SIZE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        &mut u,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    (*p).size = u as libc::c_double;
    return 0 as libc::c_int;
}
unsafe extern "C" fn srcfn(mut p: *mut htmlimg_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh44 = (*p).src;
    *fresh44 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn scalefn(mut p: *mut htmlimg_t, mut v: *mut libc::c_char) -> libc::c_int {
    let ref mut fresh45 = (*p).scale;
    *fresh45 = strdup(v);
    return 0 as libc::c_int;
}
unsafe extern "C" fn alignfn(mut p: *mut libc::c_int, mut v: *mut libc::c_char) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    if strcasecmp(v, b"RIGHT\0" as *const u8 as *const libc::c_char) == 0 {
        *p = 'r' as i32;
    } else if strcasecmp(v, b"LEFT\0" as *const u8 as *const libc::c_char) == 0 {
        *p = 'l' as i32;
    } else if strcasecmp(v, b"CENTER\0" as *const u8 as *const libc::c_char) == 0 {
        *p = 'n' as i32;
    } else {
        agerr(
            AGWARN,
            b"Illegal value %s for ALIGN - ignored\n\0" as *const u8 as *const libc::c_char,
            v,
        );
        rv = 1 as libc::c_int;
    }
    return rv;
}
static mut tbl_items: [attr_item; 22] = unsafe {
    [
        {
            let mut init = attr_item {
                name: b"align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    halignfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    bgcolorfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"border\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    borderfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"cellborder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmltbl_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    cellborderfn
                        as unsafe extern "C" fn(*mut htmltbl_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"cellpadding\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    cellpaddingfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"cellspacing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    cellspacingfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    pencolorfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"columns\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmltbl_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    columnsfn
                        as unsafe extern "C" fn(*mut htmltbl_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"fixedsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    fixedsizefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"gradientangle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    gradientanglefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    heightfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"href\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    hreffn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    idfn as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"port\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    portfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"rows\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmltbl_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    rowsfn
                        as unsafe extern "C" fn(*mut htmltbl_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"sides\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    sidesfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    stylefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"target\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    targetfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"title\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    titlefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"tooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    titlefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"valign\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    valignfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    widthfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
    ]
};
static mut cell_items: [attr_item; 22] = unsafe {
    [
        {
            let mut init = attr_item {
                name: b"align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    cell_halignfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"balign\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    balignfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    bgcolorfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"border\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    borderfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"cellpadding\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    cellpaddingfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"cellspacing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    cellspacingfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    pencolorfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"colspan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmlcell_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    colspanfn
                        as unsafe extern "C" fn(*mut htmlcell_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"fixedsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    fixedsizefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"gradientangle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    gradientanglefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    heightfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"href\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    hreffn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    idfn as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"port\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    portfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"rowspan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmlcell_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    rowspanfn
                        as unsafe extern "C" fn(*mut htmlcell_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"sides\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    sidesfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    stylefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"target\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    targetfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"title\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    titlefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"tooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    titlefn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"valign\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    valignfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    widthfn
                        as unsafe extern "C" fn(*mut htmldata_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
    ]
};
static mut font_items: [attr_item; 3] = unsafe {
    [
        {
            let mut init = attr_item {
                name: b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut textfont_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    fontcolorfn
                        as unsafe extern "C" fn(*mut textfont_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"face\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut textfont_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    facefn
                        as unsafe extern "C" fn(*mut textfont_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"point-size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut textfont_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    ptsizefn
                        as unsafe extern "C" fn(*mut textfont_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
    ]
};
static mut img_items: [attr_item; 2] = unsafe {
    [
        {
            let mut init = attr_item {
                name: b"scale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmlimg_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    scalefn
                        as unsafe extern "C" fn(*mut htmlimg_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
        {
            let mut init = attr_item {
                name: b"src\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                action: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut htmlimg_t, *mut libc::c_char) -> libc::c_int>,
                    attrFn,
                >(Some(
                    srcfn as unsafe extern "C" fn(*mut htmlimg_t, *mut libc::c_char) -> libc::c_int,
                )),
            };
            init
        },
    ]
};
static mut br_items: [attr_item; 1] = unsafe {
    [{
        let mut init = attr_item {
            name: b"align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            action: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_char) -> libc::c_int>,
                attrFn,
            >(Some(
                alignfn as unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_char) -> libc::c_int,
            )),
        };
        init
    }]
};
unsafe extern "C" fn doAttrs(
    mut tp: *mut libc::c_void,
    mut items: *mut attr_item,
    mut nel: libc::c_int,
    mut atts: *mut *mut libc::c_char,
    mut s: *mut libc::c_char,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: *mut attr_item = 0 as *mut attr_item;
    let mut key: attr_item = attr_item {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        action: None,
    };
    loop {
        let fresh46 = atts;
        atts = atts.offset(1);
        name = *fresh46;
        if name.is_null() {
            break;
        }
        let fresh47 = atts;
        atts = atts.offset(1);
        val = *fresh47;
        key.name = name;
        ip = bsearch(
            &mut key as *mut attr_item as *const libc::c_void,
            items as *const libc::c_void,
            nel as size_t,
            ::std::mem::size_of::<attr_item>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut attr_item, *mut attr_item) -> libc::c_int>,
                bcmpfn,
            >(Some(
                icmp as unsafe extern "C" fn(*mut attr_item, *mut attr_item) -> libc::c_int,
            )),
        ) as *mut attr_item;
        if !ip.is_null() {
            state.warn |= ((*ip).action).expect("non-null function pointer")(tp, val);
        } else {
            agerr(
                AGWARN,
                b"Illegal attribute %s in %s - ignored\n\0" as *const u8 as *const libc::c_char,
                name,
                s,
            );
            state.warn = 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn mkBR(mut atts: *mut *mut libc::c_char) {
    htmllval.i = 0 as libc::c_int;
    doAttrs(
        &mut htmllval.i as *mut libc::c_int as *mut libc::c_void,
        br_items.as_mut_ptr(),
        (::std::mem::size_of::<[attr_item; 1]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<attr_item>() as libc::c_ulong)
            as libc::c_int,
        atts,
        b"<BR>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn mkImg(mut atts: *mut *mut libc::c_char) -> *mut htmlimg_t {
    let mut img: *mut htmlimg_t =
        zmalloc(::std::mem::size_of::<htmlimg_t>() as libc::c_ulong) as *mut htmlimg_t;
    doAttrs(
        img as *mut libc::c_void,
        img_items.as_mut_ptr(),
        (::std::mem::size_of::<[attr_item; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<attr_item>() as libc::c_ulong)
            as libc::c_int,
        atts,
        b"<IMG>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return img;
}
unsafe extern "C" fn mkFont(
    mut gvc: *mut GVC_t,
    mut atts: *mut *mut libc::c_char,
    mut flags: libc::c_uchar,
) -> *mut textfont_t {
    let mut tf: textfont_t = {
        let mut init = textfont_t {
            flags_cnt: [0; 4],
            c2rust_padding: [0; 4],
            name: 0 as *mut libc::c_char,
            color: 0 as *mut libc::c_char,
            postscript_alias: 0 as *mut PostscriptAlias,
            size: 0.0f64,
        };
        init.set_flags(0 as libc::c_int as libc::c_uint);
        init.set_cnt(0 as libc::c_int as libc::c_uint);
        init
    };
    tf.size = -1.0f64;
    if flags as libc::c_int <= FLAGS_MAX as libc::c_int {
    } else {
        __assert_fail(
            b"flags <= FLAGS_MAX\0" as *const u8 as *const libc::c_char,
            b"htmllex.c\0" as *const u8 as *const libc::c_char,
            589 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"textfont_t *mkFont(GVC_t *, char **, unsigned char)\0",
            ))
            .as_ptr(),
        );
    }
    tf.set_flags(
        (flags as libc::c_int & FLAGS_MAX as libc::c_int) as libc::c_uchar as libc::c_uint,
    );
    if !atts.is_null() {
        doAttrs(
            &mut tf as *mut textfont_t as *mut libc::c_void,
            font_items.as_mut_ptr(),
            (::std::mem::size_of::<[attr_item; 3]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<attr_item>() as libc::c_ulong)
                as libc::c_int,
            atts,
            b"<FONT>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return (Some(((*(*gvc).textfont_dt).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        (*gvc).textfont_dt,
        &mut tf as *mut textfont_t as *mut libc::c_void,
        0o1 as libc::c_int,
    ) as *mut textfont_t;
}
unsafe extern "C" fn mkCell(mut atts: *mut *mut libc::c_char) -> *mut htmlcell_t {
    let mut cell: *mut htmlcell_t =
        zmalloc(::std::mem::size_of::<htmlcell_t>() as libc::c_ulong) as *mut htmlcell_t;
    (*cell).cspan = 1 as libc::c_int as libc::c_ushort;
    (*cell).rspan = 1 as libc::c_int as libc::c_ushort;
    doAttrs(
        cell as *mut libc::c_void,
        cell_items.as_mut_ptr(),
        (::std::mem::size_of::<[attr_item; 22]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<attr_item>() as libc::c_ulong)
            as libc::c_int,
        atts,
        b"<TD>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return cell;
}
unsafe extern "C" fn mkTbl(mut atts: *mut *mut libc::c_char) -> *mut htmltbl_t {
    let mut tbl: *mut htmltbl_t =
        zmalloc(::std::mem::size_of::<htmltbl_t>() as libc::c_ulong) as *mut htmltbl_t;
    (*tbl).rc = -(1 as libc::c_int);
    (*tbl).cb = -(1 as libc::c_int) as libc::c_schar;
    doAttrs(
        tbl as *mut libc::c_void,
        tbl_items.as_mut_ptr(),
        (::std::mem::size_of::<[attr_item; 22]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<attr_item>() as libc::c_ulong)
            as libc::c_int,
        atts,
        b"<TABLE>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return tbl;
}
unsafe extern "C" fn startElement(
    mut user: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut atts: *mut *mut libc::c_char,
) {
    let mut gvc: *mut GVC_t = user as *mut GVC_t;
    if strcasecmp(name, b"TABLE\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.tbl = mkTbl(atts);
        state.inCell = 0 as libc::c_int as libc::c_char;
        state.tok = 286 as libc::c_int;
    } else if strcasecmp(name, b"TR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcasecmp(name, b"TH\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        state.inCell = 0 as libc::c_int as libc::c_char;
        state.tok = 260 as libc::c_int;
    } else if strcasecmp(name, b"TD\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.inCell = 1 as libc::c_int as libc::c_char;
        htmllval.cell = mkCell(atts);
        state.tok = 287 as libc::c_int;
    } else if strcasecmp(name, b"FONT\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(gvc, atts, 0 as libc::c_int as libc::c_uchar);
        state.tok = 288 as libc::c_int;
    } else if strcasecmp(name, b"B\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(
            gvc,
            0 as *mut *mut libc::c_char,
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
        );
        state.tok = 290 as libc::c_int;
    } else if strcasecmp(name, b"S\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(
            gvc,
            0 as *mut *mut libc::c_char,
            ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uchar,
        );
        state.tok = 295 as libc::c_int;
    } else if strcasecmp(name, b"U\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(
            gvc,
            0 as *mut *mut libc::c_char,
            ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
        );
        state.tok = 291 as libc::c_int;
    } else if strcasecmp(name, b"O\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(
            gvc,
            0 as *mut *mut libc::c_char,
            ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uchar,
        );
        state.tok = 292 as libc::c_int;
    } else if strcasecmp(name, b"I\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(
            gvc,
            0 as *mut *mut libc::c_char,
            ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
        );
        state.tok = 289 as libc::c_int;
    } else if strcasecmp(name, b"SUP\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(
            gvc,
            0 as *mut *mut libc::c_char,
            ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uchar,
        );
        state.tok = 293 as libc::c_int;
    } else if strcasecmp(name, b"SUB\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.font = mkFont(
            gvc,
            0 as *mut *mut libc::c_char,
            ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uchar,
        );
        state.tok = 294 as libc::c_int;
    } else if strcasecmp(name, b"BR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        mkBR(atts);
        state.tok = 283 as libc::c_int;
    } else if strcasecmp(name, b"HR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 277 as libc::c_int;
    } else if strcasecmp(name, b"VR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 280 as libc::c_int;
    } else if strcasecmp(name, b"IMG\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        htmllval.img = mkImg(atts);
        state.tok = 285 as libc::c_int;
    } else if strcasecmp(name, b"HTML\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 262 as libc::c_int;
    } else {
        lexerror(name);
    };
}
unsafe extern "C" fn endElement(mut user: *mut libc::c_void, mut name: *const libc::c_char) {
    if strcasecmp(name, b"TABLE\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 264 as libc::c_int;
        state.inCell = 1 as libc::c_int as libc::c_char;
    } else if strcasecmp(name, b"TR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcasecmp(name, b"TH\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        state.tok = 261 as libc::c_int;
    } else if strcasecmp(name, b"TD\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 265 as libc::c_int;
        state.inCell = 0 as libc::c_int as libc::c_char;
    } else if strcasecmp(name, b"HTML\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 263 as libc::c_int;
    } else if strcasecmp(name, b"FONT\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 266 as libc::c_int;
    } else if strcasecmp(name, b"B\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 270 as libc::c_int;
    } else if strcasecmp(name, b"U\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 271 as libc::c_int;
    } else if strcasecmp(name, b"O\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 272 as libc::c_int;
    } else if strcasecmp(name, b"I\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 269 as libc::c_int;
    } else if strcasecmp(name, b"SUP\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 273 as libc::c_int;
    } else if strcasecmp(name, b"SUB\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 274 as libc::c_int;
    } else if strcasecmp(name, b"S\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        state.tok = 275 as libc::c_int;
    } else if strcasecmp(name, b"BR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if state.tok == 283 as libc::c_int {
            state.tok = 282 as libc::c_int;
        } else {
            state.tok = 258 as libc::c_int;
        }
    } else if strcasecmp(name, b"HR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if state.tok == 277 as libc::c_int {
            state.tok = 276 as libc::c_int;
        } else {
            state.tok = 278 as libc::c_int;
        }
    } else if strcasecmp(name, b"VR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if state.tok == 280 as libc::c_int {
            state.tok = 279 as libc::c_int;
        } else {
            state.tok = 281 as libc::c_int;
        }
    } else if strcasecmp(name, b"IMG\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if state.tok == 285 as libc::c_int {
            state.tok = 284 as libc::c_int;
        } else {
            state.tok = 259 as libc::c_int;
        }
    } else {
        lexerror(name);
    };
}
unsafe extern "C" fn characterData(
    mut user: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_uchar = 0;
    if state.inCell != 0 {
        i = length;
        while i != 0 {
            let fresh48 = s;
            s = s.offset(1);
            c = *fresh48 as libc::c_uchar;
            if c as libc::c_int >= ' ' as i32 {
                cnt += 1;
                agxbputc(state.xb, c as libc::c_char);
            }
            i -= 1;
        }
        if cnt != 0 {
            state.tok = 267 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn initHTMLlexer(
    mut src: *mut libc::c_char,
    mut xb: *mut agxbuf,
    mut env: *mut htmlenv_t,
) -> libc::c_int {
    state.xb = xb;
    agxbinit(
        &mut state.lb,
        128 as libc::c_int as libc::c_uint,
        0 as *mut libc::c_char,
    );
    state.ptr = src;
    state.mode = 0 as libc::c_int as libc::c_char;
    state.warn = 0 as libc::c_int;
    state.error = 0 as libc::c_int;
    state.currtoklen = 0 as libc::c_int as size_t;
    state.prevtoklen = 0 as libc::c_int as size_t;
    state.inCell = 1 as libc::c_int as libc::c_char;
    state.parser = XML_ParserCreate(charsetToStr(
        (*((*((*env).g as *mut Agobj_t)).data as *mut Agraphinfo_t)).charset as libc::c_int,
    ));
    XML_SetUserData(
        state.parser,
        (*((*((*env).g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc as *mut libc::c_void,
    );
    XML_SetElementHandler(
        state.parser,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *mut *mut libc::c_char,
                ) -> (),
            >,
            XML_StartElementHandler,
        >(Some(
            startElement
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *mut *mut libc::c_char,
                ) -> (),
        )),
        Some(endElement as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> ()),
    );
    XML_SetCharacterDataHandler(
        state.parser,
        Some(
            characterData
                as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> (),
        ),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clearHTMLlexer() -> libc::c_int {
    let mut rv: libc::c_int = state.warn | state.error;
    XML_ParserFree(state.parser);
    agxbfree(&mut state.lb);
    return rv;
}
unsafe extern "C" fn eatComment(mut p: *mut libc::c_char) -> *mut libc::c_char {
    let mut depth: libc::c_int = 1 as libc::c_int;
    let mut s: *mut libc::c_char = p;
    let mut c: libc::c_char = 0;
    while depth != 0 && {
        let fresh49 = s;
        s = s.offset(1);
        c = *fresh49;
        c as libc::c_int != 0
    } {
        if c as libc::c_int == '<' as i32 {
            depth += 1;
        } else if c as libc::c_int == '>' as i32 {
            depth -= 1;
        }
    }
    s = s.offset(-1);
    if *s != 0 {
        let mut t: *mut libc::c_char = s.offset(-(2 as libc::c_int as isize));
        if t < p
            || strncmp(
                t,
                b"--\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) != 0
        {
            agerr(
                AGWARN,
                b"Unclosed comment\n\0" as *const u8 as *const libc::c_char,
            );
            state.warn = 1 as libc::c_int;
        }
    }
    return s;
}
unsafe extern "C" fn findNext(mut s: *mut libc::c_char, mut xb: *mut agxbuf) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = s.offset(1 as libc::c_int as isize);
    let mut c: libc::c_char = 0;
    if *s as libc::c_int == '<' as i32 {
        if strncmp(
            t,
            b"!--\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            t = eatComment(t.offset(3 as libc::c_int as isize));
        } else {
            while *t as libc::c_int != 0 && *t as libc::c_int != '>' as i32 {
                t = t.offset(1);
            }
        }
        if *t as libc::c_int != '>' as i32 {
            agerr(
                AGWARN,
                b"Label closed before end of HTML element\n\0" as *const u8 as *const libc::c_char,
            );
            state.warn = 1 as libc::c_int;
        } else {
            t = t.offset(1);
        }
    } else {
        t = s;
        loop {
            c = *t;
            if !(c as libc::c_int != 0 && c as libc::c_int != '<' as i32) {
                break;
            }
            if c as libc::c_int == '&' as i32
                && *t.offset(1 as libc::c_int as isize) as libc::c_int != '#' as i32
            {
                t = scanEntity(t.offset(1 as libc::c_int as isize), xb);
            } else {
                agxbputc(xb, c);
                t = t.offset(1);
            }
        }
    }
    return t;
}
unsafe extern "C" fn protect_rsqb(mut xb: *mut agxbuf) {
    if (*xb).buf == (*xb).ptr {
        return;
    }
    if *((*xb).ptr).offset(-(1 as libc::c_int) as isize) as libc::c_int != ']' as i32 {
        return;
    }
    let ref mut fresh50 = (*xb).ptr;
    *fresh50 = (*fresh50).offset(-1);
    agxbput(xb, b"&#93;\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn htmllineno() -> libc::c_int {
    return XML_GetCurrentLineNumber(state.parser) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htmllex() -> libc::c_int {
    static mut begin_html: *mut libc::c_char =
        b"<HTML>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    static mut end_html: *mut libc::c_char =
        b"</HTML>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut llen: size_t = 0;
    let mut rv: libc::c_int = 0;
    state.tok = 0 as libc::c_int;
    loop {
        if state.mode as libc::c_int == 2 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if state.mode as libc::c_int == 0 as libc::c_int {
            state.mode = 1 as libc::c_int as libc::c_char;
            s = begin_html;
            len = strlen(s);
            endp = 0 as *mut libc::c_char;
        } else {
            s = state.ptr;
            if *s as libc::c_int == '\0' as i32 {
                state.mode = 2 as libc::c_int as libc::c_char;
                s = end_html;
                len = strlen(s);
            } else {
                endp = findNext(s, &mut state.lb);
                len = endp.offset_from(s) as libc::c_long as size_t;
            }
        }
        protect_rsqb(&mut state.lb);
        state.prevtok = state.currtok;
        state.prevtoklen = state.currtoklen;
        state.currtok = s;
        state.currtoklen = len;
        llen = agxblen(&mut state.lb);
        if llen != 0 {
            if llen <= 2147483647 as libc::c_int as size_t
                && !(b"XML token too long for expat API\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"llen <= (size_t)INT_MAX && \"XML token too long for expat API\"\0"
                        as *const u8 as *const libc::c_char,
                    b"htmllex.c\0" as *const u8 as *const libc::c_char,
                    1081 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"int htmllex()\0"))
                        .as_ptr(),
                );
            }
            rv = XML_Parse(
                state.parser,
                agxbuse(&mut state.lb),
                llen as libc::c_int,
                0 as libc::c_int,
            ) as libc::c_int;
        } else {
            if len <= 2147483647 as libc::c_int as size_t
                && !(b"XML token too long for expat API\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"len <= (size_t)INT_MAX && \"XML token too long for expat API\"\0" as *const u8
                        as *const libc::c_char,
                    b"htmllex.c\0" as *const u8 as *const libc::c_char,
                    1084 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"int htmllex()\0"))
                        .as_ptr(),
                );
            }
            rv = XML_Parse(
                state.parser,
                s,
                len as libc::c_int,
                if len != 0 {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                },
            ) as libc::c_int;
        }
        if rv == XML_STATUS_ERROR as libc::c_int {
            if state.error == 0 {
                agerr(
                    AGERR,
                    b"%s in line %d \n\0" as *const u8 as *const libc::c_char,
                    XML_ErrorString(XML_GetErrorCode(state.parser)),
                    htmllineno(),
                );
                error_context();
                state.error = 1 as libc::c_int;
                state.tok = 268 as libc::c_int;
            }
        }
        if !endp.is_null() {
            state.ptr = endp;
        }
        if !(state.tok == 0 as libc::c_int) {
            break;
        }
    }
    return state.tok;
}
