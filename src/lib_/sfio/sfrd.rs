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
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    fn _sfexcept(_: *mut Sfio_t, _: libc::c_int, _: ssize_t, _: *mut Sfdisc_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sfpkrd(
        _: libc::c_int,
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_long,
        _: libc::c_int,
    ) -> ssize_t;
    fn sfsk(
        _: *mut Sfio_t,
        _: libc::c_longlong,
        _: libc::c_int,
        _: *mut Sfdisc_t,
    ) -> libc::c_longlong;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
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
unsafe extern "C" fn _sfwrsync() {
    let mut p: *mut Sfpool_t = 0 as *mut Sfpool_t;
    let mut f: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut n: libc::c_int = 0;
    p = _Sfextern.sf_pool.next;
    while !p.is_null() {
        if !((*p).n_sf <= 0 as libc::c_int) {
            f = *((*p).sf).offset(0 as libc::c_int as isize);
            if (if (*f).mode
                & (0o100 as libc::c_uint | 0o40 as libc::c_uint | 0o400 as libc::c_uint)
                != 0
            {
                1 as libc::c_int
            } else {
                (if (*f).mode & 0o10000 as libc::c_uint != 0 {
                    (Some((_Sfextern.sf_stdsync).expect("non-null function pointer")))
                        .expect("non-null function pointer")(f)
                } else {
                    0 as libc::c_int
                })
            }) == 0
                && (*f).next > (*f).data
                && (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
                && (*f).extent < 0 as libc::c_int as libc::c_longlong
            {
                _sfflsbuf(f, -(1 as libc::c_int));
            }
        }
        p = (*p).next;
    }
    n = 0 as libc::c_int;
    while n < _Sfextern.sf_pool.n_sf {
        f = *(_Sfextern.sf_pool.sf).offset(n as isize);
        if (if (*f).mode & (0o100 as libc::c_uint | 0o40 as libc::c_uint | 0o400 as libc::c_uint)
            != 0
        {
            1 as libc::c_int
        } else {
            (if (*f).mode & 0o10000 as libc::c_uint != 0 {
                (Some((_Sfextern.sf_stdsync).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f)
            } else {
                0 as libc::c_int
            })
        }) == 0
            && (*f).next > (*f).data
            && (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
            && (*f).extent < 0 as libc::c_int as libc::c_longlong
        {
            _sfflsbuf(f, -(1 as libc::c_int));
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sfrd(
    mut f: *mut Sfio_t,
    mut buf: *mut libc::c_void,
    mut n: size_t,
    mut disc: *mut Sfdisc_t,
) -> ssize_t {
    let mut current_block: u64;
    let mut r: libc::c_longlong = 0;
    let mut dc: *mut Sfdisc_t = 0 as *mut Sfdisc_t;
    let mut local: libc::c_int = 0;
    let mut dosync: libc::c_int = 0;
    let mut oerrno: libc::c_int = 0;
    let mut rcrv: libc::c_uint = 0;
    if f.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    rcrv = (*f).mode & (0o10 as libc::c_uint | 0o20 as libc::c_uint);
    if rcrv != 0 {
        (*f).mode &= !(0o10 as libc::c_uint | 0o20 as libc::c_uint);
    }
    let ref mut fresh0 = (*f).bits;
    *fresh0 = (*fresh0 as libc::c_int & !(0o40 as libc::c_int) as libc::c_ushort as libc::c_int)
        as libc::c_ushort;
    if (*f).mode & 0o1000 as libc::c_uint != 0 {
        return -(1 as libc::c_int) as ssize_t;
    }
    if local == 0 && (*f).bits as libc::c_int & 0o1000 as libc::c_int == 0 {
        if (*f).mode != 0o1 as libc::c_int as libc::c_uint
            && _sfmode(f, 0o1 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        if (*f).next < (*f).endb {
            (*f).mode |= 0o100000 as libc::c_uint;
            if sfsync(f) < 0 as libc::c_int {
                return -(1 as libc::c_int) as ssize_t;
            }
            let ref mut fresh1 = (*f).endw;
            *fresh1 = (*f).data;
            let ref mut fresh2 = (*f).endr;
            *fresh2 = *fresh1;
            let ref mut fresh3 = (*f).endb;
            *fresh3 = *fresh2;
            let ref mut fresh4 = (*f).next;
            *fresh4 = *fresh3;
        }
    }
    dosync = 0 as libc::c_int;
    loop {
        if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
            && ((*f).file as libc::c_int) < 0 as libc::c_int
        {
            return 0 as libc::c_int as ssize_t;
        }
        let ref mut fresh5 = (*f).flags;
        *fresh5 = (*fresh5 as libc::c_int
            & !(0o200 as libc::c_int | 0o400 as libc::c_int) as libc::c_ushort as libc::c_int)
            as libc::c_ushort;
        dc = disc;
        if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
            r = ((*f).data)
                .offset((*f).extent as isize)
                .offset_from((*f).next) as libc::c_long as libc::c_longlong;
            if r < 0 as libc::c_int as libc::c_longlong {
                r = 0 as libc::c_int as libc::c_longlong;
            }
            if !(r <= 0 as libc::c_int as libc::c_longlong) {
                return r as ssize_t;
            }
        } else {
            let mut d: *mut Sfdisc_t = 0 as *mut Sfdisc_t;
            if dc.is_null() {
                dc = (*f).disc;
                d = dc;
            } else {
                d = if (*f).bits as libc::c_int & 0o1000 as libc::c_int != 0 {
                    dc = (*dc).disc;
                    dc
                } else {
                    dc
                };
            }
            while !d.is_null() && ((*d).readf).is_none() {
                d = (*d).disc;
            }
            if !d.is_null() {
                dc = d;
            }
            if !dc.is_null()
                && ((*dc).exceptf).is_some()
                && (*f).flags as libc::c_int & 0o2000 as libc::c_int != 0
            {
                let mut rv: libc::c_int = 0;
                if local != 0 {
                    (*f).mode |= 0o100000 as libc::c_uint;
                }
                rv = _sfexcept(f, 0o1 as libc::c_int, n as ssize_t, dc);
                if rv > 0 as libc::c_int {
                    n = rv as size_t;
                } else if rv < 0 as libc::c_int {
                    let ref mut fresh6 = (*f).flags;
                    *fresh6 = (*fresh6 as libc::c_int | 0o400 as libc::c_int) as libc::c_ushort;
                    return rv as ssize_t;
                }
            }
            if dosync == 0 && (*f).extent < 0 as libc::c_int as libc::c_longlong {
                dosync = 1 as libc::c_int;
                _sfwrsync();
            }
            if (*f).extent >= 0 as libc::c_int as libc::c_longlong
                && (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
            {
                if (*f).flags as libc::c_int & 0o4000 as libc::c_int == 0 {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    (*f).here = sfsk(f, (*f).here, 0 as libc::c_int, dc);
                } else {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    (*f).here = sfsk(
                        f,
                        0 as libc::c_int as libc::c_longlong,
                        1 as libc::c_int,
                        dc,
                    );
                }
            }
            oerrno = *__errno_location();
            *__errno_location() = 0 as libc::c_int;
            if !dc.is_null() && ((*dc).readf).is_some() {
                let mut share: libc::c_int = (*f).flags as libc::c_int & 0o100 as libc::c_int;
                if rcrv != 0 {
                    (*f).mode |= rcrv;
                } else {
                    let ref mut fresh7 = (*f).flags;
                    *fresh7 = (*fresh7 as libc::c_int
                        & !(0o100 as libc::c_int) as libc::c_ushort as libc::c_int)
                        as libc::c_ushort;
                }
                let mut dcdown: libc::c_int = (*f).bits as libc::c_int & 0o1000 as libc::c_int;
                let ref mut fresh8 = (*f).bits;
                *fresh8 = (*fresh8 as libc::c_int | 0o1000 as libc::c_int) as libc::c_ushort;
                r = (Some(((*dc).readf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f, buf, n, dc)
                    as libc::c_longlong;
                if dcdown == 0 {
                    let ref mut fresh9 = (*f).bits;
                    *fresh9 = (*fresh9 as libc::c_int
                        & !(0o1000 as libc::c_int) as libc::c_ushort as libc::c_int)
                        as libc::c_ushort;
                }
                if rcrv != 0 {
                    (*f).mode &= !rcrv;
                } else {
                    let ref mut fresh10 = (*f).flags;
                    *fresh10 = (*fresh10 as libc::c_int | share) as libc::c_ushort;
                }
            } else if (*f).extent < 0 as libc::c_int as libc::c_longlong
                && (*f).bits as libc::c_int & 0o10 as libc::c_int != 0
            {
                r = 0 as libc::c_int as libc::c_longlong;
            } else if (*f).extent < 0 as libc::c_int as libc::c_longlong
                && (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
                && rcrv != 0
            {
                r = sfpkrd(
                    (*f).file as libc::c_int,
                    buf,
                    n,
                    if rcrv & 0o10 as libc::c_uint != 0 {
                        (*f).getr as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    },
                    -(1 as libc::c_long),
                    if rcrv & 0o20 as libc::c_uint != 0 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                ) as libc::c_longlong;
                if r > 0 as libc::c_int as libc::c_longlong {
                    if rcrv & 0o20 as libc::c_uint != 0 {
                        (*f).mode |= 0o1000 as libc::c_uint;
                    } else {
                        (*f).mode |= 0o10 as libc::c_uint;
                    }
                }
            } else {
                r = read((*f).file as libc::c_int, buf, n) as libc::c_longlong;
            }
            if *__errno_location() == 0 as libc::c_int {
                *__errno_location() = oerrno;
            }
            if r > 0 as libc::c_int as libc::c_longlong {
                if (*f).bits as libc::c_int & 0o1000 as libc::c_int == 0 {
                    if (*f).mode & 0o1000 as libc::c_uint == 0 {
                        (*f).here += r;
                        if (*f).extent >= 0 as libc::c_int as libc::c_longlong
                            && (*f).extent < (*f).here
                        {
                            (*f).extent = (*f).here;
                        }
                    }
                    if buf as *mut libc::c_uchar >= (*f).data
                        && (buf as *mut libc::c_uchar) < ((*f).data).offset((*f).size as isize)
                    {
                        let ref mut fresh11 = (*f).endr;
                        *fresh11 = (buf as *mut libc::c_uchar).offset(r as isize);
                        let ref mut fresh12 = (*f).endb;
                        *fresh12 = *fresh11;
                    }
                }
                return r as ssize_t;
            }
        }
        if local != 0 {
            (*f).mode |= 0o100000 as libc::c_uint;
        }
        match _sfexcept(f, 0o1 as libc::c_int, r as ssize_t, dc) {
            0 => {
                n = (if local != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    r as ssize_t
                }) as size_t;
                return n as ssize_t;
            }
            1 => {
                if local == 0 && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0 {
                    current_block = 7430944104800206518;
                } else {
                    current_block = 13908603829010809519;
                }
            }
            2 => {
                current_block = 13908603829010809519;
            }
            3 | _ => {
                current_block = 7430944104800206518;
            }
        }
        match current_block {
            13908603829010809519 => return -(1 as libc::c_int) as ssize_t,
            _ => {
                dc = (*f).disc;
                while !dc.is_null() {
                    if dc == disc {
                        break;
                    }
                    dc = (*dc).disc;
                }
                disc = dc;
            }
        }
    }
}
