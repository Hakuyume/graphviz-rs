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
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sincos(x: libc::c_double, s: *mut libc::c_double, c: *mut libc::c_double);
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Lib: *mut *const libc::c_char;
    static mut N_height: *mut Agsym_t;
    static mut N_width: *mut Agsym_t;
    static mut N_color: *mut Agsym_t;
    static mut N_fillcolor: *mut Agsym_t;
    static mut N_activepencolor: *mut Agsym_t;
    static mut N_activefillcolor: *mut Agsym_t;
    static mut N_selectedpencolor: *mut Agsym_t;
    static mut N_selectedfillcolor: *mut Agsym_t;
    static mut N_visitedpencolor: *mut Agsym_t;
    static mut N_visitedfillcolor: *mut Agsym_t;
    static mut N_deletedpencolor: *mut Agsym_t;
    static mut N_deletedfillcolor: *mut Agsym_t;
    static mut N_nojustify: *mut Agsym_t;
    static mut N_style: *mut Agsym_t;
    static mut N_sides: *mut Agsym_t;
    static mut N_peripheries: *mut Agsym_t;
    static mut N_orientation: *mut Agsym_t;
    static mut N_skew: *mut Agsym_t;
    static mut N_distortion: *mut Agsym_t;
    static mut N_fixed: *mut Agsym_t;
    static mut N_imagescale: *mut Agsym_t;
    static mut N_imagepos: *mut Agsym_t;
    static mut N_penwidth: *mut Agsym_t;
    static mut N_gradientangle: *mut Agsym_t;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn flip_rec_boxf(b: boxf, p: pointf) -> boxf;
    fn ccwrotatepf(p: pointf, ccwrot: libc::c_int) -> pointf;
    fn cwrotatepf(p: pointf, cwrot: libc::c_int) -> pointf;
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
    fn safefile(filename: *const libc::c_char) -> *const libc::c_char;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn findStopColor(
        colorlist: *mut libc::c_char,
        clrs: *mut *mut libc::c_char,
        frac: *mut libc::c_float,
    ) -> bool;
    fn polyBB(poly: *mut polygon_t) -> boxf;
    fn gvusershape_size(g: *mut graph_t, name: *mut libc::c_char) -> point;
    fn gvrender_begin_anchor(
        job: *mut GVJ_t,
        href: *mut libc::c_char,
        tooltip: *mut libc::c_char,
        target: *mut libc::c_char,
        id: *mut libc::c_char,
    );
    fn gvrender_end_anchor(job: *mut GVJ_t);
    fn gvrender_set_pencolor(job: *mut GVJ_t, name: *mut libc::c_char);
    fn gvrender_set_penwidth(job: *mut GVJ_t, penwidth: libc::c_double);
    fn gvrender_set_fillcolor(job: *mut GVJ_t, name: *mut libc::c_char);
    fn gvrender_set_gradient_vals(
        job: *mut GVJ_t,
        stopcolor: *mut libc::c_char,
        angle: libc::c_int,
        frac: libc::c_float,
    );
    fn gvrender_set_style(job: *mut GVJ_t, s: *mut *mut libc::c_char);
    fn gvrender_ellipse(
        job: *mut GVJ_t,
        AF: *mut pointf,
        n: libc::c_int,
        filled: libc::c_int,
    );
    fn gvrender_polygon(
        job: *mut GVJ_t,
        af: *mut pointf,
        n: libc::c_int,
        filled: libc::c_int,
    );
    fn gvrender_box(job: *mut GVJ_t, BF: boxf, filled: libc::c_int);
    fn gvrender_beziercurve(
        job: *mut GVJ_t,
        AF: *mut pointf,
        n: libc::c_int,
        arrow_at_start: libc::c_int,
        arrow_at_end: libc::c_int,
        filled: libc::c_int,
    );
    fn gvrender_polyline(job: *mut GVJ_t, AF: *mut pointf, n: libc::c_int);
    fn bezier_clip(
        inside_context: *mut inside_t,
        insidefn: Option::<unsafe extern "C" fn(*mut inside_t, pointf) -> bool>,
        sp: *mut pointf,
        left_inside: bool,
    );
    fn emit_label(job: *mut GVJ_t, emit_state: emit_state_t, _: *mut textlabel_t);
    fn stripedBox(
        job: *mut GVJ_t,
        AF: *mut pointf,
        clrs: *mut libc::c_char,
        rotate: libc::c_int,
    ) -> libc::c_int;
    fn wedgedEllipse(
        job: *mut GVJ_t,
        pf: *mut pointf,
        clrs: *mut libc::c_char,
    ) -> libc::c_int;
    fn parse_style(s: *mut libc::c_char) -> *mut *mut libc::c_char;
    fn epsf_free(n: *mut node_t);
    fn epsf_init(n: *mut node_t);
    fn free_label(_: *mut textlabel_t);
    fn make_label(
        obj: *mut libc::c_void,
        str: *mut libc::c_char,
        kind: libc::c_int,
        fontsize: libc::c_double,
        fontname: *mut libc::c_char,
        fontcolor: *mut libc::c_char,
    ) -> *mut textlabel_t;
    fn gvrender_usershape(
        job: *mut GVJ_t,
        name: *mut libc::c_char,
        AF: *mut pointf,
        n: libc::c_int,
        filled: bool,
        imagescale: *mut libc::c_char,
        imagepos: *mut libc::c_char,
    );
    fn html_port(
        n: *mut node_t,
        pname: *mut libc::c_char,
        sides: *mut libc::c_int,
    ) -> *mut boxf;
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
pub type uint64_t = __uint64_t;
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
pub struct field_t {
    pub size: pointf,
    pub b: boxf,
    pub n_flds: libc::c_int,
    pub lp: *mut textlabel_t,
    pub fld: *mut *mut field_t,
    pub id: *mut libc::c_char,
    pub LR: libc::c_uchar,
    pub sides: libc::c_uchar,
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
pub struct epsf_s {
    pub macro_id: libc::c_int,
    pub offset: point,
}
pub type epsf_t = epsf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly_desc_t {
    pub size_gen: Option::<unsafe extern "C" fn(pointf) -> pointf>,
    pub vertex_gen: Option::<unsafe extern "C" fn(*mut pointf, *mut pointf) -> ()>,
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
unsafe extern "C" fn mid_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = (p.x + q.x) / 2.0f64;
    r.y = (p.y + q.y) / 2.0f64;
    return r;
}
#[inline]
unsafe extern "C" fn interpolate_pointf(
    mut t: libc::c_double,
    mut p: pointf,
    mut q: pointf,
) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x + t * (q.x - p.x);
    r.y = p.y + t * (q.y - p.y);
    return r;
}
static mut Center: port = {
    let mut init = port {
        p: pointf { x: 0., y: 0. },
        theta: -(1 as libc::c_int) as libc::c_double,
        bp: 0 as *const boxf as *mut boxf,
        defined: false,
        constrained: false,
        clip: 1 as libc::c_int != 0,
        dyna: false,
        order: 0,
        side: 0,
        name: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
static mut point_style: [*mut libc::c_char; 3] = [
    b"invis\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"filled\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut star_gen: poly_desc_t = unsafe {
    {
        let mut init = poly_desc_t {
            size_gen: Some(star_size as unsafe extern "C" fn(pointf) -> pointf),
            vertex_gen: Some(
                star_vertices as unsafe extern "C" fn(*mut pointf, *mut pointf) -> (),
            ),
        };
        init
    }
};
static mut cylinder_gen: poly_desc_t = unsafe {
    {
        let mut init = poly_desc_t {
            size_gen: Some(cylinder_size as unsafe extern "C" fn(pointf) -> pointf),
            vertex_gen: Some(
                cylinder_vertices as unsafe extern "C" fn(*mut pointf, *mut pointf) -> (),
            ),
        };
        init
    }
};
static mut p_polygon: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 0,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_ellipse: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 1 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_circle: polygon_t = {
    let mut init = polygon_t {
        regular: 1 as libc::c_int,
        peripheries: 1 as libc::c_int,
        sides: 1 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_egg: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 1 as libc::c_int,
        orientation: 0.,
        distortion: -0.3f64,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_triangle: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 3 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_box: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_square: polygon_t = {
    let mut init = polygon_t {
        regular: 1 as libc::c_int,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_plaintext: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 0,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_plain: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 0,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_diamond: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 45.0f64,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_trapezium: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: -0.4f64,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_parallelogram: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.6f64,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_house: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 5 as libc::c_int,
        orientation: 0.,
        distortion: -0.64f64,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_pentagon: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 5 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_hexagon: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 6 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_septagon: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 7 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_octagon: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 8 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_note: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (1 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_tab: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (2 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_folder: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (3 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_box3d: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (4 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_component: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (5 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_underline: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (1 as libc::c_int) << 10 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_cylinder: polygon_t = unsafe {
    {
        let mut init = polygon_t {
            regular: 0,
            peripheries: 1 as libc::c_int,
            sides: 19 as libc::c_int,
            orientation: 0.,
            distortion: 0.,
            skew: 0.,
            option: (26 as libc::c_int) << 24 as libc::c_int,
            vertices: &cylinder_gen as *const poly_desc_t as *mut poly_desc_t
                as *mut pointf,
        };
        init
    }
};
static mut p_doublecircle: polygon_t = {
    let mut init = polygon_t {
        regular: 1 as libc::c_int,
        peripheries: 2 as libc::c_int,
        sides: 1 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_invtriangle: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 3 as libc::c_int,
        orientation: 180.0f64,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_invtrapezium: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 180.0f64,
        distortion: -0.4f64,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_invhouse: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 5 as libc::c_int,
        orientation: 180.0f64,
        distortion: -0.64f64,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_doubleoctagon: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 2 as libc::c_int,
        sides: 8 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_tripleoctagon: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 3 as libc::c_int,
        sides: 8 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: 0,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_Mdiamond: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 45.0f64,
        distortion: 0.,
        skew: 0.,
        option: (1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_Msquare: polygon_t = {
    let mut init = polygon_t {
        regular: 1 as libc::c_int,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (1 as libc::c_int) << 3 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_Mcircle: polygon_t = {
    let mut init = polygon_t {
        regular: 1 as libc::c_int,
        peripheries: 1 as libc::c_int,
        sides: 1 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_star: polygon_t = unsafe {
    {
        let mut init = polygon_t {
            regular: 0,
            peripheries: 1 as libc::c_int,
            sides: 10 as libc::c_int,
            orientation: 0.,
            distortion: 0.,
            skew: 0.,
            option: 0,
            vertices: &star_gen as *const poly_desc_t as *mut poly_desc_t as *mut pointf,
        };
        init
    }
};
static mut p_promoter: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (6 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_cds: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (7 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_terminator: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (8 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_utr: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (9 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_insulator: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (17 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_ribosite: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (18 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_rnastab: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (19 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_proteasesite: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (20 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_proteinstab: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (21 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_primersite: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (10 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_restrictionsite: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (11 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_fivepoverhang: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (12 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_threepoverhang: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (13 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_noverhang: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (14 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_assembly: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (15 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_signature: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (16 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_rpromoter: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (22 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_rarrow: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (23 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_larrow: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (24 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut p_lpromoter: polygon_t = {
    let mut init = polygon_t {
        regular: 0,
        peripheries: 1 as libc::c_int,
        sides: 4 as libc::c_int,
        orientation: 0.,
        distortion: 0.,
        skew: 0.,
        option: (25 as libc::c_int) << 24 as libc::c_int,
        vertices: 0 as *const pointf as *mut pointf,
    };
    init
};
static mut poly_fns: shape_functions = unsafe {
    {
        let mut init = shape_functions {
            initfn: Some(poly_init as unsafe extern "C" fn(*mut node_t) -> ()),
            freefn: Some(poly_free as unsafe extern "C" fn(*mut node_t) -> ()),
            portfn: Some(
                poly_port
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> port,
            ),
            insidefn: Some(
                poly_inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool,
            ),
            pboxfn: Some(
                poly_path
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut port,
                        libc::c_int,
                        *mut boxf,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            codefn: Some(
                poly_gencode as unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> (),
            ),
        };
        init
    }
};
static mut point_fns: shape_functions = unsafe {
    {
        let mut init = shape_functions {
            initfn: Some(point_init as unsafe extern "C" fn(*mut node_t) -> ()),
            freefn: Some(poly_free as unsafe extern "C" fn(*mut node_t) -> ()),
            portfn: Some(
                poly_port
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> port,
            ),
            insidefn: Some(
                point_inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool,
            ),
            pboxfn: None,
            codefn: Some(
                point_gencode as unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> (),
            ),
        };
        init
    }
};
static mut record_fns: shape_functions = unsafe {
    {
        let mut init = shape_functions {
            initfn: Some(record_init as unsafe extern "C" fn(*mut node_t) -> ()),
            freefn: Some(record_free as unsafe extern "C" fn(*mut node_t) -> ()),
            portfn: Some(
                record_port
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> port,
            ),
            insidefn: Some(
                record_inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool,
            ),
            pboxfn: Some(
                record_path
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut port,
                        libc::c_int,
                        *mut boxf,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            codefn: Some(
                record_gencode as unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> (),
            ),
        };
        init
    }
};
static mut epsf_fns: shape_functions = unsafe {
    {
        let mut init = shape_functions {
            initfn: Some(epsf_init as unsafe extern "C" fn(*mut node_t) -> ()),
            freefn: Some(epsf_free as unsafe extern "C" fn(*mut node_t) -> ()),
            portfn: Some(
                poly_port
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> port,
            ),
            insidefn: Some(
                epsf_inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool,
            ),
            pboxfn: None,
            codefn: Some(
                epsf_gencode as unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> (),
            ),
        };
        init
    }
};
static mut star_fns: shape_functions = unsafe {
    {
        let mut init = shape_functions {
            initfn: Some(poly_init as unsafe extern "C" fn(*mut node_t) -> ()),
            freefn: Some(poly_free as unsafe extern "C" fn(*mut node_t) -> ()),
            portfn: Some(
                poly_port
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> port,
            ),
            insidefn: Some(
                star_inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool,
            ),
            pboxfn: Some(
                poly_path
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut port,
                        libc::c_int,
                        *mut boxf,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            codefn: Some(
                poly_gencode as unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> (),
            ),
        };
        init
    }
};
static mut cylinder_fns: shape_functions = unsafe {
    {
        let mut init = shape_functions {
            initfn: Some(poly_init as unsafe extern "C" fn(*mut node_t) -> ()),
            freefn: Some(poly_free as unsafe extern "C" fn(*mut node_t) -> ()),
            portfn: Some(
                poly_port
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> port,
            ),
            insidefn: Some(
                poly_inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool,
            ),
            pboxfn: Some(
                poly_path
                    as unsafe extern "C" fn(
                        *mut node_t,
                        *mut port,
                        libc::c_int,
                        *mut boxf,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            codefn: Some(
                poly_gencode as unsafe extern "C" fn(*mut GVJ_t, *mut node_t) -> (),
            ),
        };
        init
    }
};
static mut Shapes: [shape_desc; 63] = unsafe {
    [
        {
            let mut init = shape_desc {
                name: b"box\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_box as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"polygon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_polygon as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"ellipse\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_ellipse as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"oval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_ellipse as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"circle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_circle as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"point\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &point_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_circle as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"egg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_egg as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"triangle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_triangle as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_plaintext as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"plaintext\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_plaintext as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"plain\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_plain as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"diamond\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_diamond as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"trapezium\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_trapezium as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"parallelogram\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_parallelogram as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"house\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_house as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"pentagon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_pentagon as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"hexagon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_hexagon as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"septagon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_septagon as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"octagon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_octagon as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"note\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_note as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"tab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_tab as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"folder\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_folder as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"box3d\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_box3d as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"component\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_component as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"cylinder\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &cylinder_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_cylinder as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"rect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_box as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"rectangle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_box as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"square\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_square as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"doublecircle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_doublecircle as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"doubleoctagon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_doubleoctagon as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"tripleoctagon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_tripleoctagon as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"invtriangle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_invtriangle as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"invtrapezium\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_invtrapezium as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"invhouse\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_invhouse as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"underline\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_underline as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"Mdiamond\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_Mdiamond as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"Msquare\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_Msquare as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"Mcircle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_Mcircle as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"promoter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_promoter as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"cds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_cds as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"terminator\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_terminator as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"utr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_utr as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"insulator\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_insulator as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"ribosite\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_ribosite as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"rnastab\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_rnastab as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"proteasesite\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_proteasesite as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"proteinstab\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_proteinstab as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"primersite\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_primersite as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"restrictionsite\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_restrictionsite as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"fivepoverhang\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_fivepoverhang as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"threepoverhang\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_threepoverhang as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"noverhang\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_noverhang as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"assembly\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_assembly as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"signature\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_signature as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"rpromoter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_rpromoter as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"larrow\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_larrow as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"rarrow\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_rarrow as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"lpromoter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &poly_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_lpromoter as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"record\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &record_fns as *const shape_functions as *mut shape_functions,
                polygon: 0 as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"Mrecord\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fns: &record_fns as *const shape_functions as *mut shape_functions,
                polygon: 0 as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"epsf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &epsf_fns as *const shape_functions as *mut shape_functions,
                polygon: 0 as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: b"star\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fns: &star_fns as *const shape_functions as *mut shape_functions,
                polygon: &p_star as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
        {
            let mut init = shape_desc {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                fns: 0 as *const shape_functions as *mut shape_functions,
                polygon: 0 as *const polygon_t as *mut polygon_t,
                usershape: false,
            };
            init
        },
    ]
};
unsafe extern "C" fn unrecognized(mut n: *mut node_t, mut p: *mut libc::c_char) {
    agerr(
        AGWARN,
        b"node %s, port %s unrecognized\n\0" as *const u8 as *const libc::c_char,
        agnameof(n as *mut libc::c_void),
        p,
    );
}
unsafe extern "C" fn quant(
    mut val: libc::c_double,
    mut q: libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    i = (val / q) as libc::c_int;
    if i as libc::c_double * q + 0.00001f64 < val {
        i += 1;
    }
    return i as libc::c_double * q;
}
unsafe extern "C" fn same_side(
    mut p0: pointf,
    mut p1: pointf,
    mut L0: pointf,
    mut L1: pointf,
) -> libc::c_int {
    let mut s0: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    a = -(L1.y - L0.y);
    b = L1.x - L0.x;
    c = a * L0.x + b * L0.y;
    s0 = (a * p0.x + b * p0.y - c >= 0 as libc::c_int as libc::c_double) as libc::c_int;
    s1 = (a * p1.x + b * p1.y - c >= 0 as libc::c_int as libc::c_double) as libc::c_int;
    return (s0 == s1) as libc::c_int;
}
unsafe extern "C" fn penColor(
    mut job: *mut GVJ_t,
    mut n: *mut node_t,
) -> *mut libc::c_char {
    let mut color: *mut libc::c_char = 0 as *mut libc::c_char;
    color = late_nnstring(
        n as *mut libc::c_void,
        N_color,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *color.offset(0 as libc::c_int as isize) == 0 {
        color = b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    gvrender_set_pencolor(job, color);
    return color;
}
unsafe extern "C" fn findFillDflt(
    mut n: *mut node_t,
    mut dflt: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut color: *mut libc::c_char = 0 as *mut libc::c_char;
    color = late_nnstring(
        n as *mut libc::c_void,
        N_fillcolor,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *color.offset(0 as libc::c_int as isize) == 0 {
        color = late_nnstring(
            n as *mut libc::c_void,
            N_color,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *color.offset(0 as libc::c_int as isize) == 0 {
            color = dflt;
        }
    }
    return color;
}
unsafe extern "C" fn findFill(mut n: *mut node_t) -> *mut libc::c_char {
    return findFillDflt(
        n,
        b"lightgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn isBox(mut n: *mut node_t) -> libc::c_int {
    let mut p: *mut polygon_t = 0 as *mut polygon_t;
    p = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon;
    if !p.is_null() {
        return ((*p).sides == 4 as libc::c_int
            && (if (*p).orientation >= 0 as libc::c_int as libc::c_double {
                ((*p).orientation + 0.5f64) as libc::c_int
            } else {
                ((*p).orientation - 0.5f64) as libc::c_int
            }) % 90 as libc::c_int == 0 as libc::c_int && (*p).distortion == 0.0f64
            && (*p).skew == 0.0f64) as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn isEllipse(mut n: *mut node_t) -> libc::c_int {
    let mut p: *mut polygon_t = 0 as *mut polygon_t;
    p = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon;
    if !p.is_null() {
        return ((*p).sides <= 2 as libc::c_int) as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn checkStyle(
    mut n: *mut node_t,
    mut flagp: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut style: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pstyle: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut istyle: libc::c_int = 0 as libc::c_int;
    let mut poly: *mut polygon_t = 0 as *mut polygon_t;
    style = late_nnstring(
        n as *mut libc::c_void,
        N_style,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *style.offset(0 as libc::c_int as isize) != 0 {
        let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut qp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        pstyle = parse_style(style);
        pp = pstyle;
        loop {
            p = *pp;
            if p.is_null() {
                break;
            }
            if strcmp(p, b"filled\0" as *const u8 as *const libc::c_char) == 0 {
                istyle |= (1 as libc::c_int) << 0 as libc::c_int;
                pp = pp.offset(1);
            } else if strcmp(p, b"rounded\0" as *const u8 as *const libc::c_char) == 0 {
                istyle |= (1 as libc::c_int) << 2 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh0 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh0 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else if strcmp(p, b"diagonals\0" as *const u8 as *const libc::c_char) == 0
                {
                istyle |= (1 as libc::c_int) << 3 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh1 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh1 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else if strcmp(p, b"invis\0" as *const u8 as *const libc::c_char) == 0 {
                istyle |= (1 as libc::c_int) << 5 as libc::c_int;
                pp = pp.offset(1);
            } else if strcmp(p, b"radial\0" as *const u8 as *const libc::c_char) == 0 {
                istyle
                    |= (1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh2 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh2 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else if strcmp(p, b"striped\0" as *const u8 as *const libc::c_char) == 0
                    && isBox(n) != 0
                {
                istyle |= (1 as libc::c_int) << 6 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh3 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh3 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else if strcmp(p, b"wedged\0" as *const u8 as *const libc::c_char) == 0
                    && isEllipse(n) != 0
                {
                istyle |= (1 as libc::c_int) << 9 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh4 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh4 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else {
                pp = pp.offset(1);
            }
        }
    }
    poly = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon;
    if !poly.is_null() {
        istyle |= (*poly).option;
    }
    *flagp = istyle;
    return pstyle;
}
unsafe extern "C" fn stylenode(mut job: *mut GVJ_t, mut n: *mut node_t) -> libc::c_int {
    let mut pstyle: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut istyle: libc::c_int = 0;
    let mut penwidth: libc::c_double = 0.;
    pstyle = checkStyle(n, &mut istyle);
    if !pstyle.is_null() {
        gvrender_set_style(job, pstyle);
    }
    if !N_penwidth.is_null()
        && {
            s = agxget(n as *mut libc::c_void, N_penwidth);
            !s.is_null()
        } && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        penwidth = late_double(n as *mut libc::c_void, N_penwidth, 1.0f64, 0.0f64);
        gvrender_set_penwidth(job, penwidth);
    }
    return istyle;
}
unsafe extern "C" fn Mcircle_hack(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut AF: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut p: pointf = pointf { x: 0., y: 0. };
    y = 0.7500f64;
    x = 0.6614f64;
    p.y = y * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
    p.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw * x;
    AF[0 as libc::c_int
        as usize] = add_pointf(
        p,
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
    );
    AF[1 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y;
    AF[1 as libc::c_int as usize]
        .x = AF[0 as libc::c_int as usize].x - 2 as libc::c_int as libc::c_double * p.x;
    gvrender_polyline(job, AF.as_mut_ptr(), 2 as libc::c_int);
    AF[0 as libc::c_int as usize].y -= 2 as libc::c_int as libc::c_double * p.y;
    AF[1 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y;
    gvrender_polyline(job, AF.as_mut_ptr(), 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn round_corners(
    mut job: *mut GVJ_t,
    mut AF: *mut pointf,
    mut sides: libc::c_int,
    mut style: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut B: *mut pointf = 0 as *mut pointf;
    let mut C: [pointf; 5] = [pointf { x: 0., y: 0. }; 5];
    let mut D: *mut pointf = 0 as *mut pointf;
    let mut p0: pointf = pointf { x: 0., y: 0. };
    let mut p1: pointf = pointf { x: 0., y: 0. };
    let mut rbconst: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut seg: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut shape: libc::c_int = 0;
    let mut pts: *mut pointf = 0 as *mut pointf;
    shape = style & (127 as libc::c_int) << 24 as libc::c_int;
    if style & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        mode = (1 as libc::c_int) << 3 as libc::c_int;
    } else if style & (127 as libc::c_int) << 24 as libc::c_int != 0 {
        mode = shape;
    } else {
        mode = (1 as libc::c_int) << 2 as libc::c_int;
    }
    if mode == (26 as libc::c_int) << 24 as libc::c_int {
        cylinder_draw(job, AF, sides, filled);
        return;
    }
    B = gcalloc(
        (4 as libc::c_int * sides + 4 as libc::c_int) as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    i = 0 as libc::c_int;
    rbconst = 12 as libc::c_int as libc::c_double;
    seg = 0 as libc::c_int;
    while seg < sides {
        p0 = *AF.offset(seg as isize);
        if seg < sides - 1 as libc::c_int {
            p1 = *AF.offset((seg + 1 as libc::c_int) as isize);
        } else {
            p1 = *AF.offset(0 as libc::c_int as isize);
        }
        dx = p1.x - p0.x;
        dy = p1.y - p0.y;
        d = hypot(dx, dy);
        rbconst = if rbconst < d / 3.0f64 { rbconst } else { d / 3.0f64 };
        seg += 1;
    }
    seg = 0 as libc::c_int;
    while seg < sides {
        p0 = *AF.offset(seg as isize);
        if seg < sides - 1 as libc::c_int {
            p1 = *AF.offset((seg + 1 as libc::c_int) as isize);
        } else {
            p1 = *AF.offset(0 as libc::c_int as isize);
        }
        dx = p1.x - p0.x;
        dy = p1.y - p0.y;
        d = hypot(dx, dy);
        t = rbconst / d;
        if shape == (4 as libc::c_int) << 24 as libc::c_int
            || shape == (5 as libc::c_int) << 24 as libc::c_int
        {
            t /= 3 as libc::c_int as libc::c_double;
        } else if shape == (1 as libc::c_int) << 24 as libc::c_int {
            t /= 2 as libc::c_int as libc::c_double;
        }
        if mode != (1 as libc::c_int) << 2 as libc::c_int {
            let fresh5 = i;
            i = i + 1;
            *B.offset(fresh5 as isize) = p0;
        } else {
            let fresh6 = i;
            i = i + 1;
            *B.offset(fresh6 as isize) = interpolate_pointf(0.5f64 * t, p0, p1);
        }
        let fresh7 = i;
        i = i + 1;
        *B.offset(fresh7 as isize) = interpolate_pointf(t, p0, p1);
        let fresh8 = i;
        i = i + 1;
        *B.offset(fresh8 as isize) = interpolate_pointf(1.0f64 - t, p0, p1);
        if mode == (1 as libc::c_int) << 2 as libc::c_int {
            let fresh9 = i;
            i = i + 1;
            *B.offset(fresh9 as isize) = interpolate_pointf(1.0f64 - 0.5f64 * t, p0, p1);
        }
        seg += 1;
    }
    let fresh10 = i;
    i = i + 1;
    *B.offset(fresh10 as isize) = *B.offset(0 as libc::c_int as isize);
    let fresh11 = i;
    i = i + 1;
    *B.offset(fresh11 as isize) = *B.offset(1 as libc::c_int as isize);
    let fresh12 = i;
    i = i + 1;
    *B.offset(fresh12 as isize) = *B.offset(2 as libc::c_int as isize);
    match mode {
        4 => {
            pts = gcalloc(
                (6 as libc::c_int * sides + 2 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            i = 0 as libc::c_int;
            seg = 0 as libc::c_int;
            while seg < sides {
                let fresh13 = i;
                i = i + 1;
                *pts
                    .offset(
                        fresh13 as isize,
                    ) = *B.offset((4 as libc::c_int * seg) as isize);
                let fresh14 = i;
                i = i + 1;
                *pts
                    .offset(
                        fresh14 as isize,
                    ) = *B.offset((4 as libc::c_int * seg + 1 as libc::c_int) as isize);
                let fresh15 = i;
                i = i + 1;
                *pts
                    .offset(
                        fresh15 as isize,
                    ) = *B.offset((4 as libc::c_int * seg + 1 as libc::c_int) as isize);
                let fresh16 = i;
                i = i + 1;
                *pts
                    .offset(
                        fresh16 as isize,
                    ) = *B.offset((4 as libc::c_int * seg + 2 as libc::c_int) as isize);
                let fresh17 = i;
                i = i + 1;
                *pts
                    .offset(
                        fresh17 as isize,
                    ) = *B.offset((4 as libc::c_int * seg + 2 as libc::c_int) as isize);
                let fresh18 = i;
                i = i + 1;
                *pts
                    .offset(
                        fresh18 as isize,
                    ) = *B.offset((4 as libc::c_int * seg + 3 as libc::c_int) as isize);
                seg += 1;
            }
            let fresh19 = i;
            i = i + 1;
            *pts.offset(fresh19 as isize) = *pts.offset(0 as libc::c_int as isize);
            let fresh20 = i;
            i = i + 1;
            *pts.offset(fresh20 as isize) = *pts.offset(1 as libc::c_int as isize);
            gvrender_beziercurve(
                job,
                pts.offset(1 as libc::c_int as isize),
                i - 1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                filled,
            );
            free(pts as *mut libc::c_void);
        }
        8 => {
            gvrender_polygon(job, AF, sides, filled);
            seg = 0 as libc::c_int;
            while seg < sides {
                C[0 as libc::c_int
                    as usize] = *B
                    .offset((3 as libc::c_int * seg + 2 as libc::c_int) as isize);
                C[1 as libc::c_int
                    as usize] = *B
                    .offset((3 as libc::c_int * seg + 4 as libc::c_int) as isize);
                gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
                seg += 1;
            }
        }
        16777216 => {
            D = gcalloc(
                (sides + 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            seg = 1 as libc::c_int;
            while seg < sides {
                *D.offset(seg as isize) = *AF.offset(seg as isize);
                seg += 1;
            }
            *D
                .offset(
                    0 as libc::c_int as isize,
                ) = *B
                .offset(
                    (3 as libc::c_int * (sides - 1 as libc::c_int) + 4 as libc::c_int)
                        as isize,
                );
            *D
                .offset(
                    sides as isize,
                ) = *B
                .offset(
                    (3 as libc::c_int * (sides - 1 as libc::c_int) + 2 as libc::c_int)
                        as isize,
                );
            gvrender_polygon(job, D, sides + 1 as libc::c_int, filled);
            free(D as *mut libc::c_void);
            seg = sides - 1 as libc::c_int;
            C[0 as libc::c_int
                as usize] = *B
                .offset((3 as libc::c_int * seg + 2 as libc::c_int) as isize);
            C[1 as libc::c_int
                as usize] = *B
                .offset((3 as libc::c_int * seg + 4 as libc::c_int) as isize);
            C[2 as libc::c_int as usize]
                .x = C[1 as libc::c_int as usize].x
                + (C[0 as libc::c_int as usize].x
                    - (*B.offset((3 as libc::c_int * seg + 3 as libc::c_int) as isize))
                        .x);
            C[2 as libc::c_int as usize]
                .y = C[1 as libc::c_int as usize].y
                + (C[0 as libc::c_int as usize].y
                    - (*B.offset((3 as libc::c_int * seg + 3 as libc::c_int) as isize))
                        .y);
            gvrender_polyline(
                job,
                C.as_mut_ptr().offset(1 as libc::c_int as isize),
                2 as libc::c_int,
            );
            C[1 as libc::c_int as usize] = C[2 as libc::c_int as usize];
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
        }
        33554432 => {
            D = gcalloc(
                (sides + 2 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            *D.offset(0 as libc::c_int as isize) = *AF.offset(0 as libc::c_int as isize);
            *D.offset(1 as libc::c_int as isize) = *B.offset(2 as libc::c_int as isize);
            (*D.offset(2 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(3 as libc::c_int as isize)).x
                    - (*B.offset(4 as libc::c_int as isize)).x)
                    / 3 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*B.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 3 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*B.offset(3 as libc::c_int as isize)).x
                + ((*B.offset(3 as libc::c_int as isize)).x
                    - (*B.offset(4 as libc::c_int as isize)).x)
                    / 3 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 3 as libc::c_int as libc::c_double;
            seg = 4 as libc::c_int;
            while seg < sides + 2 as libc::c_int {
                *D.offset(seg as isize) = *AF.offset((seg - 2 as libc::c_int) as isize);
                seg += 1;
            }
            gvrender_polygon(job, D, sides + 2 as libc::c_int, filled);
            free(D as *mut libc::c_void);
            C[0 as libc::c_int as usize] = *B.offset(3 as libc::c_int as isize);
            C[1 as libc::c_int as usize] = *B.offset(2 as libc::c_int as isize);
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
        }
        50331648 => {
            D = gcalloc(
                (sides + 3 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            *D.offset(0 as libc::c_int as isize) = *AF.offset(0 as libc::c_int as isize);
            (*D.offset(1 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x
                - ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*B.offset(1 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 3 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x
                - 2 as libc::c_int as libc::c_double
                    * ((*AF.offset(0 as libc::c_int as isize)).x
                        - (*B.offset(1 as libc::c_int as isize)).x);
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x
                - 2.25f64
                    * ((*AF.offset(0 as libc::c_int as isize)).x
                        - (*B.offset(1 as libc::c_int as isize)).x);
            (*D.offset(3 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*B.offset(3 as libc::c_int as isize)).x;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y;
            seg = 4 as libc::c_int;
            while seg < sides + 3 as libc::c_int {
                *D.offset(seg as isize) = *AF.offset((seg - 3 as libc::c_int) as isize);
                seg += 1;
            }
            gvrender_polygon(job, D, sides + 3 as libc::c_int, filled);
            free(D as *mut libc::c_void);
        }
        67108864 => {
            if sides == 4 as libc::c_int {} else {
                __assert_fail(
                    b"sides == 4\0" as *const u8 as *const libc::c_char,
                    b"shapes.c\0" as *const u8 as *const libc::c_char,
                    709 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 53],
                        &[libc::c_char; 53],
                    >(b"void round_corners(GVJ_t *, pointf *, int, int, int)\0"))
                        .as_ptr(),
                );
            }
            D = gcalloc(
                (sides + 2 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            *D.offset(0 as libc::c_int as isize) = *AF.offset(0 as libc::c_int as isize);
            *D.offset(1 as libc::c_int as isize) = *B.offset(2 as libc::c_int as isize);
            *D.offset(2 as libc::c_int as isize) = *B.offset(4 as libc::c_int as isize);
            *D.offset(3 as libc::c_int as isize) = *AF.offset(2 as libc::c_int as isize);
            *D.offset(4 as libc::c_int as isize) = *B.offset(8 as libc::c_int as isize);
            *D.offset(5 as libc::c_int as isize) = *B.offset(10 as libc::c_int as isize);
            gvrender_polygon(job, D, sides + 2 as libc::c_int, filled);
            free(D as *mut libc::c_void);
            C[0 as libc::c_int as usize]
                .x = (*B.offset(1 as libc::c_int as isize)).x
                + ((*B.offset(11 as libc::c_int as isize)).x
                    - (*B.offset(0 as libc::c_int as isize)).x);
            C[0 as libc::c_int as usize]
                .y = (*B.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(11 as libc::c_int as isize)).y
                    - (*B.offset(0 as libc::c_int as isize)).y);
            C[1 as libc::c_int as usize] = *B.offset(4 as libc::c_int as isize);
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[1 as libc::c_int as usize] = *B.offset(8 as libc::c_int as isize);
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[1 as libc::c_int as usize] = *B.offset(0 as libc::c_int as isize);
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
        }
        83886080 => {
            if sides == 4 as libc::c_int {} else {
                __assert_fail(
                    b"sides == 4\0" as *const u8 as *const libc::c_char,
                    b"shapes.c\0" as *const u8 as *const libc::c_char,
                    732 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 53],
                        &[libc::c_char; 53],
                    >(b"void round_corners(GVJ_t *, pointf *, int, int, int)\0"))
                        .as_ptr(),
                );
            }
            D = gcalloc(
                (sides + 8 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            *D.offset(0 as libc::c_int as isize) = *AF.offset(0 as libc::c_int as isize);
            *D.offset(1 as libc::c_int as isize) = *AF.offset(1 as libc::c_int as isize);
            (*D.offset(2 as libc::c_int as isize))
                .x = (*B.offset(3 as libc::c_int as isize)).x
                + ((*B.offset(4 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(2 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y
                + ((*B.offset(4 as libc::c_int as isize)).y
                    - (*B.offset(3 as libc::c_int as isize)).y);
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(3 as libc::c_int as isize)).x
                    - (*B.offset(2 as libc::c_int as isize)).x);
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(2 as libc::c_int as isize)).y);
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(3 as libc::c_int as isize)).x
                + ((*B.offset(4 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y
                + ((*B.offset(4 as libc::c_int as isize)).y
                    - (*B.offset(3 as libc::c_int as isize)).y);
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(4 as libc::c_int as isize)).x
                + ((*D.offset(2 as libc::c_int as isize)).x
                    - (*D.offset(3 as libc::c_int as isize)).x);
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(4 as libc::c_int as isize)).y
                + ((*D.offset(2 as libc::c_int as isize)).y
                    - (*D.offset(3 as libc::c_int as isize)).y);
            (*D.offset(9 as libc::c_int as isize))
                .x = (*B.offset(6 as libc::c_int as isize)).x
                + ((*B.offset(5 as libc::c_int as isize)).x
                    - (*B.offset(6 as libc::c_int as isize)).x);
            (*D.offset(9 as libc::c_int as isize))
                .y = (*B.offset(6 as libc::c_int as isize)).y
                + ((*B.offset(5 as libc::c_int as isize)).y
                    - (*B.offset(6 as libc::c_int as isize)).y);
            (*D.offset(8 as libc::c_int as isize))
                .x = (*D.offset(9 as libc::c_int as isize)).x
                + ((*B.offset(6 as libc::c_int as isize)).x
                    - (*B.offset(7 as libc::c_int as isize)).x);
            (*D.offset(8 as libc::c_int as isize))
                .y = (*D.offset(9 as libc::c_int as isize)).y
                + ((*B.offset(6 as libc::c_int as isize)).y
                    - (*B.offset(7 as libc::c_int as isize)).y);
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x
                + ((*B.offset(5 as libc::c_int as isize)).x
                    - (*B.offset(6 as libc::c_int as isize)).x);
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(8 as libc::c_int as isize)).y
                + ((*B.offset(5 as libc::c_int as isize)).y
                    - (*B.offset(6 as libc::c_int as isize)).y);
            (*D.offset(6 as libc::c_int as isize))
                .x = (*D.offset(7 as libc::c_int as isize)).x
                + ((*D.offset(9 as libc::c_int as isize)).x
                    - (*D.offset(8 as libc::c_int as isize)).x);
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(7 as libc::c_int as isize)).y
                + ((*D.offset(9 as libc::c_int as isize)).y
                    - (*D.offset(8 as libc::c_int as isize)).y);
            *D
                .offset(
                    10 as libc::c_int as isize,
                ) = *AF.offset(2 as libc::c_int as isize);
            *D
                .offset(
                    11 as libc::c_int as isize,
                ) = *AF.offset(3 as libc::c_int as isize);
            gvrender_polygon(job, D, sides + 8 as libc::c_int, filled);
            C[0 as libc::c_int as usize] = *D.offset(2 as libc::c_int as isize);
            C[1 as libc::c_int as usize]
                .x = (*D.offset(2 as libc::c_int as isize)).x
                - ((*D.offset(3 as libc::c_int as isize)).x
                    - (*D.offset(2 as libc::c_int as isize)).x);
            C[1 as libc::c_int as usize]
                .y = (*D.offset(2 as libc::c_int as isize)).y
                - ((*D.offset(3 as libc::c_int as isize)).y
                    - (*D.offset(2 as libc::c_int as isize)).y);
            C[2 as libc::c_int as usize]
                .x = C[1 as libc::c_int as usize].x
                + ((*D.offset(4 as libc::c_int as isize)).x
                    - (*D.offset(3 as libc::c_int as isize)).x);
            C[2 as libc::c_int as usize]
                .y = C[1 as libc::c_int as usize].y
                + ((*D.offset(4 as libc::c_int as isize)).y
                    - (*D.offset(3 as libc::c_int as isize)).y);
            C[3 as libc::c_int as usize] = *D.offset(5 as libc::c_int as isize);
            gvrender_polyline(job, C.as_mut_ptr(), 4 as libc::c_int);
            C[0 as libc::c_int as usize] = *D.offset(6 as libc::c_int as isize);
            C[1 as libc::c_int as usize]
                .x = (*D.offset(6 as libc::c_int as isize)).x
                - ((*D.offset(7 as libc::c_int as isize)).x
                    - (*D.offset(6 as libc::c_int as isize)).x);
            C[1 as libc::c_int as usize]
                .y = (*D.offset(6 as libc::c_int as isize)).y
                - ((*D.offset(7 as libc::c_int as isize)).y
                    - (*D.offset(6 as libc::c_int as isize)).y);
            C[2 as libc::c_int as usize]
                .x = C[1 as libc::c_int as usize].x
                + ((*D.offset(8 as libc::c_int as isize)).x
                    - (*D.offset(7 as libc::c_int as isize)).x);
            C[2 as libc::c_int as usize]
                .y = C[1 as libc::c_int as usize].y
                + ((*D.offset(8 as libc::c_int as isize)).y
                    - (*D.offset(7 as libc::c_int as isize)).y);
            C[3 as libc::c_int as usize] = *D.offset(9 as libc::c_int as isize);
            gvrender_polyline(job, C.as_mut_ptr(), 4 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        100663296 => {
            D = gcalloc(
                (sides + 5 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    * 3 as libc::c_int as libc::c_double
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(3 as libc::c_int as isize)).x;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y);
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(4 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(4 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(6 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(6 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(8 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(8 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            gvrender_polygon(job, D, sides + 5 as libc::c_int, filled);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        117440512 => {
            D = gcalloc(
                (sides + 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*B.offset(1 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*B.offset(3 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*AF.offset(2 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                - ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            gvrender_polygon(job, D, sides + 1 as libc::c_int, filled);
            free(D as *mut libc::c_void);
        }
        134217728 => {
            D = gcalloc(
                (sides + 4 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(4 as libc::c_int as isize)).x;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(6 as libc::c_int as isize)).x;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 4 as libc::c_int, filled);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        150994944 => {
            D = gcalloc(
                (sides + 2 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(4 as libc::c_int as isize)).x;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 2 as libc::c_int, filled);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        167772160 => {
            D = gcalloc(
                (sides + 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y);
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(3 as libc::c_int as isize)).x;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 1 as libc::c_int, filled);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        184549376 => {
            D = gcalloc(
                (sides + 4 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(3 as libc::c_int as isize)).x;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(4 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*D.offset(5 as libc::c_int as isize)).x;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(5 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(6 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 4 as libc::c_int, filled);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*D.offset(4 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*D.offset(7 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        201326592 => {
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + 2 as libc::c_int as libc::c_double
                    * ((*B.offset(2 as libc::c_int as isize)).x
                        - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            free(D as *mut libc::c_void);
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    * 5 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            C[0 as libc::c_int as usize].x = (*D.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        218103808 => {
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x
                - 2 as libc::c_int as libc::c_double
                    * ((*B.offset(3 as libc::c_int as isize)).y
                        - (*B.offset(4 as libc::c_int as isize)).y);
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            free(D as *mut libc::c_void);
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    * 5 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y);
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*D.offset(3 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        234881024 => {
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 9 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            free(D as *mut libc::c_void);
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 9 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    * 5 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            free(D as *mut libc::c_void);
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    * 5 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            free(D as *mut libc::c_void);
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            C[0 as libc::c_int as usize].x = (*D.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 9 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        251658240 => {
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + 2 as libc::c_int as libc::c_double
                    * ((*B.offset(2 as libc::c_int as isize)).x
                        - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            free(D as *mut libc::c_void);
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    * 5 as libc::c_int as libc::c_double
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + 2 as libc::c_int as libc::c_double
                    * ((*B.offset(2 as libc::c_int as isize)).x
                        - (*B.offset(3 as libc::c_int as isize)).x);
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            C[0 as libc::c_int as usize].x = (*D.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*D.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        268435456 => {
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*B.offset(1 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*B.offset(3 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*AF.offset(2 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polygon(job, D, sides, filled);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize]
                .x = C[0 as libc::c_int as usize].x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize]
                .y = C[0 as libc::c_int as usize].y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize]
                .x = C[0 as libc::c_int as usize].x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize]
                .y = C[0 as libc::c_int as usize].y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize]
                .x = (*AF.offset(0 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].y = C[0 as libc::c_int as usize].y;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        285212672 => {
            D = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides, filled);
            free(D as *mut libc::c_void);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = C[0 as libc::c_int as usize].x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            C[2 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            C[2 as libc::c_int as usize].y = C[1 as libc::c_int as usize].y;
            C[3 as libc::c_int as usize].x = C[2 as libc::c_int as usize].x;
            C[3 as libc::c_int as usize].y = C[0 as libc::c_int as usize].y;
            C[4 as libc::c_int as usize] = C[0 as libc::c_int as usize];
            gvrender_polyline(job, C.as_mut_ptr(), 5 as libc::c_int);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    * 3 as libc::c_int as libc::c_double
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
        }
        301989888 => {
            D = gcalloc(
                (sides + 12 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(4 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(6 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(5 as libc::c_int as isize)).y;
            (*D.offset(8 as libc::c_int as isize))
                .x = (*D.offset(7 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(8 as libc::c_int as isize))
                .y = (*D.offset(7 as libc::c_int as isize)).y;
            (*D.offset(9 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x;
            (*D.offset(9 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y;
            (*D.offset(10 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(10 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(11 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x;
            (*D.offset(11 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(12 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x;
            (*D.offset(12 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(13 as libc::c_int as isize))
                .x = (*D.offset(10 as libc::c_int as isize)).x;
            (*D.offset(13 as libc::c_int as isize))
                .y = (*D.offset(12 as libc::c_int as isize)).y;
            (*D.offset(14 as libc::c_int as isize))
                .x = (*D.offset(6 as libc::c_int as isize)).x;
            (*D.offset(14 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(15 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(15 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 12 as libc::c_int, filled);
            C[0 as libc::c_int as usize].x = (*D.offset(14 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = C[0 as libc::c_int as usize].x;
            C[1 as libc::c_int as usize]
                .y = C[0 as libc::c_int as usize].y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*D.offset(14 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = C[0 as libc::c_int as usize].x;
            C[1 as libc::c_int as usize]
                .y = C[0 as libc::c_int as usize].y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        318767104 => {
            D = gcalloc(
                (sides + 4 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(3 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(4 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*D.offset(5 as libc::c_int as isize)).x;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(4 as libc::c_int as isize)).x;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 4 as libc::c_int, filled);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = C[0 as libc::c_int as usize].x;
            C[1 as libc::c_int as usize]
                .y = C[0 as libc::c_int as usize].y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = C[0 as libc::c_int as usize].x;
            C[1 as libc::c_int as usize]
                .y = C[0 as libc::c_int as usize].y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        335544320 => {
            D = gcalloc(
                (sides + 12 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(4 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(6 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(5 as libc::c_int as isize)).y;
            (*D.offset(8 as libc::c_int as isize))
                .x = (*D.offset(7 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(8 as libc::c_int as isize))
                .y = (*D.offset(7 as libc::c_int as isize)).y;
            (*D.offset(9 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x;
            (*D.offset(9 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y;
            (*D.offset(10 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(10 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(11 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x;
            (*D.offset(11 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(12 as libc::c_int as isize))
                .x = (*D.offset(8 as libc::c_int as isize)).x;
            (*D.offset(12 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            (*D.offset(13 as libc::c_int as isize))
                .x = (*D.offset(10 as libc::c_int as isize)).x;
            (*D.offset(13 as libc::c_int as isize))
                .y = (*D.offset(12 as libc::c_int as isize)).y;
            (*D.offset(14 as libc::c_int as isize))
                .x = (*D.offset(6 as libc::c_int as isize)).x;
            (*D.offset(14 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(15 as libc::c_int as isize))
                .x = (*D.offset(2 as libc::c_int as isize)).x;
            (*D.offset(15 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 12 as libc::c_int, filled);
            C[0 as libc::c_int as usize] = *D.offset(14 as libc::c_int as isize);
            C[1 as libc::c_int as usize].x = C[0 as libc::c_int as usize].x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        352321536 => {
            D = gcalloc(
                (sides + 4 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*D.offset(1 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*D.offset(0 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*D.offset(3 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 4 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*D.offset(3 as libc::c_int as isize)).y;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*D.offset(4 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 8 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*D.offset(2 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*D.offset(5 as libc::c_int as isize)).x;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*D.offset(1 as libc::c_int as isize)).y;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*D.offset(4 as libc::c_int as isize)).x;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*D.offset(0 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 4 as libc::c_int, filled);
            C[0 as libc::c_int as usize]
                .x = (*AF.offset(1 as libc::c_int as isize)).x
                + ((*AF.offset(0 as libc::c_int as isize)).x
                    - (*AF.offset(1 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            C[0 as libc::c_int as usize].y = (*D.offset(0 as libc::c_int as isize)).y;
            C[1 as libc::c_int as usize].x = C[0 as libc::c_int as usize].x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            C[0 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
            C[0 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            C[1 as libc::c_int as usize].x = (*AF.offset(0 as libc::c_int as isize)).x;
            C[1 as libc::c_int as usize]
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polyline(job, C.as_mut_ptr(), 2 as libc::c_int);
            free(D as *mut libc::c_void);
        }
        369098752 => {
            D = gcalloc(
                (sides + 5 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*B.offset(1 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*B.offset(3 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*AF.offset(2 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*AF.offset(3 as libc::c_int as isize)).y;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                - ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            (*D.offset(8 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y;
            (*D.offset(8 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polygon(job, D, sides + 5 as libc::c_int, filled);
            free(D as *mut libc::c_void);
        }
        385875968 => {
            D = gcalloc(
                (sides + 3 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*B.offset(1 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*B.offset(3 as libc::c_int as isize)).x;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*B.offset(3 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*AF.offset(2 as libc::c_int as isize)).x;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*AF.offset(3 as libc::c_int as isize)).y;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                - ((*AF.offset(0 as libc::c_int as isize)).y
                    - (*AF.offset(3 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            gvrender_polygon(job, D, sides + 3 as libc::c_int, filled);
            free(D as *mut libc::c_void);
        }
        402653184 => {
            D = gcalloc(
                (sides + 3 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*B.offset(2 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*AF.offset(1 as libc::c_int as isize)).y
                - ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*AF.offset(3 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            gvrender_polygon(job, D, sides + 3 as libc::c_int, filled);
            free(D as *mut libc::c_void);
        }
        419430400 => {
            D = gcalloc(
                (sides + 5 as libc::c_int) as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*D.offset(0 as libc::c_int as isize))
                .x = (*AF.offset(0 as libc::c_int as isize)).x;
            (*D.offset(0 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(1 as libc::c_int as isize))
                .y = (*AF.offset(0 as libc::c_int as isize)).y
                - ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(2 as libc::c_int as isize))
                .y = (*B.offset(2 as libc::c_int as isize)).y;
            (*D.offset(3 as libc::c_int as isize))
                .x = (*AF.offset(1 as libc::c_int as isize)).x;
            (*D.offset(3 as libc::c_int as isize))
                .y = (*AF.offset(1 as libc::c_int as isize)).y
                - ((*AF.offset(1 as libc::c_int as isize)).y
                    - (*AF.offset(2 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(4 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y;
            (*D.offset(5 as libc::c_int as isize))
                .y = (*AF.offset(2 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(5 as libc::c_int as isize))
                .x = (*B.offset(2 as libc::c_int as isize)).x
                + ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .y = (*AF.offset(3 as libc::c_int as isize)).y
                + ((*B.offset(3 as libc::c_int as isize)).y
                    - (*B.offset(4 as libc::c_int as isize)).y)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(6 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(7 as libc::c_int as isize))
                .x = (*B.offset(1 as libc::c_int as isize)).x
                - ((*B.offset(2 as libc::c_int as isize)).x
                    - (*B.offset(3 as libc::c_int as isize)).x)
                    / 2 as libc::c_int as libc::c_double;
            (*D.offset(7 as libc::c_int as isize))
                .y = (*AF.offset(3 as libc::c_int as isize)).y;
            (*D.offset(8 as libc::c_int as isize))
                .x = (*AF.offset(3 as libc::c_int as isize)).x;
            (*D.offset(8 as libc::c_int as isize))
                .y = (*AF.offset(3 as libc::c_int as isize)).y;
            gvrender_polygon(job, D, sides + 5 as libc::c_int, filled);
            free(D as *mut libc::c_void);
        }
        _ => {}
    }
    free(B as *mut libc::c_void);
}
unsafe extern "C" fn userSize(mut n: *mut node_t) -> libc::c_double {
    let mut w: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    w = late_double(n as *mut libc::c_void, N_width, 0.0f64, 0.01f64);
    h = late_double(n as *mut libc::c_void, N_height, 0.0f64, 0.02f64);
    return (if (if w > h { w } else { h }) * 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        ((if w > h { w } else { h }) * 72 as libc::c_int as libc::c_double + 0.5f64)
            as libc::c_int
    } else {
        ((if w > h { w } else { h }) * 72 as libc::c_int as libc::c_double - 0.5f64)
            as libc::c_int
    }) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn shapeOf(mut n: *mut node_t) -> shape_kind {
    let mut sh: *mut shape_desc = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .shape;
    let mut ifn: Option::<unsafe extern "C" fn(*mut node_t) -> ()> = None;
    if sh.is_null() {
        return SH_UNSET;
    }
    ifn = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns).initfn;
    if ifn == Some(poly_init as unsafe extern "C" fn(*mut node_t) -> ()) {
        return SH_POLY
    } else if ifn == Some(record_init as unsafe extern "C" fn(*mut node_t) -> ()) {
        return SH_RECORD
    } else if ifn == Some(point_init as unsafe extern "C" fn(*mut node_t) -> ()) {
        return SH_POINT
    } else if ifn == Some(epsf_init as unsafe extern "C" fn(*mut node_t) -> ()) {
        return SH_EPSF
    } else {
        return SH_UNSET
    };
}
#[no_mangle]
pub unsafe extern "C" fn isPolygon(mut n: *mut node_t) -> bool {
    return !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null()
        && (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns).initfn
            == Some(poly_init as unsafe extern "C" fn(*mut node_t) -> ());
}
unsafe extern "C" fn poly_init(mut n: *mut node_t) {
    let mut dimen: pointf = pointf { x: 0., y: 0. };
    let mut min_bb: pointf = pointf { x: 0., y: 0. };
    let mut bb: pointf = pointf { x: 0., y: 0. };
    let mut imagesize: point = point { x: 0, y: 0 };
    let mut vertices: *mut pointf = 0 as *mut pointf;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fxd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: libc::c_double = 0.;
    let mut alpha_0: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut gamma: libc::c_double = 0.;
    let mut orientation: libc::c_double = 0.;
    let mut distortion: libc::c_double = 0.;
    let mut skew: libc::c_double = 0.;
    let mut sectorangle: libc::c_double = 0.;
    let mut sidelength: libc::c_double = 0.;
    let mut skewdist: libc::c_double = 0.;
    let mut gdistortion: libc::c_double = 0.;
    let mut gskew: libc::c_double = 0.;
    let mut angle: libc::c_double = 0.;
    let mut sinx: libc::c_double = 0.;
    let mut cosx: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut scalex: libc::c_double = 0.;
    let mut scaley: libc::c_double = 0.;
    let mut width: libc::c_double = 0.;
    let mut height: libc::c_double = 0.;
    let mut marginx: libc::c_double = 0.;
    let mut marginy: libc::c_double = 0.;
    let mut spacex: libc::c_double = 0.;
    let mut regular: libc::c_int = 0;
    let mut peripheries: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut isBox_0: libc::c_int = 0;
    let mut outp: libc::c_int = 0;
    let mut poly: *mut polygon_t = zmalloc(
        ::std::mem::size_of::<polygon_t>() as libc::c_ulong,
    ) as *mut polygon_t;
    let mut isPlain: bool = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .shape)
        .polygon == &mut p_plain as *mut polygon_t;
    regular = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon)
        .regular;
    peripheries = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape)
        .polygon)
        .peripheries;
    sides = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon)
        .sides;
    orientation = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape)
        .polygon)
        .orientation;
    skew = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon)
        .skew;
    distortion = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape)
        .polygon)
        .distortion;
    regular
        |= if mapbool(
            agget(
                n as *mut libc::c_void,
                b"regular\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        ) as libc::c_int != 0
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else {
            0 as libc::c_int
        };
    if isPlain {
        height = 0 as libc::c_int as libc::c_double;
        width = height;
    } else if regular != 0 {
        let mut sz: libc::c_double = userSize(n);
        if sz > 0.0f64 {
            height = sz;
            width = height;
        } else {
            width = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width;
            height = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height;
            height = (if (if width < height { width } else { height })
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                ((if width < height { width } else { height })
                    * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                ((if width < height { width } else { height })
                    * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            }) as libc::c_double;
            width = height;
        }
    } else {
        width = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        }) as libc::c_double;
        height = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        }) as libc::c_double;
    }
    peripheries = late_int(
        n as *mut libc::c_void,
        N_peripheries,
        peripheries,
        0 as libc::c_int,
    );
    orientation += late_double(n as *mut libc::c_void, N_orientation, 0.0f64, -360.0f64);
    if sides == 0 as libc::c_int {
        skew = late_double(n as *mut libc::c_void, N_skew, 0.0f64, -100.0f64);
        sides = late_int(
            n as *mut libc::c_void,
            N_sides,
            4 as libc::c_int,
            0 as libc::c_int,
        );
        distortion = late_double(
            n as *mut libc::c_void,
            N_distortion,
            0.0f64,
            -100.0f64,
        );
    }
    dimen = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).dimen;
    if dimen.x > 0 as libc::c_int as libc::c_double
        || dimen.y > 0 as libc::c_int as libc::c_double
    {
        if !isPlain {
            p = agget(
                n as *mut libc::c_void,
                b"margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !p.is_null() {
                marginy = 0 as libc::c_int as libc::c_double;
                marginx = marginy;
                i = sscanf(
                    p,
                    b"%lf,%lf\0" as *const u8 as *const libc::c_char,
                    &mut marginx as *mut libc::c_double,
                    &mut marginy as *mut libc::c_double,
                );
                marginx = fmax(marginx, 0 as libc::c_int as libc::c_double);
                marginy = fmax(marginy, 0 as libc::c_int as libc::c_double);
                if i > 0 as libc::c_int {
                    dimen.x
                        += (2 as libc::c_int
                            * (if marginx * 72 as libc::c_int as libc::c_double
                                >= 0 as libc::c_int as libc::c_double
                            {
                                (marginx * 72 as libc::c_int as libc::c_double + 0.5f64)
                                    as libc::c_int
                            } else {
                                (marginx * 72 as libc::c_int as libc::c_double - 0.5f64)
                                    as libc::c_int
                            })) as libc::c_double;
                    if i > 1 as libc::c_int {
                        dimen.y
                            += (2 as libc::c_int
                                * (if marginy * 72 as libc::c_int as libc::c_double
                                    >= 0 as libc::c_int as libc::c_double
                                {
                                    (marginy * 72 as libc::c_int as libc::c_double + 0.5f64)
                                        as libc::c_int
                                } else {
                                    (marginy * 72 as libc::c_int as libc::c_double - 0.5f64)
                                        as libc::c_int
                                })) as libc::c_double;
                    } else {
                        dimen.y
                            += (2 as libc::c_int
                                * (if marginx * 72 as libc::c_int as libc::c_double
                                    >= 0 as libc::c_int as libc::c_double
                                {
                                    (marginx * 72 as libc::c_int as libc::c_double + 0.5f64)
                                        as libc::c_int
                                } else {
                                    (marginx * 72 as libc::c_int as libc::c_double - 0.5f64)
                                        as libc::c_int
                                })) as libc::c_double;
                    }
                } else {
                    dimen.x += (4 as libc::c_int * 4 as libc::c_int) as libc::c_double;
                    dimen.y += (2 as libc::c_int * 4 as libc::c_int) as libc::c_double;
                }
            } else {
                dimen.x += (4 as libc::c_int * 4 as libc::c_int) as libc::c_double;
                dimen.y += (2 as libc::c_int * 4 as libc::c_int) as libc::c_double;
            }
        }
    }
    spacex = dimen.x
        - (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).dimen.x;
    temp = (*(*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .drawing)
        .quantum;
    if temp > 0.0f64 {
        temp = (if temp * 72 as libc::c_int as libc::c_double
            >= 0 as libc::c_int as libc::c_double
        {
            (temp * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            (temp * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        }) as libc::c_double;
        dimen.x = quant(dimen.x, temp);
        dimen.y = quant(dimen.y, temp);
    }
    imagesize.y = 0 as libc::c_int;
    imagesize.x = imagesize.y;
    if (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).usershape {
        if strcmp(
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
            b"custom\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            sfile = agget(
                n as *mut libc::c_void,
                b"shapefile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            imagesize = gvusershape_size(agraphof(n as *mut libc::c_void), sfile);
            if imagesize.x == -(1 as libc::c_int) && imagesize.y == -(1 as libc::c_int) {
                agerr(
                    AGWARN,
                    b"No or improper shapefile=\"%s\" for node \"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    if !sfile.is_null() {
                        sfile as *const libc::c_char
                    } else {
                        b"<nil>\0" as *const u8 as *const libc::c_char
                    },
                    agnameof(n as *mut libc::c_void),
                );
                imagesize.y = 0 as libc::c_int;
                imagesize.x = imagesize.y;
            } else {
                (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .has_images = 1 as libc::c_int != 0;
                imagesize.x += 2 as libc::c_int;
                imagesize.y += 2 as libc::c_int;
            }
        }
    } else {
        sfile = agget(
            n as *mut libc::c_void,
            b"image\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !sfile.is_null() && *sfile as libc::c_int != '\0' as i32 {
            imagesize = gvusershape_size(agraphof(n as *mut libc::c_void), sfile);
            if imagesize.x == -(1 as libc::c_int) && imagesize.y == -(1 as libc::c_int) {
                agerr(
                    AGWARN,
                    b"No or improper image=\"%s\" for node \"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    if !sfile.is_null() {
                        sfile as *const libc::c_char
                    } else {
                        b"<nil>\0" as *const u8 as *const libc::c_char
                    },
                    agnameof(n as *mut libc::c_void),
                );
                imagesize.y = 0 as libc::c_int;
                imagesize.x = imagesize.y;
            } else {
                (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .has_images = 1 as libc::c_int != 0;
                imagesize.x += 2 as libc::c_int;
                imagesize.y += 2 as libc::c_int;
            }
        }
    }
    bb
        .x = if dimen.x > imagesize.x as libc::c_double {
        dimen.x
    } else {
        imagesize.x as libc::c_double
    };
    bb
        .y = if dimen.y > imagesize.y as libc::c_double {
        dimen.y
    } else {
        imagesize.y as libc::c_double
    };
    if sides <= 2 as libc::c_int && (distortion != 0.0f64 || skew != 0.0f64) {
        sides = 120 as libc::c_int;
    }
    p = agget(
        n as *mut libc::c_void,
        b"labelloc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null()
        && (*p.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
            || *p.offset(0 as libc::c_int as isize) as libc::c_int == 'b' as i32)
    {
        (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
            .valign = *p.offset(0 as libc::c_int as isize);
    } else {
        (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
            .valign = 'c' as i32 as libc::c_char;
    }
    isBox_0 = (sides == 4 as libc::c_int
        && (if orientation >= 0 as libc::c_int as libc::c_double {
            (orientation + 0.5f64) as libc::c_int
        } else {
            (orientation - 0.5f64) as libc::c_int
        }) % 90 as libc::c_int == 0 as libc::c_int && distortion == 0.0f64
        && skew == 0.0f64) as libc::c_int;
    if !(isBox_0 != 0) {
        if !((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon)
            .vertices)
            .is_null()
        {
            let mut pd: *mut poly_desc_t = (*(*(*((*(n as *mut Agobj_t)).data
                as *mut Agnodeinfo_t))
                .shape)
                .polygon)
                .vertices as *mut poly_desc_t;
            bb = ((*pd).size_gen).expect("non-null function pointer")(bb);
        } else {
            temp = bb.y * 1.41421356237309504880f64;
            if height > temp
                && (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).valign
                    as libc::c_int == 'c' as i32
            {
                bb.x *= sqrt(1.0f64 / (1.0f64 - bb.y / height * (bb.y / height)));
            } else {
                bb.x *= 1.41421356237309504880f64;
                bb.y = temp;
            }
            if sides > 2 as libc::c_int {
                temp = cos(3.14159265358979323846f64 / sides as libc::c_double);
                bb.x /= temp;
                bb.y /= temp;
            }
        }
    }
    min_bb = bb;
    fxd = late_string(
        n as *mut libc::c_void,
        N_fixed,
        b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *fxd as libc::c_int == 's' as i32
        && strcmp(fxd, b"shape\0" as *const u8 as *const libc::c_char) == 0
    {
        bb.x = width;
        bb.y = height;
        (*poly).option |= (1 as libc::c_int) << 11 as libc::c_int;
    } else if mapbool(fxd) {
        if width < (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).dimen.x
            || height
                < (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).dimen.y
        {
            agerr(
                AGWARN,
                b"node '%s', graph '%s' size too small for label\n\0" as *const u8
                    as *const libc::c_char,
                agnameof(n as *mut libc::c_void),
                agnameof(agraphof(n as *mut libc::c_void) as *mut libc::c_void),
            );
        }
        bb.x = width;
        bb.y = height;
    } else {
        width = if width > bb.x { width } else { bb.x };
        bb.x = width;
        height = if height > bb.y { height } else { bb.y };
        bb.y = height;
    }
    if regular != 0 {
        bb.y = if bb.x > bb.y { bb.x } else { bb.y };
        bb.x = bb.y;
        height = bb.x;
        width = height;
    }
    if !mapbool(
        late_string(
            n as *mut libc::c_void,
            N_nojustify,
            b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) {
        if isBox_0 != 0 {
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
                .space
                .x = (if dimen.x > bb.x { dimen.x } else { bb.x }) - spacex;
        } else if dimen.y < bb.y {
            temp = bb.x * sqrt(1.0f64 - dimen.y * dimen.y / (bb.y * bb.y));
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
                .space
                .x = (if dimen.x > temp { dimen.x } else { temp }) - spacex;
        } else {
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
                .space
                .x = dimen.x - spacex;
        }
    } else {
        (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
            .space
            .x = dimen.x - spacex;
    }
    if (*poly).option & (1 as libc::c_int) << 11 as libc::c_int == 0 as libc::c_int {
        temp = bb.y - min_bb.y;
        if dimen.y < imagesize.y as libc::c_double {
            temp += imagesize.y as libc::c_double - dimen.y;
        }
        (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
            .space
            .y = dimen.y + temp;
    }
    outp = peripheries;
    if peripheries < 1 as libc::c_int {
        outp = 1 as libc::c_int;
    }
    if sides < 3 as libc::c_int {
        sides = 2 as libc::c_int;
        vertices = gcalloc(
            (outp * sides) as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        let mut P: pointf = pointf { x: 0., y: 0. };
        P.x = bb.x / 2.0f64;
        P.y = bb.y / 2.0f64;
        (*vertices.offset(0 as libc::c_int as isize)).x = -P.x;
        (*vertices.offset(0 as libc::c_int as isize)).y = -P.y;
        *vertices.offset(1 as libc::c_int as isize) = P;
        if peripheries > 1 as libc::c_int {
            j = 1 as libc::c_int;
            i = 2 as libc::c_int;
            while j < peripheries {
                P.x += 4 as libc::c_int as libc::c_double;
                P.y += 4 as libc::c_int as libc::c_double;
                (*vertices.offset(i as isize)).x = -P.x;
                (*vertices.offset(i as isize)).y = -P.y;
                i += 1;
                (*vertices.offset(i as isize)).x = P.x;
                (*vertices.offset(i as isize)).y = P.y;
                i += 1;
                j += 1;
            }
            bb.x = 2.0f64 * P.x;
            bb.y = 2.0f64 * P.y;
        }
    } else {
        vertices = gcalloc(
            (outp * sides) as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        if !((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon)
            .vertices)
            .is_null()
        {
            let mut pd_0: *mut poly_desc_t = (*(*(*((*(n as *mut Agobj_t)).data
                as *mut Agnodeinfo_t))
                .shape)
                .polygon)
                .vertices as *mut poly_desc_t;
            ((*pd_0).vertex_gen).expect("non-null function pointer")(vertices, &mut bb);
            xmax = bb.x / 2 as libc::c_int as libc::c_double;
            ymax = bb.y / 2 as libc::c_int as libc::c_double;
        } else {
            sectorangle = 2.0f64 * 3.14159265358979323846f64 / sides as libc::c_double;
            sidelength = sin(sectorangle / 2.0f64);
            skewdist = hypot(fabs(distortion) + fabs(skew), 1.0f64);
            gdistortion = distortion * 1.41421356237309504880f64
                / cos(sectorangle / 2.0f64);
            gskew = skew / 2.0f64;
            angle = (sectorangle - 3.14159265358979323846f64) / 2.0f64;
            sincos(angle, &mut sinx, &mut cosx);
            let mut R: pointf = pointf { x: 0., y: 0. };
            R.x = 0.5f64 * cosx;
            R.y = 0.5f64 * sinx;
            ymax = 0.0f64;
            xmax = ymax;
            angle += (3.14159265358979323846f64 - sectorangle) / 2.0f64;
            i = 0 as libc::c_int;
            while i < sides {
                angle += sectorangle;
                sincos(angle, &mut sinx, &mut cosx);
                R.x += sidelength * cosx;
                R.y += sidelength * sinx;
                let mut P_0: pointf = pointf { x: 0., y: 0. };
                P_0.x = R.x * (skewdist + R.y * gdistortion) + R.y * gskew;
                P_0.y = R.y;
                alpha_0 = orientation / 180.0f64 * 3.14159265358979323846f64
                    + atan2(P_0.y, P_0.x);
                sincos(alpha_0, &mut sinx, &mut cosx);
                P_0.y = hypot(P_0.x, P_0.y);
                P_0.x = P_0.y;
                P_0.x *= cosx;
                P_0.y *= sinx;
                P_0.x *= bb.x;
                P_0.y *= bb.y;
                xmax = if fabs(P_0.x) > xmax { fabs(P_0.x) } else { xmax };
                ymax = if fabs(P_0.y) > ymax { fabs(P_0.y) } else { ymax };
                *vertices.offset(i as isize) = P_0;
                if isBox_0 != 0 {
                    (*vertices.offset(1 as libc::c_int as isize)).x = -P_0.x;
                    (*vertices.offset(1 as libc::c_int as isize)).y = P_0.y;
                    (*vertices.offset(2 as libc::c_int as isize)).x = -P_0.x;
                    (*vertices.offset(2 as libc::c_int as isize)).y = -P_0.y;
                    (*vertices.offset(3 as libc::c_int as isize)).x = P_0.x;
                    (*vertices.offset(3 as libc::c_int as isize)).y = -P_0.y;
                    break;
                } else {
                    i += 1;
                }
            }
        }
        xmax *= 2.0f64;
        ymax *= 2.0f64;
        bb.x = if width > xmax { width } else { xmax };
        bb.y = if height > ymax { height } else { ymax };
        scalex = bb.x / xmax;
        scaley = bb.y / ymax;
        i = 0 as libc::c_int;
        while i < sides {
            let mut P_1: pointf = *vertices.offset(i as isize);
            P_1.x *= scalex;
            P_1.y *= scaley;
            *vertices.offset(i as isize) = P_1;
            i += 1;
        }
        if peripheries > 1 as libc::c_int {
            let mut Q: pointf = *vertices.offset((sides - 1 as libc::c_int) as isize);
            let mut R_0: pointf = *vertices.offset(0 as libc::c_int as isize);
            beta = atan2(R_0.y - Q.y, R_0.x - Q.x);
            i = 0 as libc::c_int;
            while i < sides {
                Q = R_0;
                R_0 = *vertices.offset(((i + 1 as libc::c_int) % sides) as isize);
                alpha_0 = beta;
                beta = atan2(R_0.y - Q.y, R_0.x - Q.x);
                gamma = (alpha_0 + 3.14159265358979323846f64 - beta) / 2.0f64;
                temp = 4 as libc::c_int as libc::c_double / sin(gamma);
                sincos(alpha_0 - gamma, &mut sinx, &mut cosx);
                sinx *= temp;
                cosx *= temp;
                j = 1 as libc::c_int;
                while j < peripheries {
                    Q.x += cosx;
                    Q.y += sinx;
                    *vertices.offset((i + j * sides) as isize) = Q;
                    j += 1;
                }
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < sides {
                let mut P_2: pointf = *vertices
                    .offset((i + (peripheries - 1 as libc::c_int) * sides) as isize);
                bb
                    .x = if 2.0f64 * fabs(P_2.x) > bb.x {
                    2.0f64 * fabs(P_2.x)
                } else {
                    bb.x
                };
                bb
                    .y = if 2.0f64 * fabs(P_2.y) > bb.y {
                    2.0f64 * fabs(P_2.y)
                } else {
                    bb.y
                };
                i += 1;
            }
        }
    }
    (*poly).regular = regular;
    (*poly).peripheries = peripheries;
    (*poly).sides = sides;
    (*poly).orientation = orientation;
    (*poly).skew = skew;
    (*poly).distortion = distortion;
    let ref mut fresh21 = (*poly).vertices;
    *fresh21 = vertices;
    if (*poly).option & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .width = (if dimen.x > bb.x { dimen.x } else { bb.x })
            / 72 as libc::c_int as libc::c_double;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .height = (if dimen.y > bb.y { dimen.y } else { bb.y })
            / 72 as libc::c_int as libc::c_double;
    } else {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .width = bb.x / 72 as libc::c_int as libc::c_double;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .height = bb.y / 72 as libc::c_int as libc::c_double;
    }
    let ref mut fresh22 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .shape_info;
    *fresh22 = poly as *mut libc::c_void;
}
unsafe extern "C" fn poly_free(mut n: *mut node_t) {
    let mut p: *mut polygon_t = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .shape_info as *mut polygon_t;
    if !p.is_null() {
        free((*p).vertices as *mut libc::c_void);
        free(p as *mut libc::c_void);
    }
}
unsafe extern "C" fn poly_inside(
    mut inside_context: *mut inside_t,
    mut p: pointf,
) -> bool {
    static mut lastn: *mut node_t = 0 as *const node_t as *mut node_t;
    static mut poly: *mut polygon_t = 0 as *const polygon_t as *mut polygon_t;
    static mut last: libc::c_int = 0;
    static mut outp: libc::c_int = 0;
    static mut sides: libc::c_int = 0;
    static mut O: pointf = pointf { x: 0., y: 0. };
    static mut vertex: *mut pointf = 0 as *const pointf as *mut pointf;
    static mut xsize: libc::c_double = 0.;
    static mut ysize: libc::c_double = 0.;
    static mut scalex: libc::c_double = 0.;
    static mut scaley: libc::c_double = 0.;
    static mut box_URx: libc::c_double = 0.;
    static mut box_URy: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut P: pointf = pointf { x: 0., y: 0. };
    let mut Q: pointf = pointf { x: 0., y: 0. };
    let mut R: pointf = pointf { x: 0., y: 0. };
    let mut bp: *mut boxf = 0 as *mut boxf;
    let mut n: *mut node_t = 0 as *mut node_t;
    if inside_context.is_null() {
        lastn = 0 as *mut node_t;
        return 0 as libc::c_int != 0;
    }
    bp = (*inside_context).s.bp;
    n = (*inside_context).s.n;
    P = ccwrotatepf(
        p,
        90 as libc::c_int
            * ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int),
    );
    if !bp.is_null() {
        let mut bbox: boxf = *bp;
        return bbox.LL.x <= P.x && P.x <= bbox.UR.x
            && (bbox.LL.y <= P.y && P.y <= bbox.UR.y);
    }
    if n != lastn {
        let mut n_width: libc::c_double = 0.;
        let mut n_height: libc::c_double = 0.;
        poly = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
            as *mut polygon_t;
        vertex = (*poly).vertices;
        sides = (*poly).sides;
        if (*poly).option & (1 as libc::c_int) << 11 as libc::c_int != 0 {
            let mut bb: boxf = polyBB(poly);
            n_width = bb.UR.x - bb.LL.x;
            n_height = bb.UR.y - bb.LL.y;
            if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                ysize = n_width;
                xsize = n_height;
            } else {
                xsize = n_width;
                ysize = n_height;
            }
        } else {
            if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                ysize = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
                xsize = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht;
            } else {
                xsize = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
                ysize = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht;
            }
            n_width = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                    * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                    * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            }) as libc::c_double;
            n_height = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                    * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                    * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            }) as libc::c_double;
        }
        if xsize == 0.0f64 {
            xsize = 1.0f64;
        }
        if ysize == 0.0f64 {
            ysize = 1.0f64;
        }
        scalex = n_width / xsize;
        scaley = n_height / ysize;
        box_URx = n_width / 2.0f64;
        box_URy = n_height / 2.0f64;
        outp = ((*poly).peripheries - 1 as libc::c_int) * sides;
        if outp < 0 as libc::c_int {
            outp = 0 as libc::c_int;
        }
        lastn = n;
    }
    P.x *= scalex;
    P.y *= scaley;
    if fabs(P.x) > box_URx || fabs(P.y) > box_URy {
        return 0 as libc::c_int != 0;
    }
    if sides <= 2 as libc::c_int {
        return hypot(P.x / box_URx, P.y / box_URy) < 1.0f64;
    }
    i = last % sides;
    i1 = (i + 1 as libc::c_int) % sides;
    Q = *vertex.offset((i + outp) as isize);
    R = *vertex.offset((i1 + outp) as isize);
    if same_side(P, O, Q, R) == 0 {
        return 0 as libc::c_int != 0;
    }
    s = same_side(P, Q, R, O);
    if s != 0 && same_side(P, R, O, Q) != 0 {
        return 1 as libc::c_int != 0;
    }
    j = 1 as libc::c_int;
    while j < sides {
        if s != 0 {
            i = i1;
            i1 = (i + 1 as libc::c_int) % sides;
        } else {
            i1 = i;
            i = (i + sides - 1 as libc::c_int) % sides;
        }
        if same_side(
            P,
            O,
            *vertex.offset((i + outp) as isize),
            *vertex.offset((i1 + outp) as isize),
        ) == 0
        {
            last = i;
            return 0 as libc::c_int != 0;
        }
        j += 1;
    }
    last = i;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn poly_path(
    mut n: *mut node_t,
    mut p: *mut port,
    mut side: libc::c_int,
    mut rv: *mut boxf,
    mut kptr: *mut libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn invflip_side(
    mut side: libc::c_int,
    mut rankdir: libc::c_int,
) -> libc::c_int {
    match rankdir {
        0 => {}
        2 => {
            match side {
                4 => {
                    side = (1 as libc::c_int) << 0 as libc::c_int;
                }
                1 => {
                    side = (1 as libc::c_int) << 2 as libc::c_int;
                }
                _ => {}
            }
        }
        1 => {
            match side {
                4 => {
                    side = (1 as libc::c_int) << 1 as libc::c_int;
                }
                1 => {
                    side = (1 as libc::c_int) << 3 as libc::c_int;
                }
                8 => {
                    side = (1 as libc::c_int) << 2 as libc::c_int;
                }
                2 => {
                    side = (1 as libc::c_int) << 0 as libc::c_int;
                }
                _ => {}
            }
        }
        3 => {
            match side {
                4 => {
                    side = (1 as libc::c_int) << 1 as libc::c_int;
                }
                1 => {
                    side = (1 as libc::c_int) << 3 as libc::c_int;
                }
                8 => {
                    side = (1 as libc::c_int) << 0 as libc::c_int;
                }
                2 => {
                    side = (1 as libc::c_int) << 2 as libc::c_int;
                }
                _ => {}
            }
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"shapes.c\0" as *const u8 as *const libc::c_char,
                2438 as libc::c_int,
            );
            abort();
        }
    }
    return side;
}
unsafe extern "C" fn invflip_angle(
    mut angle: libc::c_double,
    mut rankdir: libc::c_int,
) -> libc::c_double {
    match rankdir {
        2 => {
            angle *= -(1 as libc::c_int) as libc::c_double;
        }
        1 => {
            angle -= 3.14159265358979323846f64 * 0.5f64;
        }
        3 => {
            if angle == 3.14159265358979323846f64 {
                angle = -0.5f64 * 3.14159265358979323846f64;
            } else if angle == 3.14159265358979323846f64 * 0.75f64 {
                angle = -0.25f64 * 3.14159265358979323846f64;
            } else if angle == 3.14159265358979323846f64 * 0.5f64 {
                angle = 0 as libc::c_int as libc::c_double;
            } else if angle == 0 as libc::c_int as libc::c_double {
                angle = 3.14159265358979323846f64 * 0.5f64;
            } else if angle == 3.14159265358979323846f64 * -0.25f64 {
                angle = 3.14159265358979323846f64 * 0.75f64;
            } else if angle == 3.14159265358979323846f64 * -0.5f64 {
                angle = 3.14159265358979323846f64;
            }
        }
        0 | _ => {}
    }
    return angle;
}
unsafe extern "C" fn compassPoint(
    mut ictxt: *mut inside_t,
    mut y: libc::c_double,
    mut x: libc::c_double,
) -> pointf {
    let mut curve: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut n: *mut node_t = (*ictxt).s.n;
    let mut g: *mut graph_t = agraphof(n as *mut libc::c_void);
    let mut rd: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rankdir & 0x3 as libc::c_int;
    let mut p: pointf = pointf { x: 0., y: 0. };
    p.x = x;
    p.y = y;
    if rd != 0 {
        p = cwrotatepf(p, 90 as libc::c_int * rd);
    }
    curve[0 as libc::c_int as usize].y = 0 as libc::c_int as libc::c_double;
    curve[0 as libc::c_int as usize].x = curve[0 as libc::c_int as usize].y;
    curve[1 as libc::c_int as usize] = curve[0 as libc::c_int as usize];
    curve[2 as libc::c_int as usize] = p;
    curve[3 as libc::c_int as usize] = curve[2 as libc::c_int as usize];
    bezier_clip(
        ictxt,
        (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns).insidefn,
        curve.as_mut_ptr(),
        1 as libc::c_int != 0,
    );
    if rd != 0 {
        curve[0 as libc::c_int
            as usize] = ccwrotatepf(
            curve[0 as libc::c_int as usize],
            90 as libc::c_int * rd,
        );
    }
    return curve[0 as libc::c_int as usize];
}
unsafe extern "C" fn compassPort(
    mut n: *mut node_t,
    mut bp: *mut boxf,
    mut pp: *mut port,
    mut compass: *mut libc::c_char,
    mut sides: libc::c_int,
    mut ictxt: *mut inside_t,
) -> libc::c_int {
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut ctr: pointf = pointf { x: 0., y: 0. };
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut theta: libc::c_double = 0.0f64;
    let mut constrain: bool = 0 as libc::c_int != 0;
    let mut dyna: bool = 0 as libc::c_int != 0;
    let mut side: libc::c_int = 0 as libc::c_int;
    let mut clip: bool = 1 as libc::c_int != 0;
    let mut defined: bool = false;
    let mut maxv: libc::c_double = 0.;
    if !bp.is_null() {
        b = *bp;
        p = pointfof(
            (b.LL.x + b.UR.x) / 2 as libc::c_int as libc::c_double,
            (b.LL.y + b.UR.y) / 2 as libc::c_int as libc::c_double,
        );
        defined = 1 as libc::c_int != 0;
    } else {
        p.y = 0.0f64;
        p.x = p.y;
        if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
        {
            b.UR.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
            b.LL.x = -b.UR.x;
            b.UR.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
            b.LL.y = -b.UR.y;
        } else {
            b.UR.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
            b.LL.y = -b.UR.y;
            b.UR.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
            b.LL.x = -b.UR.x;
        }
        defined = 0 as libc::c_int != 0;
    }
    maxv = if b.UR.x > b.UR.y { b.UR.x } else { b.UR.y };
    maxv *= 4.0f64;
    ctr = p;
    if !compass.is_null() && *compass as libc::c_int != 0 {
        let fresh23 = compass;
        compass = compass.offset(1);
        match *fresh23 as libc::c_int {
            101 => {
                if *compass != 0 {
                    rv = 1 as libc::c_int;
                } else {
                    if !ictxt.is_null() {
                        p = compassPoint(ictxt, ctr.y, maxv);
                    } else {
                        p.x = b.UR.x;
                    }
                    theta = 0.0f64;
                    constrain = 1 as libc::c_int != 0;
                    defined = 1 as libc::c_int != 0;
                    clip = 0 as libc::c_int != 0;
                    side = sides & (1 as libc::c_int) << 1 as libc::c_int;
                }
            }
            115 => {
                p.y = b.LL.y;
                constrain = 1 as libc::c_int != 0;
                clip = 0 as libc::c_int != 0;
                match *compass as libc::c_int {
                    0 => {
                        theta = -3.14159265358979323846f64 * 0.5f64;
                        defined = 1 as libc::c_int != 0;
                        if !ictxt.is_null() {
                            p = compassPoint(ictxt, -maxv, ctr.x);
                        } else {
                            p.x = ctr.x;
                        }
                        side = sides & (1 as libc::c_int) << 0 as libc::c_int;
                    }
                    101 => {
                        theta = -3.14159265358979323846f64 * 0.25f64;
                        defined = 1 as libc::c_int != 0;
                        if !ictxt.is_null() {
                            p = compassPoint(ictxt, -maxv, maxv);
                        } else {
                            p.x = b.UR.x;
                        }
                        side = sides
                            & ((1 as libc::c_int) << 0 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int);
                    }
                    119 => {
                        theta = -3.14159265358979323846f64 * 0.75f64;
                        defined = 1 as libc::c_int != 0;
                        if !ictxt.is_null() {
                            p = compassPoint(ictxt, -maxv, -maxv);
                        } else {
                            p.x = b.LL.x;
                        }
                        side = sides
                            & ((1 as libc::c_int) << 0 as libc::c_int
                                | (1 as libc::c_int) << 3 as libc::c_int);
                    }
                    _ => {
                        p.y = ctr.y;
                        constrain = 0 as libc::c_int != 0;
                        clip = 1 as libc::c_int != 0;
                        rv = 1 as libc::c_int;
                    }
                }
            }
            119 => {
                if *compass != 0 {
                    rv = 1 as libc::c_int;
                } else {
                    if !ictxt.is_null() {
                        p = compassPoint(ictxt, ctr.y, -maxv);
                    } else {
                        p.x = b.LL.x;
                    }
                    theta = 3.14159265358979323846f64;
                    constrain = 1 as libc::c_int != 0;
                    defined = 1 as libc::c_int != 0;
                    clip = 0 as libc::c_int != 0;
                    side = sides & (1 as libc::c_int) << 3 as libc::c_int;
                }
            }
            110 => {
                p.y = b.UR.y;
                constrain = 1 as libc::c_int != 0;
                clip = 0 as libc::c_int != 0;
                match *compass as libc::c_int {
                    0 => {
                        defined = 1 as libc::c_int != 0;
                        theta = 3.14159265358979323846f64 * 0.5f64;
                        if !ictxt.is_null() {
                            p = compassPoint(ictxt, maxv, ctr.x);
                        } else {
                            p.x = ctr.x;
                        }
                        side = sides & (1 as libc::c_int) << 2 as libc::c_int;
                    }
                    101 => {
                        defined = 1 as libc::c_int != 0;
                        theta = 3.14159265358979323846f64 * 0.25f64;
                        if !ictxt.is_null() {
                            p = compassPoint(ictxt, maxv, maxv);
                        } else {
                            p.x = b.UR.x;
                        }
                        side = sides
                            & ((1 as libc::c_int) << 2 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int);
                    }
                    119 => {
                        defined = 1 as libc::c_int != 0;
                        theta = 3.14159265358979323846f64 * 0.75f64;
                        if !ictxt.is_null() {
                            p = compassPoint(ictxt, maxv, -maxv);
                        } else {
                            p.x = b.LL.x;
                        }
                        side = sides
                            & ((1 as libc::c_int) << 2 as libc::c_int
                                | (1 as libc::c_int) << 3 as libc::c_int);
                    }
                    _ => {
                        p.y = ctr.y;
                        constrain = 0 as libc::c_int != 0;
                        clip = 1 as libc::c_int != 0;
                        rv = 1 as libc::c_int;
                    }
                }
            }
            95 => {
                dyna = 1 as libc::c_int != 0;
                side = sides;
            }
            99 => {}
            _ => {
                rv = 1 as libc::c_int;
            }
        }
    }
    p = cwrotatepf(
        p,
        90 as libc::c_int
            * ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int),
    );
    if dyna {
        (*pp).side = side as libc::c_uchar;
    } else {
        (*pp)
            .side = invflip_side(
            side,
            (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int,
        ) as libc::c_uchar;
    }
    let ref mut fresh24 = (*pp).bp;
    *fresh24 = bp;
    (*pp)
        .p
        .x = (if p.x >= 0 as libc::c_int as libc::c_double {
        (p.x + 0.5f64) as libc::c_int
    } else {
        (p.x - 0.5f64) as libc::c_int
    }) as libc::c_double;
    (*pp)
        .p
        .y = (if p.y >= 0 as libc::c_int as libc::c_double {
        (p.y + 0.5f64) as libc::c_int
    } else {
        (p.y - 0.5f64) as libc::c_int
    }) as libc::c_double;
    (*pp)
        .theta = invflip_angle(
        theta,
        (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int,
    );
    if p.x == 0 as libc::c_int as libc::c_double
        && p.y == 0 as libc::c_int as libc::c_double
    {
        (*pp).order = (256 as libc::c_int / 2 as libc::c_int) as libc::c_uchar;
    } else {
        let mut angle: libc::c_double = atan2(p.y, p.x)
            + 1.5f64 * 3.14159265358979323846f64;
        if angle >= 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64 {
            angle -= 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
        }
        (*pp)
            .order = (256 as libc::c_int as libc::c_double * angle
            / (2 as libc::c_int as libc::c_double * 3.14159265358979323846f64))
            as libc::c_int as libc::c_uchar;
    }
    (*pp).constrained = constrain;
    (*pp).defined = defined;
    (*pp).clip = clip;
    (*pp).dyna = dyna;
    return rv;
}
unsafe extern "C" fn poly_port(
    mut n: *mut node_t,
    mut portname: *mut libc::c_char,
    mut compass: *mut libc::c_char,
) -> port {
    let mut rv: port = port {
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
    let mut bp: *mut boxf = 0 as *mut boxf;
    let mut sides: libc::c_int = 0;
    if *portname.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return Center;
    }
    if compass.is_null() {
        compass = b"_\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sides = (1 as libc::c_int) << 0 as libc::c_int
        | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int;
    if (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).html as libc::c_int
        != 0
        && {
            bp = html_port(n, portname, &mut sides);
            !bp.is_null()
        }
    {
        if compassPort(n, bp, &mut rv, compass, sides, 0 as *mut inside_t) != 0 {
            agerr(
                AGWARN,
                b"node %s, port %s, unrecognized compass point '%s' - ignored\n\0"
                    as *const u8 as *const libc::c_char,
                agnameof(n as *mut libc::c_void),
                portname,
                compass,
            );
        }
    } else {
        let mut ictxtp: *mut inside_t = 0 as *mut inside_t;
        let mut ictxt: inside_t = inside_t {
            a: C2RustUnnamed_9 {
                p: 0 as *mut pointf,
                r: 0 as *mut libc::c_double,
            },
        };
        if (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).polygon
            == &mut p_box as *mut polygon_t
        {
            ictxtp = 0 as *mut inside_t;
        } else {
            ictxt.s.n = n;
            ictxt.s.bp = 0 as *mut boxf;
            ictxtp = &mut ictxt;
        }
        if compassPort(n, 0 as *mut boxf, &mut rv, portname, sides, ictxtp) != 0 {
            unrecognized(n, portname);
        }
    }
    rv.name = 0 as *mut libc::c_char;
    return rv;
}
unsafe extern "C" fn poly_gencode(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut poly: *mut polygon_t = 0 as *mut polygon_t;
    let mut xsize: libc::c_double = 0.;
    let mut ysize: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut peripheries: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut P: pointf = pointf { x: 0., y: 0. };
    let mut vertices: *mut pointf = 0 as *mut pointf;
    static mut AF: *mut pointf = 0 as *const pointf as *mut pointf;
    static mut A_size: libc::c_int = 0;
    let mut filled: libc::c_int = 0;
    let mut usershape_p: bool = false;
    let mut pfilled: bool = false;
    let mut color: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut doMap: libc::c_int = (!((*obj).url).is_null()
        || (*obj).explicit_tooltip() as libc::c_int != 0) as libc::c_int;
    let mut fillcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pencolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clrs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    if doMap != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
    }
    poly = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
        as *mut polygon_t;
    vertices = (*poly).vertices;
    sides = (*poly).sides;
    peripheries = (*poly).peripheries;
    if A_size < sides {
        A_size = sides + 5 as libc::c_int;
        AF = if !AF.is_null() {
            grealloc(
                AF as *mut libc::c_void,
                (A_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
            ) as *mut pointf
        } else {
            gmalloc(
                (A_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
            ) as *mut pointf
        };
    }
    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
        .pos = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    xsize = ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
        / (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        }) as libc::c_double;
    ysize = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
        / (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        }) as libc::c_double;
    style = stylenode(job, n);
    clrs[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        pencolor = late_nnstring(
            n as *mut libc::c_void,
            N_activepencolor,
            b"#808080\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, pencolor);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_activefillcolor,
            b"#fcfcfc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
        filled = 1 as libc::c_int;
    } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state
            as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
        pencolor = late_nnstring(
            n as *mut libc::c_void,
            N_selectedpencolor,
            b"#303030\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, pencolor);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_selectedfillcolor,
            b"#e8e8e8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
        filled = 1 as libc::c_int;
    } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state
            as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0
        {
        pencolor = late_nnstring(
            n as *mut libc::c_void,
            N_deletedpencolor,
            b"#e0e0e0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, pencolor);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_deletedfillcolor,
            b"#f0f0f0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
        filled = 1 as libc::c_int;
    } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state
            as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
        {
        pencolor = late_nnstring(
            n as *mut libc::c_void,
            N_visitedpencolor,
            b"#101010\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, pencolor);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_visitedfillcolor,
            b"#f8f8f8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
        filled = 1 as libc::c_int;
    } else {
        if style & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            let mut frac: libc::c_float = 0.;
            fillcolor = findFill(n);
            if findStopColor(fillcolor, clrs.as_mut_ptr(), &mut frac) {
                gvrender_set_fillcolor(job, clrs[0 as libc::c_int as usize]);
                if !(clrs[1 as libc::c_int as usize]).is_null() {
                    gvrender_set_gradient_vals(
                        job,
                        clrs[1 as libc::c_int as usize],
                        late_int(
                            n as *mut libc::c_void,
                            N_gradientangle,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        ),
                        frac,
                    );
                } else {
                    gvrender_set_gradient_vals(
                        job,
                        b"black\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        late_int(
                            n as *mut libc::c_void,
                            N_gradientangle,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        ),
                        frac,
                    );
                }
                if style & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                    filled = 3 as libc::c_int;
                } else {
                    filled = 2 as libc::c_int;
                }
            } else {
                gvrender_set_fillcolor(job, fillcolor);
                filled = 1 as libc::c_int;
            }
        } else if style
                & ((1 as libc::c_int) << 6 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) != 0
            {
            fillcolor = findFill(n);
            filled = 1 as libc::c_int;
        } else {
            filled = 0 as libc::c_int;
        }
        pencolor = penColor(job, n);
    }
    pfilled = !(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).usershape
        || strcmp(
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
            b"custom\0" as *const u8 as *const libc::c_char,
        ) == 0;
    if peripheries == 0 as libc::c_int && filled != 0 as libc::c_int
        && pfilled as libc::c_int != 0
    {
        peripheries = 1 as libc::c_int;
        gvrender_set_pencolor(
            job,
            b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    j = 0 as libc::c_int;
    while j < peripheries {
        i = 0 as libc::c_int;
        while i < sides {
            P = *vertices.offset((i + j * sides) as isize);
            (*AF.offset(i as isize))
                .x = P.x * xsize
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
            (*AF.offset(i as isize))
                .y = P.y * ysize
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
            i += 1;
        }
        if sides <= 2 as libc::c_int {
            if style & (1 as libc::c_int) << 9 as libc::c_int != 0
                && j == 0 as libc::c_int && !(strchr(fillcolor, ':' as i32)).is_null()
            {
                let mut rv: libc::c_int = wedgedEllipse(job, AF, fillcolor);
                if rv > 1 as libc::c_int {
                    agerr(
                        AGPREV,
                        b"in node %s\n\0" as *const u8 as *const libc::c_char,
                        agnameof(n as *mut libc::c_void),
                    );
                }
                filled = 0 as libc::c_int;
            }
            gvrender_ellipse(job, AF, sides, filled);
            if style & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                Mcircle_hack(job, n);
            }
        } else if style & (1 as libc::c_int) << 6 as libc::c_int != 0 {
            if j == 0 as libc::c_int {
                let mut rv_0: libc::c_int = stripedBox(
                    job,
                    AF,
                    fillcolor,
                    1 as libc::c_int,
                );
                if rv_0 > 1 as libc::c_int {
                    agerr(
                        AGPREV,
                        b"in node %s\n\0" as *const u8 as *const libc::c_char,
                        agnameof(n as *mut libc::c_void),
                    );
                }
            }
            gvrender_polygon(job, AF, sides, 0 as libc::c_int);
        } else if style & (1 as libc::c_int) << 10 as libc::c_int != 0 {
            gvrender_set_pencolor(
                job,
                b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            gvrender_polygon(job, AF, sides, filled);
            gvrender_set_pencolor(job, pencolor);
            gvrender_polyline(
                job,
                AF.offset(2 as libc::c_int as isize),
                2 as libc::c_int,
            );
        } else if style
                & ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int
                    | (127 as libc::c_int) << 24 as libc::c_int) != 0
            {
            round_corners(job, AF, sides, style, filled);
        } else {
            gvrender_polygon(job, AF, sides, filled);
        }
        filled = 0 as libc::c_int;
        j += 1;
    }
    usershape_p = 0 as libc::c_int != 0;
    if (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).usershape {
        name = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name;
        if strcmp(name, b"custom\0" as *const u8 as *const libc::c_char) == 0 {
            name = agget(
                n as *mut libc::c_void,
                b"shapefile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !name.is_null()
                && *name.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                usershape_p = 1 as libc::c_int != 0;
            }
        } else {
            usershape_p = 1 as libc::c_int != 0;
        }
    } else {
        name = agget(
            n as *mut libc::c_void,
            b"image\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !name.is_null() && *name.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            usershape_p = 1 as libc::c_int != 0;
        }
    }
    if usershape_p {
        i = 0 as libc::c_int;
        while i < sides {
            P = *vertices.offset(i as isize);
            (*AF.offset(i as isize))
                .x = P.x * xsize
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
            (*AF.offset(i as isize))
                .y = P.y * ysize
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
            i += 1;
        }
        if filled != 0 as libc::c_int && pfilled as libc::c_int != 0 {
            if sides <= 2 as libc::c_int {
                if style & (1 as libc::c_int) << 9 as libc::c_int != 0
                    && j == 0 as libc::c_int
                    && !(strchr(fillcolor, ':' as i32)).is_null()
                {
                    let mut rv_1: libc::c_int = wedgedEllipse(job, AF, fillcolor);
                    if rv_1 > 1 as libc::c_int {
                        agerr(
                            AGPREV,
                            b"in node %s\n\0" as *const u8 as *const libc::c_char,
                            agnameof(n as *mut libc::c_void),
                        );
                    }
                    filled = 0 as libc::c_int;
                }
                gvrender_ellipse(job, AF, sides, filled);
                if style & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                    Mcircle_hack(job, n);
                }
            } else if style & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                let mut rv_2: libc::c_int = stripedBox(
                    job,
                    AF,
                    fillcolor,
                    1 as libc::c_int,
                );
                if rv_2 > 1 as libc::c_int {
                    agerr(
                        AGPREV,
                        b"in node %s\n\0" as *const u8 as *const libc::c_char,
                        agnameof(n as *mut libc::c_void),
                    );
                }
                gvrender_polygon(job, AF, sides, 0 as libc::c_int);
            } else if style
                    & ((1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 3 as libc::c_int) != 0
                {
                round_corners(job, AF, sides, style, filled);
            } else {
                gvrender_polygon(job, AF, sides, filled);
            }
        }
        gvrender_usershape(
            job,
            name,
            AF,
            sides,
            filled != 0 as libc::c_int,
            late_string(
                n as *mut libc::c_void,
                N_imagescale,
                b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            late_string(
                n as *mut libc::c_void,
                N_imagepos,
                b"mc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        );
        filled = 0 as libc::c_int;
    }
    free(clrs[0 as libc::c_int as usize] as *mut libc::c_void);
    emit_label(
        job,
        EMIT_NLABEL,
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label,
    );
    if doMap != 0 {
        if (*job).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            gvrender_begin_anchor(
                job,
                (*obj).url,
                (*obj).tooltip,
                (*obj).target,
                (*obj).id,
            );
        }
        gvrender_end_anchor(job);
    }
}
unsafe extern "C" fn point_init(mut n: *mut node_t) {
    let mut poly: *mut polygon_t = zmalloc(
        ::std::mem::size_of::<polygon_t>() as libc::c_ulong,
    ) as *mut polygon_t;
    let mut sides: libc::c_int = 0;
    let mut outp: libc::c_int = 0;
    let mut peripheries: libc::c_int = (*(*(*((*(n as *mut Agobj_t)).data
        as *mut Agnodeinfo_t))
        .shape)
        .polygon)
        .peripheries;
    let mut sz: libc::c_double = 0.;
    let mut P: pointf = pointf { x: 0., y: 0. };
    let mut vertices: *mut pointf = 0 as *mut pointf;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    w = late_double(n as *mut libc::c_void, N_width, 1.7976931348623157e+308f64, 0.0f64);
    h = late_double(
        n as *mut libc::c_void,
        N_height,
        1.7976931348623157e+308f64,
        0.0f64,
    );
    w = if w < h { w } else { h };
    if w == 1.7976931348623157e+308f64 && h == 1.7976931348623157e+308f64 {
        let ref mut fresh25 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .height;
        *fresh25 = 0.05f64;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width = *fresh25;
    } else {
        w = if w < h { w } else { h };
        if w > 0.0f64 {
            w = if w > 0.0003f64 { w } else { 0.0003f64 };
        }
        let ref mut fresh26 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .height;
        *fresh26 = w;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width = *fresh26;
    }
    sz = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
        * 72 as libc::c_int as libc::c_double;
    peripheries = late_int(
        n as *mut libc::c_void,
        N_peripheries,
        peripheries,
        0 as libc::c_int,
    );
    if peripheries < 1 as libc::c_int {
        outp = 1 as libc::c_int;
    } else {
        outp = peripheries;
    }
    sides = 2 as libc::c_int;
    vertices = gcalloc(
        (outp * sides) as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    P.x = sz / 2.0f64;
    P.y = P.x;
    (*vertices.offset(0 as libc::c_int as isize)).x = -P.x;
    (*vertices.offset(0 as libc::c_int as isize)).y = -P.y;
    *vertices.offset(1 as libc::c_int as isize) = P;
    if peripheries > 1 as libc::c_int {
        j = 1 as libc::c_int;
        i = 2 as libc::c_int;
        while j < peripheries {
            P.x += 4 as libc::c_int as libc::c_double;
            P.y += 4 as libc::c_int as libc::c_double;
            (*vertices.offset(i as isize)).x = -P.x;
            (*vertices.offset(i as isize)).y = -P.y;
            i += 1;
            (*vertices.offset(i as isize)).x = P.x;
            (*vertices.offset(i as isize)).y = P.y;
            i += 1;
            j += 1;
        }
        sz = 2.0f64 * P.x;
    }
    (*poly).regular = 1 as libc::c_int;
    (*poly).peripheries = peripheries;
    (*poly).sides = 2 as libc::c_int;
    (*poly).orientation = 0 as libc::c_int as libc::c_double;
    (*poly).skew = 0 as libc::c_int as libc::c_double;
    (*poly).distortion = 0 as libc::c_int as libc::c_double;
    let ref mut fresh27 = (*poly).vertices;
    *fresh27 = vertices;
    let ref mut fresh28 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width;
    *fresh28 = sz / 72 as libc::c_int as libc::c_double;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height = *fresh28;
    let ref mut fresh29 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .shape_info;
    *fresh29 = poly as *mut libc::c_void;
}
unsafe extern "C" fn point_inside(
    mut inside_context: *mut inside_t,
    mut p: pointf,
) -> bool {
    static mut lastn: *mut node_t = 0 as *const node_t as *mut node_t;
    static mut radius: libc::c_double = 0.;
    let mut P: pointf = pointf { x: 0., y: 0. };
    let mut n: *mut node_t = 0 as *mut node_t;
    if inside_context.is_null() {
        lastn = 0 as *mut node_t;
        return 0 as libc::c_int != 0;
    }
    n = (*inside_context).s.n;
    P = ccwrotatepf(
        p,
        90 as libc::c_int
            * ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int),
    );
    if n != lastn {
        let mut outp: libc::c_int = 0;
        let mut poly: *mut polygon_t = (*((*(n as *mut Agobj_t)).data
            as *mut Agnodeinfo_t))
            .shape_info as *mut polygon_t;
        outp = 2 as libc::c_int * ((*poly).peripheries - 1 as libc::c_int);
        if outp < 0 as libc::c_int {
            outp = 0 as libc::c_int;
        }
        radius = (*((*poly).vertices).offset((outp + 1 as libc::c_int) as isize)).x;
        lastn = n;
    }
    if fabs(P.x) > radius || fabs(P.y) > radius {
        return 0 as libc::c_int != 0;
    }
    return hypot(P.x, P.y) <= radius;
}
unsafe extern "C" fn point_gencode(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut poly: *mut polygon_t = 0 as *mut polygon_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut peripheries: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut P: pointf = pointf { x: 0., y: 0. };
    let mut vertices: *mut pointf = 0 as *mut pointf;
    static mut AF: *mut pointf = 0 as *const pointf as *mut pointf;
    static mut A_size: libc::c_int = 0;
    let mut filled: bool = false;
    let mut color: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut doMap: libc::c_int = (!((*obj).url).is_null()
        || (*obj).explicit_tooltip() as libc::c_int != 0) as libc::c_int;
    if doMap != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
    }
    poly = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
        as *mut polygon_t;
    vertices = (*poly).vertices;
    sides = (*poly).sides;
    peripheries = (*poly).peripheries;
    if A_size < sides {
        A_size = sides + 2 as libc::c_int;
        AF = if !AF.is_null() {
            grealloc(
                AF as *mut libc::c_void,
                (A_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
            ) as *mut pointf
        } else {
            gmalloc(
                (A_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
            ) as *mut pointf
        };
    }
    checkStyle(n, &mut style);
    if style & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        gvrender_set_style(job, point_style.as_mut_ptr());
    } else {
        gvrender_set_style(
            job,
            &mut *point_style.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
    }
    if !N_penwidth.is_null() {
        gvrender_set_penwidth(
            job,
            late_double(n as *mut libc::c_void, N_penwidth, 1.0f64, 0.0f64),
        );
    }
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        color = late_nnstring(
            n as *mut libc::c_void,
            N_activepencolor,
            b"#808080\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, color);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_activefillcolor,
            b"#fcfcfc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
    } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state
            as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
        color = late_nnstring(
            n as *mut libc::c_void,
            N_selectedpencolor,
            b"#303030\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, color);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_selectedfillcolor,
            b"#e8e8e8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
    } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state
            as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0
        {
        color = late_nnstring(
            n as *mut libc::c_void,
            N_deletedpencolor,
            b"#e0e0e0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, color);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_deletedfillcolor,
            b"#f0f0f0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
    } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state
            as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
        {
        color = late_nnstring(
            n as *mut libc::c_void,
            N_visitedpencolor,
            b"#101010\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_pencolor(job, color);
        color = late_nnstring(
            n as *mut libc::c_void,
            N_visitedfillcolor,
            b"#f8f8f8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
    } else {
        color = findFillDflt(
            n,
            b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gvrender_set_fillcolor(job, color);
        penColor(job, n);
    }
    filled = 1 as libc::c_int != 0;
    if peripheries == 0 as libc::c_int {
        peripheries = 1 as libc::c_int;
        if *color.offset(0 as libc::c_int as isize) != 0 {
            gvrender_set_pencolor(job, color);
        }
    }
    j = 0 as libc::c_int;
    while j < peripheries {
        i = 0 as libc::c_int;
        while i < sides {
            P = *vertices.offset((i + j * sides) as isize);
            (*AF.offset(i as isize))
                .x = P.x + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
            (*AF.offset(i as isize))
                .y = P.y + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
            i += 1;
        }
        gvrender_ellipse(job, AF, sides, filled as libc::c_int);
        filled = 0 as libc::c_int != 0;
        j += 1;
    }
    if doMap != 0 {
        if (*job).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            gvrender_begin_anchor(
                job,
                (*obj).url,
                (*obj).tooltip,
                (*obj).target,
                (*obj).id,
            );
        }
        gvrender_end_anchor(job);
    }
}
static mut reclblp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn free_field(mut f: *mut field_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*f).n_flds {
        free_field(*((*f).fld).offset(i as isize));
        i += 1;
    }
    free((*f).id as *mut libc::c_void);
    free_label((*f).lp);
    free((*f).fld as *mut libc::c_void);
    free(f as *mut libc::c_void);
}
unsafe extern "C" fn parse_error(
    mut rv: *mut field_t,
    mut port: *mut libc::c_char,
) -> *mut field_t {
    free_field(rv);
    if !port.is_null() {
        free(port as *mut libc::c_void);
    }
    return 0 as *mut field_t;
}
unsafe extern "C" fn parse_reclbl(
    mut n: *mut node_t,
    mut LR: bool,
    mut flag: libc::c_int,
    mut text: *mut libc::c_char,
) -> *mut field_t {
    let mut fp: *mut field_t = 0 as *mut field_t;
    let mut rv: *mut field_t = zmalloc(::std::mem::size_of::<field_t>() as libc::c_ulong)
        as *mut field_t;
    let mut tsp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hstsp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hspsp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpport: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxf: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut wflag: libc::c_int = 0;
    let mut ishardspace: libc::c_int = 0;
    let mut fi: libc::c_int = 0;
    let mut lbl: *mut textlabel_t = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .label;
    let mut uc: libc::c_uchar = 0;
    fp = 0 as *mut field_t;
    let mut current_block_8: u64;
    maxf = 1 as libc::c_int;
    cnt = 0 as libc::c_int;
    sp = reclblp;
    while *sp != 0 {
        if *sp as libc::c_int == '\\' as i32 {
            sp = sp.offset(1);
            if *sp as libc::c_int != 0
                && (*sp as libc::c_int == '{' as i32 || *sp as libc::c_int == '}' as i32
                    || *sp as libc::c_int == '|' as i32
                    || *sp as libc::c_int == '\\' as i32)
            {
                current_block_8 = 7095457783677275021;
            } else {
                current_block_8 = 3640593987805443782;
            }
        } else {
            current_block_8 = 3640593987805443782;
        }
        match current_block_8 {
            3640593987805443782 => {
                if *sp as libc::c_int == '{' as i32 {
                    cnt += 1;
                } else if *sp as libc::c_int == '}' as i32 {
                    cnt -= 1;
                } else if *sp as libc::c_int == '|' as i32 && cnt == 0 as libc::c_int {
                    maxf += 1;
                }
                if cnt < 0 as libc::c_int {
                    break;
                }
            }
            _ => {}
        }
        sp = sp.offset(1);
    }
    let ref mut fresh30 = (*rv).fld;
    *fresh30 = gcalloc(
        maxf as size_t,
        ::std::mem::size_of::<*mut field_t>() as libc::c_ulong,
    ) as *mut *mut field_t;
    (*rv).LR = LR as libc::c_uchar;
    mode = 0 as libc::c_int;
    fi = 0 as libc::c_int;
    tsp = text;
    hstsp = tsp;
    wflag = (0 as libc::c_int == 0) as libc::c_int;
    ishardspace = 0 as libc::c_int;
    while wflag != 0 {
        uc = *(reclblp as *mut libc::c_uchar);
        if uc as libc::c_int != 0 && (uc as libc::c_int) < ' ' as i32 {
            reclblp = reclblp.offset(1);
        } else {
            match *reclblp as libc::c_int {
                60 => {
                    if mode & (4 as libc::c_int | 2 as libc::c_int) != 0 {
                        return parse_error(rv, tmpport);
                    }
                    if !(*lbl).html {
                        mode |= 2 as libc::c_int | 16 as libc::c_int;
                        reclblp = reclblp.offset(1);
                        psp = text;
                        hspsp = psp;
                        continue;
                    }
                }
                62 => {
                    if !(*lbl).html {
                        if mode & 16 as libc::c_int == 0 {
                            return parse_error(rv, tmpport);
                        }
                        if psp > text.offset(1 as libc::c_int as isize)
                            && psp.offset(-(1 as libc::c_int as isize)) != hspsp
                            && *psp.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                == ' ' as i32
                        {
                            psp = psp.offset(-1);
                        }
                        *psp = '\0' as i32 as libc::c_char;
                        tmpport = strdup(text);
                        mode &= !(16 as libc::c_int);
                        reclblp = reclblp.offset(1);
                        continue;
                    }
                }
                123 => {
                    reclblp = reclblp.offset(1);
                    if mode != 0 as libc::c_int || *reclblp == 0 {
                        return parse_error(rv, tmpport);
                    }
                    mode = 4 as libc::c_int;
                    let fresh31 = fi;
                    fi = fi + 1;
                    let ref mut fresh32 = *((*rv).fld).offset(fresh31 as isize);
                    *fresh32 = parse_reclbl(n, !LR, 0 as libc::c_int, text);
                    if (*fresh32).is_null() {
                        return parse_error(rv, tmpport);
                    }
                    continue;
                }
                125 | 124 | 0 => {
                    if *reclblp == 0 && flag == 0 || mode & 16 as libc::c_int != 0 {
                        return parse_error(rv, tmpport);
                    }
                    if mode & 4 as libc::c_int == 0 {
                        let fresh33 = fi;
                        fi = fi + 1;
                        let ref mut fresh34 = *((*rv).fld).offset(fresh33 as isize);
                        *fresh34 = zmalloc(
                            ::std::mem::size_of::<field_t>() as libc::c_ulong,
                        ) as *mut field_t;
                        fp = *fresh34;
                    }
                    if !tmpport.is_null() {
                        let ref mut fresh35 = (*fp).id;
                        *fresh35 = tmpport;
                        tmpport = 0 as *mut libc::c_char;
                    }
                    if mode & (1 as libc::c_int | 4 as libc::c_int) == 0 {
                        mode |= 1 as libc::c_int;
                        let fresh36 = tsp;
                        tsp = tsp.offset(1);
                        *fresh36 = ' ' as i32 as libc::c_char;
                    }
                    if mode & 1 as libc::c_int != 0 {
                        if tsp > text.offset(1 as libc::c_int as isize)
                            && tsp.offset(-(1 as libc::c_int as isize)) != hstsp
                            && *tsp.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                == ' ' as i32
                        {
                            tsp = tsp.offset(-1);
                        }
                        *tsp = '\0' as i32 as libc::c_char;
                        let ref mut fresh37 = (*fp).lp;
                        *fresh37 = make_label(
                            n as *mut libc::c_void,
                            text,
                            if (*lbl).html as libc::c_int != 0 {
                                (1 as libc::c_int) << 1 as libc::c_int
                            } else {
                                (0 as libc::c_int) << 1 as libc::c_int
                            },
                            (*lbl).fontsize,
                            (*lbl).fontname,
                            (*lbl).fontcolor,
                        );
                        (*fp)
                            .LR = (0 as libc::c_int == 0) as libc::c_int
                            as libc::c_uchar;
                        tsp = text;
                        hstsp = tsp;
                    }
                    if *reclblp != 0 {
                        if *reclblp as libc::c_int == '}' as i32 {
                            reclblp = reclblp.offset(1);
                            (*rv).n_flds = fi;
                            return rv;
                        }
                        mode = 0 as libc::c_int;
                        reclblp = reclblp.offset(1);
                    } else {
                        wflag = 0 as libc::c_int;
                    }
                    continue;
                }
                92 => {
                    if *reclblp.offset(1 as libc::c_int as isize) != 0 {
                        if *reclblp.offset(1 as libc::c_int as isize) as libc::c_int
                            == '{' as i32
                            || *reclblp.offset(1 as libc::c_int as isize) as libc::c_int
                                == '}' as i32
                            || *reclblp.offset(1 as libc::c_int as isize) as libc::c_int
                                == '|' as i32
                            || *reclblp.offset(1 as libc::c_int as isize) as libc::c_int
                                == '<' as i32
                            || *reclblp.offset(1 as libc::c_int as isize) as libc::c_int
                                == '>' as i32
                        {
                            reclblp = reclblp.offset(1);
                        } else if *reclblp.offset(1 as libc::c_int as isize)
                                as libc::c_int == ' ' as i32 && !(*lbl).html
                            {
                            ishardspace = (0 as libc::c_int == 0) as libc::c_int;
                            reclblp = reclblp.offset(1);
                        } else {
                            let fresh38 = tsp;
                            tsp = tsp.offset(1);
                            *fresh38 = '\\' as i32 as libc::c_char;
                            mode |= 8 as libc::c_int | 1 as libc::c_int;
                            reclblp = reclblp.offset(1);
                        }
                    }
                }
                _ => {}
            }
            if mode & 4 as libc::c_int != 0 && *reclblp as libc::c_int != ' ' as i32 {
                return parse_error(rv, tmpport);
            }
            if mode & (8 as libc::c_int | 16 as libc::c_int) == 0
                && *reclblp as libc::c_int != ' ' as i32
            {
                mode |= 8 as libc::c_int | 1 as libc::c_int;
            }
            if mode & 8 as libc::c_int != 0 {
                if !(*reclblp as libc::c_int == ' ' as i32 && ishardspace == 0
                    && *tsp.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        == ' ' as i32 && !(*lbl).html)
                {
                    let fresh39 = tsp;
                    tsp = tsp.offset(1);
                    *fresh39 = *reclblp;
                }
                if ishardspace != 0 {
                    hstsp = tsp.offset(-(1 as libc::c_int as isize));
                }
            } else if mode & 16 as libc::c_int != 0 {
                if !(*reclblp as libc::c_int == ' ' as i32 && ishardspace == 0
                    && (psp == text
                        || *psp.offset(-(1 as libc::c_int as isize)) as libc::c_int
                            == ' ' as i32))
                {
                    let fresh40 = psp;
                    psp = psp.offset(1);
                    *fresh40 = *reclblp;
                }
                if ishardspace != 0 {
                    hspsp = psp.offset(-(1 as libc::c_int as isize));
                }
            }
            reclblp = reclblp.offset(1);
            while *reclblp as libc::c_int & 128 as libc::c_int != 0 {
                let fresh41 = reclblp;
                reclblp = reclblp.offset(1);
                let fresh42 = tsp;
                tsp = tsp.offset(1);
                *fresh42 = *fresh41;
            }
        }
    }
    (*rv).n_flds = fi;
    return rv;
}
unsafe extern "C" fn size_reclbl(mut n: *mut node_t, mut f: *mut field_t) -> pointf {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut marginx: libc::c_double = 0.;
    let mut marginy: libc::c_double = 0.;
    let mut d: pointf = pointf { x: 0., y: 0. };
    let mut d0: pointf = pointf { x: 0., y: 0. };
    let mut dimen: pointf = pointf { x: 0., y: 0. };
    if !((*f).lp).is_null() {
        dimen = (*(*f).lp).dimen;
        if dimen.x > 0.0f64 || dimen.y > 0.0f64 {
            p = agget(
                n as *mut libc::c_void,
                b"margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !p.is_null() {
                i = sscanf(
                    p,
                    b"%lf,%lf\0" as *const u8 as *const libc::c_char,
                    &mut marginx as *mut libc::c_double,
                    &mut marginy as *mut libc::c_double,
                );
                if i > 0 as libc::c_int {
                    dimen.x
                        += (2 as libc::c_int
                            * (if marginx * 72 as libc::c_int as libc::c_double
                                >= 0 as libc::c_int as libc::c_double
                            {
                                (marginx * 72 as libc::c_int as libc::c_double + 0.5f64)
                                    as libc::c_int
                            } else {
                                (marginx * 72 as libc::c_int as libc::c_double - 0.5f64)
                                    as libc::c_int
                            })) as libc::c_double;
                    if i > 1 as libc::c_int {
                        dimen.y
                            += (2 as libc::c_int
                                * (if marginy * 72 as libc::c_int as libc::c_double
                                    >= 0 as libc::c_int as libc::c_double
                                {
                                    (marginy * 72 as libc::c_int as libc::c_double + 0.5f64)
                                        as libc::c_int
                                } else {
                                    (marginy * 72 as libc::c_int as libc::c_double - 0.5f64)
                                        as libc::c_int
                                })) as libc::c_double;
                    } else {
                        dimen.y
                            += (2 as libc::c_int
                                * (if marginx * 72 as libc::c_int as libc::c_double
                                    >= 0 as libc::c_int as libc::c_double
                                {
                                    (marginx * 72 as libc::c_int as libc::c_double + 0.5f64)
                                        as libc::c_int
                                } else {
                                    (marginx * 72 as libc::c_int as libc::c_double - 0.5f64)
                                        as libc::c_int
                                })) as libc::c_double;
                    }
                } else {
                    dimen.x += (4 as libc::c_int * 4 as libc::c_int) as libc::c_double;
                    dimen.y += (2 as libc::c_int * 4 as libc::c_int) as libc::c_double;
                }
            } else {
                dimen.x += (4 as libc::c_int * 4 as libc::c_int) as libc::c_double;
                dimen.y += (2 as libc::c_int * 4 as libc::c_int) as libc::c_double;
            }
        }
        d = dimen;
    } else {
        d.y = 0 as libc::c_int as libc::c_double;
        d.x = d.y;
        i = 0 as libc::c_int;
        while i < (*f).n_flds {
            d0 = size_reclbl(n, *((*f).fld).offset(i as isize));
            if (*f).LR != 0 {
                d.x += d0.x;
                d.y = if d.y > d0.y { d.y } else { d0.y };
            } else {
                d.y += d0.y;
                d.x = if d.x > d0.x { d.x } else { d0.x };
            }
            i += 1;
        }
    }
    (*f).size = d;
    return d;
}
unsafe extern "C" fn resize_reclbl(
    mut f: *mut field_t,
    mut sz: pointf,
    mut nojustify_p: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut amt: libc::c_int = 0;
    let mut inc: libc::c_double = 0.;
    let mut d: pointf = pointf { x: 0., y: 0. };
    let mut newsz: pointf = pointf { x: 0., y: 0. };
    let mut sf: *mut field_t = 0 as *mut field_t;
    d.x = sz.x - (*f).size.x;
    d.y = sz.y - (*f).size.y;
    (*f).size = sz;
    if !((*f).lp).is_null() && nojustify_p == 0 {
        (*(*f).lp).space.x += d.x;
        (*(*f).lp).space.y += d.y;
    }
    if (*f).n_flds != 0 {
        if (*f).LR != 0 {
            inc = d.x / (*f).n_flds as libc::c_double;
        } else {
            inc = d.y / (*f).n_flds as libc::c_double;
        }
        i = 0 as libc::c_int;
        while i < (*f).n_flds {
            sf = *((*f).fld).offset(i as isize);
            amt = ((i + 1 as libc::c_int) as libc::c_double * inc) as libc::c_int
                - (i as libc::c_double * inc) as libc::c_int;
            if (*f).LR != 0 {
                newsz = pointfof((*sf).size.x + amt as libc::c_double, sz.y);
            } else {
                newsz = pointfof(sz.x, (*sf).size.y + amt as libc::c_double);
            }
            resize_reclbl(sf, newsz, nojustify_p);
            i += 1;
        }
    }
}
unsafe extern "C" fn pos_reclbl(
    mut f: *mut field_t,
    mut ul: pointf,
    mut sides: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    (*f).sides = sides as libc::c_uchar;
    (*f).b.LL = pointfof(ul.x, ul.y - (*f).size.y);
    (*f).b.UR = pointfof(ul.x + (*f).size.x, ul.y);
    last = (*f).n_flds - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= last {
        if sides != 0 {
            if (*f).LR != 0 {
                if i == 0 as libc::c_int {
                    if i == last {
                        mask = (1 as libc::c_int) << 2 as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 3 as libc::c_int;
                    } else {
                        mask = (1 as libc::c_int) << 2 as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 3 as libc::c_int;
                    }
                } else if i == last {
                    mask = (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int;
                } else {
                    mask = (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int;
                }
            } else if i == 0 as libc::c_int {
                if i == last {
                    mask = (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 3 as libc::c_int;
                } else {
                    mask = (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 3 as libc::c_int;
                }
            } else if i == last {
                mask = (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int;
            } else {
                mask = (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int;
            }
        } else {
            mask = 0 as libc::c_int;
        }
        pos_reclbl(*((*f).fld).offset(i as isize), ul, sides & mask);
        if (*f).LR != 0 {
            ul.x = ul.x + (**((*f).fld).offset(i as isize)).size.x;
        } else {
            ul.y = ul.y - (**((*f).fld).offset(i as isize)).size.y;
        }
        i += 1;
    }
}
unsafe extern "C" fn record_init(mut n: *mut node_t) {
    let mut info: *mut field_t = 0 as *mut field_t;
    let mut ul: pointf = pointf { x: 0., y: 0. };
    let mut sz: pointf = pointf { x: 0., y: 0. };
    let mut flip: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut textbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sides: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int
        | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int;
    flip = ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .rankdir >> 2 as libc::c_int & 1 as libc::c_int == 0) as libc::c_int;
    reclblp = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).text;
    len = strlen(reclblp);
    len = if (if len > 1 as libc::c_int as libc::c_ulong {
        len
    } else {
        1 as libc::c_int as libc::c_ulong
    }) > strlen(b"\\N\0" as *const u8 as *const libc::c_char)
    {
        if len > 1 as libc::c_int as libc::c_ulong {
            len
        } else {
            1 as libc::c_int as libc::c_ulong
        }
    } else {
        strlen(b"\\N\0" as *const u8 as *const libc::c_char)
    };
    textbuf = gcalloc(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    info = parse_reclbl(n, flip != 0, 1 as libc::c_int, textbuf);
    if info.is_null() {
        agerr(
            AGERR,
            b"bad label format %s\n\0" as *const u8 as *const libc::c_char,
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).text,
        );
        reclblp = b"\\N\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        info = parse_reclbl(n, flip != 0, 1 as libc::c_int, textbuf);
    }
    free(textbuf as *mut libc::c_void);
    size_reclbl(n, info);
    sz
        .x = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
        * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
    {
        ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
    } else {
        ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
    }) as libc::c_double;
    sz
        .y = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
        * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
    {
        ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
    } else {
        ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
    }) as libc::c_double;
    if mapbool(
        late_string(
            n as *mut libc::c_void,
            N_fixed,
            b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) {
        sz.x < (*info).size.x || sz.y < (*info).size.y;
    } else {
        sz.x = if (*info).size.x > sz.x { (*info).size.x } else { sz.x };
        sz.y = if (*info).size.y > sz.y { (*info).size.y } else { sz.y };
    }
    resize_reclbl(
        info,
        sz,
        if mapbool(
            late_string(
                n as *mut libc::c_void,
                N_nojustify,
                b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        ) as libc::c_int != 0
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else {
            0 as libc::c_int
        },
    );
    ul = pointfof(-sz.x / 2.0f64, sz.y / 2.0f64);
    pos_reclbl(info, ul, sides);
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .width = (*info).size.x / 72 as libc::c_int as libc::c_double;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .height = ((*info).size.y + 1 as libc::c_int as libc::c_double)
        / 72 as libc::c_int as libc::c_double;
    let ref mut fresh43 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .shape_info;
    *fresh43 = info as *mut libc::c_void;
}
unsafe extern "C" fn record_free(mut n: *mut node_t) {
    let mut p: *mut field_t = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .shape_info as *mut field_t;
    free_field(p);
}
unsafe extern "C" fn map_rec_port(
    mut f: *mut field_t,
    mut str: *mut libc::c_char,
) -> *mut field_t {
    let mut rv: *mut field_t = 0 as *mut field_t;
    let mut sub: libc::c_int = 0;
    if !((*f).id).is_null() && strcmp((*f).id, str) == 0 {
        rv = f;
    } else {
        rv = 0 as *mut field_t;
        sub = 0 as libc::c_int;
        while sub < (*f).n_flds {
            rv = map_rec_port(*((*f).fld).offset(sub as isize), str);
            if !rv.is_null() {
                break;
            }
            sub += 1;
        }
    }
    return rv;
}
unsafe extern "C" fn record_port(
    mut n: *mut node_t,
    mut portname: *mut libc::c_char,
    mut compass: *mut libc::c_char,
) -> port {
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut subf: *mut field_t = 0 as *mut field_t;
    let mut rv: port = port {
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
    let mut sides: libc::c_int = 0;
    if *portname.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return Center;
    }
    sides = (1 as libc::c_int) << 0 as libc::c_int
        | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int;
    if compass.is_null() {
        compass = b"_\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    f = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info as *mut field_t;
    subf = map_rec_port(f, portname);
    if !subf.is_null() {
        if compassPort(
            n,
            &mut (*subf).b,
            &mut rv,
            compass,
            (*subf).sides as libc::c_int,
            0 as *mut inside_t,
        ) != 0
        {
            agerr(
                AGWARN,
                b"node %s, port %s, unrecognized compass point '%s' - ignored\n\0"
                    as *const u8 as *const libc::c_char,
                agnameof(n as *mut libc::c_void),
                portname,
                compass,
            );
        }
    } else if compassPort(n, &mut (*f).b, &mut rv, portname, sides, 0 as *mut inside_t)
            != 0
        {
        unrecognized(n, portname);
    }
    return rv;
}
unsafe extern "C" fn record_inside(
    mut inside_context: *mut inside_t,
    mut p: pointf,
) -> bool {
    let mut fld0: *mut field_t = 0 as *mut field_t;
    let mut bp: *mut boxf = (*inside_context).s.bp;
    let mut n: *mut node_t = (*inside_context).s.n;
    let mut bbox: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    p = ccwrotatepf(
        p,
        90 as libc::c_int
            * ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int),
    );
    if bp.is_null() {
        fld0 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
            as *mut field_t;
        bbox = (*fld0).b;
    } else {
        bbox = *bp;
    }
    return bbox.LL.x <= p.x && p.x <= bbox.UR.x
        && (bbox.LL.y <= p.y && p.y <= bbox.UR.y);
}
unsafe extern "C" fn record_path(
    mut n: *mut node_t,
    mut prt: *mut port,
    mut side: libc::c_int,
    mut rv: *mut boxf,
    mut kptr: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ls: libc::c_int = 0;
    let mut rs: libc::c_int = 0;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut info: *mut field_t = 0 as *mut field_t;
    if !(*prt).defined {
        return 0 as libc::c_int;
    }
    p = (*prt).p;
    info = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
        as *mut field_t;
    i = 0 as libc::c_int;
    while i < (*info).n_flds {
        if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int == 0
        {
            ls = (**((*info).fld).offset(i as isize)).b.LL.x as libc::c_int;
            rs = (**((*info).fld).offset(i as isize)).b.UR.x as libc::c_int;
        } else {
            ls = (**((*info).fld).offset(i as isize)).b.LL.y as libc::c_int;
            rs = (**((*info).fld).offset(i as isize)).b.UR.y as libc::c_int;
        }
        if ls as libc::c_double <= p.x && p.x <= rs as libc::c_double {
            if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                *rv
                    .offset(
                        0 as libc::c_int as isize,
                    ) = flip_rec_boxf(
                    (**((*info).fld).offset(i as isize)).b,
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
                );
            } else {
                (*rv.offset(0 as libc::c_int as isize))
                    .LL
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    + ls as libc::c_double;
                (*rv.offset(0 as libc::c_int as isize))
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                (*rv.offset(0 as libc::c_int as isize))
                    .UR
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    + rs as libc::c_double;
            }
            (*rv.offset(0 as libc::c_int as isize))
                .UR
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double;
            *kptr = 1 as libc::c_int;
            break;
        } else {
            i += 1;
        }
    }
    return side;
}
unsafe extern "C" fn gen_fields(
    mut job: *mut GVJ_t,
    mut n: *mut node_t,
    mut f: *mut field_t,
) {
    let mut i: libc::c_int = 0;
    let mut AF: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut coord: pointf = pointf { x: 0., y: 0. };
    if !((*f).lp).is_null() {
        (*(*f).lp)
            .pos = add_pointf(
            mid_pointf((*f).b.LL, (*f).b.UR),
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        );
        emit_label(job, EMIT_NLABEL, (*f).lp);
        penColor(job, n);
    }
    coord = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    i = 0 as libc::c_int;
    while i < (*f).n_flds {
        if i > 0 as libc::c_int {
            if (*f).LR != 0 {
                AF[0 as libc::c_int as usize] = (**((*f).fld).offset(i as isize)).b.LL;
                AF[1 as libc::c_int as usize].x = AF[0 as libc::c_int as usize].x;
                AF[1 as libc::c_int as usize]
                    .y = (**((*f).fld).offset(i as isize)).b.UR.y;
            } else {
                AF[1 as libc::c_int as usize] = (**((*f).fld).offset(i as isize)).b.UR;
                AF[0 as libc::c_int as usize]
                    .x = (**((*f).fld).offset(i as isize)).b.LL.x;
                AF[0 as libc::c_int as usize].y = AF[1 as libc::c_int as usize].y;
            }
            AF[0 as libc::c_int
                as usize] = add_pointf(AF[0 as libc::c_int as usize], coord);
            AF[1 as libc::c_int
                as usize] = add_pointf(AF[1 as libc::c_int as usize], coord);
            gvrender_polyline(job, AF.as_mut_ptr(), 2 as libc::c_int);
        }
        gen_fields(job, n, *((*f).fld).offset(i as isize));
        i += 1;
    }
}
unsafe extern "C" fn record_gencode(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut BF: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut AF: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut style: libc::c_int = 0;
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut doMap: libc::c_int = (!((*obj).url).is_null()
        || (*obj).explicit_tooltip() as libc::c_int != 0) as libc::c_int;
    let mut filled: libc::c_int = 0;
    let mut clrs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    f = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info as *mut field_t;
    BF = (*f).b;
    BF.LL.x += (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
    BF.LL.y += (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
    BF.UR.x += (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
    BF.UR.y += (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
    if doMap != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
    }
    style = stylenode(job, n);
    penColor(job, n);
    clrs[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
    if style & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        let mut fillcolor: *mut libc::c_char = findFill(n);
        let mut frac: libc::c_float = 0.;
        if findStopColor(fillcolor, clrs.as_mut_ptr(), &mut frac) {
            gvrender_set_fillcolor(job, clrs[0 as libc::c_int as usize]);
            if !(clrs[1 as libc::c_int as usize]).is_null() {
                gvrender_set_gradient_vals(
                    job,
                    clrs[1 as libc::c_int as usize],
                    late_int(
                        n as *mut libc::c_void,
                        N_gradientangle,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    ),
                    frac,
                );
            } else {
                gvrender_set_gradient_vals(
                    job,
                    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    late_int(
                        n as *mut libc::c_void,
                        N_gradientangle,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    ),
                    frac,
                );
            }
            if style & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                filled = 3 as libc::c_int;
            } else {
                filled = 2 as libc::c_int;
            }
        } else {
            filled = 1 as libc::c_int;
            gvrender_set_fillcolor(job, fillcolor);
        }
    } else {
        filled = 0 as libc::c_int;
    }
    if strcmp(
        (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
        b"Mrecord\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        style |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    if style
        & ((1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int
            | (127 as libc::c_int) << 24 as libc::c_int) != 0
    {
        AF[0 as libc::c_int as usize] = BF.LL;
        AF[2 as libc::c_int as usize] = BF.UR;
        AF[1 as libc::c_int as usize].x = AF[2 as libc::c_int as usize].x;
        AF[1 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y;
        AF[3 as libc::c_int as usize].x = AF[0 as libc::c_int as usize].x;
        AF[3 as libc::c_int as usize].y = AF[2 as libc::c_int as usize].y;
        round_corners(job, AF.as_mut_ptr(), 4 as libc::c_int, style, filled);
    } else {
        gvrender_box(job, BF, filled);
    }
    gen_fields(job, n, f);
    free(clrs[0 as libc::c_int as usize] as *mut libc::c_void);
    if doMap != 0 {
        if (*job).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            gvrender_begin_anchor(
                job,
                (*obj).url,
                (*obj).tooltip,
                (*obj).target,
                (*obj).id,
            );
        }
        gvrender_end_anchor(job);
    }
}
static mut UserShape: *mut *mut shape_desc = 0 as *const *mut shape_desc
    as *mut *mut shape_desc;
static mut N_UserShape: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn find_user_shape(
    mut name: *const libc::c_char,
) -> *mut shape_desc {
    let mut i: libc::c_int = 0;
    if !UserShape.is_null() {
        i = 0 as libc::c_int;
        while i < N_UserShape {
            if strcmp((**UserShape.offset(i as isize)).name, name) == 0 {
                return *UserShape.offset(i as isize);
            }
            i += 1;
        }
    }
    return 0 as *mut shape_desc;
}
unsafe extern "C" fn user_shape(mut name: *mut libc::c_char) -> *mut shape_desc {
    let mut i: libc::c_int = 0;
    let mut p: *mut shape_desc = 0 as *mut shape_desc;
    p = find_user_shape(name);
    if !p.is_null() {
        return p;
    }
    let fresh44 = N_UserShape;
    N_UserShape = N_UserShape + 1;
    i = fresh44;
    UserShape = if !UserShape.is_null() {
        grealloc(
            UserShape as *mut libc::c_void,
            (N_UserShape as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut shape_desc>() as libc::c_ulong),
        ) as *mut *mut shape_desc
    } else {
        gmalloc(
            (N_UserShape as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut shape_desc>() as libc::c_ulong),
        ) as *mut *mut shape_desc
    };
    let ref mut fresh45 = *UserShape.offset(i as isize);
    *fresh45 = zmalloc(::std::mem::size_of::<shape_desc>() as libc::c_ulong)
        as *mut shape_desc;
    p = *fresh45;
    *p = Shapes[0 as libc::c_int as usize];
    let ref mut fresh46 = (*p).name;
    *fresh46 = strdup(name);
    if Lib.is_null()
        && strcmp(name, b"custom\0" as *const u8 as *const libc::c_char) != 0
    {
        agerr(
            AGWARN,
            b"using %s for unknown shape %s\n\0" as *const u8 as *const libc::c_char,
            Shapes[0 as libc::c_int as usize].name,
            (*p).name,
        );
        (*p).usershape = 0 as libc::c_int != 0;
    } else {
        (*p).usershape = 1 as libc::c_int != 0;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn bind_shape(
    mut name: *mut libc::c_char,
    mut np: *mut node_t,
) -> *mut shape_desc {
    let mut ptr: *mut shape_desc = 0 as *mut shape_desc;
    let mut rv: *mut shape_desc = 0 as *mut shape_desc;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    str = safefile(
        agget(
            np as *mut libc::c_void,
            b"shapefile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    if !str.is_null() && strcmp(name, b"epsf\0" as *const u8 as *const libc::c_char) != 0
    {
        name = b"custom\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if strcmp(name, b"custom\0" as *const u8 as *const libc::c_char) != 0 {
        ptr = Shapes.as_mut_ptr();
        while !((*ptr).name).is_null() {
            if strcmp((*ptr).name, name) == 0 {
                rv = ptr;
                break;
            } else {
                ptr = ptr.offset(1);
            }
        }
    }
    if rv.is_null() {
        rv = user_shape(name);
    }
    return rv;
}
unsafe extern "C" fn epsf_inside(
    mut inside_context: *mut inside_t,
    mut p: pointf,
) -> bool {
    let mut P: pointf = pointf { x: 0., y: 0. };
    let mut x2: libc::c_double = 0.;
    let mut n: *mut node_t = (*inside_context).s.n;
    P = ccwrotatepf(
        p,
        90 as libc::c_int
            * ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int),
    );
    x2 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
        / 2 as libc::c_int as libc::c_double;
    return P.y >= -x2 && P.y <= x2
        && P.x >= -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
        && P.x <= (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
}
unsafe extern "C" fn epsf_gencode(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut desc: *mut epsf_t = 0 as *mut epsf_t;
    let mut doMap: libc::c_int = (!((*obj).url).is_null()
        || (*obj).explicit_tooltip() as libc::c_int != 0) as libc::c_int;
    desc = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
        as *mut epsf_t;
    if desc.is_null() {
        return;
    }
    if doMap != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
    }
    if !desc.is_null() {
        fprintf(
            (*job).output_file,
            b"%.5g %.5g translate newpath user_shape_%d\n\0" as *const u8
                as *const libc::c_char,
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                + (*desc).offset.x as libc::c_double,
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                + (*desc).offset.y as libc::c_double,
            (*desc).macro_id,
        );
    }
    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
        .pos = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    emit_label(
        job,
        EMIT_NLABEL,
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label,
    );
    if doMap != 0 {
        if (*job).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            gvrender_begin_anchor(
                job,
                (*obj).url,
                (*obj).tooltip,
                (*obj).target,
                (*obj).id,
            );
        }
        gvrender_end_anchor(job);
    }
}
unsafe extern "C" fn star_size(mut sz0: pointf) -> pointf {
    let mut sz: pointf = pointf { x: 0., y: 0. };
    let mut r0: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    rx = sz0.x
        / (2 as libc::c_int as libc::c_double
            * cos(3.14159265358979323846f64 / 10.0f64));
    ry = sz0.y
        / (sin(3.14159265358979323846f64 / 10.0f64)
            + sin(
                3 as libc::c_int as libc::c_double
                    * (3.14159265358979323846f64 / 10.0f64),
            ));
    r0 = if rx > ry { rx } else { ry };
    r = r0
        * sin(
            2 as libc::c_int as libc::c_double
                * (2 as libc::c_int as libc::c_double
                    * (3.14159265358979323846f64 / 10.0f64)),
        )
        * cos(2 as libc::c_int as libc::c_double * (3.14159265358979323846f64 / 10.0f64))
        / (cos(3.14159265358979323846f64 / 10.0f64)
            * cos(
                2 as libc::c_int as libc::c_double
                    * (2 as libc::c_int as libc::c_double
                        * (3.14159265358979323846f64 / 10.0f64)),
            ));
    sz
        .x = 2 as libc::c_int as libc::c_double * r
        * cos(3.14159265358979323846f64 / 10.0f64);
    sz
        .y = r
        * (1 as libc::c_int as libc::c_double
            + sin(
                3 as libc::c_int as libc::c_double
                    * (3.14159265358979323846f64 / 10.0f64),
            ));
    return sz;
}
unsafe extern "C" fn star_vertices(mut vertices: *mut pointf, mut bb: *mut pointf) {
    let mut i: libc::c_int = 0;
    let mut sz: pointf = *bb;
    let mut offset: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut aspect: libc::c_double = (1 as libc::c_int as libc::c_double
        + sin(
            3 as libc::c_int as libc::c_double * (3.14159265358979323846f64 / 10.0f64),
        ))
        / (2 as libc::c_int as libc::c_double
            * cos(3.14159265358979323846f64 / 10.0f64));
    let mut r: libc::c_double = 0.;
    let mut r0: libc::c_double = 0.;
    let mut theta: libc::c_double = 3.14159265358979323846f64 / 10.0f64;
    a = sz.y / sz.x;
    if a > aspect {
        sz.x = sz.y / aspect;
    } else if a < aspect {
        sz.y = sz.x * aspect;
    }
    r = sz.x
        / (2 as libc::c_int as libc::c_double
            * cos(3.14159265358979323846f64 / 10.0f64));
    r0 = r * cos(3.14159265358979323846f64 / 10.0f64)
        * cos(
            2 as libc::c_int as libc::c_double
                * (2 as libc::c_int as libc::c_double
                    * (3.14159265358979323846f64 / 10.0f64)),
        )
        / (sin(
            2 as libc::c_int as libc::c_double
                * (2 as libc::c_int as libc::c_double
                    * (3.14159265358979323846f64 / 10.0f64)),
        )
            * cos(
                2 as libc::c_int as libc::c_double
                    * (3.14159265358979323846f64 / 10.0f64),
            ));
    offset = r
        * (1 as libc::c_int as libc::c_double
            - sin(
                3 as libc::c_int as libc::c_double
                    * (3.14159265358979323846f64 / 10.0f64),
            )) / 2 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        (*vertices.offset(i as isize)).x = r * cos(theta);
        (*vertices.offset(i as isize)).y = r * sin(theta) - offset;
        theta
            += 2 as libc::c_int as libc::c_double
                * (3.14159265358979323846f64 / 10.0f64);
        (*vertices.offset((i + 1 as libc::c_int) as isize)).x = r0 * cos(theta);
        (*vertices.offset((i + 1 as libc::c_int) as isize)).y = r0 * sin(theta) - offset;
        theta
            += 2 as libc::c_int as libc::c_double
                * (3.14159265358979323846f64 / 10.0f64);
        i += 2 as libc::c_int;
    }
    *bb = sz;
}
unsafe extern "C" fn star_inside(
    mut inside_context: *mut inside_t,
    mut p: pointf,
) -> bool {
    static mut lastn: *mut node_t = 0 as *const node_t as *mut node_t;
    static mut poly: *mut polygon_t = 0 as *const polygon_t as *mut polygon_t;
    static mut outp: libc::c_int = 0;
    static mut sides: libc::c_int = 0;
    static mut vertex: *mut pointf = 0 as *const pointf as *mut pointf;
    static mut O: pointf = pointf { x: 0., y: 0. };
    if inside_context.is_null() {
        lastn = 0 as *mut node_t;
        return 0 as libc::c_int != 0;
    }
    let mut bp: *mut boxf = (*inside_context).s.bp;
    let mut n: *mut node_t = (*inside_context).s.n;
    let mut P: pointf = pointf { x: 0., y: 0. };
    let mut Q: pointf = pointf { x: 0., y: 0. };
    let mut R: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut outcnt: libc::c_int = 0;
    P = ccwrotatepf(
        p,
        90 as libc::c_int
            * ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int),
    );
    if !bp.is_null() {
        let mut bbox: boxf = *bp;
        return bbox.LL.x <= P.x && P.x <= bbox.UR.x
            && (bbox.LL.y <= P.y && P.y <= bbox.UR.y);
    }
    if n != lastn {
        poly = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
            as *mut polygon_t;
        vertex = (*poly).vertices;
        sides = (*poly).sides;
        outp = ((*poly).peripheries - 1 as libc::c_int) * sides;
        if outp < 0 as libc::c_int {
            outp = 0 as libc::c_int;
        }
        lastn = n;
    }
    outcnt = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < sides {
        Q = *vertex.offset((i + outp) as isize);
        R = *vertex.offset(((i + 4 as libc::c_int) % sides + outp) as isize);
        if same_side(P, O, Q, R) == 0 {
            outcnt += 1;
        }
        if outcnt == 2 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        i += 2 as libc::c_int;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cylinder_size(mut sz: pointf) -> pointf {
    sz.y *= 1.375f64;
    return sz;
}
unsafe extern "C" fn cylinder_vertices(mut vertices: *mut pointf, mut bb: *mut pointf) {
    let mut x: libc::c_double = (*bb).x / 2 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = (*bb).y / 2 as libc::c_int as libc::c_double;
    let mut yr: libc::c_double = (*bb).y / 11 as libc::c_int as libc::c_double;
    (*vertices.offset(0 as libc::c_int as isize)).x = x;
    (*vertices.offset(0 as libc::c_int as isize)).y = y - yr;
    (*vertices.offset(1 as libc::c_int as isize)).x = x;
    (*vertices.offset(1 as libc::c_int as isize))
        .y = y - (1 as libc::c_int as libc::c_double - 0.551784f64) * yr;
    (*vertices.offset(2 as libc::c_int as isize)).x = 0.551784f64 * x;
    (*vertices.offset(2 as libc::c_int as isize)).y = y;
    (*vertices.offset(3 as libc::c_int as isize)).x = 0 as libc::c_int as libc::c_double;
    (*vertices.offset(3 as libc::c_int as isize)).y = y;
    (*vertices.offset(4 as libc::c_int as isize)).x = -0.551784f64 * x;
    (*vertices.offset(4 as libc::c_int as isize)).y = y;
    (*vertices.offset(5 as libc::c_int as isize)).x = -x;
    (*vertices.offset(5 as libc::c_int as isize))
        .y = (*vertices.offset(1 as libc::c_int as isize)).y;
    (*vertices.offset(6 as libc::c_int as isize)).x = -x;
    (*vertices.offset(6 as libc::c_int as isize)).y = y - yr;
    *vertices
        .offset(7 as libc::c_int as isize) = *vertices.offset(6 as libc::c_int as isize);
    (*vertices.offset(8 as libc::c_int as isize)).x = -x;
    (*vertices.offset(8 as libc::c_int as isize)).y = yr - y;
    *vertices
        .offset(9 as libc::c_int as isize) = *vertices.offset(8 as libc::c_int as isize);
    (*vertices.offset(10 as libc::c_int as isize)).x = -x;
    (*vertices.offset(10 as libc::c_int as isize))
        .y = -(*vertices.offset(1 as libc::c_int as isize)).y;
    (*vertices.offset(11 as libc::c_int as isize))
        .x = (*vertices.offset(4 as libc::c_int as isize)).x;
    (*vertices.offset(11 as libc::c_int as isize))
        .y = -(*vertices.offset(4 as libc::c_int as isize)).y;
    (*vertices.offset(12 as libc::c_int as isize))
        .x = (*vertices.offset(3 as libc::c_int as isize)).x;
    (*vertices.offset(12 as libc::c_int as isize))
        .y = -(*vertices.offset(3 as libc::c_int as isize)).y;
    (*vertices.offset(13 as libc::c_int as isize))
        .x = (*vertices.offset(2 as libc::c_int as isize)).x;
    (*vertices.offset(13 as libc::c_int as isize))
        .y = -(*vertices.offset(2 as libc::c_int as isize)).y;
    (*vertices.offset(14 as libc::c_int as isize))
        .x = (*vertices.offset(1 as libc::c_int as isize)).x;
    (*vertices.offset(14 as libc::c_int as isize))
        .y = -(*vertices.offset(1 as libc::c_int as isize)).y;
    (*vertices.offset(15 as libc::c_int as isize))
        .x = (*vertices.offset(0 as libc::c_int as isize)).x;
    (*vertices.offset(15 as libc::c_int as isize))
        .y = -(*vertices.offset(0 as libc::c_int as isize)).y;
    *vertices
        .offset(
            16 as libc::c_int as isize,
        ) = *vertices.offset(15 as libc::c_int as isize);
    let ref mut fresh47 = *vertices.offset(17 as libc::c_int as isize);
    *fresh47 = *vertices.offset(0 as libc::c_int as isize);
    *vertices.offset(18 as libc::c_int as isize) = *fresh47;
}
unsafe extern "C" fn cylinder_draw(
    mut job: *mut GVJ_t,
    mut AF: *mut pointf,
    mut sides: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut vertices: [pointf; 7] = [pointf { x: 0., y: 0. }; 7];
    let mut y0: libc::c_double = (*AF.offset(0 as libc::c_int as isize)).y;
    let mut y02: libc::c_double = y0 + y0;
    vertices[0 as libc::c_int as usize] = *AF.offset(0 as libc::c_int as isize);
    vertices[1 as libc::c_int as usize].x = (*AF.offset(1 as libc::c_int as isize)).x;
    vertices[1 as libc::c_int as usize]
        .y = y02 - (*AF.offset(1 as libc::c_int as isize)).y;
    vertices[2 as libc::c_int as usize].x = (*AF.offset(2 as libc::c_int as isize)).x;
    vertices[2 as libc::c_int as usize]
        .y = y02 - (*AF.offset(2 as libc::c_int as isize)).y;
    vertices[3 as libc::c_int as usize].x = (*AF.offset(3 as libc::c_int as isize)).x;
    vertices[3 as libc::c_int as usize]
        .y = y02 - (*AF.offset(3 as libc::c_int as isize)).y;
    vertices[4 as libc::c_int as usize].x = (*AF.offset(4 as libc::c_int as isize)).x;
    vertices[4 as libc::c_int as usize]
        .y = y02 - (*AF.offset(4 as libc::c_int as isize)).y;
    vertices[5 as libc::c_int as usize].x = (*AF.offset(5 as libc::c_int as isize)).x;
    vertices[5 as libc::c_int as usize]
        .y = y02 - (*AF.offset(5 as libc::c_int as isize)).y;
    vertices[6 as libc::c_int as usize] = *AF.offset(6 as libc::c_int as isize);
    gvrender_beziercurve(job, AF, sides, 0 as libc::c_int, 0 as libc::c_int, filled);
    gvrender_beziercurve(
        job,
        vertices.as_mut_ptr(),
        7 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
static mut side_port: [*mut libc::c_char; 4] = [
    b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn cvtPt(mut p: pointf, mut rankdir: libc::c_int) -> point {
    let mut q: pointf = {
        let mut init = pointf_s {
            x: 0 as libc::c_int as libc::c_double,
            y: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut Q: point = point { x: 0, y: 0 };
    match rankdir {
        0 => {
            q = p;
        }
        2 => {
            q.x = p.x;
            q.y = -p.y;
        }
        1 => {
            q.y = p.x;
            q.x = -p.y;
        }
        3 => {
            q.y = p.x;
            q.x = p.y;
        }
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"shapes.c\0" as *const u8 as *const libc::c_char,
                4091 as libc::c_int,
            );
            abort();
        }
    }
    Q
        .x = (if q.x >= 0 as libc::c_int as libc::c_double {
        (q.x + 0.5f64) as libc::c_int
    } else {
        (q.x - 0.5f64) as libc::c_int
    });
    Q
        .y = (if q.y >= 0 as libc::c_int as libc::c_double {
        (q.y + 0.5f64) as libc::c_int
    } else {
        (q.y - 0.5f64) as libc::c_int
    });
    return Q;
}
unsafe extern "C" fn closestSide(
    mut n: *mut node_t,
    mut other: *mut node_t,
    mut oldport: *mut port,
) -> *mut libc::c_char {
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut rkd: libc::c_int = (*((*((*agraphof(n as *mut libc::c_void)).root
        as *mut Agobj_t))
        .data as *mut Agraphinfo_t))
        .rankdir & 0x3 as libc::c_int;
    let mut p: point = {
        let mut init = point {
            x: 0 as libc::c_int,
            y: 0 as libc::c_int,
        };
        init
    };
    let mut pt: point = cvtPt(
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        rkd,
    );
    let mut opt: point = cvtPt(
        (*((*(other as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        rkd,
    );
    let mut sides: libc::c_int = (*oldport).side as libc::c_int;
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut mind: libc::c_int = 0 as libc::c_int;
    if sides == 0 as libc::c_int
        || sides
            == (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
    {
        return rv;
    }
    if !((*oldport).bp).is_null() {
        b = *(*oldport).bp;
    } else if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
        {
        b
            .UR
            .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
            / 2 as libc::c_int as libc::c_double;
        b.LL.x = -b.UR.x;
        b.UR.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
        b.LL.y = -b.UR.y;
    } else {
        b
            .UR
            .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
            / 2 as libc::c_int as libc::c_double;
        b.LL.y = -b.UR.y;
        b.UR.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
        b.LL.x = -b.UR.x;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !(sides & (1 as libc::c_int) << i == 0 as libc::c_int) {
            match i {
                0 => {
                    p.y = b.LL.y as libc::c_int;
                    p
                        .x = ((b.LL.x + b.UR.x) / 2 as libc::c_int as libc::c_double)
                        as libc::c_int;
                }
                1 => {
                    p.x = b.UR.x as libc::c_int;
                    p
                        .y = ((b.LL.y + b.UR.y) / 2 as libc::c_int as libc::c_double)
                        as libc::c_int;
                }
                2 => {
                    p.y = b.UR.y as libc::c_int;
                    p
                        .x = ((b.LL.x + b.UR.x) / 2 as libc::c_int as libc::c_double)
                        as libc::c_int;
                }
                3 => {
                    p.x = b.LL.x as libc::c_int;
                    p
                        .y = ((b.LL.y + b.UR.y) / 2 as libc::c_int as libc::c_double)
                        as libc::c_int;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s:%d: claimed unreachable code was reached\0" as *const u8
                            as *const libc::c_char,
                        b"shapes.c\0" as *const u8 as *const libc::c_char,
                        4159 as libc::c_int,
                    );
                    abort();
                }
            }
            p.x += pt.x;
            p.y += pt.y;
            d = (p.x - opt.x) * (p.x - opt.x) + (p.y - opt.y) * (p.y - opt.y);
            if rv.is_null() || d < mind {
                mind = d;
                rv = side_port[i as usize];
            }
        }
        i += 1;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn resolvePort(
    mut n: *mut node_t,
    mut other: *mut node_t,
    mut oldport: *mut port,
) -> port {
    let mut rv: port = port {
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
    let mut compass: *mut libc::c_char = closestSide(n, other, oldport);
    rv.name = (*oldport).name;
    compassPort(
        n,
        (*oldport).bp,
        &mut rv,
        compass,
        (*oldport).side as libc::c_int,
        0 as *mut inside_t,
    );
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn resolvePorts(mut e: *mut edge_t) {
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.dyna {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port = resolvePort(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            &mut (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port,
        );
    }
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.dyna {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port = resolvePort(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node,
            &mut (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn gv_initShapes() {
    let mut p: pointf = {
        let mut init = pointf_s {
            x: 0 as libc::c_int as libc::c_double,
            y: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    poly_inside(0 as *mut inside_t, p);
    point_inside(0 as *mut inside_t, p);
    star_inside(0 as *mut inside_t, p);
}
