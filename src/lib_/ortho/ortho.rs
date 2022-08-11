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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn updateWts(g: *mut sgraph, cp: *mut cell, ep: *mut sedge);
    fn freeMaze(_: *mut maze);
    fn mkMaze(_: *mut graph_t) -> *mut maze;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn make_graph(n: libc::c_int) -> *mut rawgraph;
    fn free_graph(_: *mut rawgraph);
    fn insert_edge(_: *mut rawgraph, v1: libc::c_int, v2: libc::c_int);
    fn remove_redge(_: *mut rawgraph, v1: libc::c_int, v2: libc::c_int);
    fn edge_exists(_: *mut rawgraph, v1: libc::c_int, v2: libc::c_int) -> bool;
    fn top_sort(_: *mut rawgraph);
    fn reset(_: *mut sgraph);
    fn shortPath(g: *mut sgraph, from: *mut snode, to: *mut snode) -> libc::c_int;
    fn createSEdge(
        g: *mut sgraph,
        v0: *mut snode,
        v1: *mut snode,
        wt: libc::c_double,
    ) -> *mut sedge;
    fn PQgen(sz: libc::c_int);
    fn PQfree();
    fn clip_and_install(
        fe: *mut edge_t,
        hn: *mut node_t,
        ps: *mut pointf,
        pn: libc::c_int,
        info: *mut splineInfo,
    );
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut Verbose: libc::c_uchar;
    static mut Nop: libc::c_int;
    static mut Concentrate: libc::c_uchar;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn addEdgeLabels(e: *mut edge_t);
    fn newPS() -> *mut PointSet;
    fn freePS(_: *mut PointSet);
    fn addPS(_: *mut PointSet, _: libc::c_int, _: libc::c_int);
    fn isInPS(_: *mut PointSet, _: libc::c_int, _: libc::c_int) -> libc::c_int;
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
pub type qsort_cmpf = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct splineInfo {
    pub swapEnds: Option::<unsafe extern "C" fn(*mut edge_t) -> bool>,
    pub splineMerge: Option::<unsafe extern "C" fn(*mut node_t) -> bool>,
    pub ignoreSwap: bool,
    pub isOrtho: bool,
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
pub struct vertex {
    pub color: libc::c_int,
    pub topsort_order: libc::c_int,
    pub adj_list: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawgraph {
    pub nvs: libc::c_int,
    pub vertices: *mut vertex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paird {
    pub p1: libc::c_double,
    pub p2: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair {
    pub a: libc::c_int,
    pub b: libc::c_int,
}
pub type bend = libc::c_uint;
pub const B_RIGHT: bend = 4;
pub const B_DOWN: bend = 3;
pub const B_LEFT: bend = 2;
pub const B_UP: bend = 1;
pub const B_NODE: bend = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment {
    pub isVert: bool,
    pub comm_coord: libc::c_double,
    pub p: paird,
    pub l1: bend,
    pub l2: bend,
    pub ind_no: libc::c_int,
    pub track_no: libc::c_int,
    pub prev: *mut segment,
    pub next: *mut segment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct route {
    pub n: size_t,
    pub segs: *mut segment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel {
    pub link: Dtlink_t,
    pub p: paird,
    pub cnt: libc::c_int,
    pub seg_list: *mut *mut segment,
    pub G: *mut rawgraph,
    pub cp: *mut cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub flags: libc::c_int,
    pub nedges: libc::c_int,
    pub edges: [*mut sedge; 6],
    pub nsides: libc::c_int,
    pub sides: *mut *mut snode,
    pub bb: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snode {
    pub n_val: libc::c_int,
    pub n_idx: libc::c_int,
    pub n_dad: *mut snode,
    pub n_edge: *mut sedge,
    pub n_adj: libc::c_short,
    pub save_n_adj: libc::c_short,
    pub cells: [*mut cell; 2],
    pub adj_edge_list: *mut libc::c_int,
    pub index: libc::c_int,
    pub isVert: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sedge {
    pub weight: libc::c_double,
    pub cnt: libc::c_int,
    pub v1: libc::c_int,
    pub v2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sgraph {
    pub nnodes: libc::c_int,
    pub nedges: libc::c_int,
    pub save_nnodes: libc::c_int,
    pub save_nedges: libc::c_int,
    pub nodes: *mut snode,
    pub edges: *mut sedge,
}
pub type C2RustUnnamed_8 = libc::c_uint;
pub const M_BOTTOM: C2RustUnnamed_8 = 3;
pub const M_LEFT: C2RustUnnamed_8 = 2;
pub const M_TOP: C2RustUnnamed_8 = 1;
pub const M_RIGHT: C2RustUnnamed_8 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maze {
    pub ncells: libc::c_int,
    pub ngcells: libc::c_int,
    pub cells: *mut cell,
    pub gcells: *mut cell,
    pub sg: *mut sgraph,
    pub hchans: *mut Dt_t,
    pub vchans: *mut Dt_t,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct epair_t {
    pub d: libc::c_int,
    pub e: *mut Agedge_t,
}
pub type PointSet = Dict_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chanItem {
    pub link: Dtlink_t,
    pub v: libc::c_double,
    pub chans: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intitem {
    pub id: libc::c_int,
    pub link: Dtlink_t,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[no_mangle]
pub static mut odb_flags: libc::c_int = 0;
unsafe extern "C" fn cellOf(mut p: *mut snode, mut q: *mut snode) -> *mut cell {
    let mut cp: *mut cell = (*p).cells[0 as libc::c_int as usize];
    if cp == (*q).cells[0 as libc::c_int as usize]
        || cp == (*q).cells[1 as libc::c_int as usize]
    {
        return cp
    } else {
        return (*p).cells[1 as libc::c_int as usize]
    };
}
unsafe extern "C" fn midPt(mut cp: *mut cell) -> pointf {
    let mut p: pointf = pointf { x: 0., y: 0. };
    p.x = ((*cp).bb.LL.x + (*cp).bb.UR.x) / 2.0f64;
    p.y = ((*cp).bb.LL.y + (*cp).bb.UR.y) / 2.0f64;
    return p;
}
unsafe extern "C" fn sidePt(mut ptr: *mut snode, mut cp: *mut cell) -> pointf {
    let mut pt: pointf = pointf { x: 0., y: 0. };
    if cp == (*ptr).cells[1 as libc::c_int as usize] {
        if (*ptr).isVert {
            pt.x = (*cp).bb.LL.x;
            pt.y = ((*cp).bb.LL.y + (*cp).bb.UR.y) / 2.0f64;
        } else {
            pt.x = ((*cp).bb.LL.x + (*cp).bb.UR.x) / 2.0f64;
            pt.y = (*cp).bb.LL.y;
        }
    } else if (*ptr).isVert {
        pt.x = (*cp).bb.UR.x;
        pt.y = ((*cp).bb.LL.y + (*cp).bb.UR.y) / 2.0f64;
    } else {
        pt.x = ((*cp).bb.LL.x + (*cp).bb.UR.x) / 2.0f64;
        pt.y = (*cp).bb.UR.y;
    }
    return pt;
}
unsafe extern "C" fn setSeg(
    mut sp: *mut segment,
    mut dir: bool,
    mut fix: libc::c_double,
    mut b1: libc::c_double,
    mut b2: libc::c_double,
    mut l1: libc::c_int,
    mut l2: libc::c_int,
) {
    (*sp).isVert = dir;
    (*sp).comm_coord = fix;
    if b1 < b2 {
        (*sp).p.p1 = b1;
        (*sp).p.p2 = b2;
        (*sp).l1 = l1 as bend;
        (*sp).l2 = l2 as bend;
    } else {
        (*sp).p.p2 = b1;
        (*sp).p.p1 = b2;
        (*sp).l2 = l1 as bend;
        (*sp).l1 = l2 as bend;
    };
}
unsafe extern "C" fn convertSPtoRoute(
    mut g: *mut sgraph,
    mut fst: *mut snode,
    mut lst: *mut snode,
) -> route {
    let mut rte: route = route {
        n: 0,
        segs: 0 as *mut segment,
    };
    let mut ptr: *mut snode = 0 as *mut snode;
    let mut next: *mut snode = 0 as *mut snode;
    let mut prev: *mut snode = 0 as *mut snode;
    let mut sz: size_t = 0 as libc::c_int as size_t;
    let mut cp: *mut cell = 0 as *mut cell;
    let mut ncp: *mut cell = 0 as *mut cell;
    let mut seg: segment = segment {
        isVert: false,
        comm_coord: 0.,
        p: paird { p1: 0., p2: 0. },
        l1: B_NODE,
        l2: B_NODE,
        ind_no: 0,
        track_no: 0,
        prev: 0 as *mut segment,
        next: 0 as *mut segment,
    };
    let mut fix: libc::c_double = 0.;
    let mut b1: libc::c_double = 0.;
    let mut b2: libc::c_double = 0.;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut bp1: pointf = pointf { x: 0., y: 0. };
    let mut bp2: pointf = pointf { x: 0., y: 0. };
    let mut prevbp: pointf = {
        let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
        init
    };
    ptr = fst;
    while !ptr.is_null() {
        sz = sz.wrapping_add(1);
        ptr = (*ptr).n_dad;
    }
    rte.n = 0 as libc::c_int as size_t;
    if sz >= 2 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"sz >= 2\0" as *const u8 as *const libc::c_char,
            b"ortho.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"route convertSPtoRoute(sgraph *, snode *, snode *)\0"))
                .as_ptr(),
        );
    }
    rte
        .segs = gcalloc(
        sz.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<segment>() as libc::c_ulong,
    ) as *mut segment;
    seg.next = 0 as *mut segment;
    seg.prev = seg.next;
    prev = (*fst).n_dad;
    ptr = prev;
    next = (*ptr).n_dad;
    if (*(*ptr).cells[0 as libc::c_int as usize]).flags & 1 as libc::c_int != 0 {
        cp = (*ptr).cells[1 as libc::c_int as usize];
    } else {
        cp = (*ptr).cells[0 as libc::c_int as usize];
    }
    bp1 = sidePt(ptr, cp);
    while !((*next).n_dad).is_null() {
        ncp = cellOf(prev, next);
        updateWts(g, ncp, (*ptr).n_edge);
        if (*ptr).isVert as libc::c_int != (*next).isVert as libc::c_int
            || (*next).n_dad == lst
        {
            if (*ptr).isVert as libc::c_int != (*next).isVert as libc::c_int {
                bp2 = midPt(ncp);
            } else {
                bp2 = sidePt(next, ncp);
            }
            if (*ptr).isVert {
                if ptr == (*fst).n_dad {
                    l1 = B_NODE as libc::c_int;
                } else if prevbp.y > bp1.y {
                    l1 = B_UP as libc::c_int;
                } else {
                    l1 = B_DOWN as libc::c_int;
                }
                if (*ptr).isVert as libc::c_int != (*next).isVert as libc::c_int {
                    if (*next).cells[0 as libc::c_int as usize] == ncp {
                        l2 = B_UP as libc::c_int;
                    } else {
                        l2 = B_DOWN as libc::c_int;
                    }
                } else {
                    l2 = B_NODE as libc::c_int;
                }
                fix = (*cp).bb.LL.y;
                b1 = (*cp).bb.LL.x;
                b2 = (*ncp).bb.LL.x;
            } else {
                if ptr == (*fst).n_dad {
                    l1 = B_NODE as libc::c_int;
                } else if prevbp.x > bp1.x {
                    l1 = B_RIGHT as libc::c_int;
                } else {
                    l1 = B_LEFT as libc::c_int;
                }
                if (*ptr).isVert as libc::c_int != (*next).isVert as libc::c_int {
                    if (*next).cells[0 as libc::c_int as usize] == ncp {
                        l2 = B_RIGHT as libc::c_int;
                    } else {
                        l2 = B_LEFT as libc::c_int;
                    }
                } else {
                    l2 = B_NODE as libc::c_int;
                }
                fix = (*cp).bb.LL.x;
                b1 = (*cp).bb.LL.y;
                b2 = (*ncp).bb.LL.y;
            }
            setSeg(&mut seg, !(*ptr).isVert, fix, b1, b2, l1, l2);
            let fresh0 = rte.n;
            rte.n = (rte.n).wrapping_add(1);
            *(rte.segs).offset(fresh0 as isize) = seg;
            cp = ncp;
            prevbp = bp1;
            bp1 = bp2;
            if (*ptr).isVert as libc::c_int != (*next).isVert as libc::c_int
                && (*next).n_dad == lst
            {
                bp2 = sidePt(next, ncp);
                l2 = B_NODE as libc::c_int;
                if (*next).isVert {
                    if prevbp.y > bp1.y {
                        l1 = B_UP as libc::c_int;
                    } else {
                        l1 = B_DOWN as libc::c_int;
                    }
                    fix = (*cp).bb.LL.y;
                    b1 = (*cp).bb.LL.x;
                    b2 = (*ncp).bb.LL.x;
                } else {
                    if prevbp.x > bp1.x {
                        l1 = B_RIGHT as libc::c_int;
                    } else {
                        l1 = B_LEFT as libc::c_int;
                    }
                    fix = (*cp).bb.LL.x;
                    b1 = (*cp).bb.LL.y;
                    b2 = (*ncp).bb.LL.y;
                }
                setSeg(&mut seg, !(*next).isVert, fix, b1, b2, l1, l2);
                let fresh1 = rte.n;
                rte.n = (rte.n).wrapping_add(1);
                *(rte.segs).offset(fresh1 as isize) = seg;
            }
            ptr = next;
        }
        prev = next;
        next = (*next).n_dad;
    }
    rte
        .segs = realloc(
        rte.segs as *mut libc::c_void,
        (rte.n).wrapping_mul(::std::mem::size_of::<segment>() as libc::c_ulong),
    ) as *mut segment;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < rte.n {
        if i > 0 as libc::c_int as libc::c_ulong {
            let ref mut fresh2 = (*(rte.segs).offset(i as isize)).prev;
            *fresh2 = (rte.segs)
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        }
        if i < (rte.n).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let ref mut fresh3 = (*(rte.segs).offset(i as isize)).next;
            *fresh3 = (rte.segs)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        }
        i = i.wrapping_add(1);
    }
    return rte;
}
unsafe extern "C" fn freeChannel(
    mut d: *mut Dt_t,
    mut cp: *mut channel,
    mut disc: *mut Dtdisc_t,
) {
    free_graph((*cp).G);
    free((*cp).seg_list as *mut libc::c_void);
    free(cp as *mut libc::c_void);
}
unsafe extern "C" fn freeChanItem(
    mut d: *mut Dt_t,
    mut cp: *mut chanItem,
    mut disc: *mut Dtdisc_t,
) {
    dtclose((*cp).chans);
    free(cp as *mut libc::c_void);
}
unsafe extern "C" fn chancmpid(
    mut d: *mut Dt_t,
    mut key1: *mut paird,
    mut key2: *mut paird,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if (*key1).p1 > (*key2).p1 {
        if (*key1).p2 <= (*key2).p2 {
            return 0 as libc::c_int
        } else {
            return 1 as libc::c_int
        }
    } else if (*key1).p1 < (*key2).p1 {
        if (*key1).p2 >= (*key2).p2 {
            return 0 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn dcmpid(
    mut d: *mut Dt_t,
    mut key1: *mut libc::c_double,
    mut key2: *mut libc::c_double,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if *key1 > *key2 {
        return 1 as libc::c_int
    } else if *key1 < *key2 {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
static mut chanDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<paird>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut channel, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freeChannel
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut channel,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut paird,
                        *mut paird,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    chancmpid
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut paird,
                            *mut paird,
                            *mut Dtdisc_t,
                        ) -> libc::c_int,
                ),
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
static mut chanItemDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut chanItem, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freeChanItem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut chanItem,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    dcmpid
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut libc::c_double,
                            *mut libc::c_double,
                            *mut Dtdisc_t,
                        ) -> libc::c_int,
                ),
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn addChan(
    mut chdict: *mut Dt_t,
    mut cp: *mut channel,
    mut j: libc::c_double,
) {
    let mut subd: *mut chanItem = (Some(
        ((*chdict).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        chdict,
        &mut j as *mut libc::c_double as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut chanItem;
    if subd.is_null() {
        subd = zmalloc(::std::mem::size_of::<chanItem>() as libc::c_ulong)
            as *mut chanItem;
        (*subd).v = j;
        let ref mut fresh4 = (*subd).chans;
        *fresh4 = dtopen(&mut chanDisc, Dtoset);
        (Some(((*chdict).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(chdict, subd as *mut libc::c_void, 0o1 as libc::c_int);
    }
    (Some(((*(*subd).chans).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*subd).chans, cp as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn extractHChans(mut mp: *mut maze) -> *mut Dt_t {
    let mut i: libc::c_int = 0;
    let mut np: *mut snode = 0 as *mut snode;
    let mut hchans: *mut Dt_t = dtopen(&mut chanItemDisc, Dtoset);
    i = 0 as libc::c_int;
    while i < (*mp).ncells {
        let mut chp: *mut channel = 0 as *mut channel;
        let mut cp: *mut cell = ((*mp).cells).offset(i as isize);
        let mut nextcp: *mut cell = 0 as *mut cell;
        if !((*cp).flags & 4 as libc::c_int != 0) {
            loop {
                np = *((*cp).sides).offset(M_LEFT as libc::c_int as isize);
                if !(!np.is_null()
                    && {
                        nextcp = (*np).cells[0 as libc::c_int as usize];
                        !nextcp.is_null()
                    } && (*nextcp).flags & 1 as libc::c_int == 0)
                {
                    break;
                }
                cp = nextcp;
            }
            chp = zmalloc(::std::mem::size_of::<channel>() as libc::c_ulong)
                as *mut channel;
            let ref mut fresh5 = (*chp).cp;
            *fresh5 = cp;
            (*chp).p.p1 = (*cp).bb.LL.x;
            (*cp).flags |= 4 as libc::c_int;
            loop {
                np = *((*cp).sides).offset(M_RIGHT as libc::c_int as isize);
                if !(!np.is_null()
                    && {
                        nextcp = (*np).cells[1 as libc::c_int as usize];
                        !nextcp.is_null()
                    } && (*nextcp).flags & 1 as libc::c_int == 0)
                {
                    break;
                }
                cp = nextcp;
                (*cp).flags |= 4 as libc::c_int;
            }
            (*chp).p.p2 = (*cp).bb.UR.x;
            addChan(hchans, chp, (*(*chp).cp).bb.LL.y);
        }
        i += 1;
    }
    return hchans;
}
unsafe extern "C" fn extractVChans(mut mp: *mut maze) -> *mut Dt_t {
    let mut i: libc::c_int = 0;
    let mut np: *mut snode = 0 as *mut snode;
    let mut vchans: *mut Dt_t = dtopen(&mut chanItemDisc, Dtoset);
    i = 0 as libc::c_int;
    while i < (*mp).ncells {
        let mut chp: *mut channel = 0 as *mut channel;
        let mut cp: *mut cell = ((*mp).cells).offset(i as isize);
        let mut nextcp: *mut cell = 0 as *mut cell;
        if !((*cp).flags & 2 as libc::c_int != 0) {
            loop {
                np = *((*cp).sides).offset(M_BOTTOM as libc::c_int as isize);
                if !(!np.is_null()
                    && {
                        nextcp = (*np).cells[0 as libc::c_int as usize];
                        !nextcp.is_null()
                    } && (*nextcp).flags & 1 as libc::c_int == 0)
                {
                    break;
                }
                cp = nextcp;
            }
            chp = zmalloc(::std::mem::size_of::<channel>() as libc::c_ulong)
                as *mut channel;
            let ref mut fresh6 = (*chp).cp;
            *fresh6 = cp;
            (*chp).p.p1 = (*cp).bb.LL.y;
            (*cp).flags |= 2 as libc::c_int;
            loop {
                np = *((*cp).sides).offset(M_TOP as libc::c_int as isize);
                if !(!np.is_null()
                    && {
                        nextcp = (*np).cells[1 as libc::c_int as usize];
                        !nextcp.is_null()
                    } && (*nextcp).flags & 1 as libc::c_int == 0)
                {
                    break;
                }
                cp = nextcp;
                (*cp).flags |= 2 as libc::c_int;
            }
            (*chp).p.p2 = (*cp).bb.UR.y;
            addChan(vchans, chp, (*(*chp).cp).bb.LL.x);
        }
        i += 1;
    }
    return vchans;
}
unsafe extern "C" fn insertChan(mut chan: *mut channel, mut seg: *mut segment) {
    let ref mut fresh7 = (*chan).cnt;
    let fresh8 = *fresh7;
    *fresh7 = *fresh7 + 1;
    (*seg).ind_no = fresh8;
    let ref mut fresh9 = (*chan).seg_list;
    *fresh9 = if !((*chan).seg_list).is_null() {
        grealloc(
            (*chan).seg_list as *mut libc::c_void,
            ((*chan).cnt as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut segment>() as libc::c_ulong),
        ) as *mut *mut segment
    } else {
        gmalloc(
            ((*chan).cnt as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut segment>() as libc::c_ulong),
        ) as *mut *mut segment
    };
    let ref mut fresh10 = *((*chan).seg_list)
        .offset(((*chan).cnt - 1 as libc::c_int) as isize);
    *fresh10 = seg;
}
unsafe extern "C" fn chanSearch(
    mut chans: *mut Dt_t,
    mut seg: *mut segment,
) -> *mut channel {
    let mut cp: *mut channel = 0 as *mut channel;
    let mut chani: *mut chanItem = (Some(
        ((*chans).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        chans,
        &mut (*seg).comm_coord as *mut libc::c_double as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut chanItem;
    if !chani.is_null() {} else {
        __assert_fail(
            b"chani\0" as *const u8 as *const libc::c_char,
            b"ortho.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"channel *chanSearch(Dt_t *, segment *)\0"))
                .as_ptr(),
        );
    }
    cp = (Some(((*(*chani).chans).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*chani).chans,
        &mut (*seg).p as *mut paird as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut channel;
    if !cp.is_null() {} else {
        __assert_fail(
            b"cp\0" as *const u8 as *const libc::c_char,
            b"ortho.c\0" as *const u8 as *const libc::c_char,
            442 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"channel *chanSearch(Dt_t *, segment *)\0"))
                .as_ptr(),
        );
    }
    return cp;
}
unsafe extern "C" fn assignSegs(
    mut nrtes: size_t,
    mut route_list: *mut route,
    mut mp: *mut maze,
) {
    let mut chan: *mut channel = 0 as *mut channel;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < nrtes {
        let mut rte: route = *route_list.offset(i as isize);
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < rte.n {
            let mut seg: *mut segment = (rte.segs).offset(j as isize);
            if (*seg).isVert {
                chan = chanSearch((*mp).vchans, seg);
            } else {
                chan = chanSearch((*mp).hchans, seg);
            }
            insertChan(chan, seg);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn addLoop(
    mut sg: *mut sgraph,
    mut cp: *mut cell,
    mut dp: *mut snode,
    mut sp: *mut snode,
) {
    let mut i: libc::c_int = 0;
    let mut onTop: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cp).nsides {
        let mut onp: *mut snode = *((*cp).sides).offset(i as isize);
        if !(*onp).isVert {
            if (*onp).cells[0 as libc::c_int as usize] == cp {
                onTop = 1 as libc::c_int;
            } else {
                onTop = 0 as libc::c_int;
            }
            if onTop != 0 {
                createSEdge(sg, sp, onp, 0 as libc::c_int as libc::c_double);
            } else {
                createSEdge(sg, dp, onp, 0 as libc::c_int as libc::c_double);
            }
        }
        i += 1;
    }
    (*sg).nnodes += 2 as libc::c_int;
}
unsafe extern "C" fn addNodeEdges(
    mut sg: *mut sgraph,
    mut cp: *mut cell,
    mut np: *mut snode,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cp).nsides {
        let mut onp: *mut snode = *((*cp).sides).offset(i as isize);
        createSEdge(sg, np, onp, 0 as libc::c_int as libc::c_double);
        i += 1;
    }
    let ref mut fresh11 = (*sg).nnodes;
    *fresh11 += 1;
    let ref mut fresh12 = (*np).cells[1 as libc::c_int as usize];
    *fresh12 = cp;
    let ref mut fresh13 = (*np).cells[0 as libc::c_int as usize];
    *fresh13 = *fresh12;
}
unsafe extern "C" fn bendToStr(mut b: bend) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match b as libc::c_uint {
        0 => {
            s = b"B_NODE\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        1 => {
            s = b"B_UP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            s = b"B_LEFT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        3 => {
            s = b"B_DOWN\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            if b as libc::c_uint == B_RIGHT as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"b == B_RIGHT\0" as *const u8 as *const libc::c_char,
                    b"ortho.c\0" as *const u8 as *const libc::c_char,
                    530 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"char *bendToStr(bend)\0"))
                        .as_ptr(),
                );
            }
            s = b"B_RIGHT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    return s;
}
unsafe extern "C" fn putSeg(mut fp: *mut FILE, mut seg: *mut segment) {
    if (*seg).isVert {
        fprintf(
            fp,
            b"((%f,%f),(%f,%f)) %s %s\0" as *const u8 as *const libc::c_char,
            (*seg).comm_coord,
            (*seg).p.p1,
            (*seg).comm_coord,
            (*seg).p.p2,
            bendToStr((*seg).l1),
            bendToStr((*seg).l2),
        );
    } else {
        fprintf(
            fp,
            b"((%f,%f),(%f,%f)) %s %s\0" as *const u8 as *const libc::c_char,
            (*seg).p.p1,
            (*seg).comm_coord,
            (*seg).p.p2,
            (*seg).comm_coord,
            bendToStr((*seg).l1),
            bendToStr((*seg).l2),
        );
    };
}
unsafe extern "C" fn dumpChanG(mut cp: *mut channel, mut v: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut ip: *mut intitem = 0 as *mut intitem;
    let mut adj: *mut Dt_t = 0 as *mut Dt_t;
    if (*cp).cnt < 2 as libc::c_int {
        return;
    }
    fprintf(
        stderr,
        b"channel %d (%f,%f)\n\0" as *const u8 as *const libc::c_char,
        v,
        (*cp).p.p1,
        (*cp).p.p2,
    );
    k = 0 as libc::c_int;
    while k < (*cp).cnt {
        adj = (*((*(*cp).G).vertices).offset(k as isize)).adj_list;
        if !(dtsize(adj) == 0 as libc::c_int) {
            putSeg(stderr, *((*cp).seg_list).offset(k as isize));
            fputs(b" ->\n\0" as *const u8 as *const libc::c_char, stderr);
            ip = (Some(((*adj).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(adj, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut intitem;
            while !ip.is_null() {
                fputs(b"     \0" as *const u8 as *const libc::c_char, stderr);
                putSeg(stderr, *((*cp).seg_list).offset((*ip).id as isize));
                fputs(b"\n\0" as *const u8 as *const libc::c_char, stderr);
                ip = (Some(((*adj).searchf).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(adj, ip as *mut libc::c_void, 0o10 as libc::c_int) as *mut intitem;
            }
        }
        k += 1;
    }
}
unsafe extern "C" fn assignTrackNo(mut chans: *mut Dt_t) {
    let mut lp: *mut Dt_t = 0 as *mut Dt_t;
    let mut l1: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut l2: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut cp: *mut channel = 0 as *mut channel;
    let mut k: libc::c_int = 0;
    l1 = dtflatten(chans);
    while !l1.is_null() {
        lp = (*(l1 as *mut chanItem)).chans;
        l2 = dtflatten(lp);
        while !l2.is_null() {
            cp = l2 as *mut channel;
            if (*cp).cnt != 0 {
                if odb_flags & 8 as libc::c_int != 0 {
                    dumpChanG(cp, (*(l1 as *mut chanItem)).v as libc::c_int);
                }
                top_sort((*cp).G);
                k = 0 as libc::c_int;
                while k < (*cp).cnt {
                    (**((*cp).seg_list).offset(k as isize))
                        .track_no = (*((*(*cp).G).vertices).offset(k as isize))
                        .topsort_order + 1 as libc::c_int;
                    k += 1;
                }
            }
            l2 = (*l2).right;
        }
        l1 = (*l1).right;
    }
}
unsafe extern "C" fn create_graphs(mut chans: *mut Dt_t) {
    let mut lp: *mut Dt_t = 0 as *mut Dt_t;
    let mut l1: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut l2: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut cp: *mut channel = 0 as *mut channel;
    l1 = dtflatten(chans);
    while !l1.is_null() {
        lp = (*(l1 as *mut chanItem)).chans;
        l2 = dtflatten(lp);
        while !l2.is_null() {
            cp = l2 as *mut channel;
            let ref mut fresh14 = (*cp).G;
            *fresh14 = make_graph((*cp).cnt);
            l2 = (*l2).right;
        }
        l1 = (*l1).right;
    }
}
unsafe extern "C" fn eqEndSeg(
    mut S1l2: bend,
    mut S2l2: bend,
    mut T1: bend,
    mut T2: bend,
) -> libc::c_int {
    if S1l2 as libc::c_uint == T2 as libc::c_uint
        && S2l2 as libc::c_uint != T2 as libc::c_uint
        || S1l2 as libc::c_uint == B_NODE as libc::c_int as libc::c_uint
            && S2l2 as libc::c_uint == T1 as libc::c_uint
    {
        return 0 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn overlapSeg(
    mut S1: *mut segment,
    mut S2: *mut segment,
    mut T1: bend,
    mut T2: bend,
) -> libc::c_int {
    if (*S1).p.p2 < (*S2).p.p2 {
        if (*S1).l2 as libc::c_uint == T1 as libc::c_uint
            && (*S2).l1 as libc::c_uint == T2 as libc::c_uint
        {
            return -(1 as libc::c_int)
        } else if (*S1).l2 as libc::c_uint == T2 as libc::c_uint
                && (*S2).l1 as libc::c_uint == T1 as libc::c_uint
            {
            return 1 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    }
    if (*S1).p.p2 > (*S2).p.p2 {
        if (*S2).l1 as libc::c_uint == T2 as libc::c_uint
            && (*S2).l2 as libc::c_uint == T2 as libc::c_uint
        {
            return -(1 as libc::c_int)
        } else if (*S2).l1 as libc::c_uint == T1 as libc::c_uint
                && (*S2).l2 as libc::c_uint == T1 as libc::c_uint
            {
            return 1 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    }
    if (*S2).l1 as libc::c_uint == T2 as libc::c_uint {
        return eqEndSeg((*S1).l2, (*S2).l2, T1, T2);
    }
    return -(1 as libc::c_int) * eqEndSeg((*S2).l2, (*S1).l2, T1, T2);
}
unsafe extern "C" fn ellSeg(mut S1l1: bend, mut S1l2: bend, mut T: bend) -> libc::c_int {
    if S1l1 as libc::c_uint == T as libc::c_uint {
        if S1l2 as libc::c_uint == T as libc::c_uint {
            return -(1 as libc::c_int)
        } else {
            return 0 as libc::c_int
        }
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn segCmp(
    mut S1: *mut segment,
    mut S2: *mut segment,
    mut T1: bend,
    mut T2: bend,
) -> libc::c_int {
    if (*S1).p.p2 < (*S2).p.p1 || (*S1).p.p1 > (*S2).p.p2 {
        return 0 as libc::c_int;
    }
    if (*S1).p.p1 < (*S2).p.p1 && (*S2).p.p1 < (*S1).p.p2 {
        return overlapSeg(S1, S2, T1, T2);
    }
    if (*S2).p.p1 < (*S1).p.p1 && (*S1).p.p1 < (*S2).p.p2 {
        return -(1 as libc::c_int) * overlapSeg(S2, S1, T1, T2);
    }
    if (*S1).p.p1 == (*S2).p.p1 {
        if (*S1).p.p2 == (*S2).p.p2 {
            if (*S1).l1 as libc::c_uint == (*S2).l1 as libc::c_uint
                && (*S1).l2 as libc::c_uint == (*S2).l2 as libc::c_uint
            {
                return 0 as libc::c_int;
            }
            if (*S2).l1 as libc::c_uint == (*S2).l2 as libc::c_uint {
                if (*S2).l1 as libc::c_uint == T1 as libc::c_uint {
                    return 1 as libc::c_int;
                }
                if (*S2).l1 as libc::c_uint == T2 as libc::c_uint {
                    return -(1 as libc::c_int);
                }
                if (*S1).l1 as libc::c_uint != T1 as libc::c_uint
                    && (*S1).l2 as libc::c_uint != T1 as libc::c_uint
                {
                    return 1 as libc::c_int;
                }
                if (*S1).l1 as libc::c_uint != T2 as libc::c_uint
                    && (*S1).l2 as libc::c_uint != T2 as libc::c_uint
                {
                    return -(1 as libc::c_int);
                }
                return 0 as libc::c_int;
            }
            if (*S2).l1 as libc::c_uint == T1 as libc::c_uint
                && (*S2).l2 as libc::c_uint == T2 as libc::c_uint
            {
                if (*S1).l1 as libc::c_uint != T1 as libc::c_uint
                    && (*S1).l2 as libc::c_uint == T2 as libc::c_uint
                {
                    return 1 as libc::c_int;
                }
                if (*S1).l1 as libc::c_uint == T1 as libc::c_uint
                    && (*S1).l2 as libc::c_uint != T2 as libc::c_uint
                {
                    return -(1 as libc::c_int);
                }
                return 0 as libc::c_int;
            }
            if (*S2).l2 as libc::c_uint == T1 as libc::c_uint
                && (*S2).l1 as libc::c_uint == T2 as libc::c_uint
            {
                if (*S1).l2 as libc::c_uint != T1 as libc::c_uint
                    && (*S1).l1 as libc::c_uint == T2 as libc::c_uint
                {
                    return 1 as libc::c_int;
                }
                if (*S1).l2 as libc::c_uint == T1 as libc::c_uint
                    && (*S1).l1 as libc::c_uint != T2 as libc::c_uint
                {
                    return -(1 as libc::c_int);
                }
                return 0 as libc::c_int;
            }
            if (*S2).l1 as libc::c_uint == B_NODE as libc::c_int as libc::c_uint
                && (*S2).l2 as libc::c_uint == T1 as libc::c_uint
            {
                return ellSeg((*S1).l1, (*S1).l2, T1);
            }
            if (*S2).l1 as libc::c_uint == B_NODE as libc::c_int as libc::c_uint
                && (*S2).l2 as libc::c_uint == T2 as libc::c_uint
            {
                return -(1 as libc::c_int) * ellSeg((*S1).l1, (*S1).l2, T2);
            }
            if (*S2).l1 as libc::c_uint == T1 as libc::c_uint
                && (*S2).l2 as libc::c_uint == B_NODE as libc::c_int as libc::c_uint
            {
                return ellSeg((*S1).l2, (*S1).l1, T1);
            }
            return -(1 as libc::c_int) * ellSeg((*S1).l2, (*S1).l1, T2);
        }
        if (*S1).p.p2 < (*S2).p.p2 {
            if (*S1).l2 as libc::c_uint == T1 as libc::c_uint {
                return eqEndSeg((*S2).l1, (*S1).l1, T1, T2);
            }
            return -(1 as libc::c_int) * eqEndSeg((*S2).l1, (*S1).l1, T1, T2);
        }
        if (*S2).l2 as libc::c_uint == T2 as libc::c_uint {
            return eqEndSeg((*S1).l1, (*S2).l1, T1, T2);
        }
        return -(1 as libc::c_int) * eqEndSeg((*S1).l1, (*S2).l1, T1, T2);
    }
    if (*S1).p.p2 == (*S2).p.p1 {
        if (*S1).l2 as libc::c_uint == (*S2).l1 as libc::c_uint {
            return 0 as libc::c_int;
        }
        if (*S1).l2 as libc::c_uint == T2 as libc::c_uint {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if (*S1).l1 as libc::c_uint == (*S2).l2 as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*S1).l1 as libc::c_uint == T2 as libc::c_uint {
        return 1 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn seg_cmp(mut S1: *mut segment, mut S2: *mut segment) -> libc::c_int {
    if (*S1).isVert as libc::c_int != (*S2).isVert as libc::c_int
        || (*S1).comm_coord != (*S2).comm_coord
    {
        agerr(
            AGERR,
            b"incomparable segments !! -- Aborting\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(2 as libc::c_int);
    }
    if (*S1).isVert {
        return segCmp(S1, S2, B_RIGHT, B_LEFT)
    } else {
        return segCmp(S1, S2, B_DOWN, B_UP)
    };
}
unsafe extern "C" fn add_edges_in_G(mut cp: *mut channel) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut seg_list: *mut *mut segment = (*cp).seg_list;
    let mut size: libc::c_int = (*cp).cnt;
    let mut G: *mut rawgraph = (*cp).G;
    x = 0 as libc::c_int;
    while (x + 1 as libc::c_int) < size {
        y = x + 1 as libc::c_int;
        while y < size {
            let mut cmp: libc::c_int = seg_cmp(
                *seg_list.offset(x as isize),
                *seg_list.offset(y as isize),
            );
            if cmp == -(2 as libc::c_int) {
                return -(1 as libc::c_int)
            } else {
                if cmp > 0 as libc::c_int {
                    insert_edge(G, x, y);
                } else if cmp == -(1 as libc::c_int) {
                    insert_edge(G, y, x);
                }
            }
            y += 1;
        }
        x += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_np_edges(mut chans: *mut Dt_t) -> libc::c_int {
    let mut lp: *mut Dt_t = 0 as *mut Dt_t;
    let mut l1: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut l2: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut cp: *mut channel = 0 as *mut channel;
    l1 = dtflatten(chans);
    while !l1.is_null() {
        lp = (*(l1 as *mut chanItem)).chans;
        l2 = dtflatten(lp);
        while !l2.is_null() {
            cp = l2 as *mut channel;
            if (*cp).cnt != 0 {
                if add_edges_in_G(cp) != 0 {
                    return -(1 as libc::c_int);
                }
            }
            l2 = (*l2).right;
        }
        l1 = (*l1).right;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_seg(
    mut seg: *mut segment,
    mut dir: libc::c_int,
) -> *mut segment {
    if !seg.is_null() {} else {
        __assert_fail(
            b"seg\0" as *const u8 as *const libc::c_char,
            b"ortho.c\0" as *const u8 as *const libc::c_char,
            787 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"segment *next_seg(segment *, int)\0"))
                .as_ptr(),
        );
    }
    if dir == 0 { return (*seg).prev } else { return (*seg).next };
}
unsafe extern "C" fn propagate_prec(
    mut seg: *mut segment,
    mut prec: libc::c_int,
    mut hops: libc::c_int,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut ans: libc::c_int = prec;
    let mut next: *mut segment = 0 as *mut segment;
    let mut current: *mut segment = 0 as *mut segment;
    current = seg;
    x = 1 as libc::c_int;
    while x <= hops {
        next = next_seg(current, dir);
        if !(*current).isVert {
            if (*next).comm_coord == (*current).p.p1 {
                if (*current).l1 as libc::c_uint == B_UP as libc::c_int as libc::c_uint {
                    ans *= -(1 as libc::c_int);
                }
            } else if (*current).l2 as libc::c_uint
                    == B_DOWN as libc::c_int as libc::c_uint
                {
                ans *= -(1 as libc::c_int);
            }
        } else if (*next).comm_coord == (*current).p.p1 {
            if (*current).l1 as libc::c_uint == B_RIGHT as libc::c_int as libc::c_uint {
                ans *= -(1 as libc::c_int);
            }
        } else if (*current).l2 as libc::c_uint == B_LEFT as libc::c_int as libc::c_uint
            {
            ans *= -(1 as libc::c_int);
        }
        current = next;
        x += 1;
    }
    return ans;
}
unsafe extern "C" fn is_parallel(mut s1: *mut segment, mut s2: *mut segment) -> bool {
    if (*s1).comm_coord == (*s2).comm_coord {} else {
        __assert_fail(
            b"s1->comm_coord==s2->comm_coord\0" as *const u8 as *const libc::c_char,
            b"ortho.c\0" as *const u8 as *const libc::c_char,
            832 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"_Bool is_parallel(segment *, segment *)\0"))
                .as_ptr(),
        );
    }
    return (*s1).p.p1 == (*s2).p.p1 && (*s1).p.p2 == (*s2).p.p2
        && (*s1).l1 as libc::c_uint == (*s2).l1 as libc::c_uint
        && (*s1).l2 as libc::c_uint == (*s2).l2 as libc::c_uint;
}
unsafe extern "C" fn decide_point(
    mut ret: *mut pair,
    mut si: *mut segment,
    mut sj: *mut segment,
    mut dir1: libc::c_int,
    mut dir2: libc::c_int,
) -> libc::c_int {
    let mut prec: libc::c_int = 0;
    let mut ans: libc::c_int = 0 as libc::c_int;
    let mut temp: libc::c_int = 0;
    let mut np1: *mut segment = 0 as *mut segment;
    let mut np2: *mut segment = 0 as *mut segment;
    loop {
        np1 = next_seg(si, dir1);
        if !(!np1.is_null()
            && {
                np2 = next_seg(sj, dir2);
                !np2.is_null()
            } && is_parallel(np1, np2) as libc::c_int != 0)
        {
            break;
        }
        ans += 1;
        si = np1;
        sj = np2;
    }
    if np1.is_null() {
        prec = 0 as libc::c_int;
    } else if np2.is_null() {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"ortho.c\0" as *const u8 as *const libc::c_char,
            860 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int decide_point(pair *, segment *, segment *, int, int)\0"))
                .as_ptr(),
        );
    } else {
        temp = seg_cmp(np1, np2);
        if temp == -(2 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        prec = propagate_prec(
            np1,
            temp,
            ans + 1 as libc::c_int,
            1 as libc::c_int - dir1,
        );
    }
    (*ret).a = ans;
    (*ret).b = prec;
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_parallel_edges(
    mut seg1: *mut segment,
    mut seg2: *mut segment,
    mut dir1: libc::c_int,
    mut dir2: libc::c_int,
    mut hops: libc::c_int,
    mut mp: *mut maze,
) {
    let mut x: libc::c_int = 0;
    let mut chan: *mut channel = 0 as *mut channel;
    let mut nchan: *mut channel = 0 as *mut channel;
    let mut prev1: *mut segment = 0 as *mut segment;
    let mut prev2: *mut segment = 0 as *mut segment;
    if (*seg1).isVert {
        chan = chanSearch((*mp).vchans, seg1);
    } else {
        chan = chanSearch((*mp).hchans, seg1);
    }
    insert_edge((*chan).G, (*seg1).ind_no, (*seg2).ind_no);
    x = 1 as libc::c_int;
    while x <= hops {
        prev1 = next_seg(seg1, dir1);
        prev2 = next_seg(seg2, dir2);
        if !(*seg1).isVert {
            nchan = chanSearch((*mp).vchans, prev1);
            if (*prev1).comm_coord == (*seg1).p.p1 {
                if (*seg1).l1 as libc::c_uint == B_UP as libc::c_int as libc::c_uint {
                    if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                        insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
                    } else {
                        insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
                    }
                } else if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                    insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
                } else {
                    insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
                }
            } else if (*seg1).l2 as libc::c_uint == B_UP as libc::c_int as libc::c_uint {
                if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                    insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
                } else {
                    insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
                }
            } else if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
            } else {
                insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
            }
        } else {
            nchan = chanSearch((*mp).hchans, prev1);
            if (*prev1).comm_coord == (*seg1).p.p1 {
                if (*seg1).l1 as libc::c_uint == B_LEFT as libc::c_int as libc::c_uint {
                    if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                        insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
                    } else {
                        insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
                    }
                } else if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                    insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
                } else {
                    insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
                }
            } else if (*seg1).l2 as libc::c_uint == B_LEFT as libc::c_int as libc::c_uint
                {
                if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                    insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
                } else {
                    insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
                }
            } else if edge_exists((*chan).G, (*seg1).ind_no, (*seg2).ind_no) {
                insert_edge((*nchan).G, (*prev1).ind_no, (*prev2).ind_no);
            } else {
                insert_edge((*nchan).G, (*prev2).ind_no, (*prev1).ind_no);
            }
        }
        chan = nchan;
        seg1 = prev1;
        seg2 = prev2;
        x += 1;
    }
}
unsafe extern "C" fn removeEdge(
    mut seg1: *mut segment,
    mut seg2: *mut segment,
    mut dir: libc::c_int,
    mut mp: *mut maze,
) {
    let mut ptr1: *mut segment = 0 as *mut segment;
    let mut ptr2: *mut segment = 0 as *mut segment;
    let mut chan: *mut channel = 0 as *mut channel;
    ptr1 = seg1;
    ptr2 = seg2;
    while is_parallel(ptr1, ptr2) {
        ptr1 = next_seg(ptr1, 1 as libc::c_int);
        ptr2 = next_seg(ptr2, dir);
    }
    if (*ptr1).isVert {
        chan = chanSearch((*mp).vchans, ptr1);
    } else {
        chan = chanSearch((*mp).hchans, ptr1);
    }
    remove_redge((*chan).G, (*ptr1).ind_no, (*ptr2).ind_no);
}
unsafe extern "C" fn addPEdges(mut cp: *mut channel, mut mp: *mut maze) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut hops: pair = pair { a: 0, b: 0 };
    let mut prec1: libc::c_int = 0;
    let mut prec2: libc::c_int = 0;
    let mut p: pair = pair { a: 0, b: 0 };
    let mut G: *mut rawgraph = (*cp).G;
    let mut segs: *mut *mut segment = (*cp).seg_list;
    i = 0 as libc::c_int;
    while (i + 1 as libc::c_int) < (*cp).cnt {
        j = i + 1 as libc::c_int;
        while j < (*cp).cnt {
            if !edge_exists(G, i, j) && !edge_exists(G, j, i) {
                if is_parallel(*segs.offset(i as isize), *segs.offset(j as isize)) {
                    if ((**segs.offset(i as isize)).prev).is_null() {
                        if ((**segs.offset(j as isize)).prev).is_null() {
                            dir = 0 as libc::c_int;
                        } else {
                            dir = 1 as libc::c_int;
                        }
                    } else if ((**segs.offset(j as isize)).prev).is_null() {
                        dir = 1 as libc::c_int;
                    } else if (*(**segs.offset(i as isize)).prev).comm_coord
                            == (*(**segs.offset(j as isize)).prev).comm_coord
                        {
                        dir = 0 as libc::c_int;
                    } else {
                        dir = 1 as libc::c_int;
                    }
                    if decide_point(
                        &mut p,
                        *segs.offset(i as isize),
                        *segs.offset(j as isize),
                        0 as libc::c_int,
                        dir,
                    ) != 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    hops.a = p.a;
                    prec1 = p.b;
                    if decide_point(
                        &mut p,
                        *segs.offset(i as isize),
                        *segs.offset(j as isize),
                        1 as libc::c_int,
                        1 as libc::c_int - dir,
                    ) != 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    hops.b = p.a;
                    prec2 = p.b;
                    if prec1 == -(1 as libc::c_int) {
                        set_parallel_edges(
                            *segs.offset(j as isize),
                            *segs.offset(i as isize),
                            dir,
                            0 as libc::c_int,
                            hops.a,
                            mp,
                        );
                        set_parallel_edges(
                            *segs.offset(j as isize),
                            *segs.offset(i as isize),
                            1 as libc::c_int - dir,
                            1 as libc::c_int,
                            hops.b,
                            mp,
                        );
                        if prec2 == 1 as libc::c_int {
                            removeEdge(
                                *segs.offset(i as isize),
                                *segs.offset(j as isize),
                                1 as libc::c_int - dir,
                                mp,
                            );
                        }
                    } else if prec1 == 0 as libc::c_int {
                        if prec2 == -(1 as libc::c_int) {
                            set_parallel_edges(
                                *segs.offset(j as isize),
                                *segs.offset(i as isize),
                                dir,
                                0 as libc::c_int,
                                hops.a,
                                mp,
                            );
                            set_parallel_edges(
                                *segs.offset(j as isize),
                                *segs.offset(i as isize),
                                1 as libc::c_int - dir,
                                1 as libc::c_int,
                                hops.b,
                                mp,
                            );
                        } else if prec2 == 0 as libc::c_int {
                            set_parallel_edges(
                                *segs.offset(i as isize),
                                *segs.offset(j as isize),
                                0 as libc::c_int,
                                dir,
                                hops.a,
                                mp,
                            );
                            set_parallel_edges(
                                *segs.offset(i as isize),
                                *segs.offset(j as isize),
                                1 as libc::c_int,
                                1 as libc::c_int - dir,
                                hops.b,
                                mp,
                            );
                        } else if prec2 == 1 as libc::c_int {
                            set_parallel_edges(
                                *segs.offset(i as isize),
                                *segs.offset(j as isize),
                                0 as libc::c_int,
                                dir,
                                hops.a,
                                mp,
                            );
                            set_parallel_edges(
                                *segs.offset(i as isize),
                                *segs.offset(j as isize),
                                1 as libc::c_int,
                                1 as libc::c_int - dir,
                                hops.b,
                                mp,
                            );
                        }
                    } else if prec1 == 1 as libc::c_int {
                        set_parallel_edges(
                            *segs.offset(i as isize),
                            *segs.offset(j as isize),
                            0 as libc::c_int,
                            dir,
                            hops.a,
                            mp,
                        );
                        set_parallel_edges(
                            *segs.offset(i as isize),
                            *segs.offset(j as isize),
                            1 as libc::c_int,
                            1 as libc::c_int - dir,
                            hops.b,
                            mp,
                        );
                        if prec2 == -(1 as libc::c_int) {
                            removeEdge(
                                *segs.offset(i as isize),
                                *segs.offset(j as isize),
                                1 as libc::c_int - dir,
                                mp,
                            );
                        }
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_p_edges(
    mut chans: *mut Dt_t,
    mut mp: *mut maze,
) -> libc::c_int {
    let mut lp: *mut Dt_t = 0 as *mut Dt_t;
    let mut l1: *mut Dtlink_t = 0 as *mut Dtlink_t;
    let mut l2: *mut Dtlink_t = 0 as *mut Dtlink_t;
    l1 = dtflatten(chans);
    while !l1.is_null() {
        lp = (*(l1 as *mut chanItem)).chans;
        l2 = dtflatten(lp);
        while !l2.is_null() {
            if addPEdges(l2 as *mut channel, mp) != 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            l2 = (*l2).right;
        }
        l1 = (*l1).right;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn assignTracks(mut mp: *mut maze) -> libc::c_int {
    create_graphs((*mp).hchans);
    create_graphs((*mp).vchans);
    if add_np_edges((*mp).hchans) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if add_np_edges((*mp).vchans) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if add_p_edges((*mp).hchans, mp) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if add_p_edges((*mp).vchans, mp) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    assignTrackNo((*mp).hchans);
    assignTrackNo((*mp).vchans);
    return 0 as libc::c_int;
}
unsafe extern "C" fn vtrack(mut seg: *mut segment, mut m: *mut maze) -> libc::c_double {
    let mut chp: *mut channel = chanSearch((*m).vchans, seg);
    let mut f: libc::c_double = (*seg).track_no as libc::c_double
        / ((*chp).cnt + 1 as libc::c_int) as libc::c_double;
    let mut left: libc::c_double = (*(*chp).cp).bb.LL.x;
    let mut right: libc::c_double = (*(*chp).cp).bb.UR.x;
    return left + f * (right - left);
}
unsafe extern "C" fn htrack(mut seg: *mut segment, mut m: *mut maze) -> libc::c_int {
    let mut chp: *mut channel = chanSearch((*m).hchans, seg);
    let mut f: libc::c_double = 1.0f64
        - (*seg).track_no as libc::c_double
            / ((*chp).cnt + 1 as libc::c_int) as libc::c_double;
    let mut lo: libc::c_double = (*(*chp).cp).bb.LL.y;
    let mut hi: libc::c_double = (*(*chp).cp).bb.UR.y;
    return (lo + f * (hi - lo)) as libc::c_int;
}
unsafe extern "C" fn addPoints(mut p0: pointf, mut p1: pointf) -> pointf {
    p0.x += p1.x;
    p0.y += p1.y;
    return p0;
}
unsafe extern "C" fn attachOrthoEdges(
    mut mp: *mut maze,
    mut n_edges: size_t,
    mut route_list: *mut route,
    mut sinfo_0: *mut splineInfo,
    mut es: *mut epair_t,
    mut doLbls: libc::c_int,
) {
    let mut ipt: libc::c_int = 0;
    let mut ispline: *mut pointf = 0 as *mut pointf;
    let mut splsz: size_t = 0 as libc::c_int as size_t;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut p1: pointf = pointf { x: 0., y: 0. };
    let mut q1: pointf = pointf { x: 0., y: 0. };
    let mut rte: route = route {
        n: 0,
        segs: 0 as *mut segment,
    };
    let mut seg: *mut segment = 0 as *mut segment;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut lbl: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut irte: size_t = 0 as libc::c_int as size_t;
    while irte < n_edges {
        e = (*es.offset(irte as isize)).e;
        p1 = addPoints(
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
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
        );
        q1 = addPoints(
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
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
        );
        rte = *route_list.offset(irte as isize);
        let mut npts: size_t = (1 as libc::c_int as libc::c_ulong)
            .wrapping_add((3 as libc::c_int as libc::c_ulong).wrapping_mul(rte.n));
        if npts > splsz {
            free(ispline as *mut libc::c_void);
            ispline = gcalloc(npts, ::std::mem::size_of::<pointf>() as libc::c_ulong)
                as *mut pointf;
            splsz = npts;
        }
        seg = rte.segs;
        if (*seg).isVert {
            p.x = vtrack(seg, mp);
            p.y = p1.y;
        } else {
            p.y = htrack(seg, mp) as libc::c_double;
            p.x = p1.x;
        }
        let ref mut fresh15 = *ispline.offset(1 as libc::c_int as isize);
        *fresh15 = p;
        *ispline.offset(0 as libc::c_int as isize) = *fresh15;
        ipt = 2 as libc::c_int;
        let mut i: size_t = 1 as libc::c_int as size_t;
        while i < rte.n {
            seg = (rte.segs).offset(i as isize);
            if (*seg).isVert {
                p.x = vtrack(seg, mp);
            } else {
                p.y = htrack(seg, mp) as libc::c_double;
            }
            let ref mut fresh16 = *ispline.offset(ipt as isize);
            *fresh16 = p;
            let ref mut fresh17 = *ispline.offset((ipt + 1 as libc::c_int) as isize);
            *fresh17 = *fresh16;
            *ispline.offset((ipt + 2 as libc::c_int) as isize) = *fresh17;
            ipt += 3 as libc::c_int;
            i = i.wrapping_add(1);
        }
        if (*seg).isVert {
            p.x = vtrack(seg, mp);
            p.y = q1.y;
        } else {
            p.y = htrack(seg, mp) as libc::c_double;
            p.x = q1.x;
        }
        let ref mut fresh18 = *ispline.offset((ipt + 1 as libc::c_int) as isize);
        *fresh18 = p;
        *ispline.offset(ipt as isize) = *fresh18;
        if Verbose as libc::c_int > 1 as libc::c_int {
            fprintf(
                stderr,
                b"ortho %s %s\n\0" as *const u8 as *const libc::c_char,
                agnameof(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node as *mut libc::c_void,
                ),
                agnameof(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node as *mut libc::c_void,
                ),
            );
        }
        clip_and_install(
            e,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            ispline,
            npts as libc::c_int,
            sinfo_0,
        );
        if doLbls != 0
            && {
                lbl = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
                !lbl.is_null()
            } && !(*lbl).set
        {
            addEdgeLabels(e);
        }
        irte = irte.wrapping_add(1);
    }
    free(ispline as *mut libc::c_void);
}
unsafe extern "C" fn edgeLen(mut e: *mut Agedge_t) -> libc::c_int {
    let mut p: pointf = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord;
    let mut q: pointf = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord;
    return ((p.x - q.x) * (p.x - q.x) + (p.y - q.y) * (p.y - q.y)) as libc::c_int;
}
unsafe extern "C" fn edgecmp(mut e0: *mut epair_t, mut e1: *mut epair_t) -> libc::c_int {
    return (*e0).d - (*e1).d;
}
unsafe extern "C" fn spline_merge(mut n: *mut node_t) -> bool {
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn swap_ends_p(mut e: *mut edge_t) -> bool {
    return 0 as libc::c_int != 0;
}
static mut sinfo: splineInfo = unsafe {
    {
        let mut init = splineInfo {
            swapEnds: Some(swap_ends_p as unsafe extern "C" fn(*mut edge_t) -> bool),
            splineMerge: Some(spline_merge as unsafe extern "C" fn(*mut node_t) -> bool),
            ignoreSwap: 1 as libc::c_int != 0,
            isOrtho: 1 as libc::c_int != 0,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn orthoEdges(mut g: *mut Agraph_t, mut doLbls: libc::c_int) {
    let mut current_block: u64;
    let mut sg: *mut sgraph = 0 as *mut sgraph;
    let mut mp: *mut maze = 0 as *mut maze;
    let mut route_list: *mut route = 0 as *mut route;
    let mut gstart: libc::c_int = 0;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sn: *mut snode = 0 as *mut snode;
    let mut dn: *mut snode = 0 as *mut snode;
    let mut es: *mut epair_t = gcalloc(
        agnedges(g) as size_t,
        ::std::mem::size_of::<epair_t>() as libc::c_ulong,
    ) as *mut epair_t;
    let mut start: *mut cell = 0 as *mut cell;
    let mut dest: *mut cell = 0 as *mut cell;
    let mut ps: *mut PointSet = 0 as *mut PointSet;
    let mut lbl: *mut textlabel_t = 0 as *mut textlabel_t;
    if Concentrate != 0 {
        ps = newPS();
    }
    let mut s: *mut libc::c_char = agget(
        g as *mut libc::c_void,
        b"odb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut c: libc::c_char = 0;
    odb_flags = 0 as libc::c_int;
    if !s.is_null() && *s as libc::c_int != '\0' as i32 {
        loop {
            let fresh19 = s;
            s = s.offset(1);
            c = *fresh19;
            if !(c != 0) {
                break;
            }
            match c as libc::c_int {
                99 => {
                    odb_flags |= 8 as libc::c_int;
                }
                105 => {
                    odb_flags |= 2 as libc::c_int | 16 as libc::c_int;
                }
                109 => {
                    odb_flags |= 1 as libc::c_int;
                }
                114 => {
                    odb_flags |= 4 as libc::c_int;
                }
                115 => {
                    odb_flags |= 2 as libc::c_int;
                }
                _ => {}
            }
        }
    }
    if doLbls != 0 {
        agerr(
            AGWARN,
            b"Orthogonal edges do not currently handle edge labels. Try using xlabels.\n\0"
                as *const u8 as *const libc::c_char,
        );
        doLbls = 0 as libc::c_int;
    }
    mp = mkMaze(g);
    sg = (*mp).sg;
    if odb_flags & 2 as libc::c_int != 0 {
        emitSearchGraph(stderr, sg);
    }
    let mut n_edges: size_t = 0 as libc::c_int as size_t;
    n = agfstnode(g);
    while !n.is_null() {
        let mut current_block_28: u64;
        e = agfstout(g, n);
        while !e.is_null() {
            if !(Nop == 2 as libc::c_int
                && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
                    .is_null())
            {
                if Concentrate != 0 {
                    let mut ti: libc::c_int = ((*((*if ((*(e as *mut Agobj_t)).tag)
                        .objtype() as libc::c_int == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node as *mut Agobj_t))
                        .tag)
                        .seq() as libc::c_int;
                    let mut hi: libc::c_int = ((*((*if ((*(e as *mut Agobj_t)).tag)
                        .objtype() as libc::c_int == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node as *mut Agobj_t))
                        .tag)
                        .seq() as libc::c_int;
                    if ti <= hi {
                        if isInPS(ps, ti, hi) != 0 {
                            current_block_28 = 11048769245176032998;
                        } else {
                            addPS(ps, ti, hi);
                            current_block_28 = 11763295167351361500;
                        }
                    } else if isInPS(ps, hi, ti) != 0 {
                        current_block_28 = 11048769245176032998;
                    } else {
                        addPS(ps, hi, ti);
                        current_block_28 = 11763295167351361500;
                    }
                } else {
                    current_block_28 = 11763295167351361500;
                }
                match current_block_28 {
                    11048769245176032998 => {}
                    _ => {
                        let ref mut fresh20 = (*es.offset(n_edges as isize)).e;
                        *fresh20 = e;
                        (*es.offset(n_edges as isize)).d = edgeLen(e);
                        n_edges = n_edges.wrapping_add(1);
                    }
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    route_list = gcalloc(n_edges, ::std::mem::size_of::<route>() as libc::c_ulong)
        as *mut route;
    qsort(
        es as *mut libc::c_void,
        n_edges,
        ::std::mem::size_of::<epair_t>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut epair_t, *mut epair_t) -> libc::c_int>,
            qsort_cmpf,
        >(
            Some(
                edgecmp
                    as unsafe extern "C" fn(*mut epair_t, *mut epair_t) -> libc::c_int,
            ),
        ),
    );
    gstart = (*sg).nnodes;
    PQgen((*sg).nnodes + 2 as libc::c_int);
    sn = &mut *((*sg).nodes).offset(gstart as isize) as *mut snode;
    dn = &mut *((*sg).nodes).offset((gstart + 1 as libc::c_int) as isize) as *mut snode;
    let mut i: size_t = 0 as libc::c_int as size_t;
    loop {
        if !(i < n_edges) {
            current_block = 4746626699541760585;
            break;
        }
        if i > 0 as libc::c_int as libc::c_ulong && odb_flags & 16 as libc::c_int != 0 {
            emitSearchGraph(stderr, sg);
        }
        e = (*es.offset(i as isize)).e;
        start = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .alg as *mut cell;
        dest = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .alg as *mut cell;
        if !(doLbls != 0
            && {
                lbl = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
                !lbl.is_null()
            } && (*lbl).set as libc::c_int != 0)
        {
            if start == dest {
                addLoop(sg, start, dn, sn);
            } else {
                addNodeEdges(sg, dest, dn);
                addNodeEdges(sg, start, sn);
            }
            if shortPath(sg, dn, sn) != 0 {
                current_block = 11686457702477534238;
                break;
            }
        }
        *route_list.offset(i as isize) = convertSPtoRoute(sg, sn, dn);
        reset(sg);
        i = i.wrapping_add(1);
    }
    match current_block {
        4746626699541760585 => {
            PQfree();
            let ref mut fresh21 = (*mp).hchans;
            *fresh21 = extractHChans(mp);
            let ref mut fresh22 = (*mp).vchans;
            *fresh22 = extractVChans(mp);
            assignSegs(n_edges, route_list, mp);
            if !(assignTracks(mp) != 0 as libc::c_int) {
                if odb_flags & 4 as libc::c_int != 0 {
                    emitGraph(stderr, mp, n_edges, route_list, es);
                }
                attachOrthoEdges(mp, n_edges, route_list, &mut sinfo, es, doLbls);
            }
        }
        _ => {}
    }
    if Concentrate != 0 {
        freePS(ps);
    }
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < n_edges {
        free((*route_list.offset(i_0 as isize)).segs as *mut libc::c_void);
        i_0 = i_0.wrapping_add(1);
    }
    free(route_list as *mut libc::c_void);
    freeMaze(mp);
    free(es as *mut libc::c_void);
}
static mut prolog2: *mut libc::c_char = b"%%!PS-Adobe-2.0\n%%%%BoundingBox: (atend)\n/point {\n  /Y exch def\n  /X exch def\n  newpath\n  X Y 3 0 360 arc fill\n} def\n/cell {\n  /Y exch def\n  /X exch def\n  /y exch def\n  /x exch def\n  newpath\n  x y moveto\n  x Y lineto\n  X Y lineto\n  X y lineto\n  closepath stroke\n} def\n/node {\n /u exch def\n /r exch def\n /d exch def\n /l exch def\n newpath l d moveto\n r d lineto r u lineto l u lineto\n closepath fill\n} def\n\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut epilog2: *mut libc::c_char = b"showpage\n%%%%Trailer\n%%%%BoundingBox: %d %d %d %d\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn coordOf(mut cp: *mut cell, mut np: *mut snode) -> point {
    let mut p: point = point { x: 0, y: 0 };
    if *((*cp).sides).offset(M_TOP as libc::c_int as isize) == np {
        p
            .x = (((*cp).bb.LL.x + (*cp).bb.UR.x) / 2 as libc::c_int as libc::c_double)
            as libc::c_int;
        p.y = (*cp).bb.UR.y as libc::c_int;
    } else if *((*cp).sides).offset(M_BOTTOM as libc::c_int as isize) == np {
        p
            .x = (((*cp).bb.LL.x + (*cp).bb.UR.x) / 2 as libc::c_int as libc::c_double)
            as libc::c_int;
        p.y = (*cp).bb.LL.y as libc::c_int;
    } else if *((*cp).sides).offset(M_LEFT as libc::c_int as isize) == np {
        p
            .y = (((*cp).bb.LL.y + (*cp).bb.UR.y) / 2 as libc::c_int as libc::c_double)
            as libc::c_int;
        p.x = (*cp).bb.LL.x as libc::c_int;
    } else if *((*cp).sides).offset(M_RIGHT as libc::c_int as isize) == np {
        p
            .y = (((*cp).bb.LL.y + (*cp).bb.UR.y) / 2 as libc::c_int as libc::c_double)
            as libc::c_int;
        p.x = (*cp).bb.UR.x as libc::c_int;
    } else {
        agerr(
            AGERR,
            b"Node not adjacent to cell -- Aborting\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
unsafe extern "C" fn emitEdge(
    mut fp: *mut FILE,
    mut e: *mut Agedge_t,
    mut rte: route,
    mut m: *mut maze,
    mut bb: boxf,
) -> boxf {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut n: boxf = (*((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .alg as *mut cell))
        .bb;
    let mut seg: *mut segment = rte.segs;
    if (*seg).isVert {
        x = vtrack(seg, m) as libc::c_int;
        y = ((n.UR.y + n.LL.y) / 2 as libc::c_int as libc::c_double) as libc::c_int;
    } else {
        y = htrack(seg, m);
        x = ((n.UR.x + n.LL.x) / 2 as libc::c_int as libc::c_double) as libc::c_int;
    }
    bb
        .LL
        .x = if bb.LL.x < (1 as libc::c_int * x) as libc::c_double {
        bb.LL.x
    } else {
        (1 as libc::c_int * x) as libc::c_double
    };
    bb
        .LL
        .y = if bb.LL.y < (1 as libc::c_int * y) as libc::c_double {
        bb.LL.y
    } else {
        (1 as libc::c_int * y) as libc::c_double
    };
    bb
        .UR
        .x = if bb.UR.x > (1 as libc::c_int * x) as libc::c_double {
        bb.UR.x
    } else {
        (1 as libc::c_int * x) as libc::c_double
    };
    bb
        .UR
        .y = if bb.UR.y > (1 as libc::c_int * y) as libc::c_double {
        bb.UR.y
    } else {
        (1 as libc::c_int * y) as libc::c_double
    };
    fprintf(
        fp,
        b"newpath %d %d moveto\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int * x,
        1 as libc::c_int * y,
    );
    let mut i: size_t = 1 as libc::c_int as size_t;
    while i < rte.n {
        seg = (rte.segs).offset(i as isize);
        if (*seg).isVert {
            x = vtrack(seg, m) as libc::c_int;
        } else {
            y = htrack(seg, m);
        }
        bb
            .LL
            .x = if bb.LL.x < (1 as libc::c_int * x) as libc::c_double {
            bb.LL.x
        } else {
            (1 as libc::c_int * x) as libc::c_double
        };
        bb
            .LL
            .y = if bb.LL.y < (1 as libc::c_int * y) as libc::c_double {
            bb.LL.y
        } else {
            (1 as libc::c_int * y) as libc::c_double
        };
        bb
            .UR
            .x = if bb.UR.x > (1 as libc::c_int * x) as libc::c_double {
            bb.UR.x
        } else {
            (1 as libc::c_int * x) as libc::c_double
        };
        bb
            .UR
            .y = if bb.UR.y > (1 as libc::c_int * y) as libc::c_double {
            bb.UR.y
        } else {
            (1 as libc::c_int * y) as libc::c_double
        };
        fprintf(
            fp,
            b"%d %d lineto\n\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int * x,
            1 as libc::c_int * y,
        );
        i = i.wrapping_add(1);
    }
    n = (*((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .alg as *mut cell))
        .bb;
    if (*seg).isVert {
        x = vtrack(seg, m) as libc::c_int;
        y = ((n.UR.y + n.LL.y) / 2 as libc::c_int as libc::c_double) as libc::c_int;
    } else {
        y = htrack(seg, m);
        x = ((n.LL.x + n.UR.x) / 2 as libc::c_int as libc::c_double) as libc::c_int;
    }
    bb
        .LL
        .x = if bb.LL.x < (1 as libc::c_int * x) as libc::c_double {
        bb.LL.x
    } else {
        (1 as libc::c_int * x) as libc::c_double
    };
    bb
        .LL
        .y = if bb.LL.y < (1 as libc::c_int * y) as libc::c_double {
        bb.LL.y
    } else {
        (1 as libc::c_int * y) as libc::c_double
    };
    bb
        .UR
        .x = if bb.UR.x > (1 as libc::c_int * x) as libc::c_double {
        bb.UR.x
    } else {
        (1 as libc::c_int * x) as libc::c_double
    };
    bb
        .UR
        .y = if bb.UR.y > (1 as libc::c_int * y) as libc::c_double {
        bb.UR.y
    } else {
        (1 as libc::c_int * y) as libc::c_double
    };
    fprintf(
        fp,
        b"%d %d lineto stroke\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int * x,
        1 as libc::c_int * y,
    );
    return bb;
}
unsafe extern "C" fn emitSearchGraph(mut fp: *mut FILE, mut sg: *mut sgraph) {
    let mut cp: *mut cell = 0 as *mut cell;
    let mut np: *mut snode = 0 as *mut snode;
    let mut ep: *mut sedge = 0 as *mut sedge;
    let mut p: point = point { x: 0, y: 0 };
    let mut i: libc::c_int = 0;
    fputs(b"graph G {\n\0" as *const u8 as *const libc::c_char, fp);
    fputs(b" node[shape=point]\n\0" as *const u8 as *const libc::c_char, fp);
    fputs(b" layout=neato\n\0" as *const u8 as *const libc::c_char, fp);
    i = 0 as libc::c_int;
    while i < (*sg).nnodes {
        np = ((*sg).nodes).offset(i as isize);
        cp = (*np).cells[0 as libc::c_int as usize];
        if cp == (*np).cells[1 as libc::c_int as usize] {
            let mut pf: pointf = midPt(cp);
            p.x = pf.x as libc::c_int;
            p.y = pf.y as libc::c_int;
        } else {
            if (*cp).flags & 1 as libc::c_int != 0 {
                cp = (*np).cells[1 as libc::c_int as usize];
            }
            p = coordOf(cp, np);
        }
        fprintf(
            fp,
            b"  %d [pos=\"%d,%d!\"]\n\0" as *const u8 as *const libc::c_char,
            i,
            p.x,
            p.y,
        );
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*sg).nedges {
        ep = ((*sg).edges).offset(i as isize);
        fprintf(
            fp,
            b"  %d -- %d[label=\"%f\"]\n\0" as *const u8 as *const libc::c_char,
            (*ep).v1,
            (*ep).v2,
            (*ep).weight,
        );
        i += 1;
    }
    fputs(b"}\n\0" as *const u8 as *const libc::c_char, fp);
}
unsafe extern "C" fn emitGraph(
    mut fp: *mut FILE,
    mut mp: *mut maze,
    mut n_edges: size_t,
    mut route_list: *mut route,
    mut es: *mut epair_t,
) {
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut absbb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut bbox: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    absbb.LL.y = 1.7976931348623157e+308f64;
    absbb.LL.x = absbb.LL.y;
    absbb.UR.y = -1.7976931348623157e+308f64;
    absbb.UR.x = absbb.UR.y;
    fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, prolog2);
    fprintf(
        fp,
        b"%d %d translate\n\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
        10 as libc::c_int,
    );
    fputs(b"0 0 1 setrgbcolor\n\0" as *const u8 as *const libc::c_char, fp);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*mp).ngcells {
        bb = (*((*mp).gcells).offset(i as isize)).bb;
        fprintf(
            fp,
            b"%f %f %f %f node\n\0" as *const u8 as *const libc::c_char,
            bb.LL.x,
            bb.LL.y,
            bb.UR.x,
            bb.UR.y,
        );
        i += 1;
    }
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    while i_0 < n_edges {
        absbb = emitEdge(
            fp,
            (*es.offset(i_0 as isize)).e,
            *route_list.offset(i_0 as isize),
            mp,
            absbb,
        );
        i_0 = i_0.wrapping_add(1);
    }
    fputs(b"0.8 0.8 0.8 setrgbcolor\n\0" as *const u8 as *const libc::c_char, fp);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < (*mp).ncells {
        bb = (*((*mp).cells).offset(i_1 as isize)).bb;
        fprintf(
            fp,
            b"%f %f %f %f cell\n\0" as *const u8 as *const libc::c_char,
            bb.LL.x,
            bb.LL.y,
            bb.UR.x,
            bb.UR.y,
        );
        absbb.LL.x = if absbb.LL.x < bb.LL.x { absbb.LL.x } else { bb.LL.x };
        absbb.LL.y = if absbb.LL.y < bb.LL.y { absbb.LL.y } else { bb.LL.y };
        absbb.UR.x = if absbb.UR.x > bb.UR.x { absbb.UR.x } else { bb.UR.x };
        absbb.UR.y = if absbb.UR.y > bb.UR.y { absbb.UR.y } else { bb.UR.y };
        i_1 += 1;
    }
    bbox.LL.x = (absbb.LL.x + 10 as libc::c_int as libc::c_double) as libc::c_int;
    bbox.LL.y = (absbb.LL.y + 10 as libc::c_int as libc::c_double) as libc::c_int;
    bbox.UR.x = (absbb.UR.x + 10 as libc::c_int as libc::c_double) as libc::c_int;
    bbox.UR.y = (absbb.UR.y + 10 as libc::c_int as libc::c_double) as libc::c_int;
    fprintf(fp, epilog2, bbox.LL.x, bbox.LL.y, bbox.UR.x, bbox.UR.y);
}
