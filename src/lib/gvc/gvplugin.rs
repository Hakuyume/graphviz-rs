#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type lt__handle;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn lt_dlinit() -> libc::c_int;
    fn lt_dlopen(filename: *const libc::c_char) -> lt_dlhandle;
    fn lt_dlsym(handle: lt_dlhandle, name: *const libc::c_char) -> *mut libc::c_void;
    fn lt_dlerror() -> *const libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Agdirected: Agdesc_t;
    fn gvconfig_libdir(gvc: *mut GVC_t) -> *mut libc::c_char;
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
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type lt_dlhandle = *mut lt__handle;
pub type va_list = __builtin_va_list;
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
pub type uint64_t = __uint64_t;
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
pub type Agedge_t = Agedge_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_api_t {
    pub api: api_t,
    pub types: *mut gvplugin_installed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_library_t {
    pub packagename: *mut libc::c_char,
    pub apis: *mut gvplugin_api_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strview_t {
    pub data: *const libc::c_char,
    pub size: size_t,
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
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
#[inline]
unsafe extern "C" fn gv_strndup(
    mut original: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    copy = strndup(original, length);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
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
unsafe extern "C" fn agxbprint(
    mut xb: *mut agxbuf,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut size: size_t = 0;
    let mut result: libc::c_int = 0;
    ap = args.clone();
    let mut ap2: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap2 = ap.clone();
    rc = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap2.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    size = (rc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long
        as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int agxbprint(agxbuf *, const char *, ...)\0"))
                .as_ptr(),
        );
    }
    if result > 0 as libc::c_int {
        let ref mut fresh7 = (*xb).ptr;
        *fresh7 = (*fresh7).offset(result as size_t as isize);
    }
    return result;
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
#[inline]
unsafe extern "C" fn strview(
    mut referent: *const libc::c_char,
    mut terminator: libc::c_char,
) -> strview_t {
    if !referent.is_null() {} else {
        __assert_fail(
            b"referent != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/strview.h\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"strview_t strview(const char *, char)\0"))
                .as_ptr(),
        );
    }
    let mut end: *const libc::c_char = strchr(referent, terminator as libc::c_int);
    if !end.is_null() {
        return {
            let mut init = strview_t {
                data: referent,
                size: end.offset_from(referent) as libc::c_long as size_t,
            };
            init
        };
    }
    return {
        let mut init = strview_t {
            data: referent,
            size: strlen(referent),
        };
        init
    };
}
#[inline]
unsafe extern "C" fn strview_case_eq(mut a: strview_t, mut b: strview_t) -> bool {
    if !(a.data).is_null() {} else {
        __assert_fail(
            b"a.data != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/strview.h\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"_Bool strview_case_eq(strview_t, strview_t)\0"))
                .as_ptr(),
        );
    }
    if !(b.data).is_null() {} else {
        __assert_fail(
            b"b.data != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/strview.h\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"_Bool strview_case_eq(strview_t, strview_t)\0"))
                .as_ptr(),
        );
    }
    if a.size != b.size {
        return 0 as libc::c_int != 0;
    }
    return strncasecmp(a.data, b.data, a.size) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn strview_cmp(mut a: strview_t, mut b: strview_t) -> libc::c_int {
    let mut min_size: size_t = if a.size > b.size { b.size } else { a.size };
    let mut cmp: libc::c_int = strncmp(a.data, b.data, min_size);
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    if a.size > b.size {
        return 1 as libc::c_int;
    }
    if a.size < b.size {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn strview_eq(mut a: strview_t, mut b: strview_t) -> bool {
    if !(a.data).is_null() {} else {
        __assert_fail(
            b"a.data != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/strview.h\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"_Bool strview_eq(strview_t, strview_t)\0"))
                .as_ptr(),
        );
    }
    if !(b.data).is_null() {} else {
        __assert_fail(
            b"b.data != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/strview.h\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"_Bool strview_eq(strview_t, strview_t)\0"))
                .as_ptr(),
        );
    }
    return strview_cmp(a, b) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn strview_str_eq(
    mut a: strview_t,
    mut b: *const libc::c_char,
) -> bool {
    if !(a.data).is_null() {} else {
        __assert_fail(
            b"a.data != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/strview.h\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"_Bool strview_str_eq(strview_t, const char *)\0"))
                .as_ptr(),
        );
    }
    if !b.is_null() {} else {
        __assert_fail(
            b"b != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/strview.h\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"_Bool strview_str_eq(strview_t, const char *)\0"))
                .as_ptr(),
        );
    }
    return strview_eq(a, strview(b, '\0' as i32 as libc::c_char));
}
static mut api_names: [*mut libc::c_char; 5] = [
    b"render\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"layout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"textlayout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"device\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"loadimage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn gvplugin_api(mut str: *const libc::c_char) -> api_t {
    let mut api: size_t = 0 as libc::c_int as size_t;
    while api
        < (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        if strcmp(str, api_names[api as usize]) == 0 as libc::c_int {
            return api as api_t;
        }
        api = api.wrapping_add(1);
    }
    return 4294967295 as api_t;
}
#[no_mangle]
pub unsafe extern "C" fn gvplugin_api_name(mut api: api_t) -> *mut libc::c_char {
    if api as libc::c_ulong
        >= (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        return 0 as *mut libc::c_char;
    }
    return api_names[api as usize];
}
#[no_mangle]
pub unsafe extern "C" fn gvplugin_install(
    mut gvc: *mut GVC_t,
    mut api: api_t,
    mut typestr: *const libc::c_char,
    mut quality: libc::c_int,
    mut package: *mut gvplugin_package_t,
    mut typeptr: *mut gvplugin_installed_t,
) -> bool {
    let mut plugin: *mut gvplugin_available_t = 0 as *mut gvplugin_available_t;
    let mut pnext: *mut *mut gvplugin_available_t = 0 as *mut *mut gvplugin_available_t;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = strdup(typestr);
    if t.is_null() {
        return 0 as libc::c_int != 0;
    }
    let type_0: strview_t = strview(typestr, ':' as i32 as libc::c_char);
    pnext = &mut *((*gvc).apis).as_mut_ptr().offset(api as isize)
        as *mut *mut gvplugin_available_t;
    while !(*pnext).is_null() {
        let next_type: strview_t = strview(
            (**pnext).typestr,
            ':' as i32 as libc::c_char,
        );
        if strview_cmp(type_0, next_type) <= 0 as libc::c_int {
            break;
        }
        pnext = &mut (**pnext).next;
    }
    while !(*pnext).is_null() {
        let next_type_0: strview_t = strview(
            (**pnext).typestr,
            ':' as i32 as libc::c_char,
        );
        if !strview_eq(type_0, next_type_0) {
            break;
        }
        if quality >= (**pnext).quality {
            break;
        }
        pnext = &mut (**pnext).next;
    }
    plugin = gmalloc(::std::mem::size_of::<gvplugin_available_t>() as libc::c_ulong)
        as *mut gvplugin_available_t;
    let ref mut fresh11 = (*plugin).next;
    *fresh11 = *pnext;
    *pnext = plugin;
    let ref mut fresh12 = (*plugin).typestr;
    *fresh12 = t;
    (*plugin).quality = quality;
    let ref mut fresh13 = (*plugin).package;
    *fresh13 = package;
    let ref mut fresh14 = (*plugin).typeptr;
    *fresh14 = typeptr;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn gvplugin_activate(
    mut gvc: *mut GVC_t,
    mut api: api_t,
    mut typestr: *const libc::c_char,
    mut name: *const libc::c_char,
    mut plugin_path: *const libc::c_char,
    mut typeptr: *mut gvplugin_installed_t,
) {
    let mut pnext: *mut gvplugin_available_t = 0 as *mut gvplugin_available_t;
    pnext = (*gvc).apis[api as usize];
    while !pnext.is_null() {
        if strcasecmp(typestr, (*pnext).typestr) == 0 as libc::c_int
            && strcasecmp(name, (*(*pnext).package).name) == 0 as libc::c_int
            && !((*(*pnext).package).path).is_null()
            && strcasecmp(plugin_path, (*(*pnext).package).path) == 0 as libc::c_int
        {
            let ref mut fresh15 = (*pnext).typeptr;
            *fresh15 = typeptr;
            return;
        }
        pnext = (*pnext).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvplugin_library_load(
    mut gvc: *mut GVC_t,
    mut path: *mut libc::c_char,
) -> *mut gvplugin_library_t {
    let mut hndl: lt_dlhandle = 0 as *mut lt__handle;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sym: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    static mut p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut lenp: size_t = 0;
    let mut libdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut suffix: *mut libc::c_char = b"_LTX_library\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if (*gvc).common.demand_loading == 0 {
        return 0 as *mut gvplugin_library_t;
    }
    libdir = gvconfig_libdir(gvc);
    len = (strlen(libdir))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(path))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if len > lenp {
        lenp = len.wrapping_add(20 as libc::c_int as libc::c_ulong);
        p = grealloc(p as *mut libc::c_void, lenp) as *mut libc::c_char;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        strcpy(p, path);
    } else {
        strcpy(p, libdir);
        strcat(p, b"/\0" as *const u8 as *const libc::c_char);
        strcat(p, path);
    }
    if lt_dlinit() != 0 {
        agerr(AGERR, b"failed to init libltdl\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut gvplugin_library_t;
    }
    hndl = lt_dlopen(p);
    if hndl.is_null() {
        if stat(p, &mut sb) == 0 as libc::c_int {
            agerr(
                AGWARN,
                b"Could not load \"%s\" - %s\n\0" as *const u8 as *const libc::c_char,
                p,
                b"It was found, so perhaps one of its dependents was not.  Try ldd.\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            agerr(
                AGWARN,
                b"Could not load \"%s\" - %s\n\0" as *const u8 as *const libc::c_char,
                p,
                lt_dlerror(),
            );
        }
        return 0 as *mut gvplugin_library_t;
    }
    if (*gvc).common.verbose >= 2 as libc::c_int {
        fprintf(stderr, b"Loading %s\n\0" as *const u8 as *const libc::c_char, p);
    }
    s = strrchr(
        p,
        (*::std::mem::transmute::<
            &[u8; 2],
            &[libc::c_char; 2],
        >(b"/\0"))[0 as libc::c_int as usize] as libc::c_int,
    );
    len = strlen(s);
    if len < strlen(b"/libgvplugin_x\0" as *const u8 as *const libc::c_char) {
        agerr(
            AGERR,
            b"invalid plugin path \"%s\"\n\0" as *const u8 as *const libc::c_char,
            p,
        );
        return 0 as *mut gvplugin_library_t;
    }
    sym = gmalloc(
        len.wrapping_add(strlen(suffix)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(sym, s.offset(4 as libc::c_int as isize));
    s = strchr(sym, '.' as i32);
    strcpy(s, suffix);
    ptr = lt_dlsym(hndl, sym);
    if ptr.is_null() {
        agerr(
            AGERR,
            b"failed to resolve %s in %s\n\0" as *const u8 as *const libc::c_char,
            sym,
            p,
        );
        free(sym as *mut libc::c_void);
        return 0 as *mut gvplugin_library_t;
    }
    free(sym as *mut libc::c_void);
    return ptr as *mut gvplugin_library_t;
}
#[no_mangle]
pub unsafe extern "C" fn gvplugin_load(
    mut gvc: *mut GVC_t,
    mut api: api_t,
    mut str: *const libc::c_char,
) -> *mut gvplugin_available_t {
    let mut pnext: *mut gvplugin_available_t = 0 as *mut gvplugin_available_t;
    let mut rv: *mut gvplugin_available_t = 0 as *mut gvplugin_available_t;
    let mut library: *mut gvplugin_library_t = 0 as *mut gvplugin_library_t;
    let mut apis: *mut gvplugin_api_t = 0 as *mut gvplugin_api_t;
    let mut types: *mut gvplugin_installed_t = 0 as *mut gvplugin_installed_t;
    let mut i: libc::c_int = 0;
    let mut apidep: api_t = API_render;
    if api as libc::c_uint == API_device as libc::c_int as libc::c_uint
        || api as libc::c_uint == API_loadimage as libc::c_int as libc::c_uint
    {
        apidep = API_render;
    } else {
        apidep = api;
    }
    let reqtyp: strview_t = strview(str, ':' as i32 as libc::c_char);
    let mut reqdep: strview_t = {
        let mut init = strview_t {
            data: 0 as *const libc::c_char,
            size: 0,
        };
        init
    };
    let mut reqpkg: strview_t = {
        let mut init = strview_t {
            data: 0 as *const libc::c_char,
            size: 0,
        };
        init
    };
    if *(reqtyp.data).offset(reqtyp.size as isize) as libc::c_int == ':' as i32 {
        reqdep = strview(
            (reqtyp.data)
                .offset(reqtyp.size as isize)
                .offset(strlen(b":\0" as *const u8 as *const libc::c_char) as isize),
            ':' as i32 as libc::c_char,
        );
        if *(reqdep.data).offset(reqdep.size as isize) as libc::c_int == ':' as i32 {
            reqpkg = strview(
                (reqdep.data)
                    .offset(reqdep.size as isize)
                    .offset(strlen(b":\0" as *const u8 as *const libc::c_char) as isize),
                '\0' as i32 as libc::c_char,
            );
        }
    }
    let mut current_block_12: u64;
    pnext = (*gvc).apis[api as usize];
    while !pnext.is_null() {
        let typ: strview_t = strview((*pnext).typestr, ':' as i32 as libc::c_char);
        let mut dep: strview_t = {
            let mut init = strview_t {
                data: 0 as *const libc::c_char,
                size: 0,
            };
            init
        };
        if *(typ.data).offset(typ.size as isize) as libc::c_int == ':' as i32 {
            dep = strview(
                (typ.data)
                    .offset(typ.size as isize)
                    .offset(strlen(b":\0" as *const u8 as *const libc::c_char) as isize),
                '\0' as i32 as libc::c_char,
            );
        }
        if strview_eq(typ, reqtyp) {
            if !(dep.data).is_null() && !(reqdep.data).is_null() {
                if !strview_eq(dep, reqdep) {
                    current_block_12 = 12349973810996921269;
                } else {
                    current_block_12 = 11194104282611034094;
                }
            } else {
                current_block_12 = 11194104282611034094;
            }
            match current_block_12 {
                12349973810996921269 => {}
                _ => {
                    if (reqpkg.data).is_null()
                        || strview_str_eq(reqpkg, (*(*pnext).package).name)
                            as libc::c_int != 0
                    {
                        if !(!(dep.data).is_null()
                            && apidep as libc::c_uint != api as libc::c_uint)
                        {
                            break;
                        }
                        if !(gvplugin_load(gvc, apidep, dep.data)).is_null() {
                            break;
                        }
                    }
                }
            }
        }
        pnext = (*pnext).next;
    }
    rv = pnext;
    if !rv.is_null() && ((*rv).typeptr).is_null() {
        library = gvplugin_library_load(gvc, (*(*rv).package).path);
        if !library.is_null() {
            apis = (*library).apis;
            loop {
                types = (*apis).types;
                if types.is_null() {
                    break;
                }
                i = 0 as libc::c_int;
                while !((*types.offset(i as isize)).type_0).is_null() {
                    gvplugin_activate(
                        gvc,
                        (*apis).api,
                        (*types.offset(i as isize)).type_0,
                        (*library).packagename,
                        (*(*rv).package).path,
                        &mut *types.offset(i as isize),
                    );
                    i += 1;
                }
                apis = apis.offset(1);
            }
            if (*gvc).common.verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Activated plugin library: %s\n\0" as *const u8
                        as *const libc::c_char,
                    if !((*(*rv).package).path).is_null() {
                        (*(*rv).package).path as *const libc::c_char
                    } else {
                        b"<builtin>\0" as *const u8 as *const libc::c_char
                    },
                );
            }
        }
    }
    if !rv.is_null() && ((*rv).typeptr).is_null() {
        rv = 0 as *mut gvplugin_available_t;
    }
    if !rv.is_null() && (*gvc).common.verbose >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"Using %s: %s:%s\n\0" as *const u8 as *const libc::c_char,
            api_names[api as usize],
            (*rv).typestr,
            (*(*rv).package).name,
        );
    }
    let ref mut fresh16 = (*gvc).api[api as usize];
    *fresh16 = rv;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn gvplugin_list(
    mut gvc: *mut GVC_t,
    mut api: api_t,
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    static mut first: libc::c_int = 1 as libc::c_int;
    let mut pnext: *const gvplugin_available_t = 0 as *const gvplugin_available_t;
    let mut plugin: *const gvplugin_available_t = 0 as *const gvplugin_available_t;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new: bool = 1 as libc::c_int != 0;
    static mut xb: agxbuf = agxbuf {
        buf: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        eptr: 0 as *const libc::c_char as *mut libc::c_char,
        dyna: 0,
    };
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    if first != 0 {
        agxbinit(&mut xb, 0 as libc::c_int as libc::c_uint, 0 as *mut libc::c_char);
        first = 0 as libc::c_int;
    }
    let strv: strview_t = strview(str, ':' as i32 as libc::c_char);
    plugin = (*gvc).apis[api as usize];
    if *(strv.data).offset(strv.size as isize) as libc::c_int == ':' as i32 {
        pnext = plugin;
        while !pnext.is_null() {
            let type_0: strview_t = strview(
                (*pnext).typestr,
                ':' as i32 as libc::c_char,
            );
            if *str.offset(0 as libc::c_int as isize) == 0
                || strview_case_eq(strv, type_0) as libc::c_int != 0
            {
                agxbprint(
                    &mut xb as *mut agxbuf,
                    b" %s:%s\0" as *const u8 as *const libc::c_char,
                    (*pnext).typestr,
                    (*(*pnext).package).name,
                );
                new = 0 as libc::c_int != 0;
            }
            pnext = (*pnext).next;
        }
    }
    if new {
        let mut type_last: strview_t = {
            let mut init = strview_t {
                data: 0 as *const libc::c_char,
                size: 0,
            };
            init
        };
        pnext = plugin;
        while !pnext.is_null() {
            let type_1: strview_t = strview(
                (*pnext).typestr,
                ':' as i32 as libc::c_char,
            );
            if (type_last.data).is_null() || !strview_case_eq(type_last, type_1) {
                agxbprint(
                    &mut xb as *mut agxbuf,
                    b" %.*s\0" as *const u8 as *const libc::c_char,
                    type_1.size as libc::c_int,
                    type_1.data,
                );
                new = 0 as libc::c_int != 0;
            }
            type_last = type_1;
            pnext = (*pnext).next;
        }
    }
    if new {
        bp = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        bp = agxbuse(&mut xb);
    }
    return bp;
}
#[no_mangle]
pub unsafe extern "C" fn gvPluginList(
    mut gvc: *mut GVC_t,
    mut kind: *const libc::c_char,
    mut sz: *mut libc::c_int,
    mut str: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut api: libc::c_int = 0;
    let mut pnext: *const gvplugin_available_t = 0 as *const gvplugin_available_t;
    let mut plugin: *const gvplugin_available_t = 0 as *const gvplugin_available_t;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if kind.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    api = 0 as libc::c_int;
    while (api as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        if strcasecmp(kind, api_names[api as usize]) == 0 {
            break;
        }
        api += 1;
    }
    if api as libc::c_ulong
        == (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        agerr(
            AGERR,
            b"unrecognized api name \"%s\"\n\0" as *const u8 as *const libc::c_char,
            kind,
        );
        return 0 as *mut *mut libc::c_char;
    }
    plugin = (*gvc).apis[api as usize];
    let mut typestr_last: strview_t = {
        let mut init = strview_t {
            data: 0 as *const libc::c_char,
            size: 0,
        };
        init
    };
    pnext = plugin;
    while !pnext.is_null() {
        let mut q: strview_t = strview((*pnext).typestr, ':' as i32 as libc::c_char);
        if (typestr_last.data).is_null() || !strview_case_eq(typestr_last, q) {
            list = grealloc(
                list as *mut libc::c_void,
                ((cnt + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            let fresh17 = cnt;
            cnt = cnt + 1;
            let ref mut fresh18 = *list.offset(fresh17 as isize);
            *fresh18 = gv_strndup(q.data, q.size);
        }
        typestr_last = q;
        pnext = (*pnext).next;
    }
    *sz = cnt;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn gvplugin_write_status(mut gvc: *mut GVC_t) {
    let mut api: libc::c_int = 0;
    if (*gvc).common.demand_loading != 0 {
        fprintf(
            stderr,
            b"The plugin configuration file:\n\t%s\n\0" as *const u8
                as *const libc::c_char,
            (*gvc).config_path,
        );
        if (*gvc).config_found {
            fprintf(
                stderr,
                b"\t\twas successfully loaded.\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(
                stderr,
                b"\t\twas not found or not usable. No on-demand plugins.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else {
        fprintf(
            stderr,
            b"Demand loading of plugins is disabled.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    api = 0 as libc::c_int;
    while (api as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        if (*gvc).common.verbose >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"    %s\t: %s\n\0" as *const u8 as *const libc::c_char,
                api_names[api as usize],
                gvplugin_list(
                    gvc,
                    api as api_t,
                    b":\0" as *const u8 as *const libc::c_char,
                ),
            );
        } else {
            fprintf(
                stderr,
                b"    %s\t: %s\n\0" as *const u8 as *const libc::c_char,
                api_names[api as usize],
                gvplugin_list(
                    gvc,
                    api as api_t,
                    b"?\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        api += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvplugin_graph(mut gvc: *mut GVC_t) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut ssg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut m: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut loadimage_n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut renderer_n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut device_n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut textlayout_n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut layout_n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut a: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut package: *mut gvplugin_package_t = 0 as *mut gvplugin_package_t;
    let mut pnext: *const gvplugin_available_t = 0 as *const gvplugin_available_t;
    let mut bufa: [libc::c_char; 100] = [0; 100];
    let mut buf1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufb: [libc::c_char; 100] = [0; 100];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lq: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut api: libc::c_int = 0;
    let mut neededge_loadimage: libc::c_int = 0;
    let mut neededge_device: libc::c_int = 0;
    g = agopen(
        b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agdirected,
        0 as *mut Agdisc_t,
    );
    agattr(
        g,
        0 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        0 as libc::c_int,
        b"rankdir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        0 as libc::c_int,
        b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        0 as libc::c_int,
        b"ranksep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        1 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\\N\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        1 as libc::c_int,
        b"shape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        1 as libc::c_int,
        b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        1 as libc::c_int,
        b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    agattr(
        g,
        2 as libc::c_int,
        b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    a = agattr(
        g,
        0 as libc::c_int,
        b"rankdir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    agxset(g as *mut libc::c_void, a, b"LR\0" as *const u8 as *const libc::c_char);
    a = agattr(
        g,
        0 as libc::c_int,
        b"ranksep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    agxset(g as *mut libc::c_void, a, b"2.5\0" as *const u8 as *const libc::c_char);
    a = agattr(
        g,
        0 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    agxset(g as *mut libc::c_void, a, b"Plugins\0" as *const u8 as *const libc::c_char);
    package = (*gvc).packages;
    while !package.is_null() {
        layout_n = 0 as *mut Agnode_t;
        textlayout_n = layout_n;
        device_n = textlayout_n;
        renderer_n = device_n;
        loadimage_n = renderer_n;
        neededge_device = 0 as libc::c_int;
        neededge_loadimage = neededge_device;
        strcpy(bufa.as_mut_ptr(), b"cluster_\0" as *const u8 as *const libc::c_char);
        strcat(bufa.as_mut_ptr(), (*package).name);
        sg = agsubg(g, bufa.as_mut_ptr(), 1 as libc::c_int);
        a = agattr(
            sg,
            0 as libc::c_int,
            b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
        agxset(sg as *mut libc::c_void, a, (*package).name);
        strcpy(bufa.as_mut_ptr(), (*package).name);
        strcat(bufa.as_mut_ptr(), b"_\0" as *const u8 as *const libc::c_char);
        buf1 = bufa.as_mut_ptr().offset(strlen(bufa.as_mut_ptr()) as isize);
        api = 0 as libc::c_int;
        while (api as libc::c_ulong)
            < (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                )
        {
            strcpy(buf1, api_names[api as usize]);
            ssg = agsubg(sg, bufa.as_mut_ptr(), 1 as libc::c_int);
            a = agattr(
                ssg,
                0 as libc::c_int,
                b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            );
            agxset(
                ssg as *mut libc::c_void,
                a,
                b"same\0" as *const u8 as *const libc::c_char,
            );
            strcat(buf1, b"_\0" as *const u8 as *const libc::c_char);
            buf2 = bufa.as_mut_ptr().offset(strlen(bufa.as_mut_ptr()) as isize);
            pnext = (*gvc).apis[api as usize];
            while !pnext.is_null() {
                if (*pnext).package == package {
                    q = gv_strdup((*pnext).typestr);
                    t = q;
                    p = strchr(q, ':' as i32);
                    if !p.is_null() {
                        let fresh19 = p;
                        p = p.offset(1);
                        *fresh19 = '\0' as i32 as libc::c_char;
                    }
                    match api {
                        3 | 4 => {
                            lq = q;
                            if strncmp(
                                q,
                                b"jp\0" as *const u8 as *const libc::c_char,
                                2 as libc::c_int as libc::c_ulong,
                            ) == 0
                            {
                                q = b"jpg\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                                lq = b"jpeg\\njpe\\njpg\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char;
                            } else if strncmp(
                                    q,
                                    b"tif\0" as *const u8 as *const libc::c_char,
                                    3 as libc::c_int as libc::c_ulong,
                                ) == 0
                                {
                                q = b"tif\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                                lq = b"tiff\\ntif\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            } else if strcmp(
                                    q,
                                    b"x11\0" as *const u8 as *const libc::c_char,
                                ) == 0
                                    || strcmp(q, b"xlib\0" as *const u8 as *const libc::c_char)
                                        == 0
                                {
                                q = b"x11\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                                lq = b"x11\\nxlib\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            } else if strcmp(
                                    q,
                                    b"dot\0" as *const u8 as *const libc::c_char,
                                ) == 0
                                    || strcmp(q, b"gv\0" as *const u8 as *const libc::c_char)
                                        == 0
                                {
                                q = b"gv\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                                lq = b"gv\\ndot\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            strcpy(buf2, q);
                            n = agnode(ssg, bufa.as_mut_ptr(), 1 as libc::c_int);
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"label\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            agxset(n as *mut libc::c_void, a, lq);
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"width\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            agxset(
                                n as *mut libc::c_void,
                                a,
                                b"1.0\0" as *const u8 as *const libc::c_char,
                            );
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"shape\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            if api == API_device as libc::c_int {
                                agxset(
                                    n as *mut libc::c_void,
                                    a,
                                    b"box\0" as *const u8 as *const libc::c_char,
                                );
                                device_n = n;
                            } else {
                                agxset(
                                    n as *mut libc::c_void,
                                    a,
                                    b"box\0" as *const u8 as *const libc::c_char,
                                );
                                loadimage_n = n;
                            }
                            if !(!p.is_null() && *p as libc::c_int != 0) {
                                strcpy(
                                    bufb.as_mut_ptr(),
                                    b"render_cg\0" as *const u8 as *const libc::c_char,
                                );
                                m = agnode(sg, bufb.as_mut_ptr(), 0 as libc::c_int);
                                if m.is_null() {
                                    m = agnode(sg, bufb.as_mut_ptr(), 1 as libc::c_int);
                                    a = agattr(
                                        g,
                                        0 as libc::c_int,
                                        b"label\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        0 as *const libc::c_char,
                                    );
                                    agxset(
                                        m as *mut libc::c_void,
                                        a,
                                        b"cg\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                agedge(sg, m, n, 0 as *mut libc::c_char, 1 as libc::c_int);
                            }
                        }
                        0 => {
                            strcpy(bufb.as_mut_ptr(), api_names[api as usize]);
                            strcat(
                                bufb.as_mut_ptr(),
                                b"_\0" as *const u8 as *const libc::c_char,
                            );
                            strcat(bufb.as_mut_ptr(), q);
                            n = agnode(ssg, bufb.as_mut_ptr(), 1 as libc::c_int);
                            renderer_n = n;
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"label\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            agxset(n as *mut libc::c_void, a, q);
                        }
                        2 => {
                            strcpy(bufb.as_mut_ptr(), api_names[api as usize]);
                            strcat(
                                bufb.as_mut_ptr(),
                                b"_\0" as *const u8 as *const libc::c_char,
                            );
                            strcat(bufb.as_mut_ptr(), q);
                            n = agnode(ssg, bufb.as_mut_ptr(), 1 as libc::c_int);
                            textlayout_n = n;
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"shape\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            agxset(
                                n as *mut libc::c_void,
                                a,
                                b"invtriangle\0" as *const u8 as *const libc::c_char,
                            );
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"label\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            agxset(
                                n as *mut libc::c_void,
                                a,
                                b"T\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        1 => {
                            strcpy(bufb.as_mut_ptr(), api_names[api as usize]);
                            strcat(
                                bufb.as_mut_ptr(),
                                b"_\0" as *const u8 as *const libc::c_char,
                            );
                            strcat(bufb.as_mut_ptr(), q);
                            n = agnode(ssg, bufb.as_mut_ptr(), 1 as libc::c_int);
                            layout_n = n;
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"shape\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            agxset(
                                n as *mut libc::c_void,
                                a,
                                b"hexagon\0" as *const u8 as *const libc::c_char,
                            );
                            a = agattr(
                                g,
                                1 as libc::c_int,
                                b"label\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *const libc::c_char,
                            );
                            agxset(n as *mut libc::c_void, a, q);
                        }
                        _ => {}
                    }
                    free(t as *mut libc::c_void);
                }
                pnext = (*pnext).next;
            }
            if api == API_loadimage as libc::c_int && loadimage_n.is_null() {
                neededge_loadimage = 1 as libc::c_int;
                strcpy(buf2, b"invis\0" as *const u8 as *const libc::c_char);
                n = agnode(ssg, bufa.as_mut_ptr(), 1 as libc::c_int);
                loadimage_n = n;
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"invis\0" as *const u8 as *const libc::c_char,
                );
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"1.0\0" as *const u8 as *const libc::c_char,
                );
                strcpy(buf2, b"invis_src\0" as *const u8 as *const libc::c_char);
                n = agnode(g, bufa.as_mut_ptr(), 1 as libc::c_int);
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"invis\0" as *const u8 as *const libc::c_char,
                );
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                e = agedge(g, n, loadimage_n, 0 as *mut libc::c_char, 1 as libc::c_int);
                a = agattr(
                    g,
                    2 as libc::c_int,
                    b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    e as *mut libc::c_void,
                    a,
                    b"invis\0" as *const u8 as *const libc::c_char,
                );
            }
            if api == API_render as libc::c_int && renderer_n.is_null() {
                neededge_loadimage = 1 as libc::c_int;
                neededge_device = 1 as libc::c_int;
                strcpy(buf2, b"invis\0" as *const u8 as *const libc::c_char);
                n = agnode(ssg, bufa.as_mut_ptr(), 1 as libc::c_int);
                renderer_n = n;
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"invis\0" as *const u8 as *const libc::c_char,
                );
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            if api == API_device as libc::c_int && device_n.is_null() {
                neededge_device = 1 as libc::c_int;
                strcpy(buf2, b"invis\0" as *const u8 as *const libc::c_char);
                n = agnode(ssg, bufa.as_mut_ptr(), 1 as libc::c_int);
                device_n = n;
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"invis\0" as *const u8 as *const libc::c_char,
                );
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                a = agattr(
                    g,
                    1 as libc::c_int,
                    b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *const libc::c_char,
                );
                agxset(
                    n as *mut libc::c_void,
                    a,
                    b"1.0\0" as *const u8 as *const libc::c_char,
                );
            }
            api += 1;
        }
        if neededge_loadimage != 0 {
            e = agedge(
                sg,
                loadimage_n,
                renderer_n,
                0 as *mut libc::c_char,
                1 as libc::c_int,
            );
            a = agattr(
                g,
                2 as libc::c_int,
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            );
            agxset(
                e as *mut libc::c_void,
                a,
                b"invis\0" as *const u8 as *const libc::c_char,
            );
        }
        if neededge_device != 0 {
            e = agedge(
                sg,
                renderer_n,
                device_n,
                0 as *mut libc::c_char,
                1 as libc::c_int,
            );
            a = agattr(
                g,
                2 as libc::c_int,
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            );
            agxset(
                e as *mut libc::c_void,
                a,
                b"invis\0" as *const u8 as *const libc::c_char,
            );
        }
        if !textlayout_n.is_null() {
            e = agedge(
                sg,
                loadimage_n,
                textlayout_n,
                0 as *mut libc::c_char,
                1 as libc::c_int,
            );
            a = agattr(
                g,
                2 as libc::c_int,
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            );
            agxset(
                e as *mut libc::c_void,
                a,
                b"invis\0" as *const u8 as *const libc::c_char,
            );
        }
        if !layout_n.is_null() {
            e = agedge(
                sg,
                loadimage_n,
                layout_n,
                0 as *mut libc::c_char,
                1 as libc::c_int,
            );
            a = agattr(
                g,
                2 as libc::c_int,
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            );
            agxset(
                e as *mut libc::c_void,
                a,
                b"invis\0" as *const u8 as *const libc::c_char,
            );
        }
        package = (*package).next;
    }
    ssg = agsubg(
        g,
        b"output_formats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    a = agattr(
        ssg,
        0 as libc::c_int,
        b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    agxset(ssg as *mut libc::c_void, a, b"same\0" as *const u8 as *const libc::c_char);
    package = (*gvc).packages;
    while !package.is_null() {
        strcpy(bufa.as_mut_ptr(), (*package).name);
        strcat(bufa.as_mut_ptr(), b"_\0" as *const u8 as *const libc::c_char);
        buf1 = bufa.as_mut_ptr().offset(strlen(bufa.as_mut_ptr()) as isize);
        api = 0 as libc::c_int;
        while (api as libc::c_ulong)
            < (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                )
        {
            strcpy(buf1, api_names[api as usize]);
            strcat(buf1, b"_\0" as *const u8 as *const libc::c_char);
            buf2 = bufa.as_mut_ptr().offset(strlen(bufa.as_mut_ptr()) as isize);
            pnext = (*gvc).apis[api as usize];
            while !pnext.is_null() {
                if (*pnext).package == package {
                    q = gv_strdup((*pnext).typestr);
                    t = q;
                    p = strchr(q, ':' as i32);
                    if !p.is_null() {
                        let fresh20 = p;
                        p = p.offset(1);
                        *fresh20 = '\0' as i32 as libc::c_char;
                    }
                    lq = q;
                    if strncmp(
                        q,
                        b"jp\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) == 0
                    {
                        q = b"jpg\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        lq = b"jpeg\\njpe\\njpg\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else if strncmp(
                            q,
                            b"tif\0" as *const u8 as *const libc::c_char,
                            3 as libc::c_int as libc::c_ulong,
                        ) == 0
                        {
                        q = b"tif\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        lq = b"tiff\\ntif\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else if strcmp(q, b"x11\0" as *const u8 as *const libc::c_char)
                            == 0
                            || strcmp(q, b"xlib\0" as *const u8 as *const libc::c_char)
                                == 0
                        {
                        q = b"x11\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        lq = b"x11\\nxlib\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else if strcmp(q, b"dot\0" as *const u8 as *const libc::c_char)
                            == 0
                            || strcmp(q, b"gv\0" as *const u8 as *const libc::c_char)
                                == 0
                        {
                        q = b"gv\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        lq = b"gv\\ndot\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    match api {
                        3 => {
                            strcpy(buf2, q);
                            n = agnode(g, bufa.as_mut_ptr(), 1 as libc::c_int);
                            strcpy(
                                bufb.as_mut_ptr(),
                                b"output_\0" as *const u8 as *const libc::c_char,
                            );
                            strcat(bufb.as_mut_ptr(), q);
                            m = agnode(ssg, bufb.as_mut_ptr(), 0 as libc::c_int);
                            if m.is_null() {
                                m = agnode(ssg, bufb.as_mut_ptr(), 1 as libc::c_int);
                                a = agattr(
                                    g,
                                    1 as libc::c_int,
                                    b"label\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                agxset(m as *mut libc::c_void, a, lq);
                                a = agattr(
                                    g,
                                    1 as libc::c_int,
                                    b"shape\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                agxset(
                                    m as *mut libc::c_void,
                                    a,
                                    b"note\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            e = agedge(
                                g,
                                n,
                                m,
                                0 as *mut libc::c_char,
                                0 as libc::c_int,
                            );
                            if e.is_null() {
                                e = agedge(
                                    g,
                                    n,
                                    m,
                                    0 as *mut libc::c_char,
                                    1 as libc::c_int,
                                );
                            }
                            if !p.is_null() && *p as libc::c_int != 0 {
                                strcpy(
                                    bufb.as_mut_ptr(),
                                    b"render_\0" as *const u8 as *const libc::c_char,
                                );
                                strcat(bufb.as_mut_ptr(), p);
                                m = agnode(ssg, bufb.as_mut_ptr(), 0 as libc::c_int);
                                if m.is_null() {
                                    m = agnode(g, bufb.as_mut_ptr(), 1 as libc::c_int);
                                }
                                e = agedge(
                                    g,
                                    m,
                                    n,
                                    0 as *mut libc::c_char,
                                    0 as libc::c_int,
                                );
                                if e.is_null() {
                                    e = agedge(
                                        g,
                                        m,
                                        n,
                                        0 as *mut libc::c_char,
                                        1 as libc::c_int,
                                    );
                                }
                            }
                        }
                        4 => {
                            strcpy(buf2, q);
                            n = agnode(g, bufa.as_mut_ptr(), 1 as libc::c_int);
                            strcpy(
                                bufb.as_mut_ptr(),
                                b"input_\0" as *const u8 as *const libc::c_char,
                            );
                            strcat(bufb.as_mut_ptr(), q);
                            m = agnode(g, bufb.as_mut_ptr(), 0 as libc::c_int);
                            if m.is_null() {
                                m = agnode(g, bufb.as_mut_ptr(), 1 as libc::c_int);
                                a = agattr(
                                    g,
                                    1 as libc::c_int,
                                    b"label\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                agxset(m as *mut libc::c_void, a, lq);
                                a = agattr(
                                    g,
                                    1 as libc::c_int,
                                    b"shape\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    0 as *const libc::c_char,
                                );
                                agxset(
                                    m as *mut libc::c_void,
                                    a,
                                    b"note\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            e = agedge(
                                g,
                                m,
                                n,
                                0 as *mut libc::c_char,
                                0 as libc::c_int,
                            );
                            if e.is_null() {
                                e = agedge(
                                    g,
                                    m,
                                    n,
                                    0 as *mut libc::c_char,
                                    1 as libc::c_int,
                                );
                            }
                            strcpy(
                                bufb.as_mut_ptr(),
                                b"render_\0" as *const u8 as *const libc::c_char,
                            );
                            strcat(bufb.as_mut_ptr(), p);
                            m = agnode(g, bufb.as_mut_ptr(), 0 as libc::c_int);
                            if m.is_null() {
                                m = agnode(g, bufb.as_mut_ptr(), 1 as libc::c_int);
                            }
                            e = agedge(
                                g,
                                n,
                                m,
                                0 as *mut libc::c_char,
                                0 as libc::c_int,
                            );
                            if e.is_null() {
                                e = agedge(
                                    g,
                                    n,
                                    m,
                                    0 as *mut libc::c_char,
                                    1 as libc::c_int,
                                );
                            }
                        }
                        _ => {}
                    }
                    free(t as *mut libc::c_void);
                }
                pnext = (*pnext).next;
            }
            api += 1;
        }
        package = (*package).next;
    }
    return g;
}
