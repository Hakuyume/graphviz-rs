#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn agxset(
        obj: *mut libc::c_void,
        sym: *mut Agsym_t,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agclean(g: *mut Agraph_t, kind: libc::c_int, rec_name: *mut libc::c_char);
    fn aginit(
        g: *mut Agraph_t,
        kind: libc::c_int,
        rec_name: *const libc::c_char,
        rec_size: libc::c_int,
        move_to_front: libc::c_int,
    );
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agread(chan: *mut libc::c_void, disc: *mut Agdisc_t) -> *mut Agraph_t;
    fn agwrite(g: *mut Agraph_t, chan: *mut libc::c_void) -> libc::c_int;
    fn agnode(
        g: *mut Agraph_t,
        name: *mut libc::c_char,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn agattrsym(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut Agsym_t;
    fn dtopen(_: *mut Dtdisc_t, _: *mut Dtmethod_t) -> *mut Dt_t;
    static mut Dtoset: *mut Dtmethod_t;
    fn newIngraph(
        _: *mut ingraph_state,
        _: *mut *mut libc::c_char,
        _: opengfn,
    ) -> *mut ingraph_state;
    fn nextGraph(_: *mut ingraph_state) -> *mut Agraph_t;
    fn fileName(_: *mut ingraph_state) -> *mut libc::c_char;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
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
pub struct nodedata_t {
    pub hdr: Agrec_t,
    pub dist: libc::c_double,
    pub prev: *mut Agnode_t,
    pub done: libc::c_int,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut CmdName: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Files: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut Nodes: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut setall: bool = false;
static mut doPath: bool = false;
static mut doDirected: bool = false;
static mut len_sym: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
unsafe extern "C" fn getlength(mut e: *mut Agedge_t) -> libc::c_double {
    let mut len: libc::c_double = 0.;
    let mut lens: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !len_sym.is_null()
        && {
            lens = agxget(e as *mut libc::c_void, len_sym);
            *lens as libc::c_int != 0
        }
    {
        len = strtod(lens, &mut p);
        if len < 0 as libc::c_int as libc::c_double || p == lens {
            len = 1 as libc::c_int as libc::c_double;
        }
    } else {
        len = 1 as libc::c_int as libc::c_double;
    }
    return len;
}
unsafe extern "C" fn cmpf(
    mut d: *mut Dt_t,
    mut key1: *mut libc::c_void,
    mut key2: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut t: libc::c_double = 0.;
    t = (*((*(key1 as *mut Agnode_t)).base.data as *mut nodedata_t)).dist
        - (*((*(key2 as *mut Agnode_t)).base.data as *mut nodedata_t)).dist;
    if t < 0 as libc::c_int as libc::c_double {
        return -(1 as libc::c_int);
    }
    if t > 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if key1 < key2 {
        return -(1 as libc::c_int);
    }
    if key1 > key2 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut MyDisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0 as libc::c_int,
            size: 0 as libc::c_int,
            link: -(1 as libc::c_int),
            makef: None,
            freef: None,
            comparf: Some(
                cmpf
                    as unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
            ),
            hashf: None,
            memoryf: None,
            eventf: None,
        };
        init
    }
};
unsafe extern "C" fn extract_min(mut Q: *mut Dict_t) -> *mut Agnode_t {
    let mut rv: *mut Agnode_t = 0 as *mut Agnode_t;
    rv = (Some(((*(Q as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(Q, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agnode_t;
    (Some(((*(Q as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(Q, rv as *mut libc::c_void, 0o2 as libc::c_int);
    return rv;
}
unsafe extern "C" fn update(
    mut Q: *mut Dict_t,
    mut dest: *mut Agnode_t,
    mut src: *mut Agnode_t,
    mut len: libc::c_double,
) {
    let mut newlen: libc::c_double = (*((*src).base.data as *mut nodedata_t)).dist + len;
    let mut oldlen: libc::c_double = (*((*dest).base.data as *mut nodedata_t)).dist;
    if oldlen == 0 as libc::c_int as libc::c_double {
        (*((*dest).base.data as *mut nodedata_t)).dist = newlen;
        if doPath {
            let ref mut fresh0 = (*((*dest).base.data as *mut nodedata_t)).prev;
            *fresh0 = src;
        }
        (Some(((*(Q as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(Q, dest as *mut libc::c_void, 0o1 as libc::c_int);
    } else if newlen < oldlen {
        (Some(((*(Q as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(Q, dest as *mut libc::c_void, 0o2 as libc::c_int);
        (*((*dest).base.data as *mut nodedata_t)).dist = newlen;
        if doPath {
            let ref mut fresh1 = (*((*dest).base.data as *mut nodedata_t)).prev;
            *fresh1 = src;
        }
        (Some(((*(Q as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(Q, dest as *mut libc::c_void, 0o1 as libc::c_int);
    }
}
unsafe extern "C" fn pre(mut g: *mut Agraph_t) {
    len_sym = agattr(
        g,
        2 as libc::c_int,
        b"len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char,
    );
    aginit(
        g,
        1 as libc::c_int,
        b"dijkstra\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<nodedata_t>() as libc::c_ulong as libc::c_int,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn post(mut g: *mut Agraph_t) {
    let mut v: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut prev: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut dflt: [libc::c_char; 256] = [0; 256];
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut psym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut dist: libc::c_double = 0.;
    let mut oldmax: libc::c_double = 0.;
    let mut maxdist: libc::c_double = 0.0f64;
    sym = agattr(
        g,
        1 as libc::c_int,
        b"dist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if doPath {
        psym = agattr(
            g,
            1 as libc::c_int,
            b"prev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if setall {
        snprintf(
            dflt.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%.3lf\0" as *const u8 as *const libc::c_char,
            ::std::f64::INFINITY,
        );
    }
    v = agfstnode(g);
    while !v.is_null() {
        dist = (*((*v).base.data as *mut nodedata_t)).dist;
        if dist != 0. {
            dist -= 1.;
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%.3lf\0" as *const u8 as *const libc::c_char,
                dist,
            );
            agxset(v as *mut libc::c_void, sym, buf.as_mut_ptr());
            if doPath as libc::c_int != 0
                && {
                    prev = (*((*v).base.data as *mut nodedata_t)).prev;
                    !prev.is_null()
                }
            {
                agxset(
                    v as *mut libc::c_void,
                    psym,
                    agnameof(prev as *mut libc::c_void),
                );
            }
            if maxdist < dist {
                maxdist = dist;
            }
        } else if setall {
            agxset(v as *mut libc::c_void, sym, dflt.as_mut_ptr());
        }
        v = agnxtnode(g, v);
    }
    sym = agattrsym(
        g as *mut libc::c_void,
        b"maxdist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !sym.is_null() {
        if !setall {
            oldmax = atof(agxget(g as *mut libc::c_void, sym));
            if oldmax > maxdist {
                maxdist = oldmax;
            }
        }
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%.3lf\0" as *const u8 as *const libc::c_char,
            maxdist,
        );
        agxset(g as *mut libc::c_void, sym, buf.as_mut_ptr());
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%.3lf\0" as *const u8 as *const libc::c_char,
            maxdist,
        );
        agattr(
            g,
            0 as libc::c_int,
            b"maxdist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    agclean(
        g,
        1 as libc::c_int,
        b"dijkstra\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    agclean(
        g,
        2 as libc::c_int,
        b"dijkstra\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn dijkstra(
    mut Q: *mut Dict_t,
    mut G: *mut Agraph_t,
    mut n: *mut Agnode_t,
) {
    let mut u: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    pre(G);
    (*((*n).base.data as *mut nodedata_t)).dist = 1 as libc::c_int as libc::c_double;
    (Some(((*(Q as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(Q, n as *mut libc::c_void, 0o1 as libc::c_int);
    if doDirected {
        loop {
            u = extract_min(Q);
            if u.is_null() {
                break;
            }
            (*((*u).base.data as *mut nodedata_t)).done = 1 as libc::c_int;
            e = agfstout(G, u);
            while !e.is_null() {
                if (*((*(*e).node).base.data as *mut nodedata_t)).done == 0 {
                    update(Q, (*e).node, u, getlength(e));
                }
                e = agnxtout(G, e);
            }
        }
    } else {
        loop {
            u = extract_min(Q);
            if u.is_null() {
                break;
            }
            (*((*u).base.data as *mut nodedata_t)).done = 1 as libc::c_int;
            e = agfstedge(G, u);
            while !e.is_null() {
                if (*((*(*e).node).base.data as *mut nodedata_t)).done == 0 {
                    update(Q, (*e).node, u, getlength(e));
                }
                e = agnxtedge(G, e, u);
            }
        }
    }
    post(G);
}
static mut useString: *mut libc::c_char = b"Usage: dijkstra [-ap?] <node> [<file> <node> <file>]\n  -a - for nodes in a different component, set dist very large\n  -d - use forward directed edges\n  -p - attach shortest path info\n  -? - print usage\nIf no files are specified, stdin is used\n\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn usage(mut v: libc::c_int) {
    printf(b"%s\0" as *const u8 as *const libc::c_char, useString);
    graphviz_exit(v);
}
unsafe extern "C" fn init(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    CmdName = *argv.offset(0 as libc::c_int as isize);
    opterr = 0 as libc::c_int;
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"adp?\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            97 => {
                setall = 1 as libc::c_int != 0;
            }
            100 => {
                doDirected = 1 as libc::c_int != 0;
            }
            112 => {
                doPath = 1 as libc::c_int != 0;
            }
            63 => {
                if optopt == '\0' as i32 || optopt == '?' as i32 {
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
    if argc == 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s: no node specified\n\0" as *const u8 as *const libc::c_char,
            CmdName,
        );
        usage(1 as libc::c_int);
    }
    Files = calloc(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        (argc as size_t)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    Nodes = calloc(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        (argc as size_t)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    j = i;
    while i < argc {
        let fresh2 = i;
        i = i + 1;
        let ref mut fresh3 = *Nodes.offset(j as isize);
        *fresh3 = *argv.offset(fresh2 as isize);
        let ref mut fresh4 = *Files.offset(j as isize);
        *fresh4 = (if !(*argv.offset(i as isize)).is_null() {
            *argv.offset(i as isize) as *const libc::c_char
        } else {
            b"-\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        j += 1;
        i += 1;
    }
    let ref mut fresh5 = *Files.offset(j as isize);
    *fresh5 = 0 as *mut libc::c_char;
    let ref mut fresh6 = *Nodes.offset(j as isize);
    *fresh6 = *fresh5;
}
unsafe extern "C" fn gread(mut fp: *mut FILE) -> *mut Agraph_t {
    return agread(fp as *mut libc::c_void, 0 as *mut Agdisc_t);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
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
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut code: libc::c_int = 0 as libc::c_int;
    let mut Q: *mut Dict_t = 0 as *mut Dict_t;
    init(argc, argv);
    newIngraph(
        &mut ig,
        Files,
        Some(gread as unsafe extern "C" fn(*mut FILE) -> *mut Agraph_t),
    );
    Q = dtopen(&mut MyDisc, Dtoset);
    loop {
        g = nextGraph(&mut ig);
        if g.is_null() {
            break;
        }
        (Some(((*(Q as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(Q, 0 as *mut libc::c_void, 0o100 as libc::c_int);
        n = agnode(g, *Nodes.offset(i as isize), 0 as libc::c_int);
        if !n.is_null() {
            dijkstra(Q, g, n);
        } else {
            fprintf(
                stderr,
                b"%s: no node %s in graph %s in %s\n\0" as *const u8
                    as *const libc::c_char,
                CmdName,
                *Nodes.offset(i as isize),
                agnameof(g as *mut libc::c_void),
                fileName(&mut ig),
            );
            code = 1 as libc::c_int;
        }
        agwrite(g, stdout as *mut libc::c_void);
        fflush(stdout);
        agclose(g);
        i = i.wrapping_add(1);
    }
    free(Nodes as *mut libc::c_void);
    free(Files as *mut libc::c_void);
    graphviz_exit(code);
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
