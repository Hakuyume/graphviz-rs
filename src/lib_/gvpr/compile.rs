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
#![feature(label_break_value, register_tool)]
extern "C" {
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sfopen(_: *mut Sfio_t, _: *const libc::c_char, _: *const libc::c_char) -> *mut Sfio_t;
    fn sfstack(_: *mut Sfio_t, _: *mut Sfio_t) -> *mut Sfio_t;
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfseek(_: *mut Sfio_t, _: libc::c_longlong, _: libc::c_int) -> libc::c_longlong;
    fn sfputr(_: *mut Sfio_t, _: *const libc::c_char, _: libc::c_int) -> ssize_t;
    fn sfprintf(_: *mut Sfio_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut AgMemDisc: Agmemdisc_t;
    static mut AgIdDisc: Agiddisc_t;
    fn agopen(name: *mut libc::c_char, desc: Agdesc_t, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agisstrict(g: *mut Agraph_t) -> libc::c_int;
    fn agnode(g: *mut Agraph_t, name: *mut libc::c_char, createflag: libc::c_int) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agcontains(_: *mut Agraph_t, _: *mut libc::c_void) -> libc::c_int;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agobjkind(_: *mut libc::c_void) -> libc::c_int;
    fn aghtmlstr(_: *const libc::c_char) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agattrsym(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut Agsym_t;
    fn agnxtattr(g: *mut Agraph_t, kind: libc::c_int, attr: *mut Agsym_t) -> *mut Agsym_t;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agxset(obj: *mut libc::c_void, sym: *mut Agsym_t, value: *const libc::c_char)
        -> libc::c_int;
    fn agsubg(g: *mut Agraph_t, name: *mut libc::c_char, cflag: libc::c_int) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn agnedges(g: *mut Agraph_t) -> libc::c_int;
    fn agdegree(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        in_0: libc::c_int,
        out: libc::c_int,
    ) -> libc::c_int;
    fn strgrpmatch(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn vmfree(vm: *mut Vmalloc_t, data: *mut libc::c_void);
    fn vmstrdup(_: *mut Vmalloc_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn exnoncast(_: *mut Exnode_t) -> *mut Exnode_t;
    fn exclose(_: *mut Expr_t, _: libc::c_int);
    fn excomp(
        _: *mut Expr_t,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: *mut Sfio_t,
    ) -> libc::c_int;
    fn exdump(_: *mut Expr_t, _: *mut Exnode_t, _: *mut Sfio_t) -> libc::c_int;
    fn exerror(_: *const libc::c_char, _: ...);
    fn exexpr(
        _: *mut Expr_t,
        _: *const libc::c_char,
        _: *mut Exid_t,
        _: libc::c_int,
    ) -> *mut Exnode_t;
    fn exopen(_: *mut Exdisc_t) -> *mut Expr_t;
    fn exstring(_: *mut Expr_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn exstralloc(_: *mut Expr_t, _: size_t) -> *mut libc::c_void;
    fn exzero(_: libc::c_int) -> Extype_t;
    fn exinit();
    fn exisAssign(_: *mut Exnode_t) -> libc::c_int;
    fn findBinding(state: *mut Gpr_t, _: *mut libc::c_char) -> *mut gvprbinding;
    fn validTVT(_: libc::c_int) -> libc::c_int;
    static mut Agdirected: Agdesc_t;
    fn setErrorLine(_: libc::c_int);
    fn setErrorFileLine(_: *mut libc::c_char, _: libc::c_int);
    fn getErrorErrors() -> libc::c_int;
    fn _err_msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn errorf(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    );
    fn nodeInduce(selected: *mut Agraph_t);
    fn clone(g: *mut Agraph_t, obj: *mut Agobj_t) -> *mut Agobj_t;
    fn cloneG(g: *mut Agraph_t, name: *mut libc::c_char) -> *mut Agraph_t;
    fn copy(g: *mut Agraph_t, obj: *mut Agobj_t) -> *mut Agobj_t;
    fn copyAttr(obj: *mut Agobj_t, obj1: *mut Agobj_t) -> libc::c_int;
    fn indexOf(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn rindexOf(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    #[link_name = "match"]
    fn match_0(str: *mut libc::c_char, pat: *mut libc::c_char) -> libc::c_int;
    fn lockGraph(g: *mut Agraph_t, _: libc::c_int) -> libc::c_int;
    fn compOf(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agraph_t;
    fn isEdge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        key: *mut libc::c_char,
    ) -> *mut Agedge_t;
    fn addNode(g: *mut Agraph_t, n: *mut Agnode_t, doAdd: libc::c_int) -> *mut Agnode_t;
    fn addEdge(g: *mut Agraph_t, e: *mut Agedge_t, doAdd: libc::c_int) -> *mut Agedge_t;
    fn sameG(
        p1: *mut libc::c_void,
        p2: *mut libc::c_void,
        fn_0: *mut libc::c_char,
        msg: *mut libc::c_char,
    ) -> *mut Agraph_t;
    fn compare(_: *mut Agobj_t, _: *mut Agobj_t) -> libc::c_int;
    fn sfioWrite(_: *mut Agraph_t, _: *mut Sfio_t, _: *mut Agiodisc_t) -> libc::c_int;
    fn writeFile(_: *mut Agraph_t, _: *mut libc::c_char, _: *mut Agiodisc_t) -> libc::c_int;
    fn fwriteFile(
        _: *mut Expr_t,
        _: *mut Agraph_t,
        _: libc::c_int,
        _: *mut Agiodisc_t,
    ) -> libc::c_int;
    fn readFile(_: *mut libc::c_char) -> *mut Agraph_t;
    fn freadFile(_: *mut Expr_t, _: libc::c_int) -> *mut Agraph_t;
    fn openFile(_: *mut Expr_t, _: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn closeFile(_: *mut Expr_t, _: libc::c_int) -> libc::c_int;
    fn readLine(_: *mut Expr_t, _: libc::c_int) -> *mut libc::c_char;
    fn canon(pgm: *mut Expr_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn toHtml(_: *mut Agraph_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn toLower(pgm: *mut Expr_t, _: *mut libc::c_char, _: *mut Sfio_t) -> *mut libc::c_char;
    fn toUpper(pgm: *mut Expr_t, _: *mut libc::c_char, _: *mut Sfio_t) -> *mut libc::c_char;
    fn deleteObj(g: *mut Agraph_t, obj: *mut Agobj_t) -> libc::c_int;
    fn colorx(
        ex: *mut Expr_t,
        incolor: *mut libc::c_char,
        fmt: *mut libc::c_char,
        fp: *mut Sfio_t,
    ) -> *mut libc::c_char;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type intmax_t = __intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfio_s {
    pub next: *mut libc::c_uchar,
    pub endw: *mut libc::c_uchar,
    pub endr: *mut libc::c_uchar,
    pub endb: *mut libc::c_uchar,
    pub push: *mut Sfio_t,
    pub flags: libc::c_ushort,
    pub file: libc::c_short,
    pub data: *mut libc::c_uchar,
    pub size: ssize_t,
    pub val: ssize_t,
}
pub type Sfio_t = _sfio_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _case_info {
    pub gstart: libc::c_int,
    pub guard: *mut libc::c_char,
    pub astart: libc::c_int,
    pub action: *mut libc::c_char,
    pub next: *mut _case_info,
}
pub type case_info = _case_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _parse_block {
    pub l_beging: libc::c_int,
    pub begg_stmt: *mut libc::c_char,
    pub n_nstmts: libc::c_int,
    pub n_estmts: libc::c_int,
    pub node_stmts: *mut case_info,
    pub edge_stmts: *mut case_info,
    pub next: *mut _parse_block,
}
pub type parse_block = _parse_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_prog {
    pub source: *mut libc::c_char,
    pub l_begin: libc::c_int,
    pub l_end: libc::c_int,
    pub l_endg: libc::c_int,
    pub begin_stmt: *mut libc::c_char,
    pub n_blocks: libc::c_int,
    pub blocks: *mut parse_block,
    pub endg_stmt: *mut libc::c_char,
    pub end_stmt: *mut libc::c_char,
}
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
pub type Dtevent_f = Option<
    unsafe extern "C" fn(*mut Dt_t, libc::c_int, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_int,
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
pub type Dtsearch_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void>;
pub type Dtmemory_f = Option<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, size_t, *mut Dtdisc_t) -> *mut libc::c_void,
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
pub type agobjfn_t =
    Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> ()>;
pub type Agraph_t = Agraph_s;
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
pub struct _vmalloc_s {
    pub allocated: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub type Vmalloc_t = _vmalloc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union EX_STYPE {
    pub expr: *mut Exnode_s,
    pub floating: libc::c_double,
    pub reference: *mut Exref_s,
    pub id: *mut Exid_s,
    pub integer: libc::c_longlong,
    pub op: libc::c_int,
    pub string: *mut libc::c_char,
    pub user: *mut libc::c_void,
    pub buffer: *mut Exbuf_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exbuf_s {
    pub size: uint64_t,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exid_s {
    pub link: Dtlink_t,
    pub lex: libc::c_long,
    pub index: libc::c_long,
    pub type_0: libc::c_long,
    pub index_type: libc::c_long,
    pub flags: libc::c_long,
    pub value: *mut Exnode_t,
    pub local: Exlocal_t,
    pub isstatic: libc::c_long,
    pub name: [libc::c_char; 32],
}
pub type Exlocal_t = Exlocal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exlocal_s {
    pub number: libc::c_longlong,
    pub pointer: *mut libc::c_char,
}
pub type Exnode_t = Exnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exnode_s {
    pub type_0: libc::c_int,
    pub op: libc::c_int,
    pub binary: libc::c_int,
    pub local: Exlocal_t,
    pub compiled: C2RustUnnamed_6,
    pub data: Exdata_t,
}
pub type Exdata_t = Exdata_u;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Exdata_u {
    pub constant: C2RustUnnamed_5,
    pub operand: C2RustUnnamed_4,
    pub select: C2RustUnnamed_3,
    pub variable: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub symbol: *mut Exid_t,
    pub reference: *mut Exref_t,
    pub index: *mut Exnode_t,
    pub dyna: *mut Exnode_t,
}
pub type Exref_t = Exref_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exref_s {
    pub next: *mut Exref_t,
    pub symbol: *mut Exid_t,
    pub index: *mut Exnode_t,
}
pub type Exid_t = Exid_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub statement: *mut Exnode_t,
    pub next: *mut Exnode_t,
    pub constant: *mut *mut Extype_t,
}
pub type Extype_t = EX_STYPE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub left: *mut Exnode_t,
    pub right: *mut Exnode_t,
    pub last: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub value: Extype_t,
    pub reference: *mut Exid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub floating: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_double>,
    pub integer: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_longlong>,
    pub string: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> *mut libc::c_char>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exdisc_s {
    pub version: uint64_t,
    pub flags: uint64_t,
    pub symbols: *mut Exid_t,
    pub data: *mut *mut libc::c_char,
    pub lib: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
    pub castf: Option<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *const libc::c_char,
            libc::c_int,
            *mut Exid_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub convertf: Option<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exid_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub binaryf: Option<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exnode_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub typename: Option<unsafe extern "C" fn(*mut Expr_t, libc::c_int) -> *mut libc::c_char>,
    pub stringof: Option<
        unsafe extern "C" fn(*mut Expr_t, *mut Exnode_t, libc::c_int, *mut Exdisc_t) -> libc::c_int,
    >,
    pub keyf:
        Option<unsafe extern "C" fn(*mut Expr_t, Extype_t, libc::c_int, *mut Exdisc_t) -> Extype_t>,
    pub errorf: Exerror_f,
    pub getf: Option<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exid_t,
            *mut Exref_t,
            *mut libc::c_void,
            libc::c_int,
            *mut Exdisc_t,
        ) -> Extype_t,
    >,
    pub reff: Option<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exid_t,
            *mut Exref_t,
            *mut libc::c_char,
            libc::c_int,
            *mut Exdisc_t,
        ) -> Extype_t,
    >,
    pub setf: Option<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exid_t,
            *mut Exref_t,
            *mut libc::c_void,
            libc::c_int,
            Extype_t,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub matchf: Option<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *const libc::c_char,
            *mut Exnode_t,
            *const libc::c_char,
            *mut libc::c_void,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub exitf: Exexit_f,
    pub types: *mut libc::c_int,
    pub user: *mut libc::c_void,
}
pub type Exexit_f = Option<unsafe extern "C" fn(*mut Expr_t, *mut Exdisc_t, libc::c_int) -> ()>;
pub type Exdisc_t = Exdisc_s;
pub type Expr_t = Expr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr_s {
    pub id: *const libc::c_char,
    pub symbols: *mut Dt_t,
    pub more: *const libc::c_char,
    pub file: [*mut Sfio_t; 10],
    pub vm: *mut Vmalloc_t,
}
pub type Exerror_f = Option<
    unsafe extern "C" fn(
        *mut Expr_t,
        *mut Exdisc_t,
        libc::c_int,
        *const libc::c_char,
        ...
    ) -> libc::c_int,
>;
pub type gvpruserfn = Option<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvprbinding {
    pub name: *mut libc::c_char,
    pub fn_0: gvpruserfn,
}
pub type trav_type = libc::c_uint;
pub const TV_prepostrev: trav_type = 12;
pub const TV_prepostfwd: trav_type = 11;
pub const TV_prepostdfs: trav_type = 10;
pub const TV_postrev: trav_type = 9;
pub const TV_postfwd: trav_type = 8;
pub const TV_postdfs: trav_type = 7;
pub const TV_rev: trav_type = 6;
pub const TV_fwd: trav_type = 5;
pub const TV_dfs: trav_type = 4;
pub const TV_bfs: trav_type = 3;
pub const TV_en: trav_type = 2;
pub const TV_ne: trav_type = 1;
pub const TV_flat: trav_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gpr_t {
    pub curgraph: *mut Agraph_t,
    pub nextgraph: *mut Agraph_t,
    pub target: *mut Agraph_t,
    pub outgraph: *mut Agraph_t,
    pub curobj: *mut Agobj_t,
    pub tmp: *mut Sfio_t,
    pub dp: *mut Exdisc_t,
    pub errf: Exerror_f,
    pub exitf: Exexit_f,
    pub tgtname: *mut libc::c_char,
    pub infname: *mut libc::c_char,
    pub outFile: *mut Sfio_t,
    pub dfltIO: *mut Agiodisc_t,
    pub tvt: trav_type,
    pub tvroot: *mut Agnode_t,
    pub tvnext: *mut Agnode_t,
    pub tvedge: *mut Agedge_t,
    pub name_used: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub flags: libc::c_int,
    pub bindings: *mut gvprbinding,
    pub n_bindings: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_stmt {
    pub guard: *mut Exnode_t,
    pub action: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nval_t {
    pub h: Agrec_t,
    pub iu: Extype_t,
    pub ine: *mut Agedge_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gval_t {
    pub h: Agrec_t,
    pub lock: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uval_t {
    pub h: Agrec_t,
}
pub type ndata = nval_t;
pub type edata = uval_t;
pub type gdata = gval_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_block {
    pub begg_stmt: *mut Exnode_t,
    pub walks: libc::c_int,
    pub n_nstmts: libc::c_int,
    pub n_estmts: libc::c_int,
    pub node_stmts: *mut case_stmt,
    pub edge_stmts: *mut case_stmt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comp_prog {
    pub flags: libc::c_int,
    pub prog: *mut Expr_t,
    pub begin_stmt: *mut Exnode_t,
    pub n_blocks: libc::c_int,
    pub blocks: *mut comp_block,
    pub endg_stmt: *mut Exnode_t,
    pub end_stmt: *mut Exnode_t,
}
pub const T_obj: C2RustUnnamed_8 = 30;
pub const C_null: C2RustUnnamed_8 = 138;
pub const T_tvtyp: C2RustUnnamed_8 = 31;
pub const C_prepostrev: C2RustUnnamed_8 = 137;
pub const C_prepostfwd: C2RustUnnamed_8 = 136;
pub const C_prepostdfs: C2RustUnnamed_8 = 135;
pub const C_postrev: C2RustUnnamed_8 = 134;
pub const C_postfwd: C2RustUnnamed_8 = 133;
pub const C_postdfs: C2RustUnnamed_8 = 132;
pub const C_rev: C2RustUnnamed_8 = 131;
pub const C_fwd: C2RustUnnamed_8 = 130;
pub const C_dfs: C2RustUnnamed_8 = 129;
pub const C_bfs: C2RustUnnamed_8 = 128;
pub const C_en: C2RustUnnamed_8 = 127;
pub const C_ne: C2RustUnnamed_8 = 126;
pub const C_flat: C2RustUnnamed_8 = 125;
pub const F_call: C2RustUnnamed_8 = 124;
pub const F_colorx: C2RustUnnamed_8 = 123;
pub const F_atof: C2RustUnnamed_8 = 122;
pub const F_atoi: C2RustUnnamed_8 = 121;
pub const F_strcmp: C2RustUnnamed_8 = 120;
pub const F_toupper: C2RustUnnamed_8 = 119;
pub const F_tolower: C2RustUnnamed_8 = 118;
pub const F_nxtattr: C2RustUnnamed_8 = 117;
pub const F_fstattr: C2RustUnnamed_8 = 116;
pub const F_isattr: C2RustUnnamed_8 = 115;
pub const F_hasattr: C2RustUnnamed_8 = 114;
pub const F_dset: C2RustUnnamed_8 = 113;
pub const F_dget: C2RustUnnamed_8 = 112;
pub const F_set: C2RustUnnamed_8 = 111;
pub const F_get: C2RustUnnamed_8 = 110;
pub const F_canon: C2RustUnnamed_8 = 109;
pub const F_ishtml: C2RustUnnamed_8 = 108;
pub const F_html: C2RustUnnamed_8 = 107;
pub const F_urof: C2RustUnnamed_8 = 106;
pub const F_llof: C2RustUnnamed_8 = 105;
pub const F_yof: C2RustUnnamed_8 = 104;
pub const F_xof: C2RustUnnamed_8 = 103;
pub const F_sys: C2RustUnnamed_8 = 102;
pub const F_max: C2RustUnnamed_8 = 101;
pub const F_min: C2RustUnnamed_8 = 100;
pub const F_log: C2RustUnnamed_8 = 99;
pub const F_pow: C2RustUnnamed_8 = 98;
pub const F_exp: C2RustUnnamed_8 = 97;
pub const F_atan2: C2RustUnnamed_8 = 96;
pub const F_sin: C2RustUnnamed_8 = 95;
pub const F_cos: C2RustUnnamed_8 = 94;
pub const F_sqrt: C2RustUnnamed_8 = 93;
pub const F_nedges: C2RustUnnamed_8 = 92;
pub const F_nnodes: C2RustUnnamed_8 = 91;
pub const F_lock: C2RustUnnamed_8 = 90;
pub const F_copya: C2RustUnnamed_8 = 89;
pub const F_copy: C2RustUnnamed_8 = 88;
pub const F_cloneG: C2RustUnnamed_8 = 87;
pub const F_clone: C2RustUnnamed_8 = 86;
pub const F_delete: C2RustUnnamed_8 = 85;
pub const F_isstrict: C2RustUnnamed_8 = 84;
pub const F_isdirect: C2RustUnnamed_8 = 83;
pub const F_induce: C2RustUnnamed_8 = 82;
pub const F_readl: C2RustUnnamed_8 = 81;
pub const F_closef: C2RustUnnamed_8 = 80;
pub const F_openf: C2RustUnnamed_8 = 79;
pub const F_freadg: C2RustUnnamed_8 = 78;
pub const F_fwriteg: C2RustUnnamed_8 = 77;
pub const F_readg: C2RustUnnamed_8 = 76;
pub const F_writeg: C2RustUnnamed_8 = 75;
pub const F_write: C2RustUnnamed_8 = 74;
pub const F_match: C2RustUnnamed_8 = 73;
pub const F_length: C2RustUnnamed_8 = 72;
pub const F_issubedge: C2RustUnnamed_8 = 71;
pub const F_isedgesg: C2RustUnnamed_8 = 70;
pub const F_isedge: C2RustUnnamed_8 = 69;
pub const F_rindex: C2RustUnnamed_8 = 68;
pub const F_index: C2RustUnnamed_8 = 67;
pub const F_kindof: C2RustUnnamed_8 = 66;
pub const F_compof: C2RustUnnamed_8 = 65;
pub const F_nxtedgesg: C2RustUnnamed_8 = 64;
pub const F_fstedgesg: C2RustUnnamed_8 = 63;
pub const F_nxtinsg: C2RustUnnamed_8 = 62;
pub const F_fstinsg: C2RustUnnamed_8 = 61;
pub const F_nxtoutsg: C2RustUnnamed_8 = 60;
pub const F_fstoutsg: C2RustUnnamed_8 = 59;
pub const F_nxtedge: C2RustUnnamed_8 = 58;
pub const F_fstedge: C2RustUnnamed_8 = 57;
pub const F_nxtin: C2RustUnnamed_8 = 56;
pub const F_fstin: C2RustUnnamed_8 = 55;
pub const F_nxtout: C2RustUnnamed_8 = 54;
pub const F_fstout: C2RustUnnamed_8 = 53;
pub const F_opp: C2RustUnnamed_8 = 52;
pub const F_addedge: C2RustUnnamed_8 = 51;
pub const F_edgesg: C2RustUnnamed_8 = 50;
pub const F_edge: C2RustUnnamed_8 = 49;
pub const F_isin: C2RustUnnamed_8 = 48;
pub const F_degree: C2RustUnnamed_8 = 47;
pub const F_outdegree: C2RustUnnamed_8 = 46;
pub const F_indegree: C2RustUnnamed_8 = 45;
pub const F_issubnode: C2RustUnnamed_8 = 44;
pub const F_isnode: C2RustUnnamed_8 = 43;
pub const F_nxtnodesg: C2RustUnnamed_8 = 42;
pub const F_nxtnode: C2RustUnnamed_8 = 41;
pub const F_fstnode: C2RustUnnamed_8 = 40;
pub const F_addnode: C2RustUnnamed_8 = 39;
pub const F_node: C2RustUnnamed_8 = 38;
pub const F_nxtsubg: C2RustUnnamed_8 = 37;
pub const F_fstsubg: C2RustUnnamed_8 = 36;
pub const F_issubg: C2RustUnnamed_8 = 35;
pub const F_subg: C2RustUnnamed_8 = 34;
pub const F_graph: C2RustUnnamed_8 = 33;
pub const A_ARGV: C2RustUnnamed_8 = 32;
pub const T_graph: C2RustUnnamed_8 = 29;
pub const T_edge: C2RustUnnamed_8 = 28;
pub const T_node: C2RustUnnamed_8 = 27;
pub const M_strict: C2RustUnnamed_8 = 26;
pub const M_directed: C2RustUnnamed_8 = 25;
pub const M_n_nodes: C2RustUnnamed_8 = 24;
pub const M_n_edges: C2RustUnnamed_8 = 23;
pub const M_parent: C2RustUnnamed_8 = 22;
pub const M_root: C2RustUnnamed_8 = 21;
pub const M_outdegree: C2RustUnnamed_8 = 20;
pub const M_indegree: C2RustUnnamed_8 = 19;
pub const M_name: C2RustUnnamed_8 = 18;
pub const M_tail: C2RustUnnamed_8 = 17;
pub const M_head: C2RustUnnamed_8 = 16;
pub const M_Y: C2RustUnnamed_8 = 15;
pub const M_X: C2RustUnnamed_8 = 14;
pub const M_degree: C2RustUnnamed_8 = 13;
pub const V_ARGC: C2RustUnnamed_8 = 12;
pub const V_travtype: C2RustUnnamed_8 = 11;
pub const V_travedge: C2RustUnnamed_8 = 10;
pub const V_travnext: C2RustUnnamed_8 = 9;
pub const V_travroot: C2RustUnnamed_8 = 8;
pub const V_infname: C2RustUnnamed_8 = 7;
pub const V_tgtname: C2RustUnnamed_8 = 6;
pub const V_outgraph: C2RustUnnamed_8 = 5;
pub const V_targt: C2RustUnnamed_8 = 4;
pub const V_nextg: C2RustUnnamed_8 = 3;
pub const V_thisg: C2RustUnnamed_8 = 2;
pub const V_this: C2RustUnnamed_8 = 1;
pub type tctype = libc::c_ushort;
pub const CHARS_FOR_NUL_TERM_INT: C2RustUnnamed_7 = 12;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrieState {
    pub def: libc::c_short,
    pub trans_base: libc::c_short,
    pub mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrieTrans {
    pub c: libc::c_short,
    pub next_state: libc::c_short,
}
pub const _ISlower: C2RustUnnamed_13 = 512;
pub const _ISupper: C2RustUnnamed_13 = 256;
pub const LAST_M: C2RustUnnamed_10 = 26;
pub const MAXNAME: C2RustUnnamed_12 = 138;
pub const MINNAME: C2RustUnnamed_11 = 1;
pub const LAST_V: C2RustUnnamed_9 = 12;
pub type C2RustUnnamed_7 = libc::c_uint;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const RESERVED: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub type C2RustUnnamed_10 = libc::c_uint;
pub type C2RustUnnamed_11 = libc::c_uint;
pub type C2RustUnnamed_12 = libc::c_uint;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_13 = 8;
pub const _ISpunct: C2RustUnnamed_13 = 4;
pub const _IScntrl: C2RustUnnamed_13 = 2;
pub const _ISblank: C2RustUnnamed_13 = 1;
pub const _ISgraph: C2RustUnnamed_13 = 32768;
pub const _ISprint: C2RustUnnamed_13 = 16384;
pub const _ISspace: C2RustUnnamed_13 = 8192;
pub const _ISxdigit: C2RustUnnamed_13 = 4096;
pub const _ISdigit: C2RustUnnamed_13 = 2048;
pub const _ISalpha: C2RustUnnamed_13 = 1024;
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
unsafe extern "C" fn isedge(mut obj: *mut Agobj_t) -> libc::c_int {
    return (((*obj).tag).objtype() as libc::c_int == 2 as libc::c_int
        || ((*obj).tag).objtype() as libc::c_int == 3 as libc::c_int) as libc::c_int;
}
static mut symbols: [Exid_t; 139] = unsafe {
    [
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_this as libc::c_int as libc::c_long,
                type_0: T_obj as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_thisg as libc::c_int as libc::c_long,
                type_0: T_graph as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$G\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_nextg as libc::c_int as libc::c_long,
                type_0: T_graph as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$NG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_targt as libc::c_int as libc::c_long,
                type_0: T_graph as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$T\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_outgraph as libc::c_int as libc::c_long,
                type_0: T_graph as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$O\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_tgtname as libc::c_int as libc::c_long,
                type_0: 263 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$tgtname\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_infname as libc::c_int as libc::c_long,
                type_0: 263 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_travroot as libc::c_int as libc::c_long,
                type_0: T_node as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$tvroot\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_travnext as libc::c_int as libc::c_long,
                type_0: T_node as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$tvnext\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_travedge as libc::c_int as libc::c_long,
                type_0: T_edge as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$tvedge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_travtype as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"$tvtype\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: V_ARGC as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"ARGC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_degree as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"degree\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_X as libc::c_int as libc::c_long,
                type_0: 262 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_Y as libc::c_int as libc::c_long,
                type_0: 262 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_head as libc::c_int as libc::c_long,
                type_0: T_node as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"head\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_tail as libc::c_int as libc::c_long,
                type_0: T_node as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"tail\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_name as libc::c_int as libc::c_long,
                type_0: 263 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"name\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_indegree as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"indegree\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_outdegree as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"outdegree\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_root as libc::c_int as libc::c_long,
                type_0: T_graph as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"root\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_parent as libc::c_int as libc::c_long,
                type_0: T_graph as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_n_edges as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"n_edges\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_n_nodes as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"n_nodes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_directed as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"directed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 283 as libc::c_int as libc::c_long,
                index: M_strict as libc::c_int as libc::c_long,
                type_0: 259 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"strict\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: T_node as libc::c_int as libc::c_long,
                type_0: T_node as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"node_t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: T_edge as libc::c_int as libc::c_long,
                type_0: T_edge as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"edge_t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: T_graph as libc::c_int as libc::c_long,
                type_0: T_graph as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"graph_t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: T_obj as libc::c_int as libc::c_long,
                type_0: T_obj as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"obj_t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: T_tvtyp as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"tvtype_t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 267 as libc::c_int as libc::c_long,
                index: A_ARGV as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o2 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"ARGV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_graph as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"graph\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_subg as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"subg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_issubg as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isSubg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstsubg as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstsubg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtsubg as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtsubg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_node as libc::c_int as libc::c_long,
                type_0: (0x4 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"node\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_addnode as libc::c_int as libc::c_long,
                type_0: (0x4 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"subnode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstnode as libc::c_int as libc::c_long,
                type_0: (0x4 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstnode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtnode as libc::c_int as libc::c_long,
                type_0: (0x4 as libc::c_int
                    | (0x4 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtnode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtnodesg as libc::c_int as libc::c_long,
                type_0: (0x4 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtnode_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_isnode as libc::c_int as libc::c_long,
                type_0: (0x4 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isNode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_issubnode as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isSubnode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_indegree as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"indegreeOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_outdegree as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"outdegreeOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_degree as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"degreeOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_isin as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x7 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isIn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_edge as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x4 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"edge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_edgesg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 4 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"edge_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_addedge as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x5 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"subedge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_opp as libc::c_int as libc::c_long,
                type_0: (0x4 as libc::c_int
                    | (0x5 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"opp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstout as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x4 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtout as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x5 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstin as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x4 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtin as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x5 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstedge as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x4 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstedge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtedge as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x5 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtedge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstoutsg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstout_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtoutsg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x5 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtout_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstinsg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstin_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtinsg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x5 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtin_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstedgesg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstedge_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtedgesg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x5 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtedge_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_compof as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"compOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_kindof as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0x7 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"kindOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_index as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"index\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_rindex as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"rindex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_isedge as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x4 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isEdge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_isedgesg as libc::c_int as libc::c_long,
                type_0: (0x5 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0x4 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 4 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isEdge_sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_issubedge as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x5 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isSubedge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_length as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_match as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_write as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"write\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_writeg as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"writeG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_readg as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"readG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fwriteg as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o2 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fwriteG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_freadg as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0o2 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"freadG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_openf as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"openF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_closef as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o2 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"closeF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_readl as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o2 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"readL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_induce as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"induce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_isdirect as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isDirect\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_isstrict as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isStrict\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_delete as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x7 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"delete\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_clone as libc::c_int as libc::c_long,
                type_0: (0x7 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x7 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"clone\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_cloneG as libc::c_int as libc::c_long,
                type_0: (0x6 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"cloneG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_copy as libc::c_int as libc::c_long,
                type_0: (0x7 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x7 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"copy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_copya as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x7 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0x7 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"copyA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_lock as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o2 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"lock\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nnodes as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nNodes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nedges as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nEdges\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_sqrt as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"sqrt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_cos as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"cos\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_sin as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"sin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_atan2 as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o1 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"atan2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_exp as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"exp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_pow as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o1 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"pow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_log as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"log\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_min as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o1 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"MIN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_max as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o1 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o1 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"MAX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_sys as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"system\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_xof as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"xOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_yof as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"yOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_llof as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"llOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_urof as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"urOf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_html as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"html\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_ishtml as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"ishtml\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_canon as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"canon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_get as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0x7 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"aget\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_set as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x7 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"aset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_dget as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"getDflt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_dset as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 4 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"setDflt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_hasattr as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x7 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"hasAttr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_isattr as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"isAttr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_fstattr as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"fstAttr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_nxtattr as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0x6 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 3 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"nxtAttr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_tolower as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"tolower\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_toupper as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"toupper\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_strcmp as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"strcmp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_atoi as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"atoi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_atof as libc::c_int as libc::c_long,
                type_0: (0o1 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"atof\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_colorx as libc::c_int as libc::c_long,
                type_0: (0o3 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"colorx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 279 as libc::c_int as libc::c_long,
                index: F_call as libc::c_int as libc::c_long,
                type_0: (0o2 as libc::c_int
                    | (0o3 as libc::c_int) << 1 as libc::c_int * 4 as libc::c_int
                    | (0o3 as libc::c_int) << 2 as libc::c_int * 4 as libc::c_int)
                    as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_flat as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_flat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_ne as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_ne\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_en as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_en\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_bfs as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_bfs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_dfs as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_dfs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_fwd as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_fwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_rev as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_rev\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_postdfs as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_postdfs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_postfwd as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_postfwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_postrev as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_postrev\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_prepostdfs as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_prepostdfs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_prepostfwd as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_prepostfwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_prepostrev as libc::c_int as libc::c_long,
                type_0: T_tvtyp as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"TV_prepostrev\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 271 as libc::c_int as libc::c_long,
                index: C_null as libc::c_int as libc::c_long,
                type_0: T_obj as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"NULL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed { _hash: 0 },
                    };
                    init
                },
                lex: 0 as libc::c_int as libc::c_long,
                index: 0 as libc::c_int as libc::c_long,
                type_0: 0 as libc::c_int as libc::c_long,
                index_type: 0 as libc::c_int as libc::c_long,
                flags: 0 as libc::c_int as libc::c_long,
                value: 0 as *const Exnode_t as *mut Exnode_t,
                local: {
                    let mut init = Exlocal_s {
                        number: 0 as libc::c_int as libc::c_longlong,
                        pointer: 0 as *const libc::c_char as *mut libc::c_char,
                    };
                    init
                },
                isstatic: 0 as libc::c_int as libc::c_long,
                name: [
                    0 as libc::c_int as libc::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            };
            init
        },
    ]
};
static mut typenames: [*mut libc::c_char; 5] = [
    b"node_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"edge_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"graph_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"obj_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tvtype_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut tchk: [[tctype; 2]; 27] = [
    [0 as libc::c_int as tctype, 0 as libc::c_int as tctype],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x4 as libc::c_int
            | (1 as libc::c_int) << 0x5 as libc::c_int
            | (1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0o3 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0o3 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x5 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0x8 as libc::c_int) as tctype,
    ],
    [
        0 as libc::c_int as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o1 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o1 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x5 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x5 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x4 as libc::c_int
            | (1 as libc::c_int) << 0x5 as libc::c_int
            | (1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o3 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x4 as libc::c_int
            | (1 as libc::c_int) << 0x5 as libc::c_int
            | (1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
    [
        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype,
        ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype,
    ],
];
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut TrieStateTbl: [TrieState; 70] = [
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 0 as libc::c_int as libc::c_short,
            mask: 0x3b8620 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 9 as libc::c_int as libc::c_short,
            mask: 0x440 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 11 as libc::c_int as libc::c_short,
            mask: 0x100 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 12 as libc::c_int as libc::c_short,
            mask: 0x80000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 13 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 14 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x4 as libc::c_int) as libc::c_short,
            trans_base: 15 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 15 as libc::c_int as libc::c_short,
            mask: 0x80000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 16 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 17 as libc::c_int as libc::c_short,
            mask: 0x10 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 18 as libc::c_int as libc::c_short,
            mask: 0x200000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 19 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 20 as libc::c_int as libc::c_short,
            mask: 0x20 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x6 as libc::c_int) as libc::c_short,
            trans_base: 21 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 21 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 22 as libc::c_int as libc::c_short,
            mask: 0x4 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 23 as libc::c_int as libc::c_short,
            mask: 0x20 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x5 as libc::c_int) as libc::c_short,
            trans_base: 24 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 24 as libc::c_int as libc::c_short,
            mask: 0x8000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 25 as libc::c_int as libc::c_short,
            mask: 0x20 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 26 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 27 as libc::c_int as libc::c_short,
            mask: 0x100 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 28 as libc::c_int as libc::c_short,
            mask: 0x80000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 29 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 30 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x4 as libc::c_int) as libc::c_short,
            trans_base: 31 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 31 as libc::c_int as libc::c_short,
            mask: 0x5 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 33 as libc::c_int as libc::c_short,
            mask: 0x8040 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 35 as libc::c_int as libc::c_short,
            mask: 0x20 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 36 as libc::c_int as libc::c_short,
            mask: 0x100 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 37 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 38 as libc::c_int as libc::c_short,
            mask: 0x100000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x6 as libc::c_int) as libc::c_short,
            trans_base: 39 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 39 as libc::c_int as libc::c_short,
            mask: 0x10000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 40 as libc::c_int as libc::c_short,
            mask: 0x20 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 41 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 42 as libc::c_int as libc::c_short,
            mask: 0x100000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x6 as libc::c_int) as libc::c_short,
            trans_base: 43 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 43 as libc::c_int as libc::c_short,
            mask: 0x4000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 44 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x4 as libc::c_int
                | (1 as libc::c_int) << 0x5 as libc::c_int
                | (1 as libc::c_int) << 0x6 as libc::c_int) as libc::c_short,
            trans_base: 45 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 45 as libc::c_int as libc::c_short,
            mask: 0x400000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 46 as libc::c_int as libc::c_short,
            mask: 0x200000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 47 as libc::c_int as libc::c_short,
            mask: 0x20 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 48 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 49 as libc::c_int as libc::c_short,
            mask: 0x100 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 50 as libc::c_int as libc::c_short,
            mask: 0x80000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 51 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 52 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x4 as libc::c_int) as libc::c_short,
            trans_base: 53 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 53 as libc::c_int as libc::c_short,
            mask: 0x4 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 54 as libc::c_int as libc::c_short,
            mask: 0x80000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 55 as libc::c_int as libc::c_short,
            mask: 0x40 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 56 as libc::c_int as libc::c_short,
            mask: 0x8000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 57 as libc::c_int as libc::c_short,
            mask: 0x200000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x6 as libc::c_int) as libc::c_short,
            trans_base: 58 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 58 as libc::c_int as libc::c_short,
            mask: 0x10000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 59 as libc::c_int as libc::c_short,
            mask: 0x10000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 60 as libc::c_int as libc::c_short,
            mask: 0x200000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x4 as libc::c_int
                | (1 as libc::c_int) << 0x5 as libc::c_int
                | (1 as libc::c_int) << 0x6 as libc::c_int) as libc::c_short,
            trans_base: 61 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 61 as libc::c_int as libc::c_short,
            mask: 0x200000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 62 as libc::c_int as libc::c_short,
            mask: 0x80000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 63 as libc::c_int as libc::c_short,
            mask: 0x400 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 64 as libc::c_int as libc::c_short,
            mask: 0x10 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 65 as libc::c_int as libc::c_short,
            mask: 0x200000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x6 as libc::c_int) as libc::c_short,
            trans_base: 66 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 66 as libc::c_int as libc::c_short,
            mask: 0x4 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 67 as libc::c_int as libc::c_short,
            mask: 0x400 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: -(1 as libc::c_int) as libc::c_short,
            trans_base: 68 as libc::c_int as libc::c_short,
            mask: 0x2000 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = TrieState {
            def: ((1 as libc::c_int) << 0x5 as libc::c_int) as libc::c_short,
            trans_base: 69 as libc::c_int as libc::c_short,
            mask: 0 as libc::c_int as libc::c_long,
        };
        init
    },
];
static mut CharMask: [libc::c_long; 28] = [
    0x1 as libc::c_int as libc::c_long,
    0 as libc::c_int as libc::c_long,
    0x4 as libc::c_int as libc::c_long,
    0x8 as libc::c_int as libc::c_long,
    0x10 as libc::c_int as libc::c_long,
    0x20 as libc::c_int as libc::c_long,
    0x40 as libc::c_int as libc::c_long,
    0x80 as libc::c_int as libc::c_long,
    0x100 as libc::c_int as libc::c_long,
    0x200 as libc::c_int as libc::c_long,
    0x400 as libc::c_int as libc::c_long,
    0x800 as libc::c_int as libc::c_long,
    0x1000 as libc::c_int as libc::c_long,
    0x2000 as libc::c_int as libc::c_long,
    0x4000 as libc::c_int as libc::c_long,
    0x8000 as libc::c_int as libc::c_long,
    0x10000 as libc::c_int as libc::c_long,
    0x20000 as libc::c_int as libc::c_long,
    0x40000 as libc::c_int as libc::c_long,
    0x80000 as libc::c_int as libc::c_long,
    0x100000 as libc::c_int as libc::c_long,
    0x200000 as libc::c_int as libc::c_long,
    0x400000 as libc::c_int as libc::c_long,
    0x800000 as libc::c_int as libc::c_long,
    0x1000000 as libc::c_int as libc::c_long,
    0x2000000 as libc::c_int as libc::c_long,
    0x4000000 as libc::c_int as libc::c_long,
    0x8000000 as libc::c_int as libc::c_long,
];
static mut TFA_State: libc::c_short = 0;
#[no_mangle]
pub static mut TrieTransTbl: [TrieTrans; 69] = [
    {
        let mut init = TrieTrans {
            c: 'd' as i32 as libc::c_short,
            next_state: 1 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'h' as i32 as libc::c_short,
            next_state: 14 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'i' as i32 as libc::c_short,
            next_state: 18 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'n' as i32 as libc::c_short,
            next_state: 26 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'o' as i32 as libc::c_short,
            next_state: 41 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'p' as i32 as libc::c_short,
            next_state: 50 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'r' as i32 as libc::c_short,
            next_state: 56 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 's' as i32 as libc::c_short,
            next_state: 60 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 't' as i32 as libc::c_short,
            next_state: 66 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 2 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'i' as i32 as libc::c_short,
            next_state: 7 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'g' as i32 as libc::c_short,
            next_state: 3 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'r' as i32 as libc::c_short,
            next_state: 4 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 5 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 6 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'r' as i32 as libc::c_short,
            next_state: 8 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 9 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'c' as i32 as libc::c_short,
            next_state: 10 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 't' as i32 as libc::c_short,
            next_state: 11 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 12 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'd' as i32 as libc::c_short,
            next_state: 13 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 15 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'a' as i32 as libc::c_short,
            next_state: 16 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'd' as i32 as libc::c_short,
            next_state: 17 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'n' as i32 as libc::c_short,
            next_state: 19 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'd' as i32 as libc::c_short,
            next_state: 20 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 21 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'g' as i32 as libc::c_short,
            next_state: 22 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'r' as i32 as libc::c_short,
            next_state: 23 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 24 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 25 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: '_' as i32 as libc::c_short,
            next_state: 27 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'a' as i32 as libc::c_short,
            next_state: 38 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 28 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'n' as i32 as libc::c_short,
            next_state: 33 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'd' as i32 as libc::c_short,
            next_state: 29 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'g' as i32 as libc::c_short,
            next_state: 30 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 31 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 's' as i32 as libc::c_short,
            next_state: 32 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'o' as i32 as libc::c_short,
            next_state: 34 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'd' as i32 as libc::c_short,
            next_state: 35 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 36 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 's' as i32 as libc::c_short,
            next_state: 37 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'm' as i32 as libc::c_short,
            next_state: 39 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 40 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'u' as i32 as libc::c_short,
            next_state: 42 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 't' as i32 as libc::c_short,
            next_state: 43 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'd' as i32 as libc::c_short,
            next_state: 44 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 45 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'g' as i32 as libc::c_short,
            next_state: 46 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'r' as i32 as libc::c_short,
            next_state: 47 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 48 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 49 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'a' as i32 as libc::c_short,
            next_state: 51 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'r' as i32 as libc::c_short,
            next_state: 52 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'e' as i32 as libc::c_short,
            next_state: 53 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'n' as i32 as libc::c_short,
            next_state: 54 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 't' as i32 as libc::c_short,
            next_state: 55 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'o' as i32 as libc::c_short,
            next_state: 57 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'o' as i32 as libc::c_short,
            next_state: 58 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 't' as i32 as libc::c_short,
            next_state: 59 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 't' as i32 as libc::c_short,
            next_state: 61 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'r' as i32 as libc::c_short,
            next_state: 62 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'i' as i32 as libc::c_short,
            next_state: 63 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'c' as i32 as libc::c_short,
            next_state: 64 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 't' as i32 as libc::c_short,
            next_state: 65 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'a' as i32 as libc::c_short,
            next_state: 67 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'i' as i32 as libc::c_short,
            next_state: 68 as libc::c_int as libc::c_short,
        };
        init
    },
    {
        let mut init = TrieTrans {
            c: 'l' as i32 as libc::c_short,
            next_state: 69 as libc::c_int as libc::c_short,
        };
        init
    },
];
unsafe extern "C" fn int2ptr(mut i: libc::c_longlong) -> *mut libc::c_void {
    return i as intptr_t as *mut libc::c_void;
}
unsafe extern "C" fn ptr2int(mut p: *const libc::c_void) -> libc::c_longlong {
    return p as intptr_t as libc::c_longlong;
}
unsafe extern "C" fn iofread(
    mut chan: *mut libc::c_void,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    return read(
        if !chan.is_null() {
            (*(chan as *mut Sfio_t)).file as libc::c_int
        } else {
            -(1 as libc::c_int)
        },
        buf as *mut libc::c_void,
        bufsize as size_t,
    ) as libc::c_int;
}
unsafe extern "C" fn ioputstr(
    mut chan: *mut libc::c_void,
    mut str: *const libc::c_char,
) -> libc::c_int {
    return sfputr(chan as *mut Sfio_t, str, -(1 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn ioflush(mut chan: *mut libc::c_void) -> libc::c_int {
    return sfsync(chan as *mut Sfio_t);
}
static mut gprIoDisc: Agiodisc_t = unsafe {
    {
        let mut init = Agiodisc_s {
            afread: Some(
                iofread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            putstr: Some(
                ioputstr
                    as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
            ),
            flush: Some(ioflush as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        };
        init
    }
};
static mut gprDisc: Agdisc_t = unsafe {
    {
        let mut init = Agdisc_s {
            mem: &AgMemDisc as *const Agmemdisc_t as *mut Agmemdisc_t,
            id: &AgIdDisc as *const Agiddisc_t as *mut Agiddisc_t,
            io: &gprIoDisc as *const Agiodisc_t as *mut Agiodisc_t,
        };
        init
    }
};
unsafe extern "C" fn nameOf(
    mut ex: *mut Expr_t,
    mut obj: *mut Agobj_t,
    mut tmps: *mut Sfio_t,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    match ((*obj).tag).objtype() as libc::c_int {
        1 | 0 => {
            s = agnameof(obj as *mut libc::c_void);
        }
        _ => {
            e = obj as *mut Agedge_t;
            key = agnameof(
                (if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }) as *mut libc::c_void,
            );
            sfputr(
                tmps,
                agnameof(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                    .node as *mut libc::c_void,
                ),
                -(1 as libc::c_int),
            );
            if agisdirected(agraphof(e as *mut libc::c_void)) != 0 {
                sfputr(
                    tmps,
                    b"->\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            } else {
                sfputr(
                    tmps,
                    b"--\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            }
            sfputr(
                tmps,
                agnameof(
                    (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    })
                    .node as *mut libc::c_void,
                ),
                -(1 as libc::c_int),
            );
            if !key.is_null() && *key as libc::c_int != 0 {
                if (*tmps).next >= (*tmps).endw {
                    _sfflsbuf(tmps, '[' as i32 as libc::c_uchar as libc::c_int);
                } else {
                    let ref mut fresh0 = (*tmps).next;
                    let fresh1 = *fresh0;
                    *fresh0 = (*fresh0).offset(1);
                    *fresh1 = '[' as i32 as libc::c_uchar;
                };
                sfputr(tmps, key, -(1 as libc::c_int));
                if (*tmps).next >= (*tmps).endw {
                    _sfflsbuf(tmps, ']' as i32 as libc::c_uchar as libc::c_int);
                } else {
                    let ref mut fresh2 = (*tmps).next;
                    let fresh3 = *fresh2;
                    *fresh2 = (*fresh2).offset(1);
                    *fresh3 = ']' as i32 as libc::c_uchar;
                };
            }
            if (*tmps).next >= (*tmps).endw {
                _sfflsbuf(tmps, 0 as libc::c_int as libc::c_uchar as libc::c_int);
            } else {
                let ref mut fresh4 = (*tmps).next;
                let fresh5 = *fresh4;
                *fresh4 = (*fresh4).offset(1);
                *fresh5 = 0 as libc::c_int as libc::c_uchar;
            };
            let ref mut fresh6 = (*tmps).next;
            *fresh6 = (*tmps).data;
            s = exstring(ex, *fresh6 as *mut libc::c_char);
        }
    }
    return s;
}
unsafe extern "C" fn bbOf(
    mut pgm: *mut Expr_t,
    mut pt: *mut libc::c_char,
    mut getll: bool,
) -> *mut libc::c_char {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if sscanf(
        pt,
        b"%lf,%lf,%lf,%lf\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
        &mut u as *mut libc::c_double,
        &mut v as *mut libc::c_double,
    ) == 4 as libc::c_int
    {
        p = strchr(pt, ',' as i32);
        p = strchr(p.offset(1 as libc::c_int as isize), ',' as i32);
        if getll {
            let mut len: size_t = p.offset_from(pt) as libc::c_long as size_t;
            s = exstralloc(pgm, len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            strncpy(s, pt, len);
            *s.offset(len as isize) = '\0' as i32 as libc::c_char;
        } else {
            s = exstring(pgm, p.offset(1 as libc::c_int as isize));
        }
    } else {
        s = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return s;
}
unsafe extern "C" fn xyOf(
    mut pgm: *mut Expr_t,
    mut pt: *mut libc::c_char,
    mut getx: bool,
) -> *mut libc::c_char {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if sscanf(
        pt,
        b"%lf,%lf\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
        &mut y as *mut libc::c_double,
    ) == 2 as libc::c_int
    {
        p = strchr(pt, ',' as i32);
        if getx {
            let mut len: size_t = p.offset_from(pt) as libc::c_long as size_t;
            v = exstralloc(pgm, len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            strncpy(v, pt, len);
            *v.offset(len as isize) = '\0' as i32 as libc::c_char;
        } else {
            v = exstring(pgm, p.offset(1 as libc::c_int as isize));
        }
    } else {
        v = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return v;
}
unsafe extern "C" fn posOf(
    mut np: *mut Agnode_t,
    mut idx: libc::c_int,
    mut v: *mut libc::c_double,
) -> libc::c_int {
    static mut root: *mut Agraph_t = 0 as *const Agraph_t as *mut Agraph_t;
    static mut pos: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
    let mut nroot: *mut Agraph_t = agroot(np as *mut libc::c_void);
    let mut ps: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: [libc::c_double; 2] = [0.; 2];
    if root != nroot {
        root = nroot;
        pos = agattr(
            root,
            1 as libc::c_int,
            b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
    }
    if pos.is_null() {
        return -(1 as libc::c_int);
    }
    ps = agxget(np as *mut libc::c_void, pos);
    if sscanf(
        ps,
        b"%lf,%lf\0" as *const u8 as *const libc::c_char,
        &mut *p.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_double,
        &mut *p.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_double,
    ) == 2 as libc::c_int
    {
        *v = p[idx as usize];
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn xargs(mut args: *mut libc::c_char) -> Agdesc_t {
    let mut desc: Agdesc_t = Agdirected;
    let mut c: libc::c_char = 0;
    loop {
        let fresh7 = args;
        args = args.offset(1);
        c = *fresh7;
        if !(c != 0) {
            break;
        }
        match c as libc::c_int {
            117 | 85 => {
                desc.set_directed(0 as libc::c_int as libc::c_uint);
            }
            100 | 68 => {
                desc.set_directed(1 as libc::c_int as libc::c_uint);
            }
            115 | 83 => {
                desc.set_strict(1 as libc::c_int as libc::c_uint);
            }
            110 | 78 => {
                desc.set_directed(0 as libc::c_int as libc::c_uint);
            }
            _ => {
                _err_msg(
                    1 as libc::c_int,
                    b"unknown graph descriptor '%c' : ignored\0" as *const u8
                        as *const libc::c_char,
                    c as libc::c_int,
                );
            }
        }
    }
    return desc;
}
unsafe extern "C" fn deparse(
    mut ex: *mut Expr_t,
    mut n: *mut Exnode_t,
    mut sf: *mut Sfio_t,
) -> *mut libc::c_char {
    exdump(ex, n, sf);
    if (*sf).next >= (*sf).endw {
        _sfflsbuf(sf, 0 as libc::c_int as libc::c_uchar as libc::c_int);
    } else {
        let ref mut fresh8 = (*sf).next;
        let fresh9 = *fresh8;
        *fresh8 = (*fresh8).offset(1);
        *fresh9 = 0 as libc::c_int as libc::c_uchar;
    };
    let ref mut fresh10 = (*sf).next;
    *fresh10 = (*sf).data;
    return *fresh10 as *mut libc::c_char;
}
unsafe extern "C" fn deref(
    mut pgm: *mut Expr_t,
    mut x: *mut Exnode_t,
    mut ref_0: *mut Exref_t,
    mut objp: *mut Agobj_t,
    mut state: *mut Gpr_t,
) -> *mut Agobj_t {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if ref_0.is_null() {
        return objp;
    } else {
        if (*(*ref_0).symbol).lex == 275 as libc::c_int as libc::c_long {
            ptr = int2ptr(
                (*(*(*x).data.variable.dyna).data.variable.dyna)
                    .data
                    .constant
                    .value
                    .integer,
            );
            if ptr.is_null() {
                exerror(
                    b"null reference %s in expression %s.%s\0" as *const u8 as *const libc::c_char,
                    ((*(*ref_0).symbol).name).as_mut_ptr(),
                    ((*(*ref_0).symbol).name).as_mut_ptr(),
                    deparse(pgm, x, (*state).tmp),
                );
                return ptr as *mut Agobj_t;
            } else {
                return deref(pgm, x, (*ref_0).next, ptr as *mut Agobj_t, state);
            }
        } else {
            match (*(*ref_0).symbol).index {
                5 => {
                    return deref(
                        pgm,
                        x,
                        (*ref_0).next,
                        (*state).outgraph as *mut Agobj_t,
                        state,
                    );
                }
                1 => return deref(pgm, x, (*ref_0).next, (*state).curobj, state),
                2 => {
                    return deref(
                        pgm,
                        x,
                        (*ref_0).next,
                        (*state).curgraph as *mut Agobj_t,
                        state,
                    );
                }
                3 => {
                    return deref(
                        pgm,
                        x,
                        (*ref_0).next,
                        (*state).nextgraph as *mut Agobj_t,
                        state,
                    );
                }
                4 => {
                    return deref(
                        pgm,
                        x,
                        (*ref_0).next,
                        (*state).target as *mut Agobj_t,
                        state,
                    );
                }
                10 => {
                    return deref(
                        pgm,
                        x,
                        (*ref_0).next,
                        (*state).tvedge as *mut Agobj_t,
                        state,
                    );
                }
                8 => {
                    return deref(
                        pgm,
                        x,
                        (*ref_0).next,
                        (*state).tvroot as *mut Agobj_t,
                        state,
                    );
                }
                9 => {
                    return deref(
                        pgm,
                        x,
                        (*ref_0).next,
                        (*state).tvnext as *mut Agobj_t,
                        state,
                    );
                }
                16 => {
                    if objp.is_null() && {
                        objp = (*state).curobj;
                        objp.is_null()
                    } {
                        exerror(
                            b"Current object $ not defined\0" as *const u8 as *const libc::c_char,
                        );
                        return 0 as *mut Agobj_t;
                    }
                    if isedge(objp) != 0 {
                        return deref(
                            pgm,
                            x,
                            (*ref_0).next,
                            (*if ((*(objp as *mut Agedge_t as *mut Agobj_t)).tag).objtype()
                                as libc::c_int
                                == 2 as libc::c_int
                            {
                                objp as *mut Agedge_t
                            } else {
                                (objp as *mut Agedge_t).offset(-(1 as libc::c_int as isize))
                            })
                            .node as *mut Agobj_t,
                            state,
                        );
                    } else {
                        exerror(b"head of non-edge\0" as *const u8 as *const libc::c_char);
                    }
                }
                17 => {
                    if objp.is_null() && {
                        objp = (*state).curobj;
                        objp.is_null()
                    } {
                        exerror(
                            b"Current object $ not defined\0" as *const u8 as *const libc::c_char,
                        );
                        return 0 as *mut Agobj_t;
                    }
                    if isedge(objp) != 0 {
                        return deref(
                            pgm,
                            x,
                            (*ref_0).next,
                            (*if ((*(objp as *mut Agedge_t as *mut Agobj_t)).tag).objtype()
                                as libc::c_int
                                == 3 as libc::c_int
                            {
                                objp as *mut Agedge_t
                            } else {
                                (objp as *mut Agedge_t).offset(1 as libc::c_int as isize)
                            })
                            .node as *mut Agobj_t,
                            state,
                        );
                    } else {
                        exerror(
                            b"tail of non-edge %p\0" as *const u8 as *const libc::c_char,
                            objp,
                        );
                    }
                }
                _ => {
                    exerror(
                        b"%s : illegal reference\0" as *const u8 as *const libc::c_char,
                        ((*(*ref_0).symbol).name).as_mut_ptr(),
                    );
                }
            }
        }
    }
    return 0 as *mut Agobj_t;
}
unsafe extern "C" fn assignable(mut objp: *mut Agobj_t, mut name: *mut libc::c_uchar) {
    let mut ch: libc::c_uint = 0;
    let mut rv: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = name;
    TFA_State = 0 as libc::c_int as libc::c_short;
    while TFA_State as libc::c_int >= 0 as libc::c_int && {
        ch = *p as libc::c_uint;
        ch != 0
    } {
        let mut current_block_9: u64;
        let mut c: libc::c_char = (if ch > 127 as libc::c_int as libc::c_uint {
            127 as libc::c_int as libc::c_uint
        } else {
            ch
        }) as libc::c_char;
        if TFA_State as libc::c_int >= 0 as libc::c_int {
            if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                c = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = c as libc::c_int;
                            __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(c as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc()).offset(c as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
                current_block_9 = 13183875560443969876;
            } else if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || c as libc::c_int == '_' as i32)
            {
                TFA_State = -(1 as libc::c_int) as libc::c_short;
                current_block_9 = 8457315219000651999;
            } else {
                current_block_9 = 13183875560443969876;
            }
            match current_block_9 {
                8457315219000651999 => {}
                _ => {
                    if TrieStateTbl[TFA_State as usize].mask
                        & CharMask[(c as libc::c_int - '_' as i32) as usize]
                        != 0
                    {
                        let mut i: libc::c_short = TrieStateTbl[TFA_State as usize].trans_base;
                        while TrieTransTbl[i as usize].c as libc::c_int != c as libc::c_int {
                            i += 1;
                        }
                        TFA_State = TrieTransTbl[i as usize].next_state;
                    } else {
                        TFA_State = -(1 as libc::c_int) as libc::c_short;
                    }
                }
            }
        }
        p = p.offset(1);
    }
    rv = if (TFA_State as libc::c_int) < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        TrieStateTbl[TFA_State as usize].def as libc::c_int
    };
    if rv < 0 as libc::c_int {
        return;
    }
    match ((*objp).tag).objtype() as libc::c_int {
        0 => {
            if rv & (1 as libc::c_int) << 0x6 as libc::c_int != 0 {
                exerror(
                    b"Cannot assign to pseudo-graph attribute %s\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
            }
        }
        1 => {
            if rv & (1 as libc::c_int) << 0x4 as libc::c_int != 0 {
                exerror(
                    b"Cannot assign to pseudo-node attribute %s\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
            }
        }
        _ => {
            if rv & (1 as libc::c_int) << 0x5 as libc::c_int != 0 {
                exerror(
                    b"Cannot assign to pseudo-edge attribute %s\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
            }
        }
    };
}
unsafe extern "C" fn setattr(
    mut objp: *mut Agobj_t,
    mut name: *mut libc::c_char,
    mut val: *mut libc::c_char,
) -> libc::c_int {
    let mut gsym: *mut Agsym_t = agattrsym(objp as *mut libc::c_void, name);
    if gsym.is_null() {
        gsym = agattr(
            agroot(agraphof(objp as *mut libc::c_void) as *mut libc::c_void),
            ((*objp).tag).objtype() as libc::c_int,
            name,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    return agxset(objp as *mut libc::c_void, gsym, val);
}
unsafe extern "C" fn kindToStr(mut kind: libc::c_int) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match kind {
        0 => {
            s = b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        1 => {
            s = b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            s = b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    return s;
}
unsafe extern "C" fn kindOf(mut objp: *mut Agobj_t) -> *mut libc::c_char {
    return kindToStr(agobjkind(objp as *mut libc::c_void));
}
unsafe extern "C" fn lookup(
    mut pgm: *mut Expr_t,
    mut objp: *mut Agobj_t,
    mut sym: *mut Exid_t,
    mut v: *mut Extype_t,
    mut state: *mut Gpr_t,
) -> libc::c_int {
    if (*sym).lex == 283 as libc::c_int as libc::c_long {
        match (*sym).index {
            16 => {
                if isedge(objp) != 0 {
                    (*v).integer = ptr2int(
                        (*if ((*(objp as *mut Agedge_t as *mut Agobj_t)).tag).objtype()
                            as libc::c_int
                            == 2 as libc::c_int
                        {
                            objp as *mut Agedge_t
                        } else {
                            (objp as *mut Agedge_t).offset(-(1 as libc::c_int as isize))
                        })
                        .node as *const libc::c_void,
                    );
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"head of non-edge\0" as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
            17 => {
                if isedge(objp) != 0 {
                    (*v).integer = ptr2int(
                        (*if ((*(objp as *mut Agedge_t as *mut Agobj_t)).tag).objtype()
                            as libc::c_int
                            == 3 as libc::c_int
                        {
                            objp as *mut Agedge_t
                        } else {
                            (objp as *mut Agedge_t).offset(1 as libc::c_int as isize)
                        })
                        .node as *const libc::c_void,
                    );
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"tail of non-edge\0" as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
            18 => {
                let ref mut fresh11 = (*v).string;
                *fresh11 = nameOf(pgm, objp, (*state).tmp);
            }
            19 => {
                if ((*objp).tag).objtype() as libc::c_int == 1 as libc::c_int {
                    (*v).integer = agdegree(
                        agroot(objp as *mut libc::c_void),
                        objp as *mut Agnode_t,
                        1 as libc::c_int,
                        0 as libc::c_int,
                    ) as libc::c_longlong;
                } else {
                    exerror(b"indegree of non-node\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            20 => {
                if ((*objp).tag).objtype() as libc::c_int == 1 as libc::c_int {
                    (*v).integer = agdegree(
                        agroot(objp as *mut libc::c_void),
                        objp as *mut Agnode_t,
                        0 as libc::c_int,
                        1 as libc::c_int,
                    ) as libc::c_longlong;
                } else {
                    exerror(b"outdegree of non-node\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            13 => {
                if ((*objp).tag).objtype() as libc::c_int == 1 as libc::c_int {
                    (*v).integer = agdegree(
                        agroot(objp as *mut libc::c_void),
                        objp as *mut Agnode_t,
                        1 as libc::c_int,
                        1 as libc::c_int,
                    ) as libc::c_longlong;
                } else {
                    exerror(b"degree of non-node\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            14 => {
                if ((*objp).tag).objtype() as libc::c_int == 1 as libc::c_int {
                    if posOf(objp as *mut Agnode_t, 0 as libc::c_int, &mut (*v).floating) != 0 {
                        exerror(
                            b"no x coordinate for node \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            agnameof(objp as *mut libc::c_void),
                        );
                    }
                } else {
                    exerror(b"x coordinate of non-node\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            15 => {
                if ((*objp).tag).objtype() as libc::c_int == 1 as libc::c_int {
                    if posOf(objp as *mut Agnode_t, 1 as libc::c_int, &mut (*v).floating) != 0 {
                        exerror(
                            b"no y coordinate for node \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            agnameof(objp as *mut libc::c_void),
                        );
                    }
                } else {
                    exerror(b"x coordinate of non-node\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            22 => {
                if ((*objp).tag).objtype() as libc::c_int == 0 as libc::c_int {
                    (*v).integer = ptr2int(agparent(objp as *mut Agraph_t) as *const libc::c_void);
                } else {
                    exerror(b"parent of non-graph\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            21 => {
                (*v).integer = ptr2int(agroot(
                    agraphof(objp as *mut libc::c_void) as *mut libc::c_void
                ) as *const libc::c_void);
            }
            23 => {
                if ((*objp).tag).objtype() as libc::c_int == 0 as libc::c_int {
                    (*v).integer = agnedges(objp as *mut Agraph_t) as libc::c_longlong;
                } else {
                    exerror(b"n_edges of non-graph\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            24 => {
                if ((*objp).tag).objtype() as libc::c_int == 0 as libc::c_int {
                    (*v).integer = agnnodes(objp as *mut Agraph_t) as libc::c_longlong;
                } else {
                    exerror(b"n_nodes of non-graph\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            25 => {
                if ((*objp).tag).objtype() as libc::c_int == 0 as libc::c_int {
                    (*v).integer = agisdirected(objp as *mut Agraph_t) as libc::c_longlong;
                } else {
                    exerror(b"directed of non-graph\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            26 => {
                if ((*objp).tag).objtype() as libc::c_int == 0 as libc::c_int {
                    (*v).integer = agisstrict(objp as *mut Agraph_t) as libc::c_longlong;
                } else {
                    exerror(b"strict of non-graph\0" as *const u8 as *const libc::c_char);
                    return -(1 as libc::c_int);
                }
            }
            _ => {
                _err_msg(
                    1 as libc::c_int,
                    b"%s : illegal reference\0" as *const u8 as *const libc::c_char,
                    ((*sym).name).as_mut_ptr(),
                );
                return -(1 as libc::c_int);
            }
        }
    } else {
        let mut gsym: *mut Agsym_t =
            agattrsym(objp as *mut libc::c_void, ((*sym).name).as_mut_ptr());
        if gsym.is_null() {
            gsym = agattr(
                agroot(agraphof(objp as *mut libc::c_void) as *mut libc::c_void),
                ((*objp).tag).objtype() as libc::c_int,
                ((*sym).name).as_mut_ptr(),
                b"\0" as *const u8 as *const libc::c_char,
            );
            _err_msg(
                1 as libc::c_int,
                b"Using value of uninitialized %s attribute \"%s\" of \"%s\"\0" as *const u8
                    as *const libc::c_char,
                kindOf(objp),
                ((*sym).name).as_mut_ptr(),
                nameOf(pgm, objp, (*state).tmp),
            );
        }
        let ref mut fresh12 = (*v).string;
        *fresh12 = agxget(objp as *mut libc::c_void, gsym);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn getArg(mut n: libc::c_int, mut state: *mut Gpr_t) -> *mut libc::c_char {
    if n >= (*state).argc {
        exerror(
            b"program references ARGV[%d] - undefined\0" as *const u8 as *const libc::c_char,
            n,
        );
        return 0 as *mut libc::c_char;
    }
    return *((*state).argv).offset(n as isize);
}
unsafe extern "C" fn setDfltAttr(
    mut gp: *mut Agraph_t,
    mut k: *mut libc::c_char,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut kind: libc::c_int = 0;
    match *k as libc::c_int {
        71 => {
            kind = 0 as libc::c_int;
        }
        69 => {
            kind = 2 as libc::c_int;
        }
        78 => {
            kind = 1 as libc::c_int;
        }
        _ => {
            _err_msg(
                1 as libc::c_int,
                b"Unknown kind \"%s\" passed to setDflt()\0" as *const u8 as *const libc::c_char,
                k,
            );
            return 1 as libc::c_int;
        }
    }
    agattr(gp, kind, name, value);
    return 0 as libc::c_int;
}
unsafe extern "C" fn toKind(mut k: *mut libc::c_char, mut fn_0: *mut libc::c_char) -> libc::c_int {
    let mut kind: libc::c_int = 0;
    match *k as libc::c_int {
        71 => {
            kind = 0 as libc::c_int;
        }
        69 => {
            kind = 2 as libc::c_int;
        }
        78 => {
            kind = 1 as libc::c_int;
        }
        _ => {
            exerror(
                b"Unknown kind \"%s\" passed to %s()\0" as *const u8 as *const libc::c_char,
                k,
                fn_0,
            );
            kind = 0 as libc::c_int;
        }
    }
    return kind;
}
unsafe extern "C" fn nxtAttr(
    mut gp: *mut Agraph_t,
    mut k: *mut libc::c_char,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut fn_0: *mut libc::c_char = (if !name.is_null() {
        b"nxtAttr\0" as *const u8 as *const libc::c_char
    } else {
        b"fstAttr\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut kind: libc::c_int = toKind(k, fn_0);
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    if !name.is_null() {
        sym = agattr(gp, kind, name, 0 as *const libc::c_char);
        if sym.is_null() {
            exerror(
                b"Third argument \"%s\" in nxtAttr() must be the name of an existing attribute\0"
                    as *const u8 as *const libc::c_char,
                name,
            );
            return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    } else {
        sym = 0 as *mut Agsym_t;
    }
    sym = agnxtattr(gp, kind, sym);
    if !sym.is_null() {
        return (*sym).name;
    } else {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    };
}
unsafe extern "C" fn getDfltAttr(
    mut gp: *mut Agraph_t,
    mut k: *mut libc::c_char,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut kind: libc::c_int = toKind(
        k,
        b"getDflt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut sym: *mut Agsym_t = agattr(gp, kind, name, 0 as *const libc::c_char);
    if sym.is_null() {
        sym = agattr(gp, kind, name, b"\0" as *const u8 as *const libc::c_char);
        _err_msg(
            1 as libc::c_int,
            b"Uninitialized %s attribute \"%s\" in %s\0" as *const u8 as *const libc::c_char,
            kindToStr(kind),
            name,
            b"getDflt\0" as *const u8 as *const libc::c_char,
        );
    }
    return (*sym).defval;
}
unsafe extern "C" fn getval(
    mut pgm: *mut Expr_t,
    mut node: *mut Exnode_t,
    mut sym: *mut Exid_t,
    mut ref_0: *mut Exref_t,
    mut env: *mut libc::c_void,
    mut elt: libc::c_int,
    mut disc: *mut Exdisc_t,
) -> Extype_t {
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut state: *mut Gpr_t = 0 as *mut Gpr_t;
    let mut args: *mut Extype_t = 0 as *mut Extype_t;
    let mut objp: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut objp1: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gp: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut hp: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut ep: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: *mut gvprbinding = 0 as *mut gvprbinding;
    if (*sym).lex != 271 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(
            b"sym->lex != CONSTANT\0" as *const u8 as *const libc::c_char,
            b"compile.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"Extype_t getval(Expr_t *, Exnode_t *, Exid_t *, Exref_t *, void *, int, Exdisc_t *)\0",
            ))
                .as_ptr(),
        );
    }
    if elt == -(2 as libc::c_int) {
        args = env as *mut Extype_t;
        state = (*disc).user as *mut Gpr_t;
        match (*sym).index {
            33 => {
                gp = openG(
                    (*args.offset(0 as libc::c_int as isize)).string,
                    xargs((*args.offset(1 as libc::c_int as isize)).string),
                );
                v.integer = ptr2int(gp as *const libc::c_void);
            }
            34 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    gp = openSubg(gp, (*args.offset(1 as libc::c_int as isize)).string);
                    v.integer = ptr2int(gp as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to subg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            35 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    v.integer = ptr2int(agsubg(
                        gp,
                        (*args.offset(1 as libc::c_int as isize)).string,
                        0 as libc::c_int,
                    ) as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to isSubg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            36 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    gp = agfstsubg(gp);
                    v.integer = ptr2int(gp as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to fstsubg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            37 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    gp = agnxtsubg(gp);
                    v.integer = ptr2int(gp as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to nxtsubg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            38 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    np = openNode(gp, (*args.offset(1 as libc::c_int as isize)).string);
                    v.integer = ptr2int(np as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to node()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            39 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to addNode()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to addNode()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = ptr2int(addNode(gp, np, 1 as libc::c_int) as *const libc::c_void);
                }
            }
            40 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    np = agfstnode(gp);
                    v.integer = ptr2int(np as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to fstnode()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            41 => {
                np = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if !np.is_null() {
                    np = agnxtnode(agroot(np as *mut libc::c_void), np);
                    v.integer = ptr2int(np as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to nxtnode()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            42 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    np = agnxtnode(gp, np);
                    v.integer = ptr2int(np as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to nxtnode_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            43 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    v.integer = ptr2int(agnode(
                        gp,
                        (*args.offset(1 as libc::c_int as isize)).string,
                        0 as libc::c_int,
                    ) as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to isNode()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            44 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    v.integer = ptr2int(addNode(gp, np, 0 as libc::c_int) as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to isSubnode()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            45 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    v.integer =
                        agdegree(gp, np, 1 as libc::c_int, 0 as libc::c_int) as libc::c_longlong;
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to indegreeOf()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            46 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    v.integer =
                        agdegree(gp, np, 0 as libc::c_int, 1 as libc::c_int) as libc::c_longlong;
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to outdegreeOf()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            47 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    v.integer =
                        agdegree(gp, np, 1 as libc::c_int, 1 as libc::c_int) as libc::c_longlong;
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to degreeOf()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            48 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                objp = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agobj_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to isIn()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if objp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL object passed to isIn()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = agcontains(gp, objp as *mut libc::c_void) as libc::c_longlong;
                }
            }
            65 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to compOf()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to compOf()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = ptr2int(compOf(gp, np) as *const libc::c_void);
                }
            }
            66 => {
                objp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agobj_t;
                if objp.is_null() {
                    exerror(
                        b"NULL object passed to kindOf()\0" as *const u8 as *const libc::c_char,
                    );
                    v.string = 0 as *mut libc::c_char;
                } else {
                    match ((*objp).tag).objtype() as libc::c_int {
                        0 => {
                            v.string =
                                b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        }
                        1 => {
                            v.string =
                                b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        }
                        3 | 2 => {
                            v.string =
                                b"E\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        }
                        _ => {}
                    }
                }
            }
            49 => {
                key = (*args.offset(2 as libc::c_int as isize)).string;
                if *key as libc::c_int == '\0' as i32 {
                    key = 0 as *mut libc::c_char;
                }
                np = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agnode_t;
                hp = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL tail node passed to edge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if hp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL head node passed to edge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    ep = openEdge(0 as *mut Agraph_t, np, hp, key);
                    v.integer = ptr2int(ep as *const libc::c_void);
                }
            }
            50 => {
                key = (*args.offset(3 as libc::c_int as isize)).string;
                if *key as libc::c_int == '\0' as i32 {
                    key = 0 as *mut libc::c_char;
                }
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                hp = int2ptr((*args.offset(2 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL tail node passed to edge_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if hp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL head node passed to edge_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    ep = openEdge(gp, np, hp, key);
                    v.integer = ptr2int(ep as *const libc::c_void);
                }
            }
            51 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                ep = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agedge_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to addEdge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if ep.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to addEdge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = ptr2int(addEdge(gp, ep, 1 as libc::c_int) as *const libc::c_void);
                }
            }
            52 => {
                ep = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agedge_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if ep.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to opp()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to opp()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    if (*(if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        ep
                    } else {
                        ep.offset(-(1 as libc::c_int as isize))
                    }))
                    .node
                        == np
                    {
                        np = (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 3 as libc::c_int
                        {
                            ep
                        } else {
                            ep.offset(1 as libc::c_int as isize)
                        })
                        .node;
                    } else {
                        np = (*if ((*(ep as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            ep
                        } else {
                            ep.offset(-(1 as libc::c_int as isize))
                        })
                        .node;
                    }
                    v.integer = ptr2int(np as *const libc::c_void);
                }
            }
            69 => {
                key = (*args.offset(2 as libc::c_int as isize)).string;
                if *key as libc::c_int == '\0' as i32 {
                    key = 0 as *mut libc::c_char;
                }
                np = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agnode_t;
                hp = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL tail node passed to isEdge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if hp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL head node passed to isEdge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer =
                        ptr2int(isEdge(agroot(np as *mut libc::c_void), np, hp, key)
                            as *const libc::c_void);
                }
            }
            70 => {
                key = (*args.offset(3 as libc::c_int as isize)).string;
                if *key as libc::c_int == '\0' as i32 {
                    key = 0 as *mut libc::c_char;
                }
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                hp = int2ptr((*args.offset(2 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL tail node passed to isEdge_sg()\0" as *const u8
                            as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if hp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL head node passed to isEdge_sg()\0" as *const u8
                            as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = ptr2int(isEdge(gp, np, hp, key) as *const libc::c_void);
                }
            }
            71 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                ep = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agedge_t;
                if gp.is_null() {
                    gp = agroot(ep as *mut libc::c_void);
                }
                if !ep.is_null() {
                    v.integer = ptr2int(addEdge(gp, ep, 0 as libc::c_int) as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to isSubedge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            53 => {
                np = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if !np.is_null() {
                    ep = agfstout(agroot(np as *mut libc::c_void), np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to fstout()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            59 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    ep = agfstout(gp, np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to fstout_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            54 => {
                ep = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agedge_t;
                if !ep.is_null() {
                    ep = agnxtout(agroot(ep as *mut libc::c_void), ep);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to nxtout()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            60 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                ep = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agedge_t;
                if gp.is_null() {
                    gp = agroot(ep as *mut libc::c_void);
                }
                if !ep.is_null() {
                    ep = agnxtout(gp, ep);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to nxtout_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            55 => {
                np = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if !np.is_null() {
                    ep = agfstin(agroot(np as *mut libc::c_void), np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to fstin()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            61 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    ep = agfstin(gp, np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to fstin_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            56 => {
                ep = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agedge_t;
                if !ep.is_null() {
                    ep = agnxtin(agroot(ep as *mut libc::c_void), ep);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to nxtin()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            62 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                ep = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agedge_t;
                if gp.is_null() {
                    gp = agroot(ep as *mut libc::c_void);
                }
                if !ep.is_null() {
                    ep = agnxtin(gp, ep);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to nxtin_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            57 => {
                np = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if !np.is_null() {
                    ep = agfstedge(agroot(np as *mut libc::c_void), np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to fstedge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            63 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if !np.is_null() {
                    ep = agfstedge(gp, np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to fstedge_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            58 => {
                ep = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agedge_t;
                np = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if ep.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to nxtedge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to nxtedge()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    ep = agnxtedge(agroot(np as *mut libc::c_void), ep, np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                }
            }
            64 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                ep = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agedge_t;
                np = int2ptr((*args.offset(2 as libc::c_int as isize)).integer) as *mut Agnode_t;
                if gp.is_null() {
                    gp = agroot(np as *mut libc::c_void);
                }
                if ep.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL edge passed to nxtedge_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if np.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL node passed to nxtedge_sg()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    ep = agnxtedge(gp, ep, np);
                    v.integer = ptr2int(ep as *const libc::c_void);
                }
            }
            88 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                objp = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agobj_t;
                if objp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL object passed to clone()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = ptr2int(copy(gp, objp) as *const libc::c_void);
                }
            }
            86 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                objp = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agobj_t;
                if objp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL object passed to clone()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = ptr2int(clone(gp, objp) as *const libc::c_void);
                }
            }
            87 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    gp = cloneG(gp, (*args.offset(1 as libc::c_int as isize)).string);
                    v.integer = ptr2int(gp as *const libc::c_void);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to cloneG()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            89 => {
                objp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agobj_t;
                objp1 = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agobj_t;
                if !(!objp.is_null() && !objp1.is_null()) {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL object passed to copyA()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = copyAttr(objp, objp1) as libc::c_longlong;
                }
            }
            82 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to induce()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else {
                    nodeInduce(gp);
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            74 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to write()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer =
                        sfioWrite(gp, (*state).outFile, (*state).dfltIO) as libc::c_longlong;
                }
            }
            75 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to writeG()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = writeFile(
                        gp,
                        (*args.offset(1 as libc::c_int as isize)).string,
                        (*state).dfltIO,
                    ) as libc::c_longlong;
                }
            }
            76 => {
                gp = readFile((*args.offset(0 as libc::c_int as isize)).string);
                v.integer = ptr2int(gp as *const libc::c_void);
            }
            77 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to fwriteG()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = fwriteFile(
                        pgm,
                        gp,
                        (*args.offset(1 as libc::c_int as isize)).integer as libc::c_int,
                        (*state).dfltIO,
                    ) as libc::c_longlong;
                }
            }
            78 => {
                gp = freadFile(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).integer as libc::c_int,
                );
                v.integer = ptr2int(gp as *const libc::c_void);
            }
            79 => {
                v.integer = openFile(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    (*args.offset(1 as libc::c_int as isize)).string,
                ) as libc::c_longlong;
            }
            80 => {
                v.integer = closeFile(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).integer as libc::c_int,
                ) as libc::c_longlong;
            }
            81 => {
                v.string = readLine(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).integer as libc::c_int,
                );
            }
            83 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to isDirect()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = agisdirected(gp) as libc::c_longlong;
                }
            }
            84 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to isStrict()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = agisstrict(gp) as libc::c_longlong;
                }
            }
            85 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                objp = int2ptr((*args.offset(1 as libc::c_int as isize)).integer) as *mut Agobj_t;
                if objp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL object passed to delete()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else if objp == (*state).curgraph as *mut Agobj_t {
                    _err_msg(
                        1 as libc::c_int,
                        b"cannot delete current graph $G\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else if objp == (*state).target as *mut Agobj_t {
                    _err_msg(
                        1 as libc::c_int,
                        b"cannot delete target graph $T\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else if objp == (*state).curobj {
                    v.integer = deleteObj(gp, objp) as libc::c_longlong;
                    if v.integer == 0 {
                        let ref mut fresh13 = (*state).curobj;
                        *fresh13 = 0 as *mut Agobj_t;
                    }
                } else {
                    v.integer = deleteObj(gp, objp) as libc::c_longlong;
                }
            }
            90 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to lock()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = -(1 as libc::c_int) as libc::c_longlong;
                } else {
                    v.integer = lockGraph(
                        gp,
                        (*args.offset(1 as libc::c_int as isize)).integer as libc::c_int,
                    ) as libc::c_longlong;
                }
            }
            91 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to nNodes()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = agnnodes(gp) as libc::c_longlong;
                }
            }
            92 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if gp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to nEdges()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = agnedges(gp) as libc::c_longlong;
                }
            }
            121 => {
                v.integer =
                    atoi((*args.offset(0 as libc::c_int as isize)).string) as libc::c_longlong;
            }
            122 => {
                v.floating = atof((*args.offset(0 as libc::c_int as isize)).string);
            }
            93 => {
                v.floating = sqrt((*args.offset(0 as libc::c_int as isize)).floating);
            }
            94 => {
                v.floating = cos((*args.offset(0 as libc::c_int as isize)).floating);
            }
            95 => {
                v.floating = sin((*args.offset(0 as libc::c_int as isize)).floating);
            }
            96 => {
                v.floating = atan2(
                    (*args.offset(0 as libc::c_int as isize)).floating,
                    (*args.offset(1 as libc::c_int as isize)).floating,
                );
            }
            97 => {
                v.floating = exp((*args.offset(0 as libc::c_int as isize)).floating);
            }
            98 => {
                v.floating = pow(
                    (*args.offset(0 as libc::c_int as isize)).floating,
                    (*args.offset(1 as libc::c_int as isize)).floating,
                );
            }
            99 => {
                v.floating = log((*args.offset(0 as libc::c_int as isize)).floating);
            }
            100 => {
                v.floating = if (*args.offset(0 as libc::c_int as isize)).floating
                    < (*args.offset(1 as libc::c_int as isize)).floating
                {
                    (*args.offset(0 as libc::c_int as isize)).floating
                } else {
                    (*args.offset(1 as libc::c_int as isize)).floating
                };
            }
            101 => {
                v.floating = if (*args.offset(0 as libc::c_int as isize)).floating
                    > (*args.offset(1 as libc::c_int as isize)).floating
                {
                    (*args.offset(0 as libc::c_int as isize)).floating
                } else {
                    (*args.offset(1 as libc::c_int as isize)).floating
                };
            }
            102 => {
                v.integer =
                    system((*args.offset(0 as libc::c_int as isize)).string) as libc::c_longlong;
            }
            114 | 110 => {
                objp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agobj_t;
                name = (*args.offset(1 as libc::c_int as isize)).string;
                if objp.is_null() {
                    exerror(
                        b"NULL object passed to aget()/hasAttr()\0" as *const u8
                            as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else if name.is_null() {
                    exerror(
                        b"NULL name passed to aget()/hasAttr()\0" as *const u8
                            as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                } else {
                    let mut gsym: *mut Agsym_t = agattrsym(objp as *mut libc::c_void, name);
                    if (*sym).index == F_hasattr as libc::c_int as libc::c_long {
                        v.integer = (gsym != 0 as *mut libc::c_void as *mut Agsym_t) as libc::c_int
                            as libc::c_longlong;
                    } else {
                        if gsym.is_null() {
                            gsym = agattr(
                                agroot(agraphof(objp as *mut libc::c_void) as *mut libc::c_void),
                                ((*objp).tag).objtype() as libc::c_int,
                                name,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                            _err_msg(
                                1 as libc::c_int,
                                b"Using value of %s uninitialized attribute \"%s\" of \"%s\" in aget()\0"
                                    as *const u8 as *const libc::c_char,
                                kindOf(objp),
                                name,
                                nameOf(pgm, objp, (*state).tmp),
                            );
                        }
                        v.string = agxget(objp as *mut libc::c_void, gsym);
                    }
                }
            }
            111 => {
                objp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agobj_t;
                if objp.is_null() {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL object passed to aset()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else {
                    let mut name_0: *mut libc::c_char =
                        (*args.offset(1 as libc::c_int as isize)).string;
                    let mut value: *mut libc::c_char =
                        (*args.offset(2 as libc::c_int as isize)).string;
                    if name_0.is_null() {
                        _err_msg(
                            1 as libc::c_int,
                            b"NULL name passed to aset()\0" as *const u8 as *const libc::c_char,
                        );
                        v.integer = 1 as libc::c_int as libc::c_longlong;
                    } else if value.is_null() {
                        _err_msg(
                            1 as libc::c_int,
                            b"NULL value passed to aset()\0" as *const u8 as *const libc::c_char,
                        );
                        v.integer = 1 as libc::c_int as libc::c_longlong;
                    } else {
                        v.integer = setattr(objp, name_0, value) as libc::c_longlong;
                    }
                }
            }
            113 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    let mut kind: *mut libc::c_char =
                        (*args.offset(1 as libc::c_int as isize)).string;
                    let mut name_1: *mut libc::c_char =
                        (*args.offset(2 as libc::c_int as isize)).string;
                    let mut value_0: *mut libc::c_char =
                        (*args.offset(3 as libc::c_int as isize)).string;
                    if name_1.is_null() {
                        _err_msg(
                            1 as libc::c_int,
                            b"NULL name passed to setDflt()\0" as *const u8 as *const libc::c_char,
                        );
                        v.integer = 1 as libc::c_int as libc::c_longlong;
                    } else if value_0.is_null() {
                        _err_msg(
                            1 as libc::c_int,
                            b"NULL value passed to setDflt()\0" as *const u8 as *const libc::c_char,
                        );
                        v.integer = 1 as libc::c_int as libc::c_longlong;
                    } else if kind.is_null() {
                        _err_msg(
                            1 as libc::c_int,
                            b"NULL kind passed to setDflt()\0" as *const u8 as *const libc::c_char,
                        );
                        v.integer = 1 as libc::c_int as libc::c_longlong;
                    } else {
                        v.integer = setDfltAttr(gp, kind, name_1, value_0) as libc::c_longlong;
                    }
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to node()\0" as *const u8 as *const libc::c_char,
                    );
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            116 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    let mut kind_0: *mut libc::c_char =
                        (*args.offset(1 as libc::c_int as isize)).string;
                    if kind_0.is_null() {
                        _err_msg(
                            2 as libc::c_int,
                            b"NULL kind passed to fstAttr()\0" as *const u8 as *const libc::c_char,
                        );
                        v.string = 0 as *mut libc::c_char;
                    } else {
                        v.string = nxtAttr(gp, kind_0, 0 as *mut libc::c_char);
                    }
                } else {
                    exerror(
                        b"NULL graph passed to fstAttr()\0" as *const u8 as *const libc::c_char,
                    );
                    v.string = 0 as *mut libc::c_char;
                }
            }
            117 | 115 | 112 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    let mut kind_1: *mut libc::c_char =
                        (*args.offset(1 as libc::c_int as isize)).string;
                    let mut name_2: *mut libc::c_char =
                        (*args.offset(2 as libc::c_int as isize)).string;
                    if name_2.is_null() {
                        exerror(
                            b"NULL name passed to %s\0" as *const u8 as *const libc::c_char,
                            ((*sym).name).as_mut_ptr(),
                        );
                        v.string = 0 as *mut libc::c_char;
                    } else if kind_1.is_null() {
                        exerror(
                            b"NULL kind passed to %s\0" as *const u8 as *const libc::c_char,
                            ((*sym).name).as_mut_ptr(),
                        );
                        v.string = 0 as *mut libc::c_char;
                    } else if (*sym).index == F_isattr as libc::c_int as libc::c_long {
                        v.integer = (agattr(
                            gp,
                            toKind(kind_1, ((*sym).name).as_mut_ptr()),
                            name_2,
                            0 as *const libc::c_char,
                        ) != 0 as *mut libc::c_void as *mut Agsym_t)
                            as libc::c_int as libc::c_longlong;
                    } else if (*sym).index == F_nxtattr as libc::c_int as libc::c_long {
                        v.string = nxtAttr(gp, kind_1, name_2);
                    } else {
                        v.string = getDfltAttr(gp, kind_1, name_2);
                    }
                } else {
                    exerror(
                        b"NULL graph passed to %s\0" as *const u8 as *const libc::c_char,
                        ((*sym).name).as_mut_ptr(),
                    );
                    v.string = 0 as *mut libc::c_char;
                }
            }
            109 => {
                v.string = canon(pgm, (*args.offset(0 as libc::c_int as isize)).string);
            }
            108 => {
                v.integer =
                    aghtmlstr((*args.offset(0 as libc::c_int as isize)).string) as libc::c_longlong;
            }
            107 => {
                gp = int2ptr((*args.offset(0 as libc::c_int as isize)).integer) as *mut Agraph_t;
                if !gp.is_null() {
                    v.string = toHtml(gp, (*args.offset(1 as libc::c_int as isize)).string);
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"NULL graph passed to html()\0" as *const u8 as *const libc::c_char,
                    );
                    v.string = 0 as *mut libc::c_char;
                }
            }
            118 => {
                v.string = toLower(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    (*state).tmp,
                );
            }
            123 => {
                v.string = colorx(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    (*args.offset(1 as libc::c_int as isize)).string,
                    (*state).tmp,
                );
            }
            120 => {
                if !((*args.offset(0 as libc::c_int as isize)).string).is_null() {
                    if !((*args.offset(1 as libc::c_int as isize)).string).is_null() {
                        v.integer = strcmp(
                            (*args.offset(0 as libc::c_int as isize)).string,
                            (*args.offset(1 as libc::c_int as isize)).string,
                        ) as libc::c_longlong;
                    } else {
                        v.integer = -(1 as libc::c_int) as libc::c_longlong;
                    }
                } else if !((*args.offset(1 as libc::c_int as isize)).string).is_null() {
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                } else {
                    v.integer = 0 as libc::c_int as libc::c_longlong;
                }
            }
            119 => {
                v.string = toUpper(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    (*state).tmp,
                );
            }
            103 => {
                v.string = xyOf(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    1 as libc::c_int != 0,
                );
            }
            104 => {
                v.string = xyOf(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    0 as libc::c_int != 0,
                );
            }
            105 => {
                v.string = bbOf(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    1 as libc::c_int != 0,
                );
            }
            106 => {
                v.string = bbOf(
                    pgm,
                    (*args.offset(0 as libc::c_int as isize)).string,
                    0 as libc::c_int != 0,
                );
            }
            72 => {
                v.integer =
                    strlen((*args.offset(0 as libc::c_int as isize)).string) as libc::c_longlong;
            }
            67 => {
                v.integer = indexOf(
                    (*args.offset(0 as libc::c_int as isize)).string,
                    (*args.offset(1 as libc::c_int as isize)).string,
                ) as libc::c_longlong;
            }
            68 => {
                v.integer = rindexOf(
                    (*args.offset(0 as libc::c_int as isize)).string,
                    (*args.offset(1 as libc::c_int as isize)).string,
                ) as libc::c_longlong;
            }
            73 => {
                v.integer = match_0(
                    (*args.offset(0 as libc::c_int as isize)).string,
                    (*args.offset(1 as libc::c_int as isize)).string,
                ) as libc::c_longlong;
            }
            124 => {
                bp = findBinding(state, (*args.offset(0 as libc::c_int as isize)).string);
                if !bp.is_null() {
                    v.integer = ((*bp).fn_0).expect("non-null function pointer")(
                        (*args.offset(1 as libc::c_int as isize)).string,
                    ) as libc::c_longlong;
                } else {
                    v.integer = -(1 as libc::c_int) as libc::c_longlong;
                }
            }
            _ => {
                v.integer = -(1 as libc::c_int) as libc::c_longlong;
                exerror(
                    b"unknown function call: %s\0" as *const u8 as *const libc::c_char,
                    ((*sym).name).as_mut_ptr(),
                );
            }
        }
        return v;
    } else {
        if elt == -(3 as libc::c_int) {
            args = env as *mut Extype_t;
            state = (*disc).user as *mut Gpr_t;
            match (*sym).index {
                32 => {
                    v.string = getArg(
                        (*args.offset(0 as libc::c_int as isize)).integer as libc::c_int,
                        state,
                    );
                }
                _ => {
                    exerror(
                        b"unknown array name: %s\0" as *const u8 as *const libc::c_char,
                        ((*sym).name).as_mut_ptr(),
                    );
                    v.string = 0 as *mut libc::c_char;
                }
            }
            return v;
        }
    }
    state = env as *mut Gpr_t;
    if !ref_0.is_null() {
        objp = deref(pgm, node, ref_0, 0 as *mut Agobj_t, state);
        if objp.is_null() {
            exerror(
                b"null reference in expression %s\0" as *const u8 as *const libc::c_char,
                deparse(pgm, node, (*state).tmp),
            );
        }
    } else if (*sym).lex == 283 as libc::c_int as libc::c_long
        && (*sym).index <= LAST_V as libc::c_int as libc::c_long
    {
        match (*sym).index {
            1 => {
                v.integer = ptr2int((*state).curobj as *const libc::c_void);
            }
            2 => {
                v.integer = ptr2int((*state).curgraph as *const libc::c_void);
            }
            3 => {
                v.integer = ptr2int((*state).nextgraph as *const libc::c_void);
            }
            4 => {
                v.integer = ptr2int((*state).target as *const libc::c_void);
            }
            5 => {
                v.integer = ptr2int((*state).outgraph as *const libc::c_void);
            }
            6 => {
                v.string = (*state).tgtname;
            }
            7 => {
                v.string = (*state).infname;
            }
            12 => {
                v.integer = (*state).argc as libc::c_longlong;
            }
            11 => {
                v.integer = (*state).tvt as libc::c_longlong;
            }
            8 => {
                v.integer = ptr2int((*state).tvroot as *const libc::c_void);
            }
            9 => {
                v.integer = ptr2int((*state).tvnext as *const libc::c_void);
            }
            10 => {
                v.integer = ptr2int((*state).tvedge as *const libc::c_void);
            }
            _ => {}
        }
        return v;
    } else {
        objp = (*state).curobj;
        if objp.is_null() {
            exerror(
                b"current object $ not defined as reference for %s\0" as *const u8
                    as *const libc::c_char,
                deparse(pgm, node, (*state).tmp),
            );
        }
    }
    if !objp.is_null() {
        if lookup(pgm, objp, sym, &mut v, state) != 0 {
            exerror(
                b"in expression %s\0" as *const u8 as *const libc::c_char,
                deparse(pgm, node, (*state).tmp),
            );
            v.integer = 0 as libc::c_int as libc::c_longlong;
        }
    } else {
        v.integer = 0 as libc::c_int as libc::c_longlong;
    }
    return v;
}
unsafe extern "C" fn typeName(mut pg: *mut Expr_t, mut op: libc::c_int) -> *mut libc::c_char {
    return typenames[(op - (LAST_M as libc::c_int + 1 as libc::c_int)) as usize];
}
unsafe extern "C" fn setval(
    mut pgm: *mut Expr_t,
    mut x: *mut Exnode_t,
    mut sym: *mut Exid_t,
    mut ref_0: *mut Exref_t,
    mut env: *mut libc::c_void,
    mut elt: libc::c_int,
    mut v: Extype_t,
    mut disc: *mut Exdisc_t,
) -> libc::c_int {
    let mut state: *mut Gpr_t = 0 as *mut Gpr_t;
    let mut objp: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut iv: libc::c_int = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    state = env as *mut Gpr_t;
    if !ref_0.is_null() {
        objp = deref(pgm, x, ref_0, 0 as *mut Agobj_t, state);
        if objp.is_null() {
            exerror(
                b"in expression %s.%s\0" as *const u8 as *const libc::c_char,
                ((*(*ref_0).symbol).name).as_mut_ptr(),
                deparse(pgm, x, (*state).tmp),
            );
            return -(1 as libc::c_int);
        }
    } else if MINNAME as libc::c_int as libc::c_long <= (*sym).index
        && (*sym).index <= MAXNAME as libc::c_int as libc::c_long
    {
        match (*sym).index {
            5 => {
                let ref mut fresh14 = (*state).outgraph;
                *fresh14 = int2ptr(v.integer) as *mut Agraph_t;
            }
            11 => {
                iv = v.integer as libc::c_int;
                if validTVT(v.integer as libc::c_int) != 0 {
                    (*state).tvt = iv as trav_type;
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"unexpected value %d assigned to %s : ignored\0" as *const u8
                            as *const libc::c_char,
                        iv,
                        typeName(pgm, T_tvtyp as libc::c_int),
                    );
                }
            }
            8 => {
                np = int2ptr(v.integer) as *mut Agnode_t;
                if np.is_null() || agroot(np as *mut libc::c_void) == (*state).curgraph {
                    let ref mut fresh15 = (*state).tvroot;
                    *fresh15 = np;
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"cannot set $tvroot, node %s not in $G : ignored\0" as *const u8
                            as *const libc::c_char,
                        agnameof(np as *mut libc::c_void),
                    );
                }
            }
            9 => {
                np = int2ptr(v.integer) as *mut Agnode_t;
                if np.is_null() || agroot(np as *mut libc::c_void) == (*state).curgraph {
                    let ref mut fresh16 = (*state).tvnext;
                    *fresh16 = np;
                    (*state).flags |= 8 as libc::c_int;
                } else {
                    _err_msg(
                        1 as libc::c_int,
                        b"cannot set $tvnext, node %s not in $G : ignored\0" as *const u8
                            as *const libc::c_char,
                        agnameof(np as *mut libc::c_void),
                    );
                }
            }
            6 => {
                if strcmp((*state).tgtname, v.string) != 0 {
                    vmfree((*pgm).vm, (*state).tgtname as *mut libc::c_void);
                    let ref mut fresh17 = (*state).tgtname;
                    *fresh17 = vmstrdup((*pgm).vm, v.string);
                    (*state).name_used = 0 as libc::c_int;
                }
            }
            _ => {
                rv = -(1 as libc::c_int);
            }
        }
        return rv;
    } else {
        objp = (*state).curobj;
        if objp.is_null() {
            exerror(
                b"current object $ undefined in expression %s\0" as *const u8
                    as *const libc::c_char,
                deparse(pgm, x, (*state).tmp),
            );
            return -(1 as libc::c_int);
        }
    }
    assignable(objp, ((*sym).name).as_mut_ptr() as *mut libc::c_uchar);
    return setattr(objp, ((*sym).name).as_mut_ptr(), v.string);
}
static mut codePhase: libc::c_int = 0;
unsafe extern "C" fn typeChk(mut intype: tctype, mut sym: *mut Exid_t) -> tctype {
    let mut dom: tctype = 0 as libc::c_int as tctype;
    let mut rng: tctype = 0 as libc::c_int as tctype;
    match (*sym).lex {
        275 => {
            dom = 0 as libc::c_int as tctype;
            match (*sym).type_0 {
                30 => {
                    rng = ((1 as libc::c_int) << 0x4 as libc::c_int
                        | (1 as libc::c_int) << 0x5 as libc::c_int
                        | (1 as libc::c_int) << 0x6 as libc::c_int)
                        as tctype;
                }
                27 => {
                    rng = ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype;
                }
                29 => {
                    rng = ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype;
                }
                28 => {
                    rng = ((1 as libc::c_int) << 0x5 as libc::c_int) as tctype;
                }
                259 => {
                    rng = ((1 as libc::c_int) << 0o2 as libc::c_int) as tctype;
                }
                262 => {
                    rng = ((1 as libc::c_int) << 0o1 as libc::c_int) as tctype;
                }
                263 => {
                    rng = ((1 as libc::c_int) << 0o3 as libc::c_int) as tctype;
                }
                _ => {
                    exerror(
                        b"unknown dynamic type %ld of symbol %s\0" as *const u8
                            as *const libc::c_char,
                        (*sym).type_0,
                        ((*sym).name).as_mut_ptr(),
                    );
                }
            }
        }
        283 => {
            if (*sym).index <= MAXNAME as libc::c_int as libc::c_long {
                match (*sym).index {
                    8 | 1 | 2 | 3 => {
                        if !(1 as libc::c_int <= codePhase && codePhase <= 4 as libc::c_int) {
                            exerror(
                                b"keyword %s cannot be used in BEGIN/END statements\0" as *const u8
                                    as *const libc::c_char,
                                ((*sym).name).as_mut_ptr(),
                            );
                        }
                    }
                    4 => {
                        if !(2 as libc::c_int <= codePhase && codePhase <= 4 as libc::c_int) {
                            exerror(
                                b"keyword %s cannot be used in BEGIN/BEG_G/END statements\0"
                                    as *const u8
                                    as *const libc::c_char,
                                ((*sym).name).as_mut_ptr(),
                            );
                        }
                    }
                    _ => {}
                }
                dom = tchk[(*sym).index as usize][0 as libc::c_int as usize];
                rng = tchk[(*sym).index as usize][1 as libc::c_int as usize];
            } else {
                dom = ((1 as libc::c_int) << 0x4 as libc::c_int
                    | (1 as libc::c_int) << 0x5 as libc::c_int
                    | (1 as libc::c_int) << 0x6 as libc::c_int) as tctype;
                rng = ((1 as libc::c_int) << 0o3 as libc::c_int) as tctype;
            }
        }
        287 => {
            if intype == 0 && !(1 as libc::c_int <= codePhase && codePhase <= 4 as libc::c_int) {
                exerror(
                    b"undeclared, unmodified names like \"%s\" cannot be\nused in BEGIN and END statements\0"
                        as *const u8 as *const libc::c_char,
                    ((*sym).name).as_mut_ptr(),
                );
            }
            dom = ((1 as libc::c_int) << 0x4 as libc::c_int
                | (1 as libc::c_int) << 0x5 as libc::c_int
                | (1 as libc::c_int) << 0x6 as libc::c_int) as tctype;
            rng = ((1 as libc::c_int) << 0o3 as libc::c_int) as tctype;
        }
        _ => {
            exerror(
                b"unexpected symbol in typeChk: name %s, lex %ld\0" as *const u8
                    as *const libc::c_char,
                ((*sym).name).as_mut_ptr(),
                (*sym).lex,
            );
        }
    }
    if dom != 0 {
        if intype == 0 {
            intype = ((1 as libc::c_int) << 0x4 as libc::c_int
                | (1 as libc::c_int) << 0x5 as libc::c_int
                | (1 as libc::c_int) << 0x6 as libc::c_int) as tctype;
        }
        if dom as libc::c_int & intype as libc::c_int == 0 {
            rng = 0 as libc::c_int as tctype;
        }
    } else if intype != 0 {
        rng = 0 as libc::c_int as tctype;
    }
    return rng;
}
unsafe extern "C" fn typeChkExp(mut ref_0: *mut Exref_t, mut sym: *mut Exid_t) -> tctype {
    let mut ty: tctype = 0;
    if !ref_0.is_null() {
        ty = typeChk(0 as libc::c_int as tctype, (*ref_0).symbol);
        ref_0 = (*ref_0).next;
        while ty as libc::c_int != 0 && !ref_0.is_null() {
            ty = typeChk(ty, (*ref_0).symbol);
            ref_0 = (*ref_0).next;
        }
        if ty == 0 {
            return 0 as libc::c_int as tctype;
        }
    } else {
        ty = 0 as libc::c_int as tctype;
    }
    return typeChk(ty, sym);
}
unsafe extern "C" fn refval(
    mut pgm: *mut Expr_t,
    mut node: *mut Exnode_t,
    mut sym: *mut Exid_t,
    mut ref_0: *mut Exref_t,
    mut str: *mut libc::c_char,
    mut elt: libc::c_int,
    mut disc: *mut Exdisc_t,
) -> Extype_t {
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    if (*sym).lex == 271 as libc::c_int as libc::c_long {
        match (*sym).index {
            125 => {
                v.integer = TV_flat as libc::c_int as libc::c_longlong;
            }
            126 => {
                v.integer = TV_ne as libc::c_int as libc::c_longlong;
            }
            127 => {
                v.integer = TV_en as libc::c_int as libc::c_longlong;
            }
            128 => {
                v.integer = TV_bfs as libc::c_int as libc::c_longlong;
            }
            129 => {
                v.integer = TV_dfs as libc::c_int as libc::c_longlong;
            }
            130 => {
                v.integer = TV_fwd as libc::c_int as libc::c_longlong;
            }
            131 => {
                v.integer = TV_rev as libc::c_int as libc::c_longlong;
            }
            132 => {
                v.integer = TV_postdfs as libc::c_int as libc::c_longlong;
            }
            133 => {
                v.integer = TV_postfwd as libc::c_int as libc::c_longlong;
            }
            134 => {
                v.integer = TV_postrev as libc::c_int as libc::c_longlong;
            }
            135 => {
                v.integer = TV_prepostdfs as libc::c_int as libc::c_longlong;
            }
            136 => {
                v.integer = TV_prepostfwd as libc::c_int as libc::c_longlong;
            }
            137 => {
                v.integer = TV_prepostrev as libc::c_int as libc::c_longlong;
            }
            138 => {
                v.integer = 0 as libc::c_int as libc::c_longlong;
            }
            _ => {
                v = exzero((*node).type_0);
            }
        }
    } else {
        if typeChkExp(ref_0, sym) == 0 {
            let mut state: *mut Gpr_t = (*disc).user as *mut Gpr_t;
            exerror(
                b"type error using %s\0" as *const u8 as *const libc::c_char,
                deparse(pgm, node, (*state).tmp),
            );
        }
        v = exzero((*node).type_0);
    }
    return v;
}
unsafe extern "C" fn binary(
    mut pg: *mut Expr_t,
    mut l: *mut Exnode_t,
    mut ex: *mut Exnode_t,
    mut r: *mut Exnode_t,
    mut arg: libc::c_int,
    mut disc: *mut Exdisc_t,
) -> libc::c_int {
    let mut lobjp: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut robjp: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if (*l).type_0 > 258 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !r.is_null() && (*r).type_0 > 258 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !((*ex).type_0 >= 259 as libc::c_int && (*ex).type_0 <= 261 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if (*l).type_0 == T_tvtyp as libc::c_int {
        let mut li: libc::c_int = 0;
        let mut ri: libc::c_int = 0;
        if r.is_null() {
            return -(1 as libc::c_int);
        }
        if (*r).type_0 != T_tvtyp as libc::c_int {
            return -(1 as libc::c_int);
        }
        li = (*l).data.constant.value.integer as libc::c_int;
        ri = (*r).data.constant.value.integer as libc::c_int;
        match (*ex).op {
            325 => {
                if arg != 0 {
                    return 0 as libc::c_int;
                }
                (*l).data.constant.value.integer = (li == ri) as libc::c_int as libc::c_longlong;
                ret = 0 as libc::c_int;
            }
            326 => {
                if arg != 0 {
                    return 0 as libc::c_int;
                }
                (*l).data.constant.value.integer = (li != ri) as libc::c_int as libc::c_longlong;
                ret = 0 as libc::c_int;
            }
            60 => {
                if arg != 0 {
                    return 0 as libc::c_int;
                }
                (*l).data.constant.value.integer = (li < ri) as libc::c_int as libc::c_longlong;
                ret = 0 as libc::c_int;
            }
            327 => {
                if arg != 0 {
                    return 0 as libc::c_int;
                }
                (*l).data.constant.value.integer = (li <= ri) as libc::c_int as libc::c_longlong;
                ret = 0 as libc::c_int;
            }
            328 => {
                if arg != 0 {
                    return 0 as libc::c_int;
                }
                (*l).data.constant.value.integer = (li >= ri) as libc::c_int as libc::c_longlong;
                ret = 0 as libc::c_int;
            }
            62 => {
                if arg != 0 {
                    return 0 as libc::c_int;
                }
                (*l).data.constant.value.integer = (li > ri) as libc::c_int as libc::c_longlong;
                ret = 0 as libc::c_int;
            }
            _ => {}
        }
    }
    if !r.is_null() && (*r).type_0 == T_tvtyp as libc::c_int {
        return -(1 as libc::c_int);
    }
    lobjp = int2ptr((*l).data.constant.value.integer) as *mut Agobj_t;
    if !r.is_null() {
        robjp = int2ptr((*r).data.constant.value.integer) as *mut Agobj_t;
    } else {
        robjp = 0 as *mut Agobj_t;
    }
    match (*ex).op {
        325 => {
            if arg != 0 {
                return 0 as libc::c_int;
            }
            (*l).data.constant.value.integer =
                (compare(lobjp, robjp) == 0) as libc::c_int as libc::c_longlong;
            ret = 0 as libc::c_int;
        }
        326 => {
            if arg != 0 {
                return 0 as libc::c_int;
            }
            (*l).data.constant.value.integer = compare(lobjp, robjp) as libc::c_longlong;
            ret = 0 as libc::c_int;
        }
        60 => {
            if arg != 0 {
                return 0 as libc::c_int;
            }
            (*l).data.constant.value.integer =
                (compare(lobjp, robjp) < 0 as libc::c_int) as libc::c_int as libc::c_longlong;
            ret = 0 as libc::c_int;
        }
        327 => {
            if arg != 0 {
                return 0 as libc::c_int;
            }
            (*l).data.constant.value.integer =
                (compare(lobjp, robjp) <= 0 as libc::c_int) as libc::c_int as libc::c_longlong;
            ret = 0 as libc::c_int;
        }
        328 => {
            if arg != 0 {
                return 0 as libc::c_int;
            }
            (*l).data.constant.value.integer =
                (compare(lobjp, robjp) >= 0 as libc::c_int) as libc::c_int as libc::c_longlong;
            ret = 0 as libc::c_int;
        }
        62 => {
            if arg != 0 {
                return 0 as libc::c_int;
            }
            (*l).data.constant.value.integer =
                (compare(lobjp, robjp) > 0 as libc::c_int) as libc::c_int as libc::c_longlong;
            ret = 0 as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn strToTvtype(mut s: *mut libc::c_char) -> libc::c_int {
    let mut rt: libc::c_int = 0 as libc::c_int;
    let mut sfx: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncmp(
        s,
        b"TV_\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        sfx = s.offset(3 as libc::c_int as isize);
        if strcmp(sfx, b"flat\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_flat as libc::c_int;
        } else if strcmp(sfx, b"ne\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_ne as libc::c_int;
        } else if strcmp(sfx, b"en\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_en as libc::c_int;
        } else if strcmp(sfx, b"bfs\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_bfs as libc::c_int;
        } else if strcmp(sfx, b"dfs\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_dfs as libc::c_int;
        } else if strcmp(sfx, b"fwd\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_fwd as libc::c_int;
        } else if strcmp(sfx, b"rev\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_rev as libc::c_int;
        } else if strcmp(sfx, b"postdfs\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_postdfs as libc::c_int;
        } else if strcmp(sfx, b"postfwd\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_postfwd as libc::c_int;
        } else if strcmp(sfx, b"postrev\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_postrev as libc::c_int;
        } else if strcmp(sfx, b"prepostdfs\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_prepostdfs as libc::c_int;
        } else if strcmp(sfx, b"prepostfwd\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_prepostfwd as libc::c_int;
        } else if strcmp(sfx, b"prepostrev\0" as *const u8 as *const libc::c_char) == 0 {
            rt = TV_prepostrev as libc::c_int;
        } else {
            exerror(
                b"illegal string \"%s\" for type tvtype_t\0" as *const u8 as *const libc::c_char,
                s,
            );
        }
    } else {
        exerror(
            b"illegal string \"%s\" for type tvtype_t\0" as *const u8 as *const libc::c_char,
            s,
        );
    }
    return rt;
}
unsafe extern "C" fn tvtypeToStr(mut v: libc::c_int) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match v {
        0 => {
            s = b"TV_flat\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        1 => {
            s = b"TV_ne\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            s = b"TV_en\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        3 => {
            s = b"TV_bfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        4 => {
            s = b"TV_dfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        5 => {
            s = b"TV_fwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        6 => {
            s = b"TV_rev\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        7 => {
            s = b"TV_postdfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        8 => {
            s = b"TV_postfwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        9 => {
            s = b"TV_postrev\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        10 => {
            s = b"TV_prepostdfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        11 => {
            s = b"TV_prepostfwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        12 => {
            s = b"TV_prepostrev\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            exerror(
                b"Unexpected value %d for type tvtype_t\0" as *const u8 as *const libc::c_char,
                v,
            );
        }
    }
    return s;
}
unsafe extern "C" fn stringOf(
    mut prog: *mut Expr_t,
    mut x: *mut Exnode_t,
    mut arg: libc::c_int,
    mut disc: *mut Exdisc_t,
) -> libc::c_int {
    let mut objp: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut rv: libc::c_int = 0 as libc::c_int;
    if arg != 0 {
        return 0 as libc::c_int;
    }
    if (*x).type_0 == T_tvtyp as libc::c_int {
        let ref mut fresh18 = (*x).data.constant.value.string;
        *fresh18 = tvtypeToStr((*x).data.constant.value.integer as libc::c_int);
        if (*fresh18).is_null() {
            rv = -(1 as libc::c_int);
        }
    } else {
        objp = int2ptr((*x).data.constant.value.integer) as *mut Agobj_t;
        if objp.is_null() {
            exerror(
                b"cannot generate name for NULL %s\0" as *const u8 as *const libc::c_char,
                typeName(prog, (*x).type_0),
            );
            rv = -(1 as libc::c_int);
        } else {
            let mut state: *mut Gpr_t = (*disc).user as *mut Gpr_t;
            let ref mut fresh19 = (*x).data.constant.value.string;
            *fresh19 = nameOf(prog, objp, (*state).tmp);
        }
    }
    (*x).type_0 = 263 as libc::c_int;
    return rv;
}
unsafe extern "C" fn convert(
    mut prog: *mut Expr_t,
    mut x: *mut Exnode_t,
    mut type_0: libc::c_int,
    mut xref: *mut Exid_t,
    mut arg: libc::c_int,
    mut disc: *mut Exdisc_t,
) -> libc::c_int {
    let mut objp: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if type_0 > 258 as libc::c_int && (*x).type_0 > 258 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if type_0 == T_obj as libc::c_int && (*x).type_0 <= T_obj as libc::c_int {
        ret = 0 as libc::c_int;
    } else if type_0 <= T_obj as libc::c_int && (*x).type_0 == 259 as libc::c_int {
        if (*x).data.constant.value.integer == 0 as libc::c_int as libc::c_longlong {
            ret = 0 as libc::c_int;
        }
    } else if type_0 == 259 as libc::c_int {
        ret = 0 as libc::c_int;
    } else if (*x).type_0 == T_obj as libc::c_int {
        if arg != 0 {
            if type_0 != 262 as libc::c_int && type_0 <= T_obj as libc::c_int {
                ret = 0 as libc::c_int;
            }
        } else {
            objp = int2ptr((*x).data.constant.value.integer) as *mut Agobj_t;
            match type_0 {
                29 => {
                    if objp.is_null() || ((*objp).tag).objtype() as libc::c_int == 0 as libc::c_int
                    {
                        ret = 0 as libc::c_int;
                    }
                }
                27 => {
                    if objp.is_null() || ((*objp).tag).objtype() as libc::c_int == 1 as libc::c_int
                    {
                        ret = 0 as libc::c_int;
                    }
                }
                28 => {
                    if objp.is_null() || isedge(objp) != 0 {
                        ret = 0 as libc::c_int;
                    }
                }
                _ => {}
            }
        }
    } else if type_0 == 263 as libc::c_int {
        if (*x).type_0 == T_tvtyp as libc::c_int {
            ret = 0 as libc::c_int;
            if arg == 0 {
                let ref mut fresh20 = (*x).data.constant.value.string;
                *fresh20 = tvtypeToStr((*x).data.constant.value.integer as libc::c_int);
            }
        }
    } else if type_0 == T_tvtyp as libc::c_int && (*x).type_0 == 259 as libc::c_int {
        if arg != 0 {
            ret = 0 as libc::c_int;
        } else if validTVT((*x).data.constant.value.integer as libc::c_int) != 0 {
            ret = 0 as libc::c_int;
        } else {
            exerror(
                b"Integer value %ld not legal for type tvtype_t\0" as *const u8
                    as *const libc::c_char,
                (*x).data.constant.value.integer as intmax_t,
            );
        }
    } else if (*x).type_0 == type_0 {
        ret = 0 as libc::c_int;
    } else if (*x).type_0 == 263 as libc::c_int {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        if type_0 == T_tvtyp as libc::c_int {
            if arg != 0 {
                ret = 0 as libc::c_int;
            } else {
                ret = 0 as libc::c_int;
                s = (*x).data.constant.value.string;
                (*x).data.constant.value.integer = strToTvtype(s) as libc::c_longlong;
            }
        }
    }
    if arg == 0 && ret == 0 as libc::c_int {
        (*x).type_0 = type_0;
    }
    return ret;
}
unsafe extern "C" fn keyval(
    mut pgm: *mut Expr_t,
    mut v: Extype_t,
    mut type_0: libc::c_int,
    mut disc: *mut Exdisc_t,
) -> Extype_t {
    if type_0 <= T_obj as libc::c_int {
        v.integer = (*(int2ptr(v.integer) as *mut Agobj_t)).tag.id as libc::c_longlong;
    }
    return v;
}
unsafe extern "C" fn matchval(
    mut pgm: *mut Expr_t,
    mut xstr: *mut Exnode_t,
    mut str: *const libc::c_char,
    mut xpat: *mut Exnode_t,
    mut pat: *const libc::c_char,
    mut env: *mut libc::c_void,
    mut disc: *mut Exdisc_t,
) -> libc::c_int {
    return strgrpmatch(
        str,
        pat,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int,
    );
}
static mut a2t: [libc::c_int; 8] = [
    0 as libc::c_int,
    262 as libc::c_int,
    259 as libc::c_int,
    263 as libc::c_int,
    T_node as libc::c_int,
    T_edge as libc::c_int,
    T_graph as libc::c_int,
    T_obj as libc::c_int,
];
unsafe extern "C" fn initDisc(mut state: *mut Gpr_t) -> *mut Exdisc_t {
    let mut dp: *mut Exdisc_t = 0 as *mut Exdisc_t;
    dp = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<Exdisc_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Exdisc_t
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<Exdisc_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Exdisc_t
    };
    if dp.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"could not create libexp discipline: out of memory\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut Exdisc_t;
    }
    (*dp).version = 20000101 as libc::c_long as uint64_t;
    (*dp).flags = ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 9 as libc::c_int)
        as uint64_t;
    let ref mut fresh21 = (*dp).symbols;
    *fresh21 = symbols.as_mut_ptr();
    let ref mut fresh22 = (*dp).convertf;
    *fresh22 = Some(
        convert
            as unsafe extern "C" fn(
                *mut Expr_t,
                *mut Exnode_t,
                libc::c_int,
                *mut Exid_t,
                libc::c_int,
                *mut Exdisc_t,
            ) -> libc::c_int,
    );
    let ref mut fresh23 = (*dp).stringof;
    *fresh23 = Some(
        stringOf
            as unsafe extern "C" fn(
                *mut Expr_t,
                *mut Exnode_t,
                libc::c_int,
                *mut Exdisc_t,
            ) -> libc::c_int,
    );
    let ref mut fresh24 = (*dp).binaryf;
    *fresh24 = Some(
        binary
            as unsafe extern "C" fn(
                *mut Expr_t,
                *mut Exnode_t,
                *mut Exnode_t,
                *mut Exnode_t,
                libc::c_int,
                *mut Exdisc_t,
            ) -> libc::c_int,
    );
    let ref mut fresh25 = (*dp).typename;
    *fresh25 =
        Some(typeName as unsafe extern "C" fn(*mut Expr_t, libc::c_int) -> *mut libc::c_char);
    if ((*state).errf).is_some() {
        let ref mut fresh26 = (*dp).errorf;
        *fresh26 = (*state).errf;
    } else {
        let ref mut fresh27 = (*dp).errorf;
        *fresh27 = ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_int,
                    *const libc::c_char,
                    ...
                ) -> (),
            >,
            Exerror_f,
        >(Some(
            errorf
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_int,
                    *const libc::c_char,
                    ...
                ) -> (),
        ));
    }
    let ref mut fresh28 = (*dp).keyf;
    *fresh28 = Some(
        keyval
            as unsafe extern "C" fn(*mut Expr_t, Extype_t, libc::c_int, *mut Exdisc_t) -> Extype_t,
    );
    let ref mut fresh29 = (*dp).getf;
    *fresh29 = Some(
        getval
            as unsafe extern "C" fn(
                *mut Expr_t,
                *mut Exnode_t,
                *mut Exid_t,
                *mut Exref_t,
                *mut libc::c_void,
                libc::c_int,
                *mut Exdisc_t,
            ) -> Extype_t,
    );
    let ref mut fresh30 = (*dp).reff;
    *fresh30 = Some(
        refval
            as unsafe extern "C" fn(
                *mut Expr_t,
                *mut Exnode_t,
                *mut Exid_t,
                *mut Exref_t,
                *mut libc::c_char,
                libc::c_int,
                *mut Exdisc_t,
            ) -> Extype_t,
    );
    let ref mut fresh31 = (*dp).setf;
    *fresh31 = Some(
        setval
            as unsafe extern "C" fn(
                *mut Expr_t,
                *mut Exnode_t,
                *mut Exid_t,
                *mut Exref_t,
                *mut libc::c_void,
                libc::c_int,
                Extype_t,
                *mut Exdisc_t,
            ) -> libc::c_int,
    );
    let ref mut fresh32 = (*dp).matchf;
    *fresh32 = Some(
        matchval
            as unsafe extern "C" fn(
                *mut Expr_t,
                *mut Exnode_t,
                *const libc::c_char,
                *mut Exnode_t,
                *const libc::c_char,
                *mut libc::c_void,
                *mut Exdisc_t,
            ) -> libc::c_int,
    );
    let ref mut fresh33 = (*dp).exitf;
    *fresh33 = (*state).exitf;
    let ref mut fresh34 = (*dp).types;
    *fresh34 = a2t.as_mut_ptr();
    let ref mut fresh35 = (*dp).user;
    *fresh35 = state as *mut libc::c_void;
    let ref mut fresh36 = (*state).dp;
    *fresh36 = dp;
    return dp;
}
unsafe extern "C" fn compile(
    mut prog: *mut Expr_t,
    mut src: *mut libc::c_char,
    mut input: *mut libc::c_char,
    mut line: libc::c_int,
    mut lbl: *const libc::c_char,
    mut sfx: *const libc::c_char,
    mut kind: libc::c_int,
) -> *mut Exnode_t {
    let mut e: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut sf: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut prefix: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut rv: libc::c_int = 0;
    if !sfx.is_null() {
        sf = sfopen(
            0 as *mut Sfio_t,
            sfx,
            b"rs\0" as *const u8 as *const libc::c_char,
        );
        if !input.is_null() {
            prefix = sfopen(
                0 as *mut Sfio_t,
                input,
                b"rs\0" as *const u8 as *const libc::c_char,
            );
            sfstack(sf, prefix);
        }
    } else {
        sf = sfopen(
            0 as *mut Sfio_t,
            input,
            b"rs\0" as *const u8 as *const libc::c_char,
        );
    }
    if !lbl.is_null() {
        prefix = sfopen(
            0 as *mut Sfio_t,
            0 as *const libc::c_char,
            b"sr+\0" as *const u8 as *const libc::c_char,
        );
        sfprintf(prefix, b"%s:\n\0" as *const u8 as *const libc::c_char, lbl);
        sfseek(
            prefix,
            0 as libc::c_int as libc::c_longlong,
            0 as libc::c_int,
        );
        sfstack(sf, prefix);
        line -= 1;
    }
    if src.is_null() {
        src = b"<command line>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    rv = excomp(prog, src, line, 0 as *const libc::c_char, sf);
    sfclose(sf);
    if rv >= 0 as libc::c_int && getErrorErrors() == 0 as libc::c_int {
        e = exexpr(prog, lbl, 0 as *mut Exid_t, kind);
    }
    return e;
}
unsafe extern "C" fn checkGuard(
    mut gp: *mut Exnode_t,
    mut src: *mut libc::c_char,
    mut line: libc::c_int,
) {
    gp = exnoncast(gp);
    if !gp.is_null() && exisAssign(gp) != 0 {
        if !src.is_null() {
            setErrorFileLine(src, line);
        }
        _err_msg(
            1 as libc::c_int,
            b"assignment used as bool in guard\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn mkStmts(
    mut prog: *mut Expr_t,
    mut src: *mut libc::c_char,
    mut sp: *mut case_info,
    mut cnt: libc::c_int,
    mut lbl: *const libc::c_char,
) -> *mut case_stmt {
    let mut cs: *mut case_stmt = 0 as *mut case_stmt;
    let mut i: libc::c_int = 0;
    static mut LONGEST_CALLER_PREFIX: [libc::c_char; 10] =
        unsafe { *::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"_begin_g_\0") };
    static mut LONGEST_INFIX: [libc::c_char; 4] =
        unsafe { *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"__a\0") };
    let mut tmp: [libc::c_char; 35] = [0; 35];
    if (strlen(lbl))
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(CHARS_FOR_NUL_TERM_INT as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        <= ::std::mem::size_of::<[libc::c_char; 35]>() as libc::c_ulong
    {
    } else {
        __assert_fail(
            b"strlen(lbl) + sizeof(LONGEST_INFIX) - 1 + CHARS_FOR_NUL_TERM_INT - 1 + 1 <= sizeof(tmp)\0"
                as *const u8 as *const libc::c_char,
            b"compile.c\0" as *const u8 as *const libc::c_char,
            2354 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"case_stmt *mkStmts(Expr_t *, char *, case_info *, int, const char *)\0"))
                .as_ptr(),
        );
    }
    cs = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<case_stmt>() as libc::c_ulong)
                .wrapping_mul(cnt as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut case_stmt
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<case_stmt>() as libc::c_ulong)
                .wrapping_mul(cnt as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut case_stmt
    };
    i = 0 as libc::c_int;
    while i < cnt {
        if !((*sp).guard).is_null() {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 35]>() as libc::c_ulong,
                b"%s_g%d\0" as *const u8 as *const libc::c_char,
                lbl,
                i,
            );
            let ref mut fresh37 = (*cs.offset(i as isize)).guard;
            *fresh37 = compile(
                prog,
                src,
                (*sp).guard,
                (*sp).gstart,
                tmp.as_mut_ptr(),
                0 as *const libc::c_char,
                259 as libc::c_int,
            );
            if getErrorErrors() != 0 {
                break;
            }
            checkGuard((*cs.offset(i as isize)).guard, src, (*sp).gstart);
        }
        if !((*sp).action).is_null() {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 35]>() as libc::c_ulong,
                b"%s_a%d\0" as *const u8 as *const libc::c_char,
                lbl,
                i,
            );
            let ref mut fresh38 = (*cs.offset(i as isize)).action;
            *fresh38 = compile(
                prog,
                src,
                (*sp).action,
                (*sp).astart,
                tmp.as_mut_ptr(),
                0 as *const libc::c_char,
                259 as libc::c_int,
            );
            if getErrorErrors() != 0 {
                break;
            }
            if ((*cs.offset(i as isize)).action).is_null() {
                snprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 35]>() as libc::c_ulong,
                    b"%s__a%d\0" as *const u8 as *const libc::c_char,
                    lbl,
                    i,
                );
                let ref mut fresh39 = (*cs.offset(i as isize)).action;
                *fresh39 = compile(
                    prog,
                    src,
                    b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*sp).astart,
                    tmp.as_mut_ptr(),
                    0 as *const libc::c_char,
                    259 as libc::c_int,
                );
            }
        }
        sp = (*sp).next;
        i += 1;
    }
    return cs;
}
unsafe extern "C" fn mkBlock(
    mut bp: *mut comp_block,
    mut prog: *mut Expr_t,
    mut src: *mut libc::c_char,
    mut inp: *mut parse_block,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rv: libc::c_int = 0 as libc::c_int;
    codePhase = 1 as libc::c_int;
    if !((*inp).begg_stmt).is_null() {
        static mut PREFIX: [libc::c_char; 10] =
            unsafe { *::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"_begin_g_\0") };
        let mut label: [libc::c_char; 21] = [0; 21];
        snprintf(
            label.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
            b"%s%d\0" as *const u8 as *const libc::c_char,
            PREFIX.as_ptr(),
            i,
        );
        symbols[0 as libc::c_int as usize].type_0 = T_graph as libc::c_int as libc::c_long;
        tchk[V_this as libc::c_int as usize][1 as libc::c_int as usize] =
            ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype;
        let ref mut fresh40 = (*bp).begg_stmt;
        *fresh40 = compile(
            prog,
            src,
            (*inp).begg_stmt,
            (*inp).l_beging,
            label.as_mut_ptr(),
            0 as *const libc::c_char,
            264 as libc::c_int,
        );
        if getErrorErrors() != 0 {
            current_block = 8751752917435436902;
        } else {
            rv |= 0x2 as libc::c_int;
            current_block = 3276175668257526147;
        }
    } else {
        current_block = 3276175668257526147;
    }
    match current_block {
        3276175668257526147 => {
            codePhase = 2 as libc::c_int;
            if !((*inp).node_stmts).is_null() {
                static mut PREFIX_0: [libc::c_char; 4] =
                    unsafe { *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"_nd\0") };
                let mut label_0: [libc::c_char; 15] = [0; 15];
                symbols[0 as libc::c_int as usize].type_0 = T_node as libc::c_int as libc::c_long;
                tchk[V_this as libc::c_int as usize][1 as libc::c_int as usize] =
                    ((1 as libc::c_int) << 0x4 as libc::c_int) as tctype;
                (*bp).n_nstmts = (*inp).n_nstmts;
                snprintf(
                    label_0.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong,
                    b"%s%d\0" as *const u8 as *const libc::c_char,
                    PREFIX_0.as_ptr(),
                    i,
                );
                let ref mut fresh41 = (*bp).node_stmts;
                *fresh41 = mkStmts(
                    prog,
                    src,
                    (*inp).node_stmts,
                    (*inp).n_nstmts,
                    label_0.as_mut_ptr(),
                );
                if getErrorErrors() != 0 {
                    current_block = 8751752917435436902;
                } else {
                    (*bp).walks |= 0x1 as libc::c_int;
                    current_block = 6057473163062296781;
                }
            } else {
                current_block = 6057473163062296781;
            }
            match current_block {
                8751752917435436902 => {}
                _ => {
                    codePhase = 3 as libc::c_int;
                    if !((*inp).edge_stmts).is_null() {
                        static mut PREFIX_1: [libc::c_char; 4] = unsafe {
                            *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"_eg\0")
                        };
                        let mut label_1: [libc::c_char; 15] = [0; 15];
                        symbols[0 as libc::c_int as usize].type_0 =
                            T_edge as libc::c_int as libc::c_long;
                        tchk[V_this as libc::c_int as usize][1 as libc::c_int as usize] =
                            ((1 as libc::c_int) << 0x5 as libc::c_int) as tctype;
                        (*bp).n_estmts = (*inp).n_estmts;
                        snprintf(
                            label_1.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong,
                            b"%s%d\0" as *const u8 as *const libc::c_char,
                            PREFIX_1.as_ptr(),
                            i,
                        );
                        let ref mut fresh42 = (*bp).edge_stmts;
                        *fresh42 = mkStmts(
                            prog,
                            src,
                            (*inp).edge_stmts,
                            (*inp).n_estmts,
                            label_1.as_mut_ptr(),
                        );
                        if !(getErrorErrors() != 0) {
                            (*bp).walks |= 0x1 as libc::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if getErrorErrors() != 0 {
        free((*bp).node_stmts as *mut libc::c_void);
        free((*bp).edge_stmts as *mut libc::c_void);
        let ref mut fresh43 = (*bp).node_stmts;
        *fresh43 = 0 as *mut case_stmt;
        let ref mut fresh44 = (*bp).edge_stmts;
        *fresh44 = 0 as *mut case_stmt;
    }
    return rv | (*bp).walks;
}
unsafe extern "C" fn doFlags(mut flags: libc::c_int) -> *const libc::c_char {
    if flags & 0x1 as libc::c_int != 0 {
        if flags & 0x2 as libc::c_int != 0 {
            return b"\n$O = $G;\ninduce($O);\n\0" as *const u8 as *const libc::c_char;
        }
        return b"\n$O = $G;\n\0" as *const u8 as *const libc::c_char;
    }
    if flags & 0x2 as libc::c_int != 0 {
        return b"\ninduce($O);\n\0" as *const u8 as *const libc::c_char;
    }
    return b"\n\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn compileProg(
    mut inp: *mut parse_prog,
    mut state: *mut Gpr_t,
    mut flags: libc::c_int,
) -> *mut comp_prog {
    let mut current_block: u64;
    let mut p: *mut comp_prog = 0 as *mut comp_prog;
    let mut endg_sfx: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut useflags: libc::c_int = 0 as libc::c_int;
    let ref mut fresh45 = (*state).dfltIO;
    *fresh45 = &mut gprIoDisc;
    if (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<tctype>() as libc::c_ulong)
        >= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong
    {
    } else {
        __assert_fail(
            b"BITS_PER_BYTE * sizeof(tctype) >= (1 << TBITS)\0" as *const u8 as *const libc::c_char,
            b"compile.c\0" as *const u8 as *const libc::c_char,
            2473 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"comp_prog *compileProg(parse_prog *, Gpr_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    p = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<comp_prog>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut comp_prog
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<comp_prog>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut comp_prog
    };
    if p.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"could not create compiled program: out of memory\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        if flags != 0 {
            endg_sfx = doFlags(flags);
        }
        if !(initDisc(state)).is_null() {
            exinit();
            let ref mut fresh46 = (*p).prog;
            *fresh46 = exopen((*state).dp);
            if !(*fresh46).is_null() {
                codePhase = 0 as libc::c_int;
                if !((*inp).begin_stmt).is_null() {
                    let ref mut fresh47 = (*p).begin_stmt;
                    *fresh47 = compile(
                        (*p).prog,
                        (*inp).source,
                        (*inp).begin_stmt,
                        (*inp).l_begin,
                        0 as *const libc::c_char,
                        0 as *const libc::c_char,
                        264 as libc::c_int,
                    );
                    if getErrorErrors() != 0 {
                        current_block = 627910762490817868;
                    } else {
                        current_block = 15976848397966268834;
                    }
                } else {
                    current_block = 15976848397966268834;
                }
                match current_block {
                    627910762490817868 => {}
                    _ => {
                        if !((*inp).blocks).is_null() {
                            let mut bp: *mut comp_block = 0 as *mut comp_block;
                            let mut ibp: *mut parse_block = (*inp).blocks;
                            bp = if 0 as libc::c_int != 0 {
                                realloc(
                                    0 as *mut libc::c_char as *mut libc::c_void,
                                    (::std::mem::size_of::<comp_block>() as libc::c_ulong)
                                        .wrapping_mul((*inp).n_blocks as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut comp_block
                            } else {
                                calloc(
                                    1 as libc::c_int as libc::c_ulong,
                                    (::std::mem::size_of::<comp_block>() as libc::c_ulong)
                                        .wrapping_mul((*inp).n_blocks as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut comp_block
                            };
                            let ref mut fresh48 = (*p).blocks;
                            *fresh48 = bp;
                            i = 0 as libc::c_int;
                            loop {
                                if !(i < (*inp).n_blocks) {
                                    current_block = 15345278821338558188;
                                    break;
                                }
                                useflags |= mkBlock(bp, (*p).prog, (*inp).source, ibp, i);
                                if getErrorErrors() != 0 {
                                    current_block = 627910762490817868;
                                    break;
                                }
                                ibp = (*ibp).next;
                                let ref mut fresh49 = (*p).n_blocks;
                                *fresh49 += 1;
                                bp = bp.offset(1);
                                i += 1;
                            }
                        } else {
                            current_block = 15345278821338558188;
                        }
                        match current_block {
                            627910762490817868 => {}
                            _ => {
                                (*p).flags = useflags;
                                codePhase = 4 as libc::c_int;
                                if !((*inp).endg_stmt).is_null() || !endg_sfx.is_null() {
                                    symbols[0 as libc::c_int as usize].type_0 =
                                        T_graph as libc::c_int as libc::c_long;
                                    tchk[V_this as libc::c_int as usize]
                                        [1 as libc::c_int as usize] =
                                        ((1 as libc::c_int) << 0x6 as libc::c_int) as tctype;
                                    let ref mut fresh50 = (*p).endg_stmt;
                                    *fresh50 = compile(
                                        (*p).prog,
                                        (*inp).source,
                                        (*inp).endg_stmt,
                                        (*inp).l_endg,
                                        b"_end_g\0" as *const u8 as *const libc::c_char,
                                        endg_sfx,
                                        264 as libc::c_int,
                                    );
                                    if getErrorErrors() != 0 {
                                        current_block = 627910762490817868;
                                    } else {
                                        current_block = 9520865839495247062;
                                    }
                                } else {
                                    current_block = 9520865839495247062;
                                }
                                match current_block {
                                    627910762490817868 => {}
                                    _ => {
                                        codePhase = 5 as libc::c_int;
                                        if !((*inp).end_stmt).is_null() {
                                            symbols[0 as libc::c_int as usize].type_0 =
                                                T_obj as libc::c_int as libc::c_long;
                                            let ref mut fresh51 = (*p).end_stmt;
                                            *fresh51 = compile(
                                                (*p).prog,
                                                (*inp).source,
                                                (*inp).end_stmt,
                                                (*inp).l_end,
                                                b"_end_\0" as *const u8 as *const libc::c_char,
                                                0 as *const libc::c_char,
                                                264 as libc::c_int,
                                            );
                                            if getErrorErrors() != 0 {
                                                current_block = 627910762490817868;
                                            } else {
                                                current_block = 15090052786889560393;
                                            }
                                        } else {
                                            current_block = 15090052786889560393;
                                        }
                                        match current_block {
                                            627910762490817868 => {}
                                            _ => {
                                                setErrorLine(0 as libc::c_int);
                                                if !((*p).end_stmt).is_null() {
                                                    (*p).flags |= 0x4 as libc::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if getErrorErrors() != 0 {
        freeCompileProg(p);
        p = 0 as *mut comp_prog;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn freeCompileProg(mut p: *mut comp_prog) {
    let mut bp: *mut comp_block = 0 as *mut comp_block;
    let mut i: libc::c_int = 0;
    if p.is_null() {
        return;
    }
    exclose((*p).prog, 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*p).n_blocks {
        bp = ((*p).blocks).offset(i as isize);
        free((*bp).node_stmts as *mut libc::c_void);
        free((*bp).edge_stmts as *mut libc::c_void);
        i += 1;
    }
    free((*p).blocks as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn walksGraph(mut p: *mut comp_block) -> libc::c_int {
    return (*p).walks;
}
#[no_mangle]
pub unsafe extern "C" fn usesGraph(mut p: *mut comp_prog) -> libc::c_int {
    return (*p).flags;
}
#[no_mangle]
pub unsafe extern "C" fn readG(mut fp: *mut Sfio_t) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    g = agread(fp as *mut libc::c_void, &mut gprDisc);
    if !g.is_null() {
        aginit(
            g,
            0 as libc::c_int,
            b"userval\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<gdata>() as libc::c_ulong as libc::c_int,
            0 as libc::c_int,
        );
        aginit(
            g,
            1 as libc::c_int,
            b"userval\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<ndata>() as libc::c_ulong as libc::c_int,
            0 as libc::c_int,
        );
        aginit(
            g,
            2 as libc::c_int,
            b"userval\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<edata>() as libc::c_ulong as libc::c_int,
            0 as libc::c_int,
        );
    }
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn openG(mut name: *mut libc::c_char, mut desc: Agdesc_t) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    g = agopen(name, desc, &mut gprDisc);
    if !g.is_null() {
        agbindrec(
            g as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<gdata>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        );
    }
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn openSubg(
    mut g: *mut Agraph_t,
    mut name: *mut libc::c_char,
) -> *mut Agraph_t {
    let mut sg: *mut Agraph_t = 0 as *mut Agraph_t;
    sg = agsubg(g, name, 1 as libc::c_int);
    if !sg.is_null()
        && (aggetrec(
            sg as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ))
        .is_null()
    {
        agbindrec(
            sg as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<gdata>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        );
    }
    return sg;
}
#[no_mangle]
pub unsafe extern "C" fn openNode(
    mut g: *mut Agraph_t,
    mut name: *mut libc::c_char,
) -> *mut Agnode_t {
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    np = agnode(g, name, 1 as libc::c_int);
    if !np.is_null()
        && (aggetrec(
            np as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ))
        .is_null()
    {
        agbindrec(
            np as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<ndata>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        );
    }
    return np;
}
#[no_mangle]
pub unsafe extern "C" fn openEdge(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut h: *mut Agnode_t,
    mut key: *mut libc::c_char,
) -> *mut Agedge_t {
    let mut ep: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut root: *mut Agraph_t = 0 as *mut Agraph_t;
    root = sameG(
        t as *mut libc::c_void,
        h as *mut libc::c_void,
        b"openEdge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"tail and head nodes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if root.is_null() {
        return 0 as *mut Agedge_t;
    }
    if !g.is_null() {
        if (sameG(
            g as *mut libc::c_void,
            root as *mut libc::c_void,
            b"openEdge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"subgraph and nodes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
        .is_null()
        {
            return 0 as *mut Agedge_t;
        }
    } else {
        g = root;
    }
    ep = agedge(g, t, h, key, 1 as libc::c_int);
    if !ep.is_null()
        && (aggetrec(
            ep as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ))
        .is_null()
    {
        agbindrec(
            ep as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<edata>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        );
    }
    return ep;
}
