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
#![feature(c_variadic, label_break_value, register_tool)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut sfstderr: *mut Sfio_t;
    fn sfopen(_: *mut Sfio_t, _: *const libc::c_char, _: *const libc::c_char) -> *mut Sfio_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfprintf(_: *mut Sfio_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn _sffilbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn pathfind(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut libc::c_char,
        _: size_t,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exerror(_: *const libc::c_char, _: ...);
    fn vmalloc(vm: *mut Vmalloc_t, size: size_t) -> *mut libc::c_void;
    fn exnospace() -> *mut libc::c_char;
    static mut _err_info: Error_info_t;
    static mut Dtset: *mut Dtmethod_t;
    fn vmfree(vm: *mut Vmalloc_t, data: *mut libc::c_void);
    fn vmclear(_: *mut Vmalloc_t) -> libc::c_int;
    fn vmclose(_: *mut Vmalloc_t) -> libc::c_int;
    fn exstash(_: *mut Sfio_t, _: *mut Vmalloc_t) -> *mut libc::c_char;
    fn vmstrdup(_: *mut Vmalloc_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn exopname(_: libc::c_int) -> *mut libc::c_char;
    fn exeval(_: *mut Expr_t, _: *mut Exnode_t, _: *mut libc::c_void) -> Extype_t;
    fn exzero(_: libc::c_int) -> Extype_t;
    static mut Dtoset: *mut Dtmethod_t;
    fn _err_msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn exwarn(_: *const libc::c_char, _: ...);
    fn extoken_fn(_: *mut Expr_t) -> libc::c_int;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn dtview(_: *mut Dt_t, _: *mut Dt_t) -> *mut Dt_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
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
pub type __ssize_t = libc::c_long;
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
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
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
    pub compiled: C2RustUnnamed_13,
    pub data: Exdata_t,
    pub subop: libc::c_int,
}
pub type Exdata_t = Exdata_u;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Exdata_u {
    pub constant: C2RustUnnamed_12,
    pub operand: C2RustUnnamed_11,
    pub select: C2RustUnnamed_10,
    pub variable: C2RustUnnamed_9,
    pub next: *mut Exnode_t,
    pub value: Extype_t,
    pub call: C2RustUnnamed_8,
    pub generate: C2RustUnnamed_7,
    pub split: C2RustUnnamed_6,
    pub print: C2RustUnnamed_5,
    pub string: C2RustUnnamed_4,
    pub procedure: C2RustUnnamed_1,
    pub scan: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub descriptor: *mut Exnode_t,
    pub format: *mut Exnode_t,
    pub args: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub type Dtlink_t = _dtlink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _hash: libc::c_uint,
    pub _left: *mut Dtlink_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub base: *mut Exnode_t,
    pub pat: *mut Exnode_t,
    pub repl: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub descriptor: *mut Exnode_t,
    pub args: *mut Print_t,
}
pub type Print_t = Print_s;
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
pub struct C2RustUnnamed_6 {
    pub array: *mut Exid_t,
    pub string: *mut Exnode_t,
    pub seps: *mut Exnode_t,
}
pub type Exid_t = Exid_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub array: *mut Exnode_t,
    pub index: *mut Exid_t,
    pub statement: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub procedure: *mut Exid_t,
    pub args: *mut Exnode_t,
}
pub type Extype_t = EX_STYPE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub statement: *mut Exnode_t,
    pub next: *mut Exnode_t,
    pub constant: *mut *mut Extype_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub left: *mut Exnode_t,
    pub right: *mut Exnode_t,
    pub last: *mut Exnode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub value: Extype_t,
    pub reference: *mut Exid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub floating: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_double>,
    pub integer: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_longlong>,
    pub string: Option<unsafe extern "C" fn(*mut *mut libc::c_char) -> *mut libc::c_char>,
}
pub type yytype_int16 = libc::c_short;
pub type yytype_uint8 = libc::c_uchar;
pub type Exinput_t = Exinput_s;
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
pub type Exdisc_t = Exdisc_s;
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
pub type Exerror_f = Option<
    unsafe extern "C" fn(
        *mut Expr_t,
        *mut Exdisc_t,
        libc::c_int,
        *const libc::c_char,
        ...
    ) -> libc::c_int,
