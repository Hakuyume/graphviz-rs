#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sfwr(
        _: *mut Sfio_t,
        _: *const libc::c_void,
        _: size_t,
        _: *mut Sfdisc_t,
    ) -> ssize_t;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
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
#[no_mangle]
pub unsafe extern "C" fn sfwrite(
    mut f: *mut Sfio_t,
    mut buf: *const libc::c_void,
    mut n: size_t,
) -> ssize_t {
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut begs: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut w: ssize_t = 0;
    let mut local: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    if buf.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    if (*f).mode & 0o400 as libc::c_uint != 0 {
        if (*f).mode & 0o2 as libc::c_int as libc::c_uint == 0
            && (*f).flags as libc::c_int & (0o1 as libc::c_int | 0o2 as libc::c_int)
                != 0o1 as libc::c_int | 0o2 as libc::c_int
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        if buf != (*f).next as *const libc::c_void
            && (((*f).rsrv).is_null()
                || ((*(*f).rsrv).data).as_mut_ptr() != buf as *mut libc::c_uchar)
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        (*f).mode &= !(0o400 as libc::c_uint);
        if (*f).mode & 0o1000 as libc::c_uint != 0 {
            let mut buf_0: [libc::c_char; 16] = [0; 16];
            let mut r: ssize_t = 0;
            w = n as ssize_t;
            while w > 0 as libc::c_int as libc::c_long {
                r = w;
                if r as libc::c_ulong
                    > ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                {
                    r = ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                        as ssize_t;
                }
                r = read(
                    (*f).file as libc::c_int,
                    buf_0.as_mut_ptr() as *mut libc::c_void,
                    r as size_t,
                );
                if r <= 0 as libc::c_int as libc::c_long {
                    n = (n as libc::c_ulong).wrapping_sub(w as libc::c_ulong) as size_t
                        as size_t;
                    break;
                } else {
                    w -= r;
                }
            }
            (*f).mode &= !(0o1000 as libc::c_uint);
            let ref mut fresh0 = (*f).endb;
            *fresh0 = ((*f).data).offset(n as isize);
            let ref mut fresh1 = (*f).here;
            *fresh1 = (*fresh1 as libc::c_ulonglong).wrapping_add(n as libc::c_ulonglong)
                as libc::c_longlong as libc::c_longlong;
        }
        if (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
            && !((*f).proc_0).is_null()
        {
            let ref mut fresh2 = (*f).next;
            *fresh2 = (*fresh2).offset(n as isize);
        }
    }
    begs = buf as *mut libc::c_uchar;
    s = begs;
    loop {
        if (*f).mode
            & !(0o20 as libc::c_uint | 0o10 as libc::c_uint
                | (if local != 0 {
                    0o40 as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                })) != 0o2 as libc::c_int as libc::c_uint
            && _sfmode(f, 0o2 as libc::c_int, local) < 0 as libc::c_int
        {
            w = if s > begs {
                s.offset_from(begs) as libc::c_long
            } else {
                -(1 as libc::c_int) as libc::c_long
            };
            return w;
        }
        (*f).mode |= 0o40 as libc::c_uint;
        let ref mut fresh3 = (*f).endw;
        *fresh3 = (*f).data;
        let ref mut fresh4 = (*f).endr;
        *fresh4 = *fresh3;
        w = ((*f).endb).offset_from((*f).next) as libc::c_long;
        if s == (*f).next {
            if w > n as ssize_t {
                w = n as ssize_t;
            }
            s = s.offset(w as isize);
            let ref mut fresh5 = (*f).next;
            *fresh5 = s;
            n = (n as libc::c_ulong).wrapping_sub(w as libc::c_ulong) as size_t
                as size_t;
            break;
        } else {
            if w == 0 as libc::c_int as libc::c_long
                || (*f).flags as libc::c_int & 0o20000 as libc::c_int != 0
                    && w < n as ssize_t
            {
                if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    sfwr(
                        f,
                        s as *mut libc::c_void,
                        n.wrapping_sub(w as libc::c_ulong),
                        (*f).disc,
                    );
                    w = ((*f).endb).offset_from((*f).next) as libc::c_long;
                    if w < n as ssize_t {
                        break;
                    }
                } else if (*f).next > (*f).data {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    _sfflsbuf(f, -(1 as libc::c_int));
                    w = ((*f).endb).offset_from((*f).next) as libc::c_long;
                    if w < n as ssize_t
                        && (*f).flags as libc::c_int & 0o20000 as libc::c_int != 0
                        && (*f).next > (*f).data
                    {
                        break;
                    }
                }
            }
            if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
                && (*f).next == (*f).data
                && (n as ssize_t >= (*f).size
                    || n >= 1024 as libc::c_int as libc::c_ulong
                        && n as ssize_t >= (*f).size / 16 as libc::c_int as libc::c_long)
            {
                (*f).mode |= 0o100000 as libc::c_uint;
                w = sfwr(f, s as *mut libc::c_void, n, (*f).disc);
                if w <= 0 as libc::c_int as libc::c_long {
                    break;
                }
            } else {
                if w > n as ssize_t {
                    w = n as ssize_t;
                }
                if w <= 0 as libc::c_int as libc::c_long {
                    break;
                }
                memcpy(
                    (*f).next as *mut libc::c_void,
                    s as *const libc::c_void,
                    w as libc::c_ulong,
                );
                let ref mut fresh6 = (*f).next;
                *fresh6 = (*fresh6).offset(w as isize);
            }
            s = s.offset(w as isize);
            n = (n as libc::c_ulong).wrapping_sub(w as libc::c_ulong) as size_t
                as size_t;
            if n <= 0 as libc::c_int as libc::c_ulong {
                break;
            }
            (*f).mode &= !(0o40 as libc::c_uint);
        }
    }
    if (*f).extent < 0 as libc::c_int as libc::c_longlong
        && (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
        && (*f).flags as libc::c_int & 0o4000 as libc::c_int == 0
    {
        (*f).mode |= 0o100000 as libc::c_uint;
        _sfflsbuf(f, -(1 as libc::c_int));
    } else if n == 0 as libc::c_int as libc::c_ulong
            && (*f).flags as libc::c_int & 0o40 as libc::c_int != 0
            && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
        {
        n = ((*f).next).offset_from((*f).data) as libc::c_long as size_t;
        w = s.offset_from(begs) as libc::c_long;
        if n as ssize_t > w {
            n = w as size_t;
        }
        if n > 0 as libc::c_int as libc::c_ulong
            && n < 128 as libc::c_int as libc::c_ulong
        {
            next = ((*f).next).offset(-(1 as libc::c_int as isize));
            while n > 0 as libc::c_int as libc::c_ulong {
                if *next as libc::c_int == '\n' as i32 {
                    n = 128 as libc::c_int as size_t;
                    break;
                } else {
                    n = n.wrapping_sub(1);
                    next = next.offset(-1);
                }
            }
        }
        if n >= 128 as libc::c_int as libc::c_ulong {
            (*f).mode |= 0o100000 as libc::c_uint;
            _sfflsbuf(f, -(1 as libc::c_int));
        }
    }
    if local != 0 {} else {
        (*f).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh7 = (*f).endr;
            *fresh7 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh8 = (*f).endw;
                *fresh8 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh9 = (*f).endr;
                *fresh9 = (*f).data;
                let ref mut fresh10 = (*f).endw;
                *fresh10 = *fresh9;
            };
        };
    };
    w = s.offset_from(begs) as libc::c_long;
    return w;
}
