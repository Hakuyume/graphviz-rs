#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut _Sfi: ssize_t;
    static mut sfstdin: *mut Sfio_t;
    static mut sfstdout: *mut Sfio_t;
    static mut sfstderr: *mut Sfio_t;
    fn sfsync(_: *mut Sfio_t) -> libc::c_int;
    static mut _Sfextern: Sfextern_t;
    fn getpagesize() -> libc::c_int;
    fn _sfpopen(
        _: *mut Sfio_t,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn sfsk(
        _: *mut Sfio_t,
        _: libc::c_longlong,
        _: libc::c_int,
        _: *mut Sfdisc_t,
    ) -> libc::c_longlong;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn _sfsetpool(_: *mut Sfio_t) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type Stat_t = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const have_sys_stat_h: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn sfsetbuf(
    mut f: *mut Sfio_t,
    mut buf: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut sf_malloc: libc::c_int = 0;
    let mut obuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut disc: *mut Sfdisc_t = 0 as *mut Sfdisc_t;
    let mut osize: ssize_t = 0;
    let mut blksize: ssize_t = 0;
    let mut oflags: libc::c_int = 0;
    let mut init: libc::c_int = 0;
    let mut local: libc::c_int = 0;
    let mut st: Stat_t = Stat_t {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if f.is_null() {
        return 0 as *mut libc::c_void;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    if size == 0 as libc::c_int as libc::c_ulong && !buf.is_null() {
        let ref mut fresh0 = (*f).val;
        *fresh0 = if (*f).bits as libc::c_int & 0o1 as libc::c_int != 0 {
            ((*f).endb).offset_from((*f).data) as libc::c_long
        } else {
            (*f).size
        };
        _Sfi = *fresh0;
        return (*f).data as *mut libc::c_void;
    }
    if _Sfextern.sf_exiting != 0 && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
        && (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
    {
        buf = 0 as *mut libc::c_void;
        size = 0 as libc::c_int as size_t;
    }
    init = ((*f).mode & 0o4 as libc::c_uint) as libc::c_int;
    if init != 0 {
        if ((*f).pool).is_null() && _sfsetpool(f) < 0 as libc::c_int {
            return 0 as *mut libc::c_void;
        }
    } else if (*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
            != (*f).mode
                & !(0o20 as libc::c_uint | 0o10 as libc::c_uint
                    | (if local != 0 {
                        0o40 as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    })) && _sfmode(f, 0 as libc::c_int, local) < 0 as libc::c_int
        {
        return 0 as *mut libc::c_void
    }
    if init != 0 {
        (*f)
            .mode = (*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint
            | 0o40 as libc::c_uint;
    } else {
        let mut rv: libc::c_int = 0;
        if !((*f).proc_0).is_null()
            && (*f).flags as libc::c_int & 0o1 as libc::c_int != 0
            && (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0
            && _sfmode(f, 0o1 as libc::c_int, local) < 0 as libc::c_int
        {
            return 0 as *mut libc::c_void;
        }
        (*f).mode |= 0o40 as libc::c_uint;
        let ref mut fresh1 = (*f).endw;
        *fresh1 = (*f).data;
        let ref mut fresh2 = (*f).endr;
        *fresh2 = *fresh1;
        (*f).mode |= 0o100000 as libc::c_uint;
        rv = sfsync(f);
        if local != 0 {} else {
            (*f).mode
                &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
            if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
                let ref mut fresh3 = (*f).endr;
                *fresh3 = (*f).endb;
            } else {
                if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                    let ref mut fresh4 = (*f).endw;
                    *fresh4 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
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
        if rv < 0 as libc::c_int {
            return 0 as *mut libc::c_void;
        }
        (*f).mode &= !(0o4000 as libc::c_uint);
    }
    (*f).mode |= 0o40 as libc::c_uint;
    let ref mut fresh7 = (*f).endw;
    *fresh7 = (*f).data;
    let ref mut fresh8 = (*f).endr;
    *fresh8 = *fresh7;
    blksize = 0 as libc::c_int as ssize_t;
    oflags = (*f).flags as libc::c_int;
    if (*f).data == ((*f).tiny).as_mut_ptr() {
        let ref mut fresh9 = (*f).data;
        *fresh9 = 0 as *mut libc::c_uchar;
        (*f).size = 0 as libc::c_int as ssize_t;
    }
    obuf = (*f).data;
    osize = (*f).size;
    let ref mut fresh10 = (*f).flags;
    *fresh10 = (*fresh10 as libc::c_int
        & !(0o20 as libc::c_int) as libc::c_ushort as libc::c_int) as libc::c_ushort;
    let ref mut fresh11 = (*f).bits;
    *fresh11 = (*fresh11 as libc::c_int
        & !(0o1 as libc::c_int) as libc::c_ushort as libc::c_int) as libc::c_ushort;
    if (*f).flags as libc::c_int
        & (0o1 as libc::c_int | 0o2 as libc::c_int | 0o4 as libc::c_int)
        == 0o1 as libc::c_int | 0o4 as libc::c_int
        && (size == 18446744073709551615 as libc::c_ulong || buf.is_null())
    {
        size = 0 as libc::c_int as size_t;
    }
    disc = (*f).disc;
    while !disc.is_null() {
        if ((*disc).seekf).is_some() {
            break;
        }
        disc = (*disc).disc;
    }
    if (init != 0 || local != 0) && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0 {
        st.st_mode = 0 as libc::c_int as __mode_t;
        if have_sys_stat_h as libc::c_int == 0 || !disc.is_null() {
            (*f).mode |= 0o100000 as libc::c_uint;
            let ref mut fresh12 = (*f).here;
            *fresh12 = sfsk(
                f,
                0 as libc::c_int as libc::c_longlong,
                1 as libc::c_int,
                disc,
            );
            if *fresh12 < 0 as libc::c_int as libc::c_longlong {
                current_block = 2215113774637557111;
            } else {
                let mut e: libc::c_longlong = 0;
                (*f).mode |= 0o100000 as libc::c_uint;
                e = sfsk(
                    f,
                    0 as libc::c_int as libc::c_longlong,
                    2 as libc::c_int,
                    disc,
                );
                if e >= 0 as libc::c_int as libc::c_longlong {
                    (*f).extent = if e > (*f).here { e } else { (*f).here };
                }
                (*f).mode |= 0o100000 as libc::c_uint;
                sfsk(f, (*f).here, 0 as libc::c_int, disc);
                current_block = 17084314706199238786;
            }
        } else {
            if fstat((*f).file as libc::c_int, &mut st) < 0 as libc::c_int {
                (*f).here = -(1 as libc::c_int) as libc::c_longlong;
            } else if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                    || st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                {
                (*f).mode |= 0o100000 as libc::c_uint;
                (*f)
                    .here = sfsk(
                    f,
                    0 as libc::c_int as libc::c_longlong,
                    1 as libc::c_int,
                    (*f).disc,
                );
            } else {
                (*f).here = -(1 as libc::c_int) as libc::c_longlong;
            }
            if (*f).here >= 0 as libc::c_int as libc::c_longlong {
                (*f).extent = st.st_size as libc::c_longlong;
                if f == sfstdin || f == sfstdout || f == sfstderr {
                    let ref mut fresh13 = (*f).flags;
                    *fresh13 = (*fresh13 as libc::c_int
                        | (0o100 as libc::c_int | 0o4000 as libc::c_int))
                        as libc::c_ushort;
                }
                current_block = 200744462051969938;
            } else {
                current_block = 2215113774637557111;
            }
        }
        match current_block {
            17084314706199238786 => {}
            _ => {
                match current_block {
                    2215113774637557111 => {
                        (*f).extent = -(1 as libc::c_int) as libc::c_longlong;
                        (*f).here = 0 as libc::c_int as libc::c_longlong;
                        if init != 0 {
                            if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o20000 as libc::c_int as libc::c_uint
                            {
                                let mut oerrno: libc::c_int = *__errno_location();
                                blksize = 1024 as libc::c_int as ssize_t;
                                if (*f).flags as libc::c_int & 0o40 as libc::c_int == 0
                                    && isatty((*f).file as libc::c_int) != 0
                                {
                                    let ref mut fresh14 = (*f).flags;
                                    *fresh14 = (*fresh14 as libc::c_int | 0o40 as libc::c_int)
                                        as libc::c_ushort;
                                } else {
                                    let mut dev: libc::c_int = 0;
                                    let mut ino: libc::c_int = 0;
                                    dev = st.st_dev as libc::c_int;
                                    ino = st.st_ino as libc::c_int;
                                    if stat(
                                        b"/dev/null\0" as *const u8 as *const libc::c_char,
                                        &mut st,
                                    ) >= 0 as libc::c_int && dev == st.st_dev as libc::c_int
                                        && ino == st.st_ino as libc::c_int
                                    {
                                        (*f).extent = -(1 as libc::c_int) as libc::c_longlong;
                                        let ref mut fresh15 = (*f).bits;
                                        *fresh15 = (*fresh15 as libc::c_int | 0o10 as libc::c_int)
                                            as libc::c_ushort;
                                    }
                                }
                                *__errno_location() = oerrno;
                            }
                            if ((*f).proc_0).is_null()
                                && (*f).bits as libc::c_int & 0o2 as libc::c_int != 0
                            {
                                _sfpopen(
                                    f,
                                    -(1 as libc::c_int),
                                    -(1 as libc::c_int),
                                    0 as libc::c_int,
                                );
                            }
                        }
                    }
                    _ => {}
                }
                if _Sfextern.sf_page <= 0 as libc::c_int as libc::c_long {
                    _Sfextern.sf_page = getpagesize() as size_t as ssize_t;
                    if _Sfextern.sf_page <= 0 as libc::c_int as libc::c_long {
                        _Sfextern
                            .sf_page = (1024 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            )
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as ssize_t;
                    }
                }
            }
        }
    }
    if size == 18446744073709551615 as libc::c_ulong {
        if init != 0 && osize > 0 as libc::c_int as libc::c_long {
            size = osize as size_t;
        } else if f == sfstderr && (*f).mode & 0o2 as libc::c_int as libc::c_uint != 0 {
            size = 0 as libc::c_int as size_t;
        } else if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
            size = 1024 as libc::c_int as size_t;
        } else if (*f).flags as libc::c_int & 0o1 as libc::c_int != 0
                && (*f).bits as libc::c_int & 0o2 as libc::c_int == 0
                && (*f).extent > 0 as libc::c_int as libc::c_longlong
                && (*f).extent < _Sfextern.sf_page as libc::c_longlong
            {
            size = ((*f).extent as size_t)
                .wrapping_add(1024 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_int as libc::c_ulong);
        } else {
            size = _Sfextern.sf_page as size_t;
            if (size as ssize_t) < blksize {
                size = blksize as size_t;
            }
        }
        buf = 0 as *mut libc::c_void;
    }
    sf_malloc = 0 as libc::c_int;
    if size > 0 as libc::c_int as libc::c_ulong && buf.is_null()
        && (*f).bits as libc::c_int & 0o1 as libc::c_int == 0
    {
        if !obuf.is_null() && size == osize as size_t && init != 0 {
            buf = obuf as *mut libc::c_void;
            obuf = 0 as *mut libc::c_uchar;
            sf_malloc = oflags & 0o20 as libc::c_int;
        }
        if buf.is_null() {
            while buf.is_null() && size > 0 as libc::c_int as libc::c_ulong {
                buf = malloc(size);
                if !buf.is_null() {
                    break;
                }
                size = (size as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            if size > 0 as libc::c_int as libc::c_ulong {
                sf_malloc = 0o20 as libc::c_int;
            }
        }
    }
    if size == 0 as libc::c_int as libc::c_ulong
        && (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
        && (*f).bits as libc::c_int & 0o1 as libc::c_int == 0
        && (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0
    {
        size = ::std::mem::size_of::<[libc::c_uchar; 1]>() as libc::c_ulong;
        buf = ((*f).tiny).as_mut_ptr() as *mut libc::c_void;
    }
    (*f).size = size as ssize_t;
    let ref mut fresh16 = (*f).endw;
    *fresh16 = buf as *mut libc::c_uchar;
    let ref mut fresh17 = (*f).endr;
    *fresh17 = *fresh16;
    let ref mut fresh18 = (*f).data;
    *fresh18 = *fresh17;
    let ref mut fresh19 = (*f).next;
    *fresh19 = *fresh18;
    let ref mut fresh20 = (*f).endb;
    *fresh20 = if (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0 {
        (*f).data
    } else {
        ((*f).data).offset(size as isize)
    };
    if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
        (*f)
            .extent = (if sf_malloc == 0
            && ((*f).flags as libc::c_int & 0o1 as libc::c_int != 0
                || (*f).bits as libc::c_int & 0o2 as libc::c_int != 0)
        {
            size
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_longlong;
        (*f).here = 0 as libc::c_int as libc::c_longlong;
        if (*f).mode & 0o1 as libc::c_int as libc::c_uint != 0 && sf_malloc == 0 {
            let ref mut fresh21 = (*f).endb;
            *fresh21 = ((*f).data).offset(size as isize);
        }
    }
    (*f)
        .flags = ((*f).flags as libc::c_int & !(0o20 as libc::c_int) | sf_malloc)
        as libc::c_ushort;
    if !obuf.is_null() && obuf != (*f).data && osize > 0 as libc::c_int as libc::c_long
        && oflags & 0o20 as libc::c_int != 0
    {
        free(obuf as *mut libc::c_void);
        obuf = 0 as *mut libc::c_uchar;
    }
    let ref mut fresh22 = (*f).val;
    *fresh22 = if !obuf.is_null() { osize } else { 0 as libc::c_int as libc::c_long };
    _Sfi = *fresh22;
    if local != 0 {} else {
        (*f).mode
            &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh23 = (*f).endr;
            *fresh23 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh24 = (*f).endw;
                *fresh24 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh25 = (*f).endr;
                *fresh25 = (*f).data;
                let ref mut fresh26 = (*f).endw;
                *fresh26 = *fresh25;
            };
        };
    };
    return obuf as *mut libc::c_void;
}
