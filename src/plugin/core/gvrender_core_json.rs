#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    pub type htmllabel_t;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn freeXDot(_: *mut xdot);
    fn parseXDot(_: *mut libc::c_char) -> *mut xdot;
    static mut stderr: *mut FILE;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut AgIoDisc: Agiodisc_t;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agisstrict(g: *mut Agraph_t) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agnxtattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        attr: *mut Agsym_t,
    ) -> *mut Agsym_t;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn latin1ToUTF8(_: *mut libc::c_char) -> *mut libc::c_char;
    fn attach_attrs(g: *mut graph_t);
    fn gvRender(
        gvc: *mut GVC_t,
        g: *mut graph_t,
        format: *const libc::c_char,
        out: *mut FILE,
    ) -> libc::c_int;
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvflush(job: *mut GVJ_t) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn gvCloneGVC(_: *mut GVC_t) -> *mut GVC_t;
    fn gvFreeCloneGVC(_: *mut GVC_t);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type xdot_grad_type = libc::c_uint;
pub const xd_radial: xdot_grad_type = 2;
pub const xd_linear: xdot_grad_type = 1;
pub const xd_none: xdot_grad_type = 0;
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
pub struct xdot_color {
    pub type_0: xdot_grad_type,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub clr: *mut libc::c_char,
    pub ling: xdot_linear_grad,
    pub ring: xdot_radial_grad,
}
pub type xdot_align = libc::c_uint;
pub const xd_right: xdot_align = 2;
pub const xd_center: xdot_align = 1;
pub const xd_left: xdot_align = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
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
pub struct xdot_polyline {
    pub cnt: libc::c_int,
    pub pts: *mut xdot_point,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_image {
    pub pos: xdot_rect,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_font {
    pub size: libc::c_double,
    pub name: *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xdot_op {
    pub kind: xdot_kind,
    pub u: C2RustUnnamed_0,
    pub drawfunc: drawfunc_t,
}
pub type drawfunc_t = Option::<unsafe extern "C" fn(*mut xdot_op, libc::c_int) -> ()>;
pub type xdot_op = _xdot_op;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type freefunc_t = Option::<unsafe extern "C" fn(*mut xdot_op) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot {
    pub cnt: libc::c_int,
    pub sz: libc::c_int,
    pub ops: *mut xdot_op,
    pub freefunc: freefunc_t,
    pub flags: libc::c_int,
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
#[derive(Copy, Clone, BitfieldStruct)]
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
#[derive(Copy, Clone, BitfieldStruct)]
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
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub hh: C2RustUnnamed_4,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub graph: C2RustUnnamed_5,
    pub node: C2RustUnnamed_5,
    pub edge: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub type qsort_cmpf = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_7,
    pub s: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
pub type C2RustUnnamed_10 = libc::c_uint;
pub const FORMAT_XDOT_JSON: C2RustUnnamed_10 = 3;
pub const FORMAT_DOT_JSON: C2RustUnnamed_10 = 2;
pub const FORMAT_JSON0: C2RustUnnamed_10 = 1;
pub const FORMAT_JSON: C2RustUnnamed_10 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t {
    pub Level: libc::c_int,
    pub isLatin: bool,
    pub doXDot: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvid_t {
    pub h: Agrec_t,
    pub id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intm {
    pub link: Dtlink_t,
    pub id: *mut libc::c_char,
    pub v: libc::c_int,
}
pub type putstrfn = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
>;
pub type flushfn = Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;
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
unsafe extern "C" fn json_begin_graph(mut job: *mut GVJ_t) {
    if (*job).render.id == FORMAT_JSON as libc::c_int {
        let mut gvc: *mut GVC_t = gvCloneGVC((*job).gvc);
        let mut g: *mut graph_t = (*(*job).obj).u.g;
        gvRender(gvc, g, b"xdot\0" as *const u8 as *const libc::c_char, 0 as *mut FILE);
        gvFreeCloneGVC(gvc);
    } else if (*job).render.id == FORMAT_JSON0 as libc::c_int {
        attach_attrs((*(*job).gvc).g);
    }
}
unsafe extern "C" fn stoj(
    mut ins: *mut libc::c_char,
    mut sp: *mut state_t,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut input: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut xb: agxbuf = agxbuf {
        buf: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        eptr: 0 as *const libc::c_char as *mut libc::c_char,
        dyna: 0,
    };
    let mut c: libc::c_char = 0;
    if (*sp).isLatin {
        input = latin1ToUTF8(ins);
    } else {
        input = ins;
    }
    if (xb.buf).is_null() {
        agxbinit(&mut xb, 8192 as libc::c_int as libc::c_uint, 0 as *mut libc::c_char);
    }
    s = input;
    loop {
        c = *s;
        if !(c != 0) {
            break;
        }
        match c as libc::c_int {
            34 => {
                agxbput(&mut xb, b"\\\"\0" as *const u8 as *const libc::c_char);
            }
            92 => {
                agxbput(&mut xb, b"\\\\\0" as *const u8 as *const libc::c_char);
            }
            47 => {
                agxbput(&mut xb, b"\\/\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                agxbput(&mut xb, b"\\b\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                agxbput(&mut xb, b"\\f\0" as *const u8 as *const libc::c_char);
            }
            10 => {
                agxbput(&mut xb, b"\\n\0" as *const u8 as *const libc::c_char);
            }
            13 => {
                agxbput(&mut xb, b"\\r\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                agxbput(&mut xb, b"\\t\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                agxbputc(&mut xb, c);
            }
        }
        s = s.offset(1);
    }
    s = agxbuse(&mut xb);
    if (*sp).isLatin {
        free(input as *mut libc::c_void);
    }
    return s;
}
unsafe extern "C" fn indent(mut job: *mut GVJ_t, mut level: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = level;
    while i > 0 as libc::c_int {
        gvputs(job, b"  \0" as *const u8 as *const libc::c_char);
        i -= 1;
    }
}
unsafe extern "C" fn set_attrwf(
    mut g: *mut Agraph_t,
    mut toplevel: bool,
    mut value: bool,
) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let ref mut fresh11 = (*(g as *mut Agobj_t)).tag;
    (*fresh11).set_attrwf(value as libc::c_uint);
    subg = agfstsubg(g);
    while !subg.is_null() {
        set_attrwf(subg, 0 as libc::c_int != 0, value);
        subg = agnxtsubg(subg);
    }
    if toplevel {
        n = agfstnode(g);
        while !n.is_null() {
            let ref mut fresh12 = (*(n as *mut Agobj_t)).tag;
            (*fresh12).set_attrwf(value as libc::c_uint);
            e = agfstout(g, n);
            while !e.is_null() {
                let ref mut fresh13 = (*(e as *mut Agobj_t)).tag;
                (*fresh13).set_attrwf(value as libc::c_uint);
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
    }
}
unsafe extern "C" fn write_polyline(
    mut job: *mut GVJ_t,
    mut polyline: *mut xdot_polyline,
) {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = (*polyline).cnt;
    let mut pts: *mut xdot_point = (*polyline).pts;
    gvprintf(job, b"\"points\": [\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < cnt {
        if i > 0 as libc::c_int {
            gvprintf(job, b",\0" as *const u8 as *const libc::c_char);
        }
        gvprintf(
            job,
            b"[%.03f,%.03f]\0" as *const u8 as *const libc::c_char,
            (*pts.offset(i as isize)).x,
            (*pts.offset(i as isize)).y,
        );
        i += 1;
    }
    gvprintf(job, b"]\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn write_stops(
    mut job: *mut GVJ_t,
    mut n_stops: libc::c_int,
    mut stp: *mut xdot_color_stop,
    mut sp: *mut state_t,
) {
    let mut i: libc::c_int = 0;
    gvprintf(job, b"\"stops\": [\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n_stops {
        if i > 0 as libc::c_int {
            gvprintf(job, b",\0" as *const u8 as *const libc::c_char);
        }
        gvprintf(
            job,
            b"{\"frac\": %.03f, \"color\": \"%s\"}\0" as *const u8
                as *const libc::c_char,
            (*stp.offset(i as isize)).frac as libc::c_double,
            stoj((*stp.offset(i as isize)).color, sp),
        );
        i += 1;
    }
    gvprintf(job, b"]\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn write_radial_grad(
    mut job: *mut GVJ_t,
    mut rg: *mut xdot_radial_grad,
    mut sp: *mut state_t,
) {
    indent(job, (*sp).Level);
    gvprintf(
        job,
        b"\"p0\": [%.03f,%.03f,%.03f],\n\0" as *const u8 as *const libc::c_char,
        (*rg).x0,
        (*rg).y0,
        (*rg).r0,
    );
    indent(job, (*sp).Level);
    gvprintf(
        job,
        b"\"p1\": [%.03f,%.03f,%.03f],\n\0" as *const u8 as *const libc::c_char,
        (*rg).x1,
        (*rg).y1,
        (*rg).r1,
    );
    indent(job, (*sp).Level);
    write_stops(job, (*rg).n_stops, (*rg).stops, sp);
}
unsafe extern "C" fn write_linear_grad(
    mut job: *mut GVJ_t,
    mut lg: *mut xdot_linear_grad,
    mut sp: *mut state_t,
) {
    indent(job, (*sp).Level);
    gvprintf(
        job,
        b"\"p0\": [%.03f,%.03f],\n\0" as *const u8 as *const libc::c_char,
        (*lg).x0,
        (*lg).y0,
    );
    indent(job, (*sp).Level);
    gvprintf(
        job,
        b"\"p1\": [%.03f,%.03f],\n\0" as *const u8 as *const libc::c_char,
        (*lg).x1,
        (*lg).y1,
    );
    indent(job, (*sp).Level);
    write_stops(job, (*lg).n_stops, (*lg).stops, sp);
}
unsafe extern "C" fn write_xdot(
    mut op: *mut xdot_op,
    mut job: *mut GVJ_t,
    mut sp: *mut state_t,
) {
    let ref mut fresh14 = (*sp).Level;
    let fresh15 = *fresh14;
    *fresh14 = *fresh14 + 1;
    indent(job, fresh15);
    gvputs(job, b"{\n\0" as *const u8 as *const libc::c_char);
    indent(job, (*sp).Level);
    match (*op).kind as libc::c_uint {
        0 | 1 => {
            gvprintf(
                job,
                b"\"op\": \"%c\",\n\0" as *const u8 as *const libc::c_char,
                if (*op).kind as libc::c_uint
                    == xd_filled_ellipse as libc::c_int as libc::c_uint
                {
                    'E' as i32
                } else {
                    'e' as i32
                },
            );
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"rect\": [%.03f,%.03f,%.03f,%.03f]\n\0" as *const u8
                    as *const libc::c_char,
                (*op).u.ellipse.x,
                (*op).u.ellipse.y,
                (*op).u.ellipse.w,
                (*op).u.ellipse.h,
            );
        }
        2 | 3 => {
            gvprintf(
                job,
                b"\"op\": \"%c\",\n\0" as *const u8 as *const libc::c_char,
                if (*op).kind as libc::c_uint
                    == xd_filled_polygon as libc::c_int as libc::c_uint
                {
                    'P' as i32
                } else {
                    'p' as i32
                },
            );
            indent(job, (*sp).Level);
            write_polyline(job, &mut (*op).u.polygon);
        }
        4 | 5 => {
            gvprintf(
                job,
                b"\"op\": \"%c\",\n\0" as *const u8 as *const libc::c_char,
                if (*op).kind as libc::c_uint
                    == xd_filled_bezier as libc::c_int as libc::c_uint
                {
                    'B' as i32
                } else {
                    'b' as i32
                },
            );
            indent(job, (*sp).Level);
            write_polyline(job, &mut (*op).u.bezier);
        }
        6 => {
            gvprintf(job, b"\"op\": \"L\",\n\0" as *const u8 as *const libc::c_char);
            indent(job, (*sp).Level);
            write_polyline(job, &mut (*op).u.polyline);
        }
        7 => {
            gvprintf(job, b"\"op\": \"T\",\n\0" as *const u8 as *const libc::c_char);
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"pt\": [%.03f,%.03f],\n\0" as *const u8 as *const libc::c_char,
                (*op).u.text.x,
                (*op).u.text.y,
            );
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"align\": \"%c\",\n\0" as *const u8 as *const libc::c_char,
                if (*op).u.text.align as libc::c_uint
                    == xd_left as libc::c_int as libc::c_uint
                {
                    'l' as i32
                } else if (*op).u.text.align as libc::c_uint
                        == xd_center as libc::c_int as libc::c_uint
                    {
                    'c' as i32
                } else {
                    'r' as i32
                },
            );
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"width\": %.03f,\n\0" as *const u8 as *const libc::c_char,
                (*op).u.text.width,
            );
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"text\": \"%s\"\n\0" as *const u8 as *const libc::c_char,
                stoj((*op).u.text.text, sp),
            );
        }
        8 | 9 => {
            gvprintf(
                job,
                b"\"op\": \"%c\",\n\0" as *const u8 as *const libc::c_char,
                if (*op).kind as libc::c_uint
                    == xd_fill_color as libc::c_int as libc::c_uint
                {
                    'C' as i32
                } else {
                    'c' as i32
                },
            );
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"grad\": \"none\",\n\0" as *const u8 as *const libc::c_char,
            );
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"color\": \"%s\"\n\0" as *const u8 as *const libc::c_char,
                stoj((*op).u.color, sp),
            );
        }
        14 | 13 => {
            gvprintf(
                job,
                b"\"op\": \"%c\",\n\0" as *const u8 as *const libc::c_char,
                if (*op).kind as libc::c_uint
                    == xd_grad_fill_color as libc::c_int as libc::c_uint
                {
                    'C' as i32
                } else {
                    'c' as i32
                },
            );
            indent(job, (*sp).Level);
            if (*op).u.grad_color.type_0 as libc::c_uint
                == xd_none as libc::c_int as libc::c_uint
            {
                gvprintf(
                    job,
                    b"\"grad\": \"none\",\n\0" as *const u8 as *const libc::c_char,
                );
                indent(job, (*sp).Level);
                gvprintf(
                    job,
                    b"\"color\": \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    stoj((*op).u.grad_color.u.clr, sp),
                );
            } else if (*op).u.grad_color.type_0 as libc::c_uint
                    == xd_linear as libc::c_int as libc::c_uint
                {
                gvprintf(
                    job,
                    b"\"grad\": \"linear\",\n\0" as *const u8 as *const libc::c_char,
                );
                indent(job, (*sp).Level);
                write_linear_grad(job, &mut (*op).u.grad_color.u.ling, sp);
            } else {
                gvprintf(
                    job,
                    b"\"grad\": \"radial\",\n\0" as *const u8 as *const libc::c_char,
                );
                indent(job, (*sp).Level);
                write_radial_grad(job, &mut (*op).u.grad_color.u.ring, sp);
            }
        }
        10 => {
            gvprintf(job, b"\"op\": \"F\",\n\0" as *const u8 as *const libc::c_char);
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"size\": %.03f,\n\0" as *const u8 as *const libc::c_char,
                (*op).u.font.size,
            );
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"face\": \"%s\"\n\0" as *const u8 as *const libc::c_char,
                stoj((*op).u.font.name, sp),
            );
        }
        11 => {
            gvprintf(job, b"\"op\": \"S\",\n\0" as *const u8 as *const libc::c_char);
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"style\": \"%s\"\n\0" as *const u8 as *const libc::c_char,
                stoj((*op).u.style, sp),
            );
        }
        12 => {}
        15 => {
            gvprintf(job, b"\"op\": \"t\",\n\0" as *const u8 as *const libc::c_char);
            indent(job, (*sp).Level);
            gvprintf(
                job,
                b"\"fontchar\": %d\n\0" as *const u8 as *const libc::c_char,
                (*op).u.fontchar,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"gvrender_core_json.c\0" as *const u8 as *const libc::c_char,
                301 as libc::c_int,
            );
            abort();
        }
    }
    let ref mut fresh16 = (*sp).Level;
    *fresh16 -= 1;
    indent(job, (*sp).Level);
    gvputs(job, b"}\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn write_xdots(
    mut val: *mut libc::c_char,
    mut job: *mut GVJ_t,
    mut sp: *mut state_t,
) {
    let mut cmds: *mut xdot = 0 as *mut xdot;
    let mut i: libc::c_int = 0;
    if val.is_null() || *val as libc::c_int == '\0' as i32 {
        return;
    }
    cmds = parseXDot(val);
    if cmds.is_null() {
        agerr(
            AGWARN,
            b"Could not parse xdot \"%s\"\n\0" as *const u8 as *const libc::c_char,
            val,
        );
        return;
    }
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    let ref mut fresh17 = (*sp).Level;
    let fresh18 = *fresh17;
    *fresh17 = *fresh17 + 1;
    indent(job, fresh18);
    gvputs(job, b"[\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*cmds).cnt {
        if i > 0 as libc::c_int {
            gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
        }
        write_xdot(((*cmds).ops).offset(i as isize), job, sp);
        i += 1;
    }
    let ref mut fresh19 = (*sp).Level;
    *fresh19 -= 1;
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    indent(job, (*sp).Level);
    gvputs(job, b"]\0" as *const u8 as *const libc::c_char);
    freeXDot(cmds);
}
unsafe extern "C" fn isXDot(mut name: *mut libc::c_char) -> libc::c_int {
    let fresh20 = name;
    name = name.offset(1);
    return (*fresh20 as libc::c_int == '_' as i32
        && (strcmp(name, b"draw_\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(name, b"ldraw_\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(name, b"hdraw_\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(name, b"tdraw_\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(name, b"hldraw_\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(name, b"tldraw_\0" as *const u8 as *const libc::c_char) == 0))
        as libc::c_int;
}
unsafe extern "C" fn write_attrs(
    mut obj: *mut Agobj_t,
    mut job: *mut GVJ_t,
    mut sp: *mut state_t,
) {
    let mut g: *mut Agraph_t = agroot(obj as *mut libc::c_void);
    let mut type_0: libc::c_int = ((*obj).tag).objtype() as libc::c_int;
    let mut attrval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sym: *mut Agsym_t = agnxtattr(g, type_0, 0 as *mut Agsym_t);
    if sym.is_null() {
        return;
    }
    while !sym.is_null() {
        attrval = agxget(obj as *mut libc::c_void, sym);
        if !attrval.is_null() {
            if !(*attrval as libc::c_int == '\0' as i32
                && strcmp((*sym).name, b"label\0" as *const u8 as *const libc::c_char)
                    != 0)
            {
                gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
                indent(job, (*sp).Level);
                gvprintf(
                    job,
                    b"\"%s\": \0" as *const u8 as *const libc::c_char,
                    stoj((*sym).name, sp),
                );
                if (*sp).doXDot as libc::c_int != 0 && isXDot((*sym).name) != 0 {
                    write_xdots(agxget(obj as *mut libc::c_void, sym), job, sp);
                } else {
                    gvprintf(
                        job,
                        b"\"%s\"\0" as *const u8 as *const libc::c_char,
                        stoj(agxget(obj as *mut libc::c_void, sym), sp),
                    );
                }
            }
        }
        sym = agnxtattr(g, type_0, sym);
    }
}
unsafe extern "C" fn write_hdr(
    mut g: *mut Agraph_t,
    mut job: *mut GVJ_t,
    mut top: libc::c_int,
    mut sp: *mut state_t,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = agnameof(g as *mut libc::c_void);
    indent(job, (*sp).Level);
    gvprintf(
        job,
        b"\"name\": \"%s\"\0" as *const u8 as *const libc::c_char,
        stoj(name, sp),
    );
    if top != 0 {
        gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"directed\": %s,\n\0" as *const u8 as *const libc::c_char,
            if agisdirected(g) != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"strict\": %s\0" as *const u8 as *const libc::c_char,
            if agisstrict(g) != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    }
}
unsafe extern "C" fn write_subg(
    mut g: *mut Agraph_t,
    mut job: *mut GVJ_t,
    mut sp: *mut state_t,
) {
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    write_graph(g, job, 0 as libc::c_int, sp);
    sg = agfstsubg(g);
    while !sg.is_null() {
        gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
        write_subg(sg, job, sp);
        sg = agnxtsubg(sg);
    }
}
unsafe extern "C" fn write_subgs(
    mut g: *mut Agraph_t,
    mut job: *mut GVJ_t,
    mut top: libc::c_int,
    mut sp: *mut state_t,
) -> libc::c_int {
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    sg = agfstsubg(g);
    if sg.is_null() {
        return 0 as libc::c_int;
    }
    gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
    let ref mut fresh21 = (*sp).Level;
    let fresh22 = *fresh21;
    *fresh21 = *fresh21 + 1;
    indent(job, fresh22);
    if top != 0 {
        gvputs(job, b"\"objects\": [\n\0" as *const u8 as *const libc::c_char);
    } else {
        gvputs(job, b"\"subgraphs\": [\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
    }
    let mut separator: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    while !sg.is_null() {
        gvputs(job, separator);
        if top != 0 {
            write_subg(sg, job, sp);
        } else {
            gvprintf(
                job,
                b"%d\0" as *const u8 as *const libc::c_char,
                (*(aggetrec(
                    sg as *mut libc::c_void,
                    b"id\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut gvid_t))
                    .id,
            );
        }
        separator = b",\n\0" as *const u8 as *const libc::c_char;
        sg = agnxtsubg(sg);
    }
    if top == 0 {
        let ref mut fresh23 = (*sp).Level;
        *fresh23 -= 1;
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
        gvputs(job, b"]\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn agseqasc(
    mut lhs: *mut *mut Agedge_t,
    mut rhs: *mut *mut Agedge_t,
) -> libc::c_int {
    let mut e1: *mut Agedge_t = *lhs;
    let mut e2: *mut Agedge_t = *rhs;
    if (((*(e1 as *mut Agobj_t)).tag).seq() as libc::c_int)
        < ((*(e2 as *mut Agobj_t)).tag).seq() as libc::c_int
    {
        return -(1 as libc::c_int)
    } else if ((*(e1 as *mut Agobj_t)).tag).seq() as libc::c_int
            > ((*(e2 as *mut Agobj_t)).tag).seq() as libc::c_int
        {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn write_edge(
    mut e: *mut Agedge_t,
    mut job: *mut GVJ_t,
    mut top: libc::c_int,
    mut sp: *mut state_t,
) {
    if top != 0 {
        let ref mut fresh24 = (*sp).Level;
        let fresh25 = *fresh24;
        *fresh24 = *fresh24 + 1;
        indent(job, fresh25);
        gvputs(job, b"{\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"_gvid\": %d,\n\0" as *const u8 as *const libc::c_char,
            (*(aggetrec(
                e as *mut libc::c_void,
                b"id\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut gvid_t))
                .id,
        );
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"tail\": %d,\n\0" as *const u8 as *const libc::c_char,
            (*(aggetrec(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut libc::c_void,
                b"id\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut gvid_t))
                .id,
        );
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"head\": %d\0" as *const u8 as *const libc::c_char,
            (*(aggetrec(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut libc::c_void,
                b"id\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut gvid_t))
                .id,
        );
        write_attrs(e as *mut Agobj_t, job, sp);
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
        let ref mut fresh26 = (*sp).Level;
        *fresh26 -= 1;
        indent(job, (*sp).Level);
        gvputs(job, b"}\0" as *const u8 as *const libc::c_char);
    } else {
        gvprintf(
            job,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*(aggetrec(
                e as *mut libc::c_void,
                b"id\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut gvid_t))
                .id,
        );
    };
}
unsafe extern "C" fn write_edges(
    mut g: *mut Agraph_t,
    mut job: *mut GVJ_t,
    mut top: libc::c_int,
    mut sp: *mut state_t,
) -> libc::c_int {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut np: *mut Agnode_t = agfstnode(g);
    while !np.is_null() {
        let mut ep: *mut Agedge_t = agfstout(g, np);
        while !ep.is_null() {
            count = count.wrapping_add(1);
            ep = agnxtout(g, ep);
        }
        np = agnxtnode(g, np);
    }
    if count == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut edges: *mut *mut Agedge_t = gcalloc(
        count,
        ::std::mem::size_of::<*mut Agedge_t>() as libc::c_ulong,
    ) as *mut *mut Agedge_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut np_0: *mut Agnode_t = agfstnode(g);
    while !np_0.is_null() {
        let mut ep_0: *mut Agedge_t = agfstout(g, np_0);
        while !ep_0.is_null() {
            let ref mut fresh27 = *edges.offset(i as isize);
            *fresh27 = ep_0;
            i = i.wrapping_add(1);
            ep_0 = agnxtout(g, ep_0);
        }
        np_0 = agnxtnode(g, np_0);
    }
    qsort(
        edges as *mut libc::c_void,
        count,
        ::std::mem::size_of::<*mut Agedge_t>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut *mut Agedge_t,
                    *mut *mut Agedge_t,
                ) -> libc::c_int,
            >,
            qsort_cmpf,
        >(
            Some(
                agseqasc
                    as unsafe extern "C" fn(
                        *mut *mut Agedge_t,
                        *mut *mut Agedge_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
    let ref mut fresh28 = (*sp).Level;
    let fresh29 = *fresh28;
    *fresh28 = *fresh28 + 1;
    indent(job, fresh29);
    gvputs(job, b"\"edges\": [\n\0" as *const u8 as *const libc::c_char);
    if top == 0 {
        indent(job, (*sp).Level);
    }
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < count {
        if j > 0 as libc::c_int as libc::c_ulong {
            if top != 0 {
                gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
            } else {
                gvputs(job, b",\0" as *const u8 as *const libc::c_char);
            }
        }
        write_edge(*edges.offset(j as isize), job, top, sp);
        j = j.wrapping_add(1);
    }
    free(edges as *mut libc::c_void);
    let ref mut fresh30 = (*sp).Level;
    *fresh30 -= 1;
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    indent(job, (*sp).Level);
    gvputs(job, b"]\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn write_node(
    mut n: *mut Agnode_t,
    mut job: *mut GVJ_t,
    mut top: libc::c_int,
    mut sp: *mut state_t,
) {
    if top != 0 {
        let ref mut fresh31 = (*sp).Level;
        let fresh32 = *fresh31;
        *fresh31 = *fresh31 + 1;
        indent(job, fresh32);
        gvputs(job, b"{\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"_gvid\": %d,\n\0" as *const u8 as *const libc::c_char,
            (*(aggetrec(
                n as *mut libc::c_void,
                b"id\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut gvid_t))
                .id,
        );
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"name\": \"%s\"\0" as *const u8 as *const libc::c_char,
            stoj(agnameof(n as *mut libc::c_void), sp),
        );
        write_attrs(n as *mut Agobj_t, job, sp);
        gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
        let ref mut fresh33 = (*sp).Level;
        *fresh33 -= 1;
        indent(job, (*sp).Level);
        gvputs(job, b"}\0" as *const u8 as *const libc::c_char);
    } else {
        gvprintf(
            job,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*(aggetrec(
                n as *mut libc::c_void,
                b"id\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut gvid_t))
                .id,
        );
    };
}
unsafe extern "C" fn write_nodes(
    mut g: *mut Agraph_t,
    mut job: *mut GVJ_t,
    mut top: libc::c_int,
    mut has_subgs: libc::c_int,
    mut sp: *mut state_t,
) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfstnode(g);
    if n.is_null() {
        if has_subgs != 0 && top != 0 {
            let ref mut fresh34 = (*sp).Level;
            *fresh34 -= 1;
            gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
            indent(job, (*sp).Level);
            gvputs(job, b"]\0" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int;
    }
    gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
    if top != 0 {
        if has_subgs == 0 {
            let ref mut fresh35 = (*sp).Level;
            let fresh36 = *fresh35;
            *fresh35 = *fresh35 + 1;
            indent(job, fresh36);
            gvputs(job, b"\"objects\": [\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        let ref mut fresh37 = (*sp).Level;
        let fresh38 = *fresh37;
        *fresh37 = *fresh37 + 1;
        indent(job, fresh38);
        gvputs(job, b"\"nodes\": [\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
    }
    let mut separator: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    while !n.is_null() {
        if !(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
            gvputs(job, separator);
            write_node(n, job, top, sp);
            separator = if top != 0 {
                b",\n\0" as *const u8 as *const libc::c_char
            } else {
                b",\0" as *const u8 as *const libc::c_char
            };
        }
        n = agnxtnode(g, n);
    }
    let ref mut fresh39 = (*sp).Level;
    *fresh39 -= 1;
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    indent(job, (*sp).Level);
    gvputs(job, b"]\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn freef(
    mut dt: *mut Dt_t,
    mut obj: *mut intm,
    mut disc: *mut Dtdisc_t,
) {
    free((*obj).id as *mut libc::c_void);
    free(obj as *mut libc::c_void);
}
static mut intDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                *mut libc::c_void,
                Dtmake_f,
            >(0 as *const libc::c_void as *mut libc::c_void),
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut intm, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freef
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut intm,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                *mut libc::c_void,
                Dtcompar_f,
            >(0 as *const libc::c_void as *mut libc::c_void),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn lookup(
    mut map: *mut Dt_t,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut ip: *mut intm = (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, name as *mut libc::c_void, 0o1000 as libc::c_int) as *mut intm;
    if !ip.is_null() { return (*ip).v } else { return -(1 as libc::c_int) };
}
unsafe extern "C" fn insert(
    mut map: *mut Dt_t,
    mut name: *mut libc::c_char,
    mut v: libc::c_int,
) {
    let mut ip: *mut intm = (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, name as *mut libc::c_void, 0o1000 as libc::c_int) as *mut intm;
    if !ip.is_null() {
        if (*ip).v != v {
            agerr(
                AGWARN,
                b"Duplicate cluster name \"%s\"\n\0" as *const u8 as *const libc::c_char,
                name,
            );
        }
        return;
    }
    ip = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<intm>() as libc::c_ulong,
    ) as *mut intm;
    let ref mut fresh40 = (*ip).id;
    *fresh40 = strdup(name);
    (*ip).v = v;
    (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, ip as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn label_subgs(
    mut g: *mut Agraph_t,
    mut lbl: libc::c_int,
    mut map: *mut Dt_t,
) -> libc::c_int {
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    if g != agroot(g as *mut libc::c_void) {
        let fresh41 = lbl;
        lbl = lbl + 1;
        (*(aggetrec(
            g as *mut libc::c_void,
            b"id\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut gvid_t))
            .id = fresh41;
        if strncmp(
            agnameof(g as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            insert(
                map,
                agnameof(g as *mut libc::c_void),
                (*(aggetrec(
                    g as *mut libc::c_void,
                    b"id\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut gvid_t))
                    .id,
            );
        }
    }
    sg = agfstsubg(g);
    while !sg.is_null() {
        lbl = label_subgs(sg, lbl, map);
        sg = agnxtsubg(sg);
    }
    return lbl;
}
unsafe extern "C" fn write_graph(
    mut g: *mut Agraph_t,
    mut job: *mut GVJ_t,
    mut top: libc::c_int,
    mut sp: *mut state_t,
) {
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ep: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut ncnt: libc::c_int = 0 as libc::c_int;
    let mut ecnt: libc::c_int = 0 as libc::c_int;
    let mut sgcnt: libc::c_int = 0 as libc::c_int;
    let mut has_subgs: libc::c_int = 0;
    let mut map: *mut Dt_t = 0 as *mut Dt_t;
    if top != 0 {
        map = dtopen(&mut intDisc, Dtoset);
        aginit(
            g,
            1 as libc::c_int,
            b"id\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<gvid_t>() as libc::c_ulong as libc::c_int,
            0 as libc::c_int,
        );
        aginit(
            g,
            2 as libc::c_int,
            b"id\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<gvid_t>() as libc::c_ulong as libc::c_int,
            0 as libc::c_int,
        );
        aginit(
            g,
            0 as libc::c_int,
            b"id\0" as *const u8 as *const libc::c_char,
            -(::std::mem::size_of::<gvid_t>() as libc::c_ulong as libc::c_int),
            0 as libc::c_int,
        );
        sgcnt = label_subgs(g, sgcnt, map);
        np = agfstnode(g);
        while !np.is_null() {
            if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
                (*(aggetrec(
                    np as *mut libc::c_void,
                    b"id\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut gvid_t))
                    .id = lookup(map, agnameof(np as *mut libc::c_void));
            } else {
                let fresh42 = ncnt;
                ncnt = ncnt + 1;
                (*(aggetrec(
                    np as *mut libc::c_void,
                    b"id\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut gvid_t))
                    .id = sgcnt + fresh42;
            }
            ep = agfstout(g, np);
            while !ep.is_null() {
                let fresh43 = ecnt;
                ecnt = ecnt + 1;
                (*(aggetrec(
                    ep as *mut libc::c_void,
                    b"id\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut gvid_t))
                    .id = fresh43;
                ep = agnxtout(g, ep);
            }
            np = agnxtnode(g, np);
        }
        dtclose(map);
    }
    let ref mut fresh44 = (*sp).Level;
    let fresh45 = *fresh44;
    *fresh44 = *fresh44 + 1;
    indent(job, fresh45);
    gvputs(job, b"{\n\0" as *const u8 as *const libc::c_char);
    write_hdr(g, job, top, sp);
    write_attrs(g as *mut Agobj_t, job, sp);
    if top != 0 {
        gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"_subgraph_cnt\": %d\0" as *const u8 as *const libc::c_char,
            sgcnt,
        );
    } else {
        gvputs(job, b",\n\0" as *const u8 as *const libc::c_char);
        indent(job, (*sp).Level);
        gvprintf(
            job,
            b"\"_gvid\": %d\0" as *const u8 as *const libc::c_char,
            (*(aggetrec(
                g as *mut libc::c_void,
                b"id\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut gvid_t))
                .id,
        );
    }
    has_subgs = write_subgs(g, job, top, sp);
    write_nodes(g, job, top, has_subgs, sp);
    write_edges(g, job, top, sp);
    gvputs(job, b"\n\0" as *const u8 as *const libc::c_char);
    let ref mut fresh46 = (*sp).Level;
    *fresh46 -= 1;
    indent(job, (*sp).Level);
    if top != 0 {
        gvputs(job, b"}\n\0" as *const u8 as *const libc::c_char);
    } else {
        gvputs(job, b"}\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn json_end_graph(mut job: *mut GVJ_t) {
    let mut g: *mut graph_t = (*(*job).obj).u.g;
    let mut sp: state_t = state_t {
        Level: 0,
        isLatin: false,
        doXDot: false,
    };
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
    let ref mut fresh47 = (*(*g).clos).disc.io;
    *fresh47 = &mut io;
    set_attrwf(g, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    sp.Level = 0 as libc::c_int;
    sp
        .isLatin = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).charset
        as libc::c_int == 1 as libc::c_int;
    sp
        .doXDot = (*job).render.id == FORMAT_JSON as libc::c_int
        || (*job).render.id == FORMAT_XDOT_JSON as libc::c_int;
    write_graph(g, job, (0 as libc::c_int == 0) as libc::c_int, &mut sp);
}
#[no_mangle]
pub static mut json_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: None,
            end_job: None,
            begin_graph: Some(
                json_begin_graph as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            end_graph: Some(json_end_graph as unsafe extern "C" fn(*mut GVJ_t) -> ()),
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
pub static mut render_features_json1: gvrender_features_t = {
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
pub static mut render_features_json: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 13 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 23 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int,
        default_pad: 0.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: COLOR_STRING,
    };
    init
};
#[no_mangle]
pub static mut device_features_json_nop: gvdevice_features_t = {
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
pub static mut device_features_json: gvdevice_features_t = {
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
pub static mut gvrender_json_types: [gvplugin_installed_t; 5] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JSON as libc::c_int,
                type_0: b"json\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &json_engine as *const gvrender_engine_t
                    as *mut gvrender_engine_t as *mut libc::c_void,
                features: &render_features_json as *const gvrender_features_t
                    as *mut gvrender_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JSON0 as libc::c_int,
                type_0: b"json0\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &json_engine as *const gvrender_engine_t
                    as *mut gvrender_engine_t as *mut libc::c_void,
                features: &render_features_json as *const gvrender_features_t
                    as *mut gvrender_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_DOT_JSON as libc::c_int,
                type_0: b"dot_json\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &json_engine as *const gvrender_engine_t
                    as *mut gvrender_engine_t as *mut libc::c_void,
                features: &render_features_json as *const gvrender_features_t
                    as *mut gvrender_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_XDOT_JSON as libc::c_int,
                type_0: b"xdot_json\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &json_engine as *const gvrender_engine_t
                    as *mut gvrender_engine_t as *mut libc::c_void,
                features: &render_features_json as *const gvrender_features_t
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
pub static mut gvdevice_json_types: [gvplugin_installed_t; 5] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JSON as libc::c_int,
                type_0: b"json:json\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_json as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JSON0 as libc::c_int,
                type_0: b"json0:json\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_json as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_DOT_JSON as libc::c_int,
                type_0: b"dot_json:json\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_json_nop as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_XDOT_JSON as libc::c_int,
                type_0: b"xdot_json:json\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_json_nop as *const gvdevice_features_t
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
