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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fmtbuf(n: size_t) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _IScntrl: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[no_mangle]
pub unsafe extern "C" fn fmtquote(
    mut as_0: *const libc::c_char,
    mut qb: *const libc::c_char,
    mut qe: *const libc::c_char,
    mut n: size_t,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut s: *const libc::c_uchar = as_0 as *const libc::c_uchar;
    let mut e: *const libc::c_uchar = s.offset(n as isize);
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut escaped: libc::c_int = 0;
    let mut spaced: libc::c_int = 0;
    let mut shell: libc::c_int = 0;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = (4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong));
    if !qb.is_null() {
        len = (len as libc::c_ulong).wrapping_add(strlen(qb)) as size_t as size_t;
    }
    if !qe.is_null() {
        len = (len as libc::c_ulong).wrapping_add(strlen(qe)) as size_t as size_t;
    }
    buf = fmtbuf(len);
    b = buf;
    shell = 0 as libc::c_int;
    if !qb.is_null() {
        if *qb.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
            && *qb.offset(1 as libc::c_int as isize) as libc::c_int == '\'' as i32
            && *qb.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            shell = 1 as libc::c_int;
        }
        loop {
            let fresh0 = qb;
            qb = qb.offset(1);
            *b = *fresh0;
            if !(*b != 0) {
                break;
            }
            b = b.offset(1);
        }
    }
    f = b;
    spaced = (flags & 0x1 as libc::c_int != 0) as libc::c_int;
    escaped = spaced;
    while s < e {
        let fresh1 = s;
        s = s.offset(1);
        let mut c: libc::c_int = *fresh1 as libc::c_int;
        if flags & 0x2 as libc::c_int == 0
            && (*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
                || c == '\\' as i32)
        {
            escaped = 1 as libc::c_int;
            let fresh2 = b;
            b = b.offset(1);
            *fresh2 = '\\' as i32 as libc::c_char;
            match c {
                7 => {
                    c = 'a' as i32;
                }
                8 => {
                    c = 'b' as i32;
                }
                12 => {
                    c = 'f' as i32;
                }
                10 => {
                    c = 'n' as i32;
                }
                13 => {
                    c = 'r' as i32;
                }
                9 => {
                    c = 't' as i32;
                }
                11 => {
                    c = 'v' as i32;
                }
                27 => {
                    c = 'E' as i32;
                }
                92 => {}
                _ => {
                    if flags & 0x8 as libc::c_int == 0 || c & 0o200 as libc::c_int == 0 {
                        let fresh3 = b;
                        b = b.offset(1);
                        *fresh3 = ('0' as i32 + (c >> 6 as libc::c_int & 0o7 as libc::c_int))
                            as libc::c_char;
                        let fresh4 = b;
                        b = b.offset(1);
                        *fresh4 = ('0' as i32 + (c >> 3 as libc::c_int & 0o7 as libc::c_int))
                            as libc::c_char;
                        c = '0' as i32 + (c & 0o7 as libc::c_int);
                    } else {
                        b = b.offset(-1);
                    }
                }
            }
        } else if c == '\\' as i32 {
            escaped = 1 as libc::c_int;
            let fresh5 = b;
            b = b.offset(1);
            *fresh5 = c as libc::c_char;
            if *s != 0 {
                let fresh6 = s;
                s = s.offset(1);
                c = *fresh6 as libc::c_int;
            }
        } else if !qe.is_null() && !(strchr(qe, c)).is_null()
            || flags & 0x4 as libc::c_int != 0 && shell == 0 && (c == '$' as i32 || c == '`' as i32)
        {
            escaped = 1 as libc::c_int;
            let fresh7 = b;
            b = b.offset(1);
            *fresh7 = '\\' as i32 as libc::c_char;
        } else if spaced == 0
            && escaped == 0
            && (*(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || (flags & 0x4 as libc::c_int != 0 || shell != 0)
                    && (!(strchr(b"\";~&|()<>[]*?\0" as *const u8 as *const libc::c_char, c))
                        .is_null()
                        || c == '#' as i32
                            && (b == f
                                || *(*__ctype_b_loc())
                                    .offset(*b.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                        as isize)
                                    as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0)))
        {
            spaced = 1 as libc::c_int;
        }
        let fresh8 = b;
        b = b.offset(1);
        *fresh8 = c as libc::c_char;
    }
    if !qb.is_null() {
        if escaped == 0 {
            buf = buf.offset((shell + (spaced == 0) as libc::c_int) as isize);
        }
        if !qe.is_null() && (escaped != 0 || spaced != 0) {
            loop {
                let fresh9 = qe;
                qe = qe.offset(1);
                *b = *fresh9;
                if !(*b != 0) {
                    break;
                }
                b = b.offset(1);
            }
        }
    }
    *b = 0 as libc::c_int as libc::c_char;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn fmtesq(
    mut as_0: *const libc::c_char,
    mut qs: *const libc::c_char,
) -> *mut libc::c_char {
    return fmtquote(
        as_0,
        0 as *const libc::c_char,
        qs,
        strlen(as_0),
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fmtesc(mut as_0: *const libc::c_char) -> *mut libc::c_char {
    return fmtquote(
        as_0,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        strlen(as_0),
        0 as libc::c_int,
    );
}
