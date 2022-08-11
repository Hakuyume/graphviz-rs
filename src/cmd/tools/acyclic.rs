#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agisstrict(g: *mut Agraph_t) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agedge(
        g: *mut Agraph_t,
        t: *mut Agnode_t,
        h: *mut Agnode_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agdelete(g: *mut Agraph_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agcopyattr(oldobj: *mut libc::c_void, newobj: *mut libc::c_void) -> libc::c_int;
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agsafeset(
        obj: *mut libc::c_void,
        name: *mut libc::c_char,
        value: *const libc::c_char,
        def: *const libc::c_char,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
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
pub struct Agnodeinfo_t {
    pub h: Agrec_t,
    pub mark: libc::c_int,
    pub onstack: libc::c_int,
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut inFile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut outFile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut doWrite: libc::c_int = 1 as libc::c_int;
static mut Verbose: libc::c_int = 0;
static mut cmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut num_rev: libc::c_int = 0;
unsafe extern "C" fn addRevEdge(mut g: *mut Agraph_t, mut e: *mut Agedge_t) {
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut f: *mut Agedge_t = agedge(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node,
        agnameof(e as *mut libc::c_void),
        1 as libc::c_int,
    );
    agcopyattr(e as *mut libc::c_void, f as *mut libc::c_void);
    num_rev += 1;
    sym = agattr(
        g,
        2 as libc::c_int,
        b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if !sym.is_null() {
        agsafeset(
            f as *mut libc::c_void,
            b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxget(e as *mut libc::c_void, sym),
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    sym = agattr(
        g,
        2 as libc::c_int,
        b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    if !sym.is_null() {
        agsafeset(
            f as *mut libc::c_void,
            b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            agxget(e as *mut libc::c_void, sym),
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn dfs(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut hasCycle: libc::c_int,
) -> libc::c_int {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut f: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    (*((*t).base.data as *mut Agnodeinfo_t)).mark = 1 as libc::c_int;
    (*((*t).base.data as *mut Agnodeinfo_t)).onstack = 1 as libc::c_int;
    e = agfstout(g, t);
    while !e.is_null() {
        f = agnxtout(g, e);
        if !((*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        }))
            .node
            == (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node)
        {
            h = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            })
                .node;
            if (*((*h).base.data as *mut Agnodeinfo_t)).onstack != 0 {
                if agisstrict(g) != 0 {
                    if (agedge(g, h, t, 0 as *mut libc::c_char, 0 as libc::c_int))
                        .is_null()
                    {
                        addRevEdge(g, e);
                    }
                } else {
                    let mut key: *mut libc::c_char = agnameof(e as *mut libc::c_void);
                    if key.is_null()
                        || (agedge(g, h, t, key, 0 as libc::c_int)).is_null()
                    {
                        addRevEdge(g, e);
                    }
                }
                agdelete(g, e as *mut libc::c_void);
                hasCycle = 1 as libc::c_int;
            } else if (*((*h).base.data as *mut Agnodeinfo_t)).mark == 0 as libc::c_int {
                hasCycle |= dfs(g, h, hasCycle);
            }
        }
        e = f;
    }
    (*((*t).base.data as *mut Agnodeinfo_t)).onstack = 0 as libc::c_int;
    return hasCycle;
}
static mut useString: *mut libc::c_char = b"Usage: %s [-nv?] [-o outfile] <file>\n  -o <file> - put output in <file>\n  -n        - do not output graph\n  -v        - verbose\n  -?        - print usage\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    fprintf(stderr, useString, cmd);
    graphviz_exit(v);
}
unsafe extern "C" fn openFile(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut modestr: *mut libc::c_char = 0 as *mut libc::c_char;
    fp = fopen(name, mode);
    if fp.is_null() {
        if *mode as libc::c_int == 'r' as i32 {
            modestr = b"reading\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            modestr = b"writing\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        fprintf(
            stderr,
            b"%s: could not open file %s for %s\n\0" as *const u8 as *const libc::c_char,
            cmd,
            name,
            modestr,
        );
        graphviz_exit(-(1 as libc::c_int));
    }
    return fp;
}
unsafe extern "C" fn init(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    cmd = *argv.offset(0 as libc::c_int as isize);
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b":vno:\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            111 => {
                if !outFile.is_null() {
                    fclose(outFile);
                }
                outFile = openFile(optarg, b"w\0" as *const u8 as *const libc::c_char);
            }
            110 => {
                doWrite = 0 as libc::c_int;
            }
            118 => {
                Verbose = 1 as libc::c_int;
            }
            63 => {
                if optopt == '?' as i32 {
                    usage(0 as libc::c_int);
                } else {
                    fprintf(
                        stderr,
                        b"%s: option -%c unrecognized\n\0" as *const u8
                            as *const libc::c_char,
                        cmd,
                        optopt,
                    );
                    usage(-(1 as libc::c_int));
                }
            }
            58 => {
                fprintf(
                    stderr,
                    b"%s: missing argument for option -%c\n\0" as *const u8
                        as *const libc::c_char,
                    cmd,
                    optopt,
                );
                usage(-(1 as libc::c_int));
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"acyclic.c\0" as *const u8 as *const libc::c_char,
                    167 as libc::c_int,
                );
                abort();
            }
        }
    }
    if optind < argc {
        inFile = openFile(
            *argv.offset(optind as isize),
            b"r\0" as *const u8 as *const libc::c_char,
        );
    } else {
        inFile = stdin;
    }
    if outFile.is_null() {
        outFile = stdout;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut rv: libc::c_int = 0 as libc::c_int;
    init(argc, argv);
    g = agread(inFile as *mut libc::c_void, 0 as *mut Agdisc_t);
    if !g.is_null() {
        if agisdirected(g) != 0 {
            aginit(
                g,
                1 as libc::c_int,
                b"info\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<Agnodeinfo_t>() as libc::c_ulong as libc::c_int,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            n = agfstnode(g);
            while !n.is_null() {
                if (*((*n).base.data as *mut Agnodeinfo_t)).mark == 0 as libc::c_int {
                    rv |= dfs(g, n, 0 as libc::c_int);
                }
                n = agnxtnode(g, n);
            }
            if doWrite != 0 {
                agwrite(g, outFile as *mut libc::c_void);
                fflush(outFile);
            }
            if Verbose != 0 {
                if rv != 0 {
                    fprintf(
                        stderr,
                        b"Graph \"%s\" has cycles; %d reversed edges\n\0" as *const u8
                            as *const libc::c_char,
                        agnameof(g as *mut libc::c_void),
                        num_rev,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"Graph \"%s\" is acyclic\n\0" as *const u8
                            as *const libc::c_char,
                        agnameof(g as *mut libc::c_void),
                    );
                }
            }
        } else {
            rv = -(1 as libc::c_int);
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"Graph \"%s\" is undirected\n\0" as *const u8
                        as *const libc::c_char,
                    agnameof(g as *mut libc::c_void),
                );
            }
        }
        graphviz_exit(rv);
    } else {
        graphviz_exit(-(1 as libc::c_int));
    };
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
