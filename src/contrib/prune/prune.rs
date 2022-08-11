#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut optind: libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agdelnode(g: *mut Agraph_t, arg_n: *mut Agnode_t) -> libc::c_int;
    fn agdeledge(g: *mut Agraph_t, arg_e: *mut Agedge_t) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    fn new_generic_list(size: uint64_t) -> *mut generic_list_t;
    fn add_to_generic_list(
        list: *mut generic_list_t,
        element: gl_data,
    ) -> *mut generic_list_t;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtlink_s {
    pub right: *mut Dtlink_t,
    pub hl: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub graph: C2RustUnnamed_2,
    pub node: C2RustUnnamed_2,
    pub edge: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub u: C2RustUnnamed_3,
    pub ctr: libc::c_int,
    pub ingraphs: libc::c_int,
    pub fp: *mut libc::c_void,
    pub fns: *mut ingdisc,
    pub heap: bool,
    pub errors: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub Files: *mut *mut libc::c_char,
    pub Graphs: *mut *mut Agraph_t,
}
pub type gl_data = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct generic_list_s {
    pub used: uint64_t,
    pub size: uint64_t,
    pub data: *mut gl_data,
}
pub type generic_list_t = generic_list_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strattr_s {
    pub n: *mut libc::c_char,
    pub v: *mut libc::c_char,
}
pub type strattr_t = strattr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndata {
    pub h: Agrec_t,
    pub mark: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[no_mangle]
pub static mut verbose: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut progname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ig: ingraph_state = ingraph_state {
        u: C2RustUnnamed_3 {
            Files: 0 as *mut *mut libc::c_char,
        },
        ctr: 0,
        ingraphs: 0,
        fp: 0 as *mut libc::c_void,
        fns: 0 as *mut ingdisc,
        heap: false,
        errors: 0,
    };
    let mut graph: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut node: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut edge: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut nexte: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut attr: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut files: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut attr_list: *mut generic_list_t = 0 as *mut generic_list_t;
    let mut node_list: *mut generic_list_t = 0 as *mut generic_list_t;
    let mut i: libc::c_ulong = 0;
    let mut j: libc::c_ulong = 0;
    opterr = 0 as libc::c_int;
    progname = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if progname.is_null() {
        progname = *argv.offset(0 as libc::c_int as isize);
    } else {
        progname = progname.offset(1);
    }
    attr_list = new_generic_list(16 as libc::c_int as uint64_t);
    node_list = new_generic_list(16 as libc::c_int as uint64_t);
    loop {
        c = getopt(argc, argv, b"hvn:N:\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            78 => {
                attr_list = addattr(attr_list, optarg);
            }
            110 => {
                node_list = addnode(node_list, optarg);
            }
            104 => {
                help_message(progname);
                graphviz_exit(0 as libc::c_int);
            }
            118 => {
                verbose = 1 as libc::c_int;
            }
            63 => {
                if optopt == '?' as i32 {
                    help_message(progname);
                    graphviz_exit(0 as libc::c_int);
                } else {
                    if *(*__ctype_b_loc()).offset(optopt as isize) as libc::c_int
                        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        fprintf(
                            stderr,
                            b"Unknown option `-%c'.\n\0" as *const u8
                                as *const libc::c_char,
                            optopt,
                        );
                    } else {
                        fprintf(
                            stderr,
                            b"Unknown option character `\\x%X'.\n\0" as *const u8
                                as *const libc::c_char,
                            optopt,
                        );
                    }
                }
                graphviz_exit(1 as libc::c_int);
            }
            _ => {
                help_message(progname);
                graphviz_exit(1 as libc::c_int);
            }
        }
    }
    if optind < argc {
        files = &mut *argv.offset(optind as isize) as *mut *mut libc::c_char;
    } else {
        files = 0 as *mut *mut libc::c_char;
    }
    newIngraph(
        &mut ig,
        files,
        Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
    );
    loop {
        graph = nextGraph(&mut ig);
        if graph.is_null() {
            break;
        }
        if agisdirected(graph) == 0 as libc::c_int {
            fprintf(
                stderr,
                b"*** Error: Graph is undirected! Pruning works only with directed graphs!\n\0"
                    as *const u8 as *const libc::c_char,
            );
            graphviz_exit(1 as libc::c_int);
        }
        aginit(
            graph,
            1 as libc::c_int,
            b"mk\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<ndata>() as libc::c_ulong as libc::c_int,
            1 as libc::c_int,
        );
        i = 0 as libc::c_int as libc::c_ulong;
        while i < (*node_list).used {
            if verbose == 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"Pruning node %s\n\0" as *const u8 as *const libc::c_char,
                    *((*node_list).data).offset(i as isize) as *mut libc::c_char,
                );
            }
            node = agnode(
                graph,
                *((*node_list).data).offset(i as isize) as *mut libc::c_char,
                0 as libc::c_int,
            );
            if node.is_null() {
                fprintf(
                    stderr,
                    b"*** Warning: No such node: %s -- gracefully skipping this one\n\0"
                        as *const u8 as *const libc::c_char,
                    *((*node_list).data).offset(i as isize) as *mut libc::c_char,
                );
            } else {
                (*((*(node as *mut Agobj_t)).data as *mut ndata))
                    .mark = 1 as libc::c_int;
                edge = agfstout(graph, node);
                while !edge.is_null() {
                    nexte = agnxtout(graph, edge);
                    if (*(if ((*(edge as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        edge
                    } else {
                        edge.offset(-(1 as libc::c_int as isize))
                    }))
                        .node != node
                    {
                        if verbose == 1 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Processing descendant: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                agnameof(
                                    (*if ((*(edge as *mut Agobj_t)).tag).objtype()
                                        as libc::c_int == 2 as libc::c_int
                                    {
                                        edge
                                    } else {
                                        edge.offset(-(1 as libc::c_int as isize))
                                    })
                                        .node as *mut libc::c_void,
                                ),
                            );
                        }
                        if remove_child(
                            graph,
                            (*if ((*(edge as *mut Agobj_t)).tag).objtype() as libc::c_int
                                == 2 as libc::c_int
                            {
                                edge
                            } else {
                                edge.offset(-(1 as libc::c_int as isize))
                            })
                                .node,
                        ) == 0
                        {
                            agdelete(graph, edge as *mut libc::c_void);
                        }
                    }
                    edge = nexte;
                }
                (*((*(node as *mut Agobj_t)).data as *mut ndata))
                    .mark = 0 as libc::c_int;
                j = 0 as libc::c_int as libc::c_ulong;
                while j < (*attr_list).used {
                    attr = agattr(
                        graph,
                        1 as libc::c_int,
                        (*(*((*attr_list).data).offset(j as isize) as *mut strattr_t)).n,
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    if attr.is_null() {
                        fprintf(
                            stderr,
                            b"Couldn't create attribute: %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*(*((*attr_list).data).offset(j as isize)
                                as *mut strattr_t))
                                .n,
                        );
                        graphviz_exit(1 as libc::c_int);
                    }
                    agxset(
                        node as *mut libc::c_void,
                        attr,
                        (*(*((*attr_list).data).offset(j as isize) as *mut strattr_t)).v,
                    );
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
        agwrite(graph, stdout as *mut libc::c_void);
        agclose(graph);
    }
    free(attr_list as *mut libc::c_void);
    free(node_list as *mut libc::c_void);
    graphviz_exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn remove_child(
    mut graph: *mut Agraph_t,
    mut node: *mut Agnode_t,
) -> libc::c_int {
    let mut edge: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut nexte: *mut Agedge_t = 0 as *mut Agedge_t;
    if (*((*(node as *mut Agobj_t)).data as *mut ndata)).mark & 1 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    (*((*(node as *mut Agobj_t)).data as *mut ndata)).mark = 1 as libc::c_int;
    edge = agfstin(graph, node);
    if !edge.is_null() && !(agnxtin(graph, edge)).is_null() {
        (*((*(node as *mut Agobj_t)).data as *mut ndata)).mark = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    edge = agfstout(graph, node);
    while !edge.is_null() {
        nexte = agnxtout(graph, edge);
        if (*(if ((*(edge as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            edge
        } else {
            edge.offset(-(1 as libc::c_int as isize))
        }))
            .node != node
        {
            if verbose != 0 {
                fprintf(
                    stderr,
                    b"Processing descendant: %s\n\0" as *const u8 as *const libc::c_char,
                    agnameof(
                        (*if ((*(edge as *mut Agobj_t)).tag).objtype() as libc::c_int
                            == 2 as libc::c_int
                        {
                            edge
                        } else {
                            edge.offset(-(1 as libc::c_int as isize))
                        })
                            .node as *mut libc::c_void,
                    ),
                );
            }
            if remove_child(
                graph,
                (*if ((*(edge as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    edge
                } else {
                    edge.offset(-(1 as libc::c_int as isize))
                })
                    .node,
            ) == 0
            {
                agdeledge(graph, edge);
            }
        }
        edge = nexte;
    }
    agdelnode(graph, node);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn help_message(mut progname: *const libc::c_char) {
    fprintf(
        stderr,
        b"Usage: %s [options] [<files>]\n\nOptions:\n  -h :           Print this message\n  -? :           Print this message\n  -v :           Verbose\n  -n<node> :     Name node to prune.\n  -N<attrspec> : Attribute specification to apply to pruned nodes\n\nBoth options `-n' and `-N' can be used multiple times on the command line.\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
#[no_mangle]
pub unsafe extern "C" fn addattr(
    mut l: *mut generic_list_t,
    mut a: *mut libc::c_char,
) -> *mut generic_list_t {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut strattr_t = 0 as *mut strattr_t;
    sp = malloc(::std::mem::size_of::<strattr_t>() as libc::c_ulong) as *mut strattr_t;
    if sp.is_null() {
        perror(b"[addattr()->malloc()]\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    p = strchr(a, '=' as i32);
    if p.is_null() {
        fprintf(
            stderr,
            b"Invalid argument specification: %s\n\0" as *const u8
                as *const libc::c_char,
            a,
        );
        graphviz_exit(1 as libc::c_int);
    }
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '\0' as i32 as libc::c_char;
    let ref mut fresh1 = (*sp).n;
    *fresh1 = strdup(a);
    if ((*sp).n).is_null() {
        perror(b"[addattr()->strdup()]\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    let ref mut fresh2 = (*sp).v;
    *fresh2 = strdup(p);
    if ((*sp).v).is_null() {
        perror(b"[addattr()->strdup()]\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return add_to_generic_list(l, sp as gl_data);
}
#[no_mangle]
pub unsafe extern "C" fn addnode(
    mut l: *mut generic_list_t,
    mut n: *mut libc::c_char,
) -> *mut generic_list_t {
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    sp = strdup(n);
    if sp.is_null() {
        perror(b"[addnode()->strdup()]\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return add_to_generic_list(l, sp as gl_data);
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
