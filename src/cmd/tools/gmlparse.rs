#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    static mut Dtqueue: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn initgmlscan(_: *mut FILE);
    fn gmlerrors() -> libc::c_int;
    fn gmlerror(_: *const libc::c_char);
    fn gmllexeof();
    fn gmllex() -> libc::c_int;
    static mut Agundirected: Agdesc_t;
    fn agsafeset(
        obj: *mut libc::c_void,
        name: *mut libc::c_char,
        value: *const libc::c_char,
        def: *const libc::c_char,
    ) -> libc::c_int;
    fn agsubg(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        cflag: libc::c_int,
    ) -> *mut Agraph_t;
    static mut Agdirected: Agdesc_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agopen(
        name: *mut libc::c_char,
        desc: Agdesc_t,
        disc: *mut Agdisc_t,
    ) -> *mut Agraph_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub struct gmlattr {
    pub link: Dtlink_t,
    pub kind: libc::c_ushort,
    pub sort: libc::c_ushort,
    pub name: *mut libc::c_char,
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub value: *mut libc::c_char,
    pub lp: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmlnode {
    pub link: Dtlink_t,
    pub id: *mut libc::c_char,
    pub attrlist: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmledge {
    pub link: Dtlink_t,
    pub source: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub attrlist: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmlgraph {
    pub link: Dtlink_t,
    pub parent: *mut gmlgraph,
    pub directed: libc::c_int,
    pub attrlist: *mut Dt_t,
    pub nodelist: *mut Dt_t,
    pub edgelist: *mut Dt_t,
    pub graphlist: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub union GMLSTYPE {
    pub i: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub np: *mut gmlnode,
    pub ep: *mut gmledge,
    pub ap: *mut gmlattr,
    pub list: *mut Dt_t,
}
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
pub const FIRST_ALLOCATION: C2RustUnnamed_3 = 512;
pub type C2RustUnnamed_3 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: GMLSTYPE,
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
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
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
        let ref mut fresh0 = (*xb).ptr;
        *fresh0 = (*fresh0).offset(result as size_t as isize);
    }
    return result;
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
    let ref mut fresh1 = (*xb).buf;
    *fresh1 = nbuf;
    let ref mut fresh2 = (*xb).ptr;
    *fresh2 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh3 = (*xb).eptr;
    *fresh3 = ((*xb).buf).offset(nsize as isize);
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
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh4 = (*xb).buf;
        *fresh4 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh5 = (*xb).buf;
        *fresh5 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh6 = (*xb).eptr;
    *fresh6 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh7 = (*xb).ptr;
    *fresh7 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
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
unsafe extern "C" fn agxbput_n(
    mut xb: *mut agxbuf,
    mut s: *const libc::c_char,
    mut ssz: size_t,
) -> size_t {
    if ((*xb).ptr).offset(ssz as isize) > (*xb).eptr {
        agxbmore(xb, ssz);
    }
    memcpy((*xb).ptr as *mut libc::c_void, s as *const libc::c_void, ssz);
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
unsafe extern "C" fn agxblen(mut xb: *const agxbuf) -> size_t {
    return ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn agxbclear(mut xb: *mut agxbuf) {
    let ref mut fresh12 = (*xb).ptr;
    *fresh12 = (*xb).buf;
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
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh13 = (*stack).size;
    *fresh13 = (*fresh13).wrapping_sub(1);
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
        let ref mut fresh14 = (*stack).base;
        *fresh14 = b;
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
    let ref mut fresh15 = *((*stack).base).offset((*stack).size as isize);
    *fresh15 = item;
    let ref mut fresh16 = (*stack).size;
    *fresh16 = (*fresh16).wrapping_add(1);
    return 0 as libc::c_int;
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
static mut G: *mut gmlgraph = 0 as *const gmlgraph as *mut gmlgraph;
static mut N: *mut gmlnode = 0 as *const gmlnode as *mut gmlnode;
static mut E: *mut gmledge = 0 as *const gmledge as *mut gmledge;
static mut L: *mut Dt_t = 0 as *const Dt_t as *mut Dt_t;
static mut liststk: gv_stack_t = gv_stack_t {
    base: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    size: 0,
    capacity: 0,
};
unsafe extern "C" fn free_node(
    mut d: *mut Dt_t,
    mut p: *mut gmlnode,
    mut ds: *mut Dtdisc_t,
) {
    if p.is_null() {
        return;
    }
    if !((*p).attrlist).is_null() {
        dtclose((*p).attrlist);
    }
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn free_edge(
    mut d: *mut Dt_t,
    mut p: *mut gmledge,
    mut ds: *mut Dtdisc_t,
) {
    if p.is_null() {
        return;
    }
    if !((*p).attrlist).is_null() {
        dtclose((*p).attrlist);
    }
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn free_graph(
    mut d: *mut Dt_t,
    mut p: *mut gmlgraph,
    mut ds: *mut Dtdisc_t,
) {
    if p.is_null() {
        return;
    }
    if !((*p).nodelist).is_null() {
        dtclose((*p).nodelist);
    }
    if !((*p).edgelist).is_null() {
        dtclose((*p).edgelist);
    }
    if !((*p).attrlist).is_null() {
        dtclose((*p).attrlist);
    }
    if !((*p).graphlist).is_null() {
        dtclose((*p).graphlist);
    }
    free(p as *mut libc::c_void);
}
static mut nodeDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 24 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<*mut Dt_t>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut gmlnode, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_node
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut gmlnode,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
static mut edgeDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 32 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<*mut Dt_t>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut gmledge, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_edge
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut gmledge,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
static mut attrDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 24 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut gmlattr, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_attr
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut gmlattr,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
static mut graphDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 40 as libc::c_ulong as libc::c_int,
            size: ::std::mem::size_of::<*mut Dt_t>() as libc::c_ulong as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut Dt_t, *mut gmlgraph, *mut Dtdisc_t) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_graph
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut gmlgraph,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: None,
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn cleanup() {
    while !stack_is_empty(&mut liststk) {
        let mut dt: *mut Dt_t = stack_pop(&mut liststk) as *mut Dt_t;
        dtclose(dt);
    }
    stack_reset(&mut liststk);
    if !L.is_null() {
        dtclose(L);
        L = 0 as *mut Dt_t;
    }
    if !N.is_null() {
        free_node(0 as *mut Dt_t, N, 0 as *mut Dtdisc_t);
        N = 0 as *mut gmlnode;
    }
    if !E.is_null() {
        free_edge(0 as *mut Dt_t, E, 0 as *mut Dtdisc_t);
        E = 0 as *mut gmledge;
    }
    if !G.is_null() {
        free_graph(0 as *mut Dt_t, G, 0 as *mut Dtdisc_t);
        G = 0 as *mut gmlgraph;
    }
}
unsafe extern "C" fn pushAlist() {
    let mut lp: *mut Dt_t = dtopen(&mut attrDisc, Dtqueue);
    if !L.is_null() {
        stack_push_or_exit(&mut liststk, L as *mut libc::c_void);
    }
    L = lp;
}
unsafe extern "C" fn popAlist() -> *mut Dt_t {
    let mut lp: *mut Dt_t = L;
    if !stack_is_empty(&mut liststk) {
        L = stack_pop(&mut liststk) as *mut Dt_t;
    } else {
        L = 0 as *mut Dt_t;
    }
    return lp;
}
unsafe extern "C" fn popG() {
    G = (*G).parent;
}
unsafe extern "C" fn pushG() {
    let mut g: *mut gmlgraph = malloc(::std::mem::size_of::<gmlgraph>() as libc::c_ulong)
        as *mut gmlgraph;
    let ref mut fresh17 = (*g).attrlist;
    *fresh17 = dtopen(&mut attrDisc, Dtqueue);
    let ref mut fresh18 = (*g).nodelist;
    *fresh18 = dtopen(&mut nodeDisc, Dtqueue);
    let ref mut fresh19 = (*g).edgelist;
    *fresh19 = dtopen(&mut edgeDisc, Dtqueue);
    let ref mut fresh20 = (*g).graphlist;
    *fresh20 = dtopen(&mut graphDisc, Dtqueue);
    let ref mut fresh21 = (*g).parent;
    *fresh21 = G;
    (*g).directed = -(1 as libc::c_int);
    if !G.is_null() {
        (Some(((*(*G).graphlist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*G).graphlist, g as *mut libc::c_void, 0o1 as libc::c_int);
    }
    G = g;
}
unsafe extern "C" fn mkNode() -> *mut gmlnode {
    let mut np: *mut gmlnode = malloc(::std::mem::size_of::<gmlnode>() as libc::c_ulong)
        as *mut gmlnode;
    let ref mut fresh22 = (*np).attrlist;
    *fresh22 = dtopen(&mut attrDisc, Dtqueue);
    let ref mut fresh23 = (*np).id;
    *fresh23 = 0 as *mut libc::c_char;
    return np;
}
unsafe extern "C" fn mkEdge() -> *mut gmledge {
    let mut ep: *mut gmledge = malloc(::std::mem::size_of::<gmledge>() as libc::c_ulong)
        as *mut gmledge;
    let ref mut fresh24 = (*ep).attrlist;
    *fresh24 = dtopen(&mut attrDisc, Dtqueue);
    let ref mut fresh25 = (*ep).source;
    *fresh25 = 0 as *mut libc::c_char;
    let ref mut fresh26 = (*ep).target;
    *fresh26 = 0 as *mut libc::c_char;
    return ep;
}
unsafe extern "C" fn mkAttr(
    mut name: *mut libc::c_char,
    mut sort: libc::c_ushort,
    mut kind: libc::c_ushort,
    mut str: *mut libc::c_char,
    mut list: *mut Dt_t,
) -> *mut gmlattr {
    let mut gp: *mut gmlattr = malloc(::std::mem::size_of::<gmlattr>() as libc::c_ulong)
        as *mut gmlattr;
    if !name.is_null() || sort as libc::c_int != 0 {} else {
        __assert_fail(
            b"name || sort\0" as *const u8 as *const libc::c_char,
            b"../../cmd/tools/gmlparse.y\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"gmlattr *mkAttr(char *, unsigned short, unsigned short, char *, Dt_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if name.is_null() {
        name = strdup(sortToStr(sort));
    }
    (*gp).sort = sort;
    (*gp).kind = kind;
    let ref mut fresh27 = (*gp).name;
    *fresh27 = name;
    if !str.is_null() {
        let ref mut fresh28 = (*gp).u.value;
        *fresh28 = str;
    } else {
        if dtsize(list) == 0 as libc::c_int {
            dtclose(list);
            list = 0 as *mut Dt_t;
        }
        let ref mut fresh29 = (*gp).u.lp;
        *fresh29 = list;
    }
    return gp;
}
unsafe extern "C" fn setDir(mut d: *mut libc::c_char) -> libc::c_int {
    let mut g: *mut gmlgraph = 0 as *mut gmlgraph;
    let mut dir: libc::c_int = atoi(d);
    free(d as *mut libc::c_void);
    if dir < 0 as libc::c_int {
        dir = -(1 as libc::c_int);
    } else if dir > 0 as libc::c_int {
        dir = 1 as libc::c_int;
    } else {
        dir = 0 as libc::c_int;
    }
    (*G).directed = dir;
    if dir >= 0 as libc::c_int {
        g = (*G).parent;
        while !g.is_null() {
            if (*g).directed < 0 as libc::c_int {
                (*g).directed = dir;
            } else if (*g).directed != dir {
                return 1 as libc::c_int
            }
            g = (*g).parent;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_attr(
    mut d: *mut Dt_t,
    mut p: *mut gmlattr,
    mut ds: *mut Dtdisc_t,
) {
    if p.is_null() {
        return;
    }
    if (*p).kind as libc::c_int == 289 as libc::c_int && !((*p).u.lp).is_null() {
        dtclose((*p).u.lp);
    } else {
        free((*p).u.value as *mut libc::c_void);
    }
    free((*p).name as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn deparseAttr(mut ap: *mut gmlattr, mut xb: *mut agxbuf) {
    if (*ap).kind as libc::c_int == 289 as libc::c_int {
        agxbprint(xb, b"%s \0" as *const u8 as *const libc::c_char, (*ap).name);
        deparseList((*ap).u.lp, xb);
    } else if (*ap).kind as libc::c_int == 286 as libc::c_int {
        agxbprint(
            xb,
            b"%s \"%s\"\0" as *const u8 as *const libc::c_char,
            (*ap).name,
            (*ap).u.value,
        );
    } else {
        agxbprint(
            xb,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            (*ap).name,
            (*ap).u.value,
        );
    };
}
unsafe extern "C" fn deparseList(mut alist: *mut Dt_t, mut xb: *mut agxbuf) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    agxbput(xb, b"[ \0" as *const u8 as *const libc::c_char);
    if !alist.is_null() {
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
        while !ap.is_null() {
            deparseAttr(ap, xb);
            agxbputc(xb, ' ' as i32 as libc::c_char);
            ap = (Some(((*alist).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
        }
    }
    agxbput(xb, b"]\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn unknown(
    mut obj: *mut Agobj_t,
    mut ap: *mut gmlattr,
    mut xb: *mut agxbuf,
) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*ap).kind as libc::c_int == 289 as libc::c_int {
        deparseList((*ap).u.lp, xb);
        str = agxbuse(xb);
    } else {
        str = (*ap).u.value;
    }
    agsafeset(
        obj as *mut libc::c_void,
        (*ap).name,
        str,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn addNodeLabelGraphics(
    mut np: *mut Agnode_t,
    mut alist: *mut Dt_t,
    mut unk: *mut agxbuf,
) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    if alist.is_null() {
        return;
    }
    ap = (Some(((*alist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
    while !ap.is_null() {
        if (*ap).sort as libc::c_int == 280 as libc::c_int {
            agsafeset(
                np as *mut libc::c_void,
                b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 283 as libc::c_int {
            agsafeset(
                np as *mut libc::c_void,
                b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 281 as libc::c_int {
            agsafeset(
                np as *mut libc::c_void,
                b"fontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 282 as libc::c_int {
            agsafeset(
                np as *mut libc::c_void,
                b"fontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if cnt != 0 {
                agxbputc(unk, ' ' as i32 as libc::c_char);
            } else {
                agxbput(unk, b"[ \0" as *const u8 as *const libc::c_char);
            }
            deparseAttr(ap, unk);
            cnt += 1;
        }
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
    }
    if cnt != 0 {
        agxbput(unk, b" ]\0" as *const u8 as *const libc::c_char);
        agsafeset(
            np as *mut libc::c_void,
            b"LabelGraphics\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(unk),
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        agxbclear(unk);
    };
}
unsafe extern "C" fn addEdgeLabelGraphics(
    mut ep: *mut Agedge_t,
    mut alist: *mut Dt_t,
    mut xb: *mut agxbuf,
    mut unk: *mut agxbuf,
) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    let mut x: *mut libc::c_char = b"0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut y: *mut libc::c_char = b"0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    if alist.is_null() {
        return;
    }
    ap = (Some(((*alist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
    while !ap.is_null() {
        if (*ap).sort as libc::c_int == 280 as libc::c_int {
            agsafeset(
                ep as *mut libc::c_void,
                b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 283 as libc::c_int {
            agsafeset(
                ep as *mut libc::c_void,
                b"fontcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 281 as libc::c_int {
            agsafeset(
                ep as *mut libc::c_void,
                b"fontsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 282 as libc::c_int {
            agsafeset(
                ep as *mut libc::c_void,
                b"fontname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 264 as libc::c_int {
            x = (*ap).u.value;
        } else if (*ap).sort as libc::c_int == 265 as libc::c_int {
            y = (*ap).u.value;
        } else {
            if cnt != 0 {
                agxbputc(unk, ' ' as i32 as libc::c_char);
            } else {
                agxbput(unk, b"[ \0" as *const u8 as *const libc::c_char);
            }
            deparseAttr(ap, unk);
            cnt += 1;
        }
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
    }
    agxbprint(xb, b"%s,%s\0" as *const u8 as *const libc::c_char, x, y);
    agsafeset(
        ep as *mut libc::c_void,
        b"lp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        agxbuse(xb),
        b"\0" as *const u8 as *const libc::c_char,
    );
    if cnt != 0 {
        agxbput(unk, b" ]\0" as *const u8 as *const libc::c_char);
        agsafeset(
            ep as *mut libc::c_void,
            b"LabelGraphics\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(unk),
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        agxbclear(unk);
    };
}
unsafe extern "C" fn addNodeGraphics(
    mut np: *mut Agnode_t,
    mut alist: *mut Dt_t,
    mut xb: *mut agxbuf,
    mut unk: *mut agxbuf,
) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    let mut x: *mut libc::c_char = b"0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut y: *mut libc::c_char = b"0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut d: libc::c_double = 0.;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    ap = (Some(((*alist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
    while !ap.is_null() {
        if (*ap).sort as libc::c_int == 264 as libc::c_int {
            x = (*ap).u.value;
        } else if (*ap).sort as libc::c_int == 265 as libc::c_int {
            y = (*ap).u.value;
        } else if (*ap).sort as libc::c_int == 266 as libc::c_int {
            d = atof((*ap).u.value);
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                b"%.04f\0" as *const u8 as *const libc::c_char,
                d / 72.0f64,
            );
            agsafeset(
                np as *mut libc::c_void,
                b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 267 as libc::c_int {
            d = atof((*ap).u.value);
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                b"%.04f\0" as *const u8 as *const libc::c_char,
                d / 72.0f64,
            );
            agsafeset(
                np as *mut libc::c_void,
                b"height\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 271 as libc::c_int {
            agsafeset(
                np as *mut libc::c_void,
                b"shape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 272 as libc::c_int {
            agsafeset(
                np as *mut libc::c_void,
                b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 273 as libc::c_int {
            agsafeset(
                np as *mut libc::c_void,
                b"pencolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 276 as libc::c_int
                || (*ap).sort as libc::c_int == 275 as libc::c_int
            {
            agsafeset(
                np as *mut libc::c_void,
                b"penwidth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 277 as libc::c_int
                || (*ap).sort as libc::c_int == 274 as libc::c_int
            {
            agsafeset(
                np as *mut libc::c_void,
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if cnt != 0 {
                agxbputc(unk, ' ' as i32 as libc::c_char);
            } else {
                agxbput(unk, b"[ \0" as *const u8 as *const libc::c_char);
            }
            deparseAttr(ap, unk);
            cnt += 1;
        }
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
    }
    agxbprint(xb, b"%s,%s\0" as *const u8 as *const libc::c_char, x, y);
    agsafeset(
        np as *mut libc::c_void,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        agxbuse(xb),
        b"\0" as *const u8 as *const libc::c_char,
    );
    if cnt != 0 {
        agxbput(unk, b" ]\0" as *const u8 as *const libc::c_char);
        agsafeset(
            np as *mut libc::c_void,
            b"graphics\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(unk),
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        agxbclear(unk);
    };
}
unsafe extern "C" fn addEdgePoint(
    mut ep: *mut Agedge_t,
    mut alist: *mut Dt_t,
    mut xb: *mut agxbuf,
) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    let mut x: *mut libc::c_char = b"0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut y: *mut libc::c_char = b"0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    ap = (Some(((*alist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
    while !ap.is_null() {
        if (*ap).sort as libc::c_int == 264 as libc::c_int {
            x = (*ap).u.value;
        } else if (*ap).sort as libc::c_int == 265 as libc::c_int {
            y = (*ap).u.value;
        } else {
            fprintf(
                stderr,
                b"non-X/Y field in point attribute\0" as *const u8 as *const libc::c_char,
            );
            unknown(ep as *mut Agobj_t, ap, xb);
        }
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
    }
    if agxblen(xb) != 0 {
        agxbputc(xb, ' ' as i32 as libc::c_char);
    }
    agxbprint(xb, b"%s,%s\0" as *const u8 as *const libc::c_char, x, y);
}
unsafe extern "C" fn addEdgePos(
    mut ep: *mut Agedge_t,
    mut alist: *mut Dt_t,
    mut xb: *mut agxbuf,
) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    if alist.is_null() {
        return;
    }
    ap = (Some(((*alist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
    while !ap.is_null() {
        if (*ap).sort as libc::c_int == 279 as libc::c_int {
            addEdgePoint(ep, (*ap).u.lp, xb);
        } else {
            fprintf(
                stderr,
                b"non-point field in line attribute\0" as *const u8
                    as *const libc::c_char,
            );
            unknown(ep as *mut Agobj_t, ap, xb);
        }
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
    }
    agsafeset(
        ep as *mut libc::c_void,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        agxbuse(xb),
        b"\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn addEdgeGraphics(
    mut ep: *mut Agedge_t,
    mut alist: *mut Dt_t,
    mut xb: *mut agxbuf,
    mut unk: *mut agxbuf,
) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    ap = (Some(((*alist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
    while !ap.is_null() {
        if (*ap).sort as libc::c_int == 276 as libc::c_int {
            agsafeset(
                ep as *mut libc::c_void,
                b"penwidth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 277 as libc::c_int {
            agsafeset(
                ep as *mut libc::c_void,
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 272 as libc::c_int {
            agsafeset(
                ep as *mut libc::c_void,
                b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else if (*ap).sort as libc::c_int == 278 as libc::c_int {
            addEdgePos(ep, (*ap).u.lp, xb);
        } else {
            if cnt != 0 {
                agxbputc(unk, ' ' as i32 as libc::c_char);
            } else {
                agxbput(unk, b"[ \0" as *const u8 as *const libc::c_char);
            }
            deparseAttr(ap, unk);
            cnt += 1;
        }
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
    }
    if cnt != 0 {
        agxbput(unk, b" ]\0" as *const u8 as *const libc::c_char);
        agsafeset(
            ep as *mut libc::c_void,
            b"graphics\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxbuse(unk),
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        agxbclear(unk);
    };
}
unsafe extern "C" fn addAttrs(
    mut obj: *mut Agobj_t,
    mut alist: *mut Dt_t,
    mut xb: *mut agxbuf,
    mut unk: *mut agxbuf,
) {
    let mut ap: *mut gmlattr = 0 as *mut gmlattr;
    ap = (Some(((*alist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(alist, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut gmlattr;
    while !ap.is_null() {
        if (*ap).sort as libc::c_int == 269 as libc::c_int {
            if ((*obj).tag).objtype() as libc::c_int == 1 as libc::c_int {
                addNodeGraphics(obj as *mut Agnode_t, (*ap).u.lp, xb, unk);
            } else if ((*obj).tag).objtype() as libc::c_int == 2 as libc::c_int {
                addEdgeGraphics(obj as *mut Agedge_t, (*ap).u.lp, xb, unk);
            } else {
                unknown(obj, ap, xb);
            }
        } else if (*ap).sort as libc::c_int == 270 as libc::c_int {
            if ((*obj).tag).objtype() as libc::c_int == 1 as libc::c_int {
                addNodeLabelGraphics(obj as *mut Agnode_t, (*ap).u.lp, unk);
            } else if ((*obj).tag).objtype() as libc::c_int == 2 as libc::c_int {
                addEdgeLabelGraphics(obj as *mut Agedge_t, (*ap).u.lp, xb, unk);
            } else {
                unknown(obj, ap, xb);
            }
        } else if (*ap).sort as libc::c_int == 268 as libc::c_int
                && ((*obj).tag).objtype() as libc::c_int != 0 as libc::c_int
            {
            agsafeset(
                obj as *mut libc::c_void,
                b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ap).u.value,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            unknown(obj, ap, xb);
        }
        ap = (Some(((*alist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(alist, ap as *mut libc::c_void, 0o10 as libc::c_int) as *mut gmlattr;
    }
}
unsafe extern "C" fn mkGraph(
    mut graph: *mut gmlgraph,
    mut parent: *mut Agraph_t,
    mut name: *mut libc::c_char,
    mut xb: *mut agxbuf,
    mut unk: *mut agxbuf,
) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut np: *mut gmlnode = 0 as *mut gmlnode;
    let mut ep: *mut gmledge = 0 as *mut gmledge;
    let mut gp: *mut gmlgraph = 0 as *mut gmlgraph;
    if !parent.is_null() {
        g = agsubg(parent, 0 as *mut libc::c_char, 1 as libc::c_int);
    } else if (*graph).directed >= 1 as libc::c_int {
        g = agopen(name, Agdirected, 0 as *mut Agdisc_t);
    } else {
        g = agopen(name, Agundirected, 0 as *mut Agdisc_t);
    }
    if parent.is_null() && !L.is_null() {
        addAttrs(g as *mut Agobj_t, L, xb, unk);
    }
    np = (Some(((*(*graph).nodelist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*graph).nodelist, 0 as *mut libc::c_void, 0o200 as libc::c_int)
        as *mut gmlnode;
    while !np.is_null() {
        if ((*np).id).is_null() {
            fprintf(
                stderr,
                b"node without an id attribute\0" as *const u8 as *const libc::c_char,
            );
            graphviz_exit(1 as libc::c_int);
        }
        n = agnode(g, (*np).id, 1 as libc::c_int);
        addAttrs(n as *mut Agobj_t, (*np).attrlist, xb, unk);
        np = (Some(((*(*graph).nodelist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*graph).nodelist, np as *mut libc::c_void, 0o10 as libc::c_int)
            as *mut gmlnode;
    }
    ep = (Some(((*(*graph).edgelist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*graph).edgelist, 0 as *mut libc::c_void, 0o200 as libc::c_int)
        as *mut gmledge;
    while !ep.is_null() {
        if ((*ep).source).is_null() {
            fprintf(
                stderr,
                b"edge without an source attribute\0" as *const u8 as *const libc::c_char,
            );
            graphviz_exit(1 as libc::c_int);
        }
        if ((*ep).target).is_null() {
            fprintf(
                stderr,
                b"node without an target attribute\0" as *const u8 as *const libc::c_char,
            );
            graphviz_exit(1 as libc::c_int);
        }
        n = agnode(g, (*ep).source, 1 as libc::c_int);
        h = agnode(g, (*ep).target, 1 as libc::c_int);
        e = agedge(g, n, h, 0 as *mut libc::c_char, 1 as libc::c_int);
        addAttrs(e as *mut Agobj_t, (*ep).attrlist, xb, unk);
        ep = (Some(((*(*graph).edgelist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*graph).edgelist, ep as *mut libc::c_void, 0o10 as libc::c_int)
            as *mut gmledge;
    }
    gp = (Some(((*(*graph).graphlist).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*graph).graphlist, 0 as *mut libc::c_void, 0o200 as libc::c_int)
        as *mut gmlgraph;
    while !gp.is_null() {
        mkGraph(gp, g, 0 as *mut libc::c_char, xb, unk);
        gp = (Some(((*(*graph).graphlist).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*graph).graphlist, gp as *mut libc::c_void, 0o10 as libc::c_int)
            as *mut gmlgraph;
    }
    addAttrs(g as *mut Agobj_t, (*graph).attrlist, xb, unk);
    return g;
}
static mut yytranslate: [yytype_uint8; 290] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
];
#[no_mangle]
pub unsafe extern "C" fn gml_to_gv(
    mut name: *mut libc::c_char,
    mut fp: *mut FILE,
    mut cnt: libc::c_int,
    mut errors: *mut libc::c_int,
) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut unk: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut unknownb: [libc::c_char; 8192] = [0; 8192];
    let mut error: libc::c_int = 0;
    if cnt == 0 as libc::c_int {
        initgmlscan(fp);
    } else {
        initgmlscan(0 as *mut FILE);
    }
    L = 0 as *mut Dt_t;
    pushAlist();
    gmlparse();
    error = gmlerrors();
    *errors |= error;
    if G.is_null() || error != 0 {
        g = 0 as *mut Agraph_t;
    } else {
        agxbinit(&mut xb, 8192 as libc::c_int as libc::c_uint, buf.as_mut_ptr());
        agxbinit(&mut unk, 8192 as libc::c_int as libc::c_uint, unknownb.as_mut_ptr());
        g = mkGraph(G, 0 as *mut Agraph_t, name, &mut xb, &mut unk);
        agxbfree(&mut xb);
    }
    cleanup();
    return g;
}
unsafe extern "C" fn sortToStr(mut sort: libc::c_ushort) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match sort as libc::c_int {
        258 => {
            s = b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        259 => {
            s = b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        260 => {
            s = b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        261 => {
            s = b"directed\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        287 => {
            s = b"id\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        262 => {
            s = b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        263 => {
            s = b"target\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        264 => {
            s = b"xval\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        265 => {
            s = b"yval\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        266 => {
            s = b"wval\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        267 => {
            s = b"hval\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        268 => {
            s = b"label\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        269 => {
            s = b"graphics\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        270 => {
            s = b"labelGraphics\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        271 => {
            s = b"type\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        272 => {
            s = b"fill\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        273 => {
            s = b"outline\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        274 => {
            s = b"outlineStyle\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        275 => {
            s = b"outlineWidth\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        276 => {
            s = b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        277 => {
            s = b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        278 => {
            s = b"line\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        279 => {
            s = b"point\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        280 => {
            s = b"text\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        281 => {
            s = b"fontSize\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        282 => {
            s = b"fontName\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        283 => {
            s = b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        284 => {
            s = b"integer\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        285 => {
            s = b"real\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        286 => {
            s = b"string\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        288 => {
            s = b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        289 => {
            s = b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        91 => {
            s = b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        93 => {
            s = b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            s = 0 as *mut libc::c_char;
        }
    }
    return s;
}
static mut yypact: [yytype_int16; 102] = [
    1 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(24 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    -(21 as libc::c_int) as yytype_int16,
    -(28 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    30 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    193 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    168 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    91 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 102] = [
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 20] = [
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    118 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    79 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    62 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    82 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    138 as libc::c_int as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(25 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int8; 20] = [
    -(1 as libc::c_int) as yytype_int8,
    23 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 227] = [
    58 as libc::c_int as yytype_int8,
    -(4 as libc::c_int) as yytype_int8,
    1 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    -(34 as libc::c_int) as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 227] = [
    25 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    33 as libc::c_int as yytype_int8,
];
static mut yystos: [yytype_uint8; 102] = [
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 64] = [
    0 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
];
static mut yyr2: [yytype_uint8; 64] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut GMLSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut gmlchar: libc::c_int = 0;
#[no_mangle]
pub static mut gmllval: GMLSTYPE = GMLSTYPE { i: 0 };
#[no_mangle]
pub static mut gmlnerrs: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn gmlparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: libc::c_int = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyvsa: [GMLSTYPE; 200] = [GMLSTYPE { i: 0 }; 200];
    let mut yyvs: *mut GMLSTYPE = 0 as *mut GMLSTYPE;
    let mut yyvsp: *mut GMLSTYPE = 0 as *mut GMLSTYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: GMLSTYPE = GMLSTYPE { i: 0 };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    gmlnerrs = 0 as libc::c_int;
    gmlchar = -(2 as libc::c_int);
    'c_8702: loop {
        *yyssp = yystate as yytype_int16;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_ulong = (yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong;
            if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                current_block = 16729921059818448615;
                break;
            }
            yystacksize = yystacksize.wrapping_mul(2 as libc::c_int as libc::c_ulong);
            if (10000 as libc::c_int as libc::c_ulong) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_ulong;
            }
            let mut yyss1: *mut yytype_int16 = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                yystacksize
                    .wrapping_mul(
                        (::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<GMLSTYPE>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 16729921059818448615;
                break;
            }
            let mut yynewbytes: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yytype_int16 as *mut libc::c_void,
                yyss as *const libc::c_void,
                yysize
                    .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            let mut yynewbytes_0: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut GMLSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<GMLSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                .wrapping_mul(::std::mem::size_of::<GMLSTYPE>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes_0
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 8978805484006108354;
                break;
            }
        }
        if yystate == 55 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 7457128069125143372;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(29 as libc::c_int) {
                current_block = 14871405356348849564;
            } else {
                if gmlchar == -(2 as libc::c_int) {
                    gmlchar = gmllex();
                }
                if gmlchar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    gmlchar = yytoken;
                } else {
                    yytoken = if gmlchar as libc::c_uint
                        <= 289 as libc::c_int as libc::c_uint
                    {
                        yytranslate[gmlchar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (226 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 14871405356348849564;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 17903698783913276344;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                        }
                        gmlchar = -(2 as libc::c_int);
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = gmllval;
                        current_block = 6980673265629891554;
                    }
                }
            }
            match current_block {
                14871405356348849564 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if gmlchar == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if gmlchar as libc::c_uint
                                <= 289 as libc::c_int as libc::c_uint
                            {
                            yytranslate[gmlchar as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            gmlnerrs += 1;
                            gmlerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if gmlchar <= 0 as libc::c_int {
                                if gmlchar == 0 as libc::c_int {
                                    current_block = 8978805484006108354;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut gmllval,
                                );
                                gmlchar = -(2 as libc::c_int);
                            }
                        }
                        yyerrstatus = 3 as libc::c_int;
                        loop {
                            yyn = yypact[yystate as usize] as libc::c_int;
                            if !(yyn == -(29 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn && yyn <= 226 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 8978805484006108354;
                                break 'c_8702;
                            }
                            yydestruct(
                                b"Error: popping\0" as *const u8 as *const libc::c_char,
                                yystos[yystate as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            yystate = *yyssp as libc::c_int;
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = gmllval;
                        yystate = yyn;
                        current_block = 6980673265629891554;
                    } else {
                        current_block = 17903698783913276344;
                    }
                }
                _ => {}
            }
            match current_block {
                17903698783913276344 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            gmllexeof();
                            if !((*G).parent).is_null() {
                                popG();
                            }
                        }
                        3 => {
                            cleanup();
                            current_block = 8978805484006108354;
                            break;
                        }
                        5 => {
                            pushG();
                        }
                        11 => {
                            (Some(
                                ((*(*G).nodelist).searchf)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*G).nodelist,
                                (*yyvsp.offset(0 as libc::c_int as isize)).np
                                    as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        12 => {
                            (Some(
                                ((*(*G).edgelist).searchf)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*G).edgelist,
                                (*yyvsp.offset(0 as libc::c_int as isize)).ep
                                    as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        14 => {
                            if setDir((*yyvsp.offset(0 as libc::c_int as isize)).str_0)
                                != 0
                            {
                                gmlerror(
                                    b"mixed directed and undirected graphs\0" as *const u8
                                        as *const libc::c_char,
                                );
                                cleanup();
                                current_block = 8978805484006108354;
                                break;
                            }
                        }
                        15 => {
                            (Some(
                                ((*(*G).attrlist).searchf)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*G).attrlist,
                                mkAttr(
                                    strdup(b"id\0" as *const u8 as *const libc::c_char),
                                    0 as libc::c_int as libc::c_ushort,
                                    284 as libc::c_int as libc::c_ushort,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                    0 as *mut Dt_t,
                                ) as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        16 => {
                            (Some(
                                ((*(*G).attrlist).searchf)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*G).attrlist,
                                (*yyvsp.offset(0 as libc::c_int as isize)).ap
                                    as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        17 => {
                            N = mkNode();
                        }
                        18 => {
                            yyval.np = N;
                            N = 0 as *mut gmlnode;
                        }
                        21 => {
                            let ref mut fresh30 = (*N).id;
                            *fresh30 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        22 => {
                            (Some(
                                ((*(*N).attrlist).searchf)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*N).attrlist,
                                (*yyvsp.offset(0 as libc::c_int as isize)).ap
                                    as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        23 => {
                            E = mkEdge();
                        }
                        24 => {
                            yyval.ep = E;
                            E = 0 as *mut gmledge;
                        }
                        27 => {
                            let ref mut fresh31 = (*E).source;
                            *fresh31 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        28 => {
                            let ref mut fresh32 = (*E).target;
                            *fresh32 = (*yyvsp.offset(0 as libc::c_int as isize)).str_0;
                        }
                        29 => {
                            (Some(
                                ((*(*E).attrlist).searchf)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*E).attrlist,
                                mkAttr(
                                    strdup(b"id\0" as *const u8 as *const libc::c_char),
                                    0 as libc::c_int as libc::c_ushort,
                                    284 as libc::c_int as libc::c_ushort,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                    0 as *mut Dt_t,
                                ) as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        30 => {
                            (Some(
                                ((*(*E).attrlist).searchf)
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*E).attrlist,
                                (*yyvsp.offset(0 as libc::c_int as isize)).ap
                                    as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        31 => {
                            pushAlist();
                        }
                        32 => {
                            yyval.list = popAlist();
                        }
                        35 => {
                            (Some(((*L).searchf).expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                L,
                                (*yyvsp.offset(0 as libc::c_int as isize)).ap
                                    as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        36 => {
                            (Some(((*L).searchf).expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                L,
                                (*yyvsp.offset(0 as libc::c_int as isize)).ap
                                    as *mut libc::c_void,
                                0o1 as libc::c_int,
                            );
                        }
                        37 => {
                            yyval
                                .ap = mkAttr(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).str_0,
                                0 as libc::c_int as libc::c_ushort,
                                284 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        38 => {
                            yyval
                                .ap = mkAttr(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).str_0,
                                0 as libc::c_int as libc::c_ushort,
                                285 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        39 => {
                            yyval
                                .ap = mkAttr(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).str_0,
                                0 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        40 => {
                            yyval
                                .ap = mkAttr(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).str_0,
                                0 as libc::c_int as libc::c_ushort,
                                289 as libc::c_int as libc::c_ushort,
                                0 as *mut libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).list,
                            );
                        }
                        41 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                264 as libc::c_int as libc::c_ushort,
                                285 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        42 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                264 as libc::c_int as libc::c_ushort,
                                285 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        43 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                265 as libc::c_int as libc::c_ushort,
                                285 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        44 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                266 as libc::c_int as libc::c_ushort,
                                285 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        45 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                267 as libc::c_int as libc::c_ushort,
                                285 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        46 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                268 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        47 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                269 as libc::c_int as libc::c_ushort,
                                289 as libc::c_int as libc::c_ushort,
                                0 as *mut libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).list,
                            );
                        }
                        48 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                270 as libc::c_int as libc::c_ushort,
                                289 as libc::c_int as libc::c_ushort,
                                0 as *mut libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).list,
                            );
                        }
                        49 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                271 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        50 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                272 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        51 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                273 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        52 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                274 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        53 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                275 as libc::c_int as libc::c_ushort,
                                284 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        54 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                276 as libc::c_int as libc::c_ushort,
                                285 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        55 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                276 as libc::c_int as libc::c_ushort,
                                284 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        56 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                277 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        57 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                277 as libc::c_int as libc::c_ushort,
                                289 as libc::c_int as libc::c_ushort,
                                0 as *mut libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).list,
                            );
                        }
                        58 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                278 as libc::c_int as libc::c_ushort,
                                289 as libc::c_int as libc::c_ushort,
                                0 as *mut libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).list,
                            );
                        }
                        59 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                279 as libc::c_int as libc::c_ushort,
                                289 as libc::c_int as libc::c_ushort,
                                0 as *mut libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).list,
                            );
                        }
                        60 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                280 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        61 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                282 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        62 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                282 as libc::c_int as libc::c_ushort,
                                284 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        63 => {
                            yyval
                                .ap = mkAttr(
                                0 as *mut libc::c_char,
                                283 as libc::c_int as libc::c_ushort,
                                286 as libc::c_int as libc::c_ushort,
                                (*yyvsp.offset(0 as libc::c_int as isize)).str_0,
                                0 as *mut Dt_t,
                            );
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    yyn = yyr1[yyn as usize] as libc::c_int;
                    yystate = yypgoto[(yyn - 37 as libc::c_int) as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    if 0 as libc::c_int <= yystate && yystate <= 226 as libc::c_int
                        && yycheck[yystate as usize] as libc::c_int
                            == *yyssp as libc::c_int
                    {
                        yystate = yytable[yystate as usize] as libc::c_int;
                    } else {
                        yystate = yydefgoto[(yyn - 37 as libc::c_int) as usize]
                            as libc::c_int;
                    }
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
        }
    }
    match current_block {
        16729921059818448615 => {
            gmlerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        8978805484006108354 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if gmlchar != -(2 as libc::c_int) {
        yytoken = if gmlchar as libc::c_uint <= 289 as libc::c_int as libc::c_uint {
            yytranslate[gmlchar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut gmllval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
