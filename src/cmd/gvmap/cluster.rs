#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Verbose: libc::c_uchar;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    fn initDotIO(g: *mut Agraph_t);
    fn attached_clustering(
        g: *mut Agraph_t,
        maxcluster: libc::c_int,
        clustering_scheme: libc::c_int,
    );
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const CLUSTERING_MQ: C2RustUnnamed_2 = 1;
pub const CLUSTERING_MODULARITY: C2RustUnnamed_2 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opts_t {
    pub outfp: *mut FILE,
    pub infiles: *mut *mut libc::c_char,
    pub maxcluster: libc::c_int,
    pub clustering_method: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut usestr: [libc::c_char; 255] = unsafe {
    *::std::mem::transmute::<
        &[u8; 255],
        &[libc::c_char; 255],
    >(
        b"    -C k - generate no more than k clusters (0)\n       0 : no limit\n    -c k - use clustering method k (0)\n       0 : use modularity\n       1 : use modularity quality\n    -o <outfile> - output file (stdout)\n    -v   - verbose mode\n    -?   - print usage\n\0",
    )
};
unsafe extern "C" fn usage(mut cmd: *mut libc::c_char, mut eval: libc::c_int) {
    fprintf(
        stderr,
        b"Usage: %s <options> graphfile\n\0" as *const u8 as *const libc::c_char,
        cmd,
    );
    fputs(usestr.as_ptr(), stderr);
    graphviz_exit(eval);
}
unsafe extern "C" fn openFile(
    mut name: *const libc::c_char,
    mut cmd: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"%s: could not open file %s for writing\n\0" as *const u8
                as *const libc::c_char,
            cmd,
            name,
        );
        graphviz_exit(-(1 as libc::c_int));
    }
    return fp;
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut opts: *mut opts_t,
) {
    let mut cmd: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut c: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    (*opts).maxcluster = 0 as libc::c_int;
    let ref mut fresh0 = (*opts).outfp;
    *fresh0 = stdout;
    Verbose = 0 as libc::c_int as libc::c_uchar;
    (*opts).clustering_method = CLUSTERING_MODULARITY as libc::c_int;
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b":vC:c:o:?\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            99 => {
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_int,
                ) == 0 as libc::c_int || v < 0 as libc::c_int
                {
                    usage(cmd, 1 as libc::c_int);
                } else {
                    (*opts).clustering_method = v;
                }
            }
            67 => {
                if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut v as *mut libc::c_int,
                ) == 0 as libc::c_int || v < 0 as libc::c_int
                {
                    usage(cmd, 1 as libc::c_int);
                } else {
                    (*opts).maxcluster = v;
                }
            }
            111 => {
                let ref mut fresh1 = (*opts).outfp;
                *fresh1 = openFile(optarg, cmd);
            }
            118 => {
                Verbose = 1 as libc::c_int as libc::c_uchar;
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    usage(cmd, 0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b" option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        optopt,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
            }
            _ => {}
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc != 0 {
        let ref mut fresh2 = (*opts).infiles;
        *fresh2 = argv;
    } else {
        let ref mut fresh3 = (*opts).infiles;
        *fresh3 = 0 as *mut *mut libc::c_char;
    };
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe extern "C" fn clusterGraph(
    mut g: *mut Agraph_t,
    mut maxcluster: libc::c_int,
    mut clustering_method: libc::c_int,
) {
    initDotIO(g);
    attached_clustering(g, maxcluster, clustering_method);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut prevg: *mut Agraph_t = 0 as *mut Agraph_t;
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
    let mut opts: opts_t = opts_t {
        outfp: 0 as *mut FILE,
        infiles: 0 as *mut *mut libc::c_char,
        maxcluster: 0,
        clustering_method: 0,
    };
    init(argc, argv, &mut opts);
    newIngraph(
        &mut ig,
        opts.infiles,
        Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
    );
    loop {
        g = nextGraph(&mut ig);
        if g.is_null() {
            break;
        }
        if !prevg.is_null() {
            agclose(prevg);
        }
        clusterGraph(g, opts.maxcluster, opts.clustering_method);
        agwrite(g, opts.outfp as *mut libc::c_void);
        prevg = g;
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
