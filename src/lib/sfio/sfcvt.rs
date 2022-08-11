#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
use num_traits::ToPrimitive;
extern "C" {
    static mut _Sfi: ssize_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut _Sftable: Sftab_t;
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
static mut Inf: *mut libc::c_char = b"Inf\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
static mut Zero: *mut libc::c_char = b"0\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn _sfcvt(
    mut dv: *mut libc::c_void,
    mut n_digit: libc::c_int,
    mut decpt: *mut libc::c_int,
    mut sign: *mut libc::c_int,
    mut format: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_long = 0;
    let mut v: libc::c_long = 0;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endsp: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut Buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if Buf.is_null()
        && {
            Buf = malloc(
                ((256 as libc::c_int + 1024 as libc::c_int) as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_char;
            Buf.is_null()
        }
    {
        _Sfi = 3 as libc::c_int as ssize_t;
        return Inf;
    }
    *decpt = 0 as libc::c_int;
    *sign = *decpt;
    let mut dval: libc::c_double = *(dv as *mut libc::c_double);
    if dval == 0.0f64 {
        _Sfi = 1 as libc::c_int as ssize_t;
        return Zero;
    } else {
        *sign = (dval < 0.0f64) as libc::c_int;
        if *sign != 0 {
            dval = -dval;
        }
    }
    n = 0 as libc::c_int as libc::c_long;
    if dval >= 9223372036854775807 as libc::c_long as libc::c_double {
        v = (6 as libc::c_int - 1 as libc::c_int) as libc::c_long;
        loop {
            if f128::f128::new(dval) < _Sftable.sf_pos10[v as usize] {
                v -= 1 as libc::c_int as libc::c_long;
            } else {
                dval = (f128::f128::from(dval) * _Sftable.sf_neg10[v as usize])
                    .to_f64()
                    .unwrap();
                n += ((1 as libc::c_int) << v) as libc::c_long;
                if n >= 1024 as libc::c_int as libc::c_long {
                    _Sfi = 3 as libc::c_int as ssize_t;
                    return Inf;
                }
            }
            if !(dval >= 9223372036854775807 as libc::c_long as libc::c_double) {
                break;
            }
        }
    }
    *decpt = n as libc::c_int;
    sp = Buf.offset((1024 as libc::c_int / 2 as libc::c_int) as isize);
    buf = sp;
    v = dval as libc::c_int as libc::c_long;
    if v != 0 as libc::c_int as libc::c_long {
        dval -= v as libc::c_double;
        while v as uint64_t >= 10000 as libc::c_int as libc::c_ulong {
            n = v;
            v = (v as uint64_t).wrapping_div(10000 as libc::c_int as libc::c_ulong)
                as libc::c_long;
            n = (n as uint64_t)
                .wrapping_sub(
                    (v as uint64_t).wrapping_mul(10000 as libc::c_int as libc::c_ulong),
                ) as libc::c_long;
            sp = sp.offset(-(4 as libc::c_int as isize));
            if n < (5 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                if n < (2 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    if n < (1 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                        *sp
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                    } else {
                        *sp
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '1' as i32 as libc::c_char;
                        n -= (1 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                    }
                } else if n < (3 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '2' as i32 as libc::c_char;
                    n -= (2 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                } else if n < (4 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '3' as i32 as libc::c_char;
                    n -= (3 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(0 as libc::c_int as isize) = '4' as i32 as libc::c_char;
                    n -= (4 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                }
            } else if n < (7 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                if n < (6 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '5' as i32 as libc::c_char;
                    n -= (5 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(0 as libc::c_int as isize) = '6' as i32 as libc::c_char;
                    n -= (6 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                }
            } else if n < (8 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                *sp.offset(0 as libc::c_int as isize) = '7' as i32 as libc::c_char;
                n -= (7 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            } else if n < (9 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                *sp.offset(0 as libc::c_int as isize) = '8' as i32 as libc::c_char;
                n -= (8 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            } else {
                *sp.offset(0 as libc::c_int as isize) = '9' as i32 as libc::c_char;
                n -= (9 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            }
            if n < (5 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                if n < (2 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    if n < (1 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                        *sp
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                    } else {
                        *sp
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '1' as i32 as libc::c_char;
                        n -= (1 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                    }
                } else if n < (3 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(1 as libc::c_int as isize) = '2' as i32 as libc::c_char;
                    n -= (2 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else if n < (4 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(1 as libc::c_int as isize) = '3' as i32 as libc::c_char;
                    n -= (3 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(1 as libc::c_int as isize) = '4' as i32 as libc::c_char;
                    n -= (4 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                }
            } else if n < (7 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                if n < (6 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(1 as libc::c_int as isize) = '5' as i32 as libc::c_char;
                    n -= (5 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(1 as libc::c_int as isize) = '6' as i32 as libc::c_char;
                    n -= (6 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                }
            } else if n < (8 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                *sp.offset(1 as libc::c_int as isize) = '7' as i32 as libc::c_char;
                n -= (7 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            } else if n < (9 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                *sp.offset(1 as libc::c_int as isize) = '8' as i32 as libc::c_char;
                n -= (8 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            } else {
                *sp.offset(1 as libc::c_int as isize) = '9' as i32 as libc::c_char;
                n -= (9 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            }
            n <<= 1 as libc::c_int;
            ep = ((_Sftable.sf_dec).as_mut_ptr() as *mut libc::c_char)
                .offset(n as isize);
            *sp.offset(2 as libc::c_int as isize) = *ep;
            *sp
                .offset(
                    3 as libc::c_int as isize,
                ) = *ep.offset(1 as libc::c_int as isize);
        }
        if v < 100 as libc::c_int as libc::c_long {
            if v < 10 as libc::c_int as libc::c_long {
                sp = sp.offset(-(1 as libc::c_int as isize));
                *sp
                    .offset(
                        0 as libc::c_int as isize,
                    ) = ('0' as i32 as libc::c_long + v) as libc::c_char;
            } else {
                sp = sp.offset(-(2 as libc::c_int as isize));
                v <<= 1 as libc::c_int;
                ep = ((_Sftable.sf_dec).as_mut_ptr() as *mut libc::c_char)
                    .offset(v as isize);
                *sp.offset(0 as libc::c_int as isize) = *ep;
                *sp
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *ep.offset(1 as libc::c_int as isize);
            }
        } else if v < 1000 as libc::c_int as libc::c_long {
            sp = sp.offset(-(3 as libc::c_int as isize));
            if v < (5 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                if v < (2 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    if v < (1 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                        *sp
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                    } else {
                        *sp
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '1' as i32 as libc::c_char;
                        v -= (1 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                    }
                } else if v < (3 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '2' as i32 as libc::c_char;
                    v -= (2 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else if v < (4 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '3' as i32 as libc::c_char;
                    v -= (3 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(0 as libc::c_int as isize) = '4' as i32 as libc::c_char;
                    v -= (4 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                }
            } else if v < (7 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                if v < (6 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '5' as i32 as libc::c_char;
                    v -= (5 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(0 as libc::c_int as isize) = '6' as i32 as libc::c_char;
                    v -= (6 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                }
            } else if v < (8 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                *sp.offset(0 as libc::c_int as isize) = '7' as i32 as libc::c_char;
                v -= (7 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            } else if v < (9 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                *sp.offset(0 as libc::c_int as isize) = '8' as i32 as libc::c_char;
                v -= (8 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            } else {
                *sp.offset(0 as libc::c_int as isize) = '9' as i32 as libc::c_char;
                v -= (9 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            }
            v <<= 1 as libc::c_int;
            ep = ((_Sftable.sf_dec).as_mut_ptr() as *mut libc::c_char)
                .offset(v as isize);
            *sp.offset(1 as libc::c_int as isize) = *ep;
            *sp
                .offset(
                    2 as libc::c_int as isize,
                ) = *ep.offset(1 as libc::c_int as isize);
        } else {
            sp = sp.offset(-(4 as libc::c_int as isize));
            if v < (5 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                if v < (2 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    if v < (1 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                        *sp
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                    } else {
                        *sp
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '1' as i32 as libc::c_char;
                        v -= (1 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                    }
                } else if v < (3 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '2' as i32 as libc::c_char;
                    v -= (2 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                } else if v < (4 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '3' as i32 as libc::c_char;
                    v -= (3 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(0 as libc::c_int as isize) = '4' as i32 as libc::c_char;
                    v -= (4 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                }
            } else if v < (7 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                if v < (6 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                    *sp.offset(0 as libc::c_int as isize) = '5' as i32 as libc::c_char;
                    v -= (5 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(0 as libc::c_int as isize) = '6' as i32 as libc::c_char;
                    v -= (6 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
                }
            } else if v < (8 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                *sp.offset(0 as libc::c_int as isize) = '7' as i32 as libc::c_char;
                v -= (7 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            } else if v < (9 as libc::c_int * 1000 as libc::c_int) as libc::c_long {
                *sp.offset(0 as libc::c_int as isize) = '8' as i32 as libc::c_char;
                v -= (8 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            } else {
                *sp.offset(0 as libc::c_int as isize) = '9' as i32 as libc::c_char;
                v -= (9 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
            }
            if v < (5 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                if v < (2 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    if v < (1 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                        *sp
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '0' as i32 as libc::c_char;
                    } else {
                        *sp
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '1' as i32 as libc::c_char;
                        v -= (1 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                    }
                } else if v < (3 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(1 as libc::c_int as isize) = '2' as i32 as libc::c_char;
                    v -= (2 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else if v < (4 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(1 as libc::c_int as isize) = '3' as i32 as libc::c_char;
                    v -= (3 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(1 as libc::c_int as isize) = '4' as i32 as libc::c_char;
                    v -= (4 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                }
            } else if v < (7 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                if v < (6 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                    *sp.offset(1 as libc::c_int as isize) = '5' as i32 as libc::c_char;
                    v -= (5 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                } else {
                    *sp.offset(1 as libc::c_int as isize) = '6' as i32 as libc::c_char;
                    v -= (6 as libc::c_int * 100 as libc::c_int) as libc::c_long;
                }
            } else if v < (8 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                *sp.offset(1 as libc::c_int as isize) = '7' as i32 as libc::c_char;
                v -= (7 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            } else if v < (9 as libc::c_int * 100 as libc::c_int) as libc::c_long {
                *sp.offset(1 as libc::c_int as isize) = '8' as i32 as libc::c_char;
                v -= (8 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            } else {
                *sp.offset(1 as libc::c_int as isize) = '9' as i32 as libc::c_char;
                v -= (9 as libc::c_int * 100 as libc::c_int) as libc::c_long;
            }
            v <<= 1 as libc::c_int;
            ep = ((_Sftable.sf_dec).as_mut_ptr() as *mut libc::c_char)
                .offset(v as isize);
            *sp.offset(2 as libc::c_int as isize) = *ep;
            *sp
                .offset(
                    3 as libc::c_int as isize,
                ) = *ep.offset(1 as libc::c_int as isize);
        }
        n = buf.offset_from(sp) as libc::c_long;
        *decpt += n as libc::c_int;
        if *decpt >= 1024 as libc::c_int {
            _Sfi = 3 as libc::c_int as ssize_t;
            return Inf;
        }
        buf = sp;
        sp = Buf.offset((1024 as libc::c_int / 2 as libc::c_int) as isize);
    } else {
        n = 0 as libc::c_int as libc::c_long;
    }
    n = (if format & 0o1000000000 as libc::c_int != 0 || *decpt <= 0 as libc::c_int {
        1 as libc::c_int
    } else {
        *decpt + 1 as libc::c_int
    }) as libc::c_long - n;
    if n_digit > 0 as libc::c_int {
        n += n_digit as libc::c_long;
    }
    ep = sp.offset(n as isize);
    endsp = Buf
        .offset(
            ((256 as libc::c_int + 1024 as libc::c_int) as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
        );
    if ep > endsp {
        ep = endsp;
    }
    if sp > ep {
        sp = ep;
        current_block = 9567781456482637466;
    } else {
        if format & 0o1000000000 as libc::c_int != 0 && *decpt == 0 as libc::c_int
            && dval > 0.0f64
        {
            let mut d: libc::c_double = 0.;
            loop {
                d = dval * 10.0f64;
                if !(d as libc::c_int == 0 as libc::c_int) {
                    break;
                }
                dval = d;
                *decpt -= 1 as libc::c_int;
            }
        }
        loop {
            if !(sp < ep) {
                current_block = 9567781456482637466;
                break;
            }
            if dval <= 0.0f64 {
                loop {
                    let fresh0 = sp;
                    sp = sp.offset(1);
                    *fresh0 = '0' as i32 as libc::c_char;
                    if !(sp < ep) {
                        break;
                    }
                }
                current_block = 2759833318435085044;
                break;
            } else {
                dval *= 10.0f64;
                n = dval as libc::c_int as libc::c_long;
                if n < 10 as libc::c_int as libc::c_long {
                    let fresh1 = sp;
                    sp = sp.offset(1);
                    *fresh1 = ('0' as i32 as libc::c_long + n) as libc::c_char;
                    dval -= n as libc::c_double;
                } else {
                    loop {
                        let fresh2 = sp;
                        sp = sp.offset(1);
                        *fresh2 = '9' as i32 as libc::c_char;
                        if !(sp < ep) {
                            break;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        9567781456482637466 => {
            if ep <= buf {
                ep = buf.offset(1 as libc::c_int as isize);
            } else if ep < endsp {
                sp = sp.offset(-1);
                *sp = (*sp as libc::c_int + 5 as libc::c_int) as libc::c_char;
                while *sp as libc::c_int > '9' as i32 {
                    *sp = '0' as i32 as libc::c_char;
                    if sp > buf {
                        sp = sp.offset(-1);
                        *sp = (*sp as libc::c_int + 1 as libc::c_int) as libc::c_char;
                    } else {
                        *sp = '1' as i32 as libc::c_char;
                        *decpt += 1 as libc::c_int;
                        if format & 0o1000000000 as libc::c_int == 0 {
                            *ep
                                .offset(
                                    -(1 as libc::c_int) as isize,
                                ) = '0' as i32 as libc::c_char;
                            ep = ep.offset(1 as libc::c_int as isize);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    ep = ep.offset(-1);
    *ep = '\0' as i32 as libc::c_char;
    _Sfi = ep.offset_from(buf) as libc::c_long;
    return buf;
}
