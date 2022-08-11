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
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    static mut _Sfextern: Sfextern_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub unsafe extern "C" fn sfnew(
    mut oldf: *mut Sfio_t,
    mut buf: *mut libc::c_void,
    mut size: size_t,
    mut file: libc::c_int,
    mut flags: libc::c_int,
) -> *mut Sfio_t {
    let mut f: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut sflags: libc::c_int = 0;
    if flags & (0o1 as libc::c_int | 0o2 as libc::c_int) == 0 {
        return 0 as *mut Sfio_t;
    }
    sflags = 0 as libc::c_int;
    f = oldf;
    if !f.is_null() {
        if flags & 0o200 as libc::c_int != 0 {
            oldf = 0 as *mut Sfio_t;
        } else if (*f).mode & 0o20000 as libc::c_uint != 0 {
            if (*f).flags as libc::c_int & 0o1000 as libc::c_int == 0 {
                return 0 as *mut Sfio_t;
            }
            sflags = (*f).flags as libc::c_int;
            oldf = 0 as *mut Sfio_t;
        } else {
            sflags = (*f).flags as libc::c_int;
            if (*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint != (*f).mode
                && _sfmode(f, 0 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
                || {
                    (*f).mode |= 0o100000 as libc::c_uint;
                    sfclose(f) < 0 as libc::c_int
                }
            {
                return 0 as *mut Sfio_t;
            }
            if !((*f).data).is_null()
                && (flags & 0o4 as libc::c_int != 0
                    || size != 18446744073709551615 as libc::c_ulong)
            {
                if sflags & 0o20 as libc::c_int != 0 {
                    free((*f).data as *mut libc::c_void);
                }
                let ref mut fresh0 = (*f).data;
                *fresh0 = 0 as *mut libc::c_uchar;
            }
            if ((*f).data).is_null() {
                sflags &= !(0o20 as libc::c_int) as libc::c_ushort as libc::c_int;
            }
        }
    }
    if f.is_null() {
        if flags & 0o4 as libc::c_int == 0 && file >= 0 as libc::c_int && file <= 2 as libc::c_int {
            f = if file == 0 as libc::c_int {
                sfstdin
            } else if file == 1 as libc::c_int {
                sfstdout
            } else {
                sfstderr
            };
            if !f.is_null() {
                if (*f).mode & 0o20000 as libc::c_uint != 0 {
                    sflags = (*f).flags as libc::c_int;
                } else {
                    f = 0 as *mut Sfio_t;
                }
            }
        }
        if f.is_null() {
            f = malloc(::std::mem::size_of::<Sfio_t>() as libc::c_ulong) as *mut Sfio_t;
            if f.is_null() {
                return 0 as *mut Sfio_t;
            }
            let ref mut fresh1 = (*f).next;
            *fresh1 = 0 as *mut libc::c_uchar;
            let ref mut fresh2 = (*f).endw;
            *fresh2 = 0 as *mut libc::c_uchar;
            let ref mut fresh3 = (*f).endr;
            *fresh3 = 0 as *mut libc::c_uchar;
            let ref mut fresh4 = (*f).endb;
            *fresh4 = 0 as *mut libc::c_uchar;
            let ref mut fresh5 = (*f).push;
            *fresh5 = 0 as *mut Sfio_t;
            (*f).flags = 0 as libc::c_int as libc::c_ushort;
            (*f).file = -(1 as libc::c_int) as libc::c_short;
            let ref mut fresh6 = (*f).data;
            *fresh6 = 0 as *mut libc::c_uchar;
            (*f).size = -(1 as libc::c_int) as ssize_t;
            (*f).val = -(1 as libc::c_int) as ssize_t;
            (*f).extent = -(1 as libc::c_int) as libc::c_longlong;
            (*f).here = 0 as libc::c_int as libc::c_longlong;
            (*f).getr = 0 as libc::c_int as libc::c_uchar;
            (*f).tiny[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            (*f).bits = 0 as libc::c_int as libc::c_ushort;
            (*f).mode = 0 as libc::c_int as libc::c_uint;
            let ref mut fresh7 = (*f).disc;
            *fresh7 = 0 as *mut _sfdisc_s;
            let ref mut fresh8 = (*f).pool;
            *fresh8 = 0 as *mut _sfpool_s;
            let ref mut fresh9 = (*f).rsrv;
            *fresh9 = 0 as *mut _sfrsrv_s;
            let ref mut fresh10 = (*f).proc_0;
            *fresh10 = 0 as *mut _sfproc_s;
            let ref mut fresh11 = (*f).stdio;
            *fresh11 = 0 as *mut libc::c_void;
            (*f).lpos = 0 as libc::c_int as libc::c_longlong;
            (*f).iosz = 0 as libc::c_int as size_t;
        }
    }
    (*f).mode = (if flags & 0o1 as libc::c_int != 0 {
        0o1 as libc::c_int
    } else {
        0o2 as libc::c_int
    }) as libc::c_uint;
    (*f).flags = (flags & 0o77177 as libc::c_int
        | sflags & (0o20 as libc::c_int | 0o1000 as libc::c_int))
        as libc::c_ushort;
    (*f).bits = (if flags & (0o1 as libc::c_int | 0o2 as libc::c_int)
        == 0o1 as libc::c_int | 0o2 as libc::c_int
    {
        0o2 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_ushort;
    (*f).file = file as libc::c_short;
    let ref mut fresh12 = (*f).extent;
    *fresh12 = 0 as libc::c_int as libc::c_longlong;
    (*f).here = *fresh12;
    let ref mut fresh13 = (*f).tiny[0 as libc::c_int as usize];
    *fresh13 = 0 as libc::c_int as libc::c_uchar;
    (*f).getr = *fresh13;
    (*f).mode |= 0o4 as libc::c_uint;
    if size != 18446744073709551615 as libc::c_ulong {
        (*f).size = size as ssize_t;
        let ref mut fresh14 = (*f).data;
        *fresh14 = if size <= 0 as libc::c_int as libc::c_ulong {
            0 as *mut libc::c_uchar
        } else {
            buf as *mut libc::c_uchar
        };
    }
    let ref mut fresh15 = (*f).next;
    *fresh15 = (*f).data;
    let ref mut fresh16 = (*f).endw;
    *fresh16 = *fresh15;
    let ref mut fresh17 = (*f).endr;
    *fresh17 = *fresh16;
    let ref mut fresh18 = (*f).endb;
    *fresh18 = *fresh17;
    if (_Sfextern.sf_notify).is_some() {
        (_Sfextern.sf_notify).expect("non-null function pointer")(
            f,
            0 as libc::c_int,
            (*f).file as libc::c_int,
        );
    }
    if (*f).flags as libc::c_int & 0o4 as libc::c_int != 0 {
        _sfmode(
            f,
            ((*f).mode & (0o1 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint) as libc::c_int,
            0 as libc::c_int,
        );
    }
    return f;
}
