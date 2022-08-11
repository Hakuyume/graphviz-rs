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
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn gvprintdouble(job: *mut GVJ_t, num: libc::c_double);
    fn gvprintpointf(job: *mut GVJ_t, p: pointf);
    fn gvprintpointflist(job: *mut GVJ_t, p: *mut pointf, n: libc::c_int);
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_engine_s {
    pub initialize: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub format: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub finalize: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
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
    pub begin_job: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_job: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_graph: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_graph: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_layer:
        Option<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char, libc::c_int, libc::c_int) -> ()>,
    pub end_layer: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_page: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_page: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_cluster: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_cluster: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_nodes: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_nodes: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edges: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edges: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_node: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_node: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edge: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edge: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_anchor: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub end_anchor: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_label: Option<unsafe extern "C" fn(*mut GVJ_t, label_type) -> ()>,
    pub end_label: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub textspan: Option<unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> ()>,
    pub resolve_color: Option<unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> ()>,
    pub ellipse: Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> ()>,
    pub polygon:
        Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int, libc::c_int) -> ()>,
    pub beziercurve: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut pointf,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub polyline: Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> ()>,
    pub comment: Option<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> ()>,
    pub library_shape: Option<
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
    pub free_layout: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FORMAT_TK: C2RustUnnamed_4 = 0;
