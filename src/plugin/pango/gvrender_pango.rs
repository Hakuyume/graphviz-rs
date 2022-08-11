#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type GVC_s;
    pub type _PangoLayout;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _cairo_pattern;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn get_gradient_points(
        A: *mut pointf,
        G: *mut pointf,
        n: libc::c_int,
        angle: libc::c_double,
        flags: libc::c_int,
    );
    fn gvwrite(job: *mut GVJ_t, s: *const libc::c_char, len: size_t) -> size_t;
    fn pango_cairo_show_layout(cr: *mut cairo_t, layout: *mut PangoLayout);
    fn cairo_pattern_add_color_stop_rgba(
        pattern: *mut cairo_pattern_t,
        offset: libc::c_double,
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
        alpha: libc::c_double,
    );
    fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    fn cairo_pattern_create_radial(
        cx0: libc::c_double,
        cy0: libc::c_double,
        radius0: libc::c_double,
        cx1: libc::c_double,
        cy1: libc::c_double,
        radius1: libc::c_double,
    ) -> *mut cairo_pattern_t;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_save(cr: *mut cairo_t);
    fn cairo_restore(cr: *mut cairo_t);
    fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
    fn cairo_set_source_rgba(
        cr: *mut cairo_t,
        red: libc::c_double,
        green: libc::c_double,
        blue: libc::c_double,
        alpha: libc::c_double,
    );
    fn cairo_set_line_width(cr: *mut cairo_t, width: libc::c_double);
    fn cairo_set_dash(
        cr: *mut cairo_t,
        dashes: *const libc::c_double,
        num_dashes: libc::c_int,
        offset: libc::c_double,
    );
    fn cairo_translate(cr: *mut cairo_t, tx: libc::c_double, ty: libc::c_double);
    fn cairo_scale(cr: *mut cairo_t, sx: libc::c_double, sy: libc::c_double);
    fn cairo_rotate(cr: *mut cairo_t, angle: libc::c_double);
    fn cairo_set_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    fn cairo_user_to_device(
        cr: *mut cairo_t,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
    );
    fn cairo_move_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);
    fn cairo_line_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);
    fn cairo_curve_to(
        cr: *mut cairo_t,
        x1: libc::c_double,
        y1: libc::c_double,
        x2: libc::c_double,
        y2: libc::c_double,
        x3: libc::c_double,
        y3: libc::c_double,
    );
    fn cairo_arc(
        cr: *mut cairo_t,
        xc: libc::c_double,
        yc: libc::c_double,
        radius: libc::c_double,
        angle1: libc::c_double,
        angle2: libc::c_double,
    );
    fn cairo_rectangle(
        cr: *mut cairo_t,
        x: libc::c_double,
        y: libc::c_double,
        width: libc::c_double,
        height: libc::c_double,
    );
    fn cairo_close_path(cr: *mut cairo_t);
    fn cairo_stroke(cr: *mut cairo_t);
    fn cairo_fill_preserve(cr: *mut cairo_t);
    fn cairo_show_page(cr: *mut cairo_t);
    fn cairo_clip(cr: *mut cairo_t);
    fn cairo_tag_begin(
        cr: *mut cairo_t,
        tag_name: *const libc::c_char,
        attributes: *const libc::c_char,
    );
    fn cairo_tag_end(cr: *mut cairo_t, tag_name: *const libc::c_char);
    fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t);
    fn cairo_get_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    fn cairo_status_to_string(status: cairo_status_t) -> *const libc::c_char;
    fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    fn cairo_surface_finish(surface: *mut cairo_surface_t);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    fn cairo_surface_write_to_png_stream(
        surface: *mut cairo_surface_t,
        write_func: cairo_write_func_t,
        closure: *mut libc::c_void,
    ) -> cairo_status_t;
    fn cairo_image_surface_create(
        format: cairo_format_t,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) -> *mut libc::c_uchar;
    fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> libc::c_int;
    fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> libc::c_int;
    fn cairo_pattern_create_linear(
        x0: libc::c_double,
        y0: libc::c_double,
        x1: libc::c_double,
        y1: libc::c_double,
    ) -> *mut cairo_pattern_t;
    fn cairo_ps_surface_create_for_stream(
        write_func: cairo_write_func_t,
        closure: *mut libc::c_void,
        width_in_points: libc::c_double,
        height_in_points: libc::c_double,
    ) -> *mut cairo_surface_t;
    fn cairo_ps_surface_set_eps(surface: *mut cairo_surface_t, eps: cairo_bool_t);
    fn cairo_pdf_surface_create_for_stream(
        write_func: cairo_write_func_t,
        closure: *mut libc::c_void,
        width_in_points: libc::c_double,
        height_in_points: libc::c_double,
    ) -> *mut cairo_surface_t;
    fn cairo_svg_surface_create_for_stream(
        write_func: cairo_write_func_t,
        closure: *mut libc::c_void,
        width_in_points: libc::c_double,
        height_in_points: libc::c_double,
    ) -> *mut cairo_surface_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvrender_engine_s {
    pub begin_job: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_job: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_graph: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_graph: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_layer: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub end_layer: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_page: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_page: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_cluster: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_cluster: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_nodes: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_nodes: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edges: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edges: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_node: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_node: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_edge: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub end_edge: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_anchor: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub end_anchor: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub begin_label: Option::<unsafe extern "C" fn(*mut GVJ_t, label_type) -> ()>,
    pub end_label: Option::<unsafe extern "C" fn(*mut GVJ_t) -> ()>,
    pub textspan: Option::<
        unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
    >,
    pub resolve_color: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut gvcolor_t) -> ()>,
    pub ellipse: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
    >,
    pub polygon: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int, libc::c_int) -> (),
    >,
    pub beziercurve: Option::<
        unsafe extern "C" fn(
            *mut GVJ_t,
            *mut pointf,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub polyline: Option::<
        unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
    >,
    pub comment: Option::<unsafe extern "C" fn(*mut GVJ_t, *mut libc::c_char) -> ()>,
    pub library_shape: Option::<
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
pub type label_type = libc::c_uint;
pub const LABEL_HTML: label_type = 1;
pub const LABEL_PLAIN: label_type = 0;
pub type obj_state_t = obj_state_s;
#[derive(Copy, Clone, BitfieldStruct)]
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
pub struct gvplugin_installed_t {
    pub id: libc::c_int,
    pub type_0: *const libc::c_char,
    pub quality: libc::c_int,
    pub engine: *mut libc::c_void,
    pub features: *mut libc::c_void,
}
pub type PangoLayout = _PangoLayout;
pub type cairo_bool_t = libc::c_int;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_matrix {
    pub xx: libc::c_double,
    pub yx: libc::c_double,
    pub xy: libc::c_double,
    pub yy: libc::c_double,
    pub x0: libc::c_double,
    pub y0: libc::c_double,
}
pub type cairo_matrix_t = _cairo_matrix;
pub type cairo_pattern_t = _cairo_pattern;
pub type _cairo_status = libc::c_uint;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 44;
pub const CAIRO_STATUS_DWRITE_ERROR: _cairo_status = 43;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub type cairo_status_t = _cairo_status;
pub type _cairo_format = libc::c_int;
pub const CAIRO_FORMAT_RGBA128F: _cairo_format = 7;
pub const CAIRO_FORMAT_RGB96F: _cairo_format = 6;
pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;
pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
pub const CAIRO_FORMAT_A1: _cairo_format = 3;
pub const CAIRO_FORMAT_A8: _cairo_format = 2;
pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
pub type cairo_format_t = _cairo_format;
pub type cairo_write_func_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_uchar,
        libc::c_uint,
    ) -> cairo_status_t,
