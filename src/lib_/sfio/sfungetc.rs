#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sfnew(
        _: *mut Sfio_t,
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut Sfio_t;
    fn sfstack(_: *mut Sfio_t, _: *mut Sfio_t) -> *mut Sfio_t;
    fn sfdisc(_: *mut Sfio_t, _: *mut Sfdisc_t) -> *mut Sfdisc_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
unsafe extern "C" fn _uexcept(
    mut f: *mut Sfio_t,
    mut type_0: libc::c_int,
    mut val: *mut libc::c_void,
    mut disc: *mut Sfdisc_t,
) -> libc::c_int {
    if disc != &mut _Sfextern.sf_udisc as *mut _sfdisc_s {
        return -(1 as libc::c_int);
    }
    if type_0 != 4 as libc::c_int {
        sfclose(
            (_Sfextern.sf_stack).expect("non-null function pointer")(f, 0 as *mut Sfio_t),
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sfungetc(
    mut f: *mut Sfio_t,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut uf: *mut Sfio_t = 0 as *mut Sfio_t;
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    if c < 0 as libc::c_int
        || (*f).mode != 0o1 as libc::c_int as libc::c_uint
            && _sfmode(f, 0o1 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*f).mode |= 0o40 as libc::c_uint;
    let ref mut fresh0 = (*f).endw;
    *fresh0 = (*f).data;
    let ref mut fresh1 = (*f).endr;
    *fresh1 = *fresh0;
    if (*f).next > (*f).data
        && *((*f).next).offset(-(1 as libc::c_int) as isize) as libc::c_int
            == c as libc::c_uchar as libc::c_int
    {
        let ref mut fresh2 = (*f).next;
        *fresh2 = (*fresh2).offset(-(1 as libc::c_int as isize));
    } else {
        if (*f).disc != &mut _Sfextern.sf_udisc as *mut _sfdisc_s {
            uf = sfnew(
                0 as *mut Sfio_t,
                0 as *mut libc::c_void,
                18446744073709551615 as libc::c_ulong,
                -(1 as libc::c_int),
                0o4 as libc::c_int | 0o1 as libc::c_int,
            );
            if uf.is_null() {
                c = -(1 as libc::c_int);
                current_block = 11452210031284351422;
            } else {
                _Sfextern
                    .sf_udisc
                    .exceptf = Some(
                    _uexcept
                        as unsafe extern "C" fn(
                            *mut Sfio_t,
                            libc::c_int,
                            *mut libc::c_void,
                            *mut Sfdisc_t,
                        ) -> libc::c_int,
                );
                sfdisc(uf, &mut _Sfextern.sf_udisc);
                if 0 as libc::c_int != 0 {} else {
                    (*f).mode
                        &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint
                            | 0o20 as libc::c_uint);
                    if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                        let ref mut fresh3 = (*f).endr;
                        *fresh3 = (*f).endb;
                    } else {
                        if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                            let ref mut fresh4 = (*f).endw;
                            *fresh4 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int
                                != 0
                            {
                                (*f).data
                            } else {
                                (*f).endb
                            });
                        } else {
                            let ref mut fresh5 = (*f).endr;
                            *fresh5 = (*f).data;
                            let ref mut fresh6 = (*f).endw;
                            *fresh6 = *fresh5;
                        };
                    };
                };
                sfstack(f, uf);
                (*f).mode |= 0o40 as libc::c_uint;
                let ref mut fresh7 = (*f).endw;
                *fresh7 = (*f).data;
                let ref mut fresh8 = (*f).endr;
                *fresh8 = *fresh7;
                current_block = 8457315219000651999;
            }
        } else {
            current_block = 8457315219000651999;
        }
        match current_block {
            11452210031284351422 => {}
            _ => {
                if (*f).next == (*f).data {
                    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    if (*f).size < 0 as libc::c_int as libc::c_long {
                        (*f).size = 0 as libc::c_int as ssize_t;
                    }
                    data = malloc(
                        ((*f).size + 16 as libc::c_int as libc::c_long) as libc::c_ulong,
                    ) as *mut libc::c_uchar;
                    if data.is_null() {
                        c = -(1 as libc::c_int);
                        current_block = 11452210031284351422;
                    } else {
                        let ref mut fresh9 = (*f).flags;
                        *fresh9 = (*fresh9 as libc::c_int | 0o20 as libc::c_int)
                            as libc::c_ushort;
                        if !((*f).data).is_null() {
                            memcpy(
                                data.offset(16 as libc::c_int as isize)
                                    as *mut libc::c_void,
                                (*f).data as *const libc::c_void,
                                (*f).size as libc::c_ulong,
                            );
                        }
                        let ref mut fresh10 = (*f).size;
                        *fresh10 += 16 as libc::c_int as libc::c_long;
                        let ref mut fresh11 = (*f).data;
                        *fresh11 = data;
                        let ref mut fresh12 = (*f).next;
                        *fresh12 = data.offset(16 as libc::c_int as isize);
                        let ref mut fresh13 = (*f).endb;
                        *fresh13 = data.offset((*f).size as isize);
                        current_block = 11932355480408055363;
                    }
                } else {
                    current_block = 11932355480408055363;
                }
                match current_block {
                    11452210031284351422 => {}
                    _ => {
                        let ref mut fresh14 = (*f).next;
                        *fresh14 = (*fresh14).offset(-1);
                        **fresh14 = c as libc::c_uchar;
                    }
                }
            }
        }
    }
    if 0 as libc::c_int != 0 {} else {
        (*f).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh15 = (*f).endr;
            *fresh15 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh16 = (*f).endw;
                *fresh16 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
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
    return c;
}
