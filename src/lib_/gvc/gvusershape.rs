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
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    pub type htmllabel_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Dttree: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn safefile(filename: *const libc::c_char) -> *const libc::c_char;
    static mut Gvimagepath: *mut libc::c_char;
    static mut HTTPServerEnVar: *mut libc::c_char;
    fn find_user_shape(_: *const libc::c_char) -> *mut shape_desc;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
    pub hl: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub u: C2RustUnnamed_2,
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
    pub u: C2RustUnnamed_1,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub union C2RustUnnamed_2 {
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
    pub hh: C2RustUnnamed_3,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub graph: C2RustUnnamed_4,
    pub node: C2RustUnnamed_4,
    pub edge: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub free_layout: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_6,
    pub s: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct textlabel_t {
    pub text: *mut libc::c_char,
    pub fontname: *mut libc::c_char,
    pub fontcolor: *mut libc::c_char,
    pub charset: libc::c_int,
    pub fontsize: libc::c_double,
    pub dimen: pointf,
    pub space: pointf,
    pub pos: pointf,
    pub u: C2RustUnnamed_7,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub txt: C2RustUnnamed_8,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub span: *mut textspan_t,
    pub nspans: libc::c_short,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_t {
    pub s: *mut libc::c_char,
    pub buf: *mut libc::c_char,
    pub fp: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_t {
    pub key_start: size_t,
    pub key_extent: size_t,
    pub value_start: size_t,
    pub value_extent: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct knowntype_t {
    pub template: *mut libc::c_char,
    pub size: libc::c_int,
    pub type_0: libc::c_int,
    pub stringtype: *mut libc::c_char,
}
static mut ImageDict: *mut Dict_t = 0 as *const Dict_t as *mut Dict_t;
static mut knowntypes: [knowntype_t; 10] = [knowntype_t {
    template: 0 as *mut libc::c_char,
    size: 0,
    type_0: 0,
    stringtype: 0 as *mut libc::c_char,
}; 10];
unsafe extern "C" fn imagetype(mut us: *mut usershape_t) -> libc::c_int {
    let mut header: [libc::c_char; 20] = [0; 20];
    let mut line: [libc::c_char; 200] = [0; 200];
    if !((*us).f).is_null()
        && fread(
            header.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            20 as libc::c_int as libc::c_ulong,
            (*us).f,
        ) == 20 as libc::c_int as libc::c_ulong
    {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[knowntype_t; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<knowntype_t>() as libc::c_ulong)
        {
            if memcmp(
                header.as_mut_ptr() as *const libc::c_void,
                knowntypes[i as usize].template as *const libc::c_void,
                knowntypes[i as usize].size as libc::c_ulong,
            ) == 0
            {
                let ref mut fresh0 = (*us).stringtype;
                *fresh0 = knowntypes[i as usize].stringtype;
                (*us).type_0 = knowntypes[i as usize].type_0 as imagetype_t;
                if (*us).type_0 as libc::c_uint == FT_XML as libc::c_int as libc::c_uint {
                    while !(fgets(
                        line.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong
                            as libc::c_int,
                        (*us).f,
                    ))
                    .is_null()
                    {
                        if memcmp(
                            line.as_mut_ptr() as *const libc::c_void,
                            b"<svg\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0
                        {
                            let ref mut fresh1 = (*us).stringtype;
                            *fresh1 =
                                b"svg\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                            let ref mut fresh2 = (*us).type_0;
                            *fresh2 = FT_SVG;
                            return *fresh2 as libc::c_int;
                        }
                    }
                } else if (*us).type_0 as libc::c_uint == FT_RIFF as libc::c_int as libc::c_uint {
                    if memcmp(
                        header.as_mut_ptr().offset(8 as libc::c_int as isize)
                            as *const libc::c_void,
                        b"WEBP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                    {
                        let ref mut fresh3 = (*us).stringtype;
                        *fresh3 =
                            b"webp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        let ref mut fresh4 = (*us).type_0;
                        *fresh4 = FT_WEBP;
                        return *fresh4 as libc::c_int;
                    }
                }
                return (*us).type_0 as libc::c_int;
            }
            i = i.wrapping_add(1);
        }
    }
    let ref mut fresh5 = (*us).stringtype;
    *fresh5 = b"(lib)\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*us).type_0 = FT_NULL;
    return FT_NULL as libc::c_int;
}
unsafe extern "C" fn get_int_lsb_first(
    mut f: *mut FILE,
    mut sz: size_t,
    mut val: *mut libc::c_uint,
) -> bool {
    let mut ch: libc::c_int = 0;
    *val = 0 as libc::c_int as libc::c_uint;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < sz {
        ch = fgetc(f);
        if feof(f) != 0 {
            return 0 as libc::c_int != 0;
        }
        *val |= (ch as libc::c_uint) << (8 as libc::c_int as libc::c_ulong).wrapping_mul(i);
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_int_msb_first(
    mut f: *mut FILE,
    mut sz: size_t,
    mut val: *mut libc::c_uint,
) -> bool {
    let mut ch: libc::c_int = 0;
    *val = 0 as libc::c_int as libc::c_uint;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < sz {
        ch = fgetc(f);
        if feof(f) != 0 {
            return 0 as libc::c_int != 0;
        }
        *val <<= 8 as libc::c_int;
        *val |= ch as libc::c_uint;
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn svg_units_convert(
    mut n: libc::c_double,
    mut u: *mut libc::c_char,
) -> libc::c_uint {
    if strcmp(u, b"in\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return (if n * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double {
            (n * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            (n * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        }) as libc::c_uint;
    }
    if strcmp(u, b"px\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return (if n * 72 as libc::c_int as libc::c_double / 96 as libc::c_int as libc::c_double
            >= 0 as libc::c_int as libc::c_double
        {
            (n * 72 as libc::c_int as libc::c_double / 96 as libc::c_int as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            (n * 72 as libc::c_int as libc::c_double / 96 as libc::c_int as libc::c_double - 0.5f64)
                as libc::c_int
        }) as libc::c_uint;
    }
    if strcmp(u, b"pc\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return (if n * 72 as libc::c_int as libc::c_double / 6 as libc::c_int as libc::c_double
            >= 0 as libc::c_int as libc::c_double
        {
            (n * 72 as libc::c_int as libc::c_double / 6 as libc::c_int as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            (n * 72 as libc::c_int as libc::c_double / 6 as libc::c_int as libc::c_double - 0.5f64)
                as libc::c_int
        }) as libc::c_uint;
    }
    if strcmp(u, b"pt\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(u, b"\"\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return (if n >= 0 as libc::c_int as libc::c_double {
            (n + 0.5f64) as libc::c_int
        } else {
            (n - 0.5f64) as libc::c_int
        }) as libc::c_uint;
    }
    if strcmp(u, b"cm\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return (if n * (72 as libc::c_int as libc::c_double * 0.393700787f64)
            >= 0 as libc::c_int as libc::c_double
        {
            (n * (72 as libc::c_int as libc::c_double * 0.393700787f64) + 0.5f64) as libc::c_int
        } else {
            (n * (72 as libc::c_int as libc::c_double * 0.393700787f64) - 0.5f64) as libc::c_int
        }) as libc::c_uint;
    }
    if strcmp(u, b"mm\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return (if n * (72 as libc::c_int as libc::c_double * 0.0393700787f64)
            >= 0 as libc::c_int as libc::c_double
        {
            (n * (72 as libc::c_int as libc::c_double * 0.0393700787f64) + 0.5f64) as libc::c_int
        } else {
            (n * (72 as libc::c_int as libc::c_double * 0.0393700787f64) - 0.5f64) as libc::c_int
        }) as libc::c_uint;
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn find_attribute(
    mut s: *const libc::c_char,
    mut result: *mut match_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while *s.offset(i as isize) as libc::c_int != '\0' as i32 {
        if *s.offset(i as isize) as libc::c_int >= 'a' as i32
            && *s.offset(i as isize) as libc::c_int <= 'z' as i32
        {
            (*result).key_start = i;
            (*result).key_extent = 1 as libc::c_int as size_t;
            i = i.wrapping_add(1);
            while *s.offset(i as isize) as libc::c_int >= 'a' as i32
                && *s.offset(i as isize) as libc::c_int <= 'z' as i32
                || *s.offset(i as isize) as libc::c_int >= 'A' as i32
                    && *s.offset(i as isize) as libc::c_int <= 'Z' as i32
            {
                i = i.wrapping_add(1);
                let ref mut fresh6 = (*result).key_extent;
                *fresh6 = (*fresh6).wrapping_add(1);
            }
            if *s.offset(i as isize) as libc::c_int == '=' as i32
                && *s.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '"' as i32
            {
                i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
                (*result).value_start = i;
                (*result).value_extent = 0 as libc::c_int as size_t;
                while *s.offset(i as isize) as libc::c_int != '"' as i32
                    && *s.offset(i as isize) as libc::c_int != '\0' as i32
                {
                    i = i.wrapping_add(1);
                    let ref mut fresh7 = (*result).value_extent;
                    *fresh7 = (*fresh7).wrapping_add(1);
                }
                if *s.offset(i as isize) as libc::c_int == '"' as i32 {
                    return 0 as libc::c_int;
                }
            }
        } else {
            i = i.wrapping_add(1);
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn svg_size(mut us: *mut usershape_t) {
    let mut w: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut u: [libc::c_char; 10] = [0; 10];
    let mut attribute: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut re_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 200] = [0; 200];
    let mut wFlag: bool = 0 as libc::c_int != 0;
    let mut hFlag: bool = 0 as libc::c_int != 0;
    fseek((*us).f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    while !(fgets(
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong as libc::c_int,
        (*us).f,
    ))
    .is_null()
        && (!wFlag || !hFlag)
    {
        re_string = line.as_mut_ptr();
        let mut match_0: match_t = match_t {
            key_start: 0,
            key_extent: 0,
            value_start: 0,
            value_extent: 0,
        };
        while find_attribute(re_string, &mut match_0) == 0 as libc::c_int {
            *re_string.offset((match_0.value_start).wrapping_add(match_0.value_extent) as isize) =
                '\0' as i32 as libc::c_char;
            attribute = re_string.offset(match_0.key_start as isize);
            value = re_string.offset(match_0.value_start as isize);
            re_string = re_string.offset(
                (match_0.value_start)
                    .wrapping_add(match_0.value_extent)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
            if match_0.key_extent == strlen(b"width\0" as *const u8 as *const libc::c_char)
                && strncmp(
                    attribute,
                    b"width\0" as *const u8 as *const libc::c_char,
                    match_0.key_extent,
                ) == 0 as libc::c_int
            {
                if sscanf(
                    value,
                    b"%lf%2s\0" as *const u8 as *const libc::c_char,
                    &mut n as *mut libc::c_double,
                    u.as_mut_ptr(),
                ) == 2 as libc::c_int
                {
                    w = svg_units_convert(n, u.as_mut_ptr());
                    wFlag = 1 as libc::c_int != 0;
                } else if sscanf(
                    value,
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut n as *mut libc::c_double,
                ) == 1 as libc::c_int
                {
                    w = svg_units_convert(
                        n,
                        b"pt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    wFlag = 1 as libc::c_int != 0;
                }
                if hFlag {
                    break;
                }
            } else if match_0.key_extent == strlen(b"height\0" as *const u8 as *const libc::c_char)
                && strncmp(
                    attribute,
                    b"height\0" as *const u8 as *const libc::c_char,
                    match_0.key_extent,
                ) == 0 as libc::c_int
            {
                if sscanf(
                    value,
                    b"%lf%2s\0" as *const u8 as *const libc::c_char,
                    &mut n as *mut libc::c_double,
                    u.as_mut_ptr(),
                ) == 2 as libc::c_int
                {
                    h = svg_units_convert(n, u.as_mut_ptr());
                    hFlag = 1 as libc::c_int != 0;
                } else if sscanf(
                    value,
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut n as *mut libc::c_double,
                ) == 1 as libc::c_int
                {
                    h = svg_units_convert(
                        n,
                        b"pt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    hFlag = 1 as libc::c_int != 0;
                }
                if wFlag {
                    break;
                }
            } else {
                if !(match_0.key_extent == strlen(b"viewBox\0" as *const u8 as *const libc::c_char)
                    && strncmp(
                        attribute,
                        b"viewBox\0" as *const u8 as *const libc::c_char,
                        match_0.key_extent,
                    ) == 0 as libc::c_int
                    && sscanf(
                        value,
                        b"%lf %lf %lf %lf\0" as *const u8 as *const libc::c_char,
                        &mut x0 as *mut libc::c_double,
                        &mut y0 as *mut libc::c_double,
                        &mut x1 as *mut libc::c_double,
                        &mut y1 as *mut libc::c_double,
                    ) == 4 as libc::c_int)
                {
                    continue;
                }
                w = (x1 - x0 + 1 as libc::c_int as libc::c_double) as libc::c_uint;
                h = (y1 - y0 + 1 as libc::c_int as libc::c_double) as libc::c_uint;
                wFlag = 1 as libc::c_int != 0;
                hFlag = 1 as libc::c_int != 0;
                break;
            }
        }
    }
    (*us).dpi = 0 as libc::c_int;
    (*us).w = w as libc::c_int;
    (*us).h = h as libc::c_int;
}
unsafe extern "C" fn png_size(mut us: *mut usershape_t) {
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    (*us).dpi = 0 as libc::c_int;
    fseek((*us).f, 16 as libc::c_int as libc::c_long, 0 as libc::c_int);
    if get_int_msb_first((*us).f, 4 as libc::c_int as size_t, &mut w) as libc::c_int != 0
        && get_int_msb_first((*us).f, 4 as libc::c_int as size_t, &mut h) as libc::c_int != 0
    {
        (*us).w = w as libc::c_int;
        (*us).h = h as libc::c_int;
    }
}
unsafe extern "C" fn ico_size(mut us: *mut usershape_t) {
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    (*us).dpi = 0 as libc::c_int;
    fseek((*us).f, 6 as libc::c_int as libc::c_long, 0 as libc::c_int);
    if get_int_msb_first((*us).f, 1 as libc::c_int as size_t, &mut w) as libc::c_int != 0
        && get_int_msb_first((*us).f, 1 as libc::c_int as size_t, &mut h) as libc::c_int != 0
    {
        (*us).w = w as libc::c_int;
        (*us).h = h as libc::c_int;
    }
}
unsafe extern "C" fn webp_size(mut us: *mut usershape_t) {
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    (*us).dpi = 0 as libc::c_int;
    fseek((*us).f, 15 as libc::c_int as libc::c_long, 0 as libc::c_int);
    if fgetc((*us).f) == 'X' as i32 {
        fseek((*us).f, 24 as libc::c_int as libc::c_long, 0 as libc::c_int);
        if get_int_lsb_first((*us).f, 4 as libc::c_int as size_t, &mut w) as libc::c_int != 0
            && get_int_lsb_first((*us).f, 4 as libc::c_int as size_t, &mut h) as libc::c_int != 0
        {
            (*us).w = w as libc::c_int;
            (*us).h = h as libc::c_int;
        }
    } else {
        fseek((*us).f, 26 as libc::c_int as libc::c_long, 0 as libc::c_int);
        if get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut w) as libc::c_int != 0
            && get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut h) as libc::c_int != 0
        {
            (*us).w = w as libc::c_int;
            (*us).h = h as libc::c_int;
        }
    };
}
unsafe extern "C" fn gif_size(mut us: *mut usershape_t) {
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    (*us).dpi = 0 as libc::c_int;
    fseek((*us).f, 6 as libc::c_int as libc::c_long, 0 as libc::c_int);
    if get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut w) as libc::c_int != 0
        && get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut h) as libc::c_int != 0
    {
        (*us).w = w as libc::c_int;
        (*us).h = h as libc::c_int;
    }
}
unsafe extern "C" fn bmp_size(mut us: *mut usershape_t) {
    let mut size_x_msw: libc::c_uint = 0;
    let mut size_x_lsw: libc::c_uint = 0;
    let mut size_y_msw: libc::c_uint = 0;
    let mut size_y_lsw: libc::c_uint = 0;
    (*us).dpi = 0 as libc::c_int;
    fseek((*us).f, 16 as libc::c_int as libc::c_long, 0 as libc::c_int);
    if get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut size_x_msw) as libc::c_int != 0
        && get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut size_x_lsw) as libc::c_int
            != 0
        && get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut size_y_msw) as libc::c_int
            != 0
        && get_int_lsb_first((*us).f, 2 as libc::c_int as size_t, &mut size_y_lsw) as libc::c_int
            != 0
    {
        (*us).w = (size_x_msw << 16 as libc::c_int | size_x_lsw) as libc::c_int;
        (*us).h = (size_y_msw << 16 as libc::c_int | size_y_lsw) as libc::c_int;
    }
}
unsafe extern "C" fn jpeg_size(mut us: *mut usershape_t) {
    let mut marker: libc::c_uint = 0;
    let mut length: libc::c_uint = 0;
    let mut size_x: libc::c_uint = 0;
    let mut size_y: libc::c_uint = 0;
    static mut standalone_markers: [libc::c_uchar; 11] = [
        0x1 as libc::c_int as libc::c_uchar,
        0xd0 as libc::c_int as libc::c_uchar,
        0xd1 as libc::c_int as libc::c_uchar,
        0xd2 as libc::c_int as libc::c_uchar,
        0xd3 as libc::c_int as libc::c_uchar,
        0xd4 as libc::c_int as libc::c_uchar,
        0xd5 as libc::c_int as libc::c_uchar,
        0xd6 as libc::c_int as libc::c_uchar,
        0xd7 as libc::c_int as libc::c_uchar,
        0xd8 as libc::c_int as libc::c_uchar,
        0xd9 as libc::c_int as libc::c_uchar,
    ];
    (*us).dpi = 0 as libc::c_int;
    loop {
        if !get_int_msb_first((*us).f, 1 as libc::c_int as size_t, &mut marker) {
            return;
        }
        if marker == 0xff as libc::c_int as libc::c_uint {
            continue;
        }
        if !(memchr(
            standalone_markers.as_ptr() as *const libc::c_void,
            marker as libc::c_int,
            ::std::mem::size_of::<[libc::c_uchar; 11]>() as libc::c_ulong,
        ))
        .is_null()
        {
            continue;
        }
        if marker == 0xc0 as libc::c_int as libc::c_uint {
            if fseek((*us).f, 3 as libc::c_int as libc::c_long, 1 as libc::c_int)
                == 0 as libc::c_int
                && get_int_msb_first((*us).f, 2 as libc::c_int as size_t, &mut size_x)
                    as libc::c_int
                    != 0
                && get_int_msb_first((*us).f, 2 as libc::c_int as size_t, &mut size_y)
                    as libc::c_int
                    != 0
            {
                (*us).h = size_x as libc::c_int;
                (*us).w = size_y as libc::c_int;
            }
            return;
        }
        if marker == 0xc2 as libc::c_int as libc::c_uint {
            if fseek((*us).f, 3 as libc::c_int as libc::c_long, 1 as libc::c_int)
                != 0 as libc::c_int
            {
                return;
            }
            if get_int_msb_first((*us).f, 2 as libc::c_int as size_t, &mut size_x) as libc::c_int
                != 0
                && get_int_msb_first((*us).f, 2 as libc::c_int as size_t, &mut size_y)
                    as libc::c_int
                    != 0
            {
                (*us).h = size_x as libc::c_int;
                (*us).w = size_y as libc::c_int;
            }
            return;
        }
        if !get_int_msb_first((*us).f, 2 as libc::c_int as size_t, &mut length) {
            return;
        }
        fseek(
            (*us).f,
            length.wrapping_sub(2 as libc::c_int as libc::c_uint) as libc::c_long,
            1 as libc::c_int,
        );
    }
}
unsafe extern "C" fn ps_size(mut us: *mut usershape_t) {
    let mut line: [libc::c_char; 8192] = [0; 8192];
    let mut lx: libc::c_int = 0;
    let mut ly: libc::c_int = 0;
    let mut ux: libc::c_int = 0;
    let mut uy: libc::c_int = 0;
    let mut linep: *mut libc::c_char = 0 as *mut libc::c_char;
    (*us).dpi = 72 as libc::c_int;
    fseek((*us).f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    let mut saw_bb: bool = 0 as libc::c_int != 0;
    while !(fgets(
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
        (*us).f,
    ))
    .is_null()
    {
        linep = strstr(
            line.as_mut_ptr(),
            b"%%BoundingBox:\0" as *const u8 as *const libc::c_char,
        );
        if linep.is_null() {
            continue;
        }
        if !(sscanf(
            linep,
            b"%%%%BoundingBox: %d %d %d %d\0" as *const u8 as *const libc::c_char,
            &mut lx as *mut libc::c_int,
            &mut ly as *mut libc::c_int,
            &mut ux as *mut libc::c_int,
            &mut uy as *mut libc::c_int,
        ) == 4 as libc::c_int)
        {
            continue;
        }
        saw_bb = 1 as libc::c_int != 0;
        break;
    }
    if saw_bb {
        (*us).x = lx;
        (*us).y = ly;
        (*us).w = ux - lx;
        (*us).h = uy - ly;
    }
}
unsafe extern "C" fn nxtc(mut str: *mut stream_t) -> libc::c_uchar {
    if !(fgets((*str).buf, 8192 as libc::c_int, (*str).fp)).is_null() {
        let ref mut fresh8 = (*str).s;
        *fresh8 = (*str).buf;
        return *(*str).s as libc::c_uchar;
    }
    return '\0' as i32 as libc::c_uchar;
}
unsafe extern "C" fn skipWS(mut str: *mut stream_t) {
    let mut c: libc::c_uchar = 0;
    loop {
        c = (if *(*str).s as libc::c_int != 0 {
            *(*str).s as libc::c_int
        } else {
            nxtc(str) as libc::c_int
        }) as libc::c_uchar;
        if !(c != 0) {
            break;
        }
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let ref mut fresh9 = (*str).s;
            *fresh9 = (*fresh9).offset(1);
        } else {
            return;
        }
    }
}
unsafe extern "C" fn scanNum(
    mut tok: *mut libc::c_char,
    mut dp: *mut libc::c_double,
) -> libc::c_int {
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: libc::c_double = strtod(tok, &mut endp);
    if tok == endp {
        return 1 as libc::c_int;
    }
    *dp = d;
    return 0 as libc::c_int;
}
unsafe extern "C" fn getNum(mut str: *mut stream_t, mut buf: *mut libc::c_char) {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    skipWS(str);
    loop {
        c = (if *(*str).s as libc::c_int != 0 {
            *(*str).s as libc::c_int
        } else {
            nxtc(str) as libc::c_int
        }) as libc::c_char;
        if !(c as libc::c_int != 0
            && (*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || c as libc::c_int == '.' as i32))
        {
            break;
        }
        let fresh10 = len;
        len = len + 1;
        *buf.offset(fresh10 as isize) = c;
        let ref mut fresh11 = (*str).s;
        *fresh11 = (*fresh11).offset(1);
        if len == 8192 as libc::c_int - 1 as libc::c_int {
            break;
        }
    }
    *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn boxof(mut str: *mut stream_t, mut bp: *mut boxf) -> libc::c_int {
    let mut tok: [libc::c_char; 8192] = [0; 8192];
    skipWS(str);
    if (if *(*str).s as libc::c_int != 0 {
        *(*str).s as libc::c_int
    } else {
        nxtc(str) as libc::c_int
    }) != '[' as i32
    {
        return 1 as libc::c_int;
    }
    let ref mut fresh12 = (*str).s;
    *fresh12 = (*fresh12).offset(1);
    getNum(str, tok.as_mut_ptr());
    if scanNum(tok.as_mut_ptr(), &mut (*bp).LL.x) != 0 {
        return 1 as libc::c_int;
    }
    getNum(str, tok.as_mut_ptr());
    if scanNum(tok.as_mut_ptr(), &mut (*bp).LL.y) != 0 {
        return 1 as libc::c_int;
    }
    getNum(str, tok.as_mut_ptr());
    if scanNum(tok.as_mut_ptr(), &mut (*bp).UR.x) != 0 {
        return 1 as libc::c_int;
    }
    getNum(str, tok.as_mut_ptr());
    if scanNum(tok.as_mut_ptr(), &mut (*bp).UR.y) != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn bboxPDF(mut fp: *mut FILE, mut bp: *mut boxf) -> libc::c_int {
    let mut str: stream_t = stream_t {
        s: 0 as *mut libc::c_char,
        buf: 0 as *mut libc::c_char,
        fp: 0 as *mut FILE,
    };
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    while !(fgets(buf.as_mut_ptr(), 8192 as libc::c_int, fp)).is_null() {
        s = strstr(
            buf.as_mut_ptr(),
            b"/MediaBox\0" as *const u8 as *const libc::c_char,
        );
        if !s.is_null() {
            str.buf = buf.as_mut_ptr();
            str.s = s.offset(
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
            str.fp = fp;
            return boxof(&mut str, bp);
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pdf_size(mut us: *mut usershape_t) {
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    (*us).dpi = 0 as libc::c_int;
    fseek((*us).f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    if bboxPDF((*us).f, &mut bb) == 0 {
        (*us).x = bb.LL.x as libc::c_int;
        (*us).y = bb.LL.y as libc::c_int;
        (*us).w = (bb.UR.x - bb.LL.x) as libc::c_int;
        (*us).h = (bb.UR.y - bb.LL.y) as libc::c_int;
    }
}
unsafe extern "C" fn usershape_close(
    mut dict: *mut Dict_t,
    mut p: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) {
    let mut us: *mut usershape_t = p as *mut usershape_t;
    if !((*us).f).is_null() {
        fclose((*us).f);
    }
    if !((*us).data).is_null() && ((*us).datafree).is_some() {
        ((*us).datafree).expect("non-null function pointer")(us);
    }
    free(us as *mut libc::c_void);
}
static mut ImageDictDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0,
            makef: None,
            freef: Some(
                usershape_close
                    as unsafe extern "C" fn(*mut Dict_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn gvusershape_find(mut name: *const libc::c_char) -> *mut usershape_t {
    let mut us: *mut usershape_t = 0 as *mut usershape_t;
    if !name.is_null() {
    } else {
        __assert_fail(
            b"name\0" as *const u8 as *const libc::c_char,
            b"gvusershape.c\0" as *const u8 as *const libc::c_char,
            578 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"usershape_t *gvusershape_find(const char *)\0",
            ))
            .as_ptr(),
        );
    }
    if *name.offset(0 as libc::c_int as isize) != 0 {
    } else {
        __assert_fail(
            b"name[0]\0" as *const u8 as *const libc::c_char,
            b"gvusershape.c\0" as *const u8 as *const libc::c_char,
            579 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"usershape_t *gvusershape_find(const char *)\0",
            ))
            .as_ptr(),
        );
    }
    if ImageDict.is_null() {
        return 0 as *mut usershape_t;
    }
    us = (Some(((*(ImageDict as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        ImageDict,
        name as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut usershape_t;
    return us;
}
#[no_mangle]
pub unsafe extern "C" fn gvusershape_file_access(mut us: *mut usershape_t) -> bool {
    static mut usershape_files_open_cnt: libc::c_int = 0;
    let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
    if !us.is_null() {
    } else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvusershape.c\0" as *const u8 as *const libc::c_char,
            594 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"_Bool gvusershape_file_access(usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*us).name).is_null() {
    } else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvusershape.c\0" as *const u8 as *const libc::c_char,
            595 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"_Bool gvusershape_file_access(usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if *((*us).name).offset(0 as libc::c_int as isize) != 0 {
    } else {
        __assert_fail(
            b"us->name[0]\0" as *const u8 as *const libc::c_char,
            b"gvusershape.c\0" as *const u8 as *const libc::c_char,
            596 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"_Bool gvusershape_file_access(usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*us).f).is_null() {
        fseek((*us).f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    } else {
        fn_0 = safefile((*us).name);
        if fn_0.is_null() {
            agerr(
                AGWARN,
                b"Filename \"%s\" is unsafe\n\0" as *const u8 as *const libc::c_char,
                (*us).name,
            );
            return 0 as libc::c_int != 0;
        }
        let ref mut fresh13 = (*us).f;
        *fresh13 = fopen(fn_0, b"rb\0" as *const u8 as *const libc::c_char);
        if ((*us).f).is_null() {
            agerr(
                AGWARN,
                b"%s while opening %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
                fn_0,
            );
            return 0 as libc::c_int != 0;
        }
        if usershape_files_open_cnt >= 50 as libc::c_int {
            (*us).nocache = 1 as libc::c_int != 0;
        } else {
            usershape_files_open_cnt += 1;
        }
    }
    if !((*us).f).is_null() {
    } else {
        __assert_fail(
            b"us->f\0" as *const u8 as *const libc::c_char,
            b"gvusershape.c\0" as *const u8 as *const libc::c_char,
            615 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"_Bool gvusershape_file_access(usershape_t *)\0",
            ))
            .as_ptr(),
        );
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn gvusershape_file_release(mut us: *mut usershape_t) {
    if (*us).nocache {
        if !((*us).f).is_null() {
            fclose((*us).f);
            let ref mut fresh14 = (*us).f;
            *fresh14 = 0 as *mut FILE;
        }
    }
}
unsafe extern "C" fn freeUsershape(mut us: *mut usershape_t) {
    if !((*us).name).is_null() {
        agstrfree(0 as *mut Agraph_t, (*us).name);
    }
    free(us as *mut libc::c_void);
}
unsafe extern "C" fn gvusershape_open(mut name: *const libc::c_char) -> *mut usershape_t {
    let mut us: *mut usershape_t = 0 as *mut usershape_t;
    if !name.is_null() {
    } else {
        __assert_fail(
            b"name\0" as *const u8 as *const libc::c_char,
            b"gvusershape.c\0" as *const u8 as *const libc::c_char,
            639 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"usershape_t *gvusershape_open(const char *)\0",
            ))
            .as_ptr(),
        );
    }
    if ImageDict.is_null() {
        ImageDict = dtopen(&mut ImageDictDisc, Dttree);
    }
    us = gvusershape_find(name);
    if us.is_null() {
        us = zmalloc(::std::mem::size_of::<usershape_t>() as libc::c_ulong) as *mut usershape_t;
        let ref mut fresh15 = (*us).name;
        *fresh15 = agstrdup(0 as *mut Agraph_t, name);
        if !gvusershape_file_access(us) {
            freeUsershape(us);
            return 0 as *mut usershape_t;
        }
        if !((*us).f).is_null() {
        } else {
            __assert_fail(
                b"us->f\0" as *const u8 as *const libc::c_char,
                b"gvusershape.c\0" as *const u8 as *const libc::c_char,
                653 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"usershape_t *gvusershape_open(const char *)\0",
                ))
                .as_ptr(),
            );
        }
        match imagetype(us) {
            0 => {
                let ref mut fresh16 = (*us).data;
                *fresh16 = find_user_shape((*us).name) as *mut libc::c_void;
                if (*fresh16).is_null() {
                    agerr(
                        AGWARN,
                        b"\"%s\" was not found as a file or as a shape library member\n\0"
                            as *const u8 as *const libc::c_char,
                        (*us).name,
                    );
                    freeUsershape(us);
                    return 0 as *mut usershape_t;
                }
            }
            2 => {
                gif_size(us);
            }
            3 => {
                png_size(us);
            }
            1 => {
                bmp_size(us);
            }
            4 => {
                jpeg_size(us);
            }
            6 => {
                ps_size(us);
            }
            11 => {
                webp_size(us);
            }
            8 => {
                svg_size(us);
            }
            5 => {
                pdf_size(us);
            }
            12 => {
                ico_size(us);
            }
            7 | _ => {}
        }
        gvusershape_file_release(us);
        (Some(((*(ImageDict as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            ImageDict,
            us as *mut libc::c_void,
            0o1 as libc::c_int,
        );
        return us;
    }
    gvusershape_file_release(us);
    return us;
}
#[no_mangle]
pub unsafe extern "C" fn gvusershape_size_dpi(mut us: *mut usershape_t, mut dpi: pointf) -> point {
    let mut rv: point = point { x: 0, y: 0 };
    if us.is_null() {
        rv.y = -(1 as libc::c_int);
        rv.x = rv.y;
    } else {
        if (*us).dpi != 0 as libc::c_int {
            dpi.y = (*us).dpi as libc::c_double;
            dpi.x = dpi.y;
        }
        rv.x = (((*us).w * 72 as libc::c_int) as libc::c_double / dpi.x) as libc::c_int;
        rv.y = (((*us).h * 72 as libc::c_int) as libc::c_double / dpi.y) as libc::c_int;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn gvusershape_size(
    mut g: *mut graph_t,
    mut name: *mut libc::c_char,
) -> point {
    let mut rv: point = point { x: 0, y: 0 };
    let mut dpi: pointf = pointf { x: 0., y: 0. };
    static mut oldpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut us: *mut usershape_t = 0 as *mut usershape_t;
    if name.is_null() || *name as libc::c_int == '\0' as i32 {
        rv.y = -(1 as libc::c_int);
        rv.x = rv.y;
        return rv;
    }
    if HTTPServerEnVar.is_null() && oldpath != Gvimagepath {
        oldpath = Gvimagepath;
        if !ImageDict.is_null() {
            dtclose(ImageDict);
            ImageDict = 0 as *mut Dict_t;
        }
    }
    dpi.y = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).dpi;
    if dpi.y >= 1.0f64 {
        dpi.x = dpi.y;
    } else {
        dpi.y = 96 as libc::c_int as libc::c_double;
        dpi.x = dpi.y;
    }
    us = gvusershape_open(name);
    rv = gvusershape_size_dpi(us, dpi);
    return rv;
}
unsafe extern "C" fn run_static_initializers() {
    knowntypes = [
        {
            let mut init = knowntype_t {
                template: b"\x89PNG\r\n\x1A\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_PNG as libc::c_int,
                stringtype: b"png\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"%!PS-Adobe-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_PS as libc::c_int,
                stringtype: b"ps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"BM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_BMP as libc::c_int,
                stringtype: b"bmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"GIF8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_GIF as libc::c_int,
                stringtype: b"gif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"\xFF\xD8\xFF\xE0\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_JPEG as libc::c_int,
                stringtype: b"jpeg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"%PDF-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_PDF as libc::c_int,
                stringtype: b"pdf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"\xC5\xD0\xD3\xC6\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_EPS as libc::c_int,
                stringtype: b"eps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"<?xml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_XML as libc::c_int,
                stringtype: b"xml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"RIFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_RIFF as libc::c_int,
                stringtype: b"riff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = knowntype_t {
                template: b"\0\0\x01\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                size: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
                type_0: FT_ICO as libc::c_int,
                stringtype: b"ico\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
