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
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn tmpfile() -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
pub type agusererrf = Option<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>;
static mut agerrno: agerrlevel_t = AGWARN;
static mut agerrlevel: agerrlevel_t = AGWARN;
static mut agmaxerr: libc::c_int = 0;
static mut aglast: libc::c_long = 0;
static mut agerrout: *mut FILE = 0 as *const FILE as *mut FILE;
static mut usererrf: agusererrf = None;
#[no_mangle]
pub unsafe extern "C" fn agseterrf(mut newf: agusererrf) -> agusererrf {
    let mut oldf: agusererrf = usererrf;
    usererrf = newf;
    return oldf;
}
#[no_mangle]
pub unsafe extern "C" fn agseterr(mut lvl: agerrlevel_t) -> agerrlevel_t {
    let mut oldv: agerrlevel_t = agerrlevel;
    agerrlevel = lvl;
    return oldv;
}
#[no_mangle]
pub unsafe extern "C" fn aglasterr() -> *mut libc::c_char {
    if agerrout.is_null() {
        return 0 as *mut libc::c_char;
    }
    fflush(agerrout);
    let mut endpos: libc::c_long = ftell(agerrout);
    let mut len: size_t = (endpos - aglast) as size_t;
    let mut buf: *mut libc::c_char =
        malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    fseek(agerrout, aglast, 0 as libc::c_int);
    len = fread(
        buf as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        len,
        agerrout,
    );
    *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    fseek(agerrout, endpos, 0 as libc::c_int);
    return buf;
}
unsafe extern "C" fn userout(
    mut level: agerrlevel_t,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut bufsz: size_t = 0;
    let mut args2: ::std::ffi::VaListImpl;
    args2 = args.clone();
    let mut rc: libc::c_int = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        args2.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s: vsnprintf failure\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"userout\0")).as_ptr(),
        );
        return;
    }
    bufsz = (rc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut buf: *mut libc::c_char = malloc(bufsz) as *mut libc::c_char;
    if buf.is_null() {
        fprintf(
            stderr,
            b"%s: could not allocate memory\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"userout\0")).as_ptr(),
        );
        return;
    }
    if level as libc::c_uint != AGPREV as libc::c_int as libc::c_uint {
        usererrf.expect("non-null function pointer")(
            (if level as libc::c_uint == AGERR as libc::c_int as libc::c_uint {
                b"Error\0" as *const u8 as *const libc::c_char
            } else {
                b"Warning\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
        );
        usererrf.expect("non-null function pointer")(
            b": \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    let mut rc_0: libc::c_int = vsnprintf(buf, bufsz, fmt, args.as_va_list());
    if rc_0 < 0 as libc::c_int {
        free(buf as *mut libc::c_void);
        fprintf(
            stderr,
            b"%s: vsnprintf failure\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"userout\0")).as_ptr(),
        );
        return;
    }
    usererrf.expect("non-null function pointer")(buf);
    free(buf as *mut libc::c_void);
}
unsafe extern "C" fn agerr_va(
    mut level: agerrlevel_t,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> libc::c_int {
    let mut lvl: agerrlevel_t = AGWARN;
    lvl = (if level as libc::c_uint == AGPREV as libc::c_int as libc::c_uint {
        agerrno as libc::c_uint
    } else if level as libc::c_uint == AGMAX as libc::c_int as libc::c_uint {
        AGERR as libc::c_int as libc::c_uint
    } else {
        level as libc::c_uint
    }) as agerrlevel_t;
    agerrno = lvl;
    agmaxerr = if agmaxerr > agerrno as libc::c_int {
        agmaxerr
    } else {
        agerrno as libc::c_int
    };
    if lvl as libc::c_uint >= agerrlevel as libc::c_uint {
        if usererrf.is_some() {
            userout(level, fmt, args.as_va_list());
        } else {
            if level as libc::c_uint != AGPREV as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"%s: \0" as *const u8 as *const libc::c_char,
                    if level as libc::c_uint == AGERR as libc::c_int as libc::c_uint {
                        b"Error\0" as *const u8 as *const libc::c_char
                    } else {
                        b"Warning\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            vfprintf(stderr, fmt, args.as_va_list());
        }
        return 0 as libc::c_int;
    }
    if agerrout.is_null() {
        agerrout = tmpfile();
        if agerrout.is_null() {
            return 1 as libc::c_int;
        }
    }
    if level as libc::c_uint != AGPREV as libc::c_int as libc::c_uint {
        aglast = ftell(agerrout);
    }
    vfprintf(agerrout, fmt, args.as_va_list());
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn agerr(
    mut level: agerrlevel_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut ret: libc::c_int = 0;
    args_0 = args.clone();
    ret = agerr_va(level, fmt, args_0.as_va_list());
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn agerrorf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    agerr_va(AGERR, fmt, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn agwarningf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    agerr_va(AGWARN, fmt, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn agerrors() -> libc::c_int {
    return agmaxerr;
}
#[no_mangle]
pub unsafe extern "C" fn agreseterrors() -> libc::c_int {
    let mut rc: libc::c_int = agmaxerr;
    agmaxerr = 0 as libc::c_int;
    return rc;
}
