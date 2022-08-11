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
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agpushdisc(g: *mut Agraph_t, disc: *mut Agcbdisc_t, state: *mut libc::c_void);
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agsubedge(
        g: *mut Agraph_t,
        e: *mut Agedge_t,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agcontains(_: *mut Agraph_t, _: *mut libc::c_void) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Agstrictdirected: Agdesc_t;
    static mut Verbose: libc::c_uchar;
    static mut CL_type: libc::c_int;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn zrealloc(
        _: *mut libc::c_void,
        _: size_t,
        _: size_t,
        _: size_t,
    ) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn UF_find(_: *mut Agnode_t) -> *mut Agnode_t;
    fn UF_union(_: *mut Agnode_t, _: *mut Agnode_t) -> *mut Agnode_t;
    fn UF_singleton(_: *mut Agnode_t);
    fn mapBool(_: *const libc::c_char, _: bool) -> bool;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn maptoken(
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn is_a_cluster(g: *mut Agraph_t) -> bool;
    fn do_graph_label(sg: *mut graph_t);
    fn rank(g: *mut graph_t, balance: libc::c_int, maxiter: libc::c_int) -> libc::c_int;
    fn rank2(
        g: *mut graph_t,
        balance: libc::c_int,
        maxiter: libc::c_int,
        search_size: libc::c_int,
    ) -> libc::c_int;
    fn rank3(g: *mut graph_t, asp: *mut aspect_t);
    fn initEdgeTypes(g: *mut graph_t);
    fn init_UF_size(g: *mut graph_t);
    fn acyclic(_: *mut Agraph_t);
    fn class1(_: *mut Agraph_t);
    fn decompose(_: *mut Agraph_t, _: libc::c_int);
    fn reverse_edge(_: *mut Agedge_t);
    fn virtual_edge(
        _: *mut Agnode_t,
        _: *mut Agnode_t,
        _: *mut Agedge_t,
    ) -> *mut Agedge_t;
    fn dot_root(_: *mut libc::c_void) -> *mut Agraph_t;
    static mut E_constr: *mut Agsym_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aspect_t {
    pub targetAR: libc::c_double,
    pub combiAR: libc::c_double,
    pub prevIterations: libc::c_int,
    pub curIterations: libc::c_int,
    pub nextIter: libc::c_int,
    pub nPasses: libc::c_int,
    pub badGraph: libc::c_int,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn renewlist(mut L: *mut elist) {
    let mut i: libc::c_int = 0;
    i = (*L).size;
    while i >= 0 as libc::c_int {
        let ref mut fresh0 = *((*L).list).offset(i as isize);
        *fresh0 = 0 as *mut edge_t;
        i -= 1;
    }
    (*L).size = 0 as libc::c_int;
}
unsafe extern "C" fn cleanup1(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    let mut c: libc::c_int = 0;
    c = 0 as libc::c_int;
    while c < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size {
        let ref mut fresh1 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
        *fresh1 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.list)
            .offset(c as isize);
        n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
        while !n.is_null() {
            renewlist(&mut (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0);
            renewlist(&mut (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out);
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .mark = 0 as libc::c_int as size_t;
            n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
        }
        c += 1;
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            f = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
            if !f.is_null()
                && e != (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig
            {
                let ref mut fresh2 = (*((*(e as *mut Agobj_t)).data
                    as *mut Agedgeinfo_t))
                    .to_virt;
                *fresh2 = 0 as *mut edge_t;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            f = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
            if !f.is_null()
                && (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig == e
            {
                free((*f).base.data as *mut libc::c_void);
                free(f as *mut libc::c_void);
                let ref mut fresh3 = (*((*(e as *mut Agobj_t)).data
                    as *mut Agedgeinfo_t))
                    .to_virt;
                *fresh3 = 0 as *mut edge_t;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    free(
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.list
            as *mut libc::c_void,
    );
    let ref mut fresh4 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.list;
    *fresh4 = 0 as *mut *mut node_t;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size = 0 as libc::c_int;
}
unsafe extern "C" fn edgelabel_ranks(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        n = agfstnode(g);
        while !n.is_null() {
            e = agfstout(g, n);
            while !e.is_null() {
                let ref mut fresh5 = (*((*(e as *mut Agobj_t)).data
                    as *mut Agedgeinfo_t))
                    .minlen;
                *fresh5 = (*fresh5 as libc::c_int * 2 as libc::c_int) as libc::c_ushort;
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .ranksep = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ranksep
            + 1 as libc::c_int) / 2 as libc::c_int;
    }
}
unsafe extern "C" fn collapse_rankset(
    mut g: *mut graph_t,
    mut subg: *mut graph_t,
    mut kind: libc::c_int,
) {
    let mut u: *mut node_t = 0 as *mut node_t;
    let mut v: *mut node_t = 0 as *mut node_t;
    v = agfstnode(subg);
    u = v;
    if !u.is_null() {
        (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .ranktype = kind as libc::c_char;
        loop {
            v = agnxtnode(subg, v);
            if v.is_null() {
                break;
            }
            UF_union(u, v);
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .ranktype = (*((*(u as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .ranktype;
        }
        match kind {
            2 | 3 => {
                if ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset)
                    .is_null()
                {
                    let ref mut fresh6 = (*((*(g as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .minset;
                    *fresh6 = u;
                } else {
                    let ref mut fresh7 = (*((*(g as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .minset;
                    *fresh7 = UF_union(
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset,
                        u,
                    );
                }
            }
            4 | 5 => {
                if ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset)
                    .is_null()
                {
                    let ref mut fresh8 = (*((*(g as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .maxset;
                    *fresh8 = u;
                } else {
                    let ref mut fresh9 = (*((*(g as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .maxset;
                    *fresh9 = UF_union(
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset,
                        u,
                    );
                }
            }
            _ => {}
        }
        match kind {
            3 => {
                (*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset
                    as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .ranktype = kind as libc::c_char;
            }
            5 => {
                (*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset
                    as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .ranktype = kind as libc::c_char;
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn rank_set_class(mut g: *mut graph_t) -> libc::c_int {
    static mut name: [*mut libc::c_char; 6] = [
        b"same\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"min\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"max\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    static mut class: [libc::c_int; 6] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut val: libc::c_int = 0;
    if is_cluster(g) {
        return 7 as libc::c_int;
    }
    val = maptoken(
        agget(
            g as *mut libc::c_void,
            b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        name.as_mut_ptr(),
        class.as_mut_ptr(),
    );
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).set_type = val as libc::c_char;
    return val;
}
unsafe extern "C" fn make_new_cluster(
    mut g: *mut graph_t,
    mut subg: *mut graph_t,
) -> libc::c_int {
    let mut cno: libc::c_int = 0;
    let ref mut fresh10 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .n_cluster;
    *fresh10 += 1;
    cno = *fresh10;
    let ref mut fresh11 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust;
    *fresh11 = if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
        .is_null()
    {
        zrealloc(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust
                as *mut libc::c_void,
            (cno + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut graph_t>() as libc::c_ulong,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster as size_t,
        ) as *mut *mut graph_t
    } else {
        zmalloc(
            ((cno + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut graph_t>() as libc::c_ulong),
        ) as *mut *mut graph_t
    };
    let ref mut fresh12 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
        .offset(cno as isize);
    *fresh12 = subg;
    do_graph_label(subg);
    return cno;
}
unsafe extern "C" fn node_induce(mut par: *mut graph_t, mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut nn: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    n = agfstnode(g);
    while !n.is_null() {
        nn = agnxtnode(g, n);
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ranktype != 0 {
            agdelete(g, n as *mut libc::c_void);
        } else {
            i = 1 as libc::c_int;
            while i < (*((*(par as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
                if agcontains(
                    *((*((*(par as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                        .offset(i as isize),
                    n as *mut libc::c_void,
                ) != 0
                {
                    break;
                }
                i += 1;
            }
            if i < (*((*(par as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
                agdelete(g, n as *mut libc::c_void);
            }
            let ref mut fresh13 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .clust;
            *fresh13 = 0 as *mut graph_t;
        }
        n = nn;
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(dot_root(g as *mut libc::c_void), n);
        while !e.is_null() {
            if agcontains(
                g,
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut libc::c_void,
            ) != 0
            {
                agsubedge(g, e, 1 as libc::c_int);
            }
            e = agnxtout(dot_root(g as *mut libc::c_void), e);
        }
        n = agnxtnode(g, n);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dot_scan_ranks(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut leader: *mut node_t = 0 as *mut node_t;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .minrank = 2147483647 as libc::c_int;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank = -(1 as libc::c_int);
    n = agfstnode(g);
    while !n.is_null() {
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
        {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .maxrank = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        }
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            > (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
        {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .minrank = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        }
        if leader.is_null() {
            leader = n;
        } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                < (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            {
            leader = n;
        }
        n = agnxtnode(g, n);
    }
    let ref mut fresh14 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).leader;
    *fresh14 = leader;
}
unsafe extern "C" fn cluster_leader(mut clust: *mut graph_t) {
    let mut leader: *mut node_t = 0 as *mut node_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut maxrank: libc::c_int = 0 as libc::c_int;
    leader = 0 as *mut node_t;
    n = (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank == 0 as libc::c_int
            && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                as libc::c_int == 0 as libc::c_int
        {
            leader = n;
        }
        if maxrank < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank {
            maxrank = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    if !leader.is_null() {} else {
        __assert_fail(
            b"leader != NULL\0" as *const u8 as *const libc::c_char,
            b"rank.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void cluster_leader(graph_t *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh15 = (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .leader;
    *fresh15 = leader;
    n = agfstnode(clust);
    while !n.is_null() {
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).UF_size
            <= 1 as libc::c_int || n == leader
        {} else {
            __assert_fail(
                b"ND_UF_size(n) <= 1 || n == leader\0" as *const u8
                    as *const libc::c_char,
                b"rank.c\0" as *const u8 as *const libc::c_char,
                237 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void cluster_leader(graph_t *)\0"))
                    .as_ptr(),
            );
        }
        UF_union(n, leader);
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .ranktype = 7 as libc::c_int as libc::c_char;
        n = agnxtnode(clust, n);
    }
}
unsafe extern "C" fn collapse_cluster(mut g: *mut graph_t, mut subg: *mut graph_t) {
    if !((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).parent).is_null() {
        return;
    }
    let ref mut fresh16 = (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .parent;
    *fresh16 = g;
    node_induce(g, subg);
    if (agfstnode(subg)).is_null() {
        return;
    }
    make_new_cluster(g, subg);
    if CL_type == 100 as libc::c_int {
        dot1_rank(subg, 0 as *mut aspect_t);
        cluster_leader(subg);
    } else {
        dot_scan_ranks(subg);
    };
}
unsafe extern "C" fn collapse_sets(mut rg: *mut graph_t, mut g: *mut graph_t) {
    let mut c: libc::c_int = 0;
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        c = rank_set_class(subg);
        if c != 0 {
            if c == 7 as libc::c_int && CL_type == 100 as libc::c_int {
                collapse_cluster(rg, subg);
            } else {
                collapse_rankset(rg, subg, c);
            }
        } else {
            collapse_sets(rg, subg);
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn find_clusters(mut g: *mut graph_t) {
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    subg = agfstsubg(dot_root(g as *mut libc::c_void));
    while !subg.is_null() {
        if (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).set_type
            as libc::c_int == 7 as libc::c_int
        {
            collapse_cluster(g, subg);
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn set_minmax(mut g: *mut graph_t) {
    let mut c: libc::c_int = 0;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
        += (*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).leader
            as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
        += (*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).leader
            as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        set_minmax(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
        );
        c += 1;
    }
}
unsafe extern "C" fn minmax_edges(mut g: *mut graph_t) -> point {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut slen: point = point { x: 0, y: 0 };
    slen.y = 0 as libc::c_int;
    slen.x = slen.y;
    if ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset).is_null()
        && ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset).is_null()
    {
        return slen;
    }
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset).is_null() {
        let ref mut fresh17 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .minset;
        *fresh17 = UF_find((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset);
    }
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset).is_null() {
        let ref mut fresh18 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .maxset;
        *fresh18 = UF_find((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset);
    }
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset;
    if !n.is_null() {
        slen
            .y = ((*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset
            as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .ranktype as libc::c_int == 5 as libc::c_int) as libc::c_int;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize);
            if e.is_null() {
                break;
            }
            if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node
                == UF_find(
                    (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node,
                )
            {} else {
                __assert_fail(
                    b"aghead(e) == UF_find(aghead(e))\0" as *const u8
                        as *const libc::c_char,
                    b"rank.c\0" as *const u8 as *const libc::c_char,
                    341 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 30],
                        &[libc::c_char; 30],
                    >(b"point minmax_edges(graph_t *)\0"))
                        .as_ptr(),
                );
            }
            reverse_edge(e);
        }
    }
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset;
    if !n.is_null() {
        slen
            .x = ((*((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset
            as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .ranktype as libc::c_int == 3 as libc::c_int) as libc::c_int;
        loop {
            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(0 as libc::c_int as isize);
            if e.is_null() {
                break;
            }
            if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node
                == UF_find(
                    (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    }))
                        .node,
                )
            {} else {
                __assert_fail(
                    b"agtail(e) == UF_find(agtail(e))\0" as *const u8
                        as *const libc::c_char,
                    b"rank.c\0" as *const u8 as *const libc::c_char,
                    348 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 30],
                        &[libc::c_char; 30],
                    >(b"point minmax_edges(graph_t *)\0"))
                        .as_ptr(),
                );
            }
            reverse_edge(e);
        }
    }
    return slen;
}
unsafe extern "C" fn minmax_edges2(mut g: *mut graph_t, mut slen: point) -> libc::c_int {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset).is_null()
        || !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset).is_null()
    {
        n = agfstnode(g);
        while !n.is_null() {
            if !(n != UF_find(n)) {
                if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                    == 0 as libc::c_int
                    && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset)
                        .is_null()
                    && n != (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset
                {
                    e = virtual_edge(
                        n,
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxset,
                        0 as *mut Agedge_t,
                    );
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .minlen = slen.y as libc::c_ushort;
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .weight = 0 as libc::c_int;
                }
                if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                    == 0 as libc::c_int
                    && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset)
                        .is_null()
                    && n != (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset
                {
                    e = virtual_edge(
                        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minset,
                        n,
                        0 as *mut Agedge_t,
                    );
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .minlen = slen.x as libc::c_ushort;
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .weight = 0 as libc::c_int;
                }
            }
            n = agnxtnode(g, n);
        }
    }
    return (e != 0 as *mut edge_t) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rank1(mut g: *mut graph_t) {
    let mut maxiter: libc::c_int = 2147483647 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = agget(
        g as *mut libc::c_void,
        b"nslimit1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        maxiter = (atof(s) * agnnodes(g) as libc::c_double) as libc::c_int;
    }
    c = 0 as libc::c_int;
    while c < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size {
        let ref mut fresh19 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .nlist;
        *fresh19 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.list)
            .offset(c as isize);
        rank(
            g,
            if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
            maxiter,
        );
        c += 1;
    }
}
unsafe extern "C" fn expand_ranksets(mut g: *mut graph_t, mut asp: *mut aspect_t) {
    let mut c: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut leader: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    if !n.is_null() {
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .minrank = 2147483647 as libc::c_int;
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .maxrank = -(1 as libc::c_int);
        while !n.is_null() {
            leader = UF_find(n);
            if leader != n
                && (asp.is_null()
                    || (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                        == 0 as libc::c_int)
            {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                    += (*((*(leader as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
            }
            if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
                < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            {
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .maxrank = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .rank;
            }
            if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
                > (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            {
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .minrank = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .rank;
            }
            if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ranktype
                as libc::c_int != 0
                && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ranktype
                    as libc::c_int != 6 as libc::c_int
            {
                UF_singleton(n);
            }
            n = agnxtnode(g, n);
        }
        if g == dot_root(g as *mut libc::c_void) {
            if CL_type == 100 as libc::c_int {
                c = 1 as libc::c_int;
                while c
                    <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster
                {
                    set_minmax(
                        *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                            .offset(c as isize),
                    );
                    c += 1;
                }
            } else {
                find_clusters(g);
            }
        }
    } else {
        let ref mut fresh20 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .maxrank;
        *fresh20 = 0 as libc::c_int;
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank = *fresh20;
    };
}
unsafe extern "C" fn dot1_rank(mut g: *mut graph_t, mut asp: *mut aspect_t) {
    let mut p: point = point { x: 0, y: 0 };
    edgelabel_ranks(g);
    if !asp.is_null() {
        init_UF_size(g);
        initEdgeTypes(g);
    }
    collapse_sets(g, g);
    class1(g);
    p = minmax_edges(g);
    decompose(g, 0 as libc::c_int);
    if !asp.is_null()
        && ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).comp.size
            > 1 as libc::c_int
            || (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster
                > 0 as libc::c_int)
    {
        (*asp).badGraph = 1 as libc::c_int;
        asp = 0 as *mut aspect_t;
    }
    acyclic(g);
    if minmax_edges2(g, p) != 0 {
        decompose(g, 0 as libc::c_int);
    }
    if !asp.is_null() {
        rank3(g, asp);
    } else {
        rank1(g);
    }
    expand_ranksets(g, asp);
    cleanup1(g);
}
#[no_mangle]
pub unsafe extern "C" fn dot_rank(mut g: *mut graph_t, mut asp: *mut aspect_t) {
    if !(agget(
        g as *mut libc::c_void,
        b"newrank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .is_null()
    {
        let ref mut fresh21 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .flags;
        *fresh21 = (*fresh21 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as libc::c_ushort;
        dot2_rank(g, asp);
    } else {
        dot1_rank(g, asp);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Maxrank = %d, minrank = %d\n\0" as *const u8 as *const libc::c_char,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_cluster(mut g: *mut graph_t) -> bool {
    return is_a_cluster(g);
}
unsafe extern "C" fn set_parent(mut g: *mut graph_t, mut p: *mut graph_t) {
    let ref mut fresh22 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).parent;
    *fresh22 = p;
    make_new_cluster(p, g);
    node_induce(p, g);
}
unsafe extern "C" fn is_empty(mut g: *mut graph_t) -> bool {
    return (agfstnode(g)).is_null();
}
unsafe extern "C" fn is_a_strong_cluster(mut g: *mut graph_t) -> bool {
    let mut str: *mut libc::c_char = agget(
        g as *mut libc::c_void,
        b"compact\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return mapBool(str, 0 as libc::c_int != 0);
}
unsafe extern "C" fn rankset_kind(mut g: *mut graph_t) -> libc::c_int {
    let mut str: *mut libc::c_char = agget(
        g as *mut libc::c_void,
        b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        if strcmp(str, b"min\0" as *const u8 as *const libc::c_char) == 0 {
            return 2 as libc::c_int;
        }
        if strcmp(str, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            return 3 as libc::c_int;
        }
        if strcmp(str, b"max\0" as *const u8 as *const libc::c_char) == 0 {
            return 4 as libc::c_int;
        }
        if strcmp(str, b"sink\0" as *const u8 as *const libc::c_char) == 0 {
            return 5 as libc::c_int;
        }
        if strcmp(str, b"same\0" as *const u8 as *const libc::c_char) == 0 {
            return 1 as libc::c_int;
        }
    }
    return 6 as libc::c_int;
}
unsafe extern "C" fn is_nonconstraint(mut e: *mut edge_t) -> bool {
    let mut constr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !E_constr.is_null()
        && {
            constr = agxget(e as *mut libc::c_void, E_constr);
            !constr.is_null()
        }
    {
        if *constr.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && !mapbool(constr)
        {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn find(mut n: *mut node_t) -> *mut node_t {
    let mut set: *mut node_t = 0 as *mut node_t;
    set = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).set;
    if !set.is_null() {
        if set != n {
            let ref mut fresh23 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .set;
            *fresh23 = find(set);
            set = *fresh23;
        }
    } else {
        let ref mut fresh24 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).set;
        *fresh24 = n;
        set = *fresh24;
    }
    return set;
}
unsafe extern "C" fn union_one(
    mut leader: *mut node_t,
    mut n: *mut node_t,
) -> *mut node_t {
    if !n.is_null() {
        let ref mut fresh25 = (*((*(find(n) as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .set;
        *fresh25 = find(leader);
        return *fresh25;
    } else {
        return leader
    };
}
unsafe extern "C" fn union_all(mut g: *mut graph_t) -> *mut node_t {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut leader: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    if n.is_null() {
        return n;
    }
    leader = find(n);
    loop {
        n = agnxtnode(g, n);
        if n.is_null() {
            break;
        }
        union_one(leader, n);
    }
    return leader;
}
unsafe extern "C" fn compile_samerank(
    mut ug: *mut graph_t,
    mut parent_clust: *mut graph_t,
) {
    let mut s: *mut graph_t = 0 as *mut graph_t;
    let mut clust: *mut graph_t = 0 as *mut graph_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut leader: *mut node_t = 0 as *mut node_t;
    if is_empty(ug) {
        return;
    }
    if is_a_cluster(ug) {
        clust = ug;
        if !parent_clust.is_null() {
            (*((*(ug as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .level = (*((*(parent_clust as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .level + 1 as libc::c_int;
            set_parent(ug, parent_clust);
        } else {
            (*((*(ug as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .level = 0 as libc::c_int;
        }
    } else {
        clust = parent_clust;
    }
    s = agfstsubg(ug);
    while !s.is_null() {
        compile_samerank(s, clust);
        s = agnxtsubg(s);
    }
    if is_a_cluster(ug) {
        n = agfstnode(ug);
        while !n.is_null() {
            if ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null() {
                let ref mut fresh26 = (*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .clust;
                *fresh26 = ug;
            }
            n = agnxtnode(ug, n);
        }
    }
    let mut current_block_32: u64;
    match rankset_kind(ug) {
        3 => {
            (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .has_sourcerank = 1 as libc::c_int != 0;
            current_block_32 = 6168486577495969577;
        }
        2 => {
            current_block_32 = 6168486577495969577;
        }
        5 => {
            (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .has_sinkrank = 1 as libc::c_int != 0;
            current_block_32 = 6723228742371396536;
        }
        4 => {
            current_block_32 = 6723228742371396536;
        }
        1 => {
            leader = union_all(ug);
            current_block_32 = 6417057564578538666;
        }
        6 => {
            current_block_32 = 6417057564578538666;
        }
        _ => {
            agerr(
                AGWARN,
                b"%s has unrecognized rank=%s\0" as *const u8 as *const libc::c_char,
                agnameof(ug as *mut libc::c_void),
                agget(
                    ug as *mut libc::c_void,
                    b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            current_block_32 = 6417057564578538666;
        }
    }
    match current_block_32 {
        6723228742371396536 => {
            leader = union_all(ug);
            if !clust.is_null() {
                let ref mut fresh28 = (*((*(clust as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .maxrep;
                *fresh28 = union_one(
                    leader,
                    (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrep,
                );
            }
        }
        6168486577495969577 => {
            leader = union_all(ug);
            if !clust.is_null() {
                let ref mut fresh27 = (*((*(clust as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .minrep;
                *fresh27 = union_one(
                    leader,
                    (*((*(clust as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrep,
                );
            }
        }
        _ => {}
    }
    if is_a_cluster(ug) as libc::c_int != 0
        && !((*((*(ug as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrep).is_null()
    {
        if (*((*(ug as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrep
            == (*((*(ug as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrep
        {
            let mut up: *mut node_t = union_all(ug);
            let ref mut fresh29 = (*((*(ug as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .minrep;
            *fresh29 = up;
            let ref mut fresh30 = (*((*(ug as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .maxrep;
            *fresh30 = up;
        }
    }
}
unsafe extern "C" fn dot_lca(
    mut c0: *mut graph_t,
    mut c1: *mut graph_t,
) -> *mut graph_t {
    while c0 != c1 {
        if (*((*(c0 as *mut Agobj_t)).data as *mut Agraphinfo_t)).level
            >= (*((*(c1 as *mut Agobj_t)).data as *mut Agraphinfo_t)).level
        {
            c0 = (*((*(c0 as *mut Agobj_t)).data as *mut Agraphinfo_t)).parent;
        } else {
            c1 = (*((*(c1 as *mut Agobj_t)).data as *mut Agraphinfo_t)).parent;
        }
    }
    return c0;
}
unsafe extern "C" fn is_internal_to_cluster(mut e: *mut edge_t) -> bool {
    let mut par: *mut graph_t = 0 as *mut graph_t;
    let mut ct: *mut graph_t = 0 as *mut graph_t;
    let mut ch: *mut graph_t = 0 as *mut graph_t;
    ct = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .clust;
    ch = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .clust;
    if ct == ch {
        return 1 as libc::c_int != 0;
    }
    par = dot_lca(ct, ch);
    if par == ct || par == ch {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
static mut Last_node: *mut node_t = 0 as *const node_t as *mut node_t;
unsafe extern "C" fn makeXnode(
    mut G: *mut graph_t,
    mut name: *mut libc::c_char,
) -> *mut node_t {
    let mut n: *mut node_t = agnode(G, name, 1 as libc::c_int);
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size = 0 as libc::c_int;
    let ref mut fresh31 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .in_0
        .list;
    *fresh31 = gcalloc(
        (4 as libc::c_int + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
    ) as *mut *mut edge_t;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size = 0 as libc::c_int;
    let ref mut fresh32 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list;
    *fresh32 = gcalloc(
        (4 as libc::c_int + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
    ) as *mut *mut edge_t;
    if !Last_node.is_null() {
        let ref mut fresh33 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).prev;
        *fresh33 = Last_node;
        let ref mut fresh34 = (*((*(Last_node as *mut Agobj_t)).data
            as *mut Agnodeinfo_t))
            .next;
        *fresh34 = n;
    } else {
        let ref mut fresh35 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).prev;
        *fresh35 = 0 as *mut node_t;
        let ref mut fresh36 = (*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .nlist;
        *fresh36 = n;
    }
    Last_node = n;
    let ref mut fresh37 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    *fresh37 = 0 as *mut node_t;
    return n;
}
unsafe extern "C" fn compile_nodes(mut g: *mut graph_t, mut Xg: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    Last_node = 0 as *mut node_t;
    n = agfstnode(g);
    while !n.is_null() {
        if find(n) == n {
            let ref mut fresh38 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .rep;
            *fresh38 = makeXnode(Xg, agnameof(n as *mut libc::c_void));
        }
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        if ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rep).is_null() {
            let ref mut fresh39 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .rep;
            *fresh39 = (*((*(find(n) as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rep;
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn merge(
    mut e: *mut edge_t,
    mut minlen: libc::c_int,
    mut weight: libc::c_int,
) {
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .minlen = (if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
        as libc::c_int > minlen
    {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int
    } else {
        minlen
    }) as libc::c_ushort;
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight += weight;
}
unsafe extern "C" fn strong(
    mut g: *mut graph_t,
    mut t: *mut node_t,
    mut h: *mut node_t,
    mut orig: *mut edge_t,
) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    e = agedge(g, t, h, 0 as *mut libc::c_char, 0 as libc::c_int);
    if !e.is_null()
        || {
            e = agedge(g, h, t, 0 as *mut libc::c_char, 0 as libc::c_int);
            !e.is_null()
        }
        || {
            e = agedge(g, t, h, 0 as *mut libc::c_char, 1 as libc::c_int);
            !e.is_null()
        }
    {
        merge(
            e,
            (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                as libc::c_int,
            (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight,
        );
    } else {
        agerr(
            AGERR,
            b"ranking: failure to create strong constraint edge between nodes %s and %s\n\0"
                as *const u8 as *const libc::c_char,
            agnameof(t as *mut libc::c_void),
            agnameof(h as *mut libc::c_void),
        );
    };
}
unsafe extern "C" fn weak(
    mut g: *mut graph_t,
    mut t: *mut node_t,
    mut h: *mut node_t,
    mut orig: *mut edge_t,
) {
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    static mut id: libc::c_int = 0;
    let mut buf: [libc::c_char; 100] = [0; 100];
    e = agfstin(g, t);
    while !e.is_null() {
        v = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
        f = agfstout(g, v);
        if !f.is_null()
            && (*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                f
            } else {
                f.offset(-(1 as libc::c_int as isize))
            }))
                .node == h
        {
            return;
        }
        e = agnxtin(g, e);
    }
    if e.is_null() {
        let fresh40 = id;
        id = id + 1;
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            b"_weak_%d\0" as *const u8 as *const libc::c_char,
            fresh40,
        );
        v = makeXnode(g, buf.as_mut_ptr());
        e = agedge(g, v, t, 0 as *mut libc::c_char, 1 as libc::c_int);
        f = agedge(g, v, h, 0 as *mut libc::c_char, 1 as libc::c_int);
    }
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .minlen = (if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
        as libc::c_int > 0 as libc::c_int
    {
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_ushort;
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
        += (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
            * 1000 as libc::c_int;
    (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .minlen = (if (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
        as libc::c_int
        > (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int
    {
        (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int
    } else {
        (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int
    }) as libc::c_ushort;
    (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight
        += (*((*(orig as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight;
}
unsafe extern "C" fn compile_edges(mut ug: *mut graph_t, mut Xg: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut Xt: *mut node_t = 0 as *mut node_t;
    let mut Xh: *mut node_t = 0 as *mut node_t;
    let mut tc: *mut graph_t = 0 as *mut graph_t;
    let mut hc: *mut graph_t = 0 as *mut graph_t;
    n = agfstnode(ug);
    while !n.is_null() {
        Xt = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rep;
        e = agfstout(ug, n);
        while !e.is_null() {
            if !is_nonconstraint(e) {
                Xh = (*((*(find(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                ) as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rep;
                if !(Xt == Xh) {
                    tc = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .clust;
                    hc = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .clust;
                    if is_internal_to_cluster(e) {
                        let mut clust_tail: *mut graph_t = (*((*((*if ((*(e
                            as *mut Agobj_t))
                            .tag)
                            .objtype() as libc::c_int == 3 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(1 as libc::c_int as isize)
                        })
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .clust;
                        let mut clust_head: *mut graph_t = (*((*((*if ((*(e
                            as *mut Agobj_t))
                            .tag)
                            .objtype() as libc::c_int == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        })
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .clust;
                        if !clust_tail.is_null()
                            && find(
                                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                    == 3 as libc::c_int
                                {
                                    e
                                } else {
                                    e.offset(1 as libc::c_int as isize)
                                }))
                                    .node,
                            )
                                == (*((*(clust_tail as *mut Agobj_t)).data
                                    as *mut Agraphinfo_t))
                                    .maxrep
                            || !clust_head.is_null()
                                && find(
                                    (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                        == 2 as libc::c_int
                                    {
                                        e
                                    } else {
                                        e.offset(-(1 as libc::c_int as isize))
                                    }))
                                        .node,
                                )
                                    == (*((*(clust_head as *mut Agobj_t)).data
                                        as *mut Agraphinfo_t))
                                        .minrep
                        {
                            let mut temp: *mut node_t = Xt;
                            Xt = Xh;
                            Xh = temp;
                        }
                        strong(Xg, Xt, Xh, e);
                    } else if is_a_strong_cluster(tc) as libc::c_int != 0
                            || is_a_strong_cluster(hc) as libc::c_int != 0
                        {
                        weak(Xg, Xt, Xh, e);
                    } else {
                        strong(Xg, Xt, Xh, e);
                    }
                }
            }
            e = agnxtout(ug, e);
        }
        n = agnxtnode(ug, n);
    }
}
unsafe extern "C" fn compile_clusters(
    mut g: *mut graph_t,
    mut Xg: *mut graph_t,
    mut top: *mut node_t,
    mut bot: *mut node_t,
) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut rep: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut sub: *mut graph_t = 0 as *mut graph_t;
    if is_a_cluster(g) as libc::c_int != 0 && is_a_strong_cluster(g) as libc::c_int != 0
    {
        n = agfstnode(g);
        while !n.is_null() {
            if (agfstin(g, n)).is_null() {
                rep = (*((*(find(n) as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rep;
                if top.is_null() {
                    top = makeXnode(
                        Xg,
                        b"\x7Ftop\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                agedge(Xg, top, rep, 0 as *mut libc::c_char, 1 as libc::c_int);
            }
            if (agfstout(g, n)).is_null() {
                rep = (*((*(find(n) as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rep;
                if bot.is_null() {
                    bot = makeXnode(
                        Xg,
                        b"\x7Fbot\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                agedge(Xg, rep, bot, 0 as *mut libc::c_char, 1 as libc::c_int);
            }
            n = agnxtnode(g, n);
        }
        if !top.is_null() && !bot.is_null() {
            e = agedge(Xg, top, bot, 0 as *mut libc::c_char, 1 as libc::c_int);
            merge(e, 0 as libc::c_int, 1000 as libc::c_int);
        }
    }
    sub = agfstsubg(g);
    while !sub.is_null() {
        compile_clusters(sub, Xg, top, bot);
        sub = agnxtsubg(sub);
    }
}
unsafe extern "C" fn reverse_edge2(mut g: *mut graph_t, mut e: *mut edge_t) {
    let mut rev: *mut edge_t = 0 as *mut edge_t;
    rev = agedge(
        g,
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
    if rev.is_null() {
        rev = agedge(
            g,
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
            0 as *mut libc::c_char,
            1 as libc::c_int,
        );
    }
    merge(
        rev,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen as libc::c_int,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).weight,
    );
    agdelete(g, e as *mut libc::c_void);
}
unsafe extern "C" fn dfs(mut g: *mut graph_t, mut v: *mut node_t) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    let mut w: *mut node_t = 0 as *mut node_t;
    if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark != 0 {
        return;
    }
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .onstack = 1 as libc::c_int as libc::c_char;
    e = agfstout(g, v);
    while !e.is_null() {
        f = agnxtout(g, e);
        w = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        if (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).onstack != 0 {
            reverse_edge2(g, e);
        } else if (*((*(w as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark == 0 {
            dfs(g, w);
        }
        e = f;
    }
    (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .onstack = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn break_cycles(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .mark = 0 as libc::c_int as size_t;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .onstack = 0 as libc::c_int as libc::c_char;
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        dfs(g, n);
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn setMinMax(mut g: *mut graph_t, mut doRoot: libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut leader: *mut node_t = 0 as *mut node_t;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        setMinMax(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(c as isize),
            0 as libc::c_int,
        );
        c += 1;
    }
    if ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).parent).is_null()
        && doRoot == 0
    {
        return;
    }
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .minrank = 2147483647 as libc::c_int;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank = -(1 as libc::c_int);
    n = agfstnode(g);
    while !n.is_null() {
        v = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank < v {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank = v;
        }
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank > v {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank = v;
            leader = n;
        }
        n = agnxtnode(g, n);
    }
    let ref mut fresh41 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).leader;
    *fresh41 = leader;
}
unsafe extern "C" fn readout_levels(
    mut g: *mut graph_t,
    mut Xg: *mut graph_t,
    mut ncc: libc::c_int,
) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut xn: *mut node_t = 0 as *mut node_t;
    let mut minrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut doRoot: libc::c_int = 0 as libc::c_int;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .minrank = 2147483647 as libc::c_int;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank = -(1 as libc::c_int);
    if ncc > 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        minrk = gcalloc(
            (ncc + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i = 1 as libc::c_int;
        while i <= ncc {
            *minrk.offset(i as isize) = 2147483647 as libc::c_int;
            i += 1;
        }
    }
    n = agfstnode(g);
    while !n.is_null() {
        xn = (*((*(find(n) as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rep;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .rank = (*((*(xn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank
            < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
        {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .maxrank = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        }
        if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            > (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
        {
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .minrank = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        }
        if !minrk.is_null() {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .hops = (*((*(xn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops;
            *minrk
                .offset(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops as isize,
                ) = if *minrk
                .offset(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops as isize,
                ) < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            {
                *minrk
                    .offset(
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops
                            as isize,
                    )
            } else {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            };
        }
        n = agnxtnode(g, n);
    }
    if !minrk.is_null() {
        n = agfstnode(g);
        while !n.is_null() {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                -= *minrk
                    .offset(
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops
                            as isize,
                    );
            n = agnxtnode(g, n);
        }
        doRoot = 1 as libc::c_int;
    } else if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
            > 0 as libc::c_int
        {
        let mut delta: libc::c_int = (*((*(g as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .minrank;
        n = agfstnode(g);
        while !n.is_null() {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank -= delta;
            n = agnxtnode(g, n);
        }
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank -= delta;
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank -= delta;
    }
    setMinMax(g, doRoot);
    n = agfstnode(Xg);
    while !n.is_null() {
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list
                as *mut libc::c_void,
        );
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list
                as *mut libc::c_void,
        );
        n = agnxtnode(Xg, n);
    }
    free((*((*(agfstnode(g) as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg);
    n = agfstnode(g);
    while !n.is_null() {
        let ref mut fresh42 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg;
        *fresh42 = 0 as *mut libc::c_void;
        n = agnxtnode(g, n);
    }
    if !minrk.is_null() {
        free(minrk as *mut libc::c_void);
    }
}
unsafe extern "C" fn dfscc(
    mut g: *mut graph_t,
    mut n: *mut node_t,
    mut cc: libc::c_int,
) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops == 0 as libc::c_int {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops = cc;
        e = agfstout(g, n);
        while !e.is_null() {
            dfscc(
                g,
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                cc,
            );
            e = agnxtout(g, e);
        }
        e = agfstin(g, n);
        while !e.is_null() {
            dfscc(
                g,
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node,
                cc,
            );
            e = agnxtin(g, e);
        }
    }
}
unsafe extern "C" fn connect_components(mut g: *mut graph_t) -> libc::c_int {
    let mut cc: libc::c_int = 0 as libc::c_int;
    let mut n: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops = 0 as libc::c_int;
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops == 0 as libc::c_int
        {
            cc += 1;
            dfscc(g, n, cc);
        }
        n = agnxtnode(g, n);
    }
    if cc > 1 as libc::c_int {
        let mut root: *mut node_t = makeXnode(
            g,
            b"\x7Froot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut ncc: libc::c_int = 1 as libc::c_int;
        n = agfstnode(g);
        while !n.is_null() {
            if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).hops == ncc {
                agedge(g, root, n, 0 as *mut libc::c_char, 1 as libc::c_int);
                ncc += 1;
            }
            n = agnxtnode(g, n);
        }
    }
    return cc;
}
unsafe extern "C" fn add_fast_edges(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            let ref mut fresh43 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list;
            *fresh43 = if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .is_null()
            {
                grealloc(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list
                        as *mut libc::c_void,
                    (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                        + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut edge_t
            } else {
                gmalloc(
                    (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                        + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut edge_t
            };
            let ref mut fresh44 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .size;
            let fresh45 = *fresh44;
            *fresh44 = *fresh44 + 1;
            let ref mut fresh46 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .offset(fresh45 as isize);
            *fresh46 = e;
            let ref mut fresh47 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .offset(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                        as isize,
                );
            *fresh47 = 0 as *mut edge_t;
            let ref mut fresh48 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .in_0
                .list;
            *fresh48 = if !((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .in_0
                .list)
                .is_null()
            {
                grealloc(
                    (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .in_0
                        .list as *mut libc::c_void,
                    (((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .in_0
                        .size + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut edge_t
            } else {
                gmalloc(
                    (((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .in_0
                        .size + 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut edge_t
            };
            let ref mut fresh49 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .in_0
                .size;
            let fresh50 = *fresh49;
            *fresh49 = *fresh49 + 1;
            let ref mut fresh51 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .in_0
                .list)
                .offset(fresh50 as isize);
            *fresh51 = e;
            let ref mut fresh52 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .in_0
                .list)
                .offset(
                    (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .in_0
                        .size as isize,
                );
            *fresh52 = 0 as *mut edge_t;
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn my_init_graph(
    mut g: *mut Agraph_t,
    mut graph: *mut Agobj_t,
    mut arg: *mut libc::c_void,
) {
    let mut sz: *mut libc::c_int = arg as *mut libc::c_int;
    agbindrec(
        graph as *mut libc::c_void,
        b"level graph rec\0" as *const u8 as *const libc::c_char,
        *sz.offset(0 as libc::c_int as isize) as libc::c_uint,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn my_init_node(
    mut g: *mut Agraph_t,
    mut node: *mut Agobj_t,
    mut arg: *mut libc::c_void,
) {
    let mut sz: *mut libc::c_int = arg as *mut libc::c_int;
    agbindrec(
        node as *mut libc::c_void,
        b"level node rec\0" as *const u8 as *const libc::c_char,
        *sz.offset(1 as libc::c_int as isize) as libc::c_uint,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn my_init_edge(
    mut g: *mut Agraph_t,
    mut edge: *mut Agobj_t,
    mut arg: *mut libc::c_void,
) {
    let mut sz: *mut libc::c_int = arg as *mut libc::c_int;
    agbindrec(
        edge as *mut libc::c_void,
        b"level edge rec\0" as *const u8 as *const libc::c_char,
        *sz.offset(2 as libc::c_int as isize) as libc::c_uint,
        1 as libc::c_int,
    );
}
static mut mydisc: Agcbdisc_t = unsafe {
    {
        let mut init = Agcbdisc_s {
            graph: {
                let mut init = C2RustUnnamed_3 {
                    ins: Some(
                        my_init_graph
                            as unsafe extern "C" fn(
                                *mut Agraph_t,
                                *mut Agobj_t,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    mod_0: None,
                    del: None,
                };
                init
            },
            node: {
                let mut init = C2RustUnnamed_3 {
                    ins: Some(
                        my_init_node
                            as unsafe extern "C" fn(
                                *mut Agraph_t,
                                *mut Agobj_t,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    mod_0: None,
                    del: None,
                };
                init
            },
            edge: {
                let mut init = C2RustUnnamed_3 {
                    ins: Some(
                        my_init_edge
                            as unsafe extern "C" fn(
                                *mut Agraph_t,
                                *mut Agobj_t,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    mod_0: None,
                    del: None,
                };
                init
            },
        };
        init
    }
};
#[no_mangle]
pub static mut infosizes: [libc::c_int; 3] = [
    ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
    ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
    ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_int,
];
unsafe extern "C" fn dot2_rank(mut g: *mut graph_t, mut asp: *mut aspect_t) {
    let mut ssize: libc::c_int = 0;
    let mut ncc: libc::c_int = 0;
    let mut maxiter: libc::c_int = 2147483647 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut Xg: *mut graph_t = 0 as *mut graph_t;
    Last_node = 0 as *mut node_t;
    Xg = agopen(
        b"level assignment constraints\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        Agstrictdirected,
        0 as *mut Agdisc_t,
    );
    agbindrec(
        Xg as *mut libc::c_void,
        b"level graph rec\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agpushdisc(Xg, &mut mydisc, infosizes.as_mut_ptr() as *mut libc::c_void);
    edgelabel_ranks(g);
    s = agget(
        g as *mut libc::c_void,
        b"nslimit1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        maxiter = (atof(s) * agnnodes(g) as libc::c_double) as libc::c_int;
    } else {
        maxiter = 2147483647 as libc::c_int;
    }
    compile_samerank(g, 0 as *mut graph_t);
    compile_nodes(g, Xg);
    compile_edges(g, Xg);
    compile_clusters(g, Xg, 0 as *mut node_t, 0 as *mut node_t);
    break_cycles(Xg);
    ncc = connect_components(Xg);
    add_fast_edges(Xg);
    if !asp.is_null() {
        init_UF_size(Xg);
        initEdgeTypes(Xg);
    }
    s = agget(
        g as *mut libc::c_void,
        b"searchsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null() {
        ssize = atoi(s);
    } else {
        ssize = -(1 as libc::c_int);
    }
    rank2(Xg, 1 as libc::c_int, maxiter, ssize);
    readout_levels(g, Xg, ncc);
    agclose(Xg);
}
