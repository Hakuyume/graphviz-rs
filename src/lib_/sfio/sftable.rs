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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
pub type Sftab_t = _sftab_;
unsafe extern "C" fn sffmtint(
    mut str: *const libc::c_char,
    mut v: *mut libc::c_int,
) -> *mut libc::c_char {
    *v = 0 as libc::c_int;
    while *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        *v = *v * 10 as libc::c_int + (*str as libc::c_int - '0' as i32);
        str = str.offset(1);
    }
    *v -= 1 as libc::c_int;
    return str as *mut libc::c_char;
}
unsafe extern "C" fn sffmtpos(
    mut f: *mut Sfio_t,
    mut form: *const libc::c_char,
    mut args: ::core::ffi::VaList,
    mut type_0: libc::c_int,
) -> *mut Fmtpos_t {
    let mut current_block: u64;
    let mut base: libc::c_int = 0;
    let mut fmt: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut dot: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut precis: libc::c_int = 0;
    let mut n_str: ssize_t = 0;
    let mut size: ssize_t = 0 as libc::c_int as ssize_t;
    let mut t_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut dollar: libc::c_int = 0;
    let mut decimal: libc::c_char = 0;
    let mut thousand: libc::c_char = 0;
    let mut ft: *mut Sffmt_t = 0 as *mut Sffmt_t;
    let mut savft: Sffmt_t = Sffmt_t {
        extf: None,
        eventf: None,
        form: 0 as *mut libc::c_char,
        args: (::core::mem::MaybeUninit::uninit()).assume_init(),
        fmt: 0,
        size: 0,
        flags: 0,
        width: 0,
        precis: 0,
        base: 0,
        t_str: 0 as *mut libc::c_char,
        n_str: 0,
    };
    let mut fp: *mut Fmtpos_t = 0 as *mut Fmtpos_t;
    let mut argp: libc::c_int = 0;
    let mut argn: libc::c_int = 0;
    let mut maxp: libc::c_int = 0;
    let mut need: [libc::c_int; 5] = [0; 5];
    if type_0 < 0 as libc::c_int {
        fp = 0 as *mut Fmtpos_t;
    } else {
        fp = sffmtpos(f, form, args.as_va_list(), -(1 as libc::c_int));
        if fp.is_null() {
            return 0 as *mut Fmtpos_t;
        }
    }
    dollar = 0 as libc::c_int;
    thousand = 0 as libc::c_int as libc::c_char;
    decimal = thousand;
    maxp = -(1 as libc::c_int);
    argn = maxp;
    loop {
        n = *form as libc::c_int;
        if !(n != 0) {
            break;
        }
        if n != '%' as i32 {
            let fresh0 = form;
            form = form.offset(1);
            sp = fresh0 as *mut libc::c_char;
            while *form as libc::c_int != 0 && *form as libc::c_int != '%' as i32 {
                form = form.offset(1 as libc::c_int as isize);
            }
        } else {
            form = form.offset(1 as libc::c_int as isize);
            if *form as libc::c_int == 0 as libc::c_int {
                break;
            }
            if *form as libc::c_int == '%' as i32 {
                form = form.offset(1 as libc::c_int as isize);
            } else {
                if *form as libc::c_int == '*' as i32 && type_0 > 0 as libc::c_int {
                    skip = 1 as libc::c_int;
                    form = form.offset(1 as libc::c_int as isize);
                    argp = -(1 as libc::c_int);
                } else {
                    skip = 0 as libc::c_int;
                    sp = sffmtint(form, &mut argp);
                    if *sp as libc::c_int == '$' as i32 {
                        dollar = 1 as libc::c_int;
                        form = sp.offset(1 as libc::c_int as isize);
                    } else {
                        argp = -(1 as libc::c_int);
                    }
                }
                dot = 0 as libc::c_int;
                flags = dot;
                t_str = 0 as *mut libc::c_char;
                n_str = 0 as libc::c_int as ssize_t;
                base = -(1 as libc::c_int);
                precis = base;
                width = precis;
                size = width as ssize_t;
                n = 0 as libc::c_int;
                while n < 5 as libc::c_int {
                    need[n as usize] = -(1 as libc::c_int);
                    n += 1;
                }
                '_loop_flags: loop {
                    let fresh1 = form;
                    form = form.offset(1);
                    fmt = *fresh1 as libc::c_int;
                    match fmt {
                        40 => {
                            t_str = form as *mut libc::c_char;
                            v = 1 as libc::c_int;
                            loop {
                                let fresh2 = form;
                                form = form.offset(1);
                                match *fresh2 as libc::c_int {
                                    0 => {
                                        form = t_str;
                                        t_str = 0 as *mut libc::c_char;
                                        n_str = 0 as libc::c_int as ssize_t;
                                        continue '_loop_flags;
                                    }
                                    40 => {
                                        v += 1 as libc::c_int;
                                    }
                                    41 => {
                                        v -= 1 as libc::c_int;
                                        if v != 0 as libc::c_int {
                                            continue;
                                        }
                                        n_str = form.offset_from(t_str) as libc::c_long;
                                        if *t_str as libc::c_int == '*' as i32 {
                                            t_str = sffmtint(
                                                t_str.offset(1 as libc::c_int as isize),
                                                &mut n,
                                            );
                                            if *t_str as libc::c_int == '$' as i32 {
                                                dollar = 1 as libc::c_int;
                                            } else {
                                                n = -(1 as libc::c_int);
                                            }
                                            n = (if n < 0 as libc::c_int {
                                                argn += 1 as libc::c_int;
                                                argn
                                            } else {
                                                argn = n;
                                                argn
                                            });
                                            if n > maxp {
                                                maxp = n;
                                            }
                                            if !fp.is_null()
                                                && (*fp.offset(n as isize)).ft.fmt
                                                    == 0 as libc::c_int
                                            {
                                                (*fp.offset(n as isize)).ft.fmt = '(' as i32;
                                                let ref mut fresh3 =
                                                    (*fp.offset(n as isize)).ft.form;
                                                *fresh3 = form as *mut libc::c_char;
                                            }
                                            need[3 as libc::c_int as usize] = n;
                                        }
                                        continue '_loop_flags;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        45 => {
                            flags |= 0o100 as libc::c_int;
                            flags &= !(0o1000 as libc::c_int);
                            continue;
                        }
                        48 => {
                            if flags & 0o100 as libc::c_int == 0 {
                                flags |= 0o1000 as libc::c_int;
                            }
                            continue;
                        }
                        32 => {
                            if flags & 0o200 as libc::c_int == 0 {
                                flags |= 0o400 as libc::c_int;
                            }
                            continue;
                        }
                        43 => {
                            flags |= 0o200 as libc::c_int;
                            flags &= !(0o400 as libc::c_int);
                            continue;
                        }
                        35 => {
                            flags |= 0o2000 as libc::c_int;
                            continue;
                        }
                        39 => {
                            let mut lv: *mut lconv = 0 as *mut lconv;
                            if decimal as libc::c_int == 0 as libc::c_int {
                                decimal = '.' as i32 as libc::c_char;
                                lv = localeconv();
                                if !lv.is_null() {
                                    if !((*lv).decimal_point).is_null()
                                        && *((*lv).decimal_point).offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            != 0
                                    {
                                        decimal = *((*lv).decimal_point)
                                            .offset(0 as libc::c_int as isize);
                                    }
                                    if !((*lv).thousands_sep).is_null()
                                        && *((*lv).thousands_sep).offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            != 0
                                    {
                                        thousand = *((*lv).thousands_sep)
                                            .offset(0 as libc::c_int as isize);
                                    }
                                }
                            }
                            if thousand != 0 {
                                flags |= 0o4000 as libc::c_int;
                            }
                            continue;
                        }
                        46 => {
                            dot += 1 as libc::c_int;
                            if dot == 2 as libc::c_int {
                                base = 0 as libc::c_int;
                            }
                            if *(*__ctype_b_loc()).offset(*form as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                let fresh4 = form;
                                form = form.offset(1);
                                fmt = *fresh4 as libc::c_int;
                                current_block = 1290827960411992432;
                            } else {
                                if *form as libc::c_int != '*' as i32 {
                                    continue;
                                }
                                form = form.offset(1 as libc::c_int as isize);
                                current_block = 14995344608209714559;
                            }
                        }
                        42 => {
                            current_block = 14995344608209714559;
                        }
                        49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                            current_block = 1290827960411992432;
                        }
                        73 => {
                            size = 0 as libc::c_int as ssize_t;
                            flags = flags
                                & !(0o20000 as libc::c_int
                                    | 0o10 as libc::c_int
                                    | 0o40000 as libc::c_int
                                    | 0o100000 as libc::c_int
                                    | 0o200000 as libc::c_int
                                    | 0o2000000 as libc::c_int
                                    | 0o4000000 as libc::c_int
                                    | 0o20 as libc::c_int
                                    | 0o40 as libc::c_int)
                                | 0o2000000 as libc::c_int;
                            if *(*__ctype_b_loc()).offset(*form as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                n = *form as libc::c_int;
                                while *(*__ctype_b_loc()).offset(n as isize) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    size = size * 10 as libc::c_int as libc::c_long
                                        + (n - '0' as i32) as libc::c_long;
                                    form = form.offset(1);
                                    n = *form as libc::c_int;
                                }
                            } else if *form as libc::c_int == '*' as i32 {
                                form = sffmtint(form.offset(1 as libc::c_int as isize), &mut n);
                                if *form as libc::c_int == '$' as i32 {
                                    dollar = 1 as libc::c_int;
                                    form = form.offset(1 as libc::c_int as isize);
                                } else {
                                    n = -(1 as libc::c_int);
                                }
                                n = (if n < 0 as libc::c_int {
                                    argn += 1 as libc::c_int;
                                    argn
                                } else {
                                    argn = n;
                                    argn
                                });
                                if n > maxp {
                                    maxp = n;
                                }
                                if !fp.is_null()
                                    && (*fp.offset(n as isize)).ft.fmt == 0 as libc::c_int
                                {
                                    (*fp.offset(n as isize)).ft.fmt = 'I' as i32;
                                    (*fp.offset(n as isize)).ft.size =
                                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                            as ssize_t;
                                    let ref mut fresh6 = (*fp.offset(n as isize)).ft.form;
                                    *fresh6 = form as *mut libc::c_char;
                                }
                                need[4 as libc::c_int as usize] = n;
                            }
                            continue;
                        }
                        108 => {
                            size = -(1 as libc::c_int) as ssize_t;
                            flags &= !(0o20000 as libc::c_int
                                | 0o10 as libc::c_int
                                | 0o40000 as libc::c_int
                                | 0o100000 as libc::c_int
                                | 0o200000 as libc::c_int
                                | 0o2000000 as libc::c_int
                                | 0o4000000 as libc::c_int
                                | 0o20 as libc::c_int
                                | 0o40 as libc::c_int);
                            if *form as libc::c_int == 'l' as i32 {
                                form = form.offset(1 as libc::c_int as isize);
                                flags |= 0o100000 as libc::c_int;
                            } else {
                                flags |= 0o40000 as libc::c_int;
                            }
                            continue;
                        }
                        104 => {
                            size = -(1 as libc::c_int) as ssize_t;
                            flags &= !(0o20000 as libc::c_int
                                | 0o10 as libc::c_int
                                | 0o40000 as libc::c_int
                                | 0o100000 as libc::c_int
                                | 0o200000 as libc::c_int
                                | 0o2000000 as libc::c_int
                                | 0o4000000 as libc::c_int
                                | 0o20 as libc::c_int
                                | 0o40 as libc::c_int);
                            if *form as libc::c_int == 'h' as i32 {
                                form = form.offset(1 as libc::c_int as isize);
                                flags |= 0o10 as libc::c_int;
                            } else {
                                flags |= 0o20000 as libc::c_int;
                            }
                            continue;
                        }
                        76 => {
                            size = -(1 as libc::c_int) as ssize_t;
                            flags = flags
                                & !(0o20000 as libc::c_int
                                    | 0o10 as libc::c_int
                                    | 0o40000 as libc::c_int
                                    | 0o100000 as libc::c_int
                                    | 0o200000 as libc::c_int
                                    | 0o2000000 as libc::c_int
                                    | 0o4000000 as libc::c_int
                                    | 0o20 as libc::c_int
                                    | 0o40 as libc::c_int)
                                | 0o200000 as libc::c_int;
                            continue;
                        }
                        _ => {
                            break;
                        }
                    }
                    match current_block {
                        1290827960411992432 => {
                            v = fmt - '0' as i32;
                            fmt = *form as libc::c_int;
                            while *(*__ctype_b_loc()).offset(fmt as isize) as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                v = v * 10 as libc::c_int + (fmt - '0' as i32);
                                form = form.offset(1);
                                fmt = *form as libc::c_int;
                            }
                            if dot == 0 as libc::c_int {
                                width = v;
                            } else if dot == 1 as libc::c_int {
                                precis = v;
                            } else if dot == 2 as libc::c_int {
                                base = v;
                            }
                        }
                        _ => {
                            form = sffmtint(form, &mut n);
                            if *form as libc::c_int == '$' as i32 {
                                dollar = 1 as libc::c_int;
                                form = form.offset(1 as libc::c_int as isize);
                            } else {
                                n = -(1 as libc::c_int);
                            }
                            n = (if n < 0 as libc::c_int {
                                argn += 1 as libc::c_int;
                                argn
                            } else {
                                argn = n;
                                argn
                            });
                            if n > maxp {
                                maxp = n;
                            }
                            if !fp.is_null() && (*fp.offset(n as isize)).ft.fmt == 0 as libc::c_int
                            {
                                (*fp.offset(n as isize)).ft.fmt = '.' as i32;
                                (*fp.offset(n as isize)).ft.size = dot as ssize_t;
                                let ref mut fresh5 = (*fp.offset(n as isize)).ft.form;
                                *fresh5 = form as *mut libc::c_char;
                            }
                            if dot <= 2 as libc::c_int {
                                need[dot as usize] = n;
                            }
                        }
                    }
                }
                if flags
                    & ((0o20000 as libc::c_int
                        | 0o10 as libc::c_int
                        | 0o40000 as libc::c_int
                        | 0o100000 as libc::c_int
                        | 0o200000 as libc::c_int
                        | 0o2000000 as libc::c_int
                        | 0o4000000 as libc::c_int
                        | 0o20 as libc::c_int
                        | 0o40 as libc::c_int)
                        & !(0o2000000 as libc::c_int))
                    != 0
                {
                    if _Sftable.sf_type[fmt as usize] as libc::c_int
                        & (0o1 as libc::c_int | 0o2 as libc::c_int)
                        != 0
                        || fmt == 'n' as i32
                    {
                        size = (if flags & 0o100000 as libc::c_int != 0 {
                            ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong
                        } else if flags & 0o40000 as libc::c_int != 0 {
                            ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                        } else if flags & 0o20000 as libc::c_int != 0 {
                            ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                        } else if flags & 0o10 as libc::c_int != 0 {
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        } else if flags & 0o4000000 as libc::c_int != 0 {
                            ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong
                        } else if flags & 0o20 as libc::c_int != 0 {
                            ::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong
                        } else if flags & 0o40 as libc::c_int != 0 {
                            ::core::mem::size_of::<size_t>() as libc::c_ulong
                        } else {
                            -(1 as libc::c_int) as libc::c_ulong
                        }) as ssize_t;
                    } else if _Sftable.sf_type[fmt as usize] as libc::c_int & 0o4 as libc::c_int
                        != 0
                    {
                        size = (if flags & 0o200000 as libc::c_int != 0 {
                            ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                        } else if flags & (0o40000 as libc::c_int | 0o100000 as libc::c_int) != 0 {
                            ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                        } else {
                            -(1 as libc::c_int) as libc::c_ulong
                        }) as ssize_t;
                    }
                }
                if skip != 0 {
                    continue;
                }
                argp = (if argp < 0 as libc::c_int {
                    argn += 1 as libc::c_int;
                    argn
                } else {
                    argn = argp;
                    argn
                });
                if argp > maxp {
                    maxp = argp;
                }
                if dollar != 0 && fmt == '!' as i32 {
                    return 0 as *mut Fmtpos_t;
                }
                if !fp.is_null() && (*fp.offset(argp as isize)).ft.fmt == 0 as libc::c_int {
                    let ref mut fresh7 = (*fp.offset(argp as isize)).ft.form;
                    *fresh7 = form as *mut libc::c_char;
                    let ref mut fresh8 = (*fp.offset(argp as isize)).fmt;
                    *fresh8 = fmt;
                    (*fp.offset(argp as isize)).ft.fmt = *fresh8;
                    (*fp.offset(argp as isize)).ft.size = size;
                    (*fp.offset(argp as isize)).ft.flags = flags;
                    (*fp.offset(argp as isize)).ft.width = width;
                    (*fp.offset(argp as isize)).ft.precis = precis;
                    (*fp.offset(argp as isize)).ft.base = base;
                    let ref mut fresh9 = (*fp.offset(argp as isize)).ft.t_str;
                    *fresh9 = t_str;
                    (*fp.offset(argp as isize)).ft.n_str = n_str;
                    n = 0 as libc::c_int;
                    while n < 5 as libc::c_int {
                        (*fp.offset(argp as isize)).need[n as usize] = need[n as usize];
                        n += 1;
                    }
                }
            }
        }
    }
    if fp.is_null() {
        if dollar == 0 || {
            fp = malloc(
                ((maxp + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<Fmtpos_t>() as libc::c_ulong),
            ) as *mut Fmtpos_t;
            fp.is_null()
        } {
            return 0 as *mut Fmtpos_t;
        }
        n = 0 as libc::c_int;
        while n <= maxp {
            (*fp.offset(n as isize)).ft.fmt = 0 as libc::c_int;
            n += 1;
        }
        return fp;
    }
    n = 0 as libc::c_int;
    ft = 0 as *mut Sffmt_t;
    while n <= maxp {
        if (*fp.offset(n as isize)).ft.fmt == 0 as libc::c_int {
            (*fp.offset(n as isize)).ft.fmt = 'd' as i32;
            (*fp.offset(n as isize)).ft.width = 0 as libc::c_int;
            (*fp.offset(n as isize)).ft.precis = 0 as libc::c_int;
            (*fp.offset(n as isize)).ft.base = 0 as libc::c_int;
            (*fp.offset(n as isize)).ft.size = 0 as libc::c_int as ssize_t;
            let ref mut fresh10 = (*fp.offset(n as isize)).ft.t_str;
            *fresh10 = 0 as *mut libc::c_char;
            (*fp.offset(n as isize)).ft.n_str = 0 as libc::c_int as ssize_t;
            (*fp.offset(n as isize)).ft.flags = 0 as libc::c_int;
            v = 0 as libc::c_int;
            while v < 5 as libc::c_int {
                (*fp.offset(n as isize)).need[v as usize] = -(1 as libc::c_int);
                v += 1;
            }
        }
        let mut current_block_236: u64;
        if !ft.is_null() && ((*ft).extf).is_some() {
            let ref mut fresh11 = (*fp.offset(n as isize)).ft.extf;
            *fresh11 = (*ft).extf;
            let ref mut fresh12 = (*fp.offset(n as isize)).ft.eventf;
            *fresh12 = (*ft).eventf;
            v = (*fp.offset(n as isize)).need[0 as libc::c_int as usize];
            if v >= 0 as libc::c_int && v < n {
                (*fp.offset(n as isize)).ft.width = (*fp.offset(v as isize)).argv.i;
            }
            v = (*fp.offset(n as isize)).need[1 as libc::c_int as usize];
            if v >= 0 as libc::c_int && v < n {
                (*fp.offset(n as isize)).ft.precis = (*fp.offset(v as isize)).argv.i;
            }
            v = (*fp.offset(n as isize)).need[2 as libc::c_int as usize];
            if v >= 0 as libc::c_int && v < n {
                (*fp.offset(n as isize)).ft.base = (*fp.offset(v as isize)).argv.i;
            }
            v = (*fp.offset(n as isize)).need[3 as libc::c_int as usize];
            if v >= 0 as libc::c_int && v < n {
                let ref mut fresh13 = (*fp.offset(n as isize)).ft.t_str;
                *fresh13 = (*fp.offset(v as isize)).argv.s;
            }
            v = (*fp.offset(n as isize)).need[4 as libc::c_int as usize];
            if v >= 0 as libc::c_int && v < n {
                (*fp.offset(n as isize)).ft.size = (*fp.offset(v as isize)).argv.i as ssize_t;
            }
            memcpy(
                ft as *mut libc::c_void,
                &mut (*fp.offset(n as isize)).ft as *mut Sffmt_t as *const libc::c_void,
                ::core::mem::size_of::<Sffmt_t>() as libc::c_ulong,
            );
            (*ft).args = args.clone();
            (*ft).flags |= 0o1000000 as libc::c_int;
            v = (Some(((*ft).extf).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                &mut (*fp.offset(n as isize)).argv as *mut Argv_t as *mut libc::c_void,
                ft,
            );
            *args = ((*ft).args).clone();
            memcpy(
                &mut (*fp.offset(n as isize)).ft as *mut Sffmt_t as *mut libc::c_void,
                ft as *const libc::c_void,
                ::core::mem::size_of::<Sffmt_t>() as libc::c_ulong,
            );
            if v < 0 as libc::c_int {
                memcpy(
                    ft as *mut libc::c_void,
                    &mut savft as *mut Sffmt_t as *const libc::c_void,
                    ::core::mem::size_of::<Sffmt_t>() as libc::c_ulong,
                );
                ft = 0 as *mut Sffmt_t;
            }
            if (*fp.offset(n as isize)).ft.flags & 0o400000 as libc::c_int == 0 {
                current_block_236 = 822504560048054417;
            } else {
                current_block_236 = 17723783950272744135;
            }
        } else {
            current_block_236 = 822504560048054417;
        }
        match current_block_236 {
            822504560048054417 => {
                if (*fp.offset(n as isize)).ft.fmt == '(' as i32 {
                    let ref mut fresh14 = (*fp.offset(n as isize)).argv.s;
                    *fresh14 = args.arg::<*mut libc::c_char>();
                    (*fp.offset(n as isize)).ft.size =
                        strlen((*fp.offset(n as isize)).argv.s) as ssize_t;
                } else if (*fp.offset(n as isize)).ft.fmt == '.' as i32
                    || (*fp.offset(n as isize)).ft.fmt == 'I' as i32
                {
                    (*fp.offset(n as isize)).argv.i = args.arg::<libc::c_int>();
                } else if (*fp.offset(n as isize)).ft.fmt == '!' as i32 {
                    if !ft.is_null() {
                        memcpy(
                            ft as *mut libc::c_void,
                            &mut savft as *mut Sffmt_t as *const libc::c_void,
                            ::core::mem::size_of::<Sffmt_t>() as libc::c_ulong,
                        );
                    }
                    ft = args.arg::<*mut Sffmt_t>();
                    let ref mut fresh15 = (*fp.offset(n as isize)).argv.ft;
                    *fresh15 = ft;
                    if !((*ft).form).is_null() {
                        ft = 0 as *mut Sffmt_t;
                    }
                    if !ft.is_null() {
                        memcpy(
                            &mut savft as *mut Sffmt_t as *mut libc::c_void,
                            ft as *const libc::c_void,
                            ::core::mem::size_of::<Sffmt_t>() as libc::c_ulong,
                        );
                    }
                } else if type_0 > 0 as libc::c_int {
                    let ref mut fresh16 = (*fp.offset(n as isize)).argv.vp;
                    *fresh16 = args.arg::<*mut libc::c_void>();
                } else {
                    match _Sftable.sf_type[(*fp.offset(n as isize)).ft.fmt as usize] as libc::c_int
                    {
                        1 | 2 => {
                            if size as libc::c_ulong
                                == ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                                || size == 0 as libc::c_int as libc::c_long
                                    && ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                                        == ::core::mem::size_of::<libc::c_longlong>()
                                            as libc::c_ulong
                                || size == 64 as libc::c_int as libc::c_long
                                    && size as libc::c_ulong
                                        == (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            {
                                (*fp.offset(n as isize)).argv.l = args.arg::<libc::c_long>();
                            } else {
                                (*fp.offset(n as isize)).argv.i = args.arg::<libc::c_int>();
                            }
                        }
                        4 => {
                            (*fp.offset(n as isize)).argv.d = args.arg::<libc::c_double>();
                        }
                        16 => {
                            let ref mut fresh17 = (*fp.offset(n as isize)).argv.vp;
                            *fresh17 = args.arg::<*mut libc::c_void>();
                        }
                        8 => {
                            if (*fp.offset(n as isize)).ft.base >= 0 as libc::c_int {
                                let ref mut fresh18 = (*fp.offset(n as isize)).argv.s;
                                *fresh18 = args.arg::<*mut libc::c_char>();
                            } else {
                                (*fp.offset(n as isize)).argv.c =
                                    args.arg::<libc::c_int>() as libc::c_char;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        n += 1;
    }
    if !ft.is_null() {
        memcpy(
            ft as *mut libc::c_void,
            &mut savft as *mut Sffmt_t as *const libc::c_void,
            ::core::mem::size_of::<Sffmt_t>() as libc::c_ulong,
        );
    }
    return fp;
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
        _Sftable.sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            d as libc::c_uchar;
        _Sftable.sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            d as libc::c_uchar;
        d += 1;
    }
    while d < 36 as libc::c_int {
        _Sftable.sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            d as libc::c_uchar;
        _Sftable.sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            d as libc::c_uchar;
        d += 1;
    }
    l = 10 as libc::c_int;
    while d < 62 as libc::c_int {
        _Sftable.sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            l as libc::c_uchar;
        _Sftable.sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            d as libc::c_uchar;
        l += 1;
        d += 1;
    }
    while d < 64 as libc::c_int {
        _Sftable.sf_cv36[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            d as libc::c_uchar;
        _Sftable.sf_cv64[*(_Sftable.sf_digits).offset(d as isize) as libc::c_uchar as usize] =
            d as libc::c_uchar;
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
                    as for<'a, 'f> unsafe extern "C" fn(
                        *mut Sfio_t,
                        *const libc::c_char,
                        ::std::ffi::VaList<'a, 'f>,
                        libc::c_int,
                    ) -> *mut Fmtpos_t<'a>,
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
