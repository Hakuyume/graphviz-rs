#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn localeconv() -> *mut lconv;
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
#[derive()]
#[repr(C)]
pub struct _sffmt_s<'a> {
    pub extf: Sffmtext_f,
    pub eventf: Sffmtevent_f,
    pub form: *mut libc::c_char,
    pub args: ::std::ffi::VaListImpl::<'a>,
    pub fmt: libc::c_int,
    pub size: ssize_t,
    pub flags: libc::c_int,
    pub width: libc::c_int,
    pub precis: libc::c_int,
    pub base: libc::c_int,
    pub t_str: *mut libc::c_char,
    pub n_str: ssize_t,
}
pub type Sffmtevent_f = Option::<
    unsafe extern "C" fn(
        *mut Sfio_t,
        libc::c_int,
        *mut libc::c_void,
        *mut Sffmt_t,
    ) -> libc::c_int,
>;
pub type Sffmt_t = _sffmt_s;
pub type Sffmtext_f = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut Sffmt_t) -> libc::c_int,
>;
pub type ptrdiff_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type Fmtpos_t = _fmtpos_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fmtpos_s {
    pub ft: Sffmt_t,
    pub argv: Argv_t,
    pub fmt: libc::c_int,
    pub need: [libc::c_int; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Argv_t {
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
    pub ft: *mut Sffmt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sftab_ {
    pub sf_pos10: [f128::f128; 6],
    pub sf_neg10: [f128::f128; 6],
    pub sf_dec: [libc::c_uchar; 200],
    pub sf_digits: *mut libc::c_char,
    pub sf_cvinitf: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub sf_cvinit: libc::c_int,
    pub sf_fmtposf: Option::<
        unsafe extern "C" fn(
            *mut Sfio_t,
            *const libc::c_char,
            ::std::ffi::VaList,
            libc::c_int,
        ) -> *mut Fmtpos_t,
    >,
    pub sf_fmtintf: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_int) -> *mut libc::c_char,
    >,
    pub sf_cv36: [libc::c_uchar; 256],
    pub sf_cv64: [libc::c_uchar; 256],
    pub sf_type: [libc::c_uchar; 256],
}
pub type Sftab_t = _sftab_;
unsafe extern "C" fn sffmtint(
    mut str: *const libc::c_char,
    mut v: *mut libc::c_int,
) -> *mut libc::c_char {
    *v = 0 as libc::c_int;
    while *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *v = *v * 10 as libc::c_int + (*str as libc::c_int - '0' as i32);
        str = str.offset(1);
    }
    *v -= 1 as libc::c_int;
    return str as *mut libc::c_char;
}
unsafe extern "C" fn sfcvinit() -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    d = 0 as libc::c_int;
    while d <= 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
        _Sftable.sf_cv36[d as usize] = 64 as libc::c_int as libc::c_uchar;
        _Sftable.sf_cv64[d as usize] = 64 as libc::c_int as libc::c_uchar;
        d += 1;
    }
    d = 0 as libc::c_int;
    while d < 10 as libc::c_int {
        _Sftable
            .sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = d as libc::c_uchar;
        _Sftable
            .sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = d as libc::c_uchar;
        d += 1;
    }
    while d < 36 as libc::c_int {
        _Sftable
            .sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = d as libc::c_uchar;
        _Sftable
            .sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = d as libc::c_uchar;
        d += 1;
    }
    l = 10 as libc::c_int;
    while d < 62 as libc::c_int {
        _Sftable
            .sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = l as libc::c_uchar;
        _Sftable
            .sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = d as libc::c_uchar;
        l += 1;
        d += 1;
    }
    while d < 64 as libc::c_int {
        _Sftable
            .sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = d as libc::c_uchar;
        _Sftable
            .sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar
            as usize] = d as libc::c_uchar;
        d += 1;
    }
    _Sftable.sf_type['i' as i32 as usize] = 0o1 as libc::c_int as libc::c_uchar;
    _Sftable.sf_type['d' as i32 as usize] = _Sftable.sf_type['i' as i32 as usize];
    _Sftable.sf_type['X' as i32 as usize] = 0o2 as libc::c_int as libc::c_uchar;
    _Sftable.sf_type['x' as i32 as usize] = _Sftable.sf_type['X' as i32 as usize];
    _Sftable.sf_type['o' as i32 as usize] = _Sftable.sf_type['x' as i32 as usize];
    _Sftable.sf_type['u' as i32 as usize] = _Sftable.sf_type['o' as i32 as usize];
    _Sftable.sf_type['f' as i32 as usize] = 0o4 as libc::c_int as libc::c_uchar;
    _Sftable.sf_type['G' as i32 as usize] = _Sftable.sf_type['f' as i32 as usize];
    _Sftable.sf_type['g' as i32 as usize] = _Sftable.sf_type['G' as i32 as usize];
    _Sftable.sf_type['E' as i32 as usize] = _Sftable.sf_type['g' as i32 as usize];
    _Sftable.sf_type['e' as i32 as usize] = _Sftable.sf_type['E' as i32 as usize];
    _Sftable.sf_type['!' as i32 as usize] = 0o20 as libc::c_int as libc::c_uchar;
    _Sftable.sf_type['p' as i32 as usize] = _Sftable.sf_type['!' as i32 as usize];
    _Sftable.sf_type['n' as i32 as usize] = _Sftable.sf_type['p' as i32 as usize];
    _Sftable.sf_type['s' as i32 as usize] = _Sftable.sf_type['n' as i32 as usize];
    _Sftable.sf_type['c' as i32 as usize] = 0o10 as libc::c_int as libc::c_uchar;
    _Sftable.sf_type['[' as i32 as usize] = 0o40 as libc::c_int as libc::c_uchar;
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut _Sftable: Sftab_t = unsafe {
    {
        let mut init = _sftab_ {
            sf_pos10: [
                f128::f128::new(1e1f64),
                f128::f128::new(1e2f64),
                f128::f128::new(1e4f64),
                f128::f128::new(1e8f64),
                f128::f128::new(1e16f64),
                f128::f128::new(1e32f64),
            ],
            sf_neg10: [
                f128::f128::new(1e-1f64),
                f128::f128::new(1e-2f64),
                f128::f128::new(1e-4f64),
                f128::f128::new(1e-8f64),
                f128::f128::new(1e-16f64),
                f128::f128::new(1e-32f64),
            ],
            sf_dec: [
                '0' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '0' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '1' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '2' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '3' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '4' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '5' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '6' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '7' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '8' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
                '9' as i32 as libc::c_uchar,
            ],
            sf_digits: b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@_\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            sf_cvinitf: Some(sfcvinit as unsafe extern "C" fn() -> libc::c_int),
            sf_cvinit: 0 as libc::c_int,
            sf_fmtposf: Some(
                sffmtpos
                    as unsafe extern "C" fn(
                        *mut Sfio_t,
                        *const libc::c_char,
                        ::std::ffi::VaList,
                        libc::c_int,
                    ) -> *mut Fmtpos_t,
            ),
            sf_fmtintf: Some(
                sffmtint
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_int,
                    ) -> *mut libc::c_char,
            ),
            sf_cv36: [0; 256],
            sf_cv64: [0; 256],
            sf_type: [0; 256],
        };
        init
    }
};
