#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type freeblock;
    pub type freenode;
    fn getfree(_: *mut Freelist) -> *mut libc::c_void;
    fn freeinit(_: *mut Freelist, _: libc::c_int);
    fn makefree(_: *mut libc::c_void, _: *mut Freelist);
    static mut deltax: libc::c_double;
    static mut sqrt_nsites: libc::c_int;
    static mut bottomsite: *mut Site;
    fn getsite() -> *mut Site;
    static mut xmin: libc::c_double;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
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
#[no_mangle]
pub static mut ELleftend: *mut Halfedge = 0 as *const Halfedge as *mut Halfedge;
#[no_mangle]
pub static mut ELrightend: *mut Halfedge = 0 as *const Halfedge as *mut Halfedge;
static mut hfl: Freelist = Freelist {
    head: 0 as *const freenode as *mut freenode,
    blocklist: 0 as *const freeblock as *mut freeblock,
    nodesize: 0,
};
static mut ELhashsize: libc::c_int = 0;
static mut ELhash: *mut *mut Halfedge = 0 as *const *mut Halfedge as *mut *mut Halfedge;
static mut ntry: libc::c_int = 0;
static mut totalsearch: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ELcleanup() {
    freeinit(
        &mut hfl,
        ::std::mem::size_of::<Halfedge>() as libc::c_ulong as libc::c_int,
    );
    free(ELhash as *mut libc::c_void);
    ELhash = 0 as *mut *mut Halfedge;
}
#[no_mangle]
pub unsafe extern "C" fn ELinitialize() {
    let mut i: libc::c_int = 0;
    freeinit(
        &mut hfl,
        ::std::mem::size_of::<Halfedge>() as libc::c_ulong as libc::c_int,
    );
    ELhashsize = 2 as libc::c_int * sqrt_nsites;
    if ELhash.is_null() {
        ELhash = gcalloc(
            ELhashsize as size_t,
            ::std::mem::size_of::<*mut Halfedge>() as libc::c_ulong,
        ) as *mut *mut Halfedge;
    }
    i = 0 as libc::c_int;
    while i < ELhashsize {
        let ref mut fresh0 = *ELhash.offset(i as isize);
        *fresh0 = 0 as *mut Halfedge;
        i += 1;
    }
    ELleftend = HEcreate(0 as *mut Edge, 0 as libc::c_int as libc::c_char);
    ELrightend = HEcreate(0 as *mut Edge, 0 as libc::c_int as libc::c_char);
    let ref mut fresh1 = (*ELleftend).ELleft;
    *fresh1 = 0 as *mut Halfedge;
    let ref mut fresh2 = (*ELleftend).ELright;
    *fresh2 = ELrightend;
    let ref mut fresh3 = (*ELrightend).ELleft;
    *fresh3 = ELleftend;
    let ref mut fresh4 = (*ELrightend).ELright;
    *fresh4 = 0 as *mut Halfedge;
    let ref mut fresh5 = *ELhash.offset(0 as libc::c_int as isize);
    *fresh5 = ELleftend;
    let ref mut fresh6 = *ELhash.offset((ELhashsize - 1 as libc::c_int) as isize);
    *fresh6 = ELrightend;
}
#[no_mangle]
pub unsafe extern "C" fn hintersect(
    mut el1: *mut Halfedge,
    mut el2: *mut Halfedge,
) -> *mut Site {
    let mut e1: *mut Edge = 0 as *mut Edge;
    let mut e2: *mut Edge = 0 as *mut Edge;
    let mut e: *mut Edge = 0 as *mut Edge;
    let mut el: *mut Halfedge = 0 as *mut Halfedge;
    let mut d: libc::c_double = 0.;
    let mut xint: libc::c_double = 0.;
    let mut yint: libc::c_double = 0.;
    let mut right_of_site: bool = false;
    let mut v: *mut Site = 0 as *mut Site;
    e1 = (*el1).ELedge;
    e2 = (*el2).ELedge;
    if e1.is_null() || e2.is_null() {
        return 0 as *mut Site;
    }
    if (*e1).reg[1 as libc::c_int as usize] == (*e2).reg[1 as libc::c_int as usize] {
        return 0 as *mut Site;
    }
    d = (*e1).a * (*e2).b - (*e1).b * (*e2).a;
    if -1.0e-10f64 < d && d < 1.0e-10f64 {
        return 0 as *mut Site;
    }
    xint = ((*e1).c * (*e2).b - (*e2).c * (*e1).b) / d;
    yint = ((*e2).c * (*e1).a - (*e1).c * (*e2).a) / d;
    if (*(*e1).reg[1 as libc::c_int as usize]).coord.y
        < (*(*e2).reg[1 as libc::c_int as usize]).coord.y
        || (*(*e1).reg[1 as libc::c_int as usize]).coord.y
            == (*(*e2).reg[1 as libc::c_int as usize]).coord.y
            && (*(*e1).reg[1 as libc::c_int as usize]).coord.x
                < (*(*e2).reg[1 as libc::c_int as usize]).coord.x
    {
        el = el1;
        e = e1;
    } else {
        el = el2;
        e = e2;
    }
    right_of_site = xint >= (*(*e).reg[1 as libc::c_int as usize]).coord.x;
    if right_of_site as libc::c_int != 0 && (*el).ELpm as libc::c_int == 0 as libc::c_int
        || !right_of_site && (*el).ELpm as libc::c_int == 1 as libc::c_int
    {
        return 0 as *mut Site;
    }
    v = getsite();
    (*v).refcnt = 0 as libc::c_int;
    (*v).coord.x = xint;
    (*v).coord.y = yint;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn right_of(
    mut el: *mut Halfedge,
    mut p: *mut Point,
) -> libc::c_int {
    let mut e: *mut Edge = 0 as *mut Edge;
    let mut topsite: *mut Site = 0 as *mut Site;
    let mut above: libc::c_int = 0;
    let mut fast: libc::c_int = 0;
    let mut dxp: libc::c_double = 0.;
    let mut dyp: libc::c_double = 0.;
    let mut dxs: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut t3: libc::c_double = 0.;
    let mut yl: libc::c_double = 0.;
    e = (*el).ELedge;
    topsite = (*e).reg[1 as libc::c_int as usize];
    let mut right_of_site: bool = (*p).x > (*topsite).coord.x;
    if right_of_site as libc::c_int != 0 && (*el).ELpm as libc::c_int == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if !right_of_site && (*el).ELpm as libc::c_int == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*e).a == 1.0f64 {
        dyp = (*p).y - (*topsite).coord.y;
        dxp = (*p).x - (*topsite).coord.x;
        fast = 0 as libc::c_int;
        if !right_of_site && (*e).b < 0.0f64
            || right_of_site as libc::c_int != 0 && (*e).b >= 0.0f64
        {
            above = (dyp >= (*e).b * dxp) as libc::c_int;
            fast = above;
        } else {
            above = ((*p).x + (*p).y * (*e).b > (*e).c) as libc::c_int;
            if (*e).b < 0.0f64 {
                above = (above == 0) as libc::c_int;
            }
            if above == 0 {
                fast = 1 as libc::c_int;
            }
        }
        if fast == 0 {
            dxs = (*topsite).coord.x - (*(*e).reg[0 as libc::c_int as usize]).coord.x;
            above = ((*e).b * (dxp * dxp - dyp * dyp)
                < dxs * dyp * (1.0f64 + 2.0f64 * dxp / dxs + (*e).b * (*e).b))
                as libc::c_int;
            if (*e).b < 0.0f64 {
                above = (above == 0) as libc::c_int;
            }
        }
    } else {
        yl = (*e).c - (*e).a * (*p).x;
        t1 = (*p).y - yl;
        t2 = (*p).x - (*topsite).coord.x;
        t3 = yl - (*topsite).coord.y;
        above = (t1 * t1 > t2 * t2 + t3 * t3) as libc::c_int;
    }
    return if (*el).ELpm as libc::c_int == 0 as libc::c_int {
        above
    } else {
        (above == 0) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn HEcreate(
    mut e: *mut Edge,
    mut pm: libc::c_char,
) -> *mut Halfedge {
    let mut answer: *mut Halfedge = 0 as *mut Halfedge;
    answer = getfree(&mut hfl) as *mut Halfedge;
    let ref mut fresh7 = (*answer).ELedge;
    *fresh7 = e;
    (*answer).ELpm = pm;
    let ref mut fresh8 = (*answer).PQnext;
    *fresh8 = 0 as *mut Halfedge;
    let ref mut fresh9 = (*answer).vertex;
    *fresh9 = 0 as *mut Site;
    (*answer).ELrefcnt = 0 as libc::c_int;
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn ELinsert(mut lb: *mut Halfedge, mut new: *mut Halfedge) {
    let ref mut fresh10 = (*new).ELleft;
    *fresh10 = lb;
    let ref mut fresh11 = (*new).ELright;
    *fresh11 = (*lb).ELright;
    let ref mut fresh12 = (*(*lb).ELright).ELleft;
    *fresh12 = new;
    let ref mut fresh13 = (*lb).ELright;
    *fresh13 = new;
}
unsafe extern "C" fn ELgethash(mut b: libc::c_int) -> *mut Halfedge {
    let mut he: *mut Halfedge = 0 as *mut Halfedge;
    if b < 0 as libc::c_int || b >= ELhashsize {
        return 0 as *mut Halfedge;
    }
    he = *ELhash.offset(b as isize);
    if he.is_null() || (*he).ELedge != -(2 as libc::c_int) as *mut Edge {
        return he;
    }
    let ref mut fresh14 = *ELhash.offset(b as isize);
    *fresh14 = 0 as *mut Halfedge;
    let ref mut fresh15 = (*he).ELrefcnt;
    *fresh15 -= 1;
    if *fresh15 == 0 as libc::c_int {
        makefree(he as *mut libc::c_void, &mut hfl);
    }
    return 0 as *mut Halfedge;
}
#[no_mangle]
pub unsafe extern "C" fn ELleftbnd(mut p: *mut Point) -> *mut Halfedge {
    let mut i: libc::c_int = 0;
    let mut bucket: libc::c_int = 0;
    let mut he: *mut Halfedge = 0 as *mut Halfedge;
    bucket = (((*p).x - xmin) / deltax * ELhashsize as libc::c_double) as libc::c_int;
    if bucket < 0 as libc::c_int {
        bucket = 0 as libc::c_int;
    }
    if bucket >= ELhashsize {
        bucket = ELhashsize - 1 as libc::c_int;
    }
    he = ELgethash(bucket);
    if he.is_null() {
        i = 1 as libc::c_int;
        loop {
            he = ELgethash(bucket - i);
            if !he.is_null() {
                break;
            }
            he = ELgethash(bucket + i);
            if !he.is_null() {
                break;
            }
            i += 1;
        }
        totalsearch += i;
    }
    ntry += 1;
    if he == ELleftend || he != ELrightend && right_of(he, p) != 0 {
        loop {
            he = (*he).ELright;
            if !(he != ELrightend && right_of(he, p) != 0) {
                break;
            }
        }
        he = (*he).ELleft;
    } else {
        loop {
            he = (*he).ELleft;
            if !(he != ELleftend && right_of(he, p) == 0) {
                break;
            }
        }
    }
    if bucket > 0 as libc::c_int && bucket < ELhashsize - 1 as libc::c_int {
        if !(*ELhash.offset(bucket as isize)).is_null() {
            let ref mut fresh16 = (**ELhash.offset(bucket as isize)).ELrefcnt;
            *fresh16 -= 1;
        }
        let ref mut fresh17 = *ELhash.offset(bucket as isize);
        *fresh17 = he;
        let ref mut fresh18 = (**ELhash.offset(bucket as isize)).ELrefcnt;
        *fresh18 += 1;
    }
    return he;
}
#[no_mangle]
pub unsafe extern "C" fn ELdelete(mut he: *mut Halfedge) {
    let ref mut fresh19 = (*(*he).ELleft).ELright;
    *fresh19 = (*he).ELright;
    let ref mut fresh20 = (*(*he).ELright).ELleft;
    *fresh20 = (*he).ELleft;
    let ref mut fresh21 = (*he).ELedge;
    *fresh21 = -(2 as libc::c_int) as *mut Edge;
}
#[no_mangle]
pub unsafe extern "C" fn ELright(mut he: *mut Halfedge) -> *mut Halfedge {
    return (*he).ELright;
}
#[no_mangle]
pub unsafe extern "C" fn ELleft(mut he: *mut Halfedge) -> *mut Halfedge {
    return (*he).ELleft;
}
#[no_mangle]
pub unsafe extern "C" fn leftreg(mut he: *mut Halfedge) -> *mut Site {
    if ((*he).ELedge).is_null() {
        return bottomsite;
    }
    return if (*he).ELpm as libc::c_int == 0 as libc::c_int {
        (*(*he).ELedge).reg[0 as libc::c_int as usize]
    } else {
        (*(*he).ELedge).reg[1 as libc::c_int as usize]
    };
}
#[no_mangle]
pub unsafe extern "C" fn rightreg(mut he: *mut Halfedge) -> *mut Site {
    if ((*he).ELedge).is_null() {
        return bottomsite;
    }
    return if (*he).ELpm as libc::c_int == 0 as libc::c_int {
        (*(*he).ELedge).reg[1 as libc::c_int as usize]
    } else {
        (*(*he).ELedge).reg[0 as libc::c_int as usize]
    };
}
