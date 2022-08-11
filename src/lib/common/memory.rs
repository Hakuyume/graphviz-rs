#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
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
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc(mut nbytes: size_t) -> *mut libc::c_void {
    if nbytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    return gcalloc(1 as libc::c_int as size_t, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn zrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut elt: size_t,
    mut osize: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, size.wrapping_mul(elt));
    if (p.is_null() && size != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    if osize < size {
        memset(
            (p as *mut libc::c_char).offset(osize.wrapping_mul(elt) as isize)
                as *mut libc::c_void,
            '\0' as i32,
            size.wrapping_sub(osize).wrapping_mul(elt),
        );
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn gcalloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut rv: *mut libc::c_char = calloc(nmemb, size) as *mut libc::c_char;
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong && rv.is_null()) as libc::c_int
        as libc::c_long != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return rv as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn gmalloc(mut nbytes: size_t) -> *mut libc::c_void {
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    if nbytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    rv = malloc(nbytes) as *mut libc::c_char;
    if (rv == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return rv as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn grealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, size);
    if (p.is_null() && size != 0) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
