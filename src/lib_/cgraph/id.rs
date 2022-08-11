#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agstrbind(g: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn aginternalmapprint(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        id: IDTYPE,
    ) -> *mut libc::c_char;
    fn aginternalmapinsert(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        str: *mut libc::c_char,
        result: IDTYPE,
    );
    fn aginternalmaplookup(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        str: *mut libc::c_char,
        result: *mut IDTYPE,
    ) -> libc::c_int;
    fn aginternalmapdelete(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        id: IDTYPE,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
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
unsafe extern "C" fn idopen(
    mut g: *mut Agraph_t,
    mut disc: *mut Agdisc_t,
) -> *mut libc::c_void {
    return g as *mut libc::c_void;
}
unsafe extern "C" fn idmap(
    mut state: *mut libc::c_void,
    mut objtype: libc::c_int,
    mut str: *mut libc::c_char,
    mut id: *mut IDTYPE,
    mut createflag: libc::c_int,
) -> libc::c_long {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut ctr: IDTYPE = 1 as libc::c_int as IDTYPE;
    if !str.is_null() {
        let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
        g = state as *mut Agraph_t;
        if createflag != 0 {
            s = agstrdup(g, str);
        } else {
            s = agstrbind(g, str);
        }
        *id = s as uintptr_t;
    } else {
        *id = ctr;
        ctr = (ctr as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as IDTYPE as IDTYPE;
    }
    return (0 as libc::c_int == 0) as libc::c_int as libc::c_long;
}
unsafe extern "C" fn idalloc(
    mut state: *mut libc::c_void,
    mut objtype: libc::c_int,
    mut request: IDTYPE,
) -> libc::c_long {
    return 0 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn idfree(
    mut state: *mut libc::c_void,
    mut objtype: libc::c_int,
    mut id: IDTYPE,
) {
    if id.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        agstrfree(state as *mut Agraph_t, id as *mut libc::c_char);
    }
}
unsafe extern "C" fn idprint(
    mut state: *mut libc::c_void,
    mut objtype: libc::c_int,
    mut id: IDTYPE,
) -> *mut libc::c_char {
    if id.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        return id as *mut libc::c_char
    } else {
        return 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn idclose(mut state: *mut libc::c_void) {}
unsafe extern "C" fn idregister(
    mut state: *mut libc::c_void,
    mut objtype: libc::c_int,
    mut obj: *mut libc::c_void,
) {}
#[no_mangle]
pub static mut AgIdDisc: Agiddisc_t = unsafe {
    {
        let mut init = Agiddisc_s {
            open: Some(
                idopen
                    as unsafe extern "C" fn(
                        *mut Agraph_t,
                        *mut Agdisc_t,
                    ) -> *mut libc::c_void,
            ),
            map: Some(
                idmap
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        *mut libc::c_char,
                        *mut IDTYPE,
                        libc::c_int,
                    ) -> libc::c_long,
            ),
            alloc: Some(
                idalloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        IDTYPE,
                    ) -> libc::c_long,
            ),
            free: Some(
                idfree
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int, IDTYPE) -> (),
            ),
            print: Some(
                idprint
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        IDTYPE,
                    ) -> *mut libc::c_char,
            ),
            close: Some(idclose as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            idregister: Some(
                idregister
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn agmapnametoid(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut str: *mut libc::c_char,
    mut result: *mut IDTYPE,
    mut createflag: libc::c_int,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    if !str.is_null()
        && *str.offset(0 as libc::c_int as isize) as libc::c_int != '%' as i32
    {
        rv = ((*(*(*g).clos).disc.id).map)
            .expect(
                "non-null function pointer",
            )((*(*g).clos).state.id, objtype, str, result, createflag) as libc::c_int;
        if rv != 0 {
            return rv;
        }
    }
    if !str.is_null() {
        rv = aginternalmaplookup(g, objtype, str, result);
        if rv != 0 {
            return rv;
        }
    } else {
        rv = 0 as libc::c_int;
    }
    if createflag != 0 {
        rv = ((*(*(*g).clos).disc.id).map)
            .expect(
                "non-null function pointer",
            )((*(*g).clos).state.id, objtype, 0 as *mut libc::c_char, result, createflag)
            as libc::c_int;
        if rv != 0 && !str.is_null() {
            aginternalmapinsert(g, objtype, str, *result);
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agallocid(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut request: IDTYPE,
) -> libc::c_int {
    return ((*(*(*g).clos).disc.id).alloc)
        .expect("non-null function pointer")((*(*g).clos).state.id, objtype, request)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agfreeid(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut id: IDTYPE,
) {
    aginternalmapdelete(g, objtype, id);
    ((*(*(*g).clos).disc.id).free)
        .expect("non-null function pointer")((*(*g).clos).state.id, objtype, id);
}
#[no_mangle]
pub unsafe extern "C" fn agnameof(mut obj: *mut libc::c_void) -> *mut libc::c_char {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    g = agraphof(obj);
    rv = aginternalmapprint(
        g,
        ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int,
        (*(obj as *mut Agobj_t)).tag.id,
    );
    if !rv.is_null() {
        return rv;
    }
    if ((*(*(*g).clos).disc.id).print).is_some() {
        rv = ((*(*(*g).clos).disc.id).print)
            .expect(
                "non-null function pointer",
            )(
            (*(*g).clos).state.id,
            ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int,
            (*(obj as *mut Agobj_t)).tag.id,
        );
        if !rv.is_null() {
            return rv;
        }
    }
    if ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int != 2 as libc::c_int {
        static mut buf: [libc::c_char; 32] = [0; 32];
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%c%lu\0" as *const u8 as *const libc::c_char,
            '%' as i32,
            (*(obj as *mut Agobj_t)).tag.id,
        );
        rv = buf.as_mut_ptr();
    } else {
        rv = 0 as *mut libc::c_char;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn agregister(
    mut g: *mut Agraph_t,
    mut objtype: libc::c_int,
    mut obj: *mut libc::c_void,
) {
    ((*(*(*g).clos).disc.id).idregister)
        .expect("non-null function pointer")((*(*g).clos).state.id, objtype, obj);
}
