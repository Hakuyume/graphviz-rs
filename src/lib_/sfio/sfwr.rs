#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    fn _sfexcept(
        _: *mut Sfio_t,
        _: libc::c_int,
        _: ssize_t,
        _: *mut Sfdisc_t,
    ) -> libc::c_int;
    fn sfsk(
        _: *mut Sfio_t,
        _: libc::c_longlong,
        _: libc::c_int,
        _: *mut Sfdisc_t,
    ) -> libc::c_longlong;
    fn __errno_location() -> *mut libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    static mut _Sfextern: Sfextern_t;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
}
pub type __ssize_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
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
unsafe extern "C" fn sfoutput(
    mut f: *mut Sfio_t,
    mut buf: *mut libc::c_char,
    mut n: size_t,
) -> ssize_t {
    let mut current_block: u64;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: ssize_t = 0;
    let mut w: ssize_t = 0;
    let mut wr: ssize_t = 0;
    w = 0 as libc::c_int as ssize_t;
    s = w;
    wbuf = buf;
    endbuf = buf.offset(n as isize);
    while n > 0 as libc::c_int as libc::c_ulong {
        if (n as ssize_t) < _Sfextern.sf_page {
            buf = buf.offset(n as isize);
            n = 0 as libc::c_int as size_t;
            s = n as ssize_t;
        } else {
            while n as ssize_t >= _Sfextern.sf_page {
                sp = buf.offset(1 as libc::c_int as isize);
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                    && *buf
                        .offset(
                            (_Sfextern.sf_page - 1 as libc::c_int as libc::c_long)
                                as isize,
                        ) as libc::c_int == 0 as libc::c_int
                {
                    loop {
                        if !((sp as uintptr_t)
                            .wrapping_rem(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ) != 0)
                        {
                            current_block = 12599329904712511516;
                            break;
                        }
                        if *sp as libc::c_int != 0 as libc::c_int {
                            current_block = 430059253837001076;
                            break;
                        }
                        sp = sp.offset(1 as libc::c_int as isize);
                    }
                    match current_block {
                        430059253837001076 => {}
                        _ => {
                            loop {
                                if !(sp < endbuf) {
                                    current_block = 17833034027772472439;
                                    break;
                                }
                                if *(sp as *mut libc::c_int) != 0 as libc::c_int {
                                    current_block = 430059253837001076;
                                    break;
                                }
                                sp = sp
                                    .offset(
                                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as isize,
                                    );
                            }
                            match current_block {
                                430059253837001076 => {}
                                _ => {
                                    if sp > endbuf {
                                        sp = sp
                                            .offset(
                                                -(::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                    as isize),
                                            );
                                        while sp < endbuf {
                                            if *sp as libc::c_int != 0 as libc::c_int {
                                                break;
                                            }
                                            sp = sp.offset(1 as libc::c_int as isize);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                s = sp.offset_from(buf) as libc::c_long;
                if s >= _Sfextern.sf_page {
                    break;
                } else {
                    n = (n as libc::c_ulong)
                        .wrapping_sub(_Sfextern.sf_page as libc::c_ulong) as size_t
                        as size_t;
                    buf = buf.offset(_Sfextern.sf_page as isize);
                }
            }
        }
        if buf > wbuf {
            if (n as ssize_t) < _Sfextern.sf_page {
                buf = endbuf;
                s = 0 as libc::c_int as ssize_t;
                n = s as size_t;
            }
            wr = write(
                (*f).file as libc::c_int,
                wbuf as *const libc::c_void,
                buf.offset_from(wbuf) as libc::c_long as size_t,
            );
            if wr > 0 as libc::c_int as libc::c_long {
                w += wr;
                let ref mut fresh0 = (*f).bits;
                *fresh0 = (*fresh0 as libc::c_int
                    & !(0o4 as libc::c_int) as libc::c_ushort as libc::c_int)
                    as libc::c_ushort;
            }
            if wr != buf.offset_from(wbuf) as libc::c_long {
                break;
            }
            wbuf = buf;
        }
        if !(s >= _Sfextern.sf_page) {
            continue;
        }
        s = s / _Sfextern.sf_page * _Sfextern.sf_page;
        (*f).mode |= 0o100000 as libc::c_uint;
        if sfsk(f, s as libc::c_longlong, 1 as libc::c_int, 0 as *mut Sfdisc_t)
            < 0 as libc::c_int as libc::c_longlong
        {
            break;
        }
        w += s;
        n = (n as libc::c_ulong).wrapping_sub(s as libc::c_ulong) as size_t as size_t;
        buf = buf.offset(s as isize);
        wbuf = buf;
        let ref mut fresh1 = (*f).bits;
        *fresh1 = (*fresh1 as libc::c_int | 0o4 as libc::c_int) as libc::c_ushort;
        if n > 0 as libc::c_int as libc::c_ulong {
            s = if n as ssize_t <= _Sfextern.sf_page {
                1 as libc::c_int as libc::c_long
            } else {
                _Sfextern.sf_page
            };
            buf = buf.offset(s as isize);
            n = (n as libc::c_ulong).wrapping_sub(s as libc::c_ulong) as size_t
                as size_t;
        }
    }
    return if w > 0 as libc::c_int as libc::c_long {
        w
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
#[no_mangle]
pub unsafe extern "C" fn sfwr(
    mut f: *mut Sfio_t,
    mut buf: *const libc::c_void,
    mut n: size_t,
    mut disc: *mut Sfdisc_t,
) -> ssize_t {
    let mut current_block: u64;
    let mut w: ssize_t = 0;
    let mut dc: *mut Sfdisc_t = 0 as *mut Sfdisc_t;
    let mut local: libc::c_int = 0;
    let mut oerrno: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    if local == 0 && (*f).bits as libc::c_int & 0o1000 as libc::c_int == 0 {
        if (*f).mode != 0o2 as libc::c_int as libc::c_uint
            && _sfmode(f, 0o2 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        if (*f).next > (*f).data
            && {
                (*f).mode |= 0o100000 as libc::c_uint;
                sfsync(f) < 0 as libc::c_int
            }
        {
            return -(1 as libc::c_int) as ssize_t;
        }
    }
    loop {
        if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
            && ((*f).file as libc::c_int) < 0 as libc::c_int
        {
            return 0 as libc::c_int as ssize_t;
        }
        let ref mut fresh2 = (*f).flags;
        *fresh2 = (*fresh2 as libc::c_int
            & !(0o200 as libc::c_int | 0o400 as libc::c_int) as libc::c_ushort
                as libc::c_int) as libc::c_ushort;
        dc = disc;
        if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
            w = n
                .wrapping_add(
                    ((*f).next).offset_from((*f).data) as libc::c_long as libc::c_ulong,
                ) as ssize_t;
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
            while !d.is_null() && ((*d).writef).is_none() {
                d = (*d).disc;
            }
            if !d.is_null() {
                dc = d;
            }
            if !dc.is_null() && ((*dc).exceptf).is_some()
                && (*f).flags as libc::c_int & 0o2000 as libc::c_int != 0
            {
                let mut rv: libc::c_int = 0;
                if local != 0 {
                    (*f).mode |= 0o100000 as libc::c_uint;
                }
                rv = _sfexcept(f, 0o2 as libc::c_int, n as ssize_t, dc);
                if rv > 0 as libc::c_int {
                    n = rv as size_t;
                } else if rv < 0 as libc::c_int {
                    let ref mut fresh3 = (*f).flags;
                    *fresh3 = (*fresh3 as libc::c_int | 0o400 as libc::c_int)
                        as libc::c_ushort;
                    return rv as ssize_t;
                }
            }
            if (*f).extent >= 0 as libc::c_int as libc::c_longlong {
                if (*f).flags as libc::c_int & 0o10 as libc::c_int != 0 {
                    if (*f).here != (*f).extent
                        || (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
                    {
                        (*f).mode |= 0o100000 as libc::c_uint;
                        (*f)
                            .here = sfsk(
                            f,
                            0 as libc::c_int as libc::c_longlong,
                            2 as libc::c_int,
                            dc,
                        );
                        (*f).extent = (*f).here;
                    }
                } else if (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
                        && (*f).flags as libc::c_int & 0o4000 as libc::c_int == 0
                    {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    (*f).here = sfsk(f, (*f).here, 0 as libc::c_int, dc);
                }
            }
            oerrno = *__errno_location();
            *__errno_location() = 0 as libc::c_int;
            if !dc.is_null() && ((*dc).writef).is_some() {
                let mut dcdown: libc::c_int = (*f).bits as libc::c_int
                    & 0o1000 as libc::c_int;
                let ref mut fresh4 = (*f).bits;
                *fresh4 = (*fresh4 as libc::c_int | 0o1000 as libc::c_int)
                    as libc::c_ushort;
                w = (Some(((*dc).writef).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f, buf, n, dc);
                if dcdown == 0 {
                    let ref mut fresh5 = (*f).bits;
                    *fresh5 = (*fresh5 as libc::c_int
                        & !(0o1000 as libc::c_int) as libc::c_ushort as libc::c_int)
                        as libc::c_ushort;
                }
            } else if (*f).extent < 0 as libc::c_int as libc::c_longlong
                    && (*f).bits as libc::c_int & 0o10 as libc::c_int != 0
                {
                w = n as ssize_t;
            } else {
                if (*f).flags as libc::c_int & 0o20000 as libc::c_int != 0 {
                    current_block = 11863087442525386961;
                } else if n as ssize_t >= _Sfextern.sf_page
                        && (*f).flags as libc::c_int
                            & (0o100 as libc::c_int | 0o10 as libc::c_int) == 0
                        && (*f).here == (*f).extent
                        && (*f).here % _Sfextern.sf_page as libc::c_longlong
                            == 0 as libc::c_int as libc::c_longlong
                    {
                    w = sfoutput(f, buf as *mut libc::c_char, n);
                    if w <= 0 as libc::c_int as libc::c_long {
                        current_block = 11863087442525386961;
                    } else {
                        current_block = 10853015579903106591;
                    }
                } else {
                    current_block = 11863087442525386961;
                }
                match current_block {
                    10853015579903106591 => {}
                    _ => {
                        w = write((*f).file as libc::c_int, buf, n);
                        if w > 0 as libc::c_int as libc::c_long {
                            let ref mut fresh6 = (*f).bits;
                            *fresh6 = (*fresh6 as libc::c_int
                                & !(0o4 as libc::c_int) as libc::c_ushort as libc::c_int)
                                as libc::c_ushort;
                        }
                    }
                }
            }
            if *__errno_location() == 0 as libc::c_int {
                *__errno_location() = oerrno;
            }
            if w > 0 as libc::c_int as libc::c_long {
                if (*f).bits as libc::c_int & 0o1000 as libc::c_int == 0 {
                    if (*f).flags as libc::c_int
                        & (0o10 as libc::c_int | 0o4000 as libc::c_int) != 0
                    {
                        (*f).mode |= 0o100000 as libc::c_uint;
                        (*f)
                            .here = sfsk(
                            f,
                            0 as libc::c_int as libc::c_longlong,
                            1 as libc::c_int,
                            dc,
                        );
                    } else {
                        (*f).here += w as libc::c_longlong;
                    }
                    if (*f).extent >= 0 as libc::c_int as libc::c_longlong
                        && (*f).here > (*f).extent
                    {
                        (*f).extent = (*f).here;
                    }
                }
                return w;
            }
        }
        if local != 0 {
            (*f).mode |= 0o100000 as libc::c_uint;
        }
        match _sfexcept(f, 0o2 as libc::c_int, w, dc) {
            0 => {
                w = if local != 0 { 0 as libc::c_int as libc::c_long } else { w };
                return w;
            }
            1 => {
                if local == 0 && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0 {
                    current_block = 17209826222385471788;
                } else {
                    current_block = 6100027559543931881;
                }
            }
            2 => {
                current_block = 6100027559543931881;
            }
            3 | _ => {
                current_block = 17209826222385471788;
            }
        }
        match current_block {
            6100027559543931881 => return -(1 as libc::c_int) as ssize_t,
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
    };
}
