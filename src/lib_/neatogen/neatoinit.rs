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
    fn time(__timer: *mut time_t) -> time_t;
    fn getpid() -> __pid_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn srand48(__seedval: libc::c_long);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
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
    fn agdelrec(obj: *mut libc::c_void, name: *const libc::c_char) -> libc::c_int;
    fn agclean(g: *mut Agraph_t, kind: libc::c_int, rec_name: *mut libc::c_char);
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
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Nop: libc::c_int;
    static mut PSinputscale: libc::c_double;
    static mut Epsilon: libc::c_double;
    static mut MaxIter: libc::c_int;
    static mut Ndim: libc::c_int;
    static mut State: libc::c_int;
    static mut N_z: *mut Agsym_t;
    static mut E_weight: *mut Agsym_t;
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
    fn late_int(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn late_double(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_double,
        _: libc::c_double,
    ) -> libc::c_double;
    fn get_inputscale(g: *mut graph_t) -> libc::c_double;
    fn mapBool(_: *const libc::c_char, _: bool) -> bool;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn common_init_node(n: *mut node_t);
    fn common_init_edge(e: *mut edge_t) -> libc::c_int;
    fn compute_bb(_: *mut Agraph_t);
    fn setEdgeType(g: *mut graph_t, dflt: libc::c_int);
    fn gv_nodesize(n: *mut Agnode_t, flip: bool);
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn scan_graph_mode(G: *mut graph_t, mode: libc::c_int) -> libc::c_int;
    fn arrow_flags(e: *mut Agedge_t, sflag: *mut libc::c_int, eflag: *mut libc::c_int);
    fn do_graph_label(sg: *mut graph_t);
    fn gv_postprocess(_: *mut Agraph_t, _: libc::c_int);
    fn free_label(_: *mut textlabel_t);
    fn gv_free_splines(e: *mut edge_t);
    fn gv_cleanup_edge(e: *mut Agedge_t);
    fn gv_cleanup_node(n: *mut Agnode_t);
    fn new_spline(e: *mut edge_t, sz: libc::c_int) -> *mut bezier;
    fn xdotBB(g: *mut graph_t) -> boxf;
    fn sepFactor(G: *mut graph_t) -> expand_t;
    fn adjustNodes(G: *mut graph_t) -> libc::c_int;
    fn removeOverlapWith(_: *mut graph_t, _: *mut adjust_data) -> libc::c_int;
    fn graphAdjustMode(
        G: *mut graph_t,
        _: *mut adjust_data,
        dflt: *mut libc::c_char,
    ) -> *mut adjust_data;
    fn shortest_path(_: *mut graph_t, _: libc::c_int);
    fn solve_model(_: *mut graph_t, _: libc::c_int);
    fn jitter3d(_: *mut Agnode_t, _: libc::c_int);
    fn circuit_model(_: *mut graph_t, _: libc::c_int) -> libc::c_int;
    fn diffeq_model(_: *mut graph_t, _: libc::c_int);
    fn initial_positions(_: *mut graph_t, _: libc::c_int);
    fn jitter_d(_: *mut Agnode_t, _: libc::c_int, _: libc::c_int);
    fn spline_edges0(_: *mut Agraph_t, _: bool);
    fn neato_set_aspect(g: *mut graph_t) -> bool;
    fn neato_translate(g: *mut Agraph_t);
    fn scan_graph(_: *mut graph_t) -> libc::c_int;
    fn free_scan_graph(_: *mut graph_t);
    fn spline_edges(_: *mut Agraph_t);
    fn packGraphs(
        _: libc::c_int,
        _: *mut *mut Agraph_t,
        _: *mut Agraph_t,
        _: *mut pack_info,
    ) -> libc::c_int;
    fn getPack(_: *mut Agraph_t, not_def: libc::c_int, dflt: libc::c_int) -> libc::c_int;
    fn getPackModeInfo(
        g: *mut Agraph_t,
        dflt: pack_mode,
        _: *mut pack_info,
    ) -> pack_mode;
    fn pccomps(
        _: *mut Agraph_t,
        _: *mut libc::c_int,
        _: *mut libc::c_char,
        _: *mut bool,
    ) -> *mut *mut Agraph_t;
    fn nodeInduce(_: *mut Agraph_t) -> libc::c_int;
    fn freeGraphData(graph: *mut vtx_data);
    fn stress_majorization_kD_mkernel(
        graph: *mut vtx_data,
        n: libc::c_int,
        coords: *mut *mut libc::c_double,
        nodes: *mut *mut node_t,
        dim: libc::c_int,
        opts: libc::c_int,
        model: libc::c_int,
        maxi: libc::c_int,
    ) -> libc::c_int;
    fn stress_majorization_with_hierarchy(
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut *mut libc::c_double,
        _: *mut *mut node_t,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn stress_majorization_cola(
        _: *mut vtx_data,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_double,
        _: *mut *mut node_t,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut ipsep_options,
    ) -> libc::c_int;
    fn compute_apsp_artifical_weights(
        _: *mut vtx_data,
        _: libc::c_int,
    ) -> *mut *mut DistType;
    fn newPM() -> *mut PointMap;
    fn clearPM(_: *mut PointMap);
    fn freePM(_: *mut PointMap);
    fn insertPM(
        _: *mut PointMap,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn sgd(_: *mut graph_t, _: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint8_t = __uint8_t;
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
    pub u: C2RustUnnamed_1,
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
    pub u: C2RustUnnamed_0,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub union C2RustUnnamed_1 {
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
    pub hl: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
    pub hh: C2RustUnnamed_3,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub graph: C2RustUnnamed_4,
    pub node: C2RustUnnamed_4,
    pub edge: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub type attrsym_t = Agsym_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_6,
    pub s: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub u: C2RustUnnamed_7,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub txt: C2RustUnnamed_8,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct adjust_data {
    pub mode: adjust_mode,
    pub print: *mut libc::c_char,
    pub value: libc::c_int,
    pub scaling: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expand_t {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub doAdd: bool,
}
pub const AllEdges: pos_edge = 2;
pub type pos_edge = libc::c_uint;
pub const SomeEdges: pos_edge = 1;
pub const NoEdges: pos_edge = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vtx_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
    pub eweights: *mut libc::c_float,
    pub edists: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cluster_data {
    pub nvars: libc::c_int,
    pub nclusters: libc::c_int,
    pub clustersizes: *mut libc::c_int,
    pub clusters: *mut *mut libc::c_int,
    pub ntoplevel: libc::c_int,
    pub toplevel: *mut libc::c_int,
    pub bb: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bitarray_t {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub size_bits: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub block: [uint8_t; 8],
    pub base: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipsep_options {
    pub diredges: libc::c_int,
    pub edge_gap: libc::c_double,
    pub noverlap: libc::c_int,
    pub gap: pointf,
    pub nsize: *mut pointf,
    pub clusters: *mut cluster_data,
}
pub type PointMap = Dict_t;
pub type DistType = libc::c_int;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn pointfof(mut x: libc::c_double, mut y: libc::c_double) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = x;
    r.y = y;
    return r;
}
#[inline]
unsafe extern "C" fn bitarray_get(mut self_0: bitarray_t, mut index: size_t) -> bool {
    if index < self_0.size_bits
        && !(b"out of bounds access\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"index < self.size_bits && \"out of bounds access\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"_Bool bitarray_get(bitarray_t, size_t)\0"))
                .as_ptr(),
        );
    }
    let mut base: *const uint8_t = 0 as *const uint8_t;
    if self_0.size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        base = (self_0.c2rust_unnamed.block).as_mut_ptr();
    } else {
        base = self_0.c2rust_unnamed.base;
    }
    return *base.offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int >> index.wrapping_rem(8 as libc::c_int as libc::c_ulong)
        & 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn bitarray_new(
    mut self_0: *mut bitarray_t,
    mut size_bits: size_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int bitarray_new(bitarray_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    if (*self_0).size_bits == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"self->size_bits == 0\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int bitarray_new(bitarray_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    if size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*self_0).c2rust_unnamed.block).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
        );
    } else {
        let mut capacity: size_t = size_bits
            .wrapping_div(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if size_bits.wrapping_rem(8 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as libc::c_ulong,
            );
        let mut base: *mut uint8_t = calloc(
            capacity,
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        ) as *mut uint8_t;
        if (base == 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int
            as libc::c_long != 0
        {
            return 12 as libc::c_int;
        }
        let ref mut fresh0 = (*self_0).c2rust_unnamed.base;
        *fresh0 = base;
    }
    (*self_0).size_bits = size_bits;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bitarray_new_or_exit(mut size_bits: size_t) -> bitarray_t {
    let mut ba: bitarray_t = bitarray_t {
        c2rust_unnamed: C2RustUnnamed_9 { block: [0; 8] },
        size_bits: 0,
    };
    memset(
        &mut ba as *mut bitarray_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bitarray_t>() as libc::c_ulong,
    );
    let mut error: libc::c_int = bitarray_new(&mut ba, size_bits);
    if (error != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    return ba;
}
#[inline]
unsafe extern "C" fn bitarray_set(
    mut self_0: *mut bitarray_t,
    mut index: size_t,
    mut value: bool,
) {
    if index < (*self_0).size_bits
        && !(b"out of bounds access\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"index < self->size_bits && \"out of bounds access\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void bitarray_set(bitarray_t *, size_t, _Bool)\0"))
                .as_ptr(),
        );
    }
    let mut base: *mut uint8_t = 0 as *mut uint8_t;
    if (*self_0).size_bits
        <= (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        base = ((*self_0).c2rust_unnamed.block).as_mut_ptr();
    } else {
        base = (*self_0).c2rust_unnamed.base;
    }
    if value {
        let ref mut fresh1 = *base
            .offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
        *fresh1 = (*fresh1 as libc::c_int
            | ((1 as libc::c_int)
                << index.wrapping_rem(8 as libc::c_int as libc::c_ulong)) as uint8_t
                as libc::c_int) as uint8_t;
    } else {
        let ref mut fresh2 = *base
            .offset(index.wrapping_div(8 as libc::c_int as libc::c_ulong) as isize);
        *fresh2 = (*fresh2 as libc::c_int
            & !((1 as libc::c_int)
                << index.wrapping_rem(8 as libc::c_int as libc::c_ulong)) as uint8_t
                as libc::c_int) as uint8_t;
    };
}
#[inline]
unsafe extern "C" fn bitarray_reset(mut self_0: *mut bitarray_t) {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/bitarray.h\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void bitarray_reset(bitarray_t *)\0"))
                .as_ptr(),
        );
    }
    if (*self_0).size_bits
        > (::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        free((*self_0).c2rust_unnamed.base as *mut libc::c_void);
    }
    memset(
        self_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bitarray_t>() as libc::c_ulong,
    );
}
static mut N_pos: *mut attrsym_t = 0 as *const attrsym_t as *mut attrsym_t;
static mut Pack: libc::c_int = 0;
static mut cc_pfx: *mut libc::c_char = b"_neato_cc\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn neato_init_node(mut n: *mut node_t) {
    agbindrec(
        n as *mut libc::c_void,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    common_init_node(n);
    let ref mut fresh3 = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos;
    *fresh3 = gcalloc(
        (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .ndim as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    gv_nodesize(
        n,
        (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0,
    );
}
unsafe extern "C" fn neato_init_edge(mut e: *mut edge_t) {
    agbindrec(
        e as *mut libc::c_void,
        b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    common_init_edge(e);
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
        .factor = late_double(e as *mut libc::c_void, E_weight, 1.0f64, 1.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn user_pos(
    mut posptr: *mut attrsym_t,
    mut pinptr: *mut attrsym_t,
    mut np: *mut node_t,
    mut nG: libc::c_int,
) -> libc::c_int {
    let mut pvec: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut z: libc::c_double = 0.;
    if posptr.is_null() {
        return 0 as libc::c_int;
    }
    pvec = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos;
    p = agxget(np as *mut libc::c_void, posptr);
    if *p.offset(0 as libc::c_int as isize) != 0 {
        c = '\0' as i32 as libc::c_char;
        if Ndim >= 3 as libc::c_int
            && sscanf(
                p,
                b"%lf,%lf,%lf%c\0" as *const u8 as *const libc::c_char,
                pvec,
                pvec.offset(1 as libc::c_int as isize),
                pvec.offset(2 as libc::c_int as isize),
                &mut c as *mut libc::c_char,
            ) >= 3 as libc::c_int
        {
            (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .pinned = 1 as libc::c_int as libc::c_uchar;
            if PSinputscale > 0.0f64 {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < Ndim {
                    *pvec.offset(i as isize) = *pvec.offset(i as isize) / PSinputscale;
                    i += 1;
                }
            }
            if Ndim > 3 as libc::c_int {
                jitter_d(np, nG, 3 as libc::c_int);
            }
            if c as libc::c_int == '!' as i32
                || !pinptr.is_null()
                    && mapbool(agxget(np as *mut libc::c_void, pinptr)) as libc::c_int
                        != 0
            {
                (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .pinned = 3 as libc::c_int as libc::c_uchar;
            }
            return (0 as libc::c_int == 0) as libc::c_int;
        } else {
            if sscanf(
                p,
                b"%lf,%lf%c\0" as *const u8 as *const libc::c_char,
                pvec,
                pvec.offset(1 as libc::c_int as isize),
                &mut c as *mut libc::c_char,
            ) >= 2 as libc::c_int
            {
                (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .pinned = 1 as libc::c_int as libc::c_uchar;
                if PSinputscale > 0.0f64 {
                    let mut i_0: libc::c_int = 0;
                    i_0 = 0 as libc::c_int;
                    while i_0 < Ndim {
                        *pvec.offset(i_0 as isize) /= PSinputscale;
                        i_0 += 1;
                    }
                }
                if Ndim > 2 as libc::c_int {
                    if !N_z.is_null()
                        && {
                            p = agxget(np as *mut libc::c_void, N_z);
                            !p.is_null()
                        }
                        && sscanf(
                            p,
                            b"%lf\0" as *const u8 as *const libc::c_char,
                            &mut z as *mut libc::c_double,
                        ) == 1 as libc::c_int
                    {
                        if PSinputscale > 0.0f64 {
                            *pvec.offset(2 as libc::c_int as isize) = z / PSinputscale;
                        } else {
                            *pvec.offset(2 as libc::c_int as isize) = z;
                        }
                        jitter_d(np, nG, 3 as libc::c_int);
                    } else {
                        jitter3d(np, nG);
                    }
                }
                if c as libc::c_int == '!' as i32
                    || !pinptr.is_null()
                        && mapbool(agxget(np as *mut libc::c_void, pinptr))
                            as libc::c_int != 0
                {
                    (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .pinned = 3 as libc::c_int as libc::c_uchar;
                }
                return (0 as libc::c_int == 0) as libc::c_int;
            } else {
                agerr(
                    AGERR,
                    b"node %s, position %s, expected two doubles\n\0" as *const u8
                        as *const libc::c_char,
                    agnameof(np as *mut libc::c_void),
                    p,
                );
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn neato_init_node_edge(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut nG: libc::c_int = agnnodes(g);
    let mut N_pin: *mut attrsym_t = 0 as *mut attrsym_t;
    N_pos = agattr(
        g,
        1 as libc::c_int,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_pin = agattr(
        g,
        1 as libc::c_int,
        b"pin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    n = agfstnode(g);
    while !n.is_null() {
        neato_init_node(n);
        user_pos(N_pos, N_pin, n, nG);
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            neato_init_edge(e);
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn neato_cleanup_graph(mut g: *mut graph_t) {
    if Nop != 0 || Pack < 0 as libc::c_int {
        free_scan_graph(g);
        free(
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust
                as *mut libc::c_void,
        );
    }
    if g != agroot(g as *mut libc::c_void) {
        agclean(
            g,
            0 as libc::c_int,
            b"Agraphinfo_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn neato_cleanup(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            gv_cleanup_edge(e);
            e = agnxtout(g, e);
        }
        gv_cleanup_node(n);
        n = agnxtnode(g, n);
    }
    neato_cleanup_graph(g);
}
unsafe extern "C" fn numFields(mut pos: *mut libc::c_uchar) -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_uchar = 0;
    loop {
        while *(*__ctype_b_loc()).offset(*pos as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            pos = pos.offset(1);
        }
        c = *pos;
        if c != 0 {
            cnt += 1;
            loop {
                c = *pos;
                if !(c as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                    && c as libc::c_int != ';' as i32)
                {
                    break;
                }
                pos = pos.offset(1);
            }
        }
        if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            break;
        }
    }
    return cnt;
}
unsafe extern "C" fn set_label(
    mut obj: *mut libc::c_void,
    mut l: *mut textlabel_t,
    mut name: *mut libc::c_char,
) {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut lp: *mut libc::c_char = 0 as *mut libc::c_char;
    lp = agget(obj, name);
    if !lp.is_null()
        && sscanf(
            lp,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut x as *mut libc::c_double,
            &mut y as *mut libc::c_double,
        ) == 2 as libc::c_int
    {
        (*l).pos = pointfof(x, y);
        (*l).set = 1 as libc::c_int != 0;
    }
}
unsafe extern "C" fn cluster_map(
    mut mastergraph: *mut graph_t,
    mut g: *mut graph_t,
) -> *mut cluster_data {
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut cs: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut cn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nclusters: libc::c_int = 0 as libc::c_int;
    let mut assigned: bitarray_t = bitarray_new_or_exit(agnnodes(g) as size_t);
    let mut cdata: *mut cluster_data = gmalloc(
        ::std::mem::size_of::<cluster_data>() as libc::c_ulong,
    ) as *mut cluster_data;
    (*cdata).ntoplevel = agnnodes(g);
    subg = agfstsubg(mastergraph);
    while !subg.is_null() {
        if strncmp(
            agnameof(subg as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            nclusters += 1;
        }
        subg = agnxtsubg(subg);
    }
    (*cdata).nvars = 0 as libc::c_int;
    (*cdata).nclusters = nclusters;
    let ref mut fresh4 = (*cdata).clusters;
    *fresh4 = gcalloc(
        nclusters as size_t,
        ::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
    ) as *mut *mut libc::c_int;
    cs = *fresh4;
    let ref mut fresh5 = (*cdata).clustersizes;
    *fresh5 = gcalloc(
        nclusters as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    cn = *fresh5;
    subg = agfstsubg(mastergraph);
    while !subg.is_null() {
        if strncmp(
            agnameof(subg as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
            *cn = agnnodes(subg);
            (*cdata).nvars += *cn;
            let fresh6 = cn;
            cn = cn.offset(1);
            let fresh7 = cs;
            cs = cs.offset(1);
            *fresh7 = gcalloc(
                *fresh6 as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            c = *fresh7;
            n = agfstnode(subg);
            while !n.is_null() {
                let mut gn: *mut node_t = 0 as *mut node_t;
                let mut ind: libc::c_int = 0 as libc::c_int;
                gn = agfstnode(g);
                while !gn.is_null() {
                    if ((*(gn as *mut Agobj_t)).tag).seq() as libc::c_int
                        == ((*(n as *mut Agobj_t)).tag).seq() as libc::c_int
                    {
                        break;
                    }
                    ind += 1;
                    gn = agnxtnode(g, gn);
                }
                let fresh8 = c;
                c = c.offset(1);
                *fresh8 = ind;
                bitarray_set(&mut assigned, ind as size_t, 1 as libc::c_int != 0);
                let ref mut fresh9 = (*cdata).ntoplevel;
                *fresh9 -= 1;
                n = agnxtnode(subg, n);
            }
        }
        subg = agnxtsubg(subg);
    }
    let ref mut fresh10 = (*cdata).bb;
    *fresh10 = gcalloc(
        (*cdata).nclusters as size_t,
        ::std::mem::size_of::<boxf>() as libc::c_ulong,
    ) as *mut boxf;
    let ref mut fresh11 = (*cdata).toplevel;
    *fresh11 = gcalloc(
        (*cdata).ntoplevel as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    j = 0 as libc::c_int;
    i = j;
    while i < agnnodes(g) {
        if !bitarray_get(assigned, i as size_t) {
            let fresh12 = j;
            j = j + 1;
            *((*cdata).toplevel).offset(fresh12 as isize) = i;
        }
        i += 1;
    }
    if (*cdata).ntoplevel == agnnodes(g) - (*cdata).nvars {} else {
        __assert_fail(
            b"cdata->ntoplevel==agnnodes(g)-cdata->nvars\0" as *const u8
                as *const libc::c_char,
            b"neatoinit.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"cluster_data *cluster_map(graph_t *, graph_t *)\0"))
                .as_ptr(),
        );
    }
    bitarray_reset(&mut assigned);
    return cdata;
}
unsafe extern "C" fn freeClusterData(mut c: *mut cluster_data) {
    if (*c).nclusters > 0 as libc::c_int {
        free(*((*c).clusters).offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free((*c).clusters as *mut libc::c_void);
        free((*c).clustersizes as *mut libc::c_void);
        free((*c).toplevel as *mut libc::c_void);
        free((*c).bb as *mut libc::c_void);
    }
    free(c as *mut libc::c_void);
}
unsafe extern "C" fn user_spline(
    mut E_pos: *mut attrsym_t,
    mut e: *mut edge_t,
) -> libc::c_int {
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut npts: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut ps: *mut pointf = 0 as *mut pointf;
    let mut pp: *mut pointf = 0 as *mut pointf;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut sflag: libc::c_int = 0 as libc::c_int;
    let mut eflag: libc::c_int = 0 as libc::c_int;
    let mut sp: pointf = {
        let mut init = pointf_s {
            x: 0 as libc::c_int as libc::c_double,
            y: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut ep: pointf = {
        let mut init = pointf_s {
            x: 0 as libc::c_int as libc::c_double,
            y: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut newspl: *mut bezier = 0 as *mut bezier;
    let mut more: libc::c_int = 1 as libc::c_int;
    let mut stype: libc::c_int = 0;
    let mut etype: libc::c_int = 0;
    static mut warned: bool = false;
    pos = agxget(e as *mut libc::c_void, E_pos);
    if *pos as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    arrow_flags(e, &mut stype, &mut etype);
    loop {
        i = sscanf(
            pos,
            b"s,%lf,%lf%n\0" as *const u8 as *const libc::c_char,
            &mut x as *mut libc::c_double,
            &mut y as *mut libc::c_double,
            &mut nc as *mut libc::c_int,
        );
        if i == 2 as libc::c_int {
            sflag = 1 as libc::c_int;
            pos = pos.offset(nc as isize);
            sp.x = x;
            sp.y = y;
        }
        i = sscanf(
            pos,
            b" e,%lf,%lf%n\0" as *const u8 as *const libc::c_char,
            &mut x as *mut libc::c_double,
            &mut y as *mut libc::c_double,
            &mut nc as *mut libc::c_int,
        );
        if i == 2 as libc::c_int {
            eflag = 1 as libc::c_int;
            pos = pos.offset(nc as isize);
            ep.x = x;
            ep.y = y;
        }
        npts = numFields(pos as *mut libc::c_uchar);
        n = npts;
        if n < 4 as libc::c_int || n % 3 as libc::c_int != 1 as libc::c_int {
            gv_free_splines(e);
            if !warned {
                warned = 1 as libc::c_int != 0;
                agerr(
                    AGWARN,
                    b"pos attribute for edge (%s,%s) doesn't have 3n+1 points\n\0"
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
            return 0 as libc::c_int;
        }
        ps = if 0 as libc::c_int != 0 {
            grealloc(
                0 as *mut libc::c_void,
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
            ) as *mut pointf
        } else {
            gmalloc(
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
            ) as *mut pointf
        };
        pp = ps;
        while n != 0 {
            i = sscanf(
                pos,
                b"%lf,%lf%n\0" as *const u8 as *const libc::c_char,
                &mut x as *mut libc::c_double,
                &mut y as *mut libc::c_double,
                &mut nc as *mut libc::c_int,
            );
            if i < 2 as libc::c_int {
                if !warned {
                    warned = 1 as libc::c_int != 0;
                    agerr(
                        AGWARN,
                        b"syntax error in pos attribute for edge (%s,%s)\n\0"
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
                free(ps as *mut libc::c_void);
                gv_free_splines(e);
                return 0 as libc::c_int;
            }
            pos = pos.offset(nc as isize);
            (*pp).x = x;
            (*pp).y = y;
            pp = pp.offset(1);
            n -= 1;
        }
        while *(*__ctype_b_loc()).offset(*pos as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            pos = pos.offset(1);
        }
        if *pos as libc::c_int == '\0' as i32 {
            more = 0 as libc::c_int;
        } else {
            pos = pos.offset(1);
        }
        newspl = new_spline(e, npts);
        if sflag != 0 {
            (*newspl).sflag = stype;
            (*newspl).sp = sp;
        }
        if eflag != 0 {
            (*newspl).eflag = etype;
            (*newspl).ep = ep;
        }
        i = 0 as libc::c_int;
        while i < npts {
            *((*newspl).list).offset(i as isize) = *ps.offset(i as isize);
            i += 1;
        }
        free(ps as *mut libc::c_void);
        if !(more != 0) {
            break;
        }
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
        set_label(
            e as *mut libc::c_void,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label,
            b"lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel).is_null() {
        set_label(
            e as *mut libc::c_void,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel,
            b"xlp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label).is_null() {
        set_label(
            e as *mut libc::c_void,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label,
            b"head_lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label).is_null() {
        set_label(
            e as *mut libc::c_void,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label,
            b"tail_lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn nop_init_edges(mut g: *mut Agraph_t) -> pos_edge {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut nedges: libc::c_int = 0 as libc::c_int;
    let mut E_pos: *mut attrsym_t = 0 as *mut attrsym_t;
    if agnedges(g) == 0 as libc::c_int {
        return AllEdges;
    }
    E_pos = agattr(
        g,
        2 as libc::c_int,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if E_pos.is_null() || Nop < 2 as libc::c_int {
        return NoEdges;
    }
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if user_spline(E_pos, e) != 0 {
                nedges += 1;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    if nedges != 0 {
        if nedges == agnedges(g) { return AllEdges } else { return SomeEdges }
    } else {
        return NoEdges
    };
}
unsafe extern "C" fn freeEdgeInfo(mut g: *mut Agraph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            gv_free_splines(e);
            free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label);
            free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel);
            free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label);
            free_label((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label);
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn chkBB(
    mut g: *mut Agraph_t,
    mut G_bb: *mut attrsym_t,
    mut bbp: *mut boxf,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    s = agxget(g as *mut libc::c_void, G_bb);
    if sscanf(
        s,
        b"%lf,%lf,%lf,%lf\0" as *const u8 as *const libc::c_char,
        &mut bb.LL.x as *mut libc::c_double,
        &mut bb.LL.y as *mut libc::c_double,
        &mut bb.UR.x as *mut libc::c_double,
        &mut bb.UR.y as *mut libc::c_double,
    ) == 4 as libc::c_int
    {
        if bb.LL.y > bb.UR.y {
            let mut tmp: libc::c_double = bb.LL.y;
            bb.LL.y = bb.UR.y;
            bb.UR.y = tmp;
        }
        *bbp = bb;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn add_cluster(mut g: *mut Agraph_t, mut subg: *mut Agraph_t) {
    let mut cno: libc::c_int = 0;
    let ref mut fresh13 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .n_cluster;
    *fresh13 += 1;
    cno = *fresh13;
    let ref mut fresh14 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust;
    *fresh14 = if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
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
    let ref mut fresh15 = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust)
        .offset(cno as isize);
    *fresh15 = subg;
    do_graph_label(subg);
}
unsafe extern "C" fn dfs(
    mut subg: *mut Agraph_t,
    mut parentg: *mut Agraph_t,
    mut G_lp: *mut attrsym_t,
    mut G_bb: *mut attrsym_t,
) {
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    if strncmp(
        agnameof(subg as *mut libc::c_void),
        b"cluster\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0 && chkBB(subg, G_bb, &mut bb) != 0
    {
        agbindrec(
            subg as *mut libc::c_void,
            b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
            1 as libc::c_int,
        );
        (*((*(subg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb = bb;
        add_cluster(parentg, subg);
        nop_init_graphs(subg, G_lp, G_bb);
    } else {
        let mut sg: *mut graph_t = 0 as *mut graph_t;
        sg = agfstsubg(subg);
        while !sg.is_null() {
            dfs(sg, parentg, G_lp, G_bb);
            sg = agnxtsubg(sg);
        }
    };
}
unsafe extern "C" fn nop_init_graphs(
    mut g: *mut Agraph_t,
    mut G_lp: *mut attrsym_t,
    mut G_bb: *mut attrsym_t,
) {
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null()
        && !G_lp.is_null()
    {
        s = agxget(g as *mut libc::c_void, G_lp);
        if sscanf(
            s,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut x as *mut libc::c_double,
            &mut y as *mut libc::c_double,
        ) == 2 as libc::c_int
        {
            (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
                .pos = pointfof(x, y);
            (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
                .set = 1 as libc::c_int != 0;
        }
    }
    if G_bb.is_null() {
        return;
    }
    subg = agfstsubg(g);
    while !subg.is_null() {
        dfs(subg, g, G_lp, G_bb);
        subg = agnxtsubg(subg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_nop(
    mut g: *mut Agraph_t,
    mut adjust: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut np: *mut node_t = 0 as *mut node_t;
    let mut posEdges: pos_edge = NoEdges;
    let mut G_lp: *mut attrsym_t = agattr(
        g,
        0 as libc::c_int,
        b"lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let mut G_bb: *mut attrsym_t = agattr(
        g,
        0 as libc::c_int,
        b"bb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let mut didAdjust: libc::c_int = 0 as libc::c_int;
    let mut haveBackground: libc::c_int = 0;
    let mut translate: bool = !mapBool(
        agget(
            g as *mut libc::c_void,
            b"notranslate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        0 as libc::c_int != 0,
    );
    if G_bb.is_null() {
        G_bb = agattr(
            g,
            0 as libc::c_int,
            b"bb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    scan_graph(g);
    i = 0 as libc::c_int;
    loop {
        np = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).neato_nlist)
            .offset(i as isize);
        if np.is_null() {
            break;
        }
        if !((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned as libc::c_int
            > 0 as libc::c_int)
            && strncmp(
                agnameof(np as *mut libc::c_void),
                b"cluster\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) != 0
        {
            agerr(
                AGERR,
                b"node %s in graph %s has no position\n\0" as *const u8
                    as *const libc::c_char,
                agnameof(np as *mut libc::c_void),
                agnameof(g as *mut libc::c_void),
            );
            return -(1 as libc::c_int);
        }
        if !((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null() {
            set_label(
                np as *mut libc::c_void,
                (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel,
                b"xlp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
    nop_init_graphs(g, G_lp, G_bb);
    posEdges = nop_init_edges(g);
    if !((*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).xdots)
        .is_null()
    {
        haveBackground = 1 as libc::c_int;
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
            .ratio_kind = R_NONE;
    } else {
        haveBackground = 0 as libc::c_int;
    }
    if adjust != 0 && Nop == 1 as libc::c_int && haveBackground == 0 {
        didAdjust = adjustNodes(g);
    }
    if didAdjust != 0 {
        if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null() {
            (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label)
                .set = 0 as libc::c_int != 0;
        }
    }
    compute_bb(g);
    if haveBackground != 0 {
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb = xdotBB(g);
    }
    if adjust == 0 {
        let mut n: *mut node_t = 0 as *mut node_t;
        State = 1 as libc::c_int;
        n = agfstnode(g);
        while !n.is_null() {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .x = 72 as libc::c_int as libc::c_double
                * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize);
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .y = 72 as libc::c_int as libc::c_double
                * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize);
            n = agnxtnode(g, n);
        }
    } else {
        let mut didShift: bool = false;
        if translate as libc::c_int != 0 && haveBackground == 0
            && ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                != 0 as libc::c_int as libc::c_double
                || (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
                    != 0 as libc::c_int as libc::c_double)
        {
            neato_translate(g);
        }
        didShift = neato_set_aspect(g);
        if posEdges as libc::c_uint != NoEdges as libc::c_int as libc::c_uint
            && (didShift as libc::c_int != 0 || didAdjust != 0)
        {
            freeEdgeInfo(g);
            posEdges = NoEdges;
        }
        if posEdges as libc::c_uint != AllEdges as libc::c_int as libc::c_uint {
            spline_edges0(g, 0 as libc::c_int != 0);
        } else {
            State = 1 as libc::c_int;
        }
    }
    return haveBackground;
}
unsafe extern "C" fn neato_init_graph(mut g: *mut Agraph_t) {
    let mut outdim: libc::c_int = 0;
    setEdgeType(g, (1 as libc::c_int) << 1 as libc::c_int);
    outdim = late_int(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"dimen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        2 as libc::c_int,
        2 as libc::c_int,
    );
    (*((*(agroot(g as *mut libc::c_void) as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .ndim = late_int(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"dim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        outdim,
        2 as libc::c_int,
    ) as libc::c_ushort;
    let ref mut fresh16 = (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .ndim;
    *fresh16 = (if ((*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim
        as libc::c_int) < 10 as libc::c_int
    {
        (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).ndim as libc::c_int
    } else {
        10 as libc::c_int
    }) as libc::c_ushort;
    Ndim = *fresh16 as libc::c_int;
    (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .odim = (if outdim < Ndim { outdim } else { Ndim }) as libc::c_ushort;
    neato_init_node_edge(g);
}
unsafe extern "C" fn neatoModel(mut g: *mut graph_t) -> libc::c_int {
    let mut p: *mut libc::c_char = agget(
        g as *mut libc::c_void,
        b"model\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if p.is_null() || strcmp(p, b"\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int;
    }
    if strcmp(p, b"circuit\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int;
    }
    if strcmp(p, b"subset\0" as *const u8 as *const libc::c_char) == 0 {
        return 2 as libc::c_int;
    }
    if strcmp(p, b"shortpath\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int;
    }
    if strcmp(p, b"mds\0" as *const u8 as *const libc::c_char) == 0 {
        if !(agattr(
            g,
            2 as libc::c_int,
            b"len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ))
            .is_null()
        {
            return 3 as libc::c_int
        } else {
            agerr(
                AGWARN,
                b"edges in graph %s have no len attribute. Hence, the mds model\n\0"
                    as *const u8 as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
            );
            agerr(
                AGPREV,
                b"is inappropriate. Reverting to the shortest path model.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    agerr(
        AGWARN,
        b"Unknown value %s for attribute \"model\" in graph %s - ignored\n\0"
            as *const u8 as *const libc::c_char,
        p,
        agnameof(g as *mut libc::c_void),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn neatoMode(mut g: *mut graph_t) -> libc::c_int {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode: libc::c_int = 1 as libc::c_int;
    str = agget(
        g as *mut libc::c_void,
        b"mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && strcmp(str, b"\0" as *const u8 as *const libc::c_char) != 0 {
        if strcmp(str, b"KK\0" as *const u8 as *const libc::c_char) == 0 {
            mode = 0 as libc::c_int;
        } else if strcmp(str, b"major\0" as *const u8 as *const libc::c_char) == 0 {
            mode = 1 as libc::c_int;
        } else if strcmp(str, b"sgd\0" as *const u8 as *const libc::c_char) == 0 {
            mode = 4 as libc::c_int;
        } else if strcmp(str, b"hier\0" as *const u8 as *const libc::c_char) == 0 {
            mode = 2 as libc::c_int;
        } else if strcmp(str, b"ipsep\0" as *const u8 as *const libc::c_char) == 0 {
            mode = 3 as libc::c_int;
        } else {
            agerr(
                AGWARN,
                b"Illegal value %s for attribute \"mode\" in graph %s - ignored\n\0"
                    as *const u8 as *const libc::c_char,
                str,
                agnameof(g as *mut libc::c_void),
            );
        }
    }
    return mode;
}
unsafe extern "C" fn checkEdge(
    mut pm: *mut PointMap,
    mut ep: *mut edge_t,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = (*((*((*if ((*(ep as *mut Agobj_t)).tag).objtype()
        as libc::c_int == 3 as libc::c_int
    {
        ep
    } else {
        ep.offset(1 as libc::c_int as isize)
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .id;
    let mut j: libc::c_int = (*((*((*if ((*(ep as *mut Agobj_t)).tag).objtype()
        as libc::c_int == 2 as libc::c_int
    {
        ep
    } else {
        ep.offset(-(1 as libc::c_int as isize))
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .id;
    let mut tmp: libc::c_int = 0;
    if i > j {
        tmp = i;
        i = j;
        j = tmp;
    }
    return insertPM(pm, i, j, idx);
}
unsafe extern "C" fn dfsCycle(
    mut graph: *mut vtx_data,
    mut i: libc::c_int,
    mut mode: libc::c_int,
    mut nodes: *mut *mut node_t,
) {
    let mut np: *mut node_t = 0 as *mut node_t;
    let mut hp: *mut node_t = 0 as *mut node_t;
    let mut j: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut x: libc::c_double = if mode == 3 as libc::c_int { -1.0f64 } else { 1.0f64 };
    np = *nodes.offset(i as isize);
    (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .mark = (0 as libc::c_int == 0) as libc::c_int as size_t;
    (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .onstack = 1 as libc::c_int as libc::c_char;
    e = 1 as libc::c_int;
    while e < (*graph.offset(i as isize)).nedges {
        if !(*((*graph.offset(i as isize)).edists).offset(e as isize) as libc::c_double
            == 1.0f64)
        {
            j = *((*graph.offset(i as isize)).edges).offset(e as isize);
            hp = *nodes.offset(j as isize);
            if (*((*(hp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).onstack != 0 {
                *((*graph.offset(i as isize)).edists)
                    .offset(e as isize) = x as libc::c_float;
                f = 1 as libc::c_int;
                while f < (*graph.offset(j as isize)).nedges
                    && *((*graph.offset(j as isize)).edges).offset(f as isize) != i
                {
                    f += 1;
                }
                if f < (*graph.offset(j as isize)).nedges {} else {
                    __assert_fail(
                        b"f < graph[j].nedges\0" as *const u8 as *const libc::c_char,
                        b"neatoinit.c\0" as *const u8 as *const libc::c_char,
                        720 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 47],
                            &[libc::c_char; 47],
                        >(b"void dfsCycle(vtx_data *, int, int, node_t **)\0"))
                            .as_ptr(),
                    );
                }
                *((*graph.offset(j as isize)).edists)
                    .offset(f as isize) = -1.0f64 as libc::c_float;
            } else if (*((*(hp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark == 0 {
                dfsCycle(graph, j, mode, nodes);
            }
        }
        e += 1;
    }
    (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .onstack = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn acyclic(
    mut graph: *mut vtx_data,
    mut nv: libc::c_int,
    mut mode: libc::c_int,
    mut nodes: *mut *mut node_t,
) {
    let mut i: libc::c_int = 0;
    let mut np: *mut node_t = 0 as *mut node_t;
    i = 0 as libc::c_int;
    while i < nv {
        np = *nodes.offset(i as isize);
        (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .mark = 0 as libc::c_int as size_t;
        (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .onstack = 0 as libc::c_int as libc::c_char;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nv {
        if !((*((*(*nodes.offset(i as isize) as *mut Agobj_t)).data
            as *mut Agnodeinfo_t))
            .mark != 0)
        {
            dfsCycle(graph, i, mode, nodes);
        }
        i += 1;
    }
}
unsafe extern "C" fn makeGraphData(
    mut g: *mut graph_t,
    mut nv: libc::c_int,
    mut nedges: *mut libc::c_int,
    mut mode: libc::c_int,
    mut model: libc::c_int,
    mut nodedata: *mut *mut *mut node_t,
) -> *mut vtx_data {
    let mut graph: *mut vtx_data = 0 as *mut vtx_data;
    let mut nodes: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut ne: libc::c_int = agnedges(g);
    let mut edges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ewgts: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut np: *mut node_t = 0 as *mut node_t;
    let mut ep: *mut edge_t = 0 as *mut edge_t;
    let mut eweights: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut edists: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut haveLen: *mut attrsym_t = 0 as *mut attrsym_t;
    let mut haveWt: libc::c_int = 0;
    let mut haveDir: libc::c_int = 0;
    let mut ps: *mut PointMap = newPM();
    let mut i: libc::c_int = 0;
    let mut i_nedges: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    if model == 2 as libc::c_int {
        haveLen = 0 as *mut attrsym_t;
        haveWt = 0 as libc::c_int;
    } else {
        haveLen = agattr(
            g,
            2 as libc::c_int,
            b"len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
        haveWt = (E_weight != 0 as *mut Agsym_t) as libc::c_int;
    }
    if mode == 2 as libc::c_int || mode == 3 as libc::c_int {
        haveDir = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        haveDir = 0 as libc::c_int;
    }
    graph = gcalloc(nv as size_t, ::std::mem::size_of::<vtx_data>() as libc::c_ulong)
        as *mut vtx_data;
    nodes = gcalloc(nv as size_t, ::std::mem::size_of::<*mut node_t>() as libc::c_ulong)
        as *mut *mut node_t;
    edges = gcalloc(
        (2 as libc::c_int * ne + nv) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if !haveLen.is_null() || haveDir != 0 {
        ewgts = gcalloc(
            (2 as libc::c_int * ne + nv) as size_t,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
    }
    if haveWt != 0 {
        eweights = gcalloc(
            (2 as libc::c_int * ne + nv) as size_t,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
    }
    if haveDir != 0 {
        edists = gcalloc(
            (2 as libc::c_int * ne + nv) as size_t,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
    }
    i = 0 as libc::c_int;
    ne = 0 as libc::c_int;
    np = agfstnode(g);
    while !np.is_null() {
        let mut j: libc::c_int = 1 as libc::c_int;
        clearPM(ps);
        if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id == i {} else {
            __assert_fail(
                b"ND_id(np) == i\0" as *const u8 as *const libc::c_char,
                b"neatoinit.c\0" as *const u8 as *const libc::c_char,
                818 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"vtx_data *makeGraphData(graph_t *, int, int *, int, int, node_t ***)\0",
                ))
                    .as_ptr(),
            );
        }
        let ref mut fresh17 = *nodes.offset(i as isize);
        *fresh17 = np;
        let fresh18 = edges;
        edges = edges.offset(1);
        let ref mut fresh19 = (*graph.offset(i as isize)).edges;
        *fresh19 = fresh18;
        if !haveLen.is_null() || haveDir != 0 {
            let fresh20 = ewgts;
            ewgts = ewgts.offset(1);
            let ref mut fresh21 = (*graph.offset(i as isize)).ewgts;
            *fresh21 = fresh20;
        } else {
            let ref mut fresh22 = (*graph.offset(i as isize)).ewgts;
            *fresh22 = 0 as *mut libc::c_float;
        }
        if haveWt != 0 {
            let fresh23 = eweights;
            eweights = eweights.offset(1);
            let ref mut fresh24 = (*graph.offset(i as isize)).eweights;
            *fresh24 = fresh23;
        } else {
            let ref mut fresh25 = (*graph.offset(i as isize)).eweights;
            *fresh25 = 0 as *mut libc::c_float;
        }
        if haveDir != 0 {
            let fresh26 = edists;
            edists = edists.offset(1);
            let ref mut fresh27 = (*graph.offset(i as isize)).edists;
            *fresh27 = fresh26;
        } else {
            let ref mut fresh28 = (*graph.offset(i as isize)).edists;
            *fresh28 = 0 as *mut libc::c_float;
        }
        i_nedges = 1 as libc::c_int;
        ep = agfstedge(g, np);
        while !ep.is_null() {
            if !((*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                ep
            } else {
                ep.offset(-(1 as libc::c_int as isize))
            }))
                .node
                == (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    ep
                } else {
                    ep.offset(1 as libc::c_int as isize)
                }))
                    .node)
            {
                idx = checkEdge(ps, ep, j);
                if idx != j {
                    if haveWt != 0 {
                        let ref mut fresh29 = *((*graph.offset(i as isize)).eweights)
                            .offset(idx as isize);
                        *fresh29 = (*fresh29 as libc::c_double
                            + (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .factor) as libc::c_float;
                    }
                    if !haveLen.is_null() {
                        let mut curlen: libc::c_int = *((*graph.offset(i as isize))
                            .ewgts)
                            .offset(idx as isize) as libc::c_int;
                        *((*graph.offset(i as isize)).ewgts)
                            .offset(
                                idx as isize,
                            ) = (if (*((*(ep as *mut Agobj_t)).data
                            as *mut Agedgeinfo_t))
                            .dist > curlen as libc::c_double
                        {
                            (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist
                        } else {
                            curlen as libc::c_double
                        }) as libc::c_float;
                    }
                } else {
                    let mut vp: *mut node_t = if (*(if ((*(ep as *mut Agobj_t)).tag)
                        .objtype() as libc::c_int == 3 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(1 as libc::c_int as isize)
                    }))
                        .node == np
                    {
                        (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            ep
                        } else {
                            ep.offset(-(1 as libc::c_int as isize))
                        })
                            .node
                    } else {
                        (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            ep
                        } else {
                            ep.offset(1 as libc::c_int as isize)
                        })
                            .node
                    };
                    ne += 1;
                    j += 1;
                    let fresh30 = edges;
                    edges = edges.offset(1);
                    *fresh30 = (*((*(vp as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id;
                    if haveWt != 0 {
                        let fresh31 = eweights;
                        eweights = eweights.offset(1);
                        *fresh31 = (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .factor as libc::c_float;
                    }
                    if !haveLen.is_null() {
                        let fresh32 = ewgts;
                        ewgts = ewgts.offset(1);
                        *fresh32 = (*((*(ep as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .dist as libc::c_float;
                    } else if haveDir != 0 {
                        let fresh33 = ewgts;
                        ewgts = ewgts.offset(1);
                        *fresh33 = 1.0f64 as libc::c_float;
                    }
                    if haveDir != 0 {
                        let mut s: *mut libc::c_char = agget(
                            ep as *mut libc::c_void,
                            b"dir\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        if !s.is_null()
                            && strncmp(
                                s,
                                b"none\0" as *const u8 as *const libc::c_char,
                                4 as libc::c_int as libc::c_ulong,
                            ) == 0
                        {
                            let fresh34 = edists;
                            edists = edists.offset(1);
                            *fresh34 = 0 as libc::c_int as libc::c_float;
                        } else {
                            let fresh35 = edists;
                            edists = edists.offset(1);
                            *fresh35 = (if np
                                == (*(if ((*(ep as *mut Agobj_t)).tag).objtype()
                                    as libc::c_int == 2 as libc::c_int
                                {
                                    ep
                                } else {
                                    ep.offset(-(1 as libc::c_int as isize))
                                }))
                                    .node
                            {
                                1.0f64
                            } else {
                                -1.0f64
                            }) as libc::c_float;
                        }
                    }
                    i_nedges += 1;
                }
            }
            ep = agnxtedge(g, ep, np);
        }
        (*graph.offset(i as isize)).nedges = i_nedges;
        *((*graph.offset(i as isize)).edges).offset(0 as libc::c_int as isize) = i;
        i += 1;
        np = agnxtnode(g, np);
    }
    if haveDir != 0 {
        acyclic(graph, nv, mode, nodes);
    }
    ne /= 2 as libc::c_int;
    if ne != agnedges(g) {
        edges = grealloc(
            (*graph.offset(0 as libc::c_int as isize)).edges as *mut libc::c_void,
            ((2 as libc::c_int * ne + nv) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if !haveLen.is_null() {
            ewgts = grealloc(
                (*graph.offset(0 as libc::c_int as isize)).ewgts as *mut libc::c_void,
                ((2 as libc::c_int * ne + nv) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_float;
        }
        if haveWt != 0 {
            eweights = grealloc(
                (*graph.offset(0 as libc::c_int as isize)).eweights as *mut libc::c_void,
                ((2 as libc::c_int * ne + nv) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_float;
        }
        i = 0 as libc::c_int;
        while i < nv {
            let mut sz: libc::c_int = (*graph.offset(i as isize)).nedges;
            let ref mut fresh36 = (*graph.offset(i as isize)).edges;
            *fresh36 = edges;
            edges = edges.offset(sz as isize);
            if !haveLen.is_null() {
                let ref mut fresh37 = (*graph.offset(i as isize)).ewgts;
                *fresh37 = ewgts;
                ewgts = ewgts.offset(sz as isize);
            }
            if haveWt != 0 {
                let ref mut fresh38 = (*graph.offset(i as isize)).eweights;
                *fresh38 = eweights;
                eweights = eweights.offset(sz as isize);
            }
            i += 1;
        }
    }
    *nedges = ne;
    if !nodedata.is_null() {
        *nodedata = nodes;
    } else {
        free(nodes as *mut libc::c_void);
    }
    freePM(ps);
    return graph;
}
unsafe extern "C" fn initRegular(mut G: *mut graph_t, mut nG: libc::c_int) {
    let mut a: libc::c_double = 0.;
    let mut da: libc::c_double = 0.;
    let mut np: *mut node_t = 0 as *mut node_t;
    a = 0.0f64;
    da = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / nG as libc::c_double;
    np = agfstnode(G);
    while !np.is_null() {
        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) = nG as libc::c_double * 1.0f64 * cos(a);
        *((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) = nG as libc::c_double * 1.0f64 * sin(a);
        (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
            .pinned = 1 as libc::c_int as libc::c_uchar;
        a = a + da;
        if Ndim > 2 as libc::c_int {
            jitter3d(np, nG);
        }
        np = agnxtnode(G, np);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setSeed(
    mut G: *mut graph_t,
    mut dflt: libc::c_int,
    mut seedp: *mut libc::c_long,
) -> libc::c_int {
    let mut smallbuf: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = agget(
        G as *mut libc::c_void,
        b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut init: libc::c_int = dflt;
    if p.is_null() || *p as libc::c_int == '\0' as i32 {
        return dflt;
    }
    if *(*__ctype_b_loc()).offset(*(p as *mut libc::c_uchar) as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        if strncmp(
            p,
            b"self\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            init = 0 as libc::c_int;
            p = p
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        } else if strncmp(
                p,
                b"regular\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
            init = 1 as libc::c_int;
            p = p
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        } else if strncmp(
                p,
                b"random\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
            init = 2 as libc::c_int;
            p = p
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        } else {
            init = dflt;
        }
    } else if *(*__ctype_b_loc())
            .offset(*(p as *mut libc::c_uchar) as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
        init = 2 as libc::c_int;
    }
    if init == 2 as libc::c_int {
        let mut seed: libc::c_long = 0;
        if *(*__ctype_b_loc()).offset(*(p as *mut libc::c_uchar) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
            || sscanf(
                p,
                b"%ld\0" as *const u8 as *const libc::c_char,
                &mut seed as *mut libc::c_long,
            ) < 1 as libc::c_int
        {
            seed = (getpid() as libc::c_uint ^ time(0 as *mut time_t) as libc::c_uint)
                as libc::c_long;
            snprintf(
                smallbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%ld\0" as *const u8 as *const libc::c_char,
                seed,
            );
            agset(
                G as *mut libc::c_void,
                b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                smallbuf.as_mut_ptr(),
            );
        }
        *seedp = seed;
    }
    return init;
}
unsafe extern "C" fn checkExp(mut G: *mut graph_t) -> libc::c_int {
    let mut exp: libc::c_int = late_int(
        G as *mut libc::c_void,
        agattr(
            G,
            0 as libc::c_int,
            b"stresswt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        2 as libc::c_int,
        0 as libc::c_int,
    );
    if exp == 0 as libc::c_int || exp > 2 as libc::c_int {
        agerr(
            AGWARN,
            b"%s attribute value must be 1 or 2 - ignoring\n\0" as *const u8
                as *const libc::c_char,
            b"stresswt\0" as *const u8 as *const libc::c_char,
        );
        exp = 2 as libc::c_int;
    }
    return exp;
}
#[no_mangle]
pub unsafe extern "C" fn checkStart(
    mut G: *mut graph_t,
    mut nG: libc::c_int,
    mut dflt: libc::c_int,
) -> libc::c_int {
    let mut seed: libc::c_long = 0;
    let mut init: libc::c_int = 0;
    seed = 1 as libc::c_int as libc::c_long;
    init = setSeed(G, dflt, &mut seed);
    if !N_pos.is_null() && init != 2 as libc::c_int {
        agerr(
            AGWARN,
            b"node positions are ignored unless start=random\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if init == 1 as libc::c_int {
        initRegular(G, nG);
    }
    srand48(seed);
    return init;
}
unsafe extern "C" fn majorization(
    mut mg: *mut graph_t,
    mut g: *mut graph_t,
    mut nv: libc::c_int,
    mut mode: libc::c_int,
    mut model: libc::c_int,
    mut dim: libc::c_int,
    mut am: *mut adjust_data,
) {
    let mut coords: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut ne: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut gp: *mut vtx_data = 0 as *mut vtx_data;
    let mut nodes: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut margin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    let mut init: libc::c_int = checkStart(
        g,
        nv,
        if mode == 2 as libc::c_int { 0 as libc::c_int } else { 2 as libc::c_int },
    );
    let mut opts: libc::c_int = checkExp(g);
    if init == 0 as libc::c_int {
        opts |= 0x4 as libc::c_int;
    }
    coords = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut libc::c_double;
    let ref mut fresh39 = *coords.offset(0 as libc::c_int as isize);
    *fresh39 = gcalloc(
        (nv * dim) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < Ndim {
        let ref mut fresh40 = *coords.offset(i as isize);
        *fresh40 = (*coords.offset(0 as libc::c_int as isize)).offset((i * nv) as isize);
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"model %d smart_init %d stresswt %d iterations %d tol %f\n\0" as *const u8
                as *const libc::c_char,
            model,
            (init == 0 as libc::c_int) as libc::c_int,
            opts & 0x3 as libc::c_int,
            MaxIter,
            Epsilon,
        );
        fprintf(stderr, b"convert graph: \0" as *const u8 as *const libc::c_char);
        start_timer();
        fprintf(stderr, b"majorization\n\0" as *const u8 as *const libc::c_char);
    }
    gp = makeGraphData(g, nv, &mut ne, mode, model, &mut nodes);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"%d nodes %.2f sec\n\0" as *const u8 as *const libc::c_char,
            nv,
            elapsed_sec(),
        );
    }
    if mode != 1 as libc::c_int {
        let mut lgap: libc::c_double = late_double(
            g as *mut libc::c_void,
            agattr(
                g,
                0 as libc::c_int,
                b"levelsgap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            ),
            0.0f64,
            -1.7976931348623157e+308f64,
        );
        if mode == 2 as libc::c_int {
            rv = stress_majorization_with_hierarchy(
                gp,
                nv,
                coords,
                nodes,
                Ndim,
                opts,
                model,
                MaxIter,
                lgap,
            );
        } else {
            let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut opt: ipsep_options = ipsep_options {
                diredges: 0,
                edge_gap: 0.,
                noverlap: 0,
                gap: pointf { x: 0., y: 0. },
                nsize: 0 as *mut pointf,
                clusters: 0 as *mut cluster_data,
            };
            let mut nsize: *mut pointf = 0 as *mut pointf;
            let mut cs: *mut cluster_data = cluster_map(mg, g);
            nsize = gcalloc(
                nv as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            opt.edge_gap = lgap;
            opt.nsize = nsize;
            opt.clusters = cs;
            str = agget(
                g as *mut libc::c_void,
                b"diredgeconstraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            if mapbool(str) {
                opt.diredges = 1 as libc::c_int;
                if Verbose != 0 {
                    fprintf(
                        stderr,
                        b"Generating Edge Constraints...\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if !str.is_null()
                    && strncasecmp(
                        str,
                        b"hier\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                opt.diredges = 2 as libc::c_int;
                if Verbose != 0 {
                    fprintf(
                        stderr,
                        b"Generating DiG-CoLa Edge Constraints...\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                opt.diredges = 0 as libc::c_int;
            }
            if (*am).mode as libc::c_uint == AM_IPSEP as libc::c_int as libc::c_uint {
                opt.noverlap = 1 as libc::c_int;
                if Verbose != 0 {
                    fprintf(
                        stderr,
                        b"Generating Non-overlap Constraints...\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if (*am).mode as libc::c_uint
                    == AM_VPSC as libc::c_int as libc::c_uint
                {
                opt.noverlap = 2 as libc::c_int;
                if Verbose != 0 {
                    fprintf(
                        stderr,
                        b"Removing overlaps as postprocess...\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                opt.noverlap = 0 as libc::c_int;
            }
            margin = sepFactor(g);
            if margin.doAdd {
                opt
                    .gap
                    .x = 2.0f64
                    * (margin.x as libc::c_double / 72 as libc::c_int as libc::c_double);
                opt
                    .gap
                    .y = 2.0f64
                    * (margin.y as libc::c_double / 72 as libc::c_int as libc::c_double);
            } else {
                opt
                    .gap
                    .y = 2.0f64
                    * (4 as libc::c_int as libc::c_double
                        / 72 as libc::c_int as libc::c_double);
                opt.gap.x = opt.gap.y;
            }
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"gap=%f,%f\n\0" as *const u8 as *const libc::c_char,
                    opt.gap.x,
                    opt.gap.y,
                );
            }
            let mut i_0: size_t = 0 as libc::c_int as size_t;
            v = agfstnode(g);
            while !v.is_null() {
                (*nsize.offset(i_0 as isize))
                    .x = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width;
                (*nsize.offset(i_0 as isize))
                    .y = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height;
                v = agnxtnode(g, v);
                i_0 = i_0.wrapping_add(1);
            }
            rv = stress_majorization_cola(
                gp,
                nv,
                ne,
                coords,
                nodes,
                Ndim,
                model,
                MaxIter,
                &mut opt,
            );
            freeClusterData(cs);
            free(nsize as *mut libc::c_void);
        }
    } else {
        rv = stress_majorization_kD_mkernel(
            gp,
            nv,
            coords,
            nodes,
            Ndim,
            opts,
            model,
            MaxIter,
        );
    }
    if rv < 0 as libc::c_int {
        agerr(AGPREV, b"layout aborted\n\0" as *const u8 as *const libc::c_char);
    } else {
        v = agfstnode(g);
        while !v.is_null() {
            let mut idx: libc::c_int = (*((*(v as *mut Agobj_t)).data
                as *mut Agnodeinfo_t))
                .id;
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < Ndim {
                *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(
                        i_1 as isize,
                    ) = *(*coords.offset(i_1 as isize)).offset(idx as isize);
                i_1 += 1;
            }
            v = agnxtnode(g, v);
        }
    }
    freeGraphData(gp);
    free(*coords.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(coords as *mut libc::c_void);
    free(nodes as *mut libc::c_void);
}
unsafe extern "C" fn subset_model(mut G: *mut Agraph_t, mut nG: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ne: libc::c_int = 0;
    let mut Dij: *mut *mut DistType = 0 as *mut *mut DistType;
    let mut gp: *mut vtx_data = 0 as *mut vtx_data;
    gp = makeGraphData(
        G,
        nG,
        &mut ne,
        0 as libc::c_int,
        2 as libc::c_int,
        0 as *mut *mut *mut node_t,
    );
    Dij = compute_apsp_artifical_weights(gp, nG);
    i = 0 as libc::c_int;
    while i < nG {
        j = 0 as libc::c_int;
        while j < nG {
            *(*((*((*(G as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
                .offset(i as isize))
                .offset(
                    j as isize,
                ) = *(*Dij.offset(i as isize)).offset(j as isize) as libc::c_double;
            j += 1;
        }
        i += 1;
    }
    free(*Dij.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(Dij as *mut libc::c_void);
    freeGraphData(gp);
}
unsafe extern "C" fn mds_model(mut g: *mut graph_t) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    v = agfstnode(g);
    while !v.is_null() {
        e = agfstout(g, v);
        while !e.is_null() {
            i = ((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node as *mut Agobj_t))
                .tag)
                .seq() as libc::c_long;
            j = ((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .tag)
                .seq() as libc::c_long;
            if !(i == j) {
                let ref mut fresh41 = *(*((*((*(g as *mut Agobj_t)).data
                    as *mut Agraphinfo_t))
                    .dist)
                    .offset(j as isize))
                    .offset(i as isize);
                *fresh41 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).dist;
                *(*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).dist)
                    .offset(i as isize))
                    .offset(j as isize) = *fresh41;
            }
            e = agnxtout(g, e);
        }
        v = agnxtnode(g, v);
    }
}
unsafe extern "C" fn kkNeato(
    mut g: *mut Agraph_t,
    mut nG: libc::c_int,
    mut model: libc::c_int,
) {
    if model == 2 as libc::c_int {
        subset_model(g, nG);
    } else if model == 1 as libc::c_int {
        if circuit_model(g, nG) == 0 {
            agerr(
                AGWARN,
                b"graph %s is disconnected. Hence, the circuit model\n\0" as *const u8
                    as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
            );
            agerr(
                AGPREV,
                b"is undefined. Reverting to the shortest path model.\n\0" as *const u8
                    as *const libc::c_char,
            );
            agerr(
                AGPREV,
                b"Alternatively, consider running neato using -Gpack=true or decomposing\n\0"
                    as *const u8 as *const libc::c_char,
            );
            agerr(
                AGPREV,
                b"the graph into connected components.\n\0" as *const u8
                    as *const libc::c_char,
            );
            shortest_path(g, nG);
        }
    } else if model == 3 as libc::c_int {
        shortest_path(g, nG);
        mds_model(g);
    } else {
        shortest_path(g, nG);
    }
    initial_positions(g, nG);
    diffeq_model(g, nG);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Solving model %d iterations %d tol %f\n\0" as *const u8
                as *const libc::c_char,
            model,
            MaxIter,
            Epsilon,
        );
        start_timer();
    }
    solve_model(g, nG);
}
unsafe extern "C" fn neatoLayout(
    mut mg: *mut Agraph_t,
    mut g: *mut Agraph_t,
    mut layoutMode: libc::c_int,
    mut layoutModel: libc::c_int,
    mut am: *mut adjust_data,
) {
    let mut nG: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str = agget(
        g as *mut libc::c_void,
        b"maxiter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() {
        MaxIter = atoi(str);
    } else if layoutMode == 1 as libc::c_int {
        MaxIter = 200 as libc::c_int;
    } else if layoutMode == 4 as libc::c_int {
        MaxIter = 30 as libc::c_int;
    } else {
        MaxIter = 100 as libc::c_int * agnnodes(g);
    }
    nG = scan_graph_mode(g, layoutMode);
    if nG < 2 as libc::c_int || MaxIter < 0 as libc::c_int {
        return;
    }
    if layoutMode == 0 as libc::c_int {
        kkNeato(g, nG, layoutModel);
    } else if layoutMode == 4 as libc::c_int {
        sgd(g, layoutModel);
    } else {
        majorization(mg, g, nG, layoutMode, layoutModel, Ndim, am);
    };
}
unsafe extern "C" fn addZ(mut g: *mut Agraph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if Ndim >= 3 as libc::c_int && !N_z.is_null() {
        n = agfstnode(g);
        while !n.is_null() {
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                b"%lf\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_double
                    * *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                        .offset(2 as libc::c_int as isize),
            );
            agxset(n as *mut libc::c_void, N_z, buf.as_mut_ptr());
            n = agnxtnode(g, n);
        }
    }
}
unsafe extern "C" fn addCluster(mut g: *mut graph_t) {
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    subg = agfstsubg(agroot(g as *mut libc::c_void));
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
            add_cluster(g, subg);
            compute_bb(subg);
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn doEdges(mut g: *mut Agraph_t) {
    compute_bb(g);
    spline_edges0(g, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn neato_layout(mut g: *mut Agraph_t) {
    let mut layoutMode: libc::c_int = 0;
    let mut model: libc::c_int = 0;
    let mut mode: pack_mode = l_undef;
    let mut pinfo: pack_info = pack_info {
        aspect: 0.,
        sz: 0,
        margin: 0,
        doSplines: 0,
        mode: l_undef,
        fixed: 0 as *mut bool,
        vals: 0 as *mut packval_t,
        flags: 0,
    };
    let mut am: adjust_data = adjust_data {
        mode: AM_NONE,
        print: 0 as *mut libc::c_char,
        value: 0,
        scaling: 0.,
    };
    let mut save_scale: libc::c_double = PSinputscale;
    if Nop != 0 {
        let mut ret: libc::c_int = 0;
        PSinputscale = 72 as libc::c_int as libc::c_double;
        neato_init_graph(g);
        addZ(g);
        ret = init_nop(g, 1 as libc::c_int);
        if ret < 0 as libc::c_int {
            agerr(
                AGPREV,
                b"as required by the -n flag\n\0" as *const u8 as *const libc::c_char,
            );
            return;
        } else {
            gv_postprocess(g, 0 as libc::c_int);
        }
    } else {
        let mut noTranslate: bool = mapBool(
            agget(
                g as *mut libc::c_void,
                b"notranslate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            0 as libc::c_int != 0,
        );
        PSinputscale = get_inputscale(g);
        neato_init_graph(g);
        layoutMode = neatoMode(g);
        graphAdjustMode(g, &mut am, 0 as *mut libc::c_char);
        model = neatoModel(g);
        mode = getPackModeInfo(g, l_undef, &mut pinfo);
        Pack = getPack(g, -(1 as libc::c_int), 8 as libc::c_int);
        if mode as libc::c_uint == l_undef as libc::c_int as libc::c_uint {
            if Pack < 0 as libc::c_int && layoutMode != 0 {
                Pack = 8 as libc::c_int;
            }
            pinfo.mode = l_node;
        } else if Pack < 0 as libc::c_int {
            Pack = 8 as libc::c_int;
        }
        if Pack >= 0 as libc::c_int {
            let mut gc: *mut graph_t = 0 as *mut graph_t;
            let mut cc: *mut *mut graph_t = 0 as *mut *mut graph_t;
            let mut n_cc: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut pin: bool = false;
            cc = pccomps(g, &mut n_cc, cc_pfx, &mut pin);
            if n_cc > 1 as libc::c_int {
                let mut bp: *mut bool = 0 as *mut bool;
                i = 0 as libc::c_int;
                while i < n_cc {
                    gc = *cc.offset(i as isize);
                    nodeInduce(gc);
                    neatoLayout(g, gc, layoutMode, model, &mut am);
                    removeOverlapWith(gc, &mut am);
                    setEdgeType(gc, (1 as libc::c_int) << 1 as libc::c_int);
                    if noTranslate {
                        doEdges(gc);
                    } else {
                        spline_edges(gc);
                    }
                    i += 1;
                }
                if pin {
                    bp = gcalloc(
                        n_cc as size_t,
                        ::std::mem::size_of::<bool>() as libc::c_ulong,
                    ) as *mut bool;
                    *bp.offset(0 as libc::c_int as isize) = 1 as libc::c_int != 0;
                } else {
                    bp = 0 as *mut bool;
                }
                pinfo.margin = Pack as libc::c_uint;
                pinfo.fixed = bp;
                pinfo.doSplines = 1 as libc::c_int;
                packGraphs(n_cc, cc, g, &mut pinfo);
                free(bp as *mut libc::c_void);
            } else {
                neatoLayout(g, g, layoutMode, model, &mut am);
                removeOverlapWith(g, &mut am);
                if noTranslate {
                    doEdges(g);
                } else {
                    spline_edges(g);
                }
            }
            compute_bb(g);
            addZ(g);
            i = 0 as libc::c_int;
            while i < n_cc {
                gc = *cc.offset(i as isize);
                free_scan_graph(gc);
                agdelrec(
                    gc as *mut libc::c_void,
                    b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
                );
                agdelete(g, gc as *mut libc::c_void);
                i += 1;
            }
            free(cc as *mut libc::c_void);
            addCluster(g);
        } else {
            neatoLayout(g, g, layoutMode, model, &mut am);
            removeOverlapWith(g, &mut am);
            addZ(g);
            if noTranslate {
                doEdges(g);
            } else {
                spline_edges(g);
            }
        }
        gv_postprocess(g, !noTranslate as libc::c_int);
    }
    PSinputscale = save_scale;
}
