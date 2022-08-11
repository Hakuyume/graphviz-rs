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
pub unsafe extern "C" fn strcopy(
    mut s: *mut libc::c_char,
    mut t: *const libc::c_char,
) -> *mut libc::c_char {
    if t.is_null() {
        return s;
    }
    loop {
        let fresh0 = t;
        t = t.offset(1);
        let fresh1 = s;
        s = s.offset(1);
        *fresh1 = *fresh0;
        if !(*fresh1 != 0) {
            break;
        }
    }
    s = s.offset(-1);
    return s;
}
