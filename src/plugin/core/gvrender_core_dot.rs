#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type GVC_s;
    pub type htmllabel_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut AgIoDisc: Agiodisc_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agsafeset(
        obj: *mut libc::c_void,
        name: *mut libc::c_char,
        value: *const libc::c_char,
        def: *const libc::c_char,
    ) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn attach_attrs_and_arrows(
        _: *mut graph_t,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    );
    fn write_plain(job: *mut GVJ_t, g: *mut graph_t, f: *mut FILE, extend: bool);
    fn yDir(y: libc::c_double) -> libc::c_double;
    fn undoClusterEdges(g: *mut graph_t);
    fn safe_dcl(
        g: *mut graph_t,
        obj_kind: libc::c_int,
        name: *mut libc::c_char,
        def: *mut libc::c_char,
    ) -> *mut attrsym_t;
    fn get_gradient_points(
        A: *mut pointf,
        G: *mut pointf,
        n: libc::c_int,
        angle: libc::c_double,
        flags: libc::c_int,
    );
    fn attach_attrs(g: *mut graph_t);
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvflush(job: *mut GVJ_t) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
    pub initialize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub format: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub finalize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvrender_engine_s {
    pub begin_job: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_job: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_graph: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_graph: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_layer: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub end_layer: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_page: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_page: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_cluster: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_cluster: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_nodes: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_nodes: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edges: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edges: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_node: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_node: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edge: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edge: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_anchor: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub end_anchor: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_label: Option::<unsafe extern "C" fn(*mut GVJ_t, label_type) -> ()>,
    pub end_label: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub textspan: Option::<
        unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
    >,
    pub resolve_color: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> ()>,
    pub ellipse: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
    >,
    pub polygon: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int, libc::c_int) -> (),
    >,
    pub beziercurve: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut pointf,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub polyline: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
    >,
    pub comment: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> ()>,
    pub library_shape: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut pointf,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
}
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
pub type PostscriptAlias = _PostscriptAlias;
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
pub type label_type = libc::c_uint;
pub const LABEL_HTML: label_type = 1;
pub const LABEL_PLAIN: label_type = 0;
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
pub type Agedge_t = Agedge_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
pub type attrsym_t = Agsym_s;
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
    pub u: C2RustUnnamed_5,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub txt: C2RustUnnamed_6,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub span: *mut textspan_t,
    pub nspans: libc::c_short,
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
pub type usershape_t = usershape_s;
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
pub type format_type = libc::c_uint;
pub const FORMAT_XDOT14: format_type = 6;
pub const FORMAT_XDOT12: format_type = 5;
pub const FORMAT_XDOT: format_type = 4;
pub const FORMAT_PLAIN_EXT: format_type = 3;
pub const FORMAT_PLAIN: format_type = 2;
pub const FORMAT_CANON: format_type = 1;
pub const FORMAT_DOT: format_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_state_t {
    pub g_draw: *mut attrsym_t,
    pub g_l_draw: *mut attrsym_t,
    pub n_draw: *mut attrsym_t,
    pub n_l_draw: *mut attrsym_t,
    pub e_draw: *mut attrsym_t,
    pub h_draw: *mut attrsym_t,
    pub t_draw: *mut attrsym_t,
    pub e_l_draw: *mut attrsym_t,
    pub hl_draw: *mut attrsym_t,
    pub tl_draw: *mut attrsym_t,
    pub buf: [[libc::c_char; 8192]; 8],
    pub version: libc::c_ushort,
    pub version_s: *mut libc::c_char,
}
pub type putstrfn = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
>;
pub type flushfn = Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
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
unsafe extern "C" fn gv_alloc(mut size: size_t) -> *mut libc::c_void {
    return gv_calloc(1 as libc::c_int as size_t, size);
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
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
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
unsafe extern "C" fn agxbpop(mut xb: *mut agxbuf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if (*xb).ptr > (*xb).buf {
        let ref mut fresh4 = (*xb).ptr;
        let fresh5 = *fresh4;
        *fresh4 = (*fresh4).offset(-1);
        c = *fresh5 as libc::c_int;
        return c;
    } else {
        return -(1 as libc::c_int)
    };
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
    let ref mut fresh6 = (*xb).buf;
    *fresh6 = nbuf;
    let ref mut fresh7 = (*xb).ptr;
    *fresh7 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh8 = (*xb).eptr;
    *fresh8 = ((*xb).buf).offset(nsize as isize);
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
        let ref mut fresh9 = (*xb).ptr;
        *fresh9 = (*fresh9).offset(result as size_t as isize);
    }
    return result;
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
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = (*fresh10).offset(ssz as isize);
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
    let ref mut fresh11 = (*xb).ptr;
    let fresh12 = *fresh11;
    *fresh11 = (*fresh11).offset(1);
    *fresh12 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh13 = (*xb).ptr;
    *fresh13 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn agxblen(mut xb: *const agxbuf) -> size_t {
    return ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
}
static mut xbuf: [agxbuf; 8] = [agxbuf {
    buf: 0 as *const libc::c_char as *mut libc::c_char,
    ptr: 0 as *const libc::c_char as *mut libc::c_char,
    eptr: 0 as *const libc::c_char as *mut libc::c_char,
    dyna: 0,
}; 8];
static mut xbufs: [*mut agxbuf; 12] = [0 as *const agxbuf as *mut agxbuf; 12];
static mut penwidth: [libc::c_double; 12] = [
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
];
static mut textflags: [libc::c_uint; 12] = [0; 12];
static mut xd: *mut xdot_state_t = 0 as *const xdot_state_t as *mut xdot_state_t;
unsafe extern "C" fn xdot_str_xbuf(
    mut xb: *mut agxbuf,
    mut pfx: *mut libc::c_char,
    mut s: *const libc::c_char,
) {
    agxbprint(
        xb,
        b"%s%zu -%s \0" as *const u8 as *const libc::c_char,
        pfx,
        strlen(s),
        s,
    );
}
unsafe extern "C" fn xdot_str(
    mut job: *mut GVJ_t,
    mut pfx: *mut libc::c_char,
    mut s: *const libc::c_char,
) {
    let mut emit_state: emit_state_t = (*(*job).obj).emit_state;
    xdot_str_xbuf(xbufs[emit_state as usize], pfx, s);
}
unsafe extern "C" fn xdot_trim_zeros(
    mut buf: *mut libc::c_char,
    mut addSpace: libc::c_int,
) {
    let mut dotp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    dotp = strchr(buf, '.' as i32);
    if !dotp.is_null() {
        p = dotp.offset(1 as libc::c_int as isize);
        while *p != 0 {
            p = p.offset(1);
        }
        p = p.offset(-1);
        while *p as libc::c_int == '0' as i32 {
            let fresh14 = p;
            p = p.offset(-1);
            *fresh14 = '\0' as i32 as libc::c_char;
        }
        if *p as libc::c_int == '.' as i32 {
            *p = '\0' as i32 as libc::c_char;
        } else {
            p = p.offset(1);
        }
    } else if addSpace != 0 {
        p = buf.offset(strlen(buf) as isize);
    }
    if addSpace != 0 {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = ' ' as i32 as libc::c_char;
        *p = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn xdot_fmt_num(mut buf: *mut libc::c_char, mut v: libc::c_double) {
    if v > -0.00000001f64 && v < 0.00000001f64 {
        strcpy(buf, b"0 \0" as *const u8 as *const libc::c_char);
        return;
    }
    sprintf(buf, b"%.02f\0" as *const u8 as *const libc::c_char, v);
    xdot_trim_zeros(buf, 1 as libc::c_int);
}
unsafe extern "C" fn xdot_point(mut xb: *mut agxbuf, mut p: pointf) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    xdot_fmt_num(buf.as_mut_ptr(), p.x);
    agxbput(xb, buf.as_mut_ptr());
    xdot_fmt_num(buf.as_mut_ptr(), yDir(p.y));
    agxbput(xb, buf.as_mut_ptr());
}
unsafe extern "C" fn xdot_num(mut xb: *mut agxbuf, mut v: libc::c_double) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    xdot_fmt_num(buf.as_mut_ptr(), v);
    agxbput(xb, buf.as_mut_ptr());
}
unsafe extern "C" fn xdot_points(
    mut job: *mut GVJ_t,
    mut c: libc::c_char,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    let mut emit_state: emit_state_t = (*(*job).obj).emit_state;
    let mut i: libc::c_int = 0;
    agxbprint(
        xbufs[emit_state as usize],
        b"%c %d \0" as *const u8 as *const libc::c_char,
        c as libc::c_int,
        n,
    );
    i = 0 as libc::c_int;
    while i < n {
        xdot_point(xbufs[emit_state as usize], *A.offset(i as isize));
        i += 1;
    }
}
unsafe extern "C" fn color2str(mut rgba: *mut libc::c_uchar) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 10] = [0; 10];
    if *rgba.offset(3 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            *rgba.offset(0 as libc::c_int as isize) as libc::c_int,
            *rgba.offset(1 as libc::c_int as isize) as libc::c_int,
            *rgba.offset(2 as libc::c_int as isize) as libc::c_int,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            b"#%02x%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            *rgba.offset(0 as libc::c_int as isize) as libc::c_int,
            *rgba.offset(1 as libc::c_int as isize) as libc::c_int,
            *rgba.offset(2 as libc::c_int as isize) as libc::c_int,
            *rgba.offset(3 as libc::c_int as isize) as libc::c_int,
        );
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn xdot_pencolor(mut job: *mut GVJ_t) {
    xdot_str(
        job,
        b"c \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        color2str(((*(*job).obj).pencolor.u.rgba).as_mut_ptr()),
    );
}
unsafe extern "C" fn xdot_fillcolor(mut job: *mut GVJ_t) {
    xdot_str(
        job,
        b"C \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        color2str(((*(*job).obj).fillcolor.u.rgba).as_mut_ptr()),
    );
}
unsafe extern "C" fn xdot_style(mut job: *mut GVJ_t) {
    let mut buf0: [libc::c_char; 8192] = [0; 8192];
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        eptr: 0 as *const libc::c_char as *mut libc::c_char,
        dyna: 0,
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut more: libc::c_int = 0;
    agxbinit(&mut xb, 8192 as libc::c_int as libc::c_uint, buf0.as_mut_ptr());
    if (*(*job).obj).penwidth != penwidth[(*(*job).obj).emit_state as usize] {
        penwidth[(*(*job).obj).emit_state as usize] = (*(*job).obj).penwidth;
        agxbput(&mut xb, b"setlinewidth(\0" as *const u8 as *const libc::c_char);
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%.3f\0" as *const u8 as *const libc::c_char,
            (*(*job).obj).penwidth,
        );
        xdot_trim_zeros(buf.as_mut_ptr(), 0 as libc::c_int);
        agxbprint(
            &mut xb as *mut agxbuf,
            b"%s)\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        xdot_str(
            job,
            b"S \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(&mut xb),
        );
    }
    s = (*(*job).obj).rawstyle;
    if s.is_null() {
        return;
    }
    loop {
        let fresh16 = s;
        s = s.offset(1);
        p = *fresh16;
        if p.is_null() {
            break;
        }
        if strcmp(p, b"filled\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(p, b"bold\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(p, b"setlinewidth\0" as *const u8 as *const libc::c_char) == 0
        {
            continue;
        }
        agxbput(&mut xb, p);
        while *p != 0 {
            p = p.offset(1);
        }
        p = p.offset(1);
        if *p != 0 {
            agxbputc(&mut xb, '(' as i32 as libc::c_char);
            more = 0 as libc::c_int;
            while *p != 0 {
                if more != 0 {
                    agxbputc(&mut xb, ',' as i32 as libc::c_char);
                }
                agxbput(&mut xb, p);
                while *p != 0 {
                    p = p.offset(1);
                }
                p = p.offset(1);
                more += 1;
            }
            agxbputc(&mut xb, ')' as i32 as libc::c_char);
        }
        xdot_str(
            job,
            b"S \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(&mut xb),
        );
    }
    agxbfree(&mut xb);
}
unsafe extern "C" fn put_escaping_backslashes(
    mut n: *mut Agobj_t,
    mut sym: *mut Agsym_t,
    mut value: *const libc::c_char,
) {
    let mut buf: agxbuf = agxbuf {
        buf: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        eptr: 0 as *const libc::c_char as *mut libc::c_char,
        dyna: 0,
    };
    agxbinit(&mut buf, 0 as libc::c_int as libc::c_uint, 0 as *mut libc::c_char);
    while *value as libc::c_int != '\0' as i32 {
        if *value as libc::c_int == '\\' as i32 {
            agxbputc(&mut buf, '\\' as i32 as libc::c_char);
        }
        agxbputc(&mut buf, *value);
        value = value.offset(1);
    }
    agxset(n as *mut libc::c_void, sym, agxbuse(&mut buf));
    agxbfree(&mut buf);
}
unsafe extern "C" fn xdot_end_node(mut job: *mut GVJ_t) {
    let mut n: *mut Agnode_t = (*(*job).obj).u.n;
    if agxblen(xbufs[EMIT_NDRAW as libc::c_int as usize]) != 0 {
        agxset(
            n as *mut libc::c_void,
            (*xd).n_draw,
            agxbuse(xbufs[EMIT_NDRAW as libc::c_int as usize]),
        );
    }
    if agxblen(xbufs[EMIT_NLABEL as libc::c_int as usize]) != 0 {
        put_escaping_backslashes(
            &mut (*n).base,
            (*xd).n_l_draw,
            agxbuse(xbufs[EMIT_NLABEL as libc::c_int as usize]),
        );
    }
    penwidth[EMIT_NDRAW as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_NLABEL as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    textflags[EMIT_NDRAW as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_NLABEL as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn xdot_end_edge(mut job: *mut GVJ_t) {
    let mut e: *mut Agedge_t = (*(*job).obj).u.e;
    if agxblen(xbufs[EMIT_EDRAW as libc::c_int as usize]) != 0 {
        agxset(
            e as *mut libc::c_void,
            (*xd).e_draw,
            agxbuse(xbufs[EMIT_EDRAW as libc::c_int as usize]),
        );
    }
    if agxblen(xbufs[EMIT_TDRAW as libc::c_int as usize]) != 0 {
        agxset(
            e as *mut libc::c_void,
            (*xd).t_draw,
            agxbuse(xbufs[EMIT_TDRAW as libc::c_int as usize]),
        );
    }
    if agxblen(xbufs[EMIT_HDRAW as libc::c_int as usize]) != 0 {
        agxset(
            e as *mut libc::c_void,
            (*xd).h_draw,
            agxbuse(xbufs[EMIT_HDRAW as libc::c_int as usize]),
        );
    }
    if agxblen(xbufs[EMIT_ELABEL as libc::c_int as usize]) != 0 {
        put_escaping_backslashes(
            &mut (*e).base,
            (*xd).e_l_draw,
            agxbuse(xbufs[EMIT_ELABEL as libc::c_int as usize]),
        );
    }
    if agxblen(xbufs[EMIT_TLABEL as libc::c_int as usize]) != 0 {
        agxset(
            e as *mut libc::c_void,
            (*xd).tl_draw,
            agxbuse(xbufs[EMIT_TLABEL as libc::c_int as usize]),
        );
    }
    if agxblen(xbufs[EMIT_HLABEL as libc::c_int as usize]) != 0 {
        agxset(
            e as *mut libc::c_void,
            (*xd).hl_draw,
            agxbuse(xbufs[EMIT_HLABEL as libc::c_int as usize]),
        );
    }
    penwidth[EMIT_EDRAW as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_ELABEL as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_TDRAW as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_HDRAW as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_TLABEL as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_HLABEL as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    textflags[EMIT_EDRAW as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_ELABEL as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_TDRAW as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_HDRAW as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_TLABEL as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_HLABEL as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn xdot_end_cluster(mut job: *mut GVJ_t) {
    let mut cluster_g: *mut Agraph_t = (*(*job).obj).u.sg;
    agxset(
        cluster_g as *mut libc::c_void,
        (*xd).g_draw,
        agxbuse(xbufs[EMIT_CDRAW as libc::c_int as usize]),
    );
    if !((*((*(cluster_g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null() {
        agxset(
            cluster_g as *mut libc::c_void,
            (*xd).g_l_draw,
            agxbuse(xbufs[EMIT_CLABEL as libc::c_int as usize]),
        );
    }
    penwidth[EMIT_CDRAW as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_CLABEL as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    textflags[EMIT_CDRAW as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_CLABEL as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn versionStr2Version(mut str: *mut libc::c_char) -> libc::c_ushort {
    let mut c: libc::c_char = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = str;
    let mut us: libc::c_ushort = 0;
    loop {
        let fresh17 = s;
        s = s.offset(1);
        c = *fresh17;
        if !(c != 0) {
            break;
        }
        if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            continue;
        }
        if n < 8192 as libc::c_int - 1 as libc::c_int {
            let fresh18 = n;
            n = n + 1;
            buf[fresh18 as usize] = c;
        } else {
            agerr(
                AGWARN,
                b"xdot version \"%s\" too long\0" as *const u8 as *const libc::c_char,
                str,
            );
            break;
        }
    }
    buf[n as usize] = '\0' as i32 as libc::c_char;
    us = atoi(buf.as_mut_ptr()) as libc::c_ushort;
    return us;
}
unsafe extern "C" fn xdot_begin_graph(
    mut g: *mut graph_t,
    mut s_arrows: libc::c_int,
    mut e_arrows: libc::c_int,
    mut id: format_type,
) {
    let mut i: libc::c_int = 0;
    let mut us: libc::c_ushort = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    xd = gv_alloc(::std::mem::size_of::<xdot_state_t>() as libc::c_ulong)
        as *mut xdot_state_t;
    if id as libc::c_uint == FORMAT_XDOT14 as libc::c_int as libc::c_uint {
        (*xd).version = 14 as libc::c_int as libc::c_ushort;
        let ref mut fresh19 = (*xd).version_s;
        *fresh19 = b"1.4\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if id as libc::c_uint == FORMAT_XDOT12 as libc::c_int as libc::c_uint {
        (*xd).version = 12 as libc::c_int as libc::c_ushort;
        let ref mut fresh20 = (*xd).version_s;
        *fresh20 = b"1.2\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        s = agget(
            g as *mut libc::c_void,
            b"xdotversion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && {
                us = versionStr2Version(s);
                us as libc::c_int > 10 as libc::c_int
            }
        {
            (*xd).version = us;
            let ref mut fresh21 = (*xd).version_s;
            *fresh21 = s;
        } else {
            (*xd)
                .version = versionStr2Version(
                b"1.7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            let ref mut fresh22 = (*xd).version_s;
            *fresh22 = b"1.7\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster != 0 {
        let ref mut fresh23 = (*xd).g_draw;
        *fresh23 = safe_dcl(
            g,
            0 as libc::c_int,
            b"_draw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let ref mut fresh24 = (*xd).g_draw;
        *fresh24 = 0 as *mut attrsym_t;
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 3 as libc::c_int != 0
    {
        let ref mut fresh25 = (*xd).g_l_draw;
        *fresh25 = safe_dcl(
            g,
            0 as libc::c_int,
            b"_ldraw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let ref mut fresh26 = (*xd).g_l_draw;
        *fresh26 = 0 as *mut attrsym_t;
    }
    let ref mut fresh27 = (*xd).n_draw;
    *fresh27 = safe_dcl(
        g,
        1 as libc::c_int,
        b"_draw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh28 = (*xd).n_l_draw;
    *fresh28 = safe_dcl(
        g,
        1 as libc::c_int,
        b"_ldraw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh29 = (*xd).e_draw;
    *fresh29 = safe_dcl(
        g,
        2 as libc::c_int,
        b"_draw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if e_arrows != 0 {
        let ref mut fresh30 = (*xd).h_draw;
        *fresh30 = safe_dcl(
            g,
            2 as libc::c_int,
            b"_hdraw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let ref mut fresh31 = (*xd).h_draw;
        *fresh31 = 0 as *mut attrsym_t;
    }
    if s_arrows != 0 {
        let ref mut fresh32 = (*xd).t_draw;
        *fresh32 = safe_dcl(
            g,
            2 as libc::c_int,
            b"_tdraw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let ref mut fresh33 = (*xd).t_draw;
        *fresh33 = 0 as *mut attrsym_t;
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 5 as libc::c_int) != 0
    {
        let ref mut fresh34 = (*xd).e_l_draw;
        *fresh34 = safe_dcl(
            g,
            2 as libc::c_int,
            b"_ldraw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let ref mut fresh35 = (*xd).e_l_draw;
        *fresh35 = 0 as *mut attrsym_t;
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 1 as libc::c_int != 0
    {
        let ref mut fresh36 = (*xd).hl_draw;
        *fresh36 = safe_dcl(
            g,
            2 as libc::c_int,
            b"_hldraw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let ref mut fresh37 = (*xd).hl_draw;
        *fresh37 = 0 as *mut attrsym_t;
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 2 as libc::c_int != 0
    {
        let ref mut fresh38 = (*xd).tl_draw;
        *fresh38 = safe_dcl(
            g,
            2 as libc::c_int,
            b"_tldraw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let ref mut fresh39 = (*xd).tl_draw;
        *fresh39 = 0 as *mut attrsym_t;
    }
    i = 0 as libc::c_int;
    while i < EMIT_HLABEL as libc::c_int + 1 as libc::c_int {
        agxbinit(
            xbuf.as_mut_ptr().offset(i as isize),
            8192 as libc::c_int as libc::c_uint,
            ((*xd).buf[i as usize]).as_mut_ptr(),
        );
        i += 1;
    }
}
unsafe extern "C" fn dot_begin_graph(mut job: *mut GVJ_t) {
    let mut e_arrows: libc::c_int = 0;
    let mut s_arrows: libc::c_int = 0;
    let mut g: *mut graph_t = (*(*job).obj).u.g;
    match (*job).render.id {
        0 => {
            attach_attrs(g);
        }
        1 => {
            if !(aggetrec(
                g as *mut libc::c_void,
                b"cl_edge_info\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ))
                .is_null()
            {
                undoClusterEdges(g);
            }
        }
        2 | 3 => {}
        4 | 5 | 6 => {
            attach_attrs_and_arrows(g, &mut s_arrows, &mut e_arrows);
            xdot_begin_graph(g, s_arrows, e_arrows, (*job).render.id as format_type);
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"gvrender_core_dot.c\0" as *const u8 as *const libc::c_char,
                498 as libc::c_int,
            );
            abort();
        }
    };
}
unsafe extern "C" fn xdot_end_graph(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    if agxblen(xbufs[EMIT_GDRAW as libc::c_int as usize]) != 0 {
        if ((*xd).g_draw).is_null() {
            let ref mut fresh40 = (*xd).g_draw;
            *fresh40 = safe_dcl(
                g,
                0 as libc::c_int,
                b"_draw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        agxset(
            g as *mut libc::c_void,
            (*xd).g_draw,
            agxbuse(xbufs[EMIT_GDRAW as libc::c_int as usize]),
        );
    }
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null() {
        put_escaping_backslashes(
            &mut (*g).base,
            (*xd).g_l_draw,
            agxbuse(xbufs[EMIT_GLABEL as libc::c_int as usize]),
        );
    }
    agsafeset(
        g as *mut libc::c_void,
        b"xdotversion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*xd).version_s,
        b"\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < EMIT_HLABEL as libc::c_int + 1 as libc::c_int {
        agxbfree(xbuf.as_mut_ptr().offset(i as isize));
        i += 1;
    }
    free(xd as *mut libc::c_void);
    penwidth[EMIT_GDRAW as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    penwidth[EMIT_GLABEL as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    textflags[EMIT_GDRAW as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    textflags[EMIT_GLABEL as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn dot_end_graph(mut job: *mut GVJ_t) {
    let mut g: *mut graph_t = (*(*job).obj).u.g;
    let mut io_save: *mut Agiodisc_t = 0 as *mut Agiodisc_t;
    static mut io: Agiodisc_t = Agiodisc_t {
        afread: None,
        putstr: None,
        flush: None,
    };
    if (io.afread).is_none() {
        io.afread = AgIoDisc.afread;
        io
            .putstr = ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> libc::c_int,
            >,
            putstrfn,
        >(
            Some(
                gvputs
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        );
        io
            .flush = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int>,
            flushfn,
        >(Some(gvflush as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int));
    }
    io_save = (*(*g).clos).disc.io;
    let ref mut fresh41 = (*(*g).clos).disc.io;
    *fresh41 = &mut io;
    match (*job).render.id {
        2 => {
            write_plain(job, g, job as *mut FILE, 0 as libc::c_int != 0);
        }
        3 => {
            write_plain(job, g, job as *mut FILE, 1 as libc::c_int != 0);
        }
        0 | 1 => {
            if (*job).flags & (1 as libc::c_int) << 27 as libc::c_int == 0 {
                agwrite(g, job as *mut libc::c_void);
            }
        }
        4 | 5 | 6 => {
            xdot_end_graph(g);
            if (*job).flags & (1 as libc::c_int) << 27 as libc::c_int == 0 {
                agwrite(g, job as *mut libc::c_void);
            }
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"gvrender_core_dot.c\0" as *const u8 as *const libc::c_char,
                560 as libc::c_int,
            );
            abort();
        }
    }
    let ref mut fresh42 = (*(*g).clos).disc.io;
    *fresh42 = io_save;
}
static mut flag_masks: [libc::c_uint; 3] = [
    0x1f as libc::c_int as libc::c_uint,
    0x3f as libc::c_int as libc::c_uint,
    0x7f as libc::c_int as libc::c_uint,
];
unsafe extern "C" fn xdot_textspan(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut span: *mut textspan_t,
) {
    let mut emit_state: emit_state_t = (*(*job).obj).emit_state;
    let mut flags: libc::c_uint = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut j: libc::c_int = 0;
    agxbput(xbufs[emit_state as usize], b"F \0" as *const u8 as *const libc::c_char);
    xdot_fmt_num(buf.as_mut_ptr(), (*(*span).font).size);
    agxbput(xbufs[emit_state as usize], buf.as_mut_ptr());
    xdot_str(
        job,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*(*span).font).name,
    );
    xdot_pencolor(job);
    match (*span).just as libc::c_int {
        108 => {
            j = -(1 as libc::c_int);
        }
        114 => {
            j = 1 as libc::c_int;
        }
        110 | _ => {
            j = 0 as libc::c_int;
        }
    }
    if !((*span).font).is_null() {
        flags = (*(*span).font).flags();
    } else {
        flags = 0 as libc::c_int as libc::c_uint;
    }
    if (*xd).version as libc::c_int >= 15 as libc::c_int {
        let mut mask: libc::c_uint = flag_masks[((*xd).version as libc::c_int
            - 15 as libc::c_int) as usize];
        let mut bits: libc::c_uint = flags & mask;
        if textflags[emit_state as usize] != bits {
            agxbprint(
                xbufs[emit_state as usize],
                b"t %u \0" as *const u8 as *const libc::c_char,
                bits,
            );
            textflags[emit_state as usize] = bits;
        }
    }
    p.y += (*span).yoffset_centerline;
    agxbput(xbufs[emit_state as usize], b"T \0" as *const u8 as *const libc::c_char);
    xdot_point(xbufs[emit_state as usize], p);
    agxbprint(
        xbufs[emit_state as usize],
        b"%d \0" as *const u8 as *const libc::c_char,
        j,
    );
    xdot_fmt_num(buf.as_mut_ptr(), (*span).size.x);
    agxbput(xbufs[emit_state as usize], buf.as_mut_ptr());
    xdot_str(
        job,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*span).str_0,
    );
}
unsafe extern "C" fn xdot_color_stop(
    mut xb: *mut agxbuf,
    mut v: libc::c_float,
    mut clr: *mut gvcolor_t,
) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"%.03f\0" as *const u8 as *const libc::c_char,
        v as libc::c_double,
    );
    xdot_trim_zeros(buf.as_mut_ptr(), 1 as libc::c_int);
    xdot_str_xbuf(xb, buf.as_mut_ptr(), color2str(((*clr).u.rgba).as_mut_ptr()));
}
unsafe extern "C" fn xdot_gradient_fillcolor(
    mut job: *mut GVJ_t,
    mut filled: libc::c_int,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    let mut buf0: [libc::c_char; 8192] = [0; 8192];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        eptr: 0 as *const libc::c_char as *mut libc::c_char,
        dyna: 0,
    };
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut angle: libc::c_double = (*obj).gradient_angle as libc::c_double
        * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double;
    let mut G: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut c1: pointf = pointf { x: 0., y: 0. };
    let mut c2: pointf = pointf { x: 0., y: 0. };
    if ((*xd).version as libc::c_int) < 14 as libc::c_int {
        xdot_fillcolor(job);
        return;
    }
    agxbinit(&mut xb, 8192 as libc::c_int as libc::c_uint, buf0.as_mut_ptr());
    if filled == 2 as libc::c_int {
        get_gradient_points(A, G.as_mut_ptr(), n, angle, 2 as libc::c_int);
        agxbputc(&mut xb, '[' as i32 as libc::c_char);
        xdot_point(&mut xb, G[0 as libc::c_int as usize]);
        xdot_point(&mut xb, G[1 as libc::c_int as usize]);
    } else {
        get_gradient_points(
            A,
            G.as_mut_ptr(),
            n,
            0 as libc::c_int as libc::c_double,
            3 as libc::c_int,
        );
        let mut r2: libc::c_double = G[1 as libc::c_int as usize].y;
        if angle == 0 as libc::c_int as libc::c_double {
            c1.x = G[0 as libc::c_int as usize].x;
            c1.y = G[0 as libc::c_int as usize].y;
        } else {
            c1
                .x = G[0 as libc::c_int as usize].x
                + r2 / 4 as libc::c_int as libc::c_double * cos(angle);
            c1
                .y = G[0 as libc::c_int as usize].y
                + r2 / 4 as libc::c_int as libc::c_double * sin(angle);
        }
        c2.x = G[0 as libc::c_int as usize].x;
        c2.y = G[0 as libc::c_int as usize].y;
        let mut r1: libc::c_double = r2 / 4 as libc::c_int as libc::c_double;
        agxbputc(&mut xb, '(' as i32 as libc::c_char);
        xdot_point(&mut xb, c1);
        xdot_num(&mut xb, r1);
        xdot_point(&mut xb, c2);
        xdot_num(&mut xb, r2);
    }
    agxbput(&mut xb, b"2 \0" as *const u8 as *const libc::c_char);
    if (*obj).gradient_frac > 0 as libc::c_int as libc::c_float {
        xdot_color_stop(&mut xb, (*obj).gradient_frac, &mut (*obj).fillcolor);
        xdot_color_stop(&mut xb, (*obj).gradient_frac, &mut (*obj).stopcolor);
    } else {
        xdot_color_stop(
            &mut xb,
            0 as libc::c_int as libc::c_float,
            &mut (*obj).fillcolor,
        );
        xdot_color_stop(
            &mut xb,
            1 as libc::c_int as libc::c_float,
            &mut (*obj).stopcolor,
        );
    }
    agxbpop(&mut xb);
    if filled == 2 as libc::c_int {
        agxbputc(&mut xb, ']' as i32 as libc::c_char);
    } else {
        agxbputc(&mut xb, ')' as i32 as libc::c_char);
    }
    xdot_str(
        job,
        b"C \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        agxbuse(&mut xb),
    );
    agxbfree(&mut xb);
}
unsafe extern "C" fn xdot_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut emit_state: emit_state_t = (*(*job).obj).emit_state;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    xdot_style(job);
    xdot_pencolor(job);
    if filled != 0 {
        if filled == 2 as libc::c_int || filled == 3 as libc::c_int {
            xdot_gradient_fillcolor(job, filled, A, 2 as libc::c_int);
        } else {
            xdot_fillcolor(job);
        }
        agxbput(xbufs[emit_state as usize], b"E \0" as *const u8 as *const libc::c_char);
    } else {
        agxbput(xbufs[emit_state as usize], b"e \0" as *const u8 as *const libc::c_char);
    }
    xdot_point(xbufs[emit_state as usize], *A.offset(0 as libc::c_int as isize));
    xdot_fmt_num(
        buf.as_mut_ptr(),
        (*A.offset(1 as libc::c_int as isize)).x
            - (*A.offset(0 as libc::c_int as isize)).x,
    );
    agxbput(xbufs[emit_state as usize], buf.as_mut_ptr());
    xdot_fmt_num(
        buf.as_mut_ptr(),
        (*A.offset(1 as libc::c_int as isize)).y
            - (*A.offset(0 as libc::c_int as isize)).y,
    );
    agxbput(xbufs[emit_state as usize], buf.as_mut_ptr());
}
unsafe extern "C" fn xdot_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    xdot_style(job);
    xdot_pencolor(job);
    if filled != 0 {
        if filled == 2 as libc::c_int || filled == 3 as libc::c_int {
            xdot_gradient_fillcolor(job, filled, A, n);
        } else {
            xdot_fillcolor(job);
        }
        xdot_points(job, 'b' as i32 as libc::c_char, A, n);
    } else {
        xdot_points(job, 'B' as i32 as libc::c_char, A, n);
    };
}
unsafe extern "C" fn xdot_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    xdot_style(job);
    xdot_pencolor(job);
    if filled != 0 {
        if filled == 2 as libc::c_int || filled == 3 as libc::c_int {
            xdot_gradient_fillcolor(job, filled, A, n);
        } else {
            xdot_fillcolor(job);
        }
        xdot_points(job, 'P' as i32 as libc::c_char, A, n);
    } else {
        xdot_points(job, 'p' as i32 as libc::c_char, A, n);
    };
}
unsafe extern "C" fn xdot_polyline(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    xdot_style(job);
    xdot_pencolor(job);
    xdot_points(job, 'L' as i32 as libc::c_char, A, n);
}
#[no_mangle]
pub unsafe extern "C" fn core_loadimage_xdot(
    mut job: *mut GVJ_t,
    mut us: *mut usershape_t,
    mut b: boxf,
    mut filled: bool,
) {
    let mut emit_state: emit_state_t = (*(*job).obj).emit_state;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    agxbput(xbufs[emit_state as usize], b"I \0" as *const u8 as *const libc::c_char);
    xdot_point(xbufs[emit_state as usize], b.LL);
    xdot_fmt_num(buf.as_mut_ptr(), b.UR.x - b.LL.x);
    agxbput(xbufs[emit_state as usize], buf.as_mut_ptr());
    xdot_fmt_num(buf.as_mut_ptr(), b.UR.y - b.LL.y);
    agxbput(xbufs[emit_state as usize], buf.as_mut_ptr());
    xdot_str(
        job,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*us).name,
    );
}
#[no_mangle]
pub static mut dot_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: None,
            end_job: None,
            begin_graph: Some(dot_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_graph: Some(dot_end_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_layer: None,
            end_layer: None,
            begin_page: None,
            end_page: None,
            begin_cluster: None,
            end_cluster: None,
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: None,
            end_node: None,
            begin_edge: None,
            end_edge: None,
            begin_anchor: None,
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: None,
            resolve_color: None,
            ellipse: None,
            polygon: None,
            beziercurve: None,
            polyline: None,
            comment: None,
            library_shape: None,
        };
        init
    }
};
#[no_mangle]
pub static mut xdot_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: None,
            end_job: None,
            begin_graph: Some(dot_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_graph: Some(dot_end_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_layer: None,
            end_layer: None,
            begin_page: None,
            end_page: None,
            begin_cluster: None,
            end_cluster: Some(
                xdot_end_cluster as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: None,
            end_node: Some(xdot_end_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_edge: None,
            end_edge: Some(xdot_end_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_anchor: None,
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                xdot_textspan
                    as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: None,
            ellipse: Some(
                xdot_ellipse
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                xdot_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                xdot_bezier
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            polyline: Some(
                xdot_polyline
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: None,
            library_shape: None,
        };
        init
    }
};
#[no_mangle]
pub static mut render_features_dot: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 13 as libc::c_int,
        default_pad: 0.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: COLOR_STRING,
    };
    init
};
#[no_mangle]
pub static mut render_features_xdot: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 13 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 23 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int,
        default_pad: 0.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: RGBA_BYTE,
    };
    init
};
#[no_mangle]
pub static mut device_features_canon: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 26 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut device_features_dot: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: 0 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_dot_types: [gvplugin_installed_t; 3] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_DOT as libc::c_int,
                type_0: b"dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &dot_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_dot as *const gvrender_features_t
                    as *mut gvrender_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_XDOT as libc::c_int,
                type_0: b"xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &xdot_engine as *const gvrender_engine_t
                    as *mut gvrender_engine_t as *mut libc::c_void,
                features: &render_features_xdot as *const gvrender_features_t
                    as *mut gvrender_features_t as *mut libc::c_void,
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
#[no_mangle]
pub static mut gvdevice_dot_types: [gvplugin_installed_t; 9] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_DOT as libc::c_int,
                type_0: b"dot:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_dot as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_DOT as libc::c_int,
                type_0: b"gv:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_dot as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_CANON as libc::c_int,
                type_0: b"canon:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_canon as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PLAIN as libc::c_int,
                type_0: b"plain:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_dot as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PLAIN_EXT as libc::c_int,
                type_0: b"plain-ext:dot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_dot as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_XDOT as libc::c_int,
                type_0: b"xdot:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_dot as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_XDOT12 as libc::c_int,
                type_0: b"xdot1.2:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_dot as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_XDOT14 as libc::c_int,
                type_0: b"xdot1.4:xdot\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_dot as *const gvdevice_features_t
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
unsafe extern "C" fn run_static_initializers() {
    xbufs = [
        xbuf.as_mut_ptr().offset(EMIT_GDRAW as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_CDRAW as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_TDRAW as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_HDRAW as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_GLABEL as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_CLABEL as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_TLABEL as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_HLABEL as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_CDRAW as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_CDRAW as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_CLABEL as libc::c_int as isize),
        xbuf.as_mut_ptr().offset(EMIT_CLABEL as libc::c_int as isize),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
