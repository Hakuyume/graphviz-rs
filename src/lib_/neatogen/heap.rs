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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    static mut ymin: libc::c_double;
    static mut sqrt_nsites: libc::c_int;
    fn deref(_: *mut Site);
    #[link_name = "ref"]
    fn ref_0(_: *mut Site);
    static mut deltay: libc::c_double;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
pub type Point = pointf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Site {
    pub coord: Point,
    pub sitenbr: libc::c_int,
    pub refcnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Edge {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub ep: [*mut Site; 2],
    pub reg: [*mut Site; 2],
    pub edgenbr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Halfedge {
    pub ELleft: *mut Halfedge,
    pub ELright: *mut Halfedge,
    pub ELedge: *mut Edge,
    pub ELrefcnt: libc::c_int,
    pub ELpm: libc::c_char,
    pub vertex: *mut Site,
    pub ystar: libc::c_double,
    pub PQnext: *mut Halfedge,
}
static mut PQhash: *mut Halfedge = 0 as *const Halfedge as *mut Halfedge;
static mut PQhashsize: libc::c_int = 0;
static mut PQcount: libc::c_int = 0;
static mut PQmin: libc::c_int = 0;
unsafe extern "C" fn PQbucket(mut he: *mut Halfedge) -> libc::c_int {
    let mut bucket: libc::c_int = 0;
    let mut b: libc::c_double = 0.;
    b = ((*he).ystar - ymin) / deltay * PQhashsize as libc::c_double;
    if b < 0 as libc::c_int as libc::c_double {
        bucket = 0 as libc::c_int;
    } else if b >= PQhashsize as libc::c_double {
        bucket = PQhashsize - 1 as libc::c_int;
    } else {
        bucket = b as libc::c_int;
    }
    if bucket < PQmin {
        PQmin = bucket;
    }
    return bucket;
}
#[no_mangle]
pub unsafe extern "C" fn PQinsert(
    mut he: *mut Halfedge,
    mut v: *mut Site,
    mut offset: libc::c_double,
) {
    let mut last: *mut Halfedge = 0 as *mut Halfedge;
    let mut next: *mut Halfedge = 0 as *mut Halfedge;
    let ref mut fresh0 = (*he).vertex;
    *fresh0 = v;
    ref_0(v);
    (*he).ystar = (*v).coord.y + offset;
    last = &mut *PQhash
        .offset((PQbucket as unsafe extern "C" fn(*mut Halfedge) -> libc::c_int)(he) as isize)
        as *mut Halfedge;
    loop {
        next = (*last).PQnext;
        if !(!next.is_null()
            && ((*he).ystar > (*next).ystar
                || (*he).ystar == (*next).ystar && (*v).coord.x > (*(*next).vertex).coord.x))
        {
            break;
        }
        last = next;
    }
    let ref mut fresh1 = (*he).PQnext;
    *fresh1 = (*last).PQnext;
    let ref mut fresh2 = (*last).PQnext;
    *fresh2 = he;
    PQcount += 1;
}
#[no_mangle]
pub unsafe extern "C" fn PQdelete(mut he: *mut Halfedge) {
    let mut last: *mut Halfedge = 0 as *mut Halfedge;
    if !((*he).vertex).is_null() {
        last = &mut *PQhash
            .offset((PQbucket as unsafe extern "C" fn(*mut Halfedge) -> libc::c_int)(he) as isize)
            as *mut Halfedge;
        while (*last).PQnext != he {
            last = (*last).PQnext;
        }
        let ref mut fresh3 = (*last).PQnext;
        *fresh3 = (*he).PQnext;
        PQcount -= 1;
        deref((*he).vertex);
        let ref mut fresh4 = (*he).vertex;
        *fresh4 = 0 as *mut Site;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQempty() -> bool {
    return PQcount == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PQ_min() -> Point {
    let mut answer: Point = Point { x: 0., y: 0. };
    while ((*PQhash.offset(PQmin as isize)).PQnext).is_null() {
        PQmin += 1;
    }
    answer.x = (*(*(*PQhash.offset(PQmin as isize)).PQnext).vertex).coord.x;
    answer.y = (*(*PQhash.offset(PQmin as isize)).PQnext).ystar;
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn PQextractmin() -> *mut Halfedge {
    let mut curr: *mut Halfedge = 0 as *mut Halfedge;
    curr = (*PQhash.offset(PQmin as isize)).PQnext;
    let ref mut fresh5 = (*PQhash.offset(PQmin as isize)).PQnext;
    *fresh5 = (*curr).PQnext;
    PQcount -= 1;
    return curr;
}
#[no_mangle]
pub unsafe extern "C" fn PQcleanup() {
    free(PQhash as *mut libc::c_void);
    PQhash = 0 as *mut Halfedge;
}
#[no_mangle]
pub unsafe extern "C" fn PQinitialize() {
    let mut i: libc::c_int = 0;
    PQcount = 0 as libc::c_int;
    PQmin = 0 as libc::c_int;
    PQhashsize = 4 as libc::c_int * sqrt_nsites;
    if PQhash.is_null() {
        PQhash = gcalloc(
            PQhashsize as size_t,
            ::std::mem::size_of::<Halfedge>() as libc::c_ulong,
        ) as *mut Halfedge;
    }
    i = 0 as libc::c_int;
    while i < PQhashsize {
        let ref mut fresh6 = (*PQhash.offset(i as isize)).PQnext;
        *fresh6 = 0 as *mut Halfedge;
        i += 1;
    }
}
unsafe extern "C" fn PQdumphe(mut p: *mut Halfedge) {
    printf(
        b"  [%p] %p %p %d %d %d %d %f\n\0" as *const u8 as *const libc::c_char,
        p,
        (*p).ELleft,
        (*p).ELright,
        (*(*p).ELedge).edgenbr,
        (*p).ELrefcnt,
        (*p).ELpm as libc::c_int,
        if !((*p).vertex).is_null() {
            (*(*p).vertex).sitenbr
        } else {
            -(1 as libc::c_int)
        },
        (*p).ystar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PQdump() {
    let mut i: libc::c_int = 0;
    let mut p: *mut Halfedge = 0 as *mut Halfedge;
    i = 0 as libc::c_int;
    while i < PQhashsize {
        printf(b"[%d]\n\0" as *const u8 as *const libc::c_char, i);
        p = (*PQhash.offset(i as isize)).PQnext;
        while !p.is_null() {
            PQdumphe(p);
            p = (*p).PQnext;
        }
        i += 1;
    }
}
