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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn new_queue(_: libc::c_int) -> *mut nodequeue;
    fn free_queue(_: *mut nodequeue);
    fn enqueue(_: *mut nodequeue, _: *mut Agnode_t);
    fn dequeue(_: *mut nodequeue) -> *mut Agnode_t;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
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
pub struct nodequeue {
    pub store: *mut *mut node_t,
    pub limit: *mut *mut node_t,
    pub head: *mut *mut node_t,
    pub tail: *mut *mut node_t,
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
pub type subtree_t = subtree_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subtree_s {
    pub rep: *mut node_t,
    pub size: libc::c_int,
    pub heap_index: libc::c_int,
    pub par: *mut subtree_s,
}
pub type STheap_t = STheap_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STheap_s {
    pub elt: *mut *mut subtree_t,
    pub size: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut G: *mut graph_t = 0 as *const graph_t as *mut graph_t;
static mut N_nodes: libc::c_int = 0;
static mut N_edges: libc::c_int = 0;
static mut Minrank: libc::c_int = 0;
static mut Maxrank: libc::c_int = 0;
static mut S_i: libc::c_int = 0;
static mut Search_size: libc::c_int = 0;
static mut Tree_node: nlist_t = nlist_t {
    list: 0 as *const *mut node_t as *mut *mut node_t,
    size: 0,
};
static mut Tree_edge: elist = elist {
    list: 0 as *const *mut edge_t as *mut *mut edge_t,
    size: 0,
};
unsafe extern "C" fn add_tree_edge(mut e: *mut edge_t) -> libc::c_int {
    let mut n: *mut node_t = 0 as *mut node_t;
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        >= 0 as libc::c_int
    {
        agerr(
            AGERR,
            b"add_tree_edge: missing tree edge\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index = Tree_edge.size;
    let fresh0 = Tree_edge.size;
    Tree_edge.size = Tree_edge.size + 1;
    let ref mut fresh1 = *(Tree_edge.list).offset(fresh0 as isize);
    *fresh1 = e;
    if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .mark == 0
    {
        let fresh2 = Tree_node.size;
        Tree_node.size = Tree_node.size + 1;
        let ref mut fresh3 = *(Tree_node.list).offset(fresh2 as isize);
        *fresh3 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
    }
    if (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .mark == 0
    {
        let fresh4 = Tree_node.size;
        Tree_node.size = Tree_node.size + 1;
        let ref mut fresh5 = *(Tree_node.list).offset(fresh4 as isize);
        *fresh5 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
    }
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
    let ref mut fresh6 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .size;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    let ref mut fresh8 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .list)
        .offset(fresh7 as isize);
    *fresh8 = e;
    let ref mut fresh9 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .list)
        .offset(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.size as isize,
        );
    *fresh9 = 0 as *mut edge_t;
    if (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
        .offset(
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.size
                - 1 as libc::c_int) as isize,
        ))
        .is_null()
    {
        agerr(
            AGERR,
            b"add_tree_edge: empty outedge list\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
    let ref mut fresh10 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .size;
    let fresh11 = *fresh10;
    *fresh10 = *fresh10 + 1;
    let ref mut fresh12 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .list)
        .offset(fresh11 as isize);
    *fresh12 = e;
    let ref mut fresh13 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .list)
        .offset(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.size as isize,
        );
    *fresh13 = 0 as *mut edge_t;
    if (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
        .offset(
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.size
                - 1 as libc::c_int) as isize,
        ))
        .is_null()
    {
        agerr(
            AGERR,
            b"add_tree_edge: empty inedge list\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exchange_tree_edges(mut e: *mut edge_t, mut f: *mut edge_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .tree_index = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index;
    let ref mut fresh14 = *(Tree_edge.list)
        .offset(
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index as isize,
        );
    *fresh14 = f;
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .tree_index = -(1 as libc::c_int);
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    let ref mut fresh15 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .size;
    *fresh15 -= 1;
    i = *fresh15;
    j = 0 as libc::c_int;
    while j <= i {
        if *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list)
            .offset(j as isize) == e
        {
            break;
        }
        j += 1;
    }
    let ref mut fresh16 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .list)
        .offset(j as isize);
    *fresh16 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list)
        .offset(i as isize);
    let ref mut fresh17 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .list)
        .offset(i as isize);
    *fresh17 = 0 as *mut edge_t;
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let ref mut fresh18 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .size;
    *fresh18 -= 1;
    i = *fresh18;
    j = 0 as libc::c_int;
    while j <= i {
        if *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list)
            .offset(j as isize) == e
        {
            break;
        }
        j += 1;
    }
    let ref mut fresh19 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .list)
        .offset(j as isize);
    *fresh19 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list)
        .offset(i as isize);
    let ref mut fresh20 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .list)
        .offset(i as isize);
    *fresh20 = 0 as *mut edge_t;
    n = (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        f
    } else {
        f.offset(1 as libc::c_int as isize)
    })
        .node;
    let ref mut fresh21 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .size;
    let fresh22 = *fresh21;
    *fresh21 = *fresh21 + 1;
    let ref mut fresh23 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .list)
        .offset(fresh22 as isize);
    *fresh23 = f;
    let ref mut fresh24 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_out
        .list)
        .offset(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.size as isize,
        );
    *fresh24 = 0 as *mut edge_t;
    n = (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        f
    } else {
        f.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let ref mut fresh25 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .size;
    let fresh26 = *fresh25;
    *fresh25 = *fresh25 + 1;
    let ref mut fresh27 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .list)
        .offset(fresh26 as isize);
    *fresh27 = f;
    let ref mut fresh28 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .tree_in
        .list)
        .offset(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.size as isize,
        );
    *fresh28 = 0 as *mut edge_t;
}
unsafe extern "C" fn init_rank() {
    let mut i: libc::c_int = 0;
    let mut ctr: libc::c_int = 0;
    let mut Q: *mut nodequeue = 0 as *mut nodequeue;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    Q = new_queue(N_nodes);
    ctr = 0 as libc::c_int;
    v = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !v.is_null() {
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).priority
            == 0 as libc::c_int
        {
            enqueue(Q, v);
        }
        v = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    loop {
        v = dequeue(Q);
        if v.is_null() {
            break;
        }
        (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank = 0 as libc::c_int;
        ctr += 1;
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .rank = if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                > (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rank
                    + (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int
            {
                (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            } else {
                (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rank
                    + (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int
            };
            i += 1;
        }
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            let ref mut fresh29 = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .priority;
            *fresh29 -= 1;
            if *fresh29 <= 0 as libc::c_int {
                enqueue(
                    Q,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                );
            }
            i += 1;
        }
    }
    if ctr != N_nodes {
        agerr(AGERR, b"trouble in init_rank\n\0" as *const u8 as *const libc::c_char);
        v = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
        while !v.is_null() {
            if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).priority != 0 {
                agerr(
                    AGPREV,
                    b"\t%s %d\n\0" as *const u8 as *const libc::c_char,
                    agnameof(v as *mut libc::c_void),
                    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).priority,
                );
            }
            v = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
        }
    }
    free_queue(Q);
}
unsafe extern "C" fn leave_edge() -> *mut edge_t {
    let mut f: *mut edge_t = 0 as *mut edge_t;
    let mut rv: *mut edge_t = 0 as *mut edge_t;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    j = S_i;
    while S_i < Tree_edge.size {
        f = *(Tree_edge.list).offset(S_i as isize);
        if (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue
            < 0 as libc::c_int
        {
            if !rv.is_null() {
                if (*((*(rv as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue
                    > (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue
                {
                    rv = f;
                }
            } else {
                rv = *(Tree_edge.list).offset(S_i as isize);
            }
            cnt += 1;
            if cnt >= Search_size {
                return rv;
            }
        }
        S_i += 1;
    }
    if j > 0 as libc::c_int {
        S_i = 0 as libc::c_int;
        while S_i < j {
            f = *(Tree_edge.list).offset(S_i as isize);
            if (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue
                < 0 as libc::c_int
            {
                if !rv.is_null() {
                    if (*((*(rv as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue
                        > (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue
                    {
                        rv = f;
                    }
                } else {
                    rv = *(Tree_edge.list).offset(S_i as isize);
                }
                cnt += 1;
                if cnt >= Search_size {
                    return rv;
                }
            }
            S_i += 1;
        }
    }
    return rv;
}
static mut Enter: *mut edge_t = 0 as *const edge_t as *mut edge_t;
static mut Low: libc::c_int = 0;
static mut Lim: libc::c_int = 0;
static mut Slack: libc::c_int = 0;
unsafe extern "C" fn dfs_enter_outedge(mut v: *mut node_t) {
    let mut i: libc::c_int = 0;
    let mut slack: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            >= 0 as libc::c_int)
        {
            if !(Low
                <= (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .lim
                && (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .lim <= Lim)
            {
                slack = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int;
                if slack < Slack || Enter.is_null() {
                    Enter = e;
                    Slack = slack;
                }
            }
        } else if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .lim < (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
            {
            dfs_enter_outedge(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list)
            .offset(i as isize);
        if !(!e.is_null() && Slack > 0 as libc::c_int) {
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
            .lim < (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
        {
            dfs_enter_outedge(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node,
            );
        }
        i += 1;
    };
}
unsafe extern "C" fn dfs_enter_inedge(mut v: *mut node_t) {
    let mut i: libc::c_int = 0;
    let mut slack: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            >= 0 as libc::c_int)
        {
            if !(Low
                <= (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .lim
                && (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .lim <= Lim)
            {
                slack = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int;
                if slack < Slack || Enter.is_null() {
                    Enter = e;
                    Slack = slack;
                }
            }
        } else if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .lim < (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
            {
            dfs_enter_inedge(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list)
            .offset(i as isize);
        if !(!e.is_null() && Slack > 0 as libc::c_int) {
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
            .lim < (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
        {
            dfs_enter_inedge(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
            );
        }
        i += 1;
    };
}
unsafe extern "C" fn enter_edge(mut e: *mut edge_t) -> *mut edge_t {
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut outsearch: libc::c_int = 0;
    if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .lim
        < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .lim
    {
        v = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
        outsearch = 0 as libc::c_int;
    } else {
        v = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        outsearch = (0 as libc::c_int == 0) as libc::c_int;
    }
    Enter = 0 as *mut edge_t;
    Slack = 2147483647 as libc::c_int;
    Low = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low;
    Lim = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim;
    if outsearch != 0 {
        dfs_enter_outedge(v);
    } else {
        dfs_enter_inedge(v);
    }
    return Enter;
}
unsafe extern "C" fn init_cutvalues() {
    dfs_range(
        (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist,
        0 as *mut edge_t,
        1 as libc::c_int,
    );
    dfs_cutval(
        (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist,
        0 as *mut edge_t,
    );
}
unsafe extern "C" fn tight_subtree_search(
    mut v: *mut Agnode_t,
    mut st: *mut subtree_t,
) -> libc::c_int {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    rv = 1 as libc::c_int;
    let ref mut fresh30 = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par;
    *fresh30 = st as *mut edge_t;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            >= 0 as libc::c_int)
        {
            if ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .par as *mut subtree_t)
                .is_null()
                && (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int == 0 as libc::c_int
            {
                if add_tree_edge(e) != 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                rv
                    += tight_subtree_search(
                        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        })
                            .node,
                        st,
                    );
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            >= 0 as libc::c_int)
        {
            if ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .par as *mut subtree_t)
                .is_null()
                && (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int == 0 as libc::c_int
            {
                if add_tree_edge(e) != 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                rv
                    += tight_subtree_search(
                        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        })
                            .node,
                        st,
                    );
            }
        }
        i += 1;
    }
    return rv;
}
unsafe extern "C" fn find_tight_subtree(mut v: *mut Agnode_t) -> *mut subtree_t {
    let mut rv: *mut subtree_t = 0 as *mut subtree_t;
    rv = zmalloc(::std::mem::size_of::<subtree_t>() as libc::c_ulong) as *mut subtree_t;
    let ref mut fresh31 = (*rv).rep;
    *fresh31 = v;
    (*rv).size = tight_subtree_search(v, rv);
    if (*rv).size < 0 as libc::c_int {
        free(rv as *mut libc::c_void);
        return 0 as *mut subtree_t;
    }
    let ref mut fresh32 = (*rv).par;
    *fresh32 = rv;
    return rv;
}
unsafe extern "C" fn STsetFind(mut n0: *mut Agnode_t) -> *mut subtree_t {
    let mut s0: *mut subtree_t = (*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .par as *mut subtree_t;
    while !((*s0).par).is_null() && (*s0).par != s0 {
        if !((*(*s0).par).par).is_null() {
            let ref mut fresh33 = (*s0).par;
            *fresh33 = (*(*s0).par).par;
        }
        s0 = (*s0).par;
    }
    return s0;
}
unsafe extern "C" fn STsetUnion(
    mut s0: *mut subtree_t,
    mut s1: *mut subtree_t,
) -> *mut subtree_t {
    let mut r0: *mut subtree_t = 0 as *mut subtree_t;
    let mut r1: *mut subtree_t = 0 as *mut subtree_t;
    let mut r: *mut subtree_t = 0 as *mut subtree_t;
    r0 = s0;
    while !((*r0).par).is_null() && (*r0).par != r0 {
        r0 = (*r0).par;
    }
    r1 = s1;
    while !((*r1).par).is_null() && (*r1).par != r1 {
        r1 = (*r1).par;
    }
    if r0 == r1 {
        return r0;
    }
    if (*r0).heap_index > -(1 as libc::c_int) || (*r1).heap_index > -(1 as libc::c_int)
    {} else {
        __assert_fail(
            b"r0->heap_index > -1 || r1->heap_index > -1\0" as *const u8
                as *const libc::c_char,
            b"ns.c\0" as *const u8 as *const libc::c_char,
            331 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"subtree_t *STsetUnion(subtree_t *, subtree_t *)\0"))
                .as_ptr(),
        );
    }
    if (*r1).heap_index == -(1 as libc::c_int) {
        r = r0;
    } else if (*r0).heap_index == -(1 as libc::c_int) {
        r = r1;
    } else if (*r1).size < (*r0).size {
        r = r0;
    } else {
        r = r1;
    }
    let ref mut fresh34 = (*r1).par;
    *fresh34 = r;
    let ref mut fresh35 = (*r0).par;
    *fresh35 = *fresh34;
    (*r).size = (*r0).size + (*r1).size;
    if (*r).heap_index >= 0 as libc::c_int {} else {
        __assert_fail(
            b"r->heap_index >= 0\0" as *const u8 as *const libc::c_char,
            b"ns.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"subtree_t *STsetUnion(subtree_t *, subtree_t *)\0"))
                .as_ptr(),
        );
    }
    return r;
}
unsafe extern "C" fn inter_tree_edge_search(
    mut v: *mut Agnode_t,
    mut from: *mut Agnode_t,
    mut best: *mut Agedge_t,
) -> *mut Agedge_t {
    let mut i: libc::c_int = 0;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut ts: *mut subtree_t = STsetFind(v);
    if !best.is_null()
        && (*((*((*(if ((*(best as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            best
        } else {
            best.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank
            - (*((*((*(if ((*(best as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                best
            } else {
                best.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .rank
            - (*((*(best as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                as libc::c_int == 0 as libc::c_int
    {
        return best;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            >= 0 as libc::c_int
        {
            if !((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node == from)
            {
                best = inter_tree_edge_search(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    v,
                    best,
                );
            }
        } else if STsetFind(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node,
            ) != ts
            {
            if best.is_null()
                || ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int)
                    < (*((*((*(if ((*(best as *mut Agobj_t)).tag).objtype()
                        as libc::c_int == 2 as libc::c_int
                    {
                        best
                    } else {
                        best.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                        - (*((*((*(if ((*(best as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            best
                        } else {
                            best.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .rank
                        - (*((*(best as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                            as libc::c_int
            {
                best = e;
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            >= 0 as libc::c_int
        {
            if !((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node == from)
            {
                best = inter_tree_edge_search(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node,
                    v,
                    best,
                );
            }
        } else if STsetFind(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node,
            ) != ts
            {
            if best.is_null()
                || ((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int)
                    < (*((*((*(if ((*(best as *mut Agobj_t)).tag).objtype()
                        as libc::c_int == 2 as libc::c_int
                    {
                        best
                    } else {
                        best.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                        - (*((*((*(if ((*(best as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            best
                        } else {
                            best.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .rank
                        - (*((*(best as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                            as libc::c_int
            {
                best = e;
            }
        }
        i += 1;
    }
    return best;
}
unsafe extern "C" fn inter_tree_edge(mut tree: *mut subtree_t) -> *mut Agedge_t {
    let mut rv: *mut Agedge_t = 0 as *mut Agedge_t;
    rv = inter_tree_edge_search((*tree).rep, 0 as *mut Agnode_t, 0 as *mut Agedge_t);
    return rv;
}
unsafe extern "C" fn STheapsize(mut heap: *mut STheap_t) -> libc::c_int {
    return (*heap).size;
}
unsafe extern "C" fn STheapify(mut heap: *mut STheap_t, mut i: libc::c_int) {
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut smallest: libc::c_int = 0;
    let mut elt: *mut *mut subtree_t = (*heap).elt;
    loop {
        left = 2 as libc::c_int * (i + 1 as libc::c_int) - 1 as libc::c_int;
        right = 2 as libc::c_int * (i + 1 as libc::c_int);
        if left < (*heap).size
            && (**elt.offset(left as isize)).size < (**elt.offset(i as isize)).size
        {
            smallest = left;
        } else {
            smallest = i;
        }
        if right < (*heap).size
            && (**elt.offset(right as isize)).size
                < (**elt.offset(smallest as isize)).size
        {
            smallest = right;
        } else {
            smallest = i;
        }
        if !(smallest != i) {
            break;
        }
        let mut temp: *mut subtree_t = 0 as *mut subtree_t;
        temp = *elt.offset(i as isize);
        let ref mut fresh36 = *elt.offset(i as isize);
        *fresh36 = *elt.offset(smallest as isize);
        let ref mut fresh37 = *elt.offset(smallest as isize);
        *fresh37 = temp;
        (**elt.offset(i as isize)).heap_index = i;
        (**elt.offset(smallest as isize)).heap_index = smallest;
        i = smallest;
        if !(i < (*heap).size) {
            break;
        }
    };
}
unsafe extern "C" fn STbuildheap(
    mut elt: *mut *mut subtree_t,
    mut size: libc::c_int,
) -> *mut STheap_t {
    let mut i: libc::c_int = 0;
    let mut heap: *mut STheap_t = 0 as *mut STheap_t;
    heap = zmalloc(::std::mem::size_of::<STheap_t>() as libc::c_ulong) as *mut STheap_t;
    let ref mut fresh38 = (*heap).elt;
    *fresh38 = elt;
    (*heap).size = size;
    i = 0 as libc::c_int;
    while i < (*heap).size {
        (**((*heap).elt).offset(i as isize)).heap_index = i;
        i += 1;
    }
    i = (*heap).size / 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        STheapify(heap, i);
        i -= 1;
    }
    return heap;
}
unsafe extern "C" fn STextractmin(mut heap: *mut STheap_t) -> *mut subtree_t {
    let mut rv: *mut subtree_t = 0 as *mut subtree_t;
    rv = *((*heap).elt).offset(0 as libc::c_int as isize);
    (*rv).heap_index = -(1 as libc::c_int);
    let ref mut fresh39 = *((*heap).elt).offset(0 as libc::c_int as isize);
    *fresh39 = *((*heap).elt).offset(((*heap).size - 1 as libc::c_int) as isize);
    (**((*heap).elt).offset(0 as libc::c_int as isize)).heap_index = 0 as libc::c_int;
    let ref mut fresh40 = *((*heap).elt)
        .offset(((*heap).size - 1 as libc::c_int) as isize);
    *fresh40 = rv;
    let ref mut fresh41 = (*heap).size;
    *fresh41 -= 1;
    STheapify(heap, 0 as libc::c_int);
    return rv;
}
unsafe extern "C" fn tree_adjust(
    mut v: *mut Agnode_t,
    mut from: *mut Agnode_t,
    mut delta: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut w: *mut Agnode_t = 0 as *mut Agnode_t;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .rank = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank + delta;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        w = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
        if w != from {
            tree_adjust(w, v, delta);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        w = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        if w != from {
            tree_adjust(w, v, delta);
        }
        i += 1;
    };
}
unsafe extern "C" fn merge_trees(mut e: *mut Agedge_t) -> *mut subtree_t {
    let mut delta: libc::c_int = 0;
    let mut t0: *mut subtree_t = 0 as *mut subtree_t;
    let mut t1: *mut subtree_t = 0 as *mut subtree_t;
    let mut rv: *mut subtree_t = 0 as *mut subtree_t;
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        >= 0 as libc::c_int)
    {} else {
        __assert_fail(
            b"!TREE_EDGE(e)\0" as *const u8 as *const libc::c_char,
            b"ns.c\0" as *const u8 as *const libc::c_char,
            465 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"subtree_t *merge_trees(Agedge_t *)\0"))
                .as_ptr(),
        );
    }
    t0 = STsetFind(
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node,
    );
    t1 = STsetFind(
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
    );
    if (*t0).heap_index == -(1 as libc::c_int) {
        delta = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .rank
            - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                as libc::c_int;
        tree_adjust((*t0).rep, 0 as *mut Agnode_t, delta);
    } else {
        delta = -((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .rank
            - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                as libc::c_int);
        tree_adjust((*t1).rep, 0 as *mut Agnode_t, delta);
    }
    if add_tree_edge(e) != 0 as libc::c_int {
        return 0 as *mut subtree_t;
    }
    rv = STsetUnion(t0, t1);
    return rv;
}
unsafe extern "C" fn feasible_tree() -> libc::c_int {
    let mut current_block: u64;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ee: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut tree: *mut *mut subtree_t = 0 as *mut *mut subtree_t;
    let mut tree0: *mut subtree_t = 0 as *mut subtree_t;
    let mut tree1: *mut subtree_t = 0 as *mut subtree_t;
    let mut i: libc::c_int = 0;
    let mut subtree_count: libc::c_int = 0 as libc::c_int;
    let mut heap: *mut STheap_t = 0 as *mut STheap_t;
    let mut error: libc::c_int = 0 as libc::c_int;
    n = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        let ref mut fresh42 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par;
        *fresh42 = 0 as *mut edge_t;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    tree = gcalloc(
        N_nodes as size_t,
        ::std::mem::size_of::<*mut subtree_t>() as libc::c_ulong,
    ) as *mut *mut subtree_t;
    n = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    loop {
        if n.is_null() {
            current_block = 5143058163439228106;
            break;
        }
        if ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par as *mut subtree_t)
            .is_null()
        {
            let ref mut fresh43 = *tree.offset(subtree_count as isize);
            *fresh43 = find_tight_subtree(n);
            if (*tree.offset(subtree_count as isize)).is_null() {
                error = 2 as libc::c_int;
                current_block = 17628209378111741016;
                break;
            } else {
                subtree_count += 1;
            }
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    match current_block {
        5143058163439228106 => {
            heap = STbuildheap(tree, subtree_count);
            while STheapsize(heap) > 1 as libc::c_int {
                tree0 = STextractmin(heap);
                ee = inter_tree_edge(tree0);
                if ee.is_null() {
                    error = 1 as libc::c_int;
                    break;
                } else {
                    tree1 = merge_trees(ee);
                    if tree1.is_null() {
                        error = 2 as libc::c_int;
                        break;
                    } else {
                        STheapify(heap, (*tree1).heap_index);
                    }
                }
            }
        }
        _ => {}
    }
    free(heap as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < subtree_count {
        free(*tree.offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    free(tree as *mut libc::c_void);
    if error != 0 {
        return error;
    }
    if Tree_edge.size == N_nodes - 1 as libc::c_int {} else {
        __assert_fail(
            b"Tree_edge.size == N_nodes - 1\0" as *const u8 as *const libc::c_char,
            b"ns.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"int feasible_tree(void)\0"))
                .as_ptr(),
        );
    }
    init_cutvalues();
    return 0 as libc::c_int;
}
unsafe extern "C" fn treeupdate(
    mut v: *mut Agnode_t,
    mut w: *mut Agnode_t,
    mut cutvalue: libc::c_int,
    mut dir: libc::c_int,
) -> *mut Agnode_t {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut d: libc::c_int = 0;
    while !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low
        <= (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
        && (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
            <= (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim)
    {
        e = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par;
        if v
            == (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node
        {
            d = dir;
        } else {
            d = (dir == 0) as libc::c_int;
        }
        if d != 0 {
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue += cutvalue;
        } else {
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue -= cutvalue;
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
            .lim
            > (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .lim
        {
            v = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node;
        } else {
            v = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
        }
    }
    return v;
}
unsafe extern "C" fn rerank(mut v: *mut Agnode_t, mut delta: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank -= delta;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if e != (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par {
            rerank(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                delta,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if e != (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par {
            rerank(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node,
                delta,
            );
        }
        i += 1;
    };
}
unsafe extern "C" fn update(mut e: *mut edge_t, mut f: *mut edge_t) -> libc::c_int {
    let mut cutvalue: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut lca: *mut Agnode_t = 0 as *mut Agnode_t;
    delta = (*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        f
    } else {
        f.offset(-(1 as libc::c_int as isize))
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .rank
        - (*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            f
        } else {
            f.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank
        - (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int;
    if delta > 0 as libc::c_int {
        let mut s: libc::c_int = 0;
        s = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .tree_in
            .size
            + (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .tree_out
                .size;
        if s == 1 as libc::c_int {
            rerank(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node,
                delta,
            );
        } else {
            s = (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .tree_in
                .size
                + (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .tree_out
                    .size;
            if s == 1 as libc::c_int {
                rerank(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    -delta,
                );
            } else if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .lim
                    < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .lim
                {
                rerank(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node,
                    delta,
                );
            } else {
                rerank(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    -delta,
                );
            }
        }
    }
    cutvalue = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue;
    lca = treeupdate(
        (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            f
        } else {
            f.offset(1 as libc::c_int as isize)
        })
            .node,
        (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            f
        } else {
            f.offset(-(1 as libc::c_int as isize))
        })
            .node,
        cutvalue,
        1 as libc::c_int,
    );
    if treeupdate(
        (*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            f
        } else {
            f.offset(-(1 as libc::c_int as isize))
        }))
            .node,
        (*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            f
        } else {
            f.offset(1 as libc::c_int as isize)
        }))
            .node,
        cutvalue,
        0 as libc::c_int,
    ) != lca
    {
        agerr(
            AGERR,
            b"update: mismatched lca in treeupdates\n\0" as *const u8
                as *const libc::c_char,
        );
        return 2 as libc::c_int;
    }
    (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue = -cutvalue;
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue = 0 as libc::c_int;
    exchange_tree_edges(e, f);
    dfs_range(
        lca,
        (*((*(lca as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par,
        (*((*(lca as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn scan_and_normalize() {
    let mut n: *mut node_t = 0 as *mut node_t;
    Minrank = 2147483647 as libc::c_int;
    Maxrank = -(2147483647 as libc::c_int);
    n = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 0 as libc::c_int
        {
            Minrank = if Minrank
                < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            {
                Minrank
            } else {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            };
            Maxrank = if Maxrank
                > (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            {
                Maxrank
            } else {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            };
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    if Minrank != 0 as libc::c_int {
        n = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
        while !n.is_null() {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank -= Minrank;
            n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
        }
        Maxrank -= Minrank;
        Minrank = 0 as libc::c_int;
    }
}
unsafe extern "C" fn freeTreeList(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list
                as *mut libc::c_void,
        );
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list
                as *mut libc::c_void,
        );
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .mark = 0 as libc::c_int as size_t;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn LR_balance() {
    let mut i: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    i = 0 as libc::c_int;
    while i < Tree_edge.size {
        e = *(Tree_edge.list).offset(i as isize);
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue
            == 0 as libc::c_int
        {
            f = enter_edge(e);
            if !f.is_null() {
                delta = (*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    f
                } else {
                    f.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rank
                    - (*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        f
                    } else {
                        f.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    - (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int;
                if !(delta <= 1 as libc::c_int) {
                    if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .lim
                        < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .lim
                    {
                        rerank(
                            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                == 3 as libc::c_int
                            {
                                e
                            } else {
                                e.offset(1 as libc::c_int as isize)
                            })
                                .node,
                            delta / 2 as libc::c_int,
                        );
                    } else {
                        rerank(
                            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                == 2 as libc::c_int
                            {
                                e
                            } else {
                                e.offset(-(1 as libc::c_int as isize))
                            })
                                .node,
                            -delta / 2 as libc::c_int,
                        );
                    }
                }
            }
        }
        i += 1;
    }
    freeTreeList(G);
}
unsafe extern "C" fn decreasingrankcmpf(
    mut n0: *mut *mut node_t,
    mut n1: *mut *mut node_t,
) -> libc::c_int {
    return (*((*(*n1 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
        - (*((*(*n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
}
unsafe extern "C" fn increasingrankcmpf(
    mut n0: *mut *mut node_t,
    mut n1: *mut *mut node_t,
) -> libc::c_int {
    return (*((*(*n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
        - (*((*(*n1 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
}
unsafe extern "C" fn TB_balance() {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut choice: libc::c_int = 0;
    let mut nrank: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut inweight: libc::c_int = 0;
    let mut outweight: libc::c_int = 0;
    let mut adj: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    scan_and_normalize();
    nrank = gcalloc(
        (Maxrank + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i <= Maxrank {
        *nrank.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    s = agget(
        G as *mut libc::c_void,
        b"TBbalance\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        if strcmp(s, b"min\0" as *const u8 as *const libc::c_char) == 0 {
            adj = 1 as libc::c_int;
        } else if strcmp(s, b"max\0" as *const u8 as *const libc::c_char) == 0 {
            adj = 2 as libc::c_int;
        }
        if adj != 0 {
            n = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
            while !n.is_null() {
                if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 0 as libc::c_int
                {
                    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                        == 0 as libc::c_int && adj == 1 as libc::c_int
                    {
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .rank = Minrank;
                    }
                    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                        == 0 as libc::c_int && adj == 2 as libc::c_int
                    {
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .rank = Maxrank;
                    }
                }
                n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
            }
        }
    }
    ii = 0 as libc::c_int;
    n = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        let ref mut fresh44 = *(Tree_node.list).offset(ii as isize);
        *fresh44 = n;
        ii += 1;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    Tree_node.size = ii;
    qsort(
        Tree_node.list as *mut libc::c_void,
        Tree_node.size as size_t,
        ::std::mem::size_of::<*mut node_t>() as libc::c_ulong,
        if adj > 1 as libc::c_int {
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut *mut node_t,
                        *mut *mut node_t,
                    ) -> libc::c_int,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
            >(
                Some(
                    decreasingrankcmpf
                        as unsafe extern "C" fn(
                            *mut *mut node_t,
                            *mut *mut node_t,
                        ) -> libc::c_int,
                ),
            )
        } else {
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut *mut node_t,
                        *mut *mut node_t,
                    ) -> libc::c_int,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
            >(
                Some(
                    increasingrankcmpf
                        as unsafe extern "C" fn(
                            *mut *mut node_t,
                            *mut *mut node_t,
                        ) -> libc::c_int,
                ),
            )
        },
    );
    i = 0 as libc::c_int;
    while i < Tree_node.size {
        n = *(Tree_node.list).offset(i as isize);
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 0 as libc::c_int
        {
            let ref mut fresh45 = *nrank
                .offset(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
                );
            *fresh45 += 1;
        }
        i += 1;
    }
    ii = 0 as libc::c_int;
    while ii < Tree_node.size {
        n = *(Tree_node.list).offset(ii as isize);
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
            as libc::c_int != 0 as libc::c_int)
        {
            outweight = 0 as libc::c_int;
            inweight = outweight;
            low = 0 as libc::c_int;
            high = Maxrank;
            i = 0 as libc::c_int;
            loop {
                e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                    .offset(i as isize);
                if e.is_null() {
                    break;
                }
                inweight += (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight;
                low = if low
                    > (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                        + (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                            as libc::c_int
                {
                    low
                } else {
                    (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                        + (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                            as libc::c_int
                };
                i += 1;
            }
            i = 0 as libc::c_int;
            loop {
                e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                    .offset(i as isize);
                if e.is_null() {
                    break;
                }
                outweight
                    += (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight;
                high = if high
                    < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                        - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                            as libc::c_int
                {
                    high
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
                        .rank
                        - (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                            as libc::c_int
                };
                i += 1;
            }
            if low < 0 as libc::c_int {
                low = 0 as libc::c_int;
            }
            if adj != 0 {
                if inweight == outweight {
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .rank = if adj == 1 as libc::c_int { low } else { high };
                }
            } else if inweight == outweight {
                choice = low;
                i = low + 1 as libc::c_int;
                while i <= high {
                    if *nrank.offset(i as isize) < *nrank.offset(choice as isize) {
                        choice = i;
                    }
                    i += 1;
                }
                let ref mut fresh46 = *nrank
                    .offset(
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                            as isize,
                    );
                *fresh46 -= 1;
                let ref mut fresh47 = *nrank.offset(choice as isize);
                *fresh47 += 1;
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank = choice;
            }
            free(
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list
                    as *mut libc::c_void,
            );
            free(
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list
                    as *mut libc::c_void,
            );
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .mark = 0 as libc::c_int as size_t;
        }
        ii += 1;
    }
    free(nrank as *mut libc::c_void);
}
unsafe extern "C" fn init_graph(mut g: *mut graph_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut feasible: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    G = g;
    S_i = 0 as libc::c_int;
    N_edges = S_i;
    N_nodes = N_edges;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .mark = 0 as libc::c_int as size_t;
        N_nodes += 1;
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            N_edges += 1;
            i += 1;
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    Tree_node
        .list = if !(Tree_node.list).is_null() {
        grealloc(
            Tree_node.list as *mut libc::c_void,
            (N_nodes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut node_t>() as libc::c_ulong),
        ) as *mut *mut node_t
    } else {
        gmalloc(
            (N_nodes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut node_t>() as libc::c_ulong),
        ) as *mut *mut node_t
    };
    Tree_node.size = 0 as libc::c_int;
    Tree_edge
        .list = if !(Tree_edge.list).is_null() {
        grealloc(
            Tree_edge.list as *mut libc::c_void,
            (N_nodes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
        ) as *mut *mut edge_t
    } else {
        gmalloc(
            (N_nodes as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
        ) as *mut *mut edge_t
    };
    Tree_edge.size = 0 as libc::c_int;
    feasible = (0 as libc::c_int == 0) as libc::c_int;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .priority = 0 as libc::c_int;
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            let ref mut fresh48 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .priority;
            *fresh48 += 1;
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .cutvalue = 0 as libc::c_int;
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tree_index = -(1 as libc::c_int);
            if feasible != 0
                && (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
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
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                    < (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                        as libc::c_int
            {
                feasible = 0 as libc::c_int;
            }
            i += 1;
        }
        let ref mut fresh49 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .tree_in
            .list;
        *fresh49 = gcalloc(
            (i + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .tree_in
            .size = 0 as libc::c_int;
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            i += 1;
        }
        let ref mut fresh50 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .tree_out
            .list;
        *fresh50 = gcalloc(
            (i + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .tree_out
            .size = 0 as libc::c_int;
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    return feasible;
}
unsafe extern "C" fn graphSize(
    mut g: *mut graph_t,
    mut nn: *mut libc::c_int,
    mut ne: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut nnodes: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    nedges = 0 as libc::c_int;
    nnodes = nedges;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        nnodes += 1;
        i = 0 as libc::c_int;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(i as isize);
            if e.is_null() {
                break;
            }
            nedges += 1;
            i += 1;
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    *nn = nnodes;
    *ne = nedges;
}
#[no_mangle]
pub unsafe extern "C" fn rank2(
    mut g: *mut graph_t,
    mut balance: libc::c_int,
    mut maxiter: libc::c_int,
    mut search_size: libc::c_int,
) -> libc::c_int {
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut feasible: libc::c_int = 0;
    let mut ns: *mut libc::c_char = b"network simplex: \0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    if Verbose != 0 {
        let mut nn: libc::c_int = 0;
        let mut ne: libc::c_int = 0;
        graphSize(g, &mut nn, &mut ne);
        fprintf(
            stderr,
            b"%s %d nodes %d edges maxiter=%d balance=%d\n\0" as *const u8
                as *const libc::c_char,
            ns,
            nn,
            ne,
            maxiter,
            balance,
        );
        start_timer();
    }
    feasible = init_graph(g);
    if feasible == 0 {
        init_rank();
    }
    if search_size >= 0 as libc::c_int {
        Search_size = search_size;
    } else {
        Search_size = 30 as libc::c_int;
    }
    let mut err: libc::c_int = feasible_tree();
    if err != 0 as libc::c_int {
        freeTreeList(g);
        return err;
    }
    if maxiter <= 0 as libc::c_int {
        freeTreeList(g);
        return 0 as libc::c_int;
    }
    loop {
        e = leave_edge();
        if e.is_null() {
            break;
        }
        let mut err_0: libc::c_int = 0;
        f = enter_edge(e);
        err_0 = update(e, f);
        if err_0 != 0 as libc::c_int {
            freeTreeList(g);
            return err_0;
        }
        iter += 1;
        if Verbose as libc::c_int != 0 && iter % 100 as libc::c_int == 0 as libc::c_int {
            if iter % 1000 as libc::c_int == 100 as libc::c_int {
                fputs(ns, stderr);
            }
            fprintf(stderr, b"%d \0" as *const u8 as *const libc::c_char, iter);
            if iter % 1000 as libc::c_int == 0 as libc::c_int {
                fputc('\n' as i32, stderr);
            }
        }
        if iter >= maxiter {
            break;
        }
    }
    match balance {
        1 => {
            TB_balance();
        }
        2 => {
            LR_balance();
        }
        _ => {
            scan_and_normalize();
            freeTreeList(G);
        }
    }
    if Verbose != 0 {
        if iter >= 100 as libc::c_int {
            fputc('\n' as i32, stderr);
        }
        fprintf(
            stderr,
            b"%s%d nodes %d edges %d iter %.2f sec\n\0" as *const u8
                as *const libc::c_char,
            ns,
            N_nodes,
            N_edges,
            iter,
            elapsed_sec(),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rank(
    mut g: *mut graph_t,
    mut balance: libc::c_int,
    mut maxiter: libc::c_int,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut search_size: libc::c_int = 0;
    s = agget(
        g as *mut libc::c_void,
        b"searchsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        search_size = atoi(s);
    } else {
        search_size = 30 as libc::c_int;
    }
    return rank2(g, balance, maxiter, search_size);
}
unsafe extern "C" fn x_cutval(mut f: *mut edge_t) {
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    if (*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        f
    } else {
        f.offset(1 as libc::c_int as isize)
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .par == f
    {
        v = (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            f
        } else {
            f.offset(1 as libc::c_int as isize)
        })
            .node;
        dir = 1 as libc::c_int;
    } else {
        v = (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            f
        } else {
            f.offset(-(1 as libc::c_int as isize))
        })
            .node;
        dir = -(1 as libc::c_int);
    }
    sum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        sum += x_val(e, v, dir);
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        sum += x_val(e, v, dir);
        i += 1;
    }
    (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue = sum;
}
unsafe extern "C" fn x_val(
    mut e: *mut edge_t,
    mut v: *mut node_t,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut other: *mut node_t = 0 as *mut node_t;
    let mut d: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    }))
        .node == v
    {
        other = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
    } else {
        other = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
    }
    if !((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low
        <= (*((*(other as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
        && (*((*(other as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim
            <= (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim)
    {
        f = 1 as libc::c_int;
        rv = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight;
    } else {
        f = 0 as libc::c_int;
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            >= 0 as libc::c_int
        {
            rv = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).cutvalue;
        } else {
            rv = 0 as libc::c_int;
        }
        rv -= (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight;
    }
    if dir > 0 as libc::c_int {
        if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
            .node == v
        {
            d = 1 as libc::c_int;
        } else {
            d = -(1 as libc::c_int);
        }
    } else if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node == v
        {
        d = 1 as libc::c_int;
    } else {
        d = -(1 as libc::c_int);
    }
    if f != 0 {
        d = -d;
    }
    if d < 0 as libc::c_int {
        rv = -rv;
    }
    return rv;
}
unsafe extern "C" fn dfs_cutval(mut v: *mut node_t, mut par: *mut edge_t) {
    let mut i: libc::c_int = 0;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if e != par {
            dfs_cutval(
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
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if e != par {
            dfs_cutval(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node,
                e,
            );
        }
        i += 1;
    }
    if !par.is_null() {
        x_cutval(par);
    }
}
unsafe extern "C" fn dfs_range(
    mut v: *mut node_t,
    mut par: *mut edge_t,
    mut low: libc::c_int,
) -> libc::c_int {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    lim = low;
    let ref mut fresh51 = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).par;
    *fresh51 = par;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).low = low;
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_out.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if e != par {
            lim = dfs_range(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                e,
                lim,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    loop {
        e = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).tree_in.list)
            .offset(i as isize);
        if e.is_null() {
            break;
        }
        if e != par {
            lim = dfs_range(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node,
                e,
                lim,
            );
        }
        i += 1;
    }
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lim = lim;
    return lim + 1 as libc::c_int;
}
