#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut gmllval: GMLSTYPE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
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
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmlattr {
    pub link: Dtlink_t,
    pub kind: libc::c_ushort,
    pub sort: libc::c_ushort,
    pub name: *mut libc::c_char,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub value: *mut libc::c_char,
    pub lp: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmlnode {
    pub link: Dtlink_t,
    pub id: *mut libc::c_char,
    pub attrlist: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmledge {
    pub link: Dtlink_t,
    pub source: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub attrlist: *mut Dt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union GMLSTYPE {
    pub i: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub np: *mut gmlnode,
    pub ep: *mut gmledge,
    pub ap: *mut gmlattr,
    pub list: *mut Dt_t,
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
#[inline]
unsafe extern "C" fn gv_strdup(mut original: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(original);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
static mut line_num: libc::c_int = 1 as libc::c_int;
static mut errors: libc::c_int = 0;
static mut Ifile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn initgmlscan(mut ifile: *mut FILE) {
    if !ifile.is_null() {
        Ifile = ifile;
        line_num = 1 as libc::c_int;
    }
    errors = 0 as libc::c_int;
}
static mut Sbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn beginstr() {
    if Sbuf.is_null()
        && !(b"leaking memory\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"Sbuf == NULL && \"leaking memory\"\0" as *const u8 as *const libc::c_char,
            b"../../cmd/tools/gmlscan.l\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"void beginstr(void)\0"))
                .as_ptr(),
        );
    }
    Sbuf = gv_strdup(b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn addstr(mut src: *const libc::c_char) {
    if !Sbuf.is_null()
        && !(b"missing beginstr()\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"Sbuf != NULL && \"missing beginstr()\"\0" as *const u8
                as *const libc::c_char,
            b"../../cmd/tools/gmlscan.l\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void addstr(const char *)\0"))
                .as_ptr(),
        );
    }
    let mut old_size: size_t = (strlen(Sbuf))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut new_size: size_t = old_size.wrapping_add(strlen(src));
    Sbuf = gv_realloc(Sbuf as *mut libc::c_void, old_size, new_size)
        as *mut libc::c_char;
    strcat(Sbuf, src);
}
unsafe extern "C" fn endstr() {
    if !Sbuf.is_null()
        && !(b"missing beginstr()\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"Sbuf != NULL && \"missing beginstr()\"\0" as *const u8
                as *const libc::c_char,
            b"../../cmd/tools/gmlscan.l\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"void endstr(void)\0"))
                .as_ptr(),
        );
    }
    gmllval.str_0 = Sbuf;
    Sbuf = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn gmlerror(mut str: *const libc::c_char) {
    if errors != 0 {
        return;
    }
    errors = 1 as libc::c_int;
    agerr(
        AGWARN,
        b" %s in line %d near '%s'\n\0" as *const u8 as *const libc::c_char,
        str,
        line_num,
        gmltext,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gmlerrors() -> libc::c_int {
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn gmllexeof() {
    yyunput('@' as i32, gmltext);
}
#[no_mangle]
pub unsafe extern "C" fn gmlwrap() -> libc::c_int {
    return 1 as libc::c_int;
}
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
#[no_mangle]
pub static mut gmlleng: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
#[no_mangle]
pub static mut gmlin: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut gmlout: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut gmllineno: libc::c_int = 1 as libc::c_int;
static mut yy_accept: [flex_int16_t; 143] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
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
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    11 as libc::c_int as YY_CHAR,
    12 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    14 as libc::c_int as YY_CHAR,
    15 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    17 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    20 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    22 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    24 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    27 as libc::c_int as YY_CHAR,
    28 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    29 as libc::c_int as YY_CHAR,
    30 as libc::c_int as YY_CHAR,
    31 as libc::c_int as YY_CHAR,
    32 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    33 as libc::c_int as YY_CHAR,
    34 as libc::c_int as YY_CHAR,
    35 as libc::c_int as YY_CHAR,
    36 as libc::c_int as YY_CHAR,
    37 as libc::c_int as YY_CHAR,
    38 as libc::c_int as YY_CHAR,
    39 as libc::c_int as YY_CHAR,
    40 as libc::c_int as YY_CHAR,
    41 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    42 as libc::c_int as YY_CHAR,
    43 as libc::c_int as YY_CHAR,
    44 as libc::c_int as YY_CHAR,
    45 as libc::c_int as YY_CHAR,
    46 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    47 as libc::c_int as YY_CHAR,
    48 as libc::c_int as YY_CHAR,
    49 as libc::c_int as YY_CHAR,
    50 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    51 as libc::c_int as YY_CHAR,
    52 as libc::c_int as YY_CHAR,
    53 as libc::c_int as YY_CHAR,
    54 as libc::c_int as YY_CHAR,
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
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_meta: [YY_CHAR; 55] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
];
static mut yy_base: [flex_int16_t; 147] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    158 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    159 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    174 as libc::c_int as flex_int16_t,
    157 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    180 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    214 as libc::c_int as flex_int16_t,
    215 as libc::c_int as flex_int16_t,
    216 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    228 as libc::c_int as flex_int16_t,
    217 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    224 as libc::c_int as flex_int16_t,
    222 as libc::c_int as flex_int16_t,
    221 as libc::c_int as flex_int16_t,
    234 as libc::c_int as flex_int16_t,
    223 as libc::c_int as flex_int16_t,
    242 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    238 as libc::c_int as flex_int16_t,
    232 as libc::c_int as flex_int16_t,
    248 as libc::c_int as flex_int16_t,
    249 as libc::c_int as flex_int16_t,
    247 as libc::c_int as flex_int16_t,
    240 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    301 as libc::c_int as flex_int16_t,
    288 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    292 as libc::c_int as flex_int16_t,
    296 as libc::c_int as flex_int16_t,
];
static mut yy_def: [flex_int16_t; 147] = [
    0 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 356] = [
    0 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 356] = [
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
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    132 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    136 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    145 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut gml_flex_debug: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut gmltext: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn gmllex() -> libc::c_int {
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
        if gmlin.is_null() {
            gmlin = stdin;
        }
        if gmlout.is_null() {
            gmlout = stdout;
        }
        if if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        }
            .is_null()
        {
            gmlensure_buffer_stack();
            let ref mut fresh0 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh0 = gml_create_buffer(gmlin, 16384 as libc::c_int);
        }
        gml_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        yy_current_state
            += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
        'c_8770: loop {
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
                    if yy_current_state >= 143 as libc::c_int {
                        yy_c = yy_meta[yy_c as usize];
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_int + yy_c as libc::c_int) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 301 as libc::c_int)
                {
                    break;
                }
            }
            'c_8771: loop {
                yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                if yy_act == 0 as libc::c_int {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                }
                gmltext = yy_bp;
                gmlleng = yy_cp.offset_from(yy_bp) as libc::c_long as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue 'c_8771;
                        }
                        1 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return -(1 as libc::c_int);
                        }
                        2 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            break 'c_8770;
                        }
                        3 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8770;
                        }
                        4 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break 'c_8770;
                        }
                        5 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 258 as libc::c_int;
                        }
                        6 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 259 as libc::c_int;
                        }
                        7 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 260 as libc::c_int;
                        }
                        8 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 261 as libc::c_int;
                        }
                        9 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 287 as libc::c_int;
                        }
                        10 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 262 as libc::c_int;
                        }
                        11 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 263 as libc::c_int;
                        }
                        12 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 264 as libc::c_int;
                        }
                        13 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 265 as libc::c_int;
                        }
                        14 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 266 as libc::c_int;
                        }
                        15 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 267 as libc::c_int;
                        }
                        16 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 268 as libc::c_int;
                        }
                        17 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 269 as libc::c_int;
                        }
                        18 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 270 as libc::c_int;
                        }
                        19 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 271 as libc::c_int;
                        }
                        20 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 272 as libc::c_int;
                        }
                        21 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 273 as libc::c_int;
                        }
                        22 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 274 as libc::c_int;
                        }
                        23 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 275 as libc::c_int;
                        }
                        24 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 276 as libc::c_int;
                        }
                        25 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 277 as libc::c_int;
                        }
                        26 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 278 as libc::c_int;
                        }
                        27 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 279 as libc::c_int;
                        }
                        28 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 280 as libc::c_int;
                        }
                        29 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 281 as libc::c_int;
                        }
                        30 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 282 as libc::c_int;
                        }
                        31 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return 283 as libc::c_int;
                        }
                        32 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            gmllval.str_0 = strdup(gmltext);
                            return 284 as libc::c_int;
                        }
                        33 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            gmllval.str_0 = strdup(gmltext);
                            return 285 as libc::c_int;
                        }
                        34 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            gmllval.str_0 = strdup(gmltext);
                            return 288 as libc::c_int;
                        }
                        35 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 1 as libc::c_int;
                            beginstr();
                            break 'c_8770;
                        }
                        36 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            endstr();
                            return 286 as libc::c_int;
                        }
                        37 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            addstr(gmltext);
                            break 'c_8770;
                        }
                        38 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return *gmltext.offset(0 as libc::c_int as isize)
                                as libc::c_int;
                        }
                        39 => {
                            if gmlleng > 0 as libc::c_int {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*gmltext
                                    .offset((gmlleng - 1 as libc::c_int) as isize)
                                    as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            fwrite(
                                gmltext as *const libc::c_void,
                                gmlleng as size_t,
                                1 as libc::c_int as libc::c_ulong,
                                gmlout,
                            ) != 0;
                            break 'c_8770;
                        }
                        41 | 42 => return 0 as libc::c_int,
                        40 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(gmltext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh1 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh1 = gmlin;
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
                                yy_c_buf_p = gmltext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = gmltext.offset(0 as libc::c_int as isize);
                                if yy_next_state != 0 {
                                    current_block = 12463749970033092792;
                                    break;
                                } else {
                                    current_block = 12153365054289215322;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        if gmlwrap() != 0 {
                                            yy_c_buf_p = gmltext.offset(0 as libc::c_int as isize);
                                            yy_act = 40 as libc::c_int
                                                + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                                + 1 as libc::c_int;
                                        } else {
                                            if yy_did_buffer_switch_on_eof == 0 {
                                                gmlrestart(gmlin);
                                            }
                                            break 'c_8770;
                                        }
                                    }
                                    0 => {
                                        yy_c_buf_p = gmltext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = gmltext.offset(0 as libc::c_int as isize);
                                        break 'c_8771;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((**yy_buffer_stack
                                            .offset(yy_buffer_stack_top as isize))
                                            .yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut libc::c_char;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = gmltext.offset(0 as libc::c_int as isize);
                                        continue 'c_8771;
                                    }
                                    _ => {
                                        break 'c_8770;
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
                    12153365054289215322 => {
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
    let mut source: *mut libc::c_char = gmltext;
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
        if yy_c_buf_p.offset_from(gmltext) as libc::c_long
            - 0 as libc::c_int as libc::c_long == 1 as libc::c_int as libc::c_long
        {
            return 1 as libc::c_int
        } else {
            return 2 as libc::c_int
        }
    }
    number_to_move = (yy_c_buf_p.offset_from(gmltext) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh2 = source;
        source = source.offset(1);
        let fresh3 = dest;
        dest = dest.offset(1);
        *fresh3 = *fresh2;
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
                let ref mut fresh4 = (*b).yy_ch_buf;
                *fresh4 = gmlrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t,
                ) as *mut libc::c_char;
            } else {
                let ref mut fresh5 = (*b).yy_ch_buf;
                *fresh5 = 0 as *mut libc::c_char;
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
        yy_n_chars = fread(
            &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(number_to_move as isize) as *mut libc::c_char
                as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            num_to_read as libc::c_ulong,
            Ifile,
        ) as libc::c_int;
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
            gmlrestart(gmlin);
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
        let ref mut fresh6 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh6 = gmlrealloc(
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
    gmltext = &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_current_state
        += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
    yy_cp = gmltext.offset(0 as libc::c_int as isize);
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
            if yy_current_state >= 143 as libc::c_int {
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
        if yy_current_state >= 143 as libc::c_int {
            yy_c = yy_meta[yy_c as usize];
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 142 as libc::c_int) as libc::c_int;
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
    gmltext = yy_bp;
    yy_hold_char = *yy_cp;
    yy_c_buf_p = yy_cp;
}
#[no_mangle]
pub unsafe extern "C" fn gmlrestart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        gmlensure_buffer_stack();
        let ref mut fresh7 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh7 = gml_create_buffer(gmlin, 16384 as libc::c_int);
    }
    gml_init_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
        input_file,
    );
    gml_load_buffer_state();
}
#[no_mangle]
pub unsafe extern "C" fn gml_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    gmlensure_buffer_stack();
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
        let ref mut fresh8 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh8 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh9 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh9 = new_buffer;
    gml_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
