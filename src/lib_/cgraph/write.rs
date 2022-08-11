#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn agisdirected(g: *mut Agraph_t) -> libc::c_int;
    fn agisstrict(g: *mut Agraph_t) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn dtview(_: *mut Dt_t, _: *mut Dt_t) -> *mut Dt_t;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn aghtmlstr(_: *const libc::c_char) -> libc::c_int;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agnameof(_: *mut libc::c_void) -> *mut libc::c_char;
    fn agattrrec(obj: *mut libc::c_void) -> *mut Agattr_t;
    fn agxget(obj: *mut libc::c_void, sym: *mut Agsym_t) -> *mut libc::c_char;
    fn agsubedge(
        g: *mut Agraph_t,
        e: *mut Agedge_t,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agdatadict(g: *mut Agraph_t, cflag: libc::c_int) -> *mut Agdatadict_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agsubnode(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agattr(
        g: *mut Agraph_t,
        kind: libc::c_int,
        name: *mut libc::c_char,
        value: *const libc::c_char,
    ) -> *mut Agsym_t;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn agget(obj: *mut libc::c_void, name: *mut libc::c_char) -> *mut libc::c_char;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agattr_s {
    pub h: Agrec_t,
    pub dict: *mut Dict_t,
    pub str_0: *mut *mut libc::c_char,
}
pub type Agattr_t = Agattr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdatadict_s {
    pub h: Agrec_t,
    pub dict: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub n: *mut Dict_t,
    pub e: *mut Dict_t,
    pub g: *mut Dict_t,
}
pub type Agdatadict_t = Agdatadict_s;
pub type iochan_t = ();
unsafe extern "C" fn ioput(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    return ((*(*(*g).clos).disc.io).putstr)
        .expect("non-null function pointer")(ofile, str);
}
static mut Level: libc::c_int = 0;
static mut Max_outputline: libc::c_int = 128 as libc::c_int;
static mut Tailport: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
static mut Headport: *mut Agsym_t = 0 as *const Agsym_t as *mut Agsym_t;
unsafe extern "C" fn indent(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = Level;
    while i > 0 as libc::c_int {
        if ioput(
            g,
            ofile,
            b"\t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        i -= 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_id_char(mut c: libc::c_char) -> bool {
    return *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
        || c as libc::c_int == '.' as i32 || c as libc::c_int == '-' as i32
        || !(c as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int);
}
unsafe extern "C" fn _agstrcanon(
    mut arg: *mut libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uc: libc::c_char = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut dotcnt: libc::c_int = 0 as libc::c_int;
    let mut needs_quotes: bool = 0 as libc::c_int != 0;
    let mut maybe_num: bool = false;
    let mut backslash_pending: bool = 0 as libc::c_int != 0;
    static mut tokenlist: [*const libc::c_char; 7] = [
        b"node\0" as *const u8 as *const libc::c_char,
        b"edge\0" as *const u8 as *const libc::c_char,
        b"strict\0" as *const u8 as *const libc::c_char,
        b"graph\0" as *const u8 as *const libc::c_char,
        b"digraph\0" as *const u8 as *const libc::c_char,
        b"subgraph\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut tok: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if arg.is_null()
        || *arg.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return b"\"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    s = arg;
    p = buf;
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '"' as i32 as libc::c_char;
    let fresh1 = s;
    s = s.offset(1);
    uc = *fresh1;
    maybe_num = *(*__ctype_b_loc()).offset(uc as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 as libc::c_int
        || uc as libc::c_int == '.' as i32 || uc as libc::c_int == '-' as i32;
    while uc != 0 {
        if uc as libc::c_int == '"' as i32 {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = '\\' as i32 as libc::c_char;
            needs_quotes = 1 as libc::c_int != 0;
        } else if maybe_num {
            if uc as libc::c_int == '-' as i32 {
                if cnt != 0 {
                    maybe_num = 0 as libc::c_int != 0;
                    needs_quotes = 1 as libc::c_int != 0;
                }
            } else if uc as libc::c_int == '.' as i32 {
                let fresh3 = dotcnt;
                dotcnt = dotcnt + 1;
                if fresh3 != 0 {
                    maybe_num = 0 as libc::c_int != 0;
                    needs_quotes = 1 as libc::c_int != 0;
                }
            } else if *(*__ctype_b_loc()).offset(uc as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                maybe_num = 0 as libc::c_int != 0;
                needs_quotes = 1 as libc::c_int != 0;
            }
        } else if !(*(*__ctype_b_loc()).offset(uc as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || uc as libc::c_int == '_' as i32
                || !(uc as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int))
            {
            needs_quotes = 1 as libc::c_int != 0;
        }
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = uc;
        let fresh5 = s;
        s = s.offset(1);
        uc = *fresh5;
        cnt += 1;
        if Max_outputline != 0 {
            if uc as libc::c_int != 0 && backslash_pending as libc::c_int != 0
                && !(is_id_char(*p.offset(-(1 as libc::c_int) as isize)) as libc::c_int
                    != 0
                    || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32) && is_id_char(uc) as libc::c_int != 0
            {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = '\\' as i32 as libc::c_char;
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = '\n' as i32 as libc::c_char;
                needs_quotes = 1 as libc::c_int != 0;
                backslash_pending = 0 as libc::c_int != 0;
                cnt = 0 as libc::c_int;
            } else if uc as libc::c_int != 0 && cnt >= Max_outputline {
                if !(is_id_char(*p.offset(-(1 as libc::c_int) as isize)) as libc::c_int
                    != 0
                    || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32) && is_id_char(uc) as libc::c_int != 0
                {
                    let fresh8 = p;
                    p = p.offset(1);
                    *fresh8 = '\\' as i32 as libc::c_char;
                    let fresh9 = p;
                    p = p.offset(1);
                    *fresh9 = '\n' as i32 as libc::c_char;
                    needs_quotes = 1 as libc::c_int != 0;
                    cnt = 0 as libc::c_int;
                } else {
                    backslash_pending = 1 as libc::c_int != 0;
                }
            }
        }
    }
    let fresh10 = p;
    p = p.offset(1);
    *fresh10 = '"' as i32 as libc::c_char;
    *p = '\0' as i32 as libc::c_char;
    if needs_quotes as libc::c_int != 0
        || cnt == 1 as libc::c_int
            && (*arg as libc::c_int == '.' as i32 || *arg as libc::c_int == '-' as i32)
    {
        return buf;
    }
    tok = tokenlist.as_mut_ptr();
    while !(*tok).is_null() {
        if strcasecmp(*tok, arg) == 0 {
            return buf;
        }
        tok = tok.offset(1);
    }
    return arg;
}
unsafe extern "C" fn agcanonhtmlstr(
    mut arg: *const libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    sprintf(buf, b"<%s>\0" as *const u8 as *const libc::c_char, arg);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn agstrcanon(
    mut arg: *mut libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    if aghtmlstr(arg) != 0 {
        return agcanonhtmlstr(arg, buf)
    } else {
        return _agstrcanon(arg, buf)
    };
}
unsafe extern "C" fn getoutputbuffer(mut str: *const libc::c_char) -> *mut libc::c_char {
    static mut rv: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut len: size_t = 0 as libc::c_int as size_t;
    let mut req: size_t = 0;
    req = if (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(strlen(str))
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        > 8192 as libc::c_int as libc::c_ulong
    {
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(strlen(str))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
    } else {
        8192 as libc::c_int as libc::c_ulong
    };
    if req > len {
        let mut r: *mut libc::c_char = realloc(rv as *mut libc::c_void, req)
            as *mut libc::c_char;
        if (r == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as *mut libc::c_char;
        }
        rv = r;
        len = req;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agcanonStr(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut buffer: *mut libc::c_char = getoutputbuffer(str);
    if (buffer == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        return 0 as *mut libc::c_char;
    }
    return agstrcanon(str, buffer);
}
#[no_mangle]
pub unsafe extern "C" fn agcanon(
    mut str: *mut libc::c_char,
    mut html: libc::c_int,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = getoutputbuffer(str);
    if (buf == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        return 0 as *mut libc::c_char;
    }
    if html != 0 {
        return agcanonhtmlstr(str, buf)
    } else {
        return _agstrcanon(str, buf)
    };
}
unsafe extern "C" fn _write_canonstr(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
    mut str: *mut libc::c_char,
    mut chk: libc::c_int,
) -> libc::c_int {
    if chk != 0 {
        str = agcanonStr(str);
    } else {
        let mut buffer: *mut libc::c_char = getoutputbuffer(str);
        if (buffer == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
            as libc::c_long != 0
        {
            return -(1 as libc::c_int);
        }
        str = _agstrcanon(str, buffer);
    }
    return ioput(g, ofile, str);
}
unsafe extern "C" fn write_canonstr(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    s = agstrdup(g, str);
    r = _write_canonstr(g, ofile, s, (0 as libc::c_int == 0) as libc::c_int);
    agstrfree(g, s);
    return r;
}
unsafe extern "C" fn write_dict(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
    mut name: *mut libc::c_char,
    mut dict: *mut Dict_t,
    mut top: libc::c_int,
) -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut view: *mut Dict_t = 0 as *mut Dict_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut psym: *mut Agsym_t = 0 as *mut Agsym_t;
    if top == 0 {
        view = dtview(dict, 0 as *mut Dt_t);
    } else {
        view = 0 as *mut Dict_t;
    }
    let mut current_block_41: u64;
    sym = (Some(((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(dict, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agsym_t;
    while !sym.is_null() {
        if (((*sym).defval).is_null()
            || *((*sym).defval).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32) && (*sym).print == 0
        {
            if view.is_null() {
                current_block_41 = 7095457783677275021;
            } else {
                psym = (Some(
                    ((*(view as *mut Dt_t)).searchf).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(view, sym as *mut libc::c_void, 0o4 as libc::c_int)
                    as *mut Agsym_t;
                if !psym.is_null() {} else {
                    __assert_fail(
                        b"psym\0" as *const u8 as *const libc::c_char,
                        b"write.c\0" as *const u8 as *const libc::c_char,
                        256 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 62],
                            &[libc::c_char; 62],
                        >(
                            b"int write_dict(Agraph_t *, iochan_t *, char *, Dict_t *, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if (((*psym).defval).is_null()
                    || *((*psym).defval).offset(0 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32) && (*psym).print as libc::c_int != 0
                {
                    current_block_41 = 7095457783677275021;
                } else {
                    current_block_41 = 12209867499936983673;
                }
            }
        } else {
            current_block_41 = 12209867499936983673;
        }
        match current_block_41 {
            12209867499936983673 => {
                let fresh11 = cnt;
                cnt = cnt + 1;
                if fresh11 == 0 as libc::c_int {
                    if indent(g, ofile) == -(1 as libc::c_int) {
                        return -(1 as libc::c_int);
                    }
                    if ioput(g, ofile, name) == -(1 as libc::c_int) {
                        return -(1 as libc::c_int);
                    }
                    if ioput(
                        g,
                        ofile,
                        b" [\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == -(1 as libc::c_int)
                    {
                        return -(1 as libc::c_int);
                    }
                    Level += 1;
                } else {
                    if ioput(
                        g,
                        ofile,
                        b",\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == -(1 as libc::c_int)
                    {
                        return -(1 as libc::c_int);
                    }
                    if indent(g, ofile) == -(1 as libc::c_int) {
                        return -(1 as libc::c_int);
                    }
                }
                if write_canonstr(g, ofile, (*sym).name) == -(1 as libc::c_int) {
                    return -(1 as libc::c_int);
                }
                if ioput(
                    g,
                    ofile,
                    b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == -(1 as libc::c_int)
                {
                    return -(1 as libc::c_int);
                }
                if write_canonstr(g, ofile, (*sym).defval) == -(1 as libc::c_int) {
                    return -(1 as libc::c_int);
                }
            }
            _ => {}
        }
        sym = (Some(
            ((*(dict as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(dict, sym as *mut libc::c_void, 0o10 as libc::c_int) as *mut Agsym_t;
    }
    if cnt > 0 as libc::c_int {
        Level -= 1;
        if cnt > 1 as libc::c_int {
            if ioput(
                g,
                ofile,
                b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            if indent(g, ofile) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
        if ioput(
            g,
            ofile,
            b"];\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
    }
    if top == 0 {
        dtview(dict, view);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_dicts(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
    mut top: libc::c_int,
) -> libc::c_int {
    let mut def: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    def = agdatadict(g, 0 as libc::c_int);
    if !def.is_null() {
        if write_dict(
            g,
            ofile,
            b"graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*def).dict.g,
            top,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        if write_dict(
            g,
            ofile,
            b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*def).dict.n,
            top,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        if write_dict(
            g,
            ofile,
            b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*def).dict.e,
            top,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_hdr(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
    mut top: libc::c_int,
) -> libc::c_int {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kind: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strict: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut root: bool = 0 as libc::c_int != 0;
    let mut hasName: bool = 1 as libc::c_int != 0;
    strict = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if top == 0 && !(agparent(g)).is_null() {
        kind = b"sub\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        root = 1 as libc::c_int != 0;
        if ((*g).desc).directed() != 0 {
            kind = b"di\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            kind = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if agisstrict(g) != 0 {
            strict = b"strict \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        Tailport = agattr(
            g,
            2 as libc::c_int,
            b"tailport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
        Headport = agattr(
            g,
            2 as libc::c_int,
            b"headport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char,
        );
    }
    name = agnameof(g as *mut libc::c_void);
    sep = b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
    {
        name = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        sep = name;
        hasName = 0 as libc::c_int != 0;
    }
    if indent(g, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if ioput(g, ofile, strict) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if root as libc::c_int != 0 || hasName as libc::c_int != 0 {
        if ioput(g, ofile, kind) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if ioput(
            g,
            ofile,
            b"graph \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
    }
    if hasName {
        if write_canonstr(g, ofile, name) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if ioput(g, ofile, sep) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if ioput(g, ofile, b"{\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    Level += 1;
    if write_dicts(g, ofile, top) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    let ref mut fresh12 = (*(g as *mut Agobj_t)).tag;
    (*fresh12).set_attrwf((0 as libc::c_int == 0) as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_trl(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
) -> libc::c_int {
    Level -= 1;
    if indent(g, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if ioput(g, ofile, b"}\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn irrelevant_subgraph(mut g: *mut Agraph_t) -> bool {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sdata: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut pdata: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut rdata: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut dd: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = agnameof(g as *mut libc::c_void);
    if !name.is_null()
        && *name.offset(0 as libc::c_int as isize) as libc::c_int != '%' as i32
    {
        return 0 as libc::c_int != 0;
    }
    sdata = agattrrec(g as *mut libc::c_void);
    if !sdata.is_null()
        && {
            pdata = agattrrec(agparent(g) as *mut libc::c_void);
            !pdata.is_null()
        }
    {
        rdata = agattrrec(agroot(g as *mut libc::c_void) as *mut libc::c_void);
        n = dtsize((*rdata).dict);
        i = 0 as libc::c_int;
        while i < n {
            if !(*((*sdata).str_0).offset(i as isize)).is_null()
                && !(*((*pdata).str_0).offset(i as isize)).is_null()
                && strcmp(
                    *((*sdata).str_0).offset(i as isize),
                    *((*pdata).str_0).offset(i as isize),
                ) != 0
            {
                return 0 as libc::c_int != 0;
            }
            i += 1;
        }
    }
    dd = agdatadict(g, 0 as libc::c_int);
    if dd.is_null() {
        return 1 as libc::c_int != 0;
    }
    if dtsize((*dd).dict.n) > 0 as libc::c_int || dtsize((*dd).dict.e) > 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn node_in_subg(mut g: *mut Agraph_t, mut n: *mut Agnode_t) -> bool {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        if !irrelevant_subgraph(subg) {
            if !(agsubnode(subg, n, 0 as libc::c_int)).is_null() {
                return 1 as libc::c_int != 0;
            }
        }
        subg = agnxtsubg(subg);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn has_no_edges(mut g: *mut Agraph_t, mut n: *mut Agnode_t) -> bool {
    return (agfstin(g, n)).is_null() && (agfstout(g, n)).is_null();
}
unsafe extern "C" fn has_no_predecessor_below(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut val: uint64_t,
) -> bool {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    if (((*(n as *mut Agobj_t)).tag).seq() as libc::c_ulong) < val {
        return 0 as libc::c_int != 0;
    }
    e = agfstin(g, n);
    while !e.is_null() {
        if (((*((*e).node as *mut Agobj_t)).tag).seq() as libc::c_ulong) < val {
            return 0 as libc::c_int != 0;
        }
        e = agnxtin(g, e);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn not_default_attrs(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> bool {
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    data = agattrrec(n as *mut libc::c_void);
    if !data.is_null() {
        sym = (Some(
            ((*((*data).dict as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*data).dict, 0 as *mut libc::c_void, 0o200 as libc::c_int)
            as *mut Agsym_t;
        while !sym.is_null() {
            if *((*data).str_0).offset((*sym).id as isize) != (*sym).defval {
                return 1 as libc::c_int != 0;
            }
            sym = (Some(
                ((*((*data).dict as *mut Dt_t)).searchf)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )((*data).dict, sym as *mut libc::c_void, 0o10 as libc::c_int)
                as *mut Agsym_t;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn write_subgs(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
) -> libc::c_int {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        if irrelevant_subgraph(subg) {
            write_subgs(subg, ofile);
        } else {
            if write_hdr(subg, ofile, 0 as libc::c_int) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            if write_body(subg, ofile) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            if write_trl(subg, ofile) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
        subg = agnxtsubg(subg);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_edge_name(
    mut e: *mut Agedge_t,
    mut ofile: *mut libc::c_void,
    mut terminate: libc::c_int,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    p = agnameof(e as *mut libc::c_void);
    g = agraphof(e as *mut libc::c_void);
    if !(p.is_null()
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32)
    {
        if terminate == 0 {
            Level += 1;
        }
        if ioput(
            g,
            ofile,
            b"\t[key=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        if write_canonstr(g, ofile, p) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if terminate != 0 {
            if ioput(
                g,
                ofile,
                b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
        }
        rv = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        rv = 0 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn write_nondefault_attrs(
    mut obj: *mut libc::c_void,
    mut ofile: *mut libc::c_void,
    mut defdict: *mut Dict_t,
) -> libc::c_int {
    let mut data: *mut Agattr_t = 0 as *mut Agattr_t;
    let mut sym: *mut Agsym_t = 0 as *mut Agsym_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut rv: libc::c_int = 0;
    if ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int
        || ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int
    {
        rv = write_edge_name(obj as *mut Agedge_t, ofile, 0 as libc::c_int);
        if rv == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if rv != 0 {
            cnt += 1;
        }
    }
    data = agattrrec(obj);
    g = agraphof(obj);
    if !data.is_null() {
        let mut current_block_40: u64;
        sym = (Some(
            ((*(defdict as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(defdict, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agsym_t;
        while !sym.is_null() {
            if ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
                || ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
            {
                if !Tailport.is_null() && (*sym).id == (*Tailport).id {
                    current_block_40 = 11050875288958768710;
                } else if !Headport.is_null() && (*sym).id == (*Headport).id {
                    current_block_40 = 11050875288958768710;
                } else {
                    current_block_40 = 6009453772311597924;
                }
            } else {
                current_block_40 = 6009453772311597924;
            }
            match current_block_40 {
                6009453772311597924 => {
                    if *((*data).str_0).offset((*sym).id as isize) != (*sym).defval {
                        let fresh13 = cnt;
                        cnt = cnt + 1;
                        if fresh13 == 0 as libc::c_int {
                            if ioput(
                                g,
                                ofile,
                                b"\t[\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ) == -(1 as libc::c_int)
                            {
                                return -(1 as libc::c_int);
                            }
                            Level += 1;
                        } else {
                            if ioput(
                                g,
                                ofile,
                                b",\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ) == -(1 as libc::c_int)
                            {
                                return -(1 as libc::c_int);
                            }
                            if indent(g, ofile) == -(1 as libc::c_int) {
                                return -(1 as libc::c_int);
                            }
                        }
                        if write_canonstr(g, ofile, (*sym).name) == -(1 as libc::c_int) {
                            return -(1 as libc::c_int);
                        }
                        if ioput(
                            g,
                            ofile,
                            b"=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ) == -(1 as libc::c_int)
                        {
                            return -(1 as libc::c_int);
                        }
                        if write_canonstr(
                            g,
                            ofile,
                            *((*data).str_0).offset((*sym).id as isize),
                        ) == -(1 as libc::c_int)
                        {
                            return -(1 as libc::c_int);
                        }
                    }
                }
                _ => {}
            }
            sym = (Some(
                ((*(defdict as *mut Dt_t)).searchf).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(defdict, sym as *mut libc::c_void, 0o10 as libc::c_int)
                as *mut Agsym_t;
        }
    }
    if cnt > 0 as libc::c_int {
        if ioput(
            g,
            ofile,
            b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        Level -= 1;
    }
    let ref mut fresh14 = (*(obj as *mut Agobj_t)).tag;
    (*fresh14).set_attrwf((0 as libc::c_int == 0) as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_nodename(
    mut n: *mut Agnode_t,
    mut ofile: *mut libc::c_void,
) -> libc::c_int {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    name = agnameof(n as *mut libc::c_void);
    g = agraphof(n as *mut libc::c_void);
    if !name.is_null() {
        if write_canonstr(g, ofile, name) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    } else {
        let mut buf: [libc::c_char; 30] = [0; 30];
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            b"_%lu_SUSPECT\0" as *const u8 as *const libc::c_char,
            (*(n as *mut Agobj_t)).tag.id,
        );
        if ioput(g, ofile, buf.as_mut_ptr()) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn attrs_written(mut obj: *mut libc::c_void) -> libc::c_int {
    return ((*(obj as *mut Agobj_t)).tag).attrwf() as libc::c_int;
}
unsafe extern "C" fn write_node(
    mut n: *mut Agnode_t,
    mut ofile: *mut libc::c_void,
    mut d: *mut Dict_t,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    g = agraphof(n as *mut libc::c_void);
    if indent(g, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if write_nodename(n, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if attrs_written(n as *mut libc::c_void) == 0 {
        if write_nondefault_attrs(n as *mut libc::c_void, ofile, d)
            == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
    }
    return ioput(
        g,
        ofile,
        b";\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn write_node_test(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut pred_id: uint64_t,
) -> bool {
    if !node_in_subg(g, n) && has_no_predecessor_below(g, n, pred_id) as libc::c_int != 0
    {
        if has_no_edges(g, n) as libc::c_int != 0
            || not_default_attrs(g, n) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn write_port(
    mut e: *mut Agedge_t,
    mut ofile: *mut libc::c_void,
    mut port: *mut Agsym_t,
) -> libc::c_int {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    if port.is_null() {
        return 0 as libc::c_int;
    }
    g = agraphof(e as *mut libc::c_void);
    val = agxget(e as *mut libc::c_void, port);
    if *val.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    if ioput(g, ofile, b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    if aghtmlstr(val) != 0 {
        if write_canonstr(g, ofile, val) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    } else {
        let mut s: *mut libc::c_char = strchr(val, ':' as i32);
        if !s.is_null() {
            *s = '\0' as i32 as libc::c_char;
            if _write_canonstr(g, ofile, val, 0 as libc::c_int) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            if ioput(
                g,
                ofile,
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            if _write_canonstr(
                g,
                ofile,
                s.offset(1 as libc::c_int as isize),
                0 as libc::c_int,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            *s = ':' as i32 as libc::c_char;
        } else if _write_canonstr(g, ofile, val, 0 as libc::c_int) == -(1 as libc::c_int)
            {
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_edge_test(
    mut g: *mut Agraph_t,
    mut e: *mut Agedge_t,
) -> libc::c_int {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    subg = agfstsubg(g);
    while !subg.is_null() {
        if !irrelevant_subgraph(subg) {
            if !(agsubedge(subg, e, 0 as libc::c_int)).is_null() {
                return 0 as libc::c_int;
            }
        }
        subg = agnxtsubg(subg);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn write_edge(
    mut e: *mut Agedge_t,
    mut ofile: *mut libc::c_void,
    mut d: *mut Dict_t,
) -> libc::c_int {
    let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    t = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    h = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    g = agraphof(t as *mut libc::c_void);
    if indent(g, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if write_nodename(t, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if write_port(e, ofile, Tailport) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if ioput(
        g,
        ofile,
        (if agisdirected(agraphof(t as *mut libc::c_void)) != 0 {
            b" -> \0" as *const u8 as *const libc::c_char
        } else {
            b" -- \0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    if write_nodename(h, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if write_port(e, ofile, Headport) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if attrs_written(e as *mut libc::c_void) == 0 {
        if write_nondefault_attrs(e as *mut libc::c_void, ofile, d)
            == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
    } else if write_edge_name(e, ofile, (0 as libc::c_int == 0) as libc::c_int)
            == -(1 as libc::c_int)
        {
        return -(1 as libc::c_int)
    }
    return ioput(
        g,
        ofile,
        b";\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn write_body(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut prev: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut dd: *mut Agdatadict_t = 0 as *mut Agdatadict_t;
    if write_subgs(g, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    dd = agdatadict(g, 0 as libc::c_int);
    n = agfstnode(g);
    while !n.is_null() {
        if write_node_test(g, n, ((*(n as *mut Agobj_t)).tag).seq() as uint64_t) {
            if write_node(
                n,
                ofile,
                (if !dd.is_null() { (*dd).dict.n } else { 0 as *mut Dict_t }),
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
        }
        prev = n;
        e = agfstout(g, n);
        while !e.is_null() {
            if prev
                != (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                }))
                    .node
                && write_node_test(
                    g,
                    (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node,
                    ((*(n as *mut Agobj_t)).tag).seq() as uint64_t,
                ) as libc::c_int != 0
            {
                if write_node(
                    (*(if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                        == 2 as libc::c_int
                    {
                        e
                    } else {
                        e.offset(-(1 as libc::c_int as isize))
                    }))
                        .node,
                    ofile,
                    (if !dd.is_null() { (*dd).dict.n } else { 0 as *mut Dict_t }),
                ) == -(1 as libc::c_int)
                {
                    return -(1 as libc::c_int);
                }
                prev = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
                    == 2 as libc::c_int
                {
                    e
                } else {
                    e.offset(-(1 as libc::c_int as isize))
                })
                    .node;
            }
            if write_edge_test(g, e) != 0 {
                if write_edge(
                    e,
                    ofile,
                    (if !dd.is_null() { (*dd).dict.e } else { 0 as *mut Dict_t }),
                ) == -(1 as libc::c_int)
                {
                    return -(1 as libc::c_int);
                }
            }
            e = agnxtout(g, e);
        }
        n = agnxtnode(g, n);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_attrwf(
    mut g: *mut Agraph_t,
    mut toplevel: bool,
    mut value: bool,
) {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let ref mut fresh15 = (*(g as *mut Agobj_t)).tag;
    (*fresh15).set_attrwf(value as libc::c_uint);
    subg = agfstsubg(g);
    while !subg.is_null() {
        set_attrwf(subg, 0 as libc::c_int != 0, value);
        subg = agnxtsubg(subg);
    }
    if toplevel {
        n = agfstnode(g);
        while !n.is_null() {
            let ref mut fresh16 = (*(n as *mut Agobj_t)).tag;
            (*fresh16).set_attrwf(value as libc::c_uint);
            e = agfstout(g, n);
            while !e.is_null() {
                let ref mut fresh17 = (*(e as *mut Agobj_t)).tag;
                (*fresh17).set_attrwf(value as libc::c_uint);
                e = agnxtout(g, e);
            }
            n = agnxtnode(g, n);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn agwrite(
    mut g: *mut Agraph_t,
    mut ofile: *mut libc::c_void,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    Level = 0 as libc::c_int;
    s = agget(
        g as *mut libc::c_void,
        b"linelength\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !s.is_null()
        && *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut len: libc::c_ulong = strtoul(
            s,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        if (len == 0 as libc::c_int as libc::c_ulong
            || len >= 60 as libc::c_int as libc::c_ulong)
            && len <= 2147483647 as libc::c_int as libc::c_ulong
        {
            Max_outputline = len as libc::c_int;
        }
    }
    set_attrwf(g, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    if write_hdr(g, ofile, (0 as libc::c_int == 0) as libc::c_int) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    if write_body(g, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if write_trl(g, ofile) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    Max_outputline = 128 as libc::c_int;
    return ((*(*(*g).clos).disc.io).flush).expect("non-null function pointer")(ofile);
}
