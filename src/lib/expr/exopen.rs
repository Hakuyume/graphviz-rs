#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut sfstdin: *mut Sfio_t;
    static mut sfstdout: *mut Sfio_t;
    static mut sfstderr: *mut Sfio_t;
    fn sfnew(
        _: *mut Sfio_t,
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut Sfio_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut Dtset: *mut Dtmethod_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    fn vmopen() -> *mut Vmalloc_t;
    fn exclose(_: *mut Expr_t, _: libc::c_int);
    static mut exbuiltin: [Exid_t; 0];
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
#[no_mangle]
pub unsafe extern "C" fn exopen(mut disc: *mut Exdisc_t) -> *mut Expr_t {
    let mut program: *mut Expr_t = 0 as *mut Expr_t;
    let mut sym: *mut Exid_t = 0 as *mut Exid_t;
    program = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<Expr_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Expr_t
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<Expr_t>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut Expr_t
    };
    if program.is_null() {
        return 0 as *mut Expr_t;
    }
    (*program).symdisc.key = 88 as libc::c_ulong as libc::c_int;
    let ref mut fresh0 = (*program).symbols;
    *fresh0 = dtopen(&mut (*program).symdisc, Dtset);
    if (*fresh0).is_null()
        || {
            let ref mut fresh1 = (*program).tmp;
            *fresh1 = sfnew(
                0 as *mut Sfio_t,
                0 as *mut libc::c_char as *mut libc::c_void,
                -(1 as libc::c_int) as size_t,
                -(1 as libc::c_int),
                0o2 as libc::c_int | 0o4 as libc::c_int,
            );
            (*fresh1).is_null()
        }
        || {
            let ref mut fresh2 = (*program).vm;
            *fresh2 = vmopen();
            (*fresh2).is_null()
        }
        || {
            let ref mut fresh3 = (*program).ve;
            *fresh3 = vmopen();
            (*fresh3).is_null()
        }
    {
        exclose(program, 1 as libc::c_int);
        return 0 as *mut Expr_t;
    }
    let ref mut fresh4 = (*program).id;
    *fresh4 = b"libexpr:expr\0" as *const u8 as *const libc::c_char;
    let ref mut fresh5 = (*program).disc;
    *fresh5 = disc;
    let ref mut fresh6 = (*program).linep;
    *fresh6 = ((*program).line).as_mut_ptr();
    (*program).linewrap = 0 as libc::c_int;
    let ref mut fresh7 = (*program).file[0 as libc::c_int as usize];
    *fresh7 = sfstdin;
    let ref mut fresh8 = (*program).file[1 as libc::c_int as usize];
    *fresh8 = sfstdout;
    let ref mut fresh9 = (*program).file[2 as libc::c_int as usize];
    *fresh9 = sfstderr;
    strcpy(
        ((*program).main.name).as_mut_ptr(),
        b"main\0" as *const u8 as *const libc::c_char,
    );
    (*program).main.lex = 293 as libc::c_int as libc::c_long;
    (*program).main.index = 293 as libc::c_int as libc::c_long;
    (Some(((*(*program).symbols).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*program).symbols,
        &mut (*program).main as *mut Exid_t as *mut libc::c_void,
        0o1 as libc::c_int,
    );
    if (*disc).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong == 0 {
        sym = exbuiltin.as_mut_ptr();
        while *((*sym).name).as_mut_ptr() != 0 {
            (Some(((*(*program).symbols).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*program).symbols, sym as *mut libc::c_void, 0o1 as libc::c_int);
            sym = sym.offset(1);
        }
    }
    sym = (*disc).symbols;
    if !sym.is_null() {
        while *((*sym).name).as_mut_ptr() != 0 {
            (Some(((*(*program).symbols).searchf).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*program).symbols, sym as *mut libc::c_void, 0o1 as libc::c_int);
            sym = sym.offset(1);
        }
    }
    return program;
}
