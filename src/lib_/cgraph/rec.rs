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
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn agfstnode(g: *mut Agraph_t) -> *mut Agnode_t;
    fn agnxtnode(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agnode_t;
    fn agfstout(g: *mut Agraph_t, n: *mut Agnode_t) -> *mut Agedge_t;
    fn agnxtout(g: *mut Agraph_t, e: *mut Agedge_t) -> *mut Agedge_t;
    fn agraphof(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agroot(obj: *mut libc::c_void) -> *mut Agraph_t;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrfree(_: *mut Agraph_t, _: *const libc::c_char) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn agalloc(g: *mut Agraph_t, size: size_t) -> *mut libc::c_void;
    fn agfree(g: *mut Agraph_t, ptr: *mut libc::c_void);
    fn abort() -> !;
    fn agapply(
        g: *mut Agraph_t,
        obj: *mut Agobj_t,
        fn_0: agobjfn_t,
        arg: *mut libc::c_void,
        preorder: libc::c_int,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agnxtsubg(subg: *mut Agraph_t) -> *mut Agraph_t;
    fn agfstsubg(g: *mut Agraph_t) -> *mut Agraph_t;
    fn abs(_: libc::c_int) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn streq(mut a: *const libc::c_char, mut b: *const libc::c_char) -> bool {
    return strcmp(a, b) == 0 as libc::c_int;
}
unsafe extern "C" fn set_data(mut obj: *mut Agobj_t, mut data: *mut Agrec_t, mut mtflock: bool) {
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let ref mut fresh0 = (*obj).data;
    *fresh0 = data;
    let ref mut fresh1 = (*obj).tag;
    (*fresh1).set_mtflock(mtflock as libc::c_uint);
    if ((*obj).tag).objtype() as libc::c_int == 3 as libc::c_int
        || ((*obj).tag).objtype() as libc::c_int == 2 as libc::c_int
    {
        e = if ((*(obj as *mut Agedge_t as *mut Agobj_t)).tag).objtype() as libc::c_int
            == 3 as libc::c_int
        {
            (obj as *mut Agedge_t).offset(-(1 as libc::c_int as isize))
        } else {
            (obj as *mut Agedge_t).offset(1 as libc::c_int as isize)
        };
        let ref mut fresh2 = (*(e as *mut Agobj_t)).data;
        *fresh2 = data;
        let ref mut fresh3 = (*e).base.tag;
        (*fresh3).set_mtflock(mtflock as libc::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn aggetrec(
    mut obj: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut mtf: libc::c_int,
) -> *mut Agrec_t {
    let mut hdr: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut d: *mut Agrec_t = 0 as *mut Agrec_t;
    let mut first: *mut Agrec_t = 0 as *mut Agrec_t;
    hdr = obj as *mut Agobj_t;
    d = (*hdr).data;
    first = d;
    while !d.is_null() {
        if streq(name, (*d).name) {
            break;
        }
        d = (*d).next;
        if !(d == first) {
            continue;
        }
        d = 0 as *mut Agrec_t;
        break;
    }
    if !d.is_null() {
        if ((*hdr).tag).mtflock() != 0 {
            if mtf != 0 && (*hdr).data != d {
                agerr(
                    AGERR,
                    b"move to front lock inconsistency\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if d != first || mtf != ((*hdr).tag).mtflock() as libc::c_int {
            set_data(hdr, d, mtf != 0 as libc::c_int);
        }
    }
    return d;
}
unsafe extern "C" fn objputrec(mut obj: *mut Agobj_t, mut arg: *mut libc::c_void) {
    let mut firstrec: *mut Agrec_t = 0 as *mut Agrec_t;
    let mut newrec: *mut Agrec_t = 0 as *mut Agrec_t;
    newrec = arg as *mut Agrec_t;
    firstrec = (*obj).data;
    if firstrec.is_null() {
        let ref mut fresh4 = (*newrec).next;
        *fresh4 = newrec;
    } else if (*firstrec).next == firstrec {
        let ref mut fresh5 = (*firstrec).next;
        *fresh5 = newrec;
        let ref mut fresh6 = (*newrec).next;
        *fresh6 = firstrec;
    } else {
        let ref mut fresh7 = (*newrec).next;
        *fresh7 = (*firstrec).next;
        let ref mut fresh8 = (*firstrec).next;
        *fresh8 = newrec;
    }
    if ((*obj).tag).mtflock() == 0 {
        set_data(obj, newrec, 0 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn agbindrec(
    mut arg_obj: *mut libc::c_void,
    mut recname: *const libc::c_char,
    mut recsize: libc::c_uint,
    mut move_to_front: libc::c_int,
) -> *mut libc::c_void {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut obj: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut rec: *mut Agrec_t = 0 as *mut Agrec_t;
    obj = arg_obj as *mut Agobj_t;
    g = agraphof(obj as *mut libc::c_void);
    rec = aggetrec(obj as *mut libc::c_void, recname, 0 as libc::c_int);
    if rec.is_null() && recsize > 0 as libc::c_int as libc::c_uint {
        rec = agalloc(g, recsize as size_t) as *mut Agrec_t;
        let ref mut fresh9 = (*rec).name;
        *fresh9 = agstrdup(g, recname);
        objputrec(obj, rec as *mut libc::c_void);
    }
    if move_to_front != 0 {
        aggetrec(arg_obj, recname, (0 as libc::c_int == 0) as libc::c_int);
    }
    return rec as *mut libc::c_void;
}
unsafe extern "C" fn objdelrec(
    mut g: *mut Agraph_t,
    mut obj: *mut Agobj_t,
    mut arg_rec: *mut libc::c_void,
) {
    let mut rec: *mut Agrec_t = arg_rec as *mut Agrec_t;
    let mut newrec: *mut Agrec_t = 0 as *mut Agrec_t;
    if (*obj).data == rec {
        if (*rec).next == rec {
            newrec = 0 as *mut Agrec_t;
        } else {
            newrec = (*rec).next;
        }
        set_data(obj, newrec, 0 as libc::c_int != 0);
    }
}
unsafe extern "C" fn listdelrec(mut obj: *mut Agobj_t, mut rec: *mut Agrec_t) {
    let mut prev: *mut Agrec_t = 0 as *mut Agrec_t;
    prev = (*obj).data;
    while (*prev).next != rec {
        prev = (*prev).next;
        if prev != (*obj).data {
        } else {
            __assert_fail(
                b"prev != obj->data\0" as *const u8 as *const libc::c_char,
                b"rec.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                    b"void listdelrec(Agobj_t *, Agrec_t *)\0",
                ))
                .as_ptr(),
            );
        }
    }
    let ref mut fresh10 = (*prev).next;
    *fresh10 = (*rec).next;
}
#[no_mangle]
pub unsafe extern "C" fn agdelrec(
    mut arg_obj: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut obj: *mut Agobj_t = 0 as *mut Agobj_t;
    let mut rec: *mut Agrec_t = 0 as *mut Agrec_t;
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    obj = arg_obj as *mut Agobj_t;
    g = agraphof(obj as *mut libc::c_void);
    rec = aggetrec(obj as *mut libc::c_void, name, 0 as libc::c_int);
    if !rec.is_null() {
        listdelrec(obj, rec);
        match ((*obj).tag).objtype() as libc::c_int {
            0 => {
                objdelrec(g, obj, rec as *mut libc::c_void);
            }
            1 | 3 | 2 => {
                agapply(
                    agroot(g as *mut libc::c_void),
                    obj,
                    Some(
                        objdelrec
                            as unsafe extern "C" fn(
                                *mut Agraph_t,
                                *mut Agobj_t,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                    rec as *mut libc::c_void,
                    0 as libc::c_int,
                );
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"rec.c\0" as *const u8 as *const libc::c_char,
                    156 as libc::c_int,
                );
                abort();
            }
        }
        agstrfree(g, (*rec).name);
        agfree(g, rec as *mut libc::c_void);
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn simple_delrec(
    mut g: *mut Agraph_t,
    mut obj: *mut Agobj_t,
    mut rec_name: *mut libc::c_void,
) {
    agdelrec(obj as *mut libc::c_void, rec_name as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn aginit(
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
    mut rec_name: *const libc::c_char,
    mut arg_rec_size: libc::c_int,
    mut mtf: libc::c_int,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    let mut s: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut rec_size: libc::c_uint = 0;
    let mut recur: bool = arg_rec_size < 0 as libc::c_int;
    rec_size = abs(arg_rec_size) as libc::c_uint;
    match kind {
        0 => {
            agbindrec(g as *mut libc::c_void, rec_name, rec_size, mtf);
            if recur {
                s = agfstsubg(g);
                while !s.is_null() {
                    aginit(s, kind, rec_name, arg_rec_size, mtf);
                    s = agnxtsubg(s);
                }
            }
        }
        1 | 2 | 3 => {
            n = agfstnode(g);
            while !n.is_null() {
                if kind == 1 as libc::c_int {
                    agbindrec(n as *mut libc::c_void, rec_name, rec_size, mtf);
                } else {
                    e = agfstout(g, n);
                    while !e.is_null() {
                        agbindrec(e as *mut libc::c_void, rec_name, rec_size, mtf);
                        e = agnxtout(g, e);
                    }
                }
                n = agnxtnode(g, n);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn agclean(
    mut g: *mut Agraph_t,
    mut kind: libc::c_int,
    mut rec_name: *mut libc::c_char,
) {
    let mut n: *mut Agnode_t = 0 as *mut Agnode_t;
    let mut e: *mut Agedge_t = 0 as *mut Agedge_t;
    match kind {
        0 => {
            agapply(
                g,
                g as *mut Agobj_t,
                Some(
                    simple_delrec
                        as unsafe extern "C" fn(
                            *mut Agraph_t,
                            *mut Agobj_t,
                            *mut libc::c_void,
                        ) -> (),
                ),
                rec_name as *mut libc::c_void,
                (0 as libc::c_int == 0) as libc::c_int,
            );
        }
        1 | 2 | 3 => {
            n = agfstnode(g);
            while !n.is_null() {
                if kind == 1 as libc::c_int {
                    agdelrec(n as *mut libc::c_void, rec_name);
                } else {
                    e = agfstout(g, n);
                    while !e.is_null() {
                        agdelrec(e as *mut libc::c_void, rec_name);
                        e = agnxtout(g, e);
                    }
                }
                n = agnxtnode(g, n);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn agrecclose(mut obj: *mut Agobj_t) {
    let mut g: *mut Agraph_t = 0 as *mut Agraph_t;
    let mut rec: *mut Agrec_t = 0 as *mut Agrec_t;
    let mut nrec: *mut Agrec_t = 0 as *mut Agrec_t;
    g = agraphof(obj as *mut libc::c_void);
    rec = (*obj).data;
    if !rec.is_null() {
        loop {
            nrec = (*rec).next;
            agstrfree(g, (*rec).name);
            agfree(g, rec as *mut libc::c_void);
            rec = nrec;
            if !(rec != (*obj).data) {
                break;
            }
        }
    }
    let ref mut fresh11 = (*obj).data;
    *fresh11 = 0 as *mut Agrec_t;
}
