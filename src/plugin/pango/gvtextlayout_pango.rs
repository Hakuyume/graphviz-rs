#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GData;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _PangoAttrList;
    pub type _PangoLayout;
    pub type _cairo_font_options;
    pub type FT_LibraryRec_;
    pub type FT_DriverRec_;
    pub type FT_Face_InternalRec_;
    pub type FT_Size_InternalRec_;
    pub type FT_Slot_InternalRec_;
    pub type FT_SubGlyphRec_;
    pub type _FcPattern;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn xml_escape(
        s: *const libc::c_char,
        flags: xml_flags_t,
        cb: Option::<
            unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
        >,
        state: *mut libc::c_void,
    ) -> libc::c_int;
    fn g_free(mem: gpointer);
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_object_unref(object: gpointer);
    fn pango_font_description_free(desc: *mut PangoFontDescription);
    fn pango_font_description_set_size(desc: *mut PangoFontDescription, size: gint);
    fn pango_font_description_from_string(
        str: *const libc::c_char,
    ) -> *mut PangoFontDescription;
    fn pango_font_description_to_string(
        desc: *const PangoFontDescription,
    ) -> *mut libc::c_char;
    fn pango_font_describe(font: *mut PangoFont) -> *mut PangoFontDescription;
    fn pango_font_map_create_context(fontmap: *mut PangoFontMap) -> *mut PangoContext;
    fn pango_font_map_load_font(
        fontmap: *mut PangoFontMap,
        context: *mut PangoContext,
        desc: *const PangoFontDescription,
    ) -> *mut PangoFont;
    fn pango_layout_new(context: *mut PangoContext) -> *mut PangoLayout;
    fn pango_layout_set_attributes(layout: *mut PangoLayout, attrs: *mut PangoAttrList);
    fn pango_layout_set_text(
        layout: *mut PangoLayout,
        text: *const libc::c_char,
        length: libc::c_int,
    );
    fn pango_layout_set_font_description(
        layout: *mut PangoLayout,
        desc: *const PangoFontDescription,
    );
    fn pango_layout_get_extents(
        layout: *mut PangoLayout,
        ink_rect: *mut PangoRectangle,
        logical_rect: *mut PangoRectangle,
    );
    fn pango_layout_get_baseline(layout: *mut PangoLayout) -> libc::c_int;
    fn pango_parse_markup(
        markup_text: *const libc::c_char,
        length: libc::c_int,
        accel_marker: gunichar,
        attr_list: *mut *mut PangoAttrList,
        text: *mut *mut libc::c_char,
        accel_char: *mut gunichar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn cairo_font_options_create() -> *mut cairo_font_options_t;
    fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    fn cairo_font_options_set_antialias(
        options: *mut cairo_font_options_t,
        antialias: cairo_antialias_t,
    );
    fn cairo_font_options_set_subpixel_order(
        options: *mut cairo_font_options_t,
        subpixel_order: cairo_subpixel_order_t,
    );
    fn cairo_font_options_set_hint_style(
        options: *mut cairo_font_options_t,
        hint_style: cairo_hint_style_t,
    );
    fn cairo_font_options_set_hint_metrics(
        options: *mut cairo_font_options_t,
        hint_metrics: cairo_hint_metrics_t,
    );
    fn pango_cairo_font_map_new() -> *mut PangoFontMap;
    fn pango_cairo_context_set_font_options(
        context: *mut PangoContext,
        options: *const cairo_font_options_t,
    );
    fn pango_cairo_context_set_resolution(
        context: *mut PangoContext,
        dpi: libc::c_double,
    );
    fn g_type_check_instance_cast(
        instance: *mut GTypeInstance,
        iface_type: GType,
    ) -> *mut GTypeInstance;
    fn get_font_mapping(pfm: *mut PangoFontMap) -> *mut gv_font_map;
    fn pango_fc_font_get_type() -> GType;
    fn pango_fc_font_lock_face(font: *mut PangoFcFont) -> FT_Face;
    fn pango_fc_font_unlock_face(font: *mut PangoFcFont);
}
pub type size_t = libc::c_ulong;
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
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvtextlayout_engine_s {
    pub textlayout: Option::<
        unsafe extern "C" fn(*mut textspan_t, *mut *mut libc::c_char) -> bool,
    >,
}
pub type gvtextlayout_engine_t = gvtextlayout_engine_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct xml_flags_t {
    #[bitfield(name = "raw", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "dash", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "nbsp", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "utf8", ty = "libc::c_uint", bits = "3..=3")]
    pub raw_dash_nbsp_utf8: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type gunichar = guint32;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type GParamFlags = libc::c_int;
pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const G_PARAM_PRIVATE: GParamFlags = 32;
pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const G_PARAM_READWRITE: GParamFlags = 3;
pub const G_PARAM_WRITABLE: GParamFlags = 2;
pub const G_PARAM_READABLE: GParamFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
pub type GParamSpec = _GParamSpec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor: Option::<
        unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject,
    >,
    pub set_property: Option::<
        unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    >,
    pub get_property: Option::<
        unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    >,
    pub dispose: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub finalize: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub dispatch_properties_changed: Option::<
        unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> (),
    >,
    pub notify: Option::<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>,
    pub constructed: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub flags: gsize,
    pub pdummy: [gpointer; 6],
}
pub type GObjectConstructParam = _GObjectConstructParam;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}
pub type GObjectClass = _GObjectClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFont {
    pub parent_instance: GObject,
}
pub type PangoFont = _PangoFont;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFontMap {
    pub parent_instance: GObject,
}
pub type PangoFontMap = _PangoFontMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoRectangle {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type PangoRectangle = _PangoRectangle;
pub type PangoContext = _PangoContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoMatrix {
    pub xx: libc::c_double,
    pub xy: libc::c_double,
    pub yx: libc::c_double,
    pub yy: libc::c_double,
    pub x0: libc::c_double,
    pub y0: libc::c_double,
}
pub type PangoMatrix = _PangoMatrix;
pub type PangoFontDescription = _PangoFontDescription;
pub type PangoAttrList = _PangoAttrList;
pub type PangoLayout = _PangoLayout;
pub type _cairo_antialias = libc::c_uint;
pub const CAIRO_ANTIALIAS_BEST: _cairo_antialias = 6;
pub const CAIRO_ANTIALIAS_GOOD: _cairo_antialias = 5;
pub const CAIRO_ANTIALIAS_FAST: _cairo_antialias = 4;
pub const CAIRO_ANTIALIAS_SUBPIXEL: _cairo_antialias = 3;
pub const CAIRO_ANTIALIAS_GRAY: _cairo_antialias = 2;
pub const CAIRO_ANTIALIAS_NONE: _cairo_antialias = 1;
pub const CAIRO_ANTIALIAS_DEFAULT: _cairo_antialias = 0;
pub type cairo_antialias_t = _cairo_antialias;
pub type _cairo_subpixel_order = libc::c_uint;
pub const CAIRO_SUBPIXEL_ORDER_VBGR: _cairo_subpixel_order = 4;
pub const CAIRO_SUBPIXEL_ORDER_VRGB: _cairo_subpixel_order = 3;
pub const CAIRO_SUBPIXEL_ORDER_BGR: _cairo_subpixel_order = 2;
pub const CAIRO_SUBPIXEL_ORDER_RGB: _cairo_subpixel_order = 1;
pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: _cairo_subpixel_order = 0;
pub type cairo_subpixel_order_t = _cairo_subpixel_order;
pub type _cairo_hint_style = libc::c_uint;
pub const CAIRO_HINT_STYLE_FULL: _cairo_hint_style = 4;
pub const CAIRO_HINT_STYLE_MEDIUM: _cairo_hint_style = 3;
pub const CAIRO_HINT_STYLE_SLIGHT: _cairo_hint_style = 2;
pub const CAIRO_HINT_STYLE_NONE: _cairo_hint_style = 1;
pub const CAIRO_HINT_STYLE_DEFAULT: _cairo_hint_style = 0;
pub type cairo_hint_style_t = _cairo_hint_style;
pub type _cairo_hint_metrics = libc::c_uint;
pub const CAIRO_HINT_METRICS_ON: _cairo_hint_metrics = 2;
pub const CAIRO_HINT_METRICS_OFF: _cairo_hint_metrics = 1;
pub const CAIRO_HINT_METRICS_DEFAULT: _cairo_hint_metrics = 0;
pub type cairo_hint_metrics_t = _cairo_hint_metrics;
pub type cairo_font_options_t = _cairo_font_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_font_map {
    pub gv_ps_fontname: *mut libc::c_char,
    pub gv_font: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_MemoryRec_ {
    pub user: *mut libc::c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}
pub type FT_Realloc_Func = Option::<
    unsafe extern "C" fn(
        FT_Memory,
        libc::c_long,
        libc::c_long,
        *mut libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type FT_Memory = *mut FT_MemoryRec_;
pub type FT_Free_Func = Option::<
    unsafe extern "C" fn(FT_Memory, *mut libc::c_void) -> (),
>;
pub type FT_Alloc_Func = Option::<
    unsafe extern "C" fn(FT_Memory, libc::c_long) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_StreamRec_ {
    pub base: *mut libc::c_uchar,
    pub size: libc::c_ulong,
    pub pos: libc::c_ulong,
    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,
    pub memory: FT_Memory,
    pub cursor: *mut libc::c_uchar,
    pub limit: *mut libc::c_uchar,
}
pub type FT_Stream_CloseFunc = Option::<unsafe extern "C" fn(FT_Stream) -> ()>;
pub type FT_Stream = *mut FT_StreamRec_;
pub type FT_Stream_IoFunc = Option::<
    unsafe extern "C" fn(
        FT_Stream,
        libc::c_ulong,
        *mut libc::c_uchar,
        libc::c_ulong,
    ) -> libc::c_ulong,
>;
pub type FT_StreamDesc = FT_StreamDesc_;
#[derive(Copy, Clone)]
#[repr(C)]
pub union FT_StreamDesc_ {
    pub value: libc::c_long,
    pub pointer: *mut libc::c_void,
}
pub type FT_Pos = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Vector_ {
    pub x: FT_Pos,
    pub y: FT_Pos,
}
pub type FT_Vector = FT_Vector_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_BBox_ {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}
pub type FT_BBox = FT_BBox_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_ {
    pub rows: libc::c_uint,
    pub width: libc::c_uint,
    pub pitch: libc::c_int,
    pub buffer: *mut libc::c_uchar,
    pub num_grays: libc::c_ushort,
    pub pixel_mode: libc::c_uchar,
    pub palette_mode: libc::c_uchar,
    pub palette: *mut libc::c_void,
}
pub type FT_Bitmap = FT_Bitmap_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Outline_ {
    pub n_contours: libc::c_short,
    pub n_points: libc::c_short,
    pub points: *mut FT_Vector,
    pub tags: *mut libc::c_char,
    pub contours: *mut libc::c_short,
    pub flags: libc::c_int,
}
pub type FT_Outline = FT_Outline_;
pub type FT_Glyph_Format_ = libc::c_uint;
pub const FT_GLYPH_FORMAT_SVG: FT_Glyph_Format_ = 1398163232;
pub const FT_GLYPH_FORMAT_PLOTTER: FT_Glyph_Format_ = 1886154612;
pub const FT_GLYPH_FORMAT_OUTLINE: FT_Glyph_Format_ = 1869968492;
pub const FT_GLYPH_FORMAT_BITMAP: FT_Glyph_Format_ = 1651078259;
pub const FT_GLYPH_FORMAT_COMPOSITE: FT_Glyph_Format_ = 1668246896;
pub const FT_GLYPH_FORMAT_NONE: FT_Glyph_Format_ = 0;
pub type FT_Glyph_Format = FT_Glyph_Format_;
pub type FT_String = libc::c_char;
pub type FT_Short = libc::c_short;
pub type FT_UShort = libc::c_ushort;
pub type FT_Int = libc::c_int;
pub type FT_UInt = libc::c_uint;
pub type FT_Long = libc::c_long;
pub type FT_Fixed = libc::c_long;
pub type FT_Generic_Finalizer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Generic_ {
    pub data: *mut libc::c_void,
    pub finalizer: FT_Generic_Finalizer,
}
pub type FT_Generic = FT_Generic_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListNodeRec_ {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *mut libc::c_void,
}
pub type FT_ListNode = *mut FT_ListNodeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListRec_ {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}
pub type FT_ListRec = FT_ListRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Glyph_Metrics_ {
    pub width: FT_Pos,
    pub height: FT_Pos,
    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,
    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}
pub type FT_Glyph_Metrics = FT_Glyph_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_Size_ {
    pub height: FT_Short,
    pub width: FT_Short,
    pub size: FT_Pos,
    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos,
}
pub type FT_Bitmap_Size = FT_Bitmap_Size_;
pub type FT_Library = *mut FT_LibraryRec_;
pub type FT_Driver = *mut FT_DriverRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_FaceRec_ {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,
    pub face_flags: FT_Long,
    pub style_flags: FT_Long,
    pub num_glyphs: FT_Long,
    pub family_name: *mut FT_String,
    pub style_name: *mut FT_String,
    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *mut FT_Bitmap_Size,
    pub num_charmaps: FT_Int,
    pub charmaps: *mut FT_CharMap,
    pub generic: FT_Generic,
    pub bbox: FT_BBox,
    pub units_per_EM: FT_UShort,
    pub ascender: FT_Short,
    pub descender: FT_Short,
    pub height: FT_Short,
    pub max_advance_width: FT_Short,
    pub max_advance_height: FT_Short,
    pub underline_position: FT_Short,
    pub underline_thickness: FT_Short,
    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,
    pub driver: FT_Driver,
    pub memory: FT_Memory,
    pub stream: FT_Stream,
    pub sizes_list: FT_ListRec,
    pub autohint: FT_Generic,
    pub extensions: *mut libc::c_void,
    pub internal: FT_Face_Internal,
}
pub type FT_Face_Internal = *mut FT_Face_InternalRec_;
pub type FT_CharMap = *mut FT_CharMapRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_CharMapRec_ {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}
pub type FT_Encoding = FT_Encoding_;
pub type FT_Encoding_ = libc::c_uint;
pub const FT_ENCODING_APPLE_ROMAN: FT_Encoding_ = 1634889070;
pub const FT_ENCODING_OLD_LATIN_2: FT_Encoding_ = 1818326066;
pub const FT_ENCODING_ADOBE_LATIN_1: FT_Encoding_ = 1818326065;
pub const FT_ENCODING_ADOBE_CUSTOM: FT_Encoding_ = 1094992451;
pub const FT_ENCODING_ADOBE_EXPERT: FT_Encoding_ = 1094992453;
pub const FT_ENCODING_ADOBE_STANDARD: FT_Encoding_ = 1094995778;
pub const FT_ENCODING_MS_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_MS_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_MS_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_MS_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_MS_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_PRC: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_UNICODE: FT_Encoding_ = 1970170211;
pub const FT_ENCODING_MS_SYMBOL: FT_Encoding_ = 1937337698;
pub const FT_ENCODING_NONE: FT_Encoding_ = 0;
pub type FT_Face = *mut FT_FaceRec_;
pub type FT_Size = *mut FT_SizeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_SizeRec_ {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}
pub type FT_Size_Internal = *mut FT_Size_InternalRec_;
pub type FT_Size_Metrics = FT_Size_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Size_Metrics_ {
    pub x_ppem: FT_UShort,
    pub y_ppem: FT_UShort,
    pub x_scale: FT_Fixed,
    pub y_scale: FT_Fixed,
    pub ascender: FT_Pos,
    pub descender: FT_Pos,
    pub height: FT_Pos,
    pub max_advance: FT_Pos,
}
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_GlyphSlotRec_ {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub glyph_index: FT_UInt,
    pub generic: FT_Generic,
    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,
    pub format: FT_Glyph_Format,
    pub bitmap: FT_Bitmap,
    pub bitmap_left: FT_Int,
    pub bitmap_top: FT_Int,
    pub outline: FT_Outline,
    pub num_subglyphs: FT_UInt,
    pub subglyphs: FT_SubGlyph,
    pub control_data: *mut libc::c_void,
    pub control_len: libc::c_long,
    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,
    pub other: *mut libc::c_void,
    pub internal: FT_Slot_Internal,
}
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec_;
pub type FT_SubGlyph = *mut FT_SubGlyphRec_;
pub type FcPattern = _FcPattern;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _PangoFcFont {
    pub parent_instance: PangoFont,
    pub font_pattern: *mut FcPattern,
    pub fontmap: *mut PangoFontMap,
    pub priv_0: gpointer,
    pub matrix: PangoMatrix,
    pub description: *mut PangoFontDescription,
    pub metrics_by_lang: *mut GSList,
    #[bitfield(name = "is_hinted", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "is_transformed", ty = "guint", bits = "1..=1")]
    pub is_hinted_is_transformed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type PangoFcFont = _PangoFcFont;
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh0 = (*xb).buf;
        *fresh0 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh1 = (*xb).buf;
        *fresh1 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh2 = (*xb).eptr;
    *fresh2 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh3 = (*xb).ptr;
    *fresh3 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn agxbfree(mut xb: *mut agxbuf) {
    if (*xb).dyna != 0 {
        free((*xb).buf as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn agxbmore(mut xb: *mut agxbuf, mut ssz: size_t) {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut nsize: size_t = 0 as libc::c_int as size_t;
    let mut nbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    size = ((*xb).eptr).offset_from((*xb).buf) as libc::c_long as size_t;
    nsize = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    if size.wrapping_add(ssz) > nsize {
        nsize = size.wrapping_add(ssz);
    }
    cnt = ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
    if (*xb).dyna != 0 {
        nbuf = gv_recalloc(
            (*xb).buf as *mut libc::c_void,
            size,
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        nbuf = gv_calloc(nsize, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as *mut libc::c_char;
        memcpy(nbuf as *mut libc::c_void, (*xb).buf as *const libc::c_void, cnt);
        (*xb).dyna = 1 as libc::c_int;
    }
    let ref mut fresh4 = (*xb).buf;
    *fresh4 = nbuf;
    let ref mut fresh5 = (*xb).ptr;
    *fresh5 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh6 = (*xb).eptr;
    *fresh6 = ((*xb).buf).offset(nsize as isize);
}
#[inline]
unsafe extern "C" fn agxbput_n(
    mut xb: *mut agxbuf,
    mut s: *const libc::c_char,
    mut ssz: size_t,
) -> size_t {
    if ((*xb).ptr).offset(ssz as isize) > (*xb).eptr {
        agxbmore(xb, ssz);
    }
    memcpy((*xb).ptr as *mut libc::c_void, s as *const libc::c_void, ssz);
    let ref mut fresh7 = (*xb).ptr;
    *fresh7 = (*fresh7).offset(ssz as isize);
    return ssz;
}
#[inline]
unsafe extern "C" fn agxbput(mut xb: *mut agxbuf, mut s: *const libc::c_char) -> size_t {
    let mut ssz: size_t = strlen(s);
    return agxbput_n(xb, s, ssz);
}
#[inline]
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh8 = (*xb).ptr;
    let fresh9 = *fresh8;
    *fresh8 = (*fresh8).offset(1);
    *fresh9 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn gv_realloc(
    mut ptr: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, new_size);
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    if new_size > old_size {
        memset(
            (p as *mut libc::c_char).offset(old_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            new_size.wrapping_sub(old_size),
        );
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_recalloc(
    mut ptr: *mut libc::c_void,
    mut old_nmemb: size_t,
    mut new_nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size > 0 as libc::c_int as libc::c_ulong
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size))
        as libc::c_int as libc::c_long != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(ptr, old_nmemb.wrapping_mul(size), new_nmemb.wrapping_mul(size));
}
#[inline]
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
#[inline]
unsafe extern "C" fn gv_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = (*xb).buf;
    return (*xb).ptr;
}
unsafe extern "C" fn pango_free_layout(mut layout: *mut libc::c_void) {
    g_object_unref(layout);
}
unsafe extern "C" fn pango_psfontResolve(
    mut pa: *mut PostscriptAlias,
) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    strcpy(buf.as_mut_ptr(), (*pa).family);
    strcat(buf.as_mut_ptr(), b",\0" as *const u8 as *const libc::c_char);
    if !((*pa).weight).is_null() {
        strcat(buf.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        strcat(buf.as_mut_ptr(), (*pa).weight);
    }
    if !((*pa).stretch).is_null() {
        strcat(buf.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        strcat(buf.as_mut_ptr(), (*pa).stretch);
    }
    if !((*pa).style).is_null() {
        strcat(buf.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        strcat(buf.as_mut_ptr(), (*pa).style);
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn agxbput_int(
    mut buffer: *mut libc::c_void,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut len: size_t = agxbput(buffer as *mut agxbuf, s);
    if len <= 2147483647 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"len <= INT_MAX\0" as *const u8 as *const libc::c_char,
            b"gvtextlayout_pango.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int agxbput_int(void *, const char *)\0"))
                .as_ptr(),
        );
    }
    return len as libc::c_int;
}
unsafe extern "C" fn pango_textlayout(
    mut span: *mut textspan_t,
    mut fontpath: *mut *mut libc::c_char,
) -> bool {
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    static mut fontmap: *mut PangoFontMap = 0 as *const PangoFontMap
        as *mut PangoFontMap;
    static mut context: *mut PangoContext = 0 as *const PangoContext
        as *mut PangoContext;
    static mut desc: *mut PangoFontDescription = 0 as *const PangoFontDescription
        as *mut PangoFontDescription;
    static mut fontname: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut fontsize: libc::c_double = 0.;
    static mut gv_fmap: *mut gv_font_map = 0 as *const gv_font_map as *mut gv_font_map;
    let mut fnt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psfnt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut layout: *mut PangoLayout = 0 as *mut PangoLayout;
    let mut logical_rect: PangoRectangle = PangoRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut options: *mut cairo_font_options_t = 0 as *mut cairo_font_options_t;
    let mut font: *mut PangoFont = 0 as *mut PangoFont;
    let mut attrs: *mut PangoAttrList = 0 as *mut PangoAttrList;
    let mut error: *mut GError = 0 as *mut GError;
    let mut flags: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut textlayout_scale: libc::c_double = 0.;
    let mut pA: *mut PostscriptAlias = 0 as *mut PostscriptAlias;
    if context.is_null() {
        fontmap = pango_cairo_font_map_new();
        gv_fmap = get_font_mapping(fontmap);
        context = pango_font_map_create_context(fontmap);
        options = cairo_font_options_create();
        cairo_font_options_set_antialias(options, CAIRO_ANTIALIAS_GRAY);
        cairo_font_options_set_hint_style(options, CAIRO_HINT_STYLE_FULL);
        cairo_font_options_set_hint_metrics(options, CAIRO_HINT_METRICS_ON);
        cairo_font_options_set_subpixel_order(options, CAIRO_SUBPIXEL_ORDER_BGR);
        pango_cairo_context_set_font_options(context, options);
        pango_cairo_context_set_resolution(context, 96.0f64);
        cairo_font_options_destroy(options);
        g_object_unref(fontmap as gpointer);
    }
    if fontname.is_null() || strcmp(fontname, (*(*span).font).name) != 0 as libc::c_int
        || fontsize != (*(*span).font).size
    {
        if ((2147483647 as libc::c_int / 1024 as libc::c_int) as libc::c_double)
            < (*(*span).font).size
        {
            return 0 as libc::c_int != 0;
        }
        free(fontname as *mut libc::c_void);
        fontname = gv_strdup((*(*span).font).name);
        fontsize = (*(*span).font).size;
        pango_font_description_free(desc);
        pA = (*(*span).font).postscript_alias;
        if !pA.is_null() {
            fnt = (*gv_fmap.offset((*pA).xfig_code as isize)).gv_font;
            psfnt = fnt;
            if psfnt.is_null() {
                fnt = pango_psfontResolve(pA);
                psfnt = fnt;
            }
        } else {
            fnt = fontname;
        }
        desc = pango_font_description_from_string(fnt);
        pango_font_description_set_size(
            desc,
            (fontsize * 1024 as libc::c_int as libc::c_double) as gint,
        );
        if !fontpath.is_null()
            && {
                font = pango_font_map_load_font(fontmap, context, desc);
                !font.is_null()
            }
        {
            let mut fontclass: *const libc::c_char = 0 as *const libc::c_char;
            fontclass = g_type_name(
                (*((*(font as *mut GTypeInstance)).g_class as *mut GObjectClass
                    as *mut GTypeClass))
                    .g_type,
            );
            buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            if !psfnt.is_null() {
                strcat(
                    buf.as_mut_ptr(),
                    b"(ps:pango  \0" as *const u8 as *const libc::c_char,
                );
                strcat(buf.as_mut_ptr(), psfnt);
                strcat(buf.as_mut_ptr(), b") \0" as *const u8 as *const libc::c_char);
            }
            strcat(buf.as_mut_ptr(), b"(\0" as *const u8 as *const libc::c_char);
            strcat(buf.as_mut_ptr(), fontclass);
            strcat(buf.as_mut_ptr(), b") \0" as *const u8 as *const libc::c_char);
            if strcmp(
                fontclass,
                b"PangoCairoFcFont\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut face: FT_Face = 0 as *mut FT_FaceRec_;
                let mut fcfont: *mut PangoFcFont = 0 as *mut PangoFcFont;
                let mut stream: FT_Stream = 0 as *mut FT_StreamRec_;
                let mut streamdesc: FT_StreamDesc = FT_StreamDesc_ { value: 0 };
                fcfont = g_type_check_instance_cast(
                    font as *mut GTypeInstance,
                    pango_fc_font_get_type(),
                ) as *mut libc::c_void as *mut PangoFcFont;
                face = pango_fc_font_lock_face(fcfont);
                if !face.is_null() {
                    strcat(
                        buf.as_mut_ptr(),
                        b"\"\0" as *const u8 as *const libc::c_char,
                    );
                    strcat(buf.as_mut_ptr(), (*face).family_name);
                    strcat(
                        buf.as_mut_ptr(),
                        b", \0" as *const u8 as *const libc::c_char,
                    );
                    strcat(buf.as_mut_ptr(), (*face).style_name);
                    strcat(
                        buf.as_mut_ptr(),
                        b"\" \0" as *const u8 as *const libc::c_char,
                    );
                    stream = (*face).stream;
                    if !stream.is_null() {
                        streamdesc = (*stream).pathname;
                        if !(streamdesc.pointer).is_null() {
                            strcat(
                                buf.as_mut_ptr(),
                                streamdesc.pointer as *mut libc::c_char,
                            );
                        } else {
                            strcat(
                                buf.as_mut_ptr(),
                                b"*no pathname available*\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    } else {
                        strcat(
                            buf.as_mut_ptr(),
                            b"*no stream available*\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                pango_fc_font_unlock_face(fcfont);
            } else {
                let mut tdesc: *mut PangoFontDescription = 0
                    as *mut PangoFontDescription;
                let mut tfont: *mut libc::c_char = 0 as *mut libc::c_char;
                tdesc = pango_font_describe(font);
                tfont = pango_font_description_to_string(tdesc);
                strcat(buf.as_mut_ptr(), b"\"\0" as *const u8 as *const libc::c_char);
                strcat(buf.as_mut_ptr(), tfont);
                strcat(buf.as_mut_ptr(), b"\" \0" as *const u8 as *const libc::c_char);
                g_free(tfont as gpointer);
            }
            *fontpath = buf.as_mut_ptr();
        }
    }
    if !((*span).font).is_null()
        && {
            flags = (*(*span).font).flags() as libc::c_int;
            flags != 0
        }
    {
        let mut buf_0: [libc::c_char; 8192] = [0; 8192];
        let mut xb: agxbuf = agxbuf {
            buf: 0 as *mut libc::c_char,
            ptr: 0 as *mut libc::c_char,
            eptr: 0 as *mut libc::c_char,
            dyna: 0,
        };
        agxbinit(&mut xb, 8192 as libc::c_int as libc::c_uint, buf_0.as_mut_ptr());
        agxbput(&mut xb, b"<span\0" as *const u8 as *const libc::c_char);
        if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            agxbput(&mut xb, b" weight=\"bold\"\0" as *const u8 as *const libc::c_char);
        }
        if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            agxbput(&mut xb, b" style=\"italic\"\0" as *const u8 as *const libc::c_char);
        }
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            agxbput(
                &mut xb,
                b" underline=\"single\"\0" as *const u8 as *const libc::c_char,
            );
        }
        if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
            agxbput(
                &mut xb,
                b" strikethrough=\"true\"\0" as *const u8 as *const libc::c_char,
            );
        }
        agxbput(&mut xb, b">\0" as *const u8 as *const libc::c_char);
        if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            agxbput(&mut xb, b"<sup>\0" as *const u8 as *const libc::c_char);
        }
        if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            agxbput(&mut xb, b"<sub>\0" as *const u8 as *const libc::c_char);
        }
        let xml_flags: xml_flags_t = {
            let mut init = xml_flags_t {
                raw_dash_nbsp_utf8: [0; 1],
                c2rust_padding: [0; 3],
            };
            init.set_raw(1 as libc::c_int as libc::c_uint);
            init.set_dash(1 as libc::c_int as libc::c_uint);
            init.set_nbsp(1 as libc::c_int as libc::c_uint);
            init.set_utf8(0);
            init
        };
        xml_escape(
            (*span).str_0,
            xml_flags,
            Some(
                agxbput_int
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
            &mut xb as *mut agxbuf as *mut libc::c_void,
        );
        if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            agxbput(&mut xb, b"</sub>\0" as *const u8 as *const libc::c_char);
        }
        if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            agxbput(&mut xb, b"</sup>\0" as *const u8 as *const libc::c_char);
        }
        agxbput(&mut xb, b"</span>\0" as *const u8 as *const libc::c_char);
        if pango_parse_markup(
            agxbuse(&mut xb),
            -(1 as libc::c_int),
            0 as libc::c_int as gunichar,
            &mut attrs,
            &mut text,
            0 as *mut gunichar,
            &mut error,
        ) == 0
        {
            fprintf(
                stderr,
                b"Error - pango_parse_markup: %s\n\0" as *const u8
                    as *const libc::c_char,
                (*error).message,
            );
            text = (*span).str_0;
            attrs = 0 as *mut PangoAttrList;
        }
        agxbfree(&mut xb);
    } else {
        text = (*span).str_0;
        attrs = 0 as *mut PangoAttrList;
    }
    layout = pango_layout_new(context);
    let ref mut fresh11 = (*span).layout;
    *fresh11 = layout as *mut libc::c_void;
    let ref mut fresh12 = (*span).free_layout;
    *fresh12 = Some(pango_free_layout as unsafe extern "C" fn(*mut libc::c_void) -> ());
    pango_layout_set_text(layout, text, -(1 as libc::c_int));
    pango_layout_set_font_description(layout, desc);
    if !attrs.is_null() {
        pango_layout_set_attributes(layout, attrs);
    }
    pango_layout_get_extents(layout, 0 as *mut PangoRectangle, &mut logical_rect);
    if logical_rect.width == 0 as libc::c_int {
        logical_rect.height = 0 as libc::c_int;
    }
    textlayout_scale = 72 as libc::c_int as libc::c_double
        / (96.0f64 * 1024 as libc::c_int as libc::c_double);
    (*span)
        .size
        .x = (logical_rect.width as libc::c_double * textlayout_scale
        + 1 as libc::c_int as libc::c_double) as libc::c_int as libc::c_double;
    (*span)
        .size
        .y = (logical_rect.height as libc::c_double * textlayout_scale
        + 1 as libc::c_int as libc::c_double) as libc::c_int as libc::c_double;
    (*span)
        .size
        .y = ((*(*span).font).size * 1.1f64 + 0.5f64) as libc::c_int as libc::c_double;
    (*span)
        .yoffset_layout = pango_layout_get_baseline(layout) as libc::c_double
        * textlayout_scale;
    (*span).yoffset_centerline = 0.2f64 * (*(*span).font).size;
    return logical_rect.width != 0 as libc::c_int
        || strcmp(text, b"\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int;
}
static mut pango_textlayout_engine: gvtextlayout_engine_t = unsafe {
    {
        let mut init = gvtextlayout_engine_s {
            textlayout: Some(
                pango_textlayout
                    as unsafe extern "C" fn(
                        *mut textspan_t,
                        *mut *mut libc::c_char,
                    ) -> bool,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gvtextlayout_pango_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: 0 as libc::c_int,
                type_0: b"textlayout\0" as *const u8 as *const libc::c_char,
                quality: 10 as libc::c_int,
                engine: &pango_textlayout_engine as *const gvtextlayout_engine_t
                    as *mut gvtextlayout_engine_t as *mut libc::c_void,
                features: 0 as *const libc::c_void as *mut libc::c_void,
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
