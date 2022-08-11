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
    static mut sqrt_nsites: libc::c_int;
    fn free(_: *mut libc::c_void);
    fn gmalloc(_: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenode {
    pub nextfree: *mut freenode,
}
pub type Freenode = freenode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freeblock {
    pub next: *mut freeblock,
    pub nodes: *mut freenode,
}
pub type Freeblock = freeblock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freelist {
    pub head: *mut freenode,
    pub blocklist: *mut freeblock,
    pub nodesize: libc::c_int,
}
pub type Freelist = freelist;
unsafe extern "C" fn gcd(mut y: libc::c_int, mut x: libc::c_int) -> libc::c_int {
    while x != y {
        if y < x {
            x = x - y;
        } else {
            y = y - x;
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn freeinit(mut fl: *mut Freelist, mut size: libc::c_int) {
    let ref mut fresh0 = (*fl).head;
    *fresh0 = 0 as *mut freenode;
    (*fl).nodesize = (if (size as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<Freenode>() as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        size as libc::c_ulong
    } else if (::std::mem::size_of::<Freenode>() as libc::c_ulong)
        .wrapping_rem(size as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        ::std::mem::size_of::<Freenode>() as libc::c_ulong
    } else {
        (size as libc::c_ulong).wrapping_mul(
            (::std::mem::size_of::<Freenode>() as libc::c_ulong).wrapping_div(gcd(
                size,
                ::std::mem::size_of::<Freenode>() as libc::c_ulong as libc::c_int,
            )
                as libc::c_ulong),
        )
    }) as libc::c_int;
    if !((*fl).blocklist).is_null() {
        let mut bp: *mut Freeblock = 0 as *mut Freeblock;
        let mut np: *mut Freeblock = 0 as *mut Freeblock;
        bp = (*fl).blocklist;
        while !bp.is_null() {
            np = (*bp).next;
            free((*bp).nodes as *mut libc::c_void);
            free(bp as *mut libc::c_void);
            bp = np;
        }
    }
    let ref mut fresh1 = (*fl).blocklist;
    *fresh1 = 0 as *mut freeblock;
}
#[no_mangle]
pub unsafe extern "C" fn getfree(mut fl: *mut Freelist) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut t: *mut Freenode = 0 as *mut Freenode;
    let mut mem: *mut Freeblock = 0 as *mut Freeblock;
    if ((*fl).head).is_null() {
        let mut size: libc::c_int = (*fl).nodesize;
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        mem = gmalloc(::std::mem::size_of::<Freeblock>() as libc::c_ulong) as *mut Freeblock;
        let ref mut fresh2 = (*mem).nodes;
        *fresh2 = gmalloc((sqrt_nsites * size) as size_t) as *mut freenode;
        cp = (*mem).nodes as *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < sqrt_nsites {
            makefree(cp.offset((i * size) as isize) as *mut libc::c_void, fl);
            i += 1;
        }
        let ref mut fresh3 = (*mem).next;
        *fresh3 = (*fl).blocklist;
        let ref mut fresh4 = (*fl).blocklist;
        *fresh4 = mem;
    }
    t = (*fl).head;
    let ref mut fresh5 = (*fl).head;
    *fresh5 = (*t).nextfree;
    return t as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn makefree(mut curr: *mut libc::c_void, mut fl: *mut Freelist) {
    let ref mut fresh6 = (*(curr as *mut Freenode)).nextfree;
    *fresh6 = (*fl).head;
    let ref mut fresh7 = (*fl).head;
    *fresh7 = curr as *mut freenode;
}
