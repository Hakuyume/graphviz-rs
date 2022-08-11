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
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    pub type htmllabel_t;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn free(_: *mut libc::c_void);
    fn drand48() -> libc::c_double;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
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
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Reduce: libc::c_uchar;
    static mut Nop: libc::c_int;
    static mut Epsilon: libc::c_double;
    static mut MaxIter: libc::c_int;
    static mut Ndim: libc::c_int;
    static mut Initial_dist: libc::c_double;
    static mut Damping: libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn test_toggle() -> libc::c_int;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn getdouble(g: *mut graph_t, name: *mut libc::c_char, result: *mut libc::c_double);
    fn checkStart(G: *mut graph_t, nG: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn solve(
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: libc::c_int,
    );
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut Epsilon2: libc::c_double = 0.;
unsafe extern "C" fn fpow32(mut x: libc::c_double) -> libc::c_double {
    x = sqrt(x);
    return x * x * x;
}
unsafe extern "C" fn distvec(
    mut p0: *mut libc::c_double,
    mut p1: *mut libc::c_double,
    mut vec: *mut libc::c_double,
) -> libc::c_double {
    let mut k: libc::c_int = 0;
    let mut dist: libc::c_double = 0.0f64;
    k = 0 as libc::c_int;
    while k < Ndim {
        *vec.offset(k as isize) = *p0.offset(k as isize) - *p1.offset(k as isize);
        dist += *vec.offset(k as isize) * *vec.offset(k as isize);
        k += 1;
    }
    dist = sqrt(dist);
    return dist;
}
#[no_mangle]
pub unsafe extern "C" fn new_array(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut ival: libc::c_double,
) -> *mut *mut libc::c_double {
    let mut rv: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut mem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    rv = gcalloc(
        m as size_t,
        ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut libc::c_double;
    mem = gcalloc(
        (m * n) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        let ref mut fresh0 = *rv.offset(i as isize);
        *fresh0 = mem;
        mem = mem.offset(n as isize);
        j = 0 as libc::c_int;
        while j < n {
            *(*rv.offset(i as isize)).offset(j as isize) = ival;
            j += 1;
        }
        i += 1;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn free_array(mut rv: *mut *mut libc::c_double) {
    if !rv.is_null() {
        free(*rv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(rv as *mut libc::c_void);
    }
}
unsafe extern "C" fn new_3array(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut p: libc::c_int,
    mut ival: libc::c_double,
) -> *mut *mut *mut libc::c_double {
    let mut rv: *mut *mut *mut libc::c_double = 0 as *mut *mut *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    rv = gcalloc(
        (m + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut *mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        let ref mut fresh1 = *rv.offset(i as isize);
        *fresh1 = gcalloc(
            (n + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
        ) as *mut *mut libc::c_double;
        j = 0 as libc::c_int;
        while j < n {
            let ref mut fresh2 = *(*rv.offset(i as isize)).offset(j as isize);
            *fresh2 = gcalloc(
                p as size_t,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            ) as *mut libc::c_double;
            k = 0 as libc::c_int;
            while k < p {
                *(*(*rv.offset(i as isize)).offset(j as isize)).offset(k as isize) = ival;
                k += 1;
            }
            j += 1;
        }
        let ref mut fresh3 = *(*rv.offset(i as isize)).offset(j as isize);
        *fresh3 = 0 as *mut libc::c_double;
        i += 1;
    }
    let ref mut fresh4 = *rv.offset(i as isize);
    *fresh4 = 0 as *mut *mut libc::c_double;
    return rv;
}
unsafe extern "C" fn free_3array(mut rv: *mut *mut *mut libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !rv.is_null() {
        i = 0 as libc::c_int;
        while !(*rv.offset(i as isize)).is_null() {
            j = 0 as libc::c_int;
            while !(*(*rv.offset(i as isize)).offset(j as isize)).is_null() {
                free(*(*rv.offset(i as isize)).offset(j as isize) as *mut libc::c_void);
                j += 1;
            }
            free(*rv.offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        free(rv as *mut libc::c_void);
    }
}
unsafe extern "C" fn lenattr(
    mut e: *mut edge_t,
    mut index: *mut Agsym_t,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if index.is_null() {
        return 1 as libc::c_int;
    }
    s = agxget(e as *mut libc::c_void, index);
    if *s as libc::c_int == '\0' as i32 {
        return 1 as libc::c_int;
    }
    if sscanf(s, b"%lf\0" as *const u8 as *const libc::c_char, val) < 1 as libc::c_int
        || *val < 0 as libc::c_int as libc::c_double
        || *val == 0 as libc::c_int as libc::c_double && Nop == 0
    {
        agerr(
            AGWARN,
            b"bad edge len \"%s\"\0" as *const u8 as *const libc::c_char,
            s,
        );
        return 2 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn degreeKind(
    mut g: *mut graph_t,
    mut n: *mut node_t,
    mut op: *mut *mut node_t,
) -> libc::c_int {
    let mut ep: *mut edge_t = 0 as *mut edge_t;
    let mut deg: libc::c_int = 0 as libc::c_int;
    let mut other: *mut node_t = 0 as *mut node_t;
    ep = agfstedge(g, n);
    while !ep.is_null() {
        if !((*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            ep
        } else {
            ep.offset(-(1 as libc::c_int as isize))
        }))
        .node
            == (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                ep
            } else {
                ep.offset(1 as libc::c_int as isize)
            }))
            .node)
        {
            if deg == 1 as libc::c_int {
                if !((*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(1 as libc::c_int as isize)
                }))
                .node
                    == n
                    && (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(-(1 as libc::c_int as isize))
                    }))
                    .node
                        == other
                    || (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(1 as libc::c_int as isize)
                    }))
                    .node
                        == other
                        && (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            ep
                        } else {
                            ep.offset(-(1 as libc::c_int as isize))
                        }))
                        .node
                            == n)
                {
                    return 2 as libc::c_int;
                }
            } else {
                if (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(1 as libc::c_int as isize)
                }))
                .node
                    == n
                {
                    other = (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(-(1 as libc::c_int as isize))
                    })
                    .node;
                } else {
                    other = (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(1 as libc::c_int as isize)
                    })
                    .node;
                }
                *op = other;
                deg += 1;
            }
        }
        ep = agnxtedge(g, ep, n);
    }
    return deg;
}
unsafe extern "C" fn prune(
    mut G: *mut graph_t,
    mut np: *mut node_t,
    mut next: *mut node_t,
) -> *mut node_t {
    let mut other: *mut node_t = 0 as *mut node_t;
    let mut deg: libc::c_int = 0;
    while !np.is_null() {
        deg = degreeKind(G, np, &mut other);
        if deg == 0 as libc::c_int {
            if next == np {
                next = agnxtnode(G, np);
            }
            agdelete((*G).root, np as *mut libc::c_void);
            np = 0 as *mut node_t;
        } else if deg == 1 as libc::c_int {
            if next == np {
                next = agnxtnode(G, np);
            }
            agdelete((*G).root, np as *mut libc::c_void);
            np = other;
        } else {
            np = 0 as *mut node_t;
        }
    }
    return next;
}
unsafe extern "C" fn setEdgeLen(
    mut G: *mut graph_t,
    mut np: *mut node_t,
    mut lenx: *mut Agsym_t,
    mut dfltlen: libc::c_double,
) -> libc::c_double {
    let mut ep: *mut edge_t = 0 as *mut edge_t;
    let mut total_len: libc::c_double = 0.0f64;
    let mut len: libc::c_double = 0.;
    let mut err: libc::c_int = 0;
    ep = agfstout(G, np);
    while !ep.is_null() {
        err = lenattr(ep, lenx, &mut len);
        if err != 0 {
            if err == 2 as libc::c_int {
                agerr(
                    AGPREV,
                    b" in %s - setting to %.02f\n\0" as *const u8 as *const libc::c_char,
                    agnameof(G as *mut libc::c_void),
                    dfltlen,
                );
            }
            len = dfltlen;
        }
        (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist = len;
        total_len += len;
        ep = agnxtout(G, ep);
    }
    return total_len;
}
#[no_mangle]
pub unsafe extern "C" fn scan_graph_mode(
    mut G: *mut graph_t,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nV: libc::c_int = 0;
    let mut nE: libc::c_int = 0;
    let mut deg: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut np: *mut node_t = 0 as *mut node_t;
    let mut xp: *mut node_t = 0 as *mut node_t;
    let mut other: *mut node_t = 0 as *mut node_t;
    let mut total_len: libc::c_double = 0.0f64;
    let mut dfltlen: libc::c_double = 1.0f64;
    let mut lenx: *mut Agsym_t = 0 as *mut Agsym_t;
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Scanning graph %s, %d nodes\n\0" as *const u8 as *const libc::c_char,
            agnameof(G as *mut libc::c_void),
            agnnodes(G),
        );
    }
    if Reduce != 0 {
        np = agfstnode(G);
        while !np.is_null() {
            xp = agnxtnode(G, np);
            deg = degreeKind(G, np, &mut other);
            if deg == 0 as libc::c_int {
                agdelete((*G).root, np as *mut libc::c_void);
            } else if deg == 1 as libc::c_int {
                agdelete((*G).root, np as *mut libc::c_void);
                xp = prune(G, other, xp);
            }
            np = xp;
        }
    }
    nV = agnnodes(G);
    nE = agnedges(G);
    lenx = agattr(
        G,
        2 as libc::c_int,
        b"len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if mode == 0 as libc::c_int {
        Epsilon = 0.0001f64 * nV as libc::c_double;
        getdouble(
            G,
            b"epsilon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut Epsilon,
        );
        str = agget(
            (*G).root as *mut libc::c_void,
            b"Damping\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() {
            Damping = atof(str);
        } else {
            Damping = 0.99f64;
        }
        let ref mut fresh5 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist;
        *fresh5 = gcalloc(
            (nV + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
        ) as *mut *mut node_t;
        i = 0 as libc::c_int;
        np = agfstnode(G);
        while !np.is_null() {
            let ref mut fresh6 = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .neato_nlist)
                .offset(i as isize);
            *fresh6 = np;
            let fresh7 = i;
            i = i + 1;
            (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id = fresh7;
            (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = -(1 as libc::c_int);
            total_len += setEdgeLen(G, np, lenx, dfltlen);
            np = agnxtnode(G, np);
        }
    } else if mode == 4 as libc::c_int {
        Epsilon = 0.01f64;
        getdouble(
            G,
            b"epsilon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut Epsilon,
        );
        let ref mut fresh8 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist;
        *fresh8 = gcalloc(
            (nV + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
        ) as *mut *mut node_t;
        i = 0 as libc::c_int;
        np = agfstnode(G);
        while !np.is_null() {
            let ref mut fresh9 = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .neato_nlist)
                .offset(i as isize);
            *fresh9 = np;
            let fresh10 = i;
            i = i + 1;
            (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id = fresh10;
            total_len += setEdgeLen(G, np, lenx, dfltlen);
            np = agnxtnode(G, np);
        }
    } else {
        Epsilon = 1e-4f64;
        getdouble(
            G,
            b"epsilon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut Epsilon,
        );
        i = 0 as libc::c_int;
        np = agfstnode(G);
        while !np.is_null() {
            let fresh11 = i;
            i = i + 1;
            (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id = fresh11;
            total_len += setEdgeLen(G, np, lenx, dfltlen);
            np = agnxtnode(G, np);
        }
    }
    str = agget(
        G as *mut libc::c_void,
        b"defaultdist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        Initial_dist = if Epsilon > atof(str) {
            Epsilon
        } else {
            atof(str)
        };
    } else {
        Initial_dist = total_len
            / (if nE > 0 as libc::c_int {
                nE
            } else {
                1 as libc::c_int
            }) as libc::c_double
            * sqrt(nV as libc::c_double)
            + 1 as libc::c_int as libc::c_double;
    }
    if Nop == 0 && mode == 0 as libc::c_int {
        let ref mut fresh12 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist;
        *fresh12 = new_array(nV, nV, Initial_dist);
        let ref mut fresh13 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).spring;
        *fresh13 = new_array(nV, nV, 1.0f64);
        let ref mut fresh14 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t;
        *fresh14 = new_array(nV, Ndim, 1.0f64);
        let ref mut fresh15 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t;
        *fresh15 = new_3array(nV, nV, Ndim, 0.0f64);
    }
    return nV;
}
#[no_mangle]
pub unsafe extern "C" fn scan_graph(mut g: *mut graph_t) -> libc::c_int {
    return scan_graph_mode(g, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn free_scan_graph(mut g: *mut graph_t) {
    free((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist as *mut libc::c_void);
    if Nop == 0 {
        free_array((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist);
        free_array((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).spring);
        free_array((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t);
        free_3array((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).t);
        let ref mut fresh16 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).t;
        *fresh16 = 0 as *mut *mut *mut libc::c_double;
    }
}
#[no_mangle]
pub unsafe extern "C" fn jitter_d(mut np: *mut node_t, mut nG: libc::c_int, mut n: libc::c_int) {
    let mut k: libc::c_int = 0;
    k = n;
    while k < Ndim {
        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos).offset(k as isize) =
            nG as libc::c_double * drand48();
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn jitter3d(mut np: *mut node_t, mut nG: libc::c_int) {
    jitter_d(np, nG, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn randompos(mut np: *mut node_t, mut nG: libc::c_int) {
    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(0 as libc::c_int as isize) = nG as libc::c_double * drand48();
    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(1 as libc::c_int as isize) = nG as libc::c_double * drand48();
    if Ndim > 2 as libc::c_int {
        jitter3d(np, nG);
    }
}
#[no_mangle]
pub unsafe extern "C" fn initial_positions(mut G: *mut graph_t, mut nG: libc::c_int) {
    let mut init: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut np: *mut node_t = 0 as *mut node_t;
    static mut once: libc::c_int = 0 as libc::c_int;
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Setting initial positions\n\0" as *const u8 as *const libc::c_char,
        );
    }
    init = checkStart(G, nG, 2 as libc::c_int);
    if init == 1 as libc::c_int {
        return;
    }
    if init == 0 as libc::c_int && once == 0 as libc::c_int {
        agerr(
            AGWARN,
            b"start=0 not supported with mode=self - ignored\n\0" as *const u8
                as *const libc::c_char,
        );
        once = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    loop {
        np =
            *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(i as isize);
        if np.is_null() {
            break;
        }
        if !((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned as libc::c_int
            > 0 as libc::c_int)
        {
            randompos(np, 1 as libc::c_int);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn diffeq_model(mut G: *mut graph_t, mut nG: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dist: libc::c_double = 0.;
    let mut D: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut K: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut del: [libc::c_double; 10] = [0.; 10];
    let mut f: libc::c_double = 0.;
    let mut vi: *mut node_t = 0 as *mut node_t;
    let mut vj: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Setting up spring model: \0" as *const u8 as *const libc::c_char,
        );
        start_timer();
    }
    K = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).spring;
    D = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist;
    i = 0 as libc::c_int;
    while i < nG {
        j = 0 as libc::c_int;
        while j < i {
            f = 1.0f64
                / (*(*D.offset(i as isize)).offset(j as isize)
                    * *(*D.offset(i as isize)).offset(j as isize));
            e = agedge(
                G,
                *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
                    .offset(i as isize),
                *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
                    .offset(j as isize),
                0 as *mut libc::c_char,
                0 as libc::c_int,
            );
            if !e.is_null() {
                f = f * (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).factor;
            }
            let ref mut fresh17 = *(*K.offset(j as isize)).offset(i as isize);
            *fresh17 = f;
            *(*K.offset(i as isize)).offset(j as isize) = *fresh17;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nG {
        k = 0 as libc::c_int;
        while k < Ndim {
            *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t).offset(i as isize))
                .offset(k as isize) = 0.0f64;
            k += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        vi =
            *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(i as isize);
        if vi.is_null() {
            break;
        }
        j = 0 as libc::c_int;
        while j < nG {
            if !(i == j) {
                vj = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
                    .offset(j as isize);
                dist = distvec(
                    (*((*(vi as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos,
                    (*((*(vj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos,
                    del.as_mut_ptr(),
                );
                k = 0 as libc::c_int;
                while k < Ndim {
                    *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                        .offset(i as isize))
                    .offset(j as isize))
                    .offset(k as isize) =
                        *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).spring)
                            .offset(i as isize))
                        .offset(j as isize)
                            * (del[k as usize]
                                - *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
                                    .offset(i as isize))
                                .offset(j as isize)
                                    * del[k as usize]
                                    / dist);
                    *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t)
                        .offset(i as isize))
                    .offset(k as isize) +=
                        *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                            .offset(i as isize))
                        .offset(j as isize))
                        .offset(k as isize);
                    k += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"%.2f sec\n\0" as *const u8 as *const libc::c_char,
            elapsed_sec(),
        );
    }
}
unsafe extern "C" fn total_e(mut G: *mut graph_t, mut nG: libc::c_int) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut e: libc::c_double = 0.0f64;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut ip: *mut node_t = 0 as *mut node_t;
    let mut jp: *mut node_t = 0 as *mut node_t;
    i = 0 as libc::c_int;
    while i < nG - 1 as libc::c_int {
        ip =
            *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(i as isize);
        j = i + 1 as libc::c_int;
        while j < nG {
            jp = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
                .offset(j as isize);
            t0 = 0.0f64;
            d = 0 as libc::c_int;
            while d < Ndim {
                t1 = *((*((*(ip as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(d as isize)
                    - *((*((*(jp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(d as isize);
                t0 += t1 * t1;
                d += 1;
            }
            e = e + *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).spring)
                .offset(i as isize))
            .offset(j as isize)
                * (t0
                    + *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
                        .offset(i as isize))
                    .offset(j as isize)
                        * *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
                            .offset(i as isize))
                        .offset(j as isize)
                    - 2.0f64
                        * *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
                            .offset(i as isize))
                        .offset(j as isize)
                        * sqrt(t0));
            j += 1;
        }
        i += 1;
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn solve_model(mut G: *mut graph_t, mut nG: libc::c_int) {
    let mut np: *mut node_t = 0 as *mut node_t;
    Epsilon2 = Epsilon * Epsilon;
    loop {
        np = choose_node(G, nG);
        if np.is_null() {
            break;
        }
        move_node(G, nG, np);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"\nfinal e = %f\0" as *const u8 as *const libc::c_char,
            total_e(G, nG),
        );
        fprintf(
            stderr,
            b" %d%s iterations %.2f sec\n\0" as *const u8 as *const libc::c_char,
            (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).move_0,
            if (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).move_0 == MaxIter {
                b"!\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            elapsed_sec(),
        );
    }
    if (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).move_0 == MaxIter {
        agerr(
            AGWARN,
            b"Max. iterations (%d) reached on graph %s\n\0" as *const u8 as *const libc::c_char,
            MaxIter,
            agnameof(G as *mut libc::c_void),
        );
    }
}
unsafe extern "C" fn update_arrays(mut G: *mut graph_t, mut nG: libc::c_int, mut i: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut del: [libc::c_double; 10] = [0.; 10];
    let mut dist: libc::c_double = 0.;
    let mut old: libc::c_double = 0.;
    let mut vi: *mut node_t = 0 as *mut node_t;
    let mut vj: *mut node_t = 0 as *mut node_t;
    vi = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(i as isize);
    k = 0 as libc::c_int;
    while k < Ndim {
        *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t).offset(i as isize))
            .offset(k as isize) = 0.0f64;
        k += 1;
    }
    j = 0 as libc::c_int;
    while j < nG {
        if !(i == j) {
            vj = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
                .offset(j as isize);
            dist = distvec(
                (*((*(vi as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos,
                (*((*(vj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos,
                del.as_mut_ptr(),
            );
            k = 0 as libc::c_int;
            while k < Ndim {
                old = *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                    .offset(i as isize))
                .offset(j as isize))
                .offset(k as isize);
                *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                    .offset(i as isize))
                .offset(j as isize))
                .offset(k as isize) = *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .spring)
                    .offset(i as isize))
                .offset(j as isize)
                    * (del[k as usize]
                        - *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
                            .offset(i as isize))
                        .offset(j as isize)
                            * del[k as usize]
                            / dist);
                *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t)
                    .offset(i as isize))
                .offset(k as isize) +=
                    *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                        .offset(i as isize))
                    .offset(j as isize))
                    .offset(k as isize);
                old = *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                    .offset(j as isize))
                .offset(i as isize))
                .offset(k as isize);
                *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                    .offset(j as isize))
                .offset(i as isize))
                .offset(k as isize) =
                    -*(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                        .offset(i as isize))
                    .offset(j as isize))
                    .offset(k as isize);
                *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t)
                    .offset(j as isize))
                .offset(k as isize) +=
                    *(*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).t)
                        .offset(j as isize))
                    .offset(i as isize))
                    .offset(k as isize)
                        - old;
                k += 1;
            }
        }
        j += 1;
    }
}
unsafe extern "C" fn D2E(
    mut G: *mut graph_t,
    mut nG: libc::c_int,
    mut n: libc::c_int,
    mut M: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut vi: *mut node_t = 0 as *mut node_t;
    let mut vn: *mut node_t = 0 as *mut node_t;
    let mut scale: libc::c_double = 0.;
    let mut sq: libc::c_double = 0.;
    let mut t: [libc::c_double; 10] = [0.; 10];
    let mut K: *mut *mut libc::c_double =
        (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).spring;
    let mut D: *mut *mut libc::c_double =
        (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist;
    vn = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(n as isize);
    l = 0 as libc::c_int;
    while l < Ndim {
        k = 0 as libc::c_int;
        while k < Ndim {
            *M.offset((l * Ndim + k) as isize) = 0.0f64;
            k += 1;
        }
        l += 1;
    }
    i = 0 as libc::c_int;
    while i < nG {
        if !(n == i) {
            vi = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
                .offset(i as isize);
            sq = 0.0f64;
            k = 0 as libc::c_int;
            while k < Ndim {
                t[k as usize] = *((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(k as isize)
                    - *((*((*(vi as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(k as isize);
                sq += t[k as usize] * t[k as usize];
                k += 1;
            }
            scale = 1 as libc::c_int as libc::c_double / fpow32(sq);
            k = 0 as libc::c_int;
            while k < Ndim {
                l = 0 as libc::c_int;
                while l < k {
                    *M.offset((l * Ndim + k) as isize) += *(*K.offset(n as isize))
                        .offset(i as isize)
                        * *(*D.offset(n as isize)).offset(i as isize)
                        * t[k as usize]
                        * t[l as usize]
                        * scale;
                    l += 1;
                }
                *M.offset((k * Ndim + k) as isize) += *(*K.offset(n as isize)).offset(i as isize)
                    * (1.0f64
                        - *(*D.offset(n as isize)).offset(i as isize)
                            * (sq - t[k as usize] * t[k as usize])
                            * scale);
                k += 1;
            }
        }
        i += 1;
    }
    k = 1 as libc::c_int;
    while k < Ndim {
        l = 0 as libc::c_int;
        while l < k {
            *M.offset((k * Ndim + l) as isize) = *M.offset((l * Ndim + k) as isize);
            l += 1;
        }
        k += 1;
    }
}
unsafe extern "C" fn choose_node(mut G: *mut graph_t, mut nG: libc::c_int) -> *mut Agnode_t {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    let mut choice: *mut node_t = 0 as *mut node_t;
    let mut np: *mut node_t = 0 as *mut node_t;
    static mut cnt: libc::c_int = 0 as libc::c_int;
    cnt += 1;
    if (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).move_0 >= MaxIter {
        return 0 as *mut Agnode_t;
    }
    max = 0.0f64;
    choice = 0 as *mut node_t;
    i = 0 as libc::c_int;
    while i < nG {
        np =
            *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(i as isize);
        if !((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned as libc::c_int
            > 1 as libc::c_int)
        {
            m = 0.0f64;
            k = 0 as libc::c_int;
            while k < Ndim {
                m += *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t)
                    .offset(i as isize))
                .offset(k as isize)
                    * *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t)
                        .offset(i as isize))
                    .offset(k as isize);
                k += 1;
            }
            if m > max {
                choice = np;
                max = m;
            }
        }
        i += 1;
    }
    if max < Epsilon2 {
        choice = 0 as *mut node_t;
    } else if Verbose as libc::c_int != 0 && cnt % 100 as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"%.3f \0" as *const u8 as *const libc::c_char,
            sqrt(max),
        );
        if cnt % 1000 as libc::c_int == 0 as libc::c_int {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return choice;
}
unsafe extern "C" fn move_node(mut G: *mut graph_t, mut nG: libc::c_int, mut n: *mut node_t) {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    static mut a: *mut libc::c_double = 0 as *const libc::c_double as *mut libc::c_double;
    static mut b: [libc::c_double; 10] = [0.; 10];
    static mut c: [libc::c_double; 10] = [0.; 10];
    m = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id;
    a = if !a.is_null() {
        grealloc(
            a as *mut libc::c_void,
            ((Ndim * Ndim) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    } else {
        gmalloc(
            ((Ndim * Ndim) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    };
    D2E(G, nG, m, a);
    i = 0 as libc::c_int;
    while i < Ndim {
        c[i as usize] = -*(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).sum_t)
            .offset(m as isize))
        .offset(i as isize);
        i += 1;
    }
    solve(a, b.as_mut_ptr(), c.as_mut_ptr(), Ndim);
    i = 0 as libc::c_int;
    while i < Ndim {
        b[i as usize] = (Damping
            + 2 as libc::c_int as libc::c_double
                * (1 as libc::c_int as libc::c_double - Damping)
                * drand48())
            * b[i as usize];
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos).offset(i as isize) +=
            b[i as usize];
        i += 1;
    }
    let ref mut fresh18 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).move_0;
    *fresh18 += 1;
    update_arrays(G, nG, m);
    if test_toggle() != 0 {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < Ndim {
            sum += fabs(b[i as usize]);
            i += 1;
        }
        sum = sqrt(sum);
        fprintf(
            stderr,
            b"%s %.3f\n\0" as *const u8 as *const libc::c_char,
            agnameof(n as *mut libc::c_void),
            sum,
        );
    }
}
static mut Heap: *mut *mut node_t = 0 as *const *mut node_t as *mut *mut node_t;
static mut Heapsize: libc::c_int = 0;
static mut Src: *mut node_t = 0 as *const node_t as *mut node_t;
unsafe extern "C" fn heapup(mut v: *mut node_t) {
    let mut i: libc::c_int = 0;
    let mut par: libc::c_int = 0;
    let mut u: *mut node_t = 0 as *mut node_t;
    i = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex;
    while i > 0 as libc::c_int {
        par = (i - 1 as libc::c_int) / 2 as libc::c_int;
        u = *Heap.offset(par as isize);
        if (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist
            <= (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist
        {
            break;
        }
        let ref mut fresh19 = *Heap.offset(par as isize);
        *fresh19 = v;
        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = par;
        let ref mut fresh20 = *Heap.offset(i as isize);
        *fresh20 = u;
        (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = i;
        i = par;
    }
}
unsafe extern "C" fn heapdown(mut v: *mut node_t) {
    let mut i: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut u: *mut node_t = 0 as *mut node_t;
    i = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex;
    loop {
        left = 2 as libc::c_int * i + 1 as libc::c_int;
        if !(left < Heapsize) {
            break;
        }
        right = left + 1 as libc::c_int;
        if right < Heapsize
            && (*((*(*Heap.offset(right as isize) as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist
                < (*((*(*Heap.offset(left as isize) as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .dist
        {
            c = right;
        } else {
            c = left;
        }
        u = *Heap.offset(c as isize);
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist
            <= (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist
        {
            break;
        }
        let ref mut fresh21 = *Heap.offset(c as isize);
        *fresh21 = v;
        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = c;
        let ref mut fresh22 = *Heap.offset(i as isize);
        *fresh22 = u;
        (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = i;
        i = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn neato_enqueue(mut v: *mut node_t) {
    let mut i: libc::c_int = 0;
    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex < 0 as libc::c_int {
    } else {
        __assert_fail(
            b"ND_heapindex(v) < 0\0" as *const u8 as *const libc::c_char,
            b"stuff.c\0" as *const u8 as *const libc::c_char,
            627 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"void neato_enqueue(node_t *)\0",
            ))
            .as_ptr(),
        );
    }
    let fresh23 = Heapsize;
    Heapsize = Heapsize + 1;
    i = fresh23;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = i;
    let ref mut fresh24 = *Heap.offset(i as isize);
    *fresh24 = v;
    if i > 0 as libc::c_int {
        heapup(v);
    }
}
#[no_mangle]
pub unsafe extern "C" fn neato_dequeue() -> *mut node_t {
    let mut i: libc::c_int = 0;
    let mut rv: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    if Heapsize == 0 as libc::c_int {
        return 0 as *mut node_t;
    }
    rv = *Heap.offset(0 as libc::c_int as isize);
    Heapsize -= 1;
    i = Heapsize;
    v = *Heap.offset(i as isize);
    let ref mut fresh25 = *Heap.offset(0 as libc::c_int as isize);
    *fresh25 = v;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = 0 as libc::c_int;
    if i > 1 as libc::c_int {
        heapdown(v);
    }
    (*((*(rv as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex = -(1 as libc::c_int);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn shortest_path(mut G: *mut graph_t, mut nG: libc::c_int) {
    let mut v: *mut node_t = 0 as *mut node_t;
    Heap = gcalloc(
        (nG + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
    ) as *mut *mut node_t;
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Calculating shortest paths: \0" as *const u8 as *const libc::c_char,
        );
        start_timer();
    }
    v = agfstnode(G);
    while !v.is_null() {
        s1(G, v);
        v = agnxtnode(G, v);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"%.2f sec\n\0" as *const u8 as *const libc::c_char,
            elapsed_sec(),
        );
    }
    free(Heap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn s1(mut G: *mut graph_t, mut node: *mut node_t) {
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut t: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    t = 0 as libc::c_int;
    loop {
        v = *((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist).offset(t as isize);
        if v.is_null() {
            break;
        }
        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist = Initial_dist;
        t += 1;
    }
    Src = node;
    (*((*(Src as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist =
        0 as libc::c_int as libc::c_double;
    (*((*(Src as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops = 0 as libc::c_int;
    neato_enqueue(Src);
    loop {
        v = neato_dequeue();
        if v.is_null() {
            break;
        }
        if v != Src {
            make_spring(
                G,
                Src,
                v,
                (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist,
            );
        }
        e = agfstedge(G, v);
        while !e.is_null() {
            u = (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
            .node;
            if u == v {
                u = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
            }
            f = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist
                + (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist;
            if (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist > f {
                (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).dist = f;
                if (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).heapindex
                    >= 0 as libc::c_int
                {
                    heapup(u);
                } else {
                    (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops =
                        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops
                            + 1 as libc::c_int;
                    neato_enqueue(u);
                }
            }
            e = agnxtedge(G, e, v);
        }
    }
}
unsafe extern "C" fn make_spring(
    mut G: *mut graph_t,
    mut u: *mut node_t,
    mut v: *mut node_t,
    mut f: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id;
    j = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id;
    let ref mut fresh26 = *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
        .offset(j as isize))
    .offset(i as isize);
    *fresh26 = f;
    *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist).offset(i as isize))
        .offset(j as isize) = *fresh26;
}