>;
pub type Vmalloc_t = _vmalloc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vmalloc_s {
    pub allocated: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
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
pub type yytype_uint16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: EX_STYPE,
}
pub type Error_info_t = Error_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Error_info_s {
    pub errors: libc::c_int,
    pub indent: libc::c_int,
    pub line: libc::c_int,
    pub warnings: libc::c_int,
    pub trace: libc::c_int,
    pub file: *mut libc::c_char,
    pub id: *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn exinit() {
    memset(
        &mut expr as *mut Exstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Exstate_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub static mut expr: Exstate_t = Exstate_t {
    id: 0 as *const Exid_t as *mut Exid_t,
    declare: 0,
    lastref: 0 as *const Exref_t as *mut Exref_t,
    nolabel: 0,
    null: Exinput_t {
        next: 0 as *const Exinput_s as *mut Exinput_s,
        close: 0,
        file: 0 as *const libc::c_char as *mut libc::c_char,
        fp: 0 as *const Sfio_t as *mut Sfio_t,
        line: 0,
        nesting: 0,
        peek: 0,
        unit: 0,
        pushback: 0 as *const libc::c_char as *mut libc::c_char,
        bp: 0 as *const libc::c_char as *mut libc::c_char,
        pp: 0 as *const libc::c_char as *mut libc::c_char,
        sp: 0 as *const libc::c_char as *mut libc::c_char,
    },
    program: 0 as *const Expr_t as *mut Expr_t,
    procedure: 0 as *const Exnode_t as *mut Exnode_t,
    refs: 0 as *const Exref_t as *mut Exref_t,
    assigned: 0,
    instatic: 0,
    statics: 0,
    swstate: 0 as *const Switch_t as *mut Switch_t,
    nullstring: [0; 1],
};
#[no_mangle]
pub unsafe extern "C" fn excast(
    mut p: *mut Expr_t,
    mut x: *mut Exnode_t,
    mut type_0: libc::c_int,
    mut xref: *mut Exnode_t,
    mut arg: libc::c_int,
) -> *mut Exnode_t {
    let mut t2t: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    if !x.is_null() && (*x).type_0 != type_0 && type_0 != 0 && type_0 != 264 as libc::c_int {
        if (*x).type_0 == 0 {
            (*x).type_0 = type_0;
            return x;
        }
        t2t = typecast[(if (*x).type_0 >= 259 as libc::c_int && (*x).type_0 <= 263 as libc::c_int {
            (*x).type_0 - 259 as libc::c_int + 1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as usize][(if type_0 >= 259 as libc::c_int && type_0 <= 263 as libc::c_int {
            type_0 - 259 as libc::c_int + 1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as usize];
        if t2t == 0 {
            return x;
        }
        if t2t >= 315 as libc::c_int && ((*(*p).disc).convertf).is_none() {
            exerror(
                b"cannot convert %s to %s\0" as *const u8 as *const libc::c_char,
                extypename(p, (*x).type_0),
                extypename(p, type_0),
            );
        }
        if (*x).op != 271 as libc::c_int {
            let mut sym: *mut Exid_t = if !xref.is_null() {
                (*xref).data.variable.symbol
            } else {
                0 as *mut Exid_t
            };
            if t2t >= 315 as libc::c_int {
                let mut a: libc::c_int = if arg != 0 { arg } else { 1 as libc::c_int };
                if (Some(((*(*p).disc).convertf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    p, x, type_0, sym, a, (*p).disc
                ) < 0 as libc::c_int
                {
                    if !xref.is_null() {
                        if (*sym).lex == 279 as libc::c_int as libc::c_long && arg != 0 {
                            exerror(
                                b"%s: cannot use value of type %s as argument %d in function %s\0"
                                    as *const u8
                                    as *const libc::c_char,
                                ((*sym).name).as_mut_ptr(),
                                extypename(p, (*x).type_0),
                                arg,
                                ((*sym).name).as_mut_ptr(),
                            );
                        } else {
                            exerror(
                                b"%s: cannot convert %s to %s\0" as *const u8
                                    as *const libc::c_char,
                                ((*(*xref).data.variable.symbol).name).as_mut_ptr(),
                                extypename(p, (*x).type_0),
                                extypename(p, type_0),
                            );
                        }
                    } else {
                        exerror(
                            b"cannot convert %s to %s\0" as *const u8 as *const libc::c_char,
                            extypename(p, (*x).type_0),
                            extypename(p, type_0),
                        );
                    }
                }
            }
            x = exnewnode(p, t2t, 0 as libc::c_int, type_0, x, xref);
        } else {
            match t2t {
                315 | 316 | 317 | 318 | 319 | 320 | 321 => {
                    if !xref.is_null() && (*xref).op == 283 as libc::c_int {
                        if (Some(((*(*p).disc).convertf).expect("non-null function pointer")))
                            .expect("non-null function pointer")(
                            p,
                            x,
                            type_0,
                            (*xref).data.variable.symbol,
                            arg,
                            (*p).disc,
                        ) < 0 as libc::c_int
                        {
                            exerror(
                                b"%s: cannot cast constant %s to %s\0" as *const u8
                                    as *const libc::c_char,
                                ((*(*xref).data.variable.symbol).name).as_mut_ptr(),
                                extypename(p, (*x).type_0),
                                extypename(p, type_0),
                            );
                        }
                    } else if (Some(((*(*p).disc).convertf).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        p,
                        x,
                        type_0,
                        0 as *mut Exid_t,
                        arg,
                        (*p).disc,
                    ) < 0 as libc::c_int
                    {
                        exerror(
                            b"cannot cast constant %s to %s\0" as *const u8 as *const libc::c_char,
                            extypename(p, (*x).type_0),
                            extypename(p, type_0),
                        );
                    }
                }
                308 => {
                    (*x).data.constant.value.integer =
                        (*x).data.constant.value.floating as libc::c_longlong;
                }
                309 => {
                    let ref mut fresh0 = (*x).data.constant.value.string;
                    *fresh0 = exprintf(
                        (*p).vm,
                        b"%g\0" as *const u8 as *const libc::c_char,
                        (*x).data.constant.value.floating,
                    );
                }
                310 => {
                    (*x).data.constant.value.floating =
                        (*x).data.constant.value.integer as libc::c_double;
                }
                311 => {
                    let ref mut fresh1 = (*x).data.constant.value.string;
                    *fresh1 = exprintf(
                        (*p).vm,
                        b"%lld\0" as *const u8 as *const libc::c_char,
                        (*x).data.constant.value.integer,
                    );
                }
                313 => {
                    s = (*x).data.constant.value.string;
                    (*x).data.constant.value.integer = strtod(s, &mut e) as libc::c_longlong;
                    if *e != 0 {
                        (*x).data.constant.value.floating = (*s as libc::c_int != 0 as libc::c_int)
                            as libc::c_int
                            as libc::c_double;
                    }
                }
                314 => {
                    s = (*x).data.constant.value.string;
                    (*x).data.constant.value.integer = strtoll(s, &mut e, 0 as libc::c_int);
                    if *e != 0 {
                        (*x).data.constant.value.integer = (*s as libc::c_int != 0 as libc::c_int)
                            as libc::c_int
                            as libc::c_longlong;
                    }
                }
                _ => {
                    exerror(
                        b"internal error: %d: unknown cast op\0" as *const u8
                            as *const libc::c_char,
                        t2t,
                    );
                }
            }
        }
        (*x).type_0 = type_0;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn expush(
    mut p: *mut Expr_t,
    mut name: *const libc::c_char,
    mut line: libc::c_int,
    mut sp: *const libc::c_char,
    mut fp: *mut Sfio_t,
) -> libc::c_int {
    let mut in_0: *mut Exinput_t = 0 as *mut Exinput_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    in_0 = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<Exinput_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Exinput_t
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<Exinput_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Exinput_t
    };
    if in_0.is_null() {
        exnospace();
        return -(1 as libc::c_int);
    }
    if ((*p).input).is_null() {
        let ref mut fresh2 = (*p).input;
        *fresh2 = &mut expr.null;
    }
    let ref mut fresh3 = (*in_0).sp;
    *fresh3 = sp as *mut libc::c_char;
    let ref mut fresh4 = (*in_0).bp;
    *fresh4 = *fresh3;
    if (*fresh4).is_null() {
        let ref mut fresh5 = (*in_0).fp;
        *fresh5 = fp;
        if !(*fresh5).is_null() {
            (*in_0).close = 0 as libc::c_int;
        } else if !name.is_null() {
            s = pathfind(
                name,
                (*(*p).disc).lib,
                (*(*p).disc).type_0,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            );
            if s.is_null() || {
                let ref mut fresh6 = (*in_0).fp;
                *fresh6 = sfopen(
                    0 as *mut Sfio_t,
                    s,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                (*fresh6).is_null()
            } {
                exerror(
                    b"%s: file not found\0" as *const u8 as *const libc::c_char,
                    name,
                );
                let ref mut fresh7 = (*in_0).sp;
                *fresh7 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                let ref mut fresh8 = (*in_0).bp;
                *fresh8 = *fresh7;
            } else {
                name = vmstrdup((*p).vm, s) as *const libc::c_char;
                (*in_0).close = 1 as libc::c_int;
            }
        }
    } else {
        let ref mut fresh9 = (*in_0).fp;
        *fresh9 = 0 as *mut Sfio_t;
    }
    let ref mut fresh10 = (*in_0).next;
    *fresh10 = (*p).input;
    if ((**fresh10).next).is_null() {
        (*p).errors = 0 as libc::c_int;
        if (*(*p).disc).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0 {
            if line >= 0 as libc::c_int {
                _err_info.line = line;
            }
        } else if _err_info.line == 0 {
            _err_info.line = 1 as libc::c_int;
        }
    } else if line >= 0 as libc::c_int {
        _err_info.line = line;
    }
    let ref mut fresh11 = (*p).linep;
    *fresh11 = ((*p).line).as_mut_ptr();
    (*p).linewrap = 0 as libc::c_int;
    (*p).eof = 0 as libc::c_int;
    let ref mut fresh12 = (*p).input;
    *fresh12 = in_0;
    let ref mut fresh13 = (*in_0).file;
    *fresh13 = _err_info.file;
    if line >= 0 as libc::c_int {
        _err_info.file = name as *mut libc::c_char;
    }
    (*in_0).line = _err_info.line;
    (*in_0).nesting = 0 as libc::c_int;
    (*in_0).unit = (name.is_null() && line == 0) as libc::c_int;
    let ref mut fresh14 = (*p).program;
    *fresh14 = expr.program;
    expr.program = p;
    return 0 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn extypename(
    mut p: *mut Expr_t,
    mut type_0: libc::c_int,
) -> *mut libc::c_char {
    if type_0 > 258 as libc::c_int {
        return typename[(if type_0 >= 259 as libc::c_int && type_0 <= 263 as libc::c_int {
            type_0 - 259 as libc::c_int + 1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as usize];
    }
    return ((*(*p).disc).typename).expect("non-null function pointer")(p, type_0);
}
static mut typename: [*mut libc::c_char; 6] = [
    b"external\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"integer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"unsigned\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"char\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"float\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn expop(mut p: *mut Expr_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut in_0: *mut Exinput_t = 0 as *mut Exinput_t;
    in_0 = (*p).input;
    if in_0.is_null() || ((*in_0).next).is_null() || (*in_0).unit != 0 {
        return -(1 as libc::c_int);
    }
    if (*in_0).nesting != 0 {
        exerror(b"unbalanced quote or nesting construct\0" as *const u8 as *const libc::c_char);
    }
    _err_info.file = (*in_0).file;
    if !((*(*in_0).next).next).is_null() {
        _err_info.line = (*in_0).line;
    } else {
        if (*p).errors != 0 && !((*in_0).fp).is_null() && (*p).linep != ((*p).line).as_mut_ptr() {
            loop {
                c = (if (*(*in_0).fp).next >= (*(*in_0).fp).endr {
                    _sffilbuf((*in_0).fp, 0 as libc::c_int)
                } else {
                    let ref mut fresh15 = (*(*in_0).fp).next;
                    let fresh16 = *fresh15;
                    *fresh15 = (*fresh15).offset(1);
                    *fresh16 as libc::c_int
                });
                if !(c != -(1 as libc::c_int)) {
                    break;
                }
                if !(c == '\n' as i32) {
                    continue;
                }
                _err_info.line += 1;
                break;
            }
        }
        if (*(*p).disc).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0 {
            _err_info.line = (*in_0).line;
        }
    }
    if !((*in_0).fp).is_null() && (*in_0).close != 0 {
        sfclose((*in_0).fp);
    }
    if !((*in_0).pushback).is_null() {
        free((*in_0).pushback as *mut libc::c_void);
    }
    let ref mut fresh17 = (*p).input;
    *fresh17 = (*in_0).next;
    free(in_0 as *mut libc::c_void);
    let ref mut fresh18 = (*p).linep;
    *fresh18 = ((*p).line).as_mut_ptr();
    (*p).linewrap = 0 as libc::c_int;
    if !((*p).program).is_null() {
        expr.program = (*p).program;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn excomp(
    mut p: *mut Expr_t,
    mut name: *const libc::c_char,
    mut line: libc::c_int,
    mut sp: *const libc::c_char,
    mut fp: *mut Sfio_t,
) -> libc::c_int {
    let mut v: *mut Exid_t = 0 as *mut Exid_t;
    let mut eof: libc::c_int = 0;
    let ref mut fresh19 = (*p).more;
    *fresh19 = 0 as *const libc::c_char;
    eof = (*p).eof;
    if sp.is_null() && fp.is_null() {
        if ((*p).input).is_null() {
            return -(1 as libc::c_int);
        }
    } else if expush(p, name, line, sp, fp) != 0 {
        return -(1 as libc::c_int);
    } else {
        (*(*p).input).unit = (line >= 0 as libc::c_int) as libc::c_int;
    }
    ex_parse();
    (*(*p).input).unit = 0 as libc::c_int;
    expop(p);
    (*p).eof = eof;
    if expr.statics != 0 {
        v = (Some(((*(*p).symbols).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            (*p).symbols,
            0 as *mut libc::c_void,
            0o200 as libc::c_int,
        ) as *mut Exid_t;
        while !v.is_null() {
            if (*v).isstatic != 0 {
                (Some(((*(*p).symbols).searchf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    (*p).symbols,
                    v as *mut libc::c_void,
                    0o2 as libc::c_int,
                );
                expr.statics -= 1;
                if expr.statics == 0 {
                    break;
                }
            }
            v = (Some(((*(*p).symbols).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                (*p).symbols,
                v as *mut libc::c_void,
                0o10 as libc::c_int,
            ) as *mut Exid_t;
        }
        expr.statics = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn exclose(mut p: *mut Expr_t, mut all: libc::c_int) {
    let mut in_0: *mut Exinput_t = 0 as *mut Exinput_t;
    if !p.is_null() {
        if all != 0 {
            let mut i: size_t = 0;
            i = 3 as libc::c_int as size_t;
            while i
                < (::std::mem::size_of::<[*mut Sfio_t; 10]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut Sfio_t>() as libc::c_ulong)
            {
                if !((*p).file[i as usize]).is_null() {
                    sfclose((*p).file[i as usize]);
                }
                i = i.wrapping_add(1);
            }
            if !((*p).vm).is_null() {
                vmclose((*p).vm);
            }
            if !((*p).ve).is_null() {
                vmclose((*p).ve);
            }
            if !((*p).symbols).is_null() {
                dtclose((*p).symbols);
            }
            if !((*p).tmp).is_null() {
                sfclose((*p).tmp);
            }
            loop {
                in_0 = (*p).input;
                if in_0.is_null() {
                    break;
                }
                if !((*in_0).pushback).is_null() {
                    free((*in_0).pushback as *mut libc::c_void);
                }
                if !((*in_0).fp).is_null() && (*in_0).close != 0 {
                    sfclose((*in_0).fp);
                }
                let ref mut fresh20 = (*p).input;
                *fresh20 = (*in_0).next;
                if !(*fresh20).is_null() {
                    free(in_0 as *mut libc::c_void);
                }
            }
            free(p as *mut libc::c_void);
        } else {
            vmclear((*p).ve);
            let ref mut fresh21 = (*p).main.value;
            *fresh21 = 0 as *mut Exnode_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn exnewnode(
    mut p: *mut Expr_t,
    mut op: libc::c_int,
    mut binary: libc::c_int,
    mut type_0: libc::c_int,
    mut left: *mut Exnode_t,
    mut right: *mut Exnode_t,
) -> *mut Exnode_t {
    let mut x: *mut Exnode_t = 0 as *mut Exnode_t;
    x = vmalloc((*p).vm, ::std::mem::size_of::<Exnode_t>() as libc::c_ulong) as *mut Exnode_t;
    memset(
        x as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Exnode_t>() as libc::c_ulong,
    );
    (*x).op = op;
    (*x).type_0 = type_0;
    (*x).binary = binary;
    (*x).local.number = 0 as libc::c_int as libc::c_longlong;
    let ref mut fresh22 = (*x).local.pointer;
    *fresh22 = 0 as *mut libc::c_char;
    let ref mut fresh23 = (*x).data.operand.left;
    *fresh23 = left;
    let ref mut fresh24 = (*x).data.operand.right;
    *fresh24 = right;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn exnoncast(mut x: *mut Exnode_t) -> *mut Exnode_t {
    while !x.is_null() && (*x).op >= 308 as libc::c_int && (*x).op <= 321 as libc::c_int {
        x = (*x).data.operand.left;
    }
    return x;
}
static mut typecast: [[libc::c_int; 6]; 6] = [
    [
        321 as libc::c_int,
        319 as libc::c_int,
        319 as libc::c_int,
        319 as libc::c_int,
        318 as libc::c_int,
        320 as libc::c_int,
    ],
    [
        316 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        310 as libc::c_int,
        311 as libc::c_int,
    ],
    [
        316 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        310 as libc::c_int,
        311 as libc::c_int,
    ],
    [
        316 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        310 as libc::c_int,
        311 as libc::c_int,
    ],
    [
        315 as libc::c_int,
        308 as libc::c_int,
        308 as libc::c_int,
        308 as libc::c_int,
        0 as libc::c_int,
        309 as libc::c_int,
    ],
    [
        317 as libc::c_int,
        314 as libc::c_int,
        314 as libc::c_int,
        314 as libc::c_int,
        313 as libc::c_int,
        0 as libc::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn exfreenode(mut p: *mut Expr_t, mut x: *mut Exnode_t) {
    let mut pr: *mut Print_t = 0 as *mut Print_t;
    let mut r: *mut Exref_t = 0 as *mut Exref_t;
    let mut pn: *mut Print_t = 0 as *mut Print_t;
    let mut rn: *mut Exref_t = 0 as *mut Exref_t;
    match (*x).op {
        269 => {
            if !((*x).data.call.args).is_null() {
                exfreenode(p, (*x).data.call.args);
            }
        }
        271 => {}
        274 => {
            if !((*x).data.select.next).is_null() {
                exfreenode(p, (*x).data.select.next);
            }
        }
        275 => {
            if !((*x).data.variable.index).is_null() {
                exfreenode(p, (*x).data.variable.index);
            }
            if !((*(*x).data.variable.symbol).local.pointer).is_null() {
                dtclose((*(*x).data.variable.symbol).local.pointer as *mut Dt_t);
                let ref mut fresh25 = (*(*x).data.variable.symbol).local.pointer;
                *fresh25 = 0 as *mut libc::c_char;
            }
        }
        35 => {
            if !((*(*x).data.variable.symbol).local.pointer).is_null() {
                dtclose((*(*x).data.variable.symbol).local.pointer as *mut Dt_t);
                let ref mut fresh26 = (*(*x).data.variable.symbol).local.pointer;
                *fresh26 = 0 as *mut libc::c_char;
            }
        }
        306 => {
            if !((*x).data.variable.index).is_null() {
                exfreenode(p, (*x).data.variable.index);
            }
            if !((*(*x).data.variable.symbol).local.pointer).is_null() {
                dtclose((*(*x).data.variable.symbol).local.pointer as *mut Dt_t);
                let ref mut fresh27 = (*(*x).data.variable.symbol).local.pointer;
                *fresh27 = 0 as *mut libc::c_char;
            }
        }
        281 | 282 => {
            if !((*x).data.generate.statement).is_null() {
                exfreenode(p, (*x).data.generate.statement);
            }
        }
        283 => {
            rn = (*x).data.variable.reference;
            loop {
                r = rn;
                if r.is_null() {
                    break;
                }
                rn = (*r).next;
                vmfree((*p).vm, r as *mut libc::c_void);
            }
            if !((*x).data.variable.index).is_null() {
                exfreenode(p, (*x).data.variable.index);
            }
        }
        280 | 302 | 303 => {
            exfreenode(p, (*x).data.string.base);
            exfreenode(p, (*x).data.string.pat);
            if !((*x).data.string.repl).is_null() {
                exfreenode(p, (*x).data.string.repl);
            }
        }
        305 | 298 => {
            if !((*x).data.split.seps).is_null() {
                exfreenode(p, (*x).data.split.seps);
            }
            exfreenode(p, (*x).data.split.string);
            if !((*(*x).data.split.array).local.pointer).is_null() {
                dtclose((*(*x).data.split.array).local.pointer as *mut Dt_t);
                let ref mut fresh28 = (*(*x).data.split.array).local.pointer;
                *fresh28 = 0 as *mut libc::c_char;
            }
        }
        291 => {
            exfreenode(p, (*x).data.operand.left);
        }
        292 | 299 => {
            if !((*x).data.print.descriptor).is_null() {
                exfreenode(p, (*x).data.print.descriptor);
            }
            pn = (*x).data.print.args;
            loop {
                pr = pn;
                if pr.is_null() {
                    break;
                }
                let mut i: size_t = 0;
                i = 0 as libc::c_int as size_t;
                while i
                    < (::std::mem::size_of::<[*mut Exnode_s; 3]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<*mut Exnode_s>() as libc::c_ulong)
                    && !((*pr).param[i as usize]).is_null()
                {
                    exfreenode(p, (*pr).param[i as usize]);
                    i = i.wrapping_add(1);
                }
                if !((*pr).arg).is_null() {
                    exfreenode(p, (*pr).arg);
                }
                pn = (*pr).next;
                vmfree((*p).vm, pr as *mut libc::c_void);
            }
        }
        _ => {
            if !((*x).data.operand.left).is_null() {
                exfreenode(p, (*x).data.operand.left);
            }
            if !((*x).data.operand.right).is_null() {
                exfreenode(p, (*x).data.operand.right);
            }
        }
    }
    vmfree((*p).vm, x as *mut libc::c_void);
}
unsafe extern "C" fn makeVar(
    mut prog: *mut Expr_t,
    mut s: *mut Exid_t,
    mut idx: *mut Exnode_t,
    mut dyna: *mut Exnode_t,
    mut refs: *mut Exref_t,
) -> *mut Exnode_t {
    let mut nn: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut kind: libc::c_int = 0;
    let mut sym: *mut Exid_t = 0 as *mut Exid_t;
    if !refs.is_null() {
        if !((*refs).next).is_null() {
            sym = (*(*refs).next).symbol;
            let ref mut fresh29 = (*(*refs).next).symbol;
            *fresh29 = (*refs).symbol;
        } else {
            sym = (*refs).symbol;
        }
        let ref mut fresh30 = (*refs).symbol;
        *fresh30 = s;
        let ref mut fresh31 = (*refs).index;
        *fresh31 = idx;
    } else {
        sym = s;
    }
    if (*sym).type_0 != 0 {
        kind = (*sym).type_0 as libc::c_int;
    } else {
        kind = 263 as libc::c_int;
    }
    nn = exnewnode(
        prog,
        283 as libc::c_int,
        0 as libc::c_int,
        kind,
        0 as *mut Exnode_t,
        0 as *mut Exnode_t,
    );
    let ref mut fresh32 = (*nn).data.variable.symbol;
    *fresh32 = sym;
    let ref mut fresh33 = (*nn).data.variable.reference;
    *fresh33 = refs;
    let ref mut fresh34 = (*nn).data.variable.index;
    *fresh34 = 0 as *mut Exnode_t;
    let ref mut fresh35 = (*nn).data.variable.dyna;
    *fresh35 = dyna;
    if ((*(*prog).disc).getf).is_none() {
        exerror(
            b"%s: identifier references not supported\0" as *const u8 as *const libc::c_char,
            ((*sym).name).as_mut_ptr(),
        );
    } else if ((*(*expr.program).disc).reff).is_some() {
        (Some(((*(*expr.program).disc).reff).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            prog,
            nn,
            (*nn).data.variable.symbol,
            refs,
            0 as *mut libc::c_char,
            -(1 as libc::c_int),
            (*prog).disc,
        );
    }
    return nn;
}
unsafe extern "C" fn preprint(mut args: *mut Exnode_t) -> *mut Print_t {
    let mut current_block: u64;
    let mut x: *mut Print_t = 0 as *mut Print_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut Print_t = 0 as *mut Print_t;
    let mut q: *mut Print_t = 0 as *mut Print_t;
    if args.is_null() || (*(*args).data.operand.left).type_0 != 263 as libc::c_int {
        exerror(b"format string argument expected\0" as *const u8 as *const libc::c_char);
    }
    if (*(*args).data.operand.left).op != 271 as libc::c_int {
        x = vmalloc(
            (*expr.program).vm,
            ::std::mem::size_of::<Print_t>() as libc::c_ulong,
        ) as *mut Print_t;
        memset(
            x as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<Print_t>() as libc::c_ulong,
        );
        let ref mut fresh36 = (*x).arg;
        *fresh36 = args;
        return x;
    }
    f = (*(*args).data.operand.left).data.constant.value.string;
    args = (*args).data.operand.right;
    s = f;
    while *s != 0 {
        if (*(*expr.program).tmp).next >= (*(*expr.program).tmp).endw {
            _sfflsbuf((*expr.program).tmp, *s as libc::c_uchar as libc::c_int);
        } else {
            let ref mut fresh37 = (*(*expr.program).tmp).next;
            let fresh38 = *fresh37;
            *fresh37 = (*fresh37).offset(1);
            *fresh38 = *s as libc::c_uchar;
        };
        if *s as libc::c_int == '%' as i32 {
            s = s.offset(1);
            if *s == 0 {
                exerror(
                    b"%s: trailing %% in format\0" as *const u8 as *const libc::c_char,
                    f,
                );
            }
            if *s as libc::c_int != '%' as i32 {
                break;
            }
            if !args.is_null() {
                if (*(*expr.program).tmp).next >= (*(*expr.program).tmp).endw {
                    _sfflsbuf(
                        (*expr.program).tmp,
                        '%' as i32 as libc::c_uchar as libc::c_int,
                    );
                } else {
                    let ref mut fresh39 = (*(*expr.program).tmp).next;
                    let fresh40 = *fresh39;
                    *fresh39 = (*fresh39).offset(1);
                    *fresh40 = '%' as i32 as libc::c_uchar;
                };
            }
        }
        s = s.offset(1);
    }
    x = 0 as *mut Print_t;
    's_133: loop {
        q = vmalloc(
            (*expr.program).vm,
            ::std::mem::size_of::<Print_t>() as libc::c_ulong,
        ) as *mut Print_t;
        if !x.is_null() {
            let ref mut fresh41 = (*x).next;
            *fresh41 = q;
        } else {
            p = q;
        }
        x = q;
        memset(
            x as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<Print_t>() as libc::c_ulong,
        );
        if *s != 0 {
            i = 0 as libc::c_int;
            t = 259 as libc::c_int;
            loop {
                let fresh42 = s;
                s = s.offset(1);
                c = *fresh42 as libc::c_int;
                match c {
                    0 => {
                        exerror(
                            b"unterminated %%... in format\0" as *const u8 as *const libc::c_char,
                        );
                        current_block = 11696355557053689112;
                        break 's_133;
                    }
                    42 => {
                        if i as libc::c_ulong
                            >= (::std::mem::size_of::<[*mut Exnode_s; 3]>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<*mut Exnode_s>() as libc::c_ulong
                                )
                        {
                            *s = 0 as libc::c_int as libc::c_char;
                            exerror(
                                b"format %s has too many * arguments\0" as *const u8
                                    as *const libc::c_char,
                                f,
                            );
                            current_block = 11696355557053689112;
                            break 's_133;
                        } else if args.is_null() {
                            *s = 0 as libc::c_int as libc::c_char;
                            exerror(
                                b"format %s * argument expected\0" as *const u8
                                    as *const libc::c_char,
                                f,
                            );
                            current_block = 11696355557053689112;
                            break 's_133;
                        } else {
                            let fresh43 = i;
                            i = i + 1;
                            let ref mut fresh44 = (*x).param[fresh43 as usize];
                            *fresh44 = (*args).data.operand.left;
                            args = (*args).data.operand.right;
                        }
                    }
                    40 => {
                        n = 1 as libc::c_int;
                        loop {
                            if (*(*expr.program).tmp).next >= (*(*expr.program).tmp).endw {
                                _sfflsbuf((*expr.program).tmp, c as libc::c_uchar as libc::c_int);
                            } else {
                                let ref mut fresh45 = (*(*expr.program).tmp).next;
                                let fresh46 = *fresh45;
                                *fresh45 = (*fresh45).offset(1);
                                *fresh46 = c as libc::c_uchar;
                            };
                            let fresh47 = s;
                            s = s.offset(1);
                            c = *fresh47 as libc::c_int;
                            match c {
                                0 => {
                                    s = s.offset(-1);
                                    break;
                                }
                                40 => {
                                    n += 1;
                                }
                                41 => {
                                    n -= 1;
                                    if n <= 0 as libc::c_int {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    99 | 100 => {
                        break;
                    }
                    101 | 102 | 103 => {
                        t = 262 as libc::c_int;
                        break;
                    }
                    104 => {
                        exerror(
                            b"short formats not supported\0" as *const u8 as *const libc::c_char,
                        );
                        current_block = 11696355557053689112;
                        break 's_133;
                    }
                    108 => {
                        t = 259 as libc::c_int;
                    }
                    111 | 117 | 120 | 84 => {
                        t = 260 as libc::c_int;
                        break;
                    }
                    115 | 83 => {
                        t = 263 as libc::c_int;
                        break;
                    }
                    _ => {
                        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            break;
                        }
                    }
                }
                if (*(*expr.program).tmp).next >= (*(*expr.program).tmp).endw {
                    _sfflsbuf((*expr.program).tmp, c as libc::c_uchar as libc::c_int);
                } else {
                    let ref mut fresh48 = (*(*expr.program).tmp).next;
                    let fresh49 = *fresh48;
                    *fresh48 = (*fresh48).offset(1);
                    *fresh49 = c as libc::c_uchar;
                };
            }
            if (*(*expr.program).tmp).next >= (*(*expr.program).tmp).endw {
                _sfflsbuf((*expr.program).tmp, c as libc::c_uchar as libc::c_int);
            } else {
                let ref mut fresh50 = (*(*expr.program).tmp).next;
                let fresh51 = *fresh50;
                *fresh50 = (*fresh50).offset(1);
                *fresh51 = c as libc::c_uchar;
            };
            e = s;
            while *s != 0 {
                if *s as libc::c_int == '%' as i32 {
                    s = s.offset(1);
                    if *s == 0 {
                        *e = 0 as libc::c_int as libc::c_char;
                        exerror(
                            b"%s: trailing %% in format\0" as *const u8 as *const libc::c_char,
                            f,
                        );
                        current_block = 11696355557053689112;
                        break 's_133;
                    } else if *s as libc::c_int != '%' as i32 {
                        s = s.offset(-1);
                        break;
                    }
                }
                if (*(*expr.program).tmp).next >= (*(*expr.program).tmp).endw {
                    _sfflsbuf((*expr.program).tmp, *s as libc::c_uchar as libc::c_int);
                } else {
                    let ref mut fresh52 = (*(*expr.program).tmp).next;
                    let fresh53 = *fresh52;
                    *fresh52 = (*fresh52).offset(1);
                    *fresh53 = *s as libc::c_uchar;
                };
                s = s.offset(1);
            }
            if args.is_null() {
                *e = 0 as libc::c_int as libc::c_char;
                exerror(
                    b"%s format argument expected\0" as *const u8 as *const libc::c_char,
                    f,
                );
                current_block = 11696355557053689112;
                break;
            } else {
                let ref mut fresh54 = (*x).arg;
                *fresh54 = (*args).data.operand.left;
                match t {
                    262 => {
                        if (*(*x).arg).type_0 != 262 as libc::c_int {
                            let ref mut fresh55 = (*x).arg;
                            *fresh55 = exnewnode(
                                expr.program,
                                if (*(*x).arg).type_0 == 263 as libc::c_int {
                                    313 as libc::c_int
                                } else if (*(*x).arg).type_0 >= 259 as libc::c_int
                                    && (*(*x).arg).type_0 <= 261 as libc::c_int
                                {
                                    310 as libc::c_int
                                } else {
                                    318 as libc::c_int
                                },
                                0 as libc::c_int,
                                262 as libc::c_int,
                                (*x).arg,
                                if (*(*x).arg).op == 283 as libc::c_int {
                                    (*x).arg
                                } else {
                                    0 as *mut Exnode_t
                                },
                            );
                        }
                    }
                    259 | 260 => {
                        if !((*(*x).arg).type_0 >= 259 as libc::c_int
                            && (*(*x).arg).type_0 <= 261 as libc::c_int)
                        {
                            let ref mut fresh56 = (*x).arg;
                            *fresh56 = exnewnode(
                                expr.program,
                                if (*(*x).arg).type_0 == 263 as libc::c_int {
                                    314 as libc::c_int
                                } else if (*(*x).arg).type_0 == 262 as libc::c_int {
                                    308 as libc::c_int
                                } else {
                                    319 as libc::c_int
                                },
                                0 as libc::c_int,
                                259 as libc::c_int,
                                (*x).arg,
                                if (*(*x).arg).op == 283 as libc::c_int {
                                    (*x).arg
                                } else {
                                    0 as *mut Exnode_t
                                },
                            );
                        }
                        (*(*x).arg).type_0 = t;
                    }
                    263 => {
                        if (*(*x).arg).type_0 != 263 as libc::c_int {
                            if (*(*x).arg).op == 271 as libc::c_int
                                && !((*(*x).arg).data.constant.reference).is_null()
                                && ((*(*expr.program).disc).convertf).is_some()
                            {
                                if (Some(
                                    ((*(*expr.program).disc).convertf)
                                        .expect("non-null function pointer"),
                                ))
                                .expect("non-null function pointer")(
                                    expr.program,
                                    (*x).arg,
                                    263 as libc::c_int,
                                    (*(*x).arg).data.constant.reference,
                                    0 as libc::c_int,
                                    (*expr.program).disc,
                                ) < 0 as libc::c_int
                                {
                                    exerror(
                                        b"cannot convert string format argument\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    let ref mut fresh57 = (*(*x).arg).data.constant.value.string;
                                    *fresh57 = vmstrdup(
                                        (*expr.program).vm,
                                        (*(*x).arg).data.constant.value.string,
                                    );
                                }
                            } else if ((*(*expr.program).disc).convertf).is_none()
                                || (*(*x).arg).op != 283 as libc::c_int
                                    && (*(*x).arg).op != 275 as libc::c_int
                                    && (*(*x).arg).op != 315 as libc::c_int
                                    && (*(*x).arg).op != 316 as libc::c_int
                                    && (*(*x).arg).op != 317 as libc::c_int
                            {
                                exerror(
                                    b"string format argument expected\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                let ref mut fresh58 = (*x).arg;
                                *fresh58 = exnewnode(
                                    expr.program,
                                    if (*(*x).arg).type_0 == 262 as libc::c_int {
                                        309 as libc::c_int
                                    } else if (*(*x).arg).type_0 >= 259 as libc::c_int
                                        && (*(*x).arg).type_0 <= 261 as libc::c_int
                                    {
                                        311 as libc::c_int
                                    } else {
                                        320 as libc::c_int
                                    },
                                    0 as libc::c_int,
                                    263 as libc::c_int,
                                    (*x).arg,
                                    if (*(*x).arg).op == 283 as libc::c_int {
                                        (*x).arg
                                    } else {
                                        0 as *mut Exnode_t
                                    },
                                );
                            }
                        }
                    }
                    _ => {}
                }
                args = (*args).data.operand.right;
            }
        }
        let ref mut fresh59 = (*x).format;
        *fresh59 = exstash((*expr.program).tmp, (*expr.program).vm);
        if *s == 0 {
            current_block = 6478348674394853609;
            break;
        }
        f = s;
    }
    match current_block {
        6478348674394853609 => {
            if !args.is_null() {
                exerror(b"too many format arguments\0" as *const u8 as *const libc::c_char);
            }
        }
        _ => {}
    }
    if (0 as libc::c_int) < 0 as libc::c_int
        || 0 as libc::c_int as libc::c_long > (*(*expr.program).tmp).size
    {
    } else {
        let ref mut fresh60 = (*(*expr.program).tmp).next;
        *fresh60 = ((*(*expr.program).tmp).data).offset(0 as libc::c_int as isize);
    };
    return p;
}
unsafe extern "C" fn exprint(
    mut p: *mut Expr_t,
    mut ex: *mut Exid_t,
    mut args: *mut Exnode_t,
) -> *mut Exnode_t {
    let mut arg: *mut Exnode_t = args;
    let mut pr: *mut Exnode_t = 0 as *mut Exnode_t;
    while !arg.is_null() {
        if (*(*arg).data.operand.left).type_0 != 263 as libc::c_int {
            let ref mut fresh61 = (*arg).data.operand.left;
            *fresh61 = exstringOf(p, (*arg).data.operand.left);
        }
        arg = (*arg).data.operand.right;
    }
    pr = exnewnode(
        p,
        (*ex).index as libc::c_int,
        1 as libc::c_int,
        (*ex).type_0 as libc::c_int,
        args,
        0 as *mut Exnode_t,
    );
    return pr;
}
unsafe extern "C" fn exstringOf(mut p: *mut Expr_t, mut x: *mut Exnode_t) -> *mut Exnode_t {
    let mut type_0: libc::c_int = (*x).type_0;
    let mut cvt: libc::c_int = 0 as libc::c_int;
    if type_0 == 0 {
        (*x).type_0 = 263 as libc::c_int;
        return x;
    }
    if !(type_0 > 258 as libc::c_int) && ((*(*p).disc).stringof).is_none() {
        exerror(
            b"cannot convert %s to STRING\0" as *const u8 as *const libc::c_char,
            extypename(p, type_0),
        );
    }
    if (*x).op != 271 as libc::c_int {
        if !(type_0 > 258 as libc::c_int) {
            if (Some(((*(*p).disc).stringof).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                p, x, 1 as libc::c_int, (*p).disc
            ) < 0 as libc::c_int
            {
                exerror(
                    b"cannot convert %s to STRING\0" as *const u8 as *const libc::c_char,
                    extypename(p, type_0),
                );
            }
            cvt = 322 as libc::c_int;
        } else {
            match type_0 {
                262 => {
                    cvt = 309 as libc::c_int;
                }
                259 => {
                    cvt = 311 as libc::c_int;
                }
                _ => {}
            }
        }
        x = exnewnode(
            p,
            cvt,
            0 as libc::c_int,
            263 as libc::c_int,
            x,
            0 as *mut Exnode_t,
        );
    } else if !(type_0 > 258 as libc::c_int) {
        if (Some(((*(*p).disc).stringof).expect("non-null function pointer")))
            .expect("non-null function pointer")(p, x, 0 as libc::c_int, (*p).disc)
            < 0 as libc::c_int
        {
            exerror(
                b"cannot convert constant %s to STRING\0" as *const u8 as *const libc::c_char,
                extypename(p, (*x).type_0),
            );
        }
    } else {
        match type_0 {
            262 => {
                let ref mut fresh62 = (*x).data.constant.value.string;
                *fresh62 = exprintf(
                    (*p).vm,
                    b"%g\0" as *const u8 as *const libc::c_char,
                    (*x).data.constant.value.floating,
                );
            }
            259 => {
                let ref mut fresh63 = (*x).data.constant.value.string;
                *fresh63 = exprintf(
                    (*p).vm,
                    b"%lld\0" as *const u8 as *const libc::c_char,
                    (*x).data.constant.value.integer,
                );
            }
            _ => {
                exerror(
                    b"internal error: %d: unknown type\0" as *const u8 as *const libc::c_char,
                    type_0,
                );
            }
        }
    }
    (*x).type_0 = 263 as libc::c_int;
    return x;
}
unsafe extern "C" fn exnewsplit(
    mut p: *mut Expr_t,
    mut op: libc::c_int,
    mut dyn_0: *mut Exid_t,
    mut s: *mut Exnode_t,
    mut seps: *mut Exnode_t,
) -> *mut Exnode_t {
    let mut ss: *mut Exnode_t = 0 as *mut Exnode_t;
    if ((*dyn_0).local.pointer).is_null() {
        exerror(
            b"cannot use non-array %s in %s\0" as *const u8 as *const libc::c_char,
            ((*dyn_0).name).as_mut_ptr(),
            exopname(op),
        );
    }
    if (*dyn_0).index_type > 0 as libc::c_int as libc::c_long
        && (*dyn_0).index_type != 259 as libc::c_int as libc::c_long
    {
        exerror(
            b"in %s, array %s must have integer index type, not %s\0" as *const u8
                as *const libc::c_char,
            exopname(op),
            ((*dyn_0).name).as_mut_ptr(),
            extypename(p, (*s).type_0),
        );
    }
    if (*dyn_0).type_0 != 263 as libc::c_int as libc::c_long {
        exerror(
            b"in %s, array %s entries must have string type, not %s\0" as *const u8
                as *const libc::c_char,
            exopname(op),
            ((*dyn_0).name).as_mut_ptr(),
            extypename(p, (*s).type_0),
        );
    }
    if (*s).type_0 != 263 as libc::c_int {
        exerror(
            b"first argument to %s must have string type, not %s\0" as *const u8
                as *const libc::c_char,
            exopname(op),
            extypename(p, (*s).type_0),
        );
    }
    if !seps.is_null() && (*seps).type_0 != 263 as libc::c_int {
        exerror(
            b"third argument to %s must have string type, not %s\0" as *const u8
                as *const libc::c_char,
            exopname(op),
            extypename(p, (*seps).type_0),
        );
    }
    ss = exnewnode(
        p,
        op,
        0 as libc::c_int,
        259 as libc::c_int,
        0 as *mut Exnode_t,
        0 as *mut Exnode_t,
    );
    let ref mut fresh64 = (*ss).data.split.array;
    *fresh64 = dyn_0;
    let ref mut fresh65 = (*ss).data.split.string;
    *fresh65 = s;
    let ref mut fresh66 = (*ss).data.split.seps;
    *fresh66 = seps;
    return ss;
}
unsafe extern "C" fn exnewsubstr(mut p: *mut Expr_t, mut args: *mut Exnode_t) -> *mut Exnode_t {
    let mut base: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut pat: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut repl: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut ss: *mut Exnode_t = 0 as *mut Exnode_t;
    base = extract(p, &mut args, 263 as libc::c_int);
    if base.is_null() {
        exerror(b"invalid first argument to substr operator\0" as *const u8 as *const libc::c_char);
    }
    pat = extract(p, &mut args, 259 as libc::c_int);
    if pat.is_null() {
        exerror(
            b"invalid second argument to substr operator\0" as *const u8 as *const libc::c_char,
        );
    }
    if !args.is_null() {
        repl = extract(p, &mut args, 259 as libc::c_int);
        if repl.is_null() {
            exerror(
                b"invalid third argument to substr operator\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        repl = 0 as *mut Exnode_t;
    }
    if !args.is_null() {
        exerror(b"too many arguments to substr operator\0" as *const u8 as *const libc::c_char);
    }
    ss = exnewnode(
        p,
        303 as libc::c_int,
        0 as libc::c_int,
        263 as libc::c_int,
        0 as *mut Exnode_t,
        0 as *mut Exnode_t,
    );
    let ref mut fresh67 = (*ss).data.string.base;
    *fresh67 = base;
    let ref mut fresh68 = (*ss).data.string.pat;
    *fresh68 = pat;
    let ref mut fresh69 = (*ss).data.string.repl;
    *fresh69 = repl;
    return ss;
}
unsafe extern "C" fn extract(
    mut p: *mut Expr_t,
    mut argp: *mut *mut Exnode_t,
    mut type_0: libc::c_int,
) -> *mut Exnode_t {
    let mut args: *mut Exnode_t = *argp;
    let mut left: *mut Exnode_t = 0 as *mut Exnode_t;
    if args.is_null() || type_0 != (*(*args).data.operand.left).type_0 {
        return 0 as *mut Exnode_t;
    }
    *argp = (*args).data.operand.right;
    left = (*args).data.operand.left;
    let ref mut fresh70 = (*args).data.operand.right;
    *fresh70 = 0 as *mut Exnode_t;
    let ref mut fresh71 = (*args).data.operand.left;
    *fresh71 = *fresh70;
    exfreenode(p, args);
    return left;
}
unsafe extern "C" fn exnewsub(
    mut p: *mut Expr_t,
    mut args: *mut Exnode_t,
    mut op: libc::c_int,
) -> *mut Exnode_t {
    let mut base: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut pat: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut repl: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut ss: *mut Exnode_t = 0 as *mut Exnode_t;
    base = extract(p, &mut args, 263 as libc::c_int);
    if base.is_null() {
        exerror(b"invalid first argument to sub operator\0" as *const u8 as *const libc::c_char);
    }
    pat = extract(p, &mut args, 263 as libc::c_int);
    if pat.is_null() {
        exerror(b"invalid second argument to sub operator\0" as *const u8 as *const libc::c_char);
    }
    if !args.is_null() {
        repl = extract(p, &mut args, 263 as libc::c_int);
        if repl.is_null() {
            exerror(
                b"invalid third argument to sub operator\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        repl = 0 as *mut Exnode_t;
    }
    if !args.is_null() {
        exerror(b"too many arguments to sub operator\0" as *const u8 as *const libc::c_char);
    }
    ss = exnewnode(
        p,
        op,
        0 as libc::c_int,
        263 as libc::c_int,
        0 as *mut Exnode_t,
        0 as *mut Exnode_t,
    );
    let ref mut fresh72 = (*ss).data.string.base;
    *fresh72 = base;
    let ref mut fresh73 = (*ss).data.string.pat;
    *fresh73 = pat;
    let ref mut fresh74 = (*ss).data.string.repl;
    *fresh74 = repl;
    return ss;
}
unsafe extern "C" fn call(
    mut ref_0: *mut Exref_t,
    mut fun: *mut Exid_t,
    mut args: *mut Exnode_t,
) -> *mut Exnode_t {
    let mut t: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut x: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut num: libc::c_int = 0;
    x = exnewnode(
        expr.program,
        283 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut Exnode_t,
        0 as *mut Exnode_t,
    );
    t = (*fun).type_0 as libc::c_int;
    fun = if !ref_0.is_null()
        && (*(*expr.program).disc).flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong
            != 0
    {
        qualify(ref_0, fun)
    } else {
        fun
    };
    let ref mut fresh75 = (*x).data.variable.symbol;
    *fresh75 = fun;
    let ref mut fresh76 = (*x).data.variable.reference;
    *fresh76 = ref_0;
    num = 0 as libc::c_int;
    t >>= 4 as libc::c_int;
    loop {
        type_0 = T(t);
        if !(type_0 != 0) {
            break;
        }
        if args.is_null() {
            exerror(
                b"%s: not enough args\0" as *const u8 as *const libc::c_char,
                ((*fun).name).as_mut_ptr(),
            );
            return args;
        }
        num += 1;
        if type_0 != (*(*args).data.operand.left).type_0 {
            let ref mut fresh77 = (*args).data.operand.left;
            *fresh77 = excast(
                expr.program,
                (*args).data.operand.left,
                type_0,
                0 as *mut Exnode_t,
                num,
            );
        }
        args = (*args).data.operand.right;
        t >>= 4 as libc::c_int;
    }
    if !args.is_null() {
        exerror(
            b"%s: too many args\0" as *const u8 as *const libc::c_char,
            ((*fun).name).as_mut_ptr(),
        );
    }
    return x;
}
unsafe extern "C" fn T(mut t: libc::c_int) -> libc::c_int {
    if !((*(*expr.program).disc).types).is_null() {
        return *((*(*expr.program).disc).types)
            .offset((t & ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as isize);
    } else {
        return a2t[(t & ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int) as usize];
    };
}
static mut a2t: [libc::c_int; 4] = [
    0 as libc::c_int,
    262 as libc::c_int,
    259 as libc::c_int,
    263 as libc::c_int,
];
unsafe extern "C" fn qualify(mut ref_0: *mut Exref_t, mut sym: *mut Exid_t) -> *mut Exid_t {
    let mut x: *mut Exid_t = 0 as *mut Exid_t;
    while !((*ref_0).next).is_null() {
        ref_0 = (*ref_0).next;
    }
    let mut len: size_t = (strlen(((*(*ref_0).symbol).name).as_mut_ptr()))
        .wrapping_add(strlen(((*sym).name).as_mut_ptr()))
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    let mut s: *mut libc::c_char =
        malloc((::std::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(len))
            as *mut libc::c_char;
    if s.is_null() {
        exnospace();
        return 0 as *mut Exid_t;
    }
    snprintf(
        s,
        len,
        b"%s.%s\0" as *const u8 as *const libc::c_char,
        ((*(*ref_0).symbol).name).as_mut_ptr(),
        ((*sym).name).as_mut_ptr(),
    );
    x = (Some(((*(*expr.program).symbols).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        (*expr.program).symbols,
        s as *mut libc::c_void,
        0o1000 as libc::c_int,
    ) as *mut Exid_t;
    if x.is_null() {
        x = if 0 as libc::c_int != 0 {
            realloc(
                0 as *mut libc::c_char as *mut libc::c_void,
                (::std::mem::size_of::<Exid_t>() as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (strlen(s))
                            .wrapping_sub(32 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut Exid_t
        } else {
            calloc(
                1 as libc::c_int as libc::c_ulong,
                (::std::mem::size_of::<Exid_t>() as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (strlen(s))
                            .wrapping_sub(32 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut Exid_t
        };
        if !x.is_null() {
            memcpy(
                x as *mut libc::c_void,
                sym as *const libc::c_void,
                (::std::mem::size_of::<Exid_t>() as libc::c_ulong)
                    .wrapping_sub(32 as libc::c_int as libc::c_ulong),
            );
            strcpy(((*x).name).as_mut_ptr(), s);
            (Some(((*(*expr.program).symbols).searchf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                (*expr.program).symbols,
                x as *mut libc::c_void,
                0o1 as libc::c_int,
            );
        } else {
            exnospace();
            x = sym;
        }
    }
    free(s as *mut libc::c_void);
    return x;
}
unsafe extern "C" fn checkBinary(
    mut p: *mut Expr_t,
    mut l: *mut Exnode_t,
    mut ex: *mut Exnode_t,
    mut r: *mut Exnode_t,
) {
    if (Some(((*(*p).disc).binaryf).expect("non-null function pointer")))
        .expect("non-null function pointer")(p, l, ex, r, 1 as libc::c_int, (*p).disc)
        < 0 as libc::c_int
    {
        if !r.is_null() {
            exerror(
                b"cannot apply operator %s to expressions of types %s and %s\0" as *const u8
                    as *const libc::c_char,
                exopname((*ex).op),
                extypename(p, (*l).type_0),
                extypename(p, (*r).type_0),
            );
        } else {
            exerror(
                b"cannot apply operator %s to expression of type %s\0" as *const u8
                    as *const libc::c_char,
                exopname((*ex).op),
                extypename(p, (*l).type_0),
            );
        }
    }
}
unsafe extern "C" fn cmpKey(
    mut d: *mut Dt_t,
    mut key1: *mut Extype_t,
    mut key2: *mut Extype_t,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    if (*key1).integer < (*key2).integer {
        return -(1 as libc::c_int);
    } else if (*key1).integer > (*key2).integer {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn checkName(mut id: *mut Exid_t) {
    match (*id).lex {
        275 => {
            exerror(
                b"Variable \"%s\" already declared\0" as *const u8 as *const libc::c_char,
                ((*id).name).as_mut_ptr(),
            );
        }
        279 => {
            exerror(
                b"Name \"%s\" already used as a function\0" as *const u8 as *const libc::c_char,
                ((*id).name).as_mut_ptr(),
            );
        }
        283 => {
            exerror(
                b"Name \"%s\" already used as a keyword\0" as *const u8 as *const libc::c_char,
                ((*id).name).as_mut_ptr(),
            );
        }
        287 => {}
        _ => {
            _err_msg(
                0xff as libc::c_int,
                b"Unexpected token \"%s\" as name in dcl_item\0" as *const u8
                    as *const libc::c_char,
                ((*id).name).as_mut_ptr(),
            );
        }
    };
}
static mut swstate: Switch_t = Switch_t {
    prev: 0 as *const Switch_s as *mut Switch_s,
    firstcase: 0 as *const Exnode_t as *mut Exnode_t,
    lastcase: 0 as *const Exnode_t as *mut Exnode_t,
    defcase: 0 as *const Exnode_t as *mut Exnode_t,
    base: 0 as *const *mut Extype_t as *mut *mut Extype_t,
    cur: 0 as *const *mut Extype_t as *mut *mut Extype_t,
    last: 0 as *const *mut Extype_t as *mut *mut Extype_t,
    def: 0,
    type_0: 0,
};
#[no_mangle]
pub unsafe extern "C" fn exisAssign(mut n: *mut Exnode_t) -> libc::c_int {
    return ((*n).op == '=' as i32 && (*n).subop == '=' as i32) as libc::c_int;
}
static mut yytranslate: [yytype_uint8; 337] = [
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
    91 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
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
    71 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
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
    104 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
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
    100 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
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
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
];
static mut yyrline: [yytype_uint16; 143] = [
    0 as libc::c_int as yytype_uint16,
    189 as libc::c_int as yytype_uint16,
    189 as libc::c_int as yytype_uint16,
    210 as libc::c_int as yytype_uint16,
    211 as libc::c_int as yytype_uint16,
    214 as libc::c_int as yytype_uint16,
    214 as libc::c_int as yytype_uint16,
    254 as libc::c_int as yytype_uint16,
    257 as libc::c_int as yytype_uint16,
    272 as libc::c_int as yytype_uint16,
    276 as libc::c_int as yytype_uint16,
    280 as libc::c_int as yytype_uint16,
    280 as libc::c_int as yytype_uint16,
    280 as libc::c_int as yytype_uint16,
    285 as libc::c_int as yytype_uint16,
    295 as libc::c_int as yytype_uint16,
    308 as libc::c_int as yytype_uint16,
    323 as libc::c_int as yytype_uint16,
    336 as libc::c_int as yytype_uint16,
    344 as libc::c_int as yytype_uint16,
    355 as libc::c_int as yytype_uint16,
    365 as libc::c_int as yytype_uint16,
    365 as libc::c_int as yytype_uint16,
    376 as libc::c_int as yytype_uint16,
    388 as libc::c_int as yytype_uint16,
    392 as libc::c_int as yytype_uint16,
    405 as libc::c_int as yytype_uint16,
    435 as libc::c_int as yytype_uint16,
    438 as libc::c_int as yytype_uint16,
    470 as libc::c_int as yytype_uint16,
    471 as libc::c_int as yytype_uint16,
    474 as libc::c_int as yytype_uint16,
    495 as libc::c_int as yytype_uint16,
    502 as libc::c_int as yytype_uint16,
    505 as libc::c_int as yytype_uint16,
    511 as libc::c_int as yytype_uint16,
    512 as libc::c_int as yytype_uint16,
    519 as libc::c_int as yytype_uint16,
    519 as libc::c_int as yytype_uint16,
    568 as libc::c_int as yytype_uint16,
    569 as libc::c_int as yytype_uint16,
    570 as libc::c_int as yytype_uint16,
    571 as libc::c_int as yytype_uint16,
    574 as libc::c_int as yytype_uint16,
    575 as libc::c_int as yytype_uint16,
    579 as libc::c_int as yytype_uint16,
    582 as libc::c_int as yytype_uint16,
    589 as libc::c_int as yytype_uint16,
    592 as libc::c_int as yytype_uint16,
    595 as libc::c_int as yytype_uint16,
    599 as libc::c_int as yytype_uint16,
    603 as libc::c_int as yytype_uint16,
    656 as libc::c_int as yytype_uint16,
    660 as libc::c_int as yytype_uint16,
    664 as libc::c_int as yytype_uint16,
    668 as libc::c_int as yytype_uint16,
    672 as libc::c_int as yytype_uint16,
    676 as libc::c_int as yytype_uint16,
    680 as libc::c_int as yytype_uint16,
    684 as libc::c_int as yytype_uint16,
    688 as libc::c_int as yytype_uint16,
    692 as libc::c_int as yytype_uint16,
    696 as libc::c_int as yytype_uint16,
    700 as libc::c_int as yytype_uint16,
    704 as libc::c_int as yytype_uint16,
    708 as libc::c_int as yytype_uint16,
    712 as libc::c_int as yytype_uint16,
    716 as libc::c_int as yytype_uint16,
    729 as libc::c_int as yytype_uint16,
    733 as libc::c_int as yytype_uint16,
    743 as libc::c_int as yytype_uint16,
    743 as libc::c_int as yytype_uint16,
    743 as libc::c_int as yytype_uint16,
    784 as libc::c_int as yytype_uint16,
    804 as libc::c_int as yytype_uint16,
    811 as libc::c_int as yytype_uint16,
    815 as libc::c_int as yytype_uint16,
    819 as libc::c_int as yytype_uint16,
    823 as libc::c_int as yytype_uint16,
    827 as libc::c_int as yytype_uint16,
    831 as libc::c_int as yytype_uint16,
    835 as libc::c_int as yytype_uint16,
    839 as libc::c_int as yytype_uint16,
    843 as libc::c_int as yytype_uint16,
    847 as libc::c_int as yytype_uint16,
    851 as libc::c_int as yytype_uint16,
    855 as libc::c_int as yytype_uint16,
    861 as libc::c_int as yytype_uint16,
    865 as libc::c_int as yytype_uint16,
    869 as libc::c_int as yytype_uint16,
    875 as libc::c_int as yytype_uint16,
    880 as libc::c_int as yytype_uint16,
    884 as libc::c_int as yytype_uint16,
    909 as libc::c_int as yytype_uint16,
    945 as libc::c_int as yytype_uint16,
    965 as libc::c_int as yytype_uint16,
    973 as libc::c_int as yytype_uint16,
    981 as libc::c_int as yytype_uint16,
    992 as libc::c_int as yytype_uint16,
    996 as libc::c_int as yytype_uint16,
    1000 as libc::c_int as yytype_uint16,
    1003 as libc::c_int as yytype_uint16,
    1004 as libc::c_int as yytype_uint16,
    1006 as libc::c_int as yytype_uint16,
    1014 as libc::c_int as yytype_uint16,
    1019 as libc::c_int as yytype_uint16,
    1024 as libc::c_int as yytype_uint16,
    1029 as libc::c_int as yytype_uint16,
    1036 as libc::c_int as yytype_uint16,
    1037 as libc::c_int as yytype_uint16,
    1038 as libc::c_int as yytype_uint16,
    1041 as libc::c_int as yytype_uint16,
    1042 as libc::c_int as yytype_uint16,
    1045 as libc::c_int as yytype_uint16,
    1049 as libc::c_int as yytype_uint16,
    1069 as libc::c_int as yytype_uint16,
    1082 as libc::c_int as yytype_uint16,
    1085 as libc::c_int as yytype_uint16,
    1089 as libc::c_int as yytype_uint16,
    1103 as libc::c_int as yytype_uint16,
    1106 as libc::c_int as yytype_uint16,
    1113 as libc::c_int as yytype_uint16,
    1116 as libc::c_int as yytype_uint16,
    1124 as libc::c_int as yytype_uint16,
    1129 as libc::c_int as yytype_uint16,
    1136 as libc::c_int as yytype_uint16,
    1139 as libc::c_int as yytype_uint16,
    1145 as libc::c_int as yytype_uint16,
    1148 as libc::c_int as yytype_uint16,
    1152 as libc::c_int as yytype_uint16,
    1163 as libc::c_int as yytype_uint16,
    1163 as libc::c_int as yytype_uint16,
    1176 as libc::c_int as yytype_uint16,
    1179 as libc::c_int as yytype_uint16,
    1192 as libc::c_int as yytype_uint16,
    1213 as libc::c_int as yytype_uint16,
    1217 as libc::c_int as yytype_uint16,
    1223 as libc::c_int as yytype_uint16,
    1226 as libc::c_int as yytype_uint16,
    1233 as libc::c_int as yytype_uint16,
    1234 as libc::c_int as yytype_uint16,
    1251 as libc::c_int as yytype_uint16,
    1234 as libc::c_int as yytype_uint16,
];
static mut yytname: [*const libc::c_char; 152] = [
    b"$end\0" as *const u8 as *const libc::c_char,
    b"error\0" as *const u8 as *const libc::c_char,
    b"$undefined\0" as *const u8 as *const libc::c_char,
    b"MINTOKEN\0" as *const u8 as *const libc::c_char,
    b"INTEGER\0" as *const u8 as *const libc::c_char,
    b"UNSIGNED\0" as *const u8 as *const libc::c_char,
    b"CHARACTER\0" as *const u8 as *const libc::c_char,
    b"FLOATING\0" as *const u8 as *const libc::c_char,
    b"STRING\0" as *const u8 as *const libc::c_char,
    b"VOIDTYPE\0" as *const u8 as *const libc::c_char,
    b"STATIC\0" as *const u8 as *const libc::c_char,
    b"ADDRESS\0" as *const u8 as *const libc::c_char,
    b"ARRAY\0" as *const u8 as *const libc::c_char,
    b"BREAK\0" as *const u8 as *const libc::c_char,
    b"CALL\0" as *const u8 as *const libc::c_char,
    b"CASE\0" as *const u8 as *const libc::c_char,
    b"CONSTANT\0" as *const u8 as *const libc::c_char,
    b"CONTINUE\0" as *const u8 as *const libc::c_char,
    b"DECLARE\0" as *const u8 as *const libc::c_char,
    b"DEFAULT\0" as *const u8 as *const libc::c_char,
    b"DYNAMIC\0" as *const u8 as *const libc::c_char,
    b"ELSE\0" as *const u8 as *const libc::c_char,
    b"EXIT\0" as *const u8 as *const libc::c_char,
    b"FOR\0" as *const u8 as *const libc::c_char,
    b"FUNCTION\0" as *const u8 as *const libc::c_char,
    b"GSUB\0" as *const u8 as *const libc::c_char,
    b"ITERATE\0" as *const u8 as *const libc::c_char,
    b"ITERATER\0" as *const u8 as *const libc::c_char,
    b"ID\0" as *const u8 as *const libc::c_char,
    b"IF\0" as *const u8 as *const libc::c_char,
    b"LABEL\0" as *const u8 as *const libc::c_char,
    b"MEMBER\0" as *const u8 as *const libc::c_char,
    b"NAME\0" as *const u8 as *const libc::c_char,
    b"POS\0" as *const u8 as *const libc::c_char,
    b"PRAGMA\0" as *const u8 as *const libc::c_char,
    b"PRE\0" as *const u8 as *const libc::c_char,
    b"PRINT\0" as *const u8 as *const libc::c_char,
    b"PRINTF\0" as *const u8 as *const libc::c_char,
    b"PROCEDURE\0" as *const u8 as *const libc::c_char,
    b"QUERY\0" as *const u8 as *const libc::c_char,
    b"RAND\0" as *const u8 as *const libc::c_char,
    b"RETURN\0" as *const u8 as *const libc::c_char,
    b"SCANF\0" as *const u8 as *const libc::c_char,
    b"SPLIT\0" as *const u8 as *const libc::c_char,
    b"SPRINTF\0" as *const u8 as *const libc::c_char,
    b"SRAND\0" as *const u8 as *const libc::c_char,
    b"SSCANF\0" as *const u8 as *const libc::c_char,
    b"SUB\0" as *const u8 as *const libc::c_char,
    b"SUBSTR\0" as *const u8 as *const libc::c_char,
    b"SWITCH\0" as *const u8 as *const libc::c_char,
    b"TOKENS\0" as *const u8 as *const libc::c_char,
    b"UNSET\0" as *const u8 as *const libc::c_char,
    b"WHILE\0" as *const u8 as *const libc::c_char,
    b"F2I\0" as *const u8 as *const libc::c_char,
    b"F2S\0" as *const u8 as *const libc::c_char,
    b"I2F\0" as *const u8 as *const libc::c_char,
    b"I2S\0" as *const u8 as *const libc::c_char,
    b"S2B\0" as *const u8 as *const libc::c_char,
    b"S2F\0" as *const u8 as *const libc::c_char,
    b"S2I\0" as *const u8 as *const libc::c_char,
    b"F2X\0" as *const u8 as *const libc::c_char,
    b"I2X\0" as *const u8 as *const libc::c_char,
    b"S2X\0" as *const u8 as *const libc::c_char,
    b"X2F\0" as *const u8 as *const libc::c_char,
    b"X2I\0" as *const u8 as *const libc::c_char,
    b"X2S\0" as *const u8 as *const libc::c_char,
    b"X2X\0" as *const u8 as *const libc::c_char,
    b"XPRINT\0" as *const u8 as *const libc::c_char,
    b"','\0" as *const u8 as *const libc::c_char,
    b"'='\0" as *const u8 as *const libc::c_char,
    b"'?'\0" as *const u8 as *const libc::c_char,
    b"':'\0" as *const u8 as *const libc::c_char,
    b"OR\0" as *const u8 as *const libc::c_char,
    b"AND\0" as *const u8 as *const libc::c_char,
    b"'|'\0" as *const u8 as *const libc::c_char,
    b"'^'\0" as *const u8 as *const libc::c_char,
    b"'&'\0" as *const u8 as *const libc::c_char,
    b"EQ\0" as *const u8 as *const libc::c_char,
    b"NE\0" as *const u8 as *const libc::c_char,
    b"'<'\0" as *const u8 as *const libc::c_char,
    b"'>'\0" as *const u8 as *const libc::c_char,
    b"LE\0" as *const u8 as *const libc::c_char,
    b"GE\0" as *const u8 as *const libc::c_char,
    b"LSH\0" as *const u8 as *const libc::c_char,
    b"RSH\0" as *const u8 as *const libc::c_char,
    b"'+'\0" as *const u8 as *const libc::c_char,
    b"'-'\0" as *const u8 as *const libc::c_char,
    b"IN_OP\0" as *const u8 as *const libc::c_char,
    b"'*'\0" as *const u8 as *const libc::c_char,
    b"'/'\0" as *const u8 as *const libc::c_char,
    b"'%'\0" as *const u8 as *const libc::c_char,
    b"'!'\0" as *const u8 as *const libc::c_char,
    b"'~'\0" as *const u8 as *const libc::c_char,
    b"'#'\0" as *const u8 as *const libc::c_char,
    b"UNARY\0" as *const u8 as *const libc::c_char,
    b"INC\0" as *const u8 as *const libc::c_char,
    b"DEC\0" as *const u8 as *const libc::c_char,
    b"CAST\0" as *const u8 as *const libc::c_char,
    b"'('\0" as *const u8 as *const libc::c_char,
    b"MAXTOKEN\0" as *const u8 as *const libc::c_char,
    b"'{'\0" as *const u8 as *const libc::c_char,
    b"'}'\0" as *const u8 as *const libc::c_char,
    b"';'\0" as *const u8 as *const libc::c_char,
    b"')'\0" as *const u8 as *const libc::c_char,
    b"'['\0" as *const u8 as *const libc::c_char,
    b"']'\0" as *const u8 as *const libc::c_char,
    b"'.'\0" as *const u8 as *const libc::c_char,
    b"$accept\0" as *const u8 as *const libc::c_char,
    b"program\0" as *const u8 as *const libc::c_char,
    b"action_list\0" as *const u8 as *const libc::c_char,
    b"action\0" as *const u8 as *const libc::c_char,
    b"$@1\0" as *const u8 as *const libc::c_char,
    b"statement_list\0" as *const u8 as *const libc::c_char,
    b"statement\0" as *const u8 as *const libc::c_char,
    b"$@2\0" as *const u8 as *const libc::c_char,
    b"$@3\0" as *const u8 as *const libc::c_char,
    b"$@4\0" as *const u8 as *const libc::c_char,
    b"switch_list\0" as *const u8 as *const libc::c_char,
    b"switch_item\0" as *const u8 as *const libc::c_char,
    b"case_list\0" as *const u8 as *const libc::c_char,
    b"case_item\0" as *const u8 as *const libc::c_char,
    b"static\0" as *const u8 as *const libc::c_char,
    b"dcl_list\0" as *const u8 as *const libc::c_char,
    b"dcl_item\0" as *const u8 as *const libc::c_char,
    b"$@5\0" as *const u8 as *const libc::c_char,
    b"dcl_name\0" as *const u8 as *const libc::c_char,
    b"name\0" as *const u8 as *const libc::c_char,
    b"else_opt\0" as *const u8 as *const libc::c_char,
    b"expr_opt\0" as *const u8 as *const libc::c_char,
    b"expr\0" as *const u8 as *const libc::c_char,
    b"$@6\0" as *const u8 as *const libc::c_char,
    b"$@7\0" as *const u8 as *const libc::c_char,
    b"splitop\0" as *const u8 as *const libc::c_char,
    b"constant\0" as *const u8 as *const libc::c_char,
    b"print\0" as *const u8 as *const libc::c_char,
    b"scan\0" as *const u8 as *const libc::c_char,
    b"variable\0" as *const u8 as *const libc::c_char,
    b"array\0" as *const u8 as *const libc::c_char,
    b"index\0" as *const u8 as *const libc::c_char,
    b"args\0" as *const u8 as *const libc::c_char,
    b"arg_list\0" as *const u8 as *const libc::c_char,
    b"formals\0" as *const u8 as *const libc::c_char,
    b"formal_list\0" as *const u8 as *const libc::c_char,
    b"formal_item\0" as *const u8 as *const libc::c_char,
    b"$@8\0" as *const u8 as *const libc::c_char,
    b"members\0" as *const u8 as *const libc::c_char,
    b"member\0" as *const u8 as *const libc::c_char,
    b"assign\0" as *const u8 as *const libc::c_char,
    b"initialize\0" as *const u8 as *const libc::c_char,
    b"$@9\0" as *const u8 as *const libc::c_char,
    b"$@10\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut yypact: [yytype_int16; 286] = [
    -(144 as libc::c_int) as yytype_int16,
    9 as libc::c_int as yytype_int16,
    200 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(89 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(80 as libc::c_int) as yytype_int16,
    -(71 as libc::c_int) as yytype_int16,
    -(64 as libc::c_int) as yytype_int16,
    -(63 as libc::c_int) as yytype_int16,
    -(43 as libc::c_int) as yytype_int16,
    -(35 as libc::c_int) as yytype_int16,
    -(27 as libc::c_int) as yytype_int16,
    -(11 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    596 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    92 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    49 as libc::c_int as yytype_int16,
    872 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    -(37 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(27 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(12 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    284 as libc::c_int as yytype_int16,
    299 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    158 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    916 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    185 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    383 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    -(56 as libc::c_int) as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    482 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    577 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    872 as libc::c_int as yytype_int16,
    -(51 as libc::c_int) as yytype_int16,
    672 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    916 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    934 as libc::c_int as yytype_int16,
    951 as libc::c_int as yytype_int16,
    967 as libc::c_int as yytype_int16,
    982 as libc::c_int as yytype_int16,
    996 as libc::c_int as yytype_int16,
    1010 as libc::c_int as yytype_int16,
    1010 as libc::c_int as yytype_int16,
    1022 as libc::c_int as yytype_int16,
    1022 as libc::c_int as yytype_int16,
    1022 as libc::c_int as yytype_int16,
    1022 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    895 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    916 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    497 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    497 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    497 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    116 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    497 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    87 as libc::c_int as yytype_int16,
    849 as libc::c_int as yytype_int16,
    786 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    916 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    168 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    767 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    200 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(49 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(54 as libc::c_int) as yytype_int16,
    691 as libc::c_int as yytype_int16,
    497 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    87 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    126 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    691 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    128 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(7 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(16 as libc::c_int) as yytype_int16,
    -(44 as libc::c_int) as yytype_int16,
    916 as libc::c_int as yytype_int16,
    815 as libc::c_int as yytype_int16,
    497 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    129 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    183 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    200 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    238 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    174 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    210 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(10 as libc::c_int) as yytype_int16,
    176 as libc::c_int as yytype_int16,
    262 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    181 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    398 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 286] = [
    7 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
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
    7 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
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
    121 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 44] = [
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(48 as libc::c_int) as yytype_int16,
    -(143 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    48 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(9 as libc::c_int) as yytype_int16,
    -(36 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    34 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    101 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    24 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    224 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 44] = [
    -(1 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    206 as libc::c_int as yytype_int16,
    200 as libc::c_int as yytype_int16,
    243 as libc::c_int as yytype_int16,
    253 as libc::c_int as yytype_int16,
    254 as libc::c_int as yytype_int16,
    255 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    225 as libc::c_int as yytype_int16,
    226 as libc::c_int as yytype_int16,
    237 as libc::c_int as yytype_int16,
    227 as libc::c_int as yytype_int16,
    279 as libc::c_int as yytype_int16,
    232 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    159 as libc::c_int as yytype_int16,
    238 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    246 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    271 as libc::c_int as yytype_int16,
    272 as libc::c_int as yytype_int16,
    273 as libc::c_int as yytype_int16,
    274 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    260 as libc::c_int as yytype_int16,
    268 as libc::c_int as yytype_int16,
    275 as libc::c_int as yytype_int16,
];
static mut yytable: [yytype_int16; 1113] = [
    93 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    256 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    250 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    277 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    239 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    235 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    278 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    213 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    214 as libc::c_int as yytype_int16,
    240 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    216 as libc::c_int as yytype_int16,
    202 as libc::c_int as yytype_int16,
    236 as libc::c_int as yytype_int16,
    258 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    215 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    219 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    160 as libc::c_int as yytype_int16,
    161 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    164 as libc::c_int as yytype_int16,
    165 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    171 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    174 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    176 as libc::c_int as yytype_int16,
    177 as libc::c_int as yytype_int16,
    178 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    182 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    257 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    252 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    250 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    251 as libc::c_int as yytype_int16,
    262 as libc::c_int as yytype_int16,
    221 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    222 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    223 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    204 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    224 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    207 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    180 as libc::c_int as yytype_int16,
    181 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    211 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    220 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    175 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    218 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    212 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    183 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    187 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    233 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    192 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    247 as libc::c_int as yytype_int16,
    248 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    265 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    198 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    199 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    209 as libc::c_int as yytype_int16,
    210 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(33 as libc::c_int) as yytype_int16,
    217 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    241 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    245 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    264 as libc::c_int as yytype_int16,
    267 as libc::c_int as yytype_int16,
    284 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    269 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    270 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(126 as libc::c_int) as yytype_int16,
    276 as libc::c_int as yytype_int16,
    280 as libc::c_int as yytype_int16,
    281 as libc::c_int as yytype_int16,
    283 as libc::c_int as yytype_int16,
    266 as libc::c_int as yytype_int16,
    244 as libc::c_int as yytype_int16,
    263 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    282 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    259 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    155 as libc::c_int as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    186 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    285 as libc::c_int as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(47 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    229 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    234 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    261 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    228 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    -(127 as libc::c_int) as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
];
static mut yycheck: [yytype_int16; 1113] = [
    48 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    203 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    231 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    249 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    159 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    205 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    201 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    208 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    187 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    238 as libc::c_int as yytype_int16,
    239 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    254 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    230 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    283 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    254 as libc::c_int as yytype_int16,
    235 as libc::c_int as yytype_int16,
    250 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    276 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    246 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    100 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    102 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    103 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    103 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    100 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    103 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    98 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    100 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    102 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    18 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    103 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    98 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    103 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    98 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    22 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    103 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    91 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    95 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    68 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
];
static mut yystos: [yytype_uint8; 286] = [
    0 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    149 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    150 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 143] = [
    0 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    124 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    143 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    149 as libc::c_int as yytype_uint8,
    150 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
];
static mut yyr2: [yytype_uint8; 143] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn yy_symbol_value_print(
    mut yyoutput: *mut Sfio_t,
    mut yytype: libc::c_int,
    yyvaluep: *const EX_STYPE,
) {
    let mut yyo: *mut Sfio_t = yyoutput;
    if yyvaluep.is_null() {
        return;
    }
}
unsafe extern "C" fn yy_symbol_print(
    mut yyoutput: *mut Sfio_t,
    mut yytype: libc::c_int,
    yyvaluep: *const EX_STYPE,
) {
    sfprintf(
        yyoutput,
        b"%s %s (\0" as *const u8 as *const libc::c_char,
        if yytype < 107 as libc::c_int {
            b"token\0" as *const u8 as *const libc::c_char
        } else {
            b"nterm\0" as *const u8 as *const libc::c_char
        },
        yytname[yytype as usize],
    );
    yy_symbol_value_print(yyoutput, yytype, yyvaluep);
    sfprintf(yyoutput, b")\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn yy_stack_print(mut yybottom: *mut yytype_int16, mut yytop: *mut yytype_int16) {
    sfprintf(sfstderr, b"Stack now\0" as *const u8 as *const libc::c_char);
    while yybottom <= yytop {
        let mut yybot: libc::c_int = *yybottom as libc::c_int;
        sfprintf(
            sfstderr,
            b" %d\0" as *const u8 as *const libc::c_char,
            yybot,
        );
        yybottom = yybottom.offset(1);
    }
    sfprintf(sfstderr, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn yy_reduce_print(
    mut yyssp: *mut yytype_int16,
    mut yyvsp: *mut EX_STYPE,
    mut yyrule: libc::c_int,
) {
    let mut yylno: libc::c_ulong = yyrline[yyrule as usize] as libc::c_ulong;
    let mut yynrhs: libc::c_int = yyr2[yyrule as usize] as libc::c_int;
    let mut yyi: libc::c_int = 0;
    sfprintf(
        sfstderr,
        b"Reducing stack by rule %d (line %lu):\n\0" as *const u8 as *const libc::c_char,
        yyrule - 1 as libc::c_int,
        yylno,
    );
    yyi = 0 as libc::c_int;
    while yyi < yynrhs {
        sfprintf(
            sfstderr,
            b"   $%d = \0" as *const u8 as *const libc::c_char,
            yyi + 1 as libc::c_int,
        );
        yy_symbol_print(
            sfstderr,
            yystos[*yyssp.offset((yyi + 1 as libc::c_int - yynrhs) as isize) as usize]
                as libc::c_int,
            &mut *yyvsp.offset((yyi + 1 as libc::c_int - yynrhs) as isize),
        );
        sfprintf(sfstderr, b"\n\0" as *const u8 as *const libc::c_char);
        yyi += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn exop(mut index: size_t) -> *const libc::c_char {
    let mut minid: size_t = 0;
    minid = 0 as libc::c_int as size_t;
    while !(yytname[minid as usize]).is_null() {
        if strcmp(
            yytname[minid as usize],
            b"MINTOKEN\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            break;
        }
        minid = minid.wrapping_add(1);
    }
    if !(yytname[minid as usize]).is_null()
        && !(b"failed to find MINTOKEN; incorrect token list in exparse.y?\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"yytname[minid] != NULL && \"failed to find MINTOKEN; incorrect token list in exparse.y?\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/expr/exparse.y\0" as *const u8 as *const libc::c_char,
            1294 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"const char *exop(size_t)\0"))
                .as_ptr(),
        );
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = minid;
    i = j;
    while !(yytname[i as usize]).is_null() {
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while *(yytname[i as usize]).offset(k as isize) as libc::c_int != '\0' as i32 {
            if *(yytname[i as usize]).offset(k as isize) as libc::c_int != '_' as i32
                && *(*__ctype_b_loc())
                    .offset(*(yytname[i as usize]).offset(k as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
            {
                break;
            }
            k = k.wrapping_add(1);
        }
        if !(*(yytname[i as usize]).offset(k as isize) as libc::c_int != '\0' as i32) {
            if j == index.wrapping_add(minid) {
                return yytname[i as usize];
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub static mut ex_debug: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ex_error(mut message: *const libc::c_char) {
    exerror(b"%s\0" as *const u8 as *const libc::c_char, message);
}
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut EX_STYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
    if ex_debug != 0 {
        sfprintf(
            sfstderr,
            b"%s \0" as *const u8 as *const libc::c_char,
            yymsg,
        );
        yy_symbol_print(sfstderr, yytype, yyvaluep);
        sfprintf(sfstderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub static mut ex_char: libc::c_int = 0;
#[no_mangle]
pub static mut ex_lval: EX_STYPE = EX_STYPE {
    expr: 0 as *const Exnode_s as *mut Exnode_s,
};
#[no_mangle]
pub static mut ex_nerrs: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ex_parse() -> libc::c_int {
    let mut rel: libc::c_int = 0;
    let mut current_block: u64;
    let mut yystate: libc::c_int = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyvsa: [EX_STYPE; 200] = [EX_STYPE {
        expr: 0 as *const Exnode_s as *mut Exnode_s,
    }; 200];
    let mut yyvs: *mut EX_STYPE = 0 as *mut EX_STYPE;
    let mut yyvsp: *mut EX_STYPE = 0 as *mut EX_STYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: EX_STYPE = EX_STYPE {
        expr: 0 as *const Exnode_s as *mut Exnode_s,
    };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    if ex_debug != 0 {
        sfprintf(
            sfstderr,
            b"Starting parse\n\0" as *const u8 as *const libc::c_char,
        );
    }
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    ex_nerrs = 0 as libc::c_int;
    ex_char = -(2 as libc::c_int);
    'c_3638: loop {
        *yyssp = yystate as yytype_int16;
        if yyss
            .offset(yystacksize as isize)
            .offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_ulong = (yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong;
            if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                current_block = 12056922904886382946;
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
                            .wrapping_add(::std::mem::size_of::<EX_STYPE>() as libc::c_ulong),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 12056922904886382946;
                break;
            }
            let mut yynewbytes: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yytype_int16 as *mut libc::c_void,
                yyss as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr.offset(
                yynewbytes.wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong) as isize,
            );
            let mut yynewbytes_0: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut EX_STYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<EX_STYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                .wrapping_mul(::std::mem::size_of::<EX_STYPE>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr.offset(
                yynewbytes_0.wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                    as isize,
            );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss
                .offset(yysize as isize)
                .offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs
                .offset(yysize as isize)
                .offset(-(1 as libc::c_int as isize));
            if ex_debug != 0 {
                sfprintf(
                    sfstderr,
                    b"Stack size increased to %lu\n\0" as *const u8 as *const libc::c_char,
                    yystacksize,
                );
            }
            if yyss
                .offset(yystacksize as isize)
                .offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 6934551251188356327;
                break;
            }
        }
        if ex_debug != 0 {
            sfprintf(
                sfstderr,
                b"Entering state %d\n\0" as *const u8 as *const libc::c_char,
                yystate,
            );
        }
        if yystate == 3 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 7481093856908124627;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(144 as libc::c_int) {
                current_block = 12878331909665404271;
            } else {
                if ex_char == -(2 as libc::c_int) {
                    if ex_debug != 0 {
                        sfprintf(
                            sfstderr,
                            b"Reading a token: \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    ex_char = extoken_fn(expr.program);
                }
                if ex_char <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    ex_char = yytoken;
                    if ex_debug != 0 {
                        sfprintf(
                            sfstderr,
                            b"Now at end of input.\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    yytoken = if ex_char as libc::c_uint <= 336 as libc::c_int as libc::c_uint {
                        yytranslate[ex_char as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                    if ex_debug != 0 {
                        sfprintf(
                            sfstderr,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            b"Next token is\0" as *const u8 as *const libc::c_char,
                        );
                        yy_symbol_print(sfstderr, yytoken, &mut ex_lval);
                        sfprintf(sfstderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int
                    || (1112 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 12878331909665404271;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        if yyn == -(127 as libc::c_int) {
                            current_block = 4173045695846325437;
                        } else {
                            yyn = -yyn;
                            current_block = 9599741646002396977;
                        }
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                        }
                        if ex_debug != 0 {
                            sfprintf(
                                sfstderr,
                                b"%s \0" as *const u8 as *const libc::c_char,
                                b"Shifting\0" as *const u8 as *const libc::c_char,
                            );
                            yy_symbol_print(sfstderr, yytoken, &mut ex_lval);
                            sfprintf(sfstderr, b"\n\0" as *const u8 as *const libc::c_char);
                        }
                        ex_char = -(2 as libc::c_int);
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = ex_lval;
                        current_block = 11372858789397587571;
                    }
                }
            }
            match current_block {
                12878331909665404271 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        current_block = 4173045695846325437;
                    } else {
                        current_block = 9599741646002396977;
                    }
                }
                _ => {}
            }
            match current_block {
                4173045695846325437 => {
                    yytoken = if ex_char == -(2 as libc::c_int) {
                        -(2 as libc::c_int)
                    } else if ex_char as libc::c_uint <= 336 as libc::c_int as libc::c_uint {
                        yytranslate[ex_char as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                    if yyerrstatus == 0 {
                        ex_nerrs += 1;
                        ex_error(b"syntax error\0" as *const u8 as *const libc::c_char);
                    }
                    if yyerrstatus == 3 as libc::c_int {
                        if ex_char <= 0 as libc::c_int {
                            if ex_char == 0 as libc::c_int {
                                current_block = 6934551251188356327;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                yytoken,
                                &mut ex_lval,
                            );
                            ex_char = -(2 as libc::c_int);
                        }
                    }
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(144 as libc::c_int)) {
                            yyn += 1 as libc::c_int;
                            if 0 as libc::c_int <= yyn
                                && yyn <= 1112 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 6934551251188356327;
                            break 'c_3638;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as libc::c_int,
                            yyvsp,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as libc::c_int;
                        if ex_debug != 0 {
                            yy_stack_print(yyss, yyssp);
                        }
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = ex_lval;
                    if ex_debug != 0 {
                        sfprintf(
                            sfstderr,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            b"Shifting\0" as *const u8 as *const libc::c_char,
                        );
                        yy_symbol_print(sfstderr, yystos[yyn as usize] as libc::c_int, yyvsp);
                        sfprintf(sfstderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                    yystate = yyn;
                }
                9599741646002396977 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    if ex_debug != 0 {
                        yy_reduce_print(yyssp, yyvsp, yyn);
                    }
                    match yyn {
                        2 => {
                            if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null()
                                && (*(*expr.program).disc).flags
                                    & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong
                                    == 0
                            {
                                if !((*expr.program).main.value).is_null()
                                    && (*(*expr.program).disc).flags
                                        & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong
                                        == 0
                                {
                                    exfreenode(expr.program, (*expr.program).main.value);
                                }
                                if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).op
                                    == 312 as libc::c_int
                                {
                                    let mut x: *mut Exnode_t = 0 as *mut Exnode_t;
                                    x = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                    let ref mut fresh78 =
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                    *fresh78 = (*x).data.operand.left;
                                    let ref mut fresh79 = (*x).data.operand.left;
                                    *fresh79 = 0 as *mut Exnode_t;
                                    exfreenode(expr.program, x);
                                }
                                (*expr.program).main.lex = 293 as libc::c_int as libc::c_long;
                                let ref mut fresh80 = (*expr.program).main.value;
                                *fresh80 = exnewnode(
                                    expr.program,
                                    293 as libc::c_int,
                                    1 as libc::c_int,
                                    (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0,
                                    0 as *mut Exnode_t,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        5 => {
                            let mut disc: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
                            if !(expr.procedure).is_null() {
                                exerror(
                                    b"no nested function definitions\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id).lex =
                                293 as libc::c_int as libc::c_long;
                            let ref mut fresh81 =
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id).value;
                            *fresh81 = exnewnode(
                                expr.program,
                                293 as libc::c_int,
                                1 as libc::c_int,
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            expr.procedure = *fresh81;
                            (*expr.procedure).type_0 = 259 as libc::c_int;
                            disc = if 0 as libc::c_int != 0 {
                                realloc(
                                    0 as *mut libc::c_char as *mut libc::c_void,
                                    (::std::mem::size_of::<Dtdisc_t>() as libc::c_ulong)
                                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut Dtdisc_t
                            } else {
                                calloc(
                                    1 as libc::c_int as libc::c_ulong,
                                    (::std::mem::size_of::<Dtdisc_t>() as libc::c_ulong)
                                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut Dtdisc_t
                            };
                            if disc.is_null() {
                                exnospace();
                            }
                            (*disc).key = 88 as libc::c_ulong as libc::c_int;
                            if expr.assigned != 0
                                && strcmp(
                                    ((*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id).name)
                                        .as_mut_ptr(),
                                    b"begin\0" as *const u8 as *const libc::c_char,
                                ) != 0
                            {
                                let ref mut fresh82 = (*expr.procedure).data.procedure.frame;
                                *fresh82 = dtopen(disc, Dtset);
                                if (*fresh82).is_null()
                                    || (dtview(
                                        (*expr.procedure).data.procedure.frame,
                                        (*expr.program).symbols,
                                    ))
                                    .is_null()
                                {
                                    exnospace();
                                }
                                let ref mut fresh83 = (*expr.program).frame;
                                *fresh83 = (*expr.procedure).data.procedure.frame;
                                let ref mut fresh84 = (*expr.program).symbols;
                                *fresh84 = *fresh83;
                            }
                            current_block = 16782213842982696595;
                        }
                        6 => {
                            expr.procedure = 0 as *mut Exnode_t;
                            if !((*expr.program).frame).is_null() {
                                let ref mut fresh85 = (*expr.program).symbols;
                                *fresh85 = (*(*expr.program).frame).view;
                                dtview((*expr.program).frame, 0 as *mut Dt_t);
                                let ref mut fresh86 = (*expr.program).frame;
                                *fresh86 = 0 as *mut Dt_t;
                            }
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).expr).is_null()
                                && (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).op
                                    == 312 as libc::c_int
                            {
                                let mut x_0: *mut Exnode_t = 0 as *mut Exnode_t;
                                x_0 = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                let ref mut fresh87 =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                *fresh87 = (*x_0).data.operand.left;
                                let ref mut fresh88 = (*x_0).data.operand.left;
                                *fresh88 = 0 as *mut Exnode_t;
                                exfreenode(expr.program, x_0);
                            }
                            let ref mut fresh89 =
                                (*(*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).value)
                                    .data
                                    .operand
                                    .right;
                            *fresh89 = excast(
                                expr.program,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as libc::c_int,
                            );
                            current_block = 16782213842982696595;
                        }
                        7 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            current_block = 16782213842982696595;
                        }
                        8 => {
                            if ((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null() {
                                yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            } else if ((*yyvsp.offset(0 as libc::c_int as isize)).expr).is_null() {
                                yyval.expr = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                            } else if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).op
                                == 271 as libc::c_int
                            {
                                exfreenode(
                                    expr.program,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                );
                                yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            } else {
                                yyval.expr = exnewnode(
                                    expr.program,
                                    ';' as i32,
                                    1 as libc::c_int,
                                    (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        9 => {
                            yyval.expr = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        10 => {
                            yyval.expr = if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                .is_null()
                                && (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                    == 263 as libc::c_int
                            {
                                exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    0 as *mut Exnode_t,
                                )
                            } else {
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr
                            };
                            current_block = 16782213842982696595;
                        }
                        11 => {
                            expr.instatic =
                                (*yyvsp.offset(0 as libc::c_int as isize)).integer as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        12 => {
                            expr.declare = (*(*yyvsp.offset(0 as libc::c_int as isize)).id).type_0
                                as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        13 => {
                            yyval.expr = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                            expr.declare = 0 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        14 => {
                            if exisAssign((*yyvsp.offset(-(3 as libc::c_int) as isize)).expr) != 0 {
                                exwarn(
                                    b"assignment used as boolean in if statement\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                let ref mut fresh90 =
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr;
                                *fresh90 = exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr,
                                    0 as *mut Exnode_t,
                                );
                            } else if !((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr)
                                .type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh91 =
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr;
                                *fresh91 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                (*(*yyvsp.offset(-(5 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                1 as libc::c_int,
                                259 as libc::c_int,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr,
                                exnewnode(
                                    expr.program,
                                    ':' as i32,
                                    1 as libc::c_int,
                                    if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                        .is_null()
                                    {
                                        (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                    } else {
                                        0 as libc::c_int
                                    },
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                ),
                            );
                            current_block = 16782213842982696595;
                        }
                        15 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                281 as libc::c_int,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh92 = (*yyval.expr).data.generate.array;
                            *fresh92 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                            if ((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                .data
                                .variable
                                .index)
                                .is_null()
                                || (*(*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .data
                                    .variable
                                    .index)
                                    .op
                                    != 275 as libc::c_int
                            {
                                exerror(
                                    b"simple index variable expected\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            let ref mut fresh93 = (*yyval.expr).data.generate.index;
                            *fresh93 = (*(*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                .data
                                .variable
                                .index)
                                .data
                                .variable
                                .symbol;
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).op
                                == 283 as libc::c_int
                                && (*(*yyval.expr).data.generate.index).type_0
                                    != 259 as libc::c_int as libc::c_long
                            {
                                exerror(
                                    b"integer index variable expected\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            exfreenode(
                                expr.program,
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .data
                                    .variable
                                    .index,
                            );
                            let ref mut fresh94 = (*(*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .expr)
                                .data
                                .variable
                                .index;
                            *fresh94 = 0 as *mut Exnode_t;
                            let ref mut fresh95 = (*yyval.expr).data.generate.statement;
                            *fresh95 = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        16 => {
                            if ((*yyvsp.offset(-(4 as libc::c_int) as isize)).expr).is_null() {
                                let ref mut fresh96 =
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr;
                                *fresh96 = exnewnode(
                                    expr.program,
                                    271 as libc::c_int,
                                    0 as libc::c_int,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as *mut Exnode_t,
                                );
                                (*(*yyvsp.offset(-(4 as libc::c_int) as isize)).expr)
                                    .data
                                    .constant
                                    .value
                                    .integer = 1 as libc::c_int as libc::c_longlong;
                            } else if (*(*yyvsp.offset(-(4 as libc::c_int) as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                let ref mut fresh97 =
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr;
                                *fresh97 = exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr,
                                    0 as *mut Exnode_t,
                                );
                            } else if !((*(*yyvsp.offset(-(4 as libc::c_int) as isize)).expr)
                                .type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(-(4 as libc::c_int) as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh98 =
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr;
                                *fresh98 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                (*(*yyvsp.offset(-(8 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                1 as libc::c_int,
                                259 as libc::c_int,
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).expr,
                                exnewnode(
                                    expr.program,
                                    ';' as i32,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                ),
                            );
                            if !((*yyvsp.offset(-(6 as libc::c_int) as isize)).expr).is_null() {
                                yyval.expr = exnewnode(
                                    expr.program,
                                    ';' as i32,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr,
                                    yyval.expr,
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        17 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                282 as libc::c_int,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh99 = (*yyval.expr).data.generate.array;
                            *fresh99 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                            if ((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                .data
                                .variable
                                .index)
                                .is_null()
                                || (*(*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .data
                                    .variable
                                    .index)
                                    .op
                                    != 275 as libc::c_int
                            {
                                exerror(
                                    b"simple index variable expected\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            let ref mut fresh100 = (*yyval.expr).data.generate.index;
                            *fresh100 = (*(*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                .data
                                .variable
                                .index)
                                .data
                                .variable
                                .symbol;
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).op
                                == 283 as libc::c_int
                                && (*(*yyval.expr).data.generate.index).type_0
                                    != 259 as libc::c_int as libc::c_long
                            {
                                exerror(
                                    b"integer index variable expected\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            exfreenode(
                                expr.program,
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .data
                                    .variable
                                    .index,
                            );
                            let ref mut fresh101 = (*(*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .expr)
                                .data
                                .variable
                                .index;
                            *fresh101 = 0 as *mut Exnode_t;
                            let ref mut fresh102 = (*yyval.expr).data.generate.statement;
                            *fresh102 = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        18 => {
                            if ((*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id)
                                .local
                                .pointer)
                                .is_null()
                            {
                                exerror(
                                    b"cannot apply unset to non-array %s\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id).name)
                                        .as_mut_ptr(),
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                306 as libc::c_int,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh103 = (*yyval.expr).data.variable.symbol;
                            *fresh103 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).id;
                            let ref mut fresh104 = (*yyval.expr).data.variable.index;
                            *fresh104 = 0 as *mut Exnode_t;
                            current_block = 16782213842982696595;
                        }
                        19 => {
                            if ((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                .local
                                .pointer)
                                .is_null()
                            {
                                exerror(
                                    b"cannot apply unset to non-array %s\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).name)
                                        .as_mut_ptr(),
                                );
                            }
                            if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).index_type
                                > 0 as libc::c_int as libc::c_long
                                && (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                    as libc::c_long
                                    != (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                        .index_type
                            {
                                exerror(
                                    b"%s indices must have type %s, not %s\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).name)
                                        .as_mut_ptr(),
                                    extypename(
                                        expr.program,
                                        (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                            .index_type
                                            as libc::c_int,
                                    ),
                                    extypename(
                                        expr.program,
                                        (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                            .type_0,
                                    ),
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                306 as libc::c_int,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh105 = (*yyval.expr).data.variable.symbol;
                            *fresh105 = (*yyvsp.offset(-(3 as libc::c_int) as isize)).id;
                            let ref mut fresh106 = (*yyval.expr).data.variable.index;
                            *fresh106 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        20 => {
                            if exisAssign((*yyvsp.offset(-(2 as libc::c_int) as isize)).expr) != 0 {
                                exwarn(
                                    b"assignment used as boolean in while statement\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                let ref mut fresh107 =
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                                *fresh107 = exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                    0 as *mut Exnode_t,
                                );
                            } else if !((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                .type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh108 =
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                                *fresh108 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                (*(*yyvsp.offset(-(4 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                1 as libc::c_int,
                                259 as libc::c_int,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                exnewnode(
                                    expr.program,
                                    ';' as i32,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                ),
                            );
                            current_block = 16782213842982696595;
                        }
                        21 => {
                            expr.declare =
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0;
                            current_block = 16782213842982696595;
                        }
                        22 => {
                            let mut sw: *mut Switch_t = expr.swstate;
                            yyval.expr = exnewnode(
                                expr.program,
                                (*(*yyvsp.offset(-(7 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                1 as libc::c_int,
                                259 as libc::c_int,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).expr,
                                exnewnode(
                                    expr.program,
                                    274 as libc::c_int,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    (*sw).defcase,
                                    (*sw).firstcase,
                                ),
                            );
                            expr.swstate = (*expr.swstate).prev;
                            free((*sw).base as *mut libc::c_void);
                            if sw != &mut swstate as *mut Switch_t {
                                free(sw as *mut libc::c_void);
                            }
                            expr.declare = 0 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        23 | 24 => {
                            if ((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null() {
                                let ref mut fresh109 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh109 = exnewnode(
                                    expr.program,
                                    271 as libc::c_int,
                                    0 as libc::c_int,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as *mut Exnode_t,
                                );
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .constant
                                    .value
                                    .integer = 1 as libc::c_int as libc::c_longlong;
                            } else if !((*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                .type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh110 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh110 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                1 as libc::c_int,
                                259 as libc::c_int,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        25 => {
                            if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null() {
                                if !(expr.procedure).is_null() && (*expr.procedure).type_0 == 0 {
                                    exerror(
                                        b"return in void function\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                let ref mut fresh111 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh111 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    if !(expr.procedure).is_null() {
                                        (*expr.procedure).type_0
                                    } else {
                                        259 as libc::c_int
                                    },
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                296 as libc::c_int,
                                1 as libc::c_int,
                                if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null() {
                                    (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                } else {
                                    0 as libc::c_int
                                },
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        26 => {
                            let mut sw_0: *mut Switch_t = 0 as *mut Switch_t;
                            let mut n: libc::c_int = 0;
                            if !(expr.swstate).is_null() {
                                sw_0 = if 0 as libc::c_int != 0 {
                                    realloc(
                                        0 as *mut libc::c_char as *mut libc::c_void,
                                        (::std::mem::size_of::<Switch_t>() as libc::c_ulong)
                                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                    ) as *mut Switch_t
                                } else {
                                    calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        (::std::mem::size_of::<Switch_t>() as libc::c_ulong)
                                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                    ) as *mut Switch_t
                                };
                                if sw_0.is_null() {
                                    exnospace();
                                    sw_0 = &mut swstate;
                                }
                                let ref mut fresh112 = (*sw_0).prev;
                                *fresh112 = expr.swstate;
                            } else {
                                sw_0 = &mut swstate;
                            }
                            expr.swstate = sw_0;
                            (*sw_0).type_0 = expr.declare;
                            let ref mut fresh113 = (*sw_0).firstcase;
                            *fresh113 = 0 as *mut Exnode_t;
                            let ref mut fresh114 = (*sw_0).lastcase;
                            *fresh114 = 0 as *mut Exnode_t;
                            let ref mut fresh115 = (*sw_0).defcase;
                            *fresh115 = 0 as *mut Exnode_t;
                            (*sw_0).def = 0 as libc::c_int;
                            n = 8 as libc::c_int;
                            let ref mut fresh116 = (*sw_0).base;
                            *fresh116 = if 0 as libc::c_int != 0 {
                                realloc(
                                    0 as *mut libc::c_char as *mut libc::c_void,
                                    (::std::mem::size_of::<*mut Extype_t>() as libc::c_ulong)
                                        .wrapping_mul(n as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut *mut Extype_t
                            } else {
                                calloc(
                                    1 as libc::c_int as libc::c_ulong,
                                    (::std::mem::size_of::<*mut Extype_t>() as libc::c_ulong)
                                        .wrapping_mul(n as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut *mut Extype_t
                            };
                            if (*fresh116).is_null() {
                                exnospace();
                                n = 0 as libc::c_int;
                            }
                            let ref mut fresh117 = (*sw_0).cur;
                            *fresh117 = (*sw_0).base;
                            let ref mut fresh118 = (*sw_0).last;
                            *fresh118 = ((*sw_0).base).offset(n as isize);
                            current_block = 16782213842982696595;
                        }
                        28 => {
                            let mut sw_1: *mut Switch_t = expr.swstate;
                            let mut n_0: libc::c_int = 0;
                            yyval.expr = exnewnode(
                                expr.program,
                                270 as libc::c_int,
                                1 as libc::c_int,
                                0 as libc::c_int,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            if (*sw_1).cur > (*sw_1).base {
                                if !((*sw_1).lastcase).is_null() {
                                    let ref mut fresh119 = (*(*sw_1).lastcase).data.select.next;
                                    *fresh119 = yyval.expr;
                                } else {
                                    let ref mut fresh120 = (*sw_1).firstcase;
                                    *fresh120 = yyval.expr;
                                }
                                let ref mut fresh121 = (*sw_1).lastcase;
                                *fresh121 = yyval.expr;
                                n_0 = ((*sw_1).cur).offset_from((*sw_1).base) as libc::c_long
                                    as libc::c_int;
                                let ref mut fresh122 = (*sw_1).cur;
                                *fresh122 = (*sw_1).base;
                                let ref mut fresh123 = (*yyval.expr).data.select.constant;
                                *fresh123 = vmalloc(
                                    (*expr.program).vm,
                                    ((n_0 + 1 as libc::c_int) as libc::c_ulong).wrapping_mul(
                                        ::std::mem::size_of::<*mut Extype_t>() as libc::c_ulong,
                                    ),
                                ) as *mut *mut Extype_t;
                                memcpy(
                                    (*yyval.expr).data.select.constant as *mut libc::c_void,
                                    (*sw_1).base as *const libc::c_void,
                                    (n_0 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                                        *mut Extype_t,
                                    >(
                                    )
                                        as libc::c_ulong),
                                );
                                let ref mut fresh124 =
                                    *((*yyval.expr).data.select.constant).offset(n_0 as isize);
                                *fresh124 = 0 as *mut Extype_t;
                            } else {
                                let ref mut fresh125 = (*yyval.expr).data.select.constant;
                                *fresh125 = 0 as *mut *mut Extype_t;
                            }
                            if (*sw_1).def != 0 {
                                (*sw_1).def = 0 as libc::c_int;
                                if !((*sw_1).defcase).is_null() {
                                    exerror(
                                        b"duplicate default in switch\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    let ref mut fresh126 = (*sw_1).defcase;
                                    *fresh126 = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                }
                            }
                            current_block = 16782213842982696595;
                        }
                        31 => {
                            let mut n_1: libc::c_int = 0;
                            if (*expr.swstate).cur >= (*expr.swstate).last {
                                n_1 = ((*expr.swstate).cur).offset_from((*expr.swstate).base)
                                    as libc::c_long
                                    as libc::c_int;
                                let ref mut fresh127 = (*expr.swstate).base;
                                *fresh127 = if !((*expr.swstate).base).is_null() {
                                    realloc(
                                        (*expr.swstate).base as *mut libc::c_char
                                            as *mut libc::c_void,
                                        (::std::mem::size_of::<*mut Extype_t>() as libc::c_ulong)
                                            .wrapping_mul((2 as libc::c_int * n_1) as libc::c_ulong)
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                    ) as *mut *mut Extype_t
                                } else {
                                    calloc(
                                        1 as libc::c_int as libc::c_ulong,
                                        (::std::mem::size_of::<*mut Extype_t>() as libc::c_ulong)
                                            .wrapping_mul((2 as libc::c_int * n_1) as libc::c_ulong)
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                    ) as *mut *mut Extype_t
                                };
                                if (*fresh127).is_null() {
                                    exerror(
                                        b"too many case labels for switch\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    n_1 = 0 as libc::c_int;
                                }
                                let ref mut fresh128 = (*expr.swstate).cur;
                                *fresh128 = ((*expr.swstate).base).offset(n_1 as isize);
                                let ref mut fresh129 = (*expr.swstate).last;
                                *fresh129 = ((*expr.swstate).base)
                                    .offset((2 as libc::c_int * n_1) as isize);
                            }
                            if !((*expr.swstate).cur).is_null() {
                                let ref mut fresh130 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh130 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    (*expr.swstate).type_0,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                                let ref mut fresh131 = (*expr.swstate).cur;
                                let fresh132 = *fresh131;
                                *fresh131 = (*fresh131).offset(1);
                                *fresh132 = &mut (*(*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .expr)
                                    .data
                                    .constant
                                    .value;
                            }
                            current_block = 16782213842982696595;
                        }
                        32 => {
                            (*expr.swstate).def = 1 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        33 => {
                            yyval.integer = 0 as libc::c_int as libc::c_longlong;
                            current_block = 16782213842982696595;
                        }
                        34 => {
                            yyval.integer = 1 as libc::c_int as libc::c_longlong;
                            current_block = 16782213842982696595;
                        }
                        36 => {
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).expr).is_null() {
                                yyval.expr = if !((*yyvsp.offset(-(2 as libc::c_int) as isize))
                                    .expr)
                                    .is_null()
                                {
                                    exnewnode(
                                        expr.program,
                                        ',' as i32,
                                        1 as libc::c_int,
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    )
                                } else {
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr
                                };
                            }
                            current_block = 16782213842982696595;
                        }
                        37 => {
                            checkName((*yyvsp.offset(0 as libc::c_int as isize)).id);
                            expr.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            current_block = 16782213842982696595;
                        }
                        38 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0 == 0
                                || expr.declare != 0
                            {
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0 =
                                    expr.declare as libc::c_long;
                            }
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).expr).is_null()
                                && (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).op
                                    == 293 as libc::c_int
                            {
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).lex =
                                    293 as libc::c_int as libc::c_long;
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0 =
                                    (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                        as libc::c_long;
                                let ref mut fresh133 =
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).value;
                                *fresh133 = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            } else {
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).lex =
                                    275 as libc::c_int as libc::c_long;
                                let ref mut fresh134 =
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).value;
                                *fresh134 = exnewnode(
                                    expr.program,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as *mut Exnode_t,
                                );
                                if (*yyvsp.offset(-(1 as libc::c_int) as isize)).integer != 0
                                    && ((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                        .local
                                        .pointer)
                                        .is_null()
                                {
                                    let mut disc_0: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
                                    disc_0 = if 0 as libc::c_int != 0 {
                                        realloc(
                                            0 as *mut libc::c_char as *mut libc::c_void,
                                            (::std::mem::size_of::<Dtdisc_t>() as libc::c_ulong)
                                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                        ) as *mut Dtdisc_t
                                    } else {
                                        calloc(
                                            1 as libc::c_int as libc::c_ulong,
                                            (::std::mem::size_of::<Dtdisc_t>() as libc::c_ulong)
                                                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                        ) as *mut Dtdisc_t
                                    };
                                    if disc_0.is_null() {
                                        exnospace();
                                    }
                                    if (*yyvsp.offset(-(1 as libc::c_int) as isize)).integer
                                        == 259 as libc::c_int as libc::c_longlong
                                    {
                                        (*disc_0).key = 16 as libc::c_ulong as libc::c_int;
                                        (*disc_0).size = ::std::mem::size_of::<Extype_t>()
                                            as libc::c_ulong
                                            as libc::c_int;
                                        let ref mut fresh135 = (*disc_0).comparf;
                                        *fresh135 = ::std::mem::transmute::<
                                            Option<
                                                unsafe extern "C" fn(
                                                    *mut Dt_t,
                                                    *mut Extype_t,
                                                    *mut Extype_t,
                                                    *mut Dtdisc_t,
                                                )
                                                    -> libc::c_int,
                                            >,
                                            Dtcompar_f,
                                        >(Some(
                                            cmpKey
                                                as unsafe extern "C" fn(
                                                    *mut Dt_t,
                                                    *mut Extype_t,
                                                    *mut Extype_t,
                                                    *mut Dtdisc_t,
                                                )
                                                    -> libc::c_int,
                                        ));
                                    } else {
                                        (*disc_0).key = 32 as libc::c_ulong as libc::c_int;
                                    }
                                    let ref mut fresh136 =
                                        (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                            .local
                                            .pointer;
                                    *fresh136 = dtopen(disc_0, Dtoset) as *mut libc::c_char;
                                    if (*fresh136).is_null() {
                                        exerror(
                                            b"%s: cannot initialize associative array\0"
                                                as *const u8
                                                as *const libc::c_char,
                                            ((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                                .name)
                                                .as_mut_ptr(),
                                        );
                                    }
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                        .index_type = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .integer
                                        as libc::c_long;
                                }
                                if !((*yyvsp.offset(0 as libc::c_int as isize)).expr).is_null() {
                                    if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                        as libc::c_long
                                        != (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                            .type_0
                                    {
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 =
                                            (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                                .type_0
                                                as libc::c_int;
                                        let ref mut fresh137 =
                                            (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                                .data
                                                .operand
                                                .right;
                                        *fresh137 = excast(
                                            expr.program,
                                            (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                                .data
                                                .operand
                                                .right,
                                            (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id)
                                                .type_0
                                                as libc::c_int,
                                            0 as *mut Exnode_t,
                                            0 as libc::c_int,
                                        );
                                    }
                                    let ref mut fresh138 =
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                            .data
                                            .operand
                                            .left;
                                    *fresh138 = exnewnode(
                                        expr.program,
                                        275 as libc::c_int,
                                        0 as libc::c_int,
                                        (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                            as libc::c_int,
                                        0 as *mut Exnode_t,
                                        0 as *mut Exnode_t,
                                    );
                                    let ref mut fresh139 =
                                        (*(*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                            .data
                                            .operand
                                            .left)
                                            .data
                                            .variable
                                            .symbol;
                                    *fresh139 = (*yyvsp.offset(-(3 as libc::c_int) as isize)).id;
                                    yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                } else if (*yyvsp.offset(-(1 as libc::c_int) as isize)).integer == 0
                                {
                                    (*(*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).value)
                                        .data
                                        .value = exzero(
                                        (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                            as libc::c_int,
                                    );
                                }
                            }
                            current_block = 16782213842982696595;
                        }
                        45 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            current_block = 16782213842982696595;
                        }
                        46 => {
                            yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        47 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            current_block = 16782213842982696595;
                        }
                        49 => {
                            yyval.expr = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        50 => {
                            yyval.expr = if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                .type_0 as libc::c_long
                                == (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).type_0
                            {
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr
                            } else {
                                excast(
                                    expr.program,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).type_0
                                        as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                )
                            };
                            current_block = 16782213842982696595;
                        }
                        51 => {
                            rel = 0;
                            current_block = 3290856744090149140;
                        }
                        58 | 59 | 60 | 61 | 62 => {
                            current_block = 3290856744090149140;
                        }
                        52 | 53 | 54 | 55 | 56 | 57 | 63 | 64 | 65 | 66 => {
                            current_block = 720742499249991355;
                        }
                        67 | 68 => {
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                let ref mut fresh146 =
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                                *fresh146 = exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                    0 as *mut Exnode_t,
                                );
                            } else if !((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                .type_0
                                > 258 as libc::c_int)
                            {
                                let ref mut fresh147 =
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                                *fresh147 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                let ref mut fresh148 =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                *fresh148 = exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    0 as *mut Exnode_t,
                                );
                            } else if !((*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                > 258 as libc::c_int)
                            {
                                let ref mut fresh149 =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                *fresh149 = excast(
                                    expr.program,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            current_block = 720742499249991355;
                        }
                        69 => {
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).op
                                == 271 as libc::c_int
                            {
                                exfreenode(
                                    expr.program,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                );
                                yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            } else {
                                yyval.expr = exnewnode(
                                    expr.program,
                                    ',' as i32,
                                    1 as libc::c_int,
                                    (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        70 => {
                            expr.nolabel = 1 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        71 => {
                            expr.nolabel = 0 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        72 => {
                            if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0 == 0 {
                                if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 == 0 {
                                    let ref mut fresh150 =
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0;
                                    *fresh150 = 259 as libc::c_int;
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0 =
                                        *fresh150;
                                } else {
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0 =
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0;
                                }
                            } else if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 == 0
                            {
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 =
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0;
                            }
                            if (*(*yyvsp.offset(-(6 as libc::c_int) as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                let ref mut fresh151 =
                                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr;
                                *fresh151 = exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr,
                                    0 as *mut Exnode_t,
                                );
                            } else if !((*(*yyvsp.offset(-(6 as libc::c_int) as isize)).expr)
                                .type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(-(6 as libc::c_int) as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh152 =
                                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr;
                                *fresh152 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0
                                != (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                            {
                                if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0
                                    == 263 as libc::c_int
                                    || (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                        == 263 as libc::c_int
                                {
                                    exerror(
                                        b"if statement string type mismatch\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr)
                                    .type_0
                                    == 262 as libc::c_int
                                {
                                    let ref mut fresh153 =
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                    *fresh153 = excast(
                                        expr.program,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                        262 as libc::c_int,
                                        0 as *mut Exnode_t,
                                        0 as libc::c_int,
                                    );
                                } else if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                    == 262 as libc::c_int
                                {
                                    let ref mut fresh154 =
                                        (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr;
                                    *fresh154 = excast(
                                        expr.program,
                                        (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr,
                                        262 as libc::c_int,
                                        0 as *mut Exnode_t,
                                        0 as libc::c_int,
                                    );
                                }
                            }
                            if (*(*yyvsp.offset(-(6 as libc::c_int) as isize)).expr).op
                                == 271 as libc::c_int
                            {
                                if (*(*yyvsp.offset(-(6 as libc::c_int) as isize)).expr)
                                    .data
                                    .constant
                                    .value
                                    .integer
                                    != 0
                                {
                                    yyval.expr = (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr;
                                    exfreenode(
                                        expr.program,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    );
                                } else {
                                    yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                    exfreenode(
                                        expr.program,
                                        (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr,
                                    );
                                }
                                exfreenode(
                                    expr.program,
                                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr,
                                );
                            } else {
                                yyval.expr = exnewnode(
                                    expr.program,
                                    '?' as i32,
                                    1 as libc::c_int,
                                    (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr).type_0,
                                    (*yyvsp.offset(-(6 as libc::c_int) as isize)).expr,
                                    exnewnode(
                                        expr.program,
                                        ':' as i32,
                                        1 as libc::c_int,
                                        (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).expr)
                                            .type_0,
                                        (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    ),
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        74 => {
                            if ((*(*yyvsp.offset(0 as libc::c_int as isize)).id)
                                .local
                                .pointer)
                                .is_null()
                            {
                                exerror(
                                    b"cannot apply '#' operator to non-array %s\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(0 as libc::c_int as isize)).id).name)
                                        .as_mut_ptr(),
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                '#' as i32,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh157 = (*yyval.expr).data.variable.symbol;
                            *fresh157 = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            current_block = 16782213842982696595;
                        }
                        73 | 75 => {
                            if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                let ref mut fresh155 =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                *fresh155 = exnewnode(
                                    expr.program,
                                    312 as libc::c_int,
                                    1 as libc::c_int,
                                    259 as libc::c_int,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    0 as *mut Exnode_t,
                                );
                            } else if !((*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh156 =
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                *fresh156 = excast(
                                    expr.program,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            current_block = 17643436426743203513;
                        }
                        76 => {
                            current_block = 17643436426743203513;
                        }
                        77 => {
                            yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        78 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                266 as libc::c_int,
                                0 as libc::c_int,
                                T((*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0),
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        79 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                267 as libc::c_int,
                                1 as libc::c_int,
                                T((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int),
                                call(
                                    0 as *mut Exref_t,
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                            );
                            current_block = 16782213842982696595;
                        }
                        80 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                279 as libc::c_int,
                                1 as libc::c_int,
                                T((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int),
                                call(
                                    0 as *mut Exref_t,
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                ),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                            );
                            current_block = 16782213842982696595;
                        }
                        81 => {
                            yyval.expr = exnewsub(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                280 as libc::c_int,
                            );
                            current_block = 16782213842982696595;
                        }
                        82 => {
                            yyval.expr = exnewsub(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                302 as libc::c_int,
                            );
                            current_block = 16782213842982696595;
                        }
                        83 => {
                            yyval.expr = exnewsubstr(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                            );
                            current_block = 16782213842982696595;
                        }
                        84 => {
                            yyval.expr = exnewsplit(
                                expr.program,
                                (*(*yyvsp.offset(-(5 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        85 => {
                            yyval.expr = exnewsplit(
                                expr.program,
                                (*(*yyvsp.offset(-(7 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).expr,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                            );
                            current_block = 16782213842982696595;
                        }
                        86 => {
                            if !((*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh158 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh158 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                277 as libc::c_int,
                                1 as libc::c_int,
                                259 as libc::c_int,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        87 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                295 as libc::c_int,
                                0 as libc::c_int,
                                262 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        88 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                300 as libc::c_int,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        89 => {
                            if !((*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                >= 259 as libc::c_int
                                && (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                    <= 261 as libc::c_int)
                            {
                                let ref mut fresh159 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh159 = excast(
                                    expr.program,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    259 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as libc::c_int,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                300 as libc::c_int,
                                1 as libc::c_int,
                                259 as libc::c_int,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        90 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                269 as libc::c_int,
                                1 as libc::c_int,
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                            );
                            let ref mut fresh160 = (*yyval.expr).data.call.procedure;
                            *fresh160 = (*yyvsp.offset(-(3 as libc::c_int) as isize)).id;
                            current_block = 16782213842982696595;
                        }
                        91 => {
                            yyval.expr = exprint(
                                expr.program,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).id,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                            );
                            current_block = 16782213842982696595;
                        }
                        92 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                0 as libc::c_int,
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null()
                                && (*(*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .left)
                                    .type_0
                                    == 259 as libc::c_int
                            {
                                let ref mut fresh161 = (*yyval.expr).data.print.descriptor;
                                *fresh161 = (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .left;
                                let ref mut fresh162 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh162 = (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .right;
                            } else {
                                match (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).index {
                                    294 => {
                                        let ref mut fresh163 = (*yyval.expr).data.print.descriptor;
                                        *fresh163 = exnewnode(
                                            expr.program,
                                            271 as libc::c_int,
                                            0 as libc::c_int,
                                            259 as libc::c_int,
                                            0 as *mut Exnode_t,
                                            0 as *mut Exnode_t,
                                        );
                                        (*(*yyval.expr).data.print.descriptor)
                                            .data
                                            .constant
                                            .value
                                            .integer = 2 as libc::c_int as libc::c_longlong;
                                    }
                                    292 => {
                                        let ref mut fresh164 = (*yyval.expr).data.print.descriptor;
                                        *fresh164 = exnewnode(
                                            expr.program,
                                            271 as libc::c_int,
                                            0 as libc::c_int,
                                            259 as libc::c_int,
                                            0 as *mut Exnode_t,
                                            0 as *mut Exnode_t,
                                        );
                                        (*(*yyval.expr).data.print.descriptor)
                                            .data
                                            .constant
                                            .value
                                            .integer = 1 as libc::c_int as libc::c_longlong;
                                    }
                                    299 => {
                                        let ref mut fresh165 = (*yyval.expr).data.print.descriptor;
                                        *fresh165 = 0 as *mut Exnode_t;
                                    }
                                    _ => {}
                                }
                            }
                            let ref mut fresh166 = (*yyval.expr).data.print.args;
                            *fresh166 =
                                preprint((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr);
                            current_block = 16782213842982696595;
                        }
                        93 => {
                            let mut x_1: *mut Exnode_t = 0 as *mut Exnode_t;
                            yyval.expr = exnewnode(
                                expr.program,
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).index
                                    as libc::c_int,
                                0 as libc::c_int,
                                (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null()
                                && (*(*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .left)
                                    .type_0
                                    == 259 as libc::c_int
                            {
                                let ref mut fresh167 = (*yyval.expr).data.scan.descriptor;
                                *fresh167 = (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .left;
                                let ref mut fresh168 =
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                *fresh168 = (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .right;
                            } else {
                                match (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).index {
                                    297 => {
                                        let ref mut fresh169 = (*yyval.expr).data.scan.descriptor;
                                        *fresh169 = 0 as *mut Exnode_t;
                                    }
                                    301 => {
                                        if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                            .is_null()
                                            && (*(*(*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                .expr)
                                                .data
                                                .operand
                                                .left)
                                                .type_0
                                                == 263 as libc::c_int
                                        {
                                            let ref mut fresh170 =
                                                (*yyval.expr).data.scan.descriptor;
                                            *fresh170 = (*(*yyvsp
                                                .offset(-(1 as libc::c_int) as isize))
                                            .expr)
                                                .data
                                                .operand
                                                .left;
                                            let ref mut fresh171 =
                                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                            *fresh171 = (*(*yyvsp
                                                .offset(-(1 as libc::c_int) as isize))
                                            .expr)
                                                .data
                                                .operand
                                                .right;
                                        } else {
                                            exerror(
                                                b"%s: string argument expected\0" as *const u8
                                                    as *const libc::c_char,
                                                ((*(*yyvsp.offset(-(3 as libc::c_int) as isize))
                                                    .id)
                                                    .name)
                                                    .as_mut_ptr(),
                                            );
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            if ((*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).is_null()
                                || ((*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .left)
                                    .is_null()
                                || (*(*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .left)
                                    .type_0
                                    != 263 as libc::c_int
                            {
                                exerror(
                                    b"%s: format argument expected\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).name)
                                        .as_mut_ptr(),
                                );
                            }
                            let ref mut fresh172 = (*yyval.expr).data.scan.format;
                            *fresh172 = (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                .data
                                .operand
                                .left;
                            let ref mut fresh173 = (*yyval.expr).data.scan.args;
                            *fresh173 = (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                .data
                                .operand
                                .right;
                            x_1 = *fresh173;
                            while !x_1.is_null() {
                                if (*(*x_1).data.operand.left).op != 266 as libc::c_int {
                                    exerror(
                                        b"%s: address argument expected\0" as *const u8
                                            as *const libc::c_char,
                                        ((*(*yyvsp.offset(-(3 as libc::c_int) as isize)).id).name)
                                            .as_mut_ptr(),
                                    );
                                }
                                let ref mut fresh174 = (*x_1).data.operand.left;
                                *fresh174 = (*(*x_1).data.operand.left).data.operand.left;
                                x_1 = (*x_1).data.operand.right;
                            }
                            current_block = 16782213842982696595;
                        }
                        94 => {
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).expr).is_null() {
                                if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).op
                                    == 283 as libc::c_int
                                    && ((*(*expr.program).disc).setf).is_none()
                                {
                                    exerror(
                                        b"%s: variable assignment not supported\0" as *const u8
                                            as *const libc::c_char,
                                        ((*(*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                            .data
                                            .variable
                                            .symbol)
                                            .name)
                                            .as_mut_ptr(),
                                    );
                                } else {
                                    if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                        == 0
                                    {
                                        (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                            .type_0 = (*(*yyvsp.offset(0 as libc::c_int as isize))
                                            .expr)
                                            .type_0;
                                    } else if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                        .type_0
                                        != (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                            .type_0
                                    {
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 =
                                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                                .type_0;
                                        let ref mut fresh175 =
                                            (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                                .data
                                                .operand
                                                .right;
                                        *fresh175 = excast(
                                            expr.program,
                                            (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                                .data
                                                .operand
                                                .right,
                                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                                .type_0,
                                            0 as *mut Exnode_t,
                                            0 as libc::c_int,
                                        );
                                    }
                                    let ref mut fresh176 =
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                            .data
                                            .operand
                                            .left;
                                    *fresh176 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                                    yyval.expr = (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                }
                            }
                            current_block = 16782213842982696595;
                        }
                        97 => {
                            if ((*(*yyvsp.offset(0 as libc::c_int as isize)).id)
                                .local
                                .pointer)
                                .is_null()
                            {
                                exerror(
                                    b"cannot apply IN to non-array %s\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(0 as libc::c_int as isize)).id).name)
                                        .as_mut_ptr(),
                                );
                            }
                            if (*(*yyvsp.offset(0 as libc::c_int as isize)).id).index_type
                                > 0 as libc::c_int as libc::c_long
                                && (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0
                                    as libc::c_long
                                    != (*(*yyvsp.offset(0 as libc::c_int as isize)).id).index_type
                            {
                                exerror(
                                    b"%s indices must have type %s, not %s\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(0 as libc::c_int as isize)).id).name)
                                        .as_mut_ptr(),
                                    extypename(
                                        expr.program,
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).id).index_type
                                            as libc::c_int,
                                    ),
                                    extypename(
                                        expr.program,
                                        (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                            .type_0,
                                    ),
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                331 as libc::c_int,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh177 = (*yyval.expr).data.variable.symbol;
                            *fresh177 = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            let ref mut fresh178 = (*yyval.expr).data.variable.index;
                            *fresh178 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        95 | 98 => {
                            if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                exerror(
                                    b"++ and -- invalid for string variables\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).op,
                                0 as libc::c_int,
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            (*yyval.expr).subop = 290 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        96 | 99 => {
                            if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                == 263 as libc::c_int
                            {
                                exerror(
                                    b"++ and -- invalid for string variables\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                (*yyvsp.offset(0 as libc::c_int as isize)).op,
                                0 as libc::c_int,
                                (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            (*yyval.expr).subop = 288 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        103 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                271 as libc::c_int,
                                0 as libc::c_int,
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            if ((*(*expr.program).disc).reff).is_none() {
                                exerror(
                                    b"%s: identifier references not supported\0" as *const u8
                                        as *const libc::c_char,
                                    ((*(*yyvsp.offset(0 as libc::c_int as isize)).id).name)
                                        .as_mut_ptr(),
                                );
                            } else {
                                (*yyval.expr).data.constant.value = (Some(
                                    ((*(*expr.program).disc).reff)
                                        .expect("non-null function pointer"),
                                ))
                                .expect("non-null function pointer")(
                                    expr.program,
                                    yyval.expr,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).id,
                                    0 as *mut Exref_t,
                                    0 as *mut libc::c_char,
                                    -(1 as libc::c_int),
                                    (*expr.program).disc,
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        104 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                271 as libc::c_int,
                                0 as libc::c_int,
                                262 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            (*yyval.expr).data.constant.value.floating =
                                (*yyvsp.offset(0 as libc::c_int as isize)).floating;
                            current_block = 16782213842982696595;
                        }
                        105 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                271 as libc::c_int,
                                0 as libc::c_int,
                                259 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            (*yyval.expr).data.constant.value.integer =
                                (*yyvsp.offset(0 as libc::c_int as isize)).integer;
                            current_block = 16782213842982696595;
                        }
                        106 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                271 as libc::c_int,
                                0 as libc::c_int,
                                263 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh179 = (*yyval.expr).data.constant.value.string;
                            *fresh179 = (*yyvsp.offset(0 as libc::c_int as isize)).string;
                            current_block = 16782213842982696595;
                        }
                        107 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                271 as libc::c_int,
                                0 as libc::c_int,
                                260 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            (*yyval.expr).data.constant.value.integer =
                                (*yyvsp.offset(0 as libc::c_int as isize)).integer;
                            current_block = 16782213842982696595;
                        }
                        113 => {
                            yyval.expr = makeVar(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).id,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                                (*yyvsp.offset(0 as libc::c_int as isize)).reference,
                            );
                            current_block = 16782213842982696595;
                        }
                        114 => {
                            let mut n_2: *mut Exnode_t = 0 as *mut Exnode_t;
                            n_2 = exnewnode(
                                expr.program,
                                275 as libc::c_int,
                                0 as libc::c_int,
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh180 = (*n_2).data.variable.symbol;
                            *fresh180 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).id;
                            let ref mut fresh181 = (*n_2).data.variable.reference;
                            *fresh181 = 0 as *mut Exref_t;
                            let ref mut fresh182 = (*n_2).data.variable.index;
                            *fresh182 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                            if (*fresh182 == 0 as *mut Exnode_t) as libc::c_int
                                != ((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id)
                                    .local
                                    .pointer
                                    == 0 as *mut libc::c_char)
                                    as libc::c_int
                            {
                                exerror(
                                    b"%s: is%s an array\0" as *const u8 as *const libc::c_char,
                                    ((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).name)
                                        .as_mut_ptr(),
                                    if !((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id)
                                        .local
                                        .pointer)
                                        .is_null()
                                    {
                                        b"\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b" not\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            if !((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id)
                                .local
                                .pointer)
                                .is_null()
                                && (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).index_type
                                    > 0 as libc::c_int as libc::c_long
                            {
                                if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr).type_0
                                    as libc::c_long
                                    != (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id)
                                        .index_type
                                {
                                    exerror(
                                        b"%s: indices must have type %s, not %s\0" as *const u8
                                            as *const libc::c_char,
                                        ((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).name)
                                            .as_mut_ptr(),
                                        extypename(
                                            expr.program,
                                            (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id)
                                                .index_type
                                                as libc::c_int,
                                        ),
                                        extypename(
                                            expr.program,
                                            (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).expr)
                                                .type_0,
                                        ),
                                    );
                                }
                            }
                            if !((*yyvsp.offset(0 as libc::c_int as isize)).reference).is_null() {
                                let ref mut fresh183 = (*n_2).data.variable.dyna;
                                *fresh183 = exnewnode(
                                    expr.program,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    0 as *mut Exnode_t,
                                    0 as *mut Exnode_t,
                                );
                                yyval.expr = makeVar(
                                    expr.program,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).id,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                    n_2,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).reference,
                                );
                            } else {
                                yyval.expr = n_2;
                            }
                            current_block = 16782213842982696595;
                        }
                        115 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                283 as libc::c_int,
                                0 as libc::c_int,
                                263 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh184 = (*yyval.expr).data.variable.symbol;
                            *fresh184 = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            let ref mut fresh185 = (*yyval.expr).data.variable.reference;
                            *fresh185 = 0 as *mut Exref_t;
                            let ref mut fresh186 = (*yyval.expr).data.variable.index;
                            *fresh186 = 0 as *mut Exnode_t;
                            let ref mut fresh187 = (*yyval.expr).data.variable.dyna;
                            *fresh187 = 0 as *mut Exnode_t;
                            if (*(*expr.program).disc).flags
                                & ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong
                                == 0
                            {
                                exerror(
                                    b"unknown identifier\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        116 => {
                            yyval.integer = 0 as libc::c_int as libc::c_longlong;
                            current_block = 16782213842982696595;
                        }
                        117 => {
                            yyval.integer = -(1 as libc::c_int) as libc::c_longlong;
                            current_block = 16782213842982696595;
                        }
                        118 => {
                            if (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id).type_0
                                >= 259 as libc::c_int as libc::c_long
                                && (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id).type_0
                                    <= 261 as libc::c_int as libc::c_long
                            {
                                yyval.integer = 259 as libc::c_int as libc::c_longlong;
                            } else {
                                yyval.integer = (*(*yyvsp.offset(-(1 as libc::c_int) as isize)).id)
                                    .type_0
                                    as libc::c_longlong;
                            }
                            current_block = 16782213842982696595;
                        }
                        119 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            current_block = 16782213842982696595;
                        }
                        120 => {
                            yyval.expr = (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr;
                            current_block = 16782213842982696595;
                        }
                        121 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            current_block = 16782213842982696595;
                        }
                        122 => {
                            yyval.expr = (*(*yyvsp.offset(0 as libc::c_int as isize)).expr)
                                .data
                                .operand
                                .left;
                            let ref mut fresh188 = (*(*yyvsp.offset(0 as libc::c_int as isize))
                                .expr)
                                .data
                                .operand
                                .right;
                            *fresh188 = 0 as *mut Exnode_t;
                            let ref mut fresh189 = (*(*yyvsp.offset(0 as libc::c_int as isize))
                                .expr)
                                .data
                                .operand
                                .left;
                            *fresh189 = *fresh188;
                            exfreenode(
                                expr.program,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                            );
                            current_block = 16782213842982696595;
                        }
                        123 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                ',' as i32,
                                1 as libc::c_int,
                                0 as libc::c_int,
                                exnewnode(
                                    expr.program,
                                    ',' as i32,
                                    1 as libc::c_int,
                                    (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    0 as *mut Exnode_t,
                                ),
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh190 = (*yyval.expr).data.operand.right;
                            *fresh190 = (*yyval.expr).data.operand.left;
                            current_block = 16782213842982696595;
                        }
                        124 => {
                            let ref mut fresh191 =
                                (*(*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .data
                                    .operand
                                    .right)
                                    .data
                                    .operand
                                    .right;
                            *fresh191 = exnewnode(
                                expr.program,
                                ',' as i32,
                                1 as libc::c_int,
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh192 = (*(*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .expr)
                                .data
                                .operand
                                .right;
                            *fresh192 = *fresh191;
                            current_block = 16782213842982696595;
                        }
                        125 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            current_block = 16782213842982696595;
                        }
                        126 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            if (*(*yyvsp.offset(0 as libc::c_int as isize)).id).type_0 != 0 {
                                exerror(b"(void) expected\0" as *const u8 as *const libc::c_char);
                            }
                            current_block = 16782213842982696595;
                        }
                        128 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                ',' as i32,
                                1 as libc::c_int,
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        129 => {
                            let mut x_2: *mut Exnode_t = 0 as *mut Exnode_t;
                            let mut y: *mut Exnode_t = 0 as *mut Exnode_t;
                            yyval.expr = (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                            x_2 = (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                            loop {
                                y = (*x_2).data.operand.right;
                                if y.is_null() {
                                    break;
                                }
                                x_2 = y;
                            }
                            let ref mut fresh193 = (*x_2).data.operand.right;
                            *fresh193 = exnewnode(
                                expr.program,
                                ',' as i32,
                                1 as libc::c_int,
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            current_block = 16782213842982696595;
                        }
                        130 => {
                            expr.declare = (*(*yyvsp.offset(0 as libc::c_int as isize)).id).type_0
                                as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        131 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                283 as libc::c_int,
                                0 as libc::c_int,
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).type_0
                                    as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh194 = (*yyval.expr).data.variable.symbol;
                            *fresh194 = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            (*(*yyvsp.offset(0 as libc::c_int as isize)).id).lex =
                                275 as libc::c_int as libc::c_long;
                            (*(*yyvsp.offset(0 as libc::c_int as isize)).id).type_0 =
                                (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).id).type_0;
                            let ref mut fresh195 =
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).id).value;
                            *fresh195 = exnewnode(
                                expr.program,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            let ref mut fresh196 = (*expr.procedure).data.procedure.arity;
                            *fresh196 += 1;
                            expr.declare = 0 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        132 => {
                            expr.lastref = 0 as *mut Exref_t;
                            expr.refs = expr.lastref;
                            yyval.reference = expr.refs;
                            current_block = 16782213842982696595;
                        }
                        133 => {
                            let mut r: *mut Exref_t = 0 as *mut Exref_t;
                            r = vmalloc(
                                (*expr.program).vm,
                                ::std::mem::size_of::<Exref_t>() as libc::c_ulong,
                            ) as *mut Exref_t;
                            memset(
                                r as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<Exref_t>() as libc::c_ulong,
                            );
                            let ref mut fresh197 = (*r).symbol;
                            *fresh197 = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            expr.refs = r;
                            expr.lastref = r;
                            let ref mut fresh198 = (*r).next;
                            *fresh198 = 0 as *mut Exref_t;
                            let ref mut fresh199 = (*r).index;
                            *fresh199 = 0 as *mut Exnode_t;
                            yyval.reference = expr.refs;
                            current_block = 16782213842982696595;
                        }
                        134 => {
                            let mut r_0: *mut Exref_t = 0 as *mut Exref_t;
                            let mut l: *mut Exref_t = 0 as *mut Exref_t;
                            r_0 = vmalloc(
                                (*expr.program).vm,
                                ::std::mem::size_of::<Exref_t>() as libc::c_ulong,
                            ) as *mut Exref_t;
                            memset(
                                r_0 as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<Exref_t>() as libc::c_ulong,
                            );
                            let ref mut fresh200 = (*r_0).symbol;
                            *fresh200 = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            let ref mut fresh201 = (*r_0).index;
                            *fresh201 = 0 as *mut Exnode_t;
                            let ref mut fresh202 = (*r_0).next;
                            *fresh202 = 0 as *mut Exref_t;
                            l = vmalloc(
                                (*expr.program).vm,
                                ::std::mem::size_of::<Exref_t>() as libc::c_ulong,
                            ) as *mut Exref_t;
                            memset(
                                l as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<Exref_t>() as libc::c_ulong,
                            );
                            let ref mut fresh203 = (*l).symbol;
                            *fresh203 = (*yyvsp.offset(-(1 as libc::c_int) as isize)).id;
                            let ref mut fresh204 = (*l).index;
                            *fresh204 = 0 as *mut Exnode_t;
                            let ref mut fresh205 = (*l).next;
                            *fresh205 = r_0;
                            expr.refs = l;
                            expr.lastref = r_0;
                            yyval.reference = expr.refs;
                            current_block = 16782213842982696595;
                        }
                        135 => {
                            yyval.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            current_block = 16782213842982696595;
                        }
                        136 => {
                            yyval.id = (*yyvsp.offset(0 as libc::c_int as isize)).id;
                            current_block = 16782213842982696595;
                        }
                        137 => {
                            yyval.expr = 0 as *mut Exnode_s;
                            current_block = 16782213842982696595;
                        }
                        138 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                '=' as i32,
                                1 as libc::c_int,
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                0 as *mut Exnode_t,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                            );
                            (*yyval.expr).subop = (*yyvsp.offset(-(1 as libc::c_int) as isize)).op;
                            current_block = 16782213842982696595;
                        }
                        140 => {
                            let mut disc_1: *mut Dtdisc_t = 0 as *mut Dtdisc_t;
                            if !(expr.procedure).is_null() {
                                exerror(
                                    b"%s: nested function definitions not supported\0" as *const u8
                                        as *const libc::c_char,
                                    ((*expr.id).name).as_mut_ptr(),
                                );
                            }
                            expr.procedure = exnewnode(
                                expr.program,
                                293 as libc::c_int,
                                1 as libc::c_int,
                                expr.declare,
                                0 as *mut Exnode_t,
                                0 as *mut Exnode_t,
                            );
                            disc_1 = if 0 as libc::c_int != 0 {
                                realloc(
                                    0 as *mut libc::c_char as *mut libc::c_void,
                                    (::std::mem::size_of::<Dtdisc_t>() as libc::c_ulong)
                                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut Dtdisc_t
                            } else {
                                calloc(
                                    1 as libc::c_int as libc::c_ulong,
                                    (::std::mem::size_of::<Dtdisc_t>() as libc::c_ulong)
                                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ) as *mut Dtdisc_t
                            };
                            if disc_1.is_null() {
                                exnospace();
                            }
                            (*disc_1).key = 88 as libc::c_ulong as libc::c_int;
                            if strcmp(
                                ((*expr.id).name).as_mut_ptr(),
                                b"begin\0" as *const u8 as *const libc::c_char,
                            ) != 0
                            {
                                let ref mut fresh206 = (*expr.procedure).data.procedure.frame;
                                *fresh206 = dtopen(disc_1, Dtset);
                                if (*fresh206).is_null()
                                    || (dtview(
                                        (*expr.procedure).data.procedure.frame,
                                        (*expr.program).symbols,
                                    ))
                                    .is_null()
                                {
                                    exnospace();
                                }
                                let ref mut fresh207 = (*expr.program).frame;
                                *fresh207 = (*expr.procedure).data.procedure.frame;
                                let ref mut fresh208 = (*expr.program).symbols;
                                *fresh208 = *fresh207;
                                (*expr.program).formals = 1 as libc::c_int;
                            }
                            expr.declare = 0 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        141 => {
                            (*expr.id).lex = 293 as libc::c_int as libc::c_long;
                            (*expr.id).type_0 = (*expr.procedure).type_0 as libc::c_long;
                            (*expr.program).formals = 0 as libc::c_int;
                            expr.declare = 0 as libc::c_int;
                            current_block = 16782213842982696595;
                        }
                        142 => {
                            yyval.expr = expr.procedure;
                            expr.procedure = 0 as *mut Exnode_t;
                            if !((*expr.program).frame).is_null() {
                                let ref mut fresh209 = (*expr.program).symbols;
                                *fresh209 = (*(*expr.program).frame).view;
                                dtview((*expr.program).frame, 0 as *mut Dt_t);
                                let ref mut fresh210 = (*expr.program).frame;
                                *fresh210 = 0 as *mut Dt_t;
                            }
                            let ref mut fresh211 = (*yyval.expr).data.operand.left;
                            *fresh211 = (*yyvsp.offset(-(5 as libc::c_int) as isize)).expr;
                            let ref mut fresh212 = (*yyval.expr).data.operand.right;
                            *fresh212 = excast(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).expr,
                                (*yyval.expr).type_0,
                                0 as *mut Exnode_t,
                                0 as libc::c_int,
                            );
                            let ref mut fresh213 = (*expr.program).linep;
                            *fresh213 = (*fresh213).offset(-1);
                            (*(*expr.program).input).peek = ';' as i32;
                            current_block = 16782213842982696595;
                        }
                        _ => {
                            current_block = 16782213842982696595;
                        }
                    }
                    match current_block {
                        17643436426743203513 => {
                            yyval.expr = exnewnode(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).op,
                                1 as libc::c_int,
                                if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                    == 260 as libc::c_int
                                {
                                    259 as libc::c_int
                                } else {
                                    (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                },
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                0 as *mut Exnode_t,
                            );
                            if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).op
                                == 271 as libc::c_int
                            {
                                (*yyval.expr).data.constant.value =
                                    exeval(expr.program, yyval.expr, 0 as *mut libc::c_void);
                                (*yyval.expr).binary = 0 as libc::c_int;
                                (*yyval.expr).op = 271 as libc::c_int;
                                exfreenode(
                                    expr.program,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                );
                            } else if !((*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                > 258 as libc::c_int)
                            {
                                checkBinary(
                                    expr.program,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                    yyval.expr,
                                    0 as *mut Exnode_t,
                                );
                            }
                            current_block = 16782213842982696595;
                        }
                        720742499249991355 => {
                            rel = 0 as libc::c_int;
                            current_block = 7062641858472886451;
                        }
                        3290856744090149140 => {
                            rel = 259 as libc::c_int;
                            current_block = 7062641858472886451;
                        }
                        _ => {}
                    }
                    match current_block {
                        7062641858472886451 => {
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0 == 0 {
                                if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 == 0 {
                                    let ref mut fresh140 =
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0;
                                    *fresh140 = if rel != 0 {
                                        263 as libc::c_int
                                    } else {
                                        259 as libc::c_int
                                    };
                                    (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0 =
                                        *fresh140;
                                } else {
                                    (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0 =
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0;
                                }
                            } else if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 == 0
                            {
                                (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0 =
                                    (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0;
                            }
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0
                                != (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                            {
                                if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).type_0
                                    == 263 as libc::c_int
                                {
                                    let ref mut fresh141 =
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                                    *fresh141 = excast(
                                        expr.program,
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                        (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                        0 as libc::c_int,
                                    );
                                } else if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                    == 263 as libc::c_int
                                {
                                    let ref mut fresh142 =
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                    *fresh142 = excast(
                                        expr.program,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                        (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                            .type_0,
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                        0 as libc::c_int,
                                    );
                                } else if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .type_0
                                    == 262 as libc::c_int
                                {
                                    let ref mut fresh143 =
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr;
                                    *fresh143 = excast(
                                        expr.program,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                        262 as libc::c_int,
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                        0 as libc::c_int,
                                    );
                                } else if (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                    == 262 as libc::c_int
                                {
                                    let ref mut fresh144 =
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr;
                                    *fresh144 = excast(
                                        expr.program,
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                        262 as libc::c_int,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                        0 as libc::c_int,
                                    );
                                }
                            }
                            if rel == 0 {
                                rel = if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .type_0
                                    == 263 as libc::c_int
                                {
                                    263 as libc::c_int
                                } else if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                    .type_0
                                    == 260 as libc::c_int
                                {
                                    260 as libc::c_int
                                } else {
                                    (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                };
                            }
                            yyval.expr = exnewnode(
                                expr.program,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).op,
                                1 as libc::c_int,
                                rel,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                            );
                            if (*expr.program).errors == 0
                                && (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr).op
                                    == 271 as libc::c_int
                                && (*(*yyvsp.offset(0 as libc::c_int as isize)).expr).op
                                    == 271 as libc::c_int
                            {
                                (*yyval.expr).data.constant.value =
                                    exeval(expr.program, yyval.expr, 0 as *mut libc::c_void);
                                if (*yyval.expr).type_0 == 263 as libc::c_int {
                                    let ref mut fresh145 = (*yyval.expr).data.constant.value.string;
                                    *fresh145 = vmstrdup(
                                        (*expr.program).vm,
                                        (*yyval.expr).data.constant.value.string,
                                    );
                                }
                                (*yyval.expr).binary = 0 as libc::c_int;
                                (*yyval.expr).op = 271 as libc::c_int;
                                exfreenode(
                                    expr.program,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                );
                                exfreenode(
                                    expr.program,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                );
                            } else if !((*(*yyvsp.offset(-(2 as libc::c_int) as isize)).expr)
                                .type_0
                                > 258 as libc::c_int)
                                || !((*(*yyvsp.offset(0 as libc::c_int as isize)).expr).type_0
                                    > 258 as libc::c_int)
                            {
                                checkBinary(
                                    expr.program,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).expr,
                                    yyval.expr,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).expr,
                                );
                            }
                        }
                        _ => {}
                    }
                    if ex_debug != 0 {
                        sfprintf(
                            sfstderr,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            b"-> $$ =\0" as *const u8 as *const libc::c_char,
                        );
                        yy_symbol_print(sfstderr, yyr1[yyn as usize] as libc::c_int, &mut yyval);
                        sfprintf(sfstderr, b"\n\0" as *const u8 as *const libc::c_char);
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    if ex_debug != 0 {
                        yy_stack_print(yyss, yyssp);
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    yyn = yyr1[yyn as usize] as libc::c_int;
                    yystate = yypgoto[(yyn - 107 as libc::c_int) as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    if 0 as libc::c_int <= yystate
                        && yystate <= 1112 as libc::c_int
                        && yycheck[yystate as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        yystate = yytable[yystate as usize] as libc::c_int;
                    } else {
                        yystate = yydefgoto[(yyn - 107 as libc::c_int) as usize] as libc::c_int;
                    }
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
        }
    }
    match current_block {
        12056922904886382946 => {
            ex_error(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        6934551251188356327 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if ex_char != -(2 as libc::c_int) {
        yytoken = if ex_char as libc::c_uint <= 336 as libc::c_int as libc::c_uint {
            yytranslate[ex_char as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut ex_lval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    if ex_debug != 0 {
        yy_stack_print(yyss, yyssp);
    }
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
