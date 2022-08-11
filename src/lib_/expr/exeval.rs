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
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn drand48() -> libc::c_double;
    fn srand48(__seedval: libc::c_long);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    static mut sfstdout: *mut Sfio_t;
    fn sfnew(
        _: *mut Sfio_t,
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut Sfio_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfputr(_: *mut Sfio_t, _: *const libc::c_char, _: libc::c_int) -> ssize_t;
    fn sfprintf(_: *mut Sfio_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sfscanf(_: *mut Sfio_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sfsscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fmtbuf(n: size_t) -> *mut libc::c_char;
    fn fmtquote(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: size_t,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn strmatch(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strgrpmatch(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn excast(
        _: *mut Expr_t,
        _: *mut Exnode_t,
        _: libc::c_int,
        _: *mut Exnode_t,
        _: libc::c_int,
    ) -> *mut Exnode_t;
    fn exerror(_: *const libc::c_char, _: ...);
    fn exwarn(_: *const libc::c_char, _: ...);
    fn exzero(_: libc::c_int) -> Extype_t;
    fn exnospace() -> *mut libc::c_char;
    fn extypename(p: *mut Expr_t, _: libc::c_int) -> *mut libc::c_char;
    fn exstash(_: *mut Sfio_t, _: *mut Vmalloc_t) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    static mut expr: Exstate_t;
    fn vmalloc(vm: *mut Vmalloc_t, size: size_t) -> *mut libc::c_void;
    fn vmresize(vm: *mut Vmalloc_t, data: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn vmfree(vm: *mut Vmalloc_t, data: *mut libc::c_void);
    fn vmstrdup(_: *mut Vmalloc_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn exop(id: size_t) -> *const libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
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
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
#[derive(Clone)]
#[repr(C)]
pub struct _sffmt_s<'a> {
    pub extf: Sffmtext_f,
    pub eventf: Sffmtevent_f,
    pub form: *mut libc::c_char,
    pub args: ::std::ffi::VaListImpl<'a>,
    pub fmt: libc::c_int,
    pub size: ssize_t,
    pub flags: libc::c_int,
    pub width: libc::c_int,
    pub precis: libc::c_int,
    pub base: libc::c_int,
    pub t_str: *mut libc::c_char,
    pub n_str: ssize_t,
}
pub type Sffmtevent_f = Option<
    unsafe extern "C" fn(*mut Sfio_t, libc::c_int, *mut libc::c_void, *mut Sffmt_t) -> libc::c_int,
>;
pub type Sffmt_t<'a> = _sffmt_s<'a>;
pub type Sffmtext_f = Option<unsafe extern "C" fn(*mut libc::c_void, *mut Sffmt_t) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exinput_s {
    pub next: *mut Exinput_s,
    pub close: libc::c_int,
    pub file: *mut libc::c_char,
    pub fp: *mut Sfio_t,
    pub line: libc::c_int,
    pub nesting: libc::c_int,
    pub peek: libc::c_int,
    pub unit: libc::c_int,
    pub pushback: *mut libc::c_char,
    pub bp: *mut libc::c_char,
    pub pp: *mut libc::c_char,
    pub sp: *mut libc::c_char,
}
pub type Exinput_t = Exinput_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Print_s {
    pub next: *mut Print_s,
    pub format: *mut libc::c_char,
    pub param: [*mut Exnode_s; 3],
    pub arg: *mut Exnode_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exnode_s {
    pub type_0: libc::c_int,
    pub op: libc::c_int,
    pub binary: libc::c_int,
    pub local: Exlocal_t,
    pub compiled: C2RustUnnamed_12,
    pub data: Exdata_t,
    pub subop: libc::c_int,
}
pub type Exdata_t = Exdata_u;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Exdata_u {
    pub constant: C2RustUnnamed_11,
    pub operand: C2RustUnnamed_10,
    pub select: C2RustUnnamed_9,
    pub variable: C2RustUnnamed_8,
    pub next: *mut Exnode_t,
    pub value: Extype_t,
    pub call: C2RustUnnamed_7,
    pub generate: C2RustUnnamed_6,
    pub split: C2RustUnnamed_5,
    pub print: C2RustUnnamed_4,
    pub string: C2RustUnnamed_3,
    pub procedure: C2RustUnnamed_0,
    pub scan: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub descriptor: *mut Exnode_t,
    pub format: *mut Exnode_t,
    pub args: *mut Exnode_t,
}
pub type Exnode_t = Exnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub args: *mut Exnode_t,
    pub body: *mut Exnode_t,
    pub frame: *mut Dt_t,
    pub arity: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub base: *mut Exnode_t,
    pub pat: *mut Exnode_t,
    pub repl: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub descriptor: *mut Exnode_t,
    pub args: *mut Print_t,
}
pub type Print_t = Print_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub array: *mut Exid_t,
    pub string: *mut Exnode_t,
    pub seps: *mut Exnode_t,
}
pub type Exid_t = Exid_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub array: *mut Exnode_t,
    pub index: *mut Exid_t,
    pub statement: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub procedure: *mut Exid_t,
    pub args: *mut Exnode_t,
}
pub type Extype_t = EX_STYPE;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exref_s {
    pub next: *mut Exref_t,
    pub symbol: *mut Exid_t,
    pub index: *mut Exnode_t,
}
pub type Exref_t = Exref_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub symbol: *mut Exid_t,
    pub reference: *mut Exref_t,
    pub index: *mut Exnode_t,
    pub dyna: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub statement: *mut Exnode_t,
    pub next: *mut Exnode_t,
    pub constant: *mut *mut Extype_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub left: *mut Exnode_t,
    pub right: *mut Exnode_t,
    pub last: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub value: Extype_t,
    pub reference: *mut Exid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub floating: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_double>,
    pub integer: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_longlong>,
    pub string: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> *mut libc::c_char>,
}
pub type intmax_t = __intmax_t;
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
    pub ve: *mut Vmalloc_t,
    pub frame: *mut Dt_t,
    pub symdisc: Dtdisc_t,
    pub disc: *mut Exdisc_t,
    pub input: *mut Exinput_t,
    pub program: *mut Expr_t,
    pub tmp: *mut Sfio_t,
    pub loopret: Extype_t,
    pub main: Exid_t,
    pub line: [libc::c_char; 512],
    pub linep: *mut libc::c_char,
    pub eof: libc::c_int,
    pub errors: libc::c_int,
    pub formals: libc::c_int,
    pub linewrap: libc::c_int,
    pub loopcount: libc::c_int,
    pub loopop: libc::c_int,
    pub nesting: libc::c_int,
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
pub type Exassoc_t = Exassoc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exassoc_s {
    pub link: Dtlink_t,
    pub key: Extype_t,
    pub value: Extype_t,
    pub name: [libc::c_char; 1],
}
#[derive(Clone)]
#[repr(C)]
pub struct Fmt_t<'a> {
    pub fmt: Sffmt_t<'a>,
    pub expr: *mut Expr_t,
    pub env: *mut libc::c_void,
    pub args: *mut Print_t,
    pub value: Extype_t,
    pub actuals: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub const _ISalnum: C2RustUnnamed_13 = 8;
pub const _ISlower: C2RustUnnamed_13 = 512;
pub const _ISupper: C2RustUnnamed_13 = 256;
pub const _ISalpha: C2RustUnnamed_13 = 1024;
pub type Exstate_t = Exstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exstate_s {
    pub id: *mut Exid_t,
    pub declare: libc::c_int,
    pub lastref: *mut Exref_t,
    pub nolabel: libc::c_int,
    pub null: Exinput_t,
    pub program: *mut Expr_t,
    pub procedure: *mut Exnode_t,
    pub refs: *mut Exref_t,
    pub assigned: libc::c_int,
    pub instatic: libc::c_int,
    pub statics: libc::c_int,
    pub swstate: *mut Switch_t,
    pub nullstring: [libc::c_char; 1],
}
pub type Switch_t = Switch_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Switch_s {
    pub prev: *mut Switch_s,
    pub firstcase: *mut Exnode_t,
    pub lastcase: *mut Exnode_t,
    pub defcase: *mut Exnode_t,
    pub base: *mut *mut Extype_t,
    pub cur: *mut *mut Extype_t,
    pub last: *mut *mut Extype_t,
    pub def: libc::c_int,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_t {
    pub allocator: *mut Vmalloc_t,
    pub base: *mut libc::c_char,
    pub size: size_t,
    pub capacity: size_t,
}
pub const _ISdigit: C2RustUnnamed_13 = 2048;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_13 = 4;
pub const _IScntrl: C2RustUnnamed_13 = 2;
pub const _ISblank: C2RustUnnamed_13 = 1;
pub const _ISgraph: C2RustUnnamed_13 = 32768;
pub const _ISprint: C2RustUnnamed_13 = 16384;
pub const _ISspace: C2RustUnnamed_13 = 8192;
pub const _ISxdigit: C2RustUnnamed_13 = 4096;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn exprintf(
    mut vm: *mut Vmalloc_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut ap2: ::std::ffi::VaListImpl;
    ap2 = ap.clone();
    let mut len: libc::c_int = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap2.as_va_list(),
    );
    if len >= 0 as libc::c_int
        && !(b"invalid vsnprintf call\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"len >= 0 && \"invalid vsnprintf call\"\0" as *const u8 as *const libc::c_char,
            b"../../lib/expr/expr.h\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"char *exprintf(Vmalloc_t *, const char *, ...)\0",
            ))
            .as_ptr(),
        );
    }
    len += 1;
    let mut s: *mut libc::c_char = vmalloc(vm, len as size_t) as *mut libc::c_char;
    if s.is_null() {
        return exnospace();
    }
    vsnprintf(s, len as size_t, fmt, ap.as_va_list());
    return s;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn lexname(mut op: libc::c_int, mut subop: libc::c_int) -> *const libc::c_char {
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut n: libc::c_int = 0;
    static mut buf: [[libc::c_char; 23]; 4] = [[0; 23]; 4];
    if op > 258 as libc::c_int && op < 336 as libc::c_int {
        return exop((op as size_t).wrapping_sub(258 as libc::c_int as libc::c_ulong));
    }
    n += 1;
    if n >= 4 as libc::c_int {
        n = 0 as libc::c_int;
    }
    b = (buf[n as usize]).as_mut_ptr();
    if op == '=' as i32 {
        if subop > 258 as libc::c_int && subop < 336 as libc::c_int {
            snprintf(
                b,
                23 as libc::c_int as libc::c_ulong,
                b"%s=\0" as *const u8 as *const libc::c_char,
                exop((subop as size_t).wrapping_sub(258 as libc::c_int as libc::c_ulong)),
            );
        } else if subop > ' ' as i32 && subop <= '~' as i32 {
            snprintf(
                b,
                23 as libc::c_int as libc::c_ulong,
                b"%c=\0" as *const u8 as *const libc::c_char,
                subop,
            );
        } else {
            snprintf(
                b,
                23 as libc::c_int as libc::c_ulong,
                b"(%d)=\0" as *const u8 as *const libc::c_char,
                subop,
            );
        }
    } else if subop < 0 as libc::c_int {
        snprintf(
            b,
            23 as libc::c_int as libc::c_ulong,
            b"(EXTERNAL:%d)\0" as *const u8 as *const libc::c_char,
            op,
        );
    } else if op > ' ' as i32 && op <= '~' as i32 {
        snprintf(
            b,
            23 as libc::c_int as libc::c_ulong,
            b"%c\0" as *const u8 as *const libc::c_char,
            op,
        );
    } else {
        snprintf(
            b,
            23 as libc::c_int as libc::c_ulong,
            b"(%d)\0" as *const u8 as *const libc::c_char,
            op,
        );
    }
    return b;
}
unsafe extern "C" fn evaldyn(
    mut ex: *mut Expr_t,
    mut exnode: *mut Exnode_t,
    mut env: *mut libc::c_void,
    mut delete: libc::c_int,
) -> libc::c_int {
    let mut b: *mut Exassoc_t = 0 as *mut Exassoc_t;
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut key: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut keyname: *mut libc::c_char = 0 as *mut libc::c_char;
    v = eval(ex, (*exnode).data.variable.index, env);
    if (*(*exnode).data.variable.symbol).index_type == 259 as libc::c_int as libc::c_long {
        b = (Some(
            ((*((*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                .expect("non-null function pointer"),
        ))
        .expect("non-null function pointer")(
            (*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t,
            &mut v as *mut Extype_t as *mut libc::c_void,
            0o1000 as libc::c_int,
        ) as *mut Exassoc_t;
        if b.is_null() {
            return 0 as libc::c_int;
        }
    } else {
        let mut type_0: libc::c_int = (*(*exnode).data.variable.index).type_0;
        if type_0 != 263 as libc::c_int {
            if !(type_0 > 258 as libc::c_int) {
                key = (Some(((*(*ex).disc).keyf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    ex, v, type_0, (*ex).disc
                );
            } else {
                key.integer = v.integer;
            }
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%llx\0" as *const u8 as *const libc::c_char,
                key.integer as libc::c_ulonglong,
            );
            keyname = buf.as_mut_ptr();
        } else {
            keyname = v.string;
        }
        b = (Some(
            ((*((*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                .expect("non-null function pointer"),
        ))
        .expect("non-null function pointer")(
            (*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t,
            keyname as *mut libc::c_void,
            0o1000 as libc::c_int,
        ) as *mut Exassoc_t;
        if b.is_null() {
            return 0 as libc::c_int;
        }
    }
    if delete != 0 {
        (Some(
            ((*((*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                .expect("non-null function pointer"),
        ))
        .expect("non-null function pointer")(
            (*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t,
            b as *mut libc::c_void,
            0o2 as libc::c_int,
        );
        free(b as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn getdyn(
    mut ex: *mut Expr_t,
    mut exnode: *mut Exnode_t,
    mut env: *mut libc::c_void,
    mut assoc: *mut *mut Exassoc_t,
) -> Extype_t {
    let mut b: *mut Exassoc_t = 0 as *mut Exassoc_t;
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    if !((*exnode).data.variable.index).is_null() {
        let mut key: Extype_t = EX_STYPE {
            expr: 0 as *mut Exnode_s,
        };
        let mut buf: [libc::c_char; 17] = [0; 17];
        let mut keyname: *mut libc::c_char = 0 as *mut libc::c_char;
        v = eval(ex, (*exnode).data.variable.index, env);
        if (*(*exnode).data.variable.symbol).index_type == 259 as libc::c_int as libc::c_long {
            b = (Some(
                ((*((*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                    .expect("non-null function pointer"),
            ))
            .expect("non-null function pointer")(
                (*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t,
                &mut v as *mut Extype_t as *mut libc::c_void,
                0o1000 as libc::c_int,
            ) as *mut Exassoc_t;
            if b.is_null() {
                b = if 0 as libc::c_int != 0 {
                    realloc(
                        0 as *mut libc::c_char as *mut libc::c_void,
                        (::std::mem::size_of::<Exassoc_t>() as libc::c_ulong)
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ) as *mut Exassoc_t
                } else {
                    calloc(
                        1 as libc::c_int as libc::c_ulong,
                        (::std::mem::size_of::<Exassoc_t>() as libc::c_ulong)
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as libc::c_ulong),
                    ) as *mut Exassoc_t
                };
                if b.is_null() {
                    exnospace();
                }
                (*b).key = v;
                (Some(
                    ((*((*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                        .expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(
                    (*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t,
                    b as *mut libc::c_void,
                    0o1 as libc::c_int,
                );
            }
        } else {
            let mut type_0: libc::c_int = (*(*exnode).data.variable.index).type_0;
            if type_0 != 263 as libc::c_int {
                if !(type_0 > 258 as libc::c_int) {
                    key = (Some(((*(*ex).disc).keyf).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        ex, v, type_0, (*ex).disc
                    );
                } else {
                    key.integer = v.integer;
                }
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
                    b"%llx\0" as *const u8 as *const libc::c_char,
                    key.integer as libc::c_ulonglong,
                );
                keyname = buf.as_mut_ptr();
            } else {
                keyname = v.string;
            }
            b = (Some(
                ((*((*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                    .expect("non-null function pointer"),
            ))
            .expect("non-null function pointer")(
                (*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t,
                keyname as *mut libc::c_void,
                0o1000 as libc::c_int,
            ) as *mut Exassoc_t;
            if b.is_null() {
                b = if 0 as libc::c_int != 0 {
                    realloc(
                        0 as *mut libc::c_char as *mut libc::c_void,
                        (::std::mem::size_of::<Exassoc_t>() as libc::c_ulong)
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(keyname)),
                    ) as *mut Exassoc_t
                } else {
                    calloc(
                        1 as libc::c_int as libc::c_ulong,
                        (::std::mem::size_of::<Exassoc_t>() as libc::c_ulong)
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen(keyname)),
                    ) as *mut Exassoc_t
                };
                if b.is_null() {
                    exnospace();
                }
                strcpy(((*b).name).as_mut_ptr(), keyname);
                (*b).key = v;
                (Some(
                    ((*((*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                        .expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(
                    (*(*exnode).data.variable.symbol).local.pointer as *mut Dt_t,
                    b as *mut libc::c_void,
                    0o1 as libc::c_int,
                );
            }
        }
        *assoc = b;
        if !b.is_null() {
            if (*(*exnode).data.variable.symbol).type_0 == 263 as libc::c_int as libc::c_long
                && ((*b).value.string).is_null()
            {
                (*b).value = exzero((*(*exnode).data.variable.symbol).type_0 as libc::c_int);
            }
            return (*b).value;
        }
        v = exzero((*(*exnode).data.variable.symbol).type_0 as libc::c_int);
        return v;
    }
    *assoc = 0 as *mut Exassoc_t;
    return (*(*(*exnode).data.variable.symbol).value)
        .data
        .constant
        .value;
}
unsafe extern "C" fn streqn(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: size_t,
) -> bool {
    return strlen(s2) == n && strncmp(s1, s2, n) == 0 as libc::c_int;
}
unsafe extern "C" fn prformat(mut vp: *mut libc::c_void, mut dp: *mut Sffmt_t) -> libc::c_int {
    let mut fmt: *mut Fmt_t = dp as *mut Fmt_t;
    let mut node: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0 as libc::c_int;
    let mut tm: time_t = 0;
    let mut stm: *mut tm = 0 as *mut tm;
    (*dp).flags |= 0o400000 as libc::c_int;
    if !((*fmt).args).is_null() {
        node = if (*dp).fmt == '*' as i32 {
            (*(*fmt).args).param[(*dp).size as usize]
        } else {
            (*(*fmt).args).arg
        };
        if !node.is_null() {
            (*fmt).value = exeval((*fmt).expr, node, (*fmt).env);
        } else {
            (*fmt).value.integer = 0 as libc::c_int as libc::c_longlong;
        }
        to = (*(*(*fmt).args).arg).type_0;
    } else {
        let ref mut fresh0 = (*fmt).actuals;
        *fresh0 = (*(*fmt).actuals).data.operand.right;
        if (*fresh0).is_null() {
            exerror(b"printf: not enough arguments\0" as *const u8 as *const libc::c_char);
        } else {
            node = (*(*fmt).actuals).data.operand.left;
            from = (*node).type_0;
            match (*dp).fmt {
                102 | 103 => {
                    to = 262 as libc::c_int;
                }
                115 => {
                    to = 263 as libc::c_int;
                }
                _ => match from {
                    259 | 260 => {
                        to = from;
                    }
                    _ => {
                        to = 259 as libc::c_int;
                    }
                },
            }
            if to == from {
                (*fmt).value = exeval((*fmt).expr, node, (*fmt).env);
            } else {
                node = excast((*fmt).expr, node, to, 0 as *mut Exnode_t, 0 as libc::c_int);
                (*fmt).value = exeval((*fmt).expr, node, (*fmt).env);
                let ref mut fresh1 = (*node).data.operand.left;
                *fresh1 = 0 as *mut Exnode_t;
                vmfree((*(*fmt).expr).vm, node as *mut libc::c_void);
                if to == 263 as libc::c_int {
                    if !((*fmt).value.string).is_null() {
                        let mut n: size_t = strlen((*fmt).value.string);
                        s = fmtbuf(n.wrapping_add(1 as libc::c_int as libc::c_ulong));
                        if !s.is_null() {
                            memcpy(
                                s as *mut libc::c_void,
                                (*fmt).value.string as *const libc::c_void,
                                n.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                        }
                        vmfree((*(*fmt).expr).vm, (*fmt).value.string as *mut libc::c_void);
                        let ref mut fresh2 = (*fmt).value.string;
                        *fresh2 = s;
                    }
                    if ((*fmt).value.string).is_null() {
                        let ref mut fresh3 = (*fmt).value.string;
                        *fresh3 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    }
                }
            }
        }
    }
    match to {
        263 => {
            let ref mut fresh4 = *(vp as *mut *mut libc::c_char);
            *fresh4 = (*fmt).value.string;
            (*fmt).fmt.size = -(1 as libc::c_int) as ssize_t;
        }
        262 => {
            *(vp as *mut libc::c_double) = (*fmt).value.floating;
            (*fmt).fmt.size = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as ssize_t;
        }
        _ => {
            *(vp as *mut libc::c_longlong) = (*fmt).value.integer;
            (*dp).size = ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong as ssize_t;
        }
    }
    let mut txt: *const libc::c_char = 0 as *const libc::c_char;
    let mut txt_len: size_t = 0 as libc::c_int as size_t;
    if (*dp).n_str > 0 as libc::c_int as libc::c_long {
        txt = (*dp).t_str;
        txt_len = (*dp).n_str as size_t;
    }
    match (*dp).fmt {
        113 | 81 => {
            s = *(vp as *mut *mut libc::c_char);
            let ref mut fresh5 = *(vp as *mut *mut libc::c_char);
            *fresh5 = fmtquote(
                s,
                b"$'\0" as *const u8 as *const libc::c_char,
                b"'\0" as *const u8 as *const libc::c_char,
                strlen(s),
                0 as libc::c_int,
            );
            (*dp).fmt = 's' as i32;
            (*dp).size = -(1 as libc::c_int) as ssize_t;
        }
        83 => {
            (*dp).flags &= !(0o40000 as libc::c_int);
            s = *(vp as *mut *mut libc::c_char);
            if !txt.is_null() {
                if streqn(
                    txt,
                    b"identifier\0" as *const u8 as *const libc::c_char,
                    txt_len,
                ) {
                    if *s as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                    {
                        let fresh6 = s;
                        s = s.offset(1);
                        *fresh6 = '_' as i32 as libc::c_char;
                    }
                    while *s != 0 {
                        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            *s = '_' as i32 as libc::c_char;
                        }
                        s = s.offset(1);
                    }
                } else if streqn(
                    txt,
                    b"invert\0" as *const u8 as *const libc::c_char,
                    txt_len,
                ) {
                    while *s != 0 {
                        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            *s = ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *s as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(*s as libc::c_int);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset(*s as libc::c_int as isize);
                                }
                                __res
                            }) as libc::c_char;
                        } else if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize)
                            as libc::c_int
                            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            *s = ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *s as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = toupper(*s as libc::c_int);
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc())
                                        .offset(*s as libc::c_int as isize);
                                }
                                __res
                            }) as libc::c_char;
                        }
                        s = s.offset(1);
                    }
                } else if streqn(txt, b"lower\0" as *const u8 as *const libc::c_char, txt_len) {
                    while *s != 0 {
                        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            *s = ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *s as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = tolower(*s as libc::c_int);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset(*s as libc::c_int as isize);
                                }
                                __res
                            }) as libc::c_char;
                        }
                        s = s.offset(1);
                    }
                } else if streqn(txt, b"upper\0" as *const u8 as *const libc::c_char, txt_len) {
                    while *s != 0 {
                        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            *s = ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *s as libc::c_int;
                                        __res = if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        };
                                    } else {
                                        __res = toupper(*s as libc::c_int);
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc())
                                        .offset(*s as libc::c_int as isize);
                                }
                                __res
                            }) as libc::c_char;
                        }
                        s = s.offset(1);
                    }
                } else if streqn(
                    txt,
                    b"variable\0" as *const u8 as *const libc::c_char,
                    txt_len,
                ) {
                    while *s != 0 {
                        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                            && *s as libc::c_int != '_' as i32
                        {
                            *s = '.' as i32 as libc::c_char;
                        }
                        s = s.offset(1);
                    }
                }
            }
            (*dp).fmt = 's' as i32;
            (*dp).size = -(1 as libc::c_int) as ssize_t;
        }
        116 | 84 => {
            tm = *(vp as *mut libc::c_longlong) as time_t;
            if tm == -(1 as libc::c_int) as libc::c_long {
                tm = time(0 as *mut time_t);
            }
            if txt.is_null() {
                exerror(b"printf: no time format provided\0" as *const u8 as *const libc::c_char);
            } else {
                s = fmtbuf(80 as libc::c_int as size_t);
                stm = localtime(&mut tm);
                let mut format: *mut libc::c_char = malloc(
                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(txt_len.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                ) as *mut libc::c_char;
                if format.is_null() {
                    exerror(b"printf: out of memory\0" as *const u8 as *const libc::c_char);
                } else {
                    strncpy(format, txt, txt_len);
                    *format.offset(txt_len as isize) = '\0' as i32 as libc::c_char;
                    strftime(s, 80 as libc::c_int as size_t, format, stm);
                    free(format as *mut libc::c_void);
                    let ref mut fresh7 = *(vp as *mut *mut libc::c_char);
                    *fresh7 = s;
                }
            }
            (*dp).fmt = 's' as i32;
            (*dp).size = -(1 as libc::c_int) as ssize_t;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn prints(
    mut ex: *mut Expr_t,
    mut exnode: *mut Exnode_t,
    mut env: *mut libc::c_void,
    mut sp: *mut Sfio_t,
) -> libc::c_int {
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut args: *mut Exnode_t = 0 as *mut Exnode_t;
    args = (*exnode).data.operand.left;
    while !args.is_null() {
        v = eval(ex, (*args).data.operand.left, env);
        sfputr(sp, v.string, -(1 as libc::c_int));
        args = (*args).data.operand.right;
    }
    if (*sp).next >= (*sp).endw {
        _sfflsbuf(sp, '\n' as i32 as libc::c_uchar as libc::c_int);
    } else {
        let ref mut fresh8 = (*sp).next;
        let fresh9 = *fresh8;
        *fresh8 = (*fresh8).offset(1);
        *fresh9 = '\n' as i32 as libc::c_uchar;
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn print(
    mut ex: *mut Expr_t,
    mut exnode: *mut Exnode_t,
    mut env: *mut libc::c_void,
    mut sp: *mut Sfio_t,
) -> libc::c_int {
    let mut x: *mut Print_t = 0 as *mut Print_t;
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut fmt: Fmt_t = Fmt_t {
        fmt: Sffmt_t {
            extf: None,
            eventf: None,
            form: 0 as *mut libc::c_char,
            args: (::std::mem::MaybeUninit::uninit()).assume_init(),
            fmt: 0,
            size: 0,
            flags: 0,
            width: 0,
            precis: 0,
            base: 0,
            t_str: 0 as *mut libc::c_char,
            n_str: 0,
        },
        expr: 0 as *mut Expr_t,
        env: 0 as *mut libc::c_void,
        args: 0 as *mut Print_t,
        value: EX_STYPE {
            expr: 0 as *mut Exnode_s,
        },
        actuals: 0 as *mut Exnode_t,
    };
    if sp.is_null() {
        v = eval(ex, (*exnode).data.print.descriptor, env);
        if v.integer < 0 as libc::c_int as libc::c_longlong
            || v.integer as libc::c_ulonglong
                >= (::std::mem::size_of::<[*mut Sfio_t; 10]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut Sfio_t>() as libc::c_ulong)
                    as libc::c_ulonglong
            || {
                sp = (*ex).file[v.integer as usize];
                sp.is_null() && {
                    let ref mut fresh10 = (*ex).file[v.integer as usize];
                    *fresh10 = sfnew(
                        0 as *mut Sfio_t,
                        0 as *mut libc::c_void,
                        18446744073709551615 as libc::c_ulong,
                        v.integer as libc::c_int,
                        0o1 as libc::c_int | 0o2 as libc::c_int,
                    );
                    sp = *fresh10;
                    sp.is_null()
                }
            }
        {
            exerror(
                b"printf: %ld: invalid descriptor\0" as *const u8 as *const libc::c_char,
                v.integer as intmax_t,
            );
            return -(1 as libc::c_int);
        }
    }
    memset(
        &mut fmt as *mut Fmt_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Fmt_t>() as libc::c_ulong,
    );
    fmt.fmt.extf =
        Some(prformat as unsafe extern "C" fn(*mut libc::c_void, *mut Sffmt_t) -> libc::c_int);
    fmt.expr = ex;
    fmt.env = env;
    x = (*exnode).data.print.args;
    if !((*x).format).is_null() {
        loop {
            if !((*x).arg).is_null() {
                fmt.fmt.form = (*x).format;
                fmt.args = x;
                sfprintf(
                    sp,
                    b"%!\0" as *const u8 as *const libc::c_char,
                    &mut fmt as *mut Fmt_t,
                );
            } else {
                sfputr(sp, (*x).format, -(1 as libc::c_int));
            }
            x = (*x).next;
            if x.is_null() {
                break;
            }
        }
    } else {
        v = eval(ex, (*(*x).arg).data.operand.left, env);
        fmt.fmt.form = v.string;
        fmt.actuals = (*x).arg;
        sfprintf(
            sp,
            b"%!\0" as *const u8 as *const libc::c_char,
            &mut fmt as *mut Fmt_t,
        );
        if !((*fmt.actuals).data.operand.right).is_null() {
            exerror(
                b"(s)printf: \"%s\": too many arguments\0" as *const u8 as *const libc::c_char,
                fmt.fmt.form,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn scformat(mut vp: *mut libc::c_void, mut dp: *mut Sffmt_t) -> libc::c_int {
    let mut fmt: *mut Fmt_t = dp as *mut Fmt_t;
    let mut node: *mut Exnode_t = 0 as *mut Exnode_t;
    if ((*fmt).actuals).is_null() {
        exerror(b"scanf: not enough arguments\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    node = (*(*fmt).actuals).data.operand.left;
    match (*dp).fmt {
        102 | 103 => {
            if (*node).type_0 != 262 as libc::c_int {
                exerror(
                    b"scanf: %s: floating variable address argument expected\0" as *const u8
                        as *const libc::c_char,
                    ((*(*node).data.variable.symbol).name).as_mut_ptr(),
                );
                return -(1 as libc::c_int);
            }
            (*fmt).fmt.size = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as ssize_t;
            let ref mut fresh11 = *(vp as *mut *mut libc::c_void);
            *fresh11 = &mut (*(*(*node).data.variable.symbol).value).data.constant.value
                as *mut Extype_t as *mut libc::c_void;
        }
        115 | 91 => {
            if (*node).type_0 != 263 as libc::c_int {
                exerror(
                    b"scanf: %s: string variable address argument expected\0" as *const u8
                        as *const libc::c_char,
                    ((*(*node).data.variable.symbol).name).as_mut_ptr(),
                );
                return -(1 as libc::c_int);
            }
            if (*(*(*node).data.variable.symbol).value)
                .data
                .constant
                .value
                .string
                == (expr.nullstring).as_mut_ptr()
            {
                let ref mut fresh12 = (*(*(*node).data.variable.symbol).value)
                    .data
                    .constant
                    .value
                    .string;
                *fresh12 = 0 as *mut libc::c_char;
            }
            (*fmt).fmt.size = 1024 as libc::c_int as ssize_t;
            let mut s: *mut libc::c_char = (*(*(*node).data.variable.symbol).value)
                .data
                .constant
                .value
                .string;
            vmfree((*(*fmt).expr).vm, s as *mut libc::c_void);
            s = vmalloc(
                (*(*fmt).expr).vm,
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((*fmt).fmt.size as size_t),
            ) as *mut libc::c_char;
            memset(
                s as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((*fmt).fmt.size as size_t),
            );
            let ref mut fresh13 = *(vp as *mut *mut libc::c_void);
            *fresh13 = s as *mut libc::c_void;
            let ref mut fresh14 = (*(*(*node).data.variable.symbol).value)
                .data
                .constant
                .value
                .string;
            *fresh14 = s;
        }
        99 => {
            if (*node).type_0 != 261 as libc::c_int {
                exerror(
                    b"scanf: %s: char variable address argument expected\0" as *const u8
                        as *const libc::c_char,
                    ((*(*node).data.variable.symbol).name).as_mut_ptr(),
                );
                return -(1 as libc::c_int);
            }
            (*fmt).fmt.size = ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong as ssize_t;
            let ref mut fresh15 = *(vp as *mut *mut libc::c_void);
            *fresh15 = &mut (*(*(*node).data.variable.symbol).value).data.constant.value
                as *mut Extype_t as *mut libc::c_void;
        }
        _ => {
            if (*node).type_0 != 259 as libc::c_int && (*node).type_0 != 260 as libc::c_int {
                exerror(
                    b"scanf: %s: integer variable address argument expected\0" as *const u8
                        as *const libc::c_char,
                    ((*(*node).data.variable.symbol).name).as_mut_ptr(),
                );
                return -(1 as libc::c_int);
            }
            (*dp).size = ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong as ssize_t;
            let ref mut fresh16 = *(vp as *mut *mut libc::c_void);
            *fresh16 = &mut (*(*(*node).data.variable.symbol).value).data.constant.value
                as *mut Extype_t as *mut libc::c_void;
        }
    }
    let ref mut fresh17 = (*fmt).actuals;
    *fresh17 = (*(*fmt).actuals).data.operand.right;
    (*dp).flags |= 0o400000 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn scan(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut env: *mut libc::c_void,
    mut sp: *mut Sfio_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut u: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut fmt: Fmt_t = Fmt_t {
        fmt: Sffmt_t {
            extf: None,
            eventf: None,
            form: 0 as *mut libc::c_char,
            args: (::std::mem::MaybeUninit::uninit()).assume_init(),
            fmt: 0,
            size: 0,
            flags: 0,
            width: 0,
            precis: 0,
            base: 0,
            t_str: 0 as *mut libc::c_char,
            n_str: 0,
        },
        expr: 0 as *mut Expr_t,
        env: 0 as *mut libc::c_void,
        args: 0 as *mut Print_t,
        value: EX_STYPE {
            expr: 0 as *mut Exnode_s,
        },
        actuals: 0 as *mut Exnode_t,
    };
    let mut n: libc::c_int = 0;
    if sp.is_null() {
        if !((*expr_0).data.scan.descriptor).is_null() {
            v = eval(ex, (*expr_0).data.scan.descriptor, env);
            if (*(*expr_0).data.scan.descriptor).type_0 == 263 as libc::c_int {
                current_block = 6518399209965601516;
            } else {
                current_block = 7351195479953500246;
            }
        } else {
            v.integer = 0 as libc::c_int as libc::c_longlong;
            current_block = 7351195479953500246;
        }
        match current_block {
            6518399209965601516 => {}
            _ => {
                if v.integer < 0 as libc::c_int as libc::c_longlong
                    || v.integer as libc::c_ulonglong
                        >= (::std::mem::size_of::<[*mut Sfio_t; 10]>() as libc::c_ulong)
                            .wrapping_div(::std::mem::size_of::<*mut Sfio_t>() as libc::c_ulong)
                            as libc::c_ulonglong
                    || {
                        sp = (*ex).file[v.integer as usize];
                        sp.is_null() && {
                            let ref mut fresh18 = (*ex).file[v.integer as usize];
                            *fresh18 = sfnew(
                                0 as *mut Sfio_t,
                                0 as *mut libc::c_void,
                                18446744073709551615 as libc::c_ulong,
                                v.integer as libc::c_int,
                                0o1 as libc::c_int | 0o2 as libc::c_int,
                            );
                            sp = *fresh18;
                            sp.is_null()
                        }
                    }
                {
                    exerror(
                        b"scanf: %ld: invalid descriptor\0" as *const u8 as *const libc::c_char,
                        v.integer as intmax_t,
                    );
                    return 0 as libc::c_int;
                }
            }
        }
    }
    memset(
        &mut fmt as *mut Fmt_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Fmt_t>() as libc::c_ulong,
    );
    fmt.fmt.extf =
        Some(scformat as unsafe extern "C" fn(*mut libc::c_void, *mut Sffmt_t) -> libc::c_int);
    fmt.expr = ex;
    fmt.env = env;
    u = eval(ex, (*expr_0).data.scan.format, env);
    fmt.fmt.form = u.string;
    fmt.actuals = (*expr_0).data.scan.args;
    n = if !sp.is_null() {
        sfscanf(
            sp,
            b"%!\0" as *const u8 as *const libc::c_char,
            &mut fmt as *mut Fmt_t,
        )
    } else {
        sfsscanf(
            v.string,
            b"%!\0" as *const u8 as *const libc::c_char,
            &mut fmt as *mut Fmt_t,
        )
    };
    if !(fmt.actuals).is_null() && *fmt.fmt.form == 0 {
        exerror(
            b"scanf: %s: too many arguments\0" as *const u8 as *const libc::c_char,
            ((*(*(*fmt.actuals).data.operand.left).data.variable.symbol).name).as_mut_ptr(),
        );
    }
    return n;
}
unsafe extern "C" fn str_add(
    mut ex: *mut Expr_t,
    mut l: *mut libc::c_char,
    mut r: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut sz: size_t = (strlen(l))
        .wrapping_add(strlen(r))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut s: *mut libc::c_char = vmalloc((*ex).ve, sz) as *mut libc::c_char;
    if s.is_null() {
        return exnospace();
    }
    snprintf(s, sz, b"%s%s\0" as *const u8 as *const libc::c_char, l, r);
    return s;
}
unsafe extern "C" fn str_ior(
    mut ex: *mut Expr_t,
    mut l: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 1 as libc::c_int as size_t;
    let mut p: *const libc::c_char = l;
    while *p as libc::c_int != '\0' as i32 {
        if (strchr(p.offset(1 as libc::c_int as isize), *p as libc::c_int)).is_null() {
            len = len.wrapping_add(1);
        }
        p = p.offset(1);
    }
    let mut p_0: *const libc::c_char = r;
    while *p_0 as libc::c_int != '\0' as i32 {
        if (strchr(l, *p_0 as libc::c_int)).is_null()
            && (strchr(p_0.offset(1 as libc::c_int as isize), *p_0 as libc::c_int)).is_null()
        {
            len = len.wrapping_add(1);
        }
        p_0 = p_0.offset(1);
    }
    let mut result: *mut libc::c_char = vmalloc((*ex).ve, len) as *mut libc::c_char;
    if result.is_null() {
        return exnospace();
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut p_1: *const libc::c_char = l;
    while *p_1 as libc::c_int != '\0' as i32 {
        if (strchr(p_1.offset(1 as libc::c_int as isize), *p_1 as libc::c_int)).is_null() {
            if i < len
                && !(b"incorrect preceding length computation\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"i < len && \"incorrect preceding length computation\"\0" as *const u8
                        as *const libc::c_char,
                    b"exeval.c\0" as *const u8 as *const libc::c_char,
                    589 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"char *str_ior(Expr_t *, const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
            *result.offset(i as isize) = *p_1;
            i = i.wrapping_add(1);
        }
        p_1 = p_1.offset(1);
    }
    let mut p_2: *const libc::c_char = r;
    while *p_2 as libc::c_int != '\0' as i32 {
        if (strchr(l, *p_2 as libc::c_int)).is_null()
            && (strchr(p_2.offset(1 as libc::c_int as isize), *p_2 as libc::c_int)).is_null()
        {
            if i < len
                && !(b"incorrect preceding length computation\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"i < len && \"incorrect preceding length computation\"\0" as *const u8
                        as *const libc::c_char,
                    b"exeval.c\0" as *const u8 as *const libc::c_char,
                    596 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"char *str_ior(Expr_t *, const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
            *result.offset(i as isize) = *p_2;
            i = i.wrapping_add(1);
        }
        p_2 = p_2.offset(1);
    }
    if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == len
        && !(b"incorrect preceding length computation\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"i + 1 == len && \"incorrect preceding length computation\"\0" as *const u8
                as *const libc::c_char,
            b"exeval.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"char *str_ior(Expr_t *, const char *, const char *)\0",
            ))
            .as_ptr(),
        );
    }
    *result.offset(i as isize) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn str_and(
    mut ex: *mut Expr_t,
    mut l: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 1 as libc::c_int as size_t;
    let mut p: *const libc::c_char = l;
    while *p as libc::c_int != '\0' as i32 {
        if !(strchr(r, *p as libc::c_int)).is_null()
            && (strchr(p.offset(1 as libc::c_int as isize), *p as libc::c_int)).is_null()
        {
            len = len.wrapping_add(1);
        }
        p = p.offset(1);
    }
    let mut result: *mut libc::c_char = vmalloc((*ex).ve, len) as *mut libc::c_char;
    if result.is_null() {
        return exnospace();
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut p_0: *const libc::c_char = l;
    while *p_0 as libc::c_int != '\0' as i32 {
        if !(strchr(r, *p_0 as libc::c_int)).is_null()
            && (strchr(p_0.offset(1 as libc::c_int as isize), *p_0 as libc::c_int)).is_null()
        {
            if i < len
                && !(b"incorrect preceding length computation\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"i < len && \"incorrect preceding length computation\"\0" as *const u8
                        as *const libc::c_char,
                    b"exeval.c\0" as *const u8 as *const libc::c_char,
                    631 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"char *str_and(Expr_t *, const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
            *result.offset(i as isize) = *p_0;
            i = i.wrapping_add(1);
        }
        p_0 = p_0.offset(1);
    }
    if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == len
        && !(b"incorrect preceding length computation\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"i + 1 == len && \"incorrect preceding length computation\"\0" as *const u8
                as *const libc::c_char,
            b"exeval.c\0" as *const u8 as *const libc::c_char,
            636 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"char *str_and(Expr_t *, const char *, const char *)\0",
            ))
            .as_ptr(),
        );
    }
    *result.offset(i as isize) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn str_xor(
    mut ex: *mut Expr_t,
    mut l: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 1 as libc::c_int as size_t;
    let mut p: *const libc::c_char = l;
    while *p as libc::c_int != '\0' as i32 {
        if (strchr(r, *p as libc::c_int)).is_null()
            && (strchr(p.offset(1 as libc::c_int as isize), *p as libc::c_int)).is_null()
        {
            len = len.wrapping_add(1);
        }
        p = p.offset(1);
    }
    let mut p_0: *const libc::c_char = r;
    while *p_0 as libc::c_int != '\0' as i32 {
        if (strchr(l, *p_0 as libc::c_int)).is_null()
            && (strchr(p_0.offset(1 as libc::c_int as isize), *p_0 as libc::c_int)).is_null()
        {
            len = len.wrapping_add(1);
        }
        p_0 = p_0.offset(1);
    }
    let mut result: *mut libc::c_char = vmalloc((*ex).ve, len) as *mut libc::c_char;
    if result.is_null() {
        return exnospace();
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut p_1: *const libc::c_char = l;
    while *p_1 as libc::c_int != '\0' as i32 {
        if (strchr(r, *p_1 as libc::c_int)).is_null()
            && (strchr(p_1.offset(1 as libc::c_int as isize), *p_1 as libc::c_int)).is_null()
        {
            if i < len
                && !(b"incorrect preceding length computation\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"i < len && \"incorrect preceding length computation\"\0" as *const u8
                        as *const libc::c_char,
                    b"exeval.c\0" as *const u8 as *const libc::c_char,
                    671 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"char *str_xor(Expr_t *, const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
            *result.offset(i as isize) = *p_1;
            i = i.wrapping_add(1);
        }
        p_1 = p_1.offset(1);
    }
    let mut p_2: *const libc::c_char = r;
    while *p_2 as libc::c_int != '\0' as i32 {
        if (strchr(l, *p_2 as libc::c_int)).is_null()
            && (strchr(p_2.offset(1 as libc::c_int as isize), *p_2 as libc::c_int)).is_null()
        {
            if i < len
                && !(b"incorrect preceding length computation\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"i < len && \"incorrect preceding length computation\"\0" as *const u8
                        as *const libc::c_char,
                    b"exeval.c\0" as *const u8 as *const libc::c_char,
                    678 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"char *str_xor(Expr_t *, const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
            *result.offset(i as isize) = *p_2;
            i = i.wrapping_add(1);
        }
        p_2 = p_2.offset(1);
    }
    if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == len
        && !(b"incorrect preceding length computation\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"i + 1 == len && \"incorrect preceding length computation\"\0" as *const u8
                as *const libc::c_char,
            b"exeval.c\0" as *const u8 as *const libc::c_char,
            683 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"char *str_xor(Expr_t *, const char *, const char *)\0",
            ))
            .as_ptr(),
        );
    }
    *result.offset(i as isize) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn str_mod(
    mut ex: *mut Expr_t,
    mut l: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 1 as libc::c_int as size_t;
    let mut p: *const libc::c_char = l;
    while *p as libc::c_int != '\0' as i32 {
        if (strchr(r, *p as libc::c_int)).is_null()
            && (strchr(p.offset(1 as libc::c_int as isize), *p as libc::c_int)).is_null()
        {
            len = len.wrapping_add(1);
        }
        p = p.offset(1);
    }
    let mut result: *mut libc::c_char = vmalloc((*ex).ve, len) as *mut libc::c_char;
    if result.is_null() {
        return exnospace();
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut p_0: *const libc::c_char = l;
    while *p_0 as libc::c_int != '\0' as i32 {
        if (strchr(r, *p_0 as libc::c_int)).is_null()
            && (strchr(p_0.offset(1 as libc::c_int as isize), *p_0 as libc::c_int)).is_null()
        {
            if i < len
                && !(b"incorrect preceding length computation\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"i < len && \"incorrect preceding length computation\"\0" as *const u8
                        as *const libc::c_char,
                    b"exeval.c\0" as *const u8 as *const libc::c_char,
                    713 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"char *str_mod(Expr_t *, const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
            *result.offset(i as isize) = *p_0;
            i = i.wrapping_add(1);
        }
        p_0 = p_0.offset(1);
    }
    if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == len
        && !(b"incorrect preceding length computation\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"i + 1 == len && \"incorrect preceding length computation\"\0" as *const u8
                as *const libc::c_char,
            b"exeval.c\0" as *const u8 as *const libc::c_char,
            718 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"char *str_mod(Expr_t *, const char *, const char *)\0",
            ))
            .as_ptr(),
        );
    }
    *result.offset(i as isize) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn str_mpy(
    mut ex: *mut Expr_t,
    mut l: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = strlen(l);
    let mut len2: size_t = strlen(r);
    if len2 < len {
        len = len2;
    }
    len = len.wrapping_add(1);
    let mut result: *mut libc::c_char = vmalloc((*ex).ve, len) as *mut libc::c_char;
    if result.is_null() {
        return exnospace();
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while *l.offset(i as isize) as libc::c_int != '\0' as i32
        && *r.offset(i as isize) as libc::c_int != '\0' as i32
    {
        if i < len
            && !(b"incorrect preceding length computation\0" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"i < len && \"incorrect preceding length computation\"\0" as *const u8
                    as *const libc::c_char,
                b"exeval.c\0" as *const u8 as *const libc::c_char,
                749 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                    b"char *str_mpy(Expr_t *, const char *, const char *)\0",
                ))
                .as_ptr(),
            );
        }
        *result.offset(i as isize) =
            (if *l.offset(i as isize) as libc::c_int == *r.offset(i as isize) as libc::c_int {
                *l.offset(i as isize) as libc::c_int
            } else {
                ' ' as i32
            }) as libc::c_char;
        i = i.wrapping_add(1);
    }
    if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == len
        && !(b"incorrect preceding length computation\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"i + 1 == len && \"incorrect preceding length computation\"\0" as *const u8
                as *const libc::c_char,
            b"exeval.c\0" as *const u8 as *const libc::c_char,
            752 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"char *str_mpy(Expr_t *, const char *, const char *)\0",
            ))
            .as_ptr(),
        );
    }
    *result.offset(i as isize) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn buffer_append(
    mut b: *mut buffer_t,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if ((*b).capacity).wrapping_sub((*b).size) < len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut c: size_t = if (*b).capacity == 0 as libc::c_int as libc::c_ulong {
            8192 as libc::c_int as libc::c_ulong
        } else {
            ((*b).capacity).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        };
        if c.wrapping_sub((*b).size) < len.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            c = ((*b).size)
                .wrapping_add(len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
        let mut p: *mut libc::c_char =
            vmresize((*b).allocator, (*b).base as *mut libc::c_void, c) as *mut libc::c_char;
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        let ref mut fresh19 = (*b).base;
        *fresh19 = p;
        (*b).capacity = c;
    }
    if ((*b).capacity).wrapping_sub((*b).size)
        >= len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        && !(b"incorrect logic in buffer expansion; still no room for appended string\0"
            as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"b->capacity - b->size >= len + 1 && \"incorrect logic in buffer expansion; still no room for appended \" \"string\"\0"
                as *const u8 as *const libc::c_char,
            b"exeval.c\0" as *const u8 as *const libc::c_char,
            790 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"int buffer_append(buffer_t *, const char *, size_t)\0"))
                .as_ptr(),
        );
    }
    strncpy(((*b).base).offset((*b).size as isize), s, len);
    let ref mut fresh20 = (*b).size;
    *fresh20 = (*fresh20 as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    *((*b).base).offset((*b).size as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn replace(
    mut s: *mut buffer_t,
    mut base: *mut libc::c_char,
    mut repl: *mut libc::c_char,
    mut ng: libc::c_int,
    mut sub: *mut libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut idx: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    loop {
        let fresh21 = repl;
        repl = repl.offset(1);
        c = *fresh21;
        if !(c != 0) {
            break;
        }
        if c as libc::c_int == '\\' as i32 {
            c = *repl;
            if c as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
            {
                idx = c as libc::c_int - '0' as i32;
                if idx < ng {
                    offset = *sub.offset((2 as libc::c_int * idx) as isize);
                    if buffer_append(
                        s,
                        base.offset(offset as isize),
                        (*sub.offset((2 as libc::c_int * idx + 1 as libc::c_int) as isize) - offset)
                            as size_t,
                    ) != 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                }
                repl = repl.offset(1);
            } else if buffer_append(
                s,
                b"\\\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            ) != 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
        } else if buffer_append(s, &mut c, 1 as libc::c_int as size_t) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn addItem(mut arr: *mut Dt_t, mut v: Extype_t, mut tok: *mut libc::c_char) {
    let mut b: *mut Exassoc_t = 0 as *mut Exassoc_t;
    b = (Some(((*arr).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        arr,
        &mut v as *mut Extype_t as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut Exassoc_t;
    if b.is_null() {
        b = if 0 as libc::c_int != 0 {
            realloc(
                0 as *mut libc::c_char as *mut libc::c_void,
                (::std::mem::size_of::<Exassoc_t>() as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as libc::c_ulong),
            ) as *mut Exassoc_t
        } else {
            calloc(
                1 as libc::c_int as libc::c_ulong,
                (::std::mem::size_of::<Exassoc_t>() as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as libc::c_ulong),
            ) as *mut Exassoc_t
        };
        if b.is_null() {
            exerror(b"out of space [assoc]\0" as *const u8 as *const libc::c_char);
        }
        (*b).key = v;
        (Some(((*arr).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            arr, b as *mut libc::c_void, 0o1 as libc::c_int
        );
    }
    let ref mut fresh22 = (*b).value.string;
    *fresh22 = tok;
}
unsafe extern "C" fn exsplit(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut env: *mut libc::c_void,
) -> Extype_t {
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut seps: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0;
    let mut arr: *mut Dt_t = (*(*expr_0).data.split.array).local.pointer as *mut Dt_t;
    str = (eval(ex, (*expr_0).data.split.string, env)).string;
    if !((*expr_0).data.split.seps).is_null() {
        seps = (eval(ex, (*expr_0).data.split.seps, env)).string;
    } else {
        seps = b" \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    v.integer = 0 as libc::c_int as libc::c_longlong;
    while *str != 0 {
        sz = strspn(str, seps);
        if sz != 0 {
            if v.integer == 0 as libc::c_int as libc::c_longlong {
                addItem(
                    arr,
                    v,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                v.integer += 1;
            }
            let mut i: size_t = 1 as libc::c_int as size_t;
            while i < sz {
                addItem(
                    arr,
                    v,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                v.integer += 1;
                i = i.wrapping_add(1);
            }
        }
        str = str.offset(sz as isize);
        if *str as libc::c_int == '\0' as i32 {
            addItem(
                arr,
                v,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            v.integer += 1;
            break;
        } else {
            sz = strcspn(str, seps);
            tok = vmalloc((*ex).vm, sz.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            if tok.is_null() {
                tok = exnospace();
            } else {
                memcpy(tok as *mut libc::c_void, str as *const libc::c_void, sz);
                *tok.offset(sz as isize) = '\0' as i32 as libc::c_char;
            }
            addItem(arr, v, tok);
            v.integer += 1;
            str = str.offset(sz as isize);
        }
    }
    return v;
}
unsafe extern "C" fn extokens(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut env: *mut libc::c_void,
) -> Extype_t {
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut seps: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0;
    let mut arr: *mut Dt_t = (*(*expr_0).data.split.array).local.pointer as *mut Dt_t;
    str = (eval(ex, (*expr_0).data.split.string, env)).string;
    if !((*expr_0).data.split.seps).is_null() {
        seps = (eval(ex, (*expr_0).data.split.seps, env)).string;
    } else {
        seps = b" \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    v.integer = 0 as libc::c_int as libc::c_longlong;
    while *str != 0 {
        sz = strspn(str, seps);
        str = str.offset(sz as isize);
        if *str as libc::c_int == '\0' as i32 {
            break;
        }
        sz = strcspn(str, seps);
        if sz != 0 {
        } else {
            __assert_fail(
                b"sz\0" as *const u8 as *const libc::c_char,
                b"exeval.c\0" as *const u8 as *const libc::c_char,
                933 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"Extype_t extokens(Expr_t *, Exnode_t *, void *)\0",
                ))
                .as_ptr(),
            );
        }
        tok = vmalloc((*ex).vm, sz.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if tok.is_null() {
            tok = exnospace();
        } else {
            memcpy(tok as *mut libc::c_void, str as *const libc::c_void, sz);
            *tok.offset(sz as isize) = '\0' as i32 as libc::c_char;
        }
        addItem(arr, v, tok);
        v.integer += 1;
        str = str.offset(sz as isize);
    }
    return v;
}
unsafe extern "C" fn exsub(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut env: *mut libc::c_void,
    mut global: bool,
) -> Extype_t {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut repl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut sub: [libc::c_int; 20] = [0; 20];
    let mut flags: libc::c_int = 0o1 as libc::c_int;
    let mut ng: libc::c_int = 0;
    str = (eval(ex, (*expr_0).data.string.base, env)).string;
    pat = (eval(ex, (*expr_0).data.string.pat, env)).string;
    if !((*expr_0).data.string.repl).is_null() {
        repl = (eval(ex, (*expr_0).data.string.repl, env)).string;
    } else {
        repl = 0 as *mut libc::c_char;
    }
    if !global {
        if *pat as libc::c_int == '^' as i32 {
            pat = pat.offset(1);
            flags |= 0o2 as libc::c_int;
        }
        p = pat;
        while *p != 0 {
            p = p.offset(1);
        }
        if p > pat {
            p = p.offset(-1);
        }
        if *p as libc::c_int == '$' as i32 {
            if p > pat && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32 {
                let fresh23 = p;
                p = p.offset(-1);
                *fresh23 = '\0' as i32 as libc::c_char;
                *p = '$' as i32 as libc::c_char;
            } else {
                flags |= 0o4 as libc::c_int;
                *p = '\0' as i32 as libc::c_char;
            }
        }
    }
    if *pat as libc::c_int == '\0' as i32 {
        v.string = vmstrdup((*ex).ve, str);
        return v;
    }
    ng = strgrpmatch(
        str,
        pat,
        sub.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_int; 20]>() as libc::c_ulong).wrapping_div(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as libc::c_int,
        flags,
    );
    if ng == 0 as libc::c_int {
        v.string = vmstrdup((*ex).ve, str);
        return v;
    }
    if sub[0 as libc::c_int as usize] == sub[1 as libc::c_int as usize] {
        exwarn(
            b"pattern match of empty string - ill-specified pattern \"%s\"?\0" as *const u8
                as *const libc::c_char,
            pat,
        );
        v.string = vmstrdup((*ex).ve, str);
        return v;
    }
    let mut buffer: buffer_t = {
        let mut init = buffer_t {
            allocator: (*ex).ve,
            base: 0 as *mut libc::c_char,
            size: 0,
            capacity: 0,
        };
        init
    };
    if buffer_append(&mut buffer, str, sub[0 as libc::c_int as usize] as size_t) != 0 as libc::c_int
    {
        v.string = exnospace();
        return v;
    }
    if !repl.is_null() {
        if replace(&mut buffer, str, repl, ng, sub.as_mut_ptr()) != 0 as libc::c_int {
            v.string = exnospace();
            return v;
        }
    }
    s = str.offset(sub[1 as libc::c_int as usize] as isize);
    if global {
        loop {
            ng = strgrpmatch(
                s,
                pat,
                sub.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_int; 20]>() as libc::c_ulong).wrapping_div(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
                ) as libc::c_int,
                flags,
            );
            if !(ng != 0) {
                break;
            }
            if buffer_append(&mut buffer, s, sub[0 as libc::c_int as usize] as size_t)
                != 0 as libc::c_int
            {
                v.string = exnospace();
                return v;
            }
            if !repl.is_null() {
                if replace(&mut buffer, s, repl, ng, sub.as_mut_ptr()) != 0 as libc::c_int {
                    v.string = exnospace();
                    return v;
                }
            }
            s = s.offset(sub[1 as libc::c_int as usize] as isize);
        }
    }
    if buffer_append(&mut buffer, s, strlen(s)) != 0 as libc::c_int {
        v.string = exnospace();
        return v;
    }
    v.string = buffer.base;
    return v;
}
unsafe extern "C" fn exsubstr(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut env: *mut libc::c_void,
) -> Extype_t {
    let mut s: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut i: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut l: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut len: libc::c_int = 0;
    s = eval(ex, (*expr_0).data.string.base, env);
    len = strlen(s.string) as libc::c_int;
    i = eval(ex, (*expr_0).data.string.pat, env);
    if i.integer < 0 as libc::c_int as libc::c_longlong || (len as libc::c_longlong) < i.integer {
        exerror(
            b"illegal start index in substr(%s,%ld)\0" as *const u8 as *const libc::c_char,
            s.string,
            i.integer as intmax_t,
        );
    }
    if !((*expr_0).data.string.repl).is_null() {
        l = eval(ex, (*expr_0).data.string.repl, env);
        if l.integer < 0 as libc::c_int as libc::c_longlong
            || len as libc::c_longlong - i.integer < l.integer
        {
            exerror(
                b"illegal length in substr(%s,%ld,%ld)\0" as *const u8 as *const libc::c_char,
                s.string,
                i.integer as intmax_t,
                l.integer as intmax_t,
            );
        }
    } else {
        l.integer = len as libc::c_longlong - i.integer;
    }
    v.string = vmalloc(
        (*ex).ve,
        (l.integer + 1 as libc::c_int as libc::c_longlong) as size_t,
    ) as *mut libc::c_char;
    if !((*expr_0).data.string.repl).is_null() {
        strncpy(
            v.string,
            (s.string).offset(i.integer as isize),
            l.integer as libc::c_ulong,
        );
        *(v.string).offset(l.integer as isize) = '\0' as i32 as libc::c_char;
    } else {
        strcpy(v.string, (s.string).offset(i.integer as isize));
    }
    return v;
}
unsafe extern "C" fn xConvert(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut type_0: libc::c_int,
    mut v: Extype_t,
    mut tmp: *mut Exnode_t,
) {
    *tmp = *(*expr_0).data.operand.left;
    (*tmp).data.constant.value = v;
    if ((*(*ex).disc).convertf).expect("non-null function pointer")(
        ex,
        tmp,
        type_0,
        if !((*expr_0).data.operand.right).is_null() {
            (*(*expr_0).data.operand.right).data.variable.symbol
        } else {
            0 as *mut Exid_t
        },
        0 as libc::c_int,
        (*ex).disc,
    ) != 0
    {
        exerror(
            b"%s: cannot convert %s value to %s\0" as *const u8 as *const libc::c_char,
            ((*(*(*expr_0).data.operand.left).data.variable.symbol).name).as_mut_ptr(),
            extypename(ex, (*(*expr_0).data.operand.left).type_0),
            extypename(ex, type_0),
        );
    }
    (*tmp).type_0 = type_0;
}
unsafe extern "C" fn xPrint(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut v: Extype_t,
    mut tmp: *mut Exnode_t,
) {
    *tmp = *(*expr_0).data.operand.left;
    (*tmp).data.constant.value = v;
    if (Some(((*(*ex).disc).stringof).expect("non-null function pointer")))
        .expect("non-null function pointer")(ex, tmp, 0 as libc::c_int, (*ex).disc)
        != 0
    {
        exerror(
            b"%s: no string representation of %s value\0" as *const u8 as *const libc::c_char,
            ((*(*(*expr_0).data.operand.left).data.variable.symbol).name).as_mut_ptr(),
            extypename(ex, (*(*expr_0).data.operand.left).type_0),
        );
    }
    (*tmp).type_0 = 263 as libc::c_int;
}
static mut seed: libc::c_long = 0;
unsafe extern "C" fn eval(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut env: *mut libc::c_void,
) -> Extype_t {
    let mut current_block: u64;
    let mut x: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut a: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut t: *mut *mut Extype_t = 0 as *mut *mut Extype_t;
    let mut n: libc::c_int = 0;
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut r: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut i: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: Exnode_t = Exnode_t {
        type_0: 0,
        op: 0,
        binary: 0,
        local: Exlocal_t {
            number: 0,
            pointer: 0 as *mut libc::c_char,
        },
        compiled: C2RustUnnamed_12 { floating: None },
        data: Exdata_u {
            constant: C2RustUnnamed_11 {
                value: EX_STYPE {
                    expr: 0 as *mut Exnode_s,
                },
                reference: 0 as *mut Exid_t,
            },
        },
        subop: 0,
    };
    let mut rtmp: Exnode_t = Exnode_t {
        type_0: 0,
        op: 0,
        binary: 0,
        local: Exlocal_t {
            number: 0,
            pointer: 0 as *mut libc::c_char,
        },
        compiled: C2RustUnnamed_12 { floating: None },
        data: Exdata_u {
            constant: C2RustUnnamed_11 {
                value: EX_STYPE {
                    expr: 0 as *mut Exnode_s,
                },
                reference: 0 as *mut Exid_t,
            },
        },
        subop: 0,
    };
    let mut rp: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut assoc: *mut Exassoc_t = 0 as *mut Exassoc_t;
    let mut args: [Extype_t; 65] = [EX_STYPE {
        expr: 0 as *mut Exnode_s,
    }; 65];
    let mut save: [Extype_t; 64] = [EX_STYPE {
        expr: 0 as *mut Exnode_s,
    }; 64];
    if expr_0.is_null() || (*ex).loopcount != 0 {
        v.integer = 1 as libc::c_int as libc::c_longlong;
        return v;
    }
    x = (*expr_0).data.operand.left;
    match (*expr_0).op {
        268 | 272 => {
            v = eval(ex, x, env);
            (*ex).loopcount = v.integer as libc::c_int;
            (*ex).loopop = (*expr_0).op;
            return v;
        }
        271 => return (*expr_0).data.constant.value,
        334 => {
            n = -(1 as libc::c_int);
            current_block = 15143230840993836487;
        }
        275 => return getdyn(ex, expr_0, env, &mut assoc),
        298 => return exsplit(ex, expr_0, env),
        305 => return extokens(ex, expr_0, env),
        280 => return exsub(ex, expr_0, env, 1 as libc::c_int != 0),
        302 => return exsub(ex, expr_0, env, 0 as libc::c_int != 0),
        303 => return exsubstr(ex, expr_0, env),
        300 => {
            v.integer = seed as libc::c_longlong;
            if (*expr_0).binary != 0 {
                seed = (eval(ex, x, env)).integer as libc::c_long;
            } else {
                seed = time(0 as *mut time_t);
            }
            srand48(seed);
            return v;
        }
        295 => {
            v.floating = drand48();
            return v;
        }
        277 => {
            v = eval(ex, x, env);
            if ((*(*ex).disc).exitf).is_some() {
                ((*(*ex).disc).exitf).expect("non-null function pointer")(
                    ex,
                    env as *mut Exdisc_t,
                    v.integer as libc::c_int,
                );
            } else {
                graphviz_exit(v.integer as libc::c_int);
            }
            v.integer = -(1 as libc::c_int) as libc::c_longlong;
            return v;
        }
        284 => {
            v = eval(ex, x, env);
            if v.integer != 0 {
                eval(ex, (*(*expr_0).data.operand.right).data.operand.left, env);
            } else {
                eval(ex, (*(*expr_0).data.operand.right).data.operand.right, env);
            }
            v.integer = 1 as libc::c_int as libc::c_longlong;
            return v;
        }
        278 | 307 => {
            expr_0 = (*expr_0).data.operand.right;
            loop {
                r = eval(ex, x, env);
                if r.integer == 0 {
                    v.integer = 1 as libc::c_int as libc::c_longlong;
                    return v;
                }
                if !((*expr_0).data.operand.right).is_null() {
                    eval(ex, (*expr_0).data.operand.right, env);
                    if (*ex).loopcount > 0 as libc::c_int && {
                        let ref mut fresh24 = (*ex).loopcount;
                        *fresh24 -= 1;
                        *fresh24 > 0 as libc::c_int || (*ex).loopop != 272 as libc::c_int
                    } {
                        v.integer = 0 as libc::c_int as libc::c_longlong;
                        return v;
                    }
                }
                if !((*expr_0).data.operand.left).is_null() {
                    eval(ex, (*expr_0).data.operand.left, env);
                }
            }
        }
        304 => {
            v = eval(ex, x, env);
            i.integer = (*x).type_0 as libc::c_longlong;
            r.integer = 0 as libc::c_int as libc::c_longlong;
            x = (*expr_0).data.operand.right;
            a = (*x).data.select.statement;
            n = 0 as libc::c_int;
            loop {
                x = (*x).data.select.next;
                if x.is_null() {
                    break;
                }
                t = (*x).data.select.constant;
                if t.is_null() {
                    n = 1 as libc::c_int;
                } else {
                    let mut current_block_99: u64;
                    while !(*t).is_null() {
                        match i.integer as libc::c_int {
                            259 | 260 => {
                                if (**t).integer == v.integer {
                                    current_block_99 = 5195798230510548452;
                                } else {
                                    current_block_99 = 5265702136860997526;
                                }
                            }
                            263 => {
                                if if (*(*ex).disc).version
                                    >= 19981111 as libc::c_long as libc::c_ulong
                                    && ((*(*ex).disc).matchf).is_some()
                                {
                                    ((*(*ex).disc).matchf).expect("non-null function pointer")(
                                        ex,
                                        x,
                                        (**t).string,
                                        (*expr_0).data.operand.left,
                                        v.string,
                                        env,
                                        (*ex).disc,
                                    )
                                } else {
                                    strmatch((**t).string, v.string)
                                } != 0
                                {
                                    current_block_99 = 5195798230510548452;
                                } else {
                                    current_block_99 = 5265702136860997526;
                                }
                            }
                            262 => {
                                if (**t).floating == v.floating {
                                    current_block_99 = 5195798230510548452;
                                } else {
                                    current_block_99 = 5265702136860997526;
                                }
                            }
                            _ => {
                                current_block_99 = 5195798230510548452;
                            }
                        }
                        match current_block_99 {
                            5265702136860997526 => {
                                t = t.offset(1);
                            }
                            _ => {
                                n = 1 as libc::c_int;
                                break;
                            }
                        }
                    }
                }
                if !(n != 0) {
                    continue;
                }
                if ((*x).data.select.statement).is_null() {
                    r.integer = 1 as libc::c_int as libc::c_longlong;
                    break;
                } else {
                    r = eval(ex, (*x).data.select.statement, env);
                    if !((*ex).loopcount > 0 as libc::c_int) {
                        continue;
                    }
                    let ref mut fresh25 = (*ex).loopcount;
                    *fresh25 -= 1;
                    break;
                }
            }
            if n == 0 && !a.is_null() {
                r = eval(ex, a, env);
                if (*ex).loopcount > 0 as libc::c_int {
                    let ref mut fresh26 = (*ex).loopcount;
                    *fresh26 -= 1;
                }
            }
            return r;
        }
        281 => {
            v.integer = 0 as libc::c_int as libc::c_longlong;
            if (*(*expr_0).data.generate.array).op == 275 as libc::c_int {
                n = ((*(*expr_0).data.generate.index).type_0 == 263 as libc::c_int as libc::c_long)
                    as libc::c_int;
                assoc = (Some(
                    ((*((*(*(*expr_0).data.generate.array).data.variable.symbol)
                        .local
                        .pointer as *mut Dt_t))
                        .searchf)
                        .expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(
                    (*(*(*expr_0).data.generate.array).data.variable.symbol)
                        .local
                        .pointer as *mut Dt_t,
                    0 as *mut libc::c_void,
                    0o200 as libc::c_int,
                ) as *mut Exassoc_t;
                while !assoc.is_null() {
                    v.integer += 1;
                    if n != 0 {
                        let ref mut fresh27 = (*(*(*expr_0).data.generate.index).value)
                            .data
                            .constant
                            .value
                            .string;
                        *fresh27 = ((*assoc).name).as_mut_ptr();
                    } else {
                        (*(*(*expr_0).data.generate.index).value)
                            .data
                            .constant
                            .value = (*assoc).key;
                    }
                    eval(ex, (*expr_0).data.generate.statement, env);
                    if (*ex).loopcount > 0 as libc::c_int && {
                        let ref mut fresh28 = (*ex).loopcount;
                        *fresh28 -= 1;
                        *fresh28 > 0 as libc::c_int || (*ex).loopop != 272 as libc::c_int
                    } {
                        v.integer = 0 as libc::c_int as libc::c_longlong;
                        break;
                    } else {
                        assoc = (Some(
                            ((*((*(*(*expr_0).data.generate.array).data.variable.symbol)
                                .local
                                .pointer as *mut Dt_t))
                                .searchf)
                                .expect("non-null function pointer"),
                        ))
                        .expect("non-null function pointer")(
                            (*(*(*expr_0).data.generate.array).data.variable.symbol)
                                .local
                                .pointer as *mut Dt_t,
                            assoc as *mut libc::c_void,
                            0o10 as libc::c_int,
                        ) as *mut Exassoc_t;
                    }
                }
            } else {
                r = ((*(*ex).disc).getf).expect("non-null function pointer")(
                    ex,
                    expr_0,
                    (*(*expr_0).data.generate.array).data.variable.symbol,
                    (*(*expr_0).data.generate.array).data.variable.reference,
                    env,
                    0 as libc::c_int,
                    (*ex).disc,
                );
                v.integer = 0 as libc::c_int as libc::c_longlong;
                while v.integer < r.integer {
                    (*(*(*expr_0).data.generate.index).value)
                        .data
                        .constant
                        .value
                        .integer = v.integer;
                    eval(ex, (*expr_0).data.generate.statement, env);
                    if (*ex).loopcount > 0 as libc::c_int && {
                        let ref mut fresh29 = (*ex).loopcount;
                        *fresh29 -= 1;
                        *fresh29 > 0 as libc::c_int || (*ex).loopop != 272 as libc::c_int
                    } {
                        v.integer = 0 as libc::c_int as libc::c_longlong;
                        break;
                    } else {
                        v.integer += 1;
                    }
                }
            }
            return v;
        }
        282 => {
            v.integer = 0 as libc::c_int as libc::c_longlong;
            if (*(*expr_0).data.generate.array).op == 275 as libc::c_int {
                n = ((*(*expr_0).data.generate.index).type_0 == 263 as libc::c_int as libc::c_long)
                    as libc::c_int;
                assoc = (Some(
                    ((*((*(*(*expr_0).data.generate.array).data.variable.symbol)
                        .local
                        .pointer as *mut Dt_t))
                        .searchf)
                        .expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(
                    (*(*(*expr_0).data.generate.array).data.variable.symbol)
                        .local
                        .pointer as *mut Dt_t,
                    0 as *mut libc::c_void,
                    0o400 as libc::c_int,
                ) as *mut Exassoc_t;
                while !assoc.is_null() {
                    v.integer += 1;
                    if n != 0 {
                        let ref mut fresh30 = (*(*(*expr_0).data.generate.index).value)
                            .data
                            .constant
                            .value
                            .string;
                        *fresh30 = ((*assoc).name).as_mut_ptr();
                    } else {
                        (*(*(*expr_0).data.generate.index).value)
                            .data
                            .constant
                            .value = (*assoc).key;
                    }
                    eval(ex, (*expr_0).data.generate.statement, env);
                    if (*ex).loopcount > 0 as libc::c_int && {
                        let ref mut fresh31 = (*ex).loopcount;
                        *fresh31 -= 1;
                        *fresh31 > 0 as libc::c_int || (*ex).loopop != 272 as libc::c_int
                    } {
                        v.integer = 0 as libc::c_int as libc::c_longlong;
                        break;
                    } else {
                        assoc = (Some(
                            ((*((*(*(*expr_0).data.generate.array).data.variable.symbol)
                                .local
                                .pointer as *mut Dt_t))
                                .searchf)
                                .expect("non-null function pointer"),
                        ))
                        .expect("non-null function pointer")(
                            (*(*(*expr_0).data.generate.array).data.variable.symbol)
                                .local
                                .pointer as *mut Dt_t,
                            assoc as *mut libc::c_void,
                            0o20 as libc::c_int,
                        ) as *mut Exassoc_t;
                    }
                }
            } else {
                r = ((*(*ex).disc).getf).expect("non-null function pointer")(
                    ex,
                    expr_0,
                    (*(*expr_0).data.generate.array).data.variable.symbol,
                    (*(*expr_0).data.generate.array).data.variable.reference,
                    env,
                    0 as libc::c_int,
                    (*ex).disc,
                );
                v.integer = r.integer - 1 as libc::c_int as libc::c_longlong;
                while 0 as libc::c_int as libc::c_longlong <= v.integer {
                    (*(*(*expr_0).data.generate.index).value)
                        .data
                        .constant
                        .value
                        .integer = v.integer;
                    eval(ex, (*expr_0).data.generate.statement, env);
                    if (*ex).loopcount > 0 as libc::c_int && {
                        let ref mut fresh32 = (*ex).loopcount;
                        *fresh32 -= 1;
                        *fresh32 > 0 as libc::c_int || (*ex).loopop != 272 as libc::c_int
                    } {
                        v.integer = 0 as libc::c_int as libc::c_longlong;
                        break;
                    } else {
                        v.integer -= 1;
                    }
                }
            }
            return v;
        }
        35 => {
            v.integer = dtsize((*(*expr_0).data.variable.symbol).local.pointer as *mut Dt_t)
                as libc::c_longlong;
            return v;
        }
        331 => {
            v.integer = evaldyn(ex, expr_0, env, 0 as libc::c_int) as libc::c_longlong;
            return v;
        }
        306 => {
            if !((*expr_0).data.variable.index).is_null() {
                v.integer = evaldyn(ex, expr_0, env, 1 as libc::c_int) as libc::c_longlong;
            } else {
                (Some(
                    ((*((*(*expr_0).data.variable.symbol).local.pointer as *mut Dt_t)).searchf)
                        .expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(
                    (*(*expr_0).data.variable.symbol).local.pointer as *mut Dt_t,
                    0 as *mut libc::c_void,
                    0o100 as libc::c_int,
                );
                v.integer = 0 as libc::c_int as libc::c_longlong;
            }
            return v;
        }
        269 => {
            x = (*expr_0).data.call.args;
            n = 0 as libc::c_int;
            a = (*(*(*expr_0).data.call.procedure).value)
                .data
                .procedure
                .args;
            while !a.is_null() && !x.is_null() {
                if (n as libc::c_ulong)
                    < (::std::mem::size_of::<[Extype_t; 65]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Extype_t>() as libc::c_ulong)
                {
                    save[n as usize] = (*(*(*(*a).data.operand.left).data.variable.symbol).value)
                        .data
                        .constant
                        .value;
                    let fresh33 = n;
                    n = n + 1;
                    args[fresh33 as usize] = eval(ex, (*x).data.operand.left, env);
                } else {
                    (*(*(*(*a).data.operand.left).data.variable.symbol).value)
                        .data
                        .constant
                        .value = eval(ex, (*x).data.operand.left, env);
                }
                x = (*x).data.operand.right;
                a = (*a).data.operand.right;
            }
            n = 0 as libc::c_int;
            a = (*(*(*expr_0).data.call.procedure).value)
                .data
                .procedure
                .args;
            while !a.is_null()
                && (n as libc::c_ulong)
                    < (::std::mem::size_of::<[Extype_t; 64]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Extype_t>() as libc::c_ulong)
            {
                let fresh34 = n;
                n = n + 1;
                (*(*(*(*a).data.operand.left).data.variable.symbol).value)
                    .data
                    .constant
                    .value = args[fresh34 as usize];
                a = (*a).data.operand.right;
            }
            if !x.is_null() {
                exerror(b"too many actual args\0" as *const u8 as *const libc::c_char);
            } else if !a.is_null() {
                exerror(b"not enough actual args\0" as *const u8 as *const libc::c_char);
            }
            v = exeval(
                ex,
                (*(*(*expr_0).data.call.procedure).value)
                    .data
                    .procedure
                    .body,
                env,
            );
            n = 0 as libc::c_int;
            a = (*(*(*expr_0).data.call.procedure).value)
                .data
                .procedure
                .args;
            while !a.is_null()
                && (n as libc::c_ulong)
                    < (::std::mem::size_of::<[Extype_t; 64]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Extype_t>() as libc::c_ulong)
            {
                let fresh35 = n;
                n = n + 1;
                (*(*(*(*a).data.operand.left).data.variable.symbol).value)
                    .data
                    .constant
                    .value = save[fresh35 as usize];
                a = (*a).data.operand.right;
            }
            return v;
        }
        267 => {
            n = 0 as libc::c_int;
            x = (*expr_0).data.operand.right;
            while !x.is_null()
                && (n as libc::c_ulong)
                    < (::std::mem::size_of::<[Extype_t; 65]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Extype_t>() as libc::c_ulong)
            {
                let fresh36 = n;
                n = n + 1;
                args[fresh36 as usize] = eval(ex, (*x).data.operand.left, env);
                x = (*x).data.operand.right;
            }
            return ((*(*ex).disc).getf).expect("non-null function pointer")(
                ex,
                (*expr_0).data.operand.left,
                (*(*expr_0).data.operand.left).data.variable.symbol,
                (*(*expr_0).data.operand.left).data.variable.reference,
                args.as_mut_ptr() as *mut libc::c_void,
                -(3 as libc::c_int),
                (*ex).disc,
            );
        }
        279 => {
            n = 0 as libc::c_int;
            let fresh37 = n;
            n = n + 1;
            args[fresh37 as usize].string = env as *mut libc::c_char;
            x = (*expr_0).data.operand.right;
            while !x.is_null()
                && (n as libc::c_ulong)
                    < (::std::mem::size_of::<[Extype_t; 65]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Extype_t>() as libc::c_ulong)
            {
                let fresh38 = n;
                n = n + 1;
                args[fresh38 as usize] = eval(ex, (*x).data.operand.left, env);
                x = (*x).data.operand.right;
            }
            return ((*(*ex).disc).getf).expect("non-null function pointer")(
                ex,
                (*expr_0).data.operand.left,
                (*(*expr_0).data.operand.left).data.variable.symbol,
                (*(*expr_0).data.operand.left).data.variable.reference,
                args.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
                -(2 as libc::c_int),
                (*ex).disc,
            );
        }
        283 => {
            if !((*expr_0).data.variable.index).is_null() {
                i = eval(ex, (*expr_0).data.variable.index, env);
            } else {
                i.integer = -(1 as libc::c_int) as libc::c_longlong;
            }
            if !((*expr_0).data.variable.dyna).is_null() {
                let mut locv_1: Extype_t = EX_STYPE {
                    expr: 0 as *mut Exnode_s,
                };
                locv_1 = getdyn(ex, (*expr_0).data.variable.dyna, env, &mut assoc);
                (*(*(*expr_0).data.variable.dyna).data.variable.dyna)
                    .data
                    .constant
                    .value = locv_1;
            }
            return ((*(*ex).disc).getf).expect("non-null function pointer")(
                ex,
                expr_0,
                (*expr_0).data.variable.symbol,
                (*expr_0).data.variable.reference,
                env,
                i.integer as libc::c_int,
                (*ex).disc,
            );
        }
        333 => {
            n = 1 as libc::c_int;
            current_block = 15143230840993836487;
        }
        291 => {
            v.integer = prints(ex, expr_0, env, sfstdout) as libc::c_longlong;
            return v;
        }
        292 => {
            v.integer = print(ex, expr_0, env, 0 as *mut Sfio_t) as libc::c_longlong;
            return v;
        }
        296 => {
            (*ex).loopret = eval(ex, x, env);
            (*ex).loopcount = 32767 as libc::c_int;
            (*ex).loopop = (*expr_0).op;
            return (*ex).loopret;
        }
        297 | 301 => {
            v.integer = scan(ex, expr_0, env, 0 as *mut Sfio_t) as libc::c_longlong;
            return v;
        }
        299 => {
            let mut buffer: *mut Sfio_t = sfnew(
                0 as *mut Sfio_t,
                0 as *mut libc::c_char as *mut libc::c_void,
                -(1 as libc::c_int) as size_t,
                -(1 as libc::c_int),
                0o2 as libc::c_int | 0o4 as libc::c_int,
            );
            if buffer.is_null() {
                fprintf(
                    stderr,
                    b"out of memory\n\0" as *const u8 as *const libc::c_char,
                );
                graphviz_exit(1 as libc::c_int);
            }
            print(ex, expr_0, env, buffer);
            v.string = exstash(buffer, (*ex).ve);
            sfclose(buffer);
            return v;
        }
        61 => {
            v = eval(ex, (*expr_0).data.operand.right, env);
            if (*expr_0).subop != '=' as i32 {
                r = v;
                if (*x).op == 275 as libc::c_int {
                    v = getdyn(ex, x, env, &mut assoc);
                } else {
                    if !((*x).data.variable.index).is_null() {
                        v = eval(ex, (*x).data.variable.index, env);
                    } else {
                        v.integer = -(1 as libc::c_int) as libc::c_longlong;
                    }
                    if !((*x).data.variable.dyna).is_null() {
                        let mut locv_2: Extype_t = EX_STYPE {
                            expr: 0 as *mut Exnode_s,
                        };
                        locv_2 = getdyn(ex, (*x).data.variable.dyna, env, &mut assoc);
                        (*(*(*x).data.variable.dyna).data.variable.dyna)
                            .data
                            .constant
                            .value = locv_2;
                    }
                    v = ((*(*ex).disc).getf).expect("non-null function pointer")(
                        ex,
                        x,
                        (*x).data.variable.symbol,
                        (*x).data.variable.reference,
                        env,
                        v.integer as libc::c_int,
                        (*ex).disc,
                    );
                }
                match (*x).type_0 {
                    262 => {
                        current_block = 13787714106568798477;
                        match current_block {
                            13787714106568798477 => match (*expr_0).subop {
                                43 => {
                                    current_block = 11538870999030207062;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                45 => {
                                    current_block = 13236682061463366761;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 3790561273297268167;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                47 => {
                                    current_block = 18315435834437996746;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 7381876040995060082;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 14575275458752860498;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 8323211640247371006;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 11091651100383110420;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                329 => {
                                    current_block = 15385998965109782319;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                330 => {
                                    current_block = 15395163328284184564;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                            11515888952988033665 => match (*expr_0).subop {
                                43 => {
                                    current_block = 13034768034686864816;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                45 => {
                                    current_block = 3632332525568699835;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 3119644467204629641;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                47 => {
                                    current_block = 4155040754965348757;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 16153603300287884505;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 12154886366113648751;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 14747310749158537291;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 8869653012208538513;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                329 => {
                                    current_block = 14539215783803676789;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                330 => {
                                    current_block = 17916663512757813856;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                            _ => match (*expr_0).subop {
                                43 => {
                                    current_block = 3636433389734256616;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 6388602163840042976;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 516796709286287865;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 14728029199942657210;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 11002239204738513371;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 8050654829899665499;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                        }
                    }
                    259 | 260 => {
                        current_block = 11515888952988033665;
                        match current_block {
                            13787714106568798477 => match (*expr_0).subop {
                                43 => {
                                    current_block = 11538870999030207062;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                45 => {
                                    current_block = 13236682061463366761;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 3790561273297268167;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                47 => {
                                    current_block = 18315435834437996746;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 7381876040995060082;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 14575275458752860498;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 8323211640247371006;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 11091651100383110420;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                329 => {
                                    current_block = 15385998965109782319;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                330 => {
                                    current_block = 15395163328284184564;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                            11515888952988033665 => match (*expr_0).subop {
                                43 => {
                                    current_block = 13034768034686864816;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                45 => {
                                    current_block = 3632332525568699835;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 3119644467204629641;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                47 => {
                                    current_block = 4155040754965348757;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 16153603300287884505;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 12154886366113648751;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 14747310749158537291;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 8869653012208538513;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                329 => {
                                    current_block = 14539215783803676789;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                330 => {
                                    current_block = 17916663512757813856;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                            _ => match (*expr_0).subop {
                                43 => {
                                    current_block = 3636433389734256616;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 6388602163840042976;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 516796709286287865;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 14728029199942657210;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 11002239204738513371;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 8050654829899665499;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                        }
                    }
                    263 => {
                        current_block = 11238760380257417228;
                        match current_block {
                            13787714106568798477 => match (*expr_0).subop {
                                43 => {
                                    current_block = 11538870999030207062;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                45 => {
                                    current_block = 13236682061463366761;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 3790561273297268167;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                47 => {
                                    current_block = 18315435834437996746;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 7381876040995060082;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 14575275458752860498;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 8323211640247371006;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 11091651100383110420;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                329 => {
                                    current_block = 15385998965109782319;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                330 => {
                                    current_block = 15395163328284184564;
                                    match current_block {
                                        15395163328284184564 => {
                                            v.floating = (v.floating as libc::c_ulonglong
                                                >> r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        13236682061463366761 => {
                                            v.floating -= r.floating;
                                        }
                                        3790561273297268167 => {
                                            v.floating *= r.floating;
                                        }
                                        18315435834437996746 => {
                                            if r.floating == 0.0f64 {
                                                exerror(
                                                    b"floating divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating /= r.floating;
                                            }
                                        }
                                        7381876040995060082 => {
                                            r.integer = r.floating as libc::c_longlong;
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"floating 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.floating = (v.floating as libc::c_longlong
                                                    % r.integer)
                                                    as libc::c_double;
                                            }
                                        }
                                        14575275458752860498 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                & r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        8323211640247371006 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                | r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        11091651100383110420 => {
                                            v.floating = (v.floating as libc::c_longlong
                                                ^ r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        15385998965109782319 => {
                                            v.floating = ((v.floating as libc::c_longlong)
                                                << r.floating as libc::c_longlong)
                                                as libc::c_double;
                                        }
                                        _ => {
                                            v.floating += r.floating;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                            11515888952988033665 => match (*expr_0).subop {
                                43 => {
                                    current_block = 13034768034686864816;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                45 => {
                                    current_block = 3632332525568699835;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 3119644467204629641;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                47 => {
                                    current_block = 4155040754965348757;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 16153603300287884505;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 12154886366113648751;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 14747310749158537291;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 8869653012208538513;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                329 => {
                                    current_block = 14539215783803676789;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                330 => {
                                    current_block = 17916663512757813856;
                                    match current_block {
                                        17916663512757813856 => {
                                            v.integer = (v.integer as libc::c_ulonglong
                                                >> r.integer)
                                                as libc::c_longlong;
                                        }
                                        3632332525568699835 => {
                                            v.integer -= r.integer;
                                        }
                                        3119644467204629641 => {
                                            v.integer *= r.integer;
                                        }
                                        4155040754965348757 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer divide by 0\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer /= r.integer;
                                            }
                                        }
                                        16153603300287884505 => {
                                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                                exerror(
                                                    b"integer 0 modulus\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                v.integer %= r.integer;
                                            }
                                        }
                                        12154886366113648751 => {
                                            v.integer &= r.integer;
                                        }
                                        14747310749158537291 => {
                                            v.integer |= r.integer;
                                        }
                                        8869653012208538513 => {
                                            v.integer ^= r.integer;
                                        }
                                        14539215783803676789 => {
                                            v.integer <<= r.integer;
                                        }
                                        _ => {
                                            v.integer += r.integer;
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                            _ => match (*expr_0).subop {
                                43 => {
                                    current_block = 3636433389734256616;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                124 => {
                                    current_block = 6388602163840042976;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                38 => {
                                    current_block = 516796709286287865;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                94 => {
                                    current_block = 14728029199942657210;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                37 => {
                                    current_block = 11002239204738513371;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                42 => {
                                    current_block = 8050654829899665499;
                                    match current_block {
                                        8050654829899665499 => {
                                            v.string = str_mpy(ex, v.string, r.string);
                                        }
                                        6388602163840042976 => {
                                            v.string = str_ior(ex, v.string, r.string);
                                        }
                                        516796709286287865 => {
                                            v.string = str_and(ex, v.string, r.string);
                                        }
                                        14728029199942657210 => {
                                            v.string = str_xor(ex, v.string, r.string);
                                        }
                                        11002239204738513371 => {
                                            v.string = str_mod(ex, v.string, r.string);
                                        }
                                        _ => {
                                            v.string = str_add(ex, v.string, r.string);
                                        }
                                    }
                                    current_block = 7499465236084769340;
                                }
                                _ => {
                                    current_block = 2805727839212370108;
                                }
                            },
                        }
                    }
                    _ => {
                        current_block = 2805727839212370108;
                    }
                }
            } else {
                if (*x).op == 275 as libc::c_int {
                    getdyn(ex, x, env, &mut assoc);
                } else {
                    assoc = 0 as *mut Exassoc_t;
                }
                current_block = 7499465236084769340;
            }
            match current_block {
                2805727839212370108 => {}
                _ => {
                    r = v;
                    current_block = 14477994319607004593;
                }
            }
        }
        59 | 44 => {
            v = eval(ex, x, env);
            loop {
                expr_0 = (*expr_0).data.operand.right;
                if !(!expr_0.is_null()
                    && ((*expr_0).op == ';' as i32 || (*expr_0).op == ',' as i32))
                {
                    break;
                }
                v = eval(ex, (*expr_0).data.operand.left, env);
                if (*ex).loopcount != 0 {
                    return v;
                }
            }
            return if !expr_0.is_null() {
                eval(ex, expr_0, env)
            } else {
                v
            };
        }
        63 => {
            v = eval(ex, x, env);
            return if v.integer != 0 {
                eval(ex, (*(*expr_0).data.operand.right).data.operand.left, env)
            } else {
                eval(ex, (*(*expr_0).data.operand.right).data.operand.right, env)
            };
        }
        324 => {
            v = eval(ex, x, env);
            return if v.integer != 0 {
                eval(ex, (*expr_0).data.operand.right, env)
            } else {
                v
            };
        }
        323 => {
            v = eval(ex, x, env);
            return if v.integer != 0 {
                v
            } else {
                eval(ex, (*expr_0).data.operand.right, env)
            };
        }
        _ => {
            v = eval(ex, x, env);
            x = (*expr_0).data.operand.right;
            if !x.is_null() {
                r = eval(ex, x, env);
                if !((*x).type_0 > 258 as libc::c_int) && (*expr_0).binary != 0 {
                    tmp = *(*expr_0).data.operand.left;
                    tmp.data.constant.value = v;
                    rtmp = *x;
                    rtmp.data.constant.value = r;
                    if ((*(*ex).disc).binaryf).expect("non-null function pointer")(
                        ex,
                        &mut tmp,
                        expr_0,
                        &mut rtmp,
                        0 as libc::c_int,
                        (*ex).disc,
                    ) == 0
                    {
                        return tmp.data.constant.value;
                    }
                }
            }
            match (*(*expr_0).data.operand.left).type_0 {
                262 => {
                    match (*expr_0).op {
                        308 => {
                            v.integer = v.floating as libc::c_longlong;
                            return v;
                        }
                        309 => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if (*(*expr_0).data.operand.left).op != 275 as libc::c_int
                                && (*(*expr_0).data.operand.left).op != 283 as libc::c_int
                            {
                                tmp.data.constant.value.string = exprintf(
                                    (*ex).ve,
                                    b"%g\0" as *const u8 as *const libc::c_char,
                                    v.floating,
                                );
                            } else if ((*(*ex).disc).convertf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                263 as libc::c_int,
                                if !((*expr_0).data.operand.right).is_null() {
                                    (*(*expr_0).data.operand.right).data.variable.symbol
                                } else {
                                    0 as *mut Exid_t
                                },
                                0 as libc::c_int,
                                (*ex).disc,
                            ) != 0
                            {
                                tmp.data.constant.value.string = exprintf(
                                    (*ex).ve,
                                    b"%g\0" as *const u8 as *const libc::c_char,
                                    v.floating,
                                );
                            }
                            tmp.type_0 = 263 as libc::c_int;
                            return tmp.data.constant.value;
                        }
                        315 => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if ((*(*ex).disc).convertf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                (*expr_0).type_0,
                                if !((*expr_0).data.operand.right).is_null() {
                                    (*(*expr_0).data.operand.right).data.variable.symbol
                                } else {
                                    0 as *mut Exid_t
                                },
                                0 as libc::c_int,
                                (*ex).disc,
                            ) != 0
                            {
                                exerror(
                                    b"%s: cannot convert floating value to external\0" as *const u8
                                        as *const libc::c_char,
                                    ((*tmp.data.variable.symbol).name).as_mut_ptr(),
                                );
                            }
                            tmp.type_0 = (*expr_0).type_0;
                            return tmp.data.constant.value;
                        }
                        33 => {
                            v.floating = (v.floating as libc::c_longlong == 0) as libc::c_int
                                as libc::c_double;
                            return v;
                        }
                        126 => {
                            v.floating = !(v.floating as libc::c_longlong) as libc::c_double;
                            return v;
                        }
                        45 => {
                            if !x.is_null() {
                                v.floating -= r.floating;
                            } else {
                                v.floating = -v.floating;
                            }
                            return v;
                        }
                        43 => {
                            v.floating += r.floating;
                            return v;
                        }
                        38 => {
                            v.floating = (v.floating as libc::c_longlong
                                & r.floating as libc::c_longlong)
                                as libc::c_double;
                            return v;
                        }
                        124 => {
                            v.floating = (v.floating as libc::c_longlong
                                | r.floating as libc::c_longlong)
                                as libc::c_double;
                            return v;
                        }
                        94 => {
                            v.floating = (v.floating as libc::c_longlong
                                ^ r.floating as libc::c_longlong)
                                as libc::c_double;
                            return v;
                        }
                        42 => {
                            v.floating *= r.floating;
                            return v;
                        }
                        47 => {
                            if r.floating == 0.0f64 {
                                exerror(
                                    b"floating divide by 0\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                v.floating /= r.floating;
                            }
                            return v;
                        }
                        37 => {
                            r.integer = r.floating as libc::c_longlong;
                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                exerror(
                                    b"floating 0 modulus\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                v.floating =
                                    (v.floating as libc::c_longlong % r.integer) as libc::c_double;
                            }
                            return v;
                        }
                        60 => {
                            v.integer =
                                (v.floating < r.floating) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        327 => {
                            v.integer =
                                (v.floating <= r.floating) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        325 => {
                            v.integer =
                                (v.floating == r.floating) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        326 => {
                            v.integer =
                                (v.floating != r.floating) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        328 => {
                            v.integer =
                                (v.floating >= r.floating) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        62 => {
                            v.integer =
                                (v.floating > r.floating) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        329 => {
                            v.integer = ((v.floating as libc::c_ulonglong)
                                << r.floating as libc::c_longlong)
                                as libc::c_longlong;
                            return v;
                        }
                        330 => {
                            v.integer = (v.floating as libc::c_ulonglong
                                >> r.floating as libc::c_longlong)
                                as libc::c_longlong;
                            return v;
                        }
                        _ => {}
                    }
                    current_block = 2805727839212370108;
                }
                260 => {
                    match (*expr_0).op {
                        60 => {
                            v.integer = ((v.integer as libc::c_ulonglong)
                                < r.integer as libc::c_ulonglong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        327 => {
                            v.integer = (v.integer as libc::c_ulonglong
                                <= r.integer as libc::c_ulonglong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        328 => {
                            v.integer = (v.integer as libc::c_ulonglong
                                >= r.integer as libc::c_ulonglong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        62 => {
                            v.integer = (v.integer as libc::c_ulonglong
                                > r.integer as libc::c_ulonglong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        _ => {}
                    }
                    current_block = 8615874688957722140;
                }
                259 => {
                    current_block = 8615874688957722140;
                }
                263 => {
                    match (*expr_0).op {
                        312 => {
                            v.integer = (*v.string as libc::c_int != 0 as libc::c_int)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        313 => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if ((*(*ex).disc).convertf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                262 as libc::c_int,
                                if !((*expr_0).data.operand.right).is_null() {
                                    (*(*expr_0).data.operand.right).data.variable.symbol
                                } else {
                                    0 as *mut Exid_t
                                },
                                0 as libc::c_int,
                                (*ex).disc,
                            ) != 0
                            {
                                tmp.data.constant.value.floating = strtod(v.string, &mut e);
                                if *e != 0 {
                                    tmp.data.constant.value.floating = (*v.string as libc::c_int
                                        != 0 as libc::c_int)
                                        as libc::c_int
                                        as libc::c_double;
                                }
                            }
                            tmp.type_0 = 262 as libc::c_int;
                            return tmp.data.constant.value;
                        }
                        314 => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if ((*(*ex).disc).convertf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                259 as libc::c_int,
                                if !((*expr_0).data.operand.right).is_null() {
                                    (*(*expr_0).data.operand.right).data.variable.symbol
                                } else {
                                    0 as *mut Exid_t
                                },
                                0 as libc::c_int,
                                (*ex).disc,
                            ) != 0
                            {
                                if !(v.string).is_null() {
                                    tmp.data.constant.value.integer =
                                        strtoll(v.string, &mut e, 0 as libc::c_int);
                                    if *e != 0 {
                                        tmp.data.constant.value.integer = (*v.string as libc::c_int
                                            != 0 as libc::c_int)
                                            as libc::c_int
                                            as libc::c_longlong;
                                    }
                                } else {
                                    tmp.data.constant.value.integer =
                                        0 as libc::c_int as libc::c_longlong;
                                }
                            }
                            tmp.type_0 = 259 as libc::c_int;
                            return tmp.data.constant.value;
                        }
                        317 => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if ((*(*ex).disc).convertf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                (*expr_0).type_0,
                                if !((*expr_0).data.operand.right).is_null() {
                                    (*(*expr_0).data.operand.right).data.variable.symbol
                                } else {
                                    0 as *mut Exid_t
                                },
                                0 as libc::c_int,
                                (*ex).disc,
                            ) != 0
                            {
                                exerror(
                                    b"%s: cannot convert string value to external\0" as *const u8
                                        as *const libc::c_char,
                                    ((*tmp.data.variable.symbol).name).as_mut_ptr(),
                                );
                            }
                            tmp.type_0 = (*expr_0).type_0;
                            return tmp.data.constant.value;
                        }
                        325 | 326 => {
                            v.integer = ((if !(v.string).is_null() && !(r.string).is_null() {
                                (if (*(*ex).disc).version
                                    >= 19981111 as libc::c_long as libc::c_ulong
                                    && ((*(*ex).disc).matchf).is_some()
                                {
                                    ((*(*ex).disc).matchf).expect("non-null function pointer")(
                                        ex,
                                        (*expr_0).data.operand.left,
                                        v.string,
                                        (*expr_0).data.operand.right,
                                        r.string,
                                        env,
                                        (*ex).disc,
                                    )
                                } else {
                                    strmatch(v.string, r.string)
                                })
                            } else {
                                (v.string == r.string) as libc::c_int
                            }) == ((*expr_0).op == 325 as libc::c_int) as libc::c_int)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        43 => {
                            v.string = str_add(ex, v.string, r.string);
                            return v;
                        }
                        124 => {
                            v.string = str_ior(ex, v.string, r.string);
                            return v;
                        }
                        38 => {
                            v.string = str_and(ex, v.string, r.string);
                            return v;
                        }
                        94 => {
                            v.string = str_xor(ex, v.string, r.string);
                            return v;
                        }
                        37 => {
                            v.string = str_mod(ex, v.string, r.string);
                            return v;
                        }
                        42 => {
                            v.string = str_mpy(ex, v.string, r.string);
                            return v;
                        }
                        _ => {}
                    }
                    v.integer = strcoll(v.string, r.string) as libc::c_longlong;
                    match (*expr_0).op {
                        60 => {
                            v.integer = (v.integer < 0 as libc::c_int as libc::c_longlong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        327 => {
                            v.integer = (v.integer <= 0 as libc::c_int as libc::c_longlong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        328 => {
                            v.integer = (v.integer >= 0 as libc::c_int as libc::c_longlong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        62 => {
                            v.integer = (v.integer > 0 as libc::c_int as libc::c_longlong)
                                as libc::c_int
                                as libc::c_longlong;
                            return v;
                        }
                        _ => {}
                    }
                    current_block = 2805727839212370108;
                }
                _ => {
                    match (*expr_0).op {
                        318 => {
                            xConvert(ex, expr_0, 262 as libc::c_int, v, &mut tmp);
                            return tmp.data.constant.value;
                        }
                        319 => {
                            xConvert(ex, expr_0, 259 as libc::c_int, v, &mut tmp);
                            return tmp.data.constant.value;
                        }
                        320 => {
                            xConvert(ex, expr_0, 263 as libc::c_int, v, &mut tmp);
                            return tmp.data.constant.value;
                        }
                        321 => {
                            xConvert(ex, expr_0, (*expr_0).type_0, v, &mut tmp);
                            return tmp.data.constant.value;
                        }
                        322 => {
                            xPrint(ex, expr_0, v, &mut tmp);
                            return tmp.data.constant.value;
                        }
                        _ => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if !x.is_null() {
                                rtmp = *x;
                                rtmp.data.constant.value = r;
                                rp = &mut rtmp;
                            } else {
                                rp = 0 as *mut Exnode_t;
                            }
                            if ((*(*ex).disc).binaryf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                expr_0,
                                rp,
                                0 as libc::c_int,
                                (*ex).disc,
                            ) == 0
                            {
                                return tmp.data.constant.value;
                            }
                        }
                    }
                    current_block = 8615874688957722140;
                }
            }
            match current_block {
                2805727839212370108 => {}
                _ => {
                    match (*expr_0).op {
                        310 => {
                            if (*expr_0).type_0 == 260 as libc::c_int {
                                v.floating = v.integer as libc::c_ulonglong as libc::c_double;
                            } else {
                                v.floating = v.integer as libc::c_double;
                            }
                            return v;
                        }
                        311 => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if (*(*expr_0).data.operand.left).op != 275 as libc::c_int
                                && (*(*expr_0).data.operand.left).op != 283 as libc::c_int
                            {
                                let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
                                if (*(*expr_0).data.operand.left).type_0 == 260 as libc::c_int {
                                    str = exprintf(
                                        (*ex).ve,
                                        b"%llu\0" as *const u8 as *const libc::c_char,
                                        v.integer as libc::c_ulonglong,
                                    );
                                } else {
                                    str = exprintf(
                                        (*ex).ve,
                                        b"%lld\0" as *const u8 as *const libc::c_char,
                                        v.integer,
                                    );
                                }
                                tmp.data.constant.value.string = str;
                            } else if ((*(*ex).disc).convertf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                263 as libc::c_int,
                                if !((*expr_0).data.operand.right).is_null() {
                                    (*(*expr_0).data.operand.right).data.variable.symbol
                                } else {
                                    0 as *mut Exid_t
                                },
                                0 as libc::c_int,
                                (*ex).disc,
                            ) != 0
                            {
                                let mut str_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                if (*(*expr_0).data.operand.left).type_0 == 260 as libc::c_int {
                                    str_0 = exprintf(
                                        (*ex).ve,
                                        b"%llu\0" as *const u8 as *const libc::c_char,
                                        v.integer as libc::c_ulonglong,
                                    );
                                } else {
                                    str_0 = exprintf(
                                        (*ex).ve,
                                        b"%lld\0" as *const u8 as *const libc::c_char,
                                        v.integer,
                                    );
                                }
                                tmp.data.constant.value.string = str_0;
                            }
                            tmp.type_0 = 263 as libc::c_int;
                            return tmp.data.constant.value;
                        }
                        316 => {
                            tmp = *(*expr_0).data.operand.left;
                            tmp.data.constant.value = v;
                            if ((*(*ex).disc).convertf).expect("non-null function pointer")(
                                ex,
                                &mut tmp,
                                (*expr_0).type_0,
                                if !((*expr_0).data.operand.right).is_null() {
                                    (*(*expr_0).data.operand.right).data.variable.symbol
                                } else {
                                    0 as *mut Exid_t
                                },
                                0 as libc::c_int,
                                (*ex).disc,
                            ) != 0
                            {
                                exerror(
                                    b"%s: cannot convert integer value to external\0" as *const u8
                                        as *const libc::c_char,
                                    ((*tmp.data.variable.symbol).name).as_mut_ptr(),
                                );
                            }
                            tmp.type_0 = (*expr_0).type_0;
                            return tmp.data.constant.value;
                        }
                        33 => {
                            v.integer = (v.integer == 0) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        126 => {
                            v.integer = !v.integer;
                            return v;
                        }
                        45 => {
                            if !x.is_null() {
                                v.integer -= r.integer;
                            } else {
                                v.integer = -v.integer;
                            }
                            return v;
                        }
                        43 => {
                            v.integer += r.integer;
                            return v;
                        }
                        38 => {
                            v.integer &= r.integer;
                            return v;
                        }
                        124 => {
                            v.integer |= r.integer;
                            return v;
                        }
                        94 => {
                            v.integer ^= r.integer;
                            return v;
                        }
                        42 => {
                            v.integer *= r.integer;
                            return v;
                        }
                        47 => {
                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                exerror(
                                    b"integer divide by 0\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                v.integer /= r.integer;
                            }
                            return v;
                        }
                        37 => {
                            if r.integer == 0 as libc::c_int as libc::c_longlong {
                                exerror(b"integer 0 modulus\0" as *const u8 as *const libc::c_char);
                            } else {
                                v.integer %= r.integer;
                            }
                            return v;
                        }
                        325 => {
                            v.integer = (v.integer == r.integer) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        326 => {
                            v.integer = (v.integer != r.integer) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        329 => {
                            v.integer = v.integer << r.integer;
                            return v;
                        }
                        330 => {
                            v.integer =
                                (v.integer as libc::c_ulonglong >> r.integer) as libc::c_longlong;
                            return v;
                        }
                        60 => {
                            v.integer = (v.integer < r.integer) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        327 => {
                            v.integer = (v.integer <= r.integer) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        328 => {
                            v.integer = (v.integer >= r.integer) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        62 => {
                            v.integer = (v.integer > r.integer) as libc::c_int as libc::c_longlong;
                            return v;
                        }
                        _ => {}
                    }
                    current_block = 2805727839212370108;
                }
            }
        }
    }
    match current_block {
        15143230840993836487 => {
            if (*x).op == 275 as libc::c_int {
                r = getdyn(ex, x, env, &mut assoc);
            } else {
                if !((*x).data.variable.index).is_null() {
                    i = eval(ex, (*x).data.variable.index, env);
                } else {
                    i.integer = -(1 as libc::c_int) as libc::c_longlong;
                }
                if !((*x).data.variable.dyna).is_null() {
                    let mut locv: Extype_t = EX_STYPE {
                        expr: 0 as *mut Exnode_s,
                    };
                    locv = getdyn(ex, (*x).data.variable.dyna, env, &mut assoc);
                    (*(*(*x).data.variable.dyna).data.variable.dyna)
                        .data
                        .constant
                        .value = locv;
                }
                r = ((*(*ex).disc).getf).expect("non-null function pointer")(
                    ex,
                    x,
                    (*x).data.variable.symbol,
                    (*x).data.variable.reference,
                    env,
                    i.integer as libc::c_int,
                    (*ex).disc,
                );
            }
            v = r;
            match (*x).type_0 {
                262 => {
                    current_block = 7543625045781985282;
                    match current_block {
                        6349310606458602318 => {
                            v.integer += n as libc::c_longlong;
                        }
                        _ => {
                            v.floating += n as libc::c_double;
                        }
                    }
                    current_block = 14477994319607004593;
                }
                259 | 260 => {
                    current_block = 6349310606458602318;
                    match current_block {
                        6349310606458602318 => {
                            v.integer += n as libc::c_longlong;
                        }
                        _ => {
                            v.floating += n as libc::c_double;
                        }
                    }
                    current_block = 14477994319607004593;
                }
                _ => {
                    current_block = 2805727839212370108;
                }
            }
        }
        _ => {}
    }
    match current_block {
        2805727839212370108 => {
            if (*expr_0).binary != 0 {
                exerror(
                    b"operator %s %s %s not implemented\0" as *const u8 as *const libc::c_char,
                    lexname((*(*expr_0).data.operand.left).type_0, -(1 as libc::c_int)),
                    lexname((*expr_0).op, (*expr_0).subop),
                    if !((*expr_0).data.operand.right).is_null() {
                        lexname((*(*expr_0).data.operand.right).type_0, -(1 as libc::c_int))
                    } else {
                        b"UNARY\0" as *const u8 as *const libc::c_char
                    },
                );
            } else {
                exerror(
                    b"operator %s %s not implemented\0" as *const u8 as *const libc::c_char,
                    lexname((*expr_0).op, (*expr_0).subop),
                    lexname((*(*expr_0).data.operand.left).type_0, -(1 as libc::c_int)),
                );
            }
            return exzero((*expr_0).type_0);
        }
        _ => {
            if (*x).op == 275 as libc::c_int {
                if (*x).type_0 == 263 as libc::c_int {
                    v.string = vmstrdup((*ex).vm, v.string);
                    e = if !assoc.is_null() {
                        (*assoc).value.string
                    } else {
                        (*(*(*x).data.variable.symbol).value)
                            .data
                            .constant
                            .value
                            .string
                    };
                    if !e.is_null() {
                        vmfree((*ex).vm, e as *mut libc::c_void);
                    }
                }
                if !assoc.is_null() {
                    (*assoc).value = v;
                } else {
                    (*(*(*x).data.variable.symbol).value).data.constant.value = v;
                }
            } else {
                if !((*x).data.variable.index).is_null() {
                    i = eval(ex, (*x).data.variable.index, env);
                } else {
                    i.integer = -(1 as libc::c_int) as libc::c_longlong;
                }
                if !((*x).data.variable.dyna).is_null() {
                    let mut locv_0: Extype_t = EX_STYPE {
                        expr: 0 as *mut Exnode_s,
                    };
                    locv_0 = getdyn(ex, (*x).data.variable.dyna, env, &mut assoc);
                    (*(*(*x).data.variable.dyna).data.variable.dyna)
                        .data
                        .constant
                        .value = locv_0;
                }
                if ((*(*ex).disc).setf).expect("non-null function pointer")(
                    ex,
                    x,
                    (*x).data.variable.symbol,
                    (*x).data.variable.reference,
                    env,
                    i.integer as libc::c_int,
                    v,
                    (*ex).disc,
                ) < 0 as libc::c_int
                {
                    exerror(
                        b"%s: cannot set value\0" as *const u8 as *const libc::c_char,
                        ((*(*x).data.variable.symbol).name).as_mut_ptr(),
                    );
                }
            }
            if (*expr_0).subop == 290 as libc::c_int {
                r = v;
            }
            return r;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn exeval(
    mut ex: *mut Expr_t,
    mut expr_0: *mut Exnode_t,
    mut env: *mut libc::c_void,
) -> Extype_t {
    let mut v: Extype_t = EX_STYPE {
        expr: 0 as *mut Exnode_s,
    };
    if ((*expr_0).compiled.integer).is_some() {
        match (*expr_0).type_0 {
            262 => {
                v.floating = ((*expr_0).compiled.floating).expect("non-null function pointer")(
                    (*(*ex).disc).data,
                );
            }
            263 => {
                v.string = ((*expr_0).compiled.string).expect("non-null function pointer")(
                    (*(*ex).disc).data,
                );
            }
            _ => {
                v.integer = ((*expr_0).compiled.integer).expect("non-null function pointer")(
                    (*(*ex).disc).data,
                );
            }
        }
    } else {
        v = eval(ex, expr_0, env);
        if (*ex).loopcount > 0 as libc::c_int {
            (*ex).loopcount = 0 as libc::c_int;
            if (*ex).loopop == 296 as libc::c_int {
                return (*ex).loopret;
            }
        }
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn exstring(
    mut ex: *mut Expr_t,
    mut s: *mut libc::c_char,
) -> *mut libc::c_char {
    return vmstrdup((*ex).ve, s);
}
#[no_mangle]
pub unsafe extern "C" fn exstralloc(mut ex: *mut Expr_t, mut sz: size_t) -> *mut libc::c_void {
    return vmalloc((*ex).ve, sz);
}
