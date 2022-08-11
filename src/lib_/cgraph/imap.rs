#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut Dttree: *mut Dtmethod_t;
    fn dtclose(_: *mut Dt_t) -> libc::c_int;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrbind(g: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    static mut Ag_G_global: *mut Agraph_t;
    fn agdtopen(
        g: *mut Agraph_t,
        disc: *mut Dtdisc_t,
        method: *mut Dtmethod_t,
    ) -> *mut Dict_t;
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
pub type IMapEntry_t = IMapEntry_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMapEntry_s {
    pub namedict_link: Dtlink_t,
    pub iddict_link: Dtlink_t,
    pub id: IDTYPE,
    pub str_0: *mut libc::c_char,
}
unsafe extern "C" fn idcmpf(
    mut d: *mut Dict_t,
    mut arg_p0: *mut libc::c_void,
    mut arg_p1: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut p0: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    let mut p1: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    p0 = arg_p0 as *mut IMapEntry_t;
    p1 = arg_p1 as *mut IMapEntry_t;
    if (*p0).id > (*p1).id {
        return 1 as libc::c_int
    } else if (*p0).id < (*p1).id {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn namecmpf(
    mut d: *mut Dict_t,
    mut arg_p0: *mut libc::c_void,
    mut arg_p1: *mut libc::c_void,
    mut disc: *mut Dtdisc_t,
) -> libc::c_int {
    let mut p0: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    let mut p1: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    p0 = arg_p0 as *mut IMapEntry_t;
    p1 = arg_p1 as *mut IMapEntry_t;
    if (*p0).str_0 > (*p1).str_0 {
        return 1 as libc::c_int
    } else if (*p0).str_0 < (*p1).str_0 {
        return -(1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
static mut LookupByName: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0 as libc::c_int,
            size: 0 as libc::c_int,
            link: 0 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: Some(
                namecmpf
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
static mut LookupById: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 0 as libc::c_int,
            size: 0 as libc::c_int,
            link: 16 as libc::c_ulong as libc::c_int,
            makef: None,
            freef: None,
            comparf: Some(
                idcmpf
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
pub unsafe extern "C" fn aginternalmaplookup(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut str: *mut libc::c_char,
    mut result: *mut IDTYPE,
) -> libc::c_int {
    let mut d: *mut Dict_t = 0 as *mut Dict_t;
    let mut sym: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    let mut template: IMapEntry_t = IMapEntry_t {
        namedict_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        iddict_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id: 0,
        str_0: 0 as *mut libc::c_char,
    };
    let mut search_str: *mut libc::c_char = 0 as *mut libc::c_char;
    if objtype == 3 as libc::c_int {
        objtype = 2 as libc::c_int;
    }
    d = (*(*g).clos).lookup_by_name[objtype as usize];
    if !d.is_null() {
        search_str = agstrbind(g, str);
        if !search_str.is_null() {
            template.str_0 = search_str;
            sym = (Some(
                ((*(d as *mut Dt_t)).searchf).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                d,
                &mut template as *mut IMapEntry_t as *mut libc::c_void,
                0o4 as libc::c_int,
            ) as *mut IMapEntry_t;
            if !sym.is_null() {
                *result = (*sym).id;
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aginternalmapinsert(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut str: *mut libc::c_char,
    mut id: IDTYPE,
) {
    let mut ent: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    let mut d_name_to_id: *mut Dict_t = 0 as *mut Dict_t;
    let mut d_id_to_name: *mut Dict_t = 0 as *mut Dict_t;
    ent = agalloc(g, ::std::mem::size_of::<IMapEntry_t>() as libc::c_ulong)
        as *mut IMapEntry_t;
    (*ent).id = id;
    let ref mut fresh0 = (*ent).str_0;
    *fresh0 = agstrdup(g, str);
    if objtype == 3 as libc::c_int {
        objtype = 2 as libc::c_int;
    }
    d_name_to_id = (*(*g).clos).lookup_by_name[objtype as usize];
    if d_name_to_id.is_null() {
        let ref mut fresh1 = (*(*g).clos).lookup_by_name[objtype as usize];
        *fresh1 = agdtopen(g, &mut LookupByName, Dttree);
        d_name_to_id = *fresh1;
    }
    d_id_to_name = (*(*g).clos).lookup_by_id[objtype as usize];
    if d_id_to_name.is_null() {
        let ref mut fresh2 = (*(*g).clos).lookup_by_id[objtype as usize];
        *fresh2 = agdtopen(g, &mut LookupById, Dttree);
        d_id_to_name = *fresh2;
    }
    (Some(((*(d_name_to_id as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(d_name_to_id, ent as *mut libc::c_void, 0o1 as libc::c_int);
    (Some(((*(d_id_to_name as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(d_id_to_name, ent as *mut libc::c_void, 0o1 as libc::c_int);
}
unsafe extern "C" fn find_isym(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut id: IDTYPE,
) -> *mut IMapEntry_t {
    let mut d: *mut Dict_t = 0 as *mut Dict_t;
    let mut isym: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    let mut itemplate: IMapEntry_t = IMapEntry_t {
        namedict_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        iddict_link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        id: 0,
        str_0: 0 as *mut libc::c_char,
    };
    if objtype == 3 as libc::c_int {
        objtype = 2 as libc::c_int;
    }
    d = (*(*g).clos).lookup_by_id[objtype as usize];
    if !d.is_null() {
        itemplate.id = id;
        isym = (Some(((*(d as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            d,
            &mut itemplate as *mut IMapEntry_t as *mut libc::c_void,
            0o4 as libc::c_int,
        ) as *mut IMapEntry_t;
    } else {
        isym = 0 as *mut IMapEntry_t;
    }
    return isym;
}
#[no_mangle]
pub unsafe extern "C" fn aginternalmapprint(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut id: IDTYPE,
) -> *mut libc::c_char {
    let mut isym: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    isym = find_isym(g, objtype, id);
    if !isym.is_null() {
        return (*isym).str_0;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn aginternalmapdelete(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut id: IDTYPE,
) -> libc::c_int {
    let mut isym: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    if objtype == 3 as libc::c_int {
        objtype = 2 as libc::c_int;
    }
    isym = find_isym(g, objtype, id);
    if !isym.is_null() {
        (Some(
            ((*(*((*(*g).clos).lookup_by_name).as_mut_ptr().offset(objtype as isize)
                as *mut Dt_t))
                .searchf)
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            (*(*g).clos).lookup_by_name[objtype as usize],
            isym as *mut libc::c_void,
            0o2 as libc::c_int,
        );
        (Some(
            ((*(*((*(*g).clos).lookup_by_id).as_mut_ptr().offset(objtype as isize)
                as *mut Dt_t))
                .searchf)
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            (*(*g).clos).lookup_by_id[objtype as usize],
            isym as *mut libc::c_void,
            0o2 as libc::c_int,
        );
        agstrfree(g, (*isym).str_0);
        agfree(g, isym as *mut libc::c_void);
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aginternalmapclearlocalnames(mut g: *mut Agraph_t) {
    let mut i: libc::c_int = 0;
    let mut sym: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    let mut nxt: *mut IMapEntry_t = 0 as *mut IMapEntry_t;
    let mut d_name: *mut *mut Dict_t = 0 as *mut *mut Dict_t;
    Ag_G_global = g;
    d_name = ((*(*g).clos).lookup_by_name).as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if !(*d_name.offset(i as isize)).is_null() {
            sym = (Some(
                ((*(*d_name.offset(i as isize) as *mut Dt_t)).searchf)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                *d_name.offset(i as isize),
                0 as *mut libc::c_void,
                0o200 as libc::c_int,
            ) as *mut IMapEntry_t;
            while !sym.is_null() {
                nxt = (Some(
                    ((*(*d_name.offset(i as isize) as *mut Dt_t)).searchf)
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    *d_name.offset(i as isize),
                    sym as *mut libc::c_void,
                    0o10 as libc::c_int,
                ) as *mut IMapEntry_t;
                if *((*sym).str_0).offset(0 as libc::c_int as isize) as libc::c_int
                    == '%' as i32
                {
                    aginternalmapdelete(g, i, (*sym).id);
                }
                sym = nxt;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn closeit(mut d: *mut *mut Dict_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if !(*d.offset(i as isize)).is_null() {
            dtclose(*d.offset(i as isize));
            let ref mut fresh3 = *d.offset(i as isize);
            *fresh3 = 0 as *mut Dict_t;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn aginternalmapclose(mut g: *mut Agraph_t) {
    Ag_G_global = g;
    closeit(((*(*g).clos).lookup_by_name).as_mut_ptr());
    closeit(((*(*g).clos).lookup_by_id).as_mut_ptr());
}
