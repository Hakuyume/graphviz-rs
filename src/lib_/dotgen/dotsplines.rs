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
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
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
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agnxtattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        attr: *mut Agsym_t,
    ) -> *mut Agsym_t;
    fn agcopyattr(oldobj: *mut libc::c_void, newobj: *mut libc::c_void) -> libc::c_int;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
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
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Agdirected: Agdesc_t;
    static mut Agundirected: Agdesc_t;
    static mut State: libc::c_int;
    static mut EdgeLabelsDone: libc::c_int;
    static mut G_ordering: *mut Agsym_t;
    static mut N_height: *mut Agsym_t;
    static mut N_width: *mut Agsym_t;
    static mut N_shape: *mut Agsym_t;
    static mut N_fontsize: *mut Agsym_t;
    static mut N_fontname: *mut Agsym_t;
    static mut N_fontcolor: *mut Agsym_t;
    static mut N_label: *mut Agsym_t;
    static mut N_xlabel: *mut Agsym_t;
    static mut N_nojustify: *mut Agsym_t;
    static mut N_style: *mut Agsym_t;
    static mut N_showboxes: *mut Agsym_t;
    static mut N_sides: *mut Agsym_t;
    static mut N_peripheries: *mut Agsym_t;
    static mut N_ordering: *mut Agsym_t;
    static mut N_orientation: *mut Agsym_t;
    static mut N_skew: *mut Agsym_t;
    static mut N_distortion: *mut Agsym_t;
    static mut N_fixed: *mut Agsym_t;
    static mut N_group: *mut Agsym_t;
    static mut E_weight: *mut Agsym_t;
    static mut E_minlen: *mut Agsym_t;
    static mut E_fontsize: *mut Agsym_t;
    static mut E_fontname: *mut Agsym_t;
    static mut E_fontcolor: *mut Agsym_t;
    static mut E_label: *mut Agsym_t;
    static mut E_xlabel: *mut Agsym_t;
    static mut E_constr: *mut Agsym_t;
    static mut E_label_float: *mut Agsym_t;
    static mut E_samehead: *mut Agsym_t;
    static mut E_sametail: *mut Agsym_t;
    static mut E_headlabel: *mut Agsym_t;
    static mut E_taillabel: *mut Agsym_t;
    static mut E_labelfontsize: *mut Agsym_t;
    static mut E_labelfontname: *mut Agsym_t;
    static mut E_labelfontcolor: *mut Agsym_t;
    static mut E_labeldistance: *mut Agsym_t;
    static mut E_labelangle: *mut Agsym_t;
    static mut E_tailclip: *mut Agsym_t;
    static mut E_headclip: *mut Agsym_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn updateBB(g: *mut graph_t, lp: *mut textlabel_t);
    fn setEdgeType(g: *mut graph_t, dflt: libc::c_int);
    fn add_box(_: *mut path, _: boxf);
    fn beginpath(
        _: *mut path,
        _: *mut Agedge_t,
        _: libc::c_int,
        _: *mut pathend_t,
        _: bool,
    );
    fn makeStraightEdges(
        g: *mut graph_t,
        edges: *mut *mut edge_t,
        e_cnt: libc::c_int,
        et: libc::c_int,
        sinfo_0: *mut splineInfo,
    );
    fn clip_and_install(
        fe: *mut edge_t,
        hn: *mut node_t,
        ps: *mut pointf,
        pn: libc::c_int,
        info: *mut splineInfo,
    );
    fn dotneato_postprocess(_: *mut Agraph_t);
    fn endpath(
        _: *mut path,
        _: *mut Agedge_t,
        _: libc::c_int,
        _: *mut pathend_t,
        _: bool,
    );
    fn getsplinepoints(e: *mut edge_t) -> *mut splines;
    fn makeSelfEdge(
        edges: *mut *mut edge_t,
        ind: libc::c_int,
        cnt: libc::c_int,
        sizex: libc::c_double,
        sizey: libc::c_double,
        sinfo_0: *mut splineInfo,
    );
    fn new_spline(e: *mut edge_t, sz: libc::c_int) -> *mut bezier;
    fn place_portlabel(e: *mut edge_t, head_p: bool) -> libc::c_int;
    fn routesplinesinit() -> libc::c_int;
    fn routesplines(_: *mut path, _: *mut libc::c_int) -> *mut pointf;
    fn routesplinesterm();
    fn simpleSplineRoute(
        _: pointf,
        _: pointf,
        _: Ppoly_t,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> *mut pointf;
    fn routepolylines(pp: *mut path, npoints: *mut libc::c_int) -> *mut pointf;
    fn shapeOf(_: *mut node_t) -> shape_kind;
    fn update_bb_bz(bb: *mut boxf, cp: *mut pointf);
    fn dot_cleanup(g: *mut graph_t);
    fn dot_init_node_edge(g: *mut graph_t);
    fn mark_lowclusters(_: *mut Agraph_t);
    fn dot_mincross(_: *mut Agraph_t, _: libc::c_int);
    fn dot_position(_: *mut Agraph_t, _: *mut aspect_t);
    fn dot_rank(_: *mut Agraph_t, _: *mut aspect_t);
    fn dot_sameports(_: *mut Agraph_t);
    fn orthoEdges(g: *mut Agraph_t, useLbls: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct Agedgepair_s {
    pub out: Agedge_t,
    pub in_0: Agedge_t,
}
pub type Agedgepair_t = Agedgepair_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
pub type qsort_cmpf = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct splineInfo {
    pub swapEnds: Option::<unsafe extern "C" fn(*mut edge_t) -> bool>,
    pub splineMerge: Option::<unsafe extern "C" fn(*mut node_t) -> bool>,
    pub ignoreSwap: bool,
    pub isOrtho: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pathend_t {
    pub nb: boxf,
    pub np: pointf,
    pub sidemask: libc::c_int,
    pub boxn: libc::c_int,
    pub boxes: [boxf; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path {
    pub start: port,
    pub end: port,
    pub nbox: libc::c_int,
    pub boxes: *mut boxf,
    pub data: *mut libc::c_void,
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
pub type shape_kind = libc::c_uint;
pub const SH_EPSF: shape_kind = 4;
pub const SH_POINT: shape_kind = 3;
pub const SH_RECORD: shape_kind = 2;
pub const SH_POLY: shape_kind = 1;
pub const SH_UNSET: shape_kind = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spline_info_t {
    pub LeftBound: libc::c_int,
    pub RightBound: libc::c_int,
    pub Splinesep: libc::c_int,
    pub Multisep: libc::c_int,
    pub Rank_box: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxes_t {
    pub data: *mut boxf,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_state_t {
    pub E_constr: *mut attrsym_t,
    pub E_samehead: *mut attrsym_t,
    pub E_sametail: *mut attrsym_t,
    pub E_weight: *mut attrsym_t,
    pub E_minlen: *mut attrsym_t,
    pub E_fontcolor: *mut attrsym_t,
    pub E_fontname: *mut attrsym_t,
    pub E_fontsize: *mut attrsym_t,
    pub E_headclip: *mut attrsym_t,
    pub E_headlabel: *mut attrsym_t,
    pub E_label: *mut attrsym_t,
    pub E_label_float: *mut attrsym_t,
    pub E_labelfontcolor: *mut attrsym_t,
    pub E_labelfontname: *mut attrsym_t,
    pub E_labelfontsize: *mut attrsym_t,
    pub E_tailclip: *mut attrsym_t,
    pub E_taillabel: *mut attrsym_t,
    pub E_xlabel: *mut attrsym_t,
    pub N_height: *mut attrsym_t,
    pub N_width: *mut attrsym_t,
    pub N_shape: *mut attrsym_t,
    pub N_style: *mut attrsym_t,
    pub N_fontsize: *mut attrsym_t,
    pub N_fontname: *mut attrsym_t,
    pub N_fontcolor: *mut attrsym_t,
    pub N_label: *mut attrsym_t,
    pub N_xlabel: *mut attrsym_t,
    pub N_showboxes: *mut attrsym_t,
    pub N_ordering: *mut attrsym_t,
    pub N_sides: *mut attrsym_t,
    pub N_peripheries: *mut attrsym_t,
    pub N_skew: *mut attrsym_t,
    pub N_orientation: *mut attrsym_t,
    pub N_distortion: *mut attrsym_t,
    pub N_fixed: *mut attrsym_t,
    pub N_nojustify: *mut attrsym_t,
    pub N_group: *mut attrsym_t,
    pub G_ordering: *mut attrsym_t,
    pub State: libc::c_int,
}
#[inline]
unsafe extern "C" fn pointfof(mut x: libc::c_double, mut y: libc::c_double) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = x;
    r.y = y;
    return r;
}
#[inline]
unsafe extern "C" fn boxfof(
    mut llx: libc::c_double,
    mut lly: libc::c_double,
    mut urx: libc::c_double,
    mut ury: libc::c_double,
) -> boxf {
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    b.LL.x = llx;
    b.LL.y = lly;
    b.UR.x = urx;
    b.UR.y = ury;
    return b;
}
#[inline]
unsafe extern "C" fn add_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x + q.x;
    r.y = p.y + q.y;
    return r;
}
unsafe extern "C" fn boxes_append(mut boxes: *mut boxes_t, mut item: boxf) {
    if !boxes.is_null() {} else {
        __assert_fail(
            b"boxes != NULL\0" as *const u8 as *const libc::c_char,
            b"dotsplines.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void boxes_append(boxes_t *, boxf)\0"))
                .as_ptr(),
        );
    }
    if (*boxes).size == (*boxes).capacity {
        let mut c: size_t = if (*boxes).capacity == 0 as libc::c_int as libc::c_ulong {
            128 as libc::c_int as libc::c_ulong
        } else {
            ((*boxes).capacity).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        };
        let ref mut fresh0 = (*boxes).data;
        *fresh0 = grealloc(
            (*boxes).data as *mut libc::c_void,
            c.wrapping_mul(::std::mem::size_of::<boxf>() as libc::c_ulong),
        ) as *mut boxf;
        (*boxes).capacity = c;
    }
    *((*boxes).data).offset((*boxes).size as isize) = item;
    let ref mut fresh1 = (*boxes).size;
    *fresh1 = (*fresh1).wrapping_add(1);
}
unsafe extern "C" fn boxes_clear(mut boxes: *mut boxes_t) {
    if !boxes.is_null() {} else {
        __assert_fail(
            b"boxes != NULL\0" as *const u8 as *const libc::c_char,
            b"dotsplines.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void boxes_clear(boxes_t *)\0"))
                .as_ptr(),
        );
    }
    (*boxes).size = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn boxes_free(mut boxes: *mut boxes_t) {
    if !boxes.is_null() {} else {
        __assert_fail(
            b"boxes != NULL\0" as *const u8 as *const libc::c_char,
            b"dotsplines.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void boxes_free(boxes_t *)\0"))
                .as_ptr(),
        );
    }
    free((*boxes).data as *mut libc::c_void);
    memset(
        boxes as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<boxes_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn getmainedge(mut e: *mut edge_t) -> *mut edge_t {
    let mut le: *mut edge_t = e;
    while !((*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt).is_null() {
        le = (*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
    }
    while !((*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig).is_null() {
        le = (*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    return le;
}
unsafe extern "C" fn spline_merge(mut n: *mut node_t) -> bool {
    return (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 1 as libc::c_int
        && ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
            > 1 as libc::c_int
            || (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                > 1 as libc::c_int);
}
unsafe extern "C" fn swap_ends_p(mut e: *mut edge_t) -> bool {
    while !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig).is_null() {
        e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
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
        .rank
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
    {
        return 0 as libc::c_int != 0;
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
        .rank
        < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank
    {
        return 1 as libc::c_int != 0;
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
        .order
        >= (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .order
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
static mut sinfo: splineInfo = unsafe {
    {
        let mut init = splineInfo {
            swapEnds: Some(swap_ends_p as unsafe extern "C" fn(*mut edge_t) -> bool),
            splineMerge: Some(spline_merge as unsafe extern "C" fn(*mut node_t) -> bool),
            ignoreSwap: false,
            isOrtho: false,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn portcmp(mut p0: port, mut p1: port) -> libc::c_int {
    if !p1.defined {
        return if p0.defined as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    if !p0.defined {
        return -(1 as libc::c_int);
    }
    if p0.p.x < p1.p.x {
        return -(1 as libc::c_int);
    }
    if p0.p.x > p1.p.x {
        return 1 as libc::c_int;
    }
    if p0.p.y < p1.p.y {
        return -(1 as libc::c_int);
    }
    if p0.p.y > p1.p.y {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn swap_bezier(mut old: *mut bezier, mut new: *mut bezier) {
    let mut list: *mut pointf = 0 as *mut pointf;
    let mut lp: *mut pointf = 0 as *mut pointf;
    let mut olp: *mut pointf = 0 as *mut pointf;
    let mut i: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    sz = (*old).size;
    list = gcalloc(sz as size_t, ::std::mem::size_of::<pointf>() as libc::c_ulong)
        as *mut pointf;
    lp = list;
    olp = ((*old).list).offset((sz - 1 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < sz {
        let fresh2 = olp;
        olp = olp.offset(-1);
        let fresh3 = lp;
        lp = lp.offset(1);
        *fresh3 = *fresh2;
        i += 1;
    }
    let ref mut fresh4 = (*new).list;
    *fresh4 = list;
    (*new).size = sz;
    (*new).sflag = (*old).eflag;
    (*new).eflag = (*old).sflag;
    (*new).sp = (*old).ep;
    (*new).ep = (*old).sp;
}
unsafe extern "C" fn swap_spline(mut s: *mut splines) {
    let mut list: *mut bezier = 0 as *mut bezier;
    let mut lp: *mut bezier = 0 as *mut bezier;
    let mut olp: *mut bezier = 0 as *mut bezier;
    let mut i: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    sz = (*s).size;
    list = gcalloc(sz as size_t, ::std::mem::size_of::<bezier>() as libc::c_ulong)
        as *mut bezier;
    lp = list;
    olp = ((*s).list).offset((sz - 1 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < sz {
        let fresh5 = olp;
        olp = olp.offset(-1);
        let fresh6 = lp;
        lp = lp.offset(1);
        swap_bezier(fresh5, fresh6);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < sz {
        free((*((*s).list).offset(i as isize)).list as *mut libc::c_void);
        i += 1;
    }
    free((*s).list as *mut libc::c_void);
    let ref mut fresh7 = (*s).list;
    *fresh7 = list;
}
unsafe extern "C" fn edge_normalize(mut g: *mut graph_t) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if (sinfo.swapEnds).expect("non-null function pointer")(e) as libc::c_int
                != 0
                && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
            {
                swap_spline((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl);
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn resetRW(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    while !n.is_null() {
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).other.list).is_null()
        {
            let mut tmp: libc::c_double = (*((*(n as *mut Agobj_t)).data
                as *mut Agnodeinfo_t))
                .rw;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .rw = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval = tmp;
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn setEdgeLabelPos(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut l: *mut textlabel_t = 0 as *mut textlabel_t;
    n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 1 as libc::c_int
        {
            if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg).is_null() {
                let mut fe: *mut edge_t = (*((*(n as *mut Agobj_t)).data
                    as *mut Agnodeinfo_t))
                    .alg as *mut edge_t;
                l = (*((*(fe as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
                if !l.is_null() {} else {
                    __assert_fail(
                        b"l\0" as *const u8 as *const libc::c_char,
                        b"dotsplines.c\0" as *const u8 as *const libc::c_char,
                        295 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 32],
                            &[libc::c_char; 32],
                        >(b"void setEdgeLabelPos(graph_t *)\0"))
                            .as_ptr(),
                    );
                }
                (*l).pos = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
                (*l).set = 1 as libc::c_int != 0;
            } else {
                l = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label;
                if !l.is_null() {
                    place_vnlabel(n);
                }
            }
            if !l.is_null() {
                updateBB(g, l);
            }
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
}
unsafe extern "C" fn _dot_splines(mut g: *mut graph_t, mut normalize: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n_nodes: libc::c_int = 0;
    let mut n_edges: libc::c_int = 0;
    let mut ind: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut fwdedgeai: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedgebi: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedgea: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut fwdedgeb: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut e1: *mut edge_t = 0 as *mut edge_t;
    let mut ea: *mut edge_t = 0 as *mut edge_t;
    let mut eb: *mut edge_t = 0 as *mut edge_t;
    let mut le0: *mut edge_t = 0 as *mut edge_t;
    let mut le1: *mut edge_t = 0 as *mut edge_t;
    let mut edges: *mut *mut edge_t = 0 as *mut *mut edge_t;
    let mut P: *mut path = 0 as *mut path;
    let mut sd: spline_info_t = spline_info_t {
        LeftBound: 0,
        RightBound: 0,
        Splinesep: 0,
        Multisep: 0,
        Rank_box: 0 as *mut boxf,
    };
    let mut et: libc::c_int = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).flags
        as libc::c_int & (7 as libc::c_int) << 1 as libc::c_int;
    fwdedgea.out.base.data = &mut fwdedgeai as *mut Agedgeinfo_t as *mut Agrec_t;
    fwdedgeb.out.base.data = &mut fwdedgebi as *mut Agedgeinfo_t as *mut Agrec_t;
    if et == (0 as libc::c_int) << 1 as libc::c_int {
        return;
    }
    if et == (2 as libc::c_int) << 1 as libc::c_int {
        resetRW(g);
        if (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels
            as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
            agerr(
                AGWARN,
                b"edge labels with splines=curved not supported in dot - use xlabels\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if et == (4 as libc::c_int) << 1 as libc::c_int {
        resetRW(g);
        if (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels
            as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
            setEdgeLabelPos(g);
            orthoEdges(g, 1 as libc::c_int);
        } else {
            orthoEdges(g, 0 as libc::c_int);
        }
    } else {
        mark_lowclusters(g);
        if routesplinesinit() != 0 {
            return;
        }
        P = zmalloc(::std::mem::size_of::<path>() as libc::c_ulong) as *mut path;
        sd
            .Splinesep = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep
            / 4 as libc::c_int;
        sd.Multisep = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep;
        edges = gcalloc(
            128 as libc::c_int as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ) as *mut *mut edge_t;
        sd.RightBound = 0 as libc::c_int;
        sd.LeftBound = sd.RightBound;
        n_nodes = 0 as libc::c_int;
        n_edges = n_nodes;
        i = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank;
        while i <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
            n_nodes
                += (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(i as isize))
                    .n;
            n = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(i as isize))
                .v)
                .offset(0 as libc::c_int as isize);
            if !n.is_null() {
                sd
                    .LeftBound = (if (sd.LeftBound as libc::c_double)
                    < (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                {
                    sd.LeftBound as libc::c_double
                } else {
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
                }) as libc::c_int;
            }
            if (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(i as isize))
                .n != 0
                && {
                    n = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                        .offset(i as isize))
                        .v)
                        .offset(
                            ((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t))
                                .rank)
                                .offset(i as isize))
                                .n - 1 as libc::c_int) as isize,
                        );
                    !n.is_null()
                }
            {
                sd
                    .RightBound = (if sd.RightBound as libc::c_double
                    > (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                {
                    sd.RightBound as libc::c_double
                } else {
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
                }) as libc::c_int;
            }
            sd.LeftBound -= 16 as libc::c_int;
            sd.RightBound += 16 as libc::c_int;
            j = 0 as libc::c_int;
            while j
                < (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(i as isize))
                    .n
            {
                n = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(i as isize))
                    .v)
                    .offset(j as isize);
                if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).alg).is_null()
                {
                    let mut fe: *mut edge_t = (*((*(n as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .alg as *mut edge_t;
                    if !((*((*(fe as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .is_null()
                    {} else {
                        __assert_fail(
                            b"ED_label(fe)\0" as *const u8 as *const libc::c_char,
                            b"dotsplines.c\0" as *const u8 as *const libc::c_char,
                            376 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void _dot_splines(graph_t *, int)\0"))
                                .as_ptr(),
                        );
                    }
                    (*(*((*(fe as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .pos = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord;
                    (*(*((*(fe as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .set = 1 as libc::c_int != 0;
                }
                if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int != 0 as libc::c_int
                    && !(sinfo.splineMerge).expect("non-null function pointer")(n))
                {
                    k = 0 as libc::c_int;
                    loop {
                        e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .out
                            .list)
                            .offset(k as isize);
                        if e.is_null() {
                            break;
                        }
                        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .edge_type as libc::c_int == 4 as libc::c_int
                            || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .edge_type as libc::c_int == 6 as libc::c_int)
                        {
                            setflags(
                                e,
                                1 as libc::c_int,
                                16 as libc::c_int,
                                64 as libc::c_int,
                            );
                            let fresh8 = n_edges;
                            n_edges = n_edges + 1;
                            let ref mut fresh9 = *edges.offset(fresh8 as isize);
                            *fresh9 = e;
                            if n_edges % 128 as libc::c_int == 0 as libc::c_int {
                                edges = if !edges.is_null() {
                                    grealloc(
                                        edges as *mut libc::c_void,
                                        ((n_edges + 128 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                            ),
                                    ) as *mut *mut edge_t
                                } else {
                                    gmalloc(
                                        ((n_edges + 128 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                            ),
                                    ) as *mut *mut edge_t
                                };
                            }
                        }
                        k += 1;
                    }
                    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .flat_out
                        .list)
                        .is_null()
                    {
                        k = 0 as libc::c_int;
                        loop {
                            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .flat_out
                                .list)
                                .offset(k as isize);
                            if e.is_null() {
                                break;
                            }
                            setflags(
                                e,
                                2 as libc::c_int,
                                0 as libc::c_int,
                                128 as libc::c_int,
                            );
                            let fresh10 = n_edges;
                            n_edges = n_edges + 1;
                            let ref mut fresh11 = *edges.offset(fresh10 as isize);
                            *fresh11 = e;
                            if n_edges % 128 as libc::c_int == 0 as libc::c_int {
                                edges = if !edges.is_null() {
                                    grealloc(
                                        edges as *mut libc::c_void,
                                        ((n_edges + 128 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                            ),
                                    ) as *mut *mut edge_t
                                } else {
                                    gmalloc(
                                        ((n_edges + 128 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                            ),
                                    ) as *mut *mut edge_t
                                };
                            }
                            k += 1;
                        }
                    }
                    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .other
                        .list)
                        .is_null()
                    {
                        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .node_type as libc::c_int == 0 as libc::c_int
                        {
                            let mut tmp: libc::c_double = (*((*(n as *mut Agobj_t)).data
                                as *mut Agnodeinfo_t))
                                .rw;
                            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .rw = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .mval;
                            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .mval = tmp;
                        }
                        k = 0 as libc::c_int;
                        loop {
                            e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .other
                                .list)
                                .offset(k as isize);
                            if e.is_null() {
                                break;
                            }
                            setflags(
                                e,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                128 as libc::c_int,
                            );
                            let fresh12 = n_edges;
                            n_edges = n_edges + 1;
                            let ref mut fresh13 = *edges.offset(fresh12 as isize);
                            *fresh13 = e;
                            if n_edges % 128 as libc::c_int == 0 as libc::c_int {
                                edges = if !edges.is_null() {
                                    grealloc(
                                        edges as *mut libc::c_void,
                                        ((n_edges + 128 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                            ),
                                    ) as *mut *mut edge_t
                                } else {
                                    gmalloc(
                                        ((n_edges + 128 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                                            ),
                                    ) as *mut *mut edge_t
                                };
                            }
                            k += 1;
                        }
                    }
                }
                j += 1;
            }
            i += 1;
        }
        qsort(
            edges as *mut libc::c_void,
            n_edges as size_t,
            ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut *mut Agedge_t,
                        *mut *mut Agedge_t,
                    ) -> libc::c_int,
                >,
                qsort_cmpf,
            >(
                Some(
                    edgecmp
                        as unsafe extern "C" fn(
                            *mut *mut Agedge_t,
                            *mut *mut Agedge_t,
                        ) -> libc::c_int,
                ),
            ),
        );
        let ref mut fresh14 = (*P).boxes;
        *fresh14 = gcalloc(
            (n_nodes + 20 as libc::c_int * 2 as libc::c_int * 9 as libc::c_int)
                as size_t,
            ::std::mem::size_of::<boxf>() as libc::c_ulong,
        ) as *mut boxf;
        sd
            .Rank_box = gcalloc(
            i as size_t,
            ::std::mem::size_of::<boxf>() as libc::c_ulong,
        ) as *mut boxf;
        if et == (1 as libc::c_int) << 1 as libc::c_int {
            n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
            while !n.is_null() {
                if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 1 as libc::c_int
                    && !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
                        .is_null()
                {
                    place_vnlabel(n);
                }
                n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
            }
        }
        i = 0 as libc::c_int;
        while i < n_edges {
            ind = i;
            let fresh15 = i;
            i = i + 1;
            e0 = *edges.offset(fresh15 as isize);
            le0 = getmainedge(e0);
            if (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
                as libc::c_int != 0
                || (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .head_port
                    .defined as libc::c_int != 0
            {
                ea = e0;
            } else {
                ea = le0;
            }
            if (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
                & 32 as libc::c_int != 0
            {
                let mut newp: *mut edge_t = 0 as *mut edge_t;
                let mut info: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
                newp = &mut fwdedgea.out;
                info = (*newp).base.data as *mut Agedgeinfo_t;
                *info = *((*ea).base.data as *mut Agedgeinfo_t);
                *newp = *ea;
                let ref mut fresh16 = (*newp).base.data;
                *fresh16 = info as *mut Agrec_t;
                let ref mut fresh17 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 3 as libc::c_int
                {
                    newp
                } else {
                    newp.offset(1 as libc::c_int as isize)
                })
                    .node;
                *fresh17 = (*if ((*(ea as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    ea
                } else {
                    ea.offset(-(1 as libc::c_int as isize))
                })
                    .node;
                let ref mut fresh18 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
                    as libc::c_int == 2 as libc::c_int
                {
                    newp
                } else {
                    newp.offset(-(1 as libc::c_int as isize))
                })
                    .node;
                *fresh18 = (*if ((*(ea as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    ea
                } else {
                    ea.offset(1 as libc::c_int as isize)
                })
                    .node;
                (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .tail_port = (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .head_port;
                (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .head_port = (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .tail_port;
                (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                    .edge_type = 1 as libc::c_int as libc::c_char;
                let ref mut fresh19 = (*((*(newp as *mut Agobj_t)).data
                    as *mut Agedgeinfo_t))
                    .to_orig;
                *fresh19 = ea;
                ea = &mut fwdedgea.out;
            }
            cnt = 1 as libc::c_int;
            while i < n_edges {
                e1 = *edges.offset(i as isize);
                le1 = getmainedge(e1);
                if le0 != le1 {
                    break;
                }
                if !((*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).adjacent
                    != 0)
                {
                    if (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .tail_port
                        .defined as libc::c_int != 0
                        || (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .head_port
                            .defined as libc::c_int != 0
                    {
                        eb = e1;
                    } else {
                        eb = le1;
                    }
                    if (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
                        & 32 as libc::c_int != 0
                    {
                        let mut newp_0: *mut edge_t = 0 as *mut edge_t;
                        let mut info_0: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
                        newp_0 = &mut fwdedgeb.out;
                        info_0 = (*newp_0).base.data as *mut Agedgeinfo_t;
                        *info_0 = *((*eb).base.data as *mut Agedgeinfo_t);
                        *newp_0 = *eb;
                        let ref mut fresh20 = (*newp_0).base.data;
                        *fresh20 = info_0 as *mut Agrec_t;
                        let ref mut fresh21 = (*if ((*(newp_0 as *mut Agobj_t)).tag)
                            .objtype() as libc::c_int == 3 as libc::c_int
                        {
                            newp_0
                        } else {
                            newp_0.offset(1 as libc::c_int as isize)
                        })
                            .node;
                        *fresh21 = (*if ((*(eb as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            eb
                        } else {
                            eb.offset(-(1 as libc::c_int as isize))
                        })
                            .node;
                        let ref mut fresh22 = (*if ((*(newp_0 as *mut Agobj_t)).tag)
                            .objtype() as libc::c_int == 2 as libc::c_int
                        {
                            newp_0
                        } else {
                            newp_0.offset(-(1 as libc::c_int as isize))
                        })
                            .node;
                        *fresh22 = (*if ((*(eb as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            eb
                        } else {
                            eb.offset(1 as libc::c_int as isize)
                        })
                            .node;
                        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .tail_port = (*((*(eb as *mut Agobj_t)).data
                            as *mut Agedgeinfo_t))
                            .head_port;
                        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .head_port = (*((*(eb as *mut Agobj_t)).data
                            as *mut Agedgeinfo_t))
                            .tail_port;
                        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                            .edge_type = 1 as libc::c_int as libc::c_char;
                        let ref mut fresh23 = (*((*(newp_0 as *mut Agobj_t)).data
                            as *mut Agedgeinfo_t))
                            .to_orig;
                        *fresh23 = eb;
                        eb = &mut fwdedgeb.out;
                    }
                    if portcmp(
                        (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port,
                        (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port,
                    ) != 0
                    {
                        break;
                    }
                    if portcmp(
                        (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port,
                        (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port,
                    ) != 0
                    {
                        break;
                    }
                    if (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
                        & 15 as libc::c_int == 2 as libc::c_int
                        && (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label
                            != (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                .label
                    {
                        break;
                    }
                    if (*((*(*edges.offset(i as isize) as *mut Agobj_t)).data
                        as *mut Agedgeinfo_t))
                        .tree_index & 64 as libc::c_int != 0
                    {
                        break;
                    }
                }
                cnt += 1;
                i += 1;
            }
            if et == (2 as libc::c_int) << 1 as libc::c_int {
                let mut ii: libc::c_int = 0;
                let mut e0_0: *mut edge_t = 0 as *mut edge_t;
                let mut edgelist: *mut *mut edge_t = 0 as *mut *mut edge_t;
                if cnt == 1 as libc::c_int {
                    edgelist = &mut e0_0;
                } else {
                    edgelist = gcalloc(
                        cnt as size_t,
                        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
                    ) as *mut *mut edge_t;
                }
                let ref mut fresh24 = *edgelist.offset(0 as libc::c_int as isize);
                *fresh24 = getmainedge(
                    *edges.offset(ind as isize).offset(0 as libc::c_int as isize),
                );
                ii = 1 as libc::c_int;
                while ii < cnt {
                    let ref mut fresh25 = *edgelist.offset(ii as isize);
                    *fresh25 = *edges.offset(ind as isize).offset(ii as isize);
                    ii += 1;
                }
                makeStraightEdges(g, edgelist, cnt, et, &mut sinfo);
                if cnt > 1 as libc::c_int {
                    free(edgelist as *mut libc::c_void);
                }
            } else if (*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e0
                } else {
                    e0.offset(1 as libc::c_int as isize)
                }))
                    .node
                    == (*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e0
                    } else {
                        e0.offset(-(1 as libc::c_int as isize))
                    }))
                        .node
                {
                let mut b: libc::c_int = 0;
                let mut sizey: libc::c_int = 0;
                let mut r: libc::c_int = 0;
                n = (*if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e0
                } else {
                    e0.offset(1 as libc::c_int as isize)
                })
                    .node;
                r = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
                if r == (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
                    if r > 0 as libc::c_int {
                        sizey = ((*((*(*((*((*((*(g as *mut Agobj_t)).data
                            as *mut Agraphinfo_t))
                            .rank)
                            .offset((r - 1 as libc::c_int) as isize))
                            .v)
                            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .coord
                            .y
                            - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                                .coord
                                .y) as libc::c_int;
                    } else {
                        sizey = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                            as libc::c_int;
                    }
                } else if r
                        == (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).minrank
                    {
                    sizey = ((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord
                        .y
                        - (*((*(*((*((*((*(g as *mut Agobj_t)).data
                            as *mut Agraphinfo_t))
                            .rank)
                            .offset((r + 1 as libc::c_int) as isize))
                            .v)
                            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .coord
                            .y) as libc::c_int;
                } else {
                    let mut upy: libc::c_int = ((*((*(*((*((*((*(g as *mut Agobj_t)).data
                        as *mut Agraphinfo_t))
                        .rank)
                        .offset((r - 1 as libc::c_int) as isize))
                        .v)
                        .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .coord
                        .y
                        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y)
                        as libc::c_int;
                    let mut dwny: libc::c_int = ((*((*(n as *mut Agobj_t)).data
                        as *mut Agnodeinfo_t))
                        .coord
                        .y
                        - (*((*(*((*((*((*(g as *mut Agobj_t)).data
                            as *mut Agraphinfo_t))
                            .rank)
                            .offset((r + 1 as libc::c_int) as isize))
                            .v)
                            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .coord
                            .y) as libc::c_int;
                    sizey = if upy < dwny { upy } else { dwny };
                }
                makeSelfEdge(
                    edges,
                    ind,
                    cnt,
                    sd.Multisep as libc::c_double,
                    (sizey / 2 as libc::c_int) as libc::c_double,
                    &mut sinfo,
                );
                b = 0 as libc::c_int;
                while b < cnt {
                    e = *edges.offset((ind + b) as isize);
                    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .is_null()
                    {
                        updateBB(
                            g,
                            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label,
                        );
                    }
                    b += 1;
                }
            } else if (*((*((*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e0
                } else {
                    e0.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .rank
                    == (*((*((*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e0
                    } else {
                        e0.offset(-(1 as libc::c_int as isize))
                    }))
                        .node as *mut Agobj_t))
                        .data as *mut Agnodeinfo_t))
                        .rank
                {
                make_flat_edge(g, &mut sd, P, edges, ind, cnt, et);
            } else {
                make_regular_edge(g, &mut sd, P, edges, ind, cnt, et);
            }
        }
        n = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
        while !n.is_null() {
            if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                as libc::c_int == 1 as libc::c_int
                && !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label)
                    .is_null()
            {
                place_vnlabel(n);
                updateBB(g, (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label);
            }
            n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
        }
        if normalize != 0 {
            edge_normalize(g);
        }
    }
    if (!E_headlabel.is_null() || !E_taillabel.is_null())
        && (!E_labelangle.is_null() || !E_labeldistance.is_null())
    {
        n = agfstnode(g);
        while !n.is_null() {
            if !E_headlabel.is_null() {
                e = agfstin(g, n);
                while !e.is_null() {
                    if !((*((*((if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }) as *mut Agobj_t))
                        .data as *mut Agedgeinfo_t))
                        .head_label)
                        .is_null()
                    {
                        place_portlabel(
                            if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                                == 2 as libc::c_int
                            {
                                e
                            } else {
                                e.offset(-(1 as libc::c_int as isize))
                            },
                            1 as libc::c_int != 0,
                        );
                        updateBB(
                            g,
                            (*((*((if ((*(e as *mut Agobj_t)).tag).objtype()
                                as libc::c_int == 2 as libc::c_int
                            {
                                e
                            } else {
                                e.offset(-(1 as libc::c_int as isize))
                            }) as *mut Agobj_t))
                                .data as *mut Agedgeinfo_t))
                                .head_label,
                        );
                    }
                    e = agnxtin(g, e);
                }
            }
            if !E_taillabel.is_null() {
                e = agfstout(g, n);
                while !e.is_null() {
                    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .tail_label)
                        .is_null()
                    {
                        if place_portlabel(e, 0 as libc::c_int != 0) != 0 {
                            updateBB(
                                g,
                                (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                                    .tail_label,
                            );
                        }
                    }
                    e = agnxtout(g, e);
                }
            }
            n = agnxtnode(g, n);
        }
    }
    if et != (4 as libc::c_int) << 1 as libc::c_int
        && et != (2 as libc::c_int) << 1 as libc::c_int
    {
        free(edges as *mut libc::c_void);
        free((*P).boxes as *mut libc::c_void);
        free(P as *mut libc::c_void);
        free(sd.Rank_box as *mut libc::c_void);
        routesplinesterm();
    }
    State = 1 as libc::c_int;
    EdgeLabelsDone = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dot_splines(mut g: *mut graph_t) {
    _dot_splines(g, 1 as libc::c_int);
}
unsafe extern "C" fn place_vnlabel(mut n: *mut node_t) {
    let mut dimen: pointf = pointf { x: 0., y: 0. };
    let mut width: libc::c_double = 0.;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
        == 0 as libc::c_int
    {
        return;
    }
    e = *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
        .offset(0 as libc::c_int as isize);
    while (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type as libc::c_int
        != 0 as libc::c_int
    {
        e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    dimen = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen;
    width = if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
    {
        dimen.y
    } else {
        dimen.x
    };
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
        .pos
        .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        + width / 2.0f64;
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
        .pos
        .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
        .set = 1 as libc::c_int != 0;
}
unsafe extern "C" fn setflags(
    mut e: *mut edge_t,
    mut hint1: libc::c_int,
    mut hint2: libc::c_int,
    mut f3: libc::c_int,
) {
    let mut f1: libc::c_int = 0;
    let mut f2: libc::c_int = 0;
    if hint1 != 0 as libc::c_int {
        f1 = hint1;
    } else if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node
            == (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node
        {
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
            as libc::c_int != 0
            || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
                as libc::c_int != 0
        {
            f1 = 4 as libc::c_int;
        } else {
            f1 = 8 as libc::c_int;
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
            .rank
            == (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .rank
        {
        f1 = 2 as libc::c_int;
    } else {
        f1 = 1 as libc::c_int;
    }
    if hint2 != 0 as libc::c_int {
        f2 = hint2;
    } else if f1 == 1 as libc::c_int {
        f2 = if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank
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
        {
            16 as libc::c_int
        } else {
            32 as libc::c_int
        };
    } else if f1 == 2 as libc::c_int {
        f2 = if (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .order
            < (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order
        {
            16 as libc::c_int
        } else {
            32 as libc::c_int
        };
    } else {
        f2 = 16 as libc::c_int;
    }
    (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index = f1 | f2 | f3;
}
unsafe extern "C" fn edgecmp(
    mut ptr0: *mut *mut edge_t,
    mut ptr1: *mut *mut edge_t,
) -> libc::c_int {
    let mut fwdedgeai: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedgebi: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedgea: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut fwdedgeb: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut e1: *mut edge_t = 0 as *mut edge_t;
    let mut ea: *mut edge_t = 0 as *mut edge_t;
    let mut eb: *mut edge_t = 0 as *mut edge_t;
    let mut le0: *mut edge_t = 0 as *mut edge_t;
    let mut le1: *mut edge_t = 0 as *mut edge_t;
    let mut et0: libc::c_int = 0;
    let mut et1: libc::c_int = 0;
    let mut v0: libc::c_int = 0;
    let mut v1: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut t0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    fwdedgea.out.base.data = &mut fwdedgeai as *mut Agedgeinfo_t as *mut Agrec_t;
    fwdedgeb.out.base.data = &mut fwdedgebi as *mut Agedgeinfo_t as *mut Agrec_t;
    e0 = *ptr0;
    e1 = *ptr1;
    et0 = (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        & 15 as libc::c_int;
    et1 = (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        & 15 as libc::c_int;
    if et0 != et1 {
        return et1 - et0;
    }
    le0 = getmainedge(e0);
    le1 = getmainedge(e1);
    t0 = ((*((*((*(if ((*(le0 as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        le0
    } else {
        le0.offset(1 as libc::c_int as isize)
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .rank
        - (*((*((*(if ((*(le0 as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            le0
        } else {
            le0.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank) as libc::c_double;
    t1 = ((*((*((*(if ((*(le1 as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        le1
    } else {
        le1.offset(1 as libc::c_int as isize)
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .rank
        - (*((*((*(if ((*(le1 as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            le1
        } else {
            le1.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .rank) as libc::c_double;
    v0 = abs(t0 as libc::c_int);
    v1 = abs(t1 as libc::c_int);
    if v0 != v1 {
        return v0 - v1;
    }
    t0 = (*((*((*(if ((*(le0 as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        le0
    } else {
        le0.offset(1 as libc::c_int as isize)
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord
        .x
        - (*((*((*(if ((*(le0 as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            le0
        } else {
            le0.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord
            .x;
    t1 = (*((*((*(if ((*(le1 as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        le1
    } else {
        le1.offset(1 as libc::c_int as isize)
    }))
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord
        .x
        - (*((*((*(if ((*(le1 as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            le1
        } else {
            le1.offset(-(1 as libc::c_int as isize))
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord
            .x;
    v0 = abs(t0 as libc::c_int);
    v1 = abs(t1 as libc::c_int);
    if v0 != v1 {
        return v0 - v1;
    }
    if ((*(le0 as *mut Agobj_t)).tag).seq() as libc::c_int
        != ((*(le1 as *mut Agobj_t)).tag).seq() as libc::c_int
    {
        return ((*(le0 as *mut Agobj_t)).tag).seq() as libc::c_int
            - ((*(le1 as *mut Agobj_t)).tag).seq() as libc::c_int;
    }
    ea = if (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
        as libc::c_int != 0
        || (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
            as libc::c_int != 0
    {
        e0
    } else {
        le0
    };
    if (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        & 32 as libc::c_int != 0
    {
        let mut newp: *mut edge_t = 0 as *mut edge_t;
        let mut info: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
        newp = &mut fwdedgea.out;
        info = (*newp).base.data as *mut Agedgeinfo_t;
        *info = *((*ea).base.data as *mut Agedgeinfo_t);
        *newp = *ea;
        let ref mut fresh26 = (*newp).base.data;
        *fresh26 = info as *mut Agrec_t;
        let ref mut fresh27 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            newp
        } else {
            newp.offset(1 as libc::c_int as isize)
        })
            .node;
        *fresh27 = (*if ((*(ea as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            ea
        } else {
            ea.offset(-(1 as libc::c_int as isize))
        })
            .node;
        let ref mut fresh28 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 2 as libc::c_int
        {
            newp
        } else {
            newp.offset(-(1 as libc::c_int as isize))
        })
            .node;
        *fresh28 = (*if ((*(ea as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            ea
        } else {
            ea.offset(1 as libc::c_int as isize)
        })
            .node;
        (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port = (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port;
        (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port = (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port;
        (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .edge_type = 1 as libc::c_int as libc::c_char;
        let ref mut fresh29 = (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .to_orig;
        *fresh29 = ea;
        ea = &mut fwdedgea.out;
    }
    eb = if (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
        as libc::c_int != 0
        || (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
            as libc::c_int != 0
    {
        e1
    } else {
        le1
    };
    if (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        & 32 as libc::c_int != 0
    {
        let mut newp_0: *mut edge_t = 0 as *mut edge_t;
        let mut info_0: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
        newp_0 = &mut fwdedgeb.out;
        info_0 = (*newp_0).base.data as *mut Agedgeinfo_t;
        *info_0 = *((*eb).base.data as *mut Agedgeinfo_t);
        *newp_0 = *eb;
        let ref mut fresh30 = (*newp_0).base.data;
        *fresh30 = info_0 as *mut Agrec_t;
        let ref mut fresh31 = (*if ((*(newp_0 as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            newp_0
        } else {
            newp_0.offset(1 as libc::c_int as isize)
        })
            .node;
        *fresh31 = (*if ((*(eb as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            eb
        } else {
            eb.offset(-(1 as libc::c_int as isize))
        })
            .node;
        let ref mut fresh32 = (*if ((*(newp_0 as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 2 as libc::c_int
        {
            newp_0
        } else {
            newp_0.offset(-(1 as libc::c_int as isize))
        })
            .node;
        *fresh32 = (*if ((*(eb as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            eb
        } else {
            eb.offset(1 as libc::c_int as isize)
        })
            .node;
        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port = (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port;
        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port = (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port;
        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .edge_type = 1 as libc::c_int as libc::c_char;
        let ref mut fresh33 = (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .to_orig;
        *fresh33 = eb;
        eb = &mut fwdedgeb.out;
    }
    rv = portcmp(
        (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port,
        (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port,
    );
    if rv != 0 {
        return rv;
    }
    rv = portcmp(
        (*((*(ea as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port,
        (*((*(eb as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port,
    );
    if rv != 0 {
        return rv;
    }
    et0 = (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        & 192 as libc::c_int;
    et1 = (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        & 192 as libc::c_int;
    if et0 != et1 {
        return et0 - et1;
    }
    if et0 == 2 as libc::c_int
        && (*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label
            != (*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label
    {
        return ((*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
            .offset_from((*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
            as libc::c_long as libc::c_int;
    }
    return ((*(e0 as *mut Agobj_t)).tag).seq() as libc::c_int
        - ((*(e1 as *mut Agobj_t)).tag).seq() as libc::c_int;
}
unsafe extern "C" fn setState(
    mut auxg: *mut graph_t,
    mut attr_state: *mut attr_state_t,
) {
    let ref mut fresh34 = (*attr_state).E_constr;
    *fresh34 = E_constr;
    let ref mut fresh35 = (*attr_state).E_samehead;
    *fresh35 = E_samehead;
    let ref mut fresh36 = (*attr_state).E_sametail;
    *fresh36 = E_sametail;
    let ref mut fresh37 = (*attr_state).E_weight;
    *fresh37 = E_weight;
    let ref mut fresh38 = (*attr_state).E_minlen;
    *fresh38 = E_minlen;
    let ref mut fresh39 = (*attr_state).E_fontcolor;
    *fresh39 = E_fontcolor;
    let ref mut fresh40 = (*attr_state).E_fontname;
    *fresh40 = E_fontname;
    let ref mut fresh41 = (*attr_state).E_fontsize;
    *fresh41 = E_fontsize;
    let ref mut fresh42 = (*attr_state).E_headclip;
    *fresh42 = E_headclip;
    let ref mut fresh43 = (*attr_state).E_headlabel;
    *fresh43 = E_headlabel;
    let ref mut fresh44 = (*attr_state).E_label;
    *fresh44 = E_label;
    let ref mut fresh45 = (*attr_state).E_label_float;
    *fresh45 = E_label_float;
    let ref mut fresh46 = (*attr_state).E_labelfontcolor;
    *fresh46 = E_labelfontcolor;
    let ref mut fresh47 = (*attr_state).E_labelfontname;
    *fresh47 = E_labelfontname;
    let ref mut fresh48 = (*attr_state).E_labelfontsize;
    *fresh48 = E_labelfontsize;
    let ref mut fresh49 = (*attr_state).E_tailclip;
    *fresh49 = E_tailclip;
    let ref mut fresh50 = (*attr_state).E_taillabel;
    *fresh50 = E_taillabel;
    let ref mut fresh51 = (*attr_state).E_xlabel;
    *fresh51 = E_xlabel;
    let ref mut fresh52 = (*attr_state).N_height;
    *fresh52 = N_height;
    let ref mut fresh53 = (*attr_state).N_width;
    *fresh53 = N_width;
    let ref mut fresh54 = (*attr_state).N_shape;
    *fresh54 = N_shape;
    let ref mut fresh55 = (*attr_state).N_style;
    *fresh55 = N_style;
    let ref mut fresh56 = (*attr_state).N_fontsize;
    *fresh56 = N_fontsize;
    let ref mut fresh57 = (*attr_state).N_fontname;
    *fresh57 = N_fontname;
    let ref mut fresh58 = (*attr_state).N_fontcolor;
    *fresh58 = N_fontcolor;
    let ref mut fresh59 = (*attr_state).N_label;
    *fresh59 = N_label;
    let ref mut fresh60 = (*attr_state).N_xlabel;
    *fresh60 = N_xlabel;
    let ref mut fresh61 = (*attr_state).N_showboxes;
    *fresh61 = N_showboxes;
    let ref mut fresh62 = (*attr_state).N_ordering;
    *fresh62 = N_ordering;
    let ref mut fresh63 = (*attr_state).N_sides;
    *fresh63 = N_sides;
    let ref mut fresh64 = (*attr_state).N_peripheries;
    *fresh64 = N_peripheries;
    let ref mut fresh65 = (*attr_state).N_skew;
    *fresh65 = N_skew;
    let ref mut fresh66 = (*attr_state).N_orientation;
    *fresh66 = N_orientation;
    let ref mut fresh67 = (*attr_state).N_distortion;
    *fresh67 = N_distortion;
    let ref mut fresh68 = (*attr_state).N_fixed;
    *fresh68 = N_fixed;
    let ref mut fresh69 = (*attr_state).N_nojustify;
    *fresh69 = N_nojustify;
    let ref mut fresh70 = (*attr_state).N_group;
    *fresh70 = N_group;
    (*attr_state).State = State;
    let ref mut fresh71 = (*attr_state).G_ordering;
    *fresh71 = G_ordering;
    E_constr = 0 as *mut Agsym_t;
    E_samehead = agattr(
        auxg,
        2 as libc::c_int,
        b"samehead\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_sametail = agattr(
        auxg,
        2 as libc::c_int,
        b"sametail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_weight = agattr(
        auxg,
        2 as libc::c_int,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if E_weight.is_null() {
        E_weight = agattr(
            auxg,
            2 as libc::c_int,
            b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    E_minlen = 0 as *mut Agsym_t;
    E_fontcolor = 0 as *mut Agsym_t;
    E_fontname = agattr(
        auxg,
        2 as libc::c_int,
        b"fontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_fontsize = agattr(
        auxg,
        2 as libc::c_int,
        b"fontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_headclip = agattr(
        auxg,
        2 as libc::c_int,
        b"headclip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_headlabel = 0 as *mut Agsym_t;
    E_label = agattr(
        auxg,
        2 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_label_float = agattr(
        auxg,
        2 as libc::c_int,
        b"label_float\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_labelfontcolor = 0 as *mut Agsym_t;
    E_labelfontname = agattr(
        auxg,
        2 as libc::c_int,
        b"labelfontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_labelfontsize = agattr(
        auxg,
        2 as libc::c_int,
        b"labelfontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_tailclip = agattr(
        auxg,
        2 as libc::c_int,
        b"tailclip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    E_taillabel = 0 as *mut Agsym_t;
    E_xlabel = 0 as *mut Agsym_t;
    N_height = agattr(
        auxg,
        1 as libc::c_int,
        b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_width = agattr(
        auxg,
        1 as libc::c_int,
        b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_shape = agattr(
        auxg,
        1 as libc::c_int,
        b"shape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_style = 0 as *mut Agsym_t;
    N_fontsize = agattr(
        auxg,
        1 as libc::c_int,
        b"fontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fontname = agattr(
        auxg,
        1 as libc::c_int,
        b"fontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fontcolor = 0 as *mut Agsym_t;
    N_label = agattr(
        auxg,
        1 as libc::c_int,
        b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_xlabel = 0 as *mut Agsym_t;
    N_showboxes = 0 as *mut Agsym_t;
    N_ordering = agattr(
        auxg,
        1 as libc::c_int,
        b"ordering\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_sides = agattr(
        auxg,
        1 as libc::c_int,
        b"sides\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_peripheries = agattr(
        auxg,
        1 as libc::c_int,
        b"peripheries\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_skew = agattr(
        auxg,
        1 as libc::c_int,
        b"skew\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_orientation = agattr(
        auxg,
        1 as libc::c_int,
        b"orientation\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_distortion = agattr(
        auxg,
        1 as libc::c_int,
        b"distortion\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_fixed = agattr(
        auxg,
        1 as libc::c_int,
        b"fixed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    N_nojustify = 0 as *mut Agsym_t;
    N_group = 0 as *mut Agsym_t;
    G_ordering = agattr(
        auxg,
        0 as libc::c_int,
        b"ordering\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn cloneGraph(
    mut g: *mut graph_t,
    mut attr_state: *mut attr_state_t,
) -> *mut graph_t {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut auxg: *mut graph_t = 0 as *mut graph_t;
    if agisdirected(g) != 0 {
        auxg = agopen(
            b"auxg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Agdirected,
            0 as *mut Agdisc_t,
        );
    } else {
        auxg = agopen(
            b"auxg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Agundirected,
            0 as *mut Agdisc_t,
        );
    }
    agbindrec(
        auxg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agattr(
        auxg,
        0 as libc::c_int,
        b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh72 = (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .drawing;
    *fresh72 = zmalloc(::std::mem::size_of::<layout_t>() as libc::c_ulong)
        as *mut layout_t;
    (*(*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .quantum = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .quantum;
    (*(*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .dpi = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).dpi;
    (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .charset = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).charset;
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir & 0x3 as libc::c_int
        & 1 as libc::c_int != 0
    {
        (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rankdir = 0 as libc::c_int;
    } else {
        (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t))
            .rankdir = 1 as libc::c_int;
    }
    (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .nodesep = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep;
    (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .ranksep = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ranksep;
    sym = agnxtattr(agroot(g as *mut libc::c_void), 1 as libc::c_int, 0 as *mut Agsym_t);
    while !sym.is_null() {
        agattr(auxg, 1 as libc::c_int, (*sym).name, (*sym).defval);
        sym = agnxtattr(agroot(g as *mut libc::c_void), 1 as libc::c_int, sym);
    }
    sym = agnxtattr(agroot(g as *mut libc::c_void), 2 as libc::c_int, 0 as *mut Agsym_t);
    while !sym.is_null() {
        agattr(auxg, 2 as libc::c_int, (*sym).name, (*sym).defval);
        sym = agnxtattr(agroot(g as *mut libc::c_void), 2 as libc::c_int, sym);
    }
    if (agattr(
        auxg,
        2 as libc::c_int,
        b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    ))
        .is_null()
    {
        agattr(
            auxg,
            2 as libc::c_int,
            b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if (agattr(
        auxg,
        2 as libc::c_int,
        b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    ))
        .is_null()
    {
        agattr(
            auxg,
            2 as libc::c_int,
            b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    setState(auxg, attr_state);
    return auxg;
}
unsafe extern "C" fn cleanupCloneGraph(
    mut g: *mut graph_t,
    mut attr_state: *mut attr_state_t,
) {
    E_constr = (*attr_state).E_constr;
    E_samehead = (*attr_state).E_samehead;
    E_sametail = (*attr_state).E_sametail;
    E_weight = (*attr_state).E_weight;
    E_minlen = (*attr_state).E_minlen;
    E_fontcolor = (*attr_state).E_fontcolor;
    E_fontname = (*attr_state).E_fontname;
    E_fontsize = (*attr_state).E_fontsize;
    E_headclip = (*attr_state).E_headclip;
    E_headlabel = (*attr_state).E_headlabel;
    E_label = (*attr_state).E_label;
    E_label_float = (*attr_state).E_label_float;
    E_labelfontcolor = (*attr_state).E_labelfontcolor;
    E_labelfontname = (*attr_state).E_labelfontname;
    E_labelfontsize = (*attr_state).E_labelfontsize;
    E_tailclip = (*attr_state).E_tailclip;
    E_taillabel = (*attr_state).E_taillabel;
    E_xlabel = (*attr_state).E_xlabel;
    N_height = (*attr_state).N_height;
    N_width = (*attr_state).N_width;
    N_shape = (*attr_state).N_shape;
    N_style = (*attr_state).N_style;
    N_fontsize = (*attr_state).N_fontsize;
    N_fontname = (*attr_state).N_fontname;
    N_fontcolor = (*attr_state).N_fontcolor;
    N_label = (*attr_state).N_label;
    N_xlabel = (*attr_state).N_xlabel;
    N_showboxes = (*attr_state).N_showboxes;
    N_ordering = (*attr_state).N_ordering;
    N_sides = (*attr_state).N_sides;
    N_peripheries = (*attr_state).N_peripheries;
    N_skew = (*attr_state).N_skew;
    N_orientation = (*attr_state).N_orientation;
    N_distortion = (*attr_state).N_distortion;
    N_fixed = (*attr_state).N_fixed;
    N_nojustify = (*attr_state).N_nojustify;
    N_group = (*attr_state).N_group;
    G_ordering = (*attr_state).G_ordering;
    State = (*attr_state).State;
    free(attr_state as *mut libc::c_void);
    dot_cleanup(g);
    agclose(g);
}
unsafe extern "C" fn cloneNode(
    mut g: *mut graph_t,
    mut orign: *mut node_t,
    mut flipped: libc::c_int,
) -> *mut node_t {
    let mut n: *mut node_t = agnode(
        g,
        agnameof(orign as *mut libc::c_void),
        1 as libc::c_int,
    );
    agbindrec(
        n as *mut libc::c_void,
        b"Agnodeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agcopyattr(orign as *mut libc::c_void, n as *mut libc::c_void);
    if shapeOf(orign) as libc::c_uint == SH_RECORD as libc::c_int as libc::c_uint {
        let mut lbllen: libc::c_int = strlen(
            (*(*((*(orign as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).text,
        ) as libc::c_int;
        let mut buf: *mut libc::c_char = gcalloc(
            (lbllen + 3 as libc::c_int) as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        sprintf(
            buf,
            b"{%s}\0" as *const u8 as *const libc::c_char,
            (*(*((*(orign as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).text,
        );
        agset(
            n as *mut libc::c_void,
            b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf,
        );
    }
    return n;
}
unsafe extern "C" fn cloneEdge(
    mut g: *mut graph_t,
    mut tn: *mut node_t,
    mut hn: *mut node_t,
    mut orig: *mut edge_t,
) -> *mut edge_t {
    let mut e: *mut edge_t = agedge(g, tn, hn, 0 as *mut libc::c_char, 1 as libc::c_int);
    agbindrec(
        e as *mut libc::c_void,
        b"Agedgeinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agedgeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agcopyattr(orig as *mut libc::c_void, e as *mut libc::c_void);
    return e;
}
unsafe extern "C" fn transformf(
    mut p: pointf,
    mut del: pointf,
    mut flip: libc::c_int,
) -> pointf {
    if flip != 0 {
        let mut i: libc::c_double = p.x;
        p.x = p.y;
        p.y = -i;
    }
    return add_pointf(p, del);
}
unsafe extern "C" fn edgelblcmpfn(
    mut ptr0: *mut *mut edge_t,
    mut ptr1: *mut *mut edge_t,
) -> libc::c_int {
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut e1: *mut edge_t = 0 as *mut edge_t;
    let mut sz0: pointf = pointf { x: 0., y: 0. };
    let mut sz1: pointf = pointf { x: 0., y: 0. };
    e0 = *ptr0;
    e1 = *ptr1;
    if !((*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
        if !((*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
            sz0 = (*(*((*(e0 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen;
            sz1 = (*(*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen;
            if sz0.x > sz1.x {
                return -(1 as libc::c_int)
            } else if sz0.x < sz1.x {
                return 1 as libc::c_int
            } else if sz0.y > sz1.y {
                return -(1 as libc::c_int)
            } else if sz0.y < sz1.y {
                return 1 as libc::c_int
            } else {
                return 0 as libc::c_int
            }
        } else {
            return -(1 as libc::c_int)
        }
    } else if !((*((*(e1 as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn makeSimpleFlatLabels(
    mut tn: *mut node_t,
    mut hn: *mut node_t,
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut et: libc::c_int,
    mut n_lbls: libc::c_int,
) {
    let mut poly: Ppoly_t = Ppoly_t {
        ps: 0 as *mut Ppoint_t,
        pn: 0,
    };
    let mut pn: libc::c_int = 0;
    let mut e: *mut edge_t = *edges.offset(ind as isize);
    let mut points: [pointf; 10] = [pointf { x: 0., y: 0. }; 10];
    let mut tp: pointf = pointf { x: 0., y: 0. };
    let mut hp: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut pointn: libc::c_int = 0;
    let mut leftend: libc::c_double = 0.;
    let mut rightend: libc::c_double = 0.;
    let mut ctrx: libc::c_double = 0.;
    let mut ctry: libc::c_double = 0.;
    let mut miny: libc::c_double = 0.;
    let mut maxy: libc::c_double = 0.;
    let mut uminx: libc::c_double = 0.;
    let mut umaxx: libc::c_double = 0.;
    let mut lminx: libc::c_double = 0.0f64;
    let mut lmaxx: libc::c_double = 0.0f64;
    let mut earray: *mut *mut edge_t = gcalloc(
        cnt as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
    ) as *mut *mut edge_t;
    i = 0 as libc::c_int;
    while i < cnt {
        let ref mut fresh73 = *earray.offset(i as isize);
        *fresh73 = *edges.offset((ind + i) as isize);
        i += 1;
    }
    qsort(
        earray as *mut libc::c_void,
        cnt as size_t,
        ::std::mem::size_of::<*mut edge_t>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut *mut edge_t, *mut *mut edge_t) -> libc::c_int,
            >,
            qsort_cmpf,
        >(
            Some(
                edgelblcmpfn
                    as unsafe extern "C" fn(
                        *mut *mut edge_t,
                        *mut *mut edge_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    tp = add_pointf(
        (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
    );
    hp = add_pointf(
        (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
    );
    leftend = tp.x + (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
    rightend = hp.x - (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
    ctrx = (leftend + rightend) / 2.0f64;
    e = *earray.offset(0 as libc::c_int as isize);
    pointn = 0 as libc::c_int;
    let fresh74 = pointn;
    pointn = pointn + 1;
    points[fresh74 as usize] = tp;
    let fresh75 = pointn;
    pointn = pointn + 1;
    points[fresh75 as usize] = tp;
    let fresh76 = pointn;
    pointn = pointn + 1;
    points[fresh76 as usize] = hp;
    let fresh77 = pointn;
    pointn = pointn + 1;
    points[fresh77 as usize] = hp;
    clip_and_install(
        e,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
        points.as_mut_ptr(),
        pointn,
        &mut sinfo,
    );
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos.x = ctrx;
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
        .pos
        .y = tp.y
        + ((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.y
            + 6 as libc::c_int as libc::c_double) / 2.0f64;
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
        .set = 1 as libc::c_int != 0;
    miny = tp.y + 6 as libc::c_int as libc::c_double / 2.0f64;
    maxy = miny + (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.y;
    uminx = ctrx
        - (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.x
            / 2.0f64;
    umaxx = ctrx
        + (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.x
            / 2.0f64;
    i = 1 as libc::c_int;
    while i < n_lbls {
        e = *earray.offset(i as isize);
        if i % 2 as libc::c_int != 0 {
            if i == 1 as libc::c_int {
                lminx = ctrx
                    - (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .dimen
                        .x / 2.0f64;
                lmaxx = ctrx
                    + (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .dimen
                        .x / 2.0f64;
            }
            miny
                -= 6 as libc::c_int as libc::c_double
                    + (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .dimen
                        .y;
            points[0 as libc::c_int as usize] = tp;
            points[1 as libc::c_int as usize].x = tp.x;
            points[1 as libc::c_int as usize]
                .y = miny - 6 as libc::c_int as libc::c_double;
            points[2 as libc::c_int as usize].x = hp.x;
            points[2 as libc::c_int as usize].y = points[1 as libc::c_int as usize].y;
            points[3 as libc::c_int as usize] = hp;
            points[4 as libc::c_int as usize].x = lmaxx;
            points[4 as libc::c_int as usize].y = hp.y;
            points[5 as libc::c_int as usize].x = lmaxx;
            points[5 as libc::c_int as usize].y = miny;
            points[6 as libc::c_int as usize].x = lminx;
            points[6 as libc::c_int as usize].y = miny;
            points[7 as libc::c_int as usize].x = lminx;
            points[7 as libc::c_int as usize].y = tp.y;
            ctry = miny
                + (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.y
                    / 2.0f64;
        } else {
            points[0 as libc::c_int as usize] = tp;
            points[1 as libc::c_int as usize].x = uminx;
            points[1 as libc::c_int as usize].y = tp.y;
            points[2 as libc::c_int as usize].x = uminx;
            points[2 as libc::c_int as usize].y = maxy;
            points[3 as libc::c_int as usize].x = umaxx;
            points[3 as libc::c_int as usize].y = maxy;
            points[4 as libc::c_int as usize].x = umaxx;
            points[4 as libc::c_int as usize].y = hp.y;
            points[5 as libc::c_int as usize].x = hp.x;
            points[5 as libc::c_int as usize].y = hp.y;
            points[6 as libc::c_int as usize].x = hp.x;
            points[6 as libc::c_int as usize]
                .y = maxy + 6 as libc::c_int as libc::c_double;
            points[7 as libc::c_int as usize].x = tp.x;
            points[7 as libc::c_int as usize]
                .y = maxy + 6 as libc::c_int as libc::c_double;
            ctry = maxy
                + (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.y
                    / 2.0f64 + 6 as libc::c_int as libc::c_double;
            maxy
                += (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.y
                    + 6 as libc::c_int as libc::c_double;
        }
        poly.pn = 8 as libc::c_int;
        poly.ps = points.as_mut_ptr() as *mut Ppoint_t;
        let mut ps: *mut pointf = simpleSplineRoute(
            tp,
            hp,
            poly,
            &mut pn,
            (et == (3 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
        );
        if pn == 0 as libc::c_int {
            free(ps as *mut libc::c_void);
            return;
        }
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos.x = ctrx;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos.y = ctry;
        (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
            .set = 1 as libc::c_int != 0;
        clip_and_install(
            e,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            ps,
            pn,
            &mut sinfo,
        );
        free(ps as *mut libc::c_void);
        i += 1;
    }
    while i < cnt {
        e = *earray.offset(i as isize);
        if i % 2 as libc::c_int != 0 {
            if i == 1 as libc::c_int {
                lminx = (2 as libc::c_int as libc::c_double * leftend + rightend)
                    / 3.0f64;
                lmaxx = (leftend + 2 as libc::c_int as libc::c_double * rightend)
                    / 3.0f64;
            }
            miny -= 6 as libc::c_int as libc::c_double;
            points[0 as libc::c_int as usize] = tp;
            points[1 as libc::c_int as usize].x = tp.x;
            points[1 as libc::c_int as usize]
                .y = miny - 6 as libc::c_int as libc::c_double;
            points[2 as libc::c_int as usize].x = hp.x;
            points[2 as libc::c_int as usize].y = points[1 as libc::c_int as usize].y;
            points[3 as libc::c_int as usize] = hp;
            points[4 as libc::c_int as usize].x = lmaxx;
            points[4 as libc::c_int as usize].y = hp.y;
            points[5 as libc::c_int as usize].x = lmaxx;
            points[5 as libc::c_int as usize].y = miny;
            points[6 as libc::c_int as usize].x = lminx;
            points[6 as libc::c_int as usize].y = miny;
            points[7 as libc::c_int as usize].x = lminx;
            points[7 as libc::c_int as usize].y = tp.y;
        } else {
            points[0 as libc::c_int as usize] = tp;
            points[1 as libc::c_int as usize].x = uminx;
            points[1 as libc::c_int as usize].y = tp.y;
            points[2 as libc::c_int as usize].x = uminx;
            points[2 as libc::c_int as usize].y = maxy;
            points[3 as libc::c_int as usize].x = umaxx;
            points[3 as libc::c_int as usize].y = maxy;
            points[4 as libc::c_int as usize].x = umaxx;
            points[4 as libc::c_int as usize].y = hp.y;
            points[5 as libc::c_int as usize].x = hp.x;
            points[5 as libc::c_int as usize].y = hp.y;
            points[6 as libc::c_int as usize].x = hp.x;
            points[6 as libc::c_int as usize]
                .y = maxy + 6 as libc::c_int as libc::c_double;
            points[7 as libc::c_int as usize].x = tp.x;
            points[7 as libc::c_int as usize]
                .y = maxy + 6 as libc::c_int as libc::c_double;
            maxy += 6 as libc::c_int as libc::c_double;
        }
        poly.pn = 8 as libc::c_int;
        poly.ps = points.as_mut_ptr() as *mut Ppoint_t;
        let mut ps_0: *mut pointf = simpleSplineRoute(
            tp,
            hp,
            poly,
            &mut pn,
            (et == (3 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
        );
        if pn == 0 as libc::c_int {
            free(ps_0 as *mut libc::c_void);
            return;
        }
        clip_and_install(
            e,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            ps_0,
            pn,
            &mut sinfo,
        );
        free(ps_0 as *mut libc::c_void);
        i += 1;
    }
    free(earray as *mut libc::c_void);
}
unsafe extern "C" fn makeSimpleFlat(
    mut tn: *mut node_t,
    mut hn: *mut node_t,
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut et: libc::c_int,
) {
    let mut e: *mut edge_t = *edges.offset(ind as isize);
    let mut points: [pointf; 10] = [pointf { x: 0., y: 0. }; 10];
    let mut tp: pointf = pointf { x: 0., y: 0. };
    let mut hp: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut pointn: libc::c_int = 0;
    let mut stepy: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    tp = add_pointf(
        (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
    );
    hp = add_pointf(
        (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
    );
    stepy = if cnt > 1 as libc::c_int {
        (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
            / (cnt - 1 as libc::c_int) as libc::c_double
    } else {
        0.0f64
    };
    dy = tp.y
        - (if cnt > 1 as libc::c_int {
            (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64
        } else {
            0.0f64
        });
    i = 0 as libc::c_int;
    while i < cnt {
        e = *edges.offset((ind + i) as isize);
        pointn = 0 as libc::c_int;
        if et == (5 as libc::c_int) << 1 as libc::c_int
            || et == (1 as libc::c_int) << 1 as libc::c_int
        {
            let fresh78 = pointn;
            pointn = pointn + 1;
            points[fresh78 as usize] = tp;
            let fresh79 = pointn;
            pointn = pointn + 1;
            points[fresh79
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * tp.x + hp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh80 = pointn;
            pointn = pointn + 1;
            points[fresh80
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * hp.x + tp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh81 = pointn;
            pointn = pointn + 1;
            points[fresh81 as usize] = hp;
        } else {
            let fresh82 = pointn;
            pointn = pointn + 1;
            points[fresh82 as usize] = tp;
            let fresh83 = pointn;
            pointn = pointn + 1;
            points[fresh83 as usize] = tp;
            let fresh84 = pointn;
            pointn = pointn + 1;
            points[fresh84
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * tp.x + hp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh85 = pointn;
            pointn = pointn + 1;
            points[fresh85
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * tp.x + hp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh86 = pointn;
            pointn = pointn + 1;
            points[fresh86
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * tp.x + hp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh87 = pointn;
            pointn = pointn + 1;
            points[fresh87
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * hp.x + tp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh88 = pointn;
            pointn = pointn + 1;
            points[fresh88
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * hp.x + tp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh89 = pointn;
            pointn = pointn + 1;
            points[fresh89
                as usize] = pointfof(
                (2 as libc::c_int as libc::c_double * hp.x + tp.x)
                    / 3 as libc::c_int as libc::c_double,
                dy,
            );
            let fresh90 = pointn;
            pointn = pointn + 1;
            points[fresh90 as usize] = hp;
            let fresh91 = pointn;
            pointn = pointn + 1;
            points[fresh91 as usize] = hp;
        }
        dy += stepy;
        clip_and_install(
            e,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            points.as_mut_ptr(),
            pointn,
            &mut sinfo,
        );
        i += 1;
    }
}
unsafe extern "C" fn make_flat_adj_edges(
    mut g: *mut graph_t,
    mut P: *mut path,
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut e0: *mut edge_t,
    mut et: libc::c_int,
) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut tn: *mut node_t = 0 as *mut node_t;
    let mut hn: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut labels: libc::c_int = 0 as libc::c_int;
    let mut ports: libc::c_int = 0 as libc::c_int;
    let mut auxg: *mut graph_t = 0 as *mut graph_t;
    let mut subg: *mut graph_t = 0 as *mut graph_t;
    let mut auxt: *mut node_t = 0 as *mut node_t;
    let mut auxh: *mut node_t = 0 as *mut node_t;
    let mut auxe: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut midx: libc::c_int = 0;
    let mut midy: libc::c_int = 0;
    let mut leftx: libc::c_int = 0;
    let mut rightx: libc::c_int = 0;
    let mut del: pointf = pointf { x: 0., y: 0. };
    let mut hvye: *mut edge_t = 0 as *mut edge_t;
    let mut attrs: *mut attr_state_t = 0 as *mut attr_state_t;
    static mut warned: libc::c_int = 0;
    tn = (*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
    {
        e0
    } else {
        e0.offset(1 as libc::c_int as isize)
    }))
        .node;
    hn = (*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
    {
        e0
    } else {
        e0.offset(-(1 as libc::c_int as isize))
    }))
        .node;
    if shapeOf(tn) as libc::c_uint == SH_RECORD as libc::c_int as libc::c_uint
        || shapeOf(hn) as libc::c_uint == SH_RECORD as libc::c_int as libc::c_uint
    {
        if warned == 0 {
            warned = 1 as libc::c_int;
            agerr(
                AGWARN,
                b"flat edge between adjacent nodes one of which has a record shape - replace records with HTML-like labels\n\0"
                    as *const u8 as *const libc::c_char,
            );
            agerr(
                AGPREV,
                b"  Edge %s %s %s\n\0" as *const u8 as *const libc::c_char,
                agnameof(tn as *mut libc::c_void),
                if agisdirected(g) != 0 {
                    b"->\0" as *const u8 as *const libc::c_char
                } else {
                    b"--\0" as *const u8 as *const libc::c_char
                },
                agnameof(hn as *mut libc::c_void),
            );
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < cnt {
        e = *edges.offset((ind + i) as isize);
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
            labels += 1;
        }
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
            as libc::c_int != 0
            || (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
                as libc::c_int != 0
        {
            ports = 1 as libc::c_int;
        }
        i += 1;
    }
    if ports == 0 as libc::c_int {
        if labels == 0 as libc::c_int {
            makeSimpleFlat(tn, hn, edges, ind, cnt, et);
        } else {
            makeSimpleFlatLabels(tn, hn, edges, ind, cnt, et, labels);
        }
        return;
    }
    attrs = zmalloc(::std::mem::size_of::<attr_state_t>() as libc::c_ulong)
        as *mut attr_state_t;
    auxg = cloneGraph(g, attrs);
    subg = agsubg(
        auxg,
        b"xxx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    agbindrec(
        subg as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    agset(
        subg as *mut libc::c_void,
        b"rank\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"source\0" as *const u8 as *const libc::c_char,
    );
    rightx = (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        as libc::c_int;
    leftx = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        as libc::c_int;
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir & 0x3 as libc::c_int
        & 1 as libc::c_int != 0
    {
        let mut n_0: *mut node_t = 0 as *mut node_t;
        n_0 = tn;
        tn = hn;
        hn = n_0;
    }
    auxt = cloneNode(
        subg,
        tn,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int & 1 as libc::c_int,
    );
    auxh = cloneNode(
        auxg,
        hn,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
            & 0x3 as libc::c_int & 1 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < cnt {
        e = *edges.offset((ind + i) as isize);
        while (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
            as libc::c_int != 0 as libc::c_int
        {
            e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
        }
        if (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node == tn
        {
            auxe = cloneEdge(auxg, auxt, auxh, e);
        } else {
            auxe = cloneEdge(auxg, auxh, auxt, e);
        }
        let ref mut fresh92 = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).alg;
        *fresh92 = auxe as *mut libc::c_void;
        if hvye.is_null()
            && !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.defined
            && !(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.defined
        {
            hvye = auxe;
            let ref mut fresh93 = (*((*(hvye as *mut Agobj_t)).data
                as *mut Agedgeinfo_t))
                .alg;
            *fresh93 = e as *mut libc::c_void;
        }
        i += 1;
    }
    if hvye.is_null() {
        hvye = agedge(auxg, auxt, auxh, 0 as *mut libc::c_char, 1 as libc::c_int);
    }
    agxset(
        hvye as *mut libc::c_void,
        E_weight,
        b"10000\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh94 = (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc;
    *fresh94 = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc;
    let ref mut fresh95 = (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t))
        .dotroot;
    *fresh95 = auxg;
    setEdgeType(auxg, et);
    dot_init_node_edge(auxg);
    dot_rank(auxg, 0 as *mut aspect_t);
    dot_mincross(auxg, 0 as libc::c_int);
    dot_position(auxg, 0 as *mut aspect_t);
    midx = (((*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        - (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
        + (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        + (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw)
        / 2 as libc::c_int as libc::c_double) as libc::c_int;
    midy = (((*((*(auxt as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        + (*((*(auxh as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x)
        / 2 as libc::c_int as libc::c_double) as libc::c_int;
    n = (*((*(auxg as *mut Agobj_t)).data as *mut Agraphinfo_t)).nlist;
    while !n.is_null() {
        if n == auxt {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .y = rightx as libc::c_double;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .x = midy as libc::c_double;
        } else if n == auxh {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .y = leftx as libc::c_double;
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .x = midy as libc::c_double;
        } else {
            (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                .coord
                .y = midx as libc::c_double;
        }
        n = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).next;
    }
    dot_sameports(auxg);
    _dot_splines(auxg, 0 as libc::c_int);
    dotneato_postprocess(auxg);
    if (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir & 0x3 as libc::c_int
        & 1 as libc::c_int != 0
    {
        del
            .x = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            - (*((*(auxt as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
        del
            .y = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            + (*((*(auxt as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
    } else {
        del
            .x = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            - (*((*(auxt as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
        del
            .y = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            - (*((*(auxt as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y;
    }
    i = 0 as libc::c_int;
    while i < cnt {
        let mut auxbz: *mut bezier = 0 as *mut bezier;
        let mut bz: *mut bezier = 0 as *mut bezier;
        e = *edges.offset((ind + i) as isize);
        while (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type
            as libc::c_int != 0 as libc::c_int
        {
            e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
        }
        auxe = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).alg as *mut edge_t;
        if !((auxe == hvye) as libc::c_int
            & ((*((*(auxe as *mut Agobj_t)).data as *mut Agedgeinfo_t)).alg).is_null()
                as libc::c_int != 0)
        {
            auxbz = (*(*((*(auxe as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list;
            bz = new_spline(e, (*auxbz).size);
            (*bz).sflag = (*auxbz).sflag;
            (*bz)
                .sp = transformf(
                (*auxbz).sp,
                del,
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                    & 0x3 as libc::c_int & 1 as libc::c_int,
            );
            (*bz).eflag = (*auxbz).eflag;
            (*bz)
                .ep = transformf(
                (*auxbz).ep,
                del,
                (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                    & 0x3 as libc::c_int & 1 as libc::c_int,
            );
            j = 0 as libc::c_int;
            while j < (*auxbz).size {
                let mut cp: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
                let ref mut fresh96 = *((*bz).list).offset(j as isize);
                *fresh96 = transformf(
                    *((*auxbz).list).offset(j as isize),
                    del,
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                        & 0x3 as libc::c_int & 1 as libc::c_int,
                );
                cp[0 as libc::c_int as usize] = *fresh96;
                j += 1;
                if j >= (*auxbz).size {
                    break;
                }
                let ref mut fresh97 = *((*bz).list).offset(j as isize);
                *fresh97 = transformf(
                    *((*auxbz).list).offset(j as isize),
                    del,
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                        & 0x3 as libc::c_int & 1 as libc::c_int,
                );
                cp[1 as libc::c_int as usize] = *fresh97;
                j += 1;
                let ref mut fresh98 = *((*bz).list).offset(j as isize);
                *fresh98 = transformf(
                    *((*auxbz).list).offset(j as isize),
                    del,
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                        & 0x3 as libc::c_int & 1 as libc::c_int,
                );
                cp[2 as libc::c_int as usize] = *fresh98;
                j += 1;
                cp[3 as libc::c_int
                    as usize] = transformf(
                    *((*auxbz).list).offset(j as isize),
                    del,
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                        & 0x3 as libc::c_int & 1 as libc::c_int,
                );
                update_bb_bz(
                    &mut (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
                    cp.as_mut_ptr(),
                );
            }
            if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
                (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .pos = transformf(
                    (*(*((*(auxe as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                        .pos,
                    del,
                    (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rankdir
                        & 0x3 as libc::c_int & 1 as libc::c_int,
                );
                (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
                    .set = 1 as libc::c_int != 0;
                updateBB(g, (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label);
            }
        }
        i += 1;
    }
    cleanupCloneGraph(auxg, attrs);
}
unsafe extern "C" fn makeFlatEnd(
    mut g: *mut graph_t,
    mut sp: *mut spline_info_t,
    mut P: *mut path,
    mut n: *mut node_t,
    mut e: *mut edge_t,
    mut endp: *mut pathend_t,
    mut isBegin: bool,
) {
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let ref mut fresh99 = (*endp).nb;
    *fresh99 = maximal_bbox(g, sp, n, 0 as *mut Agedge_t, e);
    b = *fresh99;
    (*endp).sidemask = (1 as libc::c_int) << 2 as libc::c_int;
    if isBegin {
        beginpath(P, e, 2 as libc::c_int, endp, 0 as libc::c_int != 0);
    } else {
        endpath(P, e, 2 as libc::c_int, endp, 0 as libc::c_int != 0);
    }
    b.UR.y = (*endp).boxes[((*endp).boxn - 1 as libc::c_int) as usize].UR.y;
    b.LL.y = (*endp).boxes[((*endp).boxn - 1 as libc::c_int) as usize].LL.y;
    b = makeregularend(
        b,
        (1 as libc::c_int) << 2 as libc::c_int,
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            + (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
                ))
                .ht2,
    );
    if b.LL.x < b.UR.x && b.LL.y < b.UR.y {
        let ref mut fresh100 = (*endp).boxn;
        let fresh101 = *fresh100;
        *fresh100 = *fresh100 + 1;
        (*endp).boxes[fresh101 as usize] = b;
    }
}
unsafe extern "C" fn makeBottomFlatEnd(
    mut g: *mut graph_t,
    mut sp: *mut spline_info_t,
    mut P: *mut path,
    mut n: *mut node_t,
    mut e: *mut edge_t,
    mut endp: *mut pathend_t,
    mut isBegin: bool,
) {
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let ref mut fresh102 = (*endp).nb;
    *fresh102 = maximal_bbox(g, sp, n, 0 as *mut Agedge_t, e);
    b = *fresh102;
    (*endp).sidemask = (1 as libc::c_int) << 0 as libc::c_int;
    if isBegin {
        beginpath(P, e, 2 as libc::c_int, endp, 0 as libc::c_int != 0);
    } else {
        endpath(P, e, 2 as libc::c_int, endp, 0 as libc::c_int != 0);
    }
    b.UR.y = (*endp).boxes[((*endp).boxn - 1 as libc::c_int) as usize].UR.y;
    b.LL.y = (*endp).boxes[((*endp).boxn - 1 as libc::c_int) as usize].LL.y;
    b = makeregularend(
        b,
        (1 as libc::c_int) << 0 as libc::c_int,
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(
                    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
                ))
                .ht2,
    );
    if b.LL.x < b.UR.x && b.LL.y < b.UR.y {
        let ref mut fresh103 = (*endp).boxn;
        let fresh104 = *fresh103;
        *fresh103 = *fresh103 + 1;
        (*endp).boxes[fresh104 as usize] = b;
    }
}
unsafe extern "C" fn make_flat_labeled_edge(
    mut g: *mut graph_t,
    mut sp: *mut spline_info_t,
    mut P: *mut path,
    mut e: *mut edge_t,
    mut et: libc::c_int,
) {
    let mut tn: *mut node_t = 0 as *mut node_t;
    let mut hn: *mut node_t = 0 as *mut node_t;
    let mut ln: *mut node_t = 0 as *mut node_t;
    let mut ps: *mut pointf = 0 as *mut pointf;
    let mut ps_needs_free: bool = 0 as libc::c_int != 0;
    let mut tend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    let mut hend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    let mut lb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut i: libc::c_int = 0;
    let mut pn: libc::c_int = 0;
    let mut ydelta: libc::c_int = 0;
    let mut f: *mut edge_t = 0 as *mut edge_t;
    let mut points: [pointf; 7] = [pointf { x: 0., y: 0. }; 7];
    tn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    hn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    f = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
    while !((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt).is_null() {
        f = (*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
    }
    ln = (*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        f
    } else {
        f.offset(1 as libc::c_int as isize)
    })
        .node;
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
        .pos = (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
    (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label)
        .set = 1 as libc::c_int != 0;
    if et == (1 as libc::c_int) << 1 as libc::c_int {
        let mut startp: pointf = pointf { x: 0., y: 0. };
        let mut endp: pointf = pointf { x: 0., y: 0. };
        let mut lp: pointf = pointf { x: 0., y: 0. };
        startp = add_pointf(
            (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
        );
        endp = add_pointf(
            (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
        );
        lp = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos;
        lp.y
            -= (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen.y
                / 2.0f64;
        points[0 as libc::c_int as usize] = startp;
        points[1 as libc::c_int as usize] = points[0 as libc::c_int as usize];
        points[4 as libc::c_int as usize] = lp;
        points[3 as libc::c_int as usize] = points[4 as libc::c_int as usize];
        points[2 as libc::c_int as usize] = points[3 as libc::c_int as usize];
        points[6 as libc::c_int as usize] = endp;
        points[5 as libc::c_int as usize] = points[6 as libc::c_int as usize];
        ps = points.as_mut_ptr();
        pn = 7 as libc::c_int;
    } else {
        lb
            .LL
            .x = (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            - (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
        lb
            .UR
            .x = (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            + (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
        lb
            .UR
            .y = (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            + (*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                / 2 as libc::c_int as libc::c_double;
        ydelta = ((*((*(ln as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(
                    (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
                ))
                .ht1 - (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            + (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(
                    (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
                ))
                .ht2) as libc::c_int;
        ydelta /= 6 as libc::c_int;
        lb
            .LL
            .y = lb.UR.y
            - (if 5 as libc::c_int > ydelta { 5 as libc::c_int } else { ydelta })
                as libc::c_double;
        makeFlatEnd(g, sp, P, tn, e, &mut tend, 1 as libc::c_int != 0);
        makeFlatEnd(g, sp, P, hn, e, &mut hend, 0 as libc::c_int != 0);
        let mut boxes: [boxf; 3] = [
            {
                let mut init = boxf {
                    LL: {
                        let mut init = pointf_s {
                            x: tend.boxes[(tend.boxn - 1 as libc::c_int) as usize].LL.x,
                            y: tend.boxes[(tend.boxn - 1 as libc::c_int) as usize].UR.y,
                        };
                        init
                    },
                    UR: lb.LL,
                };
                init
            },
            {
                let mut init = boxf {
                    LL: {
                        let mut init = pointf_s {
                            x: tend.boxes[(tend.boxn - 1 as libc::c_int) as usize].LL.x,
                            y: lb.LL.y,
                        };
                        init
                    },
                    UR: {
                        let mut init = pointf_s {
                            x: hend.boxes[(hend.boxn - 1 as libc::c_int) as usize].UR.x,
                            y: lb.UR.y,
                        };
                        init
                    },
                };
                init
            },
            {
                let mut init = boxf {
                    LL: {
                        let mut init = pointf_s {
                            x: lb.UR.x,
                            y: hend.boxes[(hend.boxn - 1 as libc::c_int) as usize].UR.y,
                        };
                        init
                    },
                    UR: {
                        let mut init = pointf_s {
                            x: hend.boxes[(hend.boxn - 1 as libc::c_int) as usize].UR.x,
                            y: lb.LL.y,
                        };
                        init
                    },
                };
                init
            },
        ];
        let boxn: size_t = (::std::mem::size_of::<[boxf; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<boxf>() as libc::c_ulong);
        i = 0 as libc::c_int;
        while i < tend.boxn {
            add_box(P, tend.boxes[i as usize]);
            i += 1;
        }
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < boxn {
            add_box(P, boxes[j as usize]);
            j = j.wrapping_add(1);
        }
        i = hend.boxn - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            add_box(P, hend.boxes[i as usize]);
            i -= 1;
        }
        ps_needs_free = 1 as libc::c_int != 0;
        if et == (5 as libc::c_int) << 1 as libc::c_int {
            ps = routesplines(P, &mut pn);
        } else {
            ps = routepolylines(P, &mut pn);
        }
        if pn == 0 as libc::c_int {
            free(ps as *mut libc::c_void);
            return;
        }
    }
    clip_and_install(
        e,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
        ps,
        pn,
        &mut sinfo,
    );
    if ps_needs_free {
        free(ps as *mut libc::c_void);
    }
}
unsafe extern "C" fn make_flat_bottom_edges(
    mut g: *mut graph_t,
    mut sp: *mut spline_info_t,
    mut P: *mut path,
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut e: *mut edge_t,
    mut splines: libc::c_int,
) {
    let mut tn: *mut node_t = 0 as *mut node_t;
    let mut hn: *mut node_t = 0 as *mut node_t;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut stepx: libc::c_double = 0.;
    let mut stepy: libc::c_double = 0.;
    let mut vspace: libc::c_double = 0.;
    let mut nextr: *mut rank_t = 0 as *mut rank_t;
    let mut tend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    let mut hend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    tn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    hn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    r = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
    if r < (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).maxrank {
        nextr = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((r + 1 as libc::c_int) as isize);
        vspace = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .pht1
            - ((*((*(*((*nextr).v).offset(0 as libc::c_int as isize) as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .coord
                .y + (*nextr).pht2);
    } else {
        vspace = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ranksep
            as libc::c_double;
    }
    stepx = (*sp).Multisep as libc::c_double
        / (cnt + 1 as libc::c_int) as libc::c_double;
    stepy = vspace / (cnt + 1 as libc::c_int) as libc::c_double;
    makeBottomFlatEnd(g, sp, P, tn, e, &mut tend, 1 as libc::c_int != 0);
    makeBottomFlatEnd(g, sp, P, hn, e, &mut hend, 0 as libc::c_int != 0);
    i = 0 as libc::c_int;
    while i < cnt {
        let mut b: boxf = boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        };
        e = *edges.offset((ind + i) as isize);
        let mut boxn: size_t = 0 as libc::c_int as size_t;
        let mut boxes: [boxf; 3] = [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 3];
        b = tend.boxes[(tend.boxn - 1 as libc::c_int) as usize];
        boxes[boxn as usize].LL.x = b.LL.x;
        boxes[boxn as usize].UR.y = b.LL.y;
        boxes[boxn as usize]
            .UR
            .x = b.UR.x + (i + 1 as libc::c_int) as libc::c_double * stepx;
        boxes[boxn as usize]
            .LL
            .y = b.LL.y - (i + 1 as libc::c_int) as libc::c_double * stepy;
        boxn = boxn.wrapping_add(1);
        boxes[boxn as usize]
            .LL
            .x = tend.boxes[(tend.boxn - 1 as libc::c_int) as usize].LL.x;
        boxes[boxn as usize]
            .UR
            .y = boxes[boxn.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            .LL
            .y;
        boxes[boxn as usize]
            .UR
            .x = hend.boxes[(hend.boxn - 1 as libc::c_int) as usize].UR.x;
        boxes[boxn as usize].LL.y = boxes[boxn as usize].UR.y - stepy;
        boxn = boxn.wrapping_add(1);
        b = hend.boxes[(hend.boxn - 1 as libc::c_int) as usize];
        boxes[boxn as usize].UR.x = b.UR.x;
        boxes[boxn as usize].UR.y = b.LL.y;
        boxes[boxn as usize]
            .LL
            .x = b.LL.x - (i + 1 as libc::c_int) as libc::c_double * stepx;
        boxes[boxn as usize]
            .LL
            .y = boxes[boxn.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            .UR
            .y;
        boxn = boxn.wrapping_add(1);
        if boxn
            == (::std::mem::size_of::<[boxf; 3]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<boxf>() as libc::c_ulong)
        {} else {
            __assert_fail(
                b"boxn == sizeof(boxes) / sizeof(boxes[0])\0" as *const u8
                    as *const libc::c_char,
                b"dotsplines.c\0" as *const u8 as *const libc::c_char,
                1560 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 100],
                    &[libc::c_char; 100],
                >(
                    b"void make_flat_bottom_edges(graph_t *, spline_info_t *, path *, edge_t **, int, int, edge_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        j = 0 as libc::c_int;
        while j < tend.boxn {
            add_box(P, tend.boxes[j as usize]);
            j += 1;
        }
        let mut k: size_t = 0 as libc::c_int as size_t;
        while k < boxn {
            add_box(P, boxes[k as usize]);
            k = k.wrapping_add(1);
        }
        j = hend.boxn - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            add_box(P, hend.boxes[j as usize]);
            j -= 1;
        }
        let mut ps: *mut pointf = 0 as *mut pointf;
        let mut pn: libc::c_int = 0 as libc::c_int;
        if splines != 0 {
            ps = routesplines(P, &mut pn);
        } else {
            ps = routepolylines(P, &mut pn);
        }
        if pn == 0 as libc::c_int {
            free(ps as *mut libc::c_void);
            return;
        }
        clip_and_install(
            e,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            ps,
            pn,
            &mut sinfo,
        );
        free(ps as *mut libc::c_void);
        (*P).nbox = 0 as libc::c_int;
        i += 1;
    }
}
unsafe extern "C" fn make_flat_edge(
    mut g: *mut graph_t,
    mut sp: *mut spline_info_t,
    mut P: *mut path,
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut et: libc::c_int,
) {
    let mut tn: *mut node_t = 0 as *mut node_t;
    let mut hn: *mut node_t = 0 as *mut node_t;
    let mut fwdedgei: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedge: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut isAdjacent: libc::c_int = 0;
    let mut stepx: libc::c_double = 0.;
    let mut stepy: libc::c_double = 0.;
    let mut vspace: libc::c_double = 0.;
    let mut tside: libc::c_int = 0;
    let mut hside: libc::c_int = 0;
    let mut tend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    let mut hend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    fwdedge.out.base.data = &mut fwdedgei as *mut Agedgeinfo_t as *mut Agrec_t;
    e = *edges.offset(ind as isize);
    isAdjacent = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).adjacent
        as libc::c_int;
    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
        & 32 as libc::c_int != 0
    {
        let mut newp: *mut edge_t = 0 as *mut edge_t;
        let mut info: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
        newp = &mut fwdedge.out;
        info = (*newp).base.data as *mut Agedgeinfo_t;
        *info = *((*e).base.data as *mut Agedgeinfo_t);
        *newp = *e;
        let ref mut fresh105 = (*newp).base.data;
        *fresh105 = info as *mut Agrec_t;
        let ref mut fresh106 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            newp
        } else {
            newp.offset(1 as libc::c_int as isize)
        })
            .node;
        *fresh106 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        let ref mut fresh107 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 2 as libc::c_int
        {
            newp
        } else {
            newp.offset(-(1 as libc::c_int as isize))
        })
            .node;
        *fresh107 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
        (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port;
        (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port;
        (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .edge_type = 1 as libc::c_int as libc::c_char;
        let ref mut fresh108 = (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .to_orig;
        *fresh108 = e;
        e = &mut fwdedge.out;
    }
    i = 1 as libc::c_int;
    while i < cnt {
        if (*((*(*edges.offset((ind + i) as isize) as *mut Agobj_t)).data
            as *mut Agedgeinfo_t))
            .adjacent != 0
        {
            isAdjacent = 1 as libc::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if isAdjacent != 0 {
        make_flat_adj_edges(g, P, edges, ind, cnt, e, et);
        return;
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
        make_flat_labeled_edge(g, sp, P, e, et);
        return;
    }
    if et == (1 as libc::c_int) << 1 as libc::c_int {
        makeSimpleFlat(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            edges,
            ind,
            cnt,
            et,
        );
        return;
    }
    tside = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.side
        as libc::c_int;
    hside = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.side
        as libc::c_int;
    if tside == (1 as libc::c_int) << 0 as libc::c_int
        && hside != (1 as libc::c_int) << 2 as libc::c_int
        || hside == (1 as libc::c_int) << 0 as libc::c_int
            && tside != (1 as libc::c_int) << 2 as libc::c_int
    {
        make_flat_bottom_edges(
            g,
            sp,
            P,
            edges,
            ind,
            cnt,
            e,
            (et == (5 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
        );
        return;
    }
    tn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    hn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    r = (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank;
    if r > 0 as libc::c_int {
        let mut prevr: *mut rank_t = 0 as *mut rank_t;
        if (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels
            as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
            prevr = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset((r - 2 as libc::c_int) as isize);
        } else {
            prevr = ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset((r - 1 as libc::c_int) as isize);
        }
        vspace = (*((*(*((*prevr).v).offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .coord
            .y - (*prevr).ht1
            - (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .ht2;
    } else {
        vspace = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).ranksep
            as libc::c_double;
    }
    stepx = (*sp).Multisep as libc::c_double
        / (cnt + 1 as libc::c_int) as libc::c_double;
    stepy = vspace / (cnt + 1 as libc::c_int) as libc::c_double;
    makeFlatEnd(g, sp, P, tn, e, &mut tend, 1 as libc::c_int != 0);
    makeFlatEnd(g, sp, P, hn, e, &mut hend, 0 as libc::c_int != 0);
    i = 0 as libc::c_int;
    while i < cnt {
        let mut b: boxf = boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        };
        e = *edges.offset((ind + i) as isize);
        let mut boxn: size_t = 0 as libc::c_int as size_t;
        let mut boxes: [boxf; 3] = [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 3];
        b = tend.boxes[(tend.boxn - 1 as libc::c_int) as usize];
        boxes[boxn as usize].LL.x = b.LL.x;
        boxes[boxn as usize].LL.y = b.UR.y;
        boxes[boxn as usize]
            .UR
            .x = b.UR.x + (i + 1 as libc::c_int) as libc::c_double * stepx;
        boxes[boxn as usize]
            .UR
            .y = b.UR.y + (i + 1 as libc::c_int) as libc::c_double * stepy;
        boxn = boxn.wrapping_add(1);
        boxes[boxn as usize]
            .LL
            .x = tend.boxes[(tend.boxn - 1 as libc::c_int) as usize].LL.x;
        boxes[boxn as usize]
            .LL
            .y = boxes[boxn.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            .UR
            .y;
        boxes[boxn as usize]
            .UR
            .x = hend.boxes[(hend.boxn - 1 as libc::c_int) as usize].UR.x;
        boxes[boxn as usize].UR.y = boxes[boxn as usize].LL.y + stepy;
        boxn = boxn.wrapping_add(1);
        b = hend.boxes[(hend.boxn - 1 as libc::c_int) as usize];
        boxes[boxn as usize].UR.x = b.UR.x;
        boxes[boxn as usize].LL.y = b.UR.y;
        boxes[boxn as usize]
            .LL
            .x = b.LL.x - (i + 1 as libc::c_int) as libc::c_double * stepx;
        boxes[boxn as usize]
            .UR
            .y = boxes[boxn.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            .LL
            .y;
        boxn = boxn.wrapping_add(1);
        if boxn
            == (::std::mem::size_of::<[boxf; 3]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<boxf>() as libc::c_ulong)
        {} else {
            __assert_fail(
                b"boxn == sizeof(boxes) / sizeof(boxes[0])\0" as *const u8
                    as *const libc::c_char,
                b"dotsplines.c\0" as *const u8 as *const libc::c_char,
                1685 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void make_flat_edge(graph_t *, spline_info_t *, path *, edge_t **, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        j = 0 as libc::c_int;
        while j < tend.boxn {
            add_box(P, tend.boxes[j as usize]);
            j += 1;
        }
        let mut k: size_t = 0 as libc::c_int as size_t;
        while k < boxn {
            add_box(P, boxes[k as usize]);
            k = k.wrapping_add(1);
        }
        j = hend.boxn - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            add_box(P, hend.boxes[j as usize]);
            j -= 1;
        }
        let mut ps: *mut pointf = 0 as *mut pointf;
        let mut pn: libc::c_int = 0 as libc::c_int;
        if et == (5 as libc::c_int) << 1 as libc::c_int {
            ps = routesplines(P, &mut pn);
        } else {
            ps = routepolylines(P, &mut pn);
        }
        if pn == 0 as libc::c_int {
            free(ps as *mut libc::c_void);
            return;
        }
        clip_and_install(
            e,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            ps,
            pn,
            &mut sinfo,
        );
        free(ps as *mut libc::c_void);
        (*P).nbox = 0 as libc::c_int;
        i += 1;
    }
}
unsafe extern "C" fn leftOf(
    mut p1: pointf,
    mut p2: pointf,
    mut p3: pointf,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    d = ((p1.y - p2.y) * (p3.x - p2.x) - (p3.y - p2.y) * (p1.x - p2.x)) as libc::c_int;
    return (d > 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn makeLineEdge(
    mut g: *mut graph_t,
    mut fe: *mut edge_t,
    mut points: *mut pointf,
    mut hp: *mut *mut node_t,
) -> libc::c_int {
    let mut delr: libc::c_int = 0;
    let mut pn: libc::c_int = 0;
    let mut hn: *mut node_t = 0 as *mut node_t;
    let mut tn: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = fe;
    let mut startp: pointf = pointf { x: 0., y: 0. };
    let mut endp: pointf = pointf { x: 0., y: 0. };
    let mut lp: pointf = pointf { x: 0., y: 0. };
    let mut dimen: pointf = pointf { x: 0., y: 0. };
    let mut width: libc::c_double = 0.;
    let mut height: libc::c_double = 0.;
    while (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).edge_type as libc::c_int
        != 0 as libc::c_int
    {
        e = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig;
    }
    hn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    tn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    delr = abs(
        (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
            - (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank,
    );
    if delr == 1 as libc::c_int
        || delr == 2 as libc::c_int
            && (*((*((*g).root as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_labels
                as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    if (*(if ((*(fe as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        fe
    } else {
        fe.offset(1 as libc::c_int as isize)
    }))
        .node
        == (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node
    {
        *hp = hn;
        startp = add_pointf(
            (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
        );
        endp = add_pointf(
            (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
        );
    } else {
        *hp = tn;
        startp = add_pointf(
            (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port.p,
        );
        endp = add_pointf(
            (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord,
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port.p,
        );
    }
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null() {
        dimen = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).dimen;
        if (*((*(agraphof(hn as *mut libc::c_void) as *mut Agobj_t)).data
            as *mut Agraphinfo_t))
            .rankdir & 0x3 as libc::c_int & 1 as libc::c_int != 0
        {
            width = dimen.y;
            height = dimen.x;
        } else {
            width = dimen.x;
            height = dimen.y;
        }
        lp = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).pos;
        if leftOf(endp, startp, lp) != 0 {
            lp.x += width / 2.0f64;
            lp.y -= height / 2.0f64;
        } else {
            lp.x -= width / 2.0f64;
            lp.y += height / 2.0f64;
        }
        let ref mut fresh109 = *points.offset(0 as libc::c_int as isize);
        *fresh109 = startp;
        *points.offset(1 as libc::c_int as isize) = *fresh109;
        let ref mut fresh110 = *points.offset(4 as libc::c_int as isize);
        *fresh110 = lp;
        let ref mut fresh111 = *points.offset(3 as libc::c_int as isize);
        *fresh111 = *fresh110;
        *points.offset(2 as libc::c_int as isize) = *fresh111;
        let ref mut fresh112 = *points.offset(6 as libc::c_int as isize);
        *fresh112 = endp;
        *points.offset(5 as libc::c_int as isize) = *fresh112;
        pn = 7 as libc::c_int;
    } else {
        let ref mut fresh113 = *points.offset(0 as libc::c_int as isize);
        *fresh113 = startp;
        *points.offset(1 as libc::c_int as isize) = *fresh113;
        let ref mut fresh114 = *points.offset(2 as libc::c_int as isize);
        *fresh114 = endp;
        *points.offset(3 as libc::c_int as isize) = *fresh114;
        pn = 4 as libc::c_int;
    }
    return pn;
}
unsafe extern "C" fn make_regular_edge(
    mut g: *mut graph_t,
    mut sp: *mut spline_info_t,
    mut P: *mut path,
    mut edges: *mut *mut edge_t,
    mut ind: libc::c_int,
    mut cnt: libc::c_int,
    mut et: libc::c_int,
) {
    let mut tn: *mut node_t = 0 as *mut node_t;
    let mut hn: *mut node_t = 0 as *mut node_t;
    let mut fwdedgeai: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedgebi: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedgei: Agedgeinfo_t = Agedgeinfo_t {
        hdr: Agrec_t {
            name: 0 as *mut libc::c_char,
            next: 0 as *mut Agrec_t,
        },
        spl: 0 as *mut splines,
        tail_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        head_port: port {
            p: pointf { x: 0., y: 0. },
            theta: 0.,
            bp: 0 as *mut boxf,
            defined: false,
            constrained: false,
            clip: false,
            dyna: false,
            order: 0,
            side: 0,
            name: 0 as *mut libc::c_char,
        },
        label: 0 as *mut textlabel_t,
        head_label: 0 as *mut textlabel_t,
        tail_label: 0 as *mut textlabel_t,
        xlabel: 0 as *mut textlabel_t,
        edge_type: 0,
        compound: 0,
        adjacent: 0,
        label_ontop: 0,
        gui_state: 0,
        to_orig: 0 as *mut edge_t,
        alg: 0 as *mut libc::c_void,
        factor: 0.,
        dist: 0.,
        path: Ppoly_t {
            ps: 0 as *mut Ppoint_t,
            pn: 0,
        },
        showboxes: 0,
        conc_opp_flag: false,
        xpenalty: 0,
        weight: 0,
        cutvalue: 0,
        tree_index: 0,
        count: 0,
        minlen: 0,
        to_virt: 0 as *mut edge_t,
    };
    let mut fwdedgea: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut fwdedgeb: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut fwdedge: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed_1 { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut fe: *mut edge_t = 0 as *mut edge_t;
    let mut le: *mut edge_t = 0 as *mut edge_t;
    let mut segfirst: *mut edge_t = 0 as *mut edge_t;
    let mut tend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    let mut hend: pathend_t = pathend_t {
        nb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
        np: pointf { x: 0., y: 0. },
        sidemask: 0,
        boxn: 0,
        boxes: [boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        }; 20],
    };
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut sl: libc::c_int = 0;
    let mut si: libc::c_int = 0;
    let mut smode: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut hackflag: libc::c_int = 0;
    let mut longedge: libc::c_int = 0;
    static mut pointfs: *mut pointf = 0 as *const pointf as *mut pointf;
    static mut pointfs2: *mut pointf = 0 as *const pointf as *mut pointf;
    static mut numpts: libc::c_int = 0;
    static mut numpts2: libc::c_int = 0;
    let mut pointn: libc::c_int = 0;
    fwdedgea.out.base.data = &mut fwdedgeai as *mut Agedgeinfo_t as *mut Agrec_t;
    fwdedgeb.out.base.data = &mut fwdedgebi as *mut Agedgeinfo_t as *mut Agrec_t;
    fwdedge.out.base.data = &mut fwdedgei as *mut Agedgeinfo_t as *mut Agrec_t;
    if pointfs.is_null() {
        pointfs = gcalloc(
            2000 as libc::c_int as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        pointfs2 = gcalloc(
            2000 as libc::c_int as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        numpts = 2000 as libc::c_int;
        numpts2 = 2000 as libc::c_int;
    }
    sl = 0 as libc::c_int;
    e = *edges.offset(ind as isize);
    hackflag = 0 as libc::c_int;
    if abs(
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
            - (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .rank,
    ) > 1 as libc::c_int
    {
        fwdedgeai = *((*e).base.data as *mut Agedgeinfo_t);
        fwdedgea.out = *e;
        fwdedgea.in_0 = *e.offset(1 as libc::c_int as isize);
        fwdedgea.out.base.data = &mut fwdedgeai as *mut Agedgeinfo_t as *mut Agrec_t;
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            & 32 as libc::c_int != 0
        {
            let mut newp: *mut edge_t = 0 as *mut edge_t;
            let mut info: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
            newp = &mut fwdedgeb.out;
            info = (*newp).base.data as *mut Agedgeinfo_t;
            *info = *((*e).base.data as *mut Agedgeinfo_t);
            *newp = *e;
            let ref mut fresh115 = (*newp).base.data;
            *fresh115 = info as *mut Agrec_t;
            let ref mut fresh116 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 3 as libc::c_int
            {
                newp
            } else {
                newp.offset(1 as libc::c_int as isize)
            })
                .node;
            *fresh116 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
            let ref mut fresh117 = (*if ((*(newp as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                newp
            } else {
                newp.offset(-(1 as libc::c_int as isize))
            })
                .node;
            *fresh117 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node;
            (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port;
            (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port;
            (*((*(newp as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .edge_type = 1 as libc::c_int as libc::c_char;
            let ref mut fresh118 = (*((*(newp as *mut Agobj_t)).data
                as *mut Agedgeinfo_t))
                .to_orig;
            *fresh118 = e;
            let ref mut fresh119 = (*if ((*(&mut fwdedgea.out as *mut Agedge_t
                as *mut Agobj_t))
                .tag)
                .objtype() as libc::c_int == 3 as libc::c_int
            {
                &mut fwdedgea.out
            } else {
                (&mut fwdedgea.out as *mut Agedge_t).offset(1 as libc::c_int as isize)
            })
                .node;
            *fresh119 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
            (*((*(&mut fwdedgea.out as *mut Agedge_t as *mut Agobj_t)).data
                as *mut Agedgeinfo_t))
                .tail_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port;
        } else {
            fwdedgebi = *((*e).base.data as *mut Agedgeinfo_t);
            fwdedgeb.out = *e;
            fwdedgeb.out.base.data = &mut fwdedgebi as *mut Agedgeinfo_t as *mut Agrec_t;
            let ref mut fresh120 = (*if ((*(&mut fwdedgea.out as *mut Agedge_t
                as *mut Agobj_t))
                .tag)
                .objtype() as libc::c_int == 3 as libc::c_int
            {
                &mut fwdedgea.out
            } else {
                (&mut fwdedgea.out as *mut Agedge_t).offset(1 as libc::c_int as isize)
            })
                .node;
            *fresh120 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node;
            fwdedgeb.in_0 = *e.offset(1 as libc::c_int as isize);
        }
        le = getmainedge(e);
        while !((*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt).is_null()
        {
            le = (*((*(le as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_virt;
        }
        let ref mut fresh121 = (*if ((*(&mut fwdedgea.out as *mut Agedge_t
            as *mut Agobj_t))
            .tag)
            .objtype() as libc::c_int == 2 as libc::c_int
        {
            &mut fwdedgea.out
        } else {
            (&mut fwdedgea.out as *mut Agedge_t).offset(-(1 as libc::c_int as isize))
        })
            .node;
        *fresh121 = (*if ((*(le as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            le
        } else {
            le.offset(-(1 as libc::c_int as isize))
        })
            .node;
        (*((*(&mut fwdedgea.out as *mut Agedge_t as *mut Agobj_t)).data
            as *mut Agedgeinfo_t))
            .head_port
            .defined = 0 as libc::c_int != 0;
        (*((*(&mut fwdedgea.out as *mut Agedge_t as *mut Agobj_t)).data
            as *mut Agedgeinfo_t))
            .edge_type = 1 as libc::c_int as libc::c_char;
        let ref mut fresh122 = (*((*(&mut fwdedgea.out as *mut Agedge_t as *mut Agobj_t))
            .data as *mut Agedgeinfo_t))
            .head_port
            .p
            .y;
        *fresh122 = 0 as libc::c_int as libc::c_double;
        (*((*(&mut fwdedgea.out as *mut Agedge_t as *mut Agobj_t)).data
            as *mut Agedgeinfo_t))
            .head_port
            .p
            .x = *fresh122;
        let ref mut fresh123 = (*((*(&mut fwdedgea.out as *mut Agedge_t as *mut Agobj_t))
            .data as *mut Agedgeinfo_t))
            .to_orig;
        *fresh123 = e;
        e = &mut fwdedgea.out;
        hackflag = (0 as libc::c_int == 0) as libc::c_int;
    } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            & 32 as libc::c_int != 0
        {
        let mut newp_0: *mut edge_t = 0 as *mut edge_t;
        let mut info_0: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
        newp_0 = &mut fwdedgea.out;
        info_0 = (*newp_0).base.data as *mut Agedgeinfo_t;
        *info_0 = *((*e).base.data as *mut Agedgeinfo_t);
        *newp_0 = *e;
        let ref mut fresh124 = (*newp_0).base.data;
        *fresh124 = info_0 as *mut Agrec_t;
        let ref mut fresh125 = (*if ((*(newp_0 as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 3 as libc::c_int
        {
            newp_0
        } else {
            newp_0.offset(1 as libc::c_int as isize)
        })
            .node;
        *fresh125 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        let ref mut fresh126 = (*if ((*(newp_0 as *mut Agobj_t)).tag).objtype()
            as libc::c_int == 2 as libc::c_int
        {
            newp_0
        } else {
            newp_0.offset(-(1 as libc::c_int as isize))
        })
            .node;
        *fresh126 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .tail_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_port;
        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .head_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_port;
        (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .edge_type = 1 as libc::c_int as libc::c_char;
        let ref mut fresh127 = (*((*(newp_0 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
            .to_orig;
        *fresh127 = e;
        e = &mut fwdedgea.out;
    }
    fe = e;
    if !(et == (1 as libc::c_int) << 1 as libc::c_int
        && {
            pointn = makeLineEdge(g, fe, pointfs, &mut hn);
            pointn != 0
        })
    {
        let mut is_spline: bool = et == (5 as libc::c_int) << 1 as libc::c_int;
        let mut boxes: boxes_t = {
            let mut init = boxes_t {
                data: 0 as *mut boxf,
                size: 0,
                capacity: 0,
            };
            init
        };
        pointn = 0 as libc::c_int;
        segfirst = e;
        tn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node;
        hn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node;
        tend.nb = maximal_bbox(g, sp, tn, 0 as *mut Agedge_t, e);
        b = tend.nb;
        beginpath(P, e, 1 as libc::c_int, &mut tend, spline_merge(tn));
        b.UR.y = tend.boxes[(tend.boxn - 1 as libc::c_int) as usize].UR.y;
        b.LL.y = tend.boxes[(tend.boxn - 1 as libc::c_int) as usize].LL.y;
        b = makeregularend(
            b,
            (1 as libc::c_int) << 0 as libc::c_int,
            (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(
                        (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                            as isize,
                    ))
                    .ht1,
        );
        if b.LL.x < b.UR.x && b.LL.y < b.UR.y {
            let fresh128 = tend.boxn;
            tend.boxn = tend.boxn + 1;
            tend.boxes[fresh128 as usize] = b;
        }
        longedge = 0 as libc::c_int;
        smode = 0 as libc::c_int;
        si = -(1 as libc::c_int);
        while (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
            as libc::c_int == 1 as libc::c_int
            && !(sinfo.splineMerge).expect("non-null function pointer")(hn)
        {
            longedge = 1 as libc::c_int;
            boxes_append(
                &mut boxes,
                rank_box(
                    sp,
                    g,
                    (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank,
                ),
            );
            if smode == 0
                && {
                    sl = straight_len(hn);
                    sl
                        >= (if (*((*((*g).root as *mut Agobj_t)).data
                            as *mut Agraphinfo_t))
                            .has_labels as libc::c_int
                            & (1 as libc::c_int) << 0 as libc::c_int != 0
                        {
                            4 as libc::c_int + 1 as libc::c_int
                        } else {
                            2 as libc::c_int + 1 as libc::c_int
                        })
                }
            {
                smode = (0 as libc::c_int == 0) as libc::c_int;
                si = 1 as libc::c_int;
                sl -= 2 as libc::c_int;
            }
            if smode == 0 || si > 0 as libc::c_int {
                si -= 1;
                boxes_append(
                    &mut boxes,
                    maximal_bbox(
                        g,
                        sp,
                        hn,
                        e,
                        *((*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                            .out
                            .list)
                            .offset(0 as libc::c_int as isize),
                    ),
                );
                e = *((*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                    .offset(0 as libc::c_int as isize);
                tn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node;
                hn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
            } else {
                hend
                    .nb = maximal_bbox(
                    g,
                    sp,
                    hn,
                    e,
                    *((*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                        .offset(0 as libc::c_int as isize),
                );
                endpath(
                    P,
                    e,
                    1 as libc::c_int,
                    &mut hend,
                    spline_merge(
                        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            e
                        } else {
                            e.offset(-(1 as libc::c_int as isize))
                        })
                            .node,
                    ),
                );
                b = makeregularend(
                    hend.boxes[(hend.boxn - 1 as libc::c_int) as usize],
                    (1 as libc::c_int) << 2 as libc::c_int,
                    (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                        + (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                            .offset(
                                (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                                    as isize,
                            ))
                            .ht2,
                );
                if b.LL.x < b.UR.x && b.LL.y < b.UR.y {
                    let fresh129 = hend.boxn;
                    hend.boxn = hend.boxn + 1;
                    hend.boxes[fresh129 as usize] = b;
                }
                (*P)
                    .end
                    .theta = 3.14159265358979323846f64
                    / 2 as libc::c_int as libc::c_double;
                (*P).end.constrained = 1 as libc::c_int != 0;
                if boxes.size <= 2147483647 as libc::c_int as size_t
                    && !(b"integer overflow\0" as *const u8 as *const libc::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"boxes.size <= (size_t)INT_MAX && \"integer overflow\"\0"
                            as *const u8 as *const libc::c_char,
                        b"dotsplines.c\0" as *const u8 as *const libc::c_char,
                        1908 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 85],
                            &[libc::c_char; 85],
                        >(
                            b"void make_regular_edge(graph_t *, spline_info_t *, path *, edge_t **, int, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                completeregularpath(
                    P,
                    segfirst,
                    e,
                    &mut tend,
                    &mut hend,
                    boxes.data,
                    boxes.size as libc::c_int,
                    1 as libc::c_int,
                );
                let mut ps: *mut pointf = 0 as *mut pointf;
                let mut pn: libc::c_int = 0 as libc::c_int;
                if is_spline {
                    ps = routesplines(P, &mut pn);
                } else {
                    ps = routepolylines(P, &mut pn);
                    if et == (1 as libc::c_int) << 1 as libc::c_int
                        && pn > 4 as libc::c_int
                    {
                        *ps
                            .offset(
                                1 as libc::c_int as isize,
                            ) = *ps.offset(0 as libc::c_int as isize);
                        let ref mut fresh130 = *ps.offset(2 as libc::c_int as isize);
                        *fresh130 = *ps.offset((pn - 1 as libc::c_int) as isize);
                        *ps.offset(3 as libc::c_int as isize) = *fresh130;
                        pn = 4 as libc::c_int;
                    }
                }
                if pn == 0 as libc::c_int {
                    free(ps as *mut libc::c_void);
                    boxes_free(&mut boxes);
                    return;
                }
                if pointn + pn > numpts {
                    numpts = 2 as libc::c_int * (pointn + pn);
                    pointfs = grealloc(
                        pointfs as *mut libc::c_void,
                        (numpts as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<pointf>() as libc::c_ulong,
                            ),
                    ) as *mut pointf;
                }
                i = 0 as libc::c_int;
                while i < pn {
                    let fresh131 = pointn;
                    pointn = pointn + 1;
                    *pointfs.offset(fresh131 as isize) = *ps.offset(i as isize);
                    i += 1;
                }
                free(ps as *mut libc::c_void);
                e = straight_path(
                    *((*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                        .offset(0 as libc::c_int as isize),
                    sl,
                    pointfs,
                    &mut pointn,
                );
                recover_slack(segfirst, P);
                segfirst = e;
                tn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                    .node;
                hn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
                boxes_clear(&mut boxes);
                tend
                    .nb = maximal_bbox(
                    g,
                    sp,
                    tn,
                    *((*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                        .offset(0 as libc::c_int as isize),
                    e,
                );
                beginpath(P, e, 1 as libc::c_int, &mut tend, spline_merge(tn));
                b = makeregularend(
                    tend.boxes[(tend.boxn - 1 as libc::c_int) as usize],
                    (1 as libc::c_int) << 0 as libc::c_int,
                    (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                        - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                            .offset(
                                (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                                    as isize,
                            ))
                            .ht1,
                );
                if b.LL.x < b.UR.x && b.LL.y < b.UR.y {
                    let fresh132 = tend.boxn;
                    tend.boxn = tend.boxn + 1;
                    tend.boxes[fresh132 as usize] = b;
                }
                (*P)
                    .start
                    .theta = -3.14159265358979323846f64
                    / 2 as libc::c_int as libc::c_double;
                (*P).start.constrained = 1 as libc::c_int != 0;
                smode = 0 as libc::c_int;
            }
        }
        boxes_append(
            &mut boxes,
            rank_box(sp, g, (*((*(tn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank),
        );
        hend.nb = maximal_bbox(g, sp, hn, e, 0 as *mut Agedge_t);
        b = hend.nb;
        endpath(
            P,
            if hackflag != 0 { &mut fwdedgeb.out } else { e },
            1 as libc::c_int,
            &mut hend,
            spline_merge(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
            ),
        );
        b.UR.y = hend.boxes[(hend.boxn - 1 as libc::c_int) as usize].UR.y;
        b.LL.y = hend.boxes[(hend.boxn - 1 as libc::c_int) as usize].LL.y;
        b = makeregularend(
            b,
            (1 as libc::c_int) << 2 as libc::c_int,
            (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                + (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                    .offset(
                        (*((*(hn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank
                            as isize,
                    ))
                    .ht2,
        );
        if b.LL.x < b.UR.x && b.LL.y < b.UR.y {
            let fresh133 = hend.boxn;
            hend.boxn = hend.boxn + 1;
            hend.boxes[fresh133 as usize] = b;
        }
        if boxes.size <= 2147483647 as libc::c_int as size_t
            && !(b"integer overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"boxes.size <= (size_t)INT_MAX && \"integer overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"dotsplines.c\0" as *const u8 as *const libc::c_char,
                1964 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"void make_regular_edge(graph_t *, spline_info_t *, path *, edge_t **, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        completeregularpath(
            P,
            segfirst,
            e,
            &mut tend,
            &mut hend,
            boxes.data,
            boxes.size as libc::c_int,
            longedge,
        );
        boxes_free(&mut boxes);
        let mut ps_0: *mut pointf = 0 as *mut pointf;
        let mut pn_0: libc::c_int = 0 as libc::c_int;
        if is_spline {
            ps_0 = routesplines(P, &mut pn_0);
        } else {
            ps_0 = routepolylines(P, &mut pn_0);
        }
        if et == (1 as libc::c_int) << 1 as libc::c_int && pn_0 > 4 as libc::c_int {
            *ps_0
                .offset(
                    1 as libc::c_int as isize,
                ) = *ps_0.offset(0 as libc::c_int as isize);
            let ref mut fresh134 = *ps_0.offset(2 as libc::c_int as isize);
            *fresh134 = *ps_0.offset((pn_0 - 1 as libc::c_int) as isize);
            *ps_0.offset(3 as libc::c_int as isize) = *fresh134;
            pn_0 = 4 as libc::c_int;
        }
        if pn_0 == 0 as libc::c_int {
            free(ps_0 as *mut libc::c_void);
            return;
        }
        if pointn + pn_0 > numpts {
            numpts = 2 as libc::c_int * (pointn + pn_0);
            pointfs = grealloc(
                pointfs as *mut libc::c_void,
                (numpts as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
            ) as *mut pointf;
        }
        i = 0 as libc::c_int;
        while i < pn_0 {
            let fresh135 = pointn;
            pointn = pointn + 1;
            *pointfs.offset(fresh135 as isize) = *ps_0.offset(i as isize);
            i += 1;
        }
        free(ps_0 as *mut libc::c_void);
        recover_slack(segfirst, P);
        hn = if hackflag != 0 {
            (*if ((*(&mut fwdedgeb.out as *mut Agedge_t as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                &mut fwdedgeb.out
            } else {
                (&mut fwdedgeb.out as *mut Agedge_t).offset(-(1 as libc::c_int as isize))
            })
                .node
        } else {
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node
        };
    }
    if cnt == 1 as libc::c_int {
        clip_and_install(fe, hn, pointfs, pointn, &mut sinfo);
        return;
    }
    dx = (*sp).Multisep * (cnt - 1 as libc::c_int) / 2 as libc::c_int;
    i = 1 as libc::c_int;
    while i < pointn - 1 as libc::c_int {
        (*pointfs.offset(i as isize)).x -= dx as libc::c_double;
        i += 1;
    }
    if numpts > numpts2 {
        numpts2 = numpts;
        pointfs2 = grealloc(
            pointfs2 as *mut libc::c_void,
            (numpts2 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
        ) as *mut pointf;
    }
    i = 0 as libc::c_int;
    while i < pointn {
        *pointfs2.offset(i as isize) = *pointfs.offset(i as isize);
        i += 1;
    }
    clip_and_install(fe, hn, pointfs2, pointn, &mut sinfo);
    j = 1 as libc::c_int;
    while j < cnt {
        e = *edges.offset((ind + j) as isize);
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tree_index
            & 32 as libc::c_int != 0
        {
            let mut newp_1: *mut edge_t = 0 as *mut edge_t;
            let mut info_1: *mut Agedgeinfo_t = 0 as *mut Agedgeinfo_t;
            newp_1 = &mut fwdedge.out;
            info_1 = (*newp_1).base.data as *mut Agedgeinfo_t;
            *info_1 = *((*e).base.data as *mut Agedgeinfo_t);
            *newp_1 = *e;
            let ref mut fresh136 = (*newp_1).base.data;
            *fresh136 = info_1 as *mut Agrec_t;
            let ref mut fresh137 = (*if ((*(newp_1 as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 3 as libc::c_int
            {
                newp_1
            } else {
                newp_1.offset(1 as libc::c_int as isize)
            })
                .node;
            *fresh137 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
            let ref mut fresh138 = (*if ((*(newp_1 as *mut Agobj_t)).tag).objtype()
                as libc::c_int == 2 as libc::c_int
            {
                newp_1
            } else {
                newp_1.offset(-(1 as libc::c_int as isize))
            })
                .node;
            *fresh138 = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
                .node;
            (*((*(newp_1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port;
            (*((*(newp_1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .head_port = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .tail_port;
            (*((*(newp_1 as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                .edge_type = 1 as libc::c_int as libc::c_char;
            let ref mut fresh139 = (*((*(newp_1 as *mut Agobj_t)).data
                as *mut Agedgeinfo_t))
                .to_orig;
            *fresh139 = e;
            e = &mut fwdedge.out;
        }
        i = 1 as libc::c_int;
        while i < pointn - 1 as libc::c_int {
            (*pointfs.offset(i as isize)).x += (*sp).Multisep as libc::c_double;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < pointn {
            *pointfs2.offset(i as isize) = *pointfs.offset(i as isize);
            i += 1;
        }
        clip_and_install(
            e,
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node,
            pointfs2,
            pointn,
            &mut sinfo,
        );
        j += 1;
    }
}
unsafe extern "C" fn completeregularpath(
    mut P: *mut path,
    mut first: *mut edge_t,
    mut last: *mut edge_t,
    mut tendp: *mut pathend_t,
    mut hendp: *mut pathend_t,
    mut boxes: *mut boxf,
    mut boxn: libc::c_int,
    mut flag: libc::c_int,
) {
    let mut uleft: *mut edge_t = 0 as *mut edge_t;
    let mut uright: *mut edge_t = 0 as *mut edge_t;
    let mut lleft: *mut edge_t = 0 as *mut edge_t;
    let mut lright: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    let mut fb: libc::c_int = 0;
    let mut lb: libc::c_int = 0;
    let mut spl: *mut splines = 0 as *mut splines;
    lb = -(1 as libc::c_int);
    fb = lb;
    uright = 0 as *mut edge_t;
    uleft = uright;
    uleft = top_bound(first, -(1 as libc::c_int));
    uright = top_bound(first, 1 as libc::c_int);
    if !uleft.is_null() {
        spl = getsplinepoints(uleft);
        if spl.is_null() {
            return;
        }
    }
    if !uright.is_null() {
        spl = getsplinepoints(uright);
        if spl.is_null() {
            return;
        }
    }
    lright = 0 as *mut edge_t;
    lleft = lright;
    lleft = bot_bound(last, -(1 as libc::c_int));
    lright = bot_bound(last, 1 as libc::c_int);
    if !lleft.is_null() {
        spl = getsplinepoints(lleft);
        if spl.is_null() {
            return;
        }
    }
    if !lright.is_null() {
        spl = getsplinepoints(lright);
        if spl.is_null() {
            return;
        }
    }
    i = 0 as libc::c_int;
    while i < (*tendp).boxn {
        add_box(P, (*tendp).boxes[i as usize]);
        i += 1;
    }
    fb = (*P).nbox + 1 as libc::c_int;
    lb = fb + boxn - 3 as libc::c_int;
    i = 0 as libc::c_int;
    while i < boxn {
        add_box(P, *boxes.offset(i as isize));
        i += 1;
    }
    i = (*hendp).boxn - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        add_box(P, (*hendp).boxes[i as usize]);
        i -= 1;
    }
    adjustregularpath(P, fb, lb);
}
unsafe extern "C" fn makeregularend(
    mut b: boxf,
    mut side: libc::c_int,
    mut y: libc::c_double,
) -> boxf {
    let mut newb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    match side {
        1 => {
            newb = boxfof(b.LL.x, y, b.UR.x, b.LL.y);
        }
        4 => {
            newb = boxfof(b.LL.x, b.UR.y, b.UR.x, y);
        }
        _ => {}
    }
    return newb;
}
unsafe extern "C" fn adjustregularpath(
    mut P: *mut path,
    mut fb: libc::c_int,
    mut lb: libc::c_int,
) {
    let mut bp1: *mut boxf = 0 as *mut boxf;
    let mut bp2: *mut boxf = 0 as *mut boxf;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    i = fb - 1 as libc::c_int;
    while i < lb + 1 as libc::c_int {
        bp1 = &mut *((*P).boxes).offset(i as isize) as *mut boxf;
        if (i - fb) % 2 as libc::c_int == 0 as libc::c_int {
            if (*bp1).LL.x >= (*bp1).UR.x {
                x = (((*bp1).LL.x + (*bp1).UR.x) / 2 as libc::c_int as libc::c_double)
                    as libc::c_int;
                (*bp1).LL.x = (x - 8 as libc::c_int) as libc::c_double;
                (*bp1).UR.x = (x + 8 as libc::c_int) as libc::c_double;
            }
        } else if (*bp1).LL.x + 16 as libc::c_int as libc::c_double > (*bp1).UR.x {
            x = (((*bp1).LL.x + (*bp1).UR.x) / 2 as libc::c_int as libc::c_double)
                as libc::c_int;
            (*bp1).LL.x = (x - 8 as libc::c_int) as libc::c_double;
            (*bp1).UR.x = (x + 8 as libc::c_int) as libc::c_double;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*P).nbox - 1 as libc::c_int {
        bp1 = &mut *((*P).boxes).offset(i as isize) as *mut boxf;
        bp2 = &mut *((*P).boxes).offset((i + 1 as libc::c_int) as isize) as *mut boxf;
        if i >= fb && i <= lb && (i - fb) % 2 as libc::c_int == 0 as libc::c_int {
            if (*bp1).LL.x + 16 as libc::c_int as libc::c_double > (*bp2).UR.x {
                (*bp2).UR.x = (*bp1).LL.x + 16 as libc::c_int as libc::c_double;
            }
            if ((*bp1).UR.x - 16 as libc::c_int as libc::c_double) < (*bp2).LL.x {
                (*bp2).LL.x = (*bp1).UR.x - 16 as libc::c_int as libc::c_double;
            }
        } else if i + 1 as libc::c_int >= fb && i < lb
                && (i + 1 as libc::c_int - fb) % 2 as libc::c_int == 0 as libc::c_int
            {
            if (*bp1).LL.x + 16 as libc::c_int as libc::c_double > (*bp2).UR.x {
                (*bp1).LL.x = (*bp2).UR.x - 16 as libc::c_int as libc::c_double;
            }
            if ((*bp1).UR.x - 16 as libc::c_int as libc::c_double) < (*bp2).LL.x {
                (*bp1).UR.x = (*bp2).LL.x + 16 as libc::c_int as libc::c_double;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn rank_box(
    mut sp: *mut spline_info_t,
    mut g: *mut graph_t,
    mut r: libc::c_int,
) -> boxf {
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut left0: *mut node_t = 0 as *mut node_t;
    let mut left1: *mut node_t = 0 as *mut node_t;
    b = *((*sp).Rank_box).offset(r as isize);
    if b.LL.x == b.UR.x {
        left0 = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(r as isize))
            .v)
            .offset(0 as libc::c_int as isize);
        left1 = *((*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset((r + 1 as libc::c_int) as isize))
            .v)
            .offset(0 as libc::c_int as isize);
        b.LL.x = (*sp).LeftBound as libc::c_double;
        b
            .LL
            .y = (*((*(left1 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            + (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset((r + 1 as libc::c_int) as isize))
                .ht2;
        b.UR.x = (*sp).RightBound as libc::c_double;
        b
            .UR
            .y = (*((*(left0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
                .offset(r as isize))
                .ht1;
        *((*sp).Rank_box).offset(r as isize) = b;
    }
    return b;
}
unsafe extern "C" fn straight_len(mut n: *mut node_t) -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut v: *mut node_t = 0 as *mut node_t;
    v = n;
    loop {
        v = (*if ((*(*((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .tag)
            .objtype() as libc::c_int == 2 as libc::c_int
        {
            *((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize)
        } else {
            (*((*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize))
                .offset(-(1 as libc::c_int as isize))
        })
            .node;
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            != 1 as libc::c_int
        {
            break;
        }
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
            != 1 as libc::c_int
            || (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                != 1 as libc::c_int
        {
            break;
        }
        if (*((*(v as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            != (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        {
            break;
        }
        cnt += 1;
    }
    return cnt;
}
unsafe extern "C" fn straight_path(
    mut e: *mut edge_t,
    mut cnt: libc::c_int,
    mut plist: *mut pointf,
    mut np: *mut libc::c_int,
) -> *mut Agedge_t {
    let mut n: libc::c_int = *np;
    let mut f: *mut edge_t = e;
    loop {
        let fresh140 = cnt;
        cnt = cnt - 1;
        if !(fresh140 != 0) {
            break;
        }
        f = *((*((*((*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            f
        } else {
            f.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .out
            .list)
            .offset(0 as libc::c_int as isize);
    }
    let fresh141 = *np;
    *np = *np + 1;
    *plist.offset(fresh141 as isize) = *plist.offset((n - 1 as libc::c_int) as isize);
    let fresh142 = *np;
    *np = *np + 1;
    *plist.offset(fresh142 as isize) = *plist.offset((n - 1 as libc::c_int) as isize);
    *plist
        .offset(
            *np as isize,
        ) = (*((*((*if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
        == 3 as libc::c_int
    {
        f
    } else {
        f.offset(1 as libc::c_int as isize)
    })
        .node as *mut Agobj_t))
        .data as *mut Agnodeinfo_t))
        .coord;
    return f;
}
unsafe extern "C" fn recover_slack(mut e: *mut edge_t, mut p: *mut path) {
    let mut b: libc::c_int = 0;
    let mut vn: *mut node_t = 0 as *mut node_t;
    b = 0 as libc::c_int;
    vn = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    while (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 1 as libc::c_int
        && !(sinfo.splineMerge).expect("non-null function pointer")(vn)
    {
        while b < (*p).nbox
            && (*((*p).boxes).offset(b as isize)).LL.y
                > (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
        {
            b += 1;
        }
        if b >= (*p).nbox {
            break;
        }
        if !((*((*p).boxes).offset(b as isize)).UR.y
            < (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y)
        {
            if !((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).is_null()
            {
                resize_vn(
                    vn,
                    (*((*p).boxes).offset(b as isize)).LL.x as libc::c_int,
                    (*((*p).boxes).offset(b as isize)).UR.x as libc::c_int,
                    ((*((*p).boxes).offset(b as isize)).UR.x
                        + (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw)
                        as libc::c_int,
                );
            } else {
                resize_vn(
                    vn,
                    (*((*p).boxes).offset(b as isize)).LL.x as libc::c_int,
                    (((*((*p).boxes).offset(b as isize)).LL.x
                        + (*((*p).boxes).offset(b as isize)).UR.x)
                        / 2 as libc::c_int as libc::c_double) as libc::c_int,
                    (*((*p).boxes).offset(b as isize)).UR.x as libc::c_int,
                );
            }
        }
        vn = (*if ((*(*((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .tag)
            .objtype() as libc::c_int == 2 as libc::c_int
        {
            *((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize)
        } else {
            (*((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize))
                .offset(-(1 as libc::c_int as isize))
        })
            .node;
    }
}
unsafe extern "C" fn resize_vn(
    mut vn: *mut node_t,
    mut lx: libc::c_int,
    mut cx: libc::c_int,
    mut rx: libc::c_int,
) {
    (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .coord
        .x = cx as libc::c_double;
    (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .lw = (cx - lx) as libc::c_double;
    (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .rw = (rx - cx) as libc::c_double;
}
unsafe extern "C" fn top_bound(
    mut e: *mut edge_t,
    mut side: libc::c_int,
) -> *mut Agedge_t {
    let mut f: *mut edge_t = 0 as *mut edge_t;
    let mut ans: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    loop {
        f = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .out
            .list)
            .offset(i as isize);
        if f.is_null() {
            break;
        }
        if !(side
            * ((*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                f
            } else {
                f.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order
                - (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order) <= 0 as libc::c_int)
        {
            if !(((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
                && (((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig)
                    .is_null()
                    || ((*((*((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .to_orig as *mut Agobj_t))
                        .data as *mut Agedgeinfo_t))
                        .spl)
                        .is_null()))
            {
                if ans.is_null()
                    || side
                        * ((*((*((*(if ((*(ans as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 2 as libc::c_int
                        {
                            ans
                        } else {
                            ans.offset(-(1 as libc::c_int as isize))
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .order
                            - (*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype()
                                as libc::c_int == 2 as libc::c_int
                            {
                                f
                            } else {
                                f.offset(-(1 as libc::c_int as isize))
                            }))
                                .node as *mut Agobj_t))
                                .data as *mut Agnodeinfo_t))
                                .order) > 0 as libc::c_int
                {
                    ans = f;
                }
            }
        }
        i += 1;
    }
    return ans;
}
unsafe extern "C" fn bot_bound(
    mut e: *mut edge_t,
    mut side: libc::c_int,
) -> *mut Agedge_t {
    let mut f: *mut edge_t = 0 as *mut edge_t;
    let mut ans: *mut edge_t = 0 as *mut edge_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    loop {
        f = *((*((*((*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .in_0
            .list)
            .offset(i as isize);
        if f.is_null() {
            break;
        }
        if !(side
            * ((*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                f
            } else {
                f.offset(1 as libc::c_int as isize)
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .order
                - (*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .order) <= 0 as libc::c_int)
        {
            if !(((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
                && (((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t)).to_orig)
                    .is_null()
                    || ((*((*((*((*(f as *mut Agobj_t)).data as *mut Agedgeinfo_t))
                        .to_orig as *mut Agobj_t))
                        .data as *mut Agedgeinfo_t))
                        .spl)
                        .is_null()))
            {
                if ans.is_null()
                    || side
                        * ((*((*((*(if ((*(ans as *mut Agobj_t)).tag).objtype()
                            as libc::c_int == 3 as libc::c_int
                        {
                            ans
                        } else {
                            ans.offset(1 as libc::c_int as isize)
                        }))
                            .node as *mut Agobj_t))
                            .data as *mut Agnodeinfo_t))
                            .order
                            - (*((*((*(if ((*(f as *mut Agobj_t)).tag).objtype()
                                as libc::c_int == 3 as libc::c_int
                            {
                                f
                            } else {
                                f.offset(1 as libc::c_int as isize)
                            }))
                                .node as *mut Agobj_t))
                                .data as *mut Agnodeinfo_t))
                                .order) > 0 as libc::c_int
                {
                    ans = f;
                }
            }
        }
        i += 1;
    }
    return ans;
}
unsafe extern "C" fn cl_vninside(
    mut cl: *mut graph_t,
    mut n: *mut node_t,
) -> libc::c_int {
    return ((*((*(cl as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
        <= (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            <= (*((*(cl as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
        && ((*((*(cl as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.y
            <= (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
            && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
                <= (*((*(cl as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.y))
        as libc::c_int;
}
unsafe extern "C" fn cl_bound(
    mut g: *mut graph_t,
    mut n: *mut node_t,
    mut adj: *mut node_t,
) -> *mut Agraph_t {
    let mut rv: *mut graph_t = 0 as *mut graph_t;
    let mut cl: *mut graph_t = 0 as *mut graph_t;
    let mut tcl: *mut graph_t = 0 as *mut graph_t;
    let mut hcl: *mut graph_t = 0 as *mut graph_t;
    let mut orig: *mut edge_t = 0 as *mut edge_t;
    rv = 0 as *mut graph_t;
    if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 0 as libc::c_int
    {
        hcl = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust;
        tcl = hcl;
    } else {
        orig = (*((*(*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .data as *mut Agedgeinfo_t))
            .to_orig;
        tcl = (*((*((*if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            orig
        } else {
            orig.offset(1 as libc::c_int as isize)
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .clust;
        hcl = (*((*((*if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            orig
        } else {
            orig.offset(-(1 as libc::c_int as isize))
        })
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .clust;
    }
    if (*((*(adj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 0 as libc::c_int
    {
        cl = if (*((*(adj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust == g {
            0 as *mut graph_t
        } else {
            (*((*(adj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).clust
        };
        if !cl.is_null() && cl != tcl && cl != hcl {
            rv = cl;
        }
    } else {
        orig = (*((*(*((*((*(adj as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(0 as libc::c_int as isize) as *mut Agobj_t))
            .data as *mut Agedgeinfo_t))
            .to_orig;
        cl = if (*((*((*(if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            orig
        } else {
            orig.offset(1 as libc::c_int as isize)
        }))
            .node as *mut Agobj_t))
            .data as *mut Agnodeinfo_t))
            .clust == g
        {
            0 as *mut graph_t
        } else {
            (*((*((*if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                orig
            } else {
                orig.offset(1 as libc::c_int as isize)
            })
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .clust
        };
        if !cl.is_null() && cl != tcl && cl != hcl && cl_vninside(cl, adj) != 0 {
            rv = cl;
        } else {
            cl = if (*((*((*(if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                orig
            } else {
                orig.offset(-(1 as libc::c_int as isize))
            }))
                .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .clust == g
            {
                0 as *mut graph_t
            } else {
                (*((*((*if ((*(orig as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    orig
                } else {
                    orig.offset(-(1 as libc::c_int as isize))
                })
                    .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .clust
            };
            if !cl.is_null() && cl != tcl && cl != hcl && cl_vninside(cl, adj) != 0 {
                rv = cl;
            }
        }
    }
    return rv;
}
unsafe extern "C" fn maximal_bbox(
    mut g: *mut graph_t,
    mut sp: *mut spline_info_t,
    mut vn: *mut node_t,
    mut ie: *mut edge_t,
    mut oe: *mut edge_t,
) -> boxf {
    let mut b: libc::c_double = 0.;
    let mut nb: libc::c_double = 0.;
    let mut left_cl: *mut graph_t = 0 as *mut graph_t;
    let mut right_cl: *mut graph_t = 0 as *mut graph_t;
    let mut left: *mut node_t = 0 as *mut node_t;
    let mut right: *mut node_t = 0 as *mut node_t;
    let mut rv: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    right_cl = 0 as *mut graph_t;
    left_cl = right_cl;
    b = (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
        - (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw
        - 4 as libc::c_int as libc::c_double;
    left = neighbor(g, vn, ie, oe, -(1 as libc::c_int));
    if !left.is_null() {
        left_cl = cl_bound(g, vn, left);
        if !left_cl.is_null() {
            nb = (*((*(left_cl as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR.x
                + (*sp).Splinesep as libc::c_double;
        } else {
            nb = (*((*(left as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                + (*((*(left as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mval;
            if (*((*(left as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                as libc::c_int == 0 as libc::c_int
            {
                nb
                    += (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep
                        as libc::c_double / 2.0f64;
            } else {
                nb += (*sp).Splinesep as libc::c_double;
            }
        }
        if nb < b {
            b = nb;
        }
        rv
            .LL
            .x = (if b >= 0 as libc::c_int as libc::c_double {
            (b + 0.5f64) as libc::c_int
        } else {
            (b - 0.5f64) as libc::c_int
        }) as libc::c_double;
    } else {
        rv
            .LL
            .x = (if (if b >= 0 as libc::c_int as libc::c_double {
            (b + 0.5f64) as libc::c_int
        } else {
            (b - 0.5f64) as libc::c_int
        }) < (*sp).LeftBound
        {
            if b >= 0 as libc::c_int as libc::c_double {
                (b + 0.5f64) as libc::c_int
            } else {
                (b - 0.5f64) as libc::c_int
            }
        } else {
            (*sp).LeftBound
        }) as libc::c_double;
    }
    if (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 1 as libc::c_int
        && !((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).is_null()
    {
        b = (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            + 10 as libc::c_int as libc::c_double;
    } else {
        b = (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
            + (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw
            + 4 as libc::c_int as libc::c_double;
    }
    right = neighbor(g, vn, ie, oe, 1 as libc::c_int);
    if !right.is_null() {
        right_cl = cl_bound(g, vn, right);
        if !right_cl.is_null() {
            nb = (*((*(right_cl as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL.x
                - (*sp).Splinesep as libc::c_double;
        } else {
            nb = (*((*(right as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x
                - (*((*(right as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
            if (*((*(right as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                as libc::c_int == 0 as libc::c_int
            {
                nb
                    -= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).nodesep
                        as libc::c_double / 2.0f64;
            } else {
                nb -= (*sp).Splinesep as libc::c_double;
            }
        }
        if nb > b {
            b = nb;
        }
        rv
            .UR
            .x = (if b >= 0 as libc::c_int as libc::c_double {
            (b + 0.5f64) as libc::c_int
        } else {
            (b - 0.5f64) as libc::c_int
        }) as libc::c_double;
    } else {
        rv
            .UR
            .x = (if (if b >= 0 as libc::c_int as libc::c_double {
            (b + 0.5f64) as libc::c_int
        } else {
            (b - 0.5f64) as libc::c_int
        }) > (*sp).RightBound
        {
            if b >= 0 as libc::c_int as libc::c_double {
                (b + 0.5f64) as libc::c_int
            } else {
                (b - 0.5f64) as libc::c_int
            }
        } else {
            (*sp).RightBound
        }) as libc::c_double;
    }
    if (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
        == 1 as libc::c_int
        && !((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).is_null()
    {
        rv.UR.x -= (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
        if rv.UR.x < rv.LL.x {
            rv.UR.x = (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.x;
        }
    }
    rv
        .LL
        .y = (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
        - (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(
                (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
            ))
            .ht1;
    rv
        .UR
        .y = (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord.y
        + (*((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).rank)
            .offset(
                (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize,
            ))
            .ht2;
    return rv;
}
unsafe extern "C" fn neighbor(
    mut g: *mut graph_t,
    mut vn: *mut node_t,
    mut ie: *mut edge_t,
    mut oe: *mut edge_t,
    mut dir: libc::c_int,
) -> *mut Agnode_t {
    let mut i: libc::c_int = 0;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut rv: *mut node_t = 0 as *mut node_t;
    let mut rank: *mut rank_t = &mut *((*((*(g as *mut Agobj_t)).data
        as *mut Agraphinfo_t))
        .rank)
        .offset((*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rank as isize)
        as *mut rank_t;
    i = (*((*(vn as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order + dir;
    while i >= 0 as libc::c_int && i < (*rank).n {
        n = *((*rank).v).offset(i as isize);
        if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type as libc::c_int
            == 1 as libc::c_int
            && !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label).is_null()
        {
            rv = n;
            break;
        } else if (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                as libc::c_int == 0 as libc::c_int
            {
            rv = n;
            break;
        } else if !pathscross(n, vn, ie, oe) {
            rv = n;
            break;
        } else {
            i += dir;
        }
    }
    return rv;
}
unsafe extern "C" fn pathscross(
    mut n0: *mut node_t,
    mut n1: *mut node_t,
    mut ie1: *mut edge_t,
    mut oe1: *mut edge_t,
) -> bool {
    let mut e0: *mut edge_t = 0 as *mut edge_t;
    let mut e1: *mut edge_t = 0 as *mut edge_t;
    let mut na: *mut node_t = 0 as *mut node_t;
    let mut nb: *mut node_t = 0 as *mut node_t;
    let mut order: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    order = ((*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
        > (*((*(n1 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order) as libc::c_int;
    if (*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
        != 1 as libc::c_int
        && (*((*(n1 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
            != 1 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    e1 = oe1;
    if (*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
        == 1 as libc::c_int && !e1.is_null()
    {
        e0 = *((*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
            .offset(0 as libc::c_int as isize);
        cnt = 0 as libc::c_int;
        while cnt < 2 as libc::c_int {
            na = (*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e0
            } else {
                e0.offset(-(1 as libc::c_int as isize))
            }))
                .node;
            nb = (*(if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e1
            } else {
                e1.offset(-(1 as libc::c_int as isize))
            }))
                .node;
            if na == nb {
                break;
            }
            if order
                != ((*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                    > (*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order)
                    as libc::c_int
            {
                return 1 as libc::c_int != 0;
            }
            if (*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                != 1 as libc::c_int
                || (*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 0 as libc::c_int
            {
                break;
            }
            e0 = *((*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize);
            if (*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.size
                != 1 as libc::c_int
                || (*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 0 as libc::c_int
            {
                break;
            }
            e1 = *((*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).out.list)
                .offset(0 as libc::c_int as isize);
            cnt += 1;
        }
    }
    e1 = ie1;
    if (*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
        == 1 as libc::c_int && !e1.is_null()
    {
        e0 = *((*((*(n0 as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
            .offset(0 as libc::c_int as isize);
        cnt = 0 as libc::c_int;
        while cnt < 2 as libc::c_int {
            na = (*(if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e0
            } else {
                e0.offset(1 as libc::c_int as isize)
            }))
                .node;
            nb = (*(if ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e1
            } else {
                e1.offset(1 as libc::c_int as isize)
            }))
                .node;
            if na == nb {
                break;
            }
            if order
                != ((*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order
                    > (*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).order)
                    as libc::c_int
            {
                return 1 as libc::c_int != 0;
            }
            if (*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                != 1 as libc::c_int
                || (*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 0 as libc::c_int
            {
                break;
            }
            e0 = *((*((*(na as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(0 as libc::c_int as isize);
            if (*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.size
                != 1 as libc::c_int
                || (*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).node_type
                    as libc::c_int == 0 as libc::c_int
            {
                break;
            }
            e1 = *((*((*(nb as *mut Agobj_t)).data as *mut Agnodeinfo_t)).in_0.list)
                .offset(0 as libc::c_int as isize);
            cnt += 1;
        }
    }
    return 0 as libc::c_int != 0;
}
