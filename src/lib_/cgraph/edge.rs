#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agapply(
        g: *mut Agraph_t,
        obj: *mut Agobj_t,
        fn_0: agobjfn_t,
        arg: *mut libc::c_void,
        preorder: libc::c_int,
    ) -> libc::c_int;
    fn dtextract(_: *mut Dt_t) -> *mut Dtlink_t;
    fn dtrestore(_: *mut Dt_t, _: *mut Dtlink_t) -> libc::c_int;
    fn agisundirected(g: *mut Agraph_t) -> libc::c_int;
    fn agisstrict(g: *mut Agraph_t) -> libc::c_int;
    fn agsubnode(
        g: *mut Agraph_t,
        n: *mut Agnode_t,
        createflag: libc::c_int,
    ) -> *mut Agnode_t;
    fn agregister(g: *mut Agraph_t, objtype: libc::c_int, obj: *mut libc::c_void);
    fn agmethod_init(g: *mut Agraph_t, obj: *mut libc::c_void);
    fn agedgeattr_init(g: *mut Agraph_t, e: *mut Agedge_t);
    static mut AgDataRecName: *mut libc::c_char;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agnextseq(g: *mut Agraph_t, objtype: libc::c_int) -> uint64_t;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agmapnametoid(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        str: *mut libc::c_char,
        result: *mut IDTYPE,
        createflag: libc::c_int,
    ) -> libc::c_int;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agallocid(g: *mut Agraph_t, objtype: libc::c_int, request: IDTYPE) -> libc::c_int;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    fn agmethod_delete(g: *mut Agraph_t, obj: *mut libc::c_void);
    fn agedgeattr_delete(e: *mut Agedge_t);
    fn agrecclose(obj: *mut Agobj_t);
    fn agfreeid(g: *mut Agraph_t, objtype: libc::c_int, id: IDTYPE);
    fn agdictobjmem(
        dict: *mut Dict_t,
        p: *mut libc::c_void,
        size: size_t,
        disc: *mut Dtdisc_t,
    ) -> *mut libc::c_void;
}
pub type __uint64_t = libc::c_ulong;
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
pub struct Agedgepair_s {
    pub out: Agedge_t,
    pub in_0: Agedge_t,
}
pub type Agedgepair_t = Agedgepair_s;
static mut Tag: Agtag_t = Agtag_t {
    objtype_mtflock_attrwf_seq: [0; 4],
    c2rust_padding: [0; 4],
    id: 0,
};
#[no_mangle]
pub unsafe extern "C" fn agfstout(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> *mut Agedge_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    sn = agsubrep(g, n);
    if !sn.is_null() {
        dtrestore((*g).e_seq, (*sn).out_seq);
        e = (Some(
            ((*((*g).e_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).e_seq, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agedge_t;
        let ref mut fresh0 = (*sn).out_seq;
        *fresh0 = dtextract((*g).e_seq);
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn agnxtout(
    mut g: *mut Agraph_t,
    mut e: *mut Agedge_t,
) -> *mut Agedge_t {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut f: *mut Agedge_t = 0 as *mut Agedge_t;
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    })
        .node;
    sn = agsubrep(g, n);
    if !sn.is_null() {
        dtrestore((*g).e_seq, (*sn).out_seq);
        f = (Some(
            ((*((*g).e_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).e_seq, e as *mut libc::c_void, 0o10 as libc::c_int) as *mut Agedge_t;
        let ref mut fresh1 = (*sn).out_seq;
        *fresh1 = dtextract((*g).e_seq);
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn agfstin(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> *mut Agedge_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    sn = agsubrep(g, n);
    if !sn.is_null() {
        dtrestore((*g).e_seq, (*sn).in_seq);
        e = (Some(
            ((*((*g).e_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).e_seq, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agedge_t;
        let ref mut fresh2 = (*sn).in_seq;
        *fresh2 = dtextract((*g).e_seq);
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn agnxtin(
    mut g: *mut Agraph_t,
    mut e: *mut Agedge_t,
) -> *mut Agedge_t {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut f: *mut Agedge_t = 0 as *mut Agedge_t;
    n = (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    })
        .node;
    sn = agsubrep(g, n);
    if !sn.is_null() {
        dtrestore((*g).e_seq, (*sn).in_seq);
        f = (Some(
            ((*((*g).e_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).e_seq, e as *mut libc::c_void, 0o10 as libc::c_int) as *mut Agedge_t;
        let ref mut fresh3 = (*sn).in_seq;
        *fresh3 = dtextract((*g).e_seq);
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn agfstedge(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> *mut Agedge_t {
    let mut rv: *mut Agedge_t = 0 as *mut Agedge_t;
    rv = agfstout(g, n);
    if rv.is_null() {
        rv = agfstin(g, n);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agnxtedge(
    mut g: *mut Agraph_t,
    mut e: *mut Agedge_t,
    mut n: *mut Agnode_t,
) -> *mut Agedge_t {
    let mut rv: *mut Agedge_t = 0 as *mut Agedge_t;
    if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        rv = agnxtout(g, e);
        if rv.is_null() {
            loop {
                rv = if rv.is_null() { agfstin(g, n) } else { agnxtin(g, rv) };
                if !(!rv.is_null() && (*rv).node == n) {
                    break;
                }
            }
        }
    } else {
        loop {
            rv = agnxtin(g, e);
            e = rv;
            if !(!rv.is_null() && (*rv).node == n) {
                break;
            }
        }
    }
    return rv;
}
unsafe extern "C" fn agfindedge_by_key(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut h: *mut Agnode_t,
    mut key: Agtag_t,
) -> *mut Agedge_t {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut template: Agedge_t = Agedge_t {
        base: Agobj_t {
            tag: Agtag_t {
                objtype_mtflock_attrwf_seq: [0; 4],
                c2rust_padding: [0; 4],
                id: 0,
            },
            data: 0 as *mut Agrec_t,
        },
        id_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        seq_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        node: 0 as *mut Agnode_t,
    };
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    if t.is_null() || h.is_null() {
        return 0 as *mut Agedge_t;
    }
    template.base.tag = key;
    template.node = t;
    sn = agsubrep(g, h);
    if sn.is_null() {
        e = 0 as *mut Agedge_t;
    } else {
        dtrestore((*g).e_id, (*sn).in_id);
        e = (Some(
            ((*((*g).e_id as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            (*g).e_id,
            &mut template as *mut Agedge_t as *mut libc::c_void,
            0o4 as libc::c_int,
        ) as *mut Agedge_t;
        let ref mut fresh4 = (*sn).in_id;
        *fresh4 = dtextract((*g).e_id);
    }
    return e;
}
unsafe extern "C" fn agfindedge_by_id(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut h: *mut Agnode_t,
    mut id: IDTYPE,
) -> *mut Agedge_t {
    let mut tag: Agtag_t = Agtag_t {
        objtype_mtflock_attrwf_seq: [0; 4],
        c2rust_padding: [0; 4],
        id: 0,
    };
    tag = Tag;
    tag.set_objtype(2 as libc::c_int as libc::c_uint);
    tag.id = id;
    return agfindedge_by_key(g, t, h, tag);
}
#[no_mangle]
pub unsafe extern "C" fn agsubrep(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> *mut Agsubnode_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut template: Agsubnode_t = Agsubnode_t {
        seq_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        node: 0 as *mut Agnode_t,
        in_id: 0 as *mut Dtlink_t,
        out_id: 0 as *mut Dtlink_t,
        in_seq: 0 as *mut Dtlink_t,
        out_seq: 0 as *mut Dtlink_t,
    };
    if g == (*n).root {
        sn = &mut (*n).mainsub;
    } else {
        template.node = n;
        sn = (Some(
            ((*((*g).n_id as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            (*g).n_id,
            &mut template as *mut Agsubnode_t as *mut libc::c_void,
            0o4 as libc::c_int,
        ) as *mut Agsubnode_t;
    }
    return sn;
}
unsafe extern "C" fn ins(
    mut d: *mut Dict_t,
    mut set: *mut *mut Dtlink_t,
    mut e: *mut Agedge_t,
) {
    dtrestore(d, *set);
    (Some(((*(d as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(d, e as *mut libc::c_void, 0o1 as libc::c_int);
    *set = dtextract(d);
}
unsafe extern "C" fn del(
    mut d: *mut Dict_t,
    mut set: *mut *mut Dtlink_t,
    mut e: *mut Agedge_t,
) {
    let mut x: *mut libc::c_void = 0 as *mut libc::c_void;
    dtrestore(d, *set);
    x = (Some(((*(d as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(d, e as *mut libc::c_void, 0o2 as libc::c_int);
    if !x.is_null() {} else {
        __assert_fail(
            b"x\0" as *const u8 as *const libc::c_char,
            b"edge.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void del(Dict_t *, Dtlink_t **, Agedge_t *)\0"))
                .as_ptr(),
        );
    }
    *set = dtextract(d);
}
unsafe extern "C" fn installedge(mut g: *mut Agraph_t, mut e: *mut Agedge_t) {
    let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut out: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut in_0: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    out = if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    };
    in_0 = if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        e
    } else {
        e.offset(1 as libc::c_int as isize)
    };
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
    while !g.is_null() {
        if !(agfindedge_by_key(g, t, h, (*(e as *mut Agobj_t)).tag)).is_null() {
            break;
        }
        sn = agsubrep(g, t);
        ins((*g).e_seq, &mut (*sn).out_seq, out);
        ins((*g).e_id, &mut (*sn).out_id, out);
        sn = agsubrep(g, h);
        ins((*g).e_seq, &mut (*sn).in_seq, in_0);
        ins((*g).e_id, &mut (*sn).in_id, in_0);
        g = agparent(g);
    }
}
unsafe extern "C" fn subedge(mut g: *mut Agraph_t, mut e: *mut Agedge_t) {
    installedge(g, e);
}
unsafe extern "C" fn newedge(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut h: *mut Agnode_t,
    mut id: IDTYPE,
) -> *mut Agedge_t {
    let mut e2: *mut Agedgepair_t = 0 as *mut Agedgepair_t;
    let mut in_0: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut out: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut seq: libc::c_int = 0;
    agsubnode(g, t, (0 as libc::c_int == 0) as libc::c_int);
    agsubnode(g, h, (0 as libc::c_int == 0) as libc::c_int);
    e2 = agalloc(g, ::std::mem::size_of::<Agedgepair_t>() as libc::c_ulong)
        as *mut Agedgepair_t;
    in_0 = &mut (*e2).in_0;
    out = &mut (*e2).out;
    seq = agnextseq(g, 2 as libc::c_int) as libc::c_int;
    let ref mut fresh5 = (*(in_0 as *mut Agobj_t)).tag;
    (*fresh5).set_objtype(3 as libc::c_int as libc::c_uint);
    let ref mut fresh6 = (*(out as *mut Agobj_t)).tag;
    (*fresh6).set_objtype(2 as libc::c_int as libc::c_uint);
    let ref mut fresh7 = (*(out as *mut Agobj_t)).tag.id;
    *fresh7 = id;
    (*(in_0 as *mut Agobj_t)).tag.id = *fresh7;
    let ref mut fresh9 = (*(in_0 as *mut Agobj_t)).tag;
    let ref mut fresh8 = (*(out as *mut Agobj_t)).tag;
    (*fresh8).set_seq(seq as libc::c_uint);
    (*fresh9).set_seq((*fresh8).seq());
    let ref mut fresh10 = (*in_0).node;
    *fresh10 = t;
    let ref mut fresh11 = (*out).node;
    *fresh11 = h;
    installedge(g, out);
    if ((*g).desc).has_attrs() != 0 {
        agbindrec(
            out as *mut libc::c_void,
            AgDataRecName,
            ::std::mem::size_of::<Agattr_t>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        );
        agedgeattr_init(g, out);
    }
    agmethod_init(g, out as *mut libc::c_void);
    return out;
}
unsafe extern "C" fn ok_to_make_edge(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut h: *mut Agnode_t,
) -> libc::c_int {
    let mut key: Agtag_t = Agtag_t {
        objtype_mtflock_attrwf_seq: [0; 4],
        c2rust_padding: [0; 4],
        id: 0,
    };
    if agisstrict(g) != 0 {
        key = Tag;
        key.set_objtype(0 as libc::c_int as libc::c_uint);
        if !(agfindedge_by_key(g, t, h, key)).is_null() {
            return 0 as libc::c_int;
        }
    }
    if ((*g).desc).no_loop() as libc::c_int != 0 && t == h {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agidedge(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut h: *mut Agnode_t,
    mut id: IDTYPE,
    mut cflag: libc::c_int,
) -> *mut Agedge_t {
    let mut root: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    e = agfindedge_by_id(g, t, h, id);
    if e.is_null() && agisundirected(g) != 0 {
        e = agfindedge_by_id(g, h, t, id);
    }
    if e.is_null() && cflag != 0 && ok_to_make_edge(g, t, h) != 0 {
        root = agroot(g as *mut libc::c_void);
        if g != root
            && {
                e = agfindedge_by_id(root, t, h, id);
                !e.is_null()
            }
        {
            subedge(g, e);
        } else if agallocid(g, 2 as libc::c_int, id) != 0 {
            e = newedge(g, t, h, id);
        }
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn agedge(
    mut g: *mut Agraph_t,
    mut t: *mut Agnode_t,
    mut h: *mut Agnode_t,
    mut name: *mut libc::c_char,
    mut cflag: libc::c_int,
) -> *mut Agedge_t {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut my_id: IDTYPE = 0;
    let mut have_id: libc::c_int = 0;
    have_id = agmapnametoid(g, 2 as libc::c_int, name, &mut my_id, 0 as libc::c_int);
    if have_id != 0 || name.is_null() && (cflag == 0 || agisstrict(g) != 0) {
        let mut key: Agtag_t = Agtag_t {
            objtype_mtflock_attrwf_seq: [0; 4],
            c2rust_padding: [0; 4],
            id: 0,
        };
        key = Tag;
        if have_id != 0 {
            key.id = my_id;
            key.set_objtype(2 as libc::c_int as libc::c_uint);
        } else {
            key.set_objtype(0 as libc::c_int as libc::c_uint);
            key.id = key.objtype() as IDTYPE;
        }
        e = agfindedge_by_key(g, t, h, key);
        if e.is_null() && agisundirected(g) != 0 {
            e = agfindedge_by_key(g, h, t, key);
        }
        if !e.is_null() {
            return e;
        }
        if cflag != 0 {
            e = agfindedge_by_key(agroot(g as *mut libc::c_void), t, h, key);
            if e.is_null() && agisundirected(g) != 0 {
                e = agfindedge_by_key(agroot(g as *mut libc::c_void), h, t, key);
            }
            if !e.is_null() {
                subedge(g, e);
                return e;
            }
        }
    }
    if cflag != 0 && ok_to_make_edge(g, t, h) != 0
        && agmapnametoid(
            g,
            2 as libc::c_int,
            name,
            &mut my_id,
            (0 as libc::c_int == 0) as libc::c_int,
        ) != 0
    {
        e = newedge(g, t, h, my_id);
        agregister(g, 2 as libc::c_int, e as *mut libc::c_void);
    } else {
        e = 0 as *mut Agedge_t;
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn agdeledgeimage(
    mut g: *mut Agraph_t,
    mut e: *mut Agedge_t,
    mut ignored: *mut libc::c_void,
) {
    let mut in_0: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut out: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
        in_0 = e;
        out = e.offset(-(1 as libc::c_int as isize));
    } else {
        out = e;
        in_0 = e.offset(1 as libc::c_int as isize);
    }
    t = (*in_0).node;
    h = (*out).node;
    sn = agsubrep(g, t);
    del((*g).e_seq, &mut (*sn).out_seq, out);
    del((*g).e_id, &mut (*sn).out_id, out);
    sn = agsubrep(g, h);
    del((*g).e_seq, &mut (*sn).in_seq, in_0);
    del((*g).e_id, &mut (*sn).in_id, in_0);
}
#[no_mangle]
pub unsafe extern "C" fn agdeledge(
    mut g: *mut Agraph_t,
    mut e: *mut Agedge_t,
) -> libc::c_int {
    e = if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
        e
    } else {
        e.offset(-(1 as libc::c_int as isize))
    };
    if (agfindedge_by_key(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
        (*(e as *mut Agobj_t)).tag,
    ))
        .is_null()
    {
        return -(1 as libc::c_int);
    }
    if g == agroot(g as *mut libc::c_void) {
        if ((*g).desc).has_attrs() != 0 {
            agedgeattr_delete(e);
        }
        agmethod_delete(g, e as *mut libc::c_void);
        agrecclose(e as *mut Agobj_t);
        agfreeid(g, 2 as libc::c_int, (*(e as *mut Agobj_t)).tag.id);
    }
    if agapply(
        g,
        e as *mut Agobj_t,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Agraph_t,
                    *mut Agedge_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            agobjfn_t,
        >(
            Some(
                agdeledgeimage
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agedge_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {
        if g == agroot(g as *mut libc::c_void) {
            agfree(g, e as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn agsubedge(
    mut g: *mut Agraph_t,
    mut e: *mut Agedge_t,
    mut cflag: libc::c_int,
) -> *mut Agedge_t {
    let mut t: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut h: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut rv: *mut Agedge_t = 0 as *mut Agedge_t;
    rv = 0 as *mut Agedge_t;
    t = agsubnode(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 3 as libc::c_int {
            e
        } else {
            e.offset(1 as libc::c_int as isize)
        })
            .node,
        cflag,
    );
    h = agsubnode(
        g,
        (*if ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int == 2 as libc::c_int {
            e
        } else {
            e.offset(-(1 as libc::c_int as isize))
        })
            .node,
        cflag,
    );
    if !t.is_null() && !h.is_null() {
        rv = agfindedge_by_key(g, t, h, (*(e as *mut Agobj_t)).tag);
        if cflag != 0 && rv.is_null() {
            installedge(g, e);
            rv = e;
        }
        if !rv.is_null()
            && ((*(rv as *mut Agobj_t)).tag).objtype() as libc::c_int
                != ((*(e as *mut Agobj_t)).tag).objtype() as libc::c_int
        {
            rv = if ((*(rv as *mut Agobj_t)).tag).objtype() as libc::c_int
                == 3 as libc::c_int
            {
                rv.offset(-(1 as libc::c_int as isize))
            } else {
                rv.offset(1 as libc::c_int as isize)
            };
        }
    }
    return rv;
}
unsafe extern "C" fn agedgeidcmpf(
    mut d: *mut Dict_t,
    mut arg_e0: *mut libc::c_void,
    mut arg_e1: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut e0: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut e1: *mut Agedge_t = 0 as *mut Agedge_t;
    e0 = arg_e0 as *mut Agedge_t;
    e1 = arg_e1 as *mut Agedge_t;
    if (*((*e0).node as *mut Agobj_t)).tag.id < (*((*e1).node as *mut Agobj_t)).tag.id {
        return -(1 as libc::c_int);
    }
    if (*((*e0).node as *mut Agobj_t)).tag.id > (*((*e1).node as *mut Agobj_t)).tag.id {
        return 1 as libc::c_int;
    }
    if ((*(e0 as *mut Agobj_t)).tag).objtype() as libc::c_int != 0 as libc::c_int
        && ((*(e1 as *mut Agobj_t)).tag).objtype() as libc::c_int != 0 as libc::c_int
    {
        if (*(e0 as *mut Agobj_t)).tag.id < (*(e1 as *mut Agobj_t)).tag.id {
            return -(1 as libc::c_int);
        }
        if (*(e0 as *mut Agobj_t)).tag.id > (*(e1 as *mut Agobj_t)).tag.id {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn agedgeseqcmpf(
    mut d: *mut Dict_t,
    mut arg_e0: *mut libc::c_void,
    mut arg_e1: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut e0: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut e1: *mut Agedge_t = 0 as *mut Agedge_t;
    e0 = arg_e0 as *mut Agedge_t;
    e1 = arg_e1 as *mut Agedge_t;
    if !arg_e0.is_null() && !arg_e1.is_null() {} else {
        __assert_fail(
            b"arg_e0 && arg_e1\0" as *const u8 as *const libc::c_char,
            b"edge.c\0" as *const u8 as *const libc::c_char,
            411 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"int agedgeseqcmpf(Dict_t *, void *, void *, Dtdisc_t *)\0"))
                .as_ptr(),
        );
    }
    if (*e0).node != (*e1).node {
        if (((*((*e0).node as *mut Agobj_t)).tag).seq() as libc::c_int)
            < ((*((*e1).node as *mut Agobj_t)).tag).seq() as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if ((*((*e0).node as *mut Agobj_t)).tag).seq() as libc::c_int
            > ((*((*e1).node as *mut Agobj_t)).tag).seq() as libc::c_int
        {
            return 1 as libc::c_int;
        }
    } else {
        if (((*(e0 as *mut Agobj_t)).tag).seq() as libc::c_int)
            < ((*(e1 as *mut Agobj_t)).tag).seq() as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if ((*(e0 as *mut Agobj_t)).tag).seq() as libc::c_int
            > ((*(e1 as *mut Agobj_t)).tag).seq() as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut Ag_mainedge_seq_disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: 40 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: Some(
                agedgeseqcmpf
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
pub static mut Ag_subedge_seq_disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: -(1 as libc::c_int),
            makef: None,
            freef: None,
            comparf: Some(
                agedgeseqcmpf
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
pub static mut Ag_mainedge_id_disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: 24 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: Some(
                agedgeidcmpf
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
pub static mut Ag_subedge_id_disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: -(1 as libc::c_int),
            makef: None,
            freef: None,
            comparf: Some(
                agedgeidcmpf
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
