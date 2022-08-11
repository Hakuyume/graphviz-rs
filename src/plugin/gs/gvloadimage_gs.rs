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
    pub type _cairo_pattern;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn gvusershape_file_release(us: *mut usershape_t);
    fn gvusershape_file_access(us: *mut usershape_t) -> bool;
    fn gsapi_revision(pr: *mut gsapi_revision_t, len: libc::c_int) -> libc::c_int;
    fn gsapi_new_instance(
        pinstance: *mut *mut libc::c_void,
        caller_handle: *mut libc::c_void,
    ) -> libc::c_int;
    fn gsapi_delete_instance(instance: *mut libc::c_void);
    fn gsapi_set_stdio(
        instance: *mut libc::c_void,
        stdin_fn: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
        >,
        stdout_fn: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
        stderr_fn: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                libc::c_int,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn gsapi_init_with_args(
        instance: *mut libc::c_void,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn gsapi_run_file(
        instance: *mut libc::c_void,
        file_name: *const libc::c_char,
        user_errors: libc::c_int,
        pexit_code: *mut libc::c_int,
    ) -> libc::c_int;
    fn gsapi_exit(instance: *mut libc::c_void) -> libc::c_int;
    static gs_error_names: [*const libc::c_char; 0];
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_save(cr: *mut cairo_t);
    fn cairo_restore(cr: *mut cairo_t);
    fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
    fn cairo_translate(cr: *mut cairo_t, tx: libc::c_double, ty: libc::c_double);
    fn cairo_scale(cr: *mut cairo_t, sx: libc::c_double, sy: libc::c_double);
    fn cairo_paint(cr: *mut cairo_t);
    fn cairo_get_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    fn cairo_surface_create_similar(
        other: *mut cairo_surface_t,
        content: cairo_content_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsapi_revision_s {
    pub product: *const libc::c_char,
    pub copyright: *const libc::c_char,
    pub revision: libc::c_long,
    pub revisiondate: libc::c_long,
}
pub type gsapi_revision_t = gsapi_revision_s;
pub type gs_error_type = libc::c_int;
pub const gs_error_circular_reference: gs_error_type = -112;
pub const gs_error_handled: gs_error_type = -111;
pub const gs_error_Info: gs_error_type = -110;
pub const gs_error_NeedFile: gs_error_type = -107;
pub const gs_error_NeedInput: gs_error_type = -106;
pub const gs_error_VMreclaim: gs_error_type = -105;
pub const gs_error_ExecStackUnderflow: gs_error_type = -104;
pub const gs_error_Remap_Color: gs_error_type = -103;
pub const gs_error_InterpreterExit: gs_error_type = -102;
pub const gs_error_Quit: gs_error_type = -101;
pub const gs_error_Fatal: gs_error_type = -100;
pub const gs_error_hit_detected: gs_error_type = -99;
pub const gs_error_invalidid: gs_error_type = -30;
pub const gs_error_invalidcontext: gs_error_type = -29;
pub const gs_error_unregistered: gs_error_type = -28;
pub const gs_error_undefinedresource: gs_error_type = -27;
pub const gs_error_configurationerror: gs_error_type = -26;
pub const gs_error_VMerror: gs_error_type = -25;
pub const gs_error_unmatchedmark: gs_error_type = -24;
pub const gs_error_undefinedresult: gs_error_type = -23;
pub const gs_error_undefinedfilename: gs_error_type = -22;
pub const gs_error_undefined: gs_error_type = -21;
pub const gs_error_typecheck: gs_error_type = -20;
pub const gs_error_timeout: gs_error_type = -19;
pub const gs_error_syntaxerror: gs_error_type = -18;
pub const gs_error_stackunderflow: gs_error_type = -17;
pub const gs_error_stackoverflow: gs_error_type = -16;
pub const gs_error_rangecheck: gs_error_type = -15;
pub const gs_error_nocurrentpoint: gs_error_type = -14;
pub const gs_error_limitcheck: gs_error_type = -13;
pub const gs_error_ioerror: gs_error_type = -12;
pub const gs_error_invalidrestore: gs_error_type = -11;
pub const gs_error_invalidfont: gs_error_type = -10;
pub const gs_error_invalidfileaccess: gs_error_type = -9;
pub const gs_error_invalidexit: gs_error_type = -8;
pub const gs_error_invalidaccess: gs_error_type = -7;
pub const gs_error_interrupt: gs_error_type = -6;
pub const gs_error_execstackoverflow: gs_error_type = -5;
pub const gs_error_dictstackunderflow: gs_error_type = -4;
pub const gs_error_dictstackoverflow: gs_error_type = -3;
pub const gs_error_dictfull: gs_error_type = -2;
pub const gs_error_unknownerror: gs_error_type = -1;
pub const gs_error_ok: gs_error_type = 0;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type cairo_pattern_t = _cairo_pattern;
pub type _cairo_content = libc::c_uint;
pub const CAIRO_CONTENT_COLOR_ALPHA: _cairo_content = 12288;
pub const CAIRO_CONTENT_ALPHA: _cairo_content = 8192;
pub const CAIRO_CONTENT_COLOR: _cairo_content = 4096;
pub type cairo_content_t = _cairo_content;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FORMAT_EPS_CAIRO: C2RustUnnamed_4 = 1;
pub const FORMAT_PS_CAIRO: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gs_s {
    pub cr: *mut cairo_t,
    pub surface: *mut cairo_surface_t,
    pub pattern: *mut cairo_pattern_t,
}
pub type gs_t = gs_s;
unsafe extern "C" fn gvloadimage_gs_free(mut us: *mut usershape_t) {
    let mut gs: *mut gs_t = (*us).data as *mut gs_t;
    if !((*gs).pattern).is_null() {
        cairo_pattern_destroy((*gs).pattern);
    }
    if !((*gs).surface).is_null() {
        cairo_surface_destroy((*gs).surface);
    }
    free(gs as *mut libc::c_void);
}
unsafe extern "C" fn gs_writer(
    mut caller_handle: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut job: *mut GVJ_t = caller_handle as *mut GVJ_t;
    if (*(*job).common).verbose != 0 {
        if len >= 0 as libc::c_int {
        } else {
            __assert_fail(
                b"len >= 0\0" as *const u8 as *const libc::c_char,
                b"gvloadimage_gs.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"int gs_writer(void *, const char *, int)\0",
                ))
                .as_ptr(),
            );
        }
        return fwrite(
            str as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len as size_t,
            stderr,
        ) as libc::c_int;
    }
    return len;
}
unsafe extern "C" fn gs_error(
    mut job: *mut GVJ_t,
    mut name: *const libc::c_char,
    mut funstr: *const libc::c_char,
    mut err: libc::c_int,
) {
    let mut errsrc: *const libc::c_char = 0 as *const libc::c_char;
    if err < 0 as libc::c_int {
    } else {
        __assert_fail(
            b"err < 0\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_gs.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"void gs_error(GVJ_t *, const char *, const char *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if err >= gs_error_VMerror as libc::c_int {
        errsrc = b"PostScript Level 1\0" as *const u8 as *const libc::c_char;
    } else if err >= gs_error_unregistered as libc::c_int {
        errsrc = b"PostScript Level 2\0" as *const u8 as *const libc::c_char;
    } else if err >= gs_error_invalidid as libc::c_int {
        errsrc = b"DPS error\0" as *const u8 as *const libc::c_char;
    } else {
        errsrc = b"Ghostscript internal error\0" as *const u8 as *const libc::c_char;
    }
    ((*(*job).common).errorfn).expect("non-null function pointer")(
        b"%s: %s() returned: %d \"%s\" (%s)\n\0" as *const u8 as *const libc::c_char,
        name,
        funstr,
        err,
        *gs_error_names
            .as_ptr()
            .offset((-err - 1 as libc::c_int) as isize),
        errsrc,
    );
}
unsafe extern "C" fn gvloadimage_process_file(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut instance: *mut libc::c_void,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut exit_code: libc::c_int = 0;
    if !gvusershape_file_access(us) {
        ((*(*job).common).errorfn).expect("non-null function pointer")(
            b"Failure to read shape file\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    rc = gsapi_run_file(instance, (*us).name, -(1 as libc::c_int), &mut exit_code);
    if rc != 0 {
        gs_error(
            job,
            (*us).name,
            b"gsapi_run_file\0" as *const u8 as *const libc::c_char,
            rc,
        );
    }
    gvusershape_file_release(us);
    return rc;
}
unsafe extern "C" fn gvloadimage_process_surface(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut gs: *mut gs_t,
    mut instance: *mut libc::c_void,
) -> libc::c_int {
    let mut cr: *mut cairo_t = 0 as *mut cairo_t;
    let mut rc: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut width_height: [libc::c_char; 20] = [0; 20];
    let mut dpi: [libc::c_char; 10] = [0; 10];
    let mut cairo_context: [libc::c_char; 30] = [0; 30];
    let mut gs_args: [*mut libc::c_char; 7] = [
        b"dot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-dQUIET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-dNOPAUSE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-sDEVICE=cairo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        cairo_context.as_mut_ptr(),
        width_height.as_mut_ptr(),
        dpi.as_mut_ptr(),
    ];
    let ref mut fresh0 = (*gs).surface;
    *fresh0 = cairo_surface_create_similar(
        cairo_get_target((*gs).cr),
        CAIRO_CONTENT_COLOR_ALPHA,
        (*us).x + (*us).w,
        (*us).y + (*us).h,
    );
    cr = cairo_create((*gs).surface);
    snprintf(
        width_height.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        b"-g%dx%d\0" as *const u8 as *const libc::c_char,
        (*us).x + (*us).w,
        (*us).y + (*us).h,
    );
    snprintf(
        dpi.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
        b"-r%d\0" as *const u8 as *const libc::c_char,
        (*us).dpi,
    );
    snprintf(
        cairo_context.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
        b"-sCairoContext=%p\0" as *const u8 as *const libc::c_char,
        cr,
    );
    rc = gsapi_init_with_args(
        instance,
        (::std::mem::size_of::<[*mut libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int,
        gs_args.as_mut_ptr(),
    );
    cairo_destroy(cr);
    if rc != 0 {
        gs_error(
            job,
            (*us).name,
            b"gsapi_init_with_args\0" as *const u8 as *const libc::c_char,
            rc,
        );
    } else {
        rc = gvloadimage_process_file(job, us, instance);
    }
    if rc != 0 {
        cairo_surface_destroy((*gs).surface);
        let ref mut fresh1 = (*gs).surface;
        *fresh1 = 0 as *mut cairo_surface_t;
    }
    rc2 = gsapi_exit(instance);
    if rc2 != 0 {
        gs_error(
            job,
            (*us).name,
            b"gsapi_exit\0" as *const u8 as *const libc::c_char,
            rc2,
        );
        return rc2;
    }
    if rc == 0 {
        let ref mut fresh2 = (*gs).pattern;
        *fresh2 = cairo_pattern_create_for_surface((*gs).surface);
    }
    return rc;
}
unsafe extern "C" fn gvloadimage_gs_load(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
) -> *mut cairo_pattern_t {
    let mut gs: *mut gs_t = 0 as *mut gs_t;
    let mut gsapi_revision_info: gsapi_revision_t = gsapi_revision_t {
        product: 0 as *const libc::c_char,
        copyright: 0 as *const libc::c_char,
        revision: 0,
        revisiondate: 0,
    };
    let mut instance: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc: libc::c_int = 0;
    if !job.is_null() {
    } else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_gs.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"cairo_pattern_t *gvloadimage_gs_load(GVJ_t *, usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !us.is_null() {
    } else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_gs.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"cairo_pattern_t *gvloadimage_gs_load(GVJ_t *, usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*us).name).is_null() {
    } else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_gs.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"cairo_pattern_t *gvloadimage_gs_load(GVJ_t *, usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*us).data).is_null() {
        if (*us).datafree
            == Some(gvloadimage_gs_free as unsafe extern "C" fn(*mut usershape_t) -> ())
            && (*((*us).data as *mut gs_t)).cr == (*job).context as *mut cairo_t
        {
            gs = (*us).data as *mut gs_t;
        } else {
            ((*us).datafree).expect("non-null function pointer")(us);
            let ref mut fresh3 = (*us).data;
            *fresh3 = 0 as *mut libc::c_void;
        }
    }
    if gs.is_null() {
        gs = malloc(::std::mem::size_of::<gs_t>() as libc::c_ulong) as *mut gs_t;
        if gs.is_null() {
            ((*(*job).common).errorfn).expect("non-null function pointer")(
                b"malloc() failure\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut cairo_pattern_t;
        }
        let ref mut fresh4 = (*gs).cr;
        *fresh4 = (*job).context as *mut cairo_t;
        let ref mut fresh5 = (*gs).surface;
        *fresh5 = 0 as *mut cairo_surface_t;
        let ref mut fresh6 = (*gs).pattern;
        *fresh6 = 0 as *mut cairo_pattern_t;
        let ref mut fresh7 = (*us).data;
        *fresh7 = gs as *mut libc::c_void;
        let ref mut fresh8 = (*us).datafree;
        *fresh8 = Some(gvloadimage_gs_free as unsafe extern "C" fn(*mut usershape_t) -> ());
        rc = gsapi_revision(
            &mut gsapi_revision_info,
            ::std::mem::size_of::<gsapi_revision_t>() as libc::c_ulong as libc::c_int,
        );
        if rc != 0 && rc < ::std::mem::size_of::<gsapi_revision_t>() as libc::c_ulong as libc::c_int
        {
            ((*(*job).common).errorfn).expect("non-null function pointer")(
                b"gs revision - struct too short %d\n\0" as *const u8 as *const libc::c_char,
                rc,
            );
            return 0 as *mut cairo_pattern_t;
        }
        if gsapi_revision_info.revision < 863 as libc::c_int as libc::c_long {
            ((*(*job).common).errorfn).expect("non-null function pointer")(
                b"gs revision - too old %d\n\0" as *const u8 as *const libc::c_char,
                gsapi_revision_info.revision,
            );
            return 0 as *mut cairo_pattern_t;
        }
        rc = gsapi_new_instance(&mut instance, job as *mut libc::c_void);
        if rc != 0 {
            gs_error(
                job,
                (*us).name,
                b"gsapi_new_instance\0" as *const u8 as *const libc::c_char,
                rc,
            );
        } else {
            rc = gsapi_set_stdio(
                instance,
                None,
                Some(
                    gs_writer
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                Some(
                    gs_writer
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
            );
            if rc != 0 {
                gs_error(
                    job,
                    (*us).name,
                    b"gsapi_set_stdio\0" as *const u8 as *const libc::c_char,
                    rc,
                );
            } else {
                rc = gvloadimage_process_surface(job, us, gs, instance);
            }
            gsapi_delete_instance(instance);
        }
    }
    return (*gs).pattern;
}
unsafe extern "C" fn gvloadimage_gs_cairo(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut pattern: *mut cairo_pattern_t = gvloadimage_gs_load(job, us);
    if !pattern.is_null() {
        cairo_save(cr);
        cairo_translate(cr, b.LL.x - (*us).x as libc::c_double, -b.UR.y);
        cairo_scale(
            cr,
            (b.UR.x - b.LL.x) / (*us).w as libc::c_double,
            (b.UR.y - b.LL.y) / (*us).h as libc::c_double,
        );
        cairo_set_source(cr, pattern);
        cairo_paint(cr);
        cairo_restore(cr);
    }
}
static mut engine_cairo: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                gvloadimage_gs_cairo
                    as unsafe extern "C" fn(*mut GVJ_t, *mut usershape_t, boxf, bool) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gvloadimage_gs_types: [gvplugin_installed_t; 3] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_CAIRO as libc::c_int,
                type_0: b"ps:cairo\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_cairo as *const gvloadimage_engine_t as *mut gvloadimage_engine_t
                    as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_EPS_CAIRO as libc::c_int,
                type_0: b"eps:cairo\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_cairo as *const gvloadimage_engine_t as *mut gvloadimage_engine_t
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
