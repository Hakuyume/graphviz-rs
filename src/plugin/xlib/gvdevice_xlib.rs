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
    pub type gvloadimage_engine_s;
    pub type gvrender_engine_s;
    pub type GVC_s;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
    fn fork() -> __pid_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn inotify_init() -> libc::c_int;
    fn inotify_add_watch(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __mask: uint32_t,
    ) -> libc::c_int;
    fn inotify_rm_watch(__fd: libc::c_int, __wd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XDisplayName(_: *const libc::c_char) -> *mut libc::c_char;
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int) -> Atom;
    fn XCreateColormap(_: *mut Display, _: Window, _: *mut Visual, _: libc::c_int) -> Colormap;
    fn XCreateGC(_: *mut Display, _: Drawable, _: libc::c_ulong, _: *mut XGCValues) -> GC;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Pixmap;
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_uint,
        _: *mut Visual,
        _: libc::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XStringToKeysym(_: *const libc::c_char) -> KeySym;
    fn XSetWMProtocols(_: *mut Display, _: Window, _: *mut Atom, _: libc::c_int) -> libc::c_int;
    fn XCopyArea(
        _: *mut Display,
        _: Drawable,
        _: Drawable,
        _: GC,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XFillRectangle(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XFlush(_: *mut Display) -> libc::c_int;
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;
    fn XKeysymToKeycode(_: *mut Display, _: KeySym) -> KeyCode;
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> libc::c_int;
    fn XPending(_: *mut Display) -> libc::c_int;
    fn XSelectInput(_: *mut Display, _: Window, _: libc::c_long) -> libc::c_int;
    fn cairo_xlib_surface_create(
        dpy: *mut Display,
        drawable: Drawable,
        visual: *mut Visual,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn XConnectionNumber(_: *mut Display) -> libc::c_int;
    fn XCloseDisplay(_: *mut Display) -> libc::c_int;
    fn XAllocClassHint() -> *mut XClassHint;
    fn XAllocSizeHints() -> *mut XSizeHints;
    fn XAllocWMHints() -> *mut XWMHints;
    fn XGetVisualInfo(
        _: *mut Display,
        _: libc::c_long,
        _: *mut XVisualInfo,
        _: *mut libc::c_int,
    ) -> *mut XVisualInfo;
    fn Xutf8SetWMProperties(
        _: *mut Display,
        _: Window,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut XSizeHints,
        _: *mut XWMHints,
        _: *mut XClassHint,
    );
    fn XRenderFindVisualFormat(dpy: *mut Display, visual: *const Visual) -> *mut XRenderPictFormat;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: libc::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [libc::c_char; 0],
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub width_inc: libc::c_int,
    pub height_inc: libc::c_int,
    pub min_aspect: C2RustUnnamed_6,
    pub max_aspect: C2RustUnnamed_6,
    pub base_width: libc::c_int,
    pub base_height: libc::c_int,
    pub win_gravity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWMHints {
    pub flags: libc::c_long,
    pub input: libc::c_int,
    pub initial_state: libc::c_int,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: libc::c_int,
    pub icon_y: libc::c_int,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut libc::c_char,
    pub res_class: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: libc::c_int,
    pub depth: libc::c_int,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub colormap_size: libc::c_int,
    pub bits_per_rgb: libc::c_int,
}
pub type PictFormat = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderDirectFormat {
    pub red: libc::c_short,
    pub redMask: libc::c_short,
    pub green: libc::c_short,
    pub greenMask: libc::c_short,
    pub blue: libc::c_short,
    pub blueMask: libc::c_short,
    pub alpha: libc::c_short,
    pub alphaMask: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderPictFormat {
    pub id: PictFormat,
    pub type_0: libc::c_int,
    pub depth: libc::c_int,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_xlib_s {
    pub win: Window,
    pub event_mask: uint64_t,
    pub pix: Pixmap,
    pub gc: GC,
    pub visual: *mut Visual,
    pub cmap: Colormap,
    pub depth: libc::c_int,
    pub wm_delete_window_atom: Atom,
}
pub type window_t = window_xlib_s;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
unsafe extern "C" fn handle_configure_notify(mut job: *mut GVJ_t, mut cev: *mut XConfigureEvent) {
    if (*cev).width >= 0 as libc::c_int
        && !(b"Xlib returned an event with negative width\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"cev->width >= 0 && \"Xlib returned an event with negative width\"\0" as *const u8
                as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"void handle_configure_notify(GVJ_t *, XConfigureEvent *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*cev).height >= 0 as libc::c_int
        && !(b"Xlib returned an event with negative height\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"cev->height >= 0 && \"Xlib returned an event with negative height\"\0" as *const u8
                as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"void handle_configure_notify(GVJ_t *, XConfigureEvent *)\0",
            ))
            .as_ptr(),
        );
    }
    (*job).zoom *= 1 as libc::c_int as libc::c_double
        + (if (((*cev).width as libc::c_double - (*job).width as libc::c_double)
            / (*job).width as libc::c_double)
            < ((*cev).height as libc::c_double - (*job).height as libc::c_double)
                / (*job).height as libc::c_double
        {
            ((*cev).width as libc::c_double - (*job).width as libc::c_double)
                / (*job).width as libc::c_double
        } else {
            ((*cev).height as libc::c_double - (*job).height as libc::c_double)
                / (*job).height as libc::c_double
        });
    if (*cev).width as libc::c_uint > (*job).width || (*cev).height as libc::c_uint > (*job).height
    {
        (*job).has_grown = 1 as libc::c_int != 0;
    }
    (*job).width = (*cev).width as libc::c_uint;
    (*job).height = (*cev).height as libc::c_uint;
    (*job).needs_refresh = 1 as libc::c_int != 0;
}
unsafe extern "C" fn handle_expose(mut job: *mut GVJ_t, mut eev: *mut XExposeEvent) {
    let mut window: *mut window_t = 0 as *mut window_t;
    window = (*job).window as *mut window_t;
    if (*eev).width >= 0 as libc::c_int
        && !(b"Xlib returned an expose event with negative width\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"eev->width >= 0 && \"Xlib returned an expose event with negative width\"\0"
                as *const u8 as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void handle_expose(GVJ_t *, XExposeEvent *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*eev).height >= 0 as libc::c_int
        && !(b"Xlib returned an expose event with negative height\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"eev->height >= 0 && \"Xlib returned an expose event with negative height\"\0"
                as *const u8 as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void handle_expose(GVJ_t *, XExposeEvent *)\0",
            ))
            .as_ptr(),
        );
    }
    XCopyArea(
        (*eev).display,
        (*window).pix,
        (*eev).window,
        (*window).gc,
        (*eev).x,
        (*eev).y,
        (*eev).width as libc::c_uint,
        (*eev).height as libc::c_uint,
        (*eev).x,
        (*eev).y,
    );
}
unsafe extern "C" fn handle_client_message(
    mut job: *mut GVJ_t,
    mut cmev: *mut XClientMessageEvent,
) {
    let mut window: *mut window_t = 0 as *mut window_t;
    window = (*job).window as *mut window_t;
    if (*cmev).format == 32 as libc::c_int
        && (*cmev).data.l[0 as libc::c_int as usize] as Atom == (*window).wm_delete_window_atom
    {
        graphviz_exit(0 as libc::c_int);
    }
}
unsafe extern "C" fn handle_keypress(mut job: *mut GVJ_t, mut kev: *mut XKeyEvent) -> bool {
    let mut i: libc::c_int = 0;
    let mut keycodes: *mut KeyCode = 0 as *mut KeyCode;
    keycodes = (*job).keycodes as *mut KeyCode;
    i = 0 as libc::c_int;
    while i < (*job).numkeys {
        if (*kev).keycode == *keycodes.offset(i as isize) as libc::c_uint {
            return ((*((*job).keybindings).offset(i as isize)).callback)
                .expect("non-null function pointer")(job)
                != 0 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn find_argb_visual(mut dpy: *mut Display, mut scr: libc::c_int) -> *mut Visual {
    let mut xvi: *mut XVisualInfo = 0 as *mut XVisualInfo;
    let mut template: XVisualInfo = XVisualInfo {
        visual: 0 as *mut Visual,
        visualid: 0,
        screen: 0,
        depth: 0,
        class: 0,
        red_mask: 0,
        green_mask: 0,
        blue_mask: 0,
        colormap_size: 0,
        bits_per_rgb: 0,
    };
    let mut nvi: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut format: *mut XRenderPictFormat = 0 as *mut XRenderPictFormat;
    let mut visual: *mut Visual = 0 as *mut Visual;
    template.screen = scr;
    template.depth = 32 as libc::c_int;
    template.class = 4 as libc::c_int;
    xvi = XGetVisualInfo(
        dpy,
        (0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int) as libc::c_long,
        &mut template,
        &mut nvi,
    );
    if xvi.is_null() {
        return 0 as *mut Visual;
    }
    visual = 0 as *mut Visual;
    i = 0 as libc::c_int;
    while i < nvi {
        format = XRenderFindVisualFormat(dpy, (*xvi.offset(i as isize)).visual);
        if (*format).type_0 == 1 as libc::c_int && (*format).direct.alphaMask as libc::c_int != 0 {
            visual = (*xvi.offset(i as isize)).visual;
            break;
        } else {
            i += 1;
        }
    }
    XFree(xvi as *mut libc::c_void);
    return visual;
}
unsafe extern "C" fn browser_show(mut job: *mut GVJ_t) {
    let mut exec_argv: [*mut libc::c_char; 3] = [
        b"xdg-open\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut pid: pid_t = 0;
    exec_argv[1 as libc::c_int as usize] = (*job).selected_href;
    pid = fork();
    if pid == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"fork failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    } else if pid == 0 as libc::c_int {
        execvp(
            exec_argv[0 as libc::c_int as usize],
            exec_argv.as_mut_ptr() as *const *mut libc::c_char,
        );
        fprintf(
            stderr,
            b"error starting %s: %s\n\0" as *const u8 as *const libc::c_char,
            exec_argv[0 as libc::c_int as usize],
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn handle_xlib_events(
    mut firstjob: *mut GVJ_t,
    mut dpy: *mut Display,
) -> libc::c_int {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut window: *mut window_t = 0 as *mut window_t;
    let mut xev: XEvent = _XEvent { type_0: 0 };
    let mut pointer: pointf = pointf { x: 0., y: 0. };
    let mut rc: libc::c_int = 0 as libc::c_int;
    while XPending(dpy) != 0 {
        XNextEvent(dpy, &mut xev);
        job = firstjob;
        while !job.is_null() {
            window = (*job).window as *mut window_t;
            if xev.xany.window == (*window).win {
                match xev.xany.type_0 {
                    4 => {
                        pointer.x = xev.xbutton.x as libc::c_double;
                        pointer.y = xev.xbutton.y as libc::c_double;
                        if xev.xbutton.button >= 1 as libc::c_int as libc::c_uint
                            && xev.xbutton.button <= 5 as libc::c_int as libc::c_uint
                            && !(b"Xlib returned invalid button event\0" as *const u8
                                as *const libc::c_char)
                                .is_null()
                        {
                        } else {
                            __assert_fail(
                                b"xev.xbutton.button >= 1 && xev.xbutton.button <= 5 && \"Xlib returned invalid button event\"\0"
                                    as *const u8 as *const libc::c_char,
                                b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
                                192 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 43],
                                    &[libc::c_char; 43],
                                >(b"int handle_xlib_events(GVJ_t *, Display *)\0"))
                                    .as_ptr(),
                            );
                        }
                        ((*(*job).callbacks).button_press).expect("non-null function pointer")(
                            job,
                            xev.xbutton.button as libc::c_int,
                            pointer,
                        );
                        rc += 1;
                    }
                    6 => {
                        if (*job).button != 0 {
                            pointer.x = xev.xbutton.x as libc::c_double;
                            pointer.y = xev.xbutton.y as libc::c_double;
                            ((*(*job).callbacks).motion).expect("non-null function pointer")(
                                job, pointer,
                            );
                            rc += 1;
                        }
                    }
                    5 => {
                        pointer.x = xev.xbutton.x as libc::c_double;
                        pointer.y = xev.xbutton.y as libc::c_double;
                        if xev.xbutton.button >= 1 as libc::c_int as libc::c_uint
                            && xev.xbutton.button <= 5 as libc::c_int as libc::c_uint
                            && !(b"Xlib returned invalid button event\0" as *const u8
                                as *const libc::c_char)
                                .is_null()
                        {
                        } else {
                            __assert_fail(
                                b"xev.xbutton.button >= 1 && xev.xbutton.button <= 5 && \"Xlib returned invalid button event\"\0"
                                    as *const u8 as *const libc::c_char,
                                b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
                                209 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 43],
                                    &[libc::c_char; 43],
                                >(b"int handle_xlib_events(GVJ_t *, Display *)\0"))
                                    .as_ptr(),
                            );
                        }
                        ((*(*job).callbacks).button_release).expect("non-null function pointer")(
                            job,
                            xev.xbutton.button as libc::c_int,
                            pointer,
                        );
                        if !((*job).selected_href).is_null()
                            && *((*job).selected_href).offset(0 as libc::c_int as isize)
                                as libc::c_int
                                != 0
                            && xev.xbutton.button == 1 as libc::c_int as libc::c_uint
                        {
                            browser_show(job);
                        }
                        rc += 1;
                    }
                    2 => {
                        if handle_keypress(job, &mut xev.xkey) {
                            return -(1 as libc::c_int);
                        }
                        rc += 1;
                    }
                    22 => {
                        handle_configure_notify(job, &mut xev.xconfigure);
                        rc += 1;
                    }
                    12 => {
                        handle_expose(job, &mut xev.xexpose);
                        rc += 1;
                    }
                    33 => {
                        handle_client_message(job, &mut xev.xclient);
                        rc += 1;
                    }
                    _ => {}
                }
                break;
            } else {
                job = (*job).next_active;
            }
        }
    }
    return rc;
}
unsafe extern "C" fn update_display(mut job: *mut GVJ_t, mut dpy: *mut Display) {
    let mut window: *mut window_t = 0 as *mut window_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    window = (*job).window as *mut window_t;
    if (*job).width <= 2147483647 as libc::c_int as libc::c_uint
        && !(b"out of range width\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"job->width <= (unsigned)INT_MAX && \"out of range width\"\0" as *const u8
                as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void update_display(GVJ_t *, Display *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*job).height <= 2147483647 as libc::c_int as libc::c_uint
        && !(b"out of range height\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"job->height <= (unsigned)INT_MAX && \"out of range height\"\0" as *const u8
                as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void update_display(GVJ_t *, Display *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*job).has_grown {
        XFreePixmap(dpy, (*window).pix);
        if (*window).depth >= 0 as libc::c_int
            && !(b"Xlib returned invalid window depth\0" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"window->depth >= 0 && \"Xlib returned invalid window depth\"\0" as *const u8
                    as *const libc::c_char,
                b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
                256 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void update_display(GVJ_t *, Display *)\0",
                ))
                .as_ptr(),
            );
        }
        (*window).pix = XCreatePixmap(
            dpy,
            (*window).win,
            (*job).width,
            (*job).height,
            (*window).depth as libc::c_uint,
        );
        (*job).has_grown = 0 as libc::c_int != 0;
        (*job).needs_refresh = 1 as libc::c_int != 0;
    }
    if (*job).needs_refresh {
        XFillRectangle(
            dpy,
            (*window).pix,
            (*window).gc,
            0 as libc::c_int,
            0 as libc::c_int,
            (*job).width,
            (*job).height,
        );
        surface = cairo_xlib_surface_create(
            dpy,
            (*window).pix,
            (*window).visual,
            (*job).width as libc::c_int,
            (*job).height as libc::c_int,
        );
        let ref mut fresh0 = (*job).context;
        *fresh0 = cairo_create(surface) as *mut libc::c_void;
        (*job).external_context = 1 as libc::c_int != 0;
        ((*(*job).callbacks).refresh).expect("non-null function pointer")(job);
        cairo_surface_destroy(surface);
        XCopyArea(
            dpy,
            (*window).pix,
            (*window).win,
            (*window).gc,
            0 as libc::c_int,
            0 as libc::c_int,
            (*job).width,
            (*job).height,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        (*job).needs_refresh = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn init_window(mut job: *mut GVJ_t, mut dpy: *mut Display, mut scr: libc::c_int) {
    let mut argb: libc::c_int = 0 as libc::c_int;
    let mut base: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut gcv: XGCValues = XGCValues {
        function: 0,
        plane_mask: 0,
        foreground: 0,
        background: 0,
        line_width: 0,
        line_style: 0,
        cap_style: 0,
        join_style: 0,
        fill_style: 0,
        fill_rule: 0,
        arc_mode: 0,
        tile: 0,
        stipple: 0,
        ts_x_origin: 0,
        ts_y_origin: 0,
        font: 0,
        subwindow_mode: 0,
        graphics_exposures: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        dash_offset: 0,
        dashes: 0,
    };
    let mut attributes: XSetWindowAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut wmhints: *mut XWMHints = 0 as *mut XWMHints;
    let mut normalhints: *mut XSizeHints = 0 as *mut XSizeHints;
    let mut classhint: *mut XClassHint = 0 as *mut XClassHint;
    let mut attributemask: uint64_t = 0 as libc::c_int as uint64_t;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut window: *mut window_t = 0 as *mut window_t;
    let mut zoom_to_fit: libc::c_double = 0.;
    window = malloc(::std::mem::size_of::<window_t>() as libc::c_ulong) as *mut window_t;
    if window.is_null() {
        fprintf(
            stderr,
            b"Failed to malloc window_t\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut w: libc::c_uint = 480 as libc::c_int as libc::c_uint;
    let mut h: libc::c_uint = 325 as libc::c_int as libc::c_uint;
    zoom_to_fit = if (w as libc::c_double / (*job).width as libc::c_double)
        < h as libc::c_double / (*job).height as libc::c_double
    {
        w as libc::c_double / (*job).width as libc::c_double
    } else {
        h as libc::c_double / (*job).height as libc::c_double
    };
    if zoom_to_fit < 1.0f64 {
        (*job).zoom *= zoom_to_fit;
    }
    (*job).width = w;
    (*job).height = h;
    let ref mut fresh1 = (*job).window;
    *fresh1 = window as *mut libc::c_void;
    (*job).fit_mode = 0 as libc::c_int != 0;
    (*job).needs_refresh = 1 as libc::c_int != 0;
    if argb != 0 && {
        let ref mut fresh2 = (*window).visual;
        *fresh2 = find_argb_visual(dpy, scr);
        !(*fresh2).is_null()
    } {
        (*window).cmap = XCreateColormap(
            dpy,
            (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).root,
            (*window).visual,
            0 as libc::c_int,
        );
        attributes.override_redirect = 0 as libc::c_int;
        attributes.background_pixel = 0 as libc::c_int as libc::c_ulong;
        attributes.border_pixel = 0 as libc::c_int as libc::c_ulong;
        attributes.colormap = (*window).cmap;
        attributemask = ((1 as libc::c_long) << 1 as libc::c_int
            | (1 as libc::c_long) << 3 as libc::c_int
            | (1 as libc::c_long) << 9 as libc::c_int
            | (1 as libc::c_long) << 13 as libc::c_int) as uint64_t;
        (*window).depth = 32 as libc::c_int;
    } else {
        (*window).cmap = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).cmap;
        let ref mut fresh3 = (*window).visual;
        *fresh3 = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).root_visual;
        attributes.background_pixel =
            (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).white_pixel;
        attributes.border_pixel =
            (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).black_pixel;
        attributemask = ((1 as libc::c_long) << 1 as libc::c_int
            | (1 as libc::c_long) << 3 as libc::c_int) as uint64_t;
        (*window).depth = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).root_depth;
    }
    (*window).win = XCreateWindow(
        dpy,
        (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).root,
        0 as libc::c_int,
        0 as libc::c_int,
        (*job).width,
        (*job).height,
        0 as libc::c_int as libc::c_uint,
        (*window).depth,
        1 as libc::c_int as libc::c_uint,
        (*window).visual,
        attributemask,
        &mut attributes,
    );
    name = malloc(
        (strlen(b"graphviz: \0" as *const u8 as *const libc::c_char))
            .wrapping_add(strlen(base))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(name, b"graphviz: \0" as *const u8 as *const libc::c_char);
    strcat(name, base);
    normalhints = XAllocSizeHints();
    (*normalhints).flags = 0 as libc::c_int as libc::c_long;
    (*normalhints).x = 0 as libc::c_int;
    (*normalhints).y = 0 as libc::c_int;
    if (*job).width <= 2147483647 as libc::c_int as libc::c_uint
        && !(b"out of range width\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"job->width <= (unsigned)INT_MAX && \"out of range width\"\0" as *const u8
                as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void init_window(GVJ_t *, Display *, int)\0",
            ))
            .as_ptr(),
        );
    }
    (*normalhints).width = (*job).width as libc::c_int;
    if (*job).height <= 2147483647 as libc::c_int as libc::c_uint
        && !(b"out of range height\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"job->height <= (unsigned)INT_MAX && \"out of range height\"\0" as *const u8
                as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void init_window(GVJ_t *, Display *, int)\0",
            ))
            .as_ptr(),
        );
    }
    (*normalhints).height = (*job).height as libc::c_int;
    classhint = XAllocClassHint();
    let ref mut fresh4 = (*classhint).res_name;
    *fresh4 = b"graphviz\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let ref mut fresh5 = (*classhint).res_class;
    *fresh5 = b"Graphviz\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    wmhints = XAllocWMHints();
    (*wmhints).flags = (1 as libc::c_long) << 0 as libc::c_int;
    (*wmhints).input = 1 as libc::c_int;
    Xutf8SetWMProperties(
        dpy,
        (*window).win,
        name,
        base,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
        normalhints,
        wmhints,
        classhint,
    );
    XFree(wmhints as *mut libc::c_void);
    XFree(classhint as *mut libc::c_void);
    XFree(normalhints as *mut libc::c_void);
    free(name as *mut libc::c_void);
    if (*window).depth >= 0 as libc::c_int
        && !(b"Xlib returned invalid window depth\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"window->depth >= 0 && \"Xlib returned invalid window depth\"\0" as *const u8
                as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            368 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void init_window(GVJ_t *, Display *, int)\0",
            ))
            .as_ptr(),
        );
    }
    (*window).pix = XCreatePixmap(
        dpy,
        (*window).win,
        (*job).width,
        (*job).height,
        (*window).depth as libc::c_uint,
    );
    if argb != 0 {
        gcv.foreground = 0 as libc::c_int as libc::c_ulong;
    } else {
        gcv.foreground = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).white_pixel;
    }
    let ref mut fresh6 = (*window).gc;
    *fresh6 = XCreateGC(
        dpy,
        (*window).pix,
        ((1 as libc::c_long) << 2 as libc::c_int) as libc::c_ulong,
        &mut gcv,
    );
    update_display(job, dpy);
    (*window).event_mask = ((1 as libc::c_long) << 2 as libc::c_int
        | (1 as libc::c_long) << 3 as libc::c_int
        | (1 as libc::c_long) << 6 as libc::c_int
        | (1 as libc::c_long) << 0 as libc::c_int
        | (1 as libc::c_long) << 17 as libc::c_int
        | (1 as libc::c_long) << 15 as libc::c_int) as uint64_t;
    XSelectInput(dpy, (*window).win, (*window).event_mask as libc::c_long);
    (*window).wm_delete_window_atom = XInternAtom(
        dpy,
        b"WM_DELETE_WINDOW\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    XSetWMProtocols(
        dpy,
        (*window).win,
        &mut (*window).wm_delete_window_atom,
        1 as libc::c_int,
    );
    XMapWindow(dpy, (*window).win);
}
unsafe extern "C" fn handle_stdin_events(mut job: *mut GVJ_t) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if feof(stdin) != 0 {
        return -(1 as libc::c_int);
    }
    ((*(*job).callbacks).read).expect("non-null function pointer")(
        job,
        (*job).input_filename,
        (*job).layout_type,
    );
    rc += 1;
    return rc;
}
unsafe extern "C" fn handle_file_events(
    mut job: *mut GVJ_t,
    mut inotify_fd: libc::c_int,
) -> libc::c_int {
    let mut avail: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut bf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut event: *mut inotify_event = 0 as *mut inotify_event;
    ret = ioctl(
        inotify_fd,
        0x541b as libc::c_int as libc::c_ulong,
        &mut avail as *mut libc::c_int,
    );
    if ret < 0 as libc::c_int {
        fprintf(
            stderr,
            b"ioctl() failed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if avail != 0 {
        if avail > 0 as libc::c_int
            && !(b"invalid value from FIONREAD\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"avail > 0 && \"invalid value from FIONREAD\"\0" as *const u8
                    as *const libc::c_char,
                b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
                418 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"int handle_file_events(GVJ_t *, int)\0",
                ))
                .as_ptr(),
            );
        }
        let mut buf: *mut libc::c_void = malloc(avail as size_t);
        if buf.is_null() {
            fprintf(
                stderr,
                b"out of memory (could not allocate %d bytes)\n\0" as *const u8
                    as *const libc::c_char,
                avail,
            );
            return -(1 as libc::c_int);
        }
        len = read(inotify_fd, buf, avail as size_t) as libc::c_int;
        if len != avail {
            fprintf(
                stderr,
                b"avail = %d, len = %d\n\0" as *const u8 as *const libc::c_char,
                avail,
                len,
            );
            free(buf);
            return -(1 as libc::c_int);
        }
        bf = buf as *mut libc::c_char;
        while len > 0 as libc::c_int {
            event = bf as *mut inotify_event;
            if (*event).mask == 0x2 as libc::c_int as libc::c_uint {
                p = strrchr((*job).input_filename, '/' as i32);
                if !p.is_null() {
                    p = p.offset(1);
                } else {
                    p = (*job).input_filename;
                }
                if strcmp(((*event).name).as_mut_ptr(), p) == 0 as libc::c_int {
                    ((*(*job).callbacks).read).expect("non-null function pointer")(
                        job,
                        (*job).input_filename,
                        (*job).layout_type,
                    );
                    rc += 1;
                }
            }
            let mut ln: size_t = ((*event).len as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<inotify_event>() as libc::c_ulong);
            if ln <= len as size_t {
            } else {
                __assert_fail(
                    b"ln <= (size_t)len\0" as *const u8 as *const libc::c_char,
                    b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
                    446 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"int handle_file_events(GVJ_t *, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            bf = bf.offset(ln as isize);
            len -= ln as libc::c_int;
        }
        free(buf);
        if len != 0 as libc::c_int {
            fprintf(
                stderr,
                b"length miscalculation, len = %d\n\0" as *const u8 as *const libc::c_char,
                len,
            );
            return -(1 as libc::c_int);
        }
    }
    return rc;
}
static mut initialized: bool = false;
unsafe extern "C" fn xlib_initialize(mut firstjob: *mut GVJ_t) {
    let mut dpy: *mut Display = 0 as *mut Display;
    let mut keysym: KeySym = 0;
    let mut keycodes: *mut KeyCode = 0 as *mut KeyCode;
    let mut display_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut scr: libc::c_int = 0;
    dpy = XOpenDisplay(display_name);
    if dpy.is_null() {
        fprintf(
            stderr,
            b"Failed to open XLIB display: %s\n\0" as *const u8 as *const libc::c_char,
            XDisplayName(0 as *const libc::c_char),
        );
        return;
    }
    scr = (*(dpy as _XPrivDisplay)).default_screen;
    let ref mut fresh7 = (*firstjob).display;
    *fresh7 = dpy as *mut libc::c_void;
    (*firstjob).screen = scr;
    if (*firstjob).numkeys >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"firstjob->numkeys >= 0\0" as *const u8 as *const libc::c_char,
            b"gvdevice_xlib.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void xlib_initialize(GVJ_t *)\0",
            ))
            .as_ptr(),
        );
    }
    keycodes = malloc(
        ((*firstjob).numkeys as size_t)
            .wrapping_mul(::std::mem::size_of::<KeyCode>() as libc::c_ulong),
    ) as *mut KeyCode;
    if keycodes.is_null() {
        fprintf(
            stderr,
            b"Failed to malloc %d*KeyCode\n\0" as *const u8 as *const libc::c_char,
            (*firstjob).numkeys,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < (*firstjob).numkeys {
        keysym = XStringToKeysym((*((*firstjob).keybindings).offset(i as isize)).keystring);
        if keysym == 0 as libc::c_long as libc::c_ulong {
            fprintf(
                stderr,
                b"ERROR: No keysym for \"%s\"\n\0" as *const u8 as *const libc::c_char,
                (*((*firstjob).keybindings).offset(i as isize)).keystring,
            );
        } else {
            *keycodes.offset(i as isize) = XKeysymToKeycode(dpy, keysym);
        }
        i += 1;
    }
    let ref mut fresh8 = (*firstjob).keycodes;
    *fresh8 = keycodes as *mut libc::c_void;
    (*firstjob).device_dpi.x = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).width
        as libc::c_double
        * 25.4f64
        / (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).mwidth as libc::c_double;
    (*firstjob).device_dpi.y = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).height
        as libc::c_double
        * 25.4f64
        / (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).mheight as libc::c_double;
    (*firstjob).device_sets_dpi = 1 as libc::c_int != 0;
    initialized = 1 as libc::c_int != 0;
}
unsafe extern "C" fn xlib_finalize(mut firstjob: *mut GVJ_t) {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut dpy: *mut Display = (*firstjob).display as *mut Display;
    let mut scr: libc::c_int = (*firstjob).screen;
    let mut keycodes: *mut KeyCode = (*firstjob).keycodes as *mut KeyCode;
    let mut numfds: libc::c_int = 0;
    let mut stdin_fd: libc::c_int = 0 as libc::c_int;
    let mut xlib_fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut events: libc::c_int = 0;
    let mut rfds: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut watching_stdin_p: bool = 0 as libc::c_int != 0;
    let mut wd: libc::c_int = 0 as libc::c_int;
    let mut inotify_fd: libc::c_int = 0 as libc::c_int;
    let mut watching_file_p: bool = 0 as libc::c_int != 0;
    static mut dir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    inotify_fd = inotify_init();
    if inotify_fd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"inotify_init() failed\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !initialized {
        return;
    }
    xlib_fd = XConnectionNumber(dpy);
    numfds = xlib_fd;
    if !((*firstjob).input_filename).is_null() {
        if (*firstjob).graph_index == 0 as libc::c_int {
            watching_file_p = 1 as libc::c_int != 0;
            if *((*firstjob).input_filename).offset(0 as libc::c_int as isize) as libc::c_int
                != '/' as i32
            {
                cwd = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
                dir = realloc(
                    dir as *mut libc::c_void,
                    (strlen(cwd))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*firstjob).input_filename))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                strcpy(dir, cwd);
                strcat(dir, b"/\0" as *const u8 as *const libc::c_char);
                strcat(dir, (*firstjob).input_filename);
                free(cwd as *mut libc::c_void);
            } else {
                dir = realloc(
                    dir as *mut libc::c_void,
                    (strlen((*firstjob).input_filename))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                strcpy(dir, (*firstjob).input_filename);
            }
            p = strrchr(dir, '/' as i32);
            *p = '\0' as i32 as libc::c_char;
            wd = inotify_add_watch(inotify_fd, dir, 0x2 as libc::c_int as uint32_t);
            numfds = if inotify_fd > numfds {
                inotify_fd
            } else {
                numfds
            };
        }
    } else {
        watching_stdin_p = 1 as libc::c_int != 0;
        stdin_fd = fcntl(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
        numfds = if stdin_fd > numfds { stdin_fd } else { numfds };
    }
    job = firstjob;
    while !job.is_null() {
        init_window(job, dpy, scr);
        job = (*job).next_active;
    }
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = &mut rfds;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong)
        < (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1);
    }
    loop {
        events = 0 as libc::c_int;
        if watching_file_p {
            if rfds.__fds_bits[(inotify_fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as usize]
                & ((1 as libc::c_ulong)
                    << inotify_fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                ret = handle_file_events(firstjob, inotify_fd);
                if ret < 0 as libc::c_int {
                    break;
                }
                events += ret;
            }
            rfds.__fds_bits[(inotify_fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as usize] |= ((1 as libc::c_ulong)
                << inotify_fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as __fd_mask;
        }
        if watching_stdin_p {
            if rfds.__fds_bits[(stdin_fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as usize]
                & ((1 as libc::c_ulong)
                    << stdin_fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                ret = handle_stdin_events(firstjob);
                if ret < 0 as libc::c_int {
                    watching_stdin_p = 0 as libc::c_int != 0;
                    rfds.__fds_bits[(stdin_fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                        as usize] &= !(((1 as libc::c_ulong)
                        << stdin_fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int))
                        as __fd_mask);
                }
                events += ret;
            }
            if watching_stdin_p {
                rfds.__fds_bits[(stdin_fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as usize] |= ((1 as libc::c_ulong)
                    << stdin_fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask;
            }
        }
        ret = handle_xlib_events(firstjob, dpy);
        if ret < 0 as libc::c_int {
            break;
        }
        events += ret;
        rfds.__fds_bits[(xlib_fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize] |= ((1 as libc::c_ulong)
            << xlib_fd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask;
        if events != 0 {
            job = firstjob;
            while !job.is_null() {
                update_display(job, dpy);
                job = (*job).next_active;
            }
            XFlush(dpy);
        }
        ret = select(
            numfds + 1 as libc::c_int,
            &mut rfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
        if !(ret < 0 as libc::c_int) {
            continue;
        }
        fprintf(
            stderr,
            b"select() failed\n\0" as *const u8 as *const libc::c_char,
        );
        break;
    }
    if watching_file_p {
        ret = inotify_rm_watch(inotify_fd, wd);
    }
    XCloseDisplay(dpy);
    free(keycodes as *mut libc::c_void);
    let ref mut fresh9 = (*firstjob).keycodes;
    *fresh9 = 0 as *mut libc::c_void;
}
static mut device_features_xlib: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 8 as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int,
        default_margin: {
            let mut init = pointf_s {
                x: 0.0f64,
                y: 0.0f64,
            };
            init
        },
        default_pagesize: {
            let mut init = pointf_s {
                x: 0.0f64,
                y: 0.0f64,
            };
            init
        },
        default_dpi: {
            let mut init = pointf_s {
                x: 96.0f64,
                y: 96.0f64,
            };
            init
        },
    };
    init
};
static mut device_engine_xlib: gvdevice_engine_t = unsafe {
    {
        let mut init = gvdevice_engine_s {
            initialize: Some(xlib_initialize as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            format: None,
            finalize: Some(xlib_finalize as unsafe extern "C" fn(*mut GVJ_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut gvdevice_types_xlib: [gvplugin_installed_t; 3] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: b"xlib:cairo\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &device_engine_xlib as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_xlib as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: b"x11:cairo\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &device_engine_xlib as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_xlib as *const gvdevice_features_t
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
