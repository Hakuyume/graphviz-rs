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
    fn sfwr(_: *mut Sfio_t, _: *const libc::c_void, _: size_t, _: *mut Sfdisc_t) -> ssize_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
pub unsafe extern "C" fn _sfflsbuf(mut f: *mut Sfio_t, mut c: libc::c_int) -> libc::c_int {
    let mut n: ssize_t = 0;
    let mut w: ssize_t = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut outc: libc::c_uchar = 0;
    let mut local: libc::c_int = 0;
    let mut isall: libc::c_int = 0;
    let mut inpc: libc::c_int = c;
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    loop {
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
            return -(1 as libc::c_int);
        }
        (*f).mode |= 0o40 as libc::c_uint;
        let ref mut fresh0 = (*f).endw;
        *fresh0 = (*f).data;
        let ref mut fresh1 = (*f).endr;
        *fresh1 = *fresh0;
        data = (*f).data;
        n = ((*f).next).offset_from(data) as libc::c_long;
        if n == ((*f).endb).offset_from(data) as libc::c_long
            && (*f).flags as libc::c_int & 0o4 as libc::c_int != 0
        {
            (*f).mode |= 0o100000 as libc::c_uint;
            sfwr(
                f,
                data as *mut libc::c_void,
                1 as libc::c_int as size_t,
                (*f).disc,
            );
            if (*f).next < (*f).endb || (*f).flags as libc::c_int & 0o4 as libc::c_int == 0 {
                data = (*f).data;
                n = ((*f).next).offset_from(data) as libc::c_long;
            } else {
                if local != 0 {
                } else {
                    (*f).mode &=
                        !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
                    if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                        let ref mut fresh2 = (*f).endr;
                        *fresh2 = (*f).endb;
                    } else {
                        if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                            let ref mut fresh3 = (*f).endw;
                            *fresh3 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
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
        }
        if c >= 0 as libc::c_int {
            data = (*f).data;
            if n < ((*f).endb).offset_from(data) as libc::c_long {
                let ref mut fresh6 = (*f).next;
                let fresh7 = *fresh6;
                *fresh6 = (*fresh6).offset(1);
                *fresh7 = c as libc::c_uchar;
                if !(c == '\n' as i32
                    && (*f).flags as libc::c_int & 0o40 as libc::c_int != 0
                    && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0)
                {
                    break;
                }
                c = -(1 as libc::c_int);
                n += 1 as libc::c_int as libc::c_long;
            } else if n == 0 as libc::c_int as libc::c_long {
                outc = c as libc::c_uchar;
                data = &mut outc;
                c = -(1 as libc::c_int);
                n = 1 as libc::c_int as ssize_t;
            }
        }
        if n == 0 as libc::c_int as libc::c_long
            || (*f).flags as libc::c_int & 0o4 as libc::c_int != 0
        {
            break;
        }
        isall = ((*f).mode & 0o20 as libc::c_uint) as libc::c_int;
        if isall != 0 {
            (*f).mode &= !(0o20 as libc::c_uint);
        } else {
        };
        isall = (isall != 0
            || (*f).extent < 0 as libc::c_int as libc::c_longlong
            || (*f).flags as libc::c_int
                & (0o100 as libc::c_int | 0o10 as libc::c_int | 0o20000 as libc::c_int)
                != 0) as libc::c_int;
        (*f).mode |= 0o100000 as libc::c_uint;
        w = sfwr(f, data as *mut libc::c_void, n as size_t, (*f).disc);
        if w > 0 as libc::c_int as libc::c_long {
            n -= w;
            if n > 0 as libc::c_int as libc::c_long {
                memcpy(
                    (*f).data as *mut libc::c_void,
                    data.offset(w as isize) as *const libc::c_void,
                    n as libc::c_ulong,
                );
            }
            let ref mut fresh8 = (*f).next;
            *fresh8 = ((*f).data).offset(n as isize);
            if c < 0 as libc::c_int && (isall == 0 || n == 0 as libc::c_int as libc::c_long) {
                break;
            }
        } else if w == 0 as libc::c_int as libc::c_long {
            if local != 0 {
            } else {
                (*f).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
                if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                    let ref mut fresh9 = (*f).endr;
                    *fresh9 = (*f).endb;
                } else {
                    if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                        let ref mut fresh10 = (*f).endw;
                        *fresh10 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                            (*f).data
                        } else {
                            (*f).endb
                        });
                    } else {
                        let ref mut fresh11 = (*f).endr;
                        *fresh11 = (*f).data;
                        let ref mut fresh12 = (*f).endw;
                        *fresh12 = *fresh11;
                    };
                };
            };
            return -(1 as libc::c_int);
        } else if c < 0 as libc::c_int {
            break;
        }
        (*f).mode &= !(0o40 as libc::c_uint);
    }
    if local != 0 {
    } else {
        (*f).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh13 = (*f).endr;
            *fresh13 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh14 = (*f).endw;
                *fresh14 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh15 = (*f).endr;
                *fresh15 = (*f).data;
                let ref mut fresh16 = (*f).endw;
                *fresh16 = *fresh15;
            };
        };
    };
    if inpc < 0 as libc::c_int {
        inpc = ((*f).endb).offset_from((*f).next) as libc::c_long as libc::c_int;
    }
    return inpc;
}
