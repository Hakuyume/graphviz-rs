#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    static mut _Sfextern: Sfextern_t;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
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
pub type Sfextern_t = _sfextern_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfextern_s {
    pub sf_page: ssize_t,
    pub sf_pool: _sfpool_s,
    pub sf_pmove: Option::<
        unsafe extern "C" fn(*mut Sfio_t, libc::c_int) -> libc::c_int,
    >,
    pub sf_stack: Option::<
        unsafe extern "C" fn(*mut Sfio_t, *mut Sfio_t) -> *mut Sfio_t,
    >,
    pub sf_notify: Option::<
        unsafe extern "C" fn(*mut Sfio_t, libc::c_int, libc::c_int) -> (),
    >,
    pub sf_stdsync: Option::<unsafe extern "C" fn(*mut Sfio_t) -> libc::c_int>,
    pub sf_udisc: _sfdisc_s,
    pub sf_cleanup: Option::<unsafe extern "C" fn() -> ()>,
    pub sf_exiting: libc::c_int,
    pub sf_done: libc::c_int,
}
unsafe extern "C" fn _sfdup(mut fd: libc::c_int, mut newfd: libc::c_int) -> libc::c_int {
    let mut dupfd: libc::c_int = 0;
    loop {
        dupfd = fcntl(fd, 0 as libc::c_int, newfd);
        if !(dupfd < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
        *__errno_location() = 0 as libc::c_int;
    }
    return dupfd;
}
#[no_mangle]
pub unsafe extern "C" fn sfsetfd(
    mut f: *mut Sfio_t,
    mut newfd: libc::c_int,
) -> libc::c_int {
    let mut oldfd: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    if (*f).mode & 0o4 as libc::c_uint != 0
        && ((*f).file as libc::c_int) < 0 as libc::c_int
    {
        if newfd < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        if (*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
            != (*f).mode
            && _sfmode(f, 0 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        (*f).mode |= 0o40 as libc::c_uint;
        let ref mut fresh0 = (*f).endw;
        *fresh0 = (*f).data;
        let ref mut fresh1 = (*f).endr;
        *fresh1 = *fresh0;
        oldfd = (*f).file as libc::c_int;
        if oldfd >= 0 as libc::c_int {
            if newfd >= 0 as libc::c_int {
                newfd = _sfdup(oldfd, newfd);
                if newfd < 0 as libc::c_int {
                    if 0 as libc::c_int != 0 {} else {
                        (*f).mode
                            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint
                                | 0o20 as libc::c_uint);
                        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                            let ref mut fresh2 = (*f).endr;
                            *fresh2 = (*f).endb;
                        } else {
                            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                                let ref mut fresh3 = (*f).endw;
                                *fresh3 = (if (*f).flags as libc::c_int
                                    & 0o40 as libc::c_int != 0
                                {
                                    (*f).data
                                } else {
                                    (*f).endb
                                });
                            } else {
                                let ref mut fresh4 = (*f).endr;
                                *fresh4 = (*f).data;
                                let ref mut fresh5 = (*f).endw;
                                *fresh5 = *fresh4;
                            };
                        };
                    };
                    return -(1 as libc::c_int);
                }
                while close(oldfd) < 0 as libc::c_int
                    && *__errno_location() == 4 as libc::c_int
                {
                    *__errno_location() = 0 as libc::c_int;
                }
            } else {
                if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
                    && (*f).next > (*f).data
                    || (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
                    || (*f).disc == &mut _Sfextern.sf_udisc as *mut _sfdisc_s
                {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    if sfsync(f) < 0 as libc::c_int {
                        if 0 as libc::c_int != 0 {} else {
                            (*f).mode
                                &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint
                                    | 0o20 as libc::c_uint);
                            if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                                let ref mut fresh6 = (*f).endr;
                                *fresh6 = (*f).endb;
                            } else {
                                if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                                    let ref mut fresh7 = (*f).endw;
                                    *fresh7 = (if (*f).flags as libc::c_int
                                        & 0o40 as libc::c_int != 0
                                    {
                                        (*f).data
                                    } else {
                                        (*f).endb
                                    });
                                } else {
                                    let ref mut fresh8 = (*f).endr;
                                    *fresh8 = (*f).data;
                                    let ref mut fresh9 = (*f).endw;
                                    *fresh9 = *fresh8;
                                };
                            };
                        };
                        return -(1 as libc::c_int);
                    }
                }
                if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
                    && (*f).next > (*f).data
                    || (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
                        && (*f).extent < 0 as libc::c_int as libc::c_longlong
                        && (*f).next < (*f).endb
                {
                    if 0 as libc::c_int != 0 {} else {
                        (*f).mode
                            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint
                                | 0o20 as libc::c_uint);
                        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                            let ref mut fresh10 = (*f).endr;
                            *fresh10 = (*f).endb;
                        } else {
                            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                                let ref mut fresh11 = (*f).endw;
                                *fresh11 = (if (*f).flags as libc::c_int
                                    & 0o40 as libc::c_int != 0
                                {
                                    (*f).data
                                } else {
                                    (*f).endb
                                });
                            } else {
                                let ref mut fresh12 = (*f).endr;
                                *fresh12 = (*f).data;
                                let ref mut fresh13 = (*f).endw;
                                *fresh13 = *fresh12;
                            };
                        };
                    };
                    return -(1 as libc::c_int);
                }
                let ref mut fresh14 = (*f).endw;
                *fresh14 = (*f).data;
                let ref mut fresh15 = (*f).endr;
                *fresh15 = *fresh14;
                let ref mut fresh16 = (*f).endb;
                *fresh16 = *fresh15;
                let ref mut fresh17 = (*f).here;
                *fresh17 = 0 as libc::c_int as libc::c_longlong;
                (*f).extent = *fresh17;
                (*f)
                    .mode = (*f).mode
                    & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
                    | 0o4 as libc::c_uint;
                let ref mut fresh18 = (*f).bits;
                *fresh18 = (*fresh18 as libc::c_int
                    & !(0o10 as libc::c_int) as libc::c_ushort as libc::c_int)
                    as libc::c_ushort;
            }
        }
        if 0 as libc::c_int != 0 {} else {
            (*f).mode
                &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
            if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                let ref mut fresh19 = (*f).endr;
                *fresh19 = (*f).endb;
            } else {
                if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                    let ref mut fresh20 = (*f).endw;
                    *fresh20 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                        (*f).data
                    } else {
                        (*f).endb
                    });
                } else {
                    let ref mut fresh21 = (*f).endr;
                    *fresh21 = (*f).data;
                    let ref mut fresh22 = (*f).endw;
                    *fresh22 = *fresh21;
                };
            };
        };
    }
    if (_Sfextern.sf_notify).is_some() {
        (_Sfextern.sf_notify)
            .expect("non-null function pointer")(f, -(1 as libc::c_int), newfd);
    }
    (*f).file = newfd as libc::c_short;
    return newfd;
}
