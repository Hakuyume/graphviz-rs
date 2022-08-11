#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut opterr: libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn gxl_to_gv(_: *mut FILE) -> *mut Agraph_t;
    fn gv_to_gxl(_: *mut Agraph_t, _: *mut FILE);
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
}
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
    pub u: C2RustUnnamed_2,
    pub ctr: libc::c_int,
    pub ingraphs: libc::c_int,
    pub fp: *mut libc::c_void,
    pub fns: *mut ingdisc,
    pub heap: bool,
    pub errors: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub Files: *mut *mut libc::c_char,
    pub Graphs: *mut *mut Agraph_t,
}
pub type mode = libc::c_uint;
pub const ToGXL: mode = 2;
pub const ToGV: mode = 1;
pub const Unset: mode = 0;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut outFile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut CmdName: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut act: mode = Unset;
unsafe extern "C" fn getFile() -> *mut FILE {
    let mut rv: *mut FILE = 0 as *mut FILE;
    static mut savef: *mut FILE = 0 as *const FILE as *mut FILE;
    static mut cnt: libc::c_int = 0 as libc::c_int;
    if Files.is_null() {
        let fresh0 = cnt;
        cnt = cnt + 1;
        if fresh0 == 0 as libc::c_int {
            rv = stdin;
        }
    } else {
        if !savef.is_null() {
            fclose(savef);
        }
        while !(*Files.offset(cnt as isize)).is_null() {
            let fresh1 = cnt;
            cnt = cnt + 1;
            rv = fopen(
                *Files.offset(fresh1 as isize),
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if !rv.is_null() {
                break;
            }
            fprintf(
                stderr,
                b"Can't open %s\n\0" as *const u8 as *const libc::c_char,
                *Files.offset((cnt - 1 as libc::c_int) as isize),
            );
        }
    }
    savef = rv;
    return rv;
}
unsafe extern "C" fn openFile(mut name: *const libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"%s: could not open file %s for writing\n\0" as *const u8
                as *const libc::c_char,
            CmdName,
            name,
        );
        perror(name);
        graphviz_exit(1 as libc::c_int);
    }
    return fp;
}
static mut use_0: *const libc::c_char = b"Usage: %s [-gd?] [-o<file>] [<graphs>]\n -g        : convert to GXL\n -d        : convert to GV\n -o<file>  : output to <file> (stdout)\n -?        : usage\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    fprintf(stderr, use_0, CmdName);
    graphviz_exit(v);
}
unsafe extern "C" fn cmdName(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    sp = strrchr(path, '/' as i32);
    if !sp.is_null() {
        sp = sp.offset(1);
    } else {
        sp = path;
    }
    return sp;
}
unsafe extern "C" fn checkInput() {
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    ep = strrchr(*Files, '.' as i32);
    if ep.is_null() {
        return;
    }
    ep = ep.offset(1);
    if strcmp(ep, b"gv\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        act = ToGXL;
    } else if strcmp(ep, b"dot\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
        act = ToGXL;
    } else if strcmp(ep, b"gxl\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
        act = ToGV;
    }
}
unsafe extern "C" fn setAction() {
    if ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = *CmdName.offset(0 as libc::c_int as isize)
                    as libc::c_int;
                __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                });
            } else {
                __res = tolower(
                    *CmdName.offset(0 as libc::c_int as isize) as libc::c_int,
                );
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(
                    *CmdName.offset(0 as libc::c_int as isize) as libc::c_int as isize,
                );
        }
        __res
    }) == 'd' as i32
    {
        act = ToGXL;
    } else if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *CmdName.offset(0 as libc::c_int as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(
                        *CmdName.offset(0 as libc::c_int as isize) as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *CmdName.offset(0 as libc::c_int as isize) as libc::c_int
                            as isize,
                    );
            }
            __res
        }) == 'g' as i32
        {
        if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *CmdName.offset(1 as libc::c_int as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(
                        *CmdName.offset(1 as libc::c_int as isize) as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *CmdName.offset(1 as libc::c_int as isize) as libc::c_int
                            as isize,
                    );
            }
            __res
        }) == 'v' as i32
        {
            act = ToGXL;
        } else {
            act = ToGV;
        }
    } else if !Files.is_null() {
        checkInput();
    }
    if act as libc::c_uint == Unset as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"Cannot determine conversion type\n\0" as *const u8 as *const libc::c_char,
        );
        usage(1 as libc::c_int);
    }
}
unsafe extern "C" fn initargs(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    CmdName = cmdName(*argv.offset(0 as libc::c_int as isize));
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(argc, argv, b":gdo:\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            100 => {
                act = ToGV;
            }
            103 => {
                act = ToGXL;
            }
            111 => {
                if !outFile.is_null() {
                    fclose(outFile);
                }
                outFile = openFile(optarg);
            }
            58 => {
                fprintf(
                    stderr,
                    b"%s: option -%c missing argument\n\0" as *const u8
                        as *const libc::c_char,
                    CmdName,
                    optopt,
                );
            }
            63 => {
                if optopt == '?' as i32 {
                    usage(0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b"%s: option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        CmdName,
                        optopt,
                    );
                    graphviz_exit(1 as libc::c_int);
                }
            }
            _ => {
                fprintf(
                    stderr,
                    b"cvtgxl: unexpected error\n\0" as *const u8 as *const libc::c_char,
                );
                graphviz_exit(1 as libc::c_int);
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc > 0 as libc::c_int {
        Files = argv;
    }
    if outFile.is_null() {
        outFile = stdout;
    }
    if act as libc::c_uint == Unset as libc::c_int as libc::c_uint {
        setAction();
    }
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut G: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut prev: *mut Agraph_t = 0 as *mut Agraph_t;
    initargs(argc, argv);
    if act as libc::c_uint == ToGXL as libc::c_int as libc::c_uint {
        let mut ig: ingraph_state = ingraph_state {
            u: C2RustUnnamed_2 {
                Files: 0 as *mut *mut libc::c_char,
            },
            ctr: 0,
            ingraphs: 0,
            fp: 0 as *mut libc::c_void,
            fns: 0 as *mut ingdisc,
            heap: false,
            errors: 0,
        };
        newIngraph(
            &mut ig,
            Files,
            Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
        );
        loop {
            G = nextGraph(&mut ig);
            if G.is_null() {
                break;
            }
            if !prev.is_null() {
                agclose(prev);
            }
            prev = G;
            gv_to_gxl(G, outFile);
            fflush(outFile);
        }
    } else {
        let mut inFile: *mut FILE = 0 as *mut FILE;
        loop {
            inFile = getFile();
            if inFile.is_null() {
                break;
            }
            loop {
                G = gxl_to_gv(inFile);
                if G.is_null() {
                    break;
                }
                if !prev.is_null() {
                    agclose(prev);
                }
                prev = G;
                agwrite(G, outFile as *mut libc::c_void);
                fflush(outFile);
            }
        }
    }
    graphviz_exit(0 as libc::c_int);
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
