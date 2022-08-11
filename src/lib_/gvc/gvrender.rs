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
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn colorxlate(
        str: *mut libc::c_char,
        color: *mut gvcolor_t,
        target_type: color_type_t,
    ) -> libc::c_int;
    fn canontoken(str: *mut libc::c_char) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn gvplugin_load(
        gvc: *mut GVC_t,
        api: api_t,
        type_0: *const libc::c_char,
    ) -> *mut gvplugin_available_t;
    fn gvloadimage(
        job: *mut GVJ_t,
        us: *mut usershape_t,
        b: boxf,
        filled: bool,
        target: *const libc::c_char,
    );
    fn gvusershape_size_dpi(us: *mut usershape_t, dpi: pointf) -> point;
    fn gvusershape_find(name: *const libc::c_char) -> *mut usershape_t;
    fn gvdevice_initialize(job: *mut GVJ_t) -> libc::c_int;
    fn gvdevice_format(job: *mut GVJ_t);
    fn gvdevice_finalize(job: *mut GVJ_t);
    fn emit_once(str: *mut libc::c_char) -> libc::c_int;
    fn find_user_shape(name: *mut libc::c_char) -> *mut shape_desc;
    fn mapbool(s: *const libc::c_char) -> bool;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub type color_type_t = libc::c_uint;
