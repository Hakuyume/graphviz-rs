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
#![feature(c_variadic, extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GData;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn strcasestr(str: *const libc::c_char, pat: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn g_free(mem: gpointer);
    fn pango_font_family_list_faces(
        family: *mut PangoFontFamily,
        faces: *mut *mut *mut PangoFontFace,
        n_faces: *mut libc::c_int,
    );
    fn pango_font_family_get_name(family: *mut PangoFontFamily) -> *const libc::c_char;
    fn pango_font_face_get_face_name(face: *mut PangoFontFace) -> *const libc::c_char;
    fn pango_font_map_list_families(
        fontmap: *mut PangoFontMap,
        families: *mut *mut *mut PangoFontFamily,
        n_families: *mut libc::c_int,
    );
    static mut Verbose: libc::c_uchar;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __int32_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PostscriptAlias {
    pub name: *mut libc::c_char,
    pub family: *mut libc::c_char,
    pub weight: *mut libc::c_char,
    pub stretch: *mut libc::c_char,
    pub style: *mut libc::c_char,
    pub xfig_code: libc::c_int,
    pub svg_font_family: *mut libc::c_char,
    pub svg_font_weight: *mut libc::c_char,
    pub svg_font_style: *mut libc::c_char,
}
pub type PostscriptAlias = _PostscriptAlias;
pub type gsize = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type GData = _GData;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFontMap {
    pub parent_instance: GObject,
}
pub type PangoFontMap = _PangoFontMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFontFace {
    pub parent_instance: GObject,
}
pub type PangoFontFace = _PangoFontFace;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFontFamily {
    pub parent_instance: GObject,
}
pub type PangoFontFamily = _PangoFontFamily;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gv_font_map {
    pub gv_ps_fontname: *mut libc::c_char,
    pub gv_font: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct availfont_t {
    pub gv_ps_fontname: *mut libc::c_char,
    pub fontname: *mut libc::c_char,
    pub faces: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontdef_t {
    pub generic_name: *mut libc::c_char,
    pub fontname: *mut libc::c_char,
    pub eq_sz: libc::c_int,
    pub equiv: *mut *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct face_t {
    pub flag: libc::c_int,
    pub name: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn agxbinit(
    mut xb: *mut agxbuf,
    mut hint: libc::c_uint,
    mut init: *mut libc::c_char,
) {
    if !init.is_null() {
        let ref mut fresh0 = (*xb).buf;
        *fresh0 = init;
        (*xb).dyna = 0 as libc::c_int;
    } else {
        if hint == 0 as libc::c_int as libc::c_uint {
            hint = 8192 as libc::c_int as libc::c_uint;
        }
        (*xb).dyna = 1 as libc::c_int;
        let ref mut fresh1 = (*xb).buf;
        *fresh1 = gv_calloc(
            hint as size_t,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    let ref mut fresh2 = (*xb).eptr;
    *fresh2 = ((*xb).buf).offset(hint as isize);
    let ref mut fresh3 = (*xb).ptr;
    *fresh3 = (*xb).buf;
    *(*xb).ptr = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn agxbfree(mut xb: *mut agxbuf) {
    if (*xb).dyna != 0 {
        free((*xb).buf as *mut libc::c_void);
    }
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
unsafe extern "C" fn agxbprint(
    mut xb: *mut agxbuf,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut size: size_t = 0;
    let mut result: libc::c_int = 0;
    ap = args.clone();
    let mut ap2: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap2 = ap.clone();
    rc = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap2.as_va_list(),
    );
    if rc < 0 as libc::c_int {
        return rc;
    }
    size = (rc as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut unused_space: size_t = ((*xb).eptr).offset_from((*xb).ptr) as libc::c_long as size_t;
    if unused_space < size {
        let mut extra: size_t = size.wrapping_sub(unused_space);
        agxbmore(xb, extra);
    }
    result = vsnprintf((*xb).ptr, size, fmt, ap.as_va_list());
    if result == size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        || result < 0 as libc::c_int
    {
    } else {
        __assert_fail(
            b"result == (int)(size - 1) || result < 0\0" as *const u8 as *const libc::c_char,
            b"../../lib/cgraph/agxbuf.h\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"int agxbprint(agxbuf *, const char *, ...)\0",
            ))
            .as_ptr(),
        );
    }
    if result > 0 as libc::c_int {
        let ref mut fresh7 = (*xb).ptr;
        *fresh7 = (*fresh7).offset(result as size_t as isize);
    }
    return result;
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
    let ref mut fresh8 = (*xb).ptr;
    *fresh8 = (*fresh8).offset(ssz as isize);
    return ssz;
}
#[inline]
unsafe extern "C" fn agxbput(mut xb: *mut agxbuf, mut s: *const libc::c_char) -> size_t {
    let mut ssz: size_t = strlen(s);
    return agxbput_n(xb, s, ssz);
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
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh9 = (*xb).ptr;
    let fresh10 = *fresh9;
    *fresh9 = (*fresh9).offset(1);
    *fresh10 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn agxbuse(mut xb: *mut agxbuf) -> *mut libc::c_char {
    agxbputc(xb, '\0' as i32 as libc::c_char);
    let ref mut fresh11 = (*xb).ptr;
    *fresh11 = (*xb).buf;
    return (*xb).ptr;
}
#[inline]
unsafe extern "C" fn agxbdisown(mut xb: *mut agxbuf) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    agxbputc(xb, '\0' as i32 as libc::c_char);
    if (*xb).dyna == 0 {
        buf = strdup((*xb).buf);
        if buf.is_null() {
            return 0 as *mut libc::c_char;
        }
    } else {
        buf = (*xb).buf;
    }
    let ref mut fresh12 = (*xb).eptr;
    *fresh12 = 0 as *mut libc::c_char;
    let ref mut fresh13 = (*xb).ptr;
    *fresh13 = *fresh12;
    let ref mut fresh14 = (*xb).buf;
    *fresh14 = *fresh13;
    (*xb).dyna = 1 as libc::c_int;
    return buf;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
static mut facelist: [face_t; 11] = [
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 0 as libc::c_int,
            name: b"BOLD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 1 as libc::c_int,
            name: b"BOOK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 2 as libc::c_int,
            name: b"CONDENSED\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 3 as libc::c_int,
            name: b"DEMI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 4 as libc::c_int,
            name: b"EXTRALIGHT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 5 as libc::c_int,
            name: b"ITALIC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 6 as libc::c_int,
            name: b"LIGHT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 7 as libc::c_int,
            name: b"MEDIUM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 8 as libc::c_int,
            name: b"OBLIQUE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 9 as libc::c_int,
            name: b"REGULAR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = face_t {
            flag: (1 as libc::c_int) << 9 as libc::c_int,
            name: b"ROMAN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
static mut PS_AVANT_E: [*const libc::c_char; 10] = [
    b"URW Gothic L\0" as *const u8 as *const libc::c_char,
    b"Charcoal\0" as *const u8 as *const libc::c_char,
    b"Nimbus Sans L\0" as *const u8 as *const libc::c_char,
    b"Verdana\0" as *const u8 as *const libc::c_char,
    b"Helvetica\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Sans\0" as *const u8 as *const libc::c_char,
    b"DejaVu Sans\0" as *const u8 as *const libc::c_char,
    b"Liberation Sans\0" as *const u8 as *const libc::c_char,
    b"Luxi Sans\0" as *const u8 as *const libc::c_char,
    b"FreeSans\0" as *const u8 as *const libc::c_char,
];
static mut PS_BOOKMAN_E: [*const libc::c_char; 9] = [
    b"URW Bookman L\0" as *const u8 as *const libc::c_char,
    b"Times New Roman\0" as *const u8 as *const libc::c_char,
    b"Times\0" as *const u8 as *const libc::c_char,
    b"Nimbus Roman No9 L\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Serif\0" as *const u8 as *const libc::c_char,
    b"DejaVu Serif\0" as *const u8 as *const libc::c_char,
    b"Liberation Serif\0" as *const u8 as *const libc::c_char,
    b"Luxi Serif\0" as *const u8 as *const libc::c_char,
    b"FreeSerif\0" as *const u8 as *const libc::c_char,
];
static mut PS_COURIER_E: [*const libc::c_char; 8] = [
    b"Nimbus Mono L\0" as *const u8 as *const libc::c_char,
    b"Inconsolata\0" as *const u8 as *const libc::c_char,
    b"Courier New\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Sans Mono\0" as *const u8 as *const libc::c_char,
    b"DejaVu Sans Mono\0" as *const u8 as *const libc::c_char,
    b"Liberation Mono\0" as *const u8 as *const libc::c_char,
    b"Luxi Mono\0" as *const u8 as *const libc::c_char,
    b"FreeMono\0" as *const u8 as *const libc::c_char,
];
static mut PS_HELVETICA_E: [*const libc::c_char; 8] = [
    b"Nimbus Sans L\0" as *const u8 as *const libc::c_char,
    b"Arial\0" as *const u8 as *const libc::c_char,
    b"Verdana\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Sans\0" as *const u8 as *const libc::c_char,
    b"DejaVu Sans\0" as *const u8 as *const libc::c_char,
    b"Liberation Sans\0" as *const u8 as *const libc::c_char,
    b"Luxi Sans\0" as *const u8 as *const libc::c_char,
    b"FreeSans\0" as *const u8 as *const libc::c_char,
];
static mut PS_NEWCENT_E: [*const libc::c_char; 10] = [
    b"Century Schoolbook L\0" as *const u8 as *const libc::c_char,
    b"Times New Roman\0" as *const u8 as *const libc::c_char,
    b"Times\0" as *const u8 as *const libc::c_char,
    b"Nimbus Roman No9 L\0" as *const u8 as *const libc::c_char,
    b"Georgia\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Serif\0" as *const u8 as *const libc::c_char,
    b"DejaVu Serif\0" as *const u8 as *const libc::c_char,
    b"Liberation Serif\0" as *const u8 as *const libc::c_char,
    b"Luxi Serif\0" as *const u8 as *const libc::c_char,
    b"FreeSerif\0" as *const u8 as *const libc::c_char,
];
static mut PS_PALATINO_E: [*const libc::c_char; 11] = [
    b"URW Palladio L\0" as *const u8 as *const libc::c_char,
    b"Times New Roman\0" as *const u8 as *const libc::c_char,
    b"Times\0" as *const u8 as *const libc::c_char,
    b"Nimbus Roman No9 L\0" as *const u8 as *const libc::c_char,
    b"Norasi\0" as *const u8 as *const libc::c_char,
    b"Rekha\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Serif\0" as *const u8 as *const libc::c_char,
    b"DejaVu Serif\0" as *const u8 as *const libc::c_char,
    b"Liberation Serif\0" as *const u8 as *const libc::c_char,
    b"Luxi Serif\0" as *const u8 as *const libc::c_char,
    b"FreeSerif\0" as *const u8 as *const libc::c_char,
];
static mut PS_TIMES_E: [*const libc::c_char; 8] = [
    b"Nimbus Roman No9 L\0" as *const u8 as *const libc::c_char,
    b"Times New Roman\0" as *const u8 as *const libc::c_char,
    b"Charcoal\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Serif\0" as *const u8 as *const libc::c_char,
    b"DejaVu Serif\0" as *const u8 as *const libc::c_char,
    b"Liberation Serif\0" as *const u8 as *const libc::c_char,
    b"Luxi Serif\0" as *const u8 as *const libc::c_char,
    b"FreeSerif\0" as *const u8 as *const libc::c_char,
];
static mut PS_SYMBOL_E: [*const libc::c_char; 4] = [
    b"Impact\0" as *const u8 as *const libc::c_char,
    b"Copperplate Gothic Std\0" as *const u8 as *const libc::c_char,
    b"Cooper Std\0" as *const u8 as *const libc::c_char,
    b"Bauhaus Std\0" as *const u8 as *const libc::c_char,
];
static mut PS_CHANCERY_E: [*const libc::c_char; 10] = [
    b"URW Chancery L\0" as *const u8 as *const libc::c_char,
    b"Charcoal\0" as *const u8 as *const libc::c_char,
    b"Times New Roman\0" as *const u8 as *const libc::c_char,
    b"Times\0" as *const u8 as *const libc::c_char,
    b"Nimbus Roman No9 L\0" as *const u8 as *const libc::c_char,
    b"Bitstream Vera Serif\0" as *const u8 as *const libc::c_char,
    b"DejaVu Serif\0" as *const u8 as *const libc::c_char,
    b"Liberation Serif\0" as *const u8 as *const libc::c_char,
    b"Luxi Serif\0" as *const u8 as *const libc::c_char,
    b"FreeSerif\0" as *const u8 as *const libc::c_char,
];
static mut PS_DINGBATS_E: [*const libc::c_char; 5] = [
    b"Dingbats\0" as *const u8 as *const libc::c_char,
    b"Impact\0" as *const u8 as *const libc::c_char,
    b"Copperplate Gothic Std\0" as *const u8 as *const libc::c_char,
    b"Cooper Std\0" as *const u8 as *const libc::c_char,
    b"Bauhaus Std\0" as *const u8 as *const libc::c_char,
];
static mut gv_ps_fontdefs: [fontdef_t; 10] = [fontdef_t {
    generic_name: 0 as *mut libc::c_char,
    fontname: 0 as *mut libc::c_char,
    eq_sz: 0,
    equiv: 0 as *mut *const libc::c_char,
}; 10];
static mut postscript_alias: [PostscriptAlias; 35] = [
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-Book\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"book\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 4 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-BookOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"book\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 5 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-Demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 6 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"AvantGarde-DemiOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Gothic L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 7 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-Demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 10 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-DemiItalic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"demi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 11 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-Light\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"light\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 8 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Bookman-LightItalic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"URW Bookman L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"light\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 9 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 12 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier-Bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 14 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier-BoldOblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 15 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Courier-Oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 13 as libc::c_int,
            svg_font_family: b"monospace\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 16 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 18 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-BoldOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 19 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 20 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 22 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow-BoldOblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 23 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Narrow-Oblique\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: b"condensed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 21 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Helvetica-Oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"oblique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 17 as libc::c_int,
            svg_font_family: b"sans-Serif\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-Bold\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 26 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-BoldItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 27 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-Italic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 25 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"NewCenturySchlbk-Roman\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"Century Schoolbook L\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            weight: b"roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 24 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-Bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 30 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-BoldItalic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 31 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-Italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 29 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Palatino-Roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Palatino Linotype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 28 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Symbol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Symbol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 32 as libc::c_int,
            svg_font_family: b"fantasy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-Bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 2 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-BoldItalic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 3 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: b"bold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-Italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 1 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"Times-Roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 0 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"ZapfChancery-MediumItalic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            family: b"URW Chancery L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: b"medium\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 33 as libc::c_int,
            svg_font_family: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: b"italic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = _PostscriptAlias {
            name: b"ZapfDingbats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            family: b"Dingbats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            weight: 0 as *const libc::c_char as *mut libc::c_char,
            stretch: 0 as *const libc::c_char as *mut libc::c_char,
            style: 0 as *const libc::c_char as *mut libc::c_char,
            xfig_code: 34 as libc::c_int,
            svg_font_family: b"fantasy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            svg_font_weight: 0 as *const libc::c_char as *mut libc::c_char,
            svg_font_style: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn gv_flist_free_af(mut gv_af_p: *mut availfont_t) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[fontdef_t; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<fontdef_t>() as libc::c_ulong)
    {
        free((*gv_af_p.offset(i as isize)).fontname as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free(gv_af_p as *mut libc::c_void);
}
unsafe extern "C" fn get_faces(mut family: *mut PangoFontFamily) -> libc::c_int {
    let mut faces: *mut *mut PangoFontFace = 0 as *mut *mut PangoFontFace;
    let mut face: *mut PangoFontFace = 0 as *mut PangoFontFace;
    let mut i: libc::c_int = 0;
    let mut n_faces: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut availfaces: libc::c_int = 0 as libc::c_int;
    pango_font_family_list_faces(family, &mut faces, &mut n_faces);
    i = 0 as libc::c_int;
    while i < n_faces {
        face = *faces.offset(i as isize);
        name = pango_font_face_get_face_name(face);
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j
            < (::std::mem::size_of::<[face_t; 11]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<face_t>() as libc::c_ulong)
        {
            if !(strcasestr(name, facelist[j as usize].name)).is_null() {
                availfaces |= facelist[j as usize].flag;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        i += 1;
    }
    g_free(faces as gpointer);
    return availfaces;
}
unsafe extern "C" fn get_avail_faces(
    mut faces: libc::c_int,
    mut xb: *mut agxbuf,
) -> *mut libc::c_char {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[face_t; 11]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<face_t>() as libc::c_ulong)
    {
        if faces & facelist[i as usize].flag != 0 {
            agxbprint(
                xb,
                b"%s \0" as *const u8 as *const libc::c_char,
                facelist[i as usize].name,
            );
        }
        i = i.wrapping_add(1);
    }
    return agxbuse(xb);
}
unsafe extern "C" fn gv_get_ps_fontlist(mut fontmap: *mut PangoFontMap) -> *mut availfont_t {
    let mut families: *mut *mut PangoFontFamily = 0 as *mut *mut PangoFontFamily;
    let mut family: *mut PangoFontFamily = 0 as *mut PangoFontFamily;
    let mut gv_ps_fontdef: *mut fontdef_t = 0 as *mut fontdef_t;
    let mut n_families: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut array_sz: libc::c_int = 0;
    let mut availfaces: libc::c_int = 0 as libc::c_int;
    let mut gv_af_p: *mut availfont_t = 0 as *mut availfont_t;
    let mut gv_afs: *mut availfont_t = 0 as *mut availfont_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut family_name: *mut libc::c_char = 0 as *mut libc::c_char;
    pango_font_map_list_families(fontmap, &mut families, &mut n_families);
    gv_af_p = calloc(
        (::std::mem::size_of::<[fontdef_t; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<fontdef_t>() as libc::c_ulong),
        ::std::mem::size_of::<availfont_t>() as libc::c_ulong,
    ) as *mut availfont_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j
        < (::std::mem::size_of::<[fontdef_t; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<fontdef_t>() as libc::c_ulong)
    {
        gv_afs = gv_af_p.offset(j as isize);
        gv_ps_fontdef = gv_ps_fontdefs.as_mut_ptr().offset(j as isize);
        let ref mut fresh15 = (*gv_afs).gv_ps_fontname;
        *fresh15 = (*gv_ps_fontdef).fontname;
        family_name = 0 as *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < n_families {
            family = *families.offset(i as isize);
            name = pango_font_family_get_name(family);
            if strcasecmp((*gv_ps_fontdef).fontname, name) == 0 as libc::c_int {
                family_name = strdup(name);
                availfaces = get_faces(family);
            }
            if !family_name.is_null() {
                break;
            }
            i += 1;
        }
        if family_name.is_null() {
            array_sz = (*gv_ps_fontdef).eq_sz;
            k = 0 as libc::c_int;
            while k < array_sz {
                i = 0 as libc::c_int;
                while i < n_families {
                    family = *families.offset(i as isize);
                    name = pango_font_family_get_name(family);
                    if strcasecmp(*((*gv_ps_fontdef).equiv).offset(k as isize), name)
                        == 0 as libc::c_int
                    {
                        family_name = strdup(name);
                        availfaces = get_faces(family);
                        break;
                    } else {
                        i += 1;
                    }
                }
                if !family_name.is_null() {
                    break;
                }
                k += 1;
            }
        }
        if family_name.is_null() {
            i = 0 as libc::c_int;
            while i < n_families {
                family = *families.offset(i as isize);
                name = pango_font_family_get_name(family);
                if strcasecmp((*gv_ps_fontdef).generic_name, name) == 0 as libc::c_int {
                    family_name = strdup(name);
                    availfaces = get_faces(family);
                    break;
                } else {
                    i += 1;
                }
            }
        }
        if !family_name.is_null() && availfaces != 0 {
            let ref mut fresh16 = (*gv_afs).fontname;
            *fresh16 = family_name;
            (*gv_afs).faces = availfaces;
        } else {
            let ref mut fresh17 = (*gv_afs).fontname;
            *fresh17 = 0 as *mut libc::c_char;
            (*gv_afs).faces = 0 as libc::c_int;
            free(family_name as *mut libc::c_void);
        }
        j = j.wrapping_add(1);
    }
    g_free(families as gpointer);
    return gv_af_p;
}
unsafe extern "C" fn copyUpper(mut xb: *mut agxbuf, mut s: *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    loop {
        let fresh18 = s;
        s = s.offset(1);
        c = *fresh18 as libc::c_int;
        if !(c != 0) {
            break;
        }
        agxbputc(
            xb,
            ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = c;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(c);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(c as isize);
                }
                __res
            }) as libc::c_char,
        );
    }
}
unsafe extern "C" fn gv_get_font(
    mut gv_af_p: *mut availfont_t,
    mut ps_alias: *mut PostscriptAlias,
    mut xb: *mut agxbuf,
    mut xb2: *mut agxbuf,
) -> *mut libc::c_char {
    let mut avail_faces: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[fontdef_t; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<fontdef_t>() as libc::c_ulong)
    {
        if (*gv_af_p.offset(i as isize)).faces != 0
            && !(strstr(
                (*ps_alias).name,
                (*gv_af_p.offset(i as isize)).gv_ps_fontname,
            ))
            .is_null()
        {
            agxbprint(
                xb2,
                b"%s, \0" as *const u8 as *const libc::c_char,
                (*gv_af_p.offset(i as isize)).fontname,
            );
            avail_faces = get_avail_faces((*gv_af_p.offset(i as isize)).faces, xb);
            if !((*ps_alias).weight).is_null() {
                if !(strcasestr(avail_faces, (*ps_alias).weight)).is_null() {
                    agxbputc(xb2, ' ' as i32 as libc::c_char);
                    copyUpper(xb2, (*ps_alias).weight);
                }
            } else if !(strcasestr(
                avail_faces,
                b"REGULAR\0" as *const u8 as *const libc::c_char,
            ))
            .is_null()
            {
                agxbput(xb2, b" REGULAR\0" as *const u8 as *const libc::c_char);
            } else if !(strstr(avail_faces, b"ROMAN\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                agxbput(xb2, b" ROMAN\0" as *const u8 as *const libc::c_char);
            }
            if !((*ps_alias).stretch).is_null() {
                if !(strcasestr(avail_faces, (*ps_alias).stretch)).is_null() {
                    agxbputc(xb2, ' ' as i32 as libc::c_char);
                    copyUpper(xb2, (*ps_alias).stretch);
                }
            }
            if !((*ps_alias).style).is_null() {
                if !(strcasestr(avail_faces, (*ps_alias).style)).is_null() {
                    agxbputc(xb2, ' ' as i32 as libc::c_char);
                    copyUpper(xb2, (*ps_alias).style);
                } else if strcasecmp(
                    (*ps_alias).style,
                    b"ITALIC\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if !(strcasestr(
                        avail_faces,
                        b"OBLIQUE\0" as *const u8 as *const libc::c_char,
                    ))
                    .is_null()
                    {
                        agxbput(xb2, b" OBLIQUE\0" as *const u8 as *const libc::c_char);
                    }
                } else if strcasecmp(
                    (*ps_alias).style,
                    b"OBLIQUE\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if !(strcasestr(avail_faces, b"ITALIC\0" as *const u8 as *const libc::c_char))
                        .is_null()
                    {
                        agxbput(xb2, b" ITALIC\0" as *const u8 as *const libc::c_char);
                    }
                }
            }
            return agxbdisown(xb2);
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn printFontMap(mut gv_fmap: *mut gv_font_map, mut sz: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut font: *mut libc::c_char = 0 as *mut libc::c_char;
    j = 0 as libc::c_int;
    while j < sz {
        font = (*gv_fmap.offset(j as isize)).gv_font;
        if font.is_null() {
            fprintf(
                stderr,
                b" [%d] %s => <Not available>\n\0" as *const u8 as *const libc::c_char,
                j,
                (*gv_fmap.offset(j as isize)).gv_ps_fontname,
            );
        } else {
            fprintf(
                stderr,
                b" [%d] %s => \"%s\"\n\0" as *const u8 as *const libc::c_char,
                j,
                (*gv_fmap.offset(j as isize)).gv_ps_fontname,
                font,
            );
        }
        j += 1;
    }
}
static mut ps_fontnames_sz: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn get_font_mapping(mut fontmap: *mut PangoFontMap) -> *mut gv_font_map {
    let mut ps_alias: *mut PostscriptAlias = 0 as *mut PostscriptAlias;
    let mut gv_af_p: *mut availfont_t = 0 as *mut availfont_t;
    let mut gv_fmap: *mut gv_font_map = calloc(
        ps_fontnames_sz,
        ::std::mem::size_of::<gv_font_map>() as libc::c_ulong,
    ) as *mut gv_font_map;
    let mut xb: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut xb2: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut buf2: [libc::c_char; 8192] = [0; 8192];
    agxbinit(
        &mut xb,
        8192 as libc::c_int as libc::c_uint,
        buf.as_mut_ptr(),
    );
    agxbinit(
        &mut xb2,
        8192 as libc::c_int as libc::c_uint,
        buf2.as_mut_ptr(),
    );
    gv_af_p = gv_get_ps_fontlist(fontmap);
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < ps_fontnames_sz {
        ps_alias = &mut *postscript_alias.as_mut_ptr().offset(j as isize) as *mut PostscriptAlias;
        let ref mut fresh19 = (*gv_fmap.offset((*ps_alias).xfig_code as isize)).gv_ps_fontname;
        *fresh19 = (*ps_alias).name;
        let ref mut fresh20 = (*gv_fmap.offset((*ps_alias).xfig_code as isize)).gv_font;
        *fresh20 = gv_get_font(gv_af_p, ps_alias, &mut xb, &mut xb2);
        j = j.wrapping_add(1);
    }
    gv_flist_free_af(gv_af_p);
    agxbfree(&mut xb);
    agxbfree(&mut xb2);
    if Verbose as libc::c_int > 1 as libc::c_int {
        fprintf(
            stderr,
            b"Verbose %d\n\0" as *const u8 as *const libc::c_char,
            Verbose as libc::c_int,
        );
        printFontMap(gv_fmap, ps_fontnames_sz as libc::c_int);
    }
    return gv_fmap;
}
unsafe extern "C" fn run_static_initializers() {
    gv_ps_fontdefs = [
        {
            let mut init = fontdef_t {
                generic_name: b"sans\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"AvantGarde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_AVANT_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"Bookman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_BOOKMAN_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"monospace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fontname: b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_COURIER_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"sans\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_HELVETICA_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"NewCenturySchlbk\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_NEWCENT_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"Palatino\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_PALATINO_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"fantasy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"Symbol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_SYMBOL_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_TIMES_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"ZapfChancery\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_CHANCERY_E.as_mut_ptr(),
            };
            init
        },
        {
            let mut init = fontdef_t {
                generic_name: b"fantasy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fontname: b"ZapfDingbats\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                eq_sz: (::std::mem::size_of::<[*const libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    as libc::c_int,
                equiv: PS_DINGBATS_E.as_mut_ptr(),
            };
            init
        },
    ];
    ps_fontnames_sz = (::std::mem::size_of::<[PostscriptAlias; 35]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<PostscriptAlias>() as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
