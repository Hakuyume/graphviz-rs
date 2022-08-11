#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvrender_engine_s;
    pub type GVC_s;
    fn gdImageCreateTrueColor(sx: libc::c_int, sy: libc::c_int) -> gdImagePtr;
    fn gdImageDestroy(im: gdImagePtr);
    fn gdImageColorResolveAlpha(
        im: gdImagePtr,
        r: libc::c_int,
        g: libc::c_int,
        b: libc::c_int,
        a: libc::c_int,
    ) -> libc::c_int;
    fn gdImageTrueColorToPalette(
        im: gdImagePtr,
        ditherFlag: libc::c_int,
        colorsWanted: libc::c_int,
    ) -> libc::c_int;
    fn gdImageColorTransparent(im: gdImagePtr, color: libc::c_int);
    fn gdImagePngCtx(im: gdImagePtr, out: gdIOCtxPtr);
    fn gdImageGifCtx(im: gdImagePtr, out: gdIOCtxPtr);
    fn gdImageWBMPCtx(image: gdImagePtr, fg: libc::c_int, out: gdIOCtxPtr);
    fn gdImageJpegCtx(im: gdImagePtr, out: gdIOCtxPtr, quality: libc::c_int);
    fn gdImageGd(im: gdImagePtr, out: *mut FILE);
    fn gdImageGd2(im: gdImagePtr, out: *mut FILE, cs: libc::c_int, fmt: libc::c_int);
    fn gdImageAlphaBlending(im: gdImagePtr, alphaBlendingArg: libc::c_int);
    fn gvwrite(job: *mut GVJ_t, s: *const libc::c_char, len: size_t) -> size_t;
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
pub struct gdIOCtx {
    pub getC: Option::<unsafe extern "C" fn(gdIOCtxPtr) -> libc::c_int>,
    pub getBuf: Option::<
        unsafe extern "C" fn(gdIOCtxPtr, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub putC: Option::<unsafe extern "C" fn(gdIOCtxPtr, libc::c_int) -> ()>,
    pub putBuf: Option::<
        unsafe extern "C" fn(gdIOCtxPtr, *const libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub seek: Option::<unsafe extern "C" fn(gdIOCtxPtr, libc::c_int) -> libc::c_int>,
    pub tell: Option::<unsafe extern "C" fn(gdIOCtxPtr) -> libc::c_long>,
    pub gd_free: Option::<unsafe extern "C" fn(gdIOCtxPtr) -> ()>,
    pub data: *mut libc::c_void,
}
pub type gdIOCtxPtr = *mut gdIOCtx;
pub type gdInterpolationMethod = libc::c_uint;
pub const GD_METHOD_COUNT: gdInterpolationMethod = 30;
pub const GD_WELSH: gdInterpolationMethod = 30;
pub const GD_COSINE: gdInterpolationMethod = 29;
pub const GD_CUBIC_SPLINE: gdInterpolationMethod = 28;
pub const GD_QUADRATIC_BSPLINE: gdInterpolationMethod = 27;
pub const GD_BLACKMAN_SINC: gdInterpolationMethod = 26;
pub const GD_BLACKMAN_BESSEL: gdInterpolationMethod = 25;
pub const GD_LANCZOS8: gdInterpolationMethod = 24;
pub const GD_LANCZOS3: gdInterpolationMethod = 23;
pub const GD_LINEAR: gdInterpolationMethod = 22;
pub const GD_WEIGHTED4: gdInterpolationMethod = 21;
pub const GD_TRIANGLE: gdInterpolationMethod = 20;
pub const GD_SINC: gdInterpolationMethod = 19;
pub const GD_QUADRATIC: gdInterpolationMethod = 18;
pub const GD_POWER: gdInterpolationMethod = 17;
pub const GD_NEAREST_NEIGHBOUR: gdInterpolationMethod = 16;
pub const GD_MITCHELL: gdInterpolationMethod = 15;
pub const GD_HANNING: gdInterpolationMethod = 14;
pub const GD_HAMMING: gdInterpolationMethod = 13;
pub const GD_HERMITE: gdInterpolationMethod = 12;
pub const GD_GENERALIZED_CUBIC: gdInterpolationMethod = 11;
pub const GD_GAUSSIAN: gdInterpolationMethod = 10;
pub const GD_CATMULLROM: gdInterpolationMethod = 9;
pub const GD_BSPLINE: gdInterpolationMethod = 8;
pub const GD_BOX: gdInterpolationMethod = 7;
pub const GD_BLACKMAN: gdInterpolationMethod = 6;
pub const GD_BICUBIC_FIXED: gdInterpolationMethod = 5;
pub const GD_BICUBIC: gdInterpolationMethod = 4;
pub const GD_BILINEAR_FIXED: gdInterpolationMethod = 3;
pub const GD_BESSEL: gdInterpolationMethod = 2;
pub const GD_BELL: gdInterpolationMethod = 1;
pub const GD_DEFAULT: gdInterpolationMethod = 0;
pub type interpolation_method = Option::<
    unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdImageStruct {
    pub pixels: *mut *mut libc::c_uchar,
    pub sx: libc::c_int,
    pub sy: libc::c_int,
    pub colorsTotal: libc::c_int,
    pub red: [libc::c_int; 256],
    pub green: [libc::c_int; 256],
    pub blue: [libc::c_int; 256],
    pub open: [libc::c_int; 256],
    pub transparent: libc::c_int,
    pub polyInts: *mut libc::c_int,
    pub polyAllocated: libc::c_int,
    pub brush: *mut gdImageStruct,
    pub tile: *mut gdImageStruct,
    pub brushColorMap: [libc::c_int; 256],
    pub tileColorMap: [libc::c_int; 256],
    pub styleLength: libc::c_int,
    pub stylePos: libc::c_int,
    pub style: *mut libc::c_int,
    pub interlace: libc::c_int,
    pub thick: libc::c_int,
    pub alpha: [libc::c_int; 256],
    pub trueColor: libc::c_int,
    pub tpixels: *mut *mut libc::c_int,
    pub alphaBlendingFlag: libc::c_int,
    pub saveAlphaFlag: libc::c_int,
    pub AA: libc::c_int,
    pub AA_color: libc::c_int,
    pub AA_dont_blend: libc::c_int,
    pub cx1: libc::c_int,
    pub cy1: libc::c_int,
    pub cx2: libc::c_int,
    pub cy2: libc::c_int,
    pub res_x: libc::c_uint,
    pub res_y: libc::c_uint,
    pub paletteQuantizationMethod: libc::c_int,
    pub paletteQuantizationSpeed: libc::c_int,
    pub paletteQuantizationMinQuality: libc::c_int,
    pub paletteQuantizationMaxQuality: libc::c_int,
    pub interpolation_id: gdInterpolationMethod,
    pub interpolation: interpolation_method,
}
pub type gdImage = gdImageStruct;
pub type gdImagePtr = *mut gdImage;
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
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_0 {
    pub LL: point,
    pub UR: point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvdevice_engine_s {
    pub initialize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub format: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub finalize: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
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
pub type uint64_t = __uint64_t;
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
pub struct gd_context_t {
    pub ctx: gdIOCtx,
    pub job: *mut GVJ_t,
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FORMAT_XBM: C2RustUnnamed_4 = 6;
pub const FORMAT_GD2: C2RustUnnamed_4 = 5;
pub const FORMAT_GD: C2RustUnnamed_4 = 4;
pub const FORMAT_WBMP: C2RustUnnamed_4 = 3;
pub const FORMAT_PNG: C2RustUnnamed_4 = 2;
pub const FORMAT_JPEG: C2RustUnnamed_4 = 1;
pub const FORMAT_GIF: C2RustUnnamed_4 = 0;
#[inline]
unsafe extern "C" fn get_containing_context(mut ctx: *mut gdIOCtx) -> *mut gd_context_t {
    return (ctx as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
        as *mut gd_context_t;
}
#[no_mangle]
pub unsafe extern "C" fn gvdevice_gd_putBuf(
    mut context: *mut gdIOCtx,
    mut buffer: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut gd_context: *mut gd_context_t = get_containing_context(context);
    return gvwrite((*gd_context).job, buffer as *const libc::c_char, len as size_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvdevice_gd_putC(
    mut context: *mut gdIOCtx,
    mut C: libc::c_int,
) {
    let mut gd_context: *mut gd_context_t = get_containing_context(context);
    let mut c: libc::c_char = C as libc::c_char;
    gvwrite((*gd_context).job, &mut c, 1 as libc::c_int as size_t);
}
unsafe extern "C" fn gd_format(mut job: *mut GVJ_t) {
    let mut im: gdImagePtr = 0 as *mut gdImage;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut color: libc::c_uint = 0;
    let mut alpha: libc::c_uint = 0;
    let mut data: *mut libc::c_uint = (*job).imagedata as *mut libc::c_uint;
    let mut width: libc::c_uint = (*job).width;
    let mut height: libc::c_uint = (*job).height;
    let mut gd_context: gd_context_t = {
        let mut init = gd_context_t {
            ctx: {
                let mut init = gdIOCtx {
                    getC: None,
                    getBuf: None,
                    putC: None,
                    putBuf: None,
                    seek: None,
                    tell: None,
                    gd_free: None,
                    data: 0 as *mut libc::c_void,
                };
                init
            },
            job: 0 as *mut GVJ_t,
        };
        init
    };
    gd_context
        .ctx
        .putBuf = Some(
        gvdevice_gd_putBuf
            as unsafe extern "C" fn(
                *mut gdIOCtx,
                *const libc::c_void,
                libc::c_int,
            ) -> libc::c_int,
    );
    gd_context
        .ctx
        .putC = Some(
        gvdevice_gd_putC as unsafe extern "C" fn(*mut gdIOCtx, libc::c_int) -> (),
    );
    gd_context.job = job;
    im = gdImageCreateTrueColor(width as libc::c_int, height as libc::c_int);
    match (*job).device.id {
        2 => {
            y = 0 as libc::c_int as libc::c_uint;
            while y < height {
                x = 0 as libc::c_int as libc::c_uint;
                while x < width {
                    let fresh0 = data;
                    data = data.offset(1);
                    color = *fresh0;
                    alpha = color >> 25 as libc::c_int
                        & 0x7f as libc::c_int as libc::c_uint;
                    *(*((*im).tpixels).offset(y as isize))
                        .offset(
                            x as isize,
                        ) = (color & 0xffffff as libc::c_int as libc::c_uint
                        | (0x7f as libc::c_int as libc::c_uint).wrapping_sub(alpha)
                            << 24 as libc::c_int) as libc::c_int;
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
        }
        _ => {
            gdImageColorTransparent(im, 0x7ffffffe as libc::c_int);
            gdImageAlphaBlending(im, 0 as libc::c_int);
            y = 0 as libc::c_int as libc::c_uint;
            while y < height {
                x = 0 as libc::c_int as libc::c_uint;
                while x < width {
                    let fresh1 = data;
                    data = data.offset(1);
                    color = *fresh1;
                    alpha = color >> 25 as libc::c_int
                        & 0x7f as libc::c_int as libc::c_uint;
                    if alpha >= 0x20 as libc::c_int as libc::c_uint {
                        *(*((*im).tpixels).offset(y as isize))
                            .offset(
                                x as isize,
                            ) = (color & 0xffffff as libc::c_int as libc::c_uint
                            | (0x7f as libc::c_int as libc::c_uint).wrapping_sub(alpha)
                                << 24 as libc::c_int) as libc::c_int;
                    } else {
                        *(*((*im).tpixels).offset(y as isize))
                            .offset(x as isize) = 0x7ffffffe as libc::c_int;
                    }
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
        }
    }
    match (*job).device.id {
        0 => {
            gdImageTrueColorToPalette(im, 0 as libc::c_int, 256 as libc::c_int);
            gdImageGifCtx(im, &mut gd_context.ctx);
        }
        1 => {
            gdImageJpegCtx(im, &mut gd_context.ctx, -(1 as libc::c_int));
        }
        2 => {
            gdImageTrueColorToPalette(im, 0 as libc::c_int, 256 as libc::c_int);
            gdImagePngCtx(im, &mut gd_context.ctx);
        }
        4 => {
            gdImageGd(im, (*job).output_file);
        }
        5 => {
            gdImageGd2(im, (*job).output_file, 128 as libc::c_int, 2 as libc::c_int);
        }
        3 => {
            let mut black: libc::c_int = gdImageColorResolveAlpha(
                im,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            gdImageWBMPCtx(im, black, &mut gd_context.ctx);
        }
        _ => {}
    }
    gdImageDestroy(im);
}
static mut gd_engine: gvdevice_engine_t = unsafe {
    {
        let mut init = gvdevice_engine_s {
            initialize: None,
            format: Some(gd_format as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            finalize: None,
        };
        init
    }
};
static mut device_features_gd: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 96.0f64, y: 96.0f64 };
            init
        },
    };
    init
};
static mut device_features_gd_no_writer: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 96.0f64, y: 96.0f64 };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvdevice_gd_types: [gvplugin_installed_t; 9] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF as libc::c_int,
                type_0: b"gif:cairo\0" as *const u8 as *const libc::c_char,
                quality: 10 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_WBMP as libc::c_int,
                type_0: b"wbmp:cairo\0" as *const u8 as *const libc::c_char,
                quality: 5 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG as libc::c_int,
                type_0: b"jpe:cairo\0" as *const u8 as *const libc::c_char,
                quality: 5 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG as libc::c_int,
                type_0: b"jpeg:cairo\0" as *const u8 as *const libc::c_char,
                quality: 5 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG as libc::c_int,
                type_0: b"jpg:cairo\0" as *const u8 as *const libc::c_char,
                quality: 5 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG as libc::c_int,
                type_0: b"png:cairo\0" as *const u8 as *const libc::c_char,
                quality: 5 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GD as libc::c_int,
                type_0: b"gd:cairo\0" as *const u8 as *const libc::c_char,
                quality: 5 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd_no_writer as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GD2 as libc::c_int,
                type_0: b"gd2:cairo\0" as *const u8 as *const libc::c_char,
                quality: 5 as libc::c_int,
                engine: &gd_engine as *const gvdevice_engine_t as *mut gvdevice_engine_t
                    as *mut libc::c_void,
                features: &device_features_gd_no_writer as *const gvdevice_features_t
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
