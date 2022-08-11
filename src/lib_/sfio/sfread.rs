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
    fn _sffilbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn sfrd(_: *mut Sfio_t, _: *mut libc::c_void, _: size_t, _: *mut Sfdisc_t) -> ssize_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
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
pub unsafe extern "C" fn sfread(
    mut f: *mut Sfio_t,
    mut buf: *mut libc::c_void,
    mut n: size_t,
) -> ssize_t {
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut begs: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: ssize_t = 0;
    let mut local: libc::c_int = 0;
    let mut justseek: libc::c_int = 0;
    if f.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    local = ((*f).mode & 0o100000 as libc::c_uint) as libc::c_int;
    (*f).mode &= !(0o100000 as libc::c_uint);
    justseek = (*f).bits as libc::c_int & 0o40 as libc::c_int;
    let ref mut fresh0 = (*f).bits;
    *fresh0 = (*fresh0 as libc::c_int & !(0o40 as libc::c_int) as libc::c_ushort as libc::c_int)
        as libc::c_ushort;
    if buf.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    if (*f).mode & 0o400 as libc::c_uint != 0 {
        if (*f).mode & 0o1 as libc::c_int as libc::c_uint == 0 {
            return -(1 as libc::c_int) as ssize_t;
        }
        if (*f).mode & 0o2000 as libc::c_uint != 0 {
            if (buf as *mut libc::c_uchar).offset((*f).val as isize) != (*f).next
                && (((*f).rsrv).is_null()
                    || ((*(*f).rsrv).data).as_mut_ptr() != buf as *mut libc::c_uchar)
            {
                return -(1 as libc::c_int) as ssize_t;
            }
            (*f).mode &= !(0o400 as libc::c_uint);
            return 0 as libc::c_int as ssize_t;
        } else {
            if buf != (*f).next as *mut libc::c_void {
                return -(1 as libc::c_int) as ssize_t;
            }
            (*f).mode &= !(0o400 as libc::c_uint);
            if (*f).mode & 0o1000 as libc::c_uint != 0 {
                (*f).mode &= !(0o1000 as libc::c_uint);
                if n > 0 as libc::c_int as libc::c_ulong {
                    r = read((*f).file as libc::c_int, (*f).data as *mut libc::c_void, n);
                    n = (if r < 0 as libc::c_int as libc::c_long {
                        0 as libc::c_int as libc::c_long
                    } else {
                        r
                    }) as size_t;
                }
                let ref mut fresh1 = (*f).endb;
                *fresh1 = ((*f).data).offset(n as isize);
                let ref mut fresh2 = (*f).here;
                *fresh2 = (*fresh2 as libc::c_ulonglong).wrapping_add(n as libc::c_ulonglong)
                    as libc::c_longlong as libc::c_longlong;
            }
            let ref mut fresh3 = (*f).next;
            *fresh3 = (*fresh3).offset(n as isize);
            let ref mut fresh4 = (*f).endr;
            *fresh4 = (*f).endb;
            return n as ssize_t;
        }
    }
    begs = buf as *mut libc::c_uchar;
    s = begs;
    let mut current_block_73: u64;
    loop {
        if (*f).mode
            & !(0o20 as libc::c_uint
                | 0o10 as libc::c_uint
                | (if local != 0 {
                    0o40 as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }))
            != 0o1 as libc::c_int as libc::c_uint
            && _sfmode(f, 0o1 as libc::c_int, local) < 0 as libc::c_int
        {
            n = if s > begs {
                s.offset_from(begs) as libc::c_long as libc::c_ulong
            } else {
                -(1 as libc::c_int) as size_t
            };
            return n as ssize_t;
        }
        (*f).mode |= 0o40 as libc::c_uint;
        let ref mut fresh5 = (*f).endw;
        *fresh5 = (*f).data;
        let ref mut fresh6 = (*f).endr;
        *fresh6 = *fresh5;
        r = ((*f).endb).offset_from((*f).next) as libc::c_long;
        if r > 0 as libc::c_int as libc::c_long {
            if r > n as ssize_t {
                r = n as ssize_t;
            }
            if s != (*f).next {
                memcpy(
                    s as *mut libc::c_void,
                    (*f).next as *const libc::c_void,
                    r as libc::c_ulong,
                );
            }
            let ref mut fresh7 = (*f).next;
            *fresh7 = (*fresh7).offset(r as isize);
            s = s.offset(r as isize);
            n = (n as libc::c_ulong).wrapping_sub(r as libc::c_ulong) as size_t as size_t;
        }
        if n <= 0 as libc::c_int as libc::c_ulong {
            break;
        } else {
            if (*f).flags as libc::c_int & 0o4 as libc::c_int == 0
                && (*f).bits as libc::c_int & 0o1 as libc::c_int == 0
            {
                let ref mut fresh8 = (*f).endb;
                *fresh8 = (*f).data;
                let ref mut fresh9 = (*f).next;
                *fresh9 = *fresh8;
                if n as ssize_t >= (*f).size
                    || n >= 1024 as libc::c_int as libc::c_ulong
                        && n as ssize_t >= (*f).size / 16 as libc::c_int as libc::c_long
                    || (*f).flags as libc::c_int & 0o100 as libc::c_int != 0
                        && (*f).extent < 0 as libc::c_int as libc::c_longlong
                {
                    r = n as ssize_t;
                } else if justseek != 0 && n <= (*f).iosz && (*f).iosz <= (*f).size as libc::c_ulong
                {
                    r = (*f).iosz as ssize_t;
                } else {
                    r = (*f).size;
                }
                if r > n as ssize_t && r - r / 8 as libc::c_int as libc::c_long <= n as ssize_t {
                    r = n as ssize_t;
                }
                if r == n as ssize_t && {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    r = sfrd(f, s as *mut libc::c_void, r as size_t, (*f).disc);
                    r >= 0 as libc::c_int as libc::c_long
                } {
                    s = s.offset(r as isize);
                    n = (n as libc::c_ulong).wrapping_sub(r as libc::c_ulong) as size_t as size_t;
                    if r == 0 as libc::c_int as libc::c_long
                        || n == 0 as libc::c_int as libc::c_ulong
                    {
                        break;
                    }
                    current_block_73 = 15090052786889560393;
                } else {
                    current_block_73 = 517265880607398503;
                }
            } else {
                current_block_73 = 517265880607398503;
            }
            match current_block_73 {
                517265880607398503 => {
                    if justseek != 0 {
                        let ref mut fresh10 = (*f).bits;
                        *fresh10 =
                            (*fresh10 as libc::c_int | 0o40 as libc::c_int) as libc::c_ushort;
                    }
                    (*f).mode |= 0o100000 as libc::c_uint;
                    if _sffilbuf(f, -(1 as libc::c_int)) <= 0 as libc::c_int {
                        break;
                    }
                }
                _ => {}
            }
            (*f).mode &= !(0o40 as libc::c_uint);
        }
    }
    if local != 0 {
    } else {
        (*f).mode &= !(0o40 as libc::c_uint | 0o10 as libc::c_uint | 0o20 as libc::c_uint);
        if (*f).mode == 0o1 as libc::c_int as libc::c_uint {
            let ref mut fresh11 = (*f).endr;
            *fresh11 = (*f).endb;
        } else {
            if (*f).mode == 0o2 as libc::c_int as libc::c_uint {
                let ref mut fresh12 = (*f).endw;
                *fresh12 = (if (*f).flags as libc::c_int & 0o40 as libc::c_int != 0 {
                    (*f).data
                } else {
                    (*f).endb
                });
            } else {
                let ref mut fresh13 = (*f).endr;
                *fresh13 = (*f).data;
                let ref mut fresh14 = (*f).endw;
                *fresh14 = *fresh13;
            };
        };
    };
    r = s.offset_from(begs) as libc::c_long;
    return r;
}
