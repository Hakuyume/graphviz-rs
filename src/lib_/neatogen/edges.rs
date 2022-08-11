#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type freeblock;
    pub type freenode;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn getfree(_: *mut Freelist) -> *mut libc::c_void;
    fn freeinit(_: *mut Freelist, _: libc::c_int);
    fn makefree(_: *mut libc::c_void, _: *mut Freelist);
    fn addVertex(_: *mut Site, _: libc::c_double, _: libc::c_double);
    fn deref(_: *mut Site);
    #[link_name = "ref"]
    fn ref_0(_: *mut Site);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freelist {
    pub head: *mut freenode,
    pub blocklist: *mut freeblock,
    pub nodesize: libc::c_int,
}
pub type Freelist = freelist;
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
#[no_mangle]
pub static mut pxmin: libc::c_double = 0.;
#[no_mangle]
pub static mut pxmax: libc::c_double = 0.;
#[no_mangle]
pub static mut pymin: libc::c_double = 0.;
#[no_mangle]
pub static mut pymax: libc::c_double = 0.;
static mut nedges: libc::c_int = 0;
static mut efl: Freelist = Freelist {
    head: 0 as *const freenode as *mut freenode,
    blocklist: 0 as *const freeblock as *mut freeblock,
    nodesize: 0,
};
#[no_mangle]
pub unsafe extern "C" fn edgeinit() {
    freeinit(&mut efl, ::std::mem::size_of::<Edge>() as libc::c_ulong as libc::c_int);
    nedges = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gvbisect(mut s1: *mut Site, mut s2: *mut Site) -> *mut Edge {
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut adx: libc::c_double = 0.;
    let mut ady: libc::c_double = 0.;
    let mut newedge: *mut Edge = 0 as *mut Edge;
    newedge = getfree(&mut efl) as *mut Edge;
    let ref mut fresh0 = (*newedge).reg[0 as libc::c_int as usize];
    *fresh0 = s1;
    let ref mut fresh1 = (*newedge).reg[1 as libc::c_int as usize];
    *fresh1 = s2;
    ref_0(s1);
    ref_0(s2);
    let ref mut fresh2 = (*newedge).ep[0 as libc::c_int as usize];
    *fresh2 = 0 as *mut Site;
    let ref mut fresh3 = (*newedge).ep[1 as libc::c_int as usize];
    *fresh3 = 0 as *mut Site;
    dx = (*s2).coord.x - (*s1).coord.x;
    dy = (*s2).coord.y - (*s1).coord.y;
    adx = fabs(dx);
    ady = fabs(dy);
    (*newedge)
        .c = (*s1).coord.x * dx + (*s1).coord.y * dy + (dx * dx + dy * dy) * 0.5f64;
    if adx > ady {
        (*newedge).a = 1.0f64;
        (*newedge).b = dy / dx;
        (*newedge).c /= dx;
    } else {
        (*newedge).b = 1.0f64;
        (*newedge).a = dx / dy;
        (*newedge).c /= dy;
    }
    (*newedge).edgenbr = nedges;
    nedges += 1 as libc::c_int;
    return newedge;
}
unsafe extern "C" fn doSeg(
    mut e: *mut Edge,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut x2: libc::c_double,
    mut y2: libc::c_double,
) {
    addVertex((*e).reg[0 as libc::c_int as usize], x1, y1);
    addVertex((*e).reg[0 as libc::c_int as usize], x2, y2);
    addVertex((*e).reg[1 as libc::c_int as usize], x1, y1);
    addVertex((*e).reg[1 as libc::c_int as usize], x2, y2);
}
#[no_mangle]
pub unsafe extern "C" fn clip_line(mut e: *mut Edge) {
    let mut s1: *mut Site = 0 as *mut Site;
    let mut s2: *mut Site = 0 as *mut Site;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    if (*e).a == 1.0f64 && (*e).b >= 0.0f64 {
        s1 = (*e).ep[1 as libc::c_int as usize];
        s2 = (*e).ep[0 as libc::c_int as usize];
    } else {
        s1 = (*e).ep[0 as libc::c_int as usize];
        s2 = (*e).ep[1 as libc::c_int as usize];
    }
    if (*e).a == 1.0f64 {
        if !s1.is_null() {
            y1 = (*s1).coord.y;
            if y1 > pymax {
                return
            } else {
                if y1 >= pymin {
                    x1 = (*s1).coord.x;
                } else {
                    y1 = pymin;
                    x1 = (*e).c - (*e).b * y1;
                }
            }
        } else {
            y1 = pymin;
            x1 = (*e).c - (*e).b * y1;
        }
        if !s2.is_null() {
            y2 = (*s2).coord.y;
            if y2 < pymin {
                return
            } else {
                if y2 <= pymax {
                    x2 = (*s2).coord.x;
                } else {
                    y2 = pymax;
                    x2 = (*e).c - (*e).b * y2;
                }
            }
        } else {
            y2 = pymax;
            x2 = (*e).c - (*e).b * y2;
        }
        if x1 > pxmax && x2 > pxmax || x1 < pxmin && x2 < pxmin {
            return;
        }
        if x1 > pxmax {
            x1 = pxmax;
            y1 = ((*e).c - x1) / (*e).b;
        }
        if x1 < pxmin {
            x1 = pxmin;
            y1 = ((*e).c - x1) / (*e).b;
        }
        if x2 > pxmax {
            x2 = pxmax;
            y2 = ((*e).c - x2) / (*e).b;
        }
        if x2 < pxmin {
            x2 = pxmin;
            y2 = ((*e).c - x2) / (*e).b;
        }
    } else {
        if !s1.is_null() {
            x1 = (*s1).coord.x;
            if x1 > pxmax {
                return
            } else {
                if x1 >= pxmin {
                    y1 = (*s1).coord.y;
                } else {
                    x1 = pxmin;
                    y1 = (*e).c - (*e).a * x1;
                }
            }
        } else {
            x1 = pxmin;
            y1 = (*e).c - (*e).a * x1;
        }
        if !s2.is_null() {
            x2 = (*s2).coord.x;
            if x2 < pxmin {
                return
            } else {
                if x2 <= pxmax {
                    y2 = (*s2).coord.y;
                } else {
                    x2 = pxmax;
                    y2 = (*e).c - (*e).a * x2;
                }
            }
        } else {
            x2 = pxmax;
            y2 = (*e).c - (*e).a * x2;
        }
        if y1 > pymax && y2 > pymax || y1 < pymin && y2 < pymin {
            return;
        }
        if y1 > pymax {
            y1 = pymax;
            x1 = ((*e).c - y1) / (*e).a;
        }
        if y1 < pymin {
            y1 = pymin;
            x1 = ((*e).c - y1) / (*e).a;
        }
        if y2 > pymax {
            y2 = pymax;
            x2 = ((*e).c - y2) / (*e).a;
        }
        if y2 < pymin {
            y2 = pymin;
            x2 = ((*e).c - y2) / (*e).a;
        }
    }
    doSeg(e, x1, y1, x2, y2);
}
#[no_mangle]
pub unsafe extern "C" fn endpoint(
    mut e: *mut Edge,
    mut lr: libc::c_int,
    mut s: *mut Site,
) {
    let ref mut fresh4 = (*e).ep[lr as usize];
    *fresh4 = s;
    ref_0(s);
    if ((*e).ep[(1 as libc::c_int - lr) as usize]).is_null() {
        return;
    }
    clip_line(e);
    deref((*e).reg[0 as libc::c_int as usize]);
    deref((*e).reg[1 as libc::c_int as usize]);
    makefree(e as *mut libc::c_void, &mut efl);
}
