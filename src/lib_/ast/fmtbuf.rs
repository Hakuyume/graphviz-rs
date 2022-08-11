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
pub type size_t = libc::c_ulong;
static mut buf: [libc::c_char; 16384] = [0; 16384];
static mut nxt: *mut libc::c_char = unsafe { buf.as_ptr() as *mut _ };
#[no_mangle]
pub unsafe extern "C" fn fmtbuf(mut n: size_t) -> *mut libc::c_char {
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    if n > (&mut *buf.as_mut_ptr().offset(
        (::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong) as isize,
    ) as *mut libc::c_char)
        .offset_from(nxt) as libc::c_long as libc::c_ulong
    {
        nxt = buf.as_mut_ptr();
    }
    cur = nxt;
    nxt = nxt.offset(n as isize);
    return cur;
}
