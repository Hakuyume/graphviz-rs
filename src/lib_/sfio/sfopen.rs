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
    fn sfnew(
        _: *mut Sfio_t,
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut Sfio_t;
    fn sfsetfd(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
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
pub unsafe extern "C" fn sfopen(
    mut f: *mut Sfio_t,
    mut file: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut Sfio_t {
    let mut fd: libc::c_int = 0;
    let mut oldfd: libc::c_int = 0;
    let mut oflags: libc::c_int = 0;
    let mut sflags: libc::c_int = 0;
    sflags = _sftype(mode, &mut oflags, 0 as *mut libc::c_int);
    if sflags == 0 as libc::c_int {
        return 0 as *mut Sfio_t;
    }
    if !f.is_null() && file.is_null() && (*f).mode & 0o4 as libc::c_uint != 0 {
        if f.is_null() {
            return 0 as *mut Sfio_t;
        }
        if (*f).mode & 0o4 as libc::c_uint != 0 {
            if (*f).file as libc::c_int >= 0 as libc::c_int
                && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
                && {
                    oflags &= 0 as libc::c_int | 0 as libc::c_int | 0o2000 as libc::c_int;
                    oflags != 0 as libc::c_int
                }
            {
                let mut ctl: libc::c_int =
                    fcntl((*f).file as libc::c_int, 3 as libc::c_int, 0 as libc::c_int);
                ctl = ctl & !(0 as libc::c_int | 0 as libc::c_int | 0o2000 as libc::c_int) | oflags;
                fcntl((*f).file as libc::c_int, 4 as libc::c_int, ctl);
            }
            let ref mut fresh0 = (*f).flags;
            *fresh0 = (*fresh0 as libc::c_int
                | sflags & (0o77177 as libc::c_int & !(0o1 as libc::c_int | 0o2 as libc::c_int)))
                as libc::c_ushort;
            sflags &= 0o1 as libc::c_int | 0o2 as libc::c_int;
            if sflags != 0 as libc::c_int {
                (*f).flags = ((*f).flags as libc::c_int
                    & !(0o1 as libc::c_int | 0o2 as libc::c_int)
                    | sflags) as libc::c_ushort;
                if (*f).flags as libc::c_int & (0o1 as libc::c_int | 0o2 as libc::c_int)
                    == 0o1 as libc::c_int | 0o2 as libc::c_int
                {
                    let ref mut fresh1 = (*f).bits;
                    *fresh1 = (*fresh1 as libc::c_int | 0o2 as libc::c_int) as libc::c_ushort;
                } else {
                    let ref mut fresh2 = (*f).bits;
                    *fresh2 = (*fresh2 as libc::c_int
                        & !(0o2 as libc::c_int) as libc::c_ushort as libc::c_int)
                        as libc::c_ushort;
                }
                if (*f).flags as libc::c_int & 0o1 as libc::c_int != 0 {
                    (*f).mode = (*f).mode & !(0o2 as libc::c_int) as libc::c_uint
                        | 0o1 as libc::c_int as libc::c_uint;
                } else {
                    (*f).mode = (*f).mode & !(0o1 as libc::c_int) as libc::c_uint
                        | 0o2 as libc::c_int as libc::c_uint;
                }
            }
            return f;
        } else {
            return 0 as *mut Sfio_t;
        }
    }
    if sflags & 0o4 as libc::c_int != 0 {
        f = sfnew(
            f,
            file as *mut libc::c_char as *mut libc::c_void,
            if !file.is_null() {
                strlen(file)
            } else {
                18446744073709551615 as libc::c_ulong
            },
            -(1 as libc::c_int),
            sflags,
        );
    } else {
        if file.is_null() {
            return 0 as *mut Sfio_t;
        }
        loop {
            fd = open(
                file,
                oflags,
                0o400 as libc::c_int
                    | 0o200 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
            );
            if !(fd < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
            *__errno_location() = 0 as libc::c_int;
        }
        if fd < 0 as libc::c_int {
            return 0 as *mut Sfio_t;
        }
        oldfd = if !f.is_null() {
            (*f).file as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        f = sfnew(
            f,
            0 as *mut libc::c_void,
            18446744073709551615 as libc::c_ulong,
            fd,
            sflags,
        );
        if !f.is_null() && oldfd >= 0 as libc::c_int {
            sfsetfd(f, oldfd);
        }
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn _sftype(
    mut mode: *const libc::c_char,
    mut oflagsp: *mut libc::c_int,
    mut uflagp: *mut libc::c_int,
) -> libc::c_int {
    let mut sflags: libc::c_int = 0;
    let mut oflags: libc::c_int = 0;
    let mut uflag: libc::c_int = 0;
    if mode.is_null() {
        return 0 as libc::c_int;
    }
    uflag = 0 as libc::c_int;
    oflags = uflag;
    sflags = oflags;
    loop {
        let fresh3 = mode;
        mode = mode.offset(1);
        match *fresh3 as libc::c_int {
            119 => {
                sflags |= 0o2 as libc::c_int;
                oflags |= 0o1 as libc::c_int | 0o100 as libc::c_int;
                if sflags & 0o1 as libc::c_int == 0 {
                    oflags |= 0o1000 as libc::c_int;
                }
            }
            97 => {
                sflags |= 0o2 as libc::c_int | 0o10 as libc::c_int;
                oflags |= 0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int;
            }
            114 => {
                sflags |= 0o1 as libc::c_int;
                oflags |= 0 as libc::c_int;
            }
            115 => {
                sflags |= 0o4 as libc::c_int;
            }
            98 => {
                oflags |= 0 as libc::c_int;
            }
            116 => {
                oflags |= 0 as libc::c_int;
            }
            120 => {
                oflags |= 0o200 as libc::c_int;
            }
            43 => {
                if sflags != 0 {
                    sflags |= 0o1 as libc::c_int | 0o2 as libc::c_int;
                }
            }
            109 => {
                uflag = 0 as libc::c_int;
            }
            117 => {
                uflag = 1 as libc::c_int;
            }
            _ => {
                if oflags & 0o100 as libc::c_int == 0 {
                    oflags &= !(0o200 as libc::c_int);
                }
                if sflags & (0o1 as libc::c_int | 0o2 as libc::c_int)
                    == 0o1 as libc::c_int | 0o2 as libc::c_int
                {
                    oflags = oflags & !(0 as libc::c_int | 0o1 as libc::c_int) | 0o2 as libc::c_int;
                }
                if !oflagsp.is_null() {
                    *oflagsp = oflags;
                }
                if !uflagp.is_null() {
                    *uflagp = uflag;
                }
                if sflags & (0o4 as libc::c_int | (0o1 as libc::c_int | 0o2 as libc::c_int))
                    == 0o4 as libc::c_int
                {
                    sflags |= 0o1 as libc::c_int;
                }
                return sflags;
            }
        }
    }
}