unsafe extern "C" fn tkgen_string(mut s: *mut libc::c_char) -> *mut libc::c_char {
    return s;
}
unsafe extern "C" fn tkgen_print_color(mut job: *mut GVJ_t, mut color: gvcolor_t) {
    match color.type_0 as libc::c_uint {
        5 => {
            gvputs(job, color.u.string);
        }
        1 => {
            if color.u.rgba[3 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
                gvputs(job, b"\"\"\0" as *const u8 as *const libc::c_char);
            } else {
                gvprintf(
                    job,
                    b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
                    color.u.rgba[0 as libc::c_int as usize] as libc::c_int,
                    color.u.rgba[1 as libc::c_int as usize] as libc::c_int,
                    color.u.rgba[2 as libc::c_int as usize] as libc::c_int,
                );
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"gvrender_core_tk.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"void tkgen_print_color(GVJ_t *, gvcolor_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn tkgen_print_tags(mut job: *mut GVJ_t) {
    let mut ObjType: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ObjId: uint64_t = 0;
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut ObjFlag: libc::c_int = 0;
    match (*obj).emit_state as libc::c_uint {
        8 => {
            ObjType = b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjFlag = 1 as libc::c_int;
            ObjId = (*((*obj).u.n as *mut Agobj_t)).tag.id;
        }
        10 => {
            ObjType = b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjFlag = 0 as libc::c_int;
            ObjId = (*((*obj).u.n as *mut Agobj_t)).tag.id;
        }
        9 | 2 | 3 => {
            ObjType = b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjFlag = 1 as libc::c_int;
            ObjId = (*((*obj).u.e as *mut Agobj_t)).tag.id;
        }
        11 | 6 | 7 => {
            ObjType = b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjFlag = 0 as libc::c_int;
            ObjId = (*((*obj).u.e as *mut Agobj_t)).tag.id;
        }
        0 => {
            ObjType = b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjFlag = 1 as libc::c_int;
            ObjId = (*((*obj).u.g as *mut Agobj_t)).tag.id;
        }
        4 => {
            ObjFlag = 0 as libc::c_int;
            ObjType = b"graph label\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjId = (*((*obj).u.g as *mut Agobj_t)).tag.id;
        }
        1 => {
            ObjType = b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjFlag = 1 as libc::c_int;
            ObjId = (*((*obj).u.sg as *mut Agobj_t)).tag.id;
        }
        5 => {
            ObjType = b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            ObjFlag = 0 as libc::c_int;
            ObjId = (*((*obj).u.sg as *mut Agobj_t)).tag.id;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"gvrender_core_tk.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"void tkgen_print_tags(GVJ_t *)\0",
                ))
                .as_ptr(),
            );
        }
    }
    gvprintf(
        job,
        b" -tags {%d%s0x%lx}\0" as *const u8 as *const libc::c_char,
        ObjFlag,
        ObjType,
        ObjId,
    );
}
unsafe extern "C" fn tkgen_canvas(mut job: *mut GVJ_t) {
    if (*job).external_context {
        gvputs(job, (*job).imagedata);
    } else {
        gvputs(job, b"$c\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn tkgen_comment(mut job: *mut GVJ_t, mut str: *mut libc::c_char) {
    gvputs(job, b"# \0" as *const u8 as *const libc::c_char);
    gvputs(job, tkgen_string(str));
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn tkgen_begin_job(mut job: *mut GVJ_t) {
    gvputs(
        job,
        b"# Generated by \0" as *const u8 as *const libc::c_char,
    );
    gvputs(
        job,
        tkgen_string(*((*(*job).common).info).offset(0 as libc::c_int as isize)),
    );
    gvputs(job, b" version \0" as *const u8 as *const libc::c_char);
    gvputs(
        job,
        tkgen_string(*((*(*job).common).info).offset(1 as libc::c_int as isize)),
    );
    gvputs(job, b" (\0" as *const u8 as *const libc::c_char);
    gvputs(
        job,
        tkgen_string(*((*(*job).common).info).offset(2 as libc::c_int as isize)),
    );
    gvputs(job, b")\n\0" as *const u8 as *const libc::c_char);
}
static mut first_periphery: libc::c_int = 0;
unsafe extern "C" fn tkgen_begin_graph(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    gvputs(job, b"#\0" as *const u8 as *const libc::c_char);
    if *(agnameof((*obj).u.g as *mut libc::c_void)).offset(0 as libc::c_int as isize) != 0 {
        gvputs(job, b" Title: \0" as *const u8 as *const libc::c_char);
        gvputs(job, tkgen_string(agnameof((*obj).u.g as *mut libc::c_void)));
    }
    gvprintf(
        job,
        b" Pages: %d\n\0" as *const u8 as *const libc::c_char,
        (*job).pagesArraySize.x * (*job).pagesArraySize.y,
    );
    first_periphery = 0 as libc::c_int;
}
unsafe extern "C" fn tkgen_begin_node(mut job: *mut GVJ_t) {
    first_periphery = 1 as libc::c_int;
}
unsafe extern "C" fn tkgen_begin_edge(mut job: *mut GVJ_t) {
    first_periphery = -(1 as libc::c_int);
}
unsafe extern "C" fn tkgen_textspan(mut job: *mut GVJ_t, mut p: pointf, mut span: *mut textspan_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut font: *const libc::c_char = 0 as *const libc::c_char;
    let mut pA: *mut PostscriptAlias = 0 as *mut PostscriptAlias;
    let mut size: libc::c_int = 0;
    if (*obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint {
        size = ((*(*span).font).size * (*job).zoom) as libc::c_int;
        if size != 0 {
            tkgen_canvas(job);
            gvputs(job, b" create text \0" as *const u8 as *const libc::c_char);
            p.y -= size as libc::c_double * 0.55f64;
            gvprintpointf(job, p);
            gvputs(job, b" -text {\0" as *const u8 as *const libc::c_char);
            gvputs(job, (*span).str_0);
            gvputs(job, b"}\0" as *const u8 as *const libc::c_char);
            gvputs(job, b" -fill \0" as *const u8 as *const libc::c_char);
            tkgen_print_color(job, (*obj).pencolor);
            gvputs(job, b" -font {\0" as *const u8 as *const libc::c_char);
            pA = (*(*span).font).postscript_alias;
            if !pA.is_null() {
                font = (*pA).family;
            } else {
                font = (*(*span).font).name;
            }
            gvputs(job, b"\"\0" as *const u8 as *const libc::c_char);
            gvputs(job, font);
            gvputs(job, b"\"\0" as *const u8 as *const libc::c_char);
            gvprintf(job, b" %d}\0" as *const u8 as *const libc::c_char, size);
            match (*span).just as libc::c_int {
                108 => {
                    gvputs(job, b" -anchor w\0" as *const u8 as *const libc::c_char);
                }
                114 => {
                    gvputs(job, b" -anchor e\0" as *const u8 as *const libc::c_char);
                }
                110 | _ => {}
            }
            tkgen_print_tags(job);
            gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
}
unsafe extern "C" fn tkgen_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut r: pointf = pointf { x: 0., y: 0. };
    if (*obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint {
        r.x = (*A.offset(1 as libc::c_int as isize)).x - (*A.offset(0 as libc::c_int as isize)).x;
        r.y = (*A.offset(1 as libc::c_int as isize)).y - (*A.offset(0 as libc::c_int as isize)).y;
        (*A.offset(0 as libc::c_int as isize)).x -= r.x;
        (*A.offset(0 as libc::c_int as isize)).y -= r.y;
        tkgen_canvas(job);
        gvputs(job, b" create oval \0" as *const u8 as *const libc::c_char);
        gvprintpointflist(job, A, 2 as libc::c_int);
        gvputs(job, b" -fill \0" as *const u8 as *const libc::c_char);
        if filled != 0 {
            tkgen_print_color(job, (*obj).fillcolor);
        } else if first_periphery != 0 {
            gvputs(job, b"white\0" as *const u8 as *const libc::c_char);
        } else {
            gvputs(job, b"\"\"\0" as *const u8 as *const libc::c_char);
        }
        if first_periphery == 1 as libc::c_int {
            first_periphery = 0 as libc::c_int;
        }
        gvputs(job, b" -width \0" as *const u8 as *const libc::c_char);
        gvprintdouble(job, (*obj).penwidth);
        gvputs(job, b" -outline \0" as *const u8 as *const libc::c_char);
        tkgen_print_color(job, (*obj).pencolor);
        if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 5\0" as *const u8 as *const libc::c_char);
        }
        if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 2\0" as *const u8 as *const libc::c_char);
        }
        tkgen_print_tags(job);
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn tkgen_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    if (*obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint {
        tkgen_canvas(job);
        gvputs(job, b" create line \0" as *const u8 as *const libc::c_char);
        gvprintpointflist(job, A, n);
        gvputs(job, b" -fill \0" as *const u8 as *const libc::c_char);
        tkgen_print_color(job, (*obj).pencolor);
        gvputs(job, b" -width \0" as *const u8 as *const libc::c_char);
        gvprintdouble(job, (*obj).penwidth);
        if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 5\0" as *const u8 as *const libc::c_char);
        }
        if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 2\0" as *const u8 as *const libc::c_char);
        }
        gvputs(
            job,
            b" -smooth bezier \0" as *const u8 as *const libc::c_char,
        );
        tkgen_print_tags(job);
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn tkgen_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    if (*obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint {
        tkgen_canvas(job);
        gvputs(
            job,
            b" create polygon \0" as *const u8 as *const libc::c_char,
        );
        gvprintpointflist(job, A, n);
        gvputs(job, b" -fill \0" as *const u8 as *const libc::c_char);
        if filled != 0 {
            tkgen_print_color(job, (*obj).fillcolor);
        } else if first_periphery != 0 {
            gvputs(job, b"white\0" as *const u8 as *const libc::c_char);
        } else {
            gvputs(job, b"\"\"\0" as *const u8 as *const libc::c_char);
        }
        if first_periphery == 1 as libc::c_int {
            first_periphery = 0 as libc::c_int;
        }
        gvputs(job, b" -width \0" as *const u8 as *const libc::c_char);
        gvprintdouble(job, (*obj).penwidth);
        gvputs(job, b" -outline \0" as *const u8 as *const libc::c_char);
        tkgen_print_color(job, (*obj).pencolor);
        if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 5\0" as *const u8 as *const libc::c_char);
        }
        if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 2\0" as *const u8 as *const libc::c_char);
        }
        tkgen_print_tags(job);
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn tkgen_polyline(mut job: *mut GVJ_t, mut A: *mut pointf, mut n: libc::c_int) {
    let mut obj: *mut obj_state_t = (*job).obj;
    if (*obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint {
        tkgen_canvas(job);
        gvputs(job, b" create line \0" as *const u8 as *const libc::c_char);
        gvprintpointflist(job, A, n);
        gvputs(job, b" -fill \0" as *const u8 as *const libc::c_char);
        tkgen_print_color(job, (*obj).pencolor);
        if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 5\0" as *const u8 as *const libc::c_char);
        }
        if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
            gvputs(job, b" -dash 2\0" as *const u8 as *const libc::c_char);
        }
        tkgen_print_tags(job);
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub static mut tkgen_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: Some(tkgen_begin_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_job: None,
            begin_graph: Some(tkgen_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_graph: None,
            begin_layer: None,
            end_layer: None,
            begin_page: None,
            end_page: None,
            begin_cluster: None,
            end_cluster: None,
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: Some(tkgen_begin_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_node: None,
            begin_edge: Some(tkgen_begin_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_edge: None,
            begin_anchor: None,
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                tkgen_textspan as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: None,
            ellipse: Some(
                tkgen_ellipse as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                tkgen_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                tkgen_bezier
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
                tkgen_polyline as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: Some(
                tkgen_comment as unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> (),
            ),
            library_shape: None,
        };
        init
    }
};
#[no_mangle]
pub static mut render_features_tk: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 12 as libc::c_int | (1 as libc::c_int) << 25 as libc::c_int,
        default_pad: 4.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: COLOR_STRING,
    };
    init
};
#[no_mangle]
pub static mut device_features_tk: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: 0 as libc::c_int,
        default_margin: {
            let mut init = pointf_s {
                x: 0.0f64,
                y: 0.0f64,
            };
            init
        },
        default_pagesize: {
            let mut init = pointf_s {
                x: 0.0f64,
                y: 0.0f64,
            };
            init
        },
        default_dpi: {
            let mut init = pointf_s {
                x: 96.0f64,
                y: 96.0f64,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_tk_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_TK as libc::c_int,
                type_0: b"tk\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &tkgen_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_tk as *const gvrender_features_t
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
pub static mut gvdevice_tk_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_TK as libc::c_int,
                type_0: b"tk:tk\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_tk as *const gvdevice_features_t
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
