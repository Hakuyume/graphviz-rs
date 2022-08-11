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
    fn gcalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    static mut Verbose: libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rectangle_struct {
    pub x: [libc::c_double; 2],
    pub size: [libc::c_double; 2],
}
pub type rectangle = rectangle_struct;
unsafe extern "C" fn squarify(
    mut n: libc::c_int,
    mut area: *mut libc::c_double,
    mut recs: *mut rectangle,
    mut nadded: libc::c_int,
    mut maxarea: libc::c_double,
    mut minarea: libc::c_double,
    mut totalarea: libc::c_double,
    mut asp: libc::c_double,
    mut fillrec: rectangle,
) {
    let mut w: libc::c_double =
        if fillrec.size[0 as libc::c_int as usize] < fillrec.size[1 as libc::c_int as usize] {
            fillrec.size[0 as libc::c_int as usize]
        } else {
            fillrec.size[1 as libc::c_int as usize]
        };
    let mut i: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        return;
    }
    if Verbose != 0 {
        fprintf(
            stderr,
            b"trying to add to rect {%f +/- %f, %f +/- %f}\n\0" as *const u8 as *const libc::c_char,
            fillrec.x[0 as libc::c_int as usize],
            fillrec.size[0 as libc::c_int as usize],
            fillrec.x[1 as libc::c_int as usize],
            fillrec.size[1 as libc::c_int as usize],
        );
        fprintf(
            stderr,
            b"total added so far = %d\n\0" as *const u8 as *const libc::c_char,
            nadded,
        );
    }
    if nadded == 0 as libc::c_int {
        nadded = 1 as libc::c_int;
        minarea = *area.offset(0 as libc::c_int as isize);
        maxarea = minarea;
        asp = if *area.offset(0 as libc::c_int as isize) / (w * w)
            > w * w / *area.offset(0 as libc::c_int as isize)
        {
            *area.offset(0 as libc::c_int as isize) / (w * w)
        } else {
            w * w / *area.offset(0 as libc::c_int as isize)
        };
        totalarea = *area.offset(0 as libc::c_int as isize);
        squarify(
            n, area, recs, nadded, maxarea, minarea, totalarea, asp, fillrec,
        );
    } else {
        let mut newmaxarea: libc::c_double = 0.;
        let mut newminarea: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        let mut h: libc::c_double = 0.;
        let mut maxw: libc::c_double = 0.;
        let mut minw: libc::c_double = 0.;
        let mut newasp: libc::c_double = 0.;
        let mut hh: libc::c_double = 0.;
        let mut ww: libc::c_double = 0.;
        let mut xx: libc::c_double = 0.;
        let mut yy: libc::c_double = 0.;
        if nadded < n {
            newmaxarea = if maxarea > *area.offset(nadded as isize) {
                maxarea
            } else {
                *area.offset(nadded as isize)
            };
            newminarea = if minarea < *area.offset(nadded as isize) {
                minarea
            } else {
                *area.offset(nadded as isize)
            };
            s = totalarea + *area.offset(nadded as isize);
            h = s / w;
            maxw = newmaxarea / h;
            minw = newminarea / h;
            newasp = if h / minw > maxw / h {
                h / minw
            } else {
                maxw / h
            };
        }
        if nadded < n && newasp <= asp {
            nadded += 1;
            squarify(
                n, area, recs, nadded, newmaxarea, newminarea, s, newasp, fillrec,
            );
        } else {
            if Verbose != 0 {
                fprintf(
                    stderr,
                    b"adding %d items, total area = %f, w = %f, area/w=%f\n\0" as *const u8
                        as *const libc::c_char,
                    nadded,
                    totalarea,
                    w,
                    totalarea / w,
                );
            }
            if fillrec.size[0 as libc::c_int as usize] <= fillrec.size[1 as libc::c_int as usize] {
                hh = totalarea / w;
                xx = fillrec.x[0 as libc::c_int as usize]
                    - fillrec.size[0 as libc::c_int as usize] / 2 as libc::c_int as libc::c_double;
                i = 0 as libc::c_int;
                while i < nadded {
                    (*recs.offset(i as isize)).size[1 as libc::c_int as usize] = hh;
                    ww = *area.offset(i as isize) / hh;
                    (*recs.offset(i as isize)).size[0 as libc::c_int as usize] = ww;
                    (*recs.offset(i as isize)).x[1 as libc::c_int as usize] = fillrec.x
                        [1 as libc::c_int as usize]
                        + 0.5f64 * fillrec.size[1 as libc::c_int as usize]
                        - hh / 2 as libc::c_int as libc::c_double;
                    (*recs.offset(i as isize)).x[0 as libc::c_int as usize] =
                        xx + ww / 2 as libc::c_int as libc::c_double;
                    xx += ww;
                    i += 1;
                }
                fillrec.x[1 as libc::c_int as usize] -= hh / 2 as libc::c_int as libc::c_double;
                fillrec.size[1 as libc::c_int as usize] -= hh;
            } else {
                ww = totalarea / w;
                yy = fillrec.x[1 as libc::c_int as usize]
                    + fillrec.size[1 as libc::c_int as usize] / 2 as libc::c_int as libc::c_double;
                i = 0 as libc::c_int;
                while i < nadded {
                    (*recs.offset(i as isize)).size[0 as libc::c_int as usize] = ww;
                    hh = *area.offset(i as isize) / ww;
                    (*recs.offset(i as isize)).size[1 as libc::c_int as usize] = hh;
                    (*recs.offset(i as isize)).x[0 as libc::c_int as usize] = fillrec.x
                        [0 as libc::c_int as usize]
                        - 0.5f64 * fillrec.size[0 as libc::c_int as usize]
                        + ww / 2 as libc::c_int as libc::c_double;
                    (*recs.offset(i as isize)).x[1 as libc::c_int as usize] =
                        yy - hh / 2 as libc::c_int as libc::c_double;
                    yy -= hh;
                    i += 1;
                }
                fillrec.x[0 as libc::c_int as usize] += ww / 2 as libc::c_int as libc::c_double;
                fillrec.size[0 as libc::c_int as usize] -= ww;
            }
            squarify(
                n - nadded,
                area.offset(nadded as isize),
                recs.offset(nadded as isize),
                0 as libc::c_int,
                0.0f64,
                0.0f64,
                0.0f64,
                1.0f64,
                fillrec,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tree_map(
    mut n: libc::c_int,
    mut area: *mut libc::c_double,
    mut fillrec: rectangle,
) -> *mut rectangle {
    let mut recs: *mut rectangle = 0 as *mut rectangle;
    let mut i: libc::c_int = 0;
    let mut total: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut minarea: libc::c_double = 1.0f64;
    let mut maxarea: libc::c_double = 0.0f64;
    let mut asp: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut totalarea: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut nadded: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        total += *area.offset(i as isize);
        i += 1;
    }
    if total
        > fillrec.size[0 as libc::c_int as usize] * fillrec.size[1 as libc::c_int as usize]
            + 0.001f64
    {
        return 0 as *mut rectangle;
    }
    recs = gcalloc(
        n as size_t,
        ::std::mem::size_of::<rectangle>() as libc::c_ulong,
    ) as *mut rectangle;
    squarify(
        n, area, recs, nadded, maxarea, minarea, totalarea, asp, fillrec,
    );
    return recs;
}
#[no_mangle]
pub unsafe extern "C" fn rectangle_new(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut width: libc::c_double,
    mut height: libc::c_double,
) -> rectangle {
    let mut r: rectangle = rectangle {
        x: [0.; 2],
        size: [0.; 2],
    };
    r.x[0 as libc::c_int as usize] = x;
    r.x[1 as libc::c_int as usize] = y;
    r.size[0 as libc::c_int as usize] = width;
    r.size[1 as libc::c_int as usize] = height;
    return r;
}
