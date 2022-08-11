#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
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
pub static mut _Sfextern: Sfextern_t = Sfextern_t {
    sf_page: 0,
    sf_pool: _sfpool_s {
        next: 0 as *const Sfpool_t as *mut Sfpool_t,
        mode: 0,
        s_sf: 0,
        n_sf: 0,
        sf: 0 as *const *mut Sfio_t as *mut *mut Sfio_t,
        array: [0 as *const Sfio_t as *mut Sfio_t; 3],
    },
    sf_pmove: None,
    sf_stack: None,
    sf_notify: None,
    sf_stdsync: None,
    sf_udisc: _sfdisc_s {
        readf: None,
        writef: None,
        seekf: None,
        exceptf: None,
        disc: 0 as *const Sfdisc_t as *mut Sfdisc_t,
    },
    sf_cleanup: None,
    sf_exiting: 0,
    sf_done: 0,
};
#[no_mangle]
pub static mut _Sfi: ssize_t = -(1 as libc::c_int) as ssize_t;
#[no_mangle]
pub static mut _Sfstdin: Sfio_t = {
    let mut init = _sfio_s {
        next: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endw: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endr: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endb: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        push: 0 as *const Sfio_t as *mut Sfio_t,
        flags: ((0o1 as libc::c_int | 0o1000 as libc::c_int) & 0o77177 as libc::c_int)
            as libc::c_ushort,
        file: 0 as libc::c_int as libc::c_short,
        data: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        size: -(1 as libc::c_int) as ssize_t,
        val: -(1 as libc::c_int) as ssize_t,
        extent: 0 as libc::c_int as libc::c_longlong,
        here: 0 as libc::c_int as libc::c_longlong,
        getr: 0 as libc::c_int as libc::c_uchar,
        tiny: [0 as libc::c_int as libc::c_uchar],
        bits: 0 as libc::c_int as libc::c_ushort,
        mode: ((0o1 as libc::c_int | 0o1000 as libc::c_int)
            & (0o1 as libc::c_int | 0o2 as libc::c_int)) as libc::c_uint
            | 0o4 as libc::c_uint,
        disc: 0 as *const libc::c_void as *mut libc::c_void as *mut _sfdisc_s,
        pool: 0 as *const _sfpool_s as *mut _sfpool_s,
        rsrv: 0 as *const _sfrsrv_s as *mut _sfrsrv_s,
        proc_0: 0 as *const _sfproc_s as *mut _sfproc_s,
        stdio: 0 as *const libc::c_void as *mut libc::c_void,
        lpos: 0 as libc::c_int as libc::c_longlong,
        iosz: 0 as libc::c_int as size_t,
    };
    init
};
#[no_mangle]
pub static mut _Sfstdout: Sfio_t = {
    let mut init = _sfio_s {
        next: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endw: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endr: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endb: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        push: 0 as *const Sfio_t as *mut Sfio_t,
        flags: ((0o2 as libc::c_int | 0o1000 as libc::c_int) & 0o77177 as libc::c_int)
            as libc::c_ushort,
        file: 1 as libc::c_int as libc::c_short,
        data: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        size: -(1 as libc::c_int) as ssize_t,
        val: -(1 as libc::c_int) as ssize_t,
        extent: 0 as libc::c_int as libc::c_longlong,
        here: 0 as libc::c_int as libc::c_longlong,
        getr: 0 as libc::c_int as libc::c_uchar,
        tiny: [0 as libc::c_int as libc::c_uchar],
        bits: 0 as libc::c_int as libc::c_ushort,
        mode: ((0o2 as libc::c_int | 0o1000 as libc::c_int)
            & (0o1 as libc::c_int | 0o2 as libc::c_int)) as libc::c_uint
            | 0o4 as libc::c_uint,
        disc: 0 as *const libc::c_void as *mut libc::c_void as *mut _sfdisc_s,
        pool: 0 as *const _sfpool_s as *mut _sfpool_s,
        rsrv: 0 as *const _sfrsrv_s as *mut _sfrsrv_s,
        proc_0: 0 as *const _sfproc_s as *mut _sfproc_s,
        stdio: 0 as *const libc::c_void as *mut libc::c_void,
        lpos: 0 as libc::c_int as libc::c_longlong,
        iosz: 0 as libc::c_int as size_t,
    };
    init
};
#[no_mangle]
pub static mut _Sfstderr: Sfio_t = {
    let mut init = _sfio_s {
        next: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endw: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endr: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        endb: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        push: 0 as *const Sfio_t as *mut Sfio_t,
        flags: ((0o2 as libc::c_int | 0o1000 as libc::c_int) & 0o77177 as libc::c_int)
            as libc::c_ushort,
        file: 2 as libc::c_int as libc::c_short,
        data: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_uchar,
        size: -(1 as libc::c_int) as ssize_t,
        val: -(1 as libc::c_int) as ssize_t,
        extent: 0 as libc::c_int as libc::c_longlong,
        here: 0 as libc::c_int as libc::c_longlong,
        getr: 0 as libc::c_int as libc::c_uchar,
        tiny: [0 as libc::c_int as libc::c_uchar],
        bits: 0 as libc::c_int as libc::c_ushort,
        mode: ((0o2 as libc::c_int | 0o1000 as libc::c_int)
            & (0o1 as libc::c_int | 0o2 as libc::c_int)) as libc::c_uint
            | 0o4 as libc::c_uint,
        disc: 0 as *const libc::c_void as *mut libc::c_void as *mut _sfdisc_s,
        pool: 0 as *const _sfpool_s as *mut _sfpool_s,
        rsrv: 0 as *const _sfrsrv_s as *mut _sfrsrv_s,
        proc_0: 0 as *const _sfproc_s as *mut _sfproc_s,
        stdio: 0 as *const libc::c_void as *mut libc::c_void,
        lpos: 0 as libc::c_int as libc::c_longlong,
        iosz: 0 as libc::c_int as size_t,
    };
    init
};
#[no_mangle]
pub static mut sfstdin: *mut Sfio_t = unsafe {
    &_Sfstdin as *const Sfio_t as *mut Sfio_t
};
#[no_mangle]
pub static mut sfstdout: *mut Sfio_t = unsafe {
    &_Sfstdout as *const Sfio_t as *mut Sfio_t
};
#[no_mangle]
pub static mut sfstderr: *mut Sfio_t = unsafe {
    &_Sfstderr as *const Sfio_t as *mut Sfio_t
};
