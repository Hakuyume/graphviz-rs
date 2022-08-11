#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfprintf(_: *mut Sfio_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fmtesq(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn extype(_: libc::c_int) -> *mut libc::c_char;
    static mut exversion: *const libc::c_char;
    fn dtwalk(
        _: *mut Dt_t,
        _: Option::<
            unsafe extern "C" fn(
                *mut Dt_t,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
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
pub type Dtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
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
    pub floating: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_double,
    >,
    pub integer: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char) -> libc::c_longlong,
    >,
    pub string: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char) -> *mut libc::c_char,
    >,
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
    pub castf: Option::<
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
    pub convertf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exid_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub binaryf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            *mut Exnode_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub typename: Option::<
        unsafe extern "C" fn(*mut Expr_t, libc::c_int) -> *mut libc::c_char,
    >,
    pub stringof: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            *mut Exnode_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> libc::c_int,
    >,
    pub keyf: Option::<
        unsafe extern "C" fn(
            *mut Expr_t,
            Extype_t,
            libc::c_int,
            *mut Exdisc_t,
        ) -> Extype_t,
    >,
    pub errorf: Exerror_f,
    pub getf: Option::<
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
    pub reff: Option::<
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
    pub setf: Option::<
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
    pub matchf: Option::<
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
pub type Exexit_f = Option::<
    unsafe extern "C" fn(*mut Expr_t, *mut Exdisc_t, libc::c_int) -> (),
