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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pxy_t {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type Ppoint_t = Pxy_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ppoly_t {
    pub ps: *mut Ppoint_t,
    pub pn: libc::c_int,
}
pub type Ppolyline_t = Ppoly_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pointnlink_t {
    pub pp: *mut Ppoint_t,
    pub link: *mut pointnlink_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct triangle_t {
    pub mark: libc::c_int,
    pub e: [tedge_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tedge_t {
    pub pnl0p: *mut pointnlink_t,
    pub pnl1p: *mut pointnlink_t,
    pub ltp: *mut triangle_t,
    pub rtp: *mut triangle_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deque_t {
    pub pnlps: *mut *mut pointnlink_t,
    pub pnlpn: libc::c_int,
    pub fpnlpi: libc::c_int,
    pub lpnlpi: libc::c_int,
    pub apex: libc::c_int,
}
static mut pnls: *mut pointnlink_t = 0 as *const pointnlink_t as *mut pointnlink_t;
static mut pnlps: *mut *mut pointnlink_t = 0 as *const *mut pointnlink_t as *mut *mut pointnlink_t;
static mut pnln: libc::c_int = 0;
static mut pnll: libc::c_int = 0;
static mut tris: *mut triangle_t = 0 as *const triangle_t as *mut triangle_t;
static mut trin: libc::c_int = 0;
static mut tril: libc::c_int = 0;
static mut dq: deque_t = deque_t {
    pnlps: 0 as *const *mut pointnlink_t as *mut *mut pointnlink_t,
    pnlpn: 0,
    fpnlpi: 0,
    lpnlpi: 0,
    apex: 0,
};
static mut ops: *mut Ppoint_t = 0 as *const Ppoint_t as *mut Ppoint_t;
static mut opn: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Pshortestpath(
    mut polyp: *mut Ppoly_t,
    mut eps: *mut Ppoint_t,
    mut output: *mut Ppolyline_t,
) -> libc::c_int {
    let mut pi: libc::c_int = 0;
    let mut minpi: libc::c_int = 0;
    let mut minx: libc::c_double = 0.;
    let mut p1: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut p2: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut p3: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut trii: libc::c_int = 0;
    let mut trij: libc::c_int = 0;
    let mut ftrii: libc::c_int = 0;
    let mut ltrii: libc::c_int = 0;
    let mut ei: libc::c_int = 0;
    let mut epnls: [pointnlink_t; 2] = [pointnlink_t {
        pp: 0 as *mut Ppoint_t,
        link: 0 as *mut pointnlink_t,
    }; 2];
    let mut lpnlp: *mut pointnlink_t = 0 as *mut pointnlink_t;
    let mut rpnlp: *mut pointnlink_t = 0 as *mut pointnlink_t;
    let mut pnlp: *mut pointnlink_t = 0 as *mut pointnlink_t;
    let mut trip: *mut triangle_t = 0 as *mut triangle_t;
    let mut splitindex: libc::c_int = 0;
    if growpnls((*polyp).pn) != 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    pnll = 0 as libc::c_int;
    tril = 0 as libc::c_int;
    if growdq((*polyp).pn * 2 as libc::c_int) != 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    dq.fpnlpi = dq.pnlpn / 2 as libc::c_int;
    dq.lpnlpi = dq.fpnlpi - 1 as libc::c_int;
    pi = 0 as libc::c_int;
    minx = ::std::f64::INFINITY;
    minpi = -(1 as libc::c_int);
    while pi < (*polyp).pn {
        if minx > (*((*polyp).ps).offset(pi as isize)).x {
            minx = (*((*polyp).ps).offset(pi as isize)).x;
            minpi = pi;
        }
        pi += 1;
    }
    p2 = *((*polyp).ps).offset(minpi as isize);
    p1 = *((*polyp).ps).offset(
        (if minpi == 0 as libc::c_int {
            (*polyp).pn - 1 as libc::c_int
        } else {
            minpi - 1 as libc::c_int
        }) as isize,
    );
    p3 = *((*polyp).ps).offset(
        (if minpi == (*polyp).pn - 1 as libc::c_int {
            0 as libc::c_int
        } else {
            minpi + 1 as libc::c_int
        }) as isize,
    );
    if p1.x == p2.x && p2.x == p3.x && p3.y > p2.y
        || ccw(&mut p1, &mut p2, &mut p3) != 1 as libc::c_int
    {
        pi = (*polyp).pn - 1 as libc::c_int;
        while pi >= 0 as libc::c_int {
            if !(pi < (*polyp).pn - 1 as libc::c_int
                && (*((*polyp).ps).offset(pi as isize)).x
                    == (*((*polyp).ps).offset((pi + 1 as libc::c_int) as isize)).x
                && (*((*polyp).ps).offset(pi as isize)).y
                    == (*((*polyp).ps).offset((pi + 1 as libc::c_int) as isize)).y)
            {
                let ref mut fresh0 = (*pnls.offset(pnll as isize)).pp;
                *fresh0 = &mut *((*polyp).ps).offset(pi as isize) as *mut Ppoint_t;
                let ref mut fresh1 = (*pnls.offset(pnll as isize)).link;
                *fresh1 = &mut *pnls.offset((pnll % (*polyp).pn) as isize) as *mut pointnlink_t;
                let ref mut fresh2 = *pnlps.offset(pnll as isize);
                *fresh2 = &mut *pnls.offset(pnll as isize) as *mut pointnlink_t;
                pnll += 1;
            }
            pi -= 1;
        }
    } else {
        pi = 0 as libc::c_int;
        while pi < (*polyp).pn {
            if !(pi > 0 as libc::c_int
                && (*((*polyp).ps).offset(pi as isize)).x
                    == (*((*polyp).ps).offset((pi - 1 as libc::c_int) as isize)).x
                && (*((*polyp).ps).offset(pi as isize)).y
                    == (*((*polyp).ps).offset((pi - 1 as libc::c_int) as isize)).y)
            {
                let ref mut fresh3 = (*pnls.offset(pnll as isize)).pp;
                *fresh3 = &mut *((*polyp).ps).offset(pi as isize) as *mut Ppoint_t;
                let ref mut fresh4 = (*pnls.offset(pnll as isize)).link;
                *fresh4 = &mut *pnls.offset((pnll % (*polyp).pn) as isize) as *mut pointnlink_t;
                let ref mut fresh5 = *pnlps.offset(pnll as isize);
                *fresh5 = &mut *pnls.offset(pnll as isize) as *mut pointnlink_t;
                pnll += 1;
            }
            pi += 1;
        }
    }
    if triangulate(pnlps, pnll) != 0 {
        return -(2 as libc::c_int);
    }
    trii = 0 as libc::c_int;
    while trii < tril {
        trij = trii + 1 as libc::c_int;
        while trij < tril {
            connecttris(trii, trij);
            trij += 1;
        }
        trii += 1;
    }
    trii = 0 as libc::c_int;
    while trii < tril {
        if pointintri(trii, &mut *eps.offset(0 as libc::c_int as isize)) != 0 {
            break;
        }
        trii += 1;
    }
    if trii == tril {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            b"source point not in any triangle\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ftrii = trii;
    trii = 0 as libc::c_int;
    while trii < tril {
        if pointintri(trii, &mut *eps.offset(1 as libc::c_int as isize)) != 0 {
            break;
        }
        trii += 1;
    }
    if trii == tril {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            b"destination point not in any triangle\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ltrii = trii;
    if !marktripath(ftrii, ltrii) {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            b"cannot find triangle path\0" as *const u8 as *const libc::c_char,
        );
        if growops(2 as libc::c_int) != 0 as libc::c_int {
            return -(2 as libc::c_int);
        }
        (*output).pn = 2 as libc::c_int;
        *ops.offset(0 as libc::c_int as isize) = *eps.offset(0 as libc::c_int as isize);
        *ops.offset(1 as libc::c_int as isize) = *eps.offset(1 as libc::c_int as isize);
        let ref mut fresh6 = (*output).ps;
        *fresh6 = ops;
        return 0 as libc::c_int;
    }
    if ftrii == ltrii {
        if growops(2 as libc::c_int) != 0 as libc::c_int {
            return -(2 as libc::c_int);
        }
        (*output).pn = 2 as libc::c_int;
        *ops.offset(0 as libc::c_int as isize) = *eps.offset(0 as libc::c_int as isize);
        *ops.offset(1 as libc::c_int as isize) = *eps.offset(1 as libc::c_int as isize);
        let ref mut fresh7 = (*output).ps;
        *fresh7 = ops;
        return 0 as libc::c_int;
    }
    epnls[0 as libc::c_int as usize].pp =
        &mut *eps.offset(0 as libc::c_int as isize) as *mut Ppoint_t;
    epnls[0 as libc::c_int as usize].link = 0 as *mut pointnlink_t;
    epnls[1 as libc::c_int as usize].pp =
        &mut *eps.offset(1 as libc::c_int as isize) as *mut Ppoint_t;
    epnls[1 as libc::c_int as usize].link = 0 as *mut pointnlink_t;
    add2dq(
        1 as libc::c_int,
        &mut *epnls.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    dq.apex = dq.fpnlpi;
    trii = ftrii;
    while trii != -(1 as libc::c_int) {
        trip = &mut *tris.offset(trii as isize) as *mut triangle_t;
        (*trip).mark = 2 as libc::c_int;
        ei = 0 as libc::c_int;
        while ei < 3 as libc::c_int {
            if !((*trip).e[ei as usize].rtp).is_null()
                && (*(*trip).e[ei as usize].rtp).mark == 1 as libc::c_int
            {
                break;
            }
            ei += 1;
        }
        if ei == 3 as libc::c_int {
            if ccw(
                &mut *eps.offset(1 as libc::c_int as isize),
                (**(dq.pnlps).offset(dq.fpnlpi as isize)).pp,
                (**(dq.pnlps).offset(dq.lpnlpi as isize)).pp,
            ) == 1 as libc::c_int
            {
                lpnlp = *(dq.pnlps).offset(dq.lpnlpi as isize);
                rpnlp =
                    &mut *epnls.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut pointnlink_t;
            } else {
                lpnlp =
                    &mut *epnls.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut pointnlink_t;
                rpnlp = *(dq.pnlps).offset(dq.lpnlpi as isize);
            }
        } else {
            pnlp = (*trip).e[((ei + 1 as libc::c_int) % 3 as libc::c_int) as usize].pnl1p;
            if ccw(
                (*(*trip).e[ei as usize].pnl0p).pp,
                (*pnlp).pp,
                (*(*trip).e[ei as usize].pnl1p).pp,
            ) == 1 as libc::c_int
            {
                lpnlp = (*trip).e[ei as usize].pnl1p;
                rpnlp = (*trip).e[ei as usize].pnl0p;
            } else {
                lpnlp = (*trip).e[ei as usize].pnl0p;
                rpnlp = (*trip).e[ei as usize].pnl1p;
            }
        }
        if trii == ftrii {
            add2dq(2 as libc::c_int, lpnlp);
            add2dq(1 as libc::c_int, rpnlp);
        } else if *(dq.pnlps).offset(dq.fpnlpi as isize) != rpnlp
            && *(dq.pnlps).offset(dq.lpnlpi as isize) != rpnlp
        {
            splitindex = finddqsplit(rpnlp);
            splitdq(2 as libc::c_int, splitindex);
            add2dq(1 as libc::c_int, rpnlp);
            if splitindex > dq.apex {
                dq.apex = splitindex;
            }
        } else {
            splitindex = finddqsplit(lpnlp);
            splitdq(1 as libc::c_int, splitindex);
            add2dq(2 as libc::c_int, lpnlp);
            if splitindex < dq.apex {
                dq.apex = splitindex;
            }
        }
        trii = -(1 as libc::c_int);
        ei = 0 as libc::c_int;
        while ei < 3 as libc::c_int {
            if !((*trip).e[ei as usize].rtp).is_null()
                && (*(*trip).e[ei as usize].rtp).mark == 1 as libc::c_int
            {
                trii =
                    ((*trip).e[ei as usize].rtp).offset_from(tris) as libc::c_long as libc::c_int;
                break;
            } else {
                ei += 1;
            }
        }
    }
    pi = 0 as libc::c_int;
    pnlp = &mut *epnls.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut pointnlink_t;
    while !pnlp.is_null() {
        pi += 1;
        pnlp = (*pnlp).link;
    }
    if growops(pi) != 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    (*output).pn = pi;
    pi = pi - 1 as libc::c_int;
    pnlp = &mut *epnls.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut pointnlink_t;
    while !pnlp.is_null() {
        *ops.offset(pi as isize) = *(*pnlp).pp;
        pi -= 1;
        pnlp = (*pnlp).link;
    }
    let ref mut fresh8 = (*output).ps;
    *fresh8 = ops;
    return 0 as libc::c_int;
}
unsafe extern "C" fn triangulate(
    mut pnlps_0: *mut *mut pointnlink_t,
    mut pnln_0: libc::c_int,
) -> libc::c_int {
    let mut pnli: libc::c_int = 0;
    let mut pnlip1: libc::c_int = 0;
    let mut pnlip2: libc::c_int = 0;
    if pnln_0 > 3 as libc::c_int {
        pnli = 0 as libc::c_int;
        while pnli < pnln_0 {
            pnlip1 = (pnli + 1 as libc::c_int) % pnln_0;
            pnlip2 = (pnli + 2 as libc::c_int) % pnln_0;
            if isdiagonal(pnli, pnlip2, pnlps_0, pnln_0) {
                if loadtriangle(
                    *pnlps_0.offset(pnli as isize),
                    *pnlps_0.offset(pnlip1 as isize),
                    *pnlps_0.offset(pnlip2 as isize),
                ) != 0 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
                pnli = pnlip1;
                while pnli < pnln_0 - 1 as libc::c_int {
                    let ref mut fresh9 = *pnlps_0.offset(pnli as isize);
                    *fresh9 = *pnlps_0.offset((pnli + 1 as libc::c_int) as isize);
                    pnli += 1;
                }
                return triangulate(pnlps_0, pnln_0 - 1 as libc::c_int);
            }
            pnli += 1;
        }
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
            b"triangulation failed\0" as *const u8 as *const libc::c_char,
        );
    } else if loadtriangle(
        *pnlps_0.offset(0 as libc::c_int as isize),
        *pnlps_0.offset(1 as libc::c_int as isize),
        *pnlps_0.offset(2 as libc::c_int as isize),
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn isdiagonal(
    mut pnli: libc::c_int,
    mut pnlip2: libc::c_int,
    mut pnlps_0: *mut *mut pointnlink_t,
    mut pnln_0: libc::c_int,
) -> bool {
    let mut pnlip1: libc::c_int = 0;
    let mut pnlim1: libc::c_int = 0;
    let mut pnlj: libc::c_int = 0;
    let mut pnljp1: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    pnlip1 = (pnli + 1 as libc::c_int) % pnln_0;
    pnlim1 = (pnli + pnln_0 - 1 as libc::c_int) % pnln_0;
    if ccw(
        (**pnlps_0.offset(pnlim1 as isize)).pp,
        (**pnlps_0.offset(pnli as isize)).pp,
        (**pnlps_0.offset(pnlip1 as isize)).pp,
    ) == 1 as libc::c_int
    {
        res = (ccw(
            (**pnlps_0.offset(pnli as isize)).pp,
            (**pnlps_0.offset(pnlip2 as isize)).pp,
            (**pnlps_0.offset(pnlim1 as isize)).pp,
        ) == 1 as libc::c_int
            && ccw(
                (**pnlps_0.offset(pnlip2 as isize)).pp,
                (**pnlps_0.offset(pnli as isize)).pp,
                (**pnlps_0.offset(pnlip1 as isize)).pp,
            ) == 1 as libc::c_int) as libc::c_int;
    } else {
        res = (ccw(
            (**pnlps_0.offset(pnli as isize)).pp,
            (**pnlps_0.offset(pnlip2 as isize)).pp,
            (**pnlps_0.offset(pnlip1 as isize)).pp,
        ) == 2 as libc::c_int) as libc::c_int;
    }
    if res == 0 {
        return 0 as libc::c_int != 0;
    }
    pnlj = 0 as libc::c_int;
    while pnlj < pnln_0 {
        pnljp1 = (pnlj + 1 as libc::c_int) % pnln_0;
        if !(pnlj == pnli || pnljp1 == pnli || pnlj == pnlip2 || pnljp1 == pnlip2) {
            if intersects(
                (**pnlps_0.offset(pnli as isize)).pp,
                (**pnlps_0.offset(pnlip2 as isize)).pp,
                (**pnlps_0.offset(pnlj as isize)).pp,
                (**pnlps_0.offset(pnljp1 as isize)).pp,
            ) {
                return 0 as libc::c_int != 0;
            }
        }
        pnlj += 1;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn loadtriangle(
    mut pnlap: *mut pointnlink_t,
    mut pnlbp: *mut pointnlink_t,
    mut pnlcp: *mut pointnlink_t,
) -> libc::c_int {
    let mut trip: *mut triangle_t = 0 as *mut triangle_t;
    let mut ei: libc::c_int = 0;
    if tril >= trin {
        if growtris(trin + 20 as libc::c_int) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    let fresh10 = tril;
    tril = tril + 1;
    trip = &mut *tris.offset(fresh10 as isize) as *mut triangle_t;
    (*trip).mark = 0 as libc::c_int;
    let ref mut fresh11 = (*trip).e[0 as libc::c_int as usize].pnl0p;
    *fresh11 = pnlap;
    let ref mut fresh12 = (*trip).e[0 as libc::c_int as usize].pnl1p;
    *fresh12 = pnlbp;
    let ref mut fresh13 = (*trip).e[0 as libc::c_int as usize].rtp;
    *fresh13 = 0 as *mut triangle_t;
    let ref mut fresh14 = (*trip).e[1 as libc::c_int as usize].pnl0p;
    *fresh14 = pnlbp;
    let ref mut fresh15 = (*trip).e[1 as libc::c_int as usize].pnl1p;
    *fresh15 = pnlcp;
    let ref mut fresh16 = (*trip).e[1 as libc::c_int as usize].rtp;
    *fresh16 = 0 as *mut triangle_t;
    let ref mut fresh17 = (*trip).e[2 as libc::c_int as usize].pnl0p;
    *fresh17 = pnlcp;
    let ref mut fresh18 = (*trip).e[2 as libc::c_int as usize].pnl1p;
    *fresh18 = pnlap;
    let ref mut fresh19 = (*trip).e[2 as libc::c_int as usize].rtp;
    *fresh19 = 0 as *mut triangle_t;
    ei = 0 as libc::c_int;
    while ei < 3 as libc::c_int {
        let ref mut fresh20 = (*trip).e[ei as usize].ltp;
        *fresh20 = trip;
        ei += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn connecttris(mut tri1: libc::c_int, mut tri2: libc::c_int) {
    let mut tri1p: *mut triangle_t = 0 as *mut triangle_t;
    let mut tri2p: *mut triangle_t = 0 as *mut triangle_t;
    let mut ei: libc::c_int = 0;
    let mut ej: libc::c_int = 0;
    ei = 0 as libc::c_int;
    while ei < 3 as libc::c_int {
        ej = 0 as libc::c_int;
        while ej < 3 as libc::c_int {
            tri1p = &mut *tris.offset(tri1 as isize) as *mut triangle_t;
            tri2p = &mut *tris.offset(tri2 as isize) as *mut triangle_t;
            if (*(*tri1p).e[ei as usize].pnl0p).pp == (*(*tri2p).e[ej as usize].pnl0p).pp
                && (*(*tri1p).e[ei as usize].pnl1p).pp == (*(*tri2p).e[ej as usize].pnl1p).pp
                || (*(*tri1p).e[ei as usize].pnl0p).pp == (*(*tri2p).e[ej as usize].pnl1p).pp
                    && (*(*tri1p).e[ei as usize].pnl1p).pp == (*(*tri2p).e[ej as usize].pnl0p).pp
            {
                let ref mut fresh21 = (*tri1p).e[ei as usize].rtp;
                *fresh21 = tri2p;
                let ref mut fresh22 = (*tri2p).e[ej as usize].rtp;
                *fresh22 = tri1p;
            }
            ej += 1;
        }
        ei += 1;
    }
}
unsafe extern "C" fn marktripath(mut trii: libc::c_int, mut trij: libc::c_int) -> bool {
    let mut ei: libc::c_int = 0;
    if (*tris.offset(trii as isize)).mark != 0 {
        return 0 as libc::c_int != 0;
    }
    (*tris.offset(trii as isize)).mark = 1 as libc::c_int;
    if trii == trij {
        return 1 as libc::c_int != 0;
    }
    ei = 0 as libc::c_int;
    while ei < 3 as libc::c_int {
        if !((*tris.offset(trii as isize)).e[ei as usize].rtp).is_null()
            && marktripath(
                ((*tris.offset(trii as isize)).e[ei as usize].rtp).offset_from(tris) as libc::c_long
                    as libc::c_int,
                trij,
            ) as libc::c_int
                != 0
        {
            return 1 as libc::c_int != 0;
        }
        ei += 1;
    }
    (*tris.offset(trii as isize)).mark = 0 as libc::c_int;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn add2dq(mut side: libc::c_int, mut pnlp: *mut pointnlink_t) {
    if side == 1 as libc::c_int {
        if dq.lpnlpi - dq.fpnlpi >= 0 as libc::c_int {
            let ref mut fresh23 = (*pnlp).link;
            *fresh23 = *(dq.pnlps).offset(dq.fpnlpi as isize);
        }
        dq.fpnlpi -= 1;
        let ref mut fresh24 = *(dq.pnlps).offset(dq.fpnlpi as isize);
        *fresh24 = pnlp;
    } else {
        if dq.lpnlpi - dq.fpnlpi >= 0 as libc::c_int {
            let ref mut fresh25 = (*pnlp).link;
            *fresh25 = *(dq.pnlps).offset(dq.lpnlpi as isize);
        }
        dq.lpnlpi += 1;
        let ref mut fresh26 = *(dq.pnlps).offset(dq.lpnlpi as isize);
        *fresh26 = pnlp;
    };
}
unsafe extern "C" fn splitdq(mut side: libc::c_int, mut index: libc::c_int) {
    if side == 1 as libc::c_int {
        dq.lpnlpi = index;
    } else {
        dq.fpnlpi = index;
    };
}
unsafe extern "C" fn finddqsplit(mut pnlp: *mut pointnlink_t) -> libc::c_int {
    let mut index: libc::c_int = 0;
    index = dq.fpnlpi;
    while index < dq.apex {
        if ccw(
            (**(dq.pnlps).offset((index + 1 as libc::c_int) as isize)).pp,
            (**(dq.pnlps).offset(index as isize)).pp,
            (*pnlp).pp,
        ) == 1 as libc::c_int
        {
            return index;
        }
        index += 1;
    }
    index = dq.lpnlpi;
    while index > dq.apex {
        if ccw(
            (**(dq.pnlps).offset((index - 1 as libc::c_int) as isize)).pp,
            (**(dq.pnlps).offset(index as isize)).pp,
            (*pnlp).pp,
        ) == 2 as libc::c_int
        {
            return index;
        }
        index -= 1;
    }
    return dq.apex;
}
unsafe extern "C" fn ccw(
    mut p1p: *mut Ppoint_t,
    mut p2p: *mut Ppoint_t,
    mut p3p: *mut Ppoint_t,
) -> libc::c_int {
    let mut d: libc::c_double = 0.;
    d = ((*p1p).y - (*p2p).y) * ((*p3p).x - (*p2p).x)
        - ((*p3p).y - (*p2p).y) * ((*p1p).x - (*p2p).x);
    return if d > 0 as libc::c_int as libc::c_double {
        1 as libc::c_int
    } else if d < 0 as libc::c_int as libc::c_double {
        2 as libc::c_int
    } else {
        3 as libc::c_int
    };
}
unsafe extern "C" fn intersects(
    mut pap: *mut Ppoint_t,
    mut pbp: *mut Ppoint_t,
    mut pcp: *mut Ppoint_t,
    mut pdp: *mut Ppoint_t,
) -> bool {
    let mut ccw1: libc::c_int = 0;
    let mut ccw2: libc::c_int = 0;
    let mut ccw3: libc::c_int = 0;
    let mut ccw4: libc::c_int = 0;
    if ccw(pap, pbp, pcp) == 3 as libc::c_int
        || ccw(pap, pbp, pdp) == 3 as libc::c_int
        || ccw(pcp, pdp, pap) == 3 as libc::c_int
        || ccw(pcp, pdp, pbp) == 3 as libc::c_int
    {
        if between(pap, pbp, pcp) as libc::c_int != 0
            || between(pap, pbp, pdp) as libc::c_int != 0
            || between(pcp, pdp, pap) as libc::c_int != 0
            || between(pcp, pdp, pbp) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
    } else {
        ccw1 = if ccw(pap, pbp, pcp) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        ccw2 = if ccw(pap, pbp, pdp) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        ccw3 = if ccw(pcp, pdp, pap) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        ccw4 = if ccw(pcp, pdp, pbp) == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        return ccw1 ^ ccw2 != 0 && ccw3 ^ ccw4 != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn between(
    mut pap: *mut Ppoint_t,
    mut pbp: *mut Ppoint_t,
    mut pcp: *mut Ppoint_t,
) -> bool {
    let mut p1: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    let mut p2: Ppoint_t = Ppoint_t { x: 0., y: 0. };
    p1.x = (*pbp).x - (*pap).x;
    p1.y = (*pbp).y - (*pap).y;
    p2.x = (*pcp).x - (*pap).x;
    p2.y = (*pcp).y - (*pap).y;
    if ccw(pap, pbp, pcp) != 3 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return p2.x * p1.x + p2.y * p1.y >= 0 as libc::c_int as libc::c_double
        && p2.x * p2.x + p2.y * p2.y <= p1.x * p1.x + p1.y * p1.y;
}
unsafe extern "C" fn pointintri(mut trii: libc::c_int, mut pp: *mut Ppoint_t) -> libc::c_int {
    let mut ei: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    ei = 0 as libc::c_int;
    sum = 0 as libc::c_int;
    while ei < 3 as libc::c_int {
        if ccw(
            (*(*tris.offset(trii as isize)).e[ei as usize].pnl0p).pp,
            (*(*tris.offset(trii as isize)).e[ei as usize].pnl1p).pp,
            pp,
        ) != 2 as libc::c_int
        {
            sum += 1;
        }
        ei += 1;
    }
    return (sum == 3 as libc::c_int || sum == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn growpnls(mut newpnln: libc::c_int) -> libc::c_int {
    if newpnln <= pnln {
        return 0 as libc::c_int;
    }
    pnls = realloc(
        pnls as *mut libc::c_void,
        (::std::mem::size_of::<pointnlink_t>() as libc::c_ulong)
            .wrapping_mul(newpnln as libc::c_ulong),
    ) as *mut pointnlink_t;
    if pnls.is_null() {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            507 as libc::c_int,
            b"cannot realloc pnls\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    pnlps = realloc(
        pnlps as *mut libc::c_void,
        (::std::mem::size_of::<*mut pointnlink_t>() as libc::c_ulong)
            .wrapping_mul(newpnln as libc::c_ulong),
    ) as *mut *mut pointnlink_t;
    if pnlps.is_null() {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            512 as libc::c_int,
            b"cannot realloc pnlps\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    pnln = newpnln;
    return 0 as libc::c_int;
}
unsafe extern "C" fn growtris(mut newtrin: libc::c_int) -> libc::c_int {
    if newtrin <= trin {
        return 0 as libc::c_int;
    }
    tris = realloc(
        tris as *mut libc::c_void,
        (::std::mem::size_of::<triangle_t>() as libc::c_ulong)
            .wrapping_mul(newtrin as libc::c_ulong),
    ) as *mut triangle_t;
    if tris.is_null() {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            525 as libc::c_int,
            b"cannot realloc tris\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    trin = newtrin;
    return 0 as libc::c_int;
}
unsafe extern "C" fn growdq(mut newdqn: libc::c_int) -> libc::c_int {
    if newdqn <= dq.pnlpn {
        return 0 as libc::c_int;
    }
    dq.pnlps = realloc(
        dq.pnlps as *mut libc::c_void,
        (::std::mem::size_of::<*mut pointnlink_t>() as libc::c_ulong)
            .wrapping_mul(newdqn as libc::c_ulong),
    ) as *mut *mut pointnlink_t;
    if (dq.pnlps).is_null() {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            539 as libc::c_int,
            b"cannot realloc dq.pnls\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dq.pnlpn = newdqn;
    return 0 as libc::c_int;
}
unsafe extern "C" fn growops(mut newopn: libc::c_int) -> libc::c_int {
    if newopn <= opn {
        return 0 as libc::c_int;
    }
    ops = realloc(
        ops as *mut libc::c_void,
        (::std::mem::size_of::<Ppoint_t>() as libc::c_ulong).wrapping_mul(newopn as libc::c_ulong),
    ) as *mut Ppoint_t;
    if ops.is_null() {
        fprintf(
            stderr,
            b"libpath/%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
            b"shortest.c\0" as *const u8 as *const libc::c_char,
            552 as libc::c_int,
            b"cannot realloc ops\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    opn = newopn;
    return 0 as libc::c_int;
}
