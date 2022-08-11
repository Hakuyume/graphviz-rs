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
    pub type gvtextlayout_engine_s;
    pub type htmllabel_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agobjkind(_: *mut libc::c_void) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agprvnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn aglstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agnxtattr(g: *mut Agraph_t, kind: libc::c_int, attr: *mut Agsym_t) -> *mut Agsym_t;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn gv_argvlist_set_item(list: *mut gv_argvlist_t, index: libc::c_int, item: *mut libc::c_char);
    fn strdup_and_subst_obj(str: *mut libc::c_char, obj: *mut libc::c_void) -> *mut libc::c_char;
    fn overlap_edge(e: *mut edge_t, b: boxf) -> bool;
    fn overlap_node(n: *mut node_t, b: boxf) -> bool;
    fn emit_graph(job: *mut GVJ_t, g: *mut graph_t);
    fn gvLayout(gvc: *mut GVC_t, g: *mut graph_t, engine: *const libc::c_char) -> libc::c_int;
    fn gvRenderFilename(
        gvc: *mut GVC_t,
        g: *mut graph_t,
        format: *const libc::c_char,
        filename: *const libc::c_char,
    ) -> libc::c_int;
    fn graph_cleanup(g: *mut graph_t);
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
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvlayout_engine_s {
    pub layout: Option<unsafe extern "C" fn(*mut graph_t) -> ()>,
    pub cleanup: Option<unsafe extern "C" fn(*mut graph_t) -> ()>,
}
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
pub type C2RustUnnamed_8 = libc::c_uint;
pub const GVATTR_COLOR: C2RustUnnamed_8 = 2;
pub const GVATTR_BOOL: C2RustUnnamed_8 = 1;
pub const GVATTR_STRING: C2RustUnnamed_8 = 0;
static mut s_digraph: *mut libc::c_char =
    b"digraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_graph: *mut libc::c_char =
    b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_subgraph: *mut libc::c_char =
    b"subgraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_node: *mut libc::c_char =
    b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_edge: *mut libc::c_char =
    b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_tooltip: *mut libc::c_char =
    b"tooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_href: *mut libc::c_char =
    b"href\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_URL: *mut libc::c_char =
    b"URL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_tailport: *mut libc::c_char =
    b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_headport: *mut libc::c_char =
    b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut s_key: *mut libc::c_char =
    b"key\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn gv_graph_state(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut j: libc::c_int = 0;
    let mut a: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut list: *mut gv_argvlist_t = 0 as *mut gv_argvlist_t;
    list = &mut (*job).selected_obj_type_name;
    j = 0 as libc::c_int;
    if g == agroot(g as *mut libc::c_void) {
        if agisdirected(g) != 0 {
            let fresh0 = j;
            j = j + 1;
            gv_argvlist_set_item(list, fresh0, s_digraph);
        } else {
            let fresh1 = j;
            j = j + 1;
            gv_argvlist_set_item(list, fresh1, s_graph);
        }
    } else {
        let fresh2 = j;
        j = j + 1;
        gv_argvlist_set_item(list, fresh2, s_subgraph);
    }
    let fresh3 = j;
    j = j + 1;
    gv_argvlist_set_item(list, fresh3, agnameof(g as *mut libc::c_void));
    (*list).argc = j;
    list = &mut (*job).selected_obj_attributes;
    a = 0 as *mut Agsym_t;
    loop {
        a = agnxtattr(g, 0 as libc::c_int, a);
        if a.is_null() {
            break;
        }
        let fresh4 = j;
        j = j + 1;
        gv_argvlist_set_item(list, fresh4, (*a).name);
        let fresh5 = j;
        j = j + 1;
        gv_argvlist_set_item(list, fresh5, agxget(g as *mut libc::c_void, a));
        let fresh6 = j;
        j = j + 1;
        gv_argvlist_set_item(list, fresh6, 0 as *mut libc::c_char);
    }
    (*list).argc = j;
    a = agattr(g, 0 as libc::c_int, s_href, 0 as *const libc::c_char);
    if a.is_null() {
        a = agattr(g, 0 as libc::c_int, s_URL, 0 as *const libc::c_char);
    }
    if !a.is_null() {
        let ref mut fresh7 = (*job).selected_href;
        *fresh7 = strdup_and_subst_obj(agxget(g as *mut libc::c_void, a), g as *mut libc::c_void);
    }
}
unsafe extern "C" fn gv_node_state(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut j: libc::c_int = 0;
    let mut a: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut list: *mut gv_argvlist_t = 0 as *mut gv_argvlist_t;
    list = &mut (*job).selected_obj_type_name;
    j = 0 as libc::c_int;
    let fresh8 = j;
    j = j + 1;
    gv_argvlist_set_item(list, fresh8, s_node);
    let fresh9 = j;
    j = j + 1;
    gv_argvlist_set_item(list, fresh9, agnameof(n as *mut libc::c_void));
    (*list).argc = j;
    list = &mut (*job).selected_obj_attributes;
    g = agroot(agraphof(n as *mut libc::c_void) as *mut libc::c_void);
    a = 0 as *mut Agsym_t;
    loop {
        a = agnxtattr(g, 1 as libc::c_int, a);
        if a.is_null() {
            break;
        }
        let fresh10 = j;
        j = j + 1;
        gv_argvlist_set_item(list, fresh10, (*a).name);
        let fresh11 = j;
        j = j + 1;
        gv_argvlist_set_item(list, fresh11, agxget(n as *mut libc::c_void, a));
    }
    (*list).argc = j;
    a = agattr(
        agraphof(n as *mut libc::c_void),
        1 as libc::c_int,
        s_href,
        0 as *const libc::c_char,
    );
    if a.is_null() {
        a = agattr(
            agraphof(n as *mut libc::c_void),
            1 as libc::c_int,
            s_URL,
            0 as *const libc::c_char,
        );
    }
    if !a.is_null() {
        let ref mut fresh12 = (*job).selected_href;
        *fresh12 = strdup_and_subst_obj(agxget(n as *mut libc::c_void, a), n as *mut libc::c_void);
    }
}
unsafe extern "C" fn gv_edge_state(mut job: *mut GVJ_t, mut e: *mut edge_t) {
    let mut j: libc::c_int = 0;
    let mut a: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut nlist: *mut gv_argvlist_t = 0 as *mut gv_argvlist_t;
    let mut alist: *mut gv_argvlist_t = 0 as *mut gv_argvlist_t;
    nlist = &mut (*job).selected_obj_type_name;
    j = 0 as libc::c_int;
    let fresh13 = j;
    j = j + 1;
    gv_argvlist_set_item(nlist, fresh13, s_edge);
    let fresh14 = j;
    j = j + 1;
    gv_argvlist_set_item(
        nlist,
        fresh14,
        agnameof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
            .node as *mut libc::c_void,
        ),
    );
    j += 1;
    let fresh15 = j;
    j = j + 1;
    gv_argvlist_set_item(
        nlist,
        fresh15,
        (if agisdirected(agraphof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
            .node as *mut libc::c_void,
        )) != 0
        {
            b"->\0" as *const u8 as *const libc::c_char
        } else {
            b"--\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
    );
    let fresh16 = j;
    j = j + 1;
    gv_argvlist_set_item(
        nlist,
        fresh16,
        agnameof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
            .node as *mut libc::c_void,
        ),
    );
    j += 1;
    j += 1;
    (*nlist).argc = j;
    alist = &mut (*job).selected_obj_attributes;
    g = agroot(agraphof(
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node as *mut libc::c_void,
    ) as *mut libc::c_void);
    a = 0 as *mut Agsym_t;
    loop {
        a = agnxtattr(g, 2 as libc::c_int, a);
        if a.is_null() {
            break;
        }
        if strcmp((*a).name, s_tailport) == 0 as libc::c_int {
            gv_argvlist_set_item(nlist, 2 as libc::c_int, agxget(e as *mut libc::c_void, a));
        } else if strcmp((*a).name, s_headport) == 0 as libc::c_int {
            gv_argvlist_set_item(nlist, 5 as libc::c_int, agxget(e as *mut libc::c_void, a));
        } else if strcmp((*a).name, s_key) == 0 as libc::c_int {
            gv_argvlist_set_item(nlist, 6 as libc::c_int, agxget(e as *mut libc::c_void, a));
            continue;
        }
        let fresh17 = j;
        j = j + 1;
        gv_argvlist_set_item(alist, fresh17, (*a).name);
        let fresh18 = j;
        j = j + 1;
        gv_argvlist_set_item(alist, fresh18, agxget(e as *mut libc::c_void, a));
    }
    (*alist).argc = j;
    a = agattr(
        agraphof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
            .node as *mut libc::c_void,
        ),
        2 as libc::c_int,
        s_href,
        0 as *const libc::c_char,
    );
    if a.is_null() {
        a = agattr(
            agraphof(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                .node as *mut libc::c_void,
            ),
            2 as libc::c_int,
            s_URL,
            0 as *const libc::c_char,
        );
    }
    if !a.is_null() {
        let ref mut fresh19 = (*job).selected_href;
        *fresh19 = strdup_and_subst_obj(agxget(e as *mut libc::c_void, a), e as *mut libc::c_void);
    }
}
unsafe extern "C" fn gvevent_refresh(mut job: *mut GVJ_t) {
    let mut g: *mut graph_t = (*(*job).gvc).g;
    if ((*job).selected_obj).is_null() {
        let ref mut fresh20 = (*job).selected_obj;
        *fresh20 = g as *mut libc::c_void;
        let ref mut fresh21 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state;
        *fresh21 =
            (*fresh21 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
        gv_graph_state(job, g);
    }
    emit_graph(job, g);
    (*job).has_been_rendered = 1 as libc::c_int != 0;
}
unsafe extern "C" fn gvevent_find_cluster(mut g: *mut graph_t, mut b: boxf) -> *mut graph_t {
    let mut i: libc::c_int = 0;
    let mut sg: *mut graph_t = 0 as *mut graph_t;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        sg = gvevent_find_cluster(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(i as isize),
            b,
        );
        if !sg.is_null() {
            return sg;
        }
        i += 1;
    }
    bb.LL.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .x;
    bb.LL.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .LL
        .y;
    bb.UR.x = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .x;
    bb.UR.y = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .bb
        .UR
        .y;
    if b.UR.x >= bb.LL.x && bb.UR.x >= b.LL.x && b.UR.y >= bb.LL.y && bb.UR.y >= b.LL.y {
        return g;
    }
    return 0 as *mut graph_t;
}
unsafe extern "C" fn gvevent_find_obj(mut g: *mut graph_t, mut b: boxf) -> *mut libc::c_void {
    let mut sg: *mut graph_t = 0 as *mut graph_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if overlap_edge(e, b) {
                return e as *mut libc::c_void;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    n = aglstnode(g);
    while !n.is_null() {
        if overlap_node(n, b) {
            return n as *mut libc::c_void;
        }
        n = agprvnode(g, n);
    }
    sg = gvevent_find_cluster(g, b);
    if !sg.is_null() {
        return sg as *mut libc::c_void;
    }
    return g as *mut libc::c_void;
}
unsafe extern "C" fn gvevent_leave_obj(mut job: *mut GVJ_t) {
    let mut obj: *mut libc::c_void = (*job).current_obj;
    if !obj.is_null() {
        match agobjkind(obj) {
            0 => {
                let ref mut fresh22 = (*((*(obj as *mut graph_t as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .gui_state;
                *fresh22 = (*fresh22 as libc::c_int
                    & !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar as libc::c_int)
                    as libc::c_uchar;
            }
            1 => {
                let ref mut fresh23 = (*((*(obj as *mut node_t as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .gui_state;
                *fresh23 = (*fresh23 as libc::c_int
                    & !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar as libc::c_int)
                    as libc::c_uchar;
            }
            2 => {
                let ref mut fresh24 = (*((*(obj as *mut edge_t as *mut Agobj_t)).data
                    as *mut Agedgeinfo_t))
                    .gui_state;
                *fresh24 = (*fresh24 as libc::c_int
                    & !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar as libc::c_int)
                    as libc::c_uchar;
            }
            _ => {}
        }
    }
    let ref mut fresh25 = (*job).active_tooltip;
    *fresh25 = 0 as *mut libc::c_char;
}
unsafe extern "C" fn gvevent_enter_obj(mut job: *mut GVJ_t) {
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut g: *mut graph_t = 0 as *mut graph_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut a: *mut Agsym_t = 0 as *mut Agsym_t;
    free((*job).active_tooltip as *mut libc::c_void);
    let ref mut fresh26 = (*job).active_tooltip;
    *fresh26 = 0 as *mut libc::c_char;
    obj = (*job).current_obj;
    if !obj.is_null() {
        match agobjkind(obj) {
            0 => {
                g = obj as *mut graph_t;
                let ref mut fresh27 =
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state;
                *fresh27 = (*fresh27 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
                    as libc::c_uchar;
                a = agattr(g, 0 as libc::c_int, s_tooltip, 0 as *const libc::c_char);
                if !a.is_null() {
                    let ref mut fresh28 = (*job).active_tooltip;
                    *fresh28 = strdup_and_subst_obj(agxget(g as *mut libc::c_void, a), obj);
                }
            }
            1 => {
                n = obj as *mut node_t;
                let ref mut fresh29 =
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state;
                *fresh29 = (*fresh29 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
                    as libc::c_uchar;
                a = agattr(
                    agraphof(n as *mut libc::c_void),
                    1 as libc::c_int,
                    s_tooltip,
                    0 as *const libc::c_char,
                );
                if !a.is_null() {
                    let ref mut fresh30 = (*job).active_tooltip;
                    *fresh30 = strdup_and_subst_obj(agxget(n as *mut libc::c_void, a), obj);
                }
            }
            2 => {
                e = obj as *mut edge_t;
                let ref mut fresh31 =
                    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state;
                *fresh31 = (*fresh31 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
                    as libc::c_uchar;
                a = agattr(
                    agraphof(
                        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        })
                        .node as *mut libc::c_void,
                    ),
                    2 as libc::c_int,
                    s_tooltip,
                    0 as *const libc::c_char,
                );
                if !a.is_null() {
                    let ref mut fresh32 = (*job).active_tooltip;
                    *fresh32 = strdup_and_subst_obj(agxget(e as *mut libc::c_void, a), obj);
                }
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn pointer2graph(mut job: *mut GVJ_t, mut pointer: pointf) -> pointf {
    let mut p: pointf = pointf { x: 0., y: 0. };
    if (*job).rotation != 0 {
        p.x = pointer.y / ((*job).zoom * (*job).devscale.y) - (*job).translation.x;
        p.y = -pointer.x / ((*job).zoom * (*job).devscale.x) - (*job).translation.y;
    } else {
        p.x = pointer.x / ((*job).zoom * (*job).devscale.x) - (*job).translation.x;
        p.y = pointer.y / ((*job).zoom * (*job).devscale.y) - (*job).translation.y;
    }
    return p;
}
unsafe extern "C" fn gvevent_find_current_obj(mut job: *mut GVJ_t, mut pointer: pointf) {
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut closeenough: libc::c_double = 0.;
    let mut p: pointf = pointf { x: 0., y: 0. };
    p = pointer2graph(job, pointer);
    closeenough = 1 as libc::c_int as libc::c_double / (*job).zoom;
    b.UR.x = p.x + closeenough;
    b.UR.y = p.y + closeenough;
    b.LL.x = p.x - closeenough;
    b.LL.y = p.y - closeenough;
    obj = gvevent_find_obj((*(*job).gvc).g, b);
    if obj != (*job).current_obj {
        gvevent_leave_obj(job);
        let ref mut fresh33 = (*job).current_obj;
        *fresh33 = obj;
        gvevent_enter_obj(job);
        (*job).needs_refresh = 1 as libc::c_int != 0;
    }
}
unsafe extern "C" fn gvevent_select_current_obj(mut job: *mut GVJ_t) {
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    obj = (*job).selected_obj;
    if !obj.is_null() {
        match agobjkind(obj) {
            0 => {
                let ref mut fresh34 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state;
                *fresh34 = (*fresh34 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
                    as libc::c_uchar;
                let ref mut fresh35 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state;
                *fresh35 = (*fresh35 as libc::c_int
                    & !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar as libc::c_int)
                    as libc::c_uchar;
            }
            1 => {
                let ref mut fresh36 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state;
                *fresh36 = (*fresh36 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
                    as libc::c_uchar;
                let ref mut fresh37 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state;
                *fresh37 = (*fresh37 as libc::c_int
                    & !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar as libc::c_int)
                    as libc::c_uchar;
            }
            2 => {
                let ref mut fresh38 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state;
                *fresh38 = (*fresh38 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
                    as libc::c_uchar;
                let ref mut fresh39 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state;
                *fresh39 = (*fresh39 as libc::c_int
                    & !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar as libc::c_int)
                    as libc::c_uchar;
            }
            _ => {}
        }
    }
    free((*job).selected_href as *mut libc::c_void);
    let ref mut fresh40 = (*job).selected_href;
    *fresh40 = 0 as *mut libc::c_char;
    let ref mut fresh41 = (*job).selected_obj;
    *fresh41 = (*job).current_obj;
    obj = *fresh41;
    if !obj.is_null() {
        match agobjkind(obj) {
            0 => {
                let ref mut fresh42 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state;
                *fresh42 = (*fresh42 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uchar;
                gv_graph_state(job, obj as *mut graph_t);
            }
            1 => {
                let ref mut fresh43 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).gui_state;
                *fresh43 = (*fresh43 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uchar;
                gv_node_state(job, obj as *mut node_t);
            }
            2 => {
                let ref mut fresh44 =
                    (*((*(obj as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state;
                *fresh44 = (*fresh44 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_uchar;
                gv_edge_state(job, obj as *mut edge_t);
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn gvevent_button_press(
    mut job: *mut GVJ_t,
    mut button: libc::c_int,
    mut pointer: pointf,
) {
    match button {
        1 => {
            gvevent_find_current_obj(job, pointer);
            gvevent_select_current_obj(job);
            (*job).click = 1 as libc::c_int != 0;
            (*job).button = button as libc::c_uchar;
            (*job).needs_refresh = 1 as libc::c_int != 0;
        }
        2 => {
            (*job).click = 1 as libc::c_int != 0;
            (*job).button = button as libc::c_uchar;
            (*job).needs_refresh = 1 as libc::c_int != 0;
        }
        3 => {
            gvevent_find_current_obj(job, pointer);
            (*job).click = 1 as libc::c_int != 0;
            (*job).button = button as libc::c_uchar;
            (*job).needs_refresh = 1 as libc::c_int != 0;
        }
        4 => {
            (*job).fit_mode = 0 as libc::c_int != 0;
            if (*job).rotation != 0 {
                (*job).focus.x -= (pointer.y - (*job).height as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.y);
                (*job).focus.y += (pointer.x - (*job).width as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.x);
            } else {
                (*job).focus.x += (pointer.x - (*job).width as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.x);
                (*job).focus.y += (pointer.y - (*job).height as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.y);
            }
            (*job).zoom *= 1.1f64;
            (*job).needs_refresh = 1 as libc::c_int != 0;
        }
        5 => {
            (*job).fit_mode = 0 as libc::c_int != 0;
            (*job).zoom /= 1.1f64;
            if (*job).rotation != 0 {
                (*job).focus.x += (pointer.y - (*job).height as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.y);
                (*job).focus.y -= (pointer.x - (*job).width as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.x);
            } else {
                (*job).focus.x -= (pointer.x - (*job).width as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.x);
                (*job).focus.y -= (pointer.y - (*job).height as libc::c_double / 2.0f64)
                    * (1.1f64 - 1.0f64)
                    / ((*job).zoom * (*job).devscale.y);
            }
            (*job).needs_refresh = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    (*job).oldpointer = pointer;
}
unsafe extern "C" fn gvevent_button_release(
    mut job: *mut GVJ_t,
    mut button: libc::c_int,
    mut pointer: pointf,
) {
    (*job).click = 0 as libc::c_int != 0;
    (*job).button = 0 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn gvevent_motion(mut job: *mut GVJ_t, mut pointer: pointf) {
    let mut dx: libc::c_double = (pointer.x - (*job).oldpointer.x) / (*job).devscale.x;
    let mut dy: libc::c_double = (pointer.y - (*job).oldpointer.y) / (*job).devscale.y;
    if fabs(dx) < 0.0001f64 && fabs(dy) < 0.0001f64 {
        return;
    }
    match (*job).button as libc::c_int {
        0 => {
            gvevent_find_current_obj(job, pointer);
        }
        1 => {}
        2 => {
            if (*job).rotation != 0 {
                (*job).focus.x -= dy / (*job).zoom;
                (*job).focus.y += dx / (*job).zoom;
            } else {
                (*job).focus.x -= dx / (*job).zoom;
                (*job).focus.y -= dy / (*job).zoom;
            }
            (*job).needs_refresh = 1 as libc::c_int != 0;
        }
        3 | _ => {}
    }
    (*job).oldpointer = pointer;
}
unsafe extern "C" fn quit_cb(mut job: *mut GVJ_t) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn left_cb(mut job: *mut GVJ_t) -> libc::c_int {
    (*job).fit_mode = 0 as libc::c_int != 0;
    (*job).focus.x += 10 as libc::c_int as libc::c_double / (*job).zoom;
    (*job).needs_refresh = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn right_cb(mut job: *mut GVJ_t) -> libc::c_int {
    (*job).fit_mode = 0 as libc::c_int != 0;
    (*job).focus.x -= 10 as libc::c_int as libc::c_double / (*job).zoom;
    (*job).needs_refresh = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn up_cb(mut job: *mut GVJ_t) -> libc::c_int {
    (*job).fit_mode = 0 as libc::c_int != 0;
    (*job).focus.y += -(10 as libc::c_int as libc::c_double / (*job).zoom);
    (*job).needs_refresh = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn down_cb(mut job: *mut GVJ_t) -> libc::c_int {
    (*job).fit_mode = 0 as libc::c_int != 0;
    (*job).focus.y -= -(10 as libc::c_int as libc::c_double / (*job).zoom);
    (*job).needs_refresh = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn zoom_in_cb(mut job: *mut GVJ_t) -> libc::c_int {
    (*job).fit_mode = 0 as libc::c_int != 0;
    (*job).zoom *= 1.1f64;
    (*job).needs_refresh = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn zoom_out_cb(mut job: *mut GVJ_t) -> libc::c_int {
    (*job).fit_mode = 0 as libc::c_int != 0;
    (*job).zoom /= 1.1f64;
    (*job).needs_refresh = 1 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn toggle_fit_cb(mut job: *mut GVJ_t) -> libc::c_int {
    (*job).fit_mode = !(*job).fit_mode;
    if (*job).fit_mode {
        let mut dflt_width: libc::c_int = 0;
        let mut dflt_height: libc::c_int = 0;
        dflt_width = (*job).width as libc::c_int;
        dflt_height = (*job).height as libc::c_int;
        (*job).zoom = if ((*job).width as libc::c_double / dflt_width as libc::c_double)
            < (*job).height as libc::c_double / dflt_height as libc::c_double
        {
            (*job).width as libc::c_double / dflt_width as libc::c_double
        } else {
            (*job).height as libc::c_double / dflt_height as libc::c_double
        };
        (*job).focus.x = 0.0f64;
        (*job).focus.y = 0.0f64;
        (*job).needs_refresh = 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gvevent_modify(
    mut job: *mut GVJ_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
}
unsafe extern "C" fn gvevent_delete(mut job: *mut GVJ_t) {}
unsafe extern "C" fn gvevent_read(
    mut job: *mut GVJ_t,
    mut filename: *const libc::c_char,
    mut layout: *const libc::c_char,
) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut gvc: *mut GVC_t = 0 as *mut GVC_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut gvle: *mut gvlayout_engine_t = 0 as *mut gvlayout_engine_t;
    gvc = (*job).gvc;
    if filename.is_null() {
        g = agread(stdin as *mut libc::c_void, 0 as *mut Agdisc_t);
    } else {
        f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            return;
        }
        g = agread(f as *mut libc::c_void, 0 as *mut Agdisc_t);
        fclose(f);
    }
    if g.is_null() {
        return;
    }
    if !((*gvc).g).is_null() {
        gvle = (*gvc).layout.engine;
        if !gvle.is_null() && ((*gvle).cleanup).is_some() {
            ((*gvle).cleanup).expect("non-null function pointer")((*gvc).g);
        }
        graph_cleanup((*gvc).g);
        agclose((*gvc).g);
    }
    aginit(
        g,
        0 as libc::c_int,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    aginit(
        g,
        1 as libc::c_int,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    aginit(
        g,
        2 as libc::c_int,
        b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    let ref mut fresh45 = (*gvc).g;
    *fresh45 = g;
    let ref mut fresh46 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc;
    *fresh46 = gvc;
    if gvLayout(gvc, g, layout) == -(1 as libc::c_int) {
        return;
    }
    let ref mut fresh47 = (*job).selected_obj;
    *fresh47 = 0 as *mut libc::c_void;
    let ref mut fresh48 = (*job).current_obj;
    *fresh48 = 0 as *mut libc::c_void;
    (*job).needs_refresh = 1 as libc::c_int != 0;
}
unsafe extern "C" fn gvevent_layout(mut job: *mut GVJ_t, mut layout: *const libc::c_char) {
    gvLayout((*job).gvc, (*(*job).gvc).g, layout);
}
unsafe extern "C" fn gvevent_render(
    mut job: *mut GVJ_t,
    mut format: *const libc::c_char,
    mut filename: *const libc::c_char,
) {
    let mut save_jobs: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut save_active: *mut GVJ_t = 0 as *mut GVJ_t;
    if !((*(*job).gvc).jobs).is_null() && ((*(*job).gvc).job).is_null() {
        save_jobs = (*(*job).gvc).jobs;
        save_active = (*(*job).gvc).active_jobs;
        let ref mut fresh49 = (*(*job).gvc).jobs;
        *fresh49 = 0 as *mut GVJ_t;
        let ref mut fresh50 = (*(*job).gvc).active_jobs;
        *fresh50 = *fresh49;
    } else {
        save_jobs = 0 as *mut GVJ_t;
    }
    gvRenderFilename((*job).gvc, (*(*job).gvc).g, format, filename);
    if !save_jobs.is_null() {
        let ref mut fresh51 = (*(*job).gvc).jobs;
        *fresh51 = save_jobs;
        let ref mut fresh52 = (*(*job).gvc).active_jobs;
        *fresh52 = save_active;
    }
}
#[no_mangle]
pub static mut gvevent_key_binding: [gvevent_key_binding_t; 14] = unsafe {
    [
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"Q\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(quit_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"Left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(left_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"KP_Left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(left_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"Right\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(right_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"KP_Right\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(right_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"Up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(up_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"KP_Up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(up_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"Down\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(down_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"KP_Down\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(down_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"plus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(zoom_in_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"KP_Add\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(zoom_in_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"minus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(zoom_out_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"KP_Subtract\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                callback: Some(zoom_out_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
        {
            let mut init = gvevent_key_binding_s {
                keystring: b"F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                callback: Some(toggle_fit_cb as unsafe extern "C" fn(*mut GVJ_t) -> libc::c_int),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut gvevent_key_binding_size: libc::c_int = 0;
#[no_mangle]
pub static mut gvdevice_callbacks: gvdevice_callbacks_t = unsafe {
    {
        let mut init = gvdevice_callbacks_s {
            refresh: Some(gvevent_refresh as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            button_press: Some(
                gvevent_button_press as unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> (),
            ),
            button_release: Some(
                gvevent_button_release
                    as unsafe extern "C" fn(*mut GVJ_t, libc::c_int, pointf) -> (),
            ),
            motion: Some(gvevent_motion as unsafe extern "C" fn(*mut GVJ_t, pointf) -> ()),
            modify: Some(
                gvevent_modify
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            ),
            del: Some(gvevent_delete as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            read: Some(
                gvevent_read
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            ),
            layout: Some(
                gvevent_layout as unsafe extern "C" fn(*mut GVJ_t, *const libc::c_char) -> (),
            ),
            render: Some(
                gvevent_render
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn run_static_initializers() {
    gvevent_key_binding_size = (::std::mem::size_of::<[gvevent_key_binding_t; 14]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<gvevent_key_binding_t>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
