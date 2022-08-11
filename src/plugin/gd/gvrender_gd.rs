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
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    pub type htmllabel_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn gdImageCreate(sx: libc::c_int, sy: libc::c_int) -> gdImagePtr;
    fn gdImageCreateTrueColor(sx: libc::c_int, sy: libc::c_int) -> gdImagePtr;
    fn gdImageDestroy(im: gdImagePtr);
    fn gdImageLine(
        im: gdImagePtr,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
        color: libc::c_int,
    );
    fn gdImageFilledRectangle(
        im: gdImagePtr,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
        color: libc::c_int,
    );
    fn gdImageString(
        im: gdImagePtr,
        f: gdFontPtr,
        x: libc::c_int,
        y: libc::c_int,
        s: *mut libc::c_uchar,
        color: libc::c_int,
    );
    fn gdImageStringFTEx(
        im: gdImagePtr,
        brect: *mut libc::c_int,
        fg: libc::c_int,
        fontlist: *const libc::c_char,
        ptsize: libc::c_double,
        angle: libc::c_double,
        x: libc::c_int,
        y: libc::c_int,
        string: *const libc::c_char,
        strex: gdFTStringExtraPtr,
    ) -> *mut libc::c_char;
    fn gdImagePolygon(im: gdImagePtr, p: gdPointPtr, n: libc::c_int, c: libc::c_int);
    fn gdImageFilledPolygon(im: gdImagePtr, p: gdPointPtr, n: libc::c_int, c: libc::c_int);
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
    fn gdImagePaletteCopy(dst: gdImagePtr, src: gdImagePtr);
    fn gdImagePngCtx(im: gdImagePtr, out: gdIOCtxPtr);
    fn gdImageGifCtx(im: gdImagePtr, out: gdIOCtxPtr);
    fn gdImageWBMPCtx(image: gdImagePtr, fg: libc::c_int, out: gdIOCtxPtr);
    fn gdImageJpegCtx(im: gdImagePtr, out: gdIOCtxPtr, quality: libc::c_int);
    fn gdImageGd(im: gdImagePtr, out: *mut FILE);
    fn gdImageGd2(im: gdImagePtr, out: *mut FILE, cs: libc::c_int, fmt: libc::c_int);
    fn gdImageArc(
        im: gdImagePtr,
        cx: libc::c_int,
        cy: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        s: libc::c_int,
        e: libc::c_int,
        color: libc::c_int,
    );
    fn gdImageFilledEllipse(
        im: gdImagePtr,
        cx: libc::c_int,
        cy: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        color: libc::c_int,
    );
    fn gdImageFill(im: gdImagePtr, x: libc::c_int, y: libc::c_int, color: libc::c_int);
    fn gdImageSetBrush(im: gdImagePtr, brush: gdImagePtr);
    fn gdImageSetStyle(im: gdImagePtr, style: *mut libc::c_int, noOfPixels: libc::c_int);
    fn gdImageSetThickness(im: gdImagePtr, thickness: libc::c_int);
    fn gdImageAlphaBlending(im: gdImagePtr, alphaBlendingArg: libc::c_int);
    fn gdImageSaveAlpha(im: gdImagePtr, saveAlphaArg: libc::c_int);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn Bezier(
        V: *mut pointf,
        degree: libc::c_int,
        t: libc::c_double,
        Left: *mut pointf,
        Right: *mut pointf,
    ) -> pointf;
    fn gvdevice_gd_putBuf(
        context: *mut gdIOCtx,
        buffer: *const libc::c_void,
        len: libc::c_int,
    ) -> libc::c_int;
    fn gvdevice_gd_putC(context: *mut gdIOCtx, C: libc::c_int);
    static mut gdFontTiny: gdFontPtr;
    static mut gdFontSmall: gdFontPtr;
    static mut gdFontMediumBold: gdFontPtr;
    static mut gdFontLarge: gdFontPtr;
    static mut gdFontGiant: gdFontPtr;
    fn gd_psfontResolve(pa: *mut PostscriptAlias) -> *mut libc::c_char;
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
    pub getC: Option<unsafe extern "C" fn(gdIOCtxPtr) -> libc::c_int>,
    pub getBuf:
        Option<unsafe extern "C" fn(gdIOCtxPtr, *mut libc::c_void, libc::c_int) -> libc::c_int>,
    pub putC: Option<unsafe extern "C" fn(gdIOCtxPtr, libc::c_int) -> ()>,
    pub putBuf:
        Option<unsafe extern "C" fn(gdIOCtxPtr, *const libc::c_void, libc::c_int) -> libc::c_int>,
    pub seek: Option<unsafe extern "C" fn(gdIOCtxPtr, libc::c_int) -> libc::c_int>,
    pub tell: Option<unsafe extern "C" fn(gdIOCtxPtr) -> libc::c_long>,
    pub gd_free: Option<unsafe extern "C" fn(gdIOCtxPtr) -> ()>,
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
pub type interpolation_method =
    Option<unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double>;
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
pub struct gdFont {
    pub nchars: libc::c_int,
    pub offset: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub data: *mut libc::c_char,
}
pub type gdFontPtr = *mut gdFont;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdFTStringExtra {
    pub flags: libc::c_int,
    pub linespacing: libc::c_double,
    pub charmap: libc::c_int,
    pub hdpi: libc::c_int,
    pub vdpi: libc::c_int,
    pub xshow: *mut libc::c_char,
    pub fontpath: *mut libc::c_char,
}
pub type gdFTStringExtraPtr = *mut gdFTStringExtra;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdPoint {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type gdPointPtr = *mut gdPoint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvrender_engine_s {
    pub begin_job: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_job: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_graph: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_graph: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_layer:
        Option<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char, libc::c_int, libc::c_int) -> ()>,
    pub end_layer: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_page: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_page: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_cluster: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_cluster: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_nodes: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_nodes: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edges: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edges: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_node: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_node: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edge: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edge: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_anchor: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub end_anchor: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_label: Option<unsafe extern "C" fn(*mut GVJ_t, label_type) -> ()>,
    pub end_label: Option<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub textspan: Option<unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> ()>,
    pub resolve_color: Option<unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> ()>,
    pub ellipse: Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> ()>,
    pub polygon:
        Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int, libc::c_int) -> ()>,
    pub beziercurve: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut pointf,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub polyline: Option<unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> ()>,
    pub comment: Option<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> ()>,
    pub library_shape: Option<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut pointf,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
}
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
pub type PostscriptAlias = _PostscriptAlias;
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
pub type label_type = libc::c_uint;
pub const LABEL_HTML: label_type = 1;
pub const LABEL_PLAIN: label_type = 0;
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
pub struct gd_context_t {
    pub ctx: gdIOCtx,
    pub job: *mut GVJ_t,
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
    pub u: C2RustUnnamed_4,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub txt: C2RustUnnamed_5,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub span: *mut textspan_t,
    pub nspans: libc::c_short,
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
pub type C2RustUnnamed_6 = libc::c_uint;
pub const FORMAT_XBM: C2RustUnnamed_6 = 6;
pub const FORMAT_GD2: C2RustUnnamed_6 = 5;
pub const FORMAT_GD: C2RustUnnamed_6 = 4;
pub const FORMAT_WBMP: C2RustUnnamed_6 = 3;
pub const FORMAT_PNG: C2RustUnnamed_6 = 2;
pub const FORMAT_JPEG: C2RustUnnamed_6 = 1;
pub const FORMAT_GIF: C2RustUnnamed_6 = 0;
unsafe extern "C" fn gdgen_resolve_color(mut job: *mut GVJ_t, mut color: *mut gvcolor_t) {
    let mut im: gdImagePtr = (*job).context as gdImagePtr;
    let mut alpha: libc::c_int = 0;
    if im.is_null() {
        return;
    }
    alpha = (255 as libc::c_int - (*color).u.rgba[3 as libc::c_int as usize] as libc::c_int)
        * 127 as libc::c_int
        / 255 as libc::c_int;
    if alpha == 127 as libc::c_int {
        (*color).u.index = (*im).transparent;
    } else {
        (*color).u.index = gdImageColorResolveAlpha(
            im,
            (*color).u.rgba[0 as libc::c_int as usize] as libc::c_int,
            (*color).u.rgba[1 as libc::c_int as usize] as libc::c_int,
            (*color).u.rgba[2 as libc::c_int as usize] as libc::c_int,
            alpha,
        );
    }
    (*color).type_0 = COLOR_INDEX;
}
static mut transparent: libc::c_int = 0;
static mut basecolor: libc::c_int = 0;
unsafe extern "C" fn gdgen_begin_page(mut job: *mut GVJ_t) {
    let mut bgcolor_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut truecolor_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut truecolor_p: bool = 0 as libc::c_int != 0;
    let mut im: gdImagePtr = 0 as gdImagePtr;
    truecolor_str = agget(
        (*(*job).gvc).g as *mut libc::c_void,
        b"truecolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    bgcolor_str = agget(
        (*(*job).gvc).g as *mut libc::c_void,
        b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !truecolor_str.is_null()
        && *truecolor_str.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        truecolor_p = mapbool(truecolor_str);
    }
    if !bgcolor_str.is_null()
        && strcmp(
            bgcolor_str,
            b"transparent\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        if (*(*job).render.features).flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            truecolor_p = 1 as libc::c_int != 0;
        }
    }
    if (*((*((*(*job).gvc).g as *mut Agobj_t)).data as *mut Agraphinfo_t)).has_images {
        truecolor_p = 1 as libc::c_int != 0;
    }
    if (*job).external_context {
        if (*(*job).common).verbose != 0 {
            fprintf(
                stderr,
                b"%s: using existing GD image\n\0" as *const u8 as *const libc::c_char,
                (*(*job).common).cmdname,
            );
        }
        im = (*job).context as gdImagePtr;
    } else {
        if ((*job).width).wrapping_mul((*job).height) >= 2147483647 as libc::c_int as libc::c_uint {
            let mut scale: libc::c_double = sqrt(
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_div(((*job).width).wrapping_mul((*job).height))
                    as libc::c_double,
            );
            let ref mut fresh0 = (*job).width;
            *fresh0 = (*fresh0 as libc::c_double * scale) as libc::c_uint;
            let ref mut fresh1 = (*job).height;
            *fresh1 = (*fresh1 as libc::c_double * scale) as libc::c_uint;
            (*job).zoom *= scale;
            fprintf(
                stderr,
                b"%s: graph is too large for gd-renderer bitmaps. Scaling by %g to fit\n\0"
                    as *const u8 as *const libc::c_char,
                (*(*job).common).cmdname,
                scale,
            );
        }
        if truecolor_p {
            if (*(*job).common).verbose != 0 {
                fprintf(
                    stderr,
                    b"%s: allocating a %dK TrueColor GD image (%d x %d pixels)\n\0" as *const u8
                        as *const libc::c_char,
                    (*(*job).common).cmdname,
                    if ((*job).width)
                        .wrapping_mul((*job).height)
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        as libc::c_double
                        / 1024.0f64
                        >= 0 as libc::c_int as libc::c_double
                    {
                        (((*job).width)
                            .wrapping_mul((*job).height)
                            .wrapping_mul(4 as libc::c_int as libc::c_uint)
                            as libc::c_double
                            / 1024.0f64
                            + 0.5f64) as libc::c_int
                    } else {
                        (((*job).width)
                            .wrapping_mul((*job).height)
                            .wrapping_mul(4 as libc::c_int as libc::c_uint)
                            as libc::c_double
                            / 1024.0f64
                            - 0.5f64) as libc::c_int
                    },
                    (*job).width,
                    (*job).height,
                );
            }
            im = gdImageCreateTrueColor((*job).width as libc::c_int, (*job).height as libc::c_int);
        } else {
            if (*(*job).common).verbose != 0 {
                fprintf(
                    stderr,
                    b"%s: allocating a %dK PaletteColor GD image (%d x %d pixels)\n\0" as *const u8
                        as *const libc::c_char,
                    (*(*job).common).cmdname,
                    if ((*job).width).wrapping_mul((*job).height) as libc::c_double / 1024.0f64
                        >= 0 as libc::c_int as libc::c_double
                    {
                        (((*job).width).wrapping_mul((*job).height) as libc::c_double / 1024.0f64
                            + 0.5f64) as libc::c_int
                    } else {
                        (((*job).width).wrapping_mul((*job).height) as libc::c_double / 1024.0f64
                            - 0.5f64) as libc::c_int
                    },
                    (*job).width,
                    (*job).height,
                );
            }
            im = gdImageCreate((*job).width as libc::c_int, (*job).height as libc::c_int);
        }
        let ref mut fresh2 = (*job).context;
        *fresh2 = im as *mut libc::c_void;
    }
    if im.is_null() {
        ((*(*job).common).errorfn).expect("non-null function pointer")(
            b"gdImageCreate returned NULL. Malloc problem?\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    transparent = gdImageColorResolveAlpha(
        im,
        255 as libc::c_int - 1 as libc::c_int,
        255 as libc::c_int,
        255 as libc::c_int,
        127 as libc::c_int,
    );
    gdImageColorTransparent(im, transparent);
    gdImageAlphaBlending(im, 0 as libc::c_int);
    gdImageFill(
        im,
        (*im).sx / 2 as libc::c_int,
        (*im).sy / 2 as libc::c_int,
        transparent,
    );
    gdImageAlphaBlending(im, (0 as libc::c_int == 0) as libc::c_int);
}
unsafe extern "C" fn gdgen_end_page(mut job: *mut GVJ_t) {
    let mut im: gdImagePtr = (*job).context as gdImagePtr;
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
    gd_context.ctx.putBuf = Some(
        gvdevice_gd_putBuf
            as unsafe extern "C" fn(*mut gdIOCtx, *const libc::c_void, libc::c_int) -> libc::c_int,
    );
    gd_context.ctx.putC =
        Some(gvdevice_gd_putC as unsafe extern "C" fn(*mut gdIOCtx, libc::c_int) -> ());
    gd_context.job = job;
    if im.is_null() {
        return;
    }
    if !(*job).external_context {
        gdImageSaveAlpha(im, (basecolor == transparent) as libc::c_int);
        match (*job).render.id {
            0 => {
                gdImageTrueColorToPalette(im, 0 as libc::c_int, 256 as libc::c_int);
                gdImageGifCtx(im, &mut gd_context.ctx);
            }
            1 => {
                gdImageJpegCtx(im, &mut gd_context.ctx, -(1 as libc::c_int));
            }
            2 => {
                gdImagePngCtx(im, &mut gd_context.ctx);
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
            4 => {
                gdImageGd(im, (*job).output_file);
            }
            5 => {
                gdImageGd2(im, (*job).output_file, 128 as libc::c_int, 2 as libc::c_int);
            }
            6 | _ => {}
        }
        gdImageDestroy(im);
        let ref mut fresh3 = (*job).context;
        *fresh3 = 0 as *mut libc::c_void;
    }
}
unsafe extern "C" fn gdgen_missingfont(mut err: *mut libc::c_char, mut fontreq: *mut libc::c_char) {
    static mut lastmissing: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut n_errors: libc::c_int = 0 as libc::c_int;
    if n_errors >= 20 as libc::c_int {
        return;
    }
    if lastmissing.is_null() || strcmp(lastmissing, fontreq) != 0 {
        free(lastmissing as *mut libc::c_void);
        lastmissing = strdup(fontreq);
        n_errors += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gdgen_text(
    mut im: gdImagePtr,
    mut spf: pointf,
    mut epf: pointf,
    mut fontcolor: libc::c_int,
    mut fontsize: libc::c_double,
    mut fontdpi: libc::c_int,
    mut fontangle: libc::c_double,
    mut fontname: *mut libc::c_char,
    mut str: *mut libc::c_char,
) {
    let mut strex: gdFTStringExtra = gdFTStringExtra {
        flags: 0,
        linespacing: 0.,
        charmap: 0,
        hdpi: 0,
        vdpi: 0,
        xshow: 0 as *mut libc::c_char,
        fontpath: 0 as *mut libc::c_char,
    };
    let mut sp: point = point { x: 0, y: 0 };
    let mut ep: point = point { x: 0, y: 0 };
    sp.x = (if spf.x >= 0 as libc::c_int as libc::c_double {
        (spf.x + 0.5f64) as libc::c_int
    } else {
        (spf.x - 0.5f64) as libc::c_int
    });
    sp.y = (if spf.y >= 0 as libc::c_int as libc::c_double {
        (spf.y + 0.5f64) as libc::c_int
    } else {
        (spf.y - 0.5f64) as libc::c_int
    });
    ep.x = (if epf.x >= 0 as libc::c_int as libc::c_double {
        (epf.x + 0.5f64) as libc::c_int
    } else {
        (epf.x - 0.5f64) as libc::c_int
    });
    ep.y = (if epf.y >= 0 as libc::c_int as libc::c_double {
        (epf.y + 0.5f64) as libc::c_int
    } else {
        (epf.y - 0.5f64) as libc::c_int
    });
    strex.flags = 4 as libc::c_int;
    strex.vdpi = fontdpi;
    strex.hdpi = strex.vdpi;
    if !(strchr(fontname, '/' as i32)).is_null() {
        strex.flags |= 32 as libc::c_int;
    } else {
        strex.flags |= 64 as libc::c_int;
    }
    if !(fontsize <= 0.15f64) {
        if fontsize <= 1.5f64 {
            gdImageLine(im, sp.x, sp.y, ep.x, ep.y, fontcolor);
        } else {
            let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut brect: [libc::c_int; 8] = [0; 8];
            let mut fontlist: *mut libc::c_char = fontname;
            err = gdImageStringFTEx(
                im,
                brect.as_mut_ptr(),
                fontcolor,
                fontlist,
                fontsize,
                fontangle,
                sp.x,
                sp.y,
                str,
                &mut strex,
            );
            if !err.is_null() {
                gdgen_missingfont(err, fontname);
                sp.y += 2 as libc::c_int;
                if fontsize <= 8.5f64 {
                    gdImageString(
                        im,
                        gdFontTiny,
                        sp.x,
                        sp.y - 9 as libc::c_int,
                        str as *mut libc::c_uchar,
                        fontcolor,
                    );
                } else if fontsize <= 9.5f64 {
                    gdImageString(
                        im,
                        gdFontSmall,
                        sp.x,
                        sp.y - 12 as libc::c_int,
                        str as *mut libc::c_uchar,
                        fontcolor,
                    );
                } else if fontsize <= 10.5f64 {
                    gdImageString(
                        im,
                        gdFontMediumBold,
                        sp.x,
                        sp.y - 13 as libc::c_int,
                        str as *mut libc::c_uchar,
                        fontcolor,
                    );
                } else if fontsize <= 11.5f64 {
                    gdImageString(
                        im,
                        gdFontLarge,
                        sp.x,
                        sp.y - 14 as libc::c_int,
                        str as *mut libc::c_uchar,
                        fontcolor,
                    );
                } else {
                    gdImageString(
                        im,
                        gdFontGiant,
                        sp.x,
                        sp.y - 15 as libc::c_int,
                        str as *mut libc::c_uchar,
                        fontcolor,
                    );
                }
            }
        }
    }
}
unsafe extern "C" fn gdgen_textspan(mut job: *mut GVJ_t, mut p: pointf, mut span: *mut textspan_t) {
    let mut im: gdImagePtr = (*job).context as gdImagePtr;
    let mut spf: pointf = pointf { x: 0., y: 0. };
    let mut epf: pointf = pointf { x: 0., y: 0. };
    let mut spanwidth: libc::c_double =
        (*span).size.x * (*job).zoom * (*job).dpi.x / 72 as libc::c_int as libc::c_double;
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pA: *mut PostscriptAlias = 0 as *mut PostscriptAlias;
    if im.is_null() {
        return;
    }
    match (*span).just as libc::c_int {
        108 => {
            spf.x = 0.0f64;
        }
        114 => {
            spf.x = -spanwidth;
        }
        110 | _ => {
            spf.x = -spanwidth / 2 as libc::c_int as libc::c_double;
        }
    }
    epf.x = spf.x + spanwidth;
    if (*job).rotation != 0 {
        spf.y = -spf.x + p.y;
        epf.y = epf.x + p.y;
        spf.x = p.x;
        epf.x = spf.x;
    } else {
        spf.x += p.x;
        epf.x += p.x;
        spf.y = p.y
            - (*span).yoffset_centerline * (*job).zoom * (*job).dpi.x
                / 72 as libc::c_int as libc::c_double;
        epf.y = spf.y;
    }
    pA = (*(*span).font).postscript_alias;
    if !pA.is_null() {
        fontname = gd_psfontResolve(pA);
    } else {
        fontname = (*(*span).font).name;
    }
    gdgen_text(
        im,
        spf,
        epf,
        (*(*job).obj).pencolor.u.index,
        (*(*span).font).size * (*job).zoom,
        (*job).dpi.x as libc::c_int,
        if (*job).rotation != 0 {
            3.14159265358979323846f64 / 2 as libc::c_int as libc::c_double
        } else {
            0 as libc::c_int as libc::c_double
        },
        fontname,
        (*span).str_0,
    );
}
unsafe extern "C" fn gdgen_set_penstyle(
    mut job: *mut GVJ_t,
    mut im: gdImagePtr,
    mut brush: *mut gdImagePtr,
) -> libc::c_int {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut i: libc::c_int = 0;
    let mut pen: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut dashstyle: [libc::c_int; 40] = [0; 40];
    if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 10 as libc::c_int {
            dashstyle[i as usize] = (*obj).pencolor.u.index;
            i += 1;
        }
        while i < 20 as libc::c_int {
            dashstyle[i as usize] = -(6 as libc::c_int);
            i += 1;
        }
        gdImageSetStyle(im, dashstyle.as_mut_ptr(), 20 as libc::c_int);
        pen = -(2 as libc::c_int);
    } else if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            dashstyle[i as usize] = (*obj).pencolor.u.index;
            i += 1;
        }
        while i < 14 as libc::c_int {
            dashstyle[i as usize] = -(6 as libc::c_int);
            i += 1;
        }
        gdImageSetStyle(im, dashstyle.as_mut_ptr(), 12 as libc::c_int);
        pen = -(2 as libc::c_int);
    } else {
        pen = (*obj).pencolor.u.index;
    }
    width = ((*obj).penwidth * (*job).zoom) as libc::c_int;
    if (width as libc::c_double) < 1.0f64 {
        width = 1.0f64 as libc::c_int;
    }
    gdImageSetThickness(im, width);
    if width as libc::c_double != 1.0f64 {
        if (*im).trueColor != 0 {
            *brush = gdImageCreateTrueColor(width, width);
        } else {
            *brush = gdImageCreate(width, width);
            gdImagePaletteCopy(*brush, im);
        }
        gdImageFilledRectangle(
            *brush,
            0 as libc::c_int,
            0 as libc::c_int,
            width - 1 as libc::c_int,
            width - 1 as libc::c_int,
            (*obj).pencolor.u.index,
        );
        gdImageSetBrush(im, *brush);
        if pen == -(2 as libc::c_int) {
            pen = -(4 as libc::c_int);
        } else {
            pen = -(3 as libc::c_int);
        }
    }
    return pen;
}
unsafe extern "C" fn gdgen_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut im: gdImagePtr = (*job).context as gdImagePtr;
    let mut p0: pointf = pointf { x: 0., y: 0. };
    let mut p1: pointf = pointf { x: 0., y: 0. };
    let mut V: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut pen: libc::c_int = 0;
    let mut pen_ok: bool = false;
    let mut fill_ok: bool = false;
    let mut brush: gdImagePtr = 0 as gdImagePtr;
    let mut F: [gdPoint; 4] = [gdPoint { x: 0, y: 0 }; 4];
    if im.is_null() {
        return;
    }
    pen = gdgen_set_penstyle(job, im, &mut brush);
    pen_ok = pen != (*im).transparent;
    fill_ok = filled != 0 && (*obj).fillcolor.u.index != (*im).transparent;
    if pen_ok as libc::c_int != 0 || fill_ok as libc::c_int != 0 {
        V[3 as libc::c_int as usize] = *A.offset(0 as libc::c_int as isize);
        F[0 as libc::c_int as usize].x =
            (if (*A.offset(0 as libc::c_int as isize)).x >= 0 as libc::c_int as libc::c_double {
                ((*A.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
            } else {
                ((*A.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
            });
        F[0 as libc::c_int as usize].y =
            (if (*A.offset(0 as libc::c_int as isize)).y >= 0 as libc::c_int as libc::c_double {
                ((*A.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
            } else {
                ((*A.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
            });
        F[3 as libc::c_int as usize].x = (if (*A.offset((n - 1 as libc::c_int) as isize)).x
            >= 0 as libc::c_int as libc::c_double
        {
            ((*A.offset((n - 1 as libc::c_int) as isize)).x + 0.5f64) as libc::c_int
        } else {
            ((*A.offset((n - 1 as libc::c_int) as isize)).x - 0.5f64) as libc::c_int
        });
        F[3 as libc::c_int as usize].y = (if (*A.offset((n - 1 as libc::c_int) as isize)).y
            >= 0 as libc::c_int as libc::c_double
        {
            ((*A.offset((n - 1 as libc::c_int) as isize)).y + 0.5f64) as libc::c_int
        } else {
            ((*A.offset((n - 1 as libc::c_int) as isize)).y - 0.5f64) as libc::c_int
        });
        i = 0 as libc::c_int;
        while (i + 3 as libc::c_int) < n {
            V[0 as libc::c_int as usize] = V[3 as libc::c_int as usize];
            j = 1 as libc::c_int;
            while j <= 3 as libc::c_int {
                V[j as usize] = *A.offset((i + j) as isize);
                j += 1;
            }
            p0 = V[0 as libc::c_int as usize];
            step = 1 as libc::c_int;
            while step <= 10 as libc::c_int {
                p1 = Bezier(
                    V.as_mut_ptr(),
                    3 as libc::c_int,
                    step as libc::c_double / 10 as libc::c_int as libc::c_double,
                    0 as *mut pointf,
                    0 as *mut pointf,
                );
                F[1 as libc::c_int as usize].x = (if p0.x >= 0 as libc::c_int as libc::c_double {
                    (p0.x + 0.5f64) as libc::c_int
                } else {
                    (p0.x - 0.5f64) as libc::c_int
                });
                F[1 as libc::c_int as usize].y = (if p0.y >= 0 as libc::c_int as libc::c_double {
                    (p0.y + 0.5f64) as libc::c_int
                } else {
                    (p0.y - 0.5f64) as libc::c_int
                });
                F[2 as libc::c_int as usize].x = (if p1.x >= 0 as libc::c_int as libc::c_double {
                    (p1.x + 0.5f64) as libc::c_int
                } else {
                    (p1.x - 0.5f64) as libc::c_int
                });
                F[2 as libc::c_int as usize].y = (if p1.y >= 0 as libc::c_int as libc::c_double {
                    (p1.y + 0.5f64) as libc::c_int
                } else {
                    (p1.y - 0.5f64) as libc::c_int
                });
                if pen_ok {
                    gdImageLine(
                        im,
                        F[1 as libc::c_int as usize].x,
                        F[1 as libc::c_int as usize].y,
                        F[2 as libc::c_int as usize].x,
                        F[2 as libc::c_int as usize].y,
                        pen,
                    );
                }
                if fill_ok {
                    gdImageFilledPolygon(
                        im,
                        F.as_mut_ptr(),
                        4 as libc::c_int,
                        (*obj).fillcolor.u.index,
                    );
                }
                p0 = p1;
                step += 1;
            }
            i += 3 as libc::c_int;
        }
    }
    if !brush.is_null() {
        gdImageDestroy(brush);
    }
}
static mut points: *mut gdPoint = 0 as *const gdPoint as *mut gdPoint;
static mut points_allocated: libc::c_int = 0;
unsafe extern "C" fn gdgen_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut im: gdImagePtr = (*job).context as gdImagePtr;
    let mut brush: gdImagePtr = 0 as gdImagePtr;
    let mut i: libc::c_int = 0;
    let mut pen: libc::c_int = 0;
    let mut pen_ok: bool = false;
    let mut fill_ok: bool = false;
    if im.is_null() {
        return;
    }
    pen = gdgen_set_penstyle(job, im, &mut brush);
    pen_ok = pen != (*im).transparent;
    fill_ok = filled != 0 && (*obj).fillcolor.u.index != (*im).transparent;
    if pen_ok as libc::c_int != 0 || fill_ok as libc::c_int != 0 {
        if n > points_allocated {
            points = realloc(
                points as *mut libc::c_void,
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<gdPoint>() as libc::c_ulong),
            ) as *mut gdPoint;
            points_allocated = n;
        }
        i = 0 as libc::c_int;
        while i < n {
            (*points.offset(i as isize)).x =
                if (*A.offset(i as isize)).x >= 0 as libc::c_int as libc::c_double {
                    ((*A.offset(i as isize)).x + 0.5f64) as libc::c_int
                } else {
                    ((*A.offset(i as isize)).x - 0.5f64) as libc::c_int
                };
            (*points.offset(i as isize)).y =
                if (*A.offset(i as isize)).y >= 0 as libc::c_int as libc::c_double {
                    ((*A.offset(i as isize)).y + 0.5f64) as libc::c_int
                } else {
                    ((*A.offset(i as isize)).y - 0.5f64) as libc::c_int
                };
            i += 1;
        }
        if fill_ok {
            gdImageFilledPolygon(im, points, n, (*obj).fillcolor.u.index);
        }
        if pen_ok {
            gdImagePolygon(im, points, n, pen);
        }
    }
    if !brush.is_null() {
        gdImageDestroy(brush);
    }
}
unsafe extern "C" fn gdgen_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut im: gdImagePtr = (*job).context as gdImagePtr;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut pen: libc::c_int = 0;
    let mut pen_ok: bool = false;
    let mut fill_ok: bool = false;
    let mut brush: gdImagePtr = 0 as gdImagePtr;
    if im.is_null() {
        return;
    }
    pen = gdgen_set_penstyle(job, im, &mut brush);
    pen_ok = pen != (*im).transparent;
    fill_ok = filled != 0 && (*obj).fillcolor.u.index != (*im).transparent;
    dx = 2 as libc::c_int as libc::c_double
        * ((*A.offset(1 as libc::c_int as isize)).x - (*A.offset(0 as libc::c_int as isize)).x);
    dy = 2 as libc::c_int as libc::c_double
        * ((*A.offset(1 as libc::c_int as isize)).y - (*A.offset(0 as libc::c_int as isize)).y);
    if fill_ok {
        gdImageFilledEllipse(
            im,
            if (*A.offset(0 as libc::c_int as isize)).x >= 0 as libc::c_int as libc::c_double {
                ((*A.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
            } else {
                ((*A.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
            },
            if (*A.offset(0 as libc::c_int as isize)).y >= 0 as libc::c_int as libc::c_double {
                ((*A.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
            } else {
                ((*A.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
            },
            if dx >= 0 as libc::c_int as libc::c_double {
                (dx + 0.5f64) as libc::c_int
            } else {
                (dx - 0.5f64) as libc::c_int
            },
            if dy >= 0 as libc::c_int as libc::c_double {
                (dy + 0.5f64) as libc::c_int
            } else {
                (dy - 0.5f64) as libc::c_int
            },
            (*obj).fillcolor.u.index,
        );
    }
    if pen_ok {
        gdImageArc(
            im,
            if (*A.offset(0 as libc::c_int as isize)).x >= 0 as libc::c_int as libc::c_double {
                ((*A.offset(0 as libc::c_int as isize)).x + 0.5f64) as libc::c_int
            } else {
                ((*A.offset(0 as libc::c_int as isize)).x - 0.5f64) as libc::c_int
            },
            if (*A.offset(0 as libc::c_int as isize)).y >= 0 as libc::c_int as libc::c_double {
                ((*A.offset(0 as libc::c_int as isize)).y + 0.5f64) as libc::c_int
            } else {
                ((*A.offset(0 as libc::c_int as isize)).y - 0.5f64) as libc::c_int
            },
            if dx >= 0 as libc::c_int as libc::c_double {
                (dx + 0.5f64) as libc::c_int
            } else {
                (dx - 0.5f64) as libc::c_int
            },
            if dy >= 0 as libc::c_int as libc::c_double {
                (dy + 0.5f64) as libc::c_int
            } else {
                (dy - 0.5f64) as libc::c_int
            },
            0 as libc::c_int,
            360 as libc::c_int,
            pen,
        );
    }
    if !brush.is_null() {
        gdImageDestroy(brush);
    }
}
unsafe extern "C" fn gdgen_polyline(mut job: *mut GVJ_t, mut A: *mut pointf, mut n: libc::c_int) {
    let mut im: gdImagePtr = (*job).context as gdImagePtr;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut p1: pointf = pointf { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut pen: libc::c_int = 0;
    let mut pen_ok: bool = false;
    let mut brush: gdImagePtr = 0 as gdImagePtr;
    if im.is_null() {
        return;
    }
    pen = gdgen_set_penstyle(job, im, &mut brush);
    pen_ok = pen != (*im).transparent;
    if pen_ok {
        p = *A.offset(0 as libc::c_int as isize);
        i = 1 as libc::c_int;
        while i < n {
            p1 = *A.offset(i as isize);
            gdImageLine(
                im,
                if p.x >= 0 as libc::c_int as libc::c_double {
                    (p.x + 0.5f64) as libc::c_int
                } else {
                    (p.x - 0.5f64) as libc::c_int
                },
                if p.y >= 0 as libc::c_int as libc::c_double {
                    (p.y + 0.5f64) as libc::c_int
                } else {
                    (p.y - 0.5f64) as libc::c_int
                },
                if p1.x >= 0 as libc::c_int as libc::c_double {
                    (p1.x + 0.5f64) as libc::c_int
                } else {
                    (p1.x - 0.5f64) as libc::c_int
                },
                if p1.y >= 0 as libc::c_int as libc::c_double {
                    (p1.y + 0.5f64) as libc::c_int
                } else {
                    (p1.y - 0.5f64) as libc::c_int
                },
                pen,
            );
            p = p1;
            i += 1;
        }
    }
    if !brush.is_null() {
        gdImageDestroy(brush);
    }
}
static mut gdgen_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: None,
            end_job: None,
            begin_graph: None,
            end_graph: None,
            begin_layer: None,
            end_layer: None,
            begin_page: Some(gdgen_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            end_page: Some(gdgen_end_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_cluster: None,
            end_cluster: None,
            begin_nodes: None,
            end_nodes: None,
            begin_edges: None,
            end_edges: None,
            begin_node: None,
            end_node: None,
            begin_edge: None,
            end_edge: None,
            begin_anchor: None,
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                gdgen_textspan as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: Some(
                gdgen_resolve_color as unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> (),
            ),
            ellipse: Some(
                gdgen_ellipse as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                gdgen_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                gdgen_bezier
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            polyline: Some(
                gdgen_polyline as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: None,
            library_shape: None,
        };
        init
    }
};
static mut render_features_gd: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 12 as libc::c_int,
        default_pad: 4.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: RGBA_BYTE,
    };
    init
};
static mut device_features_gd: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 9 as libc::c_int,
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
static mut device_features_gd_tc: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 9 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
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
static mut device_features_gd_tc_no_writer: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int,
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
#[no_mangle]
pub static mut gvrender_gd_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GD as libc::c_int,
                type_0: b"gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: &gdgen_engine as *const gvrender_engine_t as *mut gvrender_engine_t
                    as *mut libc::c_void,
                features: &render_features_gd as *const gvrender_features_t
                    as *mut gvrender_features_t as *mut libc::c_void,
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
#[no_mangle]
pub static mut gvdevice_gd_types2: [gvplugin_installed_t; 9] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GIF as libc::c_int,
                type_0: b"gif:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd_tc as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_WBMP as libc::c_int,
                type_0: b"wbmp:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG as libc::c_int,
                type_0: b"jpe:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG as libc::c_int,
                type_0: b"jpeg:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_JPEG as libc::c_int,
                type_0: b"jpg:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG as libc::c_int,
                type_0: b"png:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd_tc as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GD as libc::c_int,
                type_0: b"gd:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd_tc_no_writer as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_GD2 as libc::c_int,
                type_0: b"gd2:gd\0" as *const u8 as *const libc::c_char,
                quality: 1 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_gd_tc_no_writer as *const gvdevice_features_t
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
