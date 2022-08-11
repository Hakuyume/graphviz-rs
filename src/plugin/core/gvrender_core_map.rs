#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type GVC_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn xml_escape(
        s: *const libc::c_char,
        flags: xml_flags_t,
        cb: Option::<
            unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
        >,
        state: *mut libc::c_void,
    ) -> libc::c_int;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvputs_xml(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
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
pub const FORMAT_CMAPX: C2RustUnnamed_4 = 3;
pub const FORMAT_CMAP: C2RustUnnamed_4 = 2;
pub const FORMAT_ISMAP: C2RustUnnamed_4 = 1;
pub const FORMAT_IMAP: C2RustUnnamed_4 = 0;
unsafe extern "C" fn xml_url_puts(mut job: *mut GVJ_t, mut s: *const libc::c_char) {
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
        s,
        flags,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
        >(
            Some(
                gvputs
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
        job as *mut libc::c_void,
    );
}
unsafe extern "C" fn map_output_shape(
    mut job: *mut GVJ_t,
    mut map_shape: map_shape_t,
    mut AF: *mut pointf,
    mut nump: libc::c_int,
    mut url: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if AF.is_null() || nump == 0 {
        return;
    }
    if (*job).render.id == FORMAT_IMAP as libc::c_int && !url.is_null()
        && *url.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        match map_shape as libc::c_uint {
            0 => {
                let mut A: point = point { x: 0, y: 0 };
                let mut B: point = point { x: 0, y: 0 };
                A
                    .x = (if (*AF.offset(0 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                A
                    .y = (if (*AF.offset(0 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                B
                    .x = (if (*AF.offset(1 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                B
                    .y = (if (*AF.offset(1 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                gvprintf(
                    job,
                    b"rect %s %d,%d %d,%d\n\0" as *const u8 as *const libc::c_char,
                    url,
                    A.x,
                    B.y,
                    B.x,
                    A.y,
                );
            }
            1 => {
                let mut A_0: point = point { x: 0, y: 0 };
                let mut B_0: point = point { x: 0, y: 0 };
                A_0
                    .x = (if (*AF.offset(0 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                A_0
                    .y = (if (*AF.offset(0 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                B_0
                    .x = (if (*AF.offset(1 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                B_0
                    .y = (if (*AF.offset(1 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                gvprintf(
                    job,
                    b"circle %s %d,%d,%d\n\0" as *const u8 as *const libc::c_char,
                    url,
                    A_0.x,
                    A_0.y,
                    B_0.x - A_0.x,
                );
            }
            2 => {
                gvprintf(job, b"poly %s\0" as *const u8 as *const libc::c_char, url);
                i = 0 as libc::c_int;
                while i < nump {
                    let mut A_1: point = point { x: 0, y: 0 };
                    A_1
                        .x = (if (*AF.offset(i as isize)).x
                        >= 0 as libc::c_int as libc::c_double
                    {
                        ((*AF.offset(i as isize)).x + 0.5f64) as libc::c_int
                    } else {
                        ((*AF.offset(i as isize)).x - 0.5f64) as libc::c_int
                    });
                    A_1
                        .y = (if (*AF.offset(i as isize)).y
                        >= 0 as libc::c_int as libc::c_double
                    {
                        ((*AF.offset(i as isize)).y + 0.5f64) as libc::c_int
                    } else {
                        ((*AF.offset(i as isize)).y - 0.5f64) as libc::c_int
                    });
                    gvprintf(
                        job,
                        b" %d,%d\0" as *const u8 as *const libc::c_char,
                        A_1.x,
                        A_1.y,
                    );
                    i += 1;
                }
                gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"gvrender_core_map.c\0" as *const u8 as *const libc::c_char,
                    64 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 91],
                        &[libc::c_char; 91],
                    >(
                        b"void map_output_shape(GVJ_t *, map_shape_t, pointf *, int, char *, char *, char *, char *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    } else if (*job).render.id == FORMAT_ISMAP as libc::c_int && !url.is_null()
            && *url.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
        match map_shape as libc::c_uint {
            0 => {
                let mut A_2: point = point { x: 0, y: 0 };
                let mut B_1: point = point { x: 0, y: 0 };
                A_2
                    .x = (if (*AF.offset(0 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                A_2
                    .y = (if (*AF.offset(0 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                B_1
                    .x = (if (*AF.offset(1 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                B_1
                    .y = (if (*AF.offset(1 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                gvprintf(
                    job,
                    b"rectangle (%d,%d) (%d,%d) %s %s\n\0" as *const u8
                        as *const libc::c_char,
                    A_2.x,
                    B_1.y,
                    B_1.x,
                    A_2.y,
                    url,
                    tooltip,
                );
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"gvrender_core_map.c\0" as *const u8 as *const libc::c_char,
                    80 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 91],
                        &[libc::c_char; 91],
                    >(
                        b"void map_output_shape(GVJ_t *, map_shape_t, pointf *, int, char *, char *, char *, char *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    } else if (*job).render.id == FORMAT_CMAP as libc::c_int
            || (*job).render.id == FORMAT_CMAPX as libc::c_int
        {
        match map_shape as libc::c_uint {
            1 => {
                gvputs(
                    job,
                    b"<area shape=\"circle\"\0" as *const u8 as *const libc::c_char,
                );
            }
            0 => {
                gvputs(
                    job,
                    b"<area shape=\"rect\"\0" as *const u8 as *const libc::c_char,
                );
            }
            2 => {
                gvputs(
                    job,
                    b"<area shape=\"poly\"\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"gvrender_core_map.c\0" as *const u8 as *const libc::c_char,
                    96 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 91],
                        &[libc::c_char; 91],
                    >(
                        b"void map_output_shape(GVJ_t *, map_shape_t, pointf *, int, char *, char *, char *, char *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        if !id.is_null() && *id.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvputs(job, b" id=\"\0" as *const u8 as *const libc::c_char);
            xml_url_puts(job, id);
            gvputs(job, b"\"\0" as *const u8 as *const libc::c_char);
        }
        if !url.is_null() && *url.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvputs(job, b" href=\"\0" as *const u8 as *const libc::c_char);
            xml_url_puts(job, url);
            gvputs(job, b"\"\0" as *const u8 as *const libc::c_char);
        }
        if !target.is_null()
            && *target.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            gvputs(job, b" target=\"\0" as *const u8 as *const libc::c_char);
            gvputs_xml(job, target);
            gvputs(job, b"\"\0" as *const u8 as *const libc::c_char);
        }
        if !tooltip.is_null()
            && *tooltip.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            gvputs(job, b" title=\"\0" as *const u8 as *const libc::c_char);
            gvputs_xml(job, tooltip);
            gvputs(job, b"\"\0" as *const u8 as *const libc::c_char);
        }
        gvputs(job, b" alt=\"\"\0" as *const u8 as *const libc::c_char);
        gvputs(job, b" coords=\"\0" as *const u8 as *const libc::c_char);
        match map_shape as libc::c_uint {
            1 => {
                let mut A_3: point = point { x: 0, y: 0 };
                let mut B_2: point = point { x: 0, y: 0 };
                A_3
                    .x = (if (*AF.offset(0 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                A_3
                    .y = (if (*AF.offset(0 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                B_2
                    .x = (if (*AF.offset(1 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                B_2
                    .y = (if (*AF.offset(1 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                gvprintf(
                    job,
                    b"%d,%d,%d\0" as *const u8 as *const libc::c_char,
                    A_3.x,
                    A_3.y,
                    B_2.x - A_3.x,
                );
            }
            0 => {
                let mut A_4: point = point { x: 0, y: 0 };
                let mut B_3: point = point { x: 0, y: 0 };
                A_4
                    .x = (if (*AF.offset(0 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                A_4
                    .y = (if (*AF.offset(0 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                B_3
                    .x = (if (*AF.offset(1 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                B_3
                    .y = (if (*AF.offset(1 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(1 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(1 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                gvprintf(
                    job,
                    b"%d,%d,%d,%d\0" as *const u8 as *const libc::c_char,
                    A_4.x,
                    B_3.y,
                    B_3.x,
                    A_4.y,
                );
            }
            2 => {
                let mut A_5: point = point { x: 0, y: 0 };
                A_5
                    .x = (if (*AF.offset(0 as libc::c_int as isize)).x
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
                });
                A_5
                    .y = (if (*AF.offset(0 as libc::c_int as isize)).y
                    >= 0 as libc::c_int as libc::c_double
                {
                    ((*AF.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*AF.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
                });
                gvprintf(
                    job,
                    b"%d,%d\0" as *const u8 as *const libc::c_char,
                    A_5.x,
                    A_5.y,
                );
                i = 1 as libc::c_int;
                while i < nump {
                    A_5
                        .x = (if (*AF.offset(i as isize)).x
                        >= 0 as libc::c_int as libc::c_double
                    {
                        ((*AF.offset(i as isize)).x + 0.5f64) as libc::c_int
                    } else {
                        ((*AF.offset(i as isize)).x - 0.5f64) as libc::c_int
                    });
                    A_5
                        .y = (if (*AF.offset(i as isize)).y
                        >= 0 as libc::c_int as libc::c_double
                    {
                        ((*AF.offset(i as isize)).y + 0.5f64) as libc::c_int
                    } else {
                        ((*AF.offset(i as isize)).y - 0.5f64) as libc::c_int
                    });
                    gvprintf(
                        job,
                        b",%d,%d\0" as *const u8 as *const libc::c_char,
                        A_5.x,
                        A_5.y,
                    );
                    i += 1;
                }
            }
            _ => {}
        }
        if (*job).render.id == FORMAT_CMAPX as libc::c_int {
            gvputs(job, b"\"/>\n\0" as *const u8 as *const libc::c_char);
        } else {
            gvputs(job, b"\">\n\0" as *const u8 as *const libc::c_char);
        }
    }
}
unsafe extern "C" fn map_begin_page(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    match (*job).render.id {
        0 => {
            gvputs(job, b"base referer\n\0" as *const u8 as *const libc::c_char);
            if !((*obj).url).is_null()
                && *((*obj).url).offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                gvputs(job, b"default \0" as *const u8 as *const libc::c_char);
                gvputs_xml(job, (*obj).url);
                gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        1 => {
            if !((*obj).url).is_null()
                && *((*obj).url).offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                gvputs(job, b"default \0" as *const u8 as *const libc::c_char);
                gvputs_xml(job, (*obj).url);
                gvputs(job, b" \0" as *const u8 as *const libc::c_char);
                gvputs_xml(job, agnameof((*obj).u.g as *mut libc::c_void));
                gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        3 => {
            gvputs(job, b"<map id=\"\0" as *const u8 as *const libc::c_char);
            gvputs_xml(job, agnameof((*obj).u.g as *mut libc::c_void));
            gvputs(job, b"\" name=\"\0" as *const u8 as *const libc::c_char);
            gvputs_xml(job, agnameof((*obj).u.g as *mut libc::c_void));
            gvputs(job, b"\">\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    };
}
unsafe extern "C" fn map_end_page(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    match (*job).render.id {
        2 => {
            map_output_shape(
                job,
                (*obj).url_map_shape,
                (*obj).url_map_p,
                (*obj).url_map_n,
                (*obj).url,
                (*obj).tooltip,
                (*obj).target,
                (*obj).id,
            );
        }
        3 => {
            map_output_shape(
                job,
                (*obj).url_map_shape,
                (*obj).url_map_p,
                (*obj).url_map_n,
                (*obj).url,
                (*obj).tooltip,
                (*obj).target,
                (*obj).id,
            );
            gvputs(job, b"</map>\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    };
}
unsafe extern "C" fn map_begin_anchor(
    mut job: *mut GVJ_t,
    mut url: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    map_output_shape(
        job,
        (*obj).url_map_shape,
        (*obj).url_map_p,
        (*obj).url_map_n,
        url,
        tooltip,
        target,
        id,
    );
}
static mut map_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: None,
            end_job: None,
            begin_graph: None,
            end_graph: None,
            begin_layer: None,
            end_layer: None,
            begin_page: Some(map_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_page: Some(map_end_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_cluster: None,
            end_cluster: None,
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: None,
            end_node: None,
            begin_edge: None,
            end_edge: None,
            begin_anchor: Some(
                map_begin_anchor
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
            textspan: None,
            resolve_color: None,
            ellipse: None,
            polygon: None,
            beziercurve: None,
            polyline: None,
            comment: None,
            library_shape: None,
        };
        init
    }
};
static mut render_features_map: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int
            | (1 as libc::c_int) << 23 as libc::c_int
            | (1 as libc::c_int) << 17 as libc::c_int,
        default_pad: 4.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: HSVA_DOUBLE,
    };
    init
};
static mut device_features_map: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 18 as libc::c_int
            | (1 as libc::c_int) << 19 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 96.0f64, y: 96.0f64 };
            init
        },
    };
    init
};
static mut device_features_map_nopoly: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: 0 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 96.0f64, y: 96.0f64 };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_map_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_ISMAP as libc::c_int,
                type_0: b"map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &map_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_map as *const gvrender_features_t
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
pub static mut gvdevice_map_types: [gvplugin_installed_t; 7] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_ISMAP as libc::c_int,
                type_0: b"ismap:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_map_nopoly as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_CMAP as libc::c_int,
                type_0: b"cmap:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_map as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_IMAP as libc::c_int,
                type_0: b"imap:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_map as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_CMAPX as libc::c_int,
                type_0: b"cmapx:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_map as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_IMAP as libc::c_int,
                type_0: b"imap_np:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_map_nopoly as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_CMAPX as libc::c_int,
                type_0: b"cmapx_np:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_map_nopoly as *const gvdevice_features_t
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
