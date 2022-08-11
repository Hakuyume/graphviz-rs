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
pub unsafe extern "C" fn dtstrhash(
    mut h: libc::c_uint,
    mut args: *mut libc::c_void,
    mut n: libc::c_int,
) -> libc::c_uint {
    let mut s: *mut libc::c_uchar = args as *mut libc::c_uchar;
    if n <= 0 as libc::c_int {
        while *s as libc::c_int != 0 as libc::c_int {
            h = h
                .wrapping_add(
                    ((*s.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
                        as libc::c_uint,
                )
                .wrapping_add(*s.offset(1 as libc::c_int as isize) as libc::c_uint)
                .wrapping_mul(17109811 as libc::c_int as libc::c_uint);
            s = s.offset(
                (if *s.offset(1 as libc::c_int as isize) as libc::c_int != 0 {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as isize,
            );
        }
        n = s.offset_from(args as *mut libc::c_uchar) as libc::c_long as libc::c_int;
    } else {
        let mut ends: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        ends = s.offset(n as isize).offset(-(1 as libc::c_int as isize));
        while s < ends {
            h = h
                .wrapping_add(
                    ((*s.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
                        as libc::c_uint,
                )
                .wrapping_add(*s.offset(1 as libc::c_int as isize) as libc::c_uint)
                .wrapping_mul(17109811 as libc::c_int as libc::c_uint);
            s = s.offset(2 as libc::c_int as isize);
        }
        if s <= ends {
            h = h
                .wrapping_add(
                    ((*s.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
                        as libc::c_uint,
                )
                .wrapping_mul(17109811 as libc::c_int as libc::c_uint);
        }
    }
    return h
        .wrapping_add(n as libc::c_uint)
        .wrapping_mul(17109811 as libc::c_int as libc::c_uint);
}
