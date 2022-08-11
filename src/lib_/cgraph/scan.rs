#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut aaglval: AAGSTYPE;
    fn agstrdup(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agstrdup_html(_: *mut Agraph_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut Ag_G_global: *mut Agraph_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type FILE = _IO_FILE;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type flex_uint8_t = uint8_t;
pub type flex_int16_t = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: libc::c_int,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type yy_size_t = size_t;
pub type YY_CHAR = flex_uint8_t;
pub type yy_state_type = libc::c_int;
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
pub type Agdisc_t = Agdisc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Agdisc_s {
    pub mem: *mut Agmemdisc_t,
    pub id: *mut Agiddisc_t,
    pub io: *mut Agiodisc_t,
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
pub type IDTYPE = uint64_t;
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
pub type Dtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Dt_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Dtdisc_t,
    ) -> libc::c_int,
>;
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
pub type Dtdata_t = _dtdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtdata_s {
    pub type_0: libc::c_int,
    pub here: *mut Dtlink_t,
    pub hh: C2RustUnnamed,
    pub ntab: libc::c_int,
    pub size: libc::c_int,
    pub loop_0: libc::c_int,
    pub minp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _htab: *mut *mut Dtlink_t,
    pub _head: *mut Dtlink_t,
}
pub type Dtlink_t = _dtlink_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union AAGSTYPE {
    pub i: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub n: *mut Agnode_s,
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
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh0 = (*xb).buf;
        *fresh0 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh1 = (*xb).buf;
        *fresh1 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh2 = (*xb).eptr;
    *fresh2 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh3 = (*xb).ptr;
    *fresh3 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn agxbfree(mut xb: *mut agxbuf) {
    if (*xb).dyna != 0 {
        free((*xb).buf as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn agxbmore(mut xb: *mut agxbuf, mut ssz: size_t) {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut nsize: size_t = 0 as libc::c_int as size_t;
    let mut nbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    size = ((*xb).eptr).offset_from((*xb).buf) as libc::c_long as size_t;
    nsize = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    if size.wrapping_add(ssz) > nsize {
        nsize = size.wrapping_add(ssz);
    }
    cnt = ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
    if (*xb).dyna != 0 {
        nbuf = gv_recalloc(
            (*xb).buf as *mut libc::c_void,
            size,
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        nbuf = gv_calloc(nsize, ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as *mut libc::c_char;
        memcpy(nbuf as *mut libc::c_void, (*xb).buf as *const libc::c_void, cnt);
        (*xb).dyna = 1 as libc::c_int;
    }
    let ref mut fresh4 = (*xb).buf;
    *fresh4 = nbuf;
    let ref mut fresh5 = (*xb).ptr;
    *fresh5 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh6 = (*xb).eptr;
    *fresh6 = ((*xb).buf).offset(nsize as isize);
}
#[inline]
unsafe extern "C" fn agxbprint(
    mut xb: *mut agxbuf,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut size: size_t = 0;
    let mut result: libc::c_int = 0;
    ap = args.clone();
    let mut ap2: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap2 = ap.clone();
    rc = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap2.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    size = (rc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long
        as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8
                as *const libc::c_char,
            b"./agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int agxbprint(agxbuf *, const char *, ...)\0"))
                .as_ptr(),
        );
    }
    if result > 0 as libc::c_int {
        let ref mut fresh7 = (*xb).ptr;
        *fresh7 = (*fresh7).offset(result as size_t as isize);
    }
    return result;
}
#[inline]
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh8 = (*xb).ptr;
    let fresh9 = *fresh8;
    *fresh8 = (*fresh8).offset(1);
    *fresh9 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn gv_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_recalloc(
    mut ptr: *mut libc::c_void,
    mut old_nmemb: size_t,
    mut new_nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size > 0 as libc::c_int as libc::c_ulong
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void *gv_recalloc(void *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size))
        as libc::c_int as libc::c_long != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(ptr, old_nmemb.wrapping_mul(size), new_nmemb.wrapping_mul(size));
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn gv_realloc(
    mut ptr: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, new_size);
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    if new_size > old_size {
        memset(
            (p as *mut libc::c_char).offset(old_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            new_size.wrapping_sub(old_size),
        );
    }
    return p;
}
static mut line_num: libc::c_int = 1 as libc::c_int;
static mut html_nest: libc::c_int = 0 as libc::c_int;
static mut InputFile: *const libc::c_char = 0 as *const libc::c_char;
static mut Disc: *mut Agdisc_t = 0 as *const Agdisc_t as *mut Agdisc_t;
static mut Ifile: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
static mut graphType: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn agreadline(mut n: libc::c_int) {
    line_num = n;
}
#[no_mangle]
pub unsafe extern "C" fn agsetfile(mut f: *const libc::c_char) {
    InputFile = f;
    line_num = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aglexinit(
    mut disc: *mut Agdisc_t,
    mut ifile: *mut libc::c_void,
) {
    Disc = disc;
    Ifile = ifile;
    graphType = 0 as libc::c_int;
}
#[no_mangle]
pub static mut gv_isatty_suppression: libc::c_int = 0;
static mut Sbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Sptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Send: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn beginstr() {
    if Sbuf.is_null() {
        Sbuf = malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        Send = Sbuf.offset(8192 as libc::c_int as isize);
    }
    Sptr = Sbuf;
    *Sptr = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn addstr(mut src: *mut libc::c_char) {
    let mut c: libc::c_char = 0;
    if Sptr > Sbuf {
        Sptr = Sptr.offset(-1);
    }
    loop {
        loop {
            let fresh11 = src;
            src = src.offset(1);
            let fresh12 = Sptr;
            Sptr = Sptr.offset(1);
            *fresh12 = *fresh11;
            c = *fresh12;
            if !(c as libc::c_int != 0 && Sptr < Send) {
                break;
            }
        }
        if c != 0 {
            let mut sz: libc::c_long = Send.offset_from(Sbuf) as libc::c_long;
            let mut off: libc::c_long = Sptr.offset_from(Sbuf) as libc::c_long;
            sz *= 2 as libc::c_int as libc::c_long;
            Sbuf = realloc(Sbuf as *mut libc::c_void, sz as libc::c_ulong)
                as *mut libc::c_char;
            Send = Sbuf.offset(sz as isize);
            Sptr = Sbuf.offset(off as isize);
        }
        if !(c != 0) {
            break;
        }
    };
}
unsafe extern "C" fn endstr() {
    aaglval.str_0 = agstrdup(Ag_G_global, Sbuf);
    *Sbuf = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn endstr_html() {
    aaglval.str_0 = agstrdup_html(Ag_G_global, Sbuf);
    *Sbuf = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn storeFileName(mut fname: *mut libc::c_char, mut len: libc::c_int) {
    static mut cnt: libc::c_int = 0;
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if len > cnt {
        buf = realloc(
            buf as *mut libc::c_void,
            (len + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        cnt = len;
    }
    strcpy(buf, fname);
    InputFile = buf;
}
unsafe extern "C" fn ppDirective() {
    let mut r: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut lineno: libc::c_int = 0;
    let mut buf: [libc::c_char; 2] = [0; 2];
    let mut s: *mut libc::c_char = aagtext.offset(1 as libc::c_int as isize);
    if strncmp(
        s,
        b"line\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        s = s.offset(4 as libc::c_int as isize);
    }
    r = sscanf(
        s,
        b"%d %1[\"]%n\0" as *const u8 as *const libc::c_char,
        &mut lineno as *mut libc::c_int,
        buf.as_mut_ptr(),
        &mut cnt as *mut libc::c_int,
    );
    if r > 0 as libc::c_int {
        line_num = lineno - 1 as libc::c_int;
        if r > 1 as libc::c_int {
            let mut p: *mut libc::c_char = s.offset(cnt as isize);
            let mut e: *mut libc::c_char = p;
            while *e as libc::c_int != 0 && *e as libc::c_int != '"' as i32 {
                e = e.offset(1);
            }
            if e != p && *e as libc::c_int == '"' as i32 {
                *e = '\0' as i32 as libc::c_char;
                storeFileName(p, e.offset_from(p) as libc::c_long as libc::c_int);
            }
        }
    }
}
unsafe extern "C" fn twoDots() -> bool {
    let mut dot: *const libc::c_char = strchr(aagtext, '.' as i32);
    return !dot.is_null()
        && dot
            != &mut *aagtext.offset((aagleng - 1 as libc::c_int) as isize)
                as *mut libc::c_char as *const libc::c_char;
}
unsafe extern "C" fn chkNum() -> libc::c_int {
    let mut c: libc::c_uchar = *aagtext.offset((aagleng - 1 as libc::c_int) as isize)
        as libc::c_uchar;
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        && c as libc::c_int != '.' as i32
        || c as libc::c_int == '.' as i32 && twoDots() as libc::c_int != 0
    {
        let mut fname: *const libc::c_char = 0 as *const libc::c_char;
        if !InputFile.is_null() {
            fname = InputFile;
        } else {
            fname = b"input\0" as *const u8 as *const libc::c_char;
        }
        agerr(
            AGWARN,
            b"syntax ambiguity - badly delimited number '%s' in line %d of %s splits into two tokens\n\0"
                as *const u8 as *const libc::c_char,
            aagtext,
            line_num,
            fname,
        );
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn aagerror(mut str: *const libc::c_char) {
    let mut xbuf: [libc::c_char; 8192] = [0; 8192];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    agxbinit(&mut xb, 8192 as libc::c_int as libc::c_uint, xbuf.as_mut_ptr());
    if !InputFile.is_null() {
        agxbprint(
            &mut xb as *mut agxbuf,
            b"%s: \0" as *const u8 as *const libc::c_char,
            InputFile,
        );
    }
    agxbprint(
        &mut xb as *mut agxbuf,
        b"%s in line %d\0" as *const u8 as *const libc::c_char,
        str,
        line_num,
    );
    if *aagtext != 0 {
        agxbprint(
            &mut xb as *mut agxbuf,
            b" near '%s'\0" as *const u8 as *const libc::c_char,
            aagtext,
        );
    } else {
        match (yy_start - 1 as libc::c_int) / 2 as libc::c_int {
            2 => {
                agxbprint(
                    &mut xb as *mut agxbuf,
                    b" scanning a quoted string (missing endquote? longer than %d?)\0"
                        as *const u8 as *const libc::c_char,
                    16384 as libc::c_int,
                );
                if *Sbuf != 0 {
                    let mut len: size_t = strlen(Sbuf);
                    if len > 80 as libc::c_int as libc::c_ulong {
                        *Sbuf
                            .offset(
                                80 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                    }
                    agxbprint(
                        &mut xb as *mut agxbuf,
                        b"\nString starting:\"%s\0" as *const u8 as *const libc::c_char,
                        Sbuf,
                    );
                }
            }
            3 => {
                agxbprint(
                    &mut xb as *mut agxbuf,
                    b" scanning a HTML string (missing '>'? bad nesting? longer than %d?)\0"
                        as *const u8 as *const libc::c_char,
                    16384 as libc::c_int,
                );
                if *Sbuf != 0 {
                    let mut len_0: size_t = strlen(Sbuf);
                    if len_0 > 80 as libc::c_int as libc::c_ulong {
                        *Sbuf
                            .offset(
                                80 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                    }
                    agxbprint(
                        &mut xb as *mut agxbuf,
                        b"\nString starting:<%s\0" as *const u8 as *const libc::c_char,
                        Sbuf,
                    );
                }
            }
            1 => {
                agxbprint(
                    &mut xb as *mut agxbuf,
                    b" scanning a /*...*/ comment (missing '*/? longer than %d?)\0"
                        as *const u8 as *const libc::c_char,
                    16384 as libc::c_int,
                );
            }
            _ => {}
        }
    }
    agxbputc(&mut xb, '\n' as i32 as libc::c_char);
    agerr(AGERR, b"%s\0" as *const u8 as *const libc::c_char, agxbuse(&mut xb));
    agxbfree(&mut xb);
    yy_start = 1 as libc::c_int + 2 as libc::c_int * 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aglexeof() {
    yyunput('@' as i32, aagtext);
}
#[no_mangle]
pub unsafe extern "C" fn aglexbad() {
    aag_flush_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn aagwrap() -> libc::c_int {
    return 1 as libc::c_int;
}
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
#[no_mangle]
pub static mut aagleng: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
#[no_mangle]
pub static mut aagin: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut aagout: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut aaglineno: libc::c_int = 1 as libc::c_int;
static mut yy_accept: [flex_int16_t; 93] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
];
static mut yy_ec: [YY_CHAR; 256] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    11 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    12 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    14 as libc::c_int as YY_CHAR,
    15 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    17 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    20 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    22 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    24 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    27 as libc::c_int as YY_CHAR,
    28 as libc::c_int as YY_CHAR,
    29 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    30 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    31 as libc::c_int as YY_CHAR,
    32 as libc::c_int as YY_CHAR,
    33 as libc::c_int as YY_CHAR,
    34 as libc::c_int as YY_CHAR,
    35 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    36 as libc::c_int as YY_CHAR,
    37 as libc::c_int as YY_CHAR,
    38 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    39 as libc::c_int as YY_CHAR,
    40 as libc::c_int as YY_CHAR,
    41 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    42 as libc::c_int as YY_CHAR,
    43 as libc::c_int as YY_CHAR,
    44 as libc::c_int as YY_CHAR,
    45 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    46 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    47 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    48 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
];
static mut yy_meta: [YY_CHAR; 49] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
];
static mut yy_base: [flex_int16_t; 105] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    174 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    219 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    168 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    184 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
];
static mut yy_def: [flex_int16_t; 105] = [
    0 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 268] = [
    0 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 268] = [
    0 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut aag_flex_debug: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut aagtext: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn aaglex() -> libc::c_int {
    let mut yy_amount_of_matched_text: libc::c_int = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    if yy_init == 0 {
        yy_init = 1 as libc::c_int;
        if yy_start == 0 {
            yy_start = 1 as libc::c_int;
        }
        if aagin.is_null() {
            aagin = stdin;
        }
        if aagout.is_null() {
            aagout = stdout;
        }
        if if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        }
            .is_null()
        {
            aagensure_buffer_stack();
            let ref mut fresh13 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh13 = aag_create_buffer(aagin, 16384 as libc::c_int);
        }
        aag_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        yy_current_state
            += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
        'c_8027: loop {
            loop {
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as YY_CHAR as usize];
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 93 as libc::c_int {
                        yy_c = yy_meta[yy_c as usize];
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_int + yy_c as libc::c_int) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 219 as libc::c_int)
                {
                    break;
                }
            }
            'c_8028: loop {
                yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                if yy_act == 0 as libc::c_int {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                }
                aagtext = yy_bp;
                aagleng = yy_cp.offset_from(yy_bp) as libc::c_long as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue 'c_8028;
                        }
                        1 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return -(1 as libc::c_int);
                        }
                        2 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            break 'c_8027;
                        }
                        3 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 1 as libc::c_int;
                            break 'c_8027;
                        }
                        4 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8027;
                        }
                        5 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8027;
                        }
                        6 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            break 'c_8027;
                        }
                        7 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8027;
                        }
                        8 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            ppDirective();
                            break 'c_8027;
                        }
                        9 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8027;
                        }
                        10 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8027;
                        }
                        11 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8027;
                        }
                        12 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 259 as libc::c_int;
                        }
                        13 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 260 as libc::c_int;
                        }
                        14 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            if graphType == 0 {
                                graphType = 258 as libc::c_int;
                            }
                            return 258 as libc::c_int;
                        }
                        15 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            if graphType == 0 {
                                graphType = 261 as libc::c_int;
                            }
                            return 261 as libc::c_int;
                        }
                        16 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 263 as libc::c_int;
                        }
                        17 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 262 as libc::c_int;
                        }
                        18 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            if graphType == 261 as libc::c_int {
                                return 264 as libc::c_int
                            } else {
                                return '-' as i32
                            }
                        }
                        19 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            if graphType == 258 as libc::c_int {
                                return 264 as libc::c_int
                            } else {
                                return '-' as i32
                            }
                        }
                        20 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            aaglval.str_0 = agstrdup(Ag_G_global, aagtext);
                            return 267 as libc::c_int;
                        }
                        21 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            if chkNum() != 0 {
                                let mut yyless_macro_arg: libc::c_int = aagleng
                                    - 1 as libc::c_int;
                                *yy_cp = yy_hold_char;
                                yy_cp = yy_bp
                                    .offset(yyless_macro_arg as isize)
                                    .offset(-(0 as libc::c_int as isize));
                                yy_c_buf_p = yy_cp;
                                aagtext = yy_bp;
                                aagleng = yy_cp.offset_from(yy_bp) as libc::c_long
                                    as libc::c_int;
                                yy_hold_char = *yy_cp;
                                *yy_cp = '\0' as i32 as libc::c_char;
                                yy_c_buf_p = yy_cp;
                            }
                            aaglval.str_0 = agstrdup(Ag_G_global, aagtext);
                            return 267 as libc::c_int;
                        }
                        22 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 2 as libc::c_int;
                            beginstr();
                            break 'c_8027;
                        }
                        23 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            endstr();
                            return 268 as libc::c_int;
                        }
                        24 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            addstr(
                                b"\"\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            break 'c_8027;
                        }
                        25 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            addstr(
                                b"\\\\\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            break 'c_8027;
                        }
                        26 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            break 'c_8027;
                        }
                        27 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            addstr(
                                b"\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            line_num += 1;
                            break 'c_8027;
                        }
                        28 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            addstr(aagtext);
                            break 'c_8027;
                        }
                        29 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 3 as libc::c_int;
                            html_nest = 1 as libc::c_int;
                            beginstr();
                            break 'c_8027;
                        }
                        30 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            html_nest -= 1;
                            if html_nest != 0 {
                                addstr(aagtext);
                            } else {
                                yy_start = 1 as libc::c_int
                                    + 2 as libc::c_int * 0 as libc::c_int;
                                endstr_html();
                                return 268 as libc::c_int;
                            }
                            break 'c_8027;
                        }
                        31 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            html_nest += 1;
                            addstr(aagtext);
                            break 'c_8027;
                        }
                        32 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            addstr(aagtext);
                            line_num += 1;
                            break 'c_8027;
                        }
                        33 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            addstr(aagtext);
                            break 'c_8027;
                        }
                        34 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return *aagtext.offset(0 as libc::c_int as isize)
                                as libc::c_int;
                        }
                        35 => {
                            if aagleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*aagtext
                                    .offset((aagleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            fwrite(
                                aagtext as *const libc::c_void,
                                aagleng as size_t,
                                1 as libc::c_int as libc::c_ulong,
                                aagout,
                            ) != 0;
                            break 'c_8027;
                        }
                        37 | 38 | 39 | 40 => return 0 as libc::c_int,
                        36 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(aagtext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh14 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh14 = aagin;
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_buffer_status = 1 as libc::c_int;
                            }
                            if yy_c_buf_p
                                <= &mut *((**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_ch_buf)
                                    .offset(yy_n_chars as isize) as *mut libc::c_char
                            {
                                yy_next_state = 0;
                                yy_c_buf_p = aagtext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = aagtext.offset(0 as libc::c_int as isize);
                                if yy_next_state != 0 {
                                    current_block = 5085513901186096404;
                                    break;
                                } else {
                                    current_block = 1365401674109093360;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        if aagwrap() != 0 {
                                            yy_c_buf_p = aagtext.offset(0 as libc::c_int as isize);
                                            yy_act = 36 as libc::c_int
                                                + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                                + 1 as libc::c_int;
                                        } else {
                                            if yy_did_buffer_switch_on_eof == 0 {
                                                aagrestart(aagin);
                                            }
                                            break 'c_8027;
                                        }
                                    }
                                    0 => {
                                        yy_c_buf_p = aagtext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = aagtext.offset(0 as libc::c_int as isize);
                                        break 'c_8028;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((**yy_buffer_stack
                                            .offset(yy_buffer_stack_top as isize))
                                            .yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut libc::c_char;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = aagtext.offset(0 as libc::c_int as isize);
                                        continue 'c_8028;
                                    }
                                    _ => {
                                        break 'c_8027;
                                    }
                                }
                            }
                        }
                        _ => {
                            yy_fatal_error(
                                b"fatal flex scanner internal error--no action found\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                match current_block {
                    1365401674109093360 => {
                        yy_cp = yy_c_buf_p;
                    }
                    _ => {
                        yy_c_buf_p = yy_c_buf_p.offset(1);
                        yy_cp = yy_c_buf_p;
                        yy_current_state = yy_next_state;
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yy_get_next_buffer() -> libc::c_int {
    let mut dest: *mut libc::c_char = (**yy_buffer_stack
        .offset(yy_buffer_stack_top as isize))
        .yy_ch_buf;
    let mut source: *mut libc::c_char = aagtext;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    if yy_c_buf_p
        > &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset((yy_n_chars + 1 as libc::c_int) as isize) as *mut libc::c_char
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_fill_buffer
        == 0 as libc::c_int
    {
        if yy_c_buf_p.offset_from(aagtext) as libc::c_long
            - 0 as libc::c_int as libc::c_long == 1 as libc::c_int as libc::c_long
        {
            return 1 as libc::c_int
        } else {
            return 2 as libc::c_int
        }
    }
    number_to_move = (yy_c_buf_p.offset_from(aagtext) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh15 = source;
        source = source.offset(1);
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = *fresh15;
        i += 1;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as libc::c_int
    {
        yy_n_chars = 0 as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: libc::c_int = (**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_buf_size - number_to_move - 1 as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            let mut b: YY_BUFFER_STATE = *yy_buffer_stack
                .offset(yy_buffer_stack_top as isize);
            let mut yy_c_buf_p_offset: libc::c_int = yy_c_buf_p
                .offset_from((*b).yy_ch_buf) as libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int = (*b).yy_buf_size * 2 as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b).yy_buf_size += (*b).yy_buf_size / 8 as libc::c_int;
                } else {
                    (*b).yy_buf_size *= 2 as libc::c_int;
                }
                let ref mut fresh17 = (*b).yy_ch_buf;
                *fresh17 = aagrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t,
                ) as *mut libc::c_char;
            } else {
                let ref mut fresh18 = (*b).yy_ch_buf;
                *fresh18 = 0 as *mut libc::c_char;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const libc::c_char,
                );
            }
            yy_c_buf_p = &mut *((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize)
                as *mut libc::c_char;
            num_to_read = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buf_size - number_to_move - 1 as libc::c_int;
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int;
        }
        yy_n_chars = ((*(*Disc).io).afread)
            .expect(
                "non-null function pointer",
            )(
            Ifile,
            &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(number_to_move as isize),
            num_to_read,
        );
        if yy_n_chars < 0 as libc::c_int {
            yy_fatal_error(
                b"input in flex scanner failed\0" as *const u8 as *const libc::c_char,
            );
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
            ret_val = 1 as libc::c_int;
            aagrestart(aagin);
        } else {
            ret_val = 2 as libc::c_int;
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buffer_status = 2 as libc::c_int;
        }
    } else {
        ret_val = 0 as libc::c_int;
    }
    if yy_n_chars + number_to_move
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        let mut new_size_0: libc::c_int = yy_n_chars + number_to_move
            + (yy_n_chars >> 1 as libc::c_int);
        let ref mut fresh19 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh19 = aagrealloc(
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
                as *mut libc::c_void,
            new_size_0 as yy_size_t,
        ) as *mut libc::c_char;
        if ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf).is_null()
        {
            yy_fatal_error(
                b"out of dynamic memory in yy_get_next_buffer()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_size = new_size_0 - 2 as libc::c_int;
    }
    yy_n_chars += number_to_move;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as libc::c_int as libc::c_char;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(
            (yy_n_chars + 1 as libc::c_int) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    aagtext = &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_current_state
        += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
    yy_cp = aagtext.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as libc::c_int != 0 {
            yy_ec[*yy_cp as YY_CHAR as usize] as libc::c_int
        } else {
            1 as libc::c_int
        }) as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 93 as libc::c_int {
                yy_c = yy_meta[yy_c as usize];
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
    }
    return yy_current_state;
}
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: libc::c_int = 0;
    let mut yy_cp: *mut libc::c_char = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 93 as libc::c_int {
            yy_c = yy_meta[yy_c as usize];
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 92 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
unsafe extern "C" fn yyunput(mut c: libc::c_int, mut yy_bp: *mut libc::c_char) {
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_cp = yy_c_buf_p;
    *yy_cp = yy_hold_char;
    if yy_cp
        < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset(2 as libc::c_int as isize)
    {
        let mut number_to_move: libc::c_int = yy_n_chars + 2 as libc::c_int;
        let mut dest: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(
                ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
                    + 2 as libc::c_int) as isize,
            ) as *mut libc::c_char;
        let mut source: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(number_to_move as isize) as *mut libc::c_char;
        while source > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
        {
            source = source.offset(-1);
            dest = dest.offset(-1);
            *dest = *source;
        }
        yy_cp = yy_cp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_bp = yy_bp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_size;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
        if yy_cp
            < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(2 as libc::c_int as isize)
        {
            yy_fatal_error(
                b"flex scanner push-back overflow\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    yy_cp = yy_cp.offset(-1);
    *yy_cp = c as libc::c_char;
    aagtext = yy_bp;
    yy_hold_char = *yy_cp;
    yy_c_buf_p = yy_cp;
}
#[no_mangle]
pub unsafe extern "C" fn aagrestart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        aagensure_buffer_stack();
        let ref mut fresh20 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh20 = aag_create_buffer(aagin, 16384 as libc::c_int);
    }
    aag_init_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
        input_file,
    );
    aag_load_buffer_state();
}
#[no_mangle]
pub unsafe extern "C" fn aag_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    aagensure_buffer_stack();
    if (if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }) == new_buffer
    {
        return;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh21 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh21 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh22 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh22 = new_buffer;
    aag_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
unsafe extern "C" fn aag_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    aagtext = yy_c_buf_p;
    aagin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
#[no_mangle]
pub unsafe extern "C" fn aag_create_buffer(
    mut file: *mut FILE,
    mut size: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = aagalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size;
    let ref mut fresh23 = (*b).yy_ch_buf;
    *fresh23 = aagalloc(((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t)
        as *mut libc::c_char;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    aag_init_buffer(b, file);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn aag_delete_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        let ref mut fresh24 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh24 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        aagfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    aagfree(b as *mut libc::c_void);
}
unsafe extern "C" fn aag_init_buffer(mut b: YY_BUFFER_STATE, mut file: *mut FILE) {
    let mut oerrno: libc::c_int = *__errno_location();
    aag_flush_buffer(b);
    let ref mut fresh25 = (*b).yy_input_file;
    *fresh25 = file;
    (*b).yy_fill_buffer = 1 as libc::c_int;
    if b
        != (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        (*b).yy_bs_lineno = 1 as libc::c_int;
        (*b).yy_bs_column = 0 as libc::c_int;
    }
    (*b)
        .yy_is_interactive = if !file.is_null() {
        (gv_isatty_suppression > 0 as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
    *__errno_location() = oerrno;
}
#[no_mangle]
pub unsafe extern "C" fn aag_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as libc::c_int;
    *((*b).yy_ch_buf)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *((*b).yy_ch_buf)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    let ref mut fresh26 = (*b).yy_buf_pos;
    *fresh26 = &mut *((*b).yy_ch_buf).offset(0 as libc::c_int as isize)
        as *mut libc::c_char;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        aag_load_buffer_state();
    }
}
#[no_mangle]
pub unsafe extern "C" fn aagpush_buffer_state(mut new_buffer: YY_BUFFER_STATE) {
    if new_buffer.is_null() {
        return;
    }
    aagensure_buffer_stack();
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh27 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh27 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_add(1);
    }
    let ref mut fresh28 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh28 = new_buffer;
    aag_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aagpop_buffer_state() {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        return;
    }
    aag_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    let ref mut fresh29 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh29 = 0 as YY_BUFFER_STATE;
    if yy_buffer_stack_top > 0 as libc::c_int as libc::c_ulong {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_sub(1);
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        aag_load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as libc::c_int;
    }
}
unsafe extern "C" fn aagensure_buffer_stack() {
    let mut num_to_alloc: yy_size_t = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int as yy_size_t;
        yy_buffer_stack = aagalloc(
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack as *mut libc::c_void,
            0 as libc::c_int,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return;
    }
    if yy_buffer_stack_top
        >= yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut grow_size: yy_size_t = 8 as libc::c_int as yy_size_t;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size);
        yy_buffer_stack = aagrealloc(
            yy_buffer_stack as *mut libc::c_void,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack.offset(yy_buffer_stack_max as isize) as *mut libc::c_void,
            0 as libc::c_int,
            grow_size
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn aag_scan_buffer(
    mut base: *mut libc::c_char,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as libc::c_int as libc::c_ulong
        || *base.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
        || *base.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
    {
        return 0 as YY_BUFFER_STATE;
    }
    b = aagalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b)
        .yy_buf_size = size.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let ref mut fresh30 = (*b).yy_ch_buf;
    *fresh30 = base;
    let ref mut fresh31 = (*b).yy_buf_pos;
    *fresh31 = *fresh30;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    let ref mut fresh32 = (*b).yy_input_file;
    *fresh32 = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    aag_switch_to_buffer(b);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn aag_scan_string(
    mut yystr: *const libc::c_char,
) -> YY_BUFFER_STATE {
    return aag_scan_bytes(yystr, strlen(yystr) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn aag_scan_bytes(
    mut yybytes: *const libc::c_char,
    mut _yybytes_len: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    buf = aagalloc(n) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1;
    }
    let ref mut fresh33 = *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh33 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh33;
    b = aag_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) -> ! {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn aagget_lineno() -> libc::c_int {
    return aaglineno;
}
#[no_mangle]
pub unsafe extern "C" fn aagget_in() -> *mut FILE {
    return aagin;
}
#[no_mangle]
pub unsafe extern "C" fn aagget_out() -> *mut FILE {
    return aagout;
}
#[no_mangle]
pub unsafe extern "C" fn aagget_leng() -> libc::c_int {
    return aagleng;
}
#[no_mangle]
pub unsafe extern "C" fn aagget_text() -> *mut libc::c_char {
    return aagtext;
}
#[no_mangle]
pub unsafe extern "C" fn aagset_lineno(mut _line_number: libc::c_int) {
    aaglineno = _line_number;
}
#[no_mangle]
pub unsafe extern "C" fn aagset_in(mut _in_str: *mut FILE) {
    aagin = _in_str;
}
#[no_mangle]
pub unsafe extern "C" fn aagset_out(mut _out_str: *mut FILE) {
    aagout = _out_str;
}
#[no_mangle]
pub unsafe extern "C" fn aagget_debug() -> libc::c_int {
    return aag_flex_debug;
}
#[no_mangle]
pub unsafe extern "C" fn aagset_debug(mut _bdebug: libc::c_int) {
    aag_flex_debug = _bdebug;
}
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as libc::c_int as size_t;
    yy_buffer_stack_max = 0 as libc::c_int as size_t;
    yy_c_buf_p = 0 as *mut libc::c_char;
    yy_init = 0 as libc::c_int;
    yy_start = 0 as libc::c_int;
    aagin = 0 as *mut FILE;
    aagout = 0 as *mut FILE;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aaglex_destroy() -> libc::c_int {
    while !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        aag_delete_buffer(
            if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            },
        );
        let ref mut fresh34 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh34 = 0 as YY_BUFFER_STATE;
        aagpop_buffer_state();
    }
    aagfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aagalloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn aagrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn aagfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
}