pub const COLOR_INDEX: color_type_t = 6;
pub const COLOR_STRING: color_type_t = 5;
pub const RGBA_DOUBLE: color_type_t = 4;
pub const CMYK_BYTE: color_type_t = 3;
pub const RGBA_WORD: color_type_t = 2;
pub const RGBA_BYTE: color_type_t = 1;
pub const HSVA_DOUBLE: color_type_t = 0;
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
pub type gvcolor_t = color_s;
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
pub type api_t = libc::c_uint;
pub const API_loadimage: api_t = 4;
pub const API_device: api_t = 3;
pub const API_textlayout: api_t = 2;
pub const API_layout: api_t = 1;
pub const API_render: api_t = 0;
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
pub type uint64_t = __uint64_t;
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
pub type imagetype_t = libc::c_uint;
pub const FT_TIFF: imagetype_t = 13;
pub const FT_ICO: imagetype_t = 12;
pub const FT_WEBP: imagetype_t = 11;
pub const FT_RIFF: imagetype_t = 10;
pub const FT_XML: imagetype_t = 9;
pub const FT_SVG: imagetype_t = 8;
pub const FT_EPS: imagetype_t = 7;
pub const FT_PS: imagetype_t = 6;
pub const FT_PDF: imagetype_t = 5;
pub const FT_JPEG: imagetype_t = 4;
pub const FT_PNG: imagetype_t = 3;
pub const FT_GIF: imagetype_t = 2;
pub const FT_BMP: imagetype_t = 1;
pub const FT_NULL: imagetype_t = 0;
pub type imagescale_t = libc::c_uint;
pub const IMAGESCALE_BOTH: imagescale_t = 4;
pub const IMAGESCALE_HEIGHT: imagescale_t = 3;
pub const IMAGESCALE_WIDTH: imagescale_t = 2;
pub const IMAGESCALE_TRUE: imagescale_t = 1;
pub const IMAGESCALE_FALSE: imagescale_t = 0;
pub type imagepos_t = libc::c_uint;
pub const IMAGEPOS_BOTTOM_RIGHT: imagepos_t = 8;
pub const IMAGEPOS_BOTTOM_CENTER: imagepos_t = 7;
pub const IMAGEPOS_BOTTOM_LEFT: imagepos_t = 6;
pub const IMAGEPOS_MIDDLE_RIGHT: imagepos_t = 5;
pub const IMAGEPOS_MIDDLE_CENTER: imagepos_t = 4;
pub const IMAGEPOS_MIDDLE_LEFT: imagepos_t = 3;
pub const IMAGEPOS_TOP_RIGHT: imagepos_t = 2;
pub const IMAGEPOS_TOP_CENTER: imagepos_t = 1;
pub const IMAGEPOS_TOP_LEFT: imagepos_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usershape_s {
    pub link: Dtlink_t,
    pub name: *const libc::c_char,
    pub macro_id: libc::c_int,
    pub must_inline: bool,
    pub nocache: bool,
    pub f: *mut FILE,
    pub type_0: imagetype_t,
    pub stringtype: *mut libc::c_char,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub dpi: libc::c_int,
    pub data: *mut libc::c_void,
    pub datasize: size_t,
    pub datafree: Option<unsafe extern "C" fn(*mut usershape_t) -> ()>,
}
pub type usershape_t = usershape_s;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
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
#[no_mangle]
pub unsafe extern "C" fn gvrender_select(
    mut job: *mut GVJ_t,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut gvc: *mut GVC_t = (*job).gvc;
    let mut plugin: *mut gvplugin_available_t = 0 as *mut gvplugin_available_t;
    let mut typeptr: *mut gvplugin_installed_t = 0 as *mut gvplugin_installed_t;
    gvplugin_load(gvc, API_device, str);
    plugin = (*gvc).api[API_device as libc::c_int as usize];
    if !plugin.is_null() {
        typeptr = (*plugin).typeptr;
        let ref mut fresh0 = (*job).device.engine;
        *fresh0 = (*typeptr).engine as *mut gvdevice_engine_t;
        let ref mut fresh1 = (*job).device.features;
        *fresh1 = (*typeptr).features as *mut gvdevice_features_t;
        (*job).device.id = (*typeptr).id;
        let ref mut fresh2 = (*job).device.type_0;
        *fresh2 = (*plugin).typestr;
        (*job).flags |= (*(*job).device.features).flags;
    } else {
        return 999 as libc::c_int;
    }
    plugin = (*gvc).api[API_render as libc::c_int as usize];
    if !plugin.is_null() {
        typeptr = (*plugin).typeptr;
        let ref mut fresh3 = (*job).render.engine;
        *fresh3 = (*typeptr).engine as *mut gvrender_engine_t;
        let ref mut fresh4 = (*job).render.features;
        *fresh4 = (*typeptr).features as *mut gvrender_features_t;
        let ref mut fresh5 = (*job).render.type_0;
        *fresh5 = (*plugin).typestr;
        (*job).flags |= (*(*job).render.features).flags;
        if !((*job).device.engine).is_null() {
            (*job).render.id = (*typeptr).id;
        } else {
            (*job).render.id = (*job).device.id;
        }
        return 300 as libc::c_int;
    }
    let ref mut fresh6 = (*job).render.engine;
    *fresh6 = 0 as *mut gvrender_engine_t;
    return 999 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_features(mut job: *mut GVJ_t) -> libc::c_int {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    let mut features: libc::c_int = 0 as libc::c_int;
    if !gvre.is_null() {
        features = (*(*job).render.features).flags;
    }
    return features;
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_job(mut job: *mut GVJ_t) -> libc::c_int {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if gvdevice_initialize(job) != 0 {
        return 1 as libc::c_int;
    }
    if !gvre.is_null() {
        if ((*gvre).begin_job).is_some() {
            ((*gvre).begin_job).expect("non-null function pointer")(job);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_job(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_job).is_some() {
            ((*gvre).end_job).expect("non-null function pointer")(job);
        }
    }
    let ref mut fresh7 = (*(*job).gvc).common.lib;
    *fresh7 = 0 as *mut *const libc::c_char;
    gvdevice_finalize(job);
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_ptf(mut job: *mut GVJ_t, mut p: pointf) -> pointf {
    let mut rv: pointf = pointf { x: 0., y: 0. };
    let mut translation: pointf = pointf { x: 0., y: 0. };
    let mut scale: pointf = pointf { x: 0., y: 0. };
    translation = (*job).translation;
    scale.x = (*job).zoom * (*job).devscale.x;
    scale.y = (*job).zoom * (*job).devscale.y;
    if (*job).rotation != 0 {
        rv.x = -(p.y + translation.y) * scale.x;
        rv.y = (p.x + translation.x) * scale.y;
    } else {
        rv.x = (p.x + translation.x) * scale.x;
        rv.y = (p.y + translation.y) * scale.y;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_ptf_A(
    mut job: *mut GVJ_t,
    mut af: *mut pointf,
    mut AF: *mut pointf,
    mut n: libc::c_int,
) -> *mut pointf {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    let mut translation: pointf = pointf { x: 0., y: 0. };
    let mut scale: pointf = pointf { x: 0., y: 0. };
    translation = (*job).translation;
    scale.x = (*job).zoom * (*job).devscale.x;
    scale.y = (*job).zoom * (*job).devscale.y;
    if (*job).rotation != 0 {
        i = 0 as libc::c_int;
        while i < n {
            t = -((*af.offset(i as isize)).y + translation.y) * scale.x;
            (*AF.offset(i as isize)).y = ((*af.offset(i as isize)).x + translation.x) * scale.y;
            (*AF.offset(i as isize)).x = t;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            (*AF.offset(i as isize)).x = ((*af.offset(i as isize)).x + translation.x) * scale.x;
            (*AF.offset(i as isize)).y = ((*af.offset(i as isize)).y + translation.y) * scale.y;
            i += 1;
        }
    }
    return AF;
}
unsafe extern "C" fn gvrender_comparestr(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    return strcmp(
        *(s1 as *const *mut libc::c_char),
        *(s2 as *const *mut libc::c_char),
    );
}
unsafe extern "C" fn gvrender_resolve_color(
    mut features: *mut gvrender_features_t,
    mut name: *mut libc::c_char,
    mut color: *mut gvcolor_t,
) {
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    let ref mut fresh8 = (*color).u.string;
    *fresh8 = name;
    (*color).type_0 = COLOR_STRING;
    tok = canontoken(name);
    if ((*features).knowncolors).is_null()
        || (bsearch(
            &mut tok as *mut *mut libc::c_char as *const libc::c_void,
            (*features).knowncolors as *const libc::c_void,
            (*features).sz_knowncolors as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            Some(
                gvrender_comparestr
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ))
        .is_null()
    {
        rc = colorxlate(name, color, (*features).color_type);
        if rc != 0 as libc::c_int {
            if rc == 1 as libc::c_int {
                let mut missedcolor: *mut libc::c_char =
                    gmalloc((strlen(name)).wrapping_add(16 as libc::c_int as libc::c_ulong))
                        as *mut libc::c_char;
                sprintf(
                    missedcolor,
                    b"color %s\0" as *const u8 as *const libc::c_char,
                    name,
                );
                if emit_once(missedcolor) != 0 {
                    agerr(
                        AGWARN,
                        b"%s is not a known color.\n\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
                free(missedcolor as *mut libc::c_void);
            } else {
                agerr(
                    AGERR,
                    b"error in colxlate()\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_graph(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_graph).is_some() {
            ((*gvre).begin_graph).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_graph(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_graph).is_some() {
            ((*gvre).end_graph).expect("non-null function pointer")(job);
        }
    }
    gvdevice_format(job);
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_page(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_page).is_some() {
            ((*gvre).begin_page).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_page(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_page).is_some() {
            ((*gvre).end_page).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_layer(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_layer).is_some() {
            ((*gvre).begin_layer).expect("non-null function pointer")(
                job,
                *((*(*job).gvc).layerIDs).offset((*job).layerNum as isize),
                (*job).layerNum,
                (*job).numLayers,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_layer(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_layer).is_some() {
            ((*gvre).end_layer).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_cluster(mut job: *mut GVJ_t, mut sg: *mut graph_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_cluster).is_some() {
            ((*gvre).begin_cluster).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_cluster(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_cluster).is_some() {
            ((*gvre).end_cluster).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_nodes(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_nodes).is_some() {
            ((*gvre).begin_nodes).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_nodes(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_nodes).is_some() {
            ((*gvre).end_nodes).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_edges(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_edges).is_some() {
            ((*gvre).begin_edges).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_edges(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_edges).is_some() {
            ((*gvre).end_edges).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_node(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_node).is_some() {
            ((*gvre).begin_node).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_node(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_node).is_some() {
            ((*gvre).end_node).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_edge(mut job: *mut GVJ_t, mut e: *mut edge_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_edge).is_some() {
            ((*gvre).begin_edge).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_edge(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_edge).is_some() {
            ((*gvre).end_edge).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_anchor(
    mut job: *mut GVJ_t,
    mut href: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_anchor).is_some() {
            ((*gvre).begin_anchor).expect("non-null function pointer")(
                job, href, tooltip, target, id,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_anchor(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_anchor).is_some() {
            ((*gvre).end_anchor).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_begin_label(mut job: *mut GVJ_t, mut type_0: label_type) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).begin_label).is_some() {
            ((*gvre).begin_label).expect("non-null function pointer")(job, type_0);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_end_label(mut job: *mut GVJ_t) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).end_label).is_some() {
            ((*gvre).end_label).expect("non-null function pointer")(job);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_textspan(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut span: *mut textspan_t,
) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    let mut PF: pointf = pointf { x: 0., y: 0. };
    if !((*span).str_0).is_null()
        && *((*span).str_0).offset(0 as libc::c_int as isize) as libc::c_int != 0
        && (((*job).obj).is_null()
            || (*(*job).obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint)
    {
        if (*job).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
            PF = p;
        } else {
            PF = gvrender_ptf(job, p);
        }
        if !gvre.is_null() {
            if ((*gvre).textspan).is_some() {
                ((*gvre).textspan).expect("non-null function pointer")(job, PF, span);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_set_pencolor(mut job: *mut GVJ_t, mut name: *mut libc::c_char) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    let mut color: *mut gvcolor_t = &mut (*(*job).obj).pencolor;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strchr(name, ':' as i32);
    if !cp.is_null() {
        *cp = '\0' as i32 as libc::c_char;
    }
    if !gvre.is_null() {
        gvrender_resolve_color((*job).render.features, name, color);
        if ((*gvre).resolve_color).is_some() {
            ((*gvre).resolve_color).expect("non-null function pointer")(job, color);
        }
    }
    if !cp.is_null() {
        *cp = ':' as i32 as libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_set_fillcolor(mut job: *mut GVJ_t, mut name: *mut libc::c_char) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    let mut color: *mut gvcolor_t = &mut (*(*job).obj).fillcolor;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strchr(name, ':' as i32);
    if !cp.is_null() {
        *cp = '\0' as i32 as libc::c_char;
    }
    if !gvre.is_null() {
        gvrender_resolve_color((*job).render.features, name, color);
        if ((*gvre).resolve_color).is_some() {
            ((*gvre).resolve_color).expect("non-null function pointer")(job, color);
        }
    }
    if !cp.is_null() {
        *cp = ':' as i32 as libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_set_gradient_vals(
    mut job: *mut GVJ_t,
    mut stopcolor: *mut libc::c_char,
    mut angle: libc::c_int,
    mut frac: libc::c_float,
) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    let mut color: *mut gvcolor_t = &mut (*(*job).obj).stopcolor;
    if !gvre.is_null() {
        gvrender_resolve_color((*job).render.features, stopcolor, color);
        if ((*gvre).resolve_color).is_some() {
            ((*gvre).resolve_color).expect("non-null function pointer")(job, color);
        }
    }
    (*(*job).obj).gradient_angle = angle;
    (*(*job).obj).gradient_frac = frac;
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_set_style(mut job: *mut GVJ_t, mut s: *mut *mut libc::c_char) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let ref mut fresh9 = (*obj).rawstyle;
    *fresh9 = s;
    if !gvre.is_null() {
        if !s.is_null() {
            loop {
                let fresh10 = s;
                s = s.offset(1);
                line = *fresh10;
                p = line;
                if p.is_null() {
                    break;
                }
                if strcmp(line, b"solid\0" as *const u8 as *const libc::c_char) == 0 {
                    (*obj).pen = PEN_SOLID;
                } else if strcmp(line, b"dashed\0" as *const u8 as *const libc::c_char) == 0 {
                    (*obj).pen = PEN_DASHED;
                } else if strcmp(line, b"dotted\0" as *const u8 as *const libc::c_char) == 0 {
                    (*obj).pen = PEN_DOTTED;
                } else if strcmp(line, b"invis\0" as *const u8 as *const libc::c_char) == 0
                    || strcmp(line, b"invisible\0" as *const u8 as *const libc::c_char) == 0
                {
                    (*obj).pen = PEN_NONE;
                } else if strcmp(line, b"bold\0" as *const u8 as *const libc::c_char) == 0 {
                    (*obj).penwidth = 2.0f64;
                } else if strcmp(line, b"setlinewidth\0" as *const u8 as *const libc::c_char) == 0 {
                    while *p != 0 {
                        p = p.offset(1);
                    }
                    p = p.offset(1);
                    (*obj).penwidth = atof(p);
                } else if strcmp(line, b"filled\0" as *const u8 as *const libc::c_char) == 0 {
                    (*obj).fill = FILL_SOLID;
                } else if strcmp(line, b"unfilled\0" as *const u8 as *const libc::c_char) == 0 {
                    (*obj).fill = FILL_NONE;
                } else if !(strcmp(line, b"tapered\0" as *const u8 as *const libc::c_char) == 0) {
                    agerr(
                        AGWARN,
                        b"gvrender_set_style: unsupported style %s - ignoring\n\0" as *const u8
                            as *const libc::c_char,
                        line,
                    );
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_ellipse(
    mut job: *mut GVJ_t,
    mut pf: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).ellipse).is_some()
            && (*(*job).obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint
        {
            let mut af: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
            af[0 as libc::c_int as usize].x = ((*pf.offset(0 as libc::c_int as isize)).x
                + (*pf.offset(1 as libc::c_int as isize)).x)
                / 2.0f64;
            af[0 as libc::c_int as usize].y = ((*pf.offset(0 as libc::c_int as isize)).y
                + (*pf.offset(1 as libc::c_int as isize)).y)
                / 2.0f64;
            af[1 as libc::c_int as usize] = *pf.offset(1 as libc::c_int as isize);
            if (*job).flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
                gvrender_ptf_A(job, af.as_mut_ptr(), af.as_mut_ptr(), 2 as libc::c_int);
            }
            ((*gvre).ellipse).expect("non-null function pointer")(job, af.as_mut_ptr(), filled);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_polygon(
    mut job: *mut GVJ_t,
    mut af: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut noPoly: libc::c_int = 0 as libc::c_int;
    let mut save_pencolor: gvcolor_t = gvcolor_t {
        u: C2RustUnnamed { RGBA: [0.; 4] },
        type_0: HSVA_DOUBLE,
    };
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).polygon).is_some()
            && (*(*job).obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint
        {
            if filled & 4 as libc::c_int != 0 {
                noPoly = 1 as libc::c_int;
                filled &= !(4 as libc::c_int);
                save_pencolor = (*(*job).obj).pencolor;
                (*(*job).obj).pencolor = (*(*job).obj).fillcolor;
            }
            if (*job).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
                ((*gvre).polygon).expect("non-null function pointer")(job, af, n, filled);
            } else {
                let mut AF: *mut pointf = 0 as *mut pointf;
                if n >= 0 as libc::c_int {
                } else {
                    __assert_fail(
                        b"n >= 0\0" as *const u8 as *const libc::c_char,
                        b"gvrender.c\0" as *const u8 as *const libc::c_char,
                        570 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                            b"void gvrender_polygon(GVJ_t *, pointf *, int, int)\0",
                        ))
                        .as_ptr(),
                    );
                }
                AF = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<pointf>() as libc::c_ulong,
                ) as *mut pointf;
                gvrender_ptf_A(job, af, AF, n);
                ((*gvre).polygon).expect("non-null function pointer")(job, AF, n, filled);
                free(AF as *mut libc::c_void);
            }
            if noPoly != 0 {
                (*(*job).obj).pencolor = save_pencolor;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_box(mut job: *mut GVJ_t, mut B: boxf, mut filled: libc::c_int) {
    let mut A: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    A[0 as libc::c_int as usize] = B.LL;
    A[2 as libc::c_int as usize] = B.UR;
    A[1 as libc::c_int as usize].x = A[0 as libc::c_int as usize].x;
    A[1 as libc::c_int as usize].y = A[2 as libc::c_int as usize].y;
    A[3 as libc::c_int as usize].x = A[2 as libc::c_int as usize].x;
    A[3 as libc::c_int as usize].y = A[0 as libc::c_int as usize].y;
    gvrender_polygon(job, A.as_mut_ptr(), 4 as libc::c_int, filled);
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_beziercurve(
    mut job: *mut GVJ_t,
    mut af: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).beziercurve).is_some()
            && (*(*job).obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint
        {
            if (*job).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
                ((*gvre).beziercurve).expect("non-null function pointer")(
                    job,
                    af,
                    n,
                    arrow_at_start,
                    arrow_at_end,
                    filled,
                );
            } else {
                let mut AF: *mut pointf = 0 as *mut pointf;
                if n >= 0 as libc::c_int {
                } else {
                    __assert_fail(
                        b"n >= 0\0" as *const u8 as *const libc::c_char,
                        b"gvrender.c\0" as *const u8 as *const libc::c_char,
                        609 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                            b"void gvrender_beziercurve(GVJ_t *, pointf *, int, int, int, int)\0",
                        ))
                        .as_ptr(),
                    );
                }
                AF = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<pointf>() as libc::c_ulong,
                ) as *mut pointf;
                gvrender_ptf_A(job, af, AF, n);
                ((*gvre).beziercurve).expect("non-null function pointer")(
                    job,
                    AF,
                    n,
                    arrow_at_start,
                    arrow_at_end,
                    filled,
                );
                free(AF as *mut libc::c_void);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_polyline(
    mut job: *mut GVJ_t,
    mut af: *mut pointf,
    mut n: libc::c_int,
) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        if ((*gvre).polyline).is_some()
            && (*(*job).obj).pen as libc::c_uint != PEN_NONE as libc::c_int as libc::c_uint
        {
            if (*job).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
                ((*gvre).polyline).expect("non-null function pointer")(job, af, n);
            } else {
                let mut AF: *mut pointf = 0 as *mut pointf;
                if n >= 0 as libc::c_int {
                } else {
                    __assert_fail(
                        b"n >= 0\0" as *const u8 as *const libc::c_char,
                        b"gvrender.c\0" as *const u8 as *const libc::c_char,
                        630 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"void gvrender_polyline(GVJ_t *, pointf *, int)\0",
                        ))
                        .as_ptr(),
                    );
                }
                AF = gcalloc(
                    n as size_t,
                    ::std::mem::size_of::<pointf>() as libc::c_ulong,
                ) as *mut pointf;
                gvrender_ptf_A(job, af, AF, n);
                ((*gvre).polyline).expect("non-null function pointer")(job, AF, n);
                free(AF as *mut libc::c_void);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_comment(mut job: *mut GVJ_t, mut str: *mut libc::c_char) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if str.is_null() || *str.offset(0 as libc::c_int as isize) == 0 {
        return;
    }
    if !gvre.is_null() {
        if ((*gvre).comment).is_some() {
            ((*gvre).comment).expect("non-null function pointer")(job, str);
        }
    }
}
unsafe extern "C" fn get_imagescale(mut s: *mut libc::c_char) -> imagescale_t {
    if *s as libc::c_int == '\0' as i32 {
        return IMAGESCALE_FALSE;
    }
    if strcasecmp(s, b"width\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGESCALE_WIDTH;
    }
    if strcasecmp(s, b"height\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGESCALE_HEIGHT;
    }
    if strcasecmp(s, b"both\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGESCALE_BOTH;
    }
    if mapbool(s) {
        return IMAGESCALE_TRUE;
    }
    return IMAGESCALE_FALSE;
}
unsafe extern "C" fn get_imagepos(mut s: *mut libc::c_char) -> imagepos_t {
    if *s as libc::c_int == '\0' as i32 {
        return IMAGEPOS_MIDDLE_CENTER;
    }
    if strcasecmp(s, b"tl\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_TOP_LEFT;
    }
    if strcasecmp(s, b"tc\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_TOP_CENTER;
    }
    if strcasecmp(s, b"tr\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_TOP_RIGHT;
    }
    if strcasecmp(s, b"ml\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_MIDDLE_LEFT;
    }
    if strcasecmp(s, b"mc\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_MIDDLE_CENTER;
    }
    if strcasecmp(s, b"mr\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_MIDDLE_RIGHT;
    }
    if strcasecmp(s, b"bl\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_BOTTOM_LEFT;
    }
    if strcasecmp(s, b"bc\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_BOTTOM_CENTER;
    }
    if strcasecmp(s, b"br\0" as *const u8 as *const libc::c_char) == 0 {
        return IMAGEPOS_BOTTOM_RIGHT;
    }
    return IMAGEPOS_MIDDLE_CENTER;
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_usershape(
    mut job: *mut GVJ_t,
    mut name: *mut libc::c_char,
    mut a: *mut pointf,
    mut n: libc::c_int,
    mut filled: bool,
    mut imagescale: *mut libc::c_char,
    mut imagepos: *mut libc::c_char,
) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    let mut us: *mut usershape_t = 0 as *mut usershape_t;
    let mut iw: libc::c_double = 0.;
    let mut ih: libc::c_double = 0.;
    let mut pw: libc::c_double = 0.;
    let mut ph: libc::c_double = 0.;
    let mut scalex: libc::c_double = 0.;
    let mut scaley: libc::c_double = 0.;
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut i: libc::c_int = 0;
    let mut isz: point = point { x: 0, y: 0 };
    let mut position: imagepos_t = IMAGEPOS_TOP_LEFT;
    if !job.is_null() {
    } else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvrender.c\0" as *const u8 as *const libc::c_char,
            709 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 79], &[libc::c_char; 79]>(
                b"void gvrender_usershape(GVJ_t *, char *, pointf *, int, _Bool, char *, char *)\0",
            ))
            .as_ptr(),
        );
    }
    if !name.is_null() {
    } else {
        __assert_fail(
            b"name\0" as *const u8 as *const libc::c_char,
            b"gvrender.c\0" as *const u8 as *const libc::c_char,
            710 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 79], &[libc::c_char; 79]>(
                b"void gvrender_usershape(GVJ_t *, char *, pointf *, int, _Bool, char *, char *)\0",
            ))
            .as_ptr(),
        );
    }
    if *name.offset(0 as libc::c_int as isize) != 0 {
    } else {
        __assert_fail(
            b"name[0]\0" as *const u8 as *const libc::c_char,
            b"gvrender.c\0" as *const u8 as *const libc::c_char,
            711 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 79], &[libc::c_char; 79]>(
                b"void gvrender_usershape(GVJ_t *, char *, pointf *, int, _Bool, char *, char *)\0",
            ))
            .as_ptr(),
        );
    }
    us = gvusershape_find(name);
    if us.is_null() {
        if !(find_user_shape(name)).is_null() {
            if !gvre.is_null() && ((*gvre).library_shape).is_some() {
                ((*gvre).library_shape).expect("non-null function pointer")(
                    job,
                    name,
                    a,
                    n,
                    filled as libc::c_int,
                );
            }
        }
        return;
    }
    isz = gvusershape_size_dpi(us, (*job).dpi);
    if isz.x <= 0 as libc::c_int && isz.y <= 0 as libc::c_int {
        return;
    }
    b.UR = *a.offset(0 as libc::c_int as isize);
    b.LL = b.UR;
    i = 1 as libc::c_int;
    while i < n {
        b.LL.x = (if b.LL.x < (*a.offset(i as isize)).x {
            b.LL.x
        } else {
            (*a.offset(i as isize)).x
        });
        b.LL.y = (if b.LL.y < (*a.offset(i as isize)).y {
            b.LL.y
        } else {
            (*a.offset(i as isize)).y
        });
        b.UR.x = (if b.UR.x > (*a.offset(i as isize)).x {
            b.UR.x
        } else {
            (*a.offset(i as isize)).x
        });
        b.UR.y = (if b.UR.y > (*a.offset(i as isize)).y {
            b.UR.y
        } else {
            (*a.offset(i as isize)).y
        });
        i += 1;
    }
    pw = b.UR.x - b.LL.x;
    ph = b.UR.y - b.LL.y;
    ih = isz.y as libc::c_double;
    iw = isz.x as libc::c_double;
    scalex = pw / iw;
    scaley = ph / ih;
    match get_imagescale(imagescale) as libc::c_uint {
        1 => {
            if scalex < scaley {
                iw *= scalex;
                ih *= scalex;
            } else {
                iw *= scaley;
                ih *= scaley;
            }
        }
        2 => {
            iw *= scalex;
        }
        3 => {
            ih *= scaley;
        }
        4 => {
            iw *= scalex;
            ih *= scaley;
        }
        0 | _ => {}
    }
    position = get_imagepos(imagepos);
    if iw < pw {
        match position as libc::c_uint {
            0 | 3 | 6 => {
                b.UR.x = b.LL.x + iw;
            }
            2 | 5 | 8 => {
                b.LL.x += pw - iw;
                b.UR.x = b.LL.x + iw;
            }
            _ => {
                b.LL.x += (pw - iw) / 2.0f64;
                b.UR.x -= (pw - iw) / 2.0f64;
            }
        }
    }
    if ih < ph {
        match position as libc::c_uint {
            0 | 1 | 2 => {
                b.LL.y = b.UR.y - ih;
            }
            6 | 7 | 8 => {
                b.LL.y += ih;
                b.UR.y = b.LL.y - ih;
            }
            _ => {
                b.LL.y += (ph - ih) / 2.0f64;
                b.UR.y -= (ph - ih) / 2.0f64;
            }
        }
    }
    if (*job).flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
        b.LL = gvrender_ptf(job, b.LL);
        b.UR = gvrender_ptf(job, b.UR);
    }
    if b.LL.x > b.UR.x {
        let mut d: libc::c_double = b.LL.x;
        b.LL.x = b.UR.x;
        b.UR.x = d;
    }
    if b.LL.y > b.UR.y {
        let mut d_0: libc::c_double = b.LL.y;
        b.LL.y = b.UR.y;
        b.UR.y = d_0;
    }
    if !gvre.is_null() {
        gvloadimage(job, us, b, filled, (*job).render.type_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvrender_set_penwidth(mut job: *mut GVJ_t, mut penwidth: libc::c_double) {
    let mut gvre: *mut gvrender_engine_t = (*job).render.engine;
    if !gvre.is_null() {
        (*(*job).obj).penwidth = penwidth;
    }
}
