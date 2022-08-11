#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[no_mangle]
pub unsafe extern "C" fn chresc(
    mut s: *const libc::c_char,
    mut p: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    let fresh0 = s;
    s = s.offset(1);
    c = *fresh0 as libc::c_int;
    match c {
        0 => {
            s = s.offset(-1);
        }
        92 => {
            let fresh1 = s;
            s = s.offset(1);
            c = *fresh1 as libc::c_int;
            match c {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    c -= '0' as i32;
                    q = s.offset(2 as libc::c_int as isize);
                    while s < q {
                        match *s as libc::c_int {
                            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                                let fresh2 = s;
                                s = s.offset(1);
                                c = (c << 3 as libc::c_int) + *fresh2 as libc::c_int
                                    - '0' as i32;
                            }
                            _ => {
                                q = s;
                            }
                        }
                    }
                }
                97 => {
                    c = 0o7 as libc::c_int;
                }
                98 => {
                    c = '\u{8}' as i32;
                }
                102 => {
                    c = '\u{c}' as i32;
                }
                110 => {
                    c = '\n' as i32;
                }
                114 => {
                    c = '\r' as i32;
                }
                115 => {
                    c = ' ' as i32;
                }
                116 => {
                    c = '\t' as i32;
                }
                118 => {
                    c = 0o13 as libc::c_int;
                }
                120 => {
                    c = 0 as libc::c_int;
                    q = s;
                    while !q.is_null() {
                        match *s as libc::c_int {
                            97 | 98 | 99 | 100 | 101 | 102 => {
                                let fresh3 = s;
                                s = s.offset(1);
                                c = (c << 4 as libc::c_int) + *fresh3 as libc::c_int
                                    - 'a' as i32 + 10 as libc::c_int;
                            }
                            65 | 66 | 67 | 68 | 69 | 70 => {
                                let fresh4 = s;
                                s = s.offset(1);
                                c = (c << 4 as libc::c_int) + *fresh4 as libc::c_int
                                    - 'A' as i32 + 10 as libc::c_int;
                            }
                            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                                let fresh5 = s;
                                s = s.offset(1);
                                c = (c << 4 as libc::c_int) + *fresh5 as libc::c_int
                                    - '0' as i32;
                            }
                            _ => {
                                q = 0 as *const libc::c_char;
                            }
                        }
                    }
                }
                69 => {
                    c = 0o33 as libc::c_int;
                }
                0 => {
                    s = s.offset(-1);
                }
                _ => {}
            }
        }
        _ => {}
    }
    if !p.is_null() {
        *p = s as *mut libc::c_char;
    }
    return c;
}
