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
    pub type htmllabel_t;
    pub type _grid;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn drand48() -> libc::c_double;
    fn srand48(__seedval: libc::c_long);
    fn time(__timer: *mut time_t) -> time_t;
    fn getpid() -> __pid_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut fdp_parms: *mut fdpParms_s;
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
    fn mkGrid(_: libc::c_int) -> *mut Grid;
    fn adjustGrid(g: *mut Grid, nnodes: libc::c_int);
    fn clearGrid(_: *mut Grid);
    fn addGrid(_: *mut Grid, _: libc::c_int, _: libc::c_int, _: *mut Agnode_t);
    fn walkGrid(
        _: *mut Grid,
        _: Option<unsafe extern "C" fn(*mut Dt_t, *mut cell, *mut Grid) -> libc::c_int>,
    );
    fn findGrid(_: *mut Grid, _: libc::c_int, _: libc::c_int) -> *mut cell;
    fn delGrid(_: *mut Grid);
    fn setSeed(_: *mut graph_t, dflt: libc::c_int, seedp: *mut libc::c_long) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdpParms_s {
    pub useGrid: libc::c_int,
    pub useNew: libc::c_int,
    pub numIters: libc::c_int,
    pub unscaled: libc::c_int,
    pub C: libc::c_double,
    pub Tfact: libc::c_double,
    pub K: libc::c_double,
    pub T0: libc::c_double,
}
pub type Grid = _grid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _node_list {
    pub node: *mut Agnode_t,
    pub next: *mut _node_list,
}
pub type node_list = _node_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gridpt {
    pub i: libc::c_int,
    pub j: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub p: gridpt,
    pub nodes: *mut node_list,
    pub link: Dtlink_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bport_s {
    pub e: *mut edge_t,
    pub n: *mut node_t,
    pub alpha: libc::c_double,
}
pub type bport_t = bport_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdata {
    pub ports: *mut bport_t,
    pub nports: libc::c_int,
    pub bb: boxf,
    pub flags: libc::c_int,
    pub level: libc::c_int,
    pub parent: *mut graph_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dndata {
    pub deg: libc::c_int,
    pub wdeg: libc::c_int,
    pub dn: *mut node_t,
    pub disp: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xparams {
    pub numIters: libc::c_int,
    pub T0: libc::c_double,
    pub K: libc::c_double,
    pub C: libc::c_double,
    pub loopcnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parms_t {
    pub useGrid: libc::c_int,
    pub useNew: libc::c_int,
    pub seed: libc::c_long,
    pub numIters: libc::c_int,
    pub maxIters: libc::c_int,
    pub unscaled: libc::c_int,
    pub C: libc::c_double,
    pub Tfact: libc::c_double,
    pub K: libc::c_double,
    pub T0: libc::c_double,
    pub smode: libc::c_int,
    pub Cell: libc::c_double,
    pub Cell2: libc::c_double,
    pub K2: libc::c_double,
    pub Wd: libc::c_double,
    pub Ht: libc::c_double,
    pub Wd2: libc::c_double,
    pub Ht2: libc::c_double,
    pub pass1: libc::c_int,
    pub loopcnt: libc::c_int,
}
static mut parms: parms_t = parms_t {
    useGrid: 0,
    useNew: 0,
    seed: 0,
    numIters: 0,
    maxIters: 0,
    unscaled: 0,
    C: 0.,
    Tfact: 0.,
    K: 0.,
    T0: 0.,
    smode: 0,
    Cell: 0.,
    Cell2: 0.,
    K2: 0.,
    Wd: 0.,
    Ht: 0.,
    Wd2: 0.,
    Ht2: 0.,
    pass1: 0,
    loopcnt: 0,
};
unsafe extern "C" fn cool(mut t: libc::c_int) -> libc::c_double {
    return parms.T0 * (parms.maxIters - t) as libc::c_double / parms.maxIters as libc::c_double;
}
unsafe extern "C" fn reset_params() {
    parms.T0 = -1.0f64;
}
unsafe extern "C" fn init_params(mut g: *mut graph_t, mut xpms: *mut xparams) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if parms.T0 == -1.0f64 {
        let mut nnodes: libc::c_int = agnnodes(g);
        parms.T0 = parms.Tfact * parms.K * sqrt(nnodes as libc::c_double)
            / 5 as libc::c_int as libc::c_double;
        ret = 1 as libc::c_int;
    }
    (*xpms).T0 = cool(parms.pass1);
    (*xpms).K = parms.K;
    (*xpms).C = parms.C;
    (*xpms).numIters = parms.maxIters - parms.pass1;
    if parms.numIters >= 0 as libc::c_int {
        if parms.numIters <= parms.pass1 {
            parms.loopcnt = parms.numIters;
            (*xpms).loopcnt = 0 as libc::c_int;
        } else if parms.numIters <= parms.maxIters {
            parms.loopcnt = parms.pass1;
            (*xpms).loopcnt = parms.numIters - parms.pass1;
        }
    } else {
        parms.loopcnt = parms.pass1;
        (*xpms).loopcnt = (*xpms).numIters;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fdp_initParams(mut g: *mut graph_t) {
    parms.useGrid = (*fdp_parms).useGrid;
    parms.useNew = (*fdp_parms).useNew;
    parms.numIters = (*fdp_parms).numIters;
    parms.unscaled = (*fdp_parms).unscaled;
    parms.Cell = 0.0f64;
    parms.C = (*fdp_parms).C;
    parms.Tfact = (*fdp_parms).Tfact;
    parms.maxIters = late_int(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"maxiter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        600 as libc::c_int,
        0 as libc::c_int,
    );
    parms.K = late_double(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        0.3f64,
        0.0f64,
    );
    (*fdp_parms).K = parms.K;
    if (*fdp_parms).T0 == -1.0f64 {
        parms.T0 = late_double(
            g as *mut libc::c_void,
            agattr(
                g,
                0 as libc::c_int,
                b"T0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            ),
            -1.0f64,
            0.0f64,
        );
    } else {
        parms.T0 = (*fdp_parms).T0;
    }
    parms.seed = 1 as libc::c_int as libc::c_long;
    parms.smode = setSeed(g, 2 as libc::c_int, &mut parms.seed);
    if parms.smode == 0 as libc::c_int {
        agerr(
            AGWARN,
            b"fdp does not support start=self - ignoring\n\0" as *const u8 as *const libc::c_char,
        );
        parms.seed = 2 as libc::c_int as libc::c_long;
    }
    parms.pass1 = parms.unscaled * parms.maxIters / 100 as libc::c_int;
    parms.K2 = parms.K * parms.K;
    if parms.useGrid != 0 {
        if parms.Cell <= 0.0f64 {
            parms.Cell = 3 as libc::c_int as libc::c_double * parms.K;
        }
        parms.Cell2 = parms.Cell * parms.Cell;
    }
}
unsafe extern "C" fn doRep(
    mut p: *mut node_t,
    mut q: *mut node_t,
    mut xdelta: libc::c_double,
    mut ydelta: libc::c_double,
    mut dist2: libc::c_double,
) {
    let mut force: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    while dist2 == 0.0f64 {
        xdelta = (5 as libc::c_int - rand() % 10 as libc::c_int) as libc::c_double;
        ydelta = (5 as libc::c_int - rand() % 10 as libc::c_int) as libc::c_double;
        dist2 = xdelta * xdelta + ydelta * ydelta;
    }
    if parms.useNew != 0 {
        dist = sqrt(dist2);
        force = parms.K2 / (dist * dist2);
    } else {
        force = parms.K2 / dist2;
    }
    if ((*((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).dn).is_null()
        && ((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null()
        && (((*((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).dn)
            .is_null()
            && ((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null())
    {
        force *= 10.0f64;
    }
    (*((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [0 as libc::c_int as usize] += xdelta * force;
    (*((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [1 as libc::c_int as usize] += ydelta * force;
    (*((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [0 as libc::c_int as usize] -= xdelta * force;
    (*((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [1 as libc::c_int as usize] -= ydelta * force;
}
unsafe extern "C" fn applyRep(mut p: *mut Agnode_t, mut q: *mut Agnode_t) {
    let mut xdelta: libc::c_double = 0.;
    let mut ydelta: libc::c_double = 0.;
    xdelta = *((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(0 as libc::c_int as isize)
        - *((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize);
    ydelta = *((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(1 as libc::c_int as isize)
        - *((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize);
    doRep(p, q, xdelta, ydelta, xdelta * xdelta + ydelta * ydelta);
}
unsafe extern "C" fn doNeighbor(
    mut grid: *mut Grid,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut nodes: *mut node_list,
) {
    let mut cellp: *mut cell = findGrid(grid, i, j);
    let mut qs: *mut node_list = 0 as *mut node_list;
    let mut p: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut q: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut xdelta: libc::c_double = 0.;
    let mut ydelta: libc::c_double = 0.;
    let mut dist2: libc::c_double = 0.;
    if !cellp.is_null() {
        while !nodes.is_null() {
            p = (*nodes).node;
            qs = (*cellp).nodes;
            while !qs.is_null() {
                q = (*qs).node;
                xdelta = *((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize)
                    - *((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize);
                ydelta = *((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize)
                    - *((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize);
                dist2 = xdelta * xdelta + ydelta * ydelta;
                if dist2 < parms.Cell2 {
                    doRep(p, q, xdelta, ydelta, dist2);
                }
                qs = (*qs).next;
            }
            nodes = (*nodes).next;
        }
    }
}
unsafe extern "C" fn gridRepulse(
    mut dt: *mut Dt_t,
    mut cellp: *mut cell,
    mut grid: *mut Grid,
) -> libc::c_int {
    let mut nodes: *mut node_list = (*cellp).nodes;
    let mut i: libc::c_int = (*cellp).p.i;
    let mut j: libc::c_int = (*cellp).p.j;
    let mut p: *mut node_list = 0 as *mut node_list;
    let mut q: *mut node_list = 0 as *mut node_list;
    p = nodes;
    while !p.is_null() {
        q = nodes;
        while !q.is_null() {
            if p != q {
                applyRep((*p).node, (*q).node);
            }
            q = (*q).next;
        }
        p = (*p).next;
    }
    doNeighbor(grid, i - 1 as libc::c_int, j - 1 as libc::c_int, nodes);
    doNeighbor(grid, i - 1 as libc::c_int, j, nodes);
    doNeighbor(grid, i - 1 as libc::c_int, j + 1 as libc::c_int, nodes);
    doNeighbor(grid, i, j - 1 as libc::c_int, nodes);
    doNeighbor(grid, i, j + 1 as libc::c_int, nodes);
    doNeighbor(grid, i + 1 as libc::c_int, j - 1 as libc::c_int, nodes);
    doNeighbor(grid, i + 1 as libc::c_int, j, nodes);
    doNeighbor(grid, i + 1 as libc::c_int, j + 1 as libc::c_int, nodes);
    return 0 as libc::c_int;
}
unsafe extern "C" fn applyAttr(mut p: *mut Agnode_t, mut q: *mut Agnode_t, mut e: *mut Agedge_t) {
    let mut xdelta: libc::c_double = 0.;
    let mut ydelta: libc::c_double = 0.;
    let mut force: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut dist2: libc::c_double = 0.;
    xdelta = *((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(0 as libc::c_int as isize)
        - *((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize);
    ydelta = *((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(1 as libc::c_int as isize)
        - *((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize);
    dist2 = xdelta * xdelta + ydelta * ydelta;
    while dist2 == 0.0f64 {
        xdelta = (5 as libc::c_int - rand() % 10 as libc::c_int) as libc::c_double;
        ydelta = (5 as libc::c_int - rand() % 10 as libc::c_int) as libc::c_double;
        dist2 = xdelta * xdelta + ydelta * ydelta;
    }
    dist = sqrt(dist2);
    if parms.useNew != 0 {
        force = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).factor
            * (dist - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist)
            / dist;
    } else {
        force = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).factor * dist
            / (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist;
    }
    (*((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [0 as libc::c_int as usize] -= xdelta * force;
    (*((*((*(q as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [1 as libc::c_int as usize] -= ydelta * force;
    (*((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [0 as libc::c_int as usize] += xdelta * force;
    (*((*((*(p as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
        [1 as libc::c_int as usize] += ydelta * force;
}
unsafe extern "C" fn updatePos(
    mut g: *mut Agraph_t,
    mut temp: libc::c_double,
    mut pp: *mut bport_t,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut temp2: libc::c_double = 0.;
    let mut len2: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    temp2 = temp * temp;
    n = agfstnode(g);
    while !n.is_null() {
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned as libc::c_int
            & 2 as libc::c_int
            != 0)
        {
            dx = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
                [0 as libc::c_int as usize];
            dy = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
                [1 as libc::c_int as usize];
            len2 = dx * dx + dy * dy;
            if len2 < temp2 {
                x = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize)
                    + dx;
                y = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize)
                    + dy;
            } else {
                let mut fact: libc::c_double = temp / sqrt(len2);
                x = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize)
                    + dx * fact;
                y = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize)
                    + dy * fact;
            }
            if !pp.is_null() {
                d = sqrt(x * x / parms.Wd2 + y * y / parms.Ht2);
                if ((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata))
                    .dn)
                    .is_null()
                    && ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null()
                {
                    *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize) = x / d;
                    *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize) = y / d;
                } else if d >= 1.0f64 {
                    *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize) = 0.95f64 * x / d;
                    *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize) = 0.95f64 * y / d;
                } else {
                    *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize) = x;
                    *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize) = y;
                }
            } else {
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize) = x;
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize) = y;
            }
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn gAdjust(
    mut g: *mut Agraph_t,
    mut temp: libc::c_double,
    mut pp: *mut bport_t,
    mut grid: *mut Grid,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    if temp <= 0.0f64 {
        return;
    }
    clearGrid(grid);
    n = agfstnode(g);
    while !n.is_null() {
        let ref mut fresh0 = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
            as *mut dndata))
            .disp[1 as libc::c_int as usize];
        *fresh0 = 0 as libc::c_int as libc::c_double;
        (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
            [0 as libc::c_int as usize] = *fresh0;
        addGrid(
            grid,
            floor(
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize)
                    / parms.Cell,
            ) as libc::c_int,
            floor(
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize)
                    / parms.Cell,
            ) as libc::c_int,
            n,
        );
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if n != (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
            .node
            {
                applyAttr(
                    n,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    e,
                );
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    walkGrid(
        grid,
        Some(gridRepulse as unsafe extern "C" fn(*mut Dt_t, *mut cell, *mut Grid) -> libc::c_int),
    );
    updatePos(g, temp, pp);
}
unsafe extern "C" fn adjust(mut g: *mut Agraph_t, mut temp: libc::c_double, mut pp: *mut bport_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut n1: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    if temp <= 0.0f64 {
        return;
    }
    n = agfstnode(g);
    while !n.is_null() {
        let ref mut fresh1 = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
            as *mut dndata))
            .disp[1 as libc::c_int as usize];
        *fresh1 = 0 as libc::c_int as libc::c_double;
        (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
            [0 as libc::c_int as usize] = *fresh1;
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        n1 = agnxtnode(g, n);
        while !n1.is_null() {
            applyRep(n, n1);
            n1 = agnxtnode(g, n1);
        }
        e = agfstout(g, n);
        while !e.is_null() {
            if n != (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
            .node
            {
                applyAttr(
                    n,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    e,
                );
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    updatePos(g, temp, pp);
}
unsafe extern "C" fn initPositions(mut g: *mut graph_t, mut pp: *mut bport_t) -> pointf {
    let mut nG: libc::c_int = agnnodes(g)
        - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata)).nports;
    let mut size: libc::c_double = 0.;
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut n_pos: libc::c_int = 0 as libc::c_int;
    let mut bb: boxf = {
        let mut init = boxf {
            LL: {
                let mut init = pointf_s {
                    x: 0 as libc::c_int as libc::c_double,
                    y: 0 as libc::c_int as libc::c_double,
                };
                init
            },
            UR: {
                let mut init = pointf_s {
                    x: 0 as libc::c_int as libc::c_double,
                    y: 0 as libc::c_int as libc::c_double,
                };
                init
            },
        };
        init
    };
    let mut ctr: pointf = pointf { x: 0., y: 0. };
    let mut local_seed: libc::c_long = 0;
    let mut PItimes2: libc::c_double = 3.14159265358979323846f64 * 2.0f64;
    np = agfstnode(g);
    while !np.is_null() {
        if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned != 0 {
            if n_pos != 0 {
                bb.LL.x = if *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize)
                    < bb.LL.x
                {
                    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize)
                } else {
                    bb.LL.x
                };
                bb.LL.y = if *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize)
                    < bb.LL.y
                {
                    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize)
                } else {
                    bb.LL.y
                };
                bb.UR.x = if *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize)
                    > bb.UR.x
                {
                    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize)
                } else {
                    bb.UR.x
                };
                bb.UR.y = if *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize)
                    > bb.UR.y
                {
                    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize)
                } else {
                    bb.UR.y
                };
            } else {
                bb.LL.x = *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize);
                bb.UR.x = bb.LL.x;
                bb.LL.y = *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize);
                bb.UR.y = bb.LL.y;
            }
            n_pos += 1;
        }
        np = agnxtnode(g, np);
    }
    size = parms.K * (sqrt(nG as libc::c_double) + 1.0f64);
    parms.Ht = 1.2f64 * (size / 2.0f64);
    parms.Wd = parms.Ht;
    if n_pos == 1 as libc::c_int {
        ctr.x = bb.LL.x;
        ctr.y = bb.LL.y;
    } else if n_pos > 1 as libc::c_int {
        let mut alpha: libc::c_double = 0.;
        let mut area: libc::c_double = 0.;
        let mut width: libc::c_double = 0.;
        let mut height: libc::c_double = 0.;
        let mut quot: libc::c_double = 0.;
        ctr.x = (bb.LL.x + bb.UR.x) / 2.0f64;
        ctr.y = (bb.LL.y + bb.UR.y) / 2.0f64;
        width = 1.2f64 * (bb.UR.x - bb.LL.x);
        height = 1.2f64 * (bb.UR.y - bb.LL.y);
        area = 4.0f64 * parms.Wd * parms.Ht;
        quot = width * height / area;
        if quot >= 1.0f64 {
            parms.Wd = width / 2.0f64;
            parms.Ht = height / 2.0f64;
        } else if quot > 0.0f64 {
            quot = 2.0f64 * sqrt(quot);
            parms.Wd = width / quot;
            parms.Ht = height / quot;
        } else if width > 0 as libc::c_int as libc::c_double {
            height = area / width;
            parms.Wd = width / 2.0f64;
            parms.Ht = height / 2.0f64;
        } else if height > 0 as libc::c_int as libc::c_double {
            width = area / height;
            parms.Wd = width / 2.0f64;
            parms.Ht = height / 2.0f64;
        }
        alpha = atan2(parms.Ht, parms.Wd);
        parms.Wd = parms.Wd / cos(alpha);
        parms.Ht = parms.Ht / sin(alpha);
    } else {
        ctr.y = 0 as libc::c_int as libc::c_double;
        ctr.x = ctr.y;
    }
    parms.Wd2 = parms.Wd * parms.Wd;
    parms.Ht2 = parms.Ht * parms.Ht;
    if parms.smode == 2 as libc::c_int {
        local_seed = parms.seed;
    } else {
        local_seed = getpid() as libc::c_long ^ time(0 as *mut time_t);
    }
    srand48(local_seed);
    if !pp.is_null() {
        while !((*pp).e).is_null() {
            np = (*pp).n;
            *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) = parms.Wd * cos((*pp).alpha) + ctr.x;
            *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) = parms.Ht * sin((*pp).alpha) + ctr.y;
            (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned =
                1 as libc::c_int as libc::c_uchar;
            pp = pp.offset(1);
        }
        np = agfstnode(g);
        while !np.is_null() {
            if !(((*((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata))
                .dn)
                .is_null()
                && ((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null())
            {
                if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned != 0 {
                    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize) -= ctr.x;
                    *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize) -= ctr.y;
                } else {
                    let mut p: pointf = {
                        let mut init = pointf_s {
                            x: 0.0f64,
                            y: 0.0f64,
                        };
                        init
                    };
                    let mut cnt: libc::c_int = 0 as libc::c_int;
                    let mut op: *mut node_t = 0 as *mut node_t;
                    let mut ep: *mut edge_t = 0 as *mut edge_t;
                    ep = agfstedge(g, np);
                    while !ep.is_null() {
                        if !((*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            ep
                        } else {
                            ep.offset(-(1 as libc::c_int as isize))
                        }))
                        .node
                            == (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                                == 3 as libc::c_int
                            {
                                ep
                            } else {
                                ep.offset(1 as libc::c_int as isize)
                            }))
                            .node)
                        {
                            op = if (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                                == 2 as libc::c_int
                            {
                                ep
                            } else {
                                ep.offset(-(1 as libc::c_int as isize))
                            }))
                            .node
                                == np
                            {
                                (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                                    == 3 as libc::c_int
                                {
                                    ep
                                } else {
                                    ep.offset(1 as libc::c_int as isize)
                                })
                                .node
                            } else {
                                (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                                    == 2 as libc::c_int
                                {
                                    ep
                                } else {
                                    ep.offset(-(1 as libc::c_int as isize))
                                })
                                .node
                            };
                            if (*((*(op as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned
                                as libc::c_int
                                > 0 as libc::c_int
                            {
                                if cnt != 0 {
                                    p.x = (p.x * cnt as libc::c_double
                                        + *((*((*(op as *mut Agobj_t)).data
                                            as *mut Agnodeinfo_t))
                                            .pos)
                                            .offset(0 as libc::c_int as isize))
                                        / (cnt + 1 as libc::c_int) as libc::c_double;
                                    p.y = (p.y * cnt as libc::c_double
                                        + *((*((*(op as *mut Agobj_t)).data
                                            as *mut Agnodeinfo_t))
                                            .pos)
                                            .offset(1 as libc::c_int as isize))
                                        / (cnt + 1 as libc::c_int) as libc::c_double;
                                } else {
                                    p.x = *((*((*(op as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                        .pos)
                                        .offset(0 as libc::c_int as isize);
                                    p.y = *((*((*(op as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                        .pos)
                                        .offset(1 as libc::c_int as isize);
                                }
                                cnt += 1;
                            }
                        }
                        ep = agnxtedge(g, ep, np);
                    }
                    if cnt > 1 as libc::c_int {
                        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                            .offset(0 as libc::c_int as isize) = p.x;
                        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                            .offset(1 as libc::c_int as isize) = p.y;
                    } else if cnt == 1 as libc::c_int {
                        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                            .offset(0 as libc::c_int as isize) = 0.98f64 * p.x + 0.1f64 * ctr.x;
                        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                            .offset(1 as libc::c_int as isize) = 0.9f64 * p.y + 0.1f64 * ctr.y;
                    } else {
                        let mut angle: libc::c_double = PItimes2 * drand48();
                        let mut radius: libc::c_double = 0.9f64 * drand48();
                        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                            .offset(0 as libc::c_int as isize) = radius * parms.Wd * cos(angle);
                        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                            .offset(1 as libc::c_int as isize) = radius * parms.Ht * sin(angle);
                    }
                    (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned =
                        1 as libc::c_int as libc::c_uchar;
                }
            }
            np = agnxtnode(g, np);
        }
    } else if n_pos != 0 {
        np = agfstnode(g);
        while !np.is_null() {
            if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned != 0 {
                *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize) -= ctr.x;
                *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize) -= ctr.y;
            } else {
                *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize) = parms.Wd * (2.0f64 * drand48() - 1.0f64);
                *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize) = parms.Ht * (2.0f64 * drand48() - 1.0f64);
            }
            np = agnxtnode(g, np);
        }
    } else {
        np = agfstnode(g);
        while !np.is_null() {
            *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) = parms.Wd * (2.0f64 * drand48() - 1.0f64);
            *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) = parms.Ht * (2.0f64 * drand48() - 1.0f64);
            np = agnxtnode(g, np);
        }
    }
    return ctr;
}
#[no_mangle]
pub unsafe extern "C" fn dumpstat(mut g: *mut graph_t) {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut l: libc::c_double = 0.;
    let mut max2: libc::c_double = 0.0f64;
    let mut np: *mut node_t = 0 as *mut node_t;
    let mut ep: *mut edge_t = 0 as *mut edge_t;
    np = agfstnode(g);
    while !np.is_null() {
        dx = (*((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
            [0 as libc::c_int as usize];
        dy = (*((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
            [1 as libc::c_int as usize];
        l = dx * dx + dy * dy;
        if l > max2 {
            max2 = l;
        }
        fprintf(
            stderr,
            b"%s: (%f,%f) (%f,%f)\n\0" as *const u8 as *const libc::c_char,
            agnameof(np as *mut libc::c_void),
            *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize),
            *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize),
            (*((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
                [0 as libc::c_int as usize],
            (*((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut dndata)).disp
                [1 as libc::c_int as usize],
        );
        np = agnxtnode(g, np);
    }
    fprintf(
        stderr,
        b"max delta = %f\n\0" as *const u8 as *const libc::c_char,
        sqrt(max2),
    );
    np = agfstnode(g);
    while !np.is_null() {
        ep = agfstout(g, np);
        while !ep.is_null() {
            dx = *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize)
                - *((*((*((*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                }))
                .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .pos)
                    .offset(0 as libc::c_int as isize);
            dy = *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize)
                - *((*((*((*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(-(1 as libc::c_int as isize))
                }))
                .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .pos)
                    .offset(1 as libc::c_int as isize);
            fprintf(
                stderr,
                b"  %s --  %s  (%f)\n\0" as *const u8 as *const libc::c_char,
                agnameof(np as *mut libc::c_void),
                agnameof(
                    (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(-(1 as libc::c_int as isize))
                    })
                    .node as *mut libc::c_void,
                ),
                hypot(dx, dy),
            );
            ep = agnxtout(g, ep);
        }
        np = agnxtnode(g, np);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdp_tLayout(mut g: *mut graph_t, mut xpms: *mut xparams) {
    let mut i: libc::c_int = 0;
    let mut reset: libc::c_int = 0;
    let mut pp: *mut bport_t =
        (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata)).ports;
    let mut temp: libc::c_double = 0.;
    let mut grid: *mut Grid = 0 as *mut Grid;
    let mut ctr: pointf = pointf { x: 0., y: 0. };
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    reset = init_params(g, xpms);
    temp = parms.T0;
    ctr = initPositions(g, pp);
    if parms.useGrid != 0 {
        grid = mkGrid(agnnodes(g));
        adjustGrid(grid, agnnodes(g));
        i = 0 as libc::c_int;
        while i < parms.loopcnt {
            temp = cool(i);
            gAdjust(g, temp, pp, grid);
            i += 1;
        }
        delGrid(grid);
    } else {
        i = 0 as libc::c_int;
        while i < parms.loopcnt {
            temp = cool(i);
            adjust(g, temp, pp);
            i += 1;
        }
    }
    if ctr.x != 0.0f64 || ctr.y != 0.0f64 {
        n = agfstnode(g);
        while !n.is_null() {
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) += ctr.x;
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) += ctr.y;
            n = agnxtnode(g, n);
        }
    }
    if reset != 0 {
        reset_params();
    }
}
