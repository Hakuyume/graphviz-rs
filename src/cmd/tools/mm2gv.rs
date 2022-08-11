#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
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
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut Agdirected: Agdesc_t;
    static mut Agundirected: Agdesc_t;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn SparseMatrix_delete(A: SparseMatrix);
    fn SparseMatrix_make_undirected(A: SparseMatrix) -> SparseMatrix;
    fn SparseMatrix_to_square_matrix(
        A: SparseMatrix,
        bipartite_options: libc::c_int,
    ) -> SparseMatrix;
    fn distance(
        x: *mut libc::c_double,
        dim: libc::c_int,
        i: libc::c_int,
        j: libc::c_int,
    ) -> libc::c_double;
    fn strip_dir(s: *mut libc::c_char) -> *mut libc::c_char;
    fn SparseMatrix_import_matrix_market(f: *mut FILE) -> SparseMatrix;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
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
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MATRIX_UNDIRECTED: C2RustUnnamed_2 = 16;
pub const MATRIX_HERMITIAN: C2RustUnnamed_2 = 8;
pub const MATRIX_SKEW: C2RustUnnamed_2 = 4;
pub const MATRIX_SYMMETRIC: C2RustUnnamed_2 = 2;
pub const MATRIX_PATTERN_SYMMETRIC: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const BIPARTITE_ALWAYS: C2RustUnnamed_3 = 3;
pub const BIPARTITE_UNSYM: C2RustUnnamed_3 = 2;
pub const BIPARTITE_PATTERN_UNSYM: C2RustUnnamed_3 = 1;
pub const BIPARTITE_RECT: C2RustUnnamed_3 = 0;
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_4 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_4 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_4 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_4 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnodeinfo_t {
    pub h: Agrec_t,
    pub id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parms_t {
    pub inf: *mut FILE,
    pub outf: *mut FILE,
    pub infile: *mut libc::c_char,
    pub undirected: libc::c_int,
    pub with_label: libc::c_int,
    pub with_color: libc::c_int,
    pub with_val: libc::c_int,
    pub bipartite: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init_0: *mut libc::c_char,
) {
    if !init_0.is_null() {
        let ref mut fresh0 = (*xb).buf;
        *fresh0 = init_0;
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
static mut cmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn Hue2RGB(
    mut v1: libc::c_double,
    mut v2: libc::c_double,
    mut H: libc::c_double,
) -> libc::c_double {
    if H < 0.0f64 {
        H += 1.0f64;
    }
    if H > 1.0f64 {
        H -= 1.0f64;
    }
    if 6.0f64 * H < 1.0f64 {
        return v1 + (v2 - v1) * 6.0f64 * H;
    }
    if 2.0f64 * H < 1.0f64 {
        return v2;
    }
    if 3.0f64 * H < 2.0f64 {
        return v1 + (v2 - v1) * (2.0f64 / 3.0f64 - H) * 6.0f64;
    }
    return v1;
}
unsafe extern "C" fn hue2rgb(
    mut hue: libc::c_double,
    mut color: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut v1: libc::c_double = 0.;
    let mut v2: libc::c_double = 0.;
    let mut lightness: libc::c_double = 0.5f64;
    let mut saturation: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut red: libc::c_int = 0;
    let mut blue: libc::c_int = 0;
    let mut green: libc::c_int = 0;
    if lightness < 0.5f64 {
        v2 = lightness * (1.0f64 + saturation);
    } else {
        v2 = lightness + saturation - saturation * lightness;
    }
    v1 = 2.0f64 * lightness - v2;
    red = (255.0f64 * Hue2RGB(v1, v2, hue + 1.0f64 / 3.0f64) + 0.5f64) as libc::c_int;
    green = (255.0f64 * Hue2RGB(v1, v2, hue) + 0.5f64) as libc::c_int;
    blue = (255.0f64 * Hue2RGB(v1, v2, hue - 1.0f64 / 3.0f64) + 0.5f64) as libc::c_int;
    sprintf(
        color,
        b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        red,
        green,
        blue,
    );
    return color;
}
unsafe extern "C" fn makeDotGraph(
    mut A: SparseMatrix,
    mut name: *mut libc::c_char,
    mut dim: libc::c_int,
    mut x: *mut libc::c_double,
    mut with_color: libc::c_int,
    mut with_label: libc::c_int,
    mut with_val: libc::c_int,
) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut sym2: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut sym3: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut val: *mut libc::c_double = (*A).a as *mut libc::c_double;
    let mut arr: *mut *mut Agnode_t = calloc(
        (*A).m as libc::c_ulong,
        ::std::mem::size_of::<*mut Agnode_t>() as libc::c_ulong,
    ) as *mut *mut Agnode_t;
    let mut color: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut cstring: [libc::c_char; 8] = [0; 8];
    name = strip_dir(name);
    if with_color != 0 {
        if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int && val.is_null() {
            fprintf(
                stderr,
                b"Warning: input matrix has no values, -c flag ignored\n\0" as *const u8
                    as *const libc::c_char,
            );
            with_color = 0 as libc::c_int;
        } else if (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int && x.is_null() {
            fprintf(
                stderr,
                b"Warning: input has no coordinates, -c flag ignored\n\0" as *const u8
                    as *const libc::c_char,
            );
            with_color = 0 as libc::c_int;
        }
    }
    if (*A).property & MATRIX_UNDIRECTED as libc::c_int != 0 {
        g = agopen(
            b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Agundirected,
            0 as *mut Agdisc_t,
        );
    } else {
        g = agopen(
            b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Agdirected,
            0 as *mut Agdisc_t,
        );
    }
    if with_val != 0 {
        sym = agattr(
            g,
            2 as libc::c_int,
            b"len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
        );
    }
    agxbinit(&mut xb, 1024 as libc::c_int as libc::c_uint, string.as_mut_ptr());
    if with_label != 0 {
        agxbprint(
            &mut xb as *mut agxbuf,
            b"%s. %d nodes, %d edges.\0" as *const u8 as *const libc::c_char,
            name,
            (*A).m,
            (*A).nz,
        );
        agattr(
            g,
            0 as libc::c_int,
            b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(&mut xb),
        );
    }
    i = 0 as libc::c_int;
    while i < (*A).m {
        sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, i);
        n = agnode(g, buf.as_mut_ptr(), 1 as libc::c_int);
        agbindrec(
            n as *mut libc::c_void,
            b"nodeinfo\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
            1 as libc::c_int,
        );
        (*((*n).base.data as *mut Agnodeinfo_t)).id = i;
        let ref mut fresh11 = *arr.offset(i as isize);
        *fresh11 = n;
        i += 1;
    }
    if with_color != 0 {
        let mut maxdist: libc::c_double = 0.0f64;
        let mut mindist: libc::c_double = 0.0f64;
        let mut first: libc::c_int = 1 as libc::c_int;
        sym2 = agattr(
            g,
            2 as libc::c_int,
            b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        sym3 = agattr(
            g,
            2 as libc::c_int,
            b"wt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        agattr(
            g,
            0 as libc::c_int,
            b"bgcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"black\0" as *const u8 as *const libc::c_char,
        );
        color = calloc(
            (*A).nz as libc::c_ulong,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        n = agfstnode(g);
        while !n.is_null() {
            i = (*((*n).base.data as *mut Agnodeinfo_t)).id;
            if (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    *color
                        .offset(
                            j as isize,
                        ) = distance(x, dim, i, *ja.offset(j as isize));
                    if i != *ja.offset(j as isize) {
                        if first != 0 {
                            mindist = *color.offset(j as isize);
                            first = 0 as libc::c_int;
                        } else {
                            mindist = if mindist < *color.offset(j as isize) {
                                mindist
                            } else {
                                *color.offset(j as isize)
                            };
                        }
                    }
                    maxdist = if *color.offset(j as isize) > maxdist {
                        *color.offset(j as isize)
                    } else {
                        maxdist
                    };
                    j += 1;
                }
            } else {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if !val.is_null() {
                        *color.offset(j as isize) = fabs(*val.offset(j as isize));
                    } else {
                        *color.offset(j as isize) = 1 as libc::c_int as libc::c_double;
                    }
                    if i != *ja.offset(j as isize) {
                        if first != 0 {
                            mindist = *color.offset(j as isize);
                            first = 0 as libc::c_int;
                        } else {
                            mindist = if mindist < *color.offset(j as isize) {
                                mindist
                            } else {
                                *color.offset(j as isize)
                            };
                        }
                    }
                    maxdist = if *color.offset(j as isize) > maxdist {
                        *color.offset(j as isize)
                    } else {
                        maxdist
                    };
                    j += 1;
                }
            }
            n = agnxtnode(g, n);
        }
        n = agfstnode(g);
        while !n.is_null() {
            i = (*((*n).base.data as *mut Agnodeinfo_t)).id;
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                *color
                    .offset(
                        j as isize,
                    ) = (*color.offset(j as isize) - mindist)
                    / (if maxdist - mindist > 0.000001f64 {
                        maxdist - mindist
                    } else {
                        0.000001f64
                    });
                j += 1;
            }
            n = agnxtnode(g, n);
        }
    }
    n = agfstnode(g);
    while !n.is_null() {
        i = (*((*n).base.data as *mut Agnodeinfo_t)).id;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            h = *arr.offset(*ja.offset(j as isize) as isize);
            e = agedge(g, n, h, 0 as *mut libc::c_char, 1 as libc::c_int);
            if !sym.is_null() && !val.is_null() {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%f\0" as *const u8 as *const libc::c_char,
                    *val.offset(j as isize),
                );
                agxset(e as *mut libc::c_void, sym, buf.as_mut_ptr());
            }
            if with_color != 0 {
                agxset(
                    e as *mut libc::c_void,
                    sym2,
                    hue2rgb(0.65f64 * *color.offset(j as isize), cstring.as_mut_ptr()),
                );
                sprintf(
                    buf.as_mut_ptr(),
                    b"%f\0" as *const u8 as *const libc::c_char,
                    *color.offset(j as isize),
                );
                agxset(e as *mut libc::c_void, sym3, buf.as_mut_ptr());
            }
            j += 1;
        }
        n = agnxtnode(g, n);
    }
    agxbfree(&mut xb);
    free(color as *mut libc::c_void);
    free(arr as *mut libc::c_void);
    return g;
}
static mut useString: *mut libc::c_char = b"Usage: %s [-uvcl] [-o file] matrix_market_filename\n  -u   - make graph undirected\n  -U i - treat non-square matrix as a bipartite graph\n         i = 0   never\n         i = 1   if pattern unsymmetric (default)\n         i = 2   if matrix unsymmetric\n         i = 3   always\n  -v   - assign len to edges\n  -c   - assign color and wt to edges\n  -l   - add label\n  -o <file> - output file \n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut eval: libc::c_int) {
    fprintf(stderr, useString, cmd);
    graphviz_exit(eval);
}
unsafe extern "C" fn openF(
    mut fname: *mut libc::c_char,
    mut mode: *mut libc::c_char,
) -> *mut FILE {
    let mut f: *mut FILE = fopen(fname, mode);
    if f.is_null() {
        fprintf(
            stderr,
            b"Could not open %s for %s\n\0" as *const u8 as *const libc::c_char,
            fname,
            if *mode as libc::c_int == 'r' as i32 {
                b"reading\0" as *const u8 as *const libc::c_char
            } else {
                b"writing\0" as *const u8 as *const libc::c_char
            },
        );
        graphviz_exit(1 as libc::c_int);
    }
    return f;
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut p: *mut parms_t,
) {
    let mut c: libc::c_int = 0;
    cmd = *argv.offset(0 as libc::c_int as isize);
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(argc, argv, b":o:uvclU:?\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            111 => {
                let ref mut fresh12 = (*p).outf;
                *fresh12 = openF(
                    optarg,
                    b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            108 => {
                (*p).with_label = 1 as libc::c_int;
            }
            117 => {
                (*p).undirected = 1 as libc::c_int;
            }
            118 => {
                (*p).with_val = 1 as libc::c_int;
            }
            99 => {
                (*p).with_color = 1 as libc::c_int;
            }
            85 => {
                let mut u: libc::c_int = 0;
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut u as *mut libc::c_int,
                ) <= 0 as libc::c_int || u < 0 as libc::c_int
                    || u > BIPARTITE_ALWAYS as libc::c_int
                {
                    usage(1 as libc::c_int);
                } else {
                    (*p).bipartite = u;
                }
            }
            58 => {
                fprintf(
                    stderr,
                    b"%s: option -%c missing argument - ignored\n\0" as *const u8
                        as *const libc::c_char,
                    cmd,
                    optopt,
                );
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    usage(0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b"%s: option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        cmd,
                        optopt,
                    );
                    usage(1 as libc::c_int);
                }
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"mm2gv.c\0" as *const u8 as *const libc::c_char,
                    290 as libc::c_int,
                );
                abort();
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc > 0 as libc::c_int {
        let ref mut fresh13 = (*p).infile;
        *fresh13 = *argv.offset(0 as libc::c_int as isize);
        let ref mut fresh14 = (*p).inf;
        *fresh14 = openF(
            *argv.offset(0 as libc::c_int as isize),
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut dim: libc::c_int = 0 as libc::c_int;
    let mut pv: parms_t = parms_t {
        inf: 0 as *mut FILE,
        outf: 0 as *mut FILE,
        infile: 0 as *mut libc::c_char,
        undirected: 0,
        with_label: 0,
        with_color: 0,
        with_val: 0,
        bipartite: 0,
    };
    pv.inf = stdin;
    pv.outf = stdout;
    pv.infile = b"stdin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pv.undirected = 0 as libc::c_int;
    pv.with_label = 0 as libc::c_int;
    pv.with_color = 0 as libc::c_int;
    pv.with_val = 0 as libc::c_int;
    pv.bipartite = BIPARTITE_PATTERN_UNSYM as libc::c_int;
    init(argc, argv, &mut pv);
    A = SparseMatrix_import_matrix_market(pv.inf);
    if A.is_null() {
        fprintf(
            stderr,
            b"Unable to read input file \"%s\"\n\0" as *const u8 as *const libc::c_char,
            pv.infile,
        );
        usage(1 as libc::c_int);
    }
    A = SparseMatrix_to_square_matrix(A, pv.bipartite);
    if A.is_null() {
        fprintf(
            stderr,
            b"cannot import from file %s\n\0" as *const u8 as *const libc::c_char,
            pv.infile,
        );
        graphviz_exit(1 as libc::c_int);
    }
    if pv.undirected != 0 {
        let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
        B = SparseMatrix_make_undirected(A);
        SparseMatrix_delete(A);
        A = B;
    }
    g = makeDotGraph(
        A,
        pv.infile,
        dim,
        0 as *mut libc::c_double,
        pv.with_color,
        pv.with_label,
        pv.with_val,
    );
    agwrite(g, pv.outf as *mut libc::c_void);
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
