#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[no_mangle]
pub unsafe extern "C" fn pathcanon(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dots: libc::c_int = 0;
    dots = 0 as libc::c_int;
    if *path as libc::c_int == '/' as i32
        && *path.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        loop {
            path = path.offset(1);
            if !(*path as libc::c_int == '/' as i32
                && *path.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
            {
                break;
            }
        }
    }
    t = path;
    s = t;
    r = s;
    loop {
        let mut current_block_31: u64;
        let fresh0 = s;
        s = s.offset(1);
        let fresh1 = t;
        t = t.offset(1);
        *fresh1 = *fresh0;
        match *fresh1 as libc::c_int {
            46 => {
                dots += 1;
                current_block_31 = 980989089337379490;
            }
            0 => {
                s = s.offset(-1);
                current_block_31 = 14308170633765484085;
            }
            47 => {
                current_block_31 = 14308170633765484085;
            }
            _ => {
                dots = 4 as libc::c_int;
                current_block_31 = 980989089337379490;
            }
        }
        match current_block_31 {
            14308170633765484085 => {
                while *s as libc::c_int == '/' as i32 {
                    s = s.offset(1);
                }
                match dots {
                    1 => {
                        t = t.offset(-(2 as libc::c_int as isize));
                    }
                    2 => {
                        if t.offset(-(5 as libc::c_int as isize)) < r {
                            if t.offset(-(4 as libc::c_int as isize)) == r {
                                t = r.offset(1 as libc::c_int as isize);
                            } else {
                                r = t;
                            }
                        } else {
                            t = t.offset(-(5 as libc::c_int as isize));
                            while t > r
                                && *t.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                    != '/' as i32
                            {
                                t = t.offset(-1);
                            }
                        }
                    }
                    3 => {
                        r = t;
                    }
                    _ => {}
                }
                if *s == 0 {
                    if t > path && *t.offset(-(1 as libc::c_int as isize)) == 0 {
                        t = t.offset(-1);
                    }
                    if t == path {
                        let fresh2 = t;
                        t = t.offset(1);
                        *fresh2 = '.' as i32 as libc::c_char;
                    } else if (s <= path
                            || *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                != '/' as i32) && t > path.offset(1 as libc::c_int as isize)
                            && *t.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                == '/' as i32
                        {
                        t = t.offset(-1);
                    }
                    *t = 0 as libc::c_int as libc::c_char;
                    return t;
                }
                dots = 0 as libc::c_int;
            }
            _ => {}
        }
    };
}
