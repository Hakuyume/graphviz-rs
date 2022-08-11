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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    static mut stderr: *mut FILE;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
    fn textspan_size(gvc: *mut GVC_t, span: *mut textspan_t) -> pointf;
    fn make_simple_label(gvc: *mut GVC_t, rv: *mut textlabel_t);
    fn round_corners(
        job: *mut GVJ_t,
        AF: *mut pointf,
        sides: libc::c_int,
        style: libc::c_int,
        filled: libc::c_int,
    );
    fn rank(g: *mut graph_t, balance: libc::c_int, maxiter: libc::c_int) -> libc::c_int;
    fn push_obj_state(job: *mut GVJ_t) -> *mut obj_state_t;
    fn pop_obj_state(job: *mut GVJ_t);
    fn initMapData(
        _: *mut GVJ_t,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn emit_map_rect(job: *mut GVJ_t, b: boxf);
    fn getObjId(job: *mut GVJ_t, obj: *mut libc::c_void, xb: *mut agxbuf) -> *mut libc::c_char;
    fn gvrender_usershape(
        job: *mut GVJ_t,
        name: *mut libc::c_char,
        AF: *mut pointf,
        n: libc::c_int,
        filled: bool,
        imagescale: *mut libc::c_char,
        imagepos: *mut libc::c_char,
    );
    fn gvrender_polyline(job: *mut GVJ_t, AF: *mut pointf, n: libc::c_int);
    fn gvrender_box(job: *mut GVJ_t, BF: boxf, filled: libc::c_int);
    fn gvrender_set_style(job: *mut GVJ_t, s: *mut *mut libc::c_char);
    fn gvrender_set_gradient_vals(
        job: *mut GVJ_t,
        stopcolor: *mut libc::c_char,
        angle: libc::c_int,
        frac: libc::c_float,
    );
    fn gvrender_set_fillcolor(job: *mut GVJ_t, name: *mut libc::c_char);
    fn gvrender_set_penwidth(job: *mut GVJ_t, penwidth: libc::c_double);
    fn gvrender_set_pencolor(job: *mut GVJ_t, name: *mut libc::c_char);
    fn gvrender_textspan(job: *mut GVJ_t, p: pointf, span: *mut textspan_t);
    fn gvrender_end_label(job: *mut GVJ_t);
    fn gvrender_begin_label(job: *mut GVJ_t, type_0: label_type);
    fn gvrender_end_anchor(job: *mut GVJ_t);
    fn gvrender_begin_anchor(
        job: *mut GVJ_t,
        href: *mut libc::c_char,
        tooltip: *mut libc::c_char,
        target: *mut libc::c_char,
        id: *mut libc::c_char,
    );
    fn gvusershape_size(g: *mut graph_t, name: *mut libc::c_char) -> point;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn strdup_and_subst_obj(str: *mut libc::c_char, obj: *mut libc::c_void) -> *mut libc::c_char;
    fn findStopColor(
        colorlist: *mut libc::c_char,
        clrs: *mut *mut libc::c_char,
        frac: *mut libc::c_float,
    ) -> bool;
    fn latin1ToUTF8(_: *mut libc::c_char) -> *mut libc::c_char;
    fn htmlEntityUTF8(_: *mut libc::c_char, g: *mut graph_t) -> *mut libc::c_char;
    fn agopen(name: *mut libc::c_char, desc: Agdesc_t, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agnode(g: *mut Agraph_t, name: *mut libc::c_char, createflag: libc::c_int) -> *mut Agnode_t;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agobjkind(_: *mut libc::c_void) -> libc::c_int;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Agstrictdirected: Agdesc_t;
    fn parseHTML(_: *mut libc::c_char, _: *mut libc::c_int, _: *mut htmlenv_t) -> *mut htmllabel_t;
    fn newPS() -> *mut PointSet;
    fn freePS(_: *mut PointSet);
    fn addPS(_: *mut PointSet, _: libc::c_int, _: libc::c_int);
    fn isInPS(_: *mut PointSet, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn openIntSet() -> *mut Dt_t;
    fn addIntSet(_: *mut Dt_t, _: libc::c_int);
    fn inIntSet(_: *mut Dt_t, _: libc::c_int) -> libc::c_int;
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
    pub free_layout: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
    pub cleanup: Option<unsafe extern "C" fn(*mut graph_t) -> ()>,
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
pub type label_type = libc::c_uint;
pub const LABEL_HTML: label_type = 1;
pub const LABEL_PLAIN: label_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pitem {
    pub link: Dtlink_t,
    pub u: C2RustUnnamed_12,
    pub ruled: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub rp: *mut Dt_t,
    pub cp: *mut htmlcell_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlenv_t {
    pub pos: pointf,
    pub finfo: textfont_t,
    pub obj: *mut libc::c_void,
    pub g: *mut graph_t,
    pub imgscale: *mut libc::c_char,
    pub objid: *mut libc::c_char,
    pub objid_set: bool,
}
pub type PointSet = Dict_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlmap_data_t {
    pub url: *mut libc::c_char,
    pub tooltip: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub explicit_tooltip: bool,
    pub LL: point,
    pub UR: point,
}
#[inline]
unsafe extern "C" fn boxfof(
    mut llx: libc::c_double,
    mut lly: libc::c_double,
    mut urx: libc::c_double,
    mut ury: libc::c_double,
) -> boxf {
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    b.LL.x = llx;
    b.LL.y = lly;
    b.UR.x = urx;
    b.UR.y = ury;
    return b;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_calloc(mut nmemb: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong
        && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
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
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
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
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size)) as libc::c_int
        as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(
        ptr,
        old_nmemb.wrapping_mul(size),
        new_nmemb.wrapping_mul(size),
    );
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
        nbuf = gv_calloc(
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            nbuf as *mut libc::c_void,
            (*xb).buf as *const libc::c_void,
            cnt,
        );
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
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {
    } else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"int agxbprint(agxbuf *, const char *, ...)\0",
            ))
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
unsafe extern "C" fn agxbput_n(
    mut xb: *mut agxbuf,
    mut s: *const libc::c_char,
    mut ssz: size_t,
) -> size_t {
    if ((*xb).ptr).offset(ssz as isize) > (*xb).eptr {
        agxbmore(xb, ssz);
    }
    memcpy(
        (*xb).ptr as *mut libc::c_void,
        s as *const libc::c_void,
        ssz,
    );
    let ref mut fresh8 = (*xb).ptr;
    *fresh8 = (*fresh8).offset(ssz as isize);
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
    let ref mut fresh9 = (*xb).ptr;
    let fresh10 = *fresh9;
    *fresh9 = (*fresh9).offset(1);
    *fresh10 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh11 = (*xb).ptr;
    *fresh11 = (*xb).buf;
    return (*xb).ptr;
}
unsafe extern "C" fn pushFontInfo(
    mut env: *mut htmlenv_t,
    mut fp: *mut textfont_t,
    mut savp: *mut textfont_t,
) {
    if !((*env).finfo.name).is_null() {
        if !((*fp).name).is_null() {
            let ref mut fresh12 = (*savp).name;
            *fresh12 = (*env).finfo.name;
            let ref mut fresh13 = (*env).finfo.name;
            *fresh13 = (*fp).name;
        } else {
            let ref mut fresh14 = (*savp).name;
            *fresh14 = 0 as *mut libc::c_char;
        }
    }
    if !((*env).finfo.color).is_null() {
        if !((*fp).color).is_null() {
            let ref mut fresh15 = (*savp).color;
            *fresh15 = (*env).finfo.color;
            let ref mut fresh16 = (*env).finfo.color;
            *fresh16 = (*fp).color;
        } else {
            let ref mut fresh17 = (*savp).color;
            *fresh17 = 0 as *mut libc::c_char;
        }
    }
    if (*env).finfo.size >= 0 as libc::c_int as libc::c_double {
        if (*fp).size >= 0 as libc::c_int as libc::c_double {
            (*savp).size = (*env).finfo.size;
            (*env).finfo.size = (*fp).size;
        } else {
            (*savp).size = -1.0f64;
        }
    }
}
unsafe extern "C" fn popFontInfo(mut env: *mut htmlenv_t, mut savp: *mut textfont_t) {
    if !((*savp).name).is_null() {
        let ref mut fresh18 = (*env).finfo.name;
        *fresh18 = (*savp).name;
    }
    if !((*savp).color).is_null() {
        let ref mut fresh19 = (*env).finfo.color;
        *fresh19 = (*savp).color;
    }
    if (*savp).size >= 0.0f64 {
        (*env).finfo.size = (*savp).size;
    }
}
unsafe extern "C" fn emit_htextspans(
    mut job: *mut GVJ_t,
    mut nspans: libc::c_int,
    mut spans: *mut htextspan_t,
    mut p: pointf,
    mut halfwidth_x: libc::c_double,
    mut finfo: textfont_t,
    mut b: boxf,
    mut simple: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut center_x: libc::c_double = 0.;
    let mut left_x: libc::c_double = 0.;
    let mut right_x: libc::c_double = 0.;
    let mut tl: textspan_t = textspan_t {
        str_0: 0 as *mut libc::c_char,
        font: 0 as *mut textfont_t,
        layout: 0 as *mut libc::c_void,
        free_layout: None,
        yoffset_layout: 0.,
        yoffset_centerline: 0.,
        size: pointf { x: 0., y: 0. },
        just: 0,
    };
    let mut tf: textfont_t = textfont_t {
        name: 0 as *mut libc::c_char,
        color: 0 as *mut libc::c_char,
        postscript_alias: 0 as *mut PostscriptAlias,
        size: 0.,
        flags_cnt: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut p_: pointf = {
        let mut init = pointf_s {
            x: 0.0f64,
            y: 0.0f64,
        };
        init
    };
    let mut ti: *mut textspan_t = 0 as *mut textspan_t;
    center_x = p.x;
    left_x = center_x - halfwidth_x;
    right_x = center_x + halfwidth_x;
    p_.y = p.y + (b.UR.y - b.LL.y) / 2.0f64;
    gvrender_begin_label(job, LABEL_HTML);
    i = 0 as libc::c_int;
    while i < nspans {
        match (*spans.offset(i as isize)).just as libc::c_int {
            108 => {
                p.x = left_x;
            }
            114 => {
                p.x = right_x - (*spans.offset(i as isize)).size;
            }
            110 | _ => {
                p.x = center_x - (*spans.offset(i as isize)).size / 2.0f64;
            }
        }
        p_.y -= (*spans.offset(i as isize)).lfsize;
        ti = (*spans.offset(i as isize)).items;
        j = 0 as libc::c_int;
        while j < (*spans.offset(i as isize)).nitems as libc::c_int {
            if !((*ti).font).is_null() && (*(*ti).font).size > 0 as libc::c_int as libc::c_double {
                tf.size = (*(*ti).font).size;
            } else {
                tf.size = finfo.size;
            }
            if !((*ti).font).is_null() && !((*(*ti).font).name).is_null() {
                tf.name = (*(*ti).font).name;
            } else {
                tf.name = finfo.name;
            }
            if !((*ti).font).is_null() && !((*(*ti).font).color).is_null() {
                tf.color = (*(*ti).font).color;
            } else {
                tf.color = finfo.color;
            }
            if !((*ti).font).is_null() && (*(*ti).font).flags() as libc::c_int != 0 {
                tf.set_flags((*(*ti).font).flags());
            } else {
                tf.set_flags(0 as libc::c_int as libc::c_uint);
            }
            gvrender_set_pencolor(job, tf.color);
            tl.str_0 = (*ti).str_0;
            tl.font = &mut tf;
            tl.yoffset_layout = (*ti).yoffset_layout;
            if simple != 0 {
                tl.yoffset_centerline = (*ti).yoffset_centerline;
            } else {
                tl.yoffset_centerline = 1 as libc::c_int as libc::c_double;
            }
            let ref mut fresh20 = (*tl.font).postscript_alias;
            *fresh20 = (*(*ti).font).postscript_alias;
            tl.layout = (*ti).layout;
            tl.size.x = (*ti).size.x;
            tl.size.y = (*spans.offset(i as isize)).lfsize;
            tl.just = 'l' as i32 as libc::c_char;
            p_.x = p.x;
            gvrender_textspan(job, p_, &mut tl);
            p.x += (*ti).size.x;
            ti = ti.offset(1);
            j += 1;
        }
        i += 1;
    }
    gvrender_end_label(job);
}
unsafe extern "C" fn emit_html_txt(
    mut job: *mut GVJ_t,
    mut tp: *mut htmltxt_t,
    mut env: *mut htmlenv_t,
) {
    let mut halfwidth_x: libc::c_double = 0.;
    let mut p: pointf = pointf { x: 0., y: 0. };
    if ((*tp).nspans as libc::c_int) < 1 as libc::c_int {
        return;
    }
    halfwidth_x = ((*tp).box_0.UR.x - (*tp).box_0.LL.x) / 2.0f64;
    p.x = (*env).pos.x + ((*tp).box_0.UR.x + (*tp).box_0.LL.x) / 2.0f64;
    p.y = (*env).pos.y + ((*tp).box_0.UR.y + (*tp).box_0.LL.y) / 2.0f64;
    emit_htextspans(
        job,
        (*tp).nspans as libc::c_int,
        (*tp).spans,
        p,
        halfwidth_x,
        (*env).finfo,
        (*tp).box_0,
        (*tp).simple as libc::c_int,
    );
}
unsafe extern "C" fn doSide(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut wd: libc::c_double,
    mut ht: libc::c_double,
) {
    let mut BF: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    BF.LL = p;
    BF.UR.x = p.x + wd;
    BF.UR.y = p.y + ht;
    gvrender_box(job, BF, 1 as libc::c_int);
}
unsafe extern "C" fn mkPts(
    mut AF: *mut pointf,
    mut b: boxf,
    mut border: libc::c_int,
) -> *mut pointf {
    *AF.offset(0 as libc::c_int as isize) = b.LL;
    *AF.offset(2 as libc::c_int as isize) = b.UR;
    if border > 1 as libc::c_int {
        let mut delta: libc::c_double = border as libc::c_double / 2.0f64;
        (*AF.offset(0 as libc::c_int as isize)).x += delta;
        (*AF.offset(0 as libc::c_int as isize)).y += delta;
        (*AF.offset(2 as libc::c_int as isize)).x -= delta;
        (*AF.offset(2 as libc::c_int as isize)).y -= delta;
    }
    (*AF.offset(1 as libc::c_int as isize)).x = (*AF.offset(2 as libc::c_int as isize)).x;
    (*AF.offset(1 as libc::c_int as isize)).y = (*AF.offset(0 as libc::c_int as isize)).y;
    (*AF.offset(3 as libc::c_int as isize)).x = (*AF.offset(0 as libc::c_int as isize)).x;
    (*AF.offset(3 as libc::c_int as isize)).y = (*AF.offset(2 as libc::c_int as isize)).y;
    return AF;
}
unsafe extern "C" fn doBorder(mut job: *mut GVJ_t, mut dp: *mut htmldata_t, mut b: boxf) {
    let mut AF: [pointf; 7] = [pointf { x: 0., y: 0. }; 7];
    let mut sptr: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut color: *mut libc::c_char = (if !((*dp).pencolor).is_null() {
        (*dp).pencolor as *const libc::c_char
    } else {
        b"black\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut sides: libc::c_ushort = 0;
    gvrender_set_pencolor(job, color);
    if (*dp).style as libc::c_int
        & ((1 as libc::c_int) << 8 as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int)
        != 0
    {
        sptr[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
        sptr[0 as libc::c_int as usize] = sptr[1 as libc::c_int as usize];
        if (*dp).style as libc::c_int & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            sptr[0 as libc::c_int as usize] =
                b"dashed\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if (*dp).style as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            sptr[0 as libc::c_int as usize] =
                b"dotted\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        gvrender_set_style(job, sptr.as_mut_ptr());
    } else {
        gvrender_set_style(job, (*(*job).gvc).defaultlinestyle);
    }
    gvrender_set_penwidth(job, (*dp).border as libc::c_double);
    if (*dp).style as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        round_corners(
            job,
            mkPts(AF.as_mut_ptr(), b, (*dp).border as libc::c_int),
            4 as libc::c_int,
            (1 as libc::c_int) << 2 as libc::c_int,
            0 as libc::c_int,
        );
    } else {
        sides = ((*dp).flags as libc::c_int
            & ((1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int
                | (1 as libc::c_int) << 12 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int)) as libc::c_ushort;
        if sides != 0 {
            mkPts(
                AF.as_mut_ptr().offset(1 as libc::c_int as isize),
                b,
                (*dp).border as libc::c_int,
            );
            match sides as libc::c_int {
                8192 => {
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(1 as libc::c_int as isize),
                        2 as libc::c_int,
                    );
                }
                4096 => {
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(2 as libc::c_int as isize),
                        2 as libc::c_int,
                    );
                }
                2048 => {
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(3 as libc::c_int as isize),
                        2 as libc::c_int,
                    );
                }
                1024 => {
                    AF[0 as libc::c_int as usize] = AF[4 as libc::c_int as usize];
                    gvrender_polyline(job, AF.as_mut_ptr(), 2 as libc::c_int);
                }
                12288 => {
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(1 as libc::c_int as isize),
                        3 as libc::c_int,
                    );
                }
                6144 => {
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(2 as libc::c_int as isize),
                        3 as libc::c_int,
                    );
                }
                3072 => {
                    AF[5 as libc::c_int as usize] = AF[1 as libc::c_int as usize];
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(3 as libc::c_int as isize),
                        3 as libc::c_int,
                    );
                }
                9216 => {
                    AF[0 as libc::c_int as usize] = AF[4 as libc::c_int as usize];
                    gvrender_polyline(job, AF.as_mut_ptr(), 3 as libc::c_int);
                }
                14336 => {
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(1 as libc::c_int as isize),
                        4 as libc::c_int,
                    );
                }
                7168 => {
                    AF[5 as libc::c_int as usize] = AF[1 as libc::c_int as usize];
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(2 as libc::c_int as isize),
                        4 as libc::c_int,
                    );
                }
                11264 => {
                    AF[5 as libc::c_int as usize] = AF[1 as libc::c_int as usize];
                    AF[6 as libc::c_int as usize] = AF[2 as libc::c_int as usize];
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(3 as libc::c_int as isize),
                        4 as libc::c_int,
                    );
                }
                13312 => {
                    AF[0 as libc::c_int as usize] = AF[4 as libc::c_int as usize];
                    gvrender_polyline(job, AF.as_mut_ptr(), 4 as libc::c_int);
                }
                10240 => {
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(1 as libc::c_int as isize),
                        2 as libc::c_int,
                    );
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(3 as libc::c_int as isize),
                        2 as libc::c_int,
                    );
                }
                5120 => {
                    AF[0 as libc::c_int as usize] = AF[4 as libc::c_int as usize];
                    gvrender_polyline(job, AF.as_mut_ptr(), 2 as libc::c_int);
                    gvrender_polyline(
                        job,
                        AF.as_mut_ptr().offset(2 as libc::c_int as isize),
                        2 as libc::c_int,
                    );
                }
                _ => {}
            }
        } else {
            if (*dp).border as libc::c_int > 1 as libc::c_int {
                let mut delta: libc::c_double = (*dp).border as libc::c_double / 2.0f64;
                b.LL.x += delta;
                b.LL.y += delta;
                b.UR.x -= delta;
                b.UR.y -= delta;
            }
            gvrender_box(job, b, 0 as libc::c_int);
        }
    };
}
unsafe extern "C" fn setFill(
    mut job: *mut GVJ_t,
    mut color: *mut libc::c_char,
    mut angle: libc::c_int,
    mut style: libc::c_int,
    mut clrs: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut filled: libc::c_int = 0;
    let mut frac: libc::c_float = 0.;
    if findStopColor(color, clrs, &mut frac) {
        gvrender_set_fillcolor(job, *clrs.offset(0 as libc::c_int as isize));
        if !(*clrs.offset(1 as libc::c_int as isize)).is_null() {
            gvrender_set_gradient_vals(job, *clrs.offset(1 as libc::c_int as isize), angle, frac);
        } else {
            gvrender_set_gradient_vals(
                job,
                b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                angle,
                frac,
            );
        }
        if style & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            filled = 3 as libc::c_int;
        } else {
            filled = 2 as libc::c_int;
        }
    } else {
        gvrender_set_fillcolor(job, color);
        filled = 1 as libc::c_int;
    }
    gvrender_set_pencolor(
        job,
        b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return filled;
}
unsafe extern "C" fn initAnchor(
    mut job: *mut GVJ_t,
    mut env: *mut htmlenv_t,
    mut data: *mut htmldata_t,
    mut b: boxf,
    mut save: *mut htmlmap_data_t,
) -> libc::c_int {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut changed: libc::c_int = 0;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut anchorId: libc::c_int = 0;
    let mut internalId: libc::c_int = 0 as libc::c_int;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut buf: [libc::c_char; 128] = [0; 128];
    let ref mut fresh21 = (*save).url;
    *fresh21 = (*obj).url;
    let ref mut fresh22 = (*save).tooltip;
    *fresh22 = (*obj).tooltip;
    let ref mut fresh23 = (*save).target;
    *fresh23 = (*obj).target;
    let ref mut fresh24 = (*save).id;
    *fresh24 = (*obj).id;
    (*save).explicit_tooltip = (*obj).explicit_tooltip() as libc::c_int != 0 as libc::c_int;
    id = (*data).id;
    if id.is_null() || *id == 0 {
        agxbinit(
            &mut xb,
            128 as libc::c_int as libc::c_uint,
            buf.as_mut_ptr(),
        );
        if ((*env).objid).is_null() {
            let ref mut fresh25 = (*env).objid;
            *fresh25 = strdup(getObjId(job, (*obj).u.n as *mut libc::c_void, &mut xb));
            (*env).objid_set = 1 as libc::c_int != 0;
        }
        let fresh26 = anchorId;
        anchorId = anchorId + 1;
        agxbprint(
            &mut xb as *mut agxbuf,
            b"%s_%d\0" as *const u8 as *const libc::c_char,
            (*env).objid,
            fresh26,
        );
        id = agxbuse(&mut xb);
        internalId = 1 as libc::c_int;
    }
    changed = initMapData(
        job,
        0 as *mut libc::c_char,
        (*data).href,
        (*data).title,
        (*data).target,
        id,
        (*obj).u.g as *mut libc::c_void,
    );
    if internalId != 0 {
        agxbfree(&mut xb);
    }
    if changed != 0 {
        if !((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0 {
            emit_map_rect(job, b);
            gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
        }
    }
    return changed;
}
unsafe extern "C" fn endAnchor(mut job: *mut GVJ_t, mut save: *mut htmlmap_data_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    if !((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0 {
        gvrender_end_anchor(job);
    }
    if (*obj).url != (*save).url {
        free((*obj).url as *mut libc::c_void);
        let ref mut fresh27 = (*obj).url;
        *fresh27 = (*save).url;
    }
    if (*obj).tooltip != (*save).tooltip {
        free((*obj).tooltip as *mut libc::c_void);
        let ref mut fresh28 = (*obj).tooltip;
        *fresh28 = (*save).tooltip;
    }
    if (*obj).target != (*save).target {
        free((*obj).target as *mut libc::c_void);
        let ref mut fresh29 = (*obj).target;
        *fresh29 = (*save).target;
    }
    if (*obj).id != (*save).id {
        free((*obj).id as *mut libc::c_void);
        let ref mut fresh30 = (*obj).id;
        *fresh30 = (*save).id;
    }
    (*obj).set_explicit_tooltip((*save).explicit_tooltip as libc::c_uint);
}
unsafe extern "C" fn emit_html_rules(
    mut job: *mut GVJ_t,
    mut cp: *mut htmlcell_t,
    mut env: *mut htmlenv_t,
    mut color: *mut libc::c_char,
    mut nextc: *mut htmlcell_t,
) {
    let mut rule_pt: pointf = pointf { x: 0., y: 0. };
    let mut rule_length: libc::c_double = 0.;
    let mut base: libc::c_uchar = 0;
    let mut pts: boxf = (*cp).data.box_0;
    let mut pos: pointf = (*env).pos;
    if color.is_null() {
        color = b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    gvrender_set_fillcolor(job, color);
    gvrender_set_pencolor(job, color);
    pts = (*cp).data.box_0;
    pts.LL.x += pos.x;
    pts.UR.x += pos.x;
    pts.LL.y += pos.y;
    pts.UR.y += pos.y;
    if (*cp).ruled as libc::c_int & 1 as libc::c_int != 0
        && ((*cp).col as libc::c_int + (*cp).cspan as libc::c_int) < (*(*cp).parent).cc
    {
        if (*cp).row as libc::c_int == 0 as libc::c_int {
            base = ((*(*cp).parent).data.border as libc::c_int
                + (*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int)
                as libc::c_uchar;
            rule_pt.y = pts.LL.y
                - ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double;
        } else if (*cp).row as libc::c_int + (*cp).rspan as libc::c_int == (*(*cp).parent).rc {
            base = ((*(*cp).parent).data.border as libc::c_int
                + (*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int)
                as libc::c_uchar;
            rule_pt.y = pts.LL.y
                - ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double
                - base as libc::c_int as libc::c_double;
        } else {
            base = 0 as libc::c_int as libc::c_uchar;
            rule_pt.y = pts.LL.y
                - ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double;
        }
        rule_pt.x = pts.UR.x
            + ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double;
        rule_length = base as libc::c_int as libc::c_double + pts.UR.y - pts.LL.y
            + (*(*cp).parent).data.space as libc::c_int as libc::c_double;
        doSide(
            job,
            rule_pt,
            0 as libc::c_int as libc::c_double,
            rule_length,
        );
    }
    if (*cp).ruled as libc::c_int & 2 as libc::c_int != 0
        && ((*cp).row as libc::c_int + (*cp).rspan as libc::c_int) < (*(*cp).parent).rc
    {
        if (*cp).col as libc::c_int == 0 as libc::c_int {
            base = ((*(*cp).parent).data.border as libc::c_int
                + (*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int)
                as libc::c_uchar;
            rule_pt.x = pts.LL.x
                - base as libc::c_int as libc::c_double
                - ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double;
            if (*cp).col as libc::c_int + (*cp).cspan as libc::c_int == (*(*cp).parent).cc {
                base = (base as libc::c_int * 2 as libc::c_int) as libc::c_uchar;
            } else if !nextc.is_null() && (*nextc).row as libc::c_int != (*cp).row as libc::c_int {
                base = (base as libc::c_double
                    + ((*(*cp).parent).data.box_0.UR.x + pos.x
                        - (pts.UR.x
                            + ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int)
                                as libc::c_double))) as libc::c_uchar;
            }
        } else if (*cp).col as libc::c_int + (*cp).cspan as libc::c_int == (*(*cp).parent).cc {
            base = ((*(*cp).parent).data.border as libc::c_int
                + (*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int)
                as libc::c_uchar;
            rule_pt.x = pts.LL.x
                - ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double;
        } else {
            base = 0 as libc::c_int as libc::c_uchar;
            rule_pt.x = pts.LL.x
                - ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double;
            if !nextc.is_null() && (*nextc).row as libc::c_int != (*cp).row as libc::c_int {
                base = (base as libc::c_double
                    + ((*(*cp).parent).data.box_0.UR.x + pos.x
                        - (pts.UR.x
                            + ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int)
                                as libc::c_double))) as libc::c_uchar;
            }
        }
        rule_pt.y = pts.LL.y
            - ((*(*cp).parent).data.space as libc::c_int / 2 as libc::c_int) as libc::c_double;
        rule_length = base as libc::c_int as libc::c_double + pts.UR.x - pts.LL.x
            + (*(*cp).parent).data.space as libc::c_int as libc::c_double;
        doSide(
            job,
            rule_pt,
            rule_length,
            0 as libc::c_int as libc::c_double,
        );
    }
}
unsafe extern "C" fn emit_html_tbl(
    mut job: *mut GVJ_t,
    mut tbl: *mut htmltbl_t,
    mut env: *mut htmlenv_t,
) {
    let mut pts: boxf = (*tbl).data.box_0;
    let mut pos: pointf = (*env).pos;
    let mut cells: *mut *mut htmlcell_t = (*tbl).u.n.cells;
    let mut cp: *mut htmlcell_t = 0 as *mut htmlcell_t;
    static mut savef: textfont_t = textfont_t {
        name: 0 as *mut libc::c_char,
        color: 0 as *mut libc::c_char,
        postscript_alias: 0 as *mut PostscriptAlias,
        size: 0.,
        flags_cnt: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut saved: htmlmap_data_t = htmlmap_data_t {
        url: 0 as *mut libc::c_char,
        tooltip: 0 as *mut libc::c_char,
        target: 0 as *mut libc::c_char,
        id: 0 as *mut libc::c_char,
        explicit_tooltip: false,
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    let mut anchor: libc::c_int = 0;
    let mut doAnchor: libc::c_int =
        (!((*tbl).data.href).is_null() || !((*tbl).data.target).is_null()) as libc::c_int;
    let mut AF: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    if !((*tbl).font).is_null() {
        pushFontInfo(env, (*tbl).font, &mut savef);
    }
    pts.LL.x += pos.x;
    pts.UR.x += pos.x;
    pts.LL.y += pos.y;
    pts.UR.y += pos.y;
    if doAnchor != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        anchor = initAnchor(job, env, &mut (*tbl).data, pts, &mut saved);
    } else {
        anchor = 0 as libc::c_int;
    }
    if (*tbl).data.style as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        if !((*tbl).data.bgcolor).is_null() {
            let mut clrs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
            let mut filled: libc::c_int = setFill(
                job,
                (*tbl).data.bgcolor,
                (*tbl).data.gradientangle,
                (*tbl).data.style as libc::c_int,
                clrs.as_mut_ptr(),
            );
            if (*tbl).data.style as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                round_corners(
                    job,
                    mkPts(AF.as_mut_ptr(), pts, (*tbl).data.border as libc::c_int),
                    4 as libc::c_int,
                    (1 as libc::c_int) << 2 as libc::c_int,
                    filled,
                );
            } else {
                gvrender_box(job, pts, filled);
            }
            free(clrs[0 as libc::c_int as usize] as *mut libc::c_void);
        }
        while !(*cells).is_null() {
            emit_html_cell(job, *cells, env);
            cells = cells.offset(1);
        }
        cells = (*tbl).u.n.cells;
        gvrender_set_penwidth(job, 1.0f64);
        loop {
            let fresh31 = cells;
            cells = cells.offset(1);
            cp = *fresh31;
            if cp.is_null() {
                break;
            }
            if (*cp).ruled != 0 {
                emit_html_rules(job, cp, env, (*tbl).data.pencolor, *cells);
            }
        }
        if (*tbl).data.border != 0 {
            doBorder(job, &mut (*tbl).data, pts);
        }
    }
    if anchor != 0 {
        endAnchor(job, &mut saved);
    }
    if doAnchor != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        if initAnchor(job, env, &mut (*tbl).data, pts, &mut saved) != 0 {
            endAnchor(job, &mut saved);
        }
    }
    if !((*tbl).font).is_null() {
        popFontInfo(env, &mut savef);
    }
}
unsafe extern "C" fn emit_html_img(
    mut job: *mut GVJ_t,
    mut cp: *mut htmlimg_t,
    mut env: *mut htmlenv_t,
) {
    let mut A: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut bb: boxf = (*cp).box_0;
    let mut scale: *mut libc::c_char = 0 as *mut libc::c_char;
    bb.LL.x += (*env).pos.x;
    bb.LL.y += (*env).pos.y;
    bb.UR.x += (*env).pos.x;
    bb.UR.y += (*env).pos.y;
    A[0 as libc::c_int as usize] = bb.UR;
    A[2 as libc::c_int as usize] = bb.LL;
    A[1 as libc::c_int as usize].x = A[2 as libc::c_int as usize].x;
    A[1 as libc::c_int as usize].y = A[0 as libc::c_int as usize].y;
    A[3 as libc::c_int as usize].x = A[0 as libc::c_int as usize].x;
    A[3 as libc::c_int as usize].y = A[2 as libc::c_int as usize].y;
    if !((*cp).scale).is_null() {
        scale = (*cp).scale;
    } else {
        scale = (*env).imgscale;
    }
    if !((*cp).src).is_null() {
    } else {
        __assert_fail(
            b"cp->src\0" as *const u8 as *const libc::c_char,
            b"htmltable.c\0" as *const u8 as *const libc::c_char,
            622 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"void emit_html_img(GVJ_t *, htmlimg_t *, htmlenv_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if *((*cp).src).offset(0 as libc::c_int as isize) != 0 {
    } else {
        __assert_fail(
            b"cp->src[0]\0" as *const u8 as *const libc::c_char,
            b"htmltable.c\0" as *const u8 as *const libc::c_char,
            623 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"void emit_html_img(GVJ_t *, htmlimg_t *, htmlenv_t *)\0",
            ))
            .as_ptr(),
        );
    }
    gvrender_usershape(
        job,
        (*cp).src,
        A.as_mut_ptr(),
        4 as libc::c_int,
        1 as libc::c_int != 0,
        scale,
        b"mc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn emit_html_cell(
    mut job: *mut GVJ_t,
    mut cp: *mut htmlcell_t,
    mut env: *mut htmlenv_t,
) {
    let mut saved: htmlmap_data_t = htmlmap_data_t {
        url: 0 as *mut libc::c_char,
        tooltip: 0 as *mut libc::c_char,
        target: 0 as *mut libc::c_char,
        id: 0 as *mut libc::c_char,
        explicit_tooltip: false,
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    let mut pts: boxf = (*cp).data.box_0;
    let mut pos: pointf = (*env).pos;
    let mut inAnchor: libc::c_int = 0;
    let mut doAnchor: libc::c_int =
        (!((*cp).data.href).is_null() || !((*cp).data.target).is_null()) as libc::c_int;
    let mut AF: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    pts.LL.x += pos.x;
    pts.UR.x += pos.x;
    pts.LL.y += pos.y;
    pts.UR.y += pos.y;
    if doAnchor != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        inAnchor = initAnchor(job, env, &mut (*cp).data, pts, &mut saved);
    } else {
        inAnchor = 0 as libc::c_int;
    }
    if (*cp).data.style as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        if !((*cp).data.bgcolor).is_null() {
            let mut clrs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
            let mut filled: libc::c_int = setFill(
                job,
                (*cp).data.bgcolor,
                (*cp).data.gradientangle,
                (*cp).data.style as libc::c_int,
                clrs.as_mut_ptr(),
            );
            if (*cp).data.style as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                round_corners(
                    job,
                    mkPts(AF.as_mut_ptr(), pts, (*cp).data.border as libc::c_int),
                    4 as libc::c_int,
                    (1 as libc::c_int) << 2 as libc::c_int,
                    filled,
                );
            } else {
                gvrender_box(job, pts, filled);
            }
            free(clrs[0 as libc::c_int as usize] as *mut libc::c_void);
        }
        if (*cp).data.border != 0 {
            doBorder(job, &mut (*cp).data, pts);
        }
        if (*cp).child.kind as libc::c_int == 1 as libc::c_int {
            emit_html_tbl(job, (*cp).child.u.tbl, env);
        } else if (*cp).child.kind as libc::c_int == 3 as libc::c_int {
            emit_html_img(job, (*cp).child.u.img, env);
        } else {
            emit_html_txt(job, (*cp).child.u.txt, env);
        }
    }
    if inAnchor != 0 {
        endAnchor(job, &mut saved);
    }
    if doAnchor != 0 && (*job).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        if initAnchor(job, env, &mut (*cp).data, pts, &mut saved) != 0 {
            endAnchor(job, &mut saved);
        }
    }
}
unsafe extern "C" fn allocObj(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    let mut parent: *mut obj_state_t = 0 as *mut obj_state_t;
    obj = push_obj_state(job);
    parent = (*obj).parent;
    (*obj).type_0 = (*parent).type_0;
    (*obj).emit_state = (*parent).emit_state;
    match (*obj).type_0 as libc::c_uint {
        2 => {
            let ref mut fresh32 = (*obj).u.n;
            *fresh32 = (*parent).u.n;
        }
        0 => {
            let ref mut fresh33 = (*obj).u.g;
            *fresh33 = (*parent).u.g;
        }
        1 => {
            let ref mut fresh34 = (*obj).u.sg;
            *fresh34 = (*parent).u.sg;
        }
        3 => {
            let ref mut fresh35 = (*obj).u.e;
            *fresh35 = (*parent).u.e;
        }
        _ => {}
    }
    let ref mut fresh36 = (*obj).url;
    *fresh36 = (*parent).url;
    let ref mut fresh37 = (*obj).tooltip;
    *fresh37 = (*parent).tooltip;
    let ref mut fresh38 = (*obj).target;
    *fresh38 = (*parent).target;
    (*obj).set_explicit_tooltip((*parent).explicit_tooltip());
}
unsafe extern "C" fn freeObj(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let ref mut fresh39 = (*obj).url;
    *fresh39 = 0 as *mut libc::c_char;
    let ref mut fresh40 = (*obj).tooltip;
    *fresh40 = 0 as *mut libc::c_char;
    let ref mut fresh41 = (*obj).target;
    *fresh41 = 0 as *mut libc::c_char;
    let ref mut fresh42 = (*obj).id;
    *fresh42 = 0 as *mut libc::c_char;
    pop_obj_state(job);
}
unsafe extern "C" fn heightOfLbl(mut lp: *mut htmllabel_t) -> libc::c_double {
    let mut sz: libc::c_double = 0.0f64;
    match (*lp).kind as libc::c_int {
        1 => {
            sz = (*(*lp).u.tbl).data.box_0.UR.y - (*(*lp).u.tbl).data.box_0.LL.y;
        }
        3 => {
            sz = (*(*lp).u.img).box_0.UR.y - (*(*lp).u.img).box_0.LL.y;
        }
        2 => {
            sz = (*(*lp).u.txt).box_0.UR.y - (*(*lp).u.txt).box_0.LL.y;
        }
        _ => {}
    }
    return sz;
}
#[no_mangle]
pub unsafe extern "C" fn emit_html_label(
    mut job: *mut GVJ_t,
    mut lp: *mut htmllabel_t,
    mut tp: *mut textlabel_t,
) {
    let mut env: htmlenv_t = htmlenv_t {
        pos: pointf { x: 0., y: 0. },
        finfo: textfont_t {
            name: 0 as *mut libc::c_char,
            color: 0 as *mut libc::c_char,
            postscript_alias: 0 as *mut PostscriptAlias,
            size: 0.,
            flags_cnt: [0; 4],
            c2rust_padding: [0; 4],
        },
        obj: 0 as *mut libc::c_void,
        g: 0 as *mut graph_t,
        imgscale: 0 as *mut libc::c_char,
        objid: 0 as *mut libc::c_char,
        objid_set: false,
    };
    let mut p: pointf = pointf { x: 0., y: 0. };
    allocObj(job);
    p = (*tp).pos;
    match (*tp).valign as libc::c_int {
        116 => {
            p.y = (*tp).pos.y + ((*tp).space.y - heightOfLbl(lp)) / 2.0f64
                - 1 as libc::c_int as libc::c_double;
        }
        98 => {
            p.y = (*tp).pos.y
                - ((*tp).space.y - heightOfLbl(lp)) / 2.0f64
                - 1 as libc::c_int as libc::c_double;
        }
        _ => {}
    }
    env.pos = p;
    env.finfo.color = (*tp).fontcolor;
    env.finfo.name = (*tp).fontname;
    env.finfo.size = (*tp).fontsize;
    env.imgscale = agget(
        (*(*job).obj).u.n as *mut libc::c_void,
        b"imagescale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    env.objid = (*(*job).obj).id;
    env.objid_set = 0 as libc::c_int != 0;
    if (env.imgscale).is_null()
        || *(env.imgscale).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        env.imgscale = b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*lp).kind as libc::c_int == 1 as libc::c_int {
        let mut tbl: *mut htmltbl_t = (*lp).u.tbl;
        gvrender_set_style(job, (*(*job).gvc).defaultlinestyle);
        if !((*tbl).data.pencolor).is_null() {
            gvrender_set_pencolor(job, (*tbl).data.pencolor);
        } else {
            gvrender_set_pencolor(
                job,
                b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        emit_html_tbl(job, tbl, &mut env);
    } else {
        emit_html_txt(job, (*lp).u.txt, &mut env);
    }
    if env.objid_set {
        free(env.objid as *mut libc::c_void);
    }
    freeObj(job);
}
#[no_mangle]
pub unsafe extern "C" fn free_html_data(mut dp: *mut htmldata_t) {
    free((*dp).href as *mut libc::c_void);
    free((*dp).port as *mut libc::c_void);
    free((*dp).target as *mut libc::c_void);
    free((*dp).id as *mut libc::c_void);
    free((*dp).title as *mut libc::c_void);
    free((*dp).bgcolor as *mut libc::c_void);
    free((*dp).pencolor as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_html_text(mut t: *mut htmltxt_t) {
    let mut tl: *mut htextspan_t = 0 as *mut htextspan_t;
    let mut ti: *mut textspan_t = 0 as *mut textspan_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if t.is_null() {
        return;
    }
    tl = (*t).spans;
    i = 0 as libc::c_int;
    while i < (*t).nspans as libc::c_int {
        ti = (*tl).items;
        j = 0 as libc::c_int;
        while j < (*tl).nitems as libc::c_int {
            free((*ti).str_0 as *mut libc::c_void);
            if !((*ti).layout).is_null() && ((*ti).free_layout).is_some() {
                ((*ti).free_layout).expect("non-null function pointer")((*ti).layout);
            }
            ti = ti.offset(1);
            j += 1;
        }
        tl = tl.offset(1);
        i += 1;
    }
    free((*t).spans as *mut libc::c_void);
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn free_html_img(mut ip: *mut htmlimg_t) {
    free((*ip).src as *mut libc::c_void);
    free(ip as *mut libc::c_void);
}
unsafe extern "C" fn free_html_cell(mut cp: *mut htmlcell_t) {
    free_html_label(&mut (*cp).child, 0 as libc::c_int);
    free_html_data(&mut (*cp).data);
    free(cp as *mut libc::c_void);
}
unsafe extern "C" fn free_html_tbl(mut tbl: *mut htmltbl_t) {
    let mut cells: *mut *mut htmlcell_t = 0 as *mut *mut htmlcell_t;
    if (*tbl).rc == -(1 as libc::c_int) {
        dtclose((*tbl).u.p.rows);
    } else {
        cells = (*tbl).u.n.cells;
        free((*tbl).heights as *mut libc::c_void);
        free((*tbl).widths as *mut libc::c_void);
        while !(*cells).is_null() {
            free_html_cell(*cells);
            cells = cells.offset(1);
        }
        free((*tbl).u.n.cells as *mut libc::c_void);
    }
    free_html_data(&mut (*tbl).data);
    free(tbl as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_html_label(mut lp: *mut htmllabel_t, mut root: libc::c_int) {
    if (*lp).kind as libc::c_int == 1 as libc::c_int {
        free_html_tbl((*lp).u.tbl);
    } else if (*lp).kind as libc::c_int == 3 as libc::c_int {
        free_html_img((*lp).u.img);
    } else {
        free_html_text((*lp).u.txt);
    }
    if root != 0 {
        free(lp as *mut libc::c_void);
    }
}
unsafe extern "C" fn portToCell(
    mut cp: *mut htmlcell_t,
    mut id: *mut libc::c_char,
) -> *mut htmldata_t {
    let mut rv: *mut htmldata_t = 0 as *mut htmldata_t;
    if !((*cp).data.port).is_null() && strcasecmp((*cp).data.port, id) == 0 as libc::c_int {
        rv = &mut (*cp).data;
    } else if (*cp).child.kind as libc::c_int == 1 as libc::c_int {
        rv = portToTbl((*cp).child.u.tbl, id);
    } else {
        rv = 0 as *mut htmldata_t;
    }
    return rv;
}
unsafe extern "C" fn portToTbl(
    mut tp: *mut htmltbl_t,
    mut id: *mut libc::c_char,
) -> *mut htmldata_t {
    let mut rv: *mut htmldata_t = 0 as *mut htmldata_t;
    let mut cells: *mut *mut htmlcell_t = 0 as *mut *mut htmlcell_t;
    let mut cp: *mut htmlcell_t = 0 as *mut htmlcell_t;
    if !((*tp).data.port).is_null() && strcasecmp((*tp).data.port, id) == 0 as libc::c_int {
        rv = &mut (*tp).data;
    } else {
        rv = 0 as *mut htmldata_t;
        cells = (*tp).u.n.cells;
        loop {
            let fresh43 = cells;
            cells = cells.offset(1);
            cp = *fresh43;
            if cp.is_null() {
                break;
            }
            rv = portToCell(cp, id);
            if !rv.is_null() {
                break;
            }
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn html_port(
    mut n: *mut node_t,
    mut pname: *mut libc::c_char,
    mut sides: *mut libc::c_int,
) -> *mut boxf {
    let mut tp: *mut htmldata_t = 0 as *mut htmldata_t;
    let mut lbl: *mut htmllabel_t = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
        .u
        .html;
    let mut rv: *mut boxf = 0 as *mut boxf;
    if (*lbl).kind as libc::c_int == 2 as libc::c_int {
        return 0 as *mut boxf;
    }
    tp = portToTbl((*lbl).u.tbl, pname);
    if !tp.is_null() {
        rv = &mut (*tp).box_0;
        *sides = (*tp).sides as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn size_html_txt(
    mut gvc: *mut GVC_t,
    mut ftxt: *mut htmltxt_t,
    mut env: *mut htmlenv_t,
) -> libc::c_int {
    let mut xsize: libc::c_double = 0.0f64;
    let mut ysize: libc::c_double = 0.0f64;
    let mut lsize: libc::c_double = 0.;
    let mut mxfsize: libc::c_double = 0.0f64;
    let mut curbline: libc::c_double = 0.0f64;
    let mut sz: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut width: libc::c_double = 0.;
    let mut lp: textspan_t = textspan_t {
        str_0: 0 as *mut libc::c_char,
        font: 0 as *mut textfont_t,
        layout: 0 as *mut libc::c_void,
        free_layout: None,
        yoffset_layout: 0.,
        yoffset_centerline: 0.,
        size: pointf { x: 0., y: 0. },
        just: 0,
    };
    let mut tf: textfont_t = {
        let mut init = textfont_t {
            flags_cnt: [0; 4],
            c2rust_padding: [0; 4],
            name: 0 as *mut libc::c_char,
            color: 0 as *mut libc::c_char,
            postscript_alias: 0 as *mut PostscriptAlias,
            size: 0.0f64,
        };
        init.set_flags(0 as libc::c_int as libc::c_uint);
        init.set_cnt(0 as libc::c_int as libc::c_uint);
        init
    };
    let mut maxoffset: libc::c_double = 0.;
    let mut mxysize: libc::c_double = 0.0f64;
    let mut simple: bool = 1 as libc::c_int != 0;
    let mut prev_fsize: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    let mut prev_fname: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*ftxt).nspans as libc::c_int {
        if (*((*ftxt).spans).offset(i as isize)).nitems as libc::c_int > 1 as libc::c_int {
            simple = 0 as libc::c_int != 0;
            break;
        } else {
            if !((*((*((*ftxt).spans).offset(i as isize)).items).offset(0 as libc::c_int as isize))
                .font)
                .is_null()
            {
                if (*(*((*((*ftxt).spans).offset(i as isize)).items)
                    .offset(0 as libc::c_int as isize))
                .font)
                    .flags()
                    != 0
                {
                    simple = 0 as libc::c_int != 0;
                    break;
                } else {
                    if (*(*((*((*ftxt).spans).offset(i as isize)).items)
                        .offset(0 as libc::c_int as isize))
                    .font)
                        .size
                        > 0 as libc::c_int as libc::c_double
                    {
                        tf.size = (*(*((*((*ftxt).spans).offset(i as isize)).items)
                            .offset(0 as libc::c_int as isize))
                        .font)
                            .size;
                    } else {
                        tf.size = (*env).finfo.size;
                    }
                    if !((*(*((*((*ftxt).spans).offset(i as isize)).items)
                        .offset(0 as libc::c_int as isize))
                    .font)
                        .name)
                        .is_null()
                    {
                        tf.name = (*(*((*((*ftxt).spans).offset(i as isize)).items)
                            .offset(0 as libc::c_int as isize))
                        .font)
                            .name;
                    } else {
                        tf.name = (*env).finfo.name;
                    }
                }
            } else {
                tf.size = (*env).finfo.size;
                tf.name = (*env).finfo.name;
            }
            if prev_fsize == -(1 as libc::c_int) as libc::c_double {
                prev_fsize = tf.size;
            } else if tf.size != prev_fsize {
                simple = 0 as libc::c_int != 0;
                break;
            }
            if prev_fname.is_null() {
                prev_fname = tf.name;
            } else if strcmp(tf.name, prev_fname) != 0 {
                simple = 0 as libc::c_int != 0;
                break;
            }
            i += 1;
        }
    }
    (*ftxt).simple = simple as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*ftxt).nspans as libc::c_int {
        width = 0 as libc::c_int as libc::c_double;
        mxfsize = 0 as libc::c_int as libc::c_double;
        maxoffset = mxfsize;
        mxysize = maxoffset;
        j = 0 as libc::c_int;
        while j < (*((*ftxt).spans).offset(i as isize)).nitems as libc::c_int {
            lp.str_0 = strdup_and_subst_obj(
                (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).str_0,
                (*env).obj,
            );
            if !((*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font).is_null()
            {
                if (*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font)
                    .flags()
                    != 0
                {
                    tf.set_flags(
                        (*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font)
                            .flags(),
                    );
                } else if ((*env).finfo).flags() as libc::c_int > 0 as libc::c_int {
                    tf.set_flags(((*env).finfo).flags());
                } else {
                    tf.set_flags(0 as libc::c_int as libc::c_uint);
                }
                if (*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font).size
                    > 0 as libc::c_int as libc::c_double
                {
                    tf.size =
                        (*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font)
                            .size;
                } else {
                    tf.size = (*env).finfo.size;
                }
                if !((*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font)
                    .name)
                    .is_null()
                {
                    tf.name =
                        (*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font)
                            .name;
                } else {
                    tf.name = (*env).finfo.name;
                }
                if !((*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font)
                    .color)
                    .is_null()
                {
                    tf.color =
                        (*(*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font)
                            .color;
                } else {
                    tf.color = (*env).finfo.color;
                }
            } else {
                tf.size = (*env).finfo.size;
                tf.name = (*env).finfo.name;
                tf.color = (*env).finfo.color;
                tf.set_flags(((*env).finfo).flags());
            }
            lp.font = (Some(((*(*gvc).textfont_dt).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                (*gvc).textfont_dt,
                &mut tf as *mut textfont_t as *mut libc::c_void,
                0o1 as libc::c_int,
            ) as *mut textfont_t;
            sz = textspan_size(gvc, &mut lp);
            free(
                (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).str_0
                    as *mut libc::c_void,
            );
            let ref mut fresh44 =
                (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).str_0;
            *fresh44 = lp.str_0;
            (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize))
                .size
                .x = sz.x;
            (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).yoffset_layout =
                lp.yoffset_layout;
            (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize))
                .yoffset_centerline = lp.yoffset_centerline;
            let ref mut fresh45 =
                (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).font;
            *fresh45 = lp.font;
            let ref mut fresh46 =
                (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).layout;
            *fresh46 = lp.layout;
            let ref mut fresh47 =
                (*((*((*ftxt).spans).offset(i as isize)).items).offset(j as isize)).free_layout;
            *fresh47 = lp.free_layout;
            width += sz.x;
            mxfsize = if tf.size > mxfsize { tf.size } else { mxfsize };
            mxysize = if sz.y > mxysize { sz.y } else { mxysize };
            maxoffset = if lp.yoffset_centerline > maxoffset {
                lp.yoffset_centerline
            } else {
                maxoffset
            };
            j += 1;
        }
        (*((*ftxt).spans).offset(i as isize)).size = width;
        if simple {
            lsize = mxysize;
            if i == 0 as libc::c_int {
                (*((*ftxt).spans).offset(i as isize)).lfsize = mxfsize;
            } else {
                (*((*ftxt).spans).offset(i as isize)).lfsize = mxysize;
            }
        } else {
            lsize = mxfsize;
            if i == 0 as libc::c_int {
                (*((*ftxt).spans).offset(i as isize)).lfsize = mxfsize - maxoffset;
            } else {
                (*((*ftxt).spans).offset(i as isize)).lfsize =
                    mxfsize + ysize - curbline - maxoffset;
            }
        }
        curbline += (*((*ftxt).spans).offset(i as isize)).lfsize;
        xsize = if width > xsize { width } else { xsize };
        ysize += lsize;
        i += 1;
    }
    (*ftxt).box_0.UR.x = xsize;
    if (*ftxt).nspans as libc::c_int == 1 as libc::c_int {
        (*ftxt).box_0.UR.y = mxysize;
    } else {
        (*ftxt).box_0.UR.y = ysize;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn size_html_img(
    mut img: *mut htmlimg_t,
    mut env: *mut htmlenv_t,
) -> libc::c_int {
    let mut b: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    let mut rv: libc::c_int = 0;
    b.LL.y = 0 as libc::c_int;
    b.LL.x = b.LL.y;
    b.UR = gvusershape_size((*env).g, (*img).src);
    if b.UR.x == -(1 as libc::c_int) && b.UR.y == -(1 as libc::c_int) {
        rv = 1 as libc::c_int;
        b.UR.y = 0 as libc::c_int;
        b.UR.x = b.UR.y;
        agerr(
            AGERR,
            b"No or improper image file=\"%s\"\n\0" as *const u8 as *const libc::c_char,
            (*img).src,
        );
    } else {
        rv = 0 as libc::c_int;
        (*((*((*env).g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_images =
            1 as libc::c_int != 0;
    }
    (*img).box_0.LL.x = b.LL.x as libc::c_double;
    (*img).box_0.LL.y = b.LL.y as libc::c_double;
    (*img).box_0.UR.x = b.UR.x as libc::c_double;
    (*img).box_0.UR.y = b.UR.y as libc::c_double;
    return rv;
}
unsafe extern "C" fn size_html_cell(
    mut g: *mut graph_t,
    mut cp: *mut htmlcell_t,
    mut parent: *mut htmltbl_t,
    mut env: *mut htmlenv_t,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut sz: pointf = pointf { x: 0., y: 0. };
    let mut child_sz: pointf = pointf { x: 0., y: 0. };
    let mut margin: libc::c_int = 0;
    let ref mut fresh48 = (*cp).parent;
    *fresh48 = parent;
    if (*cp).data.flags as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        if (*parent).data.flags as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
            (*cp).data.pad = (*parent).data.pad;
        } else {
            (*cp).data.pad = 2 as libc::c_int as libc::c_uchar;
        }
    }
    if (*cp).data.flags as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        if (*parent).cb as libc::c_int >= 0 as libc::c_int {
            (*cp).data.border = (*parent).cb as libc::c_uchar;
        } else if (*parent).data.flags as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
        {
            (*cp).data.border = (*parent).data.border;
        } else {
            (*cp).data.border = 1 as libc::c_int as libc::c_uchar;
        }
    }
    if (*cp).child.kind as libc::c_int == 1 as libc::c_int {
        rv = size_html_tbl(g, (*cp).child.u.tbl, cp, env);
        child_sz = (*(*cp).child.u.tbl).data.box_0.UR;
    } else if (*cp).child.kind as libc::c_int == 3 as libc::c_int {
        rv = size_html_img((*cp).child.u.img, env);
        child_sz = (*(*cp).child.u.img).box_0.UR;
    } else {
        rv = size_html_txt(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc,
            (*cp).child.u.txt,
            env,
        );
        child_sz = (*(*cp).child.u.txt).box_0.UR;
    }
    margin = 2 as libc::c_int * ((*cp).data.pad as libc::c_int + (*cp).data.border as libc::c_int);
    sz.x = child_sz.x + margin as libc::c_double;
    sz.y = child_sz.y + margin as libc::c_double;
    if (*cp).data.flags as libc::c_int & 1 as libc::c_int != 0 {
        if (*cp).data.width as libc::c_int != 0 && (*cp).data.height as libc::c_int != 0 {
            if (((*cp).data.width as libc::c_int as libc::c_double) < sz.x
                || ((*cp).data.height as libc::c_int as libc::c_double) < sz.y)
                && (*cp).child.kind as libc::c_int != 3 as libc::c_int
            {
                agerr(
                    AGWARN,
                    b"cell size too small for content\n\0" as *const u8 as *const libc::c_char,
                );
                rv = 1 as libc::c_int;
            }
            sz.y = 0 as libc::c_int as libc::c_double;
            sz.x = sz.y;
        } else {
            agerr(
                AGWARN,
                b"fixed cell size with unspecified width or height\n\0" as *const u8
                    as *const libc::c_char,
            );
            rv = 1 as libc::c_int;
        }
    }
    (*cp).data.box_0.UR.x = if sz.x > (*cp).data.width as libc::c_int as libc::c_double {
        sz.x
    } else {
        (*cp).data.width as libc::c_int as libc::c_double
    };
    (*cp).data.box_0.UR.y = if sz.y > (*cp).data.height as libc::c_int as libc::c_double {
        sz.y
    } else {
        (*cp).data.height as libc::c_int as libc::c_double
    };
    return rv;
}
unsafe extern "C" fn findCol(
    mut ps: *mut PointSet,
    mut row: libc::c_int,
    mut col: libc::c_int,
    mut cellp: *mut htmlcell_t,
) -> libc::c_int {
    let mut notFound: libc::c_int = 1 as libc::c_int;
    let mut lastc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut end: libc::c_int = (*cellp).cspan as libc::c_int - 1 as libc::c_int;
    while notFound != 0 {
        lastc = col + end;
        c = lastc;
        while c >= col {
            if isInPS(ps, c, row) != 0 {
                break;
            }
            c -= 1;
        }
        if c >= col {
            col = c + 1 as libc::c_int;
        } else {
            notFound = 0 as libc::c_int;
        }
    }
    j = col;
    while j < col + (*cellp).cspan as libc::c_int {
        i = row;
        while i < row + (*cellp).rspan as libc::c_int {
            addPS(ps, j, i);
            i += 1;
        }
        j += 1;
    }
    return col;
}
unsafe extern "C" fn processTbl(
    mut g: *mut graph_t,
    mut tbl: *mut htmltbl_t,
    mut env: *mut htmlenv_t,
) -> libc::c_int {
    let mut rp: *mut pitem = 0 as *mut pitem;
    let mut cp: *mut pitem = 0 as *mut pitem;
    let mut cdict: *mut Dt_t = 0 as *mut Dt_t;
    let mut cellp: *mut htmlcell_t = 0 as *mut htmlcell_t;
    let mut cells: *mut *mut htmlcell_t = 0 as *mut *mut htmlcell_t;
    let mut rows: *mut Dt_t = (*tbl).u.p.rows;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut n_rows: libc::c_int = 0 as libc::c_int;
    let mut n_cols: libc::c_int = 0 as libc::c_int;
    let mut ps: *mut PointSet = newPS();
    let mut is: *mut Dt_t = openIntSet();
    rp = dtflatten(rows) as *mut pitem;
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut r: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    while !rp.is_null() {
        cdict = (*rp).u.rp;
        cp = dtflatten(cdict) as *mut pitem;
        while !cp.is_null() {
            cellp = (*cp).u.cp;
            cnt = cnt.wrapping_add(1);
            cp = (*(cp as *mut Dtlink_t)).right as *mut pitem;
        }
        if (*rp).ruled != 0 {
            addIntSet(is, r as libc::c_int + 1 as libc::c_int);
        }
        rp = (*(rp as *mut Dtlink_t)).right as *mut pitem;
        r = r.wrapping_add(1);
    }
    let ref mut fresh49 = (*tbl).u.n.cells;
    *fresh49 = gcalloc(
        cnt.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<*mut htmlcell_t>() as libc::c_ulong,
    ) as *mut *mut htmlcell_t;
    cells = *fresh49;
    rp = dtflatten(rows) as *mut pitem;
    r = 0 as libc::c_int as libc::c_ushort;
    while !rp.is_null() {
        cdict = (*rp).u.rp;
        cp = dtflatten(cdict) as *mut pitem;
        let mut c: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
        while !cp.is_null() {
            cellp = (*cp).u.cp;
            let fresh50 = cells;
            cells = cells.offset(1);
            *fresh50 = cellp;
            rv |= size_html_cell(g, cellp, tbl, env);
            c = findCol(ps, r as libc::c_int, c as libc::c_int, cellp) as libc::c_ushort;
            (*cellp).row = r;
            (*cellp).col = c;
            c = (c as libc::c_int + (*cellp).cspan as libc::c_int) as libc::c_ushort;
            n_cols = if c as libc::c_int > n_cols {
                c as libc::c_int
            } else {
                n_cols
            };
            n_rows = if r as libc::c_int + (*cellp).rspan as libc::c_int > n_rows {
                r as libc::c_int + (*cellp).rspan as libc::c_int
            } else {
                n_rows
            };
            if inIntSet(is, r as libc::c_int + (*cellp).rspan as libc::c_int) != 0 {
                let ref mut fresh51 = (*cellp).ruled;
                *fresh51 = (*fresh51 as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
            }
            cp = (*(cp as *mut Dtlink_t)).right as *mut pitem;
        }
        rp = (*(rp as *mut Dtlink_t)).right as *mut pitem;
        r = r.wrapping_add(1);
    }
    (*tbl).rc = n_rows;
    (*tbl).cc = n_cols;
    dtclose(rows);
    dtclose(is);
    freePS(ps);
    return rv;
}
unsafe extern "C" fn sizeLinearArray(mut tbl: *mut htmltbl_t) {
    let mut cp: *mut htmlcell_t = 0 as *mut htmlcell_t;
    let mut cells: *mut *mut htmlcell_t = 0 as *mut *mut htmlcell_t;
    let mut wd: libc::c_int = 0;
    let mut ht: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let ref mut fresh52 = (*tbl).heights;
    *fresh52 = gcalloc(
        ((*tbl).rc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh53 = (*tbl).widths;
    *fresh53 = gcalloc(
        ((*tbl).cc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    cells = (*tbl).u.n.cells;
    while !(*cells).is_null() {
        cp = *cells;
        if (*cp).rspan as libc::c_int == 1 as libc::c_int {
            ht = (*cp).data.box_0.UR.y as libc::c_int;
        } else {
            ht = (((*cp).data.box_0.UR.y
                - (((*tbl).data.space as libc::c_int - 1 as libc::c_int)
                    * ((*cp).rspan as libc::c_int - 1 as libc::c_int))
                    as libc::c_double)
                / (*cp).rspan as libc::c_int as libc::c_double) as libc::c_int;
            ht = if ht > 1 as libc::c_int {
                ht
            } else {
                1 as libc::c_int
            };
        }
        if (*cp).cspan as libc::c_int == 1 as libc::c_int {
            wd = (*cp).data.box_0.UR.x as libc::c_int;
        } else {
            wd = (((*cp).data.box_0.UR.x
                - (((*tbl).data.space as libc::c_int - 1 as libc::c_int)
                    * ((*cp).cspan as libc::c_int - 1 as libc::c_int))
                    as libc::c_double)
                / (*cp).cspan as libc::c_int as libc::c_double) as libc::c_int;
            wd = if wd > 1 as libc::c_int {
                wd
            } else {
                1 as libc::c_int
            };
        }
        i = (*cp).row as libc::c_int;
        while i < (*cp).row as libc::c_int + (*cp).rspan as libc::c_int {
            y = *((*tbl).heights).offset(i as isize);
            *((*tbl).heights).offset(i as isize) = if y > ht { y } else { ht };
            i += 1;
        }
        i = (*cp).col as libc::c_int;
        while i < (*cp).col as libc::c_int + (*cp).cspan as libc::c_int {
            x = *((*tbl).widths).offset(i as isize);
            *((*tbl).widths).offset(i as isize) = if x > wd { x } else { wd };
            i += 1;
        }
        cells = cells.offset(1);
    }
}
unsafe extern "C" fn closeGraphs(mut rowg: *mut graph_t, mut colg: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    n = (*((*(colg as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .list as *mut libc::c_void,
        );
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list as *mut libc::c_void,
        );
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    agclose(rowg);
    agclose(colg);
}
unsafe extern "C" fn checkChain(mut g: *mut graph_t) {
    let mut t: *mut node_t = 0 as *mut node_t;
    let mut h: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    t = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    h = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    while !h.is_null() {
        if (agedge(g, t, h, 0 as *mut libc::c_char, 0 as libc::c_int)).is_null() {
            e = agedge(g, t, h, 0 as *mut libc::c_char, 1 as libc::c_int);
            agbindrec(
                e as *mut libc::c_void,
                b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen =
                0 as libc::c_int as libc::c_ushort;
            let ref mut fresh54 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list;
            *fresh54 = if !((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .is_null()
            {
                grealloc(
                    (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .out
                        .list as *mut libc::c_void,
                    (((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .out
                        .size
                        + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
                ) as *mut *mut edge_t
            } else {
                gmalloc(
                    (((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .out
                        .size
                        + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
                ) as *mut *mut edge_t
            };
            let ref mut fresh55 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .size;
            let fresh56 = *fresh55;
            *fresh55 = *fresh55 + 1;
            let ref mut fresh57 = *((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .offset(fresh56 as isize);
            *fresh57 = e;
            let ref mut fresh58 = *((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .offset(
                    (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .out
                        .size as isize,
                );
            *fresh58 = 0 as *mut edge_t;
            let ref mut fresh59 = (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .list;
            *fresh59 = if !((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .list)
                .is_null()
            {
                grealloc(
                    (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .in_0
                        .list as *mut libc::c_void,
                    (((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .in_0
                        .size
                        + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
                ) as *mut *mut edge_t
            } else {
                gmalloc(
                    (((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .in_0
                        .size
                        + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
                ) as *mut *mut edge_t
            };
            let ref mut fresh60 = (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .size;
            let fresh61 = *fresh60;
            *fresh60 = *fresh60 + 1;
            let ref mut fresh62 = *((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .list)
                .offset(fresh61 as isize);
            *fresh62 = e;
            let ref mut fresh63 = *((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .list)
                .offset(
                    (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .in_0
                        .size as isize,
                );
            *fresh63 = 0 as *mut edge_t;
        }
        t = h;
        h = (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn checkEdge(
    mut g: *mut graph_t,
    mut t: *mut node_t,
    mut h: *mut node_t,
    mut sz: libc::c_int,
) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    e = agedge(g, t, h, 0 as *mut libc::c_char, 0 as libc::c_int);
    if !e.is_null() {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen =
            (if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int > sz {
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int
            } else {
                sz
            }) as libc::c_ushort;
    } else {
        e = agedge(g, t, h, 0 as *mut libc::c_char, 1 as libc::c_int);
        agbindrec(
            e as *mut libc::c_void,
            b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_uint,
            1 as libc::c_int,
        );
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen = sz as libc::c_ushort;
        let ref mut fresh64 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list;
        *fresh64 = if !((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list)
            .is_null()
        {
            grealloc(
                (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .out
                    .list as *mut libc::c_void,
                (((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .out
                    .size
                    + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
            ) as *mut *mut edge_t
        } else {
            gmalloc(
                (((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .out
                    .size
                    + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
            ) as *mut *mut edge_t
        };
        let ref mut fresh65 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .size;
        let fresh66 = *fresh65;
        *fresh65 = *fresh65 + 1;
        let ref mut fresh67 = *((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list)
            .offset(fresh66 as isize);
        *fresh67 = e;
        let ref mut fresh68 = *((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list)
            .offset(
                (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .out
                    .size as isize,
            );
        *fresh68 = 0 as *mut edge_t;
        let ref mut fresh69 = (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list;
        *fresh69 = if !((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list)
            .is_null()
        {
            grealloc(
                (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .in_0
                    .list as *mut libc::c_void,
                (((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .in_0
                    .size
                    + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
            ) as *mut *mut edge_t
        } else {
            gmalloc(
                (((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .in_0
                    .size
                    + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
            ) as *mut *mut edge_t
        };
        let ref mut fresh70 = (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .size;
        let fresh71 = *fresh70;
        *fresh70 = *fresh70 + 1;
        let ref mut fresh72 = *((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list)
            .offset(fresh71 as isize);
        *fresh72 = e;
        let ref mut fresh73 = *((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list)
            .offset(
                (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .in_0
                    .size as isize,
            );
        *fresh73 = 0 as *mut edge_t;
    };
}
unsafe extern "C" fn makeGraphs(
    mut tbl: *mut htmltbl_t,
    mut rowg: *mut graph_t,
    mut colg: *mut graph_t,
) {
    let mut cp: *mut htmlcell_t = 0 as *mut htmlcell_t;
    let mut cells: *mut *mut htmlcell_t = 0 as *mut *mut htmlcell_t;
    let mut t: *mut node_t = 0 as *mut node_t;
    let mut lastn: *mut node_t = 0 as *mut node_t;
    let mut h: *mut node_t = 0 as *mut node_t;
    let mut i: libc::c_int = 0;
    let mut value_buffer: [libc::c_char; 12] = [0; 12];
    lastn = 0 as *mut node_t;
    i = 0 as libc::c_int;
    while i <= (*tbl).cc {
        snprintf(
            value_buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            i,
        );
        t = agnode(colg, value_buffer.as_mut_ptr(), 1 as libc::c_int);
        agbindrec(
            t as *mut libc::c_void,
            b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
            1 as libc::c_int,
        );
        (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .size = 0 as libc::c_int;
        let ref mut fresh74 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list;
        *fresh74 = gcalloc(
            ((*tbl).rc + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .size = 0 as libc::c_int;
        let ref mut fresh75 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list;
        *fresh75 = gcalloc(
            ((*tbl).rc + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        if !lastn.is_null() {
            let ref mut fresh76 = (*((*(lastn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
            *fresh76 = t;
            lastn = t;
        } else {
            let ref mut fresh77 = (*((*(colg as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
            *fresh77 = t;
            lastn = *fresh77;
        }
        i += 1;
    }
    lastn = 0 as *mut node_t;
    i = 0 as libc::c_int;
    while i <= (*tbl).rc {
        snprintf(
            value_buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            i,
        );
        t = agnode(rowg, value_buffer.as_mut_ptr(), 1 as libc::c_int);
        agbindrec(
            t as *mut libc::c_void,
            b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
            1 as libc::c_int,
        );
        (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .size = 0 as libc::c_int;
        let ref mut fresh78 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list;
        *fresh78 = gcalloc(
            ((*tbl).cc + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .size = 0 as libc::c_int;
        let ref mut fresh79 = (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list;
        *fresh79 = gcalloc(
            ((*tbl).cc + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        if !lastn.is_null() {
            let ref mut fresh80 = (*((*(lastn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
            *fresh80 = t;
            lastn = t;
        } else {
            let ref mut fresh81 = (*((*(rowg as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
            *fresh81 = t;
            lastn = *fresh81;
        }
        i += 1;
    }
    cells = (*tbl).u.n.cells;
    while !(*cells).is_null() {
        cp = *cells;
        snprintf(
            value_buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*cp).col as libc::c_int,
        );
        t = agnode(colg, value_buffer.as_mut_ptr(), 0 as libc::c_int);
        snprintf(
            value_buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*cp).col as libc::c_int + (*cp).cspan as libc::c_int,
        );
        h = agnode(colg, value_buffer.as_mut_ptr(), 0 as libc::c_int);
        checkEdge(colg, t, h, (*cp).data.box_0.UR.x as libc::c_int);
        snprintf(
            value_buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*cp).row as libc::c_int,
        );
        t = agnode(rowg, value_buffer.as_mut_ptr(), 0 as libc::c_int);
        snprintf(
            value_buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*cp).row as libc::c_int + (*cp).rspan as libc::c_int,
        );
        h = agnode(rowg, value_buffer.as_mut_ptr(), 0 as libc::c_int);
        checkEdge(rowg, t, h, (*cp).data.box_0.UR.y as libc::c_int);
        cells = cells.offset(1);
    }
    checkChain(colg);
    checkChain(rowg);
}
unsafe extern "C" fn setSizes(
    mut tbl: *mut htmltbl_t,
    mut rowg: *mut graph_t,
    mut colg: *mut graph_t,
) {
    let mut i: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut prev: libc::c_int = 0;
    prev = 0 as libc::c_int;
    n = (*((*(rowg as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    i = 0 as libc::c_int;
    n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    while !n.is_null() {
        *((*tbl).heights).offset(i as isize) =
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank - prev;
        prev = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        i += 1;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    prev = 0 as libc::c_int;
    n = (*((*(colg as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    i = 0 as libc::c_int;
    n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    while !n.is_null() {
        *((*tbl).widths).offset(i as isize) =
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank - prev;
        prev = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        i += 1;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn sizeArray(mut tbl: *mut htmltbl_t) {
    let mut rowg: *mut graph_t = 0 as *mut graph_t;
    let mut colg: *mut graph_t = 0 as *mut graph_t;
    let mut dir: Agdesc_t = Agstrictdirected;
    if (*tbl).rc == 1 as libc::c_int || (*tbl).cc == 1 as libc::c_int {
        sizeLinearArray(tbl);
        return;
    }
    let ref mut fresh82 = (*tbl).heights;
    *fresh82 = gcalloc(
        ((*tbl).rc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh83 = (*tbl).widths;
    *fresh83 = gcalloc(
        ((*tbl).cc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    rowg = agopen(
        b"rowg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dir,
        0 as *mut Agdisc_t,
    );
    colg = agopen(
        b"colg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dir,
        0 as *mut Agdisc_t,
    );
    agbindrec(
        rowg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agbindrec(
        colg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    makeGraphs(tbl, rowg, colg);
    rank(rowg, 2 as libc::c_int, 2147483647 as libc::c_int);
    rank(colg, 2 as libc::c_int, 2147483647 as libc::c_int);
    setSizes(tbl, rowg, colg);
    closeGraphs(rowg, colg);
}
unsafe extern "C" fn pos_html_img(mut cp: *mut htmlimg_t, mut pos: boxf) {
    (*cp).box_0 = pos;
}
unsafe extern "C" fn pos_html_txt(mut ftxt: *mut htmltxt_t, mut c: libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ftxt).nspans as libc::c_int {
        if (*((*ftxt).spans).offset(i as isize)).just as libc::c_int == 0 as libc::c_int {
            (*((*ftxt).spans).offset(i as isize)).just = c;
        }
        i += 1;
    }
}
unsafe extern "C" fn pos_html_cell(mut cp: *mut htmlcell_t, mut pos: boxf, mut sides: libc::c_int) {
    let mut delx: libc::c_double = 0.;
    let mut dely: libc::c_double = 0.;
    let mut oldsz: pointf = pointf { x: 0., y: 0. };
    let mut cbox: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    if ((*cp).data.pencolor).is_null() && !((*(*cp).parent).data.pencolor).is_null() {
        let ref mut fresh84 = (*cp).data.pencolor;
        *fresh84 = strdup((*(*cp).parent).data.pencolor);
    }
    if (*cp).data.flags as libc::c_int & 1 as libc::c_int != 0 {
        oldsz = (*cp).data.box_0.UR;
        delx = pos.UR.x - pos.LL.x - oldsz.x;
        if delx > 0 as libc::c_int as libc::c_double {
            match (*cp).data.flags as libc::c_int
                & ((1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            {
                4 => {
                    pos.UR.x = pos.LL.x + oldsz.x;
                }
                2 => {
                    pos.UR.x += delx;
                    pos.LL.x += delx;
                }
                _ => {
                    pos.LL.x += delx / 2 as libc::c_int as libc::c_double;
                    pos.UR.x -= delx / 2 as libc::c_int as libc::c_double;
                }
            }
        }
        dely = pos.UR.y - pos.LL.y - oldsz.y;
        if dely > 0 as libc::c_int as libc::c_double {
            match (*cp).data.flags as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            {
                16 => {
                    pos.UR.y = pos.LL.y + oldsz.y;
                }
                8 => {
                    pos.UR.y += dely;
                    pos.LL.y += dely;
                }
                _ => {
                    pos.LL.y += dely / 2 as libc::c_int as libc::c_double;
                    pos.UR.y -= dely / 2 as libc::c_int as libc::c_double;
                }
            }
        }
    }
    (*cp).data.box_0 = pos;
    (*cp).data.sides = sides as libc::c_uchar;
    cbox.LL.x = pos.LL.x
        + (*cp).data.border as libc::c_int as libc::c_double
        + (*cp).data.pad as libc::c_int as libc::c_double;
    cbox.LL.y = pos.LL.y
        + (*cp).data.border as libc::c_int as libc::c_double
        + (*cp).data.pad as libc::c_int as libc::c_double;
    cbox.UR.x = pos.UR.x
        - (*cp).data.border as libc::c_int as libc::c_double
        - (*cp).data.pad as libc::c_int as libc::c_double;
    cbox.UR.y = pos.UR.y
        - (*cp).data.border as libc::c_int as libc::c_double
        - (*cp).data.pad as libc::c_int as libc::c_double;
    if (*cp).child.kind as libc::c_int == 1 as libc::c_int {
        pos_html_tbl((*cp).child.u.tbl, cbox, sides);
    } else if (*cp).child.kind as libc::c_int == 3 as libc::c_int {
        oldsz = (*(*cp).child.u.img).box_0.UR;
        delx = cbox.UR.x - cbox.LL.x - oldsz.x;
        if delx > 0 as libc::c_int as libc::c_double {
            match (*cp).data.flags as libc::c_int
                & ((1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            {
                4 => {
                    cbox.UR.x -= delx;
                }
                2 => {
                    cbox.LL.x += delx;
                }
                _ => {}
            }
        }
        dely = cbox.UR.y - cbox.LL.y - oldsz.y;
        if dely > 0 as libc::c_int as libc::c_double {
            match (*cp).data.flags as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            {
                16 => {
                    cbox.UR.y -= dely;
                }
                8 => {
                    cbox.LL.y += dely;
                }
                _ => {}
            }
        }
        pos_html_img((*cp).child.u.img, cbox);
    } else {
        let mut dfltalign: libc::c_char = 0;
        let mut af: libc::c_int = 0;
        oldsz = (*(*cp).child.u.txt).box_0.UR;
        delx = cbox.UR.x - cbox.LL.x - oldsz.x;
        if delx > 0 as libc::c_int as libc::c_double && {
            af = (*cp).data.flags as libc::c_int
                & ((1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int);
            af != (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
        } {
            match af {
                4 => {
                    cbox.UR.x -= delx;
                }
                2 => {
                    cbox.LL.x += delx;
                }
                _ => {
                    cbox.LL.x += delx / 2 as libc::c_int as libc::c_double;
                    cbox.UR.x -= delx / 2 as libc::c_int as libc::c_double;
                }
            }
        }
        dely = cbox.UR.y - cbox.LL.y - oldsz.y;
        if dely > 0 as libc::c_int as libc::c_double {
            match (*cp).data.flags as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            {
                16 => {
                    cbox.UR.y -= dely;
                }
                8 => {
                    cbox.LL.y += dely;
                }
                _ => {
                    cbox.LL.y += dely / 2 as libc::c_int as libc::c_double;
                    cbox.UR.y -= dely / 2 as libc::c_int as libc::c_double;
                }
            }
        }
        (*(*cp).child.u.txt).box_0 = cbox;
        match (*cp).data.flags as libc::c_int
            & ((1 as libc::c_int) << 8 as libc::c_int | (1 as libc::c_int) << 9 as libc::c_int)
        {
            512 => {
                dfltalign = 'l' as i32 as libc::c_char;
            }
            256 => {
                dfltalign = 'r' as i32 as libc::c_char;
            }
            _ => {
                dfltalign = 'n' as i32 as libc::c_char;
            }
        }
        pos_html_txt((*cp).child.u.txt, dfltalign);
    };
}
unsafe extern "C" fn pos_html_tbl(mut tbl: *mut htmltbl_t, mut pos: boxf, mut sides: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut delx: libc::c_int = 0;
    let mut dely: libc::c_int = 0;
    let mut oldsz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut extra: libc::c_int = 0;
    let mut plus: libc::c_int = 0;
    let mut cells: *mut *mut htmlcell_t = (*tbl).u.n.cells;
    let mut cp: *mut htmlcell_t = 0 as *mut htmlcell_t;
    let mut cbox: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    if !((*tbl).u.n.parent).is_null()
        && !((*(*tbl).u.n.parent).data.pencolor).is_null()
        && ((*tbl).data.pencolor).is_null()
    {
        let ref mut fresh85 = (*tbl).data.pencolor;
        *fresh85 = strdup((*(*tbl).u.n.parent).data.pencolor);
    }
    oldsz = (*tbl).data.box_0.UR.x as libc::c_int;
    delx = (pos.UR.x - pos.LL.x - oldsz as libc::c_double) as libc::c_int;
    if delx >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"delx >= 0\0" as *const u8 as *const libc::c_char,
            b"htmltable.c\0" as *const u8 as *const libc::c_char,
            1686 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void pos_html_tbl(htmltbl_t *, boxf, int)\0",
            ))
            .as_ptr(),
        );
    }
    oldsz = (*tbl).data.box_0.UR.y as libc::c_int;
    dely = (pos.UR.y - pos.LL.y - oldsz as libc::c_double) as libc::c_int;
    if dely >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"dely >= 0\0" as *const u8 as *const libc::c_char,
            b"htmltable.c\0" as *const u8 as *const libc::c_char,
            1689 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void pos_html_tbl(htmltbl_t *, boxf, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*tbl).data.flags as libc::c_int & 1 as libc::c_int != 0 {
        if delx > 0 as libc::c_int {
            match (*tbl).data.flags as libc::c_int
                & ((1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            {
                4 => {
                    pos.UR.x = pos.LL.x + oldsz as libc::c_double;
                }
                2 => {
                    pos.UR.x += delx as libc::c_double;
                    pos.LL.x += delx as libc::c_double;
                }
                _ => {
                    pos.LL.x += (delx / 2 as libc::c_int) as libc::c_double;
                    pos.UR.x -= (delx / 2 as libc::c_int) as libc::c_double;
                }
            }
            delx = 0 as libc::c_int;
        }
        if dely > 0 as libc::c_int {
            match (*tbl).data.flags as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            {
                16 => {
                    pos.UR.y = pos.LL.y + oldsz as libc::c_double;
                }
                8 => {
                    pos.LL.y += dely as libc::c_double;
                    pos.UR.y = pos.LL.y + oldsz as libc::c_double;
                }
                _ => {
                    pos.LL.y += (dely / 2 as libc::c_int) as libc::c_double;
                    pos.UR.y -= (dely / 2 as libc::c_int) as libc::c_double;
                }
            }
            dely = 0 as libc::c_int;
        }
    }
    x = (pos.LL.x
        + (*tbl).data.border as libc::c_int as libc::c_double
        + (*tbl).data.space as libc::c_int as libc::c_double) as libc::c_int;
    extra = delx / (*tbl).cc;
    plus = if delx - extra * (*tbl).cc >= 0 as libc::c_int {
        ((delx - extra * (*tbl).cc) as libc::c_double + 0.5f64) as libc::c_int
    } else {
        ((delx - extra * (*tbl).cc) as libc::c_double - 0.5f64) as libc::c_int
    };
    i = 0 as libc::c_int;
    while i <= (*tbl).cc {
        delx = *((*tbl).widths).offset(i as isize)
            + extra
            + (if i < plus {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            });
        *((*tbl).widths).offset(i as isize) = x;
        x += delx + (*tbl).data.space as libc::c_int;
        i += 1;
    }
    y = (pos.UR.y
        - (*tbl).data.border as libc::c_int as libc::c_double
        - (*tbl).data.space as libc::c_int as libc::c_double) as libc::c_int;
    extra = dely / (*tbl).rc;
    plus = if dely - extra * (*tbl).rc >= 0 as libc::c_int {
        ((dely - extra * (*tbl).rc) as libc::c_double + 0.5f64) as libc::c_int
    } else {
        ((dely - extra * (*tbl).rc) as libc::c_double - 0.5f64) as libc::c_int
    };
    i = 0 as libc::c_int;
    while i <= (*tbl).rc {
        dely = *((*tbl).heights).offset(i as isize)
            + extra
            + (if i < plus {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            });
        *((*tbl).heights).offset(i as isize) = y;
        y -= dely + (*tbl).data.space as libc::c_int;
        i += 1;
    }
    loop {
        let fresh86 = cells;
        cells = cells.offset(1);
        cp = *fresh86;
        if cp.is_null() {
            break;
        }
        let mut mask: libc::c_int = 0 as libc::c_int;
        if sides != 0 {
            if (*cp).col as libc::c_int == 0 as libc::c_int {
                mask |= (1 as libc::c_int) << 3 as libc::c_int;
            }
            if (*cp).row as libc::c_int == 0 as libc::c_int {
                mask |= (1 as libc::c_int) << 2 as libc::c_int;
            }
            if (*cp).col as libc::c_int + (*cp).cspan as libc::c_int == (*tbl).cc {
                mask |= (1 as libc::c_int) << 1 as libc::c_int;
            }
            if (*cp).row as libc::c_int + (*cp).rspan as libc::c_int == (*tbl).rc {
                mask |= (1 as libc::c_int) << 0 as libc::c_int;
            }
        }
        cbox.LL.x = *((*tbl).widths).offset((*cp).col as isize) as libc::c_double;
        cbox.UR.x = (*((*tbl).widths)
            .offset(((*cp).col as libc::c_int + (*cp).cspan as libc::c_int) as isize)
            - (*tbl).data.space as libc::c_int) as libc::c_double;
        cbox.UR.y = *((*tbl).heights).offset((*cp).row as isize) as libc::c_double;
        cbox.LL.y = (*((*tbl).heights)
            .offset(((*cp).row as libc::c_int + (*cp).rspan as libc::c_int) as isize)
            + (*tbl).data.space as libc::c_int) as libc::c_double;
        pos_html_cell(cp, cbox, sides & mask);
    }
    (*tbl).data.sides = sides as libc::c_uchar;
    (*tbl).data.box_0 = pos;
}
unsafe extern "C" fn size_html_tbl(
    mut g: *mut graph_t,
    mut tbl: *mut htmltbl_t,
    mut parent: *mut htmlcell_t,
    mut env: *mut htmlenv_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut wd: libc::c_int = 0;
    let mut ht: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    static mut savef: textfont_t = textfont_t {
        name: 0 as *mut libc::c_char,
        color: 0 as *mut libc::c_char,
        postscript_alias: 0 as *mut PostscriptAlias,
        size: 0.,
        flags_cnt: [0; 4],
        c2rust_padding: [0; 4],
    };
    if !((*tbl).font).is_null() {
        pushFontInfo(env, (*tbl).font, &mut savef);
    }
    let ref mut fresh87 = (*tbl).u.n.parent;
    *fresh87 = parent;
    rv = processTbl(g, tbl, env);
    if (*tbl).data.flags as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int == 0 {
        (*tbl).data.space = 2 as libc::c_int as libc::c_schar;
    }
    if (*tbl).data.flags as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        (*tbl).data.border = 1 as libc::c_int as libc::c_uchar;
    }
    sizeArray(tbl);
    wd = ((*tbl).cc + 1 as libc::c_int) * (*tbl).data.space as libc::c_int
        + 2 as libc::c_int * (*tbl).data.border as libc::c_int;
    ht = ((*tbl).rc + 1 as libc::c_int) * (*tbl).data.space as libc::c_int
        + 2 as libc::c_int * (*tbl).data.border as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*tbl).cc {
        wd += *((*tbl).widths).offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*tbl).rc {
        ht += *((*tbl).heights).offset(i as isize);
        i += 1;
    }
    if (*tbl).data.flags as libc::c_int & 1 as libc::c_int != 0 {
        if (*tbl).data.width as libc::c_int != 0 && (*tbl).data.height as libc::c_int != 0 {
            if ((*tbl).data.width as libc::c_int) < wd || ((*tbl).data.height as libc::c_int) < ht {
                agerr(
                    AGWARN,
                    b"table size too small for content\n\0" as *const u8 as *const libc::c_char,
                );
                rv = 1 as libc::c_int;
            }
            ht = 0 as libc::c_int;
            wd = ht;
        } else {
            agerr(
                AGWARN,
                b"fixed table size with unspecified width or height\n\0" as *const u8
                    as *const libc::c_char,
            );
            rv = 1 as libc::c_int;
        }
    }
    (*tbl).data.box_0.UR.x = (if wd > (*tbl).data.width as libc::c_int {
        wd
    } else {
        (*tbl).data.width as libc::c_int
    }) as libc::c_double;
    (*tbl).data.box_0.UR.y = (if ht > (*tbl).data.height as libc::c_int {
        ht
    } else {
        (*tbl).data.height as libc::c_int
    }) as libc::c_double;
    if !((*tbl).font).is_null() {
        popFontInfo(env, &mut savef);
    }
    return rv;
}
unsafe extern "C" fn nameOf(mut obj: *mut libc::c_void, mut xb: *mut agxbuf) -> *mut libc::c_char {
    let mut ep: *mut Agedge_t = 0 as *mut Agedge_t;
    match agobjkind(obj) {
        0 => {
            agxbput(xb, agnameof(obj));
        }
        1 => {
            agxbput(xb, agnameof(obj));
        }
        2 => {
            ep = obj as *mut Agedge_t;
            agxbput(
                xb,
                agnameof(
                    (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(1 as libc::c_int as isize)
                    })
                    .node as *mut libc::c_void,
                ),
            );
            agxbput(
                xb,
                agnameof(
                    (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(-(1 as libc::c_int as isize))
                    })
                    .node as *mut libc::c_void,
                ),
            );
            if agisdirected(agraphof(
                (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                })
                .node as *mut libc::c_void,
            )) != 0
            {
                agxbput(xb, b"->\0" as *const u8 as *const libc::c_char);
            } else {
                agxbput(xb, b"--\0" as *const u8 as *const libc::c_char);
            }
        }
        _ => {}
    }
    return agxbuse(xb);
}
unsafe extern "C" fn getPenColor(mut obj: *mut libc::c_void) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str = agget(
        obj,
        b"pencolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        return str;
    } else {
        str = agget(
            obj,
            b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            return str;
        } else {
            return 0 as *mut libc::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_html_label(
    mut obj: *mut libc::c_void,
    mut lp: *mut textlabel_t,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut wd2: libc::c_double = 0.;
    let mut ht2: libc::c_double = 0.;
    let mut box_0: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut g: *mut graph_t = 0 as *mut graph_t;
    let mut lbl: *mut htmllabel_t = 0 as *mut htmllabel_t;
    let mut env: htmlenv_t = htmlenv_t {
        pos: pointf { x: 0., y: 0. },
        finfo: textfont_t {
            name: 0 as *mut libc::c_char,
            color: 0 as *mut libc::c_char,
            postscript_alias: 0 as *mut PostscriptAlias,
            size: 0.,
            flags_cnt: [0; 4],
            c2rust_padding: [0; 4],
        },
        obj: 0 as *mut libc::c_void,
        g: 0 as *mut graph_t,
        imgscale: 0 as *mut libc::c_char,
        objid: 0 as *mut libc::c_char,
        objid_set: false,
    };
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    env.obj = obj;
    match agobjkind(obj) {
        0 => {
            env.g = (*(obj as *mut Agraph_t)).root;
        }
        1 => {
            env.g = agraphof(obj);
        }
        2 => {
            env.g = agraphof(
                (*if ((*(obj as *mut Agedge_t as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    obj as *mut Agedge_t
                } else {
                    (obj as *mut Agedge_t).offset(-(1 as libc::c_int as isize))
                })
                .node as *mut libc::c_void,
            );
        }
        _ => {}
    }
    g = (*env.g).root;
    env.finfo.size = (*lp).fontsize;
    env.finfo.name = (*lp).fontname;
    env.finfo.color = (*lp).fontcolor;
    (env.finfo).set_flags(0 as libc::c_int as libc::c_uint);
    lbl = parseHTML((*lp).text, &mut rv, &mut env);
    if lbl.is_null() {
        let mut xb: agxbuf = agxbuf {
            buf: 0 as *mut libc::c_char,
            ptr: 0 as *mut libc::c_char,
            eptr: 0 as *mut libc::c_char,
            dyna: 0,
        };
        let mut buf: [libc::c_char; 128] = [0; 128];
        agxbinit(
            &mut xb,
            128 as libc::c_int as libc::c_uint,
            buf.as_mut_ptr(),
        );
        (*lp).html = 0 as libc::c_int != 0;
        let ref mut fresh88 = (*lp).text;
        *fresh88 = strdup(nameOf(obj, &mut xb));
        match (*lp).charset {
            1 => {
                s = latin1ToUTF8((*lp).text);
            }
            _ => {
                s = htmlEntityUTF8((*lp).text, env.g);
            }
        }
        free((*lp).text as *mut libc::c_void);
        let ref mut fresh89 = (*lp).text;
        *fresh89 = s;
        make_simple_label(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc,
            lp,
        );
        agxbfree(&mut xb);
        return rv;
    }
    if (*lbl).kind as libc::c_int == 1 as libc::c_int {
        if ((*(*lbl).u.tbl).data.pencolor).is_null() && !(getPenColor(obj)).is_null() {
            let ref mut fresh90 = (*(*lbl).u.tbl).data.pencolor;
            *fresh90 = strdup(getPenColor(obj));
        }
        rv |= size_html_tbl(g, (*lbl).u.tbl, 0 as *mut htmlcell_t, &mut env);
        wd2 = (*(*lbl).u.tbl).data.box_0.UR.x / 2 as libc::c_int as libc::c_double;
        ht2 = (*(*lbl).u.tbl).data.box_0.UR.y / 2 as libc::c_int as libc::c_double;
        box_0 = boxfof(-wd2, -ht2, wd2, ht2);
        pos_html_tbl(
            (*lbl).u.tbl,
            box_0,
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
        );
        (*lp).dimen.x = box_0.UR.x - box_0.LL.x;
        (*lp).dimen.y = box_0.UR.y - box_0.LL.y;
    } else {
        rv |= size_html_txt(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc,
            (*lbl).u.txt,
            &mut env,
        );
        wd2 = (*(*lbl).u.txt).box_0.UR.x / 2 as libc::c_int as libc::c_double;
        ht2 = (*(*lbl).u.txt).box_0.UR.y / 2 as libc::c_int as libc::c_double;
        box_0 = boxfof(-wd2, -ht2, wd2, ht2);
        (*(*lbl).u.txt).box_0 = box_0;
        (*lp).dimen.x = box_0.UR.x - box_0.LL.x;
        (*lp).dimen.y = box_0.UR.y - box_0.LL.y;
    }
    let ref mut fresh91 = (*lp).u.html;
    *fresh91 = lbl;
    if (*lbl).kind as libc::c_int == 1 as libc::c_int {
        free((*lp).text as *mut libc::c_void);
        let ref mut fresh92 = (*lp).text;
        *fresh92 = strdup(b"<TABLE>\0" as *const u8 as *const libc::c_char);
    }
    return rv;
}
