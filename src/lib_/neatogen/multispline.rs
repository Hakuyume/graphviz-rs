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
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Concentrate: libc::c_uchar;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn line_intersect(
        a: pointf,
        b: pointf,
        c: pointf,
        d: pointf,
        p: *mut pointf,
    ) -> libc::c_int;
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
    fn addEdgeLabels(e: *mut edge_t);
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
    fn area2(_: Ppoint_t, _: Ppoint_t, _: Ppoint_t) -> COORD;
    fn wind(a: Ppoint_t, b: Ppoint_t, c: Ppoint_t) -> libc::c_int;
    static mut Dtoset: *mut Dtmethod_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn freeSurface(s: *mut surface_t);
    fn mkSurface(
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        n: libc::c_int,
        segs: *mut libc::c_int,
        nsegs: libc::c_int,
    ) -> *mut surface_t;
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
pub type COORD = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct router_s {
    pub pn: libc::c_int,
    pub ps: *mut pointf,
    pub obs: *mut libc::c_int,
    pub tris: *mut libc::c_int,
    pub trimap: *mut Dt_t,
    pub tn: libc::c_int,
    pub tg: *mut tgraph,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tgraph {
    pub nodes: *mut tnode,
    pub edges: *mut tedge,
    pub nedges: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tedge {
    pub t: libc::c_int,
    pub h: libc::c_int,
    pub seg: ipair,
    pub dist: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipair {
    pub i: libc::c_int,
    pub j: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tnode {
    pub ne: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ctr: pointf,
}
pub type router_t = router_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surface_t {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub nfaces: libc::c_int,
    pub faces: *mut libc::c_int,
    pub neigh: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub link: Dtlink_t,
    pub a: [libc::c_int; 2],
    pub t: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tripoly_t {
    pub poly: Ppoly_t,
    pub triMap: *mut *mut tri,
}
pub type tri = _tri;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tri {
    pub v: ipair,
    pub nxttri: *mut _tri,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct side_t {
    pub v: libc::c_int,
    pub ts: *mut tri,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ipair {
    pub link: Dtlink_t,
    pub i: libc::c_int,
    pub j: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PQ {
    pub pq: *mut libc::c_int,
    pub PQcnt: libc::c_int,
    pub PQsize: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PPQ {
    pub pq: PQ,
    pub vals: *mut libc::c_float,
    pub idxs: *mut libc::c_int,
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
unsafe extern "C" fn cmpItem(
    mut d: *mut Dt_t,
    mut p1: *mut libc::c_int,
    mut p2: *mut libc::c_int,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if *p1.offset(0 as libc::c_int as isize) < *p2.offset(0 as libc::c_int as isize) {
        return -(1 as libc::c_int)
    } else if *p1.offset(0 as libc::c_int as isize)
            > *p2.offset(0 as libc::c_int as isize)
        {
        return 1 as libc::c_int
    } else if *p1.offset(1 as libc::c_int as isize)
            < *p2.offset(1 as libc::c_int as isize)
        {
        return -(1 as libc::c_int)
    } else if *p1.offset(1 as libc::c_int as isize)
            > *p2.offset(1 as libc::c_int as isize)
        {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn newItem(
    mut d: *mut Dt_t,
    mut objp: *mut item,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut newp: *mut item = zmalloc(::std::mem::size_of::<item>() as libc::c_ulong)
        as *mut item;
    (*newp).a[0 as libc::c_int as usize] = (*objp).a[0 as libc::c_int as usize];
    (*newp).a[1 as libc::c_int as usize] = (*objp).a[1 as libc::c_int as usize];
    (*newp).t = (*objp).t;
    return newp as *mut libc::c_void;
}
unsafe extern "C" fn freeItem(
    mut d: *mut Dt_t,
    mut obj: *mut item,
    mut disc: *mut Dtdisc_t,
) {
    free(obj as *mut libc::c_void);
}
static mut itemdisc: Dtdisc_t = Dtdisc_t {
    key: 0,
    size: 0,
    link: 0,
    makef: None,
    freef: None,
    comparf: None,
    hashf: None,
    memoryf: None,
    eventf: None,
};
unsafe extern "C" fn addMap(
    mut map: *mut Dt_t,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut t: libc::c_int,
) {
    let mut it: item = item {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed_1 { _hash: 0 },
        },
        a: [0; 2],
        t: 0,
    };
    let mut tmp: libc::c_int = 0;
    if a > b {
        tmp = a;
        a = b;
        b = tmp;
    }
    it.a[0 as libc::c_int as usize] = a;
    it.a[1 as libc::c_int as usize] = b;
    it.t = t;
    (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, &mut it as *mut item as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn mapSegToTri(mut sf: *mut surface_t) -> *mut Dt_t {
    let mut map: *mut Dt_t = dtopen(&mut itemdisc, Dtoset);
    let mut i: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ps: *mut libc::c_int = (*sf).faces;
    i = 0 as libc::c_int;
    while i < (*sf).nfaces {
        let fresh0 = ps;
        ps = ps.offset(1);
        a = *fresh0;
        let fresh1 = ps;
        ps = ps.offset(1);
        b = *fresh1;
        let fresh2 = ps;
        ps = ps.offset(1);
        c = *fresh2;
        addMap(map, a, b, i);
        addMap(map, b, c, i);
        addMap(map, c, a, i);
        i += 1;
    }
    return map;
}
unsafe extern "C" fn findMap(
    mut map: *mut Dt_t,
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    let mut it: item = item {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed_1 { _hash: 0 },
        },
        a: [0; 2],
        t: 0,
    };
    let mut ip: *mut item = 0 as *mut item;
    if a > b {
        let mut tmp: libc::c_int = a;
        a = b;
        b = tmp;
    }
    it.a[0 as libc::c_int as usize] = a;
    it.a[1 as libc::c_int as usize] = b;
    ip = (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, &mut it as *mut item as *mut libc::c_void, 0o4 as libc::c_int)
        as *mut item;
    if !ip.is_null() {} else {
        __assert_fail(
            b"ip\0" as *const u8 as *const libc::c_char,
            b"multispline.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"int findMap(Dt_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    return (*ip).t;
}
unsafe extern "C" fn cmpIpair(
    mut d: *mut Dt_t,
    mut p1: *mut libc::c_int,
    mut p2: *mut libc::c_int,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    return *p1 - *p2;
}
unsafe extern "C" fn newIpair(
    mut d: *mut Dt_t,
    mut objp: *mut Ipair,
    mut disc: *mut Dtdisc_t,
) -> *mut libc::c_void {
    let mut newp: *mut Ipair = zmalloc(::std::mem::size_of::<Ipair>() as libc::c_ulong)
        as *mut Ipair;
    (*newp).i = (*objp).i;
    (*newp).j = (*objp).j;
    return newp as *mut libc::c_void;
}
unsafe extern "C" fn freeIpair(
    mut d: *mut Dt_t,
    mut obj: *mut Ipair,
    mut disc: *mut Dtdisc_t,
) {
    free(obj as *mut libc::c_void);
}
static mut ipairdisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut Ipair,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
                >,
                Dtmake_f,
            >(
                Some(
                    newIpair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut Ipair,
                            *mut Dtdisc_t,
                        ) -> *mut libc::c_void,
                ),
            ),
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut Ipair, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freeIpair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut Ipair,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    cmpIpair
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut libc::c_int,
                            *mut libc::c_int,
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
unsafe extern "C" fn vmapAdd(
    mut map: *mut Dt_t,
    mut i: libc::c_int,
    mut j: libc::c_int,
) {
    let mut obj: Ipair = Ipair {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed_1 { _hash: 0 },
        },
        i: 0,
        j: 0,
    };
    obj.i = i;
    obj.j = j;
    (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, &mut obj as *mut Ipair as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn vMap(mut map: *mut Dt_t, mut i: libc::c_int) -> libc::c_int {
    let mut ip: *mut Ipair = 0 as *mut Ipair;
    ip = (Some(((*map).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(map, &mut i as *mut libc::c_int as *mut libc::c_void, 0o1000 as libc::c_int)
        as *mut Ipair;
    return (*ip).j;
}
unsafe extern "C" fn mapTri(mut map: *mut Dt_t, mut tp: *mut tri) {
    while !tp.is_null() {
        (*tp).v.i = vMap(map, (*tp).v.i);
        (*tp).v.j = vMap(map, (*tp).v.j);
        tp = (*tp).nxttri;
    }
}
unsafe extern "C" fn addTri(
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut oldp: *mut tri,
) -> *mut tri {
    let mut tp: *mut tri = zmalloc(::std::mem::size_of::<tri>() as libc::c_ulong)
        as *mut tri;
    (*tp).v.i = i;
    (*tp).v.j = j;
    let ref mut fresh3 = (*tp).nxttri;
    *fresh3 = oldp;
    return tp;
}
unsafe extern "C" fn bisect(
    mut pp: pointf,
    mut cp: pointf,
    mut np: pointf,
) -> libc::c_double {
    let mut theta: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    theta = atan2(np.y - cp.y, np.x - cp.x);
    phi = atan2(pp.y - cp.y, pp.x - cp.x);
    return (theta + phi) / 2.0f64;
}
unsafe extern "C" fn raySeg(
    mut v: pointf,
    mut w: pointf,
    mut a: pointf,
    mut b: pointf,
) -> libc::c_int {
    let mut wa: libc::c_int = wind(v, w, a);
    let mut wb: libc::c_int = wind(v, w, b);
    if wa == wb {
        return 0 as libc::c_int;
    }
    if wa == 0 as libc::c_int {
        return (wind(v, b, w) * wind(v, b, a) >= 0 as libc::c_int) as libc::c_int
    } else {
        return (wind(v, a, w) * wind(v, a, b) >= 0 as libc::c_int) as libc::c_int
    };
}
unsafe extern "C" fn raySegIntersect(
    mut v: pointf,
    mut w: pointf,
    mut a: pointf,
    mut b: pointf,
    mut p: *mut pointf,
) -> libc::c_int {
    if raySeg(v, w, a, b) != 0 {
        return line_intersect(v, w, a, b, p)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn triPoint(
    mut trip: *mut tripoly_t,
    mut vx: libc::c_int,
    mut v: pointf,
    mut w: pointf,
    mut ip: *mut pointf,
) -> libc::c_int {
    let mut tp: *mut tri = 0 as *mut tri;
    tp = *((*trip).triMap).offset(vx as isize);
    while !tp.is_null() {
        if raySegIntersect(
            v,
            w,
            *((*trip).poly.ps).offset((*tp).v.i as isize),
            *((*trip).poly.ps).offset((*tp).v.j as isize),
            ip,
        ) != 0
        {
            return 0 as libc::c_int;
        }
        tp = (*tp).nxttri;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ctrlPtIdx(mut v: pointf, mut polys: *mut Ppoly_t) -> libc::c_int {
    let mut w: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*polys).pn {
        w = *((*polys).ps).offset(i as isize);
        if w.x == v.x && w.y == v.y {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mkCtrlPts(
    mut s: libc::c_int,
    mut mult: libc::c_int,
    mut prev: pointf,
    mut v: pointf,
    mut nxt: pointf,
    mut trip: *mut tripoly_t,
) -> *mut pointf {
    let mut ps: *mut pointf = 0 as *mut pointf;
    let mut idx: libc::c_int = ctrlPtIdx(v, &mut (*trip).poly);
    let mut i: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut sep: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut sinTheta: libc::c_double = 0.;
    let mut cosTheta: libc::c_double = 0.;
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut w: pointf = pointf { x: 0., y: 0. };
    if idx < 0 as libc::c_int {
        return 0 as *mut pointf;
    }
    ps = gcalloc(mult as size_t, ::std::mem::size_of::<pointf>() as libc::c_ulong)
        as *mut pointf;
    theta = bisect(prev, v, nxt);
    sinTheta = sin(theta);
    cosTheta = cos(theta);
    w.x = v.x + 100 as libc::c_int as libc::c_double * cosTheta;
    w.y = v.y + 100 as libc::c_int as libc::c_double * sinTheta;
    if idx > s {
        if wind(prev, v, w) != 1 as libc::c_int {
            sinTheta *= -(1 as libc::c_int) as libc::c_double;
            cosTheta *= -(1 as libc::c_int) as libc::c_double;
            w.x = v.x + 100 as libc::c_int as libc::c_double * cosTheta;
            w.y = v.y + 100 as libc::c_int as libc::c_double * sinTheta;
        }
    } else if wind(prev, v, w) != -(1 as libc::c_int) {
        sinTheta *= -(1 as libc::c_int) as libc::c_double;
        cosTheta *= -(1 as libc::c_int) as libc::c_double;
        w.x = v.x + 100 as libc::c_int as libc::c_double * cosTheta;
        w.y = v.y + 100 as libc::c_int as libc::c_double * sinTheta;
    }
    if triPoint(trip, idx, v, w, &mut q) != 0 {
        return 0 as *mut pointf;
    }
    d = sqrt((q.x - v.x) * (q.x - v.x) + (q.y - v.y) * (q.y - v.y));
    if d >= (mult * 15 as libc::c_int) as libc::c_double {
        sep = 15 as libc::c_int as libc::c_double;
    } else {
        sep = d / mult as libc::c_double;
    }
    if idx < s {
        i = 0 as libc::c_int;
        while i < mult {
            (*ps.offset(i as isize)).x = v.x + i as libc::c_double * sep * cosTheta;
            (*ps.offset(i as isize)).y = v.y + i as libc::c_double * sep * sinTheta;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < mult {
            (*ps.offset((mult - i - 1 as libc::c_int) as isize))
                .x = v.x + i as libc::c_double * sep * cosTheta;
            (*ps.offset((mult - i - 1 as libc::c_int) as isize))
                .y = v.y + i as libc::c_double * sep * sinTheta;
            i += 1;
        }
    }
    return ps;
}
unsafe extern "C" fn triCenter(
    mut pts: *mut pointf,
    mut idxs: *mut libc::c_int,
) -> pointf {
    let fresh4 = idxs;
    idxs = idxs.offset(1);
    let mut a: pointf = *pts.offset(*fresh4 as isize);
    let fresh5 = idxs;
    idxs = idxs.offset(1);
    let mut b: pointf = *pts.offset(*fresh5 as isize);
    let fresh6 = idxs;
    idxs = idxs.offset(1);
    let mut c: pointf = *pts.offset(*fresh6 as isize);
    let mut p: pointf = pointf { x: 0., y: 0. };
    p.x = (a.x + b.x + c.x) / 3.0f64;
    p.y = (a.y + b.y + c.y) / 3.0f64;
    return p;
}
unsafe extern "C" fn bbox(
    mut obsp: *mut *mut Ppoly_t,
    mut npoly: libc::c_int,
    mut np: *mut libc::c_int,
) -> boxf {
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut obs: *mut Ppoly_t = 0 as *mut Ppoly_t;
    bb.LL.y = 1.7976931348623157e+308f64;
    bb.LL.x = bb.LL.y;
    bb.UR.y = -1.7976931348623157e+308f64;
    bb.UR.x = bb.UR.y;
    i = 0 as libc::c_int;
    while i < npoly {
        let fresh7 = obsp;
        obsp = obsp.offset(1);
        obs = *fresh7;
        j = 0 as libc::c_int;
        while j < (*obs).pn {
            p = *((*obs).ps).offset(j as isize);
            if p.x < bb.LL.x {
                bb.LL.x = p.x;
            }
            if p.x > bb.UR.x {
                bb.UR.x = p.x;
            }
            if p.y < bb.LL.y {
                bb.LL.y = p.y;
            }
            if p.y > bb.UR.y {
                bb.UR.y = p.y;
            }
            cnt += 1;
            j += 1;
        }
        i += 1;
    }
    *np = cnt;
    bb.LL.x -= 32 as libc::c_int as libc::c_double;
    bb.LL.y -= 32 as libc::c_int as libc::c_double;
    bb.UR.x += 32 as libc::c_int as libc::c_double;
    bb.UR.y += 32 as libc::c_int as libc::c_double;
    return bb;
}
unsafe extern "C" fn mkTriIndices(mut sf: *mut surface_t) -> *mut libc::c_int {
    let mut tris: *mut libc::c_int = gcalloc(
        (3 as libc::c_int * (*sf).nfaces) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    memcpy(
        tris as *mut libc::c_void,
        (*sf).faces as *const libc::c_void,
        ((3 as libc::c_int * (*sf).nfaces) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    return tris;
}
unsafe extern "C" fn degT(mut ip: *mut libc::c_int) -> libc::c_int {
    ip = ip.offset(1);
    let fresh8 = ip;
    ip = ip.offset(1);
    if *fresh8 == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    if *ip == -(1 as libc::c_int) {
        return 2 as libc::c_int
    } else {
        return 3 as libc::c_int
    };
}
unsafe extern "C" fn sharedEdge(
    mut p: *mut libc::c_int,
    mut q: *mut libc::c_int,
) -> ipair {
    let mut pt: ipair = ipair { i: 0, j: 0 };
    let mut tmp: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    p1 = *p;
    p2 = *p.offset(1 as libc::c_int as isize);
    if p1 == *q {
        if p2 != *q.offset(1 as libc::c_int as isize)
            && p2 != *q.offset(2 as libc::c_int as isize)
        {
            p2 = *p.offset(2 as libc::c_int as isize);
        }
    } else if p1 == *q.offset(1 as libc::c_int as isize) {
        if p2 != *q && p2 != *q.offset(2 as libc::c_int as isize) {
            p2 = *p.offset(2 as libc::c_int as isize);
        }
    } else if p1 == *q.offset(2 as libc::c_int as isize) {
        if p2 != *q && p2 != *q.offset(1 as libc::c_int as isize) {
            p2 = *p.offset(2 as libc::c_int as isize);
        }
    } else {
        p1 = *p.offset(2 as libc::c_int as isize);
    }
    if p1 > p2 {
        tmp = p1;
        p1 = p2;
        p2 = tmp;
    }
    pt.i = p1;
    pt.j = p2;
    return pt;
}
unsafe extern "C" fn addTriEdge(
    mut g: *mut tgraph,
    mut t: libc::c_int,
    mut h: libc::c_int,
    mut d: libc::c_double,
    mut seg: ipair,
) {
    let mut ep: *mut tedge = ((*g).edges).offset((*g).nedges as isize);
    let mut tp: *mut tnode = ((*g).nodes).offset(t as isize);
    let mut hp: *mut tnode = ((*g).nodes).offset(h as isize);
    (*ep).t = t;
    (*ep).h = h;
    (*ep)
        .dist = sqrt(
        ((*tp).ctr.x - (*hp).ctr.x) * ((*tp).ctr.x - (*hp).ctr.x)
            + ((*tp).ctr.y - (*hp).ctr.y) * ((*tp).ctr.y - (*hp).ctr.y),
    );
    (*ep).seg = seg;
    let ref mut fresh9 = (*tp).ne;
    let fresh10 = *fresh9;
    *fresh9 = *fresh9 + 1;
    *((*tp).edges).offset(fresh10 as isize) = (*g).nedges;
    let ref mut fresh11 = (*hp).ne;
    let fresh12 = *fresh11;
    *fresh11 = *fresh11 + 1;
    *((*hp).edges).offset(fresh12 as isize) = (*g).nedges;
    let ref mut fresh13 = (*g).nedges;
    *fresh13 += 1;
}
unsafe extern "C" fn freeTriGraph(mut tg: *mut tgraph) {
    free((*(*tg).nodes).edges as *mut libc::c_void);
    free((*tg).nodes as *mut libc::c_void);
    free((*tg).edges as *mut libc::c_void);
    free(tg as *mut libc::c_void);
}
unsafe extern "C" fn mkTriGraph(
    mut sf: *mut surface_t,
    mut maxv: libc::c_int,
    mut pts: *mut pointf,
) -> *mut tgraph {
    let mut g: *mut tgraph = 0 as *mut tgraph;
    let mut np: *mut tnode = 0 as *mut tnode;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ne: libc::c_int = 0 as libc::c_int;
    let mut edgei: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jp: *mut libc::c_int = 0 as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int * (*sf).nfaces {
        if *((*sf).neigh).offset(i as isize) != -(1 as libc::c_int) {
            ne += 1;
        }
        i += 1;
    }
    g = gmalloc(::std::mem::size_of::<tgraph>() as libc::c_ulong) as *mut tgraph;
    let ref mut fresh14 = (*g).nodes;
    *fresh14 = gcalloc(
        ((*sf).nfaces + 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<tnode>() as libc::c_ulong,
    ) as *mut tnode;
    edgei = gcalloc(
        ((*sf).nfaces + ne + 2 as libc::c_int * maxv) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let ref mut fresh15 = (*g).edges;
    *fresh15 = gcalloc(
        (ne / 2 as libc::c_int + 2 as libc::c_int * maxv) as size_t,
        ::std::mem::size_of::<tedge>() as libc::c_ulong,
    ) as *mut tedge;
    (*g).nedges = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*sf).nfaces {
        np = ((*g).nodes).offset(i as isize);
        (*np).ne = 0 as libc::c_int;
        let ref mut fresh16 = (*np).edges;
        *fresh16 = edgei;
        (*np)
            .ctr = triCenter(pts, ((*sf).faces).offset((3 as libc::c_int * i) as isize));
        edgei = edgei
            .offset(
                (degT(((*sf).neigh).offset((3 as libc::c_int * i) as isize))
                    + 1 as libc::c_int) as isize,
            );
        i += 1;
    }
    np = ((*g).nodes).offset(i as isize);
    (*np).ne = 0 as libc::c_int;
    let ref mut fresh17 = (*np).edges;
    *fresh17 = edgei;
    np = np.offset(1);
    (*np).ne = 0 as libc::c_int;
    let ref mut fresh18 = (*np).edges;
    *fresh18 = edgei.offset(maxv as isize);
    i = 0 as libc::c_int;
    while i < (*sf).nfaces {
        np = ((*g).nodes).offset(i as isize);
        jp = ((*sf).neigh).offset((3 as libc::c_int * i) as isize);
        ne = 0 as libc::c_int;
        while ne < 3 as libc::c_int
            && {
                let fresh19 = jp;
                jp = jp.offset(1);
                j = *fresh19;
                j != -(1 as libc::c_int)
            }
        {
            if i < j {
                let mut dist: libc::c_double = sqrt(
                    ((*np).ctr.x - (*((*g).nodes).offset(j as isize)).ctr.x)
                        * ((*np).ctr.x - (*((*g).nodes).offset(j as isize)).ctr.x)
                        + ((*np).ctr.y - (*((*g).nodes).offset(j as isize)).ctr.y)
                            * ((*np).ctr.y - (*((*g).nodes).offset(j as isize)).ctr.y),
                );
                let mut seg: ipair = sharedEdge(
                    ((*sf).faces).offset((3 as libc::c_int * i) as isize),
                    ((*sf).faces).offset((3 as libc::c_int * j) as isize),
                );
                addTriEdge(g, i, j, dist, seg);
            }
            ne += 1;
        }
        i += 1;
    }
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn freeRouter(mut rtr: *mut router_t) {
    free((*rtr).ps as *mut libc::c_void);
    free((*rtr).obs as *mut libc::c_void);
    free((*rtr).tris as *mut libc::c_void);
    dtclose((*rtr).trimap);
    freeTriGraph((*rtr).tg);
    free(rtr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mkRouter(
    mut obsp: *mut *mut Ppoly_t,
    mut npoly: libc::c_int,
) -> *mut router_t {
    let mut rtr: *mut router_t = zmalloc(
        ::std::mem::size_of::<router_t>() as libc::c_ulong,
    ) as *mut router_t;
    let mut obs: *mut Ppoly_t = 0 as *mut Ppoly_t;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut pts: *mut pointf = 0 as *mut pointf;
    let mut npts: libc::c_int = 0;
    let mut sf: *mut surface_t = 0 as *mut surface_t;
    let mut segs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut maxv: libc::c_int = 4 as libc::c_int;
    let mut obsi: *mut libc::c_int = gcalloc(
        (npoly + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ix: libc::c_int = 4 as libc::c_int;
    let mut six: libc::c_int = 0 as libc::c_int;
    bb = bbox(obsp, npoly, &mut npts);
    npts += 4 as libc::c_int;
    pts = gcalloc(npts as size_t, ::std::mem::size_of::<pointf>() as libc::c_ulong)
        as *mut pointf;
    segs = gcalloc(
        (2 as libc::c_int * npts) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *pts.offset(0 as libc::c_int as isize) = bb.LL;
    (*pts.offset(1 as libc::c_int as isize)).x = bb.UR.x;
    (*pts.offset(1 as libc::c_int as isize)).y = bb.LL.y;
    *pts.offset(2 as libc::c_int as isize) = bb.UR;
    (*pts.offset(3 as libc::c_int as isize)).x = bb.LL.x;
    (*pts.offset(3 as libc::c_int as isize)).y = bb.UR.y;
    i = 1 as libc::c_int;
    while i <= 4 as libc::c_int {
        let fresh20 = six;
        six = six + 1;
        *segs.offset(fresh20 as isize) = i - 1 as libc::c_int;
        if i < 4 as libc::c_int {
            let fresh21 = six;
            six = six + 1;
            *segs.offset(fresh21 as isize) = i;
        } else {
            let fresh22 = six;
            six = six + 1;
            *segs.offset(fresh22 as isize) = 0 as libc::c_int;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < npoly {
        *obsi.offset(i as isize) = ix;
        let fresh23 = obsp;
        obsp = obsp.offset(1);
        obs = *fresh23;
        j = 1 as libc::c_int;
        while j <= (*obs).pn {
            let fresh24 = six;
            six = six + 1;
            *segs.offset(fresh24 as isize) = ix;
            if j < (*obs).pn {
                let fresh25 = six;
                six = six + 1;
                *segs.offset(fresh25 as isize) = ix + 1 as libc::c_int;
            } else {
                let fresh26 = six;
                six = six + 1;
                *segs.offset(fresh26 as isize) = *obsi.offset(i as isize);
            }
            let fresh27 = ix;
            ix = ix + 1;
            *pts
                .offset(
                    fresh27 as isize,
                ) = *((*obs).ps).offset((j - 1 as libc::c_int) as isize);
            j += 1;
        }
        if (*obs).pn > maxv {
            maxv = (*obs).pn;
        }
        i += 1;
    }
    *obsi.offset(i as isize) = ix;
    x = gcalloc(npts as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    y = gcalloc(npts as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < npts {
        *x.offset(i as isize) = (*pts.offset(i as isize)).x;
        *y.offset(i as isize) = (*pts.offset(i as isize)).y;
        i += 1;
    }
    sf = mkSurface(x, y, npts, segs, npts);
    free(x as *mut libc::c_void);
    free(y as *mut libc::c_void);
    free(segs as *mut libc::c_void);
    let ref mut fresh28 = (*rtr).ps;
    *fresh28 = pts;
    (*rtr).pn = npts;
    let ref mut fresh29 = (*rtr).obs;
    *fresh29 = obsi;
    let ref mut fresh30 = (*rtr).tris;
    *fresh30 = mkTriIndices(sf);
    let ref mut fresh31 = (*rtr).trimap;
    *fresh31 = mapSegToTri(sf);
    (*rtr).tn = (*sf).nfaces;
    let ref mut fresh32 = (*rtr).tg;
    *fresh32 = mkTriGraph(sf, maxv, pts);
    freeSurface(sf);
    return rtr;
}
unsafe extern "C" fn finishEdge(
    mut e: *mut edge_t,
    mut spl: Ppoly_t,
    mut flip: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut spline: *mut pointf = gcalloc(
        spl.pn as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    if flip != 0 {
        j = 0 as libc::c_int;
        while j < spl.pn {
            *spline
                .offset(
                    (spl.pn - 1 as libc::c_int - j) as isize,
                ) = *(spl.ps).offset(j as isize);
            j += 1;
        }
    } else {
        j = 0 as libc::c_int;
        while j < spl.pn {
            *spline.offset(j as isize) = *(spl.ps).offset(j as isize);
            j += 1;
        }
    }
    if Verbose as libc::c_int > 1 as libc::c_int {
        fprintf(
            stderr,
            b"spline %s %s\n\0" as *const u8 as *const libc::c_char,
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
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
        spline,
        spl.pn,
        &mut sinfo,
    );
    free(spline as *mut libc::c_void);
    addEdgeLabels(e);
}
unsafe extern "C" fn tweakEnd(
    mut poly: Ppoly_t,
    mut s: libc::c_int,
    mut pl: Ppolyline_t,
    mut q: Ppoint_t,
) -> Ppoint_t {
    let mut prv: Ppoint_t = pointf { x: 0., y: 0. };
    let mut nxt: Ppoint_t = pointf { x: 0., y: 0. };
    let mut p: Ppoint_t = pointf { x: 0., y: 0. };
    p = *(poly.ps).offset(s as isize);
    nxt = *(poly.ps).offset(((s + 1 as libc::c_int) % poly.pn) as isize);
    if s == 0 as libc::c_int {
        prv = *(poly.ps).offset((poly.pn - 1 as libc::c_int) as isize);
    } else {
        prv = *(poly.ps).offset((s - 1 as libc::c_int) as isize);
    }
    if q.x == nxt.x && q.y == nxt.y || q.x == prv.x && q.y == prv.y {
        let mut m: Ppoint_t = pointf { x: 0., y: 0. };
        let mut d: libc::c_double = 0.;
        m.x = (nxt.x + prv.x) / 2.0f64 - p.x;
        m.y = (nxt.y + prv.y) / 2.0f64 - p.y;
        d = sqrt(m.x * m.x + m.y * m.y);
        p.x += 0.1f64 * m.x / d;
        p.y += 0.1f64 * m.y / d;
    }
    return p;
}
unsafe extern "C" fn tweakPath(
    mut poly: Ppoly_t,
    mut s: libc::c_int,
    mut t: libc::c_int,
    mut pl: Ppolyline_t,
) {
    *(pl.ps)
        .offset(
            0 as libc::c_int as isize,
        ) = tweakEnd(poly, s, pl, *(pl.ps).offset(1 as libc::c_int as isize));
    *(pl.ps)
        .offset(
            (pl.pn - 1 as libc::c_int) as isize,
        ) = tweakEnd(poly, t, pl, *(pl.ps).offset((pl.pn - 2 as libc::c_int) as isize));
}
unsafe extern "C" fn genroute(
    mut trip: *mut tripoly_t,
    mut s: libc::c_int,
    mut t: libc::c_int,
    mut e: *mut edge_t,
    mut doPolyline: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut eps: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut evs: [Pvector_t; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut cpts: *mut *mut pointf = 0 as *mut *mut pointf;
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
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mmpl: Ppolyline_t = Ppolyline_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut medges: *mut Pedge_t = gcalloc(
        (*trip).poly.pn as size_t,
        ::std::mem::size_of::<Pedge_t>() as libc::c_ulong,
    ) as *mut Pedge_t;
    let mut pn: libc::c_int = 0;
    let mut mult: libc::c_int = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .count as libc::c_int;
    let mut head: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype()
        as libc::c_int == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let mut rv: libc::c_int = 0 as libc::c_int;
    poly.ps = 0 as *mut Ppoint_t;
    pl.pn = 0 as libc::c_int;
    eps[0 as libc::c_int as usize].x = (*((*trip).poly.ps).offset(s as isize)).x;
    eps[0 as libc::c_int as usize].y = (*((*trip).poly.ps).offset(s as isize)).y;
    eps[1 as libc::c_int as usize].x = (*((*trip).poly.ps).offset(t as isize)).x;
    eps[1 as libc::c_int as usize].y = (*((*trip).poly.ps).offset(t as isize)).y;
    if Pshortestpath(&mut (*trip).poly, eps.as_mut_ptr(), &mut pl) < 0 as libc::c_int {
        agerr(
            AGWARN,
            b"Could not create control points for multiple spline for edge (%s,%s)\n\0"
                as *const u8 as *const libc::c_char,
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
        rv = 1 as libc::c_int;
    } else if pl.pn == 2 as libc::c_int {
        makeStraightEdge(agraphof(head as *mut libc::c_void), e, doPolyline, &mut sinfo);
    } else {
        evs[0 as libc::c_int as usize].y = 0 as libc::c_int as libc::c_double;
        evs[0 as libc::c_int as usize].x = evs[0 as libc::c_int as usize].y;
        evs[1 as libc::c_int as usize].y = 0 as libc::c_int as libc::c_double;
        evs[1 as libc::c_int as usize].x = evs[1 as libc::c_int as usize].y;
        if mult == 1 as libc::c_int || Concentrate as libc::c_int != 0 {
            poly = (*trip).poly;
            j = 0 as libc::c_int;
            while j < poly.pn {
                (*medges.offset(j as isize)).a = *(poly.ps).offset(j as isize);
                (*medges.offset(j as isize))
                    .b = *(poly.ps).offset(((j + 1 as libc::c_int) % poly.pn) as isize);
                j += 1;
            }
            tweakPath(poly, s, t, pl);
            if Proutespline(medges, poly.pn, pl, evs.as_mut_ptr(), &mut spl)
                < 0 as libc::c_int
            {
                agerr(
                    AGWARN,
                    b"Could not create control points for multiple spline for edge (%s,%s)\n\0"
                        as *const u8 as *const libc::c_char,
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
                rv = 1 as libc::c_int;
            } else {
                finishEdge(
                    e,
                    spl,
                    ((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node != head) as libc::c_int,
                );
                free(medges as *mut libc::c_void);
                return 0 as libc::c_int;
            }
        } else {
            pn = 2 as libc::c_int * (pl.pn - 1 as libc::c_int);
            cpts = gcalloc(
                (pl.pn - 2 as libc::c_int) as size_t,
                ::std::mem::size_of::<*mut pointf>() as libc::c_ulong,
            ) as *mut *mut pointf;
            i = 0 as libc::c_int;
            loop {
                if !(i < pl.pn - 2 as libc::c_int) {
                    current_block = 11743904203796629665;
                    break;
                }
                let ref mut fresh33 = *cpts.offset(i as isize);
                *fresh33 = mkCtrlPts(
                    t,
                    mult + 1 as libc::c_int,
                    *(pl.ps).offset(i as isize),
                    *(pl.ps).offset((i + 1 as libc::c_int) as isize),
                    *(pl.ps).offset((i + 2 as libc::c_int) as isize),
                    trip,
                );
                if (*cpts.offset(i as isize)).is_null() {
                    agerr(
                        AGWARN,
                        b"Could not create control points for multiple spline for edge (%s,%s)\n\0"
                            as *const u8 as *const libc::c_char,
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
                    rv = 1 as libc::c_int;
                    current_block = 11715752260437999342;
                    break;
                } else {
                    i += 1;
                }
            }
            match current_block {
                11715752260437999342 => {}
                _ => {
                    poly
                        .ps = gcalloc(
                        pn as size_t,
                        ::std::mem::size_of::<pointf>() as libc::c_ulong,
                    ) as *mut pointf;
                    poly.pn = pn;
                    i = 0 as libc::c_int;
                    while i < mult {
                        *(poly.ps)
                            .offset(
                                0 as libc::c_int as isize,
                            ) = eps[0 as libc::c_int as usize];
                        j = 1 as libc::c_int;
                        while j < pl.pn - 1 as libc::c_int {
                            *(poly.ps)
                                .offset(
                                    j as isize,
                                ) = *(*cpts.offset((j - 1 as libc::c_int) as isize))
                                .offset(i as isize);
                            j += 1;
                        }
                        *(poly.ps)
                            .offset(
                                (pl.pn - 1 as libc::c_int) as isize,
                            ) = eps[1 as libc::c_int as usize];
                        j = 1 as libc::c_int;
                        while j < pl.pn - 1 as libc::c_int {
                            *(poly.ps)
                                .offset(
                                    (pn - j) as isize,
                                ) = *(*cpts.offset((j - 1 as libc::c_int) as isize))
                                .offset((i + 1 as libc::c_int) as isize);
                            j += 1;
                        }
                        if Pshortestpath(&mut poly, eps.as_mut_ptr(), &mut mmpl)
                            < 0 as libc::c_int
                        {
                            agerr(
                                AGWARN,
                                b"Could not create control points for multiple spline for edge (%s,%s)\n\0"
                                    as *const u8 as *const libc::c_char,
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
                            rv = 1 as libc::c_int;
                            break;
                        } else {
                            if doPolyline != 0 {
                                make_polyline(mmpl, &mut spl);
                            } else {
                                j = 0 as libc::c_int;
                                while j < poly.pn {
                                    (*medges.offset(j as isize))
                                        .a = *(poly.ps).offset(j as isize);
                                    (*medges.offset(j as isize))
                                        .b = *(poly.ps)
                                        .offset(((j + 1 as libc::c_int) % poly.pn) as isize);
                                    j += 1;
                                }
                                tweakPath(
                                    poly,
                                    0 as libc::c_int,
                                    pl.pn - 1 as libc::c_int,
                                    mmpl,
                                );
                                if Proutespline(
                                    medges,
                                    poly.pn,
                                    mmpl,
                                    evs.as_mut_ptr(),
                                    &mut spl,
                                ) < 0 as libc::c_int
                                {
                                    agerr(
                                        AGWARN,
                                        b"Could not create control points for multiple spline for edge (%s,%s)\n\0"
                                            as *const u8 as *const libc::c_char,
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
                                    rv = 1 as libc::c_int;
                                    break;
                                }
                            }
                            finishEdge(
                                e,
                                spl,
                                ((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                    == 2 as libc::c_int
                                {
                                    e
                                } else {
                                    e.offset(-(1 as libc::c_int as isize))
                                }))
                                    .node != head) as libc::c_int,
                            );
                            e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .to_virt;
                            i += 1;
                        }
                    }
                }
            }
        }
    }
    if !cpts.is_null() {
        i = 0 as libc::c_int;
        while i < pl.pn - 2 as libc::c_int {
            free(*cpts.offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        free(cpts as *mut libc::c_void);
    }
    free(medges as *mut libc::c_void);
    free(poly.ps as *mut libc::c_void);
    return rv;
}
unsafe extern "C" fn inCone(
    mut a: pointf,
    mut b: pointf,
    mut c: pointf,
    mut q: pointf,
) -> libc::c_int {
    return (area2(q, a, b) >= -0.0000000001f64 && area2(q, b, c) >= -0.0000000001f64)
        as libc::c_int;
}
static mut north: pointf = {
    let mut init = pointf_s {
        x: 0 as libc::c_int as libc::c_double,
        y: 1 as libc::c_int as libc::c_double,
    };
    init
};
static mut northeast: pointf = {
    let mut init = pointf_s {
        x: 1 as libc::c_int as libc::c_double,
        y: 1 as libc::c_int as libc::c_double,
    };
    init
};
static mut east: pointf = {
    let mut init = pointf_s {
        x: 1 as libc::c_int as libc::c_double,
        y: 0 as libc::c_int as libc::c_double,
    };
    init
};
static mut southeast: pointf = {
    let mut init = pointf_s {
        x: 1 as libc::c_int as libc::c_double,
        y: -(1 as libc::c_int) as libc::c_double,
    };
    init
};
static mut south: pointf = {
    let mut init = pointf_s {
        x: 0 as libc::c_int as libc::c_double,
        y: -(1 as libc::c_int) as libc::c_double,
    };
    init
};
static mut southwest: pointf = {
    let mut init = pointf_s {
        x: -(1 as libc::c_int) as libc::c_double,
        y: -(1 as libc::c_int) as libc::c_double,
    };
    init
};
static mut west: pointf = {
    let mut init = pointf_s {
        x: -(1 as libc::c_int) as libc::c_double,
        y: 0 as libc::c_int as libc::c_double,
    };
    init
};
static mut northwest: pointf = {
    let mut init = pointf_s {
        x: -(1 as libc::c_int) as libc::c_double,
        y: 1 as libc::c_int as libc::c_double,
    };
    init
};
unsafe extern "C" fn addEndpoint(
    mut rtr: *mut router_t,
    mut p: pointf,
    mut v: *mut node_t,
    mut v_id: libc::c_int,
    mut sides: libc::c_int,
) {
    let mut obs_id: libc::c_int = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .lim;
    let mut starti: libc::c_int = *((*rtr).obs).offset(obs_id as isize);
    let mut endi: libc::c_int = *((*rtr).obs)
        .offset((obs_id + 1 as libc::c_int) as isize);
    let mut pts: *mut pointf = (*rtr).ps;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut vr: pointf = pointf { x: 0., y: 0. };
    let mut v0: pointf = pointf { x: 0., y: 0. };
    let mut v1: pointf = pointf { x: 0., y: 0. };
    match sides {
        4 => {
            vr = add_pointf(p, north);
            v0 = add_pointf(p, northwest);
            v1 = add_pointf(p, northeast);
        }
        6 => {
            vr = add_pointf(p, northeast);
            v0 = add_pointf(p, north);
            v1 = add_pointf(p, east);
        }
        2 => {
            vr = add_pointf(p, east);
            v0 = add_pointf(p, northeast);
            v1 = add_pointf(p, southeast);
        }
        3 => {
            vr = add_pointf(p, southeast);
            v0 = add_pointf(p, east);
            v1 = add_pointf(p, south);
        }
        1 => {
            vr = add_pointf(p, south);
            v0 = add_pointf(p, southeast);
            v1 = add_pointf(p, southwest);
        }
        9 => {
            vr = add_pointf(p, southwest);
            v0 = add_pointf(p, south);
            v1 = add_pointf(p, west);
        }
        8 => {
            vr = add_pointf(p, west);
            v0 = add_pointf(p, southwest);
            v1 = add_pointf(p, northwest);
        }
        12 => {
            vr = add_pointf(p, northwest);
            v0 = add_pointf(p, west);
            v1 = add_pointf(p, north);
        }
        0 => {}
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"multispline.c\0" as *const u8 as *const libc::c_char,
                986 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void addEndpoint(router_t *, pointf, node_t *, int, int)\0"))
                    .as_ptr(),
            );
        }
    }
    (*((*(*rtr).tg).nodes).offset(v_id as isize)).ne = 0 as libc::c_int;
    (*((*(*rtr).tg).nodes).offset(v_id as isize)).ctr = p;
    i = starti;
    while i < endi {
        let mut seg: ipair = ipair { i: 0, j: 0 };
        seg.i = i;
        if i < endi - 1 as libc::c_int {
            seg.j = i + 1 as libc::c_int;
        } else {
            seg.j = starti;
        }
        t = findMap((*rtr).trimap, seg.i, seg.j);
        if !(sides != 0 && inCone(v0, p, v1, *pts.offset(seg.i as isize)) == 0
            && inCone(v0, p, v1, *pts.offset(seg.j as isize)) == 0
            && raySeg(p, vr, *pts.offset(seg.i as isize), *pts.offset(seg.j as isize))
                == 0)
        {
            d = sqrt(
                (p.x - (*((*(*rtr).tg).nodes).offset(t as isize)).ctr.x)
                    * (p.x - (*((*(*rtr).tg).nodes).offset(t as isize)).ctr.x)
                    + (p.y - (*((*(*rtr).tg).nodes).offset(t as isize)).ctr.y)
                        * (p.y - (*((*(*rtr).tg).nodes).offset(t as isize)).ctr.y),
            );
            addTriEdge((*rtr).tg, v_id, t, d, seg);
        }
        i += 1;
    }
}
unsafe extern "C" fn edgeToSeg(
    mut tg: *mut tgraph,
    mut i: libc::c_int,
    mut j: libc::c_int,
) -> ipair {
    let mut ip: ipair = {
        let mut init = ipair {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
        };
        init
    };
    let mut np: *mut tnode = ((*tg).nodes).offset(i as isize);
    let mut ep: *mut tedge = 0 as *mut tedge;
    i = 0 as libc::c_int;
    while i < (*np).ne {
        ep = ((*tg).edges).offset(*((*np).edges).offset(i as isize) as isize);
        if (*ep).t == j || (*ep).h == j {
            return (*ep).seg;
        }
        i += 1;
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"multispline.c\0" as *const u8 as *const libc::c_char,
        1027 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 36],
            &[libc::c_char; 36],
        >(b"ipair edgeToSeg(tgraph *, int, int)\0"))
            .as_ptr(),
    );
    return ip;
}
unsafe extern "C" fn freeTripoly(mut trip: *mut tripoly_t) {
    let mut i: libc::c_int = 0;
    let mut tp: *mut tri = 0 as *mut tri;
    let mut nxt: *mut tri = 0 as *mut tri;
    free((*trip).poly.ps as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < (*trip).poly.pn {
        tp = *((*trip).triMap).offset(i as isize);
        while !tp.is_null() {
            nxt = (*tp).nxttri;
            free(tp as *mut libc::c_void);
            tp = nxt;
        }
        i += 1;
    }
    free((*trip).triMap as *mut libc::c_void);
    free(trip as *mut libc::c_void);
}
unsafe extern "C" fn mkPoly(
    mut rtr: *mut router_t,
    mut dad: *mut libc::c_int,
    mut s: libc::c_int,
    mut t: libc::c_int,
    mut p_s: pointf,
    mut p_t: pointf,
    mut sx: *mut libc::c_int,
) -> *mut tripoly_t {
    let mut ps: *mut tripoly_t = 0 as *mut tripoly_t;
    let mut nxt: libc::c_int = 0;
    let mut p: ipair = ipair { i: 0, j: 0 };
    let mut nt: libc::c_int = 0 as libc::c_int;
    let mut side1: *mut side_t = 0 as *mut side_t;
    let mut side2: *mut side_t = 0 as *mut side_t;
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut cnt1: libc::c_int = 0 as libc::c_int;
    let mut cnt2: libc::c_int = 0 as libc::c_int;
    let mut pts: *mut pointf = 0 as *mut pointf;
    let mut pps: *mut pointf = 0 as *mut pointf;
    let mut vmap: *mut Dt_t = 0 as *mut Dt_t;
    let mut trim: *mut *mut tri = 0 as *mut *mut tri;
    nxt = *dad.offset(t as isize);
    while nxt != s {
        nt += 1;
        nxt = *dad.offset(nxt as isize);
    }
    side1 = gcalloc(
        (nt + 4 as libc::c_int) as size_t,
        ::std::mem::size_of::<side_t>() as libc::c_ulong,
    ) as *mut side_t;
    side2 = gcalloc(
        (nt + 4 as libc::c_int) as size_t,
        ::std::mem::size_of::<side_t>() as libc::c_ulong,
    ) as *mut side_t;
    nxt = *dad.offset(t as isize);
    p = edgeToSeg((*rtr).tg, nxt, t);
    let ref mut fresh34 = (*side1.offset(cnt1 as isize)).ts;
    *fresh34 = addTri(-(1 as libc::c_int), p.j, 0 as *mut tri);
    let fresh35 = cnt1;
    cnt1 = cnt1 + 1;
    (*side1.offset(fresh35 as isize)).v = p.i;
    let ref mut fresh36 = (*side2.offset(cnt2 as isize)).ts;
    *fresh36 = addTri(-(1 as libc::c_int), p.i, 0 as *mut tri);
    let fresh37 = cnt2;
    cnt2 = cnt2 + 1;
    (*side2.offset(fresh37 as isize)).v = p.j;
    t = nxt;
    nxt = *dad.offset(t as isize);
    while nxt >= 0 as libc::c_int {
        p = edgeToSeg((*rtr).tg, t, nxt);
        if p.i == (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v {
            let ref mut fresh38 = (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts;
            *fresh38 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                p.j,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh39 = (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts;
            *fresh39 = addTri(
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                p.j,
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh40 = (*side2.offset(cnt2 as isize)).ts;
            *fresh40 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                0 as *mut tri,
            );
            let fresh41 = cnt2;
            cnt2 = cnt2 + 1;
            (*side2.offset(fresh41 as isize)).v = p.j;
        } else if p.i == (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v {
            let ref mut fresh42 = (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts;
            *fresh42 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                p.j,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh43 = (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts;
            *fresh43 = addTri(
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                p.j,
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh44 = (*side1.offset(cnt1 as isize)).ts;
            *fresh44 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                0 as *mut tri,
            );
            let fresh45 = cnt1;
            cnt1 = cnt1 + 1;
            (*side1.offset(fresh45 as isize)).v = p.j;
        } else if p.j == (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v {
            let ref mut fresh46 = (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts;
            *fresh46 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                p.i,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh47 = (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts;
            *fresh47 = addTri(
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                p.i,
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh48 = (*side2.offset(cnt2 as isize)).ts;
            *fresh48 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                0 as *mut tri,
            );
            let fresh49 = cnt2;
            cnt2 = cnt2 + 1;
            (*side2.offset(fresh49 as isize)).v = p.i;
        } else {
            let ref mut fresh50 = (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts;
            *fresh50 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                p.i,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh51 = (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts;
            *fresh51 = addTri(
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                p.i,
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts,
            );
            let ref mut fresh52 = (*side1.offset(cnt1 as isize)).ts;
            *fresh52 = addTri(
                (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
                (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
                0 as *mut tri,
            );
            let fresh53 = cnt1;
            cnt1 = cnt1 + 1;
            (*side1.offset(fresh53 as isize)).v = p.i;
        }
        t = nxt;
        nxt = *dad.offset(nxt as isize);
    }
    let ref mut fresh54 = (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts;
    *fresh54 = addTri(
        -(2 as libc::c_int),
        (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).v,
        (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).ts,
    );
    let ref mut fresh55 = (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts;
    *fresh55 = addTri(
        -(2 as libc::c_int),
        (*side1.offset((cnt1 - 1 as libc::c_int) as isize)).v,
        (*side2.offset((cnt2 - 1 as libc::c_int) as isize)).ts,
    );
    vmap = dtopen(&mut ipairdisc, Dtoset);
    vmapAdd(vmap, -(1 as libc::c_int), 0 as libc::c_int);
    vmapAdd(vmap, -(2 as libc::c_int), cnt1 + 1 as libc::c_int);
    pts = gcalloc(
        (nt + 4 as libc::c_int) as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    pps = pts;
    trim = gcalloc(
        (nt + 4 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut tri>() as libc::c_ulong,
    ) as *mut *mut tri;
    let fresh56 = pps;
    pps = pps.offset(1);
    *fresh56 = p_t;
    idx = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < cnt1 {
        vmapAdd(vmap, (*side1.offset(i as isize)).v, idx);
        let fresh57 = pps;
        pps = pps.offset(1);
        *fresh57 = *((*rtr).ps).offset((*side1.offset(i as isize)).v as isize);
        let fresh58 = idx;
        idx = idx + 1;
        let ref mut fresh59 = *trim.offset(fresh58 as isize);
        *fresh59 = (*side1.offset(i as isize)).ts;
        i += 1;
    }
    let fresh60 = pps;
    pps = pps.offset(1);
    *fresh60 = p_s;
    idx += 1;
    i = cnt2 - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        vmapAdd(vmap, (*side2.offset(i as isize)).v, idx);
        let fresh61 = pps;
        pps = pps.offset(1);
        *fresh61 = *((*rtr).ps).offset((*side2.offset(i as isize)).v as isize);
        let fresh62 = idx;
        idx = idx + 1;
        let ref mut fresh63 = *trim.offset(fresh62 as isize);
        *fresh63 = (*side2.offset(i as isize)).ts;
        i -= 1;
    }
    i = 0 as libc::c_int;
    while i < nt + 4 as libc::c_int {
        mapTri(vmap, *trim.offset(i as isize));
        i += 1;
    }
    ps = zmalloc(::std::mem::size_of::<tripoly_t>() as libc::c_ulong) as *mut tripoly_t;
    (*ps).poly.pn = nt + 4 as libc::c_int;
    let ref mut fresh64 = (*ps).poly.ps;
    *fresh64 = pts;
    let ref mut fresh65 = (*ps).triMap;
    *fresh65 = trim;
    free(side1 as *mut libc::c_void);
    free(side2 as *mut libc::c_void);
    dtclose(vmap);
    *sx = cnt1 + 1 as libc::c_int;
    return ps;
}
unsafe extern "C" fn resetGraph(
    mut g: *mut tgraph,
    mut ncnt: libc::c_int,
    mut ecnt: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut np: *mut tnode = (*g).nodes;
    (*g).nedges = ecnt;
    i = 0 as libc::c_int;
    while i < ncnt {
        if ((*np).edges).offset((*np).ne as isize)
            == (*np.offset(1 as libc::c_int as isize)).edges
        {
            let ref mut fresh66 = (*np).ne;
            *fresh66 -= 1;
        }
        np = np.offset(1);
        i += 1;
    }
}
unsafe extern "C" fn PQgen(
    mut pq: *mut PQ,
    mut sz: libc::c_int,
    mut guard: libc::c_int,
) {
    let ref mut fresh67 = (*pq).pq;
    *fresh67 = gcalloc(
        (sz + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *((*pq).pq).offset(0 as libc::c_int as isize) = guard;
    (*pq).PQsize = sz;
    (*pq).PQcnt = 0 as libc::c_int;
}
unsafe extern "C" fn PQfree(mut pq: *mut PQ, mut freeAll: libc::c_int) {
    free((*pq).pq as *mut libc::c_void);
    if freeAll != 0 {
        free(pq as *mut libc::c_void);
    }
}
unsafe extern "C" fn PQinit(mut pq: *mut PQ) {
    (*pq).PQcnt = 0 as libc::c_int;
}
unsafe extern "C" fn PQupheap(mut ppq: *mut PQ, mut k: libc::c_int) {
    let mut pq: *mut libc::c_int = (*ppq).pq;
    let mut x: libc::c_int = *pq.offset(k as isize);
    let mut v: libc::c_float = *((*(ppq as *mut PPQ)).vals).offset(x as isize);
    let mut next: libc::c_int = k / 2 as libc::c_int;
    let mut n: libc::c_int = 0;
    loop {
        n = *pq.offset(next as isize);
        if !(*((*(ppq as *mut PPQ)).vals).offset(n as isize) < v) {
            break;
        }
        *pq.offset(k as isize) = n;
        *((*(ppq as *mut PPQ)).idxs).offset(n as isize) = k;
        k = next;
        next /= 2 as libc::c_int;
    }
    *pq.offset(k as isize) = x;
    *((*(ppq as *mut PPQ)).idxs).offset(x as isize) = k;
}
unsafe extern "C" fn PQinsert(mut pq: *mut PQ, mut np: libc::c_int) -> libc::c_int {
    if (*pq).PQcnt == (*pq).PQsize {
        agerr(AGERR, b"Heap overflow\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let ref mut fresh68 = (*pq).PQcnt;
    *fresh68 += 1;
    *((*pq).pq).offset((*pq).PQcnt as isize) = np;
    PQupheap(pq, (*pq).PQcnt);
    return 0 as libc::c_int;
}
unsafe extern "C" fn PQdownheap(mut ppq: *mut PQ, mut k: libc::c_int) {
    let mut pq: *mut libc::c_int = (*ppq).pq;
    let mut x: libc::c_int = *pq.offset(k as isize);
    let mut v: libc::c_float = *((*(ppq as *mut PPQ)).vals).offset(x as isize);
    let mut lim: libc::c_int = (*ppq).PQcnt / 2 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    while k <= lim {
        j = k + k;
        n = *pq.offset(j as isize);
        if j < (*ppq).PQcnt {
            if *((*(ppq as *mut PPQ)).vals).offset(n as isize)
                < *((*(ppq as *mut PPQ)).vals)
                    .offset(*pq.offset((j + 1 as libc::c_int) as isize) as isize)
            {
                j += 1;
                n = *pq.offset(j as isize);
            }
        }
        if v >= *((*(ppq as *mut PPQ)).vals).offset(n as isize) {
            break;
        }
        *pq.offset(k as isize) = n;
        *((*(ppq as *mut PPQ)).idxs).offset(n as isize) = k;
        k = j;
    }
    *pq.offset(k as isize) = x;
    *((*(ppq as *mut PPQ)).idxs).offset(x as isize) = k;
}
unsafe extern "C" fn PQremove(mut pq: *mut PQ) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if (*pq).PQcnt != 0 {
        n = *((*pq).pq).offset(1 as libc::c_int as isize);
        *((*pq).pq)
            .offset(
                1 as libc::c_int as isize,
            ) = *((*pq).pq).offset((*pq).PQcnt as isize);
        let ref mut fresh69 = (*pq).PQcnt;
        *fresh69 -= 1;
        if (*pq).PQcnt != 0 {
            PQdownheap(pq, 1 as libc::c_int);
        }
        return n;
    } else {
        return *((*pq).pq).offset(0 as libc::c_int as isize)
    };
}
unsafe extern "C" fn PQupdate(
    mut pq: *mut PQ,
    mut n: libc::c_int,
    mut d: libc::c_float,
) {
    *((*(pq as *mut PPQ)).vals).offset(n as isize) = d;
    PQupheap(pq, *((*(pq as *mut PPQ)).idxs).offset(n as isize));
}
unsafe extern "C" fn triPath(
    mut g: *mut tgraph,
    mut n: libc::c_int,
    mut v0: libc::c_int,
    mut v1: libc::c_int,
    mut pq: *mut PQ,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut adjn: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut np: *mut tnode = 0 as *mut tnode;
    let mut e: *mut tedge = 0 as *mut tedge;
    let mut dad: *mut libc::c_int = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*pq).PQsize {
        *((*(pq as *mut PPQ)).vals).offset(i as isize) = -3.40282347e+38f32;
        i += 1;
    }
    PQinit(pq);
    *dad.offset(v0 as isize) = -(1 as libc::c_int);
    *((*(pq as *mut PPQ)).vals).offset(v0 as isize) = 0 as libc::c_int as libc::c_float;
    if PQinsert(pq, v0) != 0 {
        return 0 as *mut libc::c_int;
    }
    loop {
        i = PQremove(pq);
        if !(i != -(1 as libc::c_int)) {
            break;
        }
        *((*(pq as *mut PPQ)).vals).offset(i as isize)
            *= -(1 as libc::c_int) as libc::c_float;
        if i == v1 {
            break;
        }
        np = ((*g).nodes).offset(i as isize);
        j = 0 as libc::c_int;
        while j < (*np).ne {
            e = ((*g).edges).offset(*((*np).edges).offset(j as isize) as isize);
            if (*e).t == i {
                adjn = (*e).h;
            } else {
                adjn = (*e).t;
            }
            if *((*(pq as *mut PPQ)).vals).offset(adjn as isize)
                < 0 as libc::c_int as libc::c_float
            {
                d = -(*((*(pq as *mut PPQ)).vals).offset(i as isize) as libc::c_double
                    + (*e).dist);
                if *((*(pq as *mut PPQ)).vals).offset(adjn as isize)
                    == -3.40282347e+38f32
                {
                    *((*(pq as *mut PPQ)).vals)
                        .offset(adjn as isize) = d as libc::c_float;
                    *dad.offset(adjn as isize) = i;
                    if PQinsert(pq, adjn) != 0 {
                        return 0 as *mut libc::c_int;
                    }
                } else if (*((*(pq as *mut PPQ)).vals).offset(adjn as isize)
                        as libc::c_double) < d
                    {
                    PQupdate(pq, adjn, d as libc::c_float);
                    *dad.offset(adjn as isize) = i;
                }
            }
            j += 1;
        }
    }
    return dad;
}
#[no_mangle]
pub unsafe extern "C" fn makeMultiSpline(
    mut e: *mut edge_t,
    mut rtr: *mut router_t,
    mut doPolyline: libc::c_int,
) -> libc::c_int {
    let mut line: Ppolyline_t = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .path;
    let mut t: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    let mut h: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let mut t_p: pointf = *(line.ps).offset(0 as libc::c_int as isize);
    let mut h_p: pointf = *(line.ps).offset((line.pn - 1 as libc::c_int) as isize);
    let mut poly: *mut tripoly_t = 0 as *mut tripoly_t;
    let mut idx: libc::c_int = 0;
    let mut sp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t_id: libc::c_int = (*rtr).tn;
    let mut h_id: libc::c_int = (*rtr).tn + 1 as libc::c_int;
    let mut ecnt: libc::c_int = (*(*rtr).tg).nedges;
    let mut pq: PPQ = PPQ {
        pq: PQ {
            pq: 0 as *mut libc::c_int,
            PQcnt: 0,
            PQsize: 0,
        },
        vals: 0 as *mut libc::c_float,
        idxs: 0 as *mut libc::c_int,
    };
    let mut idxs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vals: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut ret: libc::c_int = 0;
    addEndpoint(
        rtr,
        t_p,
        t,
        t_id,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int,
    );
    addEndpoint(
        rtr,
        h_p,
        h,
        h_id,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
            as libc::c_int,
    );
    PQgen(&mut pq.pq, (*rtr).tn + 2 as libc::c_int, -(1 as libc::c_int));
    idxs = gcalloc(
        (pq.pq.PQsize + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    vals = gcalloc(
        (pq.pq.PQsize + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    *vals.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
    pq.vals = vals.offset(1 as libc::c_int as isize);
    pq.idxs = idxs.offset(1 as libc::c_int as isize);
    sp = triPath(
        (*rtr).tg,
        (*rtr).tn + 2 as libc::c_int,
        h_id,
        t_id,
        &mut pq as *mut PPQ as *mut PQ,
    );
    free(vals as *mut libc::c_void);
    free(idxs as *mut libc::c_void);
    PQfree(&mut pq.pq, 0 as libc::c_int);
    if !sp.is_null() {
        poly = mkPoly(rtr, sp, h_id, t_id, h_p, t_p, &mut idx);
        free(sp as *mut libc::c_void);
        ret = genroute(poly, 0 as libc::c_int, idx, e, doPolyline);
        freeTripoly(poly);
    } else {
        ret = -(1 as libc::c_int);
    }
    resetGraph((*rtr).tg, (*rtr).tn, ecnt);
    return ret;
}
unsafe extern "C" fn run_static_initializers() {
    itemdisc = {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut item,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
                >,
                Dtmake_f,
            >(
                Some(
                    newItem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut item,
                            *mut Dtdisc_t,
                        ) -> *mut libc::c_void,
                ),
            ),
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut item, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    freeItem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut item,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
                >,
                Dtcompar_f,
            >(
                Some(
                    cmpItem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut libc::c_int,
                            *mut libc::c_int,
                            *mut Dtdisc_t,
                        ) -> libc::c_int,
                ),
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
