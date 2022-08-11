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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn agerr(level: agerrlevel_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    static mut Verbose: libc::c_uchar;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn in_poly(argpoly: Ppoly_t, q: Ppoint_t) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
}
pub type Ppoint_t = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ppoly_t {
    pub ps: *mut Ppoint_t,
    pub pn: libc::c_int,
}
pub type agerrlevel_t = libc::c_uint;
pub const AGPREV: agerrlevel_t = 3;
pub const AGMAX: agerrlevel_t = 2;
pub const AGERR: agerrlevel_t = 1;
pub const AGWARN: agerrlevel_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vertex {
    pub pos: pointf,
    pub poly: *mut polygon,
    pub active: *mut active_edge,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct active_edge {
    pub name: *mut vertex,
    pub next: *mut active_edge,
    pub last: *mut active_edge,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polygon {
    pub start: *mut vertex,
    pub finish: *mut vertex,
    pub bb: boxf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intersection {
    pub firstv: *mut vertex,
    pub secondv: *mut vertex,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data {
    pub nvertices: libc::c_int,
    pub npolygons: libc::c_int,
    pub ninters: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct active_edge_list {
    pub first: *mut active_edge,
    pub final_0: *mut active_edge,
    pub number: libc::c_int,
}
unsafe extern "C" fn sign(mut v: libc::c_double) -> libc::c_int {
    if v < 0 as libc::c_int as libc::c_double {
        return -(1 as libc::c_int);
    }
    if v > 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sgnarea(mut l: *mut vertex, mut m: *mut vertex, mut i: *mut libc::c_int) {
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    a = (*l).pos.x;
    b = (*l).pos.y;
    c = (*(if l == (*(*l).poly).finish {
        (*(*l).poly).start
    } else {
        l.offset(1 as libc::c_int as isize)
    }))
    .pos
    .x - a;
    d = (*(if l == (*(*l).poly).finish {
        (*(*l).poly).start
    } else {
        l.offset(1 as libc::c_int as isize)
    }))
    .pos
    .y - b;
    e = (*m).pos.x - a;
    f = (*m).pos.y - b;
    g = (*(if m == (*(*m).poly).finish {
        (*(*m).poly).start
    } else {
        m.offset(1 as libc::c_int as isize)
    }))
    .pos
    .x - a;
    h = (*(if m == (*(*m).poly).finish {
        (*(*m).poly).start
    } else {
        m.offset(1 as libc::c_int as isize)
    }))
    .pos
    .y - b;
    t = c * f - d * e;
    *i.offset(0 as libc::c_int as isize) = sign(t);
    t = c * h - d * g;
    *i.offset(1 as libc::c_int as isize) = sign(t);
    *i.offset(2 as libc::c_int as isize) =
        *i.offset(0 as libc::c_int as isize) * *i.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn between(
    mut f: libc::c_double,
    mut g: libc::c_double,
    mut h: libc::c_double,
) -> libc::c_int {
    if f < g {
        if g < h {
            return 1 as libc::c_int;
        }
        if g > h {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if f > g {
        if g > h {
            return 1 as libc::c_int;
        }
        if g < h {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn online(
    mut l: *mut vertex,
    mut m: *mut vertex,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut a: pointf = Ppoint_t { x: 0., y: 0. };
    let mut b: pointf = Ppoint_t { x: 0., y: 0. };
    let mut c: pointf = Ppoint_t { x: 0., y: 0. };
    a = (*l).pos;
    b = (*if l == (*(*l).poly).finish {
        (*(*l).poly).start
    } else {
        l.offset(1 as libc::c_int as isize)
    })
    .pos;
    c = if i == 0 as libc::c_int {
        (*m).pos
    } else {
        (*if m == (*(*m).poly).finish {
            (*(*m).poly).start
        } else {
            m.offset(1 as libc::c_int as isize)
        })
        .pos
    };
    return if a.x == b.x {
        (a.x == c.x && -(1 as libc::c_int) != between(a.y, c.y, b.y)) as libc::c_int
    } else {
        between(a.x, c.x, b.x)
    };
}
unsafe extern "C" fn intpoint(
    mut l: *mut vertex,
    mut m: *mut vertex,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut cond: libc::c_int,
) -> libc::c_int {
    let mut ls: pointf = Ppoint_t { x: 0., y: 0. };
    let mut le: pointf = Ppoint_t { x: 0., y: 0. };
    let mut ms: pointf = Ppoint_t { x: 0., y: 0. };
    let mut me: pointf = Ppoint_t { x: 0., y: 0. };
    let mut pt1: pointf = Ppoint_t { x: 0., y: 0. };
    let mut pt2: pointf = Ppoint_t { x: 0., y: 0. };
    let mut m1: libc::c_double = 0.;
    let mut m2: libc::c_double = 0.;
    let mut c1: libc::c_double = 0.;
    let mut c2: libc::c_double = 0.;
    if cond <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ls = (*l).pos;
    le = (*if l == (*(*l).poly).finish {
        (*(*l).poly).start
    } else {
        l.offset(1 as libc::c_int as isize)
    })
    .pos;
    ms = (*m).pos;
    me = (*if m == (*(*m).poly).finish {
        (*(*m).poly).start
    } else {
        m.offset(1 as libc::c_int as isize)
    })
    .pos;
    match cond {
        3 => {
            if ls.x == le.x {
                *x = ls.x;
                *y = me.y + (ms.y - me.y) / (ms.x - me.x) * (*x - me.x);
            } else if ms.x == me.x {
                *x = ms.x;
                *y = le.y + (ls.y - le.y) / (ls.x - le.x) * (*x - le.x);
            } else {
                m1 = (ms.y - me.y) / (ms.x - me.x);
                m2 = (ls.y - le.y) / (ls.x - le.x);
                c1 = ms.y - m1 * ms.x;
                c2 = ls.y - m2 * ls.x;
                *x = (c2 - c1) / (m1 - m2);
                *y = (m1 * c2 - c1 * m2) / (m1 - m2);
            }
        }
        2 => {
            if online(l, m, 0 as libc::c_int) == -(1 as libc::c_int) {
                pt1 = ms;
                pt2 = if online(m, l, 1 as libc::c_int) == -(1 as libc::c_int) {
                    if online(m, l, 0 as libc::c_int) == -(1 as libc::c_int) {
                        le
                    } else {
                        ls
                    }
                } else {
                    me
                };
            } else if online(l, m, 1 as libc::c_int) == -(1 as libc::c_int) {
                pt1 = me;
                pt2 = if online(l, m, 0 as libc::c_int) == -(1 as libc::c_int) {
                    if online(m, l, 0 as libc::c_int) == -(1 as libc::c_int) {
                        le
                    } else {
                        ls
                    }
                } else {
                    ms
                };
            } else {
                if online(m, l, 0 as libc::c_int) != -(1 as libc::c_int) {
                    return 0 as libc::c_int;
                }
                pt1 = ls;
                pt2 = le;
            }
            *x = (pt1.x + pt2.x) / 2 as libc::c_int as libc::c_double;
            *y = (pt1.y + pt2.y) / 2 as libc::c_int as libc::c_double;
        }
        1 => {
            if (ls.x - le.x) * (ms.y - ls.y) == (ls.y - le.y) * (ms.x - ls.x) {
                *x = ms.x;
                *y = ms.y;
            } else {
                *x = me.x;
                *y = me.y;
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn putSeg(mut i: libc::c_int, mut v: *mut vertex) {
    fprintf(
        stderr,
        b"seg#%d : (%.3f, %.3f) (%.3f, %.3f)\n\0" as *const u8 as *const libc::c_char,
        i,
        (*v).pos.x,
        (*v).pos.y,
        (*if v == (*(*v).poly).finish {
            (*(*v).poly).start
        } else {
            v.offset(1 as libc::c_int as isize)
        })
        .pos
        .x,
        (*if v == (*(*v).poly).finish {
            (*(*v).poly).start
        } else {
            v.offset(1 as libc::c_int as isize)
        })
        .pos
        .y,
    );
}
unsafe extern "C" fn realIntersect(
    mut firstv: *mut vertex,
    mut secondv: *mut vertex,
    mut p: pointf,
) -> libc::c_int {
    let mut vft: pointf = Ppoint_t { x: 0., y: 0. };
    let mut vsd: pointf = Ppoint_t { x: 0., y: 0. };
    let mut avft: pointf = Ppoint_t { x: 0., y: 0. };
    let mut avsd: pointf = Ppoint_t { x: 0., y: 0. };
    vft = (*firstv).pos;
    avft = (*if firstv == (*(*firstv).poly).finish {
        (*(*firstv).poly).start
    } else {
        firstv.offset(1 as libc::c_int as isize)
    })
    .pos;
    vsd = (*secondv).pos;
    avsd = (*if secondv == (*(*secondv).poly).finish {
        (*(*secondv).poly).start
    } else {
        secondv.offset(1 as libc::c_int as isize)
    })
    .pos;
    if vft.x != avft.x && vsd.x != avsd.x
        || vft.x == avft.x && !(vft.x == p.x && vft.y == p.y) && !(avft.x == p.x && avft.y == p.y)
        || vsd.x == avsd.x && !(vsd.x == p.x && vsd.y == p.y) && !(avsd.x == p.x && avsd.y == p.y)
    {
        if Verbose as libc::c_int > 1 as libc::c_int {
            fprintf(
                stderr,
                b"\nintersection at %.3f %.3f\n\0" as *const u8 as *const libc::c_char,
                p.x,
                p.y,
            );
            putSeg(1 as libc::c_int, firstv);
            putSeg(2 as libc::c_int, secondv);
        }
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn find_intersection(
    mut l: *mut vertex,
    mut m: *mut vertex,
    mut ilist: *mut intersection,
    mut input: *mut data,
) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut p: pointf = Ppoint_t { x: 0., y: 0. };
    let mut i: [libc::c_int; 3] = [0; 3];
    sgnarea(l, m, i.as_mut_ptr());
    if i[2 as libc::c_int as usize] > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if i[2 as libc::c_int as usize] < 0 as libc::c_int {
        sgnarea(m, l, i.as_mut_ptr());
        if i[2 as libc::c_int as usize] > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if intpoint(
            l,
            m,
            &mut x,
            &mut y,
            if i[2 as libc::c_int as usize] < 0 as libc::c_int {
                3 as libc::c_int
            } else {
                online(m, l, abs(i[0 as libc::c_int as usize]))
            },
        ) == 0
        {
            return 0 as libc::c_int;
        }
    } else if intpoint(
        l,
        m,
        &mut x,
        &mut y,
        if i[0 as libc::c_int as usize] == i[1 as libc::c_int as usize] {
            2 as libc::c_int
                * (if online(l, m, 0 as libc::c_int) > online(l, m, 1 as libc::c_int) {
                    online(l, m, 0 as libc::c_int)
                } else {
                    online(l, m, 1 as libc::c_int)
                })
        } else {
            online(l, m, abs(i[0 as libc::c_int as usize]))
        },
    ) == 0
    {
        return 0 as libc::c_int;
    }
    p.x = x;
    p.y = y;
    return realIntersect(l, m, p);
}
unsafe extern "C" fn gt(mut a: *const libc::c_void, mut b: *const libc::c_void) -> libc::c_int {
    let mut i: *const *const vertex = a as *const *const vertex;
    let mut j: *const *const vertex = b as *const *const vertex;
    if (**i).pos.x > (**j).pos.x {
        return 1 as libc::c_int;
    }
    if (**i).pos.x < (**j).pos.x {
        return -(1 as libc::c_int);
    }
    if (**i).pos.y > (**j).pos.y {
        return 1 as libc::c_int;
    }
    if (**i).pos.y < (**j).pos.y {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_ints(
    mut vertex_list: *mut vertex,
    mut input: *mut data,
    mut ilist: *mut intersection,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut all: active_edge_list = active_edge_list {
        first: 0 as *mut active_edge,
        final_0: 0 as *mut active_edge,
        number: 0,
    };
    let mut new: *mut active_edge = 0 as *mut active_edge;
    let mut tempa: *mut active_edge = 0 as *mut active_edge;
    let mut pt1: *mut vertex = 0 as *mut vertex;
    let mut pt2: *mut vertex = 0 as *mut vertex;
    let mut templ: *mut vertex = 0 as *mut vertex;
    let mut pvertex: *mut *mut vertex = 0 as *mut *mut vertex;
    (*input).ninters = 0 as libc::c_int;
    all.final_0 = 0 as *mut active_edge;
    all.first = all.final_0;
    all.number = 0 as libc::c_int;
    pvertex = gcalloc(
        (*input).nvertices as size_t,
        ::std::mem::size_of::<*mut vertex>() as libc::c_ulong,
    ) as *mut *mut vertex;
    i = 0 as libc::c_int;
    while i < (*input).nvertices {
        let ref mut fresh0 = *pvertex.offset(i as isize);
        *fresh0 = vertex_list.offset(i as isize);
        i += 1;
    }
    qsort(
        pvertex as *mut libc::c_void,
        (*input).nvertices as size_t,
        ::std::mem::size_of::<*mut vertex>() as libc::c_ulong,
        Some(gt as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int),
    );
    i = 0 as libc::c_int;
    's_50: while i < (*input).nvertices {
        pt1 = *pvertex.offset(i as isize);
        pt2 = if *pvertex.offset(i as isize) == (*(**pvertex.offset(i as isize)).poly).start {
            (*(**pvertex.offset(i as isize)).poly).finish
        } else {
            (*pvertex.offset(i as isize)).offset(-(1 as libc::c_int as isize))
        };
        templ = pt2;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            match gt(
                &mut pt1 as *mut *mut vertex as *const libc::c_void,
                &mut pt2 as *mut *mut vertex as *const libc::c_void,
            ) {
                -1 => {
                    tempa = all.first;
                    j = 0 as libc::c_int;
                    while j < all.number {
                        found = find_intersection((*tempa).name, templ, ilist, input);
                        if found != 0 {
                            break 's_50;
                        }
                        j += 1;
                        tempa = (*tempa).next;
                    }
                    new = gmalloc(::std::mem::size_of::<active_edge>() as libc::c_ulong)
                        as *mut active_edge;
                    if all.number == 0 as libc::c_int {
                        all.first = new;
                        let ref mut fresh1 = (*new).last;
                        *fresh1 = 0 as *mut active_edge;
                    } else {
                        let ref mut fresh2 = (*all.final_0).next;
                        *fresh2 = new;
                        let ref mut fresh3 = (*new).last;
                        *fresh3 = all.final_0;
                    }
                    let ref mut fresh4 = (*new).name;
                    *fresh4 = templ;
                    let ref mut fresh5 = (*new).next;
                    *fresh5 = 0 as *mut active_edge;
                    let ref mut fresh6 = (*templ).active;
                    *fresh6 = new;
                    all.final_0 = new;
                    all.number += 1;
                }
                1 => {
                    tempa = (*templ).active;
                    if tempa.is_null() {
                        agerr(
                            AGERR,
                            b"trying to delete a non-line\n\0" as *const u8 as *const libc::c_char,
                        );
                        return -(1 as libc::c_int);
                    }
                    if all.number == 1 as libc::c_int {
                        all.first = 0 as *mut active_edge;
                        all.final_0 = all.first;
                    } else if tempa == all.first {
                        all.first = (*all.first).next;
                        let ref mut fresh7 = (*all.first).last;
                        *fresh7 = 0 as *mut active_edge;
                    } else if tempa == all.final_0 {
                        all.final_0 = (*all.final_0).last;
                        let ref mut fresh8 = (*all.final_0).next;
                        *fresh8 = 0 as *mut active_edge;
                    } else {
                        let ref mut fresh9 = (*(*tempa).last).next;
                        *fresh9 = (*tempa).next;
                        let ref mut fresh10 = (*(*tempa).next).last;
                        *fresh10 = (*tempa).last;
                    }
                    free(tempa as *mut libc::c_void);
                    all.number -= 1;
                    let ref mut fresh11 = (*templ).active;
                    *fresh11 = 0 as *mut active_edge;
                }
                _ => {}
            }
            pt2 = if *pvertex.offset(i as isize) == (*(**pvertex.offset(i as isize)).poly).finish {
                (*(**pvertex.offset(i as isize)).poly).start
            } else {
                (*pvertex.offset(i as isize)).offset(1 as libc::c_int as isize)
            };
            templ = *pvertex.offset(i as isize);
            k += 1;
        }
        i += 1;
    }
    tempa = all.first;
    j = 0 as libc::c_int;
    while j < all.number {
        new = (*tempa).next;
        free(tempa as *mut libc::c_void);
        j += 1;
        tempa = new;
    }
    free(pvertex as *mut libc::c_void);
    return found;
}
unsafe extern "C" fn findInside(
    mut polys: *mut *mut Ppoly_t,
    mut n_polys: libc::c_int,
    mut polygon_list: *mut polygon,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pt: pointf = Ppoint_t { x: 0., y: 0. };
    let mut p1: *mut Ppoly_t = 0 as *mut Ppoly_t;
    let mut p2: *mut Ppoly_t = 0 as *mut Ppoly_t;
    i = 0 as libc::c_int;
    while i < n_polys {
        p1 = *polys.offset(i as isize);
        pt = *((*p1).ps).offset(0 as libc::c_int as isize);
        j = i + 1 as libc::c_int;
        while j < n_polys {
            p2 = *polys.offset(j as isize);
            if (*polygon_list.offset(i as isize)).bb.LL.x
                <= (*polygon_list.offset(j as isize)).bb.UR.x
                && (*polygon_list.offset(i as isize)).bb.LL.x
                    >= (*polygon_list.offset(j as isize)).bb.LL.x
                && (*polygon_list.offset(i as isize)).bb.LL.y
                    <= (*polygon_list.offset(j as isize)).bb.UR.y
                && (*polygon_list.offset(i as isize)).bb.LL.y
                    >= (*polygon_list.offset(j as isize)).bb.LL.y
                && ((*polygon_list.offset(i as isize)).bb.UR.x
                    <= (*polygon_list.offset(j as isize)).bb.UR.x
                    && (*polygon_list.offset(i as isize)).bb.UR.x
                        >= (*polygon_list.offset(j as isize)).bb.LL.x
                    && (*polygon_list.offset(i as isize)).bb.UR.y
                        <= (*polygon_list.offset(j as isize)).bb.UR.y
                    && (*polygon_list.offset(i as isize)).bb.UR.y
                        >= (*polygon_list.offset(j as isize)).bb.LL.y)
            {
                if in_poly(*p2, pt) != 0 {
                    return 1 as libc::c_int;
                }
            } else if (*polygon_list.offset(j as isize)).bb.LL.x
                <= (*polygon_list.offset(i as isize)).bb.UR.x
                && (*polygon_list.offset(j as isize)).bb.LL.x
                    >= (*polygon_list.offset(i as isize)).bb.LL.x
                && (*polygon_list.offset(j as isize)).bb.LL.y
                    <= (*polygon_list.offset(i as isize)).bb.UR.y
                && (*polygon_list.offset(j as isize)).bb.LL.y
                    >= (*polygon_list.offset(i as isize)).bb.LL.y
                && ((*polygon_list.offset(j as isize)).bb.UR.x
                    <= (*polygon_list.offset(i as isize)).bb.UR.x
                    && (*polygon_list.offset(j as isize)).bb.UR.x
                        >= (*polygon_list.offset(i as isize)).bb.LL.x
                    && (*polygon_list.offset(j as isize)).bb.UR.y
                        <= (*polygon_list.offset(i as isize)).bb.UR.y
                    && (*polygon_list.offset(j as isize)).bb.UR.y
                        >= (*polygon_list.offset(i as isize)).bb.LL.y)
            {
                if in_poly(*p1, *((*p2).ps).offset(0 as libc::c_int as isize)) != 0 {
                    return 1 as libc::c_int;
                }
            }
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Plegal_arrangement(
    mut polys: *mut *mut Ppoly_t,
    mut n_polys: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut vno: libc::c_int = 0;
    let mut nverts: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut vertex_list: *mut vertex = 0 as *mut vertex;
    let mut polygon_list: *mut polygon = 0 as *mut polygon;
    let mut input: data = data {
        nvertices: 0,
        npolygons: 0,
        ninters: 0,
    };
    let mut ilist: [intersection; 10000] = [intersection {
        firstv: 0 as *mut vertex,
        secondv: 0 as *mut vertex,
        x: 0.,
        y: 0.,
    }; 10000];
    let mut bb: boxf = boxf {
        LL: Ppoint_t { x: 0., y: 0. },
        UR: Ppoint_t { x: 0., y: 0. },
    };
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    polygon_list = gcalloc(
        n_polys as size_t,
        ::std::mem::size_of::<polygon>() as libc::c_ulong,
    ) as *mut polygon;
    nverts = 0 as libc::c_int;
    i = nverts;
    while i < n_polys {
        nverts += (**polys.offset(i as isize)).pn;
        i += 1;
    }
    vertex_list = gcalloc(
        nverts as size_t,
        ::std::mem::size_of::<vertex>() as libc::c_ulong,
    ) as *mut vertex;
    vno = 0 as libc::c_int;
    i = vno;
    while i < n_polys {
        let ref mut fresh12 = (*polygon_list.offset(i as isize)).start;
        *fresh12 = &mut *vertex_list.offset(vno as isize) as *mut vertex;
        bb.LL.y = 1.7976931348623157e+308f64;
        bb.LL.x = bb.LL.y;
        bb.UR.y = -1.7976931348623157e+308f64;
        bb.UR.x = bb.UR.y;
        j = 0 as libc::c_int;
        while j < (**polys.offset(i as isize)).pn {
            x = (*((**polys.offset(i as isize)).ps).offset(j as isize)).x;
            y = (*((**polys.offset(i as isize)).ps).offset(j as isize)).y;
            bb.LL.x = if bb.LL.x < x { bb.LL.x } else { x };
            bb.LL.y = if bb.LL.y < y { bb.LL.y } else { y };
            bb.UR.x = if bb.UR.x > x { bb.UR.x } else { x };
            bb.UR.y = if bb.UR.y > y { bb.UR.y } else { y };
            (*vertex_list.offset(vno as isize)).pos.x = x;
            (*vertex_list.offset(vno as isize)).pos.y = y;
            let ref mut fresh13 = (*vertex_list.offset(vno as isize)).poly;
            *fresh13 = &mut *polygon_list.offset(i as isize) as *mut polygon;
            let ref mut fresh14 = (*vertex_list.offset(vno as isize)).active;
            *fresh14 = 0 as *mut active_edge;
            vno += 1;
            j += 1;
        }
        let ref mut fresh15 = (*polygon_list.offset(i as isize)).finish;
        *fresh15 = &mut *vertex_list.offset((vno - 1 as libc::c_int) as isize) as *mut vertex;
        (*polygon_list.offset(i as isize)).bb = bb;
        i += 1;
    }
    input.nvertices = nverts;
    input.npolygons = n_polys;
    found = find_ints(vertex_list, &mut input, ilist.as_mut_ptr());
    if found < 0 as libc::c_int {
        free(polygon_list as *mut libc::c_void);
        free(vertex_list as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    if found == 0 {
        found = findInside(polys, n_polys, polygon_list);
    }
    free(polygon_list as *mut libc::c_void);
    free(vertex_list as *mut libc::c_void);
    return (found == 0) as libc::c_int;
}
