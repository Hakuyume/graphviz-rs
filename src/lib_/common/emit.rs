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
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gvloadimage_engine_s;
    pub type gvdevice_engine_s;
    pub type gvrender_engine_s;
    pub type gvlayout_engine_s;
    pub type gvtextlayout_engine_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn freePath(p: *mut Ppolyline_t);
    static mut Dtoset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agnode(g: *mut Agraph_t, name: *mut libc::c_char, createflag: libc::c_int) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agcontains(_: *mut Agraph_t, _: *mut libc::c_void) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agobjkind(_: *mut libc::c_void) -> libc::c_int;
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
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn textspan_size(gvc: *mut GVC_t, span: *mut textspan_t) -> pointf;
    fn free_textspan(tl: *mut textspan_t, _: libc::c_int);
    fn round_corners(
        job: *mut GVJ_t,
        AF: *mut pointf,
        sides: libc::c_int,
        style: libc::c_int,
        filled: libc::c_int,
    );
    fn shapeOf(_: *mut node_t) -> shape_kind;
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn setColorScheme(s: *mut libc::c_char);
    fn ptToLine2(l1: pointf, l2: pointf, p: pointf) -> libc::c_double;
    fn rect2poly(p: *mut pointf);
    fn Bezier(
        _: *mut pointf,
        _: libc::c_int,
        _: libc::c_double,
        _: *mut pointf,
        _: *mut pointf,
    ) -> pointf;
    fn strdup_and_subst_obj(str: *mut libc::c_char, obj: *mut libc::c_void) -> *mut libc::c_char;
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
    fn late_nnstring(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn late_string(
        _: *mut libc::c_void,
        _: *mut Agsym_t,
        _: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn mapBool(_: *const libc::c_char, _: bool) -> bool;
    fn mapbool(_: *const libc::c_char) -> bool;
    fn overlap_label(lp: *mut textlabel_t, b: boxf) -> bool;
    fn latin1ToUTF8(_: *mut libc::c_char) -> *mut libc::c_char;
    fn htmlEntityUTF8(_: *mut libc::c_char, g: *mut graph_t) -> *mut libc::c_char;
    fn dotneato_closest(spl: *mut splines, p: pointf) -> pointf;
    fn start_timer();
    fn elapsed_sec() -> libc::c_double;
    fn gvjobs_first(gvc: *mut GVC_t) -> *mut GVJ_t;
    fn gvjobs_next(gvc: *mut GVC_t) -> *mut GVJ_t;
    fn gvrender_ptf_A(
        job: *mut GVJ_t,
        af: *mut pointf,
        AF: *mut pointf,
        n: libc::c_int,
    ) -> *mut pointf;
    fn gvrender_begin_job(job: *mut GVJ_t) -> libc::c_int;
    fn gvrender_end_job(job: *mut GVJ_t);
    fn gvrender_select(job: *mut GVJ_t, lang: *const libc::c_char) -> libc::c_int;
    fn gvrender_begin_graph(job: *mut GVJ_t, g: *mut graph_t);
    fn gvrender_end_graph(job: *mut GVJ_t);
    fn gvrender_begin_page(job: *mut GVJ_t);
    fn gvrender_end_page(job: *mut GVJ_t);
    fn gvrender_begin_layer(job: *mut GVJ_t);
    fn gvrender_end_layer(job: *mut GVJ_t);
    fn gvrender_begin_cluster(job: *mut GVJ_t, sg: *mut graph_t);
    fn gvrender_end_cluster(job: *mut GVJ_t, g: *mut graph_t);
    fn gvrender_begin_nodes(job: *mut GVJ_t);
    fn gvrender_end_nodes(job: *mut GVJ_t);
    fn gvrender_begin_edges(job: *mut GVJ_t);
    fn gvrender_end_edges(job: *mut GVJ_t);
    fn gvrender_begin_node(job: *mut GVJ_t, n: *mut node_t);
    fn gvrender_end_node(job: *mut GVJ_t);
    fn gvrender_begin_edge(job: *mut GVJ_t, e: *mut edge_t);
    fn gvrender_end_edge(job: *mut GVJ_t);
    fn gvrender_begin_anchor(
        job: *mut GVJ_t,
        href: *mut libc::c_char,
        tooltip: *mut libc::c_char,
        target: *mut libc::c_char,
        id: *mut libc::c_char,
    );
    fn gvrender_end_anchor(job: *mut GVJ_t);
    fn gvrender_textspan(job: *mut GVJ_t, p: pointf, span: *mut textspan_t);
    fn gvrender_set_pencolor(job: *mut GVJ_t, name: *mut libc::c_char);
    fn gvrender_set_penwidth(job: *mut GVJ_t, penwidth: libc::c_double);
    fn gvrender_set_fillcolor(job: *mut GVJ_t, name: *mut libc::c_char);
    fn gvrender_set_gradient_vals(
        job: *mut GVJ_t,
        stopcolor: *mut libc::c_char,
        angle: libc::c_int,
        frac: libc::c_float,
    );
    fn gvrender_set_style(job: *mut GVJ_t, s: *mut *mut libc::c_char);
    fn gvrender_ellipse(job: *mut GVJ_t, AF: *mut pointf, n: libc::c_int, filled: libc::c_int);
    fn gvrender_polygon(job: *mut GVJ_t, af: *mut pointf, n: libc::c_int, filled: libc::c_int);
    fn gvrender_box(job: *mut GVJ_t, BF: boxf, filled: libc::c_int);
    fn gvrender_beziercurve(
        job: *mut GVJ_t,
        AF: *mut pointf,
        n: libc::c_int,
        arrow_at_start: libc::c_int,
        arrow_at_end: libc::c_int,
        filled: libc::c_int,
    );
    fn gvrender_polyline(job: *mut GVJ_t, AF: *mut pointf, n: libc::c_int);
    fn gvrender_comment(job: *mut GVJ_t, str: *mut libc::c_char);
    fn arrow_bb(p: pointf, u: pointf, arrowsize: libc::c_double) -> boxf;
    fn arrow_gen(
        job: *mut GVJ_t,
        emit_state: emit_state_t,
        p: pointf,
        u: pointf,
        arrowsize: libc::c_double,
        penwidth: libc::c_double,
        flag: libc::c_int,
    );
    fn ellipticWedge(
        ctr: pointf,
        major: libc::c_double,
        minor: libc::c_double,
        angle0: libc::c_double,
        angle1: libc::c_double,
    ) -> *mut Ppolyline_t;
    fn emit_label(job: *mut GVJ_t, emit_state: emit_state_t, _: *mut textlabel_t);
    fn taper(
        _: *mut bezier,
        radfunc_t: Option<
            unsafe extern "C" fn(libc::c_double, libc::c_double, libc::c_double) -> libc::c_double,
        >,
        initwid: libc::c_double,
        linejoin: libc::c_int,
        linecap: libc::c_int,
    ) -> *mut stroke_t;
    static mut G_gradientangle: *mut Agsym_t;
    static mut G_penwidth: *mut Agsym_t;
    static mut G_peripheries: *mut Agsym_t;
    static mut G_deletedfillcolor: *mut Agsym_t;
    static mut G_deletedpencolor: *mut Agsym_t;
    static mut Y_invert: libc::c_int;
    static mut G_activepencolor: *mut Agsym_t;
    static mut G_activefillcolor: *mut Agsym_t;
    static mut G_visitedpencolor: *mut Agsym_t;
    static mut G_visitedfillcolor: *mut Agsym_t;
    static mut N_style: *mut Agsym_t;
    static mut N_fontname: *mut Agsym_t;
    static mut N_fontsize: *mut Agsym_t;
    static mut E_comment: *mut Agsym_t;
    static mut E_fillcolor: *mut Agsym_t;
    static mut E_activepencolor: *mut Agsym_t;
    static mut E_activefillcolor: *mut Agsym_t;
    static mut E_selectedpencolor: *mut Agsym_t;
    static mut E_selectedfillcolor: *mut Agsym_t;
    static mut E_visitedpencolor: *mut Agsym_t;
    static mut E_visitedfillcolor: *mut Agsym_t;
    static mut E_deletedpencolor: *mut Agsym_t;
    static mut E_deletedfillcolor: *mut Agsym_t;
    static mut E_dir: *mut Agsym_t;
    static mut E_color: *mut Agsym_t;
    static mut N_comment: *mut Agsym_t;
    static mut N_layer: *mut Agsym_t;
    static mut E_style: *mut Agsym_t;
    static mut E_decorate: *mut Agsym_t;
    static mut E_arrowsz: *mut Agsym_t;
    static mut E_layer: *mut Agsym_t;
    static mut E_penwidth: *mut Agsym_t;
    fn parseXDotF(_: *mut libc::c_char, opfns: *mut drawfunc_t, sz: libc::c_int) -> *mut xdot;
    static mut gvevent_key_binding: [gvevent_key_binding_t; 0];
    static mut gvevent_key_binding_size: libc::c_int;
    static mut gvdevice_callbacks: gvdevice_callbacks_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;
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
pub struct htmllabel_t {
    pub u: C2RustUnnamed_5,
    pub kind: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub tbl: *mut htmltbl_t,
    pub txt: *mut htmltxt_t,
    pub img: *mut htmlimg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlimg_t {
    pub box_0: boxf,
    pub src: *mut libc::c_char,
    pub scale: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmltxt_t {
    pub spans: *mut htextspan_t,
    pub nspans: libc::c_short,
    pub simple: libc::c_char,
    pub box_0: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htextspan_t {
    pub items: *mut textspan_t,
    pub nitems: libc::c_short,
    pub just: libc::c_char,
    pub size: libc::c_double,
    pub lfsize: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmltbl_t {
    pub data: htmldata_t,
    pub u: C2RustUnnamed_6,
    pub cb: libc::c_schar,
    pub heights: *mut libc::c_int,
    pub widths: *mut libc::c_int,
    pub rc: libc::c_int,
    pub cc: libc::c_int,
    pub font: *mut textfont_t,
    pub flags: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub n: C2RustUnnamed_8,
    pub p: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub prev: *mut htmltbl_t,
    pub rows: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub parent: *mut htmlcell_t,
    pub cells: *mut *mut htmlcell_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmlcell_t {
    pub data: htmldata_t,
    pub cspan: libc::c_ushort,
    pub rspan: libc::c_ushort,
    pub col: libc::c_ushort,
    pub row: libc::c_ushort,
    pub child: htmllabel_t,
    pub parent: *mut htmltbl_t,
    pub ruled: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htmldata_t {
    pub href: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub bgcolor: *mut libc::c_char,
    pub pencolor: *mut libc::c_char,
    pub gradientangle: libc::c_int,
    pub space: libc::c_schar,
    pub border: libc::c_uchar,
    pub pad: libc::c_uchar,
    pub sides: libc::c_uchar,
    pub flags: libc::c_ushort,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub style: libc::c_ushort,
    pub box_0: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union inside_t {
    pub a: C2RustUnnamed_10,
    pub s: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub n: *mut node_t,
    pub bp: *mut boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub u: C2RustUnnamed_11,
    pub valign: libc::c_char,
    pub set: bool,
    pub html: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub txt: C2RustUnnamed_12,
    pub html: *mut htmllabel_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
pub struct stroke_t {
    pub nvertices: libc::c_int,
    pub flags: libc::c_int,
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
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colorsegs_t {
    pub numc: libc::c_int,
    pub base: *mut libc::c_char,
    pub segs: *mut colorseg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colorseg_t {
    pub color: *mut libc::c_char,
    pub t: libc::c_float,
    pub hasFraction: bool,
}
pub type radfunc_t =
    Option<unsafe extern "C" fn(libc::c_double, libc::c_double, libc::c_double) -> libc::c_double>;
pub type segitem_t = segitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segitem_s {
    pub p: pointf,
    pub next: *mut segitem_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token_t {
    pub type_0: libc::c_int,
    pub start: *const libc::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot {
    pub cnt: libc::c_int,
    pub sz: libc::c_int,
    pub ops: *mut xdot_op,
    pub freefunc: freefunc_t,
    pub flags: libc::c_int,
}
pub type freefunc_t = Option<unsafe extern "C" fn(*mut xdot_op) -> ()>;
pub type xdot_op = _xdot_op;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xdot_op {
    pub kind: xdot_kind,
    pub u: C2RustUnnamed_13,
    pub drawfunc: drawfunc_t,
}
pub type drawfunc_t = Option<unsafe extern "C" fn(*mut xdot_op, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub ellipse: xdot_rect,
    pub polygon: xdot_polyline,
    pub polyline: xdot_polyline,
    pub bezier: xdot_polyline,
    pub text: xdot_text,
    pub image: xdot_image,
    pub color: *mut libc::c_char,
    pub grad_color: xdot_color,
    pub font: xdot_font,
    pub style: *mut libc::c_char,
    pub fontchar: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_font {
    pub size: libc::c_double,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_color {
    pub type_0: xdot_grad_type,
    pub u: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub clr: *mut libc::c_char,
    pub ling: xdot_linear_grad,
    pub ring: xdot_radial_grad,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_radial_grad {
    pub x0: libc::c_double,
    pub y0: libc::c_double,
    pub r0: libc::c_double,
    pub x1: libc::c_double,
    pub y1: libc::c_double,
    pub r1: libc::c_double,
    pub n_stops: libc::c_int,
    pub stops: *mut xdot_color_stop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_color_stop {
    pub frac: libc::c_float,
    pub color: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_linear_grad {
    pub x0: libc::c_double,
    pub y0: libc::c_double,
    pub x1: libc::c_double,
    pub y1: libc::c_double,
    pub n_stops: libc::c_int,
    pub stops: *mut xdot_color_stop,
}
pub type xdot_grad_type = libc::c_uint;
pub const xd_radial: xdot_grad_type = 2;
pub const xd_linear: xdot_grad_type = 1;
pub const xd_none: xdot_grad_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_image {
    pub pos: xdot_rect,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_rect {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub w: libc::c_double,
    pub h: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_text {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub align: xdot_align,
    pub width: libc::c_double,
    pub text: *mut libc::c_char,
}
pub type xdot_align = libc::c_uint;
pub const xd_right: xdot_align = 2;
pub const xd_center: xdot_align = 1;
pub const xd_left: xdot_align = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_polyline {
    pub cnt: libc::c_int,
    pub pts: *mut xdot_point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
pub type xdot_kind = libc::c_uint;
pub const xd_fontchar: xdot_kind = 15;
pub const xd_grad_pen_color: xdot_kind = 14;
pub const xd_grad_fill_color: xdot_kind = 13;
pub const xd_image: xdot_kind = 12;
pub const xd_style: xdot_kind = 11;
pub const xd_font: xdot_kind = 10;
pub const xd_pen_color: xdot_kind = 9;
pub const xd_fill_color: xdot_kind = 8;
pub const xd_text: xdot_kind = 7;
pub const xd_polyline: xdot_kind = 6;
pub const xd_unfilled_bezier: xdot_kind = 5;
pub const xd_filled_bezier: xdot_kind = 4;
pub const xd_unfilled_polygon: xdot_kind = 3;
pub const xd_filled_polygon: xdot_kind = 2;
pub const xd_unfilled_ellipse: xdot_kind = 1;
pub const xd_filled_ellipse: xdot_kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exdot_op {
    pub op: xdot_op,
    pub bb: boxf,
    pub span: *mut textspan_t,
}
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
unsafe extern "C" fn add_point(mut p: point, mut q: point) -> point {
    let mut r: point = point { x: 0, y: 0 };
    r.x = p.x + q.x;
    r.y = p.y + q.y;
    return r;
}
#[inline]
unsafe extern "C" fn exch_xy(mut p: point) -> point {
    let mut r: point = point { x: 0, y: 0 };
    r.x = p.y;
    r.y = p.x;
    return r;
}
#[inline]
unsafe extern "C" fn exch_xyf(mut p: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.y;
    r.y = p.x;
    return r;
}
#[inline]
unsafe extern "C" fn boxf_overlap(mut b0: boxf, mut b1: boxf) -> libc::c_int {
    return (b0.UR.x >= b1.LL.x && b1.UR.x >= b0.LL.x && b0.UR.y >= b1.LL.y && b1.UR.y >= b0.LL.y)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_calloc(mut nmemb: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong
        && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_realloc(
    mut ptr: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, new_size);
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
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
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size)) as libc::c_int
        as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(
        ptr,
        old_nmemb.wrapping_mul(size),
        new_nmemb.wrapping_mul(size),
    );
}
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
        nbuf = gv_calloc(
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            nbuf as *mut libc::c_void,
            (*xb).buf as *const libc::c_void,
            cnt,
        );
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
unsafe extern "C" fn agxbprint(
    mut xb: *mut agxbuf,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut size: size_t = 0;
    let mut result: libc::c_int = 0;
    ap = args.clone();
    let mut ap2: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap2 = ap.clone();
    rc = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap2.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    size = (rc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {
    } else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"int agxbprint(agxbuf *, const char *, ...)\0",
            ))
            .as_ptr(),
        );
    }
    if result > 0 as libc::c_int {
        let ref mut fresh7 = (*xb).ptr;
        *fresh7 = (*fresh7).offset(result as size_t as isize);
    }
    return result;
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
    memcpy(
        (*xb).ptr as *mut libc::c_void,
        s as *const libc::c_void,
        ssz,
    );
    let ref mut fresh8 = (*xb).ptr;
    *fresh8 = (*fresh8).offset(ssz as isize);
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
    let ref mut fresh9 = (*xb).ptr;
    let fresh10 = *fresh9;
    *fresh9 = (*fresh9).offset(1);
    *fresh10 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh11 = (*xb).ptr;
    *fresh11 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn agxbnext(mut xb: *mut agxbuf) -> *mut libc::c_char {
    return (*xb).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn init_xdot(mut g: *mut Agraph_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xd: *mut xdot = 0 as *mut xdot;
    p = agget(
        g as *mut libc::c_void,
        b"_background\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(!p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int != 0) {
        p = agget(
            g as *mut libc::c_void,
            b"_draw_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(!p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int != 0) {
            return 0 as *mut libc::c_void;
        }
    }
    xd = parseXDotF(
        p,
        0 as *mut drawfunc_t,
        ::std::mem::size_of::<exdot_op>() as libc::c_ulong as libc::c_int,
    );
    if xd.is_null() {
        agerr(
            AGWARN,
            b"Could not parse \"_background\" attribute in graph %s\n\0" as *const u8
                as *const libc::c_char,
            agnameof(g as *mut libc::c_void),
        );
        agerr(
            AGPREV,
            b"  \"%s\"\n\0" as *const u8 as *const libc::c_char,
            p,
        );
    }
    return xd as *mut libc::c_void;
}
static mut defaultlinestyle: [*mut libc::c_char; 3] = [
    b"solid\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"setlinewidth\01\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn push_obj_state(mut job: *mut GVJ_t) -> *mut obj_state_t {
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    let mut parent: *mut obj_state_t = 0 as *mut obj_state_t;
    obj = zmalloc(::std::mem::size_of::<obj_state_t>() as libc::c_ulong) as *mut obj_state_t;
    let ref mut fresh12 = (*obj).parent;
    *fresh12 = (*job).obj;
    parent = *fresh12;
    let ref mut fresh13 = (*job).obj;
    *fresh13 = obj;
    if !parent.is_null() {
        (*obj).pencolor = (*parent).pencolor;
        (*obj).fillcolor = (*parent).fillcolor;
        (*obj).pen = (*parent).pen;
        (*obj).fill = (*parent).fill;
        (*obj).penwidth = (*parent).penwidth;
        (*obj).gradient_angle = (*parent).gradient_angle;
        (*obj).stopcolor = (*parent).stopcolor;
    } else {
        (*obj).pen = PEN_SOLID;
        (*obj).fill = FILL_NONE;
        (*obj).penwidth = 1.0f64;
    }
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn pop_obj_state(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\0" as *const u8 as *const libc::c_char,
            b"emit.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void pop_obj_state(GVJ_t *)\0",
            ))
            .as_ptr(),
        );
    }
    free((*obj).id as *mut libc::c_void);
    free((*obj).url as *mut libc::c_void);
    free((*obj).labelurl as *mut libc::c_void);
    free((*obj).tailurl as *mut libc::c_void);
    free((*obj).headurl as *mut libc::c_void);
    free((*obj).tooltip as *mut libc::c_void);
    free((*obj).labeltooltip as *mut libc::c_void);
    free((*obj).tailtooltip as *mut libc::c_void);
    free((*obj).headtooltip as *mut libc::c_void);
    free((*obj).target as *mut libc::c_void);
    free((*obj).labeltarget as *mut libc::c_void);
    free((*obj).tailtarget as *mut libc::c_void);
    free((*obj).headtarget as *mut libc::c_void);
    free((*obj).url_map_p as *mut libc::c_void);
    free((*obj).url_bsplinemap_p as *mut libc::c_void);
    free((*obj).url_bsplinemap_n as *mut libc::c_void);
    let ref mut fresh14 = (*job).obj;
    *fresh14 = (*obj).parent;
    free(obj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn initMapData(
    mut job: *mut GVJ_t,
    mut lbl: *mut libc::c_char,
    mut url: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
    mut gobj: *mut libc::c_void,
) -> libc::c_int {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut flags: libc::c_int = (*job).flags;
    let mut assigned: libc::c_int = 0 as libc::c_int;
    if flags & (1 as libc::c_int) << 15 as libc::c_int != 0 && !lbl.is_null() {
        let ref mut fresh15 = (*obj).label;
        *fresh15 = lbl;
    }
    if flags & (1 as libc::c_int) << 16 as libc::c_int != 0 {
        let ref mut fresh16 = (*obj).id;
        *fresh16 = strdup_and_subst_obj(id, gobj);
        if !url.is_null() && *url.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            let ref mut fresh17 = (*obj).url;
            *fresh17 = strdup_and_subst_obj(url, gobj);
            assigned = 1 as libc::c_int;
        }
    }
    if flags & (1 as libc::c_int) << 22 as libc::c_int != 0 {
        if !tooltip.is_null() && *tooltip.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            let ref mut fresh18 = (*obj).tooltip;
            *fresh18 = strdup_and_subst_obj(tooltip, gobj);
            (*obj).set_explicit_tooltip(1 as libc::c_int as libc::c_uint);
            assigned = 1 as libc::c_int;
        } else if !((*obj).label).is_null() {
            let ref mut fresh19 = (*obj).tooltip;
            *fresh19 = strdup((*obj).label);
            assigned = 1 as libc::c_int;
        }
    }
    if flags & (1 as libc::c_int) << 23 as libc::c_int != 0
        && !target.is_null()
        && *target.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        let ref mut fresh20 = (*obj).target;
        *fresh20 = strdup_and_subst_obj(target, gobj);
        assigned = 1 as libc::c_int;
    }
    return assigned;
}
unsafe extern "C" fn layerPagePrefix(mut job: *mut GVJ_t, mut xb: *mut agxbuf) {
    if (*job).layerNum > 1 as libc::c_int
        && (*job).flags & (1 as libc::c_int) << 6 as libc::c_int != 0
    {
        agxbprint(
            xb,
            b"%s_\0" as *const u8 as *const libc::c_char,
            *((*(*job).gvc).layerIDs).offset((*job).layerNum as isize),
        );
    }
    if (*job).pagesArrayElem.x > 0 as libc::c_int || (*job).pagesArrayElem.y > 0 as libc::c_int {
        agxbprint(
            xb,
            b"page%d,%d_\0" as *const u8 as *const libc::c_char,
            (*job).pagesArrayElem.x,
            (*job).pagesArrayElem.y,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn getObjId(
    mut job: *mut GVJ_t,
    mut obj: *mut libc::c_void,
    mut xb: *mut agxbuf,
) -> *mut libc::c_char {
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut root: *mut graph_t = (*(*job).gvc).g;
    let mut gid: *mut libc::c_char =
        (*(*((*(root as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).id;
    let mut idnum: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut pfx: *mut libc::c_char = 0 as *mut libc::c_char;
    layerPagePrefix(job, xb);
    id = agget(
        obj,
        b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !id.is_null() && *id as libc::c_int != '\0' as i32 {
        agxbput(xb, id);
        return agxbuse(xb);
    }
    if obj != root as *mut libc::c_void && !gid.is_null() {
        agxbprint(xb, b"%s_\0" as *const u8 as *const libc::c_char, gid);
    }
    match agobjkind(obj) {
        0 => {
            idnum = ((*(obj as *mut Agobj_t)).tag).seq() as libc::c_long;
            if root == obj as *mut graph_t {
                pfx = b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else {
                pfx = b"clust\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
        }
        1 => {
            idnum = ((*(obj as *mut Agnode_t as *mut Agobj_t)).tag).seq() as libc::c_long;
            pfx = b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            idnum = ((*(obj as *mut Agedge_t as *mut Agobj_t)).tag).seq() as libc::c_long;
            pfx = b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {}
    }
    agxbprint(
        xb,
        b"%s%ld\0" as *const u8 as *const libc::c_char,
        pfx,
        idnum,
    );
    return agxbuse(xb);
}
unsafe extern "C" fn interpretCRNL(mut ins: *mut libc::c_char) -> *mut libc::c_char {
    let mut rets: *mut libc::c_char = ins;
    let mut outs: *mut libc::c_char = ins;
    let mut c: libc::c_char = 0;
    let mut backslash_seen: bool = 0 as libc::c_int != 0;
    loop {
        let fresh21 = ins;
        ins = ins.offset(1);
        c = *fresh21;
        if !(c != 0) {
            break;
        }
        if backslash_seen {
            match c as libc::c_int {
                110 | 108 => {
                    let fresh22 = outs;
                    outs = outs.offset(1);
                    *fresh22 = '\n' as i32 as libc::c_char;
                }
                114 => {
                    let fresh23 = outs;
                    outs = outs.offset(1);
                    *fresh23 = '\r' as i32 as libc::c_char;
                }
                _ => {
                    let fresh24 = outs;
                    outs = outs.offset(1);
                    *fresh24 = c;
                }
            }
            backslash_seen = 0 as libc::c_int != 0;
        } else if c as libc::c_int == '\\' as i32 {
            backslash_seen = 1 as libc::c_int != 0;
        } else {
            let fresh25 = outs;
            outs = outs.offset(1);
            *fresh25 = c;
        }
    }
    *outs = '\0' as i32 as libc::c_char;
    return rets;
}
unsafe extern "C" fn preprocessTooltip(
    mut s: *mut libc::c_char,
    mut gobj: *mut libc::c_void,
) -> *mut libc::c_char {
    let mut g: *mut Agraph_t = agroot(gobj);
    let mut charset: libc::c_int =
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).charset as libc::c_int;
    let mut news: *mut libc::c_char = 0 as *mut libc::c_char;
    match charset {
        1 => {
            news = latin1ToUTF8(s);
        }
        _ => {
            news = htmlEntityUTF8(s, g);
        }
    }
    return interpretCRNL(news);
}
unsafe extern "C" fn initObjMapData(
    mut job: *mut GVJ_t,
    mut lab: *mut textlabel_t,
    mut gobj: *mut libc::c_void,
) {
    let mut lbl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url: *mut libc::c_char = agget(
        gobj,
        b"href\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut tooltip: *mut libc::c_char = agget(
        gobj,
        b"tooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut target: *mut libc::c_char = agget(
        gobj,
        b"target\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    agxbinit(
        &mut xb,
        128 as libc::c_int as libc::c_uint,
        buf.as_mut_ptr(),
    );
    if !lab.is_null() {
        lbl = (*lab).text;
    } else {
        lbl = 0 as *mut libc::c_char;
    }
    if url.is_null() || *url == 0 {
        url = agget(
            gobj,
            b"URL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    id = getObjId(job, gobj, &mut xb);
    if !tooltip.is_null() {
        tooltip = preprocessTooltip(tooltip, gobj);
    }
    initMapData(job, lbl, url, tooltip, target, id, gobj);
    free(tooltip as *mut libc::c_void);
    agxbfree(&mut xb);
}
unsafe extern "C" fn map_point(mut job: *mut GVJ_t, mut pf: pointf) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut flags: libc::c_int = (*job).flags;
    let mut p: *mut pointf = 0 as *mut pointf;
    if flags & ((1 as libc::c_int) << 16 as libc::c_int | (1 as libc::c_int) << 22 as libc::c_int)
        != 0
    {
        if flags & (1 as libc::c_int) << 17 as libc::c_int != 0 {
            (*obj).url_map_shape = MAP_RECTANGLE;
            (*obj).url_map_n = 2 as libc::c_int;
        } else {
            (*obj).url_map_shape = MAP_POLYGON;
            (*obj).url_map_n = 4 as libc::c_int;
        }
        free((*obj).url_map_p as *mut libc::c_void);
        p = gcalloc(
            (*obj).url_map_n as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        let ref mut fresh26 = (*obj).url_map_p;
        *fresh26 = p;
        (*p.offset(0 as libc::c_int as isize)).x = pf.x - 3 as libc::c_int as libc::c_double;
        (*p.offset(0 as libc::c_int as isize)).y = pf.y - 3 as libc::c_int as libc::c_double;
        (*p.offset(1 as libc::c_int as isize)).x = pf.x + 3 as libc::c_int as libc::c_double;
        (*p.offset(1 as libc::c_int as isize)).y = pf.y + 3 as libc::c_int as libc::c_double;
        if flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
            gvrender_ptf_A(job, p, p, 2 as libc::c_int);
        }
        if flags & (1 as libc::c_int) << 17 as libc::c_int == 0 {
            rect2poly(p);
        }
    }
}
unsafe extern "C" fn checkClusterStyle(
    mut sg: *mut graph_t,
    mut flagp: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    let mut style: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pstyle: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut istyle: libc::c_int = 0 as libc::c_int;
    style = agget(
        sg as *mut libc::c_void,
        b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !style.is_null() && *style.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut qp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        pstyle = parse_style(style);
        pp = pstyle;
        loop {
            p = *pp;
            if p.is_null() {
                break;
            }
            if strcmp(p, b"filled\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                istyle |= (1 as libc::c_int) << 0 as libc::c_int;
                pp = pp.offset(1);
            } else if strcmp(p, b"radial\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                istyle |=
                    (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh27 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh27 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else if strcmp(p, b"striped\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                istyle |= (1 as libc::c_int) << 6 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh28 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh28 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else if strcmp(p, b"rounded\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                istyle |= (1 as libc::c_int) << 2 as libc::c_int;
                qp = pp;
                loop {
                    qp = qp.offset(1);
                    let ref mut fresh29 = *qp.offset(-(1 as libc::c_int as isize));
                    *fresh29 = *qp;
                    if (*qp).is_null() {
                        break;
                    }
                }
            } else {
                pp = pp.offset(1);
            }
        }
    }
    *flagp = istyle;
    return pstyle;
}
unsafe extern "C" fn freeSegs(mut segs: *mut colorsegs_t) {
    free((*segs).base as *mut libc::c_void);
    free((*segs).segs as *mut libc::c_void);
    free(segs as *mut libc::c_void);
}
unsafe extern "C" fn getSegLen(mut s: *mut libc::c_char) -> libc::c_double {
    let mut p: *mut libc::c_char = strchr(s, ';' as i32);
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: libc::c_double = 0.;
    if p.is_null() {
        return 0 as libc::c_int as libc::c_double;
    }
    let fresh30 = p;
    p = p.offset(1);
    *fresh30 = '\0' as i32 as libc::c_char;
    v = strtod(p, &mut endp);
    if endp != p {
        if v >= 0 as libc::c_int as libc::c_double {
            return v;
        }
    }
    return -(1 as libc::c_int) as libc::c_double;
}
unsafe extern "C" fn parseSegs(
    mut clrs: *mut libc::c_char,
    mut nseg: libc::c_int,
    mut psegs: *mut *mut colorsegs_t,
) -> libc::c_int {
    let mut segs: *mut colorsegs_t =
        zmalloc(::std::mem::size_of::<colorsegs_t>() as libc::c_ulong) as *mut colorsegs_t;
    let mut s: *mut colorseg_t = 0 as *mut colorseg_t;
    let mut colors: *mut libc::c_char = strdup(clrs);
    let mut color: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cnum: libc::c_int = 0 as libc::c_int;
    let mut v: libc::c_double = 0.;
    let mut left: libc::c_double = 1 as libc::c_int as libc::c_double;
    static mut doWarn: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut rval: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if nseg == 0 as libc::c_int {
        nseg = 1 as libc::c_int;
        p = colors;
        while *p != 0 {
            if *p as libc::c_int == ':' as i32 {
                nseg += 1;
            }
            p = p.offset(1);
        }
    }
    let ref mut fresh31 = (*segs).base;
    *fresh31 = colors;
    s = gcalloc(
        (nseg + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<colorseg_t>() as libc::c_ulong,
    ) as *mut colorseg_t;
    let ref mut fresh32 = (*segs).segs;
    *fresh32 = s;
    color = strtok(colors, b":\0" as *const u8 as *const libc::c_char);
    while !color.is_null() {
        v = getSegLen(color);
        if v >= 0 as libc::c_int as libc::c_double {
            let mut del: libc::c_double = v - left;
            if del > 0 as libc::c_int as libc::c_double {
                if doWarn != 0 && !(del < 1E-5f64 && del > -1E-5f64) {
                    agerr(
                        AGWARN,
                        b"Total size > 1 in \"%s\" color spec \0" as *const u8
                            as *const libc::c_char,
                        clrs,
                    );
                    doWarn = 0 as libc::c_int;
                    rval = 3 as libc::c_int;
                }
                v = left;
            }
            left -= v;
            if v > 0 as libc::c_int as libc::c_double {
                (*s.offset(cnum as isize)).hasFraction = 1 as libc::c_int != 0;
            }
            if *color != 0 {
                let ref mut fresh33 = (*s.offset(cnum as isize)).color;
                *fresh33 = color;
            }
            let fresh34 = cnum;
            cnum = cnum + 1;
            (*s.offset(fresh34 as isize)).t = v as libc::c_float;
        } else {
            if doWarn != 0 {
                agerr(
                    AGERR,
                    b"Illegal value in \"%s\" color attribute; float expected after ';'\n\0"
                        as *const u8 as *const libc::c_char,
                    clrs,
                );
                doWarn = 0 as libc::c_int;
                rval = 2 as libc::c_int;
            } else {
                rval = 1 as libc::c_int;
            }
            freeSegs(segs);
            return rval;
        }
        if left < 1E-5f64 && left > -1E-5f64 {
            left = 0 as libc::c_int as libc::c_double;
            break;
        } else {
            color = strtok(
                0 as *mut libc::c_char,
                b":\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if left > 0 as libc::c_int as libc::c_double {
        nseg = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < cnum {
            if (*s.offset(i as isize)).t == 0 as libc::c_int as libc::c_float {
                nseg += 1;
            }
            i += 1;
        }
        if nseg > 0 as libc::c_int {
            let mut delta: libc::c_double = left / nseg as libc::c_double;
            i = 0 as libc::c_int;
            while i < cnum {
                if (*s.offset(i as isize)).t == 0 as libc::c_int as libc::c_float {
                    (*s.offset(i as isize)).t = delta as libc::c_float;
                }
                i += 1;
            }
        } else {
            let ref mut fresh35 = (*s.offset((cnum - 1 as libc::c_int) as isize)).t;
            *fresh35 = (*fresh35 as libc::c_double + left) as libc::c_float;
        }
    }
    i = cnum - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*s.offset(i as isize)).t > 0 as libc::c_int as libc::c_float {
            break;
        }
        i -= 1;
    }
    let ref mut fresh36 = (*s.offset((i + 1 as libc::c_int) as isize)).color;
    *fresh36 = 0 as *mut libc::c_char;
    (*segs).numc = i + 1 as libc::c_int;
    *psegs = segs;
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn wedgedEllipse(
    mut job: *mut GVJ_t,
    mut pf: *mut pointf,
    mut clrs: *mut libc::c_char,
) -> libc::c_int {
    let mut segs: *mut colorsegs_t = 0 as *mut colorsegs_t;
    let mut s: *mut colorseg_t = 0 as *mut colorseg_t;
    let mut rv: libc::c_int = 0;
    let mut save_penwidth: libc::c_double = (*(*job).obj).penwidth;
    let mut ctr: pointf = pointf { x: 0., y: 0. };
    let mut semi: pointf = pointf { x: 0., y: 0. };
    let mut pp: *mut Ppolyline_t = 0 as *mut Ppolyline_t;
    let mut angle0: libc::c_double = 0.;
    let mut angle1: libc::c_double = 0.;
    rv = parseSegs(clrs, 0 as libc::c_int, &mut segs);
    if rv == 1 as libc::c_int || rv == 2 as libc::c_int {
        return rv;
    }
    ctr.x = ((*pf.offset(0 as libc::c_int as isize)).x + (*pf.offset(1 as libc::c_int as isize)).x)
        / 2.0f64;
    ctr.y = ((*pf.offset(0 as libc::c_int as isize)).y + (*pf.offset(1 as libc::c_int as isize)).y)
        / 2.0f64;
    semi.x = (*pf.offset(1 as libc::c_int as isize)).x - ctr.x;
    semi.y = (*pf.offset(1 as libc::c_int as isize)).y - ctr.y;
    if save_penwidth > 0.5f64 {
        gvrender_set_penwidth(job, 0.5f64);
    }
    angle0 = 0 as libc::c_int as libc::c_double;
    s = (*segs).segs;
    while !((*s).color).is_null() {
        if !((*s).t == 0 as libc::c_int as libc::c_float) {
            gvrender_set_fillcolor(
                job,
                (if !((*s).color).is_null() {
                    (*s).color as *const libc::c_char
                } else {
                    b"black\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
            );
            if ((*s.offset(1 as libc::c_int as isize)).color).is_null() {
                angle1 = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
            } else {
                angle1 = angle0
                    + 2 as libc::c_int as libc::c_double
                        * 3.14159265358979323846f64
                        * (*s).t as libc::c_double;
            }
            pp = ellipticWedge(ctr, semi.x, semi.y, angle0, angle1);
            gvrender_beziercurve(
                job,
                (*pp).ps,
                (*pp).pn,
                0 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            angle0 = angle1;
            freePath(pp);
        }
        s = s.offset(1);
    }
    if save_penwidth > 0.5f64 {
        gvrender_set_penwidth(job, save_penwidth);
    }
    freeSegs(segs);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn stripedBox(
    mut job: *mut GVJ_t,
    mut AF: *mut pointf,
    mut clrs: *mut libc::c_char,
    mut rotate: libc::c_int,
) -> libc::c_int {
    let mut segs: *mut colorsegs_t = 0 as *mut colorsegs_t;
    let mut s: *mut colorseg_t = 0 as *mut colorseg_t;
    let mut rv: libc::c_int = 0;
    let mut xdelta: libc::c_double = 0.;
    let mut pts: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut lastx: libc::c_double = 0.;
    let mut save_penwidth: libc::c_double = (*(*job).obj).penwidth;
    rv = parseSegs(clrs, 0 as libc::c_int, &mut segs);
    if rv == 1 as libc::c_int || rv == 2 as libc::c_int {
        return rv;
    }
    if rotate != 0 {
        pts[0 as libc::c_int as usize] = *AF.offset(2 as libc::c_int as isize);
        pts[1 as libc::c_int as usize] = *AF.offset(3 as libc::c_int as isize);
        pts[2 as libc::c_int as usize] = *AF.offset(0 as libc::c_int as isize);
        pts[3 as libc::c_int as usize] = *AF.offset(1 as libc::c_int as isize);
    } else {
        pts[0 as libc::c_int as usize] = *AF.offset(0 as libc::c_int as isize);
        pts[1 as libc::c_int as usize] = *AF.offset(1 as libc::c_int as isize);
        pts[2 as libc::c_int as usize] = *AF.offset(2 as libc::c_int as isize);
        pts[3 as libc::c_int as usize] = *AF.offset(3 as libc::c_int as isize);
    }
    lastx = pts[1 as libc::c_int as usize].x;
    xdelta = pts[1 as libc::c_int as usize].x - pts[0 as libc::c_int as usize].x;
    pts[2 as libc::c_int as usize].x = pts[0 as libc::c_int as usize].x;
    pts[1 as libc::c_int as usize].x = pts[2 as libc::c_int as usize].x;
    if save_penwidth > 0.5f64 {
        gvrender_set_penwidth(job, 0.5f64);
    }
    s = (*segs).segs;
    while !((*s).color).is_null() {
        if !((*s).t == 0 as libc::c_int as libc::c_float) {
            gvrender_set_fillcolor(
                job,
                (if !((*s).color).is_null() {
                    (*s).color as *const libc::c_char
                } else {
                    b"black\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
            );
            if ((*s.offset(1 as libc::c_int as isize)).color).is_null() {
                pts[2 as libc::c_int as usize].x = lastx;
                pts[1 as libc::c_int as usize].x = pts[2 as libc::c_int as usize].x;
            } else {
                pts[2 as libc::c_int as usize].x =
                    pts[0 as libc::c_int as usize].x + xdelta * (*s).t as libc::c_double;
                pts[1 as libc::c_int as usize].x = pts[2 as libc::c_int as usize].x;
            }
            gvrender_polygon(job, pts.as_mut_ptr(), 4 as libc::c_int, 1 as libc::c_int);
            pts[3 as libc::c_int as usize].x = pts[1 as libc::c_int as usize].x;
            pts[0 as libc::c_int as usize].x = pts[3 as libc::c_int as usize].x;
        }
        s = s.offset(1);
    }
    if save_penwidth > 0.5f64 {
        gvrender_set_penwidth(job, save_penwidth);
    }
    freeSegs(segs);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn emit_map_rect(mut job: *mut GVJ_t, mut b: boxf) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut flags: libc::c_int = (*job).flags;
    let mut p: *mut pointf = 0 as *mut pointf;
    if flags & ((1 as libc::c_int) << 16 as libc::c_int | (1 as libc::c_int) << 22 as libc::c_int)
        != 0
    {
        if flags & (1 as libc::c_int) << 17 as libc::c_int != 0 {
            (*obj).url_map_shape = MAP_RECTANGLE;
            (*obj).url_map_n = 2 as libc::c_int;
        } else {
            (*obj).url_map_shape = MAP_POLYGON;
            (*obj).url_map_n = 4 as libc::c_int;
        }
        free((*obj).url_map_p as *mut libc::c_void);
        p = gcalloc(
            (*obj).url_map_n as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        let ref mut fresh37 = (*obj).url_map_p;
        *fresh37 = p;
        *p.offset(0 as libc::c_int as isize) = b.LL;
        *p.offset(1 as libc::c_int as isize) = b.UR;
        if flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
            gvrender_ptf_A(job, p, p, 2 as libc::c_int);
        }
        if flags & (1 as libc::c_int) << 17 as libc::c_int == 0 {
            rect2poly(p);
        }
    }
}
unsafe extern "C" fn map_label(mut job: *mut GVJ_t, mut lab: *mut textlabel_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut flags: libc::c_int = (*job).flags;
    let mut p: *mut pointf = 0 as *mut pointf;
    if flags & ((1 as libc::c_int) << 16 as libc::c_int | (1 as libc::c_int) << 22 as libc::c_int)
        != 0
    {
        if flags & (1 as libc::c_int) << 17 as libc::c_int != 0 {
            (*obj).url_map_shape = MAP_RECTANGLE;
            (*obj).url_map_n = 2 as libc::c_int;
        } else {
            (*obj).url_map_shape = MAP_POLYGON;
            (*obj).url_map_n = 4 as libc::c_int;
        }
        free((*obj).url_map_p as *mut libc::c_void);
        p = gcalloc(
            (*obj).url_map_n as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        let ref mut fresh38 = (*obj).url_map_p;
        *fresh38 = p;
        (*p.offset(0 as libc::c_int as isize)).x = (*lab).pos.x - (*lab).dimen.x / 2.0f64;
        (*p.offset(0 as libc::c_int as isize)).y = (*lab).pos.y - (*lab).dimen.y / 2.0f64;
        (*p.offset(1 as libc::c_int as isize)).x = (*lab).pos.x + (*lab).dimen.x / 2.0f64;
        (*p.offset(1 as libc::c_int as isize)).y = (*lab).pos.y + (*lab).dimen.y / 2.0f64;
        if flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
            gvrender_ptf_A(job, p, p, 2 as libc::c_int);
        }
        if flags & (1 as libc::c_int) << 17 as libc::c_int == 0 {
            rect2poly(p);
        }
    }
}
unsafe extern "C" fn isRect(mut p: *mut polygon_t) -> bool {
    return (*p).sides == 4 as libc::c_int
        && (if (*p).orientation >= 0 as libc::c_int as libc::c_double {
            ((*p).orientation + 0.5f64) as libc::c_int
        } else {
            ((*p).orientation - 0.5f64) as libc::c_int
        }) % 90 as libc::c_int
            == 0 as libc::c_int
        && (*p).distortion == 0.0f64
        && (*p).skew == 0.0f64;
}
unsafe extern "C" fn isFilled(mut n: *mut node_t) -> bool {
    let mut style: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut r: bool = 0 as libc::c_int != 0;
    style = late_nnstring(
        n as *mut libc::c_void,
        N_style,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *style.offset(0 as libc::c_int as isize) != 0 {
        pp = parse_style(style);
        loop {
            p = *pp;
            if p.is_null() {
                break;
            }
            if strcmp(p, b"filled\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                r = 1 as libc::c_int != 0;
            }
            pp = pp.offset(1);
        }
    }
    return r;
}
unsafe extern "C" fn pEllipse(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut np: libc::c_int,
) -> *mut pointf {
    let mut theta: libc::c_double = 0.0f64;
    let mut deltheta: libc::c_double =
        2 as libc::c_int as libc::c_double * 3.14159265358979323846f64 / np as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut ps: *mut pointf = 0 as *mut pointf;
    ps = gcalloc(
        np as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    i = 0 as libc::c_int;
    while i < np {
        (*ps.offset(i as isize)).x = a * cos(theta);
        (*ps.offset(i as isize)).y = b * sin(theta);
        theta += deltheta;
        i += 1;
    }
    return ps;
}
unsafe extern "C" fn check_control_points(mut cp: *mut pointf) -> bool {
    let mut dis1: libc::c_double = ptToLine2(
        *cp.offset(0 as libc::c_int as isize),
        *cp.offset(3 as libc::c_int as isize),
        *cp.offset(1 as libc::c_int as isize),
    );
    let mut dis2: libc::c_double = ptToLine2(
        *cp.offset(0 as libc::c_int as isize),
        *cp.offset(3 as libc::c_int as isize),
        *cp.offset(2 as libc::c_int as isize),
    );
    return dis1 < 2.0f64 * 2.0f64 && dis2 < 2.0f64 * 2.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn update_bb_bz(mut bb: *mut boxf, mut cp: *mut pointf) {
    if (*cp.offset(0 as libc::c_int as isize)).x > (*bb).UR.x
        || (*cp.offset(0 as libc::c_int as isize)).x < (*bb).LL.x
        || (*cp.offset(0 as libc::c_int as isize)).y > (*bb).UR.y
        || (*cp.offset(0 as libc::c_int as isize)).y < (*bb).LL.y
        || (*cp.offset(1 as libc::c_int as isize)).x > (*bb).UR.x
        || (*cp.offset(1 as libc::c_int as isize)).x < (*bb).LL.x
        || (*cp.offset(1 as libc::c_int as isize)).y > (*bb).UR.y
        || (*cp.offset(1 as libc::c_int as isize)).y < (*bb).LL.y
        || (*cp.offset(2 as libc::c_int as isize)).x > (*bb).UR.x
        || (*cp.offset(2 as libc::c_int as isize)).x < (*bb).LL.x
        || (*cp.offset(2 as libc::c_int as isize)).y > (*bb).UR.y
        || (*cp.offset(2 as libc::c_int as isize)).y < (*bb).LL.y
        || (*cp.offset(3 as libc::c_int as isize)).x > (*bb).UR.x
        || (*cp.offset(3 as libc::c_int as isize)).x < (*bb).LL.x
        || (*cp.offset(3 as libc::c_int as isize)).y > (*bb).UR.y
        || (*cp.offset(3 as libc::c_int as isize)).y < (*bb).LL.y
    {
        if check_control_points(cp) {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                if (*cp.offset(i as isize)).x > (*bb).UR.x {
                    (*bb).UR.x = (*cp.offset(i as isize)).x;
                } else if (*cp.offset(i as isize)).x < (*bb).LL.x {
                    (*bb).LL.x = (*cp.offset(i as isize)).x;
                }
                if (*cp.offset(i as isize)).y > (*bb).UR.y {
                    (*bb).UR.y = (*cp.offset(i as isize)).y;
                } else if (*cp.offset(i as isize)).y < (*bb).LL.y {
                    (*bb).LL.y = (*cp.offset(i as isize)).y;
                }
                i += 1;
            }
        } else {
            let mut left: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
            let mut right: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
            Bezier(
                cp,
                3 as libc::c_int,
                0.5f64,
                left.as_mut_ptr(),
                right.as_mut_ptr(),
            );
            update_bb_bz(bb, left.as_mut_ptr());
            update_bb_bz(bb, right.as_mut_ptr());
        }
    }
}
unsafe extern "C" fn appendSeg(mut p: pointf, mut lp: *mut segitem_t) -> *mut segitem_t {
    let mut s: *mut segitem_t =
        gmalloc(::std::mem::size_of::<segitem_t>() as libc::c_ulong) as *mut segitem_t;
    let ref mut fresh39 = (*s).next;
    *fresh39 = 0 as *mut segitem_s;
    (*s).p = p;
    let ref mut fresh40 = (*lp).next;
    *fresh40 = s;
    return s;
}
unsafe extern "C" fn map_bspline_poly(
    mut pbs_p: *mut *mut pointf,
    mut pbs_n: *mut *mut libc::c_int,
    mut pbs_poly_n: *mut libc::c_int,
    mut n: libc::c_int,
    mut p1: *mut pointf,
    mut p2: *mut pointf,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut nump: libc::c_int = 0 as libc::c_int;
    let mut last: libc::c_int = 2 as libc::c_int * n - 1 as libc::c_int;
    while i < *pbs_poly_n {
        nump += *(*pbs_n).offset(i as isize);
        i += 1;
    }
    *pbs_poly_n += 1;
    *pbs_n = grealloc(
        *pbs_n as *mut libc::c_void,
        (*pbs_poly_n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *(*pbs_n).offset(i as isize) = 2 as libc::c_int * n;
    *pbs_p = grealloc(
        *pbs_p as *mut libc::c_void,
        ((nump + 2 as libc::c_int * n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
    ) as *mut pointf;
    i = 0 as libc::c_int;
    while i < n {
        *(*pbs_p).offset((nump + i) as isize) = *p1.offset(i as isize);
        *(*pbs_p).offset((nump + last - i) as isize) = *p2.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn approx_bezier(mut cp: *mut pointf, mut lp: *mut segitem_t) -> *mut segitem_t {
    let mut left: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut right: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    if check_control_points(cp) {
        if (*lp).next == 1 as libc::c_int as *mut segitem_t {
            let ref mut fresh41 = (*lp).next;
            *fresh41 = 0 as *mut segitem_s;
            (*lp).p = *cp.offset(0 as libc::c_int as isize);
        }
        lp = appendSeg(*cp.offset(3 as libc::c_int as isize), lp);
    } else {
        Bezier(
            cp,
            3 as libc::c_int,
            0.5f64,
            left.as_mut_ptr(),
            right.as_mut_ptr(),
        );
        lp = approx_bezier(left.as_mut_ptr(), lp);
        lp = approx_bezier(right.as_mut_ptr(), lp);
    }
    return lp;
}
unsafe extern "C" fn bisect(mut pp: pointf, mut cp: pointf, mut np: pointf) -> libc::c_double {
    let mut ang: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    theta = atan2(np.y - cp.y, np.x - cp.x);
    phi = atan2(pp.y - cp.y, pp.x - cp.x);
    ang = theta - phi;
    if ang > 0 as libc::c_int as libc::c_double {
        ang -= 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
    }
    return phi + ang / 2.0f64;
}
unsafe extern "C" fn mkSegPts(
    mut prv: *mut segitem_t,
    mut cur: *mut segitem_t,
    mut nxt: *mut segitem_t,
    mut p1: *mut pointf,
    mut p2: *mut pointf,
    mut w2: libc::c_double,
) {
    let mut cp: pointf = pointf { x: 0., y: 0. };
    let mut pp: pointf = pointf { x: 0., y: 0. };
    let mut np: pointf = pointf { x: 0., y: 0. };
    let mut theta: libc::c_double = 0.;
    let mut delx: libc::c_double = 0.;
    let mut dely: libc::c_double = 0.;
    let mut p: pointf = pointf { x: 0., y: 0. };
    cp = (*cur).p;
    if !prv.is_null() {
        pp = (*prv).p;
        if !nxt.is_null() {
            np = (*nxt).p;
        } else {
            np.x = 2 as libc::c_int as libc::c_double * cp.x - pp.x;
            np.y = 2 as libc::c_int as libc::c_double * cp.y - pp.y;
        }
    } else {
        np = (*nxt).p;
        pp.x = 2 as libc::c_int as libc::c_double * cp.x - np.x;
        pp.y = 2 as libc::c_int as libc::c_double * cp.y - np.y;
    }
    theta = bisect(pp, cp, np);
    delx = w2 * cos(theta);
    dely = w2 * sin(theta);
    p.x = cp.x + delx;
    p.y = cp.y + dely;
    *p1 = p;
    p.x = cp.x - delx;
    p.y = cp.y - dely;
    *p2 = p;
}
unsafe extern "C" fn map_output_bspline(
    mut pbs: *mut *mut pointf,
    mut pbs_n: *mut *mut libc::c_int,
    mut pbs_poly_n: *mut libc::c_int,
    mut bp: *mut bezier,
    mut w2: libc::c_double,
) {
    let mut segl: *mut segitem_t =
        gmalloc(::std::mem::size_of::<segitem_t>() as libc::c_ulong) as *mut segitem_t;
    let mut segp: *mut segitem_t = segl;
    let mut segprev: *mut segitem_t = 0 as *mut segitem_t;
    let mut segnext: *mut segitem_t = 0 as *mut segitem_t;
    let mut nc: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut pts: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut pt1: [pointf; 50] = [pointf { x: 0., y: 0. }; 50];
    let mut pt2: [pointf; 50] = [pointf { x: 0., y: 0. }; 50];
    let ref mut fresh42 = (*segl).next;
    *fresh42 = 1 as libc::c_int as *mut segitem_t;
    nc = ((*bp).size - 1 as libc::c_int) / 3 as libc::c_int;
    j = 0 as libc::c_int;
    while j < nc {
        k = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            pts[k as usize] = *((*bp).list).offset((3 as libc::c_int * j + k) as isize);
            k += 1;
        }
        segp = approx_bezier(pts.as_mut_ptr(), segp);
        j += 1;
    }
    segp = segl;
    segprev = 0 as *mut segitem_t;
    cnt = 0 as libc::c_int;
    while !segp.is_null() {
        segnext = (*segp).next;
        mkSegPts(
            segprev,
            segp,
            segnext,
            pt1.as_mut_ptr().offset(cnt as isize),
            pt2.as_mut_ptr().offset(cnt as isize),
            w2,
        );
        cnt += 1;
        if segnext.is_null() || cnt == 50 as libc::c_int {
            map_bspline_poly(
                pbs,
                pbs_n,
                pbs_poly_n,
                cnt,
                pt1.as_mut_ptr(),
                pt2.as_mut_ptr(),
            );
            pt1[0 as libc::c_int as usize] = pt1[(cnt - 1 as libc::c_int) as usize];
            pt2[0 as libc::c_int as usize] = pt2[(cnt - 1 as libc::c_int) as usize];
            cnt = 1 as libc::c_int;
        }
        segprev = segp;
        segp = segnext;
    }
    while !segl.is_null() {
        segp = (*segl).next;
        free(segl as *mut libc::c_void);
        segl = segp;
    }
}
unsafe extern "C" fn is_natural_number(mut sstr: *const libc::c_char) -> bool {
    let mut str: *const libc::c_char = sstr;
    while *str != 0 {
        let fresh43 = str;
        str = str.offset(1);
        if *(*__ctype_b_loc()).offset(*fresh43 as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn layer_index(
    mut gvc: *mut GVC_t,
    mut str: *mut libc::c_char,
    mut all: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if strcmp(str, b"all\0" as *const u8 as *const libc::c_char) == 0 {
        return all;
    }
    if is_natural_number(str) {
        return atoi(str);
    }
    if !((*gvc).layerIDs).is_null() {
        i = 1 as libc::c_int;
        while i <= (*gvc).numLayers {
            if strcmp(str, *((*gvc).layerIDs).offset(i as isize)) == 0 {
                return i;
            }
            i += 1;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn selectedLayer(
    mut gvc: *mut GVC_t,
    mut layerNum: libc::c_int,
    mut numLayers: libc::c_int,
    mut spec: *mut libc::c_char,
) -> bool {
    let mut n0: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut w0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_part_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut part_in_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut rval: bool = 0 as libc::c_int != 0;
    agxbinit(
        &mut xb,
        128 as libc::c_int as libc::c_uint,
        buf.as_mut_ptr(),
    );
    agxbput(&mut xb, spec);
    part_in_p = agxbuse(&mut xb);
    while !rval && {
        cur = strtok_r(part_in_p, (*gvc).layerListDelims, &mut buf_part_p);
        !cur.is_null()
    } {
        w0 = strtok_r(cur, (*gvc).layerDelims, &mut buf_p);
        w1 = w0;
        if !w0.is_null() {
            w1 = strtok_r(0 as *mut libc::c_char, (*gvc).layerDelims, &mut buf_p);
        }
        if !w0.is_null() && !w1.is_null() {
            n0 = layer_index(gvc, w0, 0 as libc::c_int);
            n1 = layer_index(gvc, w1, numLayers);
            if n0 >= 0 as libc::c_int || n1 >= 0 as libc::c_int {
                if n0 > n1 {
                    let mut t: libc::c_int = n0;
                    n0 = n1;
                    n1 = t;
                }
                rval = n0 <= layerNum && layerNum <= n1;
            }
        } else if !w0.is_null() || !w1.is_null() {
            n0 = layer_index(gvc, w0, layerNum);
            rval = n0 == layerNum;
        } else {
            rval = 0 as libc::c_int != 0;
        }
        part_in_p = 0 as *mut libc::c_char;
    }
    agxbfree(&mut xb);
    return rval;
}
unsafe extern "C" fn selectedlayer(mut job: *mut GVJ_t, mut spec: *mut libc::c_char) -> bool {
    return selectedLayer((*job).gvc, (*job).layerNum, (*job).numLayers, spec);
}
unsafe extern "C" fn parse_layerselect(
    mut gvc: *mut GVC_t,
    mut p: *mut libc::c_char,
) -> *mut libc::c_int {
    let mut laylist: *mut libc::c_int = gcalloc(
        ((*gvc).numLayers + 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*gvc).numLayers {
        if selectedLayer(gvc, i, (*gvc).numLayers, p) {
            cnt += 1;
            *laylist.offset(cnt as isize) = i;
        }
        i += 1;
    }
    if cnt != 0 {
        *laylist.offset(0 as libc::c_int as isize) = cnt;
        *laylist.offset((cnt + 1 as libc::c_int) as isize) = (*gvc).numLayers + 1 as libc::c_int;
    } else {
        agerr(
            AGWARN,
            b"The layerselect attribute \"%s\" does not match any layer specifed by the layers attribute - ignored.\n\0"
                as *const u8 as *const libc::c_char,
            p,
        );
        *laylist.offset(0 as libc::c_int as isize) = cnt;
        free(laylist as *mut libc::c_void);
        laylist = 0 as *mut libc::c_int;
    }
    return laylist;
}
unsafe extern "C" fn parse_layers(
    mut gvc: *mut GVC_t,
    mut g: *mut graph_t,
    mut p: *mut libc::c_char,
) -> libc::c_int {
    let mut ntok: libc::c_int = 0;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: libc::c_int = 0;
    let ref mut fresh44 = (*gvc).layerDelims;
    *fresh44 = agget(
        g as *mut libc::c_void,
        b"layersep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if ((*gvc).layerDelims).is_null() {
        let ref mut fresh45 = (*gvc).layerDelims;
        *fresh45 = b":\t \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    let ref mut fresh46 = (*gvc).layerListDelims;
    *fresh46 = agget(
        g as *mut libc::c_void,
        b"layerlistsep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if ((*gvc).layerListDelims).is_null() {
        let ref mut fresh47 = (*gvc).layerListDelims;
        *fresh47 = b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    tok = strpbrk((*gvc).layerDelims, (*gvc).layerListDelims);
    if !tok.is_null() {
        agerr(
            AGWARN,
            b"The character '%c' appears in both the layersep and layerlistsep attributes - layerlistsep ignored.\n\0"
                as *const u8 as *const libc::c_char,
            *tok as libc::c_int,
        );
        let ref mut fresh48 = (*gvc).layerListDelims;
        *fresh48 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    ntok = 0 as libc::c_int;
    sz = 0 as libc::c_int;
    let ref mut fresh49 = (*gvc).layers;
    *fresh49 = strdup(p);
    tok = strtok((*gvc).layers, (*gvc).layerDelims);
    while !tok.is_null() {
        ntok += 1;
        if ntok > sz {
            sz += 128 as libc::c_int;
            let ref mut fresh50 = (*gvc).layerIDs;
            *fresh50 = if !((*gvc).layerIDs).is_null() {
                grealloc(
                    (*gvc).layerIDs as *mut libc::c_void,
                    (sz as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
                ) as *mut *mut libc::c_char
            } else {
                gmalloc(
                    (sz as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
                ) as *mut *mut libc::c_char
            };
        }
        let ref mut fresh51 = *((*gvc).layerIDs).offset(ntok as isize);
        *fresh51 = tok;
        tok = strtok(0 as *mut libc::c_char, (*gvc).layerDelims);
    }
    if ntok != 0 {
        let ref mut fresh52 = (*gvc).layerIDs;
        *fresh52 = grealloc(
            (*gvc).layerIDs as *mut libc::c_void,
            ((ntok + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        let ref mut fresh53 = *((*gvc).layerIDs).offset(0 as libc::c_int as isize);
        *fresh53 = 0 as *mut libc::c_char;
        let ref mut fresh54 = *((*gvc).layerIDs).offset((ntok + 1 as libc::c_int) as isize);
        *fresh54 = 0 as *mut libc::c_char;
    }
    return ntok;
}
unsafe extern "C" fn chkOrder(mut g: *mut graph_t) -> libc::c_int {
    let mut p: *mut libc::c_char = agget(
        g as *mut libc::c_void,
        b"outputorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        if strcmp(p, b"nodesfirst\0" as *const u8 as *const libc::c_char) == 0 {
            return (1 as libc::c_int) << 0 as libc::c_int;
        }
        if strcmp(p, b"edgesfirst\0" as *const u8 as *const libc::c_char) == 0 {
            return (1 as libc::c_int) << 4 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_layering(mut gvc: *mut GVC_t, mut g: *mut graph_t) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    free((*gvc).layers as *mut libc::c_void);
    let ref mut fresh55 = (*gvc).layers;
    *fresh55 = 0 as *mut libc::c_char;
    free((*gvc).layerIDs as *mut libc::c_void);
    let ref mut fresh56 = (*gvc).layerIDs;
    *fresh56 = 0 as *mut *mut libc::c_char;
    free((*gvc).layerlist as *mut libc::c_void);
    let ref mut fresh57 = (*gvc).layerlist;
    *fresh57 = 0 as *mut libc::c_int;
    str = agget(
        g as *mut libc::c_void,
        b"layers\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() {
        (*gvc).numLayers = parse_layers(gvc, g, str);
        str = agget(
            g as *mut libc::c_void,
            b"layerselect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str as libc::c_int != 0 {
            let ref mut fresh58 = (*gvc).layerlist;
            *fresh58 = parse_layerselect(gvc, str);
        }
    } else {
        let ref mut fresh59 = (*gvc).layerIDs;
        *fresh59 = 0 as *mut *mut libc::c_char;
        (*gvc).numLayers = 1 as libc::c_int;
    };
}
unsafe extern "C" fn numPhysicalLayers(mut job: *mut GVJ_t) -> libc::c_int {
    if !((*(*job).gvc).layerlist).is_null() {
        return *((*(*job).gvc).layerlist).offset(0 as libc::c_int as isize);
    } else {
        return (*job).numLayers;
    };
}
unsafe extern "C" fn firstlayer(mut job: *mut GVJ_t, mut listp: *mut *mut libc::c_int) {
    (*job).numLayers = (*(*job).gvc).numLayers;
    if !((*(*job).gvc).layerlist).is_null() {
        let mut list: *mut libc::c_int = (*(*job).gvc).layerlist;
        let fresh60 = list;
        list = list.offset(1);
        let mut cnt: libc::c_int = *fresh60;
        if cnt > 1 as libc::c_int && (*job).flags & (1 as libc::c_int) << 6 as libc::c_int == 0 {
            agerr(
                AGWARN,
                b"layers not supported in %s output\n\0" as *const u8 as *const libc::c_char,
                (*job).output_langname,
            );
            *list.offset(1 as libc::c_int as isize) = (*job).numLayers + 1 as libc::c_int;
        }
        let fresh61 = list;
        list = list.offset(1);
        (*job).layerNum = *fresh61;
        *listp = list;
    } else {
        if (*job).numLayers > 1 as libc::c_int
            && (*job).flags & (1 as libc::c_int) << 6 as libc::c_int == 0
        {
            agerr(
                AGWARN,
                b"layers not supported in %s output\n\0" as *const u8 as *const libc::c_char,
                (*job).output_langname,
            );
            (*job).numLayers = 1 as libc::c_int;
        }
        (*job).layerNum = 1 as libc::c_int;
        *listp = 0 as *mut libc::c_int;
    };
}
unsafe extern "C" fn validlayer(mut job: *mut GVJ_t) -> bool {
    return (*job).layerNum <= (*job).numLayers;
}
unsafe extern "C" fn nextlayer(mut job: *mut GVJ_t, mut listp: *mut *mut libc::c_int) {
    let mut list: *mut libc::c_int = *listp;
    if !list.is_null() {
        let fresh62 = list;
        list = list.offset(1);
        (*job).layerNum = *fresh62;
        *listp = list;
    } else {
        let ref mut fresh63 = (*job).layerNum;
        *fresh63 += 1;
    };
}
unsafe extern "C" fn pagecode(mut job: *mut GVJ_t, mut c: libc::c_char) -> point {
    let mut rv: point = point { x: 0, y: 0 };
    rv.y = 0 as libc::c_int;
    rv.x = rv.y;
    match c as libc::c_int {
        84 => {
            (*job).pagesArrayFirst.y = (*job).pagesArraySize.y - 1 as libc::c_int;
            rv.y = -(1 as libc::c_int);
        }
        66 => {
            rv.y = 1 as libc::c_int;
        }
        76 => {
            rv.x = 1 as libc::c_int;
        }
        82 => {
            (*job).pagesArrayFirst.x = (*job).pagesArraySize.x - 1 as libc::c_int;
            rv.x = -(1 as libc::c_int);
        }
        _ => {}
    }
    return rv;
}
unsafe extern "C" fn init_job_pagination(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut gvc: *mut GVC_t = (*job).gvc;
    let mut pageSize: pointf = pointf { x: 0., y: 0. };
    let mut imageSize: pointf = pointf { x: 0., y: 0. };
    let mut margin: pointf = pointf { x: 0., y: 0. };
    let mut centering: pointf = {
        let mut init = pointf_s {
            x: 0.0f64,
            y: 0.0f64,
        };
        init
    };
    imageSize = (*job).view;
    if (*job).rotation != 0 {
        imageSize = exch_xyf(imageSize);
    }
    margin = (*job).margin;
    if (*gvc).graph_sets_pageSize as libc::c_int != 0
        && (*job).flags & (1 as libc::c_int) << 5 as libc::c_int != 0
    {
        pageSize.x = (*gvc).pageSize.x - 2 as libc::c_int as libc::c_double * margin.x;
        pageSize.y = (*gvc).pageSize.y - 2 as libc::c_int as libc::c_double * margin.y;
        if pageSize.x < 0.0001f64 {
            (*job).pagesArraySize.x = 1 as libc::c_int;
        } else {
            (*job).pagesArraySize.x = (imageSize.x / pageSize.x) as libc::c_int;
            if imageSize.x - (*job).pagesArraySize.x as libc::c_double * pageSize.x > 0.0001f64 {
                let ref mut fresh64 = (*job).pagesArraySize.x;
                *fresh64 += 1;
            }
        }
        if pageSize.y < 0.0001f64 {
            (*job).pagesArraySize.y = 1 as libc::c_int;
        } else {
            (*job).pagesArraySize.y = (imageSize.y / pageSize.y) as libc::c_int;
            if imageSize.y - (*job).pagesArraySize.y as libc::c_double * pageSize.y > 0.0001f64 {
                let ref mut fresh65 = (*job).pagesArraySize.y;
                *fresh65 += 1;
            }
        }
        (*job).numPages = (*job).pagesArraySize.x * (*job).pagesArraySize.y;
        imageSize.x = if imageSize.x < pageSize.x {
            imageSize.x
        } else {
            pageSize.x
        };
        imageSize.y = if imageSize.y < pageSize.y {
            imageSize.y
        } else {
            pageSize.y
        };
    } else {
        if !((*job).render.features).is_null() {
            pageSize.x = (*(*job).device.features).default_pagesize.x
                - 2 as libc::c_int as libc::c_double * margin.x;
            pageSize.x = fmax(pageSize.x, 0 as libc::c_int as libc::c_double);
            pageSize.y = (*(*job).device.features).default_pagesize.y
                - 2 as libc::c_int as libc::c_double * margin.y;
            pageSize.y = fmax(pageSize.y, 0 as libc::c_int as libc::c_double);
        } else {
            pageSize.y = 0.0f64;
            pageSize.x = pageSize.y;
        }
        let ref mut fresh66 = (*job).numPages;
        *fresh66 = 1 as libc::c_int;
        let ref mut fresh67 = (*job).pagesArraySize.y;
        *fresh67 = *fresh66;
        (*job).pagesArraySize.x = *fresh67;
        pageSize.x = fmax(pageSize.x, imageSize.x);
        pageSize.y = fmax(pageSize.y, imageSize.y);
    }
    (*job).width = (if (pageSize.x + 2 as libc::c_int as libc::c_double * margin.x) * (*job).dpi.x
        / 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        ((pageSize.x + 2 as libc::c_int as libc::c_double * margin.x) * (*job).dpi.x
            / 72 as libc::c_int as libc::c_double
            + 0.5f64) as libc::c_int
    } else {
        ((pageSize.x + 2 as libc::c_int as libc::c_double * margin.x) * (*job).dpi.x
            / 72 as libc::c_int as libc::c_double
            - 0.5f64) as libc::c_int
    }) as libc::c_uint;
    (*job).height = (if (pageSize.y + 2 as libc::c_int as libc::c_double * margin.y) * (*job).dpi.y
        / 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        ((pageSize.y + 2 as libc::c_int as libc::c_double * margin.y) * (*job).dpi.y
            / 72 as libc::c_int as libc::c_double
            + 0.5f64) as libc::c_int
    } else {
        ((pageSize.y + 2 as libc::c_int as libc::c_double * margin.y) * (*job).dpi.y
            / 72 as libc::c_int as libc::c_double
            - 0.5f64) as libc::c_int
    }) as libc::c_uint;
    let ref mut fresh68 = (*job).pagesArrayMinor.y;
    *fresh68 = 0 as libc::c_int;
    let ref mut fresh69 = (*job).pagesArrayMinor.x;
    *fresh69 = *fresh68;
    let ref mut fresh70 = (*job).pagesArrayMajor.y;
    *fresh70 = *fresh69;
    (*job).pagesArrayMajor.x = *fresh70;
    let ref mut fresh71 = (*job).pagesArrayFirst.y;
    *fresh71 = 0 as libc::c_int;
    (*job).pagesArrayFirst.x = *fresh71;
    (*job).pagesArrayMajor = pagecode(job, *((*gvc).pagedir).offset(0 as libc::c_int as isize));
    (*job).pagesArrayMinor = pagecode(job, *((*gvc).pagedir).offset(1 as libc::c_int as isize));
    if abs((*job).pagesArrayMajor.x + (*job).pagesArrayMinor.x) != 1 as libc::c_int
        || abs((*job).pagesArrayMajor.y + (*job).pagesArrayMinor.y) != 1 as libc::c_int
    {
        (*job).pagesArrayMajor = pagecode(job, 'B' as i32 as libc::c_char);
        (*job).pagesArrayMinor = pagecode(job, 'L' as i32 as libc::c_char);
        agerr(
            AGWARN,
            b"pagedir=%s ignored\n\0" as *const u8 as *const libc::c_char,
            (*gvc).pagedir,
        );
    }
    if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).centered {
        if pageSize.x > imageSize.x {
            centering.x = (pageSize.x - imageSize.x) / 2 as libc::c_int as libc::c_double;
        }
        if pageSize.y > imageSize.y {
            centering.y = (pageSize.y - imageSize.y) / 2 as libc::c_int as libc::c_double;
        }
    }
    if (*job).rotation != 0 {
        imageSize = exch_xyf(imageSize);
        pageSize = exch_xyf(pageSize);
        margin = exch_xyf(margin);
        centering = exch_xyf(centering);
    }
    (*job).canvasBox.LL.x = margin.x + centering.x;
    (*job).canvasBox.LL.y = margin.y + centering.y;
    (*job).canvasBox.UR.x = margin.x + centering.x + imageSize.x;
    (*job).canvasBox.UR.y = margin.y + centering.y + imageSize.y;
    (*job).pageSize.x = imageSize.x / (*job).zoom;
    (*job).pageSize.y = imageSize.y / (*job).zoom;
    (*job).pageBoundingBox.LL.x = if (*job).canvasBox.LL.x * (*job).dpi.x
        / 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        ((*job).canvasBox.LL.x * (*job).dpi.x / 72 as libc::c_int as libc::c_double + 0.5f64)
            as libc::c_int
    } else {
        ((*job).canvasBox.LL.x * (*job).dpi.x / 72 as libc::c_int as libc::c_double - 0.5f64)
            as libc::c_int
    };
    (*job).pageBoundingBox.LL.y = if (*job).canvasBox.LL.y * (*job).dpi.y
        / 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        ((*job).canvasBox.LL.y * (*job).dpi.y / 72 as libc::c_int as libc::c_double + 0.5f64)
            as libc::c_int
    } else {
        ((*job).canvasBox.LL.y * (*job).dpi.y / 72 as libc::c_int as libc::c_double - 0.5f64)
            as libc::c_int
    };
    (*job).pageBoundingBox.UR.x = if (*job).canvasBox.UR.x * (*job).dpi.x
        / 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        ((*job).canvasBox.UR.x * (*job).dpi.x / 72 as libc::c_int as libc::c_double + 0.5f64)
            as libc::c_int
    } else {
        ((*job).canvasBox.UR.x * (*job).dpi.x / 72 as libc::c_int as libc::c_double - 0.5f64)
            as libc::c_int
    };
    (*job).pageBoundingBox.UR.y = if (*job).canvasBox.UR.y * (*job).dpi.y
        / 72 as libc::c_int as libc::c_double
        >= 0 as libc::c_int as libc::c_double
    {
        ((*job).canvasBox.UR.y * (*job).dpi.y / 72 as libc::c_int as libc::c_double + 0.5f64)
            as libc::c_int
    } else {
        ((*job).canvasBox.UR.y * (*job).dpi.y / 72 as libc::c_int as libc::c_double - 0.5f64)
            as libc::c_int
    };
    if (*job).rotation != 0 {
        (*job).pageBoundingBox.LL = exch_xy((*job).pageBoundingBox.LL);
        (*job).pageBoundingBox.UR = exch_xy((*job).pageBoundingBox.UR);
        (*job).canvasBox.LL = exch_xyf((*job).canvasBox.LL);
        (*job).canvasBox.UR = exch_xyf((*job).canvasBox.UR);
    }
}
unsafe extern "C" fn firstpage(mut job: *mut GVJ_t) {
    (*job).pagesArrayElem = (*job).pagesArrayFirst;
}
unsafe extern "C" fn validpage(mut job: *mut GVJ_t) -> bool {
    return (*job).pagesArrayElem.x >= 0 as libc::c_int
        && (*job).pagesArrayElem.x < (*job).pagesArraySize.x
        && (*job).pagesArrayElem.y >= 0 as libc::c_int
        && (*job).pagesArrayElem.y < (*job).pagesArraySize.y;
}
unsafe extern "C" fn nextpage(mut job: *mut GVJ_t) {
    (*job).pagesArrayElem = add_point((*job).pagesArrayElem, (*job).pagesArrayMinor);
    if !validpage(job) {
        if (*job).pagesArrayMajor.y != 0 {
            (*job).pagesArrayElem.x = (*job).pagesArrayFirst.x;
        } else {
            (*job).pagesArrayElem.y = (*job).pagesArrayFirst.y;
        }
        (*job).pagesArrayElem = add_point((*job).pagesArrayElem, (*job).pagesArrayMajor);
    }
}
unsafe extern "C" fn write_edge_test(mut g: *mut Agraph_t, mut e: *mut Agedge_t) -> bool {
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut c: libc::c_int = 0;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        sg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize);
        if agcontains(sg, e as *mut libc::c_void) != 0 {
            return 0 as libc::c_int != 0;
        }
        c += 1;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn write_node_test(mut g: *mut Agraph_t, mut n: *mut Agnode_t) -> bool {
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut c: libc::c_int = 0;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        sg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize);
        if agcontains(sg, n as *mut libc::c_void) != 0 {
            return 0 as libc::c_int != 0;
        }
        c += 1;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn copyPts(
    mut pts: *mut pointf,
    mut ptsize: *mut libc::c_int,
    mut inpts: *mut xdot_point,
    mut numpts: libc::c_int,
) -> *mut pointf {
    let mut i: libc::c_int = 0;
    let mut sz: libc::c_int = *ptsize;
    if numpts > sz {
        sz = if 2 as libc::c_int * sz > numpts {
            2 as libc::c_int * sz
        } else {
            numpts
        };
        pts = grealloc(
            pts as *mut libc::c_void,
            (sz as libc::c_ulong).wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
        ) as *mut pointf;
        *ptsize = sz;
    }
    i = 0 as libc::c_int;
    while i < numpts {
        (*pts.offset(i as isize)).x = (*inpts.offset(i as isize)).x;
        (*pts.offset(i as isize)).y = (*inpts.offset(i as isize)).y;
        i += 1;
    }
    return pts;
}
unsafe extern "C" fn emit_xdot(mut job: *mut GVJ_t, mut xd: *mut xdot) {
    let mut image_warn: libc::c_int = 1 as libc::c_int;
    let mut ptsize: libc::c_int = 1000 as libc::c_int;
    let mut pts: *mut pointf = gcalloc(
        1000 as libc::c_int as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    let mut op: *mut exdot_op = 0 as *mut exdot_op;
    let mut i: libc::c_int = 0;
    let mut angle: libc::c_int = 0;
    let mut styles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut filled: libc::c_int = 1 as libc::c_int;
    op = (*xd).ops as *mut exdot_op;
    i = 0 as libc::c_int;
    while i < (*xd).cnt {
        match (*op).op.kind as libc::c_uint {
            0 | 1 => {
                if boxf_overlap((*op).bb, (*job).clip) != 0 {
                    (*pts.offset(0 as libc::c_int as isize)).x =
                        (*op).op.u.ellipse.x - (*op).op.u.ellipse.w;
                    (*pts.offset(0 as libc::c_int as isize)).y =
                        (*op).op.u.ellipse.y - (*op).op.u.ellipse.h;
                    (*pts.offset(1 as libc::c_int as isize)).x =
                        (*op).op.u.ellipse.x + (*op).op.u.ellipse.w;
                    (*pts.offset(1 as libc::c_int as isize)).y =
                        (*op).op.u.ellipse.y + (*op).op.u.ellipse.h;
                    gvrender_ellipse(
                        job,
                        pts,
                        2 as libc::c_int,
                        if (*op).op.kind as libc::c_uint
                            == xd_filled_ellipse as libc::c_int as libc::c_uint
                        {
                            filled
                        } else {
                            0 as libc::c_int
                        },
                    );
                }
            }
            2 | 3 => {
                if boxf_overlap((*op).bb, (*job).clip) != 0 {
                    pts = copyPts(
                        pts,
                        &mut ptsize,
                        (*op).op.u.polygon.pts,
                        (*op).op.u.polygon.cnt,
                    );
                    gvrender_polygon(
                        job,
                        pts,
                        (*op).op.u.polygon.cnt,
                        if (*op).op.kind as libc::c_uint
                            == xd_filled_polygon as libc::c_int as libc::c_uint
                        {
                            filled
                        } else {
                            0 as libc::c_int
                        },
                    );
                }
            }
            4 | 5 => {
                if boxf_overlap((*op).bb, (*job).clip) != 0 {
                    pts = copyPts(
                        pts,
                        &mut ptsize,
                        (*op).op.u.bezier.pts,
                        (*op).op.u.bezier.cnt,
                    );
                    gvrender_beziercurve(
                        job,
                        pts,
                        (*op).op.u.bezier.cnt,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        if (*op).op.kind as libc::c_uint
                            == xd_filled_bezier as libc::c_int as libc::c_uint
                        {
                            filled
                        } else {
                            0 as libc::c_int
                        },
                    );
                }
            }
            6 => {
                if boxf_overlap((*op).bb, (*job).clip) != 0 {
                    pts = copyPts(
                        pts,
                        &mut ptsize,
                        (*op).op.u.polyline.pts,
                        (*op).op.u.polyline.cnt,
                    );
                    gvrender_polyline(job, pts, (*op).op.u.polyline.cnt);
                }
            }
            7 => {
                if boxf_overlap((*op).bb, (*job).clip) != 0 {
                    (*pts.offset(0 as libc::c_int as isize)).x = (*op).op.u.text.x;
                    (*pts.offset(0 as libc::c_int as isize)).y = (*op).op.u.text.y;
                    gvrender_textspan(job, *pts.offset(0 as libc::c_int as isize), (*op).span);
                }
            }
            8 => {
                gvrender_set_fillcolor(job, (*op).op.u.color);
                filled = 1 as libc::c_int;
            }
            9 => {
                gvrender_set_pencolor(job, (*op).op.u.color);
                filled = 1 as libc::c_int;
            }
            13 => {
                let mut clr0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut clr1: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut frac: libc::c_float = 0.;
                if (*op).op.u.grad_color.type_0 as libc::c_uint
                    == xd_radial as libc::c_int as libc::c_uint
                {
                    let mut p: *mut xdot_radial_grad = &mut (*op).op.u.grad_color.u.ring;
                    clr0 = (*((*p).stops).offset(0 as libc::c_int as isize)).color;
                    clr1 = (*((*p).stops).offset(1 as libc::c_int as isize)).color;
                    frac = (*((*p).stops).offset(1 as libc::c_int as isize)).frac;
                    if (*p).x1 == (*p).x0 && (*p).y1 == (*p).y0 {
                        angle = 0 as libc::c_int;
                    } else {
                        angle = (180.0f64 * acos(((*p).x0 - (*p).x1) / (*p).r0)
                            / 3.14159265358979323846f64)
                            as libc::c_int;
                    }
                    gvrender_set_fillcolor(job, clr0);
                    gvrender_set_gradient_vals(job, clr1, angle, frac);
                    filled = 3 as libc::c_int;
                } else {
                    let mut p_0: *mut xdot_linear_grad = &mut (*op).op.u.grad_color.u.ling;
                    clr0 = (*((*p_0).stops).offset(0 as libc::c_int as isize)).color;
                    clr1 = (*((*p_0).stops).offset(1 as libc::c_int as isize)).color;
                    frac = (*((*p_0).stops).offset(1 as libc::c_int as isize)).frac;
                    angle = (180.0f64 * atan2((*p_0).y1 - (*p_0).y0, (*p_0).x1 - (*p_0).x0)
                        / 3.14159265358979323846f64) as libc::c_int;
                    gvrender_set_fillcolor(job, clr0);
                    gvrender_set_gradient_vals(job, clr1, angle, frac);
                    filled = 2 as libc::c_int;
                }
            }
            14 => {
                agerr(
                    AGWARN,
                    b"gradient pen colors not yet supported.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            10 => {}
            11 => {
                styles = parse_style((*op).op.u.style);
                gvrender_set_style(job, styles);
            }
            15 => {}
            12 => {
                if image_warn != 0 {
                    agerr(
                        AGWARN,
                        b"Images unsupported in \"background\" attribute\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    image_warn = 0 as libc::c_int;
                }
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"emit.c\0" as *const u8 as *const libc::c_char,
                    1581 as libc::c_int,
                );
                abort();
            }
        }
        op = op.offset(1);
        i += 1;
    }
    if !styles.is_null() {
        gvrender_set_style(job, (*(*job).gvc).defaultlinestyle);
    }
    free(pts as *mut libc::c_void);
}
unsafe extern "C" fn emit_background(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut xd: *mut xdot = 0 as *mut xdot;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dfltColor: libc::c_int = 0;
    str = agget(
        g as *mut libc::c_void,
        b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !(!str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0) {
        str = b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        dfltColor = 1 as libc::c_int;
    } else {
        dfltColor = 0 as libc::c_int;
    }
    if (*job).flags & (1 as libc::c_int) << 8 as libc::c_int == 0
        && strcmp(str, b"transparent\0" as *const u8 as *const libc::c_char) == 0
    {
        str = b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        dfltColor = 1 as libc::c_int;
    }
    if !((*job).flags & (1 as libc::c_int) << 8 as libc::c_int != 0
        && strcmp(str, b"transparent\0" as *const u8 as *const libc::c_char) == 0
        || (*job).flags & (1 as libc::c_int) << 25 as libc::c_int != 0 && dfltColor != 0)
    {
        let mut clrs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
        let mut frac: libc::c_float = 0.;
        if findStopColor(str, clrs.as_mut_ptr(), &mut frac) {
            let mut filled: libc::c_int = 0;
            let mut istyle: libc::c_int = 0 as libc::c_int;
            gvrender_set_fillcolor(job, clrs[0 as libc::c_int as usize]);
            gvrender_set_pencolor(
                job,
                b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            checkClusterStyle(g, &mut istyle);
            if !(clrs[1 as libc::c_int as usize]).is_null() {
                gvrender_set_gradient_vals(
                    job,
                    clrs[1 as libc::c_int as usize],
                    late_int(
                        g as *mut libc::c_void,
                        G_gradientangle,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    ),
                    frac,
                );
            } else {
                gvrender_set_gradient_vals(
                    job,
                    b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    late_int(
                        g as *mut libc::c_void,
                        G_gradientangle,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    ),
                    frac,
                );
            }
            if istyle & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                filled = 3 as libc::c_int;
            } else {
                filled = 2 as libc::c_int;
            }
            gvrender_box(job, (*job).clip, filled);
            free(clrs[0 as libc::c_int as usize] as *mut libc::c_void);
        } else {
            gvrender_set_fillcolor(job, str);
            gvrender_set_pencolor(
                job,
                b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            gvrender_box(job, (*job).clip, 1 as libc::c_int);
        }
    }
    xd = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).xdots as *mut xdot;
    if !xd.is_null() {
        emit_xdot(job, xd);
    }
}
unsafe extern "C" fn setup_page(mut job: *mut GVJ_t) {
    let mut pagesArrayElem: point = (*job).pagesArrayElem;
    let mut pagesArraySize: point = (*job).pagesArraySize;
    if (*job).rotation != 0 {
        pagesArrayElem = exch_xy(pagesArrayElem);
        pagesArraySize = exch_xy(pagesArraySize);
    }
    (*job).pageBox.LL.x = pagesArrayElem.x as libc::c_double * (*job).pageSize.x - (*job).pad.x;
    (*job).pageBox.LL.y = pagesArrayElem.y as libc::c_double * (*job).pageSize.y - (*job).pad.y;
    (*job).pageBox.UR.x = (*job).pageBox.LL.x + (*job).pageSize.x;
    (*job).pageBox.UR.y = (*job).pageBox.LL.y + (*job).pageSize.y;
    if (*(*job).common).viewNum == 0 as libc::c_int {
        (*job).boundingBox = (*job).pageBoundingBox;
    } else {
        (*job).boundingBox.LL.x = (if (*job).boundingBox.LL.x < (*job).pageBoundingBox.LL.x {
            (*job).boundingBox.LL.x
        } else {
            (*job).pageBoundingBox.LL.x
        });
        (*job).boundingBox.LL.y = (if (*job).boundingBox.LL.y < (*job).pageBoundingBox.LL.y {
            (*job).boundingBox.LL.y
        } else {
            (*job).pageBoundingBox.LL.y
        });
        (*job).boundingBox.UR.x = (if (*job).boundingBox.UR.x > (*job).pageBoundingBox.UR.x {
            (*job).boundingBox.UR.x
        } else {
            (*job).pageBoundingBox.UR.x
        });
        (*job).boundingBox.UR.y = (if (*job).boundingBox.UR.y > (*job).pageBoundingBox.UR.y {
            (*job).boundingBox.UR.y
        } else {
            (*job).pageBoundingBox.UR.y
        });
    }
    if (*job).flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        (*job).clip.LL.x = (*job).focus.x - (*job).view.x / 2.0f64;
        (*job).clip.LL.y = (*job).focus.y - (*job).view.y / 2.0f64;
        (*job).clip.UR.x = (*job).focus.x + (*job).view.x / 2.0f64;
        (*job).clip.UR.y = (*job).focus.y + (*job).view.y / 2.0f64;
    } else {
        (*job).clip.LL.x = (*job).focus.x
            + (*job).pageSize.x
                * (pagesArrayElem.x as libc::c_double
                    - pagesArraySize.x as libc::c_double / 2.0f64);
        (*job).clip.LL.y = (*job).focus.y
            + (*job).pageSize.y
                * (pagesArrayElem.y as libc::c_double
                    - pagesArraySize.y as libc::c_double / 2.0f64);
        (*job).clip.UR.x = (*job).clip.LL.x + (*job).pageSize.x;
        (*job).clip.UR.y = (*job).clip.LL.y + (*job).pageSize.y;
    }
    if (*job).rotation != 0 {
        (*job).translation.y = -(*job).clip.UR.y - (*job).canvasBox.LL.y / (*job).zoom;
        if (*job).flags & (1 as libc::c_int) << 12 as libc::c_int != 0 || Y_invert != 0 {
            (*job).translation.x = -(*job).clip.UR.x - (*job).canvasBox.LL.x / (*job).zoom;
        } else {
            (*job).translation.x = -(*job).clip.LL.x + (*job).canvasBox.LL.x / (*job).zoom;
        }
    } else {
        (*job).translation.x = -(*job).clip.LL.x + (*job).canvasBox.LL.x / (*job).zoom;
        if (*job).flags & (1 as libc::c_int) << 12 as libc::c_int != 0 || Y_invert != 0 {
            (*job).translation.y = -(*job).clip.UR.y - (*job).canvasBox.LL.y / (*job).zoom;
        } else {
            (*job).translation.y = -(*job).clip.LL.y + (*job).canvasBox.LL.y / (*job).zoom;
        }
    };
}
unsafe extern "C" fn node_in_layer(
    mut job: *mut GVJ_t,
    mut g: *mut graph_t,
    mut n: *mut node_t,
) -> bool {
    let mut pn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pe: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    if (*job).numLayers <= 1 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    pn = late_string(
        n as *mut libc::c_void,
        N_layer,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if selectedlayer(job, pn) {
        return 1 as libc::c_int != 0;
    }
    if *pn.offset(0 as libc::c_int as isize) != 0 {
        return 0 as libc::c_int != 0;
    }
    e = agfstedge(g, n);
    if e.is_null() {
        return 1 as libc::c_int != 0;
    }
    e = agfstedge(g, n);
    while !e.is_null() {
        pe = late_string(
            e as *mut libc::c_void,
            E_layer,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *pe.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
            || selectedlayer(job, pe) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        e = agnxtedge(g, e, n);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn edge_in_layer(mut job: *mut GVJ_t, mut e: *mut edge_t) -> bool {
    let mut pe: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cnt: libc::c_int = 0;
    if (*job).numLayers <= 1 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    pe = late_string(
        e as *mut libc::c_void,
        E_layer,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if selectedlayer(job, pe) {
        return 1 as libc::c_int != 0;
    }
    if *pe.offset(0 as libc::c_int as isize) != 0 {
        return 0 as libc::c_int != 0;
    }
    cnt = 0 as libc::c_int;
    while cnt < 2 as libc::c_int {
        pn = late_string(
            (if cnt < 1 as libc::c_int {
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                .node
            } else {
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                .node
            }) as *mut libc::c_void,
            N_layer,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *pn.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
            || selectedlayer(job, pn) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        cnt += 1;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn clust_in_layer(mut job: *mut GVJ_t, mut sg: *mut graph_t) -> bool {
    let mut pg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut node_t = 0 as *mut node_t;
    if (*job).numLayers <= 1 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    pg = late_string(
        sg as *mut libc::c_void,
        agattr(
            sg,
            0 as libc::c_int,
            b"layer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if selectedlayer(job, pg) {
        return 1 as libc::c_int != 0;
    }
    if *pg.offset(0 as libc::c_int as isize) != 0 {
        return 0 as libc::c_int != 0;
    }
    n = agfstnode(sg);
    while !n.is_null() {
        if node_in_layer(job, sg, n) {
            return 1 as libc::c_int != 0;
        }
        n = agnxtnode(sg, n);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn node_in_box(mut n: *mut node_t, mut b: boxf) -> bool {
    return boxf_overlap((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).bb, b)
        != 0 as libc::c_int;
}
unsafe extern "C" fn emit_begin_node(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    let mut flags: libc::c_int = (*job).flags;
    let mut sides: libc::c_int = 0;
    let mut peripheries: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rect: libc::c_int = 0 as libc::c_int;
    let mut shape: libc::c_int = 0;
    let mut nump: libc::c_int = 0 as libc::c_int;
    let mut poly: *mut polygon_t = 0 as *mut polygon_t;
    let mut vertices: *mut pointf = 0 as *mut pointf;
    let mut p: *mut pointf = 0 as *mut pointf;
    let mut coord: pointf = pointf { x: 0., y: 0. };
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    obj = push_obj_state(job);
    (*obj).type_0 = NODE_OBJTYPE;
    let ref mut fresh72 = (*obj).u.n;
    *fresh72 = n;
    (*obj).emit_state = EMIT_NDRAW;
    if flags & (1 as libc::c_int) << 24 as libc::c_int != 0 {
        if (*((*(agraphof(n as *mut libc::c_void) as *mut Agobj_t)).data as *mut Agraphinfo_t)).odim
            as libc::c_int
            >= 3 as libc::c_int
        {
            (*obj).z = (if *((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                .offset(2 as libc::c_int as isize)
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(2 as libc::c_int as isize)
                    * 72 as libc::c_int as libc::c_double
                    + 0.5f64) as libc::c_int
            } else {
                (*((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).pos)
                    .offset(2 as libc::c_int as isize)
                    * 72 as libc::c_int as libc::c_double
                    - 0.5f64) as libc::c_int
            }) as libc::c_double;
        } else {
            (*obj).z = 0.0f64;
        }
    }
    initObjMapData(
        job,
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).label,
        n as *mut libc::c_void,
    );
    if flags & ((1 as libc::c_int) << 16 as libc::c_int | (1 as libc::c_int) << 22 as libc::c_int)
        != 0
        && (!((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0)
    {
        shape = shapeOf(n) as libc::c_int;
        coord = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).coord;
        let mut filled: bool = isFilled(n);
        if shape == SH_POLY as libc::c_int || shape == SH_POINT as libc::c_int {
            poly =
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape_info as *mut polygon_t;
            if isRect(poly) as libc::c_int != 0
                && ((*poly).peripheries != 0 || filled as libc::c_int != 0)
            {
                rect = 1 as libc::c_int;
            }
        }
        if !poly.is_null() && rect == 0 && flags & (1 as libc::c_int) << 19 as libc::c_int != 0 {
            if (*poly).sides < 3 as libc::c_int {
                sides = 1 as libc::c_int;
            } else {
                sides = (*poly).sides;
            }
            if (*poly).peripheries < 2 as libc::c_int {
                peripheries = 1 as libc::c_int;
            } else {
                peripheries = (*poly).peripheries;
            }
            vertices = (*poly).vertices;
            s = agget(
                n as *mut libc::c_void,
                b"samplepoints\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !s.is_null() {
                nump = atoi(s);
            }
            if nump < 4 as libc::c_int || nump > 60 as libc::c_int {
                nump = 20 as libc::c_int;
            }
            if (*poly).peripheries == 0 as libc::c_int && !filled {
                (*obj).url_map_shape = MAP_RECTANGLE;
                nump = 2 as libc::c_int;
                p = gcalloc(
                    nump as size_t,
                    ::std::mem::size_of::<pointf>() as libc::c_ulong,
                ) as *mut pointf;
                (*p.offset(0 as libc::c_int as isize)).x =
                    coord.x - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                (*p.offset(0 as libc::c_int as isize)).y =
                    coord.y - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
                (*p.offset(1 as libc::c_int as isize)).x =
                    coord.x + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
                (*p.offset(1 as libc::c_int as isize)).y =
                    coord.y + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
            } else if (*poly).sides < 3 as libc::c_int
                && (*poly).skew == 0.0f64
                && (*poly).distortion == 0.0f64
            {
                if (*poly).regular != 0 {
                    (*obj).url_map_shape = MAP_CIRCLE;
                    nump = 2 as libc::c_int;
                    p = gcalloc(
                        nump as size_t,
                        ::std::mem::size_of::<pointf>() as libc::c_ulong,
                    ) as *mut pointf;
                    (*p.offset(0 as libc::c_int as isize)).x = coord.x;
                    (*p.offset(0 as libc::c_int as isize)).y = coord.y;
                    (*p.offset(1 as libc::c_int as isize)).x = coord.x
                        + (*vertices
                            .offset((2 as libc::c_int * peripheries - 1 as libc::c_int) as isize))
                        .x;
                    (*p.offset(1 as libc::c_int as isize)).y = coord.y
                        + (*vertices
                            .offset((2 as libc::c_int * peripheries - 1 as libc::c_int) as isize))
                        .y;
                } else {
                    (*obj).url_map_shape = MAP_POLYGON;
                    p = pEllipse(
                        (*vertices
                            .offset((2 as libc::c_int * peripheries - 1 as libc::c_int) as isize))
                        .x,
                        (*vertices
                            .offset((2 as libc::c_int * peripheries - 1 as libc::c_int) as isize))
                        .y,
                        nump,
                    );
                    i = 0 as libc::c_int;
                    while i < nump {
                        (*p.offset(i as isize)).x += coord.x;
                        (*p.offset(i as isize)).y += coord.y;
                        i += 1;
                    }
                }
            } else {
                let mut offset: libc::c_int = (peripheries - 1 as libc::c_int) * (*poly).sides;
                (*obj).url_map_shape = MAP_POLYGON;
                if (*poly).sides >= nump {
                    let mut delta: libc::c_int = (*poly).sides / nump;
                    p = gcalloc(
                        nump as size_t,
                        ::std::mem::size_of::<pointf>() as libc::c_ulong,
                    ) as *mut pointf;
                    i = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j < nump {
                        (*p.offset(j as isize)).x =
                            coord.x + (*vertices.offset((i + offset) as isize)).x;
                        (*p.offset(j as isize)).y =
                            coord.y + (*vertices.offset((i + offset) as isize)).y;
                        i += delta;
                        j += 1;
                    }
                } else {
                    nump = sides;
                    p = gcalloc(
                        nump as size_t,
                        ::std::mem::size_of::<pointf>() as libc::c_ulong,
                    ) as *mut pointf;
                    i = 0 as libc::c_int;
                    while i < nump {
                        (*p.offset(i as isize)).x =
                            coord.x + (*vertices.offset((i + offset) as isize)).x;
                        (*p.offset(i as isize)).y =
                            coord.y + (*vertices.offset((i + offset) as isize)).y;
                        i += 1;
                    }
                }
            }
        } else {
            (*obj).url_map_shape = MAP_RECTANGLE;
            nump = 2 as libc::c_int;
            p = gcalloc(
                nump as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            (*p.offset(0 as libc::c_int as isize)).x =
                coord.x - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
            (*p.offset(0 as libc::c_int as isize)).y = coord.y
                - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double;
            (*p.offset(1 as libc::c_int as isize)).x =
                coord.x + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
            (*p.offset(1 as libc::c_int as isize)).y = coord.y
                + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht
                    / 2 as libc::c_int as libc::c_double;
        }
        if flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
            gvrender_ptf_A(job, p, p, nump);
        }
        let ref mut fresh73 = (*obj).url_map_p;
        *fresh73 = p;
        (*obj).url_map_n = nump;
    }
    setColorScheme(agget(
        n as *mut libc::c_void,
        b"colorscheme\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    gvrender_begin_node(job, n);
}
unsafe extern "C" fn emit_end_node(mut job: *mut GVJ_t) {
    gvrender_end_node(job);
    pop_obj_state(job);
}
unsafe extern "C" fn emit_node(mut job: *mut GVJ_t, mut n: *mut node_t) {
    let mut gvc: *mut GVC_t = (*job).gvc;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut style: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut styles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut sp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).is_null()
        && node_in_layer(job, agraphof(n as *mut libc::c_void), n) as libc::c_int != 0
        && node_in_box(n, (*job).clip) as libc::c_int != 0
        && (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).state as libc::c_int
            != (*gvc).common.viewNum
    {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).state =
            (*gvc).common.viewNum as libc::c_char;
        gvrender_comment(job, agnameof(n as *mut libc::c_void));
        s = late_string(
            n as *mut libc::c_void,
            N_comment,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *s.offset(0 as libc::c_int as isize) != 0 {
            gvrender_comment(job, s);
        }
        style = late_string(
            n as *mut libc::c_void,
            N_style,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *style.offset(0 as libc::c_int as isize) != 0 {
            styles = parse_style(style);
            sp = styles;
            loop {
                let fresh74 = sp;
                sp = sp.offset(1);
                p = *fresh74;
                if p.is_null() {
                    break;
                }
                if strcmp(p, b"invis\0" as *const u8 as *const libc::c_char) == 0 {
                    return;
                }
            }
        }
        emit_begin_node(job, n);
        ((*(*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).shape).fns).codefn)
            .expect("non-null function pointer")(job, n);
        if !((*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).is_null()
            && (*(*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel).set as libc::c_int
                != 0
        {
            emit_label(
                job,
                EMIT_NLABEL,
                (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).xlabel,
            );
        }
        emit_end_node(job);
    }
}
unsafe extern "C" fn computeoffset_p(
    mut p: pointf,
    mut q: pointf,
    mut d: libc::c_double,
) -> pointf {
    let mut res: pointf = pointf { x: 0., y: 0. };
    let mut x: libc::c_double = p.x - q.x;
    let mut y: libc::c_double = p.y - q.y;
    d /= sqrt(x * x + y * y + 0.0001f64);
    res.x = y * d;
    res.y = -x * d;
    return res;
}
unsafe extern "C" fn computeoffset_qr(
    mut p: pointf,
    mut q: pointf,
    mut r: pointf,
    mut s: pointf,
    mut d: libc::c_double,
) -> pointf {
    let mut res: pointf = pointf { x: 0., y: 0. };
    let mut len: libc::c_double = 0.;
    let mut x: libc::c_double = q.x - r.x;
    let mut y: libc::c_double = q.y - r.y;
    len = hypot(x, y);
    if len < 0.0001f64 {
        x = p.x - s.x;
        y = p.y - s.y;
        len = sqrt(x * x + y * y + 0.0001f64);
    }
    d /= len;
    res.x = y * d;
    res.y = -x * d;
    return res;
}
unsafe extern "C" fn emit_attachment(
    mut job: *mut GVJ_t,
    mut lp: *mut textlabel_t,
    mut spl: *mut splines,
) {
    let mut sz: pointf = pointf { x: 0., y: 0. };
    let mut AF: [pointf; 3] = [pointf { x: 0., y: 0. }; 3];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = (*lp).text;
    while *s != 0 {
        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            break;
        }
        s = s.offset(1);
    }
    if *s as libc::c_int == '\0' as i32 {
        return;
    }
    sz = (*lp).dimen;
    AF[0 as libc::c_int as usize] =
        pointfof((*lp).pos.x + sz.x / 2.0f64, (*lp).pos.y - sz.y / 2.0f64);
    AF[1 as libc::c_int as usize] = pointfof(
        AF[0 as libc::c_int as usize].x - sz.x,
        AF[0 as libc::c_int as usize].y,
    );
    AF[2 as libc::c_int as usize] = dotneato_closest(spl, (*lp).pos);
    gvrender_set_style(job, (*(*job).gvc).defaultlinestyle);
    gvrender_set_pencolor(job, (*lp).fontcolor);
    gvrender_polyline(job, AF.as_mut_ptr(), 3 as libc::c_int);
}
unsafe extern "C" fn default_pencolor(
    mut pencolor: *mut libc::c_char,
    mut deflt: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut ncol: size_t = 0;
    ncol = 1 as libc::c_int as size_t;
    p = pencolor;
    while *p != 0 {
        if *p as libc::c_int == ':' as i32 {
            ncol = ncol.wrapping_add(1);
        }
        p = p.offset(1);
    }
    len = ncol.wrapping_mul((strlen(deflt)).wrapping_add(1 as libc::c_int as libc::c_ulong));
    if bufsz < len {
        bufsz = len.wrapping_add(10 as libc::c_int as libc::c_ulong);
        buf = realloc(buf as *mut libc::c_void, bufsz) as *mut libc::c_char;
    }
    strcpy(buf, deflt);
    loop {
        ncol = ncol.wrapping_sub(1);
        if !(ncol != 0) {
            break;
        }
        strcat(buf, b":\0" as *const u8 as *const libc::c_char);
        strcat(buf, deflt);
    }
    return buf;
}
unsafe extern "C" fn approxLen(mut pts: *mut pointf) -> libc::c_double {
    let mut d: libc::c_double = sqrt(
        ((*pts.offset(0 as libc::c_int as isize)).x - (*pts.offset(1 as libc::c_int as isize)).x)
            * ((*pts.offset(0 as libc::c_int as isize)).x
                - (*pts.offset(1 as libc::c_int as isize)).x)
            + ((*pts.offset(0 as libc::c_int as isize)).y
                - (*pts.offset(1 as libc::c_int as isize)).y)
                * ((*pts.offset(0 as libc::c_int as isize)).y
                    - (*pts.offset(1 as libc::c_int as isize)).y),
    );
    d += sqrt(
        ((*pts.offset(1 as libc::c_int as isize)).x - (*pts.offset(2 as libc::c_int as isize)).x)
            * ((*pts.offset(1 as libc::c_int as isize)).x
                - (*pts.offset(2 as libc::c_int as isize)).x)
            + ((*pts.offset(1 as libc::c_int as isize)).y
                - (*pts.offset(2 as libc::c_int as isize)).y)
                * ((*pts.offset(1 as libc::c_int as isize)).y
                    - (*pts.offset(2 as libc::c_int as isize)).y),
    );
    d += sqrt(
        ((*pts.offset(2 as libc::c_int as isize)).x - (*pts.offset(3 as libc::c_int as isize)).x)
            * ((*pts.offset(2 as libc::c_int as isize)).x
                - (*pts.offset(3 as libc::c_int as isize)).x)
            + ((*pts.offset(2 as libc::c_int as isize)).y
                - (*pts.offset(3 as libc::c_int as isize)).y)
                * ((*pts.offset(2 as libc::c_int as isize)).y
                    - (*pts.offset(3 as libc::c_int as isize)).y),
    );
    return d;
}
unsafe extern "C" fn splitBSpline(
    mut bz: *mut bezier,
    mut t: libc::c_float,
    mut left: *mut bezier,
    mut right: *mut bezier,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cnt: libc::c_int = ((*bz).size - 1 as libc::c_int) / 3 as libc::c_int;
    let mut lens: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut last: libc::c_double = 0.;
    let mut len: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut pts: *mut pointf = 0 as *mut pointf;
    let mut r: libc::c_float = 0.;
    if cnt == 1 as libc::c_int {
        (*left).size = 4 as libc::c_int;
        let ref mut fresh75 = (*left).list;
        *fresh75 = gcalloc(
            4 as libc::c_int as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        (*right).size = 4 as libc::c_int;
        let ref mut fresh76 = (*right).list;
        *fresh76 = gcalloc(
            4 as libc::c_int as size_t,
            ::std::mem::size_of::<pointf>() as libc::c_ulong,
        ) as *mut pointf;
        Bezier(
            (*bz).list,
            3 as libc::c_int,
            t as libc::c_double,
            (*left).list,
            (*right).list,
        );
        return;
    }
    lens = gcalloc(
        cnt as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    sum = 0 as libc::c_int as libc::c_double;
    pts = (*bz).list;
    i = 0 as libc::c_int;
    while i < cnt {
        *lens.offset(i as isize) = approxLen(pts);
        sum += *lens.offset(i as isize);
        pts = pts.offset(3 as libc::c_int as isize);
        i += 1;
    }
    len = t as libc::c_double * sum;
    sum = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < cnt {
        sum += *lens.offset(i as isize);
        if sum >= len {
            break;
        }
        i += 1;
    }
    (*left).size = 3 as libc::c_int * (i + 1 as libc::c_int) + 1 as libc::c_int;
    let ref mut fresh77 = (*left).list;
    *fresh77 = gcalloc(
        (*left).size as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    (*right).size = 3 as libc::c_int * (cnt - i) + 1 as libc::c_int;
    let ref mut fresh78 = (*right).list;
    *fresh78 = gcalloc(
        (*right).size as size_t,
        ::std::mem::size_of::<pointf>() as libc::c_ulong,
    ) as *mut pointf;
    j = 0 as libc::c_int;
    while j < (*left).size {
        *((*left).list).offset(j as isize) = *((*bz).list).offset(j as isize);
        j += 1;
    }
    k = j - 4 as libc::c_int;
    j = 0 as libc::c_int;
    while j < (*right).size {
        let fresh79 = k;
        k = k + 1;
        *((*right).list).offset(j as isize) = *((*bz).list).offset(fresh79 as isize);
        j += 1;
    }
    last = *lens.offset(i as isize);
    r = ((len - (sum - last)) / last) as libc::c_float;
    Bezier(
        ((*bz).list).offset((3 as libc::c_int * i) as isize),
        3 as libc::c_int,
        r as libc::c_double,
        ((*left).list).offset((3 as libc::c_int * i) as isize),
        (*right).list,
    );
    free(lens as *mut libc::c_void);
}
unsafe extern "C" fn multicolor(
    mut job: *mut GVJ_t,
    mut e: *mut edge_t,
    mut styles: *mut *mut libc::c_char,
    mut colors: *mut libc::c_char,
    mut num: libc::c_int,
    mut arrowsize: libc::c_double,
    mut penwidth: libc::c_double,
) -> libc::c_int {
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut bz0: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut bz_l: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut bz_r: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut i: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut segs: *mut colorsegs_t = 0 as *mut colorsegs_t;
    let mut s: *mut colorseg_t = 0 as *mut colorseg_t;
    let mut endcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left: libc::c_double = 0.;
    let mut first: libc::c_int = 0;
    rv = parseSegs(colors, num, &mut segs);
    if rv > 1 as libc::c_int {
        let mut g: *mut Agraph_t = agraphof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            })
            .node as *mut libc::c_void,
        );
        agerr(
            AGPREV,
            b"in edge %s%s%s\n\0" as *const u8 as *const libc::c_char,
            agnameof(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                .node as *mut libc::c_void,
            ),
            if agisdirected(g) != 0 {
                b" -> \0" as *const u8 as *const libc::c_char
            } else {
                b" -- \0" as *const u8 as *const libc::c_char
            },
            agnameof(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                .node as *mut libc::c_void,
            ),
        );
        if rv == 2 as libc::c_int {
            return 1 as libc::c_int;
        }
    } else if rv == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
        left = 1 as libc::c_int as libc::c_double;
        bz =
            *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list).offset(i as isize);
        first = 1 as libc::c_int;
        s = (*segs).segs;
        while !((*s).color).is_null() {
            if !(((*s).t as libc::c_double) < 1E-5f64 && (*s).t as libc::c_double > -1E-5f64) {
                gvrender_set_pencolor(job, (*s).color);
                left -= (*s).t as libc::c_double;
                endcolor = (*s).color;
                if first != 0 {
                    first = 0 as libc::c_int;
                    splitBSpline(&mut bz, (*s).t, &mut bz_l, &mut bz_r);
                    gvrender_beziercurve(
                        job,
                        bz_l.list,
                        bz_l.size,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    free(bz_l.list as *mut libc::c_void);
                    if left < 1E-5f64 && left > -1E-5f64 {
                        free(bz_r.list as *mut libc::c_void);
                        break;
                    }
                } else if left < 1E-5f64 && left > -1E-5f64 {
                    gvrender_beziercurve(
                        job,
                        bz_r.list,
                        bz_r.size,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    free(bz_r.list as *mut libc::c_void);
                    break;
                } else {
                    bz0 = bz_r;
                    splitBSpline(
                        &mut bz0,
                        ((*s).t as libc::c_double / (left + (*s).t as libc::c_double))
                            as libc::c_float,
                        &mut bz_l,
                        &mut bz_r,
                    );
                    free(bz0.list as *mut libc::c_void);
                    gvrender_beziercurve(
                        job,
                        bz_l.list,
                        bz_l.size,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    free(bz_l.list as *mut libc::c_void);
                }
            }
            s = s.offset(1);
        }
        if bz.sflag != 0 {
            gvrender_set_pencolor(job, (*(*segs).segs).color);
            gvrender_set_fillcolor(job, (*(*segs).segs).color);
            arrow_gen(
                job,
                EMIT_TDRAW,
                bz.sp,
                *(bz.list).offset(0 as libc::c_int as isize),
                arrowsize,
                penwidth,
                bz.sflag,
            );
        }
        if bz.eflag != 0 {
            gvrender_set_pencolor(job, endcolor);
            gvrender_set_fillcolor(job, endcolor);
            arrow_gen(
                job,
                EMIT_HDRAW,
                bz.ep,
                *(bz.list).offset((bz.size - 1 as libc::c_int) as isize),
                arrowsize,
                penwidth,
                bz.eflag,
            );
        }
        if (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size > 1 as libc::c_int
            && (bz.sflag != 0 || bz.eflag != 0)
            && !styles.is_null()
        {
            gvrender_set_style(job, styles);
        }
        i += 1;
    }
    freeSegs(segs);
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_stroke(mut sp: *mut stroke_t) {
    if !sp.is_null() {
        free((*sp).vertices as *mut libc::c_void);
        free(sp as *mut libc::c_void);
    }
}
unsafe extern "C" fn forfunc(
    mut curlen: libc::c_double,
    mut totallen: libc::c_double,
    mut initwid: libc::c_double,
) -> libc::c_double {
    return (1 as libc::c_int as libc::c_double - curlen / totallen) * initwid / 2.0f64;
}
unsafe extern "C" fn revfunc(
    mut curlen: libc::c_double,
    mut totallen: libc::c_double,
    mut initwid: libc::c_double,
) -> libc::c_double {
    return curlen / totallen * initwid / 2.0f64;
}
unsafe extern "C" fn nonefunc(
    mut curlen: libc::c_double,
    mut totallen: libc::c_double,
    mut initwid: libc::c_double,
) -> libc::c_double {
    return initwid / 2.0f64;
}
unsafe extern "C" fn bothfunc(
    mut curlen: libc::c_double,
    mut totallen: libc::c_double,
    mut initwid: libc::c_double,
) -> libc::c_double {
    let mut fr: libc::c_double = curlen / totallen;
    if fr <= 0.5f64 {
        return fr * initwid;
    }
    return (1 as libc::c_int as libc::c_double - fr) * initwid;
}
unsafe extern "C" fn taperfun(mut e: *mut edge_t) -> radfunc_t {
    let mut attr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !E_dir.is_null() && {
        attr = agxget(e as *mut libc::c_void, E_dir);
        *attr.offset(0 as libc::c_int as isize) as libc::c_int != 0
    } {
        if strcmp(attr, b"forward\0" as *const u8 as *const libc::c_char) == 0 {
            return Some(
                forfunc
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            );
        }
        if strcmp(attr, b"back\0" as *const u8 as *const libc::c_char) == 0 {
            return Some(
                revfunc
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            );
        }
        if strcmp(attr, b"both\0" as *const u8 as *const libc::c_char) == 0 {
            return Some(
                bothfunc
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            );
        }
        if strcmp(attr, b"none\0" as *const u8 as *const libc::c_char) == 0 {
            return Some(
                nonefunc
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            );
        }
    }
    return if agisdirected(agraphof(
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
        .node as *mut libc::c_void,
    )) != 0
    {
        Some(
            forfunc
                as unsafe extern "C" fn(
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_double,
        )
    } else {
        Some(
            nonefunc
                as unsafe extern "C" fn(
                    libc::c_double,
                    libc::c_double,
                    libc::c_double,
                ) -> libc::c_double,
        )
    };
}
unsafe extern "C" fn emit_edge_graphics(
    mut job: *mut GVJ_t,
    mut e: *mut edge_t,
    mut styles: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnum: libc::c_int = 0;
    let mut numc: libc::c_int = 0 as libc::c_int;
    let mut numsemi: libc::c_int = 0 as libc::c_int;
    let mut color: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pencolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fillcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut headcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tailcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut colors: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut offspl: splines = splines {
        list: 0 as *mut bezier,
        size: 0,
        bb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
    };
    let mut tmpspl: splines = splines {
        list: 0 as *mut bezier,
        size: 0,
        bb: boxf {
            LL: pointf { x: 0., y: 0. },
            UR: pointf { x: 0., y: 0. },
        },
    };
    let mut pf0: pointf = pointf { x: 0., y: 0. };
    let mut pf1: pointf = pointf { x: 0., y: 0. };
    let mut pf2: pointf = {
        let mut init = pointf_s {
            x: 0 as libc::c_int as libc::c_double,
            y: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut pf3: pointf = pointf { x: 0., y: 0. };
    let mut offlist: *mut pointf = 0 as *mut pointf;
    let mut tmplist: *mut pointf = 0 as *mut pointf;
    let mut arrowsize: libc::c_double = 0.;
    let mut numc2: libc::c_double = 0.;
    let mut penwidth: libc::c_double = (*(*job).obj).penwidth;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tapered: bool = 0 as libc::c_int != 0;
    setColorScheme(agget(
        e as *mut libc::c_void,
        b"colorscheme\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
        arrowsize = late_double(e as *mut libc::c_void, E_arrowsz, 1.0f64, 0.0f64);
        color = late_string(
            e as *mut libc::c_void,
            E_color,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !styles.is_null() {
            let mut sp: *mut *mut libc::c_char = styles;
            loop {
                let fresh80 = sp;
                sp = sp.offset(1);
                p = *fresh80;
                if p.is_null() {
                    break;
                }
                if !(strcmp(p, b"tapered\0" as *const u8 as *const libc::c_char) == 0) {
                    continue;
                }
                tapered = 1 as libc::c_int != 0;
                break;
            }
        }
        p = color;
        while *p != 0 {
            if *p as libc::c_int == ':' as i32 {
                numc += 1;
            } else if *p as libc::c_int == ';' as i32 {
                numsemi += 1;
            }
            p = p.offset(1);
        }
        if numsemi != 0 && numc != 0 {
            if multicolor(
                job,
                e,
                styles,
                color,
                numc + 1 as libc::c_int,
                arrowsize,
                penwidth,
            ) != 0
            {
                color = b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else {
                return;
            }
        }
        pencolor = color;
        fillcolor = pencolor;
        if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state as libc::c_int
            & (1 as libc::c_int) << 0 as libc::c_int
            != 0
        {
            pencolor = late_nnstring(
                e as *mut libc::c_void,
                E_activepencolor,
                default_pencolor(
                    pencolor,
                    b"#808080\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            fillcolor = late_nnstring(
                e as *mut libc::c_void,
                E_activefillcolor,
                b"#fcfcfc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int
            != 0
        {
            pencolor = late_nnstring(
                e as *mut libc::c_void,
                E_selectedpencolor,
                default_pencolor(
                    pencolor,
                    b"#303030\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            fillcolor = late_nnstring(
                e as *mut libc::c_void,
                E_selectedfillcolor,
                b"#e8e8e8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state as libc::c_int
            & (1 as libc::c_int) << 3 as libc::c_int
            != 0
        {
            pencolor = late_nnstring(
                e as *mut libc::c_void,
                E_deletedpencolor,
                default_pencolor(
                    pencolor,
                    b"#e0e0e0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            fillcolor = late_nnstring(
                e as *mut libc::c_void,
                E_deletedfillcolor,
                b"#f0f0f0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int
            != 0
        {
            pencolor = late_nnstring(
                e as *mut libc::c_void,
                E_visitedpencolor,
                default_pencolor(
                    pencolor,
                    b"#101010\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
            fillcolor = late_nnstring(
                e as *mut libc::c_void,
                E_visitedfillcolor,
                b"#f8f8f8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            fillcolor = late_nnstring(e as *mut libc::c_void, E_fillcolor, color);
        }
        if pencolor != color {
            gvrender_set_pencolor(job, pencolor);
        }
        if fillcolor != color {
            gvrender_set_fillcolor(job, fillcolor);
        }
        color = pencolor;
        if tapered {
            let mut stp: *mut stroke_t = 0 as *mut stroke_t;
            if *color as libc::c_int == '\0' as i32 {
                color = b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            if *fillcolor as libc::c_int == '\0' as i32 {
                fillcolor = b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            gvrender_set_pencolor(
                job,
                b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            gvrender_set_fillcolor(job, color);
            bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                .offset(0 as libc::c_int as isize);
            stp = taper(
                &mut bz,
                taperfun(e),
                penwidth,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            gvrender_polygon(
                job,
                (*stp).vertices,
                (*stp).nvertices,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            free_stroke(stp);
            gvrender_set_pencolor(job, color);
            if fillcolor != color {
                gvrender_set_fillcolor(job, fillcolor);
            }
            if bz.sflag != 0 {
                arrow_gen(
                    job,
                    EMIT_TDRAW,
                    bz.sp,
                    *(bz.list).offset(0 as libc::c_int as isize),
                    arrowsize,
                    penwidth,
                    bz.sflag,
                );
            }
            if bz.eflag != 0 {
                arrow_gen(
                    job,
                    EMIT_HDRAW,
                    bz.ep,
                    *(bz.list).offset((bz.size - 1 as libc::c_int) as isize),
                    arrowsize,
                    penwidth,
                    bz.eflag,
                );
            }
        } else if numc != 0 {
            offspl.size = (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size;
            tmpspl.size = offspl.size;
            offspl.list = malloc(
                (::std::mem::size_of::<bezier>() as libc::c_ulong)
                    .wrapping_mul(offspl.size as libc::c_ulong),
            ) as *mut bezier;
            tmpspl.list = malloc(
                (::std::mem::size_of::<bezier>() as libc::c_ulong)
                    .wrapping_mul(tmpspl.size as libc::c_ulong),
            ) as *mut bezier;
            numc2 = (2 as libc::c_int + numc) as libc::c_double / 2.0f64;
            i = 0 as libc::c_int;
            while i < offspl.size {
                bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                    .offset(i as isize);
                let ref mut fresh81 = (*(offspl.list).offset(i as isize)).size;
                *fresh81 = bz.size;
                (*(tmpspl.list).offset(i as isize)).size = *fresh81;
                let ref mut fresh82 = (*(offspl.list).offset(i as isize)).list;
                *fresh82 = malloc(
                    (::std::mem::size_of::<pointf>() as libc::c_ulong)
                        .wrapping_mul(bz.size as libc::c_ulong),
                ) as *mut pointf;
                offlist = *fresh82;
                let ref mut fresh83 = (*(tmpspl.list).offset(i as isize)).list;
                *fresh83 = malloc(
                    (::std::mem::size_of::<pointf>() as libc::c_ulong)
                        .wrapping_mul(bz.size as libc::c_ulong),
                ) as *mut pointf;
                tmplist = *fresh83;
                pf3 = *(bz.list).offset(0 as libc::c_int as isize);
                j = 0 as libc::c_int;
                while j < bz.size - 1 as libc::c_int {
                    pf0 = pf3;
                    pf1 = *(bz.list).offset((j + 1 as libc::c_int) as isize);
                    if j == 0 as libc::c_int {
                        *offlist.offset(j as isize) = computeoffset_p(pf0, pf1, 2.0f64);
                    } else {
                        *offlist.offset(j as isize) = computeoffset_p(pf2, pf1, 2.0f64);
                    }
                    pf2 = *(bz.list).offset((j + 2 as libc::c_int) as isize);
                    pf3 = *(bz.list).offset((j + 3 as libc::c_int) as isize);
                    let ref mut fresh84 = *offlist.offset((j + 2 as libc::c_int) as isize);
                    *fresh84 = computeoffset_qr(pf0, pf1, pf2, pf3, 2.0f64);
                    *offlist.offset((j + 1 as libc::c_int) as isize) = *fresh84;
                    (*tmplist.offset(j as isize)).x =
                        pf0.x - numc2 * (*offlist.offset(j as isize)).x;
                    (*tmplist.offset(j as isize)).y =
                        pf0.y - numc2 * (*offlist.offset(j as isize)).y;
                    (*tmplist.offset((j + 1 as libc::c_int) as isize)).x =
                        pf1.x - numc2 * (*offlist.offset((j + 1 as libc::c_int) as isize)).x;
                    (*tmplist.offset((j + 1 as libc::c_int) as isize)).y =
                        pf1.y - numc2 * (*offlist.offset((j + 1 as libc::c_int) as isize)).y;
                    (*tmplist.offset((j + 2 as libc::c_int) as isize)).x =
                        pf2.x - numc2 * (*offlist.offset((j + 2 as libc::c_int) as isize)).x;
                    (*tmplist.offset((j + 2 as libc::c_int) as isize)).y =
                        pf2.y - numc2 * (*offlist.offset((j + 2 as libc::c_int) as isize)).y;
                    j += 3 as libc::c_int;
                }
                *offlist.offset(j as isize) = computeoffset_p(pf2, pf3, 2.0f64);
                (*tmplist.offset(j as isize)).x = pf3.x - numc2 * (*offlist.offset(j as isize)).x;
                (*tmplist.offset(j as isize)).y = pf3.y - numc2 * (*offlist.offset(j as isize)).y;
                i += 1;
            }
            tailcolor = color;
            headcolor = tailcolor;
            lastcolor = headcolor;
            colors = strdup(color);
            cnum = 0 as libc::c_int;
            color = strtok(colors, b":\0" as *const u8 as *const libc::c_char);
            while !color.is_null() {
                if *color.offset(0 as libc::c_int as isize) == 0 {
                    color = b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                }
                if color != lastcolor {
                    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state
                        as libc::c_int
                        & ((1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int)
                        == 0
                    {
                        gvrender_set_pencolor(job, color);
                        gvrender_set_fillcolor(job, color);
                    }
                    lastcolor = color;
                }
                if cnum == 0 as libc::c_int {
                    tailcolor = color;
                    headcolor = tailcolor;
                }
                if cnum == 1 as libc::c_int {
                    tailcolor = color;
                }
                i = 0 as libc::c_int;
                while i < tmpspl.size {
                    tmplist = (*(tmpspl.list).offset(i as isize)).list;
                    offlist = (*(offspl.list).offset(i as isize)).list;
                    j = 0 as libc::c_int;
                    while j < (*(tmpspl.list).offset(i as isize)).size {
                        (*tmplist.offset(j as isize)).x += (*offlist.offset(j as isize)).x;
                        (*tmplist.offset(j as isize)).y += (*offlist.offset(j as isize)).y;
                        j += 1;
                    }
                    gvrender_beziercurve(
                        job,
                        tmplist,
                        (*(tmpspl.list).offset(i as isize)).size,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    i += 1;
                }
                cnum += 1;
                color = strtok(
                    0 as *mut libc::c_char,
                    b":\0" as *const u8 as *const libc::c_char,
                );
            }
            if bz.sflag != 0 {
                if color != tailcolor {
                    color = tailcolor;
                    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state
                        as libc::c_int
                        & ((1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int)
                        == 0
                    {
                        gvrender_set_pencolor(job, color);
                        gvrender_set_fillcolor(job, color);
                    }
                }
                arrow_gen(
                    job,
                    EMIT_TDRAW,
                    bz.sp,
                    *(bz.list).offset(0 as libc::c_int as isize),
                    arrowsize,
                    penwidth,
                    bz.sflag,
                );
            }
            if bz.eflag != 0 {
                if color != headcolor {
                    color = headcolor;
                    if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state
                        as libc::c_int
                        & ((1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int)
                        == 0
                    {
                        gvrender_set_pencolor(job, color);
                        gvrender_set_fillcolor(job, color);
                    }
                }
                arrow_gen(
                    job,
                    EMIT_HDRAW,
                    bz.ep,
                    *(bz.list).offset((bz.size - 1 as libc::c_int) as isize),
                    arrowsize,
                    penwidth,
                    bz.eflag,
                );
            }
            free(colors as *mut libc::c_void);
            i = 0 as libc::c_int;
            while i < offspl.size {
                free((*(offspl.list).offset(i as isize)).list as *mut libc::c_void);
                free((*(tmpspl.list).offset(i as isize)).list as *mut libc::c_void);
                i += 1;
            }
            free(offspl.list as *mut libc::c_void);
            free(tmpspl.list as *mut libc::c_void);
        } else {
            if (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).gui_state as libc::c_int
                & ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
                == 0
            {
                if *color.offset(0 as libc::c_int as isize) != 0 {
                    gvrender_set_pencolor(job, color);
                    gvrender_set_fillcolor(job, fillcolor);
                } else {
                    gvrender_set_pencolor(
                        job,
                        b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    if *fillcolor.offset(0 as libc::c_int as isize) != 0 {
                        gvrender_set_fillcolor(job, fillcolor);
                    } else {
                        gvrender_set_fillcolor(
                            job,
                            b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                }
            }
            i = 0 as libc::c_int;
            while i < (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size {
                bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
                    .offset(i as isize);
                if (*job).flags & (1 as libc::c_int) << 14 as libc::c_int != 0 {
                    gvrender_beziercurve(
                        job,
                        bz.list,
                        bz.size,
                        bz.sflag,
                        bz.eflag,
                        0 as libc::c_int,
                    );
                } else {
                    gvrender_beziercurve(
                        job,
                        bz.list,
                        bz.size,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    if bz.sflag != 0 {
                        arrow_gen(
                            job,
                            EMIT_TDRAW,
                            bz.sp,
                            *(bz.list).offset(0 as libc::c_int as isize),
                            arrowsize,
                            penwidth,
                            bz.sflag,
                        );
                    }
                    if bz.eflag != 0 {
                        arrow_gen(
                            job,
                            EMIT_HDRAW,
                            bz.ep,
                            *(bz.list).offset((bz.size - 1 as libc::c_int) as isize),
                            arrowsize,
                            penwidth,
                            bz.eflag,
                        );
                    }
                    if (*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size
                        > 1 as libc::c_int
                        && (bz.sflag != 0 || bz.eflag != 0)
                        && !styles.is_null()
                    {
                        gvrender_set_style(job, styles);
                    }
                }
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn edge_in_box(mut e: *mut edge_t, mut b: boxf) -> bool {
    let mut spl: *mut splines = 0 as *mut splines;
    let mut lp: *mut textlabel_t = 0 as *mut textlabel_t;
    spl = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
    if !spl.is_null() && boxf_overlap((*spl).bb, b) != 0 {
        return 1 as libc::c_int != 0;
    }
    lp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
    if !lp.is_null() && overlap_label(lp, b) as libc::c_int != 0 {
        return 1 as libc::c_int != 0;
    }
    lp = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel;
    if !lp.is_null() && (*lp).set as libc::c_int != 0 && overlap_label(lp, b) as libc::c_int != 0 {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn emit_begin_edge(
    mut job: *mut GVJ_t,
    mut e: *mut edge_t,
    mut styles: *mut *mut libc::c_char,
) {
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    let mut flags: libc::c_int = (*job).flags;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lab: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut tlab: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut hlab: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut pbs: *mut pointf = 0 as *mut pointf;
    let mut i: libc::c_int = 0;
    let mut nump: libc::c_int = 0;
    let mut pbs_n: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pbs_poly_n: libc::c_int = 0 as libc::c_int;
    let mut dflt_url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dflt_target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut penwidth: libc::c_double = 0.;
    obj = push_obj_state(job);
    (*obj).type_0 = EDGE_OBJTYPE;
    let ref mut fresh85 = (*obj).u.e;
    *fresh85 = e;
    (*obj).emit_state = EMIT_EDRAW;
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).is_null()
        && !(*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label).html
        && mapBool(
            agget(
                e as *mut libc::c_void,
                b"labelaligned\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            0 as libc::c_int != 0,
        ) as libc::c_int
            != 0
    {
        (*obj).set_labeledgealigned(1 as libc::c_int as libc::c_uint);
    }
    if !styles.is_null() && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
        gvrender_set_style(job, styles);
    }
    if !E_penwidth.is_null()
        && {
            s = agxget(e as *mut libc::c_void, E_penwidth);
            !s.is_null()
        }
        && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        penwidth = late_double(e as *mut libc::c_void, E_penwidth, 1.0f64, 0.0f64);
        gvrender_set_penwidth(job, penwidth);
    }
    if flags & (1 as libc::c_int) << 24 as libc::c_int != 0 {
        if (*((*(agraphof(
            (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
            .node as *mut libc::c_void,
        ) as *mut Agobj_t))
            .data as *mut Agraphinfo_t))
            .odim as libc::c_int
            >= 3 as libc::c_int
        {
            (*obj).tail_z = (if *((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(2 as libc::c_int as isize)
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .pos)
                    .offset(2 as libc::c_int as isize)
                    * 72 as libc::c_int as libc::c_double
                    + 0.5f64) as libc::c_int
            } else {
                (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 3 as libc::c_int
                {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .pos)
                    .offset(2 as libc::c_int as isize)
                    * 72 as libc::c_int as libc::c_double
                    - 0.5f64) as libc::c_int
            }) as libc::c_double;
            (*obj).head_z = (if *((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
            .node as *mut Agobj_t))
                .data as *mut Agnodeinfo_t))
                .pos)
                .offset(2 as libc::c_int as isize)
                * 72 as libc::c_int as libc::c_double
                >= 0 as libc::c_int as libc::c_double
            {
                (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .pos)
                    .offset(2 as libc::c_int as isize)
                    * 72 as libc::c_int as libc::c_double
                    + 0.5f64) as libc::c_int
            } else {
                (*((*((*((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                .node as *mut Agobj_t))
                    .data as *mut Agnodeinfo_t))
                    .pos)
                    .offset(2 as libc::c_int as isize)
                    * 72 as libc::c_int as libc::c_double
                    - 0.5f64) as libc::c_int
            }) as libc::c_double;
        } else {
            let ref mut fresh86 = (*obj).head_z;
            *fresh86 = 0.0f64;
            (*obj).tail_z = *fresh86;
        }
    }
    if flags & (1 as libc::c_int) << 15 as libc::c_int != 0 {
        lab = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label;
        if !lab.is_null() {
            let ref mut fresh87 = (*obj).label;
            *fresh87 = (*lab).text;
        }
        let ref mut fresh88 = (*obj).xlabel;
        *fresh88 = (*obj).label;
        let ref mut fresh89 = (*obj).headlabel;
        *fresh89 = *fresh88;
        let ref mut fresh90 = (*obj).taillabel;
        *fresh90 = *fresh89;
        tlab = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel;
        if !tlab.is_null() {
            let ref mut fresh91 = (*obj).xlabel;
            *fresh91 = (*tlab).text;
        }
        tlab = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label;
        if !tlab.is_null() {
            let ref mut fresh92 = (*obj).taillabel;
            *fresh92 = (*tlab).text;
        }
        hlab = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label;
        if !hlab.is_null() {
            let ref mut fresh93 = (*obj).headlabel;
            *fresh93 = (*hlab).text;
        }
    }
    if flags & (1 as libc::c_int) << 16 as libc::c_int != 0 {
        let mut xb: agxbuf = agxbuf {
            buf: 0 as *mut libc::c_char,
            ptr: 0 as *mut libc::c_char,
            eptr: 0 as *mut libc::c_char,
            dyna: 0,
        };
        let mut xbuf: [libc::c_char; 128] = [0; 128];
        agxbinit(
            &mut xb,
            128 as libc::c_int as libc::c_uint,
            xbuf.as_mut_ptr(),
        );
        s = getObjId(job, e as *mut libc::c_void, &mut xb);
        let ref mut fresh94 = (*obj).id;
        *fresh94 = strdup_and_subst_obj(s, e as *mut libc::c_void);
        agxbfree(&mut xb);
        s = agget(
            e as *mut libc::c_void,
            b"href\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 || {
            s = agget(
                e as *mut libc::c_void,
                b"URL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            dflt_url = strdup_and_subst_obj(s, e as *mut libc::c_void);
        }
        s = agget(
            e as *mut libc::c_void,
            b"edgehref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 || {
            s = agget(
                e as *mut libc::c_void,
                b"edgeURL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            let ref mut fresh95 = (*obj).url;
            *fresh95 = strdup_and_subst_obj(s, e as *mut libc::c_void);
        } else if !dflt_url.is_null() {
            let ref mut fresh96 = (*obj).url;
            *fresh96 = strdup(dflt_url);
        }
        s = agget(
            e as *mut libc::c_void,
            b"labelhref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 || {
            s = agget(
                e as *mut libc::c_void,
                b"labelURL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            let ref mut fresh97 = (*obj).labelurl;
            *fresh97 = strdup_and_subst_obj(s, e as *mut libc::c_void);
        } else if !dflt_url.is_null() {
            let ref mut fresh98 = (*obj).labelurl;
            *fresh98 = strdup(dflt_url);
        }
        s = agget(
            e as *mut libc::c_void,
            b"tailhref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 || {
            s = agget(
                e as *mut libc::c_void,
                b"tailURL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            let ref mut fresh99 = (*obj).tailurl;
            *fresh99 = strdup_and_subst_obj(s, e as *mut libc::c_void);
            (*obj).set_explicit_tailurl(1 as libc::c_int as libc::c_uint);
        } else if !dflt_url.is_null() {
            let ref mut fresh100 = (*obj).tailurl;
            *fresh100 = strdup(dflt_url);
        }
        s = agget(
            e as *mut libc::c_void,
            b"headhref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 || {
            s = agget(
                e as *mut libc::c_void,
                b"headURL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            let ref mut fresh101 = (*obj).headurl;
            *fresh101 = strdup_and_subst_obj(s, e as *mut libc::c_void);
            (*obj).set_explicit_headurl(1 as libc::c_int as libc::c_uint);
        } else if !dflt_url.is_null() {
            let ref mut fresh102 = (*obj).headurl;
            *fresh102 = strdup(dflt_url);
        }
    }
    if flags & (1 as libc::c_int) << 23 as libc::c_int != 0 {
        s = agget(
            e as *mut libc::c_void,
            b"target\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            dflt_target = strdup_and_subst_obj(s, e as *mut libc::c_void);
        }
        s = agget(
            e as *mut libc::c_void,
            b"edgetarget\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            (*obj).set_explicit_edgetarget(1 as libc::c_int as libc::c_uint);
            let ref mut fresh103 = (*obj).target;
            *fresh103 = strdup_and_subst_obj(s, e as *mut libc::c_void);
        } else if !dflt_target.is_null() {
            let ref mut fresh104 = (*obj).target;
            *fresh104 = strdup(dflt_target);
        }
        s = agget(
            e as *mut libc::c_void,
            b"labeltarget\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            let ref mut fresh105 = (*obj).labeltarget;
            *fresh105 = strdup_and_subst_obj(s, e as *mut libc::c_void);
        } else if !dflt_target.is_null() {
            let ref mut fresh106 = (*obj).labeltarget;
            *fresh106 = strdup(dflt_target);
        }
        s = agget(
            e as *mut libc::c_void,
            b"tailtarget\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            let ref mut fresh107 = (*obj).tailtarget;
            *fresh107 = strdup_and_subst_obj(s, e as *mut libc::c_void);
            (*obj).set_explicit_tailtarget(1 as libc::c_int as libc::c_uint);
        } else if !dflt_target.is_null() {
            let ref mut fresh108 = (*obj).tailtarget;
            *fresh108 = strdup(dflt_target);
        }
        s = agget(
            e as *mut libc::c_void,
            b"headtarget\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            (*obj).set_explicit_headtarget(1 as libc::c_int as libc::c_uint);
            let ref mut fresh109 = (*obj).headtarget;
            *fresh109 = strdup_and_subst_obj(s, e as *mut libc::c_void);
        } else if !dflt_target.is_null() {
            let ref mut fresh110 = (*obj).headtarget;
            *fresh110 = strdup(dflt_target);
        }
    }
    if flags & (1 as libc::c_int) << 22 as libc::c_int != 0 {
        s = agget(
            e as *mut libc::c_void,
            b"tooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 || {
            s = agget(
                e as *mut libc::c_void,
                b"edgetooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
        } {
            let mut tooltip: *mut libc::c_char = preprocessTooltip(s, e as *mut libc::c_void);
            let ref mut fresh111 = (*obj).tooltip;
            *fresh111 = strdup_and_subst_obj(tooltip, e as *mut libc::c_void);
            free(tooltip as *mut libc::c_void);
            (*obj).set_explicit_tooltip(1 as libc::c_int as libc::c_uint);
        } else if !((*obj).label).is_null() {
            let ref mut fresh112 = (*obj).tooltip;
            *fresh112 = strdup((*obj).label);
        }
        s = agget(
            e as *mut libc::c_void,
            b"labeltooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            let mut tooltip_0: *mut libc::c_char = preprocessTooltip(s, e as *mut libc::c_void);
            let ref mut fresh113 = (*obj).labeltooltip;
            *fresh113 = strdup_and_subst_obj(tooltip_0, e as *mut libc::c_void);
            free(tooltip_0 as *mut libc::c_void);
            (*obj).set_explicit_labeltooltip(1 as libc::c_int as libc::c_uint);
        } else if !((*obj).label).is_null() {
            let ref mut fresh114 = (*obj).labeltooltip;
            *fresh114 = strdup((*obj).label);
        }
        s = agget(
            e as *mut libc::c_void,
            b"tailtooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            let mut tooltip_1: *mut libc::c_char = preprocessTooltip(s, e as *mut libc::c_void);
            let ref mut fresh115 = (*obj).tailtooltip;
            *fresh115 = strdup_and_subst_obj(tooltip_1, e as *mut libc::c_void);
            free(tooltip_1 as *mut libc::c_void);
            (*obj).set_explicit_tailtooltip(1 as libc::c_int as libc::c_uint);
        } else if !((*obj).taillabel).is_null() {
            let ref mut fresh116 = (*obj).tailtooltip;
            *fresh116 = strdup((*obj).taillabel);
        }
        s = agget(
            e as *mut libc::c_void,
            b"headtooltip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            let mut tooltip_2: *mut libc::c_char = preprocessTooltip(s, e as *mut libc::c_void);
            let ref mut fresh117 = (*obj).headtooltip;
            *fresh117 = strdup_and_subst_obj(tooltip_2, e as *mut libc::c_void);
            free(tooltip_2 as *mut libc::c_void);
            (*obj).set_explicit_headtooltip(1 as libc::c_int as libc::c_uint);
        } else if !((*obj).headlabel).is_null() {
            let ref mut fresh118 = (*obj).headtooltip;
            *fresh118 = strdup((*obj).headlabel);
        }
    }
    free(dflt_url as *mut libc::c_void);
    free(dflt_target as *mut libc::c_void);
    if flags & ((1 as libc::c_int) << 16 as libc::c_int | (1 as libc::c_int) << 22 as libc::c_int)
        != 0
    {
        if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
            && (!((*obj).url).is_null() || !((*obj).tooltip).is_null())
            && flags & (1 as libc::c_int) << 19 as libc::c_int != 0
        {
            let mut ns: libc::c_int = 0;
            let mut spl: *mut splines = 0 as *mut splines;
            let mut w2: libc::c_double = if (*(*job).obj).penwidth / 2.0f64 > 2.0f64 {
                (*(*job).obj).penwidth / 2.0f64
            } else {
                2.0f64
            };
            spl = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
            ns = (*spl).size;
            i = 0 as libc::c_int;
            while i < ns {
                map_output_bspline(
                    &mut pbs,
                    &mut pbs_n,
                    &mut pbs_poly_n,
                    ((*spl).list).offset(i as isize),
                    w2,
                );
                i += 1;
            }
            (*obj).url_bsplinemap_poly_n = pbs_poly_n;
            let ref mut fresh119 = (*obj).url_bsplinemap_n;
            *fresh119 = pbs_n;
            if flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
                nump = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < pbs_poly_n {
                    nump += *pbs_n.offset(i as isize);
                    i += 1;
                }
                gvrender_ptf_A(job, pbs, pbs, nump);
            }
            let ref mut fresh120 = (*obj).url_bsplinemap_p;
            *fresh120 = pbs;
            (*obj).url_map_shape = MAP_POLYGON;
            let ref mut fresh121 = (*obj).url_map_p;
            *fresh121 = pbs;
            (*obj).url_map_n = *pbs_n.offset(0 as libc::c_int as isize);
        }
    }
    gvrender_begin_edge(job, e);
    if !((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0 {
        gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
    }
}
unsafe extern "C" fn emit_edge_label(
    mut job: *mut GVJ_t,
    mut lbl: *mut textlabel_t,
    mut lkind: emit_state_t,
    mut explicit: libc::c_int,
    mut url: *mut libc::c_char,
    mut tooltip: *mut libc::c_char,
    mut target: *mut libc::c_char,
    mut id: *mut libc::c_char,
    mut spl: *mut splines,
) {
    let mut flags: libc::c_int = (*job).flags;
    let mut old_emit_state: emit_state_t = EMIT_GDRAW;
    let mut newid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if lbl.is_null() || !(*lbl).set {
        return;
    }
    if !id.is_null() {
        newid = gcalloc(
            (strlen(id)).wrapping_add(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong),
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        match lkind as libc::c_uint {
            11 => {
                type_0 = b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            7 => {
                type_0 = b"headlabel\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            6 => {
                type_0 = b"taillabel\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"emit.c\0" as *const u8 as *const libc::c_char,
                    2691 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 107],
                        &[libc::c_char; 107],
                    >(
                        b"void emit_edge_label(GVJ_t *, textlabel_t *, emit_state_t, int, char *, char *, char *, char *, splines *)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        sprintf(
            newid,
            b"%s-%s\0" as *const u8 as *const libc::c_char,
            id,
            type_0,
        );
    } else {
        newid = 0 as *mut libc::c_char;
    }
    old_emit_state = (*(*job).obj).emit_state;
    (*(*job).obj).emit_state = lkind;
    if (!url.is_null() || explicit != 0) && flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        map_label(job, lbl);
        gvrender_begin_anchor(job, url, tooltip, target, newid);
    }
    emit_label(job, lkind, lbl);
    if !spl.is_null() {
        emit_attachment(job, lbl, spl);
    }
    if !url.is_null() || explicit != 0 {
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            map_label(job, lbl);
            gvrender_begin_anchor(job, url, tooltip, target, newid);
        }
        gvrender_end_anchor(job);
    }
    free(newid as *mut libc::c_void);
    (*(*job).obj).emit_state = old_emit_state;
}
unsafe extern "C" fn nodeIntersect(
    mut job: *mut GVJ_t,
    mut p: pointf,
    mut explicit_iurl: bool,
    mut iurl: *mut libc::c_char,
    mut explicit_itooltip: bool,
) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut explicit: bool = false;
    if explicit_iurl {
        url = iurl;
    } else {
        url = (*obj).url;
    }
    if explicit_itooltip {
        explicit = 1 as libc::c_int != 0;
    } else if (*obj).explicit_tooltip() != 0 {
        explicit = 1 as libc::c_int != 0;
    } else {
        explicit = 0 as libc::c_int != 0;
    }
    if !url.is_null() || explicit as libc::c_int != 0 {
        map_point(job, p);
    }
}
unsafe extern "C" fn emit_end_edge(mut job: *mut GVJ_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut e: *mut edge_t = (*obj).u.e;
    let mut i: libc::c_int = 0;
    let mut nump: libc::c_int = 0;
    if !((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0 {
        gvrender_end_anchor(job);
        if (*obj).url_bsplinemap_poly_n != 0 {
            nump = *((*obj).url_bsplinemap_n).offset(0 as libc::c_int as isize);
            i = 1 as libc::c_int;
            while i < (*obj).url_bsplinemap_poly_n {
                (*obj).url_map_n = *((*obj).url_bsplinemap_n).offset(i as isize);
                let ref mut fresh122 = (*obj).url_map_p;
                *fresh122 = &mut *((*obj).url_bsplinemap_p).offset(nump as isize) as *mut pointf;
                gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
                gvrender_end_anchor(job);
                nump += *((*obj).url_bsplinemap_n).offset(i as isize);
                i += 1;
            }
        }
    }
    (*obj).url_map_n = 0 as libc::c_int;
    let ref mut fresh123 = (*obj).url_map_p;
    *fresh123 = 0 as *mut pointf;
    if !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null() {
        let mut p: pointf = pointf { x: 0., y: 0. };
        let mut bz: bezier = bezier {
            list: 0 as *mut pointf,
            size: 0,
            sflag: 0,
            eflag: 0,
            sp: pointf { x: 0., y: 0. },
            ep: pointf { x: 0., y: 0. },
        };
        bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list)
            .offset(0 as libc::c_int as isize);
        if bz.sflag != 0 {
            p = bz.sp;
        } else {
            p = *(bz.list).offset(0 as libc::c_int as isize);
        }
        nodeIntersect(
            job,
            p,
            (*obj).explicit_tailurl() as libc::c_int != 0 as libc::c_int,
            (*obj).tailurl,
            (*obj).explicit_tailtooltip() as libc::c_int != 0 as libc::c_int,
        );
        bz = *((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).list).offset(
            ((*(*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).size - 1 as libc::c_int)
                as isize,
        );
        if bz.eflag != 0 {
            p = bz.ep;
        } else {
            p = *(bz.list).offset((bz.size - 1 as libc::c_int) as isize);
        }
        nodeIntersect(
            job,
            p,
            (*obj).explicit_headurl() as libc::c_int != 0 as libc::c_int,
            (*obj).headurl,
            (*obj).explicit_headtooltip() as libc::c_int != 0 as libc::c_int,
        );
    }
    emit_edge_label(
        job,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).label,
        EMIT_ELABEL,
        (*obj).explicit_labeltooltip() as libc::c_int,
        (*obj).labelurl,
        (*obj).labeltooltip,
        (*obj).labeltarget,
        (*obj).id,
        if mapbool(late_string(
            e as *mut libc::c_void,
            E_decorate,
            b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )) as libc::c_int
            != 0
            && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
        {
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl
        } else {
            0 as *mut splines
        },
    );
    emit_edge_label(
        job,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).xlabel,
        EMIT_ELABEL,
        (*obj).explicit_labeltooltip() as libc::c_int,
        (*obj).labelurl,
        (*obj).labeltooltip,
        (*obj).labeltarget,
        (*obj).id,
        if mapbool(late_string(
            e as *mut libc::c_void,
            E_decorate,
            b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )) as libc::c_int
            != 0
            && !((*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl).is_null()
        {
            (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl
        } else {
            0 as *mut splines
        },
    );
    emit_edge_label(
        job,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).head_label,
        EMIT_HLABEL,
        (*obj).explicit_headtooltip() as libc::c_int,
        (*obj).headurl,
        (*obj).headtooltip,
        (*obj).headtarget,
        (*obj).id,
        0 as *mut splines,
    );
    emit_edge_label(
        job,
        (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).tail_label,
        EMIT_TLABEL,
        (*obj).explicit_tailtooltip() as libc::c_int,
        (*obj).tailurl,
        (*obj).tailtooltip,
        (*obj).tailtarget,
        (*obj).id,
        0 as *mut splines,
    );
    gvrender_end_edge(job);
    pop_obj_state(job);
}
unsafe extern "C" fn emit_edge(mut job: *mut GVJ_t, mut e: *mut edge_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut style: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut styles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut sp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if edge_in_box(e, (*job).clip) as libc::c_int != 0 && edge_in_layer(job, e) as libc::c_int != 0
    {
        s = malloc(
            (strlen(agnameof(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                }))
                .node as *mut libc::c_void,
            )))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(agnameof(
                (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                .node as *mut libc::c_void,
            )))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(
            s,
            agnameof(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                    e
                } else {
                    e.offset(1 as libc::c_int as isize)
                })
                .node as *mut libc::c_void,
            ),
        );
        if agisdirected(agraphof(
            (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
            .node as *mut libc::c_void,
        )) != 0
        {
            strcat(s, b"->\0" as *const u8 as *const libc::c_char);
        } else {
            strcat(s, b"--\0" as *const u8 as *const libc::c_char);
        }
        strcat(
            s,
            agnameof(
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                .node as *mut libc::c_void,
            ),
        );
        gvrender_comment(job, s);
        free(s as *mut libc::c_void);
        s = late_string(
            e as *mut libc::c_void,
            E_comment,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *s.offset(0 as libc::c_int as isize) != 0 {
            gvrender_comment(job, s);
        }
        style = late_string(
            e as *mut libc::c_void,
            E_style,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *style.offset(0 as libc::c_int as isize) != 0 {
            styles = parse_style(style);
            sp = styles;
            loop {
                let fresh124 = sp;
                sp = sp.offset(1);
                p = *fresh124;
                if p.is_null() {
                    break;
                }
                if strcmp(p, b"invis\0" as *const u8 as *const libc::c_char) == 0 {
                    return;
                }
            }
        }
        emit_begin_edge(job, e, styles);
        emit_edge_graphics(job, e, styles);
        emit_end_edge(job);
    }
}
static mut adjust: [libc::c_char; 3] = [
    'l' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
];
unsafe extern "C" fn expandBB(mut bb: *mut boxf, mut p: pointf) {
    (*bb).UR.x = fmax((*bb).UR.x, p.x);
    (*bb).LL.x = fmin((*bb).LL.x, p.x);
    (*bb).UR.y = fmax((*bb).UR.y, p.y);
    (*bb).LL.y = fmin((*bb).LL.y, p.y);
}
unsafe extern "C" fn ptsBB(
    mut inpts: *mut xdot_point,
    mut numpts: libc::c_int,
    mut bb: *mut boxf,
) -> boxf {
    let mut opbb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut i: libc::c_int = 0;
    opbb.UR.x = (*inpts).x;
    opbb.LL.x = opbb.UR.x;
    opbb.UR.y = (*inpts).y;
    opbb.LL.y = opbb.UR.y;
    i = 1 as libc::c_int;
    while i < numpts {
        inpts = inpts.offset(1);
        if (*inpts).x < opbb.LL.x {
            opbb.LL.x = (*inpts).x;
        } else if (*inpts).x > opbb.UR.x {
            opbb.UR.x = (*inpts).x;
        }
        if (*inpts).y < opbb.LL.y {
            opbb.LL.y = (*inpts).y;
        } else if (*inpts).y > opbb.UR.y {
            opbb.UR.y = (*inpts).y;
        }
        i += 1;
    }
    expandBB(bb, opbb.LL);
    expandBB(bb, opbb.UR);
    return opbb;
}
unsafe extern "C" fn textBB(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut span: *mut textspan_t,
) -> boxf {
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut sz: pointf = (*span).size;
    match (*span).just as libc::c_int {
        108 => {
            bb.LL.x = x;
            bb.UR.x = bb.LL.x + sz.x;
        }
        110 => {
            bb.LL.x = x - sz.x / 2.0f64;
            bb.UR.x = x + sz.x / 2.0f64;
        }
        114 => {
            bb.UR.x = x;
            bb.LL.x = bb.UR.x - sz.x;
        }
        _ => {}
    }
    bb.UR.y = y + (*span).yoffset_layout;
    bb.LL.y = bb.UR.y - sz.y;
    return bb;
}
unsafe extern "C" fn freePara(mut op: *mut exdot_op) {
    if (*op).op.kind as libc::c_uint == xd_text as libc::c_int as libc::c_uint {
        free_textspan((*op).span, 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xdotBB(mut g: *mut Agraph_t) -> boxf {
    let mut gvc: *mut GVC_t = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).gvc;
    let mut op: *mut exdot_op = 0 as *mut exdot_op;
    let mut i: libc::c_int = 0;
    let mut fontsize: libc::c_double = 0.0f64;
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pts: [pointf; 2] = [pointf { x: 0., y: 0. }; 2];
    let mut bb0: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut bb: boxf = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb;
    let mut xd: *mut xdot =
        (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).xdots as *mut xdot;
    let mut tf: textfont_t = textfont_t {
        name: 0 as *mut libc::c_char,
        color: 0 as *mut libc::c_char,
        postscript_alias: 0 as *mut PostscriptAlias,
        size: 0.,
        flags_cnt: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut null_tf: textfont_t = {
        let mut init = textfont_t {
            flags_cnt: [0; 4],
            c2rust_padding: [0; 4],
            name: 0 as *mut libc::c_char,
            color: 0 as *mut libc::c_char,
            postscript_alias: 0 as *mut PostscriptAlias,
            size: 0.0f64,
        };
        init.set_flags(0 as libc::c_int as libc::c_uint);
        init.set_cnt(0 as libc::c_int as libc::c_uint);
        init
    };
    let mut fontflags: libc::c_int = 0 as libc::c_int;
    if xd.is_null() {
        return bb;
    }
    if bb.LL.x == bb.UR.x && bb.LL.y == bb.UR.y {
        bb.LL.y = 1.7976931348623157e+308f64;
        bb.LL.x = bb.LL.y;
        bb.UR.y = -1.7976931348623157e+308f64;
        bb.UR.x = bb.UR.y;
    }
    op = (*xd).ops as *mut exdot_op;
    i = 0 as libc::c_int;
    while i < (*xd).cnt {
        tf = null_tf;
        match (*op).op.kind as libc::c_uint {
            0 | 1 => {
                pts[0 as libc::c_int as usize].x = (*op).op.u.ellipse.x - (*op).op.u.ellipse.w;
                pts[0 as libc::c_int as usize].y = (*op).op.u.ellipse.y - (*op).op.u.ellipse.h;
                pts[1 as libc::c_int as usize].x = (*op).op.u.ellipse.x + (*op).op.u.ellipse.w;
                pts[1 as libc::c_int as usize].y = (*op).op.u.ellipse.y + (*op).op.u.ellipse.h;
                (*op).bb.LL = pts[0 as libc::c_int as usize];
                (*op).bb.UR = pts[1 as libc::c_int as usize];
                expandBB(&mut bb, pts[0 as libc::c_int as usize]);
                expandBB(&mut bb, pts[1 as libc::c_int as usize]);
            }
            2 | 3 => {
                (*op).bb = ptsBB((*op).op.u.polygon.pts, (*op).op.u.polygon.cnt, &mut bb);
            }
            4 | 5 => {
                (*op).bb = ptsBB((*op).op.u.polygon.pts, (*op).op.u.polygon.cnt, &mut bb);
            }
            6 => {
                (*op).bb = ptsBB((*op).op.u.polygon.pts, (*op).op.u.polygon.cnt, &mut bb);
            }
            7 => {
                let ref mut fresh125 = (*op).span;
                *fresh125 = zmalloc(::std::mem::size_of::<textspan_t>() as libc::c_ulong)
                    as *mut textspan_t;
                let ref mut fresh126 = (*(*op).span).str_0;
                *fresh126 = strdup((*op).op.u.text.text);
                (*(*op).span).just = adjust[(*op).op.u.text.align as usize];
                tf.name = fontname;
                tf.size = fontsize;
                tf.set_flags(fontflags as libc::c_uint);
                let ref mut fresh127 = (*(*op).span).font;
                *fresh127 =
                    (Some(((*(*gvc).textfont_dt).searchf).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        (*gvc).textfont_dt,
                        &mut tf as *mut textfont_t as *mut libc::c_void,
                        0o1 as libc::c_int,
                    ) as *mut textfont_t;
                textspan_size(gvc, (*op).span);
                bb0 = textBB((*op).op.u.text.x, (*op).op.u.text.y, (*op).span);
                (*op).bb = bb0;
                expandBB(&mut bb, bb0.LL);
                expandBB(&mut bb, bb0.UR);
                if ((*xd).freefunc).is_none() {
                    let ref mut fresh128 = (*xd).freefunc;
                    *fresh128 = ::std::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut exdot_op) -> ()>,
                        freefunc_t,
                    >(Some(
                        freePara as unsafe extern "C" fn(*mut exdot_op) -> (),
                    ));
                }
            }
            10 => {
                fontsize = (*op).op.u.font.size;
                fontname = (*op).op.u.font.name;
            }
            15 => {
                fontflags = (*op).op.u.fontchar as libc::c_int;
            }
            _ => {}
        }
        op = op.offset(1);
        i += 1;
    }
    return bb;
}
unsafe extern "C" fn init_gvc(mut gvc: *mut GVC_t, mut g: *mut graph_t) {
    let mut xf: libc::c_double = 0.;
    let mut yf: libc::c_double = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let ref mut fresh129 = (*gvc).g;
    *fresh129 = g;
    (*gvc).graph_sets_margin = 0 as libc::c_int != 0;
    p = agget(
        g as *mut libc::c_void,
        b"margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        i = sscanf(
            p,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut xf as *mut libc::c_double,
            &mut yf as *mut libc::c_double,
        );
        if i > 0 as libc::c_int {
            let ref mut fresh130 = (*gvc).margin.y;
            *fresh130 = xf * 72 as libc::c_int as libc::c_double;
            (*gvc).margin.x = *fresh130;
            if i > 1 as libc::c_int {
                (*gvc).margin.y = yf * 72 as libc::c_int as libc::c_double;
            }
            (*gvc).graph_sets_margin = 1 as libc::c_int != 0;
        }
    }
    (*gvc).graph_sets_pad = 0 as libc::c_int != 0;
    p = agget(
        g as *mut libc::c_void,
        b"pad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        i = sscanf(
            p,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut xf as *mut libc::c_double,
            &mut yf as *mut libc::c_double,
        );
        if i > 0 as libc::c_int {
            let ref mut fresh131 = (*gvc).pad.y;
            *fresh131 = xf * 72 as libc::c_int as libc::c_double;
            (*gvc).pad.x = *fresh131;
            if i > 1 as libc::c_int {
                (*gvc).pad.y = yf * 72 as libc::c_int as libc::c_double;
            }
            (*gvc).graph_sets_pad = 1 as libc::c_int != 0;
        }
    }
    (*gvc).graph_sets_pageSize = 0 as libc::c_int != 0;
    (*gvc).pageSize = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).page;
    if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .page
        .x
        > 0.001f64
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
            .page
            .y
            > 0.001f64
    {
        (*gvc).graph_sets_pageSize = 1 as libc::c_int != 0;
    }
    if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).landscape {
        (*gvc).rotation = 90 as libc::c_int;
    } else {
        (*gvc).rotation = 0 as libc::c_int;
    }
    let ref mut fresh132 = (*gvc).pagedir;
    *fresh132 = b"BL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    p = agget(
        g as *mut libc::c_void,
        b"pagedir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        let ref mut fresh133 = (*gvc).pagedir;
        *fresh133 = p;
    }
    (*gvc).bb = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb;
    G_peripheries = agattr(
        g,
        0 as libc::c_int,
        b"peripheries\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    G_penwidth = agattr(
        g,
        0 as libc::c_int,
        b"penwidth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let ref mut fresh134 = (*gvc).defaultfontname;
    *fresh134 = late_nnstring(
        0 as *mut libc::c_void,
        N_fontname,
        b"Times-Roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*gvc).defaultfontsize = late_double(0 as *mut libc::c_void, N_fontsize, 14.0f64, 1.0f64);
    let ref mut fresh135 = (*gvc).defaultlinestyle;
    *fresh135 = defaultlinestyle.as_mut_ptr();
    let ref mut fresh136 = (*gvc).graphname;
    *fresh136 = agnameof(g as *mut libc::c_void);
}
unsafe extern "C" fn init_job_pad(mut job: *mut GVJ_t) {
    let mut gvc: *mut GVC_t = (*job).gvc;
    if (*gvc).graph_sets_pad {
        (*job).pad = (*gvc).pad;
    } else {
        match (*job).output_lang {
            300 => {
                let ref mut fresh137 = (*job).pad.y;
                *fresh137 = (*(*job).render.features).default_pad;
                (*job).pad.x = *fresh137;
            }
            _ => {
                let ref mut fresh138 = (*job).pad.y;
                *fresh138 = 4 as libc::c_int as libc::c_double;
                (*job).pad.x = *fresh138;
            }
        }
    };
}
unsafe extern "C" fn init_job_margin(mut job: *mut GVJ_t) {
    let mut gvc: *mut GVC_t = (*job).gvc;
    if (*gvc).graph_sets_margin {
        (*job).margin = (*gvc).margin;
    } else {
        match (*job).output_lang {
            300 => {
                (*job).margin = (*(*job).device.features).default_margin;
            }
            2 | 3 | 4 | 22 | 21 | 30 => {
                let ref mut fresh139 = (*job).margin.y;
                *fresh139 = 36 as libc::c_int as libc::c_double;
                (*job).margin.x = *fresh139;
            }
            _ => {
                let ref mut fresh140 = (*job).margin.y;
                *fresh140 = 0 as libc::c_int as libc::c_double;
                (*job).margin.x = *fresh140;
            }
        }
    };
}
unsafe extern "C" fn init_job_dpi(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut firstjob: *mut GVJ_t = (*(*job).gvc).active_jobs;
    if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).dpi
        != 0 as libc::c_int as libc::c_double
    {
        let ref mut fresh141 = (*job).dpi.y;
        *fresh141 = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).dpi;
        (*job).dpi.x = *fresh141;
    } else if !firstjob.is_null() && (*firstjob).device_sets_dpi as libc::c_int != 0 {
        (*job).dpi = (*firstjob).device_dpi;
    } else {
        match (*job).output_lang {
            300 => {
                (*job).dpi = (*(*job).device.features).default_dpi;
            }
            _ => {
                let ref mut fresh142 = (*job).dpi.y;
                *fresh142 = 96 as libc::c_int as libc::c_double;
                (*job).dpi.x = *fresh142;
            }
        }
    };
}
unsafe extern "C" fn init_job_viewport(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut gvc: *mut GVC_t = (*job).gvc;
    let mut LL: pointf = pointf { x: 0., y: 0. };
    let mut UR: pointf = pointf { x: 0., y: 0. };
    let mut size: pointf = pointf { x: 0., y: 0. };
    let mut sz: pointf = pointf { x: 0., y: 0. };
    let mut X: libc::c_double = 0.;
    let mut Y: libc::c_double = 0.;
    let mut Z: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut rv: libc::c_int = 0;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nodename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut junk: *mut libc::c_char = 0 as *mut libc::c_char;
    UR = (*gvc).bb.UR;
    LL = (*gvc).bb.LL;
    (*job).bb.LL.x = LL.x - (*job).pad.x;
    (*job).bb.LL.y = LL.y - (*job).pad.y;
    (*job).bb.UR.x = UR.x + (*job).pad.x;
    (*job).bb.UR.y = UR.y + (*job).pad.y;
    sz.x = (*job).bb.UR.x - (*job).bb.LL.x;
    sz.y = (*job).bb.UR.y - (*job).bb.LL.y;
    Z = 1.0f64;
    if (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
        .size
        .x
        > 0.001f64
        && (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing)
            .size
            .y
            > 0.001f64
    {
        size = (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).size;
        if sz.x == 0 as libc::c_int as libc::c_double {
            sz.x = size.x;
        }
        if sz.y == 0 as libc::c_int as libc::c_double {
            sz.y = size.y;
        }
        if size.x < sz.x
            || size.y < sz.y
            || (*(*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).filled
                as libc::c_int
                != 0
                && size.x > sz.x
                && size.y > sz.y
        {
            Z = if size.x / sz.x < size.y / sz.y {
                size.x / sz.x
            } else {
                size.y / sz.y
            };
        }
    }
    x = (LL.x + UR.x) / 2.0f64;
    y = (LL.y + UR.y) / 2.0f64;
    (*job).rotation = (*(*job).gvc).rotation;
    X = sz.x * Z;
    Y = sz.y * Z;
    str = agget(
        g as *mut libc::c_void,
        b"viewport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() {
        nodename = malloc((strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        junk = malloc((strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        rv = sscanf(
            str,
            b"%lf,%lf,%lf,'%[^']'\0" as *const u8 as *const libc::c_char,
            &mut X as *mut libc::c_double,
            &mut Y as *mut libc::c_double,
            &mut Z as *mut libc::c_double,
            nodename,
        );
        if rv == 4 as libc::c_int {
            n = agnode((*g).root, nodename, 0 as libc::c_int);
            if !n.is_null() {
                x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .x;
                y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                    .coord
                    .y;
            }
        } else {
            rv = sscanf(
                str,
                b"%lf,%lf,%lf,%[^,]%s\0" as *const u8 as *const libc::c_char,
                &mut X as *mut libc::c_double,
                &mut Y as *mut libc::c_double,
                &mut Z as *mut libc::c_double,
                nodename,
                junk,
            );
            if rv == 4 as libc::c_int {
                n = agnode((*g).root, nodename, 0 as libc::c_int);
                if !n.is_null() {
                    x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord
                        .x;
                    y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
                        .coord
                        .y;
                }
            } else {
                rv = sscanf(
                    str,
                    b"%lf,%lf,%lf,%lf,%lf\0" as *const u8 as *const libc::c_char,
                    &mut X as *mut libc::c_double,
                    &mut Y as *mut libc::c_double,
                    &mut Z as *mut libc::c_double,
                    &mut x as *mut libc::c_double,
                    &mut y as *mut libc::c_double,
                );
            }
        }
        free(nodename as *mut libc::c_void);
        free(junk as *mut libc::c_void);
    }
    (*job).view.x = X;
    (*job).view.y = Y;
    (*job).zoom = Z;
    (*job).focus.x = x;
    (*job).focus.y = y;
}
unsafe extern "C" fn emit_cluster_colors(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut sg: *mut graph_t = 0 as *mut graph_t;
    let mut c: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        sg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize);
        emit_cluster_colors(job, sg);
        str = agget(
            sg as *mut libc::c_void,
            b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_pencolor(job, str);
        }
        str = agget(
            sg as *mut libc::c_void,
            b"pencolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_pencolor(job, str);
        }
        str = agget(
            sg as *mut libc::c_void,
            b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_pencolor(job, str);
        }
        str = agget(
            sg as *mut libc::c_void,
            b"fillcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_fillcolor(job, str);
        }
        str = agget(
            sg as *mut libc::c_void,
            b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_pencolor(job, str);
        }
        c += 1;
    }
}
unsafe extern "C" fn emit_colors(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut colors: *mut libc::c_char = 0 as *mut libc::c_char;
    gvrender_set_fillcolor(
        job,
        b"lightgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    str = agget(
        g as *mut libc::c_void,
        b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        gvrender_set_fillcolor(job, str);
    }
    str = agget(
        g as *mut libc::c_void,
        b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        gvrender_set_pencolor(job, str);
    }
    emit_cluster_colors(job, g);
    n = agfstnode(g);
    while !n.is_null() {
        str = agget(
            n as *mut libc::c_void,
            b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_pencolor(job, str);
        }
        str = agget(
            n as *mut libc::c_void,
            b"pencolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_fillcolor(job, str);
        }
        str = agget(
            n as *mut libc::c_void,
            b"fillcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            if !(strchr(str, ':' as i32)).is_null() {
                colors = strdup(str);
                str = strtok(colors, b":\0" as *const u8 as *const libc::c_char);
                while !str.is_null() {
                    if *str.offset(0 as libc::c_int as isize) != 0 {
                        gvrender_set_pencolor(job, str);
                    }
                    str = strtok(
                        0 as *mut libc::c_char,
                        b":\0" as *const u8 as *const libc::c_char,
                    );
                }
                free(colors as *mut libc::c_void);
            } else {
                gvrender_set_pencolor(job, str);
            }
        }
        str = agget(
            n as *mut libc::c_void,
            b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            gvrender_set_pencolor(job, str);
        }
        e = agfstout(g, n);
        while !e.is_null() {
            str = agget(
                e as *mut libc::c_void,
                b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                if !(strchr(str, ':' as i32)).is_null() {
                    colors = strdup(str);
                    str = strtok(colors, b":\0" as *const u8 as *const libc::c_char);
                    while !str.is_null() {
                        if *str.offset(0 as libc::c_int as isize) != 0 {
                            gvrender_set_pencolor(job, str);
                        }
                        str = strtok(
                            0 as *mut libc::c_char,
                            b":\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    free(colors as *mut libc::c_void);
                } else {
                    gvrender_set_pencolor(job, str);
                }
            }
            str = agget(
                e as *mut libc::c_void,
                b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                gvrender_set_pencolor(job, str);
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn emit_view(mut job: *mut GVJ_t, mut g: *mut graph_t, mut flags: libc::c_int) {
    let mut gvc: *mut GVC_t = (*job).gvc;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let ref mut fresh143 = (*gvc).common.viewNum;
    *fresh143 += 1;
    if flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        emit_clusters(job, g, flags);
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        gvrender_begin_nodes(job);
        n = agfstnode(g);
        while !n.is_null() {
            emit_node(job, n);
            n = agnxtnode(g, n);
        }
        gvrender_end_nodes(job);
        gvrender_begin_edges(job);
        n = agfstnode(g);
        while !n.is_null() {
            e = agfstout(g, n);
            while !e.is_null() {
                emit_edge(job, e);
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
        gvrender_end_edges(job);
    } else if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        gvrender_begin_edges(job);
        n = agfstnode(g);
        while !n.is_null() {
            e = agfstout(g, n);
            while !e.is_null() {
                emit_edge(job, e);
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
        gvrender_end_edges(job);
        gvrender_begin_nodes(job);
        n = agfstnode(g);
        while !n.is_null() {
            emit_node(job, n);
            n = agnxtnode(g, n);
        }
        gvrender_end_nodes(job);
    } else if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        gvrender_begin_nodes(job);
        n = agfstnode(g);
        while !n.is_null() {
            if write_node_test(g, n) {
                emit_node(job, n);
            }
            n = agnxtnode(g, n);
        }
        gvrender_end_nodes(job);
        gvrender_begin_edges(job);
        n = agfstnode(g);
        while !n.is_null() {
            e = agfstout(g, n);
            while !e.is_null() {
                if write_edge_test(g, e) {
                    emit_edge(job, e);
                }
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
        gvrender_end_edges(job);
    } else {
        n = agfstnode(g);
        while !n.is_null() {
            emit_node(job, n);
            e = agfstout(g, n);
            while !e.is_null() {
                emit_node(
                    job,
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                        .node,
                );
                emit_edge(job, e);
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        emit_clusters(job, g, flags);
    }
}
unsafe extern "C" fn emit_begin_graph(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    obj = push_obj_state(job);
    (*obj).type_0 = ROOTGRAPH_OBJTYPE;
    let ref mut fresh144 = (*obj).u.g;
    *fresh144 = g;
    (*obj).emit_state = EMIT_GDRAW;
    initObjMapData(
        job,
        (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label,
        g as *mut libc::c_void,
    );
    gvrender_begin_graph(job, g);
}
unsafe extern "C" fn emit_end_graph(mut job: *mut GVJ_t) {
    gvrender_end_graph(job);
    pop_obj_state(job);
}
unsafe extern "C" fn emit_page(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut obj: *mut obj_state_t = (*job).obj;
    let mut nump: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = (*job).flags;
    let mut lab: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut p: *mut pointf = 0 as *mut pointf;
    let mut saveid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    if (*job).layerNum > 1 as libc::c_int
        || (*job).pagesArrayElem.x > 0 as libc::c_int
        || (*job).pagesArrayElem.x > 0 as libc::c_int
    {
        agxbinit(
            &mut xb,
            128 as libc::c_int as libc::c_uint,
            buf.as_mut_ptr(),
        );
        saveid = (*obj).id;
        layerPagePrefix(job, &mut xb);
        agxbput(&mut xb, saveid);
        let ref mut fresh145 = (*obj).id;
        *fresh145 = agxbuse(&mut xb);
    } else {
        saveid = 0 as *mut libc::c_char;
    }
    setColorScheme(agget(
        g as *mut libc::c_void,
        b"colorscheme\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    setup_page(job);
    gvrender_begin_page(job);
    gvrender_set_pencolor(
        job,
        b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    gvrender_set_fillcolor(
        job,
        b"lightgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if flags & ((1 as libc::c_int) << 16 as libc::c_int | (1 as libc::c_int) << 22 as libc::c_int)
        != 0
        && (!((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0)
    {
        if flags
            & ((1 as libc::c_int) << 17 as libc::c_int | (1 as libc::c_int) << 19 as libc::c_int)
            != 0
        {
            if flags & (1 as libc::c_int) << 17 as libc::c_int != 0 {
                (*obj).url_map_shape = MAP_RECTANGLE;
                nump = 2 as libc::c_int;
            } else {
                (*obj).url_map_shape = MAP_POLYGON;
                nump = 4 as libc::c_int;
            }
            p = gcalloc(
                nump as size_t,
                ::std::mem::size_of::<pointf>() as libc::c_ulong,
            ) as *mut pointf;
            *p.offset(0 as libc::c_int as isize) = (*job).pageBox.LL;
            *p.offset(1 as libc::c_int as isize) = (*job).pageBox.UR;
            if flags & (1 as libc::c_int) << 17 as libc::c_int == 0 {
                rect2poly(p);
            }
        }
        if flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
            gvrender_ptf_A(job, p, p, nump);
        }
        let ref mut fresh146 = (*obj).url_map_p;
        *fresh146 = p;
        (*obj).url_map_n = nump;
    }
    if flags & (1 as libc::c_int) << 15 as libc::c_int != 0 && {
        lab = (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label;
        !lab.is_null()
    } {
        let ref mut fresh147 = (*obj).label;
        *fresh147 = (*lab).text;
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int == 0
        && (!((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0)
    {
        emit_map_rect(job, (*job).clip);
        gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
    }
    emit_background(job, g);
    if !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label).is_null() {
        emit_label(
            job,
            EMIT_GLABEL,
            (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).label,
        );
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int == 0
        && (!((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0)
    {
        gvrender_end_anchor(job);
    }
    emit_view(job, g, flags);
    gvrender_end_page(job);
    if !saveid.is_null() {
        agxbfree(&mut xb);
        let ref mut fresh148 = (*obj).id;
        *fresh148 = saveid;
    }
}
#[no_mangle]
pub unsafe extern "C" fn emit_graph(mut job: *mut GVJ_t, mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_int = (*job).flags;
    let mut lp: *mut libc::c_int = 0 as *mut libc::c_int;
    (*job).scale.x = (*job).zoom * (*job).dpi.x / 72 as libc::c_int as libc::c_double;
    (*job).scale.y = (*job).zoom * (*job).dpi.y / 72 as libc::c_int as libc::c_double;
    (*job).devscale.x = (*job).dpi.x / 72 as libc::c_int as libc::c_double;
    (*job).devscale.y = (*job).dpi.y / 72 as libc::c_int as libc::c_double;
    if (*job).flags & (1 as libc::c_int) << 12 as libc::c_int != 0 || Y_invert != 0 {
        (*job).devscale.y *= -(1 as libc::c_int) as libc::c_double;
    }
    if (*job).rotation != 0 {
        (*job).view.y = (*job).width as libc::c_double / (*job).scale.y;
        (*job).view.x = (*job).height as libc::c_double / (*job).scale.x;
    } else {
        (*job).view.x = (*job).width as libc::c_double / (*job).scale.x;
        (*job).view.y = (*job).height as libc::c_double / (*job).scale.y;
    }
    s = late_string(
        g as *mut libc::c_void,
        agattr(
            g,
            0 as libc::c_int,
            b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        ),
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    gvrender_comment(job, s);
    (*job).layerNum = 0 as libc::c_int;
    emit_begin_graph(job, g);
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        emit_colors(job, g);
    }
    n = agfstnode(g);
    while !n.is_null() {
        (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).state =
            0 as libc::c_int as libc::c_char;
        n = agnxtnode(g, n);
    }
    firstlayer(job, &mut lp);
    while validlayer(job) {
        if numPhysicalLayers(job) > 1 as libc::c_int {
            gvrender_begin_layer(job);
        }
        firstpage(job);
        while validpage(job) {
            emit_page(job, g);
            nextpage(job);
        }
        if numPhysicalLayers(job) > 1 as libc::c_int {
            gvrender_end_layer(job);
        }
        nextlayer(job, &mut lp);
    }
    emit_end_graph(job);
}
unsafe extern "C" fn free_string_entry(
    mut dict: *mut Dict_t,
    mut key: *mut libc::c_char,
    mut disc: *mut Dtdisc_t,
) {
    free(key as *mut libc::c_void);
}
static mut strings: *mut Dict_t = 0 as *const Dict_t as *mut Dict_t;
static mut stringdict: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: -(1 as libc::c_int),
            makef: None,
            freef: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Dict_t, *mut libc::c_char, *mut Dtdisc_t) -> ()>,
                Dtfree_f,
            >(Some(
                free_string_entry
                    as unsafe extern "C" fn(*mut Dict_t, *mut libc::c_char, *mut Dtdisc_t) -> (),
            )),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn emit_once(mut str: *mut libc::c_char) -> libc::c_int {
    if strings.is_null() {
        strings = dtopen(&mut stringdict, Dtoset);
    }
    if ((Some(((*(strings as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        strings, str as *mut libc::c_void, 0o4 as libc::c_int
    ))
    .is_null()
    {
        (Some(((*(strings as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            strings,
            strdup(str) as *mut libc::c_void,
            0o1 as libc::c_int,
        );
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn emit_once_reset() {
    if !strings.is_null() {
        dtclose(strings);
        strings = 0 as *mut Dict_t;
    }
}
unsafe extern "C" fn emit_begin_cluster(mut job: *mut GVJ_t, mut sg: *mut Agraph_t) {
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    obj = push_obj_state(job);
    (*obj).type_0 = CLUSTER_OBJTYPE;
    let ref mut fresh149 = (*obj).u.sg;
    *fresh149 = sg;
    (*obj).emit_state = EMIT_CDRAW;
    initObjMapData(
        job,
        (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label,
        sg as *mut libc::c_void,
    );
    gvrender_begin_cluster(job, sg);
}
unsafe extern "C" fn emit_end_cluster(mut job: *mut GVJ_t, mut g: *mut Agraph_t) {
    gvrender_end_cluster(job, g);
    pop_obj_state(job);
}
#[no_mangle]
pub unsafe extern "C" fn emit_clusters(
    mut job: *mut GVJ_t,
    mut g: *mut Agraph_t,
    mut flags: libc::c_int,
) {
    let mut doPerim: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut istyle: libc::c_int = 0;
    let mut filled: libc::c_int = 0;
    let mut AF: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut color: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fillcolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pencolor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut style: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sg: *mut graph_t = 0 as *mut graph_t;
    let mut n: *mut node_t = 0 as *mut node_t;
    let mut e: *mut edge_t = 0 as *mut edge_t;
    let mut obj: *mut obj_state_t = 0 as *mut obj_state_t;
    let mut lab: *mut textlabel_t = 0 as *mut textlabel_t;
    let mut doAnchor: libc::c_int = 0;
    let mut penwidth: libc::c_double = 0.;
    let mut clrs: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    c = 1 as libc::c_int;
    while c <= (*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).n_cluster {
        sg = *((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).clust).offset(c as isize);
        if clust_in_layer(job, sg) {
            if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                emit_clusters(job, sg, flags);
            }
            emit_begin_cluster(job, sg);
            obj = (*job).obj;
            doAnchor = (!((*obj).url).is_null() || (*obj).explicit_tooltip() as libc::c_int != 0)
                as libc::c_int;
            setColorScheme(agget(
                sg as *mut libc::c_void,
                b"colorscheme\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ));
            if doAnchor != 0 && flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
                emit_map_rect(
                    job,
                    (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
                );
                gvrender_begin_anchor(job, (*obj).url, (*obj).tooltip, (*obj).target, (*obj).id);
            }
            filled = 0 as libc::c_int;
            istyle = 0 as libc::c_int;
            style = checkClusterStyle(sg, &mut istyle);
            if !style.is_null() {
                gvrender_set_style(job, style);
                if istyle & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                    filled = 1 as libc::c_int;
                }
            }
            pencolor = 0 as *mut libc::c_char;
            fillcolor = pencolor;
            if (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state as libc::c_int
                & (1 as libc::c_int) << 0 as libc::c_int
                != 0
            {
                pencolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_activepencolor,
                    b"#808080\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                fillcolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_activefillcolor,
                    b"#fcfcfc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                filled = (0 as libc::c_int == 0) as libc::c_int;
            } else if (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state
                as libc::c_int
                & (1 as libc::c_int) << 1 as libc::c_int
                != 0
            {
                pencolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_activepencolor,
                    b"#303030\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                fillcolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_activefillcolor,
                    b"#e8e8e8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                filled = (0 as libc::c_int == 0) as libc::c_int;
            } else if (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state
                as libc::c_int
                & (1 as libc::c_int) << 3 as libc::c_int
                != 0
            {
                pencolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_deletedpencolor,
                    b"#e0e0e0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                fillcolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_deletedfillcolor,
                    b"#f0f0f0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                filled = (0 as libc::c_int == 0) as libc::c_int;
            } else if (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).gui_state
                as libc::c_int
                & (1 as libc::c_int) << 2 as libc::c_int
                != 0
            {
                pencolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_visitedpencolor,
                    b"#101010\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                fillcolor = late_nnstring(
                    sg as *mut libc::c_void,
                    G_visitedfillcolor,
                    b"#f8f8f8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                filled = (0 as libc::c_int == 0) as libc::c_int;
            } else {
                color = agget(
                    sg as *mut libc::c_void,
                    b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if !color.is_null() && *color.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    pencolor = color;
                    fillcolor = pencolor;
                }
                color = agget(
                    sg as *mut libc::c_void,
                    b"pencolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if !color.is_null() && *color.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    pencolor = color;
                }
                color = agget(
                    sg as *mut libc::c_void,
                    b"fillcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if !color.is_null() && *color.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    fillcolor = color;
                }
                if (filled == 0 || fillcolor.is_null())
                    && {
                        color = agget(
                            sg as *mut libc::c_void,
                            b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        !color.is_null()
                    }
                    && *color.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    fillcolor = color;
                    filled = 1 as libc::c_int;
                }
            }
            if pencolor.is_null() {
                pencolor = b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            if fillcolor.is_null() {
                fillcolor = b"lightgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            clrs[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
            if filled != 0 {
                let mut frac: libc::c_float = 0.;
                if findStopColor(fillcolor, clrs.as_mut_ptr(), &mut frac) {
                    gvrender_set_fillcolor(job, clrs[0 as libc::c_int as usize]);
                    if !(clrs[1 as libc::c_int as usize]).is_null() {
                        gvrender_set_gradient_vals(
                            job,
                            clrs[1 as libc::c_int as usize],
                            late_int(
                                sg as *mut libc::c_void,
                                G_gradientangle,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            ),
                            frac,
                        );
                    } else {
                        gvrender_set_gradient_vals(
                            job,
                            b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            late_int(
                                sg as *mut libc::c_void,
                                G_gradientangle,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            ),
                            frac,
                        );
                    }
                    if istyle & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                        filled = 3 as libc::c_int;
                    } else {
                        filled = 2 as libc::c_int;
                    }
                } else {
                    gvrender_set_fillcolor(job, fillcolor);
                }
            }
            if !G_penwidth.is_null() && {
                s = agxget(sg as *mut libc::c_void, G_penwidth);
                !s.is_null() && *s.offset(0 as libc::c_int as isize) as libc::c_int != 0
            } {
                penwidth = late_double(sg as *mut libc::c_void, G_penwidth, 1.0f64, 0.0f64);
                gvrender_set_penwidth(job, penwidth);
            }
            if istyle & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                doPerim = late_int(
                    sg as *mut libc::c_void,
                    G_peripheries,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
                if doPerim != 0 || filled != 0 {
                    AF[0 as libc::c_int as usize] =
                        (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL;
                    AF[2 as libc::c_int as usize] =
                        (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR;
                    AF[1 as libc::c_int as usize].x = AF[2 as libc::c_int as usize].x;
                    AF[1 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y;
                    AF[3 as libc::c_int as usize].x = AF[0 as libc::c_int as usize].x;
                    AF[3 as libc::c_int as usize].y = AF[2 as libc::c_int as usize].y;
                    if doPerim != 0 {
                        gvrender_set_pencolor(job, pencolor);
                    } else {
                        gvrender_set_pencolor(
                            job,
                            b"transparent\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    round_corners(job, AF.as_mut_ptr(), 4 as libc::c_int, istyle, filled);
                }
            } else if istyle & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                let mut rv: libc::c_int = 0;
                AF[0 as libc::c_int as usize] =
                    (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.LL;
                AF[2 as libc::c_int as usize] =
                    (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb.UR;
                AF[1 as libc::c_int as usize].x = AF[2 as libc::c_int as usize].x;
                AF[1 as libc::c_int as usize].y = AF[0 as libc::c_int as usize].y;
                AF[3 as libc::c_int as usize].x = AF[0 as libc::c_int as usize].x;
                AF[3 as libc::c_int as usize].y = AF[2 as libc::c_int as usize].y;
                if late_int(
                    sg as *mut libc::c_void,
                    G_peripheries,
                    1 as libc::c_int,
                    0 as libc::c_int,
                ) == 0 as libc::c_int
                {
                    gvrender_set_pencolor(
                        job,
                        b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    gvrender_set_pencolor(job, pencolor);
                }
                rv = stripedBox(job, AF.as_mut_ptr(), fillcolor, 0 as libc::c_int);
                if rv > 1 as libc::c_int {
                    agerr(
                        AGPREV,
                        b"in cluster %s\n\0" as *const u8 as *const libc::c_char,
                        agnameof(sg as *mut libc::c_void),
                    );
                }
                gvrender_box(
                    job,
                    (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
                    0 as libc::c_int,
                );
            } else if late_int(
                sg as *mut libc::c_void,
                G_peripheries,
                1 as libc::c_int,
                0 as libc::c_int,
            ) != 0
            {
                gvrender_set_pencolor(job, pencolor);
                gvrender_box(
                    job,
                    (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
                    filled,
                );
            } else if filled != 0 {
                gvrender_set_pencolor(
                    job,
                    b"transparent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                gvrender_box(
                    job,
                    (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
                    filled,
                );
            }
            free(clrs[0 as libc::c_int as usize] as *mut libc::c_void);
            lab = (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).label;
            if !lab.is_null() {
                emit_label(job, EMIT_CLABEL, lab);
            }
            if doAnchor != 0 {
                if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    emit_map_rect(
                        job,
                        (*((*(sg as *mut Agobj_t)).data as *mut Agraphinfo_t)).bb,
                    );
                    gvrender_begin_anchor(
                        job,
                        (*obj).url,
                        (*obj).tooltip,
                        (*obj).target,
                        (*obj).id,
                    );
                }
                gvrender_end_anchor(job);
            }
            if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                n = agfstnode(sg);
                while !n.is_null() {
                    emit_node(job, n);
                    e = agfstout(sg, n);
                    while !e.is_null() {
                        emit_edge(job, e);
                        e = agnxtout(sg, e);
                    }
                    n = agnxtnode(sg, n);
                }
            }
            emit_end_cluster(job, g);
            if flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
                emit_clusters(job, sg, flags);
            }
        }
        c += 1;
    }
}
unsafe extern "C" fn is_style_delim(mut c: libc::c_int) -> bool {
    match c {
        40 | 41 | 44 | 0 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn style_token(mut s: *mut *mut libc::c_char) -> token_t {
    let mut p: *mut libc::c_char = *s;
    let mut token: libc::c_int = 0;
    while *p as libc::c_int != 0
        && (*(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || *p as libc::c_int == ',' as i32)
    {
        p = p.offset(1);
    }
    let mut start: *const libc::c_char = p;
    match *p as libc::c_int {
        0 => {
            token = 0 as libc::c_int;
        }
        40 | 41 => {
            let fresh150 = p;
            p = p.offset(1);
            token = *fresh150 as libc::c_int;
        }
        _ => {
            token = 1 as libc::c_int;
            while !is_style_delim(*p as libc::c_int) {
                p = p.offset(1);
            }
        }
    }
    *s = p;
    if start <= p as *const libc::c_char {
    } else {
        __assert_fail(
            b"start <= p\0" as *const u8 as *const libc::c_char,
            b"emit.c\0" as *const u8 as *const libc::c_char,
            3788 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"token_t style_token(char **)\0",
            ))
            .as_ptr(),
        );
    }
    let mut size: size_t = p.offset_from(start) as libc::c_long as size_t;
    return {
        let mut init = token_t {
            type_0: token,
            start: start,
            size: size,
        };
        init
    };
}
static mut outbuf: [libc::c_char; 128] = [0; 128];
static mut ps_xb: agxbuf = agxbuf {
    buf: 0 as *mut libc::c_char,
    ptr: 0 as *mut libc::c_char,
    eptr: 0 as *mut libc::c_char,
    dyna: 0,
};
#[no_mangle]
pub unsafe extern "C" fn parse_style(mut s: *mut libc::c_char) -> *mut *mut libc::c_char {
    static mut parse: [*mut libc::c_char; 64] = [0 as *const libc::c_char as *mut libc::c_char; 64];
    static mut is_first: bool = 1 as libc::c_int != 0;
    let mut fun: libc::c_int = 0 as libc::c_int;
    let mut in_parens: bool = 0 as libc::c_int != 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if is_first {
        agxbinit(
            &mut ps_xb,
            128 as libc::c_int as libc::c_uint,
            outbuf.as_mut_ptr(),
        );
        is_first = 0 as libc::c_int != 0;
    }
    p = s;
    loop {
        let mut c: token_t = style_token(&mut p);
        if c.type_0 == 0 as libc::c_int {
            break;
        }
        match c.type_0 {
            40 => {
                if in_parens {
                    agerr(
                        AGERR,
                        b"nesting not allowed in style: %s\n\0" as *const u8 as *const libc::c_char,
                        s,
                    );
                    parse[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
                    return parse.as_mut_ptr();
                }
                in_parens = 1 as libc::c_int != 0;
            }
            41 => {
                if !in_parens {
                    agerr(
                        AGERR,
                        b"unmatched ')' in style: %s\n\0" as *const u8 as *const libc::c_char,
                        s,
                    );
                    parse[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
                    return parse.as_mut_ptr();
                }
                in_parens = 0 as libc::c_int != 0;
            }
            _ => {
                if !in_parens {
                    if fun == 64 as libc::c_int - 1 as libc::c_int {
                        agerr(
                            AGWARN,
                            b"truncating style '%s'\n\0" as *const u8 as *const libc::c_char,
                            s,
                        );
                        parse[fun as usize] = 0 as *mut libc::c_char;
                        return parse.as_mut_ptr();
                    }
                    agxbputc(&mut ps_xb, '\0' as i32 as libc::c_char);
                    let fresh151 = fun;
                    fun = fun + 1;
                    parse[fresh151 as usize] = agxbnext(&mut ps_xb);
                }
                agxbput_n(&mut ps_xb, c.start, c.size);
                agxbputc(&mut ps_xb, '\0' as i32 as libc::c_char);
            }
        }
    }
    if in_parens {
        agerr(
            AGERR,
            b"unmatched '(' in style: %s\n\0" as *const u8 as *const libc::c_char,
            s,
        );
        parse[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        return parse.as_mut_ptr();
    }
    parse[fun as usize] = 0 as *mut libc::c_char;
    agxbuse(&mut ps_xb);
    return parse.as_mut_ptr();
}
unsafe extern "C" fn bezier_bb(mut bz: bezier) -> boxf {
    let mut i: libc::c_int = 0;
    let mut p: pointf = pointf { x: 0., y: 0. };
    let mut p1: pointf = pointf { x: 0., y: 0. };
    let mut p2: pointf = pointf { x: 0., y: 0. };
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    if bz.size > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"bz.size > 0\0" as *const u8 as *const libc::c_char,
            b"emit.c\0" as *const u8 as *const libc::c_char,
            3874 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"boxf bezier_bb(bezier)\0"))
                .as_ptr(),
        );
    }
    if bz.size % 3 as libc::c_int == 1 as libc::c_int {
    } else {
        __assert_fail(
            b"bz.size % 3 == 1\0" as *const u8 as *const libc::c_char,
            b"emit.c\0" as *const u8 as *const libc::c_char,
            3875 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"boxf bezier_bb(bezier)\0"))
                .as_ptr(),
        );
    }
    bb.UR = *(bz.list).offset(0 as libc::c_int as isize);
    bb.LL = bb.UR;
    i = 1 as libc::c_int;
    while i < bz.size {
        p1 = *(bz.list).offset(i as isize);
        i += 1;
        p2 = *(bz.list).offset(i as isize);
        i += 1;
        p.x = (p1.x + p2.x) / 2 as libc::c_int as libc::c_double;
        p.y = (p1.y + p2.y) / 2 as libc::c_int as libc::c_double;
        bb.LL.x = (if bb.LL.x < p.x { bb.LL.x } else { p.x });
        bb.LL.y = (if bb.LL.y < p.y { bb.LL.y } else { p.y });
        bb.UR.x = (if bb.UR.x > p.x { bb.UR.x } else { p.x });
        bb.UR.y = (if bb.UR.y > p.y { bb.UR.y } else { p.y });
        p = *(bz.list).offset(i as isize);
        bb.LL.x = (if bb.LL.x < p.x { bb.LL.x } else { p.x });
        bb.LL.y = (if bb.LL.y < p.y { bb.LL.y } else { p.y });
        bb.UR.x = (if bb.UR.x > p.x { bb.UR.x } else { p.x });
        bb.UR.y = (if bb.UR.y > p.y { bb.UR.y } else { p.y });
        i += 1;
    }
    return bb;
}
unsafe extern "C" fn init_splines_bb(mut spl: *mut splines) {
    let mut i: libc::c_int = 0;
    let mut bz: bezier = bezier {
        list: 0 as *mut pointf,
        size: 0,
        sflag: 0,
        eflag: 0,
        sp: pointf { x: 0., y: 0. },
        ep: pointf { x: 0., y: 0. },
    };
    let mut bb: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    let mut b: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    if (*spl).size > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"spl->size > 0\0" as *const u8 as *const libc::c_char,
            b"emit.c\0" as *const u8 as *const libc::c_char,
            3900 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void init_splines_bb(splines *)\0",
            ))
            .as_ptr(),
        );
    }
    bz = *((*spl).list).offset(0 as libc::c_int as isize);
    bb = bezier_bb(bz);
    i = 0 as libc::c_int;
    while i < (*spl).size {
        if i > 0 as libc::c_int {
            bz = *((*spl).list).offset(i as isize);
            b = bezier_bb(bz);
            bb.LL.x = (if bb.LL.x < b.LL.x { bb.LL.x } else { b.LL.x });
            bb.LL.y = (if bb.LL.y < b.LL.y { bb.LL.y } else { b.LL.y });
            bb.UR.x = (if bb.UR.x > b.UR.x { bb.UR.x } else { b.UR.x });
            bb.UR.y = (if bb.UR.y > b.UR.y { bb.UR.y } else { b.UR.y });
        }
        if bz.sflag != 0 {
            b = arrow_bb(
                bz.sp,
                *(bz.list).offset(0 as libc::c_int as isize),
                1 as libc::c_int as libc::c_double,
            );
            bb.LL.x = (if bb.LL.x < b.LL.x { bb.LL.x } else { b.LL.x });
            bb.LL.y = (if bb.LL.y < b.LL.y { bb.LL.y } else { b.LL.y });
            bb.UR.x = (if bb.UR.x > b.UR.x { bb.UR.x } else { b.UR.x });
            bb.UR.y = (if bb.UR.y > b.UR.y { bb.UR.y } else { b.UR.y });
        }
        if bz.eflag != 0 {
            b = arrow_bb(
                bz.ep,
                *(bz.list).offset((bz.size - 1 as libc::c_int) as isize),
                1 as libc::c_int as libc::c_double,
            );
            bb.LL.x = (if bb.LL.x < b.LL.x { bb.LL.x } else { b.LL.x });
            bb.LL.y = (if bb.LL.y < b.LL.y { bb.LL.y } else { b.LL.y });
            bb.UR.x = (if bb.UR.x > b.UR.x { bb.UR.x } else { b.UR.x });
            bb.UR.y = (if bb.UR.y > b.UR.y { bb.UR.y } else { b.UR.y });
        }
        i += 1;
    }
    (*spl).bb = bb;
}
unsafe extern "C" fn init_bb_edge(mut e: *mut edge_t) {
    let mut spl: *mut splines = 0 as *mut splines;
    spl = (*((*(e as *mut Agobj_t)).data as *mut Agedgeinfo_t)).spl;
    if !spl.is_null() {
        init_splines_bb(spl);
    }
}
unsafe extern "C" fn init_bb_node(mut g: *mut graph_t, mut n: *mut node_t) {
    let mut e: *mut edge_t = 0 as *mut edge_t;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .bb
        .LL
        .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .coord
        .x
        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).lw;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .bb
        .LL
        .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .coord
        .y
        - (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .bb
        .UR
        .x = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .coord
        .x
        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).rw;
    (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .bb
        .UR
        .y = (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t))
        .coord
        .y
        + (*((*(n as *mut Agobj_t)).data as *mut Agnodeinfo_t)).ht / 2.0f64;
    e = agfstout(g, n);
    while !e.is_null() {
        init_bb_edge(e);
        e = agnxtout(g, e);
    }
}
unsafe extern "C" fn init_bb(mut g: *mut graph_t) {
    let mut n: *mut node_t = 0 as *mut node_t;
    n = agfstnode(g);
    while !n.is_null() {
        init_bb_node(g, n);
        n = agnxtnode(g, n);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gv_fixLocale(mut set: libc::c_int) {
    static mut save_locale: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut cnt: libc::c_int = 0;
    if set != 0 {
        cnt += 1;
        if cnt == 1 as libc::c_int {
            save_locale = strdup(setlocale(1 as libc::c_int, 0 as *const libc::c_char));
            setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
        }
    } else if cnt > 0 as libc::c_int {
        cnt -= 1;
        if cnt == 0 as libc::c_int {
            setlocale(1 as libc::c_int, save_locale);
            free(save_locale as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gvRenderJobs(mut gvc: *mut GVC_t, mut g: *mut graph_t) -> libc::c_int {
    static mut prevjob: *mut GVJ_t = 0 as *const GVJ_t as *mut GVJ_t;
    let mut job: *mut GVJ_t = 0 as *mut GVJ_t;
    let mut firstjob: *mut GVJ_t = 0 as *mut GVJ_t;
    if Verbose != 0 {
        start_timer();
    }
    if !(!(agbindrec(
        g as *mut libc::c_void,
        b"Agraphinfo_t\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_uint,
        (0 as libc::c_int == 0) as libc::c_int,
    ))
    .is_null()
        && !((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).is_null())
    {
        agerr(
            AGERR,
            b"Layout was not done.  Missing layout plugins? \n\0" as *const u8
                as *const libc::c_char,
        );
        if Verbose != 0 {
            fprintf(
                stderr,
                b"gvRenderJobs %s: %.2f secs.\n\0" as *const u8 as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
                elapsed_sec(),
            );
        }
        return -(1 as libc::c_int);
    }
    init_bb(g);
    init_gvc(gvc, g);
    init_layering(gvc, g);
    gv_fixLocale(1 as libc::c_int);
    let mut current_block_67: u64;
    job = gvjobs_first(gvc);
    while !job.is_null() {
        if !((*gvc).gvg).is_null() {
            let ref mut fresh152 = (*job).input_filename;
            *fresh152 = (*(*gvc).gvg).input_filename;
            (*job).graph_index = (*(*gvc).gvg).graph_index;
        } else {
            let ref mut fresh153 = (*job).input_filename;
            *fresh153 = 0 as *mut libc::c_char;
            (*job).graph_index = 0 as libc::c_int;
        }
        let ref mut fresh154 = (*job).common;
        *fresh154 = &mut (*gvc).common;
        let ref mut fresh155 = (*job).layout_type;
        *fresh155 = (*gvc).layout.type_0;
        let ref mut fresh156 = (*job).keybindings;
        *fresh156 = gvevent_key_binding.as_mut_ptr();
        (*job).numkeys = gvevent_key_binding_size;
        if ((*((*(g as *mut Agobj_t)).data as *mut Agraphinfo_t)).drawing).is_null() {
            agerr(
                AGERR,
                b"layout was not done\n\0" as *const u8 as *const libc::c_char,
            );
            gv_fixLocale(0 as libc::c_int);
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"gvRenderJobs %s: %.2f secs.\n\0" as *const u8 as *const libc::c_char,
                    agnameof(g as *mut libc::c_void),
                    elapsed_sec(),
                );
            }
            return -(1 as libc::c_int);
        }
        (*job).output_lang = gvrender_select(job, (*job).output_langname);
        if (*job).output_lang == 999 as libc::c_int {
            agerr(
                AGERR,
                b"renderer for %s is unavailable\n\0" as *const u8 as *const libc::c_char,
                (*job).output_langname,
            );
            gv_fixLocale(0 as libc::c_int);
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"gvRenderJobs %s: %.2f secs.\n\0" as *const u8 as *const libc::c_char,
                    agnameof(g as *mut libc::c_void),
                    elapsed_sec(),
                );
            }
            return -(1 as libc::c_int);
        }
        match (*job).output_lang {
            21 => {
                (*job).flags |= (1 as libc::c_int) << 0 as libc::c_int;
            }
            _ => {
                (*job).flags |= chkOrder(g);
            }
        }
        firstjob = (*gvc).active_jobs;
        if !firstjob.is_null() {
            if (*firstjob).flags & (1 as libc::c_int) << 5 as libc::c_int == 0
                || strcmp((*job).output_langname, (*firstjob).output_langname) != 0
            {
                gvrender_end_job(firstjob);
                let ref mut fresh157 = (*gvc).active_jobs;
                *fresh157 = 0 as *mut GVJ_t;
                (*gvc).common.viewNum = 0 as libc::c_int;
                prevjob = 0 as *mut GVJ_t;
            }
        } else {
            prevjob = 0 as *mut GVJ_t;
        }
        if !prevjob.is_null() {
            let ref mut fresh158 = (*prevjob).next_active;
            *fresh158 = job;
            let ref mut fresh159 = (*job).output_file;
            *fresh159 = (*prevjob).output_file;
            current_block_67 = 12829669402821218572;
        } else if gvrender_begin_job(job) != 0 {
            current_block_67 = 7651349459974463963;
        } else {
            let ref mut fresh160 = (*gvc).active_jobs;
            *fresh160 = job;
            current_block_67 = 12829669402821218572;
        }
        match current_block_67 {
            12829669402821218572 => {
                let ref mut fresh161 = (*job).next_active;
                *fresh161 = 0 as *mut GVJ_t;
                let ref mut fresh162 = (*job).callbacks;
                *fresh162 = &mut gvdevice_callbacks;
                init_job_pad(job);
                init_job_margin(job);
                init_job_dpi(job, g);
                init_job_viewport(job, g);
                init_job_pagination(job, g);
                if (*job).flags & (1 as libc::c_int) << 7 as libc::c_int == 0 {
                    emit_graph(job, g);
                }
                prevjob = job;
            }
            _ => {}
        }
        job = gvjobs_next(gvc);
    }
    gv_fixLocale(0 as libc::c_int);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"gvRenderJobs %s: %.2f secs.\n\0" as *const u8 as *const libc::c_char,
            agnameof(g as *mut libc::c_void),
            elapsed_sec(),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findStopColor(
    mut colorlist: *mut libc::c_char,
    mut clrs: *mut *mut libc::c_char,
    mut frac: *mut libc::c_float,
) -> bool {
    let mut segs: *mut colorsegs_t = 0 as *mut colorsegs_t;
    let mut rv: libc::c_int = 0;
    rv = parseSegs(colorlist, 0 as libc::c_int, &mut segs);
    if rv != 0
        || (*segs).numc < 2 as libc::c_int
        || ((*((*segs).segs).offset(0 as libc::c_int as isize)).color).is_null()
    {
        let ref mut fresh163 = *clrs.offset(0 as libc::c_int as isize);
        *fresh163 = 0 as *mut libc::c_char;
        if !segs.is_null() {
            freeSegs(segs);
        }
        return 0 as libc::c_int != 0;
    }
    if (*segs).numc > 2 as libc::c_int {
        agerr(
            AGWARN,
            b"More than 2 colors specified for a gradient - ignoring remaining\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    let ref mut fresh164 = *clrs.offset(0 as libc::c_int as isize);
    *fresh164 = gcalloc(
        (strlen(colorlist)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    strcpy(
        *clrs.offset(0 as libc::c_int as isize),
        (*((*segs).segs).offset(0 as libc::c_int as isize)).color,
    );
    if !((*((*segs).segs).offset(1 as libc::c_int as isize)).color).is_null() {
        let ref mut fresh165 = *clrs.offset(1 as libc::c_int as isize);
        *fresh165 = (*clrs.offset(0 as libc::c_int as isize)).offset(
            (strlen(*clrs.offset(0 as libc::c_int as isize)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        );
        strcpy(
            *clrs.offset(1 as libc::c_int as isize),
            (*((*segs).segs).offset(1 as libc::c_int as isize)).color,
        );
    } else {
        let ref mut fresh166 = *clrs.offset(1 as libc::c_int as isize);
        *fresh166 = 0 as *mut libc::c_char;
    }
    if (*((*segs).segs).offset(0 as libc::c_int as isize)).hasFraction {
        *frac = (*((*segs).segs).offset(0 as libc::c_int as isize)).t;
    } else if (*((*segs).segs).offset(1 as libc::c_int as isize)).hasFraction {
        *frac = 1 as libc::c_int as libc::c_float
            - (*((*segs).segs).offset(1 as libc::c_int as isize)).t;
    } else {
        *frac = 0 as libc::c_int as libc::c_float;
    }
    freeSegs(segs);
    return 1 as libc::c_int != 0;
}
