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
    pub type vconfig_s;
    pub type htmllabel_t;
    pub type router_s;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn drand48() -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn Plegal_arrangement(polys: *mut *mut Ppoly_t, n_polys: libc::c_int) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Nop: libc::c_int;
    static mut Concentrate: libc::c_uchar;
    static mut State: libc::c_int;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn updateBB(g: *mut graph_t, lp: *mut textlabel_t);
    fn compute_bb(_: *mut Agraph_t);
    fn polyBB(poly: *mut polygon_t) -> boxf;
    fn makeStraightEdge(
        g: *mut graph_t,
        e: *mut edge_t,
        edgetype: libc::c_int,
        info: *mut splineInfo,
    );
    fn clip_and_install(
        fe: *mut edge_t,
        hn: *mut node_t,
        ps: *mut pointf,
        pn: libc::c_int,
        info: *mut splineInfo,
    );
    fn makeSelfEdge(
        edges: *mut *mut edge_t,
        ind: libc::c_int,
        cnt: libc::c_int,
        sizex: libc::c_double,
        sizey: libc::c_double,
        sinfo_0: *mut splineInfo,
    );
    fn makePortLabels(e: *mut edge_t);
    fn addEdgeLabels(e: *mut edge_t);
    fn resolvePorts(e: *mut edge_t);
    fn shapeOf(_: *mut node_t) -> shape_kind;
    fn Proutespline(
        barriers: *mut Pedge_t,
        n_barriers: libc::c_int,
        input_route: Ppolyline_t,
        endpoint_slopes: *mut Pvector_t,
        output_route: *mut Ppolyline_t,
    ) -> libc::c_int;
    fn make_polyline(line: Ppolyline_t, sline: *mut Ppolyline_t);
    fn esepFactor(G: *mut graph_t) -> expand_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn Pobsopen(obstacles: *mut *mut Ppoly_t, n_obstacles: libc::c_int) -> *mut vconfig_t;
    fn Pobsclose(config: *mut vconfig_t);
    fn Pobspath(
        config: *mut vconfig_t,
        p0: Ppoint_t,
        poly0: libc::c_int,
        p1: Ppoint_t,
        poly1: libc::c_int,
        output_route: *mut Ppolyline_t,
    ) -> libc::c_int;
    fn freeRouter(rtr: *mut router_t);
    fn mkRouter(obs: *mut *mut Ppoly_t, npoly: libc::c_int) -> *mut router_t;
    fn makeMultiSpline(e: *mut edge_t, rtr: *mut router_t, _: libc::c_int) -> libc::c_int;
    fn in_poly(argpoly: Ppoly_t, q: Ppoint_t) -> libc::c_int;
    fn orthoEdges(g: *mut Agraph_t, useLbls: libc::c_int);
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
pub type Pvector_t = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ppoly_t {
    pub ps: *mut Ppoint_t,
    pub pn: libc::c_int,
}
pub type Ppolyline_t = Ppoly_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pedge_t {
    pub a: Ppoint_t,
    pub b: Ppoint_t,
}
pub type vconfig_t = vconfig_s;
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
pub struct splineInfo {
    pub swapEnds: Option<unsafe extern "C" fn(*mut edge_t) -> bool>,
    pub splineMerge: Option<unsafe extern "C" fn(*mut node_t) -> bool>,
    pub ignoreSwap: bool,
    pub isOrtho: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path {
    pub start: port,
    pub end: port,
    pub nbox: libc::c_int,
    pub boxes: *mut boxf,
    pub data: *mut libc::c_void,
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
pub struct expand_t {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub doAdd: bool,
}
pub type router_t = router_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edgeitem {
    pub link: Dtlink_t,
    pub id: edgeinfo,
    pub e: *mut edge_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edgeinfo {
    pub n1: *mut node_t,
    pub p1: pointf,
    pub n2: *mut node_t,
    pub p2: pointf,
}
#[inline]
unsafe extern "C" fn add_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x + q.x;
    r.y = p.y + q.y;
    return r;
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
            ignoreSwap: false,
            isOrtho: false,
        };
        init
    }
};
unsafe extern "C" fn make_barriers(
    mut poly: *mut *mut Ppoly_t,
    mut npoly: libc::c_int,
    mut pp: libc::c_int,
    mut qp: libc::c_int,
    mut barriers: *mut *mut Pedge_t,
    mut n_barriers: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut bar: *mut Pedge_t = 0 as *mut Pedge_t;
    n = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < npoly {
        if !(i == pp) {
            if !(i == qp) {
                n += (**poly.offset(i as isize)).pn;
            }
        }
        i += 1;
    }
    bar = gcalloc(
        n as size_t,
        ::std::mem::size_of::<Pedge_t>() as libc::c_ulong,
    ) as *mut Pedge_t;
    b = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < npoly {
        if !(i == pp) {
            if !(i == qp) {
                j = 0 as libc::c_int;
                while j < (**poly.offset(i as isize)).pn {
                    k = j + 1 as libc::c_int;
                    if k >= (**poly.offset(i as isize)).pn {
                        k = 0 as libc::c_int;
                    }
                    (*bar.offset(b as isize)).a =
                        *((**poly.offset(i as isize)).ps).offset(j as isize);
                    (*bar.offset(b as isize)).b =
                        *((**poly.offset(i as isize)).ps).offset(k as isize);
                    b += 1;
                    j += 1;
                }
            }
        }
        i += 1;
    }
    if b == n {
    } else {
        __assert_fail(
            b"b == n\0" as *const u8 as *const libc::c_char,
            b"neatosplines.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void make_barriers(Ppoly_t **, int, int, int, Pedge_t **, int *)\0",
            ))
            .as_ptr(),
        );
    }
    *barriers = bar;
    *n_barriers = n;
}
unsafe extern "C" fn genPt(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut c: pointf,
) -> Ppoint_t {
    let mut p: Ppoint_t = pointf { x: 0., y: 0. };
    p.x = x + c.x;
    p.y = y + c.y;
    return p;
}
unsafe extern "C" fn recPt(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut c: pointf,
    mut m: *mut expand_t,
) -> Ppoint_t {
    let mut p: Ppoint_t = pointf { x: 0., y: 0. };
    p.x = x * (*m).x as libc::c_double + c.x;
    p.y = y * (*m).y as libc::c_double + c.y;
    return p;
}
unsafe extern "C" fn newitem(
    mut d: *mut Dt_t,
    mut obj: *mut edgeitem,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut newp: *mut edgeitem = 0 as *mut edgeitem;
    newp = zmalloc(::std::mem::size_of::<edgeitem>() as libc::c_ulong) as *mut edgeitem;
    (*newp).id = (*obj).id;
    let ref mut fresh0 = (*newp).e;
    *fresh0 = (*obj).e;
    (*((*((*newp).e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count =
        1 as libc::c_int as libc::c_short;
    return newp as *mut libc::c_void;
}
unsafe extern "C" fn freeitem(mut d: *mut Dt_t, mut obj: *mut edgeitem, mut disc: *mut Dtdisc_t) {
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn cmpitems(
    mut d: *mut Dt_t,
    mut key1: *mut edgeinfo,
    mut key2: *mut edgeinfo,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if (*key1).n1 > (*key2).n1 {
        return 1 as libc::c_int;
    }
    if (*key1).n1 < (*key2).n1 {
        return -(1 as libc::c_int);
    }
    if (*key1).n2 > (*key2).n2 {
        return 1 as libc::c_int;
    }
    if (*key1).n2 < (*key2).n2 {
        return -(1 as libc::c_int);
    }
    if (*key1).p1.x > (*key2).p1.x {
        return 1 as libc::c_int;
    }
    if (*key1).p1.x < (*key2).p1.x {
        return -(1 as libc::c_int);
    }
    if (*key1).p1.y > (*key2).p1.y {
        return 1 as libc::c_int;
    }
    if (*key1).p1.y < (*key2).p1.y {
        return -(1 as libc::c_int);
    }
    if (*key1).p2.x > (*key2).p2.x {
        return 1 as libc::c_int;
    }
    if (*key1).p2.x < (*key2).p2.x {
        return -(1 as libc::c_int);
    }
    if (*key1).p2.y > (*key2).p2.y {
        return 1 as libc::c_int;
    }
    if (*key1).p2.y < (*key2).p2.y {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut edgeItemDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<edgeinfo>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut edgeitem,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
                >,
                Dtmake_f,
            >(Some(
                newitem
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut edgeitem,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
            )),
            freef: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Dt_t, *mut edgeitem, *mut Dtdisc_t) -> ()>,
                Dtfree_f,
            >(Some(
                freeitem as unsafe extern "C" fn(*mut Dt_t, *mut edgeitem, *mut Dtdisc_t) -> (),
            )),
            comparf: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut edgeinfo,
                        *mut edgeinfo,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(Some(
                cmpitems
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut edgeinfo,
                        *mut edgeinfo,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
            )),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn equivEdge(mut map: *mut Dt_t, mut e: *mut edge_t) -> *mut edge_t {
    let mut test: edgeinfo = edgeinfo {
        n1: 0 as *mut node_t,
        p1: pointf { x: 0., y: 0. },
        n2: 0 as *mut node_t,
        p2: pointf { x: 0., y: 0. },
    };
    let mut dummy: edgeitem = edgeitem {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed_1 { _hash: 0 },
        },
        id: edgeinfo {
            n1: 0 as *mut node_t,
            p1: pointf { x: 0., y: 0. },
            n2: 0 as *mut node_t,
            p2: pointf { x: 0., y: 0. },
        },
        e: 0 as *mut edge_t,
    };
    let mut ip: *mut edgeitem = 0 as *mut edgeitem;
    if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    }))
    .node
        < (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
        .node
    {
        test.n1 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node;
        test.p1 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .p;
        test.n2 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node;
        test.p2 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .p;
    } else if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    }))
    .node
        > (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
        .node
    {
        test.n2 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node;
        test.p2 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .p;
        test.n1 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node;
        test.p1 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .p;
    } else {
        let mut hp: pointf = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .p;
        let mut tp: pointf = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .p;
        if tp.x < hp.x {
            test.p1 = tp;
            test.p2 = hp;
        } else if tp.x > hp.x {
            test.p1 = hp;
            test.p2 = tp;
        } else if tp.y < hp.y {
            test.p1 = tp;
            test.p2 = hp;
        } else if tp.y > hp.y {
            test.p1 = hp;
            test.p2 = tp;
        } else {
            test.p2 = tp;
            test.p1 = test.p2;
        }
        test.n1 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node;
        test.n2 = test.n1;
    }
    dummy.id = test;
    dummy.e = e;
    ip = (Some(((*map).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        map,
        &mut dummy as *mut edgeitem as *mut libc::c_void,
        0o1 as libc::c_int,
    ) as *mut edgeitem;
    return (*ip).e;
}
#[no_mangle]
pub unsafe extern "C" fn makeSelfArcs(mut e: *mut edge_t, mut stepx: libc::c_int) {
    let mut cnt: libc::c_int =
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count as libc::c_int;
    if cnt == 1 as libc::c_int || Concentrate as libc::c_int != 0 {
        let mut edges1: [*mut edge_t; 1] = [0 as *mut edge_t; 1];
        edges1[0 as libc::c_int as usize] = e;
        makeSelfEdge(
            edges1.as_mut_ptr(),
            0 as libc::c_int,
            1 as libc::c_int,
            stepx as libc::c_double,
            stepx as libc::c_double,
            &mut sinfo,
        );
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
            updateBB(
                agraphof(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                    .node as *mut libc::c_void,
                ),
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label,
            );
        }
        makePortLabels(e);
    } else {
        let mut i: libc::c_int = 0;
        let mut edges: *mut *mut edge_t = gcalloc(
            cnt as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        i = 0 as libc::c_int;
        while i < cnt {
            let ref mut fresh1 = *edges.offset(i as isize);
            *fresh1 = e;
            e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
            i += 1;
        }
        makeSelfEdge(
            edges,
            0 as libc::c_int,
            cnt,
            stepx as libc::c_double,
            stepx as libc::c_double,
            &mut sinfo,
        );
        i = 0 as libc::c_int;
        while i < cnt {
            e = *edges.offset(i as isize);
            if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
                updateBB(
                    agraphof(
                        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        })
                        .node as *mut libc::c_void,
                    ),
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label,
                );
            }
            makePortLabels(e);
            i += 1;
        }
        free(edges as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn makeObstacle(
    mut n: *mut node_t,
    mut pmargin: *mut expand_t,
    mut isOrtho: bool,
) -> *mut Ppoly_t {
    let mut obs: *mut Ppoly_t = 0 as *mut Ppoly_t;
    let mut poly: *mut polygon_t = 0 as *mut polygon_t;
    let mut adj: libc::c_double = 0.0f64;
    let mut j: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut polyp: pointf = pointf { x: 0., y: 0. };
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut pt: pointf = pointf { x: 0., y: 0. };
    let mut fld: *mut field_t = 0 as *mut field_t;
    let mut isPoly: libc::c_int = 0;
    let mut verts: *mut pointf = 0 as *mut pointf;
    let mut vs: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut margin: pointf = pointf { x: 0., y: 0. };
    match shapeOf(n) as libc::c_uint {
        1 | 3 => {
            obs = zmalloc(::std::mem::size_of::<Ppoly_t>() as libc::c_ulong) as *mut Ppoly_t;
            poly =
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info as *mut polygon_t;
            if isOrtho {
                isPoly = 1 as libc::c_int;
                sides = 4 as libc::c_int;
                verts = vs.as_mut_ptr();
                margin.y = 0 as libc::c_int as libc::c_double;
                margin.x = margin.y;
                if (*poly).option & (1 as libc::c_int) << 11 as libc::c_int != 0 {
                    b = polyBB(poly);
                    vs[0 as libc::c_int as usize] = b.LL;
                    vs[1 as libc::c_int as usize].x = b.UR.x;
                    vs[1 as libc::c_int as usize].y = b.LL.y;
                    vs[2 as libc::c_int as usize] = b.UR;
                    vs[3 as libc::c_int as usize].x = b.LL.x;
                    vs[3 as libc::c_int as usize].y = b.UR.y;
                } else {
                    p.x = -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                    p.y = -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
                    vs[0 as libc::c_int as usize] = p;
                    p.x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                    vs[1 as libc::c_int as usize] = p;
                    p.y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
                    vs[2 as libc::c_int as usize] = p;
                    p.x = -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                    vs[3 as libc::c_int as usize] = p;
                }
            } else if (*poly).sides >= 3 as libc::c_int {
                isPoly = 1 as libc::c_int;
                sides = (*poly).sides;
                verts = (*poly).vertices;
                margin.x = (*pmargin).x as libc::c_double;
                margin.y = (*pmargin).y as libc::c_double;
            } else {
                isPoly = 0 as libc::c_int;
                sides = 8 as libc::c_int;
                adj = drand48() * 0.01f64;
            }
            (*obs).pn = sides;
            let ref mut fresh2 = (*obs).ps;
            *fresh2 = gcalloc(
                sides as size_t,
                ::std::mem::size_of::<Ppoint_t>() as libc::c_ulong,
            ) as *mut Ppoint_t;
            j = 0 as libc::c_int;
            while j < sides {
                let mut xmargin: libc::c_double = 0.0f64;
                let mut ymargin: libc::c_double = 0.0f64;
                if isPoly != 0 {
                    if (*pmargin).doAdd {
                        if sides == 4 as libc::c_int {
                            match j {
                                0 => {
                                    xmargin = margin.x;
                                    ymargin = margin.y;
                                }
                                1 => {
                                    xmargin = -margin.x;
                                    ymargin = margin.y;
                                }
                                2 => {
                                    xmargin = -margin.x;
                                    ymargin = -margin.y;
                                }
                                3 => {
                                    xmargin = margin.x;
                                    ymargin = -margin.y;
                                }
                                _ => {
                                    fprintf(
                                        stderr,
                                        b"%s:%d: claimed unreachable code was reached\0"
                                            as *const u8
                                            as *const libc::c_char,
                                        b"neatosplines.c\0" as *const u8 as *const libc::c_char,
                                        363 as libc::c_int,
                                    );
                                    abort();
                                }
                            }
                            polyp.x = (*verts.offset(j as isize)).x + xmargin;
                            polyp.y = (*verts.offset(j as isize)).y + ymargin;
                        } else {
                            let mut h: libc::c_double = sqrt(
                                (*verts.offset(j as isize)).x * (*verts.offset(j as isize)).x
                                    + (*verts.offset(j as isize)).y * (*verts.offset(j as isize)).y,
                            );
                            polyp.x = (*verts.offset(j as isize)).x * (1.0f64 + margin.x / h);
                            polyp.y = (*verts.offset(j as isize)).y * (1.0f64 + margin.y / h);
                        }
                    } else {
                        polyp.x = (*verts.offset(j as isize)).x * margin.x;
                        polyp.y = (*verts.offset(j as isize)).y * margin.y;
                    }
                } else {
                    let mut c: libc::c_double = 0.;
                    let mut s: libc::c_double = 0.;
                    c = cos(2.0f64 * 3.14159265358979323846f64 * j as libc::c_double
                        / sides as libc::c_double
                        + adj);
                    s = sin(2.0f64 * 3.14159265358979323846f64 * j as libc::c_double
                        / sides as libc::c_double
                        + adj);
                    if (*pmargin).doAdd {
                        polyp.x = c
                            * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                                + (*pmargin).x as libc::c_double)
                            / 2.0f64;
                        polyp.y = s
                            * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                                + (*pmargin).y as libc::c_double)
                            / 2.0f64;
                    } else {
                        polyp.x = (*pmargin).x as libc::c_double
                            * c
                            * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
                            / 2.0f64;
                        polyp.y = (*pmargin).y as libc::c_double
                            * s
                            * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                            / 2.0f64;
                    }
                }
                (*((*obs).ps).offset((sides - j - 1 as libc::c_int) as isize)).x = polyp.x
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord
                        .x;
                (*((*obs).ps).offset((sides - j - 1 as libc::c_int) as isize)).y = polyp.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord
                        .y;
                j += 1;
            }
        }
        2 => {
            fld = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info as *mut field_t;
            b = (*fld).b;
            obs = zmalloc(::std::mem::size_of::<Ppoly_t>() as libc::c_ulong) as *mut Ppoly_t;
            (*obs).pn = 4 as libc::c_int;
            let ref mut fresh3 = (*obs).ps;
            *fresh3 = gcalloc(
                4 as libc::c_int as size_t,
                ::std::mem::size_of::<Ppoint_t>() as libc::c_ulong,
            ) as *mut Ppoint_t;
            pt = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
            if (*pmargin).doAdd {
                *((*obs).ps).offset(0 as libc::c_int as isize) = genPt(
                    b.LL.x - (*pmargin).x as libc::c_double,
                    b.LL.y - (*pmargin).y as libc::c_double,
                    pt,
                );
                *((*obs).ps).offset(1 as libc::c_int as isize) = genPt(
                    b.LL.x - (*pmargin).x as libc::c_double,
                    b.UR.y + (*pmargin).y as libc::c_double,
                    pt,
                );
                *((*obs).ps).offset(2 as libc::c_int as isize) = genPt(
                    b.UR.x + (*pmargin).x as libc::c_double,
                    b.UR.y + (*pmargin).y as libc::c_double,
                    pt,
                );
                *((*obs).ps).offset(3 as libc::c_int as isize) = genPt(
                    b.UR.x + (*pmargin).x as libc::c_double,
                    b.LL.y - (*pmargin).y as libc::c_double,
                    pt,
                );
            } else {
                *((*obs).ps).offset(0 as libc::c_int as isize) = recPt(b.LL.x, b.LL.y, pt, pmargin);
                *((*obs).ps).offset(1 as libc::c_int as isize) = recPt(b.LL.x, b.UR.y, pt, pmargin);
                *((*obs).ps).offset(2 as libc::c_int as isize) = recPt(b.UR.x, b.UR.y, pt, pmargin);
                *((*obs).ps).offset(3 as libc::c_int as isize) = recPt(b.UR.x, b.LL.y, pt, pmargin);
            }
        }
        4 => {
            obs = zmalloc(::std::mem::size_of::<Ppoly_t>() as libc::c_ulong) as *mut Ppoly_t;
            (*obs).pn = 4 as libc::c_int;
            let ref mut fresh4 = (*obs).ps;
            *fresh4 = gcalloc(
                4 as libc::c_int as size_t,
                ::std::mem::size_of::<Ppoint_t>() as libc::c_ulong,
            ) as *mut Ppoint_t;
            pt = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
            if (*pmargin).doAdd {
                *((*obs).ps).offset(0 as libc::c_int as isize) = genPt(
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                        - (*pmargin).x as libc::c_double,
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        - (*pmargin).y as libc::c_double,
                    pt,
                );
                *((*obs).ps).offset(1 as libc::c_int as isize) = genPt(
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                        - (*pmargin).x as libc::c_double,
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        + (*pmargin).y as libc::c_double,
                    pt,
                );
                *((*obs).ps).offset(2 as libc::c_int as isize) = genPt(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                        + (*pmargin).x as libc::c_double,
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        + (*pmargin).y as libc::c_double,
                    pt,
                );
                *((*obs).ps).offset(3 as libc::c_int as isize) = genPt(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                        + (*pmargin).x as libc::c_double,
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        - (*pmargin).y as libc::c_double,
                    pt,
                );
            } else {
                *((*obs).ps).offset(0 as libc::c_int as isize) = recPt(
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw,
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht,
                    pt,
                    pmargin,
                );
                *((*obs).ps).offset(1 as libc::c_int as isize) = recPt(
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw,
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht,
                    pt,
                    pmargin,
                );
                *((*obs).ps).offset(2 as libc::c_int as isize) = recPt(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw,
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht,
                    pt,
                    pmargin,
                );
                *((*obs).ps).offset(3 as libc::c_int as isize) = recPt(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw,
                    -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht,
                    pt,
                    pmargin,
                );
            }
        }
        _ => {
            obs = 0 as *mut Ppoly_t;
        }
    }
    return obs;
}
#[no_mangle]
pub unsafe extern "C" fn getPath(
    mut e: *mut edge_t,
    mut vconfig: *mut vconfig_t,
    mut chkPts: libc::c_int,
    mut obs: *mut *mut Ppoly_t,
    mut npoly: libc::c_int,
) -> Ppolyline_t {
    let mut line: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut pp: libc::c_int = 0;
    let mut qp: libc::c_int = 0;
    let mut p: Ppoint_t = pointf { x: 0., y: 0. };
    let mut q: Ppoint_t = pointf { x: 0., y: 0. };
    p = add_pointf(
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .p,
    );
    q = add_pointf(
        (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .p,
    );
    qp = -(1111 as libc::c_int);
    pp = qp;
    if chkPts != 0 {
        pp = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .lim;
        qp = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .lim;
    }
    Pobspath(vconfig, p, pp, q, qp, &mut line);
    return line;
}
unsafe extern "C" fn makePolyline(mut e: *mut edge_t) {
    let mut spl: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut line: Ppolyline_t = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).path;
    make_polyline(line, &mut spl);
    if Verbose as libc::c_int > 1 as libc::c_int {
        fprintf(
            stderr,
            b"polyline %s %s\n\0" as *const u8 as *const libc::c_char,
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
    }
    clip_and_install(
        e,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node,
        spl.ps,
        spl.pn,
        &mut sinfo,
    );
    addEdgeLabels(e);
}
#[no_mangle]
pub unsafe extern "C" fn makeSpline(
    mut e: *mut edge_t,
    mut obs: *mut *mut Ppoly_t,
    mut npoly: libc::c_int,
    mut chkPts: bool,
) {
    let mut line: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut spline: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut slopes: [Pvector_t; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut i: libc::c_int = 0;
    let mut n_barriers: libc::c_int = 0;
    let mut pp: libc::c_int = 0;
    let mut qp: libc::c_int = 0;
    let mut p: Ppoint_t = pointf { x: 0., y: 0. };
    let mut q: Ppoint_t = pointf { x: 0., y: 0. };
    let mut barriers: *mut Pedge_t = 0 as *mut Pedge_t;
    line = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).path;
    p = *(line.ps).offset(0 as libc::c_int as isize);
    q = *(line.ps).offset((line.pn - 1 as libc::c_int) as isize);
    qp = -(1111 as libc::c_int);
    pp = qp;
    if chkPts {
        i = 0 as libc::c_int;
        while i < npoly {
            if pp == -(1111 as libc::c_int) && in_poly(**obs.offset(i as isize), p) != 0 {
                pp = i;
            }
            if qp == -(1111 as libc::c_int) && in_poly(**obs.offset(i as isize), q) != 0 {
                qp = i;
            }
            i += 1;
        }
    }
    make_barriers(obs, npoly, pp, qp, &mut barriers, &mut n_barriers);
    slopes[0 as libc::c_int as usize].y = 0.0f64;
    slopes[0 as libc::c_int as usize].x = slopes[0 as libc::c_int as usize].y;
    slopes[1 as libc::c_int as usize].y = 0.0f64;
    slopes[1 as libc::c_int as usize].x = slopes[1 as libc::c_int as usize].y;
    if Proutespline(barriers, n_barriers, line, slopes.as_mut_ptr(), &mut spline) < 0 as libc::c_int
    {
        agerr(
            AGERR,
            b"makeSpline: failed to make spline edge (%s,%s)\n\0" as *const u8
                as *const libc::c_char,
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
        return;
    }
    if Verbose as libc::c_int > 1 as libc::c_int {
        fprintf(
            stderr,
            b"spline %s %s\n\0" as *const u8 as *const libc::c_char,
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
    }
    clip_and_install(
        e,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node,
        spline.ps,
        spline.pn,
        &mut sinfo,
    );
    free(barriers as *mut libc::c_void);
    addEdgeLabels(e);
}
unsafe extern "C" fn _spline_edges(
    mut g: *mut graph_t,
    mut pmargin: *mut expand_t,
    mut edgetype: libc::c_int,
) -> libc::c_int {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut obs: *mut *mut Ppoly_t = 0 as *mut *mut Ppoly_t;
    let mut obp: *mut Ppoly_t = 0 as *mut Ppoly_t;
    let mut cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut npoly: libc::c_int = 0;
    let mut vconfig: *mut vconfig_t = 0 as *mut vconfig_t;
    let mut P: *mut path = 0 as *mut path;
    let mut useEdges: libc::c_int = (Nop > 1 as libc::c_int) as libc::c_int;
    let mut legal: libc::c_int = 0 as libc::c_int;
    let mut rtr: *mut router_t = 0 as *mut router_t;
    if edgetype >= (3 as libc::c_int) << 1 as libc::c_int {
        obs = gcalloc(
            agnnodes(g) as size_t,
            ::std::mem::size_of::<*mut Ppoly_t>() as libc::c_ulong,
        ) as *mut *mut Ppoly_t;
        n = agfstnode(g);
        while !n.is_null() {
            obp = makeObstacle(
                n,
                pmargin,
                edgetype == (4 as libc::c_int) << 1 as libc::c_int,
            );
            if !obp.is_null() {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim = i;
                let fresh5 = i;
                i = i + 1;
                let ref mut fresh6 = *obs.offset(fresh5 as isize);
                *fresh6 = obp;
            } else {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim = -(1111 as libc::c_int);
            }
            n = agnxtnode(g, n);
        }
    } else {
        obs = 0 as *mut *mut Ppoly_t;
    }
    npoly = i;
    if !obs.is_null() {
        legal = Plegal_arrangement(obs, npoly);
        if legal != 0 {
            if edgetype != (4 as libc::c_int) << 1 as libc::c_int {
                vconfig = Pobsopen(obs, npoly);
            }
        } else if edgetype == (4 as libc::c_int) << 1 as libc::c_int {
            agerr(
                AGWARN,
                b"the bounding boxes of some nodes touch - falling back to straight line edges\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            agerr(
                AGWARN,
                b"some nodes with margin (%.02f,%.02f) touch - falling back to straight line edges\n\0"
                    as *const u8 as *const libc::c_char,
                (*pmargin).x as libc::c_double,
                (*pmargin).y as libc::c_double,
            );
        }
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Creating edges using %s\n\0" as *const u8 as *const libc::c_char,
            if legal != 0 && edgetype == (4 as libc::c_int) << 1 as libc::c_int {
                b"orthogonal lines\0" as *const u8 as *const libc::c_char
            } else if !vconfig.is_null() {
                if edgetype == (5 as libc::c_int) << 1 as libc::c_int {
                    b"splines\0" as *const u8 as *const libc::c_char
                } else {
                    b"polylines\0" as *const u8 as *const libc::c_char
                }
            } else {
                b"line segments\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !vconfig.is_null() {
        n = agfstnode(g);
        while !n.is_null() {
            e = agfstout(g, n);
            while !e.is_null() {
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).path = getPath(
                    e,
                    vconfig,
                    (0 as libc::c_int == 0) as libc::c_int,
                    obs,
                    npoly,
                );
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
    } else if legal != 0 && edgetype == (4 as libc::c_int) << 1 as libc::c_int {
        orthoEdges(g, 0 as libc::c_int);
        useEdges = 1 as libc::c_int;
    }
    n = agfstnode(g);
    while !n.is_null() {
        let mut current_block_70: u64;
        e = agfstout(g, n);
        while !e.is_null() {
            let mut head: *mut node_t =
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                .node;
            if useEdges != 0
                && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
            {
                add_pointf(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .tail_port
                        .p,
                );
                add_pointf(
                    (*((*(head as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .head_port
                        .p,
                );
                addEdgeLabels(e);
            } else if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count as libc::c_int
                == 0 as libc::c_int)
            {
                if n == head {
                    if P.is_null() {
                        P = zmalloc(::std::mem::size_of::<path>() as libc::c_ulong) as *mut path;
                        let ref mut fresh7 = (*P).boxes;
                        *fresh7 = gcalloc(
                            (agnnodes(g) + 20 as libc::c_int * 2 as libc::c_int * 9 as libc::c_int)
                                as size_t,
                            ::std::mem::size_of::<boxf>() as libc::c_ulong,
                        ) as *mut boxf;
                    }
                    makeSelfArcs(
                        e,
                        (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep,
                    );
                } else if !vconfig.is_null() {
                    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count as libc::c_int
                        > 1 as libc::c_int
                        || ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .tail_port
                            .side as libc::c_int
                            != 0
                            || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .head_port
                                .side as libc::c_int
                                != 0)
                    {
                        let mut fail: libc::c_int = 0 as libc::c_int;
                        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .path
                            .pn
                            == 2 as libc::c_int
                            && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .tail_port
                                .side as libc::c_int
                                != 0
                                || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                    .head_port
                                    .side as libc::c_int
                                    != 0)
                        {
                            makeStraightEdge(g, e, edgetype, &mut sinfo);
                        } else {
                            if rtr.is_null() {
                                rtr = mkRouter(obs, npoly);
                            }
                            fail = makeMultiSpline(
                                e,
                                rtr,
                                (edgetype == (3 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
                            );
                        }
                        if fail == 0 {
                            current_block_70 = 10399321362245223758;
                        } else {
                            current_block_70 = 10853015579903106591;
                        }
                    } else {
                        current_block_70 = 10853015579903106591;
                    }
                    match current_block_70 {
                        10399321362245223758 => {}
                        _ => {
                            cnt = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count
                                as libc::c_int;
                            if Concentrate != 0 {
                                cnt = 1 as libc::c_int;
                            }
                            e0 = e;
                            i = 0 as libc::c_int;
                            while i < cnt {
                                if edgetype == (5 as libc::c_int) << 1 as libc::c_int {
                                    makeSpline(e0, obs, npoly, 1 as libc::c_int != 0);
                                } else {
                                    makePolyline(e0);
                                }
                                e0 = (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
                                i += 1;
                            }
                        }
                    }
                } else {
                    makeStraightEdge(g, e, edgetype, &mut sinfo);
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    if !rtr.is_null() {
        freeRouter(rtr);
    }
    if !vconfig.is_null() {
        Pobsclose(vconfig);
    }
    if !P.is_null() {
        free((*P).boxes as *mut libc::c_void);
        free(P as *mut libc::c_void);
    }
    if !obs.is_null() {
        i = 0 as libc::c_int;
        while i < npoly {
            free((**obs.offset(i as isize)).ps as *mut libc::c_void);
            free(*obs.offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        free(obs as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn splineEdges(
    mut g: *mut graph_t,
    mut edgefn: Option<
        unsafe extern "C" fn(*mut graph_t, *mut expand_t, libc::c_int) -> libc::c_int,
    >,
    mut edgetype: libc::c_int,
) -> libc::c_int {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut margin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    let mut map: *mut Dt_t = 0 as *mut Dt_t;
    margin = esepFactor(g);
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            resolvePorts(e);
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    map = dtopen(&mut edgeItemDisc, Dtoset);
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if Nop > 1 as libc::c_int
                && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
            {
                let ref mut fresh8 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count;
                *fresh8 += 1;
            } else {
                let mut leader: *mut edge_t = equivEdge(map, e);
                if leader != e {
                    let ref mut fresh9 =
                        (*((*(leader as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count;
                    *fresh9 += 1;
                    let ref mut fresh10 =
                        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
                    *fresh10 = (*((*(leader as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
                    let ref mut fresh11 =
                        (*((*(leader as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
                    *fresh11 = e;
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    dtclose(map);
    if edgefn.expect("non-null function pointer")(g, &mut margin, edgetype) != 0 {
        return 1 as libc::c_int;
    }
    State = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spline_edges1(
    mut g: *mut graph_t,
    mut edgetype: libc::c_int,
) -> libc::c_int {
    return splineEdges(
        g,
        Some(
            _spline_edges
                as unsafe extern "C" fn(*mut graph_t, *mut expand_t, libc::c_int) -> libc::c_int,
        ),
        edgetype,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spline_edges0(mut g: *mut graph_t, mut set_aspect: bool) {
    let mut et: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags
        as libc::c_int
        & (7 as libc::c_int) << 1 as libc::c_int;
    if set_aspect {
        neato_set_aspect(g);
    }
    if et == (0 as libc::c_int) << 1 as libc::c_int {
        return;
    }
    spline_edges1(g, et);
}
unsafe extern "C" fn shiftClusters(mut g: *mut graph_t, mut offset: pointf) {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        shiftClusters(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(i as isize),
            offset,
        );
        i += 1;
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .x -= offset.x;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .y -= offset.y;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .x -= offset.x;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .y -= offset.y;
}
#[no_mangle]
pub unsafe extern "C" fn spline_edges(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut offset: pointf = pointf { x: 0., y: 0. };
    compute_bb(g);
    offset.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .x
        / 72 as libc::c_int as libc::c_double;
    offset.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .y
        / 72 as libc::c_int as libc::c_double;
    n = agfstnode(g);
    while !n.is_null() {
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) -= offset.x;
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) -= offset.y;
        n = agnxtnode(g, n);
    }
    shiftClusters(
        g,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL,
    );
    spline_edges0(g, 1 as libc::c_int != 0);
}
unsafe extern "C" fn scaleEdge(mut e: *mut edge_t, mut xf: libc::c_double, mut yf: libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pt: *mut pointf = 0 as *mut pointf;
    let mut bez: *mut bezier = 0 as *mut bezier;
    let mut delh: pointf = pointf { x: 0., y: 0. };
    let mut delt: pointf = pointf { x: 0., y: 0. };
    delh.x =
        72 as libc::c_int as libc::c_double
            * (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(0 as libc::c_int as isize)
                * (xf - 1.0f64));
    delh.y =
        72 as libc::c_int as libc::c_double
            * (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(1 as libc::c_int as isize)
                * (yf - 1.0f64));
    delt.x =
        72 as libc::c_int as libc::c_double
            * (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(0 as libc::c_int as isize)
                * (xf - 1.0f64));
    delt.y =
        72 as libc::c_int as libc::c_double
            * (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(1 as libc::c_int as isize)
                * (yf - 1.0f64));
    bez = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list;
    i = 0 as libc::c_int;
    while i < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
        pt = (*bez).list;
        j = 0 as libc::c_int;
        while j < (*bez).size {
            if i == 0 as libc::c_int && j == 0 as libc::c_int {
                (*pt).x += delt.x;
                (*pt).y += delt.y;
            } else if i
                == (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size
                    - 1 as libc::c_int
                && j == (*bez).size - 1 as libc::c_int
            {
                (*pt).x += delh.x;
                (*pt).y += delh.y;
            } else {
                (*pt).x *= xf;
                (*pt).y *= yf;
            }
            pt = pt.offset(1);
            j += 1;
        }
        if (*bez).sflag != 0 {
            (*bez).sp.x += delt.x;
            (*bez).sp.y += delt.y;
        }
        if (*bez).eflag != 0 {
            (*bez).ep.x += delh.x;
            (*bez).ep.y += delh.y;
        }
        bez = bez.offset(1);
        i += 1;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null()
        && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).set as libc::c_int != 0
    {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
            .pos
            .x *= xf;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
            .pos
            .y *= yf;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).is_null()
        && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).set as libc::c_int
            != 0
    {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label)
            .pos
            .x += delh.x;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label)
            .pos
            .y += delh.y;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).is_null()
        && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).set as libc::c_int
            != 0
    {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label)
            .pos
            .x += delt.x;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label)
            .pos
            .y += delt.y;
    }
}
unsafe extern "C" fn scaleBB(mut g: *mut graph_t, mut xf: libc::c_double, mut yf: libc::c_double) {
    let mut i: libc::c_int = 0;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .x *= xf;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .y *= yf;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .x *= xf;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .y *= yf;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set as libc::c_int != 0
    {
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
            .pos
            .x *= xf;
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
            .pos
            .y *= yf;
    }
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        scaleBB(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(i as isize),
            xf,
            yf,
        );
        i += 1;
    }
}
unsafe extern "C" fn translateE(mut e: *mut edge_t, mut offset: pointf) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pt: *mut pointf = 0 as *mut pointf;
    let mut bez: *mut bezier = 0 as *mut bezier;
    bez = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list;
    i = 0 as libc::c_int;
    while i < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
        pt = (*bez).list;
        j = 0 as libc::c_int;
        while j < (*bez).size {
            (*pt).x -= offset.x;
            (*pt).y -= offset.y;
            pt = pt.offset(1);
            j += 1;
        }
        if (*bez).sflag != 0 {
            (*bez).sp.x -= offset.x;
            (*bez).sp.y -= offset.y;
        }
        if (*bez).eflag != 0 {
            (*bez).ep.x -= offset.x;
            (*bez).ep.y -= offset.y;
        }
        bez = bez.offset(1);
        i += 1;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null()
        && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).set as libc::c_int != 0
    {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
            .pos
            .x -= offset.x;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
            .pos
            .y -= offset.y;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).is_null()
        && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).set as libc::c_int != 0
    {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel)
            .pos
            .x -= offset.x;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel)
            .pos
            .y -= offset.y;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).is_null()
        && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).set as libc::c_int
            != 0
    {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label)
            .pos
            .x -= offset.x;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label)
            .pos
            .y -= offset.y;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).is_null()
        && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).set as libc::c_int
            != 0
    {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label)
            .pos
            .x -= offset.x;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label)
            .pos
            .y -= offset.y;
    }
}
unsafe extern "C" fn translateG(mut g: *mut Agraph_t, mut offset: pointf) {
    let mut i: libc::c_int = 0;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .x -= offset.x;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .y -= offset.y;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .x -= offset.x;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .y -= offset.y;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set as libc::c_int != 0
    {
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
            .pos
            .x -= offset.x;
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
            .pos
            .y -= offset.y;
    }
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        translateG(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(i as isize),
            offset,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn neato_translate(mut g: *mut Agraph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut offset: pointf = pointf { x: 0., y: 0. };
    let mut ll: pointf = pointf { x: 0., y: 0. };
    ll = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL;
    offset.x = ll.x / 72 as libc::c_int as libc::c_double;
    offset.y = ll.y / 72 as libc::c_int as libc::c_double;
    n = agfstnode(g);
    while !n.is_null() {
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) -= offset.x;
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) -= offset.y;
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null()
            && (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).set as libc::c_int
                != 0
        {
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel)
                .pos
                .x -= ll.x;
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel)
                .pos
                .y -= ll.y;
        }
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
                translateE(e, ll);
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    translateG(g, ll);
}
unsafe extern "C" fn _neato_set_aspect(mut g: *mut graph_t) -> bool {
    let mut xf: libc::c_double = 0.;
    let mut yf: libc::c_double = 0.;
    let mut actual: libc::c_double = 0.;
    let mut desired: libc::c_double = 0.;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut translated: bool = 0 as libc::c_int != 0;
    if (*g).root != g {
        return 0 as libc::c_int != 0;
    }
    if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio_kind as u64 != 0 {
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .bb
            .LL
            .x
            != 0.
            || (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .LL
                .y
                != 0.
        {
            translated = 1 as libc::c_int != 0;
            neato_translate(g);
        }
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int
            & 1 as libc::c_int
            != 0
        {
            let mut t: libc::c_double = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .x;
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .y;
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .y = t;
        }
        if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio_kind
            as libc::c_uint
            == R_FILL as libc::c_int as libc::c_uint
        {
            if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .size
                .x
                <= 0 as libc::c_int as libc::c_double
            {
                return translated;
            }
            xf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .size
                .x
                / (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x;
            yf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .size
                .y
                / (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .y;
            if xf < 1.0f64 || yf < 1.0f64 {
                if xf < yf {
                    yf /= xf;
                    xf = 1.0f64;
                } else {
                    xf /= yf;
                    yf = 1.0f64;
                }
            }
        } else if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio_kind
            as libc::c_uint
            == R_EXPAND as libc::c_int as libc::c_uint
        {
            if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .size
                .x
                <= 0 as libc::c_int as libc::c_double
            {
                return translated;
            }
            xf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .size
                .x
                / (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x;
            yf = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
                .size
                .y
                / (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .y;
            if xf > 1.0f64 && yf > 1.0f64 {
                let mut scale: libc::c_double = if xf < yf { xf } else { yf };
                yf = scale;
                xf = yf;
            } else {
                return translated;
            }
        } else if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio_kind
            as libc::c_uint
            == R_VALUE as libc::c_int as libc::c_uint
        {
            desired = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).ratio;
            actual = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .bb
                .UR
                .y
                / (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x;
            if actual < desired {
                yf = desired / actual;
                xf = 1.0f64;
            } else {
                xf = actual / desired;
                yf = 1.0f64;
            }
        } else {
            return translated;
        }
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int
            & 1 as libc::c_int
            != 0
        {
            let mut t_0: libc::c_double = xf;
            xf = yf;
            yf = t_0;
        }
        if Nop > 1 as libc::c_int {
            let mut e: *mut edge_t = 0 as *mut edge_t;
            n = agfstnode(g);
            while !n.is_null() {
                e = agfstout(g, n);
                while !e.is_null() {
                    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
                        scaleEdge(e, xf, yf);
                    }
                    e = agnxtout(g, e);
                }
                n = agnxtnode(g, n);
            }
        }
        n = agfstnode(g);
        while !n.is_null() {
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) *= xf;
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) *= yf;
            n = agnxtnode(g, n);
        }
        scaleBB(g, xf, yf);
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn neato_set_aspect(mut g: *mut graph_t) -> bool {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut moved: bool = 0 as libc::c_int != 0;
    moved = _neato_set_aspect(g);
    n = agfstnode(g);
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .coord
            .x = 72 as libc::c_int as libc::c_double
            * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize);
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .coord
            .y = 72 as libc::c_int as libc::c_double
            * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize);
        n = agnxtnode(g, n);
    }
    return moved;
}
