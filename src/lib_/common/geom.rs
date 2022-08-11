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
    fn sincos(__x: libc::c_double, __sinx: *mut libc::c_double, __cosx: *mut libc::c_double);
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: libc::c_int,
    pub y: libc::c_int,
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
pub struct boxf {
    pub LL: pointf,
    pub UR: pointf,
}
#[inline]
unsafe extern "C" fn sub_pointf(mut p: pointf, mut q: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = p.x - q.x;
    r.y = p.y - q.y;
    return r;
}
#[inline]
unsafe extern "C" fn perp(mut p: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = -p.y;
    r.y = p.x;
    return r;
}
#[inline]
unsafe extern "C" fn scale(mut c: libc::c_double, mut p: pointf) -> pointf {
    let mut r: pointf = pointf { x: 0., y: 0. };
    r.x = c * p.x;
    r.y = c * p.y;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn lineToBox(mut p: pointf, mut q: pointf, mut b: boxf) -> libc::c_int {
    let mut inside1: libc::c_int = 0;
    let mut inside2: libc::c_int = 0;
    inside1 = (p.x >= b.LL.x && p.x <= b.UR.x && p.y >= b.LL.y && p.y <= b.UR.y) as libc::c_int;
    inside2 = (q.x >= b.LL.x && q.x <= b.UR.x && q.y >= b.LL.y && q.y <= b.UR.y) as libc::c_int;
    if inside1 != inside2 {
        return 0 as libc::c_int;
    }
    if inside1 & inside2 != 0 {
        return 1 as libc::c_int;
    }
    if p.x == q.x {
        if (p.y >= b.LL.y) as libc::c_int ^ (q.y >= b.LL.y) as libc::c_int != 0
            && p.x >= b.LL.x
            && p.x <= b.UR.x
        {
            return 0 as libc::c_int;
        }
    } else if p.y == q.y {
        if (p.x >= b.LL.x) as libc::c_int ^ (q.x >= b.LL.x) as libc::c_int != 0
            && p.y >= b.LL.y
            && p.y <= b.UR.y
        {
            return 0 as libc::c_int;
        }
    } else {
        let mut m: libc::c_double = 0.;
        let mut x: libc::c_double = 0.;
        let mut y: libc::c_double = 0.;
        let mut low: libc::c_double = 0.;
        let mut high: libc::c_double = 0.;
        m = (q.y - p.y) / (q.x - p.x);
        if p.x < q.x {
            low = p.x;
            high = q.x;
        } else {
            low = q.x;
            high = p.x;
        }
        y = p.y + (b.LL.x - p.x) * m;
        if b.LL.x >= low && b.LL.x <= high && y >= b.LL.y && y <= b.UR.y {
            return 0 as libc::c_int;
        }
        y += (b.UR.x - b.LL.x) * m;
        if y >= b.LL.y && y <= b.UR.y && b.UR.x >= low && b.UR.x <= high {
            return 0 as libc::c_int;
        }
        if p.y < q.y {
            low = p.y;
            high = q.y;
        } else {
            low = q.y;
            high = p.y;
        }
        x = p.x + (b.LL.y - p.y) / m;
        if x >= b.LL.x && x <= b.UR.x && b.LL.y >= low && b.LL.y <= high {
            return 0 as libc::c_int;
        }
        x += (b.UR.y - b.LL.y) / m;
        if x >= b.LL.x && x <= b.UR.x && b.UR.y >= low && b.UR.y <= high {
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rect2poly(mut p: *mut pointf) {
    let ref mut fresh0 = (*p.offset(2 as libc::c_int as isize)).x;
    *fresh0 = (*p.offset(1 as libc::c_int as isize)).x;
    (*p.offset(3 as libc::c_int as isize)).x = *fresh0;
    (*p.offset(2 as libc::c_int as isize)).y = (*p.offset(1 as libc::c_int as isize)).y;
    (*p.offset(3 as libc::c_int as isize)).y = (*p.offset(0 as libc::c_int as isize)).y;
    (*p.offset(1 as libc::c_int as isize)).x = (*p.offset(0 as libc::c_int as isize)).x;
}
unsafe extern "C" fn rotatepf(mut p: pointf, mut cwrot: libc::c_int) -> pointf {
    static mut sina: libc::c_double = 0.;
    static mut cosa: libc::c_double = 0.;
    static mut last_cwrot: libc::c_int = 0;
    let mut P: pointf = pointf { x: 0., y: 0. };
    if cwrot != last_cwrot {
        sincos(
            cwrot as libc::c_double
                / (2 as libc::c_int as libc::c_double * 3.14159265358979323846f64),
            &mut sina,
            &mut cosa,
        );
        last_cwrot = cwrot;
    }
    P.x = p.x * cosa - p.y * sina;
    P.y = p.y * cosa + p.x * sina;
    return P;
}
unsafe extern "C" fn rotatep(mut p: point, mut cwrot: libc::c_int) -> point {
    let mut pf: pointf = pointf { x: 0., y: 0. };
    pf.x = p.x as libc::c_double;
    pf.y = p.y as libc::c_double;
    pf = rotatepf(pf, cwrot);
    p.x = (if pf.x >= 0 as libc::c_int as libc::c_double {
        (pf.x + 0.5f64) as libc::c_int
    } else {
        (pf.x - 0.5f64) as libc::c_int
    });
    p.y = (if pf.y >= 0 as libc::c_int as libc::c_double {
        (pf.y + 0.5f64) as libc::c_int
    } else {
        (pf.y - 0.5f64) as libc::c_int
    });
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn cwrotatep(mut p: point, mut cwrot: libc::c_int) -> point {
    let mut x: libc::c_int = p.x;
    let mut y: libc::c_int = p.y;
    match cwrot {
        0 => {}
        90 => {
            p.x = y;
            p.y = -x;
        }
        180 => {
            p.x = x;
            p.y = -y;
        }
        270 => {
            p.x = y;
            p.y = x;
        }
        _ => {
            if cwrot < 0 as libc::c_int {
                return ccwrotatep(p, -cwrot);
            }
            if cwrot > 360 as libc::c_int {
                return cwrotatep(p, cwrot % 360 as libc::c_int);
            }
            return rotatep(p, cwrot);
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn cwrotatepf(mut p: pointf, mut cwrot: libc::c_int) -> pointf {
    let mut x: libc::c_double = p.x;
    let mut y: libc::c_double = p.y;
    match cwrot {
        0 => {}
        90 => {
            p.x = y;
            p.y = -x;
        }
        180 => {
            p.x = x;
            p.y = -y;
        }
        270 => {
            p.x = y;
            p.y = x;
        }
        _ => {
            if cwrot < 0 as libc::c_int {
                return ccwrotatepf(p, -cwrot);
            }
            if cwrot > 360 as libc::c_int {
                return cwrotatepf(p, cwrot % 360 as libc::c_int);
            }
            return rotatepf(p, cwrot);
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ccwrotatep(mut p: point, mut ccwrot: libc::c_int) -> point {
    let mut x: libc::c_int = p.x;
    let mut y: libc::c_int = p.y;
    match ccwrot {
        0 => {}
        90 => {
            p.x = -y;
            p.y = x;
        }
        180 => {
            p.x = x;
            p.y = -y;
        }
        270 => {
            p.x = y;
            p.y = x;
        }
        _ => {
            if ccwrot < 0 as libc::c_int {
                return cwrotatep(p, -ccwrot);
            }
            if ccwrot > 360 as libc::c_int {
                return ccwrotatep(p, ccwrot % 360 as libc::c_int);
            }
            return rotatep(p, 360 as libc::c_int - ccwrot);
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ccwrotatepf(mut p: pointf, mut ccwrot: libc::c_int) -> pointf {
    let mut x: libc::c_double = p.x;
    let mut y: libc::c_double = p.y;
    match ccwrot {
        0 => {}
        90 => {
            p.x = -y;
            p.y = x;
        }
        180 => {
            p.x = x;
            p.y = -y;
        }
        270 => {
            p.x = y;
            p.y = x;
        }
        _ => {
            if ccwrot < 0 as libc::c_int {
                return cwrotatepf(p, -ccwrot);
            }
            if ccwrot > 360 as libc::c_int {
                return ccwrotatepf(p, ccwrot % 360 as libc::c_int);
            }
            return rotatepf(p, 360 as libc::c_int - ccwrot);
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn flip_rec_boxf(mut b: boxf, mut p: pointf) -> boxf {
    let mut r: boxf = boxf {
        LL: pointf { x: 0., y: 0. },
        UR: pointf { x: 0., y: 0. },
    };
    r.UR.x = b.UR.y;
    r.UR.y = b.UR.x;
    r.LL.x = b.LL.y;
    r.LL.y = b.LL.x;
    r.LL.x += p.x;
    r.LL.y += p.y;
    r.UR.x += p.x;
    r.UR.y += p.y;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn ptToLine2(mut a: pointf, mut b: pointf, mut p: pointf) -> libc::c_double {
    let mut dx: libc::c_double = b.x - a.x;
    let mut dy: libc::c_double = b.y - a.y;
    let mut a2: libc::c_double = (p.y - a.y) * dx - (p.x - a.x) * dy;
    a2 *= a2;
    if a2 < 0.0000000001f64 {
        return 0.0f64;
    }
    return a2 / (dx * dx + dy * dy);
}
#[no_mangle]
pub unsafe extern "C" fn line_intersect(
    mut a: pointf,
    mut b: pointf,
    mut c: pointf,
    mut d: pointf,
    mut p: *mut pointf,
) -> libc::c_int {
    let mut mv: pointf = sub_pointf(b, a);
    let mut lv: pointf = sub_pointf(d, c);
    let mut ln: pointf = perp(lv);
    let mut lc: libc::c_double = -(ln.x * c.x + ln.y * c.y);
    let mut dt: libc::c_double = ln.x * mv.x + ln.y * mv.y;
    if fabs(dt) < 0.0000000001f64 {
        return 0 as libc::c_int;
    }
    *p = sub_pointf(a, scale((ln.x * a.x + ln.y * a.y + lc) / dt, mv));
    return 1 as libc::c_int;
}
