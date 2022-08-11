#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
use num_traits::ToPrimitive;
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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabsf(_: libc::c_float) -> libc::c_float;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn drand48() -> libc::c_double;
    fn rand() -> libc::c_int;
    fn fmaxf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    fn free_array(rv: *mut *mut libc::c_double);
    fn new_array(
        i: libc::c_int,
        j: libc::c_int,
        val: libc::c_double,
    ) -> *mut *mut libc::c_double;
    fn solveCircuit(
        nG: libc::c_int,
        Gm: *mut *mut libc::c_double,
        Gm_inv: *mut *mut libc::c_double,
    ) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    static mut Epsilon: libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn dijkstra(_: libc::c_int, _: *mut vtx_data, _: libc::c_int, _: *mut DistType);
    fn dijkstra_f(
        _: libc::c_int,
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut libc::c_float,
    );
    fn dijkstra_bounded(
        _: libc::c_int,
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut DistType,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn mkQueue(_: *mut Queue, _: libc::c_int);
    fn freeQueue(_: *mut Queue);
    fn bfs(
        _: libc::c_int,
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut DistType,
        _: *mut Queue,
    );
    fn bfs_bounded(
        _: libc::c_int,
        _: *mut vtx_data,
        _: libc::c_int,
        _: *mut DistType,
        _: *mut Queue,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn PCA_alloc(
        _: *mut *mut DistType,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_double,
        _: libc::c_int,
    );
    fn iterativePCA_1D(
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_double,
    ) -> bool;
    fn orthog1(n: libc::c_int, vec: *mut libc::c_double);
    fn right_mult_with_vector_transpose(
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
    );
    fn right_mult_with_vector_d(
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
    );
    fn mult_dense_mat(
        _: *mut *mut libc::c_double,
        _: *mut *mut libc::c_float,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        C: *mut *mut *mut libc::c_float,
    );
    fn mult_sparse_dense_mat_transpose(
        _: *mut vtx_data,
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut *mut libc::c_float,
    );
    fn right_mult_with_vector_ff(
        _: *mut libc::c_float,
        _: libc::c_int,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
    );
    fn copy_vectorf(
        n: libc::c_int,
        source: *mut libc::c_float,
        dest: *mut libc::c_float,
    );
    fn vectors_inner_productf(
        n: libc::c_int,
        vector1: *mut libc::c_float,
        vector2: *mut libc::c_float,
    ) -> libc::c_double;
    fn set_vector_valf(n: libc::c_int, val: libc::c_float, result: *mut libc::c_float);
    fn square_vec(n: libc::c_int, vec: *mut libc::c_float);
    fn invert_vec(n: libc::c_int, vec: *mut libc::c_float);
    fn sqrt_vecf(n: libc::c_int, source: *mut libc::c_float, target: *mut libc::c_float);
    fn invert_sqrt_vec(n: libc::c_int, vec: *mut libc::c_float);
    fn conjugate_gradient_f(
        _: *mut *mut libc::c_float,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
        _: bool,
    ) -> libc::c_int;
    fn conjugate_gradient_mkernel(
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: *mut libc::c_float,
        _: libc::c_int,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_int;
    fn embed_graph(
        graph: *mut vtx_data,
        n: libc::c_int,
        dim: libc::c_int,
        _: *mut *mut *mut DistType,
        _: libc::c_int,
    );
    fn center_coordinate(_: *mut *mut DistType, _: libc::c_int, _: libc::c_int);
    fn fill_neighbors_vec_unweighted(
        _: *mut vtx_data,
        vtx: libc::c_int,
        vtx_vec: *mut libc::c_int,
    );
    fn common_neighbors(
        _: *mut vtx_data,
        v: libc::c_int,
        u: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn empty_neighbors_vec(
        graph: *mut vtx_data,
        vtx: libc::c_int,
        vtx_vec: *mut libc::c_int,
    );
    fn distance_kD(
        _: *mut *mut libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn compute_new_weights(graph: *mut vtx_data, n: libc::c_int);
    fn restore_old_weights(
        graph: *mut vtx_data,
        n: libc::c_int,
        old_weights: *mut libc::c_float,
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
pub struct elist {
    pub list: *mut *mut edge_t,
    pub size: libc::c_int,
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
pub struct vtx_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub ewgts: *mut libc::c_float,
    pub eweights: *mut libc::c_float,
    pub edists: *mut libc::c_float,
}
pub type DistType = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Queue {
    pub data: *mut libc::c_int,
    pub queueSize: libc::c_int,
    pub end: libc::c_int,
    pub start: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dist_data {
    pub nedges: libc::c_int,
    pub edges: *mut libc::c_int,
    pub edist: *mut DistType,
    pub free_mem: bool,
}
unsafe extern "C" fn compute_stressf(
    mut coords: *mut *mut libc::c_float,
    mut lap: *mut libc::c_float,
    mut dim: libc::c_int,
    mut n: libc::c_int,
    mut exp: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut Dij: libc::c_double = 0.;
    sum = 0 as libc::c_int as libc::c_double;
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        count += 1;
        j = 1 as libc::c_int;
        while j < n - i {
            dist = 0 as libc::c_int as libc::c_double;
            neighbor = i + j;
            l = 0 as libc::c_int;
            while l < dim {
                dist
                    += ((*(*coords.offset(l as isize)).offset(i as isize)
                        - *(*coords.offset(l as isize)).offset(neighbor as isize))
                        * (*(*coords.offset(l as isize)).offset(i as isize)
                            - *(*coords.offset(l as isize)).offset(neighbor as isize)))
                        as libc::c_double;
                l += 1;
            }
            dist = sqrt(dist);
            if exp == 2 as libc::c_int {
                Dij = 1.0f64 / sqrt(*lap.offset(count as isize) as libc::c_double);
                sum
                    += (Dij - dist) * (Dij - dist)
                        * *lap.offset(count as isize) as libc::c_double;
            } else {
                Dij = 1.0f64 / *lap.offset(count as isize) as libc::c_double;
                sum
                    += (Dij - dist) * (Dij - dist)
                        * *lap.offset(count as isize) as libc::c_double;
            }
            j += 1;
            count += 1;
        }
        i += 1;
    }
    return sum;
}
unsafe extern "C" fn compute_stress1(
    mut coords: *mut *mut libc::c_double,
    mut distances: *mut dist_data,
    mut dim: libc::c_int,
    mut n: libc::c_int,
    mut exp: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut dist: libc::c_double = 0.;
    let mut Dij: libc::c_double = 0.;
    sum = 0 as libc::c_int as libc::c_double;
    if exp == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < n {
            j = 0 as libc::c_int;
            while j < (*distances.offset(i as isize)).nedges {
                node = *((*distances.offset(i as isize)).edges).offset(j as isize);
                if !(node <= i) {
                    dist = 0 as libc::c_int as libc::c_double;
                    l = 0 as libc::c_int;
                    while l < dim {
                        dist
                            += (*(*coords.offset(l as isize)).offset(i as isize)
                                - *(*coords.offset(l as isize)).offset(node as isize))
                                * (*(*coords.offset(l as isize)).offset(i as isize)
                                    - *(*coords.offset(l as isize)).offset(node as isize));
                        l += 1;
                    }
                    dist = sqrt(dist);
                    Dij = *((*distances.offset(i as isize)).edist).offset(j as isize)
                        as libc::c_double;
                    sum += (Dij - dist) * (Dij - dist) / (Dij * Dij);
                }
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            j = 0 as libc::c_int;
            while j < (*distances.offset(i as isize)).nedges {
                node = *((*distances.offset(i as isize)).edges).offset(j as isize);
                if !(node <= i) {
                    dist = 0 as libc::c_int as libc::c_double;
                    l = 0 as libc::c_int;
                    while l < dim {
                        dist
                            += (*(*coords.offset(l as isize)).offset(i as isize)
                                - *(*coords.offset(l as isize)).offset(node as isize))
                                * (*(*coords.offset(l as isize)).offset(i as isize)
                                    - *(*coords.offset(l as isize)).offset(node as isize));
                        l += 1;
                    }
                    dist = sqrt(dist);
                    Dij = *((*distances.offset(i as isize)).edist).offset(j as isize)
                        as libc::c_double;
                    sum += (Dij - dist) * (Dij - dist) / Dij;
                }
                j += 1;
            }
            i += 1;
        }
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn initLayout(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut coords: *mut *mut libc::c_double,
    mut nodes: *mut *mut node_t,
) -> libc::c_int {
    let mut np: *mut node_t = 0 as *mut node_t;
    let mut xp: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut yp: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pt: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut pinned: libc::c_int = 0 as libc::c_int;
    xp = *coords.offset(0 as libc::c_int as isize);
    yp = *coords.offset(1 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < n {
        np = *nodes.offset(i as isize);
        if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned as libc::c_int
            > 0 as libc::c_int
        {
            pt = (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos;
            let fresh0 = pt;
            pt = pt.offset(1);
            let fresh1 = xp;
            xp = xp.offset(1);
            *fresh1 = *fresh0;
            let fresh2 = pt;
            pt = pt.offset(1);
            let fresh3 = yp;
            yp = yp.offset(1);
            *fresh3 = *fresh2;
            if dim > 2 as libc::c_int {
                d = 2 as libc::c_int;
                while d < dim {
                    let fresh4 = pt;
                    pt = pt.offset(1);
                    *(*coords.offset(d as isize)).offset(i as isize) = *fresh4;
                    d += 1;
                }
            }
            if (*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned
                as libc::c_int > 1 as libc::c_int
            {
                pinned = 1 as libc::c_int;
            }
        } else {
            let fresh5 = xp;
            xp = xp.offset(1);
            *fresh5 = drand48();
            let fresh6 = yp;
            yp = yp.offset(1);
            *fresh6 = drand48();
            if dim > 2 as libc::c_int {
                d = 2 as libc::c_int;
                while d < dim {
                    *(*coords.offset(d as isize)).offset(i as isize) = drand48();
                    d += 1;
                }
            }
        }
        i += 1;
    }
    d = 0 as libc::c_int;
    while d < dim {
        orthog1(n, *coords.offset(d as isize));
        d += 1;
    }
    return pinned;
}
#[no_mangle]
pub unsafe extern "C" fn circuitModel(
    mut graph: *mut vtx_data,
    mut nG: libc::c_int,
) -> *mut libc::c_float {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut Dij: *mut libc::c_float = gcalloc(
        (nG * (nG + 1 as libc::c_int) / 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut Gm: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut Gm_inv: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    Gm = new_array(nG, nG, 0.0f64);
    Gm_inv = new_array(nG, nG, 0.0f64);
    if !((*graph).ewgts).is_null() {
        i = 0 as libc::c_int;
        while i < nG {
            e = 1 as libc::c_int;
            while e < (*graph.offset(i as isize)).nedges {
                j = *((*graph.offset(i as isize)).edges).offset(e as isize);
                let ref mut fresh7 = *(*Gm.offset(j as isize)).offset(i as isize);
                *fresh7 = -1.0f64
                    / *((*graph.offset(i as isize)).ewgts).offset(e as isize)
                        as libc::c_double;
                *(*Gm.offset(i as isize)).offset(j as isize) = *fresh7;
                e += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < nG {
            e = 1 as libc::c_int;
            while e < (*graph.offset(i as isize)).nedges {
                j = *((*graph.offset(i as isize)).edges).offset(e as isize);
                let ref mut fresh8 = *(*Gm.offset(j as isize)).offset(i as isize);
                *fresh8 = -1.0f64;
                *(*Gm.offset(i as isize)).offset(j as isize) = *fresh8;
                e += 1;
            }
            i += 1;
        }
    }
    rv = solveCircuit(nG, Gm, Gm_inv);
    if rv != 0 {
        let mut v: libc::c_float = 0.;
        count = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nG {
            j = i;
            while j < nG {
                if i == j {
                    v = 0.0f64 as libc::c_float;
                } else {
                    v = (*(*Gm_inv.offset(i as isize)).offset(i as isize)
                        + *(*Gm_inv.offset(j as isize)).offset(j as isize)
                        - 2.0f64 * *(*Gm_inv.offset(i as isize)).offset(j as isize))
                        as libc::c_float;
                }
                let fresh9 = count;
                count = count + 1;
                *Dij.offset(fresh9 as isize) = v;
                j += 1;
            }
            i += 1;
        }
    } else {
        free(Dij as *mut libc::c_void);
        Dij = 0 as *mut libc::c_float;
    }
    free_array(Gm);
    free_array(Gm_inv);
    return Dij;
}
unsafe extern "C" fn sparse_stress_subspace_majorization_kD(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut coords: *mut *mut libc::c_double,
    mut dim: libc::c_int,
    mut smart_ini: libc::c_int,
    mut exp: libc::c_int,
    mut reweight_graph: libc::c_int,
    mut n_iterations: libc::c_int,
    mut dist_bound: libc::c_int,
    mut num_centers: libc::c_int,
) -> libc::c_int {
    let mut iterations: libc::c_int = 0;
    let mut conj_tol: libc::c_double = 1e-3f64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut subspace_dim: libc::c_int = if (50 as libc::c_int) < n {
        50 as libc::c_int
    } else {
        n
    };
    let mut subspace: *mut *mut libc::c_double = gcalloc(
        subspace_dim as size_t,
        ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut libc::c_double;
    let mut d_storage: *mut libc::c_double = gcalloc(
        (subspace_dim * n) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut num_centers_local: libc::c_int = 0;
    let mut full_coords: *mut *mut DistType = 0 as *mut *mut DistType;
    let mut CenterIndex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut invCenterIndex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Q: Queue = Queue {
        data: 0 as *mut libc::c_int,
        queueSize: 0,
        end: 0,
        start: 0,
    };
    let mut old_weights: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut Dij: *mut *mut DistType = 0 as *mut *mut DistType;
    let mut dist: *mut DistType = 0 as *mut DistType;
    let mut max_dist: DistType = 0;
    let mut storage: *mut DistType = 0 as *mut DistType;
    let mut visited_nodes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut distances: *mut dist_data = 0 as *mut dist_data;
    let mut available_space: libc::c_int = 0;
    let mut storage1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut storage2: *mut DistType = 0 as *mut DistType;
    let mut num_visited_nodes: libc::c_int = 0;
    let mut num_neighbors: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    let mut dist_list: *mut DistType = 0 as *mut DistType;
    let mut lap: *mut vtx_data = 0 as *mut vtx_data;
    let mut edges: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ewgts: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut degree: libc::c_double = 0.;
    let mut directions: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut tmp_mat: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut matrix: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut dist_ij: libc::c_double = 0.;
    let mut b: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut b_restricted: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut L_ij: libc::c_double = 0.;
    let mut old_stress: libc::c_double = 0.;
    let mut new_stress: libc::c_double = 0.;
    let mut converged: bool = false;
    i = 0 as libc::c_int;
    while i < subspace_dim {
        let ref mut fresh10 = *subspace.offset(i as isize);
        *fresh10 = d_storage.offset((i * n) as isize);
        i += 1;
    }
    num_centers_local = if n
        < (if 2 as libc::c_int * subspace_dim > 50 as libc::c_int {
            2 as libc::c_int * subspace_dim
        } else {
            50 as libc::c_int
        })
    {
        n
    } else if 2 as libc::c_int * subspace_dim > 50 as libc::c_int {
        2 as libc::c_int * subspace_dim
    } else {
        50 as libc::c_int
    };
    full_coords = 0 as *mut *mut DistType;
    embed_graph(graph, n, num_centers_local, &mut full_coords, reweight_graph);
    center_coordinate(full_coords, n, num_centers_local);
    PCA_alloc(full_coords, num_centers_local, n, subspace, subspace_dim);
    free(*full_coords.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(full_coords as *mut libc::c_void);
    CenterIndex = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *CenterIndex.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    invCenterIndex = 0 as *mut libc::c_int;
    mkQueue(&mut Q, n);
    old_weights = (*graph.offset(0 as libc::c_int as isize)).ewgts;
    if reweight_graph != 0 {
        compute_new_weights(graph, n);
    }
    Dij = 0 as *mut *mut DistType;
    dist = gcalloc(n as size_t, ::std::mem::size_of::<DistType>() as libc::c_ulong)
        as *mut DistType;
    if !(num_centers == 0 as libc::c_int) {
        invCenterIndex = gcalloc(
            num_centers as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        storage = gcalloc(
            (n * num_centers) as size_t,
            ::std::mem::size_of::<DistType>() as libc::c_ulong,
        ) as *mut DistType;
        Dij = gcalloc(
            num_centers as size_t,
            ::std::mem::size_of::<*mut DistType>() as libc::c_ulong,
        ) as *mut *mut DistType;
        i = 0 as libc::c_int;
        while i < num_centers {
            let ref mut fresh11 = *Dij.offset(i as isize);
            *fresh11 = storage.offset((i * n) as isize);
            i += 1;
        }
        node = rand() % n;
        *CenterIndex.offset(node as isize) = 0 as libc::c_int;
        *invCenterIndex.offset(0 as libc::c_int as isize) = node;
        if reweight_graph != 0 {
            dijkstra(node, graph, n, *Dij.offset(0 as libc::c_int as isize));
        } else {
            bfs(node, graph, n, *Dij.offset(0 as libc::c_int as isize), &mut Q);
        }
        max_dist = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < n {
            *dist
                .offset(
                    i as isize,
                ) = *(*Dij.offset(0 as libc::c_int as isize)).offset(i as isize);
            if *dist.offset(i as isize) > max_dist {
                node = i;
                max_dist = *dist.offset(i as isize);
            }
            i += 1;
        }
        i = 1 as libc::c_int;
        while i < num_centers {
            *CenterIndex.offset(node as isize) = i;
            *invCenterIndex.offset(i as isize) = node;
            if reweight_graph != 0 {
                dijkstra(node, graph, n, *Dij.offset(i as isize));
            } else {
                bfs(node, graph, n, *Dij.offset(i as isize), &mut Q);
            }
            max_dist = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < n {
                *dist
                    .offset(
                        j as isize,
                    ) = if *dist.offset(j as isize)
                    < *(*Dij.offset(i as isize)).offset(j as isize)
                {
                    *dist.offset(j as isize)
                } else {
                    *(*Dij.offset(i as isize)).offset(j as isize)
                };
                if *dist.offset(j as isize) > max_dist
                    || *dist.offset(j as isize) == max_dist
                        && rand() % (j + 1 as libc::c_int) == 0 as libc::c_int
                {
                    node = j;
                    max_dist = *dist.offset(j as isize);
                }
                j += 1;
            }
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < n {
        *dist.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    visited_nodes = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    distances = gcalloc(n as size_t, ::std::mem::size_of::<dist_data>() as libc::c_ulong)
        as *mut dist_data;
    available_space = 0 as libc::c_int;
    nedges = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if *CenterIndex.offset(i as isize) >= 0 as libc::c_int {
            let ref mut fresh12 = (*distances.offset(i as isize)).edges;
            *fresh12 = gcalloc(
                (n - 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            let ref mut fresh13 = (*distances.offset(i as isize)).edist;
            *fresh13 = gcalloc(
                (n - 1 as libc::c_int) as size_t,
                ::std::mem::size_of::<DistType>() as libc::c_ulong,
            ) as *mut DistType;
            (*distances.offset(i as isize)).nedges = n - 1 as libc::c_int;
            nedges += n - 1 as libc::c_int;
            (*distances.offset(i as isize)).free_mem = 1 as libc::c_int != 0;
            index = *CenterIndex.offset(i as isize);
            j = 0 as libc::c_int;
            while j < i {
                *((*distances.offset(i as isize)).edges).offset(j as isize) = j;
                *((*distances.offset(i as isize)).edist)
                    .offset(
                        j as isize,
                    ) = *(*Dij.offset(index as isize)).offset(j as isize);
                j += 1;
            }
            j = i + 1 as libc::c_int;
            while j < n {
                *((*distances.offset(i as isize)).edges)
                    .offset((j - 1 as libc::c_int) as isize) = j;
                *((*distances.offset(i as isize)).edist)
                    .offset(
                        (j - 1 as libc::c_int) as isize,
                    ) = *(*Dij.offset(index as isize)).offset(j as isize);
                j += 1;
            }
        } else {
            if dist_bound > 0 as libc::c_int {
                if reweight_graph != 0 {
                    num_visited_nodes = dijkstra_bounded(
                        i,
                        graph,
                        n,
                        dist,
                        dist_bound,
                        visited_nodes,
                    );
                } else {
                    num_visited_nodes = bfs_bounded(
                        i,
                        graph,
                        n,
                        dist,
                        &mut Q,
                        dist_bound,
                        visited_nodes,
                    );
                }
                j = 0 as libc::c_int;
                while j < num_visited_nodes {
                    if *CenterIndex.offset(*visited_nodes.offset(j as isize) as isize)
                        < 0 as libc::c_int && *visited_nodes.offset(j as isize) != i
                    {
                        j += 1;
                    } else {
                        *dist
                            .offset(
                                *visited_nodes.offset(j as isize) as isize,
                            ) = -(1 as libc::c_int);
                        num_visited_nodes -= 1;
                        *visited_nodes
                            .offset(
                                j as isize,
                            ) = *visited_nodes.offset(num_visited_nodes as isize);
                    }
                }
            } else {
                num_visited_nodes = 0 as libc::c_int;
            }
            num_neighbors = num_visited_nodes + num_centers;
            if num_neighbors > available_space {
                available_space = (dist_bound + 1 as libc::c_int) * n;
                storage1 = gcalloc(
                    available_space as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                storage2 = gcalloc(
                    available_space as size_t,
                    ::std::mem::size_of::<DistType>() as libc::c_ulong,
                ) as *mut DistType;
                (*distances.offset(i as isize)).free_mem = 1 as libc::c_int != 0;
            } else {
                (*distances.offset(i as isize)).free_mem = 0 as libc::c_int != 0;
            }
            let ref mut fresh14 = (*distances.offset(i as isize)).edges;
            *fresh14 = storage1;
            let ref mut fresh15 = (*distances.offset(i as isize)).edist;
            *fresh15 = storage2;
            (*distances.offset(i as isize)).nedges = num_neighbors;
            nedges += num_neighbors;
            j = 0 as libc::c_int;
            while j < num_visited_nodes {
                *storage1.offset(j as isize) = *visited_nodes.offset(j as isize);
                *storage2
                    .offset(
                        j as isize,
                    ) = *dist.offset(*visited_nodes.offset(j as isize) as isize);
                *dist
                    .offset(
                        *visited_nodes.offset(j as isize) as isize,
                    ) = -(1 as libc::c_int);
                j += 1;
            }
            j = num_visited_nodes;
            while j < num_neighbors {
                index = j - num_visited_nodes;
                *storage1.offset(j as isize) = *invCenterIndex.offset(index as isize);
                *storage2
                    .offset(
                        j as isize,
                    ) = *(*Dij.offset(index as isize)).offset(i as isize);
                j += 1;
            }
            storage1 = storage1.offset(num_neighbors as isize);
            storage2 = storage2.offset(num_neighbors as isize);
            available_space -= num_neighbors;
        }
        i += 1;
    }
    free(dist as *mut libc::c_void);
    free(visited_nodes as *mut libc::c_void);
    if !Dij.is_null() {
        free(*Dij.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(Dij as *mut libc::c_void);
    }
    lap = gcalloc(n as size_t, ::std::mem::size_of::<vtx_data>() as libc::c_ulong)
        as *mut vtx_data;
    edges = gcalloc(
        (nedges + n) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    ewgts = gcalloc(
        (nedges + n) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh16 = (*lap.offset(i as isize)).edges;
        *fresh16 = edges;
        let ref mut fresh17 = (*lap.offset(i as isize)).ewgts;
        *fresh17 = ewgts;
        (*lap.offset(i as isize))
            .nedges = (*distances.offset(i as isize)).nedges + 1 as libc::c_int;
        dist_list = ((*distances.offset(i as isize)).edist)
            .offset(-(1 as libc::c_int as isize));
        degree = 0 as libc::c_int as libc::c_double;
        if exp == 2 as libc::c_int {
            j = 1 as libc::c_int;
            while j < (*lap.offset(i as isize)).nedges {
                *edges
                    .offset(
                        j as isize,
                    ) = *((*distances.offset(i as isize)).edges)
                    .offset((j - 1 as libc::c_int) as isize);
                *ewgts
                    .offset(
                        j as isize,
                    ) = -1.0f64 as libc::c_float
                    / (*dist_list.offset(j as isize) as libc::c_float
                        * *dist_list.offset(j as isize) as libc::c_float);
                degree -= *ewgts.offset(j as isize) as libc::c_double;
                j += 1;
            }
        } else {
            j = 1 as libc::c_int;
            while j < (*lap.offset(i as isize)).nedges {
                *edges
                    .offset(
                        j as isize,
                    ) = *((*distances.offset(i as isize)).edges)
                    .offset((j - 1 as libc::c_int) as isize);
                *ewgts
                    .offset(
                        j as isize,
                    ) = -1.0f32 / *dist_list.offset(j as isize) as libc::c_float;
                degree -= *ewgts.offset(j as isize) as libc::c_double;
                j += 1;
            }
        }
        *edges.offset(0 as libc::c_int as isize) = i;
        *ewgts.offset(0 as libc::c_int as isize) = degree as libc::c_float;
        edges = edges.offset((*lap.offset(i as isize)).nedges as isize);
        ewgts = ewgts.offset((*lap.offset(i as isize)).nedges as isize);
        i += 1;
    }
    directions = gcalloc(
        dim as size_t,
        ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
    ) as *mut *mut libc::c_double;
    let ref mut fresh18 = *directions.offset(0 as libc::c_int as isize);
    *fresh18 = gcalloc(
        (dim * subspace_dim) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i < dim {
        let ref mut fresh19 = *directions.offset(i as isize);
        *fresh19 = (*directions.offset(0 as libc::c_int as isize))
            .offset((i * subspace_dim) as isize);
        i += 1;
    }
    if smart_ini != 0 {
        k = 0 as libc::c_int;
        while k < dim {
            i = 0 as libc::c_int;
            while i < subspace_dim {
                *(*directions.offset(k as isize))
                    .offset(i as isize) = 0 as libc::c_int as libc::c_double;
                i += 1;
            }
            k += 1;
        }
        if dim != 2 as libc::c_int {
            k = 0 as libc::c_int;
            while k < dim {
                *(*directions.offset(k as isize))
                    .offset(k as isize) = 1 as libc::c_int as libc::c_double;
                k += 1;
            }
        } else {
            *(*directions.offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
            if !iterativePCA_1D(
                subspace,
                subspace_dim,
                n,
                *directions.offset(1 as libc::c_int as isize),
            ) {
                k = 0 as libc::c_int;
                while k < subspace_dim {
                    *(*directions.offset(1 as libc::c_int as isize))
                        .offset(k as isize) = 0 as libc::c_int as libc::c_double;
                    k += 1;
                }
                *(*directions.offset(1 as libc::c_int as isize))
                    .offset(
                        1 as libc::c_int as isize,
                    ) = 1 as libc::c_int as libc::c_double;
            }
        }
    } else {
        k = 0 as libc::c_int;
        while k < dim {
            i = 0 as libc::c_int;
            while i < subspace_dim {
                *(*directions.offset(k as isize))
                    .offset(
                        i as isize,
                    ) = rand() as libc::c_double
                    / 2147483647 as libc::c_int as libc::c_double;
                i += 1;
            }
            k += 1;
        }
    }
    k = 0 as libc::c_int;
    while k < dim {
        right_mult_with_vector_transpose(
            subspace,
            n,
            subspace_dim,
            *directions.offset(k as isize),
            *coords.offset(k as isize),
        );
        k += 1;
    }
    tmp_mat = 0 as *mut *mut libc::c_float;
    matrix = 0 as *mut *mut libc::c_float;
    mult_sparse_dense_mat_transpose(lap, subspace, n, subspace_dim, &mut tmp_mat);
    mult_dense_mat(subspace, tmp_mat, subspace_dim, n, subspace_dim, &mut matrix);
    free(*tmp_mat.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(tmp_mat as *mut libc::c_void);
    b = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as *mut libc::c_double;
    b_restricted = gcalloc(
        subspace_dim as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    old_stress = compute_stress1(coords, distances, dim, n, exp);
    converged = 0 as libc::c_int != 0;
    iterations = 0 as libc::c_int;
    's_1175: while iterations < n_iterations && !converged {
        k = 0 as libc::c_int;
        while k < dim {
            i = 0 as libc::c_int;
            while i < n {
                degree = 0 as libc::c_int as libc::c_double;
                *b.offset(i as isize) = 0 as libc::c_int as libc::c_double;
                dist_list = ((*distances.offset(i as isize)).edist)
                    .offset(-(1 as libc::c_int as isize));
                edges = (*lap.offset(i as isize)).edges;
                ewgts = (*lap.offset(i as isize)).ewgts;
                j = 1 as libc::c_int;
                while j < (*lap.offset(i as isize)).nedges {
                    node = *edges.offset(j as isize);
                    dist_ij = distance_kD(coords, dim, i, node);
                    if dist_ij > 1e-30f64 {
                        L_ij = (-*ewgts.offset(j as isize)
                            * *dist_list.offset(j as isize) as libc::c_float)
                            as libc::c_double / dist_ij;
                        degree -= L_ij;
                        *b.offset(i as isize)
                            += L_ij
                                * *(*coords.offset(k as isize)).offset(node as isize);
                    }
                    j += 1;
                }
                *b.offset(i as isize)
                    += degree * *(*coords.offset(k as isize)).offset(i as isize);
                i += 1;
            }
            right_mult_with_vector_d(subspace, subspace_dim, n, b, b_restricted);
            if conjugate_gradient_f(
                matrix,
                *directions.offset(k as isize),
                b_restricted,
                subspace_dim,
                conj_tol,
                subspace_dim,
                0 as libc::c_int != 0,
            ) != 0
            {
                iterations = -(1 as libc::c_int);
                break 's_1175;
            } else {
                right_mult_with_vector_transpose(
                    subspace,
                    n,
                    subspace_dim,
                    *directions.offset(k as isize),
                    *coords.offset(k as isize),
                );
                k += 1;
            }
        }
        converged = iterations % 2 as libc::c_int == 0 as libc::c_int;
        if converged {
            new_stress = compute_stress1(coords, distances, dim, n, exp);
            converged = fabs(new_stress - old_stress) / (new_stress + 1e-10f64)
                < Epsilon;
            old_stress = new_stress;
        }
        iterations += 1;
    }
    free(b_restricted as *mut libc::c_void);
    free(b as *mut libc::c_void);
    if reweight_graph != 0 {
        restore_old_weights(graph, n, old_weights);
    }
    i = 0 as libc::c_int;
    while i < n {
        if (*distances.offset(i as isize)).free_mem {
            free((*distances.offset(i as isize)).edges as *mut libc::c_void);
            free((*distances.offset(i as isize)).edist as *mut libc::c_void);
        }
        i += 1;
    }
    free(distances as *mut libc::c_void);
    free((*lap.offset(0 as libc::c_int as isize)).edges as *mut libc::c_void);
    free((*lap.offset(0 as libc::c_int as isize)).ewgts as *mut libc::c_void);
    free(lap as *mut libc::c_void);
    free(CenterIndex as *mut libc::c_void);
    free(invCenterIndex as *mut libc::c_void);
    free(*directions.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(directions as *mut libc::c_void);
    if !matrix.is_null() {
        free(*matrix.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(matrix as *mut libc::c_void);
    }
    free(*subspace.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    free(subspace as *mut libc::c_void);
    freeQueue(&mut Q);
    return iterations;
}
unsafe extern "C" fn compute_weighted_apsp_packed(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
) -> *mut libc::c_float {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut Dij: *mut libc::c_float = gcalloc(
        (n * (n + 1 as libc::c_int) / 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut Di: *mut libc::c_float = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut Q: Queue = Queue {
        data: 0 as *mut libc::c_int,
        queueSize: 0,
        end: 0,
        start: 0,
    };
    mkQueue(&mut Q, n);
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        dijkstra_f(i, graph, n, Di);
        j = i;
        while j < n {
            let fresh20 = count;
            count = count + 1;
            *Dij.offset(fresh20 as isize) = *Di.offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    free(Di as *mut libc::c_void);
    freeQueue(&mut Q);
    return Dij;
}
#[no_mangle]
pub unsafe extern "C" fn mdsModel(
    mut graph: *mut vtx_data,
    mut nG: libc::c_int,
) -> *mut libc::c_float {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut Dij: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut shift: libc::c_int = 0 as libc::c_int;
    let mut delta: libc::c_double = 0.0f64;
    if ((*graph).ewgts).is_null() {
        return 0 as *mut libc::c_float;
    }
    Dij = compute_weighted_apsp_packed(graph, nG);
    i = 0 as libc::c_int;
    while i < nG {
        shift += i;
        e = 1 as libc::c_int;
        while e < (*graph.offset(i as isize)).nedges {
            j = *((*graph.offset(i as isize)).edges).offset(e as isize);
            if !(j < i) {
                delta
                    += fabsf(
                        *Dij.offset((i * nG + j - shift) as isize)
                            - *((*graph.offset(i as isize)).ewgts).offset(e as isize),
                    ) as libc::c_double;
                *Dij
                    .offset(
                        (i * nG + j - shift) as isize,
                    ) = *((*graph.offset(i as isize)).ewgts).offset(e as isize);
            }
            e += 1;
        }
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"mdsModel: delta = %f\n\0" as *const u8 as *const libc::c_char,
            delta,
        );
    }
    return Dij;
}
#[no_mangle]
pub unsafe extern "C" fn compute_apsp_packed(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
) -> *mut libc::c_float {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut Dij: *mut libc::c_float = gcalloc(
        (n * (n + 1 as libc::c_int) / 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut Di: *mut DistType = gcalloc(
        n as size_t,
        ::std::mem::size_of::<DistType>() as libc::c_ulong,
    ) as *mut DistType;
    let mut Q: Queue = Queue {
        data: 0 as *mut libc::c_int,
        queueSize: 0,
        end: 0,
        start: 0,
    };
    mkQueue(&mut Q, n);
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        bfs(i, graph, n, Di, &mut Q);
        j = i;
        while j < n {
            let fresh21 = count;
            count = count + 1;
            *Dij.offset(fresh21 as isize) = *Di.offset(j as isize) as libc::c_float;
            j += 1;
        }
        i += 1;
    }
    free(Di as *mut libc::c_void);
    freeQueue(&mut Q);
    return Dij;
}
#[no_mangle]
pub unsafe extern "C" fn compute_apsp_artifical_weights_packed(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
) -> *mut libc::c_float {
    let mut Dij: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut old_weights: *mut libc::c_float = (*graph.offset(0 as libc::c_int as isize))
        .ewgts;
    let mut nedges: libc::c_int = 0 as libc::c_int;
    let mut weights: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut vtx_vec: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut deg_i: libc::c_int = 0;
    let mut deg_j: libc::c_int = 0;
    let mut neighbor: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        nedges += (*graph.offset(i as isize)).nedges;
        i += 1;
    }
    weights = gcalloc(
        nedges as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    vtx_vec = gcalloc(n as size_t, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *vtx_vec.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    if !((*graph).ewgts).is_null() {
        i = 0 as libc::c_int;
        while i < n {
            fill_neighbors_vec_unweighted(graph, i, vtx_vec);
            deg_i = (*graph.offset(i as isize)).nedges - 1 as libc::c_int;
            j = 1 as libc::c_int;
            while j <= deg_i {
                neighbor = *((*graph.offset(i as isize)).edges).offset(j as isize);
                deg_j = (*graph.offset(neighbor as isize)).nedges - 1 as libc::c_int;
                *weights
                    .offset(
                        j as isize,
                    ) = fmaxf(
                    (deg_i + deg_j
                        - 2 as libc::c_int
                            * common_neighbors(graph, i, neighbor, vtx_vec))
                        as libc::c_float,
                    *((*graph.offset(i as isize)).ewgts).offset(j as isize),
                );
                j += 1;
            }
            empty_neighbors_vec(graph, i, vtx_vec);
            let ref mut fresh22 = (*graph.offset(i as isize)).ewgts;
            *fresh22 = weights;
            weights = weights.offset((*graph.offset(i as isize)).nedges as isize);
            i += 1;
        }
        Dij = compute_weighted_apsp_packed(graph, n);
    } else {
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh23 = (*graph.offset(i as isize)).ewgts;
            *fresh23 = weights;
            fill_neighbors_vec_unweighted(graph, i, vtx_vec);
            deg_i = (*graph.offset(i as isize)).nedges - 1 as libc::c_int;
            j = 1 as libc::c_int;
            while j <= deg_i {
                neighbor = *((*graph.offset(i as isize)).edges).offset(j as isize);
                deg_j = (*graph.offset(neighbor as isize)).nedges - 1 as libc::c_int;
                *weights
                    .offset(
                        j as isize,
                    ) = deg_i as libc::c_float + deg_j as libc::c_float
                    - (2 as libc::c_int * common_neighbors(graph, i, neighbor, vtx_vec))
                        as libc::c_float;
                j += 1;
            }
            empty_neighbors_vec(graph, i, vtx_vec);
            weights = weights.offset((*graph.offset(i as isize)).nedges as isize);
            i += 1;
        }
        Dij = compute_apsp_packed(graph, n);
    }
    free(vtx_vec as *mut libc::c_void);
    free((*graph.offset(0 as libc::c_int as isize)).ewgts as *mut libc::c_void);
    let ref mut fresh24 = (*graph.offset(0 as libc::c_int as isize)).ewgts;
    *fresh24 = 0 as *mut libc::c_float;
    if !old_weights.is_null() {
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh25 = (*graph.offset(i as isize)).ewgts;
            *fresh25 = old_weights;
            old_weights = old_weights
                .offset((*graph.offset(i as isize)).nedges as isize);
            i += 1;
        }
    }
    return Dij;
}
#[no_mangle]
pub unsafe extern "C" fn stress_majorization_kD_mkernel(
    mut graph: *mut vtx_data,
    mut n: libc::c_int,
    mut d_coords: *mut *mut libc::c_double,
    mut nodes: *mut *mut node_t,
    mut dim: libc::c_int,
    mut opts: libc::c_int,
    mut model: libc::c_int,
    mut maxi: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iterations: libc::c_int = 0;
    let mut conj_tol: libc::c_double = 1e-3f64;
    let mut Dij: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut coords: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut f_storage: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut constant_term: libc::c_float = 0.;
    let mut count: libc::c_int = 0;
    let mut degree: f128::f128 = f128::f128::ZERO;
    let mut lap_length: libc::c_int = 0;
    let mut lap2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut degrees: *mut f128::f128 = 0 as *mut f128::f128;
    let mut step: libc::c_int = 0;
    let mut val: libc::c_float = 0.;
    let mut old_stress: libc::c_double = 0.;
    let mut new_stress: libc::c_double = 0.;
    let mut converged: bool = false;
    let mut b: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut tmp_coords: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dist_accumulator: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut lap1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut smart_ini: libc::c_int = opts & 0x4 as libc::c_int;
    let mut exp: libc::c_int = opts & 0x3 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut havePinned: libc::c_int = 0;
    if maxi < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if Verbose != 0 {
        start_timer();
    }
    if model == 2 as libc::c_int {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Calculating subset model\0" as *const u8 as *const libc::c_char,
            );
        }
        Dij = compute_apsp_artifical_weights_packed(graph, n);
    } else if model == 1 as libc::c_int {
        Dij = circuitModel(graph, n);
        if Dij.is_null() {
            agerr(
                AGWARN,
                b"graph is disconnected. Hence, the circuit model\n\0" as *const u8
                    as *const libc::c_char,
            );
            agerr(
                AGPREV,
                b"is undefined. Reverting to the shortest path model.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if model == 3 as libc::c_int {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Calculating MDS model\0" as *const u8 as *const libc::c_char,
            );
        }
        Dij = mdsModel(graph, n);
    }
    if Dij.is_null() {
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Calculating shortest paths\0" as *const u8 as *const libc::c_char,
            );
        }
        if !((*graph).ewgts).is_null() {
            Dij = compute_weighted_apsp_packed(graph, n);
        } else {
            Dij = compute_apsp_packed(graph, n);
        }
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b": %.2f sec\n\0" as *const u8 as *const libc::c_char,
            elapsed_sec(),
        );
        fprintf(
            stderr,
            b"Setting initial positions\0" as *const u8 as *const libc::c_char,
        );
        start_timer();
    }
    if smart_ini != 0 && n > 1 as libc::c_int {
        havePinned = 0 as libc::c_int;
        if sparse_stress_subspace_majorization_kD(
            graph,
            n,
            d_coords,
            dim,
            smart_ini,
            exp,
            (model == 2 as libc::c_int) as libc::c_int,
            50 as libc::c_int,
            0 as libc::c_int,
            40 as libc::c_int,
        ) < 0 as libc::c_int
        {
            iterations = -(1 as libc::c_int);
            current_block = 15300195249473174512;
        } else {
            i = 0 as libc::c_int;
            while i < dim {
                let mut max: libc::c_double = 1 as libc::c_int as libc::c_double;
                j = 0 as libc::c_int;
                while j < n {
                    if fabs(*(*d_coords.offset(i as isize)).offset(j as isize)) > max {
                        max = fabs(*(*d_coords.offset(i as isize)).offset(j as isize));
                    }
                    j += 1;
                }
                j = 0 as libc::c_int;
                while j < n {
                    *(*d_coords.offset(i as isize)).offset(j as isize) /= max;
                    j += 1;
                }
                j = 0 as libc::c_int;
                while j < n {
                    *(*d_coords.offset(i as isize)).offset(j as isize)
                        += 1e-6f64 * (drand48() - 0.5f64);
                    j += 1;
                }
                orthog1(n, *d_coords.offset(i as isize));
                i += 1;
            }
            current_block = 7189308829251266000;
        }
    } else {
        havePinned = initLayout(graph, n, dim, d_coords, nodes);
        current_block = 7189308829251266000;
    }
    match current_block {
        7189308829251266000 => {
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b": %.2f sec\0" as *const u8 as *const libc::c_char,
                    elapsed_sec(),
                );
            }
            if n == 1 as libc::c_int || maxi == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b": %.2f sec\n\0" as *const u8 as *const libc::c_char,
                    elapsed_sec(),
                );
                fprintf(
                    stderr,
                    b"Setting up stress function\0" as *const u8 as *const libc::c_char,
                );
                start_timer();
            }
            coords = gcalloc(
                dim as size_t,
                ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
            ) as *mut *mut libc::c_float;
            f_storage = gcalloc(
                (dim * n) as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            i = 0 as libc::c_int;
            while i < dim {
                let ref mut fresh26 = *coords.offset(i as isize);
                *fresh26 = f_storage.offset((i * n) as isize);
                j = 0 as libc::c_int;
                while j < n {
                    *(*coords.offset(i as isize))
                        .offset(
                            j as isize,
                        ) = *(*d_coords.offset(i as isize)).offset(j as isize)
                        as libc::c_float;
                    j += 1;
                }
                i += 1;
            }
            if exp != 0 {
                constant_term = n as libc::c_float
                    * (n - 1 as libc::c_int) as libc::c_float
                    / 2 as libc::c_int as libc::c_float;
            } else {
                constant_term = 0 as libc::c_int as libc::c_float;
                count = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n - 1 as libc::c_int {
                    count += 1;
                    j = 1 as libc::c_int;
                    while j < n - i {
                        constant_term += *Dij.offset(count as isize);
                        j += 1;
                        count += 1;
                    }
                    i += 1;
                }
            }
            lap_length = n * (n + 1 as libc::c_int) / 2 as libc::c_int;
            lap2 = Dij;
            if exp == 2 as libc::c_int {
                square_vec(lap_length, lap2);
            }
            invert_vec(lap_length, lap2);
            count = 0 as libc::c_int;
            degrees = gcalloc(
                n as size_t,
                ::std::mem::size_of::<f128::f128>() as libc::c_ulong,
            ) as *mut f128::f128;
            memset(
                degrees as *mut libc::c_void,
                0 as libc::c_int,
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f128::f128>() as libc::c_ulong),
            );
            i = 0 as libc::c_int;
            while i < n - 1 as libc::c_int {
                degree = f128::f128::new(0 as libc::c_int);
                count += 1;
                j = 1 as libc::c_int;
                while j < n - i {
                    val = *lap2.offset(count as isize);
                    degree += f128::f128::new(val);
                    *degrees.offset((i + j) as isize) -= f128::f128::new(val);
                    j += 1;
                    count += 1;
                }
                *degrees.offset(i as isize) -= degree;
                i += 1;
            }
            step = n;
            count = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < n {
                *lap2
                    .offset(
                        count as isize,
                    ) = (*degrees.offset(i as isize)).to_f32().unwrap();
                i += 1;
                count += step;
                step -= 1;
            }
            b = gcalloc(
                dim as size_t,
                ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
            ) as *mut *mut libc::c_float;
            let ref mut fresh27 = *b.offset(0 as libc::c_int as isize);
            *fresh27 = gcalloc(
                (dim * n) as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            k = 1 as libc::c_int;
            while k < dim {
                let ref mut fresh28 = *b.offset(k as isize);
                *fresh28 = (*b.offset(0 as libc::c_int as isize))
                    .offset((k * n) as isize);
                k += 1;
            }
            tmp_coords = gcalloc(
                n as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            dist_accumulator = gcalloc(
                n as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            lap1 = gcalloc(
                lap_length as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            old_stress = 1.7976931348623157e+308f64;
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b": %.2f sec\n\0" as *const u8 as *const libc::c_char,
                    elapsed_sec(),
                );
                fprintf(
                    stderr,
                    b"Solving model: \0" as *const u8 as *const libc::c_char,
                );
                start_timer();
            }
            converged = 0 as libc::c_int != 0;
            iterations = 0 as libc::c_int;
            's_681: loop {
                if !(iterations < maxi && !converged) {
                    current_block = 17916325244215494384;
                    break;
                }
                memset(
                    degrees as *mut libc::c_void,
                    0 as libc::c_int,
                    (n as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<f128::f128>() as libc::c_ulong,
                        ),
                );
                if exp == 2 as libc::c_int {
                    sqrt_vecf(lap_length, lap2, lap1);
                }
                count = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n - 1 as libc::c_int {
                    len = n - i - 1 as libc::c_int;
                    set_vector_valf(
                        len,
                        0 as libc::c_int as libc::c_float,
                        dist_accumulator,
                    );
                    k = 0 as libc::c_int;
                    while k < dim {
                        let mut x: size_t = 0;
                        x = 0 as libc::c_int as size_t;
                        while x < len as size_t {
                            let mut tmp: libc::c_float = *(*coords.offset(k as isize))
                                .offset(i as isize)
                                + -1.0f32
                                    * *(*coords.offset(k as isize))
                                        .offset(i as isize)
                                        .offset(1 as libc::c_int as isize)
                                        .offset(x as isize);
                            *dist_accumulator.offset(x as isize) += tmp * tmp;
                            x = x.wrapping_add(1);
                        }
                        k += 1;
                    }
                    invert_sqrt_vec(len, dist_accumulator);
                    j = 0 as libc::c_int;
                    while j < len {
                        if *dist_accumulator.offset(j as isize) >= 3.40282347e+38f32
                            || *dist_accumulator.offset(j as isize)
                                < 0 as libc::c_int as libc::c_float
                        {
                            *dist_accumulator
                                .offset(j as isize) = 0 as libc::c_int as libc::c_float;
                        }
                        j += 1;
                    }
                    count += 1;
                    degree = f128::f128::new(0 as libc::c_int);
                    if exp == 2 as libc::c_int {
                        j = 0 as libc::c_int;
                        while j < len {
                            let ref mut fresh29 = *lap1.offset(count as isize);
                            *fresh29 *= *dist_accumulator.offset(j as isize);
                            val = *fresh29;
                            degree += f128::f128::new(val);
                            *degrees.offset((i + j + 1 as libc::c_int) as isize)
                                -= f128::f128::new(val);
                            j += 1;
                            count += 1;
                        }
                    } else {
                        j = 0 as libc::c_int;
                        while j < len {
                            let ref mut fresh30 = *lap1.offset(count as isize);
                            *fresh30 = *dist_accumulator.offset(j as isize);
                            val = *fresh30;
                            degree += f128::f128::new(val);
                            *degrees.offset((i + j + 1 as libc::c_int) as isize)
                                -= f128::f128::new(val);
                            j += 1;
                            count += 1;
                        }
                    }
                    *degrees.offset(i as isize) -= degree;
                    i += 1;
                }
                step = n;
                count = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < n {
                    *lap1
                        .offset(
                            count as isize,
                        ) = (*degrees.offset(i as isize)).to_f32().unwrap();
                    i += 1;
                    count += step;
                    step -= 1;
                }
                k = 0 as libc::c_int;
                while k < dim {
                    right_mult_with_vector_ff(
                        lap1,
                        n,
                        *coords.offset(k as isize),
                        *b.offset(k as isize),
                    );
                    k += 1;
                }
                new_stress = 0 as libc::c_int as libc::c_double;
                k = 0 as libc::c_int;
                while k < dim {
                    new_stress
                        += vectors_inner_productf(
                            n,
                            *coords.offset(k as isize),
                            *b.offset(k as isize),
                        );
                    k += 1;
                }
                new_stress *= 2 as libc::c_int as libc::c_double;
                new_stress += constant_term as libc::c_double;
                k = 0 as libc::c_int;
                while k < dim {
                    right_mult_with_vector_ff(
                        lap2,
                        n,
                        *coords.offset(k as isize),
                        tmp_coords,
                    );
                    new_stress
                        -= vectors_inner_productf(
                            n,
                            *coords.offset(k as isize),
                            tmp_coords,
                        );
                    k += 1;
                }
                let mut diff: libc::c_double = old_stress - new_stress;
                let mut change: libc::c_double = fabs(diff);
                converged = change / old_stress < Epsilon || new_stress < Epsilon;
                old_stress = new_stress;
                k = 0 as libc::c_int;
                while k < dim {
                    let mut np: *mut node_t = 0 as *mut node_t;
                    if havePinned != 0 {
                        copy_vectorf(n, *coords.offset(k as isize), tmp_coords);
                        if conjugate_gradient_mkernel(
                            lap2,
                            tmp_coords,
                            *b.offset(k as isize),
                            n,
                            conj_tol,
                            n,
                        ) < 0 as libc::c_int
                        {
                            iterations = -(1 as libc::c_int);
                            current_block = 15300195249473174512;
                            break 's_681;
                        } else {
                            i = 0 as libc::c_int;
                            while i < n {
                                np = *nodes.offset(i as isize);
                                if !((*((*(np as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                    .pinned as libc::c_int > 1 as libc::c_int)
                                {
                                    *(*coords.offset(k as isize))
                                        .offset(i as isize) = *tmp_coords.offset(i as isize);
                                }
                                i += 1;
                            }
                        }
                    } else if conjugate_gradient_mkernel(
                            lap2,
                            *coords.offset(k as isize),
                            *b.offset(k as isize),
                            n,
                            conj_tol,
                            n,
                        ) < 0 as libc::c_int
                        {
                        iterations = -(1 as libc::c_int);
                        current_block = 15300195249473174512;
                        break 's_681;
                    }
                    k += 1;
                }
                if Verbose as libc::c_int != 0
                    && iterations % 5 as libc::c_int == 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"%.3f \0" as *const u8 as *const libc::c_char,
                        new_stress,
                    );
                    if (iterations + 5 as libc::c_int) % 50 as libc::c_int
                        == 0 as libc::c_int
                    {
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                }
                iterations += 1;
            }
            match current_block {
                15300195249473174512 => {}
                _ => {
                    if Verbose != 0 {
                        fprintf(
                            stderr,
                            b"\nfinal e = %f %d iterations %.2f sec\n\0" as *const u8
                                as *const libc::c_char,
                            compute_stressf(coords, lap2, dim, n, exp),
                            iterations,
                            elapsed_sec(),
                        );
                    }
                    i = 0 as libc::c_int;
                    while i < dim {
                        j = 0 as libc::c_int;
                        while j < n {
                            *(*d_coords.offset(i as isize))
                                .offset(
                                    j as isize,
                                ) = *(*coords.offset(i as isize)).offset(j as isize)
                                as libc::c_double;
                            j += 1;
                        }
                        i += 1;
                    }
                }
            }
        }
        _ => {}
    }
    free(f_storage as *mut libc::c_void);
    free(coords as *mut libc::c_void);
    free(lap2 as *mut libc::c_void);
    if !b.is_null() {
        free(*b.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(b as *mut libc::c_void);
    }
    free(tmp_coords as *mut libc::c_void);
    free(dist_accumulator as *mut libc::c_void);
    free(degrees as *mut libc::c_void);
    free(lap1 as *mut libc::c_void);
    return iterations;
}
