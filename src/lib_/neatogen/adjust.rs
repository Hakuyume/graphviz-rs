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
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn free(_: *mut libc::c_void);
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    static mut Verbose: libc::c_uchar;
    static mut Ndim: libc::c_int;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn late_double(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: libc::c_double,
        _: libc::c_double,
    ) -> libc::c_double;
    fn mapBool(_: *const libc::c_char, _: bool) -> bool;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn SparseMatrix_from_coordinate_arrays(
        nz: libc::c_int,
        m: libc::c_int,
        n: libc::c_int,
        irn: *mut libc::c_int,
        jcn: *mut libc::c_int,
        val: *mut libc::c_void,
        type_0: libc::c_int,
        sz: size_t,
    ) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_is_symmetric(A: SparseMatrix, test_pattern_symmetry_only: bool) -> libc::c_int;
    fn SparseMatrix_remove_diagonal(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(A: SparseMatrix) -> SparseMatrix;
    fn cAdjust(_: *mut graph_t, _: libc::c_int) -> libc::c_int;
    fn scAdjust(_: *mut graph_t, _: libc::c_int) -> libc::c_int;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut xmin: libc::c_double;
    static mut xmax: libc::c_double;
    static mut ymin: libc::c_double;
    fn geominit();
    static mut deltax: libc::c_double;
    static mut ymax: libc::c_double;
    static mut deltay: libc::c_double;
    static mut nsites: libc::c_int;
    fn dist_2(_: *mut Point, _: *mut Point) -> libc::c_double;
    fn voronoi(_: libc::c_int, _: Option<unsafe extern "C" fn() -> *mut Site>);
    fn siteinit();
    fn polyOverlap(_: Point, _: *mut Poly, _: Point, _: *mut Poly) -> libc::c_int;
    fn breakPoly(_: *mut Poly);
    fn infoinit();
    fn addVertex(_: *mut Site, _: libc::c_double, _: libc::c_double);
    fn polyFree();
    fn makePoly(_: *mut Poly, _: *mut Agnode_t, _: libc::c_float, _: libc::c_float) -> libc::c_int;
    fn makeAddPoly(
        _: *mut Poly,
        _: *mut Agnode_t,
        _: libc::c_float,
        _: libc::c_float,
    ) -> libc::c_int;
    static mut nodeInfo: *mut Info_t;
    static mut pxmin: libc::c_double;
    static mut pxmax: libc::c_double;
    static mut pymin: libc::c_double;
    static mut pymax: libc::c_double;
    fn edgeinit();
    fn ELcleanup();
    fn PQcleanup();
    fn remove_overlap(
        dim_0: libc::c_int,
        A: SparseMatrix,
        x: *mut libc::c_double,
        label_sizes: *mut libc::c_double,
        ntry: libc::c_int,
        initial_scaling: libc::c_double,
        edge_labeling_scheme: libc::c_int,
        n_constr_nodes: libc::c_int,
        constr_nodes: *mut libc::c_int,
        A_constr: SparseMatrix,
        doShrink: libc::c_int,
    );
    fn removeoverlaps(_: libc::c_int, _: *mut *mut libc::c_float, _: *mut ipsep_options);
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
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
pub struct SparseMatrix_struct {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nz: libc::c_int,
    pub nzmax: libc::c_int,
    pub type_0: libc::c_int,
    pub ia: *mut libc::c_int,
    pub ja: *mut libc::c_int,
    pub a: *mut libc::c_void,
    pub format: libc::c_int,
    pub property: libc::c_int,
    pub size: libc::c_int,
}
pub type SparseMatrix = *mut SparseMatrix_struct;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_8 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_8 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_8 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_8 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_8 = 1;
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
pub const _ISspace: C2RustUnnamed_10 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Site {
    pub coord: Point,
    pub sitenbr: libc::c_int,
    pub refcnt: libc::c_int,
}
pub type Point = pointf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Info_t {
    pub node: *mut Agnode_t,
    pub site: Site,
    pub overlaps: libc::c_int,
    pub poly: Poly,
    pub verts: *mut PtItem,
}
pub type PtItem = ptitem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptitem {
    pub next: *mut ptitem,
    pub p: Point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Poly {
    pub origin: Point,
    pub corner: Point,
    pub nverts: libc::c_int,
    pub verts: *mut Point,
    pub kind: libc::c_int,
}
pub const dim: C2RustUnnamed_9 = 2;
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
pub struct ipsep_options {
    pub diredges: libc::c_int,
    pub edge_gap: libc::c_double,
    pub noverlap: libc::c_int,
    pub gap: pointf,
    pub nsize: *mut pointf,
    pub clusters: *mut cluster_data,
}
pub type C2RustUnnamed_9 = libc::c_uint;
pub const ELSCHEME_NONE: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookup_t {
    pub mode: adjust_mode,
    pub attrib: *mut libc::c_char,
    pub len: libc::c_int,
    pub print: *mut libc::c_char,
}
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const ELSCHEME_STRAIGHTLINE_PENALTY2: C2RustUnnamed_11 = 4;
pub const ELSCHEME_STRAIGHTLINE_PENALTY: C2RustUnnamed_11 = 3;
pub const ELSCHEME_PENALTY2: C2RustUnnamed_11 = 2;
pub const ELSCHEME_PENALTY: C2RustUnnamed_11 = 1;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut margin: libc::c_double = 0.05f64;
static mut incr: libc::c_double = 0.05f64;
static mut iterations: libc::c_int = -(1 as libc::c_int);
static mut useIter: libc::c_int = 0 as libc::c_int;
static mut doAll: libc::c_int = 0 as libc::c_int;
static mut sites: *mut *mut Site = 0 as *const *mut Site as *mut *mut Site;
static mut endSite: *mut *mut Site = 0 as *const *mut Site as *mut *mut Site;
static mut nw: Point = Point { x: 0., y: 0. };
static mut ne: Point = Point { x: 0., y: 0. };
static mut sw: Point = Point { x: 0., y: 0. };
static mut se: Point = Point { x: 0., y: 0. };
static mut nextSite: *mut *mut Site = 0 as *const *mut Site as *mut *mut Site;
unsafe extern "C" fn setBoundBox(mut ll: *mut Point, mut ur: *mut Point) {
    pxmin = (*ll).x;
    pxmax = (*ur).x;
    pymin = (*ll).y;
    pymax = (*ur).y;
    sw.x = pxmin;
    nw.x = sw.x;
    se.x = pxmax;
    ne.x = se.x;
    ne.y = pymax;
    nw.y = ne.y;
    se.y = pymin;
    sw.y = se.y;
}
unsafe extern "C" fn freeNodes() {
    let mut i: libc::c_int = 0;
    let mut ip: *mut Info_t = nodeInfo;
    i = 0 as libc::c_int;
    while i < nsites {
        breakPoly(&mut (*ip).poly);
        ip = ip.offset(1);
        i += 1;
    }
    polyFree();
    infoinit();
    free(nodeInfo as *mut libc::c_void);
}
unsafe extern "C" fn chkBoundBox(mut graph: *mut Agraph_t) {
    let mut ll: Point = Point { x: 0., y: 0. };
    let mut ur: Point = Point { x: 0., y: 0. };
    let mut ip: *mut Info_t = nodeInfo;
    let mut pp: *mut Poly = &mut (*ip).poly;
    let mut x: libc::c_double = (*ip).site.coord.x;
    let mut y: libc::c_double = (*ip).site.coord.y;
    let mut x_min: libc::c_double = (*pp).origin.x + x;
    let mut y_min: libc::c_double = (*pp).origin.y + y;
    let mut x_max: libc::c_double = (*pp).corner.x + x;
    let mut y_max: libc::c_double = (*pp).corner.y + y;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < nsites {
        ip = ip.offset(1);
        pp = &mut (*ip).poly;
        x = (*ip).site.coord.x;
        y = (*ip).site.coord.y;
        x_min = fmin(x_min, (*pp).origin.x + x);
        y_min = fmin(y_min, (*pp).origin.y + y);
        x_max = fmax(x_max, (*pp).corner.x + x);
        y_max = fmax(y_max, (*pp).corner.y + y);
        i += 1;
    }
    let mut marg: *mut libc::c_char = agget(
        graph as *mut libc::c_void,
        b"voro_margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !marg.is_null() && *marg as libc::c_int != '\0' as i32 {
        margin = atof(marg);
    }
    let mut ydelta: libc::c_double = margin * (y_max - y_min);
    let mut xdelta: libc::c_double = margin * (x_max - x_min);
    ll.x = x_min - xdelta;
    ll.y = y_min - ydelta;
    ur.x = x_max + xdelta;
    ur.y = y_max + ydelta;
    setBoundBox(&mut ll, &mut ur);
}
unsafe extern "C" fn makeInfo(mut graph: *mut Agraph_t) -> libc::c_int {
    let mut node: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut i: libc::c_int = 0;
    let mut ip: *mut Info_t = 0 as *mut Info_t;
    let mut pmargin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    let mut polyf: Option<
        unsafe extern "C" fn(*mut Poly, *mut Agnode_t, libc::c_float, libc::c_float) -> libc::c_int,
    > = None;
    nsites = agnnodes(graph);
    geominit();
    nodeInfo = gcalloc(
        nsites as size_t,
        ::std::mem::size_of::<Info_t>() as libc::c_ulong,
    ) as *mut Info_t;
    node = agfstnode(graph);
    ip = nodeInfo;
    pmargin = sepFactor(graph);
    if pmargin.doAdd {
        polyf = Some(
            makeAddPoly
                as unsafe extern "C" fn(
                    *mut Poly,
                    *mut Agnode_t,
                    libc::c_float,
                    libc::c_float,
                ) -> libc::c_int,
        );
        pmargin.x =
            (pmargin.x as libc::c_double / 72 as libc::c_int as libc::c_double) as libc::c_float;
        pmargin.y =
            (pmargin.y as libc::c_double / 72 as libc::c_int as libc::c_double) as libc::c_float;
    } else {
        polyf = Some(
            makePoly
                as unsafe extern "C" fn(
                    *mut Poly,
                    *mut Agnode_t,
                    libc::c_float,
                    libc::c_float,
                ) -> libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < nsites {
        (*ip).site.coord.x = *((*((*(node as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize);
        (*ip).site.coord.y = *((*((*(node as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize);
        if polyf.expect("non-null function pointer")(&mut (*ip).poly, node, pmargin.x, pmargin.y)
            != 0
        {
            free(nodeInfo as *mut libc::c_void);
            nodeInfo = 0 as *mut Info_t;
            return 1 as libc::c_int;
        }
        (*ip).site.sitenbr = i;
        (*ip).site.refcnt = 1 as libc::c_int;
        let ref mut fresh0 = (*ip).node;
        *fresh0 = node;
        let ref mut fresh1 = (*ip).verts;
        *fresh1 = 0 as *mut PtItem;
        node = agnxtnode(graph, node);
        ip = ip.offset(1);
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn scomp(
    mut S1: *const libc::c_void,
    mut S2: *const libc::c_void,
) -> libc::c_int {
    let mut s1: *const Site = 0 as *const Site;
    let mut s2: *const Site = 0 as *const Site;
    s1 = *(S1 as *const *mut Site);
    s2 = *(S2 as *const *mut Site);
    if (*s1).coord.y < (*s2).coord.y {
        return -(1 as libc::c_int);
    }
    if (*s1).coord.y > (*s2).coord.y {
        return 1 as libc::c_int;
    }
    if (*s1).coord.x < (*s2).coord.x {
        return -(1 as libc::c_int);
    }
    if (*s1).coord.x > (*s2).coord.x {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sortSites() {
    let mut i: libc::c_int = 0;
    let mut sp: *mut *mut Site = 0 as *mut *mut Site;
    let mut ip: *mut Info_t = 0 as *mut Info_t;
    if sites.is_null() {
        sites = gcalloc(
            nsites as size_t,
            ::std::mem::size_of::<*mut Site>() as libc::c_ulong,
        ) as *mut *mut Site;
        endSite = sites.offset(nsites as isize);
    }
    sp = sites;
    ip = nodeInfo;
    infoinit();
    i = 0 as libc::c_int;
    while i < nsites {
        let fresh2 = sp;
        sp = sp.offset(1);
        *fresh2 = &mut (*ip).site;
        let ref mut fresh3 = (*ip).verts;
        *fresh3 = 0 as *mut PtItem;
        (*ip).site.refcnt = 1 as libc::c_int;
        ip = ip.offset(1);
        i += 1;
    }
    qsort(
        sites as *mut libc::c_void,
        nsites as size_t,
        ::std::mem::size_of::<*mut Site>() as libc::c_ulong,
        Some(
            scomp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    nextSite = sites;
}
unsafe extern "C" fn geomUpdate(mut doSort: libc::c_int) {
    let mut i: libc::c_int = 0;
    if doSort != 0 {
        sortSites();
    }
    xmin = (**sites.offset(0 as libc::c_int as isize)).coord.x;
    xmax = (**sites.offset(0 as libc::c_int as isize)).coord.x;
    i = 1 as libc::c_int;
    while i < nsites {
        xmin = fmin(xmin, (**sites.offset(i as isize)).coord.x);
        xmax = fmax(xmax, (**sites.offset(i as isize)).coord.x);
        i += 1;
    }
    ymin = (**sites.offset(0 as libc::c_int as isize)).coord.y;
    ymax = (**sites.offset((nsites - 1 as libc::c_int) as isize))
        .coord
        .y;
    deltay = ymax - ymin;
    deltax = xmax - xmin;
}
unsafe extern "C" fn nextOne() -> *mut Site {
    if nextSite < endSite {
        let fresh4 = nextSite;
        nextSite = nextSite.offset(1);
        return *fresh4;
    } else {
        return 0 as *mut Site;
    };
}
unsafe extern "C" fn rmEquality() {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut ip: *mut *mut Site = 0 as *mut *mut Site;
    let mut jp: *mut *mut Site = 0 as *mut *mut Site;
    let mut kp: *mut *mut Site = 0 as *mut *mut Site;
    let mut xdel: libc::c_double = 0.;
    sortSites();
    ip = sites;
    while ip < endSite {
        jp = ip.offset(1 as libc::c_int as isize);
        if jp >= endSite || (**jp).coord.x != (**ip).coord.x || (**jp).coord.y != (**ip).coord.y {
            ip = jp;
        } else {
            cnt = 2 as libc::c_int;
            kp = jp.offset(1 as libc::c_int as isize);
            while kp < endSite
                && (**kp).coord.x == (**ip).coord.x
                && (**kp).coord.y == (**ip).coord.y
            {
                cnt += 1;
                jp = kp;
                kp = jp.offset(1 as libc::c_int as isize);
            }
            if kp < endSite && (**kp).coord.y == (**ip).coord.y {
                xdel = ((**kp).coord.x - (**ip).coord.x) / cnt as libc::c_double;
                i = 1 as libc::c_int;
                jp = ip.offset(1 as libc::c_int as isize);
                while jp < kp {
                    (**jp).coord.x += i as libc::c_double * xdel;
                    i += 1;
                    jp = jp.offset(1);
                }
            } else {
                let mut info: *mut Info_t = 0 as *mut Info_t;
                jp = ip.offset(1 as libc::c_int as isize);
                while jp < kp {
                    info = nodeInfo.offset((**ip).sitenbr as isize);
                    xdel = (*info).poly.corner.x - (*info).poly.origin.x;
                    info = nodeInfo.offset((**jp).sitenbr as isize);
                    xdel += (*info).poly.corner.x - (*info).poly.origin.x;
                    (**jp).coord.x = (**ip).coord.x + xdel / 2 as libc::c_int as libc::c_double;
                    ip = ip.offset(1);
                    jp = jp.offset(1);
                }
            }
            ip = kp;
        }
    }
}
unsafe extern "C" fn countOverlap(mut iter: libc::c_int) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ip: *mut Info_t = nodeInfo;
    let mut jp: *mut Info_t = 0 as *mut Info_t;
    i = 0 as libc::c_int;
    while i < nsites {
        (*nodeInfo.offset(i as isize)).overlaps = 0 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nsites - 1 as libc::c_int {
        jp = ip.offset(1 as libc::c_int as isize);
        j = i + 1 as libc::c_int;
        while j < nsites {
            if polyOverlap(
                (*ip).site.coord,
                &mut (*ip).poly,
                (*jp).site.coord,
                &mut (*jp).poly,
            ) != 0
            {
                count += 1;
                (*ip).overlaps = 1 as libc::c_int;
                (*jp).overlaps = 1 as libc::c_int;
            }
            jp = jp.offset(1);
            j += 1;
        }
        ip = ip.offset(1);
        i += 1;
    }
    if Verbose as libc::c_int > 1 as libc::c_int {
        fprintf(
            stderr,
            b"overlap [%d] : %d\n\0" as *const u8 as *const libc::c_char,
            iter,
            count,
        );
    }
    return count;
}
unsafe extern "C" fn increaseBoundBox() {
    let mut ydelta: libc::c_double = 0.;
    let mut xdelta: libc::c_double = 0.;
    let mut ll: Point = Point { x: 0., y: 0. };
    let mut ur: Point = Point { x: 0., y: 0. };
    ur.x = pxmax;
    ur.y = pymax;
    ll.x = pxmin;
    ll.y = pymin;
    ydelta = incr * (ur.y - ll.y);
    xdelta = incr * (ur.x - ll.x);
    ur.x += xdelta;
    ur.y += ydelta;
    ll.x -= xdelta;
    ll.y -= ydelta;
    setBoundBox(&mut ll, &mut ur);
}
unsafe extern "C" fn areaOf(mut a: Point, mut b: Point, mut c: Point) -> libc::c_double {
    return fabs(a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y))
        / 2 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn centroidOf(
    mut a: Point,
    mut b: Point,
    mut c: Point,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    *x = (a.x + b.x + c.x) / 3 as libc::c_int as libc::c_double;
    *y = (a.y + b.y + c.y) / 3 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn newpos(mut ip: *mut Info_t) {
    let mut anchor: *mut PtItem = (*ip).verts;
    let mut p: *mut PtItem = 0 as *mut PtItem;
    let mut q: *mut PtItem = 0 as *mut PtItem;
    let mut totalArea: libc::c_double = 0.0f64;
    let mut cx: libc::c_double = 0.0f64;
    let mut cy: libc::c_double = 0.0f64;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut area: libc::c_double = 0.;
    p = (*anchor).next;
    q = (*p).next;
    while !q.is_null() {
        area = areaOf((*anchor).p, (*p).p, (*q).p);
        centroidOf((*anchor).p, (*p).p, (*q).p, &mut x, &mut y);
        cx += area * x;
        cy += area * y;
        totalArea += area;
        p = q;
        q = (*q).next;
    }
    (*ip).site.coord.x = cx / totalArea;
    (*ip).site.coord.y = cy / totalArea;
}
unsafe extern "C" fn addCorners() {
    let mut ip: *mut Info_t = nodeInfo;
    let mut sws: *mut Info_t = ip;
    let mut nws: *mut Info_t = ip;
    let mut ses: *mut Info_t = ip;
    let mut nes: *mut Info_t = ip;
    let mut swd: libc::c_double = dist_2(&mut (*ip).site.coord, &mut sw);
    let mut nwd: libc::c_double = dist_2(&mut (*ip).site.coord, &mut nw);
    let mut sed: libc::c_double = dist_2(&mut (*ip).site.coord, &mut se);
    let mut ned: libc::c_double = dist_2(&mut (*ip).site.coord, &mut ne);
    let mut d: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    ip = ip.offset(1);
    i = 1 as libc::c_int;
    while i < nsites {
        d = dist_2(&mut (*ip).site.coord, &mut sw);
        if d < swd {
            swd = d;
            sws = ip;
        }
        d = dist_2(&mut (*ip).site.coord, &mut se);
        if d < sed {
            sed = d;
            ses = ip;
        }
        d = dist_2(&mut (*ip).site.coord, &mut nw);
        if d < nwd {
            nwd = d;
            nws = ip;
        }
        d = dist_2(&mut (*ip).site.coord, &mut ne);
        if d < ned {
            ned = d;
            nes = ip;
        }
        ip = ip.offset(1);
        i += 1;
    }
    addVertex(&mut (*sws).site, sw.x, sw.y);
    addVertex(&mut (*ses).site, se.x, se.y);
    addVertex(&mut (*nws).site, nw.x, nw.y);
    addVertex(&mut (*nes).site, ne.x, ne.y);
}
unsafe extern "C" fn newPos() {
    let mut i: libc::c_int = 0;
    let mut ip: *mut Info_t = nodeInfo;
    addCorners();
    i = 0 as libc::c_int;
    while i < nsites {
        if doAll != 0 || (*ip).overlaps != 0 {
            newpos(ip);
        }
        ip = ip.offset(1);
        i += 1;
    }
}
unsafe extern "C" fn cleanup() {
    PQcleanup();
    ELcleanup();
    siteinit();
    edgeinit();
}
unsafe extern "C" fn vAdjust() -> libc::c_int {
    let mut iterCnt: libc::c_int = 0 as libc::c_int;
    let mut overlapCnt: libc::c_int = 0 as libc::c_int;
    let mut badLevel: libc::c_int = 0 as libc::c_int;
    let mut increaseCnt: libc::c_int = 0 as libc::c_int;
    let mut cnt: libc::c_int = 0;
    if useIter == 0 || iterations > 0 as libc::c_int {
        overlapCnt = countOverlap(iterCnt);
    }
    if overlapCnt == 0 as libc::c_int || iterations == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    rmEquality();
    geomUpdate(0 as libc::c_int);
    voronoi(
        0 as libc::c_int,
        Some(nextOne as unsafe extern "C" fn() -> *mut Site),
    );
    loop {
        newPos();
        iterCnt += 1;
        if useIter != 0 && iterCnt == iterations {
            break;
        }
        cnt = countOverlap(iterCnt);
        if cnt == 0 as libc::c_int {
            break;
        }
        if cnt >= overlapCnt {
            badLevel += 1;
        } else {
            badLevel = 0 as libc::c_int;
        }
        overlapCnt = cnt;
        match badLevel {
            0 => {
                doAll = 1 as libc::c_int;
            }
            _ => {
                doAll = 1 as libc::c_int;
                increaseCnt += 1;
                increaseBoundBox();
            }
        }
        geomUpdate(1 as libc::c_int);
        voronoi(
            0 as libc::c_int,
            Some(nextOne as unsafe extern "C" fn() -> *mut Site),
        );
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Number of iterations = %d\n\0" as *const u8 as *const libc::c_char,
            iterCnt,
        );
        fprintf(
            stderr,
            b"Number of increases = %d\n\0" as *const u8 as *const libc::c_char,
            increaseCnt,
        );
    }
    cleanup();
    return 1 as libc::c_int;
}
unsafe extern "C" fn rePos() -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut ip: *mut Info_t = nodeInfo;
    let mut f: libc::c_double = 1.0f64 + incr;
    i = 0 as libc::c_int;
    while i < nsites {
        (*ip).site.coord.x *= f;
        (*ip).site.coord.y *= f;
        ip = ip.offset(1);
        i += 1;
    }
    return f;
}
unsafe extern "C" fn sAdjust() -> libc::c_int {
    let mut iterCnt: libc::c_int = 0 as libc::c_int;
    let mut overlapCnt: libc::c_int = 0 as libc::c_int;
    let mut cnt: libc::c_int = 0;
    if useIter == 0 || iterations > 0 as libc::c_int {
        overlapCnt = countOverlap(iterCnt);
    }
    if overlapCnt == 0 as libc::c_int || iterations == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    rmEquality();
    loop {
        rePos();
        iterCnt += 1;
        if useIter != 0 && iterCnt == iterations {
            break;
        }
        cnt = countOverlap(iterCnt);
        if cnt == 0 as libc::c_int {
            break;
        }
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Number of iterations = %d\n\0" as *const u8 as *const libc::c_char,
            iterCnt,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateGraph() {
    let mut i: libc::c_int = 0;
    let mut ip: *mut Info_t = 0 as *mut Info_t;
    ip = nodeInfo;
    i = 0 as libc::c_int;
    while i < nsites {
        *((*((*((*ip).node as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) = (*ip).site.coord.x;
        *((*((*((*ip).node as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) = (*ip).site.coord.y;
        ip = ip.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getSizes(
    mut g: *mut Agraph_t,
    mut pad: pointf,
    mut n_elabels: *mut libc::c_int,
    mut elabels: *mut *mut libc::c_int,
) -> *mut libc::c_double {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut sizes: *mut libc::c_double = gcalloc(
        (Ndim * agnnodes(g)) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut nedge_nodes: libc::c_int = 0 as libc::c_int;
    let mut elabs: *mut libc::c_int = 0 as *mut libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        if !elabels.is_null()
            && strncmp(
                agnameof(n as *mut libc::c_void),
                b"|edgelabel|\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            nedge_nodes += 1;
        }
        i = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id;
        *sizes.offset((i * Ndim) as isize) =
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width * 0.5f64 + pad.x;
        *sizes.offset((i * Ndim + 1 as libc::c_int) as isize) =
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height * 0.5f64 + pad.y;
        n = agnxtnode(g, n);
    }
    if !elabels.is_null() && nedge_nodes != 0 {
        elabs = gcalloc(
            nedge_nodes as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        nedge_nodes = 0 as libc::c_int;
        n = agfstnode(g);
        while !n.is_null() {
            if strncmp(
                agnameof(n as *mut libc::c_void),
                b"|edgelabel|\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
            {
                let fresh5 = nedge_nodes;
                nedge_nodes = nedge_nodes + 1;
                *elabs.offset(fresh5 as isize) =
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id;
            }
            n = agnxtnode(g, n);
        }
        *elabels = elabs;
        *n_elabels = nedge_nodes;
    }
    return sizes;
}
#[no_mangle]
pub unsafe extern "C" fn makeMatrix(
    mut g: *mut Agraph_t,
    mut D: *mut SparseMatrix,
) -> SparseMatrix {
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut nnodes: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut I: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut J: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_double = 0.;
    let mut type_0: libc::c_int = MATRIX_TYPE_REAL as libc::c_int;
    let mut symD: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut valD: *mut libc::c_double = 0 as *mut libc::c_double;
    if g.is_null() {
        return 0 as SparseMatrix;
    }
    nnodes = agnnodes(g);
    nedges = agnedges(g);
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        let fresh6 = i;
        i = i + 1;
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id = fresh6;
        n = agnxtnode(g, n);
    }
    I = gcalloc(
        nedges as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    J = gcalloc(
        nedges as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    val = gcalloc(
        nedges as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    sym = agattr(
        g,
        2 as libc::c_int,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if !D.is_null() {
        symD = agattr(
            g,
            2 as libc::c_int,
            b"len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
        valD = gcalloc(
            nedges as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
    }
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        row = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id;
        e = agfstout(g, n);
        while !e.is_null() {
            *I.offset(i as isize) = row;
            *J
                .offset(
                    i as isize,
                ) = (*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .id;
            if sym.is_null()
                || sscanf(
                    agxget(e as *mut libc::c_void, sym),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_double,
                ) != 1 as libc::c_int
            {
                v = 1 as libc::c_int as libc::c_double;
            }
            *val.offset(i as isize) = v;
            if !symD.is_null() {
                if sscanf(
                    agxget(e as *mut libc::c_void, symD),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_double,
                ) != 1 as libc::c_int
                {
                    v = 1 as libc::c_int as libc::c_double;
                }
                *valD.offset(i as isize) = v;
            }
            i += 1;
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    A = SparseMatrix_from_coordinate_arrays(
        nedges,
        nnodes,
        nnodes,
        I,
        J,
        val as *mut libc::c_void,
        type_0,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    if !D.is_null() {
        *D = SparseMatrix_from_coordinate_arrays(
            nedges,
            nnodes,
            nnodes,
            I,
            J,
            valD as *mut libc::c_void,
            type_0,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        );
    }
    free(I as *mut libc::c_void);
    free(J as *mut libc::c_void);
    free(val as *mut libc::c_void);
    free(valD as *mut libc::c_void);
    return A;
}
unsafe extern "C" fn fdpAdjust(mut g: *mut graph_t, mut am: *mut adjust_data) -> libc::c_int {
    let mut A0: SparseMatrix = makeMatrix(g, 0 as *mut SparseMatrix);
    let mut A: SparseMatrix = A0;
    let mut sizes: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pos: *mut libc::c_double = gcalloc(
        (Ndim * agnnodes(g)) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut sep: expand_t = sepFactor(g);
    let mut pad: pointf = Point { x: 0., y: 0. };
    if sep.doAdd {
        pad.x = sep.x as libc::c_double / 72 as libc::c_int as libc::c_double;
        pad.y = sep.y as libc::c_double / 72 as libc::c_int as libc::c_double;
    } else {
        pad.x = 4 as libc::c_int as libc::c_double / 72 as libc::c_int as libc::c_double;
        pad.y = 4 as libc::c_int as libc::c_double / 72 as libc::c_int as libc::c_double;
    }
    sizes = getSizes(g, pad, 0 as *mut libc::c_int, 0 as *mut *mut libc::c_int);
    n = agfstnode(g);
    while !n.is_null() {
        let mut npos: *mut libc::c_double =
            pos.offset((Ndim * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id) as isize);
        i = 0 as libc::c_int;
        while i < Ndim {
            *npos.offset(i as isize) =
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos).offset(i as isize);
            i += 1;
        }
        n = agnxtnode(g, n);
    }
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) == 0
        || (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int
    {
        A = SparseMatrix_get_real_adjacency_matrix_symmetrized(A);
    } else {
        A = SparseMatrix_remove_diagonal(A);
    }
    remove_overlap(
        Ndim,
        A,
        pos,
        sizes,
        (*am).value,
        (*am).scaling,
        ELSCHEME_NONE as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_int,
        0 as SparseMatrix,
        if mapBool(
            agget(
                g as *mut libc::c_void,
                b"overlap_shrink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            1 as libc::c_int != 0,
        ) as libc::c_int
            != 0
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else {
            0 as libc::c_int
        },
    );
    n = agfstnode(g);
    while !n.is_null() {
        let mut npos_0: *mut libc::c_double =
            pos.offset((Ndim * (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).id) as isize);
        i = 0 as libc::c_int;
        while i < Ndim {
            *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos).offset(i as isize) =
                *npos_0.offset(i as isize);
            i += 1;
        }
        n = agnxtnode(g, n);
    }
    free(sizes as *mut libc::c_void);
    free(pos as *mut libc::c_void);
    if A != A0 {
        SparseMatrix_delete(A);
    }
    SparseMatrix_delete(A0);
    return flag;
}
unsafe extern "C" fn vpscAdjust(mut G: *mut graph_t) -> libc::c_int {
    let mut nnodes: libc::c_int = agnnodes(G);
    let mut opt: ipsep_options = ipsep_options {
        diredges: 0,
        edge_gap: 0.,
        noverlap: 0,
        gap: Point { x: 0., y: 0. },
        nsize: 0 as *mut pointf,
        clusters: 0 as *mut cluster_data,
    };
    let mut nsize: *mut pointf = gcalloc(
        nnodes as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    let mut coords: [*mut libc::c_float; 2] = [0 as *mut libc::c_float; 2];
    let mut f_storage: *mut libc::c_float = gcalloc(
        (dim as libc::c_int * nnodes) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut exp_margin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    i = 0 as libc::c_int;
    while i < dim as libc::c_int {
        coords[i as usize] = f_storage.offset((i * nnodes) as isize);
        i += 1;
    }
    j = 0 as libc::c_int;
    v = agfstnode(G);
    while !v.is_null() {
        i = 0 as libc::c_int;
        while i < dim as libc::c_int {
            *(coords[i as usize]).offset(j as isize) =
                *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos).offset(i as isize)
                    as libc::c_float;
            i += 1;
        }
        (*nsize.offset(j as isize)).x = (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).width;
        (*nsize.offset(j as isize)).y =
            (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).height;
        j += 1;
        v = agnxtnode(G, v);
    }
    opt.diredges = 0 as libc::c_int;
    opt.edge_gap = 0 as libc::c_int as libc::c_double;
    opt.noverlap = 2 as libc::c_int;
    opt.clusters =
        zmalloc(::std::mem::size_of::<cluster_data>() as libc::c_ulong) as *mut cluster_data;
    exp_margin = sepFactor(G);
    if exp_margin.doAdd {
        opt.gap.x = 2.0f64 * (exp_margin.x as libc::c_double / 72 as libc::c_int as libc::c_double);
        opt.gap.y = 2.0f64 * (exp_margin.y as libc::c_double / 72 as libc::c_int as libc::c_double);
    } else {
        opt.gap.y =
            2.0f64 * (4 as libc::c_int as libc::c_double / 72 as libc::c_int as libc::c_double);
        opt.gap.x = opt.gap.y;
    }
    opt.nsize = nsize;
    removeoverlaps(nnodes, coords.as_mut_ptr(), &mut opt);
    j = 0 as libc::c_int;
    v = agfstnode(G);
    while !v.is_null() {
        i = 0 as libc::c_int;
        while i < dim as libc::c_int {
            *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos).offset(i as isize) =
                *(coords[i as usize]).offset(j as isize) as libc::c_double;
            i += 1;
        }
        j += 1;
        v = agnxtnode(G, v);
    }
    free(opt.clusters as *mut libc::c_void);
    free(f_storage as *mut libc::c_void);
    free(nsize as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn angleSet(mut g: *mut graph_t, mut phi: *mut libc::c_double) -> libc::c_int {
    let mut ang: libc::c_double = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: *mut libc::c_char = agget(
        g as *mut libc::c_void,
        b"normalize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if a.is_null() || *a as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    ang = strtod(a, &mut p);
    if p == a {
        if mapbool(a) {
            ang = 0.0f64;
        } else {
            return 0 as libc::c_int;
        }
    }
    while ang > 180 as libc::c_int as libc::c_double {
        ang -= 360 as libc::c_int as libc::c_double;
    }
    while ang <= -(180 as libc::c_int) as libc::c_double {
        ang += 360 as libc::c_int as libc::c_double;
    }
    *phi = ang / 180.0f64 * 3.14159265358979323846f64;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn normalize(mut g: *mut graph_t) -> libc::c_int {
    let mut v: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut phi: libc::c_double = 0.;
    let mut cosv: libc::c_double = 0.;
    let mut sinv: libc::c_double = 0.;
    let mut p: pointf = Point { x: 0., y: 0. };
    let mut orig: pointf = Point { x: 0., y: 0. };
    let mut ret: libc::c_int = 0;
    if angleSet(g, &mut phi) == 0 {
        return 0 as libc::c_int;
    }
    v = agfstnode(g);
    p.x = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(0 as libc::c_int as isize);
    p.y = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
        .offset(1 as libc::c_int as isize);
    v = agfstnode(g);
    while !v.is_null() {
        *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(0 as libc::c_int as isize) -= p.x;
        *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
            .offset(1 as libc::c_int as isize) -= p.y;
        v = agnxtnode(g, v);
    }
    if p.x != 0. || p.y != 0. {
        ret = 1 as libc::c_int;
    } else {
        ret = 0 as libc::c_int;
    }
    e = 0 as *mut edge_t;
    v = agfstnode(g);
    while !v.is_null() {
        e = agfstout(g, v);
        if !e.is_null() {
            break;
        }
        v = agnxtnode(g, v);
    }
    if e.is_null() {
        return ret;
    }
    phi -= atan2(
        *((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .pos)
            .offset(1 as libc::c_int as isize)
            - *((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(1 as libc::c_int as isize),
        *((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        }))
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .pos)
            .offset(0 as libc::c_int as isize)
            - *((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(0 as libc::c_int as isize),
    );
    if phi != 0. {
        orig.x = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .pos)
            .offset(0 as libc::c_int as isize);
        orig.y = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
        .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .pos)
            .offset(1 as libc::c_int as isize);
        cosv = cos(phi);
        sinv = sin(phi);
        v = agfstnode(g);
        while !v.is_null() {
            p.x = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize)
                - orig.x;
            p.y = *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize)
                - orig.y;
            *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(0 as libc::c_int as isize) = p.x * cosv - p.y * sinv + orig.x;
            *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(1 as libc::c_int as isize) = p.x * sinv + p.y * cosv + orig.y;
            v = agnxtnode(g, v);
        }
        return 1 as libc::c_int;
    } else {
        return ret;
    };
}
static mut adjustMode: [lookup_t; 18] = [lookup_t {
    mode: AM_NONE,
    attrib: 0 as *mut libc::c_char,
    len: 0,
    print: 0 as *mut libc::c_char,
}; 18];
unsafe extern "C" fn setPrismValues(
    mut g: *mut Agraph_t,
    mut s: *mut libc::c_char,
    mut dp: *mut adjust_data,
) {
    let mut v: libc::c_int = 0;
    if sscanf(
        s,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut v as *mut libc::c_int,
    ) > 0 as libc::c_int
        && v >= 0 as libc::c_int
    {
        (*dp).value = v;
    } else {
        (*dp).value = 1000 as libc::c_int;
    }
    (*dp).scaling = late_double(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"overlap_scaling\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        -4.0f64,
        -1.0e10f64,
    );
}
unsafe extern "C" fn getAdjustMode(
    mut g: *mut Agraph_t,
    mut s: *mut libc::c_char,
    mut dp: *mut adjust_data,
) -> *mut adjust_data {
    let mut ap: *mut lookup_t = adjustMode.as_mut_ptr().offset(1 as libc::c_int as isize);
    if s.is_null() || *s as libc::c_int == '\0' as i32 {
        (*dp).mode = adjustMode[0 as libc::c_int as usize].mode;
        let ref mut fresh7 = (*dp).print;
        *fresh7 = adjustMode[0 as libc::c_int as usize].print;
    } else {
        while !((*ap).attrib).is_null() {
            if strncasecmp(s, (*ap).attrib, (*ap).len as libc::c_ulong) == 0 {
                if ((*ap).print).is_null() {
                    agerr(
                        AGWARN,
                        b"Overlap value \"%s\" unsupported - ignored\n\0" as *const u8
                            as *const libc::c_char,
                        (*ap).attrib,
                    );
                    ap = &mut *adjustMode.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut lookup_t;
                }
                (*dp).mode = (*ap).mode;
                let ref mut fresh8 = (*dp).print;
                *fresh8 = (*ap).print;
                if (*ap).mode as libc::c_uint == AM_PRISM as libc::c_int as libc::c_uint {
                    setPrismValues(g, s.offset((*ap).len as isize), dp);
                }
                break;
            } else {
                ap = ap.offset(1);
            }
        }
        if ((*ap).attrib).is_null() {
            let mut v: bool = mapBool(s, 0 as libc::c_int != 0);
            let mut unmappable: bool =
                v as libc::c_int != mapBool(s, 1 as libc::c_int != 0) as libc::c_int;
            if unmappable {
                agerr(
                    AGWARN,
                    b"Unrecognized overlap value \"%s\" - using false\n\0" as *const u8
                        as *const libc::c_char,
                    s,
                );
                v = 0 as libc::c_int != 0;
            }
            if v {
                (*dp).mode = adjustMode[0 as libc::c_int as usize].mode;
                let ref mut fresh9 = (*dp).print;
                *fresh9 = adjustMode[0 as libc::c_int as usize].print;
            } else {
                (*dp).mode = adjustMode[1 as libc::c_int as usize].mode;
                let ref mut fresh10 = (*dp).print;
                *fresh10 = adjustMode[1 as libc::c_int as usize].print;
            }
            if (*dp).mode as libc::c_uint == AM_PRISM as libc::c_int as libc::c_uint {
                setPrismValues(
                    g,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    dp,
                );
            }
        }
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"overlap: %s value %d scaling %.04f\n\0" as *const u8 as *const libc::c_char,
            (*dp).print,
            (*dp).value,
            (*dp).scaling,
        );
    }
    return dp;
}
#[no_mangle]
pub unsafe extern "C" fn graphAdjustMode(
    mut G: *mut graph_t,
    mut dp: *mut adjust_data,
    mut dflt: *mut libc::c_char,
) -> *mut adjust_data {
    let mut am: *mut libc::c_char = agget(
        G as *mut libc::c_void,
        b"overlap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return getAdjustMode(
        G,
        (if !am.is_null() {
            am as *const libc::c_char
        } else if !dflt.is_null() {
            dflt as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        dp,
    );
}
unsafe extern "C" fn simpleScale(mut g: *mut graph_t) -> libc::c_int {
    let mut sc: pointf = Point { x: 0., y: 0. };
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = agget(
        g as *mut libc::c_void,
        b"scale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        i = sscanf(
            p,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut sc.x as *mut libc::c_double,
            &mut sc.y as *mut libc::c_double,
        );
        if i != 0 {
            if fabs(sc.x) < 0.000000001f64 {
                return 0 as libc::c_int;
            }
            if i == 1 as libc::c_int {
                sc.y = sc.x;
            } else if fabs(sc.y) < 0.000000001f64 {
                return 0 as libc::c_int;
            }
            if sc.y == 1 as libc::c_int as libc::c_double
                && sc.x == 1 as libc::c_int as libc::c_double
            {
                return 0 as libc::c_int;
            }
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"scale = (%.03f,%.03f)\n\0" as *const u8 as *const libc::c_char,
                    sc.x,
                    sc.y,
                );
            }
            n = agfstnode(g);
            while !n.is_null() {
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(0 as libc::c_int as isize) *= sc.x;
                *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(1 as libc::c_int as isize) *= sc.y;
                n = agnxtnode(g, n);
            }
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn removeOverlapWith(
    mut G: *mut graph_t,
    mut am: *mut adjust_data,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut nret: libc::c_int = 0;
    if agnnodes(G) < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    nret = normalize(G);
    nret += simpleScale(G);
    if (*am).mode as libc::c_uint == AM_NONE as libc::c_int as libc::c_uint {
        return nret;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Adjusting %s using %s\n\0" as *const u8 as *const libc::c_char,
            agnameof(G as *mut libc::c_void),
            (*am).print,
        );
    }
    if (*am).mode as libc::c_uint > AM_SCALE as libc::c_int as libc::c_uint {
        match (*am).mode as libc::c_uint {
            3 => {
                ret = scAdjust(G, 1 as libc::c_int);
            }
            4 => {
                ret = scAdjust(G, 0 as libc::c_int);
            }
            5 => {
                ret = 0 as libc::c_int;
            }
            6 => {
                ret = 0 as libc::c_int;
            }
            12 | 11 | 13 | 14 | 8 | 7 | 9 | 10 => {
                cAdjust(G, (*am).mode as libc::c_int);
                ret = 0 as libc::c_int;
            }
            15 => {
                ret = scAdjust(G, -(1 as libc::c_int));
            }
            18 => {
                ret = fdpAdjust(G, am);
            }
            17 => return nret,
            16 => {
                ret = vpscAdjust(G);
            }
            _ => {
                if (*am).mode as libc::c_uint != AM_VOR as libc::c_int as libc::c_uint
                    && (*am).mode as libc::c_uint != AM_SCALE as libc::c_int as libc::c_uint
                {
                    agerr(
                        AGWARN,
                        b"Unhandled adjust option %s\n\0" as *const u8 as *const libc::c_char,
                        (*am).print,
                    );
                }
                ret = 0 as libc::c_int;
            }
        }
        return nret + ret;
    }
    if makeInfo(G) != 0 {
        freeNodes();
        free(sites as *mut libc::c_void);
        sites = 0 as *mut *mut Site;
        return nret;
    }
    chkBoundBox(G);
    if (*am).mode as libc::c_uint == AM_SCALE as libc::c_int as libc::c_uint {
        ret = sAdjust();
    } else {
        ret = vAdjust();
    }
    if ret != 0 {
        updateGraph();
    }
    freeNodes();
    free(sites as *mut libc::c_void);
    sites = 0 as *mut *mut Site;
    return ret + nret;
}
#[no_mangle]
pub unsafe extern "C" fn removeOverlapAs(
    mut G: *mut graph_t,
    mut flag: *mut libc::c_char,
) -> libc::c_int {
    let mut am: adjust_data = adjust_data {
        mode: AM_NONE,
        print: 0 as *mut libc::c_char,
        value: 0,
        scaling: 0.,
    };
    if agnnodes(G) < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    getAdjustMode(G, flag, &mut am);
    return removeOverlapWith(G, &mut am);
}
#[no_mangle]
pub unsafe extern "C" fn adjustNodes(mut G: *mut graph_t) -> libc::c_int {
    return removeOverlapAs(
        G,
        agget(
            G as *mut libc::c_void,
            b"overlap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
}
unsafe extern "C" fn parseFactor(
    mut s: *mut libc::c_char,
    mut pp: *mut expand_t,
    mut sepfact: libc::c_float,
    mut dflt: libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        s = s.offset(1);
    }
    if *s as libc::c_int == '+' as i32 {
        s = s.offset(1);
        (*pp).doAdd = 1 as libc::c_int != 0;
    } else {
        (*pp).doAdd = 0 as libc::c_int != 0;
    }
    i = sscanf(
        s,
        b"%f,%f\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_float,
        &mut y as *mut libc::c_float,
    );
    if i != 0 {
        if i == 1 as libc::c_int {
            y = x;
        }
        if (*pp).doAdd {
            if sepfact > 1 as libc::c_int as libc::c_float {
                (*pp).x = if dflt < x / sepfact {
                    dflt
                } else {
                    x / sepfact
                };
                (*pp).y = if dflt < y / sepfact {
                    dflt
                } else {
                    y / sepfact
                };
            } else if sepfact < 1 as libc::c_int as libc::c_float {
                (*pp).x = if dflt > x / sepfact {
                    dflt
                } else {
                    x / sepfact
                };
                (*pp).y = if dflt > y / sepfact {
                    dflt
                } else {
                    y / sepfact
                };
            } else {
                (*pp).x = x;
                (*pp).y = y;
            }
        } else {
            (*pp).x = 1.0f32 + x / sepfact;
            (*pp).y = 1.0f32 + y / sepfact;
        }
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn sepFactor(mut g: *mut graph_t) -> expand_t {
    let mut pmargin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    let mut marg: *mut libc::c_char = 0 as *mut libc::c_char;
    marg = agget(
        g as *mut libc::c_void,
        b"sep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(!marg.is_null()
        && parseFactor(
            marg,
            &mut pmargin,
            1.0f64 as libc::c_float,
            0 as libc::c_int as libc::c_float,
        ) != 0)
    {
        marg = agget(
            g as *mut libc::c_void,
            b"esep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(!marg.is_null()
            && parseFactor(
                marg,
                &mut pmargin,
                0.8f32,
                4 as libc::c_int as libc::c_float,
            ) != 0)
        {
            pmargin.y = 4 as libc::c_int as libc::c_float;
            pmargin.x = pmargin.y;
            pmargin.doAdd = 1 as libc::c_int != 0;
        }
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Node separation: add=%d (%f,%f)\n\0" as *const u8 as *const libc::c_char,
            pmargin.doAdd as libc::c_int,
            pmargin.x as libc::c_double,
            pmargin.y as libc::c_double,
        );
    }
    return pmargin;
}
#[no_mangle]
pub unsafe extern "C" fn esepFactor(mut g: *mut graph_t) -> expand_t {
    let mut pmargin: expand_t = expand_t {
        x: 0.,
        y: 0.,
        doAdd: false,
    };
    let mut marg: *mut libc::c_char = 0 as *mut libc::c_char;
    marg = agget(
        g as *mut libc::c_void,
        b"esep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(!marg.is_null()
        && parseFactor(
            marg,
            &mut pmargin,
            1.0f64 as libc::c_float,
            0 as libc::c_int as libc::c_float,
        ) != 0)
    {
        marg = agget(
            g as *mut libc::c_void,
            b"sep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(!marg.is_null()
            && parseFactor(
                marg,
                &mut pmargin,
                1.0f32 / 0.8f32,
                0.8f32 * 4 as libc::c_int as libc::c_float,
            ) != 0)
        {
            pmargin.y = 0.8f32 * 4 as libc::c_int as libc::c_float;
            pmargin.x = pmargin.y;
            pmargin.doAdd = 1 as libc::c_int != 0;
        }
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"Edge separation: add=%d (%f,%f)\n\0" as *const u8 as *const libc::c_char,
            pmargin.doAdd as libc::c_int,
            pmargin.x as libc::c_double,
            pmargin.y as libc::c_double,
        );
    }
    return pmargin;
}
unsafe extern "C" fn run_static_initializers() {
    adjustMode = [
        {
            let mut init = lookup_t {
                mode: AM_NONE,
                attrib: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_PRISM,
                attrib: b"prism\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"prism\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_VOR,
                attrib: b"voronoi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"Voronoi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_NSCALE,
                attrib: b"scale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"scaling\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_COMPRESS,
                attrib: b"compress\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"compress\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_VPSC,
                attrib: b"vpsc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"vpsc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_IPSEP,
                attrib: b"ipsep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"ipsep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_SCALE,
                attrib: b"oscale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"old scaling\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_SCALEXY,
                attrib: b"scalexy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"x and y scaling\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_ORTHO,
                attrib: b"ortho\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_ORTHO_YX,
                attrib: b"ortho_yx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_ORTHOXY,
                attrib: b"orthoxy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"xy orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_ORTHOYX,
                attrib: b"orthoyx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"yx orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_PORTHO,
                attrib: b"portho\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"pseudo-orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_PORTHO_YX,
                attrib: b"portho_yx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"pseudo-orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_PORTHOXY,
                attrib: b"porthoxy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"xy pseudo-orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_PORTHOYX,
                attrib: b"porthoyx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                print: b"yx pseudo-orthogonal constraints\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = lookup_t {
                mode: AM_NONE,
                attrib: 0 as *mut libc::c_char,
                len: 0 as libc::c_int,
                print: 0 as *mut libc::c_char,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
