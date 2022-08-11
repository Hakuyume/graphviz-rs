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
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sfopen(_: *mut Sfio_t, _: *const libc::c_char, _: *const libc::c_char) -> *mut Sfio_t;
    fn sfclose(_: *mut Sfio_t) -> libc::c_int;
    fn sfungetc(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn _sffilbuf(_: *mut Sfio_t, _: libc::c_int) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn _err_msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn getErrorErrors() -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub struct _sfio_s {
    pub next: *mut libc::c_uchar,
    pub endw: *mut libc::c_uchar,
    pub endr: *mut libc::c_uchar,
    pub endb: *mut libc::c_uchar,
    pub push: *mut Sfio_t,
    pub flags: libc::c_ushort,
    pub file: libc::c_short,
    pub data: *mut libc::c_uchar,
    pub size: ssize_t,
    pub val: ssize_t,
}
pub type Sfio_t = _sfio_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agxbuf {
    pub buf: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub eptr: *mut libc::c_char,
    pub dyna: libc::c_int,
}
pub type case_t = libc::c_uint;
pub const Error: case_t = 7;
pub const Eof: case_t = 6;
pub const Edge: case_t = 5;
pub const Node: case_t = 4;
pub const EndG: case_t = 3;
pub const BeginG: case_t = 2;
pub const End: case_t = 1;
pub const Begin: case_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _case_info {
    pub gstart: libc::c_int,
    pub guard: *mut libc::c_char,
    pub astart: libc::c_int,
    pub action: *mut libc::c_char,
    pub next: *mut _case_info,
}
pub type case_info = _case_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _parse_block {
    pub l_beging: libc::c_int,
    pub begg_stmt: *mut libc::c_char,
    pub n_nstmts: libc::c_int,
    pub n_estmts: libc::c_int,
    pub node_stmts: *mut case_info,
    pub edge_stmts: *mut case_info,
    pub next: *mut _parse_block,
}
pub type parse_block = _parse_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_prog {
    pub source: *mut libc::c_char,
    pub l_begin: libc::c_int,
    pub l_end: libc::c_int,
    pub l_endg: libc::c_int,
    pub begin_stmt: *mut libc::c_char,
    pub n_blocks: libc::c_int,
    pub blocks: *mut parse_block,
    pub endg_stmt: *mut libc::c_char,
    pub end_stmt: *mut libc::c_char,
}
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISalpha: C2RustUnnamed = 1024;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
unsafe extern "C" fn agxbputc(mut xb: *mut agxbuf, mut c: libc::c_char) -> libc::c_int {
    if (*xb).ptr >= (*xb).eptr {
        agxbmore(xb, 1 as libc::c_int as size_t);
    }
    let ref mut fresh7 = (*xb).ptr;
    let fresh8 = *fresh7;
    *fresh7 = (*fresh7).offset(1);
    *fresh8 = c;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
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
    let ref mut fresh9 = (*xb).eptr;
    *fresh9 = 0 as *mut libc::c_char;
    let ref mut fresh10 = (*xb).ptr;
    *fresh10 = *fresh9;
    let ref mut fresh11 = (*xb).buf;
    *fresh11 = *fresh10;
    (*xb).dyna = 1 as libc::c_int;
    return buf;
}
static mut lineno: libc::c_int = 1 as libc::c_int;
static mut col0: libc::c_int = 1 as libc::c_int;
static mut startLine: libc::c_int = 1 as libc::c_int;
static mut kwLine: libc::c_int = 1 as libc::c_int;
static mut case_str: [*mut libc::c_char; 8] = [
    b"BEGIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"END\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"BEG_G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"END_G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ERROR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn caseStr(mut cs: case_t) -> *mut libc::c_char {
    return case_str[cs as libc::c_int as usize];
}
unsafe extern "C" fn eol(mut str: *mut Sfio_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        c = (if (*str).next >= (*str).endr {
            _sffilbuf(str, 0 as libc::c_int)
        } else {
            let ref mut fresh12 = (*str).next;
            let fresh13 = *fresh12;
            *fresh12 = (*fresh12).offset(1);
            *fresh13 as libc::c_int
        });
        if !(c != '\n' as i32) {
            break;
        }
        if c < 0 as libc::c_int {
            return c;
        }
    }
    lineno += 1;
    col0 = 1 as libc::c_int;
    return c;
}
unsafe extern "C" fn readc(mut str: *mut Sfio_t, mut ostr: *mut agxbuf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    c = if (*str).next >= (*str).endr {
        _sffilbuf(str, 0 as libc::c_int)
    } else {
        let ref mut fresh14 = (*str).next;
        let fresh15 = *fresh14;
        *fresh14 = (*fresh14).offset(1);
        *fresh15 as libc::c_int
    };
    match c {
        10 => {
            lineno += 1;
            col0 = 1 as libc::c_int;
        }
        35 => {
            if col0 != 0 {
                c = eol(str);
            } else {
                col0 = 0 as libc::c_int;
            }
        }
        47 => {
            cc = if (*str).next >= (*str).endr {
                _sffilbuf(str, 0 as libc::c_int)
            } else {
                let ref mut fresh16 = (*str).next;
                let fresh17 = *fresh16;
                *fresh16 = (*fresh16).offset(1);
                *fresh17 as libc::c_int
            };
            match cc {
                42 => loop {
                    c = if (*str).next >= (*str).endr {
                        _sffilbuf(str, 0 as libc::c_int)
                    } else {
                        let ref mut fresh18 = (*str).next;
                        let fresh19 = *fresh18;
                        *fresh18 = (*fresh18).offset(1);
                        *fresh19 as libc::c_int
                    };
                    match c {
                        10 => {
                            lineno += 1;
                            if !ostr.is_null() {
                                agxbputc(ostr, c as libc::c_char);
                            }
                        }
                        42 => {
                            cc = if (*str).next >= (*str).endr {
                                _sffilbuf(str, 0 as libc::c_int)
                            } else {
                                let ref mut fresh20 = (*str).next;
                                let fresh21 = *fresh20;
                                *fresh20 = (*fresh20).offset(1);
                                *fresh21 as libc::c_int
                            };
                            match cc {
                                -1 => return cc,
                                10 => {
                                    lineno += 1;
                                    if !ostr.is_null() {
                                        agxbputc(ostr, cc as libc::c_char);
                                    }
                                }
                                42 => {
                                    sfungetc(str, cc);
                                }
                                47 => {
                                    col0 = 0 as libc::c_int;
                                    return ' ' as i32;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                },
                47 => {
                    c = eol(str);
                }
                _ => {
                    if cc >= '\0' as i32 {
                        sfungetc(str, cc);
                    }
                }
            }
        }
        _ => {
            col0 = 0 as libc::c_int;
        }
    }
    return c;
}
unsafe extern "C" fn unreadc(mut str: *mut Sfio_t, mut c: libc::c_int) {
    sfungetc(str, c);
    if c == '\n' as i32 {
        lineno -= 1;
    }
}
unsafe extern "C" fn skipWS(mut str: *mut Sfio_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        c = readc(str, 0 as *mut agxbuf);
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return c;
        }
    }
}
unsafe extern "C" fn parseID(
    mut str: *mut Sfio_t,
    mut c: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bsize: size_t,
) {
    let mut more: bool = 1 as libc::c_int != 0;
    let mut ptr: *mut libc::c_char = buf;
    let mut eptr: *mut libc::c_char =
        buf.offset(bsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    let fresh22 = ptr;
    ptr = ptr.offset(1);
    *fresh22 = c as libc::c_char;
    while more {
        c = readc(str, 0 as *mut agxbuf);
        if c < 0 as libc::c_int {
            more = 0 as libc::c_int != 0;
        }
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || c == '_' as i32
        {
            if ptr == eptr {
                more = 0 as libc::c_int != 0;
            } else {
                let fresh23 = ptr;
                ptr = ptr.offset(1);
                *fresh23 = c as libc::c_char;
            }
        } else {
            more = 0 as libc::c_int != 0;
            unreadc(str, c);
        }
    }
    *ptr = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn parseKind(mut str: *mut Sfio_t) -> case_t {
    let mut c: libc::c_int = 0;
    let mut buf: [libc::c_char; 8] = [0; 8];
    let mut cs: case_t = Error;
    c = skipWS(str);
    if c < 0 as libc::c_int {
        return Eof;
    }
    if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        _err_msg(
            2 as libc::c_int,
            b"expected keyword BEGIN/END/N/E...; found '%c', line %d\0" as *const u8
                as *const libc::c_char,
            c,
            lineno,
        );
        return Error;
    }
    kwLine = lineno;
    parseID(str, c, buf.as_mut_ptr(), 8 as libc::c_int as size_t);
    if strcmp(
        buf.as_mut_ptr(),
        b"BEGIN\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        cs = Begin;
    } else if strcmp(
        buf.as_mut_ptr(),
        b"BEG_G\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        cs = BeginG;
    } else if strcmp(buf.as_mut_ptr(), b"E\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        cs = Edge;
    } else if strcmp(
        buf.as_mut_ptr(),
        b"END\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        cs = End;
    } else if strcmp(
        buf.as_mut_ptr(),
        b"END_G\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        cs = EndG;
    } else if strcmp(buf.as_mut_ptr(), b"N\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        cs = Node;
    }
    if cs as libc::c_uint == Error as libc::c_int as libc::c_uint {
        _err_msg(
            2 as libc::c_int,
            b"unexpected keyword \"%s\", line %d\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
            kwLine,
        );
    }
    return cs;
}
unsafe extern "C" fn endString(
    mut ins: *mut Sfio_t,
    mut outs: *mut agxbuf,
    mut ec: libc::c_char,
) -> libc::c_int {
    let mut sline: libc::c_int = lineno;
    let mut c: libc::c_int = 0;
    loop {
        c = (if (*ins).next >= (*ins).endr {
            _sffilbuf(ins, 0 as libc::c_int)
        } else {
            let ref mut fresh24 = (*ins).next;
            let fresh25 = *fresh24;
            *fresh24 = (*fresh24).offset(1);
            *fresh25 as libc::c_int
        });
        if !(c != ec as libc::c_int) {
            break;
        }
        if c == '\\' as i32 {
            agxbputc(outs, c as libc::c_char);
            c = if (*ins).next >= (*ins).endr {
                _sffilbuf(ins, 0 as libc::c_int)
            } else {
                let ref mut fresh26 = (*ins).next;
                let fresh27 = *fresh26;
                *fresh26 = (*fresh26).offset(1);
                *fresh27 as libc::c_int
            };
        }
        if c < 0 as libc::c_int {
            _err_msg(
                2 as libc::c_int,
                b"unclosed string, start line %d\0" as *const u8 as *const libc::c_char,
                sline,
            );
            return c;
        }
        if c == '\n' as i32 {
            lineno += 1;
        }
        agxbputc(outs, c as libc::c_char);
    }
    agxbputc(outs, c as libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn endBracket(
    mut ins: *mut Sfio_t,
    mut outs: *mut agxbuf,
    mut bc: libc::c_char,
    mut ec: libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop {
        c = readc(ins, outs);
        if c < 0 as libc::c_int || c == ec as libc::c_int {
            return c;
        } else {
            if c == bc as libc::c_int {
                agxbputc(outs, c as libc::c_char);
                c = endBracket(ins, outs, bc, ec);
                if c < 0 as libc::c_int {
                    return c;
                } else {
                    agxbputc(outs, c as libc::c_char);
                }
            } else if c == '\'' as i32 || c == '"' as i32 {
                agxbputc(outs, c as libc::c_char);
                if endString(ins, outs, c as libc::c_char) != 0 {
                    return -(1 as libc::c_int);
                }
            } else {
                agxbputc(outs, c as libc::c_char);
            }
        }
    }
}
unsafe extern "C" fn parseBracket(
    mut str: *mut Sfio_t,
    mut buf: *mut agxbuf,
    mut bc: libc::c_int,
    mut ec: libc::c_int,
) -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    c = skipWS(str);
    if c < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if c != bc {
        unreadc(str, c);
        return 0 as *mut libc::c_char;
    }
    startLine = lineno;
    c = endBracket(str, buf, bc as libc::c_char, ec as libc::c_char);
    if c < 0 as libc::c_int {
        if getErrorErrors() == 0 {
            _err_msg(
                2 as libc::c_int,
                b"unclosed bracket %c%c expression, start line %d\0" as *const u8
                    as *const libc::c_char,
                bc,
                ec,
                startLine,
            );
        }
        return 0 as *mut libc::c_char;
    } else {
        return agxbdisown(buf);
    };
}
unsafe extern "C" fn parseAction(mut str: *mut Sfio_t, mut buf: *mut agxbuf) -> *mut libc::c_char {
    return parseBracket(str, buf, '{' as i32, '}' as i32);
}
unsafe extern "C" fn parseGuard(mut str: *mut Sfio_t, mut buf: *mut agxbuf) -> *mut libc::c_char {
    return parseBracket(str, buf, '[' as i32, ']' as i32);
}
unsafe extern "C" fn parseCase(
    mut str: *mut Sfio_t,
    mut guard: *mut *mut libc::c_char,
    mut gline: *mut libc::c_int,
    mut action: *mut *mut libc::c_char,
    mut aline: *mut libc::c_int,
) -> case_t {
    let mut kind: case_t = Begin;
    let mut buf: agxbuf = agxbuf {
        buf: 0 as *mut libc::c_char,
        ptr: 0 as *mut libc::c_char,
        eptr: 0 as *mut libc::c_char,
        dyna: 0,
    };
    agxbinit(
        &mut buf,
        0 as libc::c_int as libc::c_uint,
        0 as *mut libc::c_char,
    );
    kind = parseKind(str);
    match kind as libc::c_uint {
        0 | 2 | 1 | 3 => {
            *action = parseAction(str, &mut buf);
            *aline = startLine;
            if getErrorErrors() != 0 {
                kind = Error;
            }
        }
        5 | 4 => {
            *guard = parseGuard(str, &mut buf);
            *gline = startLine;
            if getErrorErrors() == 0 {
                *action = parseAction(str, &mut buf);
                *aline = startLine;
            }
            if getErrorErrors() != 0 {
                kind = Error;
            }
        }
        6 | 7 | _ => {}
    }
    agxbfree(&mut buf);
    return kind;
}
unsafe extern "C" fn addBlock(
    mut last: *mut parse_block,
    mut stmt: *mut libc::c_char,
    mut line: libc::c_int,
    mut n_nstmts: libc::c_int,
    mut nodelist: *mut case_info,
    mut n_estmts: libc::c_int,
    mut edgelist: *mut case_info,
) -> *mut parse_block {
    let mut item: *mut parse_block = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<parse_block>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut parse_block
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<parse_block>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut parse_block
    };
    (*item).l_beging = line;
    let ref mut fresh28 = (*item).begg_stmt;
    *fresh28 = stmt;
    (*item).n_nstmts = n_nstmts;
    (*item).n_estmts = n_estmts;
    let ref mut fresh29 = (*item).node_stmts;
    *fresh29 = nodelist;
    let ref mut fresh30 = (*item).edge_stmts;
    *fresh30 = edgelist;
    if !last.is_null() {
        let ref mut fresh31 = (*last).next;
        *fresh31 = item;
    }
    return item;
}
unsafe extern "C" fn addCase(
    mut last: *mut case_info,
    mut guard: *mut libc::c_char,
    mut gline: libc::c_int,
    mut action: *mut libc::c_char,
    mut line: libc::c_int,
    mut cnt: *mut libc::c_int,
) -> *mut case_info {
    let mut item: *mut case_info = 0 as *mut case_info;
    if guard.is_null() && action.is_null() {
        _err_msg(
            1 as libc::c_int,
            b"Case with neither guard nor action, line %d - ignored\0" as *const u8
                as *const libc::c_char,
            kwLine,
        );
        return last;
    }
    *cnt += 1;
    item = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<case_info>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut case_info
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<case_info>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut case_info
    };
    let ref mut fresh32 = (*item).guard;
    *fresh32 = guard;
    let ref mut fresh33 = (*item).action;
    *fresh33 = action;
    let ref mut fresh34 = (*item).next;
    *fresh34 = 0 as *mut _case_info;
    if !guard.is_null() {
        (*item).gstart = gline;
    }
    if !action.is_null() {
        (*item).astart = line;
    }
    if !last.is_null() {
        let ref mut fresh35 = (*last).next;
        *fresh35 = item;
    }
    return item;
}
unsafe extern "C" fn bindAction(
    mut cs: case_t,
    mut action: *mut libc::c_char,
    mut aline: libc::c_int,
    mut ap: *mut *mut libc::c_char,
    mut lp: *mut libc::c_int,
) {
    if action.is_null() {
        _err_msg(
            1 as libc::c_int,
            b"%s with no action, line %d - ignored\0" as *const u8 as *const libc::c_char,
            caseStr(cs),
            kwLine,
        );
    } else if !(*ap).is_null() {
        _err_msg(
            2 as libc::c_int,
            b"additional %s section, line %d\0" as *const u8 as *const libc::c_char,
            caseStr(cs),
            kwLine,
        );
    } else {
        *ap = action;
        *lp = aline;
    };
}
#[no_mangle]
pub unsafe extern "C" fn parseProg(
    mut input: *mut libc::c_char,
    mut isFile: libc::c_int,
) -> *mut parse_prog {
    let mut prog: *mut parse_prog = 0 as *mut parse_prog;
    let mut str: *mut Sfio_t = 0 as *mut Sfio_t;
    let mut mode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut guard: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut action: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut more: bool = false;
    let mut blocklist: *mut parse_block = 0 as *mut parse_block;
    let mut edgelist: *mut case_info = 0 as *mut case_info;
    let mut nodelist: *mut case_info = 0 as *mut case_info;
    let mut blockl: *mut parse_block = 0 as *mut parse_block;
    let mut edgel: *mut case_info = 0 as *mut case_info;
    let mut nodel: *mut case_info = 0 as *mut case_info;
    let mut n_blocks: libc::c_int = 0 as libc::c_int;
    let mut n_nstmts: libc::c_int = 0 as libc::c_int;
    let mut n_estmts: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 0 as libc::c_int;
    let mut gline: libc::c_int = 0 as libc::c_int;
    let mut l_beging: libc::c_int = 0 as libc::c_int;
    let mut begg_stmt: *mut libc::c_char = 0 as *mut libc::c_char;
    kwLine = 1 as libc::c_int;
    startLine = kwLine;
    col0 = startLine;
    lineno = col0;
    prog = if 0 as libc::c_int != 0 {
        realloc(
            0 as *mut libc::c_char as *mut libc::c_void,
            (::std::mem::size_of::<parse_prog>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut parse_prog
    } else {
        calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<parse_prog>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as libc::c_ulong),
        ) as *mut parse_prog
    };
    if prog.is_null() {
        _err_msg(
            2 as libc::c_int,
            b"parseProg: out of memory\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut parse_prog;
    }
    if isFile != 0 {
        mode = b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        let ref mut fresh36 = (*prog).source;
        *fresh36 = input;
    } else {
        mode = b"rs\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        let ref mut fresh37 = (*prog).source;
        *fresh37 = 0 as *mut libc::c_char;
    }
    str = sfopen(0 as *mut Sfio_t, input, mode);
    if str.is_null() {
        if isFile != 0 {
            _err_msg(
                2 as libc::c_int,
                b"could not open %s for reading\0" as *const u8 as *const libc::c_char,
                input,
            );
        } else {
            _err_msg(
                2 as libc::c_int,
                b"parseProg : unable to create sfio stream\0" as *const u8 as *const libc::c_char,
            );
        }
        free(prog as *mut libc::c_void);
        return 0 as *mut parse_prog;
    }
    begg_stmt = 0 as *mut libc::c_char;
    more = 1 as libc::c_int != 0;
    while more {
        match parseCase(str, &mut guard, &mut gline, &mut action, &mut line) as libc::c_uint {
            0 => {
                bindAction(
                    Begin,
                    action,
                    line,
                    &mut (*prog).begin_stmt,
                    &mut (*prog).l_begin,
                );
            }
            2 => {
                if !action.is_null()
                    && (!begg_stmt.is_null() || !nodelist.is_null() || !edgelist.is_null())
                {
                    blockl = addBlock(
                        blockl, begg_stmt, l_beging, n_nstmts, nodelist, n_estmts, edgelist,
                    );
                    if blocklist.is_null() {
                        blocklist = blockl;
                    }
                    n_blocks += 1;
                    n_estmts = 0 as libc::c_int;
                    n_nstmts = n_estmts;
                    nodelist = 0 as *mut case_info;
                    edgelist = nodelist;
                    nodel = edgelist;
                    edgel = nodel;
                    begg_stmt = 0 as *mut libc::c_char;
                }
                bindAction(BeginG, action, line, &mut begg_stmt, &mut l_beging);
            }
            1 => {
                bindAction(End, action, line, &mut (*prog).end_stmt, &mut (*prog).l_end);
            }
            3 => {
                bindAction(
                    EndG,
                    action,
                    line,
                    &mut (*prog).endg_stmt,
                    &mut (*prog).l_endg,
                );
            }
            6 => {
                more = 0 as libc::c_int != 0;
            }
            4 => {
                nodel = addCase(nodel, guard, gline, action, line, &mut n_nstmts);
                if nodelist.is_null() {
                    nodelist = nodel;
                }
            }
            5 => {
                edgel = addCase(edgel, guard, gline, action, line, &mut n_estmts);
                if edgelist.is_null() {
                    edgelist = edgel;
                }
            }
            7 => {
                more = 0 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if !begg_stmt.is_null() || !nodelist.is_null() || !edgelist.is_null() {
        blockl = addBlock(
            blockl, begg_stmt, l_beging, n_nstmts, nodelist, n_estmts, edgelist,
        );
        if blocklist.is_null() {
            blocklist = blockl;
        }
        n_blocks += 1;
    }
    (*prog).n_blocks = n_blocks;
    let ref mut fresh38 = (*prog).blocks;
    *fresh38 = blocklist;
    sfclose(str);
    if getErrorErrors() != 0 {
        freeParseProg(prog);
        prog = 0 as *mut parse_prog;
    }
    return prog;
}
unsafe extern "C" fn freeCaseList(mut ip: *mut case_info) {
    let mut nxt: *mut case_info = 0 as *mut case_info;
    while !ip.is_null() {
        nxt = (*ip).next;
        free((*ip).guard as *mut libc::c_void);
        free((*ip).action as *mut libc::c_void);
        free(ip as *mut libc::c_void);
        ip = nxt;
    }
}
unsafe extern "C" fn freeBlocks(mut ip: *mut parse_block) {
    let mut nxt: *mut parse_block = 0 as *mut parse_block;
    while !ip.is_null() {
        nxt = (*ip).next;
        free((*ip).begg_stmt as *mut libc::c_void);
        freeCaseList((*ip).node_stmts);
        freeCaseList((*ip).edge_stmts);
        ip = nxt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn freeParseProg(mut prog: *mut parse_prog) {
    if prog.is_null() {
        return;
    }
    free((*prog).begin_stmt as *mut libc::c_void);
    freeBlocks((*prog).blocks);
    free((*prog).endg_stmt as *mut libc::c_void);
    free((*prog).end_stmt as *mut libc::c_void);
    free(prog as *mut libc::c_void);
}
