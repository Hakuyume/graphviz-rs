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
#![feature(register_tool)]
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agdictobjmem(
        dict: *mut Dict_t,
        p: *mut libc::c_void,
        size: size_t,
        disc: *mut Dtdisc_t,
    ) -> *mut libc::c_void;
    fn agdictobjfree(dict: *mut Dict_t, p: *mut libc::c_void, disc: *mut Dtdisc_t);
    fn agdtopen(g: *mut Agraph_t, disc: *mut Dtdisc_t, method: *mut Dtmethod_t) -> *mut Dict_t;
    fn agdtdelete(g: *mut Agraph_t, dict: *mut Dict_t, obj: *mut libc::c_void) -> libc::c_int;
    fn agdtclose(g: *mut Agraph_t, dict: *mut Dict_t) -> libc::c_int;
    static mut Dttree: *mut Dtmethod_t;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct refstr_t {
    pub link: Dtlink_t,
    #[bitfield(name = "refcnt", ty = "uint64_t", bits = "0..=62")]
    #[bitfield(name = "is_html", ty = "uint64_t", bits = "63..=63")]
    pub refcnt_is_html: [u8; 8],
    pub s: *mut libc::c_char,
    pub store: [libc::c_char; 1],
}
static mut Refstrdisc: Dtdisc_t = unsafe {
    {
        let mut init = _dtdisc_s {
            key: 24 as libc::c_ulong as libc::c_int,
            size: -(1 as libc::c_int),
            link: 0 as libc::c_int,
            makef: None,
            freef: Some(
                agdictobjfree
                    as unsafe extern "C" fn(*mut Dict_t, *mut libc::c_void, *mut Dtdisc_t) -> (),
            ),
            comparf: None,
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
static mut Refdict_default: *mut Dict_t = 0 as *const Dict_t as *mut Dict_t;
unsafe extern "C" fn refdict(mut g: *mut Agraph_t) -> *mut Dict_t {
    let mut dictref: *mut *mut Dict_t = 0 as *mut *mut Dict_t;
    if !g.is_null() {
        dictref = &mut (*(*g).clos).strdict;
    } else {
        dictref = &mut Refdict_default;
    }
    if (*dictref).is_null() {
        *dictref = agdtopen(g, &mut Refstrdisc, Dttree);
    }
    return *dictref;
}
#[no_mangle]
pub unsafe extern "C" fn agstrclose(mut g: *mut Agraph_t) -> libc::c_int {
    return agdtclose(g, refdict(g));
}
unsafe extern "C" fn refsymbind(
    mut strdict: *mut Dict_t,
    mut s: *const libc::c_char,
) -> *mut refstr_t {
    let mut key: refstr_t = refstr_t {
        link: Dtlink_t {
            right: 0 as *mut Dtlink_t,
            hl: C2RustUnnamed { _hash: 0 },
        },
        refcnt_is_html: [0; 8],
        s: 0 as *mut libc::c_char,
        store: [0; 1],
    };
    let mut r: *mut refstr_t = 0 as *mut refstr_t;
    key.s = s as *mut libc::c_char;
    r = (Some(((*(strdict as *mut Dt_t)).searchf).expect("non-null function pointer")))
        .expect("non-null function pointer")(
        strdict,
        &mut key as *mut refstr_t as *mut libc::c_void,
        0o4 as libc::c_int,
    ) as *mut refstr_t;
    return r;
}
unsafe extern "C" fn refstrbind(
    mut strdict: *mut Dict_t,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut r: *mut refstr_t = 0 as *mut refstr_t;
    r = refsymbind(strdict, s);
    if !r.is_null() {
        return (*r).s;
    } else {
        return 0 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn agstrbind(
    mut g: *mut Agraph_t,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    return refstrbind(refdict(g), s);
}
unsafe extern "C" fn agstrdup_internal(
    mut g: *mut Agraph_t,
    mut s: *const libc::c_char,
    mut is_html: bool,
) -> *mut libc::c_char {
    let mut r: *mut refstr_t = 0 as *mut refstr_t;
    let mut strdict: *mut Dict_t = 0 as *mut Dict_t;
    let mut sz: size_t = 0;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    strdict = refdict(g);
    r = refsymbind(strdict, s);
    if !r.is_null() {
        (*r).set_refcnt((*r).refcnt() + 1);
    } else {
        sz = (::std::mem::size_of::<refstr_t>() as libc::c_ulong).wrapping_add(strlen(s));
        if !g.is_null() {
            r = agalloc(g, sz) as *mut refstr_t;
        } else {
            r = malloc(sz) as *mut refstr_t;
            if (sz > 0 as libc::c_int as libc::c_ulong && r.is_null()) as libc::c_int
                as libc::c_long
                != 0
            {
                return 0 as *mut libc::c_char;
            }
        }
        (*r).set_refcnt(1 as libc::c_int as uint64_t);
        (*r).set_is_html(is_html as uint64_t);
        strcpy(((*r).store).as_mut_ptr(), s);
        let ref mut fresh0 = (*r).s;
        *fresh0 = ((*r).store).as_mut_ptr();
        (Some(((*(strdict as *mut Dt_t)).searchf).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            strdict,
            r as *mut libc::c_void,
            0o1 as libc::c_int,
        );
    }
    return (*r).s;
}
#[no_mangle]
pub unsafe extern "C" fn agstrdup(
    mut g: *mut Agraph_t,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    return agstrdup_internal(g, s, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn agstrdup_html(
    mut g: *mut Agraph_t,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    return agstrdup_internal(g, s, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn agstrfree(
    mut g: *mut Agraph_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut r: *mut refstr_t = 0 as *mut refstr_t;
    let mut strdict: *mut Dict_t = 0 as *mut Dict_t;
    if s.is_null() {
        return -(1 as libc::c_int);
    }
    strdict = refdict(g);
    r = refsymbind(strdict, s);
    if !r.is_null() && (*r).s == s as *mut libc::c_char {
        (*r).set_refcnt((*r).refcnt() - 1);
        if (*r).refcnt() == 0 as libc::c_int as libc::c_ulong {
            agdtdelete(g, strdict, r as *mut libc::c_void);
        }
    }
    if r.is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aghtmlstr(mut s: *const libc::c_char) -> libc::c_int {
    let mut key: *const refstr_t = 0 as *const refstr_t;
    if s.is_null() {
        return 0 as libc::c_int;
    }
    key = s.offset(-(32 as libc::c_ulong as isize)) as *const refstr_t;
    return (*key).is_html() as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agmarkhtmlstr(mut s: *mut libc::c_char) {
    let mut key: *mut refstr_t = 0 as *mut refstr_t;
    if s.is_null() {
        return;
    }
    key = s.offset(-(32 as libc::c_ulong as isize)) as *mut refstr_t;
    (*key).set_is_html(1 as libc::c_int as uint64_t);
}
