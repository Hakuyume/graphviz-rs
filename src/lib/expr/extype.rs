#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[no_mangle]
pub unsafe extern "C" fn extype(mut type_0: libc::c_int) -> *mut libc::c_char {
    match type_0 {
        262 => {
            return b"double\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        263 => return b"char*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        260 => {
            return b"uintmax_t\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => {}
    }
    return b"intmax_t\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
