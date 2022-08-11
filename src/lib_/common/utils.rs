#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agsubnode(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agcontains(_: *mut Agraph_t, _: *mut libc::c_void) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    static mut Gvfilepath: *mut libc::c_char;
    static mut Gvimagepath: *mut libc::c_char;
    static mut HTTPServerEnVar: *mut libc::c_char;
    static mut PSinputscale: libc::c_double;
    static mut N_height: *mut Agsym_t;
    static mut N_width: *mut Agsym_t;
    static mut N_shape: *mut Agsym_t;
    static mut N_fontsize: *mut Agsym_t;
    static mut N_fontname: *mut Agsym_t;
    static mut N_fontcolor: *mut Agsym_t;
    static mut N_label: *mut Agsym_t;
    static mut N_xlabel: *mut Agsym_t;
    static mut N_style: *mut Agsym_t;
    static mut N_showboxes: *mut Agsym_t;
    static mut E_fontsize: *mut Agsym_t;
    static mut E_fontname: *mut Agsym_t;
    static mut E_fontcolor: *mut Agsym_t;
    static mut E_label: *mut Agsym_t;
    static mut E_xlabel: *mut Agsym_t;
    static mut E_label_float: *mut Agsym_t;
    static mut E_headlabel: *mut Agsym_t;
    static mut E_taillabel: *mut Agsym_t;
    static mut E_labelfontsize: *mut Agsym_t;
    static mut E_labelfontname: *mut Agsym_t;
    static mut E_labelfontcolor: *mut Agsym_t;
    static mut E_tailclip: *mut Agsym_t;
    static mut E_headclip: *mut Agsym_t;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn lineToBox(p1: pointf, p2: pointf, b: boxf) -> libc::c_int;
    fn free_label(_: *mut textlabel_t);
    fn make_label(
        obj: *mut libc::c_void,
        str: *mut libc::c_char,
        kind: libc::c_int,
        fontsize: libc::c_double,
        fontname: *mut libc::c_char,
        fontcolor: *mut libc::c_char,
    ) -> *mut textlabel_t;
    fn shapeOf(_: *mut node_t) -> shape_kind;
    fn bind_shape(name: *mut libc::c_char, _: *mut node_t) -> *mut shape_desc;
    fn arrow_bb(p: pointf, u: pointf, arrowsize: libc::c_double) -> boxf;
    fn agobjkind(_: *mut libc::c_void) -> libc::c_int;
    fn aghtmlstr(_: *const libc::c_char) -> libc::c_int;
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agnxtattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        attr: *mut Agsym_t,
    ) -> *mut Agsym_t;
    fn agcopyattr(oldobj: *mut libc::c_void, newobj: *mut libc::c_void) -> libc::c_int;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn agdelrec(obj: *mut libc::c_void, name: *const libc::c_char) -> libc::c_int;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint64_t = __uint64_t;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub type Ppoint_t = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ppoly_t {
    pub ps: *mut Ppoint_t,
    pub pn: libc::c_int,
}
pub type Ppolyline_t = Ppoly_t;
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
pub type Agedge_t = Agedge_s;
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
pub union inside_t {
    pub a: C2RustUnnamed_9,
    pub s: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct bezier {
    pub list: *mut pointf,
    pub size: libc::c_int,
    pub sflag: libc::c_int,
    pub eflag: libc::c_int,
    pub sp: pointf,
    pub ep: pointf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct splines {
    pub list: *mut bezier,
    pub size: libc::c_int,
    pub bb: boxf,
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
    pub u: C2RustUnnamed_10,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub txt: C2RustUnnamed_11,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub type shape_kind = libc::c_uint;
pub const SH_EPSF: shape_kind = 4;
pub const SH_POINT: shape_kind = 3;
pub const SH_RECORD: shape_kind = 2;
pub const SH_POLY: shape_kind = 1;
pub const SH_UNSET: shape_kind = 0;
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
pub struct nodequeue {
    pub store: *mut *mut node_t,
    pub limit: *mut *mut node_t,
    pub head: *mut *mut node_t,
    pub tail: *mut *mut node_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elist {
    pub list: *mut *mut edge_t,
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
pub struct Agnodeinfo_t {
    pub hdr: Agrec_t,
    pub shape: *mut shape_desc,
    pub shape_info: *mut libc::c_void,
    pub coord: pointf,
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub bb: boxf,
    pub ht: libc::c_double,
    pub lw: libc::c_double,
    pub rw: libc::c_double,
    pub label: *mut textlabel_t,
    pub xlabel: *mut textlabel_t,
    pub alg: *mut libc::c_void,
    pub state: libc::c_char,
    pub gui_state: libc::c_uchar,
    pub clustnode: bool,
    pub pinned: libc::c_uchar,
    pub id: libc::c_int,
    pub heapindex: libc::c_int,
    pub hops: libc::c_int,
    pub pos: *mut libc::c_double,
    pub dist: libc::c_double,
    pub showboxes: libc::c_uchar,
    pub has_port: bool,
    pub rep: *mut node_t,
    pub set: *mut node_t,
    pub node_type: libc::c_char,
    pub mark: size_t,
    pub onstack: libc::c_char,
    pub ranktype: libc::c_char,
    pub weight_class: libc::c_char,
    pub next: *mut node_t,
    pub prev: *mut node_t,
    pub in_0: elist,
    pub out: elist,
    pub flat_out: elist,
    pub flat_in: elist,
    pub other: elist,
    pub clust: *mut graph_t,
    pub UF_size: libc::c_int,
    pub UF_parent: *mut node_t,
    pub inleaf: *mut node_t,
    pub outleaf: *mut node_t,
    pub rank: libc::c_int,
    pub order: libc::c_int,
    pub mval: libc::c_double,
    pub save_in: elist,
    pub save_out: elist,
    pub tree_in: elist,
    pub tree_out: elist,
    pub par: *mut edge_t,
    pub low: libc::c_int,
    pub lim: libc::c_int,
    pub priority: libc::c_int,
    pub pad: [libc::c_double; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agedgeinfo_t {
    pub hdr: Agrec_t,
    pub spl: *mut splines,
    pub tail_port: port,
    pub head_port: port,
    pub label: *mut textlabel_t,
    pub head_label: *mut textlabel_t,
    pub tail_label: *mut textlabel_t,
    pub xlabel: *mut textlabel_t,
    pub edge_type: libc::c_char,
    pub compound: libc::c_char,
    pub adjacent: libc::c_char,
    pub label_ontop: libc::c_char,
    pub gui_state: libc::c_uchar,
    pub to_orig: *mut edge_t,
    pub alg: *mut libc::c_void,
    pub factor: libc::c_double,
    pub dist: libc::c_double,
    pub path: Ppolyline_t,
    pub showboxes: libc::c_uchar,
    pub conc_opp_flag: bool,
    pub xpenalty: libc::c_short,
    pub weight: libc::c_int,
    pub cutvalue: libc::c_int,
    pub tree_index: libc::c_int,
    pub count: libc::c_short,
    pub minlen: libc::c_ushort,
    pub to_virt: *mut edge_t,
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
pub struct cl_edge_t {
    pub hdr: Agrec_t,
    pub n_cluster_edges: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clust_t {
    pub link: Dtlink_t,
    pub name: *mut libc::c_char,
    pub clp: *mut Agraph_t,
}
pub const _ISdigit: C2RustUnnamed_12 = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontinfo {
    pub fontsize: libc::c_double,
    pub fontname: *mut libc::c_char,
    pub fontcolor: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub link: Dtlink_t,
    pub p: [*mut libc::c_void; 2],
    pub t: *mut node_t,
    pub h: *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entities_s {
    pub name: *mut libc::c_char,
    pub value: libc::c_int,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_12 = 8;
pub const _ISpunct: C2RustUnnamed_12 = 4;
pub const _IScntrl: C2RustUnnamed_12 = 2;
pub const _ISblank: C2RustUnnamed_12 = 1;
pub const _ISgraph: C2RustUnnamed_12 = 32768;
pub const _ISprint: C2RustUnnamed_12 = 16384;
pub const _ISspace: C2RustUnnamed_12 = 8192;
pub const _ISxdigit: C2RustUnnamed_12 = 4096;
pub const _ISalpha: C2RustUnnamed_12 = 1024;
pub const _ISlower: C2RustUnnamed_12 = 512;
pub const _ISupper: C2RustUnnamed_12 = 256;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
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
unsafe extern "C" fn pointfof(mut x: libc::c_double, mut y: libc::c_double) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = x;
    r.y = y;
    return r;
}
#[inline]
unsafe extern "C" fn add_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x + q.x;
    r.y = p.y + q.y;
    return r;
}
#[inline]
unsafe extern "C" fn sub_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x - q.x;
    r.y = p.y - q.y;
    return r;
}
#[inline]
unsafe extern "C" fn mid_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = (p.x + q.x) / 2.0f64;
    r.y = (p.y + q.y) / 2.0f64;
    return r;
}
#[inline]
unsafe extern "C" fn boxf_overlap(mut b0: boxf, mut b1: boxf) -> libc::c_int {
    return (b0.UR.x >= b1.LL.x && b1.UR.x >= b0.LL.x && b0.UR.y >= b1.LL.y
        && b1.UR.y >= b0.LL.y) as libc::c_int;
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
unsafe extern "C" fn agxbdisown(mut xb: *mut agxbuf) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    agxbputc(xb, '\0' as i32 as libc::c_char);
    if (*xb).dyna == 0 {
        buf = strdup((*xb).buf);
        if buf.is_null() {
            return 0 as *mut libc::c_char;
        }
    } else {
        buf = (*xb).buf;
    }
    let ref mut fresh11 = (*xb).eptr;
    *fresh11 = 0 as *mut libc::c_char;
    let ref mut fresh12 = (*xb).ptr;
    *fresh12 = *fresh11;
    let ref mut fresh13 = (*xb).buf;
    *fresh13 = *fresh12;
    (*xb).dyna = 1 as libc::c_int;
    return buf;
}
static mut entities: [entities_s; 252] = [
    {
        let mut init = entities_s {
            name: b"AElig\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 198 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Aacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 193 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Acirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 194 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Agrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 192 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Alpha\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 913 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 197 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Atilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 195 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Auml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 196 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Beta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 914 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ccedil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 199 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Chi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 935 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Dagger\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8225 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Delta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 916 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ETH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 208 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Eacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 201 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ecirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 202 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Egrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Epsilon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 917 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Eta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 919 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Euml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 203 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Gamma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 915 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Iacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 205 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Icirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 206 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Igrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 204 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Iota\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 921 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Iuml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 207 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Kappa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 922 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Lambda\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 923 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Mu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 924 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ntilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 209 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Nu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 925 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"OElig\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 338 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Oacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 211 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ocirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 212 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ograve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 210 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Omega\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 937 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Omicron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 927 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Oslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 216 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Otilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 213 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ouml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 214 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Phi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 934 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Pi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 928 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Prime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8243 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Psi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 936 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Rho\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 929 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Scaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 352 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Sigma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 931 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"THORN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 222 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Tau\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 932 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Theta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 920 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Uacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 218 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ucirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 219 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Ugrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 217 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Upsilon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 933 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Uuml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Xi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 926 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Yacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 221 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Yuml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 376 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"Zeta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 918 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"aacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 225 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"acirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 226 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"acute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 180 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"aelig\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 230 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"agrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 224 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"alefsym\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8501 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"alpha\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 945 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"amp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 38 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"and\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8743 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ang\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8736 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 229 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"asymp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8776 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"atilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 227 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"auml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 228 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"bdquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8222 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"beta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 946 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"brvbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 166 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"bull\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8226 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"cap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8745 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ccedil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 231 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"cedil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 184 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"cent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 162 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"chi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 967 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"circ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 710 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"clubs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 9827 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"cong\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8773 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"copy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 169 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"crarr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8629 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"cup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8746 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"curren\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 164 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"dArr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8659 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"dagger\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8224 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"darr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8595 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"deg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 176 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"delta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 948 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"diams\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 9830 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"divide\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 247 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"eacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 233 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ecirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 234 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"egrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 232 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"empty\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8709 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"emsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8195 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ensp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8194 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"epsilon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 949 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"equiv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8801 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"eta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 951 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"eth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 240 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"euml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 235 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"euro\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8364 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"exist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8707 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"fnof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 402 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"forall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8704 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"frac12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 189 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"frac14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 188 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"frac34\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 190 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"frasl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8260 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"gamma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 947 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8805 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"gt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 62 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"hArr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8660 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"harr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8596 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"hearts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 9829 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"hellip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8230 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"iacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 237 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"icirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 238 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"iexcl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 161 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"igrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 236 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"image\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8465 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"infin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8734 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"int\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8747 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"iota\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 953 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"iquest\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 191 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"isin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8712 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"iuml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 239 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"kappa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 954 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lArr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8656 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lambda\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 955 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lang\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 9001 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"laquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 171 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"larr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8592 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lceil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8968 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ldquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8220 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"le\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8804 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lfloor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8970 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lowast\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8727 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"loz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 9674 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lrm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8206 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lsaquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8249 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lsquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8216 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"lt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"macr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 175 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"mdash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8212 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"micro\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 181 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"middot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 183 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"minus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8722 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"mu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 956 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"nabla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8711 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"nbsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 160 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ndash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8211 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ne\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8800 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ni\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8715 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"not\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 172 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"notin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8713 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"nsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8836 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ntilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 241 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"nu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 957 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"oacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 243 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ocirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 244 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"oelig\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 339 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ograve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 242 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"oline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8254 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"omega\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 969 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"omicron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 959 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"oplus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8853 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"or\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8744 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ordf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 170 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ordm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 186 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"oslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 248 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"otilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 245 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"otimes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8855 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ouml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 246 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"para\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 182 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"part\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8706 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"permil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8240 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"perp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8869 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"phi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 966 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"pi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 960 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"piv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 982 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"plusmn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 177 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"pound\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 163 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"prime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8242 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"prod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8719 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"prop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8733 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"psi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 968 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"quot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 34 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rArr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8658 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"radic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8730 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rang\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 9002 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"raquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 187 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rarr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8594 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rceil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8969 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rdquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8221 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"real\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8476 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"reg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 174 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rfloor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8971 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rho\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 961 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rlm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8207 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rsaquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8250 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"rsquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8217 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sbquo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8218 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"scaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 353 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sdot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8901 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 167 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"shy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 173 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sigma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 963 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sigmaf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 962 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8764 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"spades\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 9824 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8834 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sube\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8838 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8721 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8835 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sup1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 185 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sup2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 178 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"sup3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 179 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"supe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8839 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"szlig\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 223 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"tau\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 964 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"there4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8756 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"theta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 952 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"thetasym\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 977 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"thinsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8201 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"thorn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 254 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"tilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 732 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 215 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"trade\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8482 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"uArr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8657 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"uacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 250 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"uarr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8593 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ucirc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 251 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"ugrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 249 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"uml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 168 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"upsih\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 978 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"upsilon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 965 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"uuml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 252 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"weierp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8472 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"xi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 958 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"yacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 253 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"yen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 165 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"yuml\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 255 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"zeta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 950 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"zwj\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8205 as libc::c_int,
        };
        init
    },
    {
        let mut init = entities_s {
            name: b"zwnj\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: 8204 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn new_queue(mut sz: libc::c_int) -> *mut nodequeue {
    let mut q: *mut nodequeue = zmalloc(
        ::std::mem::size_of::<nodequeue>() as libc::c_ulong,
    ) as *mut nodequeue;
    if sz <= 1 as libc::c_int {
        sz = 2 as libc::c_int;
    }
    let ref mut fresh14 = (*q).store;
    *fresh14 = gcalloc(
        sz as size_t,
        ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
    ) as *mut *mut node_t;
    let ref mut fresh15 = (*q).tail;
    *fresh15 = *fresh14;
    let ref mut fresh16 = (*q).head;
    *fresh16 = *fresh15;
    let ref mut fresh17 = (*q).limit;
    *fresh17 = ((*q).store).offset(sz as isize);
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn free_queue(mut q: *mut nodequeue) {
    free((*q).store as *mut libc::c_void);
    free(q as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn enqueue(mut q: *mut nodequeue, mut n: *mut node_t) {
    let ref mut fresh18 = (*q).tail;
    let fresh19 = *fresh18;
    *fresh18 = (*fresh18).offset(1);
    *fresh19 = n;
    if (*q).tail >= (*q).limit {
        let ref mut fresh20 = (*q).tail;
        *fresh20 = (*q).store;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dequeue(mut q: *mut nodequeue) -> *mut Agnode_t {
    let mut n: *mut node_t = 0 as *mut node_t;
    if (*q).head == (*q).tail {
        n = 0 as *mut node_t;
    } else {
        let ref mut fresh21 = (*q).head;
        let fresh22 = *fresh21;
        *fresh21 = (*fresh21).offset(1);
        n = *fresh22;
        if (*q).head >= (*q).limit {
            let ref mut fresh23 = (*q).head;
            *fresh23 = (*q).store;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn late_int(
    mut obj: *mut libc::c_void,
    mut attr: *mut attrsym_t,
    mut def: libc::c_int,
    mut low: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rv: libc::c_int = 0;
    if attr.is_null() {
        return def;
    }
    p = agxget(obj, attr);
    if p.is_null() || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return def;
    }
    rv = strtol(p, &mut endp, 10 as libc::c_int) as libc::c_int;
    if p == endp {
        return def;
    }
    if rv < low { return low } else { return rv };
}
#[no_mangle]
pub unsafe extern "C" fn late_double(
    mut obj: *mut libc::c_void,
    mut attr: *mut attrsym_t,
    mut def: libc::c_double,
    mut low: libc::c_double,
) -> libc::c_double {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rv: libc::c_double = 0.;
    if attr.is_null() || obj.is_null() {
        return def;
    }
    p = agxget(obj, attr);
    if p.is_null() || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return def;
    }
    rv = strtod(p, &mut endp);
    if p == endp {
        return def;
    }
    if rv < low { return low } else { return rv };
}
#[no_mangle]
pub unsafe extern "C" fn get_inputscale(mut g: *mut graph_t) -> libc::c_double {
    let mut d: libc::c_double = 0.;
    if PSinputscale > 0 as libc::c_int as libc::c_double {
        return PSinputscale;
    }
    d = late_double(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"inputscale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        -(1 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    if d == 0 as libc::c_int as libc::c_double {
        return 72 as libc::c_int as libc::c_double
    } else {
        return d
    };
}
#[no_mangle]
pub unsafe extern "C" fn late_string(
    mut obj: *mut libc::c_void,
    mut attr: *mut attrsym_t,
    mut def: *mut libc::c_char,
) -> *mut libc::c_char {
    if attr.is_null() || obj.is_null() {
        return def;
    }
    return agxget(obj, attr);
}
#[no_mangle]
pub unsafe extern "C" fn late_nnstring(
    mut obj: *mut libc::c_void,
    mut attr: *mut attrsym_t,
    mut def: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut rv: *mut libc::c_char = late_string(obj, attr, def);
    if rv.is_null()
        || *rv.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        rv = def;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn late_bool(
    mut obj: *mut libc::c_void,
    mut attr: *mut attrsym_t,
    mut def: bool,
) -> bool {
    if attr.is_null() {
        return def;
    }
    return mapbool(agxget(obj, attr));
}
#[no_mangle]
pub unsafe extern "C" fn UF_find(mut n: *mut node_t) -> *mut Agnode_t {
    while !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_parent).is_null()
        && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_parent != n
    {
        if !((*((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_parent
            as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .UF_parent)
            .is_null()
        {
            let ref mut fresh24 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .UF_parent;
            *fresh24 = (*((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .UF_parent as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .UF_parent;
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_parent;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn UF_union(
    mut u: *mut node_t,
    mut v: *mut node_t,
) -> *mut Agnode_t {
    if u == v {
        return u;
    }
    if ((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_parent).is_null() {
        let ref mut fresh25 = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .UF_parent;
        *fresh25 = u;
        (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size = 1 as libc::c_int;
    } else {
        u = UF_find(u);
    }
    if ((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_parent).is_null() {
        let ref mut fresh26 = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .UF_parent;
        *fresh26 = v;
        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size = 1 as libc::c_int;
    } else {
        v = UF_find(v);
    }
    if u == v {
        return u;
    }
    if (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id
        > (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id
    {
        let ref mut fresh27 = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .UF_parent;
        *fresh27 = v;
        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size
            += (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size;
    } else {
        let ref mut fresh28 = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .UF_parent;
        *fresh28 = u;
        (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size
            += (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size;
        v = u;
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn UF_singleton(mut u: *mut node_t) {
    (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size = 1 as libc::c_int;
    let ref mut fresh29 = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .UF_parent;
    *fresh29 = 0 as *mut node_t;
    (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .ranktype = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn UF_setname(mut u: *mut node_t, mut v: *mut node_t) {
    if u == UF_find(u) {} else {
        __assert_fail(
            b"u == UF_find(u)\0" as *const u8 as *const libc::c_char,
            b"utils.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void UF_setname(node_t *, node_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh30 = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .UF_parent;
    *fresh30 = v;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size
        += (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size;
}
#[no_mangle]
pub unsafe extern "C" fn coord(mut n: *mut node_t) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r
        .x = 72 as libc::c_int as libc::c_double
        * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize);
    r
        .y = 72 as libc::c_int as libc::c_double
        * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn Bezier(
    mut V: *mut pointf,
    mut degree: libc::c_int,
    mut t: libc::c_double,
    mut Left: *mut pointf,
    mut Right: *mut pointf,
) -> pointf {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut Vtemp: [[pointf; 6]; 6] = [[pointf { x: 0., y: 0. }; 6]; 6];
    j = 0 as libc::c_int;
    while j <= degree {
        Vtemp[0 as libc::c_int as usize][j as usize] = *V.offset(j as isize);
        j += 1;
    }
    i = 1 as libc::c_int;
    while i <= degree {
        j = 0 as libc::c_int;
        while j <= degree - i {
            Vtemp[i as usize][j as usize]
                .x = (1.0f64 - t) * Vtemp[(i - 1 as libc::c_int) as usize][j as usize].x
                + t
                    * Vtemp[(i - 1 as libc::c_int)
                            as usize][(j + 1 as libc::c_int) as usize]
                        .x;
            Vtemp[i as usize][j as usize]
                .y = (1.0f64 - t) * Vtemp[(i - 1 as libc::c_int) as usize][j as usize].y
                + t
                    * Vtemp[(i - 1 as libc::c_int)
                            as usize][(j + 1 as libc::c_int) as usize]
                        .y;
            j += 1;
        }
        i += 1;
    }
    if !Left.is_null() {
        j = 0 as libc::c_int;
        while j <= degree {
            *Left.offset(j as isize) = Vtemp[j as usize][0 as libc::c_int as usize];
            j += 1;
        }
    }
    if !Right.is_null() {
        j = 0 as libc::c_int;
        while j <= degree {
            *Right.offset(j as isize) = Vtemp[(degree - j) as usize][j as usize];
            j += 1;
        }
    }
    return Vtemp[degree as usize][0 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Fgets(mut fp: *mut FILE) -> *mut libc::c_char {
    static mut bsize: size_t = 0 as libc::c_int as size_t;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut lp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    len = 0 as libc::c_int as size_t;
    loop {
        if bsize.wrapping_sub(len) < 8192 as libc::c_int as libc::c_ulong {
            bsize = (bsize as libc::c_ulong)
                .wrapping_add(8192 as libc::c_int as libc::c_ulong) as size_t as size_t;
            buf = grealloc(buf as *mut libc::c_void, bsize) as *mut libc::c_char;
        }
        lp = fgets(buf.offset(len as isize), bsize.wrapping_sub(len) as libc::c_int, fp);
        if lp.is_null() {
            break;
        }
        len = (len as libc::c_ulong).wrapping_add(strlen(lp)) as size_t as size_t;
        if !(*buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '\n' as i32)
        {
            break;
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        return buf
    } else {
        return 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn mkDirlist(
    mut list: *const libc::c_char,
    mut maxdirlen: *mut size_t,
) -> *mut *mut libc::c_char {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = strdup(list);
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut maxlen: size_t = 0 as libc::c_int as size_t;
    dir = strtok(s, b":\0" as *const u8 as *const libc::c_char);
    while !dir.is_null() {
        dirs = if !dirs.is_null() {
            grealloc(
                dirs as *mut libc::c_void,
                ((cnt + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char
        } else {
            gmalloc(
                ((cnt + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char
        };
        let fresh31 = cnt;
        cnt = cnt + 1;
        let ref mut fresh32 = *dirs.offset(fresh31 as isize);
        *fresh32 = dir;
        maxlen = if maxlen > strlen(dir) { maxlen } else { strlen(dir) };
        dir = strtok(0 as *mut libc::c_char, b":\0" as *const u8 as *const libc::c_char);
    }
    let ref mut fresh33 = *dirs.offset(cnt as isize);
    *fresh33 = 0 as *mut libc::c_char;
    *maxdirlen = maxlen;
    return dirs;
}
unsafe extern "C" fn findPath(
    mut dirs: *mut *mut libc::c_char,
    mut maxdirlen: size_t,
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    static mut safefilename: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut dp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    safefilename = realloc(
        safefilename as *mut libc::c_void,
        maxdirlen
            .wrapping_add(strlen(str))
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    dp = dirs;
    while !(*dp).is_null() {
        sprintf(
            safefilename,
            b"%s%s%s\0" as *const u8 as *const libc::c_char,
            *dp,
            b"/\0" as *const u8 as *const libc::c_char,
            str,
        );
        if access(safefilename, 4 as libc::c_int) == 0 as libc::c_int {
            return safefilename;
        }
        dp = dp.offset(1);
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safefile(
    mut filename: *const libc::c_char,
) -> *const libc::c_char {
    static mut onetime: bool = 1 as libc::c_int != 0;
    static mut pathlist: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut maxdirlen: size_t = 0;
    static mut dirs: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if filename.is_null() || *filename.offset(0 as libc::c_int as isize) == 0 {
        return 0 as *const libc::c_char;
    }
    if !HTTPServerEnVar.is_null() {
        if Gvfilepath.is_null() || *Gvfilepath as libc::c_int == '\0' as i32 {
            if onetime {
                agerr(
                    AGWARN,
                    b"file loading is disabled because the environment contains SERVER_NAME=\"%s\"\nand the GV_FILE_PATH variable is unset or empty.\n\0"
                        as *const u8 as *const libc::c_char,
                    HTTPServerEnVar,
                );
                onetime = 0 as libc::c_int != 0;
            }
            return 0 as *const libc::c_char;
        }
        if pathlist.is_null() {
            dirs = mkDirlist(Gvfilepath, &mut maxdirlen);
            pathlist = Gvfilepath;
        }
        str = filename;
        p = strrchr(str, '/' as i32);
        if !p.is_null() {
            p = p.offset(1);
            str = p;
        }
        p = strrchr(str, '\\' as i32);
        if !p.is_null() {
            p = p.offset(1);
            str = p;
        }
        p = strrchr(str, ':' as i32);
        if !p.is_null() {
            p = p.offset(1);
            str = p;
        }
        if onetime as libc::c_int != 0 && str != filename {
            agerr(
                AGWARN,
                b"Path provided to file: \"%s\" has been ignored because files are only permitted to be loaded from the directories in \"%s\" when running in an http server.\n\0"
                    as *const u8 as *const libc::c_char,
                filename,
                Gvfilepath,
            );
            onetime = 0 as libc::c_int != 0;
        }
        return findPath(dirs, maxdirlen, str);
    }
    if pathlist != Gvimagepath {
        if !dirs.is_null() {
            free(*dirs.offset(0 as libc::c_int as isize) as *mut libc::c_void);
            free(dirs as *mut libc::c_void);
            dirs = 0 as *mut *mut libc::c_char;
        }
        pathlist = Gvimagepath;
        if !pathlist.is_null() && *pathlist as libc::c_int != 0 {
            dirs = mkDirlist(pathlist, &mut maxdirlen);
        }
    }
    if *filename as libc::c_int
        == (*::std::mem::transmute::<
            &[u8; 2],
            &[libc::c_char; 2],
        >(b"/\0"))[0 as libc::c_int as usize] as libc::c_int || dirs.is_null()
    {
        return filename;
    }
    return findPath(dirs, maxdirlen, filename);
}
#[no_mangle]
pub unsafe extern "C" fn maptoken(
    mut p: *mut libc::c_char,
    mut name: *mut *mut libc::c_char,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    loop {
        q = *name.offset(i as isize);
        if q.is_null() {
            break;
        }
        if !p.is_null() && strcmp(p, q) == 0 {
            break;
        }
        i += 1;
    }
    return *val.offset(i as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mapBool(mut p: *const libc::c_char, mut dflt: bool) -> bool {
    if p.is_null() || *p as libc::c_int == '\0' as i32 {
        return dflt;
    }
    if strcasecmp(p, b"false\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int != 0;
    }
    if strcasecmp(p, b"no\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int != 0;
    }
    if strcasecmp(p, b"true\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int != 0;
    }
    if strcasecmp(p, b"yes\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int != 0;
    }
    if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        return atoi(p) != 0 as libc::c_int
    } else {
        return dflt
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapbool(mut p: *const libc::c_char) -> bool {
    return mapBool(p, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn dotneato_closest(
    mut spl: *mut splines,
    mut pt: pointf,
) -> pointf {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut besti: libc::c_int = 0;
    let mut bestj: libc::c_int = 0;
    let mut bestdist2: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    let mut dlow2: libc::c_double = 0.;
    let mut dhigh2: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut high: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut c: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut pt2: pointf = pointf { x: 0., y: 0. };
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    bestj = -(1 as libc::c_int);
    besti = bestj;
    bestdist2 = 1e+38f64;
    i = 0 as libc::c_int;
    while i < (*spl).size {
        bz = *((*spl).list).offset(i as isize);
        j = 0 as libc::c_int;
        while j < bz.size {
            let mut b: pointf = pointf { x: 0., y: 0. };
            b.x = (*(bz.list).offset(j as isize)).x;
            b.y = (*(bz.list).offset(j as isize)).y;
            d2 = (b.x - pt.x) * (b.x - pt.x) + (b.y - pt.y) * (b.y - pt.y);
            if bestj == -(1 as libc::c_int) || d2 < bestdist2 {
                besti = i;
                bestj = j;
                bestdist2 = d2;
            }
            j += 1;
        }
        i += 1;
    }
    bz = *((*spl).list).offset(besti as isize);
    if bestj == bz.size - 1 as libc::c_int {
        bestj -= 1;
    }
    j = 3 as libc::c_int * (bestj / 3 as libc::c_int);
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        c[k as usize].x = (*(bz.list).offset((j + k) as isize)).x;
        c[k as usize].y = (*(bz.list).offset((j + k) as isize)).y;
        k += 1;
    }
    low = 0.0f64;
    high = 1.0f64;
    dlow2 = (c[0 as libc::c_int as usize].x - pt.x)
        * (c[0 as libc::c_int as usize].x - pt.x)
        + (c[0 as libc::c_int as usize].y - pt.y)
            * (c[0 as libc::c_int as usize].y - pt.y);
    dhigh2 = (c[3 as libc::c_int as usize].x - pt.x)
        * (c[3 as libc::c_int as usize].x - pt.x)
        + (c[3 as libc::c_int as usize].y - pt.y)
            * (c[3 as libc::c_int as usize].y - pt.y);
    loop {
        t = (low + high) / 2.0f64;
        pt2 = Bezier(
            c.as_mut_ptr(),
            3 as libc::c_int,
            t,
            0 as *mut pointf,
            0 as *mut pointf,
        );
        if fabs(dlow2 - dhigh2) < 1.0f64 {
            break;
        }
        if fabs(high - low) < 0.00001f64 {
            break;
        }
        if dlow2 < dhigh2 {
            high = t;
            dhigh2 = (pt2.x - pt.x) * (pt2.x - pt.x) + (pt2.y - pt.y) * (pt2.y - pt.y);
        } else {
            low = t;
            dlow2 = (pt2.x - pt.x) * (pt2.x - pt.x) + (pt2.y - pt.y) * (pt2.y - pt.y);
        }
    }
    return pt2;
}
#[no_mangle]
pub unsafe extern "C" fn spline_at_y(
    mut spl: *mut splines,
    mut y: libc::c_double,
) -> pointf {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut low: libc::c_double = 0.;
    let mut high: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut c: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut p: pointf = pointf { x: 0., y: 0. };
    static mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    i = 0 as libc::c_int;
    while i < (*spl).size {
        bz = *((*spl).list).offset(i as isize);
        if (*(bz.list).offset((bz.size - 1 as libc::c_int) as isize)).y <= y
            && y <= (*(bz.list).offset(0 as libc::c_int as isize)).y
        {
            break;
        }
        i += 1;
    }
    if y > (*(bz.list).offset(0 as libc::c_int as isize)).y {
        p = *(bz.list).offset(0 as libc::c_int as isize);
    } else if y < (*(bz.list).offset((bz.size - 1 as libc::c_int) as isize)).y {
        p = *(bz.list).offset((bz.size - 1 as libc::c_int) as isize);
    } else {
        i = 0 as libc::c_int;
        while i < bz.size {
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                if (*(bz.list).offset((i + j) as isize)).y <= y
                    && y <= (*(bz.list).offset((i + j + 1 as libc::c_int) as isize)).y
                {
                    break;
                }
                if (*(bz.list).offset((i + j) as isize)).y >= y
                    && y >= (*(bz.list).offset((i + j + 1 as libc::c_int) as isize)).y
                {
                    break;
                }
                j += 1;
            }
            if j < 3 as libc::c_int {
                break;
            }
            i += 3 as libc::c_int;
        }
        if i < bz.size {} else {
            __assert_fail(
                b"i < bz.size\0" as *const u8 as *const libc::c_char,
                b"utils.c\0" as *const u8 as *const libc::c_char,
                551 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"pointf spline_at_y(splines *, double)\0"))
                    .as_ptr(),
            );
        }
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            c[j as usize].x = (*(bz.list).offset((i + j) as isize)).x;
            c[j as usize].y = (*(bz.list).offset((i + j) as isize)).y;
            if j > 0 as libc::c_int
                && c[j as usize].y > c[(j - 1 as libc::c_int) as usize].y
            {
                c[j as usize].y = c[(j - 1 as libc::c_int) as usize].y;
            }
            j += 1;
        }
        low = 0.0f64;
        high = 1.0f64;
        loop {
            t = (low + high) / 2.0f64;
            p = Bezier(
                c.as_mut_ptr(),
                3 as libc::c_int,
                t,
                0 as *mut pointf,
                0 as *mut pointf,
            );
            d = p.y - y;
            if fabs(d) <= 1 as libc::c_int as libc::c_double {
                break;
            }
            if d < 0 as libc::c_int as libc::c_double {
                high = t;
            } else {
                low = t;
            }
        }
    }
    p.y = y;
    return p;
}
static mut Tflag: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn gvToggle(mut s: libc::c_int) {
    Tflag = (Tflag == 0) as libc::c_int;
    signal(10 as libc::c_int, Some(gvToggle as unsafe extern "C" fn(libc::c_int) -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn test_toggle() -> libc::c_int {
    return Tflag;
}
#[no_mangle]
pub unsafe extern "C" fn common_init_node(mut n: *mut node_t) {
    let mut fi: fontinfo = fontinfo {
        fontsize: 0.,
        fontname: 0 as *mut libc::c_char,
        fontcolor: 0 as *mut libc::c_char,
    };
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .width = late_double(n as *mut libc::c_void, N_width, 0.75f64, 0.01f64);
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .height = late_double(n as *mut libc::c_void, N_height, 0.5f64, 0.02f64);
    let ref mut fresh34 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape;
    *fresh34 = bind_shape(
        late_nnstring(
            n as *mut libc::c_void,
            N_shape,
            b"ellipse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        n,
    );
    str = agxget(n as *mut libc::c_void, N_label);
    fi.fontsize = late_double(n as *mut libc::c_void, N_fontsize, 14.0f64, 1.0f64);
    fi
        .fontname = late_nnstring(
        n as *mut libc::c_void,
        N_fontname,
        b"Times-Roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    fi
        .fontcolor = late_nnstring(
        n as *mut libc::c_void,
        N_fontcolor,
        b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh35 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label;
    *fresh35 = make_label(
        n as *mut libc::c_void,
        str,
        (if aghtmlstr(str) != 0 {
            (1 as libc::c_int) << 1 as libc::c_int
        } else {
            (0 as libc::c_int) << 1 as libc::c_int
        })
            | (if shapeOf(n) as libc::c_uint == SH_RECORD as libc::c_int as libc::c_uint
            {
                (2 as libc::c_int) << 1 as libc::c_int
            } else {
                (0 as libc::c_int) << 1 as libc::c_int
            }),
        fi.fontsize,
        fi.fontname,
        fi.fontcolor,
    );
    if !N_xlabel.is_null()
        && {
            str = agxget(n as *mut libc::c_void, N_xlabel);
            !str.is_null()
        } && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        let ref mut fresh36 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .xlabel;
        *fresh36 = make_label(
            n as *mut libc::c_void,
            str,
            if aghtmlstr(str) != 0 {
                (1 as libc::c_int) << 1 as libc::c_int
            } else {
                (0 as libc::c_int) << 1 as libc::c_int
            },
            fi.fontsize,
            fi.fontname,
            fi.fontcolor,
        );
        let ref mut fresh37 = (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t))
            .data as *mut Agraphinfo_t))
            .has_labels;
        *fresh37 = (*fresh37 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as libc::c_uchar;
    }
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .showboxes = late_int(
        n as *mut libc::c_void,
        N_showboxes,
        0 as libc::c_int,
        0 as libc::c_int,
    ) as libc::c_uchar;
    ((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns).initfn)
        .expect("non-null function pointer")(n);
}
unsafe extern "C" fn initFontEdgeAttr(mut e: *mut edge_t, mut fi: *mut fontinfo) {
    (*fi).fontsize = late_double(e as *mut libc::c_void, E_fontsize, 14.0f64, 1.0f64);
    let ref mut fresh38 = (*fi).fontname;
    *fresh38 = late_nnstring(
        e as *mut libc::c_void,
        E_fontname,
        b"Times-Roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh39 = (*fi).fontcolor;
    *fresh39 = late_nnstring(
        e as *mut libc::c_void,
        E_fontcolor,
        b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn initFontLabelEdgeAttr(
    mut e: *mut edge_t,
    mut fi: *mut fontinfo,
    mut lfi: *mut fontinfo,
) {
    if ((*fi).fontname).is_null() {
        initFontEdgeAttr(e, fi);
    }
    (*lfi)
        .fontsize = late_double(
        e as *mut libc::c_void,
        E_labelfontsize,
        (*fi).fontsize,
        1.0f64,
    );
    let ref mut fresh40 = (*lfi).fontname;
    *fresh40 = late_nnstring(e as *mut libc::c_void, E_labelfontname, (*fi).fontname);
    let ref mut fresh41 = (*lfi).fontcolor;
    *fresh41 = late_nnstring(e as *mut libc::c_void, E_labelfontcolor, (*fi).fontcolor);
}
unsafe extern "C" fn noClip(mut e: *mut edge_t, mut sym: *mut attrsym_t) -> bool {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rv: bool = 0 as libc::c_int != 0;
    if !sym.is_null() {
        str = agxget(e as *mut libc::c_void, sym);
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            rv = !mapbool(str);
        } else {
            rv = 0 as libc::c_int != 0;
        }
    }
    return rv;
}
unsafe extern "C" fn chkPort(
    mut pf: Option::<
        unsafe extern "C" fn(*mut node_t, *mut libc::c_char, *mut libc::c_char) -> port,
    >,
    mut n: *mut node_t,
    mut s: *mut libc::c_char,
) -> port {
    let mut pt: port = port {
        p: pointf { x: 0., y: 0. },
        theta: 0.,
        bp: 0 as *mut boxf,
        defined: false,
        constrained: false,
        clip: false,
        dyna: false,
        order: 0,
        side: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !s.is_null() {
        cp = strchr(s, ':' as i32);
    }
    if !cp.is_null() {
        *cp = '\0' as i32 as libc::c_char;
        pt = pf
            .expect(
                "non-null function pointer",
            )(n, s, cp.offset(1 as libc::c_int as isize));
        *cp = ':' as i32 as libc::c_char;
        pt.name = cp.offset(1 as libc::c_int as isize);
    } else {
        pt = pf.expect("non-null function pointer")(n, s, 0 as *mut libc::c_char);
        pt.name = s;
    }
    return pt;
}
#[no_mangle]
pub unsafe extern "C" fn common_init_edge(mut e: *mut edge_t) -> libc::c_int {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut fi: fontinfo = fontinfo {
        fontsize: 0.,
        fontname: 0 as *mut libc::c_char,
        fontcolor: 0 as *mut libc::c_char,
    };
    let mut lfi: fontinfo = fontinfo {
        fontsize: 0.,
        fontname: 0 as *mut libc::c_char,
        fontcolor: 0 as *mut libc::c_char,
    };
    let mut sg: *mut graph_t = agraphof(
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut libc::c_void,
    );
    fi.fontname = 0 as *mut libc::c_char;
    lfi.fontname = 0 as *mut libc::c_char;
    if !E_label.is_null()
        && {
            str = agxget(e as *mut libc::c_void, E_label);
            !str.is_null()
        } && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        r = 1 as libc::c_int;
        initFontEdgeAttr(e, &mut fi);
        let ref mut fresh42 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .label;
        *fresh42 = make_label(
            e as *mut libc::c_void,
            str,
            if aghtmlstr(str) != 0 {
                (1 as libc::c_int) << 1 as libc::c_int
            } else {
                (0 as libc::c_int) << 1 as libc::c_int
            },
            fi.fontsize,
            fi.fontname,
            fi.fontcolor,
        );
        let ref mut fresh43 = (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .has_labels;
        *fresh43 = (*fresh43 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
            as libc::c_uchar;
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .label_ontop = (if mapbool(
            late_string(
                e as *mut libc::c_void,
                E_label_float,
                b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        ) as libc::c_int != 0
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_char;
    }
    if !E_xlabel.is_null()
        && {
            str = agxget(e as *mut libc::c_void, E_xlabel);
            !str.is_null()
        } && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        if (fi.fontname).is_null() {
            initFontEdgeAttr(e, &mut fi);
        }
        let ref mut fresh44 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .xlabel;
        *fresh44 = make_label(
            e as *mut libc::c_void,
            str,
            if aghtmlstr(str) != 0 {
                (1 as libc::c_int) << 1 as libc::c_int
            } else {
                (0 as libc::c_int) << 1 as libc::c_int
            },
            fi.fontsize,
            fi.fontname,
            fi.fontcolor,
        );
        let ref mut fresh45 = (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .has_labels;
        *fresh45 = (*fresh45 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int)
            as libc::c_uchar;
    }
    if !E_headlabel.is_null()
        && {
            str = agxget(e as *mut libc::c_void, E_headlabel);
            !str.is_null()
        } && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        initFontLabelEdgeAttr(e, &mut fi, &mut lfi);
        let ref mut fresh46 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_label;
        *fresh46 = make_label(
            e as *mut libc::c_void,
            str,
            if aghtmlstr(str) != 0 {
                (1 as libc::c_int) << 1 as libc::c_int
            } else {
                (0 as libc::c_int) << 1 as libc::c_int
            },
            lfi.fontsize,
            lfi.fontname,
            lfi.fontcolor,
        );
        let ref mut fresh47 = (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .has_labels;
        *fresh47 = (*fresh47 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
            as libc::c_uchar;
    }
    if !E_taillabel.is_null()
        && {
            str = agxget(e as *mut libc::c_void, E_taillabel);
            !str.is_null()
        } && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        if (lfi.fontname).is_null() {
            initFontLabelEdgeAttr(e, &mut fi, &mut lfi);
        }
        let ref mut fresh48 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_label;
        *fresh48 = make_label(
            e as *mut libc::c_void,
            str,
            if aghtmlstr(str) != 0 {
                (1 as libc::c_int) << 1 as libc::c_int
            } else {
                (0 as libc::c_int) << 1 as libc::c_int
            },
            lfi.fontsize,
            lfi.fontname,
            lfi.fontcolor,
        );
        let ref mut fresh49 = (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .has_labels;
        *fresh49 = (*fresh49 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            as libc::c_uchar;
    }
    str = agget(
        e as *mut libc::c_void,
        b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if str.is_null() {
        str = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .has_port = 1 as libc::c_int != 0;
    }
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .tail_port = chkPort(
        (*(*(*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .shape)
            .fns)
            .portfn,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node,
        str,
    );
    if noClip(e, E_tailclip) {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .clip = 0 as libc::c_int != 0;
    }
    str = agget(
        e as *mut libc::c_void,
        b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if str.is_null() {
        str = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .has_port = 1 as libc::c_int != 0;
    }
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .head_port = chkPort(
        (*(*(*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .shape)
            .fns)
            .portfn,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
        str,
    );
    if noClip(e, E_headclip) {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .clip = 0 as libc::c_int != 0;
    }
    return r;
}
unsafe extern "C" fn addLabelBB(
    mut bb: boxf,
    mut lp: *mut textlabel_t,
    mut flipxy: bool,
) -> boxf {
    let mut width: libc::c_double = 0.;
    let mut height: libc::c_double = 0.;
    let mut p: pointf = (*lp).pos;
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    if flipxy {
        height = (*lp).dimen.x;
        width = (*lp).dimen.y;
    } else {
        width = (*lp).dimen.x;
        height = (*lp).dimen.y;
    }
    min = p.x - width / 2.0f64;
    max = p.x + width / 2.0f64;
    if min < bb.LL.x {
        bb.LL.x = min;
    }
    if max > bb.UR.x {
        bb.UR.x = max;
    }
    min = p.y - height / 2.0f64;
    max = p.y + height / 2.0f64;
    if min < bb.LL.y {
        bb.LL.y = min;
    }
    if max > bb.UR.y {
        bb.UR.y = max;
    }
    return bb;
}
#[no_mangle]
pub unsafe extern "C" fn polyBB(mut poly: *mut polygon_t) -> boxf {
    let mut i: libc::c_int = 0;
    let mut sides: libc::c_int = (*poly).sides;
    let mut peris: libc::c_int = if (*poly).peripheries > 1 as libc::c_int {
        (*poly).peripheries
    } else {
        1 as libc::c_int
    };
    let mut verts: *mut pointf = ((*poly).vertices)
        .offset(((peris - 1 as libc::c_int) * sides) as isize);
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    bb.UR = *verts.offset(0 as libc::c_int as isize);
    bb.LL = bb.UR;
    i = 1 as libc::c_int;
    while i < sides {
        bb
            .LL
            .x = if bb.LL.x < (*verts.offset(i as isize)).x {
            bb.LL.x
        } else {
            (*verts.offset(i as isize)).x
        };
        bb
            .LL
            .y = if bb.LL.y < (*verts.offset(i as isize)).y {
            bb.LL.y
        } else {
            (*verts.offset(i as isize)).y
        };
        bb
            .UR
            .x = if bb.UR.x > (*verts.offset(i as isize)).x {
            bb.UR.x
        } else {
            (*verts.offset(i as isize)).x
        };
        bb
            .UR
            .y = if bb.UR.y > (*verts.offset(i as isize)).y {
            bb.UR.y
        } else {
            (*verts.offset(i as isize)).y
        };
        i += 1;
    }
    return bb;
}
#[no_mangle]
pub unsafe extern "C" fn updateBB(mut g: *mut graph_t, mut lp: *mut textlabel_t) {
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb = addLabelBB(
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
        lp,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int & 1 as libc::c_int != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn compute_bb(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut BF: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut ptf: pointf = pointf { x: 0., y: 0. };
    let mut s2: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if agnnodes(g) == 0 as libc::c_int
        && (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster
            == 0 as libc::c_int
    {
        bb
            .LL = pointfof(
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
        bb
            .UR = pointfof(
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
        return;
    }
    bb
        .LL = pointfof(
        2147483647 as libc::c_int as libc::c_double,
        2147483647 as libc::c_int as libc::c_double,
    );
    bb
        .UR = pointfof(
        -(2147483647 as libc::c_int) as libc::c_double,
        -(2147483647 as libc::c_int) as libc::c_double,
    );
    n = agfstnode(g);
    while !n.is_null() {
        ptf = coord(n);
        s2
            .x = ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw) / 2.0f64;
        s2.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
        b.LL = sub_pointf(ptf, s2);
        b.UR = add_pointf(ptf, s2);
        bb.LL.x = (if bb.LL.x < b.LL.x { bb.LL.x } else { b.LL.x });
        bb.LL.y = (if bb.LL.y < b.LL.y { bb.LL.y } else { b.LL.y });
        bb.UR.x = (if bb.UR.x > b.UR.x { bb.UR.x } else { b.UR.x });
        bb.UR.y = (if bb.UR.y > b.UR.y { bb.UR.y } else { b.UR.y });
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null()
            && (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).set
                as libc::c_int != 0
        {
            bb = addLabelBB(
                bb,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel,
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                    & 0x3 as libc::c_int & 1 as libc::c_int != 0,
            );
        }
        e = agfstout(g, n);
        while !e.is_null() {
            if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
                i = 0 as libc::c_int;
                while i
                    < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size
                {
                    j = 0 as libc::c_int;
                    while j
                        < (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
                            .list)
                            .offset(i as isize))
                            .size
                    {
                        ptf = *((*((*(*((*(e as *mut Agobj_t)).data
                            as *mut Agedgeinfo_t))
                            .spl)
                            .list)
                            .offset(i as isize))
                            .list)
                            .offset(j as isize);
                        bb.LL.x = (if bb.LL.x < ptf.x { bb.LL.x } else { ptf.x });
                        bb.LL.y = (if bb.LL.y < ptf.y { bb.LL.y } else { ptf.y });
                        bb.UR.x = (if bb.UR.x > ptf.x { bb.UR.x } else { ptf.x });
                        bb.UR.y = (if bb.UR.y > ptf.y { bb.UR.y } else { ptf.y });
                        j += 1;
                    }
                    i += 1;
                }
                if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .is_null()
                    && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).set
                        as libc::c_int != 0
                {
                    bb = addLabelBB(
                        bb,
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label,
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                            & 0x3 as libc::c_int & 1 as libc::c_int != 0,
                    );
                }
                if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label)
                    .is_null()
                    && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .head_label)
                        .set as libc::c_int != 0
                {
                    bb = addLabelBB(
                        bb,
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label,
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                            & 0x3 as libc::c_int & 1 as libc::c_int != 0,
                    );
                }
                if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label)
                    .is_null()
                    && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .tail_label)
                        .set as libc::c_int != 0
                {
                    bb = addLabelBB(
                        bb,
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label,
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                            & 0x3 as libc::c_int & 1 as libc::c_int != 0,
                    );
                }
                if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel)
                    .is_null()
                    && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel)
                        .set as libc::c_int != 0
                {
                    bb = addLabelBB(
                        bb,
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel,
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                            & 0x3 as libc::c_int & 1 as libc::c_int != 0,
                    );
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        BF
            .LL
            .x = (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agraphinfo_t))
            .bb
            .LL
            .x;
        BF
            .LL
            .y = (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agraphinfo_t))
            .bb
            .LL
            .y;
        BF
            .UR
            .x = (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agraphinfo_t))
            .bb
            .UR
            .x;
        BF
            .UR
            .y = (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(i as isize) as *mut Agobj_t))
            .data as *mut Agraphinfo_t))
            .bb
            .UR
            .y;
        bb.LL.x = (if bb.LL.x < BF.LL.x { bb.LL.x } else { BF.LL.x });
        bb.LL.y = (if bb.LL.y < BF.LL.y { bb.LL.y } else { BF.LL.y });
        bb.UR.x = (if bb.UR.x > BF.UR.x { bb.UR.x } else { BF.UR.x });
        bb.UR.y = (if bb.UR.y > BF.UR.y { bb.UR.y } else { BF.UR.y });
        i += 1;
    }
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set
            as libc::c_int != 0
    {
        bb = addLabelBB(
            bb,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                & 0x3 as libc::c_int & 1 as libc::c_int != 0,
        );
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb = bb;
}
#[no_mangle]
pub unsafe extern "C" fn is_a_cluster(mut g: *mut Agraph_t) -> bool {
    return g == (*g).root
        || strncasecmp(
            agnameof(g as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        || mapBool(
            agget(
                g as *mut libc::c_void,
                b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            0 as libc::c_int != 0,
        ) as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn setAttr(
    mut g: *mut graph_t,
    mut obj: *mut libc::c_void,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ap: *mut Agsym_t,
) -> *mut Agsym_t {
    if ap.is_null() {
        match agobjkind(obj) {
            0 => {
                ap = agattr(
                    g,
                    0 as libc::c_int,
                    name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            1 => {
                ap = agattr(
                    g,
                    1 as libc::c_int,
                    name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            2 => {
                ap = agattr(
                    g,
                    2 as libc::c_int,
                    name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
    }
    agxset(obj, ap, value);
    return ap;
}
unsafe extern "C" fn clustNode(
    mut n: *mut node_t,
    mut cg: *mut graph_t,
    mut xb: *mut agxbuf,
    mut clg: *mut graph_t,
) -> *mut node_t {
    let mut cn: *mut node_t = 0 as *mut node_t;
    static mut idx: libc::c_int = 0 as libc::c_int;
    let fresh50 = idx;
    idx = idx + 1;
    agxbprint(
        xb,
        b"__%d:%s\0" as *const u8 as *const libc::c_char,
        fresh50,
        agnameof(cg as *mut libc::c_void),
    );
    cn = agnode(agroot(cg as *mut libc::c_void), agxbuse(xb), 1 as libc::c_int);
    agbindrec(
        cn as *mut libc::c_void,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    (*((*(cn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .clustnode = 1 as libc::c_int != 0;
    agsubnode(cg, cn, 1 as libc::c_int);
    agsubnode(clg, n, 1 as libc::c_int);
    N_label = setAttr(
        agraphof(cn as *mut libc::c_void),
        cn as *mut libc::c_void,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        N_label,
    );
    N_style = setAttr(
        agraphof(cn as *mut libc::c_void),
        cn as *mut libc::c_void,
        b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"invis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        N_style,
    );
    N_shape = setAttr(
        agraphof(cn as *mut libc::c_void),
        cn as *mut libc::c_void,
        b"shape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"box\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        N_shape,
    );
    return cn;
}
unsafe extern "C" fn cmpItem(
    mut d: *mut Dt_t,
    mut p1: *mut *mut libc::c_void,
    mut p2: *mut *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if *p1.offset(0 as libc::c_int as isize) < *p2.offset(0 as libc::c_int as isize) {
        return -(1 as libc::c_int)
    } else if *p1.offset(0 as libc::c_int as isize)
            > *p2.offset(0 as libc::c_int as isize)
        {
        return 1 as libc::c_int
    } else if *p1.offset(1 as libc::c_int as isize)
            < *p2.offset(1 as libc::c_int as isize)
        {
        return -(1 as libc::c_int)
    } else if *p1.offset(1 as libc::c_int as isize)
            > *p2.offset(1 as libc::c_int as isize)
        {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn newItem(
    mut d: *mut Dt_t,
    mut objp: *mut item,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut newp: *mut item = zmalloc(::std::mem::size_of::<item>() as libc::c_ulong)
        as *mut item;
    let ref mut fresh51 = (*newp).p[0 as libc::c_int as usize];
    *fresh51 = (*objp).p[0 as libc::c_int as usize];
    let ref mut fresh52 = (*newp).p[1 as libc::c_int as usize];
    *fresh52 = (*objp).p[1 as libc::c_int as usize];
    let ref mut fresh53 = (*newp).t;
    *fresh53 = (*objp).t;
    let ref mut fresh54 = (*newp).h;
    *fresh54 = (*objp).h;
    return newp as *mut libc::c_void;
}
unsafe extern "C" fn freeItem(
    mut d: *mut Dt_t,
    mut obj: *mut item,
    mut disc: *mut Dtdisc_t,
) {
    free(obj as *mut libc::c_void);
}
static mut mapDisc: Dtdisc_t = Dtdisc_t {
    key: 0,
    size: 0,
    link: 0,
    makef: None,
    freef: None,
    comparf: None,
    hashf: None,
    memoryf: None,
    eventf: None,
};
unsafe extern "C" fn cloneEdge(
    mut e: *mut edge_t,
    mut ct: *mut node_t,
    mut ch: *mut node_t,
) -> *mut edge_t {
    let mut g: *mut graph_t = agraphof(ct as *mut libc::c_void);
    let mut ce: *mut edge_t = agedge(
        g,
        ct,
        ch,
        0 as *mut libc::c_char,
        1 as libc::c_int,
    );
    agbindrec(
        ce as *mut libc::c_void,
        b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agcopyattr(e as *mut libc::c_void, ce as *mut libc::c_void);
    (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .compound = (0 as libc::c_int == 0) as libc::c_int as libc::c_char;
    return ce;
}
unsafe extern "C" fn insertEdge(
    mut map: *mut Dt_t,
    mut t: *mut libc::c_void,
    mut h: *mut libc::c_void,
    mut e: *mut edge_t,
) {
    let mut dummy: item = item {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed_1 { _hash: 0 },
        },
        p: [0 as *mut libc::c_void; 2],
        t: 0 as *mut node_t,
        h: 0 as *mut node_t,
    };
    dummy.p[0 as libc::c_int as usize] = t;
    dummy.p[1 as libc::c_int as usize] = h;
    dummy
        .t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    dummy
        .h = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, &mut dummy as *mut item as *mut libc::c_void, 0o1 as libc::c_int);
    dummy.p[0 as libc::c_int as usize] = h;
    dummy.p[1 as libc::c_int as usize] = t;
    dummy
        .t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    dummy
        .h = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, &mut dummy as *mut item as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn mapEdge(mut map: *mut Dt_t, mut e: *mut edge_t) -> *mut item {
    let mut key: [*mut libc::c_void; 2] = [0 as *mut libc::c_void; 2];
    key[0 as libc::c_int
        as usize] = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node as *mut libc::c_void;
    key[1 as libc::c_int
        as usize] = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node as *mut libc::c_void;
    return (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        map,
        &mut key as *mut [*mut libc::c_void; 2] as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut item;
}
unsafe extern "C" fn checkCompound(
    mut e: *mut edge_t,
    mut clg: *mut graph_t,
    mut xb: *mut agxbuf,
    mut map: *mut Dt_t,
    mut cmap: *mut Dt_t,
) -> libc::c_int {
    let mut tg: *mut graph_t = 0 as *mut graph_t;
    let mut hg: *mut graph_t = 0 as *mut graph_t;
    let mut cn: *mut node_t = 0 as *mut node_t;
    let mut cn1: *mut node_t = 0 as *mut node_t;
    let mut t: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    let mut h: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let mut ce: *mut edge_t = 0 as *mut edge_t;
    let mut ip: *mut item = 0 as *mut item;
    if (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
        return 0 as libc::c_int;
    }
    tg = if strncmp(
        agnameof(t as *mut libc::c_void),
        b"cluster\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        0 as *mut Agraph_t
    } else {
        findCluster(cmap, agnameof(t as *mut libc::c_void))
    };
    hg = if strncmp(
        agnameof(h as *mut libc::c_void),
        b"cluster\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        0 as *mut Agraph_t
    } else {
        findCluster(cmap, agnameof(h as *mut libc::c_void))
    };
    if tg.is_null() && hg.is_null() {
        return 0 as libc::c_int;
    }
    if tg == hg {
        agerr(
            AGWARN,
            b"cluster cycle %s -- %s not supported\n\0" as *const u8
                as *const libc::c_char,
            agnameof(t as *mut libc::c_void),
            agnameof(t as *mut libc::c_void),
        );
        return 0 as libc::c_int;
    }
    ip = mapEdge(map, e);
    if !ip.is_null() {
        cloneEdge(e, (*ip).t, (*ip).h);
        return 1 as libc::c_int;
    }
    if !hg.is_null() {
        if !tg.is_null() {
            if agcontains(hg, tg as *mut libc::c_void) != 0 {
                agerr(
                    AGWARN,
                    b"tail cluster %s inside head cluster %s\n\0" as *const u8
                        as *const libc::c_char,
                    agnameof(tg as *mut libc::c_void),
                    agnameof(hg as *mut libc::c_void),
                );
                return 0 as libc::c_int;
            }
            if agcontains(tg, hg as *mut libc::c_void) != 0 {
                agerr(
                    AGWARN,
                    b"head cluster %s inside tail cluster %s\n\0" as *const u8
                        as *const libc::c_char,
                    agnameof(hg as *mut libc::c_void),
                    agnameof(tg as *mut libc::c_void),
                );
                return 0 as libc::c_int;
            }
            cn = clustNode(t, tg, xb, clg);
            cn1 = clustNode(h, hg, xb, clg);
            ce = cloneEdge(e, cn, cn1);
            insertEdge(map, t as *mut libc::c_void, h as *mut libc::c_void, ce);
        } else {
            if agcontains(hg, t as *mut libc::c_void) != 0 {
                agerr(
                    AGWARN,
                    b"tail node %s inside head cluster %s\n\0" as *const u8
                        as *const libc::c_char,
                    agnameof(t as *mut libc::c_void),
                    agnameof(hg as *mut libc::c_void),
                );
                return 0 as libc::c_int;
            }
            cn = clustNode(h, hg, xb, clg);
            ce = cloneEdge(e, t, cn);
            insertEdge(map, t as *mut libc::c_void, h as *mut libc::c_void, ce);
        }
    } else {
        if agcontains(tg, h as *mut libc::c_void) != 0 {
            agerr(
                AGWARN,
                b"head node %s inside tail cluster %s\n\0" as *const u8
                    as *const libc::c_char,
                agnameof(h as *mut libc::c_void),
                agnameof(tg as *mut libc::c_void),
            );
            return 0 as libc::c_int;
        }
        cn = clustNode(t, tg, xb, clg);
        ce = cloneEdge(e, cn, h);
        insertEdge(map, t as *mut libc::c_void, h as *mut libc::c_void, ce);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn num_clust_edges(mut g: *mut graph_t) -> libc::c_int {
    let mut cl_info: *mut cl_edge_t = aggetrec(
        g as *mut libc::c_void,
        b"cl_edge_info\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut cl_edge_t;
    if !cl_info.is_null() {
        return (*cl_info).n_cluster_edges
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn processClusterEdges(mut g: *mut graph_t) {
    let mut num_cl_edges: libc::c_int = 0 as libc::c_int;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut nxt: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut clg: *mut graph_t = 0 as *mut graph_t;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut map: *mut Dt_t = 0 as *mut Dt_t;
    let mut cmap: *mut Dt_t = mkClustMap(g);
    let mut buf: [libc::c_char; 128] = [0; 128];
    map = dtopen(&mut mapDisc, Dtoset);
    clg = agsubg(
        g,
        b"__clusternodes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    agbindrec(
        clg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agxbinit(&mut xb, 128 as libc::c_int as libc::c_uint, buf.as_mut_ptr());
    n = agfstnode(g);
    while !n.is_null() {
        if !(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
            e = agfstout(g, n);
            while !e.is_null() {
                num_cl_edges += checkCompound(e, clg, &mut xb, map, cmap);
                e = agnxtout(g, e);
            }
        }
        n = agnxtnode(g, n);
    }
    agxbfree(&mut xb);
    dtclose(map);
    n = agfstnode(clg);
    while !n.is_null() {
        nxt = agnxtnode(clg, n);
        agdelete(g, n as *mut libc::c_void);
        n = nxt;
    }
    agclose(clg);
    if num_cl_edges != 0 {
        let mut cl_info: *mut cl_edge_t = 0 as *mut cl_edge_t;
        cl_info = agbindrec(
            g as *mut libc::c_void,
            b"cl_edge_info\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<cl_edge_t>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        ) as *mut cl_edge_t;
        (*cl_info).n_cluster_edges = num_cl_edges;
    }
    dtclose(cmap);
}
unsafe extern "C" fn mapN(mut n: *mut node_t, mut clg: *mut graph_t) -> *mut node_t {
    let mut nn: *mut node_t = 0 as *mut node_t;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut graph_t = agraphof(n as *mut libc::c_void);
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    if !(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
        return n;
    }
    agsubnode(clg, n, 1 as libc::c_int);
    name = strchr(agnameof(n as *mut libc::c_void), ':' as i32);
    if !name.is_null() {} else {
        __assert_fail(
            b"name\0" as *const u8 as *const libc::c_char,
            b"utils.c\0" as *const u8 as *const libc::c_char,
            1209 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"node_t *mapN(node_t *, graph_t *)\0"))
                .as_ptr(),
        );
    }
    name = name.offset(1);
    nn = agnode(g, name, 0 as libc::c_int);
    if !nn.is_null() {
        return nn;
    }
    nn = agnode(g, name, 1 as libc::c_int);
    agbindrec(
        nn as *mut libc::c_void,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    (*((*(nn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .clustnode = 1 as libc::c_int != 0;
    sym = agnxtattr(g, 1 as libc::c_int, 0 as *mut Agsym_t);
    while !sym.is_null() {
        if agxget(nn as *mut libc::c_void, sym) != (*sym).defval {
            agxset(nn as *mut libc::c_void, sym, (*sym).defval);
        }
        sym = agnxtattr(g, 1 as libc::c_int, sym);
    }
    return nn;
}
unsafe extern "C" fn undoCompound(mut e: *mut edge_t, mut clg: *mut graph_t) {
    let mut t: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    let mut h: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let mut ntail: *mut node_t = 0 as *mut node_t;
    let mut nhead: *mut node_t = 0 as *mut node_t;
    let mut ce: *mut edge_t = 0 as *mut edge_t;
    ntail = mapN(t, clg);
    nhead = mapN(h, clg);
    ce = cloneEdge(e, ntail, nhead);
    let ref mut fresh55 = (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
    *fresh55 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
    let ref mut fresh56 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
    *fresh56 = 0 as *mut splines;
    let ref mut fresh57 = (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
    *fresh57 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
    let ref mut fresh58 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
    *fresh58 = 0 as *mut textlabel_t;
    let ref mut fresh59 = (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel;
    *fresh59 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel;
    let ref mut fresh60 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel;
    *fresh60 = 0 as *mut textlabel_t;
    let ref mut fresh61 = (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .head_label;
    *fresh61 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label;
    let ref mut fresh62 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .head_label;
    *fresh62 = 0 as *mut textlabel_t;
    let ref mut fresh63 = (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .tail_label;
    *fresh63 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label;
    let ref mut fresh64 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .tail_label;
    *fresh64 = 0 as *mut textlabel_t;
    gv_cleanup_edge(e);
}
#[no_mangle]
pub unsafe extern "C" fn undoClusterEdges(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut nextn: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut clg: *mut graph_t = 0 as *mut graph_t;
    let mut elist: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut ecnt: libc::c_int = num_clust_edges(g);
    let mut i: libc::c_int = 0 as libc::c_int;
    if ecnt == 0 {
        return;
    }
    clg = agsubg(
        g,
        b"__clusternodes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    agbindrec(
        clg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    elist = gcalloc(
        ecnt as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
    ) as *mut *mut edge_t;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).compound != 0 {
                let fresh65 = i;
                i = i + 1;
                let ref mut fresh66 = *elist.offset(fresh65 as isize);
                *fresh66 = e;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    if i == ecnt {} else {
        __assert_fail(
            b"i == ecnt\0" as *const u8 as *const libc::c_char,
            b"utils.c\0" as *const u8 as *const libc::c_char,
            1276 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void undoClusterEdges(graph_t *)\0"))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < ecnt {
        undoCompound(*elist.offset(i as isize), clg);
        i += 1;
    }
    free(elist as *mut libc::c_void);
    n = agfstnode(clg);
    while !n.is_null() {
        nextn = agnxtnode(clg, n);
        gv_cleanup_node(n);
        agdelete(g, n as *mut libc::c_void);
        n = nextn;
    }
    agclose(clg);
}
#[no_mangle]
pub unsafe extern "C" fn safe_dcl(
    mut g: *mut graph_t,
    mut obj_kind: libc::c_int,
    mut name: *mut libc::c_char,
    mut def: *mut libc::c_char,
) -> *mut attrsym_t {
    let mut a: *mut attrsym_t = agattr(g, obj_kind, name, 0 as *const libc::c_char);
    if a.is_null() {
        a = agattr(g, obj_kind, name, def);
    }
    return a;
}
unsafe extern "C" fn comp_entities(
    mut e1: *const libc::c_void,
    mut e2: *const libc::c_void,
) -> libc::c_int {
    return strcmp((*(e1 as *const entities_s)).name, (*(e2 as *const entities_s)).name);
}
#[no_mangle]
pub unsafe extern "C" fn scanEntity(
    mut t: *mut libc::c_char,
    mut xb: *mut agxbuf,
) -> *mut libc::c_char {
    let mut endp: *mut libc::c_char = strchr(t, ';' as i32);
    let mut key: entities_s = entities_s {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        value: 0,
    };
    let mut res: *mut entities_s = 0 as *mut entities_s;
    let mut len: libc::c_int = 0;
    let mut buf: [libc::c_char; 9] = [0; 9];
    agxbputc(xb, '&' as i32 as libc::c_char);
    if endp.is_null() {
        return t;
    }
    len = endp.offset_from(t) as libc::c_long as libc::c_int;
    if len > 8 as libc::c_int || len < 2 as libc::c_int {
        return t;
    }
    strncpy(buf.as_mut_ptr(), t, len as libc::c_ulong);
    buf[len as usize] = '\0' as i32 as libc::c_char;
    key.name = buf.as_mut_ptr();
    res = bsearch(
        &mut key as *mut entities_s as *const libc::c_void,
        entities.as_mut_ptr() as *const libc::c_void,
        252 as libc::c_int as size_t,
        ::std::mem::size_of::<entities_s>() as libc::c_ulong,
        Some(
            comp_entities
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut entities_s;
    if res.is_null() {
        return t;
    }
    agxbprint(xb, b"#%d;\0" as *const u8 as *const libc::c_char, (*res).value);
    return endp.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn htmlEntity(mut s: *mut *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: entities_s = entities_s {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        value: 0,
    };
    let mut res: *mut entities_s = 0 as *mut entities_s;
    let mut entity_name_buf: [libc::c_char; 9] = [0; 9];
    let mut str: *mut libc::c_uchar = *(s as *mut *mut libc::c_uchar);
    let mut byte: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    byte = *str as libc::c_uint;
    if byte == '#' as i32 as libc::c_uint {
        byte = *str.offset(1 as libc::c_int as isize) as libc::c_uint;
        if byte == 'x' as i32 as libc::c_uint || byte == 'X' as i32 as libc::c_uint {
            i = 2 as libc::c_int;
            while i < 8 as libc::c_int {
                byte = *str.offset(i as isize) as libc::c_uint;
                if byte >= 'A' as i32 as libc::c_uint
                    && byte <= 'F' as i32 as libc::c_uint
                {
                    byte = byte
                        .wrapping_sub('A' as i32 as libc::c_uint)
                        .wrapping_add(10 as libc::c_int as libc::c_uint);
                } else if byte >= 'a' as i32 as libc::c_uint
                        && byte <= 'f' as i32 as libc::c_uint
                    {
                    byte = byte
                        .wrapping_sub('a' as i32 as libc::c_uint)
                        .wrapping_add(10 as libc::c_int as libc::c_uint);
                } else {
                    if !(byte >= '0' as i32 as libc::c_uint
                        && byte <= '9' as i32 as libc::c_uint)
                    {
                        break;
                    }
                    byte = byte.wrapping_sub('0' as i32 as libc::c_uint);
                }
                n = n * 16 as libc::c_int + byte as libc::c_int;
                i += 1;
            }
        } else {
            i = 1 as libc::c_int;
            while i < 8 as libc::c_int {
                byte = *str.offset(i as isize) as libc::c_uint;
                if !(byte >= '0' as i32 as libc::c_uint
                    && byte <= '9' as i32 as libc::c_uint)
                {
                    break;
                }
                n = n * 10 as libc::c_int + (byte as libc::c_int - '0' as i32);
                i += 1;
            }
        }
        if byte == ';' as i32 as libc::c_uint {
            str = str.offset((i + 1 as libc::c_int) as isize);
        } else {
            n = 0 as libc::c_int;
        }
    } else {
        p = entity_name_buf.as_mut_ptr();
        key.name = p;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            byte = *str.offset(i as isize) as libc::c_uint;
            if byte == '\0' as i32 as libc::c_uint {
                break;
            }
            if byte == ';' as i32 as libc::c_uint {
                let fresh67 = p;
                p = p.offset(1);
                *fresh67 = '\0' as i32 as libc::c_char;
                res = bsearch(
                    &mut key as *mut entities_s as *const libc::c_void,
                    entities.as_mut_ptr() as *const libc::c_void,
                    252 as libc::c_int as size_t,
                    ::std::mem::size_of::<entities_s>() as libc::c_ulong,
                    Some(
                        (Some(
                            comp_entities
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> libc::c_int,
                        ))
                            .expect("non-null function pointer"),
                    ),
                ) as *mut entities_s;
                if !res.is_null() {
                    n = (*res).value;
                    str = str.offset((i + 1 as libc::c_int) as isize);
                }
                break;
            } else {
                let fresh68 = p;
                p = p.offset(1);
                *fresh68 = byte as libc::c_char;
                i += 1;
            }
        }
    }
    *s = str as *mut libc::c_char;
    return n;
}
unsafe extern "C" fn cvtAndAppend(
    mut c: libc::c_uchar,
    mut xb: *mut agxbuf,
) -> libc::c_uchar {
    let mut buf: [libc::c_char; 2] = [0; 2];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    buf[0 as libc::c_int as usize] = c as libc::c_char;
    buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    s = latin1ToUTF8(buf.as_mut_ptr());
    p = s;
    len = strlen(s) as libc::c_int;
    loop {
        let fresh69 = len;
        len = len - 1;
        if !(fresh69 > 1 as libc::c_int) {
            break;
        }
        let fresh70 = p;
        p = p.offset(1);
        agxbputc(xb, *fresh70);
    }
    c = *p as libc::c_uchar;
    free(s as *mut libc::c_void);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn htmlEntityUTF8(
    mut s: *mut libc::c_char,
    mut g: *mut graph_t,
) -> *mut libc::c_char {
    static mut lastg: *mut graph_t = 0 as *const graph_t as *mut graph_t;
    static mut warned: bool = false;
    let mut ns: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut c: libc::c_uchar = 0;
    let mut v: libc::c_uint = 0;
    let mut uc: libc::c_int = 0;
    let mut ui: libc::c_int = 0;
    if lastg != g {
        lastg = g;
        warned = 0 as libc::c_int != 0;
    }
    agxbinit(&mut xb, 0 as libc::c_int as libc::c_uint, 0 as *mut libc::c_char);
    loop {
        let fresh71 = s;
        s = s.offset(1);
        c = *(fresh71 as *mut libc::c_uchar);
        if !(c != 0) {
            break;
        }
        if (c as libc::c_int) < 0xc0 as libc::c_int {
            uc = 0 as libc::c_int;
        } else if (c as libc::c_int) < 0xe0 as libc::c_int {
            uc = 1 as libc::c_int;
        } else if (c as libc::c_int) < 0xf0 as libc::c_int {
            uc = 2 as libc::c_int;
        } else if (c as libc::c_int) < 0xf8 as libc::c_int {
            uc = 3 as libc::c_int;
        } else {
            uc = -(1 as libc::c_int);
            if !warned {
                agerr(
                    AGWARN,
                    b"UTF8 codes > 4 bytes are not currently supported (graph %s) - treated as Latin-1. Perhaps \"-Gcharset=latin1\" is needed?\n\0"
                        as *const u8 as *const libc::c_char,
                    agnameof(g as *mut libc::c_void),
                );
                warned = 1 as libc::c_int != 0;
            }
            c = cvtAndAppend(c, &mut xb);
        }
        if uc == 0 as libc::c_int && c as libc::c_int == '&' as i32 {
            v = htmlEntity(&mut s) as libc::c_uint;
            if v != 0 {
                if v < 0x7f as libc::c_int as libc::c_uint {
                    c = v as libc::c_uchar;
                } else if v < 0x7ff as libc::c_int as libc::c_uint {
                    agxbputc(
                        &mut xb,
                        (v >> 6 as libc::c_int | 0xc0 as libc::c_int as libc::c_uint)
                            as libc::c_char,
                    );
                    c = (v & 0x3f as libc::c_int as libc::c_uint
                        | 0x80 as libc::c_int as libc::c_uint) as libc::c_uchar;
                } else {
                    agxbputc(
                        &mut xb,
                        (v >> 12 as libc::c_int | 0xe0 as libc::c_int as libc::c_uint)
                            as libc::c_char,
                    );
                    agxbputc(
                        &mut xb,
                        (v >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
                            | 0x80 as libc::c_int as libc::c_uint) as libc::c_char,
                    );
                    c = (v & 0x3f as libc::c_int as libc::c_uint
                        | 0x80 as libc::c_int as libc::c_uint) as libc::c_uchar;
                }
            }
        } else {
            ui = 0 as libc::c_int;
            while ui < uc {
                if *s as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int {
                    agxbputc(&mut xb, c as libc::c_char);
                    let fresh72 = s;
                    s = s.offset(1);
                    c = *(fresh72 as *mut libc::c_uchar);
                    ui += 1;
                } else {
                    if !warned {
                        agerr(
                            AGWARN,
                            b"Invalid %d-byte UTF8 found in input of graph %s - treated as Latin-1. Perhaps \"-Gcharset=latin1\" is needed?\n\0"
                                as *const u8 as *const libc::c_char,
                            uc + 1 as libc::c_int,
                            agnameof(g as *mut libc::c_void),
                        );
                        warned = 1 as libc::c_int != 0;
                    }
                    c = cvtAndAppend(c, &mut xb);
                    break;
                }
            }
        }
        agxbputc(&mut xb, c as libc::c_char);
    }
    ns = agxbdisown(&mut xb);
    agxbfree(&mut xb);
    return ns;
}
#[no_mangle]
pub unsafe extern "C" fn latin1ToUTF8(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut ns: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut v: libc::c_uint = 0;
    agxbinit(&mut xb, 0 as libc::c_int as libc::c_uint, 0 as *mut libc::c_char);
    loop {
        let fresh73 = s;
        s = s.offset(1);
        v = *(fresh73 as *mut libc::c_uchar) as libc::c_uint;
        if !(v != 0) {
            break;
        }
        if v == '&' as i32 as libc::c_uint {
            v = htmlEntity(&mut s) as libc::c_uint;
            if v == 0 {
                v = '&' as i32 as libc::c_uint;
            }
        }
        if v < 0x7f as libc::c_int as libc::c_uint {
            agxbputc(&mut xb, v as libc::c_char);
        } else if v < 0x7ff as libc::c_int as libc::c_uint {
            agxbputc(
                &mut xb,
                (v >> 6 as libc::c_int | 0xc0 as libc::c_int as libc::c_uint)
                    as libc::c_char,
            );
            agxbputc(
                &mut xb,
                (v & 0x3f as libc::c_int as libc::c_uint
                    | 0x80 as libc::c_int as libc::c_uint) as libc::c_char,
            );
        } else {
            agxbputc(
                &mut xb,
                (v >> 12 as libc::c_int | 0xe0 as libc::c_int as libc::c_uint)
                    as libc::c_char,
            );
            agxbputc(
                &mut xb,
                (v >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
                    | 0x80 as libc::c_int as libc::c_uint) as libc::c_char,
            );
            agxbputc(
                &mut xb,
                (v & 0x3f as libc::c_int as libc::c_uint
                    | 0x80 as libc::c_int as libc::c_uint) as libc::c_char,
            );
        }
    }
    ns = agxbdisown(&mut xb);
    agxbfree(&mut xb);
    return ns;
}
#[no_mangle]
pub unsafe extern "C" fn utf8ToLatin1(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut ns: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut c: libc::c_uchar = 0;
    let mut outc: libc::c_uchar = 0;
    agxbinit(&mut xb, 0 as libc::c_int as libc::c_uint, 0 as *mut libc::c_char);
    loop {
        let fresh74 = s;
        s = s.offset(1);
        c = *(fresh74 as *mut libc::c_uchar);
        if !(c != 0) {
            break;
        }
        if (c as libc::c_int) < 0x7f as libc::c_int {
            agxbputc(&mut xb, c as libc::c_char);
        } else {
            outc = ((c as libc::c_int & 0x3 as libc::c_int) << 6 as libc::c_int)
                as libc::c_uchar;
            let fresh75 = s;
            s = s.offset(1);
            c = *(fresh75 as *mut libc::c_uchar);
            outc = (outc as libc::c_int | c as libc::c_int & 0x3f as libc::c_int)
                as libc::c_uchar;
            agxbputc(&mut xb, outc as libc::c_char);
        }
    }
    ns = agxbdisown(&mut xb);
    agxbfree(&mut xb);
    return ns;
}
#[no_mangle]
pub unsafe extern "C" fn overlap_node(mut n: *mut node_t, mut b: boxf) -> bool {
    let mut ictxt: inside_t = inside_t {
        a: C2RustUnnamed_9 {
            p: 0 as *mut pointf,
            r: 0 as *mut libc::c_double,
        },
    };
    let mut p: pointf = pointf { x: 0., y: 0. };
    if !(b.UR.x >= (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).bb.LL.x
        && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).bb.UR.x >= b.LL.x
        && b.UR.y >= (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).bb.LL.y
        && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).bb.UR.y >= b.LL.y)
    {
        return 0 as libc::c_int != 0;
    }
    p = sub_pointf(
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        mid_pointf(b.UR, b.LL),
    );
    ictxt.s.n = n;
    ictxt.s.bp = 0 as *mut boxf;
    return ((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
        .insidefn)
        .expect("non-null function pointer")(&mut ictxt, p);
}
#[no_mangle]
pub unsafe extern "C" fn overlap_label(mut lp: *mut textlabel_t, mut b: boxf) -> bool {
    let mut s: pointf = pointf { x: 0., y: 0. };
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    s.x = (*lp).dimen.x / 2.0f64;
    s.y = (*lp).dimen.y / 2.0f64;
    bb.LL = sub_pointf((*lp).pos, s);
    bb.UR = add_pointf((*lp).pos, s);
    return b.UR.x >= bb.LL.x && bb.UR.x >= b.LL.x && b.UR.y >= bb.LL.y
        && bb.UR.y >= b.LL.y;
}
unsafe extern "C" fn overlap_arrow(
    mut p: pointf,
    mut u: pointf,
    mut scale: libc::c_double,
    mut b: boxf,
) -> bool {
    return b.UR.x >= (arrow_bb(p, u, scale)).LL.x
        && (arrow_bb(p, u, scale)).UR.x >= b.LL.x
        && b.UR.y >= (arrow_bb(p, u, scale)).LL.y
        && (arrow_bb(p, u, scale)).UR.y >= b.LL.y;
}
unsafe extern "C" fn overlap_bezier(mut bz: bezier, mut b: boxf) -> bool {
    let mut i: libc::c_int = 0;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut u: pointf = pointf { x: 0., y: 0. };
    if bz.size != 0 {} else {
        __assert_fail(
            b"bz.size\0" as *const u8 as *const libc::c_char,
            b"utils.c\0" as *const u8 as *const libc::c_char,
            1621 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"_Bool overlap_bezier(bezier, boxf)\0"))
                .as_ptr(),
        );
    }
    u = *(bz.list).offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < bz.size {
        p = *(bz.list).offset(i as isize);
        if lineToBox(p, u, b) != -(1 as libc::c_int) {
            return 1 as libc::c_int != 0;
        }
        u = p;
        i += 1;
    }
    if bz.sflag != 0 {
        if overlap_arrow(
            bz.sp,
            *(bz.list).offset(0 as libc::c_int as isize),
            1 as libc::c_int as libc::c_double,
            b,
        ) {
            return 1 as libc::c_int != 0;
        }
    }
    if bz.eflag != 0 {
        if overlap_arrow(
            bz.ep,
            *(bz.list).offset((bz.size - 1 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_double,
            b,
        ) {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn overlap_edge(mut e: *mut edge_t, mut b: boxf) -> bool {
    let mut i: libc::c_int = 0;
    let mut spl: *mut splines = 0 as *mut splines;
    let mut lp: *mut textlabel_t = 0 as *mut textlabel_t;
    spl = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
    if !spl.is_null() && boxf_overlap((*spl).bb, b) != 0 {
        i = 0 as libc::c_int;
        while i < (*spl).size {
            if overlap_bezier(*((*spl).list).offset(i as isize), b) {
                return 1 as libc::c_int != 0;
            }
            i += 1;
        }
    }
    lp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
    if !lp.is_null() && overlap_label(lp, b) as libc::c_int != 0 {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn edgeType(
    mut s: *const libc::c_char,
    mut dflt: libc::c_int,
) -> libc::c_int {
    if s.is_null()
        || strcmp(s, b"\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return dflt;
    }
    if *s as libc::c_int == '0' as i32 {
        return (1 as libc::c_int) << 1 as libc::c_int
    } else {
        if *s as libc::c_int >= '1' as i32 && *s as libc::c_int <= '9' as i32 {
            return (5 as libc::c_int) << 1 as libc::c_int
        } else {
            if strcasecmp(s, b"curved\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                return (2 as libc::c_int) << 1 as libc::c_int
            } else {
                if strcasecmp(s, b"compound\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    return (6 as libc::c_int) << 1 as libc::c_int
                } else {
                    if strcasecmp(s, b"false\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        return (1 as libc::c_int) << 1 as libc::c_int
                    } else {
                        if strcasecmp(s, b"line\0" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            return (1 as libc::c_int) << 1 as libc::c_int
                        } else {
                            if strcasecmp(
                                s,
                                b"none\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                            {
                                return (0 as libc::c_int) << 1 as libc::c_int
                            } else {
                                if strcasecmp(
                                    s,
                                    b"no\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    return (1 as libc::c_int) << 1 as libc::c_int
                                } else {
                                    if strcasecmp(
                                        s,
                                        b"ortho\0" as *const u8 as *const libc::c_char,
                                    ) == 0 as libc::c_int
                                    {
                                        return (4 as libc::c_int) << 1 as libc::c_int
                                    } else {
                                        if strcasecmp(
                                            s,
                                            b"polyline\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int
                                        {
                                            return (3 as libc::c_int) << 1 as libc::c_int
                                        } else {
                                            if strcasecmp(
                                                s,
                                                b"spline\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                            {
                                                return (5 as libc::c_int) << 1 as libc::c_int
                                            } else {
                                                if strcasecmp(
                                                    s,
                                                    b"true\0" as *const u8 as *const libc::c_char,
                                                ) == 0 as libc::c_int
                                                {
                                                    return (5 as libc::c_int) << 1 as libc::c_int
                                                } else {
                                                    if strcasecmp(
                                                        s,
                                                        b"yes\0" as *const u8 as *const libc::c_char,
                                                    ) == 0 as libc::c_int
                                                    {
                                                        return (5 as libc::c_int) << 1 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    agerr(
        AGWARN,
        b"Unknown \"splines\" value: \"%s\" - ignored\n\0" as *const u8
            as *const libc::c_char,
        s,
    );
    return dflt;
}
#[no_mangle]
pub unsafe extern "C" fn setEdgeType(mut g: *mut graph_t, mut dflt: libc::c_int) {
    let mut s: *mut libc::c_char = agget(
        g as *mut libc::c_void,
        b"splines\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut et: libc::c_int = 0;
    if s.is_null() {
        et = dflt;
    } else if *s as libc::c_int == '\0' as i32 {
        et = (0 as libc::c_int) << 1 as libc::c_int;
    } else {
        et = edgeType(s, dflt);
    }
    let ref mut fresh76 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags;
    *fresh76 = (*fresh76 as libc::c_int | et) as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn get_gradient_points(
    mut A: *mut pointf,
    mut G: *mut pointf,
    mut n: libc::c_int,
    mut angle: libc::c_double,
    mut flags: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    let mut min: pointf = pointf { x: 0., y: 0. };
    let mut max: pointf = pointf { x: 0., y: 0. };
    let mut center: pointf = pointf { x: 0., y: 0. };
    let mut isRadial: libc::c_int = flags & 1 as libc::c_int;
    let mut isRHS: libc::c_int = flags & 2 as libc::c_int;
    if n == 2 as libc::c_int {
        rx = (*A.offset(1 as libc::c_int as isize)).x
            - (*A.offset(0 as libc::c_int as isize)).x;
        ry = (*A.offset(1 as libc::c_int as isize)).y
            - (*A.offset(0 as libc::c_int as isize)).y;
        min.x = (*A.offset(0 as libc::c_int as isize)).x - rx;
        max.x = (*A.offset(0 as libc::c_int as isize)).x + rx;
        min.y = (*A.offset(0 as libc::c_int as isize)).y - ry;
        max.y = (*A.offset(0 as libc::c_int as isize)).y + ry;
    } else {
        max.x = (*A.offset(0 as libc::c_int as isize)).x;
        min.x = max.x;
        max.y = (*A.offset(0 as libc::c_int as isize)).y;
        min.y = max.y;
        i = 0 as libc::c_int;
        while i < n {
            min
                .x = if (*A.offset(i as isize)).x < min.x {
                (*A.offset(i as isize)).x
            } else {
                min.x
            };
            min
                .y = if (*A.offset(i as isize)).y < min.y {
                (*A.offset(i as isize)).y
            } else {
                min.y
            };
            max
                .x = if (*A.offset(i as isize)).x > max.x {
                (*A.offset(i as isize)).x
            } else {
                max.x
            };
            max
                .y = if (*A.offset(i as isize)).y > max.y {
                (*A.offset(i as isize)).y
            } else {
                max.y
            };
            i += 1;
        }
    }
    center.x = min.x + (max.x - min.x) / 2 as libc::c_int as libc::c_double;
    center.y = min.y + (max.y - min.y) / 2 as libc::c_int as libc::c_double;
    if isRadial != 0 {
        let mut inner_r: libc::c_double = 0.;
        let mut outer_r: libc::c_double = 0.;
        outer_r = hypot(center.x - min.x, center.y - min.y);
        inner_r = outer_r / 4.0f64;
        if isRHS != 0 {
            (*G.offset(0 as libc::c_int as isize)).y = center.y;
        } else {
            (*G.offset(0 as libc::c_int as isize)).y = -center.y;
        }
        (*G.offset(0 as libc::c_int as isize)).x = center.x;
        (*G.offset(1 as libc::c_int as isize)).x = inner_r;
        (*G.offset(1 as libc::c_int as isize)).y = outer_r;
    } else {
        let mut half_x: libc::c_double = max.x - center.x;
        let mut half_y: libc::c_double = max.y - center.y;
        let mut sina: libc::c_double = sin(angle);
        let mut cosa: libc::c_double = cos(angle);
        if isRHS != 0 {
            (*G.offset(0 as libc::c_int as isize)).y = center.y - half_y * sina;
            (*G.offset(1 as libc::c_int as isize)).y = center.y + half_y * sina;
        } else {
            (*G.offset(0 as libc::c_int as isize))
                .y = -center.y + (max.y - center.y) * sin(angle);
            (*G.offset(1 as libc::c_int as isize))
                .y = -center.y - (center.y - min.y) * sin(angle);
        }
        (*G.offset(0 as libc::c_int as isize)).x = center.x - half_x * cosa;
        (*G.offset(1 as libc::c_int as isize)).x = center.x + half_x * cosa;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gv_free_splines(mut e: *mut edge_t) {
    let mut i: libc::c_int = 0;
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
        i = 0 as libc::c_int;
        while i < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
            free(
                (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                    .offset(i as isize))
                    .list as *mut libc::c_void,
            );
            i += 1;
        }
        free(
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list
                as *mut libc::c_void,
        );
        free(
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl
                as *mut libc::c_void,
        );
    }
    let ref mut fresh77 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
    *fresh77 = 0 as *mut splines;
}
#[no_mangle]
pub unsafe extern "C" fn gv_cleanup_edge(mut e: *mut edge_t) {
    free(
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).path.ps
            as *mut libc::c_void,
    );
    gv_free_splines(e);
    free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label);
    free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel);
    free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label);
    free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label);
    agdelrec(
        e as *mut libc::c_void,
        b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gv_cleanup_node(mut n: *mut node_t) {
    free((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos as *mut libc::c_void);
    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null() {
        ((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns).freefn)
            .expect("non-null function pointer")(n);
    }
    free_label((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label);
    free_label((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel);
    agdelrec(
        n as *mut libc::c_void,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gv_nodesize(mut n: *mut node_t, mut flip: bool) {
    let mut w: libc::c_double = 0.;
    if flip {
        w = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double;
        let ref mut fresh78 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
        *fresh78 = w / 2 as libc::c_int as libc::c_double;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw = *fresh78;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .ht = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double;
    } else {
        w = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double;
        let ref mut fresh79 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
        *fresh79 = w / 2 as libc::c_int as libc::c_double;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw = *fresh79;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .ht = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double;
    };
}
unsafe extern "C" fn free_clust(
    mut dt: *mut Dt_t,
    mut clp: *mut clust_t,
    mut disc: *mut Dtdisc_t,
) {
    free(clp as *mut libc::c_void);
}
static mut strDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut clust_t, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_clust
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut clust_t,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn fillMap(mut g: *mut Agraph_t, mut map: *mut Dt_t) {
    let mut cl: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut c: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: *mut clust_t = 0 as *mut clust_t;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        cl = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(c as isize);
        s = agnameof(cl as *mut libc::c_void);
        if !((Some(((*map).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(map, s as *mut libc::c_void, 0o1000 as libc::c_int))
            .is_null()
        {
            agerr(
                AGWARN,
                b"Two clusters named %s - the second will be ignored\n\0" as *const u8
                    as *const libc::c_char,
                s,
            );
        } else {
            ip = zmalloc(::std::mem::size_of::<clust_t>() as libc::c_ulong)
                as *mut clust_t;
            let ref mut fresh80 = (*ip).name;
            *fresh80 = s;
            let ref mut fresh81 = (*ip).clp;
            *fresh81 = cl;
            (Some(((*map).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(map, ip as *mut libc::c_void, 0o1 as libc::c_int);
        }
        fillMap(cl, map);
        c += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mkClustMap(mut g: *mut Agraph_t) -> *mut Dt_t {
    let mut map: *mut Dt_t = dtopen(&mut strDisc, Dtoset);
    fillMap(g, map);
    return map;
}
#[no_mangle]
pub unsafe extern "C" fn findCluster(
    mut map: *mut Dt_t,
    mut name: *mut libc::c_char,
) -> *mut Agraph_t {
    let mut clp: *mut clust_t = (Some(
        ((*map).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(map, name as *mut libc::c_void, 0o1000 as libc::c_int) as *mut clust_t;
    if !clp.is_null() { return (*clp).clp } else { return 0 as *mut Agraph_t };
}
unsafe extern "C" fn run_static_initializers() {
    mapDisc = {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut item,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
                >,
                Dtmake_f,
            >(
                Some(
                    newItem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut item,
                            *mut Dtdisc_t,
                        ) -> *mut libc::c_void,
                ),
            ),
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut item, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freeItem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut item,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut *mut libc::c_void,
                        *mut *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    cmpItem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut *mut libc::c_void,
                            *mut *mut libc::c_void,
                            *mut Dtdisc_t,
                        ) -> libc::c_int,
                ),
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
