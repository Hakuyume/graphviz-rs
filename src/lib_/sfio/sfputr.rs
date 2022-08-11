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
    fn memccpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn _sfrsrv(_: *mut Sfio_t, _: ssize_t) -> *mut Sfrsrv_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type Sfrsrv_t = _sfrsrv_s;
#[no_mangle]
pub unsafe extern "C" fn sfputr(
    mut f: *mut Sfio_t,
    mut s: *const libc::c_char,
    mut rc: libc::c_int,
) -> ssize_t {
    let mut p: ssize_t = 0;
    let mut n: ssize_t = 0;
    let mut w: ssize_t = 0;
    let mut ps: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if f.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    if (*f).mode != 0o2 as libc::c_int as libc::c_uint
        && _sfmode(f, 0o2 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
    {
        return -(1 as libc::c_int) as ssize_t;
    }
    (*f).mode |= 0o40 as libc::c_uint;
    let ref mut fresh0 = (*f).endw;
    *fresh0 = (*f).data;
    let ref mut fresh1 = (*f).endr;
    *fresh1 = *fresh0;
    w = 0 as libc::c_int as ssize_t;
    while *s as libc::c_int != 0 || rc >= 0 as libc::c_int {
        ps = (*f).next;
        p = ((*f).endb).offset_from(ps) as libc::c_long;
        if p > 0 as libc::c_int as libc::c_long {
        } else {
            (*f).mode |= 0o100000 as libc::c_uint;
            p = _sfflsbuf(f, -(1 as libc::c_int)) as ssize_t;
            ps = (*f).next;
        };
        if p == 0 as libc::c_int as libc::c_long
            || (*f).flags as libc::c_int & 0o20000 as libc::c_int != 0
        {
            n = strlen(s) as ssize_t;
            if p >= n
                + (if rc < 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as libc::c_long
            {
                if n > 0 as libc::c_int as libc::c_long {
                    memcpy(
                        ps as *mut libc::c_void,
                        s as *const libc::c_void,
                        n as libc::c_ulong,
                    );
                    ps = ps.offset(n as isize);
                    w += n;
                }
                if rc >= 0 as libc::c_int {
                    let fresh2 = ps;
                    ps = ps.offset(1);
                    *fresh2 = rc as libc::c_uchar;
                    w += 1 as libc::c_int as libc::c_long;
                }
                let ref mut fresh3 = (*f).next;
                *fresh3 = ps;
            } else {
                let mut rsrv: *mut Sfrsrv_t = 0 as *mut Sfrsrv_t;
                p = n
                    + (if rc >= 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_long;
                rsrv = _sfrsrv(f, p);
                if rsrv.is_null() {
                    n = 0 as libc::c_int as ssize_t;
                } else {
                    if n > 0 as libc::c_int as libc::c_long {
                        memcpy(
                            ((*rsrv).data).as_mut_ptr() as *mut libc::c_void,
                            s as *const libc::c_void,
                            n as libc::c_ulong,
                        );
                    }
                    if rc >= 0 as libc::c_int {
                        *((*rsrv).data).as_mut_ptr().offset(n as isize) = rc as libc::c_uchar;
                    }
                    (*f).mode |= 0o100000 as libc::c_uint;
                    n = sfwrite(
                        f,
                        ((*rsrv).data).as_mut_ptr() as *const libc::c_void,
                        p as size_t,
                    );
                    if n < 0 as libc::c_int as libc::c_long {
                        n = 0 as libc::c_int as ssize_t;
                    }
                }
                w += n;
            }
            break;
        } else if *s as libc::c_int == 0 as libc::c_int {
            let fresh4 = ps;
            ps = ps.offset(1);
            *fresh4 = rc as libc::c_uchar;
            let ref mut fresh5 = (*f).next;
            *fresh5 = ps;
            w += 1 as libc::c_int as libc::c_long;
            break;
        } else {
            ps = memccpy(
                ps as *mut libc::c_void,
                s as *const libc::c_void,
                '\0' as i32,
                p as libc::c_ulong,
            ) as *mut libc::c_uchar;
            if !ps.is_null() {
                ps = ps.offset(-(1 as libc::c_int as isize));
            } else {
                ps = ((*f).next).offset(p as isize);
            }
            s = s.offset(ps.offset_from((*f).next) as libc::c_long as isize);
            w += ps.offset_from((*f).next) as libc::c_long;
            let ref mut fresh6 = (*f).next;
            *fresh6 = ps;
        }
    }
    if (*f).extent < 0 as libc::c_int as libc::c_longlong
        && (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
    {
        (*f).mode |= 0o100000 as libc::c_uint;
        _sfflsbuf(f, -(1 as libc::c_int));
    } else if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0
        && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
        && {
            n = ((*f).next).offset_from((*f).data) as libc::c_long;
            n > 0 as libc::c_int as libc::c_long
        }
    {
        if n > w {
            n = w;
        }
        let ref mut fresh7 = (*f).next;
        *fresh7 = (*fresh7).offset(-(n as isize));
        (*f).mode |= 0o100000 as libc::c_uint;
        sfwrite(f, (*f).next as *const libc::c_void, n as size_t);
    }
    if 0 as libc::c_int != 0 {
    } else {
        (*f).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh8 = (*f).endr;
            *fresh8 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh9 = (*f).endw;
                *fresh9 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh10 = (*f).endr;
                *fresh10 = (*f).data;
                let ref mut fresh11 = (*f).endw;
                *fresh11 = *fresh10;
            };
        };
    };
    return w;
}
