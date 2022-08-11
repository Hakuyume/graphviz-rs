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
#[no_mangle]
pub unsafe extern "C" fn pathcat(
    mut path: *mut libc::c_char,
    mut dirs: *const libc::c_char,
    mut sep: libc::c_int,
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = path;
    while *dirs as libc::c_int != 0 && *dirs as libc::c_int != sep {
        let fresh0 = dirs;
        dirs = dirs.offset(1);
        let fresh1 = s;
        s = s.offset(1);
        *fresh1 = *fresh0;
    }
    if s != path {
        let fresh2 = s;
        s = s.offset(1);
        *fresh2 = '/' as i32 as libc::c_char;
    }
    if !a.is_null() {
        loop {
            let fresh3 = a;
            a = a.offset(1);
            *s = *fresh3;
            if !(*s != 0) {
                break;
            }
            s = s.offset(1);
        }
        if !b.is_null() {
            let fresh4 = s;
            s = s.offset(1);
            *fresh4 = '/' as i32 as libc::c_char;
        }
    } else if b.is_null() {
        b = b".\0" as *const u8 as *const libc::c_char;
    }
    if !b.is_null() {
        loop {
            let fresh5 = b;
            b = b.offset(1);
            let fresh6 = s;
            s = s.offset(1);
            *fresh6 = *fresh5;
            if !(*fresh6 != 0) {
                break;
            }
        }
    }
    return if *dirs as libc::c_int != 0 {
        dirs = dirs.offset(1);
        dirs
    } else {
        0 as *const libc::c_char
    };
}
