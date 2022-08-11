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
    fn sfwrite(_: *mut Sfio_t, _: *const libc::c_void, _: size_t) -> ssize_t;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
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
pub type Sfexcept_f = Option<
    unsafe extern "C" fn(*mut Sfio_t, libc::c_int, *mut libc::c_void, *mut Sfdisc_t) -> libc::c_int,
>;
pub type Sfseek_f = Option<
    unsafe extern "C" fn(
        *mut Sfio_t,
        libc::c_longlong,
        libc::c_int,
        *mut Sfdisc_t,
    ) -> libc::c_longlong,
>;
pub type Sfwrite_f = Option<
    unsafe extern "C" fn(*mut Sfio_t, *const libc::c_void, size_t, *mut Sfdisc_t) -> ssize_t,
>;
pub type Sfread_f =
    Option<unsafe extern "C" fn(*mut Sfio_t, *mut libc::c_void, size_t, *mut Sfdisc_t) -> ssize_t>;
#[no_mangle]
pub unsafe extern "C" fn sfnputc(mut f: *mut Sfio_t, mut c: libc::c_int, mut n: size_t) -> ssize_t {
    let mut ps: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: ssize_t = 0;
    let mut w: ssize_t = 0;
    let mut buf: [libc::c_uchar; 128] = [0; 128];
    let mut local: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    if (*f).mode
        & !(0o20 as libc::c_uint
            | 0o10 as libc::c_uint
            | (if local != 0 {
                0o40 as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }))
        != 0o2 as libc::c_int as libc::c_uint
        && _sfmode(f, 0o2 as libc::c_int, local) < 0 as libc::c_int
    {
        return -(1 as libc::c_int) as ssize_t;
    }
    (*f).mode |= 0o40 as libc::c_uint;
    let ref mut fresh0 = (*f).endw;
    *fresh0 = (*f).data;
    let ref mut fresh1 = (*f).endr;
    *fresh1 = *fresh0;
    ps = (*f).next;
    p = ((*f).endb).offset_from(ps) as libc::c_long;
    if (p as size_t) < n {
        ps = buf.as_mut_ptr();
        p = ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong as ssize_t;
    }
    if p as size_t > n {
        p = n as ssize_t;
    }
    memset(ps as *mut libc::c_void, c, p as libc::c_ulong);
    w = n as ssize_t;
    if ps == (*f).next {
        let ref mut fresh2 = (*f).next;
        *fresh2 = (*fresh2).offset(p as isize);
        if c == '\n' as i32 {
            (*f).mode |= 0o100000 as libc::c_uint;
            _sfflsbuf(f, -(1 as libc::c_int));
        }
    } else {
        loop {
            (*f).mode |= 0o100000 as libc::c_uint;
            p = sfwrite(f, ps as *const libc::c_void, p as size_t);
            if p <= 0 as libc::c_int as libc::c_long || {
                n = (n as libc::c_ulong).wrapping_sub(p as libc::c_ulong) as size_t as size_t;
                n <= 0 as libc::c_int as libc::c_ulong
            } {
                w = (w as libc::c_ulong).wrapping_sub(n) as ssize_t as ssize_t;
                break;
            } else if p as size_t > n {
                p = n as ssize_t;
            }
        }
    }
    if local != 0 {
    } else {
        (*f).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh3 = (*f).endr;
            *fresh3 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh4 = (*f).endw;
                *fresh4 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh5 = (*f).endr;
                *fresh5 = (*f).data;
                let ref mut fresh6 = (*f).endw;
                *fresh6 = *fresh5;
            };
        };
    };
    return w;
}
