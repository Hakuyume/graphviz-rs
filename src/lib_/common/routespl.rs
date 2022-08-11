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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Concentrate: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn addEdgeLabels(e: *mut edge_t);
    fn clip_and_install(
        fe: *mut edge_t,
        hn: *mut node_t,
        ps: *mut pointf,
        pn: libc::c_int,
        info: *mut splineInfo,
    );
    fn Pshortestpath(
        boundary: *mut Ppoly_t,
        endpoints: *mut Ppoint_t,
        output_route: *mut Ppolyline_t,
    ) -> libc::c_int;
    fn Proutespline(
        barriers: *mut Pedge_t,
        n_barriers: libc::c_int,
        input_route: Ppolyline_t,
        endpoint_slopes: *mut Pvector_t,
        output_route: *mut Ppolyline_t,
    ) -> libc::c_int;
    fn make_polyline(line: Ppolyline_t, sline: *mut Ppolyline_t);
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
pub type vec = _tag_vec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tag_vec {
    pub _mem: *mut *mut libc::c_void,
    pub _elems: size_t,
    pub _capelems: size_t,
}
#[inline]
unsafe extern "C" fn add_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x + q.x;
    r.y = p.y + q.y;
    return r;
}
static mut nedges: libc::c_int = 0;
static mut nboxes: libc::c_int = 0;
static mut routeinit: libc::c_int = 0;
static mut polypoints: *mut Ppoint_t = 0 as *const Ppoint_t as *mut Ppoint_t;
static mut polypointn: libc::c_int = 0;
static mut edges: *mut Pedge_t = 0 as *const Pedge_t as *mut Pedge_t;
static mut edgen: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn simpleSplineRoute(
    mut tp: pointf,
    mut hp: pointf,
    mut poly: Ppoly_t,
    mut n_spl_pts: *mut libc::c_int,
    mut polyline: libc::c_int,
) -> *mut pointf {
    let mut pl: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut spl: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut eps: [Ppoint_t; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut evs: [Pvector_t; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut i: libc::c_int = 0;
    eps[0 as libc::c_int as usize].x = tp.x;
    eps[0 as libc::c_int as usize].y = tp.y;
    eps[1 as libc::c_int as usize].x = hp.x;
    eps[1 as libc::c_int as usize].y = hp.y;
    if Pshortestpath(&mut poly, eps.as_mut_ptr(), &mut pl) < 0 as libc::c_int {
        return 0 as *mut pointf;
    }
    if polyline != 0 {
        make_polyline(pl, &mut spl);
    } else {
        if poly.pn > edgen {
            edges = if !edges.is_null() {
                grealloc(
                    edges as *mut libc::c_void,
                    (poly.pn as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<Pedge_t>() as libc::c_ulong),
                ) as *mut Pedge_t
            } else {
                gmalloc(
                    (poly.pn as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<Pedge_t>() as libc::c_ulong),
                ) as *mut Pedge_t
            };
            edgen = poly.pn;
        }
        i = 0 as libc::c_int;
        while i < poly.pn {
            (*edges.offset(i as isize)).a = *(poly.ps).offset(i as isize);
            (*edges.offset(i as isize))
                .b = *(poly.ps).offset(((i + 1 as libc::c_int) % poly.pn) as isize);
            i += 1;
        }
        evs[0 as libc::c_int as usize].y = 0 as libc::c_int as libc::c_double;
        evs[0 as libc::c_int as usize].x = evs[0 as libc::c_int as usize].y;
        evs[1 as libc::c_int as usize].y = 0 as libc::c_int as libc::c_double;
        evs[1 as libc::c_int as usize].x = evs[1 as libc::c_int as usize].y;
        if Proutespline(edges, poly.pn, pl, evs.as_mut_ptr(), &mut spl)
            < 0 as libc::c_int
        {
            return 0 as *mut pointf;
        }
    }
    let mut ps: *mut pointf = calloc(
        spl.pn as libc::c_ulong,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    if ps.is_null() {
        agerr(AGERR, b"cannot allocate ps\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut pointf;
    }
    i = 0 as libc::c_int;
    while i < spl.pn {
        *ps.offset(i as isize) = *(spl.ps).offset(i as isize);
        i += 1;
    }
    *n_spl_pts = spl.pn;
    return ps;
}
#[no_mangle]
pub unsafe extern "C" fn routesplinesinit() -> libc::c_int {
    routeinit += 1;
    if routeinit > 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    nedges = 0 as libc::c_int;
    nboxes = 0 as libc::c_int;
    if Verbose != 0 {
        start_timer();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn routesplinesterm() {
    routeinit -= 1;
    if routeinit > 0 as libc::c_int {
        return;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"routesplines: %d edges, %d boxes %.2f sec\n\0" as *const u8
                as *const libc::c_char,
            nedges,
            nboxes,
            elapsed_sec(),
        );
    }
}
unsafe extern "C" fn limitBoxes(
    mut boxes: *mut boxf,
    mut boxn: libc::c_int,
    mut pps: *const pointf,
    mut pn: libc::c_int,
    mut delta: libc::c_int,
) {
    let mut bi: libc::c_int = 0;
    let mut si: libc::c_int = 0;
    let mut splinepi: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    let mut sp: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut num_div: libc::c_int = delta * boxn;
    splinepi = 0 as libc::c_int;
    while (splinepi + 3 as libc::c_int) < pn {
        si = 0 as libc::c_int;
        while si <= num_div {
            t = si as libc::c_double / num_div as libc::c_double;
            sp[0 as libc::c_int as usize] = *pps.offset(splinepi as isize);
            sp[1 as libc::c_int
                as usize] = *pps.offset((splinepi + 1 as libc::c_int) as isize);
            sp[2 as libc::c_int
                as usize] = *pps.offset((splinepi + 2 as libc::c_int) as isize);
            sp[3 as libc::c_int
                as usize] = *pps.offset((splinepi + 3 as libc::c_int) as isize);
            sp[0 as libc::c_int as usize].x
                += t
                    * (sp[1 as libc::c_int as usize].x
                        - sp[0 as libc::c_int as usize].x);
            sp[0 as libc::c_int as usize].y
                += t
                    * (sp[1 as libc::c_int as usize].y
                        - sp[0 as libc::c_int as usize].y);
            sp[1 as libc::c_int as usize].x
                += t
                    * (sp[2 as libc::c_int as usize].x
                        - sp[1 as libc::c_int as usize].x);
            sp[1 as libc::c_int as usize].y
                += t
                    * (sp[2 as libc::c_int as usize].y
                        - sp[1 as libc::c_int as usize].y);
            sp[2 as libc::c_int as usize].x
                += t
                    * (sp[3 as libc::c_int as usize].x
                        - sp[2 as libc::c_int as usize].x);
            sp[2 as libc::c_int as usize].y
                += t
                    * (sp[3 as libc::c_int as usize].y
                        - sp[2 as libc::c_int as usize].y);
            sp[0 as libc::c_int as usize].x
                += t
                    * (sp[1 as libc::c_int as usize].x
                        - sp[0 as libc::c_int as usize].x);
            sp[0 as libc::c_int as usize].y
                += t
                    * (sp[1 as libc::c_int as usize].y
                        - sp[0 as libc::c_int as usize].y);
            sp[1 as libc::c_int as usize].x
                += t
                    * (sp[2 as libc::c_int as usize].x
                        - sp[1 as libc::c_int as usize].x);
            sp[1 as libc::c_int as usize].y
                += t
                    * (sp[2 as libc::c_int as usize].y
                        - sp[1 as libc::c_int as usize].y);
            sp[0 as libc::c_int as usize].x
                += t
                    * (sp[1 as libc::c_int as usize].x
                        - sp[0 as libc::c_int as usize].x);
            sp[0 as libc::c_int as usize].y
                += t
                    * (sp[1 as libc::c_int as usize].y
                        - sp[0 as libc::c_int as usize].y);
            bi = 0 as libc::c_int;
            while bi < boxn {
                if sp[0 as libc::c_int as usize].y
                    <= (*boxes.offset(bi as isize)).UR.y + 0.0001f64
                    && sp[0 as libc::c_int as usize].y
                        >= (*boxes.offset(bi as isize)).LL.y - 0.0001f64
                {
                    (*boxes.offset(bi as isize))
                        .LL
                        .x = fmin(
                        (*boxes.offset(bi as isize)).LL.x,
                        sp[0 as libc::c_int as usize].x,
                    );
                    (*boxes.offset(bi as isize))
                        .UR
                        .x = fmax(
                        (*boxes.offset(bi as isize)).UR.x,
                        sp[0 as libc::c_int as usize].x,
                    );
                }
                bi += 1;
            }
            si += 1;
        }
        splinepi += 3 as libc::c_int;
    }
}
unsafe extern "C" fn _routesplines(
    mut pp: *mut path,
    mut npoints: *mut libc::c_int,
    mut polyline: libc::c_int,
) -> *mut pointf {
    let mut poly: Ppoly_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut pl: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut spl: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut splinepi: libc::c_int = 0;
    let mut eps: [Ppoint_t; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut evs: [Pvector_t; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut edgei: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut pi: libc::c_int = 0;
    let mut bi: libc::c_int = 0;
    let mut boxes: *mut boxf = 0 as *mut boxf;
    let mut boxn: libc::c_int = 0;
    let mut realedge: *mut edge_t = 0 as *mut edge_t;
    let mut flip: bool = false;
    let mut loopcnt: libc::c_int = 0;
    let mut delta: libc::c_int = 10 as libc::c_int;
    let mut unbounded: bool = false;
    *npoints = 0 as libc::c_int;
    nedges += 1;
    nboxes += (*pp).nbox;
    realedge = (*pp).data as *mut edge_t;
    while !realedge.is_null()
        && (*((*(realedge as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
            as libc::c_int != 0 as libc::c_int
    {
        realedge = (*((*(realedge as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    if realedge.is_null() {
        agerr(
            AGERR,
            b"in routesplines, cannot find NORMAL edge\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut pointf;
    }
    boxes = (*pp).boxes;
    boxn = (*pp).nbox;
    if checkpath(boxn, boxes, pp) != 0 {
        return 0 as *mut pointf;
    }
    if boxn * 8 as libc::c_int > polypointn {
        polypoints = if !polypoints.is_null() {
            grealloc(
                polypoints as *mut libc::c_void,
                ((boxn * 8 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Ppoint_t>() as libc::c_ulong),
            ) as *mut Ppoint_t
        } else {
            gmalloc(
                ((boxn * 8 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Ppoint_t>() as libc::c_ulong),
            ) as *mut Ppoint_t
        };
        polypointn = boxn * 8 as libc::c_int;
    }
    if boxn > 1 as libc::c_int
        && (*boxes.offset(0 as libc::c_int as isize)).LL.y
            > (*boxes.offset(1 as libc::c_int as isize)).LL.y
    {
        flip = 1 as libc::c_int != 0;
        bi = 0 as libc::c_int;
        while bi < boxn {
            let mut v: libc::c_double = (*boxes.offset(bi as isize)).UR.y;
            (*boxes.offset(bi as isize))
                .UR
                .y = -(1 as libc::c_int) as libc::c_double
                * (*boxes.offset(bi as isize)).LL.y;
            (*boxes.offset(bi as isize)).LL.y = -v;
            bi += 1;
        }
    } else {
        flip = 0 as libc::c_int != 0;
    }
    if (*(if ((*(realedge as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        realedge
    } else {
        realedge.offset(1 as libc::c_int as isize)
    }))
        .node
        != (*(if ((*(realedge as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            realedge
        } else {
            realedge.offset(-(1 as libc::c_int as isize))
        }))
            .node
    {
        bi = 0 as libc::c_int;
        pi = 0 as libc::c_int;
        while bi < boxn {
            prev = 0 as libc::c_int;
            next = prev;
            if bi > 0 as libc::c_int {
                prev = if (*boxes.offset(bi as isize)).LL.y
                    > (*boxes.offset((bi - 1 as libc::c_int) as isize)).LL.y
                {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
            if bi < boxn - 1 as libc::c_int {
                next = if (*boxes.offset((bi + 1 as libc::c_int) as isize)).LL.y
                    > (*boxes.offset(bi as isize)).LL.y
                {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                };
            }
            if prev != next {
                if next == -(1 as libc::c_int) || prev == 1 as libc::c_int {
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).LL.x;
                    let fresh0 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh0 as isize))
                        .y = (*boxes.offset(bi as isize)).UR.y;
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).LL.x;
                    let fresh1 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh1 as isize))
                        .y = (*boxes.offset(bi as isize)).LL.y;
                } else {
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).UR.x;
                    let fresh2 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh2 as isize))
                        .y = (*boxes.offset(bi as isize)).LL.y;
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).UR.x;
                    let fresh3 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh3 as isize))
                        .y = (*boxes.offset(bi as isize)).UR.y;
                }
            } else if prev == 0 as libc::c_int {
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).LL.x;
                let fresh4 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh4 as isize))
                    .y = (*boxes.offset(bi as isize)).UR.y;
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).LL.x;
                let fresh5 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh5 as isize))
                    .y = (*boxes.offset(bi as isize)).LL.y;
            } else if !(prev == -(1 as libc::c_int) && next == -(1 as libc::c_int)) {
                agerr(
                    AGERR,
                    b"in routesplines, illegal values of prev %d and next %d, line %d\n\0"
                        as *const u8 as *const libc::c_char,
                    prev,
                    next,
                    442 as libc::c_int,
                );
                return 0 as *mut pointf;
            }
            bi += 1;
        }
        bi = boxn - 1 as libc::c_int;
        while bi >= 0 as libc::c_int {
            prev = 0 as libc::c_int;
            next = prev;
            if bi < boxn - 1 as libc::c_int {
                prev = if (*boxes.offset(bi as isize)).LL.y
                    > (*boxes.offset((bi + 1 as libc::c_int) as isize)).LL.y
                {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
            if bi > 0 as libc::c_int {
                next = if (*boxes.offset((bi - 1 as libc::c_int) as isize)).LL.y
                    > (*boxes.offset(bi as isize)).LL.y
                {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                };
            }
            if prev != next {
                if next == -(1 as libc::c_int) || prev == 1 as libc::c_int {
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).LL.x;
                    let fresh6 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh6 as isize))
                        .y = (*boxes.offset(bi as isize)).UR.y;
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).LL.x;
                    let fresh7 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh7 as isize))
                        .y = (*boxes.offset(bi as isize)).LL.y;
                } else {
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).UR.x;
                    let fresh8 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh8 as isize))
                        .y = (*boxes.offset(bi as isize)).LL.y;
                    (*polypoints.offset(pi as isize))
                        .x = (*boxes.offset(bi as isize)).UR.x;
                    let fresh9 = pi;
                    pi = pi + 1;
                    (*polypoints.offset(fresh9 as isize))
                        .y = (*boxes.offset(bi as isize)).UR.y;
                }
            } else if prev == 0 as libc::c_int {
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).UR.x;
                let fresh10 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh10 as isize))
                    .y = (*boxes.offset(bi as isize)).LL.y;
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).UR.x;
                let fresh11 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh11 as isize))
                    .y = (*boxes.offset(bi as isize)).UR.y;
            } else {
                if !(prev == -(1 as libc::c_int) && next == -(1 as libc::c_int)) {
                    agerr(
                        AGERR,
                        b"in routesplines, illegal values of prev %d and next %d, line %d\n\0"
                            as *const u8 as *const libc::c_char,
                        prev,
                        next,
                        475 as libc::c_int,
                    );
                    return 0 as *mut pointf;
                }
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).UR.x;
                let fresh12 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh12 as isize))
                    .y = (*boxes.offset(bi as isize)).LL.y;
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).UR.x;
                let fresh13 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh13 as isize))
                    .y = (*boxes.offset(bi as isize)).UR.y;
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).LL.x;
                let fresh14 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh14 as isize))
                    .y = (*boxes.offset(bi as isize)).UR.y;
                (*polypoints.offset(pi as isize)).x = (*boxes.offset(bi as isize)).LL.x;
                let fresh15 = pi;
                pi = pi + 1;
                (*polypoints.offset(fresh15 as isize))
                    .y = (*boxes.offset(bi as isize)).LL.y;
            }
            bi -= 1;
        }
    } else {
        agerr(
            AGERR,
            b"in routesplines, edge is a loop at %s\n\0" as *const u8
                as *const libc::c_char,
            agnameof(
                (*if ((*(realedge as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    realedge
                } else {
                    realedge.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut libc::c_void,
            ),
        );
        return 0 as *mut pointf;
    }
    if flip {
        bi = 0 as libc::c_int;
        while bi < boxn {
            let mut v_0: libc::c_double = (*boxes.offset(bi as isize)).UR.y;
            (*boxes.offset(bi as isize))
                .UR
                .y = -(1 as libc::c_int) as libc::c_double
                * (*boxes.offset(bi as isize)).LL.y;
            (*boxes.offset(bi as isize)).LL.y = -v_0;
            bi += 1;
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < pi {
            (*polypoints.offset(i as isize)).y *= -(1 as libc::c_int) as libc::c_double;
            i += 1;
        }
    }
    bi = 0 as libc::c_int;
    while bi < boxn {
        (*boxes.offset(bi as isize)).LL.x = 2147483647 as libc::c_int as libc::c_double;
        (*boxes.offset(bi as isize))
            .UR
            .x = (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double;
        bi += 1;
    }
    poly.ps = polypoints;
    poly.pn = pi;
    eps[0 as libc::c_int as usize].x = (*pp).start.p.x;
    eps[0 as libc::c_int as usize].y = (*pp).start.p.y;
    eps[1 as libc::c_int as usize].x = (*pp).end.p.x;
    eps[1 as libc::c_int as usize].y = (*pp).end.p.y;
    if Pshortestpath(&mut poly, eps.as_mut_ptr(), &mut pl) < 0 as libc::c_int {
        agerr(
            AGERR,
            b"in routesplines, Pshortestpath failed\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut pointf;
    }
    if polyline != 0 {
        make_polyline(pl, &mut spl);
    } else {
        if poly.pn > edgen {
            edges = if !edges.is_null() {
                grealloc(
                    edges as *mut libc::c_void,
                    (poly.pn as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<Pedge_t>() as libc::c_ulong),
                ) as *mut Pedge_t
            } else {
                gmalloc(
                    (poly.pn as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<Pedge_t>() as libc::c_ulong),
                ) as *mut Pedge_t
            };
            edgen = poly.pn;
        }
        edgei = 0 as libc::c_int;
        while edgei < poly.pn {
            (*edges.offset(edgei as isize)).a = *polypoints.offset(edgei as isize);
            (*edges.offset(edgei as isize))
                .b = *polypoints.offset(((edgei + 1 as libc::c_int) % poly.pn) as isize);
            edgei += 1;
        }
        if (*pp).start.constrained {
            evs[0 as libc::c_int as usize].x = cos((*pp).start.theta);
            evs[0 as libc::c_int as usize].y = sin((*pp).start.theta);
        } else {
            evs[0 as libc::c_int as usize].y = 0 as libc::c_int as libc::c_double;
            evs[0 as libc::c_int as usize].x = evs[0 as libc::c_int as usize].y;
        }
        if (*pp).end.constrained {
            evs[1 as libc::c_int as usize].x = -cos((*pp).end.theta);
            evs[1 as libc::c_int as usize].y = -sin((*pp).end.theta);
        } else {
            evs[1 as libc::c_int as usize].y = 0 as libc::c_int as libc::c_double;
            evs[1 as libc::c_int as usize].x = evs[1 as libc::c_int as usize].y;
        }
        if Proutespline(edges, poly.pn, pl, evs.as_mut_ptr(), &mut spl)
            < 0 as libc::c_int
        {
            agerr(
                AGERR,
                b"in routesplines, Proutespline failed\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut pointf;
        }
    }
    let mut ps: *mut pointf = calloc(
        spl.pn as libc::c_ulong,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    if ps.is_null() {
        agerr(AGERR, b"cannot allocate ps\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut pointf;
    }
    bi = 0 as libc::c_int;
    while bi < boxn {
        (*boxes.offset(bi as isize)).LL.x = 2147483647 as libc::c_int as libc::c_double;
        (*boxes.offset(bi as isize))
            .UR
            .x = (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double;
        bi += 1;
    }
    unbounded = 1 as libc::c_int != 0;
    splinepi = 0 as libc::c_int;
    while splinepi < spl.pn {
        *ps.offset(splinepi as isize) = *(spl.ps).offset(splinepi as isize);
        splinepi += 1;
    }
    loopcnt = 0 as libc::c_int;
    while unbounded as libc::c_int != 0 && loopcnt < 15 as libc::c_int {
        limitBoxes(boxes, boxn, ps, spl.pn, delta);
        bi = 0 as libc::c_int;
        while bi < boxn {
            if (*boxes.offset(bi as isize)).LL.x
                == 2147483647 as libc::c_int as libc::c_double
                || (*boxes.offset(bi as isize)).UR.x
                    == (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_double
            {
                delta *= 2 as libc::c_int;
                if delta > 2147483647 as libc::c_int / boxn {
                    loopcnt = 15 as libc::c_int;
                }
                break;
            } else {
                bi += 1;
            }
        }
        if bi == boxn {
            unbounded = 0 as libc::c_int != 0;
        }
        loopcnt += 1;
    }
    if unbounded {
        let mut polyspl: Ppolyline_t = Ppolyline_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        };
        agerr(
            AGWARN,
            b"Unable to reclaim box space in spline routing for edge \"%s\" -> \"%s\". Something is probably seriously wrong.\n\0"
                as *const u8 as *const libc::c_char,
            agnameof(
                (*if ((*(realedge as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    realedge
                } else {
                    realedge.offset(1 as libc::c_int as isize)
                })
                    .node as *mut libc::c_void,
            ),
            agnameof(
                (*if ((*(realedge as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    realedge
                } else {
                    realedge.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut libc::c_void,
            ),
        );
        make_polyline(pl, &mut polyspl);
        limitBoxes(boxes, boxn, polyspl.ps, polyspl.pn, 10 as libc::c_int);
    }
    *npoints = spl.pn;
    return ps;
}
#[no_mangle]
pub unsafe extern "C" fn routesplines(
    mut pp: *mut path,
    mut npoints: *mut libc::c_int,
) -> *mut pointf {
    return _routesplines(pp, npoints, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn routepolylines(
    mut pp: *mut path,
    mut npoints: *mut libc::c_int,
) -> *mut pointf {
    return _routesplines(pp, npoints, 1 as libc::c_int);
}
unsafe extern "C" fn overlap(
    mut i0: libc::c_int,
    mut i1: libc::c_int,
    mut j0: libc::c_int,
    mut j1: libc::c_int,
) -> libc::c_int {
    if i1 <= j0 {
        return 0 as libc::c_int;
    }
    if i0 >= j1 {
        return 0 as libc::c_int;
    }
    if j0 <= i0 && i0 <= j1 {
        return j1 - i0;
    }
    if j0 <= i1 && i1 <= j1 {
        return i1 - j0;
    }
    return if i1 - i0 < j1 - j0 { i1 - i0 } else { j1 - j0 };
}
unsafe extern "C" fn checkpath(
    mut boxn: libc::c_int,
    mut boxes: *mut boxf,
    mut thepath: *mut path,
) -> libc::c_int {
    let mut ba: *mut boxf = 0 as *mut boxf;
    let mut bb: *mut boxf = 0 as *mut boxf;
    let mut bi: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut errs: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut xoverlap: libc::c_int = 0;
    let mut yoverlap: libc::c_int = 0;
    i = 0 as libc::c_int;
    bi = 0 as libc::c_int;
    while bi < boxn {
        if !(fabs((*boxes.offset(bi as isize)).LL.y - (*boxes.offset(bi as isize)).UR.y)
            < 0.01f64)
        {
            if !(fabs(
                (*boxes.offset(bi as isize)).LL.x - (*boxes.offset(bi as isize)).UR.x,
            ) < 0.01f64)
            {
                *boxes.offset(i as isize) = *boxes.offset(bi as isize);
                i += 1;
            }
        }
        bi += 1;
    }
    boxn = i;
    ba = &mut *boxes.offset(0 as libc::c_int as isize) as *mut boxf;
    if (*ba).LL.x > (*ba).UR.x || (*ba).LL.y > (*ba).UR.y {
        agerr(
            AGERR,
            b"in checkpath, box 0 has LL coord > UR coord\n\0" as *const u8
                as *const libc::c_char,
        );
        printpath(thepath);
        return 1 as libc::c_int;
    }
    bi = 0 as libc::c_int;
    while bi < boxn - 1 as libc::c_int {
        ba = &mut *boxes.offset(bi as isize) as *mut boxf;
        bb = &mut *boxes.offset((bi + 1 as libc::c_int) as isize) as *mut boxf;
        if (*bb).LL.x > (*bb).UR.x || (*bb).LL.y > (*bb).UR.y {
            agerr(
                AGERR,
                b"in checkpath, box %d has LL coord > UR coord\n\0" as *const u8
                    as *const libc::c_char,
                bi + 1 as libc::c_int,
            );
            printpath(thepath);
            return 1 as libc::c_int;
        }
        l = if (*ba).UR.x < (*bb).LL.x { 1 as libc::c_int } else { 0 as libc::c_int };
        r = if (*ba).LL.x > (*bb).UR.x { 1 as libc::c_int } else { 0 as libc::c_int };
        d = if (*ba).UR.y < (*bb).LL.y { 1 as libc::c_int } else { 0 as libc::c_int };
        u = if (*ba).LL.y > (*bb).UR.y { 1 as libc::c_int } else { 0 as libc::c_int };
        errs = l + r + d + u;
        if errs > 0 as libc::c_int && Verbose as libc::c_int != 0 {
            fprintf(
                stderr,
                b"in checkpath, boxes %d and %d don't touch\n\0" as *const u8
                    as *const libc::c_char,
                bi,
                bi + 1 as libc::c_int,
            );
            printpath(thepath);
        }
        if errs > 0 as libc::c_int {
            let mut xy: libc::c_int = 0;
            if l == 1 as libc::c_int {
                xy = (*ba).UR.x as libc::c_int;
                (*ba).UR.x = (*bb).LL.x;
                (*bb).LL.x = xy as libc::c_double;
                l = 0 as libc::c_int;
            } else if r == 1 as libc::c_int {
                xy = (*ba).LL.x as libc::c_int;
                (*ba).LL.x = (*bb).UR.x;
                (*bb).UR.x = xy as libc::c_double;
                r = 0 as libc::c_int;
            } else if d == 1 as libc::c_int {
                xy = (*ba).UR.y as libc::c_int;
                (*ba).UR.y = (*bb).LL.y;
                (*bb).LL.y = xy as libc::c_double;
                d = 0 as libc::c_int;
            } else if u == 1 as libc::c_int {
                xy = (*ba).LL.y as libc::c_int;
                (*ba).LL.y = (*bb).UR.y;
                (*bb).UR.y = xy as libc::c_double;
                u = 0 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < errs - 1 as libc::c_int {
                if l == 1 as libc::c_int {
                    xy = (((*ba).UR.x + (*bb).LL.x) / 2.0f64 + 0.5f64) as libc::c_int;
                    let ref mut fresh16 = (*bb).LL.x;
                    *fresh16 = xy as libc::c_double;
                    (*ba).UR.x = *fresh16;
                    l = 0 as libc::c_int;
                } else if r == 1 as libc::c_int {
                    xy = (((*ba).LL.x + (*bb).UR.x) / 2.0f64 + 0.5f64) as libc::c_int;
                    let ref mut fresh17 = (*bb).UR.x;
                    *fresh17 = xy as libc::c_double;
                    (*ba).LL.x = *fresh17;
                    r = 0 as libc::c_int;
                } else if d == 1 as libc::c_int {
                    xy = (((*ba).UR.y + (*bb).LL.y) / 2.0f64 + 0.5f64) as libc::c_int;
                    let ref mut fresh18 = (*bb).LL.y;
                    *fresh18 = xy as libc::c_double;
                    (*ba).UR.y = *fresh18;
                    d = 0 as libc::c_int;
                } else if u == 1 as libc::c_int {
                    xy = (((*ba).LL.y + (*bb).UR.y) / 2.0f64 + 0.5f64) as libc::c_int;
                    let ref mut fresh19 = (*bb).UR.y;
                    *fresh19 = xy as libc::c_double;
                    (*ba).LL.y = *fresh19;
                    u = 0 as libc::c_int;
                }
                i += 1;
            }
        }
        xoverlap = overlap(
            (*ba).LL.x as libc::c_int,
            (*ba).UR.x as libc::c_int,
            (*bb).LL.x as libc::c_int,
            (*bb).UR.x as libc::c_int,
        );
        yoverlap = overlap(
            (*ba).LL.y as libc::c_int,
            (*ba).UR.y as libc::c_int,
            (*bb).LL.y as libc::c_int,
            (*bb).UR.y as libc::c_int,
        );
        if xoverlap != 0 && yoverlap != 0 {
            if xoverlap < yoverlap {
                if (*ba).UR.x - (*ba).LL.x > (*bb).UR.x - (*bb).LL.x {
                    if (*ba).UR.x < (*bb).UR.x {
                        (*ba).UR.x = (*bb).LL.x;
                    } else {
                        (*ba).LL.x = (*bb).UR.x;
                    }
                } else if (*ba).UR.x < (*bb).UR.x {
                    (*bb).LL.x = (*ba).UR.x;
                } else {
                    (*bb).UR.x = (*ba).LL.x;
                }
            } else if (*ba).UR.y - (*ba).LL.y > (*bb).UR.y - (*bb).LL.y {
                if (*ba).UR.y < (*bb).UR.y {
                    (*ba).UR.y = (*bb).LL.y;
                } else {
                    (*ba).LL.y = (*bb).UR.y;
                }
            } else if (*ba).UR.y < (*bb).UR.y {
                (*bb).LL.y = (*ba).UR.y;
            } else {
                (*bb).UR.y = (*ba).LL.y;
            }
        }
        bi += 1;
    }
    if (*thepath).start.p.x < (*boxes.offset(0 as libc::c_int as isize)).LL.x
        || (*thepath).start.p.x > (*boxes.offset(0 as libc::c_int as isize)).UR.x
        || (*thepath).start.p.y < (*boxes.offset(0 as libc::c_int as isize)).LL.y
        || (*thepath).start.p.y > (*boxes.offset(0 as libc::c_int as isize)).UR.y
    {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"in checkpath, start port not in first box\n\0" as *const u8
                    as *const libc::c_char,
            );
            printpath(thepath);
        }
        (*thepath)
            .start
            .p
            .x = fmax(
            (*thepath).start.p.x,
            (*boxes.offset(0 as libc::c_int as isize)).LL.x,
        );
        (*thepath)
            .start
            .p
            .x = fmin(
            (*thepath).start.p.x,
            (*boxes.offset(0 as libc::c_int as isize)).UR.x,
        );
        (*thepath)
            .start
            .p
            .y = fmax(
            (*thepath).start.p.y,
            (*boxes.offset(0 as libc::c_int as isize)).LL.y,
        );
        (*thepath)
            .start
            .p
            .y = fmin(
            (*thepath).start.p.y,
            (*boxes.offset(0 as libc::c_int as isize)).UR.y,
        );
    }
    if (*thepath).end.p.x < (*boxes.offset((boxn - 1 as libc::c_int) as isize)).LL.x
        || (*thepath).end.p.x > (*boxes.offset((boxn - 1 as libc::c_int) as isize)).UR.x
        || (*thepath).end.p.y < (*boxes.offset((boxn - 1 as libc::c_int) as isize)).LL.y
        || (*thepath).end.p.y > (*boxes.offset((boxn - 1 as libc::c_int) as isize)).UR.y
    {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"in checkpath, end port not in last box\n\0" as *const u8
                    as *const libc::c_char,
            );
            printpath(thepath);
        }
        (*thepath)
            .end
            .p
            .x = fmax(
            (*thepath).end.p.x,
            (*boxes.offset((boxn - 1 as libc::c_int) as isize)).LL.x,
        );
        (*thepath)
            .end
            .p
            .x = fmin(
            (*thepath).end.p.x,
            (*boxes.offset((boxn - 1 as libc::c_int) as isize)).UR.x,
        );
        (*thepath)
            .end
            .p
            .y = fmax(
            (*thepath).end.p.y,
            (*boxes.offset((boxn - 1 as libc::c_int) as isize)).LL.y,
        );
        (*thepath)
            .end
            .p
            .y = fmin(
            (*thepath).end.p.y,
            (*boxes.offset((boxn - 1 as libc::c_int) as isize)).UR.y,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn printpath(mut pp: *mut path) {
    let mut bi: libc::c_int = 0;
    fprintf(stderr, b"%d boxes:\n\0" as *const u8 as *const libc::c_char, (*pp).nbox);
    bi = 0 as libc::c_int;
    while bi < (*pp).nbox {
        fprintf(
            stderr,
            b"%d (%.5g, %.5g), (%.5g, %.5g)\n\0" as *const u8 as *const libc::c_char,
            bi,
            (*((*pp).boxes).offset(bi as isize)).LL.x,
            (*((*pp).boxes).offset(bi as isize)).LL.y,
            (*((*pp).boxes).offset(bi as isize)).UR.x,
            (*((*pp).boxes).offset(bi as isize)).UR.y,
        );
        bi += 1;
    }
    fprintf(
        stderr,
        b"start port: (%.5g, %.5g), tangent angle: %.5g, %s\n\0" as *const u8
            as *const libc::c_char,
        (*pp).start.p.x,
        (*pp).start.p.y,
        (*pp).start.theta,
        if (*pp).start.constrained as libc::c_int != 0 {
            b"constrained\0" as *const u8 as *const libc::c_char
        } else {
            b"not constrained\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        stderr,
        b"end port: (%.5g, %.5g), tangent angle: %.5g, %s\n\0" as *const u8
            as *const libc::c_char,
        (*pp).end.p.x,
        (*pp).end.p.y,
        (*pp).end.theta,
        if (*pp).end.constrained as libc::c_int != 0 {
            b"constrained\0" as *const u8 as *const libc::c_char
        } else {
            b"not constrained\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn get_centroid(mut g: *mut Agraph_t) -> pointf {
    let mut sum: pointf = {
        let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
        init
    };
    sum
        .x = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
        + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x) / 2.0f64;
    sum
        .y = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
        + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y) / 2.0f64;
    return sum;
}
unsafe extern "C" fn vec_new() -> *mut vec {
    let mut pvec: *mut vec = malloc(::std::mem::size_of::<vec>() as libc::c_ulong)
        as *mut vec;
    (*pvec)._capelems = 10 as libc::c_int as size_t;
    (*pvec)._elems = 0 as libc::c_int as size_t;
    let ref mut fresh20 = (*pvec)._mem;
    *fresh20 = malloc(
        ((*pvec)._capelems)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    return pvec;
}
unsafe extern "C" fn vec_length(mut pvec: *const vec) -> size_t {
    return (*pvec)._elems;
}
unsafe extern "C" fn vec_get(
    mut pvec: *mut vec,
    mut index: size_t,
) -> *mut libc::c_void {
    if index < (*pvec)._elems {} else {
        __assert_fail(
            b"index < pvec->_elems\0" as *const u8 as *const libc::c_char,
            b"routespl.c\0" as *const u8 as *const libc::c_char,
            832 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void *vec_get(vec *, size_t)\0"))
                .as_ptr(),
        );
    }
    return *((*pvec)._mem).offset(index as isize);
}
unsafe extern "C" fn vec_delete(mut pvec: *mut vec) {
    free((*pvec)._mem as *mut libc::c_void);
    free(pvec as *mut libc::c_void);
}
unsafe extern "C" fn cycles_delete(mut cycles: *mut vec) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < vec_length(cycles) {
        vec_delete(vec_get(cycles, i) as *mut vec);
        i = i.wrapping_add(1);
    }
    vec_delete(cycles);
}
unsafe extern "C" fn vec_push_back(mut pvec: *mut vec, mut data: *mut libc::c_void) {
    if (*pvec)._elems == (*pvec)._capelems {
        let ref mut fresh21 = (*pvec)._capelems;
        *fresh21 = (*fresh21 as libc::c_ulong)
            .wrapping_add(10 as libc::c_int as libc::c_ulong) as size_t as size_t;
        let ref mut fresh22 = (*pvec)._mem;
        *fresh22 = realloc(
            (*pvec)._mem as *mut libc::c_void,
            ((*pvec)._capelems)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
    }
    let ref mut fresh23 = (*pvec)._elems;
    let fresh24 = *fresh23;
    *fresh23 = (*fresh23).wrapping_add(1);
    let ref mut fresh25 = *((*pvec)._mem).offset(fresh24 as isize);
    *fresh25 = data;
}
unsafe extern "C" fn vec_pop(mut pvec: *mut vec) -> *mut libc::c_void {
    if (*pvec)._elems > 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh26 = (*pvec)._elems;
        *fresh26 = (*fresh26).wrapping_sub(1);
        return *((*pvec)._mem).offset(*fresh26 as isize);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn vec_contains(
    mut pvec: *mut vec,
    mut item: *mut libc::c_void,
) -> bool {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*pvec)._elems {
        if *((*pvec)._mem).offset(i as isize) == item {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn vec_copy(mut pvec: *mut vec) -> *mut vec {
    let mut nvec: *mut vec = malloc(::std::mem::size_of::<vec>() as libc::c_ulong)
        as *mut vec;
    (*nvec)._capelems = (*pvec)._capelems;
    (*nvec)._elems = (*pvec)._elems;
    let ref mut fresh27 = (*nvec)._mem;
    *fresh27 = malloc(
        ((*pvec)._capelems)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    memcpy(
        (*nvec)._mem as *mut libc::c_void,
        (*pvec)._mem as *const libc::c_void,
        ((*pvec)._elems)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    );
    return nvec;
}
unsafe extern "C" fn cycle_contains_edge(
    mut cycle: *mut vec,
    mut edge: *mut edge_t,
) -> bool {
    let mut start: *mut node_t = (*if ((*(edge as *mut Agobj_t)).tag).objtype()
        as libc::c_int == 3 as libc::c_int
    {
        edge
    } else {
        edge.offset(1 as libc::c_int as isize)
    })
        .node;
    let mut end: *mut node_t = (*if ((*(edge as *mut Agobj_t)).tag).objtype()
        as libc::c_int == 2 as libc::c_int
    {
        edge
    } else {
        edge.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let mut c_start: *mut node_t = 0 as *mut node_t;
    let mut c_end: *mut node_t = 0 as *mut node_t;
    let mut cycle_len: size_t = vec_length(cycle);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < cycle_len {
        if i == 0 as libc::c_int as libc::c_ulong {
            c_start = vec_get(
                cycle,
                cycle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as *mut node_t;
        } else {
            c_start = vec_get(cycle, i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                as *mut node_t;
        }
        c_end = vec_get(cycle, i) as *mut node_t;
        if c_start == start && c_end == end {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_cycle_unique(mut cycles: *mut vec, mut cycle: *mut vec) -> bool {
    let mut cycle_len: size_t = vec_length(cycle);
    let mut n_cycles: size_t = vec_length(cycles);
    let mut c: size_t = 0;
    let mut i: size_t = 0;
    let mut cur_cycle: *mut vec = 0 as *mut vec;
    let mut cur_cycle_len: size_t = 0;
    let mut cur_cycle_item: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut all_items_match: bool = false;
    c = 0 as libc::c_int as size_t;
    while c < n_cycles {
        cur_cycle = vec_get(cycles, c) as *mut vec;
        cur_cycle_len = vec_length(cur_cycle);
        if cur_cycle_len == cycle_len {
            all_items_match = 1 as libc::c_int != 0;
            i = 0 as libc::c_int as size_t;
            while i < cur_cycle_len {
                cur_cycle_item = vec_get(cur_cycle, i);
                if !vec_contains(cycle, cur_cycle_item) {
                    all_items_match = 0 as libc::c_int != 0;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if all_items_match {
                return 0 as libc::c_int != 0;
            }
        }
        c = c.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn dfs(
    mut g: *mut graph_t,
    mut search: *mut node_t,
    mut visited: *mut vec,
    mut end: *mut node_t,
    mut cycles: *mut vec,
) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    if vec_contains(visited, search as *mut libc::c_void) {
        if search == end {
            if is_cycle_unique(cycles, visited) {
                let mut cycle: *mut vec = vec_copy(visited);
                vec_push_back(cycles, cycle as *mut libc::c_void);
            }
        }
    } else {
        vec_push_back(visited, search as *mut libc::c_void);
        e = agfstout(g, search);
        while !e.is_null() {
            n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
            dfs(g, n, visited, end, cycles);
            e = agnxtout(g, e);
        }
        vec_pop(visited);
    };
}
unsafe extern "C" fn find_all_cycles(mut g: *mut graph_t) -> *mut vec {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut alloced_cycles: *mut vec = vec_new();
    let mut cycles: *mut vec = vec_new();
    let mut cycle: *mut vec = 0 as *mut vec;
    n = agfstnode(g);
    while !n.is_null() {
        cycle = vec_new();
        vec_push_back(alloced_cycles, cycle as *mut libc::c_void);
        dfs(g, n, cycle, n, cycles);
        n = agnxtnode(g, n);
    }
    cycles_delete(alloced_cycles);
    return cycles;
}
unsafe extern "C" fn find_shortest_cycle_with_edge(
    mut cycles: *mut vec,
    mut edge: *mut edge_t,
    mut min_size: size_t,
) -> *mut vec {
    let mut c: size_t = 0;
    let mut cycles_len: size_t = vec_length(cycles);
    let mut cycle: *mut vec = 0 as *mut vec;
    let mut cycle_len: size_t = 0;
    let mut shortest: *mut vec = 0 as *mut vec;
    c = 0 as libc::c_int as size_t;
    while c < cycles_len {
        cycle = vec_get(cycles, c) as *mut vec;
        cycle_len = vec_length(cycle);
        if !(cycle_len < min_size) {
            if shortest.is_null() || vec_length(shortest) > cycle_len {
                if cycle_contains_edge(cycle, edge) {
                    shortest = cycle;
                }
            }
        }
        c = c.wrapping_add(1);
    }
    return shortest;
}
unsafe extern "C" fn get_cycle_centroid(
    mut g: *mut graph_t,
    mut edge: *mut edge_t,
) -> pointf {
    let mut cycles: *mut vec = find_all_cycles(g);
    let mut cycle: *mut vec = find_shortest_cycle_with_edge(
        cycles,
        edge,
        3 as libc::c_int as size_t,
    );
    let mut cycle_len: size_t = 0;
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut sum: pointf = {
        let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
        init
    };
    let mut idx: size_t = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    if cycle.is_null() {
        if !cycles.is_null() {
            cycles_delete(cycles);
        }
        return get_centroid(g);
    }
    cycle_len = vec_length(cycle);
    idx = 0 as libc::c_int as size_t;
    while idx < cycle_len {
        n = vec_get(cycle, idx) as *mut node_t;
        sum.x += (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
        sum.y += (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
        cnt = cnt.wrapping_add(1);
        idx = idx.wrapping_add(1);
    }
    if !cycles.is_null() {
        cycles_delete(cycles);
    }
    sum.x /= cnt as libc::c_double;
    sum.y /= cnt as libc::c_double;
    return sum;
}
unsafe extern "C" fn bend(mut spl: *mut pointf, mut centroid: pointf) {
    let mut midpt: pointf = pointf { x: 0., y: 0. };
    let mut a: pointf = pointf { x: 0., y: 0. };
    let mut r: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    midpt
        .x = ((*spl.offset(0 as libc::c_int as isize)).x
        + (*spl.offset(3 as libc::c_int as isize)).x) / 2.0f64;
    midpt
        .y = ((*spl.offset(0 as libc::c_int as isize)).y
        + (*spl.offset(3 as libc::c_int as isize)).y) / 2.0f64;
    dx = (*spl.offset(3 as libc::c_int as isize)).x
        - (*spl.offset(0 as libc::c_int as isize)).x;
    dy = (*spl.offset(3 as libc::c_int as isize)).y
        - (*spl.offset(0 as libc::c_int as isize)).y;
    dist = hypot(dx, dy);
    r = dist / 5.0f64;
    let mut vX: libc::c_double = centroid.x - midpt.x;
    let mut vY: libc::c_double = centroid.y - midpt.y;
    let mut magV: libc::c_double = hypot(vX, vY);
    if magV == 0 as libc::c_int as libc::c_double {
        return;
    }
    a.x = midpt.x - vX / magV * r;
    a.y = midpt.y - vY / magV * r;
    let ref mut fresh28 = (*spl.offset(2 as libc::c_int as isize)).x;
    *fresh28 = a.x;
    (*spl.offset(1 as libc::c_int as isize)).x = *fresh28;
    let ref mut fresh29 = (*spl.offset(2 as libc::c_int as isize)).y;
    *fresh29 = a.y;
    (*spl.offset(1 as libc::c_int as isize)).y = *fresh29;
}
#[no_mangle]
pub unsafe extern "C" fn makeStraightEdge(
    mut g: *mut graph_t,
    mut e: *mut edge_t,
    mut et: libc::c_int,
    mut sinfo: *mut splineInfo,
) {
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut e_cnt: libc::c_int = 0;
    e_cnt = 1 as libc::c_int;
    e0 = e;
    while e0 != (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt
        && {
            e0 = (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
            !e0.is_null()
        }
    {
        e_cnt += 1;
    }
    let mut edges_0: *mut *mut edge_t = gcalloc(
        e_cnt as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
    ) as *mut *mut edge_t;
    e0 = e;
    i = 0 as libc::c_int;
    while i < e_cnt {
        let ref mut fresh30 = *edges_0.offset(i as isize);
        *fresh30 = e0;
        e0 = (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
        i += 1;
    }
    makeStraightEdges(g, edges_0, e_cnt, et, sinfo);
    free(edges_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn makeStraightEdges(
    mut g: *mut graph_t,
    mut edges_0: *mut *mut edge_t,
    mut e_cnt: libc::c_int,
    mut et: libc::c_int,
    mut sinfo: *mut splineInfo,
) {
    let mut dumb: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut head: *mut node_t = 0 as *mut node_t;
    let mut curved: bool = et == (2 as libc::c_int) << 1 as libc::c_int;
    let mut perp: pointf = pointf { x: 0., y: 0. };
    let mut del: pointf = pointf { x: 0., y: 0. };
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut xstep: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut l_perp: libc::c_double = 0.;
    let mut dumber: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    e = *edges_0.offset(0 as libc::c_int as isize);
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    head = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    dumb[0 as libc::c_int
        as usize] = add_pointf(
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
    );
    dumb[1 as libc::c_int as usize] = dumb[0 as libc::c_int as usize];
    dumb[3 as libc::c_int
        as usize] = add_pointf(
        (*((*(head as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
    );
    dumb[2 as libc::c_int as usize] = dumb[3 as libc::c_int as usize];
    if e_cnt == 1 as libc::c_int || Concentrate as libc::c_int != 0 {
        if curved {
            bend(
                dumb.as_mut_ptr(),
                get_cycle_centroid(g, *edges_0.offset(0 as libc::c_int as isize)),
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
            dumb.as_mut_ptr(),
            4 as libc::c_int,
            sinfo,
        );
        addEdgeLabels(e);
        return;
    }
    e0 = e;
    if (dumb[0 as libc::c_int as usize].x - dumb[3 as libc::c_int as usize].x)
        * (dumb[0 as libc::c_int as usize].x - dumb[3 as libc::c_int as usize].x)
        + (dumb[0 as libc::c_int as usize].y - dumb[3 as libc::c_int as usize].y)
            * (dumb[0 as libc::c_int as usize].y - dumb[3 as libc::c_int as usize].y)
        < 0.001f64 * 0.001f64
    {
        dumb[1 as libc::c_int as usize] = dumb[0 as libc::c_int as usize];
        dumb[2 as libc::c_int as usize] = dumb[3 as libc::c_int as usize];
        del.x = 0 as libc::c_int as libc::c_double;
        del.y = 0 as libc::c_int as libc::c_double;
    } else {
        perp.x = dumb[0 as libc::c_int as usize].y - dumb[3 as libc::c_int as usize].y;
        perp.y = dumb[3 as libc::c_int as usize].x - dumb[0 as libc::c_int as usize].x;
        l_perp = sqrt(perp.x * perp.x + perp.y * perp.y);
        xstep = (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep;
        dx = xstep * (e_cnt - 1 as libc::c_int) / 2 as libc::c_int;
        dumb[1 as libc::c_int as usize]
            .x = dumb[0 as libc::c_int as usize].x
            + dx as libc::c_double * perp.x / l_perp;
        dumb[1 as libc::c_int as usize]
            .y = dumb[0 as libc::c_int as usize].y
            + dx as libc::c_double * perp.y / l_perp;
        dumb[2 as libc::c_int as usize]
            .x = dumb[3 as libc::c_int as usize].x
            + dx as libc::c_double * perp.x / l_perp;
        dumb[2 as libc::c_int as usize]
            .y = dumb[3 as libc::c_int as usize].y
            + dx as libc::c_double * perp.y / l_perp;
        del.x = -xstep as libc::c_double * perp.x / l_perp;
        del.y = -xstep as libc::c_double * perp.y / l_perp;
    }
    i = 0 as libc::c_int;
    while i < e_cnt {
        e0 = *edges_0.offset(i as isize);
        if (*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e0
        } else {
            e0.offset(-(1 as libc::c_int as isize))
        }))
            .node == head
        {
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                dumber[j as usize] = dumb[j as usize];
                j += 1;
            }
        } else {
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                dumber[(3 as libc::c_int - j) as usize] = dumb[j as usize];
                j += 1;
            }
        }
        if et == (3 as libc::c_int) << 1 as libc::c_int {
            let mut pts: [Ppoint_t; 4] = [pointf { x: 0., y: 0. }; 4];
            let mut spl: Ppolyline_t = Ppolyline_t {
                ps: 0 as *mut Ppoint_t,
                pn: 0,
            };
            let mut line: Ppolyline_t = Ppolyline_t {
                ps: 0 as *mut Ppoint_t,
                pn: 0,
            };
            line.pn = 4 as libc::c_int;
            line.ps = pts.as_mut_ptr();
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                pts[j as usize] = dumber[j as usize];
                j += 1;
            }
            make_polyline(line, &mut spl);
            clip_and_install(
                e0,
                (*if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e0
                } else {
                    e0.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                spl.ps,
                spl.pn,
                sinfo,
            );
        } else {
            clip_and_install(
                e0,
                (*if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e0
                } else {
                    e0.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                dumber.as_mut_ptr(),
                4 as libc::c_int,
                sinfo,
            );
        }
        addEdgeLabels(e0);
        dumb[1 as libc::c_int as usize].x += del.x;
        dumb[1 as libc::c_int as usize].y += del.y;
        dumb[2 as libc::c_int as usize].x += del.x;
        dumb[2 as libc::c_int as usize].y += del.y;
        i += 1;
    }
}
