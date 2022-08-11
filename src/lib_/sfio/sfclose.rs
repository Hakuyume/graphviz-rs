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
#![feature(label_break_value, register_tool)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    fn sfraise(_: *mut Sfio_t, _: libc::c_int, _: *mut libc::c_void) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn _sfpclose(_: *mut Sfio_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut _Sfextern: Sfextern_t;
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
pub type Sfextern_t = _sfextern_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sfextern_s {
    pub sf_page: ssize_t,
    pub sf_pool: _sfpool_s,
    pub sf_pmove: Option<unsafe extern "C" fn(*mut Sfio_t, libc::c_int) -> libc::c_int>,
    pub sf_stack: Option<unsafe extern "C" fn(*mut Sfio_t, *mut Sfio_t) -> *mut Sfio_t>,
    pub sf_notify: Option<unsafe extern "C" fn(*mut Sfio_t, libc::c_int, libc::c_int) -> ()>,
    pub sf_stdsync: Option<unsafe extern "C" fn(*mut Sfio_t) -> libc::c_int>,
    pub sf_udisc: _sfdisc_s,
    pub sf_cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub sf_exiting: libc::c_int,
    pub sf_done: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn sfclose(mut f: *mut Sfio_t) -> libc::c_int {
    let mut local: libc::c_int = 0;
    let mut ex: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    if (*f).mode & 0o4 as libc::c_uint == 0
        && (*f).mode
            & !(0o20 as libc::c_uint
                | 0o10 as libc::c_uint
                | (if local != 0 {
                    0o40 as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }))
            != (*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
        && (*f).mode
            & !(0o20 as libc::c_uint
                | 0o10 as libc::c_uint
                | (if local != 0 {
                    0o40 as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }))
            != (*f).mode & (0o1 as libc::c_int as libc::c_uint | 0o4000 as libc::c_uint)
        && _sfmode(f, 0 as libc::c_int, local) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    while !((*f).push).is_null() {
        let mut pop: *mut Sfio_t = 0 as *mut Sfio_t;
        pop = (_Sfextern.sf_stack).expect("non-null function pointer")(f, 0 as *mut Sfio_t);
        if pop.is_null() {
            return -(1 as libc::c_int);
        }
        if sfclose(pop) < 0 as libc::c_int {
            (_Sfextern.sf_stack).expect("non-null function pointer")(f, pop);
            return -(1 as libc::c_int);
        }
    }
    rv = 0 as libc::c_int;
    if (*f).disc == &mut _Sfextern.sf_udisc as *mut _sfdisc_s {
        let ref mut fresh0 = (*f).disc;
        *fresh0 = 0 as *mut _sfdisc_s;
    } else if (*f).file as libc::c_int >= 0 as libc::c_int {
        let ref mut fresh1 = (*f).bits;
        *fresh1 = (*fresh1 as libc::c_int | 0o400 as libc::c_int) as libc::c_ushort;
        rv = sfsync(f);
    }
    (*f).mode |= 0o40 as libc::c_uint;
    let ref mut fresh2 = (*f).endw;
    *fresh2 = (*f).data;
    let ref mut fresh3 = (*f).endr;
    *fresh3 = *fresh2;
    if !((*f).disc).is_null() && {
        (*f).mode |= 0o100000 as libc::c_uint;
        ex = sfraise(
            f,
            (if local != 0 {
                0 as libc::c_int
            } else {
                4 as libc::c_int
            }),
            0 as *mut libc::c_void,
        );
        ex != 0 as libc::c_int
    } {
        return ex;
    }
    if local == 0 && !((*f).pool).is_null() {
        if (*f).pool == &mut _Sfextern.sf_pool as *mut _sfpool_s {
            let mut n: libc::c_int = 0;
            n = 0 as libc::c_int;
            while n < _Sfextern.sf_pool.n_sf {
                if *(_Sfextern.sf_pool.sf).offset(n as isize) != f {
                    n += 1;
                } else {
                    _Sfextern.sf_pool.n_sf -= 1 as libc::c_int;
                    while n < _Sfextern.sf_pool.n_sf {
                        let ref mut fresh4 = *(_Sfextern.sf_pool.sf).offset(n as isize);
                        *fresh4 = *(_Sfextern.sf_pool.sf).offset((n + 1 as libc::c_int) as isize);
                        n += 1;
                    }
                    break;
                }
            }
        } else {
            (*f).mode &= !(0o40 as libc::c_uint);
            if (_Sfextern.sf_pmove).is_some() {
            } else {
                __assert_fail(
                    b"_Sfpmove\0" as *const u8 as *const libc::c_char,
                    b"sfclose.c\0" as *const u8 as *const libc::c_char,
                    78 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"int sfclose(Sfio_t *)\0",
                    ))
                    .as_ptr(),
                );
            }
            if (_Sfextern.sf_pmove).expect("non-null function pointer")(f, -(1 as libc::c_int))
                < 0 as libc::c_int
            {
                if 0 as libc::c_int != 0 {
                } else {
                    (*f).mode &=
                        !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
                    if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                        let ref mut fresh5 = (*f).endr;
                        *fresh5 = (*f).endb;
                    } else {
                        if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                            let ref mut fresh6 = (*f).endw;
                            *fresh6 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                                (*f).data
                            } else {
                                (*f).endb
                            });
                        } else {
                            let ref mut fresh7 = (*f).endr;
                            *fresh7 = (*f).data;
                            let ref mut fresh8 = (*f).endw;
                            *fresh8 = *fresh7;
                        };
                    };
                };
                return -(1 as libc::c_int);
            }
            (*f).mode |= 0o40 as libc::c_uint;
        }
        let ref mut fresh9 = (*f).pool;
        *fresh9 = 0 as *mut _sfpool_s;
    }
    if !((*f).data).is_null()
        && (local == 0
            || (*f).flags as libc::c_int & 0o4 as libc::c_int != 0
            || (*f).bits as libc::c_int & 0o1 as libc::c_int != 0)
    {
        if (*f).flags as libc::c_int & 0o20 as libc::c_int != 0 {
            data = (*f).data as *mut libc::c_void;
        }
        let ref mut fresh10 = (*f).data;
        *fresh10 = 0 as *mut libc::c_uchar;
        (*f).size = -(1 as libc::c_int) as ssize_t;
    }
    if (_Sfextern.sf_notify).is_some() {
        (Some((_Sfextern.sf_notify).expect("non-null function pointer")))
            .expect("non-null function pointer")(
            f, 4 as libc::c_int, (*f).file as libc::c_int
        );
    }
    if (*f).file as libc::c_int >= 0 as libc::c_int
        && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
    {
        while close((*f).file as libc::c_int) < 0 as libc::c_int
            && *__errno_location() == 4 as libc::c_int
        {
            *__errno_location() = 0 as libc::c_int;
        }
    }
    (*f).file = -(1 as libc::c_int) as libc::c_short;
    (*f).mode = 0o20000 as libc::c_uint | 0o40 as libc::c_uint;
    let ref mut fresh11 = (*f).flags;
    *fresh11 = (*fresh11 as libc::c_int & 0o1000 as libc::c_int) as libc::c_ushort;
    (*f).here = 0 as libc::c_int as libc::c_longlong;
    (*f).extent = -(1 as libc::c_int) as libc::c_longlong;
    let ref mut fresh12 = (*f).next;
    *fresh12 = (*f).data;
    let ref mut fresh13 = (*f).endw;
    *fresh13 = *fresh12;
    let ref mut fresh14 = (*f).endr;
    *fresh14 = *fresh13;
    let ref mut fresh15 = (*f).endb;
    *fresh15 = *fresh14;
    free((*f).rsrv as *mut libc::c_void);
    let ref mut fresh16 = (*f).rsrv;
    *fresh16 = 0 as *mut _sfrsrv_s;
    if !((*f).proc_0).is_null() {
        rv = _sfpclose(f);
    }
    if local == 0 {
        if !((*f).disc).is_null() && {
            (*f).mode |= 0o100000 as libc::c_uint;
            ex = sfraise(f, 11 as libc::c_int, 0 as *mut libc::c_void);
            ex != 0 as libc::c_int
        } {
            rv = ex;
        } else if (*f).flags as libc::c_int & 0o1000 as libc::c_int == 0 {
            free(f as *mut libc::c_void);
        } else {
            let ref mut fresh17 = (*f).disc;
            *fresh17 = 0 as *mut _sfdisc_s;
            let ref mut fresh18 = (*f).stdio;
            *fresh18 = 0 as *mut libc::c_void;
            (*f).mode = 0o20000 as libc::c_uint;
        }
    }
    free(data);
    return rv;
}
