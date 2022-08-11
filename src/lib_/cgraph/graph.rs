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
#![feature(label_break_value, register_tool)]
extern "C" {
    static mut AgMemDisc: Agmemdisc_t;
    static mut AgIdDisc: Agiddisc_t;
    static mut AgIoDisc: Agiodisc_t;
    fn agpopdisc(g: *mut Agraph_t, disc: *mut Agcbdisc_t) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agsubrep(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agsubnode_t;
    fn agfstin(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtin(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agdelsubg(g: *mut Agraph_t, sub: *mut Agraph_t) -> libc::c_int;
    fn agdelnode(g: *mut Agraph_t, arg_n: *mut Agnode_t) -> libc::c_int;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    static mut Dttree: *mut Dtmethod_t;
    fn dtextract(_: *mut Dt_t) -> *mut Dtlink_t;
    fn dtrestore(_: *mut Dt_t, _: *mut Dtlink_t) -> libc::c_int;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn agregister(g: *mut Agraph_t, objtype: libc::c_int, obj: *mut libc::c_void);
    fn agmethod_init(g: *mut Agraph_t, obj: *mut libc::c_void);
    fn agraphattr_init(g: *mut Agraph_t);
    fn agdictobjmem(
        dict: *mut Dict_t,
        p: *mut libc::c_void,
        size: size_t,
        disc: *mut Dtdisc_t,
    ) -> *mut libc::c_void;
    fn agdtopen(g: *mut Agraph_t, disc: *mut Dtdisc_t, method: *mut Dtmethod_t) -> *mut Dict_t;
    static mut Ag_mainedge_id_disc: Dtdisc_t;
    static mut Ag_subedge_seq_disc: Dtdisc_t;
    static mut Ag_mainedge_seq_disc: Dtdisc_t;
    static mut Ag_subnode_id_disc: Dtdisc_t;
    static mut Ag_subnode_seq_disc: Dtdisc_t;
    fn agmapnametoid(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        str: *mut libc::c_char,
        result: *mut IDTYPE,
        createflag: libc::c_int,
    ) -> libc::c_int;
    fn agstrclose(g: *mut Agraph_t) -> libc::c_int;
    fn agfreeid(g: *mut Agraph_t, objtype: libc::c_int, id: IDTYPE);
    fn agrecclose(obj: *mut Agobj_t);
    fn agraphattr_delete(g: *mut Agraph_t) -> libc::c_int;
    fn agdtclose(g: *mut Agraph_t, dict: *mut Dict_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agmethod_delete(g: *mut Agraph_t, obj: *mut libc::c_void);
    fn aginternalmapclose(g: *mut Agraph_t);
    static mut Ag_subedge_id_disc: Dtdisc_t;
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type ptrdiff_t = libc::c_long;
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
pub type Dtevent_f = Option<
    unsafe extern "C" fn(*mut Dt_t, libc::c_int, *mut libc::c_void, *mut Dtdisc_t) -> libc::c_int,
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
pub type Dtsearch_f =
    Option<unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, libc::c_int) -> *mut libc::c_void>;
pub type Dtmemory_f = Option<
    unsafe extern "C" fn(*mut Dt_t, *mut libc::c_void, size_t, *mut Dtdisc_t) -> *mut libc::c_void,
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
pub type agobjfn_t =
    Option<unsafe extern "C" fn(*mut Agraph_t, *mut Agobj_t, *mut libc::c_void) -> ()>;
pub type Agraph_t = Agraph_s;
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
#[no_mangle]
pub static mut Ag_G_global: *mut Agraph_t = 0 as *const Agraph_t as *mut Agraph_t;
unsafe extern "C" fn agclos(mut proto: *mut Agdisc_t) -> *mut Agclos_t {
    let mut memdisc: *mut Agmemdisc_t = 0 as *mut Agmemdisc_t;
    let mut memclosure: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rv: *mut Agclos_t = 0 as *mut Agclos_t;
    memdisc = if !proto.is_null() && !((*proto).mem).is_null() {
        (*proto).mem
    } else {
        &mut AgMemDisc
    };
    memclosure = ((*memdisc).open).expect("non-null function pointer")(proto);
    rv = ((*memdisc).alloc).expect("non-null function pointer")(
        memclosure,
        ::std::mem::size_of::<Agclos_t>() as libc::c_ulong,
    ) as *mut Agclos_t;
    let ref mut fresh0 = (*rv).disc.mem;
    *fresh0 = memdisc;
    let ref mut fresh1 = (*rv).state.mem;
    *fresh1 = memclosure;
    let ref mut fresh2 = (*rv).disc.id;
    *fresh2 = if !proto.is_null() && !((*proto).id).is_null() {
        (*proto).id
    } else {
        &mut AgIdDisc
    };
    let ref mut fresh3 = (*rv).disc.io;
    *fresh3 = if !proto.is_null() && !((*proto).io).is_null() {
        (*proto).io
    } else {
        &mut AgIoDisc
    };
    (*rv).callbacks_enabled = (0 as libc::c_int == 0) as libc::c_int as libc::c_uchar;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agopen(
    mut name: *mut libc::c_char,
    mut desc: Agdesc_t,
    mut arg_disc: *mut Agdisc_t,
) -> *mut Agraph_t {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut clos: *mut Agclos_t = 0 as *mut Agclos_t;
    let mut gid: IDTYPE = 0;
    clos = agclos(arg_disc);
    g = ((*(*clos).disc.mem).alloc).expect("non-null function pointer")(
        (*clos).state.mem,
        ::std::mem::size_of::<Agraph_t>() as libc::c_ulong,
    ) as *mut Agraph_t;
    let ref mut fresh4 = (*(g as *mut Agobj_t)).tag;
    (*fresh4).set_objtype(0 as libc::c_int as libc::c_uint);
    let ref mut fresh5 = (*g).clos;
    *fresh5 = clos;
    (*g).desc = desc;
    let ref mut fresh6 = (*g).desc;
    (*fresh6).set_maingraph((0 as libc::c_int == 0) as libc::c_int as libc::c_uint);
    let ref mut fresh7 = (*g).root;
    *fresh7 = g;
    let ref mut fresh8 = (*(*g).clos).state.id;
    *fresh8 = ((*(*(*g).clos).disc.id).open).expect("non-null function pointer")(g, arg_disc);
    if agmapnametoid(
        g,
        0 as libc::c_int,
        name,
        &mut gid,
        (0 as libc::c_int == 0) as libc::c_int,
    ) != 0
    {
        (*(g as *mut Agobj_t)).tag.id = gid;
    }
    g = agopen1(g);
    agregister(g, 0 as libc::c_int, g as *mut libc::c_void);
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn agopen1(mut g: *mut Agraph_t) -> *mut Agraph_t {
    let mut par: *mut Agraph_t = 0 as *mut Agraph_t;
    let ref mut fresh9 = (*g).n_seq;
    *fresh9 = agdtopen(g, &mut Ag_subnode_seq_disc, Dttree);
    let ref mut fresh10 = (*g).n_id;
    *fresh10 = agdtopen(g, &mut Ag_subnode_id_disc, Dttree);
    let ref mut fresh11 = (*g).e_seq;
    *fresh11 = agdtopen(
        g,
        if g == agroot(g as *mut libc::c_void) {
            &mut Ag_mainedge_seq_disc
        } else {
            &mut Ag_subedge_seq_disc
        },
        Dttree,
    );
    let ref mut fresh12 = (*g).e_id;
    *fresh12 = agdtopen(
        g,
        if g == agroot(g as *mut libc::c_void) {
            &mut Ag_mainedge_id_disc
        } else {
            &mut Ag_subedge_id_disc
        },
        Dttree,
    );
    let ref mut fresh13 = (*g).g_dict;
    *fresh13 = agdtopen(g, &mut Ag_subgraph_id_disc, Dttree);
    par = agparent(g);
    if !par.is_null() {
        let ref mut fresh14 = (*(g as *mut Agobj_t)).tag;
        (*fresh14).set_seq(agnextseq(par, 0 as libc::c_int) as libc::c_uint);
        (Some(((*((*par).g_dict as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            (*par).g_dict,
            g as *mut libc::c_void,
            0o1 as libc::c_int,
        );
    }
    if par.is_null() || ((*par).desc).has_attrs() as libc::c_int != 0 {
        agraphattr_init(g);
    }
    agmethod_init(g, g as *mut libc::c_void);
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn agclose(mut g: *mut Agraph_t) -> libc::c_int {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut next_subg: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut par: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut next_n: *mut Agnode_t = 0 as *mut Agnode_t;
    par = agparent(g);
    if par.is_null() && ((*(*(*g).clos).disc.mem).close).is_some() {
        agmethod_delete(g, g as *mut libc::c_void);
        agfreeid(g, 0 as libc::c_int, (*(g as *mut Agobj_t)).tag.id);
        ((*(*(*g).clos).disc.mem).close).expect("non-null function pointer")(
            (*(*g).clos).state.mem,
        );
        return 0 as libc::c_int;
    }
    subg = agfstsubg(g);
    while !subg.is_null() {
        next_subg = agnxtsubg(subg);
        agclose(subg);
        subg = next_subg;
    }
    n = agfstnode(g);
    while !n.is_null() {
        next_n = agnxtnode(g, n);
        agdelnode(g, n);
        n = next_n;
    }
    aginternalmapclose(g);
    agmethod_delete(g, g as *mut libc::c_void);
    if dtsize((*g).n_id) == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"dtsize(g->n_id) == 0\0" as *const u8 as *const libc::c_char,
            b"graph.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"int agclose(Agraph_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if agdtclose(g, (*g).n_id) != 0 {
        return -(1 as libc::c_int);
    }
    if dtsize((*g).n_seq) == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"dtsize(g->n_seq) == 0\0" as *const u8 as *const libc::c_char,
            b"graph.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"int agclose(Agraph_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if agdtclose(g, (*g).n_seq) != 0 {
        return -(1 as libc::c_int);
    }
    if dtsize((*g).e_id) == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"dtsize(g->e_id) == 0\0" as *const u8 as *const libc::c_char,
            b"graph.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"int agclose(Agraph_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if agdtclose(g, (*g).e_id) != 0 {
        return -(1 as libc::c_int);
    }
    if dtsize((*g).e_seq) == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"dtsize(g->e_seq) == 0\0" as *const u8 as *const libc::c_char,
            b"graph.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"int agclose(Agraph_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if agdtclose(g, (*g).e_seq) != 0 {
        return -(1 as libc::c_int);
    }
    if dtsize((*g).g_dict) == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"dtsize(g->g_dict) == 0\0" as *const u8 as *const libc::c_char,
            b"graph.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"int agclose(Agraph_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if agdtclose(g, (*g).g_dict) != 0 {
        return -(1 as libc::c_int);
    }
    if ((*g).desc).has_attrs() != 0 {
        if agraphattr_delete(g) != 0 {
            return -(1 as libc::c_int);
        }
    }
    agrecclose(g as *mut Agobj_t);
    agfreeid(g, 0 as libc::c_int, (*(g as *mut Agobj_t)).tag.id);
    if !par.is_null() {
        agdelsubg(par, g);
        agfree(par, g as *mut libc::c_void);
    } else {
        let mut memdisc: *mut Agmemdisc_t = 0 as *mut Agmemdisc_t;
        let mut memclos: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut clos: *mut libc::c_void = 0 as *mut libc::c_void;
        while !((*(*g).clos).cb).is_null() {
            agpopdisc(g, (*(*(*g).clos).cb).f);
        }
        ((*(*(*g).clos).disc.id).close).expect("non-null function pointer")((*(*g).clos).state.id);
        if agstrclose(g) != 0 {
            return -(1 as libc::c_int);
        }
        memdisc = (*(*g).clos).disc.mem;
        memclos = (*(*g).clos).state.mem;
        clos = (*g).clos as *mut libc::c_void;
        ((*memdisc).free).expect("non-null function pointer")(memclos, g as *mut libc::c_void);
        ((*memdisc).free).expect("non-null function pointer")(memclos, clos);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agnextseq(mut g: *mut Agraph_t, mut objtype: libc::c_int) -> uint64_t {
    let ref mut fresh15 = (*(*g).clos).seq[objtype as usize];
    *fresh15 = (*fresh15).wrapping_add(1);
    return *fresh15;
}
#[no_mangle]
pub unsafe extern "C" fn agnnodes(mut g: *mut Agraph_t) -> libc::c_int {
    return dtsize((*g).n_id);
}
#[no_mangle]
pub unsafe extern "C" fn agnedges(mut g: *mut Agraph_t) -> libc::c_int {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut rv: libc::c_int = 0 as libc::c_int;
    n = agfstnode(g);
    while !n.is_null() {
        rv += agdegree(
            g,
            n,
            0 as libc::c_int,
            (0 as libc::c_int == 0) as libc::c_int,
        );
        n = agnxtnode(g, n);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agnsubg(mut g: *mut Agraph_t) -> libc::c_int {
    return dtsize((*g).g_dict);
}
#[no_mangle]
pub unsafe extern "C" fn agisdirected(mut g: *mut Agraph_t) -> libc::c_int {
    return ((*g).desc).directed() as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agisundirected(mut g: *mut Agraph_t) -> libc::c_int {
    return (agisdirected(g) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agisstrict(mut g: *mut Agraph_t) -> libc::c_int {
    return ((*g).desc).strict() as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agissimple(mut g: *mut Agraph_t) -> libc::c_int {
    return (((*g).desc).strict() as libc::c_int != 0 && ((*g).desc).no_loop() as libc::c_int != 0)
        as libc::c_int;
}
unsafe extern "C" fn cnt(mut d: *mut Dict_t, mut set: *mut *mut Dtlink_t) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    dtrestore(d, *set);
    rv = dtsize(d);
    *set = dtextract(d);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agcountuniqedges(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut want_in: libc::c_int,
    mut want_out: libc::c_int,
) -> libc::c_int {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut rv: libc::c_int = 0 as libc::c_int;
    sn = agsubrep(g, n);
    if want_out != 0 {
        rv = cnt((*g).e_seq, &mut (*sn).out_seq);
    }
    if want_in != 0 {
        if want_out == 0 {
            rv += cnt((*g).e_seq, &mut (*sn).in_seq);
        } else {
            e = agfstin(g, n);
            while !e.is_null() {
                if (*e).node != n {
                    rv += 1;
                }
                e = agnxtin(g, e);
            }
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agdegree(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut want_in: libc::c_int,
    mut want_out: libc::c_int,
) -> libc::c_int {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut rv: libc::c_int = 0 as libc::c_int;
    sn = agsubrep(g, n);
    if !sn.is_null() {
        if want_out != 0 {
            rv += cnt((*g).e_seq, &mut (*sn).out_seq);
        }
        if want_in != 0 {
            rv += cnt((*g).e_seq, &mut (*sn).in_seq);
        }
    }
    return rv;
}
unsafe extern "C" fn agraphidcmpf(
    mut d: *mut Dict_t,
    mut arg0: *mut libc::c_void,
    mut arg1: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut v: ptrdiff_t = 0;
    let mut sg0: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut sg1: *mut Agraph_t = 0 as *mut Agraph_t;
    sg0 = arg0 as *mut Agraph_t;
    sg1 = arg1 as *mut Agraph_t;
    v = ((*(sg0 as *mut Agobj_t)).tag.id).wrapping_sub((*(sg1 as *mut Agobj_t)).tag.id)
        as ptrdiff_t;
    return if v == 0 as libc::c_int as libc::c_long {
        0 as libc::c_int
    } else if v < 0 as libc::c_int as libc::c_long {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub static mut Ag_subgraph_id_disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: 32 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: Some(
                agraphidcmpf
                    as unsafe extern "C" fn(
                        *mut Dict_t,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut Dtdisc_t,
                    ) -> libc::c_int,
            ),
            hashf: None,
            memoryf: Some(
                agdictobjmem
                    as unsafe extern "C" fn(
                        *mut Dict_t,
                        *mut libc::c_void,
                        size_t,
                        *mut Dtdisc_t,
                    ) -> *mut libc::c_void,
            ),
            eventf: None,
        };
        init
    }
};
#[no_mangle]
pub static mut Agdirected: Agdesc_t = Agdesc_t {
    directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
    c2rust_padding: [0; 3],
};
#[no_mangle]
pub static mut Agstrictdirected: Agdesc_t = Agdesc_t {
    directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
    c2rust_padding: [0; 3],
};
#[no_mangle]
pub static mut Agundirected: Agdesc_t = Agdesc_t {
    directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
    c2rust_padding: [0; 3],
};
#[no_mangle]
pub static mut Agstrictundirected: Agdesc_t = Agdesc_t {
    directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
    c2rust_padding: [0; 3],
};
#[no_mangle]
pub static mut AgDefaultDisc: Agdisc_t = unsafe {
    {
        let mut init = Agdisc_s {
            mem: &AgMemDisc as *const Agmemdisc_t as *mut Agmemdisc_t,
            id: &AgIdDisc as *const Agiddisc_t as *mut Agiddisc_t,
            io: &AgIoDisc as *const Agiodisc_t as *mut Agiodisc_t,
        };
        init
    }
};
unsafe extern "C" fn run_static_initializers() {
    Agdirected = {
        let mut init = Agdesc_s {
            directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_directed(1 as libc::c_int as libc::c_uint);
        init.set_strict(0);
        init.set_no_loop(0);
        init.set_maingraph(1 as libc::c_int as libc::c_uint);
        init.set_flatlock(0);
        init.set_no_write(0);
        init.set_has_attrs(0);
        init.set_has_cmpnd(0);
        init
    };
    Agstrictdirected = {
        let mut init = Agdesc_s {
            directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_directed(1 as libc::c_int as libc::c_uint);
        init.set_strict(1 as libc::c_int as libc::c_uint);
        init.set_no_loop(0);
        init.set_maingraph(1 as libc::c_int as libc::c_uint);
        init.set_flatlock(0);
        init.set_no_write(0);
        init.set_has_attrs(0);
        init.set_has_cmpnd(0);
        init
    };
    Agundirected = {
        let mut init = Agdesc_s {
            directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_directed(0);
        init.set_strict(0);
        init.set_no_loop(0);
        init.set_maingraph(1 as libc::c_int as libc::c_uint);
        init.set_flatlock(0);
        init.set_no_write(0);
        init.set_has_attrs(0);
        init.set_has_cmpnd(0);
        init
    };
    Agstrictundirected = {
        let mut init = Agdesc_s {
            directed_strict_no_loop_maingraph_flatlock_no_write_has_attrs_has_cmpnd: [0; 1],
            c2rust_padding: [0; 3],
        };
        init.set_directed(0);
        init.set_strict(1 as libc::c_int as libc::c_uint);
        init.set_no_loop(0);
        init.set_maingraph(1 as libc::c_int as libc::c_uint);
        init.set_flatlock(0);
        init.set_no_write(0);
        init.set_has_attrs(0);
        init.set_has_cmpnd(0);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
