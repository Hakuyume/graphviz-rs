#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Error_info_s {
    pub errors: libc::c_int,
    pub indent: libc::c_int,
    pub line: libc::c_int,
    pub warnings: libc::c_int,
    pub trace: libc::c_int,
    pub file: *mut libc::c_char,
    pub id: *mut libc::c_char,
}
pub type Error_info_t = Error_info_s;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[no_mangle]
pub static mut _err_info: Error_info_t = Error_info_t {
    errors: 0,
    indent: 0,
    line: 0,
    warnings: 0,
    trace: 0,
    file: 0 as *const libc::c_char as *mut libc::c_char,
    id: 0 as *const libc::c_char as *mut libc::c_char,
};
#[no_mangle]
pub unsafe extern "C" fn setErrorLine(mut line: libc::c_int) {
    _err_info.line = line;
}
#[no_mangle]
pub unsafe extern "C" fn setErrorFileLine(
    mut src: *mut libc::c_char,
    mut line: libc::c_int,
) {
    _err_info.file = src;
    _err_info.line = line;
}
#[no_mangle]
pub unsafe extern "C" fn setErrorId(mut id: *mut libc::c_char) {
    _err_info.id = id;
}
#[no_mangle]
pub unsafe extern "C" fn setErrorErrors(mut errors: libc::c_int) {
    _err_info.errors = errors;
}
#[no_mangle]
pub unsafe extern "C" fn getErrorErrors() -> libc::c_int {
    return _err_info.errors;
}
#[no_mangle]
pub unsafe extern "C" fn setTraceLevel(mut i: libc::c_int) {
    _err_info.trace = i;
}
#[no_mangle]
pub unsafe extern "C" fn _err_msgv(
    mut id: *const libc::c_char,
    mut level: libc::c_int,
    mut s: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut flags: libc::c_int = 0;
    if level < _err_info.trace {
        return;
    }
    if level < 0 as libc::c_int {
        flags = 0 as libc::c_int;
    } else {
        flags = level & !(0xff as libc::c_int);
        level &= 0xff as libc::c_int;
    }
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    if level != 0
        && {
            prefix = _err_info.id;
            !prefix.is_null()
                || {
                    prefix = id;
                    !prefix.is_null()
                }
        }
    {
        if flags & 0x800 as libc::c_int != 0 {
            fprintf(stderr, b"Usage: %s \0" as *const u8 as *const libc::c_char, prefix);
        } else {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, prefix);
        }
    }
    if !(flags & 0x800 as libc::c_int != 0) {
        if level < 0 as libc::c_int {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < _err_info.indent {
                fprintf(stderr, b"  \0" as *const u8 as *const libc::c_char);
                i += 1;
            }
            fprintf(stderr, b"debug%d: \0" as *const u8 as *const libc::c_char, level);
        } else if level != 0 {
            if level == 1 as libc::c_int {
                fprintf(stderr, b"warning: \0" as *const u8 as *const libc::c_char);
                _err_info.warnings += 1;
            } else {
                _err_info.errors += 1;
                if level == 0xff as libc::c_int {
                    fprintf(stderr, b"panic: \0" as *const u8 as *const libc::c_char);
                }
            }
            if _err_info.line != 0 {
                if !(_err_info.file).is_null() && *_err_info.file as libc::c_int != 0 {
                    fprintf(
                        stderr,
                        b"\"%s\", \0" as *const u8 as *const libc::c_char,
                        _err_info.file,
                    );
                }
                fprintf(
                    stderr,
                    b"line %d: \0" as *const u8 as *const libc::c_char,
                    _err_info.line,
                );
            }
        }
    }
    vfprintf(stderr, s, ap.as_va_list());
    if flags & 0x100 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"\n%s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    if level >= 3 as libc::c_int {
        graphviz_exit(level - 3 as libc::c_int + 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _err_msg(
    mut level: libc::c_int,
    mut s: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    _err_msgv(0 as *const libc::c_char, level, s, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn errorf(
    mut handle: *mut libc::c_void,
    mut discipline: *mut libc::c_void,
    mut level: libc::c_int,
    mut s: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    _err_msgv(
        if !discipline.is_null() && !handle.is_null() {
            *(handle as *mut *mut libc::c_char)
        } else {
            handle as *mut libc::c_char
        },
        level,
        s,
        ap.as_va_list(),
    );
}
