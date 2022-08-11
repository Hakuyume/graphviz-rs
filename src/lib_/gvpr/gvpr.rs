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
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optopt: libc::c_int;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    static mut sfstdin: *mut Sfio_t;
    static mut sfstdout: *mut Sfio_t;
    static mut sfstderr: *mut Sfio_t;
    fn sfopen(_: *mut Sfio_t, _: *const libc::c_char, _: *const libc::c_char) -> *mut Sfio_t;
    fn sfdisc(_: *mut Sfio_t, _: *mut Sfdisc_t) -> *mut Sfdisc_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfprintf(_: *mut Sfio_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut Dtqueue: *mut Dtmethod_t;
    fn agsubnode(g: *mut Agraph_t, n: *mut Agnode_t, createflag: libc::c_int) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agsubedge(g: *mut Agraph_t, e: *mut Agedge_t, createflag: libc::c_int) -> *mut Agedge_t;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn aggetrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        move_to_front: libc::c_int,
    ) -> *mut Agrec_t;
    fn agsubg(g: *mut Agraph_t, name: *mut libc::c_char, cflag: libc::c_int) -> *mut Agraph_t;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn openGPRState(_: *mut gpr_info) -> *mut Gpr_t;
    fn addBindings(state: *mut Gpr_t, _: *mut gvprbinding);
    fn initGPRState(_: *mut Gpr_t, _: *mut Vmalloc_t);
    fn closeGPRState(state: *mut Gpr_t);
    fn exeval(_: *mut Expr_t, _: *mut Exnode_t, _: *mut libc::c_void) -> Extype_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn newIng(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: *mut ingdisc,
    ) -> *mut ingraph_state;
    fn newIngGraphs(
        _: *mut ingraph_state,
        _: *mut *mut Agraph_t,
        _: *mut ingdisc,
    ) -> *mut ingraph_state;
    fn closeIngraph(sp: *mut ingraph_state);
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    fn fileName(_: *mut ingraph_state) -> *mut libc::c_char;
    fn compileProg(_: *mut parse_prog, _: *mut Gpr_t, _: libc::c_int) -> *mut comp_prog;
    fn freeCompileProg(p: *mut comp_prog);
    fn usesGraph(_: *mut comp_prog) -> libc::c_int;
    fn walksGraph(_: *mut comp_block) -> libc::c_int;
    fn openSubg(g: *mut Agraph_t, name: *mut libc::c_char) -> *mut Agraph_t;
    fn readG(fp: *mut Sfio_t) -> *mut Agraph_t;
    fn parseProg(_: *mut libc::c_char, _: libc::c_int) -> *mut parse_prog;
    fn freeParseProg(_: *mut parse_prog);
    fn mkQ(_: *mut Dtmethod_t) -> *mut queue;
    fn push(_: *mut queue, _: *mut libc::c_void);
    fn pop(_: *mut queue, remove: libc::c_int) -> *mut libc::c_void;
    fn freeQ(_: *mut queue);
    fn clone(g: *mut Agraph_t, obj: *mut Agobj_t) -> *mut Agobj_t;
    fn sfioWrite(_: *mut Agraph_t, _: *mut Sfio_t, _: *mut Agiodisc_t) -> libc::c_int;
    fn gvstart_timer();
    fn gvelapsed_sec() -> libc::c_double;
    fn setTraceLevel(_: libc::c_int);
    fn setErrorId(_: *mut libc::c_char);
    fn setErrorErrors(_: libc::c_int);
    fn _err_msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn _err_msgv(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    );
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
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
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub struct _sfdisc_s {
    pub readf: Sfread_f,
    pub writef: Sfwrite_f,
    pub seekf: Sfseek_f,
    pub exceptf: Sfexcept_f,
    pub disc: *mut Sfdisc_t,
}
pub type Sfdisc_t = _sfdisc_s;
pub type Sfexcept_f = Option<
    unsafe extern "C" fn(*mut Sfio_t, libc::c_int, *mut libc::c_void, *mut Sfdisc_t) -> libc::c_int,
>;
pub type Sfseek_f = Option<
    unsafe extern "C" fn(
        *mut Sfio_t,
        libc::c_longlong,
        libc::c_int,
        *mut Sfdisc_t,
    ) -> libc::c_longlong,
>;
pub type Sfwrite_f = Option<
    unsafe extern "C" fn(*mut Sfio_t, *const libc::c_void, size_t, *mut Sfdisc_t) -> ssize_t,
>;
pub type Sfread_f =
    Option<unsafe extern "C" fn(*mut Sfio_t, *mut libc::c_void, size_t, *mut Sfdisc_t) -> ssize_t>;
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
pub struct Agedgepair_s {
    pub out: Agedge_t,
    pub in_0: Agedge_t,
}
pub type Agedgepair_t = Agedgepair_s;
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
pub type gvprwr = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        size_t,
        *mut libc::c_void,
    ) -> ssize_t,
