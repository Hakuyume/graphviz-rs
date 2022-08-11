#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type GVC_s;
    fn round(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvputs_nonascii(job: *mut GVJ_t, s: *const libc::c_char);
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn Bezier(
        _: *mut pointf,
        _: libc::c_int,
        _: libc::c_double,
        _: *mut pointf,
        _: *mut pointf,
    ) -> pointf;
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FORMAT_FIG: C2RustUnnamed_4 = 0;
static mut Depth: libc::c_int = 0;
unsafe extern "C" fn figptarray(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut close: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut p: point = point { x: 0, y: 0 };
    i = 0 as libc::c_int;
    while i < n {
        p
            .x = (if (*A.offset(i as isize)).x >= 0 as libc::c_int as libc::c_double {
            ((*A.offset(i as isize)).x + 0.5f64) as libc::c_int
        } else {
            ((*A.offset(i as isize)).x - 0.5f64) as libc::c_int
        });
        p
            .y = (if (*A.offset(i as isize)).y >= 0 as libc::c_int as libc::c_double {
            ((*A.offset(i as isize)).y + 0.5f64) as libc::c_int
        } else {
            ((*A.offset(i as isize)).y - 0.5f64) as libc::c_int
        });
        gvprintf(job, b" %d %d\0" as *const u8 as *const libc::c_char, p.x, p.y);
        i += 1;
    }
    if close != 0 {
        p
            .x = (if (*A.offset(0 as libc::c_int as isize)).x
            >= 0 as libc::c_int as libc::c_double
        {
            ((*A.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
        } else {
            ((*A.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
        });
        p
            .y = (if (*A.offset(0 as libc::c_int as isize)).y
            >= 0 as libc::c_int as libc::c_double
        {
            ((*A.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
        } else {
            ((*A.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
        });
        gvprintf(job, b" %d %d\0" as *const u8 as *const libc::c_char, p.x, p.y);
    }
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn figColorResolve(
    mut new: *mut libc::c_int,
    mut r: libc::c_uchar,
    mut g: libc::c_uchar,
    mut b: libc::c_uchar,
) -> libc::c_int {
    static mut top: libc::c_int = 0 as libc::c_int;
    static mut red: [libc::c_short; 256] = [0; 256];
    static mut green: [libc::c_short; 256] = [0; 256];
    static mut blue: [libc::c_short; 256] = [0; 256];
    let mut c: libc::c_int = 0;
    let mut ct: libc::c_int = -(1 as libc::c_int);
    let mut rd: libc::c_long = 0;
    let mut gd: libc::c_long = 0;
    let mut bd: libc::c_long = 0;
    let mut dist: libc::c_long = 0;
    let mut mindist: libc::c_long = (3 as libc::c_int * 255 as libc::c_int
        * 255 as libc::c_int) as libc::c_long;
    *new = 0 as libc::c_int;
    c = 0 as libc::c_int;
    while c < top {
        rd = (red[c as usize] as libc::c_int - r as libc::c_int) as libc::c_long;
        gd = (green[c as usize] as libc::c_int - g as libc::c_int) as libc::c_long;
        bd = (blue[c as usize] as libc::c_int - b as libc::c_int) as libc::c_long;
        dist = rd * rd + gd * gd + bd * bd;
        if dist < mindist {
            if dist == 0 as libc::c_int as libc::c_long {
                return c;
            }
            mindist = dist;
            ct = c;
        }
        c += 1;
    }
    let fresh0 = top;
    top = top + 1;
    if fresh0 == 256 as libc::c_int {
        return ct;
    }
    red[c as usize] = r as libc::c_short;
    green[c as usize] = g as libc::c_short;
    blue[c as usize] = b as libc::c_short;
    *new = 1 as libc::c_int;
    return c;
}
static mut figcolor: [*mut libc::c_char; 9] = [
    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"green\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"magenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
];
unsafe extern "C" fn fig_resolve_color(mut job: *mut GVJ_t, mut color: *mut gvcolor_t) {
    let mut object_code: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    match (*color).type_0 as libc::c_uint {
        5 => {
            i = 0 as libc::c_int;
            while !(figcolor[i as usize]).is_null() {
                if strcmp(figcolor[i as usize], (*color).u.string) == 0 {
                    (*color).u.index = i;
                    break;
                } else {
                    i += 1;
                }
            }
        }
        1 => {
            i = 32 as libc::c_int
                + figColorResolve(
                    &mut new,
                    (*color).u.rgba[0 as libc::c_int as usize],
                    (*color).u.rgba[1 as libc::c_int as usize],
                    (*color).u.rgba[2 as libc::c_int as usize],
                );
            if new != 0 {
                gvprintf(
                    job,
                    b"%d %d #%02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
                    object_code,
                    i,
                    (*color).u.rgba[0 as libc::c_int as usize] as libc::c_int,
                    (*color).u.rgba[1 as libc::c_int as usize] as libc::c_int,
                    (*color).u.rgba[2 as libc::c_int as usize] as libc::c_int,
                );
            }
            (*color).u.index = i;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"gvrender_core_fig.c\0" as *const u8 as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void fig_resolve_color(GVJ_t *, gvcolor_t *)\0"))
                    .as_ptr(),
            );
        }
    }
    (*color).type_0 = COLOR_INDEX;
}
unsafe extern "C" fn fig_line_style(
    mut obj: *mut obj_state_t,
    mut line_style: *mut libc::c_int,
    mut style_val: *mut libc::c_double,
) {
    match (*obj).pen as libc::c_uint {
        1 => {
            *line_style = 1 as libc::c_int;
            *style_val = 10.0f64;
        }
        2 => {
            *line_style = 2 as libc::c_int;
            *style_val = 10.0f64;
        }
        3 | _ => {
            *line_style = 0 as libc::c_int;
            *style_val = 0.0f64;
        }
    };
}
unsafe extern "C" fn fig_comment(mut job: *mut GVJ_t, mut str: *mut libc::c_char) {
    gvprintf(job, b"# %s\n\0" as *const u8 as *const libc::c_char, str);
}
unsafe extern "C" fn fig_begin_graph(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    gvputs(job, b"#FIG 3.2\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"# Generated by %s version %s (%s)\n\0" as *const u8 as *const libc::c_char,
        *((*(*job).common).info).offset(0 as libc::c_int as isize),
        *((*(*job).common).info).offset(1 as libc::c_int as isize),
        *((*(*job).common).info).offset(2 as libc::c_int as isize),
    );
    gvprintf(
        job,
        b"# Title: %s\n\0" as *const u8 as *const libc::c_char,
        agnameof((*obj).u.g as *mut libc::c_void),
    );
    gvprintf(
        job,
        b"# Pages: %d\n\0" as *const u8 as *const libc::c_char,
        (*job).pagesArraySize.x * (*job).pagesArraySize.y,
    );
    gvputs(job, b"Portrait\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"Center\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"Inches\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"Letter\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"100.00\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"Single\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"-2\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"1200\0" as *const u8 as *const libc::c_char);
    gvputs(job, b" 2\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fig_end_graph(mut job: *mut GVJ_t) {
    gvputs(job, b"# end of FIG file\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fig_begin_page(mut job: *mut GVJ_t) {
    Depth = 2 as libc::c_int;
}
unsafe extern "C" fn fig_begin_node(mut job: *mut GVJ_t) {
    Depth = 1 as libc::c_int;
}
unsafe extern "C" fn fig_end_node(mut job: *mut GVJ_t) {
    Depth = 2 as libc::c_int;
}
unsafe extern "C" fn fig_begin_edge(mut job: *mut GVJ_t) {
    Depth = 0 as libc::c_int;
}
unsafe extern "C" fn fig_end_edge(mut job: *mut GVJ_t) {
    Depth = 2 as libc::c_int;
}
unsafe extern "C" fn fig_textspan(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut span: *mut textspan_t,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut pA: *mut PostscriptAlias = 0 as *mut PostscriptAlias;
    let mut object_code: libc::c_int = 4 as libc::c_int;
    let mut sub_type: libc::c_int = 0 as libc::c_int;
    let mut color: libc::c_int = (*obj).pencolor.u.index;
    let mut depth: libc::c_int = Depth;
    let mut pen_style: libc::c_int = 0 as libc::c_int;
    let mut font: libc::c_int = -(1 as libc::c_int);
    let mut font_size: libc::c_double = (*(*span).font).size * (*job).zoom;
    let mut angle: libc::c_double = if (*job).rotation != 0 {
        3.14159265358979323846f64 / 2.0f64
    } else {
        0.0f64
    };
    let mut font_flags: libc::c_int = 6 as libc::c_int;
    let mut height: libc::c_double = font_size;
    let mut length: libc::c_double = 2.0f64 * font_size / 3.0f64
        * strlen((*span).str_0) as libc::c_double / 2.0f64;
    pA = (*(*span).font).postscript_alias;
    if !pA.is_null() {
        font = (*pA).xfig_code;
    }
    match (*span).just as libc::c_int {
        108 => {
            sub_type = 0 as libc::c_int;
        }
        114 => {
            sub_type = 2 as libc::c_int;
        }
        110 | _ => {
            sub_type = 1 as libc::c_int;
        }
    }
    gvprintf(
        job,
        b"%d %d %d %d %d %d %.1f %.4f %d %.1f %.1f %d %d \0" as *const u8
            as *const libc::c_char,
        object_code,
        sub_type,
        color,
        depth,
        pen_style,
        font,
        font_size,
        angle,
        font_flags,
        height,
        length,
        if p.x >= 0 as libc::c_int as libc::c_double {
            (p.x + 0.5f64) as libc::c_int
        } else {
            (p.x - 0.5f64) as libc::c_int
        },
        if p.y - 72.0f64 >= 0 as libc::c_int as libc::c_double {
            (p.y - 72.0f64 + 0.5f64) as libc::c_int
        } else {
            (p.y - 72.0f64 - 0.5f64) as libc::c_int
        },
    );
    gvputs_nonascii(job, (*span).str_0);
    gvputs(job, b"\\001\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fig_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut object_code: libc::c_int = 1 as libc::c_int;
    let mut sub_type: libc::c_int = 1 as libc::c_int;
    let mut line_style: libc::c_int = 0;
    let mut thickness: libc::c_double = round((*obj).penwidth);
    let mut pen_color: libc::c_int = (*obj).pencolor.u.index;
    let mut fill_color: libc::c_int = (*obj).fillcolor.u.index;
    let mut depth: libc::c_int = Depth;
    let mut pen_style: libc::c_int = 0 as libc::c_int;
    let mut area_fill: libc::c_int = if filled != 0 {
        20 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut style_val: libc::c_double = 0.;
    let mut direction: libc::c_int = 0 as libc::c_int;
    let mut angle: libc::c_double = 0.0f64;
    let mut center_x: libc::c_int = 0;
    let mut center_y: libc::c_int = 0;
    let mut radius_x: libc::c_int = 0;
    let mut radius_y: libc::c_int = 0;
    let mut start_x: libc::c_int = 0;
    let mut start_y: libc::c_int = 0;
    let mut end_x: libc::c_int = 0;
    let mut end_y: libc::c_int = 0;
    fig_line_style(obj, &mut line_style, &mut style_val);
    center_x = if (*A.offset(0 as libc::c_int as isize)).x
        >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
    };
    start_x = center_x;
    center_y = if (*A.offset(0 as libc::c_int as isize)).y
        >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
    };
    start_y = center_y;
    radius_x = if (*A.offset(1 as libc::c_int as isize)).x
        - (*A.offset(0 as libc::c_int as isize)).x >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(1 as libc::c_int as isize)).x
            - (*A.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(1 as libc::c_int as isize)).x
            - (*A.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
    };
    radius_y = if (*A.offset(1 as libc::c_int as isize)).y
        - (*A.offset(0 as libc::c_int as isize)).y >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(1 as libc::c_int as isize)).y
            - (*A.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(1 as libc::c_int as isize)).y
            - (*A.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
    };
    end_x = if (*A.offset(1 as libc::c_int as isize)).x
        >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(1 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(1 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
    };
    end_y = if (*A.offset(1 as libc::c_int as isize)).y
        >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(1 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(1 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
    };
    gvprintf(
        job,
        b"%d %d %d %.0f %d %d %d %d %d %.3f %d %.4f %d %d %d %d %d %d %d %d\n\0"
            as *const u8 as *const libc::c_char,
        object_code,
        sub_type,
        line_style,
        thickness,
        pen_color,
        fill_color,
        depth,
        pen_style,
        area_fill,
        style_val,
        direction,
        angle,
        center_x,
        center_y,
        radius_x,
        radius_y,
        start_x,
        start_y,
        end_x,
        end_y,
    );
}
unsafe extern "C" fn fig_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut object_code: libc::c_int = 3 as libc::c_int;
    let mut sub_type: libc::c_int = 0;
    let mut line_style: libc::c_int = 0;
    let mut thickness: libc::c_double = round((*obj).penwidth);
    let mut pen_color: libc::c_int = (*obj).pencolor.u.index;
    let mut fill_color: libc::c_int = (*obj).fillcolor.u.index;
    let mut depth: libc::c_int = Depth;
    let mut pen_style: libc::c_int = 0 as libc::c_int;
    let mut area_fill: libc::c_int = 0;
    let mut style_val: libc::c_double = 0.;
    let mut cap_style: libc::c_int = 0 as libc::c_int;
    let mut forward_arrow: libc::c_int = 0 as libc::c_int;
    let mut backward_arrow: libc::c_int = 0 as libc::c_int;
    let mut npoints: libc::c_int = n;
    let mut i: libc::c_int = 0;
    let mut pf: pointf = pointf { x: 0., y: 0. };
    let mut V: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut p: point = point { x: 0, y: 0 };
    let mut j: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if n >= 4 as libc::c_int {} else {
        __assert_fail(
            b"n >= 4\0" as *const u8 as *const libc::c_char,
            b"gvrender_core_fig.c\0" as *const u8 as *const libc::c_char,
            333 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void fig_bezier(GVJ_t *, pointf *, int, int, int, int)\0"))
                .as_ptr(),
        );
    }
    buffer = malloc(
        (((npoints + 1 as libc::c_int) * (6 as libc::c_int + 1 as libc::c_int)
            * 20 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    buf = buffer;
    fig_line_style(obj, &mut line_style, &mut style_val);
    if filled != 0 {
        sub_type = 5 as libc::c_int;
        area_fill = 20 as libc::c_int;
        fill_color = (*(*job).obj).fillcolor.u.index;
    } else {
        sub_type = 4 as libc::c_int;
        area_fill = -(1 as libc::c_int);
        fill_color = 0 as libc::c_int;
    }
    V[3 as libc::c_int as usize].x = (*A.offset(0 as libc::c_int as isize)).x;
    V[3 as libc::c_int as usize].y = (*A.offset(0 as libc::c_int as isize)).y;
    count += 1;
    p
        .x = (if (*A.offset(0 as libc::c_int as isize)).x
        >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
    });
    p
        .y = (if (*A.offset(0 as libc::c_int as isize)).y
        >= 0 as libc::c_int as libc::c_double
    {
        ((*A.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
    } else {
        ((*A.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
    });
    size = sprintf(buf, b" %d %d\0" as *const u8 as *const libc::c_char, p.x, p.y);
    buf = buf.offset(size as isize);
    i = 0 as libc::c_int;
    while (i + 3 as libc::c_int) < n {
        V[0 as libc::c_int as usize] = V[3 as libc::c_int as usize];
        j = 1 as libc::c_int;
        while j <= 3 as libc::c_int {
            V[j as usize].x = (*A.offset((i + j) as isize)).x;
            V[j as usize].y = (*A.offset((i + j) as isize)).y;
            j += 1;
        }
        step = 1 as libc::c_int;
        while step <= 6 as libc::c_int {
            count += 1;
            pf = Bezier(
                V.as_mut_ptr(),
                3 as libc::c_int,
                step as libc::c_double / 6 as libc::c_int as libc::c_double,
                0 as *mut pointf,
                0 as *mut pointf,
            );
            p
                .x = (if pf.x >= 0 as libc::c_int as libc::c_double {
                (pf.x + 0.5f64) as libc::c_int
            } else {
                (pf.x - 0.5f64) as libc::c_int
            });
            p
                .y = (if pf.y >= 0 as libc::c_int as libc::c_double {
                (pf.y + 0.5f64) as libc::c_int
            } else {
                (pf.y - 0.5f64) as libc::c_int
            });
            size = sprintf(
                buf,
                b" %d %d\0" as *const u8 as *const libc::c_char,
                p.x,
                p.y,
            );
            buf = buf.offset(size as isize);
            step += 1;
        }
        i += 3 as libc::c_int;
    }
    gvprintf(
        job,
        b"%d %d %d %.0f %d %d %d %d %d %.1f %d %d %d %d\n\0" as *const u8
            as *const libc::c_char,
        object_code,
        sub_type,
        line_style,
        thickness,
        pen_color,
        fill_color,
        depth,
        pen_style,
        area_fill,
        style_val,
        cap_style,
        forward_arrow,
        backward_arrow,
        count,
    );
    gvprintf(job, b" %s\n\0" as *const u8 as *const libc::c_char, buffer);
    free(buffer as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < count {
        gvprintf(
            job,
            b" %d\0" as *const u8 as *const libc::c_char,
            if i % (count - 1 as libc::c_int) != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
        i += 1;
    }
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fig_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut object_code: libc::c_int = 2 as libc::c_int;
    let mut sub_type: libc::c_int = 3 as libc::c_int;
    let mut line_style: libc::c_int = 0;
    let mut thickness: libc::c_double = round((*obj).penwidth);
    let mut pen_color: libc::c_int = (*obj).pencolor.u.index;
    let mut fill_color: libc::c_int = (*obj).fillcolor.u.index;
    let mut depth: libc::c_int = Depth;
    let mut pen_style: libc::c_int = 0 as libc::c_int;
    let mut area_fill: libc::c_int = if filled != 0 {
        20 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut style_val: libc::c_double = 0.;
    let mut join_style: libc::c_int = 0 as libc::c_int;
    let mut cap_style: libc::c_int = 0 as libc::c_int;
    let mut radius: libc::c_int = 0 as libc::c_int;
    let mut forward_arrow: libc::c_int = 0 as libc::c_int;
    let mut backward_arrow: libc::c_int = 0 as libc::c_int;
    let mut npoints: libc::c_int = n + 1 as libc::c_int;
    fig_line_style(obj, &mut line_style, &mut style_val);
    gvprintf(
        job,
        b"%d %d %d %.0f %d %d %d %d %d %.1f %d %d %d %d %d %d\n\0" as *const u8
            as *const libc::c_char,
        object_code,
        sub_type,
        line_style,
        thickness,
        pen_color,
        fill_color,
        depth,
        pen_style,
        area_fill,
        style_val,
        join_style,
        cap_style,
        radius,
        forward_arrow,
        backward_arrow,
        npoints,
    );
    figptarray(job, A, n, 1 as libc::c_int);
}
unsafe extern "C" fn fig_polyline(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut object_code: libc::c_int = 2 as libc::c_int;
    let mut sub_type: libc::c_int = 1 as libc::c_int;
    let mut line_style: libc::c_int = 0;
    let mut thickness: libc::c_double = round((*obj).penwidth);
    let mut pen_color: libc::c_int = (*obj).pencolor.u.index;
    let mut fill_color: libc::c_int = 0 as libc::c_int;
    let mut depth: libc::c_int = Depth;
    let mut pen_style: libc::c_int = 0 as libc::c_int;
    let mut area_fill: libc::c_int = 0 as libc::c_int;
    let mut style_val: libc::c_double = 0.;
    let mut join_style: libc::c_int = 0 as libc::c_int;
    let mut cap_style: libc::c_int = 0 as libc::c_int;
    let mut radius: libc::c_int = 0 as libc::c_int;
    let mut forward_arrow: libc::c_int = 0 as libc::c_int;
    let mut backward_arrow: libc::c_int = 0 as libc::c_int;
    let mut npoints: libc::c_int = n;
    fig_line_style(obj, &mut line_style, &mut style_val);
    gvprintf(
        job,
        b"%d %d %d %.0f %d %d %d %d %d %.1f %d %d %d %d %d %d\n\0" as *const u8
            as *const libc::c_char,
        object_code,
        sub_type,
        line_style,
        thickness,
        pen_color,
        fill_color,
        depth,
        pen_style,
        area_fill,
        style_val,
        join_style,
        cap_style,
        radius,
        forward_arrow,
        backward_arrow,
        npoints,
    );
    figptarray(job, A, n, 0 as libc::c_int);
}
#[no_mangle]
pub static mut fig_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: None,
            end_job: None,
            begin_graph: Some(fig_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_graph: Some(fig_end_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_layer: None,
            end_layer: None,
            begin_page: Some(fig_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_page: None,
            begin_cluster: None,
            end_cluster: None,
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: Some(fig_begin_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_node: Some(fig_end_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_edge: Some(fig_begin_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_edge: Some(fig_end_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_anchor: None,
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                fig_textspan
                    as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: Some(
                fig_resolve_color
                    as unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> (),
            ),
            ellipse: Some(
                fig_ellipse
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                fig_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                fig_bezier
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
                fig_polyline
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: Some(
                fig_comment as unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> (),
            ),
            library_shape: None,
        };
        init
    }
};
static mut fig_knowncolors: [*mut libc::c_char; 8] = [
    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"green\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"magenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut render_features_fig: gvrender_features_t = gvrender_features_t {
    flags: 0,
    default_pad: 0.,
    knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    sz_knowncolors: 0,
    color_type: HSVA_DOUBLE,
};
#[no_mangle]
pub static mut device_features_fig: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s {
                x: 1440.0f64,
                y: 1440.0f64,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_fig_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_FIG as libc::c_int,
                type_0: b"fig\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &fig_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_fig as *const gvrender_features_t
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
pub static mut gvdevice_fig_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_FIG as libc::c_int,
                type_0: b"fig:fig\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_fig as *const gvdevice_features_t
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
    render_features_fig = {
        let mut init = gvrender_features_t {
            flags: (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 12 as libc::c_int,
            default_pad: 4.0f64,
            knowncolors: fig_knowncolors.as_mut_ptr(),
            sz_knowncolors: (::std::mem::size_of::<[*mut libc::c_char; 8]>()
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