>;
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
pub type Exerror_f = Option::<
    unsafe extern "C" fn(
        *mut Expr_t,
        *mut Exdisc_t,
        libc::c_int,
        *const libc::c_char,
        ...
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Excc_s {
    pub expr: *mut Expr_t,
    pub disc: *mut Exdisc_t,
    pub id: *mut libc::c_char,
    pub lastop: libc::c_int,
    pub tmp: libc::c_int,
    pub ccdisc: *mut Exccdisc_t,
}
pub type Exccdisc_t = Exccdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exccdisc_s {
    pub text: *mut Sfio_t,
    pub id: *mut libc::c_char,
    pub flags: uint64_t,
    pub ccf: Option::<
        unsafe extern "C" fn(
            *mut Excc_t,
            *mut Exnode_t,
            *mut Exid_t,
            *mut Exref_t,
            *mut Exnode_t,
            *mut Exccdisc_t,
        ) -> libc::c_int,
    >,
}
pub type Excc_t = Excc_s;
static mut quote: [libc::c_char; 2] = unsafe {
    *::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"\"\0")
};
#[no_mangle]
pub unsafe extern "C" fn exopname(mut op: libc::c_int) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 16] = [0; 16];
    match op {
        33 => return b"!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        37 => return b"%\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        38 => return b"&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        40 => return b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        42 => return b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        43 => return b"+\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        44 => return b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        45 => return b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        47 => return b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        58 => return b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        60 => return b"<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        61 => return b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        62 => return b">\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        63 => return b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        94 => return b"^\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        124 => return b"|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        126 => return b"~\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        324 => return b"&&\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        325 => return b"==\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        328 => return b">=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        327 => return b"<=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        329 => return b"<<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        326 => return b"!=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        323 => return b"||\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        330 => return b">>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => {}
    }
    snprintf(
        buf.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"(OP=%03o)\0" as *const u8 as *const libc::c_char,
        op,
    );
    return buf.as_mut_ptr();
}
unsafe extern "C" fn print(mut cc: *mut Excc_t, mut expr: *mut Exnode_t) {
    let mut x: *mut Print_t = 0 as *mut Print_t;
    x = (*expr).data.print.args;
    if !x.is_null() {
        sfprintf(
            (*(*cc).ccdisc).text,
            b"sfprintf(%s, \"%s\0" as *const u8 as *const libc::c_char,
            if (*(*expr).data.print.descriptor).op == 271 as libc::c_int
                && (*(*expr).data.print.descriptor).data.constant.value.integer
                    == 2 as libc::c_int as libc::c_longlong
            {
                b"sfstderr\0" as *const u8 as *const libc::c_char
            } else {
                b"sfstdout\0" as *const u8 as *const libc::c_char
            },
            fmtesq((*x).format, quote.as_ptr()),
        );
        loop {
            x = (*x).next;
            if x.is_null() {
                break;
            }
            sfprintf(
                (*(*cc).ccdisc).text,
                b"%s\0" as *const u8 as *const libc::c_char,
                fmtesq((*x).format, quote.as_ptr()),
            );
        }
        sfprintf((*(*cc).ccdisc).text, b"\"\0" as *const u8 as *const libc::c_char);
        x = (*expr).data.print.args;
        while !x.is_null() {
            if !((*x).arg).is_null() {
                let mut i: size_t = 0 as libc::c_int as size_t;
                while i
                    < (::std::mem::size_of::<[*mut Exnode_s; 3]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut Exnode_s>() as libc::c_ulong,
                        ) && !((*x).param[i as usize]).is_null()
                {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b", (\0" as *const u8 as *const libc::c_char,
                    );
                    gen(cc, (*x).param[i as usize]);
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b")\0" as *const u8 as *const libc::c_char,
                    );
                    i = i.wrapping_add(1);
                }
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b", (\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*x).arg);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b")\0" as *const u8 as *const libc::c_char,
                );
            }
            x = (*x).next;
        }
        sfprintf((*(*cc).ccdisc).text, b");\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn scan(mut cc: *mut Excc_t, mut expr: *mut Exnode_t) {
    let mut x: *mut Print_t = 0 as *mut Print_t;
    x = (*expr).data.print.args;
    if !x.is_null() {
        sfprintf(
            (*(*cc).ccdisc).text,
            b"sfscanf(sfstdin, \"%s\0" as *const u8 as *const libc::c_char,
            fmtesq((*x).format, quote.as_ptr()),
        );
        loop {
            x = (*x).next;
            if x.is_null() {
                break;
            }
            sfprintf(
                (*(*cc).ccdisc).text,
                b"%s\0" as *const u8 as *const libc::c_char,
                fmtesq((*x).format, quote.as_ptr()),
            );
        }
        sfprintf((*(*cc).ccdisc).text, b"\"\0" as *const u8 as *const libc::c_char);
        x = (*expr).data.print.args;
        while !x.is_null() {
            if !((*x).arg).is_null() {
                let mut i: size_t = 0 as libc::c_int as size_t;
                while i
                    < (::std::mem::size_of::<[*mut Exnode_s; 3]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut Exnode_s>() as libc::c_ulong,
                        ) && !((*x).param[i as usize]).is_null()
                {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b", &(\0" as *const u8 as *const libc::c_char,
                    );
                    gen(cc, (*x).param[i as usize]);
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b")\0" as *const u8 as *const libc::c_char,
                    );
                    i = i.wrapping_add(1);
                }
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b", &(\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*x).arg);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b")\0" as *const u8 as *const libc::c_char,
                );
            }
            x = (*x).next;
        }
        sfprintf((*(*cc).ccdisc).text, b");\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn gen(mut cc: *mut Excc_t, mut expr: *mut Exnode_t) {
    let mut x: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut y: *mut Exnode_t = 0 as *mut Exnode_t;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut Extype_t = 0 as *mut Extype_t;
    let mut p: *mut *mut Extype_t = 0 as *mut *mut Extype_t;
    if expr.is_null() {
        return;
    }
    if (*expr).op == 269 as libc::c_int {
        sfprintf(
            (*(*cc).ccdisc).text,
            b"%s(\0" as *const u8 as *const libc::c_char,
            ((*(*expr).data.call.procedure).name).as_mut_ptr(),
        );
        if !((*expr).data.call.args).is_null() {
            gen(cc, (*expr).data.call.args);
        }
        sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
        return;
    }
    x = (*expr).data.operand.left;
    match (*expr).op {
        268 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"break;\n\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        272 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"continue;\n\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        271 => {
            match (*expr).type_0 {
                262 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"%g\0" as *const u8 as *const libc::c_char,
                        (*expr).data.constant.value.floating,
                    );
                }
                263 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"\"%s\"\0" as *const u8 as *const libc::c_char,
                        fmtesq((*expr).data.constant.value.string, quote.as_ptr()),
                    );
                }
                260 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"%I*u\0" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
                        (*expr).data.constant.value.integer,
                    );
                }
                _ => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"%I*d\0" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
                        (*expr).data.constant.value.integer,
                    );
                }
            }
            return;
        }
        334 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"%s--\0" as *const u8 as *const libc::c_char,
                ((*(*x).data.variable.symbol).name).as_mut_ptr(),
            );
            return;
        }
        275 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"%s\0" as *const u8 as *const libc::c_char,
                ((*(*expr).data.variable.symbol).name).as_mut_ptr(),
            );
            return;
        }
        277 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"exit(\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf(
                (*(*cc).ccdisc).text,
                b");\n\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        279 => {
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b"(\0" as *const u8 as *const libc::c_char);
            y = (*expr).data.operand.right;
            if !y.is_null() {
                gen(cc, y);
            }
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        295 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"rand();\n\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        300 => {
            if (*expr).binary != 0 {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"srand(\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, x);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b");\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"srand();\n\0" as *const u8 as *const libc::c_char,
                );
            }
            return;
        }
        280 | 302 | 303 => {
            s = (if (*expr).op == 280 as libc::c_int {
                b"gsub(\0" as *const u8 as *const libc::c_char
            } else if (*expr).op == 302 as libc::c_int {
                b"sub(\0" as *const u8 as *const libc::c_char
            } else {
                b"substr(\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char;
            sfprintf((*(*cc).ccdisc).text, s);
            gen(cc, (*expr).data.string.base);
            sfprintf((*(*cc).ccdisc).text, b", \0" as *const u8 as *const libc::c_char);
            gen(cc, (*expr).data.string.pat);
            if !((*expr).data.string.repl).is_null() {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b", \0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*expr).data.string.repl);
            }
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        331 => {
            gen(cc, (*expr).data.variable.index);
            sfprintf(
                (*(*cc).ccdisc).text,
                b" in %s\0" as *const u8 as *const libc::c_char,
                ((*(*expr).data.variable.symbol).name).as_mut_ptr(),
            );
            return;
        }
        284 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"if (\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf(
                (*(*cc).ccdisc).text,
                b") {\n\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, (*(*expr).data.operand.right).data.operand.left);
            if !((*(*expr).data.operand.right).data.operand.right).is_null() {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"} else {\n\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*(*expr).data.operand.right).data.operand.right);
            }
            sfprintf((*(*cc).ccdisc).text, b"}\n\0" as *const u8 as *const libc::c_char);
            return;
        }
        278 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"for (;\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b");\0" as *const u8 as *const libc::c_char);
            if !((*expr).data.operand.left).is_null() {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"(\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*expr).data.operand.left);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b")\0" as *const u8 as *const libc::c_char,
                );
            }
            sfprintf((*(*cc).ccdisc).text, b") {\0" as *const u8 as *const libc::c_char);
            if !((*expr).data.operand.right).is_null() {
                gen(cc, (*expr).data.operand.right);
            }
            sfprintf((*(*cc).ccdisc).text, b"}\0" as *const u8 as *const libc::c_char);
            return;
        }
        283 => {
            if ((*(*cc).ccdisc).ccf).is_some() {
                (Some(((*(*cc).ccdisc).ccf).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    cc,
                    expr,
                    (*expr).data.variable.symbol,
                    (*expr).data.variable.reference,
                    (*expr).data.variable.index,
                    (*cc).ccdisc,
                );
            } else {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    ((*(*expr).data.variable.symbol).name).as_mut_ptr(),
                );
            }
            return;
        }
        333 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"%s++\0" as *const u8 as *const libc::c_char,
                ((*(*x).data.variable.symbol).name).as_mut_ptr(),
            );
            return;
        }
        281 | 282 => {
            if (*expr).op == 275 as libc::c_int {
                let ref mut fresh0 = (*cc).tmp;
                *fresh0 += 1;
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"{ Exassoc_t* %stmp_%d;\0" as *const u8 as *const libc::c_char,
                    (*cc).id,
                    *fresh0,
                );
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"for (%stmp_%d = (Exassoc_t*)dtfirst(%s); %stmp_%d && (%s = %stmp_%d->name); %stmp_%d = (Exassoc_t*)dtnext(%s, %stmp_%d)) {\0"
                        as *const u8 as *const libc::c_char,
                    (*cc).id,
                    (*cc).tmp,
                    ((*(*(*expr).data.generate.array).data.variable.symbol).name)
                        .as_mut_ptr(),
                    (*cc).id,
                    (*cc).tmp,
                    ((*(*expr).data.generate.index).name).as_mut_ptr(),
                    (*cc).id,
                    (*cc).tmp,
                    (*cc).id,
                    (*cc).tmp,
                    ((*(*(*expr).data.generate.array).data.variable.symbol).name)
                        .as_mut_ptr(),
                    (*cc).id,
                    (*cc).tmp,
                );
                gen(cc, (*expr).data.generate.statement);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"} }\0" as *const u8 as *const libc::c_char,
                );
            }
            return;
        }
        291 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"print\0" as *const u8 as *const libc::c_char,
            );
            if !x.is_null() {
                gen(cc, x);
            } else {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"()\0" as *const u8 as *const libc::c_char,
                );
            }
            return;
        }
        292 => {
            print(cc, expr);
            return;
        }
        296 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"return(\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf(
                (*(*cc).ccdisc).text,
                b");\n\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        297 => {
            scan(cc, expr);
            return;
        }
        298 | 305 => {
            if (*expr).op == 298 as libc::c_int {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"split (\0" as *const u8 as *const libc::c_char,
                );
            } else {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"tokens (\0" as *const u8 as *const libc::c_char,
                );
            }
            gen(cc, (*expr).data.split.string);
            sfprintf(
                (*(*cc).ccdisc).text,
                b", %s\0" as *const u8 as *const libc::c_char,
                ((*(*expr).data.split.array).name).as_mut_ptr(),
            );
            if !((*expr).data.split.seps).is_null() {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b",\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*expr).data.split.seps);
            }
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        304 => {
            t = (*x).type_0;
            let ref mut fresh1 = (*cc).tmp;
            *fresh1 += 1;
            sfprintf(
                (*(*cc).ccdisc).text,
                b"{ %s %stmp_%d = \0" as *const u8 as *const libc::c_char,
                extype(t),
                (*cc).id,
                *fresh1,
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b";\0" as *const u8 as *const libc::c_char);
            x = (*expr).data.operand.right;
            y = (*x).data.select.statement;
            n = 0 as libc::c_int;
            loop {
                x = (*x).data.select.next;
                if x.is_null() {
                    break;
                }
                if n != 0 {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"else \0" as *const u8 as *const libc::c_char,
                    );
                }
                p = (*x).data.select.constant;
                if p.is_null() {
                    y = (*x).data.select.statement;
                } else {
                    m = 0 as libc::c_int;
                    loop {
                        let fresh2 = p;
                        p = p.offset(1);
                        v = *fresh2;
                        if v.is_null() {
                            break;
                        }
                        if m != 0 {
                            sfprintf(
                                (*(*cc).ccdisc).text,
                                b"||\0" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            m = 1 as libc::c_int;
                            sfprintf(
                                (*(*cc).ccdisc).text,
                                b"if (\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if t == 263 as libc::c_int {
                            sfprintf(
                                (*(*cc).ccdisc).text,
                                b"strmatch(%stmp_%d, \"%s\")\0" as *const u8
                                    as *const libc::c_char,
                                (*cc).id,
                                (*cc).tmp,
                                fmtesq((*v).string, quote.as_ptr()),
                            );
                        } else {
                            sfprintf(
                                (*(*cc).ccdisc).text,
                                b"%stmp_%d == \0" as *const u8 as *const libc::c_char,
                                (*cc).id,
                                (*cc).tmp,
                            );
                            match t {
                                259 | 260 => {
                                    sfprintf(
                                        (*(*cc).ccdisc).text,
                                        b"%I*u\0" as *const u8 as *const libc::c_char,
                                        ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
                                        (*v).integer,
                                    );
                                }
                                _ => {
                                    sfprintf(
                                        (*(*cc).ccdisc).text,
                                        b"%g\0" as *const u8 as *const libc::c_char,
                                        (*v).floating,
                                    );
                                }
                            }
                        }
                    }
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b") {\0" as *const u8 as *const libc::c_char,
                    );
                    gen(cc, (*x).data.select.statement);
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"}\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            if !y.is_null() {
                if n != 0 {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"else \0" as *const u8 as *const libc::c_char,
                    );
                }
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"{\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, y);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"}\0" as *const u8 as *const libc::c_char,
                );
            }
            sfprintf((*(*cc).ccdisc).text, b"}\0" as *const u8 as *const libc::c_char);
            return;
        }
        306 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"unset(%s\0" as *const u8 as *const libc::c_char,
                ((*(*expr).data.variable.symbol).name).as_mut_ptr(),
            );
            if !((*expr).data.variable.index).is_null() {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b",\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*expr).data.variable.index);
            }
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        307 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"while (\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b") {\0" as *const u8 as *const libc::c_char);
            if !((*expr).data.operand.right).is_null() {
                gen(cc, (*expr).data.operand.right);
            }
            sfprintf((*(*cc).ccdisc).text, b"}\0" as *const u8 as *const libc::c_char);
            return;
        }
        35 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"# %s\0" as *const u8 as *const libc::c_char,
                ((*(*expr).data.variable.symbol).name).as_mut_ptr(),
            );
            return;
        }
        61 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"(%s%s=\0" as *const u8 as *const libc::c_char,
                ((*(*x).data.variable.symbol).name).as_mut_ptr(),
                if (*expr).subop == '=' as i32 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    exopname((*expr).subop) as *const libc::c_char
                },
            );
            gen(cc, (*expr).data.operand.right);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        59 => {
            loop {
                x = (*expr).data.operand.right;
                if x.is_null() {
                    let ref mut fresh3 = (*cc).lastop;
                    *fresh3 = (*(*expr).data.operand.left).op;
                    match *fresh3 {
                        278 | 284 | 292 | 291 | 296 | 307 => {}
                        _ => {
                            sfprintf(
                                (*(*cc).ccdisc).text,
                                b"_%svalue=\0" as *const u8 as *const libc::c_char,
                                (*cc).id,
                            );
                        }
                    }
                }
                gen(cc, (*expr).data.operand.left);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b";\n\0" as *const u8 as *const libc::c_char,
                );
                expr = x;
                if expr.is_null() {
                    break;
                }
                let ref mut fresh4 = (*cc).lastop;
                *fresh4 = (*expr).op;
                match *fresh4 {
                    59 => {
                        continue;
                    }
                    278 | 284 | 292 | 291 | 296 | 307 => {}
                    _ => {
                        sfprintf(
                            (*(*cc).ccdisc).text,
                            b"_%svalue=\0" as *const u8 as *const libc::c_char,
                            (*cc).id,
                        );
                    }
                }
                gen(cc, expr);
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b";\n\0" as *const u8 as *const libc::c_char,
                );
                break;
            }
            return;
        }
        44 => {
            sfprintf((*(*cc).ccdisc).text, b"(\0" as *const u8 as *const libc::c_char);
            gen(cc, x);
            loop {
                expr = (*expr).data.operand.right;
                if !(!expr.is_null() && (*expr).op == ',' as i32) {
                    break;
                }
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"), (\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, (*expr).data.operand.left);
            }
            if !expr.is_null() {
                sfprintf(
                    (*(*cc).ccdisc).text,
                    b"), (\0" as *const u8 as *const libc::c_char,
                );
                gen(cc, expr);
            }
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        63 => {
            sfprintf((*(*cc).ccdisc).text, b"(\0" as *const u8 as *const libc::c_char);
            gen(cc, x);
            sfprintf(
                (*(*cc).ccdisc).text,
                b") ? (\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, (*(*expr).data.operand.right).data.operand.left);
            sfprintf(
                (*(*cc).ccdisc).text,
                b") : (\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, (*(*expr).data.operand.right).data.operand.right);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        324 => {
            sfprintf((*(*cc).ccdisc).text, b"(\0" as *const u8 as *const libc::c_char);
            gen(cc, x);
            sfprintf(
                (*(*cc).ccdisc).text,
                b") && (\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, (*expr).data.operand.right);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        323 => {
            sfprintf((*(*cc).ccdisc).text, b"(\0" as *const u8 as *const libc::c_char);
            gen(cc, x);
            sfprintf(
                (*(*cc).ccdisc).text,
                b") || (\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, (*expr).data.operand.right);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        308 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"(%s)(\0" as *const u8 as *const libc::c_char,
                extype(259 as libc::c_int),
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        310 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"(%s)(\0" as *const u8 as *const libc::c_char,
                extype(262 as libc::c_int),
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        314 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"strtoll(\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf(
                (*(*cc).ccdisc).text,
                b",(char**)0,0)\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        319 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"X2I(\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        321 => {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"X2X(\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        _ => {}
    }
    y = (*expr).data.operand.right;
    if (*x).type_0 == 263 as libc::c_int {
        's_1297: {
            match (*expr).op {
                312 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"*(\0" as *const u8 as *const libc::c_char,
                    );
                    gen(cc, x);
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b")!=0\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
                313 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"strtod(\0" as *const u8 as *const libc::c_char,
                    );
                    gen(cc, x);
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b",0)\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
                314 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"strtol(\0" as *const u8 as *const libc::c_char,
                    );
                    gen(cc, x);
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b",0,0)\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
                317 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"** cannot convert string value to external **\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                326 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"!\0" as *const u8 as *const libc::c_char,
                    );
                }
                325 => {}
                43 | 124 | 38 | 94 | 37 | 42 => {
                    sfprintf(
                        (*(*cc).ccdisc).text,
                        b"** string bits not supported **\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                _ => {
                    break 's_1297;
                }
            }
            sfprintf(
                (*(*cc).ccdisc).text,
                b"strmatch(\0" as *const u8 as *const libc::c_char,
            );
            gen(cc, x);
            sfprintf((*(*cc).ccdisc).text, b",\0" as *const u8 as *const libc::c_char);
            gen(cc, y);
            sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
        match (*expr).op {
            60 => {
                s = b"<0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            327 => {
                s = b"<=0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            328 => {
                s = b">=0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            62 => {
                s = b">0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            _ => {
                s = b"** unknown string op **\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        }
        sfprintf(
            (*(*cc).ccdisc).text,
            b"strcoll(\0" as *const u8 as *const libc::c_char,
        );
        gen(cc, x);
        sfprintf((*(*cc).ccdisc).text, b",\0" as *const u8 as *const libc::c_char);
        gen(cc, y);
        sfprintf((*(*cc).ccdisc).text, b")%s\0" as *const u8 as *const libc::c_char, s);
        return;
    } else {
        if y.is_null() {
            sfprintf(
                (*(*cc).ccdisc).text,
                b"%s\0" as *const u8 as *const libc::c_char,
                exopname((*expr).op),
            );
        }
        sfprintf((*(*cc).ccdisc).text, b"(\0" as *const u8 as *const libc::c_char);
        gen(cc, x);
        if !y.is_null() {
            sfprintf(
                (*(*cc).ccdisc).text,
                b")%s(\0" as *const u8 as *const libc::c_char,
                exopname((*expr).op),
            );
            gen(cc, y);
        }
        sfprintf((*(*cc).ccdisc).text, b")\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn global(
    mut table: *mut Dt_t,
    mut object: *mut libc::c_void,
    mut handle: *mut libc::c_void,
) -> libc::c_int {
    let mut cc: *mut Excc_t = handle as *mut Excc_t;
    let mut sym: *mut Exid_t = object as *mut Exid_t;
    if (*sym).lex == 275 as libc::c_int as libc::c_long {
        sfprintf(
            (*(*cc).ccdisc).text,
            b"static %s\t%s;\n\0" as *const u8 as *const libc::c_char,
            extype((*sym).type_0 as libc::c_int),
            ((*sym).name).as_mut_ptr(),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn exccopen(
    mut expr: *mut Expr_t,
    mut disc: *mut Exccdisc_t,
) -> *mut Excc_t {
    let mut cc: *mut Excc_t = 0 as *mut Excc_t;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    id = (*disc).id;
    if id.is_null() {
        id = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    cc = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<Excc_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (strlen(id)).wrapping_add(2 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut Excc_t
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<Excc_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (strlen(id)).wrapping_add(2 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut Excc_t
    };
    if cc.is_null() {
        return 0 as *mut Excc_t;
    }
    let ref mut fresh5 = (*cc).expr;
    *fresh5 = expr;
    let ref mut fresh6 = (*cc).disc;
    *fresh6 = (*expr).disc;
    let ref mut fresh7 = (*cc).id;
    *fresh7 = cc.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    let ref mut fresh8 = (*cc).ccdisc;
    *fresh8 = disc;
    if (*disc).flags & 0x8000 as libc::c_int as libc::c_ulong == 0 {
        sfprintf(
            (*disc).text,
            b"/* : : generated by %s : : */\n\0" as *const u8 as *const libc::c_char,
            exversion,
        );
        sfprintf(
            (*disc).text,
            b"\n#include <ast/ast.h>\n\0" as *const u8 as *const libc::c_char,
        );
        if *id != 0 {
            snprintf(
                (*cc).id,
                (strlen(id)).wrapping_add(2 as libc::c_int as libc::c_ulong),
                b"%s_\0" as *const u8 as *const libc::c_char,
                id,
            );
        }
        sfprintf((*disc).text, b"\n\0" as *const u8 as *const libc::c_char);
        dtwalk(
            (*expr).symbols,
            Some(
                global
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            cc as *mut libc::c_void,
        );
    }
    return cc;
}
#[no_mangle]
pub unsafe extern "C" fn exccclose(mut cc: *mut Excc_t) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    if cc.is_null() {
        r = -(1 as libc::c_int);
    } else {
        if (*(*cc).ccdisc).flags & 0x8000 as libc::c_int as libc::c_ulong == 0 {
            if !((*(*cc).ccdisc).text).is_null() {
                sfclose((*(*cc).ccdisc).text);
            } else {
                r = -(1 as libc::c_int);
            }
        }
        free(cc as *mut libc::c_void);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn exdump(
    mut expr: *mut Expr_t,
    mut node: *mut Exnode_t,
    mut sp: *mut Sfio_t,
) -> libc::c_int {
    let mut cc: *mut Excc_t = 0 as *mut Excc_t;
    let mut ccdisc: Exccdisc_t = Exccdisc_t {
        text: 0 as *mut Sfio_t,
        id: 0 as *mut libc::c_char,
        flags: 0,
        ccf: None,
    };
    let mut sym: *mut Exid_t = 0 as *mut Exid_t;
    memset(
        &mut ccdisc as *mut Exccdisc_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Exccdisc_t>() as libc::c_ulong,
    );
    ccdisc.flags = 0x8000 as libc::c_int as uint64_t;
    ccdisc.text = sp;
    cc = exccopen(expr, &mut ccdisc);
    if cc.is_null() {
        return -(1 as libc::c_int);
    }
    if !node.is_null() {
        gen(cc, node);
    } else {
        sym = (Some(((*(*expr).symbols).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*expr).symbols, 0 as *mut libc::c_void, 0o200 as libc::c_int)
            as *mut Exid_t;
        while !sym.is_null() {
            if (*sym).lex == 293 as libc::c_int as libc::c_long
                && !((*sym).value).is_null()
            {
                sfprintf(
                    sp,
                    b"%s:\n\0" as *const u8 as *const libc::c_char,
                    ((*sym).name).as_mut_ptr(),
                );
                gen(cc, (*(*sym).value).data.procedure.body);
            }
            sym = (Some(
                ((*(*expr).symbols).searchf).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )((*expr).symbols, sym as *mut libc::c_void, 0o10 as libc::c_int)
                as *mut Exid_t;
        }
    }
    sfprintf(sp, b"\n\0" as *const u8 as *const libc::c_char);
    return exccclose(cc);
}
