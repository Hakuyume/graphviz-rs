#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut bottomsite: *mut Site;
    fn siteinit();
    fn dist(_: *mut Site, _: *mut Site) -> libc::c_double;
    fn deref(_: *mut Site);
    fn gvbisect(_: *mut Site, _: *mut Site) -> *mut Edge;
    fn edgeinit();
    fn endpoint(_: *mut Edge, _: libc::c_int, _: *mut Site);
    fn clip_line(e: *mut Edge);
    fn makevertex(_: *mut Site);
    static mut ELleftend: *mut Halfedge;
    static mut ELrightend: *mut Halfedge;
    fn ELinitialize();
    fn hintersect(_: *mut Halfedge, _: *mut Halfedge) -> *mut Site;
    fn HEcreate(_: *mut Edge, _: libc::c_char) -> *mut Halfedge;
    fn ELinsert(_: *mut Halfedge, _: *mut Halfedge);
    fn ELleftbnd(_: *mut Point) -> *mut Halfedge;
    fn ELdelete(_: *mut Halfedge);
    fn ELright(_: *mut Halfedge) -> *mut Halfedge;
    fn ELleft(_: *mut Halfedge) -> *mut Halfedge;
    fn leftreg(_: *mut Halfedge) -> *mut Site;
    fn rightreg(_: *mut Halfedge) -> *mut Site;
    fn PQinitialize();
    fn PQextractmin() -> *mut Halfedge;
    fn PQ_min() -> Point;
    fn PQempty() -> bool;
    fn PQdelete(_: *mut Halfedge);
    fn PQinsert(_: *mut Halfedge, _: *mut Site, _: libc::c_double);
}
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
pub unsafe extern "C" fn voronoi(
    mut triangulate: libc::c_int,
    mut nextsite: Option::<unsafe extern "C" fn() -> *mut Site>,
) {
    let mut newsite: *mut Site = 0 as *mut Site;
    let mut bot: *mut Site = 0 as *mut Site;
    let mut top: *mut Site = 0 as *mut Site;
    let mut temp: *mut Site = 0 as *mut Site;
    let mut p: *mut Site = 0 as *mut Site;
    let mut v: *mut Site = 0 as *mut Site;
    let mut newintstar: Point = {
        let mut init = Point {
            x: 0 as libc::c_int as libc::c_double,
            y: 0.,
        };
        init
    };
    let mut pm: libc::c_char = 0;
    let mut lbnd: *mut Halfedge = 0 as *mut Halfedge;
    let mut rbnd: *mut Halfedge = 0 as *mut Halfedge;
    let mut llbnd: *mut Halfedge = 0 as *mut Halfedge;
    let mut rrbnd: *mut Halfedge = 0 as *mut Halfedge;
    let mut bisector: *mut Halfedge = 0 as *mut Halfedge;
    let mut e: *mut Edge = 0 as *mut Edge;
    edgeinit();
    siteinit();
    PQinitialize();
    bottomsite = nextsite.expect("non-null function pointer")();
    ELinitialize();
    newsite = nextsite.expect("non-null function pointer")();
    loop {
        if !PQempty() {
            newintstar = PQ_min();
        }
        if !newsite.is_null()
            && (PQempty() as libc::c_int != 0 || (*newsite).coord.y < newintstar.y
                || (*newsite).coord.y == newintstar.y
                    && (*newsite).coord.x < newintstar.x)
        {
            lbnd = ELleftbnd(&mut (*newsite).coord);
            rbnd = ELright(lbnd);
            bot = rightreg(lbnd);
            e = gvbisect(bot, newsite);
            bisector = HEcreate(e, 0 as libc::c_int as libc::c_char);
            ELinsert(lbnd, bisector);
            p = hintersect(lbnd, bisector);
            if !p.is_null() {
                PQdelete(lbnd);
                PQinsert(lbnd, p, dist(p, newsite));
            }
            lbnd = bisector;
            bisector = HEcreate(e, 1 as libc::c_int as libc::c_char);
            ELinsert(lbnd, bisector);
            p = hintersect(bisector, rbnd);
            if !p.is_null() {
                PQinsert(bisector, p, dist(p, newsite));
            }
            newsite = nextsite.expect("non-null function pointer")();
        } else {
            if PQempty() {
                break;
            }
            lbnd = PQextractmin();
            llbnd = ELleft(lbnd);
            rbnd = ELright(lbnd);
            rrbnd = ELright(rbnd);
            bot = leftreg(lbnd);
            top = rightreg(rbnd);
            v = (*lbnd).vertex;
            makevertex(v);
            endpoint((*lbnd).ELedge, (*lbnd).ELpm as libc::c_int, v);
            endpoint((*rbnd).ELedge, (*rbnd).ELpm as libc::c_int, v);
            ELdelete(lbnd);
            PQdelete(rbnd);
            ELdelete(rbnd);
            pm = 0 as libc::c_int as libc::c_char;
            if (*bot).coord.y > (*top).coord.y {
                temp = bot;
                bot = top;
                top = temp;
                pm = 1 as libc::c_int as libc::c_char;
            }
            e = gvbisect(bot, top);
            bisector = HEcreate(e, pm);
            ELinsert(llbnd, bisector);
            endpoint(e, 1 as libc::c_int - pm as libc::c_int, v);
            deref(v);
            p = hintersect(llbnd, bisector);
            if !p.is_null() {
                PQdelete(llbnd);
                PQinsert(llbnd, p, dist(p, bot));
            }
            p = hintersect(bisector, rrbnd);
            if !p.is_null() {
                PQinsert(bisector, p, dist(p, bot));
            }
        }
    }
    lbnd = ELright(ELleftend);
    while lbnd != ELrightend {
        e = (*lbnd).ELedge;
        clip_line(e);
        lbnd = ELright(lbnd);
    }
}
