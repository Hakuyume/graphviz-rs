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
    static mut _Sfi: ssize_t;
    fn sfwrite(_: *mut Sfio_t, _: *const libc::c_void, _: size_t) -> ssize_t;
    fn sfnputc(_: *mut Sfio_t, _: libc::c_int, _: size_t) -> ssize_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn _sfflsbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn localeconv() -> *mut lconv;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _sfcvt(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    static mut _Sftable: Sftab_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _sfmode(_: *mut Sfio_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Clone)]
#[repr(C)]
pub struct _sffmt_s<'a> {
    pub extf: Sffmtext_f,
    pub eventf: Sffmtevent_f,
    pub form: *mut libc::c_char,
    pub args: ::std::ffi::VaListImpl<'a>,
    pub fmt: libc::c_int,
    pub size: ssize_t,
    pub flags: libc::c_int,
    pub width: libc::c_int,
    pub precis: libc::c_int,
    pub base: libc::c_int,
    pub t_str: *mut libc::c_char,
    pub n_str: ssize_t,
}
pub type Sffmtevent_f = Option<
    unsafe extern "C" fn(*mut Sfio_t, libc::c_int, *mut libc::c_void, *mut Sffmt_t) -> libc::c_int,
>;
pub type Sffmt_t<'a> = _sffmt_s<'a>;
pub type Sffmtext_f = Option<unsafe extern "C" fn(*mut libc::c_void, *mut Sffmt_t) -> libc::c_int>;
pub type Fmt_t<'a> = _fmt_s<'a>;
#[derive()]
#[repr(C)]
pub struct _fmt_s<'a> {
    pub form: *mut libc::c_char,
    pub args: ::std::ffi::VaListImpl<'a>,
    pub oform: *mut libc::c_char,
    pub oargs: ::std::ffi::VaListImpl<'a>,
    pub argn: libc::c_int,
    pub fp: *mut Fmtpos_t<'a>,
    pub ft: *mut Sffmt_t<'a>,
    pub eventf: Sffmtevent_f,
    pub next: *mut Fmt_t<'a>,
}
pub type Fmtpos_t<'a> = _fmtpos_s<'a>;
#[derive(Clone)]
#[repr(C)]
pub struct _fmtpos_s<'a> {
    pub ft: Sffmt_t<'a>,
    pub argv: Argv_t<'a>,
    pub fmt: libc::c_int,
    pub need: [libc::c_int; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Argv_t<'a> {
    pub i: libc::c_int,
    pub ip: *mut libc::c_int,
    pub l: libc::c_long,
    pub lp: *mut libc::c_long,
    pub h: libc::c_short,
    pub hp: *mut libc::c_short,
    pub ui: libc::c_uint,
    pub ul: uint64_t,
    pub uh: libc::c_ushort,
    pub ll: libc::c_longlong,
    pub llp: *mut libc::c_longlong,
    pub lu: libc::c_ulonglong,
    pub ld: f128::f128,
    pub d: libc::c_double,
    pub f: libc::c_float,
    pub c: libc::c_char,
    pub s: *mut libc::c_char,
    pub sp: *mut *mut libc::c_char,
    pub vp: *mut libc::c_void,
    pub ft: *mut Sffmt_t<'a>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISupper: C2RustUnnamed = 256;
pub type Sftab_t = _sftab_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sftab_ {
    pub sf_pos10: [f128::f128; 6],
    pub sf_neg10: [f128::f128; 6],
    pub sf_dec: [libc::c_uchar; 200],
    pub sf_digits: *mut libc::c_char,
    pub sf_cvinitf: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub sf_cvinit: libc::c_int,
    pub sf_fmtposf: Option<
        for<'a, 'f> unsafe extern "C" fn(
            *mut Sfio_t,
            *const libc::c_char,
            ::std::ffi::VaList<'a, 'f>,
            libc::c_int,
        ) -> *mut Fmtpos_t<'a>,
    >,
    pub sf_fmtintf:
        Option<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_int) -> *mut libc::c_char>,
    pub sf_cv36: [libc::c_uchar; 256],
    pub sf_cv64: [libc::c_uchar; 256],
    pub sf_type: [libc::c_uchar; 256],
}
pub type ptrdiff_t = libc::c_long;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalnum: C2RustUnnamed = 8;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISlower: C2RustUnnamed = 512;
