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
#![feature(register_tool)]
extern "C" {
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn _sffilbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn chrtoi(_: *const libc::c_char) -> libc::c_int;
    fn stresc(_: *mut libc::c_char) -> libc::c_int;
    static mut ex_lval: EX_STYPE;
    fn dtview(_: *mut Dt_t, _: *mut Dt_t) -> *mut Dt_t;
    fn vmstrdup(_: *mut Vmalloc_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn exerror(_: *const libc::c_char, _: ...);
    fn exnospace() -> *mut libc::c_char;
    fn expop(_: *mut Expr_t) -> libc::c_int;
    fn expush(
        _: *mut Expr_t,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: *mut Sfio_t,
    ) -> libc::c_int;
    fn exstash(_: *mut Sfio_t, _: *mut Vmalloc_t) -> *mut libc::c_char;
    static mut _err_info: Error_info_t;
    static mut expr: Exstate_t;
    static mut exbuiltin: [Exid_t; 0];
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub const _ISalnum: C2RustUnnamed_13 = 8;
pub const _ISalpha: C2RustUnnamed_13 = 1024;
pub const _ISdigit: C2RustUnnamed_13 = 2048;
pub const _ISspace: C2RustUnnamed_13 = 8192;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_13 = 4;
pub const _IScntrl: C2RustUnnamed_13 = 2;
pub const _ISblank: C2RustUnnamed_13 = 1;
pub const _ISgraph: C2RustUnnamed_13 = 32768;
pub const _ISprint: C2RustUnnamed_13 = 16384;
pub const _ISxdigit: C2RustUnnamed_13 = 4096;
pub const _ISlower: C2RustUnnamed_13 = 512;
pub const _ISupper: C2RustUnnamed_13 = 256;
unsafe extern "C" fn lex(mut ex: *mut Expr_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        c = (*(*ex).input).peek;
        if c != 0 {
            (*(*ex).input).peek = 0 as libc::c_int;
        } else if !((*(*ex).input).pp).is_null() {
            let ref mut fresh0 = (*(*ex).input).pp;
            let fresh1 = *fresh0;
            *fresh0 = (*fresh0).offset(1);
            c = *fresh1 as libc::c_int;
            if c == 0 {
                let ref mut fresh2 = (*(*ex).input).pp;
                *fresh2 = 0 as *mut libc::c_char;
                continue;
            }
        } else if !((*(*ex).input).sp).is_null() {
            let ref mut fresh3 = (*(*ex).input).sp;
            let fresh4 = *fresh3;
            *fresh3 = (*fresh3).offset(1);
            c = *fresh4 as libc::c_int;
            if c == 0 {
                if expop(ex) == 0 {
                    continue;
                }
                let ref mut fresh5 = (*(*ex).input).sp;
                *fresh5 = (*fresh5).offset(-1);
            }
        } else if !((*(*ex).input).fp).is_null() {
            c = (if (*(*(*ex).input).fp).next >= (*(*(*ex).input).fp).endr {
                _sffilbuf((*(*ex).input).fp, 0 as libc::c_int)
            } else {
                let ref mut fresh6 = (*(*(*ex).input).fp).next;
                let fresh7 = *fresh6;
                *fresh6 = (*fresh6).offset(1);
                *fresh7 as libc::c_int
            });
            if c == -(1 as libc::c_int) {
                if expop(ex) == 0 {
                    continue;
                }
                c = 0 as libc::c_int;
            } else if (*(*ex).disc).flags
                & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
                != 0
                && c == '\n' as i32
                && !((*(*ex).input).next).is_null()
                && ((*(*(*ex).input).next).next).is_null()
                && (*(*ex).input).nesting <= 0 as libc::c_int
            {
                _err_info.line += 1;
                expop(ex);
                c = 0 as libc::c_int;
            }
        } else {
            c = 0 as libc::c_int;
        }
        if c == '\n' as i32 {
            let ref mut fresh8 = (*ex).linep;
            *fresh8 = ((*ex).line).as_mut_ptr();
            (*ex).linewrap = 0 as libc::c_int;
        } else if c != 0 {
            if (*ex).linep
                >= &mut *((*ex).line)
                    .as_mut_ptr()
                    .offset(::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as isize)
                    as *mut libc::c_char
            {
                let ref mut fresh9 = (*ex).linep;
                *fresh9 = ((*ex).line).as_mut_ptr();
                (*ex).linewrap = 1 as libc::c_int;
            } else {
            };
            let ref mut fresh10 = (*ex).linep;
            let fresh11 = *fresh10;
            *fresh10 = (*fresh10).offset(1);
            *fresh11 = c as libc::c_char;
        }
        return c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn extoken_fn(mut ex: *mut Expr_t) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut Dt_t = 0 as *mut Dt_t;
    if (*ex).eof != 0 || (*ex).errors != 0 {
        return 0 as libc::c_int;
    }
    's_30: loop {
        c = lex(ex);
        match c {
            0 => {
                break;
            }
            47 => {
                q = lex(ex);
                match q {
                    42 => {
                        current_block = 5720623009719927633;
                        loop {
                            match current_block {
                                5720623009719927633 => match lex(ex) {
                                    10 => {
                                        if _err_info.line != 0 {
                                            _err_info.line += 1;
                                        } else {
                                            _err_info.line = 2 as libc::c_int;
                                        }
                                        current_block = 5720623009719927633;
                                    }
                                    42 => match lex(ex) {
                                        0 => {
                                            break 's_30;
                                        }
                                        10 => {
                                            if _err_info.line != 0 {
                                                _err_info.line += 1;
                                            } else {
                                                _err_info.line = 2 as libc::c_int;
                                            }
                                            current_block = 5720623009719927633;
                                        }
                                        42 => {
                                            let ref mut fresh12 = (*ex).linep;
                                            *fresh12 = (*fresh12).offset(-1);
                                            (*(*ex).input).peek = '*' as i32;
                                            current_block = 5720623009719927633;
                                        }
                                        47 => {
                                            continue 's_30;
                                        }
                                        _ => {
                                            current_block = 5720623009719927633;
                                        }
                                    },
                                    _ => {
                                        current_block = 5720623009719927633;
                                    }
                                },
                                _ => {
                                    c = lex(ex);
                                    if !(c != '\n' as i32) {
                                        break;
                                    }
                                    if c == 0 {
                                        break 's_30;
                                    } else {
                                        current_block = 11194104282611034094;
                                    }
                                }
                            }
                        }
                        current_block = 8005915163409206930;
                    }
                    47 => {
                        current_block = 11194104282611034094;
                        loop {
                            match current_block {
                                5720623009719927633 => match lex(ex) {
                                    10 => {
                                        if _err_info.line != 0 {
                                            _err_info.line += 1;
                                        } else {
                                            _err_info.line = 2 as libc::c_int;
                                        }
                                        current_block = 5720623009719927633;
                                    }
                                    42 => match lex(ex) {
                                        0 => {
                                            break 's_30;
                                        }
                                        10 => {
                                            if _err_info.line != 0 {
                                                _err_info.line += 1;
                                            } else {
                                                _err_info.line = 2 as libc::c_int;
                                            }
                                            current_block = 5720623009719927633;
                                        }
                                        42 => {
                                            let ref mut fresh12 = (*ex).linep;
                                            *fresh12 = (*fresh12).offset(-1);
                                            (*(*ex).input).peek = '*' as i32;
                                            current_block = 5720623009719927633;
                                        }
                                        47 => {
                                            continue 's_30;
                                        }
                                        _ => {
                                            current_block = 5720623009719927633;
                                        }
                                    },
                                    _ => {
                                        current_block = 5720623009719927633;
                                    }
                                },
                                _ => {
                                    c = lex(ex);
                                    if !(c != '\n' as i32) {
                                        break;
                                    }
                                    if c == 0 {
                                        break 's_30;
                                    } else {
                                        current_block = 11194104282611034094;
                                    }
                                }
                            }
                        }
                        current_block = 8005915163409206930;
                    }
                    _ => {
                        current_block = 7469032322570361295;
                    }
                }
            }
            10 => {
                current_block = 8005915163409206930;
            }
            32 | 9 | 13 => {
                continue;
            }
            40 | 123 | 91 => {
                let ref mut fresh13 = (*(*ex).input).nesting;
                *fresh13 += 1;
                ex_lval.op = c;
                return ex_lval.op;
            }
            41 | 125 | 93 => {
                let ref mut fresh14 = (*(*ex).input).nesting;
                *fresh14 -= 1;
                ex_lval.op = c;
                return ex_lval.op;
            }
            43 | 45 => {
                q = lex(ex);
                if q == c {
                    ex_lval.op = if c == '+' as i32 {
                        333 as libc::c_int
                    } else {
                        334 as libc::c_int
                    };
                    return ex_lval.op;
                }
                current_block = 7469032322570361295;
            }
            42 | 37 | 94 => {
                q = lex(ex);
                current_block = 7469032322570361295;
            }
            38 | 124 => {
                q = lex(ex);
                if q == '=' as i32 {
                    ex_lval.op = c;
                    return '=' as i32;
                }
                if q == c {
                    c = if c == '&' as i32 {
                        324 as libc::c_int
                    } else {
                        323 as libc::c_int
                    };
                } else {
                    let ref mut fresh18 = (*ex).linep;
                    *fresh18 = (*fresh18).offset(-1);
                    (*(*ex).input).peek = q;
                }
                ex_lval.op = c;
                return ex_lval.op;
            }
            60 | 62 => {
                q = lex(ex);
                if q == c {
                    c = if c == '<' as i32 {
                        329 as libc::c_int
                    } else {
                        330 as libc::c_int
                    };
                    ex_lval.op = c;
                    q = lex(ex);
                    if q == '=' as i32 {
                        c = '=' as i32;
                    } else {
                        let ref mut fresh19 = (*ex).linep;
                        *fresh19 = (*fresh19).offset(-1);
                        (*(*ex).input).peek = q;
                    }
                    return c;
                }
                current_block = 12720622314508247215;
            }
            61 | 33 => {
                q = lex(ex);
                current_block = 12720622314508247215;
            }
            35 => {
                if (*ex).linewrap == 0
                    && (*(*ex).disc).flags
                        & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong
                        == 0
                {
                    s = ((*ex).linep).offset(-(1 as libc::c_int as isize));
                    while s > ((*ex).line).as_mut_ptr()
                        && *(*__ctype_b_loc())
                            .offset(*s.offset(-(1 as libc::c_int as isize)) as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        s = s.offset(-1);
                    }
                    if s == ((*ex).line).as_mut_ptr() {
                        match extoken_fn(ex) {
                            275 | 283 | 287 => {
                                s = ((*ex_lval.id).name).as_mut_ptr();
                            }
                            _ => {
                                s = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                            }
                        }
                        if strcmp(s, b"include\0" as *const u8 as *const libc::c_char) == 0 {
                            if extoken_fn(ex) != 263 as libc::c_int {
                                exerror(
                                    b"#%s: string argument expected\0" as *const u8
                                        as *const libc::c_char,
                                    s,
                                );
                            } else if expush(
                                ex,
                                ex_lval.string,
                                1 as libc::c_int,
                                0 as *const libc::c_char,
                                0 as *mut Sfio_t,
                            ) == 0
                            {
                                let ref mut fresh21 = (*ex).linep;
                                *fresh21 = ((*ex).line).as_mut_ptr();
                                (*ex).linewrap = 0 as libc::c_int;
                                continue;
                            }
                        } else {
                            exerror(b"unknown directive\0" as *const u8 as *const libc::c_char);
                        }
                    }
                }
                ex_lval.op = c;
                return ex_lval.op;
            }
            39 | 34 => {
                q = c;
                if (0 as libc::c_int) < 0 as libc::c_int
                    || 0 as libc::c_int as libc::c_long > (*(*ex).tmp).size
                {
                } else {
                    let ref mut fresh22 = (*(*ex).tmp).next;
                    *fresh22 = ((*(*ex).tmp).data).offset(0 as libc::c_int as isize);
                };
                let ref mut fresh23 = (*(*ex).input).nesting;
                *fresh23 += 1;
                loop {
                    c = lex(ex);
                    if !(c != q) {
                        break;
                    }
                    if c == '\\' as i32 {
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh24 = (*(*ex).tmp).next;
                            let fresh25 = *fresh24;
                            *fresh24 = (*fresh24).offset(1);
                            *fresh25 = c as libc::c_uchar;
                        };
                        c = lex(ex);
                    }
                    if c == 0 {
                        exerror(
                            b"unterminated %c string\0" as *const u8 as *const libc::c_char,
                            q,
                        );
                        break 's_30;
                    } else {
                        if c == '\n' as i32 {
                            if _err_info.line != 0 {
                                _err_info.line += 1;
                            } else {
                                _err_info.line = 2 as libc::c_int;
                            }
                        }
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh26 = (*(*ex).tmp).next;
                            let fresh27 = *fresh26;
                            *fresh26 = (*fresh26).offset(1);
                            *fresh27 = c as libc::c_uchar;
                        };
                    }
                }
                let ref mut fresh28 = (*(*ex).input).nesting;
                *fresh28 -= 1;
                s = exstash((*ex).tmp, 0 as *mut Vmalloc_t);
                if q == '"' as i32
                    || (*(*ex).disc).flags
                        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                        != 0
                {
                    ex_lval.string = vmstrdup((*ex).vm, s);
                    if (ex_lval.string).is_null() {
                        break;
                    }
                    stresc(ex_lval.string);
                    return 263 as libc::c_int;
                } else {
                    ex_lval.integer = chrtoi(s) as libc::c_longlong;
                    return 259 as libc::c_int;
                }
            }
            46 => {
                c = lex(ex);
                if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    if (0 as libc::c_int) < 0 as libc::c_int
                        || 0 as libc::c_int as libc::c_long > (*(*ex).tmp).size
                    {
                    } else {
                        let ref mut fresh29 = (*(*ex).tmp).next;
                        *fresh29 = ((*(*ex).tmp).data).offset(0 as libc::c_int as isize);
                    };
                    if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                        _sfflsbuf((*ex).tmp, '0' as i32 as libc::c_uchar as libc::c_int);
                    } else {
                        let ref mut fresh30 = (*(*ex).tmp).next;
                        let fresh31 = *fresh30;
                        *fresh30 = (*fresh30).offset(1);
                        *fresh31 = '0' as i32 as libc::c_uchar;
                    };
                    if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                        _sfflsbuf((*ex).tmp, '.' as i32 as libc::c_uchar as libc::c_int);
                    } else {
                        let ref mut fresh32 = (*(*ex).tmp).next;
                        let fresh33 = *fresh32;
                        *fresh32 = (*fresh32).offset(1);
                        *fresh33 = '.' as i32 as libc::c_uchar;
                    };
                } else {
                    let ref mut fresh34 = (*ex).linep;
                    *fresh34 = (*fresh34).offset(-1);
                    (*(*ex).input).peek = c;
                    ex_lval.op = '.' as i32;
                    return ex_lval.op;
                }
                current_block = 5395845958331332369;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if (0 as libc::c_int) < 0 as libc::c_int
                    || 0 as libc::c_int as libc::c_long > (*(*ex).tmp).size
                {
                } else {
                    let ref mut fresh35 = (*(*ex).tmp).next;
                    *fresh35 = ((*(*ex).tmp).data).offset(0 as libc::c_int as isize);
                };
                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                    _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                } else {
                    let ref mut fresh36 = (*(*ex).tmp).next;
                    let fresh37 = *fresh36;
                    *fresh36 = (*fresh36).offset(1);
                    *fresh37 = c as libc::c_uchar;
                };
                q = 259 as libc::c_int;
                b = 0 as libc::c_int;
                c = lex(ex);
                if c == 'x' as i32 || c == 'X' as i32 {
                    b = 16 as libc::c_int;
                    if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                        _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                    } else {
                        let ref mut fresh38 = (*(*ex).tmp).next;
                        let fresh39 = *fresh38;
                        *fresh38 = (*fresh38).offset(1);
                        *fresh39 = c as libc::c_uchar;
                    };
                    loop {
                        c = lex(ex);
                        match c {
                            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99
                            | 100 | 101 | 102 | 65 | 66 | 67 | 68 | 69 | 70 => {}
                            _ => {
                                break;
                            }
                        }
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh40 = (*(*ex).tmp).next;
                            let fresh41 = *fresh40;
                            *fresh40 = (*fresh40).offset(1);
                            *fresh41 = c as libc::c_uchar;
                        };
                    }
                    current_block = 13176516025253886821;
                } else {
                    while *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh42 = (*(*ex).tmp).next;
                            let fresh43 = *fresh42;
                            *fresh42 = (*fresh42).offset(1);
                            *fresh43 = c as libc::c_uchar;
                        };
                        c = lex(ex);
                    }
                    if c == '#' as i32 {
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh44 = (*(*ex).tmp).next;
                            let fresh45 = *fresh44;
                            *fresh44 = (*fresh44).offset(1);
                            *fresh45 = c as libc::c_uchar;
                        };
                        loop {
                            if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                            } else {
                                let ref mut fresh46 = (*(*ex).tmp).next;
                                let fresh47 = *fresh46;
                                *fresh46 = (*fresh46).offset(1);
                                *fresh47 = c as libc::c_uchar;
                            };
                            c = lex(ex);
                            if !(*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                            {
                                break;
                            }
                        }
                        current_block = 13176516025253886821;
                    } else if c == '.' as i32 {
                        current_block = 5395845958331332369;
                    } else {
                        current_block = 14005454229507937959;
                    }
                }
            }
            _ => {
                if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                    || c == '_' as i32
                    || c == '$' as i32
                {
                    if (0 as libc::c_int) < 0 as libc::c_int
                        || 0 as libc::c_int as libc::c_long > (*(*ex).tmp).size
                    {
                    } else {
                        let ref mut fresh59 = (*(*ex).tmp).next;
                        *fresh59 = ((*(*ex).tmp).data).offset(0 as libc::c_int as isize);
                    };
                    if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                        _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                    } else {
                        let ref mut fresh60 = (*(*ex).tmp).next;
                        let fresh61 = *fresh60;
                        *fresh60 = (*fresh60).offset(1);
                        *fresh61 = c as libc::c_uchar;
                    };
                    loop {
                        c = lex(ex);
                        if !(*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                            || c == '_' as i32
                            || c == '$' as i32)
                        {
                            break;
                        }
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh62 = (*(*ex).tmp).next;
                            let fresh63 = *fresh62;
                            *fresh62 = (*fresh62).offset(1);
                            *fresh63 = c as libc::c_uchar;
                        };
                    }
                    let ref mut fresh64 = (*ex).linep;
                    *fresh64 = (*fresh64).offset(-1);
                    (*(*ex).input).peek = c;
                    s = exstash((*ex).tmp, 0 as *mut Vmalloc_t);
                    v = 0 as *mut Dt_t;
                    ex_lval.id =
                        (Some(((*(*ex).symbols).searchf).expect("non-null function pointer")))
                            .expect("non-null function pointer")(
                            (*ex).symbols,
                            s as *mut libc::c_void,
                            0o1000 as libc::c_int,
                        ) as *mut Exid_s;
                    if !v.is_null() {
                        dtview((*ex).symbols, v);
                    }
                    if (ex_lval.id).is_null() {
                        ex_lval.id = if 0 as libc::c_int != 0 {
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
                        if (ex_lval.id).is_null() {
                            exnospace();
                            break;
                        } else {
                            strcpy(((*ex_lval.id).name).as_mut_ptr(), s);
                            (*ex_lval.id).lex = 287 as libc::c_int as libc::c_long;
                            let ref mut fresh65 = (*ex_lval.id).isstatic;
                            *fresh65 = expr.instatic as libc::c_long;
                            expr.statics = (expr.statics as libc::c_long + *fresh65) as libc::c_int;
                            if c == ':' as i32
                                && expr.nolabel == 0
                                && !((*ex).frame).is_null()
                                && !((*(*ex).frame).view).is_null()
                            {
                                (Some(
                                    ((*(*(*ex).frame).view).searchf)
                                        .expect("non-null function pointer"),
                                ))
                                .expect("non-null function pointer")(
                                    (*(*ex).frame).view,
                                    ex_lval.id as *mut libc::c_void,
                                    0o1 as libc::c_int,
                                );
                            } else {
                                (Some(
                                    ((*(*ex).symbols).searchf).expect("non-null function pointer"),
                                ))
                                .expect("non-null function pointer")(
                                    (*ex).symbols,
                                    ex_lval.id as *mut libc::c_void,
                                    0o1 as libc::c_int,
                                );
                            }
                        }
                    }
                    match (*ex_lval.id).lex {
                        273 => {
                            if (*ex_lval.id).index == 261 as libc::c_int as libc::c_long {
                                if c == '*' as i32 {
                                    lex(ex);
                                    ex_lval.id = &mut *exbuiltin
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize)
                                        as *mut Exid_t;
                                }
                            }
                        }
                        287 => {
                            if c == ':' as i32 && expr.nolabel == 0 {
                                return 285 as libc::c_int;
                            }
                        }
                        289 => {
                            let mut b_0: libc::c_int = 0;
                            let mut n: libc::c_int = 0;
                            let mut pc: libc::c_int = 0 as libc::c_int;
                            let mut po: libc::c_int = 0;
                            let mut t: libc::c_int = 0;
                            if (0 as libc::c_int) < 0 as libc::c_int
                                || 0 as libc::c_int as libc::c_long > (*(*ex).tmp).size
                            {
                            } else {
                                let ref mut fresh66 = (*(*ex).tmp).next;
                                *fresh66 = ((*(*ex).tmp).data).offset(0 as libc::c_int as isize);
                            };
                            b_0 = 1 as libc::c_int;
                            n = 0 as libc::c_int;
                            po = 0 as libc::c_int;
                            t = lex(ex);
                            c = t;
                            loop {
                                match c {
                                    0 => {
                                        break 's_30;
                                    }
                                    47 => {
                                        q = lex(ex);
                                        match q {
                                            42 => loop {
                                                match lex(ex) {
                                                    10 => {
                                                        if _err_info.line != 0 {
                                                            _err_info.line += 1;
                                                        } else {
                                                            _err_info.line = 2 as libc::c_int;
                                                        }
                                                        continue;
                                                    }
                                                    42 => match lex(ex) {
                                                        0 => {
                                                            break 's_30;
                                                        }
                                                        10 => {
                                                            current_block = 4450415816793006058;
                                                            match current_block {
                                                                12827718114001486934 => {
                                                                    let ref mut fresh67 =
                                                                        (*ex).linep;
                                                                    *fresh67 =
                                                                        (*fresh67).offset(-1);
                                                                    (*(*ex).input).peek =
                                                                        '*' as i32;
                                                                    continue;
                                                                }
                                                                _ => {
                                                                    if _err_info.line != 0 {
                                                                        _err_info.line += 1;
                                                                    } else {
                                                                        _err_info.line =
                                                                            2 as libc::c_int;
                                                                    }
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        42 => {
                                                            current_block = 12827718114001486934;
                                                            match current_block {
                                                                12827718114001486934 => {
                                                                    let ref mut fresh67 =
                                                                        (*ex).linep;
                                                                    *fresh67 =
                                                                        (*fresh67).offset(-1);
                                                                    (*(*ex).input).peek =
                                                                        '*' as i32;
                                                                    continue;
                                                                }
                                                                _ => {
                                                                    if _err_info.line != 0 {
                                                                        _err_info.line += 1;
                                                                    } else {
                                                                        _err_info.line =
                                                                            2 as libc::c_int;
                                                                    }
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        47 => {}
                                                        _ => {
                                                            continue;
                                                        }
                                                    },
                                                    _ => {}
                                                }
                                                let fresh68 = b_0;
                                                b_0 = b_0 + 1;
                                                if fresh68 == 0 {
                                                    break 's_30;
                                                }
                                                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                                    _sfflsbuf(
                                                        (*ex).tmp,
                                                        ' ' as i32 as libc::c_uchar as libc::c_int,
                                                    );
                                                } else {
                                                    let ref mut fresh69 = (*(*ex).tmp).next;
                                                    let fresh70 = *fresh69;
                                                    *fresh69 = (*fresh69).offset(1);
                                                    *fresh70 = ' ' as i32 as libc::c_uchar;
                                                };
                                                break;
                                            },
                                            47 => {
                                                loop {
                                                    c = lex(ex);
                                                    if !(c != '\n' as i32) {
                                                        break;
                                                    }
                                                    if c == 0 {
                                                        break 's_30;
                                                    }
                                                }
                                                if _err_info.line != 0 {
                                                    _err_info.line += 1;
                                                } else {
                                                    _err_info.line = 2 as libc::c_int;
                                                }
                                                b_0 = 1 as libc::c_int;
                                                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                                    _sfflsbuf(
                                                        (*ex).tmp,
                                                        '\n' as i32 as libc::c_uchar as libc::c_int,
                                                    );
                                                } else {
                                                    let ref mut fresh71 = (*(*ex).tmp).next;
                                                    let fresh72 = *fresh71;
                                                    *fresh71 = (*fresh71).offset(1);
                                                    *fresh72 = '\n' as i32 as libc::c_uchar;
                                                };
                                            }
                                            _ => {
                                                b_0 = 0 as libc::c_int;
                                                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                                    _sfflsbuf(
                                                        (*ex).tmp,
                                                        c as libc::c_uchar as libc::c_int,
                                                    );
                                                } else {
                                                    let ref mut fresh73 = (*(*ex).tmp).next;
                                                    let fresh74 = *fresh73;
                                                    *fresh73 = (*fresh73).offset(1);
                                                    *fresh74 = c as libc::c_uchar;
                                                };
                                                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                                    _sfflsbuf(
                                                        (*ex).tmp,
                                                        q as libc::c_uchar as libc::c_int,
                                                    );
                                                } else {
                                                    let ref mut fresh75 = (*(*ex).tmp).next;
                                                    let fresh76 = *fresh75;
                                                    *fresh75 = (*fresh75).offset(1);
                                                    *fresh76 = q as libc::c_uchar;
                                                };
                                            }
                                        }
                                    }
                                    10 => {
                                        if _err_info.line != 0 {
                                            _err_info.line += 1;
                                        } else {
                                            _err_info.line = 2 as libc::c_int;
                                        }
                                        b_0 = 1 as libc::c_int;
                                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                            _sfflsbuf(
                                                (*ex).tmp,
                                                '\n' as i32 as libc::c_uchar as libc::c_int,
                                            );
                                        } else {
                                            let ref mut fresh77 = (*(*ex).tmp).next;
                                            let fresh78 = *fresh77;
                                            *fresh77 = (*fresh77).offset(1);
                                            *fresh78 = '\n' as i32 as libc::c_uchar;
                                        };
                                    }
                                    32 | 9 => {
                                        let fresh79 = b_0;
                                        b_0 = b_0 + 1;
                                        if fresh79 == 0 {
                                            break 's_30;
                                        }
                                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                            _sfflsbuf(
                                                (*ex).tmp,
                                                ' ' as i32 as libc::c_uchar as libc::c_int,
                                            );
                                        } else {
                                            let ref mut fresh80 = (*(*ex).tmp).next;
                                            let fresh81 = *fresh80;
                                            *fresh80 = (*fresh80).offset(1);
                                            *fresh81 = ' ' as i32 as libc::c_uchar;
                                        };
                                    }
                                    40 | 123 | 91 => {
                                        b_0 = 0 as libc::c_int;
                                        if po == 0 {
                                            po = c;
                                            match po {
                                                40 => {
                                                    pc = ')' as i32;
                                                }
                                                123 => {
                                                    pc = '}' as i32;
                                                }
                                                91 => {
                                                    pc = ']' as i32;
                                                }
                                                _ => {}
                                            }
                                            n += 1;
                                        } else if c == po {
                                            n += 1;
                                        }
                                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                                        } else {
                                            let ref mut fresh82 = (*(*ex).tmp).next;
                                            let fresh83 = *fresh82;
                                            *fresh82 = (*fresh82).offset(1);
                                            *fresh83 = c as libc::c_uchar;
                                        };
                                    }
                                    41 | 125 | 93 => {
                                        b_0 = 0 as libc::c_int;
                                        if po == 0 {
                                            let ref mut fresh84 = (*ex).linep;
                                            *fresh84 = (*fresh84).offset(-1);
                                            (*(*ex).input).peek = c;
                                            break;
                                        } else {
                                            if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                                _sfflsbuf(
                                                    (*ex).tmp,
                                                    c as libc::c_uchar as libc::c_int,
                                                );
                                            } else {
                                                let ref mut fresh85 = (*(*ex).tmp).next;
                                                let fresh86 = *fresh85;
                                                *fresh85 = (*fresh85).offset(1);
                                                *fresh86 = c as libc::c_uchar;
                                            };
                                            if c == pc && {
                                                n -= 1;
                                                n <= 0 as libc::c_int
                                            } {
                                                if t == po {
                                                    break;
                                                }
                                                po = 0 as libc::c_int;
                                            }
                                        }
                                    }
                                    59 => {
                                        b_0 = 0 as libc::c_int;
                                        if n == 0 {
                                            break;
                                        }
                                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                                        } else {
                                            let ref mut fresh87 = (*(*ex).tmp).next;
                                            let fresh88 = *fresh87;
                                            *fresh87 = (*fresh87).offset(1);
                                            *fresh88 = c as libc::c_uchar;
                                        };
                                    }
                                    39 | 34 => {
                                        b_0 = 0 as libc::c_int;
                                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                                        } else {
                                            let ref mut fresh89 = (*(*ex).tmp).next;
                                            let fresh90 = *fresh89;
                                            *fresh89 = (*fresh89).offset(1);
                                            *fresh90 = c as libc::c_uchar;
                                        };
                                        let ref mut fresh91 = (*(*ex).input).nesting;
                                        *fresh91 += 1;
                                        q = c;
                                        loop {
                                            c = lex(ex);
                                            if !(c != q) {
                                                break;
                                            }
                                            if c == '\\' as i32 {
                                                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                                    _sfflsbuf(
                                                        (*ex).tmp,
                                                        c as libc::c_uchar as libc::c_int,
                                                    );
                                                } else {
                                                    let ref mut fresh92 = (*(*ex).tmp).next;
                                                    let fresh93 = *fresh92;
                                                    *fresh92 = (*fresh92).offset(1);
                                                    *fresh93 = c as libc::c_uchar;
                                                };
                                                c = lex(ex);
                                            }
                                            if c == 0 {
                                                exerror(
                                                    b"unterminated %c string\0" as *const u8
                                                        as *const libc::c_char,
                                                    q,
                                                );
                                                break 's_30;
                                            } else {
                                                if c == '\n' as i32 {
                                                    if _err_info.line != 0 {
                                                        _err_info.line += 1;
                                                    } else {
                                                        _err_info.line = 2 as libc::c_int;
                                                    }
                                                }
                                                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                                    _sfflsbuf(
                                                        (*ex).tmp,
                                                        c as libc::c_uchar as libc::c_int,
                                                    );
                                                } else {
                                                    let ref mut fresh94 = (*(*ex).tmp).next;
                                                    let fresh95 = *fresh94;
                                                    *fresh94 = (*fresh94).offset(1);
                                                    *fresh95 = c as libc::c_uchar;
                                                };
                                            }
                                        }
                                        let ref mut fresh96 = (*(*ex).input).nesting;
                                        *fresh96 -= 1;
                                    }
                                    _ => {
                                        b_0 = 0 as libc::c_int;
                                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                                        } else {
                                            let ref mut fresh97 = (*(*ex).tmp).next;
                                            let fresh98 = *fresh97;
                                            *fresh97 = (*fresh97).offset(1);
                                            *fresh98 = c as libc::c_uchar;
                                        };
                                    }
                                }
                                c = lex(ex);
                            }
                            (Some(((*(*ex).disc).reff).expect("non-null function pointer")))
                                .expect("non-null function pointer")(
                                ex,
                                0 as *mut Exnode_t,
                                ex_lval.id,
                                0 as *mut Exref_t,
                                exstash((*ex).tmp, 0 as *mut Vmalloc_t),
                                0 as libc::c_int,
                                (*ex).disc,
                            );
                            continue;
                        }
                        _ => {}
                    }
                    return (*ex_lval.id).lex as libc::c_int;
                } else {
                    ex_lval.op = c;
                    return ex_lval.op;
                }
            }
        }
        match current_block {
            5395845958331332369 => {
                q = 262 as libc::c_int;
                if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                    _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                } else {
                    let ref mut fresh48 = (*(*ex).tmp).next;
                    let fresh49 = *fresh48;
                    *fresh48 = (*fresh48).offset(1);
                    *fresh49 = c as libc::c_uchar;
                };
                loop {
                    c = lex(ex);
                    if !(*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0)
                    {
                        break;
                    }
                    if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                        _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                    } else {
                        let ref mut fresh50 = (*(*ex).tmp).next;
                        let fresh51 = *fresh50;
                        *fresh50 = (*fresh50).offset(1);
                        *fresh51 = c as libc::c_uchar;
                    };
                }
                current_block = 14005454229507937959;
            }
            7469032322570361295 => {
                ex_lval.op = c;
                if q == '=' as i32 {
                    c = '=' as i32;
                } else if q == '%' as i32 && c == '%' as i32 {
                    if !((*(*ex).input).fp).is_null() {
                        let ref mut fresh15 = (*ex).more;
                        *fresh15 = (*(*ex).input).fp as *const libc::c_char;
                    } else {
                        let ref mut fresh16 = (*ex).more;
                        *fresh16 = (*(*ex).input).sp;
                    }
                    break;
                } else {
                    let ref mut fresh17 = (*ex).linep;
                    *fresh17 = (*fresh17).offset(-1);
                    (*(*ex).input).peek = q;
                }
                return c;
            }
            12720622314508247215 => {
                if q == '=' as i32 {
                    match c {
                        60 => {
                            c = 327 as libc::c_int;
                        }
                        62 => {
                            c = 328 as libc::c_int;
                        }
                        61 => {
                            c = 325 as libc::c_int;
                        }
                        33 => {
                            c = 326 as libc::c_int;
                        }
                        _ => {}
                    }
                } else {
                    let ref mut fresh20 = (*ex).linep;
                    *fresh20 = (*fresh20).offset(-1);
                    (*(*ex).input).peek = q;
                }
                ex_lval.op = c;
                return ex_lval.op;
            }
            8005915163409206930 => {
                if _err_info.line != 0 {
                    _err_info.line += 1;
                } else {
                    _err_info.line = 2 as libc::c_int;
                }
                continue;
            }
            _ => {}
        }
        match current_block {
            14005454229507937959 => {
                if c == 'e' as i32 || c == 'E' as i32 {
                    q = 262 as libc::c_int;
                    if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                        _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                    } else {
                        let ref mut fresh52 = (*(*ex).tmp).next;
                        let fresh53 = *fresh52;
                        *fresh52 = (*fresh52).offset(1);
                        *fresh53 = c as libc::c_uchar;
                    };
                    c = lex(ex);
                    if c == '-' as i32 || c == '+' as i32 {
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh54 = (*(*ex).tmp).next;
                            let fresh55 = *fresh54;
                            *fresh54 = (*fresh54).offset(1);
                            *fresh55 = c as libc::c_uchar;
                        };
                        c = lex(ex);
                    }
                    while *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        if (*(*ex).tmp).next >= (*(*ex).tmp).endw {
                            _sfflsbuf((*ex).tmp, c as libc::c_uchar as libc::c_int);
                        } else {
                            let ref mut fresh56 = (*(*ex).tmp).next;
                            let fresh57 = *fresh56;
                            *fresh56 = (*fresh56).offset(1);
                            *fresh57 = c as libc::c_uchar;
                        };
                        c = lex(ex);
                    }
                }
            }
            _ => {}
        }
        s = exstash((*ex).tmp, 0 as *mut Vmalloc_t);
        if q == 262 as libc::c_int {
            ex_lval.floating = strtod(s, &mut e);
        } else if c == 'u' as i32 || c == 'U' as i32 {
            q = 260 as libc::c_int;
            c = lex(ex);
            ex_lval.integer = strtoull(s, &mut e, b) as libc::c_longlong;
        } else {
            ex_lval.integer = strtoll(s, &mut e, b);
        }
        let ref mut fresh58 = (*ex).linep;
        *fresh58 = (*fresh58).offset(-1);
        (*(*ex).input).peek = c;
        if *e as libc::c_int != 0
            || *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            || c == '_' as i32
            || c == '$' as i32
        {
            exerror(
                b"%s: invalid numeric constant\0" as *const u8 as *const libc::c_char,
                s,
            );
            break;
        } else {
            return q;
        }
    }
    (*ex).eof = 1 as libc::c_int;
    ex_lval.op = ';' as i32;
    return ex_lval.op;
}
