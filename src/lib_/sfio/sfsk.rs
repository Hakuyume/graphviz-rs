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
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
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
pub unsafe extern "C" fn sfsk(
    mut f: *mut Sfio_t,
    mut addr: libc::c_longlong,
    mut type_0: libc::c_int,
    mut disc: *mut Sfdisc_t,
) -> libc::c_longlong {
    let mut p: libc::c_longlong = 0;
    let mut dc: *mut Sfdisc_t = 0 as *mut Sfdisc_t;
    let mut s: ssize_t = 0;
    let mut local: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    if local == 0 && (*f).bits as libc::c_int & 0o1000 as libc::c_int == 0 {
        mode =
            ((*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint) as libc::c_int;
        if mode != (*f).mode as libc::c_int && _sfmode(f, mode, 0 as libc::c_int) < 0 as libc::c_int
        {
            return -(1 as libc::c_int) as libc::c_longlong;
        }
        (*f).mode |= 0o100000 as libc::c_uint;
        if sfsync(f) < 0 as libc::c_int {
            return -(1 as libc::c_int) as libc::c_longlong;
        }
        let ref mut fresh0 = (*f).endw;
        *fresh0 = (*f).data;
        let ref mut fresh1 = (*f).endr;
        *fresh1 = *fresh0;
        let ref mut fresh2 = (*f).endb;
        *fresh2 = *fresh1;
        let ref mut fresh3 = (*f).next;
        *fresh3 = *fresh2;
    }
    type_0 &= 0 as libc::c_int | 1 as libc::c_int | 2 as libc::c_int;
    if type_0 > 2 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    loop {
        dc = disc;
        if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
            let mut s_0: libc::c_longlong =
                ((*f).next).offset_from((*f).data) as libc::c_long as libc::c_longlong;
            if s_0 > (*f).here {
                (*f).here = s_0;
                if s_0 > (*f).extent {
                    (*f).extent = s_0;
                }
            }
            if type_0 == 0 as libc::c_int {
                s = addr as ssize_t;
            } else if type_0 == 1 as libc::c_int {
                s = (addr + (*f).here) as ssize_t;
            } else {
                s = (addr + (*f).extent) as ssize_t;
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
            while !d.is_null() && ((*d).seekf).is_none() {
                d = (*d).disc;
            }
            if !d.is_null() {
                dc = d;
            }
            if !dc.is_null() && ((*dc).seekf).is_some() {
                let mut dcdown: libc::c_int = (*f).bits as libc::c_int & 0o1000 as libc::c_int;
                let ref mut fresh4 = (*f).bits;
                *fresh4 = (*fresh4 as libc::c_int | 0o1000 as libc::c_int) as libc::c_ushort;
                p = (Some(((*dc).seekf).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f, addr, type_0, dc);
                if dcdown == 0 {
                    let ref mut fresh5 = (*f).bits;
                    *fresh5 = (*fresh5 as libc::c_int
                        & !(0o1000 as libc::c_int) as libc::c_ushort as libc::c_int)
                        as libc::c_ushort;
                }
            } else {
                p = lseek((*f).file as libc::c_int, addr as off_t, type_0) as libc::c_longlong;
            }
            if p >= 0 as libc::c_int as libc::c_longlong {
                return p;
            }
            s = -(1 as libc::c_int) as ssize_t;
        }
        if local != 0 {
            (*f).mode |= 0o100000 as libc::c_uint;
        }
        match _sfexcept(f, 3 as libc::c_int, s, dc) {
            1 | 3 => {
                if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
                    return s as libc::c_longlong;
                }
                dc = (*f).disc;
                while !dc.is_null() {
                    if dc == disc {
                        break;
                    }
                    dc = (*dc).disc;
                }
                disc = dc;
            }
            _ => return -(1 as libc::c_int) as libc::c_longlong,
        }
    }
}
