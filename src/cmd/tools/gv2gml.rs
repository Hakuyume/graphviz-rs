#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agnxtattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        attr: *mut Agsym_t,
    ) -> *mut Agsym_t;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn xml_escape(
        s: *const libc::c_char,
        flags: xml_flags_t,
        cb: Option::<
            unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
        >,
        state: *mut libc::c_void,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct xml_flags_t {
    #[bitfield(name = "raw", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "dash", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "nbsp", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "utf8", ty = "libc::c_uint", bits = "3..=3")]
    pub raw_dash_nbsp_utf8: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
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
pub struct node_attrs {
    pub flags: libc::c_uint,
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub w: libc::c_double,
    pub h: libc::c_double,
    pub type_0: *mut libc::c_char,
    pub image: *mut libc::c_char,
    pub fill: *mut libc::c_char,
    pub outline: *mut libc::c_char,
    pub width: *mut libc::c_char,
    pub outlineStyle: *mut libc::c_char,
    pub fontColor: *mut libc::c_char,
    pub fontSize: *mut libc::c_char,
    pub fontName: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge_attrs {
    pub flags: libc::c_uint,
    pub width: *mut libc::c_char,
    pub fill: *mut libc::c_char,
    pub arrow: *mut libc::c_char,
    pub arrowhead: *mut libc::c_char,
    pub arrowtail: *mut libc::c_char,
    pub pos: *mut libc::c_char,
    pub fontColor: *mut libc::c_char,
    pub fontSize: *mut libc::c_char,
    pub fontName: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Local_Agnodeinfo_t {
    pub h: Agrec_t,
    pub id: uint64_t,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut outFile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut CmdName: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut id: uint64_t = 0;
unsafe extern "C" fn indent(mut ix: libc::c_int) {
    loop {
        let fresh0 = ix;
        ix = ix - 1;
        if !(fresh0 != 0) {
            break;
        }
        fprintf(outFile, b"  \0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn isNumber(mut s: *mut libc::c_char) -> libc::c_int {
    let mut ep: *mut libc::c_char = s;
    strtod(s, &mut ep);
    if s != ep {
        while *ep as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*ep as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            ep = ep.offset(1);
        }
        if *ep != 0 { return 0 as libc::c_int } else { return 1 as libc::c_int }
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn parseStyle(mut s: *mut libc::c_char) -> libc::c_int {
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: *mut libc::c_char = b" \t,\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    s = strdup(s);
    ip = strtok(s, sep);
    while !ip.is_null() {
        if strcmp(ip, b"invis\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(ip, b"filled\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 4 as libc::c_int;
        } else if strcmp(ip, b"dashed\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 6 as libc::c_int;
        } else if strcmp(ip, b"dotted\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 7 as libc::c_int;
        } else if strcmp(ip, b"solid\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 5 as libc::c_int;
        } else if strcmp(ip, b"bold\0" as *const u8 as *const libc::c_char) == 0 {
            flags |= (1 as libc::c_int) << 8 as libc::c_int;
        }
        ip = strtok(0 as *mut libc::c_char, sep);
    }
    free(s as *mut libc::c_void);
    return flags;
}
unsafe extern "C" fn emitInt(
    mut name: *mut libc::c_char,
    mut value: libc::c_int,
    mut ix: libc::c_int,
) {
    indent(ix);
    fprintf(outFile, b"%s %d\n\0" as *const u8 as *const libc::c_char, name, value);
}
unsafe extern "C" fn emitReal(
    mut name: *mut libc::c_char,
    mut value: libc::c_double,
    mut ix: libc::c_int,
) {
    indent(ix);
    fprintf(outFile, b"%s %g\n\0" as *const u8 as *const libc::c_char, name, value);
}
unsafe extern "C" fn emitPoint(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut ix: libc::c_int,
) {
    indent(ix);
    fprintf(
        outFile,
        b"point [ x %g y %g ]\n\0" as *const u8 as *const libc::c_char,
        x,
        y,
    );
}
unsafe extern "C" fn skipWS(mut s: *mut libc::c_char) -> *mut libc::c_char {
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
    }
    return s;
}
unsafe extern "C" fn readPoint(
    mut s: *mut libc::c_char,
    mut xp: *mut libc::c_double,
    mut yp: *mut libc::c_double,
) -> *mut libc::c_char {
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    s = skipWS(s);
    *xp = strtod(s, &mut endp);
    if s == endp {
        return 0 as *mut libc::c_char;
    }
    endp = endp.offset(1);
    s = endp;
    *yp = strtod(s, &mut endp);
    if s == endp { return 0 as *mut libc::c_char } else { return endp };
}
unsafe extern "C" fn arrowEnd(
    mut s0: *mut libc::c_char,
    mut pfx: *mut libc::c_char,
    mut fp: *mut libc::c_int,
    mut xp: *mut libc::c_double,
    mut yp: *mut libc::c_double,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = skipWS(s0);
    if strncmp(s, pfx, 2 as libc::c_int as libc::c_ulong) != 0 {
        return s;
    }
    s = s.offset(2 as libc::c_int as isize);
    s = readPoint(s, xp, yp);
    if s.is_null() {
        fprintf(
            stderr,
            b"Illegal spline end: %s\n\0" as *const u8 as *const libc::c_char,
            s0,
        );
        graphviz_exit(1 as libc::c_int);
    }
    *fp = 1 as libc::c_int;
    return s;
}
unsafe extern "C" fn emitSpline(mut s: *mut libc::c_char, mut ix: libc::c_int) {
    let mut sx: libc::c_double = 0.;
    let mut sy: libc::c_double = 0.;
    let mut ex: libc::c_double = 0.;
    let mut ey: libc::c_double = 0.;
    let mut sarrow: libc::c_int = 0 as libc::c_int;
    let mut earrow: libc::c_int = 0 as libc::c_int;
    s = arrowEnd(
        s,
        b"e,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut earrow,
        &mut ex,
        &mut ey,
    );
    s = arrowEnd(
        s,
        b"s,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut sarrow,
        &mut sx,
        &mut sy,
    );
    indent(ix);
    fprintf(outFile, b"Line [\n\0" as *const u8 as *const libc::c_char);
    if sarrow != 0 {
        emitPoint(sx, sy, ix + 1 as libc::c_int);
    }
    loop {
        s = readPoint(s, &mut sx, &mut sy);
        if s.is_null() {
            break;
        }
        emitPoint(sx, sy, ix + 1 as libc::c_int);
    }
    if earrow != 0 {
        emitPoint(ex, ey, ix + 1 as libc::c_int);
    }
    indent(ix);
    fprintf(outFile, b"]\n\0" as *const u8 as *const libc::c_char);
}
#[inline]
unsafe extern "C" fn put(
    mut stream: *mut libc::c_void,
    mut s: *const libc::c_char,
) -> libc::c_int {
    return fputs(s, stream as *mut FILE);
}
#[inline]
unsafe extern "C" fn xml_puts(
    mut stream: *mut FILE,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let flags: xml_flags_t = {
        let mut init = xml_flags_t {
            raw_dash_nbsp_utf8: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_raw(0);
        init.set_dash(1 as libc::c_int as libc::c_uint);
        init.set_nbsp(1 as libc::c_int as libc::c_uint);
        init.set_utf8(0);
        init
    };
    return xml_escape(
        s,
        flags,
        Some(
            put
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        stream as *mut libc::c_void,
    );
}
unsafe extern "C" fn emitAttr(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ix: libc::c_int,
) {
    indent(ix);
    if isNumber(value) != 0 {
        fprintf(outFile, b"%s %s\n\0" as *const u8 as *const libc::c_char, name, value);
    } else {
        fprintf(outFile, b"%s \"\0" as *const u8 as *const libc::c_char, name);
        xml_puts(outFile, value);
        fputs(b"\"\n\0" as *const u8 as *const libc::c_char, outFile);
    };
}
unsafe extern "C" fn emitNodeAttrs(
    mut G: *mut Agraph_t,
    mut np: *mut Agnode_t,
    mut ix: libc::c_int,
) {
    let mut s: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut attrs: node_attrs = node_attrs {
        flags: 0,
        x: 0.,
        y: 0.,
        w: 0.,
        h: 0.,
        type_0: 0 as *mut libc::c_char,
        image: 0 as *mut libc::c_char,
        fill: 0 as *mut libc::c_char,
        outline: 0 as *mut libc::c_char,
        width: 0 as *mut libc::c_char,
        outlineStyle: 0 as *mut libc::c_char,
        fontColor: 0 as *mut libc::c_char,
        fontSize: 0 as *mut libc::c_char,
        fontName: 0 as *mut libc::c_char,
    };
    let mut doGraphics: libc::c_int = 0 as libc::c_int;
    let mut doLabelGraphics: libc::c_int = 0 as libc::c_int;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut style: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    memset(
        &mut attrs as *mut node_attrs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<node_attrs>() as libc::c_ulong,
    );
    s = agnxtattr(G, 1 as libc::c_int, 0 as *mut Agsym_t);
    while !s.is_null() {
        if strcmp((*s).name, b"style\0" as *const u8 as *const libc::c_char) == 0 {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                style = parseStyle(v);
                if style & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                    attrs.flags
                        |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
                }
                if style & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                    attrs.flags
                        |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
                }
                if style & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                    attrs
                        .outlineStyle = b"line\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                if style & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                    attrs
                        .outlineStyle = b"dashed\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                if style & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                    attrs
                        .outlineStyle = b"dotted\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"label\0" as *const u8 as *const libc::c_char) == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if strcmp(b"\\N\0" as *const u8 as *const libc::c_char, v) == 0 {
                label = agnameof(np as *mut libc::c_void);
                emitAttr((*s).name, label, ix);
                doLabelGraphics = 1 as libc::c_int;
            } else if *v != 0 {
                label = v;
                emitAttr((*s).name, label, ix);
                doLabelGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"penwidth\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.width = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"width\0" as *const u8 as *const libc::c_char) == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.w = 72.0f64 * atof(v);
                attrs.flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"height\0" as *const u8 as *const libc::c_char) == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.h = 72.0f64 * atof(v);
                attrs.flags |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"pos\0" as *const u8 as *const libc::c_char) == 0 {
            v = agxget(np as *mut libc::c_void, s);
            if sscanf(
                v,
                b"%lf,%lf\0" as *const u8 as *const libc::c_char,
                &mut x as *mut libc::c_double,
                &mut y as *mut libc::c_double,
            ) == 2 as libc::c_int
            {
                doGraphics = 1 as libc::c_int;
                attrs.x = x;
                attrs.y = y;
                attrs.flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            }
        } else if strcmp((*s).name, b"shape\0" as *const u8 as *const libc::c_char) == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.type_0 = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"color\0" as *const u8 as *const libc::c_char) == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fill = v;
                attrs.outline = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"fillcolor\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fill = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"pencolor\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.outline = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"fontname\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fontName = v;
                doLabelGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"fontsize\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fontSize = v;
                doLabelGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"fontcolor\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(np as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fontColor = v;
                doLabelGraphics = 1 as libc::c_int;
            }
        } else {
            v = agxget(np as *mut libc::c_void, s);
            emitAttr((*s).name, v, ix);
        }
        s = agnxtattr(G, 1 as libc::c_int, s);
    }
    if doGraphics != 0 {
        fprintf(outFile, b"    graphics [\n\0" as *const u8 as *const libc::c_char);
        if attrs.flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
            emitReal(
                b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.x,
                ix + 1 as libc::c_int,
            );
            emitReal(
                b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.y,
                ix + 1 as libc::c_int,
            );
        }
        if attrs.flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
            emitReal(
                b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.w,
                ix + 1 as libc::c_int,
            );
        }
        if attrs.flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            emitReal(
                b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.h,
                ix + 1 as libc::c_int,
            );
        }
        if attrs.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            emitInt(
                b"visible\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                ix + 1 as libc::c_int,
            );
        }
        if attrs.flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0 {
            emitInt(
                b"hasFill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                1 as libc::c_int,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.type_0).is_null() {
            emitAttr(
                b"type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.type_0,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.image).is_null() {
            emitAttr(
                b"image\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.image,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fill).is_null() {
            emitAttr(
                b"fill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fill,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.outline).is_null() {
            emitAttr(
                b"outline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.outline,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.width).is_null() {
            emitAttr(
                b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.width,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.outlineStyle).is_null() {
            emitAttr(
                b"outlineStyle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                attrs.outlineStyle,
                ix + 1 as libc::c_int,
            );
        }
        fprintf(outFile, b"    ]\n\0" as *const u8 as *const libc::c_char);
    }
    if doLabelGraphics != 0 {
        fprintf(outFile, b"    LabelGraphics [\n\0" as *const u8 as *const libc::c_char);
        if !label.is_null() {
            emitAttr(
                b"text\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                label,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fontColor).is_null() {
            emitAttr(
                b"fontColor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fontColor,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fontSize).is_null() {
            emitAttr(
                b"fontSize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fontSize,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fontName).is_null() {
            emitAttr(
                b"fontName\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fontName,
                ix + 1 as libc::c_int,
            );
        }
        fprintf(outFile, b"    ]\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn emitNode(mut G: *mut Agraph_t, mut n: *mut Agnode_t) {
    agbindrec(
        n as *mut libc::c_void,
        b"nodeinfo\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<Local_Agnodeinfo_t>() as libc::c_ulong as libc::c_uint,
        1 as libc::c_int,
    );
    fprintf(
        outFile,
        b"  node [\n    id %lu\n    name \"%s\"\n\0" as *const u8 as *const libc::c_char,
        id,
        agnameof(n as *mut libc::c_void),
    );
    let fresh1 = id;
    id = id.wrapping_add(1);
    (*((*n).base.data as *mut Local_Agnodeinfo_t)).id = fresh1;
    emitNodeAttrs(G, n, 2 as libc::c_int);
    fprintf(outFile, b"  ]\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn emitEdgeAttrs(
    mut G: *mut Agraph_t,
    mut ep: *mut Agedge_t,
    mut ix: libc::c_int,
) {
    let mut s: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut attrs: edge_attrs = edge_attrs {
        flags: 0,
        width: 0 as *mut libc::c_char,
        fill: 0 as *mut libc::c_char,
        arrow: 0 as *mut libc::c_char,
        arrowhead: 0 as *mut libc::c_char,
        arrowtail: 0 as *mut libc::c_char,
        pos: 0 as *mut libc::c_char,
        fontColor: 0 as *mut libc::c_char,
        fontSize: 0 as *mut libc::c_char,
        fontName: 0 as *mut libc::c_char,
    };
    let mut doGraphics: libc::c_int = 0 as libc::c_int;
    let mut doLabelGraphics: libc::c_int = 0 as libc::c_int;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut style: libc::c_int = 0;
    memset(
        &mut attrs as *mut edge_attrs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<edge_attrs>() as libc::c_ulong,
    );
    s = agnxtattr(G, 2 as libc::c_int, 0 as *mut Agsym_t);
    while !s.is_null() {
        if strcmp((*s).name, b"style\0" as *const u8 as *const libc::c_char) == 0 {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                style = parseStyle(v);
                if style & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                    attrs.flags
                        |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
                }
                if style & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                    attrs.flags
                        |= ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
                }
                if style & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                    attrs.flags
                        |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
                }
                if style & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                    attrs.flags
                        |= ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
                }
                if style & (1 as libc::c_int) << 8 as libc::c_int != 0 {
                    attrs
                        .width = b"2\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"label\0" as *const u8 as *const libc::c_char) == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                label = v;
                emitAttr((*s).name, label, ix);
                doLabelGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"penwidth\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.width = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"pos\0" as *const u8 as *const libc::c_char) == 0 {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                doGraphics = 1 as libc::c_int;
                attrs.pos = v;
            }
        } else if strcmp((*s).name, b"dir\0" as *const u8 as *const libc::c_char) == 0 {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                doGraphics = 1 as libc::c_int;
                attrs.arrow = v;
            }
        } else if strcmp((*s).name, b"color\0" as *const u8 as *const libc::c_char) == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fill = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"pencolor\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fill = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"arrowhead\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.arrowhead = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"arrowtail\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.arrowtail = v;
                doGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"fontname\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fontName = v;
                doLabelGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"fontsize\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fontSize = v;
                doLabelGraphics = 1 as libc::c_int;
            }
        } else if strcmp((*s).name, b"fontcolor\0" as *const u8 as *const libc::c_char)
                == 0
            {
            v = agxget(ep as *mut libc::c_void, s);
            if *v != 0 {
                attrs.fontColor = v;
                doLabelGraphics = 1 as libc::c_int;
            }
        } else {
            v = agxget(ep as *mut libc::c_void, s);
            emitAttr((*s).name, v, ix);
        }
        s = agnxtattr(G, 2 as libc::c_int, s);
    }
    if doGraphics != 0 {
        fprintf(outFile, b"    graphics [\n\0" as *const u8 as *const libc::c_char);
        if !(attrs.pos).is_null() {
            emitSpline(attrs.pos, ix + 1 as libc::c_int);
        }
        if attrs.flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            emitInt(
                b"visible\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fill).is_null() {
            emitAttr(
                b"fill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fill,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.width).is_null() {
            emitAttr(
                b"width\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.width,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.arrowhead).is_null() {
            emitAttr(
                b"targetArrow\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                attrs.arrowhead,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.arrowtail).is_null() {
            emitAttr(
                b"sourceArrow\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                attrs.arrowtail,
                ix + 1 as libc::c_int,
            );
        }
        if attrs.flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint != 0 {
            emitAttr(
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"dashed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ix + 1 as libc::c_int,
            );
        } else if attrs.flags & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
                != 0
            {
            emitAttr(
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"dotted\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ix + 1 as libc::c_int,
            );
        } else if attrs.flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
                != 0
            {
            emitAttr(
                b"style\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.arrow).is_null() {
            if strcmp(attrs.arrow, b"forward\0" as *const u8 as *const libc::c_char) == 0
            {
                emitAttr(
                    b"arrow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"first\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ix + 1 as libc::c_int,
                );
            } else if strcmp(attrs.arrow, b"back\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                emitAttr(
                    b"arrow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"last\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ix + 1 as libc::c_int,
                );
            } else if strcmp(attrs.arrow, b"both\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                emitAttr(
                    b"arrow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"both\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ix + 1 as libc::c_int,
                );
            } else if strcmp(attrs.arrow, b"none\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                emitAttr(
                    b"arrow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ix + 1 as libc::c_int,
                );
            }
        }
        fprintf(outFile, b"    ]\n\0" as *const u8 as *const libc::c_char);
    }
    if doLabelGraphics != 0 {
        fprintf(outFile, b"    LabelGraphics [\n\0" as *const u8 as *const libc::c_char);
        if !label.is_null() {
            emitAttr(
                b"text\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                label,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fontColor).is_null() {
            emitAttr(
                b"fontColor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fontColor,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fontSize).is_null() {
            emitAttr(
                b"fontSize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fontSize,
                ix + 1 as libc::c_int,
            );
        }
        if !(attrs.fontName).is_null() {
            emitAttr(
                b"fontName\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attrs.fontName,
                ix + 1 as libc::c_int,
            );
        }
        fprintf(outFile, b"    ]\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn emitEdge(mut G: *mut Agraph_t, mut e: *mut Agedge_t) {
    fprintf(
        outFile,
        b"  edge [\n    id %lu\n\0" as *const u8 as *const libc::c_char,
        ((*(e as *mut Agobj_t)).tag).seq() as uint64_t,
    );
    fprintf(
        outFile,
        b"    source %lu\n\0" as *const u8 as *const libc::c_char,
        (*((*(*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node)
            .base
            .data as *mut Local_Agnodeinfo_t))
            .id,
    );
    fprintf(
        outFile,
        b"    target %lu\n\0" as *const u8 as *const libc::c_char,
        (*((*(*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 2 as libc::c_int
        {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node)
            .base
            .data as *mut Local_Agnodeinfo_t))
            .id,
    );
    emitEdgeAttrs(G, e, 2 as libc::c_int);
    fprintf(outFile, b"  ]\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn emitGraphAttrs(mut G: *mut Agraph_t) {
    let mut s: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    s = agnxtattr(G, 0 as libc::c_int, 0 as *mut Agsym_t);
    while !s.is_null() {
        v = agxget(G as *mut libc::c_void, s);
        if *v != 0 {
            emitAttr((*s).name, v, 1 as libc::c_int);
        }
        s = agnxtattr(G, 0 as libc::c_int, s);
    }
}
unsafe extern "C" fn gv_to_gml(mut G: *mut Agraph_t) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    fprintf(outFile, b"graph [\n  version 2\n\0" as *const u8 as *const libc::c_char);
    if agisdirected(G) != 0 {
        fprintf(outFile, b"  directed 1\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(outFile, b"  directed 0\n\0" as *const u8 as *const libc::c_char);
    }
    emitGraphAttrs(G);
    n = agfstnode(G);
    while !n.is_null() {
        emitNode(G, n);
        n = agnxtnode(G, n);
    }
    n = agfstnode(G);
    while !n.is_null() {
        e = agfstout(G, n);
        while !e.is_null() {
            emitEdge(G, e);
            e = agnxtout(G, e);
        }
        n = agnxtnode(G, n);
    }
    fprintf(outFile, b"]\n\0" as *const u8 as *const libc::c_char);
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
static mut useString: *mut libc::c_char = b"Usage: %s [-?] <files>\n  -o<file>  : output to <file> (stdout)\n  -? - print usage\nIf no files are specified, stdin is used\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    printf(useString, CmdName);
    graphviz_exit(v);
}
unsafe extern "C" fn cmdName(mut cmd: *mut libc::c_char) -> *mut libc::c_char {
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    sp = strrchr(cmd, '/' as i32);
    if !sp.is_null() {
        sp = sp.offset(1);
    } else {
        sp = cmd;
    }
    return sp;
}
unsafe extern "C" fn initargs(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    CmdName = cmdName(*argv.offset(0 as libc::c_int as isize));
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(argc, argv, b":o:\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            111 => {
                if !outFile.is_null() {
                    fclose(outFile);
                }
                outFile = openFile(optarg);
            }
            58 => {
                fprintf(
                    stderr,
                    b"%s: option -%c missing parameter\n\0" as *const u8
                        as *const libc::c_char,
                    CmdName,
                    optopt,
                );
                usage(1 as libc::c_int);
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
                    usage(1 as libc::c_int);
                }
            }
            _ => {}
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc != 0 {
        Files = argv;
    }
    if outFile.is_null() {
        outFile = stdout;
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
    let mut rv: libc::c_int = 0;
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
    rv = 0 as libc::c_int;
    initargs(argc, argv);
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
            id = 0 as libc::c_int as uint64_t;
            agclose(prev);
        }
        prev = G;
        gv_to_gml(G);
        fflush(outFile);
    }
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
