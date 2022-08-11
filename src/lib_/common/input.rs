#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
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
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agmemconcat(g: *mut Agraph_t, cp: *const libc::c_char) -> *mut Agraph_t;
    fn agsetfile(_: *const libc::c_char);
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn aghtmlstr(_: *const libc::c_char) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agclean(g: *mut Agraph_t, kind: libc::c_int, rec_name: *mut libc::c_char);
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agseterr(_: agerrlevel_t) -> agerrlevel_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Lib: *mut *const libc::c_char;
    static mut CmdName: *mut libc::c_char;
    static mut Gvfilepath: *mut libc::c_char;
    static mut Gvimagepath: *mut libc::c_char;
    static mut Verbose: libc::c_uchar;
    static mut Reduce: libc::c_uchar;
    static mut HTTPServerEnVar: *mut libc::c_char;
    static mut graphviz_errors: libc::c_int;
    static mut PSinputscale: libc::c_double;
    static mut CL_type: libc::c_int;
    static mut Concentrate: libc::c_uchar;
    static mut State: libc::c_int;
    static mut EdgeLabelsDone: libc::c_int;
    static mut Initial_dist: libc::c_double;
    static mut Y_invert: libc::c_int;
    static mut GvExitOnUsage: libc::c_int;
    static mut G_ordering: *mut Agsym_t;
    static mut G_gradientangle: *mut Agsym_t;
    static mut G_margin: *mut Agsym_t;
    static mut N_height: *mut Agsym_t;
    static mut N_width: *mut Agsym_t;
    static mut N_shape: *mut Agsym_t;
    static mut N_color: *mut Agsym_t;
    static mut N_fillcolor: *mut Agsym_t;
    static mut N_fontsize: *mut Agsym_t;
    static mut N_fontname: *mut Agsym_t;
    static mut N_fontcolor: *mut Agsym_t;
    static mut N_margin: *mut Agsym_t;
    static mut N_label: *mut Agsym_t;
    static mut N_xlabel: *mut Agsym_t;
    static mut N_nojustify: *mut Agsym_t;
    static mut N_style: *mut Agsym_t;
    static mut N_showboxes: *mut Agsym_t;
    static mut N_sides: *mut Agsym_t;
    static mut N_peripheries: *mut Agsym_t;
    static mut N_ordering: *mut Agsym_t;
    static mut N_orientation: *mut Agsym_t;
    static mut N_skew: *mut Agsym_t;
    static mut N_distortion: *mut Agsym_t;
    static mut N_fixed: *mut Agsym_t;
    static mut N_imagescale: *mut Agsym_t;
    static mut N_imagepos: *mut Agsym_t;
    static mut N_layer: *mut Agsym_t;
    static mut N_group: *mut Agsym_t;
    static mut N_comment: *mut Agsym_t;
    static mut N_vertices: *mut Agsym_t;
    static mut N_z: *mut Agsym_t;
    static mut N_penwidth: *mut Agsym_t;
    static mut N_gradientangle: *mut Agsym_t;
    static mut E_weight: *mut Agsym_t;
    static mut E_minlen: *mut Agsym_t;
    static mut E_color: *mut Agsym_t;
    static mut E_fillcolor: *mut Agsym_t;
    static mut E_fontsize: *mut Agsym_t;
    static mut E_fontname: *mut Agsym_t;
    static mut E_fontcolor: *mut Agsym_t;
    static mut E_label: *mut Agsym_t;
    static mut E_xlabel: *mut Agsym_t;
    static mut E_dir: *mut Agsym_t;
    static mut E_style: *mut Agsym_t;
    static mut E_decorate: *mut Agsym_t;
    static mut E_showboxes: *mut Agsym_t;
    static mut E_arrowsz: *mut Agsym_t;
    static mut E_constr: *mut Agsym_t;
    static mut E_layer: *mut Agsym_t;
    static mut E_comment: *mut Agsym_t;
    static mut E_label_float: *mut Agsym_t;
    static mut E_arrowhead: *mut Agsym_t;
    static mut E_arrowtail: *mut Agsym_t;
    static mut E_headlabel: *mut Agsym_t;
    static mut E_taillabel: *mut Agsym_t;
    static mut E_labelfontsize: *mut Agsym_t;
    static mut E_labelfontname: *mut Agsym_t;
    static mut E_labelfontcolor: *mut Agsym_t;
    static mut E_labeldistance: *mut Agsym_t;
    static mut E_labelangle: *mut Agsym_t;
    static mut E_tailclip: *mut Agsym_t;
    static mut E_headclip: *mut Agsym_t;
    static mut E_penwidth: *mut Agsym_t;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn strdup_and_subst_obj(
        str: *mut libc::c_char,
        obj: *mut libc::c_void,
    ) -> *mut libc::c_char;
    fn late_int(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn late_double(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_double,
        _: libc::c_double,
    ) -> libc::c_double;
    fn late_nnstring(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn late_string(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn maptoken(
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn gvconfig(gvc: *mut GVC_t, rescan: bool);
    fn gvplugin_list(
        gvc: *mut GVC_t,
        api: api_t,
        str: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gvplugin_graph(gvc: *mut GVC_t) -> *mut Agraph_t;
    fn gvjobs_output_filename(gvc: *mut GVC_t, name: *const libc::c_char);
    fn gvjobs_output_langname(gvc: *mut GVC_t, name: *const libc::c_char) -> bool;
    fn gvlayout_select(gvc: *mut GVC_t, str: *const libc::c_char) -> libc::c_int;
    fn make_label(
        obj: *mut libc::c_void,
        str: *mut libc::c_char,
        kind: libc::c_int,
        fontsize: libc::c_double,
        fontname: *mut libc::c_char,
        fontcolor: *mut libc::c_char,
    ) -> *mut textlabel_t;
    fn init_xdot(g: *mut Agraph_t) -> *mut libc::c_void;
    fn free_label(_: *mut textlabel_t);
    fn __errno_location() -> *mut libc::c_int;
    fn freeXDot(_: *mut xdot);
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
    pub free_layout: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
pub type attrsym_t = Agsym_s;
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
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot {
    pub cnt: libc::c_int,
    pub sz: libc::c_int,
    pub ops: *mut xdot_op,
    pub freefunc: freefunc_t,
    pub flags: libc::c_int,
}
pub type freefunc_t = Option::<unsafe extern "C" fn(*mut xdot_op) -> ()>;
pub type xdot_op = _xdot_op;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xdot_op {
    pub kind: xdot_kind,
    pub u: C2RustUnnamed_10,
    pub drawfunc: drawfunc_t,
}
pub type drawfunc_t = Option::<unsafe extern "C" fn(*mut xdot_op, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ellipse: xdot_rect,
    pub polygon: xdot_polyline,
    pub polyline: xdot_polyline,
    pub bezier: xdot_polyline,
    pub text: xdot_text,
    pub image: xdot_image,
    pub color: *mut libc::c_char,
    pub grad_color: xdot_color,
    pub font: xdot_font,
    pub style: *mut libc::c_char,
    pub fontchar: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_font {
    pub size: libc::c_double,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_color {
    pub type_0: xdot_grad_type,
    pub u: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub clr: *mut libc::c_char,
    pub ling: xdot_linear_grad,
    pub ring: xdot_radial_grad,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_radial_grad {
    pub x0: libc::c_double,
    pub y0: libc::c_double,
    pub r0: libc::c_double,
    pub x1: libc::c_double,
    pub y1: libc::c_double,
    pub r1: libc::c_double,
    pub n_stops: libc::c_int,
    pub stops: *mut xdot_color_stop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_color_stop {
    pub frac: libc::c_float,
    pub color: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_linear_grad {
    pub x0: libc::c_double,
    pub y0: libc::c_double,
    pub x1: libc::c_double,
    pub y1: libc::c_double,
    pub n_stops: libc::c_int,
    pub stops: *mut xdot_color_stop,
}
pub type xdot_grad_type = libc::c_uint;
pub const xd_radial: xdot_grad_type = 2;
pub const xd_linear: xdot_grad_type = 1;
pub const xd_none: xdot_grad_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_image {
    pub pos: xdot_rect,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_rect {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub w: libc::c_double,
    pub h: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_text {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub align: xdot_align,
    pub width: libc::c_double,
    pub text: *mut libc::c_char,
}
pub type xdot_align = libc::c_uint;
pub const xd_right: xdot_align = 2;
pub const xd_center: xdot_align = 1;
pub const xd_left: xdot_align = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_polyline {
    pub cnt: libc::c_int,
    pub pts: *mut xdot_point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
pub type xdot_kind = libc::c_uint;
pub const xd_fontchar: xdot_kind = 15;
pub const xd_grad_pen_color: xdot_kind = 14;
pub const xd_grad_fill_color: xdot_kind = 13;
pub const xd_image: xdot_kind = 12;
pub const xd_style: xdot_kind = 11;
pub const xd_font: xdot_kind = 10;
pub const xd_pen_color: xdot_kind = 9;
pub const xd_fill_color: xdot_kind = 8;
pub const xd_text: xdot_kind = 7;
pub const xd_polyline: xdot_kind = 6;
pub const xd_unfilled_bezier: xdot_kind = 5;
pub const xd_filled_bezier: xdot_kind = 4;
pub const xd_unfilled_polygon: xdot_kind = 3;
pub const xd_filled_polygon: xdot_kind = 2;
pub const xd_unfilled_ellipse: xdot_kind = 1;
pub const xd_filled_ellipse: xdot_kind = 0;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
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
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn gv_recalloc(
    mut ptr: *mut libc::c_void,
    mut old_nmemb: size_t,
    mut new_nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size > 0 as libc::c_int as libc::c_ulong
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size))
        as libc::c_int as libc::c_long != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(ptr, old_nmemb.wrapping_mul(size), new_nmemb.wrapping_mul(size));
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
        nbuf = gv_calloc(nsize, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as *mut libc::c_char;
        memcpy(nbuf as *mut libc::c_void, (*xb).buf as *const libc::c_void, cnt);
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
    memcpy((*xb).ptr as *mut libc::c_void, s as *const libc::c_void, ssz);
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
static mut usageFmt: *mut libc::c_char = b"Usage: %s [-Vv?] [-(GNE)name=val] [-(KTlso)<val>] <dot files>\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut genericItems: *mut libc::c_char = b"\n -V          - Print version and exit\n -v          - Enable verbose mode \n -Gname=val  - Set graph attribute 'name' to 'val'\n -Nname=val  - Set node attribute 'name' to 'val'\n -Ename=val  - Set edge attribute 'name' to 'val'\n -Tv         - Set output format to 'v'\n -Kv         - Set layout engine to 'v' (overrides default based on command name)\n -lv         - Use external library 'v'\n -ofile      - Write output to 'file'\n -O          - Automatically generate an output filename based on the input filename with a .'format' appended. (Causes all -ofile options to be ignored.) \n -P          - Internally generate a graph of the current plugins. \n -q[l]       - Set level of message suppression (=1)\n -s[v]       - Scale input by 'v' (=72)\n -y          - Invert y coordinate in output\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut neatoFlags: *mut libc::c_char = b"(additional options for neato)    [-x] [-n<v>]\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut neatoItems: *mut libc::c_char = b"\n -n[v]       - No layout mode 'v' (=1)\n -x          - Reduce graph\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut fdpFlags: *mut libc::c_char = b"(additional options for fdp)      [-L(gO)] [-L(nUCT)<val>]\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut fdpItems: *mut libc::c_char = b"\n -Lg         - Don't use grid\n -LO         - Use old attractive force\n -Ln<i>      - Set number of iterations to i\n -LU<i>      - Set unscaled factor to i\n -LC<v>      - Set overlap expansion factor to v\n -LT[*]<v>   - Set temperature (temperature factor) to v\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut memtestFlags: *mut libc::c_char = b"(additional options for memtest)  [-m<v>]\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut memtestItems: *mut libc::c_char = b"\n -m          - Memory test (Observe no growth with top. Kill when done.)\n -m[v]       - Memory test - v iterations.\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut configFlags: *mut libc::c_char = b"(additional options for config)  [-cv]\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut configItems: *mut libc::c_char = b"\n -c          - Configure plugins (Writes $prefix/lib/graphviz/config \n               with available plugin information.  Needs write privilege.)\n -?          - Print usage and exit\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn dotneato_usage(mut exval: libc::c_int) -> libc::c_int {
    let mut outs: *mut FILE = 0 as *mut FILE;
    if exval > 0 as libc::c_int {
        outs = stderr;
    } else {
        outs = stdout;
    }
    fprintf(outs, usageFmt, CmdName);
    fputs(neatoFlags, outs);
    fputs(fdpFlags, outs);
    fputs(memtestFlags, outs);
    fputs(configFlags, outs);
    fputs(genericItems, outs);
    fputs(neatoItems, outs);
    fputs(fdpItems, outs);
    fputs(memtestItems, outs);
    fputs(configItems, outs);
    if GvExitOnUsage != 0 && exval >= 0 as libc::c_int {
        graphviz_exit(exval);
    }
    return exval + 1 as libc::c_int;
}
unsafe extern "C" fn getFlagOpt(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut idx: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = *idx;
    let mut arg: *mut libc::c_char = *argv.offset(i as isize);
    if *arg.offset(2 as libc::c_int as isize) != 0 {
        return arg.offset(2 as libc::c_int as isize);
    }
    if i < argc - 1 as libc::c_int {
        i += 1;
        arg = *argv.offset(i as isize);
        if *arg as libc::c_int != 0 && *arg as libc::c_int != '-' as i32 {
            *idx = i;
            return arg;
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn dotneato_basename(
    mut path: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = path;
    if *s as libc::c_int == '\0' as i32 {
        return path;
    }
    while *s != 0 {
        s = s.offset(1);
    }
    s = s.offset(-1);
    while s > path
        && (*s as libc::c_int == '/' as i32 || *s as libc::c_int == '\\' as i32)
    {
        let fresh11 = s;
        s = s.offset(-1);
        *fresh11 = '\0' as i32 as libc::c_char;
    }
    if s == path {
        ret = path;
    } else {
        while s > path
            && (*s as libc::c_int != '/' as i32 && *s as libc::c_int != '\\' as i32)
        {
            s = s.offset(-1);
        }
        if *s as libc::c_int == '/' as i32 || *s as libc::c_int == '\\' as i32 {
            ret = s.offset(1 as libc::c_int as isize);
        } else {
            ret = path;
        }
    }
    return ret;
}
unsafe extern "C" fn use_library(mut gvc: *mut GVC_t, mut name: *const libc::c_char) {
    static mut cnt: size_t = 0 as libc::c_int as size_t;
    if !name.is_null() {
        Lib = if !Lib.is_null() {
            grealloc(
                Lib as *mut libc::c_void,
                cnt
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char
        } else {
            gmalloc(
                cnt
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char
        };
        let fresh12 = cnt;
        cnt = cnt.wrapping_add(1);
        let ref mut fresh13 = *Lib.offset(fresh12 as isize);
        *fresh13 = name;
        let ref mut fresh14 = *Lib.offset(cnt as isize);
        *fresh14 = 0 as *const libc::c_char;
    }
    let ref mut fresh15 = (*gvc).common.lib;
    *fresh15 = Lib;
}
unsafe extern "C" fn global_def(
    mut xb: *mut agxbuf,
    mut dcl: *mut libc::c_char,
    mut kind: libc::c_int,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rhs: *mut libc::c_char = b"true\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut sym: *mut attrsym_t = 0 as *mut attrsym_t;
    p = strchr(dcl, '=' as i32);
    if !p.is_null() {
        agxbput_n(xb, dcl, p.offset_from(dcl) as libc::c_long as size_t);
        rhs = p.offset(1 as libc::c_int as isize);
    } else {
        agxbput(xb, dcl);
    }
    sym = agattr(0 as *mut Agraph_t, kind, agxbuse(xb), rhs);
    (*sym).fixed = 1 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn gvg_init(
    mut gvc: *mut GVC_t,
    mut g: *mut graph_t,
    mut fn_0: *mut libc::c_char,
    mut gidx: libc::c_int,
) -> libc::c_int {
    let mut gvg: *mut GVG_t = 0 as *mut GVG_t;
    gvg = zmalloc(::std::mem::size_of::<GVG_t>() as libc::c_ulong) as *mut GVG_t;
    if ((*gvc).gvgs).is_null() {
        let ref mut fresh16 = (*gvc).gvgs;
        *fresh16 = gvg;
    } else {
        let ref mut fresh17 = (*(*gvc).gvg).next;
        *fresh17 = gvg;
    }
    let ref mut fresh18 = (*gvc).gvg;
    *fresh18 = gvg;
    let ref mut fresh19 = (*gvg).gvc;
    *fresh19 = gvc;
    let ref mut fresh20 = (*gvg).g;
    *fresh20 = g;
    let ref mut fresh21 = (*gvg).input_filename;
    *fresh21 = fn_0;
    (*gvg).graph_index = gidx;
    return 0 as libc::c_int;
}
static mut P_graph: *mut graph_t = 0 as *const graph_t as *mut graph_t;
#[no_mangle]
pub unsafe extern "C" fn gvPluginsGraph(mut gvc: *mut GVC_t) -> *mut graph_t {
    gvg_init(
        gvc,
        P_graph,
        b"<internal>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    return P_graph;
}
#[no_mangle]
pub unsafe extern "C" fn dotneato_args_initialize(
    mut gvc: *mut GVC_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut rest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut layout: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut nfiles: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut Kflag: libc::c_int = 0 as libc::c_int;
    HTTPServerEnVar = getenv(b"SERVER_NAME\0" as *const u8 as *const libc::c_char);
    Gvfilepath = getenv(b"GV_FILE_PATH\0" as *const u8 as *const libc::c_char);
    let ref mut fresh22 = (*gvc).common.cmdname;
    *fresh22 = dotneato_basename(*argv.offset(0 as libc::c_int as isize));
    if (*gvc).common.verbose != 0 {
        fprintf(
            stderr,
            b"%s - %s version %s (%s)\n\0" as *const u8 as *const libc::c_char,
            (*gvc).common.cmdname,
            *((*gvc).common.info).offset(0 as libc::c_int as isize),
            *((*gvc).common.info).offset(1 as libc::c_int as isize),
            *((*gvc).common.info).offset(2 as libc::c_int as isize),
        );
    }
    if (*gvc).common.config {
        gvconfig(gvc, (*gvc).common.config);
        graphviz_exit(0 as libc::c_int);
    }
    Verbose = (*gvc).common.verbose as libc::c_uchar;
    CmdName = (*gvc).common.cmdname;
    nfiles = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if !(*argv.offset(i as isize)).is_null()
            && *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int != '-' as i32
        {
            nfiles += 1;
        }
        i += 1;
    }
    let ref mut fresh23 = (*gvc).input_filenames;
    *fresh23 = gcalloc(
        (nfiles + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    nfiles = 0 as libc::c_int;
    agxbinit(&mut xb, 128 as libc::c_int as libc::c_uint, buf.as_mut_ptr());
    i = 1 as libc::c_int;
    while i < argc {
        if !(*argv.offset(i as isize)).is_null()
            && *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32
        {
            rest = &mut *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize)
                as *mut libc::c_char;
            c = *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize);
            match c as libc::c_int {
                71 => {
                    if *rest != 0 {
                        global_def(&mut xb, rest, 0 as libc::c_int);
                    } else {
                        fprintf(
                            stderr,
                            b"Missing argument for -G flag\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return dotneato_usage(1 as libc::c_int);
                    }
                }
                78 => {
                    if *rest != 0 {
                        global_def(&mut xb, rest, 1 as libc::c_int);
                    } else {
                        fprintf(
                            stderr,
                            b"Missing argument for -N flag\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return dotneato_usage(1 as libc::c_int);
                    }
                }
                69 => {
                    if *rest != 0 {
                        global_def(&mut xb, rest, 2 as libc::c_int);
                    } else {
                        fprintf(
                            stderr,
                            b"Missing argument for -E flag\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return dotneato_usage(1 as libc::c_int);
                    }
                }
                84 => {
                    val = getFlagOpt(argc, argv, &mut i);
                    if val.is_null() {
                        fprintf(
                            stderr,
                            b"Missing argument for -T flag\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return dotneato_usage(1 as libc::c_int);
                    }
                    if !gvjobs_output_langname(gvc, val) {
                        let mut fmts: *mut libc::c_char = 0 as *mut libc::c_char;
                        fprintf(
                            stderr,
                            b"Format: \"%s\" not recognized.\0" as *const u8
                                as *const libc::c_char,
                            val,
                        );
                        fmts = gvplugin_list(gvc, API_device, val);
                        if strlen(fmts) > 1 as libc::c_int as libc::c_ulong {
                            fprintf(
                                stderr,
                                b" Use one of:%s\n\0" as *const u8 as *const libc::c_char,
                                fmts,
                            );
                        } else {
                            fprintf(
                                stderr,
                                b" No formats found.\nPerhaps \"dot -c\" needs to be run (with installer's privileges) to register the plugins?\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        if GvExitOnUsage != 0 {
                            graphviz_exit(1 as libc::c_int);
                        }
                        return 2 as libc::c_int;
                    }
                }
                75 => {
                    val = getFlagOpt(argc, argv, &mut i);
                    if val.is_null() {
                        fprintf(
                            stderr,
                            b"Missing argument for -K flag\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return dotneato_usage(1 as libc::c_int);
                    }
                    v = gvlayout_select(gvc, val);
                    if v == 999 as libc::c_int {
                        fprintf(
                            stderr,
                            b"There is no layout engine support for \"%s\"\n\0"
                                as *const u8 as *const libc::c_char,
                            val,
                        );
                        if strcmp(val, b"dot\0" as *const u8 as *const libc::c_char) == 0
                        {
                            fprintf(
                                stderr,
                                b"Perhaps \"dot -c\" needs to be run (with installer's privileges) to register the plugins?\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        } else {
                            let mut lyts: *mut libc::c_char = 0 as *mut libc::c_char;
                            lyts = gvplugin_list(gvc, API_layout, val);
                            if strlen(lyts) > 1 as libc::c_int as libc::c_ulong {
                                fprintf(
                                    stderr,
                                    b" Use one of:%s\n\0" as *const u8 as *const libc::c_char,
                                    lyts,
                                );
                            } else {
                                fprintf(
                                    stderr,
                                    b" No layouts found.\nPerhaps \"dot -c\" needs to be run (with installer's privileges) to register the plugins?\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        if GvExitOnUsage != 0 {
                            graphviz_exit(1 as libc::c_int);
                        }
                        return 2 as libc::c_int;
                    }
                    Kflag = 1 as libc::c_int;
                }
                80 => {
                    P_graph = gvplugin_graph(gvc);
                }
                86 => {
                    fprintf(
                        stderr,
                        b"%s - %s version %s (%s)\n\0" as *const u8
                            as *const libc::c_char,
                        (*gvc).common.cmdname,
                        *((*gvc).common.info).offset(0 as libc::c_int as isize),
                        *((*gvc).common.info).offset(1 as libc::c_int as isize),
                        *((*gvc).common.info).offset(2 as libc::c_int as isize),
                    );
                    if GvExitOnUsage != 0 {
                        graphviz_exit(0 as libc::c_int);
                    }
                    return 1 as libc::c_int;
                }
                108 => {
                    val = getFlagOpt(argc, argv, &mut i);
                    if val.is_null() {
                        fprintf(
                            stderr,
                            b"Missing argument for -l flag\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return dotneato_usage(1 as libc::c_int);
                    }
                    use_library(gvc, val);
                }
                111 => {
                    val = getFlagOpt(argc, argv, &mut i);
                    if val.is_null() {
                        fprintf(
                            stderr,
                            b"Missing argument for -o flag\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return dotneato_usage(1 as libc::c_int);
                    }
                    if !(*gvc).common.auto_outfile_names {
                        gvjobs_output_filename(gvc, val);
                    }
                }
                113 => {
                    if *rest != 0 {
                        v = atoi(rest);
                        if v <= 0 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Invalid parameter \"%s\" for -q flag - ignored\n\0"
                                    as *const u8 as *const libc::c_char,
                                rest,
                            );
                        } else if v == 1 as libc::c_int {
                            agseterr(AGERR);
                        } else {
                            agseterr(AGMAX);
                        }
                    } else {
                        agseterr(AGERR);
                    }
                }
                115 => {
                    if *rest != 0 {
                        PSinputscale = atof(rest);
                        if PSinputscale < 0 as libc::c_int as libc::c_double {
                            fprintf(
                                stderr,
                                b"Invalid parameter \"%s\" for -s flag\n\0" as *const u8
                                    as *const libc::c_char,
                                rest,
                            );
                            return dotneato_usage(1 as libc::c_int);
                        } else {
                            if PSinputscale == 0 as libc::c_int as libc::c_double {
                                PSinputscale = 72 as libc::c_int as libc::c_double;
                            }
                        }
                    } else {
                        PSinputscale = 72 as libc::c_int as libc::c_double;
                    }
                }
                120 => {
                    Reduce = (0 as libc::c_int == 0) as libc::c_int as libc::c_uchar;
                }
                121 => {
                    Y_invert = (0 as libc::c_int == 0) as libc::c_int;
                }
                63 => return dotneato_usage(0 as libc::c_int),
                _ => {
                    agerr(
                        AGERR,
                        b"%s: option -%c unrecognized\n\n\0" as *const u8
                            as *const libc::c_char,
                        (*gvc).common.cmdname,
                        c as libc::c_int,
                    );
                    return dotneato_usage(1 as libc::c_int);
                }
            }
        } else if !(*argv.offset(i as isize)).is_null() {
            let fresh24 = nfiles;
            nfiles = nfiles + 1;
            let ref mut fresh25 = *((*gvc).input_filenames).offset(fresh24 as isize);
            *fresh25 = *argv.offset(i as isize);
        }
        i += 1;
    }
    agxbfree(&mut xb);
    if Kflag == 0 {
        layout = (*gvc).common.cmdname;
        if strcmp(layout, b"dot_static\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(layout, b"dot_builtins\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(layout, b"lt-dot\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(layout, b"lt-dot_builtins\0" as *const u8 as *const libc::c_char)
                == 0 || strcmp(layout, b"\0" as *const u8 as *const libc::c_char) == 0
        {
            layout = b"dot\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        i = gvlayout_select(gvc, layout);
        if i == 999 as libc::c_int {
            fprintf(
                stderr,
                b"There is no layout engine support for \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                layout,
            );
            if strcmp(layout, b"dot\0" as *const u8 as *const libc::c_char) == 0 {
                fprintf(
                    stderr,
                    b"Perhaps \"dot -c\" needs to be run (with installer's privileges) to register the plugins?\n\0"
                        as *const u8 as *const libc::c_char,
                );
            } else {
                let mut lyts_0: *mut libc::c_char = 0 as *mut libc::c_char;
                lyts_0 = gvplugin_list(
                    gvc,
                    API_layout,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                if strlen(lyts_0) > 1 as libc::c_int as libc::c_ulong {
                    fprintf(
                        stderr,
                        b" Use one of:%s\n\0" as *const u8 as *const libc::c_char,
                        lyts_0,
                    );
                } else {
                    fprintf(
                        stderr,
                        b" No layouts found.\nPerhaps \"dot -c\" needs to be run (with installer's privileges) to register the plugins?\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            if GvExitOnUsage != 0 {
                graphviz_exit(1 as libc::c_int);
            }
            return 2 as libc::c_int;
        }
    }
    if ((*gvc).jobs).is_null() || ((*(*gvc).jobs).output_langname).is_null() {
        if !gvjobs_output_langname(gvc, b"dot\0" as *const u8 as *const libc::c_char) {
            fprintf(
                stderr,
                b"Unable to find even the default \"-Tdot\" renderer.  Has the config\nfile been generated by running \"dot -c\" with installer's privileges?\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 2 as libc::c_int;
        }
    }
    if (agattr(
        0 as *mut Agraph_t,
        1 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    ))
        .is_null()
    {
        agattr(
            0 as *mut Agraph_t,
            1 as libc::c_int,
            b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\\N\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn getdoubles2ptf(
    mut g: *mut graph_t,
    mut name: *mut libc::c_char,
    mut result: *mut pointf,
) -> bool {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut xf: libc::c_double = 0.;
    let mut yf: libc::c_double = 0.;
    let mut c: libc::c_char = '\0' as i32 as libc::c_char;
    let mut rv: bool = 0 as libc::c_int != 0;
    p = agget(g as *mut libc::c_void, name);
    if !p.is_null() {
        i = sscanf(
            p,
            b"%lf,%lf%c\0" as *const u8 as *const libc::c_char,
            &mut xf as *mut libc::c_double,
            &mut yf as *mut libc::c_double,
            &mut c as *mut libc::c_char,
        );
        if i > 1 as libc::c_int && xf > 0 as libc::c_int as libc::c_double
            && yf > 0 as libc::c_int as libc::c_double
        {
            (*result)
                .x = (if xf * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                (xf * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                (xf * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            }) as libc::c_double;
            (*result)
                .y = (if yf * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                (yf * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                (yf * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            }) as libc::c_double;
            if c as libc::c_int == '!' as i32 {
                rv = 1 as libc::c_int != 0;
            }
        } else {
            c = '\0' as i32 as libc::c_char;
            i = sscanf(
                p,
                b"%lf%c\0" as *const u8 as *const libc::c_char,
                &mut xf as *mut libc::c_double,
                &mut c as *mut libc::c_char,
            );
            if i > 0 as libc::c_int && xf > 0 as libc::c_int as libc::c_double {
                let ref mut fresh26 = (*result).x;
                *fresh26 = (if xf * 72 as libc::c_int as libc::c_double
                    >= 0 as libc::c_int as libc::c_double
                {
                    (xf * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
                } else {
                    (xf * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
                }) as libc::c_double;
                (*result).y = *fresh26;
                if c as libc::c_int == '!' as i32 {
                    rv = 1 as libc::c_int != 0;
                }
            }
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn getdouble(
    mut g: *mut graph_t,
    mut name: *mut libc::c_char,
    mut result: *mut libc::c_double,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: libc::c_double = 0.;
    p = agget(g as *mut libc::c_void, name);
    if !p.is_null() {
        if sscanf(
            p,
            b"%lf\0" as *const u8 as *const libc::c_char,
            &mut f as *mut libc::c_double,
        ) >= 1 as libc::c_int
        {
            *result = f;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvNextInputGraph(mut gvc: *mut GVC_t) -> *mut graph_t {
    let mut g: *mut graph_t = 0 as *mut graph_t;
    static mut fn_0: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
    static mut oldfp: *mut FILE = 0 as *const FILE as *mut FILE;
    static mut fidx: libc::c_int = 0;
    static mut gidx: libc::c_int = 0;
    while g.is_null() {
        if fp.is_null() {
            fn_0 = *((*gvc).input_filenames).offset(0 as libc::c_int as isize);
            if fn_0.is_null() {
                let fresh27 = fidx;
                fidx = fidx + 1;
                if fresh27 == 0 as libc::c_int {
                    fp = stdin;
                }
            } else {
                loop {
                    let fresh28 = fidx;
                    fidx = fidx + 1;
                    fn_0 = *((*gvc).input_filenames).offset(fresh28 as isize);
                    if !(!fn_0.is_null()
                        && {
                            fp = fopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
                            fp.is_null()
                        })
                    {
                        break;
                    }
                    agerr(
                        AGERR,
                        b"%s: can't open %s: %s\n\0" as *const u8 as *const libc::c_char,
                        (*gvc).common.cmdname,
                        fn_0,
                        strerror(*__errno_location()),
                    );
                    graphviz_errors += 1;
                }
            }
        }
        if fp.is_null() {
            break;
        }
        if oldfp != fp {
            agsetfile(
                if !fn_0.is_null() {
                    fn_0 as *const libc::c_char
                } else {
                    b"<stdin>\0" as *const u8 as *const libc::c_char
                },
            );
            oldfp = fp;
        }
        g = agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
        if !g.is_null() {
            let fresh29 = gidx;
            gidx = gidx + 1;
            gvg_init(gvc, g, fn_0, fresh29);
            break;
        } else {
            if fp != stdin {
                fclose(fp);
            }
            fp = 0 as *mut FILE;
            oldfp = fp;
            gidx = 0 as libc::c_int;
        }
    }
    return g;
}
unsafe extern "C" fn findCharset(mut g: *mut graph_t) -> libc::c_int {
    let mut enc: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = late_nnstring(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"charset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        b"utf-8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if strcasecmp(p, b"latin-1\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(p, b"latin1\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(p, b"l1\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(p, b"ISO-8859-1\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(p, b"ISO_8859-1\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(p, b"ISO8859-1\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(p, b"ISO-IR-100\0" as *const u8 as *const libc::c_char) == 0
    {
        enc = 1 as libc::c_int;
    } else if strcasecmp(p, b"big-5\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(p, b"big5\0" as *const u8 as *const libc::c_char) == 0
        {
        enc = 2 as libc::c_int;
    } else if strcasecmp(p, b"utf-8\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(p, b"utf8\0" as *const u8 as *const libc::c_char) == 0
        {
        enc = 0 as libc::c_int;
    } else {
        agerr(
            AGWARN,
            b"Unsupported charset \"%s\" - assuming utf-8\n\0" as *const u8
                as *const libc::c_char,
            p,
        );
        enc = 0 as libc::c_int;
    }
    return enc;
}
unsafe extern "C" fn setRatio(mut g: *mut graph_t) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut ratio: libc::c_double = 0.;
    p = agget(
        g as *mut libc::c_void,
        b"ratio\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null()
        && {
            c = *p.offset(0 as libc::c_int as isize);
            c as libc::c_int != 0
        }
    {
        match c as libc::c_int {
            97 => {
                if strcmp(p, b"auto\0" as *const u8 as *const libc::c_char) == 0 {
                    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                        .ratio_kind = R_AUTO;
                }
            }
            99 => {
                if strcmp(p, b"compress\0" as *const u8 as *const libc::c_char) == 0 {
                    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                        .ratio_kind = R_COMPRESS;
                }
            }
            101 => {
                if strcmp(p, b"expand\0" as *const u8 as *const libc::c_char) == 0 {
                    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                        .ratio_kind = R_EXPAND;
                }
            }
            102 => {
                if strcmp(p, b"fill\0" as *const u8 as *const libc::c_char) == 0 {
                    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                        .ratio_kind = R_FILL;
                }
            }
            _ => {
                ratio = atof(p);
                if ratio > 0.0f64 {
                    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                        .ratio_kind = R_VALUE;
                    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                        .ratio = ratio;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn graph_init(mut g: *mut graph_t, mut use_rankdir: bool) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xf: libc::c_double = 0.;
    static mut rankname: [*mut libc::c_char; 4] = [
        b"local\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"global\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    static mut rankcode: [libc::c_int; 4] = [
        100 as libc::c_int,
        101 as libc::c_int,
        102 as libc::c_int,
        100 as libc::c_int,
    ];
    static mut fontnamenames: [*mut libc::c_char; 4] = [
        b"gd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"svg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    static mut fontnamecodes: [libc::c_int; 4] = [
        NATIVEFONTS as libc::c_int,
        PSFONTS as libc::c_int,
        SVGFONTS as libc::c_int,
        -(1 as libc::c_int),
    ];
    let mut rankdir: libc::c_int = 0;
    let ref mut fresh30 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing;
    *fresh30 = zmalloc(::std::mem::size_of::<layout_t>() as libc::c_ulong)
        as *mut layout_t;
    p = agget(
        g as *mut libc::c_void,
        b"postaction\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        let mut buf: *mut libc::c_char = gmalloc(
            (strlen(b"digraph {  }\0" as *const u8 as *const libc::c_char))
                .wrapping_add(strlen(p))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            buf,
            b"%s { %s }\0" as *const u8 as *const libc::c_char,
            if agisdirected(g) != 0 {
                b"digraph\0" as *const u8 as *const libc::c_char
            } else {
                b"graph\0" as *const u8 as *const libc::c_char
            },
            p,
        );
        agmemconcat(g, buf);
    }
    p = agget(
        g as *mut libc::c_void,
        b"fontpath\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null()
        || {
            p = getenv(b"DOTFONTPATH\0" as *const u8 as *const libc::c_char);
            !p.is_null()
        }
    {
        setenv(b"GDFONTPATH\0" as *const u8 as *const libc::c_char, p, 1 as libc::c_int);
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .charset = findCharset(g) as libc::c_uchar;
    if HTTPServerEnVar.is_null() {
        Gvimagepath = agget(
            g as *mut libc::c_void,
            b"imagepath\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if Gvimagepath.is_null() {
            Gvimagepath = Gvfilepath;
        }
    }
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .quantum = late_double(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"quantum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        0.0f64,
        0.0f64,
    );
    rankdir = 0 as libc::c_int;
    p = agget(
        g as *mut libc::c_void,
        b"rankdir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        if strcmp(p, b"LR\0" as *const u8 as *const libc::c_char) == 0 {
            rankdir = 1 as libc::c_int;
        } else if strcmp(p, b"BT\0" as *const u8 as *const libc::c_char) == 0 {
            rankdir = 2 as libc::c_int;
        } else if strcmp(p, b"RL\0" as *const u8 as *const libc::c_char) == 0 {
            rankdir = 3 as libc::c_int;
        }
    }
    if use_rankdir {
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rankdir = rankdir << 2 as libc::c_int | rankdir;
    } else {
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rankdir = rankdir << 2 as libc::c_int;
    }
    xf = late_double(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"nodesep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        0.25f64,
        0.02f64,
    );
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .nodesep = if xf * 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        (xf * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
    } else {
        (xf * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
    };
    p = late_string(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"ranksep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        0 as *mut libc::c_char,
    );
    if !p.is_null() {
        if sscanf(
            p,
            b"%lf\0" as *const u8 as *const libc::c_char,
            &mut xf as *mut libc::c_double,
        ) == 0 as libc::c_int
        {
            xf = 0.5f64;
        } else if xf < 0.02f64 {
            xf = 0.02f64;
        }
        if !(strstr(p, b"equally\0" as *const u8 as *const libc::c_char)).is_null() {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .exact_ranksep = 1 as libc::c_int != 0;
        }
    } else {
        xf = 0.5f64;
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .ranksep = if xf * 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        (xf * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
    } else {
        (xf * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
    };
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .showboxes = late_int(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"showboxes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        0 as libc::c_int,
        0 as libc::c_int,
    ) as libc::c_uchar;
    p = late_string(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"fontnames\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        0 as *mut libc::c_char,
    );
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .fontnames = maptoken(p, fontnamenames.as_mut_ptr(), fontnamecodes.as_mut_ptr())
        as fontname_kind;
    setRatio(g);
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .filled = getdoubles2ptf(
        g,
        b"size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).size,
    );
    getdoubles2ptf(
        g,
        b"page\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).page,
    );
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .centered = mapbool(
        agget(
            g as *mut libc::c_void,
            b"center\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    p = agget(
        g as *mut libc::c_void,
        b"rotate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
            .landscape = atoi(p) == 90 as libc::c_int;
    } else {
        p = agget(
            g as *mut libc::c_void,
            b"orientation\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !p.is_null() {
            (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .landscape = *p.offset(0 as libc::c_int as isize) as libc::c_int
                == 'l' as i32
                || *p.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32;
        } else {
            p = agget(
                g as *mut libc::c_void,
                b"landscape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !p.is_null() {
                (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                    .landscape = mapbool(p);
            }
        }
    }
    p = agget(
        g as *mut libc::c_void,
        b"clusterrank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    CL_type = maptoken(p, rankname.as_mut_ptr(), rankcode.as_mut_ptr());
    p = agget(
        g as *mut libc::c_void,
        b"concentrate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Concentrate = (if mapbool(p) as libc::c_int != 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uchar;
    State = 0 as libc::c_int;
    EdgeLabelsDone = 0 as libc::c_int;
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).dpi = 0.0f64;
    p = agget(
        g as *mut libc::c_void,
        b"dpi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int != 0
        || {
            p = agget(
                g as *mut libc::c_void,
                b"resolution\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int != 0
        }
    {
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).dpi = atof(p);
    }
    do_graph_label(g);
    Initial_dist = 1.0e+37f64;
    G_ordering = agattr(
        g,
        0 as libc::c_int,
        b"ordering\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    G_gradientangle = agattr(
        g,
        0 as libc::c_int,
        b"gradientangle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    G_margin = agattr(
        g,
        0 as libc::c_int,
        b"margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_height = agattr(
        g,
        1 as libc::c_int,
        b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_width = agattr(
        g,
        1 as libc::c_int,
        b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_shape = agattr(
        g,
        1 as libc::c_int,
        b"shape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_color = agattr(
        g,
        1 as libc::c_int,
        b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fillcolor = agattr(
        g,
        1 as libc::c_int,
        b"fillcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_style = agattr(
        g,
        1 as libc::c_int,
        b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fontsize = agattr(
        g,
        1 as libc::c_int,
        b"fontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fontname = agattr(
        g,
        1 as libc::c_int,
        b"fontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fontcolor = agattr(
        g,
        1 as libc::c_int,
        b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_label = agattr(
        g,
        1 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if N_label.is_null() {
        N_label = agattr(
            g,
            1 as libc::c_int,
            b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\\N\0" as *const u8 as *const libc::c_char,
        );
    }
    N_xlabel = agattr(
        g,
        1 as libc::c_int,
        b"xlabel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_showboxes = agattr(
        g,
        1 as libc::c_int,
        b"showboxes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_penwidth = agattr(
        g,
        1 as libc::c_int,
        b"penwidth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_ordering = agattr(
        g,
        1 as libc::c_int,
        b"ordering\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_margin = agattr(
        g,
        1 as libc::c_int,
        b"margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_sides = agattr(
        g,
        1 as libc::c_int,
        b"sides\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_peripheries = agattr(
        g,
        1 as libc::c_int,
        b"peripheries\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_skew = agattr(
        g,
        1 as libc::c_int,
        b"skew\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_orientation = agattr(
        g,
        1 as libc::c_int,
        b"orientation\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_distortion = agattr(
        g,
        1 as libc::c_int,
        b"distortion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fixed = agattr(
        g,
        1 as libc::c_int,
        b"fixedsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_imagescale = agattr(
        g,
        1 as libc::c_int,
        b"imagescale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_imagepos = agattr(
        g,
        1 as libc::c_int,
        b"imagepos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_nojustify = agattr(
        g,
        1 as libc::c_int,
        b"nojustify\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_layer = agattr(
        g,
        1 as libc::c_int,
        b"layer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_group = agattr(
        g,
        1 as libc::c_int,
        b"group\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_comment = agattr(
        g,
        1 as libc::c_int,
        b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_vertices = agattr(
        g,
        1 as libc::c_int,
        b"vertices\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_z = agattr(
        g,
        1 as libc::c_int,
        b"z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_gradientangle = agattr(
        g,
        1 as libc::c_int,
        b"gradientangle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_weight = agattr(
        g,
        2 as libc::c_int,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_color = agattr(
        g,
        2 as libc::c_int,
        b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_fillcolor = agattr(
        g,
        2 as libc::c_int,
        b"fillcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_fontsize = agattr(
        g,
        2 as libc::c_int,
        b"fontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_fontname = agattr(
        g,
        2 as libc::c_int,
        b"fontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_fontcolor = agattr(
        g,
        2 as libc::c_int,
        b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_label = agattr(
        g,
        2 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_xlabel = agattr(
        g,
        2 as libc::c_int,
        b"xlabel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_label_float = agattr(
        g,
        2 as libc::c_int,
        b"labelfloat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_dir = agattr(
        g,
        2 as libc::c_int,
        b"dir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_arrowhead = agattr(
        g,
        2 as libc::c_int,
        b"arrowhead\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_arrowtail = agattr(
        g,
        2 as libc::c_int,
        b"arrowtail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_headlabel = agattr(
        g,
        2 as libc::c_int,
        b"headlabel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_taillabel = agattr(
        g,
        2 as libc::c_int,
        b"taillabel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_labelfontsize = agattr(
        g,
        2 as libc::c_int,
        b"labelfontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_labelfontname = agattr(
        g,
        2 as libc::c_int,
        b"labelfontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_labelfontcolor = agattr(
        g,
        2 as libc::c_int,
        b"labelfontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_labeldistance = agattr(
        g,
        2 as libc::c_int,
        b"labeldistance\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_labelangle = agattr(
        g,
        2 as libc::c_int,
        b"labelangle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_minlen = agattr(
        g,
        2 as libc::c_int,
        b"minlen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_showboxes = agattr(
        g,
        2 as libc::c_int,
        b"showboxes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_style = agattr(
        g,
        2 as libc::c_int,
        b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_decorate = agattr(
        g,
        2 as libc::c_int,
        b"decorate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_arrowsz = agattr(
        g,
        2 as libc::c_int,
        b"arrowsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_constr = agattr(
        g,
        2 as libc::c_int,
        b"constraint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_layer = agattr(
        g,
        2 as libc::c_int,
        b"layer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_comment = agattr(
        g,
        2 as libc::c_int,
        b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_tailclip = agattr(
        g,
        2 as libc::c_int,
        b"tailclip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_headclip = agattr(
        g,
        2 as libc::c_int,
        b"headclip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_penwidth = agattr(
        g,
        2 as libc::c_int,
        b"penwidth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let ref mut fresh31 = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .drawing)
        .xdots;
    *fresh31 = init_xdot(g);
    p = agget(
        g as *mut libc::c_void,
        b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() && *p as libc::c_int != 0 {
        let ref mut fresh32 = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .drawing)
            .id;
        *fresh32 = strdup_and_subst_obj(p, g as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn graph_cleanup(mut g: *mut graph_t) {
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).is_null()
        && !((*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).xdots)
            .is_null()
    {
        freeXDot(
            (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).xdots
                as *mut xdot,
        );
    }
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).is_null() {
        free(
            (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).id
                as *mut libc::c_void,
        );
    }
    free(
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing
            as *mut libc::c_void,
    );
    let ref mut fresh33 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing;
    *fresh33 = 0 as *mut layout_t;
    free_label((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label);
    agclean(
        g,
        0 as libc::c_int,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn charsetToStr(mut c: libc::c_int) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match c {
        0 => {
            s = b"UTF-8\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        1 => {
            s = b"ISO-8859-1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            s = b"BIG-5\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            agerr(
                AGERR,
                b"Unsupported charset value %d\n\0" as *const u8 as *const libc::c_char,
                c,
            );
            s = b"UTF-8\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn do_graph_label(mut sg: *mut graph_t) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut just: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos_ix: libc::c_int = 0;
    str = agget(
        sg as *mut libc::c_void,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && *str as libc::c_int != '\0' as i32 {
        let mut pos_flag: libc::c_char = 0;
        let mut dimen: pointf = pointf { x: 0., y: 0. };
        let ref mut fresh34 = (*((*((*sg).root as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .has_labels;
        *fresh34 = (*fresh34 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int)
            as libc::c_uchar;
        let ref mut fresh35 = (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .label;
        *fresh35 = make_label(
            sg as *mut libc::c_void,
            str,
            if aghtmlstr(str) != 0 {
                (1 as libc::c_int) << 1 as libc::c_int
            } else {
                (0 as libc::c_int) << 1 as libc::c_int
            },
            late_double(
                sg as *mut libc::c_void,
                agattr(
                    sg,
                    0 as libc::c_int,
                    b"fontsize\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char,
                ),
                14.0f64,
                1.0f64,
            ),
            late_nnstring(
                sg as *mut libc::c_void,
                agattr(
                    sg,
                    0 as libc::c_int,
                    b"fontname\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char,
                ),
                b"Times-Roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            late_nnstring(
                sg as *mut libc::c_void,
                agattr(
                    sg,
                    0 as libc::c_int,
                    b"fontcolor\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *const libc::c_char,
                ),
                b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        );
        pos = agget(
            sg as *mut libc::c_void,
            b"labelloc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if sg != agroot(sg as *mut libc::c_void) {
            if !pos.is_null()
                && *pos.offset(0 as libc::c_int as isize) as libc::c_int == 'b' as i32
            {
                pos_flag = 0 as libc::c_int as libc::c_char;
            } else {
                pos_flag = 1 as libc::c_int as libc::c_char;
            }
        } else if !pos.is_null()
                && *pos.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
            {
            pos_flag = 1 as libc::c_int as libc::c_char;
        } else {
            pos_flag = 0 as libc::c_int as libc::c_char;
        }
        just = agget(
            sg as *mut libc::c_void,
            b"labeljust\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !just.is_null() {
            if *just.offset(0 as libc::c_int as isize) as libc::c_int == 'l' as i32 {
                pos_flag = (pos_flag as libc::c_int | 2 as libc::c_int) as libc::c_char;
            } else if *just.offset(0 as libc::c_int as isize) as libc::c_int
                    == 'r' as i32
                {
                pos_flag = (pos_flag as libc::c_int | 4 as libc::c_int) as libc::c_char;
            }
        }
        (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos = pos_flag;
        if sg == agroot(sg as *mut libc::c_void) {
            return;
        }
        dimen = (*(*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).dimen;
        dimen.x += (4 as libc::c_int * 4 as libc::c_int) as libc::c_double;
        dimen.y += (2 as libc::c_int * 4 as libc::c_int) as libc::c_double;
        if (*((*(agroot(sg as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int == 0
        {
            if (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos
                as libc::c_int & 1 as libc::c_int != 0
            {
                pos_ix = 2 as libc::c_int;
            } else {
                pos_ix = 0 as libc::c_int;
            }
            (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[pos_ix as usize] = dimen;
        } else {
            if (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label_pos
                as libc::c_int & 1 as libc::c_int != 0
            {
                pos_ix = 1 as libc::c_int;
            } else {
                pos_ix = 3 as libc::c_int;
            }
            (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[pos_ix as usize]
                .x = dimen.y;
            (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[pos_ix as usize]
                .y = dimen.x;
        }
    }
}
