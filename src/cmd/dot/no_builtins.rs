#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_symlist_t {
    pub name: *const libc::c_char,
    pub address: *mut libc::c_void,
}
#[no_mangle]
pub static mut lt_preloaded_symbols: [lt_symlist_t; 1] = [
    {
        let mut init = lt_symlist_t {
            name: 0 as *const libc::c_char,
            address: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    },
];
