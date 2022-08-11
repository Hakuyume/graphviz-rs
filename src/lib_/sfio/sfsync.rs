#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sfsk(
        _: *mut Sfio_t,
        _: libc::c_longlong,
        _: libc::c_int,
        _: *mut Sfdisc_t,
    ) -> libc::c_longlong;
    fn sfwr(
        _: *mut Sfio_t,
        _: *const libc::c_void,
        _: size_t,
        _: *mut Sfdisc_t,
    ) -> ssize_t;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    static mut _Sfextern: Sfextern_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
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
unsafe extern "C" fn _sfall() -> libc::c_int {
    let mut current_block: u64;
    let mut p: *mut Sfpool_t = 0 as *mut Sfpool_t;
    let mut next: *mut Sfpool_t = 0 as *mut Sfpool_t;
    let mut f: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut n: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut nsync: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut loop_0: libc::c_int = 0;
    loop_0 = 0 as libc::c_int;
    while loop_0 < 3 as libc::c_int {
        count = 0 as libc::c_int;
        nsync = count;
        rv = nsync;
        p = &mut _Sfextern.sf_pool;
        while !p.is_null() {
            next = (*p).next;
            while !next.is_null() {
                if (*next).n_sf > 0 as libc::c_int {
                    break;
                }
                next = (*next).next;
            }
            n = 0 as libc::c_int;
            while n
                < (if p == &mut _Sfextern.sf_pool as *mut _sfpool_s {
                    (*p).n_sf
                } else {
                    1 as libc::c_int
                })
            {
                count += 1 as libc::c_int;
                f = *((*p).sf).offset(n as isize);
                if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
                    current_block = 9445706630460089965;
                } else if if (*f).mode
                        & (0o100 as libc::c_uint | 0o40 as libc::c_uint
                            | 0o400 as libc::c_uint) != 0
                    {
                        1 as libc::c_int
                    } else if (*f).mode & 0o10000 as libc::c_uint != 0 {
                        (Some(
                            (_Sfextern.sf_stdsync).expect("non-null function pointer"),
                        ))
                            .expect("non-null function pointer")(f)
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                    current_block = 7746791466490516765;
                } else if (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
                        && (*f).mode & 0o4000 as libc::c_uint != 0
                    {
                    current_block = 9445706630460089965;
                } else if (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
                        && (*f).bits as libc::c_int & 0o1 as libc::c_int == 0
                        && (*f).next == (*f).endb
                    {
                    current_block = 9445706630460089965;
                } else if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
                        && (*f).bits as libc::c_int & 0o4 as libc::c_int == 0
                        && (*f).next == (*f).data
                    {
                    current_block = 9445706630460089965;
                } else {
                    if sfsync(f) < 0 as libc::c_int {
                        rv = -(1 as libc::c_int);
                    }
                    current_block = 9445706630460089965;
                }
                match current_block {
                    9445706630460089965 => {
                        nsync += 1 as libc::c_int;
                    }
                    _ => {}
                }
                n += 1;
            }
            p = next;
        }
        if nsync == count {
            break;
        }
        loop_0 += 1;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn sfsync(mut f: *mut Sfio_t) -> libc::c_int {
    let mut local: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut origf: *mut Sfio_t = 0 as *mut Sfio_t;
    origf = f;
    if origf.is_null() {
        return _sfall();
    }
    if origf.is_null() {
        return -(1 as libc::c_int);
    }
    local = ((*origf).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*origf).mode &= !(0o100000 as libc::c_uint);
    if (*origf).disc == &mut _Sfextern.sf_udisc as *mut _sfdisc_s {
        sfclose(
            (_Sfextern.sf_stack)
                .expect("non-null function pointer")(origf, 0 as *mut Sfio_t),
        );
    }
    rv = 0 as libc::c_int;
    if (*origf).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
        != (*origf).mode
            & !(0o20 as libc::c_uint | 0o10 as libc::c_uint
                | (if local != 0 {
                    0o40 as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                })) && _sfmode(origf, 0 as libc::c_int, local) < 0 as libc::c_int
    {
        rv = -(1 as libc::c_int);
    } else {
        while !f.is_null() {
            if (*f).flags as libc::c_int & 0o2000 as libc::c_int != 0
                && !((*f).disc).is_null() && ((*(*f).disc).exceptf).is_some()
            {
                ((*(*f).disc).exceptf)
                    .expect(
                        "non-null function pointer",
                    )(
                    f,
                    9 as libc::c_int,
                    1 as libc::c_int as *mut libc::c_void,
                    (*f).disc,
                );
            }
            (*f).mode |= 0o40 as libc::c_uint;
            let ref mut fresh0 = (*f).endw;
            *fresh0 = (*f).data;
            let ref mut fresh1 = (*f).endr;
            *fresh1 = *fresh0;
            mode = ((*f).mode & 0o100 as libc::c_uint) as libc::c_int;
            (*f).mode &= !(0o100 as libc::c_uint);
            if !((*f).flags as libc::c_int & 0o4 as libc::c_int != 0
                || (*f).mode & 0o4000 as libc::c_uint != 0)
            {
                if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
                    && ((*f).next > (*f).data
                        || (*f).bits as libc::c_int & 0o4 as libc::c_int != 0)
                {
                    let mut pool: libc::c_int = ((*f).mode & 0o200 as libc::c_uint)
                        as libc::c_int;
                    (*f).mode &= !(0o200 as libc::c_uint);
                    if (*f).next > (*f).data
                        && {
                            (*f).mode |= 0o20 as libc::c_uint;
                            (*f).mode |= 0o100000 as libc::c_uint;
                            _sfflsbuf(f, -(1 as libc::c_int)) < 0 as libc::c_int
                        }
                    {
                        rv = -(1 as libc::c_int);
                    }
                    if !((*f).extent < 0 as libc::c_int as libc::c_longlong
                        && (*f).bits as libc::c_int & 0o10 as libc::c_int != 0)
                        && (*f).bits as libc::c_int & 0o4 as libc::c_int != 0
                    {
                        (*f).mode |= 0o100000 as libc::c_uint;
                        if sfsk(
                            f,
                            -(1 as libc::c_int) as libc::c_longlong,
                            1 as libc::c_int,
                            (*f).disc,
                        ) >= 0 as libc::c_int as libc::c_longlong
                        {
                            (*f).mode |= 0o100000 as libc::c_uint;
                            sfwr(
                                f,
                                b"\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_void,
                                1 as libc::c_int as size_t,
                                (*f).disc,
                            );
                        }
                        let ref mut fresh2 = (*f).bits;
                        *fresh2 = (*fresh2 as libc::c_int
                            & !(0o4 as libc::c_int) as libc::c_ushort as libc::c_int)
                            as libc::c_ushort;
                    }
                    (*f).mode |= pool as libc::c_uint;
                }
                if (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
                    && (*f).extent >= 0 as libc::c_int as libc::c_longlong
                    && ((*f).bits as libc::c_int & 0o1 as libc::c_int != 0
                        || (*f).next < (*f).endb)
                {
                    (*f).here
                        -= ((*f).endb).offset_from((*f).next) as libc::c_long
                            as libc::c_longlong;
                    let ref mut fresh3 = (*f).endw;
                    *fresh3 = (*f).data;
                    let ref mut fresh4 = (*f).endr;
                    *fresh4 = *fresh3;
                    (*f)
                        .mode = 0o1 as libc::c_int as libc::c_uint
                        | 0o4000 as libc::c_uint | 0o40 as libc::c_uint;
                    (*f).mode |= 0o100000 as libc::c_uint;
                    sfsk(f, (*f).here, 0 as libc::c_int, (*f).disc);
                    if (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
                        && (*f).flags as libc::c_int & 0o4000 as libc::c_int == 0
                        && (*f).bits as libc::c_int & 0o1 as libc::c_int == 0
                    {
                        let ref mut fresh5 = (*f).next;
                        *fresh5 = (*f).data;
                        let ref mut fresh6 = (*f).endb;
                        *fresh6 = *fresh5;
                        (*f).mode &= !(0o4000 as libc::c_uint);
                    }
                }
            }
            (*f).mode |= mode as libc::c_uint;
            if local != 0 {} else {
                (*f).mode
                    &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint
                        | 0o20 as libc::c_uint);
                if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                    let ref mut fresh7 = (*f).endr;
                    *fresh7 = (*f).endb;
                } else {
                    if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                        let ref mut fresh8 = (*f).endw;
                        *fresh8 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int
                            != 0
                        {
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
            if (*f).flags as libc::c_int & 0o2000 as libc::c_int != 0
                && !((*f).disc).is_null() && ((*(*f).disc).exceptf).is_some()
            {
                ((*(*f).disc).exceptf)
                    .expect(
                        "non-null function pointer",
                    )(f, 9 as libc::c_int, 0 as *mut libc::c_void, (*f).disc);
            }
            f = (*f).push;
        }
    }
    if local == 0 && !f.is_null() && (*f).mode & 0o200 as libc::c_uint != 0
        && !((*f).pool).is_null()
        && f != *((*(*f).pool).sf).offset(0 as libc::c_int as isize)
    {
        (**((*(*f).pool).sf).offset(0 as libc::c_int as isize)).mode
            |= 0o100000 as libc::c_uint;
        sfsync(*((*(*f).pool).sf).offset(0 as libc::c_int as isize));
    }
    return rv;
}
