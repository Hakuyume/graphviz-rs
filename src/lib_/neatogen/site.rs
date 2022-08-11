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
#![feature(extern_types, register_tool)]
extern "C" {
    pub type freeblock;
    pub type freenode;
    fn getfree(_: *mut Freelist) -> *mut libc::c_void;
    fn freeinit(_: *mut Freelist, _: libc::c_int);
    fn makefree(_: *mut libc::c_void, _: *mut Freelist);
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freelist {
    pub head: *mut freenode,
    pub blocklist: *mut freeblock,
    pub nodesize: libc::c_int,
}
pub type Freelist = freelist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Point {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Site {
    pub coord: Point,
    pub sitenbr: libc::c_int,
    pub refcnt: libc::c_int,
}
#[no_mangle]
pub static mut siteidx: libc::c_int = 0;
#[no_mangle]
pub static mut bottomsite: *mut Site = 0 as *const Site as *mut Site;
static mut sfl: Freelist = Freelist {
    head: 0 as *const freenode as *mut freenode,
    blocklist: 0 as *const freeblock as *mut freeblock,
    nodesize: 0,
};
static mut nvertices: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn siteinit() {
    freeinit(
        &mut sfl,
        ::std::mem::size_of::<Site>() as libc::c_ulong as libc::c_int,
    );
    nvertices = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getsite() -> *mut Site {
    return getfree(&mut sfl) as *mut Site;
}
#[no_mangle]
pub unsafe extern "C" fn dist(mut s: *mut Site, mut t: *mut Site) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    dx = (*s).coord.x - (*t).coord.x;
    dy = (*s).coord.y - (*t).coord.y;
    ans = hypot(dx, dy);
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn makevertex(mut v: *mut Site) {
    (*v).sitenbr = nvertices;
    nvertices += 1;
}
#[no_mangle]
pub unsafe extern "C" fn deref(mut v: *mut Site) {
    let ref mut fresh0 = (*v).refcnt;
    *fresh0 -= 1;
    if (*v).refcnt == 0 as libc::c_int {
        makefree(v as *mut libc::c_void, &mut sfl);
    }
}
#[export_name = "ref"]
pub unsafe extern "C" fn ref_0(mut v: *mut Site) {
    let ref mut fresh1 = (*v).refcnt;
    *fresh1 += 1;
}
