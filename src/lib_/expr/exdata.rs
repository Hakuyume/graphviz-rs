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
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
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
static mut id: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
        b"\n@(#)$Id: libexpr (AT&T Research) 2011-06-30 $\0\n\0",
    )
};
#[no_mangle]
pub static mut exversion: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut exbuiltin: [Exid_t; 38] = unsafe {
    [
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 263 as libc::c_int as libc::c_long,
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
                    b"string\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 268 as libc::c_int as libc::c_long,
                index: 268 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"break\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 270 as libc::c_int as libc::c_long,
                index: 270 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"case\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 261 as libc::c_int as libc::c_long,
                type_0: 261 as libc::c_int as libc::c_long,
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
                    b"char\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 272 as libc::c_int as libc::c_long,
                index: 272 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"continue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 274 as libc::c_int as libc::c_long,
                index: 274 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"default\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 262 as libc::c_int as libc::c_long,
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
                    b"double\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 276 as libc::c_int as libc::c_long,
                index: 276 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"else\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 277 as libc::c_int as libc::c_long,
                index: 277 as libc::c_int as libc::c_long,
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
                    b"exit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 278 as libc::c_int as libc::c_long,
                index: 278 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"for\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 282 as libc::c_int as libc::c_long,
                index: 282 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"forr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 262 as libc::c_int as libc::c_long,
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
                    b"float\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 280 as libc::c_int as libc::c_long,
                index: 280 as libc::c_int as libc::c_long,
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
                    b"gsub\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 284 as libc::c_int as libc::c_long,
                index: 284 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"if\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 331 as libc::c_int as libc::c_long,
                index: 331 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"in\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 259 as libc::c_int as libc::c_long,
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
                    b"int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 259 as libc::c_int as libc::c_long,
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
                    b"long\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 291 as libc::c_int as libc::c_long,
                index: 291 as libc::c_int as libc::c_long,
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
                    b"print\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 292 as libc::c_int as libc::c_long,
                index: 292 as libc::c_int as libc::c_long,
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
                    b"printf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 294 as libc::c_int as libc::c_long,
                index: 294 as libc::c_int as libc::c_long,
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
                    b"query\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 295 as libc::c_int as libc::c_long,
                index: 295 as libc::c_int as libc::c_long,
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
                    b"rand\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 296 as libc::c_int as libc::c_long,
                index: 296 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"return\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 297 as libc::c_int as libc::c_long,
                index: 297 as libc::c_int as libc::c_long,
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
                    b"scanf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 301 as libc::c_int as libc::c_long,
                index: 301 as libc::c_int as libc::c_long,
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
                    b"sscanf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 298 as libc::c_int as libc::c_long,
                index: 298 as libc::c_int as libc::c_long,
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
                    b"split\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 299 as libc::c_int as libc::c_long,
                index: 299 as libc::c_int as libc::c_long,
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
                    b"sprintf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 300 as libc::c_int as libc::c_long,
                index: 300 as libc::c_int as libc::c_long,
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
                    b"srand\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 265 as libc::c_int as libc::c_long,
                index: 265 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"static\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 302 as libc::c_int as libc::c_long,
                index: 302 as libc::c_int as libc::c_long,
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
                    b"sub\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 303 as libc::c_int as libc::c_long,
                index: 303 as libc::c_int as libc::c_long,
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
                    b"substr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 304 as libc::c_int as libc::c_long,
                index: 304 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"switch\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 305 as libc::c_int as libc::c_long,
                index: 305 as libc::c_int as libc::c_long,
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
                    b"tokens\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 306 as libc::c_int as libc::c_long,
                index: 306 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"unset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 260 as libc::c_int as libc::c_long,
                type_0: 260 as libc::c_int as libc::c_long,
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
                    b"unsigned\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 273 as libc::c_int as libc::c_long,
                index: 264 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"void\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 307 as libc::c_int as libc::c_long,
                index: 307 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"while\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
                    };
                    init
                },
                lex: 307 as libc::c_int as libc::c_long,
                index: 307 as libc::c_int as libc::c_long,
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
                name: *::std::mem::transmute::<&[u8; 32], &mut [libc::c_char; 32]>(
                    b"while\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                ),
            };
            init
        },
        {
            let mut init = Exid_s {
                link: {
                    let mut init = _dtlink_s {
                        right: 0 as *const Dtlink_t as *mut Dtlink_t,
                        hl: C2RustUnnamed_2 { _hash: 0 },
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
unsafe extern "C" fn run_static_initializers() {
    exversion = id.as_ptr().offset(10 as libc::c_int as isize);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
