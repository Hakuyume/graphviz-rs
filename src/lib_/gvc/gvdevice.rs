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
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type internal_state;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    fn deflateBound(strm: z_streamp, sourceLen: uLong) -> uLong;
    fn crc32(crc_0: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn deflateInit2_(
        strm: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ::std::ffi::VaList)
        -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn xml_escape(
        s: *const libc::c_char,
        flags: xml_flags_t,
        cb: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
        state: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
static mut z_file_header: [libc::c_uchar; 10] = [
    0x1f as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
];
static mut z_strm: z_stream = z_stream {
    next_in: 0 as *const Bytef as *mut Bytef,
    avail_in: 0,
    total_in: 0,
    next_out: 0 as *const Bytef as *mut Bytef,
    avail_out: 0,
    total_out: 0,
    msg: 0 as *const libc::c_char as *mut libc::c_char,
    state: 0 as *const internal_state as *mut internal_state,
    zalloc: None,
    zfree: None,
    opaque: 0 as *const libc::c_void as *mut libc::c_void,
    data_type: 0,
    adler: 0,
    reserved: 0,
};
static mut df: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut dfallocated: libc::c_uint = 0;
static mut crc: uint64_t = 0;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut PAGE_ALIGN: libc::c_int = 4095 as libc::c_int;
unsafe extern "C" fn gvwrite_no_z(
    mut job: *mut GVJ_t,
    mut s: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    if ((*(*job).gvc).write_fn).is_some() {
        return ((*(*job).gvc).write_fn).expect("non-null function pointer")(
            job,
            s as *const libc::c_char,
            len,
        );
    }
    if !((*job).output_data).is_null() {
        if len
            > ((*job).output_data_allocated).wrapping_sub(
                ((*job).output_data_position).wrapping_add(1 as libc::c_int as libc::c_uint),
            ) as libc::c_ulong
        {
            (*job).output_data_allocated = (((*job).output_data_position as libc::c_ulong)
                .wrapping_add(len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(PAGE_ALIGN as libc::c_ulong)
                & !PAGE_ALIGN as libc::c_ulong)
                as libc::c_uint;
            let ref mut fresh0 = (*job).output_data;
            *fresh0 = realloc(
                (*job).output_data as *mut libc::c_void,
                (*job).output_data_allocated as libc::c_ulong,
            ) as *mut libc::c_char;
            if ((*job).output_data).is_null() {
                ((*(*job).common).errorfn).expect("non-null function pointer")(
                    b"memory allocation failure\n\0" as *const u8 as *const libc::c_char,
                );
                graphviz_exit(1 as libc::c_int);
            }
        }
        memcpy(
            ((*job).output_data).offset((*job).output_data_position as isize) as *mut libc::c_void,
            s,
            len,
        );
        let ref mut fresh1 = (*job).output_data_position;
        *fresh1 = (*fresh1 as libc::c_ulong).wrapping_add(len) as libc::c_uint as libc::c_uint;
        *((*job).output_data).offset((*job).output_data_position as isize) =
            '\0' as i32 as libc::c_char;
        return len;
    } else {
        if !((*job).output_file).is_null() {
        } else {
            __assert_fail(
                b"job->output_file != NULL\0" as *const u8 as *const libc::c_char,
                b"gvdevice.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"size_t gvwrite_no_z(GVJ_t *, const void *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        return fwrite(
            s,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            len,
            (*job).output_file,
        );
    };
}
unsafe extern "C" fn auto_output_filename(mut job: *mut GVJ_t) {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: size_t = 0;
    let mut gidx: [libc::c_char; 100] = [0; 100];
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if (*job).graph_index != 0 {
        snprintf(
            gidx.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            b".%d\0" as *const u8 as *const libc::c_char,
            (*job).graph_index + 1 as libc::c_int,
        );
    } else {
        gidx[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    fn_0 = (*job).input_filename;
    if fn_0.is_null() {
        fn_0 = b"noname.gv\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    len = (strlen(fn_0))
        .wrapping_add(strlen(gidx.as_mut_ptr()))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen((*job).output_langname))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if bufsz < len {
        bufsz = len.wrapping_add(10 as libc::c_int as libc::c_ulong);
        buf = realloc(
            buf as *mut libc::c_void,
            bufsz.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    strcpy(buf, fn_0);
    strcat(buf, gidx.as_mut_ptr());
    strcat(buf, b".\0" as *const u8 as *const libc::c_char);
    p = strdup((*job).output_langname);
    loop {
        q = strrchr(p, ':' as i32);
        if q.is_null() {
            break;
        }
        strcat(buf, q.offset(1 as libc::c_int as isize));
        strcat(buf, b".\0" as *const u8 as *const libc::c_char);
        *q = '\0' as i32 as libc::c_char;
    }
    strcat(buf, p);
    free(p as *mut libc::c_void);
    let ref mut fresh2 = (*job).output_filename;
    *fresh2 = buf;
}
#[no_mangle]
pub unsafe extern "C" fn gvdevice_initialize(mut job: *mut GVJ_t) -> libc::c_int {
    let mut gvde: *mut gvdevice_engine_t = (*job).device.engine;
    let mut gvc: *mut GVC_t = (*job).gvc;
    if !gvde.is_null() && ((*gvde).initialize).is_some() {
        ((*gvde).initialize).expect("non-null function pointer")(job);
    } else if ((*job).output_data).is_null() {
        if ((*job).output_file).is_null() {
            if (*gvc).common.auto_outfile_names {
                auto_output_filename(job);
            }
            if !((*job).output_filename).is_null() {
                let ref mut fresh3 = (*job).output_file;
                *fresh3 = fopen(
                    (*job).output_filename,
                    b"w\0" as *const u8 as *const libc::c_char,
                );
                if ((*job).output_file).is_null() {
                    ((*(*job).common).errorfn).expect("non-null function pointer")(
                        b"Could not open \"%s\" for writing : %s\n\0" as *const u8
                            as *const libc::c_char,
                        (*job).output_filename,
                        strerror(*__errno_location()),
                    );
                    return 1 as libc::c_int;
                }
            } else {
                let ref mut fresh4 = (*job).output_file;
                *fresh4 = stdout;
            }
        }
    }
    if (*job).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        let mut z: *mut z_stream = &mut z_strm;
        let ref mut fresh5 = (*z).zalloc;
        *fresh5 = None;
        let ref mut fresh6 = (*z).zfree;
        *fresh6 = None;
        let ref mut fresh7 = (*z).opaque;
        *fresh7 = 0 as voidpf;
        let ref mut fresh8 = (*z).next_in;
        *fresh8 = 0 as *mut Bytef;
        let ref mut fresh9 = (*z).next_out;
        *fresh9 = 0 as *mut Bytef;
        (*z).avail_in = 0 as libc::c_int as uInt;
        crc = crc32(
            0 as libc::c_long as uLong,
            0 as *const Bytef,
            0 as libc::c_int as uInt,
        );
        if deflateInit2_(
            z,
            -(1 as libc::c_int),
            8 as libc::c_int,
            -(15 as libc::c_int),
            9 as libc::c_int,
            0 as libc::c_int,
            b"1.2.12\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        ) != 0 as libc::c_int
        {
            ((*(*job).common).errorfn).expect("non-null function pointer")(
                b"Error initializing for deflation\n\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        gvwrite_no_z(
            job,
            z_file_header.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 10]>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvwrite(
    mut job: *mut GVJ_t,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut ret: size_t = 0;
    let mut olen: size_t = 0;
    if len == 0 || s.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if (*job).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        let mut z: z_streamp = &mut z_strm;
        let mut dflen: size_t = 0;
        dflen = deflateBound(z, len);
        if (dfallocated as libc::c_ulong) < dflen {
            dfallocated = (dflen
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(PAGE_ALIGN as libc::c_ulong)
                & !PAGE_ALIGN as libc::c_ulong) as libc::c_uint;
            df = realloc(df as *mut libc::c_void, dfallocated as libc::c_ulong)
                as *mut libc::c_uchar;
            if df.is_null() {
                ((*(*job).common).errorfn).expect("non-null function pointer")(
                    b"memory allocation failure\n\0" as *const u8 as *const libc::c_char,
                );
                graphviz_exit(1 as libc::c_int);
            }
        }
        crc = crc32(crc, s as *const libc::c_uchar, len as uInt);
        let ref mut fresh10 = (*z).next_in;
        *fresh10 = s as *mut libc::c_uchar;
        (*z).avail_in = len as uInt;
        while (*z).avail_in != 0 {
            let ref mut fresh11 = (*z).next_out;
            *fresh11 = df;
            (*z).avail_out = dfallocated;
            let mut r: libc::c_int = deflate(z, 0 as libc::c_int);
            if r != 0 as libc::c_int {
                ((*(*job).common).errorfn).expect("non-null function pointer")(
                    b"deflation problem %d\n\0" as *const u8 as *const libc::c_char,
                    r,
                );
                graphviz_exit(1 as libc::c_int);
            }
            olen = ((*z).next_out).offset_from(df) as libc::c_long as size_t;
            if olen != 0 {
                ret = gvwrite_no_z(job, df as *const libc::c_void, olen);
                if ret != olen {
                    ((*(*job).common).errorfn).expect("non-null function pointer")(
                        b"gvwrite_no_z problem %d\n\0" as *const u8 as *const libc::c_char,
                        ret,
                    );
                    graphviz_exit(1 as libc::c_int);
                }
            }
        }
    } else {
        ret = gvwrite_no_z(job, s as *const libc::c_void, len);
        if ret != len {
            ((*(*job).common).errorfn).expect("non-null function pointer")(
                b"gvwrite_no_z problem %d\n\0" as *const u8 as *const libc::c_char,
                len,
            );
            graphviz_exit(1 as libc::c_int);
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn gvferror(mut stream: *mut FILE) -> libc::c_int {
    let mut job: *mut GVJ_t = stream as *mut GVJ_t;
    if ((*(*job).gvc).write_fn).is_none() && ((*job).output_data).is_null() {
        return ferror((*job).output_file);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvputs(mut job: *mut GVJ_t, mut s: *const libc::c_char) -> libc::c_int {
    let mut len: size_t = strlen(s);
    if gvwrite(job, s, len) != len {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvputs_xml(
    mut job: *mut GVJ_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let flags: xml_flags_t = {
        let mut init = xml_flags_t {
            raw_dash_nbsp_utf8: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_raw(0);
        init.set_dash(1 as libc::c_int as libc::c_uint);
        init.set_nbsp(1 as libc::c_int as libc::c_uint);
        init.set_utf8(0);
        init
    };
    return xml_escape(
        s,
        flags,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int>,
            Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
        >(Some(
            gvputs as unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int,
        )),
        job as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gvputs_nonascii(mut job: *mut GVJ_t, mut s: *const libc::c_char) {
    while *s as libc::c_int != '\0' as i32 {
        if *s as libc::c_int == '\\' as i32 {
            gvputs(job, b"\\\\\0" as *const u8 as *const libc::c_char);
        } else if *s as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int {
            gvputc(job, *s as libc::c_int);
        } else {
            gvprintf(
                job,
                b"%03o\0" as *const u8 as *const libc::c_char,
                *s as libc::c_uint,
            );
        }
        s = s.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvputc(mut job: *mut GVJ_t, mut c: libc::c_int) -> libc::c_int {
    let cc: libc::c_char = c as libc::c_char;
    if gvwrite(job, &cc, 1 as libc::c_int as size_t) != 1 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn gvflush(mut job: *mut GVJ_t) -> libc::c_int {
    if !((*job).output_file).is_null()
        && !(*job).external_context
        && ((*(*job).gvc).write_fn).is_none()
    {
        return fflush((*job).output_file);
    } else {
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn gvdevice_close(mut job: *mut GVJ_t) {
    if !((*job).output_filename).is_null()
        && (*job).output_file != stdout
        && !(*job).external_context
    {
        if !((*job).output_file).is_null() {
            fclose((*job).output_file);
            let ref mut fresh12 = (*job).output_file;
            *fresh12 = 0 as *mut FILE;
        }
        let ref mut fresh13 = (*job).output_filename;
        *fresh13 = 0 as *const libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvdevice_format(mut job: *mut GVJ_t) {
    let mut gvde: *mut gvdevice_engine_t = (*job).device.engine;
    if !gvde.is_null() && ((*gvde).format).is_some() {
        ((*gvde).format).expect("non-null function pointer")(job);
    }
    gvflush(job);
}
#[no_mangle]
pub unsafe extern "C" fn gvdevice_finalize(mut job: *mut GVJ_t) {
    let mut gvde: *mut gvdevice_engine_t = (*job).device.engine;
    let mut finalized_p: bool = 0 as libc::c_int != 0;
    if (*job).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        let mut z: z_streamp = &mut z_strm;
        let mut out: [libc::c_uchar; 8] =
            *::std::mem::transmute::<&[u8; 8], &mut [libc::c_uchar; 8]>(b"\0\0\0\0\0\0\0\0");
        let mut ret: libc::c_int = 0;
        let mut cnt: libc::c_int = 0 as libc::c_int;
        let ref mut fresh14 = (*z).next_in;
        *fresh14 = out.as_mut_ptr();
        (*z).avail_in = 0 as libc::c_int as uInt;
        let ref mut fresh15 = (*z).next_out;
        *fresh15 = df;
        (*z).avail_out = dfallocated;
        loop {
            ret = deflate(z, 4 as libc::c_int);
            if !(ret == 0 as libc::c_int && {
                let fresh16 = cnt;
                cnt = cnt + 1;
                fresh16 <= 100 as libc::c_int
            }) {
                break;
            }
            gvwrite_no_z(
                job,
                df as *const libc::c_void,
                ((*z).next_out).offset_from(df) as libc::c_long as size_t,
            );
            let ref mut fresh17 = (*z).next_out;
            *fresh17 = df;
            (*z).avail_out = dfallocated;
        }
        if ret != 1 as libc::c_int {
            ((*(*job).common).errorfn).expect("non-null function pointer")(
                b"deflation finish problem %d cnt=%d\n\0" as *const u8 as *const libc::c_char,
                ret,
                cnt,
            );
            graphviz_exit(1 as libc::c_int);
        }
        gvwrite_no_z(
            job,
            df as *const libc::c_void,
            ((*z).next_out).offset_from(df) as libc::c_long as size_t,
        );
        ret = deflateEnd(z);
        if ret != 0 as libc::c_int {
            ((*(*job).common).errorfn).expect("non-null function pointer")(
                b"deflation end problem %d\n\0" as *const u8 as *const libc::c_char,
                ret,
            );
            graphviz_exit(1 as libc::c_int);
        }
        out[0 as libc::c_int as usize] = crc as libc::c_uchar;
        out[1 as libc::c_int as usize] = (crc >> 8 as libc::c_int) as libc::c_uchar;
        out[2 as libc::c_int as usize] = (crc >> 16 as libc::c_int) as libc::c_uchar;
        out[3 as libc::c_int as usize] = (crc >> 24 as libc::c_int) as libc::c_uchar;
        out[4 as libc::c_int as usize] = (*z).total_in as libc::c_uchar;
        out[5 as libc::c_int as usize] = ((*z).total_in >> 8 as libc::c_int) as libc::c_uchar;
        out[6 as libc::c_int as usize] = ((*z).total_in >> 16 as libc::c_int) as libc::c_uchar;
        out[7 as libc::c_int as usize] = ((*z).total_in >> 24 as libc::c_int) as libc::c_uchar;
        gvwrite_no_z(
            job,
            out.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
        );
    }
    if !gvde.is_null() {
        if ((*gvde).finalize).is_some() {
            ((*gvde).finalize).expect("non-null function pointer")(job);
            finalized_p = 1 as libc::c_int != 0;
        }
    }
    if !finalized_p {
        gvflush(job);
        gvdevice_close(job);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvprintf(
    mut job: *mut GVJ_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut len: libc::c_int = 0;
    let mut argp: ::std::ffi::VaListImpl;
    let mut bp: *mut libc::c_char = buf.as_mut_ptr();
    argp = args.clone();
    let mut argp2: ::std::ffi::VaListImpl;
    argp2 = argp.clone();
    len = vsnprintf(
        buf.as_mut_ptr(),
        8192 as libc::c_int as libc::c_ulong,
        format,
        argp2.as_va_list(),
    );
    if len < 0 as libc::c_int {
        agerr(
            AGERR,
            b"gvprintf: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return;
    } else {
        if len >= 8192 as libc::c_int {
            bp = gmalloc((len as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            len = vsprintf(bp, format, argp.as_va_list());
        }
    }
    gvwrite(job, bp, len as size_t);
    if bp != buf.as_mut_ptr() {
        free(bp as *mut libc::c_void);
    }
}
static mut maxnegnumstr: [libc::c_char; 20] = unsafe {
    *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(b"-999999999999999.99\0")
};
static mut maxnegnum: libc::c_double = -999999999999999.99f64;
unsafe extern "C" fn gvprintnum(
    mut len: *mut size_t,
    mut number: libc::c_double,
) -> *mut libc::c_char {
    static mut tmpbuf: [libc::c_char; 20] = [0; 20];
    let mut result: *mut libc::c_char = tmpbuf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as isize);
    let mut N: libc::c_long = 0;
    let mut showzeros: bool = false;
    let mut negative: bool = false;
    let mut digit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if number < maxnegnum {
        *len = (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return maxnegnumstr.as_mut_ptr();
    }
    if number > -maxnegnum {
        *len = (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong);
        return maxnegnumstr.as_mut_ptr().offset(1 as libc::c_int as isize);
    }
    number *= 10000 as libc::c_int as libc::c_double;
    if number < 0.0f64 {
        N = (number - 0.5f64) as libc::c_long;
    } else {
        N = (number + 0.5f64) as libc::c_long;
    }
    if N == 0 as libc::c_int as libc::c_long {
        *len = 1 as libc::c_int as size_t;
        return b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    negative = N < 0 as libc::c_int as libc::c_long;
    if negative {
        N = -N;
    }
    showzeros = 0 as libc::c_int != 0;
    i = 4 as libc::c_int;
    while N != 0 || i > 0 as libc::c_int {
        digit = (N % 10 as libc::c_long) as libc::c_int;
        N /= 10 as libc::c_int as libc::c_long;
        if digit != 0 || showzeros as libc::c_int != 0 {
            result = result.offset(-1);
            *result = (digit as libc::c_char as libc::c_int | '0' as i32) as libc::c_char;
            showzeros = 1 as libc::c_int != 0;
        }
        if i == 1 as libc::c_int {
            if showzeros {
                result = result.offset(-1);
                *result = '.' as i32 as libc::c_char;
            }
            showzeros = 1 as libc::c_int != 0;
        }
        i -= 1;
    }
    if negative {
        result = result.offset(-1);
        *result = '-' as i32 as libc::c_char;
    }
    *len = tmpbuf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as isize)
        .offset_from(result) as libc::c_long as size_t;
    return result;
}
unsafe extern "C" fn gv_trim_zeros(mut buf: *mut libc::c_char) {
    let mut dotp: *mut libc::c_char = strchr(buf, '.' as i32);
    if dotp.is_null() {
        return;
    }
    if *(*__ctype_b_loc()).offset(*dotp.offset(1 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
        && *(*__ctype_b_loc())
            .offset(*dotp.offset(2 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        && *dotp.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
    } else {
        __assert_fail(
            b"isdigit((int)dotp[1]) && isdigit((int)dotp[2]) && dotp[3] == '\\0'\0" as *const u8
                as *const libc::c_char,
            b"gvdevice.c\0" as *const u8 as *const libc::c_char,
            548 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void gv_trim_zeros(char *)\0",
            ))
            .as_ptr(),
        );
    }
    if *dotp.offset(2 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        if *dotp.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32 {
            *dotp = '\0' as i32 as libc::c_char;
        } else {
            *dotp.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvprintdouble(mut job: *mut GVJ_t, mut num: libc::c_double) {
    if num > -0.005f64 && num < 0.005f64 {
        gvwrite(
            job,
            b"0\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        return;
    }
    let mut buf: [libc::c_char; 50] = [0; 50];
    snprintf(
        buf.as_mut_ptr(),
        50 as libc::c_int as libc::c_ulong,
        b"%.02f\0" as *const u8 as *const libc::c_char,
        num,
    );
    gv_trim_zeros(buf.as_mut_ptr());
    gvwrite(job, buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
}
#[no_mangle]
pub unsafe extern "C" fn gvprintpointf(mut job: *mut GVJ_t, mut p: pointf) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    buf = gvprintnum(&mut len, p.x);
    gvwrite(job, buf, len);
    gvwrite(
        job,
        b" \0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    buf = gvprintnum(&mut len, p.y);
    gvwrite(job, buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn gvprintpointflist(
    mut job: *mut GVJ_t,
    mut p: *mut pointf,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        gvprintpointf(job, *p.offset(i as isize));
        i += 1;
        if i >= n {
            break;
        }
        gvwrite(
            job,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
    }
}
