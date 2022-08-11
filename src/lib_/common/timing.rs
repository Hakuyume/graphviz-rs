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
    fn times(__buffer: *mut tms) -> clock_t;
}
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}
pub type mytime_t = tms;
static mut T: mytime_t = mytime_t {
    tms_utime: 0,
    tms_stime: 0,
    tms_cutime: 0,
    tms_cstime: 0,
};
#[no_mangle]
pub unsafe extern "C" fn start_timer() {
    times(&mut T);
}
#[no_mangle]
pub unsafe extern "C" fn elapsed_sec() -> libc::c_double {
    let mut S: mytime_t = mytime_t {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut rv: libc::c_double = 0.;
    times(&mut S);
    rv = (S.tms_utime + S.tms_stime - T.tms_utime - T.tms_stime) as libc::c_double
        / 100 as libc::c_int as libc::c_double;
    return rv;
}
