#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type GVC_s;
    pub type htmllabel_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn gvprintdouble(job: *mut GVJ_t, num: libc::c_double);
    fn gvprintpointf(job: *mut GVJ_t, p: pointf);
    fn gvprintpointflist(job: *mut GVJ_t, p: *mut pointf, n: libc::c_int);
    fn ps_string(s: *mut libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn epsf_define(job: *mut GVJ_t);
    fn cat_libfile(
        job: *mut GVJ_t,
        arglib: *mut *const libc::c_char,
        stdlib: *mut *const libc::c_char,
    );
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
    pub cleanup: Option::<unsafe extern "C" fn(*mut graph_t) -> ()>,
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const FORMAT_EPS: C2RustUnnamed_6 = 2;
pub const FORMAT_PS2: C2RustUnnamed_6 = 1;
pub const FORMAT_PS: C2RustUnnamed_6 = 0;
static mut ps_txt: [*const libc::c_char; 2] = [
    b"%%BeginProlog\n/DotDict 200 dict def\nDotDict begin\n\n/setupLatin1 {\nmark\n/EncodingVector 256 array def\n EncodingVector 0\n\nISOLatin1Encoding 0 255 getinterval putinterval\nEncodingVector 45 /hyphen put\n\n% Set up ISO Latin 1 character encoding\n/starnetISO {\n        dup dup findfont dup length dict begin\n        { 1 index /FID ne { def }{ pop pop } ifelse\n        } forall\n        /Encoding EncodingVector def\n        currentdict end definefont\n} def\n/Times-Roman starnetISO def\n/Times-Italic starnetISO def\n/Times-Bold starnetISO def\n/Times-BoldItalic starnetISO def\n/Helvetica starnetISO def\n/Helvetica-Oblique starnetISO def\n/Helvetica-Bold starnetISO def\n/Helvetica-BoldOblique starnetISO def\n/Courier starnetISO def\n/Courier-Oblique starnetISO def\n/Courier-Bold starnetISO def\n/Courier-BoldOblique starnetISO def\ncleartomark\n} bind def\n\n%%BeginResource: procset graphviz 0 0\n/coord-font-family /Times-Roman def\n/default-font-family /Times-Roman def\n/coordfont coord-font-family findfont 8 scalefont def\n\n/InvScaleFactor 1.0 def\n/set_scale {\n       dup 1 exch div /InvScaleFactor exch def\n       scale\n} bind def\n\n% styles\n/solid { [] 0 setdash } bind def\n/dashed { [9 InvScaleFactor mul dup ] 0 setdash } bind def\n/dotted { [1 InvScaleFactor mul 6 InvScaleFactor mul] 0 setdash } bind def\n/invis {/fill {newpath} def /stroke {newpath} def /show {pop newpath} def} bind def\n/bold { 2 setlinewidth } bind def\n/filled { } bind def\n/unfilled { } bind def\n/rounded { } bind def\n/diagonals { } bind def\n/tapered { } bind def\n\n% hooks for setting color \n/nodecolor { sethsbcolor } bind def\n/edgecolor { sethsbcolor } bind def\n/graphcolor { sethsbcolor } bind def\n/nopcolor {pop pop pop} bind def\n\n/beginpage {\t% i j npages\n\t/npages exch def\n\t/j exch def\n\t/i exch def\n\t/str 10 string def\n\tnpages 1 gt {\n\t\tgsave\n\t\t\tcoordfont setfont\n\t\t\t0 0 moveto\n\t\t\t(\\() show i str cvs show (,) show j str cvs show (\\)) show\n\t\tgrestore\n\t} if\n} bind def\n\n/set_font {\n\tfindfont exch\n\tscalefont setfont\n} def\n\n% draw text fitted to its expected width\n/alignedtext {\t\t\t% width text\n\t/text exch def\n\t/width exch def\n\tgsave\n\t\twidth 0 gt {\n\t\t\t[] 0 setdash\n\t\t\ttext stringwidth pop width exch sub text length div 0 text ashow\n\t\t} if\n\tgrestore\n} def\n\n/boxprim {\t\t\t\t% xcorner ycorner xsize ysize\n\t\t4 2 roll\n\t\tmoveto\n\t\t2 copy\n\t\texch 0 rlineto\n\t\t0 exch rlineto\n\t\tpop neg 0 rlineto\n\t\tclosepath\n} bind def\n\n/ellipse_path {\n\t/ry exch def\n\t/rx exch def\n\t/y exch def\n\t/x exch def\n\tmatrix currentmatrix\n\tnewpath\n\tx y translate\n\trx ry scale\n\t0 0 1 0 360 arc\n\tsetmatrix\n} bind def\n\n/endpage { showpage } bind def\n/showpage { } def\n\n/layercolorseq\n\t[\t% layer color sequence - darkest to lightest\n\t\t[0 0 0]\n\t\t[.2 .8 .8]\n\t\t[.4 .8 .8]\n\t\t[.6 .8 .8]\n\t\t[.8 .8 .8]\n\t]\ndef\n\n/layerlen layercolorseq length def\n\n/setlayer {/maxlayer exch def /curlayer exch def\n\tlayercolorseq curlayer 1 sub layerlen mod get\n\taload pop sethsbcolor\n\t/nodecolor {nopcolor} def\n\t/edgecolor {nopcolor} def\n\t/graphcolor {nopcolor} def\n} bind def\n\n/onlayer { curlayer ne {invis} if } def\n\n/onlayers {\n\t/myupper exch def\n\t/mylower exch def\n\tcurlayer mylower lt\n\tcurlayer myupper gt\n\tor\n\t{invis} if\n} def\n\n/curlayer 0 def\n\n%%EndResource\n%%EndProlog\n%%BeginSetup\n14 default-font-family set_font\n% /arrowlength 10 def\n% /arrowwidth 5 def\n\n% make sure pdfmark is harmless for PS-interpreters other than Distiller\n/pdfmark where {pop} {userdict /pdfmark /cleartomark load put} ifelse\n% make '<<' and '>>' safe on PS Level 1 devices\n/languagelevel where {pop languagelevel}{1} ifelse\n2 lt {\n    userdict (<<) cvn ([) cvn load put\n    userdict (>>) cvn ([) cvn load put\n} if\n\n%%EndSetup\0"
        as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut isLatin1: libc::c_int = 0;
static mut setupLatin1: libc::c_char = 0;
unsafe extern "C" fn psgen_begin_job(mut job: *mut GVJ_t) {
    gvputs(job, b"%!PS-Adobe-3.0\0" as *const u8 as *const libc::c_char);
    if (*job).render.id == FORMAT_EPS as libc::c_int {
        gvputs(job, b" EPSF-3.0\n\0" as *const u8 as *const libc::c_char);
    } else {
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    }
    gvprintf(
        job,
        b"%%%%Creator: %s version %s (%s)\n\0" as *const u8 as *const libc::c_char,
        *((*(*job).common).info).offset(0 as libc::c_int as isize),
        *((*(*job).common).info).offset(1 as libc::c_int as isize),
        *((*(*job).common).info).offset(2 as libc::c_int as isize),
    );
}
unsafe extern "C" fn psgen_end_job(mut job: *mut GVJ_t) {
    gvputs(job, b"%%Trailer\n\0" as *const u8 as *const libc::c_char);
    if (*job).render.id != FORMAT_EPS as libc::c_int {
        gvprintf(
            job,
            b"%%%%Pages: %d\n\0" as *const u8 as *const libc::c_char,
            (*(*job).common).viewNum,
        );
    }
    if ((*(*job).common).show_boxes).is_null() {
        if (*job).render.id != FORMAT_EPS as libc::c_int {
            gvprintf(
                job,
                b"%%%%BoundingBox: %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
                (*job).boundingBox.LL.x,
                (*job).boundingBox.LL.y,
                (*job).boundingBox.UR.x,
                (*job).boundingBox.UR.y,
            );
        }
    }
    gvputs(job, b"end\nrestore\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"%%EOF\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_begin_graph(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    setupLatin1 = 0 as libc::c_int as libc::c_char;
    if (*(*job).common).viewNum == 0 as libc::c_int {
        gvprintf(
            job,
            b"%%%%Title: %s\n\0" as *const u8 as *const libc::c_char,
            agnameof((*obj).u.g as *mut libc::c_void),
        );
        if (*job).render.id != FORMAT_EPS as libc::c_int {
            gvputs(job, b"%%Pages: (atend)\n\0" as *const u8 as *const libc::c_char);
        } else {
            gvputs(job, b"%%Pages: 1\n\0" as *const u8 as *const libc::c_char);
        }
        if ((*(*job).common).show_boxes).is_null() {
            if (*job).render.id != FORMAT_EPS as libc::c_int {
                gvputs(
                    job,
                    b"%%BoundingBox: (atend)\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                gvprintf(
                    job,
                    b"%%%%BoundingBox: %d %d %d %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*job).pageBoundingBox.LL.x,
                    (*job).pageBoundingBox.LL.y,
                    (*job).pageBoundingBox.UR.x,
                    (*job).pageBoundingBox.UR.y,
                );
            }
        }
        gvputs(job, b"%%EndComments\nsave\n\0" as *const u8 as *const libc::c_char);
        cat_libfile(job, (*(*job).common).lib, ps_txt.as_mut_ptr());
        epsf_define(job);
        if !((*(*job).common).show_boxes).is_null() {
            let mut args: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
            args[0 as libc::c_int
                as usize] = *((*(*job).common).show_boxes)
                .offset(0 as libc::c_int as isize);
            args[1 as libc::c_int as usize] = 0 as *const libc::c_char;
            cat_libfile(job, 0 as *mut *const libc::c_char, args.as_mut_ptr());
        }
    }
    isLatin1 = if (*((*((*obj).u.g as *mut Agobj_t)).data as *mut Agraphinfo_t)).charset
        as libc::c_int == 1 as libc::c_int
    {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    if setupLatin1 == 0 {
        gvputs(job, b"setupLatin1\n\0" as *const u8 as *const libc::c_char);
        setupLatin1 = (0 as libc::c_int == 0) as libc::c_int as libc::c_char;
    }
    if !((*obj).url).is_null() {
        gvprintf(
            job,
            b"[ {Catalog} << /URI << /Base %s >> >>\n/PUT pdfmark\n\0" as *const u8
                as *const libc::c_char,
            ps_string((*obj).url, isLatin1),
        );
    }
}
unsafe extern "C" fn psgen_begin_layer(
    mut job: *mut GVJ_t,
    mut layername: *mut libc::c_char,
    mut layerNum: libc::c_int,
    mut numLayers: libc::c_int,
) {
    gvprintf(
        job,
        b"%d %d setlayer\n\0" as *const u8 as *const libc::c_char,
        layerNum,
        numLayers,
    );
}
unsafe extern "C" fn psgen_begin_page(mut job: *mut GVJ_t) {
    let mut pbr: box_0 = (*job).pageBoundingBox;
    gvprintf(
        job,
        b"%%%%Page: %d %d\n\0" as *const u8 as *const libc::c_char,
        (*(*job).common).viewNum + 1 as libc::c_int,
        (*(*job).common).viewNum + 1 as libc::c_int,
    );
    if ((*(*job).common).show_boxes).is_null() {
        gvprintf(
            job,
            b"%%%%PageBoundingBox: %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
            pbr.LL.x,
            pbr.LL.y,
            pbr.UR.x,
            pbr.UR.y,
        );
    }
    gvprintf(
        job,
        b"%%%%PageOrientation: %s\n\0" as *const u8 as *const libc::c_char,
        if (*job).rotation != 0 {
            b"Landscape\0" as *const u8 as *const libc::c_char
        } else {
            b"Portrait\0" as *const u8 as *const libc::c_char
        },
    );
    if (*job).render.id == FORMAT_PS2 as libc::c_int {
        gvprintf(
            job,
            b"<< /PageSize [%d %d] >> setpagedevice\n\0" as *const u8
                as *const libc::c_char,
            pbr.UR.x,
            pbr.UR.y,
        );
    }
    gvprintf(
        job,
        b"%d %d %d beginpage\n\0" as *const u8 as *const libc::c_char,
        (*job).pagesArrayElem.x,
        (*job).pagesArrayElem.y,
        (*job).numPages,
    );
    if ((*(*job).common).show_boxes).is_null() {
        gvprintf(
            job,
            b"gsave\n%d %d %d %d boxprim clip newpath\n\0" as *const u8
                as *const libc::c_char,
            pbr.LL.x,
            pbr.LL.y,
            pbr.UR.x - pbr.LL.x,
            pbr.UR.y - pbr.LL.y,
        );
    }
    gvprintf(
        job,
        b"%g %g set_scale %d rotate %g %g translate\n\0" as *const u8
            as *const libc::c_char,
        (*job).scale.x,
        (*job).scale.y,
        (*job).rotation,
        (*job).translation.x,
        (*job).translation.y,
    );
    if (*job).render.id == FORMAT_PS2 as libc::c_int {
        if pbr.UR.x >= 14400 as libc::c_int || pbr.UR.y >= 14400 as libc::c_int {
            ((*(*job).common).errorfn)
                .expect(
                    "non-null function pointer",
                )(
                b"canvas size (%d,%d) exceeds PDF limit (%d)\n\t(suggest setting a bounding box size, see dot(1))\n\0"
                    as *const u8 as *const libc::c_char,
                pbr.UR.x,
                pbr.UR.y,
                14400 as libc::c_int,
            );
        }
        gvprintf(
            job,
            b"[ /CropBox [%d %d %d %d] /PAGES pdfmark\n\0" as *const u8
                as *const libc::c_char,
            pbr.LL.x,
            pbr.LL.y,
            pbr.UR.x,
            pbr.UR.y,
        );
    }
}
unsafe extern "C" fn psgen_end_page(mut job: *mut GVJ_t) {
    if !((*(*job).common).show_boxes).is_null() {
        gvputs(job, b"0 0 0 edgecolor\n\0" as *const u8 as *const libc::c_char);
        cat_libfile(
            job,
            0 as *mut *const libc::c_char,
            ((*(*job).common).show_boxes).offset(1 as libc::c_int as isize),
        );
    }
    gvputs(job, b"endpage\nshowpage\ngrestore\n\0" as *const u8 as *const libc::c_char);
    gvputs(job, b"%%PageTrailer\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"%%%%EndPage: %d\n\0" as *const u8 as *const libc::c_char,
        (*(*job).common).viewNum,
    );
}
unsafe extern "C" fn psgen_begin_cluster(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    gvprintf(
        job,
        b"%% %s\n\0" as *const u8 as *const libc::c_char,
        agnameof((*obj).u.g as *mut libc::c_void),
    );
    gvputs(job, b"gsave\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_end_cluster(mut job: *mut GVJ_t) {
    gvputs(job, b"grestore\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_begin_node(mut job: *mut GVJ_t) {
    gvputs(job, b"gsave\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_end_node(mut job: *mut GVJ_t) {
    gvputs(job, b"grestore\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_begin_edge(mut job: *mut GVJ_t) {
    gvputs(job, b"gsave\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_end_edge(mut job: *mut GVJ_t) {
    gvputs(job, b"grestore\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_begin_anchor(
    mut job: *mut GVJ_t,
    mut url: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    if !url.is_null() && !((*obj).url_map_p).is_null() {
        gvputs(job, b"[ /Rect [ \0" as *const u8 as *const libc::c_char);
        gvprintpointflist(job, (*obj).url_map_p, 2 as libc::c_int);
        gvputs(job, b" ]\n\0" as *const u8 as *const libc::c_char);
        gvprintf(
            job,
            b"  /Border [ 0 0 0 ]\n  /Action << /Subtype /URI /URI %s >>\n  /Subtype /Link\n/ANN pdfmark\n\0"
                as *const u8 as *const libc::c_char,
            ps_string(url, isLatin1),
        );
    }
}
unsafe extern "C" fn ps_set_pen_style(mut job: *mut GVJ_t) {
    let mut penwidth: libc::c_double = (*(*job).obj).penwidth;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut *mut libc::c_char = (*(*job).obj).rawstyle;
    gvprintdouble(job, penwidth);
    gvputs(job, b" setlinewidth\n\0" as *const u8 as *const libc::c_char);
    while !s.is_null()
        && {
            let fresh0 = s;
            s = s.offset(1);
            line = *fresh0;
            p = line;
            !p.is_null()
        }
    {
        if strcmp(line, b"setlinewidth\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            continue;
        }
        while *p != 0 {
            p = p.offset(1);
        }
        p = p.offset(1);
        while *p != 0 {
            gvprintf(job, b"%s \0" as *const u8 as *const libc::c_char, p);
            while *p != 0 {
                p = p.offset(1);
            }
            p = p.offset(1);
        }
        if strcmp(line, b"invis\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*(*job).obj).penwidth = 0 as libc::c_int as libc::c_double;
        }
        gvprintf(job, b"%s\n\0" as *const u8 as *const libc::c_char, line);
    }
}
unsafe extern "C" fn ps_set_color(mut job: *mut GVJ_t, mut color: *mut gvcolor_t) {
    let mut objtype: *mut libc::c_char = 0 as *mut libc::c_char;
    if !color.is_null() {
        match (*(*job).obj).type_0 as libc::c_uint {
            0 | 1 => {
                objtype = b"graph\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            2 => {
                objtype = b"node\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            3 => {
                objtype = b"edge\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            _ => {
                objtype = b"sethsb\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        }
        gvprintf(
            job,
            b"%.5g %.5g %.5g %scolor\n\0" as *const u8 as *const libc::c_char,
            (*color).u.HSVA[0 as libc::c_int as usize],
            (*color).u.HSVA[1 as libc::c_int as usize],
            (*color).u.HSVA[2 as libc::c_int as usize],
            objtype,
        );
    }
}
unsafe extern "C" fn psgen_textspan(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut span: *mut textspan_t,
) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*job).obj).pencolor.u.HSVA[3 as libc::c_int as usize] < 0.5f64 {
        return;
    }
    ps_set_color(job, &mut (*(*job).obj).pencolor);
    gvprintdouble(job, (*(*span).font).size);
    gvprintf(
        job,
        b" /%s set_font\n\0" as *const u8 as *const libc::c_char,
        (*(*span).font).name,
    );
    str = ps_string((*span).str_0, isLatin1);
    match (*span).just as libc::c_int {
        114 => {
            p.x -= (*span).size.x;
        }
        108 => {
            p.x -= 0.0f64;
        }
        110 | _ => {
            p.x -= (*span).size.x / 2.0f64;
        }
    }
    p.y += (*span).yoffset_centerline;
    gvprintpointf(job, p);
    gvputs(job, b" moveto \0" as *const u8 as *const libc::c_char);
    gvprintdouble(job, (*span).size.x);
    gvprintf(job, b" %s alignedtext\n\0" as *const u8 as *const libc::c_char, str);
}
unsafe extern "C" fn psgen_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut AA: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    AA[0 as libc::c_int as usize] = *A.offset(0 as libc::c_int as isize);
    AA[1 as libc::c_int as usize]
        .x = (*A.offset(1 as libc::c_int as isize)).x
        - (*A.offset(0 as libc::c_int as isize)).x;
    AA[1 as libc::c_int as usize]
        .y = (*A.offset(1 as libc::c_int as isize)).y
        - (*A.offset(0 as libc::c_int as isize)).y;
    if filled != 0 && (*(*job).obj).fillcolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64
    {
        ps_set_color(job, &mut (*(*job).obj).fillcolor);
        gvprintpointflist(job, AA.as_mut_ptr(), 2 as libc::c_int);
        gvputs(job, b" ellipse_path fill\n\0" as *const u8 as *const libc::c_char);
    }
    if (*(*job).obj).pencolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64 {
        ps_set_pen_style(job);
        ps_set_color(job, &mut (*(*job).obj).pencolor);
        gvprintpointflist(job, AA.as_mut_ptr(), 2 as libc::c_int);
        gvputs(job, b" ellipse_path stroke\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn psgen_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    if filled != 0 && (*(*job).obj).fillcolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64
    {
        ps_set_color(job, &mut (*(*job).obj).fillcolor);
        gvputs(job, b"newpath \0" as *const u8 as *const libc::c_char);
        gvprintpointf(job, *A.offset(0 as libc::c_int as isize));
        gvputs(job, b" moveto\n\0" as *const u8 as *const libc::c_char);
        j = 1 as libc::c_int;
        while j < n {
            gvprintpointflist(job, &mut *A.offset(j as isize), 3 as libc::c_int);
            gvputs(job, b" curveto\n\0" as *const u8 as *const libc::c_char);
            j += 3 as libc::c_int;
        }
        gvputs(job, b"closepath fill\n\0" as *const u8 as *const libc::c_char);
    }
    if (*(*job).obj).pencolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64 {
        ps_set_pen_style(job);
        ps_set_color(job, &mut (*(*job).obj).pencolor);
        gvputs(job, b"newpath \0" as *const u8 as *const libc::c_char);
        gvprintpointf(job, *A.offset(0 as libc::c_int as isize));
        gvputs(job, b" moveto\n\0" as *const u8 as *const libc::c_char);
        j = 1 as libc::c_int;
        while j < n {
            gvprintpointflist(job, &mut *A.offset(j as isize), 3 as libc::c_int);
            gvputs(job, b" curveto\n\0" as *const u8 as *const libc::c_char);
            j += 3 as libc::c_int;
        }
        gvputs(job, b"stroke\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn psgen_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    if filled != 0 && (*(*job).obj).fillcolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64
    {
        ps_set_color(job, &mut (*(*job).obj).fillcolor);
        gvputs(job, b"newpath \0" as *const u8 as *const libc::c_char);
        gvprintpointf(job, *A.offset(0 as libc::c_int as isize));
        gvputs(job, b" moveto\n\0" as *const u8 as *const libc::c_char);
        j = 1 as libc::c_int;
        while j < n {
            gvprintpointf(job, *A.offset(j as isize));
            gvputs(job, b" lineto\n\0" as *const u8 as *const libc::c_char);
            j += 1;
        }
        gvputs(job, b"closepath fill\n\0" as *const u8 as *const libc::c_char);
    }
    if (*(*job).obj).pencolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64 {
        ps_set_pen_style(job);
        ps_set_color(job, &mut (*(*job).obj).pencolor);
        gvputs(job, b"newpath \0" as *const u8 as *const libc::c_char);
        gvprintpointf(job, *A.offset(0 as libc::c_int as isize));
        gvputs(job, b" moveto\n\0" as *const u8 as *const libc::c_char);
        j = 1 as libc::c_int;
        while j < n {
            gvprintpointf(job, *A.offset(j as isize));
            gvputs(job, b" lineto\n\0" as *const u8 as *const libc::c_char);
            j += 1;
        }
        gvputs(job, b"closepath stroke\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn psgen_polyline(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    if (*(*job).obj).pencolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64 {
        ps_set_pen_style(job);
        ps_set_color(job, &mut (*(*job).obj).pencolor);
        gvputs(job, b"newpath \0" as *const u8 as *const libc::c_char);
        gvprintpointf(job, *A.offset(0 as libc::c_int as isize));
        gvputs(job, b" moveto\n\0" as *const u8 as *const libc::c_char);
        j = 1 as libc::c_int;
        while j < n {
            gvprintpointf(job, *A.offset(j as isize));
            gvputs(job, b" lineto\n\0" as *const u8 as *const libc::c_char);
            j += 1;
        }
        gvputs(job, b"stroke\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn psgen_comment(mut job: *mut GVJ_t, mut str: *mut libc::c_char) {
    gvputs(job, b"% \0" as *const u8 as *const libc::c_char);
    gvputs(job, str);
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn psgen_library_shape(
    mut job: *mut GVJ_t,
    mut name: *mut libc::c_char,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    if filled != 0 && (*(*job).obj).fillcolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64
    {
        ps_set_color(job, &mut (*(*job).obj).fillcolor);
        gvputs(job, b"[ \0" as *const u8 as *const libc::c_char);
        gvprintpointflist(job, A, n);
        gvputs(job, b" \0" as *const u8 as *const libc::c_char);
        gvprintpointf(job, *A.offset(0 as libc::c_int as isize));
        gvprintf(
            job,
            b" ]  %d true %s\n\0" as *const u8 as *const libc::c_char,
            n,
            name,
        );
    }
    if (*(*job).obj).pencolor.u.HSVA[3 as libc::c_int as usize] > 0.5f64 {
        ps_set_pen_style(job);
        ps_set_color(job, &mut (*(*job).obj).pencolor);
        gvputs(job, b"[ \0" as *const u8 as *const libc::c_char);
        gvprintpointflist(job, A, n);
        gvputs(job, b" \0" as *const u8 as *const libc::c_char);
        gvprintpointf(job, *A.offset(0 as libc::c_int as isize));
        gvprintf(
            job,
            b" ]  %d false %s\n\0" as *const u8 as *const libc::c_char,
            n,
            name,
        );
    }
}
static mut psgen_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: Some(psgen_begin_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_job: Some(psgen_end_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_graph: Some(
                psgen_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            end_graph: None,
            begin_layer: Some(
                psgen_begin_layer
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut libc::c_char,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            end_layer: None,
            begin_page: Some(psgen_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_page: Some(psgen_end_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_cluster: Some(
                psgen_begin_cluster as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            end_cluster: Some(
                psgen_end_cluster as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: Some(psgen_begin_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_node: Some(psgen_end_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_edge: Some(psgen_begin_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_edge: Some(psgen_end_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_anchor: Some(
                psgen_begin_anchor
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> (),
            ),
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                psgen_textspan
                    as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: None,
            ellipse: Some(
                psgen_ellipse
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                psgen_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                psgen_bezier
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
                psgen_polyline
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: Some(
                psgen_comment
                    as unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> (),
            ),
            library_shape: Some(
                psgen_library_shape
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut libc::c_char,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
        };
        init
    }
};
static mut render_features_ps: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 13 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 25 as libc::c_int
            | (1 as libc::c_int) << 17 as libc::c_int,
        default_pad: 4.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: HSVA_DOUBLE,
    };
    init
};
static mut device_features_ps: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 5 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 36.0f64, y: 36.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s {
                x: 612.0f64,
                y: 792.0f64,
            };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
static mut device_features_eps: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: 0 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 36.0f64, y: 36.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s {
                x: 612.0f64,
                y: 792.0f64,
            };
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
pub static mut gvrender_ps_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS as libc::c_int,
                type_0: b"ps\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &psgen_engine as *const gvrender_engine_t
                    as *mut gvrender_engine_t as *mut libc::c_void,
                features: &render_features_ps as *const gvrender_features_t
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
pub static mut gvdevice_ps_types: [gvplugin_installed_t; 4] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS as libc::c_int,
                type_0: b"ps:ps\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_ps as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS2 as libc::c_int,
                type_0: b"ps2:ps\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_ps as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_EPS as libc::c_int,
                type_0: b"eps:ps\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_eps as *const gvdevice_features_t
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