>;
pub type gvpruserfn = Option<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvprbinding {
    pub name: *mut libc::c_char,
    pub fn_0: gvpruserfn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gvpropts {
    pub ingraphs: *mut *mut Agraph_t,
    pub n_outgraphs: libc::c_int,
    pub outgraphs: *mut *mut Agraph_t,
    pub out: gvprwr,
    pub err: gvprwr,
    pub flags: libc::c_int,
    pub bindings: *mut gvprbinding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub cmdName: *mut libc::c_char,
    pub outFile: *mut Sfio_t,
    pub program: *mut libc::c_char,
    pub useFile: libc::c_int,
    pub compflags: libc::c_int,
    pub readAhead: libc::c_int,
    pub inFiles: *mut *mut libc::c_char,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub state: libc::c_int,
    pub verbose: libc::c_int,
}
pub const _ISspace: C2RustUnnamed_9 = 8192;
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
pub struct ingraph_state {
    pub u: C2RustUnnamed_7,
    pub ctr: libc::c_int,
    pub ingraphs: libc::c_int,
    pub fp: *mut libc::c_void,
    pub fns: *mut ingdisc,
    pub heap: bool,
    pub errors: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ingdisc {
    pub openf: Option<unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_void>,
    pub readf: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut Agraph_t>,
    pub closef: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub dflt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub Files: *mut *mut libc::c_char,
    pub Graphs: *mut *mut Agraph_t,
}
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
pub struct comp_prog {
    pub flags: libc::c_int,
    pub prog: *mut Expr_t,
    pub begin_stmt: *mut Exnode_t,
    pub n_blocks: libc::c_int,
    pub blocks: *mut comp_block,
    pub endg_stmt: *mut Exnode_t,
    pub end_stmt: *mut Exnode_t,
}
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
pub struct case_stmt {
    pub guard: *mut Exnode_t,
    pub action: *mut Exnode_t,
}
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
pub type parse_block = _parse_block;
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
pub type case_info = _case_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _case_info {
    pub gstart: libc::c_int,
    pub guard: *mut libc::c_char,
    pub astart: libc::c_int,
    pub action: *mut libc::c_char,
    pub next: *mut _case_info,
}
pub type gval_t = gdata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdata {
    pub h: Agrec_t,
    pub lock: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trav_fns {
    pub fstedge: fstedgefn_t,
    pub nxtedge: nxttedgefn_t,
    pub undirected: libc::c_uchar,
    pub visit: libc::c_uchar,
}
pub type nxttedgefn_t =
    Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agedge_t, *mut Agnode_t) -> *mut Agedge_t>;
pub type fstedgefn_t = Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agnode_t) -> *mut Agedge_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_stack_t {
    pub base: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub type nval_t = ndata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndata {
    pub h: Agrec_t,
    pub iu: Extype_t,
    pub ine: *mut Agedge_t,
}
pub const FIRST_ALLOCATION: C2RustUnnamed_8 = 512;
pub type C2RustUnnamed_8 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodestream {
    pub oldroot: *mut Agnode_t,
    pub prev: *mut Agnode_t,
}
pub type queue = Dt_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gpr_info {
    pub outFile: *mut Sfio_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub errf: Exerror_f,
    pub exitf: Exexit_f,
    pub flags: libc::c_int,
}
pub type C2RustUnnamed_9 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_9 = 8;
pub const _ISpunct: C2RustUnnamed_9 = 4;
pub const _IScntrl: C2RustUnnamed_9 = 2;
pub const _ISblank: C2RustUnnamed_9 = 1;
pub const _ISgraph: C2RustUnnamed_9 = 32768;
pub const _ISprint: C2RustUnnamed_9 = 16384;
pub const _ISxdigit: C2RustUnnamed_9 = 4096;
pub const _ISdigit: C2RustUnnamed_9 = 2048;
pub const _ISalpha: C2RustUnnamed_9 = 1024;
pub const _ISlower: C2RustUnnamed_9 = 512;
pub const _ISupper: C2RustUnnamed_9 = 256;
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
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_alloc(mut size: size_t) -> *mut libc::c_void {
    return gv_calloc(1 as libc::c_int as size_t, size);
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
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh11 = (*xb).ptr;
    *fresh11 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn stack_size(mut stack: *const gv_stack_t) -> size_t {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"size_t stack_size(const gv_stack_t *)\0",
            ))
            .as_ptr(),
        );
    }
    return (*stack).size;
}
#[inline]
unsafe extern "C" fn stack_is_empty(mut stack: *const gv_stack_t) -> bool {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"_Bool stack_is_empty(const gv_stack_t *)\0",
            ))
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
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int stack_push(gv_stack_t *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*stack).size == (*stack).capacity {
        if ((18446744073709551615 as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            < (*stack).capacity) as libc::c_int as libc::c_long
            != 0
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
        if (b == 0 as *mut libc::c_void as *mut *mut libc::c_void) as libc::c_int as libc::c_long
            != 0
        {
            return 12 as libc::c_int;
        }
        (*stack).capacity = c;
        let ref mut fresh12 = (*stack).base;
        *fresh12 = b;
    }
    if !((*stack).base).is_null() {
    } else {
        __assert_fail(
            b"stack->base != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int stack_push(gv_stack_t *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    if (*stack).capacity > (*stack).size {
    } else {
        __assert_fail(
            b"stack->capacity > stack->size\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int stack_push(gv_stack_t *, void *)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh13 = *((*stack).base).offset((*stack).size as isize);
    *fresh13 = item;
    let ref mut fresh14 = (*stack).size;
    *fresh14 = (*fresh14).wrapping_add(1);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn stack_push_or_exit(mut stack: *mut gv_stack_t, mut item: *mut libc::c_void) {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void stack_push_or_exit(gv_stack_t *, void *)\0",
            ))
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
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void *stack_top(gv_stack_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if !stack_is_empty(stack)
        && !(b"access to top of an empty stack\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!stack_is_empty(stack) && \"access to top of an empty stack\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void *stack_top(gv_stack_t *)\0",
            ))
            .as_ptr(),
        );
    }
    return *((*stack).base)
        .offset(((*stack).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
}
#[inline]
unsafe extern "C" fn stack_pop(mut stack: *mut gv_stack_t) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = stack_top(stack);
    let ref mut fresh15 = (*stack).size;
    *fresh15 = (*fresh15).wrapping_sub(1);
    return top;
}
#[inline]
unsafe extern "C" fn stack_reset(mut stack: *mut gv_stack_t) {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack != NULL\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/stack.h\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"void stack_reset(gv_stack_t *)\0",
            ))
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
static mut Info: [*mut libc::c_char; 3] = [
    b"gvpr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"5.0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"20220707.1540\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut usage: *const libc::c_char = b" [-o <ofile>] [-a <args>] ([-f <prog>] | 'prog') [files]\n   -c         - use source graph for output\n   -f <pfile> - find program in file <pfile>\n   -i         - create node induced subgraph\n   -a <args>  - string arguments available as ARGV[0..]\n   -o <ofile> - write output to <ofile>; stdout by default\n   -n         - no read-ahead of input graphs\n   -q         - turn off warning messages\n   -V         - print version info\n   -?         - print usage info\nIf no files are specified, stdin is used\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn openOut(mut name: *mut libc::c_char) -> *mut Sfio_t {
    let mut outs: *mut Sfio_t = 0 as *mut Sfio_t;
    outs = sfopen(
        0 as *mut Sfio_t,
        name,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if outs.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"could not open %s for writing\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return outs;
}
unsafe extern "C" fn gettok(mut sp: *mut *mut libc::c_char) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = *sp;
    let mut ws: *mut libc::c_char = s;
    let mut rs: *mut libc::c_char = s;
    let mut c: libc::c_char = 0;
    let mut q: libc::c_char = '\0' as i32 as libc::c_char;
    while *(*__ctype_b_loc()).offset(*rs as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        rs = rs.offset(1);
    }
    c = *rs;
    if c as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    loop {
        c = *rs;
        if !(c != 0) {
            break;
        }
        if q as libc::c_int != 0 && q as libc::c_int == c as libc::c_int {
            q = '\0' as i32 as libc::c_char;
        } else if q == 0 && (c as libc::c_int == '"' as i32 || c as libc::c_int == '\'' as i32) {
            q = c;
        } else if c as libc::c_int == '\\' as i32 {
            rs = rs.offset(1);
            c = *rs;
            if c != 0 {
                let fresh16 = ws;
                ws = ws.offset(1);
                *fresh16 = c;
            } else {
                _err_msg(
                    1 as libc::c_int,
                    b"backslash in argument followed by no character - ignored\0" as *const u8
                        as *const libc::c_char,
                );
                rs = rs.offset(-1);
            }
        } else {
            if !(q as libc::c_int != 0
                || *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                    == 0)
            {
                break;
            }
            let fresh17 = ws;
            ws = ws.offset(1);
            *fresh17 = c;
        }
        rs = rs.offset(1);
    }
    if *rs != 0 {
        rs = rs.offset(1);
    } else if q != 0 {
        _err_msg(
            1 as libc::c_int,
            b"no closing quote for argument %s\0" as *const u8 as *const libc::c_char,
            s,
        );
    }
    *sp = rs;
    *ws = '\0' as i32 as libc::c_char;
    return s;
}
unsafe extern "C" fn parseArgs(
    mut s: *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut args: [*mut libc::c_char; 100] = [0 as *mut libc::c_char; 100];
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut av: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    loop {
        t = gettok(&mut s);
        if t.is_null() {
            break;
        }
        if cnt == 100 as libc::c_int {
            _err_msg(
                1 as libc::c_int,
                b"at most %d arguments allowed per -a flag - ignoring rest\0" as *const u8
                    as *const libc::c_char,
                100 as libc::c_int,
            );
            break;
        } else {
            let fresh18 = cnt;
            cnt = cnt + 1;
            args[fresh18 as usize] = t;
        }
    }
    if cnt != 0 {
        let mut oldcnt: libc::c_int = argc;
        argc = oldcnt + cnt;
        av = if !(*argv).is_null() {
            realloc(
                *argv as *mut libc::c_char as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(argc as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as libc::c_ulong),
            ) as *mut *mut libc::c_char
        } else {
            malloc(
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(argc as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as libc::c_ulong),
            ) as *mut *mut libc::c_char
        };
        i = 0 as libc::c_int;
        while i < cnt {
            let ref mut fresh19 = *av.offset((oldcnt + i) as isize);
            *fresh19 = gv_strdup(args[i as usize]);
            i += 1;
        }
        *argv = av;
    }
    return argc;
}
unsafe extern "C" fn concat(
    mut pfx: *mut libc::c_char,
    mut sfx: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut sp: *mut libc::c_char = gv_alloc(
        (strlen(pfx))
            .wrapping_add(strlen(sfx))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(sp, pfx);
    strcat(sp, sfx);
    return sp;
}
unsafe extern "C" fn resolve(
    mut arg: *mut libc::c_char,
    mut Verbose: libc::c_int,
) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut pathp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0;
    if !(strchr(arg, '/' as i32)).is_null() {
        return gv_strdup(arg);
    }
    path = getenv(b"GVPRPATH\0" as *const u8 as *const libc::c_char);
    if path.is_null() {
        path = getenv(b"GPRPATH\0" as *const u8 as *const libc::c_char);
    }
    if !path.is_null() && {
        c = *path;
        c as libc::c_int != 0
    } {
        if c as libc::c_int == ':' as i32 {
            path = concat(
                b".:/usr/local/share/graphviz/gvpr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                path,
            );
            pathp = path;
        } else {
            c = *path
                .offset((strlen(path)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            if c as libc::c_int == ':' as i32 {
                path = concat(
                    path,
                    b".:/usr/local/share/graphviz/gvpr\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                pathp = path;
            }
        }
    } else {
        path = b".:/usr/local/share/graphviz/gvpr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"PATH: %s\n\0" as *const u8 as *const libc::c_char,
            path,
        );
    }
    agxbinit(
        &mut fp,
        0 as libc::c_int as libc::c_uint,
        0 as *mut libc::c_char,
    );
    while *path as libc::c_int != 0 && fname.is_null() {
        if *path as libc::c_int == ':' as i32 {
            path = path.offset(1);
        } else {
            cp = strchr(path, ':' as i32);
            if !cp.is_null() {
                sz = cp.offset_from(path) as libc::c_long as size_t;
                agxbput_n(&mut fp, path, sz);
                path = cp.offset(1 as libc::c_int as isize);
            } else {
                sz = agxbput(&mut fp, path);
                path = path.offset(sz as isize);
            }
            agxbprint(
                &mut fp as *mut agxbuf,
                b"%c%s\0" as *const u8 as *const libc::c_char,
                '/' as i32,
                arg,
            );
            s = agxbuse(&mut fp);
            if access(s, 4 as libc::c_int) == 0 as libc::c_int {
                fname = gv_strdup(s);
            }
        }
    }
    if fname.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"Could not find file \"%s\" in GVPRPATH\0" as *const u8 as *const libc::c_char,
            arg,
        );
    }
    agxbfree(&mut fp);
    free(pathp as *mut libc::c_void);
    if Verbose != 0 {
        fprintf(
            stderr,
            b"file %s resolved to %s\n\0" as *const u8 as *const libc::c_char,
            arg,
            fname,
        );
    }
    return fname;
}
unsafe extern "C" fn getOptarg(
    mut c: libc::c_int,
    mut argp: *mut *mut libc::c_char,
    mut argip: *mut libc::c_int,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut libc::c_char = *argp;
    let mut argi: libc::c_int = *argip;
    if *arg != 0 {
        rv = arg;
        while *arg != 0 {
            arg = arg.offset(1);
        }
        *argp = arg;
    } else if argi < argc {
        let fresh20 = argi;
        argi = argi + 1;
        rv = *argv.offset(fresh20 as isize);
        *argip = argi;
    } else {
        rv = 0 as *mut libc::c_char;
        _err_msg(
            1 as libc::c_int,
            b"missing argument for option -%c\0" as *const u8 as *const libc::c_char,
            c,
        );
    }
    return rv;
}
unsafe extern "C" fn doFlags(
    mut arg: *mut libc::c_char,
    mut argi: libc::c_int,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut opts: *mut options,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        let fresh21 = arg;
        arg = arg.offset(1);
        c = *fresh21 as libc::c_int;
        if !(c != 0) {
            break;
        }
        match c {
            99 => {
                (*opts).compflags |= 0x1 as libc::c_int;
            }
            67 => {
                (*opts).compflags |= 0x1 as libc::c_int | 0x4 as libc::c_int;
            }
            102 => {
                optarg = getOptarg(c, &mut arg, &mut argi, argc, argv);
                if !optarg.is_null() && {
                    let ref mut fresh22 = (*opts).program;
                    *fresh22 = resolve(optarg, (*opts).verbose);
                    !(*fresh22).is_null()
                } {
                    (*opts).useFile = 1 as libc::c_int;
                } else {
                    return -(1 as libc::c_int);
                }
            }
            105 => {
                (*opts).compflags |= 0x2 as libc::c_int;
            }
            110 => {
                (*opts).readAhead = 0 as libc::c_int;
            }
            97 => {
                optarg = getOptarg(c, &mut arg, &mut argi, argc, argv);
                if !optarg.is_null() {
                    (*opts).argc = parseArgs(optarg, (*opts).argc, &mut (*opts).argv);
                } else {
                    return -(1 as libc::c_int);
                }
            }
            111 => {
                optarg = getOptarg(c, &mut arg, &mut argi, argc, argv);
                if optarg.is_null() || {
                    let ref mut fresh23 = (*opts).outFile;
                    *fresh23 = openOut(optarg);
                    (*fresh23).is_null()
                } {
                    return -(1 as libc::c_int);
                }
            }
            113 => {
                setTraceLevel(2 as libc::c_int);
            }
            118 => {
                (*opts).verbose = 1 as libc::c_int;
            }
            86 => {
                fprintf(
                    stderr,
                    b"%s version %s (%s)\n\0" as *const u8 as *const libc::c_char,
                    Info[0 as libc::c_int as usize],
                    Info[1 as libc::c_int as usize],
                    Info[2 as libc::c_int as usize],
                );
                return 0 as libc::c_int;
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    fprintf(
                        stderr,
                        b"Usage: gvpr%s\0" as *const u8 as *const libc::c_char,
                        usage,
                    );
                } else {
                    _err_msg(
                        0x800 as libc::c_int | 1 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        usage,
                    );
                }
                return 0 as libc::c_int;
            }
            _ => {
                _err_msg(
                    1 as libc::c_int,
                    b"option -%c unrecognized\0" as *const u8 as *const libc::c_char,
                    c,
                );
            }
        }
    }
    return argi;
}
unsafe extern "C" fn freeOpts(mut opts: options) {
    if opts.outFile != sfstdout {
        sfclose(opts.outFile);
    }
    free(opts.inFiles as *mut libc::c_void);
    if opts.useFile != 0 {
        free(opts.program as *mut libc::c_void);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < opts.argc {
        free(*(opts.argv).offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    free(opts.argv as *mut libc::c_void);
}
unsafe extern "C" fn scanArgs(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> options {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut nfiles: libc::c_int = 0;
    let mut input_filenames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opts: options = {
        let mut init = options {
            cmdName: 0 as *mut libc::c_char,
            outFile: 0 as *mut Sfio_t,
            program: 0 as *mut libc::c_char,
            useFile: 0,
            compflags: 0,
            readAhead: 0,
            inFiles: 0 as *mut *mut libc::c_char,
            argc: 0,
            argv: 0 as *mut *mut libc::c_char,
            state: 0,
            verbose: 0,
        };
        init
    };
    opts.cmdName = *argv.offset(0 as libc::c_int as isize);
    opts.state = 1 as libc::c_int;
    opts.readAhead = 1 as libc::c_int;
    setErrorId(opts.cmdName);
    opts.verbose = 0 as libc::c_int;
    nfiles = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if !(*argv.offset(i as isize)).is_null()
            && *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                != '-' as i32
        {
            nfiles += 1;
        }
        i += 1;
    }
    input_filenames = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((nfiles + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut *mut libc::c_char
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((nfiles + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut *mut libc::c_char
    };
    nfiles = 0 as libc::c_int;
    i = 1 as libc::c_int;
    loop {
        if !(i < argc) {
            current_block = 14401909646449704462;
            break;
        }
        let fresh24 = i;
        i = i + 1;
        arg = *argv.offset(fresh24 as isize);
        if *arg as libc::c_int == '-' as i32 {
            i = doFlags(
                arg.offset(1 as libc::c_int as isize),
                i,
                argc,
                argv,
                &mut opts,
            );
            if !(i <= 0 as libc::c_int) {
                continue;
            }
            opts.state = i;
            current_block = 17611144322713724474;
            break;
        } else if !arg.is_null() {
            let fresh25 = nfiles;
            nfiles = nfiles + 1;
            let ref mut fresh26 = *input_filenames.offset(fresh25 as isize);
            *fresh26 = arg;
        }
    }
    match current_block {
        14401909646449704462 => {
            if opts.useFile == 0 as libc::c_int {
                if nfiles == 0 as libc::c_int {
                    _err_msg(
                        2 as libc::c_int,
                        b"No program supplied via argument or -f option\0" as *const u8
                            as *const libc::c_char,
                    );
                    opts.state = -(1 as libc::c_int);
                } else {
                    opts.program = *input_filenames.offset(0 as libc::c_int as isize);
                    i = 1 as libc::c_int;
                    while i <= nfiles {
                        let ref mut fresh27 =
                            *input_filenames.offset((i - 1 as libc::c_int) as isize);
                        *fresh27 = *input_filenames.offset(i as isize);
                        i += 1;
                    }
                    nfiles -= 1;
                }
            }
            if nfiles == 0 as libc::c_int {
                opts.inFiles = 0 as *mut *mut libc::c_char;
                free(input_filenames as *mut libc::c_void);
                input_filenames = 0 as *mut *mut libc::c_char;
            } else {
                opts.inFiles = input_filenames;
            }
            if (opts.outFile).is_null() {
                opts.outFile = sfstdout;
            }
        }
        _ => {}
    }
    if opts.state <= 0 as libc::c_int {
        if opts.state < 0 as libc::c_int {
            _err_msg(
                0x800 as libc::c_int | 2 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                usage,
            );
        }
        free(input_filenames as *mut libc::c_void);
    }
    return opts;
}
unsafe extern "C" fn evalEdge(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut xprog: *mut comp_block,
    mut e: *mut Agedge_t,
) -> *mut Agobj_t {
    let mut i: libc::c_int = 0;
    let mut cs: *mut case_stmt = 0 as *mut case_stmt;
    let mut okay: bool = false;
    let ref mut fresh28 = (*state).curobj;
    *fresh28 = e as *mut Agobj_t;
    i = 0 as libc::c_int;
    while i < (*xprog).n_estmts {
        cs = ((*xprog).edge_stmts).offset(i as isize);
        if !((*cs).guard).is_null() {
            okay = (exeval(prog, (*cs).guard, state as *mut libc::c_void)).integer
                != 0 as libc::c_int as libc::c_longlong;
        } else {
            okay = 1 as libc::c_int != 0;
        }
        if okay {
            if !((*cs).action).is_null() {
                exeval(prog, (*cs).action, state as *mut libc::c_void);
            } else {
                agsubedge((*state).target, e, (0 as libc::c_int == 0) as libc::c_int);
            }
        }
        i += 1;
    }
    return (*state).curobj;
}
unsafe extern "C" fn evalNode(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut xprog: *mut comp_block,
    mut n: *mut Agnode_t,
) -> *mut Agobj_t {
    let mut i: libc::c_int = 0;
    let mut cs: *mut case_stmt = 0 as *mut case_stmt;
    let mut okay: bool = false;
    let ref mut fresh29 = (*state).curobj;
    *fresh29 = n as *mut Agobj_t;
    i = 0 as libc::c_int;
    while i < (*xprog).n_nstmts {
        cs = ((*xprog).node_stmts).offset(i as isize);
        if !((*cs).guard).is_null() {
            okay = (exeval(prog, (*cs).guard, state as *mut libc::c_void)).integer
                != 0 as libc::c_int as libc::c_longlong;
        } else {
            okay = 1 as libc::c_int != 0;
        }
        if okay {
            if !((*cs).action).is_null() {
                exeval(prog, (*cs).action, state as *mut libc::c_void);
            } else {
                agsubnode((*state).target, n, (0 as libc::c_int == 0) as libc::c_int);
            }
        }
        i += 1;
    }
    return (*state).curobj;
}
unsafe extern "C" fn nextNode(mut state: *mut Gpr_t, mut nodes: *mut nodestream) -> *mut Agnode_t {
    let mut np: *mut Agnode_t = 0 as *mut Agnode_t;
    if (*state).tvroot != (*nodes).oldroot {
        let ref mut fresh30 = (*nodes).oldroot;
        *fresh30 = (*state).tvroot;
        np = *fresh30;
    } else if (*state).flags & 8 as libc::c_int != 0 {
        let ref mut fresh31 = (*state).tvroot;
        *fresh31 = (*state).tvnext;
        let ref mut fresh32 = (*nodes).oldroot;
        *fresh32 = *fresh31;
        np = *fresh32;
        (*state).flags &= !(8 as libc::c_int);
    } else if !((*nodes).prev).is_null() {
        let ref mut fresh33 = (*nodes).prev;
        *fresh33 = agnxtnode((*state).curgraph, (*nodes).prev);
        np = *fresh33;
    } else {
        let ref mut fresh34 = (*nodes).prev;
        *fresh34 = agfstnode((*state).curgraph);
        np = *fresh34;
    }
    return np;
}
static mut DFSfns: trav_fns = unsafe {
    {
        let mut init = trav_fns {
            fstedge: Some(
                agfstedge as unsafe extern "C" fn(*mut Agraph_t, *mut Agnode_t) -> *mut Agedge_t,
            ),
            nxtedge: Some(
                agnxtedge
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agedge_t,
                        *mut Agnode_t,
                    ) -> *mut Agedge_t,
            ),
            undirected: 1 as libc::c_int as libc::c_uchar,
            visit: 0 as libc::c_int as libc::c_uchar,
        };
        init
    }
};
static mut FWDfns: trav_fns = unsafe {
    {
        let mut init = trav_fns {
            fstedge: Some(
                agfstout as unsafe extern "C" fn(*mut Agraph_t, *mut Agnode_t) -> *mut Agedge_t,
            ),
            nxtedge: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agedge_t) -> *mut Agedge_t>,
                nxttedgefn_t,
            >(Some(
                agnxtout as unsafe extern "C" fn(*mut Agraph_t, *mut Agedge_t) -> *mut Agedge_t,
            )),
            undirected: 0 as libc::c_int as libc::c_uchar,
            visit: 0 as libc::c_int as libc::c_uchar,
        };
        init
    }
};
static mut REVfns: trav_fns = unsafe {
    {
        let mut init = trav_fns {
            fstedge: Some(
                agfstin as unsafe extern "C" fn(*mut Agraph_t, *mut Agnode_t) -> *mut Agedge_t,
            ),
            nxtedge: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agedge_t) -> *mut Agedge_t>,
                nxttedgefn_t,
            >(Some(
                agnxtin as unsafe extern "C" fn(*mut Agraph_t, *mut Agedge_t) -> *mut Agedge_t,
            )),
            undirected: 0 as libc::c_int as libc::c_uchar,
            visit: 0 as libc::c_int as libc::c_uchar,
        };
        init
    }
};
unsafe extern "C" fn travBFS(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut xprog: *mut comp_block,
) {
    let mut nodes: nodestream = nodestream {
        oldroot: 0 as *mut Agnode_t,
        prev: 0 as *mut Agnode_t,
    };
    let mut q: *mut queue = 0 as *mut queue;
    let mut nd: *mut ndata = 0 as *mut ndata;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut cure: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut nxte: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut g: *mut Agraph_t = (*state).curgraph;
    q = mkQ(Dtqueue);
    nodes.oldroot = 0 as *mut Agnode_t;
    nodes.prev = 0 as *mut Agnode_t;
    loop {
        n = nextNode(state, &mut nodes);
        if n.is_null() {
            break;
        }
        nd = aggetrec(
            n as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut ndata;
        if (*nd).iu.integer & 1 as libc::c_int as libc::c_longlong != 0 {
            continue;
        }
        (*nd).iu.integer |= 2 as libc::c_int as libc::c_longlong;
        let ref mut fresh35 = (*nd).ine;
        *fresh35 = 0 as *mut Agedge_t;
        push(q, n as *mut libc::c_void);
        loop {
            n = pop(q, 1 as libc::c_int) as *mut Agnode_t;
            if n.is_null() {
                break;
            }
            nd = aggetrec(
                n as *mut libc::c_void,
                b"userval\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) as *mut ndata;
            (*nd).iu.integer = 1 as libc::c_int as libc::c_longlong;
            (*nd).iu.integer &= !(2 as libc::c_int) as libc::c_longlong;
            let ref mut fresh36 = (*state).tvedge;
            *fresh36 = (*nd).ine;
            if (evalNode(state, prog, xprog, n)).is_null() {
                continue;
            }
            cure = agfstedge(g, n);
            while !cure.is_null() {
                nxte = agnxtedge(g, cure, n);
                nd = aggetrec(
                    (*cure).node as *mut libc::c_void,
                    b"userval\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut ndata;
                if !((*nd).iu.integer & 1 as libc::c_int as libc::c_longlong != 0) {
                    if !(evalEdge(state, prog, xprog, cure)).is_null() {
                        if (*nd).iu.integer & 2 as libc::c_int as libc::c_longlong == 0 {
                            push(q, (*cure).node as *mut libc::c_void);
                            (*nd).iu.integer |= 2 as libc::c_int as libc::c_longlong;
                            let ref mut fresh37 = (*nd).ine;
                            *fresh37 = cure;
                        }
                    }
                }
                cure = nxte;
            }
        }
    }
    let ref mut fresh38 = (*state).tvedge;
    *fresh38 = 0 as *mut Agedge_t;
    freeQ(q);
}
unsafe extern "C" fn travDFS(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut xprog: *mut comp_block,
    mut fns: *mut trav_fns,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut stk: gv_stack_t = {
        let mut init = gv_stack_t {
            base: 0 as *mut *mut libc::c_void,
            size: 0,
            capacity: 0,
        };
        init
    };
    let mut curn: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut cure: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut entry: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut more: libc::c_int = 0;
    let mut nd: *mut ndata = 0 as *mut ndata;
    let mut nodes: nodestream = nodestream {
        oldroot: 0 as *mut Agnode_t,
        prev: 0 as *mut Agnode_t,
    };
    let mut seed: Agedgepair_t = Agedgepair_t {
        out: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
        in_0: Agedge_t {
            base: Agobj_t {
                tag: Agtag_t {
                    objtype_mtflock_attrwf_seq: [0; 4],
                    c2rust_padding: [0; 4],
                    id: 0,
                },
                data: 0 as *mut Agrec_t,
            },
            id_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed { _hash: 0 },
            },
            seq_link: Dtlink_t {
                right: 0 as *mut Dtlink_t,
                hl: C2RustUnnamed { _hash: 0 },
            },
            node: 0 as *mut Agnode_t,
        },
    };
    nodes.oldroot = 0 as *mut Agnode_t;
    nodes.prev = 0 as *mut Agnode_t;
    loop {
        n = nextNode(state, &mut nodes);
        if n.is_null() {
            break;
        }
        nd = aggetrec(
            n as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut ndata;
        if (*nd).iu.integer & 1 as libc::c_int as libc::c_longlong != 0 {
            continue;
        }
        seed.out.node = n;
        seed.in_0.node = 0 as *mut Agnode_t;
        curn = n;
        entry = &mut seed.out;
        cure = 0 as *mut Agedge_t;
        let ref mut fresh39 = (*state).tvedge;
        *fresh39 = cure;
        (*nd).iu.integer = 1 as libc::c_int as libc::c_longlong;
        (*nd).iu.integer |= 2 as libc::c_int as libc::c_longlong;
        let ref mut fresh40 = (*nd).ine;
        *fresh40 = 0 as *mut Agedge_t;
        if (*fns).visit as libc::c_int & 1 as libc::c_int != 0 {
            evalNode(state, prog, xprog, n);
        }
        more = 1 as libc::c_int;
        while more != 0 {
            if !cure.is_null() {
                cure = ((*fns).nxtedge).expect("non-null function pointer")(
                    (*state).curgraph,
                    cure,
                    curn,
                );
            } else {
                cure =
                    ((*fns).fstedge).expect("non-null function pointer")((*state).curgraph, curn);
            }
            if !cure.is_null() {
                if entry
                    == (if ((*(cure as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        cure.offset(-(1 as libc::c_int as isize))
                    } else {
                        cure.offset(1 as libc::c_int as isize)
                    })
                {
                    continue;
                }
                nd = aggetrec(
                    (*cure).node as *mut libc::c_void,
                    b"userval\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut ndata;
                if (*nd).iu.integer & 1 as libc::c_int as libc::c_longlong != 0 {
                    if (*fns).undirected != 0 {
                        if (*nd).iu.integer & 2 as libc::c_int as libc::c_longlong != 0 {
                            evalEdge(state, prog, xprog, cure);
                        }
                    } else {
                        evalEdge(state, prog, xprog, cure);
                    }
                } else {
                    evalEdge(state, prog, xprog, cure);
                    stack_push_or_exit(&mut stk, entry as *mut libc::c_void);
                    entry = cure;
                    let ref mut fresh41 = (*state).tvedge;
                    *fresh41 = entry;
                    curn = (*cure).node;
                    cure = 0 as *mut Agedge_t;
                    if (*fns).visit as libc::c_int & 1 as libc::c_int != 0 {
                        evalNode(state, prog, xprog, curn);
                    }
                    (*nd).iu.integer = 1 as libc::c_int as libc::c_longlong;
                    (*nd).iu.integer |= 2 as libc::c_int as libc::c_longlong;
                    let ref mut fresh42 = (*nd).ine;
                    *fresh42 = entry;
                }
            } else {
                if (*fns).visit as libc::c_int & 2 as libc::c_int != 0 {
                    evalNode(state, prog, xprog, curn);
                }
                nd = aggetrec(
                    curn as *mut libc::c_void,
                    b"userval\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut ndata;
                (*nd).iu.integer &= !(2 as libc::c_int) as libc::c_longlong;
                cure = entry;
                entry = (if stack_is_empty(&mut stk) as libc::c_int != 0 {
                    0 as *mut libc::c_void
                } else {
                    stack_pop(&mut stk)
                }) as *mut Agedge_t;
                if entry == &mut seed.out as *mut Agedge_t {
                    let ref mut fresh43 = (*state).tvedge;
                    *fresh43 = 0 as *mut Agedge_t;
                } else {
                    let ref mut fresh44 = (*state).tvedge;
                    *fresh44 = entry;
                }
                if !entry.is_null() {
                    curn = (*entry).node;
                } else {
                    more = 0 as libc::c_int;
                }
            }
        }
    }
    let ref mut fresh45 = (*state).tvedge;
    *fresh45 = 0 as *mut Agedge_t;
    stack_reset(&mut stk);
}
unsafe extern "C" fn travNodes(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut xprog: *mut comp_block,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut next: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut g: *mut Agraph_t = (*state).curgraph;
    n = agfstnode(g);
    while !n.is_null() {
        next = agnxtnode(g, n);
        evalNode(state, prog, xprog, n);
        n = next;
    }
}
unsafe extern "C" fn travEdges(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut xprog: *mut comp_block,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut next: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut nexte: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut g: *mut Agraph_t = (*state).curgraph;
    n = agfstnode(g);
    while !n.is_null() {
        next = agnxtnode(g, n);
        e = agfstout(g, n);
        while !e.is_null() {
            nexte = agnxtout(g, e);
            evalEdge(state, prog, xprog, e);
            e = nexte;
        }
        n = next;
    }
}
unsafe extern "C" fn travFlat(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut xprog: *mut comp_block,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut next: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut nexte: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut g: *mut Agraph_t = (*state).curgraph;
    n = agfstnode(g);
    while !n.is_null() {
        next = agnxtnode(g, n);
        if !(evalNode(state, prog, xprog, n)).is_null() {
            if (*xprog).n_estmts > 0 as libc::c_int {
                e = agfstout(g, n);
                while !e.is_null() {
                    nexte = agnxtout(g, e);
                    evalEdge(state, prog, xprog, e);
                    e = nexte;
                }
            }
        }
        n = next;
    }
}
unsafe extern "C" fn doCleanup(mut g: *mut Agraph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut nd: *mut ndata = 0 as *mut ndata;
    n = agfstnode(g);
    while !n.is_null() {
        nd = aggetrec(
            n as *mut libc::c_void,
            b"userval\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut ndata;
        let ref mut fresh46 = (*nd).ine;
        *fresh46 = 0 as *mut Agedge_t;
        (*nd).iu.integer = 0 as libc::c_int as libc::c_longlong;
        n = agnxtnode(g, n);
    }
}
unsafe extern "C" fn traverse(
    mut state: *mut Gpr_t,
    mut prog: *mut Expr_t,
    mut bp: *mut comp_block,
    mut cleanup: libc::c_int,
) -> libc::c_int {
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*state).target).is_null() {
        if (*state).name_used != 0 {
            sfprintf(
                (*state).tmp,
                b"%s%d\0" as *const u8 as *const libc::c_char,
                (*state).tgtname,
                (*state).name_used,
            );
            if (*(*state).tmp).next >= (*(*state).tmp).endw {
                _sfflsbuf(
                    (*state).tmp,
                    0 as libc::c_int as libc::c_uchar as libc::c_int,
                );
            } else {
                let ref mut fresh47 = (*(*state).tmp).next;
                let fresh48 = *fresh47;
                *fresh47 = (*fresh47).offset(1);
                *fresh48 = 0 as libc::c_int as libc::c_uchar;
            };
            let ref mut fresh49 = (*(*state).tmp).next;
            *fresh49 = (*(*state).tmp).data;
            target = *fresh49 as *mut libc::c_char;
        } else {
            target = (*state).tgtname;
        }
        let ref mut fresh50 = (*state).name_used;
        *fresh50 += 1;
        while !(agsubg((*state).curgraph, target, 0 as libc::c_int)).is_null() {
            let ref mut fresh51 = (*state).name_used;
            *fresh51 += 1;
            sfprintf(
                (*state).tmp,
                b"%s%d\0" as *const u8 as *const libc::c_char,
                (*state).tgtname,
                (*state).name_used,
            );
            if (*(*state).tmp).next >= (*(*state).tmp).endw {
                _sfflsbuf(
                    (*state).tmp,
                    0 as libc::c_int as libc::c_uchar as libc::c_int,
                );
            } else {
                let ref mut fresh52 = (*(*state).tmp).next;
                let fresh53 = *fresh52;
                *fresh52 = (*fresh52).offset(1);
                *fresh53 = 0 as libc::c_int as libc::c_uchar;
            };
            let ref mut fresh54 = (*(*state).tmp).next;
            *fresh54 = (*(*state).tmp).data;
            target = *fresh54 as *mut libc::c_char;
        }
        let ref mut fresh55 = (*state).target;
        *fresh55 = openSubg((*state).curgraph, target);
    }
    if ((*state).outgraph).is_null() {
        let ref mut fresh56 = (*state).outgraph;
        *fresh56 = (*state).target;
    }
    match (*state).tvt as libc::c_uint {
        0 => {
            travFlat(state, prog, bp);
        }
        3 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            travBFS(state, prog, bp);
            cleanup = 1 as libc::c_int;
        }
        4 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            DFSfns.visit = 1 as libc::c_int as libc::c_uchar;
            travDFS(state, prog, bp, &mut DFSfns);
            cleanup = 1 as libc::c_int;
        }
        5 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            FWDfns.visit = 1 as libc::c_int as libc::c_uchar;
            travDFS(state, prog, bp, &mut FWDfns);
            cleanup = 1 as libc::c_int;
        }
        6 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            REVfns.visit = 1 as libc::c_int as libc::c_uchar;
            travDFS(state, prog, bp, &mut REVfns);
            cleanup = 1 as libc::c_int;
        }
        7 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            DFSfns.visit = 2 as libc::c_int as libc::c_uchar;
            travDFS(state, prog, bp, &mut DFSfns);
            cleanup = 1 as libc::c_int;
        }
        8 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            FWDfns.visit = 2 as libc::c_int as libc::c_uchar;
            travDFS(state, prog, bp, &mut FWDfns);
            cleanup = 1 as libc::c_int;
        }
        9 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            REVfns.visit = 2 as libc::c_int as libc::c_uchar;
            travDFS(state, prog, bp, &mut REVfns);
            cleanup = 1 as libc::c_int;
        }
        10 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            DFSfns.visit = (2 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            travDFS(state, prog, bp, &mut DFSfns);
            cleanup = 1 as libc::c_int;
        }
        11 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            FWDfns.visit = (2 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            travDFS(state, prog, bp, &mut FWDfns);
            cleanup = 1 as libc::c_int;
        }
        12 => {
            if cleanup != 0 {
                doCleanup((*state).curgraph);
            }
            REVfns.visit = (2 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
            travDFS(state, prog, bp, &mut REVfns);
            cleanup = 1 as libc::c_int;
        }
        1 => {
            travNodes(state, prog, bp);
            travEdges(state, prog, bp);
        }
        2 => {
            travEdges(state, prog, bp);
            travNodes(state, prog, bp);
        }
        _ => {}
    }
    return cleanup;
}
unsafe extern "C" fn addOutputGraph(mut state: *mut Gpr_t, mut uopts: *mut gvpropts) {
    let mut g: *mut Agraph_t = (*state).outgraph;
    if agroot(g as *mut libc::c_void) == (*state).curgraph && ((*uopts).ingraphs).is_null() {
        g = clone(0 as *mut Agraph_t, g as *mut Agobj_t) as *mut Agraph_t;
    }
    let ref mut fresh57 = (*uopts).n_outgraphs;
    *fresh57 += 1;
    let ref mut fresh58 = (*uopts).outgraphs;
    *fresh58 = if !((*uopts).outgraphs).is_null() {
        realloc(
            (*uopts).outgraphs as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong)
                .wrapping_mul((*uopts).n_outgraphs as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut *mut Agraph_t
    } else {
        malloc(
            (::std::mem::size_of::<*mut Agraph_t>() as libc::c_ulong)
                .wrapping_mul((*uopts).n_outgraphs as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut *mut Agraph_t
    };
    let ref mut fresh59 =
        *((*uopts).outgraphs).offset(((*uopts).n_outgraphs - 1 as libc::c_int) as isize);
    *fresh59 = g;
}
unsafe extern "C" fn chkClose(mut g: *mut Agraph_t) {
    let mut data: *mut gdata = 0 as *mut gdata;
    data = aggetrec(
        g as *mut libc::c_void,
        b"userval\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut gdata;
    if (*data).lock as libc::c_int & 1 as libc::c_int != 0 {
        let ref mut fresh60 = (*data).lock;
        *fresh60 = (*fresh60 as libc::c_int | 2 as libc::c_int) as libc::c_char;
    } else {
        agclose(g);
    };
}
unsafe extern "C" fn ing_open(mut f: *mut libc::c_char) -> *mut libc::c_void {
    return sfopen(
        0 as *mut Sfio_t,
        f,
        b"r\0" as *const u8 as *const libc::c_char,
    ) as *mut libc::c_void;
}
unsafe extern "C" fn ing_read(mut fp: *mut libc::c_void) -> *mut Agraph_t {
    return readG(fp as *mut Sfio_t);
}
unsafe extern "C" fn ing_close(mut fp: *mut libc::c_void) -> libc::c_int {
    return sfclose(fp as *mut Sfio_t);
}
static mut ingDisc: ingdisc = unsafe {
    {
        let mut init = ingdisc {
            openf: Some(ing_open as unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_void),
            readf: Some(ing_read as unsafe extern "C" fn(*mut libc::c_void) -> *mut Agraph_t),
            closef: Some(ing_close as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
            dflt: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    }
};
unsafe extern "C" fn setDisc(mut sp: *mut Sfio_t, mut dp: *mut Sfdisc_t, mut fn_0: gvprwr) {
    let ref mut fresh61 = (*dp).readf;
    *fresh61 = None;
    let ref mut fresh62 = (*dp).writef;
    *fresh62 = ::std::mem::transmute::<gvprwr, Sfwrite_f>(fn_0);
    let ref mut fresh63 = (*dp).seekf;
    *fresh63 = None;
    let ref mut fresh64 = (*dp).exceptf;
    *fresh64 = None;
    let ref mut fresh65 = (*dp).disc;
    *fresh65 = 0 as *mut Sfdisc_t;
    dp = sfdisc(sp, dp);
}
static mut jbuf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn gvexitf(
    mut handle: *mut Expr_t,
    mut discipline: *mut Exdisc_t,
    mut v: libc::c_int,
) {
    longjmp(jbuf.as_mut_ptr(), v);
}
unsafe extern "C" fn gverrorf(
    mut handle: *mut Expr_t,
    mut discipline: *mut Exdisc_t,
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    _err_msgv(
        if !discipline.is_null() && !handle.is_null() {
            *(handle as *mut *mut libc::c_char)
        } else {
            handle as *mut libc::c_char
        },
        level,
        fmt,
        ap.as_va_list(),
    );
    if level >= 2 as libc::c_int {
        let mut state: *mut Gpr_t = (*discipline).user as *mut Gpr_t;
        if (*state).flags & 1 as libc::c_int != 0 {
            graphviz_exit(1 as libc::c_int);
        } else {
            if (*state).flags & 4 as libc::c_int != 0 {
                longjmp(jbuf.as_mut_ptr(), 1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvpr(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut uopts: *mut gvpropts,
) -> libc::c_int {
    let mut current_block: u64;
    let mut errdisc: Sfdisc_t = Sfdisc_t {
        readf: None,
        writef: None,
        seekf: None,
        exceptf: None,
        disc: 0 as *mut Sfdisc_t,
    };
    let mut outdisc: Sfdisc_t = Sfdisc_t {
        readf: None,
        writef: None,
        seekf: None,
        exceptf: None,
        disc: 0 as *mut Sfdisc_t,
    };
    let mut prog: *mut parse_prog = 0 as *mut parse_prog;
    let mut ing: *mut ingraph_state = 0 as *mut ingraph_state;
    let mut xprog: *mut comp_prog = 0 as *mut comp_prog;
    let mut state: *mut Gpr_t = 0 as *mut Gpr_t;
    let mut info: gpr_info = gpr_info {
        outFile: 0 as *mut Sfio_t,
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        errf: None,
        exitf: None,
        flags: 0,
    };
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut cleanup: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut incoreGraphs: libc::c_int = 0;
    let mut nextg: *mut Agraph_t = 0 as *mut Agraph_t;
    setErrorErrors(0 as libc::c_int);
    ingDisc.dflt = sfstdin as *mut libc::c_void;
    if !uopts.is_null() {
        if ((*uopts).out).is_some() {
            setDisc(sfstdout, &mut outdisc, (*uopts).out);
        }
        if ((*uopts).err).is_some() {
            setDisc(sfstderr, &mut errdisc, (*uopts).err);
        }
    }
    let mut opts: options = scanArgs(argc, argv);
    if opts.state <= 0 as libc::c_int {
        rv = opts.state;
    } else {
        if opts.verbose != 0 {
            gvstart_timer();
        }
        prog = parseProg(opts.program, opts.useFile);
        if prog.is_null() {
            rv = 1 as libc::c_int;
        } else {
            info.outFile = opts.outFile;
            info.argc = opts.argc;
            info.argv = opts.argv;
            info.errf = Some(
                gverrorf
                    as unsafe extern "C" fn(
                        *mut Expr_t,
                        *mut Exdisc_t,
                        libc::c_int,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            );
            if !uopts.is_null() {
                info.flags = (*uopts).flags;
            } else {
                info.flags = 0 as libc::c_int;
            }
            if (*uopts).flags & 1 as libc::c_int != 0 {
                info.exitf = None;
            } else {
                info.exitf = Some(
                    gvexitf as unsafe extern "C" fn(*mut Expr_t, *mut Exdisc_t, libc::c_int) -> (),
                );
            }
            state = openGPRState(&mut info);
            if state.is_null() {
                rv = 1 as libc::c_int;
            } else {
                if !((*uopts).bindings).is_null() {
                    addBindings(state, (*uopts).bindings);
                }
                xprog = compileProg(prog, state, opts.compflags);
                if xprog.is_null() {
                    rv = 1 as libc::c_int;
                } else {
                    initGPRState(state, (*(*xprog).prog).vm);
                    if (*uopts).flags & 2 as libc::c_int != 0 {
                        let ref mut fresh66 = (*uopts).outgraphs;
                        *fresh66 = 0 as *mut *mut Agraph_t;
                        (*uopts).n_outgraphs = 0 as libc::c_int;
                    }
                    if (*uopts).flags & 1 as libc::c_int == 0 {
                        (*state).flags |= 4 as libc::c_int;
                        rv = _setjmp(jbuf.as_mut_ptr());
                        if rv != 0 {
                            current_block = 3822716870468469861;
                        } else {
                            current_block = 6174974146017752131;
                        }
                    } else {
                        current_block = 6174974146017752131;
                    }
                    match current_block {
                        3822716870468469861 => {}
                        _ => {
                            if !uopts.is_null() && !((*uopts).ingraphs).is_null() {
                                incoreGraphs = 1 as libc::c_int;
                            } else {
                                incoreGraphs = 0 as libc::c_int;
                            }
                            if opts.verbose != 0 {
                                fprintf(
                                    stderr,
                                    b"Parse/compile/init: %.2f secs.\n\0" as *const u8
                                        as *const libc::c_char,
                                    gvelapsed_sec(),
                                );
                            }
                            if !((*xprog).begin_stmt).is_null() {
                                exeval(
                                    (*xprog).prog,
                                    (*xprog).begin_stmt,
                                    state as *mut libc::c_void,
                                );
                            }
                            if usesGraph(xprog) != 0 {
                                if !uopts.is_null() && !((*uopts).ingraphs).is_null() {
                                    ing = newIngGraphs(
                                        0 as *mut ingraph_state,
                                        (*uopts).ingraphs,
                                        &mut ingDisc,
                                    );
                                } else {
                                    ing =
                                        newIng(0 as *mut ingraph_state, opts.inFiles, &mut ingDisc);
                                }
                                if opts.verbose != 0 {
                                    gvstart_timer();
                                }
                                let ref mut fresh67 = (*state).curgraph;
                                *fresh67 = nextGraph(ing);
                                while !((*state).curgraph).is_null() {
                                    if opts.verbose != 0 {
                                        fprintf(
                                            stderr,
                                            b"Read graph: %.2f secs.\n\0" as *const u8
                                                as *const libc::c_char,
                                            gvelapsed_sec(),
                                        );
                                    }
                                    let ref mut fresh68 = (*state).infname;
                                    *fresh68 = fileName(ing);
                                    if opts.readAhead != 0 {
                                        let ref mut fresh69 = (*state).nextgraph;
                                        *fresh69 = nextGraph(ing);
                                        nextg = *fresh69;
                                    }
                                    cleanup = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < (*xprog).n_blocks {
                                        let mut bp: *mut comp_block =
                                            ((*xprog).blocks).offset(i as isize);
                                        if incoreGraphs != 0
                                            && opts.compflags & 0x4 as libc::c_int != 0
                                        {
                                            let ref mut fresh70 = (*state).curgraph;
                                            *fresh70 = clone(
                                                0 as *mut Agraph_t,
                                                (*state).curgraph as *mut Agobj_t,
                                            )
                                                as *mut Agraph_t;
                                        }
                                        let ref mut fresh71 = (*state).curobj;
                                        *fresh71 = (*state).curgraph as *mut Agobj_t;
                                        let ref mut fresh72 = (*state).tvroot;
                                        *fresh72 = 0 as *mut Agnode_t;
                                        if !((*bp).begg_stmt).is_null() {
                                            exeval(
                                                (*xprog).prog,
                                                (*bp).begg_stmt,
                                                state as *mut libc::c_void,
                                            );
                                        }
                                        if walksGraph(bp) != 0 {
                                            cleanup = traverse(state, (*xprog).prog, bp, cleanup);
                                        }
                                        i += 1;
                                    }
                                    let ref mut fresh73 = (*state).curobj;
                                    *fresh73 = (*state).curgraph as *mut Agobj_t;
                                    if !((*xprog).endg_stmt).is_null() {
                                        exeval(
                                            (*xprog).prog,
                                            (*xprog).endg_stmt,
                                            state as *mut libc::c_void,
                                        );
                                    }
                                    if opts.verbose != 0 {
                                        fprintf(
                                            stderr,
                                            b"Finish graph: %.2f secs.\n\0" as *const u8
                                                as *const libc::c_char,
                                            gvelapsed_sec(),
                                        );
                                    }
                                    if (*state).outgraph == (*state).curgraph
                                        && !((*state).target).is_null()
                                        && agnnodes((*state).target) == 0
                                    {
                                        agdelete(
                                            (*state).curgraph,
                                            (*state).target as *mut libc::c_void,
                                        );
                                    }
                                    if !((*state).outgraph).is_null()
                                        && (agnnodes((*state).outgraph) != 0
                                            || opts.compflags & 0x1 as libc::c_int != 0)
                                    {
                                        if !uopts.is_null()
                                            && (*uopts).flags & 2 as libc::c_int != 0
                                        {
                                            addOutputGraph(state, uopts);
                                        } else {
                                            sfioWrite(
                                                (*state).outgraph,
                                                opts.outFile,
                                                (*state).dfltIO,
                                            );
                                        }
                                    }
                                    if incoreGraphs == 0 {
                                        chkClose((*state).curgraph);
                                    }
                                    let ref mut fresh74 = (*state).target;
                                    *fresh74 = 0 as *mut Agraph_t;
                                    let ref mut fresh75 = (*state).outgraph;
                                    *fresh75 = 0 as *mut Agraph_t;
                                    if opts.verbose != 0 {
                                        gvstart_timer();
                                    }
                                    if opts.readAhead == 0 {
                                        nextg = nextGraph(ing);
                                    }
                                    if opts.verbose != 0 && !nextg.is_null() {
                                        fprintf(
                                            stderr,
                                            b"Read graph: %.2f secs.\n\0" as *const u8
                                                as *const libc::c_char,
                                            gvelapsed_sec(),
                                        );
                                    }
                                    let ref mut fresh76 = (*state).curgraph;
                                    *fresh76 = nextg;
                                }
                            }
                            let ref mut fresh77 = (*state).curgraph;
                            *fresh77 = 0 as *mut Agraph_t;
                            let ref mut fresh78 = (*state).curobj;
                            *fresh78 = 0 as *mut Agobj_t;
                            if !((*xprog).end_stmt).is_null() {
                                exeval(
                                    (*xprog).prog,
                                    (*xprog).end_stmt,
                                    state as *mut libc::c_void,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    freeParseProg(prog);
    freeCompileProg(xprog);
    closeGPRState(state);
    if !ing.is_null() {
        closeIngraph(ing);
    }
    freeOpts(opts);
    if !uopts.is_null() {
        if ((*uopts).out).is_some() {
            sfdisc(sfstdout, 0 as *mut Sfdisc_t);
        }
        if ((*uopts).err).is_some() {
            sfdisc(sfstderr, 0 as *mut Sfdisc_t);
        }
    }
    return rv;
}
