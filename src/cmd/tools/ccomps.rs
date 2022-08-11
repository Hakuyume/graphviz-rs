#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agsubnode(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
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
    fn agsubedge(
        g: *mut Agraph_t,
        e: *mut Agedge_t,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agcopyattr(oldobj: *mut libc::c_void, newobj: *mut libc::c_void) -> libc::c_int;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    static mut Agstrictundirected: Agdesc_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
}
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
pub type size_t = libc::c_ulong;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub hh: C2RustUnnamed_1,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
    pub graph: C2RustUnnamed_2,
    pub node: C2RustUnnamed_2,
    pub edge: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub const FIRST_ALLOCATION: C2RustUnnamed_3 = 512;
pub type C2RustUnnamed_3 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agraphinfo_t {
    pub h: Agrec_t,
    pub cc_subg: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnodeinfo_t {
    pub h: Agrec_t,
    pub mark: libc::c_char,
    pub ptr: *mut Agobj_t,
}
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
pub type qsort_cmpf = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
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
unsafe extern "C" fn gv_alloc(mut size: size_t) -> *mut libc::c_void {
    return gv_calloc(1 as libc::c_int as size_t, size);
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
unsafe extern "C" fn gv_strndup(
    mut original: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    copy = strndup(original, length);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn stack_size(mut stack: *const gv_stack_t) -> size_t {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"size_t stack_size(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return (*stack).size;
}
#[inline]
unsafe extern "C" fn stack_push_or_exit(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void stack_push_or_exit(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    let mut r: libc::c_int = stack_push(stack, item);
    if (r != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"stack_push failed: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(r),
        );
        graphviz_exit(1 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn stack_top(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    if !stack_is_empty(stack)
        && !(b"access to top of an empty stack\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"!stack_is_empty(stack) && \"access to top of an empty stack\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void *stack_top(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return *((*stack).base)
        .offset(
            ((*stack).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
}
#[inline]
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh0 = (*stack).size;
    *fresh0 = (*fresh0).wrapping_sub(1);
    return top;
}
#[inline]
unsafe extern "C" fn stack_reset(mut stack: *mut gv_stack_t) {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void stack_reset(gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    free((*stack).base as *mut libc::c_void);
    memset(
        stack as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gv_stack_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn stack_is_empty(mut stack: *const gv_stack_t) -> bool {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"_Bool stack_is_empty(const gv_stack_t *)\0"))
                .as_ptr(),
        );
    }
    return stack_size(stack) == 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn stack_push(
    mut stack: *mut gv_stack_t,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if !stack.is_null() {} else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).size == (*stack).capacity {
        if ((18446744073709551615 as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) < (*stack).capacity)
            as libc::c_int as libc::c_long != 0
        {
            return 75 as libc::c_int;
        }
        let mut c: size_t = if (*stack).capacity == 0 as libc::c_int as libc::c_ulong {
            FIRST_ALLOCATION as libc::c_int as libc::c_ulong
        } else {
            (2 as libc::c_int as libc::c_ulong).wrapping_mul((*stack).capacity)
        };
        let mut b: *mut *mut libc::c_void = realloc(
            (*stack).base as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong).wrapping_mul(c),
        ) as *mut *mut libc::c_void;
        if (b == 0 as *mut libc::c_void as *mut *mut libc::c_void) as libc::c_int
            as libc::c_long != 0
        {
            return 12 as libc::c_int;
        }
        (*stack).capacity = c;
        let ref mut fresh1 = (*stack).base;
        *fresh1 = b;
    }
    if !((*stack).base).is_null() {} else {
        __assert_fail(
            b"stack->base != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    if (*stack).capacity > (*stack).size {} else {
        __assert_fail(
            b"stack->capacity > stack->size\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int stack_push(gv_stack_t *, void *)\0"))
                .as_ptr(),
        );
    }
    let ref mut fresh2 = *((*stack).base).offset((*stack).size as isize);
    *fresh2 = item;
    let ref mut fresh3 = (*stack).size;
    *fresh3 = (*fresh3).wrapping_add(1);
    return 0 as libc::c_int;
}
static mut Cmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut verbose: libc::c_int = 0;
static mut printMode: libc::c_int = 0 as libc::c_int;
static mut useClusters: libc::c_int = 0 as libc::c_int;
static mut doEdges: libc::c_int = 1 as libc::c_int;
static mut doAll: libc::c_int = 1 as libc::c_int;
static mut suffix: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut outfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut path: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut sufcnt: libc::c_int = 0 as libc::c_int;
static mut sorted: libc::c_int = 0 as libc::c_int;
static mut sortIndex: libc::c_int = 0 as libc::c_int;
static mut sortFinal: libc::c_int = 0;
static mut x_index: libc::c_int = -(1 as libc::c_int);
static mut x_final: libc::c_int = -(1 as libc::c_int);
static mut x_mode: libc::c_int = 0;
static mut x_node: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut useString: *mut libc::c_char = b"Usage: ccomps [-svenCx?] [-X[#%]s[-f]] [-o<out template>] <files>\n  -s - silent\n  -x - external\n  -X - extract component\n  -C - use clusters\n  -e - do not induce edges\n  -n - do not induce subgraphs\n  -v - verbose\n  -o - output file template\n  -z - sort by size, largest first\n  -? - print usage\nIf no files are specified, stdin is used\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    printf(b"%s\0" as *const u8 as *const libc::c_char, useString);
    graphviz_exit(v);
}
unsafe extern "C" fn split() {
    let mut sfx: *mut libc::c_char = 0 as *mut libc::c_char;
    sfx = strrchr(outfile, '.' as i32);
    if !sfx.is_null() {
        suffix = sfx.offset(1 as libc::c_int as isize);
        let mut size: size_t = sfx.offset_from(outfile) as libc::c_long as size_t;
        path = gv_strndup(outfile, size);
    } else {
        path = outfile;
    };
}
unsafe extern "C" fn isCluster(mut g: *mut Agraph_t) -> libc::c_int {
    return (strncmp(
        agnameof(g as *mut libc::c_void),
        b"cluster\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn init(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    Cmd = *argv.offset(0 as libc::c_int as isize);
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b":zo:xCX:nesv?\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            111 => {
                outfile = optarg;
                split();
            }
            67 => {
                useClusters = 1 as libc::c_int;
            }
            101 => {
                doEdges = 0 as libc::c_int;
            }
            110 => {
                doAll = 0 as libc::c_int;
            }
            120 => {
                printMode = 1 as libc::c_int;
            }
            115 => {
                printMode = 2 as libc::c_int;
            }
            88 => {
                if *optarg as libc::c_int == '#' as i32
                    || *optarg as libc::c_int == '%' as i32
                {
                    let mut p: *mut libc::c_char = optarg
                        .offset(1 as libc::c_int as isize);
                    if *optarg as libc::c_int == '#' as i32 {
                        x_mode = 1 as libc::c_int;
                    } else {
                        x_mode = 2 as libc::c_int;
                    }
                    if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        x_index = strtol(p, &mut endp, 10 as libc::c_int) as libc::c_int;
                        printMode = 3 as libc::c_int;
                        if *endp as libc::c_int == '-' as i32 {
                            p = endp.offset(1 as libc::c_int as isize);
                            if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                x_final = atoi(p);
                                if x_final < x_index {
                                    printMode = 0 as libc::c_int;
                                    fprintf(
                                        stderr,
                                        b"ccomps: final index %d < start index %d in -X%s flag - ignored\n\0"
                                            as *const u8 as *const libc::c_char,
                                        x_final,
                                        x_index,
                                        optarg,
                                    );
                                }
                            } else if *p != 0 {
                                printMode = 0 as libc::c_int;
                                fprintf(
                                    stderr,
                                    b"ccomps: number expected in -X%s flag - ignored\n\0"
                                        as *const u8 as *const libc::c_char,
                                    optarg,
                                );
                            }
                        } else {
                            x_final = x_index;
                        }
                    } else {
                        fprintf(
                            stderr,
                            b"ccomps: number expected in -X%s flag - ignored\n\0"
                                as *const u8 as *const libc::c_char,
                            optarg,
                        );
                    }
                } else {
                    x_node = optarg;
                    printMode = 3 as libc::c_int;
                }
            }
            118 => {
                verbose = 1 as libc::c_int;
            }
            122 => {
                sorted = 1 as libc::c_int;
            }
            58 => {
                fprintf(
                    stderr,
                    b"ccomps: option -%c missing argument - ignored\n\0" as *const u8
                        as *const libc::c_char,
                    optopt,
                );
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    usage(0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b"ccomps: option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
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
                    b"ccomps.c\0" as *const u8 as *const libc::c_char,
                    215 as libc::c_int,
                );
                abort();
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if sorted != 0 {
        if printMode == 3 as libc::c_int && x_index >= 0 as libc::c_int {
            printMode = 0 as libc::c_int;
            sortIndex = x_index;
            sortFinal = x_final;
        } else if printMode == 1 as libc::c_int {
            sortIndex = -(1 as libc::c_int);
            printMode = 0 as libc::c_int;
        } else {
            sorted = 0 as libc::c_int;
        }
    }
    if argc > 0 as libc::c_int {
        Files = argv;
    }
}
static mut Stk: gv_stack_t = gv_stack_t {
    base: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    size: 0,
    capacity: 0,
};
unsafe extern "C" fn push(mut np: *mut Agnode_t) {
    (*((*np).base.data as *mut Agnodeinfo_t)).mark = -(1 as libc::c_int) as libc::c_char;
    stack_push_or_exit(&mut Stk, np as *mut libc::c_void);
}
unsafe extern "C" fn pop() -> *mut Agnode_t {
    if stack_is_empty(&mut Stk) {
        return 0 as *mut Agnode_t;
    }
    return stack_pop(&mut Stk) as *mut Agnode_t;
}
unsafe extern "C" fn dfs(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut out: *mut Agraph_t,
) -> libc::c_int {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut other: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    push(n);
    loop {
        n = pop();
        if n.is_null() {
            break;
        }
        (*((*n).base.data as *mut Agnodeinfo_t)).mark = 1 as libc::c_int as libc::c_char;
        cnt += 1;
        agsubnode(out, n, 1 as libc::c_int);
        e = agfstedge(g, n);
        while !e.is_null() {
            other = (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                e
            } else {
                e.offset(1 as libc::c_int as isize)
            }))
                .node;
            if other == n {
                other = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
            }
            if (*((*other).base.data as *mut Agnodeinfo_t)).mark as libc::c_int
                == 0 as libc::c_int
            {
                push(other);
            }
            e = agnxtedge(g, e, n);
        }
    }
    return cnt;
}
unsafe extern "C" fn nodeInduce(
    mut g: *mut Agraph_t,
    mut eg: *mut Agraph_t,
) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut e_cnt: libc::c_int = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(eg, n);
        while !e.is_null() {
            if !(agsubnode(
                g,
                (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node,
                0 as libc::c_int,
            ))
                .is_null()
            {
                agsubedge(g, e, 1 as libc::c_int);
                e_cnt += 1;
            }
            e = agnxtout(eg, e);
        }
        n = agnxtnode(g, n);
    }
    return e_cnt;
}
unsafe extern "C" fn getName() -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if sufcnt == 0 as libc::c_int {
        name = outfile;
    } else {
        if buf.is_null() {
            buf = gv_alloc(
                (strlen(outfile)).wrapping_add(20 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
        }
        if !suffix.is_null() {
            sprintf(
                buf,
                b"%s_%d.%s\0" as *const u8 as *const libc::c_char,
                path,
                sufcnt,
                suffix,
            );
        } else {
            sprintf(buf, b"%s_%d\0" as *const u8 as *const libc::c_char, path, sufcnt);
        }
        name = buf;
    }
    sufcnt += 1;
    return name;
}
unsafe extern "C" fn gwrite(mut g: *mut Agraph_t) {
    let mut outf: *mut FILE = 0 as *mut FILE;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if outfile.is_null() {
        agwrite(g, stdout as *mut libc::c_void);
        fflush(stdout);
    } else {
        name = getName();
        outf = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
        if outf.is_null() {
            fprintf(
                stderr,
                b"Could not open %s for writing\n\0" as *const u8 as *const libc::c_char,
                name,
            );
            perror(b"ccomps\0" as *const u8 as *const libc::c_char);
        }
        agwrite(g, outf as *mut libc::c_void);
        fclose(outf);
    };
}
unsafe extern "C" fn projectG(
    mut subg: *mut Agraph_t,
    mut g: *mut Agraph_t,
    mut inCluster: libc::c_int,
) -> *mut Agraph_t {
    let mut proj: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut m: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfstnode(subg);
    while !n.is_null() {
        m = agnode(g, agnameof(n as *mut libc::c_void), 0 as libc::c_int);
        if !m.is_null() {
            if proj.is_null() {
                proj = agsubg(g, agnameof(subg as *mut libc::c_void), 1 as libc::c_int);
            }
            agsubnode(proj, m, 1 as libc::c_int);
        }
        n = agnxtnode(subg, n);
    }
    if proj.is_null() && inCluster != 0 {
        proj = agsubg(g, agnameof(subg as *mut libc::c_void), 1 as libc::c_int);
    }
    if !proj.is_null() {
        if doEdges != 0 {
            nodeInduce(proj, subg);
        }
        agcopyattr(subg as *mut libc::c_void, proj as *mut libc::c_void);
    }
    return proj;
}
unsafe extern "C" fn subgInduce(
    mut root: *mut Agraph_t,
    mut g: *mut Agraph_t,
    mut inCluster: libc::c_int,
) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut proj: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut in_cluster: libc::c_int = 0;
    subg = agfstsubg(root);
    while !subg.is_null() {
        if !((*((*subg).base.data as *mut Agraphinfo_t)).cc_subg != 0) {
            proj = projectG(subg, g, inCluster);
            if !proj.is_null() {
                in_cluster = (inCluster != 0 || useClusters != 0 && isCluster(subg) != 0)
                    as libc::c_int;
                subgInduce(subg, proj, in_cluster);
            }
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn subGInduce(mut g: *mut Agraph_t, mut out: *mut Agraph_t) {
    subgInduce(g, out, 0 as libc::c_int);
}
unsafe extern "C" fn deriveClusters(mut dg: *mut Agraph_t, mut g: *mut Agraph_t) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        if strncmp(
            agnameof(subg as *mut libc::c_void),
            b"cluster\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            dn = agnode(dg, agnameof(subg as *mut libc::c_void), 1 as libc::c_int);
            agbindrec(
                dn as *mut libc::c_void,
                b"nodeinfo\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            let ref mut fresh4 = (*((*dn).base.data as *mut Agnodeinfo_t)).ptr;
            *fresh4 = subg as *mut Agobj_t;
            n = agfstnode(subg);
            while !n.is_null() {
                if !((*((*n).base.data as *mut Agnodeinfo_t)).ptr).is_null() {
                    fprintf(
                        stderr,
                        b"Error: node \"%s\" belongs to two non-nested clusters \"%s\" and \"%s\"\n\0"
                            as *const u8 as *const libc::c_char,
                        agnameof(n as *mut libc::c_void),
                        agnameof(subg as *mut libc::c_void),
                        agnameof(
                            (*((*n).base.data as *mut Agnodeinfo_t)).ptr as *mut Agnode_t
                                as *mut libc::c_void,
                        ),
                    );
                }
                let ref mut fresh5 = (*((*n).base.data as *mut Agnodeinfo_t)).ptr;
                *fresh5 = dn as *mut Agobj_t;
                n = agnxtnode(subg, n);
            }
        } else {
            deriveClusters(dg, subg);
        }
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn deriveGraph(mut g: *mut Agraph_t) -> *mut Agraph_t {
    let mut dg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    dg = agopen(
        b"dg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Agstrictundirected,
        0 as *mut Agdisc_t,
    );
    deriveClusters(dg, g);
    n = agfstnode(g);
    while !n.is_null() {
        if ((*((*n).base.data as *mut Agnodeinfo_t)).ptr as *mut Agnode_t).is_null() {
            dn = agnode(dg, agnameof(n as *mut libc::c_void), 1 as libc::c_int);
            agbindrec(
                dn as *mut libc::c_void,
                b"nodeinfo\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
                1 as libc::c_int,
            );
            let ref mut fresh6 = (*((*dn).base.data as *mut Agnodeinfo_t)).ptr;
            *fresh6 = n as *mut Agobj_t;
            let ref mut fresh7 = (*((*n).base.data as *mut Agnodeinfo_t)).ptr;
            *fresh7 = dn as *mut Agobj_t;
        }
        n = agnxtnode(g, n);
    }
    n = agfstnode(g);
    while !n.is_null() {
        let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
        let mut hd: *mut Agnode_t = 0 as *mut Agnode_t;
        let mut tl: *mut Agnode_t = (*((*n).base.data as *mut Agnodeinfo_t)).ptr
            as *mut Agnode_t;
        e = agfstout(g, n);
        while !e.is_null() {
            hd = (*((*(*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node)
                .base
                .data as *mut Agnodeinfo_t))
                .ptr as *mut Agnode_t;
            if !(hd == tl) {
                if hd > tl {
                    agedge(dg, tl, hd, 0 as *mut libc::c_char, 1 as libc::c_int);
                } else {
                    agedge(dg, hd, tl, 0 as *mut libc::c_char, 1 as libc::c_int);
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    return dg;
}
unsafe extern "C" fn unionNodes(mut dg: *mut Agraph_t, mut g: *mut Agraph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut clust: *mut Agraph_t = 0 as *mut Agraph_t;
    dn = agfstnode(dg);
    while !dn.is_null() {
        if ((*(*((*dn).base.data as *mut Agnodeinfo_t)).ptr).tag).objtype()
            as libc::c_int == 1 as libc::c_int
        {
            agsubnode(
                g,
                (*((*dn).base.data as *mut Agnodeinfo_t)).ptr as *mut Agnode_t,
                1 as libc::c_int,
            );
        } else {
            clust = (*((*dn).base.data as *mut Agnodeinfo_t)).ptr as *mut Agraph_t;
            n = agfstnode(clust);
            while !n.is_null() {
                agsubnode(g, n, 1 as libc::c_int);
                n = agnxtnode(clust, n);
            }
        }
        dn = agnxtnode(dg, dn);
    }
}
unsafe extern "C" fn cmp(
    mut p0: *mut *mut Agraph_t,
    mut p1: *mut *mut Agraph_t,
) -> libc::c_int {
    return agnnodes(*p1) - agnnodes(*p0);
}
unsafe extern "C" fn printSorted(mut root: *mut Agraph_t, mut c_cnt: libc::c_int) {
    let mut ccs: *mut *mut Agraph_t = gv_calloc(
        c_cnt as size_t,
        ::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong,
    ) as *mut *mut Agraph_t;
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut endi: libc::c_int = 0;
    subg = agfstsubg(root);
    while !subg.is_null() {
        if (*((*subg).base.data as *mut Agraphinfo_t)).cc_subg != 0 {
            let fresh8 = i;
            i = i + 1;
            let ref mut fresh9 = *ccs.offset(fresh8 as isize);
            *fresh9 = subg;
        }
        subg = agnxtsubg(subg);
    }
    qsort(
        ccs as *mut libc::c_void,
        c_cnt as size_t,
        ::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut *mut Agraph_t,
                    *mut *mut Agraph_t,
                ) -> libc::c_int,
            >,
            qsort_cmpf,
        >(
            Some(
                cmp
                    as unsafe extern "C" fn(
                        *mut *mut Agraph_t,
                        *mut *mut Agraph_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    if sortIndex >= 0 as libc::c_int {
        if x_mode == 1 as libc::c_int {
            if sortIndex >= c_cnt {
                fprintf(
                    stderr,
                    b"ccomps: component %d not found in graph %s - ignored\n\0"
                        as *const u8 as *const libc::c_char,
                    sortIndex,
                    agnameof(root as *mut libc::c_void),
                );
                return;
            }
            if sortFinal >= sortIndex {
                endi = sortFinal;
                if endi >= c_cnt {
                    endi = c_cnt - 1 as libc::c_int;
                }
            } else {
                endi = c_cnt - 1 as libc::c_int;
            }
            i = sortIndex;
            while i <= endi {
                subg = *ccs.offset(i as isize);
                if doAll != 0 {
                    subGInduce(root, subg);
                }
                gwrite(subg);
                i += 1;
            }
        } else if x_mode == 2 as libc::c_int {
            if sortFinal == -(1 as libc::c_int) {
                sortFinal = agnnodes(*ccs.offset(0 as libc::c_int as isize));
            }
            i = 0 as libc::c_int;
            while i < c_cnt {
                let mut sz: libc::c_int = 0;
                subg = *ccs.offset(i as isize);
                sz = agnnodes(subg);
                if !(sz > sortFinal) {
                    if sz < sortIndex {
                        break;
                    }
                    if doAll != 0 {
                        subGInduce(root, subg);
                    }
                    gwrite(subg);
                }
                i += 1;
            }
        }
    } else {
        i = 0 as libc::c_int;
        while i < c_cnt {
            subg = *ccs.offset(i as isize);
            if doAll != 0 {
                subGInduce(root, subg);
            }
            gwrite(subg);
            i += 1;
        }
    }
    free(ccs as *mut libc::c_void);
}
unsafe extern "C" fn processClusters(
    mut g: *mut Agraph_t,
    mut graphName: *mut libc::c_char,
) -> libc::c_int {
    let mut dg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n_cnt: libc::c_long = 0;
    let mut c_cnt: libc::c_long = 0;
    let mut e_cnt: libc::c_long = 0;
    let mut out: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut dout: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut dn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut extracted: libc::c_int = 0 as libc::c_int;
    dg = deriveGraph(g);
    if !x_node.is_null() {
        n = agnode(g, x_node, 0 as libc::c_int);
        if n.is_null() {
            fprintf(
                stderr,
                b"ccomps: node %s not found in graph %s\n\0" as *const u8
                    as *const libc::c_char,
                x_node,
                agnameof(g as *mut libc::c_void),
            );
            return 1 as libc::c_int;
        }
        let mut name: *mut libc::c_char = gv_alloc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(graphName)),
        ) as *mut libc::c_char;
        sprintf(name, b"%s_cc\0" as *const u8 as *const libc::c_char, graphName);
        dout = agsubg(dg, name, 1 as libc::c_int);
        out = agsubg(g, name, 1 as libc::c_int);
        free(name as *mut libc::c_void);
        aginit(
            out,
            0 as libc::c_int,
            b"graphinfo\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
            (0 as libc::c_int == 0) as libc::c_int,
        );
        (*((*out).base.data as *mut Agraphinfo_t))
            .cc_subg = 1 as libc::c_int as libc::c_char;
        dn = (*((*n).base.data as *mut Agnodeinfo_t)).ptr as *mut Agnode_t;
        n_cnt = dfs(dg, dn, dout) as libc::c_long;
        unionNodes(dout, out);
        if doEdges != 0 {
            e_cnt = nodeInduce(out, (*out).root) as libc::c_long;
        } else {
            e_cnt = 0 as libc::c_int as libc::c_long;
        }
        if doAll != 0 {
            subGInduce(g, out);
        }
        gwrite(out);
        if verbose != 0 {
            fprintf(
                stderr,
                b" %7ld nodes %7ld edges\n\0" as *const u8 as *const libc::c_char,
                n_cnt,
                e_cnt,
            );
        }
        return 0 as libc::c_int;
    }
    c_cnt = 0 as libc::c_int as libc::c_long;
    dn = agfstnode(dg);
    while !dn.is_null() {
        if !((*((*dn).base.data as *mut Agnodeinfo_t)).mark != 0) {
            let mut name_0: *mut libc::c_char = gv_alloc(
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_add(strlen(graphName))
                    .wrapping_add(32 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            sprintf(
                name_0,
                b"%s_cc_%ld\0" as *const u8 as *const libc::c_char,
                graphName,
                c_cnt,
            );
            dout = agsubg(dg, name_0, 1 as libc::c_int);
            out = agsubg(g, name_0, 1 as libc::c_int);
            free(name_0 as *mut libc::c_void);
            aginit(
                out,
                0 as libc::c_int,
                b"graphinfo\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            (*((*out).base.data as *mut Agraphinfo_t))
                .cc_subg = 1 as libc::c_int as libc::c_char;
            n_cnt = dfs(dg, dn, dout) as libc::c_long;
            unionNodes(dout, out);
            if doEdges != 0 {
                e_cnt = nodeInduce(out, (*out).root) as libc::c_long;
            } else {
                e_cnt = 0 as libc::c_int as libc::c_long;
            }
            if printMode == 1 as libc::c_int {
                if doAll != 0 {
                    subGInduce(g, out);
                }
                gwrite(out);
            } else if printMode == 3 as libc::c_int {
                if x_mode == 1 as libc::c_int {
                    if x_index as libc::c_long <= c_cnt {
                        extracted = 1 as libc::c_int;
                        if doAll != 0 {
                            subGInduce(g, out);
                        }
                        gwrite(out);
                        if c_cnt == x_final as libc::c_long {
                            return 0 as libc::c_int;
                        }
                    }
                } else if x_mode == 2 as libc::c_int {
                    let mut sz: libc::c_int = agnnodes(out);
                    if x_index <= sz && (x_final == -(1 as libc::c_int) || sz <= x_final)
                    {
                        extracted = 1 as libc::c_int;
                        if doAll != 0 {
                            subGInduce(g, out);
                        }
                        gwrite(out);
                    }
                }
            }
            if printMode != 0 as libc::c_int {
                agdelete(g, out as *mut libc::c_void);
            }
            agdelete(dg, dout as *mut libc::c_void);
            if verbose != 0 {
                fprintf(
                    stderr,
                    b"(%4ld) %7ld nodes %7ld edges\n\0" as *const u8
                        as *const libc::c_char,
                    c_cnt,
                    n_cnt,
                    e_cnt,
                );
            }
            c_cnt += 1;
        }
        dn = agnxtnode(dg, dn);
    }
    if printMode == 3 as libc::c_int && extracted == 0 && x_mode == 1 as libc::c_int {
        fprintf(
            stderr,
            b"ccomps: component %d not found in graph %s - ignored\n\0" as *const u8
                as *const libc::c_char,
            x_index,
            agnameof(g as *mut libc::c_void),
        );
        return 1 as libc::c_int;
    }
    if sorted != 0 {
        printSorted(g, c_cnt as libc::c_int);
    } else if printMode == 0 as libc::c_int {
        gwrite(g);
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"       %7d nodes %7d edges %7ld components %s\n\0" as *const u8
                as *const libc::c_char,
            agnnodes(g),
            agnedges(g),
            c_cnt,
            agnameof(g as *mut libc::c_void),
        );
    }
    agclose(dg);
    return if c_cnt != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
unsafe extern "C" fn bindGraphinfo(mut g: *mut Agraph_t) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    aginit(
        g,
        0 as libc::c_int,
        b"graphinfo\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    subg = agfstsubg(g);
    while !subg.is_null() {
        bindGraphinfo(subg);
        subg = agnxtsubg(subg);
    }
}
unsafe extern "C" fn process(
    mut g: *mut Agraph_t,
    mut graphName: *mut libc::c_char,
) -> libc::c_int {
    let mut n_cnt: libc::c_long = 0;
    let mut c_cnt: libc::c_long = 0;
    let mut e_cnt: libc::c_long = 0;
    let mut out: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut extracted: libc::c_int = 0 as libc::c_int;
    aginit(
        g,
        1 as libc::c_int,
        b"nodeinfo\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    bindGraphinfo(g);
    if useClusters != 0 {
        return processClusters(g, graphName);
    }
    if !x_node.is_null() {
        n = agnode(g, x_node, 0 as libc::c_int);
        if n.is_null() {
            fprintf(
                stderr,
                b"ccomps: node %s not found in graph %s - ignored\n\0" as *const u8
                    as *const libc::c_char,
                x_node,
                agnameof(g as *mut libc::c_void),
            );
            return 1 as libc::c_int;
        }
        let mut name: *mut libc::c_char = gv_alloc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(graphName)),
        ) as *mut libc::c_char;
        sprintf(name, b"%s_cc\0" as *const u8 as *const libc::c_char, graphName);
        out = agsubg(g, name, 1 as libc::c_int);
        free(name as *mut libc::c_void);
        aginit(
            out,
            0 as libc::c_int,
            b"graphinfo\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
            (0 as libc::c_int == 0) as libc::c_int,
        );
        (*((*out).base.data as *mut Agraphinfo_t))
            .cc_subg = 1 as libc::c_int as libc::c_char;
        n_cnt = dfs(g, n, out) as libc::c_long;
        if doEdges != 0 {
            e_cnt = nodeInduce(out, (*out).root) as libc::c_long;
        } else {
            e_cnt = 0 as libc::c_int as libc::c_long;
        }
        if doAll != 0 {
            subGInduce(g, out);
        }
        gwrite(out);
        if verbose != 0 {
            fprintf(
                stderr,
                b" %7ld nodes %7ld edges\n\0" as *const u8 as *const libc::c_char,
                n_cnt,
                e_cnt,
            );
        }
        return 0 as libc::c_int;
    }
    c_cnt = 0 as libc::c_int as libc::c_long;
    n = agfstnode(g);
    while !n.is_null() {
        if !((*((*n).base.data as *mut Agnodeinfo_t)).mark != 0) {
            let mut name_0: *mut libc::c_char = gv_alloc(
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_add(strlen(graphName))
                    .wrapping_add(32 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            sprintf(
                name_0,
                b"%s_cc_%ld\0" as *const u8 as *const libc::c_char,
                graphName,
                c_cnt,
            );
            out = agsubg(g, name_0, 1 as libc::c_int);
            free(name_0 as *mut libc::c_void);
            aginit(
                out,
                0 as libc::c_int,
                b"graphinfo\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agraphinfo_t>() as libc::c_ulong as libc::c_int,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            (*((*out).base.data as *mut Agraphinfo_t))
                .cc_subg = 1 as libc::c_int as libc::c_char;
            n_cnt = dfs(g, n, out) as libc::c_long;
            if doEdges != 0 {
                e_cnt = nodeInduce(out, (*out).root) as libc::c_long;
            } else {
                e_cnt = 0 as libc::c_int as libc::c_long;
            }
            if printMode == 1 as libc::c_int {
                if doAll != 0 {
                    subGInduce(g, out);
                }
                gwrite(out);
            } else if printMode == 3 as libc::c_int {
                if x_mode == 1 as libc::c_int {
                    if x_index as libc::c_long <= c_cnt {
                        extracted = 1 as libc::c_int;
                        if doAll != 0 {
                            subGInduce(g, out);
                        }
                        gwrite(out);
                        if c_cnt == x_final as libc::c_long {
                            return 0 as libc::c_int;
                        }
                    }
                } else if x_mode == 2 as libc::c_int {
                    let mut sz: libc::c_int = agnnodes(out);
                    if x_index <= sz && (x_final == -(1 as libc::c_int) || sz <= x_final)
                    {
                        extracted = 1 as libc::c_int;
                        if doAll != 0 {
                            subGInduce(g, out);
                        }
                        gwrite(out);
                    }
                }
            }
            if printMode != 0 as libc::c_int {
                agdelete(g, out as *mut libc::c_void);
            }
            if verbose != 0 {
                fprintf(
                    stderr,
                    b"(%4ld) %7ld nodes %7ld edges\n\0" as *const u8
                        as *const libc::c_char,
                    c_cnt,
                    n_cnt,
                    e_cnt,
                );
            }
            c_cnt += 1;
        }
        n = agnxtnode(g, n);
    }
    if printMode == 3 as libc::c_int && extracted == 0 && x_mode == 1 as libc::c_int {
        fprintf(
            stderr,
            b"ccomps: component %d not found in graph %s - ignored\n\0" as *const u8
                as *const libc::c_char,
            x_index,
            agnameof(g as *mut libc::c_void),
        );
        return 1 as libc::c_int;
    }
    if sorted != 0 {
        printSorted(g, c_cnt as libc::c_int);
    } else if printMode == 0 as libc::c_int {
        gwrite(g);
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"       %7d nodes %7d edges %7ld components %s\n\0" as *const u8
                as *const libc::c_char,
            agnnodes(g),
            agnedges(g),
            c_cnt,
            agnameof(g as *mut libc::c_void),
        );
    }
    return (c_cnt > 1 as libc::c_int as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe extern "C" fn chkGraphName(mut g: *mut Agraph_t) -> *mut libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buflen: size_t = 0 as libc::c_int as size_t;
    let mut s: *mut libc::c_char = agnameof(g as *mut libc::c_void);
    if *s as libc::c_int != '%' as i32 {
        return s;
    }
    let mut len: size_t = (strlen(s)).wrapping_add(2 as libc::c_int as libc::c_ulong);
    if len > buflen {
        buf = gv_realloc(buf as *mut libc::c_void, buflen, len) as *mut libc::c_char;
        buflen = len;
    }
    *buf.offset(0 as libc::c_int as isize) = '_' as i32 as libc::c_char;
    strcpy(buf.offset(1 as libc::c_int as isize), s);
    return buf;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
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
    let mut r: libc::c_int = 0 as libc::c_int;
    init(argc, argv);
    newIngraph(
        &mut ig,
        Files,
        Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
    );
    loop {
        g = nextGraph(&mut ig);
        if g.is_null() {
            break;
        }
        r += process(g, chkGraphName(g));
        agclose(g);
    }
    stack_reset(&mut Stk);
    graphviz_exit(r);
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