unsafe extern "C" fn gml_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    gmltext = yy_c_buf_p;
    gmlin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
#[no_mangle]
pub unsafe extern "C" fn gml_create_buffer(
    mut file: *mut FILE,
    mut size: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = gmlalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size;
    let ref mut fresh10 = (*b).yy_ch_buf;
    *fresh10 = gmlalloc(((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t)
        as *mut libc::c_char;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    gml_init_buffer(b, file);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gml_delete_buffer(mut b: YY_BUFFER_STATE) {
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
        let ref mut fresh11 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh11 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        gmlfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    gmlfree(b as *mut libc::c_void);
}
unsafe extern "C" fn gml_init_buffer(mut b: YY_BUFFER_STATE, mut file: *mut FILE) {
    let mut oerrno: libc::c_int = *__errno_location();
    gml_flush_buffer(b);
    let ref mut fresh12 = (*b).yy_input_file;
    *fresh12 = file;
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
        (isatty(fileno(file)) > 0 as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
    *__errno_location() = oerrno;
}
#[no_mangle]
pub unsafe extern "C" fn gml_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as libc::c_int;
    *((*b).yy_ch_buf)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *((*b).yy_ch_buf)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    let ref mut fresh13 = (*b).yy_buf_pos;
    *fresh13 = &mut *((*b).yy_ch_buf).offset(0 as libc::c_int as isize)
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
        gml_load_buffer_state();
    }
}
#[no_mangle]
pub unsafe extern "C" fn gmlpush_buffer_state(mut new_buffer: YY_BUFFER_STATE) {
    if new_buffer.is_null() {
        return;
    }
    gmlensure_buffer_stack();
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh14 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh14 = yy_c_buf_p;
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
    let ref mut fresh15 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh15 = new_buffer;
    gml_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gmlpop_buffer_state() {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        return;
    }
    gml_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    let ref mut fresh16 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh16 = 0 as YY_BUFFER_STATE;
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
        gml_load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as libc::c_int;
    }
}
unsafe extern "C" fn gmlensure_buffer_stack() {
    let mut num_to_alloc: yy_size_t = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int as yy_size_t;
        yy_buffer_stack = gmlalloc(
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
        yy_buffer_stack = gmlrealloc(
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
pub unsafe extern "C" fn gml_scan_buffer(
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
    b = gmlalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
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
    let ref mut fresh17 = (*b).yy_ch_buf;
    *fresh17 = base;
    let ref mut fresh18 = (*b).yy_buf_pos;
    *fresh18 = *fresh17;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    let ref mut fresh19 = (*b).yy_input_file;
    *fresh19 = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    gml_switch_to_buffer(b);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gml_scan_string(
    mut yystr: *const libc::c_char,
) -> YY_BUFFER_STATE {
    return gml_scan_bytes(yystr, strlen(yystr) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gml_scan_bytes(
    mut yybytes: *const libc::c_char,
    mut _yybytes_len: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    buf = gmlalloc(n) as *mut libc::c_char;
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
    let ref mut fresh20 = *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh20 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh20;
    b = gml_scan_buffer(buf, n);
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
pub unsafe extern "C" fn gmlget_lineno() -> libc::c_int {
    return gmllineno;
}
#[no_mangle]
pub unsafe extern "C" fn gmlget_in() -> *mut FILE {
    return gmlin;
}
#[no_mangle]
pub unsafe extern "C" fn gmlget_out() -> *mut FILE {
    return gmlout;
}
#[no_mangle]
pub unsafe extern "C" fn gmlget_leng() -> libc::c_int {
    return gmlleng;
}
#[no_mangle]
pub unsafe extern "C" fn gmlget_text() -> *mut libc::c_char {
    return gmltext;
}
#[no_mangle]
pub unsafe extern "C" fn gmlset_lineno(mut _line_number: libc::c_int) {
    gmllineno = _line_number;
}
#[no_mangle]
pub unsafe extern "C" fn gmlset_in(mut _in_str: *mut FILE) {
    gmlin = _in_str;
}
#[no_mangle]
pub unsafe extern "C" fn gmlset_out(mut _out_str: *mut FILE) {
    gmlout = _out_str;
}
#[no_mangle]
pub unsafe extern "C" fn gmlget_debug() -> libc::c_int {
    return gml_flex_debug;
}
#[no_mangle]
pub unsafe extern "C" fn gmlset_debug(mut _bdebug: libc::c_int) {
    gml_flex_debug = _bdebug;
}
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as libc::c_int as size_t;
    yy_buffer_stack_max = 0 as libc::c_int as size_t;
    yy_c_buf_p = 0 as *mut libc::c_char;
    yy_init = 0 as libc::c_int;
    yy_start = 0 as libc::c_int;
    gmlin = 0 as *mut FILE;
    gmlout = 0 as *mut FILE;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gmllex_destroy() -> libc::c_int {
    while !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        gml_delete_buffer(
            if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            },
        );
        let ref mut fresh21 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh21 = 0 as YY_BUFFER_STATE;
        gmlpop_buffer_state();
    }
    gmlfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gmlalloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn gmlrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn gmlfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
}
