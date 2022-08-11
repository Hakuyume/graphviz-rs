#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut _Sfi: ssize_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
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
pub type Sfextern_t = _sfextern_s;
#[no_mangle]
pub unsafe extern "C" fn _sfexcept(
    mut f: *mut Sfio_t,
    mut type_0: libc::c_int,
    mut io: ssize_t,
    mut disc: *mut Sfdisc_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ev: libc::c_int = 0;
    let mut local: libc::c_int = 0;
    let mut lock: libc::c_int = 0;
    let mut size: ssize_t = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    lock = ((*f).mode & 0o40 as libc::c_uint) as libc::c_int;
    if local != 0 && io <= 0 as libc::c_int as libc::c_long {
        let ref mut fresh0 = (*f).flags;
        *fresh0 = (*fresh0 as libc::c_int
            | if io < 0 as libc::c_int as libc::c_long {
                0o400 as libc::c_int
            } else {
                0o200 as libc::c_int
            }) as libc::c_ushort;
    }
    if !disc.is_null() && ((*disc).exceptf).is_some() {
        if local != 0 && lock != 0 {
            if 0 as libc::c_int != 0 {} else {
                (*f).mode
                    &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint
                        | 0o20 as libc::c_uint);
                if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                    let ref mut fresh1 = (*f).endr;
                    *fresh1 = (*f).endb;
                } else {
                    if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                        let ref mut fresh2 = (*f).endw;
                        *fresh2 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int
                            != 0
                        {
                            (*f).data
                        } else {
                            (*f).endb
                        });
                    } else {
                        let ref mut fresh3 = (*f).endr;
                        *fresh3 = (*f).data;
                        let ref mut fresh4 = (*f).endw;
                        *fresh4 = *fresh3;
                    };
                };
            };
        }
        let ref mut fresh5 = (*f).val;
        *fresh5 = io;
        _Sfi = *fresh5;
        ev = ((*disc).exceptf)
            .expect(
                "non-null function pointer",
            )(f, type_0, &mut io as *mut ssize_t as *mut libc::c_void, disc);
        if local != 0 && lock != 0 {
            (*f).mode |= 0o40 as libc::c_uint;
            let ref mut fresh6 = (*f).endw;
            *fresh6 = (*f).data;
            let ref mut fresh7 = (*f).endr;
            *fresh7 = *fresh6;
        }
        if io > 0 as libc::c_int as libc::c_long
            && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
        {
            return ev;
        }
        if ev < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if ev > 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
        if !(type_0 == 0o1 as libc::c_int) {
            if type_0 != 0o2 as libc::c_int && type_0 != 3 as libc::c_int {
                return 0 as libc::c_int;
            }
            if local != 0 && io >= 0 as libc::c_int as libc::c_long {
                if (*f).size >= 0 as libc::c_int as libc::c_long
                    && (*f).flags as libc::c_int & 0o20 as libc::c_int == 0
                {
                    current_block = 7481093856908124627;
                } else {
                    size = (*f).size;
                    if size < 0 as libc::c_int as libc::c_long {
                        size = 0 as libc::c_int as ssize_t;
                    }
                    io -= size;
                    if io <= 0 as libc::c_int as libc::c_long {
                        io = 1024 as libc::c_int as ssize_t;
                    }
                    size = (size + io + 1024 as libc::c_int as libc::c_long
                        - 1 as libc::c_int as libc::c_long)
                        / 1024 as libc::c_int as libc::c_long
                        * 1024 as libc::c_int as libc::c_long;
                    if (*f).size > 0 as libc::c_int as libc::c_long {
                        data = realloc(
                            (*f).data as *mut libc::c_void,
                            size as libc::c_ulong,
                        ) as *mut libc::c_uchar;
                    } else {
                        data = malloc(size as libc::c_ulong) as *mut libc::c_uchar;
                    }
                    if data.is_null() {
                        current_block = 7481093856908124627;
                    } else {
                        let ref mut fresh8 = (*f).endb;
                        *fresh8 = data.offset(size as isize);
                        let ref mut fresh9 = (*f).next;
                        *fresh9 = data
                            .offset(
                                ((*f).next).offset_from((*f).data) as libc::c_long as isize,
                            );
                        let ref mut fresh10 = (*f).data;
                        *fresh10 = data;
                        let ref mut fresh11 = (*f).endw;
                        *fresh11 = *fresh10;
                        let ref mut fresh12 = (*f).endr;
                        *fresh12 = *fresh11;
                        (*f).size = size;
                        current_block = 1622411330066726685;
                    }
                }
            } else {
                current_block = 1622411330066726685;
            }
            match current_block {
                7481093856908124627 => {}
                _ => return 1 as libc::c_int,
            }
        }
    } else if *__errno_location() == 4 as libc::c_int {
        if _Sfextern.sf_exiting != 0
            || (*f).bits as libc::c_int & 0o400 as libc::c_int != 0
        {
            return 0 as libc::c_int;
        }
        *__errno_location() = 0 as libc::c_int;
        let ref mut fresh13 = (*f).flags;
        *fresh13 = (*fresh13 as libc::c_int
            & !(0o200 as libc::c_int | 0o400 as libc::c_int) as libc::c_ushort
                as libc::c_int) as libc::c_ushort;
        return 3 as libc::c_int;
    }
    if local != 0 && !((*f).push).is_null()
        && (type_0 == 0o1 as libc::c_int && (*f).next >= (*f).endb
            || type_0 == 0o2 as libc::c_int && (*f).next <= (*f).data)
    {
        let mut pf: *mut Sfio_t = 0 as *mut Sfio_t;
        if lock != 0 {
            if 0 as libc::c_int != 0 {} else {
                (*f).mode
                    &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint
                        | 0o20 as libc::c_uint);
                if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                    let ref mut fresh14 = (*f).endr;
                    *fresh14 = (*f).endb;
                } else {
                    if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                        let ref mut fresh15 = (*f).endw;
                        *fresh15 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int
                            != 0
                        {
                            (*f).data
                        } else {
                            (*f).endb
                        });
                    } else {
                        let ref mut fresh16 = (*f).endr;
                        *fresh16 = (*f).data;
                        let ref mut fresh17 = (*f).endw;
                        *fresh17 = *fresh16;
                    };
                };
            };
        }
        pf = (_Sfextern.sf_stack)
            .expect("non-null function pointer")(f, 0 as *mut Sfio_t);
        ev = sfclose(pf);
        if ev < 0 as libc::c_int {
            (_Sfextern.sf_stack).expect("non-null function pointer")(f, pf);
        }
        if lock != 0 {
            (*f).mode |= 0o40 as libc::c_uint;
            let ref mut fresh18 = (*f).endw;
            *fresh18 = (*f).data;
            let ref mut fresh19 = (*f).endr;
            *fresh19 = *fresh18;
        }
        ev = if ev < 0 as libc::c_int { 0 as libc::c_int } else { 2 as libc::c_int };
    } else {
        ev = 0 as libc::c_int;
    }
    return ev;
}
