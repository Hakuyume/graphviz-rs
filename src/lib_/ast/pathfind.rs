#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn pathpath(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type Dir_t = Dir_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dir_s {
    pub next: *mut Dir_s,
    pub dir: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub head: *mut Dir_t,
    pub tail: *mut Dir_t,
}
static mut state: C2RustUnnamed = C2RustUnnamed {
    head: 0 as *const Dir_t as *mut Dir_t,
    tail: 0 as *const Dir_t as *mut Dir_t,
};
#[no_mangle]
pub unsafe extern "C" fn pathfind(
    mut name: *const libc::c_char,
    mut lib: *const libc::c_char,
    mut type_0: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut dp: *mut Dir_t = 0 as *mut Dir_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    if access(name, 4 as libc::c_int) >= 0 as libc::c_int {
        return strncpy(buf, name, size);
    }
    if !type_0.is_null() {
        snprintf(
            buf,
            size,
            b"%s.%s\0" as *const u8 as *const libc::c_char,
            name,
            type_0,
        );
        if access(buf, 4 as libc::c_int) >= 0 as libc::c_int {
            return buf;
        }
    }
    if *name as libc::c_int != '/' as i32 {
        if !(strchr(name, '.' as i32)).is_null() {
            type_0 = 0 as *const libc::c_char;
        }
        dp = state.head;
        while !dp.is_null() {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                ((*dp).dir).as_mut_ptr(),
                name,
            );
            if !(pathpath(buf, tmp.as_mut_ptr())).is_null() {
                return buf;
            }
            if !type_0.is_null() {
                snprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    b"%s/%s.%s\0" as *const u8 as *const libc::c_char,
                    ((*dp).dir).as_mut_ptr(),
                    name,
                    type_0,
                );
                if !(pathpath(buf, tmp.as_mut_ptr())).is_null() {
                    return buf;
                }
            }
            dp = (*dp).next;
        }
        if !lib.is_null() {
            s = strrchr(lib, ':' as i32);
            if !s.is_null() {
                lib = s.offset(1 as libc::c_int as isize);
            }
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                b"lib/%s/%s\0" as *const u8 as *const libc::c_char,
                lib,
                name,
            );
            if !(pathpath(buf, tmp.as_mut_ptr())).is_null() {
                return buf;
            }
            if !type_0.is_null() {
                snprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    b"lib/%s/%s.%s\0" as *const u8 as *const libc::c_char,
                    lib,
                    name,
                    type_0,
                );
                if !(pathpath(buf, tmp.as_mut_ptr())).is_null() {
                    return buf;
                }
            }
        }
    }
    return 0 as *mut libc::c_char;
}
