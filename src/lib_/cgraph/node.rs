#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    fn dtrenew(_: *mut Dt_t, _: *mut libc::c_void) -> *mut libc::c_void;
    fn dtsize(_: *mut Dt_t) -> libc::c_int;
    fn agregister(g: *mut Agraph_t, objtype: libc::c_int, obj: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agsubrep(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agsubnode_t;
    fn agmethod_init(g: *mut Agraph_t, obj: *mut libc::c_void);
    fn agnodeattr_init(g: *mut Agraph_t, n: *mut Agnode_t);
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agnextseq(g: *mut Agraph_t, objtype: libc::c_int) -> uint64_t;
    static mut AgDataRecName: *mut libc::c_char;
    fn agbindrec(
        obj: *mut libc::c_void,
        name: *const libc::c_char,
        recsize: libc::c_uint,
        move_to_front: libc::c_int,
    ) -> *mut libc::c_void;
    fn agmapnametoid(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        str: *mut libc::c_char,
        result: *mut IDTYPE,
        createflag: libc::c_int,
    ) -> libc::c_int;
    fn agallocid(g: *mut Agraph_t, objtype: libc::c_int, request: IDTYPE) -> libc::c_int;
    fn agdictobjmem(
        dict: *mut Dict_t,
        p: *mut libc::c_void,
        size: size_t,
        disc: *mut Dtdisc_t,
    ) -> *mut libc::c_void;
    fn agapply(
        g: *mut Agraph_t,
        obj: *mut Agobj_t,
        fn_0: agobjfn_t,
        arg: *mut libc::c_void,
        preorder: libc::c_int,
    ) -> libc::c_int;
    fn agfstedge(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtedge(g: *mut Agraph_t, e: *mut Agedge_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agfreeid(g: *mut Agraph_t, objtype: libc::c_int, id: IDTYPE);
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    fn agdeledgeimage(g: *mut Agraph_t, edge: *mut Agedge_t, ignored: *mut libc::c_void);
    fn agrecclose(obj: *mut Agobj_t);
    fn agmethod_delete(g: *mut Agraph_t, obj: *mut libc::c_void);
    fn agnodeattr_delete(n: *mut Agnode_t);
    fn agdeledge(g: *mut Agraph_t, arg_e: *mut Agedge_t) -> libc::c_int;
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
pub struct _dthold_s {
    pub hdr: Dtlink_t,
    pub obj: *mut libc::c_void,
}
pub type Dthold_t = _dthold_s;
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
pub struct Agattr_s {
    pub h: Agrec_t,
    pub dict: *mut Dict_t,
    pub str_0: *mut *mut libc::c_char,
}
pub type Agattr_t = Agattr_s;
#[no_mangle]
pub unsafe extern "C" fn agfindnode_by_id(
    mut g: *mut Agraph_t,
    mut id: IDTYPE,
) -> *mut Agnode_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    static mut template: Agsubnode_t = Agsubnode_t {
        seq_link: Dtlink_t {
            right: 0 as *const Dtlink_t as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id_link: Dtlink_t {
            right: 0 as *const Dtlink_t as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        node: 0 as *const Agnode_t as *mut Agnode_t,
        in_id: 0 as *const Dtlink_t as *mut Dtlink_t,
        out_id: 0 as *const Dtlink_t as *mut Dtlink_t,
        in_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
        out_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
    };
    static mut dummy: Agnode_t = Agnode_t {
        base: Agobj_t {
            tag: Agtag_t {
                objtype_mtflock_attrwf_seq: [0; 4],
                c2rust_padding: [0; 4],
                id: 0,
            },
            data: 0 as *const Agrec_t as *mut Agrec_t,
        },
        root: 0 as *const Agraph_t as *mut Agraph_t,
        mainsub: Agsubnode_t {
            seq_link: Dtlink_t {
                right: 0 as *const Dtlink_t as *mut Dtlink_t,
                hl: C2RustUnnamed { _hash: 0 },
            },
            id_link: Dtlink_t {
                right: 0 as *const Dtlink_t as *mut Dtlink_t,
                hl: C2RustUnnamed { _hash: 0 },
            },
            node: 0 as *const Agnode_t as *mut Agnode_t,
            in_id: 0 as *const Dtlink_t as *mut Dtlink_t,
            out_id: 0 as *const Dtlink_t as *mut Dtlink_t,
            in_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
            out_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
        },
    };
    dummy.base.tag.id = id;
    template.node = &mut dummy;
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
    return if !sn.is_null() { (*sn).node } else { 0 as *mut Agnode_t };
}
unsafe extern "C" fn agfindnode_by_name(
    mut g: *mut Agraph_t,
    mut name: *mut libc::c_char,
) -> *mut Agnode_t {
    let mut id: IDTYPE = 0;
    if agmapnametoid(g, 1 as libc::c_int, name, &mut id, 0 as libc::c_int) != 0 {
        return agfindnode_by_id(g, id)
    } else {
        return 0 as *mut Agnode_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn agfstnode(mut g: *mut Agraph_t) -> *mut Agnode_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    sn = (Some(
        ((*((*g).n_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )((*g).n_seq, 0 as *mut libc::c_void, 0o200 as libc::c_int) as *mut Agsubnode_t;
    return if !sn.is_null() { (*sn).node } else { 0 as *mut Agnode_t };
}
#[no_mangle]
pub unsafe extern "C" fn agnxtnode(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> *mut Agnode_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    sn = agsubrep(g, n);
    if !sn.is_null() {
        sn = (Some(
            ((*((*g).n_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).n_seq, sn as *mut libc::c_void, 0o10 as libc::c_int)
            as *mut Agsubnode_t;
    }
    return if !sn.is_null() { (*sn).node } else { 0 as *mut Agnode_t };
}
#[no_mangle]
pub unsafe extern "C" fn aglstnode(mut g: *mut Agraph_t) -> *mut Agnode_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    sn = (Some(
        ((*((*g).n_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )((*g).n_seq, 0 as *mut libc::c_void, 0o400 as libc::c_int) as *mut Agsubnode_t;
    return if !sn.is_null() { (*sn).node } else { 0 as *mut Agnode_t };
}
#[no_mangle]
pub unsafe extern "C" fn agprvnode(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> *mut Agnode_t {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    sn = agsubrep(g, n);
    if !sn.is_null() {
        sn = (Some(
            ((*((*g).n_seq as *mut Dt_t)).searchf).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).n_seq, sn as *mut libc::c_void, 0o20 as libc::c_int)
            as *mut Agsubnode_t;
    }
    return if !sn.is_null() { (*sn).node } else { 0 as *mut Agnode_t };
}
unsafe extern "C" fn newnode(
    mut g: *mut Agraph_t,
    mut id: IDTYPE,
    mut seq: uint64_t,
) -> *mut Agnode_t {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agalloc(g, ::std::mem::size_of::<Agnode_t>() as libc::c_ulong) as *mut Agnode_t;
    let ref mut fresh0 = (*(n as *mut Agobj_t)).tag;
    (*fresh0).set_objtype(1 as libc::c_int as libc::c_uint);
    (*(n as *mut Agobj_t)).tag.id = id;
    let ref mut fresh1 = (*(n as *mut Agobj_t)).tag;
    (*fresh1).set_seq(seq as libc::c_uint);
    let ref mut fresh2 = (*n).root;
    *fresh2 = agroot(g as *mut libc::c_void);
    if ((*agroot(g as *mut libc::c_void)).desc).has_attrs() != 0 {
        agbindrec(
            n as *mut libc::c_void,
            AgDataRecName,
            ::std::mem::size_of::<Agattr_t>() as libc::c_ulong as libc::c_uint,
            0 as libc::c_int,
        );
    }
    return n;
}
unsafe extern "C" fn installnode(mut g: *mut Agraph_t, mut n: *mut Agnode_t) {
    let mut sn: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut osize: libc::c_int = 0;
    if dtsize((*g).n_id) == dtsize((*g).n_seq) {} else {
        __assert_fail(
            b"dtsize(g->n_id) == dtsize(g->n_seq)\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void installnode(Agraph_t *, Agnode_t *)\0"))
                .as_ptr(),
        );
    }
    osize = dtsize((*g).n_id);
    if g == agroot(g as *mut libc::c_void) {
        sn = &mut (*n).mainsub;
    } else {
        sn = agalloc(g, ::std::mem::size_of::<Agsubnode_t>() as libc::c_ulong)
            as *mut Agsubnode_t;
    }
    let ref mut fresh3 = (*sn).node;
    *fresh3 = n;
    (Some(((*((*g).n_id as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).n_id, sn as *mut libc::c_void, 0o1 as libc::c_int);
    (Some(((*((*g).n_seq as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).n_seq, sn as *mut libc::c_void, 0o1 as libc::c_int);
    if dtsize((*g).n_id) == dtsize((*g).n_seq) {} else {
        __assert_fail(
            b"dtsize(g->n_id) == dtsize(g->n_seq)\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void installnode(Agraph_t *, Agnode_t *)\0"))
                .as_ptr(),
        );
    }
    if dtsize((*g).n_id) == osize + 1 as libc::c_int {} else {
        __assert_fail(
            b"dtsize(g->n_id) == osize + 1\0" as *const u8 as *const libc::c_char,
            b"node.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void installnode(Agraph_t *, Agnode_t *)\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn installnodetoroot(mut g: *mut Agraph_t, mut n: *mut Agnode_t) {
    let mut par: *mut Agraph_t = 0 as *mut Agraph_t;
    installnode(g, n);
    par = agparent(g);
    if !par.is_null() {
        installnodetoroot(par, n);
    }
}
unsafe extern "C" fn initnode(mut g: *mut Agraph_t, mut n: *mut Agnode_t) {
    if ((*agroot(g as *mut libc::c_void)).desc).has_attrs() != 0 {
        agnodeattr_init(g, n);
    }
    agmethod_init(g, n as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn agidnode(
    mut g: *mut Agraph_t,
    mut id: IDTYPE,
    mut cflag: libc::c_int,
) -> *mut Agnode_t {
    let mut root: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    n = agfindnode_by_id(g, id);
    if n.is_null() && cflag != 0 {
        root = agroot(g as *mut libc::c_void);
        if g != root
            && {
                n = agfindnode_by_id(root, id);
                !n.is_null()
            }
        {
            agsubnode(g, n, (0 as libc::c_int == 0) as libc::c_int);
        } else if agallocid(g, 1 as libc::c_int, id) != 0 {
            n = newnode(g, id, agnextseq(g, 1 as libc::c_int));
            installnodetoroot(g, n);
            initnode(g, n);
        } else {
            n = 0 as *mut Agnode_t;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn agnode(
    mut g: *mut Agraph_t,
    mut name: *mut libc::c_char,
    mut cflag: libc::c_int,
) -> *mut Agnode_t {
    let mut root: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut id: IDTYPE = 0;
    root = agroot(g as *mut libc::c_void);
    if agmapnametoid(g, 1 as libc::c_int, name, &mut id, 0 as libc::c_int) != 0 {
        n = agfindnode_by_id(g, id);
        if !n.is_null() {
            return n;
        }
        if cflag != 0 && g != root
            && {
                n = agfindnode_by_id(root, id);
                !n.is_null()
            }
        {
            return agsubnode(g, n, (0 as libc::c_int == 0) as libc::c_int);
        }
    }
    if cflag != 0
        && agmapnametoid(
            g,
            1 as libc::c_int,
            name,
            &mut id,
            (0 as libc::c_int == 0) as libc::c_int,
        ) != 0
    {
        n = newnode(g, id, agnextseq(g, 1 as libc::c_int));
        installnodetoroot(g, n);
        initnode(g, n);
        if !(agsubrep(g, n)).is_null() {} else {
            __assert_fail(
                b"agsubrep(g,n)\0" as *const u8 as *const libc::c_char,
                b"node.c\0" as *const u8 as *const libc::c_char,
                164 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"Agnode_t *agnode(Agraph_t *, char *, int)\0"))
                    .as_ptr(),
            );
        }
        agregister(g, 1 as libc::c_int, n as *mut libc::c_void);
        return n;
    }
    return 0 as *mut Agnode_t;
}
#[no_mangle]
pub unsafe extern "C" fn agdelnodeimage(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut ignored: *mut libc::c_void,
) {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut f: *mut Agedge_t = 0 as *mut Agedge_t;
    static mut template: Agsubnode_t = Agsubnode_t {
        seq_link: Dtlink_t {
            right: 0 as *const Dtlink_t as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id_link: Dtlink_t {
            right: 0 as *const Dtlink_t as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        node: 0 as *const Agnode_t as *mut Agnode_t,
        in_id: 0 as *const Dtlink_t as *mut Dtlink_t,
        out_id: 0 as *const Dtlink_t as *mut Dtlink_t,
        in_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
        out_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
    };
    template.node = n;
    e = agfstedge(g, n);
    while !e.is_null() {
        f = agnxtedge(g, e, n);
        agdeledgeimage(g, e, 0 as *mut libc::c_void);
        e = f;
    }
    (Some(((*((*g).n_id as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*g).n_id,
        &mut template as *mut Agsubnode_t as *mut libc::c_void,
        0o2 as libc::c_int,
    );
    (Some(((*((*g).n_seq as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*g).n_seq,
        &mut template as *mut Agsubnode_t as *mut libc::c_void,
        0o2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn agdelnode(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
) -> libc::c_int {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut f: *mut Agedge_t = 0 as *mut Agedge_t;
    if (agfindnode_by_id(g, (*(n as *mut Agobj_t)).tag.id)).is_null() {
        return -(1 as libc::c_int);
    }
    if g == agroot(g as *mut libc::c_void) {
        e = agfstedge(g, n);
        while !e.is_null() {
            f = agnxtedge(g, e, n);
            agdeledge(g, e);
            e = f;
        }
        if ((*g).desc).has_attrs() != 0 {
            agnodeattr_delete(n);
        }
        agmethod_delete(g, n as *mut libc::c_void);
        agrecclose(n as *mut Agobj_t);
        agfreeid(g, 1 as libc::c_int, (*(n as *mut Agobj_t)).tag.id);
    }
    if agapply(
        g,
        n as *mut Agobj_t,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Agraph_t,
                    *mut Agnode_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            agobjfn_t,
        >(
            Some(
                agdelnodeimage
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agnode_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {
        if g == agroot(g as *mut libc::c_void) {
            agfree(g, n as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn dict_relabel(mut n: *mut Agnode_t, mut arg: *mut libc::c_void) {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut new_id: uint64_t = 0;
    g = agraphof(n as *mut libc::c_void);
    new_id = *(arg as *mut uint64_t);
    (Some(((*((*g).n_id as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).n_id, n as *mut libc::c_void, 0o2 as libc::c_int);
    (*(n as *mut Agobj_t)).tag.id = new_id;
    (Some(((*((*g).n_id as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).n_id, n as *mut libc::c_void, 0o1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn agrelabel_node(
    mut n: *mut Agnode_t,
    mut newname: *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut new_id: IDTYPE = 0;
    g = agroot(agraphof(n as *mut libc::c_void) as *mut libc::c_void);
    if !(agfindnode_by_name(g, newname)).is_null() {
        return -(1 as libc::c_int);
    }
    if agmapnametoid(
        g,
        1 as libc::c_int,
        newname,
        &mut new_id,
        (0 as libc::c_int == 0) as libc::c_int,
    ) != 0
    {
        if (agfindnode_by_id(agroot(g as *mut libc::c_void), new_id)).is_null() {
            agfreeid(g, 1 as libc::c_int, (*(n as *mut Agobj_t)).tag.id);
            agapply(
                g,
                n as *mut Agobj_t,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut Agnode_t, *mut libc::c_void) -> (),
                    >,
                    agobjfn_t,
                >(
                    Some(
                        dict_relabel
                            as unsafe extern "C" fn(
                                *mut Agnode_t,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                ),
                &mut new_id as *mut IDTYPE as *mut libc::c_void,
                0 as libc::c_int,
            );
            return 0 as libc::c_int;
        } else {
            agfreeid(g, 1 as libc::c_int, new_id);
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn agsubnode(
    mut g: *mut Agraph_t,
    mut n0: *mut Agnode_t,
    mut cflag: libc::c_int,
) -> *mut Agnode_t {
    let mut par: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    if agroot(g as *mut libc::c_void) != (*n0).root {
        return 0 as *mut Agnode_t;
    }
    n = agfindnode_by_id(g, (*(n0 as *mut Agobj_t)).tag.id);
    if n.is_null() && cflag != 0 {
        par = agparent(g);
        if !par.is_null() {
            n = agsubnode(par, n0, cflag);
            installnode(g, n);
        }
    }
    return n;
}
unsafe extern "C" fn agsubnodeidcmpf(
    mut d: *mut Dict_t,
    mut arg0: *mut libc::c_void,
    mut arg1: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut sn0: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut sn1: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    sn0 = arg0 as *mut Agsubnode_t;
    sn1 = arg1 as *mut Agsubnode_t;
    if (*((*sn0).node as *mut Agobj_t)).tag.id < (*((*sn1).node as *mut Agobj_t)).tag.id
    {
        return -(1 as libc::c_int);
    }
    if (*((*sn0).node as *mut Agobj_t)).tag.id > (*((*sn1).node as *mut Agobj_t)).tag.id
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn agsubnodeseqcmpf(
    mut d: *mut Dict_t,
    mut arg0: *mut libc::c_void,
    mut arg1: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut sn0: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    let mut sn1: *mut Agsubnode_t = 0 as *mut Agsubnode_t;
    sn0 = arg0 as *mut Agsubnode_t;
    sn1 = arg1 as *mut Agsubnode_t;
    if (((*((*sn0).node as *mut Agobj_t)).tag).seq() as libc::c_int)
        < ((*((*sn1).node as *mut Agobj_t)).tag).seq() as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if ((*((*sn0).node as *mut Agobj_t)).tag).seq() as libc::c_int
        > ((*((*sn1).node as *mut Agobj_t)).tag).seq() as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_subnode(
    mut d: *mut Dt_t,
    mut sn: *mut Agsubnode_t,
    mut disc: *mut Dtdisc_t,
) {
    if !(sn == &mut (*(*sn).node).mainsub as *mut Agsubnode_t) {
        agfree((*(*sn).node).root, sn as *mut libc::c_void);
    }
}
#[no_mangle]
pub static mut Ag_subnode_id_disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: 16 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: Some(
                agsubnodeidcmpf
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
pub static mut Ag_subnode_seq_disc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0,
            size: 0,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Dt_t,
                        *mut Agsubnode_t,
                        *mut Dtdisc_t,
                    ) -> (),
                >,
                Dtfree_f,
            >(
                Some(
                    free_subnode
                        as unsafe extern "C" fn(
                            *mut Dt_t,
                            *mut Agsubnode_t,
                            *mut Dtdisc_t,
                        ) -> (),
                ),
            ),
            comparf: Some(
                agsubnodeseqcmpf
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
unsafe extern "C" fn agnodesetfinger(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut ignored: *mut libc::c_void,
) {
    static mut template: Agsubnode_t = Agsubnode_t {
        seq_link: Dtlink_t {
            right: 0 as *const Dtlink_t as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id_link: Dtlink_t {
            right: 0 as *const Dtlink_t as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        node: 0 as *const Agnode_t as *mut Agnode_t,
        in_id: 0 as *const Dtlink_t as *mut Dtlink_t,
        out_id: 0 as *const Dtlink_t as *mut Dtlink_t,
        in_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
        out_seq: 0 as *const Dtlink_t as *mut Dtlink_t,
    };
    template.node = n;
    (Some(((*((*g).n_seq as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*g).n_seq,
        &mut template as *mut Agsubnode_t as *mut libc::c_void,
        0o4 as libc::c_int,
    );
}
unsafe extern "C" fn agnoderenew(
    mut g: *mut Agraph_t,
    mut n: *mut Agnode_t,
    mut ignored: *mut libc::c_void,
) {
    dtrenew(
        (*g).n_seq,
        if !((*(*((*g).n_seq as *mut Dt_t)).data).here).is_null() {
            if (*(*((*g).n_seq as *mut Dt_t)).disc).link < 0 as libc::c_int {
                (*((*(*((*g).n_seq as *mut Dt_t)).data).here as *mut Dthold_t)).obj
            } else {
                ((*(*((*g).n_seq as *mut Dt_t)).data).here as *mut libc::c_char)
                    .offset(-((*(*((*g).n_seq as *mut Dt_t)).disc).link as isize))
                    as *mut libc::c_void
            }
        } else {
            0 as *mut libc::c_void
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn agnodebefore(
    mut fst: *mut Agnode_t,
    mut snd: *mut Agnode_t,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut nxt: *mut Agnode_t = 0 as *mut Agnode_t;
    g = agroot(fst as *mut libc::c_void);
    if ((*(fst as *mut Agobj_t)).tag).seq() as libc::c_int
        > ((*(snd as *mut Agobj_t)).tag).seq() as libc::c_int
    {
        return 0 as libc::c_int;
    }
    n = snd;
    if agapply(
        g,
        n as *mut Agobj_t,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Agraph_t,
                    *mut Agnode_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            agobjfn_t,
        >(
            Some(
                agnodesetfinger
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agnode_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        n as *mut libc::c_void,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    let ref mut fresh4 = (*(snd as *mut Agobj_t)).tag;
    (*fresh4)
        .set_seq(
            ((*(*g).clos).seq[1 as libc::c_int as usize])
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_uint,
        );
    if agapply(
        g,
        n as *mut Agobj_t,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Agraph_t,
                    *mut Agnode_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            agobjfn_t,
        >(
            Some(
                agnoderenew
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agnode_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        n as *mut libc::c_void,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    n = agprvnode(g, snd);
    loop {
        nxt = agprvnode(g, n);
        if agapply(
            g,
            n as *mut Agobj_t,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agnode_t,
                        *mut libc::c_void,
                    ) -> (),
                >,
                agobjfn_t,
            >(
                Some(
                    agnodesetfinger
                        as unsafe extern "C" fn(
                            *mut Agraph_t,
                            *mut Agnode_t,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            n as *mut libc::c_void,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        let ref mut fresh5 = (*(n as *mut Agobj_t)).tag;
        (*fresh5)
            .set_seq(
                (((*(n as *mut Agobj_t)).tag).seq() as libc::c_int + 1 as libc::c_int)
                    as libc::c_uint,
            );
        if agapply(
            g,
            n as *mut Agobj_t,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agnode_t,
                        *mut libc::c_void,
                    ) -> (),
                >,
                agobjfn_t,
            >(
                Some(
                    agnoderenew
                        as unsafe extern "C" fn(
                            *mut Agraph_t,
                            *mut Agnode_t,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            n as *mut libc::c_void,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if n == fst {
            break;
        }
        n = nxt;
        if n.is_null() {
            break;
        }
    }
    if agapply(
        g,
        snd as *mut Agobj_t,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Agraph_t,
                    *mut Agnode_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            agobjfn_t,
        >(
            Some(
                agnodesetfinger
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agnode_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        n as *mut libc::c_void,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    let ref mut fresh6 = (*(snd as *mut Agobj_t)).tag;
    (*fresh6)
        .set_seq(
            (((*(fst as *mut Agobj_t)).tag).seq() as libc::c_int - 1 as libc::c_int)
                as libc::c_uint,
        );
    if agapply(
        g,
        snd as *mut Agobj_t,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Agraph_t,
                    *mut Agnode_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            agobjfn_t,
        >(
            Some(
                agnoderenew
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agnode_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
        ),
        snd as *mut libc::c_void,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
