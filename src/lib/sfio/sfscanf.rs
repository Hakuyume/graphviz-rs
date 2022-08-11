#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
extern "C" {
    fn sfvscanf(
        _: *mut Sfio_t,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
    pub extent: libc::c_longlong,
    pub here: libc::c_longlong,
    pub getr: libc::c_uchar,
    pub tiny: [libc::c_uchar; 1],
    pub bits: libc::c_ushort,
    pub mode: libc::c_uint,
    pub disc: *mut _sfdisc_s,
    pub pool: *mut _sfpool_s,
    pub rsrv: *mut _sfrsrv_s,
    pub proc_0: *mut _sfproc_s,
    pub stdio: *mut libc::c_void,
    pub lpos: libc::c_longlong,
    pub iosz: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfproc_s {
    pub pid: libc::c_int,
    pub rdata: *mut libc::c_uchar,
    pub ndata: libc::c_int,
    pub size: libc::c_int,
    pub file: libc::c_int,
    pub sigp: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfrsrv_s {
    pub slen: ssize_t,
    pub size: ssize_t,
    pub data: [libc::c_uchar; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfpool_s {
    pub next: *mut Sfpool_t,
    pub mode: libc::c_int,
    pub s_sf: libc::c_int,
    pub n_sf: libc::c_int,
    pub sf: *mut *mut Sfio_t,
    pub array: [*mut Sfio_t; 3],
}
pub type Sfio_t = _sfio_s;
pub type Sfpool_t = _sfpool_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfdisc_s {
    pub readf: Sfread_f,
    pub writef: Sfwrite_f,
    pub seekf: Sfseek_f,
    pub exceptf: Sfexcept_f,
    pub disc: *mut Sfdisc_t,
}
pub type Sfdisc_t = _sfdisc_s;
pub type Sfexcept_f = Option::<
    unsafe extern "C" fn(
        *mut Sfio_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Sfdisc_t,
    ) -> libc::c_int,
>;
pub type Sfseek_f = Option::<
    unsafe extern "C" fn(
        *mut Sfio_t,
        libc::c_longlong,
        libc::c_int,
        *mut Sfdisc_t,
    ) -> libc::c_longlong,
>;
pub type Sfwrite_f = Option::<
    unsafe extern "C" fn(
        *mut Sfio_t,
        *const libc::c_void,
        size_t,
        *mut Sfdisc_t,
    ) -> ssize_t,
>;
pub type Sfread_f = Option::<
    unsafe extern "C" fn(
        *mut Sfio_t,
        *mut libc::c_void,
        size_t,
        *mut Sfdisc_t,
    ) -> ssize_t,
>;
#[no_mangle]
pub unsafe extern "C" fn sfscanf(
    mut f: *mut Sfio_t,
    mut form: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut rv: libc::c_int = 0;
    args_0 = args.clone();
    rv = if !f.is_null() && !form.is_null() {
        sfvscanf(f, form, args_0.as_va_list())
    } else {
        -(1 as libc::c_int)
    };
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn sfvsscanf(
    mut s: *const libc::c_char,
    mut form: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> libc::c_int {
    let mut f: Sfio_t = Sfio_t {
        next: 0 as *mut libc::c_uchar,
        endw: 0 as *mut libc::c_uchar,
        endr: 0 as *mut libc::c_uchar,
        endb: 0 as *mut libc::c_uchar,
        push: 0 as *mut Sfio_t,
        flags: 0,
        file: 0,
        data: 0 as *mut libc::c_uchar,
        size: 0,
        val: 0,
        extent: 0,
        here: 0,
        getr: 0,
        tiny: [0; 1],
        bits: 0,
        mode: 0,
        disc: 0 as *mut _sfdisc_s,
        pool: 0 as *mut _sfpool_s,
        rsrv: 0 as *mut _sfrsrv_s,
        proc_0: 0 as *mut _sfproc_s,
        stdio: 0 as *mut libc::c_void,
        lpos: 0,
        iosz: 0,
    };
    if s.is_null() || form.is_null() {
        return -(1 as libc::c_int);
    }
    f.next = 0 as *mut libc::c_uchar;
    f.endw = 0 as *mut libc::c_uchar;
    f.endr = 0 as *mut libc::c_uchar;
    f.endb = 0 as *mut libc::c_uchar;
    f.push = 0 as *mut Sfio_t;
    f.flags = 0 as libc::c_int as libc::c_ushort;
    f.file = -(1 as libc::c_int) as libc::c_short;
    f.data = 0 as *mut libc::c_uchar;
    f.size = -(1 as libc::c_int) as ssize_t;
    f.val = -(1 as libc::c_int) as ssize_t;
    f.extent = -(1 as libc::c_int) as libc::c_longlong;
    f.here = 0 as libc::c_int as libc::c_longlong;
    f.getr = 0 as libc::c_int as libc::c_uchar;
    f.tiny[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    f.bits = 0 as libc::c_int as libc::c_ushort;
    f.mode = 0 as libc::c_int as libc::c_uint;
    f.disc = 0 as *mut _sfdisc_s;
    f.pool = 0 as *mut _sfpool_s;
    f.rsrv = 0 as *mut _sfrsrv_s;
    f.proc_0 = 0 as *mut _sfproc_s;
    f.stdio = 0 as *mut libc::c_void;
    f.lpos = 0 as libc::c_int as libc::c_longlong;
    f.iosz = 0 as libc::c_int as size_t;
    f.flags = (0o4 as libc::c_int | 0o1 as libc::c_int) as libc::c_ushort;
    f.mode = 0o1 as libc::c_int as libc::c_uint;
    f.size = strlen(s) as ssize_t;
    f.endw = s as *mut libc::c_uchar;
    f.next = f.endw;
    f.data = f.next;
    f.endr = (f.data).offset(f.size as isize);
    f.endb = f.endr;
    return sfvscanf(&mut f, form, args.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn sfsscanf(
    mut s: *const libc::c_char,
    mut form: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut rv: libc::c_int = 0;
    args_0 = args.clone();
    rv = if !s.is_null() && !form.is_null() {
        sfvsscanf(s, form, args_0.as_va_list())
    } else {
        -(1 as libc::c_int)
    };
    return rv;
}
