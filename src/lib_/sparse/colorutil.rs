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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
unsafe extern "C" fn r2i(mut r: libc::c_float) -> libc::c_int {
    return ((255 as libc::c_int as libc::c_float * r) as libc::c_double + 0.5f64) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rgb2hex(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut cstring: *mut libc::c_char,
    mut opacity: *const libc::c_char,
) {
    sprintf(
        cstring,
        b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        r2i(r),
        r2i(g),
        r2i(b),
    );
    if !opacity.is_null() && strlen(opacity) >= 2 as libc::c_int as libc::c_ulong {
        *cstring.offset(7 as libc::c_int as isize) = *opacity.offset(0 as libc::c_int as isize);
        *cstring.offset(8 as libc::c_int as isize) = *opacity.offset(1 as libc::c_int as isize);
        *cstring.offset(9 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        *cstring.offset(7 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    };
}
unsafe extern "C" fn Hue2RGB(
    mut v1: libc::c_double,
    mut v2: libc::c_double,
    mut H: libc::c_double,
) -> libc::c_double {
    if H < 0.0f64 {
        H += 1.0f64;
    }
    if H > 1.0f64 {
        H -= 1.0f64;
    }
    if 6.0f64 * H < 1.0f64 {
        return v1 + (v2 - v1) * 6.0f64 * H;
    }
    if 2.0f64 * H < 1.0f64 {
        return v2;
    }
    if 3.0f64 * H < 2.0f64 {
        return v1 + (v2 - v1) * (2.0f64 / 3.0f64 - H) * 6.0f64;
    }
    return v1;
}
#[no_mangle]
pub unsafe extern "C" fn hue2rgb(
    mut hue: libc::c_double,
    mut color: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut v1: libc::c_double = 0.;
    let mut v2: libc::c_double = 0.;
    let mut lightness: libc::c_double = 0.5f64;
    let mut saturation: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut red: libc::c_int = 0;
    let mut blue: libc::c_int = 0;
    let mut green: libc::c_int = 0;
    if lightness < 0.5f64 {
        v2 = lightness * (1.0f64 + saturation);
    } else {
        v2 = lightness + saturation - saturation * lightness;
    }
    v1 = 2.0f64 * lightness - v2;
    red = (255.0f64 * Hue2RGB(v1, v2, hue + 1.0f64 / 3.0f64) + 0.5f64) as libc::c_int;
    green = (255.0f64 * Hue2RGB(v1, v2, hue) + 0.5f64) as libc::c_int;
    blue = (255.0f64 * Hue2RGB(v1, v2, hue - 1.0f64 / 3.0f64) + 0.5f64) as libc::c_int;
    sprintf(
        color,
        b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        red,
        green,
        blue,
    );
    return color;
}
