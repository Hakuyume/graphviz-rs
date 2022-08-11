#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
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
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn agset(
        obj: *mut libc::c_void,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agnnodes(g: *mut Agraph_t) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    static mut opterr: libc::c_int;
    static mut optind: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn colorxlate(str: *mut libc::c_char, buf: *mut libc::c_char) -> *mut libc::c_char;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agnodeinfo_t {
    pub h: Agrec_t,
    pub relrank: libc::c_double,
    pub x: [libc::c_double; 3],
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
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[no_mangle]
pub static mut Defcolor: [libc::c_double; 3] = [0.0f64, 0.0f64, 1.0f64];
#[no_mangle]
pub static mut Forward: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut LR: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut AdjustSaturation: libc::c_int = 0;
#[no_mangle]
pub static mut MinRankSaturation: libc::c_double = 0.;
#[no_mangle]
pub static mut MaxRankSaturation: libc::c_double = 0.;
unsafe extern "C" fn cmpf(
    mut n0: *mut *mut Agnode_t,
    mut n1: *mut *mut Agnode_t,
) -> libc::c_int {
    let mut t: libc::c_double = 0.;
    t = (*((**n0).base.data as *mut Agnodeinfo_t)).relrank
        - (*((**n1).base.data as *mut Agnodeinfo_t)).relrank;
    if t < 0.0f64 {
        return -(1 as libc::c_int);
    }
    if t > 0.0f64 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn setcolor(mut p: *mut libc::c_char, mut v: *mut libc::c_double) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    if sscanf(
        p,
        b"%lf %lf %lf\0" as *const u8 as *const libc::c_char,
        v,
        v.offset(1 as libc::c_int as isize),
        v.offset(2 as libc::c_int as isize),
    ) != 3 as libc::c_int && *p.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        colorxlate(p, buf.as_mut_ptr());
        sscanf(
            buf.as_mut_ptr(),
            b"%lf %lf %lf\0" as *const u8 as *const libc::c_char,
            v,
            v.offset(1 as libc::c_int as isize),
            v.offset(2 as libc::c_int as isize),
        );
    }
}
static mut Files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut useString: *mut libc::c_char = b"Usage: gvcolor [-?] <files>\n  -? - print usage\nIf no files are specified, stdin is used\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    printf(b"%s\0" as *const u8 as *const libc::c_char, useString);
    graphviz_exit(v);
}
unsafe extern "C" fn init(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b":?\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    usage(0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b"gvcolor: option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        optopt,
                    );
                    usage(1 as libc::c_int);
                }
            }
            _ => {
                fprintf(
                    stderr,
                    b"gvcolor: unexpected error\n\0" as *const u8 as *const libc::c_char,
                );
                graphviz_exit(1 as libc::c_int);
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc != 0 {
        Files = argv;
    }
}
unsafe extern "C" fn color(mut g: *mut Agraph_t) {
    let mut nn: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut v: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut nlist: *mut *mut Agnode_t = 0 as *mut *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut maxrank: libc::c_double = 0.0f64;
    let mut sum: [libc::c_double; 3] = [0.; 3];
    let mut d: libc::c_double = 0.;
    let mut lowsat: libc::c_double = 0.;
    let mut highsat: libc::c_double = 0.;
    if (agattr(
        g,
        1 as libc::c_int,
        b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    ))
        .is_null()
    {
        fprintf(
            stderr,
            b"graph must be run through 'dot' before 'gvcolor'\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    aginit(
        g,
        1 as libc::c_int,
        b"nodeinfo\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    if (agattr(
        g,
        1 as libc::c_int,
        b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    ))
        .is_null()
    {
        agattr(
            g,
            1 as libc::c_int,
            b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"filled\0" as *const u8 as *const libc::c_char,
        );
    }
    p = agget(
        g as *mut libc::c_void,
        b"Defcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        setcolor(p, Defcolor.as_mut_ptr());
    }
    p = agget(
        g as *mut libc::c_void,
        b"rankdir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32
    {
        LR = 1 as libc::c_int;
    }
    p = agget(
        g as *mut libc::c_void,
        b"flow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'b' as i32
    {
        Forward = 0 as libc::c_int;
    }
    p = agget(
        g as *mut libc::c_void,
        b"saturation\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !p.is_null() {
        if sscanf(
            p,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut lowsat as *mut libc::c_double,
            &mut highsat as *mut libc::c_double,
        ) == 2 as libc::c_int
        {
            MinRankSaturation = lowsat;
            MaxRankSaturation = highsat;
            AdjustSaturation = 1 as libc::c_int;
        }
    }
    nn = agnnodes(g);
    if nn >= 0 as libc::c_int {} else {
        __assert_fail(
            b"nn >= 0\0" as *const u8 as *const libc::c_char,
            b"gvcolor.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void color(Agraph_t *)\0"))
                .as_ptr(),
        );
    }
    let mut nnodes: size_t = nn as size_t;
    nlist = malloc(
        nnodes.wrapping_mul(::std::mem::size_of::<*mut Agnode_t>() as libc::c_ulong),
    ) as *mut *mut Agnode_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    n = agfstnode(g);
    while !n.is_null() {
        let fresh0 = i;
        i = i.wrapping_add(1);
        let ref mut fresh1 = *nlist.offset(fresh0 as isize);
        *fresh1 = n;
        p = agget(
            n as *mut libc::c_void,
            b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !p.is_null() {
            setcolor(p, ((*((*n).base.data as *mut Agnodeinfo_t)).x).as_mut_ptr());
        }
        p = agget(
            n as *mut libc::c_void,
            b"pos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        sscanf(
            p,
            b"%lf,%lf\0" as *const u8 as *const libc::c_char,
            &mut x as *mut libc::c_double,
            &mut y as *mut libc::c_double,
        );
        (*((*n).base.data as *mut Agnodeinfo_t)).relrank = if LR != 0 { x } else { y };
        maxrank = fmax(maxrank, (*((*n).base.data as *mut Agnodeinfo_t)).relrank);
        n = agnxtnode(g, n);
    }
    if LR != Forward {
        i = 0 as libc::c_int as size_t;
        while i < nnodes {
            n = *nlist.offset(i as isize);
            (*((*n).base.data as *mut Agnodeinfo_t))
                .relrank = maxrank - (*((*n).base.data as *mut Agnodeinfo_t)).relrank;
            i = i.wrapping_add(1);
        }
    }
    qsort(
        nlist as *mut libc::c_void,
        nnodes,
        ::std::mem::size_of::<*mut Agnode_t>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut *mut Agnode_t,
                    *mut *mut Agnode_t,
                ) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                cmpf
                    as unsafe extern "C" fn(
                        *mut *mut Agnode_t,
                        *mut *mut Agnode_t,
                    ) -> libc::c_int,
            ),
        ),
    );
    i = 0 as libc::c_int as size_t;
    while i < nnodes {
        n = *nlist.offset(i as isize);
        cnt = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if (*((*n).base.data as *mut Agnodeinfo_t)).x[j as usize] != 0.0f64 {
                cnt += 1;
            }
            j += 1;
        }
        if !(cnt > 0 as libc::c_int) {
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                sum[j as usize] = 0.0f64;
                j += 1;
            }
            cnt = 0 as libc::c_int;
            e = agfstedge(g, n);
            while !e.is_null() {
                v = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
                if v == n {
                    v = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 3 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(1 as libc::c_int as isize)
                    })
                        .node;
                }
                d = (*((*v).base.data as *mut Agnodeinfo_t)).relrank
                    - (*((*n).base.data as *mut Agnodeinfo_t)).relrank - 0.01f64;
                if d < 0 as libc::c_int as libc::c_double {
                    let mut t: libc::c_double = 0.0f64;
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        t += (*((*v).base.data as *mut Agnodeinfo_t)).x[j as usize];
                        sum[j as usize]
                            += (*((*v).base.data as *mut Agnodeinfo_t)).x[j as usize];
                        j += 1;
                    }
                    if t > 0.0f64 {
                        cnt += 1;
                    }
                }
                e = agnxtedge(g, e, n);
            }
            if cnt != 0 {
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    (*((*n).base.data as *mut Agnodeinfo_t))
                        .x[j as usize] = sum[j as usize] / cnt as libc::c_double;
                    j += 1;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < nnodes {
        let mut h: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        let mut b: libc::c_double = 0.;
        let mut t_0: libc::c_double = 0.;
        let mut buf: [libc::c_char; 64] = [0; 64];
        n = *nlist.offset(i as isize);
        t_0 = 0.0f64;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            t_0 += (*((*n).base.data as *mut Agnodeinfo_t)).x[j as usize];
            j += 1;
        }
        if t_0 > 0.0f64 {
            h = (*((*n).base.data as *mut Agnodeinfo_t)).x[0 as libc::c_int as usize];
            if AdjustSaturation != 0 {
                s = (*((*n).base.data as *mut Agnodeinfo_t)).relrank / maxrank;
                if Forward == 0 {
                    s = 1.0f64 - s;
                }
                s = MinRankSaturation + s * (MaxRankSaturation - MinRankSaturation);
            } else {
                s = 1.0f64;
            }
            s = s
                * (*((*n).base.data as *mut Agnodeinfo_t)).x[1 as libc::c_int as usize];
            b = (*((*n).base.data as *mut Agnodeinfo_t)).x[2 as libc::c_int as usize];
        } else {
            h = Defcolor[0 as libc::c_int as usize];
            s = Defcolor[1 as libc::c_int as usize];
            b = Defcolor[2 as libc::c_int as usize];
        }
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%f %f %f\0" as *const u8 as *const libc::c_char,
            h,
            s,
            b,
        );
        agset(
            n as *mut libc::c_void,
            b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
        i = i.wrapping_add(1);
    }
    free(nlist as *mut libc::c_void);
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
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
    init(argc, argv);
    newIngraph(
        &mut ig,
        Files,
        Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
    );
    loop {
        g = nextGraph(&mut ig);
        if g.is_null() {
            break;
        }
        color(g);
        agwrite(g, stdout as *mut libc::c_void);
        fflush(stdout);
        agclose(g);
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
