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
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    pub type dirent;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dl_iterate_phdr(
        __callback: Option<
            unsafe extern "C" fn(*mut dl_phdr_info, size_t, *mut libc::c_void) -> libc::c_int,
        >,
        __data: *mut libc::c_void,
    ) -> libc::c_int;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option<unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int>,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn gvplugin_install(
        gvc: *mut GVC_t,
        api: api_t,
        typestr: *const libc::c_char,
        quality: libc::c_int,
        package: *mut gvplugin_package_t,
        typeptr: *mut gvplugin_installed_t,
    ) -> bool;
    fn gvplugin_load(
        gvc: *mut GVC_t,
        api: api_t,
        type_0: *const libc::c_char,
    ) -> *mut gvplugin_available_t;
    fn gvplugin_library_load(gvc: *mut GVC_t, path: *mut libc::c_char) -> *mut gvplugin_library_t;
    fn gvplugin_api(str: *const libc::c_char) -> api_t;
    fn gvplugin_api_name(api: api_t) -> *mut libc::c_char;
    fn gvtextlayout_select(gvc: *mut GVC_t) -> libc::c_int;
    fn textfont_dict_open(gvc: *mut GVC_t) -> *mut Dt_t;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_0 {
    pub LL: point,
    pub UR: point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
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
pub type Elf64_Half = uint16_t;
pub type Elf64_Word = uint32_t;
pub type Elf64_Xword = uint64_t;
pub type Elf64_Addr = uint64_t;
pub type Elf64_Off = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dl_phdr_info {
    pub dlpi_addr: Elf64_Addr,
    pub dlpi_name: *const libc::c_char,
    pub dlpi_phdr: *const Elf64_Phdr,
    pub dlpi_phnum: Elf64_Half,
    pub dlpi_adds: libc::c_ulonglong,
    pub dlpi_subs: libc::c_ulonglong,
    pub dlpi_tls_modid: size_t,
    pub dlpi_tls_data: *mut libc::c_void,
}
pub type __size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int>,
}
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
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
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
unsafe extern "C" fn gvplugin_package_record(
    mut gvc: *mut GVC_t,
    mut package_path: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *mut gvplugin_package_t {
    let mut package: *mut gvplugin_package_t =
        gmalloc(::std::mem::size_of::<gvplugin_package_t>() as libc::c_ulong)
            as *mut gvplugin_package_t;
    let ref mut fresh0 = (*package).path;
    *fresh0 = if !package_path.is_null() {
        gv_strdup(package_path)
    } else {
        0 as *mut libc::c_char
    };
    let ref mut fresh1 = (*package).name;
    *fresh1 = gv_strdup(name);
    let ref mut fresh2 = (*package).next;
    *fresh2 = (*gvc).packages;
    let ref mut fresh3 = (*gvc).packages;
    *fresh3 = package;
    return package;
}
unsafe extern "C" fn separator(mut nest: *mut libc::c_int, mut tokens: *mut *mut libc::c_char) {
    let mut c: libc::c_char = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = *tokens;
    loop {
        c = *s;
        if !(c != 0) {
            break;
        }
        if c as libc::c_int == '#' as i32 {
            s = s.offset(1);
            loop {
                c = *s;
                if !(c != 0) {
                    break;
                }
                s = s.offset(1);
                if c as libc::c_int == '\n' as i32 {
                    break;
                }
            }
        } else if c as libc::c_int == '{' as i32 {
            *nest += 1;
            s = s.offset(1);
        } else if c as libc::c_int == '}' as i32 {
            *nest -= 1;
            s = s.offset(1);
        } else {
            if !(c as libc::c_int == ' ' as i32
                || c as libc::c_int == '\n' as i32
                || c as libc::c_int == '\t' as i32)
            {
                break;
            }
            s = s.offset(1);
        }
    }
    *tokens = s;
}
unsafe extern "C" fn token(
    mut nest: *mut libc::c_int,
    mut tokens: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = *tokens;
    s = t;
    loop {
        c = *s;
        if !(c != 0) {
            break;
        }
        if c as libc::c_int == '#' as i32
            || c as libc::c_int == ' ' as i32
            || c as libc::c_int == '\t' as i32
            || c as libc::c_int == '\n' as i32
            || c as libc::c_int == '{' as i32
            || c as libc::c_int == '}' as i32
        {
            break;
        }
        s = s.offset(1);
    }
    *tokens = s;
    separator(nest, tokens);
    *s = '\0' as i32 as libc::c_char;
    return t;
}
unsafe extern "C" fn gvconfig_plugin_install_from_config(
    mut gvc: *mut GVC_t,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut package_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut api: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut gv_api: api_t = API_render;
    let mut quality: libc::c_int = 0;
    let mut nest: libc::c_int = 0 as libc::c_int;
    let mut package: *mut gvplugin_package_t = 0 as *mut gvplugin_package_t;
    separator(&mut nest, &mut s);
    while *s != 0 {
        package_path = token(&mut nest, &mut s);
        if nest == 0 as libc::c_int {
            name = token(&mut nest, &mut s);
        } else {
            name = b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        package = gvplugin_package_record(gvc, package_path, name);
        loop {
            api = token(&mut nest, &mut s);
            gv_api = gvplugin_api(api);
            loop {
                if nest == 2 as libc::c_int {
                    type_0 = token(&mut nest, &mut s);
                    if nest == 2 as libc::c_int {
                        quality = atoi(token(&mut nest, &mut s));
                    } else {
                        quality = 0 as libc::c_int;
                    }
                    let mut rc: bool = gvplugin_install(
                        gvc,
                        gv_api,
                        type_0,
                        quality,
                        package,
                        0 as *mut gvplugin_installed_t,
                    );
                    if !rc {
                        agerr(
                            AGERR,
                            b"config error: %s %s %s\n\0" as *const u8 as *const libc::c_char,
                            package_path,
                            api,
                            type_0,
                        );
                        return 0 as libc::c_int;
                    }
                }
                if !(nest == 2 as libc::c_int) {
                    break;
                }
            }
            if !(nest == 1 as libc::c_int) {
                break;
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvconfig_plugin_install_from_library(
    mut gvc: *mut GVC_t,
    mut package_path: *mut libc::c_char,
    mut library: *mut gvplugin_library_t,
) {
    let mut apis: *mut gvplugin_api_t = 0 as *mut gvplugin_api_t;
    let mut types: *mut gvplugin_installed_t = 0 as *mut gvplugin_installed_t;
    let mut package: *mut gvplugin_package_t = 0 as *mut gvplugin_package_t;
    let mut i: libc::c_int = 0;
    package = gvplugin_package_record(gvc, package_path, (*library).packagename);
    apis = (*library).apis;
    loop {
        types = (*apis).types;
        if types.is_null() {
            break;
        }
        i = 0 as libc::c_int;
        while !((*types.offset(i as isize)).type_0).is_null() {
            gvplugin_install(
                gvc,
                (*apis).api,
                (*types.offset(i as isize)).type_0,
                (*types.offset(i as isize)).quality,
                package,
                &mut *types.offset(i as isize),
            );
            i += 1;
        }
        apis = apis.offset(1);
    }
}
unsafe extern "C" fn gvconfig_plugin_install_builtins(mut gvc: *mut GVC_t) {
    let mut s: *const lt_symlist_t = 0 as *const lt_symlist_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if ((*gvc).common.builtins).is_null() {
        return;
    }
    s = (*gvc).common.builtins;
    loop {
        name = (*s).name;
        if name.is_null() {
            break;
        }
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == 'g' as i32
            && !(strstr(name, b"_LTX_library\0" as *const u8 as *const libc::c_char)).is_null()
        {
            gvconfig_plugin_install_from_library(
                gvc,
                0 as *mut libc::c_char,
                (*s).address as *mut gvplugin_library_t,
            );
        }
        s = s.offset(1);
    }
}
unsafe extern "C" fn gvconfig_write_library_config(
    mut gvc: *mut GVC_t,
    mut lib_path: *mut libc::c_char,
    mut library: *mut gvplugin_library_t,
    mut f: *mut FILE,
) {
    let mut apis: *mut gvplugin_api_t = 0 as *mut gvplugin_api_t;
    let mut types: *mut gvplugin_installed_t = 0 as *mut gvplugin_installed_t;
    let mut i: libc::c_int = 0;
    fprintf(
        f,
        b"%s %s {\n\0" as *const u8 as *const libc::c_char,
        lib_path,
        (*library).packagename,
    );
    apis = (*library).apis;
    loop {
        types = (*apis).types;
        if types.is_null() {
            break;
        }
        fprintf(
            f,
            b"\t%s {\n\0" as *const u8 as *const libc::c_char,
            gvplugin_api_name((*apis).api),
        );
        i = 0 as libc::c_int;
        while !((*types.offset(i as isize)).type_0).is_null() {
            if (gvplugin_load(gvc, (*apis).api, (*types.offset(i as isize)).type_0)).is_null() {
                fprintf(f, b"#FAILS\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                f,
                b"\t\t%s %d\n\0" as *const u8 as *const libc::c_char,
                (*types.offset(i as isize)).type_0,
                (*types.offset(i as isize)).quality,
            );
            i += 1;
        }
        fputs(b"\t}\n\0" as *const u8 as *const libc::c_char, f);
        apis = apis.offset(1);
    }
    fputs(b"}\n\0" as *const u8 as *const libc::c_char, f);
}
unsafe extern "C" fn line_callback(
    mut info: *mut dl_phdr_info,
    mut size: size_t,
    mut line: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *const libc::c_char = (*info).dlpi_name;
    let mut tmp: *mut libc::c_char = strstr(p, b"/libgvc.\0" as *const u8 as *const libc::c_char);
    if !tmp.is_null() {
        *tmp = 0 as libc::c_int as libc::c_char;
        if strcmp(
            strrchr(p, '/' as i32),
            b"/.libs\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            memmove(
                line,
                p as *const libc::c_void,
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            strcat(
                line as *mut libc::c_char,
                b"/graphviz\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvconfig_libdir(mut gvc: *mut GVC_t) -> *mut libc::c_char {
    static mut line: [libc::c_char; 1024] = [0; 1024];
    static mut libdir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut dirShown: bool = 0 as libc::c_int != 0;
    if libdir.is_null() {
        libdir = getenv(b"GVBINDIR\0" as *const u8 as *const libc::c_char);
        if libdir.is_null() {
            libdir = b"/usr/local/lib/graphviz\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            dl_iterate_phdr(
                Some(
                    line_callback
                        as unsafe extern "C" fn(
                            *mut dl_phdr_info,
                            size_t,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                line.as_mut_ptr() as *mut libc::c_void,
            );
            libdir = line.as_mut_ptr();
        }
    }
    if (*gvc).common.verbose != 0 && !dirShown {
        fprintf(
            stderr,
            b"libdir = \"%s\"\n\0" as *const u8 as *const libc::c_char,
            if !libdir.is_null() {
                libdir as *const libc::c_char
            } else {
                b"<null>\0" as *const u8 as *const libc::c_char
            },
        );
        dirShown = 1 as libc::c_int != 0;
    }
    return libdir;
}
unsafe extern "C" fn is_plugin(mut filepath: *const libc::c_char) -> bool {
    if filepath.is_null() {
        return 0 as libc::c_int != 0;
    }
    static mut SUFFIX: [libc::c_char; 1] =
        unsafe { *::std::mem::transmute::<&[u8; 1], &[libc::c_char; 1]>(b"\0") };
    let mut len: size_t = strlen(filepath);
    if len < strlen(SUFFIX.as_ptr())
        || strcmp(
            filepath
                .offset(len as isize)
                .offset(-(strlen(SUFFIX.as_ptr()) as isize)),
            SUFFIX.as_ptr(),
        ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    len = (len as libc::c_ulong).wrapping_sub(strlen(SUFFIX.as_ptr())) as size_t as size_t;
    static mut VERSION: [libc::c_char; 2] =
        unsafe { *::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"6\0") };
    if len < strlen(VERSION.as_ptr())
        || strncmp(
            filepath
                .offset(len as isize)
                .offset(-(strlen(VERSION.as_ptr()) as isize)),
            VERSION.as_ptr(),
            strlen(VERSION.as_ptr()),
        ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    len = (len as libc::c_ulong).wrapping_sub(strlen(VERSION.as_ptr())) as size_t as size_t;
    static mut SO: [libc::c_char; 5] =
        unsafe { *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".so.\0") };
    if len < strlen(SO.as_ptr())
        || strncmp(
            filepath
                .offset(len as isize)
                .offset(-(strlen(SO.as_ptr()) as isize)),
            SO.as_ptr(),
            strlen(SO.as_ptr()),
        ) != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn config_rescan(mut gvc: *mut GVC_t, mut config_path: *mut libc::c_char) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut globbuf: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut config_glob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut libdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    let mut library: *mut gvplugin_library_t = 0 as *mut gvplugin_library_t;
    let mut plugin_glob: *mut libc::c_char =
        b"libgvplugin_*\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if !config_path.is_null() {
        f = fopen(config_path, b"w\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            agerr(
                AGERR,
                b"failed to open %s for write.\n\0" as *const u8 as *const libc::c_char,
                config_path,
            );
            graphviz_exit(1 as libc::c_int);
        }
        fprintf(
            f,
            b"# This file was generated by \"dot -c\" at time of install.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            f,
            b"# You may temporarily disable a plugin by removing or commenting out\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            f,
            b"# a line in this file, or you can modify its \"quality\" value to affect\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            f,
            b"# default plugin selection.\n\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            f,
            b"# Manual edits to this file **will be lost** on upgrade.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    libdir = gvconfig_libdir(gvc);
    config_glob = gmalloc(
        (strlen(libdir))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(plugin_glob))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(config_glob, libdir);
    strcat(config_glob, b"/\0" as *const u8 as *const libc::c_char);
    strcat(config_glob, plugin_glob);
    rc = glob(config_glob, 0 as libc::c_int, None, &mut globbuf);
    if rc == 0 as libc::c_int {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < globbuf.gl_pathc {
            if is_plugin(*(globbuf.gl_pathv).offset(i as isize)) {
                library = gvplugin_library_load(gvc, *(globbuf.gl_pathv).offset(i as isize));
                if !library.is_null() {
                    gvconfig_plugin_install_from_library(
                        gvc,
                        *(globbuf.gl_pathv).offset(i as isize),
                        library,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        while i_0 < globbuf.gl_pathc {
            if is_plugin(*(globbuf.gl_pathv).offset(i_0 as isize)) {
                library = gvplugin_library_load(gvc, *(globbuf.gl_pathv).offset(i_0 as isize));
                if !library.is_null() {
                    let mut p: *mut libc::c_char = strrchr(
                        *(globbuf.gl_pathv).offset(i_0 as isize),
                        (*::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"/\0"))
                            [0 as libc::c_int as usize] as libc::c_int,
                    );
                    if !p.is_null() {
                        p = p.offset(1);
                    }
                    if !f.is_null() && !p.is_null() {
                        gvconfig_write_library_config(gvc, p, library, f);
                    }
                }
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    globfree(&mut globbuf);
    free(config_glob as *mut libc::c_void);
    if !f.is_null() {
        fclose(f);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvconfig(mut gvc: *mut GVC_t, mut rescan: bool) {
    let mut rc: libc::c_int = 0;
    let mut config_st: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut libdir_st: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut config_text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut libdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut config_file_name: *mut libc::c_char =
        b"config6\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    gvconfig_plugin_install_builtins(gvc);
    (*gvc).config_found = 0 as libc::c_int != 0;
    if (*gvc).common.demand_loading != 0 {
        libdir = gvconfig_libdir(gvc);
        rc = stat(libdir, &mut libdir_st);
        if rc == -(1 as libc::c_int) {
            gvtextlayout_select(gvc);
            return;
        }
        if ((*gvc).config_path).is_null() {
            let ref mut fresh4 = (*gvc).config_path;
            *fresh4 = gmalloc(
                (strlen(libdir))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(strlen(config_file_name))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcpy((*gvc).config_path, libdir);
            strcat(
                (*gvc).config_path,
                b"/\0" as *const u8 as *const libc::c_char,
            );
            strcat((*gvc).config_path, config_file_name);
        }
        if rescan {
            config_rescan(gvc, (*gvc).config_path);
            (*gvc).config_found = 1 as libc::c_int != 0;
            gvtextlayout_select(gvc);
            return;
        }
        rc = stat((*gvc).config_path, &mut config_st);
        if rc == -(1 as libc::c_int) {
            gvtextlayout_select(gvc);
            return;
        } else {
            f = fopen(
                (*gvc).config_path,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if f.is_null() {
                agerr(
                    AGERR,
                    b"failed to open %s for read.\n\0" as *const u8 as *const libc::c_char,
                    (*gvc).config_path,
                );
                return;
            } else {
                if config_st.st_size == 0 as libc::c_int as libc::c_long {
                    agerr(
                        AGERR,
                        b"%s is zero sized.\n\0" as *const u8 as *const libc::c_char,
                        (*gvc).config_path,
                    );
                } else {
                    config_text = gmalloc(
                        (config_st.st_size as size_t)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    let mut sz: size_t = fread(
                        config_text as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        config_st.st_size as size_t,
                        f,
                    );
                    if sz == 0 as libc::c_int as libc::c_ulong {
                        agerr(
                            AGERR,
                            b"%s read error.\n\0" as *const u8 as *const libc::c_char,
                            (*gvc).config_path,
                        );
                    } else {
                        (*gvc).config_found = 1 as libc::c_int != 0;
                        *config_text.offset(sz as isize) = '\0' as i32 as libc::c_char;
                        rc = gvconfig_plugin_install_from_config(gvc, config_text);
                    }
                    free(config_text as *mut libc::c_void);
                }
            }
            if !f.is_null() {
                fclose(f);
            }
        }
    }
    gvtextlayout_select(gvc);
    textfont_dict_open(gvc);
}
