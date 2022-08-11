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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Dtqueue: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn free_html_label(_: *mut htmllabel_t, _: libc::c_int);
    fn free_html_data(_: *mut htmldata_t);
    fn free_html_text(_: *mut htmltxt_t);
    fn initHTMLlexer(
        _: *mut libc::c_char,
        _: *mut agxbuf,
        _: *mut htmlenv_t,
    ) -> libc::c_int;
    fn htmllex() -> libc::c_int;
    fn clearHTMLlexer() -> libc::c_int;
    fn htmlerror(_: *const libc::c_char);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmllabel_t {
    pub u: C2RustUnnamed_4,
    pub kind: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub tbl: *mut htmltbl_t,
    pub txt: *mut htmltxt_t,
    pub img: *mut htmlimg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlimg_t {
    pub box_0: boxf,
    pub src: *mut libc::c_char,
    pub scale: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmltxt_t {
    pub spans: *mut htextspan_t,
    pub nspans: libc::c_short,
    pub simple: libc::c_char,
    pub box_0: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htextspan_t {
    pub items: *mut textspan_t,
    pub nitems: libc::c_short,
    pub just: libc::c_char,
    pub size: libc::c_double,
    pub lfsize: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmltbl_t {
    pub data: htmldata_t,
    pub u: C2RustUnnamed_5,
    pub cb: libc::c_schar,
    pub heights: *mut libc::c_int,
    pub widths: *mut libc::c_int,
    pub rc: libc::c_int,
    pub cc: libc::c_int,
    pub font: *mut textfont_t,
    pub flags: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub n: C2RustUnnamed_7,
    pub p: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub prev: *mut htmltbl_t,
    pub rows: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub parent: *mut htmlcell_t,
    pub cells: *mut *mut htmlcell_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlcell_t {
    pub data: htmldata_t,
    pub cspan: libc::c_ushort,
    pub rspan: libc::c_ushort,
    pub col: libc::c_ushort,
    pub row: libc::c_ushort,
    pub child: htmllabel_t,
    pub parent: *mut htmltbl_t,
    pub ruled: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmldata_t {
    pub href: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub bgcolor: *mut libc::c_char,
    pub pencolor: *mut libc::c_char,
    pub gradientangle: libc::c_int,
    pub space: libc::c_schar,
    pub border: libc::c_uchar,
    pub pad: libc::c_uchar,
    pub sides: libc::c_uchar,
    pub flags: libc::c_ushort,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub style: libc::c_ushort,
    pub box_0: boxf,
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
    pub u: C2RustUnnamed_8,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub txt: C2RustUnnamed_9,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub span: *mut textspan_t,
    pub nspans: libc::c_short,
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
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pitem {
    pub link: Dtlink_t,
    pub u: C2RustUnnamed_10,
    pub ruled: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub rp: *mut Dt_t,
    pub cp: *mut htmlcell_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlenv_t {
    pub pos: pointf,
    pub finfo: textfont_t,
    pub obj: *mut libc::c_void,
    pub g: *mut graph_t,
    pub imgscale: *mut libc::c_char,
    pub objid: *mut libc::c_char,
    pub objid_set: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfont_t {
    pub cfont: *mut textfont_t,
    pub pfont: *mut sfont_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub lbl: *mut htmllabel_t,
    pub tblstack: *mut htmltbl_t,
    pub fitemList: *mut Dt_t,
    pub fspanList: *mut Dt_t,
    pub str_0: *mut agxbuf,
    pub fontstack: *mut sfont_t,
    pub gvc: *mut GVC_t,
}
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub union HTMLSTYPE {
    pub i: libc::c_int,
    pub txt: *mut htmltxt_t,
    pub cell: *mut htmlcell_t,
    pub tbl: *mut htmltbl_t,
    pub font: *mut textfont_t,
    pub img: *mut htmlimg_t,
    pub p: *mut pitem,
}
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fspan {
    pub link: Dtlink_t,
    pub lp: htextspan_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fitem {
    pub link: Dtlink_t,
    pub ti: textspan_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: HTMLSTYPE,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_realloc(
    mut ptr: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, new_size);
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    if new_size > old_size {
        memset(
            (p as *mut libc::c_char).offset(old_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            new_size.wrapping_sub(old_size),
        );
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_recalloc(
    mut ptr: *mut libc::c_void,
    mut old_nmemb: size_t,
    mut new_nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size > 0 as libc::c_int as libc::c_ulong
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size))
        as libc::c_int as libc::c_long != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(ptr, old_nmemb.wrapping_mul(size), new_nmemb.wrapping_mul(size));
}
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh0 = (*xb).buf;
        *fresh0 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh1 = (*xb).buf;
        *fresh1 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh2 = (*xb).eptr;
    *fresh2 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh3 = (*xb).ptr;
    *fresh3 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn agxbfree(mut xb: *mut agxbuf) {
    if (*xb).dyna != 0 {
        free((*xb).buf as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn agxbmore(mut xb: *mut agxbuf, mut ssz: size_t) {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut nsize: size_t = 0 as libc::c_int as size_t;
    let mut nbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    size = ((*xb).eptr).offset_from((*xb).buf) as libc::c_long as size_t;
    nsize = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    if size.wrapping_add(ssz) > nsize {
        nsize = size.wrapping_add(ssz);
    }
    cnt = ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
    if (*xb).dyna != 0 {
        nbuf = gv_recalloc(
            (*xb).buf as *mut libc::c_void,
            size,
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        nbuf = gv_calloc(nsize, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as *mut libc::c_char;
        memcpy(nbuf as *mut libc::c_void, (*xb).buf as *const libc::c_void, cnt);
        (*xb).dyna = 1 as libc::c_int;
    }
    let ref mut fresh4 = (*xb).buf;
    *fresh4 = nbuf;
    let ref mut fresh5 = (*xb).ptr;
    *fresh5 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh6 = (*xb).eptr;
    *fresh6 = ((*xb).buf).offset(nsize as isize);
}
#[inline]
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh7 = (*xb).ptr;
    let fresh8 = *fresh7;
    *fresh7 = (*fresh7).offset(1);
    *fresh8 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh9 = (*xb).ptr;
    *fresh9 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn agxbdisown(mut xb: *mut agxbuf) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    agxbputc(xb, '\0' as i32 as libc::c_char);
    if (*xb).dyna == 0 {
        buf = strdup((*xb).buf);
        if buf.is_null() {
            return 0 as *mut libc::c_char;
        }
    } else {
        buf = (*xb).buf;
    }
    let ref mut fresh10 = (*xb).eptr;
    *fresh10 = 0 as *mut libc::c_char;
    let ref mut fresh11 = (*xb).ptr;
    *fresh11 = *fresh10;
    let ref mut fresh12 = (*xb).buf;
    *fresh12 = *fresh11;
    (*xb).dyna = 1 as libc::c_int;
    return buf;
}
static mut HTMLstate: C2RustUnnamed_11 = C2RustUnnamed_11 {
    lbl: 0 as *const htmllabel_t as *mut htmllabel_t,
    tblstack: 0 as *const htmltbl_t as *mut htmltbl_t,
    fitemList: 0 as *const Dt_t as *mut Dt_t,
    fspanList: 0 as *const Dt_t as *mut Dt_t,
    str_0: 0 as *const agxbuf as *mut agxbuf,
    fontstack: 0 as *const sfont_t as *mut sfont_t,
    gvc: 0 as *const GVC_t as *mut GVC_t,
};
unsafe extern "C" fn free_ritem(
    mut d: *mut Dt_t,
    mut p: *mut pitem,
    mut ds: *mut Dtdisc_t,
) {
    dtclose((*p).u.rp);
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn free_item(
    mut d: *mut Dt_t,
    mut p: *mut libc::c_void,
    mut ds: *mut Dtdisc_t,
) {
    free(p);
}
unsafe extern "C" fn cleanTbl(mut tp: *mut htmltbl_t) {
    dtclose((*tp).u.p.rows);
    free_html_data(&mut (*tp).data);
    free(tp as *mut libc::c_void);
}
unsafe extern "C" fn cleanCell(mut cp: *mut htmlcell_t) {
    if (*cp).child.kind as libc::c_int == 1 as libc::c_int {
        cleanTbl((*cp).child.u.tbl);
    } else if (*cp).child.kind as libc::c_int == 2 as libc::c_int {
        free_html_text((*cp).child.u.txt);
    }
    free_html_data(&mut (*cp).data);
    free(cp as *mut libc::c_void);
}
unsafe extern "C" fn free_citem(
    mut d: *mut Dt_t,
    mut p: *mut pitem,
    mut ds: *mut Dtdisc_t,
) {
    cleanCell((*p).u.cp);
    free(p as *mut libc::c_void);
}
static mut rowDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut pitem, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_ritem
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut pitem,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
static mut cellDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_item
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut libc::c_void,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn free_fitem(
    mut d: *mut Dt_t,
    mut p: *mut fitem,
    mut ds: *mut Dtdisc_t,
) {
    free((*p).ti.str_0 as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn free_fspan(
    mut d: *mut Dt_t,
    mut p: *mut fspan,
    mut ds: *mut Dtdisc_t,
) {
    let mut ti: *mut textspan_t = 0 as *mut textspan_t;
    if (*p).lp.nitems != 0 {
        let mut i: libc::c_int = 0;
        ti = (*p).lp.items;
        i = 0 as libc::c_int;
        while i < (*p).lp.nitems as libc::c_int {
            free((*ti).str_0 as *mut libc::c_void);
            ti = ti.offset(1);
            i += 1;
        }
        free((*p).lp.items as *mut libc::c_void);
    }
    free(p as *mut libc::c_void);
}
static mut fstrDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_item
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut libc::c_void,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
static mut fspanDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_item
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut libc::c_void,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn appendFItemList(mut ag: *mut agxbuf) {
    let mut fi: *mut fitem = zmalloc(::std::mem::size_of::<fitem>() as libc::c_ulong)
        as *mut fitem;
    let ref mut fresh13 = (*fi).ti.str_0;
    *fresh13 = agxbdisown(ag);
    let ref mut fresh14 = (*fi).ti.font;
    *fresh14 = (*HTMLstate.fontstack).cfont;
    (Some(((*HTMLstate.fitemList).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(HTMLstate.fitemList, fi as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn appendFLineList(mut v: libc::c_int) {
    let mut cnt: libc::c_int = 0;
    let mut ln: *mut fspan = zmalloc(::std::mem::size_of::<fspan>() as libc::c_ulong)
        as *mut fspan;
    let mut fi: *mut fitem = 0 as *mut fitem;
    let mut ilist: *mut Dt_t = HTMLstate.fitemList;
    cnt = dtsize(ilist);
    (*ln).lp.just = v as libc::c_char;
    if cnt != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        (*ln).lp.nitems = cnt as libc::c_short;
        let ref mut fresh15 = (*ln).lp.items;
        *fresh15 = gcalloc(
            cnt as size_t,
            ::std::mem::size_of::<textspan_t>() as libc::c_ulong,
        ) as *mut textspan_t;
        fi = dtflatten(ilist) as *mut fitem;
        while !fi.is_null() {
            *((*ln).lp.items).offset(i as isize) = (*fi).ti;
            i += 1;
            fi = (*(fi as *mut Dtlink_t)).right as *mut fitem;
        }
    } else {
        let ref mut fresh16 = (*ln).lp.items;
        *fresh16 = zmalloc(::std::mem::size_of::<textspan_t>() as libc::c_ulong)
            as *mut textspan_t;
        (*ln).lp.nitems = 1 as libc::c_int as libc::c_short;
        let ref mut fresh17 = (*((*ln).lp.items).offset(0 as libc::c_int as isize))
            .str_0;
        *fresh17 = strdup(b"\0" as *const u8 as *const libc::c_char);
        let ref mut fresh18 = (*((*ln).lp.items).offset(0 as libc::c_int as isize)).font;
        *fresh18 = (*HTMLstate.fontstack).cfont;
    }
    (Some(((*ilist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ilist, 0 as *mut libc::c_void, 0o100 as libc::c_int);
    (Some(((*HTMLstate.fspanList).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(HTMLstate.fspanList, ln as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn mkText() -> *mut htmltxt_t {
    let mut cnt: libc::c_int = 0;
    let mut ispan: *mut Dt_t = HTMLstate.fspanList;
    let mut fl: *mut fspan = 0 as *mut fspan;
    let mut hft: *mut htmltxt_t = zmalloc(
        ::std::mem::size_of::<htmltxt_t>() as libc::c_ulong,
    ) as *mut htmltxt_t;
    if dtsize(HTMLstate.fitemList) != 0 {
        appendFLineList(0 as libc::c_int);
    }
    cnt = dtsize(ispan);
    (*hft).nspans = cnt as libc::c_short;
    if cnt != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let ref mut fresh19 = (*hft).spans;
        *fresh19 = gcalloc(
            cnt as size_t,
            ::std::mem::size_of::<htextspan_t>() as libc::c_ulong,
        ) as *mut htextspan_t;
        fl = (Some(((*ispan).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(ispan, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut fspan;
        while !fl.is_null() {
            *((*hft).spans).offset(i as isize) = (*fl).lp;
            i += 1;
            fl = (Some(((*ispan).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(ispan, fl as *mut libc::c_void, 0o10 as libc::c_int) as *mut fspan;
        }
    }
    (Some(((*ispan).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ispan, 0 as *mut libc::c_void, 0o100 as libc::c_int);
    return hft;
}
unsafe extern "C" fn lastRow() -> *mut pitem {
    let mut tbl: *mut htmltbl_t = HTMLstate.tblstack;
    let mut sp: *mut pitem = (Some(
        ((*(*tbl).u.p.rows).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )((*tbl).u.p.rows, 0 as *mut libc::c_void, 0o400 as libc::c_int) as *mut pitem;
    return sp;
}
unsafe extern "C" fn addRow() -> *mut pitem {
    let mut dp: *mut Dt_t = dtopen(&mut cellDisc, Dtqueue);
    let mut tbl: *mut htmltbl_t = HTMLstate.tblstack;
    let mut sp: *mut pitem = zmalloc(::std::mem::size_of::<pitem>() as libc::c_ulong)
        as *mut pitem;
    let ref mut fresh20 = (*sp).u.rp;
    *fresh20 = dp;
    if (*tbl).flags as libc::c_int & 2 as libc::c_int != 0 {
        (*sp).ruled = 1 as libc::c_int as libc::c_uchar;
    }
    (Some(((*(*tbl).u.p.rows).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*tbl).u.p.rows, sp as *mut libc::c_void, 0o1 as libc::c_int);
    return sp;
}
unsafe extern "C" fn setCell(
    mut cp: *mut htmlcell_t,
    mut obj: *mut libc::c_void,
    mut kind: libc::c_char,
) {
    let mut sp: *mut pitem = zmalloc(::std::mem::size_of::<pitem>() as libc::c_ulong)
        as *mut pitem;
    let mut tbl: *mut htmltbl_t = HTMLstate.tblstack;
    let mut rp: *mut pitem = (Some(
        ((*(*tbl).u.p.rows).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )((*tbl).u.p.rows, 0 as *mut libc::c_void, 0o400 as libc::c_int) as *mut pitem;
    let mut row: *mut Dt_t = (*rp).u.rp;
    let ref mut fresh21 = (*sp).u.cp;
    *fresh21 = cp;
    (Some(((*row).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(row, sp as *mut libc::c_void, 0o1 as libc::c_int);
    (*cp).child.kind = kind;
    if (*tbl).flags as libc::c_int & 1 as libc::c_int != 0 {
        (*cp).ruled = 1 as libc::c_int as libc::c_uchar;
    }
    if kind as libc::c_int == 2 as libc::c_int {
        let ref mut fresh22 = (*cp).child.u.txt;
        *fresh22 = obj as *mut htmltxt_t;
    } else if kind as libc::c_int == 3 as libc::c_int {
        let ref mut fresh23 = (*cp).child.u.img;
        *fresh23 = obj as *mut htmlimg_t;
    } else {
        let ref mut fresh24 = (*cp).child.u.tbl;
        *fresh24 = obj as *mut htmltbl_t;
    };
}
unsafe extern "C" fn mkLabel(
    mut obj: *mut libc::c_void,
    mut kind: libc::c_char,
) -> *mut htmllabel_t {
    let mut lp: *mut htmllabel_t = zmalloc(
        ::std::mem::size_of::<htmllabel_t>() as libc::c_ulong,
    ) as *mut htmllabel_t;
    (*lp).kind = kind;
    if kind as libc::c_int == 2 as libc::c_int {
        let ref mut fresh25 = (*lp).u.txt;
        *fresh25 = obj as *mut htmltxt_t;
    } else {
        let ref mut fresh26 = (*lp).u.tbl;
        *fresh26 = obj as *mut htmltbl_t;
    }
    return lp;
}
unsafe extern "C" fn freeFontstack() {
    let mut s: *mut sfont_t = 0 as *mut sfont_t;
    let mut next: *mut sfont_t = 0 as *mut sfont_t;
    s = HTMLstate.fontstack;
    loop {
        next = (*s).pfont;
        if next.is_null() {
            break;
        }
        free(s as *mut libc::c_void);
        s = next;
    };
}
unsafe extern "C" fn cleanup() {
    let mut tp: *mut htmltbl_t = HTMLstate.tblstack;
    let mut next: *mut htmltbl_t = 0 as *mut htmltbl_t;
    if !(HTMLstate.lbl).is_null() {
        free_html_label(HTMLstate.lbl, 1 as libc::c_int);
        HTMLstate.lbl = 0 as *mut htmllabel_t;
    }
    cellDisc
        .freef = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut Dt_t, *mut pitem, *mut Dtdisc_t) -> ()>,
        Dtfree_f,
    >(
        Some(
            free_citem
                as unsafe extern "C" fn(*mut Dt_t, *mut pitem, *mut Dtdisc_t) -> (),
        ),
    );
    while !tp.is_null() {
        next = (*tp).u.p.prev;
        cleanTbl(tp);
        tp = next;
    }
    cellDisc
        .freef = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
        >,
        Dtfree_f,
    >(
        Some(
            free_item
                as unsafe extern "C" fn(
                    *mut Dt_t,
                    *mut libc::c_void,
                    *mut Dtdisc_t,
                ) -> (),
        ),
    );
    fstrDisc
        .freef = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut Dt_t, *mut fitem, *mut Dtdisc_t) -> ()>,
        Dtfree_f,
    >(
        Some(
            free_fitem
                as unsafe extern "C" fn(*mut Dt_t, *mut fitem, *mut Dtdisc_t) -> (),
        ),
    );
    (Some(((*HTMLstate.fitemList).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(HTMLstate.fitemList, 0 as *mut libc::c_void, 0o100 as libc::c_int);
    fstrDisc
        .freef = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
        >,
        Dtfree_f,
    >(
        Some(
            free_item
                as unsafe extern "C" fn(
                    *mut Dt_t,
                    *mut libc::c_void,
                    *mut Dtdisc_t,
                ) -> (),
        ),
    );
    fspanDisc
        .freef = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut Dt_t, *mut fspan, *mut Dtdisc_t) -> ()>,
        Dtfree_f,
    >(
        Some(
            free_fspan
                as unsafe extern "C" fn(*mut Dt_t, *mut fspan, *mut Dtdisc_t) -> (),
        ),
    );
    (Some(((*HTMLstate.fspanList).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(HTMLstate.fspanList, 0 as *mut libc::c_void, 0o100 as libc::c_int);
    fspanDisc
        .freef = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
        >,
        Dtfree_f,
    >(
        Some(
            free_item
                as unsafe extern "C" fn(
                    *mut Dt_t,
                    *mut libc::c_void,
                    *mut Dtdisc_t,
                ) -> (),
        ),
    );
    freeFontstack();
}
unsafe extern "C" fn nonSpace(mut s: *mut libc::c_char) -> libc::c_int {
    let mut c: libc::c_char = 0;
    loop {
        let fresh27 = s;
        s = s.offset(1);
        c = *fresh27;
        if !(c != 0) {
            break;
        }
        if c as libc::c_int != ' ' as i32 {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pushFont(mut fp: *mut textfont_t) {
    let mut ft: *mut sfont_t = zmalloc(::std::mem::size_of::<sfont_t>() as libc::c_ulong)
        as *mut sfont_t;
    let mut curfont: *mut textfont_t = (*HTMLstate.fontstack).cfont;
    let mut f: textfont_t = *fp;
    if !curfont.is_null() {
        if (f.color).is_null() && !((*curfont).color).is_null() {
            f.color = (*curfont).color;
        }
        if f.size < 0.0f64 && (*curfont).size >= 0.0f64 {
            f.size = (*curfont).size;
        }
        if (f.name).is_null() && !((*curfont).name).is_null() {
            f.name = (*curfont).name;
        }
        if (*curfont).flags() != 0 {
            f.set_flags(f.flags() | (*curfont).flags() as libc::c_int as libc::c_uint);
        }
    }
    let ref mut fresh28 = (*ft).cfont;
    *fresh28 = (Some(
        ((*(*HTMLstate.gvc).textfont_dt).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        (*HTMLstate.gvc).textfont_dt,
        &mut f as *mut textfont_t as *mut libc::c_void,
        0o1 as libc::c_int,
    ) as *mut textfont_t;
    let ref mut fresh29 = (*ft).pfont;
    *fresh29 = HTMLstate.fontstack;
    HTMLstate.fontstack = ft;
}
unsafe extern "C" fn popFont() {
    let mut curfont: *mut sfont_t = HTMLstate.fontstack;
    let mut prevfont: *mut sfont_t = (*curfont).pfont;
    free(curfont as *mut libc::c_void);
    HTMLstate.fontstack = prevfont;
}
#[no_mangle]
pub unsafe extern "C" fn parseHTML(
    mut txt: *mut libc::c_char,
    mut warn: *mut libc::c_int,
    mut env: *mut htmlenv_t,
) -> *mut htmllabel_t {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut str: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut l: *mut htmllabel_t = 0 as *mut htmllabel_t;
    let mut dfltf: sfont_t = sfont_t {
        cfont: 0 as *mut textfont_t,
        pfont: 0 as *mut sfont_t,
    };
    dfltf.cfont = 0 as *mut textfont_t;
    dfltf.pfont = 0 as *mut sfont_t;
    HTMLstate.fontstack = &mut dfltf;
    HTMLstate.tblstack = 0 as *mut htmltbl_t;
    HTMLstate.lbl = 0 as *mut htmllabel_t;
    HTMLstate.gvc = (*((*((*env).g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc;
    HTMLstate.fitemList = dtopen(&mut fstrDisc, Dtqueue);
    HTMLstate.fspanList = dtopen(&mut fspanDisc, Dtqueue);
    agxbinit(&mut str, 128 as libc::c_int as libc::c_uint, buf.as_mut_ptr());
    HTMLstate.str_0 = &mut str;
    if initHTMLlexer(txt, &mut str, env) != 0 {
        *warn = 2 as libc::c_int;
        l = 0 as *mut htmllabel_t;
    } else {
        htmlparse();
        *warn = clearHTMLlexer();
        l = HTMLstate.lbl;
    }
    dtclose(HTMLstate.fitemList);
    dtclose(HTMLstate.fspanList);
    HTMLstate.fitemList = 0 as *mut Dt_t;
    HTMLstate.fspanList = 0 as *mut Dt_t;
    HTMLstate.fontstack = 0 as *mut sfont_t;
    agxbfree(&mut str);
    return l;
}
static mut yytranslate: [yytype_uint8; 296] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
];
static mut yypact: [yytype_int16; 116] = [
    8 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    209 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    5 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(5 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    -(20 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    119 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    58 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    53 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 116] = [
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 39] = [
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(4 as libc::c_int) as yytype_int16,
    232 as libc::c_int as yytype_int16,
    -(10 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    2 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(2 as libc::c_int) as yytype_int16,
    148 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    9 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(68 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(81 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
    -(82 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int8; 39] = [
    -(1 as libc::c_int) as yytype_int8,
    3 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    110 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    105 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 272] = [
    27 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    104 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    112 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    -(46 as libc::c_int) as yytype_int8,
    -(62 as libc::c_int) as yytype_int8,
    79 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    106 as libc::c_int as yytype_int8,
    109 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    114 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    115 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 272] = [
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    105 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    83 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    5 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    9 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    12 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    16 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    12 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    18 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    19 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    12 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
];
static mut yystos: [yytype_uint8; 116] = [
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 70] = [
    0 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
];
static mut yyr2: [yytype_uint8; 70] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut HTMLSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut htmlchar: libc::c_int = 0;
#[no_mangle]
pub static mut htmllval: HTMLSTYPE = HTMLSTYPE { i: 0 };
#[no_mangle]
pub static mut htmlnerrs: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn htmlparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: libc::c_int = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyvsa: [HTMLSTYPE; 200] = [HTMLSTYPE { i: 0 }; 200];
    let mut yyvs: *mut HTMLSTYPE = 0 as *mut HTMLSTYPE;
    let mut yyvsp: *mut HTMLSTYPE = 0 as *mut HTMLSTYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: HTMLSTYPE = HTMLSTYPE { i: 0 };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    htmlnerrs = 0 as libc::c_int;
    htmlchar = -(2 as libc::c_int);
    'c_9111: loop {
        *yyssp = yystate as yytype_int16;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_ulong = (yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong;
            if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                current_block = 137226434401907431;
                break;
            }
            yystacksize = yystacksize.wrapping_mul(2 as libc::c_int as libc::c_ulong);
            if (10000 as libc::c_int as libc::c_ulong) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_ulong;
            }
            let mut yyss1: *mut yytype_int16 = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                yystacksize
                    .wrapping_mul(
                        (::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<HTMLSTYPE>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 137226434401907431;
                break;
            }
            let mut yynewbytes: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yytype_int16 as *mut libc::c_void,
                yyss as *const libc::c_void,
                yysize
                    .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            let mut yynewbytes_0: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut HTMLSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<HTMLSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                .wrapping_mul(::std::mem::size_of::<HTMLSTYPE>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes_0
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 4497610576369232937;
                break;
            }
        }
        if yystate == 31 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 5946473855673060815;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(82 as libc::c_int) {
                current_block = 6674535790272155866;
            } else {
                if htmlchar == -(2 as libc::c_int) {
                    htmlchar = htmllex();
                }
                if htmlchar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    htmlchar = yytoken;
                } else {
                    yytoken = if htmlchar as libc::c_uint
                        <= 295 as libc::c_int as libc::c_uint
                    {
                        yytranslate[htmlchar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (271 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 6674535790272155866;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 3996196671532516695;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                        }
                        htmlchar = -(2 as libc::c_int);
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = htmllval;
                        current_block = 880553558318343340;
                    }
                }
            }
            match current_block {
                6674535790272155866 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if htmlchar == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if htmlchar as libc::c_uint
                                <= 295 as libc::c_int as libc::c_uint
                            {
                            yytranslate[htmlchar as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            htmlnerrs += 1;
                            htmlerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if htmlchar <= 0 as libc::c_int {
                                if htmlchar == 0 as libc::c_int {
                                    current_block = 4497610576369232937;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut htmllval,
                                );
                                htmlchar = -(2 as libc::c_int);
                            }
                        }
                        yyerrstatus = 3 as libc::c_int;
                        loop {
                            yyn = yypact[yystate as usize] as libc::c_int;
                            if !(yyn == -(82 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn && yyn <= 271 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 4497610576369232937;
                                break 'c_9111;
                            }
                            yydestruct(
                                b"Error: popping\0" as *const u8 as *const libc::c_char,
                                yystos[yystate as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            yystate = *yyssp as libc::c_int;
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = htmllval;
                        yystate = yyn;
                        current_block = 880553558318343340;
                    } else {
                        current_block = 3996196671532516695;
                    }
                }
                _ => {}
            }
            match current_block {
                3996196671532516695 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            HTMLstate
                                .lbl = mkLabel(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).txt
                                    as *mut libc::c_void,
                                2 as libc::c_int as libc::c_char,
                            );
                        }
                        3 => {
                            HTMLstate
                                .lbl = mkLabel(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).tbl
                                    as *mut libc::c_void,
                                1 as libc::c_int as libc::c_char,
                            );
                        }
                        4 => {
                            cleanup();
                            current_block = 4497610576369232937;
                            break;
                        }
                        5 => {
                            yyval.txt = mkText();
                        }
                        8 => {
                            appendFItemList(HTMLstate.str_0);
                        }
                        9 => {
                            appendFLineList(
                                (*yyvsp.offset(0 as libc::c_int as isize)).i,
                            );
                        }
                        18 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        19 => {
                            popFont();
                        }
                        20 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        21 => {
                            popFont();
                        }
                        22 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        23 => {
                            popFont();
                        }
                        24 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        25 => {
                            popFont();
                        }
                        26 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        27 => {
                            popFont();
                        }
                        28 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        29 => {
                            popFont();
                        }
                        30 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        31 => {
                            popFont();
                        }
                        32 => {
                            pushFont((*yyvsp.offset(0 as libc::c_int as isize)).font);
                        }
                        33 => {
                            popFont();
                        }
                        34 => {
                            yyval.i = (*yyvsp.offset(-(1 as libc::c_int) as isize)).i;
                        }
                        35 => {
                            yyval.i = (*yyvsp.offset(0 as libc::c_int as isize)).i;
                        }
                        38 => {
                            if nonSpace(agxbuse(HTMLstate.str_0)) != 0 {
                                htmlerror(
                                    b"Syntax error: non-space string used before <TABLE>\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                cleanup();
                                current_block = 4497610576369232937;
                                break;
                            } else {
                                let ref mut fresh30 = (*(*yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .tbl)
                                    .u
                                    .p
                                    .prev;
                                *fresh30 = HTMLstate.tblstack;
                                let ref mut fresh31 = (*(*yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .tbl)
                                    .u
                                    .p
                                    .rows;
                                *fresh31 = dtopen(&mut rowDisc, Dtqueue);
                                HTMLstate
                                    .tblstack = (*yyvsp.offset(0 as libc::c_int as isize)).tbl;
                                let ref mut fresh32 = (*(*yyvsp
                                    .offset(0 as libc::c_int as isize))
                                    .tbl)
                                    .font;
                                *fresh32 = (*HTMLstate.fontstack).cfont;
                                yyval.tbl = (*yyvsp.offset(0 as libc::c_int as isize)).tbl;
                            }
                        }
                        39 => {
                            if nonSpace(agxbuse(HTMLstate.str_0)) != 0 {
                                htmlerror(
                                    b"Syntax error: non-space string used after </TABLE>\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                cleanup();
                                current_block = 4497610576369232937;
                                break;
                            } else {
                                yyval.tbl = HTMLstate.tblstack;
                                HTMLstate.tblstack = (*HTMLstate.tblstack).u.p.prev;
                            }
                        }
                        40 => {
                            yyval.tbl = (*yyvsp.offset(0 as libc::c_int as isize)).tbl;
                        }
                        41 => {
                            yyval
                                .tbl = (*yyvsp.offset(-(1 as libc::c_int) as isize)).tbl;
                        }
                        42 => {
                            yyval
                                .tbl = (*yyvsp.offset(-(1 as libc::c_int) as isize)).tbl;
                        }
                        43 => {
                            yyval
                                .tbl = (*yyvsp.offset(-(1 as libc::c_int) as isize)).tbl;
                        }
                        44 => {
                            yyval
                                .tbl = (*yyvsp.offset(-(1 as libc::c_int) as isize)).tbl;
                        }
                        45 => {
                            yyval
                                .tbl = (*yyvsp.offset(-(1 as libc::c_int) as isize)).tbl;
                        }
                        48 => {
                            yyval.p = (*yyvsp.offset(0 as libc::c_int as isize)).p;
                        }
                        49 => {
                            yyval.p = (*yyvsp.offset(0 as libc::c_int as isize)).p;
                        }
                        50 => {
                            (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).p)
                                .ruled = 1 as libc::c_int as libc::c_uchar;
                            yyval.p = (*yyvsp.offset(0 as libc::c_int as isize)).p;
                        }
                        51 => {
                            addRow();
                        }
                        52 => {
                            yyval.p = lastRow();
                        }
                        53 => {
                            yyval.cell = (*yyvsp.offset(0 as libc::c_int as isize)).cell;
                        }
                        54 => {
                            yyval.cell = (*yyvsp.offset(0 as libc::c_int as isize)).cell;
                        }
                        55 => {
                            let ref mut fresh33 = (*(*yyvsp
                                .offset(-(2 as libc::c_int) as isize))
                                .cell)
                                .ruled;
                            *fresh33 = (*fresh33 as libc::c_int | 1 as libc::c_int)
                                as libc::c_uchar;
                            yyval.cell = (*yyvsp.offset(0 as libc::c_int as isize)).cell;
                        }
                        56 => {
                            setCell(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).cell,
                                (*yyvsp.offset(0 as libc::c_int as isize)).tbl
                                    as *mut libc::c_void,
                                1 as libc::c_int as libc::c_char,
                            );
                        }
                        57 => {
                            yyval
                                .cell = (*yyvsp.offset(-(3 as libc::c_int) as isize)).cell;
                        }
                        58 => {
                            setCell(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).cell,
                                (*yyvsp.offset(0 as libc::c_int as isize)).txt
                                    as *mut libc::c_void,
                                2 as libc::c_int as libc::c_char,
                            );
                        }
                        59 => {
                            yyval
                                .cell = (*yyvsp.offset(-(3 as libc::c_int) as isize)).cell;
                        }
                        60 => {
                            setCell(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).cell,
                                (*yyvsp.offset(0 as libc::c_int as isize)).img
                                    as *mut libc::c_void,
                                3 as libc::c_int as libc::c_char,
                            );
                        }
                        61 => {
                            yyval
                                .cell = (*yyvsp.offset(-(3 as libc::c_int) as isize)).cell;
                        }
                        62 => {
                            setCell(
                                (*yyvsp.offset(0 as libc::c_int as isize)).cell,
                                mkText() as *mut libc::c_void,
                                2 as libc::c_int as libc::c_char,
                            );
                        }
                        63 => {
                            yyval
                                .cell = (*yyvsp.offset(-(2 as libc::c_int) as isize)).cell;
                        }
                        64 => {
                            yyval
                                .img = (*yyvsp.offset(-(1 as libc::c_int) as isize)).img;
                        }
                        65 => {
                            yyval.img = (*yyvsp.offset(0 as libc::c_int as isize)).img;
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    yyn = yyr1[yyn as usize] as libc::c_int;
                    yystate = yypgoto[(yyn - 41 as libc::c_int) as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    if 0 as libc::c_int <= yystate && yystate <= 271 as libc::c_int
                        && yycheck[yystate as usize] as libc::c_int
                            == *yyssp as libc::c_int
                    {
                        yystate = yytable[yystate as usize] as libc::c_int;
                    } else {
                        yystate = yydefgoto[(yyn - 41 as libc::c_int) as usize]
                            as libc::c_int;
                    }
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
        }
    }
    match current_block {
        137226434401907431 => {
            htmlerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        4497610576369232937 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if htmlchar != -(2 as libc::c_int) {
        yytoken = if htmlchar as libc::c_uint <= 295 as libc::c_int as libc::c_uint {
            yytranslate[htmlchar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut htmllval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
