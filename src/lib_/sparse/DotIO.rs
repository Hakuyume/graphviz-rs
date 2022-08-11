#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut Verbose: libc::c_uchar;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn SparseMatrix_from_coordinate_arrays(
        nz: libc::c_int,
        m: libc::c_int,
        n: libc::c_int,
        irn: *mut libc::c_int,
        jcn: *mut libc::c_int,
        val: *mut libc::c_void,
        type_0: libc::c_int,
        sz: size_t,
    ) -> SparseMatrix;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn modularity_clustering(
        A: SparseMatrix,
        inplace: libc::c_int,
        maxcluster: libc::c_int,
        use_value: libc::c_int,
        nclusters: *mut libc::c_int,
        assignment: *mut *mut libc::c_int,
        modularity: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    fn mq_clustering(
        A: SparseMatrix,
        inplace: libc::c_int,
        maxcluster: libc::c_int,
        use_value: libc::c_int,
        nclusters: *mut libc::c_int,
        assignment: *mut *mut libc::c_int,
        mq: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    static mut palette_pastel: [[libc::c_float; 3]; 1001];
    static mut palette_blue_to_yellow: [[libc::c_float; 3]; 1001];
    static mut palette_grey_to_red: [[libc::c_float; 3]; 1001];
    static mut palette_white_to_red: [[libc::c_float; 3]; 1001];
    static mut palette_grey: [[libc::c_float; 3]; 1001];
    static mut palette_primary: [[libc::c_float; 3]; 1001];
    static mut palette_sequential_singlehue_red: [[libc::c_float; 3]; 1001];
    static mut palette_sequential_singlehue_red_lighter: [[libc::c_float; 3]; 1001];
    static mut palette_adam_blend: [[libc::c_float; 3]; 1001];
    static mut palette_adam: [[libc::c_float; 3]; 11];
    fn colorxlate(
        str: *mut libc::c_char,
        color: *mut gvcolor_t,
        target_type: color_type_t,
    ) -> libc::c_int;
    fn rgb2hex(
        r: libc::c_float,
        g: libc::c_float,
        b: libc::c_float,
        cstring: *mut libc::c_char,
        opacity: *const libc::c_char,
    );
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
pub type Agnode_t = Agnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agedge_s {
    pub base: Agobj_t,
    pub id_link: Dtlink_t,
    pub seq_link: Dtlink_t,
    pub node: *mut Agnode_t,
}
pub type Agedge_t = Agedge_s;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const FORMAT_COORD: C2RustUnnamed_2 = 2;
pub const FORMAT_CSR: C2RustUnnamed_2 = 1;
pub const FORMAT_CSC: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_3 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_3 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_3 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_3 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const COLOR_SCHEME_GREY: C2RustUnnamed_4 = 10;
pub const COLOR_SCHEME_SEQUENTIAL_SINGLEHUE_RED_LIGHTER: C2RustUnnamed_4 = 9;
pub const COLOR_SCHEME_ADAM_BLEND: C2RustUnnamed_4 = 8;
pub const COLOR_SCHEME_ADAM: C2RustUnnamed_4 = 7;
pub const COLOR_SCHEME_SEQUENTIAL_SINGLEHUE_RED: C2RustUnnamed_4 = 6;
pub const COLOR_SCHEME_PRIMARY: C2RustUnnamed_4 = 5;
pub const COLOR_SCHEME_GREY_RED: C2RustUnnamed_4 = 4;
pub const COLOR_SCHEME_WHITE_RED: C2RustUnnamed_4 = 3;
pub const COLOR_SCHEME_BLUE_YELLOW: C2RustUnnamed_4 = 2;
pub const COLOR_SCHEME_PASTEL: C2RustUnnamed_4 = 1;
pub const COLOR_SCHEME_NONE: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnodeinfo_t {
    pub h: Agrec_t,
    pub id: libc::c_uint,
}
pub const slen: C2RustUnnamed_5 = 1024;
pub type C2RustUnnamed_5 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub RGBA: [libc::c_double; 4],
    pub HSVA: [libc::c_double; 4],
    pub rgba: [libc::c_uchar; 4],
    pub cmyk: [libc::c_uchar; 4],
    pub rrggbbaa: [libc::c_int; 4],
    pub string: *mut libc::c_char,
    pub index: libc::c_int,
}
pub type gvcolor_t = color_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_s {
    pub u: C2RustUnnamed_6,
    pub type_0: color_type_t,
}
pub type color_type_t = libc::c_uint;
pub const COLOR_INDEX: color_type_t = 6;
pub const COLOR_STRING: color_type_t = 5;
pub const RGBA_DOUBLE: color_type_t = 4;
pub const CMYK_BYTE: color_type_t = 3;
pub const RGBA_WORD: color_type_t = 2;
pub const RGBA_BYTE: color_type_t = 1;
pub const HSVA_DOUBLE: color_type_t = 0;
pub const MAX_COLOR: C2RustUnnamed_8 = 1001;
pub const CLUSTERING_MODULARITY: C2RustUnnamed_7 = 0;
pub const CLUSTERING_MQ: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_7 = libc::c_uint;
pub type C2RustUnnamed_8 = libc::c_uint;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
unsafe extern "C" fn color_string(
    mut slen_0: libc::c_int,
    mut buf: *mut libc::c_char,
    mut dim: libc::c_int,
    mut color: *mut libc::c_double,
) {
    if dim > 3 as libc::c_int || dim < 1 as libc::c_int {
        fprintf(
            stderr,
            b"can only 1, 2 or 3 dimensional color space. with color value between 0 to 1\n\0"
                as *const u8 as *const libc::c_char,
        );
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"DotIO.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void color_string(int, char *, int, double *)\0"))
                .as_ptr(),
        );
    }
    if slen_0 >= 3 as libc::c_int {} else {
        __assert_fail(
            b"slen >= 3\0" as *const u8 as *const libc::c_char,
            b"DotIO.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void color_string(int, char *, int, double *)\0"))
                .as_ptr(),
        );
    }
    if dim == 3 as libc::c_int {
        sprintf(
            buf,
            b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            if ((*color.offset(0 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(0 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
            if ((*color.offset(1 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(1 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
            if ((*color.offset(2 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(2 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
        );
    } else if dim == 1 as libc::c_int {
        sprintf(
            buf,
            b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            if ((*color.offset(0 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(0 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
            if ((*color.offset(0 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(0 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
            if ((*color.offset(0 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(0 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
        );
    } else if dim == 2 as libc::c_int {
        sprintf(
            buf,
            b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            if ((*color.offset(0 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(0 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
            0 as libc::c_int,
            if ((*color.offset(1 as libc::c_int as isize)
                * 255 as libc::c_int as libc::c_double) as libc::c_uint)
                < 255 as libc::c_int as libc::c_uint
            {
                (*color.offset(1 as libc::c_int as isize)
                    * 255 as libc::c_int as libc::c_double) as libc::c_uint
            } else {
                255 as libc::c_int as libc::c_uint
            },
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn attach_edge_colors(
    mut g: *mut Agraph_t,
    mut dim: libc::c_int,
    mut colors: *mut libc::c_double,
) {
    let mut sym: *mut Agsym_t = agattr(
        g,
        2 as libc::c_int,
        b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut row: libc::c_uint = 0;
    let mut col: libc::c_uint = 0;
    let mut ie: libc::c_int = 0 as libc::c_int;
    if sym.is_null() {
        sym = agattr(
            g,
            2 as libc::c_int,
            b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    n = agfstnode(g);
    while !n.is_null() {
        row = (*((*n).base.data as *mut Agnodeinfo_t)).id;
        e = agfstout(g, n);
        while !e.is_null() {
            col = (*((*(*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node)
                .base
                .data as *mut Agnodeinfo_t))
                .id;
            if !(row == col) {
                color_string(
                    slen as libc::c_int,
                    buf.as_mut_ptr(),
                    dim,
                    colors.offset((ie * dim) as isize),
                );
                agxset(e as *mut libc::c_void, sym, buf.as_mut_ptr());
                ie += 1;
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_import_dot(
    mut g: *mut Agraph_t,
    mut dim: libc::c_int,
    mut label_sizes: *mut *mut libc::c_double,
    mut x: *mut *mut libc::c_double,
    mut n_edge_label_nodes: *mut libc::c_int,
    mut edge_label_nodes: *mut *mut libc::c_int,
    mut format: libc::c_int,
) -> SparseMatrix {
    let mut sz_0: size_t = 0;
    let mut current_block: u64;
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut psym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut nnodes: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut I: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut J: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_double = 0.;
    let mut type_0: libc::c_int = MATRIX_TYPE_REAL as libc::c_int;
    let mut padding: libc::c_double = 10 as libc::c_int as libc::c_double;
    let mut nedge_nodes: libc::c_int = 0 as libc::c_int;
    if g.is_null() {
        return 0 as SparseMatrix;
    }
    nnodes = agnnodes(g);
    nedges = agnedges(g);
    if format != FORMAT_CSR as libc::c_int && format != FORMAT_COORD as libc::c_int {
        fprintf(
            stderr,
            b"Format %d not supported\n\0" as *const u8 as *const libc::c_char,
            format,
        );
        graphviz_exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        let fresh0 = i;
        i = i + 1;
        (*((*n).base.data as *mut Agnodeinfo_t)).id = fresh0 as libc::c_uint;
        n = agnxtnode(g, n);
    }
    if format == FORMAT_COORD as libc::c_int {
        A = SparseMatrix_new(i, i, nedges, MATRIX_TYPE_REAL as libc::c_int, format);
        (*A).nz = nedges;
        I = (*A).ia;
        J = (*A).ja;
        val = (*A).a as *mut libc::c_double;
    } else {
        I = calloc(
            nedges as libc::c_ulong,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        J = calloc(
            nedges as libc::c_ulong,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        val = calloc(
            nedges as libc::c_ulong,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
    }
    sym = agattr(
        g,
        2 as libc::c_int,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        if !edge_label_nodes.is_null()
            && strncmp(
                agnameof(n as *mut libc::c_void),
                b"|edgelabel|\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            nedge_nodes += 1;
        }
        row = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
        e = agfstout(g, n);
        while !e.is_null() {
            *I.offset(i as isize) = row;
            *J
                .offset(
                    i as isize,
                ) = (*((*(*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node)
                .base
                .data as *mut Agnodeinfo_t))
                .id as libc::c_int;
            if !sym.is_null() {
                if sscanf(
                    agxget(e as *mut libc::c_void, sym),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_double,
                ) != 1 as libc::c_int
                {
                    v = 1 as libc::c_int as libc::c_double;
                }
            } else {
                v = 1 as libc::c_int as libc::c_double;
            }
            *val.offset(i as isize) = v;
            i += 1;
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    if !edge_label_nodes.is_null() {
        *edge_label_nodes = malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nedge_nodes as libc::c_ulong),
        ) as *mut libc::c_int;
        nedge_nodes = 0 as libc::c_int;
    }
    if !label_sizes.is_null() {
        *label_sizes = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(nnodes as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    n = agfstnode(g);
    while !n.is_null() {
        let mut sz: libc::c_double = 0.;
        i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
        if !edge_label_nodes.is_null()
            && strncmp(
                agnameof(n as *mut libc::c_void),
                b"|edgelabel|\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            let fresh1 = nedge_nodes;
            nedge_nodes = nedge_nodes + 1;
            *(*edge_label_nodes).offset(fresh1 as isize) = i;
        }
        if !label_sizes.is_null() {
            if !(agget(
                n as *mut libc::c_void,
                b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ))
                .is_null()
                && !(agget(
                    n as *mut libc::c_void,
                    b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ))
                    .is_null()
            {
                sscanf(
                    agget(
                        n as *mut libc::c_void,
                        b"width\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut sz as *mut libc::c_double,
                );
                *(*label_sizes)
                    .offset(
                        (i * 2 as libc::c_int) as isize,
                    ) = 72 as libc::c_int as libc::c_double * sz * 0.5f64
                    + padding * 0.5f64;
                sscanf(
                    agget(
                        n as *mut libc::c_void,
                        b"height\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut sz as *mut libc::c_double,
                );
                *(*label_sizes)
                    .offset(
                        (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                    ) = 72 as libc::c_int as libc::c_double * sz * 0.5f64
                    + padding * 0.5f64;
            } else {
                *(*label_sizes)
                    .offset(
                        (i * 2 as libc::c_int) as isize,
                    ) = (4 as libc::c_int * 72 as libc::c_int) as libc::c_double
                    * 0.75f64 * 0.5f64;
                *(*label_sizes)
                    .offset(
                        (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                    ) = (4 as libc::c_int * 72 as libc::c_int) as libc::c_double * 0.5f64
                    * 0.5f64;
            }
        }
        n = agnxtnode(g, n);
    }
    if !x.is_null()
        && {
            psym = agattr(
                g,
                1 as libc::c_int,
                b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char,
            );
            !psym.is_null()
        }
    {
        let mut has_positions: bool = 1 as libc::c_int != 0;
        let mut pval: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*x).is_null() {
            *x = malloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(dim as libc::c_ulong)
                    .wrapping_mul(nnodes as libc::c_ulong),
            ) as *mut libc::c_double;
            if !(*x).is_null() {} else {
                __assert_fail(
                    b"*x\0" as *const u8 as *const libc::c_char,
                    b"DotIO.c\0" as *const u8 as *const libc::c_char,
                    177 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"SparseMatrix SparseMatrix_import_dot(Agraph_t *, int, double **, double **, int *, int **, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
        n = agfstnode(g);
        loop {
            if !(!n.is_null() && has_positions as libc::c_int != 0) {
                current_block = 13253659531982233645;
                break;
            }
            let mut xx: libc::c_double = 0.;
            let mut yy: libc::c_double = 0.;
            let mut zz: libc::c_double = 0.;
            let mut ww: libc::c_double = 0.;
            let mut nitems: libc::c_int = 0;
            i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
            pval = agxget(n as *mut libc::c_void, psym);
            if !pval.is_null() && *pval as libc::c_int != 0 {
                if dim == 2 as libc::c_int {
                    nitems = sscanf(
                        pval,
                        b"%lf,%lf\0" as *const u8 as *const libc::c_char,
                        &mut xx as *mut libc::c_double,
                        &mut yy as *mut libc::c_double,
                    );
                    if nitems != 2 as libc::c_int {
                        has_positions = 0 as libc::c_int != 0;
                        agerr(
                            AGERR,
                            b"Node \"%s\" pos has %d < 2 values\0" as *const u8
                                as *const libc::c_char,
                            agnameof(n as *mut libc::c_void),
                            nitems,
                        );
                    }
                    *(*x).offset((i * dim) as isize) = xx;
                    *(*x).offset((i * dim + 1 as libc::c_int) as isize) = yy;
                } else if dim == 3 as libc::c_int {
                    nitems = sscanf(
                        pval,
                        b"%lf,%lf,%lf\0" as *const u8 as *const libc::c_char,
                        &mut xx as *mut libc::c_double,
                        &mut yy as *mut libc::c_double,
                        &mut zz as *mut libc::c_double,
                    );
                    if nitems != 3 as libc::c_int {
                        has_positions = 0 as libc::c_int != 0;
                        agerr(
                            AGERR,
                            b"Node \"%s\" pos has %d < 3 values\0" as *const u8
                                as *const libc::c_char,
                            agnameof(n as *mut libc::c_void),
                            nitems,
                        );
                    }
                    *(*x).offset((i * dim) as isize) = xx;
                    *(*x).offset((i * dim + 1 as libc::c_int) as isize) = yy;
                    *(*x).offset((i * dim + 2 as libc::c_int) as isize) = zz;
                } else if dim == 4 as libc::c_int {
                    nitems = sscanf(
                        pval,
                        b"%lf,%lf,%lf,%lf\0" as *const u8 as *const libc::c_char,
                        &mut xx as *mut libc::c_double,
                        &mut yy as *mut libc::c_double,
                        &mut zz as *mut libc::c_double,
                        &mut ww as *mut libc::c_double,
                    );
                    if nitems != 4 as libc::c_int {
                        has_positions = 0 as libc::c_int != 0;
                        agerr(
                            AGERR,
                            b"Node \"%s\" pos has %d < 4 values\0" as *const u8
                                as *const libc::c_char,
                            agnameof(n as *mut libc::c_void),
                            nitems,
                        );
                    }
                    *(*x).offset((i * dim) as isize) = xx;
                    *(*x).offset((i * dim + 1 as libc::c_int) as isize) = yy;
                    *(*x).offset((i * dim + 2 as libc::c_int) as isize) = zz;
                    *(*x).offset((i * dim + 3 as libc::c_int) as isize) = ww;
                } else if dim == 1 as libc::c_int {
                    nitems = sscanf(
                        pval,
                        b"%lf\0" as *const u8 as *const libc::c_char,
                        &mut xx as *mut libc::c_double,
                    );
                    if nitems != 1 as libc::c_int {
                        A = 0 as SparseMatrix;
                        current_block = 14442495880161386404;
                        break;
                    } else {
                        *(*x).offset((i * dim) as isize) = xx;
                    }
                } else {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"DotIO.c\0" as *const u8 as *const libc::c_char,
                        219 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 96],
                            &[libc::c_char; 96],
                        >(
                            b"SparseMatrix SparseMatrix_import_dot(Agraph_t *, int, double **, double **, int *, int **, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            } else {
                has_positions = 0 as libc::c_int != 0;
                agerr(
                    AGERR,
                    b"Node \"%s\" lacks position info\0" as *const u8
                        as *const libc::c_char,
                    agnameof(n as *mut libc::c_void),
                );
            }
            n = agnxtnode(g, n);
        }
        match current_block {
            14442495880161386404 => {}
            _ => {
                if !has_positions {
                    free(*x as *mut libc::c_void);
                    *x = 0 as *mut libc::c_double;
                }
                current_block = 16696653877814833746;
            }
        }
    } else {
        if !x.is_null() {
            agerr(
                AGERR,
                b"Error: graph %s has missing \"pos\" information\0" as *const u8
                    as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
            );
        }
        current_block = 16696653877814833746;
    }
    match current_block {
        16696653877814833746 => {
            sz_0 = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
            if format == FORMAT_CSR as libc::c_int {
                A = SparseMatrix_from_coordinate_arrays(
                    nedges,
                    nnodes,
                    nnodes,
                    I,
                    J,
                    val as *mut libc::c_void,
                    type_0,
                    sz_0,
                );
            }
            if !edge_label_nodes.is_null() {
                *n_edge_label_nodes = nedge_nodes;
            }
        }
        _ => {}
    }
    if format != FORMAT_COORD as libc::c_int {
        free(I as *mut libc::c_void);
        free(J as *mut libc::c_void);
        free(val as *mut libc::c_void);
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn Import_dot_splines(
    mut g: *mut Agraph_t,
    mut ne: *mut libc::c_int,
    mut xsplines: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut nedges: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    if g.is_null() {
        return 0 as libc::c_int;
    }
    nedges = agnedges(g);
    *ne = nedges;
    i = 0 as libc::c_int as libc::c_uint;
    n = agfstnode(g);
    while !n.is_null() {
        let fresh2 = i;
        i = i.wrapping_add(1);
        (*((*n).base.data as *mut Agnodeinfo_t)).id = fresh2;
        n = agnxtnode(g, n);
    }
    sym = agattr(
        g,
        2 as libc::c_int,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if sym.is_null() {
        return 0 as libc::c_int;
    }
    if (*xsplines).is_null() {
        *xsplines = malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(nedges as libc::c_ulong),
        ) as *mut *mut libc::c_char;
    }
    i = 0 as libc::c_int as libc::c_uint;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            if !sym.is_null() {
                let mut pos: *mut libc::c_char = agxget(e as *mut libc::c_void, sym);
                let ref mut fresh3 = *(*xsplines).offset(i as isize);
                *fresh3 = strdup(pos);
            } else {
                let ref mut fresh4 = *(*xsplines).offset(i as isize);
                *fresh4 = 0 as *mut libc::c_char;
            }
            i = i.wrapping_add(1);
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn hex2int(mut h: libc::c_char) -> libc::c_int {
    if h as libc::c_int >= '0' as i32 && h as libc::c_int <= '9' as i32 {
        return h as libc::c_int - '0' as i32;
    }
    if h as libc::c_int >= 'a' as i32 && h as libc::c_int <= 'f' as i32 {
        return 10 as libc::c_int + h as libc::c_int - 'a' as i32;
    }
    if h as libc::c_int >= 'A' as i32 && h as libc::c_int <= 'F' as i32 {
        return 10 as libc::c_int + h as libc::c_int - 'A' as i32;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn hexcol2rgb(mut h: *mut libc::c_char) -> libc::c_float {
    return ((hex2int(*h.offset(0 as libc::c_int as isize)) * 16 as libc::c_int
        + hex2int(*h.offset(1 as libc::c_int as isize))) as libc::c_double / 255.0f64)
        as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Dot_SetClusterColor(
    mut g: *mut Agraph_t,
    mut rgb_r: *mut libc::c_float,
    mut rgb_g: *mut libc::c_float,
    mut rgb_b: *mut libc::c_float,
    mut clusters: *mut libc::c_int,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut scluster: [libc::c_char; 20] = [0; 20];
    let mut i: libc::c_uint = 0;
    let mut clust_clr_sym: *mut Agsym_t = agattr(
        g,
        1 as libc::c_int,
        b"clustercolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if clust_clr_sym.is_null() {
        clust_clr_sym = agattr(
            g,
            1 as libc::c_int,
            b"clustercolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"-1\0" as *const u8 as *const libc::c_char,
        );
    }
    n = agfstnode(g);
    while !n.is_null() {
        i = (*((*n).base.data as *mut Agnodeinfo_t)).id;
        if !rgb_r.is_null() && !rgb_g.is_null() && !rgb_b.is_null() {
            rgb2hex(
                *rgb_r.offset(*clusters.offset(i as isize) as isize),
                *rgb_g.offset(*clusters.offset(i as isize) as isize),
                *rgb_b.offset(*clusters.offset(i as isize) as isize),
                scluster.as_mut_ptr(),
                0 as *const libc::c_char,
            );
        }
        agxset(n as *mut libc::c_void, clust_clr_sym, scluster.as_mut_ptr());
        n = agnxtnode(g, n);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Import_coord_clusters_from_dot(
    mut g: *mut Agraph_t,
    mut maxcluster: libc::c_int,
    mut dim: libc::c_int,
    mut nn: *mut libc::c_int,
    mut label_sizes: *mut *mut libc::c_double,
    mut x: *mut *mut libc::c_double,
    mut clusters: *mut *mut libc::c_int,
    mut rgb_r: *mut *mut libc::c_float,
    mut rgb_g: *mut *mut libc::c_float,
    mut rgb_b: *mut *mut libc::c_float,
    mut fsz: *mut *mut libc::c_float,
    mut labels: *mut *mut *mut libc::c_char,
    mut default_color_scheme: libc::c_int,
    mut clustering_scheme: libc::c_int,
    mut useClusters: libc::c_int,
) -> SparseMatrix {
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut clust_sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut clust_clr_sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut nnodes: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut ic: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut I: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut J: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_double = 0.;
    let mut type_0: libc::c_int = MATRIX_TYPE_REAL as libc::c_int;
    let mut scluster: [libc::c_char; 100] = [0; 100];
    let mut ff: libc::c_float = 0.;
    let mut MAX_GRPS: libc::c_int = 0;
    let mut MIN_GRPS: libc::c_int = 0;
    let mut noclusterinfo: bool = 0 as libc::c_int != 0;
    let mut first: bool = 1 as libc::c_int != 0;
    let mut pal: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut max_color: libc::c_int = MAX_COLOR as libc::c_int;
    match default_color_scheme {
        2 => {
            pal = &mut *(*palette_blue_to_yellow
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        3 => {
            pal = &mut *(*palette_white_to_red
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        4 => {
            pal = &mut *(*palette_grey_to_red
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        10 => {
            pal = &mut *(*palette_grey.as_mut_ptr().offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        1 => {
            pal = &mut *(*palette_pastel.as_mut_ptr().offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        6 => {
            fprintf(stderr, b" HERE!\n\0" as *const u8 as *const libc::c_char);
            pal = &mut *(*palette_sequential_singlehue_red
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        9 => {
            fprintf(stderr, b" HERE!\n\0" as *const u8 as *const libc::c_char);
            pal = &mut *(*palette_sequential_singlehue_red_lighter
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        5 => {
            pal = &mut *(*palette_primary.as_mut_ptr().offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        8 => {
            pal = &mut *(*palette_adam_blend
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
        7 => {
            pal = &mut *(*palette_adam.as_mut_ptr().offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
            max_color = 11 as libc::c_int;
        }
        0 => {
            pal = 0 as *mut libc::c_float;
        }
        _ => {
            pal = &mut *(*palette_pastel.as_mut_ptr().offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_float;
        }
    }
    if g.is_null() {
        return 0 as SparseMatrix;
    }
    nnodes = agnnodes(g);
    nedges = agnedges(g);
    *nn = nnodes;
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        let fresh5 = i;
        i = i + 1;
        (*((*n).base.data as *mut Agnodeinfo_t)).id = fresh5 as libc::c_uint;
        n = agnxtnode(g, n);
    }
    I = calloc(
        nedges as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    J = calloc(
        nedges as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    val = calloc(
        nedges as libc::c_ulong,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    sym = agattr(
        g,
        2 as libc::c_int,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    clust_sym = agattr(
        g,
        1 as libc::c_int,
        b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    clust_clr_sym = agattr(
        g,
        1 as libc::c_int,
        b"clustercolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        row = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
        e = agfstout(g, n);
        while !e.is_null() {
            *I.offset(i as isize) = row;
            *J
                .offset(
                    i as isize,
                ) = (*((*(*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node)
                .base
                .data as *mut Agnodeinfo_t))
                .id as libc::c_int;
            if !sym.is_null() {
                if sscanf(
                    agxget(e as *mut libc::c_void, sym),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_double,
                ) != 1 as libc::c_int
                {
                    v = 1 as libc::c_int as libc::c_double;
                }
            } else {
                v = 1 as libc::c_int as libc::c_double;
            }
            *val.offset(i as isize) = v;
            i += 1;
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    A = SparseMatrix_from_coordinate_arrays(
        nedges,
        nnodes,
        nnodes,
        I,
        J,
        val as *mut libc::c_void,
        type_0,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    *clusters = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nnodes as libc::c_ulong),
    ) as *mut libc::c_int;
    nc = 1 as libc::c_int;
    MIN_GRPS = 0 as libc::c_int;
    if useClusters != 0 {
        let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
        let mut gid: libc::c_int = 1 as libc::c_int;
        memset(
            *clusters as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nnodes as libc::c_ulong),
        );
        sg = agfstsubg(g);
        while !sg.is_null() {
            if !(strncmp(
                agnameof(sg as *mut libc::c_void),
                b"cluster\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) != 0)
            {
                gid += 1;
                n = agfstnode(sg);
                while !n.is_null() {
                    i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
                    if *(*clusters).offset(i as isize) != 0 {
                        fprintf(
                            stderr,
                            b"Warning: node %s appears in multiple clusters.\n\0"
                                as *const u8 as *const libc::c_char,
                            agnameof(n as *mut libc::c_void),
                        );
                    } else {
                        *(*clusters).offset(i as isize) = gid;
                    }
                    n = agnxtnode(sg, n);
                }
            }
            sg = agnxtsubg(sg);
        }
        n = agfstnode(g);
        while !n.is_null() {
            i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
            if *(*clusters).offset(i as isize) == 0 as libc::c_int {
                *(*clusters).offset(i as isize) = 1 as libc::c_int;
            }
            n = agnxtnode(g, n);
        }
        MIN_GRPS = 1 as libc::c_int;
        nc = gid;
    } else if !clust_sym.is_null() {
        n = agfstnode(g);
        while !n.is_null() {
            i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
            if sscanf(
                agxget(n as *mut libc::c_void, clust_sym),
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut ic as *mut libc::c_int,
            ) > 0 as libc::c_int
            {
                *(*clusters).offset(i as isize) = ic;
                nc = if nc > ic { nc } else { ic };
                if first {
                    MIN_GRPS = ic;
                    first = 0 as libc::c_int != 0;
                } else {
                    MIN_GRPS = if MIN_GRPS < ic { MIN_GRPS } else { ic };
                }
                n = agnxtnode(g, n);
            } else {
                noclusterinfo = 1 as libc::c_int != 0;
                break;
            }
        }
    } else {
        noclusterinfo = 1 as libc::c_int != 0;
    }
    MAX_GRPS = nc;
    if noclusterinfo {
        let mut use_value: libc::c_int = 1 as libc::c_int;
        let mut flag: libc::c_int = 0 as libc::c_int;
        let mut modularity: libc::c_double = 0.;
        if clust_sym.is_null() {
            clust_sym = agattr(
                g,
                1 as libc::c_int,
                b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"-1\0" as *const u8 as *const libc::c_char,
            );
        }
        if clustering_scheme == CLUSTERING_MQ as libc::c_int {
            mq_clustering(
                A,
                0 as libc::c_int,
                maxcluster,
                use_value,
                &mut nc,
                clusters,
                &mut modularity,
                &mut flag,
            );
        } else if clustering_scheme == CLUSTERING_MODULARITY as libc::c_int {
            modularity_clustering(
                A,
                0 as libc::c_int,
                maxcluster,
                use_value,
                &mut nc,
                clusters,
                &mut modularity,
                &mut flag,
            );
        } else {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"DotIO.c\0" as *const u8 as *const libc::c_char,
                487 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 168],
                    &[libc::c_char; 168],
                >(
                    b"SparseMatrix Import_coord_clusters_from_dot(Agraph_t *, int, int, int *, double **, double **, int **, float **, float **, float **, float **, char ***, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        i = 0 as libc::c_int;
        while i < nnodes {
            let ref mut fresh6 = *(*clusters).offset(i as isize);
            *fresh6 += 1;
            i += 1;
        }
        n = agfstnode(g);
        while !n.is_null() {
            i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
            snprintf(
                scluster.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                *(*clusters).offset(i as isize),
            );
            agxset(n as *mut libc::c_void, clust_sym, scluster.as_mut_ptr());
            n = agnxtnode(g, n);
        }
        MIN_GRPS = 1 as libc::c_int;
        MAX_GRPS = nc;
        if Verbose != 0 {
            fprintf(
                stderr,
                b" no complement clustering info in dot file, using modularity clustering. Modularity = %f, ncluster=%d\n\0"
                    as *const u8 as *const libc::c_char,
                modularity,
                nc,
            );
        }
    }
    *label_sizes = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dim as libc::c_ulong)
            .wrapping_mul(nnodes as libc::c_ulong),
    ) as *mut libc::c_double;
    if !pal.is_null() || !noclusterinfo && !clust_clr_sym.is_null() {
        *rgb_r = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul((1 as libc::c_int + MAX_GRPS) as libc::c_ulong),
        ) as *mut libc::c_float;
        *rgb_g = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul((1 as libc::c_int + MAX_GRPS) as libc::c_ulong),
        ) as *mut libc::c_float;
        *rgb_b = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul((1 as libc::c_int + MAX_GRPS) as libc::c_ulong),
        ) as *mut libc::c_float;
    } else {
        *rgb_r = 0 as *mut libc::c_float;
        *rgb_g = 0 as *mut libc::c_float;
        *rgb_b = 0 as *mut libc::c_float;
    }
    *fsz = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(nnodes as libc::c_ulong),
    ) as *mut libc::c_float;
    *labels = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nnodes as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    n = agfstnode(g);
    while !n.is_null() {
        let mut color: gvcolor_t = gvcolor_t {
            u: C2RustUnnamed_6 { RGBA: [0.; 4] },
            type_0: HSVA_DOUBLE,
        };
        let mut sz: libc::c_double = 0.;
        i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
        if !(agget(
            n as *mut libc::c_void,
            b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .is_null()
            && !(agget(
                n as *mut libc::c_void,
                b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ))
                .is_null()
        {
            sscanf(
                agget(
                    n as *mut libc::c_void,
                    b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
                b"%lf\0" as *const u8 as *const libc::c_char,
                &mut sz as *mut libc::c_double,
            );
            *(*label_sizes)
                .offset(
                    (i * 2 as libc::c_int) as isize,
                ) = 72 as libc::c_int as libc::c_double * (sz * 0.5f64);
            sscanf(
                agget(
                    n as *mut libc::c_void,
                    b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
                b"%lf\0" as *const u8 as *const libc::c_char,
                &mut sz as *mut libc::c_double,
            );
            *(*label_sizes)
                .offset(
                    (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = 72 as libc::c_int as libc::c_double * (sz * 0.5f64);
        } else {
            *(*label_sizes)
                .offset(
                    (i * 2 as libc::c_int) as isize,
                ) = 72 as libc::c_int as libc::c_double
                * (0.75f64 / 2 as libc::c_int as libc::c_double);
            *(*label_sizes)
                .offset(
                    (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = 72 as libc::c_int as libc::c_double
                * (0.5f64 * 2 as libc::c_int as libc::c_double);
        }
        if !(agget(
            n as *mut libc::c_void,
            b"fontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .is_null()
        {
            sscanf(
                agget(
                    n as *mut libc::c_void,
                    b"fontsize\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                b"%f\0" as *const u8 as *const libc::c_char,
                &mut ff as *mut libc::c_float,
            );
            *(*fsz).offset(i as isize) = ff;
        } else {
            *(*fsz).offset(i as isize) = 14 as libc::c_int as libc::c_float;
        }
        if !(agget(
            n as *mut libc::c_void,
            b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .is_null()
            && strcmp(
                agget(
                    n as *mut libc::c_void,
                    b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
                b"\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            && strcmp(
                agget(
                    n as *mut libc::c_void,
                    b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
                b"\\N\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
        {
            let mut lbs: *mut libc::c_char = agget(
                n as *mut libc::c_void,
                b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            let ref mut fresh7 = *(*labels).offset(i as isize);
            *fresh7 = strdup(lbs);
        } else {
            let ref mut fresh8 = *(*labels).offset(i as isize);
            *fresh8 = strdup(agnameof(n as *mut libc::c_void));
        }
        j = *(*clusters).offset(i as isize);
        if MAX_GRPS - MIN_GRPS < max_color {
            j = (j - MIN_GRPS)
                * ((max_color - 1 as libc::c_int)
                    / (if MAX_GRPS - MIN_GRPS > 1 as libc::c_int {
                        MAX_GRPS - MIN_GRPS
                    } else {
                        1 as libc::c_int
                    }));
        } else {
            j = (j - MIN_GRPS) % max_color;
        }
        if !pal.is_null() {
            *(*rgb_r)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = *pal.offset((3 as libc::c_int * j + 0 as libc::c_int) as isize);
            *(*rgb_g)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = *pal.offset((3 as libc::c_int * j + 1 as libc::c_int) as isize);
            *(*rgb_b)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = *pal.offset((3 as libc::c_int * j + 2 as libc::c_int) as isize);
        }
        if !noclusterinfo && !clust_clr_sym.is_null()
            && colorxlate(
                agxget(n as *mut libc::c_void, clust_clr_sym),
                &mut color,
                RGBA_DOUBLE,
            ) == 0 as libc::c_int
        {
            *(*rgb_r)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = color.u.RGBA[0 as libc::c_int as usize] as libc::c_float;
            *(*rgb_g)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = color.u.RGBA[1 as libc::c_int as usize] as libc::c_float;
            *(*rgb_b)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = color.u.RGBA[2 as libc::c_int as usize] as libc::c_float;
        }
        if !noclusterinfo
            && !(agget(
                n as *mut libc::c_void,
                b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ))
                .is_null()
            && !(agget(
                n as *mut libc::c_void,
                b"clustercolor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ))
                .is_null()
            && strlen(
                agget(
                    n as *mut libc::c_void,
                    b"clustercolor\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            ) >= 7 as libc::c_int as libc::c_ulong && !pal.is_null()
        {
            let mut cc: [libc::c_char; 10] = [0; 10];
            strcpy(
                cc.as_mut_ptr(),
                agget(
                    n as *mut libc::c_void,
                    b"clustercolor\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
            *(*rgb_r)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = hexcol2rgb(cc.as_mut_ptr().offset(1 as libc::c_int as isize));
            *(*rgb_g)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = hexcol2rgb(cc.as_mut_ptr().offset(3 as libc::c_int as isize));
            *(*rgb_b)
                .offset(
                    *(*clusters).offset(i as isize) as isize,
                ) = hexcol2rgb(cc.as_mut_ptr().offset(5 as libc::c_int as isize));
        }
        n = agnxtnode(g, n);
    }
    if !x.is_null() {
        let mut has_position: bool = 0 as libc::c_int != 0;
        *x = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(dim as libc::c_ulong)
                .wrapping_mul(nnodes as libc::c_ulong),
        ) as *mut libc::c_double;
        n = agfstnode(g);
        while !n.is_null() {
            let mut xx: libc::c_double = 0.;
            let mut yy: libc::c_double = 0.;
            i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
            if !(agget(
                n as *mut libc::c_void,
                b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ))
                .is_null()
            {
                has_position = 1 as libc::c_int != 0;
                sscanf(
                    agget(
                        n as *mut libc::c_void,
                        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ),
                    b"%lf,%lf\0" as *const u8 as *const libc::c_char,
                    &mut xx as *mut libc::c_double,
                    &mut yy as *mut libc::c_double,
                );
                *(*x).offset((i * dim) as isize) = xx;
                *(*x).offset((i * dim + 1 as libc::c_int) as isize) = yy;
            } else {
                fprintf(
                    stderr,
                    b"WARNING: pos field missing for node %d, set to origin\n\0"
                        as *const u8 as *const libc::c_char,
                    i,
                );
                *(*x).offset((i * dim) as isize) = 0 as libc::c_int as libc::c_double;
                *(*x)
                    .offset(
                        (i * dim + 1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as libc::c_double;
            }
            n = agnxtnode(g, n);
        }
        if !has_position {
            free(*x as *mut libc::c_void);
            *x = 0 as *mut libc::c_double;
        }
    }
    free(I as *mut libc::c_void);
    free(J as *mut libc::c_void);
    free(val as *mut libc::c_void);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn attached_clustering(
    mut g: *mut Agraph_t,
    mut maxcluster: libc::c_int,
    mut clustering_scheme: libc::c_int,
) {
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut clust_sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut nnodes: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut I: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut J: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_double = 0.;
    let mut type_0: libc::c_int = MATRIX_TYPE_REAL as libc::c_int;
    let mut sz: size_t = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
    let mut clusters: *mut libc::c_int = 0 as *mut libc::c_int;
    if g.is_null() {
        return;
    }
    nnodes = agnnodes(g);
    nedges = agnedges(g);
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        let fresh9 = i;
        i = i + 1;
        (*((*n).base.data as *mut Agnodeinfo_t)).id = fresh9 as libc::c_uint;
        n = agnxtnode(g, n);
    }
    I = calloc(
        nedges as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    J = calloc(
        nedges as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    val = calloc(
        nedges as libc::c_ulong,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    sym = agattr(
        g,
        2 as libc::c_int,
        b"weight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    clust_sym = agattr(
        g,
        1 as libc::c_int,
        b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        row = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
        e = agfstout(g, n);
        while !e.is_null() {
            *I.offset(i as isize) = row;
            *J
                .offset(
                    i as isize,
                ) = (*((*(*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node)
                .base
                .data as *mut Agnodeinfo_t))
                .id as libc::c_int;
            if !sym.is_null() {
                if sscanf(
                    agxget(e as *mut libc::c_void, sym),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_double,
                ) != 1 as libc::c_int
                {
                    v = 1 as libc::c_int as libc::c_double;
                }
            } else {
                v = 1 as libc::c_int as libc::c_double;
            }
            *val.offset(i as isize) = v;
            i += 1;
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    A = SparseMatrix_from_coordinate_arrays(
        nedges,
        nnodes,
        nnodes,
        I,
        J,
        val as *mut libc::c_void,
        type_0,
        sz,
    );
    clusters = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nnodes as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut use_value: libc::c_int = 1 as libc::c_int;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut modularity: libc::c_double = 0.;
    if clust_sym.is_null() {
        clust_sym = agattr(
            g,
            1 as libc::c_int,
            b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"-1\0" as *const u8 as *const libc::c_char,
        );
    }
    if clustering_scheme == CLUSTERING_MQ as libc::c_int {
        mq_clustering(
            A,
            0 as libc::c_int,
            maxcluster,
            use_value,
            &mut nc,
            &mut clusters,
            &mut modularity,
            &mut flag,
        );
    } else if clustering_scheme == CLUSTERING_MODULARITY as libc::c_int {
        modularity_clustering(
            A,
            0 as libc::c_int,
            maxcluster,
            use_value,
            &mut nc,
            &mut clusters,
            &mut modularity,
            &mut flag,
        );
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"DotIO.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void attached_clustering(Agraph_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < nnodes {
        let ref mut fresh10 = *clusters.offset(i as isize);
        *fresh10 += 1;
        i += 1;
    }
    n = agfstnode(g);
    while !n.is_null() {
        i = (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
        let mut value_buffer: [libc::c_char; 12] = [0; 12];
        snprintf(
            value_buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            *clusters.offset(i as isize),
        );
        agxset(n as *mut libc::c_void, clust_sym, value_buffer.as_mut_ptr());
        n = agnxtnode(g, n);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b" no complement clustering info in dot file, using modularity clustering. Modularity = %f, ncluster=%d\n\0"
                as *const u8 as *const libc::c_char,
            modularity,
            nc,
        );
    }
    free(I as *mut libc::c_void);
    free(J as *mut libc::c_void);
    free(val as *mut libc::c_void);
    free(clusters as *mut libc::c_void);
    SparseMatrix_delete(A);
}
#[no_mangle]
pub unsafe extern "C" fn initDotIO(mut g: *mut Agraph_t) {
    aginit(
        g,
        1 as libc::c_int,
        b"info\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn setDotNodeID(mut n: *mut Agnode_t, mut v: libc::c_int) {
    (*((*n).base.data as *mut Agnodeinfo_t)).id = v as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn getDotNodeID(mut n: *mut Agnode_t) -> libc::c_int {
    return (*((*n).base.data as *mut Agnodeinfo_t)).id as libc::c_int;
}
