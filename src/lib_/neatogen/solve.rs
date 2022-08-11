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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn solve(
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut c: *mut libc::c_double,
    mut n: libc::c_int,
) {
    let mut current_block: u64;
    let mut asave: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut csave: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut amax: libc::c_double = 0.;
    let mut dum: libc::c_double = 0.;
    let mut pivot: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut mp: libc::c_int = 0;
    let mut istar: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut nm: libc::c_int = 0;
    let mut nsq: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    istar = 0 as libc::c_int;
    nsq = n * n;
    asave = gcalloc(
        nsq as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    csave = gcalloc(
        n as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < n {
        *csave.offset(i as isize) = *c.offset(i as isize);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nsq {
        *asave.offset(i as isize) = *a.offset(i as isize);
        i += 1;
    }
    nm = n - 1 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        if !(i < nm) {
            current_block = 5330834795799507926;
            break;
        }
        amax = 0.0f64;
        ii = i;
        while ii < n {
            dum = fabs(*a.offset((ii * n + i) as isize));
            if !(dum < amax) {
                istar = ii;
                amax = dum;
            }
            ii += 1;
        }
        if amax < 1.0e-10f64 {
            current_block = 16076866197496778268;
            break;
        }
        j = i;
        while j < n {
            t = istar * n + j;
            dum = *a.offset(t as isize);
            *a.offset(t as isize) = *a.offset((i * n + j) as isize);
            *a.offset((i * n + j) as isize) = dum;
            j += 1;
        }
        dum = *c.offset(istar as isize);
        *c.offset(istar as isize) = *c.offset(i as isize);
        *c.offset(i as isize) = dum;
        ip = i + 1 as libc::c_int;
        ii = ip;
        while ii < n {
            pivot = *a.offset((ii * n + i) as isize) / *a.offset((i * n + i) as isize);
            *c.offset(ii as isize) = *c.offset(ii as isize) - pivot * *c.offset(i as isize);
            j = 0 as libc::c_int;
            while j < n {
                *a.offset((ii * n + j) as isize) =
                    *a.offset((ii * n + j) as isize) - pivot * *a.offset((i * n + j) as isize);
                j += 1;
            }
            ii += 1;
        }
        i += 1;
    }
    match current_block {
        5330834795799507926 => {
            if !(fabs(*a.offset((n * n - 1 as libc::c_int) as isize)) < 1.0e-10f64) {
                *b.offset((n - 1 as libc::c_int) as isize) = *c
                    .offset((n - 1 as libc::c_int) as isize)
                    / *a.offset((n * n - 1 as libc::c_int) as isize);
                k = 0 as libc::c_int;
                while k < nm {
                    m = n - k - 2 as libc::c_int;
                    *b.offset(m as isize) = *c.offset(m as isize);
                    mp = m + 1 as libc::c_int;
                    j = mp;
                    while j < n {
                        *b.offset(m as isize) = *b.offset(m as isize)
                            - *a.offset((m * n + j) as isize) * *b.offset(j as isize);
                        j += 1;
                    }
                    *b.offset(m as isize) = *b.offset(m as isize) / *a.offset((m * n + m) as isize);
                    k += 1;
                }
                i = 0 as libc::c_int;
                while i < n {
                    *c.offset(i as isize) = *csave.offset(i as isize);
                    i += 1;
                }
                i = 0 as libc::c_int;
                while i < nsq {
                    *a.offset(i as isize) = *asave.offset(i as isize);
                    i += 1;
                }
                free(asave as *mut libc::c_void);
                free(csave as *mut libc::c_void);
                return;
            }
        }
        _ => {}
    }
    printf(b"ill-conditioned\n\0" as *const u8 as *const libc::c_char);
    free(asave as *mut libc::c_void);
    free(csave as *mut libc::c_void);
}
