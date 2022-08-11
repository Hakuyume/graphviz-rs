#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type GVC_s;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn gvusershape_file_access(us: *mut usershape_t) -> bool;
    fn gvusershape_file_release(us: *mut usershape_t);
    fn epsf_emit_body(job: *mut GVJ_t, us: *mut usershape_t);
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn core_loadimage_xdot(_: *mut GVJ_t, _: *mut usershape_t, _: boxf, _: bool);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvloadimage_engine_s {
    pub loadimage: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut usershape_t, boxf, bool) -> (),
    >,
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
    pub datafree: Option::<unsafe extern "C" fn(*mut usershape_t) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_5,
    pub s: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub initfn: Option::<unsafe extern "C" fn(*mut node_t) -> ()>,
    pub freefn: Option::<unsafe extern "C" fn(*mut node_t) -> ()>,
    pub portfn: Option::<
        unsafe extern "C" fn(*mut node_t, *mut libc::c_char, *mut libc::c_char) -> port,
    >,
    pub insidefn: Option::<unsafe extern "C" fn(*mut inside_t, pointf) -> bool>,
    pub pboxfn: Option::<
        unsafe extern "C" fn(
            *mut node_t,
            *mut port,
            libc::c_int,
            *mut boxf,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub codefn: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> ()>,
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const FORMAT_GIF_TK: C2RustUnnamed_6 = 30;
pub const FORMAT_JPEG_VML: C2RustUnnamed_6 = 29;
pub const FORMAT_GIF_VML: C2RustUnnamed_6 = 28;
pub const FORMAT_PNG_VML: C2RustUnnamed_6 = 27;
pub const FORMAT_PSLIB_PS: C2RustUnnamed_6 = 26;
pub const FORMAT_PS_PS: C2RustUnnamed_6 = 25;
pub const FORMAT_JPEG_VRML: C2RustUnnamed_6 = 24;
pub const FORMAT_GIF_VRML: C2RustUnnamed_6 = 23;
pub const FORMAT_PNG_VRML: C2RustUnnamed_6 = 22;
pub const FORMAT_JPEG_FIG: C2RustUnnamed_6 = 21;
pub const FORMAT_GIF_FIG: C2RustUnnamed_6 = 20;
pub const FORMAT_PNG_FIG: C2RustUnnamed_6 = 19;
pub const FORMAT_SVG_SVG: C2RustUnnamed_6 = 18;
pub const FORMAT_JPEG_SVG: C2RustUnnamed_6 = 17;
pub const FORMAT_GIF_SVG: C2RustUnnamed_6 = 16;
pub const FORMAT_PNG_SVG: C2RustUnnamed_6 = 15;
pub const FORMAT_PS_MAP: C2RustUnnamed_6 = 14;
pub const FORMAT_SVG_MAP: C2RustUnnamed_6 = 13;
pub const FORMAT_JPEG_MAP: C2RustUnnamed_6 = 12;
pub const FORMAT_GIF_MAP: C2RustUnnamed_6 = 11;
pub const FORMAT_PNG_MAP: C2RustUnnamed_6 = 10;
pub const FORMAT_PS_DOT: C2RustUnnamed_6 = 9;
pub const FORMAT_SVG_DOT: C2RustUnnamed_6 = 8;
pub const FORMAT_JPEG_DOT: C2RustUnnamed_6 = 7;
pub const FORMAT_GIF_DOT: C2RustUnnamed_6 = 6;
pub const FORMAT_PNG_DOT: C2RustUnnamed_6 = 5;
pub const FORMAT_PS_XDOT: C2RustUnnamed_6 = 4;
pub const FORMAT_SVG_XDOT: C2RustUnnamed_6 = 3;
pub const FORMAT_JPEG_XDOT: C2RustUnnamed_6 = 2;
pub const FORMAT_GIF_XDOT: C2RustUnnamed_6 = 1;
pub const FORMAT_PNG_XDOT: C2RustUnnamed_6 = 0;
unsafe extern "C" fn core_loadimage_svg(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut width: libc::c_double = b.UR.x - b.LL.x;
    let mut height: libc::c_double = b.UR.y - b.LL.y;
    let mut originx: libc::c_double = (b.UR.x + b.LL.x - width)
        / 2 as libc::c_int as libc::c_double;
    let mut originy: libc::c_double = (b.UR.y + b.LL.y + height)
        / 2 as libc::c_int as libc::c_double;
    if !job.is_null() {} else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void core_loadimage_svg(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !us.is_null() {} else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void core_loadimage_svg(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !((*us).name).is_null() {} else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void core_loadimage_svg(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    gvputs(job, b"<image xlink:href=\"\0" as *const u8 as *const libc::c_char);
    gvputs(job, (*us).name);
    if (*job).rotation != 0 {
        gvprintf(
            job,
            b"\" width=\"%gpx\" height=\"%gpx\" preserveAspectRatio=\"xMidYMid meet\" x=\"%g\" y=\"%g\"\0"
                as *const u8 as *const libc::c_char,
            height,
            width,
            originx,
            -originy,
        );
        gvprintf(
            job,
            b" transform=\"rotate(%d %g %g)\"\0" as *const u8 as *const libc::c_char,
            (*job).rotation,
            originx,
            -originy,
        );
    } else {
        gvprintf(
            job,
            b"\" width=\"%gpx\" height=\"%gpx\" preserveAspectRatio=\"xMinYMin meet\" x=\"%g\" y=\"%g\"\0"
                as *const u8 as *const libc::c_char,
            width,
            height,
            originx,
            -originy,
        );
    }
    gvputs(job, b"/>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn core_loadimage_fig(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut bf: boxf,
    mut filled: bool,
) {
    let mut object_code: libc::c_int = 2 as libc::c_int;
    let mut sub_type: libc::c_int = 5 as libc::c_int;
    let mut line_style: libc::c_int = 0 as libc::c_int;
    let mut thickness: libc::c_int = 0 as libc::c_int;
    let mut pen_color: libc::c_int = 0 as libc::c_int;
    let mut fill_color: libc::c_int = -(1 as libc::c_int);
    let mut depth: libc::c_int = 1 as libc::c_int;
    let mut pen_style: libc::c_int = -(1 as libc::c_int);
    let mut area_fill: libc::c_int = 0 as libc::c_int;
    let mut style_val: libc::c_double = 0.0f64;
    let mut join_style: libc::c_int = 0 as libc::c_int;
    let mut cap_style: libc::c_int = 0 as libc::c_int;
    let mut radius: libc::c_int = 0 as libc::c_int;
    let mut forward_arrow: libc::c_int = 0 as libc::c_int;
    let mut backward_arrow: libc::c_int = 0 as libc::c_int;
    let mut npoints: libc::c_int = 5 as libc::c_int;
    let mut flipped: libc::c_int = 0 as libc::c_int;
    let mut b: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    if !job.is_null() {} else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void core_loadimage_fig(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !us.is_null() {} else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void core_loadimage_fig(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !((*us).name).is_null() {} else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"void core_loadimage_fig(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    b
        .LL
        .x = (if bf.LL.x >= 0 as libc::c_int as libc::c_double {
        (bf.LL.x + 0.5f64) as libc::c_int
    } else {
        (bf.LL.x - 0.5f64) as libc::c_int
    });
    b
        .LL
        .y = (if bf.LL.y >= 0 as libc::c_int as libc::c_double {
        (bf.LL.y + 0.5f64) as libc::c_int
    } else {
        (bf.LL.y - 0.5f64) as libc::c_int
    });
    b
        .UR
        .x = (if bf.UR.x >= 0 as libc::c_int as libc::c_double {
        (bf.UR.x + 0.5f64) as libc::c_int
    } else {
        (bf.UR.x - 0.5f64) as libc::c_int
    });
    b
        .UR
        .y = (if bf.UR.y >= 0 as libc::c_int as libc::c_double {
        (bf.UR.y + 0.5f64) as libc::c_int
    } else {
        (bf.UR.y - 0.5f64) as libc::c_int
    });
    gvprintf(
        job,
        b"%d %d %d %d %d %d %d %d %d %.1f %d %d %d %d %d %d\n %d %s\n\0" as *const u8
            as *const libc::c_char,
        object_code,
        sub_type,
        line_style,
        thickness,
        pen_color,
        fill_color,
        depth,
        pen_style,
        area_fill,
        style_val,
        join_style,
        cap_style,
        radius,
        forward_arrow,
        backward_arrow,
        npoints,
        flipped,
        (*us).name,
    );
    gvprintf(
        job,
        b" %d %d %d %d %d %d %d %d %d %d\n\0" as *const u8 as *const libc::c_char,
        b.LL.x,
        b.LL.y,
        b.LL.x,
        b.UR.y,
        b.UR.x,
        b.UR.y,
        b.UR.x,
        b.LL.y,
        b.LL.x,
        b.LL.y,
    );
}
unsafe extern "C" fn core_loadimage_vrml(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    if !job.is_null() {} else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void core_loadimage_vrml(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    obj = (*job).obj;
    if !obj.is_null() {} else {
        __assert_fail(
            b"obj\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void core_loadimage_vrml(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !us.is_null() {} else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void core_loadimage_vrml(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !((*us).name).is_null() {} else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void core_loadimage_vrml(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    n = (*(*job).obj).u.n;
    if !n.is_null() {} else {
        __assert_fail(
            b"n\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"void core_loadimage_vrml(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    gvprintf(job, b"Shape {\n\0" as *const u8 as *const libc::c_char);
    gvprintf(job, b"  appearance Appearance {\n\0" as *const u8 as *const libc::c_char);
    gvprintf(job, b"    material Material {\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"      ambientIntensity 0.33\n\0" as *const u8 as *const libc::c_char,
    );
    gvprintf(job, b"        diffuseColor 1 1 1\n\0" as *const u8 as *const libc::c_char);
    gvprintf(job, b"    }\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"    texture ImageTexture { url \"%s\" }\n\0" as *const u8
            as *const libc::c_char,
        (*us).name,
    );
    gvprintf(job, b"  }\n\0" as *const u8 as *const libc::c_char);
    gvprintf(job, b"}\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ps_freeimage(mut us: *mut usershape_t) {
    munmap((*us).data, (*us).datasize);
}
unsafe extern "C" fn core_loadimage_ps(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    if !job.is_null() {} else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void core_loadimage_ps(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !us.is_null() {} else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void core_loadimage_ps(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !((*us).name).is_null() {} else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void core_loadimage_ps(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !((*us).data).is_null() {
        if (*us).datafree
            != Some(ps_freeimage as unsafe extern "C" fn(*mut usershape_t) -> ())
        {
            ((*us).datafree).expect("non-null function pointer")(us);
            let ref mut fresh0 = (*us).data;
            *fresh0 = 0 as *mut libc::c_void;
            let ref mut fresh1 = (*us).datafree;
            *fresh1 = None;
            (*us).datasize = 0 as libc::c_int as size_t;
        }
    }
    if ((*us).data).is_null() {
        let mut fd: libc::c_int = 0;
        let mut statbuf: stat = stat {
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
        if !gvusershape_file_access(us) {
            return;
        }
        fd = fileno((*us).f);
        match (*us).type_0 as libc::c_uint {
            6 | 7 => {
                fstat(fd, &mut statbuf);
                (*us).datasize = statbuf.st_size as size_t;
                let ref mut fresh2 = (*us).data;
                *fresh2 = mmap(
                    0 as *mut libc::c_void,
                    (*us).datasize,
                    0x1 as libc::c_int,
                    0x1 as libc::c_int,
                    fd,
                    0 as libc::c_int as __off_t,
                );
                if (*us).data == -(1 as libc::c_int) as *mut libc::c_void {
                    let ref mut fresh3 = (*us).data;
                    *fresh3 = 0 as *mut libc::c_void;
                }
                (*us).must_inline = 1 as libc::c_int != 0;
            }
            _ => {}
        }
        if !((*us).data).is_null() {
            let ref mut fresh4 = (*us).datafree;
            *fresh4 = Some(ps_freeimage as unsafe extern "C" fn(*mut usershape_t) -> ());
        }
        gvusershape_file_release(us);
    }
    if !((*us).data).is_null() {
        gvprintf(
            job,
            b"gsave %g %g translate newpath\n\0" as *const u8 as *const libc::c_char,
            b.LL.x - (*us).x as libc::c_double,
            b.LL.y - (*us).y as libc::c_double,
        );
        if (*us).must_inline {
            epsf_emit_body(job, us);
        } else {
            gvprintf(
                job,
                b"user_shape_%d\n\0" as *const u8 as *const libc::c_char,
                (*us).macro_id,
            );
        }
        gvprintf(job, b"grestore\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn core_loadimage_pslib(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut i: libc::c_int = 0;
    let mut AF: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut shape: *mut shape_desc = 0 as *mut shape_desc;
    if !job.is_null() {} else {
        __assert_fail(
            b"job\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void core_loadimage_pslib(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !us.is_null() {} else {
        __assert_fail(
            b"us\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void core_loadimage_pslib(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    if !((*us).name).is_null() {} else {
        __assert_fail(
            b"us->name\0" as *const u8 as *const libc::c_char,
            b"gvloadimage_core.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"void core_loadimage_pslib(GVJ_t *, usershape_t *, boxf, _Bool)\0"))
                .as_ptr(),
        );
    }
    shape = (*us).data as *mut shape_desc;
    if !shape.is_null() {
        AF[0 as libc::c_int as usize] = b.LL;
        AF[2 as libc::c_int as usize] = b.UR;
        AF[1 as libc::c_int as usize].x = AF[0 as libc::c_int as usize].x;
        AF[1 as libc::c_int as usize].y = AF[2 as libc::c_int as usize].y;
        AF[3 as libc::c_int as usize].x = AF[2 as libc::c_int as usize].x;
        AF[3 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y;
        if filled {
            gvprintf(job, b"[ \0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                gvprintf(
                    job,
                    b"%g %g \0" as *const u8 as *const libc::c_char,
                    AF[i as usize].x,
                    AF[i as usize].y,
                );
                i += 1;
            }
            gvprintf(
                job,
                b"%g %g \0" as *const u8 as *const libc::c_char,
                AF[0 as libc::c_int as usize].x,
                AF[0 as libc::c_int as usize].y,
            );
            gvprintf(
                job,
                b"]  %d true %s\n\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
                (*us).name,
            );
        }
        gvprintf(job, b"[ \0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            gvprintf(
                job,
                b"%g %g \0" as *const u8 as *const libc::c_char,
                AF[i as usize].x,
                AF[i as usize].y,
            );
            i += 1;
        }
        gvprintf(
            job,
            b"%g %g \0" as *const u8 as *const libc::c_char,
            AF[0 as libc::c_int as usize].x,
            AF[0 as libc::c_int as usize].y,
        );
        gvprintf(
            job,
            b"]  %d false %s\n\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
            (*us).name,
        );
    }
}
unsafe extern "C" fn core_loadimage_vml(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut graphHeight: libc::c_uint = 0;
    graphHeight = ((*job).bb.UR.y - (*job).bb.LL.y) as libc::c_int as libc::c_uint;
    gvprintf(
        job,
        b"<v:image src=\"%s\" style=\" position:absolute; width:%.2f; height:%.2f; left:%.2f ; top:%.2f\"\0"
            as *const u8 as *const libc::c_char,
        (*us).name,
        b.UR.x - b.LL.x,
        b.UR.y - b.LL.y,
        b.LL.x,
        graphHeight as libc::c_double - b.UR.y,
    );
    gvputs(job, b" />\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn core_loadimage_tk(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    gvprintf(
        job,
        b"image create photo \"photo_%s\" -file \"%s\"\n\0" as *const u8
            as *const libc::c_char,
        (*us).name,
        (*us).name,
    );
    gvprintf(
        job,
        b"$c create image %.2f %.2f -image \"photo_%s\"\n\0" as *const u8
            as *const libc::c_char,
        (*us).name,
        (b.UR.x + b.LL.x) / 2 as libc::c_int as libc::c_double,
        (b.UR.y + b.LL.y) / 2 as libc::c_int as libc::c_double,
    );
}
unsafe extern "C" fn core_loadimage_null(
    mut gvc: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {}
static mut engine_svg: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_svg
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_fig: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_fig
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_vrml: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_vrml
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_ps: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_ps
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_pslib: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_pslib
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_null: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_null
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_xdot: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_xdot
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_vml: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_vml
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
static mut engine_tk: gvloadimage_engine_t = unsafe {
    {
        let mut init = gvloadimage_engine_s {
            loadimage: Some(
                core_loadimage_tk
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut usershape_t,
                        boxf,
                        bool,
                    ) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gvloadimage_core_types: [gvplugin_installed_t; 50] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_SVG as libc::c_int,
                type_0: b"png:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_svg as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_SVG as libc::c_int,
                type_0: b"gif:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_svg as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_SVG as libc::c_int,
                type_0: b"jpeg:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_svg as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_SVG as libc::c_int,
                type_0: b"jpe:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_svg as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_SVG as libc::c_int,
                type_0: b"jpg:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_svg as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_FIG as libc::c_int,
                type_0: b"png:fig\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_fig as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_FIG as libc::c_int,
                type_0: b"gif:fig\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_fig as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_FIG as libc::c_int,
                type_0: b"jpeg:fig\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_fig as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_FIG as libc::c_int,
                type_0: b"jpe:fig\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_fig as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_FIG as libc::c_int,
                type_0: b"jpg:fig\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_fig as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_VRML as libc::c_int,
                type_0: b"png:vrml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vrml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_VRML as libc::c_int,
                type_0: b"gif:vrml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vrml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_VRML as libc::c_int,
                type_0: b"jpeg:vrml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vrml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_VRML as libc::c_int,
                type_0: b"jpe:vrml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vrml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_VRML as libc::c_int,
                type_0: b"jpg:vrml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vrml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_PS as libc::c_int,
                type_0: b"eps:ps\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_ps as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_PS as libc::c_int,
                type_0: b"ps:ps\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_ps as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PSLIB_PS as libc::c_int,
                type_0: b"(lib):ps\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_pslib as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_MAP as libc::c_int,
                type_0: b"png:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_MAP as libc::c_int,
                type_0: b"gif:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_MAP as libc::c_int,
                type_0: b"jpeg:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_MAP as libc::c_int,
                type_0: b"jpe:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_MAP as libc::c_int,
                type_0: b"jpg:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_MAP as libc::c_int,
                type_0: b"ps:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_MAP as libc::c_int,
                type_0: b"eps:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVG_MAP as libc::c_int,
                type_0: b"svg:map\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_DOT as libc::c_int,
                type_0: b"png:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_DOT as libc::c_int,
                type_0: b"gif:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_DOT as libc::c_int,
                type_0: b"jpeg:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_DOT as libc::c_int,
                type_0: b"jpe:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_DOT as libc::c_int,
                type_0: b"jpg:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_DOT as libc::c_int,
                type_0: b"ps:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_DOT as libc::c_int,
                type_0: b"eps:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVG_DOT as libc::c_int,
                type_0: b"svg:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_null as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_XDOT as libc::c_int,
                type_0: b"png:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_XDOT as libc::c_int,
                type_0: b"gif:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_XDOT as libc::c_int,
                type_0: b"jpeg:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_XDOT as libc::c_int,
                type_0: b"jpe:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_XDOT as libc::c_int,
                type_0: b"jpg:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_XDOT as libc::c_int,
                type_0: b"ps:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS_XDOT as libc::c_int,
                type_0: b"eps:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVG_XDOT as libc::c_int,
                type_0: b"svg:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_xdot as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVG_SVG as libc::c_int,
                type_0: b"svg:svg\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_svg as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG_VML as libc::c_int,
                type_0: b"png:vml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_VML as libc::c_int,
                type_0: b"gif:vml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_VML as libc::c_int,
                type_0: b"jpeg:vml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_VML as libc::c_int,
                type_0: b"jpe:vml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG_VML as libc::c_int,
                type_0: b"jpg:vml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_vml as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF_TK as libc::c_int,
                type_0: b"gif:tk\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &engine_tk as *const gvloadimage_engine_t
                    as *mut gvloadimage_engine_t as *mut libc::c_void,
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
