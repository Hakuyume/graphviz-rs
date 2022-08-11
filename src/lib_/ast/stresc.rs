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
    fn chresc(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stresc(mut s: *mut libc::c_char) -> libc::c_int {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    t = s;
    b = t;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        c = *fresh0 as libc::c_int;
        match c {
            92 => {
                c = chresc(s.offset(-(1 as libc::c_int as isize)), &mut p);
                s = p;
            }
            0 => {
                *t = 0 as libc::c_int as libc::c_char;
                return t.offset_from(b) as libc::c_long as libc::c_int;
            }
            _ => {}
        }
        let fresh1 = t;
        t = t.offset(1);
        *fresh1 = c as libc::c_char;
    }
}
