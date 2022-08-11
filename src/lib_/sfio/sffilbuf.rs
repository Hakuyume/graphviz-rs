#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sfrd(
        _: *mut Sfio_t,
        _: *mut libc::c_void,
        _: size_t,
        _: *mut Sfdisc_t,
    ) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[no_mangle]
pub unsafe extern "C" fn _sffilbuf(
    mut f: *mut Sfio_t,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut r: ssize_t = 0;
    let mut first: libc::c_int = 0;
    let mut local: libc::c_int = 0;
    let mut rcrv: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut justseek: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    rcrv = ((*f).mode
        & (0o10 as libc::c_uint | 0o20 as libc::c_uint | 0o40 as libc::c_uint))
        as libc::c_int;
    rc = (*f).getr as libc::c_int;
    justseek = (*f).bits as libc::c_int & 0o40 as libc::c_int;
    let ref mut fresh0 = (*f).bits;
    *fresh0 = (*fresh0 as libc::c_int
        & !(0o40 as libc::c_int) as libc::c_ushort as libc::c_int) as libc::c_ushort;
    first = 1 as libc::c_int;
    loop {
        if (*f).mode
            & !(0o20 as libc::c_uint | 0o10 as libc::c_uint
                | (if local != 0 {
                    0o40 as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                })) != 0o1 as libc::c_int as libc::c_uint
            && _sfmode(f, 0o1 as libc::c_int, local) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        (*f).mode |= 0o40 as libc::c_uint;
        let ref mut fresh1 = (*f).endw;
        *fresh1 = (*f).data;
        let ref mut fresh2 = (*f).endr;
        *fresh2 = *fresh1;
        r = ((*f).endb).offset_from((*f).next) as libc::c_long;
        if r > 0 as libc::c_int as libc::c_long {
            if first != 0 && n <= 0 as libc::c_int
                || first == 0 && n as libc::c_long <= r
                || (*f).flags as libc::c_int & 0o4 as libc::c_int != 0
            {
                break;
            }
            if (*f).bits as libc::c_int & 0o1 as libc::c_int == 0
                && (*f).next > (*f).data
                && n as libc::c_long
                    > (*f).size - ((*f).endb).offset_from((*f).data) as libc::c_long
            {
                memcpy(
                    (*f).data as *mut libc::c_void,
                    (*f).next as *const libc::c_void,
                    r as size_t,
                );
                let ref mut fresh3 = (*f).next;
                *fresh3 = (*f).data;
                let ref mut fresh4 = (*f).endb;
                *fresh4 = ((*f).data).offset(r as isize);
            }
        } else if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
                && (*f).bits as libc::c_int & 0o1 as libc::c_int == 0
            {
            let ref mut fresh5 = (*f).endr;
            *fresh5 = (*f).data;
            let ref mut fresh6 = (*f).endb;
            *fresh6 = *fresh5;
            let ref mut fresh7 = (*f).next;
            *fresh7 = *fresh6;
        }
        if (*f).bits as libc::c_int & 0o1 as libc::c_int != 0 {
            r = if n > 0 as libc::c_int { n as libc::c_long } else { (*f).size };
        } else if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0 {
            r = (*f).size - ((*f).endb).offset_from((*f).data) as libc::c_long;
            if n > 0 as libc::c_int {
                if r > n as libc::c_long
                    && (*f).extent < 0 as libc::c_int as libc::c_longlong
                    && (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
                {
                    r = n as ssize_t;
                } else if justseek != 0 && n as size_t <= (*f).iosz
                        && (*f).iosz <= (*f).size as libc::c_ulong
                    {
                    r = (*f).iosz as ssize_t;
                }
            }
        }
        (*f).mode |= rcrv as libc::c_uint;
        (*f).getr = rc as libc::c_uchar;
        (*f).mode |= 0o100000 as libc::c_uint;
        r = sfrd(f, (*f).endb as *mut libc::c_void, r as size_t, (*f).disc);
        if r >= 0 as libc::c_int as libc::c_long {
            r = ((*f).endb).offset_from((*f).next) as libc::c_long;
            break;
        } else {
            first = 0 as libc::c_int;
            (*f).mode &= !(0o40 as libc::c_uint);
        }
    }
    if local != 0 {} else {
        (*f).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
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
    rcrv = if n == 0 as libc::c_int {
        if r > 0 as libc::c_int as libc::c_long {
            let ref mut fresh12 = (*f).next;
            let fresh13 = *fresh12;
            *fresh12 = (*fresh12).offset(1);
            *fresh13 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }
    } else {
        r as libc::c_int
    };
    return rcrv;
}
