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
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
pub type xdot_grad_type = libc::c_uint;
pub const xd_radial: xdot_grad_type = 2;
pub const xd_linear: xdot_grad_type = 1;
pub const xd_none: xdot_grad_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_color_stop {
    pub frac: libc::c_float,
    pub color: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_linear_grad {
    pub x0: libc::c_double,
    pub y0: libc::c_double,
    pub x1: libc::c_double,
    pub y1: libc::c_double,
    pub n_stops: libc::c_int,
    pub stops: *mut xdot_color_stop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_radial_grad {
    pub x0: libc::c_double,
    pub y0: libc::c_double,
    pub r0: libc::c_double,
    pub x1: libc::c_double,
    pub y1: libc::c_double,
    pub r1: libc::c_double,
    pub n_stops: libc::c_int,
    pub stops: *mut xdot_color_stop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_color {
    pub type_0: xdot_grad_type,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub clr: *mut libc::c_char,
    pub ling: xdot_linear_grad,
    pub ring: xdot_radial_grad,
}
pub type xdot_align = libc::c_uint;
pub const xd_right: xdot_align = 2;
pub const xd_center: xdot_align = 1;
pub const xd_left: xdot_align = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_rect {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub w: libc::c_double,
    pub h: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_polyline {
    pub cnt: libc::c_int,
    pub pts: *mut xdot_point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_text {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub align: xdot_align,
    pub width: libc::c_double,
    pub text: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_image {
    pub pos: xdot_rect,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_font {
    pub size: libc::c_double,
    pub name: *mut libc::c_char,
}
pub type xdot_kind = libc::c_uint;
pub const xd_fontchar: xdot_kind = 15;
pub const xd_grad_pen_color: xdot_kind = 14;
pub const xd_grad_fill_color: xdot_kind = 13;
pub const xd_image: xdot_kind = 12;
pub const xd_style: xdot_kind = 11;
pub const xd_font: xdot_kind = 10;
pub const xd_pen_color: xdot_kind = 9;
pub const xd_fill_color: xdot_kind = 8;
pub const xd_text: xdot_kind = 7;
pub const xd_polyline: xdot_kind = 6;
pub const xd_unfilled_bezier: xdot_kind = 5;
pub const xd_filled_bezier: xdot_kind = 4;
pub const xd_unfilled_polygon: xdot_kind = 3;
pub const xd_filled_polygon: xdot_kind = 2;
pub const xd_unfilled_ellipse: xdot_kind = 1;
pub const xd_filled_ellipse: xdot_kind = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const xop_fontchar: C2RustUnnamed_0 = 11;
pub const xop_grad_color: C2RustUnnamed_0 = 10;
pub const xop_image: C2RustUnnamed_0 = 9;
pub const xop_style: C2RustUnnamed_0 = 8;
pub const xop_font: C2RustUnnamed_0 = 7;
pub const xop_pen_color: C2RustUnnamed_0 = 6;
pub const xop_fill_color: C2RustUnnamed_0 = 5;
pub const xop_text: C2RustUnnamed_0 = 4;
pub const xop_polyline: C2RustUnnamed_0 = 3;
pub const xop_bezier: C2RustUnnamed_0 = 2;
pub const xop_polygon: C2RustUnnamed_0 = 1;
pub const xop_ellipse: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xdot_op {
    pub kind: xdot_kind,
    pub u: C2RustUnnamed_1,
    pub drawfunc: drawfunc_t,
}
pub type drawfunc_t = Option<unsafe extern "C" fn(*mut xdot_op, libc::c_int) -> ()>;
pub type xdot_op = _xdot_op;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ellipse: xdot_rect,
    pub polygon: xdot_polyline,
    pub polyline: xdot_polyline,
    pub bezier: xdot_polyline,
    pub text: xdot_text,
    pub image: xdot_image,
    pub color: *mut libc::c_char,
    pub grad_color: xdot_color,
    pub font: xdot_font,
    pub style: *mut libc::c_char,
    pub fontchar: libc::c_uint,
}
pub type freefunc_t = Option<unsafe extern "C" fn(*mut xdot_op) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot {
    pub cnt: libc::c_int,
    pub sz: libc::c_int,
    pub ops: *mut xdot_op,
    pub freefunc: freefunc_t,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdot_stats {
    pub cnt: libc::c_int,
    pub n_ellipse: libc::c_int,
    pub n_polygon: libc::c_int,
    pub n_polygon_pts: libc::c_int,
    pub n_polyline: libc::c_int,
    pub n_polyline_pts: libc::c_int,
    pub n_bezier: libc::c_int,
    pub n_bezier_pts: libc::c_int,
    pub n_text: libc::c_int,
    pub n_font: libc::c_int,
    pub n_style: libc::c_int,
    pub n_color: libc::c_int,
    pub n_image: libc::c_int,
    pub n_gradcolor: libc::c_int,
    pub n_fontchar: libc::c_int,
}
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub type pf = Option<unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()>;
pub type print_op =
    Option<unsafe extern "C" fn(*mut xdot_op, pf, *mut libc::c_void, libc::c_int) -> ()>;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh0 = (*xb).ptr;
    *fresh0 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh1 = (*xb).ptr;
    let fresh2 = *fresh1;
    *fresh1 = (*fresh1).offset(1);
    *fresh2 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbput(mut xb: *mut agxbuf, mut s: *const libc::c_char) -> size_t {
    let mut ssz: size_t = strlen(s);
    return agxbput_n(xb, s, ssz);
}
#[inline]
unsafe extern "C" fn agxbput_n(
    mut xb: *mut agxbuf,
    mut s: *const libc::c_char,
    mut ssz: size_t,
) -> size_t {
    if ((*xb).ptr).offset(ssz as isize) > (*xb).eptr {
        agxbmore(xb, ssz);
    }
    memcpy(
        (*xb).ptr as *mut libc::c_void,
        s as *const libc::c_void,
        ssz,
    );
    let ref mut fresh3 = (*xb).ptr;
    *fresh3 = (*fresh3).offset(ssz as isize);
    return ssz;
}
#[inline]
unsafe extern "C" fn agxbmore(mut xb: *mut agxbuf, mut ssz: size_t) {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut nsize: size_t = 0 as libc::c_int as size_t;
    let mut nbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    size = ((*xb).eptr).offset_from((*xb).buf) as libc::c_long as size_t;
    nsize = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
    if size.wrapping_add(ssz) > nsize {
        nsize = size.wrapping_add(ssz);
    }
    cnt = ((*xb).ptr).offset_from((*xb).buf) as libc::c_long as size_t;
    if (*xb).dyna != 0 {
        nbuf = gv_recalloc(
            (*xb).buf as *mut libc::c_void,
            size,
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        nbuf = gv_calloc(
            nsize,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            nbuf as *mut libc::c_void,
            (*xb).buf as *const libc::c_void,
            cnt,
        );
        (*xb).dyna = 1 as libc::c_int;
    }
    let ref mut fresh4 = (*xb).buf;
    *fresh4 = nbuf;
    let ref mut fresh5 = (*xb).ptr;
    *fresh5 = ((*xb).buf).offset(cnt as isize);
    let ref mut fresh6 = (*xb).eptr;
    *fresh6 = ((*xb).buf).offset(nsize as isize);
}
#[inline]
unsafe extern "C" fn agxbfree(mut xb: *mut agxbuf) {
    if (*xb).dyna != 0 {
        free((*xb).buf as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh7 = (*xb).buf;
        *fresh7 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh8 = (*xb).buf;
        *fresh8 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh9 = (*xb).eptr;
    *fresh9 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn gv_strndup(
    mut original: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    copy = strndup(original, length);
    if (copy == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return copy;
}
#[inline]
unsafe extern "C" fn gv_recalloc(
    mut ptr: *mut libc::c_void,
    mut old_nmemb: size_t,
    mut new_nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size > 0 as libc::c_int as libc::c_ulong
        && !(b"attempt to allocate array of 0-sized elements\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"size > 0 && \"attempt to allocate array of 0-sized elements\"\0" as *const u8
                as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if old_nmemb < (18446744073709551615 as libc::c_ulong).wrapping_div(size)
        && !(b"claimed previous extent is too large\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"old_nmemb < SIZE_MAX / size && \"claimed previous extent is too large\"\0"
                as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/alloc.h\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void *gv_recalloc(void *, size_t, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    if (new_nmemb > (18446744073709551615 as libc::c_ulong).wrapping_div(size)) as libc::c_int
        as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"integer overflow in dynamic memory reallocation\n\0" as *const u8
                as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return gv_realloc(
        ptr,
        old_nmemb.wrapping_mul(size),
        new_nmemb.wrapping_mul(size),
    );
}
#[inline]
unsafe extern "C" fn gv_realloc(
    mut ptr: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, new_size);
    if (new_size > 0 as libc::c_int as libc::c_ulong && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    if new_size > old_size {
        memset(
            (p as *mut libc::c_char).offset(old_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            new_size.wrapping_sub(old_size),
        );
    }
    return p;
}
#[inline]
unsafe extern "C" fn gv_alloc(mut size: size_t) -> *mut libc::c_void {
    return gv_calloc(1 as libc::c_int as size_t, size);
}
#[inline]
unsafe extern "C" fn gv_calloc(mut nmemb: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if (nmemb > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong
        && p.is_null()) as libc::c_int as libc::c_long
        != 0
    {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        graphviz_exit(1 as libc::c_int);
    }
    return p;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
unsafe extern "C" fn parseReal(
    mut s: *mut libc::c_char,
    mut fp: *mut libc::c_double,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: libc::c_double = 0.;
    d = strtod(s, &mut p);
    if p == s {
        return 0 as *mut libc::c_char;
    }
    *fp = d;
    return p;
}
unsafe extern "C" fn parseInt(
    mut s: *mut libc::c_char,
    mut ip: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    *ip = strtol(s, &mut endp, 10 as libc::c_int) as libc::c_int;
    if s == endp {
        return 0 as *mut libc::c_char;
    } else {
        return endp;
    };
}
unsafe extern "C" fn parseUInt(
    mut s: *mut libc::c_char,
    mut ip: *mut libc::c_uint,
) -> *mut libc::c_char {
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    *ip = strtoul(s, &mut endp, 10 as libc::c_int) as libc::c_uint;
    if s == endp {
        return 0 as *mut libc::c_char;
    } else {
        return endp;
    };
}
unsafe extern "C" fn parseRect(
    mut s: *mut libc::c_char,
    mut rp: *mut xdot_rect,
) -> *mut libc::c_char {
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    (*rp).x = strtod(s, &mut endp);
    if s == endp {
        return 0 as *mut libc::c_char;
    } else {
        s = endp;
    }
    (*rp).y = strtod(s, &mut endp);
    if s == endp {
        return 0 as *mut libc::c_char;
    } else {
        s = endp;
    }
    (*rp).w = strtod(s, &mut endp);
    if s == endp {
        return 0 as *mut libc::c_char;
    } else {
        s = endp;
    }
    (*rp).h = strtod(s, &mut endp);
    if s == endp {
        return 0 as *mut libc::c_char;
    } else {
        s = endp;
    }
    return s;
}
unsafe extern "C" fn parsePolyline(
    mut s: *mut libc::c_char,
    mut pp: *mut xdot_polyline,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut pts: *mut xdot_point = 0 as *mut xdot_point;
    let mut ps: *mut xdot_point = 0 as *mut xdot_point;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    s = parseInt(s, &mut i);
    if s.is_null() {
        return s;
    }
    ps = gv_calloc(
        i as size_t,
        ::std::mem::size_of::<xdot_point>() as libc::c_ulong,
    ) as *mut xdot_point;
    pts = ps;
    (*pp).cnt = i;
    i = 0 as libc::c_int;
    while i < (*pp).cnt {
        (*ps).x = strtod(s, &mut endp);
        if s == endp {
            free(pts as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        } else {
            s = endp;
        }
        (*ps).y = strtod(s, &mut endp);
        if s == endp {
            free(pts as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        } else {
            s = endp;
        }
        (*ps).z = 0 as libc::c_int as libc::c_double;
        ps = ps.offset(1);
        i += 1;
    }
    let ref mut fresh11 = (*pp).pts;
    *fresh11 = pts;
    return s;
}
unsafe extern "C" fn parseString(
    mut s: *mut libc::c_char,
    mut sp: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    s = parseInt(s, &mut i);
    if s.is_null() || i <= 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    while *s as libc::c_int != 0 && *s as libc::c_int != '-' as i32 {
        s = s.offset(1);
    }
    if *s != 0 {
        s = s.offset(1);
    } else {
        return 0 as *mut libc::c_char;
    }
    let mut c: *mut libc::c_char = gv_strndup(s, i as size_t);
    if strlen(c) != i as libc::c_ulong {
        free(c as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *sp = c;
    return s.offset(i as isize);
}
unsafe extern "C" fn parseAlign(
    mut s: *mut libc::c_char,
    mut ap: *mut xdot_align,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    s = parseInt(s, &mut i);
    if i < 0 as libc::c_int {
        *ap = xd_left;
    } else if i > 0 as libc::c_int {
        *ap = xd_right;
    } else {
        *ap = xd_center;
    }
    return s;
}
unsafe extern "C" fn parseOp(
    mut op: *mut xdot_op,
    mut s: *mut libc::c_char,
    mut ops: *mut drawfunc_t,
    mut error: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut cs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clr: xdot_color = xdot_color {
        type_0: xd_none,
        u: C2RustUnnamed {
            clr: 0 as *mut libc::c_char,
        },
    };
    *error = 0 as libc::c_int;
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        s = s.offset(1);
    }
    let fresh12 = s;
    s = s.offset(1);
    match *fresh12 as libc::c_int {
        69 => {
            (*op).kind = xd_filled_ellipse;
            s = parseRect(s, &mut (*op).u.ellipse);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh13 = (*op).drawfunc;
                *fresh13 = *ops.offset(xop_ellipse as libc::c_int as isize);
            }
        }
        101 => {
            (*op).kind = xd_unfilled_ellipse;
            s = parseRect(s, &mut (*op).u.ellipse);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh14 = (*op).drawfunc;
                *fresh14 = *ops.offset(xop_ellipse as libc::c_int as isize);
            }
        }
        80 => {
            (*op).kind = xd_filled_polygon;
            s = parsePolyline(s, &mut (*op).u.polygon);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh15 = (*op).drawfunc;
                *fresh15 = *ops.offset(xop_polygon as libc::c_int as isize);
            }
        }
        112 => {
            (*op).kind = xd_unfilled_polygon;
            s = parsePolyline(s, &mut (*op).u.polygon);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh16 = (*op).drawfunc;
                *fresh16 = *ops.offset(xop_polygon as libc::c_int as isize);
            }
        }
        98 => {
            (*op).kind = xd_filled_bezier;
            s = parsePolyline(s, &mut (*op).u.bezier);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh17 = (*op).drawfunc;
                *fresh17 = *ops.offset(xop_bezier as libc::c_int as isize);
            }
        }
        66 => {
            (*op).kind = xd_unfilled_bezier;
            s = parsePolyline(s, &mut (*op).u.bezier);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh18 = (*op).drawfunc;
                *fresh18 = *ops.offset(xop_bezier as libc::c_int as isize);
            }
        }
        99 => {
            s = parseString(s, &mut cs);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            cs = parseXDotColor(cs, &mut clr);
            if cs.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if clr.type_0 as libc::c_uint == xd_none as libc::c_int as libc::c_uint {
                (*op).kind = xd_pen_color;
                let ref mut fresh19 = (*op).u.color;
                *fresh19 = clr.u.clr;
                if !ops.is_null() {
                    let ref mut fresh20 = (*op).drawfunc;
                    *fresh20 = *ops.offset(xop_pen_color as libc::c_int as isize);
                }
            } else {
                (*op).kind = xd_grad_pen_color;
                (*op).u.grad_color = clr;
                if !ops.is_null() {
                    let ref mut fresh21 = (*op).drawfunc;
                    *fresh21 = *ops.offset(xop_grad_color as libc::c_int as isize);
                }
            }
        }
        67 => {
            s = parseString(s, &mut cs);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            cs = parseXDotColor(cs, &mut clr);
            if cs.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if clr.type_0 as libc::c_uint == xd_none as libc::c_int as libc::c_uint {
                (*op).kind = xd_fill_color;
                let ref mut fresh22 = (*op).u.color;
                *fresh22 = clr.u.clr;
                if !ops.is_null() {
                    let ref mut fresh23 = (*op).drawfunc;
                    *fresh23 = *ops.offset(xop_fill_color as libc::c_int as isize);
                }
            } else {
                (*op).kind = xd_grad_fill_color;
                (*op).u.grad_color = clr;
                if !ops.is_null() {
                    let ref mut fresh24 = (*op).drawfunc;
                    *fresh24 = *ops.offset(xop_grad_color as libc::c_int as isize);
                }
            }
        }
        76 => {
            (*op).kind = xd_polyline;
            s = parsePolyline(s, &mut (*op).u.polyline);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh25 = (*op).drawfunc;
                *fresh25 = *ops.offset(xop_polyline as libc::c_int as isize);
            }
        }
        84 => {
            (*op).kind = xd_text;
            s = parseReal(s, &mut (*op).u.text.x);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            s = parseReal(s, &mut (*op).u.text.y);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            s = parseAlign(s, &mut (*op).u.text.align);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            s = parseReal(s, &mut (*op).u.text.width);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            s = parseString(s, &mut (*op).u.text.text);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh26 = (*op).drawfunc;
                *fresh26 = *ops.offset(xop_text as libc::c_int as isize);
            }
        }
        70 => {
            (*op).kind = xd_font;
            s = parseReal(s, &mut (*op).u.font.size);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            s = parseString(s, &mut (*op).u.font.name);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh27 = (*op).drawfunc;
                *fresh27 = *ops.offset(xop_font as libc::c_int as isize);
            }
        }
        83 => {
            (*op).kind = xd_style;
            s = parseString(s, &mut (*op).u.style);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh28 = (*op).drawfunc;
                *fresh28 = *ops.offset(xop_style as libc::c_int as isize);
            }
        }
        73 => {
            (*op).kind = xd_image;
            s = parseRect(s, &mut (*op).u.image.pos);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            s = parseString(s, &mut (*op).u.image.name);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh29 = (*op).drawfunc;
                *fresh29 = *ops.offset(xop_image as libc::c_int as isize);
            }
        }
        116 => {
            (*op).kind = xd_fontchar;
            s = parseUInt(s, &mut (*op).u.fontchar);
            if s.is_null() {
                *error = 1 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            if !ops.is_null() {
                let ref mut fresh30 = (*op).drawfunc;
                *fresh30 = *ops.offset(xop_fontchar as libc::c_int as isize);
            }
        }
        0 => {
            s = 0 as *mut libc::c_char;
        }
        _ => {
            *error = 1 as libc::c_int;
            s = 0 as *mut libc::c_char;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn parseXDotFOn(
    mut s: *mut libc::c_char,
    mut fns: *mut drawfunc_t,
    mut sz: libc::c_int,
    mut x: *mut xdot,
) -> *mut xdot {
    let mut op: xdot_op = xdot_op {
        kind: xd_filled_ellipse,
        u: C2RustUnnamed_1 {
            ellipse: xdot_rect {
                x: 0.,
                y: 0.,
                w: 0.,
                h: 0.,
            },
        },
        drawfunc: None,
    };
    let mut ops: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldsz: libc::c_int = 0;
    let mut bufsz: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut initcnt: libc::c_int = 0;
    if s.is_null() {
        return x;
    }
    if x.is_null() {
        x = gv_alloc(::std::mem::size_of::<xdot>() as libc::c_ulong) as *mut xdot;
        if sz as libc::c_ulong <= ::std::mem::size_of::<xdot_op>() as libc::c_ulong {
            sz = ::std::mem::size_of::<xdot_op>() as libc::c_ulong as libc::c_int;
        }
        (*x).sz = sz;
    }
    initcnt = (*x).cnt;
    sz = (*x).sz;
    if initcnt == 0 as libc::c_int {
        bufsz = 100 as libc::c_int;
        ops = gv_calloc(100 as libc::c_int as size_t, sz as size_t) as *mut libc::c_char;
    } else {
        ops = (*x).ops as *mut libc::c_char;
        bufsz = initcnt + 100 as libc::c_int;
        ops = gv_recalloc(
            ops as *mut libc::c_void,
            initcnt as size_t,
            bufsz as size_t,
            sz as size_t,
        ) as *mut libc::c_char;
    }
    loop {
        s = parseOp(&mut op, s, fns, &mut error);
        if s.is_null() {
            break;
        }
        if (*x).cnt == bufsz {
            oldsz = bufsz;
            bufsz *= 2 as libc::c_int;
            ops = gv_recalloc(
                ops as *mut libc::c_void,
                oldsz as size_t,
                bufsz as size_t,
                sz as size_t,
            ) as *mut libc::c_char;
        }
        *(ops.offset(((*x).cnt * sz) as isize) as *mut xdot_op) = op;
        let ref mut fresh31 = (*x).cnt;
        *fresh31 += 1;
    }
    if error != 0 {
        (*x).flags |= 1 as libc::c_int;
    }
    if (*x).cnt != 0 {
        let ref mut fresh32 = (*x).ops;
        *fresh32 = gv_recalloc(
            ops as *mut libc::c_void,
            bufsz as size_t,
            (*x).cnt as size_t,
            sz as size_t,
        ) as *mut xdot_op;
    } else {
        free(ops as *mut libc::c_void);
        free(x as *mut libc::c_void);
        x = 0 as *mut xdot;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn parseXDotF(
    mut s: *mut libc::c_char,
    mut fns: *mut drawfunc_t,
    mut sz: libc::c_int,
) -> *mut xdot {
    return parseXDotFOn(s, fns, sz, 0 as *mut xdot);
}
#[no_mangle]
pub unsafe extern "C" fn parseXDot(mut s: *mut libc::c_char) -> *mut xdot {
    return parseXDotF(s, 0 as *mut drawfunc_t, 0 as libc::c_int);
}
unsafe extern "C" fn trim(mut buf: *mut libc::c_char) {
    let mut dotp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    dotp = strchr(buf, '.' as i32);
    if !dotp.is_null() {
        p = dotp.offset(1 as libc::c_int as isize);
        while *p != 0 {
            p = p.offset(1);
        }
        p = p.offset(-1);
        while *p as libc::c_int == '0' as i32 {
            let fresh33 = p;
            p = p.offset(-1);
            *fresh33 = '\0' as i32 as libc::c_char;
        }
        if *p as libc::c_int == '.' as i32 {
            *p = '\0' as i32 as libc::c_char;
        } else {
            p = p.offset(1);
        }
    }
}
unsafe extern "C" fn printRect(mut r: *mut xdot_rect, mut print: pf, mut info: *mut libc::c_void) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b" %.02f\0" as *const u8 as *const libc::c_char,
        (*r).x,
    );
    trim(buf.as_mut_ptr());
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b" %.02f\0" as *const u8 as *const libc::c_char,
        (*r).y,
    );
    trim(buf.as_mut_ptr());
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b" %.02f\0" as *const u8 as *const libc::c_char,
        (*r).w,
    );
    trim(buf.as_mut_ptr());
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b" %.02f\0" as *const u8 as *const libc::c_char,
        (*r).h,
    );
    trim(buf.as_mut_ptr());
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
}
unsafe extern "C" fn printPolyline(
    mut p: *mut xdot_polyline,
    mut print: pf,
    mut info: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 512] = [0; 512];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b" %d\0" as *const u8 as *const libc::c_char,
        (*p).cnt,
    );
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
    i = 0 as libc::c_int;
    while i < (*p).cnt {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            b" %.02f\0" as *const u8 as *const libc::c_char,
            (*((*p).pts).offset(i as isize)).x,
        );
        trim(buf.as_mut_ptr());
        print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            b" %.02f\0" as *const u8 as *const libc::c_char,
            (*((*p).pts).offset(i as isize)).y,
        );
        trim(buf.as_mut_ptr());
        print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
        i += 1;
    }
}
unsafe extern "C" fn printString(
    mut p: *mut libc::c_char,
    mut print: pf,
    mut info: *mut libc::c_void,
) {
    let mut buf: [libc::c_char; 30] = [0; 30];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
        b" %zu -\0" as *const u8 as *const libc::c_char,
        strlen(p),
    );
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
    print.expect("non-null function pointer")(p, info);
}
unsafe extern "C" fn printInt(mut i: libc::c_int, mut print: pf, mut info: *mut libc::c_void) {
    let mut buf: [libc::c_char; 30] = [0; 30];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
        b" %d\0" as *const u8 as *const libc::c_char,
        i,
    );
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
}
unsafe extern "C" fn printFloat(
    mut f: libc::c_double,
    mut print: pf,
    mut info: *mut libc::c_void,
    mut space: libc::c_int,
) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    if space != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b" %.02f\0" as *const u8 as *const libc::c_char,
            f,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%.02f\0" as *const u8 as *const libc::c_char,
            f,
        );
    }
    trim(buf.as_mut_ptr());
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
}
unsafe extern "C" fn printAlign(mut a: xdot_align, mut print: pf, mut info: *mut libc::c_void) {
    match a as libc::c_uint {
        0 => {
            print.expect("non-null function pointer")(
                b" -1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        }
        2 => {
            print.expect("non-null function pointer")(
                b" 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        }
        1 => {
            print.expect("non-null function pointer")(
                b" 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn gradprint(mut s: *mut libc::c_char, mut v: *mut libc::c_void) {
    agxbput(v as *mut agxbuf, s);
}
unsafe extern "C" fn toGradString(mut xb: *mut agxbuf, mut cp: *mut xdot_color) {
    let mut i: libc::c_int = 0;
    let mut n_stops: libc::c_int = 0;
    let mut stops: *mut xdot_color_stop = 0 as *mut xdot_color_stop;
    if (*cp).type_0 as libc::c_uint == xd_linear as libc::c_int as libc::c_uint {
        agxbputc(xb, '[' as i32 as libc::c_char);
        printFloat(
            (*cp).u.ling.x0,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            0 as libc::c_int,
        );
        printFloat(
            (*cp).u.ling.y0,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        printFloat(
            (*cp).u.ling.x1,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        printFloat(
            (*cp).u.ling.y1,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        n_stops = (*cp).u.ling.n_stops;
        stops = (*cp).u.ling.stops;
    } else {
        agxbputc(xb, '(' as i32 as libc::c_char);
        printFloat(
            (*cp).u.ring.x0,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            0 as libc::c_int,
        );
        printFloat(
            (*cp).u.ring.y0,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        printFloat(
            (*cp).u.ring.r0,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        printFloat(
            (*cp).u.ring.x1,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        printFloat(
            (*cp).u.ring.y1,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        printFloat(
            (*cp).u.ring.r1,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        n_stops = (*cp).u.ring.n_stops;
        stops = (*cp).u.ring.stops;
    }
    printInt(
        n_stops,
        Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
        xb as *mut libc::c_void,
    );
    i = 0 as libc::c_int;
    while i < n_stops {
        printFloat(
            (*stops.offset(i as isize)).frac as libc::c_double,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
            1 as libc::c_int,
        );
        printString(
            (*stops.offset(i as isize)).color,
            Some(gradprint as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
            xb as *mut libc::c_void,
        );
        i += 1;
    }
    if (*cp).type_0 as libc::c_uint == xd_linear as libc::c_int as libc::c_uint {
        agxbputc(xb, ']' as i32 as libc::c_char);
    } else {
        agxbputc(xb, ')' as i32 as libc::c_char);
    };
}
unsafe extern "C" fn printXDot_Op(
    mut op: *mut xdot_op,
    mut print: pf,
    mut info: *mut libc::c_void,
    mut more: libc::c_int,
) {
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    agxbinit(
        &mut xb,
        8192 as libc::c_int as libc::c_uint,
        buf.as_mut_ptr(),
    );
    match (*op).kind as libc::c_uint {
        0 => {
            print.expect("non-null function pointer")(
                b"E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printRect(&mut (*op).u.ellipse, print, info);
        }
        1 => {
            print.expect("non-null function pointer")(
                b"e\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printRect(&mut (*op).u.ellipse, print, info);
        }
        2 => {
            print.expect("non-null function pointer")(
                b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printPolyline(&mut (*op).u.polygon, print, info);
        }
        3 => {
            print.expect("non-null function pointer")(
                b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printPolyline(&mut (*op).u.polygon, print, info);
        }
        4 => {
            print.expect("non-null function pointer")(
                b"b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printPolyline(&mut (*op).u.bezier, print, info);
        }
        5 => {
            print.expect("non-null function pointer")(
                b"B\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printPolyline(&mut (*op).u.bezier, print, info);
        }
        9 => {
            print.expect("non-null function pointer")(
                b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printString((*op).u.color, print, info);
        }
        14 => {
            print.expect("non-null function pointer")(
                b"c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            toGradString(&mut xb, &mut (*op).u.grad_color);
            printString(agxbuse(&mut xb), print, info);
        }
        8 => {
            print.expect("non-null function pointer")(
                b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printString((*op).u.color, print, info);
        }
        13 => {
            print.expect("non-null function pointer")(
                b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            toGradString(&mut xb, &mut (*op).u.grad_color);
            printString(agxbuse(&mut xb), print, info);
        }
        6 => {
            print.expect("non-null function pointer")(
                b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printPolyline(&mut (*op).u.polyline, print, info);
        }
        7 => {
            print.expect("non-null function pointer")(
                b"T\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printInt((*op).u.text.x as libc::c_int, print, info);
            printInt((*op).u.text.y as libc::c_int, print, info);
            printAlign((*op).u.text.align, print, info);
            printInt((*op).u.text.width as libc::c_int, print, info);
            printString((*op).u.text.text, print, info);
        }
        10 => {
            print.expect("non-null function pointer")(
                b"F\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printFloat((*op).u.font.size, print, info, 1 as libc::c_int);
            printString((*op).u.font.name, print, info);
        }
        15 => {
            print.expect("non-null function pointer")(
                b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printInt((*op).u.fontchar as libc::c_int, print, info);
        }
        11 => {
            print.expect("non-null function pointer")(
                b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printString((*op).u.style, print, info);
        }
        12 => {
            print.expect("non-null function pointer")(
                b"I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printRect(&mut (*op).u.image.pos, print, info);
            printString((*op).u.image.name, print, info);
        }
        _ => {}
    }
    if more != 0 {
        print.expect("non-null function pointer")(
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            info,
        );
    }
    agxbfree(&mut xb);
}
unsafe extern "C" fn jsonRect(mut r: *mut xdot_rect, mut print: pf, mut info: *mut libc::c_void) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"[%.06f,%.06f,%.06f,%.06f]\0" as *const u8 as *const libc::c_char,
        (*r).x,
        (*r).y,
        (*r).w,
        (*r).h,
    );
    print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
}
unsafe extern "C" fn jsonPolyline(
    mut p: *mut xdot_polyline,
    mut print: pf,
    mut info: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    print.expect("non-null function pointer")(
        b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        info,
    );
    i = 0 as libc::c_int;
    while i < (*p).cnt {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%.06f,%.06f\0" as *const u8 as *const libc::c_char,
            (*((*p).pts).offset(i as isize)).x,
            (*((*p).pts).offset(i as isize)).y,
        );
        print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
        if i < (*p).cnt - 1 as libc::c_int {
            print.expect("non-null function pointer")(
                b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        }
        i += 1;
    }
    print.expect("non-null function pointer")(
        b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        info,
    );
}
unsafe extern "C" fn jsonString(
    mut p: *mut libc::c_char,
    mut print: pf,
    mut info: *mut libc::c_void,
) {
    let mut c: libc::c_char = 0;
    print.expect("non-null function pointer")(
        b"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        info,
    );
    loop {
        let fresh34 = p;
        p = p.offset(1);
        c = *fresh34;
        if !(c != 0) {
            break;
        }
        if c as libc::c_int == '"' as i32 {
            print.expect("non-null function pointer")(
                b"\\\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        } else if c as libc::c_int == '\\' as i32 {
            print.expect("non-null function pointer")(
                b"\\\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        } else {
            let mut buf: [libc::c_char; 2] = [c, '\0' as i32 as libc::c_char];
            print.expect("non-null function pointer")(buf.as_mut_ptr(), info);
        }
    }
    print.expect("non-null function pointer")(
        b"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        info,
    );
}
unsafe extern "C" fn jsonXDot_Op(
    mut op: *mut xdot_op,
    mut print: pf,
    mut info: *mut libc::c_void,
    mut more: libc::c_int,
) {
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    agxbinit(
        &mut xb,
        8192 as libc::c_int as libc::c_uint,
        buf.as_mut_ptr(),
    );
    match (*op).kind as libc::c_uint {
        0 => {
            print.expect("non-null function pointer")(
                b"{\"E\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonRect(&mut (*op).u.ellipse, print, info);
        }
        1 => {
            print.expect("non-null function pointer")(
                b"{\"e\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonRect(&mut (*op).u.ellipse, print, info);
        }
        2 => {
            print.expect("non-null function pointer")(
                b"{\"P\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonPolyline(&mut (*op).u.polygon, print, info);
        }
        3 => {
            print.expect("non-null function pointer")(
                b"{\"p\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonPolyline(&mut (*op).u.polygon, print, info);
        }
        4 => {
            print.expect("non-null function pointer")(
                b"{\"b\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonPolyline(&mut (*op).u.bezier, print, info);
        }
        5 => {
            print.expect("non-null function pointer")(
                b"{\"B\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonPolyline(&mut (*op).u.bezier, print, info);
        }
        9 => {
            print.expect("non-null function pointer")(
                b"{\"c\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonString((*op).u.color, print, info);
        }
        14 => {
            print.expect("non-null function pointer")(
                b"{\"c\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            toGradString(&mut xb, &mut (*op).u.grad_color);
            jsonString(agxbuse(&mut xb), print, info);
        }
        8 => {
            print.expect("non-null function pointer")(
                b"{\"C\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonString((*op).u.color, print, info);
        }
        13 => {
            print.expect("non-null function pointer")(
                b"{\"C\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            toGradString(&mut xb, &mut (*op).u.grad_color);
            jsonString(agxbuse(&mut xb), print, info);
        }
        6 => {
            print.expect("non-null function pointer")(
                b"{\"L\" :\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonPolyline(&mut (*op).u.polyline, print, info);
        }
        7 => {
            print.expect("non-null function pointer")(
                b"{\"T\" : [\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printInt((*op).u.text.x as libc::c_int, print, info);
            print.expect("non-null function pointer")(
                b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printInt((*op).u.text.y as libc::c_int, print, info);
            print.expect("non-null function pointer")(
                b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printAlign((*op).u.text.align, print, info);
            print.expect("non-null function pointer")(
                b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printInt((*op).u.text.width as libc::c_int, print, info);
            print.expect("non-null function pointer")(
                b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonString((*op).u.text.text, print, info);
            print.expect("non-null function pointer")(
                b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        }
        10 => {
            print.expect("non-null function pointer")(
                b"{\"F\" : [\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            (*op).kind = xd_font;
            printFloat((*op).u.font.size, print, info, 1 as libc::c_int);
            print.expect("non-null function pointer")(
                b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonString((*op).u.font.name, print, info);
            print.expect("non-null function pointer")(
                b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        }
        15 => {
            print.expect("non-null function pointer")(
                b"{\"t\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            printInt((*op).u.fontchar as libc::c_int, print, info);
        }
        11 => {
            print.expect("non-null function pointer")(
                b"{\"S\" : \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonString((*op).u.style, print, info);
        }
        12 => {
            print.expect("non-null function pointer")(
                b"{\"I\" : [\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonRect(&mut (*op).u.image.pos, print, info);
            print.expect("non-null function pointer")(
                b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
            jsonString((*op).u.image.name, print, info);
            print.expect("non-null function pointer")(
                b"]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                info,
            );
        }
        _ => {}
    }
    if more != 0 {
        print.expect("non-null function pointer")(
            b"},\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            info,
        );
    } else {
        print.expect("non-null function pointer")(
            b"}\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            info,
        );
    }
    agxbfree(&mut xb);
}
unsafe extern "C" fn _printXDot(
    mut x: *mut xdot,
    mut print: pf,
    mut info: *mut libc::c_void,
    mut ofn: print_op,
) {
    let mut i: libc::c_int = 0;
    let mut op: *mut xdot_op = 0 as *mut xdot_op;
    let mut base: *mut libc::c_char = (*x).ops as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*x).cnt {
        op = base.offset((i * (*x).sz) as isize) as *mut xdot_op;
        ofn.expect("non-null function pointer")(
            op,
            print,
            info,
            (i < (*x).cnt - 1 as libc::c_int) as libc::c_int,
        );
        i += 1;
    }
}
unsafe extern "C" fn agxbput_(mut s: *mut libc::c_char, mut xb: *mut libc::c_void) {
    agxbput(xb as *mut agxbuf, s);
}
#[no_mangle]
pub unsafe extern "C" fn sprintXDot(mut x: *mut xdot) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    agxbinit(
        &mut xb,
        8192 as libc::c_int as libc::c_uint,
        buf.as_mut_ptr(),
    );
    _printXDot(
        x,
        Some(agxbput_ as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> ()),
        &mut xb as *mut agxbuf as *mut libc::c_void,
        Some(
            printXDot_Op
                as unsafe extern "C" fn(*mut xdot_op, pf, *mut libc::c_void, libc::c_int) -> (),
        ),
    );
    s = strdup(agxbuse(&mut xb));
    agxbfree(&mut xb);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn fprintXDot(mut fp: *mut FILE, mut x: *mut xdot) {
    _printXDot(
        x,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(*const libc::c_char, *mut FILE) -> libc::c_int>,
            pf,
        >(Some(
            fputs as unsafe extern "C" fn(*const libc::c_char, *mut FILE) -> libc::c_int,
        )),
        fp as *mut libc::c_void,
        Some(
            printXDot_Op
                as unsafe extern "C" fn(*mut xdot_op, pf, *mut libc::c_void, libc::c_int) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsonXDot(mut fp: *mut FILE, mut x: *mut xdot) {
    fputs(b"[\n\0" as *const u8 as *const libc::c_char, fp);
    _printXDot(
        x,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(*const libc::c_char, *mut FILE) -> libc::c_int>,
            pf,
        >(Some(
            fputs as unsafe extern "C" fn(*const libc::c_char, *mut FILE) -> libc::c_int,
        )),
        fp as *mut libc::c_void,
        Some(
            jsonXDot_Op
                as unsafe extern "C" fn(*mut xdot_op, pf, *mut libc::c_void, libc::c_int) -> (),
        ),
    );
    fputs(b"]\n\0" as *const u8 as *const libc::c_char, fp);
}
unsafe extern "C" fn freeXOpData(mut x: *mut xdot_op) {
    match (*x).kind as libc::c_uint {
        2 | 3 => {
            free((*x).u.polyline.pts as *mut libc::c_void);
        }
        4 | 5 => {
            free((*x).u.polyline.pts as *mut libc::c_void);
        }
        6 => {
            free((*x).u.polyline.pts as *mut libc::c_void);
        }
        7 => {
            free((*x).u.text.text as *mut libc::c_void);
        }
        8 | 9 => {
            free((*x).u.color as *mut libc::c_void);
        }
        13 | 14 => {
            freeXDotColor(&mut (*x).u.grad_color);
        }
        10 => {
            free((*x).u.font.name as *mut libc::c_void);
        }
        11 => {
            free((*x).u.style as *mut libc::c_void);
        }
        12 => {
            free((*x).u.image.name as *mut libc::c_void);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeXDot(mut x: *mut xdot) {
    let mut i: libc::c_int = 0;
    let mut op: *mut xdot_op = 0 as *mut xdot_op;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ff: freefunc_t = (*x).freefunc;
    if x.is_null() {
        return;
    }
    base = (*x).ops as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*x).cnt {
        op = base.offset((i * (*x).sz) as isize) as *mut xdot_op;
        if ff.is_some() {
            ff.expect("non-null function pointer")(op);
        }
        freeXOpData(op);
        i += 1;
    }
    free(base as *mut libc::c_void);
    free(x as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn statXDot(mut x: *mut xdot, mut sp: *mut xdot_stats) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut op: *mut xdot_op = 0 as *mut xdot_op;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    if x.is_null() || sp.is_null() {
        return 1 as libc::c_int;
    }
    memset(
        sp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xdot_stats>() as libc::c_ulong,
    );
    (*sp).cnt = (*x).cnt;
    base = (*x).ops as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*x).cnt {
        op = base.offset((i * (*x).sz) as isize) as *mut xdot_op;
        match (*op).kind as libc::c_uint {
            0 | 1 => {
                let ref mut fresh35 = (*sp).n_ellipse;
                *fresh35 += 1;
            }
            2 | 3 => {
                let ref mut fresh36 = (*sp).n_polygon;
                *fresh36 += 1;
                (*sp).n_polygon_pts += (*op).u.polygon.cnt;
            }
            4 | 5 => {
                let ref mut fresh37 = (*sp).n_bezier;
                *fresh37 += 1;
                (*sp).n_bezier_pts += (*op).u.bezier.cnt;
            }
            6 => {
                let ref mut fresh38 = (*sp).n_polyline;
                *fresh38 += 1;
                (*sp).n_polyline_pts += (*op).u.polyline.cnt;
            }
            7 => {
                let ref mut fresh39 = (*sp).n_text;
                *fresh39 += 1;
            }
            12 => {
                let ref mut fresh40 = (*sp).n_image;
                *fresh40 += 1;
            }
            8 | 9 => {
                let ref mut fresh41 = (*sp).n_color;
                *fresh41 += 1;
            }
            13 | 14 => {
                let ref mut fresh42 = (*sp).n_gradcolor;
                *fresh42 += 1;
            }
            10 => {
                let ref mut fresh43 = (*sp).n_font;
                *fresh43 += 1;
            }
            15 => {
                let ref mut fresh44 = (*sp).n_fontchar;
                *fresh44 += 1;
            }
            11 => {
                let ref mut fresh45 = (*sp).n_style;
                *fresh45 += 1;
            }
            _ => {}
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn radGradient(
    mut cp: *mut libc::c_char,
    mut clr: *mut xdot_color,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = cp;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut stops: *mut xdot_color_stop = 0 as *mut xdot_color_stop;
    (*clr).type_0 = xd_radial;
    s = parseReal(s, &mut (*clr).u.ring.x0);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ring.y0);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ring.r0);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ring.x1);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ring.y1);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ring.r1);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseInt(s, &mut (*clr).u.ring.n_stops);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    stops = gv_calloc(
        (*clr).u.ring.n_stops as size_t,
        ::std::mem::size_of::<xdot_color_stop>() as libc::c_ulong,
    ) as *mut xdot_color_stop;
    i = 0 as libc::c_int;
    while i < (*clr).u.ring.n_stops {
        s = parseReal(s, &mut d);
        if s.is_null() {
            free(stops as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        (*stops.offset(i as isize)).frac = d as libc::c_float;
        s = parseString(s, &mut (*stops.offset(i as isize)).color);
        if s.is_null() {
            free(stops as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        i += 1;
    }
    let ref mut fresh46 = (*clr).u.ring.stops;
    *fresh46 = stops;
    return cp;
}
unsafe extern "C" fn linGradient(
    mut cp: *mut libc::c_char,
    mut clr: *mut xdot_color,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = cp;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut stops: *mut xdot_color_stop = 0 as *mut xdot_color_stop;
    (*clr).type_0 = xd_linear;
    s = parseReal(s, &mut (*clr).u.ling.x0);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ling.y0);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ling.x1);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseReal(s, &mut (*clr).u.ling.y1);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    s = parseInt(s, &mut (*clr).u.ling.n_stops);
    if s.is_null() {
        free(stops as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    stops = gv_calloc(
        (*clr).u.ling.n_stops as size_t,
        ::std::mem::size_of::<xdot_color_stop>() as libc::c_ulong,
    ) as *mut xdot_color_stop;
    i = 0 as libc::c_int;
    while i < (*clr).u.ling.n_stops {
        s = parseReal(s, &mut d);
        if s.is_null() {
            free(stops as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        (*stops.offset(i as isize)).frac = d as libc::c_float;
        s = parseString(s, &mut (*stops.offset(i as isize)).color);
        if s.is_null() {
            free(stops as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        i += 1;
    }
    let ref mut fresh47 = (*clr).u.ling.stops;
    *fresh47 = stops;
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn parseXDotColor(
    mut cp: *mut libc::c_char,
    mut clr: *mut xdot_color,
) -> *mut libc::c_char {
    let mut c: libc::c_char = *cp;
    match c as libc::c_int {
        91 => return linGradient(cp.offset(1 as libc::c_int as isize), clr),
        40 => return radGradient(cp.offset(1 as libc::c_int as isize), clr),
        35 | 47 => {
            (*clr).type_0 = xd_none;
            let ref mut fresh48 = (*clr).u.clr;
            *fresh48 = cp;
            return cp;
        }
        _ => {
            if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                (*clr).type_0 = xd_none;
                let ref mut fresh49 = (*clr).u.clr;
                *fresh49 = cp;
                return cp;
            } else {
                return 0 as *mut libc::c_char;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeXDotColor(mut cp: *mut xdot_color) {
    let mut i: libc::c_int = 0;
    if (*cp).type_0 as libc::c_uint == xd_linear as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < (*cp).u.ling.n_stops {
            free((*((*cp).u.ling.stops).offset(i as isize)).color as *mut libc::c_void);
            i += 1;
        }
        free((*cp).u.ling.stops as *mut libc::c_void);
    } else if (*cp).type_0 as libc::c_uint == xd_radial as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < (*cp).u.ring.n_stops {
            free((*((*cp).u.ring.stops).offset(i as isize)).color as *mut libc::c_void);
            i += 1;
        }
        free((*cp).u.ring.stops as *mut libc::c_void);
    }
}
