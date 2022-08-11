#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn agrecord_callback(
        g: *mut Agraph_t,
        obj: *mut Agobj_t,
        kind: cb_t,
        optsym: *mut Agsym_t,
    );
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    fn agclose(g: *mut Agraph_t) -> libc::c_int;
    fn agidnode(g: *mut Agraph_t, id: IDTYPE, createflag: libc::c_int) -> *mut Agnode_t;
    fn agsubedge(
        g: *mut Agraph_t,
        e: *mut Agedge_t,
        createflag: libc::c_int,
    ) -> *mut Agedge_t;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn agparent(g: *mut Agraph_t) -> *mut Agraph_t;
    fn agrelabel_node(n: *mut Agnode_t, newname: *mut libc::c_char) -> libc::c_int;
    fn agdeledge(g: *mut Agraph_t, arg_e: *mut Agedge_t) -> libc::c_int;
    fn agdelnode(g: *mut Agraph_t, arg_n: *mut Agnode_t) -> libc::c_int;
    fn agfreeid(g: *mut Agraph_t, objtype: libc::c_int, id: IDTYPE);
    fn agidsubg(g: *mut Agraph_t, id: IDTYPE, cflag: libc::c_int) -> *mut Agraph_t;
    fn abort() -> !;
    fn agmapnametoid(
        g: *mut Agraph_t,
        objtype: libc::c_int,
        str: *mut libc::c_char,
        result: *mut IDTYPE,
        createflag: libc::c_int,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
pub type FILE = _IO_FILE;
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
pub type cb_t = libc::c_uint;
pub const CB_DELETION: cb_t = 2;
pub const CB_UPDATE: cb_t = 1;
pub const CB_INITIALIZE: cb_t = 0;
#[no_mangle]
pub unsafe extern "C" fn agdelete(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
) -> libc::c_int {
    if ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int == 0 as libc::c_int
        && g != agparent(obj as *mut Agraph_t)
    {
        agerr(AGERR, b"agdelete on wrong graph\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    match ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int {
        1 => return agdelnode(g, obj as *mut Agnode_t),
        3 | 2 => return agdeledge(g, obj as *mut Agedge_t),
        0 => return agclose(obj as *mut Agraph_t),
        _ => {
            agerr(
                AGERR,
                b"agdelete on bad object\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agrename(
    mut obj: *mut Agobj_t,
    mut newname: *mut libc::c_char,
) -> libc::c_int {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut old_id: IDTYPE = 0;
    let mut new_id: IDTYPE = 0;
    match ((*obj).tag).objtype() as libc::c_int {
        0 => {
            old_id = (*obj).tag.id;
            g = agraphof(obj as *mut libc::c_void);
            if agmapnametoid(
                agroot(g as *mut libc::c_void),
                ((*obj).tag).objtype() as libc::c_int,
                newname,
                &mut new_id,
                0 as libc::c_int,
            ) == 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if new_id == old_id {
                return 0 as libc::c_int;
            }
            if agmapnametoid(
                agroot(g as *mut libc::c_void),
                ((*obj).tag).objtype() as libc::c_int,
                newname,
                &mut new_id,
                (0 as libc::c_int == 0) as libc::c_int,
            ) == 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if !(agparent(g)).is_null()
                && !(agidsubg(agparent(g), new_id, 0 as libc::c_int)).is_null()
            {
                return -(1 as libc::c_int);
            }
            agfreeid(g, 0 as libc::c_int, old_id);
            (*(g as *mut Agobj_t)).tag.id = new_id;
        }
        1 => return agrelabel_node(obj as *mut Agnode_t, newname),
        3 | 2 => return -(1 as libc::c_int),
        _ => {
            fprintf(
                stderr,
                b"%s:%d: claimed unreachable code was reached\0" as *const u8
                    as *const libc::c_char,
                b"obj.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int,
            );
            abort();
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agmethod_init(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
) {
    if (*(*g).clos).callbacks_enabled != 0 {
        aginitcb(g, obj, (*(*g).clos).cb);
    } else {
        agrecord_callback(g, obj as *mut Agobj_t, CB_INITIALIZE, 0 as *mut Agsym_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn aginitcb(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
    mut cbstack: *mut Agcbstack_t,
) {
    let mut fn_0: agobjfn_t = None;
    if cbstack.is_null() {
        return;
    }
    aginitcb(g, obj, (*cbstack).prev);
    fn_0 = None;
    match ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int {
        0 => {
            fn_0 = (*(*cbstack).f).graph.ins;
        }
        1 => {
            fn_0 = (*(*cbstack).f).node.ins;
        }
        2 => {
            fn_0 = (*(*cbstack).f).edge.ins;
        }
        _ => {}
    }
    if fn_0.is_some() {
        fn_0
            .expect(
                "non-null function pointer",
            )(g, obj as *mut Agobj_t, (*cbstack).state);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agmethod_upd(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
    mut sym: *mut Agsym_t,
) {
    if (*(*g).clos).callbacks_enabled != 0 {
        agupdcb(g, obj, sym, (*(*g).clos).cb);
    } else {
        agrecord_callback(g, obj as *mut Agobj_t, CB_UPDATE, sym);
    };
}
#[no_mangle]
pub unsafe extern "C" fn agupdcb(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
    mut sym: *mut Agsym_t,
    mut cbstack: *mut Agcbstack_t,
) {
    let mut fn_0: agobjupdfn_t = None;
    if cbstack.is_null() {
        return;
    }
    agupdcb(g, obj, sym, (*cbstack).prev);
    fn_0 = None;
    match ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int {
        0 => {
            fn_0 = (*(*cbstack).f).graph.mod_0;
        }
        1 => {
            fn_0 = (*(*cbstack).f).node.mod_0;
        }
        2 => {
            fn_0 = (*(*cbstack).f).edge.mod_0;
        }
        _ => {}
    }
    if fn_0.is_some() {
        fn_0
            .expect(
                "non-null function pointer",
            )(g, obj as *mut Agobj_t, (*cbstack).state, sym);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agmethod_delete(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
) {
    if (*(*g).clos).callbacks_enabled != 0 {
        agdelcb(g, obj, (*(*g).clos).cb);
    } else {
        agrecord_callback(g, obj as *mut Agobj_t, CB_DELETION, 0 as *mut Agsym_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn agdelcb(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
    mut cbstack: *mut Agcbstack_t,
) {
    let mut fn_0: agobjfn_t = None;
    if cbstack.is_null() {
        return;
    }
    agdelcb(g, obj, (*cbstack).prev);
    fn_0 = None;
    match ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int {
        0 => {
            fn_0 = (*(*cbstack).f).graph.del;
        }
        1 => {
            fn_0 = (*(*cbstack).f).node.del;
        }
        2 => {
            fn_0 = (*(*cbstack).f).edge.del;
        }
        _ => {}
    }
    if fn_0.is_some() {
        fn_0
            .expect(
                "non-null function pointer",
            )(g, obj as *mut Agobj_t, (*cbstack).state);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agroot(mut obj: *mut libc::c_void) -> *mut Agraph_t {
    if obj.is_null() {
        return 0 as *mut Agraph_t;
    }
    match ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int {
        3 | 2 => return (*(*(obj as *mut Agedge_t)).node).root,
        1 => return (*(obj as *mut Agnode_t)).root,
        0 => return (*(obj as *mut Agraph_t)).root,
        _ => {
            agerr(
                AGERR,
                b"agroot of a bad object\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut Agraph_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn agraphof(mut obj: *mut libc::c_void) -> *mut Agraph_t {
    match ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int {
        3 | 2 => return (*(*(obj as *mut Agedge_t)).node).root,
        1 => return (*(obj as *mut Agnode_t)).root,
        0 => return obj as *mut Agraph_t,
        _ => {
            agerr(AGERR, b"agraphof a bad object\0" as *const u8 as *const libc::c_char);
            return 0 as *mut Agraph_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn agpushdisc(
    mut g: *mut Agraph_t,
    mut cbd: *mut Agcbdisc_t,
    mut state: *mut libc::c_void,
) {
    let mut stack_ent: *mut Agcbstack_t = 0 as *mut Agcbstack_t;
    stack_ent = agalloc(g, ::std::mem::size_of::<Agcbstack_t>() as libc::c_ulong)
        as *mut Agcbstack_t;
    let ref mut fresh0 = (*stack_ent).f;
    *fresh0 = cbd;
    let ref mut fresh1 = (*stack_ent).state;
    *fresh1 = state;
    let ref mut fresh2 = (*stack_ent).prev;
    *fresh2 = (*(*g).clos).cb;
    let ref mut fresh3 = (*(*g).clos).cb;
    *fresh3 = stack_ent;
}
#[no_mangle]
pub unsafe extern "C" fn agpopdisc(
    mut g: *mut Agraph_t,
    mut cbd: *mut Agcbdisc_t,
) -> libc::c_int {
    let mut stack_ent: *mut Agcbstack_t = 0 as *mut Agcbstack_t;
    stack_ent = (*(*g).clos).cb;
    if !stack_ent.is_null() {
        if (*stack_ent).f == cbd {
            let ref mut fresh4 = (*(*g).clos).cb;
            *fresh4 = (*stack_ent).prev;
        } else {
            while !stack_ent.is_null() && (*(*stack_ent).prev).f != cbd {
                stack_ent = (*stack_ent).prev;
            }
            if !stack_ent.is_null() && !((*stack_ent).prev).is_null() {
                let ref mut fresh5 = (*stack_ent).prev;
                *fresh5 = (*(*stack_ent).prev).prev;
            }
        }
        if !stack_ent.is_null() {
            agfree(g, stack_ent as *mut libc::c_void);
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn agcontains(
    mut g: *mut Agraph_t,
    mut obj: *mut libc::c_void,
) -> libc::c_int {
    let mut subg: *mut Agraph_t = 0 as *mut Agraph_t;
    if agroot(g as *mut libc::c_void) != agroot(obj) {
        return 0 as libc::c_int;
    }
    match ((*(obj as *mut Agobj_t)).tag).objtype() as libc::c_int {
        0 => {
            subg = obj as *mut Agraph_t;
            loop {
                if subg == g {
                    return 1 as libc::c_int;
                }
                subg = agparent(subg);
                if subg.is_null() {
                    break;
                }
            }
            return 0 as libc::c_int;
        }
        1 => {
            return (agidnode(g, (*(obj as *mut Agobj_t)).tag.id, 0 as libc::c_int)
                != 0 as *mut Agnode_t) as libc::c_int;
        }
        _ => {
            return (agsubedge(g, obj as *mut Agedge_t, 0 as libc::c_int)
                != 0 as *mut Agedge_t) as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn agobjkind(mut arg: *mut libc::c_void) -> libc::c_int {
    return ((*(arg as *mut Agobj_t)).tag).objtype() as libc::c_int;
}
