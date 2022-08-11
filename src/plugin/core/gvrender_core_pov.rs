#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_engine_s {
    pub initialize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub format: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub finalize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvrender_engine_s {
    pub begin_job: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_job: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_graph: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_graph: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_layer: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub end_layer: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_page: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_page: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_cluster: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_cluster: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_nodes: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_nodes: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edges: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edges: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_node: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_node: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edge: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edge: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_anchor: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub end_anchor: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_label: Option::<unsafe extern "C" fn(*mut GVJ_t, label_type) -> ()>,
    pub end_label: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub textspan: Option::<
        unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
    >,
    pub resolve_color: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> ()>,
    pub ellipse: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
    >,
    pub polygon: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int, libc::c_int) -> (),
    >,
    pub beziercurve: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut pointf,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub polyline: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
    >,
    pub comment: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> ()>,
    pub library_shape: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut pointf,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
}
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
pub type PostscriptAlias = _PostscriptAlias;
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
pub type label_type = libc::c_uint;
pub const LABEL_HTML: label_type = 1;
pub const LABEL_PLAIN: label_type = 0;
pub type obj_state_t = obj_state_s;
#[derive(Copy, Clone, BitfieldStruct)]
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
pub type Dtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
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
pub type uint64_t = __uint64_t;
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FORMAT_POV: C2RustUnnamed_4 = 0;
static mut pov_knowncolors: [*mut libc::c_char; 122] = [
    b"aquamarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bakerschoc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blueviolet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"brass\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"brightgold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bronze\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bronze2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"brown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cadetblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"clear\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"coolcopper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"copper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"coral\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cornflowerblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkbrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkolivegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkorchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkpurple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkslateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkslategray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkslategrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darktan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkwood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dkgreencopper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dustyrose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"feldspar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"firebrick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"flesh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"forestgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"goldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray05\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray25\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray30\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray35\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray40\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray45\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray50\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray55\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray60\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray65\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray70\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray75\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray80\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray85\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray90\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray95\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"green\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"greencopper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"greenyellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"huntersgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"indianred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"khaki\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"light_purple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightsteelblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightwood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"limegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"magenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mandarinorange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"maroon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumaquamarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumforestgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumgoldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumorchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumseagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumslateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumspringgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumvioletred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumwood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"med_purple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"midnightblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"navy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"navyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"neonblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"neonpink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"newmidnightblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"newtan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"oldgold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"orange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"orangered\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"orchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"palegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"plum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"quartz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"richblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"salmon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"scarlet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"seagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"semiSweetChoc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sienna\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"silver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"skyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"slateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"spicypink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"springgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"steelblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"summersky\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"thistle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"turquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"verydarkbrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"very_light_purple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"violet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"violetred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wheat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yellowgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut layerz: libc::c_float = 0 as libc::c_int as libc::c_float;
static mut z: libc::c_float = 0 as libc::c_int as libc::c_float;
unsafe extern "C" fn el(
    mut job: *mut GVJ_t,
    mut template: *mut libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arglist: ::std::ffi::VaListImpl;
    let mut arglist2: ::std::ffi::VaListImpl;
    arglist = args.clone();
    arglist2 = arglist.clone();
    len = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        template,
        arglist.as_va_list(),
    );
    if len < 0 as libc::c_int {
        ((*(*job).common).errorfn)
            .expect(
                "non-null function pointer",
            )(
            b"pov renderer:el - %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        str = strdup(b"\0" as *const u8 as *const libc::c_char);
    } else {
        str = malloc((len as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        vsprintf(str, template, arglist2.as_va_list());
    }
    return str;
}
unsafe extern "C" fn pov_color_as_str(
    mut job: *mut GVJ_t,
    mut color: gvcolor_t,
    mut transparency: libc::c_float,
) -> *mut libc::c_char {
    let mut pov: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    match color.type_0 as libc::c_uint {
        5 => {
            if strcmp(color.u.string, b"red\0" as *const u8 as *const libc::c_char) == 0
            {
                c = el(
                    job,
                    b"%s transmit %.3f\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Red\0" as *const u8 as *const libc::c_char,
                    transparency as libc::c_double,
                );
            } else if strcmp(
                    color.u.string,
                    b"green\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                c = el(
                    job,
                    b"%s transmit %.3f\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Green\0" as *const u8 as *const libc::c_char,
                    transparency as libc::c_double,
                );
            } else if strcmp(
                    color.u.string,
                    b"blue\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                c = el(
                    job,
                    b"%s transmit %.3f\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"Blue\0" as *const u8 as *const libc::c_char,
                    transparency as libc::c_double,
                );
            } else {
                c = el(
                    job,
                    b"%s transmit %.3f\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    color.u.string,
                    transparency as libc::c_double,
                );
            }
        }
        1 => {
            c = el(
                job,
                b"rgb<%9.3f, %9.3f, %9.3f> transmit %.3f\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                color.u.rgba[0 as libc::c_int as usize] as libc::c_int as libc::c_double
                    / 256.0f64,
                color.u.rgba[1 as libc::c_int as usize] as libc::c_int as libc::c_double
                    / 256.0f64,
                color.u.rgba[2 as libc::c_int as usize] as libc::c_int as libc::c_double
                    / 256.0f64,
                transparency as libc::c_double,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"oops, internal error: unhandled color type=%d %s\n\0" as *const u8
                    as *const libc::c_char,
                color.type_0 as libc::c_uint,
                color.u.string,
            );
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"gvrender_core_pov.c\0" as *const u8 as *const libc::c_char,
                372 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"char *pov_color_as_str(GVJ_t *, gvcolor_t, float)\0"))
                    .as_ptr(),
            );
        }
    }
    pov = el(
        job,
        b"pigment { color %s }\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c,
    );
    free(c as *mut libc::c_void);
    return pov;
}
unsafe extern "C" fn pov_comment(mut job: *mut GVJ_t, mut str: *mut libc::c_char) {
    gvprintf(job, b"//*** comment: %s\n\0" as *const u8 as *const libc::c_char, str);
}
unsafe extern "C" fn pov_begin_job(mut job: *mut GVJ_t) {
    gvputs(job, b"#version 3.6;\n\0" as *const u8 as *const libc::c_char);
    gvputs(
        job,
        b"global_settings { assumed_gamma 1.0 }\n\0" as *const u8 as *const libc::c_char,
    );
    gvputs(
        job,
        b"#default { finish { ambient 0.1 diffuse 0.9 } }\n\0" as *const u8
            as *const libc::c_char,
    );
    gvputs(
        job,
        b"#include \"colors.inc\"\n#include \"textures.inc\"\n#include \"shapes.inc\"\n\0"
            as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"#declare %s = %s;\n\0" as *const u8 as *const libc::c_char,
        b"black\0" as *const u8 as *const libc::c_char,
        b"Black\0" as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"#declare %s = %s;\n\0" as *const u8 as *const libc::c_char,
        b"white\0" as *const u8 as *const libc::c_char,
        b"White\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn pov_begin_graph(mut job: *mut GVJ_t) {
    gvprintf(
        job,
        b"//*** begin_graph %s\n\0" as *const u8 as *const libc::c_char,
        agnameof((*(*job).obj).u.g as *mut libc::c_void),
    );
    let mut x: libc::c_double = (*job).view.x / 2.0f64 * (*job).scale.x;
    let mut y: libc::c_double = (*job).view.y / 2.0f64 * (*job).scale.y;
    let mut d: libc::c_double = 500 as libc::c_int as libc::c_double;
    let mut px: libc::c_double = atan(x / d) * 180.0f64 / 3.14159265358979323846f64
        * 2.0f64;
    let mut py: libc::c_double = atan(y / d) * 180.0f64 / 3.14159265358979323846f64
        * 2.0f64;
    gvprintf(
        job,
        b"camera { location <%.3f , %.3f , -500.000>\n         look_at  <%.3f , %.3f , 0.000>\n         right x * image_width / image_height\n         angle %.3f\n}\n\0"
            as *const u8 as *const libc::c_char,
        x,
        y,
        x,
        y,
        fmax(px, py) * 1.2f64,
    );
    gvputs(
        job,
        b"//sky\nplane { <0, 1, 0>, 1 hollow\n    texture {\n        pigment { bozo turbulence 0.95\n            color_map {\n                [0.00 rgb <0.05, 0.20, 0.50>]\n                [0.50 rgb <0.05, 0.20, 0.50>]\n                [0.75 rgb <1.00, 1.00, 1.00>]\n                [0.75 rgb <0.25, 0.25, 0.25>]\n                [1.00 rgb <0.50, 0.50, 0.50>]\n            }\n            scale <1.00, 1.00, 1.50> * 2.50\n            translate <0.00, 0.00, 0.00>\n        }\n        finish { ambient 1 diffuse 0 }\n    }\n    scale 10000\n}\n//mist\nfog { fog_type 2\n    distance 50\n    color rgb <1.00, 1.00, 1.00> * 0.75\n    fog_offset 0.10\n    fog_alt 1.50\n    turbulence 1.75\n}\n//gnd\nplane { <0.00, 1.00, 0.00>, 0\n    texture {\n        pigment{ color rgb <0.25, 0.45, 0.00> }\n        normal { bumps 0.75 scale 0.01 }\n        finish { phong 0.10 }\n    }\n}\n\0"
            as *const u8 as *const libc::c_char,
    );
    gvputs(
        job,
        b"light_source { <1500,3000,-2500> color White }\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn pov_end_graph(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** end_graph\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pov_begin_layer(
    mut job: *mut GVJ_t,
    mut layername: *mut libc::c_char,
    mut layerNum: libc::c_int,
    mut numLayers: libc::c_int,
) {
    gvprintf(
        job,
        b"//*** begin_layer: %s, %d/%d\n\0" as *const u8 as *const libc::c_char,
        layername,
        layerNum,
        numLayers,
    );
    layerz = (layerNum * -(10 as libc::c_int)) as libc::c_float;
}
unsafe extern "C" fn pov_end_layer(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** end_layer\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pov_begin_page(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** begin_page\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pov_end_page(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** end_page\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pov_begin_cluster(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** begin_cluster\n\0" as *const u8 as *const libc::c_char);
    layerz -= 2 as libc::c_int as libc::c_float;
}
unsafe extern "C" fn pov_end_cluster(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** end_cluster\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pov_begin_node(mut job: *mut GVJ_t) {
    gvprintf(
        job,
        b"//*** begin_node: %s\n\0" as *const u8 as *const libc::c_char,
        agnameof((*(*job).obj).u.n as *mut libc::c_void),
    );
}
unsafe extern "C" fn pov_end_node(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** end_node\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pov_begin_edge(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** begin_edge\n\0" as *const u8 as *const libc::c_char);
    layerz -= 5 as libc::c_int as libc::c_float;
}
unsafe extern "C" fn pov_end_edge(mut job: *mut GVJ_t) {
    gvputs(job, b"//*** end_edge\n\0" as *const u8 as *const libc::c_char);
    layerz += 5 as libc::c_int as libc::c_float;
}
unsafe extern "C" fn pov_textspan(
    mut job: *mut GVJ_t,
    mut c: pointf,
    mut span: *mut textspan_t,
) {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut pov: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    gvprintf(
        job,
        b"//*** textspan: %s, fontsize = %.3f, fontname = %s\n\0" as *const u8
            as *const libc::c_char,
        (*span).str_0,
        (*(*span).font).size,
        (*(*span).font).name,
    );
    z = layerz - 9 as libc::c_int as libc::c_float;
    match (*span).just as libc::c_int {
        108 => {}
        114 => {
            c.x = c.x - (*span).size.x;
        }
        110 | _ => {
            c.x = c.x - (*span).size.x / 2.0f64;
        }
    }
    x = (c.x + (*job).translation.x) * (*job).scale.x;
    y = (c.y + (*job).translation.y) * (*job).scale.y;
    s = el(
        job,
        b"scale %.3f\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*(*span).font).size * (*job).scale.x,
    );
    r = el(
        job,
        b"rotate   <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0.0f64,
        0.0f64,
        (*job).rotation as libc::c_float as libc::c_double,
    );
    t = el(
        job,
        b"translate<%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        x,
        y,
        z as libc::c_double,
    );
    p = pov_color_as_str(job, (*(*job).obj).pencolor, 0.0f64 as libc::c_float);
    pov = el(
        job,
        b"text {\n    ttf \"%s\",\n    \"%s\", %.3f, %.3f\n    %s    %s    %s    %s    %s}\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*(*span).font).name,
        0.25f64,
        0.0f64,
        (*span).str_0,
        b"    no_shadow\n\0" as *const u8 as *const libc::c_char,
        s,
        r,
        t,
        p,
    );
    gvputs(job, pov);
    free(pov as *mut libc::c_void);
    free(r as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(t as *mut libc::c_void);
    free(s as *mut libc::c_void);
}
unsafe extern "C" fn pov_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut pov: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cx: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    let mut rx: libc::c_float = 0.;
    let mut ry: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    gvputs(job, b"//*** ellipse\n\0" as *const u8 as *const libc::c_char);
    z = layerz - 6 as libc::c_int as libc::c_float;
    cx = (((*A.offset(0 as libc::c_int as isize)).x + (*job).translation.x)
        * (*job).scale.x) as libc::c_float;
    cy = (((*A.offset(0 as libc::c_int as isize)).y + (*job).translation.y)
        * (*job).scale.y) as libc::c_float;
    rx = (((*A.offset(1 as libc::c_int as isize)).x
        - (*A.offset(0 as libc::c_int as isize)).x) * (*job).scale.x) as libc::c_float;
    ry = (((*A.offset(1 as libc::c_int as isize)).y
        - (*A.offset(0 as libc::c_int as isize)).y) * (*job).scale.y) as libc::c_float;
    w = ((*(*job).obj).penwidth / (rx + ry) as libc::c_double / 2.0f64
        * 5 as libc::c_int as libc::c_double) as libc::c_float;
    s = el(
        job,
        b"scale    <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        rx as libc::c_double,
        (rx + ry) as libc::c_double / 4.0f64,
        ry as libc::c_double,
    );
    r = el(
        job,
        b"rotate   <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        90.0f64,
        0.0f64,
        (*job).rotation as libc::c_float as libc::c_double,
    );
    t = el(
        job,
        b"translate<%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        cx as libc::c_double,
        cy as libc::c_double,
        z as libc::c_double,
    );
    p = pov_color_as_str(job, (*(*job).obj).pencolor, 0.0f64 as libc::c_float);
    pov = el(
        job,
        b"torus { %.3f, %.3f\n    %s    %s    %s    %s}\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        1.0f64,
        w as libc::c_double,
        s,
        r,
        t,
        p,
    );
    gvputs(job, pov);
    free(s as *mut libc::c_void);
    free(r as *mut libc::c_void);
    free(t as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(pov as *mut libc::c_void);
    if filled != 0 {
        s = el(
            job,
            b"scale    <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            rx as libc::c_double,
            ry as libc::c_double,
            1.0f64,
        );
        r = el(
            job,
            b"rotate   <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0.0f64,
            0.0f64,
            (*job).rotation as libc::c_float as libc::c_double,
        );
        t = el(
            job,
            b"translate<%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cx as libc::c_double,
            cy as libc::c_double,
            z as libc::c_double,
        );
        p = pov_color_as_str(job, (*(*job).obj).fillcolor, 0.0f64 as libc::c_float);
        pov = el(
            job,
            b"sphere {<%9.3f, %9.3f, %9.3f>, 1.0\n    %s    %s    %s    %s}\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            0.0f64,
            0.0f64,
            0.0f64,
            s,
            r,
            t,
            p,
        );
        gvputs(job, pov);
        free(s as *mut libc::c_void);
        free(r as *mut libc::c_void);
        free(t as *mut libc::c_void);
        free(p as *mut libc::c_void);
        free(pov as *mut libc::c_void);
    }
}
unsafe extern "C" fn pov_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pov: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    gvputs(job, b"//*** bezier\n\0" as *const u8 as *const libc::c_char);
    z = layerz - 4 as libc::c_int as libc::c_float;
    s = el(
        job,
        b"scale    <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*job).scale.x,
        (*job).scale.y,
        1.0f64,
    );
    r = el(
        job,
        b"rotate   <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0.0f64,
        0.0f64,
        (*job).rotation as libc::c_float as libc::c_double,
    );
    t = el(
        job,
        b"translate<%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0.0f64,
        0.0f64,
        (z - 2 as libc::c_int as libc::c_float) as libc::c_double,
    );
    p = pov_color_as_str(job, (*(*job).obj).fillcolor, 0.0f64 as libc::c_float);
    pov = el(
        job,
        b"sphere_sweep {\n    %s\n    %d,\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"b_spline\0" as *const u8 as *const libc::c_char,
        n + 2 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < n {
        v = el(
            job,
            b"<%9.3f, %9.3f, %9.3f>, %.3f\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*A.offset(i as isize)).x + (*job).translation.x,
            (*A.offset(i as isize)).y + (*job).translation.y,
            0.0f64,
            (*(*job).obj).penwidth,
        );
        x = el(
            job,
            b"%s    %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pov,
            v,
        );
        free(v as *mut libc::c_void);
        free(pov as *mut libc::c_void);
        pov = x;
        if i == 0 as libc::c_int || i == n - 1 as libc::c_int {
            v = el(
                job,
                b"<%9.3f, %9.3f, %9.3f>, %.3f\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*A.offset(i as isize)).x + (*job).translation.x,
                (*A.offset(i as isize)).y + (*job).translation.y,
                0.0f64,
                (*(*job).obj).penwidth,
            );
            x = el(
                job,
                b"%s    %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pov,
                v,
            );
            free(v as *mut libc::c_void);
            free(pov as *mut libc::c_void);
            pov = x;
        }
        i += 1;
    }
    x = el(
        job,
        b"        tolerance 0.01\n    %s    %s    %s    %s}\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        s,
        r,
        t,
        p,
    );
    pov = el(
        job,
        b"%s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pov,
        x,
    );
    free(x as *mut libc::c_void);
    gvputs(job, pov);
    free(s as *mut libc::c_void);
    free(r as *mut libc::c_void);
    free(t as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(pov as *mut libc::c_void);
}
unsafe extern "C" fn pov_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut pov: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    gvputs(job, b"//*** polygon\n\0" as *const u8 as *const libc::c_char);
    z = layerz - 2 as libc::c_int as libc::c_float;
    s = el(
        job,
        b"scale    <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*job).scale.x,
        (*job).scale.y,
        1.0f64,
    );
    r = el(
        job,
        b"rotate   <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0.0f64,
        0.0f64,
        (*job).rotation as libc::c_float as libc::c_double,
    );
    t = el(
        job,
        b"translate<%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0.0f64,
        0.0f64,
        (z - 2 as libc::c_int as libc::c_float) as libc::c_double,
    );
    p = pov_color_as_str(job, (*(*job).obj).pencolor, 0.0f64 as libc::c_float);
    pov = el(
        job,
        b"sphere_sweep {\n    %s\n    %d,\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"linear_spline\0" as *const u8 as *const libc::c_char,
        n + 1 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < n {
        v = el(
            job,
            b"<%9.3f, %9.3f, %9.3f>, %.3f\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*A.offset(i as isize)).x + (*job).translation.x,
            (*A.offset(i as isize)).y + (*job).translation.y,
            0.0f64,
            (*(*job).obj).penwidth,
        );
        x = el(
            job,
            b"%s    %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pov,
            v,
        );
        free(v as *mut libc::c_void);
        free(pov as *mut libc::c_void);
        pov = x;
        i += 1;
    }
    v = el(
        job,
        b"<%9.3f, %9.3f, %9.3f>, %.3f\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*A.offset(0 as libc::c_int as isize)).x + (*job).translation.x,
        (*A.offset(0 as libc::c_int as isize)).y + (*job).translation.y,
        0.0f64,
        (*(*job).obj).penwidth,
    );
    x = el(
        job,
        b"%s    %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pov,
        v,
    );
    free(v as *mut libc::c_void);
    free(pov as *mut libc::c_void);
    pov = x;
    x = el(
        job,
        b"    tolerance 0.1\n    %s    %s    %s    %s}\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        s,
        r,
        t,
        p,
    );
    pov = el(
        job,
        b"%s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pov,
        x,
    );
    free(x as *mut libc::c_void);
    gvputs(job, pov);
    free(s as *mut libc::c_void);
    free(r as *mut libc::c_void);
    free(t as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(pov as *mut libc::c_void);
    if filled != 0 {
        s = el(
            job,
            b"scale    <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*job).scale.x,
            (*job).scale.y,
            1.0f64,
        );
        r = el(
            job,
            b"rotate   <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0.0f64,
            0.0f64,
            (*job).rotation as libc::c_float as libc::c_double,
        );
        t = el(
            job,
            b"translate<%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0.0f64,
            0.0f64,
            (z - 2 as libc::c_int as libc::c_float) as libc::c_double,
        );
        p = pov_color_as_str(job, (*(*job).obj).fillcolor, 0.25f64 as libc::c_float);
        pov = el(
            job,
            b"polygon { %d,\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            n,
        );
        i = 0 as libc::c_int;
        while i < n {
            v = el(
                job,
                b"<%9.3f, %9.3f, %9.3f>\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*A.offset(i as isize)).x + (*job).translation.x,
                (*A.offset(i as isize)).y + (*job).translation.y,
                0.0f64,
            );
            x = el(
                job,
                b"%s\n    %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pov,
                v,
            );
            free(v as *mut libc::c_void);
            free(pov as *mut libc::c_void);
            pov = x;
            i += 1;
        }
        x = el(
            job,
            b"\n    %s    %s    %s    %s}\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
            r,
            t,
            p,
        );
        pov = el(
            job,
            b"%s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pov,
            x,
        );
        free(x as *mut libc::c_void);
        gvputs(job, pov);
        free(s as *mut libc::c_void);
        free(r as *mut libc::c_void);
        free(t as *mut libc::c_void);
        free(p as *mut libc::c_void);
        free(pov as *mut libc::c_void);
    }
}
unsafe extern "C" fn pov_polyline(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    let mut pov: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    gvputs(job, b"//*** polyline\n\0" as *const u8 as *const libc::c_char);
    z = layerz - 6 as libc::c_int as libc::c_float;
    s = el(
        job,
        b"scale    <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*job).scale.x,
        (*job).scale.y,
        1.0f64,
    );
    r = el(
        job,
        b"rotate   <%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0.0f64,
        0.0f64,
        (*job).rotation as libc::c_float as libc::c_double,
    );
    t = el(
        job,
        b"translate<%9.3f, %9.3f, %9.3f>\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0.0f64,
        0.0f64,
        z as libc::c_double,
    );
    p = pov_color_as_str(job, (*(*job).obj).pencolor, 0.0f64 as libc::c_float);
    pov = el(
        job,
        b"sphere_sweep {\n    %s\n    %d,\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"linear_spline\0" as *const u8 as *const libc::c_char,
        n,
    );
    i = 0 as libc::c_int;
    while i < n {
        v = el(
            job,
            b"<%9.3f, %9.3f, %9.3f>, %.3f\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*A.offset(i as isize)).x + (*job).translation.x,
            (*A.offset(i as isize)).y + (*job).translation.y,
            0.0f64,
            (*(*job).obj).penwidth,
        );
        x = el(
            job,
            b"%s    %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pov,
            v,
        );
        free(v as *mut libc::c_void);
        free(pov as *mut libc::c_void);
        pov = x;
        i += 1;
    }
    x = el(
        job,
        b"    tolerance 0.01\n    %s    %s    %s    %s}\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        s,
        r,
        t,
        p,
    );
    pov = el(
        job,
        b"%s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pov,
        x,
    );
    free(x as *mut libc::c_void);
    gvputs(job, pov);
    free(s as *mut libc::c_void);
    free(r as *mut libc::c_void);
    free(t as *mut libc::c_void);
    free(p as *mut libc::c_void);
    free(pov as *mut libc::c_void);
}
#[no_mangle]
pub static mut pov_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: Some(pov_begin_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_job: None,
            begin_graph: Some(pov_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_graph: Some(pov_end_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_layer: Some(
                pov_begin_layer
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut libc::c_char,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            end_layer: Some(pov_end_layer as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_page: Some(pov_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_page: Some(pov_end_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_cluster: Some(
                pov_begin_cluster as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            end_cluster: Some(pov_end_cluster as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: Some(pov_begin_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_node: Some(pov_end_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_edge: Some(pov_begin_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_edge: Some(pov_end_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_anchor: None,
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                pov_textspan
                    as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: None,
            ellipse: Some(
                pov_ellipse
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                pov_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                pov_bezier
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            polyline: Some(
                pov_polyline
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: Some(
                pov_comment as unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> (),
            ),
            library_shape: None,
        };
        init
    }
};
#[no_mangle]
pub static mut render_features_pov: gvrender_features_t = gvrender_features_t {
    flags: 0,
    default_pad: 0.,
    knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    sz_knowncolors: 0,
    color_type: HSVA_DOUBLE,
};
#[no_mangle]
pub static mut device_features_pov: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 8 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_pov_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_POV as libc::c_int,
                type_0: b"pov\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &pov_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_pov as *const gvrender_features_t
                    as *mut gvrender_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: 0 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut gvdevice_pov_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_POV as libc::c_int,
                type_0: b"pov:pov\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_pov as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: 0 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
unsafe extern "C" fn run_static_initializers() {
    render_features_pov = {
        let mut init = gvrender_features_t {
            flags: (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 17 as libc::c_int
                | (1 as libc::c_int) << 18 as libc::c_int
                | (1 as libc::c_int) << 19 as libc::c_int
                | (1 as libc::c_int) << 20 as libc::c_int
                | (1 as libc::c_int) << 21 as libc::c_int
                | (1 as libc::c_int) << 25 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 24 as libc::c_int
                | (1 as libc::c_int) << 21 as libc::c_int,
            default_pad: 4.0f64,
            knowncolors: pov_knowncolors.as_mut_ptr(),
            sz_knowncolors: (::std::mem::size_of::<[*mut libc::c_char; 122]>()
                as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) as libc::c_int,
            color_type: RGBA_BYTE,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
