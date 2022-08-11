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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut E_dir: *mut Agsym_t;
    static mut E_arrowsz: *mut Agsym_t;
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
    fn gvrender_set_penwidth(job: *mut GVJ_t, penwidth: libc::c_double);
    fn gvrender_set_style(job: *mut GVJ_t, s: *mut *mut libc::c_char);
    fn gvrender_ellipse(job: *mut GVJ_t, AF: *mut pointf, n: libc::c_int, filled: libc::c_int);
    fn gvrender_polygon(job: *mut GVJ_t, af: *mut pointf, n: libc::c_int, filled: libc::c_int);
    fn gvrender_beziercurve(
        job: *mut GVJ_t,
        AF: *mut pointf,
        n: libc::c_int,
        arrow_at_start: libc::c_int,
        arrow_at_end: libc::c_int,
        filled: libc::c_int,
    );
    fn gvrender_polyline(job: *mut GVJ_t, AF: *mut pointf, n: libc::c_int);
    fn bezier_clip(
        inside_context: *mut inside_t,
        insidefn: Option<unsafe extern "C" fn(*mut inside_t, pointf) -> bool>,
        sp: *mut pointf,
        left_inside: bool,
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
pub struct arrowname_t {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arrowdir_t {
    pub dir: *mut libc::c_char,
    pub sflag: libc::c_int,
    pub eflag: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arrowtype_t {
    pub type_0: libc::c_int,
    pub lenfact: libc::c_double,
    pub gen: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            pointf,
            pointf,
            libc::c_double,
            libc::c_double,
            libc::c_int,
        ) -> (),
    >,
}
static mut Arrowdirs: [arrowdir_t; 5] = [
    {
        let mut init = arrowdir_t {
            dir: b"forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            sflag: 0 as libc::c_int,
            eflag: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowdir_t {
            dir: b"back\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            sflag: 1 as libc::c_int,
            eflag: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowdir_t {
            dir: b"both\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            sflag: 1 as libc::c_int,
            eflag: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowdir_t {
            dir: b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            sflag: 0 as libc::c_int,
            eflag: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowdir_t {
            dir: 0 as *const libc::c_char as *mut libc::c_char,
            sflag: 0 as libc::c_int,
            eflag: 0 as libc::c_int,
        };
        init
    },
];
static mut Arrowsynonyms: [arrowname_t; 2] = [
    {
        let mut init = arrowname_t {
            name: b"invempty\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
];
static mut Arrowmods: [arrowname_t; 6] = [
    {
        let mut init = arrowname_t {
            name: b"o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: (1 as libc::c_int) << 4 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"l\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"half\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
];
static mut Arrownames: [arrowname_t; 14] = [
    {
        let mut init = arrowname_t {
            name: b"normal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"crow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"tee\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"box\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"diamond\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"dot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"inv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"vee\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"pen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 2 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"mpty\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"curve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: b"icurve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: 7 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = arrowname_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            type_0: 0 as libc::c_int,
        };
        init
    },
];
static mut Arrowtypes: [arrowtype_t; 8] = unsafe {
    [
        {
            let mut init = arrowtype_t {
                type_0: 1 as libc::c_int,
                lenfact: 1.0f64,
                gen: Some(
                    arrow_type_normal
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = arrowtype_t {
                type_0: 2 as libc::c_int,
                lenfact: 1.0f64,
                gen: Some(
                    arrow_type_crow
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = arrowtype_t {
                type_0: 3 as libc::c_int,
                lenfact: 0.5f64,
                gen: Some(
                    arrow_type_tee
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = arrowtype_t {
                type_0: 4 as libc::c_int,
                lenfact: 1.0f64,
                gen: Some(
                    arrow_type_box
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = arrowtype_t {
                type_0: 5 as libc::c_int,
                lenfact: 1.2f64,
                gen: Some(
                    arrow_type_diamond
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = arrowtype_t {
                type_0: 6 as libc::c_int,
                lenfact: 0.8f64,
                gen: Some(
                    arrow_type_dot
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = arrowtype_t {
                type_0: 7 as libc::c_int,
                lenfact: 1.0f64,
                gen: Some(
                    arrow_type_curve
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = arrowtype_t {
                type_0: 8 as libc::c_int,
                lenfact: 0.5f64,
                gen: Some(
                    arrow_type_gap
                        as unsafe extern "C" fn(
                            *mut GVJ_t,
                            pointf,
                            pointf,
                            libc::c_double,
                            libc::c_double,
                            libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
static mut Arrowtypes_size: size_t = 0;
unsafe extern "C" fn arrow_match_name_frag(
    mut name: *mut libc::c_char,
    mut arrownames: *mut arrowname_t,
    mut flag: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut arrowname: *mut arrowname_t = 0 as *mut arrowname_t;
    let mut namelen: size_t = 0 as libc::c_int as size_t;
    let mut rest: *mut libc::c_char = name;
    arrowname = arrownames;
    while !((*arrowname).name).is_null() {
        namelen = strlen((*arrowname).name);
        if strncmp(name, (*arrowname).name, namelen) == 0 as libc::c_int {
            *flag |= (*arrowname).type_0;
            rest = rest.offset(namelen as isize);
            break;
        } else {
            arrowname = arrowname.offset(1);
        }
    }
    return rest;
}
unsafe extern "C" fn arrow_match_shape(
    mut name: *mut libc::c_char,
    mut flag: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: libc::c_int = 0 as libc::c_int;
    rest = arrow_match_name_frag(name, Arrowsynonyms.as_mut_ptr(), &mut f);
    if rest == name {
        loop {
            next = rest;
            rest = arrow_match_name_frag(next, Arrowmods.as_mut_ptr(), &mut f);
            if !(next != rest) {
                break;
            }
        }
        rest = arrow_match_name_frag(rest, Arrownames.as_mut_ptr(), &mut f);
    }
    if f != 0 && f & ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int == 0 {
        f |= 1 as libc::c_int;
    }
    *flag |= f;
    return rest;
}
unsafe extern "C" fn arrow_match_name(mut name: *mut libc::c_char, mut flag: *mut libc::c_int) {
    let mut rest: *mut libc::c_char = name;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    *flag = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *rest as libc::c_int != '\0' as i32 && i < 4 as libc::c_int {
        f = 0 as libc::c_int;
        next = rest;
        rest = arrow_match_shape(next, &mut f);
        if f == 0 as libc::c_int {
            agerr(
                AGWARN,
                b"Arrow type \"%s\" unknown - ignoring\n\0" as *const u8 as *const libc::c_char,
                next,
            );
            return;
        }
        if f == 8 as libc::c_int && i == 4 as libc::c_int - 1 as libc::c_int {
            f = 0 as libc::c_int;
        }
        if f == 8 as libc::c_int && i == 0 as libc::c_int && *rest as libc::c_int == '\0' as i32 {
            f = 0 as libc::c_int;
        }
        if f != 0 as libc::c_int {
            let fresh0 = i;
            i = i + 1;
            *flag |= f << fresh0 * 8 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn arrow_flags(
    mut e: *mut Agedge_t,
    mut sflag: *mut libc::c_int,
    mut eflag: *mut libc::c_int,
) {
    let mut attr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arrowdir: *mut arrowdir_t = 0 as *mut arrowdir_t;
    *sflag = 0 as libc::c_int;
    *eflag = if agisdirected(agraphof(e as *mut libc::c_void)) != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if !E_dir.is_null() && {
        attr = agxget(e as *mut libc::c_void, E_dir);
        *attr.offset(0 as libc::c_int as isize) as libc::c_int != 0
    } {
        arrowdir = Arrowdirs.as_mut_ptr();
        while !((*arrowdir).dir).is_null() {
            if strcmp(attr, (*arrowdir).dir) == 0 {
                *sflag = (*arrowdir).sflag;
                *eflag = (*arrowdir).eflag;
                break;
            } else {
                arrowdir = arrowdir.offset(1);
            }
        }
    }
    if *eflag == 1 as libc::c_int {
        let mut arrowhead: *mut Agsym_t = agattr(
            agraphof(e as *mut libc::c_void),
            2 as libc::c_int,
            b"arrowhead\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
        if !arrowhead.is_null() && {
            attr = agxget(e as *mut libc::c_void, arrowhead);
            *attr.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            arrow_match_name(attr, eflag);
        }
    }
    if *sflag == 1 as libc::c_int {
        let mut arrowtail: *mut Agsym_t = agattr(
            agraphof(e as *mut libc::c_void),
            2 as libc::c_int,
            b"arrowtail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
        if !arrowtail.is_null() && {
            attr = agxget(e as *mut libc::c_void, arrowtail);
            *attr.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            arrow_match_name(attr, sflag);
        }
    }
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).conc_opp_flag {
        let mut f: *mut edge_t = 0 as *mut edge_t;
        let mut s0: libc::c_int = 0;
        let mut e0: libc::c_int = 0;
        f = agedge(
            agraphof(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                .node as *mut libc::c_void,
            ),
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
            .node,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
            .node,
            0 as *mut libc::c_char,
            0 as libc::c_int,
        );
        arrow_flags(f, &mut s0, &mut e0);
        *eflag = *eflag | s0;
        *sflag = *sflag | e0;
    }
}
unsafe extern "C" fn arrow_length(mut e: *mut edge_t, mut flag: libc::c_int) -> libc::c_double {
    let mut lenfact: libc::c_double = 0.0f64;
    let mut f: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        f = flag >> i * 8 as libc::c_int
            & ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int;
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < Arrowtypes_size {
            let mut arrowtype: *const arrowtype_t =
                &*Arrowtypes.as_ptr().offset(j as isize) as *const arrowtype_t;
            if f == (*arrowtype).type_0 {
                lenfact += (*arrowtype).lenfact;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        i += 1;
    }
    return 10.0f64 * lenfact * late_double(e as *mut libc::c_void, E_arrowsz, 1.0f64, 0.0f64);
}
unsafe extern "C" fn inside(mut inside_context: *mut inside_t, mut p: pointf) -> bool {
    return (p.x - (*((*inside_context).a.p).offset(0 as libc::c_int as isize)).x)
        * (p.x - (*((*inside_context).a.p).offset(0 as libc::c_int as isize)).x)
        + (p.y - (*((*inside_context).a.p).offset(0 as libc::c_int as isize)).y)
            * (p.y - (*((*inside_context).a.p).offset(0 as libc::c_int as isize)).y)
        <= *((*inside_context).a.r).offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn arrowEndClip(
    mut e: *mut edge_t,
    mut ps: *mut pointf,
    mut startp: libc::c_int,
    mut endp: libc::c_int,
    mut spl: *mut bezier,
    mut eflag: libc::c_int,
) -> libc::c_int {
    let mut inside_context: inside_t = inside_t {
        a: C2RustUnnamed_5 {
            p: 0 as *mut pointf,
            r: 0 as *mut libc::c_double,
        },
    };
    let mut sp: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut elen: libc::c_double = 0.;
    let mut elen2: libc::c_double = 0.;
    elen = arrow_length(e, eflag);
    elen2 = elen * elen;
    (*spl).eflag = eflag;
    (*spl).ep = *ps.offset((endp + 3 as libc::c_int) as isize);
    if endp > startp
        && ((*ps.offset(endp as isize)).x - (*ps.offset((endp + 3 as libc::c_int) as isize)).x)
            * ((*ps.offset(endp as isize)).x - (*ps.offset((endp + 3 as libc::c_int) as isize)).x)
            + ((*ps.offset(endp as isize)).y - (*ps.offset((endp + 3 as libc::c_int) as isize)).y)
                * ((*ps.offset(endp as isize)).y
                    - (*ps.offset((endp + 3 as libc::c_int) as isize)).y)
            < elen2
    {
        endp -= 3 as libc::c_int;
    }
    sp[3 as libc::c_int as usize] = *ps.offset(endp as isize);
    sp[2 as libc::c_int as usize] = *ps.offset((endp + 1 as libc::c_int) as isize);
    sp[1 as libc::c_int as usize] = *ps.offset((endp + 2 as libc::c_int) as isize);
    sp[0 as libc::c_int as usize] = (*spl).ep;
    inside_context.a.p = &mut *sp.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut pointf;
    inside_context.a.r = &mut elen2;
    bezier_clip(
        &mut inside_context,
        Some(inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool),
        sp.as_mut_ptr(),
        1 as libc::c_int != 0,
    );
    *ps.offset(endp as isize) = sp[3 as libc::c_int as usize];
    *ps.offset((endp + 1 as libc::c_int) as isize) = sp[2 as libc::c_int as usize];
    *ps.offset((endp + 2 as libc::c_int) as isize) = sp[1 as libc::c_int as usize];
    *ps.offset((endp + 3 as libc::c_int) as isize) = sp[0 as libc::c_int as usize];
    return endp;
}
#[no_mangle]
pub unsafe extern "C" fn arrowStartClip(
    mut e: *mut edge_t,
    mut ps: *mut pointf,
    mut startp: libc::c_int,
    mut endp: libc::c_int,
    mut spl: *mut bezier,
    mut sflag: libc::c_int,
) -> libc::c_int {
    let mut inside_context: inside_t = inside_t {
        a: C2RustUnnamed_5 {
            p: 0 as *mut pointf,
            r: 0 as *mut libc::c_double,
        },
    };
    let mut sp: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut slen: libc::c_double = 0.;
    let mut slen2: libc::c_double = 0.;
    slen = arrow_length(e, sflag);
    slen2 = slen * slen;
    (*spl).sflag = sflag;
    (*spl).sp = *ps.offset(startp as isize);
    if endp > startp
        && ((*ps.offset(startp as isize)).x - (*ps.offset((startp + 3 as libc::c_int) as isize)).x)
            * ((*ps.offset(startp as isize)).x
                - (*ps.offset((startp + 3 as libc::c_int) as isize)).x)
            + ((*ps.offset(startp as isize)).y
                - (*ps.offset((startp + 3 as libc::c_int) as isize)).y)
                * ((*ps.offset(startp as isize)).y
                    - (*ps.offset((startp + 3 as libc::c_int) as isize)).y)
            < slen2
    {
        startp += 3 as libc::c_int;
    }
    sp[0 as libc::c_int as usize] = *ps.offset((startp + 3 as libc::c_int) as isize);
    sp[1 as libc::c_int as usize] = *ps.offset((startp + 2 as libc::c_int) as isize);
    sp[2 as libc::c_int as usize] = *ps.offset((startp + 1 as libc::c_int) as isize);
    sp[3 as libc::c_int as usize] = (*spl).sp;
    inside_context.a.p = &mut *sp.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut pointf;
    inside_context.a.r = &mut slen2;
    bezier_clip(
        &mut inside_context,
        Some(inside as unsafe extern "C" fn(*mut inside_t, pointf) -> bool),
        sp.as_mut_ptr(),
        0 as libc::c_int != 0,
    );
    *ps.offset(startp as isize) = sp[3 as libc::c_int as usize];
    *ps.offset((startp + 1 as libc::c_int) as isize) = sp[2 as libc::c_int as usize];
    *ps.offset((startp + 2 as libc::c_int) as isize) = sp[1 as libc::c_int as usize];
    *ps.offset((startp + 3 as libc::c_int) as isize) = sp[0 as libc::c_int as usize];
    return startp;
}
#[no_mangle]
pub unsafe extern "C" fn arrowOrthoClip(
    mut e: *mut edge_t,
    mut ps: *mut pointf,
    mut startp: libc::c_int,
    mut endp: libc::c_int,
    mut spl: *mut bezier,
    mut sflag: libc::c_int,
    mut eflag: libc::c_int,
) {
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut r: pointf = pointf { x: 0., y: 0. };
    let mut s: pointf = pointf { x: 0., y: 0. };
    let mut t: pointf = pointf { x: 0., y: 0. };
    let mut d: libc::c_double = 0.;
    let mut tlen: libc::c_double = 0.;
    let mut hlen: libc::c_double = 0.;
    let mut maxd: libc::c_double = 0.;
    if sflag != 0 && eflag != 0 && endp == startp {
        p = *ps.offset(endp as isize);
        q = *ps.offset((endp + 3 as libc::c_int) as isize);
        tlen = arrow_length(e, sflag);
        hlen = arrow_length(e, eflag);
        d = sqrt((p.x - q.x) * (p.x - q.x) + (p.y - q.y) * (p.y - q.y));
        if hlen + tlen >= d {
            tlen = d / 3.0f64;
            hlen = tlen;
        }
        if p.y == q.y {
            t.y = p.y;
            s.y = t.y;
            if p.x < q.x {
                t.x = q.x - hlen;
                s.x = p.x + tlen;
            } else {
                t.x = q.x + hlen;
                s.x = p.x - tlen;
            }
        } else {
            t.x = p.x;
            s.x = t.x;
            if p.y < q.y {
                t.y = q.y - hlen;
                s.y = p.y + tlen;
            } else {
                t.y = q.y + hlen;
                s.y = p.y - tlen;
            }
        }
        let ref mut fresh1 = *ps.offset((endp + 1 as libc::c_int) as isize);
        *fresh1 = s;
        *ps.offset(endp as isize) = *fresh1;
        let ref mut fresh2 = *ps.offset((endp + 3 as libc::c_int) as isize);
        *fresh2 = t;
        *ps.offset((endp + 2 as libc::c_int) as isize) = *fresh2;
        (*spl).eflag = eflag;
        (*spl).ep = p;
        (*spl).sflag = sflag;
        (*spl).sp = q;
        return;
    }
    if eflag != 0 {
        hlen = arrow_length(e, eflag);
        p = *ps.offset(endp as isize);
        q = *ps.offset((endp + 3 as libc::c_int) as isize);
        d = sqrt((p.x - q.x) * (p.x - q.x) + (p.y - q.y) * (p.y - q.y));
        maxd = 0.9f64 * d;
        if hlen >= maxd {
            hlen = maxd;
        }
        if p.y == q.y {
            r.y = p.y;
            if p.x < q.x {
                r.x = q.x - hlen;
            } else {
                r.x = q.x + hlen;
            }
        } else {
            r.x = p.x;
            if p.y < q.y {
                r.y = q.y - hlen;
            } else {
                r.y = q.y + hlen;
            }
        }
        *ps.offset((endp + 1 as libc::c_int) as isize) = p;
        let ref mut fresh3 = *ps.offset((endp + 3 as libc::c_int) as isize);
        *fresh3 = r;
        *ps.offset((endp + 2 as libc::c_int) as isize) = *fresh3;
        (*spl).eflag = eflag;
        (*spl).ep = q;
    }
    if sflag != 0 {
        tlen = arrow_length(e, sflag);
        p = *ps.offset(startp as isize);
        q = *ps.offset((startp + 3 as libc::c_int) as isize);
        d = sqrt((p.x - q.x) * (p.x - q.x) + (p.y - q.y) * (p.y - q.y));
        maxd = 0.9f64 * d;
        if tlen >= maxd {
            tlen = maxd;
        }
        if p.y == q.y {
            r.y = p.y;
            if p.x < q.x {
                r.x = p.x + tlen;
            } else {
                r.x = p.x - tlen;
            }
        } else {
            r.x = p.x;
            if p.y < q.y {
                r.y = p.y + tlen;
            } else {
                r.y = p.y - tlen;
            }
        }
        let ref mut fresh4 = *ps.offset((startp + 1 as libc::c_int) as isize);
        *fresh4 = r;
        *ps.offset(startp as isize) = *fresh4;
        *ps.offset((startp + 2 as libc::c_int) as isize) = q;
        (*spl).sflag = sflag;
        (*spl).sp = p;
    }
}
unsafe extern "C" fn arrow_type_normal(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut v: pointf = pointf { x: 0., y: 0. };
    let mut a: [pointf; 5] = [pointf { x: 0., y: 0. }; 5];
    let mut arrowwidth: libc::c_double = 0.;
    arrowwidth = 0.35f64;
    if penwidth > 4 as libc::c_int as libc::c_double {
        arrowwidth *= penwidth / 4 as libc::c_int as libc::c_double;
    }
    v.x = -u.y * arrowwidth;
    v.y = u.x * arrowwidth;
    q.x = p.x + u.x;
    q.y = p.y + u.y;
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int != 0 {
        a[4 as libc::c_int as usize] = p;
        a[0 as libc::c_int as usize] = a[4 as libc::c_int as usize];
        a[1 as libc::c_int as usize].x = p.x - v.x;
        a[1 as libc::c_int as usize].y = p.y - v.y;
        a[2 as libc::c_int as usize] = q;
        a[3 as libc::c_int as usize].x = p.x + v.x;
        a[3 as libc::c_int as usize].y = p.y + v.y;
    } else {
        a[4 as libc::c_int as usize] = q;
        a[0 as libc::c_int as usize] = a[4 as libc::c_int as usize];
        a[1 as libc::c_int as usize].x = q.x - v.x;
        a[1 as libc::c_int as usize].y = q.y - v.y;
        a[2 as libc::c_int as usize] = p;
        a[3 as libc::c_int as usize].x = q.x + v.x;
        a[3 as libc::c_int as usize].y = q.y + v.y;
    }
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int != 0 {
        gvrender_polygon(
            job,
            a.as_mut_ptr(),
            3 as libc::c_int,
            (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
        );
    } else if flag & (1 as libc::c_int) << 4 as libc::c_int + 3 as libc::c_int != 0 {
        gvrender_polygon(
            job,
            &mut *a.as_mut_ptr().offset(2 as libc::c_int as isize),
            3 as libc::c_int,
            (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
        );
    } else {
        gvrender_polygon(
            job,
            &mut *a.as_mut_ptr().offset(1 as libc::c_int as isize),
            3 as libc::c_int,
            (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
        );
    };
}
unsafe extern "C" fn arrow_type_crow(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut m: pointf = pointf { x: 0., y: 0. };
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut v: pointf = pointf { x: 0., y: 0. };
    let mut w: pointf = pointf { x: 0., y: 0. };
    let mut a: [pointf; 9] = [pointf { x: 0., y: 0. }; 9];
    let mut arrowwidth: libc::c_double = 0.;
    let mut shaftwidth: libc::c_double = 0.;
    arrowwidth = 0.45f64;
    if penwidth > 4 as libc::c_int as libc::c_double * arrowsize
        && flag & (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int != 0
    {
        arrowwidth *= penwidth / (4 as libc::c_int as libc::c_double * arrowsize);
    }
    shaftwidth = 0 as libc::c_int as libc::c_double;
    if penwidth > 1 as libc::c_int as libc::c_double
        && flag & (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int != 0
    {
        shaftwidth = 0.05f64 * (penwidth - 1 as libc::c_int as libc::c_double) / arrowsize;
    }
    v.x = -u.y * arrowwidth;
    v.y = u.x * arrowwidth;
    w.x = -u.y * shaftwidth;
    w.y = u.x * shaftwidth;
    q.x = p.x + u.x;
    q.y = p.y + u.y;
    m.x = p.x + u.x * 0.5f64;
    m.y = p.y + u.y * 0.5f64;
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int != 0 {
        a[8 as libc::c_int as usize] = p;
        a[0 as libc::c_int as usize] = a[8 as libc::c_int as usize];
        a[1 as libc::c_int as usize].x = q.x - v.x;
        a[1 as libc::c_int as usize].y = q.y - v.y;
        a[2 as libc::c_int as usize].x = m.x - w.x;
        a[2 as libc::c_int as usize].y = m.y - w.y;
        a[3 as libc::c_int as usize].x = q.x - w.x;
        a[3 as libc::c_int as usize].y = q.y - w.y;
        a[4 as libc::c_int as usize] = q;
        a[5 as libc::c_int as usize].x = q.x + w.x;
        a[5 as libc::c_int as usize].y = q.y + w.y;
        a[6 as libc::c_int as usize].x = m.x + w.x;
        a[6 as libc::c_int as usize].y = m.y + w.y;
        a[7 as libc::c_int as usize].x = q.x + v.x;
        a[7 as libc::c_int as usize].y = q.y + v.y;
    } else {
        a[8 as libc::c_int as usize] = q;
        a[0 as libc::c_int as usize] = a[8 as libc::c_int as usize];
        a[1 as libc::c_int as usize].x = p.x - v.x;
        a[1 as libc::c_int as usize].y = p.y - v.y;
        a[2 as libc::c_int as usize].x = m.x - w.x;
        a[2 as libc::c_int as usize].y = m.y - w.y;
        a[3 as libc::c_int as usize].x = p.x;
        a[3 as libc::c_int as usize].y = p.y;
        a[4 as libc::c_int as usize] = p;
        a[5 as libc::c_int as usize].x = p.x;
        a[5 as libc::c_int as usize].y = p.y;
        a[6 as libc::c_int as usize].x = m.x + w.x;
        a[6 as libc::c_int as usize].y = m.y + w.y;
        a[7 as libc::c_int as usize].x = p.x + v.x;
        a[7 as libc::c_int as usize].y = p.y + v.y;
    }
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int != 0 {
        gvrender_polygon(job, a.as_mut_ptr(), 6 as libc::c_int, 1 as libc::c_int);
    } else if flag & (1 as libc::c_int) << 4 as libc::c_int + 3 as libc::c_int != 0 {
        gvrender_polygon(
            job,
            &mut *a.as_mut_ptr().offset(3 as libc::c_int as isize),
            6 as libc::c_int,
            1 as libc::c_int,
        );
    } else {
        gvrender_polygon(job, a.as_mut_ptr(), 9 as libc::c_int, 1 as libc::c_int);
    };
}
unsafe extern "C" fn arrow_type_gap(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut a: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    q.x = p.x + u.x;
    q.y = p.y + u.y;
    a[0 as libc::c_int as usize] = p;
    a[1 as libc::c_int as usize] = q;
    gvrender_polyline(job, a.as_mut_ptr(), 2 as libc::c_int);
}
unsafe extern "C" fn arrow_type_tee(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut m: pointf = pointf { x: 0., y: 0. };
    let mut n: pointf = pointf { x: 0., y: 0. };
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut v: pointf = pointf { x: 0., y: 0. };
    let mut a: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    v.x = -u.y;
    v.y = u.x;
    q.x = p.x + u.x;
    q.y = p.y + u.y;
    m.x = p.x + u.x * 0.2f64;
    m.y = p.y + u.y * 0.2f64;
    n.x = p.x + u.x * 0.6f64;
    n.y = p.y + u.y * 0.6f64;
    a[0 as libc::c_int as usize].x = m.x + v.x;
    a[0 as libc::c_int as usize].y = m.y + v.y;
    a[1 as libc::c_int as usize].x = m.x - v.x;
    a[1 as libc::c_int as usize].y = m.y - v.y;
    a[2 as libc::c_int as usize].x = n.x - v.x;
    a[2 as libc::c_int as usize].y = n.y - v.y;
    a[3 as libc::c_int as usize].x = n.x + v.x;
    a[3 as libc::c_int as usize].y = n.y + v.y;
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int != 0 {
        a[0 as libc::c_int as usize] = m;
        a[3 as libc::c_int as usize] = n;
    } else if flag & (1 as libc::c_int) << 4 as libc::c_int + 3 as libc::c_int != 0 {
        a[1 as libc::c_int as usize] = m;
        a[2 as libc::c_int as usize] = n;
    }
    gvrender_polygon(job, a.as_mut_ptr(), 4 as libc::c_int, 1 as libc::c_int);
    a[0 as libc::c_int as usize] = p;
    a[1 as libc::c_int as usize] = q;
    gvrender_polyline(job, a.as_mut_ptr(), 2 as libc::c_int);
}
unsafe extern "C" fn arrow_type_box(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut m: pointf = pointf { x: 0., y: 0. };
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut v: pointf = pointf { x: 0., y: 0. };
    let mut a: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    v.x = -u.y * 0.4f64;
    v.y = u.x * 0.4f64;
    m.x = p.x + u.x * 0.8f64;
    m.y = p.y + u.y * 0.8f64;
    q.x = p.x + u.x;
    q.y = p.y + u.y;
    a[0 as libc::c_int as usize].x = p.x + v.x;
    a[0 as libc::c_int as usize].y = p.y + v.y;
    a[1 as libc::c_int as usize].x = p.x - v.x;
    a[1 as libc::c_int as usize].y = p.y - v.y;
    a[2 as libc::c_int as usize].x = m.x - v.x;
    a[2 as libc::c_int as usize].y = m.y - v.y;
    a[3 as libc::c_int as usize].x = m.x + v.x;
    a[3 as libc::c_int as usize].y = m.y + v.y;
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int != 0 {
        a[0 as libc::c_int as usize] = p;
        a[3 as libc::c_int as usize] = m;
    } else if flag & (1 as libc::c_int) << 4 as libc::c_int + 3 as libc::c_int != 0 {
        a[1 as libc::c_int as usize] = p;
        a[2 as libc::c_int as usize] = m;
    }
    gvrender_polygon(
        job,
        a.as_mut_ptr(),
        4 as libc::c_int,
        (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
    );
    a[0 as libc::c_int as usize] = m;
    a[1 as libc::c_int as usize] = q;
    gvrender_polyline(job, a.as_mut_ptr(), 2 as libc::c_int);
}
unsafe extern "C" fn arrow_type_diamond(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut r: pointf = pointf { x: 0., y: 0. };
    let mut v: pointf = pointf { x: 0., y: 0. };
    let mut a: [pointf; 5] = [pointf { x: 0., y: 0. }; 5];
    v.x = -u.y / 3.0f64;
    v.y = u.x / 3.0f64;
    r.x = p.x + u.x / 2.0f64;
    r.y = p.y + u.y / 2.0f64;
    q.x = p.x + u.x;
    q.y = p.y + u.y;
    a[4 as libc::c_int as usize] = q;
    a[0 as libc::c_int as usize] = a[4 as libc::c_int as usize];
    a[1 as libc::c_int as usize].x = r.x + v.x;
    a[1 as libc::c_int as usize].y = r.y + v.y;
    a[2 as libc::c_int as usize] = p;
    a[3 as libc::c_int as usize].x = r.x - v.x;
    a[3 as libc::c_int as usize].y = r.y - v.y;
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int != 0 {
        gvrender_polygon(
            job,
            &mut *a.as_mut_ptr().offset(2 as libc::c_int as isize),
            3 as libc::c_int,
            (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
        );
    } else if flag & (1 as libc::c_int) << 4 as libc::c_int + 3 as libc::c_int != 0 {
        gvrender_polygon(
            job,
            a.as_mut_ptr(),
            3 as libc::c_int,
            (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
        );
    } else {
        gvrender_polygon(
            job,
            a.as_mut_ptr(),
            4 as libc::c_int,
            (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
        );
    };
}
unsafe extern "C" fn arrow_type_dot(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut r: libc::c_double = 0.;
    let mut AF: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    r = hypot(u.x, u.y) / 2.0f64;
    AF[0 as libc::c_int as usize].x = p.x + u.x / 2.0f64 - r;
    AF[0 as libc::c_int as usize].y = p.y + u.y / 2.0f64 - r;
    AF[1 as libc::c_int as usize].x = p.x + u.x / 2.0f64 + r;
    AF[1 as libc::c_int as usize].y = p.y + u.y / 2.0f64 + r;
    gvrender_ellipse(
        job,
        AF.as_mut_ptr(),
        2 as libc::c_int,
        (flag & (1 as libc::c_int) << 4 as libc::c_int + 0 as libc::c_int == 0) as libc::c_int,
    );
}
unsafe extern "C" fn arrow_type_curve(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut arrowwidth: libc::c_double = if penwidth > 4 as libc::c_int as libc::c_double {
        0.5f64 * penwidth / 4 as libc::c_int as libc::c_double
    } else {
        0.5f64
    };
    let mut q: pointf = pointf { x: 0., y: 0. };
    let mut v: pointf = pointf { x: 0., y: 0. };
    let mut w: pointf = pointf { x: 0., y: 0. };
    let mut AF: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut a: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    q.x = p.x + u.x;
    q.y = p.y + u.y;
    v.x = -u.y * arrowwidth;
    v.y = u.x * arrowwidth;
    w.x = v.y;
    w.y = -v.x;
    a[0 as libc::c_int as usize] = p;
    a[1 as libc::c_int as usize] = q;
    AF[0 as libc::c_int as usize].x = p.x + v.x + w.x;
    AF[0 as libc::c_int as usize].y = p.y + v.y + w.y;
    AF[3 as libc::c_int as usize].x = p.x - v.x + w.x;
    AF[3 as libc::c_int as usize].y = p.y - v.y + w.y;
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 1 as libc::c_int != 0 {
        AF[1 as libc::c_int as usize].x = p.x + 0.95f64 * v.x + w.x + w.x * 4.0f64 / 3.0f64;
        AF[1 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y + w.y * 4.0f64 / 3.0f64;
        AF[2 as libc::c_int as usize].x = p.x - 0.95f64 * v.x + w.x + w.x * 4.0f64 / 3.0f64;
        AF[2 as libc::c_int as usize].y = AF[3 as libc::c_int as usize].y + w.y * 4.0f64 / 3.0f64;
    } else {
        AF[1 as libc::c_int as usize].x = p.x + 0.95f64 * v.x + w.x - w.x * 4.0f64 / 3.0f64;
        AF[1 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y - w.y * 4.0f64 / 3.0f64;
        AF[2 as libc::c_int as usize].x = p.x - 0.95f64 * v.x + w.x - w.x * 4.0f64 / 3.0f64;
        AF[2 as libc::c_int as usize].y = AF[3 as libc::c_int as usize].y - w.y * 4.0f64 / 3.0f64;
    }
    gvrender_polyline(job, a.as_mut_ptr(), 2 as libc::c_int);
    if flag & (1 as libc::c_int) << 4 as libc::c_int + 2 as libc::c_int != 0 {
        Bezier(
            AF.as_mut_ptr(),
            3 as libc::c_int,
            0.5f64,
            0 as *mut pointf,
            AF.as_mut_ptr(),
        );
    } else if flag & (1 as libc::c_int) << 4 as libc::c_int + 3 as libc::c_int != 0 {
        Bezier(
            AF.as_mut_ptr(),
            3 as libc::c_int,
            0.5f64,
            AF.as_mut_ptr(),
            0 as *mut pointf,
        );
    }
    gvrender_beziercurve(
        job,
        AF.as_mut_ptr(),
        (::std::mem::size_of::<[pointf; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<pointf>() as libc::c_ulong) as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn arrow_gen_type(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) -> pointf {
    let mut f: libc::c_int = 0;
    f = flag & ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < Arrowtypes_size {
        let mut arrowtype: *const arrowtype_t =
            &*Arrowtypes.as_ptr().offset(i as isize) as *const arrowtype_t;
        if f == (*arrowtype).type_0 {
            u.x *= (*arrowtype).lenfact * arrowsize;
            u.y *= (*arrowtype).lenfact * arrowsize;
            ((*arrowtype).gen).expect("non-null function pointer")(
                job, p, u, arrowsize, penwidth, flag,
            );
            p.x = p.x + u.x;
            p.y = p.y + u.y;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn arrow_bb(
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
) -> boxf {
    let mut s: libc::c_double = 0.;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut ax: libc::c_double = 0.;
    let mut ay: libc::c_double = 0.;
    let mut bx: libc::c_double = 0.;
    let mut by: libc::c_double = 0.;
    let mut cx: libc::c_double = 0.;
    let mut cy: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut ux2: libc::c_double = 0.;
    let mut uy2: libc::c_double = 0.;
    u.x -= p.x;
    u.y -= p.y;
    s = 10.0f64 * arrowsize / (hypot(u.x, u.y) + 0.0001f64);
    u.x += if u.x >= 0.0f64 { 0.0001f64 } else { -0.0001f64 };
    u.y += if u.y >= 0.0f64 { 0.0001f64 } else { -0.0001f64 };
    u.x *= s;
    u.y *= s;
    ux2 = u.x / 2.0f64;
    uy2 = u.y / 2.0f64;
    ax = p.x - uy2;
    ay = p.y - ux2;
    bx = p.x + uy2;
    by = p.y + ux2;
    cx = ax + u.x;
    cy = ay + u.y;
    dx = bx + u.x;
    dy = by + u.y;
    bb.UR.x = if ax
        > (if bx > (if cx > dx { cx } else { dx }) {
            bx
        } else {
            (if cx > dx { cx } else { dx })
        }) {
        ax
    } else if bx > (if cx > dx { cx } else { dx }) {
        bx
    } else if cx > dx {
        cx
    } else {
        dx
    };
    bb.UR.y = if ay
        > (if by > (if cy > dy { cy } else { dy }) {
            by
        } else {
            (if cy > dy { cy } else { dy })
        }) {
        ay
    } else if by > (if cy > dy { cy } else { dy }) {
        by
    } else if cy > dy {
        cy
    } else {
        dy
    };
    bb.LL.x = if ax
        < (if bx < (if cx < dx { cx } else { dx }) {
            bx
        } else {
            (if cx < dx { cx } else { dx })
        }) {
        ax
    } else if bx < (if cx < dx { cx } else { dx }) {
        bx
    } else if cx < dx {
        cx
    } else {
        dx
    };
    bb.LL.y = if ay
        < (if by < (if cy < dy { cy } else { dy }) {
            by
        } else {
            (if cy < dy { cy } else { dy })
        }) {
        ay
    } else if by < (if cy < dy { cy } else { dy }) {
        by
    } else if cy < dy {
        cy
    } else {
        dy
    };
    return bb;
}
#[no_mangle]
pub unsafe extern "C" fn arrow_gen(
    mut job: *mut GVJ_t,
    mut emit_state: emit_state_t,
    mut p: pointf,
    mut u: pointf,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
    mut flag: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut s: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut old_emit_state: emit_state_t = EMIT_GDRAW;
    old_emit_state = (*obj).emit_state;
    (*obj).emit_state = emit_state;
    gvrender_set_style(job, (*(*job).gvc).defaultlinestyle);
    gvrender_set_penwidth(job, penwidth);
    u.x -= p.x;
    u.y -= p.y;
    s = 10.0f64 / (hypot(u.x, u.y) + 0.0001f64);
    u.x += if u.x >= 0.0f64 { 0.0001f64 } else { -0.0001f64 };
    u.y += if u.y >= 0.0f64 { 0.0001f64 } else { -0.0001f64 };
    u.x *= s;
    u.y *= s;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        f = flag >> i * 8 as libc::c_int
            & ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
        if f == 0 as libc::c_int {
            break;
        }
        p = arrow_gen_type(job, p, u, arrowsize, penwidth, f);
        i += 1;
    }
    (*obj).emit_state = old_emit_state;
}
unsafe extern "C" fn run_static_initializers() {
    Arrowtypes_size = (::std::mem::size_of::<[arrowtype_t; 8]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<arrowtype_t>() as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