>;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const FORMAT_EPS: C2RustUnnamed_4 = 5;
pub const FORMAT_SVG: C2RustUnnamed_4 = 4;
pub const FORMAT_PDF: C2RustUnnamed_4 = 3;
pub const FORMAT_PS: C2RustUnnamed_4 = 2;
pub const FORMAT_PNG: C2RustUnnamed_4 = 1;
pub const FORMAT_CAIRO: C2RustUnnamed_4 = 0;
static mut dashed: [libc::c_double; 1] = [6.0f64];
static mut dashed_len: libc::c_int = 0;
static mut dotted: [libc::c_double; 2] = [2.0f64, 6.0f64];
static mut dotted_len: libc::c_int = 0;
unsafe extern "C" fn cairogen_set_color(
    mut cr: *mut cairo_t,
    mut color: *mut gvcolor_t,
) {
    cairo_set_source_rgba(
        cr,
        (*color).u.RGBA[0 as libc::c_int as usize],
        (*color).u.RGBA[1 as libc::c_int as usize],
        (*color).u.RGBA[2 as libc::c_int as usize],
        (*color).u.RGBA[3 as libc::c_int as usize],
    );
}
unsafe extern "C" fn cairogen_add_color_stop_rgba(
    mut pat: *mut cairo_pattern_t,
    mut stop: libc::c_double,
    mut color: *mut gvcolor_t,
) {
    cairo_pattern_add_color_stop_rgba(
        pat,
        stop,
        (*color).u.RGBA[0 as libc::c_int as usize],
        (*color).u.RGBA[1 as libc::c_int as usize],
        (*color).u.RGBA[2 as libc::c_int as usize],
        (*color).u.RGBA[3 as libc::c_int as usize],
    );
}
unsafe extern "C" fn writer(
    mut closure: *mut libc::c_void,
    mut data: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> cairo_status_t {
    if length as libc::c_ulong
        == gvwrite(closure as *mut GVJ_t, data as *const libc::c_char, length as size_t)
    {
        return CAIRO_STATUS_SUCCESS;
    }
    return CAIRO_STATUS_WRITE_ERROR;
}
unsafe extern "C" fn cairogen_begin_job(mut job: *mut GVJ_t) {
    if (*job).external_context as libc::c_int != 0 && !((*job).context).is_null() {
        cairo_save((*job).context as *mut cairo_t);
    }
}
unsafe extern "C" fn cairogen_end_job(mut job: *mut GVJ_t) {
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    if (*job).external_context {
        cairo_restore(cr);
    } else {
        cairo_destroy(cr);
        let ref mut fresh0 = (*job).context;
        *fresh0 = 0 as *mut libc::c_void;
    };
}
unsafe extern "C" fn cairogen_begin_page(mut job: *mut GVJ_t) {
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if cr.is_null() {
        match (*job).render.id {
            2 | 5 => {
                surface = cairo_ps_surface_create_for_stream(
                    Some(
                        writer
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *const libc::c_uchar,
                                libc::c_uint,
                            ) -> cairo_status_t,
                    ),
                    job as *mut libc::c_void,
                    (*job).width as libc::c_double,
                    (*job).height as libc::c_double,
                );
                if (*job).render.id == FORMAT_EPS as libc::c_int {
                    cairo_ps_surface_set_eps(
                        surface,
                        (0 as libc::c_int == 0) as libc::c_int,
                    );
                }
            }
            3 => {
                surface = cairo_pdf_surface_create_for_stream(
                    Some(
                        writer
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *const libc::c_uchar,
                                libc::c_uint,
                            ) -> cairo_status_t,
                    ),
                    job as *mut libc::c_void,
                    (*job).width as libc::c_double,
                    (*job).height as libc::c_double,
                );
            }
            4 => {
                surface = cairo_svg_surface_create_for_stream(
                    Some(
                        writer
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *const libc::c_uchar,
                                libc::c_uint,
                            ) -> cairo_status_t,
                    ),
                    job as *mut libc::c_void,
                    (*job).width as libc::c_double,
                    (*job).height as libc::c_double,
                );
            }
            0 | 1 | _ => {
                if (*job).width >= 32767 as libc::c_int as libc::c_uint
                    || (*job).height >= 32767 as libc::c_int as libc::c_uint
                {
                    let mut scale: libc::c_double = fmin(
                        32767 as libc::c_int as libc::c_double
                            / (*job).width as libc::c_double,
                        32767 as libc::c_int as libc::c_double
                            / (*job).height as libc::c_double,
                    );
                    let ref mut fresh1 = (*job).width;
                    *fresh1 = (*fresh1 as libc::c_double * scale) as libc::c_uint;
                    let ref mut fresh2 = (*job).height;
                    *fresh2 = (*fresh2 as libc::c_double * scale) as libc::c_uint;
                    (*job).scale.x *= scale;
                    (*job).scale.y *= scale;
                    fprintf(
                        stderr,
                        b"%s: graph is too large for cairo-renderer bitmaps. Scaling by %g to fit\n\0"
                            as *const u8 as *const libc::c_char,
                        (*(*job).common).cmdname,
                        scale,
                    );
                }
                surface = cairo_image_surface_create(
                    CAIRO_FORMAT_ARGB32,
                    (*job).width as libc::c_int,
                    (*job).height as libc::c_int,
                );
                if (*(*job).common).verbose != 0 {
                    fprintf(
                        stderr,
                        b"%s: allocating a %dK cairo image surface (%d x %d pixels)\n\0"
                            as *const u8 as *const libc::c_char,
                        (*(*job).common).cmdname,
                        if ((*job).width)
                            .wrapping_mul((*job).height)
                            .wrapping_mul(4 as libc::c_int as libc::c_uint)
                            as libc::c_double / 1024.0f64
                            >= 0 as libc::c_int as libc::c_double
                        {
                            (((*job).width)
                                .wrapping_mul((*job).height)
                                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                                as libc::c_double / 1024.0f64 + 0.5f64) as libc::c_int
                        } else {
                            (((*job).width)
                                .wrapping_mul((*job).height)
                                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                                as libc::c_double / 1024.0f64 - 0.5f64) as libc::c_int
                        },
                        (*job).width,
                        (*job).height,
                    );
                }
            }
        }
        status = cairo_surface_status(surface);
        if status as libc::c_uint != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s: failure to create cairo surface: %s\n\0" as *const u8
                    as *const libc::c_char,
                (*(*job).common).cmdname,
                cairo_status_to_string(status),
            );
            cairo_surface_destroy(surface);
            return;
        }
        cr = cairo_create(surface);
        cairo_surface_destroy(surface);
        let ref mut fresh3 = (*job).context;
        *fresh3 = cr as *mut libc::c_void;
    }
    cairo_scale(cr, (*job).scale.x, (*job).scale.y);
    cairo_rotate(
        cr,
        -(*job).rotation as libc::c_double * 3.14159265358979323846f64 / 180.0f64,
    );
    cairo_translate(cr, (*job).translation.x, -(*job).translation.y);
    cairo_rectangle(
        cr,
        (*job).clip.LL.x,
        -(*job).clip.LL.y,
        (*job).clip.UR.x - (*job).clip.LL.x,
        -((*job).clip.UR.y - (*job).clip.LL.y),
    );
    cairo_clip(cr);
}
unsafe extern "C" fn cairogen_end_page(mut job: *mut GVJ_t) {
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut surface: *mut cairo_surface_t = 0 as *mut cairo_surface_t;
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    match (*job).render.id {
        1 => {
            surface = cairo_get_target(cr);
            cairo_surface_write_to_png_stream(
                surface,
                Some(
                    writer
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_uchar,
                            libc::c_uint,
                        ) -> cairo_status_t,
                ),
                job as *mut libc::c_void,
            );
        }
        2 | 3 | 4 => {
            cairo_show_page(cr);
            surface = cairo_surface_reference(cairo_get_target(cr));
            cairo_surface_finish(surface);
            status = cairo_surface_status(surface);
            cairo_surface_destroy(surface);
            if status as libc::c_uint
                != CAIRO_STATUS_SUCCESS as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"cairo: %s\n\0" as *const u8 as *const libc::c_char,
                    cairo_status_to_string(status),
                );
            }
        }
        0 | _ => {
            surface = cairo_get_target(cr);
            cairo_image_surface_get_width(surface) == 0 as libc::c_int
                || cairo_image_surface_get_height(surface) == 0 as libc::c_int;
            let ref mut fresh4 = (*job).imagedata;
            *fresh4 = cairo_image_surface_get_data(surface) as *mut libc::c_char;
        }
    };
}
unsafe extern "C" fn cairogen_begin_anchor(
    mut job: *mut GVJ_t,
    mut url: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut p0x: libc::c_double = 0.;
    let mut p0y: libc::c_double = 0.;
    let mut p1x: libc::c_double = 0.;
    let mut p1y: libc::c_double = 0.;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_len: size_t = 0;
    if !url.is_null() && !((*obj).url_map_p).is_null() {
        p0x = (*((*obj).url_map_p).offset(0 as libc::c_int as isize)).x;
        p0y = -(*((*obj).url_map_p).offset(0 as libc::c_int as isize)).y;
        cairo_user_to_device(cr, &mut p0x, &mut p0y);
        p1x = (*((*obj).url_map_p).offset(1 as libc::c_int as isize)).x;
        p1y = -(*((*obj).url_map_p).offset(1 as libc::c_int as isize)).y;
        cairo_user_to_device(cr, &mut p1x, &mut p1y);
        buf_len = (strlen(url)).wrapping_add(200 as libc::c_int as libc::c_ulong);
        buf = malloc(buf_len) as *mut libc::c_char;
        snprintf(
            buf,
            buf_len,
            b"rect=[%f %f %f %f] uri='%s'\0" as *const u8 as *const libc::c_char,
            p0x,
            p0y,
            p1x - p0x,
            p1y - p0y,
            url,
        );
        cairo_tag_begin(cr, b"Link\0" as *const u8 as *const libc::c_char, buf);
        cairo_tag_end(cr, b"Link\0" as *const u8 as *const libc::c_char);
        free(buf as *mut libc::c_void);
    }
}
unsafe extern "C" fn cairogen_textspan(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut span: *mut textspan_t,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut A: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    cairo_set_dash(cr, dashed.as_mut_ptr(), 0 as libc::c_int, 0.0f64);
    cairogen_set_color(cr, &mut (*obj).pencolor);
    match (*span).just as libc::c_int {
        114 => {
            p.x -= (*span).size.x;
        }
        108 => {
            p.x -= 0.0f64;
        }
        110 | _ => {
            p.x -= (*span).size.x / 2.0f64;
        }
    }
    p.y += (*span).yoffset_centerline + (*span).yoffset_layout;
    cairo_move_to(cr, p.x, -p.y);
    cairo_save(cr);
    cairo_scale(
        cr,
        72 as libc::c_int as libc::c_double / 96.0f64,
        72 as libc::c_int as libc::c_double / 96.0f64,
    );
    pango_cairo_show_layout(cr, (*span).layout as *mut PangoLayout);
    cairo_restore(cr);
    if !((*span).font).is_null()
        && (*(*span).font).flags() as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int != 0
    {
        A[0 as libc::c_int as usize].x = p.x;
        A[1 as libc::c_int as usize].x = p.x + (*span).size.x;
        A[0 as libc::c_int as usize].y = p.y;
        A[1 as libc::c_int as usize].y = A[0 as libc::c_int as usize].y;
        cairogen_polyline(job, A.as_mut_ptr(), 2 as libc::c_int);
    }
}
unsafe extern "C" fn cairogen_set_penstyle(mut job: *mut GVJ_t, mut cr: *mut cairo_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    if (*obj).pen as libc::c_uint == PEN_DASHED as libc::c_int as libc::c_uint {
        cairo_set_dash(cr, dashed.as_mut_ptr(), dashed_len, 0.0f64);
    } else if (*obj).pen as libc::c_uint == PEN_DOTTED as libc::c_int as libc::c_uint {
        cairo_set_dash(cr, dotted.as_mut_ptr(), dotted_len, 0.0f64);
    } else {
        cairo_set_dash(cr, dashed.as_mut_ptr(), 0 as libc::c_int, 0.0f64);
    }
    cairo_set_line_width(cr, (*obj).penwidth);
}
unsafe extern "C" fn cairo_gradient_fill(
    mut cr: *mut cairo_t,
    mut obj: *mut obj_state_t,
    mut filled: libc::c_int,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    let mut pat: *mut cairo_pattern_t = 0 as *mut cairo_pattern_t;
    let mut angle: libc::c_double = (*obj).gradient_angle as libc::c_double
        * 3.14159265358979323846f64 / 180 as libc::c_int as libc::c_double;
    let mut r1: libc::c_float = 0.;
    let mut r2: libc::c_float = 0.;
    let mut G: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut c1: pointf = pointf { x: 0., y: 0. };
    if filled == 2 as libc::c_int {
        get_gradient_points(A, G.as_mut_ptr(), n, angle, 0 as libc::c_int);
        pat = cairo_pattern_create_linear(
            G[0 as libc::c_int as usize].x,
            G[0 as libc::c_int as usize].y,
            G[1 as libc::c_int as usize].x,
            G[1 as libc::c_int as usize].y,
        );
    } else {
        get_gradient_points(
            A,
            G.as_mut_ptr(),
            n,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int,
        );
        r1 = G[1 as libc::c_int as usize].x as libc::c_float;
        r2 = G[1 as libc::c_int as usize].y as libc::c_float;
        if angle == 0 as libc::c_int as libc::c_double {
            c1.x = G[0 as libc::c_int as usize].x;
            c1.y = G[0 as libc::c_int as usize].y;
        } else {
            c1.x = G[0 as libc::c_int as usize].x + r1 as libc::c_double * cos(angle);
            c1.y = G[0 as libc::c_int as usize].y - r1 as libc::c_double * sin(angle);
        }
        pat = cairo_pattern_create_radial(
            c1.x,
            c1.y,
            r1 as libc::c_double,
            G[0 as libc::c_int as usize].x,
            G[0 as libc::c_int as usize].y,
            r2 as libc::c_double,
        );
    }
    if (*obj).gradient_frac > 0 as libc::c_int as libc::c_float {
        cairogen_add_color_stop_rgba(
            pat,
            (*obj).gradient_frac as libc::c_double - 0.001f64,
            &mut (*obj).fillcolor,
        );
        cairogen_add_color_stop_rgba(
            pat,
            (*obj).gradient_frac as libc::c_double,
            &mut (*obj).stopcolor,
        );
    } else {
        cairogen_add_color_stop_rgba(
            pat,
            0 as libc::c_int as libc::c_double,
            &mut (*obj).fillcolor,
        );
        cairogen_add_color_stop_rgba(
            pat,
            1 as libc::c_int as libc::c_double,
            &mut (*obj).stopcolor,
        );
    }
    cairo_set_source(cr, pat);
    cairo_fill_preserve(cr);
    cairo_pattern_destroy(pat);
}
unsafe extern "C" fn cairogen_ellipse(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut matrix: cairo_matrix_t = cairo_matrix_t {
        xx: 0.,
        yx: 0.,
        xy: 0.,
        yy: 0.,
        x0: 0.,
        y0: 0.,
    };
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    cairogen_set_penstyle(job, cr);
    cairo_get_matrix(cr, &mut matrix);
    rx = (*A.offset(1 as libc::c_int as isize)).x
        - (*A.offset(0 as libc::c_int as isize)).x;
    ry = (*A.offset(1 as libc::c_int as isize)).y
        - (*A.offset(0 as libc::c_int as isize)).y;
    if rx < 0.01f64 {
        rx = 0.01f64;
    }
    if ry < 0.01f64 {
        ry = 0.01f64;
    }
    cairo_translate(
        cr,
        (*A.offset(0 as libc::c_int as isize)).x,
        -(*A.offset(0 as libc::c_int as isize)).y,
    );
    cairo_scale(cr, rx, ry);
    cairo_move_to(cr, 1.0f64, 0.0f64);
    cairo_arc(
        cr,
        0.0f64,
        0.0f64,
        1.0f64,
        0.0f64,
        2 as libc::c_int as libc::c_double * 3.14159265358979323846f64,
    );
    cairo_set_matrix(cr, &mut matrix);
    if filled == 2 as libc::c_int || filled == 3 as libc::c_int {
        cairo_gradient_fill(cr, obj, filled, A, 2 as libc::c_int);
    } else if filled != 0 {
        cairogen_set_color(cr, &mut (*obj).fillcolor);
        cairo_fill_preserve(cr);
    }
    cairogen_set_color(cr, &mut (*obj).pencolor);
    cairo_stroke(cr);
}
unsafe extern "C" fn cairogen_polygon(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut i: libc::c_int = 0;
    cairogen_set_penstyle(job, cr);
    cairo_move_to(
        cr,
        (*A.offset(0 as libc::c_int as isize)).x,
        -(*A.offset(0 as libc::c_int as isize)).y,
    );
    i = 1 as libc::c_int;
    while i < n {
        cairo_line_to(cr, (*A.offset(i as isize)).x, -(*A.offset(i as isize)).y);
        i += 1;
    }
    cairo_close_path(cr);
    if filled == 2 as libc::c_int || filled == 3 as libc::c_int {
        cairo_gradient_fill(cr, obj, filled, A, n);
    } else if filled != 0 {
        cairogen_set_color(cr, &mut (*obj).fillcolor);
        cairo_fill_preserve(cr);
    }
    cairogen_set_color(cr, &mut (*obj).pencolor);
    cairo_stroke(cr);
}
unsafe extern "C" fn cairogen_bezier(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
    mut arrow_at_start: libc::c_int,
    mut arrow_at_end: libc::c_int,
    mut filled: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut i: libc::c_int = 0;
    cairogen_set_penstyle(job, cr);
    cairo_move_to(
        cr,
        (*A.offset(0 as libc::c_int as isize)).x,
        -(*A.offset(0 as libc::c_int as isize)).y,
    );
    i = 1 as libc::c_int;
    while i < n {
        cairo_curve_to(
            cr,
            (*A.offset(i as isize)).x,
            -(*A.offset(i as isize)).y,
            (*A.offset((i + 1 as libc::c_int) as isize)).x,
            -(*A.offset((i + 1 as libc::c_int) as isize)).y,
            (*A.offset((i + 2 as libc::c_int) as isize)).x,
            -(*A.offset((i + 2 as libc::c_int) as isize)).y,
        );
        i += 3 as libc::c_int;
    }
    if filled == 2 as libc::c_int || filled == 3 as libc::c_int {
        cairo_gradient_fill(cr, obj, filled, A, n);
    } else if filled != 0 {
        cairogen_set_color(cr, &mut (*obj).fillcolor);
        cairo_fill_preserve(cr);
    }
    cairogen_set_color(cr, &mut (*obj).pencolor);
    cairo_stroke(cr);
}
unsafe extern "C" fn cairogen_polyline(
    mut job: *mut GVJ_t,
    mut A: *mut pointf,
    mut n: libc::c_int,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut cr: *mut cairo_t = (*job).context as *mut cairo_t;
    let mut i: libc::c_int = 0;
    cairogen_set_penstyle(job, cr);
    cairo_move_to(
        cr,
        (*A.offset(0 as libc::c_int as isize)).x,
        -(*A.offset(0 as libc::c_int as isize)).y,
    );
    i = 1 as libc::c_int;
    while i < n {
        cairo_line_to(cr, (*A.offset(i as isize)).x, -(*A.offset(i as isize)).y);
        i += 1;
    }
    cairogen_set_color(cr, &mut (*obj).pencolor);
    cairo_stroke(cr);
}
static mut cairogen_engine: gvrender_engine_t = unsafe {
    {
        let mut init = gvrender_engine_s {
            begin_job: Some(
                cairogen_begin_job as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            end_job: Some(cairogen_end_job as unsafe extern "C" fn(*mut GVJ_t) -> ()),
            begin_graph: None,
            end_graph: None,
            begin_layer: None,
            end_layer: None,
            begin_page: Some(
                cairogen_begin_page as unsafe extern "C" fn(*mut GVJ_t) -> (),
            ),
            end_page: Some(cairogen_end_page as unsafe extern "C" fn(*mut GVJ_t) -> ()),
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
            begin_anchor: Some(
                cairogen_begin_anchor
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> (),
            ),
            end_anchor: None,
            begin_label: None,
            end_label: None,
            textspan: Some(
                cairogen_textspan
                    as unsafe extern "C" fn(*mut GVJ_t, pointf, *mut textspan_t) -> (),
            ),
            resolve_color: None,
            ellipse: Some(
                cairogen_ellipse
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            polygon: Some(
                cairogen_polygon
                    as unsafe extern "C" fn(
                        *mut GVJ_t,
                        *mut pointf,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            beziercurve: Some(
                cairogen_bezier
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
                cairogen_polyline
                    as unsafe extern "C" fn(*mut GVJ_t, *mut pointf, libc::c_int) -> (),
            ),
            comment: None,
            library_shape: None,
        };
        init
    }
};
static mut render_features_cairo: gvrender_features_t = {
    let mut init = gvrender_features_t {
        flags: (1 as libc::c_int) << 12 as libc::c_int
            | (1 as libc::c_int) << 13 as libc::c_int,
        default_pad: 4.0f64,
        knowncolors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        sz_knowncolors: 0 as libc::c_int,
        color_type: RGBA_DOUBLE,
    };
    init
};
static mut device_features_png: gvdevice_features_t = {
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
static mut device_features_ps: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 25 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 36.0f64, y: 36.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
static mut device_features_eps: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 25 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 36.0f64, y: 36.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
static mut device_features_pdf: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 25 as libc::c_int
            | (1 as libc::c_int) << 16 as libc::c_int
            | (1 as libc::c_int) << 17 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        default_margin: {
            let mut init = pointf_s { x: 36.0f64, y: 36.0f64 };
            init
        },
        default_pagesize: {
            let mut init = pointf_s { x: 0.0f64, y: 0.0f64 };
            init
        },
        default_dpi: {
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
static mut device_features_svg: gvdevice_features_t = {
    let mut init = gvdevice_features_t {
        flags: (1 as libc::c_int) << 25 as libc::c_int
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
            let mut init = pointf_s { x: 72.0f64, y: 72.0f64 };
            init
        },
    };
    init
};
#[no_mangle]
pub static mut gvrender_pango_types: [gvplugin_installed_t; 2] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_CAIRO as libc::c_int,
                type_0: b"cairo\0" as *const u8 as *const libc::c_char,
                quality: 10 as libc::c_int,
                engine: &cairogen_engine as *const gvrender_engine_t
                    as *mut gvrender_engine_t as *mut libc::c_void,
                features: &render_features_cairo as *const gvrender_features_t
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
pub static mut gvdevice_pango_types: [gvplugin_installed_t; 6] = unsafe {
    [
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PNG as libc::c_int,
                type_0: b"png:cairo\0" as *const u8 as *const libc::c_char,
                quality: 10 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_png as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PS as libc::c_int,
                type_0: b"ps:cairo\0" as *const u8 as *const libc::c_char,
                quality: -(10 as libc::c_int),
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_ps as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_EPS as libc::c_int,
                type_0: b"eps:cairo\0" as *const u8 as *const libc::c_char,
                quality: -(10 as libc::c_int),
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_eps as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_PDF as libc::c_int,
                type_0: b"pdf:cairo\0" as *const u8 as *const libc::c_char,
                quality: 10 as libc::c_int,
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_pdf as *const gvdevice_features_t
                    as *mut gvdevice_features_t as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = gvplugin_installed_t {
                id: FORMAT_SVG as libc::c_int,
                type_0: b"svg:cairo\0" as *const u8 as *const libc::c_char,
                quality: -(10 as libc::c_int),
                engine: 0 as *const libc::c_void as *mut libc::c_void,
                features: &device_features_svg as *const gvdevice_features_t
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
unsafe extern "C" fn run_static_initializers() {
    dashed_len = (::std::mem::size_of::<[libc::c_double; 1]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as libc::c_int;
    dotted_len = (::std::mem::size_of::<[libc::c_double; 2]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
