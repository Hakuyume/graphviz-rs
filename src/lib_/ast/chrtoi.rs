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
    fn chresc(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chrtoi(mut s: *const libc::c_char) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    c = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while (n as libc::c_ulong)
        < (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        let fresh0 = s;
        s = s.offset(1);
        x = *(fresh0 as *const libc::c_uchar) as libc::c_int;
        match x {
            92 => {
                x = chresc(s.offset(-(1 as libc::c_int as isize)), &mut p);
                s = p as *const libc::c_char;
            }
            0 => return c,
            _ => {}
        }
        c = c << 8 as libc::c_int | x;
        n += 8 as libc::c_int;
    }
    return c;
}
