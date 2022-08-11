#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct xml_flags_t {
    #[bitfield(name = "raw", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "dash", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "nbsp", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "utf8", ty = "libc::c_uint", bits = "3..=3")]
    pub raw_dash_nbsp_utf8: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn graphviz_exit(mut status: libc::c_int) -> ! {
    exit(status);
}
#[inline]
unsafe extern "C" fn isalpha_no_locale(mut c: libc::c_char) -> bool {
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
        return 1 as libc::c_int != 0;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn xml_isentity(mut s: *const libc::c_char) -> bool {
    s = s.offset(1);
    if *s as libc::c_int == ';' as i32 {
        return 0 as libc::c_int != 0;
    }
    if *s as libc::c_int == '#' as i32 {
        s = s.offset(1);
        if *s as libc::c_int == 'x' as i32 || *s as libc::c_int == 'X' as i32 {
            s = s.offset(1);
            while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                s = s.offset(1);
            }
        } else {
            while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                s = s.offset(1);
            }
        }
    } else {
        while isalpha_no_locale(*s) {
            s = s.offset(1);
        }
    }
    if *s as libc::c_int == ';' as i32 {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn xml_core(
    mut previous: libc::c_char,
    mut current: *mut *const libc::c_char,
    mut flags: xml_flags_t,
    mut cb: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    mut state: *mut libc::c_void,
) -> libc::c_int {
    let mut s: *const libc::c_char = *current;
    let mut c: libc::c_char = *s;
    *current = (*current).offset(1);
    if c as libc::c_int == '&' as i32
        && (flags.raw() as libc::c_int != 0 || !xml_isentity(s))
    {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&amp;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == '<' as i32 {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&lt;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == '>' as i32 {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&gt;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == '-' as i32 && flags.dash() as libc::c_int != 0 {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&#45;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == ' ' as i32 && previous as libc::c_int == ' ' as i32
        && flags.nbsp() as libc::c_int != 0
    {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&#160;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == '"' as i32 {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&quot;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == '\'' as i32 {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&#39;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == '\n' as i32 && flags.raw() as libc::c_int != 0 {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&#10;\0" as *const u8 as *const libc::c_char);
    }
    if c as libc::c_int == '\r' as i32 && flags.raw() as libc::c_int != 0 {
        return cb
            .expect(
                "non-null function pointer",
            )(state, b"&#13;\0" as *const u8 as *const libc::c_char);
    }
    let mut uc: libc::c_uchar = c as libc::c_uchar;
    if uc as libc::c_int > 0x7f as libc::c_int && flags.utf8() as libc::c_int != 0 {
        let mut length: size_t = (if uc as libc::c_int >> 5 as libc::c_int
            == 6 as libc::c_int
        {
            2 as libc::c_int
        } else if uc as libc::c_int >> 4 as libc::c_int == 14 as libc::c_int {
            3 as libc::c_int
        } else if uc as libc::c_int >> 3 as libc::c_int == 30 as libc::c_int {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as size_t;
        let mut is_invalid: bool = length == 0 as libc::c_int as libc::c_ulong;
        let mut l: size_t = 1 as libc::c_int as size_t;
        while !is_invalid && length > l {
            is_invalid = (is_invalid as libc::c_int
                | (*s.offset(l as isize) as libc::c_int == '\0' as i32) as libc::c_int)
                as bool;
            l = l.wrapping_add(1);
        }
        if is_invalid {
            fprintf(
                stderr,
                b"Error during conversion to \"UTF-8\". Quiting.\n\0" as *const u8
                    as *const libc::c_char,
            );
            graphviz_exit(1 as libc::c_int);
        }
        let mut utf8_char: uint32_t = 0 as libc::c_int as uint32_t;
        match length {
            2 => {
                let mut low: uint32_t = *s.offset(1 as libc::c_int as isize) as uint32_t
                    & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                let mut high: uint32_t = *s.offset(0 as libc::c_int as isize) as uint32_t
                    & (((1 as libc::c_int) << 5 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                utf8_char = low | high << 6 as libc::c_int;
            }
            3 => {
                let mut low_0: uint32_t = *s.offset(2 as libc::c_int as isize)
                    as uint32_t
                    & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                let mut mid: uint32_t = *s.offset(1 as libc::c_int as isize) as uint32_t
                    & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                let mut high_0: uint32_t = *s.offset(0 as libc::c_int as isize)
                    as uint32_t
                    & (((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                utf8_char = low_0 | mid << 6 as libc::c_int
                    | high_0 << 12 as libc::c_int;
            }
            4 => {
                let mut low_1: uint32_t = *s.offset(3 as libc::c_int as isize)
                    as uint32_t
                    & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                let mut mid1: uint32_t = *s.offset(2 as libc::c_int as isize) as uint32_t
                    & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                let mut mid2: uint32_t = *s.offset(1 as libc::c_int as isize) as uint32_t
                    & (((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                let mut high_1: uint32_t = *s.offset(0 as libc::c_int as isize)
                    as uint32_t
                    & (((1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint;
                utf8_char = low_1 | mid1 << 6 as libc::c_int | mid2 << 12 as libc::c_int
                    | high_1 << 18 as libc::c_int;
            }
            _ => {
                fprintf(
                    stderr,
                    b"%s:%d: claimed unreachable code was reached\0" as *const u8
                        as *const libc::c_char,
                    b"xml.c\0" as *const u8 as *const libc::c_char,
                    152 as libc::c_int,
                );
                abort();
            }
        }
        let mut buffer: [libc::c_char; 13] = [0; 13];
        snprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
            b"&#x%x;\0" as *const u8 as *const libc::c_char,
            utf8_char,
        );
        *current = (*current)
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        return cb.expect("non-null function pointer")(state, buffer.as_mut_ptr());
    }
    let mut buffer_0: [libc::c_char; 2] = [c, '\0' as i32 as libc::c_char];
    return cb.expect("non-null function pointer")(state, buffer_0.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn xml_escape(
    mut s: *const libc::c_char,
    mut flags: xml_flags_t,
    mut cb: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    mut state: *mut libc::c_void,
) -> libc::c_int {
    let mut previous: libc::c_char = '\0' as i32 as libc::c_char;
    let mut rc: libc::c_int = 0 as libc::c_int;
    while *s as libc::c_int != '\0' as i32 {
        let mut p: libc::c_char = *s;
        rc = xml_core(previous, &mut s, flags, cb, state);
        if rc < 0 as libc::c_int {
            return rc;
        }
        previous = p;
    }
    return rc;
}
