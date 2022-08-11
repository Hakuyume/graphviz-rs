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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn compute_bb(_: *mut Agraph_t);
    fn coord(n: *mut node_t) -> pointf;
    fn dotneato_postprocess(_: *mut Agraph_t);
    fn newPS() -> *mut PointSet;
    fn freePS(_: *mut PointSet);
    fn insertPS(_: *mut PointSet, _: point);
    fn addPS(_: *mut PointSet, _: libc::c_int, _: libc::c_int);
    fn inPS(_: *mut PointSet, _: point) -> libc::c_int;
    fn sizeOf(_: *mut PointSet) -> libc::c_int;
    fn pointsOf(_: *mut PointSet) -> *mut point;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type pack_mode = libc::c_uint;
pub const l_aspect: pack_mode = 5;
pub const l_array: pack_mode = 4;
pub const l_graph: pack_mode = 3;
pub const l_node: pack_mode = 2;
pub const l_clust: pack_mode = 1;
pub const l_undef: pack_mode = 0;
pub type packval_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pack_info {
    pub aspect: libc::c_float,
    pub sz: libc::c_int,
    pub margin: libc::c_uint,
    pub doSplines: libc::c_int,
    pub mode: pack_mode,
    pub fixed: *mut bool,
    pub vals: *mut packval_t,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ainfo {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub index: libc::c_int,
}
pub type PointSet = Dict_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ginfo {
    pub perim: libc::c_int,
    pub cells: *mut point,
    pub nc: libc::c_int,
    pub index: libc::c_int,
}
#[inline]
unsafe extern "C" fn add_point(mut p: point, mut q: point) -> point {
    let mut r: point = point { x: 0, y: 0 };
    r.x = p.x + q.x;
    r.y = p.y + q.y;
    return r;
}
#[inline]
unsafe extern "C" fn sub_point(mut p: point, mut q: point) -> point {
    let mut r: point = point { x: 0, y: 0 };
    r.x = p.x - q.x;
    r.y = p.y - q.y;
    return r;
}
unsafe extern "C" fn computeStep(
    mut ng: libc::c_int,
    mut bbs: *mut boxf,
    mut margin: libc::c_uint,
) -> libc::c_int {
    let mut l1: libc::c_double = 0.;
    let mut l2: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut W: libc::c_double = 0.;
    let mut H: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut root: libc::c_int = 0;
    a = (100 as libc::c_int * ng - 1 as libc::c_int) as libc::c_double;
    c = 0 as libc::c_int as libc::c_double;
    b = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < ng {
        let mut bb: boxf = *bbs.offset(i as isize);
        W = bb.UR.x - bb.LL.x
            + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin) as libc::c_double;
        H = bb.UR.y - bb.LL.y
            + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin) as libc::c_double;
        b -= W + H;
        c -= W * H;
        i += 1;
    }
    d = b * b - 4.0f64 * a * c;
    if d < 0 as libc::c_int as libc::c_double {
        agerr(
            AGERR,
            b"libpack: disc = %f ( < 0)\n\0" as *const u8 as *const libc::c_char,
            d,
        );
        return -(1 as libc::c_int);
    }
    r = sqrt(d);
    l1 = (-b + r) / (2 as libc::c_int as libc::c_double * a);
    l2 = (-b - r) / (2 as libc::c_int as libc::c_double * a);
    root = l1 as libc::c_int;
    if root == 0 as libc::c_int {
        root = 1 as libc::c_int;
    }
    if Verbose as libc::c_int > 2 as libc::c_int {
        fprintf(
            stderr,
            b"Packing: compute grid size\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"a %f b %f c %f d %f r %f\n\0" as *const u8 as *const libc::c_char,
            a,
            b,
            c,
            d,
            r,
        );
        fprintf(
            stderr,
            b"root %d (%f) %d (%f)\n\0" as *const u8 as *const libc::c_char,
            root,
            l1,
            l2 as libc::c_int,
            l2,
        );
        fprintf(
            stderr,
            b" r1 %f r2 %f\n\0" as *const u8 as *const libc::c_char,
            a * l1 * l1 + b * l1 + c,
            a * l2 * l2 + b * l2 + c,
        );
    }
    return root;
}
unsafe extern "C" fn cmpf(
    mut X: *const libc::c_void,
    mut Y: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const ginfo = *(X as *const *mut ginfo);
    let mut y: *const ginfo = *(Y as *const *mut ginfo);
    return (*y).perim - (*x).perim;
}
unsafe extern "C" fn fillLine(mut p: pointf, mut q: pointf, mut ps: *mut PointSet) {
    let mut x1: libc::c_int = if p.x >= 0 as libc::c_int as libc::c_double {
        (p.x + 0.5f64) as libc::c_int
    } else {
        (p.x - 0.5f64) as libc::c_int
    };
    let mut y1: libc::c_int = if p.y >= 0 as libc::c_int as libc::c_double {
        (p.y + 0.5f64) as libc::c_int
    } else {
        (p.y - 0.5f64) as libc::c_int
    };
    let mut x2: libc::c_int = if q.x >= 0 as libc::c_int as libc::c_double {
        (q.x + 0.5f64) as libc::c_int
    } else {
        (q.x - 0.5f64) as libc::c_int
    };
    let mut y2: libc::c_int = if q.y >= 0 as libc::c_int as libc::c_double {
        (q.y + 0.5f64) as libc::c_int
    } else {
        (q.y - 0.5f64) as libc::c_int
    };
    let mut d: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ax: libc::c_int = 0;
    let mut ay: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    dx = x2 - x1;
    ax = abs(dx) << 1 as libc::c_int;
    sx = if dx < 0 as libc::c_int { -(1 as libc::c_int) } else { 1 as libc::c_int };
    dy = y2 - y1;
    ay = abs(dy) << 1 as libc::c_int;
    sy = if dy < 0 as libc::c_int { -(1 as libc::c_int) } else { 1 as libc::c_int };
    x = x1;
    y = y1;
    if ax > ay {
        d = ay - (ax >> 1 as libc::c_int);
        loop {
            addPS(ps, x, y);
            if x == x2 {
                return;
            }
            if d >= 0 as libc::c_int {
                y += sy;
                d -= ax;
            }
            x += sx;
            d += ay;
        }
    } else {
        d = ax - (ay >> 1 as libc::c_int);
        loop {
            addPS(ps, x, y);
            if y == y2 {
                return;
            }
            if d >= 0 as libc::c_int {
                x += sx;
                d -= ay;
            }
            y += sy;
            d += ax;
        }
    };
}
unsafe extern "C" fn fillEdge(
    mut e: *mut Agedge_t,
    mut p: point,
    mut ps: *mut PointSet,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut ssize: libc::c_int,
    mut doS: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut pt: pointf = pointf { x: 0., y: 0. };
    let mut hpt: pointf = pointf { x: 0., y: 0. };
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    pt.x = p.x as libc::c_double;
    pt.y = p.y as libc::c_double;
    if doS == 0 || ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
    {
        h = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        hpt = coord(h);
        hpt.x += dx as libc::c_double;
        hpt.y += dy as libc::c_double;
        hpt
            .x = (if hpt.x >= 0 as libc::c_int as libc::c_double {
            hpt.x / ssize as libc::c_double
        } else {
            (hpt.x + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                - 1 as libc::c_int as libc::c_double
        });
        hpt
            .y = (if hpt.y >= 0 as libc::c_int as libc::c_double {
            hpt.y / ssize as libc::c_double
        } else {
            (hpt.y + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                - 1 as libc::c_int as libc::c_double
        });
        fillLine(pt, hpt, ps);
        return;
    }
    j = 0 as libc::c_int;
    while j < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
        bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
            .offset(j as isize);
        if bz.sflag != 0 {
            pt = bz.sp;
            hpt = *(bz.list).offset(0 as libc::c_int as isize);
            k = 1 as libc::c_int;
        } else {
            pt = *(bz.list).offset(0 as libc::c_int as isize);
            hpt = *(bz.list).offset(1 as libc::c_int as isize);
            k = 2 as libc::c_int;
        }
        pt.x += dx as libc::c_double;
        pt.y += dy as libc::c_double;
        pt
            .x = (if pt.x >= 0 as libc::c_int as libc::c_double {
            pt.x / ssize as libc::c_double
        } else {
            (pt.x + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                - 1 as libc::c_int as libc::c_double
        });
        pt
            .y = (if pt.y >= 0 as libc::c_int as libc::c_double {
            pt.y / ssize as libc::c_double
        } else {
            (pt.y + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                - 1 as libc::c_int as libc::c_double
        });
        hpt.x += dx as libc::c_double;
        hpt.y += dy as libc::c_double;
        hpt
            .x = (if hpt.x >= 0 as libc::c_int as libc::c_double {
            hpt.x / ssize as libc::c_double
        } else {
            (hpt.x + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                - 1 as libc::c_int as libc::c_double
        });
        hpt
            .y = (if hpt.y >= 0 as libc::c_int as libc::c_double {
            hpt.y / ssize as libc::c_double
        } else {
            (hpt.y + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                - 1 as libc::c_int as libc::c_double
        });
        fillLine(pt, hpt, ps);
        while k < bz.size {
            pt = hpt;
            hpt = *(bz.list).offset(k as isize);
            hpt.x += dx as libc::c_double;
            hpt.y += dy as libc::c_double;
            hpt
                .x = (if hpt.x >= 0 as libc::c_int as libc::c_double {
                hpt.x / ssize as libc::c_double
            } else {
                (hpt.x + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                    - 1 as libc::c_int as libc::c_double
            });
            hpt
                .y = (if hpt.y >= 0 as libc::c_int as libc::c_double {
                hpt.y / ssize as libc::c_double
            } else {
                (hpt.y + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                    - 1 as libc::c_int as libc::c_double
            });
            fillLine(pt, hpt, ps);
            k += 1;
        }
        if bz.eflag != 0 {
            pt = hpt;
            hpt = bz.ep;
            hpt.x += dx as libc::c_double;
            hpt.y += dy as libc::c_double;
            hpt
                .x = (if hpt.x >= 0 as libc::c_int as libc::c_double {
                hpt.x / ssize as libc::c_double
            } else {
                (hpt.x + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                    - 1 as libc::c_int as libc::c_double
            });
            hpt
                .y = (if hpt.y >= 0 as libc::c_int as libc::c_double {
                hpt.y / ssize as libc::c_double
            } else {
                (hpt.y + 1 as libc::c_int as libc::c_double) / ssize as libc::c_double
                    - 1 as libc::c_int as libc::c_double
            });
            fillLine(pt, hpt, ps);
        }
        j += 1;
    }
}
unsafe extern "C" fn genBox(
    mut bb0: boxf,
    mut info: *mut ginfo,
    mut ssize: libc::c_int,
    mut margin: libc::c_uint,
    mut center: point,
    mut s: *mut libc::c_char,
) {
    let mut ps: *mut PointSet = 0 as *mut PointSet;
    let mut W: libc::c_int = 0;
    let mut H: libc::c_int = 0;
    let mut UR: point = point { x: 0, y: 0 };
    let mut LL: point = point { x: 0, y: 0 };
    let mut bb: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    bb
        .LL
        .x = (if bb0.LL.x >= 0 as libc::c_int as libc::c_double {
        (bb0.LL.x + 0.5f64) as libc::c_int
    } else {
        (bb0.LL.x - 0.5f64) as libc::c_int
    });
    bb
        .LL
        .y = (if bb0.LL.y >= 0 as libc::c_int as libc::c_double {
        (bb0.LL.y + 0.5f64) as libc::c_int
    } else {
        (bb0.LL.y - 0.5f64) as libc::c_int
    });
    bb
        .UR
        .x = (if bb0.UR.x >= 0 as libc::c_int as libc::c_double {
        (bb0.UR.x + 0.5f64) as libc::c_int
    } else {
        (bb0.UR.x - 0.5f64) as libc::c_int
    });
    bb
        .UR
        .y = (if bb0.UR.y >= 0 as libc::c_int as libc::c_double {
        (bb0.UR.y + 0.5f64) as libc::c_int
    } else {
        (bb0.UR.y - 0.5f64) as libc::c_int
    });
    ps = newPS();
    LL.x = (center.x as libc::c_uint).wrapping_sub(margin) as libc::c_int;
    LL.y = (center.y as libc::c_uint).wrapping_sub(margin) as libc::c_int;
    UR
        .x = ((center.x + bb.UR.x - bb.LL.x) as libc::c_uint).wrapping_add(margin)
        as libc::c_int;
    UR
        .y = ((center.y + bb.UR.y - bb.LL.y) as libc::c_uint).wrapping_add(margin)
        as libc::c_int;
    LL
        .x = (if LL.x >= 0 as libc::c_int {
        LL.x / ssize
    } else {
        (LL.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
    });
    LL
        .y = (if LL.y >= 0 as libc::c_int {
        LL.y / ssize
    } else {
        (LL.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
    });
    UR
        .x = (if UR.x >= 0 as libc::c_int {
        UR.x / ssize
    } else {
        (UR.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
    });
    UR
        .y = (if UR.y >= 0 as libc::c_int {
        UR.y / ssize
    } else {
        (UR.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
    });
    x = LL.x;
    while x <= UR.x {
        y = LL.y;
        while y <= UR.y {
            addPS(ps, x, y);
            y += 1;
        }
        x += 1;
    }
    let ref mut fresh0 = (*info).cells;
    *fresh0 = pointsOf(ps);
    (*info).nc = sizeOf(ps);
    W = ceil(
        (bb0.UR.x - bb0.LL.x
            + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin) as libc::c_double)
            / ssize as libc::c_double,
    ) as libc::c_int;
    H = ceil(
        (bb0.UR.y - bb0.LL.y
            + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin) as libc::c_double)
            / ssize as libc::c_double,
    ) as libc::c_int;
    (*info).perim = W + H;
    if Verbose as libc::c_int > 2 as libc::c_int {
        let mut i: libc::c_int = 0;
        fprintf(
            stderr,
            b"%s no. cells %d W %d H %d\n\0" as *const u8 as *const libc::c_char,
            s,
            (*info).nc,
            W,
            H,
        );
        i = 0 as libc::c_int;
        while i < (*info).nc {
            fprintf(
                stderr,
                b"  %d %d cell\n\0" as *const u8 as *const libc::c_char,
                (*((*info).cells).offset(i as isize)).x,
                (*((*info).cells).offset(i as isize)).y,
            );
            i += 1;
        }
    }
    freePS(ps);
}
unsafe extern "C" fn genPoly(
    mut root: *mut Agraph_t,
    mut g: *mut Agraph_t,
    mut info: *mut ginfo,
    mut ssize: libc::c_int,
    mut pinfo: *mut pack_info,
    mut center: point,
) -> libc::c_int {
    let mut ps: *mut PointSet = 0 as *mut PointSet;
    let mut W: libc::c_int = 0;
    let mut H: libc::c_int = 0;
    let mut LL: point = point { x: 0, y: 0 };
    let mut UR: point = point { x: 0, y: 0 };
    let mut pt: point = point { x: 0, y: 0 };
    let mut s2: point = point { x: 0, y: 0 };
    let mut ptf: pointf = pointf { x: 0., y: 0. };
    let mut eg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut margin: libc::c_uint = (*pinfo).margin;
    let mut doSplines: libc::c_int = (*pinfo).doSplines;
    let mut bb: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    if !root.is_null() {
        eg = root;
    } else {
        eg = g;
    }
    ps = newPS();
    dx = center.x
        - (if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
            >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x + 0.5f64)
                as libc::c_int
        } else {
            ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x - 0.5f64)
                as libc::c_int
        });
    dy = center.y
        - (if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
            >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y + 0.5f64)
                as libc::c_int
        } else {
            ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y - 0.5f64)
                as libc::c_int
        });
    if (*pinfo).mode as libc::c_uint == l_clust as libc::c_int as libc::c_uint {
        let mut i: libc::c_int = 0;
        let mut alg: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
        alg = gcalloc(
            agnnodes(g) as size_t,
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        ) as *mut *mut libc::c_void;
        i = 0 as libc::c_int;
        n = agfstnode(g);
        while !n.is_null() {
            let fresh1 = i;
            i = i + 1;
            let ref mut fresh2 = *alg.offset(fresh1 as isize);
            *fresh2 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg;
            let ref mut fresh3 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .alg;
            *fresh3 = 0 as *mut libc::c_void;
            n = agnxtnode(g, n);
        }
        i = 1 as libc::c_int;
        while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
            subg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(i as isize);
            bb
                .LL
                .x = (if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                    + 0.5f64) as libc::c_int
            } else {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                    - 0.5f64) as libc::c_int
            });
            bb
                .LL
                .y = (if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
                    + 0.5f64) as libc::c_int
            } else {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
                    - 0.5f64) as libc::c_int
            });
            bb
                .UR
                .x = (if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                    + 0.5f64) as libc::c_int
            } else {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                    - 0.5f64) as libc::c_int
            });
            bb
                .UR
                .y = (if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
                    + 0.5f64) as libc::c_int
            } else {
                ((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
                    - 0.5f64) as libc::c_int
            });
            if bb.UR.x > bb.LL.x && bb.UR.y > bb.LL.y {
                bb.LL.x += dx;
                bb.LL.y += dy;
                bb.UR.x += dx;
                bb.UR.y += dy;
                bb
                    .LL
                    .x = (bb.LL.x as libc::c_uint).wrapping_sub(margin) as libc::c_int
                    as libc::c_int;
                bb
                    .LL
                    .y = (bb.LL.y as libc::c_uint).wrapping_sub(margin) as libc::c_int
                    as libc::c_int;
                bb
                    .UR
                    .x = (bb.UR.x as libc::c_uint).wrapping_add(margin) as libc::c_int
                    as libc::c_int;
                bb
                    .UR
                    .y = (bb.UR.y as libc::c_uint).wrapping_add(margin) as libc::c_int
                    as libc::c_int;
                bb
                    .LL
                    .x = (if bb.LL.x >= 0 as libc::c_int {
                    bb.LL.x / ssize
                } else {
                    (bb.LL.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                bb
                    .LL
                    .y = (if bb.LL.y >= 0 as libc::c_int {
                    bb.LL.y / ssize
                } else {
                    (bb.LL.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                bb
                    .UR
                    .x = (if bb.UR.x >= 0 as libc::c_int {
                    bb.UR.x / ssize
                } else {
                    (bb.UR.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                bb
                    .UR
                    .y = (if bb.UR.y >= 0 as libc::c_int {
                    bb.UR.y / ssize
                } else {
                    (bb.UR.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                x = bb.LL.x;
                while x <= bb.UR.x {
                    y = bb.LL.y;
                    while y <= bb.UR.y {
                        addPS(ps, x, y);
                        y += 1;
                    }
                    x += 1;
                }
                n = agfstnode(subg);
                while !n.is_null() {
                    let ref mut fresh4 = (*((*(n as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .clust;
                    *fresh4 = subg;
                    n = agnxtnode(subg, n);
                }
            }
            i += 1;
        }
        n = agfstnode(g);
        while !n.is_null() {
            ptf = coord(n);
            pt
                .x = (if ptf.x >= 0 as libc::c_int as libc::c_double {
                (ptf.x + 0.5f64) as libc::c_int
            } else {
                (ptf.x - 0.5f64) as libc::c_int
            });
            pt
                .y = (if ptf.y >= 0 as libc::c_int as libc::c_double {
                (ptf.y + 0.5f64) as libc::c_int
            } else {
                (ptf.y - 0.5f64) as libc::c_int
            });
            pt.x += dx;
            pt.y += dy;
            if ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null() {
                s2
                    .x = (margin as libc::c_double
                    + ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
                        / 2 as libc::c_int as libc::c_double) as libc::c_int;
                s2
                    .y = (margin as libc::c_double
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double) as libc::c_int;
                LL = sub_point(pt, s2);
                UR = add_point(pt, s2);
                LL
                    .x = (if LL.x >= 0 as libc::c_int {
                    LL.x / ssize
                } else {
                    (LL.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                LL
                    .y = (if LL.y >= 0 as libc::c_int {
                    LL.y / ssize
                } else {
                    (LL.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                UR
                    .x = (if UR.x >= 0 as libc::c_int {
                    UR.x / ssize
                } else {
                    (UR.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                UR
                    .y = (if UR.y >= 0 as libc::c_int {
                    UR.y / ssize
                } else {
                    (UR.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                x = LL.x;
                while x <= UR.x {
                    y = LL.y;
                    while y <= UR.y {
                        addPS(ps, x, y);
                        y += 1;
                    }
                    x += 1;
                }
                pt
                    .x = (if pt.x >= 0 as libc::c_int {
                    pt.x / ssize
                } else {
                    (pt.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                pt
                    .y = (if pt.y >= 0 as libc::c_int {
                    pt.y / ssize
                } else {
                    (pt.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                e = agfstout(eg, n);
                while !e.is_null() {
                    fillEdge(e, pt, ps, dx, dy, ssize, doSplines);
                    e = agnxtout(eg, e);
                }
            } else {
                pt
                    .x = (if pt.x >= 0 as libc::c_int {
                    pt.x / ssize
                } else {
                    (pt.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                pt
                    .y = (if pt.y >= 0 as libc::c_int {
                    pt.y / ssize
                } else {
                    (pt.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
                });
                e = agfstout(eg, n);
                while !e.is_null() {
                    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
                        == (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .clust)
                    {
                        fillEdge(e, pt, ps, dx, dy, ssize, doSplines);
                    }
                    e = agnxtout(eg, e);
                }
            }
            n = agnxtnode(g, n);
        }
        i = 0 as libc::c_int;
        n = agfstnode(g);
        while !n.is_null() {
            let fresh5 = i;
            i = i + 1;
            let ref mut fresh6 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .alg;
            *fresh6 = *alg.offset(fresh5 as isize);
            n = agnxtnode(g, n);
        }
        free(alg as *mut libc::c_void);
    } else {
        n = agfstnode(g);
        while !n.is_null() {
            ptf = coord(n);
            pt
                .x = (if ptf.x >= 0 as libc::c_int as libc::c_double {
                (ptf.x + 0.5f64) as libc::c_int
            } else {
                (ptf.x - 0.5f64) as libc::c_int
            });
            pt
                .y = (if ptf.y >= 0 as libc::c_int as libc::c_double {
                (ptf.y + 0.5f64) as libc::c_int
            } else {
                (ptf.y - 0.5f64) as libc::c_int
            });
            pt.x += dx;
            pt.y += dy;
            s2
                .x = (margin as libc::c_double
                + ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
                    / 2 as libc::c_int as libc::c_double) as libc::c_int;
            s2
                .y = (margin as libc::c_double
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double) as libc::c_int;
            LL = sub_point(pt, s2);
            UR = add_point(pt, s2);
            LL
                .x = (if LL.x >= 0 as libc::c_int {
                LL.x / ssize
            } else {
                (LL.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
            });
            LL
                .y = (if LL.y >= 0 as libc::c_int {
                LL.y / ssize
            } else {
                (LL.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
            });
            UR
                .x = (if UR.x >= 0 as libc::c_int {
                UR.x / ssize
            } else {
                (UR.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
            });
            UR
                .y = (if UR.y >= 0 as libc::c_int {
                UR.y / ssize
            } else {
                (UR.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
            });
            x = LL.x;
            while x <= UR.x {
                y = LL.y;
                while y <= UR.y {
                    addPS(ps, x, y);
                    y += 1;
                }
                x += 1;
            }
            pt
                .x = (if pt.x >= 0 as libc::c_int {
                pt.x / ssize
            } else {
                (pt.x + 1 as libc::c_int) / ssize - 1 as libc::c_int
            });
            pt
                .y = (if pt.y >= 0 as libc::c_int {
                pt.y / ssize
            } else {
                (pt.y + 1 as libc::c_int) / ssize - 1 as libc::c_int
            });
            e = agfstout(eg, n);
            while !e.is_null() {
                fillEdge(e, pt, ps, dx, dy, ssize, doSplines);
                e = agnxtout(eg, e);
            }
            n = agnxtnode(g, n);
        }
    }
    let ref mut fresh7 = (*info).cells;
    *fresh7 = pointsOf(ps);
    (*info).nc = sizeOf(ps);
    W = ceil(
        ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
            - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
            + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin) as libc::c_double)
            / ssize as libc::c_double,
    ) as libc::c_int;
    H = ceil(
        ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
            - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
            + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin) as libc::c_double)
            / ssize as libc::c_double,
    ) as libc::c_int;
    (*info).perim = W + H;
    if Verbose as libc::c_int > 2 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        fprintf(
            stderr,
            b"%s no. cells %d W %d H %d\n\0" as *const u8 as *const libc::c_char,
            agnameof(g as *mut libc::c_void),
            (*info).nc,
            W,
            H,
        );
        i_0 = 0 as libc::c_int;
        while i_0 < (*info).nc {
            fprintf(
                stderr,
                b"  %d %d cell\n\0" as *const u8 as *const libc::c_char,
                (*((*info).cells).offset(i_0 as isize)).x,
                (*((*info).cells).offset(i_0 as isize)).y,
            );
            i_0 += 1;
        }
    }
    freePS(ps);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fits(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut info: *mut ginfo,
    mut ps: *mut PointSet,
    mut place: *mut point,
    mut step: libc::c_int,
    mut bbs: *mut boxf,
) -> libc::c_int {
    let mut cells: *mut point = (*info).cells;
    let mut n: libc::c_int = (*info).nc;
    let mut cell: point = point { x: 0, y: 0 };
    let mut i: libc::c_int = 0;
    let mut LL: point = point { x: 0, y: 0 };
    i = 0 as libc::c_int;
    while i < n {
        cell = *cells;
        cell.x += x;
        cell.y += y;
        if inPS(ps, cell) != 0 {
            return 0 as libc::c_int;
        }
        cells = cells.offset(1);
        i += 1;
    }
    LL
        .x = (if (*bbs.offset((*info).index as isize)).LL.x
        >= 0 as libc::c_int as libc::c_double
    {
        ((*bbs.offset((*info).index as isize)).LL.x + 0.5f64) as libc::c_int
    } else {
        ((*bbs.offset((*info).index as isize)).LL.x - 0.5f64) as libc::c_int
    });
    LL
        .y = (if (*bbs.offset((*info).index as isize)).LL.y
        >= 0 as libc::c_int as libc::c_double
    {
        ((*bbs.offset((*info).index as isize)).LL.y + 0.5f64) as libc::c_int
    } else {
        ((*bbs.offset((*info).index as isize)).LL.y - 0.5f64) as libc::c_int
    });
    (*place).x = step * x - LL.x;
    (*place).y = step * y - LL.y;
    cells = (*info).cells;
    i = 0 as libc::c_int;
    while i < n {
        cell = *cells;
        cell.x += x;
        cell.y += y;
        insertPS(ps, cell);
        cells = cells.offset(1);
        i += 1;
    }
    if Verbose as libc::c_int >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"cc (%d cells) at (%d,%d) (%d,%d)\n\0" as *const u8 as *const libc::c_char,
            n,
            x,
            y,
            (*place).x,
            (*place).y,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn placeFixed(
    mut info: *mut ginfo,
    mut ps: *mut PointSet,
    mut place: *mut point,
    mut center: point,
) {
    let mut cells: *mut point = (*info).cells;
    let mut n: libc::c_int = (*info).nc;
    let mut i: libc::c_int = 0;
    (*place).x = -center.x;
    (*place).y = -center.y;
    i = 0 as libc::c_int;
    while i < n {
        let fresh8 = cells;
        cells = cells.offset(1);
        insertPS(ps, *fresh8);
        i += 1;
    }
    if Verbose as libc::c_int >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"cc (%d cells) at (%d,%d)\n\0" as *const u8 as *const libc::c_char,
            n,
            (*place).x,
            (*place).y,
        );
    }
}
unsafe extern "C" fn placeGraph(
    mut i: libc::c_int,
    mut info: *mut ginfo,
    mut ps: *mut PointSet,
    mut place: *mut point,
    mut step: libc::c_int,
    mut margin: libc::c_uint,
    mut bbs: *mut boxf,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut W: libc::c_int = 0;
    let mut H: libc::c_int = 0;
    let mut bnd: libc::c_int = 0;
    let mut bb: boxf = *bbs.offset((*info).index as isize);
    if i == 0 as libc::c_int {
        W = ceil(
            (bb.UR.x - bb.LL.x
                + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin)
                    as libc::c_double) / step as libc::c_double,
        ) as libc::c_int;
        H = ceil(
            (bb.UR.y - bb.LL.y
                + (2 as libc::c_int as libc::c_uint).wrapping_mul(margin)
                    as libc::c_double) / step as libc::c_double,
        ) as libc::c_int;
        if fits(-W / 2 as libc::c_int, -H / 2 as libc::c_int, info, ps, place, step, bbs)
            != 0
        {
            return;
        }
    }
    if fits(0 as libc::c_int, 0 as libc::c_int, info, ps, place, step, bbs) != 0 {
        return;
    }
    W = ceil(bb.UR.x - bb.LL.x) as libc::c_int;
    H = ceil(bb.UR.y - bb.LL.y) as libc::c_int;
    if W >= H {
        bnd = 1 as libc::c_int;
        loop {
            x = 0 as libc::c_int;
            y = -bnd;
            while x < bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                x += 1;
            }
            while y < bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                y += 1;
            }
            while x > -bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                x -= 1;
            }
            while y > -bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                y -= 1;
            }
            while x < 0 as libc::c_int {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                x += 1;
            }
            bnd += 1;
        }
    } else {
        bnd = 1 as libc::c_int;
        loop {
            y = 0 as libc::c_int;
            x = -bnd;
            while y > -bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                y -= 1;
            }
            while x < bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                x += 1;
            }
            while y < bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                y += 1;
            }
            while x > -bnd {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                x -= 1;
            }
            while y > 0 as libc::c_int {
                if fits(x, y, info, ps, place, step, bbs) != 0 {
                    return;
                }
                y -= 1;
            }
            bnd += 1;
        }
    };
}
static mut userVals: *mut packval_t = 0 as *const packval_t as *mut packval_t;
unsafe extern "C" fn ucmpf(
    mut X: *const libc::c_void,
    mut Y: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const ainfo = *(X as *const *mut ainfo);
    let mut y: *const ainfo = *(Y as *const *mut ainfo);
    let mut dX: libc::c_int = *userVals.offset((*x).index as isize) as libc::c_int;
    let mut dY: libc::c_int = *userVals.offset((*y).index as isize) as libc::c_int;
    if dX > dY {
        return 1 as libc::c_int
    } else if dX < dY {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn acmpf(
    mut X: *const libc::c_void,
    mut Y: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const ainfo = *(X as *const *mut ainfo);
    let mut y: *const ainfo = *(Y as *const *mut ainfo);
    let mut dX: libc::c_double = (*x).height + (*x).width;
    let mut dY: libc::c_double = (*y).height + (*y).width;
    if dX < dY {
        return 1 as libc::c_int
    } else if dX > dY {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn arrayRects(
    mut ng: libc::c_int,
    mut gs: *mut boxf,
    mut pinfo: *mut pack_info,
) -> *mut point {
    let mut i: libc::c_int = 0;
    let mut nr: libc::c_int = 0 as libc::c_int;
    let mut nc: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut info: *mut ainfo = 0 as *mut ainfo;
    let mut ip: *mut ainfo = 0 as *mut ainfo;
    let mut sinfo: *mut *mut ainfo = 0 as *mut *mut ainfo;
    let mut widths: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut heights: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_double = 0.;
    let mut wd: libc::c_double = 0.;
    let mut ht: libc::c_double = 0.;
    let mut places: *mut point = gcalloc(
        ng as size_t,
        ::std::mem::size_of::<point>() as libc::c_ulong,
    ) as *mut point;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut sz: libc::c_int = 0;
    let mut rowMajor: libc::c_int = 0;
    sz = (*pinfo).sz;
    if (*pinfo).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        rowMajor = 0 as libc::c_int;
        if sz > 0 as libc::c_int {
            nr = sz;
            nc = (ng + (nr - 1 as libc::c_int)) / nr;
        } else {
            nr = ceil(sqrt(ng as libc::c_double)) as libc::c_int;
            nc = (ng + (nr - 1 as libc::c_int)) / nr;
        }
    } else {
        rowMajor = 1 as libc::c_int;
        if sz > 0 as libc::c_int {
            nc = sz;
            nr = (ng + (nc - 1 as libc::c_int)) / nc;
        } else {
            nc = ceil(sqrt(ng as libc::c_double)) as libc::c_int;
            nr = (ng + (nc - 1 as libc::c_int)) / nc;
        }
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"array packing: %s %d rows %d columns\n\0" as *const u8
                as *const libc::c_char,
            if rowMajor != 0 {
                b"row major\0" as *const u8 as *const libc::c_char
            } else {
                b"column major\0" as *const u8 as *const libc::c_char
            },
            nr,
            nc,
        );
    }
    widths = gcalloc(
        (nc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    heights = gcalloc(
        (nr + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    info = gcalloc(ng as size_t, ::std::mem::size_of::<ainfo>() as libc::c_ulong)
        as *mut ainfo;
    ip = info;
    i = 0 as libc::c_int;
    while i < ng {
        bb = *gs.offset(i as isize);
        (*ip).width = bb.UR.x - bb.LL.x + (*pinfo).margin as libc::c_double;
        (*ip).height = bb.UR.y - bb.LL.y + (*pinfo).margin as libc::c_double;
        (*ip).index = i;
        i += 1;
        ip = ip.offset(1);
    }
    sinfo = gcalloc(ng as size_t, ::std::mem::size_of::<*mut ainfo>() as libc::c_ulong)
        as *mut *mut ainfo;
    i = 0 as libc::c_int;
    while i < ng {
        let ref mut fresh9 = *sinfo.offset(i as isize);
        *fresh9 = info.offset(i as isize);
        i += 1;
    }
    if !((*pinfo).vals).is_null() {
        userVals = (*pinfo).vals;
        qsort(
            sinfo as *mut libc::c_void,
            ng as size_t,
            ::std::mem::size_of::<*mut ainfo>() as libc::c_ulong,
            Some(
                ucmpf
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    } else if (*pinfo).flags & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        qsort(
            sinfo as *mut libc::c_void,
            ng as size_t,
            ::std::mem::size_of::<*mut ainfo>() as libc::c_ulong,
            Some(
                acmpf
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    c = 0 as libc::c_int;
    r = c;
    i = 0 as libc::c_int;
    while i < ng {
        ip = *sinfo.offset(i as isize);
        *widths
            .offset(
                c as isize,
            ) = if *widths.offset(c as isize) > (*ip).width {
            *widths.offset(c as isize)
        } else {
            (*ip).width
        };
        *heights
            .offset(
                r as isize,
            ) = if *heights.offset(r as isize) > (*ip).height {
            *heights.offset(r as isize)
        } else {
            (*ip).height
        };
        if rowMajor != 0 {
            c += 1;
            if c == nc {
                c = 0 as libc::c_int;
                r += 1;
            }
        } else {
            r += 1;
            if r == nr {
                r = 0 as libc::c_int;
                c += 1;
            }
        }
        i += 1;
        ip = ip.offset(1);
    }
    wd = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i <= nc {
        v = *widths.offset(i as isize);
        *widths.offset(i as isize) = wd;
        wd += v;
        i += 1;
    }
    ht = 0 as libc::c_int as libc::c_double;
    i = nr;
    while (0 as libc::c_int) < i {
        v = *heights.offset((i - 1 as libc::c_int) as isize);
        *heights.offset(i as isize) = ht;
        ht += v;
        i -= 1;
    }
    *heights.offset(0 as libc::c_int as isize) = ht;
    c = 0 as libc::c_int;
    r = c;
    i = 0 as libc::c_int;
    while i < ng {
        let mut idx: libc::c_int = 0;
        ip = *sinfo.offset(i as isize);
        idx = (*ip).index;
        bb = *gs.offset(idx as isize);
        if (*pinfo).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            (*places.offset(idx as isize)).x = *widths.offset(c as isize) as libc::c_int;
        } else if (*pinfo).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            (*places.offset(idx as isize))
                .x = (*widths.offset((c + 1 as libc::c_int) as isize)
                - (bb.UR.x - bb.LL.x)) as libc::c_int;
        } else {
            (*places.offset(idx as isize))
                .x = ((*widths.offset(c as isize)
                + *widths.offset((c + 1 as libc::c_int) as isize) - bb.UR.x - bb.LL.x)
                / 2.0f64) as libc::c_int;
        }
        if (*pinfo).flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            (*places.offset(idx as isize))
                .y = (*heights.offset(r as isize) - (bb.UR.y - bb.LL.y)) as libc::c_int;
        } else if (*pinfo).flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
            (*places.offset(idx as isize))
                .y = *heights.offset((r + 1 as libc::c_int) as isize) as libc::c_int;
        } else {
            (*places.offset(idx as isize))
                .y = ((*heights.offset(r as isize)
                + *heights.offset((r + 1 as libc::c_int) as isize) - bb.UR.y - bb.LL.y)
                / 2.0f64) as libc::c_int;
        }
        if rowMajor != 0 {
            c += 1;
            if c == nc {
                c = 0 as libc::c_int;
                r += 1;
            }
        } else {
            r += 1;
            if r == nr {
                r = 0 as libc::c_int;
                c += 1;
            }
        }
        i += 1;
        ip = ip.offset(1);
    }
    free(info as *mut libc::c_void);
    free(sinfo as *mut libc::c_void);
    free(widths as *mut libc::c_void);
    free(heights as *mut libc::c_void);
    return places;
}
unsafe extern "C" fn polyRects(
    mut ng: libc::c_int,
    mut gs: *mut boxf,
    mut pinfo: *mut pack_info,
) -> *mut point {
    let mut stepSize: libc::c_int = 0;
    let mut info: *mut ginfo = 0 as *mut ginfo;
    let mut sinfo: *mut *mut ginfo = 0 as *mut *mut ginfo;
    let mut places: *mut point = 0 as *mut point;
    let mut ps: *mut Dict_t = 0 as *mut Dict_t;
    let mut i: libc::c_int = 0;
    let mut center: point = point { x: 0, y: 0 };
    stepSize = computeStep(ng, gs, (*pinfo).margin);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"step size = %d\n\0" as *const u8 as *const libc::c_char,
            stepSize,
        );
    }
    if stepSize <= 0 as libc::c_int {
        return 0 as *mut point;
    }
    center.y = 0 as libc::c_int;
    center.x = center.y;
    info = gcalloc(ng as size_t, ::std::mem::size_of::<ginfo>() as libc::c_ulong)
        as *mut ginfo;
    i = 0 as libc::c_int;
    while i < ng {
        (*info.offset(i as isize)).index = i;
        genBox(
            *gs.offset(i as isize),
            info.offset(i as isize),
            stepSize,
            (*pinfo).margin,
            center,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        i += 1;
    }
    sinfo = gcalloc(ng as size_t, ::std::mem::size_of::<*mut ginfo>() as libc::c_ulong)
        as *mut *mut ginfo;
    i = 0 as libc::c_int;
    while i < ng {
        let ref mut fresh10 = *sinfo.offset(i as isize);
        *fresh10 = info.offset(i as isize);
        i += 1;
    }
    qsort(
        sinfo as *mut libc::c_void,
        ng as size_t,
        ::std::mem::size_of::<*mut ginfo>() as libc::c_ulong,
        Some(
            cmpf
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    ps = newPS();
    places = gcalloc(ng as size_t, ::std::mem::size_of::<point>() as libc::c_ulong)
        as *mut point;
    i = 0 as libc::c_int;
    while i < ng {
        placeGraph(
            i,
            *sinfo.offset(i as isize),
            ps,
            places.offset((**sinfo.offset(i as isize)).index as isize),
            stepSize,
            (*pinfo).margin,
            gs,
        );
        i += 1;
    }
    free(sinfo as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < ng {
        free((*info.offset(i as isize)).cells as *mut libc::c_void);
        i += 1;
    }
    free(info as *mut libc::c_void);
    freePS(ps);
    if Verbose as libc::c_int > 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < ng {
            fprintf(
                stderr,
                b"pos[%d] %d %d\n\0" as *const u8 as *const libc::c_char,
                i,
                (*places.offset(i as isize)).x,
                (*places.offset(i as isize)).y,
            );
            i += 1;
        }
    }
    return places;
}
unsafe extern "C" fn polyGraphs(
    mut ng: libc::c_int,
    mut gs: *mut *mut Agraph_t,
    mut root: *mut Agraph_t,
    mut pinfo: *mut pack_info,
) -> *mut point {
    let mut stepSize: libc::c_int = 0;
    let mut info: *mut ginfo = 0 as *mut ginfo;
    let mut sinfo: *mut *mut ginfo = 0 as *mut *mut ginfo;
    let mut places: *mut point = 0 as *mut point;
    let mut ps: *mut Dict_t = 0 as *mut Dict_t;
    let mut i: libc::c_int = 0;
    let mut fixed: *mut bool = (*pinfo).fixed;
    let mut fixed_cnt: libc::c_int = 0 as libc::c_int;
    let mut bb: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    let mut fixed_bb: box_0 = {
        let mut init = box_0 {
            LL: {
                let mut init = point {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
            UR: {
                let mut init = point {
                    x: 0 as libc::c_int,
                    y: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    let mut center: point = point { x: 0, y: 0 };
    let mut bbs: *mut boxf = 0 as *mut boxf;
    if ng <= 0 as libc::c_int {
        return 0 as *mut point;
    }
    i = 0 as libc::c_int;
    while i < ng {
        let mut g: *mut Agraph_t = *gs.offset(i as isize);
        compute_bb(g);
        if !fixed.is_null() && *fixed.offset(i as isize) as libc::c_int != 0 {
            bb
                .LL
                .x = (if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x + 0.5f64)
                    as libc::c_int
            } else {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x - 0.5f64)
                    as libc::c_int
            });
            bb
                .LL
                .y = (if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y + 0.5f64)
                    as libc::c_int
            } else {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y - 0.5f64)
                    as libc::c_int
            });
            bb
                .UR
                .x = (if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x + 0.5f64)
                    as libc::c_int
            } else {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x - 0.5f64)
                    as libc::c_int
            });
            bb
                .UR
                .y = (if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y + 0.5f64)
                    as libc::c_int
            } else {
                ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y - 0.5f64)
                    as libc::c_int
            });
            if fixed_cnt != 0 {
                fixed_bb
                    .LL
                    .x = if bb.LL.x < fixed_bb.LL.x { bb.LL.x } else { fixed_bb.LL.x };
                fixed_bb
                    .LL
                    .y = if bb.LL.y < fixed_bb.LL.y { bb.LL.y } else { fixed_bb.LL.y };
                fixed_bb
                    .UR
                    .x = if bb.UR.x > fixed_bb.UR.x { bb.UR.x } else { fixed_bb.UR.x };
                fixed_bb
                    .UR
                    .y = if bb.UR.y > fixed_bb.UR.y { bb.UR.y } else { fixed_bb.UR.y };
            } else {
                fixed_bb = bb;
            }
            fixed_cnt += 1;
        }
        if Verbose as libc::c_int > 2 as libc::c_int {
            fprintf(
                stderr,
                b"bb[%s] %.5g %.5g %.5g %.5g\n\0" as *const u8 as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x,
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y,
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x,
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y,
            );
        }
        i += 1;
    }
    bbs = gcalloc(ng as size_t, ::std::mem::size_of::<boxf>() as libc::c_ulong)
        as *mut boxf;
    i = 0 as libc::c_int;
    while i < ng {
        *bbs
            .offset(
                i as isize,
            ) = (*((*(*gs.offset(i as isize) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .bb;
        i += 1;
    }
    stepSize = computeStep(ng, bbs, (*pinfo).margin);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"step size = %d\n\0" as *const u8 as *const libc::c_char,
            stepSize,
        );
    }
    if stepSize <= 0 as libc::c_int {
        return 0 as *mut point;
    }
    if !fixed.is_null() {
        center.x = (fixed_bb.LL.x + fixed_bb.UR.x) / 2 as libc::c_int;
        center.y = (fixed_bb.LL.y + fixed_bb.UR.y) / 2 as libc::c_int;
    } else {
        center.y = 0 as libc::c_int;
        center.x = center.y;
    }
    info = gcalloc(ng as size_t, ::std::mem::size_of::<ginfo>() as libc::c_ulong)
        as *mut ginfo;
    i = 0 as libc::c_int;
    while i < ng {
        let mut g_0: *mut Agraph_t = *gs.offset(i as isize);
        (*info.offset(i as isize)).index = i;
        if (*pinfo).mode as libc::c_uint == l_graph as libc::c_int as libc::c_uint {
            genBox(
                (*((*(g_0 as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
                info.offset(i as isize),
                stepSize,
                (*pinfo).margin,
                center,
                agnameof(g_0 as *mut libc::c_void),
            );
        } else if genPoly(
                root,
                *gs.offset(i as isize),
                info.offset(i as isize),
                stepSize,
                pinfo,
                center,
            ) != 0
            {
            return 0 as *mut point
        }
        i += 1;
    }
    sinfo = gcalloc(ng as size_t, ::std::mem::size_of::<*mut ginfo>() as libc::c_ulong)
        as *mut *mut ginfo;
    i = 0 as libc::c_int;
    while i < ng {
        let ref mut fresh11 = *sinfo.offset(i as isize);
        *fresh11 = info.offset(i as isize);
        i += 1;
    }
    qsort(
        sinfo as *mut libc::c_void,
        ng as size_t,
        ::std::mem::size_of::<*mut ginfo>() as libc::c_ulong,
        Some(
            cmpf
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    ps = newPS();
    places = gcalloc(ng as size_t, ::std::mem::size_of::<point>() as libc::c_ulong)
        as *mut point;
    if !fixed.is_null() {
        i = 0 as libc::c_int;
        while i < ng {
            if *fixed.offset(i as isize) {
                placeFixed(
                    *sinfo.offset(i as isize),
                    ps,
                    places.offset((**sinfo.offset(i as isize)).index as isize),
                    center,
                );
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < ng {
            if !*fixed.offset(i as isize) {
                placeGraph(
                    i,
                    *sinfo.offset(i as isize),
                    ps,
                    places.offset((**sinfo.offset(i as isize)).index as isize),
                    stepSize,
                    (*pinfo).margin,
                    bbs,
                );
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < ng {
            placeGraph(
                i,
                *sinfo.offset(i as isize),
                ps,
                places.offset((**sinfo.offset(i as isize)).index as isize),
                stepSize,
                (*pinfo).margin,
                bbs,
            );
            i += 1;
        }
    }
    free(sinfo as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < ng {
        free((*info.offset(i as isize)).cells as *mut libc::c_void);
        i += 1;
    }
    free(info as *mut libc::c_void);
    freePS(ps);
    free(bbs as *mut libc::c_void);
    if Verbose as libc::c_int > 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < ng {
            fprintf(
                stderr,
                b"pos[%d] %d %d\n\0" as *const u8 as *const libc::c_char,
                i,
                (*places.offset(i as isize)).x,
                (*places.offset(i as isize)).y,
            );
            i += 1;
        }
    }
    return places;
}
#[no_mangle]
pub unsafe extern "C" fn putGraphs(
    mut ng: libc::c_int,
    mut gs: *mut *mut Agraph_t,
    mut root: *mut Agraph_t,
    mut pinfo: *mut pack_info,
) -> *mut point {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut bbs: *mut boxf = 0 as *mut boxf;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut pts: *mut point = 0 as *mut point;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if ng <= 0 as libc::c_int {
        return 0 as *mut point;
    }
    if (*pinfo).mode as libc::c_uint <= l_graph as libc::c_int as libc::c_uint {
        return polyGraphs(ng, gs, root, pinfo);
    }
    bbs = gcalloc(ng as size_t, ::std::mem::size_of::<boxf>() as libc::c_ulong)
        as *mut boxf;
    i = 0 as libc::c_int;
    while i < ng {
        g = *gs.offset(i as isize);
        compute_bb(g);
        *bbs
            .offset(
                i as isize,
            ) = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb;
        i += 1;
    }
    if (*pinfo).mode as libc::c_uint == l_array as libc::c_int as libc::c_uint {
        if (*pinfo).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            let ref mut fresh12 = (*pinfo).vals;
            *fresh12 = gcalloc(
                ng as size_t,
                ::std::mem::size_of::<packval_t>() as libc::c_ulong,
            ) as *mut packval_t;
            i = 0 as libc::c_int;
            while i < ng {
                s = agget(
                    *gs.offset(i as isize) as *mut libc::c_void,
                    b"sortv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if !s.is_null()
                    && sscanf(
                        s,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut v as *mut libc::c_int,
                    ) > 0 as libc::c_int && v >= 0 as libc::c_int
                {
                    *((*pinfo).vals).offset(i as isize) = v as packval_t;
                }
                i += 1;
            }
        }
        pts = arrayRects(ng, bbs, pinfo);
        if (*pinfo).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            free((*pinfo).vals as *mut libc::c_void);
        }
    }
    free(bbs as *mut libc::c_void);
    return pts;
}
#[no_mangle]
pub unsafe extern "C" fn putRects(
    mut ng: libc::c_int,
    mut bbs: *mut boxf,
    mut pinfo: *mut pack_info,
) -> *mut point {
    if ng <= 0 as libc::c_int {
        return 0 as *mut point;
    }
    if (*pinfo).mode as libc::c_uint == l_node as libc::c_int as libc::c_uint
        || (*pinfo).mode as libc::c_uint == l_clust as libc::c_int as libc::c_uint
    {
        return 0 as *mut point;
    }
    if (*pinfo).mode as libc::c_uint == l_graph as libc::c_int as libc::c_uint {
        return polyRects(ng, bbs, pinfo)
    } else if (*pinfo).mode as libc::c_uint == l_array as libc::c_int as libc::c_uint {
        return arrayRects(ng, bbs, pinfo)
    } else {
        return 0 as *mut point
    };
}
#[no_mangle]
pub unsafe extern "C" fn packRects(
    mut ng: libc::c_int,
    mut bbs: *mut boxf,
    mut pinfo: *mut pack_info,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pp: *mut point = 0 as *mut point;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut p: point = point { x: 0, y: 0 };
    if ng < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if ng <= 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    pp = putRects(ng, bbs, pinfo);
    if pp.is_null() {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < ng {
        bb = *bbs.offset(i as isize);
        p = *pp.offset(i as isize);
        bb.LL.x += p.x as libc::c_double;
        bb.UR.x += p.x as libc::c_double;
        bb.LL.y += p.y as libc::c_double;
        bb.UR.y += p.y as libc::c_double;
        *bbs.offset(i as isize) = bb;
        i += 1;
    }
    free(pp as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn shiftEdge(
    mut e: *mut Agedge_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos.x
            += dx as libc::c_double;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos.y
            += dy as libc::c_double;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).pos.x
            += dx as libc::c_double;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).pos.y
            += dy as libc::c_double;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).pos.x
            += dx as libc::c_double;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).pos.y
            += dy as libc::c_double;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).is_null() {
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).pos.x
            += dx as libc::c_double;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).pos.y
            += dy as libc::c_double;
    }
    if ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
        return;
    }
    j = 0 as libc::c_int;
    while j < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
        bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
            .offset(j as isize);
        k = 0 as libc::c_int;
        while k < bz.size {
            (*(bz.list).offset(k as isize)).x += dx as libc::c_double;
            (*(bz.list).offset(k as isize)).y += dy as libc::c_double;
            k += 1;
        }
        if bz.sflag != 0 {
            (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                .offset(j as isize))
                .sp
                .x += dx as libc::c_double;
            (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                .offset(j as isize))
                .sp
                .y += dy as libc::c_double;
        }
        if bz.eflag != 0 {
            (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                .offset(j as isize))
                .ep
                .x += dx as libc::c_double;
            (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                .offset(j as isize))
                .ep
                .y += dy as libc::c_double;
        }
        j += 1;
    }
}
unsafe extern "C" fn shiftGraph(
    mut g: *mut Agraph_t,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
) {
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut bb: boxf = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb;
    let mut i: libc::c_int = 0;
    bb = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb;
    bb.LL.x += dx as libc::c_double;
    bb.UR.x += dx as libc::c_double;
    bb.LL.y += dy as libc::c_double;
    bb.UR.y += dy as libc::c_double;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb = bb;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).set
            as libc::c_int != 0
    {
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos.x
            += dx as libc::c_double;
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos.y
            += dy as libc::c_double;
    }
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        subg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(i as isize);
        shiftGraph(subg, dx, dy);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn shiftGraphs(
    mut ng: libc::c_int,
    mut gs: *mut *mut Agraph_t,
    mut pp: *mut point,
    mut root: *mut Agraph_t,
    mut doSplines: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut fx: libc::c_double = 0.;
    let mut fy: libc::c_double = 0.;
    let mut p: point = point { x: 0, y: 0 };
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut eg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    if ng <= 0 as libc::c_int {
        return abs(ng);
    }
    i = 0 as libc::c_int;
    while i < ng {
        g = *gs.offset(i as isize);
        if !root.is_null() {
            eg = root;
        } else {
            eg = g;
        }
        p = *pp.offset(i as isize);
        dx = p.x;
        dy = p.y;
        fx = dx as libc::c_double / 72 as libc::c_int as libc::c_double;
        fy = dy as libc::c_double / 72 as libc::c_int as libc::c_double;
        n = agfstnode(g);
        while !n.is_null() {
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) += fx;
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) += fy;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                += dx as libc::c_double;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                += dy as libc::c_double;
            if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null()
            {
                (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).pos.x
                    += dx as libc::c_double;
                (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).pos.y
                    += dy as libc::c_double;
            }
            if doSplines != 0 {
                e = agfstout(eg, n);
                while !e.is_null() {
                    shiftEdge(e, dx, dy);
                    e = agnxtout(eg, e);
                }
            }
            n = agnxtnode(g, n);
        }
        shiftGraph(g, dx, dy);
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn packGraphs(
    mut ng: libc::c_int,
    mut gs: *mut *mut Agraph_t,
    mut root: *mut Agraph_t,
    mut info: *mut pack_info,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut pp: *mut point = putGraphs(ng, gs, root, info);
    if pp.is_null() {
        return 1 as libc::c_int;
    }
    ret = shiftGraphs(ng, gs, pp, root, (*info).doSplines);
    free(pp as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn packSubgraphs(
    mut ng: libc::c_int,
    mut gs: *mut *mut Agraph_t,
    mut root: *mut Agraph_t,
    mut info: *mut pack_info,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = packGraphs(ng, gs, root, info);
    if ret == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut bb: boxf = boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        };
        let mut g: *mut graph_t = 0 as *mut graph_t;
        compute_bb(root);
        bb = (*((*(root as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb;
        i = 0 as libc::c_int;
        while i < ng {
            g = *gs.offset(i as isize);
            j = 1 as libc::c_int;
            while j <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
                bb
                    .LL
                    .x = (if bb.LL.x
                    < (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .x
                {
                    bb.LL.x
                } else {
                    (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .x
                });
                bb
                    .LL
                    .y = (if bb.LL.y
                    < (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .y
                {
                    bb.LL.y
                } else {
                    (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .LL
                        .y
                });
                bb
                    .UR
                    .x = (if bb.UR.x
                    > (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .x
                {
                    bb.UR.x
                } else {
                    (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .x
                });
                bb
                    .UR
                    .y = (if bb.UR.y
                    > (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                        .clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .y
                {
                    bb.UR.y
                } else {
                    (*((*(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                        .offset(j as isize) as *mut Agobj_t))
                        .data as *mut Agraphinfo_t))
                        .bb
                        .UR
                        .y
                });
                j += 1;
            }
            i += 1;
        }
        (*((*(root as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb = bb;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn pack_graph(
    mut ng: libc::c_int,
    mut gs: *mut *mut Agraph_t,
    mut root: *mut Agraph_t,
    mut fixed: *mut bool,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut info: pack_info = pack_info {
        aspect: 0.,
        sz: 0,
        margin: 0,
        doSplines: 0,
        mode: l_undef,
        fixed: 0 as *mut bool,
        vals: 0 as *mut packval_t,
        flags: 0,
    };
    getPackInfo(root, l_graph, 8 as libc::c_int, &mut info);
    info.doSplines = 1 as libc::c_int;
    info.fixed = fixed;
    ret = packSubgraphs(ng, gs, root, &mut info);
    if ret == 0 as libc::c_int {
        dotneato_postprocess(root);
    }
    return ret;
}
unsafe extern "C" fn chkFlags(
    mut p: *const libc::c_char,
    mut pinfo: *mut pack_info,
) -> *const libc::c_char {
    let mut c: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    if *p as libc::c_int != '_' as i32 {
        return p;
    }
    p = p.offset(1);
    more = 1 as libc::c_int;
    while more != 0
        && {
            c = *p as libc::c_int;
            c != 0
        }
    {
        match c {
            99 => {
                (*pinfo).flags |= (1 as libc::c_int) << 0 as libc::c_int;
                p = p.offset(1);
            }
            105 => {
                (*pinfo).flags |= (1 as libc::c_int) << 6 as libc::c_int;
                p = p.offset(1);
            }
            117 => {
                (*pinfo).flags |= (1 as libc::c_int) << 1 as libc::c_int;
                p = p.offset(1);
            }
            116 => {
                (*pinfo).flags |= (1 as libc::c_int) << 4 as libc::c_int;
                p = p.offset(1);
            }
            98 => {
                (*pinfo).flags |= (1 as libc::c_int) << 5 as libc::c_int;
                p = p.offset(1);
            }
            108 => {
                (*pinfo).flags |= (1 as libc::c_int) << 2 as libc::c_int;
                p = p.offset(1);
            }
            114 => {
                (*pinfo).flags |= (1 as libc::c_int) << 3 as libc::c_int;
                p = p.offset(1);
            }
            _ => {
                more = 0 as libc::c_int;
            }
        }
    }
    return p;
}
unsafe extern "C" fn mode2Str(mut m: pack_mode) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match m as libc::c_uint {
        1 => {
            s = b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            s = b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        3 => {
            s = b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        4 => {
            s = b"array\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        5 => {
            s = b"aspect\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        0 | _ => {
            s = b"undefined\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn parsePackModeInfo(
    mut p: *const libc::c_char,
    mut dflt: pack_mode,
    mut pinfo: *mut pack_info,
) -> pack_mode {
    let mut v: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if !pinfo.is_null() {} else {
        __assert_fail(
            b"pinfo\0" as *const u8 as *const libc::c_char,
            b"pack.c\0" as *const u8 as *const libc::c_char,
            1278 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"pack_mode parsePackModeInfo(const char *, pack_mode, pack_info *)\0"))
                .as_ptr(),
        );
    }
    (*pinfo).flags = 0 as libc::c_int;
    (*pinfo).mode = dflt;
    (*pinfo).sz = 0 as libc::c_int;
    let ref mut fresh13 = (*pinfo).vals;
    *fresh13 = 0 as *mut packval_t;
    if !p.is_null() && *p as libc::c_int != 0 {
        match *p as libc::c_int {
            97 => {
                if strncmp(
                    p,
                    b"array\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0
                {
                    (*pinfo).mode = l_array;
                    p = p
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    p = chkFlags(p, pinfo);
                    if sscanf(
                        p,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut i as *mut libc::c_int,
                    ) > 0 as libc::c_int && i > 0 as libc::c_int
                    {
                        (*pinfo).sz = i;
                    }
                } else if strncmp(
                        p,
                        b"aspect\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                    {
                    (*pinfo).mode = l_aspect;
                    if sscanf(
                        p
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 6]>()
                                    as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ),
                        b"%f\0" as *const u8 as *const libc::c_char,
                        &mut v as *mut libc::c_float,
                    ) > 0 as libc::c_int && v > 0 as libc::c_int as libc::c_float
                    {
                        (*pinfo).aspect = v;
                    } else {
                        (*pinfo).aspect = 1 as libc::c_int as libc::c_float;
                    }
                }
            }
            99 => {
                if strcmp(p, b"cluster\0" as *const u8 as *const libc::c_char) == 0 {
                    (*pinfo).mode = l_clust;
                }
            }
            103 => {
                if strcmp(p, b"graph\0" as *const u8 as *const libc::c_char) == 0 {
                    (*pinfo).mode = l_graph;
                }
            }
            110 => {
                if strcmp(p, b"node\0" as *const u8 as *const libc::c_char) == 0 {
                    (*pinfo).mode = l_node;
                }
            }
            _ => {}
        }
    }
    if Verbose != 0 {
        fprintf(stderr, b"pack info:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            b"  mode   %s\n\0" as *const u8 as *const libc::c_char,
            mode2Str((*pinfo).mode),
        );
        if (*pinfo).mode as libc::c_uint == l_aspect as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"  aspect %f\n\0" as *const u8 as *const libc::c_char,
                (*pinfo).aspect as libc::c_double,
            );
        }
        fprintf(
            stderr,
            b"  size   %d\n\0" as *const u8 as *const libc::c_char,
            (*pinfo).sz,
        );
        fprintf(
            stderr,
            b"  flags  %d\n\0" as *const u8 as *const libc::c_char,
            (*pinfo).flags,
        );
    }
    return (*pinfo).mode;
}
#[no_mangle]
pub unsafe extern "C" fn getPackModeInfo(
    mut g: *mut Agraph_t,
    mut dflt: pack_mode,
    mut pinfo: *mut pack_info,
) -> pack_mode {
    return parsePackModeInfo(
        agget(
            g as *mut libc::c_void,
            b"packmode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        dflt,
        pinfo,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getPackMode(
    mut g: *mut Agraph_t,
    mut dflt: pack_mode,
) -> pack_mode {
    let mut info: pack_info = pack_info {
        aspect: 0.,
        sz: 0,
        margin: 0,
        doSplines: 0,
        mode: l_undef,
        fixed: 0 as *mut bool,
        vals: 0 as *mut packval_t,
        flags: 0,
    };
    return getPackModeInfo(g, dflt, &mut info);
}
#[no_mangle]
pub unsafe extern "C" fn getPack(
    mut g: *mut Agraph_t,
    mut not_def: libc::c_int,
    mut dflt: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = not_def;
    p = agget(
        g as *mut libc::c_void,
        b"pack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        if sscanf(
            p,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_int,
        ) == 1 as libc::c_int && i >= 0 as libc::c_int
        {
            v = i;
        } else if *p as libc::c_int == 't' as i32 || *p as libc::c_int == 'T' as i32 {
            v = dflt;
        }
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn getPackInfo(
    mut g: *mut Agraph_t,
    mut dflt: pack_mode,
    mut dfltMargin: libc::c_int,
    mut pinfo: *mut pack_info,
) -> pack_mode {
    if !pinfo.is_null() {} else {
        __assert_fail(
            b"pinfo\0" as *const u8 as *const libc::c_char,
            b"pack.c\0" as *const u8 as *const libc::c_char,
            1368 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"pack_mode getPackInfo(Agraph_t *, pack_mode, int, pack_info *)\0"))
                .as_ptr(),
        );
    }
    (*pinfo).margin = getPack(g, dfltMargin, dfltMargin) as libc::c_uint;
    if Verbose != 0 {
        fprintf(
            stderr,
            b"  margin %u\n\0" as *const u8 as *const libc::c_char,
            (*pinfo).margin,
        );
    }
    (*pinfo).doSplines = 0 as libc::c_int;
    let ref mut fresh14 = (*pinfo).fixed;
    *fresh14 = 0 as *mut bool;
    getPackModeInfo(g, dflt, pinfo);
    return (*pinfo).mode;
}
