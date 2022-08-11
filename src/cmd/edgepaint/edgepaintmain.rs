#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    fn fileName(_: *mut ingraph_state) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut Files: *mut *mut libc::c_char;
    static mut CmdName: *mut libc::c_char;
    static mut Verbose: libc::c_uchar;
    fn initDotIO(g: *mut Agraph_t);
    fn edge_distinct_coloring(
        color_scheme: *mut libc::c_char,
        lightness: *mut libc::c_char,
        g: *mut Agraph_t,
        angle: libc::c_double,
        accuracy: libc::c_double,
        check_edges_with_same_endpoint: libc::c_int,
        seed: libc::c_int,
    ) -> *mut Agraph_t;
    fn knownColorScheme(_: *const libc::c_char) -> libc::c_int;
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
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const OPT_SHARE_ENDPOINT: C2RustUnnamed_3 = 133;
pub const OPT_RANDOM_SEED: C2RustUnnamed_3 = 131;
pub const OPT_LIGHTNESS: C2RustUnnamed_3 = 132;
pub const OPT_COLOR_SCHEME: C2RustUnnamed_3 = 130;
pub const OPT_ANGLE: C2RustUnnamed_3 = 129;
pub const OPT_ACCURACY: C2RustUnnamed_3 = 128;
pub type C2RustUnnamed_3 = libc::c_uint;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
static mut fname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut outfile: *mut FILE = 0 as *const FILE as *mut FILE;
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
        graphviz_exit(1 as libc::c_int);
    }
    return fp;
}
unsafe extern "C" fn usage(mut cmd: *mut libc::c_char, mut eval: libc::c_int) {
    fprintf(
        stderr,
        b"Usage: %s <options> gv file with 2D coordinates.\n\0" as *const u8
            as *const libc::c_char,
        cmd,
    );
    fprintf(
        stderr,
        b"Find a color assignment of the edges, such that edges that cross at small angle have as different as possible.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"Options are: \n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b" --accuracy=e      : accuracy with which to find the maximally different coloring for each node with regard to its neighbors. Default 0.01.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b" --angle=a         : if edge crossing is less than that angle a, then make the edge colors different. Default 15.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b" --random_seed=s   : random seed to use. s must be an integer. If s is negative, we do -s iterations with different seeds and pick the best.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b" --color_scheme=c  : palette used. The string c should be \"rgb\", \"gray\", \"lab\" (default); or\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       a comma-separated list of RGB colors in hex (e.g., \"#ff0000,#aabbed,#eeffaa\"); or\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       a string specifying a Brewer color scheme (e.g., \"accent7\"; see https://graphviz.org/doc/info/colors.html#brewer).\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b" --lightness=l1,l2 : only applied for LAB color scheme: l1 must be integer >=0, l2 integer <=100, and l1 <=l2. By default we use 0,70\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b" --share_endpoint  :  if this option is specified, edges that shares an end point are not considered in conflict if they are close to\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       parallel but is on the opposite ends of the shared point (around 180 degree).\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b" -v               : verbose\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b" -o fname         :  write output to file fname (stdout)\n\0" as *const u8
            as *const libc::c_char,
    );
    graphviz_exit(eval);
}
unsafe extern "C" fn checkG(mut g: *mut Agraph_t) -> libc::c_int {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut prevh: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfstnode(g);
    while !n.is_null() {
        e = agfstout(g, n);
        while !e.is_null() {
            h = (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 2 as libc::c_int
            {
                e
            } else {
                e.offset(-(1 as libc::c_int as isize))
            }))
                .node;
            if h == n {
                return 1 as libc::c_int;
            }
            if h == prevh {
                return 1 as libc::c_int;
            }
            prevh = h;
            e = agnxtout(g, e);
        }
        prevh = 0 as *mut Agnode_t;
        n = agnxtnode(g, n);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn strprefix(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> bool {
    return strncmp(s1, s2, strlen(s2)) == 0 as libc::c_int;
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut angle: *mut libc::c_double,
    mut accuracy: *mut libc::c_double,
    mut check_edges_with_same_endpoint: *mut libc::c_int,
    mut seed: *mut libc::c_int,
    mut color_scheme: *mut *mut libc::c_char,
    mut lightness: *mut *mut libc::c_char,
) {
    let mut cmd: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    outfile = 0 as *mut FILE;
    Verbose = 0 as libc::c_int as libc::c_uchar;
    *accuracy = 0.01f64;
    *angle = 15 as libc::c_int as libc::c_double;
    *check_edges_with_same_endpoint = 0 as libc::c_int;
    *seed = 123 as libc::c_int;
    *color_scheme = b"lab\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *lightness = 0 as *mut libc::c_char;
    loop {
        static mut opts: [option; 7] = [
            {
                let mut init = option {
                    name: b"accuracy\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_ACCURACY as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"angle\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_ANGLE as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"color_scheme\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_COLOR_SCHEME as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"random_seed\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_RANDOM_SEED as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"lightness\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_LIGHTNESS as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: b"share_endpoint\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: OPT_SHARE_ENDPOINT as libc::c_int,
                };
                init
            },
            {
                let mut init = option {
                    name: 0 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
        ];
        let mut option_index: libc::c_int = 0 as libc::c_int;
        let mut c: libc::c_int = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"a:c:r:l:o:s:v?\0" as *const u8 as *const libc::c_char,
            opts.as_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        let mut arg: *const libc::c_char = optarg;
        if c == 'a' as i32
            && strprefix(arg, b"ccuracy=\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0
        {
            c = OPT_ACCURACY as libc::c_int;
            arg = arg
                .offset(
                    strlen(b"ccuracy=\0" as *const u8 as *const libc::c_char) as isize,
                );
        } else if c == 'a' as i32
                && strprefix(arg, b"ngle=\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
            {
            c = OPT_ANGLE as libc::c_int;
            arg = arg
                .offset(strlen(b"ngle=\0" as *const u8 as *const libc::c_char) as isize);
        } else if c == 'c' as i32
                && strprefix(arg, b"olor_scheme=\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
            {
            c = OPT_COLOR_SCHEME as libc::c_int;
            arg = arg
                .offset(
                    strlen(b"olor_scheme=\0" as *const u8 as *const libc::c_char)
                        as isize,
                );
        } else if c == 'r' as i32
                && strprefix(arg, b"andom_seed=\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
            {
            c = OPT_RANDOM_SEED as libc::c_int;
            arg = arg
                .offset(
                    strlen(b"andom_seed=\0" as *const u8 as *const libc::c_char) as isize,
                );
        } else if c == 'l' as i32
                && strprefix(arg, b"ightness=\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
            {
            c = OPT_LIGHTNESS as libc::c_int;
            arg = arg
                .offset(
                    strlen(b"ightness=\0" as *const u8 as *const libc::c_char) as isize,
                );
        } else if c == 's' as i32
                && strprefix(arg, b"hare_endpoint\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
            {
            c = OPT_SHARE_ENDPOINT as libc::c_int;
        }
        match c {
            97 | 99 | 114 | 108 => {
                fprintf(
                    stderr,
                    b"option -%c unrecognized.\n\0" as *const u8 as *const libc::c_char,
                    c,
                );
                usage(cmd, 1 as libc::c_int);
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"edgepaintmain.c\0" as *const u8 as *const libc::c_char,
                    184 as libc::c_int,
                );
                abort();
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
                    usage(cmd, 0 as libc::c_int);
                }
                fprintf(
                    stderr,
                    b"option -%c unrecognized.\n\0" as *const u8 as *const libc::c_char,
                    optopt,
                );
                usage(cmd, 1 as libc::c_int);
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"edgepaintmain.c\0" as *const u8 as *const libc::c_char,
                    192 as libc::c_int,
                );
                abort();
            }
            111 => {
                if !outfile.is_null() {
                    fclose(outfile);
                }
                outfile = openFile(arg, CmdName);
            }
            118 => {
                Verbose = (0 as libc::c_int == 0) as libc::c_int as libc::c_uchar;
            }
            128 => {
                if sscanf(arg, b"%lf\0" as *const u8 as *const libc::c_char, accuracy)
                    != 1 as libc::c_int
                    || *accuracy <= 0 as libc::c_int as libc::c_double
                {
                    fprintf(
                        stderr,
                        b"--accuracy option must be a positive real number.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
            }
            129 => {
                if sscanf(arg, b"%lf\0" as *const u8 as *const libc::c_char, angle)
                    != 1 as libc::c_int || *angle <= 0 as libc::c_int as libc::c_double
                    || *angle >= 90 as libc::c_int as libc::c_double
                {
                    fprintf(
                        stderr,
                        b"--angle option must be a positive real number between 0 and 90.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
            }
            130 => {
                if knownColorScheme(arg) == 0 {
                    fprintf(
                        stderr,
                        b"--color_scheme option must be a known color scheme.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
            }
            132 => {
                let mut l1: libc::c_int = 0 as libc::c_int;
                let mut l2: libc::c_int = 70 as libc::c_int;
                if sscanf(
                    arg,
                    b"%d,%d\0" as *const u8 as *const libc::c_char,
                    &mut l1 as *mut libc::c_int,
                    &mut l2 as *mut libc::c_int,
                ) != 2 as libc::c_int || l1 < 0 as libc::c_int || l2 > 100 as libc::c_int
                    || l1 > l2
                {
                    fprintf(
                        stderr,
                        b"invalid --lightness=%s option.\n\0" as *const u8
                            as *const libc::c_char,
                        arg,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
                free(*lightness as *mut libc::c_void);
                *lightness = gv_strdup(arg);
            }
            131 => {
                if sscanf(arg, b"%d\0" as *const u8 as *const libc::c_char, seed)
                    != 1 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"--random_seed option must be an integer.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    usage(cmd, 1 as libc::c_int);
                }
            }
            133 => {
                *check_edges_with_same_endpoint = 1 as libc::c_int;
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"edgepaintmain.c\0" as *const u8 as *const libc::c_char,
                    253 as libc::c_int,
                );
                abort();
            }
        }
    }
    if argc > optind {
        Files = argv.offset(optind as isize);
    }
    if outfile.is_null() {
        outfile = stdout;
    }
}
unsafe extern "C" fn clarify(
    mut g: *mut Agraph_t,
    mut angle: libc::c_double,
    mut accuracy: libc::c_double,
    mut check_edges_with_same_endpoint: libc::c_int,
    mut seed: libc::c_int,
    mut color_scheme: *mut libc::c_char,
    mut lightness: *mut libc::c_char,
) -> libc::c_int {
    if checkG(g) != 0 {
        agerr(
            AGERR,
            b"Graph %s contains loops or multiedges\n\0" as *const u8
                as *const libc::c_char,
            agnameof(g as *mut libc::c_void),
        );
        return 1 as libc::c_int;
    }
    initDotIO(g);
    g = edge_distinct_coloring(
        color_scheme,
        lightness,
        g,
        angle,
        accuracy,
        check_edges_with_same_endpoint,
        seed,
    );
    if g.is_null() {
        return 1 as libc::c_int;
    }
    agwrite(g, stdout as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut accuracy: libc::c_double = 0.;
    let mut angle: libc::c_double = 0.;
    let mut check_edges_with_same_endpoint: libc::c_int = 0;
    let mut seed: libc::c_int = 0;
    let mut color_scheme: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lightness: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut prev: *mut Agraph_t = 0 as *mut Agraph_t;
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
    let mut rv: libc::c_int = 0 as libc::c_int;
    init(
        argc,
        argv,
        &mut angle,
        &mut accuracy,
        &mut check_edges_with_same_endpoint,
        &mut seed,
        &mut color_scheme,
        &mut lightness,
    );
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
        if !prev.is_null() {
            agclose(prev);
        }
        prev = g;
        fname = fileName(&mut ig);
        if Verbose != 0 {
            fprintf(
                stderr,
                b"Process graph %s in file %s\n\0" as *const u8 as *const libc::c_char,
                agnameof(g as *mut libc::c_void),
                fname,
            );
        }
        if clarify(
            g,
            angle,
            accuracy,
            check_edges_with_same_endpoint,
            seed,
            color_scheme,
            lightness,
        ) != 0 as libc::c_int
        {
            rv = 1 as libc::c_int;
        }
    }
    free(lightness as *mut libc::c_void);
    graphviz_exit(rv);
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
