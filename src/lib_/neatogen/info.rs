#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type freeblock;
    pub type freenode;
    fn getfree(_: *mut Freelist) -> *mut libc::c_void;
    fn freeinit(_: *mut Freelist, _: libc::c_int);
}
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
pub type Agnode_t = Agnode_s;
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
pub type Dtlink_t = _dtlink_s;
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
pub type Agraph_t = Agraph_s;
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
pub type Dict_t = _dt_s;
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
pub type Dt_t = _dt_s;
pub type Dtmethod_t = _dtmethod_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtmethod_s {
    pub searchf: Dtsearch_f,
    pub type_0: libc::c_int,
}
pub type Dtsearch_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void>;
pub type Dtmemory_f = Option<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, size_t, *mut Dtdisc_t) -> *mut libc::c_void,
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
pub type Dtevent_f = Option<
    unsafe extern "C" fn(*mut Dt_t, libc::c_int, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_int,
>;
pub type Dthash_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_uint>;
pub type Dtcompar_f = Option<
    unsafe extern "C" fn(
        *mut Dt_t,
        *mut libc::c_void,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
pub type Dtfree_f = Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> ()>;
pub type Dtmake_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, *mut Dtdisc_t) -> *mut libc::c_void>;
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
pub type agobjfn_t =
    Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> ()>;
pub type Agobj_t = Agobj_s;
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
pub type Agtag_t = Agtag_s;
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
pub type IDTYPE = uint64_t;
pub type agobjupdfn_t = Option<
    unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void, *mut Agsym_t) -> (),
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
    pub afread: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub putstr: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type Agiddisc_t = Agiddisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agiddisc_s {
    pub open: Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agdisc_t) -> *mut libc::c_void>,
    pub map: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_char,
            *mut IDTYPE,
            libc::c_int,
        ) -> libc::c_long,
    >,
    pub alloc: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> libc::c_long>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> ()>,
    pub print:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> *mut libc::c_char>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub idregister:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, *mut libc::c_void) -> ()>,
}
pub type Agmemdisc_t = Agmemdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agmemdisc_s {
    pub open: Option<unsafe extern "C" fn(*mut Agdisc_t) -> *mut libc::c_void>,
    pub alloc: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>,
    pub resize: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            size_t,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
pub struct freelist {
    pub head: *mut freenode,
    pub blocklist: *mut freeblock,
    pub nodesize: libc::c_int,
}
pub type Freelist = freelist;
pub type Point = pointf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Site {
    pub coord: Point,
    pub sitenbr: libc::c_int,
    pub refcnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Poly {
    pub origin: Point,
    pub corner: Point,
    pub nverts: libc::c_int,
    pub verts: *mut Point,
    pub kind: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptitem {
    pub next: *mut ptitem,
    pub p: Point,
}
pub type PtItem = ptitem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Info_t {
    pub node: *mut Agnode_t,
    pub site: Site,
    pub overlaps: libc::c_int,
    pub poly: Poly,
    pub verts: *mut PtItem,
}
#[no_mangle]
pub static mut nodeInfo: *mut Info_t = 0 as *const Info_t as *mut Info_t;
static mut pfl: Freelist = Freelist {
    head: 0 as *const freenode as *mut freenode,
    blocklist: 0 as *const freeblock as *mut freeblock,
    nodesize: 0,
};
#[no_mangle]
pub unsafe extern "C" fn infoinit() {
    freeinit(
        &mut pfl,
        ::std::mem::size_of::<PtItem>() as libc::c_ulong as libc::c_int,
    );
}
unsafe extern "C" fn compare(
    mut o: *mut Point,
    mut p: *mut PtItem,
    mut q: *mut PtItem,
) -> libc::c_int {
    let mut x0: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    if q.is_null() {
        return -(1 as libc::c_int);
    }
    if (*p).p.x == (*q).p.x && (*p).p.y == (*q).p.y {
        return 0 as libc::c_int;
    }
    x0 = (*p).p.x - (*o).x;
    y0 = (*p).p.y - (*o).y;
    x1 = (*q).p.x - (*o).x;
    y1 = (*q).p.y - (*o).y;
    if x0 >= 0.0f64 {
        if x1 < 0.0f64 {
            return -(1 as libc::c_int);
        } else if x0 > 0.0f64 {
            if x1 > 0.0f64 {
                a = y1 / x1;
                b = y0 / x0;
                if b < a {
                    return -(1 as libc::c_int);
                } else if b > a {
                    return 1 as libc::c_int;
                } else if x0 < x1 {
                    return -(1 as libc::c_int);
                } else {
                    return 1 as libc::c_int;
                }
            } else if y1 > 0.0f64 {
                return -(1 as libc::c_int);
            } else {
                return 1 as libc::c_int;
            }
        } else if x1 > 0.0f64 {
            if y0 <= 0.0f64 {
                return -(1 as libc::c_int);
            } else {
                return 1 as libc::c_int;
            }
        } else if y0 < y1 {
            if y1 <= 0.0f64 {
                return 1 as libc::c_int;
            } else {
                return -(1 as libc::c_int);
            }
        } else if y0 <= 0.0f64 {
            return -(1 as libc::c_int);
        } else {
            return 1 as libc::c_int;
        }
    } else if x1 >= 0.0f64 {
        return 1 as libc::c_int;
    } else {
        a = y1 / x1;
        b = y0 / x0;
        if b < a {
            return -(1 as libc::c_int);
        } else if b > a {
            return 1 as libc::c_int;
        } else if x0 > x1 {
            return -(1 as libc::c_int);
        } else {
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn addVertex(mut s: *mut Site, mut x: libc::c_double, mut y: libc::c_double) {
    let mut ip: *mut Info_t = 0 as *mut Info_t;
    let mut p: *mut PtItem = 0 as *mut PtItem;
    let mut curr: *mut PtItem = 0 as *mut PtItem;
    let mut prev: *mut PtItem = 0 as *mut PtItem;
    let mut origin: *mut Point = &mut (*s).coord;
    let mut tmp: PtItem = PtItem {
        next: 0 as *mut ptitem,
        p: Point { x: 0., y: 0. },
    };
    let mut cmp: libc::c_int = 0;
    ip = nodeInfo.offset((*s).sitenbr as isize);
    curr = (*ip).verts;
    tmp.p.x = x;
    tmp.p.y = y;
    cmp = compare(origin, &mut tmp, curr);
    if cmp == 0 as libc::c_int {
        return;
    } else {
        if cmp < 0 as libc::c_int {
            p = getfree(&mut pfl) as *mut PtItem;
            (*p).p.x = x;
            (*p).p.y = y;
            let ref mut fresh0 = (*p).next;
            *fresh0 = curr;
            let ref mut fresh1 = (*ip).verts;
            *fresh1 = p;
            return;
        }
    }
    prev = curr;
    curr = (*curr).next;
    loop {
        cmp = compare(origin, &mut tmp, curr);
        if !(cmp > 0 as libc::c_int) {
            break;
        }
        prev = curr;
        curr = (*curr).next;
    }
    if cmp == 0 as libc::c_int {
        return;
    }
    p = getfree(&mut pfl) as *mut PtItem;
    (*p).p.x = x;
    (*p).p.y = y;
    let ref mut fresh2 = (*prev).next;
    *fresh2 = p;
    let ref mut fresh3 = (*p).next;
    *fresh3 = curr;
}
