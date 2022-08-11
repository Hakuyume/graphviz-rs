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
    pub type htmllabel_t;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agcontains(_: *mut Agraph_t, _: *mut libc::c_void) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Concentrate: libc::c_uchar;
    static mut G_margin: *mut Agsym_t;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn late_int(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn UF_find(_: *mut Agnode_t) -> *mut Agnode_t;
    fn gv_nodesize(n: *mut Agnode_t, flip: bool);
    fn rank(g: *mut graph_t, balance: libc::c_int, maxiter: libc::c_int) -> libc::c_int;
    fn selfRightSpace(e: *mut edge_t) -> libc::c_double;
    fn countDummyNodes(g: *mut graph_t) -> libc::c_int;
    fn fast_edge(_: *mut Agedge_t) -> *mut Agedge_t;
    fn fast_nodeapp(_: *mut Agnode_t, _: *mut Agnode_t);
    fn find_fast_edge(_: *mut Agnode_t, _: *mut Agnode_t) -> *mut Agedge_t;
    fn flat_edges(_: *mut Agraph_t) -> libc::c_int;
    fn mark_lowclusters(_: *mut Agraph_t);
    fn unmerge_oneway(_: *mut Agedge_t);
    fn virtual_node(_: *mut Agraph_t) -> *mut Agnode_t;
    fn zapinlist(_: *mut elist, _: *mut Agedge_t);
    fn dot_root(_: *mut libc::c_void) -> *mut Agraph_t;
    fn dot_concentrate(_: *mut Agraph_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agedgepair_s {
    pub out: Agedge_t,
    pub in_0: Agedge_t,
}
pub type Agedgepair_t = Agedgepair_s;
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
pub struct aspect_t {
    pub targetAR: libc::c_double,
    pub combiAR: libc::c_double,
    pub prevIterations: libc::c_int,
    pub curIterations: libc::c_int,
    pub nextIter: libc::c_int,
    pub nPasses: libc::c_int,
    pub badGraph: libc::c_int,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn sub_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x - q.x;
    r.y = p.y - q.y;
    return r;
}
unsafe extern "C" fn largeMinlen(mut l: libc::c_double) -> libc::c_double {
    agerr(
        AGERR,
        b"Edge length %f larger than maximum %u allowed.\nCheck for overwide node(s).\n\0"
            as *const u8 as *const libc::c_char,
        l,
        32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
    );
    return (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
        as libc::c_double;
}
unsafe extern "C" fn connectGraph(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut tp: *mut node_t = 0 as *mut node_t;
    let mut hp: *mut node_t = 0 as *mut node_t;
    let mut sn: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut rp: *mut rank_t = 0 as *mut rank_t;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        rp = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize);
        found = 0 as libc::c_int;
        tp = 0 as *mut node_t;
        i = 0 as libc::c_int;
        while i < (*rp).n {
            tp = *((*rp).v).offset(i as isize);
            if !((*((*(tp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).save_out.list)
                .is_null()
            {
                j = 0 as libc::c_int;
                loop {
                    e = *((*((*(tp as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .save_out
                        .list)
                        .offset(j as isize);
                    if e.is_null() {
                        break;
                    }
                    if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank > r
                        || (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .rank > r
                    {
                        found = (0 as libc::c_int == 0) as libc::c_int;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if found != 0 {
                    break;
                }
            }
            if !((*((*(tp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).save_in.list)
                .is_null()
            {
                j = 0 as libc::c_int;
                loop {
                    e = *((*((*(tp as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .save_in
                        .list)
                        .offset(j as isize);
                    if e.is_null() {
                        break;
                    }
                    if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank > r
                        || (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .rank > r
                    {
                        found = (0 as libc::c_int == 0) as libc::c_int;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if found != 0 {
                    break;
                }
            }
            i += 1;
        }
        if !(found != 0 || tp.is_null()) {
            tp = *((*rp).v).offset(0 as libc::c_int as isize);
            if r < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
                hp = *((*rp.offset(1 as libc::c_int as isize)).v)
                    .offset(0 as libc::c_int as isize);
            } else {
                hp = *((*rp.offset(-(1 as libc::c_int as isize))).v)
                    .offset(0 as libc::c_int as isize);
            }
            if !hp.is_null() {} else {
                __assert_fail(
                    b"hp\0" as *const u8 as *const libc::c_char,
                    b"position.c\0" as *const u8 as *const libc::c_char,
                    109 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void connectGraph(graph_t *)\0"))
                        .as_ptr(),
                );
            }
            sn = virtual_node(g);
            (*((*(sn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .node_type = 2 as libc::c_int as libc::c_char;
            make_aux_edge(sn, tp, 0 as libc::c_int as libc::c_double, 0 as libc::c_int);
            make_aux_edge(sn, hp, 0 as libc::c_int as libc::c_double, 0 as libc::c_int);
            (*((*(sn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .rank = if (*((*(tp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                < (*((*(hp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            {
                (*((*(tp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            } else {
                (*((*(hp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            };
        }
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dot_position(mut g: *mut graph_t, mut asp: *mut aspect_t) {
    if ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist).is_null() {
        return;
    }
    mark_lowclusters(g);
    set_ycoords(g);
    if Concentrate != 0 {
        dot_concentrate(g);
    }
    expand_leaves(g);
    if flat_edges(g) != 0 {
        set_ycoords(g);
    }
    create_aux_edges(g);
    if rank(g, 2 as libc::c_int, nsiter2(g)) != 0 {
        connectGraph(g);
        let rank_result: libc::c_int = rank(g, 2 as libc::c_int, nsiter2(g));
        if rank_result == 0 as libc::c_int {} else {
            __assert_fail(
                b"rank_result == 0\0" as *const u8 as *const libc::c_char,
                b"position.c\0" as *const u8 as *const libc::c_char,
                133 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void dot_position(graph_t *, aspect_t *)\0"))
                    .as_ptr(),
            );
        }
    }
    set_xcoords(g);
    set_aspect(g, asp);
    remove_aux_edges(g);
}
unsafe extern "C" fn nsiter2(mut g: *mut graph_t) -> libc::c_int {
    let mut maxiter: libc::c_int = 2147483647 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = agget(
        g as *mut libc::c_void,
        b"nslimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        maxiter = (atof(s) * agnnodes(g) as libc::c_double) as libc::c_int;
    }
    return maxiter;
}
unsafe extern "C" fn go(mut u: *mut node_t, mut v: *mut node_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if u == v {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if go(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            v,
        ) != 0
        {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn canreach(mut u: *mut node_t, mut v: *mut node_t) -> libc::c_int {
    return go(u, v);
}
#[no_mangle]
pub unsafe extern "C" fn make_aux_edge(
    mut u: *mut node_t,
    mut v: *mut node_t,
    mut len: libc::c_double,
    mut wt: libc::c_int,
) -> *mut Agedge_t {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut e2: *mut Agedgepair_t = zmalloc(
        ::std::mem::size_of::<Agedgepair_t>() as libc::c_ulong,
    ) as *mut Agedgepair_t;
    let ref mut fresh0 = (*(&mut (*e2).in_0 as *mut Agedge_t as *mut Agobj_t)).tag;
    (*fresh0).set_objtype(3 as libc::c_int as libc::c_uint);
    let ref mut fresh1 = (*(&mut (*e2).out as *mut Agedge_t as *mut Agobj_t)).tag;
    (*fresh1).set_objtype(2 as libc::c_int as libc::c_uint);
    let ref mut fresh2 = (*e2).out.base.data;
    *fresh2 = zmalloc(::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong)
        as *mut Agedgeinfo_t as *mut Agrec_t;
    e = &mut (*e2).out;
    let ref mut fresh3 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    *fresh3 = u;
    let ref mut fresh4 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    *fresh4 = v;
    if len
        > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_double
    {
        len = largeMinlen(len);
    }
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .minlen = (if len >= 0 as libc::c_int as libc::c_double {
        (len + 0.5f64) as libc::c_int
    } else {
        (len - 0.5f64) as libc::c_int
    }) as libc::c_ushort;
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight = wt;
    fast_edge(e);
    return e;
}
unsafe extern "C" fn allocate_aux_edges(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n_in: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .save_in = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .save_out = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out;
        i = 0 as libc::c_int;
        while !(*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(i as isize))
            .is_null()
        {
            i += 1;
        }
        j = 0 as libc::c_int;
        while !(*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
            .offset(j as isize))
            .is_null()
        {
            j += 1;
        }
        n_in = i + j;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .size = 0 as libc::c_int;
        let ref mut fresh5 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list;
        *fresh5 = gcalloc(
            (n_in + 3 as libc::c_int + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .size = 0 as libc::c_int;
        let ref mut fresh6 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list;
        *fresh6 = gcalloc(
            (3 as libc::c_int + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn make_LR_constraints(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m0: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut width: libc::c_double = 0.;
    let mut sep: [libc::c_int; 2] = [0; 2];
    let mut nodesep: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut e1: *mut edge_t = 0 as *mut edge_t;
    let mut ff: *mut edge_t = 0 as *mut edge_t;
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut t0: *mut node_t = 0 as *mut node_t;
    let mut h0: *mut node_t = 0 as *mut node_t;
    let mut rank_0: *mut rank_t = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rank;
    if (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels
        as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        sep[0 as libc::c_int
            as usize] = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep;
        sep[1 as libc::c_int as usize] = 5 as libc::c_int;
    } else {
        sep[0 as libc::c_int
            as usize] = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep;
        sep[1 as libc::c_int as usize] = sep[0 as libc::c_int as usize];
    }
    i = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        let mut last: libc::c_double = 0.;
        let ref mut fresh7 = (*((*(*((*rank_0.offset(i as isize)).v)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank;
        *fresh7 = 0 as libc::c_int;
        last = *fresh7 as libc::c_double;
        nodesep = sep[(i & 1 as libc::c_int) as usize];
        j = 0 as libc::c_int;
        while j < (*rank_0.offset(i as isize)).n {
            u = *((*rank_0.offset(i as isize)).v).offset(j as isize);
            (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .mval = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
            if (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).other.size
                > 0 as libc::c_int
            {
                let mut sw: libc::c_double = 0 as libc::c_int as libc::c_double;
                k = 0 as libc::c_int;
                loop {
                    e = *((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .other
                        .list)
                        .offset(k as isize);
                    if e.is_null() {
                        break;
                    }
                    if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node
                        == (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node
                    {
                        sw += selfRightSpace(e);
                    }
                    k += 1;
                }
                (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw += sw;
            }
            v = *((*rank_0.offset(i as isize)).v)
                .offset((j + 1 as libc::c_int) as isize);
            if !v.is_null() {
                width = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    + (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    + nodesep as libc::c_double;
                e0 = make_aux_edge(u, v, width, 0 as libc::c_int);
                let ref mut fresh8 = (*((*(v as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .rank;
                *fresh8 = (last + width) as libc::c_int;
                last = *fresh8 as libc::c_double;
            }
            e = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut edge_t;
            if !e.is_null() {
                e0 = *((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .save_out
                    .list)
                    .offset(0 as libc::c_int as isize);
                e1 = *((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .save_out
                    .list)
                    .offset(1 as libc::c_int as isize);
                if (*((*((*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e0
                } else {
                    e0.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order
                    > (*((*((*(if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e1
                    } else {
                        e1.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .order
                {
                    ff = e0;
                    e0 = e1;
                    e1 = ff;
                }
                m0 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                    as libc::c_int
                    * (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep
                    / 2 as libc::c_int;
                m1 = (m0 as libc::c_double
                    + (*((*((*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e0
                    } else {
                        e0.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rw
                    + (*((*((*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e0
                    } else {
                        e0.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .lw) as libc::c_int;
                if canreach(
                    (*if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e0
                    } else {
                        e0.offset(1 as libc::c_int as isize)
                    })
                        .node,
                    (*if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e0
                    } else {
                        e0.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                ) == 0
                {
                    make_aux_edge(
                        (*if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e0
                        } else {
                            e0.offset(-(1 as libc::c_int as isize))
                        })
                            .node,
                        (*if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            e0
                        } else {
                            e0.offset(1 as libc::c_int as isize)
                        })
                            .node,
                        m1 as libc::c_double,
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight,
                    );
                }
                m1 = (m0 as libc::c_double
                    + (*((*((*(if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e1
                    } else {
                        e1.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rw
                    + (*((*((*(if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e1
                    } else {
                        e1.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .lw) as libc::c_int;
                if canreach(
                    (*if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e1
                    } else {
                        e1.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    (*if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e1
                    } else {
                        e1.offset(1 as libc::c_int as isize)
                    })
                        .node,
                ) == 0
                {
                    make_aux_edge(
                        (*if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            e1
                        } else {
                            e1.offset(1 as libc::c_int as isize)
                        })
                            .node,
                        (*if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e1
                        } else {
                            e1.offset(-(1 as libc::c_int as isize))
                        })
                            .node,
                        m1 as libc::c_double,
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight,
                    );
                }
            }
            k = 0 as libc::c_int;
            while k < (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).flat_out.size
            {
                e = *((*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .flat_out
                    .list)
                    .offset(k as isize);
                if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order
                    < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .order
                {
                    t0 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node;
                    h0 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node;
                } else {
                    t0 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node;
                    h0 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node;
                }
                width = (*((*(t0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    + (*((*(h0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                m0 = (((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                    as libc::c_int
                    * (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep)
                    as libc::c_double + width) as libc::c_int;
                e0 = find_fast_edge(t0, h0);
                if !e0.is_null() {
                    m0 = (if m0 as libc::c_double
                        > width
                            + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                                .nodesep as libc::c_double
                            + (if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .dist >= 0 as libc::c_int as libc::c_double
                            {
                                ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist
                                    + 0.5f64) as libc::c_int
                            } else {
                                ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist
                                    - 0.5f64) as libc::c_int
                            }) as libc::c_double
                    {
                        m0 as libc::c_double
                    } else {
                        width
                            + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                                .nodesep as libc::c_double
                            + (if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .dist >= 0 as libc::c_int as libc::c_double
                            {
                                ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist
                                    + 0.5f64) as libc::c_int
                            } else {
                                ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist
                                    - 0.5f64) as libc::c_int
                            }) as libc::c_double
                    }) as libc::c_int;
                    if m0 > 32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
                        m0 = largeMinlen(m0 as libc::c_double) as libc::c_int;
                    }
                    (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .minlen = (if (*((*(e0 as *mut Agobj_t)).data
                        as *mut Agedgeinfo_t))
                        .minlen as libc::c_int > m0
                    {
                        (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                            as libc::c_int
                    } else {
                        m0
                    }) as libc::c_ushort;
                    (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .weight = if (*((*(e0 as *mut Agobj_t)).data
                        as *mut Agedgeinfo_t))
                        .weight
                        > (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
                    {
                        (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
                    } else {
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
                    };
                } else if ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .is_null()
                    {
                    make_aux_edge(
                        t0,
                        h0,
                        m0 as libc::c_double,
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight,
                    );
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn make_edge_pairs(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut m0: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut sn: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).save_out.list)
            .is_null()
        {
            i = 0 as libc::c_int;
            loop {
                e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .save_out
                    .list)
                    .offset(i as isize);
                if e.is_null() {
                    break;
                }
                sn = virtual_node(g);
                (*((*(sn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .node_type = 2 as libc::c_int as libc::c_char;
                m0 = ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p.x
                    - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .tail_port
                        .p
                        .x) as libc::c_int;
                if m0 > 0 as libc::c_int {
                    m1 = 0 as libc::c_int;
                } else {
                    m1 = -m0;
                    m0 = 0 as libc::c_int;
                }
                make_aux_edge(
                    sn,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node,
                    (m0 + 1 as libc::c_int) as libc::c_double,
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight,
                );
                make_aux_edge(
                    sn,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    (m1 + 1 as libc::c_int) as libc::c_double,
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight,
                );
                (*((*(sn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .rank = if ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rank - m0 - 1 as libc::c_int)
                    < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank - m1 - 1 as libc::c_int
                {
                    (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank - m0 - 1 as libc::c_int
                } else {
                    (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank - m1 - 1 as libc::c_int
                };
                i += 1;
            }
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn contain_clustnodes(mut g: *mut graph_t) {
    let mut c: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if g != dot_root(g as *mut libc::c_void) {
        contain_nodes(g);
        e = find_fast_edge(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn,
        );
        if !e.is_null() {
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
                += 128 as libc::c_int;
        } else {
            make_aux_edge(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln,
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn,
                1 as libc::c_int as libc::c_double,
                128 as libc::c_int,
            );
        }
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        contain_clustnodes(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
        );
        c += 1;
    }
}
unsafe extern "C" fn vnode_not_related_to(
    mut g: *mut graph_t,
    mut v: *mut node_t,
) -> libc::c_int {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).save_out.list)
        .offset(0 as libc::c_int as isize);
    while !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig).is_null() {
        e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    if agcontains(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut libc::c_void,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    if agcontains(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut libc::c_void,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn keepout_othernodes(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut margin: libc::c_int = 0;
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    margin = late_int(
        g as *mut libc::c_void,
        G_margin,
        8 as libc::c_int,
        0 as libc::c_int,
    );
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        if !((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n == 0 as libc::c_int)
        {
            v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(0 as libc::c_int as isize);
            if !v.is_null() {
                i = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                    - 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    u = *((*((*((*(dot_root(g as *mut libc::c_void) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .rank)
                        .offset(r as isize))
                        .v)
                        .offset(i as isize);
                    if (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                        as libc::c_int == 0 as libc::c_int
                        || vnode_not_related_to(g, u) != 0
                    {
                        make_aux_edge(
                            u,
                            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln,
                            margin as libc::c_double
                                + (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw,
                            0 as libc::c_int,
                        );
                        break;
                    } else {
                        i -= 1;
                    }
                }
                i = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                    + (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .n;
                while i
                    < (*((*((*(dot_root(g as *mut libc::c_void) as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .rank)
                        .offset(r as isize))
                        .n
                {
                    u = *((*((*((*(dot_root(g as *mut libc::c_void) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .rank)
                        .offset(r as isize))
                        .v)
                        .offset(i as isize);
                    if (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                        as libc::c_int == 0 as libc::c_int
                        || vnode_not_related_to(g, u) != 0
                    {
                        make_aux_edge(
                            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn,
                            u,
                            margin as libc::c_double
                                + (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw,
                            0 as libc::c_int,
                        );
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        r += 1;
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        keepout_othernodes(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
        );
        c += 1;
    }
}
unsafe extern "C" fn contain_subclust(mut g: *mut graph_t) {
    let mut margin: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    margin = late_int(
        g as *mut libc::c_void,
        G_margin,
        8 as libc::c_int,
        0 as libc::c_int,
    );
    make_lrvn(g);
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        subg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(c as isize);
        make_lrvn(subg);
        make_aux_edge(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln,
            (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln,
            margin as libc::c_double
                + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .border[3 as libc::c_int as usize]
                    .x,
            0 as libc::c_int,
        );
        make_aux_edge(
            (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn,
            margin as libc::c_double
                + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .border[1 as libc::c_int as usize]
                    .x,
            0 as libc::c_int,
        );
        contain_subclust(subg);
        c += 1;
    }
}
unsafe extern "C" fn separate_subclust(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut margin: libc::c_int = 0;
    let mut low: *mut graph_t = 0 as *mut graph_t;
    let mut high: *mut graph_t = 0 as *mut graph_t;
    let mut left: *mut graph_t = 0 as *mut graph_t;
    let mut right: *mut graph_t = 0 as *mut graph_t;
    margin = late_int(
        g as *mut libc::c_void,
        G_margin,
        8 as libc::c_int,
        0 as libc::c_int,
    );
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        make_lrvn(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(i as isize),
        );
        i += 1;
    }
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        j = i + 1 as libc::c_int;
        while j <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
            low = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(i as isize);
            high = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(j as isize);
            if (*((*(low as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
                > (*((*(high as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            {
                let mut temp: *mut graph_t = low;
                low = high;
                high = temp;
            }
            if !((*((*(low as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
                < (*((*(high as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank)
            {
                if (*((*(*((*((*((*(low as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .rank)
                    .offset(
                        (*((*(high as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
                            as isize,
                    ))
                    .v)
                    .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order
                    < (*((*(*((*((*((*(high as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .rank)
                        .offset(
                            (*((*(high as *mut Agobj_t)).data as *mut Agraphinfo_t))
                                .minrank as isize,
                        ))
                        .v)
                        .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .order
                {
                    left = low;
                    right = high;
                } else {
                    left = high;
                    right = low;
                }
                make_aux_edge(
                    (*((*(left as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn,
                    (*((*(right as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln,
                    margin as libc::c_double,
                    0 as libc::c_int,
                );
            }
            j += 1;
        }
        separate_subclust(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(i as isize),
        );
        i += 1;
    }
}
unsafe extern "C" fn pos_clusters(mut g: *mut graph_t) {
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster > 0 as libc::c_int
    {
        contain_clustnodes(g);
        keepout_othernodes(g);
        contain_subclust(g);
        separate_subclust(g);
    }
}
unsafe extern "C" fn compress_graph(mut g: *mut graph_t) {
    let mut x: libc::c_double = 0.;
    let mut p: pointf = pointf { x: 0., y: 0. };
    if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio_kind
        as libc::c_uint != R_COMPRESS as libc::c_int as libc::c_uint
    {
        return;
    }
    p = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).size;
    if p.x * p.y <= 1 as libc::c_int as libc::c_double {
        return;
    }
    contain_nodes(g);
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir & 0x3 as libc::c_int
        & 1 as libc::c_int == 0
    {
        x = p.x;
    } else {
        x = p.y;
    }
    x = if x
        < (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_double
    {
        x
    } else {
        (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_double
    };
    make_aux_edge(
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn,
        x,
        1000 as libc::c_int,
    );
}
unsafe extern "C" fn create_aux_edges(mut g: *mut graph_t) {
    allocate_aux_edges(g);
    make_LR_constraints(g);
    make_edge_pairs(g);
    pos_clusters(g);
    compress_graph(g);
}
unsafe extern "C" fn remove_aux_edges(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut nnext: *mut node_t = 0 as *mut node_t;
    let mut nprev: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            free((*e).base.data as *mut libc::c_void);
            free(e as *mut libc::c_void);
            i += 1;
        }
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list
                as *mut libc::c_void,
        );
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list
                as *mut libc::c_void,
        );
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).save_out;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).save_in;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    nprev = 0 as *mut node_t;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        nnext = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 2 as libc::c_int
        {
            if !nprev.is_null() {
                let ref mut fresh9 = (*((*(nprev as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .next;
                *fresh9 = nnext;
            } else {
                let ref mut fresh10 = (*((*(g as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .nlist;
                *fresh10 = nnext;
            }
            free((*n).base.data as *mut libc::c_void);
            free(n as *mut libc::c_void);
        } else {
            nprev = n;
        }
        n = nnext;
    }
    let ref mut fresh11 = (*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .nlist as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .prev;
    *fresh11 = 0 as *mut node_t;
}
unsafe extern "C" fn set_xcoords(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut rank_0: *mut rank_t = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rank;
    i = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        j = 0 as libc::c_int;
        while j < (*rank_0.offset(i as isize)).n {
            v = *((*rank_0.offset(i as isize)).v).offset(j as isize);
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .x = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                as libc::c_double;
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank = i;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn adjustSimple(
    mut g: *mut graph_t,
    mut delta: libc::c_int,
    mut margin_total: libc::c_int,
) {
    let mut r: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut deltop: libc::c_int = 0;
    let mut delbottom: libc::c_int = 0;
    let mut root: *mut graph_t = dot_root(g as *mut libc::c_void);
    let mut rank_0: *mut rank_t = (*((*(root as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .rank;
    let mut maxr: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .maxrank;
    let mut minr: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .minrank;
    bottom = (delta + 1 as libc::c_int) / 2 as libc::c_int;
    delbottom = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
        + bottom as libc::c_double
        - ((*rank_0.offset(maxr as isize)).ht1 - margin_total as libc::c_double))
        as libc::c_int;
    if delbottom > 0 as libc::c_int {
        r = maxr;
        while r >= minr {
            if (*rank_0.offset(r as isize)).n > 0 as libc::c_int {
                (*((*(*((*rank_0.offset(r as isize)).v).offset(0 as libc::c_int as isize)
                    as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .coord
                    .y += delbottom as libc::c_double;
            }
            r -= 1;
        }
        deltop = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
            + (delta - bottom) as libc::c_double + delbottom as libc::c_double
            - ((*rank_0.offset(minr as isize)).ht2 - margin_total as libc::c_double))
            as libc::c_int;
    } else {
        deltop = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
            + (delta - bottom) as libc::c_double
            - ((*rank_0.offset(minr as isize)).ht2 - margin_total as libc::c_double))
            as libc::c_int;
    }
    if deltop > 0 as libc::c_int {
        r = minr - 1 as libc::c_int;
        while r >= (*((*(root as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank {
            if (*rank_0.offset(r as isize)).n > 0 as libc::c_int {
                (*((*(*((*rank_0.offset(r as isize)).v).offset(0 as libc::c_int as isize)
                    as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .coord
                    .y += deltop as libc::c_double;
            }
            r -= 1;
        }
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
        += (delta - bottom) as libc::c_double;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
        += bottom as libc::c_double;
}
unsafe extern "C" fn adjustRanks(mut g: *mut graph_t, mut margin_total: libc::c_int) {
    let mut lht: libc::c_double = 0.;
    let mut rht: libc::c_double = 0.;
    let mut maxr: libc::c_int = 0;
    let mut minr: libc::c_int = 0;
    let mut margin: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut delta: libc::c_double = 0.;
    let mut ht1: libc::c_double = 0.;
    let mut ht2: libc::c_double = 0.;
    let mut rank_0: *mut rank_t = (*((*(dot_root(g as *mut libc::c_void)
        as *mut Agobj_t))
        .data as *mut Agraphinfo_t))
        .rank;
    if g == dot_root(g as *mut libc::c_void) {
        margin = 0 as libc::c_int;
    } else {
        margin = late_int(
            g as *mut libc::c_void,
            G_margin,
            8 as libc::c_int,
            0 as libc::c_int,
        );
    }
    ht1 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1;
    ht2 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        let mut subg: *mut graph_t = *((*((*(g as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .clust)
            .offset(c as isize);
        adjustRanks(subg, margin + margin_total);
        if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            == (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
        {
            ht1 = if ht1
                > (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
                    + margin as libc::c_double
            {
                ht1
            } else {
                (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
                    + margin as libc::c_double
            };
        }
        if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            == (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
        {
            ht2 = if ht2
                > (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
                    + margin as libc::c_double
            {
                ht2
            } else {
                (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
                    + margin as libc::c_double
            };
        }
        c += 1;
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1 = ht1;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2 = ht2;
    if g != dot_root(g as *mut libc::c_void)
        && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
    {
        lht = if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .border[3 as libc::c_int as usize]
            .y
            > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[1 as libc::c_int as usize]
                .y
        {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[3 as libc::c_int as usize]
                .y
        } else {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[1 as libc::c_int as usize]
                .y
        };
        maxr = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank;
        minr = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        rht = (*((*(*((*rank_0.offset(minr as isize)).v)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord
            .y
            - (*((*(*((*rank_0.offset(maxr as isize)).v)
                .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .coord
                .y;
        delta = lht - (rht + ht1 + ht2);
        if delta > 0 as libc::c_int as libc::c_double {
            adjustSimple(g, delta as libc::c_int, margin_total);
        }
    }
    if g != dot_root(g as *mut libc::c_void) {
        (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank as isize,
            ))
            .ht2 = if (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank as isize,
            ))
            .ht2 > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
        {
            (*rank_0
                .offset(
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
                        as isize,
                ))
                .ht2
        } else {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
        };
        (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank as isize,
            ))
            .ht1 = if (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank as isize,
            ))
            .ht1 > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
        {
            (*rank_0
                .offset(
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
                        as isize,
                ))
                .ht1
        } else {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
        };
    }
}
unsafe extern "C" fn clust_ht(mut g: *mut Agraph_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut ht1: libc::c_double = 0.;
    let mut ht2: libc::c_double = 0.;
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut rank_0: *mut rank_t = (*((*(dot_root(g as *mut libc::c_void)
        as *mut Agobj_t))
        .data as *mut Agraphinfo_t))
        .rank;
    let mut margin: libc::c_int = 0;
    let mut haveClustLabel: libc::c_int = 0 as libc::c_int;
    if g == dot_root(g as *mut libc::c_void) {
        margin = 8 as libc::c_int;
    } else {
        margin = late_int(
            g as *mut libc::c_void,
            G_margin,
            8 as libc::c_int,
            0 as libc::c_int,
        );
    }
    ht1 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1;
    ht2 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        subg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(c as isize);
        haveClustLabel |= clust_ht(subg);
        if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            == (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
        {
            ht1 = if ht1
                > (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
                    + margin as libc::c_double
            {
                ht1
            } else {
                (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
                    + margin as libc::c_double
            };
        }
        if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            == (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
        {
            ht2 = if ht2
                > (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
                    + margin as libc::c_double
            {
                ht2
            } else {
                (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
                    + margin as libc::c_double
            };
        }
        c += 1;
    }
    if g != dot_root(g as *mut libc::c_void)
        && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
    {
        haveClustLabel = 1 as libc::c_int;
        if (*((*(agroot(g as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int == 0
        {
            ht1
                += (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .border[0 as libc::c_int as usize]
                    .y;
            ht2
                += (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .border[2 as libc::c_int as usize]
                    .y;
        }
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1 = ht1;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2 = ht2;
    if g != dot_root(g as *mut libc::c_void) {
        (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank as isize,
            ))
            .ht2 = if (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank as isize,
            ))
            .ht2 > ht2
        {
            (*rank_0
                .offset(
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
                        as isize,
                ))
                .ht2
        } else {
            ht2
        };
        (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank as isize,
            ))
            .ht1 = if (*rank_0
            .offset(
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank as isize,
            ))
            .ht1 > ht1
        {
            (*rank_0
                .offset(
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
                        as isize,
                ))
                .ht1
        } else {
            ht1
        };
    }
    return haveClustLabel;
}
unsafe extern "C" fn set_ycoords(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ht2: libc::c_double = 0.;
    let mut maxht: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut d0: libc::c_double = 0.;
    let mut d1: libc::c_double = 0.;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut rank_0: *mut rank_t = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rank;
    let mut clust: *mut graph_t = 0 as *mut graph_t;
    let mut lbl: libc::c_int = 0;
    maxht = 0 as libc::c_int as libc::c_double;
    ht2 = maxht;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        i = 0 as libc::c_int;
        while i < (*rank_0.offset(r as isize)).n {
            n = *((*rank_0.offset(r as isize)).v).offset(i as isize);
            ht2 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                / 2 as libc::c_int as libc::c_double;
            if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).other.list)
                .is_null()
            {
                j = 0 as libc::c_int;
                loop {
                    e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .other
                        .list)
                        .offset(j as isize);
                    if e.is_null() {
                        break;
                    }
                    if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node
                        == (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node
                    {
                        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                            .is_null()
                        {
                            ht2 = if ht2
                                > (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                    .label)
                                    .dimen
                                    .y / 2 as libc::c_int as libc::c_double
                            {
                                ht2
                            } else {
                                (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                    .label)
                                    .dimen
                                    .y / 2 as libc::c_int as libc::c_double
                            };
                        }
                    }
                    j += 1;
                }
            }
            if (*rank_0.offset(r as isize)).pht2 < ht2 {
                let ref mut fresh12 = (*rank_0.offset(r as isize)).ht2;
                *fresh12 = ht2;
                (*rank_0.offset(r as isize)).pht2 = *fresh12;
            }
            if (*rank_0.offset(r as isize)).pht1 < ht2 {
                let ref mut fresh13 = (*rank_0.offset(r as isize)).ht1;
                *fresh13 = ht2;
                (*rank_0.offset(r as isize)).pht1 = *fresh13;
            }
            clust = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust;
            if !clust.is_null() {
                let mut yoff: libc::c_int = if clust == g {
                    0 as libc::c_int
                } else {
                    late_int(
                        clust as *mut libc::c_void,
                        G_margin,
                        8 as libc::c_int,
                        0 as libc::c_int,
                    )
                };
                if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                    == (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
                {
                    (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .ht2 = if (*((*(clust as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ht2 > ht2 + yoff as libc::c_double
                    {
                        (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2
                    } else {
                        ht2 + yoff as libc::c_double
                    };
                }
                if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                    == (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
                {
                    (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .ht1 = if (*((*(clust as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ht1 > ht2 + yoff as libc::c_double
                    {
                        (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1
                    } else {
                        ht2 + yoff as libc::c_double
                    };
                }
            }
            i += 1;
        }
        r += 1;
    }
    lbl = clust_ht(g);
    maxht = 0 as libc::c_int as libc::c_double;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank;
    (*((*(*((*rank_0.offset(r as isize)).v).offset(0 as libc::c_int as isize)
        as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord
        .y = (*rank_0.offset(r as isize)).ht1;
    loop {
        r -= 1;
        if !(r >= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank) {
            break;
        }
        d0 = (*rank_0.offset((r + 1 as libc::c_int) as isize)).pht2
            + (*rank_0.offset(r as isize)).pht1
            + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ranksep
                as libc::c_double;
        d1 = (*rank_0.offset((r + 1 as libc::c_int) as isize)).ht2
            + (*rank_0.offset(r as isize)).ht1 + 8 as libc::c_int as libc::c_double;
        delta = if d0 > d1 { d0 } else { d1 };
        if (*rank_0.offset(r as isize)).n > 0 as libc::c_int {
            (*((*(*((*rank_0.offset(r as isize)).v).offset(0 as libc::c_int as isize)
                as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .coord
                .y = (*((*(*((*rank_0.offset((r + 1 as libc::c_int) as isize)).v)
                .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .coord
                .y + delta;
        }
        maxht = if maxht > delta { maxht } else { delta };
    }
    if lbl != 0
        && (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int & 1 as libc::c_int != 0
    {
        adjustRanks(g, 0 as libc::c_int);
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).exact_ranksep {
            maxht = 0 as libc::c_int as libc::c_double;
            r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank;
            d0 = (*((*(*((*rank_0.offset(r as isize)).v)
                .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .coord
                .y;
            loop {
                r -= 1;
                if !(r >= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank)
                {
                    break;
                }
                d1 = (*((*(*((*rank_0.offset(r as isize)).v)
                    .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .coord
                    .y;
                delta = d1 - d0;
                maxht = if maxht > delta { maxht } else { delta };
                d0 = d1;
            }
        }
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).exact_ranksep {
        r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            - 1 as libc::c_int;
        while r >= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank {
            if (*rank_0.offset(r as isize)).n > 0 as libc::c_int {
                (*((*(*((*rank_0.offset(r as isize)).v).offset(0 as libc::c_int as isize)
                    as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .coord
                    .y = (*((*(*((*rank_0.offset((r + 1 as libc::c_int) as isize)).v)
                    .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .coord
                    .y + maxht;
            }
            r -= 1;
        }
    }
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .coord
            .y = (*((*(*((*rank_0
            .offset((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize))
            .v)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord
            .y;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn dot_compute_bb(mut g: *mut graph_t, mut root: *mut graph_t) {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    let mut offset: libc::c_double = 0.;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut LL: pointf = pointf { x: 0., y: 0. };
    let mut UR: pointf = pointf { x: 0., y: 0. };
    if g == dot_root(g as *mut libc::c_void) {
        LL.x = 2147483647 as libc::c_int as libc::c_double;
        UR.x = -(2147483647 as libc::c_int) as libc::c_double;
        r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
            let mut rnkn: libc::c_int = (*((*((*(g as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .n;
            if !(rnkn == 0 as libc::c_int) {
                v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .v)
                    .offset(0 as libc::c_int as isize);
                if !v.is_null() {
                    c = 1 as libc::c_int;
                    while (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                        as libc::c_int != 0 as libc::c_int && c < rnkn
                    {
                        v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                            .rank)
                            .offset(r as isize))
                            .v)
                            .offset(c as isize);
                        c += 1;
                    }
                    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                        as libc::c_int == 0 as libc::c_int
                    {
                        x = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                            - (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                        LL.x = if LL.x < x { LL.x } else { x };
                        v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                            .rank)
                            .offset(r as isize))
                            .v)
                            .offset((rnkn - 1 as libc::c_int) as isize);
                        c = rnkn - 2 as libc::c_int;
                        while (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .node_type as libc::c_int != 0 as libc::c_int
                        {
                            v = *((*((*((*(g as *mut Agobj_t)).data
                                as *mut Agraphinfo_t))
                                .rank)
                                .offset(r as isize))
                                .v)
                                .offset(c as isize);
                            c -= 1;
                        }
                        x = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                            + (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
                        UR.x = if UR.x > x { UR.x } else { x };
                    }
                }
            }
            r += 1;
        }
        offset = 8 as libc::c_int as libc::c_double;
        c = 1 as libc::c_int;
        while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
            x = (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize) as *mut Agobj_t))
                .data as *mut Agraphinfo_t))
                .bb
                .LL
                .x - offset;
            LL.x = if LL.x < x { LL.x } else { x };
            x = (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize) as *mut Agobj_t))
                .data as *mut Agraphinfo_t))
                .bb
                .UR
                .x + offset;
            UR.x = if UR.x > x { UR.x } else { x };
            c += 1;
        }
    } else {
        LL
            .x = (*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln
            as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank as libc::c_double;
        UR
            .x = (*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn
            as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank as libc::c_double;
    }
    LL
        .y = (*((*(*((*((*((*(root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank as isize))
        .v)
        .offset(0 as libc::c_int as isize) as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord
        .y - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht1;
    UR
        .y = (*((*(*((*((*((*(root as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
        .offset((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank as isize))
        .v)
        .offset(0 as libc::c_int as isize) as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord
        .y + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ht2;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL = LL;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR = UR;
}
unsafe extern "C" fn rec_bb(mut g: *mut graph_t, mut root: *mut graph_t) {
    let mut c: libc::c_int = 0;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        rec_bb(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
            root,
        );
        c += 1;
    }
    dot_compute_bb(g, root);
}
unsafe extern "C" fn scale_bb(
    mut g: *mut graph_t,
    mut root: *mut graph_t,
    mut xf: libc::c_double,
    mut yf: libc::c_double,
) {
    let mut c: libc::c_int = 0;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        scale_bb(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
            root,
            xf,
            yf,
        );
        c += 1;
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x *= xf;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y *= yf;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x *= xf;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y *= yf;
}
unsafe extern "C" fn adjustAspectRatio(mut g: *mut graph_t, mut asp: *mut aspect_t) {
    let mut AR: libc::c_double = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .x - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x)
        / ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
            - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"AR=%0.4lf\t Area= %0.4lf\t\0" as *const u8 as *const libc::c_char,
            AR,
            ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x)
                * ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
                    - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y)
                / 10000.0f64,
        );
        fprintf(
            stderr,
            b"Dummy=%d\n\0" as *const u8 as *const libc::c_char,
            countDummyNodes(g),
        );
    }
    if AR > 1.1f64 * (*asp).targetAR {
        (*asp)
            .nextIter = ((*asp).targetAR
            * ((*asp).curIterations - (*asp).prevIterations) as libc::c_double / AR)
            as libc::c_int;
    } else if AR <= 0.8f64 * (*asp).targetAR {
        (*asp).nextIter = -(1 as libc::c_int);
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Going to apply another expansion.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        (*asp).nextIter = 0 as libc::c_int;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"next#iter=%d\n\0" as *const u8 as *const libc::c_char,
            (*asp).nextIter,
        );
    }
}
unsafe extern "C" fn set_aspect(mut g: *mut graph_t, mut asp: *mut aspect_t) {
    let mut xf: libc::c_double = 0.0f64;
    let mut yf: libc::c_double = 0.0f64;
    let mut actual: libc::c_double = 0.;
    let mut desired: libc::c_double = 0.;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut scale_it: bool = false;
    let mut filled: bool = false;
    let mut sz: point = point { x: 0, y: 0 };
    rec_bb(g, g);
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank > 0 as libc::c_int
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio_kind
            as libc::c_uint != 0
    {
        sz
            .x = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
            - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x)
            as libc::c_int;
        sz
            .y = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
            - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y)
            as libc::c_int;
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int & 1 as libc::c_int != 0
        {
            let mut t: libc::c_int = sz.x;
            sz.x = sz.y;
            sz.y = t;
        }
        scale_it = 0 as libc::c_int == 0;
        if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio_kind
            as libc::c_uint == R_AUTO as libc::c_int as libc::c_uint
        {
            filled = idealsize(g, 0.5f64);
        } else {
            filled = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .ratio_kind as libc::c_uint == R_FILL as libc::c_int as libc::c_uint;
        }
        if filled {
            if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).size.x
                <= 0 as libc::c_int as libc::c_double
            {
                scale_it = 0 as libc::c_int != 0;
            } else {
                xf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                    .size
                    .x / sz.x as libc::c_double;
                yf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                    .size
                    .y / sz.y as libc::c_double;
                if xf < 1.0f64 || yf < 1.0f64 {
                    if xf < yf {
                        yf = yf / xf;
                        xf = 1.0f64;
                    } else {
                        xf = xf / yf;
                        yf = 1.0f64;
                    }
                }
            }
        } else if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .ratio_kind as libc::c_uint == R_EXPAND as libc::c_int as libc::c_uint
            {
            if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).size.x
                <= 0 as libc::c_int as libc::c_double
            {
                scale_it = 0 as libc::c_int != 0;
            } else {
                xf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                    .size
                    .x / (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x;
                yf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                    .size
                    .y / (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y;
                if xf > 1.0f64 && yf > 1.0f64 {
                    let mut scale: libc::c_double = if xf < yf { xf } else { yf };
                    yf = scale;
                    xf = yf;
                } else {
                    scale_it = 0 as libc::c_int != 0;
                }
            }
        } else if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .ratio_kind as libc::c_uint == R_VALUE as libc::c_int as libc::c_uint
            {
            desired = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .ratio;
            actual = sz.y as libc::c_double / sz.x as libc::c_double;
            if actual < desired {
                yf = desired / actual;
                xf = 1.0f64;
            } else {
                xf = actual / desired;
                yf = 1.0f64;
            }
        } else {
            scale_it = 0 as libc::c_int != 0;
        }
        if scale_it {
            if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                let mut t_0: libc::c_double = xf;
                xf = yf;
                yf = t_0;
            }
            n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
            while !n.is_null() {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .x = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .x * xf >= 0 as libc::c_int as libc::c_double
                {
                    ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x * xf
                        + 0.5f64) as libc::c_int
                } else {
                    ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x * xf
                        - 0.5f64) as libc::c_int
                }) as libc::c_double;
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .y = (if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .y * yf >= 0 as libc::c_int as libc::c_double
                {
                    ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y * yf
                        + 0.5f64) as libc::c_int
                } else {
                    ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y * yf
                        - 0.5f64) as libc::c_int
                }) as libc::c_double;
                n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
            }
            scale_bb(g, g, xf, yf);
        }
    }
    if !asp.is_null() {
        adjustAspectRatio(g, asp);
    }
}
unsafe extern "C" fn resize_leaf(mut leaf: *mut node_t, mut lbound: point) -> point {
    gv_nodesize(
        leaf,
        (*((*(agraphof(leaf as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0,
    );
    (*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .coord
        .y = lbound.y as libc::c_double;
    (*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .coord
        .x = lbound.x as libc::c_double
        + (*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
    lbound
        .x = (lbound.x as libc::c_double
        + (*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
        + (*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
        + (*((*(agraphof(leaf as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .nodesep as libc::c_double) as libc::c_int;
    return lbound;
}
unsafe extern "C" fn place_leaf(
    mut ing: *mut graph_t,
    mut leaf: *mut node_t,
    mut lbound: point,
    mut order: libc::c_int,
) -> point {
    let mut leader: *mut node_t = 0 as *mut node_t;
    let mut g: *mut graph_t = dot_root(ing as *mut libc::c_void);
    leader = UF_find(leaf);
    if leaf != leader {
        fast_nodeapp(leader, leaf);
    }
    (*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order = order;
    (*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .rank = (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
    let ref mut fresh14 = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .rank)
        .offset((*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize))
        .v)
        .offset((*((*(leaf as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order as isize);
    *fresh14 = leaf;
    return resize_leaf(leaf, lbound);
}
unsafe extern "C" fn make_leafslots(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        j = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i
            < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n
        {
            v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(i as isize);
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order = j;
            if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ranktype
                as libc::c_int == 6 as libc::c_int
            {
                j = j + (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size;
            } else {
                j += 1;
            }
            i += 1;
        }
        if !(j
            <= (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n)
        {
            let ref mut fresh15 = (*((*((*(g as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .v;
            *fresh15 = if !((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .v)
                .is_null()
            {
                grealloc(
                    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(r as isize))
                        .v as *mut libc::c_void,
                    ((j + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut node_t
            } else {
                gmalloc(
                    ((j + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut node_t
            };
            i = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .v)
                    .offset(i as isize);
                let ref mut fresh16 = *((*((*((*(g as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .rank)
                    .offset(r as isize))
                    .v)
                    .offset(
                        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                            as isize,
                    );
                *fresh16 = v;
                i -= 1;
            }
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .n = j;
            let ref mut fresh17 = *((*((*((*(g as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .rank)
                .offset(r as isize))
                .v)
                .offset(j as isize);
            *fresh17 = 0 as *mut node_t;
        }
        r += 1;
    }
}
unsafe extern "C" fn do_leaves(mut g: *mut graph_t, mut leader: *mut node_t) {
    let mut j: libc::c_int = 0;
    let mut lbound: point = point { x: 0, y: 0 };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size
        <= 1 as libc::c_int
    {
        return;
    }
    lbound
        .x = ((*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        - (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw) as libc::c_int;
    lbound
        .y = (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
        as libc::c_int;
    lbound = resize_leaf(leader, lbound);
    if (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
        > 0 as libc::c_int
    {
        n = (*if ((*(*((*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .tag)
            .objtype() as libc::c_int == 2 as libc::c_int
        {
            *((*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize)
        } else {
            (*((*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize))
                .offset(-(1 as libc::c_int as isize))
        })
            .node;
        j = (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            + 1 as libc::c_int;
        e = agfstin(g, n);
        while !e.is_null() {
            let mut e1: *mut edge_t = if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            };
            if (*(if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e1
            } else {
                e1.offset(1 as libc::c_int as isize)
            }))
                .node != leader
                && UF_find(
                    (*(if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e1
                    } else {
                        e1.offset(1 as libc::c_int as isize)
                    }))
                        .node,
                ) == leader
            {
                let fresh18 = j;
                j = j + 1;
                lbound = place_leaf(
                    g,
                    (*if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e1
                    } else {
                        e1.offset(1 as libc::c_int as isize)
                    })
                        .node,
                    lbound,
                    fresh18,
                );
                unmerge_oneway(e1);
                let ref mut fresh19 = (*((*((*if ((*(e1 as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 2 as libc::c_int
                {
                    e1
                } else {
                    e1.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .in_0
                    .list;
                *fresh19 = if !((*((*((*if ((*(e1 as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 2 as libc::c_int
                {
                    e1
                } else {
                    e1.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .in_0
                    .list)
                    .is_null()
                {
                    grealloc(
                        (*((*((*if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e1
                        } else {
                            e1.offset(-(1 as libc::c_int as isize))
                        })
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .in_0
                            .list as *mut libc::c_void,
                        (((*((*((*(if ((*(e1 as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e1
                        } else {
                            e1.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .in_0
                            .size + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                } else {
                    gmalloc(
                        (((*((*((*(if ((*(e1 as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e1
                        } else {
                            e1.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .in_0
                            .size + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                };
                let ref mut fresh20 = (*((*((*if ((*(e1 as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 2 as libc::c_int
                {
                    e1
                } else {
                    e1.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .in_0
                    .size;
                let fresh21 = *fresh20;
                *fresh20 = *fresh20 + 1;
                let ref mut fresh22 = *((*((*((*if ((*(e1 as *mut Agobj_t)).tag)
                    .objtype() as libc::c_int == 2 as libc::c_int
                {
                    e1
                } else {
                    e1.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .in_0
                    .list)
                    .offset(fresh21 as isize);
                *fresh22 = e1;
                let ref mut fresh23 = *((*((*((*if ((*(e1 as *mut Agobj_t)).tag)
                    .objtype() as libc::c_int == 2 as libc::c_int
                {
                    e1
                } else {
                    e1.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .in_0
                    .list)
                    .offset(
                        (*((*((*if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e1
                        } else {
                            e1.offset(-(1 as libc::c_int as isize))
                        })
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .in_0
                            .size as isize,
                    );
                *fresh23 = 0 as *mut edge_t;
            }
            e = agnxtin(g, e);
        }
    } else {
        n = (*if ((*(*((*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .tag)
            .objtype() as libc::c_int == 3 as libc::c_int
        {
            *((*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(0 as libc::c_int as isize)
        } else {
            (*((*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(0 as libc::c_int as isize))
                .offset(1 as libc::c_int as isize)
        })
            .node;
        j = (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            + 1 as libc::c_int;
        e = agfstout(g, n);
        while !e.is_null() {
            if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node != leader
                && UF_find(
                    (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node,
                ) == leader
            {
                let fresh24 = j;
                j = j + 1;
                lbound = place_leaf(
                    g,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    lbound,
                    fresh24,
                );
                unmerge_oneway(e);
                let ref mut fresh25 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .out
                    .list;
                *fresh25 = if !((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .is_null()
                {
                    grealloc(
                        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        })
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .out
                            .list as *mut libc::c_void,
                        (((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .out
                            .size + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                } else {
                    gmalloc(
                        (((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .out
                            .size + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                };
                let ref mut fresh26 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .out
                    .size;
                let fresh27 = *fresh26;
                *fresh26 = *fresh26 + 1;
                let ref mut fresh28 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .offset(fresh27 as isize);
                *fresh28 = e;
                let ref mut fresh29 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .offset(
                        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        })
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .out
                            .size as isize,
                    );
                *fresh29 = 0 as *mut edge_t;
            }
            e = agnxtout(g, e);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ports_eq(
    mut e: *mut edge_t,
    mut f: *mut edge_t,
) -> libc::c_int {
    return ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
        as libc::c_int
        == (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
            as libc::c_int
        && ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p.x
            == (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p.x
            && (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p.y
                == (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p.y
            || !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined)
        && ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p.x
            == (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p.x
            && (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p.y
                == (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p.y
            || !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined))
        as libc::c_int;
}
unsafe extern "C" fn expand_leaves(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    make_leafslots(g);
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).inleaf).is_null() {
            do_leaves(g, (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).inleaf);
        }
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).outleaf).is_null() {
            do_leaves(g, (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).outleaf);
        }
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).other.list).is_null()
        {
            i = 0 as libc::c_int;
            loop {
                e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).other.list)
                    .offset(i as isize);
                if e.is_null() {
                    break;
                }
                d = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rank
                    - (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank;
                if !(d == 0 as libc::c_int) {
                    f = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
                    if ports_eq(e, f) == 0 {
                        zapinlist(
                            &mut (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .other,
                            e,
                        );
                        if d == 1 as libc::c_int {
                            fast_edge(e);
                        }
                        i -= 1;
                    }
                }
                i += 1;
            }
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn make_lrvn(mut g: *mut graph_t) {
    let mut ln: *mut node_t = 0 as *mut node_t;
    let mut rn: *mut node_t = 0 as *mut node_t;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln).is_null() {
        return;
    }
    ln = virtual_node(dot_root(g as *mut libc::c_void));
    (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .node_type = 2 as libc::c_int as libc::c_char;
    rn = virtual_node(dot_root(g as *mut libc::c_void));
    (*((*(rn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .node_type = 2 as libc::c_int as libc::c_char;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && g != dot_root(g as *mut libc::c_void)
        && (*((*(agroot(g as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int == 0
    {
        let mut w: libc::c_int = (if (*((*(g as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .border[0 as libc::c_int as usize]
            .x
            > (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[2 as libc::c_int as usize]
                .x
        {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[0 as libc::c_int as usize]
                .x
        } else {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[2 as libc::c_int as usize]
                .x
        }) as libc::c_int;
        make_aux_edge(ln, rn, w as libc::c_double, 0 as libc::c_int);
    }
    let ref mut fresh30 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln;
    *fresh30 = ln;
    let ref mut fresh31 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn;
    *fresh31 = rn;
}
unsafe extern "C" fn contain_nodes(mut g: *mut graph_t) {
    let mut margin: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ln: *mut node_t = 0 as *mut node_t;
    let mut rn: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    margin = late_int(
        g as *mut libc::c_void,
        G_margin,
        8 as libc::c_int,
        0 as libc::c_int,
    );
    make_lrvn(g);
    ln = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ln;
    rn = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rn;
    r = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
    while r <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        if !((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .n == 0 as libc::c_int)
        {
            v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .v)
                .offset(0 as libc::c_int as isize);
            if v.is_null() {
                agerr(
                    AGERR,
                    b"contain_nodes clust %s rank %d missing node\n\0" as *const u8
                        as *const libc::c_char,
                    agnameof(g as *mut libc::c_void),
                    r,
                );
            } else {
                make_aux_edge(
                    ln,
                    v,
                    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                        + margin as libc::c_double
                        + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                            .border[3 as libc::c_int as usize]
                            .x,
                    0 as libc::c_int,
                );
                v = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(r as isize))
                    .v)
                    .offset(
                        ((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                            .offset(r as isize))
                            .n - 1 as libc::c_int) as isize,
                    );
                make_aux_edge(
                    v,
                    rn,
                    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                        + margin as libc::c_double
                        + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                            .border[1 as libc::c_int as usize]
                            .x,
                    0 as libc::c_int,
                );
            }
        }
        r += 1;
    }
}
unsafe extern "C" fn idealsize(
    mut g: *mut graph_t,
    mut minallowed: libc::c_double,
) -> bool {
    let mut xf: libc::c_double = 0.;
    let mut yf: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut R: libc::c_double = 0.;
    let mut b: pointf = pointf { x: 0., y: 0. };
    let mut relpage: pointf = pointf { x: 0., y: 0. };
    let mut margin: pointf = pointf { x: 0., y: 0. };
    relpage = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).page;
    if relpage.x < 0.001f64 || relpage.y < 0.001f64 {
        return 0 as libc::c_int != 0;
    }
    margin = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).margin;
    relpage = sub_pointf(relpage, margin);
    relpage = sub_pointf(relpage, margin);
    b.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x;
    b.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y;
    xf = relpage.x / b.x;
    yf = relpage.y / b.y;
    if xf >= 1.0f64 && yf >= 1.0f64 {
        return 0 as libc::c_int != 0;
    }
    f = if xf < yf { xf } else { yf };
    yf = if f > minallowed { f } else { minallowed };
    xf = yf;
    R = ceil(xf * b.x / relpage.x);
    xf = R * relpage.x / b.x;
    R = ceil(yf * b.y / relpage.y);
    yf = R * relpage.y / b.y;
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).size.x = b.x * xf;
    (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).size.y = b.y * yf;
    return 1 as libc::c_int != 0;
}
