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
    pub type gvrender_engine_s;
    pub type GVC_s;
    pub type _GData;
    pub type _PangoFontDescription;
    pub type _PangoAttrList;
    pub type _GtkTreePath;
    pub type _GtkTreeModel;
    pub type _GtkTreeViewPrivate;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn g_free(mem: gpointer);
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_object_set(object: gpointer, first_property_name: *const gchar, _: ...);
    fn g_object_set_data(object: *mut GObject, key: *const gchar, data: gpointer);
    fn gtk_widget_show(widget: *mut GtkWidget);
    fn gtk_tree_path_new_from_string(path: *const gchar) -> *mut GtkTreePath;
    fn gtk_tree_path_free(path: *mut GtkTreePath);
    fn gtk_tree_model_get_type() -> GType;
    fn gtk_tree_model_get_iter(
        tree_model: *mut GtkTreeModel,
        iter: *mut GtkTreeIter,
        path: *mut GtkTreePath,
    ) -> gboolean;
    fn gtk_tree_model_get(tree_model: *mut GtkTreeModel, iter: *mut GtkTreeIter, _: ...);
    fn gtk_cell_renderer_text_new() -> *mut GtkCellRenderer;
    fn gtk_list_store_get_type() -> GType;
    fn gtk_list_store_new(n_columns: gint, _: ...) -> *mut GtkListStore;
    fn gtk_list_store_set(list_store: *mut GtkListStore, iter: *mut GtkTreeIter, _: ...);
    fn gtk_tree_view_get_type() -> GType;
    fn gtk_tree_view_set_model(tree_view: *mut GtkTreeView, model: *mut GtkTreeModel);
    fn gtk_tree_view_insert_column_with_attributes(
        tree_view: *mut GtkTreeView,
        position: gint,
        title: *const gchar,
        cell: *mut GtkCellRenderer,
        _: ...
    ) -> gint;
    fn gtk_init(argc: *mut libc::c_int, argv: *mut *mut *mut libc::c_char);
    fn gtk_set_locale() -> *mut gchar;
    fn gtk_main();
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XDisplayName(_: *const libc::c_char) -> *mut libc::c_char;
    fn create_window1() -> *mut GtkWidget;
    fn lookup_widget(widget: *mut GtkWidget, widget_name: *const gchar) -> *mut GtkWidget;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_engine_s {
    pub initialize: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub format: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub finalize: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
}
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type guint8 = libc::c_uchar;
pub type guint16 = libc::c_ushort;
pub type guint32 = libc::c_uint;
pub type gint64 = libc::c_long;
pub type guint64 = libc::c_ulong;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type glong = libc::c_long;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gfloat = libc::c_float;
pub type gdouble = libc::c_double;
pub type gpointer = *mut libc::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_4; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
pub type GConnectFlags = libc::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GInitiallyUnowned = _GObject;
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoColor {
    pub red: guint16,
    pub green: guint16,
    pub blue: guint16,
}
pub type PangoColor = _PangoColor;
pub type PangoUnderline = libc::c_uint;
pub const PANGO_UNDERLINE_ERROR_LINE: PangoUnderline = 7;
pub const PANGO_UNDERLINE_DOUBLE_LINE: PangoUnderline = 6;
pub const PANGO_UNDERLINE_SINGLE_LINE: PangoUnderline = 5;
pub const PANGO_UNDERLINE_ERROR: PangoUnderline = 4;
pub const PANGO_UNDERLINE_LOW: PangoUnderline = 3;
pub const PANGO_UNDERLINE_DOUBLE: PangoUnderline = 2;
pub const PANGO_UNDERLINE_SINGLE: PangoUnderline = 1;
pub const PANGO_UNDERLINE_NONE: PangoUnderline = 0;
pub type PangoAttrList = _PangoAttrList;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const PANGO_WRAP_WORD_CHAR: C2RustUnnamed_5 = 2;
pub const PANGO_WRAP_CHAR: C2RustUnnamed_5 = 1;
pub const PANGO_WRAP_WORD: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkRectangle {
    pub x: gint,
    pub y: gint,
    pub width: gint,
    pub height: gint,
}
pub type GdkRectangle = _GdkRectangle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkColor {
    pub pixel: guint32,
    pub red: guint16,
    pub green: guint16,
    pub blue: guint16,
}
pub type GdkColor = _GdkColor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkColormap {
    pub parent_instance: GObject,
    pub size: gint,
    pub colors: *mut GdkColor,
    pub visual: *mut GdkVisual,
    pub windowing_data: gpointer,
}
pub type GdkVisual = _GdkVisual;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkVisual {
    pub parent_instance: GObject,
    pub type_0: GdkVisualType,
    pub depth: gint,
    pub byte_order: GdkByteOrder,
    pub colormap_size: gint,
    pub bits_per_rgb: gint,
    pub red_mask: guint32,
    pub red_shift: gint,
    pub red_prec: gint,
    pub green_mask: guint32,
    pub green_shift: gint,
    pub green_prec: gint,
    pub blue_mask: guint32,
    pub blue_shift: gint,
    pub blue_prec: gint,
}
pub type GdkByteOrder = libc::c_uint;
pub const GDK_MSB_FIRST: GdkByteOrder = 1;
pub const GDK_LSB_FIRST: GdkByteOrder = 0;
pub type GdkVisualType = libc::c_uint;
pub const GDK_VISUAL_DIRECT_COLOR: GdkVisualType = 5;
pub const GDK_VISUAL_TRUE_COLOR: GdkVisualType = 4;
pub const GDK_VISUAL_PSEUDO_COLOR: GdkVisualType = 3;
pub const GDK_VISUAL_STATIC_COLOR: GdkVisualType = 2;
pub const GDK_VISUAL_GRAYSCALE: GdkVisualType = 1;
pub const GDK_VISUAL_STATIC_GRAY: GdkVisualType = 0;
pub type GdkColormap = _GdkColormap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkFont {
    pub type_0: GdkFontType,
    pub ascent: gint,
    pub descent: gint,
}
pub type GdkFontType = libc::c_uint;
pub const GDK_FONT_FONTSET: GdkFontType = 1;
pub const GDK_FONT_FONT: GdkFontType = 0;
pub type GdkFont = _GdkFont;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkGC {
    pub parent_instance: GObject,
    pub clip_x_origin: gint,
    pub clip_y_origin: gint,
    pub ts_x_origin: gint,
    pub ts_y_origin: gint,
    pub colormap: *mut GdkColormap,
}
pub type GdkGC = _GdkGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GdkDrawable {
    pub parent_instance: GObject,
}
pub type GdkPixmap = _GdkDrawable;
pub type GdkWindow = _GdkDrawable;
pub type GtkSortType = libc::c_uint;
pub const GTK_SORT_DESCENDING: GtkSortType = 1;
pub const GTK_SORT_ASCENDING: GtkSortType = 0;
pub type GtkObject = _GtkObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkObject {
    pub parent_instance: GInitiallyUnowned,
    pub flags: guint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkStyle {
    pub parent_instance: GObject,
    pub fg: [GdkColor; 5],
    pub bg: [GdkColor; 5],
    pub light: [GdkColor; 5],
    pub dark: [GdkColor; 5],
    pub mid: [GdkColor; 5],
    pub text: [GdkColor; 5],
    pub base: [GdkColor; 5],
    pub text_aa: [GdkColor; 5],
    pub black: GdkColor,
    pub white: GdkColor,
    pub font_desc: *mut PangoFontDescription,
    pub xthickness: gint,
    pub ythickness: gint,
    pub fg_gc: [*mut GdkGC; 5],
    pub bg_gc: [*mut GdkGC; 5],
    pub light_gc: [*mut GdkGC; 5],
    pub dark_gc: [*mut GdkGC; 5],
    pub mid_gc: [*mut GdkGC; 5],
    pub text_gc: [*mut GdkGC; 5],
    pub base_gc: [*mut GdkGC; 5],
    pub text_aa_gc: [*mut GdkGC; 5],
    pub black_gc: *mut GdkGC,
    pub white_gc: *mut GdkGC,
    pub bg_pixmap: [*mut GdkPixmap; 5],
    pub attach_count: gint,
    pub depth: gint,
    pub colormap: *mut GdkColormap,
    pub private_font: *mut GdkFont,
    pub private_font_desc: *mut PangoFontDescription,
    pub rc_style: *mut GtkRcStyle,
    pub styles: *mut GSList,
    pub property_cache: *mut GArray,
    pub icon_factories: *mut GSList,
}
pub type GtkRcStyle = _GtkRcStyle;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkRcStyle {
    pub parent_instance: GObject,
    pub name: *mut gchar,
    pub bg_pixmap_name: [*mut gchar; 5],
    pub font_desc: *mut PangoFontDescription,
    pub color_flags: [GtkRcFlags; 5],
    pub fg: [GdkColor; 5],
    pub bg: [GdkColor; 5],
    pub text: [GdkColor; 5],
    pub base: [GdkColor; 5],
    pub xthickness: gint,
    pub ythickness: gint,
    pub rc_properties: *mut GArray,
    pub rc_style_lists: *mut GSList,
    pub icon_factories: *mut GSList,
    #[bitfield(name = "engine_specified", ty = "guint", bits = "0..=0")]
    pub engine_specified: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GtkRcFlags = libc::c_uint;
pub const GTK_RC_BASE: GtkRcFlags = 8;
pub const GTK_RC_TEXT: GtkRcFlags = 4;
pub const GTK_RC_BG: GtkRcFlags = 2;
pub const GTK_RC_FG: GtkRcFlags = 1;
pub type GtkStyle = _GtkStyle;
pub type GtkWidget = _GtkWidget;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkWidget {
    pub object: GtkObject,
    pub private_flags: guint16,
    pub state: guint8,
    pub saved_state: guint8,
    pub name: *mut gchar,
    pub style: *mut GtkStyle,
    pub requisition: GtkRequisition,
    pub allocation: GtkAllocation,
    pub window: *mut GdkWindow,
    pub parent: *mut GtkWidget,
}
pub type GtkAllocation = GdkRectangle;
pub type GtkRequisition = _GtkRequisition;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkRequisition {
    pub width: gint,
    pub height: gint,
}
pub type GtkContainer = _GtkContainer;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkContainer {
    pub widget: GtkWidget,
    pub focus_child: *mut GtkWidget,
    #[bitfield(name = "border_width", ty = "guint", bits = "0..=15")]
    #[bitfield(name = "need_resize", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "resize_mode", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "reallocate_redraws", ty = "guint", bits = "19..=19")]
    #[bitfield(name = "has_focus_chain", ty = "guint", bits = "20..=20")]
    pub border_width_need_resize_resize_mode_reallocate_redraws_has_focus_chain: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkCellRenderer {
    pub parent: GtkObject,
    pub xalign: gfloat,
    pub yalign: gfloat,
    pub width: gint,
    pub height: gint,
    pub xpad: guint16,
    pub ypad: guint16,
    #[bitfield(name = "mode", ty = "guint", bits = "0..=1")]
    #[bitfield(name = "visible", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "is_expander", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "is_expanded", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "cell_background_set", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "sensitive", ty = "guint", bits = "6..=6")]
    #[bitfield(name = "editing", ty = "guint", bits = "7..=7")]
    pub mode_visible_is_expander_is_expanded_cell_background_set_sensitive_editing: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type GtkCellRenderer = _GtkCellRenderer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkTreeIter {
    pub stamp: gint,
    pub user_data: gpointer,
    pub user_data2: gpointer,
    pub user_data3: gpointer,
}
pub type GtkTreeIter = _GtkTreeIter;
pub type GtkTreePath = _GtkTreePath;
pub type GtkTreeModel = _GtkTreeModel;
pub type GtkTreeIterCompareFunc = Option<
    unsafe extern "C" fn(*mut GtkTreeModel, *mut GtkTreeIter, *mut GtkTreeIter, gpointer) -> gint,
>;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkCellRendererText {
    pub parent: GtkCellRenderer,
    pub text: *mut gchar,
    pub font: *mut PangoFontDescription,
    pub font_scale: gdouble,
    pub foreground: PangoColor,
    pub background: PangoColor,
    pub extra_attrs: *mut PangoAttrList,
    pub underline_style: PangoUnderline,
    pub rise: gint,
    pub fixed_height_rows: gint,
    #[bitfield(name = "strikethrough", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "editable", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "scale_set", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "foreground_set", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "background_set", ty = "guint", bits = "4..=4")]
    #[bitfield(name = "underline_set", ty = "guint", bits = "5..=5")]
    #[bitfield(name = "rise_set", ty = "guint", bits = "6..=6")]
    #[bitfield(name = "strikethrough_set", ty = "guint", bits = "7..=7")]
    #[bitfield(name = "editable_set", ty = "guint", bits = "8..=8")]
    #[bitfield(name = "calc_fixed_height", ty = "guint", bits = "9..=9")]
    pub strikethrough_editable_scale_set_foreground_set_background_set_underline_set_rise_set_strikethrough_set_editable_set_calc_fixed_height:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type GtkCellRendererText = _GtkCellRendererText;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _GtkListStore {
    pub parent: GObject,
    pub stamp: gint,
    pub seq: gpointer,
    pub _gtk_reserved1: gpointer,
    pub sort_list: *mut GList,
    pub n_columns: gint,
    pub sort_column_id: gint,
    pub order: GtkSortType,
    pub column_headers: *mut GType,
    pub length: gint,
    pub default_sort_func: GtkTreeIterCompareFunc,
    pub default_sort_data: gpointer,
    pub default_sort_destroy: GDestroyNotify,
    #[bitfield(name = "columns_dirty", ty = "guint", bits = "0..=0")]
    pub columns_dirty: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type GtkListStore = _GtkListStore;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GtkTreeView {
    pub parent: GtkContainer,
    pub priv_0: *mut GtkTreeViewPrivate,
}
pub type GtkTreeViewPrivate = _GtkTreeViewPrivate;
pub type GtkTreeView = _GtkTreeView;
pub type XID = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Window = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_6;
unsafe extern "C" fn attr_value_edited_cb(
    mut renderer: *mut GtkCellRendererText,
    mut pathStr: *mut gchar,
    mut newText: *mut gchar,
    mut data: gpointer,
) {
    let mut model: *mut GtkTreeModel =
        g_type_check_instance_cast(data as *mut GTypeInstance, gtk_tree_model_get_type())
            as *mut libc::c_void as *mut GtkTreeModel;
    let mut path: *mut GtkTreePath = 0 as *mut GtkTreePath;
    let mut iter: GtkTreeIter = GtkTreeIter {
        stamp: 0,
        user_data: 0 as *mut libc::c_void,
        user_data2: 0 as *mut libc::c_void,
        user_data3: 0 as *mut libc::c_void,
    };
    let mut old_attr: *mut gchar = 0 as *mut gchar;
    path = gtk_tree_path_new_from_string(pathStr);
    gtk_tree_model_get_iter(model, &mut iter, path);
    gtk_tree_model_get(
        model,
        &mut iter as *mut GtkTreeIter,
        1 as libc::c_int,
        &mut old_attr as *mut *mut gchar,
        -(1 as libc::c_int),
    );
    g_free(old_attr as gpointer);
    gtk_list_store_set(
        g_type_check_instance_cast(model as *mut GTypeInstance, gtk_list_store_get_type())
            as *mut libc::c_void as *mut GtkListStore,
        &mut iter as *mut GtkTreeIter,
        1 as libc::c_int,
        g_strdup(newText),
        -(1 as libc::c_int),
    );
    gtk_tree_path_free(path);
}
unsafe extern "C" fn gtk_initialize(mut firstjob: *mut GVJ_t) {
    let mut dpy: *mut Display = 0 as *mut Display;
    let mut display_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut scr: libc::c_int = 0;
    gtk_set_locale();
    gtk_init(0 as *mut libc::c_int, 0 as *mut *mut *mut libc::c_char);
    dpy = XOpenDisplay(display_name);
    if dpy.is_null() {
        fprintf(
            stderr,
            b"Failed to open XLIB display: %s\n\0" as *const u8 as *const libc::c_char,
            XDisplayName(0 as *const libc::c_char),
        );
        return;
    }
    scr = (*(dpy as _XPrivDisplay)).default_screen;
    (*firstjob).device_dpi.x = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).width
        as libc::c_double
        * 25.4f64
        / (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).mwidth as libc::c_double;
    (*firstjob).device_dpi.y = (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).height
        as libc::c_double
        * 25.4f64
        / (*((*(dpy as _XPrivDisplay)).screens).offset(scr as isize)).mheight as libc::c_double;
    (*firstjob).device_sets_dpi = 1 as libc::c_int != 0;
}
unsafe extern "C" fn gtk_finalize(mut firstjob: *mut GVJ_t) {
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut window1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut drawingarea1: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut drawingarea2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut treeview2: *mut GtkWidget = 0 as *mut GtkWidget;
    let mut attr_store: *mut GtkListStore = 0 as *mut GtkListStore;
    let mut value_renderer: *mut GtkCellRenderer = 0 as *mut GtkCellRenderer;
    job = firstjob;
    while !job.is_null() {
        window1 = create_window1();
        g_object_set_data(
            g_type_check_instance_cast(
                window1 as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"job\0" as *const u8 as *const libc::c_char,
            job as gpointer,
        );
        drawingarea1 = lookup_widget(
            window1,
            b"drawingarea1\0" as *const u8 as *const libc::c_char,
        );
        g_object_set_data(
            g_type_check_instance_cast(
                drawingarea1 as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"job\0" as *const u8 as *const libc::c_char,
            job as gpointer,
        );
        drawingarea2 = lookup_widget(
            window1,
            b"drawingarea2\0" as *const u8 as *const libc::c_char,
        );
        g_object_set_data(
            g_type_check_instance_cast(
                drawingarea2 as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"job\0" as *const u8 as *const libc::c_char,
            job as gpointer,
        );
        treeview2 = lookup_widget(window1, b"treeview2\0" as *const u8 as *const libc::c_char);
        g_object_set_data(
            g_type_check_instance_cast(
                treeview2 as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"job\0" as *const u8 as *const libc::c_char,
            job as gpointer,
        );
        attr_store = gtk_list_store_new(
            2 as libc::c_int,
            ((16 as libc::c_int) << 2 as libc::c_int) as GType,
            ((16 as libc::c_int) << 2 as libc::c_int) as GType,
        );
        gtk_tree_view_insert_column_with_attributes(
            g_type_check_instance_cast(treeview2 as *mut GTypeInstance, gtk_tree_view_get_type())
                as *mut libc::c_void as *mut GtkTreeView,
            -(1 as libc::c_int),
            b"Name\0" as *const u8 as *const libc::c_char,
            gtk_cell_renderer_text_new(),
            b"text\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void,
        );
        value_renderer = gtk_cell_renderer_text_new();
        g_signal_connect_data(
            g_type_check_instance_cast(
                value_renderer as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject as gpointer,
            b"edited\0" as *const u8 as *const libc::c_char,
            ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GtkCellRendererText,
                        *mut gchar,
                        *mut gchar,
                        gpointer,
                    ) -> (),
                >,
                GCallback,
            >(Some(
                attr_value_edited_cb
                    as unsafe extern "C" fn(
                        *mut GtkCellRendererText,
                        *mut gchar,
                        *mut gchar,
                        gpointer,
                    ) -> (),
            )),
            attr_store as gpointer,
            None,
            0 as GConnectFlags,
        );
        g_object_set(
            g_type_check_instance_cast(
                value_renderer as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject as gpointer,
            b"editable\0" as *const u8 as *const libc::c_char,
            (0 as libc::c_int == 0) as libc::c_int,
            b"wrap-mode\0" as *const u8 as *const libc::c_char,
            PANGO_WRAP_WORD as libc::c_int,
            b"wrap-width\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            0 as *mut libc::c_void,
        );
        gtk_tree_view_insert_column_with_attributes(
            g_type_check_instance_cast(treeview2 as *mut GTypeInstance, gtk_tree_view_get_type())
                as *mut libc::c_void as *mut GtkTreeView,
            -(1 as libc::c_int),
            b"Value\0" as *const u8 as *const libc::c_char,
            value_renderer,
            b"text\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            0 as *mut libc::c_void,
        );
        gtk_tree_view_set_model(
            g_type_check_instance_cast(treeview2 as *mut GTypeInstance, gtk_tree_view_get_type())
                as *mut libc::c_void as *mut GtkTreeView,
            g_type_check_instance_cast(attr_store as *mut GTypeInstance, gtk_tree_model_get_type())
                as *mut libc::c_void as *mut GtkTreeModel,
        );
        g_object_set_data(
            g_type_check_instance_cast(
                drawingarea1 as *mut GTypeInstance,
                ((20 as libc::c_int) << 2 as libc::c_int) as GType,
            ) as *mut libc::c_void as *mut GObject,
            b"attr_store\0" as *const u8 as *const libc::c_char,
            attr_store as gpointer,
        );
        gtk_widget_show(window1);
        job = (*job).next_active;
    }
    gtk_main();
}
static mut device_features_gtk: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 8 as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int,
        default_margin: {
            let mut init = pointf_s {
                x: 0.0f64,
                y: 0.0f64,
            };
            init
        },
        default_pagesize: {
            let mut init = pointf_s {
                x: 0.0f64,
                y: 0.0f64,
            };
            init
        },
        default_dpi: {
            let mut init = pointf_s {
                x: 96.0f64,
                y: 96.0f64,
            };
            init
        },
    };
    init
};
static mut device_engine_gtk: gvdevice_engine_t = unsafe {
    {
        let mut init = gvdevice_engine_s {
            initialize: Some(gtk_initialize as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            format: None,
            finalize: Some(gtk_finalize as unsafe extern "C" fn(*mut GVJ_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut gvdevice_types_gtk: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: b"gtk:cairo\0" as *const u8 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: &device_engine_gtk as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gtk as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: 0 as *const libc::c_char,
                quality: 0 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
