#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn perror(__s: *const libc::c_char);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type gl_data = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct generic_list_s {
    pub used: uint64_t,
    pub size: uint64_t,
    pub data: *mut gl_data,
}
pub type generic_list_t = generic_list_s;
#[no_mangle]
pub unsafe extern "C" fn new_generic_list(mut size: uint64_t) -> *mut generic_list_t {
    let mut list: *mut generic_list_t = 0 as *mut generic_list_t;
    list = malloc(::std::mem::size_of::<generic_list_t>() as libc::c_ulong)
        as *mut generic_list_t;
    if list.is_null() {
        perror(
            b"[new_generic_list()] Error allocating memory:\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut generic_list_t;
    }
    if size != 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh0 = (*list).data;
        *fresh0 = malloc(
            size.wrapping_mul(::std::mem::size_of::<gl_data>() as libc::c_ulong),
        ) as *mut gl_data;
        if ((*list).data).is_null() {
            perror(
                b"[new_generic_list()] Error allocating memory:\0" as *const u8
                    as *const libc::c_char,
            );
            free(list as *mut libc::c_void);
            return 0 as *mut generic_list_t;
        }
    } else {
        let ref mut fresh1 = (*list).data;
        *fresh1 = 0 as *mut gl_data;
    }
    (*list).size = size;
    (*list).used = 0 as libc::c_int as uint64_t;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn free_generic_list(mut list: *mut generic_list_t) {
    if (*list).size > 0 as libc::c_int as libc::c_ulong {
        free((*list).data as *mut libc::c_void);
    }
    free(list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn add_to_generic_list(
    mut list: *mut generic_list_t,
    mut element: gl_data,
) -> *mut generic_list_t {
    let mut new_size: uint64_t = 0;
    let mut new_data: *mut gl_data = 0 as *mut gl_data;
    if (*list).size == (*list).used {
        if (*list).size == 0 as libc::c_int as libc::c_ulong {
            new_size = 100 as libc::c_int as uint64_t;
        } else {
            new_size = ((*list).size).wrapping_mul(2 as libc::c_int as libc::c_ulong);
        }
        new_data = realloc(
            (*list).data as *mut libc::c_void,
            new_size.wrapping_mul(::std::mem::size_of::<gl_data>() as libc::c_ulong),
        ) as *mut gl_data;
        if new_data.is_null() {
            perror(
                b"[add_to_generic_list()] Error allocating memory:\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut generic_list_t;
        }
        let ref mut fresh2 = (*list).data;
        *fresh2 = new_data;
        (*list).size = new_size;
    }
    let ref mut fresh3 = (*list).used;
    let fresh4 = *fresh3;
    *fresh3 = (*fresh3).wrapping_add(1);
    let ref mut fresh5 = *((*list).data).offset(fresh4 as isize);
    *fresh5 = element;
    return list;
}
