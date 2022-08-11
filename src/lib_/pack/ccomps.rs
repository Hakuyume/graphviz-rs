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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn agsubnode(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
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
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agcopyattr(oldobj: *mut libc::c_void, newobj: *mut libc::c_void) -> libc::c_int;
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
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agclean(g: *mut Agraph_t, kind: libc::c_int, rec_name: *mut libc::c_char);
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    static mut Agstrictundirected: Agdesc_t;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub const FIRST_ALLOCATION: C2RustUnnamed_0 = 512;
pub type C2RustUnnamed_0 = libc::c_uint;
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
    pub u: C2RustUnnamed_2,
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
    pub u: C2RustUnnamed_1,
    pub type_0: color_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub union C2RustUnnamed_2 {
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
    pub hl: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub hh: C2RustUnnamed_4,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub graph: C2RustUnnamed_5,
    pub node: C2RustUnnamed_5,
    pub edge: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub type Agedge_t = Agedge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_7,
    pub s: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub u: C2RustUnnamed_8,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub txt: C2RustUnnamed_9,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct stk_t {
    pub data: gv_stack_t,
    pub actionfn: Option::<unsafe extern "C" fn(*mut Agnode_t, *mut libc::c_void) -> ()>,
    pub markfn: Option::<
        unsafe extern "C" fn(*mut Agnode_t, libc::c_int) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orig_t {
    pub h: Agrec_t,
    pub orig: *mut Agraph_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccgraphinfo_t {
    pub h: Agrec_t,
    pub cc_subg: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub g: *mut Agraph_t,
    pub n: *mut Agnode_t,
    pub v: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccgnodeinfo_t {
    pub h: Agrec_t,
    pub mark: libc::c_char,
    pub ptr: C2RustUnnamed_10,
}
#[inline]
unsafe extern "C" fn stack_size(mut stack: *const gv_stack_t) -> size_t {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"size_t stack_size(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return (*stack).size;
}
#[inline]
unsafe extern "C" fn stack_is_empty(mut stack: *const gv_stack_t) -> bool {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"_Bool stack_is_empty(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return stack_size(stack) == 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn stack_push(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).size == (*stack).capacity {
        if ((18446744073709551615 as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) < (*stack).capacity)
            as libc::c_int as libc::c_long != 0
        {
            return 75 as libc::c_int;
        }
        let mut c: size_t = if (*stack).capacity == 0 as libc::c_int as libc::c_ulong {
            FIRST_ALLOCATION as libc::c_int as libc::c_ulong
        } else {
            (2 as libc::c_int as libc::c_ulong).wrapping_mul((*stack).capacity)
        };
        let mut b: *mut *mut libc::c_void = realloc(
            (*stack).base as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong).wrapping_mul(c),
        ) as *mut *mut libc::c_void;
        if (b == 0 as *mut libc::c_void as *mut *mut libc::c_void) as libc::c_int
            as libc::c_long != 0
        {
            return 12 as libc::c_int;
        }
        (*stack).capacity = c;
        let ref mut fresh0 = (*stack).base;
        *fresh0 = b;
    }
    if !((*stack).base).is_null() {} else {
        __assert_fail(
            b"stack->base != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).capacity > (*stack).size {} else {
        __assert_fail(
            b"stack->capacity > stack->size\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh1 = *((*stack).base).offset((*stack).size as isize);
    *fresh1 = item;
    let ref mut fresh2 = (*stack).size;
    *fresh2 = (*fresh2).wrapping_add(1);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn stack_top(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    if !stack_is_empty(stack)
        && !(b"access to top of an empty stack\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"!stack_is_empty(stack) && \"access to top of an empty stack\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return *((*stack).base)
        .offset(
            ((*stack).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
}
#[inline]
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh3 = (*stack).size;
    *fresh3 = (*fresh3).wrapping_sub(1);
    return top;
}
#[inline]
unsafe extern "C" fn stack_reset(mut stack: *mut gv_stack_t) {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void stack_reset(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    free((*stack).base as *mut libc::c_void);
    memset(
        stack as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gv_stack_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn initStk(
    mut sp: *mut stk_t,
    mut actionfn: Option::<unsafe extern "C" fn(*mut Agnode_t, *mut libc::c_void) -> ()>,
    mut markfn: Option::<unsafe extern "C" fn(*mut Agnode_t, libc::c_int) -> libc::c_int>,
) {
    (*sp)
        .data = {
        let mut init = gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        };
        init
    };
    let ref mut fresh4 = (*sp).actionfn;
    *fresh4 = actionfn;
    let ref mut fresh5 = (*sp).markfn;
    *fresh5 = markfn;
}
unsafe extern "C" fn freeStk(mut sp: *mut stk_t) {
    stack_reset(&mut (*sp).data);
}
unsafe extern "C" fn push(mut sp: *mut stk_t, mut np: *mut Agnode_t) -> libc::c_int {
    ((*sp).markfn).expect("non-null function pointer")(np, 1 as libc::c_int);
    return stack_push(&mut (*sp).data, np as *mut libc::c_void);
}
unsafe extern "C" fn pop(mut sp: *mut stk_t) -> *mut Agnode_t {
    if stack_is_empty(&mut (*sp).data) {
        return 0 as *mut Agnode_t;
    }
    return stack_pop(&mut (*sp).data) as *mut Agnode_t;
}
unsafe extern "C" fn dfs(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut state: *mut libc::c_void,
    mut stk: *mut stk_t,
) -> size_t {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut other: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    if push(stk, n) != 0 as libc::c_int {
        return 18446744073709551615 as libc::c_ulong;
    }
    loop {
        n = pop(stk);
        if n.is_null() {
            break;
        }
        cnt = cnt.wrapping_add(1);
        if ((*stk).actionfn).is_some() {
            ((*stk).actionfn).expect("non-null function pointer")(n, state);
        }
        e = agfstedge(g, n);
        while !e.is_null() {
            other = (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node;
            if other == n {
                other = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
            }
            if ((*stk).markfn)
                .expect("non-null function pointer")(other, -(1 as libc::c_int)) == 0
            {
                if push(stk, other) != 0 as libc::c_int {
                    return 18446744073709551615 as libc::c_ulong;
                }
            }
            e = agnxtedge(g, e, n);
        }
    }
    return cnt;
}
unsafe extern "C" fn isLegal(mut p: *mut libc::c_char) -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    loop {
        let fresh6 = p;
        p = p.offset(1);
        c = *(fresh6 as *mut libc::c_uchar);
        if !(c != 0) {
            break;
        }
        if c as libc::c_int != '_' as i32
            && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn insertFn(mut n: *mut Agnode_t, mut state: *mut libc::c_void) {
    agsubnode(state as *mut Agraph_t, n, 1 as libc::c_int);
}
unsafe extern "C" fn markFn(mut n: *mut Agnode_t, mut v: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if v < 0 as libc::c_int {
        return (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark as libc::c_int;
    }
    ret = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).mark as libc::c_int;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .mark = v as libc::c_char as size_t;
    return ret;
}
unsafe extern "C" fn setPrefix(
    mut pfx: *mut libc::c_char,
    mut lenp: *mut size_t,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if pfx.is_null() || isLegal(pfx) == 0 {
        pfx = b"_cc_\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    len = strlen(pfx);
    if len.wrapping_add(25 as libc::c_int as libc::c_ulong) <= buflen {
        name = buf;
    } else {
        name = gmalloc(len.wrapping_add(25 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    strcpy(name, pfx);
    *lenp = len;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn pccomps(
    mut g: *mut Agraph_t,
    mut ncc: *mut libc::c_int,
    mut pfx: *mut libc::c_char,
    mut pinned: *mut bool,
) -> *mut *mut Agraph_t {
    let mut current_block: u64;
    let mut c_cnt: size_t = 0 as libc::c_int as size_t;
    let mut buffer: [libc::c_char; 128] = [0; 128];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ccs: *mut *mut Agraph_t = 0 as *mut *mut Agraph_t;
    let mut len: size_t = 0;
    let mut bnd: size_t = 10 as libc::c_int as size_t;
    let mut pin: bool = 0 as libc::c_int != 0;
    let mut stk: stk_t = stk_t {
        data: gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        },
        actionfn: None,
        markfn: None,
    };
    let mut error: libc::c_int = 0 as libc::c_int;
    if agnnodes(g) == 0 as libc::c_int {
        *ncc = 0 as libc::c_int;
        return 0 as *mut *mut Agraph_t;
    }
    name = setPrefix(pfx, &mut len, buffer.as_mut_ptr(), 128 as libc::c_int as size_t);
    ccs = gcalloc(bnd, ::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong)
        as *mut *mut Agraph_t;
    initStk(
        &mut stk,
        Some(insertFn as unsafe extern "C" fn(*mut Agnode_t, *mut libc::c_void) -> ()),
        Some(markFn as unsafe extern "C" fn(*mut Agnode_t, libc::c_int) -> libc::c_int),
    );
    n = agfstnode(g);
    while !n.is_null() {
        (stk.markfn).expect("non-null function pointer")(n, 0 as libc::c_int);
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    loop {
        if n.is_null() {
            current_block = 14359455889292382949;
            break;
        }
        if !((stk.markfn).expect("non-null function pointer")(n, -(1 as libc::c_int))
            != 0
            || !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pinned
                as libc::c_int == 3 as libc::c_int))
        {
            if out.is_null() {
                sprintf(
                    name.offset(len as isize),
                    b"%zu\0" as *const u8 as *const libc::c_char,
                    c_cnt,
                );
                out = agsubg(g, name, 1 as libc::c_int);
                agbindrec(
                    out as *mut libc::c_void,
                    b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong
                        as libc::c_uint,
                    1 as libc::c_int,
                );
                let ref mut fresh7 = *ccs.offset(c_cnt as isize);
                *fresh7 = out;
                c_cnt = c_cnt.wrapping_add(1);
                pin = 1 as libc::c_int != 0;
            }
            if dfs(g, n, out as *mut libc::c_void, &mut stk)
                == 18446744073709551615 as libc::c_ulong
            {
                error = 1 as libc::c_int;
                current_block = 4142149688065477410;
                break;
            }
        }
        n = agnxtnode(g, n);
    }
    match current_block {
        14359455889292382949 => {
            n = agfstnode(g);
            while !n.is_null() {
                if !((stk.markfn)
                    .expect("non-null function pointer")(n, -(1 as libc::c_int)) != 0)
                {
                    sprintf(
                        name.offset(len as isize),
                        b"%zu\0" as *const u8 as *const libc::c_char,
                        c_cnt,
                    );
                    out = agsubg(g, name, 1 as libc::c_int);
                    agbindrec(
                        out as *mut libc::c_void,
                        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong
                            as libc::c_uint,
                        1 as libc::c_int,
                    );
                    if dfs(g, n, out as *mut libc::c_void, &mut stk)
                        == 18446744073709551615 as libc::c_ulong
                    {
                        error = 1 as libc::c_int;
                        break;
                    } else {
                        if c_cnt == bnd {
                            bnd = (bnd as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            ccs = grealloc(
                                ccs as *mut libc::c_void,
                                bnd
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong,
                                    ),
                            ) as *mut *mut Agraph_t;
                        }
                        let ref mut fresh8 = *ccs.offset(c_cnt as isize);
                        *fresh8 = out;
                        c_cnt = c_cnt.wrapping_add(1);
                    }
                }
                n = agnxtnode(g, n);
            }
        }
        _ => {}
    }
    freeStk(&mut stk);
    if name != buffer.as_mut_ptr() {
        free(name as *mut libc::c_void);
    }
    if error != 0 {
        *ncc = 0 as libc::c_int;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < c_cnt {
            agclose(*ccs.offset(i as isize));
            i = i.wrapping_add(1);
        }
        free(ccs as *mut libc::c_void);
        ccs = 0 as *mut *mut Agraph_t;
    } else {
        ccs = grealloc(
            ccs as *mut libc::c_void,
            c_cnt.wrapping_mul(::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong),
        ) as *mut *mut Agraph_t;
        *ncc = c_cnt as libc::c_int;
        *pinned = pin;
    }
    return ccs;
}
#[no_mangle]
pub unsafe extern "C" fn ccomps(
    mut g: *mut Agraph_t,
    mut ncc: *mut libc::c_int,
    mut pfx: *mut libc::c_char,
) -> *mut *mut Agraph_t {
    let mut c_cnt: size_t = 0 as libc::c_int as size_t;
    let mut buffer: [libc::c_char; 128] = [0; 128];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ccs: *mut *mut Agraph_t = 0 as *mut *mut Agraph_t;
    let mut len: size_t = 0;
    let mut bnd: size_t = 10 as libc::c_int as size_t;
    let mut stk: stk_t = stk_t {
        data: gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        },
        actionfn: None,
        markfn: None,
    };
    if agnnodes(g) == 0 as libc::c_int {
        *ncc = 0 as libc::c_int;
        return 0 as *mut *mut Agraph_t;
    }
    name = setPrefix(pfx, &mut len, buffer.as_mut_ptr(), 128 as libc::c_int as size_t);
    ccs = gcalloc(bnd, ::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong)
        as *mut *mut Agraph_t;
    initStk(
        &mut stk,
        Some(insertFn as unsafe extern "C" fn(*mut Agnode_t, *mut libc::c_void) -> ()),
        Some(markFn as unsafe extern "C" fn(*mut Agnode_t, libc::c_int) -> libc::c_int),
    );
    n = agfstnode(g);
    while !n.is_null() {
        (stk.markfn).expect("non-null function pointer")(n, 0 as libc::c_int);
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        if !((stk.markfn).expect("non-null function pointer")(n, -(1 as libc::c_int))
            != 0)
        {
            sprintf(
                name.offset(len as isize),
                b"%zu\0" as *const u8 as *const libc::c_char,
                c_cnt,
            );
            out = agsubg(g, name, 1 as libc::c_int);
            agbindrec(
                out as *mut libc::c_void,
                b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            if dfs(g, n, out as *mut libc::c_void, &mut stk)
                == 18446744073709551615 as libc::c_ulong
            {
                freeStk(&mut stk);
                free(ccs as *mut libc::c_void);
                if name != buffer.as_mut_ptr() {
                    free(name as *mut libc::c_void);
                }
                *ncc = 0 as libc::c_int;
                return 0 as *mut *mut Agraph_t;
            }
            if c_cnt == bnd {
                bnd = (bnd as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                ccs = grealloc(
                    ccs as *mut libc::c_void,
                    bnd
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong,
                        ),
                ) as *mut *mut Agraph_t;
            }
            let ref mut fresh9 = *ccs.offset(c_cnt as isize);
            *fresh9 = out;
            c_cnt = c_cnt.wrapping_add(1);
        }
        n = agnxtnode(g, n);
    }
    freeStk(&mut stk);
    ccs = grealloc(
        ccs as *mut libc::c_void,
        c_cnt.wrapping_mul(::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong),
    ) as *mut *mut Agraph_t;
    if name != buffer.as_mut_ptr() {
        free(name as *mut libc::c_void);
    }
    *ncc = c_cnt as libc::c_int;
    return ccs;
}
unsafe extern "C" fn deriveClusters(mut dg: *mut Agraph_t, mut g: *mut Agraph_t) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        if strncmp(
            agnameof(subg as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            dn = agnode(dg, agnameof(subg as *mut libc::c_void), 1 as libc::c_int);
            agbindrec(
                dn as *mut libc::c_void,
                b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<ccgnodeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            let ref mut fresh10 = (*((*dn).base.data as *mut ccgnodeinfo_t)).ptr.g;
            *fresh10 = subg;
            n = agfstnode(subg);
            while !n.is_null() {
                if !((*(aggetrec(
                    n as *mut libc::c_void,
                    b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut ccgnodeinfo_t))
                    .ptr
                    .n)
                    .is_null()
                {
                    fprintf(
                        stderr,
                        b"Error: node \"%s\" belongs to two non-nested clusters \"%s\" and \"%s\"\n\0"
                            as *const u8 as *const libc::c_char,
                        agnameof(n as *mut libc::c_void),
                        agnameof(subg as *mut libc::c_void),
                        agnameof(
                            (*(aggetrec(
                                n as *mut libc::c_void,
                                b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int,
                            ) as *mut ccgnodeinfo_t))
                                .ptr
                                .n as *mut libc::c_void,
                        ),
                    );
                }
                let ref mut fresh11 = (*(aggetrec(
                    n as *mut libc::c_void,
                    b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut ccgnodeinfo_t))
                    .ptr
                    .n;
                *fresh11 = dn;
                n = agnxtnode(subg, n);
            }
        } else {
            deriveClusters(dg, subg);
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn deriveGraph(mut g: *mut Agraph_t) -> *mut Agraph_t {
    let mut dg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    dg = agopen(
        b"dg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agstrictundirected,
        0 as *mut Agdisc_t,
    );
    deriveClusters(dg, g);
    n = agfstnode(g);
    while !n.is_null() {
        if ((*(aggetrec(
            n as *mut libc::c_void,
            b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut ccgnodeinfo_t))
            .ptr
            .n)
            .is_null()
        {
            dn = agnode(dg, agnameof(n as *mut libc::c_void), 1 as libc::c_int);
            agbindrec(
                dn as *mut libc::c_void,
                b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<ccgnodeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            let ref mut fresh12 = (*((*dn).base.data as *mut ccgnodeinfo_t)).ptr.n;
            *fresh12 = n;
            let ref mut fresh13 = (*(aggetrec(
                n as *mut libc::c_void,
                b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut ccgnodeinfo_t))
                .ptr
                .n;
            *fresh13 = dn;
        }
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
        let mut hd: *mut Agnode_t = 0 as *mut Agnode_t;
        let mut tl: *mut Agnode_t = (*(aggetrec(
            n as *mut libc::c_void,
            b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut ccgnodeinfo_t))
            .ptr
            .n;
        e = agfstout(g, n);
        while !e.is_null() {
            hd = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
            hd = (*(aggetrec(
                hd as *mut libc::c_void,
                b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut ccgnodeinfo_t))
                .ptr
                .n;
            if !(hd == tl) {
                if hd > tl {
                    agedge(dg, tl, hd, 0 as *mut libc::c_char, 1 as libc::c_int);
                } else {
                    agedge(dg, hd, tl, 0 as *mut libc::c_char, 1 as libc::c_int);
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    return dg;
}
unsafe extern "C" fn unionNodes(mut dg: *mut Agraph_t, mut g: *mut Agraph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut clust: *mut Agraph_t = 0 as *mut Agraph_t;
    dn = agfstnode(dg);
    while !dn.is_null() {
        if ((*((*((*dn).base.data as *mut ccgnodeinfo_t)).ptr.v as *mut Agobj_t)).tag)
            .objtype() as libc::c_int == 1 as libc::c_int
        {
            agsubnode(
                g,
                (*((*dn).base.data as *mut ccgnodeinfo_t)).ptr.n,
                1 as libc::c_int,
            );
        } else {
            clust = (*((*dn).base.data as *mut ccgnodeinfo_t)).ptr.g;
            n = agfstnode(clust);
            while !n.is_null() {
                agsubnode(g, n, 1 as libc::c_int);
                n = agnxtnode(clust, n);
            }
        }
        dn = agnxtnode(dg, dn);
    }
}
unsafe extern "C" fn clMarkFn(mut n: *mut Agnode_t, mut v: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if v < 0 as libc::c_int {
        return (*((*n).base.data as *mut ccgnodeinfo_t)).mark as libc::c_int;
    }
    ret = (*((*n).base.data as *mut ccgnodeinfo_t)).mark as libc::c_int;
    (*((*n).base.data as *mut ccgnodeinfo_t)).mark = v as libc::c_char;
    return ret;
}
unsafe extern "C" fn node_induce(
    mut g: *mut Agraph_t,
    mut eg: *mut Agraph_t,
) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut e_cnt: libc::c_int = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(eg, n);
        while !e.is_null() {
            if !(agsubnode(
                g,
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                0 as libc::c_int,
            ))
                .is_null()
            {
                agsubedge(g, e, 1 as libc::c_int);
                e_cnt += 1;
            }
            e = agnxtout(eg, e);
        }
        n = agnxtnode(g, n);
    }
    return e_cnt;
}
#[no_mangle]
pub unsafe extern "C" fn mapClust(mut cl: *mut Agraph_t) -> *mut Agraph_t {
    let mut op: *mut orig_t = aggetrec(
        cl as *mut libc::c_void,
        b"orig\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut orig_t;
    if !op.is_null() {} else {
        __assert_fail(
            b"op\0" as *const u8 as *const libc::c_char,
            b"ccomps.c\0" as *const u8 as *const libc::c_char,
            477 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"Agraph_t *mapClust(Agraph_t *)\0"))
                .as_ptr(),
        );
    }
    return (*op).orig;
}
unsafe extern "C" fn projectG(
    mut subg: *mut Agraph_t,
    mut g: *mut Agraph_t,
    mut inCluster: libc::c_int,
) -> *mut Agraph_t {
    let mut proj: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut m: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut op: *mut orig_t = 0 as *mut orig_t;
    n = agfstnode(subg);
    while !n.is_null() {
        m = agnode(g, agnameof(n as *mut libc::c_void), 0 as libc::c_int);
        if !m.is_null() {
            if proj.is_null() {
                proj = agsubg(g, agnameof(subg as *mut libc::c_void), 1 as libc::c_int);
            }
            agsubnode(proj, m, 1 as libc::c_int);
        }
        n = agnxtnode(subg, n);
    }
    if proj.is_null() && inCluster != 0 {
        proj = agsubg(g, agnameof(subg as *mut libc::c_void), 1 as libc::c_int);
    }
    if !proj.is_null() {
        node_induce(proj, subg);
        agcopyattr(subg as *mut libc::c_void, proj as *mut libc::c_void);
        if strncmp(
            agnameof(proj as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            op = agbindrec(
                proj as *mut libc::c_void,
                b"orig\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<orig_t>() as libc::c_ulong as libc::c_uint,
                0 as libc::c_int,
            ) as *mut orig_t;
            let ref mut fresh14 = (*op).orig;
            *fresh14 = subg;
        }
    }
    return proj;
}
unsafe extern "C" fn subgInduce(
    mut root: *mut Agraph_t,
    mut g: *mut Agraph_t,
    mut inCluster: libc::c_int,
) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut proj: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut in_cluster: libc::c_int = 0;
    subg = agfstsubg(root);
    while !subg.is_null() {
        if !((*(aggetrec(
            subg as *mut libc::c_void,
            b"ccgraphinfo\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut ccgraphinfo_t))
            .cc_subg != 0)
        {
            proj = projectG(subg, g, inCluster);
            if !proj.is_null() {
                in_cluster = (inCluster != 0
                    || strncmp(
                        agnameof(subg as *mut libc::c_void),
                        b"cluster\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int) as libc::c_int;
                subgInduce(subg, proj, in_cluster);
            }
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn subGInduce(mut g: *mut Agraph_t, mut out: *mut Agraph_t) {
    subgInduce(g, out, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cccomps(
    mut g: *mut Agraph_t,
    mut ncc: *mut libc::c_int,
    mut pfx: *mut libc::c_char,
) -> *mut *mut Agraph_t {
    let mut dg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n_cnt: size_t = 0;
    let mut c_cnt: size_t = 0;
    let mut e_cnt: size_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut dout: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut buffer: [libc::c_char; 128] = [0; 128];
    let mut ccs: *mut *mut Agraph_t = 0 as *mut *mut Agraph_t;
    let mut stk: stk_t = stk_t {
        data: gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        },
        actionfn: None,
        markfn: None,
    };
    let mut len: size_t = 0;
    let mut sz: libc::c_int = ::std::mem::size_of::<ccgraphinfo_t>() as libc::c_ulong
        as libc::c_int;
    if agnnodes(g) == 0 as libc::c_int {
        *ncc = 0 as libc::c_int;
        return 0 as *mut *mut Agraph_t;
    }
    aginit(
        g,
        0 as libc::c_int,
        b"ccgraphinfo\0" as *const u8 as *const libc::c_char,
        -sz,
        0 as libc::c_int,
    );
    aginit(
        g,
        1 as libc::c_int,
        b"ccgnodeinfo\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<ccgnodeinfo_t>() as libc::c_ulong as libc::c_int,
        0 as libc::c_int,
    );
    name = setPrefix(pfx, &mut len, buffer.as_mut_ptr(), 128 as libc::c_int as size_t);
    dg = deriveGraph(g);
    ccs = gcalloc(
        agnnodes(dg) as size_t,
        ::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong,
    ) as *mut *mut Agraph_t;
    initStk(
        &mut stk,
        Some(insertFn as unsafe extern "C" fn(*mut Agnode_t, *mut libc::c_void) -> ()),
        Some(clMarkFn as unsafe extern "C" fn(*mut Agnode_t, libc::c_int) -> libc::c_int),
    );
    c_cnt = 0 as libc::c_int as size_t;
    dn = agfstnode(dg);
    while !dn.is_null() {
        if !((stk.markfn).expect("non-null function pointer")(dn, -(1 as libc::c_int))
            != 0)
        {
            sprintf(
                name.offset(len as isize),
                b"%zu\0" as *const u8 as *const libc::c_char,
                c_cnt,
            );
            dout = agsubg(dg, name, 1 as libc::c_int);
            out = agsubg(g, name, 1 as libc::c_int);
            agbindrec(
                out as *mut libc::c_void,
                b"ccgraphinfo\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<ccgraphinfo_t>() as libc::c_ulong as libc::c_uint,
                0 as libc::c_int,
            );
            (*(aggetrec(
                out as *mut libc::c_void,
                b"ccgraphinfo\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut ccgraphinfo_t))
                .cc_subg = 1 as libc::c_int as libc::c_char;
            n_cnt = dfs(dg, dn, dout as *mut libc::c_void, &mut stk);
            if n_cnt == 18446744073709551615 as libc::c_ulong {
                agclose(dg);
                agclean(
                    g,
                    0 as libc::c_int,
                    b"ccgraphinfo\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                agclean(
                    g,
                    1 as libc::c_int,
                    b"ccgnodeinfo\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                freeStk(&mut stk);
                free(ccs as *mut libc::c_void);
                if name != buffer.as_mut_ptr() {
                    free(name as *mut libc::c_void);
                }
                *ncc = 0 as libc::c_int;
                return 0 as *mut *mut Agraph_t;
            }
            unionNodes(dout, out);
            e_cnt = nodeInduce(out) as size_t;
            subGInduce(g, out);
            let ref mut fresh15 = *ccs.offset(c_cnt as isize);
            *fresh15 = out;
            agdelete(dg, dout as *mut libc::c_void);
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"(%4zu) %7zu nodes %7zu edges\n\0" as *const u8
                        as *const libc::c_char,
                    c_cnt,
                    n_cnt,
                    e_cnt,
                );
            }
            c_cnt = c_cnt.wrapping_add(1);
        }
        dn = agnxtnode(dg, dn);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"       %7d nodes %7d edges %7zu components %s\n\0" as *const u8
                as *const libc::c_char,
            agnnodes(g),
            agnedges(g),
            c_cnt,
            agnameof(g as *mut libc::c_void),
        );
    }
    agclose(dg);
    agclean(
        g,
        0 as libc::c_int,
        b"ccgraphinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    agclean(
        g,
        1 as libc::c_int,
        b"ccgnodeinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    freeStk(&mut stk);
    ccs = grealloc(
        ccs as *mut libc::c_void,
        c_cnt.wrapping_mul(::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong),
    ) as *mut *mut Agraph_t;
    if name != buffer.as_mut_ptr() {
        free(name as *mut libc::c_void);
    }
    *ncc = c_cnt as libc::c_int;
    return ccs;
}
#[no_mangle]
pub unsafe extern "C" fn isConnected(mut g: *mut Agraph_t) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut stk: stk_t = stk_t {
        data: gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        },
        actionfn: None,
        markfn: None,
    };
    if agnnodes(g) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    initStk(
        &mut stk,
        None,
        Some(markFn as unsafe extern "C" fn(*mut Agnode_t, libc::c_int) -> libc::c_int),
    );
    n = agfstnode(g);
    while !n.is_null() {
        (stk.markfn).expect("non-null function pointer")(n, 0 as libc::c_int);
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    cnt = dfs(g, agfstnode(g), 0 as *mut libc::c_void, &mut stk);
    freeStk(&mut stk);
    if cnt == 18446744073709551615 as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if cnt != agnnodes(g) as size_t {
        ret = 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn nodeInduce(mut g: *mut Agraph_t) -> libc::c_int {
    return node_induce(g, (*g).root);
}
