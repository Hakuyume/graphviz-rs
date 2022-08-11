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
    fn vmstrdup(_: *mut Vmalloc_t, _: *const libc::c_char) -> *mut libc::c_char;
    fn exnospace() -> *mut libc::c_char;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfio_s {
    pub next: *mut libc::c_uchar,
    pub endw: *mut libc::c_uchar,
    pub endr: *mut libc::c_uchar,
    pub endb: *mut libc::c_uchar,
    pub push: *mut Sfio_t,
    pub flags: libc::c_ushort,
    pub file: libc::c_short,
    pub data: *mut libc::c_uchar,
    pub size: ssize_t,
    pub val: ssize_t,
}
pub type Sfio_t = _sfio_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vmalloc_s {
    pub allocated: *mut *mut libc::c_void,
    pub size: size_t,
    pub capacity: size_t,
}
pub type Vmalloc_t = _vmalloc_s;
#[no_mangle]
pub unsafe extern "C" fn exstash(mut sp: *mut Sfio_t, mut vp: *mut Vmalloc_t) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*sp).next >= (*sp).endw {
        _sfflsbuf(sp, 0 as libc::c_int as libc::c_uchar as libc::c_int);
    } else {
        let ref mut fresh0 = (*sp).next;
        let fresh1 = *fresh0;
        *fresh0 = (*fresh0).offset(1);
        *fresh1 = 0 as libc::c_int as libc::c_uchar;
    };
    let ref mut fresh2 = (*sp).next;
    *fresh2 = (*sp).data;
    s = *fresh2 as *mut libc::c_char;
    return if !s.is_null()
        && (vp.is_null() || {
            s = vmstrdup(vp, s);
            !s.is_null()
        }) {
        s
    } else {
        exnospace()
    };
}
