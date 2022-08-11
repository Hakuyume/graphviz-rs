#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sfswap(_: *mut Sfio_t, _: *mut Sfio_t) -> *mut Sfio_t;
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
#[no_mangle]
pub unsafe extern "C" fn sfstack(
    mut f1: *mut Sfio_t,
    mut f2: *mut Sfio_t,
) -> *mut Sfio_t {
    let mut n: libc::c_int = 0;
    let mut rf: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut rsrv: *mut Sfrsrv_t = 0 as *mut Sfrsrv_t;
    if !f1.is_null()
        && (*f1).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
            != (*f1).mode
        && _sfmode(f1, 0 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
    {
        return 0 as *mut Sfio_t;
    }
    if !f2.is_null()
        && (*f2).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
            != (*f2).mode
        && _sfmode(f2, 0 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
    {
        return 0 as *mut Sfio_t;
    }
    if f1.is_null() {
        return f2;
    }
    _Sfextern
        .sf_stack = Some(
        sfstack as unsafe extern "C" fn(*mut Sfio_t, *mut Sfio_t) -> *mut Sfio_t,
    );
    if f2.is_null() {
        f2 = (*f1).push;
        if f2.is_null() {
            return 0 as *mut Sfio_t;
        }
        (*f2).mode &= !(0o100 as libc::c_uint);
    } else {
        if !((*f2).push).is_null() {
            return 0 as *mut Sfio_t;
        }
        if !((*f1).pool).is_null()
            && (*f1).pool != &mut _Sfextern.sf_pool as *mut _sfpool_s
            && (*f1).pool != (*f2).pool
            && f1 == *((*(*f1).pool).sf).offset(0 as libc::c_int as isize)
        {
            n = 1 as libc::c_int;
            while n < (*(*f1).pool).n_sf {
                if if (**((*(*f1).pool).sf).offset(n as isize)).mode
                    & (0o100 as libc::c_uint | 0o40 as libc::c_uint
                        | 0o400 as libc::c_uint) != 0
                {
                    1 as libc::c_int
                } else if (**((*(*f1).pool).sf).offset(n as isize)).mode
                        & 0o10000 as libc::c_uint != 0
                    {
                    (Some((_Sfextern.sf_stdsync).expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(*((*(*f1).pool).sf).offset(n as isize))
                } else {
                    0 as libc::c_int
                } != 0
                {
                    n += 1;
                } else {
                    (_Sfextern.sf_pmove)
                        .expect(
                            "non-null function pointer",
                        )(*((*(*f1).pool).sf).offset(n as isize), 0 as libc::c_int);
                    break;
                }
            }
        }
    }
    if !((*f2).pool).is_null() && (*f2).pool != &mut _Sfextern.sf_pool as *mut _sfpool_s
        && f2 != *((*(*f2).pool).sf).offset(0 as libc::c_int as isize)
    {
        (_Sfextern.sf_pmove).expect("non-null function pointer")(f2, 0 as libc::c_int);
    }
    sfswap(f1, f2);
    rsrv = (*f1).rsrv;
    let ref mut fresh0 = (*f1).rsrv;
    *fresh0 = (*f2).rsrv;
    let ref mut fresh1 = (*f2).rsrv;
    *fresh1 = rsrv;
    (*f1).mode |= 0o40 as libc::c_uint;
    let ref mut fresh2 = (*f1).endw;
    *fresh2 = (*f1).data;
    let ref mut fresh3 = (*f1).endr;
    *fresh3 = *fresh2;
    (*f2).mode |= 0o40 as libc::c_uint;
    let ref mut fresh4 = (*f2).endw;
    *fresh4 = (*f2).data;
    let ref mut fresh5 = (*f2).endr;
    *fresh5 = *fresh4;
    if (*f2).push != f2 {
        (*f2).mode |= 0o100 as libc::c_uint;
        let ref mut fresh6 = (*f1).push;
        *fresh6 = f2;
        rf = f1;
    } else {
        (*f1).mode &= !(0o100 as libc::c_uint);
        let ref mut fresh7 = (*f2).push;
        *fresh7 = 0 as *mut Sfio_t;
        rf = f2;
    }
    if 0 as libc::c_int != 0 {} else {
        (*f1).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f1).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh8 = (*f1).endr;
            *fresh8 = (*f1).endb;
        } else {
            if (*f1).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh9 = (*f1).endw;
                *fresh9 = (if (*f1).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f1).data
                } else {
                    (*f1).endb
                });
            } else {
                let ref mut fresh10 = (*f1).endr;
                *fresh10 = (*f1).data;
                let ref mut fresh11 = (*f1).endw;
                *fresh11 = *fresh10;
            };
        };
    };
    if 0 as libc::c_int != 0 {} else {
        (*f2).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f2).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh12 = (*f2).endr;
            *fresh12 = (*f2).endb;
        } else {
            if (*f2).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh13 = (*f2).endw;
                *fresh13 = (if (*f2).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f2).data
                } else {
                    (*f2).endb
                });
            } else {
                let ref mut fresh14 = (*f2).endr;
                *fresh14 = (*f2).data;
                let ref mut fresh15 = (*f2).endw;
                *fresh15 = *fresh14;
            };
        };
    };
    return rf;
}
