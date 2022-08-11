#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn exit(_: libc::c_int) -> !;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub boundary: [libc::c_int; 4],
}
pub type Rect_t = Rect;
pub type size_t = libc::c_ulong;
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[no_mangle]
pub unsafe extern "C" fn InitRect(mut r: *mut Rect_t) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (2 as libc::c_int * 2 as libc::c_int) as libc::c_ulong {
        (*r).boundary[i as usize] = 0 as libc::c_int;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn NullRect() -> Rect_t {
    let mut r: Rect_t = {
        let mut init = Rect {
            boundary: [0 as libc::c_int, 0, 0, 0],
        };
        init
    };
    r.boundary[0 as libc::c_int as usize] = 1 as libc::c_int;
    r.boundary[2 as libc::c_int as usize] = -(1 as libc::c_int);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn RectArea(mut r: *mut Rect_t) -> libc::c_uint {
    if !r.is_null() {} else {
        __assert_fail(
            b"r\0" as *const u8 as *const libc::c_char,
            b"rectangle.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"unsigned int RectArea(Rect_t *)\0"))
                .as_ptr(),
        );
    }
    if (*r).boundary[0 as libc::c_int as usize]
        > (*r).boundary[2 as libc::c_int as usize]
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mut area: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        let mut dim: libc::c_uint = ((*r)
            .boundary[i.wrapping_add(2 as libc::c_int as libc::c_ulong) as usize]
            - (*r).boundary[i as usize]) as libc::c_uint;
        if dim == 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int as libc::c_uint;
        }
        if (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_div(dim) < area
        {
            agerr(
                AGERR,
                b"label: area too large for rtree\n\0" as *const u8
                    as *const libc::c_char,
            );
            graphviz_exit(1 as libc::c_int);
        }
        area = area.wrapping_mul(dim);
        i = i.wrapping_add(1);
    }
    return area;
}
#[no_mangle]
pub unsafe extern "C" fn CombineRect(mut r: *mut Rect_t, mut rr: *mut Rect_t) -> Rect_t {
    let mut new: Rect_t = Rect_t { boundary: [0; 4] };
    if !r.is_null() && !rr.is_null() {} else {
        __assert_fail(
            b"r && rr\0" as *const u8 as *const libc::c_char,
            b"rectangle.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"Rect_t CombineRect(Rect_t *, Rect_t *)\0"))
                .as_ptr(),
        );
    }
    if (*r).boundary[0 as libc::c_int as usize]
        > (*r).boundary[2 as libc::c_int as usize]
    {
        return *rr;
    }
    if (*rr).boundary[0 as libc::c_int as usize]
        > (*rr).boundary[2 as libc::c_int as usize]
    {
        return *r;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        new
            .boundary[i
            as usize] = if (*r).boundary[i as usize] < (*rr).boundary[i as usize] {
            (*r).boundary[i as usize]
        } else {
            (*rr).boundary[i as usize]
        };
        let mut j: size_t = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
        new
            .boundary[j
            as usize] = if (*r).boundary[j as usize] > (*rr).boundary[j as usize] {
            (*r).boundary[j as usize]
        } else {
            (*rr).boundary[j as usize]
        };
        i = i.wrapping_add(1);
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn Overlap(mut r: *mut Rect_t, mut s: *mut Rect_t) -> libc::c_int {
    if !r.is_null() && !s.is_null() {} else {
        __assert_fail(
            b"r && s\0" as *const u8 as *const libc::c_char,
            b"rectangle.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"int Overlap(Rect_t *, Rect_t *)\0"))
                .as_ptr(),
        );
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        let mut j: size_t = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
        if (*r).boundary[i as usize] > (*s).boundary[j as usize]
            || (*s).boundary[i as usize] > (*r).boundary[j as usize]
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Contained(
    mut r: *mut Rect_t,
    mut s: *mut Rect_t,
) -> libc::c_int {
    if !r.is_null() && !s.is_null() {} else {
        __assert_fail(
            b"r && s\0" as *const u8 as *const libc::c_char,
            b"rectangle.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"int Contained(Rect_t *, Rect_t *)\0"))
                .as_ptr(),
        );
    }
    if (*r).boundary[0 as libc::c_int as usize]
        > (*r).boundary[2 as libc::c_int as usize]
    {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if (*s).boundary[0 as libc::c_int as usize]
        > (*s).boundary[2 as libc::c_int as usize]
    {
        return 0 as libc::c_int;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        let mut j: size_t = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
        if !((*r).boundary[i as usize] >= (*s).boundary[i as usize]
            && (*r).boundary[j as usize] <= (*s).boundary[j as usize])
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
