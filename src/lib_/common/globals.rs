#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdpParms_s {
    pub useGrid: libc::c_int,
    pub useNew: libc::c_int,
    pub numIters: libc::c_int,
    pub unscaled: libc::c_int,
    pub C: libc::c_double,
    pub Tfact: libc::c_double,
    pub K: libc::c_double,
    pub T0: libc::c_double,
}
pub type fdpParms_t = fdpParms_s;
#[no_mangle]
pub static mut Version: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut Files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut Lib: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
#[no_mangle]
pub static mut CmdName: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut Gvfilepath: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut Gvimagepath: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut Verbose: libc::c_uchar = 0;
#[no_mangle]
pub static mut Reduce: libc::c_uchar = 0;
#[no_mangle]
pub static mut MemTest: libc::c_int = 0;
#[no_mangle]
pub static mut HTTPServerEnVar: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut graphviz_errors: libc::c_int = 0;
#[no_mangle]
pub static mut Nop: libc::c_int = 0;
#[no_mangle]
pub static mut PSinputscale: libc::c_double = 0.;
#[no_mangle]
pub static mut Show_cnt: libc::c_int = 0;
#[no_mangle]
pub static mut Show_boxes: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut CL_type: libc::c_int = 0;
#[no_mangle]
pub static mut Concentrate: libc::c_uchar = 0;
#[no_mangle]
pub static mut Epsilon: libc::c_double = 0.;
#[no_mangle]
pub static mut MaxIter: libc::c_int = 0;
#[no_mangle]
pub static mut Ndim: libc::c_int = 0;
#[no_mangle]
pub static mut State: libc::c_int = 0;
#[no_mangle]
pub static mut EdgeLabelsDone: libc::c_int = 0;
#[no_mangle]
pub static mut Initial_dist: libc::c_double = 0.;
#[no_mangle]
pub static mut Damping: libc::c_double = 0.;
#[no_mangle]
pub static mut Y_invert: libc::c_int = 0;
#[no_mangle]
pub static mut GvExitOnUsage: libc::c_int = 0;
#[no_mangle]
pub static mut G_activepencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_activefillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_visitedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_visitedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_deletedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_deletedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_ordering: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_peripheries: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_penwidth: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_gradientangle: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut G_margin: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_height: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_width: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_shape: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_color: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_fillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_activepencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_activefillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_selectedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_selectedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_visitedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_visitedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_deletedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_deletedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_fontsize: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_fontname: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_fontcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_margin: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_label: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_xlabel: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_nojustify: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_style: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_showboxes: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_sides: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_peripheries: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_ordering: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_orientation: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_skew: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_distortion: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_fixed: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_imagescale: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_imagepos: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_layer: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_group: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_comment: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_vertices: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_z: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_penwidth: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut N_gradientangle: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_weight: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_minlen: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_color: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_fillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_activepencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_activefillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_selectedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_selectedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_visitedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_visitedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_deletedpencolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_deletedfillcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_fontsize: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_fontname: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_fontcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_label: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_xlabel: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_dir: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_style: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_decorate: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_showboxes: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_arrowsz: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_constr: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_layer: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_comment: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_label_float: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_samehead: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_sametail: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_arrowhead: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_arrowtail: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_headlabel: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_taillabel: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_labelfontsize: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_labelfontname: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_labelfontcolor: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_labeldistance: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_labelangle: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_tailclip: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_headclip: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
#[no_mangle]
pub static mut E_penwidth: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
static mut fdpParms: fdpParms_t = {
    let mut init = fdpParms_s {
        useGrid: 1 as libc::c_int,
        useNew: 1 as libc::c_int,
        numIters: -(1 as libc::c_int),
        unscaled: 50 as libc::c_int,
        C: 0.0f64,
        Tfact: 1.0f64,
        K: -1.0f64,
        T0: -1.0f64,
    };
    init
};
#[no_mangle]
pub static mut fdp_parms: *mut fdpParms_s = unsafe {
    &fdpParms as *const fdpParms_t as *mut fdpParms_t
};
