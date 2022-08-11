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
    static mut sfstdin: *mut Sfio_t;
    static mut sfstdout: *mut Sfio_t;
    static mut sfstderr: *mut Sfio_t;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut _Sfextern: Sfextern_t;
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
pub unsafe extern "C" fn sfswap(mut f1: *mut Sfio_t, mut f2: *mut Sfio_t) -> *mut Sfio_t {
    let mut tmp: Sfio_t = Sfio_t {
        next: 0 as *mut libc::c_uchar,
        endw: 0 as *mut libc::c_uchar,
        endr: 0 as *mut libc::c_uchar,
        endb: 0 as *mut libc::c_uchar,
        push: 0 as *mut Sfio_t,
        flags: 0,
        file: 0,
        data: 0 as *mut libc::c_uchar,
        size: 0,
        val: 0,
        extent: 0,
        here: 0,
        getr: 0,
        tiny: [0; 1],
        bits: 0,
        mode: 0,
        disc: 0 as *mut _sfdisc_s,
        pool: 0 as *mut _sfpool_s,
        rsrv: 0 as *mut _sfrsrv_s,
        proc_0: 0 as *mut _sfproc_s,
        stdio: 0 as *mut libc::c_void,
        lpos: 0,
        iosz: 0,
    };
    let mut f1pool: libc::c_int = 0;
    let mut f2pool: libc::c_int = 0;
    let mut f1mode: libc::c_int = 0;
    let mut f1flags: libc::c_int = 0;
    let mut f2flags: libc::c_int = 0;
    let mut f2mode: libc::c_uint = 0;
    if f1.is_null()
        || (*f1).mode & 0o20000 as libc::c_uint != 0
        || (if (*f1).mode & (0o100 as libc::c_uint | 0o40 as libc::c_uint | 0o400 as libc::c_uint)
            != 0
        {
            1 as libc::c_int
        } else {
            (if (*f1).mode & 0o10000 as libc::c_uint != 0 {
                (Some((_Sfextern.sf_stdsync).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f1)
            } else {
                0 as libc::c_int
            })
        }) != 0
            && (*f1).mode & 0o100 as libc::c_uint != 0
    {
        return 0 as *mut Sfio_t;
    }
    if !f2.is_null()
        && (if (*f2).mode & (0o100 as libc::c_uint | 0o40 as libc::c_uint | 0o400 as libc::c_uint)
            != 0
        {
            1 as libc::c_int
        } else {
            (if (*f2).mode & 0o10000 as libc::c_uint != 0 {
                (Some((_Sfextern.sf_stdsync).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f2)
            } else {
                0 as libc::c_int
            })
        }) != 0
        && (*f2).mode & 0o100 as libc::c_uint != 0
    {
        return 0 as *mut Sfio_t;
    }
    if f1 == f2 {
        return f2;
    }
    f1mode = (*f1).mode as libc::c_int;
    (*f1).mode |= 0o40 as libc::c_uint;
    let ref mut fresh0 = (*f1).endw;
    *fresh0 = (*f1).data;
    let ref mut fresh1 = (*f1).endr;
    *fresh1 = *fresh0;
    (*f1).mode |= 0o100 as libc::c_uint;
    if !f2.is_null() {
        f2mode = (*f2).mode;
        (*f2).mode |= 0o40 as libc::c_uint;
        let ref mut fresh2 = (*f2).endw;
        *fresh2 = (*f2).data;
        let ref mut fresh3 = (*f2).endr;
        *fresh3 = *fresh2;
        (*f2).mode |= 0o100 as libc::c_uint;
    } else {
        f2 = if (*f1).file as libc::c_int == 0 as libc::c_int {
            sfstdin
        } else if (*f1).file as libc::c_int == 1 as libc::c_int {
            sfstdout
        } else if (*f1).file as libc::c_int == 2 as libc::c_int {
            sfstderr
        } else {
            0 as *mut Sfio_t
        };
        if f2.is_null() || (*f2).mode & 0o20000 as libc::c_uint == 0 {
            f2 = malloc(::std::mem::size_of::<Sfio_t>() as libc::c_ulong) as *mut Sfio_t;
            if f2.is_null() {
                (*f1).mode = f1mode as libc::c_uint;
                if 0 as libc::c_int != 0 {
                } else {
                    (*f1).mode &=
                        !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
                    if (*f1).mode == 0o1 as libc::c_int as libc::c_uint {
                        let ref mut fresh4 = (*f1).endr;
                        *fresh4 = (*f1).endb;
                    } else {
                        if (*f1).mode == 0o2 as libc::c_int as libc::c_uint {
                            let ref mut fresh5 = (*f1).endw;
                            *fresh5 = (if (*f1).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                                (*f1).data
                            } else {
                                (*f1).endb
                            });
                        } else {
                            let ref mut fresh6 = (*f1).endr;
                            *fresh6 = (*f1).data;
                            let ref mut fresh7 = (*f1).endw;
                            *fresh7 = *fresh6;
                        };
                    };
                };
                return 0 as *mut Sfio_t;
            }
            let ref mut fresh8 = (*f2).next;
            *fresh8 = 0 as *mut libc::c_uchar;
            let ref mut fresh9 = (*f2).endw;
            *fresh9 = 0 as *mut libc::c_uchar;
            let ref mut fresh10 = (*f2).endr;
            *fresh10 = 0 as *mut libc::c_uchar;
            let ref mut fresh11 = (*f2).endb;
            *fresh11 = 0 as *mut libc::c_uchar;
            let ref mut fresh12 = (*f2).push;
            *fresh12 = 0 as *mut Sfio_t;
            (*f2).flags = 0 as libc::c_int as libc::c_ushort;
            (*f2).file = -(1 as libc::c_int) as libc::c_short;
            let ref mut fresh13 = (*f2).data;
            *fresh13 = 0 as *mut libc::c_uchar;
            (*f2).size = -(1 as libc::c_int) as ssize_t;
            (*f2).val = -(1 as libc::c_int) as ssize_t;
            (*f2).extent = -(1 as libc::c_int) as libc::c_longlong;
            (*f2).here = 0 as libc::c_int as libc::c_longlong;
            (*f2).getr = 0 as libc::c_int as libc::c_uchar;
            (*f2).tiny[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            (*f2).bits = 0 as libc::c_int as libc::c_ushort;
            (*f2).mode = 0 as libc::c_int as libc::c_uint;
            let ref mut fresh14 = (*f2).disc;
            *fresh14 = 0 as *mut _sfdisc_s;
            let ref mut fresh15 = (*f2).pool;
            *fresh15 = 0 as *mut _sfpool_s;
            let ref mut fresh16 = (*f2).rsrv;
            *fresh16 = 0 as *mut _sfrsrv_s;
            let ref mut fresh17 = (*f2).proc_0;
            *fresh17 = 0 as *mut _sfproc_s;
            let ref mut fresh18 = (*f2).stdio;
            *fresh18 = 0 as *mut libc::c_void;
            (*f2).lpos = 0 as libc::c_int as libc::c_longlong;
            (*f2).iosz = 0 as libc::c_int as size_t;
        }
        (*f2).mode = 0o20000 as libc::c_uint | 0o40 as libc::c_uint;
        f2mode = 0o20000 as libc::c_uint;
    }
    if ((*f1).pool).is_null() {
        f1pool = -(1 as libc::c_int);
    } else {
        f1pool = (*(*f1).pool).n_sf - 1 as libc::c_int;
        while f1pool >= 0 as libc::c_int {
            if *((*(*f1).pool).sf).offset(f1pool as isize) == f1 {
                break;
            }
            f1pool -= 1;
        }
    }
    if ((*f2).pool).is_null() {
        f2pool = -(1 as libc::c_int);
    } else {
        f2pool = (*(*f2).pool).n_sf - 1 as libc::c_int;
        while f2pool >= 0 as libc::c_int {
            if *((*(*f2).pool).sf).offset(f2pool as isize) == f2 {
                break;
            }
            f2pool -= 1;
        }
    }
    f1flags = (*f1).flags as libc::c_int;
    f2flags = (*f2).flags as libc::c_int;
    tmp = *f1;
    *f1 = *f2;
    *f2 = tmp;
    if f2pool >= 0 as libc::c_int {
        let ref mut fresh19 = *((*(*f1).pool).sf).offset(f2pool as isize);
        *fresh19 = f1;
    }
    if f1pool >= 0 as libc::c_int {
        let ref mut fresh20 = *((*(*f2).pool).sf).offset(f1pool as isize);
        *fresh20 = f2;
    }
    if f2flags & 0o1000 as libc::c_int != 0 {
        let ref mut fresh21 = (*f2).flags;
        *fresh21 = (*fresh21 as libc::c_int | 0o1000 as libc::c_int) as libc::c_ushort;
    } else {
        let ref mut fresh22 = (*f2).flags;
        *fresh22 = (*fresh22 as libc::c_int
            & !(0o1000 as libc::c_int) as libc::c_ushort as libc::c_int)
            as libc::c_ushort;
    }
    if f1flags & 0o1000 as libc::c_int != 0 {
        let ref mut fresh23 = (*f1).flags;
        *fresh23 = (*fresh23 as libc::c_int | 0o1000 as libc::c_int) as libc::c_ushort;
    } else {
        let ref mut fresh24 = (*f1).flags;
        *fresh24 = (*fresh24 as libc::c_int
            & !(0o1000 as libc::c_int) as libc::c_ushort as libc::c_int)
            as libc::c_ushort;
    }
    if f2mode & 0o20000 as libc::c_uint != 0 {
        if (*f1).flags as libc::c_int & 0o1000 as libc::c_int == 0 {
            free(f1 as *mut libc::c_void);
        }
    } else {
        (*f1).mode = f2mode;
        if 0 as libc::c_int != 0 {
        } else {
            (*f1).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
            if (*f1).mode == 0o1 as libc::c_int as libc::c_uint {
                let ref mut fresh25 = (*f1).endr;
                *fresh25 = (*f1).endb;
            } else {
                if (*f1).mode == 0o2 as libc::c_int as libc::c_uint {
                    let ref mut fresh26 = (*f1).endw;
                    *fresh26 = (if (*f1).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                        (*f1).data
                    } else {
                        (*f1).endb
                    });
                } else {
                    let ref mut fresh27 = (*f1).endr;
                    *fresh27 = (*f1).data;
                    let ref mut fresh28 = (*f1).endw;
                    *fresh28 = *fresh27;
                };
            };
        };
    }
    (*f2).mode = f1mode as libc::c_uint;
    if 0 as libc::c_int != 0 {
    } else {
        (*f2).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f2).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh29 = (*f2).endr;
            *fresh29 = (*f2).endb;
        } else {
            if (*f2).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh30 = (*f2).endw;
                *fresh30 = (if (*f2).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f2).data
                } else {
                    (*f2).endb
                });
            } else {
                let ref mut fresh31 = (*f2).endr;
                *fresh31 = (*f2).data;
                let ref mut fresh32 = (*f2).endw;
                *fresh32 = *fresh31;
            };
        };
    };
    return f2;
}
