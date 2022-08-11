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
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntStack_struct {
    pub last: size_t,
    pub max_len: size_t,
    pub stack: *mut libc::c_int,
}
pub type IntStack = *mut IntStack_struct;
#[no_mangle]
pub unsafe extern "C" fn IntStack_new() -> IntStack {
    let mut s: IntStack = 0 as *mut IntStack_struct;
    let mut max_len: size_t = ((1 as libc::c_int) << 5 as libc::c_int) as size_t;
    s = gmalloc(::std::mem::size_of::<IntStack_struct>() as libc::c_ulong) as IntStack;
    (*s).max_len = max_len;
    (*s).last = 18446744073709551615 as libc::c_ulong;
    let ref mut fresh0 = (*s).stack;
    *fresh0 = gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(max_len))
        as *mut libc::c_int;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn IntStack_delete(mut s: IntStack) {
    if !s.is_null() {
        free((*s).stack as *mut libc::c_void);
        free(s as *mut libc::c_void);
    }
}
unsafe extern "C" fn IntStack_realloc(mut s: IntStack) -> IntStack {
    let mut max_len: size_t = (*s).max_len;
    max_len = (max_len as libc::c_ulong).wrapping_add(
        if 10 as libc::c_int as libc::c_ulong
            > max_len.wrapping_div(5 as libc::c_int as libc::c_ulong)
        {
            10 as libc::c_int as libc::c_ulong
        } else {
            max_len.wrapping_div(5 as libc::c_int as libc::c_ulong)
        },
    ) as size_t as size_t;
    (*s).max_len = max_len;
    let ref mut fresh1 = (*s).stack;
    *fresh1 = grealloc(
        (*s).stack as *mut libc::c_void,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(max_len),
    ) as *mut libc::c_int;
    if ((*s).stack).is_null() {
        return 0 as IntStack;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn IntStack_push(mut s: IntStack, mut i: libc::c_int) -> size_t {
    if (*s).last != 18446744073709551615 as libc::c_ulong
        && (*s).last >= ((*s).max_len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if (IntStack_realloc(s)).is_null() {
            return 18446744073709551615 as libc::c_ulong;
        }
    }
    let ref mut fresh2 = (*s).last;
    *fresh2 = (*fresh2).wrapping_add(1);
    *((*s).stack).offset(*fresh2 as isize) = i;
    return (*s).last;
}
#[no_mangle]
pub unsafe extern "C" fn IntStack_pop(mut s: IntStack, mut flag: *mut libc::c_int) -> libc::c_int {
    *flag = 0 as libc::c_int;
    if (*s).last == 18446744073709551615 as libc::c_ulong {
        *flag = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    let ref mut fresh3 = (*s).last;
    let fresh4 = *fresh3;
    *fresh3 = (*fresh3).wrapping_sub(1);
    return *((*s).stack).offset(fresh4 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn IntStack_print(mut s: IntStack) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i <= (*s).last {
        fprintf(
            stderr,
            b"%d,\0" as *const u8 as *const libc::c_char,
            *((*s).stack).offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
