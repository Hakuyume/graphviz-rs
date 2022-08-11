#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfsk(
        _: *mut Sfio_t,
        _: libc::c_longlong,
        _: libc::c_int,
        _: *mut Sfdisc_t,
    ) -> libc::c_longlong;
    fn sfrd(
        _: *mut Sfio_t,
        _: *mut libc::c_void,
        _: size_t,
        _: *mut Sfdisc_t,
    ) -> ssize_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut _Sfextern: Sfextern_t;
    fn __errno_location() -> *mut libc::c_int;
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
unsafe extern "C" fn newpos(mut f: *mut Sfio_t, mut p: libc::c_longlong) {
    let ref mut fresh0 = (*f).endw;
    *fresh0 = (*f).data;
    let ref mut fresh1 = (*f).endr;
    *fresh1 = *fresh0;
    let ref mut fresh2 = (*f).next;
    *fresh2 = *fresh1;
    let ref mut fresh3 = (*f).endb;
    *fresh3 = if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0 {
        ((*f).data).offset((*f).size as isize)
    } else {
        (*f).data
    };
    let ref mut fresh4 = (*f).here;
    *fresh4 = p;
    if *fresh4 < 0 as libc::c_int as libc::c_longlong {
        (*f).extent = -(1 as libc::c_int) as libc::c_longlong;
        (*f).here = 0 as libc::c_int as libc::c_longlong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sfseek(
    mut f: *mut Sfio_t,
    mut p: libc::c_longlong,
    mut type_0: libc::c_int,
) -> libc::c_longlong {
    let mut current_block: u64;
    let mut r: libc::c_longlong = 0;
    let mut s: libc::c_longlong = 0;
    let mut a: size_t = 0;
    let mut b: size_t = 0;
    let mut c: size_t = 0;
    let mut mode: libc::c_int = 0;
    let mut local: libc::c_int = 0;
    let mut hardseek: libc::c_int = 0;
    let mut mustsync: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    hardseek = (type_0 | (*f).flags as libc::c_int)
        & (0o100 as libc::c_int | 0o4000 as libc::c_int);
    if hardseek != 0
        && (*f).mode == 0o1 as libc::c_int as libc::c_uint | 0o4000 as libc::c_uint
    {
        newpos(f, (*f).here);
        (*f).mode = 0o1 as libc::c_int as libc::c_uint;
    }
    mode = ((*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    if ((*f).mode
        & !(0o20 as libc::c_uint | 0o10 as libc::c_uint
            | (if local != 0 {
                0o40 as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }))) as libc::c_int != mode
    {
        let mut flags: libc::c_ushort = (*f).flags;
        if hardseek & 0o4000 as libc::c_int != 0 {
            let ref mut fresh5 = (*f).flags;
            *fresh5 = (*fresh5 as libc::c_int
                | (0o100 as libc::c_int | 0o4000 as libc::c_int)) as libc::c_ushort;
        }
        mode = _sfmode(f, mode, local);
        if hardseek & 0o4000 as libc::c_int != 0 {
            (*f).flags = flags;
        }
        if mode < 0 as libc::c_int {
            return -(1 as libc::c_int) as libc::c_longlong;
        }
    }
    mustsync = (type_0 & 0o100 as libc::c_int != 0 && type_0 & 0o4000 as libc::c_int == 0
        && (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
        && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0) as libc::c_int;
    type_0 &= 0 as libc::c_int | 1 as libc::c_int | 2 as libc::c_int;
    if type_0 != 0 as libc::c_int && type_0 != 1 as libc::c_int
        && type_0 != 2 as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    if (*f).extent < 0 as libc::c_int as libc::c_longlong {
        (*f).mode |= 0o100000 as libc::c_uint;
        sfsk(f, 0 as libc::c_int as libc::c_longlong, 1 as libc::c_int, (*f).disc);
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    if (*f).disc == &mut _Sfextern.sf_udisc as *mut _sfdisc_s {
        sfclose(
            (_Sfextern.sf_stack).expect("non-null function pointer")(f, 0 as *mut Sfio_t),
        );
    }
    (*f).mode |= 0o40 as libc::c_uint;
    let ref mut fresh6 = (*f).endw;
    *fresh6 = (*f).data;
    let ref mut fresh7 = (*f).endr;
    *fresh7 = *fresh6;
    let ref mut fresh8 = (*f).flags;
    *fresh8 = (*fresh8 as libc::c_int
        & !(0o200 as libc::c_int | 0o400 as libc::c_int) as libc::c_ushort
            as libc::c_int) as libc::c_ushort;
    loop {
        if !((*f).flags as libc::c_int & 0o4 as libc::c_int != 0) {
            current_block = 9859671972921157070;
            break;
        }
        let mut s_0: libc::c_longlong = ((*f).next).offset_from((*f).data)
            as libc::c_long as libc::c_longlong;
        if s_0 > (*f).here {
            (*f).here = s_0;
            if s_0 > (*f).extent {
                (*f).extent = s_0;
            }
        }
        if type_0 == 1 as libc::c_int {
            r = p
                + ((*f).next).offset_from((*f).data) as libc::c_long as libc::c_longlong;
        } else if type_0 == 2 as libc::c_int {
            r = p + (*f).extent;
        } else {
            r = p;
        }
        if r >= 0 as libc::c_int as libc::c_longlong
            && r <= (*f).size as libc::c_longlong
        {
            p = r;
            let ref mut fresh9 = (*f).next;
            *fresh9 = ((*f).data).offset(p as isize);
            (*f).here = p;
            if p > (*f).extent {
                memset(
                    ((*f).data).offset((*f).extent as isize) as *mut libc::c_void,
                    '\0' as i32,
                    (p - (*f).extent) as libc::c_int as libc::c_ulong,
                );
            }
            current_block = 12782143740587617951;
            break;
        } else {
            (*f).mode |= 0o100000 as libc::c_uint;
            if sfsk(f, r, 0 as libc::c_int, (*f).disc) != r {
                p = -(1 as libc::c_int) as libc::c_longlong;
                current_block = 12782143740587617951;
                break;
            } else {
                if !((*f).flags as libc::c_int & 0o4 as libc::c_int == 0) {
                    continue;
                }
                p = r;
                current_block = 12782143740587617951;
                break;
            }
        }
    }
    match current_block {
        9859671972921157070 => {
            if (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0 {
                if hardseek == 0 && type_0 < 2 as libc::c_int
                    && (*f).flags as libc::c_int & 0o10 as libc::c_int == 0
                {
                    s = (*f).here
                        + ((*f).next).offset_from((*f).data) as libc::c_long
                            as libc::c_longlong;
                    r = p
                        + (if type_0 == 0 as libc::c_int {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            s
                        });
                    if r == s {
                        p = r;
                        current_block = 12782143740587617951;
                    } else {
                        current_block = 307447392441238883;
                    }
                } else {
                    current_block = 307447392441238883;
                }
                match current_block {
                    12782143740587617951 => {}
                    _ => {
                        if (*f).next > (*f).data
                            && {
                                (*f).mode |= 0o100000 as libc::c_uint;
                                sfsync(f) < 0 as libc::c_int
                            }
                        {
                            p = -(1 as libc::c_int) as libc::c_longlong;
                            current_block = 12782143740587617951;
                        } else {
                            current_block = 6281126495347172768;
                        }
                    }
                }
            } else {
                current_block = 6281126495347172768;
            }
            match current_block {
                12782143740587617951 => {}
                _ => {
                    if type_0 == 2 as libc::c_int
                        || (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
                    {
                        if hardseek & 0o4000 as libc::c_int != 0
                            || type_0 == 2 as libc::c_int
                        {
                            (*f).mode |= 0o100000 as libc::c_uint;
                            p = sfsk(f, p, type_0, (*f).disc);
                        } else {
                            r = p
                                + (if type_0 == 1 as libc::c_int {
                                    (*f).here
                                } else {
                                    0 as libc::c_int as libc::c_longlong
                                });
                            p = if hardseek != 0 || r != (*f).here {
                                (*f).mode |= 0o100000 as libc::c_uint;
                                sfsk(f, r, 0 as libc::c_int, (*f).disc)
                            } else {
                                r
                            };
                        }
                        if p >= 0 as libc::c_int as libc::c_longlong {
                            newpos(f, p);
                        }
                    } else {
                        s = (*f).here
                            - ((*f).endb).offset_from((*f).next) as libc::c_long
                                as libc::c_longlong;
                        r = p
                            + (if type_0 == 1 as libc::c_int {
                                s
                            } else {
                                0 as libc::c_int as libc::c_longlong
                            });
                        if r <= (*f).here
                            && r
                                >= (*f).here
                                    - ((*f).endb).offset_from((*f).data) as libc::c_long
                                        as libc::c_longlong
                        {
                            if hardseek != 0
                                || type_0 == 1 as libc::c_int
                                    && p == 0 as libc::c_int as libc::c_longlong
                            {
                                (*f).mode |= 0o100000 as libc::c_uint;
                                s = sfsk(
                                    f,
                                    0 as libc::c_int as libc::c_longlong,
                                    1 as libc::c_int,
                                    (*f).disc,
                                );
                                if s == (*f).here
                                    || s >= 0 as libc::c_int as libc::c_longlong
                                        && hardseek & 0o4000 as libc::c_int == 0
                                        && {
                                            (*f).mode |= 0o100000 as libc::c_uint;
                                            s = sfsk(f, (*f).here, 0 as libc::c_int, (*f).disc);
                                            s == (*f).here
                                        }
                                {
                                    current_block = 8289148914625071601;
                                } else if s < 0 as libc::c_int as libc::c_longlong {
                                    p = -(1 as libc::c_int) as libc::c_longlong;
                                    current_block = 12782143740587617951;
                                } else {
                                    newpos(f, s);
                                    hardseek = 0 as libc::c_int;
                                    current_block = 496303045384785551;
                                }
                            } else {
                                current_block = 8289148914625071601;
                            }
                            match current_block {
                                496303045384785551 => {}
                                12782143740587617951 => {}
                                _ => {
                                    let ref mut fresh10 = (*f).next;
                                    *fresh10 = ((*f).endb).offset(-(((*f).here - r) as isize));
                                    p = r;
                                    current_block = 12782143740587617951;
                                }
                            }
                        } else {
                            current_block = 496303045384785551;
                        }
                        match current_block {
                            12782143740587617951 => {}
                            _ => {
                                p
                                    += (if type_0 == 1 as libc::c_int {
                                        s
                                    } else {
                                        0 as libc::c_int as libc::c_longlong
                                    });
                                if !(p < 0 as libc::c_int as libc::c_longlong) {
                                    b = ((*f).endb).offset_from((*f).data) as libc::c_long
                                        as size_t;
                                    c = ((*f).next).offset_from((*f).data) as libc::c_long
                                        as size_t;
                                    if b > 0 as libc::c_int as libc::c_ulong {
                                        if b <= 1024 as libc::c_int as libc::c_ulong {
                                            (*f).iosz = 1024 as libc::c_int as size_t;
                                        } else {
                                            c = (c as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                                as size_t;
                                            a = c
                                                .wrapping_add(
                                                    (3 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(b.wrapping_sub(c))
                                                        .wrapping_div(4 as libc::c_int as libc::c_ulong),
                                                );
                                            a = a
                                                .wrapping_add(1024 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_div(1024 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(1024 as libc::c_int as libc::c_ulong);
                                            b = b
                                                .wrapping_add(1024 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_div(1024 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(1024 as libc::c_int as libc::c_ulong);
                                            (*f)
                                                .iosz = if a < b {
                                                a
                                            } else if c
                                                    < b.wrapping_div(2 as libc::c_int as libc::c_ulong)
                                                {
                                                b.wrapping_div(2 as libc::c_int as libc::c_ulong)
                                            } else {
                                                b
                                            };
                                        }
                                    }
                                    if (*f).iosz >= (*f).size as libc::c_ulong {
                                        (*f).iosz = 0 as libc::c_int as size_t;
                                    }
                                    let ref mut fresh11 = (*f).endb;
                                    *fresh11 = (*f).data;
                                    let ref mut fresh12 = (*f).endr;
                                    *fresh12 = *fresh11;
                                    let ref mut fresh13 = (*f).next;
                                    *fresh13 = *fresh12;
                                    if p < (*f).lpos
                                        && (*f).size > 1024 as libc::c_int as libc::c_long
                                        && p + 1024 as libc::c_int as libc::c_longlong > s
                                    {
                                        r = s - (*f).size as libc::c_longlong;
                                        if r < 0 as libc::c_int as libc::c_longlong {
                                            r = 0 as libc::c_int as libc::c_longlong;
                                        }
                                    } else {
                                        r = p;
                                        if (*f).iosz > 0 as libc::c_int as libc::c_ulong
                                            && (p > (*f).lpos
                                                || p < (*f).lpos - (*f).size as libc::c_longlong)
                                        {
                                            let ref mut fresh14 = (*f).bits;
                                            *fresh14 = (*fresh14 as libc::c_int | 0o40 as libc::c_int)
                                                as libc::c_ushort;
                                        }
                                    }
                                    if (hardseek != 0 || r != (*f).here)
                                        && {
                                            (*f).mode |= 0o100000 as libc::c_uint;
                                            let ref mut fresh15 = (*f).here;
                                            *fresh15 = sfsk(f, r, 0 as libc::c_int, (*f).disc);
                                            *fresh15 != r
                                        }
                                    {
                                        if r < p {
                                            (*f).mode |= 0o100000 as libc::c_uint;
                                            (*f).here = sfsk(f, p, 0 as libc::c_int, (*f).disc);
                                        }
                                        if (*f).here != p {
                                            p = -(1 as libc::c_int) as libc::c_longlong;
                                        }
                                    } else if r < p {
                                        (*f).mode |= 0o100000 as libc::c_uint;
                                        sfrd(
                                            f,
                                            (*f).data as *mut libc::c_void,
                                            (*f).size as size_t,
                                            (*f).disc,
                                        );
                                        if p <= (*f).here
                                            && p
                                                >= (*f).here
                                                    - ((*f).endb).offset_from((*f).data) as libc::c_long
                                                        as libc::c_longlong
                                        {
                                            let ref mut fresh16 = (*f).next;
                                            *fresh16 = ((*f).endb)
                                                .offset(-(((*f).here - p) as size_t as isize));
                                        } else {
                                            let ref mut fresh17 = (*f).endb;
                                            *fresh17 = (*f).data;
                                            let ref mut fresh18 = (*f).next;
                                            *fresh18 = *fresh17;
                                            (*f).mode |= 0o100000 as libc::c_uint;
                                            let ref mut fresh19 = (*f).here;
                                            *fresh19 = sfsk(f, p, 0 as libc::c_int, (*f).disc);
                                            if *fresh19 != p {
                                                p = -(1 as libc::c_int) as libc::c_longlong;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if (*f).here < 0 as libc::c_int as libc::c_longlong {
        (*f).extent = -(1 as libc::c_int) as libc::c_longlong;
        (*f).here = 0 as libc::c_int as libc::c_longlong;
    }
    (*f).lpos = p;
    if local != 0 {} else {
        (*f).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh20 = (*f).endr;
            *fresh20 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh21 = (*f).endw;
                *fresh21 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh22 = (*f).endr;
                *fresh22 = (*f).data;
                let ref mut fresh23 = (*f).endw;
                *fresh23 = *fresh22;
            };
        };
    };
    if mustsync != 0 {
        sfsync(f);
    }
    return p;
}
