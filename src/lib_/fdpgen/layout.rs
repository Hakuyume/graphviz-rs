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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn free(_: *mut libc::c_void);
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
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn agdelrec(obj: *mut libc::c_void, name: *const libc::c_char) -> libc::c_int;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Agstrictdirected: Agdesc_t;
    static mut Verbose: libc::c_uchar;
    static mut Nop: libc::c_int;
    static mut PSinputscale: libc::c_double;
    static mut Ndim: libc::c_int;
    static mut State: libc::c_int;
    static mut G_margin: *mut Agsym_t;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn late_int(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn get_inputscale(g: *mut graph_t) -> libc::c_double;
    fn compute_bb(_: *mut Agraph_t);
    fn setEdgeType(g: *mut graph_t, dflt: libc::c_int);
    fn fdp_initParams(_: *mut graph_t);
    fn fdp_init_node_edge(g: *mut Agraph_t);
    fn do_graph_label(sg: *mut graph_t);
    fn gv_postprocess(_: *mut Agraph_t, _: libc::c_int);
    fn fdp_tLayout(_: *mut graph_t, _: *mut xparams);
    fn fdp_xLayout(_: *mut graph_t, _: *mut xparams);
    fn normalize(g: *mut graph_t) -> libc::c_int;
    fn spline_edges1(g: *mut graph_t, _: libc::c_int) -> libc::c_int;
    fn splineEdges(
        _: *mut graph_t,
        edgefn: Option::<
            unsafe extern "C" fn(*mut graph_t, *mut expand_t, libc::c_int) -> libc::c_int,
        >,
        _: libc::c_int,
    ) -> libc::c_int;
    fn neato_set_aspect(g: *mut graph_t) -> bool;
    fn findCComp(
        _: *mut graph_t,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> *mut *mut graph_t;
    fn putGraphs(
        _: libc::c_int,
        _: *mut *mut Agraph_t,
        _: *mut Agraph_t,
        _: *mut pack_info,
    ) -> *mut point;
    fn getPackInfo(
        g: *mut Agraph_t,
        dflt: pack_mode,
        dfltMargin: libc::c_int,
        _: *mut pack_info,
    ) -> pack_mode;
    fn compoundEdges(
        g: *mut graph_t,
        pm: *mut expand_t,
        splines: libc::c_int,
    ) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct bport_s {
    pub e: *mut edge_t,
    pub n: *mut node_t,
    pub alpha: libc::c_double,
}
pub type bport_t = bport_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdata {
    pub ports: *mut bport_t,
    pub nports: libc::c_int,
    pub bb: boxf,
    pub flags: libc::c_int,
    pub level: libc::c_int,
    pub parent: *mut graph_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dndata {
    pub deg: libc::c_int,
    pub wdeg: libc::c_int,
    pub dn: *mut node_t,
    pub disp: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expand_t {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub doAdd: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout_info {
    pub rootg: *mut graph_t,
    pub G_coord: *mut attrsym_t,
    pub G_width: *mut attrsym_t,
    pub G_height: *mut attrsym_t,
    pub gid: libc::c_int,
    pub pack: pack_info,
}
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
pub type packval_t = libc::c_uint;
pub type pack_mode = libc::c_uint;
pub const l_aspect: pack_mode = 5;
pub const l_array: pack_mode = 4;
pub const l_graph: pack_mode = 3;
pub const l_node: pack_mode = 2;
pub const l_clust: pack_mode = 1;
pub const l_undef: pack_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xparams {
    pub numIters: libc::c_int,
    pub T0: libc::c_double,
    pub K: libc::c_double,
    pub C: libc::c_double,
    pub loopcnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct erec {
    pub e: *mut edge_t,
    pub alpha: libc::c_double,
    pub dist2: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clist_t {
    pub cl: *mut *mut graph_t,
    pub sz: libc::c_int,
    pub cnt: libc::c_int,
}
unsafe extern "C" fn finalCC(
    mut g: *mut graph_t,
    mut c_cnt: libc::c_int,
    mut cc: *mut *mut graph_t,
    mut pts: *mut point,
    mut rg: *mut graph_t,
    mut infop: *mut layout_info,
) {
    let mut G_width: *mut attrsym_t = (*infop).G_width;
    let mut G_height: *mut attrsym_t = (*infop).G_height;
    let mut cg: *mut graph_t = 0 as *mut graph_t;
    let mut b: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    let mut bb: box_0 = box_0 {
        LL: point { x: 0, y: 0 },
        UR: point { x: 0, y: 0 },
    };
    let mut bbf: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut pt: point = point { x: 0, y: 0 };
    let mut margin: libc::c_int = 0;
    let mut cp: *mut *mut graph_t = cc;
    let mut pp: *mut point = pts;
    let mut isRoot: libc::c_int = (rg == (*infop).rootg) as libc::c_int;
    let mut isEmpty: libc::c_int = 0 as libc::c_int;
    if c_cnt != 0 {
        let fresh0 = cp;
        cp = cp.offset(1);
        cg = *fresh0;
        bb
            .LL
            .x = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
            >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x + 0.5f64)
                as libc::c_int
        } else {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x - 0.5f64)
                as libc::c_int
        });
        bb
            .LL
            .y = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
            >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y + 0.5f64)
                as libc::c_int
        } else {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y - 0.5f64)
                as libc::c_int
        });
        bb
            .UR
            .x = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
            >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x + 0.5f64)
                as libc::c_int
        } else {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x - 0.5f64)
                as libc::c_int
        });
        bb
            .UR
            .y = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
            >= 0 as libc::c_int as libc::c_double
        {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y + 0.5f64)
                as libc::c_int
        } else {
            ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y - 0.5f64)
                as libc::c_int
        });
        if c_cnt > 1 as libc::c_int {
            let fresh1 = pp;
            pp = pp.offset(1);
            pt = *fresh1;
            bb.LL.x += pt.x;
            bb.LL.y += pt.y;
            bb.UR.x += pt.x;
            bb.UR.y += pt.y;
            loop {
                let fresh2 = cp;
                cp = cp.offset(1);
                cg = *fresh2;
                if cg.is_null() {
                    break;
                }
                b
                    .LL
                    .x = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .LL
                    .x >= 0 as libc::c_int as libc::c_double
                {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                        + 0.5f64) as libc::c_int
                } else {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                        - 0.5f64) as libc::c_int
                });
                b
                    .LL
                    .y = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .LL
                    .y >= 0 as libc::c_int as libc::c_double
                {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
                        + 0.5f64) as libc::c_int
                } else {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
                        - 0.5f64) as libc::c_int
                });
                b
                    .UR
                    .x = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .x >= 0 as libc::c_int as libc::c_double
                {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                        + 0.5f64) as libc::c_int
                } else {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                        - 0.5f64) as libc::c_int
                });
                b
                    .UR
                    .y = (if (*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .bb
                    .UR
                    .y >= 0 as libc::c_int as libc::c_double
                {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
                        + 0.5f64) as libc::c_int
                } else {
                    ((*((*(cg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y
                        - 0.5f64) as libc::c_int
                });
                let fresh3 = pp;
                pp = pp.offset(1);
                pt = *fresh3;
                b.LL.x += pt.x;
                b.LL.y += pt.y;
                b.UR.x += pt.x;
                b.UR.y += pt.y;
                bb.LL.x = if bb.LL.x < b.LL.x { bb.LL.x } else { b.LL.x };
                bb.LL.y = if bb.LL.y < b.LL.y { bb.LL.y } else { b.LL.y };
                bb.UR.x = if bb.UR.x > b.UR.x { bb.UR.x } else { b.UR.x };
                bb.UR.y = if bb.UR.y > b.UR.y { bb.UR.y } else { b.UR.y };
            }
        }
    } else {
        bb.LL.x = 0 as libc::c_int;
        bb.LL.y = 0 as libc::c_int;
        bb
            .UR
            .x = late_int(
            rg as *mut libc::c_void,
            G_width,
            if 0.75f64 * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                (0.75f64 * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                (0.75f64 * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            },
            3 as libc::c_int,
        );
        bb
            .UR
            .y = late_int(
            rg as *mut libc::c_void,
            G_height,
            if 0.5f64 * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                (0.5f64 * 72 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int
            } else {
                (0.5f64 * 72 as libc::c_int as libc::c_double - 0.5f64) as libc::c_int
            },
            3 as libc::c_int,
        );
        isEmpty = 1 as libc::c_int;
    }
    if !((*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null() {
        let mut p: point = point { x: 0, y: 0 };
        let mut d: libc::c_int = 0;
        isEmpty = 0 as libc::c_int;
        p
            .x = (if (*(*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
            .dimen
            .x >= 0 as libc::c_int as libc::c_double
        {
            ((*(*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).dimen.x
                + 0.5f64) as libc::c_int
        } else {
            ((*(*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).dimen.x
                - 0.5f64) as libc::c_int
        });
        p
            .y = (if (*(*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
            .dimen
            .y >= 0 as libc::c_int as libc::c_double
        {
            ((*(*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).dimen.y
                + 0.5f64) as libc::c_int
        } else {
            ((*(*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).dimen.y
                - 0.5f64) as libc::c_int
        });
        d = p.x - (bb.UR.x - bb.LL.x);
        if d > 0 as libc::c_int {
            d /= 2 as libc::c_int;
            bb.LL.x -= d;
            bb.UR.x += d;
        }
    }
    if isRoot != 0 || isEmpty != 0 {
        margin = 0 as libc::c_int;
    } else {
        margin = late_int(
            rg as *mut libc::c_void,
            G_margin,
            8 as libc::c_int,
            0 as libc::c_int,
        );
    }
    pt.x = -bb.LL.x + margin;
    pt
        .y = ((-bb.LL.y + margin) as libc::c_double
        + (*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .border[0 as libc::c_int as usize]
            .y) as libc::c_int;
    bb.LL.x = 0 as libc::c_int;
    bb.LL.y = 0 as libc::c_int;
    bb.UR.x += pt.x + margin;
    bb
        .UR
        .y = (bb.UR.y as libc::c_double
        + ((pt.y + margin) as libc::c_double
            + (*((*(rg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .border[2 as libc::c_int as usize]
                .y)) as libc::c_int;
    if c_cnt != 0 {
        cp = cc;
        pp = pts;
        loop {
            let fresh4 = cp;
            cp = cp.offset(1);
            cg = *fresh4;
            if cg.is_null() {
                break;
            }
            let mut p_0: point = point { x: 0, y: 0 };
            let mut n: *mut node_t = 0 as *mut node_t;
            let mut del: pointf = pointf { x: 0., y: 0. };
            if !pp.is_null() {
                let fresh5 = pp;
                pp = pp.offset(1);
                p_0 = *fresh5;
                p_0.x += pt.x;
                p_0.y += pt.y;
            } else {
                p_0 = pt;
            }
            del.x = p_0.x as libc::c_double / 72 as libc::c_int as libc::c_double;
            del.y = p_0.y as libc::c_double / 72 as libc::c_int as libc::c_double;
            n = agfstnode(cg);
            while !n.is_null() {
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize) += del.x;
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize) += del.y;
                n = agnxtnode(cg, n);
            }
        }
    }
    bbf.LL.x = bb.LL.x as libc::c_double / 72 as libc::c_int as libc::c_double;
    bbf.LL.y = bb.LL.y as libc::c_double / 72 as libc::c_int as libc::c_double;
    bbf.UR.x = bb.UR.x as libc::c_double / 72 as libc::c_int as libc::c_double;
    bbf.UR.y = bb.UR.y as libc::c_double / 72 as libc::c_int as libc::c_double;
    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
        .bb = bbf;
}
unsafe extern "C" fn mkDeriveNode(
    mut dg: *mut graph_t,
    mut name: *mut libc::c_char,
) -> *mut node_t {
    let mut dn: *mut node_t = 0 as *mut node_t;
    dn = agnode(dg, name, 1 as libc::c_int);
    agbindrec(
        dn as *mut libc::c_void,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    let ref mut fresh6 = (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg;
    *fresh6 = zmalloc(::std::mem::size_of::<dndata>() as libc::c_ulong) as *mut dndata
        as *mut libc::c_void;
    let ref mut fresh7 = (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos;
    *fresh7 = gcalloc(
        (*((*(dg as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    return dn;
}
unsafe extern "C" fn freeDeriveNode(mut n: *mut node_t) {
    free((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg);
    free((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos as *mut libc::c_void);
    agdelrec(
        n as *mut libc::c_void,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn freeGData(mut g: *mut graph_t) {
    free((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg);
}
unsafe extern "C" fn freeDerivedGraph(mut g: *mut graph_t, mut cc: *mut *mut graph_t) {
    let mut cg: *mut graph_t = 0 as *mut graph_t;
    let mut dn: *mut node_t = 0 as *mut node_t;
    let mut dnxt: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    loop {
        let fresh8 = cc;
        cc = cc.offset(1);
        cg = *fresh8;
        if cg.is_null() {
            break;
        }
        freeGData(cg);
        agdelrec(
            cg as *mut libc::c_void,
            b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
        .ports)
        .is_null()
    {
        free(
            (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
                .ports as *mut libc::c_void,
        );
    }
    freeGData(g);
    agdelrec(
        g as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
    );
    dn = agfstnode(g);
    while !dn.is_null() {
        dnxt = agnxtnode(g, dn);
        e = agfstout(g, dn);
        while !e.is_null() {
            free(
                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt
                    as *mut libc::c_void,
            );
            agdelrec(
                e as *mut libc::c_void,
                b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
            );
            e = agnxtout(g, e);
        }
        freeDeriveNode(dn);
        dn = dnxt;
    }
    agclose(g);
}
unsafe extern "C" fn evalPositions(mut g: *mut graph_t, mut rootg: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut sbb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    bb = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata)).bb;
    if g != rootg {
        n = agfstnode(g);
        while !n.is_null() {
            if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust != g) {
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize) += bb.LL.x;
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize) += bb.LL.y;
            }
            n = agnxtnode(g, n);
        }
    }
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        subg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(i as isize);
        if g != rootg {
            sbb = (*((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                as *mut gdata))
                .bb;
            sbb.LL.x += bb.LL.x;
            sbb.LL.y += bb.LL.y;
            sbb.UR.x += bb.LL.x;
            sbb.UR.y += bb.LL.y;
            (*((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                as *mut gdata))
                .bb = sbb;
        }
        evalPositions(subg, rootg);
        i += 1;
    }
}
unsafe extern "C" fn initCList(mut clist: *mut clist_t) {
    let ref mut fresh9 = (*clist).cl;
    *fresh9 = 0 as *mut *mut graph_t;
    (*clist).sz = 0 as libc::c_int;
    (*clist).cnt = 0 as libc::c_int;
}
unsafe extern "C" fn addCluster(mut clist: *mut clist_t, mut subg: *mut graph_t) {
    let ref mut fresh10 = (*clist).cnt;
    *fresh10 += 1;
    if (*clist).cnt >= (*clist).sz {
        (*clist).sz += 10 as libc::c_int;
        let ref mut fresh11 = (*clist).cl;
        *fresh11 = grealloc(
            (*clist).cl as *mut libc::c_void,
            ((*clist).sz as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut graph_t>() as libc::c_ulong),
        ) as *mut *mut graph_t;
    }
    let ref mut fresh12 = *((*clist).cl).offset((*clist).cnt as isize);
    *fresh12 = subg;
}
unsafe extern "C" fn portName(
    mut g: *mut graph_t,
    mut p: *mut bport_t,
) -> *mut libc::c_char {
    let mut e: *mut edge_t = (*p).e;
    let mut h: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 2 as libc::c_int
    {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    let mut t: *mut node_t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    static mut buf: [libc::c_char; 1001] = [0; 1001];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong,
        b"_port_%s_(%d)_(%d)_%u\0" as *const u8 as *const libc::c_char,
        agnameof(g as *mut libc::c_void),
        (*((*(t as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id,
        (*((*(h as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id,
        ((*(e as *mut Agobj_t)).tag).seq() as libc::c_int,
    );
    return buf.as_mut_ptr();
}
unsafe extern "C" fn chkPos(
    mut g: *mut graph_t,
    mut n: *mut node_t,
    mut infop: *mut layout_info,
    mut bbp: *mut boxf,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut c: libc::c_char = 0;
    let mut parent: *mut graph_t = 0 as *mut graph_t;
    let mut G_coord: *mut attrsym_t = (*infop).G_coord;
    p = agxget(g as *mut libc::c_void, G_coord);
    if *p.offset(0 as libc::c_int as isize) != 0 {
        if g != (*infop).rootg {
            parent = agparent(g);
            pp = agxget(parent as *mut libc::c_void, G_coord);
            if strcmp(p, pp) == 0 {
                return;
            }
        }
        c = '\0' as i32 as libc::c_char;
        if sscanf(
            p,
            b"%lf,%lf,%lf,%lf%c\0" as *const u8 as *const libc::c_char,
            &mut bb.LL.x as *mut libc::c_double,
            &mut bb.LL.y as *mut libc::c_double,
            &mut bb.UR.x as *mut libc::c_double,
            &mut bb.UR.y as *mut libc::c_double,
            &mut c as *mut libc::c_char,
        ) >= 4 as libc::c_int
        {
            if PSinputscale > 0.0f64 {
                bb.LL.x /= PSinputscale;
                bb.LL.y /= PSinputscale;
                bb.UR.x /= PSinputscale;
                bb.UR.y /= PSinputscale;
            }
            if c as libc::c_int == '!' as i32 {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .pinned = 3 as libc::c_int as libc::c_uchar;
            } else if c as libc::c_int == '?' as i32 {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .pinned = 2 as libc::c_int as libc::c_uchar;
            } else {
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .pinned = 1 as libc::c_int as libc::c_uchar;
            }
            *bbp = bb;
        } else {
            agerr(
                AGWARN,
                b"graph %s, coord %s, expected four doubles\n\0" as *const u8
                    as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
                p,
            );
        }
    }
}
unsafe extern "C" fn addEdge(mut de: *mut edge_t, mut e: *mut edge_t) {
    let mut cnt: libc::c_short = (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .count;
    let mut el: *mut *mut edge_t = 0 as *mut *mut edge_t;
    el = (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt
        as *mut *mut edge_t;
    el = if !el.is_null() {
        grealloc(
            el as *mut libc::c_void,
            ((cnt as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
        ) as *mut *mut edge_t
    } else {
        gmalloc(
            ((cnt as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut edge_t>() as libc::c_ulong),
        ) as *mut *mut edge_t
    };
    let ref mut fresh13 = *el.offset(cnt as isize);
    *fresh13 = e;
    let ref mut fresh14 = (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
    *fresh14 = el as *mut edge_t;
    let ref mut fresh15 = (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count;
    *fresh15 += 1;
}
unsafe extern "C" fn copyAttr(
    mut g: *mut graph_t,
    mut dg: *mut graph_t,
    mut attr: *mut libc::c_char,
) {
    let mut ov_val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ov: *mut Agsym_t = 0 as *mut Agsym_t;
    ov = agattr(g, 0 as libc::c_int, attr, 0 as *const libc::c_char);
    if !ov.is_null() {
        ov_val = agxget(g as *mut libc::c_void, ov);
        ov = agattr(dg, 0 as libc::c_int, attr, 0 as *const libc::c_char);
        if !ov.is_null() {
            agxset(dg as *mut libc::c_void, ov, ov_val);
        } else {
            agattr(dg, 0 as libc::c_int, attr, ov_val);
        }
    }
}
unsafe extern "C" fn deriveGraph(
    mut g: *mut graph_t,
    mut infop: *mut layout_info,
) -> *mut graph_t {
    let mut dg: *mut graph_t = 0 as *mut graph_t;
    let mut dn: *mut node_t = 0 as *mut node_t;
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut pp: *mut bport_t = 0 as *mut bport_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut de: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut id: libc::c_int = 0 as libc::c_int;
    if Verbose as libc::c_int >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"derive graph _dg_%d of %s\n\0" as *const u8 as *const libc::c_char,
            (*infop).gid,
            agnameof(g as *mut libc::c_void),
        );
    }
    let ref mut fresh16 = (*infop).gid;
    *fresh16 += 1;
    dg = agopen(
        b"derived\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agstrictdirected,
        0 as *mut Agdisc_t,
    );
    agbindrec(
        dg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    let ref mut fresh17 = (*((*(dg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg;
    *fresh17 = zmalloc(::std::mem::size_of::<gdata>() as libc::c_ulong) as *mut gdata
        as *mut libc::c_void;
    (*((*(dg as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .ndim = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim;
    copyAttr(
        g,
        dg,
        b"overlap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    copyAttr(g, dg, b"sep\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    copyAttr(g, dg, b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        let mut fix_bb: boxf = {
            let mut init = boxf {
                LL: {
                    let mut init = pointf_s {
                        x: 1.7976931348623157e+308f64,
                        y: 1.7976931348623157e+308f64,
                    };
                    init
                },
                UR: {
                    let mut init = pointf_s {
                        x: -1.7976931348623157e+308f64,
                        y: -1.7976931348623157e+308f64,
                    };
                    init
                },
            };
            init
        };
        subg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
            .offset(i as isize);
        do_graph_label(subg);
        dn = mkDeriveNode(dg, agnameof(subg as *mut libc::c_void));
        let ref mut fresh18 = (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .clust;
        *fresh18 = subg;
        let fresh19 = id;
        id = id + 1;
        (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id = fresh19;
        if !((*infop).G_coord).is_null() {
            chkPos(subg, dn, infop, &mut fix_bb);
        }
        n = agfstnode(subg);
        while !n.is_null() {
            let ref mut fresh20 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .next;
            *fresh20 = dn;
            n = agnxtnode(subg, n);
        }
        if (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned != 0 {
            *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(
                    0 as libc::c_int as isize,
                ) = (fix_bb.LL.x + fix_bb.UR.x) / 2 as libc::c_int as libc::c_double;
            *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(
                    1 as libc::c_int as isize,
                ) = (fix_bb.LL.y + fix_bb.UR.y) / 2 as libc::c_int as libc::c_double;
        }
        i += 1;
    }
    n = agfstnode(g);
    while !n.is_null() {
        if ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next).is_null() {
            if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null()
                && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
                    != (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                        as *mut gdata))
                        .parent
            {
                agerr(
                    AGERR,
                    b"node \"%s\" is contained in two non-comparable clusters \"%s\" and \"%s\"\n\0"
                        as *const u8 as *const libc::c_char,
                    agnameof(n as *mut libc::c_void),
                    agnameof(g as *mut libc::c_void),
                    agnameof(
                        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
                            as *mut libc::c_void,
                    ),
                );
                return 0 as *mut graph_t;
            }
            let ref mut fresh21 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .clust;
            *fresh21 = g;
            if !(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
                dn = mkDeriveNode(dg, agnameof(n as *mut libc::c_void));
                let ref mut fresh22 = (*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .next;
                *fresh22 = dn;
                let fresh23 = id;
                id = id + 1;
                (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id = fresh23;
                (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .width = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width;
                (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .height = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .height;
                (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .lw = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .rw = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
                (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .ht = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht;
                let ref mut fresh24 = (*((*(dn as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .shape;
                *fresh24 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape;
                let ref mut fresh25 = (*((*(dn as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .shape_info;
                *fresh25 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .shape_info;
                if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned != 0 {
                    *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(0 as libc::c_int as isize);
                    *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(1 as libc::c_int as isize);
                    (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .pinned = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .pinned;
                }
                let ref mut fresh26 = (*((*((*(dn as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut dndata))
                    .dn;
                *fresh26 = n;
            }
        }
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        let mut e: *mut edge_t = 0 as *mut edge_t;
        let mut hd: *mut node_t = 0 as *mut node_t;
        let mut tl: *mut node_t = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .next;
        e = agfstout(g, n);
        while !e.is_null() {
            hd = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .next;
            if !(hd == tl) {
                if hd > tl {
                    de = agedge(dg, tl, hd, 0 as *mut libc::c_char, 1 as libc::c_int);
                } else {
                    de = agedge(dg, hd, tl, 0 as *mut libc::c_char, 1 as libc::c_int);
                }
                agbindrec(
                    de as *mut libc::c_void,
                    b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong
                        as libc::c_uint,
                    1 as libc::c_int,
                );
                (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .dist = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist;
                (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .factor = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .factor;
                let ref mut fresh27 = (*((*((*(hd as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut dndata))
                    .wdeg;
                *fresh27 += 1;
                let ref mut fresh28 = (*((*((*(tl as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut dndata))
                    .wdeg;
                *fresh28 += 1;
                if ((*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt)
                    .is_null()
                {
                    let ref mut fresh29 = (*((*((*(hd as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .alg as *mut dndata))
                        .deg;
                    *fresh29 += 1;
                    let ref mut fresh30 = (*((*((*(tl as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .alg as *mut dndata))
                        .deg;
                    *fresh30 += 1;
                }
                addEdge(de, e);
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    pp = (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
        .ports;
    if !pp.is_null() {
        let mut pq: *mut bport_t = 0 as *mut bport_t;
        let mut m: *mut node_t = 0 as *mut node_t;
        let mut sz: libc::c_int = (*((*((*(g as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .alg as *mut gdata))
            .nports;
        pq = gcalloc(
            (sz + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<bport_t>() as libc::c_ulong,
        ) as *mut bport_t;
        let ref mut fresh31 = (*((*((*(dg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .alg as *mut gdata))
            .ports;
        *fresh31 = pq;
        sz = 0 as libc::c_int;
        while !((*pp).e).is_null() {
            m = (*((*((*pp).n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
            if !m.is_null() {
                dn = mkDeriveNode(dg, portName(g, pp));
                sz += 1;
                let fresh32 = id;
                id = id + 1;
                (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id = fresh32;
                if dn > m {
                    de = agedge(dg, m, dn, 0 as *mut libc::c_char, 1 as libc::c_int);
                } else {
                    de = agedge(dg, dn, m, 0 as *mut libc::c_char, 1 as libc::c_int);
                }
                agbindrec(
                    de as *mut libc::c_void,
                    b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong
                        as libc::c_uint,
                    1 as libc::c_int,
                );
                (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .dist = (*((*((*pp).e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .dist;
                (*((*(de as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .factor = (*((*((*pp).e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .factor;
                addEdge(de, (*pp).e);
                let ref mut fresh33 = (*((*((*(dn as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut dndata))
                    .wdeg;
                *fresh33 += 1;
                let ref mut fresh34 = (*((*((*(m as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut dndata))
                    .wdeg;
                *fresh34 += 1;
                let ref mut fresh35 = (*((*((*(dn as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut dndata))
                    .deg;
                *fresh35 += 1;
                let ref mut fresh36 = (*((*((*(m as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut dndata))
                    .deg;
                *fresh36 += 1;
                let ref mut fresh37 = (*pq).n;
                *fresh37 = dn;
                (*pq).alpha = (*pp).alpha;
                let ref mut fresh38 = (*pq).e;
                *fresh38 = de;
                pq = pq.offset(1);
            }
            pp = pp.offset(1);
        }
        (*((*((*(dg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
            .nports = sz;
    }
    return dg;
}
unsafe extern "C" fn ecmp(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> libc::c_int {
    let mut e1: *const erec = v1 as *const erec;
    let mut e2: *const erec = v2 as *const erec;
    if (*e1).alpha > (*e2).alpha {
        return 1 as libc::c_int
    } else if (*e1).alpha < (*e2).alpha {
        return -(1 as libc::c_int)
    } else if (*e1).dist2 > (*e2).dist2 {
        return 1 as libc::c_int
    } else if (*e1).dist2 < (*e2).dist2 {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn getEdgeList(mut n: *mut node_t, mut g: *mut graph_t) -> *mut erec {
    let mut erecs: *mut erec = 0 as *mut erec;
    let mut deg: libc::c_int = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .alg as *mut dndata))
        .deg;
    let mut i: libc::c_int = 0;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut m: *mut node_t = 0 as *mut node_t;
    erecs = gcalloc(
        (deg + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<erec>() as libc::c_ulong,
    ) as *mut erec;
    i = 0 as libc::c_int;
    e = agfstedge(g, n);
    while !e.is_null() {
        if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
            .node == n
        {
            m = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node;
        } else {
            m = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
        }
        dx = *((*((*(m as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize)
            - *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize);
        dy = *((*((*(m as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize)
            - *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize);
        let ref mut fresh39 = (*erecs.offset(i as isize)).e;
        *fresh39 = e;
        (*erecs.offset(i as isize)).alpha = atan2(dy, dx);
        (*erecs.offset(i as isize)).dist2 = dx * dx + dy * dy;
        i += 1;
        e = agnxtedge(g, e, n);
    }
    if i == deg {} else {
        __assert_fail(
            b"i == deg\0" as *const u8 as *const libc::c_char,
            b"layout.c\0" as *const u8 as *const libc::c_char,
            611 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"erec *getEdgeList(node_t *, graph_t *)\0"))
                .as_ptr(),
        );
    }
    qsort(
        erecs as *mut libc::c_void,
        deg as size_t,
        ::std::mem::size_of::<erec>() as libc::c_ulong,
        Some(
            ecmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if deg >= 2 as libc::c_int {
        let mut j: libc::c_int = 0;
        let mut a: libc::c_double = 0.;
        let mut inc: libc::c_double = 0.;
        let mut delta: libc::c_double = 0.;
        let mut bnd: libc::c_double = 0.;
        i = 0 as libc::c_int;
        while i < deg - 1 as libc::c_int {
            a = (*erecs.offset(i as isize)).alpha;
            j = i + 1 as libc::c_int;
            while j < deg && (*erecs.offset(j as isize)).alpha == a {
                j += 1;
            }
            if j == i + 1 as libc::c_int {
                i = j;
            } else {
                if j == deg {
                    bnd = 3.14159265358979323846f64;
                } else {
                    bnd = (*erecs.offset(j as isize)).alpha;
                }
                delta = fmin(
                    (bnd - a) / (j - i) as libc::c_double,
                    3.14159265358979323846f64 / 90 as libc::c_int as libc::c_double,
                );
                inc = 0 as libc::c_int as libc::c_double;
                while i < j {
                    (*erecs.offset(i as isize)).alpha += inc;
                    inc += delta;
                    i += 1;
                }
            }
        }
    }
    return erecs;
}
unsafe extern "C" fn genPorts(
    mut n: *mut node_t,
    mut er: *mut erec,
    mut pp: *mut bport_t,
    mut idx: libc::c_int,
    mut bnd: libc::c_double,
) -> libc::c_int {
    let mut other: *mut node_t = 0 as *mut node_t;
    let mut cnt: libc::c_int = 0;
    let mut e: *mut edge_t = (*er).e;
    let mut el: *mut edge_t = 0 as *mut edge_t;
    let mut ep: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut angle: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut inc: libc::c_int = 0;
    cnt = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count as libc::c_int;
    if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    }))
        .node == n
    {
        other = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
    } else {
        other = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
    }
    delta = fmin(
        (bnd - (*er).alpha) / cnt as libc::c_double,
        3.14159265358979323846f64 / 90 as libc::c_int as libc::c_double,
    );
    angle = (*er).alpha;
    if n < other {
        i = idx;
        inc = 1 as libc::c_int;
    } else {
        i = idx + cnt - 1 as libc::c_int;
        inc = -(1 as libc::c_int);
        angle += delta * (cnt - 1 as libc::c_int) as libc::c_double;
        delta = -delta;
    }
    ep = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt
        as *mut *mut edge_t;
    j = 0 as libc::c_int;
    while j < (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).count as libc::c_int
    {
        el = *ep;
        let ref mut fresh40 = (*pp.offset(i as isize)).e;
        *fresh40 = el;
        let ref mut fresh41 = (*pp.offset(i as isize)).n;
        *fresh41 = if (*((*((*(if ((*(el as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            el
        } else {
            el.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .next == n
        {
            (*if ((*(el as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                el
            } else {
                el.offset(1 as libc::c_int as isize)
            })
                .node
        } else {
            (*if ((*(el as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                el
            } else {
                el.offset(-(1 as libc::c_int as isize))
            })
                .node
        };
        (*pp.offset(i as isize)).alpha = angle;
        i += inc;
        angle += delta;
        j += 1;
        ep = ep.offset(1);
    }
    return idx + cnt;
}
unsafe extern "C" fn expandCluster(
    mut n: *mut node_t,
    mut cg: *mut graph_t,
) -> *mut graph_t {
    let mut es: *mut erec = 0 as *mut erec;
    let mut ep: *mut erec = 0 as *mut erec;
    let mut next: *mut erec = 0 as *mut erec;
    let mut sg: *mut graph_t = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .clust;
    let mut pp: *mut bport_t = 0 as *mut bport_t;
    let mut sz: libc::c_int = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .alg as *mut dndata))
        .wdeg;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut bnd: libc::c_double = 0.;
    if sz != 0 as libc::c_int {
        pp = gcalloc(
            (sz + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<bport_t>() as libc::c_ulong,
        ) as *mut bport_t;
        ep = getEdgeList(n, cg);
        es = ep;
        while !((*ep).e).is_null() {
            next = ep.offset(1 as libc::c_int as isize);
            if !((*next).e).is_null() {
                bnd = (*next).alpha;
            } else {
                bnd = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                    + (*es).alpha;
            }
            idx = genPorts(n, ep, pp, idx, bnd);
            ep = next;
        }
        if idx == sz {} else {
            __assert_fail(
                b"idx == sz\0" as *const u8 as *const libc::c_char,
                b"layout.c\0" as *const u8 as *const libc::c_char,
                731 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"graph_t *expandCluster(node_t *, graph_t *)\0"))
                    .as_ptr(),
            );
        }
        let ref mut fresh42 = (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .alg as *mut gdata))
            .ports;
        *fresh42 = pp;
        (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
            .nports = sz;
        free(es as *mut libc::c_void);
    }
    return sg;
}
unsafe extern "C" fn setClustNodes(mut root: *mut graph_t) {
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut p: *mut graph_t = 0 as *mut graph_t;
    let mut ctr: pointf = pointf { x: 0., y: 0. };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut w: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    let mut h_pts: libc::c_double = 0.;
    let mut h2: libc::c_double = 0.;
    let mut w2: libc::c_double = 0.;
    let mut vertices: *mut pointf = 0 as *mut pointf;
    n = agfstnode(root);
    while !n.is_null() {
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clustnode {
            p = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust;
            bb = (*((*((*(p as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                as *mut gdata))
                .bb;
            w = bb.UR.x - bb.LL.x;
            h = bb.UR.y - bb.LL.y;
            ctr.x = w / 2.0f64;
            ctr.y = h / 2.0f64;
            w2 = w / 2.0f64 * 72 as libc::c_int as libc::c_double;
            h2 = h / 2.0f64 * 72 as libc::c_int as libc::c_double;
            h_pts = h * 72 as libc::c_int as libc::c_double;
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) = ctr.x;
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) = ctr.y;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width = w;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height = h;
            let ref mut fresh43 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .rw;
            *fresh43 = w2;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw = *fresh43;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht = h_pts;
            vertices = (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .shape_info as *mut polygon_t))
                .vertices;
            (*vertices.offset(0 as libc::c_int as isize))
                .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
            (*vertices.offset(0 as libc::c_int as isize)).y = h2;
            (*vertices.offset(1 as libc::c_int as isize))
                .x = -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
            (*vertices.offset(1 as libc::c_int as isize)).y = h2;
            (*vertices.offset(2 as libc::c_int as isize))
                .x = -(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
            (*vertices.offset(2 as libc::c_int as isize)).y = -h2;
            (*vertices.offset(3 as libc::c_int as isize))
                .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
            (*vertices.offset(3 as libc::c_int as isize)).y = -h2;
        }
        n = agnxtnode(root, n);
    }
}
unsafe extern "C" fn layout(
    mut g: *mut graph_t,
    mut infop: *mut layout_info,
) -> libc::c_int {
    let mut pts: *mut point = 0 as *mut point;
    let mut dg: *mut graph_t = 0 as *mut graph_t;
    let mut dn: *mut node_t = 0 as *mut node_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut cg: *mut graph_t = 0 as *mut graph_t;
    let mut sg: *mut graph_t = 0 as *mut graph_t;
    let mut cc: *mut *mut graph_t = 0 as *mut *mut graph_t;
    let mut pg: *mut *mut graph_t = 0 as *mut *mut graph_t;
    let mut c_cnt: libc::c_int = 0;
    let mut pinned: libc::c_int = 0;
    let mut xpms: xparams = xparams {
        numIters: 0,
        T0: 0.,
        K: 0.,
        C: 0.,
        loopcnt: 0,
    };
    if Verbose != 0 {
        fprintf(
            stderr,
            b"layout %s\n\0" as *const u8 as *const libc::c_char,
            agnameof(g as *mut libc::c_void),
        );
    }
    n = agfstnode(g);
    while !n.is_null() {
        let ref mut fresh44 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
        *fresh44 = 0 as *mut node_t;
        n = agnxtnode(g, n);
    }
    dg = deriveGraph(g, infop);
    if dg.is_null() {
        return -(1 as libc::c_int);
    }
    pg = findCComp(dg, &mut c_cnt, &mut pinned);
    cc = pg;
    loop {
        let fresh45 = pg;
        pg = pg.offset(1);
        cg = *fresh45;
        if cg.is_null() {
            break;
        }
        let mut nxtnode: *mut node_t = 0 as *mut node_t;
        fdp_tLayout(cg, &mut xpms);
        n = agfstnode(cg);
        while !n.is_null() {
            nxtnode = agnxtnode(cg, n);
            if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust).is_null() {
                let mut pt: pointf = pointf { x: 0., y: 0. };
                sg = expandCluster(n, cg);
                let mut r: libc::c_int = layout(sg, infop);
                if r != 0 as libc::c_int {
                    return r;
                }
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .width = (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .alg as *mut gdata))
                    .bb
                    .UR
                    .x;
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .height = (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                    .alg as *mut gdata))
                    .bb
                    .UR
                    .y;
                pt
                    .x = 72 as libc::c_int as libc::c_double
                    * (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                        as *mut gdata))
                        .bb
                        .UR
                        .x;
                pt
                    .y = 72 as libc::c_int as libc::c_double
                    * (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                        as *mut gdata))
                        .bb
                        .UR
                        .y;
                let ref mut fresh46 = (*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .lw;
                *fresh46 = pt.x / 2 as libc::c_int as libc::c_double;
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw = *fresh46;
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht = pt.y;
            } else if ((*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                    as *mut dndata))
                    .dn)
                    .is_null()
                    && ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust)
                        .is_null()
                {
                agdelete(cg, n as *mut libc::c_void);
            }
            n = nxtnode;
        }
        if agnnodes(cg) >= 2 as libc::c_int {
            if g == (*infop).rootg {
                normalize(cg);
            }
            fdp_xLayout(cg, &mut xpms);
        }
    }
    if c_cnt > 1 as libc::c_int {
        let mut bp: *mut bool = 0 as *mut bool;
        if pinned != 0 {
            bp = gcalloc(c_cnt as size_t, ::std::mem::size_of::<bool>() as libc::c_ulong)
                as *mut bool;
            *bp.offset(0 as libc::c_int as isize) = 1 as libc::c_int != 0;
        } else {
            bp = 0 as *mut bool;
        }
        let ref mut fresh47 = (*infop).pack.fixed;
        *fresh47 = bp;
        pts = putGraphs(c_cnt, cc, 0 as *mut Agraph_t, &mut (*infop).pack);
        free(bp as *mut libc::c_void);
    } else {
        pts = 0 as *mut point;
        if c_cnt == 1 as libc::c_int {
            compute_bb(*cc.offset(0 as libc::c_int as isize));
        }
    }
    finalCC(dg, c_cnt, cc, pts, g, infop);
    free(pts as *mut libc::c_void);
    dn = agfstnode(dg);
    while !dn.is_null() {
        sg = (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust;
        if !sg.is_null() {
            (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
                .bb
                .LL
                .x = *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize)
                - (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width
                    / 2 as libc::c_int as libc::c_double;
            (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
                .bb
                .LL
                .y = *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize)
                - (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height
                    / 2 as libc::c_int as libc::c_double;
            (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
                .bb
                .UR
                .x = (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                as *mut gdata))
                .bb
                .LL
                .x + (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width;
            (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
                .bb
                .UR
                .y = (*((*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                as *mut gdata))
                .bb
                .LL
                .y + (*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height;
        } else {
            n = (*((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg
                as *mut dndata))
                .dn;
            if !n.is_null() {
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize);
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *((*((*(dn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize);
            }
        }
        dn = agnxtnode(dg, dn);
    }
    (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
        .bb = (*((*((*(dg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
        as *mut gdata))
        .bb;
    freeDerivedGraph(dg, cc);
    free(cc as *mut libc::c_void);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"end %s\n\0" as *const u8 as *const libc::c_char,
            agnameof(g as *mut libc::c_void),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn setBB(mut g: *mut graph_t) {
    let mut i: libc::c_int = 0;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    bb
        .LL
        .x = 72 as libc::c_int as libc::c_double
        * (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
            .bb
            .LL
            .x;
    bb
        .LL
        .y = 72 as libc::c_int as libc::c_double
        * (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
            .bb
            .LL
            .y;
    bb
        .UR
        .x = 72 as libc::c_int as libc::c_double
        * (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
            .bb
            .UR
            .x;
    bb
        .UR
        .y = 72 as libc::c_int as libc::c_double
        * (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg as *mut gdata))
            .bb
            .UR
            .y;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb = bb;
    i = 1 as libc::c_int;
    while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        setBB(
            *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
                .offset(i as isize),
        );
        i += 1;
    }
}
unsafe extern "C" fn init_info(mut g: *mut graph_t, mut infop: *mut layout_info) {
    let ref mut fresh48 = (*infop).G_coord;
    *fresh48 = agattr(
        g,
        0 as libc::c_int,
        b"coords\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let ref mut fresh49 = (*infop).G_width;
    *fresh49 = agattr(
        g,
        0 as libc::c_int,
        b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let ref mut fresh50 = (*infop).G_height;
    *fresh50 = agattr(
        g,
        0 as libc::c_int,
        b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let ref mut fresh51 = (*infop).rootg;
    *fresh51 = g;
    (*infop).gid = 0 as libc::c_int;
    (*infop)
        .pack
        .mode = getPackInfo(
        g,
        l_node,
        8 as libc::c_int / 2 as libc::c_int,
        &mut (*infop).pack,
    );
}
unsafe extern "C" fn mkClusters(
    mut g: *mut graph_t,
    mut pclist: *mut clist_t,
    mut parent: *mut graph_t,
) {
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut list: clist_t = clist_t {
        cl: 0 as *mut *mut graph_t,
        sz: 0,
        cnt: 0,
    };
    let mut clist: *mut clist_t = 0 as *mut clist_t;
    if pclist.is_null() {
        clist = &mut list;
        initCList(clist);
    } else {
        clist = pclist;
    }
    subg = agfstsubg(g);
    while !subg.is_null() {
        if strncmp(
            agnameof(subg as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            agbindrec(
                subg as *mut libc::c_void,
                b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            let ref mut fresh52 = (*((*(subg as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .alg;
            *fresh52 = zmalloc(::std::mem::size_of::<gdata>() as libc::c_ulong)
                as *mut gdata as *mut libc::c_void;
            (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .ndim = (*((*(parent as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim;
            (*((*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg
                as *mut gdata))
                .level = (*((*((*(parent as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .alg as *mut gdata))
                .level + 1 as libc::c_int;
            let ref mut fresh53 = (*((*((*(subg as *mut Agobj_t)).data
                as *mut Agraphinfo_t))
                .alg as *mut gdata))
                .parent;
            *fresh53 = parent;
            addCluster(clist, subg);
            mkClusters(subg, 0 as *mut clist_t, subg);
        } else {
            mkClusters(subg, clist, parent);
        }
        subg = agnxtsubg(subg);
    }
    if pclist.is_null() {
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster = list.cnt;
        if list.cnt != 0 {
            let ref mut fresh54 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                .clust;
            *fresh54 = grealloc(
                list.cl as *mut libc::c_void,
                ((list.cnt + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut graph_t>() as libc::c_ulong),
            ) as *mut *mut graph_t;
        }
    }
}
unsafe extern "C" fn fdp_init_graph(mut g: *mut Agraph_t) {
    setEdgeType(g, (1 as libc::c_int) << 1 as libc::c_int);
    let ref mut fresh55 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).alg;
    *fresh55 = zmalloc(::std::mem::size_of::<gdata>() as libc::c_ulong) as *mut gdata
        as *mut libc::c_void;
    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .ndim = late_int(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"dim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        2 as libc::c_int,
        2 as libc::c_int,
    ) as libc::c_ushort;
    let ref mut fresh56 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim;
    *fresh56 = (if ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim
        as libc::c_int) < 10 as libc::c_int
    {
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim as libc::c_int
    } else {
        10 as libc::c_int
    }) as libc::c_ushort;
    Ndim = *fresh56 as libc::c_int;
    mkClusters(g, 0 as *mut clist_t, g);
    fdp_initParams(g);
    fdp_init_node_edge(g);
}
unsafe extern "C" fn fdpLayout(mut g: *mut graph_t) -> libc::c_int {
    let mut info: layout_info = layout_info {
        rootg: 0 as *mut graph_t,
        G_coord: 0 as *mut attrsym_t,
        G_width: 0 as *mut attrsym_t,
        G_height: 0 as *mut attrsym_t,
        gid: 0,
        pack: pack_info {
            aspect: 0.,
            sz: 0,
            margin: 0,
            doSplines: 0,
            mode: l_undef,
            fixed: 0 as *mut bool,
            vals: 0 as *mut packval_t,
            flags: 0,
        },
    };
    init_info(g, &mut info);
    let mut r: libc::c_int = layout(g, &mut info);
    if r != 0 as libc::c_int {
        return r;
    }
    setClustNodes(g);
    evalPositions(g, g);
    setBB(g);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fdpSplines(mut g: *mut graph_t) {
    let mut trySplines: libc::c_int = 0 as libc::c_int;
    let mut et: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags
        as libc::c_int & (7 as libc::c_int) << 1 as libc::c_int;
    if et > (4 as libc::c_int) << 1 as libc::c_int {
        if et == (6 as libc::c_int) << 1 as libc::c_int {
            trySplines = splineEdges(
                g,
                Some(
                    compoundEdges
                        as unsafe extern "C" fn(
                            *mut graph_t,
                            *mut expand_t,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                (5 as libc::c_int) << 1 as libc::c_int,
            );
            if trySplines != 0 {
                Nop = 2 as libc::c_int;
            }
        }
        if trySplines != 0 || et != (6 as libc::c_int) << 1 as libc::c_int {
            if !(aggetrec(
                g as *mut libc::c_void,
                b"cl_edge_info\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ))
                .is_null()
            {
                agerr(
                    AGWARN,
                    b"splines and cluster edges not supported - using line segments\n\0"
                        as *const u8 as *const libc::c_char,
                );
                et = (1 as libc::c_int) << 1 as libc::c_int;
            } else {
                spline_edges1(g, et);
            }
        }
        Nop = 0 as libc::c_int;
    }
    if State < 1 as libc::c_int {
        spline_edges1(g, et);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdp_layout(mut g: *mut graph_t) {
    let mut save_scale: libc::c_double = PSinputscale;
    PSinputscale = get_inputscale(g);
    fdp_init_graph(g);
    if fdpLayout(g) != 0 as libc::c_int {
        return;
    }
    neato_set_aspect(g);
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags as libc::c_int
        & (7 as libc::c_int) << 1 as libc::c_int
        != (0 as libc::c_int) << 1 as libc::c_int
    {
        fdpSplines(g);
    }
    gv_postprocess(g, 0 as libc::c_int);
    PSinputscale = save_scale;
}
