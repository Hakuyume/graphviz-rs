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
    fn sfsetbuf(_: *mut Sfio_t, _: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
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
pub unsafe extern "C" fn sfdisc(mut f: *mut Sfio_t, mut disc: *mut Sfdisc_t) -> *mut Sfdisc_t {
    let mut current_block: u64;
    let mut d: *mut Sfdisc_t = 0 as *mut Sfdisc_t;
    let mut rdisc: *mut Sfdisc_t = 0 as *mut Sfdisc_t;
    let mut oreadf: Sfread_f = None;
    let mut owritef: Sfwrite_f = None;
    let mut oseekf: Sfseek_f = None;
    let mut n: ssize_t = 0;
    if f.is_null() {
        return 0 as *mut Sfdisc_t;
    }
    if (*f).flags as libc::c_int & 0o1 as libc::c_int != 0
        && !((*f).proc_0).is_null()
        && (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
    {
        if _sfmode(f, 0o1 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int {
            return 0 as *mut Sfdisc_t;
        }
    } else if (*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint != (*f).mode
        && _sfmode(f, 0 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
    {
        return 0 as *mut Sfdisc_t;
    }
    (*f).mode |= 0o40 as libc::c_uint;
    let ref mut fresh0 = (*f).endw;
    *fresh0 = (*f).data;
    let ref mut fresh1 = (*f).endr;
    *fresh1 = *fresh0;
    rdisc = 0 as *mut Sfdisc_t;
    if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0 {
        if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0 && (*f).next > (*f).data
            || (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
            || (*f).disc == &mut _Sfextern.sf_udisc as *mut _sfdisc_s
        {
            (*f).mode |= 0o100000 as libc::c_uint;
            sfsync(f);
        }
        if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0 && {
            n = ((*f).next).offset_from((*f).data) as libc::c_long;
            n > 0 as libc::c_int as libc::c_long
        } || (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
            && (*f).extent < 0 as libc::c_int as libc::c_longlong
            && {
                n = ((*f).endb).offset_from((*f).next) as libc::c_long;
                n > 0 as libc::c_int as libc::c_long
            }
        {
            let mut exceptf: Sfexcept_f = None;
            let mut rv: libc::c_int = 0 as libc::c_int;
            exceptf = if !disc.is_null() {
                (*disc).exceptf
            } else if !((*f).disc).is_null() {
                (*(*f).disc).exceptf
            } else {
                None
            };
            if exceptf.is_some() {
                if 0 as libc::c_int != 0 {
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
                rv = exceptf.expect("non-null function pointer")(
                    f,
                    8 as libc::c_int,
                    &mut n as *mut ssize_t as *mut libc::c_void,
                    if !disc.is_null() { disc } else { (*f).disc },
                );
                (*f).mode |= 0o40 as libc::c_uint;
                let ref mut fresh6 = (*f).endw;
                *fresh6 = (*f).data;
                let ref mut fresh7 = (*f).endr;
                *fresh7 = *fresh6;
            }
            if rv <= 0 as libc::c_int {
                current_block = 16358980986667551981;
            } else {
                current_block = 14359455889292382949;
            }
        } else {
            current_block = 14359455889292382949;
        }
    } else {
        current_block = 14359455889292382949;
    }
    match current_block {
        14359455889292382949 => {
            d = (*f).disc;
            while !d.is_null() && ((*d).readf).is_none() {
                d = (*d).disc;
            }
            oreadf = if !d.is_null() { (*d).readf } else { None };
            d = (*f).disc;
            while !d.is_null() && ((*d).writef).is_none() {
                d = (*d).disc;
            }
            owritef = if !d.is_null() { (*d).writef } else { None };
            d = (*f).disc;
            while !d.is_null() && ((*d).seekf).is_none() {
                d = (*d).disc;
            }
            oseekf = if !d.is_null() { (*d).seekf } else { None };
            if disc.is_null() {
                d = (*f).disc;
                if d.is_null() {
                    current_block = 16358980986667551981;
                } else {
                    disc = (*d).disc;
                    if ((*d).exceptf).is_some() {
                        if 0 as libc::c_int != 0 {
                        } else {
                            (*f).mode &= !(0o40 as libc::c_uint
                                | 0o10 as libc::c_uint
                                | 0o20 as libc::c_uint);
                            if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                                let ref mut fresh8 = (*f).endr;
                                *fresh8 = (*f).endb;
                            } else {
                                if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                                    let ref mut fresh9 = (*f).endw;
                                    *fresh9 =
                                        (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
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
                        if ((*d).exceptf).expect("non-null function pointer")(
                            f,
                            6 as libc::c_int,
                            disc as *mut libc::c_void,
                            d,
                        ) < 0 as libc::c_int
                        {
                            current_block = 16358980986667551981;
                        } else {
                            (*f).mode |= 0o40 as libc::c_uint;
                            let ref mut fresh12 = (*f).endw;
                            *fresh12 = (*f).data;
                            let ref mut fresh13 = (*f).endr;
                            *fresh13 = *fresh12;
                            current_block = 13826291924415791078;
                        }
                    } else {
                        current_block = 13826291924415791078;
                    }
                    match current_block {
                        16358980986667551981 => {}
                        _ => {
                            let ref mut fresh14 = (*f).disc;
                            *fresh14 = disc;
                            rdisc = d;
                            current_block = 16779030619667747692;
                        }
                    }
                }
            } else {
                loop {
                    d = (*f).disc;
                    if !d.is_null() && ((*d).exceptf).is_some() {
                        if 0 as libc::c_int != 0 {
                        } else {
                            (*f).mode &= !(0o40 as libc::c_uint
                                | 0o10 as libc::c_uint
                                | 0o20 as libc::c_uint);
                            if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                                let ref mut fresh15 = (*f).endr;
                                *fresh15 = (*f).endb;
                            } else {
                                if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                                    let ref mut fresh16 = (*f).endw;
                                    *fresh16 =
                                        (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                                            (*f).data
                                        } else {
                                            (*f).endb
                                        });
                                } else {
                                    let ref mut fresh17 = (*f).endr;
                                    *fresh17 = (*f).data;
                                    let ref mut fresh18 = (*f).endw;
                                    *fresh18 = *fresh17;
                                };
                            };
                        };
                        if ((*d).exceptf).expect("non-null function pointer")(
                            f,
                            5 as libc::c_int,
                            disc as *mut libc::c_void,
                            d,
                        ) < 0 as libc::c_int
                        {
                            current_block = 16358980986667551981;
                            break;
                        }
                        (*f).mode |= 0o40 as libc::c_uint;
                        let ref mut fresh19 = (*f).endw;
                        *fresh19 = (*f).data;
                        let ref mut fresh20 = (*f).endr;
                        *fresh20 = *fresh19;
                    }
                    if !(d != (*f).disc) {
                        current_block = 12930649117290160518;
                        break;
                    }
                }
                match current_block {
                    16358980986667551981 => {}
                    _ => {
                        loop {
                            if d.is_null() {
                                current_block = 1134115459065347084;
                                break;
                            }
                            if d == disc {
                                current_block = 16358980986667551981;
                                break;
                            }
                            d = (*d).disc;
                        }
                        match current_block {
                            16358980986667551981 => {}
                            _ => {
                                let ref mut fresh21 = (*disc).disc;
                                *fresh21 = (*f).disc;
                                let ref mut fresh22 = (*f).disc;
                                *fresh22 = disc;
                                rdisc = disc;
                                current_block = 16779030619667747692;
                            }
                        }
                    }
                }
            }
            match current_block {
                16358980986667551981 => {}
                _ => {
                    if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0 {
                        let mut reinit: libc::c_int = 0 as libc::c_int;
                        if reinit == 0 {
                            d = (*f).disc;
                            while !d.is_null() && ((*d).readf).is_none() {
                                d = (*d).disc;
                            }
                            if (if !d.is_null() { (*d).readf } else { None }) != oreadf {
                                reinit = 1 as libc::c_int;
                            }
                        }
                        if reinit == 0 {
                            d = (*f).disc;
                            while !d.is_null() && ((*d).writef).is_none() {
                                d = (*d).disc;
                            }
                            if (if !d.is_null() { (*d).writef } else { None }) != owritef {
                                reinit = 1 as libc::c_int;
                            }
                        }
                        if reinit == 0 {
                            d = (*f).disc;
                            while !d.is_null() && ((*d).seekf).is_none() {
                                d = (*d).disc;
                            }
                            if (if !d.is_null() { (*d).seekf } else { None }) != oseekf {
                                reinit = 1 as libc::c_int;
                            }
                        }
                        if reinit != 0 {
                            (*f).mode |= 0o100000 as libc::c_uint;
                            let ref mut fresh23 = (*f).bits;
                            *fresh23 = (*fresh23 as libc::c_int
                                & !(0o10 as libc::c_int) as libc::c_ushort as libc::c_int)
                                as libc::c_ushort;
                            if (*f).bits as libc::c_int & 0o1 as libc::c_int != 0
                                || (*f).mode & 0o4 as libc::c_uint != 0
                            {
                                sfsetbuf(
                                    f,
                                    0 as *mut libc::c_void,
                                    18446744073709551615 as libc::c_ulong,
                                );
                            } else if (*f).data == ((*f).tiny).as_mut_ptr() {
                                sfsetbuf(f, 0 as *mut libc::c_void, 0 as libc::c_int as size_t);
                            } else {
                                let mut flags: libc::c_ushort = (*f).flags;
                                sfsetbuf(f, (*f).data as *mut libc::c_void, (*f).size as size_t);
                                let ref mut fresh24 = (*f).flags;
                                *fresh24 = (*fresh24 as libc::c_int
                                    | flags as libc::c_int & 0o20 as libc::c_int)
                                    as libc::c_ushort;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if 0 as libc::c_int != 0 {
    } else {
        (*f).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh25 = (*f).endr;
            *fresh25 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh26 = (*f).endw;
                *fresh26 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh27 = (*f).endr;
                *fresh27 = (*f).data;
                let ref mut fresh28 = (*f).endw;
                *fresh28 = *fresh27;
            };
        };
    };
    return rdisc;
}
