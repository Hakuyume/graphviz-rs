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
    pub type htmllabel_t;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn xml_escape(
        s: *const libc::c_char,
        flags: xml_flags_t,
        cb: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
        state: *mut libc::c_void,
    ) -> libc::c_int;
    fn strdup_and_subst_obj(str: *mut libc::c_char, obj: *mut libc::c_void) -> *mut libc::c_char;
    fn get_gradient_points(
        A: *mut pointf,
        G: *mut pointf,
        n: libc::c_int,
        angle: libc::c_double,
        flags: libc::c_int,
    );
    fn gvwrite(job: *mut GVJ_t, s: *const libc::c_char, len: size_t) -> size_t;
    fn gvputc(job: *mut GVJ_t, c: libc::c_int) -> libc::c_int;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvputs_xml(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn gvprintdouble(job: *mut GVJ_t, num: libc::c_double);
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
    pub u: C2RustUnnamed_4,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub txt: C2RustUnnamed_5,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct xml_flags_t {
    #[bitfield(name = "raw", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "dash", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "nbsp", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "utf8", ty = "libc::c_uint", bits = "3..=3")]
    pub raw_dash_nbsp_utf8: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const FORMAT_SVGZ: C2RustUnnamed_6 = 1;
pub const FORMAT_SVG: C2RustUnnamed_6 = 0;
static mut sdasharray: *mut libc::c_char =
    b"5,2\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut sdotarray: *mut libc::c_char =
    b"1,5\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn svg_bzptarray(mut job: *mut GVJ_t, mut A: *mut pointf, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    c = 'M' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < n {
        gvwrite(job, &mut c, 1 as libc::c_int as size_t);
        gvprintdouble(job, (*A.offset(i as isize)).x);
        gvputc(job, ',' as i32);
        gvprintdouble(job, -(*A.offset(i as isize)).y);
        if i == 0 as libc::c_int {
            c = 'C' as i32 as libc::c_char;
        } else {
            c = ' ' as i32 as libc::c_char;
        }
        i += 1;
    }
}
unsafe extern "C" fn svg_print_id_class(
    mut job: *mut GVJ_t,
    mut id: *mut libc::c_char,
    mut idx: *mut libc::c_char,
    mut kind: *mut libc::c_char,
    mut obj: *mut libc::c_void,
) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    gvputs(job, b"<g id=\"\0" as *const u8 as *const libc::c_char);
    gvputs_xml(job, id);
    if !idx.is_null() {
        gvputc(job, '_' as i32);
        gvputs_xml(job, idx);
    }
    gvprintf(
        job,
        b"\" class=\"%s\0" as *const u8 as *const libc::c_char,
        kind,
    );
    str = agget(
        obj,
        b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && *str as libc::c_int != 0 {
        gvputc(job, ' ' as i32);
        gvputs_xml(job, str);
    }
    gvputc(job, '"' as i32);
}
unsafe extern "C" fn svg_print_color(mut job: *mut GVJ_t, mut color: gvcolor_t) {
    match color.type_0 as libc::c_uint {
        5 => {
            gvputs(job, color.u.string);
        }
        1 => {
            if color.u.rgba[3 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
                gvputs(job, b"transparent\0" as *const u8 as *const libc::c_char);
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
                b"gvrender_core_svg.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void svg_print_color(GVJ_t *, gvcolor_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn svg_grstyle(
    mut job: *mut GVJ_t,
    mut filled: libc::c_int,
    mut gid: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    gvputs(job, b" fill=\"\0" as *const u8 as *const libc::c_char);
    if filled == 2 as libc::c_int {
        gvprintf(
            job,
            b"url(#l_%d)\0" as *const u8 as *const libc::c_char,
            gid,
        );
    } else if filled == 3 as libc::c_int {
        gvprintf(
            job,
            b"url(#r_%d)\0" as *const u8 as *const libc::c_char,
            gid,
        );
    } else if filled != 0 {
        svg_print_color(job, (*obj).fillcolor);
        if (*obj).fillcolor.type_0 as libc::c_uint == RGBA_BYTE as libc::c_int as libc::c_uint
            && (*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int
            && ((*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int)
                < 255 as libc::c_int
        {
            gvprintf(
                job,
                b"\" fill-opacity=\"%f\0" as *const u8 as *const libc::c_char,
                (*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_float
                    as libc::c_double
                    / 255.0f64,
            );
        }
    } else {
        gvputs(job, b"none\0" as *const u8 as *const libc::c_char);
    }
    gvputs(job, b"\" stroke=\"\0" as *const u8 as *const libc::c_char);
    svg_print_color(job, (*obj).pencolor);
    if (*obj).penwidth != 1.0f64 {
        gvputs(
            job,
            b"\" stroke-width=\"\0" as *const u8 as *const libc::c_char,
        );
        gvprintdouble(job, (*obj).penwidth);
    }
    if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
        gvprintf(
            job,
            b"\" stroke-dasharray=\"%s\0" as *const u8 as *const libc::c_char,
            sdasharray,
        );
    } else if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
        gvprintf(
            job,
            b"\" stroke-dasharray=\"%s\0" as *const u8 as *const libc::c_char,
            sdotarray,
        );
    }
    if (*obj).pencolor.type_0 as libc::c_uint == RGBA_BYTE as libc::c_int as libc::c_uint
        && (*obj).pencolor.u.rgba[3 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int
        && ((*obj).pencolor.u.rgba[3 as libc::c_int as usize] as libc::c_int) < 255 as libc::c_int
    {
        gvprintf(
            job,
            b"\" stroke-opacity=\"%f\0" as *const u8 as *const libc::c_char,
            (*obj).pencolor.u.rgba[3 as libc::c_int as usize] as libc::c_float as libc::c_double
                / 255.0f64,
        );
    }
    gvputc(job, '"' as i32);
}
unsafe extern "C" fn svg_comment(mut job: *mut GVJ_t, mut str: *mut libc::c_char) {
    gvputs(job, b"<!-- \0" as *const u8 as *const libc::c_char);
    gvputs_xml(job, str);
    gvputs(job, b" -->\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_job(mut job: *mut GVJ_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    gvputs(
        job,
        b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n\0" as *const u8
            as *const libc::c_char,
    );
    s = agget(
        (*(*job).gvc).g as *mut libc::c_void,
        b"stylesheet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        gvputs(
            job,
            b"<?xml-stylesheet href=\"\0" as *const u8 as *const libc::c_char,
        );
        gvputs(job, s);
        gvputs(
            job,
            b"\" type=\"text/css\"?>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    gvputs(
        job,
        b"<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\"\n \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n<!-- Generated by \0"
            as *const u8 as *const libc::c_char,
    );
    gvputs_xml(
        job,
        *((*(*job).common).info).offset(0 as libc::c_int as isize),
    );
    gvputs(job, b" version \0" as *const u8 as *const libc::c_char);
    gvputs_xml(
        job,
        *((*(*job).common).info).offset(1 as libc::c_int as isize),
    );
    gvputs(job, b" (\0" as *const u8 as *const libc::c_char);
    gvputs_xml(
        job,
        *((*(*job).common).info).offset(2 as libc::c_int as isize),
    );
    gvputs(job, b")\n -->\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_graph(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    gvputs(job, b"<!--\0" as *const u8 as *const libc::c_char);
    if *(agnameof((*obj).u.g as *mut libc::c_void)).offset(0 as libc::c_int as isize) as libc::c_int
        != 0
        && *(agnameof((*obj).u.g as *mut libc::c_void)).offset(0 as libc::c_int as isize)
            as libc::c_int
            != '%' as i32
    {
        gvputs(job, b" Title: \0" as *const u8 as *const libc::c_char);
        gvputs_xml(job, agnameof((*obj).u.g as *mut libc::c_void));
    }
    gvprintf(
        job,
        b" Pages: %d -->\n\0" as *const u8 as *const libc::c_char,
        (*job).pagesArraySize.x * (*job).pagesArraySize.y,
    );
    gvprintf(
        job,
        b"<svg width=\"%dpt\" height=\"%dpt\"\n\0" as *const u8 as *const libc::c_char,
        (*job).width,
        (*job).height,
    );
    gvprintf(
        job,
        b" viewBox=\"%.2f %.2f %.2f %.2f\"\0" as *const u8 as *const libc::c_char,
        (*job).canvasBox.LL.x,
        (*job).canvasBox.LL.y,
        (*job).canvasBox.UR.x,
        (*job).canvasBox.UR.y,
    );
    gvputs(
        job,
        b" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn svg_end_graph(mut job: *mut GVJ_t) {
    gvputs(job, b"</svg>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_layer(
    mut job: *mut GVJ_t,
    mut layername: *mut libc::c_char,
    mut layerNum: libc::c_int,
    mut numLayers: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    svg_print_id_class(
        job,
        layername,
        0 as *mut libc::c_char,
        b"layer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*obj).u.g as *mut libc::c_void,
    );
    gvputs(job, b">\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_end_layer(mut job: *mut GVJ_t) {
    gvputs(job, b"</g>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_page(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    svg_print_id_class(
        job,
        (*obj).id,
        0 as *mut libc::c_char,
        b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*obj).u.g as *mut libc::c_void,
    );
    gvputs(
        job,
        b" transform=\"scale(\0" as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"%g %g\0" as *const u8 as *const libc::c_char,
        (*job).scale.x,
        (*job).scale.y,
    );
    gvprintf(
        job,
        b") rotate(%d) translate(\0" as *const u8 as *const libc::c_char,
        -(*job).rotation,
    );
    gvprintdouble(job, (*job).translation.x);
    gvputc(job, ' ' as i32);
    gvprintdouble(job, -(*job).translation.y);
    gvputs(job, b")\">\n\0" as *const u8 as *const libc::c_char);
    if *(agnameof((*obj).u.g as *mut libc::c_void)).offset(0 as libc::c_int as isize) as libc::c_int
        != 0
        && *(agnameof((*obj).u.g as *mut libc::c_void)).offset(0 as libc::c_int as isize)
            as libc::c_int
            != '%' as i32
    {
        gvputs(job, b"<title>\0" as *const u8 as *const libc::c_char);
        gvputs_xml(job, agnameof((*obj).u.g as *mut libc::c_void));
        gvputs(job, b"</title>\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn svg_end_page(mut job: *mut GVJ_t) {
    gvputs(job, b"</g>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_cluster(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    svg_print_id_class(
        job,
        (*obj).id,
        0 as *mut libc::c_char,
        b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*obj).u.sg as *mut libc::c_void,
    );
    gvputs(job, b">\n<title>\0" as *const u8 as *const libc::c_char);
    gvputs_xml(job, agnameof((*obj).u.g as *mut libc::c_void));
    gvputs(job, b"</title>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_end_cluster(mut job: *mut GVJ_t) {
    gvputs(job, b"</g>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_node(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut idx: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*job).layerNum > 1 as libc::c_int {
        idx = *((*(*job).gvc).layerIDs).offset((*job).layerNum as isize);
    } else {
        idx = 0 as *mut libc::c_char;
    }
    svg_print_id_class(
        job,
        (*obj).id,
        idx,
        b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*obj).u.n as *mut libc::c_void,
    );
    gvputs(job, b">\n<title>\0" as *const u8 as *const libc::c_char);
    gvputs_xml(job, agnameof((*obj).u.n as *mut libc::c_void));
    gvputs(job, b"</title>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_end_node(mut job: *mut GVJ_t) {
    gvputs(job, b"</g>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_edge(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut ename: *mut libc::c_char = 0 as *mut libc::c_char;
    svg_print_id_class(
        job,
        (*obj).id,
        0 as *mut libc::c_char,
        b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*obj).u.e as *mut libc::c_void,
    );
    gvputs(job, b">\n<title>\0" as *const u8 as *const libc::c_char);
    ename = strdup_and_subst_obj(
        b"\\E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*obj).u.e as *mut libc::c_void,
    );
    gvputs_xml(job, ename);
    free(ename as *mut libc::c_void);
    gvputs(job, b"</title>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_end_edge(mut job: *mut GVJ_t) {
    gvputs(job, b"</g>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_begin_anchor(
    mut job: *mut GVJ_t,
    mut href: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
) {
    gvputs(job, b"<g\0" as *const u8 as *const libc::c_char);
    if !id.is_null() {
        gvputs(job, b" id=\"a_\0" as *const u8 as *const libc::c_char);
        gvputs_xml(job, id);
        gvputc(job, '"' as i32);
    }
    gvputs(job, b"><a\0" as *const u8 as *const libc::c_char);
    if !href.is_null() && *href.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        gvputs(job, b" xlink:href=\"\0" as *const u8 as *const libc::c_char);
        let flags: xml_flags_t = {
            let mut init = xml_flags_t {
                raw_dash_nbsp_utf8: [0; 1],
                c2rust_padding: [0; 3],
            };
            init.set_raw(0 as libc::c_int as libc::c_uint);
            init.set_dash(0);
            init.set_nbsp(0);
            init.set_utf8(0);
            init
        };
        xml_escape(
            href,
            flags,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int>,
                Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
            >(Some(
                gvputs as unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int,
            )),
            job as *mut libc::c_void,
        );
        gvputc(job, '"' as i32);
    }
    if !tooltip.is_null() && *tooltip.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        gvputs(
            job,
            b" xlink:title=\"\0" as *const u8 as *const libc::c_char,
        );
        let flags_0: xml_flags_t = {
            let mut init = xml_flags_t {
                raw_dash_nbsp_utf8: [0; 1],
                c2rust_padding: [0; 3],
            };
            init.set_raw(1 as libc::c_int as libc::c_uint);
            init.set_dash(1 as libc::c_int as libc::c_uint);
            init.set_nbsp(1 as libc::c_int as libc::c_uint);
            init.set_utf8(0);
            init
        };
        xml_escape(
            tooltip,
            flags_0,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int>,
                Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
            >(Some(
                gvputs as unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int,
            )),
            job as *mut libc::c_void,
        );
        gvputc(job, '"' as i32);
    }
    if !target.is_null() && *target.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        gvputs(job, b" target=\"\0" as *const u8 as *const libc::c_char);
        gvputs_xml(job, target);
        gvputc(job, '"' as i32);
    }
    gvputs(job, b">\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_end_anchor(mut job: *mut GVJ_t) {
    gvputs(job, b"</a>\n</g>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_textspan(mut job: *mut GVJ_t, mut p: pointf, mut span: *mut textspan_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut pA: *mut PostscriptAlias = 0 as *mut PostscriptAlias;
    let mut family: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut weight: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stretch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut style: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_uint = 0;
    gvputs(job, b"<text\0" as *const u8 as *const libc::c_char);
    match (*span).just as libc::c_int {
        108 => {
            gvputs(
                job,
                b" text-anchor=\"start\"\0" as *const u8 as *const libc::c_char,
            );
        }
        114 => {
            gvputs(
                job,
                b" text-anchor=\"end\"\0" as *const u8 as *const libc::c_char,
            );
        }
        110 | _ => {
            gvputs(
                job,
                b" text-anchor=\"middle\"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    p.y += (*span).yoffset_centerline;
    if (*obj).labeledgealigned() == 0 {
        gvputs(job, b" x=\"\0" as *const u8 as *const libc::c_char);
        gvprintdouble(job, p.x);
        gvputs(job, b"\" y=\"\0" as *const u8 as *const libc::c_char);
        gvprintdouble(job, -p.y);
        gvputs(job, b"\"\0" as *const u8 as *const libc::c_char);
    }
    pA = (*(*span).font).postscript_alias;
    if !pA.is_null() {
        match (*((*((*(*job).gvc).g as *mut Agobj_t)).data as *mut Agraphinfo_t)).fontnames
            as libc::c_uint
        {
            1 => {
                family = (*pA).name;
                weight = (*pA).weight;
                style = (*pA).style;
            }
            2 => {
                family = (*pA).svg_font_family;
                weight = (*pA).svg_font_weight;
                style = (*pA).svg_font_style;
            }
            0 | _ => {
                family = (*pA).family;
                weight = (*pA).weight;
                style = (*pA).style;
            }
        }
        stretch = (*pA).stretch;
        gvprintf(
            job,
            b" font-family=\"%s\0" as *const u8 as *const libc::c_char,
            family,
        );
        if !((*pA).svg_font_family).is_null() {
            gvprintf(
                job,
                b",%s\0" as *const u8 as *const libc::c_char,
                (*pA).svg_font_family,
            );
        }
        gvputc(job, '"' as i32);
        if !weight.is_null() {
            gvprintf(
                job,
                b" font-weight=\"%s\"\0" as *const u8 as *const libc::c_char,
                weight,
            );
        }
        if !stretch.is_null() {
            gvprintf(
                job,
                b" font-stretch=\"%s\"\0" as *const u8 as *const libc::c_char,
                stretch,
            );
        }
        if !style.is_null() {
            gvprintf(
                job,
                b" font-style=\"%s\"\0" as *const u8 as *const libc::c_char,
                style,
            );
        }
    } else {
        gvprintf(
            job,
            b" font-family=\"%s\"\0" as *const u8 as *const libc::c_char,
            (*(*span).font).name,
        );
    }
    if !((*span).font).is_null() && {
        flags = (*(*span).font).flags();
        flags != 0
    } {
        if flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 && weight.is_null()
        {
            gvputs(
                job,
                b" font-weight=\"bold\"\0" as *const u8 as *const libc::c_char,
            );
        }
        if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 && style.is_null()
        {
            gvputs(
                job,
                b" font-style=\"italic\"\0" as *const u8 as *const libc::c_char,
            );
        }
        if flags
            & ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint
            != 0
        {
            let mut comma: libc::c_int = 0 as libc::c_int;
            gvputs(
                job,
                b" text-decoration=\"\0" as *const u8 as *const libc::c_char,
            );
            if flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
                gvputs(job, b"underline\0" as *const u8 as *const libc::c_char);
                comma = 1 as libc::c_int;
            }
            if flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint != 0 {
                gvprintf(
                    job,
                    b"%soverline\0" as *const u8 as *const libc::c_char,
                    if comma != 0 {
                        b",\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                comma = 1 as libc::c_int;
            }
            if flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint != 0 {
                gvprintf(
                    job,
                    b"%sline-through\0" as *const u8 as *const libc::c_char,
                    if comma != 0 {
                        b",\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            gvputc(job, '"' as i32);
        }
        if flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            gvputs(
                job,
                b" baseline-shift=\"super\"\0" as *const u8 as *const libc::c_char,
            );
        }
        if flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0 {
            gvputs(
                job,
                b" baseline-shift=\"sub\"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    gvprintf(
        job,
        b" font-size=\"%.2f\"\0" as *const u8 as *const libc::c_char,
        (*(*span).font).size,
    );
    match (*obj).pencolor.type_0 as libc::c_uint {
        5 => {
            if strcasecmp(
                (*obj).pencolor.u.string,
                b"black\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                gvprintf(
                    job,
                    b" fill=\"%s\"\0" as *const u8 as *const libc::c_char,
                    (*obj).pencolor.u.string,
                );
            }
        }
        1 => {
            gvprintf(
                job,
                b" fill=\"#%02x%02x%02x\"\0" as *const u8 as *const libc::c_char,
                (*obj).pencolor.u.rgba[0 as libc::c_int as usize] as libc::c_int,
                (*obj).pencolor.u.rgba[1 as libc::c_int as usize] as libc::c_int,
                (*obj).pencolor.u.rgba[2 as libc::c_int as usize] as libc::c_int,
            );
            if ((*obj).pencolor.u.rgba[3 as libc::c_int as usize] as libc::c_int)
                < 255 as libc::c_int
            {
                gvprintf(
                    job,
                    b" fill-opacity=\"%f\"\0" as *const u8 as *const libc::c_char,
                    (*obj).pencolor.u.rgba[3 as libc::c_int as usize] as libc::c_float
                        as libc::c_double
                        / 255.0f64,
                );
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"gvrender_core_svg.c\0" as *const u8 as *const libc::c_char,
                470 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void svg_textspan(GVJ_t *, pointf, textspan_t *)\0",
                ))
                .as_ptr(),
            );
        }
    }
    gvputc(job, '>' as i32);
    if (*obj).labeledgealigned() != 0 {
        gvputs(
            job,
            b"<textPath xlink:href=\"#\0" as *const u8 as *const libc::c_char,
        );
        gvputs_xml(job, (*obj).id);
        gvputs(
            job,
            b"_p\" startOffset=\"50%\"><tspan x=\"0\" dy=\"\0" as *const u8 as *const libc::c_char,
        );
        gvprintdouble(job, -p.y);
        gvputs(job, b"\">\0" as *const u8 as *const libc::c_char);
    }
    let xml_flags: xml_flags_t = {
        let mut init = xml_flags_t {
            raw_dash_nbsp_utf8: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_raw(1 as libc::c_int as libc::c_uint);
        init.set_dash(1 as libc::c_int as libc::c_uint);
        init.set_nbsp(1 as libc::c_int as libc::c_uint);
        init.set_utf8(0);
        init
    };
    xml_escape(
        (*span).str_0,
        xml_flags,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int>,
            Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
        >(Some(
            gvputs as unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int,
        )),
        job as *mut libc::c_void,
    );
    if (*obj).labeledgealigned() != 0 {
        gvputs(
            job,
            b"</tspan></textPath>\0" as *const u8 as *const libc::c_char,
        );
    }
    gvputs(job, b"</text>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_gradstyle(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut G: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    static mut gradId: libc::c_int = 0;
    let fresh0 = gradId;
    gradId = gradId + 1;
    let mut id: libc::c_int = fresh0;
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut angle: libc::c_double = (*obj).gradient_angle as libc::c_double
        * 3.14159265358979323846f64
        / 180 as libc::c_int as libc::c_double;
    G[1 as libc::c_int as usize].y = 0.0f64;
    G[1 as libc::c_int as usize].x = G[1 as libc::c_int as usize].y;
    G[0 as libc::c_int as usize].y = G[1 as libc::c_int as usize].x;
    G[0 as libc::c_int as usize].x = G[0 as libc::c_int as usize].y;
    get_gradient_points(A, G.as_mut_ptr(), n, angle, 0 as libc::c_int);
    gvprintf(
        job,
        b"<defs>\n<linearGradient id=\"l_%d\" gradientUnits=\"userSpaceOnUse\" \0" as *const u8
            as *const libc::c_char,
        id,
    );
    gvputs(job, b"x1=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(job, G[0 as libc::c_int as usize].x);
    gvputs(job, b"\" y1=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(job, G[0 as libc::c_int as usize].y);
    gvputs(job, b"\" x2=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(job, G[1 as libc::c_int as usize].x);
    gvputs(job, b"\" y2=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(job, G[1 as libc::c_int as usize].y);
    gvputs(job, b"\" >\n\0" as *const u8 as *const libc::c_char);
    if (*obj).gradient_frac > 0 as libc::c_int as libc::c_float {
        gvprintf(
            job,
            b"<stop offset=\"%.03f\" style=\"stop-color:\0" as *const u8 as *const libc::c_char,
            (*obj).gradient_frac as libc::c_double - 0.001f64,
        );
    } else {
        gvputs(
            job,
            b"<stop offset=\"0\" style=\"stop-color:\0" as *const u8 as *const libc::c_char,
        );
    }
    svg_print_color(job, (*obj).fillcolor);
    gvputs(job, b";stop-opacity:\0" as *const u8 as *const libc::c_char);
    if (*obj).fillcolor.type_0 as libc::c_uint == RGBA_BYTE as libc::c_int as libc::c_uint
        && (*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int
        && ((*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int) < 255 as libc::c_int
    {
        gvprintf(
            job,
            b"%f\0" as *const u8 as *const libc::c_char,
            (*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_float as libc::c_double
                / 255.0f64,
        );
    } else {
        gvputs(job, b"1.\0" as *const u8 as *const libc::c_char);
    }
    gvputs(job, b";\"/>\n\0" as *const u8 as *const libc::c_char);
    if (*obj).gradient_frac > 0 as libc::c_int as libc::c_float {
        gvprintf(
            job,
            b"<stop offset=\"%.03f\" style=\"stop-color:\0" as *const u8 as *const libc::c_char,
            (*obj).gradient_frac as libc::c_double,
        );
    } else {
        gvputs(
            job,
            b"<stop offset=\"1\" style=\"stop-color:\0" as *const u8 as *const libc::c_char,
        );
    }
    svg_print_color(job, (*obj).stopcolor);
    gvputs(job, b";stop-opacity:\0" as *const u8 as *const libc::c_char);
    if (*obj).stopcolor.type_0 as libc::c_uint == RGBA_BYTE as libc::c_int as libc::c_uint
        && (*obj).stopcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int
        && ((*obj).stopcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int) < 255 as libc::c_int
    {
        gvprintf(
            job,
            b"%f\0" as *const u8 as *const libc::c_char,
            (*obj).stopcolor.u.rgba[3 as libc::c_int as usize] as libc::c_float as libc::c_double
                / 255.0f64,
        );
    } else {
        gvputs(job, b"1.\0" as *const u8 as *const libc::c_char);
    }
    gvputs(
        job,
        b";\"/>\n</linearGradient>\n</defs>\n\0" as *const u8 as *const libc::c_char,
    );
    return id;
}
unsafe extern "C" fn svg_rgradstyle(mut job: *mut GVJ_t) -> libc::c_int {
    let mut ifx: libc::c_double = 0.;
    let mut ify: libc::c_double = 0.;
    static mut rgradId: libc::c_int = 0;
    let fresh1 = rgradId;
    rgradId = rgradId + 1;
    let mut id: libc::c_int = fresh1;
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut angle: libc::c_double = (*obj).gradient_angle as libc::c_double
        * 3.14159265358979323846f64
        / 180 as libc::c_int as libc::c_double;
    if angle == 0.0f64 {
        ify = 50 as libc::c_int as libc::c_double;
        ifx = ify;
    } else {
        ifx = round(
            50 as libc::c_int as libc::c_double * (1 as libc::c_int as libc::c_double + cos(angle)),
        );
        ify = round(
            50 as libc::c_int as libc::c_double * (1 as libc::c_int as libc::c_double - sin(angle)),
        );
    }
    gvprintf(
        job,
        b"<defs>\n<radialGradient id=\"r_%d\" cx=\"50%%\" cy=\"50%%\" r=\"75%%\" fx=\"%.0f%%\" fy=\"%.0f%%\">\n\0"
            as *const u8 as *const libc::c_char,
        id,
        ifx,
        ify,
    );
    gvputs(
        job,
        b"<stop offset=\"0\" style=\"stop-color:\0" as *const u8 as *const libc::c_char,
    );
    svg_print_color(job, (*obj).fillcolor);
    gvputs(job, b";stop-opacity:\0" as *const u8 as *const libc::c_char);
    if (*obj).fillcolor.type_0 as libc::c_uint == RGBA_BYTE as libc::c_int as libc::c_uint
        && (*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int
        && ((*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int) < 255 as libc::c_int
    {
        gvprintf(
            job,
            b"%f\0" as *const u8 as *const libc::c_char,
            (*obj).fillcolor.u.rgba[3 as libc::c_int as usize] as libc::c_float as libc::c_double
                / 255.0f64,
        );
    } else {
        gvputs(job, b"1.\0" as *const u8 as *const libc::c_char);
    }
    gvputs(
        job,
        b";\"/>\n<stop offset=\"1\" style=\"stop-color:\0" as *const u8 as *const libc::c_char,
    );
    svg_print_color(job, (*obj).stopcolor);
    gvputs(job, b";stop-opacity:\0" as *const u8 as *const libc::c_char);
    if (*obj).stopcolor.type_0 as libc::c_uint == RGBA_BYTE as libc::c_int as libc::c_uint
        && (*obj).stopcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int
        && ((*obj).stopcolor.u.rgba[3 as libc::c_int as usize] as libc::c_int) < 255 as libc::c_int
    {
        gvprintf(
            job,
            b"%f\0" as *const u8 as *const libc::c_char,
            (*obj).stopcolor.u.rgba[3 as libc::c_int as usize] as libc::c_float as libc::c_double
                / 255.0f64,
        );
    } else {
        gvputs(job, b"1.\0" as *const u8 as *const libc::c_char);
    }
    gvputs(
        job,
        b";\"/>\n</radialGradient>\n</defs>\n\0" as *const u8 as *const libc::c_char,
    );
    return id;
}
unsafe extern "C" fn svg_ellipse(mut job: *mut GVJ_t, mut A: *mut pointf, mut filled: libc::c_int) {
    let mut gid: libc::c_int = 0 as libc::c_int;
    if filled == 2 as libc::c_int {
        gid = svg_gradstyle(job, A, 2 as libc::c_int);
    } else if filled == 3 as libc::c_int {
        gid = svg_rgradstyle(job);
    }
    gvputs(job, b"<ellipse\0" as *const u8 as *const libc::c_char);
    svg_grstyle(job, filled, gid);
    gvputs(job, b" cx=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(job, (*A.offset(0 as libc::c_int as isize)).x);
    gvputs(job, b"\" cy=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(job, -(*A.offset(0 as libc::c_int as isize)).y);
    gvputs(job, b"\" rx=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(
        job,
        (*A.offset(1 as libc::c_int as isize)).x - (*A.offset(0 as libc::c_int as isize)).x,
    );
    gvputs(job, b"\" ry=\"\0" as *const u8 as *const libc::c_char);
    gvprintdouble(
        job,
        (*A.offset(1 as libc::c_int as isize)).y - (*A.offset(0 as libc::c_int as isize)).y,
    );
    gvputs(job, b"\"/>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut gid: libc::c_int = 0 as libc::c_int;
    let mut obj: *mut obj_state_t = (*job).obj;
    if filled == 2 as libc::c_int {
        gid = svg_gradstyle(job, A, n);
    } else if filled == 3 as libc::c_int {
        gid = svg_rgradstyle(job);
    }
    gvputs(job, b"<path\0" as *const u8 as *const libc::c_char);
    if (*obj).labeledgealigned() != 0 {
        gvputs(job, b" id=\"\0" as *const u8 as *const libc::c_char);
        gvputs_xml(job, (*obj).id);
        gvputs(job, b"_p\" \0" as *const u8 as *const libc::c_char);
    }
    svg_grstyle(job, filled, gid);
    gvputs(job, b" d=\"\0" as *const u8 as *const libc::c_char);
    svg_bzptarray(job, A, n);
    gvputs(job, b"\"/>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut gid: libc::c_int = 0 as libc::c_int;
    if filled == 2 as libc::c_int {
        gid = svg_gradstyle(job, A, n);
    } else if filled == 3 as libc::c_int {
        gid = svg_rgradstyle(job);
    }
    gvputs(job, b"<polygon\0" as *const u8 as *const libc::c_char);
    svg_grstyle(job, filled, gid);
    gvputs(job, b" points=\"\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n {
        gvprintdouble(job, (*A.offset(i as isize)).x);
        gvputc(job, ',' as i32);
        gvprintdouble(job, -(*A.offset(i as isize)).y);
        gvputc(job, ' ' as i32);
        i += 1;
    }
    gvprintdouble(job, (*A.offset(0 as libc::c_int as isize)).x);
    gvputc(job, ',' as i32);
    gvprintdouble(job, -(*A.offset(0 as libc::c_int as isize)).y);
    gvputs(job, b"\"/>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn svg_polyline(mut job: *mut GVJ_t, mut A: *mut pointf, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    gvputs(job, b"<polyline\0" as *const u8 as *const libc::c_char);
    svg_grstyle(job, 0 as libc::c_int, 0 as libc::c_int);
    gvputs(job, b" points=\"\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n {
        gvprintdouble(job, (*A.offset(i as isize)).x);
        gvputc(job, ',' as i32);
        gvprintdouble(job, -(*A.offset(i as isize)).y);
        gvputc(job, ' ' as i32);
        i += 1;
    }
    gvputs(job, b"\"/>\n\0" as *const u8 as *const libc::c_char);
}
static mut svg_knowncolors: [*mut libc::c_char; 148] = [
    b"aliceblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"antiquewhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aqua\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aquamarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"azure\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beige\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bisque\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blanchedalmond\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"blueviolet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"brown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"burlywood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cadetblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"chartreuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"chocolate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"coral\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cornflowerblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cornsilk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"crimson\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkcyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkgoldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkkhaki\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkmagenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkolivegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkorange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkorchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darksalmon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkseagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkslateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkslategray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkslategrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"darkviolet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"deeppink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"deepskyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dimgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dimgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dodgerblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"firebrick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"floralwhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"forestgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fuchsia\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gainsboro\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ghostwhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"goldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"green\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"greenyellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"grey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"honeydew\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hotpink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"indianred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"indigo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ivory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"khaki\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lavender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lavenderblush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lawngreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lemonchiffon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightcoral\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightcyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightgoldenrodyellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightpink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightsalmon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightseagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightskyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightslategray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightslategrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightsteelblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lightyellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"limegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"linen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"magenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"maroon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumaquamarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumorchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumpurple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumseagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumslateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumspringgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mediumvioletred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"midnightblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mintcream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mistyrose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"moccasin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"navajowhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"navy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"oldlace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"olive\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"olivedrab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"orange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"orangered\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"orchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"palegoldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"palegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"paleturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"palevioletred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"papayawhip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"peachpuff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"peru\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"plum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"powderblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"purple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rosybrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"royalblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"saddlebrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"salmon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sandybrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"seagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"seashell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sienna\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"silver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"skyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"slateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"slategray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"slategrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"snow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"springgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"steelblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"teal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"thistle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tomato\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"turquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"violet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wheat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"whitesmoke\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yellowgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut svg_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: Some(svg_begin_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_job: None,
            begin_graph: Some(svg_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_graph: Some(svg_end_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_layer: Some(
                svg_begin_layer
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut libc::c_char,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            end_layer: Some(svg_end_layer as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_page: Some(svg_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_page: Some(svg_end_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_cluster: Some(svg_begin_cluster as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_cluster: Some(svg_end_cluster as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: Some(svg_begin_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_node: Some(svg_end_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_edge: Some(svg_begin_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_edge: Some(svg_end_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_anchor: Some(
                svg_begin_anchor
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> (),
            ),
            end_anchor: Some(svg_end_anchor as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_label: None,
            end_label: None,
            textspan: Some(
                svg_textspan as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: None,
            ellipse: Some(
                svg_ellipse as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                svg_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                svg_bezier
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
                svg_polyline as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: Some(svg_comment as unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> ()),
            library_shape: None,
        };
        init
    }
};
#[no_mangle]
pub static mut render_features_svg: gvrender_features_t = gvrender_features_t {
    flags: 0,
    default_pad: 0.,
    knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    sz_knowncolors: 0,
    color_type: HSVA_DOUBLE,
};
#[no_mangle]
pub static mut device_features_svg: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 8 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int,
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
                x: 72.0f64,
                y: 72.0f64,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut device_features_svgz: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
            | (1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 10 as libc::c_int,
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
                x: 72.0f64,
                y: 72.0f64,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_svg_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVG as libc::c_int,
                type_0: b"svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &svg_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_svg as *const gvrender_features_t
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
pub static mut gvdevice_svg_types: [gvplugin_installed_t; 3] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVG as libc::c_int,
                type_0: b"svg:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_svg as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVGZ as libc::c_int,
                type_0: b"svgz:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_svgz as *const gvdevice_features_t
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
    render_features_svg = {
        let mut init = gvrender_features_t {
            flags: (1 as libc::c_int) << 12 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int
                | (1 as libc::c_int) << 16 as libc::c_int
                | (1 as libc::c_int) << 23 as libc::c_int
                | (1 as libc::c_int) << 22 as libc::c_int,
            default_pad: 4.0f64,
            knowncolors: svg_knowncolors.as_mut_ptr(),
            sz_knowncolors: (::std::mem::size_of::<[*mut libc::c_char; 148]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                as libc::c_int,
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
