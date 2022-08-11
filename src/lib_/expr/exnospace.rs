#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn exerror(_: *const libc::c_char, _: ...);
}
#[no_mangle]
pub unsafe extern "C" fn exnospace() -> *mut libc::c_char {
    static mut null: [libc::c_char; 1] = [0; 1];
    exerror(b"out of space\0" as *const u8 as *const libc::c_char);
    return null.as_ptr() as *mut libc::c_char;
}
