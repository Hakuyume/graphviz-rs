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
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut Dtobag: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtflatten(_: *mut Dt_t) -> *mut Dtlink_t;
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
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    static mut Agstrictdirected: Agdesc_t;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn rank(g: *mut graph_t, balance: libc::c_int, maxiter: libc::c_int) -> libc::c_int;
    fn sepFactor(G: *mut graph_t) -> expand_t;
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
pub type adjust_mode = libc::c_uint;
pub const AM_PRISM: adjust_mode = 18;
pub const AM_IPSEP: adjust_mode = 17;
pub const AM_VPSC: adjust_mode = 16;
pub const AM_COMPRESS: adjust_mode = 15;
pub const AM_PORTHOYX: adjust_mode = 14;
pub const AM_PORTHOXY: adjust_mode = 13;
pub const AM_PORTHO_YX: adjust_mode = 12;
pub const AM_PORTHO: adjust_mode = 11;
pub const AM_ORTHOYX: adjust_mode = 10;
pub const AM_ORTHOXY: adjust_mode = 9;
pub const AM_ORTHO_YX: adjust_mode = 8;
pub const AM_ORTHO: adjust_mode = 7;
pub const AM_PUSHPULL: adjust_mode = 6;
pub const AM_PUSH: adjust_mode = 5;
pub const AM_SCALEXY: adjust_mode = 4;
pub const AM_NSCALE: adjust_mode = 3;
pub const AM_SCALE: adjust_mode = 2;
pub const AM_VOR: adjust_mode = 1;
pub const AM_NONE: adjust_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expand_t {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub doAdd: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nitem {
    pub link: Dtlink_t,
    pub val: libc::c_int,
    pub pos: point,
    pub np: *mut node_t,
    pub cnode: *mut node_t,
    pub vnode: *mut node_t,
    pub bb: box_0,
}
pub type intersectfn = Option::<
    unsafe extern "C" fn(*mut nitem, *mut nitem) -> libc::c_int,
>;
pub type distfn = Option::<unsafe extern "C" fn(*mut box_0, *mut box_0) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct info {
    pub pos: pointf,
    pub bb: boxf,
    pub wd2: libc::c_double,
    pub ht2: libc::c_double,
    pub np: *mut node_t,
}
pub type sortfn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
unsafe extern "C" fn cmpitem(
    mut d: *mut Dt_t,
    mut p1: *mut libc::c_int,
    mut p2: *mut libc::c_int,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    return *p1 - *p2;
}
static mut constr: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 16 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
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
                    cmpitem
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
unsafe extern "C" fn distY(mut b1: *mut box_0, mut b2: *mut box_0) -> libc::c_int {
    return ((*b1).UR.y - (*b1).LL.y + ((*b2).UR.y - (*b2).LL.y)) / 2 as libc::c_int;
}
unsafe extern "C" fn distX(mut b1: *mut box_0, mut b2: *mut box_0) -> libc::c_int {
    return ((*b1).UR.x - (*b1).LL.x + ((*b2).UR.x - (*b2).LL.x)) / 2 as libc::c_int;
}
unsafe extern "C" fn intersectX0(mut p: *mut nitem, mut q: *mut nitem) -> libc::c_int {
    let mut xdelta: libc::c_int = 0;
    let mut ydelta: libc::c_int = 0;
    let mut v: libc::c_int = ((*p).bb.LL.x <= (*q).bb.UR.x
        && (*q).bb.LL.x <= (*p).bb.UR.x) as libc::c_int;
    if v == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*p).bb.UR.y < (*q).bb.LL.y {
        return 1 as libc::c_int;
    }
    ydelta = distY(&mut (*p).bb, &mut (*q).bb) - ((*q).pos.y - (*p).pos.y);
    if (*q).pos.x >= (*p).pos.x {
        xdelta = distX(&mut (*p).bb, &mut (*q).bb) - ((*q).pos.x - (*p).pos.x);
    } else {
        xdelta = distX(&mut (*p).bb, &mut (*q).bb) - ((*p).pos.x - (*q).pos.x);
    }
    return (ydelta <= xdelta) as libc::c_int;
}
unsafe extern "C" fn intersectY0(mut p: *mut nitem, mut q: *mut nitem) -> libc::c_int {
    let mut xdelta: libc::c_int = 0;
    let mut ydelta: libc::c_int = 0;
    let mut v: libc::c_int = ((*p).bb.LL.y <= (*q).bb.UR.y
        && (*q).bb.LL.y <= (*p).bb.UR.y) as libc::c_int;
    if v == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*p).bb.UR.x < (*q).bb.LL.x {
        return 1 as libc::c_int;
    }
    xdelta = distX(&mut (*p).bb, &mut (*q).bb) - ((*q).pos.x - (*p).pos.x);
    if (*q).pos.y >= (*p).pos.y {
        ydelta = distY(&mut (*p).bb, &mut (*q).bb) - ((*q).pos.y - (*p).pos.y);
    } else {
        ydelta = distY(&mut (*p).bb, &mut (*q).bb) - ((*p).pos.y - (*q).pos.y);
    }
    return (xdelta <= ydelta) as libc::c_int;
}
unsafe extern "C" fn intersectY(mut p: *mut nitem, mut q: *mut nitem) -> libc::c_int {
    return ((*p).bb.LL.y <= (*q).bb.UR.y && (*q).bb.LL.y <= (*p).bb.UR.y) as libc::c_int;
}
unsafe extern "C" fn intersectX(mut p: *mut nitem, mut q: *mut nitem) -> libc::c_int {
    return ((*p).bb.LL.x <= (*q).bb.UR.x && (*q).bb.LL.x <= (*p).bb.UR.x) as libc::c_int;
}
unsafe extern "C" fn mapGraphs(
    mut g: *mut graph_t,
    mut cg: *mut graph_t,
    mut dist: distfn,
) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut ce: *mut edge_t = 0 as *mut edge_t;
    let mut t: *mut node_t = 0 as *mut node_t;
    let mut h: *mut node_t = 0 as *mut node_t;
    let mut tp: *mut nitem = 0 as *mut nitem;
    let mut hp: *mut nitem = 0 as *mut nitem;
    let mut delta: libc::c_int = 0;
    n = agfstnode(g);
    while !n.is_null() {
        tp = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg as *mut nitem;
        t = (*tp).cnode;
        e = agfstout(g, n);
        while !e.is_null() {
            hp = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .alg as *mut nitem;
            delta = dist
                .expect("non-null function pointer")(&mut (*tp).bb, &mut (*hp).bb);
            h = (*hp).cnode;
            ce = agedge(cg, t, h, 0 as *mut libc::c_char, 1 as libc::c_int);
            agbindrec(
                ce as *mut libc::c_void,
                b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .weight = 1 as libc::c_int;
            if ((*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                as libc::c_int) < delta
            {
                if (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t)).minlen
                    as libc::c_int as libc::c_double == 0.0f64
                {
                    let ref mut fresh0 = (*((*(t as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .out
                        .list;
                    *fresh0 = if !((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .out
                        .list)
                        .is_null()
                    {
                        grealloc(
                            (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .out
                                .list as *mut libc::c_void,
                            (((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .out
                                .size + 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                ),
                        ) as *mut *mut edge_t
                    } else {
                        gmalloc(
                            (((*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .out
                                .size + 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                ),
                        ) as *mut *mut edge_t
                    };
                    let ref mut fresh1 = (*((*(t as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .out
                        .size;
                    let fresh2 = *fresh1;
                    *fresh1 = *fresh1 + 1;
                    let ref mut fresh3 = *((*((*(t as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .out
                        .list)
                        .offset(fresh2 as isize);
                    *fresh3 = ce;
                    let ref mut fresh4 = *((*((*(t as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .out
                        .list)
                        .offset(
                            (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .out
                                .size as isize,
                        );
                    *fresh4 = 0 as *mut edge_t;
                    let ref mut fresh5 = (*((*(h as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .in_0
                        .list;
                    *fresh5 = if !((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .in_0
                        .list)
                        .is_null()
                    {
                        grealloc(
                            (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .in_0
                                .list as *mut libc::c_void,
                            (((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .in_0
                                .size + 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                ),
                        ) as *mut *mut edge_t
                    } else {
                        gmalloc(
                            (((*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .in_0
                                .size + 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                ),
                        ) as *mut *mut edge_t
                    };
                    let ref mut fresh6 = (*((*(h as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .in_0
                        .size;
                    let fresh7 = *fresh6;
                    *fresh6 = *fresh6 + 1;
                    let ref mut fresh8 = *((*((*(h as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .in_0
                        .list)
                        .offset(fresh7 as isize);
                    *fresh8 = ce;
                    let ref mut fresh9 = *((*((*(h as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .in_0
                        .list)
                        .offset(
                            (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .in_0
                                .size as isize,
                        );
                    *fresh9 = 0 as *mut edge_t;
                }
                (*((*(ce as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .minlen = delta as libc::c_ushort;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn mkNConstraintG(
    mut g: *mut graph_t,
    mut list: *mut Dt_t,
    mut intersect: intersectfn,
    mut dist: distfn,
) -> *mut graph_t {
    let mut p: *mut nitem = 0 as *mut nitem;
    let mut nxp: *mut nitem = 0 as *mut nitem;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut lastn: *mut node_t = 0 as *mut node_t;
    let mut cg: *mut graph_t = agopen(
        b"cg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agstrictdirected,
        0 as *mut Agdisc_t,
    );
    agbindrec(
        cg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    p = dtflatten(list) as *mut nitem;
    while !p.is_null() {
        n = agnode(cg, agnameof((*p).np as *mut libc::c_void), 1 as libc::c_int);
        agbindrec(
            n as *mut libc::c_void,
            b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
            1 as libc::c_int,
        );
        let ref mut fresh10 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg;
        *fresh10 = p as *mut libc::c_void;
        let ref mut fresh11 = (*p).cnode;
        *fresh11 = n;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .size = 0 as libc::c_int;
        let ref mut fresh12 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .in_0
            .list;
        *fresh12 = gcalloc(
            (0 as libc::c_int + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .size = 0 as libc::c_int;
        let ref mut fresh13 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .out
            .list;
        *fresh13 = gcalloc(
            (0 as libc::c_int + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        if !lastn.is_null() {
            let ref mut fresh14 = (*((*(lastn as *mut Agobj_t)).data
                as *mut Agnodeinfo_t))
                .next;
            *fresh14 = n;
            lastn = n;
        } else {
            let ref mut fresh15 = (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .nlist;
            *fresh15 = n;
            lastn = *fresh15;
        }
        p = (*(p as *mut Dtlink_t)).right as *mut nitem;
    }
    p = dtflatten(list) as *mut nitem;
    while !p.is_null() {
        nxp = (*(p as *mut Dtlink_t)).right as *mut nitem;
        while !nxp.is_null() {
            e = 0 as *mut edge_t;
            if intersect.expect("non-null function pointer")(p, nxp) != 0 {
                let mut delta: libc::c_double = dist
                    .expect("non-null function pointer")(&mut (*p).bb, &mut (*nxp).bb)
                    as libc::c_double;
                e = agedge(
                    cg,
                    (*p).cnode,
                    (*nxp).cnode,
                    0 as *mut libc::c_char,
                    1 as libc::c_int,
                );
                agbindrec(
                    e as *mut libc::c_void,
                    b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong
                        as libc::c_uint,
                    1 as libc::c_int,
                );
                if delta <= 0xffff as libc::c_int as libc::c_double {} else {
                    __assert_fail(
                        b"delta <= 0xFFFF\0" as *const u8 as *const libc::c_char,
                        b"constraint.c\0" as *const u8 as *const libc::c_char,
                        240 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 64],
                            &[libc::c_char; 64],
                        >(
                            b"graph_t *mkNConstraintG(graph_t *, Dt_t *, intersectfn, distfn)\0",
                        ))
                            .as_ptr(),
                    );
                }
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .minlen = delta as libc::c_ushort;
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .weight = 1 as libc::c_int;
            }
            if !e.is_null()
                && !(agedge(
                    g,
                    (*p).np,
                    (*nxp).np,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                ))
                    .is_null()
            {
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .weight = 100 as libc::c_int;
            }
            nxp = (*(nxp as *mut Dtlink_t)).right as *mut nitem;
        }
        p = (*(p as *mut Dtlink_t)).right as *mut nitem;
    }
    p = dtflatten(list) as *mut nitem;
    while !p.is_null() {
        n = (*p).cnode;
        e = agfstout(cg, n);
        while !e.is_null() {
            let ref mut fresh16 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list;
            *fresh16 = if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
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
            let ref mut fresh17 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .size;
            let fresh18 = *fresh17;
            *fresh17 = *fresh17 + 1;
            let ref mut fresh19 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .offset(fresh18 as isize);
            *fresh19 = e;
            let ref mut fresh20 = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .out
                .list)
                .offset(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                        as isize,
                );
            *fresh20 = 0 as *mut edge_t;
            let ref mut fresh21 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
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
            *fresh21 = if !((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
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
            let ref mut fresh22 = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
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
            let fresh23 = *fresh22;
            *fresh22 = *fresh22 + 1;
            let ref mut fresh24 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
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
                .offset(fresh23 as isize);
            *fresh24 = e;
            let ref mut fresh25 = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype()
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
            *fresh25 = 0 as *mut edge_t;
            e = agnxtout(cg, e);
        }
        p = (*(p as *mut Dtlink_t)).right as *mut nitem;
    }
    return cg;
}
unsafe extern "C" fn mkConstraintG(
    mut g: *mut graph_t,
    mut list: *mut Dt_t,
    mut intersect: intersectfn,
    mut dist: distfn,
) -> *mut graph_t {
    let mut p: *mut nitem = 0 as *mut nitem;
    let mut nxt: *mut nitem = 0 as *mut nitem;
    let mut nxp: *mut nitem = 0 as *mut nitem;
    let mut vg: *mut graph_t = 0 as *mut graph_t;
    let mut prev: *mut node_t = 0 as *mut node_t;
    let mut root: *mut node_t = 0 as *mut node_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut lcnt: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut oldval: libc::c_int = -(2147483647 as libc::c_int);
    let mut lastn: *mut node_t = 0 as *mut node_t;
    let mut cg: *mut graph_t = agopen(
        b"cg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agstrictdirected,
        0 as *mut Agdisc_t,
    );
    agbindrec(
        cg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    cnt = 0 as libc::c_int;
    p = dtflatten(list) as *mut nitem;
    while !p.is_null() {
        if oldval != (*p).val {
            oldval = (*p).val;
            cnt += 1;
        }
        p = (*(p as *mut Dtlink_t)).right as *mut nitem;
    }
    oldval = -(2147483647 as libc::c_int);
    lcnt = 0 as libc::c_int;
    p = dtflatten(list) as *mut nitem;
    while !p.is_null() {
        if oldval != (*p).val {
            oldval = (*p).val;
            n = agnode(cg, agnameof((*p).np as *mut libc::c_void), 1 as libc::c_int);
            agbindrec(
                n as *mut libc::c_void,
                b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            let ref mut fresh26 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .alg;
            *fresh26 = p as *mut libc::c_void;
            if !root.is_null() {
                let ref mut fresh27 = (*((*(lastn as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .next;
                *fresh27 = n;
                lastn = n;
            } else {
                root = n;
                let ref mut fresh28 = (*((*(cg as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .nlist;
                *fresh28 = n;
                lastn = *fresh28;
            }
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .size = 0 as libc::c_int;
            let ref mut fresh29 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .in_0
                .list;
            *fresh29 = gcalloc(
                (lcnt + 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
            ) as *mut *mut edge_t;
            if !prev.is_null() {
                if prev == root {
                    (*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .out
                        .size = 0 as libc::c_int;
                    let ref mut fresh30 = (*((*(prev as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .out
                        .list;
                    *fresh30 = gcalloc(
                        (2 as libc::c_int * (cnt - 1 as libc::c_int) + 1 as libc::c_int)
                            as size_t,
                        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                    ) as *mut *mut edge_t;
                } else {
                    (*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .out
                        .size = 0 as libc::c_int;
                    let ref mut fresh31 = (*((*(prev as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .out
                        .list;
                    *fresh31 = gcalloc(
                        (cnt - lcnt - 1 as libc::c_int + 1 as libc::c_int) as size_t,
                        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                    ) as *mut *mut edge_t;
                }
                e = agedge(cg, prev, n, 0 as *mut libc::c_char, 1 as libc::c_int);
                agbindrec(
                    e as *mut libc::c_void,
                    b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong
                        as libc::c_uint,
                    1 as libc::c_int,
                );
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .minlen = 10 as libc::c_int as libc::c_ushort;
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .weight = 1 as libc::c_int;
                let ref mut fresh32 = (*((*(prev as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .out
                    .list;
                *fresh32 = if !((*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .is_null()
                {
                    grealloc(
                        (*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list
                            as *mut libc::c_void,
                        (((*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .out
                            .size + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                } else {
                    gmalloc(
                        (((*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .out
                            .size + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                };
                let ref mut fresh33 = (*((*(prev as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .out
                    .size;
                let fresh34 = *fresh33;
                *fresh33 = *fresh33 + 1;
                let ref mut fresh35 = *((*((*(prev as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .offset(fresh34 as isize);
                *fresh35 = e;
                let ref mut fresh36 = *((*((*(prev as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .out
                    .list)
                    .offset(
                        (*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                            as isize,
                    );
                *fresh36 = 0 as *mut edge_t;
                let ref mut fresh37 = (*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .in_0
                    .list;
                *fresh37 = if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .in_0
                    .list)
                    .is_null()
                {
                    grealloc(
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list
                            as *mut libc::c_void,
                        (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                            + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                } else {
                    gmalloc(
                        (((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                            + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                            ),
                    ) as *mut *mut edge_t
                };
                let ref mut fresh38 = (*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .in_0
                    .size;
                let fresh39 = *fresh38;
                *fresh38 = *fresh38 + 1;
                let ref mut fresh40 = *((*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .in_0
                    .list)
                    .offset(fresh39 as isize);
                *fresh40 = e;
                let ref mut fresh41 = *((*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .in_0
                    .list)
                    .offset(
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                            as isize,
                    );
                *fresh41 = 0 as *mut edge_t;
            }
            lcnt += 1;
            prev = n;
        }
        let ref mut fresh42 = (*p).cnode;
        *fresh42 = n;
        p = (*(p as *mut Dtlink_t)).right as *mut nitem;
    }
    (*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size = 0 as libc::c_int;
    let ref mut fresh43 = (*((*(prev as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .out
        .list;
    *fresh43 = gcalloc(
        (0 as libc::c_int + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
    ) as *mut *mut edge_t;
    vg = agopen(
        b"vg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agstrictdirected,
        0 as *mut Agdisc_t,
    );
    p = dtflatten(list) as *mut nitem;
    while !p.is_null() {
        n = agnode(vg, agnameof((*p).np as *mut libc::c_void), 1 as libc::c_int);
        agbindrec(
            n as *mut libc::c_void,
            b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
            1 as libc::c_int,
        );
        let ref mut fresh44 = (*p).vnode;
        *fresh44 = n;
        let ref mut fresh45 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg;
        *fresh45 = p as *mut libc::c_void;
        p = (*(p as *mut Dtlink_t)).right as *mut nitem;
    }
    oldval = -(2147483647 as libc::c_int);
    p = dtflatten(list) as *mut nitem;
    while !p.is_null() {
        if oldval != (*p).val {
            oldval = (*p).val;
            nxt = (*(p as *mut Dtlink_t)).right as *mut nitem;
            while !nxt.is_null() {
                if (*nxt).val != oldval {
                    break;
                }
                nxt = (*(nxt as *mut Dtlink_t)).right as *mut nitem;
            }
            if nxt.is_null() {
                break;
            }
        }
        nxp = nxt;
        while !nxp.is_null() {
            if intersect.expect("non-null function pointer")(p, nxp) != 0 {
                agedge(
                    vg,
                    (*p).vnode,
                    (*nxp).vnode,
                    0 as *mut libc::c_char,
                    1 as libc::c_int,
                );
            }
            nxp = (*(nxp as *mut Dtlink_t)).right as *mut nitem;
        }
        p = (*(p as *mut Dtlink_t)).right as *mut nitem;
    }
    mapGraphs(vg, cg, dist);
    agclose(vg);
    return cg;
}
unsafe extern "C" fn closeGraph(mut cg: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    n = agfstnode(cg);
    while !n.is_null() {
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list
                as *mut libc::c_void,
        );
        free(
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list
                as *mut libc::c_void,
        );
        n = agnxtnode(cg, n);
    }
    agclose(cg);
}
unsafe extern "C" fn constrainX(
    mut g: *mut graph_t,
    mut nlist: *mut nitem,
    mut nnodes: libc::c_int,
    mut ifn: intersectfn,
    mut ortho: libc::c_int,
) {
    let mut list: *mut Dt_t = dtopen(&mut constr, Dtobag);
    let mut p: *mut nitem = nlist;
    let mut cg: *mut graph_t = 0 as *mut graph_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nnodes {
        (*p).val = (*p).pos.x;
        (Some(((*list).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(list, p as *mut libc::c_void, 0o1 as libc::c_int);
        p = p.offset(1);
        i += 1;
    }
    if ortho != 0 {
        cg = mkConstraintG(
            g,
            list,
            ifn,
            Some(distX as unsafe extern "C" fn(*mut box_0, *mut box_0) -> libc::c_int),
        );
    } else {
        cg = mkNConstraintG(
            g,
            list,
            ifn,
            Some(distX as unsafe extern "C" fn(*mut box_0, *mut box_0) -> libc::c_int),
        );
    }
    rank(cg, 2 as libc::c_int, 2147483647 as libc::c_int);
    p = nlist;
    i = 0 as libc::c_int;
    while i < nnodes {
        let mut newpos: libc::c_int = 0;
        let mut oldpos: libc::c_int = 0;
        let mut delta: libc::c_int = 0;
        oldpos = (*p).pos.x;
        newpos = (*((*((*p).cnode as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        delta = newpos - oldpos;
        (*p).pos.x = newpos;
        (*p).bb.LL.x += delta;
        (*p).bb.UR.x += delta;
        p = p.offset(1);
        i += 1;
    }
    closeGraph(cg);
    dtclose(list);
}
unsafe extern "C" fn constrainY(
    mut g: *mut graph_t,
    mut nlist: *mut nitem,
    mut nnodes: libc::c_int,
    mut ifn: intersectfn,
    mut ortho: libc::c_int,
) {
    let mut list: *mut Dt_t = dtopen(&mut constr, Dtobag);
    let mut p: *mut nitem = nlist;
    let mut cg: *mut graph_t = 0 as *mut graph_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nnodes {
        (*p).val = (*p).pos.y;
        (Some(((*list).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(list, p as *mut libc::c_void, 0o1 as libc::c_int);
        p = p.offset(1);
        i += 1;
    }
    if ortho != 0 {
        cg = mkConstraintG(
            g,
            list,
            ifn,
            Some(distY as unsafe extern "C" fn(*mut box_0, *mut box_0) -> libc::c_int),
        );
    } else {
        cg = mkNConstraintG(
            g,
            list,
            ifn,
            Some(distY as unsafe extern "C" fn(*mut box_0, *mut box_0) -> libc::c_int),
        );
    }
    rank(cg, 2 as libc::c_int, 2147483647 as libc::c_int);
    p = nlist;
    i = 0 as libc::c_int;
    while i < nnodes {
        let mut newpos: libc::c_int = 0;
        let mut oldpos: libc::c_int = 0;
        let mut delta: libc::c_int = 0;
        oldpos = (*p).pos.y;
        newpos = (*((*((*p).cnode as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
        delta = newpos - oldpos;
        (*p).pos.y = newpos;
        (*p).bb.LL.y += delta;
        (*p).bb.UR.y += delta;
        p = p.offset(1);
        i += 1;
    }
    closeGraph(cg);
    dtclose(list);
}
unsafe extern "C" fn overlaps(mut p: *mut nitem, mut cnt: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pi: *mut nitem = p;
    let mut pj: *mut nitem = 0 as *mut nitem;
    i = 0 as libc::c_int;
    while i < cnt - 1 as libc::c_int {
        pj = pi.offset(1 as libc::c_int as isize);
        j = i + 1 as libc::c_int;
        while j < cnt {
            if (*pi).bb.LL.x <= (*pj).bb.UR.x && (*pj).bb.LL.x <= (*pi).bb.UR.x
                && (*pi).bb.LL.y <= (*pj).bb.UR.y && (*pj).bb.LL.y <= (*pi).bb.UR.y
            {
                return 1 as libc::c_int;
            }
            pj = pj.offset(1);
            j += 1;
        }
        pi = pi.offset(1);
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn initItem(
    mut n: *mut node_t,
    mut p: *mut nitem,
    mut margin: expand_t,
) {
    let mut x: libc::c_int = if 10 as libc::c_int as libc::c_double
        * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) * 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        (10 as libc::c_int as libc::c_double
            * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) * 72 as libc::c_int as libc::c_double
            + 0.5f64) as libc::c_int
    } else {
        (10 as libc::c_int as libc::c_double
            * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) * 72 as libc::c_int as libc::c_double
            - 0.5f64) as libc::c_int
    };
    let mut y: libc::c_int = if 10 as libc::c_int as libc::c_double
        * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) * 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        (10 as libc::c_int as libc::c_double
            * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) * 72 as libc::c_int as libc::c_double
            + 0.5f64) as libc::c_int
    } else {
        (10 as libc::c_int as libc::c_double
            * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) * 72 as libc::c_int as libc::c_double
            - 0.5f64) as libc::c_int
    };
    let mut w2: libc::c_int = 0;
    let mut h2: libc::c_int = 0;
    let mut b: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    if margin.doAdd {
        w2 = (10 as libc::c_int as libc::c_float
            * ((if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64
                    * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64
                    * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            }) as libc::c_float + margin.x)) as libc::c_int;
        h2 = (10 as libc::c_int as libc::c_float
            * ((if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64
                    * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64
                    * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            }) as libc::c_float + margin.y)) as libc::c_int;
    } else {
        w2 = if (margin.x * (10 as libc::c_int / 2 as libc::c_int) as libc::c_float)
            as libc::c_double
            * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
            * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
        {
            ((margin.x * (10 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double
                * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            ((margin.x * (10 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double
                * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        };
        h2 = if (margin.y * (10 as libc::c_int / 2 as libc::c_int) as libc::c_float)
            as libc::c_double
            * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
            * 72 as libc::c_int as libc::c_double >= 0 as libc::c_int as libc::c_double
        {
            ((margin.y * (10 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double
                * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
        } else {
            ((margin.y * (10 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double
                * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
        };
    }
    b.LL.x = x - w2;
    b.LL.y = y - h2;
    b.UR.x = x + w2;
    b.UR.y = y + h2;
    (*p).pos.x = x;
    (*p).pos.y = y;
    let ref mut fresh46 = (*p).np;
    *fresh46 = n;
    (*p).bb = b;
}
#[no_mangle]
pub unsafe extern "C" fn cAdjust(
    mut g: *mut graph_t,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut margin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nnodes: libc::c_int = agnnodes(g);
    let mut nlist: *mut nitem = gcalloc(
        nnodes as size_t,
        ::std::mem::size_of::<nitem>() as libc::c_ulong,
    ) as *mut nitem;
    let mut p: *mut nitem = nlist;
    let mut n: *mut node_t = 0 as *mut node_t;
    margin = sepFactor(g);
    n = agfstnode(g);
    while !n.is_null() {
        initItem(n, p, margin);
        p = p.offset(1);
        n = agnxtnode(g, n);
    }
    if overlaps(nlist, nnodes) != 0 {
        let mut pt: point = point { x: 0, y: 0 };
        let mut current_block_21: u64;
        match mode as adjust_mode as libc::c_uint {
            9 => {
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                current_block_21 = 7205609094909031804;
            }
            10 => {
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                current_block_21 = 7205609094909031804;
            }
            7 => {
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY0
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                current_block_21 = 18333774006653555424;
            }
            8 => {
                current_block_21 = 18333774006653555424;
            }
            13 => {
                current_block_21 = 15287670148031779809;
            }
            14 => {
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
                current_block_21 = 7205609094909031804;
            }
            12 => {
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX0
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
                current_block_21 = 7205609094909031804;
            }
            11 | _ => {
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY0
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
                current_block_21 = 7205609094909031804;
            }
        }
        match current_block_21 {
            18333774006653555424 => {
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX0
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    1 as libc::c_int,
                );
                current_block_21 = 15287670148031779809;
            }
            _ => {}
        }
        match current_block_21 {
            15287670148031779809 => {
                constrainX(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectY
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
                constrainY(
                    g,
                    nlist,
                    nnodes,
                    Some(
                        intersectX
                            as unsafe extern "C" fn(
                                *mut nitem,
                                *mut nitem,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int,
                );
            }
            _ => {}
        }
        p = nlist;
        i = 0 as libc::c_int;
        while i < nnodes {
            n = (*p).np;
            pt = (*p).pos;
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(
                    0 as libc::c_int as isize,
                ) = pt.x as libc::c_double / 72 as libc::c_int as libc::c_double
                / 10 as libc::c_int as libc::c_double;
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(
                    1 as libc::c_int as isize,
                ) = pt.y as libc::c_double / 72 as libc::c_int as libc::c_double
                / 10 as libc::c_int as libc::c_double;
            p = p.offset(1);
            i += 1;
        }
        ret = 1 as libc::c_int;
    } else {
        ret = 0 as libc::c_int;
    }
    free(nlist as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn sortf(mut p: *mut pointf, mut q: *mut pointf) -> libc::c_int {
    if (*p).x < (*q).x {
        return -(1 as libc::c_int)
    } else if (*p).x > (*q).x {
        return 1 as libc::c_int
    } else if (*p).y < (*q).y {
        return -(1 as libc::c_int)
    } else if (*p).y > (*q).y {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn compress(mut nl: *mut info, mut nn: libc::c_int) -> libc::c_double {
    let mut p: *mut info = nl;
    let mut q: *mut info = 0 as *mut info;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    let mut sc: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut pt: pointf = pointf { x: 0., y: 0. };
    i = 0 as libc::c_int;
    while i < nn {
        q = p.offset(1 as libc::c_int as isize);
        j = i + 1 as libc::c_int;
        while j < nn {
            if (*p).bb.LL.x <= (*q).bb.UR.x && (*q).bb.LL.x <= (*p).bb.UR.x
                && (*p).bb.LL.y <= (*q).bb.UR.y && (*q).bb.LL.y <= (*p).bb.UR.y
            {
                return 0 as libc::c_int as libc::c_double;
            }
            if (*p).pos.x == (*q).pos.x {
                pt.x = ::std::f64::INFINITY;
            } else {
                pt.x = ((*p).wd2 + (*q).wd2) / fabs((*p).pos.x - (*q).pos.x);
            }
            if (*p).pos.y == (*q).pos.y {
                pt.y = ::std::f64::INFINITY;
            } else {
                pt.y = ((*p).ht2 + (*q).ht2) / fabs((*p).pos.y - (*q).pos.y);
            }
            if pt.y < pt.x {
                s = pt.y;
            } else {
                s = pt.x;
            }
            if s > sc {
                sc = s;
            }
            q = q.offset(1);
            j += 1;
        }
        p = p.offset(1);
        i += 1;
    }
    return sc;
}
unsafe extern "C" fn mkOverlapSet(
    mut nl: *mut info,
    mut nn: libc::c_int,
    mut cntp: *mut libc::c_int,
) -> *mut pointf {
    let mut p: *mut info = nl;
    let mut q: *mut info = 0 as *mut info;
    let mut sz: libc::c_int = nn;
    let mut S: *mut pointf = gcalloc(
        (sz + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nn {
        q = p.offset(1 as libc::c_int as isize);
        j = i + 1 as libc::c_int;
        while j < nn {
            if (*p).bb.LL.x <= (*q).bb.UR.x && (*q).bb.LL.x <= (*p).bb.UR.x
                && (*p).bb.LL.y <= (*q).bb.UR.y && (*q).bb.LL.y <= (*p).bb.UR.y
            {
                let mut pt: pointf = pointf { x: 0., y: 0. };
                if cnt == sz {
                    sz += nn;
                    S = grealloc(
                        S as *mut libc::c_void,
                        ((sz + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<pointf>() as libc::c_ulong,
                            ),
                    ) as *mut pointf;
                }
                if (*p).pos.x == (*q).pos.x {
                    pt.x = ::std::f64::INFINITY;
                } else {
                    pt.x = ((*p).wd2 + (*q).wd2) / fabs((*p).pos.x - (*q).pos.x);
                    if pt.x < 1 as libc::c_int as libc::c_double {
                        pt.x = 1 as libc::c_int as libc::c_double;
                    }
                }
                if (*p).pos.y == (*q).pos.y {
                    pt.y = ::std::f64::INFINITY;
                } else {
                    pt.y = ((*p).ht2 + (*q).ht2) / fabs((*p).pos.y - (*q).pos.y);
                    if pt.y < 1 as libc::c_int as libc::c_double {
                        pt.y = 1 as libc::c_int as libc::c_double;
                    }
                }
                cnt += 1;
                *S.offset(cnt as isize) = pt;
            }
            q = q.offset(1);
            j += 1;
        }
        p = p.offset(1);
        i += 1;
    }
    S = grealloc(
        S as *mut libc::c_void,
        ((cnt + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
    ) as *mut pointf;
    *cntp = cnt;
    return S;
}
unsafe extern "C" fn computeScaleXY(
    mut aarr: *mut pointf,
    mut m: libc::c_int,
) -> pointf {
    let mut barr: *mut pointf = 0 as *mut pointf;
    let mut cost: libc::c_double = 0.;
    let mut bestcost: libc::c_double = 0.;
    let mut k: libc::c_int = 0;
    let mut best: libc::c_int = 0 as libc::c_int;
    let mut scale: pointf = pointf { x: 0., y: 0. };
    (*aarr.offset(0 as libc::c_int as isize)).x = 1 as libc::c_int as libc::c_double;
    (*aarr.offset(0 as libc::c_int as isize)).y = ::std::f64::INFINITY;
    qsort(
        aarr.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        m as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut pointf, *mut pointf) -> libc::c_int>,
            sortfn_t,
        >(Some(sortf as unsafe extern "C" fn(*mut pointf, *mut pointf) -> libc::c_int)),
    );
    barr = gcalloc(
        (m + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    (*barr.offset(m as isize)).x = (*aarr.offset(m as isize)).x;
    (*barr.offset(m as isize)).y = 1 as libc::c_int as libc::c_double;
    k = m - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        (*barr.offset(k as isize)).x = (*aarr.offset(k as isize)).x;
        (*barr.offset(k as isize))
            .y = if (*aarr.offset((k + 1 as libc::c_int) as isize)).y
            > (*barr.offset((k + 1 as libc::c_int) as isize)).y
        {
            (*aarr.offset((k + 1 as libc::c_int) as isize)).y
        } else {
            (*barr.offset((k + 1 as libc::c_int) as isize)).y
        };
        k -= 1;
    }
    bestcost = ::std::f64::INFINITY;
    k = 0 as libc::c_int;
    while k <= m {
        cost = (*barr.offset(k as isize)).x * (*barr.offset(k as isize)).y;
        if cost < bestcost {
            bestcost = cost;
            best = k;
        }
        k += 1;
    }
    if bestcost < ::std::f64::INFINITY {} else {
        __assert_fail(
            b"bestcost < HUGE_VAL\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            758 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"pointf computeScaleXY(pointf *, int)\0"))
                .as_ptr(),
        );
    }
    scale.x = (*barr.offset(best as isize)).x;
    scale.y = (*barr.offset(best as isize)).y;
    return scale;
}
unsafe extern "C" fn computeScale(
    mut aarr: *mut pointf,
    mut m: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut sc: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut v: libc::c_double = 0.;
    let mut p: pointf = pointf { x: 0., y: 0. };
    aarr = aarr.offset(1);
    i = 1 as libc::c_int;
    while i <= m {
        let fresh47 = aarr;
        aarr = aarr.offset(1);
        p = *fresh47;
        v = if p.x < p.y { p.x } else { p.y };
        if v > sc {
            sc = v;
        }
        i += 1;
    }
    return sc;
}
#[no_mangle]
pub unsafe extern "C" fn scAdjust(
    mut g: *mut graph_t,
    mut equal: libc::c_int,
) -> libc::c_int {
    let mut nnodes: libc::c_int = agnnodes(g);
    let mut nlist: *mut info = gcalloc(
        nnodes as size_t,
        ::std::mem::size_of::<info>() as libc::c_ulong,
    ) as *mut info;
    let mut p: *mut info = nlist;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut s: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut margin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    let mut aarr: *mut pointf = 0 as *mut pointf;
    let mut m: libc::c_int = 0;
    margin = sepFactor(g);
    if margin.doAdd {
        margin
            .x = (margin.x as libc::c_double / 72 as libc::c_int as libc::c_double)
            as libc::c_float;
        margin
            .y = (margin.y as libc::c_double / 72 as libc::c_int as libc::c_double)
            as libc::c_float;
    }
    n = agfstnode(g);
    while !n.is_null() {
        let mut w2: libc::c_double = 0.;
        let mut h2: libc::c_double = 0.;
        if margin.doAdd {
            w2 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64
                + margin.x as libc::c_double;
            h2 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64
                + margin.y as libc::c_double;
        } else {
            w2 = margin.x as libc::c_double
                * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width / 2.0f64;
            h2 = margin.y as libc::c_double
                * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height / 2.0f64;
        }
        (*p)
            .pos
            .x = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize);
        (*p)
            .pos
            .y = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize);
        (*p).bb.LL.x = (*p).pos.x - w2;
        (*p).bb.LL.y = (*p).pos.y - h2;
        (*p).bb.UR.x = (*p).pos.x + w2;
        (*p).bb.UR.y = (*p).pos.y + h2;
        (*p).wd2 = w2;
        (*p).ht2 = h2;
        let ref mut fresh48 = (*p).np;
        *fresh48 = n;
        p = p.offset(1);
        n = agnxtnode(g, n);
    }
    if equal < 0 as libc::c_int {
        s.y = compress(nlist, nnodes);
        s.x = s.y;
        if s.x == 0 as libc::c_int as libc::c_double {
            free(nlist as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        if Verbose != 0 {
            fprintf(
                stderr,
                b"compress %g \n\0" as *const u8 as *const libc::c_char,
                s.x,
            );
        }
    } else {
        aarr = mkOverlapSet(nlist, nnodes, &mut m);
        if m == 0 as libc::c_int {
            free(aarr as *mut libc::c_void);
            free(nlist as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        if equal != 0 {
            s.y = computeScale(aarr, m);
            s.x = s.y;
        } else {
            s = computeScaleXY(aarr, m);
        }
        free(aarr as *mut libc::c_void);
        if Verbose != 0 {
            fprintf(
                stderr,
                b"scale by %g,%g \n\0" as *const u8 as *const libc::c_char,
                s.x,
                s.y,
            );
        }
    }
    p = nlist;
    i = 0 as libc::c_int;
    while i < nnodes {
        *((*((*((*p).np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) = s.x * (*p).pos.x;
        *((*((*((*p).np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) = s.y * (*p).pos.y;
        p = p.offset(1);
        i += 1;
    }
    free(nlist as *mut libc::c_void);
    return 1 as libc::c_int;
}
