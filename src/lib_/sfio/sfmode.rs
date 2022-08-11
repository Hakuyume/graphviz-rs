#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    fn sfsetbuf(_: *mut Sfio_t, _: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn sfraise(_: *mut Sfio_t, _: libc::c_int, _: *mut libc::c_void) -> libc::c_int;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfsk(
        _: *mut Sfio_t,
        _: libc::c_longlong,
        _: libc::c_int,
        _: *mut Sfdisc_t,
    ) -> libc::c_longlong;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut _Sfextern: Sfextern_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type intptr_t = libc::c_long;
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
pub type Sfrsrv_t = _sfrsrv_s;
pub type Sfproc_t = _sfproc_s;
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
pub type Sfextern_t = _sfextern_s;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type Sfsignal_f = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
static mut _Sfsigp: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn _sfcleanup() {
    let mut p: *mut Sfpool_t = 0 as *mut Sfpool_t;
    let mut f: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut n: libc::c_int = 0;
    let mut pool: libc::c_int = 0;
    _Sfextern.sf_exiting = 1001 as libc::c_int;
    sfsync(0 as *mut Sfio_t);
    p = &mut _Sfextern.sf_pool;
    while !p.is_null() {
        n = 0 as libc::c_int;
        while n < (*p).n_sf {
            f = *((*p).sf).offset(n as isize);
            if !(f.is_null()
                || (if (*f).mode
                    & (0o100 as libc::c_uint | 0o40 as libc::c_uint
                        | 0o400 as libc::c_uint) != 0
                {
                    1 as libc::c_int
                } else {
                    (if (*f).mode & 0o10000 as libc::c_uint != 0 {
                        (Some(
                            (_Sfextern.sf_stdsync).expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(f)
                    } else {
                        0 as libc::c_int
                    })
                }) != 0)
            {
                (*f).mode |= 0o40 as libc::c_uint;
                let ref mut fresh0 = (*f).endw;
                *fresh0 = (*f).data;
                let ref mut fresh1 = (*f).endr;
                *fresh1 = *fresh0;
                (*f).mode |= 0o100000 as libc::c_uint;
                sfraise(f, 14 as libc::c_int, 0 as *mut libc::c_void);
                if !((*f).flags as libc::c_int & 0o4 as libc::c_int != 0) {
                    pool = ((*f).mode & 0o200 as libc::c_uint) as libc::c_int;
                    (*f).mode &= !(0o200 as libc::c_uint);
                    if (*f).flags as libc::c_int & 0o2 as libc::c_int != 0
                        && (*f).mode & 0o2 as libc::c_int as libc::c_uint == 0
                    {
                        _sfmode(f, 0o2 as libc::c_int, 1 as libc::c_int);
                    }
                    if (*f).bits as libc::c_int & 0o1 as libc::c_int != 0
                        && !((*f).data).is_null()
                        || (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
                            && (*f).next == (*f).data
                    {
                        (*f).mode |= 0o100000 as libc::c_uint;
                        sfsetbuf(f, 0 as *mut libc::c_void, 0 as libc::c_int as size_t);
                    }
                    (*f).mode |= pool as libc::c_uint;
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
                }
            }
            n += 1;
        }
        p = (*p).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _sfsetpool(mut f: *mut Sfio_t) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *mut Sfpool_t = 0 as *mut Sfpool_t;
    let mut array: *mut *mut Sfio_t = 0 as *mut *mut Sfio_t;
    let mut n: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    if (_Sfextern.sf_cleanup).is_none() {
        _Sfextern.sf_cleanup = Some(_sfcleanup as unsafe extern "C" fn() -> ());
        atexit(Some(_sfcleanup as unsafe extern "C" fn() -> ()));
    }
    p = (*f).pool;
    if p.is_null() {
        let ref mut fresh6 = (*f).pool;
        *fresh6 = &mut _Sfextern.sf_pool;
        p = *fresh6;
    }
    rv = -(1 as libc::c_int);
    if (*p).n_sf >= (*p).s_sf {
        if (*p).s_sf == 0 as libc::c_int {
            (*p)
                .s_sf = (::std::mem::size_of::<[*mut Sfio_t; 3]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut Sfio_t>() as libc::c_ulong)
                as libc::c_int;
            let ref mut fresh7 = (*p).sf;
            *fresh7 = ((*p).array).as_mut_ptr();
            current_block = 2838571290723028321;
        } else {
            n = (if (*p).sf != ((*p).array).as_mut_ptr() {
                (*p).s_sf
            } else {
                ((*p).s_sf / 4 as libc::c_int + 1 as libc::c_int) * 4 as libc::c_int
            }) + 4 as libc::c_int;
            array = malloc(
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut Sfio_t>() as libc::c_ulong),
            ) as *mut *mut Sfio_t;
            if array.is_null() {
                current_block = 12031451804993171459;
            } else {
                memcpy(
                    array as *mut libc::c_void,
                    (*p).sf as *const libc::c_void,
                    ((*p).n_sf as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut Sfio_t>() as libc::c_ulong,
                        ),
                );
                if (*p).sf != ((*p).array).as_mut_ptr() {
                    free((*p).sf as *mut libc::c_void);
                }
                let ref mut fresh8 = (*p).sf;
                *fresh8 = array;
                (*p).s_sf = n;
                current_block = 2838571290723028321;
            }
        }
    } else {
        current_block = 2838571290723028321;
    }
    match current_block {
        2838571290723028321 => {
            let ref mut fresh9 = (*p).n_sf;
            let fresh10 = *fresh9;
            *fresh9 = *fresh9 + 1;
            let ref mut fresh11 = *((*p).sf).offset(fresh10 as isize);
            *fresh11 = f;
            rv = 0 as libc::c_int;
        }
        _ => {}
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn _sfrsrv(
    mut f: *mut Sfio_t,
    mut size: ssize_t,
) -> *mut Sfrsrv_t {
    let mut rsrv: *mut Sfrsrv_t = 0 as *mut Sfrsrv_t;
    let mut rs: *mut Sfrsrv_t = 0 as *mut Sfrsrv_t;
    size = (size + 1024 as libc::c_int as libc::c_long
        - 1 as libc::c_int as libc::c_long) / 1024 as libc::c_int as libc::c_long
        * 1024 as libc::c_int as libc::c_long;
    rsrv = (*f).rsrv;
    if rsrv.is_null() || size > (*rsrv).size {
        rs = malloc(
            (size as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Sfrsrv_t>() as libc::c_ulong),
        ) as *mut Sfrsrv_t;
        if rs.is_null() {
            size = -(1 as libc::c_int) as ssize_t;
        } else {
            if !rsrv.is_null() {
                if (*rsrv).slen > 0 as libc::c_int as libc::c_long {
                    memcpy(
                        rs as *mut libc::c_void,
                        rsrv as *const libc::c_void,
                        (::std::mem::size_of::<Sfrsrv_t>() as libc::c_ulong)
                            .wrapping_add((*rsrv).slen as libc::c_ulong),
                    );
                }
                free(rsrv as *mut libc::c_void);
            }
            rsrv = rs;
            let ref mut fresh12 = (*f).rsrv;
            *fresh12 = rsrv;
            (*rsrv).size = size;
            (*rsrv).slen = 0 as libc::c_int as ssize_t;
        }
    }
    if !rsrv.is_null() && size > 0 as libc::c_int as libc::c_long {
        (*rsrv).slen = 0 as libc::c_int as ssize_t;
    }
    return if size >= 0 as libc::c_int as libc::c_long {
        rsrv
    } else {
        0 as *mut Sfrsrv_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn _sfpopen(
    mut f: *mut Sfio_t,
    mut fd: libc::c_int,
    mut pid: libc::c_int,
    mut stdio: libc::c_int,
) -> libc::c_int {
    let mut p: *mut Sfproc_t = 0 as *mut Sfproc_t;
    if !((*f).proc_0).is_null() {
        return 0 as libc::c_int;
    }
    let ref mut fresh13 = (*f).proc_0;
    *fresh13 = malloc(::std::mem::size_of::<Sfproc_t>() as libc::c_ulong)
        as *mut _sfproc_s;
    p = *fresh13;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    (*p).pid = pid;
    let ref mut fresh14 = (*p).ndata;
    *fresh14 = 0 as libc::c_int;
    (*p).size = *fresh14;
    let ref mut fresh15 = (*p).rdata;
    *fresh15 = 0 as *mut libc::c_uchar;
    (*p).file = fd;
    (*p)
        .sigp = if stdio == 0 && pid >= 0 as libc::c_int
        && (*f).flags as libc::c_int & 0o2 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if (*p).sigp != 0 {
        let mut handler: Sfsignal_f = None;
        handler = signal(
            13 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        if handler.is_some()
            && handler
                != ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
        {
            signal(13 as libc::c_int, handler);
        }
        _Sfsigp += 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _sfpclose(mut f: *mut Sfio_t) -> libc::c_int {
    let mut p: *mut Sfproc_t = 0 as *mut Sfproc_t;
    let mut pid: libc::c_int = 0 as libc::c_int;
    let mut status: libc::c_int = 0;
    p = (*f).proc_0;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    let ref mut fresh16 = (*f).proc_0;
    *fresh16 = 0 as *mut _sfproc_s;
    free((*p).rdata as *mut libc::c_void);
    if (*p).pid < 0 as libc::c_int {
        status = 0 as libc::c_int;
    } else {
        if (*p).file >= 0 as libc::c_int {
            while close((*p).file) < 0 as libc::c_int
                && *__errno_location() == 4 as libc::c_int
            {
                *__errno_location() = 0 as libc::c_int;
            }
        }
        loop {
            pid = waitpid((*p).pid, &mut status, 0 as libc::c_int);
            if !(pid == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if pid < 0 as libc::c_int {
            status = -(1 as libc::c_int);
        }
        if (*p).sigp != 0
            && {
                _Sfsigp -= 1 as libc::c_int;
                _Sfsigp <= 0 as libc::c_int
            }
        {
            let mut handler: Sfsignal_f = None;
            handler = signal(13 as libc::c_int, None);
            if handler.is_some()
                && handler
                    != ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t)
            {
                signal(13 as libc::c_int, handler);
            }
            _Sfsigp = 0 as libc::c_int;
        }
    }
    free(p as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn _sfpmode(
    mut f: *mut Sfio_t,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut p: *mut Sfproc_t = 0 as *mut Sfproc_t;
    p = (*f).proc_0;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if type_0 == 0o2 as libc::c_int {
        (*p).ndata = ((*f).endb).offset_from((*f).next) as libc::c_long as libc::c_int;
        if (*p).ndata > (*p).size {
            free((*p).rdata as *mut libc::c_void);
            let ref mut fresh17 = (*p).rdata;
            *fresh17 = malloc((*p).ndata as libc::c_ulong) as *mut libc::c_uchar;
            if !(*fresh17).is_null() {
                (*p).size = (*p).ndata;
            } else {
                (*p).size = 0 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
        if (*p).ndata > 0 as libc::c_int {
            memcpy(
                (*p).rdata as *mut libc::c_void,
                (*f).next as *const libc::c_void,
                (*p).ndata as libc::c_ulong,
            );
        }
        let ref mut fresh18 = (*f).endb;
        *fresh18 = (*f).data;
    } else {
        if (*p).ndata as libc::c_long > (*f).size {
            (*p).ndata = (*f).size as libc::c_int;
        }
        if (*p).ndata > 0 as libc::c_int {
            memcpy(
                (*f).data as *mut libc::c_void,
                (*p).rdata as *const libc::c_void,
                (*p).ndata as libc::c_ulong,
            );
            let ref mut fresh19 = (*f).endb;
            *fresh19 = ((*f).data).offset((*p).ndata as isize);
            (*p).ndata = 0 as libc::c_int;
        }
    }
    if (*p).pid >= 0 as libc::c_int {
        type_0 = (*f).file as libc::c_int;
        (*f).file = (*p).file as libc::c_short;
        (*p).file = type_0;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _sfmode(
    mut f: *mut Sfio_t,
    mut wanted: libc::c_int,
    mut local: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut addr: libc::c_longlong = 0;
    let mut rv: libc::c_int = 0 as libc::c_int;
    if local == 0
        && (if (*f).mode
            & (0o100 as libc::c_uint | 0o40 as libc::c_uint | 0o400 as libc::c_uint) != 0
        {
            1 as libc::c_int
        } else {
            (if (*f).mode & 0o10000 as libc::c_uint != 0 {
                (Some((_Sfextern.sf_stdsync).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f)
            } else {
                0 as libc::c_int
            })
        }) != 0
        || (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
            && ((*f).file as libc::c_int) < 0 as libc::c_int
    {
        if local != 0 || ((*f).disc).is_null() || ((*(*f).disc).exceptf).is_none() {
            local = 1 as libc::c_int;
            current_block = 12413615873101752566;
        } else {
            loop {
                rv = ((*(*f).disc).exceptf)
                    .expect(
                        "non-null function pointer",
                    )(f, 13 as libc::c_int, 0 as *mut libc::c_void, (*f).disc);
                if rv < 0 as libc::c_int {
                    return rv;
                }
                if !(local == 0
                    && (if (*f).mode
                        & (0o100 as libc::c_uint | 0o40 as libc::c_uint
                            | 0o400 as libc::c_uint) != 0
                    {
                        1 as libc::c_int
                    } else {
                        (if (*f).mode & 0o10000 as libc::c_uint != 0 {
                            (Some(
                                (_Sfextern.sf_stdsync).expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(f)
                        } else {
                            0 as libc::c_int
                        })
                    }) != 0
                    || (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
                        && ((*f).file as libc::c_int) < 0 as libc::c_int)
                {
                    current_block = 3512920355445576850;
                    break;
                }
                if !(rv == 0 as libc::c_int) {
                    continue;
                }
                local = 1 as libc::c_int;
                current_block = 12413615873101752566;
                break;
            }
        }
    } else {
        current_block = 3512920355445576850;
    }
    match current_block {
        3512920355445576850 => {
            if (*f).mode & 0o2000 as libc::c_uint != 0 {
                (*f).mode &= !(0o2000 as libc::c_uint);
                if (*f).getr != 0 {
                    *((*f).next).offset(-(1 as libc::c_int) as isize) = (*f).getr;
                    (*f).getr = 0 as libc::c_int as libc::c_uchar;
                }
            }
            if (*f).mode & 0o10000 as libc::c_uint != 0 {
                (Some((_Sfextern.sf_stdsync).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f);
            }
            if (*f).disc == &mut _Sfextern.sf_udisc as *mut _sfdisc_s
                && wanted == 0o2 as libc::c_int
                && sfclose(
                    (Some((_Sfextern.sf_stack).expect("non-null function pointer")))
                        .expect("non-null function pointer")(f, 0 as *mut Sfio_t),
                ) < 0 as libc::c_int
            {
                local = 1 as libc::c_int;
                current_block = 12413615873101752566;
            } else {
                if (*f).mode & 0o200 as libc::c_uint != 0 {
                    if f == *((*(*f).pool).sf).offset(0 as libc::c_int as isize)
                        || (_Sfextern.sf_pmove)
                            .expect("non-null function pointer")(f, 0 as libc::c_int)
                            < 0 as libc::c_int
                    {
                        local = 1 as libc::c_int;
                        current_block = 12413615873101752566;
                    } else {
                        (*f).mode &= !(0o200 as libc::c_uint);
                        current_block = 5689316957504528238;
                    }
                } else {
                    current_block = 5689316957504528238;
                }
                match current_block {
                    12413615873101752566 => {}
                    _ => {
                        (*f).mode |= 0o40 as libc::c_uint;
                        let ref mut fresh20 = (*f).endw;
                        *fresh20 = (*f).data;
                        let ref mut fresh21 = (*f).endr;
                        *fresh21 = *fresh20;
                        wanted &= 0o1 as libc::c_int | 0o2 as libc::c_int;
                        if (*f).mode & 0o4 as libc::c_uint != 0 {
                            if ((*f).pool).is_null() && _sfsetpool(f) < 0 as libc::c_int
                            {
                                rv = -(1 as libc::c_int);
                                current_block = 14250728506927726536;
                            } else if wanted == 0 as libc::c_int {
                                current_block = 14250728506927726536;
                            } else if wanted
                                    != ((*f).mode
                                        & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint)
                                        as libc::c_int && (*f).flags as libc::c_int & wanted == 0
                                {
                                current_block = 12413615873101752566;
                            } else {
                                if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0
                                    && (*f).size >= 0 as libc::c_int as libc::c_long
                                    && !((*f).data).is_null()
                                {
                                    (*f).mode &= !(0o4 as libc::c_uint);
                                    (*f)
                                        .extent = (if (*f).flags as libc::c_int & 0o1 as libc::c_int
                                        != 0 || (*f).bits as libc::c_int & 0o2 as libc::c_int != 0
                                    {
                                        (*f).size
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                    }) as libc::c_longlong;
                                    (*f).here = 0 as libc::c_int as libc::c_longlong;
                                    let ref mut fresh22 = (*f).endb;
                                    *fresh22 = ((*f).data).offset((*f).size as isize);
                                    let ref mut fresh23 = (*f).endw;
                                    *fresh23 = (*f).data;
                                    let ref mut fresh24 = (*f).endr;
                                    *fresh24 = *fresh23;
                                    let ref mut fresh25 = (*f).next;
                                    *fresh25 = *fresh24;
                                    if (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0 {
                                        let ref mut fresh26 = (*f).endr;
                                        *fresh26 = (*f).endb;
                                    } else {
                                        let ref mut fresh27 = (*f).endw;
                                        *fresh27 = (*f).endb;
                                    }
                                } else {
                                    let mut n: libc::c_ushort = (*f).flags;
                                    (*f).mode |= 0o100000 as libc::c_uint;
                                    sfsetbuf(
                                        f,
                                        (*f).data as *mut libc::c_void,
                                        (*f).size as size_t,
                                    );
                                    let ref mut fresh28 = (*f).flags;
                                    *fresh28 = (*fresh28 as libc::c_int
                                        | n as libc::c_int & 0o20 as libc::c_int) as libc::c_ushort;
                                }
                                current_block = 3689906465960840878;
                            }
                        } else {
                            current_block = 3689906465960840878;
                        }
                        match current_block {
                            14250728506927726536 => {}
                            12413615873101752566 => {}
                            _ => {
                                if wanted
                                    == ((*f).mode
                                        & !(0o20 as libc::c_uint | 0o10 as libc::c_uint
                                            | (if 1 as libc::c_int != 0 {
                                                0o40 as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }))) as libc::c_int
                                {
                                    current_block = 14250728506927726536;
                                } else {
                                    match (*f).mode
                                        & !(0o20 as libc::c_uint | 0o10 as libc::c_uint
                                            | (if 1 as libc::c_int != 0 {
                                                0o40 as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }))
                                    {
                                        2 => {
                                            current_block = 16838755845805742633;
                                            match current_block {
                                                1129389781112076955 => {
                                                    if wanted != 0o2 as libc::c_int {
                                                        (*f)
                                                            .mode = 0o1 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        if (*f).flags as libc::c_int
                                                            & (0o100 as libc::c_int | 0o4000 as libc::c_int)
                                                            == 0o100 as libc::c_int | 0o4000 as libc::c_int
                                                            && {
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                addr = sfsk(
                                                                    f,
                                                                    0 as libc::c_int as libc::c_longlong,
                                                                    1 as libc::c_int,
                                                                    (*f).disc,
                                                                );
                                                                addr != (*f).here
                                                            }
                                                        {
                                                            let ref mut fresh35 = (*f).next;
                                                            *fresh35 = (*f).data;
                                                            let ref mut fresh36 = (*f).endw;
                                                            *fresh36 = *fresh35;
                                                            let ref mut fresh37 = (*f).endr;
                                                            *fresh37 = *fresh36;
                                                            let ref mut fresh38 = (*f).endb;
                                                            *fresh38 = *fresh37;
                                                            (*f).here = addr;
                                                            current_block = 14250728506927726536;
                                                        } else {
                                                            addr = (*f).here
                                                                + ((*f).endb).offset_from((*f).next) as libc::c_long
                                                                    as libc::c_longlong;
                                                            (*f).mode |= 0o100000 as libc::c_uint;
                                                            if sfsk(f, addr, 0 as libc::c_int, (*f).disc)
                                                                < 0 as libc::c_int as libc::c_longlong
                                                            {
                                                                current_block = 12413615873101752566;
                                                            } else {
                                                                (*f).here = addr;
                                                                current_block = 14250728506927726536;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 5035991803703221597;
                                                    }
                                                }
                                                16838755845805742633 => {
                                                    if wanted == 0 as libc::c_int
                                                        || wanted == 0o2 as libc::c_int
                                                    {
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).flags as libc::c_int & 0o1 as libc::c_int
                                                            == 0
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else if (*f).flags as libc::c_int & 0o4 as libc::c_int
                                                            != 0
                                                        {
                                                        let mut s: libc::c_longlong = ((*f).next)
                                                            .offset_from((*f).data) as libc::c_long as libc::c_longlong;
                                                        if s > (*f).here {
                                                            (*f).here = s;
                                                            if s > (*f).extent {
                                                                (*f).extent = s;
                                                            }
                                                        }
                                                        let ref mut fresh29 = (*f).endb;
                                                        *fresh29 = ((*f).data).offset((*f).extent as isize);
                                                        (*f).mode = 0o1 as libc::c_int as libc::c_uint;
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).next > (*f).data
                                                            && {
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                _sfflsbuf(f, -(1 as libc::c_int)) < 0 as libc::c_int
                                                            }
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else {
                                                        if (*f).size == 0 as libc::c_int as libc::c_long {
                                                            let ref mut fresh30 = (*f).data;
                                                            *fresh30 = ((*f).tiny).as_mut_ptr();
                                                            (*f)
                                                                .size = ::std::mem::size_of::<[libc::c_uchar; 1]>()
                                                                as libc::c_ulong as ssize_t;
                                                        }
                                                        let ref mut fresh31 = (*f).endb;
                                                        *fresh31 = (*f).data;
                                                        let ref mut fresh32 = (*f).endw;
                                                        *fresh32 = *fresh31;
                                                        let ref mut fresh33 = (*f).endr;
                                                        *fresh33 = *fresh32;
                                                        let ref mut fresh34 = (*f).next;
                                                        *fresh34 = *fresh33;
                                                        (*f)
                                                            .mode = 0o1 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        if !((*f).proc_0).is_null()
                                                            && _sfpmode(f, wanted) < 0 as libc::c_int
                                                        {
                                                            current_block = 12413615873101752566;
                                                        } else {
                                                            current_block = 14250728506927726536;
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                14250728506927726536 => {}
                                                12413615873101752566 => {}
                                                _ => {
                                                    if wanted != 0o2 as libc::c_int {
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).flags as libc::c_int & 0o2 as libc::c_int
                                                            == 0
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else if (*f).flags as libc::c_int & 0o4 as libc::c_int
                                                            != 0
                                                        {
                                                        let ref mut fresh39 = (*f).endb;
                                                        *fresh39 = ((*f).data).offset((*f).size as isize);
                                                        (*f)
                                                            .mode = 0o2 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        current_block = 14250728506927726536;
                                                    } else if !((*f).proc_0).is_null()
                                                            && _sfpmode(f, wanted) < 0 as libc::c_int
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else {
                                                        if (*f).mode & 0o4000 as libc::c_uint == 0 {
                                                            let mut n_0: intptr_t = ((*f).endb).offset_from((*f).next)
                                                                as libc::c_long;
                                                            if (*f).extent >= 0 as libc::c_int as libc::c_longlong
                                                                && (n_0 > 0 as libc::c_int as libc::c_long
                                                                    || !((*f).data).is_null()
                                                                        && (*f).bits as libc::c_int & 0o1 as libc::c_int != 0)
                                                            {
                                                                addr = (*f).here - n_0 as libc::c_longlong;
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                if sfsk(f, addr, 0 as libc::c_int, (*f).disc)
                                                                    < 0 as libc::c_int as libc::c_longlong
                                                                {
                                                                    current_block = 12413615873101752566;
                                                                } else {
                                                                    (*f).here = addr;
                                                                    current_block = 2362946907345824597;
                                                                }
                                                            } else {
                                                                current_block = 2362946907345824597;
                                                            }
                                                        } else {
                                                            current_block = 2362946907345824597;
                                                        }
                                                        match current_block {
                                                            12413615873101752566 => {}
                                                            _ => {
                                                                (*f)
                                                                    .mode = 0o2 as libc::c_int as libc::c_uint
                                                                    | 0o40 as libc::c_uint;
                                                                if (*f).data == ((*f).tiny).as_mut_ptr() {
                                                                    let ref mut fresh40 = (*f).next;
                                                                    *fresh40 = 0 as *mut libc::c_uchar;
                                                                    let ref mut fresh41 = (*f).data;
                                                                    *fresh41 = *fresh40;
                                                                    let ref mut fresh42 = (*f).endb;
                                                                    *fresh42 = *fresh41;
                                                                    (*f).size = 0 as libc::c_int as ssize_t;
                                                                } else {
                                                                    let ref mut fresh43 = (*f).next;
                                                                    *fresh43 = (*f).data;
                                                                    let ref mut fresh44 = (*f).endb;
                                                                    *fresh44 = (*fresh43).offset((*f).size as isize);
                                                                }
                                                                current_block = 14250728506927726536;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        2049 => {
                                            current_block = 1129389781112076955;
                                            match current_block {
                                                1129389781112076955 => {
                                                    if wanted != 0o2 as libc::c_int {
                                                        (*f)
                                                            .mode = 0o1 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        if (*f).flags as libc::c_int
                                                            & (0o100 as libc::c_int | 0o4000 as libc::c_int)
                                                            == 0o100 as libc::c_int | 0o4000 as libc::c_int
                                                            && {
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                addr = sfsk(
                                                                    f,
                                                                    0 as libc::c_int as libc::c_longlong,
                                                                    1 as libc::c_int,
                                                                    (*f).disc,
                                                                );
                                                                addr != (*f).here
                                                            }
                                                        {
                                                            let ref mut fresh35 = (*f).next;
                                                            *fresh35 = (*f).data;
                                                            let ref mut fresh36 = (*f).endw;
                                                            *fresh36 = *fresh35;
                                                            let ref mut fresh37 = (*f).endr;
                                                            *fresh37 = *fresh36;
                                                            let ref mut fresh38 = (*f).endb;
                                                            *fresh38 = *fresh37;
                                                            (*f).here = addr;
                                                            current_block = 14250728506927726536;
                                                        } else {
                                                            addr = (*f).here
                                                                + ((*f).endb).offset_from((*f).next) as libc::c_long
                                                                    as libc::c_longlong;
                                                            (*f).mode |= 0o100000 as libc::c_uint;
                                                            if sfsk(f, addr, 0 as libc::c_int, (*f).disc)
                                                                < 0 as libc::c_int as libc::c_longlong
                                                            {
                                                                current_block = 12413615873101752566;
                                                            } else {
                                                                (*f).here = addr;
                                                                current_block = 14250728506927726536;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 5035991803703221597;
                                                    }
                                                }
                                                16838755845805742633 => {
                                                    if wanted == 0 as libc::c_int
                                                        || wanted == 0o2 as libc::c_int
                                                    {
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).flags as libc::c_int & 0o1 as libc::c_int
                                                            == 0
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else if (*f).flags as libc::c_int & 0o4 as libc::c_int
                                                            != 0
                                                        {
                                                        let mut s: libc::c_longlong = ((*f).next)
                                                            .offset_from((*f).data) as libc::c_long as libc::c_longlong;
                                                        if s > (*f).here {
                                                            (*f).here = s;
                                                            if s > (*f).extent {
                                                                (*f).extent = s;
                                                            }
                                                        }
                                                        let ref mut fresh29 = (*f).endb;
                                                        *fresh29 = ((*f).data).offset((*f).extent as isize);
                                                        (*f).mode = 0o1 as libc::c_int as libc::c_uint;
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).next > (*f).data
                                                            && {
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                _sfflsbuf(f, -(1 as libc::c_int)) < 0 as libc::c_int
                                                            }
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else {
                                                        if (*f).size == 0 as libc::c_int as libc::c_long {
                                                            let ref mut fresh30 = (*f).data;
                                                            *fresh30 = ((*f).tiny).as_mut_ptr();
                                                            (*f)
                                                                .size = ::std::mem::size_of::<[libc::c_uchar; 1]>()
                                                                as libc::c_ulong as ssize_t;
                                                        }
                                                        let ref mut fresh31 = (*f).endb;
                                                        *fresh31 = (*f).data;
                                                        let ref mut fresh32 = (*f).endw;
                                                        *fresh32 = *fresh31;
                                                        let ref mut fresh33 = (*f).endr;
                                                        *fresh33 = *fresh32;
                                                        let ref mut fresh34 = (*f).next;
                                                        *fresh34 = *fresh33;
                                                        (*f)
                                                            .mode = 0o1 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        if !((*f).proc_0).is_null()
                                                            && _sfpmode(f, wanted) < 0 as libc::c_int
                                                        {
                                                            current_block = 12413615873101752566;
                                                        } else {
                                                            current_block = 14250728506927726536;
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                14250728506927726536 => {}
                                                12413615873101752566 => {}
                                                _ => {
                                                    if wanted != 0o2 as libc::c_int {
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).flags as libc::c_int & 0o2 as libc::c_int
                                                            == 0
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else if (*f).flags as libc::c_int & 0o4 as libc::c_int
                                                            != 0
                                                        {
                                                        let ref mut fresh39 = (*f).endb;
                                                        *fresh39 = ((*f).data).offset((*f).size as isize);
                                                        (*f)
                                                            .mode = 0o2 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        current_block = 14250728506927726536;
                                                    } else if !((*f).proc_0).is_null()
                                                            && _sfpmode(f, wanted) < 0 as libc::c_int
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else {
                                                        if (*f).mode & 0o4000 as libc::c_uint == 0 {
                                                            let mut n_0: intptr_t = ((*f).endb).offset_from((*f).next)
                                                                as libc::c_long;
                                                            if (*f).extent >= 0 as libc::c_int as libc::c_longlong
                                                                && (n_0 > 0 as libc::c_int as libc::c_long
                                                                    || !((*f).data).is_null()
                                                                        && (*f).bits as libc::c_int & 0o1 as libc::c_int != 0)
                                                            {
                                                                addr = (*f).here - n_0 as libc::c_longlong;
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                if sfsk(f, addr, 0 as libc::c_int, (*f).disc)
                                                                    < 0 as libc::c_int as libc::c_longlong
                                                                {
                                                                    current_block = 12413615873101752566;
                                                                } else {
                                                                    (*f).here = addr;
                                                                    current_block = 2362946907345824597;
                                                                }
                                                            } else {
                                                                current_block = 2362946907345824597;
                                                            }
                                                        } else {
                                                            current_block = 2362946907345824597;
                                                        }
                                                        match current_block {
                                                            12413615873101752566 => {}
                                                            _ => {
                                                                (*f)
                                                                    .mode = 0o2 as libc::c_int as libc::c_uint
                                                                    | 0o40 as libc::c_uint;
                                                                if (*f).data == ((*f).tiny).as_mut_ptr() {
                                                                    let ref mut fresh40 = (*f).next;
                                                                    *fresh40 = 0 as *mut libc::c_uchar;
                                                                    let ref mut fresh41 = (*f).data;
                                                                    *fresh41 = *fresh40;
                                                                    let ref mut fresh42 = (*f).endb;
                                                                    *fresh42 = *fresh41;
                                                                    (*f).size = 0 as libc::c_int as ssize_t;
                                                                } else {
                                                                    let ref mut fresh43 = (*f).next;
                                                                    *fresh43 = (*f).data;
                                                                    let ref mut fresh44 = (*f).endb;
                                                                    *fresh44 = (*fresh43).offset((*f).size as isize);
                                                                }
                                                                current_block = 14250728506927726536;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        1 => {
                                            current_block = 5035991803703221597;
                                            match current_block {
                                                1129389781112076955 => {
                                                    if wanted != 0o2 as libc::c_int {
                                                        (*f)
                                                            .mode = 0o1 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        if (*f).flags as libc::c_int
                                                            & (0o100 as libc::c_int | 0o4000 as libc::c_int)
                                                            == 0o100 as libc::c_int | 0o4000 as libc::c_int
                                                            && {
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                addr = sfsk(
                                                                    f,
                                                                    0 as libc::c_int as libc::c_longlong,
                                                                    1 as libc::c_int,
                                                                    (*f).disc,
                                                                );
                                                                addr != (*f).here
                                                            }
                                                        {
                                                            let ref mut fresh35 = (*f).next;
                                                            *fresh35 = (*f).data;
                                                            let ref mut fresh36 = (*f).endw;
                                                            *fresh36 = *fresh35;
                                                            let ref mut fresh37 = (*f).endr;
                                                            *fresh37 = *fresh36;
                                                            let ref mut fresh38 = (*f).endb;
                                                            *fresh38 = *fresh37;
                                                            (*f).here = addr;
                                                            current_block = 14250728506927726536;
                                                        } else {
                                                            addr = (*f).here
                                                                + ((*f).endb).offset_from((*f).next) as libc::c_long
                                                                    as libc::c_longlong;
                                                            (*f).mode |= 0o100000 as libc::c_uint;
                                                            if sfsk(f, addr, 0 as libc::c_int, (*f).disc)
                                                                < 0 as libc::c_int as libc::c_longlong
                                                            {
                                                                current_block = 12413615873101752566;
                                                            } else {
                                                                (*f).here = addr;
                                                                current_block = 14250728506927726536;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 5035991803703221597;
                                                    }
                                                }
                                                16838755845805742633 => {
                                                    if wanted == 0 as libc::c_int
                                                        || wanted == 0o2 as libc::c_int
                                                    {
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).flags as libc::c_int & 0o1 as libc::c_int
                                                            == 0
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else if (*f).flags as libc::c_int & 0o4 as libc::c_int
                                                            != 0
                                                        {
                                                        let mut s: libc::c_longlong = ((*f).next)
                                                            .offset_from((*f).data) as libc::c_long as libc::c_longlong;
                                                        if s > (*f).here {
                                                            (*f).here = s;
                                                            if s > (*f).extent {
                                                                (*f).extent = s;
                                                            }
                                                        }
                                                        let ref mut fresh29 = (*f).endb;
                                                        *fresh29 = ((*f).data).offset((*f).extent as isize);
                                                        (*f).mode = 0o1 as libc::c_int as libc::c_uint;
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).next > (*f).data
                                                            && {
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                _sfflsbuf(f, -(1 as libc::c_int)) < 0 as libc::c_int
                                                            }
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else {
                                                        if (*f).size == 0 as libc::c_int as libc::c_long {
                                                            let ref mut fresh30 = (*f).data;
                                                            *fresh30 = ((*f).tiny).as_mut_ptr();
                                                            (*f)
                                                                .size = ::std::mem::size_of::<[libc::c_uchar; 1]>()
                                                                as libc::c_ulong as ssize_t;
                                                        }
                                                        let ref mut fresh31 = (*f).endb;
                                                        *fresh31 = (*f).data;
                                                        let ref mut fresh32 = (*f).endw;
                                                        *fresh32 = *fresh31;
                                                        let ref mut fresh33 = (*f).endr;
                                                        *fresh33 = *fresh32;
                                                        let ref mut fresh34 = (*f).next;
                                                        *fresh34 = *fresh33;
                                                        (*f)
                                                            .mode = 0o1 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        if !((*f).proc_0).is_null()
                                                            && _sfpmode(f, wanted) < 0 as libc::c_int
                                                        {
                                                            current_block = 12413615873101752566;
                                                        } else {
                                                            current_block = 14250728506927726536;
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                            match current_block {
                                                14250728506927726536 => {}
                                                12413615873101752566 => {}
                                                _ => {
                                                    if wanted != 0o2 as libc::c_int {
                                                        current_block = 14250728506927726536;
                                                    } else if (*f).flags as libc::c_int & 0o2 as libc::c_int
                                                            == 0
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else if (*f).flags as libc::c_int & 0o4 as libc::c_int
                                                            != 0
                                                        {
                                                        let ref mut fresh39 = (*f).endb;
                                                        *fresh39 = ((*f).data).offset((*f).size as isize);
                                                        (*f)
                                                            .mode = 0o2 as libc::c_int as libc::c_uint
                                                            | 0o40 as libc::c_uint;
                                                        current_block = 14250728506927726536;
                                                    } else if !((*f).proc_0).is_null()
                                                            && _sfpmode(f, wanted) < 0 as libc::c_int
                                                        {
                                                        current_block = 12413615873101752566;
                                                    } else {
                                                        if (*f).mode & 0o4000 as libc::c_uint == 0 {
                                                            let mut n_0: intptr_t = ((*f).endb).offset_from((*f).next)
                                                                as libc::c_long;
                                                            if (*f).extent >= 0 as libc::c_int as libc::c_longlong
                                                                && (n_0 > 0 as libc::c_int as libc::c_long
                                                                    || !((*f).data).is_null()
                                                                        && (*f).bits as libc::c_int & 0o1 as libc::c_int != 0)
                                                            {
                                                                addr = (*f).here - n_0 as libc::c_longlong;
                                                                (*f).mode |= 0o100000 as libc::c_uint;
                                                                if sfsk(f, addr, 0 as libc::c_int, (*f).disc)
                                                                    < 0 as libc::c_int as libc::c_longlong
                                                                {
                                                                    current_block = 12413615873101752566;
                                                                } else {
                                                                    (*f).here = addr;
                                                                    current_block = 2362946907345824597;
                                                                }
                                                            } else {
                                                                current_block = 2362946907345824597;
                                                            }
                                                        } else {
                                                            current_block = 2362946907345824597;
                                                        }
                                                        match current_block {
                                                            12413615873101752566 => {}
                                                            _ => {
                                                                (*f)
                                                                    .mode = 0o2 as libc::c_int as libc::c_uint
                                                                    | 0o40 as libc::c_uint;
                                                                if (*f).data == ((*f).tiny).as_mut_ptr() {
                                                                    let ref mut fresh40 = (*f).next;
                                                                    *fresh40 = 0 as *mut libc::c_uchar;
                                                                    let ref mut fresh41 = (*f).data;
                                                                    *fresh41 = *fresh40;
                                                                    let ref mut fresh42 = (*f).endb;
                                                                    *fresh42 = *fresh41;
                                                                    (*f).size = 0 as libc::c_int as ssize_t;
                                                                } else {
                                                                    let ref mut fresh43 = (*f).next;
                                                                    *fresh43 = (*f).data;
                                                                    let ref mut fresh44 = (*f).endb;
                                                                    *fresh44 = (*fresh43).offset((*f).size as isize);
                                                                }
                                                                current_block = 14250728506927726536;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            current_block = 12413615873101752566;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        12413615873101752566 => {
            wanted &= 0o1 as libc::c_int | 0o2 as libc::c_int;
            if wanted == 0 as libc::c_int
                && {
                    wanted = (*f).flags as libc::c_int
                        & (0o1 as libc::c_int | 0o2 as libc::c_int);
                    wanted == 0o1 as libc::c_int | 0o2 as libc::c_int
                }
            {
                wanted = 0o1 as libc::c_int;
            }
            if wanted as libc::c_uint
                != (*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
                && (*f).file as libc::c_int >= 0 as libc::c_int
            {
                *__errno_location() = 9 as libc::c_int;
            }
            if (_Sfextern.sf_notify).is_some() {
                (_Sfextern.sf_notify)
                    .expect(
                        "non-null function pointer",
                    )(f, wanted, (*f).file as libc::c_int);
            }
            rv = -(1 as libc::c_int);
        }
        _ => {}
    }
    if local != 0 {} else {
        (*f).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh45 = (*f).endr;
            *fresh45 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh46 = (*f).endw;
                *fresh46 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh47 = (*f).endr;
                *fresh47 = (*f).data;
                let ref mut fresh48 = (*f).endw;
                *fresh48 = *fresh47;
            };
        };
    };
    return rv;
}
