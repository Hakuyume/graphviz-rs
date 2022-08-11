#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Verbose: libc::c_uchar;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(
        A: SparseMatrix,
    ) -> SparseMatrix;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn map_palette_optimal_coloring(
        color_scheme: *mut libc::c_char,
        lightness: *mut libc::c_char,
        A: SparseMatrix,
        accuracy: libc::c_double,
        seed: libc::c_int,
        rgb_r: *mut *mut libc::c_float,
        rgb_g: *mut *mut libc::c_float,
        rgb_b: *mut *mut libc::c_float,
    );
    fn map_optimal_coloring(
        seed: libc::c_int,
        A: SparseMatrix,
        rgb_r: *mut libc::c_float,
        rgb_g: *mut libc::c_float,
        rgb_b: *mut libc::c_float,
    );
    fn plot_dot_map(
        gr: *mut Agraph_t,
        n: libc::c_int,
        dim: libc::c_int,
        x: *mut libc::c_double,
        polys: SparseMatrix,
        poly_lines: SparseMatrix,
        line_width: libc::c_double,
        line_color: *const libc::c_char,
        x_poly: *mut libc::c_double,
        polys_groups: *mut libc::c_int,
        labels: *mut *mut libc::c_char,
        fsz: *mut libc::c_float,
        r: *mut libc::c_float,
        g: *mut libc::c_float,
        b: *mut libc::c_float,
        opacity: *const libc::c_char,
        A: SparseMatrix,
        _: *mut FILE,
    );
    fn improve_contiguity(
        n: libc::c_int,
        dim: libc::c_int,
        grouping: *mut libc::c_int,
        poly_point_map: SparseMatrix,
        x: *mut libc::c_double,
        graph: SparseMatrix,
    );
    fn make_map_from_rectangle_groups(
        exclude_random: libc::c_int,
        include_OK_points: libc::c_int,
        n: libc::c_int,
        dim: libc::c_int,
        x: *mut libc::c_double,
        sizes: *mut libc::c_double,
        grouping: *mut libc::c_int,
        graph: SparseMatrix,
        bounding_box_margin: *mut libc::c_double,
        nrandom: *mut libc::c_int,
        nart: *mut libc::c_int,
        nedgep: libc::c_int,
        shore_depth_tol: libc::c_double,
        edge_bridge_tol: libc::c_double,
        xcombined: *mut *mut libc::c_double,
        nverts: *mut libc::c_int,
        x_poly: *mut *mut libc::c_double,
        npolys: *mut libc::c_int,
        poly_lines: *mut SparseMatrix,
        polys: *mut SparseMatrix,
        polys_groups: *mut *mut libc::c_int,
        poly_point_map: *mut SparseMatrix,
        country_graph: *mut SparseMatrix,
        highlight_cluster: libc::c_int,
        flag: *mut libc::c_int,
    ) -> libc::c_int;
    fn remove_overlap(
        dim: libc::c_int,
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
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    fn initDotIO(g: *mut Agraph_t);
    fn Import_coord_clusters_from_dot(
        g: *mut Agraph_t,
        maxcluster: libc::c_int,
        dim: libc::c_int,
        nn: *mut libc::c_int,
        label_sizes: *mut *mut libc::c_double,
        x: *mut *mut libc::c_double,
        clusters: *mut *mut libc::c_int,
        rgb_r: *mut *mut libc::c_float,
        rgb_g: *mut *mut libc::c_float,
        rgb_b: *mut *mut libc::c_float,
        fsz: *mut *mut libc::c_float,
        labels: *mut *mut *mut libc::c_char,
        default_color_scheme: libc::c_int,
        clustering_scheme: libc::c_int,
        useClusters: libc::c_int,
    ) -> SparseMatrix;
    fn Dot_SetClusterColor(
        g: *mut Agraph_t,
        rgb_r: *mut libc::c_float,
        rgb_g: *mut libc::c_float,
        rgb_b: *mut libc::c_float,
        clustering: *mut libc::c_int,
    );
    fn knownColorScheme(_: *const libc::c_char) -> libc::c_int;
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
pub type uint64_t = __uint64_t;
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
pub type Dtlink_t = _dtlink_s;
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
pub type Dtdisc_t = _dtdisc_s;
pub type Dt_t = _dt_s;
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
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed_0,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
}
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
pub type Dict_t = _dt_s;
pub type IDTYPE = uint64_t;
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
pub type Agtag_t = Agtag_s;
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
pub type Agobj_t = Agobj_s;
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
    pub graph: C2RustUnnamed_1,
    pub node: C2RustUnnamed_1,
    pub edge: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ins: agobjfn_t,
    pub mod_0: agobjupdfn_t,
    pub del: agobjfn_t,
}
pub type agobjfn_t = Option::<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> (),
>;
pub type Agraph_t = Agraph_s;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ELSCHEME_STRAIGHTLINE_PENALTY2: C2RustUnnamed_2 = 4;
pub const ELSCHEME_STRAIGHTLINE_PENALTY: C2RustUnnamed_2 = 3;
pub const ELSCHEME_PENALTY2: C2RustUnnamed_2 = 2;
pub const ELSCHEME_PENALTY: C2RustUnnamed_2 = 1;
pub const ELSCHEME_NONE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CLUSTERING_MQ: C2RustUnnamed_3 = 1;
pub const CLUSTERING_MODULARITY: C2RustUnnamed_3 = 0;
pub type opengfn = Option::<unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ingdisc {
    pub openf: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_void>,
    pub readf: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut Agraph_t>,
    pub closef: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub dflt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ingraph_state {
    pub u: C2RustUnnamed_4,
    pub ctr: libc::c_int,
    pub ingraphs: libc::c_int,
    pub fp: *mut libc::c_void,
    pub fns: *mut ingdisc,
    pub heap: bool,
    pub errors: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub Files: *mut *mut libc::c_char,
    pub Graphs: *mut *mut Agraph_t,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const COLOR_SCHEME_GREY: C2RustUnnamed_5 = 10;
pub const COLOR_SCHEME_SEQUENTIAL_SINGLEHUE_RED_LIGHTER: C2RustUnnamed_5 = 9;
pub const COLOR_SCHEME_ADAM_BLEND: C2RustUnnamed_5 = 8;
pub const COLOR_SCHEME_ADAM: C2RustUnnamed_5 = 7;
pub const COLOR_SCHEME_SEQUENTIAL_SINGLEHUE_RED: C2RustUnnamed_5 = 6;
pub const COLOR_SCHEME_PRIMARY: C2RustUnnamed_5 = 5;
pub const COLOR_SCHEME_GREY_RED: C2RustUnnamed_5 = 4;
pub const COLOR_SCHEME_WHITE_RED: C2RustUnnamed_5 = 3;
pub const COLOR_SCHEME_BLUE_YELLOW: C2RustUnnamed_5 = 2;
pub const COLOR_SCHEME_PASTEL: C2RustUnnamed_5 = 1;
pub const COLOR_SCHEME_NONE: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct params_t {
    pub cmd: *mut libc::c_char,
    pub infiles: *mut *mut libc::c_char,
    pub outfile: *mut FILE,
    pub dim: libc::c_int,
    pub shore_depth_tol: libc::c_double,
    pub nrandom: libc::c_int,
    pub show_points: libc::c_int,
    pub bbox_margin: [libc::c_double; 2],
    pub useClusters: libc::c_int,
    pub clusterMethod: libc::c_int,
    pub plotedges: bool,
    pub color_scheme: libc::c_int,
    pub line_width: libc::c_double,
    pub color_scheme_str: *mut libc::c_char,
    pub opacity: *const libc::c_char,
    pub improve_contiguity_n: libc::c_int,
    pub nart: libc::c_int,
    pub color_optimize: bool,
    pub maxcluster: libc::c_int,
    pub nedgep: libc::c_int,
    pub line_color: *const libc::c_char,
    pub include_OK_points: libc::c_int,
    pub highlight_cluster: libc::c_int,
    pub seed: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut usestr: [libc::c_char; 1887] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1887],
        &[libc::c_char; 1887],
    >(
        b"   where graphfile must contain node positions, and widths and heights for each node. No overlap between nodes should be present. Acceptable options are: \n    -a k - average number of artificial points added along the bounding box of the labels. If < 0, a suitable value is selected automatically. (-1)\n    -b v - polygon line width, with v < 0 for no line. (0)\n    -c k - polygon color scheme (1)\n       0 : no polygons\n       1 : pastel (default)\n       2 : blue to yellow\n       3 : white to red\n       4 : light grey to red\n       5 : primary colors\n       6 : sequential single hue red \n       7 : Adam color scheme\n       8 : Adam blend\n       9 : sequential single hue lighter red \n      10 : light grey\n    -c_opacity=xx - 2-character hex string for opacity of polygons\n    -C k - generate at most k clusters. (0)\n    -d s - seed used to calculate Fiedler vector for optimal coloring\n    -D   - use top-level cluster subgraphs to specify clustering\n    -e   - show edges\n    -g c - bounding box color. If not specified, a bounding box is not drawn.\n    -h k - number of artificial points added to maintain bridge between endpoints (0)\n    -highlight=k - only draw cluster k\n    -k   - increase randomness of boundary\n    -l s - specify label\n    -m v - bounding box margin. If 0, auto-assigned (0)\n    -o <file> - put output in <file> (stdout)\n    -O   - do NOT do color assignment optimization that maximizes color difference between neighboring countries\n    -p k - show points. (0)\n       0 : no points\n       1 : all points\n       2 : label points\n       3 : random/artificial points\n    -r k - number of random points k used to define sea and lake boundaries. If 0, auto assigned. (0)\n    -s v - depth of the sea and lake shores in points. If < 0, auto assigned. (0)\n    -t n - improve contiguity up to n times. (0)\n    -v   - verbose\n    -z c - polygon line color (black)\n\0",
    )
};
unsafe extern "C" fn usage(mut cmd: *mut libc::c_char, mut eval: libc::c_int) {
    fprintf(
        stderr,
        b"Usage: %s <options> graphfile\n\0" as *const u8 as *const libc::c_char,
        cmd,
    );
    fputs(usestr.as_ptr(), stderr);
    graphviz_exit(eval);
}
unsafe extern "C" fn openFile(
    mut name: *const libc::c_char,
    mut cmd: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"%s: could not open file %s for writing\n\0" as *const u8
                as *const libc::c_char,
            cmd,
            name,
        );
        graphviz_exit(-(1 as libc::c_int));
    }
    return fp;
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pm: *mut params_t,
) {
    let mut cmd: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut c: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    let mut v: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut stmp: [libc::c_char; 3] = [0; 3];
    let ref mut fresh0 = (*pm).outfile;
    *fresh0 = 0 as *mut FILE;
    let ref mut fresh1 = (*pm).opacity;
    *fresh1 = 0 as *const libc::c_char;
    let ref mut fresh2 = (*pm).color_scheme_str;
    *fresh2 = 0 as *mut libc::c_char;
    (*pm).nrandom = -(1 as libc::c_int);
    (*pm).dim = 2 as libc::c_int;
    (*pm).shore_depth_tol = 0 as libc::c_int as libc::c_double;
    (*pm).highlight_cluster = 0 as libc::c_int;
    (*pm).useClusters = 0 as libc::c_int;
    (*pm).clusterMethod = CLUSTERING_MODULARITY as libc::c_int;
    (*pm).plotedges = 0 as libc::c_int != 0;
    (*pm).show_points = 0 as libc::c_int;
    (*pm).color_scheme = COLOR_SCHEME_PASTEL as libc::c_int;
    (*pm).line_width = 0 as libc::c_int as libc::c_double;
    (*pm).improve_contiguity_n = 0 as libc::c_int;
    (*pm).nart = -(1 as libc::c_int);
    (*pm).color_optimize = 1 as libc::c_int != 0;
    (*pm).maxcluster = 0 as libc::c_int;
    (*pm).nedgep = 0 as libc::c_int;
    let ref mut fresh3 = (*pm).cmd;
    *fresh3 = cmd;
    let ref mut fresh4 = (*pm).infiles;
    *fresh4 = 0 as *mut *mut libc::c_char;
    let ref mut fresh5 = (*pm).line_color;
    *fresh5 = b"#000000\0" as *const u8 as *const libc::c_char;
    (*pm).include_OK_points = 0 as libc::c_int;
    (*pm).seed = 123 as libc::c_int;
    let ref mut fresh6 = (*pm).bbox_margin[1 as libc::c_int as usize];
    *fresh6 = 0 as libc::c_int as libc::c_double;
    (*pm).bbox_margin[0 as libc::c_int as usize] = *fresh6;
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(
            argc,
            argv,
            b":evODQko:m:s:r:p:c:C:l:b:g:t:a:h:z:d:?\0" as *const u8
                as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            109 => {
                if sscanf(
                    optarg,
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut s as *mut libc::c_double,
                ) > 0 as libc::c_int && s != 0 as libc::c_int as libc::c_double
                {
                    let ref mut fresh7 = (*pm).bbox_margin[1 as libc::c_int as usize];
                    *fresh7 = s;
                    (*pm).bbox_margin[0 as libc::c_int as usize] = *fresh7;
                } else {
                    usage(cmd, 1 as libc::c_int);
                }
            }
            81 => {
                (*pm).clusterMethod = CLUSTERING_MQ as libc::c_int;
            }
            115 => {
                if sscanf(
                    optarg,
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut s as *mut libc::c_double,
                ) > 0 as libc::c_int
                {
                    (*pm).shore_depth_tol = s;
                } else {
                    usage(cmd, 1 as libc::c_int);
                }
            }
            104 => {
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_int,
                ) > 0 as libc::c_int
                {
                    (*pm)
                        .nedgep = if 0 as libc::c_int > v {
                        0 as libc::c_int
                    } else {
                        v
                    };
                } else if strncmp(
                        optarg,
                        b"ighlight=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                        && sscanf(
                            optarg
                                .offset(
                                    (::std::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            &mut v as *mut libc::c_int,
                        ) > 0 as libc::c_int
                    {
                    (*pm)
                        .highlight_cluster = if 0 as libc::c_int > v {
                        0 as libc::c_int
                    } else {
                        v
                    };
                } else {
                    usage(cmd, 1 as libc::c_int);
                }
            }
            114 => {
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut r as *mut libc::c_int,
                ) > 0 as libc::c_int
                {
                    (*pm).nrandom = r;
                }
            }
            116 => {
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut r as *mut libc::c_int,
                ) > 0 as libc::c_int && r > 0 as libc::c_int
                {
                    (*pm).improve_contiguity_n = r;
                }
            }
            112 => {
                (*pm).show_points = 1 as libc::c_int;
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut r as *mut libc::c_int,
                ) > 0 as libc::c_int
                {
                    (*pm)
                        .show_points = if (3 as libc::c_int) < r {
                        3 as libc::c_int
                    } else {
                        r
                    };
                }
            }
            107 => {
                (*pm).include_OK_points = 1 as libc::c_int;
            }
            118 => {
                Verbose = 1 as libc::c_int as libc::c_uchar;
            }
            68 => {
                (*pm).useClusters = 1 as libc::c_int;
            }
            101 => {
                (*pm).plotedges = 1 as libc::c_int != 0;
            }
            111 => {
                let ref mut fresh8 = (*pm).outfile;
                *fresh8 = openFile(optarg, (*pm).cmd);
            }
            79 => {
                (*pm).color_optimize = 0 as libc::c_int != 0;
            }
            97 => {
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut r as *mut libc::c_int,
                ) > 0 as libc::c_int
                {
                    (*pm).nart = r;
                } else {
                    usage(cmd, 1 as libc::c_int);
                }
            }
            99 => {
                if sscanf(
                    optarg,
                    b"_opacity=%2s\0" as *const u8 as *const libc::c_char,
                    stmp.as_mut_ptr(),
                ) > 0 as libc::c_int
                    && strlen(stmp.as_mut_ptr()) == 2 as libc::c_int as libc::c_ulong
                {
                    let ref mut fresh9 = (*pm).opacity;
                    *fresh9 = stmp.as_mut_ptr();
                } else if sscanf(
                        optarg,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut r as *mut libc::c_int,
                    ) > 0 as libc::c_int && r >= COLOR_SCHEME_NONE as libc::c_int
                        && r <= COLOR_SCHEME_GREY as libc::c_int
                    {
                    (*pm).color_scheme = r;
                } else if knownColorScheme(optarg) != 0 {
                    (*pm).color_scheme = COLOR_SCHEME_NONE as libc::c_int;
                    let ref mut fresh10 = (*pm).color_scheme_str;
                    *fresh10 = optarg;
                } else {
                    fprintf(
                        stderr,
                        b"-c option %s is invalid, must be a valid integer or string\n\0"
                            as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
            }
            100 => {
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_int,
                ) <= 0 as libc::c_int
                {
                    usage(cmd, 1 as libc::c_int);
                } else {
                    (*pm).seed = v;
                }
            }
            67 => {
                if !(sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_int,
                ) > 0 as libc::c_int && v >= 0 as libc::c_int)
                {
                    usage(cmd, 1 as libc::c_int);
                } else {
                    (*pm).maxcluster = v;
                }
            }
            103 => {}
            122 => {
                let ref mut fresh11 = (*pm).line_color;
                *fresh11 = optarg;
            }
            98 => {
                if sscanf(
                    optarg,
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut s as *mut libc::c_double,
                ) > 0 as libc::c_int
                {
                    (*pm).line_width = s;
                } else {
                    fprintf(
                        stderr,
                        b"%s: unexpected argument \"%s\" for -b flag\n\0" as *const u8
                            as *const libc::c_char,
                        cmd,
                        optarg,
                    );
                }
            }
            58 => {
                fprintf(
                    stderr,
                    b"gvpack: option -%c missing argument - ignored\n\0" as *const u8
                        as *const libc::c_char,
                    optopt,
                );
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    usage(cmd, 0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b" option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        optopt,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
            }
            108 | _ => {}
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc != 0 {
        let ref mut fresh12 = (*pm).infiles;
        *fresh12 = argv;
    }
    if ((*pm).outfile).is_null() {
        let ref mut fresh13 = (*pm).outfile;
        *fresh13 = stdout;
    }
}
unsafe extern "C" fn validateCluster(
    mut n: libc::c_int,
    mut grouping: *mut libc::c_int,
    mut clust_num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if *grouping.offset(i as isize) == clust_num {
            return clust_num;
        }
        i += 1;
    }
    fprintf(
        stderr,
        b"Highlighted cluster %d not found - ignored\n\0" as *const u8
            as *const libc::c_char,
        clust_num,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn makeMap(
    mut graph: SparseMatrix,
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut grouping: *mut libc::c_int,
    mut labels: *mut *mut libc::c_char,
    mut fsz: *mut libc::c_float,
    mut rgb_r: *mut libc::c_float,
    mut rgb_g: *mut libc::c_float,
    mut rgb_b: *mut libc::c_float,
    mut pm: *mut params_t,
    mut g: *mut Agraph_t,
) {
    let mut dim: libc::c_int = (*pm).dim;
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut poly_lines: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut polys: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut poly_point_map: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut edge_bridge_tol: libc::c_double = 0.0f64;
    let mut npolys: libc::c_int = 0;
    let mut nverts: libc::c_int = 0;
    let mut polys_groups: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut exclude_random: libc::c_int = 0;
    let mut x_poly: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xcombined: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut country_graph: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut improve_contiguity_n: libc::c_int = (*pm).improve_contiguity_n;
    let mut nr0: libc::c_int = 0;
    let mut nart0: libc::c_int = 0;
    let mut nart: libc::c_int = 0;
    let mut nrandom: libc::c_int = 0;
    exclude_random = 1 as libc::c_int;
    nrandom = (*pm).nrandom;
    nr0 = nrandom;
    nart = (*pm).nart;
    nart0 = nart;
    if (*pm).highlight_cluster != 0 {
        (*pm).highlight_cluster = validateCluster(n, grouping, (*pm).highlight_cluster);
    }
    make_map_from_rectangle_groups(
        exclude_random,
        (*pm).include_OK_points,
        n,
        dim,
        x,
        width,
        grouping,
        graph,
        ((*pm).bbox_margin).as_mut_ptr(),
        &mut nrandom,
        &mut nart,
        (*pm).nedgep,
        (*pm).shore_depth_tol,
        edge_bridge_tol,
        &mut xcombined,
        &mut nverts,
        &mut x_poly,
        &mut npolys,
        &mut poly_lines,
        &mut polys,
        &mut polys_groups,
        &mut poly_point_map,
        &mut country_graph,
        (*pm).highlight_cluster,
        &mut flag,
    );
    if Verbose != 0 {
        fprintf(stderr, b"nart = %d\n\0" as *const u8 as *const libc::c_char, nart);
    }
    if (*pm).color_optimize as libc::c_int != 0 && !country_graph.is_null()
        && !rgb_r.is_null() && !rgb_g.is_null() && !rgb_b.is_null()
    {
        map_optimal_coloring((*pm).seed, country_graph, rgb_r, rgb_g, rgb_b);
    } else if !((*pm).color_scheme_str).is_null() {
        map_palette_optimal_coloring(
            (*pm).color_scheme_str,
            b"0,100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            country_graph,
            0.01f64,
            -(10 as libc::c_int),
            &mut rgb_r,
            &mut rgb_g,
            &mut rgb_b,
        );
    }
    if !graph.is_null() && improve_contiguity_n != 0 {
        i = 0 as libc::c_int;
        while i < improve_contiguity_n {
            improve_contiguity(n, dim, grouping, poly_point_map, x, graph);
            nart = nart0;
            nrandom = nr0;
            make_map_from_rectangle_groups(
                exclude_random,
                (*pm).include_OK_points,
                n,
                dim,
                x,
                width,
                grouping,
                graph,
                ((*pm).bbox_margin).as_mut_ptr(),
                &mut nrandom,
                &mut nart,
                (*pm).nedgep,
                (*pm).shore_depth_tol,
                edge_bridge_tol,
                &mut xcombined,
                &mut nverts,
                &mut x_poly,
                &mut npolys,
                &mut poly_lines,
                &mut polys,
                &mut polys_groups,
                &mut poly_point_map,
                &mut country_graph,
                (*pm).highlight_cluster,
                &mut flag,
            );
            i += 1;
        }
        if flag == 0 {} else {
            __assert_fail(
                b"!flag\0" as *const u8 as *const libc::c_char,
                b"gvmap.c\0" as *const u8 as *const libc::c_char,
                394 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 128],
                    &[libc::c_char; 128],
                >(
                    b"void makeMap(SparseMatrix, int, double *, double *, int *, char **, float *, float *, float *, float *, params_t *, Agraph_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        let mut D: SparseMatrix = 0 as *mut SparseMatrix_struct;
        D = SparseMatrix_get_real_adjacency_matrix_symmetrized(graph);
        remove_overlap(
            dim,
            D,
            x,
            width,
            1000 as libc::c_int,
            5000.0f64,
            ELSCHEME_NONE as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_int,
            0 as SparseMatrix,
            1 as libc::c_int,
        );
        nart = nart0;
        nrandom = nr0;
        make_map_from_rectangle_groups(
            exclude_random,
            (*pm).include_OK_points,
            n,
            dim,
            x,
            width,
            grouping,
            graph,
            ((*pm).bbox_margin).as_mut_ptr(),
            &mut nrandom,
            &mut nart,
            (*pm).nedgep,
            (*pm).shore_depth_tol,
            edge_bridge_tol,
            &mut xcombined,
            &mut nverts,
            &mut x_poly,
            &mut npolys,
            &mut poly_lines,
            &mut polys,
            &mut polys_groups,
            &mut poly_point_map,
            &mut country_graph,
            (*pm).highlight_cluster,
            &mut flag,
        );
        if flag == 0 {} else {
            __assert_fail(
                b"!flag\0" as *const u8 as *const libc::c_char,
                b"gvmap.c\0" as *const u8 as *const libc::c_char,
                408 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 128],
                    &[libc::c_char; 128],
                >(
                    b"void makeMap(SparseMatrix, int, double *, double *, int *, char **, float *, float *, float *, float *, params_t *, Agraph_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    Dot_SetClusterColor(g, rgb_r, rgb_g, rgb_b, grouping);
    plot_dot_map(
        g,
        n,
        dim,
        x,
        polys,
        poly_lines,
        (*pm).line_width,
        (*pm).line_color,
        x_poly,
        polys_groups,
        labels,
        fsz,
        rgb_r,
        rgb_g,
        rgb_b,
        (*pm).opacity,
        if (*pm).plotedges as libc::c_int != 0 { graph } else { 0 as SparseMatrix },
        (*pm).outfile,
    );
    SparseMatrix_delete(polys);
    SparseMatrix_delete(poly_lines);
    SparseMatrix_delete(poly_point_map);
    free(xcombined as *mut libc::c_void);
    free(x_poly as *mut libc::c_void);
    free(polys_groups as *mut libc::c_void);
}
unsafe extern "C" fn mapFromGraph(mut g: *mut Agraph_t, mut pm: *mut params_t) {
    let mut graph: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut n: libc::c_int = 0;
    let mut width: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut labels: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut grouping: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rgb_r: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut rgb_g: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut rgb_b: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut fsz: *mut libc::c_float = 0 as *mut libc::c_float;
    initDotIO(g);
    graph = Import_coord_clusters_from_dot(
        g,
        (*pm).maxcluster,
        (*pm).dim,
        &mut n,
        &mut width,
        &mut x,
        &mut grouping,
        &mut rgb_r,
        &mut rgb_g,
        &mut rgb_b,
        &mut fsz,
        &mut labels,
        (*pm).color_scheme,
        (*pm).clusterMethod,
        (*pm).useClusters,
    );
    makeMap(graph, n, x, width, grouping, labels, fsz, rgb_r, rgb_g, rgb_b, pm, g);
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pm: params_t = params_t {
        cmd: 0 as *mut libc::c_char,
        infiles: 0 as *mut *mut libc::c_char,
        outfile: 0 as *mut FILE,
        dim: 0,
        shore_depth_tol: 0.,
        nrandom: 0,
        show_points: 0,
        bbox_margin: [0.; 2],
        useClusters: 0,
        clusterMethod: 0,
        plotedges: false,
        color_scheme: 0,
        line_width: 0.,
        color_scheme_str: 0 as *mut libc::c_char,
        opacity: 0 as *const libc::c_char,
        improve_contiguity_n: 0,
        nart: 0,
        color_optimize: false,
        maxcluster: 0,
        nedgep: 0,
        line_color: 0 as *const libc::c_char,
        include_OK_points: 0,
        highlight_cluster: 0,
        seed: 0,
    };
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut prevg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut ig: ingraph_state = ingraph_state {
        u: C2RustUnnamed_4 {
            Files: 0 as *mut *mut libc::c_char,
        },
        ctr: 0,
        ingraphs: 0,
        fp: 0 as *mut libc::c_void,
        fns: 0 as *mut ingdisc,
        heap: false,
        errors: 0,
    };
    init(argc, argv, &mut pm);
    newIngraph(
        &mut ig,
        pm.infiles,
        Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
    );
    loop {
        g = nextGraph(&mut ig);
        if g.is_null() {
            break;
        }
        if !prevg.is_null() {
            agclose(prevg);
        }
        mapFromGraph(g, &mut pm);
        prevg = g;
    }
    graphviz_exit(0 as libc::c_int);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
