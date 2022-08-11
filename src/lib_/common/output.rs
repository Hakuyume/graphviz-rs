#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agcanonStr(str: *mut libc::c_char) -> *mut libc::c_char;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agset(
        obj: *mut libc::c_void,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    static mut State: libc::c_int;
    static mut Y_invert: libc::c_int;
    static mut N_height: *mut Agsym_t;
    static mut N_width: *mut Agsym_t;
    static mut N_color: *mut Agsym_t;
    static mut N_fillcolor: *mut Agsym_t;
    static mut N_label: *mut Agsym_t;
    static mut N_style: *mut Agsym_t;
    static mut N_vertices: *mut Agsym_t;
    static mut E_color: *mut Agsym_t;
    static mut E_style: *mut Agsym_t;
    fn gv_fixLocale(set: libc::c_int);
    fn undoClusterEdges(g: *mut graph_t);
    fn isPolygon(_: *mut node_t) -> bool;
    fn safe_dcl(
        g: *mut graph_t,
        obj_kind: libc::c_int,
        name: *mut libc::c_char,
        def: *mut libc::c_char,
    ) -> *mut attrsym_t;
    fn late_nnstring(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;
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
pub type attrsym_t = Agsym_s;
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
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
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
unsafe extern "C" fn agxbpop(mut xb: *mut agxbuf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if (*xb).ptr > (*xb).buf {
        let ref mut fresh4 = (*xb).ptr;
        let fresh5 = *fresh4;
        *fresh4 = (*fresh4).offset(-1);
        c = *fresh5 as libc::c_int;
        return c;
    } else {
        return -(1 as libc::c_int)
    };
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
    let ref mut fresh6 = (*xb).buf;
    *fresh6 = nbuf;
    let ref mut fresh7 = (*xb).ptr;
    *fresh7 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh8 = (*xb).eptr;
    *fresh8 = ((*xb).buf).offset(nsize as isize);
}
#[inline]
unsafe extern "C" fn agxbprint(
    mut xb: *mut agxbuf,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut size: size_t = 0;
    let mut result: libc::c_int = 0;
    ap = args.clone();
    let mut ap2: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap2 = ap.clone();
    rc = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap2.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    size = (rc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long
        as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int agxbprint(agxbuf *, const char *, ...)\0"))
                .as_ptr(),
        );
    }
    if result > 0 as libc::c_int {
        let ref mut fresh9 = (*xb).ptr;
        *fresh9 = (*fresh9).offset(result as size_t as isize);
    }
    return result;
}
#[inline]
unsafe extern "C" fn agxbput_n(
    mut xb: *mut agxbuf,
    mut s: *const libc::c_char,
    mut ssz: size_t,
) -> size_t {
    if ((*xb).ptr).offset(ssz as isize) > (*xb).eptr {
        agxbmore(xb, ssz);
    }
    memcpy((*xb).ptr as *mut libc::c_void, s as *const libc::c_void, ssz);
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = (*fresh10).offset(ssz as isize);
    return ssz;
}
#[inline]
unsafe extern "C" fn agxbput(mut xb: *mut agxbuf, mut s: *const libc::c_char) -> size_t {
    let mut ssz: size_t = strlen(s);
    return agxbput_n(xb, s, ssz);
}
#[inline]
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh11 = (*xb).ptr;
    let fresh12 = *fresh11;
    *fresh11 = (*fresh11).offset(1);
    *fresh12 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh13 = (*xb).ptr;
    *fresh13 = (*xb).buf;
    return (*xb).ptr;
}
static mut Y_off: libc::c_double = 0.;
static mut YF_off: libc::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn yDir(mut y: libc::c_double) -> libc::c_double {
    return if Y_invert != 0 { Y_off - y } else { y };
}
static mut putstr: Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
> = None;
unsafe extern "C" fn agputs(mut s: *const libc::c_char, mut fp: *mut FILE) {
    putstr.expect("non-null function pointer")(fp as *mut libc::c_void, s);
}
unsafe extern "C" fn agputc(mut c: libc::c_int, mut fp: *mut FILE) {
    static mut buf: [libc::c_char; 2] = [
        '\0' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    buf[0 as libc::c_int as usize] = c as libc::c_char;
    putstr
        .expect("non-null function pointer")(fp as *mut libc::c_void, buf.as_mut_ptr());
}
unsafe extern "C" fn printstring(
    mut f: *mut FILE,
    mut prefix: *mut libc::c_char,
    mut s: *mut libc::c_char,
) {
    if !prefix.is_null() {
        agputs(prefix, f);
    }
    agputs(s, f);
}
unsafe extern "C" fn printint(
    mut f: *mut FILE,
    mut prefix: *mut libc::c_char,
    mut i: libc::c_int,
) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if !prefix.is_null() {
        agputs(prefix, f);
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        i,
    );
    agputs(buf.as_mut_ptr(), f);
}
unsafe extern "C" fn printdouble(
    mut f: *mut FILE,
    mut prefix: *mut libc::c_char,
    mut v: libc::c_double,
) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if !prefix.is_null() {
        agputs(prefix, f);
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"%.5g\0" as *const u8 as *const libc::c_char,
        v,
    );
    agputs(buf.as_mut_ptr(), f);
}
unsafe extern "C" fn printpoint(mut f: *mut FILE, mut p: pointf) {
    printdouble(
        f,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        p.x / 72 as libc::c_int as libc::c_double,
    );
    printdouble(
        f,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (if Y_invert != 0 { Y_off - p.y } else { p.y })
            / 72 as libc::c_int as libc::c_double,
    );
}
unsafe extern "C" fn setYInvert(mut g: *mut graph_t) {
    if Y_invert != 0 {
        Y_off = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
            + (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y;
        YF_off = Y_off / 72 as libc::c_int as libc::c_double;
    }
}
unsafe extern "C" fn canon(
    mut g: *mut graph_t,
    mut s: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ns: *mut libc::c_char = agstrdup(g, s);
    let mut cs: *mut libc::c_char = agcanonStr(ns);
    agstrfree(g, ns);
    return cs;
}
unsafe extern "C" fn writenodeandport(
    mut f: *mut FILE,
    mut node: *mut node_t,
    mut port: *mut libc::c_char,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*((*(node as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
        name = canon(
            agraphof(node as *mut libc::c_void),
            (strchr(agnameof(node as *mut libc::c_void), ':' as i32))
                .offset(1 as libc::c_int as isize),
        );
    } else {
        name = agcanonStr(agnameof(node as *mut libc::c_void));
    }
    printstring(
        f,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name,
    );
    if !port.is_null() && *port as libc::c_int != 0 {
        printstring(
            f,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agcanonStr(port),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn write_plain(
    mut job: *mut GVJ_t,
    mut g: *mut graph_t,
    mut f: *mut FILE,
    mut extend: bool,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut splinePoints: libc::c_int = 0;
    let mut tport: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hport: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut pt: pointf = pointf { x: 0., y: 0. };
    let mut lbl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fillcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    putstr = (*(*(*g).clos).disc.io).putstr;
    setYInvert(g);
    pt = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR;
    printdouble(
        f,
        b"graph \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*job).zoom,
    );
    printdouble(
        f,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pt.x / 72 as libc::c_int as libc::c_double,
    );
    printdouble(
        f,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pt.y / 72 as libc::c_int as libc::c_double,
    );
    agputc('\n' as i32, f);
    n = agfstnode(g);
    while !n.is_null() {
        if !(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
            printstring(
                f,
                b"node \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                agcanonStr(agnameof(n as *mut libc::c_void)),
            );
            printpoint(f, (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord);
            if (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).html {
                lbl = agcanonStr(agxget(n as *mut libc::c_void, N_label));
            } else {
                lbl = canon(
                    agraphof(n as *mut libc::c_void),
                    (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).text,
                );
            }
            printdouble(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width,
            );
            printdouble(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height,
            );
            printstring(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                lbl,
            );
            printstring(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                late_nnstring(
                    n as *mut libc::c_void,
                    N_style,
                    b"solid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            printstring(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
            );
            printstring(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                late_nnstring(
                    n as *mut libc::c_void,
                    N_color,
                    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            fillcolor = late_nnstring(
                n as *mut libc::c_void,
                N_fillcolor,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if *fillcolor.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                fillcolor = late_nnstring(
                    n as *mut libc::c_void,
                    N_color,
                    b"lightgrey\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            printstring(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fillcolor,
            );
            agputc('\n' as i32, f);
        }
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if extend {
                tport = agget(
                    e as *mut libc::c_void,
                    b"tailport\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                if tport.is_null() {
                    tport = b"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                hport = agget(
                    e as *mut libc::c_void,
                    b"headport\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                if hport.is_null() {
                    hport = b"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
            } else {
                hport = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                tport = hport;
            }
            if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
                splinePoints = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i
                    < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size
                {
                    bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
                        .list)
                        .offset(i as isize);
                    splinePoints += bz.size;
                    i += 1;
                }
                printstring(
                    f,
                    0 as *mut libc::c_char,
                    b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                writenodeandport(
                    f,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node,
                    tport,
                );
                writenodeandport(
                    f,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                    hport,
                );
                printint(
                    f,
                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    splinePoints,
                );
                i = 0 as libc::c_int;
                while i
                    < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size
                {
                    bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
                        .list)
                        .offset(i as isize);
                    j = 0 as libc::c_int;
                    while j < bz.size {
                        printpoint(f, *(bz.list).offset(j as isize));
                        j += 1;
                    }
                    i += 1;
                }
            }
            if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
                printstring(
                    f,
                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    canon(
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
                        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                            .text,
                    ),
                );
                printpoint(
                    f,
                    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos,
                );
            }
            printstring(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                late_nnstring(
                    e as *mut libc::c_void,
                    E_style,
                    b"solid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            printstring(
                f,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                late_nnstring(
                    e as *mut libc::c_void,
                    E_color,
                    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            agputc('\n' as i32, f);
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    agputs(b"stop\n\0" as *const u8 as *const libc::c_char, f);
}
unsafe extern "C" fn set_record_rects(
    mut n: *mut node_t,
    mut f: *mut field_t,
    mut xb: *mut agxbuf,
) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if (*f).n_flds == 0 as libc::c_int {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            b"%.5g,%.5g,%.5g,%.5g \0" as *const u8 as *const libc::c_char,
            (*f).b.LL.x + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x,
            if Y_invert != 0 {
                Y_off
                    - ((*f).b.LL.y
                        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y)
            } else {
                (*f).b.LL.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            },
            (*f).b.UR.x + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x,
            if Y_invert != 0 {
                Y_off
                    - ((*f).b.UR.y
                        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y)
            } else {
                (*f).b.UR.y
                    + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            },
        );
        agxbput(xb, buf.as_mut_ptr());
    }
    i = 0 as libc::c_int;
    while i < (*f).n_flds {
        set_record_rects(n, *((*f).fld).offset(i as isize), xb);
        i += 1;
    }
}
unsafe extern "C" fn rec_attach_bb(
    mut g: *mut graph_t,
    mut bbsym: *mut Agsym_t,
    mut lpsym: *mut Agsym_t,
    mut lwsym: *mut Agsym_t,
    mut lhsym: *mut Agsym_t,
) {
    let mut c: libc::c_int = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut pt: pointf = pointf { x: 0., y: 0. };
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"%.5g,%.5g,%.5g,%.5g\0" as *const u8 as *const libc::c_char,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x,
        if Y_invert != 0 {
            Y_off - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
        } else {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
        },
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x,
        if Y_invert != 0 {
            Y_off - (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
        } else {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
        },
    );
    agxset(g as *mut libc::c_void, bbsym, buf.as_mut_ptr());
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && *((*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).text)
            .offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        pt = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).pos;
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
            pt.x,
            if Y_invert != 0 { Y_off - pt.y } else { pt.y },
        );
        agxset(g as *mut libc::c_void, lpsym, buf.as_mut_ptr());
        pt = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).dimen;
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            b"%.2f\0" as *const u8 as *const libc::c_char,
            pt.x / 72 as libc::c_int as libc::c_double,
        );
        agxset(g as *mut libc::c_void, lwsym, buf.as_mut_ptr());
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            b"%.2f\0" as *const u8 as *const libc::c_char,
            pt.y / 72 as libc::c_int as libc::c_double,
        );
        agxset(g as *mut libc::c_void, lhsym, buf.as_mut_ptr());
    }
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        rec_attach_bb(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
            bbsym,
            lpsym,
            lwsym,
            lhsym,
        );
        c += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn attach_attrs_and_arrows(
    mut g: *mut graph_t,
    mut sp: *mut libc::c_int,
    mut ep: *mut libc::c_int,
) {
    let mut e_arrows: libc::c_int = 0;
    let mut s_arrows: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut xbuffer: [libc::c_char; 8192] = [0; 8192];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut ptf: pointf = pointf { x: 0., y: 0. };
    let mut dim3: libc::c_int = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .odim as libc::c_int >= 3 as libc::c_int) as libc::c_int;
    let mut bbsym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut lpsym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut lwsym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut lhsym: *mut Agsym_t = 0 as *mut Agsym_t;
    gv_fixLocale(1 as libc::c_int);
    s_arrows = 0 as libc::c_int;
    e_arrows = s_arrows;
    setYInvert(g);
    agxbinit(&mut xb, 8192 as libc::c_int as libc::c_uint, xbuffer.as_mut_ptr());
    safe_dcl(
        g,
        1 as libc::c_int,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    safe_dcl(
        g,
        1 as libc::c_int,
        b"rects\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    N_width = safe_dcl(
        g,
        1 as libc::c_int,
        b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    N_height = safe_dcl(
        g,
        1 as libc::c_int,
        b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    safe_dcl(
        g,
        2 as libc::c_int,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 4 as libc::c_int != 0
    {
        safe_dcl(
            g,
            1 as libc::c_int,
            b"xlp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        safe_dcl(
            g,
            2 as libc::c_int,
            b"lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 5 as libc::c_int != 0
    {
        safe_dcl(
            g,
            2 as libc::c_int,
            b"xlp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 1 as libc::c_int != 0
    {
        safe_dcl(
            g,
            2 as libc::c_int,
            b"head_lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 2 as libc::c_int != 0
    {
        safe_dcl(
            g,
            2 as libc::c_int,
            b"tail_lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 3 as libc::c_int != 0
    {
        lpsym = safe_dcl(
            g,
            0 as libc::c_int,
            b"lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        lwsym = safe_dcl(
            g,
            0 as libc::c_int,
            b"lwidth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        lhsym = safe_dcl(
            g,
            0 as libc::c_int,
            b"lheight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    bbsym = safe_dcl(
        g,
        0 as libc::c_int,
        b"bb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    n = agfstnode(g);
    while !n.is_null() {
        if dim3 != 0 {
            let mut k: libc::c_int = 0;
            agxbprint(
                &mut xb as *mut agxbuf,
                b"%.5g,%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x,
                if Y_invert != 0 {
                    Y_off - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                } else {
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                },
                72 as libc::c_int as libc::c_double
                    * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(2 as libc::c_int as isize),
            );
            k = 3 as libc::c_int;
            while k
                < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).odim
                    as libc::c_int
            {
                agxbprint(
                    &mut xb as *mut agxbuf,
                    b",%.5g\0" as *const u8 as *const libc::c_char,
                    72 as libc::c_int as libc::c_double
                        * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                            .offset(k as isize),
                );
                k += 1;
            }
            agset(
                n as *mut libc::c_void,
                b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                agxbuse(&mut xb),
            );
        } else {
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x,
                if Y_invert != 0 {
                    Y_off - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                } else {
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                },
            );
            agset(
                n as *mut libc::c_void,
                b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
            );
        }
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            b"%.5g\0" as *const u8 as *const libc::c_char,
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                / 72 as libc::c_int as libc::c_double,
        );
        agxset(n as *mut libc::c_void, N_height, buf.as_mut_ptr());
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            b"%.5g\0" as *const u8 as *const libc::c_char,
            ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
                / 72 as libc::c_int as libc::c_double,
        );
        agxset(n as *mut libc::c_void, N_width, buf.as_mut_ptr());
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null()
            && (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).set
                as libc::c_int != 0
        {
            ptf = (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).pos;
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                ptf.x,
                if Y_invert != 0 { Y_off - ptf.y } else { ptf.y },
            );
            agset(
                n as *mut libc::c_void,
                b"xlp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
            );
        }
        if strcmp(
            (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).name,
            b"record\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            set_record_rects(
                n,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
                    as *mut field_t,
                &mut xb,
            );
            agxbpop(&mut xb);
            agset(
                n as *mut libc::c_void,
                b"rects\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                agxbuse(&mut xb),
            );
        } else {
            let mut poly: *mut polygon_t = 0 as *mut polygon_t;
            let mut i_0: libc::c_int = 0;
            if !N_vertices.is_null() && isPolygon(n) as libc::c_int != 0 {
                poly = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info
                    as *mut polygon_t;
                sides = (*poly).sides;
                if sides < 3 as libc::c_int {
                    let mut p: *mut libc::c_char = agget(
                        n as *mut libc::c_void,
                        b"samplepoints\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    if !p.is_null() {
                        sides = atoi(p);
                    } else {
                        sides = 8 as libc::c_int;
                    }
                    if sides < 3 as libc::c_int {
                        sides = 8 as libc::c_int;
                    }
                }
                i_0 = 0 as libc::c_int;
                while i_0 < sides {
                    if i_0 > 0 as libc::c_int {
                        agxbputc(&mut xb, ' ' as i32 as libc::c_char);
                    }
                    if (*poly).sides >= 3 as libc::c_int {
                        agxbprint(
                            &mut xb as *mut agxbuf,
                            b"%.5g %.5g\0" as *const u8 as *const libc::c_char,
                            (*((*poly).vertices).offset(i_0 as isize)).x
                                / 72 as libc::c_int as libc::c_double,
                            if Y_invert != 0 {
                                YF_off
                                    - (*((*poly).vertices).offset(i_0 as isize)).y
                                        / 72 as libc::c_int as libc::c_double
                            } else {
                                (*((*poly).vertices).offset(i_0 as isize)).y
                                    / 72 as libc::c_int as libc::c_double
                            },
                        );
                    } else {
                        agxbprint(
                            &mut xb as *mut agxbuf,
                            b"%.5g %.5g\0" as *const u8 as *const libc::c_char,
                            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                                / 2.0f64
                                * cos(
                                    i_0 as libc::c_double / sides as libc::c_double
                                        * 3.14159265358979323846f64 * 2.0f64,
                                ),
                            if Y_invert != 0 {
                                YF_off
                                    - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                        .height / 2.0f64
                                        * sin(
                                            i_0 as libc::c_double / sides as libc::c_double
                                                * 3.14159265358979323846f64 * 2.0f64,
                                        )
                            } else {
                                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                                    / 2.0f64
                                    * sin(
                                        i_0 as libc::c_double / sides as libc::c_double
                                            * 3.14159265358979323846f64 * 2.0f64,
                                    )
                            },
                        );
                    }
                    i_0 += 1;
                }
                agxset(n as *mut libc::c_void, N_vertices, agxbuse(&mut xb));
            }
        }
        if State >= 1 as libc::c_int {
            e = agfstout(g, n);
            while !e.is_null() {
                if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
                    as libc::c_int == 6 as libc::c_int)
                {
                    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl)
                        .is_null()
                    {
                        i = 0 as libc::c_int;
                        while i
                            < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .spl)
                                .size
                        {
                            if i > 0 as libc::c_int {
                                agxbputc(&mut xb, ';' as i32 as libc::c_char);
                            }
                            if (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .spl)
                                .list)
                                .offset(i as isize))
                                .sflag != 0
                            {
                                s_arrows = 1 as libc::c_int;
                                snprintf(
                                    buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 8192]>()
                                        as libc::c_ulong,
                                    b"s,%.5g,%.5g \0" as *const u8 as *const libc::c_char,
                                    (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                        .spl)
                                        .list)
                                        .offset(i as isize))
                                        .sp
                                        .x,
                                    if Y_invert != 0 {
                                        Y_off
                                            - (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                                .spl)
                                                .list)
                                                .offset(i as isize))
                                                .sp
                                                .y
                                    } else {
                                        (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                            .spl)
                                            .list)
                                            .offset(i as isize))
                                            .sp
                                            .y
                                    },
                                );
                                agxbput(&mut xb, buf.as_mut_ptr());
                            }
                            if (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .spl)
                                .list)
                                .offset(i as isize))
                                .eflag != 0
                            {
                                e_arrows = 1 as libc::c_int;
                                snprintf(
                                    buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 8192]>()
                                        as libc::c_ulong,
                                    b"e,%.5g,%.5g \0" as *const u8 as *const libc::c_char,
                                    (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                        .spl)
                                        .list)
                                        .offset(i as isize))
                                        .ep
                                        .x,
                                    if Y_invert != 0 {
                                        Y_off
                                            - (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                                .spl)
                                                .list)
                                                .offset(i as isize))
                                                .ep
                                                .y
                                    } else {
                                        (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                            .spl)
                                            .list)
                                            .offset(i as isize))
                                            .ep
                                            .y
                                    },
                                );
                                agxbput(&mut xb, buf.as_mut_ptr());
                            }
                            j = 0 as libc::c_int;
                            while j
                                < (*((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                    .spl)
                                    .list)
                                    .offset(i as isize))
                                    .size
                            {
                                if j > 0 as libc::c_int {
                                    agxbputc(&mut xb, ' ' as i32 as libc::c_char);
                                }
                                ptf = *((*((*(*((*(e as *mut Agobj_t)).data
                                    as *mut Agedgeinfo_t))
                                    .spl)
                                    .list)
                                    .offset(i as isize))
                                    .list)
                                    .offset(j as isize);
                                agxbprint(
                                    &mut xb as *mut agxbuf,
                                    b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                                    ptf.x,
                                    if Y_invert != 0 { Y_off - ptf.y } else { ptf.y },
                                );
                                j += 1;
                            }
                            i += 1;
                        }
                        agset(
                            e as *mut libc::c_void,
                            b"pos\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            agxbuse(&mut xb),
                        );
                        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                            .is_null()
                        {
                            ptf = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .label)
                                .pos;
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as libc::c_ulong,
                                b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                                ptf.x,
                                if Y_invert != 0 { Y_off - ptf.y } else { ptf.y },
                            );
                            agset(
                                e as *mut libc::c_void,
                                b"lp\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                buf.as_mut_ptr(),
                            );
                        }
                        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .xlabel)
                            .is_null()
                            && (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .xlabel)
                                .set as libc::c_int != 0
                        {
                            ptf = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .xlabel)
                                .pos;
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as libc::c_ulong,
                                b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                                ptf.x,
                                if Y_invert != 0 { Y_off - ptf.y } else { ptf.y },
                            );
                            agset(
                                e as *mut libc::c_void,
                                b"xlp\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                buf.as_mut_ptr(),
                            );
                        }
                        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .head_label)
                            .is_null()
                        {
                            ptf = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .head_label)
                                .pos;
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as libc::c_ulong,
                                b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                                ptf.x,
                                if Y_invert != 0 { Y_off - ptf.y } else { ptf.y },
                            );
                            agset(
                                e as *mut libc::c_void,
                                b"head_lp\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                buf.as_mut_ptr(),
                            );
                        }
                        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .tail_label)
                            .is_null()
                        {
                            ptf = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .tail_label)
                                .pos;
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as libc::c_ulong,
                                b"%.5g,%.5g\0" as *const u8 as *const libc::c_char,
                                ptf.x,
                                if Y_invert != 0 { Y_off - ptf.y } else { ptf.y },
                            );
                            agset(
                                e as *mut libc::c_void,
                                b"tail_lp\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                buf.as_mut_ptr(),
                            );
                        }
                    }
                }
                e = agnxtout(g, e);
            }
        }
        n = agnxtnode(g, n);
    }
    rec_attach_bb(g, bbsym, lpsym, lwsym, lhsym);
    agxbfree(&mut xb);
    if !(aggetrec(
        g as *mut libc::c_void,
        b"cl_edge_info\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ))
        .is_null()
    {
        undoClusterEdges(g);
    }
    *sp = s_arrows;
    *ep = e_arrows;
    gv_fixLocale(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn attach_attrs(mut g: *mut graph_t) {
    let mut e: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    attach_attrs_and_arrows(g, &mut s, &mut e);
}
