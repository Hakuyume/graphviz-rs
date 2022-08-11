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
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn zmalloc(_: size_t) -> *mut libc::c_void;
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn Bezier(
        _: *mut pointf,
        _: libc::c_int,
        _: libc::c_double,
        _: *mut pointf,
        _: *mut pointf,
    ) -> pointf;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointf_s {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type pointf = pointf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bezier {
    pub list: *mut pointf,
    pub size: libc::c_int,
    pub sflag: libc::c_int,
    pub eflag: libc::c_int,
    pub sp: pointf,
    pub ep: pointf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stroke_t {
    pub nvertices: libc::c_int,
    pub flags: libc::c_int,
    pub vertices: *mut pointf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pathpoint {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub lengthsofar: libc::c_double,
    pub type_0: libc::c_char,
    pub dir: libc::c_double,
    pub lout: libc::c_double,
    pub bevel: libc::c_int,
    pub dir2: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vararr_t {
    pub pts: *mut pathpoint,
    pub cnt: libc::c_int,
    pub sz: libc::c_int,
}
pub type radfunc_t =
    Option<unsafe extern "C" fn(libc::c_double, libc::c_double, libc::c_double) -> libc::c_double>;
static mut currentmiterlimit: libc::c_double = 10.0f64;
unsafe extern "C" fn addto(mut p: *mut stroke_t, mut x: libc::c_double, mut y: libc::c_double) {
    let mut pt: pointf = pointf { x: 0., y: 0. };
    if (*p).nvertices >= (*p).flags {
        (*p).flags = 2000 as libc::c_int;
        let ref mut fresh0 = (*p).vertices;
        *fresh0 = grealloc(
            (*p).vertices as *mut libc::c_void,
            ((*p).flags as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pointf>() as libc::c_ulong),
        ) as *mut pointf;
    }
    pt.x = x;
    pt.y = y;
    let ref mut fresh1 = (*p).nvertices;
    let fresh2 = *fresh1;
    *fresh1 = *fresh1 + 1;
    *((*p).vertices).offset(fresh2 as isize) = pt;
}
unsafe extern "C" fn arcn(
    mut p: *mut stroke_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut r: libc::c_double,
    mut a1: libc::c_double,
    mut a2: libc::c_double,
) {
    let mut theta: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    addto(p, x + r * cos(a1), y + r * sin(a1));
    if r == 0 as libc::c_int as libc::c_double {
        return;
    }
    while a2 > a1 {
        a2 -= 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
    }
    theta = a1 - a2;
    while theta > 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64 {
        theta -= 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
    }
    theta /= (20 as libc::c_int - 1 as libc::c_int) as libc::c_double;
    i = 1 as libc::c_int;
    while i < 20 as libc::c_int {
        addto(
            p,
            x + r * cos(a1 - i as libc::c_double * theta),
            y + r * sin(a1 - i as libc::c_double * theta),
        );
        i += 1;
    }
}
unsafe extern "C" fn myatan(mut y: libc::c_double, mut x: libc::c_double) -> libc::c_double {
    let mut v: libc::c_double = 0.;
    if x == 0 as libc::c_int as libc::c_double && y == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    } else {
        v = atan2(y, x);
        if v >= 0 as libc::c_int as libc::c_double {
            return v;
        } else {
            return v + 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64;
        }
    };
}
unsafe extern "C" fn mymod(
    mut original: libc::c_double,
    mut modulus: libc::c_double,
) -> libc::c_double {
    let mut v: libc::c_double = 0.;
    if original < 0 as libc::c_int as libc::c_double || original >= modulus {
        v = -floor(original / modulus);
        return v * modulus + original;
    }
    return original;
}
unsafe extern "C" fn newArr() -> *mut vararr_t {
    let mut arr: *mut vararr_t =
        zmalloc(::std::mem::size_of::<vararr_t>() as libc::c_ulong) as *mut vararr_t;
    (*arr).cnt = 0 as libc::c_int;
    (*arr).sz = 2000 as libc::c_int;
    let ref mut fresh3 = (*arr).pts;
    *fresh3 = gcalloc(
        2000 as libc::c_int as size_t,
        ::std::mem::size_of::<pathpoint>() as libc::c_ulong,
    ) as *mut pathpoint;
    return arr;
}
unsafe extern "C" fn insertArr(mut arr: *mut vararr_t, mut p: pointf, mut l: libc::c_double) {
    if (*arr).cnt >= (*arr).sz {
        (*arr).sz *= 2 as libc::c_int;
        let ref mut fresh4 = (*arr).pts;
        *fresh4 = grealloc(
            (*arr).pts as *mut libc::c_void,
            ((*arr).sz as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pathpoint>() as libc::c_ulong),
        ) as *mut pathpoint;
    }
    (*((*arr).pts).offset((*arr).cnt as isize)).x = p.x;
    (*((*arr).pts).offset((*arr).cnt as isize)).y = p.y;
    let ref mut fresh5 = (*arr).cnt;
    let fresh6 = *fresh5;
    *fresh5 = *fresh5 + 1;
    (*((*arr).pts).offset(fresh6 as isize)).lengthsofar = l;
}
unsafe extern "C" fn fixArr(mut arr: *mut vararr_t) {
    if (*arr).sz > (*arr).cnt {
        let ref mut fresh7 = (*arr).pts;
        *fresh7 = grealloc(
            (*arr).pts as *mut libc::c_void,
            ((*arr).cnt as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pathpoint>() as libc::c_ulong),
        ) as *mut pathpoint;
    }
}
unsafe extern "C" fn freeArr(mut arr: *mut vararr_t) {
    free((*arr).pts as *mut libc::c_void);
    free(arr as *mut libc::c_void);
}
unsafe extern "C" fn l2dist(mut p0: pointf, mut p1: pointf) -> libc::c_double {
    let mut delx: libc::c_double = p0.x - p1.x;
    let mut dely: libc::c_double = p0.y - p1.y;
    return hypot(delx, dely);
}
unsafe extern "C" fn pathtolines(mut bez: *mut bezier) -> *mut vararr_t {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut seglen: libc::c_double = 0.;
    let mut linelen: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut arr: *mut vararr_t = newArr();
    let mut p0: pointf = pointf { x: 0., y: 0. };
    let mut p1: pointf = pointf { x: 0., y: 0. };
    let mut V: [pointf; 4] = [pointf { x: 0., y: 0. }; 4];
    let mut n: libc::c_int = (*bez).size;
    let mut A: *mut pointf = (*bez).list;
    insertArr(
        arr,
        *A.offset(0 as libc::c_int as isize),
        0 as libc::c_int as libc::c_double,
    );
    V[3 as libc::c_int as usize] = *A.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while (i + 3 as libc::c_int) < n {
        V[0 as libc::c_int as usize] = V[3 as libc::c_int as usize];
        j = 1 as libc::c_int;
        while j <= 3 as libc::c_int {
            V[j as usize] = *A.offset((i + j) as isize);
            j += 1;
        }
        p0 = V[0 as libc::c_int as usize];
        step = 1 as libc::c_int;
        while step <= 20 as libc::c_int {
            p1 = Bezier(
                V.as_mut_ptr(),
                3 as libc::c_int,
                step as libc::c_double / 20 as libc::c_int as libc::c_double,
                0 as *mut pointf,
                0 as *mut pointf,
            );
            seglen = l2dist(p0, p1);
            linelen += seglen;
            insertArr(arr, p1, linelen);
            p0 = p1;
            step += 1;
        }
        i += 3 as libc::c_int;
    }
    fixArr(arr);
    return arr;
}
unsafe extern "C" fn drawbevel(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut lineout: libc::c_double,
    mut forward: libc::c_int,
    mut dir: libc::c_double,
    mut dir2: libc::c_double,
    mut linejoin: libc::c_int,
    mut p: *mut stroke_t,
) {
    let mut a: libc::c_double = 0.;
    let mut a1: libc::c_double = 0.;
    let mut a2: libc::c_double = 0.;
    if forward != 0 {
        a1 = dir;
        a2 = dir2;
    } else {
        a1 = dir2;
        a2 = dir;
    }
    if linejoin == 1 as libc::c_int {
        a = a1 - a2;
        if a <= 3.14159265358979323846f64 * 0.1f64 / 180.0f64 {
            a += 3.14159265358979323846f64 * 360 as libc::c_int as libc::c_double / 180.0f64;
        }
        if a < 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double / 180.0f64 {
            a1 = a + a2;
            arcn(p, x, y, lineout, a1, a2);
        } else {
            addto(p, x + lineout * cos(a2), x + lineout * sin(a2));
        }
    } else {
        addto(p, x + lineout * cos(a2), x + lineout * sin(a2));
    };
}
#[no_mangle]
pub unsafe extern "C" fn taper(
    mut bez: *mut bezier,
    mut radfunc: radfunc_t,
    mut initwid: libc::c_double,
    mut linejoin: libc::c_int,
    mut linecap: libc::c_int,
) -> *mut stroke_t {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pathcount: libc::c_int = 0;
    let mut bevel: libc::c_int = 0;
    let mut direction: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut direction_2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut arr: *mut vararr_t = pathtolines(bez);
    let mut pathpoints: *mut pathpoint = 0 as *mut pathpoint;
    let mut cur_point: pathpoint = pathpoint {
        x: 0.,
        y: 0.,
        lengthsofar: 0.,
        type_0: 0,
        dir: 0.,
        lout: 0.,
        bevel: 0,
        dir2: 0.,
    };
    let mut last_point: pathpoint = pathpoint {
        x: 0.,
        y: 0.,
        lengthsofar: 0.,
        type_0: 0,
        dir: 0.,
        lout: 0.,
        bevel: 0,
        dir2: 0.,
    };
    let mut next_point: pathpoint = pathpoint {
        x: 0.,
        y: 0.,
        lengthsofar: 0.,
        type_0: 0,
        dir: 0.,
        lout: 0.,
        bevel: 0,
        dir2: 0.,
    };
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dist: libc::c_double = 0.;
    let mut nx: libc::c_double = 0.;
    let mut ny: libc::c_double = 0.;
    let mut ndir: libc::c_double = 0.;
    let mut lx: libc::c_double = 0.;
    let mut ly: libc::c_double = 0.;
    let mut ldir: libc::c_double = 0.;
    let mut lineout: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut linerad: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut linelen: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut theta: libc::c_double = 0.;
    let mut phi: libc::c_double = 0.;
    let mut p: *mut stroke_t = 0 as *mut stroke_t;
    pathcount = (*arr).cnt;
    pathpoints = (*arr).pts;
    linelen = (*pathpoints.offset((pathcount - 1 as libc::c_int) as isize)).lengthsofar;
    i = 0 as libc::c_int;
    while i < pathcount {
        l = mymod(
            (i - 1 as libc::c_int) as libc::c_double,
            pathcount as libc::c_double,
        ) as libc::c_int;
        n = mymod(
            (i + 1 as libc::c_int) as libc::c_double,
            pathcount as libc::c_double,
        ) as libc::c_int;
        cur_point = *pathpoints.offset(i as isize);
        x = cur_point.x;
        y = cur_point.y;
        dist = cur_point.lengthsofar;
        next_point = *pathpoints.offset(n as isize);
        nx = next_point.x;
        ny = next_point.y;
        ndir = myatan(ny - y, nx - x);
        last_point = *pathpoints.offset(l as isize);
        lx = last_point.x;
        ly = last_point.y;
        ldir = myatan(ly - y, lx - x);
        bevel = 0 as libc::c_int;
        direction_2 = 0 as libc::c_int as libc::c_double;
        linerad = radfunc.expect("non-null function pointer")(dist, linelen, initwid);
        if i == 0 as libc::c_int || i == pathcount - 1 as libc::c_int {
            lineout = linerad;
            if i == 0 as libc::c_int {
                direction = ndir
                    + 3.14159265358979323846f64 * 90 as libc::c_int as libc::c_double / 180.0f64;
                if linecap == 2 as libc::c_int {
                    x -= cos(ndir) * lineout;
                    y -= sin(ndir) * lineout;
                }
            } else {
                direction = ldir
                    - 3.14159265358979323846f64 * 90 as libc::c_int as libc::c_double / 180.0f64;
                if linecap == 2 as libc::c_int {
                    x -= cos(ldir) * lineout;
                    y -= sin(ldir) * lineout;
                }
            }
            direction_2 = direction;
        } else {
            theta = ndir - ldir;
            if theta < 0 as libc::c_int as libc::c_double {
                theta +=
                    3.14159265358979323846f64 * 360 as libc::c_int as libc::c_double / 180.0f64;
            }
            phi = 3.14159265358979323846f64 * 90 as libc::c_int as libc::c_double / 180.0f64
                - theta / 2 as libc::c_int as libc::c_double;
            if cos(phi) == 0 as libc::c_int as libc::c_double {
                lineout = 0 as libc::c_int as libc::c_double;
            } else {
                lineout = linerad / cos(phi);
            }
            direction = ndir
                + 3.14159265358979323846f64 * 90 as libc::c_int as libc::c_double / 180.0f64
                + phi;
            if 0 as libc::c_int != linejoin || lineout > currentmiterlimit * linerad {
                bevel = (0 as libc::c_int == 0) as libc::c_int;
                lineout = linerad;
                direction = mymod(
                    ldir - 3.14159265358979323846f64 * 90 as libc::c_int as libc::c_double
                        / 180.0f64,
                    3.14159265358979323846f64 * 360 as libc::c_int as libc::c_double / 180.0f64,
                );
                direction_2 = mymod(
                    ndir + 3.14159265358979323846f64 * 90 as libc::c_int as libc::c_double
                        / 180.0f64,
                    3.14159265358979323846f64 * 360 as libc::c_int as libc::c_double / 180.0f64,
                );
                if i == pathcount - 1 as libc::c_int {
                    bevel = 0 as libc::c_int;
                }
            } else {
                direction_2 = direction;
            }
        }
        (*pathpoints.offset(i as isize)).x = x;
        (*pathpoints.offset(i as isize)).y = y;
        (*pathpoints.offset(i as isize)).lengthsofar = dist;
        (*pathpoints.offset(i as isize)).type_0 = 'l' as i32 as libc::c_char;
        (*pathpoints.offset(i as isize)).dir = direction;
        (*pathpoints.offset(i as isize)).lout = lineout;
        (*pathpoints.offset(i as isize)).bevel = bevel;
        (*pathpoints.offset(i as isize)).dir2 = direction_2;
        i += 1;
    }
    p = zmalloc(::std::mem::size_of::<stroke_t>() as libc::c_ulong) as *mut stroke_t;
    i = 0 as libc::c_int;
    while i < pathcount {
        cur_point = *pathpoints.offset(i as isize);
        x = cur_point.x;
        y = cur_point.y;
        direction = cur_point.dir;
        lineout = cur_point.lout;
        bevel = cur_point.bevel;
        direction_2 = cur_point.dir2;
        if i == 0 as libc::c_int {
            addto(
                p,
                x + cos(direction) * lineout,
                y + sin(direction) * lineout,
            );
        } else {
            addto(
                p,
                x + cos(direction) * lineout,
                y + sin(direction) * lineout,
            );
        }
        if bevel != 0 {
            drawbevel(
                x,
                y,
                lineout,
                (0 as libc::c_int == 0) as libc::c_int,
                direction,
                direction_2,
                linejoin,
                p,
            );
        }
        i += 1;
    }
    if linecap == 1 as libc::c_int {
        arcn(
            p,
            x,
            y,
            lineout,
            direction,
            direction + 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double / 180.0f64,
        );
    } else {
        direction += 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double / 180.0f64;
        addto(
            p,
            x + cos(direction) * lineout,
            y + sin(direction) * lineout,
        );
    }
    i = pathcount - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        cur_point = *pathpoints.offset(i as isize);
        x = cur_point.x;
        y = cur_point.y;
        direction = cur_point.dir
            + 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double / 180.0f64;
        lineout = cur_point.lout;
        bevel = cur_point.bevel;
        direction_2 = cur_point.dir2
            + 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double / 180.0f64;
        addto(
            p,
            x + cos(direction_2) * lineout,
            y + sin(direction_2) * lineout,
        );
        if bevel != 0 {
            drawbevel(
                x,
                y,
                lineout,
                0 as libc::c_int,
                direction,
                direction_2,
                linejoin,
                p,
            );
        }
        i -= 1;
    }
    if linecap == 1 as libc::c_int {
        arcn(
            p,
            x,
            y,
            lineout,
            direction,
            direction + 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double / 180.0f64,
        );
    }
    freeArr(arr);
    return p;
}
