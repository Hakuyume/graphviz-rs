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
    pub type gvdevice_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    pub type htmllabel_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn agerrorf(fmt: *const libc::c_char, _: ...);
    fn gvputs(job: *mut GVJ_t, s: *const libc::c_char) -> libc::c_int;
    fn gvprintf(job: *mut GVJ_t, format: *const libc::c_char, _: ...);
    fn gdImageCreate(sx: libc::c_int, sy: libc::c_int) -> gdImagePtr;
    fn gdImageDestroy(im: gdImagePtr);
    fn gdImageFilledRectangle(
        im: gdImagePtr,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
        color: libc::c_int,
    );
    fn gdImagePolygon(im: gdImagePtr, p: gdPointPtr, n: libc::c_int, c: libc::c_int);
    fn gdImageFilledPolygon(im: gdImagePtr, p: gdPointPtr, n: libc::c_int, c: libc::c_int);
    fn gdImageColorResolveAlpha(
        im: gdImagePtr,
        r: libc::c_int,
        g: libc::c_int,
        b: libc::c_int,
        a: libc::c_int,
    ) -> libc::c_int;
    fn gdImageColorTransparent(im: gdImagePtr, color: libc::c_int);
    fn gdImagePaletteCopy(dst: gdImagePtr, src: gdImagePtr);
    fn gdImagePng(im: gdImagePtr, out: *mut FILE);
    fn gdImageArc(
        im: gdImagePtr,
        cx: libc::c_int,
        cy: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        s: libc::c_int,
        e: libc::c_int,
        color: libc::c_int,
    );
    fn gdImageFilledEllipse(
        im: gdImagePtr,
        cx: libc::c_int,
        cy: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        color: libc::c_int,
    );
    fn gdImageSetBrush(im: gdImagePtr, brush: gdImagePtr);
    fn gdImageSetStyle(im: gdImagePtr, style: *mut libc::c_int, noOfPixels: libc::c_int);
    fn gdImageSetThickness(im: gdImagePtr, thickness: libc::c_int);
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn Bezier(
        _: *mut pointf,
        _: libc::c_int,
        _: libc::c_double,
        _: *mut pointf,
        _: *mut pointf,
    ) -> pointf;
    fn gvrender_ptf(job: *mut GVJ_t, p: pointf) -> pointf;
    fn shapeOf(_: *mut node_t) -> shape_kind;
    fn wind(a: Ppoint_t, b: Ppoint_t, c: Ppoint_t) -> libc::c_int;
    fn gdgen_text(
        im: gdImagePtr,
        spf: pointf,
        epf: pointf,
        fontcolor: libc::c_int,
        fontsize: libc::c_double,
        fontdpi: libc::c_int,
        fontangle: libc::c_double,
        fontname: *mut libc::c_char,
        str: *mut libc::c_char,
    );
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
    pub begin_job: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_job: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_graph: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_graph: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_layer:
        Option<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char, libc::c_int, libc::c_int) -> ()>,
    pub end_layer: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_page: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_page: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_cluster: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_cluster: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_nodes: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_nodes: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edges: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edges: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_node: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_node: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edge: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edge: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_anchor: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub end_anchor: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_label: Option<unsafe extern "C" fn(*mut GVJ_t, label_type) -> ()>,
    pub end_label: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub textspan: Option<unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> ()>,
    pub resolve_color: Option<unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> ()>,
    pub ellipse: Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> ()>,
    pub polygon:
        Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int, libc::c_int) -> ()>,
    pub beziercurve: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut pointf,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub polyline: Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> ()>,
    pub comment: Option<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> ()>,
    pub library_shape: Option<
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
pub type Ppoint_t = pointf_s;
pub type Agedge_t = Agedge_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
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
pub struct textlabel_t {
    pub text: *mut libc::c_char,
    pub fontname: *mut libc::c_char,
    pub fontcolor: *mut libc::c_char,
    pub charset: libc::c_int,
    pub fontsize: libc::c_double,
    pub dimen: pointf,
    pub space: pointf,
    pub pos: pointf,
    pub u: C2RustUnnamed_6,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub txt: C2RustUnnamed_7,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct elist {
    pub list: *mut *mut edge_t,
    pub size: libc::c_int,
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
pub type gdInterpolationMethod = libc::c_uint;
pub const GD_METHOD_COUNT: gdInterpolationMethod = 30;
pub const GD_WELSH: gdInterpolationMethod = 30;
pub const GD_COSINE: gdInterpolationMethod = 29;
pub const GD_CUBIC_SPLINE: gdInterpolationMethod = 28;
pub const GD_QUADRATIC_BSPLINE: gdInterpolationMethod = 27;
pub const GD_BLACKMAN_SINC: gdInterpolationMethod = 26;
pub const GD_BLACKMAN_BESSEL: gdInterpolationMethod = 25;
pub const GD_LANCZOS8: gdInterpolationMethod = 24;
pub const GD_LANCZOS3: gdInterpolationMethod = 23;
pub const GD_LINEAR: gdInterpolationMethod = 22;
pub const GD_WEIGHTED4: gdInterpolationMethod = 21;
pub const GD_TRIANGLE: gdInterpolationMethod = 20;
pub const GD_SINC: gdInterpolationMethod = 19;
pub const GD_QUADRATIC: gdInterpolationMethod = 18;
pub const GD_POWER: gdInterpolationMethod = 17;
pub const GD_NEAREST_NEIGHBOUR: gdInterpolationMethod = 16;
pub const GD_MITCHELL: gdInterpolationMethod = 15;
pub const GD_HANNING: gdInterpolationMethod = 14;
pub const GD_HAMMING: gdInterpolationMethod = 13;
pub const GD_HERMITE: gdInterpolationMethod = 12;
pub const GD_GENERALIZED_CUBIC: gdInterpolationMethod = 11;
pub const GD_GAUSSIAN: gdInterpolationMethod = 10;
pub const GD_CATMULLROM: gdInterpolationMethod = 9;
pub const GD_BSPLINE: gdInterpolationMethod = 8;
pub const GD_BOX: gdInterpolationMethod = 7;
pub const GD_BLACKMAN: gdInterpolationMethod = 6;
pub const GD_BICUBIC_FIXED: gdInterpolationMethod = 5;
pub const GD_BICUBIC: gdInterpolationMethod = 4;
pub const GD_BILINEAR_FIXED: gdInterpolationMethod = 3;
pub const GD_BESSEL: gdInterpolationMethod = 2;
pub const GD_BELL: gdInterpolationMethod = 1;
pub const GD_DEFAULT: gdInterpolationMethod = 0;
pub type interpolation_method =
    Option<unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdImageStruct {
    pub pixels: *mut *mut libc::c_uchar,
    pub sx: libc::c_int,
    pub sy: libc::c_int,
    pub colorsTotal: libc::c_int,
    pub red: [libc::c_int; 256],
    pub green: [libc::c_int; 256],
    pub blue: [libc::c_int; 256],
    pub open: [libc::c_int; 256],
    pub transparent: libc::c_int,
    pub polyInts: *mut libc::c_int,
    pub polyAllocated: libc::c_int,
    pub brush: *mut gdImageStruct,
    pub tile: *mut gdImageStruct,
    pub brushColorMap: [libc::c_int; 256],
    pub tileColorMap: [libc::c_int; 256],
    pub styleLength: libc::c_int,
    pub stylePos: libc::c_int,
    pub style: *mut libc::c_int,
    pub interlace: libc::c_int,
    pub thick: libc::c_int,
    pub alpha: [libc::c_int; 256],
    pub trueColor: libc::c_int,
    pub tpixels: *mut *mut libc::c_int,
    pub alphaBlendingFlag: libc::c_int,
    pub saveAlphaFlag: libc::c_int,
    pub AA: libc::c_int,
    pub AA_color: libc::c_int,
    pub AA_dont_blend: libc::c_int,
    pub cx1: libc::c_int,
    pub cy1: libc::c_int,
    pub cx2: libc::c_int,
    pub cy2: libc::c_int,
    pub res_x: libc::c_uint,
    pub res_y: libc::c_uint,
    pub paletteQuantizationMethod: libc::c_int,
    pub paletteQuantizationSpeed: libc::c_int,
    pub paletteQuantizationMinQuality: libc::c_int,
    pub paletteQuantizationMaxQuality: libc::c_int,
    pub interpolation_id: gdInterpolationMethod,
    pub interpolation: interpolation_method,
}
pub type gdImage = gdImageStruct;
pub type gdImagePtr = *mut gdImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdPoint {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type gdPointPtr = *mut gdPoint;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const FORMAT_VRML: C2RustUnnamed_8 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t {
    pub Scale: libc::c_double,
    pub MinZ: libc::c_double,
    pub Saw_skycolor: libc::c_int,
    pub im: gdImagePtr,
    pub PNGfile: *mut FILE,
    pub IsSegment: libc::c_int,
    pub CylHt: libc::c_double,
    pub EdgeLen: libc::c_double,
    pub HeadHt: libc::c_double,
    pub TailHt: libc::c_double,
    pub Fstz: libc::c_double,
    pub Sndz: libc::c_double,
}
unsafe extern "C" fn vrml_begin_job(mut job: *mut GVJ_t) {
    let ref mut fresh0 = (*job).context;
    *fresh0 = zmalloc(::std::mem::size_of::<state_t>() as libc::c_ulong);
}
unsafe extern "C" fn vrml_end_job(mut job: *mut GVJ_t) {
    free((*job).context);
}
unsafe extern "C" fn gdirname(mut pathname: *mut libc::c_char) -> *mut libc::c_char {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    last = pathname;
    while *last != 0 {
        last = last.offset(1);
    }
    while last > pathname && {
        last = last.offset(-1);
        *last as libc::c_int == '/' as i32
    } {}
    while last > pathname && *last as libc::c_int != '/' as i32 {
        last = last.offset(-1);
    }
    if last == pathname {
        if *pathname as libc::c_int != '/' as i32 {
            *last = '.' as i32 as libc::c_char;
        } else if *pathname.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            last = last.offset(1);
        }
    } else {
        while *last as libc::c_int == '/' as i32 && last > pathname {
            last = last.offset(-1);
        }
        if last == pathname
            && *pathname as libc::c_int == '/' as i32
            && *pathname.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            last = last.offset(1);
        }
    }
    last = last.offset(1);
    *last = '\0' as i32 as libc::c_char;
    return pathname;
}
unsafe extern "C" fn nodefilename(
    mut filename: *const libc::c_char,
    mut n: *mut node_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut dir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut disposable: [libc::c_char; 1024] = [0; 1024];
    if dir.is_null() {
        if !filename.is_null() {
            dir = gdirname(strcpy(disposable.as_mut_ptr(), filename));
        } else {
            dir = b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    sprintf(
        buf,
        b"%s/node%d.png\0" as *const u8 as *const libc::c_char,
        dir,
        ((*(n as *mut Agobj_t)).tag).seq() as libc::c_int,
    );
    return buf;
}
unsafe extern "C" fn nodefile(mut filename: *const libc::c_char, mut n: *mut node_t) -> *mut FILE {
    let mut rv: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    rv = fopen(
        nodefilename(filename, n, buf.as_mut_ptr()),
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    return rv;
}
unsafe extern "C" fn vrml_node_point(
    mut job: *mut GVJ_t,
    mut n: *mut node_t,
    mut p: pointf,
) -> pointf {
    let mut rv: pointf = pointf { x: 0., y: 0. };
    let mut state: *mut state_t = (*job).context as *mut state_t;
    if (*job).rotation != 0 {
        rv.x = (p.y
            - (*job).pad.y
            - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .y
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw)
            * (*state).Scale
            + 1 as libc::c_int as libc::c_double;
        rv.y = (-(p.x - (*job).pad.x)
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .x
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64)
            * (*state).Scale
            + 1 as libc::c_int as libc::c_double;
    } else {
        rv.x = (p.x
            - (*job).pad.x
            - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .x
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw)
            * (*state).Scale
            + 1 as libc::c_int as libc::c_double;
        rv.y = (-(p.y - (*job).pad.y)
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .y
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64)
            * (*state).Scale
            + 1 as libc::c_int as libc::c_double;
    }
    return rv;
}
unsafe extern "C" fn color_index(mut im: gdImagePtr, mut color: gvcolor_t) -> libc::c_int {
    let mut alpha: libc::c_int = 0;
    alpha = (255 as libc::c_int - color.u.rgba[3 as libc::c_int as usize] as libc::c_int)
        * 127 as libc::c_int
        / 255 as libc::c_int;
    if alpha == 127 as libc::c_int {
        return (*im).transparent;
    } else {
        return gdImageColorResolveAlpha(
            im,
            color.u.rgba[0 as libc::c_int as usize] as libc::c_int,
            color.u.rgba[1 as libc::c_int as usize] as libc::c_int,
            color.u.rgba[2 as libc::c_int as usize] as libc::c_int,
            alpha,
        );
    };
}
unsafe extern "C" fn set_penstyle(
    mut job: *mut GVJ_t,
    mut im: gdImagePtr,
    mut brush: gdImagePtr,
) -> libc::c_int {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut i: libc::c_int = 0;
    let mut pen: libc::c_int = 0;
    let mut pencolor: libc::c_int = 0;
    let mut transparent: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut dashstyle: [libc::c_int; 40] = [0; 40];
    pencolor = color_index(im, (*obj).pencolor);
    pen = pencolor;
    transparent = (*im).transparent;
    if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            dashstyle[i as usize] = pencolor;
            i += 1;
        }
        while i < 40 as libc::c_int {
            dashstyle[i as usize] = transparent;
            i += 1;
        }
        gdImageSetStyle(im, dashstyle.as_mut_ptr(), 20 as libc::c_int);
        pen = -(2 as libc::c_int);
    } else if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            dashstyle[i as usize] = pencolor;
            i += 1;
        }
        while i < 24 as libc::c_int {
            dashstyle[i as usize] = transparent;
            i += 1;
        }
        gdImageSetStyle(im, dashstyle.as_mut_ptr(), 24 as libc::c_int);
        pen = -(2 as libc::c_int);
    }
    width = ((*obj).penwidth * (*job).scale.x) as libc::c_int;
    if (width as libc::c_double) < 1.0f64 {
        width = 1.0f64 as libc::c_int;
    }
    gdImageSetThickness(im, width);
    if width as libc::c_double != 1.0f64 {
        brush = gdImageCreate(width, width);
        gdImagePaletteCopy(brush, im);
        gdImageFilledRectangle(
            brush,
            0 as libc::c_int,
            0 as libc::c_int,
            width - 1 as libc::c_int,
            width - 1 as libc::c_int,
            pencolor,
        );
        gdImageSetBrush(im, brush);
        if pen == -(2 as libc::c_int) {
            pen = -(4 as libc::c_int);
        } else {
            pen = -(3 as libc::c_int);
        }
    }
    return pen;
}
unsafe extern "C" fn vrml_begin_page(mut job: *mut GVJ_t) {
    let mut state: *mut state_t = (*job).context as *mut state_t;
    (*state).Scale = 96 as libc::c_int as libc::c_double / 72 as libc::c_int as libc::c_double;
    gvputs(
        job,
        b"#VRML V2.0 utf8\n\0" as *const u8 as *const libc::c_char,
    );
    (*state).Saw_skycolor = 0 as libc::c_int;
    (*state).MinZ = 1.7976931348623157e+308f64;
    gvputs(
        job,
        b"Group { children [\n  Transform {\n\0" as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"    scale %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        0.0278f64,
        0.0278f64,
        0.0278f64,
    );
    gvputs(
        job,
        b"    children [\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn vrml_end_page(mut job: *mut GVJ_t) {
    let mut d: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut bb: box_0 = (*job).boundingBox;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    d = (if bb.UR.x - bb.LL.x > bb.UR.y - bb.LL.y {
        bb.UR.x - bb.LL.x
    } else {
        bb.UR.y - bb.LL.y
    }) as libc::c_double;
    z = 0.6667f64 * d / tan(3.14159265358979323846f64 / 8.0f64) + (*state).MinZ;
    if (*state).Saw_skycolor == 0 {
        gvputs(
            job,
            b" Background { skyColor 1 1 1 }\n\0" as *const u8 as *const libc::c_char,
        );
    }
    gvputs(job, b"  ] }\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"  Viewpoint {position %.3f %.3f %.3f}\n\0" as *const u8 as *const libc::c_char,
        (*state).Scale * (bb.UR.x + bb.LL.x) as libc::c_double / 72.0f64,
        (*state).Scale * (bb.UR.y + bb.LL.y) as libc::c_double / 72.0f64,
        (*state).Scale * 2 as libc::c_int as libc::c_double * z / 72.0f64,
    );
    gvputs(job, b"] }\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn vrml_begin_node(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut n: *mut node_t = (*obj).u.n;
    let mut z: libc::c_double = (*obj).z;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut transparent: libc::c_int = 0;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    gvprintf(
        job,
        b"# node %s\n\0" as *const u8 as *const libc::c_char,
        agnameof(n as *mut libc::c_void),
    );
    if z < (*state).MinZ {
        (*state).MinZ = z;
    }
    if shapeOf(n) as libc::c_uint != SH_POINT as libc::c_int as libc::c_uint {
        let ref mut fresh1 = (*state).PNGfile;
        *fresh1 = nodefile((*job).output_filename, n);
        if ((*state).PNGfile).is_null() {
            agerrorf(
                b"failed to open file for writing PNG node image\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        width = (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
            * (*state).Scale
            + (2 as libc::c_int * 1 as libc::c_int) as libc::c_double)
            as libc::c_int;
        height = ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht * (*state).Scale
            + (2 as libc::c_int * 1 as libc::c_int) as libc::c_double)
            as libc::c_int;
        let ref mut fresh2 = (*state).im;
        *fresh2 = gdImageCreate(width, height);
        transparent = gdImageColorResolveAlpha(
            (*state).im,
            255 as libc::c_int - 1 as libc::c_int,
            255 as libc::c_int,
            255 as libc::c_int,
            127 as libc::c_int,
        );
        gdImageColorTransparent((*state).im, transparent);
    }
}
unsafe extern "C" fn vrml_end_node(mut job: *mut GVJ_t) {
    let mut state: *mut state_t = (*job).context as *mut state_t;
    if !((*state).im).is_null() {
        if !((*state).PNGfile).is_null() {
            gdImagePng((*state).im, (*state).PNGfile);
            fclose((*state).PNGfile);
        }
        gdImageDestroy((*state).im);
        let ref mut fresh3 = (*state).im;
        *fresh3 = 0 as gdImagePtr;
    }
}
unsafe extern "C" fn vrml_begin_edge(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut e: *mut edge_t = (*obj).u.e;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    (*state).IsSegment = 0 as libc::c_int;
    gvprintf(
        job,
        b"# edge %s -> %s\n\0" as *const u8 as *const libc::c_char,
        agnameof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
            .node as *mut libc::c_void,
        ),
        agnameof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
            .node as *mut libc::c_void,
        ),
    );
    gvputs(
        job,
        b" Group { children [\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn finishSegment(mut job: *mut GVJ_t, mut e: *mut edge_t) {
    let mut p0: pointf = gvrender_ptf(
        job,
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord,
    );
    let mut p1: pointf = gvrender_ptf(
        job,
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord,
    );
    let mut o_x: libc::c_double = 0.;
    let mut o_y: libc::c_double = 0.;
    let mut o_z: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    o_x = (p0.x + p1.x) / 2 as libc::c_int as libc::c_double;
    o_y = (p0.y + p1.y) / 2 as libc::c_int as libc::c_double;
    o_z = ((*state).Fstz + (*state).Sndz) / 2 as libc::c_int as libc::c_double;
    if p0.y > p1.y {
        x = p0.x;
        y = p0.y;
        z = (*state).Fstz;
    } else {
        x = p1.x;
        y = p1.y;
        z = (*state).Sndz;
    }
    x -= o_x;
    y -= o_y;
    z -= o_z;
    if p0.y > p1.y {
        theta = acos(2 as libc::c_int as libc::c_double * y / (*state).EdgeLen)
            + 3.14159265358979323846f64;
    } else {
        theta = acos(2 as libc::c_int as libc::c_double * y / (*state).EdgeLen);
    }
    if x == 0. && z == 0. {
        x = 1 as libc::c_int as libc::c_double;
    }
    y0 = ((*state).HeadHt - (*state).TailHt) / 2.0f64;
    gvputs(job, b"      ]\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"      center 0 %.3f 0\n\0" as *const u8 as *const libc::c_char,
        y0,
    );
    gvprintf(
        job,
        b"      rotation %.3f 0 %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        -z,
        x,
        -theta,
    );
    gvprintf(
        job,
        b"      translation %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        o_x,
        o_y - y0,
        o_z,
    );
    gvputs(job, b"    }\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn vrml_end_edge(mut job: *mut GVJ_t) {
    let mut state: *mut state_t = (*job).context as *mut state_t;
    if (*state).IsSegment != 0 {
        finishSegment(job, (*(*job).obj).u.e);
    }
    gvputs(job, b"] }\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn vrml_textspan(mut job: *mut GVJ_t, mut p: pointf, mut span: *mut textspan_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut spf: pointf = pointf { x: 0., y: 0. };
    let mut epf: pointf = pointf { x: 0., y: 0. };
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut state: *mut state_t = (*job).context as *mut state_t;
    if ((*obj).u.n).is_null() || ((*state).im).is_null() {
        return;
    }
    match (*span).just as libc::c_int {
        108 => {}
        114 => {
            p.x -= (*span).size.x;
        }
        110 | _ => {
            p.x -= (*span).size.x / 2 as libc::c_int as libc::c_double;
        }
    }
    q.x = p.x + (*span).size.x;
    q.y = p.y;
    spf = vrml_node_point(job, (*obj).u.n, p);
    epf = vrml_node_point(job, (*obj).u.n, q);
    gdgen_text(
        (*state).im,
        spf,
        epf,
        color_index((*state).im, (*obj).pencolor),
        (*(*span).font).size,
        96 as libc::c_int,
        if (*job).rotation != 0 {
            3.14159265358979323846f64 / 2 as libc::c_int as libc::c_double
        } else {
            0 as libc::c_int as libc::c_double
        },
        (*(*span).font).name,
        (*span).str_0,
    );
}
unsafe extern "C" fn interpolate_zcoord(
    mut job: *mut GVJ_t,
    mut p1: pointf,
    mut fst: pointf,
    mut fstz: libc::c_double,
    mut snd: pointf,
    mut sndz: libc::c_double,
) -> libc::c_double {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut e: *mut edge_t = (*obj).u.e;
    let mut len: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut rv: libc::c_double = 0.;
    if fstz == sndz {
        return fstz;
    }
    if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    }))
    .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .rank
        != (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank
    {
        if snd.y == fst.y {
            rv = (fstz + sndz) / 2.0f64;
        } else {
            rv = fstz + (sndz - fstz) * (p1.y - fst.y) / (snd.y - fst.y);
        }
    } else {
        len = sqrt((fst.x - snd.x) * (fst.x - snd.x) + (fst.y - snd.y) * (fst.y - snd.y));
        d = sqrt((p1.x - fst.x) * (p1.x - fst.x) + (p1.y - fst.y) * (p1.y - fst.y)) / len;
        rv = fstz + d * (sndz - fstz);
    }
    return rv;
}
unsafe extern "C" fn collinear(mut A: *mut pointf) -> libc::c_int {
    let mut w: libc::c_double = 0.;
    w = wind(
        *A.offset(0 as libc::c_int as isize),
        *A.offset(1 as libc::c_int as isize),
        *A.offset(2 as libc::c_int as isize),
    ) as libc::c_double;
    return (fabs(w) <= 1 as libc::c_int as libc::c_double) as libc::c_int;
}
unsafe extern "C" fn straight(mut A: *mut pointf, mut n: libc::c_int) -> libc::c_int {
    if n != 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (collinear(A) != 0 && collinear(A.offset(1 as libc::c_int as isize)) != 0)
        as libc::c_int;
}
unsafe extern "C" fn doSegment(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut p0: pointf,
    mut z0: libc::c_double,
    mut p1: pointf,
    mut z1: libc::c_double,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut d1: libc::c_double = 0.;
    let mut d0: libc::c_double = 0.;
    let mut delx: libc::c_double = 0.;
    let mut dely: libc::c_double = 0.;
    let mut delz: libc::c_double = 0.;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    delx = p0.x - p1.x;
    dely = p0.y - p1.y;
    delz = z0 - z1;
    (*state).EdgeLen = sqrt(delx * delx + dely * dely + delz * delz);
    d0 = sqrt(
        ((*A.offset(0 as libc::c_int as isize)).x - p0.x)
            * ((*A.offset(0 as libc::c_int as isize)).x - p0.x)
            + ((*A.offset(0 as libc::c_int as isize)).y - p0.y)
                * ((*A.offset(0 as libc::c_int as isize)).y - p0.y),
    );
    d1 = sqrt(
        ((*A.offset(3 as libc::c_int as isize)).x - p1.x)
            * ((*A.offset(3 as libc::c_int as isize)).x - p1.x)
            + ((*A.offset(3 as libc::c_int as isize)).y - p1.y)
                * ((*A.offset(3 as libc::c_int as isize)).y - p1.y),
    );
    (*state).CylHt = (*state).EdgeLen - d0 - d1;
    let ref mut fresh4 = (*state).HeadHt;
    *fresh4 = 0 as libc::c_int as libc::c_double;
    (*state).TailHt = *fresh4;
    (*state).IsSegment = 1 as libc::c_int;
    gvputs(
        job,
        b"Transform {\n  children [\n    Shape {\n      geometry Cylinder {\n        bottom FALSE top FALSE\n\0"
            as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"        height %.3f radius %.3f }\n\0" as *const u8 as *const libc::c_char,
        (*state).CylHt,
        (*obj).penwidth,
    );
    gvputs(
        job,
        b"      appearance Appearance {\n        material Material {\n          ambientIntensity 0.33\n\0"
            as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"          diffuseColor %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        (*obj).pencolor.u.rgba[0 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[1 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[2 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
    );
    gvputs(
        job,
        b"        }\n      }\n    }\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn nearTail(
    mut job: *mut GVJ_t,
    mut a: pointf,
    mut e: *mut Agedge_t,
) -> libc::c_int {
    let mut tp: pointf = gvrender_ptf(
        job,
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord,
    );
    let mut hp: pointf = gvrender_ptf(
        job,
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord,
    );
    return ((a.x - tp.x) * (a.x - tp.x) + (a.y - tp.y) * (a.y - tp.y)
        < (a.x - hp.x) * (a.x - hp.x) + (a.y - hp.y) * (a.y - hp.y)) as libc::c_int;
}
unsafe extern "C" fn vrml_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut e: *mut edge_t = (*obj).u.e;
    let mut fstz: libc::c_double = 0.;
    let mut sndz: libc::c_double = 0.;
    let mut p1: pointf = pointf { x: 0., y: 0. };
    let mut V: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    if !e.is_null() {
    } else {
        __assert_fail(
            b"e\0" as *const u8 as *const libc::c_char,
            b"gvrender_gd_vrml.c\0" as *const u8 as *const libc::c_char,
            495 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"void vrml_bezier(GVJ_t *, pointf *, int, int, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh5 = (*state).Fstz;
    *fresh5 = (*obj).tail_z;
    fstz = *fresh5;
    let ref mut fresh6 = (*state).Sndz;
    *fresh6 = (*obj).head_z;
    sndz = *fresh6;
    if straight(A, n) != 0 {
        doSegment(
            job,
            A,
            gvrender_ptf(
                job,
                (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .coord,
            ),
            (*state).Fstz,
            gvrender_ptf(
                job,
                (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .coord,
            ),
            (*state).Sndz,
        );
        return;
    }
    gvputs(
        job,
        b"Shape { geometry Extrusion  {\n  spine [\0" as *const u8 as *const libc::c_char,
    );
    V[3 as libc::c_int as usize] = *A.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while (i + 3 as libc::c_int) < n {
        V[0 as libc::c_int as usize] = V[3 as libc::c_int as usize];
        j = 1 as libc::c_int;
        while j <= 3 as libc::c_int {
            V[j as usize] = *A.offset((i + j) as isize);
            j += 1;
        }
        step = 0 as libc::c_int;
        while step <= 10 as libc::c_int {
            p1 = Bezier(
                V.as_mut_ptr(),
                3 as libc::c_int,
                step as libc::c_double / 10 as libc::c_int as libc::c_double,
                0 as *mut pointf,
                0 as *mut pointf,
            );
            gvprintf(
                job,
                b" %.3f %.3f %.3f\0" as *const u8 as *const libc::c_char,
                p1.x,
                p1.y,
                interpolate_zcoord(
                    job,
                    p1,
                    *A.offset(0 as libc::c_int as isize),
                    fstz,
                    *A.offset((n - 1 as libc::c_int) as isize),
                    sndz,
                ),
            );
            step += 1;
        }
        i += 3 as libc::c_int;
    }
    gvputs(job, b" ]\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"  crossSection [ %.3f %.3f, %.3f %.3f, %.3f %.3f, %.3f %.3f ]\n\0" as *const u8
            as *const libc::c_char,
        (*obj).penwidth,
        (*obj).penwidth,
        -(*obj).penwidth,
        (*obj).penwidth,
        -(*obj).penwidth,
        -(*obj).penwidth,
        (*obj).penwidth,
        -(*obj).penwidth,
    );
    gvputs(job, b"}\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b" appearance DEF E%ld Appearance {\n\0" as *const u8 as *const libc::c_char,
        ((*(e as *mut Agobj_t)).tag).seq() as libc::c_int,
    );
    gvputs(
        job,
        b"   material Material {\n   ambientIntensity 0.33\n\0" as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"   diffuseColor %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        (*obj).pencolor.u.rgba[0 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[1 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[2 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
    );
    gvputs(job, b"   }\n }\n}\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn doArrowhead(mut job: *mut GVJ_t, mut A: *mut pointf) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut e: *mut edge_t = (*obj).u.e;
    let mut rad: libc::c_double = 0.;
    let mut ht: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut p0: pointf = pointf { x: 0., y: 0. };
    let mut state: *mut state_t = (*job).context as *mut state_t;
    p0.x = ((*A.offset(0 as libc::c_int as isize)).x + (*A.offset(2 as libc::c_int as isize)).x)
        / 2.0f64;
    p0.y = ((*A.offset(0 as libc::c_int as isize)).y + (*A.offset(2 as libc::c_int as isize)).y)
        / 2.0f64;
    rad = sqrt(
        ((*A.offset(0 as libc::c_int as isize)).x - (*A.offset(2 as libc::c_int as isize)).x)
            * ((*A.offset(0 as libc::c_int as isize)).x - (*A.offset(2 as libc::c_int as isize)).x)
            + ((*A.offset(0 as libc::c_int as isize)).y - (*A.offset(2 as libc::c_int as isize)).y)
                * ((*A.offset(0 as libc::c_int as isize)).y
                    - (*A.offset(2 as libc::c_int as isize)).y),
    ) / 2.0f64;
    ht = sqrt(
        (p0.x - (*A.offset(1 as libc::c_int as isize)).x)
            * (p0.x - (*A.offset(1 as libc::c_int as isize)).x)
            + (p0.y - (*A.offset(1 as libc::c_int as isize)).y)
                * (p0.y - (*A.offset(1 as libc::c_int as isize)).y),
    );
    y = ((*state).CylHt + ht) / 2.0f64;
    gvputs(job, b"Transform {\n\0" as *const u8 as *const libc::c_char);
    if nearTail(job, *A.offset(1 as libc::c_int as isize), e) != 0 {
        (*state).TailHt = ht;
        gvprintf(
            job,
            b"  translation 0 %.3f 0\n\0" as *const u8 as *const libc::c_char,
            -y,
        );
        gvprintf(
            job,
            b"  rotation 0 0 1 %.3f\n\0" as *const u8 as *const libc::c_char,
            3.14159265358979323846f64,
        );
    } else {
        (*state).HeadHt = ht;
        gvprintf(
            job,
            b"  translation 0 %.3f 0\n\0" as *const u8 as *const libc::c_char,
            y,
        );
    }
    gvputs(
        job,
        b"  children [\n    Shape {\n\0" as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"      geometry Cone {bottomRadius %.3f height %.3f }\n\0" as *const u8
            as *const libc::c_char,
        rad,
        ht,
    );
    gvputs(
        job,
        b"      appearance Appearance {\n        material Material {\n          ambientIntensity 0.33\n\0"
            as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"          diffuseColor %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        (*obj).pencolor.u.rgba[0 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[1 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[2 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
    );
    gvputs(
        job,
        b"        }\n      }\n    }\n  ]\n}\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn vrml_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut np: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut z: libc::c_double = (*obj).z;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut mp: pointf = pointf { x: 0., y: 0. };
    let mut points: *mut gdPoint = 0 as *mut gdPoint;
    let mut i: libc::c_int = 0;
    let mut pen: libc::c_int = 0;
    let mut brush: gdImagePtr = 0 as gdImagePtr;
    let mut theta: libc::c_double = 0.;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    match (*obj).type_0 as libc::c_uint {
        0 => {
            gvprintf(
                job,
                b" Background { skyColor %.3f %.3f %.3f }\n\0" as *const u8 as *const libc::c_char,
                (*obj).fillcolor.u.rgba[0 as libc::c_int as usize] as libc::c_int as libc::c_double
                    / 255.0f64,
                (*obj).fillcolor.u.rgba[1 as libc::c_int as usize] as libc::c_int as libc::c_double
                    / 255.0f64,
                (*obj).fillcolor.u.rgba[2 as libc::c_int as usize] as libc::c_int as libc::c_double
                    / 255.0f64,
            );
            (*state).Saw_skycolor = (0 as libc::c_int == 0) as libc::c_int;
        }
        2 => {
            n = (*obj).u.n;
            pen = set_penstyle(job, (*state).im, brush);
            points = calloc(
                np as libc::c_ulong,
                ::std::mem::size_of::<gdPoint>() as libc::c_ulong,
            ) as *mut gdPoint;
            i = 0 as libc::c_int;
            while i < np {
                mp = vrml_node_point(job, n, *A.offset(i as isize));
                (*points.offset(i as isize)).x = if mp.x >= 0 as libc::c_int as libc::c_double {
                    (mp.x + 0.5f64) as libc::c_int
                } else {
                    (mp.x - 0.5f64) as libc::c_int
                };
                (*points.offset(i as isize)).y = if mp.y >= 0 as libc::c_int as libc::c_double {
                    (mp.y + 0.5f64) as libc::c_int
                } else {
                    (mp.y - 0.5f64) as libc::c_int
                };
                i += 1;
            }
            if filled != 0 {
                gdImageFilledPolygon(
                    (*state).im,
                    points,
                    np,
                    color_index((*state).im, (*obj).fillcolor),
                );
            }
            gdImagePolygon((*state).im, points, np, pen);
            free(points as *mut libc::c_void);
            if !brush.is_null() {
                gdImageDestroy(brush);
            }
            gvputs(
                job,
                b"Shape {\n  appearance Appearance {\n    material Material {\n      ambientIntensity 0.33\n        diffuseColor 1 1 1\n    }\n\0"
                    as *const u8 as *const libc::c_char,
            );
            gvprintf(
                job,
                b"    texture ImageTexture { url \"node%ld.png\" }\n\0" as *const u8
                    as *const libc::c_char,
                ((*(n as *mut Agobj_t)).tag).seq() as libc::c_int,
            );
            gvputs(
                job,
                b"  }\n  geometry Extrusion {\n    crossSection [\0" as *const u8
                    as *const libc::c_char,
            );
            i = 0 as libc::c_int;
            while i < np {
                p.x = (*A.offset(i as isize)).x
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord
                        .x;
                p.y = (*A.offset(i as isize)).y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord
                        .y;
                gvprintf(
                    job,
                    b" %.3f %.3f,\0" as *const u8 as *const libc::c_char,
                    p.x,
                    p.y,
                );
                i += 1;
            }
            p.x = (*A.offset(0 as libc::c_int as isize)).x
                - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .x;
            p.y = (*A.offset(0 as libc::c_int as isize)).y
                - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .y;
            gvprintf(
                job,
                b" %.3f %.3f ]\n\0" as *const u8 as *const libc::c_char,
                p.x,
                p.y,
            );
            gvprintf(
                job,
                b"    spine [ %.5g %.5g %.5g, %.5g %.5g %.5g ]\n\0" as *const u8
                    as *const libc::c_char,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .x,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .y,
                z - 0.01f64,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .x,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .y,
                z + 0.01f64,
            );
            gvputs(job, b"  }\n}\n\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            e = (*obj).u.e;
            if np != 3 as libc::c_int {
                static mut flag: libc::c_int = 0;
                if flag == 0 {
                    flag += 1;
                    agerr(
                        AGWARN,
                        b"vrml_polygon: non-triangle arrowheads not supported - ignoring\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            if (*state).IsSegment != 0 {
                doArrowhead(job, A);
                return;
            }
            p.y = 0.0f64;
            p.x = p.y;
            i = 0 as libc::c_int;
            while i < np {
                p.x += (*A.offset(i as isize)).x;
                p.y += (*A.offset(i as isize)).y;
                i += 1;
            }
            p.x = p.x / np as libc::c_double;
            p.y = p.y / np as libc::c_double;
            theta = atan2(
                ((*A.offset(0 as libc::c_int as isize)).y
                    + (*A.offset(2 as libc::c_int as isize)).y)
                    / 2.0f64
                    - (*A.offset(1 as libc::c_int as isize)).y,
                ((*A.offset(0 as libc::c_int as isize)).x
                    + (*A.offset(2 as libc::c_int as isize)).x)
                    / 2.0f64
                    - (*A.offset(1 as libc::c_int as isize)).x,
            ) + 3.14159265358979323846f64 / 2.0f64;
            z = if nearTail(job, p, e) != 0 {
                (*obj).tail_z
            } else {
                (*obj).head_z
            };
            gvputs(job, b"Transform {\n\0" as *const u8 as *const libc::c_char);
            gvprintf(
                job,
                b"  translation %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
                p.x,
                p.y,
                z,
            );
            gvputs(
                job,
                b"  children [\n    Transform {\n\0" as *const u8 as *const libc::c_char,
            );
            gvprintf(
                job,
                b"      rotation 0 0 1 %.3f\n\0" as *const u8 as *const libc::c_char,
                theta,
            );
            gvputs(
                job,
                b"      children [\n        Shape {\n\0" as *const u8 as *const libc::c_char,
            );
            gvprintf(
                job,
                b"          geometry Cone {bottomRadius %.3f height %.3f }\n\0" as *const u8
                    as *const libc::c_char,
                (*obj).penwidth * 2.5f64,
                (*obj).penwidth * 10.0f64,
            );
            gvprintf(
                job,
                b"          appearance USE E%ld\n\0" as *const u8 as *const libc::c_char,
                ((*(e as *mut Agobj_t)).tag).seq() as libc::c_int,
            );
            gvputs(
                job,
                b"        }\n      ]\n    }\n  ]\n}\n\0" as *const u8 as *const libc::c_char,
            );
        }
        1 | _ => {}
    };
}
unsafe extern "C" fn doSphere(
    mut job: *mut GVJ_t,
    mut n: *mut node_t,
    mut p: pointf,
    mut z: libc::c_double,
    mut rx: libc::c_double,
    mut ry: libc::c_double,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    gvputs(job, b"Transform {\n\0" as *const u8 as *const libc::c_char);
    gvprintf(
        job,
        b"  translation %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        p.x,
        p.y,
        z,
    );
    gvprintf(
        job,
        b"  scale %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        rx,
        rx,
        rx,
    );
    gvputs(
        job,
        b"  children [\n    Transform {\n      children [\n        Shape {\n          geometry Sphere { radius 1.0 }\n          appearance Appearance {\n            material Material {\n              ambientIntensity 0.33\n\0"
            as *const u8 as *const libc::c_char,
    );
    gvprintf(
        job,
        b"              diffuseColor %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
        (*obj).pencolor.u.rgba[0 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[1 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
        (*obj).pencolor.u.rgba[2 as libc::c_int as usize] as libc::c_int as libc::c_double
            / 255.0f64,
    );
    gvputs(
        job,
        b"            }\n          }\n        }\n      ]\n    }\n  ]\n}\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn vrml_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut z: libc::c_double = (*obj).z;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut npf: pointf = pointf { x: 0., y: 0. };
    let mut nqf: pointf = pointf { x: 0., y: 0. };
    let mut np: point = point { x: 0, y: 0 };
    let mut pen: libc::c_int = 0;
    let mut brush: gdImagePtr = 0 as gdImagePtr;
    let mut state: *mut state_t = (*job).context as *mut state_t;
    rx = (*A.offset(1 as libc::c_int as isize)).x - (*A.offset(0 as libc::c_int as isize)).x;
    ry = (*A.offset(1 as libc::c_int as isize)).y - (*A.offset(0 as libc::c_int as isize)).y;
    match (*obj).type_0 as libc::c_uint {
        2 => {
            n = (*obj).u.n;
            if shapeOf(n) as libc::c_uint == SH_POINT as libc::c_int as libc::c_uint {
                doSphere(job, n, *A.offset(0 as libc::c_int as isize), z, rx, ry);
                return;
            }
            pen = set_penstyle(job, (*state).im, brush);
            npf = vrml_node_point(job, n, *A.offset(0 as libc::c_int as isize));
            nqf = vrml_node_point(job, n, *A.offset(1 as libc::c_int as isize));
            dx = if 2 as libc::c_int as libc::c_double * (nqf.x - npf.x)
                >= 0 as libc::c_int as libc::c_double
            {
                (2 as libc::c_int as libc::c_double * (nqf.x - npf.x) + 0.5f64) as libc::c_int
            } else {
                (2 as libc::c_int as libc::c_double * (nqf.x - npf.x) - 0.5f64) as libc::c_int
            };
            dy = if 2 as libc::c_int as libc::c_double * (nqf.y - npf.y)
                >= 0 as libc::c_int as libc::c_double
            {
                (2 as libc::c_int as libc::c_double * (nqf.y - npf.y) + 0.5f64) as libc::c_int
            } else {
                (2 as libc::c_int as libc::c_double * (nqf.y - npf.y) - 0.5f64) as libc::c_int
            };
            np.x = (if npf.x >= 0 as libc::c_int as libc::c_double {
                (npf.x + 0.5f64) as libc::c_int
            } else {
                (npf.x - 0.5f64) as libc::c_int
            });
            np.y = (if npf.y >= 0 as libc::c_int as libc::c_double {
                (npf.y + 0.5f64) as libc::c_int
            } else {
                (npf.y - 0.5f64) as libc::c_int
            });
            if filled != 0 {
                gdImageFilledEllipse(
                    (*state).im,
                    np.x,
                    np.y,
                    dx,
                    dy,
                    color_index((*state).im, (*obj).fillcolor),
                );
            }
            gdImageArc(
                (*state).im,
                np.x,
                np.y,
                dx,
                dy,
                0 as libc::c_int,
                360 as libc::c_int,
                pen,
            );
            if !brush.is_null() {
                gdImageDestroy(brush);
            }
            gvputs(job, b"Transform {\n\0" as *const u8 as *const libc::c_char);
            gvprintf(
                job,
                b"  translation %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
                (*A.offset(0 as libc::c_int as isize)).x,
                (*A.offset(0 as libc::c_int as isize)).y,
                z,
            );
            gvprintf(
                job,
                b"  scale %.3f %.3f 1\n\0" as *const u8 as *const libc::c_char,
                rx,
                ry,
            );
            gvputs(
                job,
                b"  children [\n    Transform {\n      rotation 1 0 0   1.57\n      children [\n        Shape {\n          geometry Cylinder { side FALSE }\n          appearance Appearance {\n            material Material {\n              ambientIntensity 0.33\n              diffuseColor 1 1 1\n            }\n\0"
                    as *const u8 as *const libc::c_char,
            );
            gvprintf(
                job,
                b"            texture ImageTexture { url \"node%ld.png\" }\n\0" as *const u8
                    as *const libc::c_char,
                ((*(n as *mut Agobj_t)).tag).seq() as libc::c_int,
            );
            gvputs(
                job,
                b"          }\n        }\n      ]\n    }\n  ]\n}\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        3 => {
            e = (*obj).u.e;
            z = if nearTail(job, *A.offset(0 as libc::c_int as isize), e) != 0 {
                (*obj).tail_z
            } else {
                (*obj).head_z
            };
            gvputs(job, b"Transform {\n\0" as *const u8 as *const libc::c_char);
            gvprintf(
                job,
                b"  translation %.3f %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
                (*A.offset(0 as libc::c_int as isize)).x,
                (*A.offset(0 as libc::c_int as isize)).y,
                z,
            );
            gvputs(
                job,
                b"  children [\n    Shape {\n\0" as *const u8 as *const libc::c_char,
            );
            gvprintf(
                job,
                b"      geometry Sphere {radius %.3f }\n\0" as *const u8 as *const libc::c_char,
                rx,
            );
            gvprintf(
                job,
                b"      appearance USE E%d\n\0" as *const u8 as *const libc::c_char,
                ((*(e as *mut Agobj_t)).tag).seq() as libc::c_int,
            );
            gvputs(
                job,
                b"    }\n  ]\n}\n\0" as *const u8 as *const libc::c_char,
            );
        }
        0 | 1 | _ => {}
    };
}
static mut vrml_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: Some(vrml_begin_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_job: Some(vrml_end_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_graph: None,
            end_graph: None,
            begin_layer: None,
            end_layer: None,
            begin_page: Some(vrml_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_page: Some(vrml_end_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_cluster: None,
            end_cluster: None,
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: Some(vrml_begin_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_node: Some(vrml_end_node as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_edge: Some(vrml_begin_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_edge: Some(vrml_end_edge as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_anchor: None,
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                vrml_textspan as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: None,
            ellipse: Some(
                vrml_ellipse as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                vrml_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                vrml_bezier
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            polyline: None,
            comment: None,
            library_shape: None,
        };
        init
    }
};
static mut render_features_vrml: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 24 as libc::c_int,
        default_pad: 0.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: RGBA_BYTE,
    };
    init
};
static mut device_features_vrml: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 9 as libc::c_int | (1 as libc::c_int) << 11 as libc::c_int,
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
                x: 72.0f64,
                y: 72.0f64,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_vrml_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_VRML as libc::c_int,
                type_0: b"vrml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &vrml_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_vrml as *const gvrender_features_t
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
pub static mut gvdevice_vrml_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_VRML as libc::c_int,
                type_0: b"vrml:vrml\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_vrml as *const gvdevice_features_t
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
