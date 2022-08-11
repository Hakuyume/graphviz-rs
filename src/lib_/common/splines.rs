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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn abort() -> !;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut E_labeldistance: *mut Agsym_t;
    static mut E_labelangle: *mut Agsym_t;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn Bezier(
        _: *mut pointf,
        _: libc::c_int,
        _: libc::c_double,
        _: *mut pointf,
        _: *mut pointf,
    ) -> pointf;
    fn late_double(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_double,
        _: libc::c_double,
    ) -> libc::c_double;
    fn updateBB(g: *mut graph_t, lp: *mut textlabel_t);
    fn dotneato_closest(spl: *mut splines, p: pointf) -> pointf;
    fn arrow_flags(e: *mut Agedge_t, sflag: *mut libc::c_int, eflag: *mut libc::c_int);
    fn arrowEndClip(
        _: *mut edge_t,
        _: *mut pointf,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut bezier,
        eflag: libc::c_int,
    ) -> libc::c_int;
    fn arrowStartClip(
        _: *mut edge_t,
        ps: *mut pointf,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut bezier,
        sflag: libc::c_int,
    ) -> libc::c_int;
    fn arrowOrthoClip(
        _: *mut edge_t,
        ps: *mut pointf,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut bezier,
        sflag: libc::c_int,
        eflag: libc::c_int,
    );
    fn resolvePort(n: *mut node_t, other: *mut node_t, oldport: *mut port) -> port;
    fn update_bb_bz(bb: *mut boxf, cp: *mut pointf);
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
pub struct pathend_t {
    pub nb: boxf,
    pub np: pointf,
    pub sidemask: libc::c_int,
    pub boxn: libc::c_int,
    pub boxes: [boxf; 20],
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
#[inline]
unsafe extern "C" fn pointfof(mut x: libc::c_double, mut y: libc::c_double) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = x;
    r.y = y;
    return r;
}
#[inline]
unsafe extern "C" fn add_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x + q.x;
    r.y = p.y + q.y;
    return r;
}
unsafe extern "C" fn arrow_clip(
    mut fe: *mut edge_t,
    mut hn: *mut node_t,
    mut ps: *mut pointf,
    mut startp: *mut libc::c_int,
    mut endp: *mut libc::c_int,
    mut spl: *mut bezier,
    mut info: *mut splineInfo,
) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut sflag: libc::c_int = 0;
    let mut eflag: libc::c_int = 0;
    let mut j: bool = false;
    e = fe;
    while !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig).is_null() {
        e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    if (*info).ignoreSwap {
        j = 0 as libc::c_int != 0;
    } else {
        j = ((*info).swapEnds).expect("non-null function pointer")(e);
    }
    arrow_flags(e, &mut sflag, &mut eflag);
    if ((*info).splineMerge).expect("non-null function pointer")(hn) {
        eflag = 0 as libc::c_int;
    }
    if ((*info).splineMerge)
        .expect(
            "non-null function pointer",
        )(
        (*if ((*(fe as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            fe
        } else {
            fe.offset(1 as libc::c_int as isize)
        })
            .node,
    )
    {
        sflag = 0 as libc::c_int;
    }
    if j {
        i = sflag;
        sflag = eflag;
        eflag = i;
    }
    if (*info).isOrtho {
        if eflag != 0 || sflag != 0 {
            arrowOrthoClip(e, ps, *startp, *endp, spl, sflag, eflag);
        }
    } else {
        if sflag != 0 {
            *startp = arrowStartClip(e, ps, *startp, *endp, spl, sflag);
        }
        if eflag != 0 {
            *endp = arrowEndClip(e, ps, *startp, *endp, spl, eflag);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bezier_clip(
    mut inside_context: *mut inside_t,
    mut inside: Option::<unsafe extern "C" fn(*mut inside_t, pointf) -> bool>,
    mut sp: *mut pointf,
    mut left_inside: bool,
) {
    let mut seg: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut best: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut pt: pointf = pointf { x: 0., y: 0. };
    let mut opt: pointf = pointf { x: 0., y: 0. };
    let mut left: *mut pointf = 0 as *mut pointf;
    let mut right: *mut pointf = 0 as *mut pointf;
    let mut low: libc::c_double = 0.;
    let mut high: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut idir: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut odir: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut found: bool = false;
    let mut i: libc::c_int = 0;
    if left_inside {
        left = 0 as *mut pointf;
        right = seg.as_mut_ptr();
        pt = *sp.offset(0 as libc::c_int as isize);
        idir = &mut low;
        odir = &mut high;
    } else {
        left = seg.as_mut_ptr();
        right = 0 as *mut pointf;
        pt = *sp.offset(3 as libc::c_int as isize);
        idir = &mut high;
        odir = &mut low;
    }
    found = 0 as libc::c_int != 0;
    low = 0.0f64;
    high = 1.0f64;
    loop {
        opt = pt;
        t = (high + low) / 2.0f64;
        pt = Bezier(sp, 3 as libc::c_int, t, left, right);
        if inside.expect("non-null function pointer")(inside_context, pt) {
            *idir = t;
        } else {
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                best[i as usize] = seg[i as usize];
                i += 1;
            }
            found = 1 as libc::c_int != 0;
            *odir = t;
        }
        if !(fabs(opt.x - pt.x) > 0.5f64 || fabs(opt.y - pt.y) > 0.5f64) {
            break;
        }
    }
    if found {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *sp.offset(i as isize) = best[i as usize];
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *sp.offset(i as isize) = seg[i as usize];
            i += 1;
        }
    };
}
unsafe extern "C" fn shape_clip0(
    mut inside_context: *mut inside_t,
    mut n: *mut node_t,
    mut curve: *mut pointf,
    mut left_inside: bool,
) {
    let mut i: libc::c_int = 0;
    let mut save_real_size: libc::c_double = 0.;
    let mut c: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    save_real_size = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        c[i as usize]
            .x = (*curve.offset(i as isize)).x
            - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
        c[i as usize]
            .y = (*curve.offset(i as isize)).y
            - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
        i += 1;
    }
    bezier_clip(
        inside_context,
        (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns).insidefn,
        c.as_mut_ptr(),
        left_inside,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*curve.offset(i as isize))
            .x = c[i as usize].x
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
        (*curve.offset(i as isize))
            .y = c[i as usize].y
            + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
        i += 1;
    }
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw = save_real_size;
}
#[no_mangle]
pub unsafe extern "C" fn shape_clip(mut n: *mut node_t, mut curve: *mut pointf) {
    let mut save_real_size: libc::c_double = 0.;
    let mut left_inside: bool = false;
    let mut c: pointf = pointf { x: 0., y: 0. };
    let mut inside_context: inside_t = inside_t {
        a: C2RustUnnamed_5 {
            p: 0 as *mut pointf,
            r: 0 as *mut libc::c_double,
        },
    };
    if ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null()
        || ((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
            .insidefn)
            .is_none()
    {
        return;
    }
    inside_context.s.n = n;
    inside_context.s.bp = 0 as *mut boxf;
    save_real_size = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
    c
        .x = (*curve.offset(0 as libc::c_int as isize)).x
        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
    c
        .y = (*curve.offset(0 as libc::c_int as isize)).y
        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
    left_inside = ((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
        .insidefn)
        .expect("non-null function pointer")(&mut inside_context, c);
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw = save_real_size;
    shape_clip0(&mut inside_context, n, curve, left_inside);
}
#[no_mangle]
pub unsafe extern "C" fn new_spline(
    mut e: *mut edge_t,
    mut sz: libc::c_int,
) -> *mut bezier {
    let mut rv: *mut bezier = 0 as *mut bezier;
    while (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type as libc::c_int
        != 0 as libc::c_int
    {
        e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    if ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
        let ref mut fresh0 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
        *fresh0 = zmalloc(::std::mem::size_of::<splines>() as libc::c_ulong)
            as *mut splines;
    }
    let ref mut fresh1 = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
        .list;
    *fresh1 = if !((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
        .is_null()
    {
        grealloc(
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list
                as *mut libc::c_void,
            (((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size
                + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<bezier>() as libc::c_ulong),
        ) as *mut bezier
    } else {
        gmalloc(
            (((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size
                + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<bezier>() as libc::c_ulong),
        ) as *mut bezier
    };
    let ref mut fresh2 = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
        .size;
    let fresh3 = *fresh2;
    *fresh2 = *fresh2 + 1;
    rv = &mut *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
        .offset(fresh3 as isize) as *mut bezier;
    let ref mut fresh4 = (*rv).list;
    *fresh4 = gcalloc(sz as size_t, ::std::mem::size_of::<pointf>() as libc::c_ulong)
        as *mut pointf;
    (*rv).size = sz;
    let ref mut fresh5 = (*rv).eflag;
    *fresh5 = 0 as libc::c_int;
    (*rv).sflag = *fresh5;
    let ref mut fresh6 = (*rv).ep.y;
    *fresh6 = 0 as libc::c_int as libc::c_double;
    let ref mut fresh7 = (*rv).ep.x;
    *fresh7 = *fresh6;
    let ref mut fresh8 = (*rv).sp.y;
    *fresh8 = *fresh7;
    (*rv).sp.x = *fresh8;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn clip_and_install(
    mut fe: *mut edge_t,
    mut hn: *mut node_t,
    mut ps: *mut pointf,
    mut pn: libc::c_int,
    mut info: *mut splineInfo,
) {
    let mut p2: pointf = pointf { x: 0., y: 0. };
    let mut newspl: *mut bezier = 0 as *mut bezier;
    let mut tn: *mut node_t = 0 as *mut node_t;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut clipTail: libc::c_int = 0;
    let mut clipHead: libc::c_int = 0;
    let mut g: *mut graph_t = 0 as *mut graph_t;
    let mut orig: *mut edge_t = 0 as *mut edge_t;
    let mut tbox: *mut boxf = 0 as *mut boxf;
    let mut hbox: *mut boxf = 0 as *mut boxf;
    let mut inside_context: inside_t = inside_t {
        a: C2RustUnnamed_5 {
            p: 0 as *mut pointf,
            r: 0 as *mut libc::c_double,
        },
    };
    tn = (*if ((*(fe as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
    {
        fe
    } else {
        fe.offset(1 as libc::c_int as isize)
    })
        .node;
    g = agraphof(tn as *mut libc::c_void);
    newspl = new_spline(fe, pn);
    orig = fe;
    while (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
        as libc::c_int != 0 as libc::c_int
    {
        orig = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    if !(*info).ignoreSwap
        && (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            == (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
        && (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
            > (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
    {
        let mut tmp: *mut node_t = 0 as *mut node_t;
        tmp = hn;
        hn = tn;
        tn = tmp;
    }
    if tn
        == (*(if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            orig
        } else {
            orig.offset(1 as libc::c_int as isize)
        }))
            .node
    {
        clipTail = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .clip as libc::c_int;
        clipHead = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .clip as libc::c_int;
        tbox = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.bp;
        hbox = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.bp;
    } else {
        clipTail = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .clip as libc::c_int;
        clipHead = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .clip as libc::c_int;
        hbox = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.bp;
        tbox = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.bp;
    }
    if clipTail != 0
        && !((*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null()
        && ((*(*(*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
            .insidefn)
            .is_some()
    {
        inside_context.s.n = tn;
        inside_context.s.bp = tbox;
        start = 0 as libc::c_int;
        while start < pn - 4 as libc::c_int {
            p2
                .x = (*ps.offset((start + 3 as libc::c_int) as isize)).x
                - (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
            p2
                .y = (*ps.offset((start + 3 as libc::c_int) as isize)).y
                - (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
            if !((*(*(*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
                .insidefn)
                .expect("non-null function pointer")(&mut inside_context, p2)
            {
                break;
            }
            start += 3 as libc::c_int;
        }
        shape_clip0(
            &mut inside_context,
            tn,
            &mut *ps.offset(start as isize),
            1 as libc::c_int != 0,
        );
    } else {
        start = 0 as libc::c_int;
    }
    if clipHead != 0
        && !((*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null()
        && ((*(*(*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
            .insidefn)
            .is_some()
    {
        inside_context.s.n = hn;
        inside_context.s.bp = hbox;
        end = pn - 4 as libc::c_int;
        while end > 0 as libc::c_int {
            p2
                .x = (*ps.offset(end as isize)).x
                - (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
            p2
                .y = (*ps.offset(end as isize)).y
                - (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
            if !((*(*(*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
                .insidefn)
                .expect("non-null function pointer")(&mut inside_context, p2)
            {
                break;
            }
            end -= 3 as libc::c_int;
        }
        shape_clip0(
            &mut inside_context,
            hn,
            &mut *ps.offset(end as isize),
            0 as libc::c_int != 0,
        );
    } else {
        end = pn - 4 as libc::c_int;
    }
    while start < pn - 4 as libc::c_int {
        if !(((*ps.offset(start as isize)).x
            - (*ps.offset((start + 3 as libc::c_int) as isize)).x)
            * ((*ps.offset(start as isize)).x
                - (*ps.offset((start + 3 as libc::c_int) as isize)).x)
            + ((*ps.offset(start as isize)).y
                - (*ps.offset((start + 3 as libc::c_int) as isize)).y)
                * ((*ps.offset(start as isize)).y
                    - (*ps.offset((start + 3 as libc::c_int) as isize)).y)
            < 0.001f64 * 0.001f64)
        {
            break;
        }
        start += 3 as libc::c_int;
    }
    while end > 0 as libc::c_int {
        if !(((*ps.offset(end as isize)).x
            - (*ps.offset((end + 3 as libc::c_int) as isize)).x)
            * ((*ps.offset(end as isize)).x
                - (*ps.offset((end + 3 as libc::c_int) as isize)).x)
            + ((*ps.offset(end as isize)).y
                - (*ps.offset((end + 3 as libc::c_int) as isize)).y)
                * ((*ps.offset(end as isize)).y
                    - (*ps.offset((end + 3 as libc::c_int) as isize)).y)
            < 0.001f64 * 0.001f64)
        {
            break;
        }
        end -= 3 as libc::c_int;
    }
    arrow_clip(fe, hn, ps, &mut start, &mut end, newspl, info);
    i = start;
    while i < end + 4 as libc::c_int {
        let mut cp: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
        *((*newspl).list).offset((i - start) as isize) = *ps.offset(i as isize);
        cp[0 as libc::c_int as usize] = *ps.offset(i as isize);
        i += 1;
        if i >= end + 4 as libc::c_int {
            break;
        }
        *((*newspl).list).offset((i - start) as isize) = *ps.offset(i as isize);
        cp[1 as libc::c_int as usize] = *ps.offset(i as isize);
        i += 1;
        *((*newspl).list).offset((i - start) as isize) = *ps.offset(i as isize);
        cp[2 as libc::c_int as usize] = *ps.offset(i as isize);
        i += 1;
        cp[3 as libc::c_int as usize] = *ps.offset(i as isize);
        update_bb_bz(
            &mut (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
            cp.as_mut_ptr(),
        );
    }
    (*newspl).size = end - start + 4 as libc::c_int;
}
unsafe extern "C" fn conc_slope(mut n: *mut node_t) -> libc::c_double {
    let mut s_in: libc::c_double = 0.;
    let mut s_out: libc::c_double = 0.;
    let mut m_in: libc::c_double = 0.;
    let mut m_out: libc::c_double = 0.;
    let mut cnt_in: libc::c_int = 0;
    let mut cnt_out: libc::c_int = 0;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut e: *mut edge_t = 0 as *mut edge_t;
    s_out = 0.0f64;
    s_in = s_out;
    cnt_in = 0 as libc::c_int;
    loop {
        e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
            .offset(cnt_in as isize);
        if e.is_null() {
            break;
        }
        s_in
            += (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .coord
                .x;
        cnt_in += 1;
    }
    cnt_out = 0 as libc::c_int;
    loop {
        e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(cnt_out as isize);
        if e.is_null() {
            break;
        }
        s_out
            += (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .coord
                .x;
        cnt_out += 1;
    }
    p
        .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        - s_in / cnt_in as libc::c_double;
    p
        .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
        - (*((*((*(if ((*(*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .tag)
            .objtype() as libc::c_int == 3 as libc::c_int
        {
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(0 as libc::c_int as isize)
        } else {
            (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(0 as libc::c_int as isize))
                .offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord
            .y;
    m_in = atan2(p.y, p.x);
    p
        .x = s_out / cnt_out as libc::c_double
        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
    p
        .y = (*((*((*(if ((*(*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .out
        .list)
        .offset(0 as libc::c_int as isize) as *mut Agobj_t))
        .tag)
        .objtype() as libc::c_int == 2 as libc::c_int
    {
        *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(0 as libc::c_int as isize)
    } else {
        (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(0 as libc::c_int as isize))
            .offset(-(1 as libc::c_int as isize))
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord
        .y - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
    m_out = atan2(p.y, p.x);
    return (m_in + m_out) / 2.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn add_box(mut P: *mut path, mut b: boxf) {
    if b.LL.x < b.UR.x && b.LL.y < b.UR.y {
        let ref mut fresh9 = (*P).nbox;
        let fresh10 = *fresh9;
        *fresh9 = *fresh9 + 1;
        *((*P).boxes).offset(fresh10 as isize) = b;
    }
}
#[no_mangle]
pub unsafe extern "C" fn beginpath(
    mut P: *mut path,
    mut e: *mut edge_t,
    mut et: libc::c_int,
    mut endp: *mut pathend_t,
    mut merge: bool,
) {
    let mut side: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut pboxfn: Option::<
        unsafe extern "C" fn(
            *mut node_t,
            *mut port,
            libc::c_int,
            *mut boxf,
            *mut libc::c_int,
        ) -> libc::c_int,
    > = None;
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.dyna {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port = resolvePort(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            &mut (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port,
        );
    }
    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null() {
        pboxfn = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
            .pboxfn;
    } else {
        pboxfn = None;
    }
    (*P)
        .start
        .p = add_pointf(
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
    );
    if merge {
        (*P)
            .start
            .theta = conc_slope(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node,
        );
        (*P).start.constrained = 1 as libc::c_int != 0;
    } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.constrained
        {
        (*P)
            .start
            .theta = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port
            .theta;
        (*P).start.constrained = 1 as libc::c_int != 0;
    } else {
        (*P).start.constrained = 0 as libc::c_int != 0;
    }
    (*P).nbox = 0 as libc::c_int;
    let ref mut fresh11 = (*P).data;
    *fresh11 = e as *mut libc::c_void;
    (*endp).np = (*P).start.p;
    if et == 1 as libc::c_int
        && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 0 as libc::c_int
        && {
            side = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
                as libc::c_int;
            side != 0
        }
    {
        let mut orig: *mut edge_t = 0 as *mut edge_t;
        let mut b0: boxf = boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        };
        let mut b: boxf = (*endp).nb;
        if side & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            (*endp).sidemask = (1 as libc::c_int) << 2 as libc::c_int;
            if (*P).start.p.x
                < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            {
                b0.LL.x = b.LL.x - 1 as libc::c_int as libc::c_double;
                b0.LL.y = (*P).start.p.y;
                b0.UR.x = b.UR.x;
                b0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double
                    + ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ranksep / 2 as libc::c_int) as libc::c_double;
                b
                    .UR
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (2 as libc::c_int - 2 as libc::c_int) as libc::c_double;
                b.UR.y = b0.LL.y;
                b
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b.LL.x -= 1.;
                (*endp).boxes[0 as libc::c_int as usize] = b0;
                (*endp).boxes[1 as libc::c_int as usize] = b;
            } else {
                b0.LL.x = b.LL.x;
                b0.LL.y = (*P).start.p.y;
                b0.UR.x = b.UR.x + 1 as libc::c_int as libc::c_double;
                b0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double
                    + ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ranksep / 2 as libc::c_int) as libc::c_double;
                b
                    .LL
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    + (2 as libc::c_int - 2 as libc::c_int) as libc::c_double;
                b.UR.y = b0.LL.y;
                b
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b.UR.x += 1.;
                (*endp).boxes[0 as libc::c_int as usize] = b0;
                (*endp).boxes[1 as libc::c_int as usize] = b;
            }
            let ref mut fresh12 = (*P).start.p.y;
            *fresh12 += 1.;
            (*endp).boxn = 2 as libc::c_int;
        } else if side & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            (*endp).sidemask = (1 as libc::c_int) << 0 as libc::c_int;
            b.UR.y = if b.UR.y > (*P).start.p.y { b.UR.y } else { (*P).start.p.y };
            (*endp).boxes[0 as libc::c_int as usize] = b;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh13 = (*P).start.p.y;
            *fresh13 -= 1.;
        } else if side & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            (*endp).sidemask = (1 as libc::c_int) << 3 as libc::c_int;
            b.UR.x = (*P).start.p.x;
            b
                .LL
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double;
            b.UR.y = (*P).start.p.y;
            (*endp).boxes[0 as libc::c_int as usize] = b;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh14 = (*P).start.p.x;
            *fresh14 -= 1.;
        } else {
            (*endp).sidemask = (1 as libc::c_int) << 1 as libc::c_int;
            b.LL.x = (*P).start.p.x;
            b
                .LL
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double;
            b.UR.y = (*P).start.p.y;
            (*endp).boxes[0 as libc::c_int as usize] = b;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh15 = (*P).start.p.x;
            *fresh15 += 1.;
        }
        orig = e;
        while !((*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig)
            .is_null()
            && (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
                as libc::c_int != 0 as libc::c_int
        {
            orig = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
        }
        if n
            == (*(if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                orig
            } else {
                orig.offset(1 as libc::c_int as isize)
            }))
                .node
        {
            (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port
                .clip = 0 as libc::c_int != 0;
        } else {
            (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port
                .clip = 0 as libc::c_int != 0;
        }
        return;
    }
    if et == 2 as libc::c_int
        && {
            side = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
                as libc::c_int;
            side != 0
        }
    {
        let mut b0_0: boxf = boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        };
        let mut b_0: boxf = (*endp).nb;
        let mut orig_0: *mut edge_t = 0 as *mut edge_t;
        if side & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            b_0.LL.y = if b_0.LL.y < (*P).start.p.y { b_0.LL.y } else { (*P).start.p.y };
            (*endp).boxes[0 as libc::c_int as usize] = b_0;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh16 = (*P).start.p.y;
            *fresh16 += 1.;
        } else if side & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                b0_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b0_0.UR.x = b_0.UR.x + 1 as libc::c_int as libc::c_double;
                b0_0.LL.x = (*P).start.p.x;
                b0_0
                    .LL
                    .y = b0_0.UR.y
                    - ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ranksep / 2 as libc::c_int) as libc::c_double;
                b_0
                    .LL
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    + (2 as libc::c_int - 2 as libc::c_int) as libc::c_double;
                b_0.LL.y = b0_0.UR.y;
                b_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.UR.x += 1.;
                (*endp).boxes[0 as libc::c_int as usize] = b0_0;
                (*endp).boxes[1 as libc::c_int as usize] = b_0;
                (*endp).boxn = 2 as libc::c_int;
            } else {
                b_0
                    .UR
                    .y = if b_0.UR.y > (*P).start.p.y {
                    b_0.UR.y
                } else {
                    (*P).start.p.y
                };
                (*endp).boxes[0 as libc::c_int as usize] = b_0;
                (*endp).boxn = 1 as libc::c_int;
            }
            let ref mut fresh17 = (*P).start.p.y;
            *fresh17 -= 1.;
        } else if side & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            b_0.UR.x = (*P).start.p.x + 1 as libc::c_int as libc::c_double;
            if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                b_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.LL.y = (*P).start.p.y - 1 as libc::c_int as libc::c_double;
            } else {
                b_0
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.UR.y = (*P).start.p.y + 1 as libc::c_int as libc::c_double;
            }
            (*endp).boxes[0 as libc::c_int as usize] = b_0;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh18 = (*P).start.p.x;
            *fresh18 -= 1.;
        } else {
            b_0.LL.x = (*P).start.p.x;
            if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                b_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.LL.y = (*P).start.p.y;
            } else {
                b_0
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.UR.y = (*P).start.p.y + 1 as libc::c_int as libc::c_double;
            }
            (*endp).boxes[0 as libc::c_int as usize] = b_0;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh19 = (*P).start.p.x;
            *fresh19 += 1.;
        }
        orig_0 = e;
        while !((*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig)
            .is_null()
            && (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
                as libc::c_int != 0 as libc::c_int
        {
            orig_0 = (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
        }
        if n
            == (*(if ((*(orig_0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                orig_0
            } else {
                orig_0.offset(1 as libc::c_int as isize)
            }))
                .node
        {
            (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port
                .clip = 0 as libc::c_int != 0;
        } else {
            (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port
                .clip = 0 as libc::c_int != 0;
        }
        (*endp).sidemask = side;
        return;
    }
    if et == 1 as libc::c_int {
        side = (1 as libc::c_int) << 0 as libc::c_int;
    } else {
        side = (*endp).sidemask;
    }
    if pboxfn.is_some()
        && {
            mask = pboxfn
                .expect(
                    "non-null function pointer",
                )(
                n,
                &mut (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port,
                side,
                &mut *((*endp).boxes).as_mut_ptr().offset(0 as libc::c_int as isize),
                &mut (*endp).boxn,
            );
            mask != 0
        }
    {
        (*endp).sidemask = mask;
    } else {
        (*endp).boxes[0 as libc::c_int as usize] = (*endp).nb;
        (*endp).boxn = 1 as libc::c_int;
        match et {
            8 => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"splines.c\0" as *const u8 as *const libc::c_char,
                    569 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 58],
                        &[libc::c_char; 58],
                    >(b"void beginpath(path *, edge_t *, int, pathend_t *, _Bool)\0"))
                        .as_ptr(),
                );
                (*endp)
                    .boxes[0 as libc::c_int as usize]
                    .UR
                    .y = (*P).start.p.y - 1 as libc::c_int as libc::c_double;
                (*endp).sidemask = (1 as libc::c_int) << 0 as libc::c_int;
            }
            2 => {
                if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                    (*endp).boxes[0 as libc::c_int as usize].LL.y = (*P).start.p.y;
                } else {
                    (*endp).boxes[0 as libc::c_int as usize].UR.y = (*P).start.p.y;
                }
            }
            1 => {
                (*endp).boxes[0 as libc::c_int as usize].UR.y = (*P).start.p.y;
                (*endp).sidemask = (1 as libc::c_int) << 0 as libc::c_int;
                let ref mut fresh20 = (*P).start.p.y;
                *fresh20 -= 1.;
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn endpath(
    mut P: *mut path,
    mut e: *mut edge_t,
    mut et: libc::c_int,
    mut endp: *mut pathend_t,
    mut merge: bool,
) {
    let mut side: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut pboxfn: Option::<
        unsafe extern "C" fn(
            *mut node_t,
            *mut port,
            libc::c_int,
            *mut boxf,
            *mut libc::c_int,
        ) -> libc::c_int,
    > = None;
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.dyna {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port = resolvePort(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node,
            &mut (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port,
        );
    }
    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null() {
        pboxfn = (*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns)
            .pboxfn;
    } else {
        pboxfn = None;
    }
    (*P)
        .end
        .p = add_pointf(
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
    );
    if merge {
        (*P)
            .end
            .theta = conc_slope(
            (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node,
        ) + 3.14159265358979323846f64;
        if (*P).end.theta
            < 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        {} else {
            __assert_fail(
                b"P->end.theta < 2 * M_PI\0" as *const u8 as *const libc::c_char,
                b"splines.c\0" as *const u8 as *const libc::c_char,
                606 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void endpath(path *, edge_t *, int, pathend_t *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        (*P).end.constrained = 1 as libc::c_int != 0;
    } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.constrained
        {
        (*P)
            .end
            .theta = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port
            .theta;
        (*P).end.constrained = 1 as libc::c_int != 0;
    } else {
        (*P).end.constrained = 0 as libc::c_int != 0;
    }
    (*endp).np = (*P).end.p;
    if et == 1 as libc::c_int
        && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 0 as libc::c_int
        && {
            side = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                as libc::c_int;
            side != 0
        }
    {
        let mut orig: *mut edge_t = 0 as *mut edge_t;
        let mut b0: boxf = boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        };
        let mut b: boxf = (*endp).nb;
        if side & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            (*endp).sidemask = (1 as libc::c_int) << 2 as libc::c_int;
            b.LL.y = if b.LL.y < (*P).end.p.y { b.LL.y } else { (*P).end.p.y };
            (*endp).boxes[0 as libc::c_int as usize] = b;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh21 = (*P).end.p.y;
            *fresh21 += 1.;
        } else if side & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            (*endp).sidemask = (1 as libc::c_int) << 0 as libc::c_int;
            if (*P).end.p.x
                < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            {
                b0.LL.x = b.LL.x - 1 as libc::c_int as libc::c_double;
                b0.UR.y = (*P).end.p.y;
                b0.UR.x = b.UR.x;
                b0
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double
                    - ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ranksep / 2 as libc::c_int) as libc::c_double;
                b
                    .UR
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (2 as libc::c_int - 2 as libc::c_int) as libc::c_double;
                b.LL.y = b0.UR.y;
                b
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b.LL.x -= 1.;
                (*endp).boxes[0 as libc::c_int as usize] = b0;
                (*endp).boxes[1 as libc::c_int as usize] = b;
            } else {
                b0.LL.x = b.LL.x;
                b0.UR.y = (*P).end.p.y;
                b0.UR.x = b.UR.x + 1 as libc::c_int as libc::c_double;
                b0
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double
                    - ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ranksep / 2 as libc::c_int) as libc::c_double;
                b
                    .LL
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    + (2 as libc::c_int - 2 as libc::c_int) as libc::c_double;
                b.LL.y = b0.UR.y;
                b
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b.UR.x += 1.;
                (*endp).boxes[0 as libc::c_int as usize] = b0;
                (*endp).boxes[1 as libc::c_int as usize] = b;
            }
            (*endp).boxn = 2 as libc::c_int;
            let ref mut fresh22 = (*P).end.p.y;
            *fresh22 -= 1.;
        } else if side & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            (*endp).sidemask = (1 as libc::c_int) << 3 as libc::c_int;
            b.UR.x = (*P).end.p.x;
            b
                .UR
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double;
            b.LL.y = (*P).end.p.y;
            (*endp).boxes[0 as libc::c_int as usize] = b;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh23 = (*P).end.p.x;
            *fresh23 -= 1.;
        } else {
            (*endp).sidemask = (1 as libc::c_int) << 1 as libc::c_int;
            b.LL.x = (*P).end.p.x;
            b
                .UR
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double;
            b.LL.y = (*P).end.p.y;
            (*endp).boxes[0 as libc::c_int as usize] = b;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh24 = (*P).end.p.x;
            *fresh24 += 1.;
        }
        orig = e;
        while !((*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig)
            .is_null()
            && (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
                as libc::c_int != 0 as libc::c_int
        {
            orig = (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
        }
        if n
            == (*(if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                orig
            } else {
                orig.offset(-(1 as libc::c_int as isize))
            }))
                .node
        {
            (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port
                .clip = 0 as libc::c_int != 0;
        } else {
            (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port
                .clip = 0 as libc::c_int != 0;
        }
        (*endp).sidemask = side;
        return;
    }
    if et == 2 as libc::c_int
        && {
            side = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                as libc::c_int;
            side != 0
        }
    {
        let mut b0_0: boxf = boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        };
        let mut b_0: boxf = (*endp).nb;
        let mut orig_0: *mut edge_t = 0 as *mut edge_t;
        if side & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            b_0.LL.y = if b_0.LL.y < (*P).end.p.y { b_0.LL.y } else { (*P).end.p.y };
            (*endp).boxes[0 as libc::c_int as usize] = b_0;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh25 = (*P).end.p.y;
            *fresh25 += 1.;
        } else if side & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                b0_0.LL.x = b_0.LL.x - 1 as libc::c_int as libc::c_double;
                b0_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b0_0.UR.x = (*P).end.p.x;
                b0_0
                    .LL
                    .y = b0_0.UR.y
                    - ((*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .ranksep / 2 as libc::c_int) as libc::c_double;
                b_0
                    .UR
                    .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - 2 as libc::c_int as libc::c_double;
                b_0.LL.y = b0_0.UR.y;
                b_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.LL.x -= 1.;
                (*endp).boxes[0 as libc::c_int as usize] = b0_0;
                (*endp).boxes[1 as libc::c_int as usize] = b_0;
                (*endp).boxn = 2 as libc::c_int;
            } else {
                b_0
                    .UR
                    .y = if b_0.UR.y > (*P).start.p.y {
                    b_0.UR.y
                } else {
                    (*P).start.p.y
                };
                (*endp).boxes[0 as libc::c_int as usize] = b_0;
                (*endp).boxn = 1 as libc::c_int;
            }
            let ref mut fresh26 = (*P).end.p.y;
            *fresh26 -= 1.;
        } else if side & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            b_0.UR.x = (*P).end.p.x + 1 as libc::c_int as libc::c_double;
            if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                b_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.LL.y = (*P).end.p.y - 1 as libc::c_int as libc::c_double;
            } else {
                b_0
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.UR.y = (*P).end.p.y + 1 as libc::c_int as libc::c_double;
            }
            (*endp).boxes[0 as libc::c_int as usize] = b_0;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh27 = (*P).end.p.x;
            *fresh27 -= 1.;
        } else {
            b_0.LL.x = (*P).end.p.x - 1 as libc::c_int as libc::c_double;
            if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                b_0
                    .UR
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.LL.y = (*P).end.p.y - 1 as libc::c_int as libc::c_double;
            } else {
                b_0
                    .LL
                    .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                        / 2 as libc::c_int as libc::c_double;
                b_0.UR.y = (*P).end.p.y;
            }
            (*endp).boxes[0 as libc::c_int as usize] = b_0;
            (*endp).boxn = 1 as libc::c_int;
            let ref mut fresh28 = (*P).end.p.x;
            *fresh28 += 1.;
        }
        orig_0 = e;
        while !((*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig)
            .is_null()
            && (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
                as libc::c_int != 0 as libc::c_int
        {
            orig_0 = (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
        }
        if n
            == (*(if ((*(orig_0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                orig_0
            } else {
                orig_0.offset(-(1 as libc::c_int as isize))
            }))
                .node
        {
            (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port
                .clip = 0 as libc::c_int != 0;
        } else {
            (*((*(orig_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port
                .clip = 0 as libc::c_int != 0;
        }
        (*endp).sidemask = side;
        return;
    }
    if et == 1 as libc::c_int {
        side = (1 as libc::c_int) << 2 as libc::c_int;
    } else {
        side = (*endp).sidemask;
    }
    if pboxfn.is_some()
        && {
            mask = pboxfn
                .expect(
                    "non-null function pointer",
                )(
                n,
                &mut (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port,
                side,
                &mut *((*endp).boxes).as_mut_ptr().offset(0 as libc::c_int as isize),
                &mut (*endp).boxn,
            );
            mask != 0
        }
    {
        (*endp).sidemask = mask;
    } else {
        (*endp).boxes[0 as libc::c_int as usize] = (*endp).nb;
        (*endp).boxn = 1 as libc::c_int;
        match et {
            8 => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"splines.c\0" as *const u8 as *const libc::c_char,
                    767 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 56],
                        &[libc::c_char; 56],
                    >(b"void endpath(path *, edge_t *, int, pathend_t *, _Bool)\0"))
                        .as_ptr(),
                );
                (*endp)
                    .boxes[0 as libc::c_int as usize]
                    .LL
                    .y = (*P).end.p.y + 1 as libc::c_int as libc::c_double;
                (*endp).sidemask = (1 as libc::c_int) << 2 as libc::c_int;
            }
            2 => {
                if (*endp).sidemask == (1 as libc::c_int) << 2 as libc::c_int {
                    (*endp).boxes[0 as libc::c_int as usize].LL.y = (*P).end.p.y;
                } else {
                    (*endp).boxes[0 as libc::c_int as usize].UR.y = (*P).end.p.y;
                }
            }
            1 => {
                (*endp).boxes[0 as libc::c_int as usize].LL.y = (*P).end.p.y;
                (*endp).sidemask = (1 as libc::c_int) << 2 as libc::c_int;
                let ref mut fresh29 = (*P).end.p.y;
                *fresh29 += 1.;
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn convert_sides_to_points(
    mut tail_side: libc::c_int,
    mut head_side: libc::c_int,
) -> libc::c_int {
    let mut vertices: [libc::c_int; 8] = [
        12 as libc::c_int,
        4 as libc::c_int,
        6 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        9 as libc::c_int,
        8 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut tail_i: libc::c_int = 0;
    let mut head_i: libc::c_int = 0;
    let mut pair_a: [[libc::c_int; 8]; 8] = [
        [
            11 as libc::c_int,
            12 as libc::c_int,
            13 as libc::c_int,
            14 as libc::c_int,
            15 as libc::c_int,
            16 as libc::c_int,
            17 as libc::c_int,
            18 as libc::c_int,
        ],
        [
            21 as libc::c_int,
            22 as libc::c_int,
            23 as libc::c_int,
            24 as libc::c_int,
            25 as libc::c_int,
            26 as libc::c_int,
            27 as libc::c_int,
            28 as libc::c_int,
        ],
        [
            31 as libc::c_int,
            32 as libc::c_int,
            33 as libc::c_int,
            34 as libc::c_int,
            35 as libc::c_int,
            36 as libc::c_int,
            37 as libc::c_int,
            38 as libc::c_int,
        ],
        [
            41 as libc::c_int,
            42 as libc::c_int,
            43 as libc::c_int,
            44 as libc::c_int,
            45 as libc::c_int,
            46 as libc::c_int,
            47 as libc::c_int,
            48 as libc::c_int,
        ],
        [
            51 as libc::c_int,
            52 as libc::c_int,
            53 as libc::c_int,
            54 as libc::c_int,
            55 as libc::c_int,
            56 as libc::c_int,
            57 as libc::c_int,
            58 as libc::c_int,
        ],
        [
            61 as libc::c_int,
            62 as libc::c_int,
            63 as libc::c_int,
            64 as libc::c_int,
            65 as libc::c_int,
            66 as libc::c_int,
            67 as libc::c_int,
            68 as libc::c_int,
        ],
        [
            71 as libc::c_int,
            72 as libc::c_int,
            73 as libc::c_int,
            74 as libc::c_int,
            75 as libc::c_int,
            76 as libc::c_int,
            77 as libc::c_int,
            78 as libc::c_int,
        ],
        [
            81 as libc::c_int,
            82 as libc::c_int,
            83 as libc::c_int,
            84 as libc::c_int,
            85 as libc::c_int,
            86 as libc::c_int,
            87 as libc::c_int,
            88 as libc::c_int,
        ],
    ];
    head_i = -(1 as libc::c_int);
    tail_i = head_i;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if head_side == vertices[i as usize] {
            head_i = i;
            break;
        } else {
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if tail_side == vertices[i as usize] {
            tail_i = i;
            break;
        } else {
            i += 1;
        }
    }
    if tail_i < 0 as libc::c_int || head_i < 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        return pair_a[tail_i as usize][head_i as usize]
    };
}
unsafe extern "C" fn selfBottom(
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut sizex: libc::c_double,
    mut stepy: libc::c_double,
    mut sinfo: *mut splineInfo,
) {
    let mut tp: pointf = pointf { x: 0., y: 0. };
    let mut hp: pointf = pointf { x: 0., y: 0. };
    let mut np: pointf = pointf { x: 0., y: 0. };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut sgn: libc::c_int = 0;
    let mut point_pair: libc::c_int = 0;
    let mut hy: libc::c_double = 0.;
    let mut ty: libc::c_double = 0.;
    let mut stepx: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut height: libc::c_double = 0.;
    let mut points: [pointf; 1000] = [pointf { x: 0., y: 0. }; 1000];
    let mut pointn: libc::c_int = 0;
    e = *edges.offset(ind as isize);
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    stepx = fmax(sizex / 2.0f64 / cnt as libc::c_double, 2.0f64);
    np = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    tp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p;
    tp.x += np.x;
    tp.y += np.y;
    hp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p;
    hp.x += np.x;
    hp.y += np.y;
    if tp.x >= hp.x {
        sgn = 1 as libc::c_int;
    } else {
        sgn = -(1 as libc::c_int);
    }
    dy = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
    dx = 0.0f64;
    point_pair = convert_sides_to_points(
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
            as libc::c_int,
    );
    match point_pair {
        67 => {
            sgn = -sgn;
        }
        _ => {}
    }
    ty = fmin(dy, 3 as libc::c_int as libc::c_double * (tp.y + dy - np.y));
    hy = fmin(dy, 3 as libc::c_int as libc::c_double * (hp.y + dy - np.y));
    i = 0 as libc::c_int;
    while i < cnt {
        let fresh30 = ind;
        ind = ind + 1;
        e = *edges.offset(fresh30 as isize);
        dy += stepy;
        ty += stepy;
        hy += stepy;
        dx += sgn as libc::c_double * stepx;
        pointn = 0 as libc::c_int;
        let fresh31 = pointn;
        pointn = pointn + 1;
        points[fresh31 as usize] = tp;
        let fresh32 = pointn;
        pointn = pointn + 1;
        points[fresh32
            as usize] = pointfof(
            tp.x + dx,
            tp.y - ty / 3 as libc::c_int as libc::c_double,
        );
        let fresh33 = pointn;
        pointn = pointn + 1;
        points[fresh33 as usize] = pointfof(tp.x + dx, np.y - dy);
        let fresh34 = pointn;
        pointn = pointn + 1;
        points[fresh34
            as usize] = pointfof(
            (tp.x + hp.x) / 2 as libc::c_int as libc::c_double,
            np.y - dy,
        );
        let fresh35 = pointn;
        pointn = pointn + 1;
        points[fresh35 as usize] = pointfof(hp.x - dx, np.y - dy);
        let fresh36 = pointn;
        pointn = pointn + 1;
        points[fresh36
            as usize] = pointfof(
            hp.x - dx,
            hp.y - hy / 3 as libc::c_int as libc::c_double,
        );
        let fresh37 = pointn;
        pointn = pointn + 1;
        points[fresh37 as usize] = hp;
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
            if (*((*(agraphof(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut libc::c_void,
            ) as *mut Agobj_t))
                .data as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                height = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .x;
            } else {
                height = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .y;
            }
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y - dy
                - height / 2.0f64;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .set = 0 as libc::c_int == 0;
            if height > stepy {
                dy += height - stepy;
            }
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
            points.as_mut_ptr(),
            pointn,
            sinfo,
        );
        i += 1;
    }
}
unsafe extern "C" fn selfTop(
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut sizex: libc::c_double,
    mut stepy: libc::c_double,
    mut sinfo: *mut splineInfo,
) {
    let mut i: libc::c_int = 0;
    let mut sgn: libc::c_int = 0;
    let mut point_pair: libc::c_int = 0;
    let mut hy: libc::c_double = 0.;
    let mut ty: libc::c_double = 0.;
    let mut stepx: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut height: libc::c_double = 0.;
    let mut tp: pointf = pointf { x: 0., y: 0. };
    let mut hp: pointf = pointf { x: 0., y: 0. };
    let mut np: pointf = pointf { x: 0., y: 0. };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut points: [pointf; 1000] = [pointf { x: 0., y: 0. }; 1000];
    let mut pointn: libc::c_int = 0;
    e = *edges.offset(ind as isize);
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    stepx = fmax(sizex / 2.0f64 / cnt as libc::c_double, 2.0f64);
    np = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    tp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p;
    tp.x += np.x;
    tp.y += np.y;
    hp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p;
    hp.x += np.x;
    hp.y += np.y;
    if tp.x >= hp.x {
        sgn = 1 as libc::c_int;
    } else {
        sgn = -(1 as libc::c_int);
    }
    dy = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
    dx = 0.0f64;
    point_pair = convert_sides_to_points(
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
            as libc::c_int,
    );
    match point_pair {
        15 => {
            dx = sgn as libc::c_double
                * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    - (hp.x - np.x) + stepx);
        }
        38 => {
            dx = sgn as libc::c_double
                * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (np.x - hp.x) + stepx);
        }
        41 => {
            dx = sgn as libc::c_double
                * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    - (tp.x - np.x) + stepx);
        }
        48 => {
            dx = sgn as libc::c_double
                * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                    - (tp.x - np.x) + stepx);
        }
        14 | 37 | 47 | 51 | 57 | 58 => {
            dx = sgn as libc::c_double
                * (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (np.x - tp.x)
                    + ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                        - (hp.x - np.x))) / 3.0f64);
        }
        73 => {
            dx = sgn as libc::c_double
                * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (np.x - tp.x) + stepx);
        }
        83 => {
            dx = sgn as libc::c_double
                * ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (np.x - tp.x));
        }
        84 => {
            dx = sgn as libc::c_double
                * (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (np.x - tp.x)
                    + ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                        - (hp.x - np.x))) / 2.0f64 + stepx);
        }
        74 | 75 | 85 => {
            dx = sgn as libc::c_double
                * (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                    - (np.x - tp.x)
                    + ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                        - (hp.x - np.x))) / 2.0f64
                    + 2 as libc::c_int as libc::c_double * stepx);
        }
        _ => {}
    }
    ty = fmin(dy, 3 as libc::c_int as libc::c_double * (np.y + dy - tp.y));
    hy = fmin(dy, 3 as libc::c_int as libc::c_double * (np.y + dy - hp.y));
    i = 0 as libc::c_int;
    while i < cnt {
        let fresh38 = ind;
        ind = ind + 1;
        e = *edges.offset(fresh38 as isize);
        dy += stepy;
        ty += stepy;
        hy += stepy;
        dx += sgn as libc::c_double * stepx;
        pointn = 0 as libc::c_int;
        let fresh39 = pointn;
        pointn = pointn + 1;
        points[fresh39 as usize] = tp;
        let fresh40 = pointn;
        pointn = pointn + 1;
        points[fresh40
            as usize] = pointfof(
            tp.x + dx,
            tp.y + ty / 3 as libc::c_int as libc::c_double,
        );
        let fresh41 = pointn;
        pointn = pointn + 1;
        points[fresh41 as usize] = pointfof(tp.x + dx, np.y + dy);
        let fresh42 = pointn;
        pointn = pointn + 1;
        points[fresh42
            as usize] = pointfof(
            (tp.x + hp.x) / 2 as libc::c_int as libc::c_double,
            np.y + dy,
        );
        let fresh43 = pointn;
        pointn = pointn + 1;
        points[fresh43 as usize] = pointfof(hp.x - dx, np.y + dy);
        let fresh44 = pointn;
        pointn = pointn + 1;
        points[fresh44
            as usize] = pointfof(
            hp.x - dx,
            hp.y + hy / 3 as libc::c_int as libc::c_double,
        );
        let fresh45 = pointn;
        pointn = pointn + 1;
        points[fresh45 as usize] = hp;
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
            if (*((*(agraphof(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut libc::c_void,
            ) as *mut Agobj_t))
                .data as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                height = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .x;
            } else {
                height = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .y;
            }
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y + dy
                + height / 2.0f64;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .set = 0 as libc::c_int == 0;
            if height > stepy {
                dy += height - stepy;
            }
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
            points.as_mut_ptr(),
            pointn,
            sinfo,
        );
        i += 1;
    }
}
unsafe extern "C" fn selfRight(
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut stepx: libc::c_double,
    mut sizey: libc::c_double,
    mut sinfo: *mut splineInfo,
) {
    let mut i: libc::c_int = 0;
    let mut sgn: libc::c_int = 0;
    let mut point_pair: libc::c_int = 0;
    let mut hx: libc::c_double = 0.;
    let mut tx: libc::c_double = 0.;
    let mut stepy: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut width: libc::c_double = 0.;
    let mut tp: pointf = pointf { x: 0., y: 0. };
    let mut hp: pointf = pointf { x: 0., y: 0. };
    let mut np: pointf = pointf { x: 0., y: 0. };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut points: [pointf; 1000] = [pointf { x: 0., y: 0. }; 1000];
    let mut pointn: libc::c_int = 0;
    e = *edges.offset(ind as isize);
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    stepy = fmax(sizey / 2.0f64 / cnt as libc::c_double, 2.0f64);
    np = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    tp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p;
    tp.x += np.x;
    tp.y += np.y;
    hp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p;
    hp.x += np.x;
    hp.y += np.y;
    if tp.y >= hp.y {
        sgn = 1 as libc::c_int;
    } else {
        sgn = -(1 as libc::c_int);
    }
    dx = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
    dy = 0 as libc::c_int as libc::c_double;
    point_pair = convert_sides_to_points(
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
            as libc::c_int,
    );
    match point_pair {
        32 | 65 => {
            if tp.y == hp.y {
                sgn = -sgn;
            }
        }
        _ => {}
    }
    tx = fmin(dx, 3 as libc::c_int as libc::c_double * (np.x + dx - tp.x));
    hx = fmin(dx, 3 as libc::c_int as libc::c_double * (np.x + dx - hp.x));
    i = 0 as libc::c_int;
    while i < cnt {
        let fresh46 = ind;
        ind = ind + 1;
        e = *edges.offset(fresh46 as isize);
        dx += stepx;
        tx += stepx;
        hx += stepx;
        dy += sgn as libc::c_double * stepy;
        pointn = 0 as libc::c_int;
        let fresh47 = pointn;
        pointn = pointn + 1;
        points[fresh47 as usize] = tp;
        let fresh48 = pointn;
        pointn = pointn + 1;
        points[fresh48
            as usize] = pointfof(
            tp.x + tx / 3 as libc::c_int as libc::c_double,
            tp.y + dy,
        );
        let fresh49 = pointn;
        pointn = pointn + 1;
        points[fresh49 as usize] = pointfof(np.x + dx, tp.y + dy);
        let fresh50 = pointn;
        pointn = pointn + 1;
        points[fresh50
            as usize] = pointfof(
            np.x + dx,
            (tp.y + hp.y) / 2 as libc::c_int as libc::c_double,
        );
        let fresh51 = pointn;
        pointn = pointn + 1;
        points[fresh51 as usize] = pointfof(np.x + dx, hp.y - dy);
        let fresh52 = pointn;
        pointn = pointn + 1;
        points[fresh52
            as usize] = pointfof(
            hp.x + hx / 3 as libc::c_int as libc::c_double,
            hp.y - dy,
        );
        let fresh53 = pointn;
        pointn = pointn + 1;
        points[fresh53 as usize] = hp;
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
            if (*((*(agraphof(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut libc::c_void,
            ) as *mut Agobj_t))
                .data as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                width = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .y;
            } else {
                width = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .x;
            }
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x + dx
                + width / 2.0f64;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .set = 0 as libc::c_int == 0;
            if width > stepx {
                dx += width - stepx;
            }
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
            points.as_mut_ptr(),
            pointn,
            sinfo,
        );
        i += 1;
    }
}
unsafe extern "C" fn selfLeft(
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut stepx: libc::c_double,
    mut sizey: libc::c_double,
    mut sinfo: *mut splineInfo,
) {
    let mut i: libc::c_int = 0;
    let mut sgn: libc::c_int = 0;
    let mut point_pair: libc::c_int = 0;
    let mut hx: libc::c_double = 0.;
    let mut tx: libc::c_double = 0.;
    let mut stepy: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut width: libc::c_double = 0.;
    let mut tp: pointf = pointf { x: 0., y: 0. };
    let mut hp: pointf = pointf { x: 0., y: 0. };
    let mut np: pointf = pointf { x: 0., y: 0. };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut points: [pointf; 1000] = [pointf { x: 0., y: 0. }; 1000];
    let mut pointn: libc::c_int = 0;
    e = *edges.offset(ind as isize);
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    stepy = fmax(sizey / 2.0f64 / cnt as libc::c_double, 2.0f64);
    np = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    tp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p;
    tp.x += np.x;
    tp.y += np.y;
    hp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p;
    hp.x += np.x;
    hp.y += np.y;
    if tp.y >= hp.y {
        sgn = 1 as libc::c_int;
    } else {
        sgn = -(1 as libc::c_int);
    }
    dx = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
    dy = 0.0f64;
    point_pair = convert_sides_to_points(
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
            as libc::c_int,
    );
    match point_pair {
        12 | 67 => {
            if tp.y == hp.y {
                sgn = -sgn;
            }
        }
        _ => {}
    }
    tx = fmin(dx, 3 as libc::c_int as libc::c_double * (tp.x + dx - np.x));
    hx = fmin(dx, 3 as libc::c_int as libc::c_double * (hp.x + dx - np.x));
    i = 0 as libc::c_int;
    while i < cnt {
        let fresh54 = ind;
        ind = ind + 1;
        e = *edges.offset(fresh54 as isize);
        dx += stepx;
        tx += stepx;
        hx += stepx;
        dy += sgn as libc::c_double * stepy;
        pointn = 0 as libc::c_int;
        let fresh55 = pointn;
        pointn = pointn + 1;
        points[fresh55 as usize] = tp;
        let fresh56 = pointn;
        pointn = pointn + 1;
        points[fresh56
            as usize] = pointfof(
            tp.x - tx / 3 as libc::c_int as libc::c_double,
            tp.y + dy,
        );
        let fresh57 = pointn;
        pointn = pointn + 1;
        points[fresh57 as usize] = pointfof(np.x - dx, tp.y + dy);
        let fresh58 = pointn;
        pointn = pointn + 1;
        points[fresh58
            as usize] = pointfof(
            np.x - dx,
            (tp.y + hp.y) / 2 as libc::c_int as libc::c_double,
        );
        let fresh59 = pointn;
        pointn = pointn + 1;
        points[fresh59 as usize] = pointfof(np.x - dx, hp.y - dy);
        let fresh60 = pointn;
        pointn = pointn + 1;
        points[fresh60
            as usize] = pointfof(
            hp.x - hx / 3 as libc::c_int as libc::c_double,
            hp.y - dy,
        );
        let fresh61 = pointn;
        pointn = pointn + 1;
        points[fresh61 as usize] = hp;
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
            if (*((*(agraphof(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut libc::c_void,
            ) as *mut Agobj_t))
                .data as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                width = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .y;
            } else {
                width = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .dimen
                    .x;
            }
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x - dx
                - width / 2.0f64;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .pos
                .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
            (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                .set = 0 as libc::c_int == 0;
            if width > stepx {
                dx += width - stepx;
            }
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
            points.as_mut_ptr(),
            pointn,
            sinfo,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn selfRightSpace(mut e: *mut edge_t) -> libc::c_double {
    let mut sw: libc::c_double = 0.;
    let mut label_width: libc::c_double = 0.;
    let mut l: *mut textlabel_t = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .label;
    if !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
        && !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
        || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int == 0
            && (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int == 0
            && ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
                as libc::c_int
                != (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                    as libc::c_int
                || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
                    as libc::c_int
                    & ((1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int) == 0)
    {
        sw = 18 as libc::c_int as libc::c_double;
        if !l.is_null() {
            label_width = if (*((*(agraphof(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut libc::c_void,
            ) as *mut Agobj_t))
                .data as *mut Agraphinfo_t))
                .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
            {
                (*l).dimen.y
            } else {
                (*l).dimen.x
            };
            sw += label_width;
        }
    } else {
        sw = 0 as libc::c_int as libc::c_double;
    }
    return sw;
}
#[no_mangle]
pub unsafe extern "C" fn makeSelfEdge(
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut sizex: libc::c_double,
    mut sizey: libc::c_double,
    mut sinfo: *mut splineInfo,
) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    e = *edges.offset(ind as isize);
    if !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
        && !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
        || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int == 0
            && (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int == 0
            && ((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
                as libc::c_int
                != (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                    as libc::c_int
                || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
                    as libc::c_int
                    & ((1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int) == 0)
    {
        selfRight(edges, ind, cnt, sizex, sizey, sinfo);
    } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0
            || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0
        {
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0
            || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
                as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            selfTop(edges, ind, cnt, sizex, sizey, sinfo);
        } else {
            selfLeft(edges, ind, cnt, sizex, sizey, sinfo);
        }
    } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
        {
        selfTop(edges, ind, cnt, sizex, sizey, sinfo);
    } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
            as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
        selfBottom(edges, ind, cnt, sizex, sizey, sinfo);
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"splines.c\0" as *const u8 as *const libc::c_char,
            1232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void makeSelfEdge(edge_t **, int, int, double, double, splineInfo *)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn makePortLabels(mut e: *mut edge_t) {
    if E_labelangle.is_null() && E_labeldistance.is_null() {
        return;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).is_null()
        && !(*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).set
    {
        if place_portlabel(e, 1 as libc::c_int != 0) != 0 {
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
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label,
            );
        }
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).is_null()
        && !(*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).set
    {
        if place_portlabel(e, 0 as libc::c_int != 0) != 0 {
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
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label,
            );
        }
    }
}
unsafe extern "C" fn endPoints(
    mut spl: *mut splines,
    mut p: *mut pointf,
    mut q: *mut pointf,
) {
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    bz = *((*spl).list).offset(0 as libc::c_int as isize);
    if bz.sflag != 0 {
        *p = bz.sp;
    } else {
        *p = *(bz.list).offset(0 as libc::c_int as isize);
    }
    bz = *((*spl).list).offset(((*spl).size - 1 as libc::c_int) as isize);
    if bz.eflag != 0 {
        *q = bz.ep;
    } else {
        *q = *(bz.list).offset((bz.size - 1 as libc::c_int) as isize);
    };
}
unsafe extern "C" fn polylineMidpoint(
    mut spl: *mut splines,
    mut pp: *mut pointf,
    mut pq: *mut pointf,
) -> pointf {
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut dist: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut pf: pointf = pointf { x: 0., y: 0. };
    let mut qf: pointf = pointf { x: 0., y: 0. };
    let mut mf: pointf = pointf { x: 0., y: 0. };
    i = 0 as libc::c_int;
    while i < (*spl).size {
        bz = *((*spl).list).offset(i as isize);
        j = 0 as libc::c_int;
        k = 3 as libc::c_int;
        while k < bz.size {
            pf = *(bz.list).offset(j as isize);
            qf = *(bz.list).offset(k as isize);
            dist += sqrt((pf.x - qf.x) * (pf.x - qf.x) + (pf.y - qf.y) * (pf.y - qf.y));
            j += 3 as libc::c_int;
            k += 3 as libc::c_int;
        }
        i += 1;
    }
    dist /= 2 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*spl).size {
        bz = *((*spl).list).offset(i as isize);
        j = 0 as libc::c_int;
        k = 3 as libc::c_int;
        while k < bz.size {
            pf = *(bz.list).offset(j as isize);
            qf = *(bz.list).offset(k as isize);
            d = sqrt((pf.x - qf.x) * (pf.x - qf.x) + (pf.y - qf.y) * (pf.y - qf.y));
            if d >= dist {
                *pp = pf;
                *pq = qf;
                mf.x = (qf.x * dist + pf.x * (d - dist)) / d;
                mf.y = (qf.y * dist + pf.y * (d - dist)) / d;
                return mf;
            } else {
                dist -= d;
            }
            j += 3 as libc::c_int;
            k += 3 as libc::c_int;
        }
        i += 1;
    }
    fprintf(
        stderr,
        b"%s:%d: claimed unreachable code was reached\0" as *const u8
            as *const libc::c_char,
        b"splines.c\0" as *const u8 as *const libc::c_char,
        1317 as libc::c_int,
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn edgeMidpoint(
    mut g: *mut graph_t,
    mut e: *mut edge_t,
) -> pointf {
    let mut et: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags
        as libc::c_int & (7 as libc::c_int) << 1 as libc::c_int;
    let mut d: pointf = pointf { x: 0., y: 0. };
    let mut spf: pointf = pointf { x: 0., y: 0. };
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut q: pointf = pointf { x: 0., y: 0. };
    endPoints((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl, &mut p, &mut q);
    if (p.x - q.x) * (p.x - q.x) + (p.y - q.y) * (p.y - q.y) < 0.001f64 * 0.001f64 {
        spf = p;
    } else if et == (5 as libc::c_int) << 1 as libc::c_int
            || et == (2 as libc::c_int) << 1 as libc::c_int
        {
        d.x = (q.x + p.x) / 2.0f64;
        d.y = (p.y + q.y) / 2.0f64;
        spf = dotneato_closest(
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl,
            d,
        );
    } else {
        spf = polylineMidpoint(
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl,
            &mut p,
            &mut q,
        );
    }
    return spf;
}
#[no_mangle]
pub unsafe extern "C" fn addEdgeLabels(mut e: *mut edge_t) {
    makePortLabels(e);
}
#[no_mangle]
pub unsafe extern "C" fn place_portlabel(
    mut e: *mut edge_t,
    mut head_p: bool,
) -> libc::c_int {
    let mut l: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut spl: *mut splines = 0 as *mut splines;
    let mut bez: *mut bezier = 0 as *mut bezier;
    let mut dist: libc::c_double = 0.;
    let mut angle: libc::c_double = 0.;
    let mut c: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut pe: pointf = pointf { x: 0., y: 0. };
    let mut pf: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut la: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ld: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type as libc::c_int
        == 6 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (E_labelangle.is_null()
        || {
            la = agxget(e as *mut libc::c_void, E_labelangle);
            *la as libc::c_int == '\0' as i32
        })
        && (E_labeldistance.is_null()
            || {
                ld = agxget(e as *mut libc::c_void, E_labeldistance);
                *ld as libc::c_int == '\0' as i32
            })
    {
        return 0 as libc::c_int;
    }
    l = if head_p as libc::c_int != 0 {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label
    } else {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label
    };
    spl = getsplinepoints(e);
    if spl.is_null() {
        return 0 as libc::c_int;
    }
    if !head_p {
        bez = &mut *((*spl).list).offset(0 as libc::c_int as isize) as *mut bezier;
        if (*bez).sflag != 0 {
            pe = (*bez).sp;
            pf = *((*bez).list).offset(0 as libc::c_int as isize);
        } else {
            pe = *((*bez).list).offset(0 as libc::c_int as isize);
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                c[i as usize] = *((*bez).list).offset(i as isize);
                i += 1;
            }
            pf = Bezier(
                c.as_mut_ptr(),
                3 as libc::c_int,
                0.1f64,
                0 as *mut pointf,
                0 as *mut pointf,
            );
        }
    } else {
        bez = &mut *((*spl).list).offset(((*spl).size - 1 as libc::c_int) as isize)
            as *mut bezier;
        if (*bez).eflag != 0 {
            pe = (*bez).ep;
            pf = *((*bez).list).offset(((*bez).size - 1 as libc::c_int) as isize);
        } else {
            pe = *((*bez).list).offset(((*bez).size - 1 as libc::c_int) as isize);
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                c[i
                    as usize] = *((*bez).list)
                    .offset(((*bez).size - 4 as libc::c_int + i) as isize);
                i += 1;
            }
            pf = Bezier(
                c.as_mut_ptr(),
                3 as libc::c_int,
                0.9f64,
                0 as *mut pointf,
                0 as *mut pointf,
            );
        }
    }
    angle = atan2(pf.y - pe.y, pf.x - pe.x)
        + late_double(
            e as *mut libc::c_void,
            E_labelangle,
            -(25 as libc::c_int) as libc::c_double,
            -180.0f64,
        ) / 180.0f64 * 3.14159265358979323846f64;
    dist = 10 as libc::c_int as libc::c_double
        * late_double(e as *mut libc::c_void, E_labeldistance, 1.0f64, 0.0f64);
    (*l).pos.x = pe.x + dist * cos(angle);
    (*l).pos.y = pe.y + dist * sin(angle);
    (*l).set = 1 as libc::c_int != 0;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getsplinepoints(mut e: *mut edge_t) -> *mut splines {
    let mut le: *mut edge_t = 0 as *mut edge_t;
    let mut sp: *mut splines = 0 as *mut splines;
    le = e;
    loop {
        sp = (*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
        if !(sp.is_null()
            && (*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
                as libc::c_int != 0 as libc::c_int)
        {
            break;
        }
        le = (*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    if sp.is_null() {
        agerr(
            AGERR,
            b"getsplinepoints: no spline points available for edge (%s,%s)\n\0"
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
    }
    return sp;
}
