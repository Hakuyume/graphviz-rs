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
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type GVC_s;
    pub type _cairo;
    pub type _cairo_surface;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn gvusershape_file_release(us: *mut usershape_t);
    fn gvusershape_file_access(us: *mut usershape_t) -> bool;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn cairo_save(cr: *mut cairo_t);
    fn cairo_restore(cr: *mut cairo_t);
    fn cairo_set_source_surface(
        cr: *mut cairo_t,
        surface: *mut cairo_surface_t,
        x: libc::c_double,
        y: libc::c_double,
    );
    fn cairo_translate(cr: *mut cairo_t, tx: libc::c_double, ty: libc::c_double);
    fn cairo_scale(cr: *mut cairo_t, sx: libc::c_double, sy: libc::c_double);
    fn cairo_paint(cr: *mut cairo_t);
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) -> *mut libc::c_uchar;
    fn cairo_image_surface_get_format(surface: *mut cairo_surface_t) -> cairo_format_t;
    fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> libc::c_int;
    fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> libc::c_int;
    fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) -> libc::c_int;
    fn cairo_image_surface_create_from_png_stream(
        read_func: cairo_read_func_t,
        closure: *mut libc::c_void,
    ) -> *mut cairo_surface_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvloadimage_engine_s {
    pub loadimage: Option<unsafe extern "C" fn(*mut GVJ_t, *mut usershape_t, boxf, bool) -> ()>,
}
pub type usershape_t = usershape_s;
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
pub type Dtlink_t = _dtlink_s;
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
    pub u: C2RustUnnamed_1,
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
    pub u: C2RustUnnamed_0,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub union C2RustUnnamed_1 {
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type _cairo_status = libc::c_uint;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 44;
pub const CAIRO_STATUS_DWRITE_ERROR: _cairo_status = 43;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub type cairo_status_t = _cairo_status;
pub type _cairo_format = libc::c_int;
pub const CAIRO_FORMAT_RGBA128F: _cairo_format = 7;
pub const CAIRO_FORMAT_RGB96F: _cairo_format = 6;
pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;
pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
pub const CAIRO_FORMAT_A1: _cairo_format = 3;
pub const CAIRO_FORMAT_A8: _cairo_format = 2;
pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
pub type cairo_format_t = _cairo_format;
pub type cairo_read_func_t = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_uchar, libc::c_uint) -> cairo_status_t,
>;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FORMAT_PNG_PS: C2RustUnnamed_4 = 1;
pub const FORMAT_PNG_CAIRO: C2RustUnnamed_4 = 0;
unsafe extern "C" fn reader(
    mut closure: *mut libc::c_void,
    mut data: *mut libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    if !closure.is_null() {
    } else {
        __assert_fail(
            b"closure\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"cairo_status_t reader(void *, unsigned char *, unsigned int)\0",
            ))
            .as_ptr(),
        );
    }
    if length as libc::c_ulong
        == fread(
            data as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            length as libc::c_ulong,
            closure as *mut FILE,
        )
        || feof(closure as *mut FILE) != 0
    {
        return CAIRO_STATUS_SUCCESS;
    }
    return CAIRO_STATUS_READ_ERROR;
}
unsafe extern "C" fn cairo_freeimage(mut us: *mut usershape_t) {
    cairo_surface_destroy((*us).data as *mut cairo_surface_t);
}
unsafe extern "C" fn cairo_loadimage(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
) -> *mut cairo_surface_t {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    if !job.is_null() {
    } else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"cairo_surface_t *cairo_loadimage(GVJ_t *, usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !us.is_null() {
    } else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"cairo_surface_t *cairo_loadimage(GVJ_t *, usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*us).name).is_null() {
    } else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"cairo_surface_t *cairo_loadimage(GVJ_t *, usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if *((*us).name).offset(0 as libc::c_int as isize) != 0 {
    } else {
        __assert_fail(
            b"us->name[0]\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"cairo_surface_t *cairo_loadimage(GVJ_t *, usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*us).data).is_null() {
        if (*us).datafree == Some(cairo_freeimage as unsafe extern "C" fn(*mut usershape_t) -> ()) {
            surface = (*us).data as *mut cairo_surface_t;
        } else {
            ((*us).datafree).expect("non-null function pointer")(us);
            let ref mut fresh0 = (*us).datafree;
            *fresh0 = None;
            let ref mut fresh1 = (*us).data;
            *fresh1 = 0 as *mut libc::c_void;
        }
    }
    if surface.is_null() {
        if !gvusershape_file_access(us) {
            return 0 as *mut cairo_surface_t;
        }
        if !((*us).f).is_null() {
        } else {
            __assert_fail(
                b"us->f\0" as *const u8 as *const libc::c_char,
                b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"cairo_surface_t *cairo_loadimage(GVJ_t *, usershape_t *)\0",
                ))
                .as_ptr(),
            );
        }
        match (*us).type_0 as libc::c_uint {
            3 => {
                surface = cairo_image_surface_create_from_png_stream(
                    Some(
                        reader
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_uchar,
                                libc::c_uint,
                            ) -> cairo_status_t,
                    ),
                    (*us).f as *mut libc::c_void,
                );
                cairo_surface_reference(surface);
            }
            _ => {
                surface = 0 as *mut cairo_surface_t;
            }
        }
        if !surface.is_null() {
            let ref mut fresh2 = (*us).data;
            *fresh2 = surface as *mut libc::c_void;
            let ref mut fresh3 = (*us).datafree;
            *fresh3 = Some(cairo_freeimage as unsafe extern "C" fn(*mut usershape_t) -> ());
        }
        gvusershape_file_release(us);
    }
    return surface;
}
unsafe extern "C" fn pango_loadimage_cairo(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    if !job.is_null() {
    } else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"void pango_loadimage_cairo(GVJ_t *, usershape_t *, boxf, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    if !us.is_null() {
    } else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"void pango_loadimage_cairo(GVJ_t *, usershape_t *, boxf, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*us).name).is_null() {
    } else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"void pango_loadimage_cairo(GVJ_t *, usershape_t *, boxf, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    if *((*us).name).offset(0 as libc::c_int as isize) != 0 {
    } else {
        __assert_fail(
            b"us->name[0]\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_pango.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"void pango_loadimage_cairo(GVJ_t *, usershape_t *, boxf, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    surface = cairo_loadimage(job, us);
    if !surface.is_null() {
        cairo_save(cr);
        cairo_translate(cr, b.LL.x, -b.UR.y);
        cairo_scale(
            cr,
            (b.UR.x - b.LL.x) / (*us).w as libc::c_double,
            (b.UR.y - b.LL.y) / (*us).h as libc::c_double,
        );
        cairo_set_source_surface(
            cr,
            surface,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
        cairo_paint(cr);
        cairo_restore(cr);
    }
}
unsafe extern "C" fn pango_loadimage_ps(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut format: cairo_format_t = CAIRO_FORMAT_ARGB32;
    let mut X: libc::c_int = 0;
    let mut Y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ix: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut alpha: libc::c_uchar = 0;
    let mut red: libc::c_uchar = 0;
    let mut green: libc::c_uchar = 0;
    let mut blue: libc::c_uchar = 0;
    surface = cairo_loadimage(job, us);
    if !surface.is_null() {
        format = cairo_image_surface_get_format(surface);
        if format as libc::c_int != CAIRO_FORMAT_ARGB32 as libc::c_int
            && format as libc::c_int != CAIRO_FORMAT_RGB24 as libc::c_int
        {
            return;
        }
        X = cairo_image_surface_get_width(surface);
        Y = cairo_image_surface_get_height(surface);
        stride = cairo_image_surface_get_stride(surface);
        data = cairo_image_surface_get_data(surface);
        gvputs(job, b"save\n\0" as *const u8 as *const libc::c_char);
        gvputs(job, b"/myctr 0 def\n\0" as *const u8 as *const libc::c_char);
        gvputs(job, b"/myarray [\n\0" as *const u8 as *const libc::c_char);
        y = 0 as libc::c_int;
        while y < Y {
            gvputs(job, b"<\0" as *const u8 as *const libc::c_char);
            ix = data.offset((y * stride) as isize);
            x = 0 as libc::c_int;
            while x < X {
                let fresh4 = ix;
                ix = ix.offset(1);
                blue = *fresh4;
                let fresh5 = ix;
                ix = ix.offset(1);
                green = *fresh5;
                let fresh6 = ix;
                ix = ix.offset(1);
                red = *fresh6;
                let fresh7 = ix;
                ix = ix.offset(1);
                alpha = *fresh7;
                if (alpha as libc::c_int) < 0x7f as libc::c_int {
                    gvputs(job, b"ffffff\0" as *const u8 as *const libc::c_char);
                } else {
                    gvprintf(
                        job,
                        b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
                        red as libc::c_int,
                        green as libc::c_int,
                        blue as libc::c_int,
                    );
                }
                x += 1;
            }
            gvputs(job, b">\n\0" as *const u8 as *const libc::c_char);
            y += 1;
        }
        gvputs(job, b"] def\n\0" as *const u8 as *const libc::c_char);
        gvputs(
            job,
            b"/myproc { myarray myctr get /myctr myctr 1 add def } def\n\0" as *const u8
                as *const libc::c_char,
        );
        gvprintf(
            job,
            b"%g %g translate\n\0" as *const u8 as *const libc::c_char,
            b.LL.x + (b.UR.x - b.LL.x) * (1.0f64 - (*job).dpi.x / 96.0f64) / 2.0f64,
            b.LL.y + (b.UR.y - b.LL.y) * (1.0f64 - (*job).dpi.y / 96.0f64) / 2.0f64,
        );
        gvprintf(
            job,
            b"%g %g scale\n\0" as *const u8 as *const libc::c_char,
            (b.UR.x - b.LL.x) * 72.0f64 / 96.0f64,
            (b.UR.y - b.LL.y) * 72.0f64 / 96.0f64,
        );
        gvprintf(
            job,
            b"%d %d 8 [%d 0 0 %d 0 %d]\n\0" as *const u8 as *const libc::c_char,
            X,
            Y,
            X,
            -Y,
            Y,
        );
        gvputs(
            job,
            b"{myproc} false 3 colorimage\n\0" as *const u8 as *const libc::c_char,
        );
        gvputs(job, b"restore\n\0" as *const u8 as *const libc::c_char);
    }
}
static mut engine_cairo: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                pango_loadimage_cairo
                    as unsafe extern "C" fn(*mut GVJ_t, *mut usershape_t, boxf, bool) -> (),
            ),
        };
        init
    }
};
static mut engine_ps: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                pango_loadimage_ps
                    as unsafe extern "C" fn(*mut GVJ_t, *mut usershape_t, boxf, bool) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gvloadimage_pango_types: [gvplugin_installed_t; 4] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_CAIRO as libc::c_int,
                type_0: b"png:cairo\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_cairo as *const gvloadimage_engine_t as *mut gvloadimage_engine_t
                    as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_PS as libc::c_int,
                type_0: b"png:lasi\0" as *const u8 as *const libc::c_char,
                quality: 2 as libc::c_int,
                engine: &engine_ps as *const gvloadimage_engine_t as *mut gvloadimage_engine_t
                    as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_PS as libc::c_int,
                type_0: b"png:ps\0" as *const u8 as *const libc::c_char,
                quality: 2 as libc::c_int,
                engine: &engine_ps as *const gvloadimage_engine_t as *mut gvloadimage_engine_t
                    as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
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
