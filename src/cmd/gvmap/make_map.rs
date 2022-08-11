#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn srand(__seed: libc::c_uint);
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut Verbose: libc::c_uchar;
    fn drand() -> libc::c_double;
    fn vector_float_take(
        n: libc::c_int,
        v: *mut libc::c_float,
        m: libc::c_int,
        p: *mut libc::c_int,
        u: *mut *mut libc::c_float,
    );
    fn distance(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn distance_cropped(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn SparseMatrix_new(
        m: libc::c_int,
        n: libc::c_int,
        nz: libc::c_int,
        type_0: libc::c_int,
        format: libc::c_int,
    ) -> SparseMatrix;
    fn SparseMatrix_from_coordinate_format(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_from_coordinate_format_not_compacted(
        A: SparseMatrix,
    ) -> SparseMatrix;
    fn SparseMatrix_export(f: *mut FILE, A: SparseMatrix);
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_coordinate_form_add_entry(
        A: SparseMatrix,
        irn: libc::c_int,
        jcn: libc::c_int,
        val: *mut libc::c_void,
    ) -> SparseMatrix;
    fn SparseMatrix_transpose(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_symmetrize(
        A: SparseMatrix,
        pattern_symmetric_only: bool,
    ) -> SparseMatrix;
    fn SparseMatrix_remove_diagonal(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_get_real_adjacency_matrix_symmetrized(
        A: SparseMatrix,
    ) -> SparseMatrix;
    fn SparseMatrix_weakly_connected_components(
        A0: SparseMatrix,
        ncomp: *mut libc::c_int,
        comps: *mut *mut libc::c_int,
        comps_ptr: *mut *mut libc::c_int,
    );
    fn SparseMatrix_sort(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_distance_matrix(
        A: SparseMatrix,
        weighted: libc::c_int,
        dist_matrix: *mut *mut libc::c_double,
    ) -> libc::c_int;
    fn SparseMatrix_from_dense(
        m: libc::c_int,
        n: libc::c_int,
        x: *mut libc::c_double,
    ) -> SparseMatrix;
    fn QuadTree_new_from_point_list(
        dim: libc::c_int,
        n: libc::c_int,
        max_level: libc::c_int,
        coord: *mut libc::c_double,
    ) -> QuadTree;
    fn QuadTree_get_nearest(
        qt: QuadTree,
        x: *mut libc::c_double,
        ymin: *mut libc::c_double,
        imin: *mut libc::c_int,
        min: *mut libc::c_double,
        flag: *mut libc::c_int,
    );
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn stress_model(
        dim: libc::c_int,
        A: SparseMatrix,
        D: SparseMatrix,
        x: *mut *mut libc::c_double,
        edge_len_weighted: libc::c_int,
        maxit: libc::c_int,
        tol: libc::c_double,
        flag: *mut libc::c_int,
    );
    fn country_graph_coloring(
        seed: libc::c_int,
        A: SparseMatrix,
        p: *mut *mut libc::c_int,
    );
    fn rgb2hex(
        r: libc::c_float,
        g: libc::c_float,
        b: libc::c_float,
        cstring: *mut libc::c_char,
        opacity: *const libc::c_char,
    );
    fn get_triangles(
        x: *mut libc::c_double,
        n: libc::c_int,
        ntris: *mut libc::c_int,
    ) -> *mut libc::c_int;
    fn node_distinct_coloring(
        color_scheme: *mut libc::c_char,
        lightness: *mut libc::c_char,
        weightedQ: bool,
        A: SparseMatrix,
        accuracy: libc::c_double,
        seed: libc::c_int,
        cdim: *mut libc::c_int,
        colors: *mut *mut libc::c_double,
    ) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const FORMAT_COORD: C2RustUnnamed = 2;
pub const FORMAT_CSR: C2RustUnnamed = 1;
pub const FORMAT_CSC: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_0 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_0 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_0 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_0 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SingleLinkedList_struct {
    pub data: *mut libc::c_void,
    pub next: SingleLinkedList,
}
pub type SingleLinkedList = *mut SingleLinkedList_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QuadTree_struct {
    pub n: libc::c_int,
    pub total_weight: libc::c_double,
    pub dim: libc::c_int,
    pub center: *mut libc::c_double,
    pub width: libc::c_double,
    pub average: *mut libc::c_double,
    pub qts: *mut QuadTree,
    pub l: SingleLinkedList,
    pub max_level: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type QuadTree = *mut QuadTree_struct;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Triangle {
    pub vertices: [libc::c_int; 3],
    pub center: [libc::c_double; 2],
}
pub const NO_DUPLICATE: C2RustUnnamed_4 = -1;
pub type C2RustUnnamed_4 = libc::c_int;
pub const POLY_LINE_NOT_REAL_EDGE: C2RustUnnamed_5 = 1;
pub const POLY_LINE_REAL_EDGE: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
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
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long
        as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int agxbprint(agxbuf *, const char *, ...)\0"))
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
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = (*xb).buf;
    return (*xb).ptr;
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
#[no_mangle]
pub unsafe extern "C" fn map_palette_optimal_coloring(
    mut color_scheme: *mut libc::c_char,
    mut lightness: *mut libc::c_char,
    mut A0: SparseMatrix,
    mut accuracy: libc::c_double,
    mut seed: libc::c_int,
    mut rgb_r: *mut *mut libc::c_float,
    mut rgb_g: *mut *mut libc::c_float,
    mut rgb_b: *mut *mut libc::c_float,
) {
    let mut colors: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut n: libc::c_int = (*A0).m;
    let mut i: libc::c_int = 0;
    let mut cdim: libc::c_int = 0;
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut weightedQ: bool = 1 as libc::c_int != 0;
    let mut dist: *mut libc::c_double = 0 as *mut libc::c_double;
    A = SparseMatrix_symmetrize(A0, 0 as libc::c_int != 0);
    SparseMatrix_distance_matrix(A, 0 as libc::c_int, &mut dist);
    SparseMatrix_delete(A);
    A = SparseMatrix_from_dense(n, n, dist);
    free(dist as *mut libc::c_void);
    A = SparseMatrix_remove_diagonal(A);
    SparseMatrix_export(stdout, A);
    node_distinct_coloring(
        color_scheme,
        lightness,
        weightedQ,
        A,
        accuracy,
        seed,
        &mut cdim,
        &mut colors,
    );
    if A != A0 {
        SparseMatrix_delete(A);
    }
    *rgb_r = gv_calloc(
        (n + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    *rgb_g = gv_calloc(
        (n + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    *rgb_b = gv_calloc(
        (n + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < n {
        *(*rgb_r)
            .offset(
                (i + 1 as libc::c_int) as isize,
            ) = *colors.offset((cdim * i) as isize) as libc::c_float;
        *(*rgb_g)
            .offset(
                (i + 1 as libc::c_int) as isize,
            ) = *colors.offset((cdim * i + 1 as libc::c_int) as isize) as libc::c_float;
        *(*rgb_b)
            .offset(
                (i + 1 as libc::c_int) as isize,
            ) = *colors.offset((cdim * i + 2 as libc::c_int) as isize) as libc::c_float;
        i += 1;
    }
    free(colors as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn map_optimal_coloring(
    mut seed: libc::c_int,
    mut A: SparseMatrix,
    mut rgb_r: *mut libc::c_float,
    mut rgb_g: *mut libc::c_float,
    mut rgb_b: *mut libc::c_float,
) {
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut u: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut n: libc::c_int = (*A).m;
    let mut i: libc::c_int = 0;
    country_graph_coloring(seed, A, &mut p);
    rgb_r = rgb_r.offset(1);
    rgb_b = rgb_b.offset(1);
    rgb_g = rgb_g.offset(1);
    vector_float_take(n, rgb_r, n, p, &mut u);
    i = 0 as libc::c_int;
    while i < n {
        *rgb_r.offset(i as isize) = *u.offset(i as isize);
        i += 1;
    }
    vector_float_take(n, rgb_g, n, p, &mut u);
    i = 0 as libc::c_int;
    while i < n {
        *rgb_g.offset(i as isize) = *u.offset(i as isize);
        i += 1;
    }
    vector_float_take(n, rgb_b, n, p, &mut u);
    i = 0 as libc::c_int;
    while i < n {
        *rgb_b.offset(i as isize) = *u.offset(i as isize);
        i += 1;
    }
    free(u as *mut libc::c_void);
}
unsafe extern "C" fn get_poly_id(
    mut ip: libc::c_int,
    mut point_poly_map: SparseMatrix,
) -> libc::c_int {
    return *((*point_poly_map).ja)
        .offset(*((*point_poly_map).ia).offset(ip as isize) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn improve_contiguity(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut grouping: *mut libc::c_int,
    mut poly_point_map: SparseMatrix,
    mut x: *mut libc::c_double,
    mut graph: SparseMatrix,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut point_poly_map: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut D: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut dist: libc::c_double = 0.;
    let mut nbad: libc::c_int = 0 as libc::c_int;
    let mut flag: libc::c_int = 0;
    let mut maxit: libc::c_int = 10 as libc::c_int;
    let mut tol: libc::c_double = 0.001f64;
    D = SparseMatrix_get_real_adjacency_matrix_symmetrized(graph);
    if (*graph).m == n {} else {
        __assert_fail(
            b"graph->m == n\0" as *const u8 as *const libc::c_char,
            b"make_map.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void improve_contiguity(int, int, int *, SparseMatrix, double *, SparseMatrix)\0",
            ))
                .as_ptr(),
        );
    }
    ia = (*D).ia;
    ja = (*D).ja;
    a = (*D).a as *mut libc::c_double;
    point_poly_map = SparseMatrix_transpose(poly_point_map);
    i = 0 as libc::c_int;
    while i < n {
        u = i;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            v = *ja.offset(j as isize);
            dist = distance_cropped(x, dim, u, v);
            if *grouping.offset(u as isize) != *grouping.offset(v as isize) {
                *a.offset(j as isize) = 1.1f64 * dist;
            } else if get_poly_id(u, point_poly_map) == get_poly_id(v, point_poly_map) {
                *a.offset(j as isize) = dist;
            } else {
                nbad += 1;
                *a.offset(j as isize) = 0.9f64 * dist;
            }
            j += 1;
        }
        i += 1;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"ratio (edges among discontiguous regions vs total edges)=%f\n\0"
                as *const u8 as *const libc::c_char,
            nbad as libc::c_double / *ia.offset(n as isize) as libc::c_double,
        );
    }
    stress_model(dim, D, D, &mut x, 0 as libc::c_int, maxit, tol, &mut flag);
    if flag == 0 {} else {
        __assert_fail(
            b"!flag\0" as *const u8 as *const libc::c_char,
            b"make_map.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void improve_contiguity(int, int, int *, SparseMatrix, double *, SparseMatrix)\0",
            ))
                .as_ptr(),
        );
    }
    SparseMatrix_delete(D);
    SparseMatrix_delete(point_poly_map);
}
unsafe extern "C" fn normal(
    mut v: *mut libc::c_double,
    mut normal_0: *mut libc::c_double,
) {
    if *v.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double {
        *normal_0.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
        *normal_0.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    } else {
        *normal_0
            .offset(0 as libc::c_int as isize) = -*v.offset(1 as libc::c_int as isize);
        *normal_0
            .offset(1 as libc::c_int as isize) = *v.offset(0 as libc::c_int as isize);
    };
}
unsafe extern "C" fn triangle_center(
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut z: *mut libc::c_double,
    mut c: *mut libc::c_double,
) {
    let mut xy: [libc::c_double; 2] = [0.; 2];
    let mut yz: [libc::c_double; 2] = [0.; 2];
    let mut nxy: [libc::c_double; 2] = [0.; 2];
    let mut nyz: [libc::c_double; 2] = [0.; 2];
    let mut ymx: [libc::c_double; 2] = [0.; 2];
    let mut ymz: [libc::c_double; 2] = [0.; 2];
    let mut beta: libc::c_double = 0.;
    let mut bot: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        ymx[i as usize] = *y.offset(i as isize) - *x.offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        ymz[i as usize] = *y.offset(i as isize) - *z.offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        xy[i as usize] = 0.5f64 * (*x.offset(i as isize) + *y.offset(i as isize));
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        yz[i as usize] = 0.5f64 * (*y.offset(i as isize) + *z.offset(i as isize));
        i += 1;
    }
    normal(ymx.as_mut_ptr(), nxy.as_mut_ptr());
    normal(ymz.as_mut_ptr(), nyz.as_mut_ptr());
    bot = nyz[0 as libc::c_int as usize]
        * (*x.offset(0 as libc::c_int as isize) - *y.offset(0 as libc::c_int as isize))
        + nyz[1 as libc::c_int as usize]
            * (*x.offset(1 as libc::c_int as isize)
                - *y.offset(1 as libc::c_int as isize));
    if bot == 0 as libc::c_int as libc::c_double {
        *c.offset(0 as libc::c_int as isize) = xy[0 as libc::c_int as usize];
        *c.offset(1 as libc::c_int as isize) = xy[1 as libc::c_int as usize];
        return;
    }
    beta = ((*x.offset(0 as libc::c_int as isize) - *y.offset(0 as libc::c_int as isize))
        * (xy[0 as libc::c_int as usize] - yz[0 as libc::c_int as usize])
        + (*x.offset(1 as libc::c_int as isize) - *y.offset(1 as libc::c_int as isize))
            * (xy[1 as libc::c_int as usize] - yz[1 as libc::c_int as usize])) / bot;
    *c
        .offset(
            0 as libc::c_int as isize,
        ) = yz[0 as libc::c_int as usize] + beta * nyz[0 as libc::c_int as usize];
    *c
        .offset(
            1 as libc::c_int as isize,
        ) = yz[1 as libc::c_int as usize] + beta * nyz[1 as libc::c_int as usize];
}
unsafe extern "C" fn matrix_add_entry(
    mut A: SparseMatrix,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut val: libc::c_int,
) -> SparseMatrix {
    let mut i1: libc::c_int = i;
    let mut j1: libc::c_int = j;
    if i < j {
        i1 = j;
        j1 = i;
    }
    A = SparseMatrix_coordinate_form_add_entry(
        A,
        j1,
        i1,
        &mut val as *mut libc::c_int as *mut libc::c_void,
    );
    return SparseMatrix_coordinate_form_add_entry(
        A,
        i1,
        j1,
        &mut val as *mut libc::c_int as *mut libc::c_void,
    );
}
unsafe extern "C" fn plot_dot_edges(mut f: *mut FILE, mut A: SparseMatrix) {
    let mut i: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = (*A).m;
    ia = (*A).ia;
    ja = (*A).ja;
    i = 0 as libc::c_int;
    while i < n {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if !(*ja.offset(j as isize) == i) {
                fprintf(
                    f,
                    b"%d -- %d;\n\0" as *const u8 as *const libc::c_char,
                    i,
                    *ja.offset(j as isize),
                );
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn plot_dot_labels(
    mut f: *mut FILE,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut labels: *mut *mut libc::c_char,
    mut fsz: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if !fsz.is_null() {
            fprintf(
                f,
                b"%d [label=\"%s\", pos=\"%lf,%lf\", fontsize=%f];\n\0" as *const u8
                    as *const libc::c_char,
                i,
                *labels.offset(i as isize),
                *x.offset((i * dim) as isize),
                *x.offset((i * dim + 1 as libc::c_int) as isize),
                *fsz.offset(i as isize) as libc::c_double,
            );
        } else {
            fprintf(
                f,
                b"%d [label=\"%s\", pos=\"%lf,%lf\"];\n\0" as *const u8
                    as *const libc::c_char,
                i,
                *labels.offset(i as isize),
                *x.offset((i * dim) as isize),
                *x.offset((i * dim + 1 as libc::c_int) as isize),
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn dot_polygon(
    mut sbuff: *mut agxbuf,
    mut np: libc::c_int,
    mut xp: *mut libc::c_float,
    mut yp: *mut libc::c_float,
    mut line_width: libc::c_double,
    mut fill: libc::c_int,
    mut cstring: *const libc::c_char,
) {
    if np > 0 as libc::c_int {
        if fill >= 0 as libc::c_int {
            agxbprint(
                sbuff,
                b" c %zu -%s C %zu -%s P %d \0" as *const u8 as *const libc::c_char,
                strlen(cstring),
                cstring,
                strlen(cstring),
                cstring,
                np,
            );
        } else if line_width > 0 as libc::c_int as libc::c_double {
            let mut len_swidth: size_t = snprintf(
                0 as *mut libc::c_char,
                0 as libc::c_int as libc::c_ulong,
                b"%f\0" as *const u8 as *const libc::c_char,
                line_width,
            ) as size_t;
            agxbprint(
                sbuff,
                b" c %zu -%s S %zu -setlinewidth(%f) L %d \0" as *const u8
                    as *const libc::c_char,
                strlen(cstring),
                cstring,
                len_swidth.wrapping_add(14 as libc::c_int as libc::c_ulong),
                line_width,
                np,
            );
        } else {
            agxbprint(
                sbuff,
                b" c %zu -%s L %d \0" as *const u8 as *const libc::c_char,
                strlen(cstring),
                cstring,
                np,
            );
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < np {
            agxbprint(
                sbuff,
                b" %f %f\0" as *const u8 as *const libc::c_char,
                *xp.offset(i as isize) as libc::c_double,
                *yp.offset(i as isize) as libc::c_double,
            );
            i += 1;
        }
    }
}
unsafe extern "C" fn dot_one_poly(
    mut sbuff: *mut agxbuf,
    mut line_width: libc::c_double,
    mut fill: libc::c_int,
    mut np: libc::c_int,
    mut xp: *mut libc::c_float,
    mut yp: *mut libc::c_float,
    mut cstring: *const libc::c_char,
) {
    dot_polygon(sbuff, np, xp, yp, line_width, fill, cstring);
}
unsafe extern "C" fn plot_dot_polygons(
    mut sbuff: *mut agxbuf,
    mut line_width: libc::c_double,
    mut line_color: *const libc::c_char,
    mut polys: SparseMatrix,
    mut x_poly: *mut libc::c_double,
    mut polys_groups: *mut libc::c_int,
    mut r: *mut libc::c_float,
    mut g: *mut libc::c_float,
    mut b: *mut libc::c_float,
    mut opacity: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*polys).ia;
    let mut ja: *mut libc::c_int = (*polys).ja;
    let mut a: *mut libc::c_int = (*polys).a as *mut libc::c_int;
    let mut npolys: libc::c_int = (*polys).m;
    let mut nverts: libc::c_int = (*polys).n;
    let mut ipoly: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut np: libc::c_int = 0 as libc::c_int;
    let mut maxlen: libc::c_int = 0 as libc::c_int;
    let mut xp: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut yp: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut fill: libc::c_int = -(1 as libc::c_int);
    let mut cstring: [libc::c_char; 10] = *::std::mem::transmute::<
        &[u8; 10],
        &mut [libc::c_char; 10],
    >(b"#aaaaaaff\0");
    let mut use_line: libc::c_int = (line_width >= 0 as libc::c_int as libc::c_double)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < npolys {
        maxlen = if maxlen
            > *ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize)
        {
            maxlen
        } else {
            *ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize)
        };
        i += 1;
    }
    xp = gv_calloc(
        maxlen as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    yp = gv_calloc(
        maxlen as size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    if Verbose != 0 {
        fprintf(stderr, b"npolys = %d\n\0" as *const u8 as *const libc::c_char, npolys);
    }
    first = abs(*a.offset(0 as libc::c_int as isize));
    ipoly = first + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < npolys {
        np = 0 as libc::c_int;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if *ja.offset(j as isize) < nverts
                && *ja.offset(j as isize) >= 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"ja[j] < nverts && ja[j] >= 0\0" as *const u8
                        as *const libc::c_char,
                    b"make_map.c\0" as *const u8 as *const libc::c_char,
                    291 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 127],
                        &[libc::c_char; 127],
                    >(
                        b"void plot_dot_polygons(agxbuf *, double, const char *, SparseMatrix, double *, int *, float *, float *, float *, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if abs(*a.offset(j as isize)) != ipoly {
                ipoly = abs(*a.offset(j as isize));
                if !r.is_null() && !g.is_null() && !b.is_null() {
                    rgb2hex(
                        *r.offset(*polys_groups.offset(i as isize) as isize),
                        *g.offset(*polys_groups.offset(i as isize) as isize),
                        *b.offset(*polys_groups.offset(i as isize) as isize),
                        cstring.as_mut_ptr(),
                        opacity,
                    );
                }
                dot_one_poly(sbuff, line_width, fill, np, xp, yp, cstring.as_mut_ptr());
                np = 0 as libc::c_int;
            }
            *xp
                .offset(
                    np as isize,
                ) = *x_poly.offset((2 as libc::c_int * *ja.offset(j as isize)) as isize)
                as libc::c_float;
            let fresh11 = np;
            np = np + 1;
            *yp
                .offset(
                    fresh11 as isize,
                ) = *x_poly
                .offset(
                    (2 as libc::c_int * *ja.offset(j as isize) + 1 as libc::c_int)
                        as isize,
                ) as libc::c_float;
            j += 1;
        }
        if use_line != 0 {
            dot_one_poly(sbuff, line_width, fill, np, xp, yp, line_color);
        } else {
            dot_one_poly(
                sbuff,
                -(1 as libc::c_int) as libc::c_double,
                1 as libc::c_int,
                np,
                xp,
                yp,
                cstring.as_mut_ptr(),
            );
        }
        i += 1;
    }
    free(xp as *mut libc::c_void);
    free(yp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn plot_dot_map(
    mut gr: *mut Agraph_t,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut polys: SparseMatrix,
    mut poly_lines: SparseMatrix,
    mut line_width: libc::c_double,
    mut line_color: *const libc::c_char,
    mut x_poly: *mut libc::c_double,
    mut polys_groups: *mut libc::c_int,
    mut labels: *mut *mut libc::c_char,
    mut fsz: *mut libc::c_float,
    mut r: *mut libc::c_float,
    mut g: *mut libc::c_float,
    mut b: *mut libc::c_float,
    mut opacity: *const libc::c_char,
    mut A: SparseMatrix,
    mut f: *mut FILE,
) {
    let mut plot_polyQ: bool = 1 as libc::c_int != 0;
    let mut sbuff: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    agxbinit(&mut sbuff, 0 as libc::c_int as libc::c_uint, 0 as *mut libc::c_char);
    if r.is_null() || g.is_null() || b.is_null() {
        plot_polyQ = 0 as libc::c_int != 0;
    }
    if gr.is_null() {
        fprintf(
            f,
            b"graph map {\n node [margin = 0 width=0.0001 height=0.00001 shape=plaintext];\n graph [outputorder=edgesfirst, bgcolor=\"#dae2ff\"]\n edge [color=\"#55555515\",fontname=\"Helvetica-Bold\"]\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        agattr(
            gr,
            1 as libc::c_int,
            b"margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            gr,
            1 as libc::c_int,
            b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0.0001\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            gr,
            1 as libc::c_int,
            b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0.0001\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            gr,
            1 as libc::c_int,
            b"shape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"plaintext\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            gr,
            1 as libc::c_int,
            b"margin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            gr,
            1 as libc::c_int,
            b"fontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Helvetica-Bold\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            gr,
            0 as libc::c_int,
            b"outputorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"edgesfirst\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            gr,
            0 as libc::c_int,
            b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"#dae2ff\0" as *const u8 as *const libc::c_char,
        );
        if A.is_null() {
            agattr(
                gr,
                2 as libc::c_int,
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"invis\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if plot_polyQ {
        if gr.is_null() {
            fprintf(f, b"_background = \"\0" as *const u8 as *const libc::c_char);
        }
        plot_dot_polygons(
            &mut sbuff,
            -1.0f64,
            0 as *const libc::c_char,
            polys,
            x_poly,
            polys_groups,
            r,
            g,
            b,
            opacity,
        );
    }
    if line_width >= 0 as libc::c_int as libc::c_double {
        plot_dot_polygons(
            &mut sbuff,
            line_width,
            line_color,
            poly_lines,
            x_poly,
            polys_groups,
            0 as *mut libc::c_float,
            0 as *mut libc::c_float,
            0 as *mut libc::c_float,
            0 as *const libc::c_char,
        );
    }
    if gr.is_null() {
        fprintf(f, b"%s\0" as *const u8 as *const libc::c_char, agxbuse(&mut sbuff));
        fprintf(f, b"\"\n\0" as *const u8 as *const libc::c_char);
    } else {
        agattr(
            gr,
            0 as libc::c_int,
            b"_background\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(&mut sbuff),
        );
        agwrite(gr, f as *mut libc::c_void);
    }
    if gr.is_null() && !labels.is_null() {
        plot_dot_labels(f, n, dim, x, labels, fsz);
    }
    if gr.is_null() && !A.is_null() {
        plot_dot_edges(f, A);
    }
    if gr.is_null() {
        fprintf(f, b"}\n\0" as *const u8 as *const libc::c_char);
    }
    agxbfree(&mut sbuff);
}
unsafe extern "C" fn get_tri(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut nt: *mut libc::c_int,
    mut T: *mut *mut Triangle,
    mut E: *mut SparseMatrix,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut i0: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut ntri: libc::c_int = 0;
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut trilist: *mut libc::c_int = get_triangles(x, n, &mut ntri);
    *T = gv_calloc(ntri as size_t, ::std::mem::size_of::<Triangle>() as libc::c_ulong)
        as *mut Triangle;
    A = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_INTEGER as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < ntri {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*(*T).offset(i as isize))
                .vertices[j
                as usize] = *trilist.offset((i * 3 as libc::c_int + j) as isize);
            j += 1;
        }
        i0 = (*(*T).offset(i as isize)).vertices[0 as libc::c_int as usize];
        i1 = (*(*T).offset(i as isize)).vertices[1 as libc::c_int as usize];
        i2 = (*(*T).offset(i as isize)).vertices[2 as libc::c_int as usize];
        triangle_center(
            &mut *x.offset((i0 * dim) as isize),
            &mut *x.offset((i1 * dim) as isize),
            &mut *x.offset((i2 * dim) as isize),
            ((*(*T).offset(i as isize)).center).as_mut_ptr(),
        );
        A = matrix_add_entry(A, i0, i1, i);
        A = matrix_add_entry(A, i1, i2, i);
        A = matrix_add_entry(A, i2, i0, i);
        i += 1;
    }
    B = SparseMatrix_from_coordinate_format_not_compacted(A);
    SparseMatrix_delete(A);
    B = SparseMatrix_sort(B);
    *E = B;
    *nt = ntri;
    free(trilist as *mut libc::c_void);
}
unsafe extern "C" fn get_country_graph(
    mut n: libc::c_int,
    mut A: SparseMatrix,
    mut groups: *mut libc::c_int,
    mut GRP_RANDOM: libc::c_int,
    mut GRP_BBOX: libc::c_int,
) -> SparseMatrix {
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut one: libc::c_int = 1 as libc::c_int;
    let mut jj: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ig1: libc::c_int = 0;
    let mut ig2: libc::c_int = 0;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut BB: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut min_grp: libc::c_int = 0;
    let mut max_grp: libc::c_int = 0;
    max_grp = *groups.offset(0 as libc::c_int as isize);
    min_grp = max_grp;
    i = 0 as libc::c_int;
    while i < n {
        max_grp = if *groups.offset(i as isize) > max_grp {
            *groups.offset(i as isize)
        } else {
            max_grp
        };
        min_grp = if *groups.offset(i as isize) < min_grp {
            *groups.offset(i as isize)
        } else {
            min_grp
        };
        i += 1;
    }
    if min_grp <= 0 as libc::c_int {
        return 0 as SparseMatrix;
    }
    B = SparseMatrix_new(
        max_grp,
        max_grp,
        1 as libc::c_int,
        MATRIX_TYPE_INTEGER as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    ia = (*A).ia;
    ja = (*A).ja;
    i = 0 as libc::c_int;
    while i < n {
        ig1 = *groups.offset(i as isize) - 1 as libc::c_int;
        SparseMatrix_coordinate_form_add_entry(
            B,
            ig1,
            ig1,
            &mut one as *mut libc::c_int as *mut libc::c_void,
        );
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            if i != jj && *groups.offset(i as isize) != *groups.offset(jj as isize)
                && *groups.offset(jj as isize) != GRP_RANDOM
                && *groups.offset(jj as isize) != GRP_BBOX
            {
                ig1 = *groups.offset(i as isize) - 1 as libc::c_int;
                ig2 = *groups.offset(jj as isize) - 1 as libc::c_int;
                SparseMatrix_coordinate_form_add_entry(
                    B,
                    ig1,
                    ig2,
                    &mut one as *mut libc::c_int as *mut libc::c_void,
                );
            }
            j += 1;
        }
        i += 1;
    }
    BB = SparseMatrix_from_coordinate_format(B);
    SparseMatrix_delete(B);
    return BB;
}
unsafe extern "C" fn conn_comp(
    mut n: libc::c_int,
    mut A: SparseMatrix,
    mut groups: *mut libc::c_int,
    mut poly_point_map: *mut SparseMatrix,
) {
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut one: libc::c_int = 1 as libc::c_int;
    let mut jj: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut BB: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut ncomps: libc::c_int = 0;
    let mut comps: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut comps_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    B = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_INTEGER as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    ia = (*A).ia;
    ja = (*A).ja;
    i = 0 as libc::c_int;
    while i < n {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            if i != jj && *groups.offset(i as isize) == *groups.offset(jj as isize) {
                SparseMatrix_coordinate_form_add_entry(
                    B,
                    i,
                    jj,
                    &mut one as *mut libc::c_int as *mut libc::c_void,
                );
            }
            j += 1;
        }
        i += 1;
    }
    BB = SparseMatrix_from_coordinate_format(B);
    SparseMatrix_weakly_connected_components(
        BB,
        &mut ncomps,
        &mut comps,
        &mut comps_ptr,
    );
    SparseMatrix_delete(B);
    SparseMatrix_delete(BB);
    *poly_point_map = SparseMatrix_new(
        ncomps,
        n,
        n,
        MATRIX_TYPE_PATTERN as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    free((**poly_point_map).ia as *mut libc::c_void);
    free((**poly_point_map).ja as *mut libc::c_void);
    let ref mut fresh12 = (**poly_point_map).ia;
    *fresh12 = comps_ptr;
    let ref mut fresh13 = (**poly_point_map).ja;
    *fresh13 = comps;
    (**poly_point_map).nz = n;
}
unsafe extern "C" fn get_poly_lines(
    mut exclude_random: libc::c_int,
    mut nt: libc::c_int,
    mut graph: SparseMatrix,
    mut E: SparseMatrix,
    mut ncomps: libc::c_int,
    mut comps_ptr: *mut libc::c_int,
    mut comps: *mut libc::c_int,
    mut groups: *mut libc::c_int,
    mut mask: *mut libc::c_int,
    mut poly_lines: *mut SparseMatrix,
    mut polys_groups: *mut *mut libc::c_int,
    mut GRP_RANDOM: libc::c_int,
    mut GRP_BBOX: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut tlist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut ipoly: libc::c_int = 0;
    let mut ipoly2: libc::c_int = 0;
    let mut nnt: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut cur: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nlink: libc::c_int = 0;
    let mut sta: libc::c_int = 0;
    let mut elist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut edim: libc::c_int = 3 as libc::c_int;
    let mut ie: *mut libc::c_int = (*E).ia;
    let mut je: *mut libc::c_int = (*E).ja;
    let mut e: *mut libc::c_int = (*E).a as *mut libc::c_int;
    let mut n: libc::c_int = (*E).m;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut gmask: *mut libc::c_int = 0 as *mut libc::c_int;
    graph = 0 as SparseMatrix;
    if !graph.is_null() {
        if (*graph).m == n {} else {
            __assert_fail(
                b"graph->m == n\0" as *const u8 as *const libc::c_char,
                b"make_map.c\0" as *const u8 as *const libc::c_char,
                498 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"void get_poly_lines(int, int, SparseMatrix, SparseMatrix, int, int *, int *, int *, int *, SparseMatrix *, int **, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
        gmask = gv_calloc(
            n as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < n {
            *gmask.offset(i as isize) = -(1 as libc::c_int);
            i += 1;
        }
        ia = (*graph).ia;
        ja = (*graph).ja;
        edim = 5 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < nt {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    elist = gv_calloc(
        (nt * edim) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    tlist = gv_calloc(
        (nt * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *poly_lines = SparseMatrix_new(
        ncomps,
        nt,
        1 as libc::c_int,
        MATRIX_TYPE_INTEGER as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    *polys_groups = gv_calloc(
        ncomps as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < nt {
        *elist.offset((i * edim + 2 as libc::c_int) as isize) = 0 as libc::c_int;
        i += 1;
    }
    nz = *ie.offset((*E).m as isize) - *ie.offset(0 as libc::c_int as isize);
    ipoly = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ncomps {
        nnt = 0 as libc::c_int;
        j = *comps_ptr.offset(i as isize);
        while j < *comps_ptr.offset((i + 1 as libc::c_int) as isize) {
            ii = *comps.offset(j as isize);
            if !graph.is_null() {
                jj = *ia.offset(ii as isize);
                while jj < *ia.offset((ii + 1 as libc::c_int) as isize) {
                    *gmask.offset(*ja.offset(jj as isize) as isize) = ii;
                    jj += 1;
                }
            }
            *(*polys_groups).offset(i as isize) = *groups.offset(ii as isize);
            if !(exclude_random != 0
                && (*groups.offset(ii as isize) == GRP_RANDOM
                    || *groups.offset(ii as isize) == GRP_BBOX))
            {
                if !(*groups.offset(ii as isize) == GRP_BBOX) {
                    jj = *ie.offset(ii as isize);
                    while jj < *ie.offset((ii + 1 as libc::c_int) as isize) {
                        if *groups.offset(*je.offset(jj as isize) as isize)
                            != *groups.offset(ii as isize) && jj < nz - 1 as libc::c_int
                            && *je.offset(jj as isize)
                                == *je.offset((jj + 1 as libc::c_int) as isize)
                        {
                            t1 = *e.offset(jj as isize);
                            t2 = *e.offset((jj + 1 as libc::c_int) as isize);
                            nlink = *elist
                                .offset((t1 * edim + 2 as libc::c_int) as isize)
                                % 2 as libc::c_int;
                            *elist.offset((t1 * edim + nlink) as isize) = t2;
                            if !graph.is_null() {
                                if *gmask.offset(*je.offset(jj as isize) as isize) == ii {
                                    *elist
                                        .offset(
                                            (t1 * edim + nlink + 3 as libc::c_int) as isize,
                                        ) = POLY_LINE_REAL_EDGE as libc::c_int;
                                } else {
                                    fprintf(
                                        stderr,
                                        b"not real!!!\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    *elist
                                        .offset(
                                            (t1 * edim + nlink + 3 as libc::c_int) as isize,
                                        ) = POLY_LINE_NOT_REAL_EDGE as libc::c_int;
                                }
                            }
                            let ref mut fresh14 = *elist
                                .offset((t1 * edim + 2 as libc::c_int) as isize);
                            *fresh14 += 1;
                            nlink = *elist
                                .offset((t2 * edim + 2 as libc::c_int) as isize)
                                % 2 as libc::c_int;
                            *elist.offset((t2 * edim + nlink) as isize) = t1;
                            let ref mut fresh15 = *elist
                                .offset((t2 * edim + 2 as libc::c_int) as isize);
                            *fresh15 += 1;
                            if !graph.is_null() {
                                if *gmask.offset(*je.offset(jj as isize) as isize) == ii {
                                    *elist
                                        .offset(
                                            (t2 * edim + nlink + 3 as libc::c_int) as isize,
                                        ) = POLY_LINE_REAL_EDGE as libc::c_int;
                                } else {
                                    fprintf(
                                        stderr,
                                        b"not real!!!\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    *elist
                                        .offset(
                                            (t2 * edim + nlink + 3 as libc::c_int) as isize,
                                        ) = POLY_LINE_NOT_REAL_EDGE as libc::c_int;
                                }
                            }
                            let fresh16 = nnt;
                            nnt = nnt + 1;
                            *tlist.offset(fresh16 as isize) = t1;
                            let fresh17 = nnt;
                            nnt = nnt + 1;
                            *tlist.offset(fresh17 as isize) = t2;
                            jj += 1;
                        }
                        jj += 1;
                    }
                }
            }
            j += 1;
        }
        j = 0 as libc::c_int;
        while j < nnt {
            t = *tlist.offset(j as isize);
            if *mask.offset(t as isize) != i {
                sta = t;
                cur = sta;
                *mask.offset(cur as isize) = i;
                next = *elist.offset((edim * t + 1 as libc::c_int) as isize);
                nlink = 1 as libc::c_int;
                if !graph.is_null()
                    && *elist.offset((edim * cur + 3 as libc::c_int) as isize)
                        == POLY_LINE_NOT_REAL_EDGE as libc::c_int
                {
                    ipoly2 = -ipoly;
                } else {
                    ipoly2 = ipoly;
                }
                SparseMatrix_coordinate_form_add_entry(
                    *poly_lines,
                    i,
                    cur,
                    &mut ipoly2 as *mut libc::c_int as *mut libc::c_void,
                );
                while next != sta {
                    *mask.offset(next as isize) = i;
                    if !graph.is_null()
                        && *elist
                            .offset((edim * cur + nlink + 3 as libc::c_int) as isize)
                            == POLY_LINE_NOT_REAL_EDGE as libc::c_int
                    {
                        ipoly2 = -ipoly;
                    } else {
                        ipoly2 = ipoly;
                    }
                    SparseMatrix_coordinate_form_add_entry(
                        *poly_lines,
                        i,
                        next,
                        &mut ipoly2 as *mut libc::c_int as *mut libc::c_void,
                    );
                    nn = *elist.offset((edim * next + 0 as libc::c_int) as isize);
                    nlink = 0 as libc::c_int;
                    if nn == cur {
                        nlink = 1 as libc::c_int;
                        nn = *elist.offset((edim * next + 1 as libc::c_int) as isize);
                    }
                    if nn != cur {} else {
                        __assert_fail(
                            b"nn != cur\0" as *const u8 as *const libc::c_char,
                            b"make_map.c\0" as *const u8 as *const libc::c_char,
                            611 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 125],
                                &[libc::c_char; 125],
                            >(
                                b"void get_poly_lines(int, int, SparseMatrix, SparseMatrix, int, int *, int *, int *, int *, SparseMatrix *, int **, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    cur = next;
                    next = nn;
                }
                if !graph.is_null()
                    && *elist.offset((edim * cur + nlink + 3 as libc::c_int) as isize)
                        == POLY_LINE_NOT_REAL_EDGE as libc::c_int
                {
                    ipoly2 = -ipoly;
                } else {
                    ipoly2 = ipoly;
                }
                SparseMatrix_coordinate_form_add_entry(
                    *poly_lines,
                    i,
                    sta,
                    &mut ipoly2 as *mut libc::c_int as *mut libc::c_void,
                );
                ipoly += 1;
            }
            j += 1;
        }
        i += 1;
    }
    A = SparseMatrix_from_coordinate_format_not_compacted(*poly_lines);
    SparseMatrix_delete(*poly_lines);
    *poly_lines = A;
    free(tlist as *mut libc::c_void);
    free(elist as *mut libc::c_void);
}
unsafe extern "C" fn plot_cycle(
    mut head: libc::c_int,
    mut cycle: *mut libc::c_int,
    mut edge_table: *mut libc::c_int,
    mut x: *mut libc::c_double,
) {
    let mut cur: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    printf(b"Graphics[{\0" as *const u8 as *const libc::c_char);
    cur = head;
    loop {
        next = *cycle.offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize);
        if !(next != head) {
            break;
        }
        x1 = *x
            .offset(
                (2 as libc::c_int
                    * *edge_table.offset((2 as libc::c_int * cur) as isize)) as isize,
            );
        y1 = *x
            .offset(
                (2 as libc::c_int * *edge_table.offset((2 as libc::c_int * cur) as isize)
                    + 1 as libc::c_int) as isize,
            );
        x2 = *x
            .offset(
                (2 as libc::c_int
                    * *edge_table
                        .offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize))
                    as isize,
            );
        y2 = *x
            .offset(
                (2 as libc::c_int
                    * *edge_table
                        .offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize)
                    + 1 as libc::c_int) as isize,
            );
        printf(
            b"{Black,Arrow[{{%f,%f},{%f,%f}}],Green,Text[%d, {%f,%f}],Text[%d, {%f,%f}]},\0"
                as *const u8 as *const libc::c_char,
            x1,
            y1,
            x2,
            y2,
            *edge_table.offset((2 as libc::c_int * cur) as isize),
            x1,
            y1,
            *edge_table.offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize),
            x2,
            y2,
        );
        cur = next;
    }
    x1 = *x
        .offset(
            (2 as libc::c_int * *edge_table.offset((2 as libc::c_int * cur) as isize))
                as isize,
        );
    y1 = *x
        .offset(
            (2 as libc::c_int * *edge_table.offset((2 as libc::c_int * cur) as isize)
                + 1 as libc::c_int) as isize,
        );
    x2 = *x
        .offset(
            (2 as libc::c_int
                * *edge_table
                    .offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize))
                as isize,
        );
    y2 = *x
        .offset(
            (2 as libc::c_int
                * *edge_table
                    .offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize)
                + 1 as libc::c_int) as isize,
        );
    printf(
        b"{Black,Arrow[{{%f,%f},{%f,%f}}],Green,Text[%d, {%f,%f}],Text[%d, {%f,%f}]}}]\0"
            as *const u8 as *const libc::c_char,
        x1,
        y1,
        x2,
        y2,
        *edge_table.offset((2 as libc::c_int * cur) as isize),
        x1,
        y1,
        *edge_table.offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize),
        x2,
        y2,
    );
}
unsafe extern "C" fn cycle_print(
    mut head: libc::c_int,
    mut cycle: *mut libc::c_int,
    mut edge_table: *mut libc::c_int,
) {
    let mut cur: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    cur = head;
    fprintf(stderr, b"cycle (edges): {\0" as *const u8 as *const libc::c_char);
    loop {
        next = *cycle.offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize);
        if !(next != head) {
            break;
        }
        fprintf(stderr, b"%d,\0" as *const u8 as *const libc::c_char, cur);
        cur = next;
    }
    fprintf(stderr, b"%d}\n\0" as *const u8 as *const libc::c_char, cur);
    cur = head;
    fprintf(stderr, b"cycle (vertices): \0" as *const u8 as *const libc::c_char);
    loop {
        next = *cycle.offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize);
        if !(next != head) {
            break;
        }
        fprintf(
            stderr,
            b"%d--\0" as *const u8 as *const libc::c_char,
            *edge_table.offset((2 as libc::c_int * cur) as isize),
        );
        cur = next;
    }
    fprintf(
        stderr,
        b"%d--%d\n\0" as *const u8 as *const libc::c_char,
        *edge_table.offset((2 as libc::c_int * cur) as isize),
        *edge_table.offset((2 as libc::c_int * cur + 1 as libc::c_int) as isize),
    );
}
unsafe extern "C" fn same_edge(
    mut ecur: libc::c_int,
    mut elast: libc::c_int,
    mut edge_table: *mut libc::c_int,
) -> libc::c_int {
    return (*edge_table.offset((2 as libc::c_int * ecur) as isize)
        == *edge_table.offset((2 as libc::c_int * elast) as isize)
        && *edge_table.offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize)
            == *edge_table.offset((2 as libc::c_int * elast + 1 as libc::c_int) as isize)
        || *edge_table.offset((2 as libc::c_int * ecur) as isize)
            == *edge_table.offset((2 as libc::c_int * elast + 1 as libc::c_int) as isize)
            && *edge_table.offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize)
                == *edge_table.offset((2 as libc::c_int * elast) as isize))
        as libc::c_int;
}
unsafe extern "C" fn get_polygon_solids(
    mut nt: libc::c_int,
    mut E: SparseMatrix,
    mut ncomps: libc::c_int,
    mut comps_ptr: *mut libc::c_int,
    mut comps: *mut libc::c_int,
    mut mask: *mut libc::c_int,
    mut x_poly: *mut libc::c_double,
    mut polys: *mut SparseMatrix,
) {
    let mut edge_table: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut half_edges: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut n: libc::c_int = (*E).m;
    let mut ie: *mut libc::c_int = (*E).ia;
    let mut je: *mut libc::c_int = (*E).ja;
    let mut e: *mut libc::c_int = (*E).a as *mut libc::c_int;
    let mut ne: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut cycle: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cycle_head: libc::c_int = 0 as libc::c_int;
    let mut edge_cycle_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut NOT_ON_CYCLE: libc::c_int = -(1 as libc::c_int);
    let mut emask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut elist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut edim: libc::c_int = 3 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut duplicate: libc::c_int = 0;
    let mut ee: libc::c_int = 0 as libc::c_int;
    let mut ecur: libc::c_int = 0;
    let mut enext: libc::c_int = 0;
    let mut eprev: libc::c_int = 0;
    let mut cur: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut nlink: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut elast: libc::c_int = 0 as libc::c_int;
    let mut etail: libc::c_int = 0;
    let mut tail: libc::c_int = 0;
    let mut ehead: libc::c_int = 0;
    let mut efirst: libc::c_int = 0;
    let mut DEBUG_CYCLE: libc::c_int = 0 as libc::c_int;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    ne = (*E).nz;
    edge_table = gv_calloc(
        (ne * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    half_edges = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_INTEGER as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    ne = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        j = *ie.offset(i as isize);
        while j < *ie.offset((i + 1 as libc::c_int) as isize) {
            if j
                < *ie.offset(n as isize) - *ie.offset(0 as libc::c_int as isize)
                    - 1 as libc::c_int && i > *je.offset(j as isize)
                && *je.offset(j as isize) == *je.offset((j + 1 as libc::c_int) as isize)
            {
                t1 = *e.offset(j as isize);
                t2 = *e.offset((j + 1 as libc::c_int) as isize);
                jj = *je.offset(j as isize);
                if jj < n {} else {
                    __assert_fail(
                        b"jj < n\0" as *const u8 as *const libc::c_char,
                        b"make_map.c\0" as *const u8 as *const libc::c_char,
                        732 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 95],
                            &[libc::c_char; 95],
                        >(
                            b"void get_polygon_solids(int, SparseMatrix, int, int *, int *, int *, double *, SparseMatrix *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                *edge_table.offset((ne * 2 as libc::c_int) as isize) = t1;
                *edge_table
                    .offset((ne * 2 as libc::c_int + 1 as libc::c_int) as isize) = t2;
                half_edges = SparseMatrix_coordinate_form_add_entry(
                    half_edges,
                    i,
                    jj,
                    &mut ne as *mut libc::c_int as *mut libc::c_void,
                );
                half_edges = SparseMatrix_coordinate_form_add_entry(
                    half_edges,
                    jj,
                    i,
                    &mut ne as *mut libc::c_int as *mut libc::c_void,
                );
                ne += 1;
                *edge_table.offset((ne * 2 as libc::c_int) as isize) = t2;
                *edge_table
                    .offset((ne * 2 as libc::c_int + 1 as libc::c_int) as isize) = t1;
                half_edges = SparseMatrix_coordinate_form_add_entry(
                    half_edges,
                    i,
                    jj,
                    &mut ne as *mut libc::c_int as *mut libc::c_void,
                );
                half_edges = SparseMatrix_coordinate_form_add_entry(
                    half_edges,
                    jj,
                    i,
                    &mut ne as *mut libc::c_int as *mut libc::c_void,
                );
                ne += 1;
                j += 1;
            }
            j += 1;
        }
        i += 1;
    }
    if (*E).nz >= ne {} else {
        __assert_fail(
            b"E->nz >= ne\0" as *const u8 as *const libc::c_char,
            b"make_map.c\0" as *const u8 as *const libc::c_char,
            750 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"void get_polygon_solids(int, SparseMatrix, int, int *, int *, int *, double *, SparseMatrix *)\0",
            ))
                .as_ptr(),
        );
    }
    cycle = gv_calloc(
        (ne * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    B = SparseMatrix_from_coordinate_format_not_compacted(half_edges);
    SparseMatrix_delete(half_edges);
    half_edges = B;
    edge_cycle_map = gv_calloc(
        ne as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    emask = gv_calloc(
        ne as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < ne {
        *edge_cycle_map.offset(i as isize) = NOT_ON_CYCLE;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < ne {
        *emask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    ie = (*half_edges).ia;
    je = (*half_edges).ja;
    e = (*half_edges).a as *mut libc::c_int;
    elist = gv_calloc(
        (nt * 3 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < nt {
        *elist.offset((i * edim + 2 as libc::c_int) as isize) = 0 as libc::c_int;
        i += 1;
    }
    *polys = SparseMatrix_new(
        ncomps,
        nt,
        1 as libc::c_int,
        MATRIX_TYPE_INTEGER as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < ncomps {
        if DEBUG_CYCLE != 0 {
            fprintf(
                stderr,
                b"\n ============  comp %d has %d members\n\0" as *const u8
                    as *const libc::c_char,
                i,
                *comps_ptr.offset((i + 1 as libc::c_int) as isize)
                    - *comps_ptr.offset(i as isize),
            );
        }
        k = *comps_ptr.offset(i as isize);
        while k < *comps_ptr.offset((i + 1 as libc::c_int) as isize) {
            ii = *comps.offset(k as isize);
            *mask.offset(ii as isize) = i;
            duplicate = NO_DUPLICATE as libc::c_int;
            if DEBUG_CYCLE != 0 {
                fprintf(
                    stderr,
                    b"member = %d has %d neighbors\n\0" as *const u8
                        as *const libc::c_char,
                    ii,
                    *ie.offset((ii + 1 as libc::c_int) as isize)
                        - *ie.offset(ii as isize),
                );
            }
            j = *ie.offset(ii as isize);
            while j < *ie.offset((ii + 1 as libc::c_int) as isize) {
                jj = *je.offset(j as isize);
                ee = *e.offset(j as isize);
                t1 = *edge_table.offset((2 as libc::c_int * ee) as isize);
                if DEBUG_CYCLE != 0 {
                    fprintf(
                        stderr,
                        b" linked with %d using half-edge %d, {head,tail} of the edge = {%d, %d}\n\0"
                            as *const u8 as *const libc::c_char,
                        jj,
                        ee,
                        t1,
                        *edge_table
                            .offset((2 as libc::c_int * ee + 1 as libc::c_int) as isize),
                    );
                }
                nlink = *elist.offset((t1 * edim + 2 as libc::c_int) as isize)
                    % 2 as libc::c_int;
                *elist.offset((t1 * edim + nlink) as isize) = ee;
                let ref mut fresh18 = *elist
                    .offset((t1 * edim + 2 as libc::c_int) as isize);
                *fresh18 += 1;
                if *edge_cycle_map.offset(ee as isize) != NOT_ON_CYCLE {
                    duplicate = ee;
                }
                *emask.offset(ee as isize) = ii;
                j += 1;
            }
            if duplicate == NO_DUPLICATE as libc::c_int {
                ecur = ee;
                cycle_head = ecur;
                *cycle
                    .offset(
                        (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                    ) = ecur;
                *cycle.offset((2 as libc::c_int * ecur) as isize) = ecur;
                *edge_cycle_map.offset(ecur as isize) = 1 as libc::c_int;
                cur = *edge_table.offset((2 as libc::c_int * ecur) as isize);
                head = cur;
                next = *edge_table
                    .offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
                if DEBUG_CYCLE != 0 {
                    fprintf(
                        stderr,
                        b"NEW CYCLE\n starting with edge %d, {head,tail}={%d,%d}\n\0"
                            as *const u8 as *const libc::c_char,
                        ee,
                        head,
                        next,
                    );
                }
                while next != head {
                    enext = *elist.offset((edim * next + 0 as libc::c_int) as isize);
                    if *edge_table.offset((2 as libc::c_int * enext) as isize) == cur
                        && *edge_table
                            .offset(
                                (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                            ) == next
                        || *edge_table.offset((2 as libc::c_int * enext) as isize)
                            == next
                            && *edge_table
                                .offset(
                                    (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                                ) == cur
                    {
                        enext = *elist.offset((edim * next + 1 as libc::c_int) as isize);
                    }
                    if DEBUG_CYCLE != 0 {
                        fprintf(
                            stderr,
                            b"cur edge = %d, next edge %d, {head,tail}={%d,%d},\n\0"
                                as *const u8 as *const libc::c_char,
                            ecur,
                            enext,
                            *edge_table.offset((2 as libc::c_int * enext) as isize),
                            *edge_table
                                .offset(
                                    (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                                ),
                        );
                    }
                    nn = *edge_table.offset((2 as libc::c_int * enext) as isize);
                    if nn == next {
                        nn = *edge_table
                            .offset(
                                (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                            );
                    }
                    *cycle
                        .offset(
                            (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                        ) = *cycle
                        .offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
                    *cycle.offset((2 as libc::c_int * enext) as isize) = ecur;
                    *cycle
                        .offset(
                            (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                        ) = enext;
                    *cycle.offset((2 as libc::c_int * ee) as isize) = enext;
                    *edge_cycle_map.offset(enext as isize) = 1 as libc::c_int;
                    ecur = enext;
                    cur = next;
                    next = nn;
                }
                if DEBUG_CYCLE != 0 {
                    cycle_print(ee, cycle, edge_table);
                }
            } else {
                ee = duplicate;
                ecur = ee;
                while *emask.offset(ecur as isize) == ii {
                    ecur = *cycle
                        .offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
                }
                if DEBUG_CYCLE != 0 {
                    fprintf(
                        stderr,
                        b" duplicating edge = %d, starting from the a non-duplicating edge %d, search backwards\n\0"
                            as *const u8 as *const libc::c_char,
                        ee,
                        ecur,
                    );
                }
                ecur = *cycle.offset((2 as libc::c_int * ecur) as isize);
                efirst = ecur;
                while *emask.offset(ecur as isize) == ii {
                    if DEBUG_CYCLE != 0 {
                        fprintf(
                            stderr,
                            b" remove edge %d (%d--%d)\n\0" as *const u8
                                as *const libc::c_char,
                            ecur,
                            *edge_table.offset((2 as libc::c_int * ecur) as isize),
                            *edge_table
                                .offset(
                                    (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                                ),
                        );
                    }
                    *edge_cycle_map.offset(ecur as isize) = NOT_ON_CYCLE;
                    enext = *cycle
                        .offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
                    eprev = *cycle.offset((2 as libc::c_int * ecur) as isize);
                    *cycle
                        .offset(
                            (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                        ) = ecur;
                    *cycle.offset((2 as libc::c_int * ecur) as isize) = ecur;
                    *cycle
                        .offset(
                            (2 as libc::c_int * eprev + 1 as libc::c_int) as isize,
                        ) = enext;
                    *cycle.offset((2 as libc::c_int * enext) as isize) = eprev;
                    elast = ecur;
                    ecur = eprev;
                }
                if DEBUG_CYCLE != 0 {
                    fprintf(
                        stderr,
                        b"remaining (broken) cycle = \0" as *const u8
                            as *const libc::c_char,
                    );
                    cycle_print(
                        *cycle
                            .offset(
                                (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                            ),
                        cycle,
                        edge_table,
                    );
                }
                ehead = ecur;
                etail = *cycle
                    .offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
                cycle_head = ehead;
                head = *edge_table
                    .offset((2 as libc::c_int * ehead + 1 as libc::c_int) as isize);
                tail = *edge_table.offset((2 as libc::c_int * etail) as isize);
                ecur = *elist.offset((edim * head + 0 as libc::c_int) as isize);
                if same_edge(ecur, elast, edge_table) != 0 {
                    ecur = *elist.offset((edim * head + 1 as libc::c_int) as isize);
                }
                if DEBUG_CYCLE != 0 {
                    fprintf(
                        stderr,
                        b"forwarding now from edge %d = {%d, %d}, try to reach vtx %d, first edge from voro = %d\n\0"
                            as *const u8 as *const libc::c_char,
                        ehead,
                        *edge_table.offset((2 as libc::c_int * ehead) as isize),
                        *edge_table
                            .offset(
                                (2 as libc::c_int * ehead + 1 as libc::c_int) as isize,
                            ),
                        tail,
                        ecur,
                    );
                }
                *cycle
                    .offset(
                        (2 as libc::c_int * ehead + 1 as libc::c_int) as isize,
                    ) = ecur;
                *cycle.offset((2 as libc::c_int * ecur) as isize) = ehead;
                *cycle.offset((2 as libc::c_int * etail) as isize) = ecur;
                *cycle
                    .offset(
                        (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                    ) = etail;
                if same_edge(ecur, efirst, edge_table) != 0 {
                    if DEBUG_CYCLE != 0 {
                        fprintf(
                            stderr,
                            b"this voro cell fill in a hole completely!!!!\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    *edge_cycle_map.offset(ecur as isize) = 1 as libc::c_int;
                    cur = *edge_table.offset((2 as libc::c_int * ecur) as isize);
                    head = cur;
                    next = *edge_table
                        .offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
                    if DEBUG_CYCLE != 0 {
                        fprintf(
                            stderr,
                            b"starting with edge %d, {head,tail}={%d,%d}\n\0"
                                as *const u8 as *const libc::c_char,
                            ecur,
                            head,
                            next,
                        );
                    }
                    while next != tail {
                        enext = *elist.offset((edim * next + 0 as libc::c_int) as isize);
                        if *edge_table.offset((2 as libc::c_int * enext) as isize) == cur
                            && *edge_table
                                .offset(
                                    (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                                ) == next
                            || *edge_table.offset((2 as libc::c_int * enext) as isize)
                                == next
                                && *edge_table
                                    .offset(
                                        (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                                    ) == cur
                        {
                            enext = *elist
                                .offset((edim * next + 1 as libc::c_int) as isize);
                        }
                        if DEBUG_CYCLE != 0 {
                            fprintf(
                                stderr,
                                b"cur edge = %d, next edge %d, {head,tail}={%d,%d},\n\0"
                                    as *const u8 as *const libc::c_char,
                                ecur,
                                enext,
                                *edge_table.offset((2 as libc::c_int * enext) as isize),
                                *edge_table
                                    .offset(
                                        (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                                    ),
                            );
                        }
                        nn = *edge_table.offset((2 as libc::c_int * enext) as isize);
                        if nn == next {
                            nn = *edge_table
                                .offset(
                                    (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                                );
                        }
                        *cycle
                            .offset(
                                (2 as libc::c_int * enext + 1 as libc::c_int) as isize,
                            ) = *cycle
                            .offset(
                                (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                            );
                        *cycle.offset((2 as libc::c_int * enext) as isize) = ecur;
                        *cycle
                            .offset(
                                (2 as libc::c_int * ecur + 1 as libc::c_int) as isize,
                            ) = enext;
                        *cycle.offset((2 as libc::c_int * etail) as isize) = enext;
                        *edge_cycle_map.offset(enext as isize) = 1 as libc::c_int;
                        ecur = enext;
                        cur = next;
                        next = nn;
                    }
                }
                if DEBUG_CYCLE != 0 && 0 as libc::c_int != 0 {
                    cycle_print(etail, cycle, edge_table);
                    plot_cycle(etail, cycle, edge_table, x_poly);
                    printf(b",\0" as *const u8 as *const libc::c_char);
                }
            }
            k += 1;
        }
        ecur = cycle_head;
        loop {
            enext = *cycle.offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
            if !(enext != cycle_head) {
                break;
            }
            *edge_cycle_map.offset(ecur as isize) = NOT_ON_CYCLE;
            head = *edge_table.offset((2 as libc::c_int * ecur) as isize);
            SparseMatrix_coordinate_form_add_entry(
                *polys,
                i,
                head,
                &mut i as *mut libc::c_int as *mut libc::c_void,
            );
            ecur = enext;
        }
        *edge_cycle_map.offset(ecur as isize) = NOT_ON_CYCLE;
        head = *edge_table.offset((2 as libc::c_int * ecur) as isize);
        tail = *edge_table.offset((2 as libc::c_int * ecur + 1 as libc::c_int) as isize);
        SparseMatrix_coordinate_form_add_entry(
            *polys,
            i,
            head,
            &mut i as *mut libc::c_int as *mut libc::c_void,
        );
        SparseMatrix_coordinate_form_add_entry(
            *polys,
            i,
            tail,
            &mut i as *mut libc::c_int as *mut libc::c_void,
        );
        i += 1;
    }
    B = SparseMatrix_from_coordinate_format_not_compacted(*polys);
    SparseMatrix_delete(*polys);
    *polys = B;
    SparseMatrix_delete(half_edges);
    free(cycle as *mut libc::c_void);
    free(edge_cycle_map as *mut libc::c_void);
    free(elist as *mut libc::c_void);
    free(emask as *mut libc::c_void);
    free(edge_table as *mut libc::c_void);
}
unsafe extern "C" fn get_polygons(
    mut exclude_random: libc::c_int,
    mut n: libc::c_int,
    mut nrandom: libc::c_int,
    mut dim: libc::c_int,
    mut graph: SparseMatrix,
    mut grouping: *mut libc::c_int,
    mut nt: libc::c_int,
    mut Tp: *mut Triangle,
    mut E: SparseMatrix,
    mut nverts: *mut libc::c_int,
    mut x_poly: *mut *mut libc::c_double,
    mut npolys: *mut libc::c_int,
    mut poly_lines: *mut SparseMatrix,
    mut polys: *mut SparseMatrix,
    mut polys_groups: *mut *mut libc::c_int,
    mut poly_point_map: *mut SparseMatrix,
    mut country_graph: *mut SparseMatrix,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut groups: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut maxgrp: libc::c_int = 0;
    let mut comps: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut comps_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ncomps: libc::c_int = 0;
    let mut GRP_RANDOM: libc::c_int = 0;
    let mut GRP_BBOX: libc::c_int = 0;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    if dim == 2 as libc::c_int {} else {
        __assert_fail(
            b"dim == 2\0" as *const u8 as *const libc::c_char,
            b"make_map.c\0" as *const u8 as *const libc::c_char,
            956 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 194],
                &[libc::c_char; 194],
            >(
                b"void get_polygons(int, int, int, int, SparseMatrix, int *, int, struct Triangle *, SparseMatrix, int *, double **, int *, SparseMatrix *, SparseMatrix *, int **, SparseMatrix *, SparseMatrix *)\0",
            ))
                .as_ptr(),
        );
    }
    *nverts = nt;
    groups = gv_calloc(
        (n + nrandom) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    maxgrp = *grouping.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < n {
        maxgrp = if maxgrp > *grouping.offset(i as isize) {
            maxgrp
        } else {
            *grouping.offset(i as isize)
        };
        *groups.offset(i as isize) = *grouping.offset(i as isize);
        i += 1;
    }
    GRP_RANDOM = maxgrp + 1 as libc::c_int;
    GRP_BBOX = maxgrp + 2 as libc::c_int;
    i = n;
    while i < n + nrandom - 4 as libc::c_int {
        *groups.offset(i as isize) = GRP_RANDOM;
        i += 1;
    }
    i = n + nrandom - 4 as libc::c_int;
    while i < n + nrandom {
        *groups.offset(i as isize) = GRP_BBOX;
        i += 1;
    }
    mask = gv_calloc(
        (if n + nrandom > 2 as libc::c_int * nt {
            n + nrandom
        } else {
            2 as libc::c_int * nt
        }) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    conn_comp(n + nrandom, E, groups, poly_point_map);
    ncomps = (**poly_point_map).m;
    comps = (**poly_point_map).ja;
    comps_ptr = (**poly_point_map).ia;
    if exclude_random != 0 {
        i = ncomps - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *groups
                .offset(*comps.offset(*comps_ptr.offset(i as isize) as isize) as isize)
                != GRP_RANDOM
                && *groups
                    .offset(
                        *comps.offset(*comps_ptr.offset(i as isize) as isize) as isize,
                    ) != GRP_BBOX
            {
                break;
            }
            i -= 1;
        }
        ncomps = i + 1 as libc::c_int;
        if Verbose != 0 {
            fprintf(
                stderr,
                b"ncomps = %d\n\0" as *const u8 as *const libc::c_char,
                ncomps,
            );
        }
    } else {
        i = ncomps - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *groups
                .offset(*comps.offset(*comps_ptr.offset(i as isize) as isize) as isize)
                != GRP_BBOX
            {
                break;
            }
            i -= 1;
        }
        ncomps = i + 1 as libc::c_int;
        if Verbose != 0 {
            fprintf(
                stderr,
                b" ncomops = %d\n\0" as *const u8 as *const libc::c_char,
                ncomps,
            );
        }
    }
    *npolys = ncomps;
    *x_poly = gv_calloc(
        (dim * nt) as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < nt {
        j = 0 as libc::c_int;
        while j < dim {
            *(*x_poly)
                .offset(
                    (i * dim + j) as isize,
                ) = (*Tp.offset(i as isize)).center[j as usize];
            j += 1;
        }
        i += 1;
    }
    get_poly_lines(
        exclude_random,
        nt,
        graph,
        E,
        ncomps,
        comps_ptr,
        comps,
        groups,
        mask,
        poly_lines,
        polys_groups,
        GRP_RANDOM,
        GRP_BBOX,
    );
    get_polygon_solids(nt, E, ncomps, comps_ptr, comps, mask, *x_poly, polys);
    B = get_country_graph(n, E, groups, GRP_RANDOM, GRP_BBOX);
    *country_graph = B;
    free(groups as *mut libc::c_void);
    free(mask as *mut libc::c_void);
}
unsafe extern "C" fn make_map_internal(
    mut exclude_random: libc::c_int,
    mut include_OK_points: libc::c_int,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x0: *mut libc::c_double,
    mut grouping0: *mut libc::c_int,
    mut graph: SparseMatrix,
    mut bounding_box_margin: *mut libc::c_double,
    mut nrandom: *mut libc::c_int,
    mut nedgep: libc::c_int,
    mut shore_depth_tol: libc::c_double,
    mut xcombined: *mut *mut libc::c_double,
    mut nverts: *mut libc::c_int,
    mut x_poly: *mut *mut libc::c_double,
    mut npolys: *mut libc::c_int,
    mut poly_lines: *mut SparseMatrix,
    mut polys: *mut SparseMatrix,
    mut polys_groups: *mut *mut libc::c_int,
    mut poly_point_map: *mut SparseMatrix,
    mut country_graph: *mut SparseMatrix,
    mut highlight_cluster: libc::c_int,
    mut flag: *mut libc::c_int,
) -> libc::c_int {
    let mut xmax: [libc::c_double; 2] = [0.; 2];
    let mut xmin: [libc::c_double; 2] = [0.; 2];
    let mut area: libc::c_double = 0.;
    let mut x: *mut libc::c_double = x0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut qt: QuadTree = 0 as *mut QuadTree_struct;
    let mut dim2: libc::c_int = 2 as libc::c_int;
    let mut nn: libc::c_int = 0 as libc::c_int;
    let mut max_qtree_level: libc::c_int = 10 as libc::c_int;
    let mut ymin: [libc::c_double; 2] = [0.; 2];
    let mut min: libc::c_double = 0.;
    let mut imin: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    let mut nzok: libc::c_int = 0 as libc::c_int;
    let mut nzok0: libc::c_int = 0 as libc::c_int;
    let mut nt: libc::c_int = 0;
    let mut xran: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut point: [libc::c_double; 2] = [0.; 2];
    let mut Tp: *mut Triangle = 0 as *mut Triangle;
    let mut E: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut boxsize: [libc::c_double; 2] = [0.; 2];
    let mut INCLUDE_OK_POINTS: libc::c_int = include_OK_points;
    let mut grouping: *mut libc::c_int = grouping0;
    let mut HIGHLIGHT_SET: libc::c_int = highlight_cluster;
    *flag = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < dim2 {
        xmax[j as usize] = *x.offset(j as isize);
        xmin[j as usize] = *x.offset(j as isize);
        j += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < dim2 {
            xmax[j
                as usize] = if xmax[j as usize] > *x.offset((i * dim + j) as isize) {
                xmax[j as usize]
            } else {
                *x.offset((i * dim + j) as isize)
            };
            xmin[j
                as usize] = if xmin[j as usize] < *x.offset((i * dim + j) as isize) {
                xmin[j as usize]
            } else {
                *x.offset((i * dim + j) as isize)
            };
            j += 1;
        }
        i += 1;
    }
    boxsize[0 as libc::c_int
        as usize] = xmax[0 as libc::c_int as usize] - xmin[0 as libc::c_int as usize];
    boxsize[1 as libc::c_int
        as usize] = xmax[1 as libc::c_int as usize] - xmin[1 as libc::c_int as usize];
    area = boxsize[0 as libc::c_int as usize] * boxsize[1 as libc::c_int as usize];
    if *nrandom == 0 as libc::c_int {
        *nrandom = n;
    } else if *nrandom < 0 as libc::c_int {
        *nrandom = -*nrandom * n;
    } else if *nrandom < 4 as libc::c_int {
        *nrandom = 0 as libc::c_int;
    } else {
        *nrandom -= 4 as libc::c_int;
    }
    if shore_depth_tol < 0 as libc::c_int as libc::c_double {
        shore_depth_tol = sqrt(area / n as libc::c_double);
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"nrandom=%d shore_depth_tol=%.08f\n\0" as *const u8 as *const libc::c_char,
            *nrandom,
            shore_depth_tol,
        );
    }
    let mut nz_0: libc::c_int = 0;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut np: libc::c_int = nedgep;
    if !graph.is_null() && np != 0 {
        fprintf(stderr, b"add art np = %d\n\0" as *const u8 as *const libc::c_char, np);
        nz_0 = (*graph).nz;
        y = gv_calloc(
            (dim * n + dim * nz_0 * np) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        i = 0 as libc::c_int;
        while i < n * dim {
            *y.offset(i as isize) = *x.offset(i as isize);
            i += 1;
        }
        grouping = gv_calloc(
            (n + nz_0 * np) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < n {
            *grouping.offset(i as isize) = *grouping0.offset(i as isize);
            i += 1;
        }
        nz_0 = n;
        i = 0 as libc::c_int;
        while i < (*graph).m {
            j = *((*graph).ia).offset(i as isize);
            while j < *((*graph).ia).offset((i + 1 as libc::c_int) as isize) {
                if HIGHLIGHT_SET == 0
                    || *grouping.offset(i as isize)
                        == *grouping.offset(*((*graph).ja).offset(j as isize) as isize)
                        && *grouping.offset(i as isize) == HIGHLIGHT_SET
                {
                    t = 0 as libc::c_int;
                    while t < np {
                        k = 0 as libc::c_int;
                        while k < dim {
                            *y
                                .offset(
                                    (nz_0 * dim + k) as isize,
                                ) = t as libc::c_double / np as libc::c_double
                                * *x.offset((i * dim + k) as isize)
                                + (1 as libc::c_int as libc::c_double
                                    - t as libc::c_double / np as libc::c_double)
                                    * *x
                                        .offset(
                                            (*((*graph).ja).offset(j as isize) * dim + k) as isize,
                                        );
                            k += 1;
                        }
                        if n + (nz_0 - n) * np + t < n + nz_0 * np
                            && n + (nz_0 - n) * np + t >= 0 as libc::c_int
                        {} else {
                            __assert_fail(
                                b"n + (nz-n)*np + t < n + nz*np && n + (nz-n)*np + t >= 0\0"
                                    as *const u8 as *const libc::c_char,
                                b"make_map.c\0" as *const u8 as *const libc::c_char,
                                1104 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 223],
                                    &[libc::c_char; 223],
                                >(
                                    b"int make_map_internal(int, int, int, int, double *, int *, SparseMatrix, double *, int *, int, double, double **, int *, double **, int *, SparseMatrix *, SparseMatrix *, int **, SparseMatrix *, SparseMatrix *, int, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        if t as libc::c_double / np as libc::c_double > 0.5f64 {
                            *grouping
                                .offset(nz_0 as isize) = *grouping.offset(i as isize);
                        } else {
                            *grouping
                                .offset(
                                    nz_0 as isize,
                                ) = *grouping
                                .offset(*((*graph).ja).offset(j as isize) as isize);
                        }
                        nz_0 += 1;
                        t += 1;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        fprintf(
            stderr,
            b"after adding edge points, n:%d->%d\n\0" as *const u8
                as *const libc::c_char,
            n,
            nz_0,
        );
        n = nz_0;
        x = y;
        qt = QuadTree_new_from_point_list(dim, nz_0, max_qtree_level, y);
    } else {
        qt = QuadTree_new_from_point_list(dim, n, max_qtree_level, x);
    }
    graph = 0 as SparseMatrix;
    if *nrandom != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < dim2 {
            if *bounding_box_margin.offset(i as isize)
                > 0 as libc::c_int as libc::c_double
            {
                xmin[i as usize] -= *bounding_box_margin.offset(i as isize);
                xmax[i as usize] += *bounding_box_margin.offset(i as isize);
            } else if *bounding_box_margin.offset(i as isize)
                    < 0 as libc::c_int as libc::c_double
                {
                xmin[i as usize]
                    -= boxsize[i as usize] * -*bounding_box_margin.offset(i as isize);
                xmax[i as usize]
                    += boxsize[i as usize] * -*bounding_box_margin.offset(i as isize);
            } else {
                xmin[i as usize]
                    -= if boxsize[i as usize] * 0.2f64 > 2.0f64 * shore_depth_tol {
                        boxsize[i as usize] * 0.2f64
                    } else {
                        2.0f64 * shore_depth_tol
                    };
                xmax[i as usize]
                    += if boxsize[i as usize] * 0.2f64
                        > 2 as libc::c_int as libc::c_double * shore_depth_tol
                    {
                        boxsize[i as usize] * 0.2f64
                    } else {
                        2 as libc::c_int as libc::c_double * shore_depth_tol
                    };
            }
            i += 1;
        }
        if Verbose != 0 {
            let mut bbm0: libc::c_double = *bounding_box_margin
                .offset(0 as libc::c_int as isize);
            let mut bbm1: libc::c_double = *bounding_box_margin
                .offset(1 as libc::c_int as isize);
            if bbm0 > 0 as libc::c_int as libc::c_double {
                fprintf(
                    stderr,
                    b"bounding box margin: %.06f\0" as *const u8 as *const libc::c_char,
                    bbm0,
                );
            } else if bbm0 < 0 as libc::c_int as libc::c_double {
                fprintf(
                    stderr,
                    b"bounding box margin: (%.06f * %.06f)\0" as *const u8
                        as *const libc::c_char,
                    boxsize[0 as libc::c_int as usize],
                    -bbm0,
                );
            } else {
                fprintf(
                    stderr,
                    b"bounding box margin: %.06f\0" as *const u8 as *const libc::c_char,
                    if boxsize[0 as libc::c_int as usize] * 0.2f64
                        > 2 as libc::c_int as libc::c_double * shore_depth_tol
                    {
                        boxsize[0 as libc::c_int as usize] * 0.2f64
                    } else {
                        2 as libc::c_int as libc::c_double * shore_depth_tol
                    },
                );
            }
            if bbm1 > 0 as libc::c_int as libc::c_double {
                fprintf(stderr, b" %.06f\n\0" as *const u8 as *const libc::c_char, bbm1);
            } else if bbm1 < 0 as libc::c_int as libc::c_double {
                fprintf(
                    stderr,
                    b" (%.06f * %.06f)\n\0" as *const u8 as *const libc::c_char,
                    boxsize[1 as libc::c_int as usize],
                    -bbm1,
                );
            } else {
                fprintf(
                    stderr,
                    b" %.06f\n\0" as *const u8 as *const libc::c_char,
                    if boxsize[1 as libc::c_int as usize] * 0.2f64
                        > 2 as libc::c_int as libc::c_double * shore_depth_tol
                    {
                        boxsize[1 as libc::c_int as usize] * 0.2f64
                    } else {
                        2 as libc::c_int as libc::c_double * shore_depth_tol
                    },
                );
            }
        }
        if *nrandom < 0 as libc::c_int {
            let mut n1: libc::c_double = 0.;
            let mut n2: libc::c_double = 0.;
            let mut area2: libc::c_double = 0.;
            area2 = (xmax[1 as libc::c_int as usize] - xmin[1 as libc::c_int as usize])
                * (xmax[0 as libc::c_int as usize] - xmin[0 as libc::c_int as usize]);
            n1 = area2 as libc::c_int as libc::c_double
                / (shore_depth_tol * shore_depth_tol);
            n2 = n as libc::c_double * (area2 as libc::c_int as libc::c_double / area);
            *nrandom = (if n1 > n2 { n1 } else { n2 }) as libc::c_int;
        }
        srand(123 as libc::c_int as libc::c_uint);
        xran = gv_calloc(
            ((*nrandom + 4 as libc::c_int) * dim2) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        nz = 0 as libc::c_int;
        if INCLUDE_OK_POINTS != 0 {
            nzok = *nrandom - 1 as libc::c_int;
            nzok0 = nzok;
            if grouping == grouping0 {
                let mut grouping2: *mut libc::c_int = gv_calloc(
                    (n + *nrandom) as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                memcpy(
                    grouping2 as *mut libc::c_void,
                    grouping as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(n as libc::c_ulong),
                );
                grouping = grouping2;
            } else {
                grouping = gv_recalloc(
                    grouping as *mut libc::c_void,
                    n as size_t,
                    (n + *nrandom) as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
            }
        }
        nn = n;
        i = 0 as libc::c_int;
        while i < *nrandom {
            j = 0 as libc::c_int;
            while j < dim2 {
                point[j
                    as usize] = xmin[j as usize]
                    + (xmax[j as usize] - xmin[j as usize]) * drand();
                j += 1;
            }
            QuadTree_get_nearest(
                qt,
                point.as_mut_ptr(),
                ymin.as_mut_ptr(),
                &mut imin,
                &mut min,
                flag,
            );
            if *flag == 0 {} else {
                __assert_fail(
                    b"!(*flag)\0" as *const u8 as *const libc::c_char,
                    b"make_map.c\0" as *const u8 as *const libc::c_char,
                    1185 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 223],
                        &[libc::c_char; 223],
                    >(
                        b"int make_map_internal(int, int, int, int, double *, int *, SparseMatrix, double *, int *, int, double, double **, int *, double **, int *, SparseMatrix *, SparseMatrix *, int **, SparseMatrix *, SparseMatrix *, int, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if min > shore_depth_tol {
                j = 0 as libc::c_int;
                while j < dim2 {
                    *xran.offset((nz * dim2 + j) as isize) = point[j as usize];
                    j += 1;
                }
                nz += 1;
            } else if INCLUDE_OK_POINTS != 0
                    && min > shore_depth_tol / 10 as libc::c_int as libc::c_double
                {
                j = 0 as libc::c_int;
                while j < dim2 {
                    *xran.offset((nzok * dim2 + j) as isize) = point[j as usize];
                    j += 1;
                }
                let fresh19 = nn;
                nn = nn + 1;
                *grouping.offset(fresh19 as isize) = *grouping.offset(imin as isize);
                nzok -= 1;
            }
            i += 1;
        }
        *nrandom = nz;
        if Verbose != 0 {
            fprintf(
                stderr,
                b"nn nrandom=%d\n\0" as *const u8 as *const libc::c_char,
                *nrandom,
            );
        }
    } else {
        xran = gv_calloc(
            (4 as libc::c_int * dim2) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < dim2 {
        xmin[i as usize] -= 0.2f64 * (xmax[i as usize] - xmin[i as usize]);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < dim2 {
        xmax[i as usize] += 0.2f64 * (xmax[i as usize] - xmin[i as usize]);
        i += 1;
    }
    i = *nrandom;
    j = 0 as libc::c_int;
    while j < dim2 {
        *xran.offset((i * dim2 + j) as isize) = xmin[j as usize];
        j += 1;
    }
    i += 1;
    j = 0 as libc::c_int;
    while j < dim2 {
        *xran.offset((i * dim2 + j) as isize) = xmax[j as usize];
        j += 1;
    }
    i += 1;
    *xran.offset((i * dim2) as isize) = xmin[0 as libc::c_int as usize];
    *xran
        .offset(
            (i * dim2 + 1 as libc::c_int) as isize,
        ) = xmax[1 as libc::c_int as usize];
    i += 1;
    *xran.offset((i * dim2) as isize) = xmax[0 as libc::c_int as usize];
    *xran
        .offset(
            (i * dim2 + 1 as libc::c_int) as isize,
        ) = xmin[1 as libc::c_int as usize];
    *nrandom += 4 as libc::c_int;
    if INCLUDE_OK_POINTS != 0 {
        *xcombined = gv_calloc(
            ((nn + *nrandom) * dim2) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
    } else {
        *xcombined = gv_calloc(
            ((n + *nrandom) * dim2) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < dim2 {
            *(*xcombined)
                .offset((i * dim2 + j) as isize) = *x.offset((i * dim + j) as isize);
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < *nrandom {
        j = 0 as libc::c_int;
        while j < dim2 {
            *(*xcombined)
                .offset(
                    ((i + nn) * dim2 + j) as isize,
                ) = *xran.offset((i * dim + j) as isize);
            j += 1;
        }
        i += 1;
    }
    if INCLUDE_OK_POINTS != 0 {
        i = 0 as libc::c_int;
        while i < nn - n {
            j = 0 as libc::c_int;
            while j < dim2 {
                *(*xcombined)
                    .offset(
                        ((i + n) * dim2 + j) as isize,
                    ) = *xran.offset(((nzok0 - i) * dim + j) as isize);
                j += 1;
            }
            i += 1;
        }
        n = nn;
    }
    let mut nz_1: libc::c_int = 0;
    let mut nh: libc::c_int = 0 as libc::c_int;
    let mut xtemp: *mut libc::c_double = 0 as *mut libc::c_double;
    if HIGHLIGHT_SET != 0 {
        if Verbose != 0 {
            fprintf(
                stderr,
                b" highlight cluster %d, n = %d\n\0" as *const u8 as *const libc::c_char,
                HIGHLIGHT_SET,
                n,
            );
        }
        xtemp = gv_calloc(
            (n * dim) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        nz_1 = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < n {
            if *grouping.offset(i as isize) == HIGHLIGHT_SET {
                nh += 1;
                j = 0 as libc::c_int;
                while j < dim {
                    let fresh20 = nz_1;
                    nz_1 = nz_1 + 1;
                    *xtemp.offset(fresh20 as isize) = *x.offset((i * dim + j) as isize);
                    j += 1;
                }
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < n {
            if *grouping.offset(i as isize) != HIGHLIGHT_SET {
                j = 0 as libc::c_int;
                while j < dim {
                    let fresh21 = nz_1;
                    nz_1 = nz_1 + 1;
                    *xtemp.offset(fresh21 as isize) = *x.offset((i * dim + j) as isize);
                    j += 1;
                }
            }
            i += 1;
        }
        if nz_1 == n * dim {} else {
            __assert_fail(
                b"nz == n*dim\0" as *const u8 as *const libc::c_char,
                b"make_map.c\0" as *const u8 as *const libc::c_char,
                1267 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 223],
                    &[libc::c_char; 223],
                >(
                    b"int make_map_internal(int, int, int, int, double *, int *, SparseMatrix, double *, int *, int, double, double **, int *, double **, int *, SparseMatrix *, SparseMatrix *, int **, SparseMatrix *, SparseMatrix *, int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        i = 0 as libc::c_int;
        while i < nh {
            *grouping.offset(i as isize) = 1 as libc::c_int;
            i += 1;
        }
        i = nh;
        while i < n {
            *grouping.offset(i as isize) = 2 as libc::c_int;
            i += 1;
        }
        memcpy(
            *xcombined as *mut libc::c_void,
            xtemp as *const libc::c_void,
            ((n * dim) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *nrandom += n - nh;
        n = nh;
        if Verbose != 0 {
            fprintf(stderr, b"nh = %d\n\0" as *const u8 as *const libc::c_char, nh);
        }
        free(xtemp as *mut libc::c_void);
    }
    get_tri(n + *nrandom, dim2, *xcombined, &mut nt, &mut Tp, &mut E);
    get_polygons(
        exclude_random,
        n,
        *nrandom,
        dim2,
        graph,
        grouping,
        nt,
        Tp,
        E,
        nverts,
        x_poly,
        npolys,
        poly_lines,
        polys,
        polys_groups,
        poly_point_map,
        country_graph,
    );
    SparseMatrix_delete(E);
    free(Tp as *mut libc::c_void);
    free(xran as *mut libc::c_void);
    if grouping != grouping0 {
        free(grouping as *mut libc::c_void);
    }
    if x != x0 {
        free(x as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_point(
    mut n: *mut libc::c_int,
    mut igrp: libc::c_int,
    mut x: *mut *mut libc::c_double,
    mut nmax: *mut libc::c_int,
    mut point: *mut libc::c_double,
    mut groups: *mut *mut libc::c_int,
) {
    if *n >= *nmax {
        let mut old_nmax: libc::c_int = *nmax;
        *nmax = (if 0.2f64 as libc::c_int * *n > 20 as libc::c_int {
            0.2f64 as libc::c_int * *n
        } else {
            20 as libc::c_int
        }) + *n;
        *x = gv_recalloc(
            *x as *mut libc::c_void,
            (2 as libc::c_int * old_nmax) as size_t,
            (2 as libc::c_int * *nmax) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        *groups = gv_recalloc(
            *groups as *mut libc::c_void,
            old_nmax as size_t,
            *nmax as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
    }
    *(*x)
        .offset(
            (*n * 2 as libc::c_int) as isize,
        ) = *point.offset(0 as libc::c_int as isize);
    *(*x)
        .offset(
            (*n * 2 as libc::c_int + 1 as libc::c_int) as isize,
        ) = *point.offset(1 as libc::c_int as isize);
    *(*groups).offset(*n as isize) = igrp;
    *n += 1;
}
unsafe extern "C" fn get_boundingbox(
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut bbox: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let ref mut fresh22 = *bbox.offset(1 as libc::c_int as isize);
    *fresh22 = *x.offset(0 as libc::c_int as isize);
    *bbox.offset(0 as libc::c_int as isize) = *fresh22;
    let ref mut fresh23 = *bbox.offset(3 as libc::c_int as isize);
    *fresh23 = *x.offset(1 as libc::c_int as isize);
    *bbox.offset(2 as libc::c_int as isize) = *fresh23;
    i = 0 as libc::c_int;
    while i < n {
        *bbox
            .offset(
                0 as libc::c_int as isize,
            ) = if *bbox.offset(0 as libc::c_int as isize)
            < *x.offset((i * dim) as isize) - *width.offset((i * dim) as isize)
        {
            *bbox.offset(0 as libc::c_int as isize)
        } else {
            *x.offset((i * dim) as isize) - *width.offset((i * dim) as isize)
        };
        *bbox
            .offset(
                1 as libc::c_int as isize,
            ) = if *bbox.offset(1 as libc::c_int as isize)
            > *x.offset((i * dim) as isize) + *width.offset((i * dim) as isize)
        {
            *bbox.offset(1 as libc::c_int as isize)
        } else {
            *x.offset((i * dim) as isize) + *width.offset((i * dim) as isize)
        };
        *bbox
            .offset(
                2 as libc::c_int as isize,
            ) = if *bbox.offset(2 as libc::c_int as isize)
            < *x.offset((i * dim + 1 as libc::c_int) as isize)
                - *width.offset((i * dim + 1 as libc::c_int) as isize)
        {
            *bbox.offset(2 as libc::c_int as isize)
        } else {
            *x.offset((i * dim + 1 as libc::c_int) as isize)
                - *width.offset((i * dim + 1 as libc::c_int) as isize)
        };
        *bbox
            .offset(
                3 as libc::c_int as isize,
            ) = if *bbox.offset(3 as libc::c_int as isize)
            > *x.offset((i * dim + 1 as libc::c_int) as isize)
                + *width.offset((i * dim + 1 as libc::c_int) as isize)
        {
            *bbox.offset(3 as libc::c_int as isize)
        } else {
            *x.offset((i * dim + 1 as libc::c_int) as isize)
                + *width.offset((i * dim + 1 as libc::c_int) as isize)
        };
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_map_from_rectangle_groups(
    mut exclude_random: libc::c_int,
    mut include_OK_points: libc::c_int,
    mut n: libc::c_int,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut sizes: *mut libc::c_double,
    mut grouping: *mut libc::c_int,
    mut graph0: SparseMatrix,
    mut bounding_box_margin: *mut libc::c_double,
    mut nrandom: *mut libc::c_int,
    mut nart: *mut libc::c_int,
    mut nedgep: libc::c_int,
    mut shore_depth_tol: libc::c_double,
    mut edge_bridge_tol: libc::c_double,
    mut xcombined: *mut *mut libc::c_double,
    mut nverts: *mut libc::c_int,
    mut x_poly: *mut *mut libc::c_double,
    mut npolys: *mut libc::c_int,
    mut poly_lines: *mut SparseMatrix,
    mut polys: *mut SparseMatrix,
    mut polys_groups: *mut *mut libc::c_int,
    mut poly_point_map: *mut SparseMatrix,
    mut country_graph: *mut SparseMatrix,
    mut highlight_cluster: libc::c_int,
    mut flag: *mut libc::c_int,
) -> libc::c_int {
    let mut dist: libc::c_double = 0.;
    let mut avgdist: libc::c_double = 0.;
    let mut X: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut N: libc::c_int = 0;
    let mut nmax: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut igrp: libc::c_int = 0;
    let mut groups: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut K: libc::c_int = *nart;
    let mut avgsize: [libc::c_double; 2] = [0.; 2];
    let mut avgsz: libc::c_double = 0.;
    let mut h: [libc::c_double; 2] = [0.; 2];
    let mut p1: libc::c_double = 0.;
    let mut p0: libc::c_double = 0.;
    let mut point: [libc::c_double; 2] = [0.; 2];
    let mut nadded: [libc::c_int; 2] = [0; 2];
    let mut res: libc::c_int = 0;
    let mut delta: [libc::c_double; 2] = [0.; 2];
    let mut graph: SparseMatrix = graph0;
    let mut bbox: [libc::c_double; 4] = [0.; 4];
    if K < 0 as libc::c_int {
        K = (10 as libc::c_int as libc::c_double
            / (1 as libc::c_int as libc::c_double + n as libc::c_double / 400.0f64))
            as libc::c_int;
    }
    *nart = 0 as libc::c_int;
    if Verbose != 0 {
        let mut maxgp: libc::c_int = *grouping.offset(0 as libc::c_int as isize);
        let mut mingp: libc::c_int = *grouping.offset(0 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < n {
            maxgp = if maxgp > *grouping.offset(i as isize) {
                maxgp
            } else {
                *grouping.offset(i as isize)
            };
            mingp = if mingp < *grouping.offset(i as isize) {
                mingp
            } else {
                *grouping.offset(i as isize)
            };
            i += 1;
        }
        fprintf(
            stderr,
            b"max grouping - min grouping + 1 = %d\n\0" as *const u8
                as *const libc::c_char,
            maxgp - mingp + 1 as libc::c_int,
        );
    }
    if sizes.is_null() {
        return make_map_internal(
            exclude_random,
            include_OK_points,
            n,
            dim,
            x,
            grouping,
            graph,
            bounding_box_margin,
            nrandom,
            nedgep,
            shore_depth_tol,
            xcombined,
            nverts,
            x_poly,
            npolys,
            poly_lines,
            polys,
            polys_groups,
            poly_point_map,
            country_graph,
            highlight_cluster,
            flag,
        )
    } else {
        avgsize[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        avgsize[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < n {
            j = 0 as libc::c_int;
            while j < 2 as libc::c_int {
                avgsize[j as usize] += *sizes.offset((i * dim + j) as isize);
                j += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            avgsize[i as usize] /= n as libc::c_double;
            i += 1;
        }
        avgsz = 0.5f64
            * (avgsize[0 as libc::c_int as usize] + avgsize[1 as libc::c_int as usize]);
        if Verbose != 0 {
            fprintf(
                stderr,
                b"avgsize = {%f, %f}\n\0" as *const u8 as *const libc::c_char,
                avgsize[0 as libc::c_int as usize],
                avgsize[1 as libc::c_int as usize],
            );
        }
        nmax = 2 as libc::c_int * n;
        X = gv_calloc(
            (dim * (n + nmax)) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        groups = gv_calloc(
            (n + nmax) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < n {
            *groups.offset(i as isize) = *grouping.offset(i as isize);
            j = 0 as libc::c_int;
            while j < 2 as libc::c_int {
                *X
                    .offset(
                        (i * 2 as libc::c_int + j) as isize,
                    ) = *x.offset((i * dim + j) as isize);
                j += 1;
            }
            i += 1;
        }
        N = n;
        if shore_depth_tol < 0 as libc::c_int as libc::c_double {
            shore_depth_tol = -shore_depth_tol * avgsz;
        } else if shore_depth_tol == 0 as libc::c_int as libc::c_double {
            let mut area: libc::c_double = 0.;
            get_boundingbox(n, dim, x, sizes, bbox.as_mut_ptr());
            area = (bbox[1 as libc::c_int as usize] - bbox[0 as libc::c_int as usize])
                * (bbox[3 as libc::c_int as usize] - bbox[2 as libc::c_int as usize]);
            shore_depth_tol = sqrt(area / n as libc::c_double);
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"setting shore length ======%f\n\0" as *const u8
                        as *const libc::c_char,
                    shore_depth_tol,
                );
            }
        }
        if K > 0 as libc::c_int {
            delta[0 as libc::c_int
                as usize] = 0.5f64 * avgsize[0 as libc::c_int as usize]
                / K as libc::c_double;
            delta[1 as libc::c_int
                as usize] = 0.5f64 * avgsize[1 as libc::c_int as usize]
                / K as libc::c_double;
        } else {
            delta[1 as libc::c_int as usize] = 0.0f64;
            delta[0 as libc::c_int as usize] = delta[1 as libc::c_int as usize];
        }
        i = 0 as libc::c_int;
        while i < n {
            igrp = *grouping.offset(i as isize);
            j = 0 as libc::c_int;
            while j < 2 as libc::c_int {
                if avgsz == 0 as libc::c_int as libc::c_double {
                    nadded[j as usize] = 0 as libc::c_int;
                } else {
                    nadded[j
                        as usize] = (K as libc::c_double
                        * *sizes.offset((i * dim + j) as isize) / avgsz) as libc::c_int;
                }
                j += 1;
            }
            if nadded[0 as libc::c_int as usize] > 0 as libc::c_int {
                h[0 as libc::c_int
                    as usize] = *sizes.offset((i * dim) as isize)
                    / nadded[0 as libc::c_int as usize] as libc::c_double;
                point[0 as libc::c_int
                    as usize] = *x.offset((i * dim) as isize)
                    - *sizes.offset((i * dim) as isize)
                        / 2 as libc::c_int as libc::c_double;
                point[1 as libc::c_int
                    as usize] = *x.offset((i * dim + 1 as libc::c_int) as isize)
                    + *sizes.offset((i * dim + 1 as libc::c_int) as isize)
                        / 2 as libc::c_int as libc::c_double;
                p1 = point[1 as libc::c_int as usize];
                add_point(
                    &mut N,
                    igrp,
                    &mut X,
                    &mut nmax,
                    point.as_mut_ptr(),
                    &mut groups,
                );
                k = 0 as libc::c_int;
                while k < nadded[0 as libc::c_int as usize] - 1 as libc::c_int {
                    point[0 as libc::c_int as usize] += h[0 as libc::c_int as usize];
                    point[1 as libc::c_int
                        as usize] = p1
                        + (0.5f64 - drand()) * delta[1 as libc::c_int as usize];
                    add_point(
                        &mut N,
                        igrp,
                        &mut X,
                        &mut nmax,
                        point.as_mut_ptr(),
                        &mut groups,
                    );
                    k += 1;
                }
                point[0 as libc::c_int
                    as usize] = *x.offset((i * dim) as isize)
                    + *sizes.offset((i * dim) as isize)
                        / 2 as libc::c_int as libc::c_double;
                point[1 as libc::c_int
                    as usize] = *x.offset((i * dim + 1 as libc::c_int) as isize)
                    - *sizes.offset((i * dim + 1 as libc::c_int) as isize)
                        / 2 as libc::c_int as libc::c_double;
                p1 = point[1 as libc::c_int as usize];
                add_point(
                    &mut N,
                    igrp,
                    &mut X,
                    &mut nmax,
                    point.as_mut_ptr(),
                    &mut groups,
                );
                k = 0 as libc::c_int;
                while k < nadded[0 as libc::c_int as usize] - 1 as libc::c_int {
                    point[0 as libc::c_int as usize] -= h[0 as libc::c_int as usize];
                    point[1 as libc::c_int
                        as usize] = p1
                        + (0.5f64 - drand()) * delta[1 as libc::c_int as usize];
                    add_point(
                        &mut N,
                        igrp,
                        &mut X,
                        &mut nmax,
                        point.as_mut_ptr(),
                        &mut groups,
                    );
                    k += 1;
                }
            }
            if nadded[1 as libc::c_int as usize] > 0 as libc::c_int {
                h[1 as libc::c_int
                    as usize] = *sizes.offset((i * dim + 1 as libc::c_int) as isize)
                    / nadded[1 as libc::c_int as usize] as libc::c_double;
                point[0 as libc::c_int
                    as usize] = *x.offset((i * dim) as isize)
                    - *sizes.offset((i * dim) as isize)
                        / 2 as libc::c_int as libc::c_double;
                p0 = point[0 as libc::c_int as usize];
                point[1 as libc::c_int
                    as usize] = *x.offset((i * dim + 1 as libc::c_int) as isize)
                    - *sizes.offset((i * dim + 1 as libc::c_int) as isize)
                        / 2 as libc::c_int as libc::c_double;
                add_point(
                    &mut N,
                    igrp,
                    &mut X,
                    &mut nmax,
                    point.as_mut_ptr(),
                    &mut groups,
                );
                k = 0 as libc::c_int;
                while k < nadded[1 as libc::c_int as usize] - 1 as libc::c_int {
                    point[0 as libc::c_int
                        as usize] = p0
                        + (0.5f64 - drand()) * delta[0 as libc::c_int as usize];
                    point[1 as libc::c_int as usize] += h[1 as libc::c_int as usize];
                    add_point(
                        &mut N,
                        igrp,
                        &mut X,
                        &mut nmax,
                        point.as_mut_ptr(),
                        &mut groups,
                    );
                    k += 1;
                }
                point[0 as libc::c_int
                    as usize] = *x.offset((i * dim) as isize)
                    + *sizes.offset((i * dim) as isize)
                        / 2 as libc::c_int as libc::c_double;
                p0 = point[0 as libc::c_int as usize];
                point[1 as libc::c_int
                    as usize] = *x.offset((i * dim + 1 as libc::c_int) as isize)
                    + *sizes.offset((i * dim + 1 as libc::c_int) as isize)
                        / 2 as libc::c_int as libc::c_double;
                add_point(
                    &mut N,
                    igrp,
                    &mut X,
                    &mut nmax,
                    point.as_mut_ptr(),
                    &mut groups,
                );
                k = 0 as libc::c_int;
                while k < nadded[1 as libc::c_int as usize] - 1 as libc::c_int {
                    point[0 as libc::c_int
                        as usize] = p0
                        + (0.5f64 - drand()) * delta[0 as libc::c_int as usize];
                    point[1 as libc::c_int as usize] -= h[1 as libc::c_int as usize];
                    add_point(
                        &mut N,
                        igrp,
                        &mut X,
                        &mut nmax,
                        point.as_mut_ptr(),
                        &mut groups,
                    );
                    k += 1;
                }
            }
            *nart = N - n;
            i += 1;
        }
        if !graph.is_null() && edge_bridge_tol != 0 as libc::c_int as libc::c_double {
            let mut ia: *mut libc::c_int = (*graph).ia;
            let mut ja: *mut libc::c_int = (*graph).ja;
            let mut nz: libc::c_int = 0 as libc::c_int;
            let mut jj: libc::c_int = 0;
            let mut KB: libc::c_int = 0;
            graph = SparseMatrix_symmetrize(graph, 1 as libc::c_int != 0);
            ia = (*graph).ia;
            ja = (*graph).ja;
            avgdist = 0.0f64;
            dist = avgdist;
            i = 0 as libc::c_int;
            while i < n {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    jj = *ja.offset(j as isize);
                    if !(jj <= i) {
                        dist = distance(x, dim, i, jj);
                        avgdist += dist;
                        nz += 1;
                    }
                    j += 1;
                }
                i += 1;
            }
            avgdist /= nz as libc::c_double;
            if edge_bridge_tol < 0 as libc::c_int as libc::c_double {
                KB = -edge_bridge_tol as libc::c_int;
            } else {
                KB = (avgdist / edge_bridge_tol) as libc::c_int;
            }
            if avgdist > 0 as libc::c_int as libc::c_double {} else {
                __assert_fail(
                    b"avgdist > 0\0" as *const u8 as *const libc::c_char,
                    b"make_map.c\0" as *const u8 as *const libc::c_char,
                    1547 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 261],
                        &[libc::c_char; 261],
                    >(
                        b"int make_map_from_rectangle_groups(int, int, int, int, double *, double *, int *, SparseMatrix, double *, int *, int *, int, double, double, double **, int *, double **, int *, SparseMatrix *, SparseMatrix *, int **, SparseMatrix *, SparseMatrix *, int, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            i = 0 as libc::c_int;
            while i < n {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    jj = *ja.offset(j as isize);
                    if !(jj <= i) {
                        dist = distance(x, dim, i, jj);
                        nadded[0 as libc::c_int
                            as usize] = ((2 as libc::c_int * KB) as libc::c_double * dist
                            / avgdist) as libc::c_int;
                        h[0 as libc::c_int
                            as usize] = 0.5f64
                            * (*x.offset((jj * dim) as isize)
                                - *x.offset((i * dim) as isize))
                            / nadded[0 as libc::c_int as usize] as libc::c_double;
                        h[1 as libc::c_int
                            as usize] = 0.5f64
                            * (*x.offset((jj * dim + 1 as libc::c_int) as isize)
                                - *x.offset((i * dim + 1 as libc::c_int) as isize))
                            / nadded[0 as libc::c_int as usize] as libc::c_double;
                        point[0 as libc::c_int as usize] = *x.offset((i * dim) as isize);
                        point[1 as libc::c_int
                            as usize] = *x.offset((i * dim + 1 as libc::c_int) as isize);
                        k = 0 as libc::c_int;
                        while k < nadded[0 as libc::c_int as usize] - 1 as libc::c_int {
                            point[0 as libc::c_int as usize]
                                += h[0 as libc::c_int as usize];
                            point[1 as libc::c_int as usize]
                                += h[1 as libc::c_int as usize];
                            add_point(
                                &mut N,
                                *grouping.offset(i as isize),
                                &mut X,
                                &mut nmax,
                                point.as_mut_ptr(),
                                &mut groups,
                            );
                            k += 1;
                        }
                        h[0 as libc::c_int
                            as usize] = 0.5f64
                            * (*x.offset((i * dim) as isize)
                                - *x.offset((jj * dim) as isize))
                            / nadded[0 as libc::c_int as usize] as libc::c_double;
                        h[1 as libc::c_int
                            as usize] = 0.5f64
                            * (*x.offset((i * dim + 1 as libc::c_int) as isize)
                                - *x.offset((jj * dim + 1 as libc::c_int) as isize))
                            / nadded[0 as libc::c_int as usize] as libc::c_double;
                        point[0 as libc::c_int
                            as usize] = *x.offset((jj * dim) as isize);
                        point[1 as libc::c_int
                            as usize] = *x
                            .offset((jj * dim + 1 as libc::c_int) as isize);
                        k = 0 as libc::c_int;
                        while k < nadded[0 as libc::c_int as usize] - 1 as libc::c_int {
                            point[0 as libc::c_int as usize]
                                += h[0 as libc::c_int as usize];
                            point[1 as libc::c_int as usize]
                                += h[1 as libc::c_int as usize];
                            add_point(
                                &mut N,
                                *grouping.offset(jj as isize),
                                &mut X,
                                &mut nmax,
                                point.as_mut_ptr(),
                                &mut groups,
                            );
                            k += 1;
                        }
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        res = make_map_internal(
            exclude_random,
            include_OK_points,
            N,
            dim,
            X,
            groups,
            graph,
            bounding_box_margin,
            nrandom,
            nedgep,
            shore_depth_tol,
            xcombined,
            nverts,
            x_poly,
            npolys,
            poly_lines,
            polys,
            polys_groups,
            poly_point_map,
            country_graph,
            highlight_cluster,
            flag,
        );
        if graph != graph0 {
            SparseMatrix_delete(graph);
        }
        free(groups as *mut libc::c_void);
        free(X as *mut libc::c_void);
    }
    return res;
}
