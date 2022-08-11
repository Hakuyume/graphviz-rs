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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
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
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsbcolor_t {
    pub name: *mut libc::c_char,
    pub h: libc::c_uchar,
    pub s: libc::c_uchar,
    pub b: libc::c_uchar,
}
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l
            .wrapping_add(__u)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void;
        }
    }
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut color_lib: [hsbcolor_t; 652] = [
    {
        let mut init = hsbcolor_t {
            name: b"aliceblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 147 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"antiquewhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 24 as libc::c_int as libc::c_uchar,
            s: 35 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"antiquewhite1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 36 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"antiquewhite2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 36 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"antiquewhite3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 36 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"antiquewhite4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 24 as libc::c_int as libc::c_uchar,
            s: 34 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"aquamarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 113 as libc::c_int as libc::c_uchar,
            s: 128 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"aquamarine1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 113 as libc::c_int as libc::c_uchar,
            s: 128 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"aquamarine2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 113 as libc::c_int as libc::c_uchar,
            s: 128 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"aquamarine3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 113 as libc::c_int as libc::c_uchar,
            s: 128 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"aquamarine4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 113 as libc::c_int as libc::c_uchar,
            s: 128 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"azure\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"azure1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"azure2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"azure3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"azure4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"beige\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 26 as libc::c_int as libc::c_uchar,
            b: 245 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"bisque\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 58 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"bisque1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 58 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"bisque2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 58 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"bisque3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 22 as libc::c_int as libc::c_uchar,
            s: 58 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"bisque4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 58 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"black\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"blanchedalmond\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 25 as libc::c_int as libc::c_uchar,
            s: 49 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"blue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"blue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"blue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"blue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"blueviolet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 192 as libc::c_int as libc::c_uchar,
            s: 206 as libc::c_int as libc::c_uchar,
            b: 226 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"brown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 190 as libc::c_int as libc::c_uchar,
            b: 165 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"brown1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 191 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"brown2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 191 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"brown3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 191 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"brown4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 190 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"burlywood\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 99 as libc::c_int as libc::c_uchar,
            b: 222 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"burlywood1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 100 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"burlywood2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 99 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"burlywood3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 99 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"burlywood4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 99 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cadetblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 128 as libc::c_int as libc::c_uchar,
            s: 103 as libc::c_int as libc::c_uchar,
            b: 160 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cadetblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 131 as libc::c_int as libc::c_uchar,
            s: 103 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cadetblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 131 as libc::c_int as libc::c_uchar,
            s: 102 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cadetblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 131 as libc::c_int as libc::c_uchar,
            s: 103 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cadetblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 131 as libc::c_int as libc::c_uchar,
            s: 102 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chartreuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 63 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chartreuse1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 63 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chartreuse2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 63 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chartreuse3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 63 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chartreuse4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 63 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chocolate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 218 as libc::c_int as libc::c_uchar,
            b: 210 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chocolate1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 219 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chocolate2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 219 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chocolate3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 218 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"chocolate4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 220 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"coral\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 11 as libc::c_int as libc::c_uchar,
            s: 175 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"coral1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 7 as libc::c_int as libc::c_uchar,
            s: 169 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"coral2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 169 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"coral3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 169 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"coral4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 168 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cornflowerblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 154 as libc::c_int as libc::c_uchar,
            s: 147 as libc::c_int as libc::c_uchar,
            b: 237 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cornsilk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 33 as libc::c_int as libc::c_uchar,
            s: 34 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cornsilk1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 33 as libc::c_int as libc::c_uchar,
            s: 34 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cornsilk2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 34 as libc::c_int as libc::c_uchar,
            s: 35 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cornsilk3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 34 as libc::c_int as libc::c_uchar,
            s: 34 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cornsilk4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 34 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"crimson\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 246 as libc::c_int as libc::c_uchar,
            s: 231 as libc::c_int as libc::c_uchar,
            b: 220 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cyan1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cyan2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cyan3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"cyan4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkgoldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 239 as libc::c_int as libc::c_uchar,
            b: 184 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkgoldenrod1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 240 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkgoldenrod2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 240 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkgoldenrod3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 240 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkgoldenrod4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 240 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 100 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkkhaki\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 39 as libc::c_int as libc::c_uchar,
            s: 110 as libc::c_int as libc::c_uchar,
            b: 189 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkolivegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 58 as libc::c_int as libc::c_uchar,
            s: 142 as libc::c_int as libc::c_uchar,
            b: 107 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkolivegreen1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 58 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkolivegreen2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 58 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkolivegreen3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 58 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkolivegreen4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 58 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 23 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorange1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 21 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorange2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 21 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorange3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 21 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorange4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 21 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 198 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 204 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorchid1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 198 as libc::c_int as libc::c_uchar,
            s: 193 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorchid2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 198 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorchid3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 198 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkorchid4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 198 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darksalmon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 10 as libc::c_int as libc::c_uchar,
            s: 121 as libc::c_int as libc::c_uchar,
            b: 233 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkseagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 61 as libc::c_int as libc::c_uchar,
            b: 188 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkseagreen1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkseagreen2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkseagreen3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkseagreen4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkslateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 175 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkslategray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 103 as libc::c_int as libc::c_uchar,
            b: 79 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkslategray1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 104 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkslategray2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 103 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkslategray3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 104 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkslategray4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 104 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkslategrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 103 as libc::c_int as libc::c_uchar,
            b: 79 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 128 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 209 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"darkviolet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 199 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 211 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deeppink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 232 as libc::c_int as libc::c_uchar,
            s: 235 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deeppink1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 232 as libc::c_int as libc::c_uchar,
            s: 235 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deeppink2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 232 as libc::c_int as libc::c_uchar,
            s: 235 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deeppink3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 232 as libc::c_int as libc::c_uchar,
            s: 235 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deeppink4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 231 as libc::c_int as libc::c_uchar,
            s: 236 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deepskyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deepskyblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deepskyblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deepskyblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"deepskyblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"dimgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 105 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"dimgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 105 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"dodgerblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 225 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"dodgerblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 225 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"dodgerblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 225 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"dodgerblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 225 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"dodgerblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 225 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"firebrick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 206 as libc::c_int as libc::c_uchar,
            b: 178 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"firebrick1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"firebrick2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"firebrick3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"firebrick4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"floralwhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 28 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"forestgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gainsboro\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 220 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"ghostwhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 7 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gold1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gold2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gold3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gold4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"goldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 217 as libc::c_int as libc::c_uchar,
            b: 218 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"goldenrod1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 218 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"goldenrod2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 218 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"goldenrod3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 218 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"goldenrod4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 30 as libc::c_int as libc::c_uchar,
            s: 218 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 192 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 26 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 31 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 33 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 36 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 38 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 41 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray17\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 43 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray18\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 46 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray19\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 48 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 51 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray21\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 54 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray22\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 56 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray23\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 59 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray24\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 61 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray25\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 64 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray26\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 66 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray27\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 69 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray28\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 71 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray29\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 74 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray30\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 77 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray31\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 79 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 82 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray33\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 84 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray34\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 87 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray35\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 89 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray36\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 92 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray37\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 94 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray38\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 97 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray39\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 99 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray40\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 102 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray41\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 105 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray42\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 107 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray43\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 110 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray44\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 112 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray45\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 115 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray46\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 117 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray47\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 120 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray48\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 122 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray49\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 125 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray50\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 127 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray51\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 130 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray52\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 133 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray53\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 135 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray54\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 138 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray55\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 140 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray56\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 143 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray57\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 145 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray58\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 148 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray59\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 150 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 15 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray60\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 153 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray61\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 156 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray62\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 158 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray63\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 161 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray64\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 163 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray65\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 166 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray66\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 168 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray67\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 171 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray68\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 173 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray69\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 176 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 18 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray70\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 179 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray71\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 181 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray72\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 184 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray73\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 186 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray74\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 189 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray75\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 191 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray76\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 194 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray77\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 196 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray78\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 199 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray79\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 201 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 20 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray80\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 204 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray81\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 207 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray82\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 209 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray83\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 212 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray84\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 214 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray85\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 217 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray86\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 219 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray87\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 222 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray88\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 224 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray89\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 227 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 23 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray90\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 229 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray91\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 232 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray92\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 235 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray93\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 237 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray94\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 240 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray95\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 242 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray96\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 245 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray97\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 247 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray98\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"gray99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 252 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"green\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"green1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"green2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"green3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"green4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"greenyellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 59 as libc::c_int as libc::c_uchar,
            s: 208 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 192 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 26 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 28 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 31 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 33 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 36 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 38 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 41 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey17\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 43 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey18\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 46 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey19\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 48 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 51 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey21\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 54 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey22\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 56 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey23\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 59 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey24\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 61 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey25\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 64 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey26\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 66 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey27\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 69 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey28\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 71 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey29\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 74 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey30\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 77 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey31\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 79 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 82 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey33\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 84 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey34\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 87 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey35\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 89 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey36\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 92 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey37\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 94 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey38\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 97 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey39\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 99 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey40\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 102 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey41\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 105 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey42\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 107 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey43\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 110 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey44\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 112 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey45\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 115 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey46\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 117 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey47\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 120 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey48\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 122 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey49\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 125 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey50\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 127 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey51\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 130 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey52\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 133 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey53\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 135 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey54\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 138 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey55\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 140 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey56\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 143 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey57\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 145 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey58\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 148 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey59\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 150 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 15 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey60\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 153 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey61\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 156 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey62\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 158 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey63\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 161 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey64\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 163 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey65\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 166 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey66\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 168 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey67\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 171 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey68\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 173 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey69\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 176 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 18 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey70\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 179 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey71\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 181 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey72\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 184 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey73\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 186 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey74\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 189 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey75\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 191 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey76\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 194 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey77\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 196 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey78\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 199 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey79\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 201 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 20 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey80\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 204 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey81\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 207 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey82\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 209 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey83\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 212 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey84\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 214 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey85\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 217 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey86\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 219 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey87\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 222 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey88\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 224 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey89\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 227 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 23 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey90\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 229 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey91\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 232 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey92\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 235 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey93\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 237 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey94\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 240 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey95\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 242 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey96\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 245 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey97\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 247 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey98\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"grey99\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 252 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"honeydew\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"honeydew1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"honeydew2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"honeydew3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"honeydew4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"hotpink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 233 as libc::c_int as libc::c_uchar,
            s: 150 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"hotpink1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 234 as libc::c_int as libc::c_uchar,
            s: 145 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"hotpink2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 235 as libc::c_int as libc::c_uchar,
            s: 141 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"hotpink3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 236 as libc::c_int as libc::c_uchar,
            s: 135 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"hotpink4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 234 as libc::c_int as libc::c_uchar,
            s: 148 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"indianred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 140 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"indianred1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 148 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"indianred2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 148 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"indianred3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 149 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"indianred4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 148 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"indigo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 194 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 130 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"ivory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"ivory1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"ivory2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"ivory3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"ivory4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"khaki\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 38 as libc::c_int as libc::c_uchar,
            s: 106 as libc::c_int as libc::c_uchar,
            b: 240 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"khaki1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 39 as libc::c_int as libc::c_uchar,
            s: 112 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"khaki2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 39 as libc::c_int as libc::c_uchar,
            s: 112 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"khaki3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 39 as libc::c_int as libc::c_uchar,
            s: 111 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"khaki4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 39 as libc::c_int as libc::c_uchar,
            s: 111 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lavender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 20 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lavenderblush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 240 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lavenderblush1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 240 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lavenderblush2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 239 as libc::c_int as libc::c_uchar,
            s: 15 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lavenderblush3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 240 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lavenderblush4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 239 as libc::c_int as libc::c_uchar,
            s: 14 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lawngreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 64 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 252 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lemonchiffon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 38 as libc::c_int as libc::c_uchar,
            s: 49 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lemonchiffon1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 38 as libc::c_int as libc::c_uchar,
            s: 49 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lemonchiffon2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 37 as libc::c_int as libc::c_uchar,
            s: 50 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lemonchiffon3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 38 as libc::c_int as libc::c_uchar,
            s: 49 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lemonchiffon4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 39 as libc::c_int as libc::c_uchar,
            s: 49 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 137 as libc::c_int as libc::c_uchar,
            s: 63 as libc::c_int as libc::c_uchar,
            b: 230 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 64 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 64 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 138 as libc::c_int as libc::c_uchar,
            s: 63 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 137 as libc::c_int as libc::c_uchar,
            s: 64 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightcoral\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 119 as libc::c_int as libc::c_uchar,
            b: 240 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightcyan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightcyan1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightcyan2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightcyan3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightcyan4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgoldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 115 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgoldenrod1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 116 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgoldenrod2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 115 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgoldenrod3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 115 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgoldenrod4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 35 as libc::c_int as libc::c_uchar,
            s: 115 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgoldenrodyellow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 40 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 211 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightgrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 211 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightpink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 248 as libc::c_int as libc::c_uchar,
            s: 73 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightpink1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 249 as libc::c_int as libc::c_uchar,
            s: 81 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightpink2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 248 as libc::c_int as libc::c_uchar,
            s: 81 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightpink3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 249 as libc::c_int as libc::c_uchar,
            s: 80 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightpink4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 249 as libc::c_int as libc::c_uchar,
            s: 80 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsalmon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 12 as libc::c_int as libc::c_uchar,
            s: 132 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsalmon1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 12 as libc::c_int as libc::c_uchar,
            s: 132 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsalmon2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 11 as libc::c_int as libc::c_uchar,
            s: 132 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsalmon3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 12 as libc::c_int as libc::c_uchar,
            s: 133 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsalmon4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 12 as libc::c_int as libc::c_uchar,
            s: 133 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightseagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 125 as libc::c_int as libc::c_uchar,
            s: 209 as libc::c_int as libc::c_uchar,
            b: 178 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightskyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 143 as libc::c_int as libc::c_uchar,
            s: 117 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightskyblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 143 as libc::c_int as libc::c_uchar,
            s: 79 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightskyblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 143 as libc::c_int as libc::c_uchar,
            s: 79 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightskyblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 142 as libc::c_int as libc::c_uchar,
            s: 79 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightskyblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 143 as libc::c_int as libc::c_uchar,
            s: 78 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightslateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 175 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightslategray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 56 as libc::c_int as libc::c_uchar,
            b: 153 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightslategrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 56 as libc::c_int as libc::c_uchar,
            b: 153 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsteelblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 151 as libc::c_int as libc::c_uchar,
            s: 52 as libc::c_int as libc::c_uchar,
            b: 222 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsteelblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 151 as libc::c_int as libc::c_uchar,
            s: 53 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsteelblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 151 as libc::c_int as libc::c_uchar,
            s: 53 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsteelblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 151 as libc::c_int as libc::c_uchar,
            s: 53 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightsteelblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 150 as libc::c_int as libc::c_uchar,
            s: 53 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightyellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightyellow1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightyellow2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightyellow3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"lightyellow4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 31 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"limegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"linen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 21 as libc::c_int as libc::c_uchar,
            s: 20 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"magenta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"magenta1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"magenta2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"magenta3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"magenta4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"maroon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 239 as libc::c_int as libc::c_uchar,
            s: 185 as libc::c_int as libc::c_uchar,
            b: 176 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"maroon1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 228 as libc::c_int as libc::c_uchar,
            s: 203 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"maroon2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 228 as libc::c_int as libc::c_uchar,
            s: 203 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"maroon3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 228 as libc::c_int as libc::c_uchar,
            s: 204 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"maroon4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 228 as libc::c_int as libc::c_uchar,
            s: 203 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumaquamarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 113 as libc::c_int as libc::c_uchar,
            s: 128 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumorchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 204 as libc::c_int as libc::c_uchar,
            s: 152 as libc::c_int as libc::c_uchar,
            b: 211 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumorchid1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 203 as libc::c_int as libc::c_uchar,
            s: 153 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumorchid2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 203 as libc::c_int as libc::c_uchar,
            s: 153 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumorchid3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 203 as libc::c_int as libc::c_uchar,
            s: 153 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumorchid4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 203 as libc::c_int as libc::c_uchar,
            s: 154 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumpurple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 183 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 219 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumpurple1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 183 as libc::c_int as libc::c_uchar,
            s: 125 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumpurple2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 183 as libc::c_int as libc::c_uchar,
            s: 125 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumpurple3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 183 as libc::c_int as libc::c_uchar,
            s: 125 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumpurple4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 183 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumseagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 103 as libc::c_int as libc::c_uchar,
            s: 169 as libc::c_int as libc::c_uchar,
            b: 179 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumslateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 176 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumspringgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 111 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 125 as libc::c_int as libc::c_uchar,
            s: 167 as libc::c_int as libc::c_uchar,
            b: 209 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mediumvioletred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 228 as libc::c_int as libc::c_uchar,
            s: 228 as libc::c_int as libc::c_uchar,
            b: 199 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"midnightblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 198 as libc::c_int as libc::c_uchar,
            b: 112 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mintcream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 106 as libc::c_int as libc::c_uchar,
            s: 9 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mistyrose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 4 as libc::c_int as libc::c_uchar,
            s: 30 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mistyrose1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 4 as libc::c_int as libc::c_uchar,
            s: 30 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mistyrose2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 4 as libc::c_int as libc::c_uchar,
            s: 30 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mistyrose3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 3 as libc::c_int as libc::c_uchar,
            s: 29 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"mistyrose4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 5 as libc::c_int as libc::c_uchar,
            s: 29 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"moccasin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 26 as libc::c_int as libc::c_uchar,
            s: 73 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"navajowhite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 25 as libc::c_int as libc::c_uchar,
            s: 81 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"navajowhite1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 25 as libc::c_int as libc::c_uchar,
            s: 81 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"navajowhite2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 25 as libc::c_int as libc::c_uchar,
            s: 82 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"navajowhite3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 25 as libc::c_int as libc::c_uchar,
            s: 82 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"navajowhite4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 25 as libc::c_int as libc::c_uchar,
            s: 82 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"navy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 128 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"navyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 170 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 128 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"oldlace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 23 as libc::c_int as libc::c_uchar,
            b: 253 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"olivedrab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 56 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 142 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"olivedrab1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 56 as libc::c_int as libc::c_uchar,
            s: 193 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"olivedrab2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 56 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"olivedrab3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 56 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"olivedrab4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 56 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orange\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orange1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orange2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orange3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orange4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orangered\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 11 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orangered1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 11 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orangered2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 11 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orangered3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 11 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orangered4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 11 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orchid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 214 as libc::c_int as libc::c_uchar,
            s: 123 as libc::c_int as libc::c_uchar,
            b: 218 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orchid1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 214 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orchid2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 214 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orchid3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 214 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"orchid4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 213 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palegoldenrod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 38 as libc::c_int as libc::c_uchar,
            s: 72 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palegreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 100 as libc::c_int as libc::c_uchar,
            b: 251 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palegreen1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 101 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palegreen2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 100 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palegreen3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 100 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palegreen4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 85 as libc::c_int as libc::c_uchar,
            s: 100 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"paleturquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 67 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"paleturquoise1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"paleturquoise2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"paleturquoise3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"paleturquoise4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 127 as libc::c_int as libc::c_uchar,
            s: 67 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palevioletred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 241 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 219 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palevioletred1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 241 as libc::c_int as libc::c_uchar,
            s: 125 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palevioletred2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 241 as libc::c_int as libc::c_uchar,
            s: 125 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palevioletred3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 241 as libc::c_int as libc::c_uchar,
            s: 125 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"palevioletred4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 241 as libc::c_int as libc::c_uchar,
            s: 124 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"papayawhip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 26 as libc::c_int as libc::c_uchar,
            s: 41 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"peachpuff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 70 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"peachpuff1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 70 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"peachpuff2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 19 as libc::c_int as libc::c_uchar,
            s: 69 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"peachpuff3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 19 as libc::c_int as libc::c_uchar,
            s: 69 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"peachpuff4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 69 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"peru\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 176 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"pink\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 247 as libc::c_int as libc::c_uchar,
            s: 63 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"pink1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 245 as libc::c_int as libc::c_uchar,
            s: 73 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"pink2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 245 as libc::c_int as libc::c_uchar,
            s: 73 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"pink3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 245 as libc::c_int as libc::c_uchar,
            s: 74 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"pink4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 245 as libc::c_int as libc::c_uchar,
            s: 73 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"plum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 70 as libc::c_int as libc::c_uchar,
            b: 221 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"plum1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"plum2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"plum3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"plum4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 67 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"powderblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 132 as libc::c_int as libc::c_uchar,
            s: 59 as libc::c_int as libc::c_uchar,
            b: 230 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"purple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 196 as libc::c_int as libc::c_uchar,
            s: 221 as libc::c_int as libc::c_uchar,
            b: 240 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"purple1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 191 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"purple2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 192 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"purple3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 192 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"purple4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 192 as libc::c_int as libc::c_uchar,
            s: 207 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"red1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"red2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"red3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"red4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"rosybrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 61 as libc::c_int as libc::c_uchar,
            b: 188 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"rosybrown1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"rosybrown2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"rosybrown3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"rosybrown4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 62 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"royalblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 159 as libc::c_int as libc::c_uchar,
            s: 181 as libc::c_int as libc::c_uchar,
            b: 225 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"royalblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 159 as libc::c_int as libc::c_uchar,
            s: 183 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"royalblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 159 as libc::c_int as libc::c_uchar,
            s: 183 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"royalblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 159 as libc::c_int as libc::c_uchar,
            s: 182 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"royalblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 159 as libc::c_int as libc::c_uchar,
            s: 183 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"saddlebrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 220 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"salmon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 4 as libc::c_int as libc::c_uchar,
            s: 138 as libc::c_int as libc::c_uchar,
            b: 250 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"salmon1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 9 as libc::c_int as libc::c_uchar,
            s: 150 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"salmon2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 9 as libc::c_int as libc::c_uchar,
            s: 150 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"salmon3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 9 as libc::c_int as libc::c_uchar,
            s: 150 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"salmon4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 9 as libc::c_int as libc::c_uchar,
            s: 150 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"sandybrown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 19 as libc::c_int as libc::c_uchar,
            s: 154 as libc::c_int as libc::c_uchar,
            b: 244 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seagreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 103 as libc::c_int as libc::c_uchar,
            s: 170 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seagreen1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 103 as libc::c_int as libc::c_uchar,
            s: 171 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seagreen2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 103 as libc::c_int as libc::c_uchar,
            s: 171 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seagreen3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 103 as libc::c_int as libc::c_uchar,
            s: 171 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seagreen4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 103 as libc::c_int as libc::c_uchar,
            s: 170 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seashell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 16 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seashell1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 17 as libc::c_int as libc::c_uchar,
            s: 16 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seashell2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 18 as libc::c_int as libc::c_uchar,
            s: 17 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seashell3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 18 as libc::c_int as libc::c_uchar,
            s: 17 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"seashell4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 18 as libc::c_int as libc::c_uchar,
            s: 16 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"sienna\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 13 as libc::c_int as libc::c_uchar,
            s: 183 as libc::c_int as libc::c_uchar,
            b: 160 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"sienna1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 13 as libc::c_int as libc::c_uchar,
            s: 184 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"sienna2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 13 as libc::c_int as libc::c_uchar,
            s: 184 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"sienna3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 13 as libc::c_int as libc::c_uchar,
            s: 184 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"sienna4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 13 as libc::c_int as libc::c_uchar,
            s: 185 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"skyblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 139 as libc::c_int as libc::c_uchar,
            s: 108 as libc::c_int as libc::c_uchar,
            b: 235 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"skyblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 144 as libc::c_int as libc::c_uchar,
            s: 120 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"skyblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 144 as libc::c_int as libc::c_uchar,
            s: 120 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"skyblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 144 as libc::c_int as libc::c_uchar,
            s: 120 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"skyblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 145 as libc::c_int as libc::c_uchar,
            s: 119 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slateblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 175 as libc::c_int as libc::c_uchar,
            s: 143 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slateblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 175 as libc::c_int as libc::c_uchar,
            s: 144 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slateblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 175 as libc::c_int as libc::c_uchar,
            s: 144 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slateblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 175 as libc::c_int as libc::c_uchar,
            s: 144 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slateblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 175 as libc::c_int as libc::c_uchar,
            s: 144 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slategray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 56 as libc::c_int as libc::c_uchar,
            b: 144 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slategray1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 149 as libc::c_int as libc::c_uchar,
            s: 56 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slategray2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 149 as libc::c_int as libc::c_uchar,
            s: 56 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slategray3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 57 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slategray4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 149 as libc::c_int as libc::c_uchar,
            s: 56 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"slategrey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 148 as libc::c_int as libc::c_uchar,
            s: 56 as libc::c_int as libc::c_uchar,
            b: 144 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"snow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 5 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"snow1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 5 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"snow2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 5 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"snow3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 4 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"snow4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 3 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"springgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 106 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"springgreen1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 106 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"springgreen2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 106 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"springgreen3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 106 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"springgreen4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 106 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"steelblue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 146 as libc::c_int as libc::c_uchar,
            s: 155 as libc::c_int as libc::c_uchar,
            b: 180 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"steelblue1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 146 as libc::c_int as libc::c_uchar,
            s: 156 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"steelblue2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 146 as libc::c_int as libc::c_uchar,
            s: 156 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"steelblue3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 146 as libc::c_int as libc::c_uchar,
            s: 156 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"steelblue4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 147 as libc::c_int as libc::c_uchar,
            s: 155 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 24 as libc::c_int as libc::c_uchar,
            s: 84 as libc::c_int as libc::c_uchar,
            b: 210 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tan1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 176 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tan2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 176 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tan3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 176 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tan4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 20 as libc::c_int as libc::c_uchar,
            s: 176 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"thistle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 29 as libc::c_int as libc::c_uchar,
            b: 216 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"thistle1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 30 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"thistle2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 30 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"thistle3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 29 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"thistle4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 29 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tomato\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 184 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tomato1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 184 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tomato2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 184 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tomato3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 184 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"tomato4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 6 as libc::c_int as libc::c_uchar,
            s: 185 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"turquoise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 123 as libc::c_int as libc::c_uchar,
            s: 182 as libc::c_int as libc::c_uchar,
            b: 224 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"turquoise1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 129 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"turquoise2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 129 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"turquoise3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 129 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"turquoise4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 129 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"violet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 212 as libc::c_int as libc::c_uchar,
            s: 115 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"violetred\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 227 as libc::c_int as libc::c_uchar,
            s: 215 as libc::c_int as libc::c_uchar,
            b: 208 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"violetred1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 235 as libc::c_int as libc::c_uchar,
            s: 193 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"violetred2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 235 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"violetred3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 235 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"violetred4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 235 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"wheat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 245 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"wheat1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 69 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"wheat2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"wheat3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 68 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"wheat4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 27 as libc::c_int as libc::c_uchar,
            s: 67 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"white\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"whitesmoke\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 0 as libc::c_int as libc::c_uchar,
            s: 0 as libc::c_int as libc::c_uchar,
            b: 245 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"yellow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"yellow1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 255 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"yellow2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 238 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"yellow3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"yellow4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 42 as libc::c_int as libc::c_uchar,
            s: 255 as libc::c_int as libc::c_uchar,
            b: 139 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = hsbcolor_t {
            name: b"yellowgreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            h: 56 as libc::c_int as libc::c_uchar,
            s: 192 as libc::c_int as libc::c_uchar,
            b: 205 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
unsafe extern "C" fn canoncolor(
    mut orig: *const libc::c_char,
    mut out: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut p: *mut libc::c_char = out;
    loop {
        let fresh0 = orig;
        orig = orig.offset(1);
        c = *fresh0;
        if !(c != 0) {
            break;
        }
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            continue;
        }
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            c = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = c as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(c as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(c as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
        }
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = c;
    }
    *p = '\0' as i32 as libc::c_char;
    return out;
}
unsafe extern "C" fn colorcmpf(
    mut a0: *const libc::c_void,
    mut a1: *const libc::c_void,
) -> libc::c_int {
    let mut p0: *const hsbcolor_t = a0 as *const hsbcolor_t;
    let mut p1: *const hsbcolor_t = a1 as *const hsbcolor_t;
    return strcmp((*p0).name, (*p1).name);
}
#[no_mangle]
pub unsafe extern "C" fn colorxlate(
    mut str: *mut libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut last: *mut hsbcolor_t = 0 as *const hsbcolor_t as *mut hsbcolor_t;
    let mut canon: [libc::c_char; 128] = [0; 128];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fake: hsbcolor_t = hsbcolor_t {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        h: 0,
        s: 0,
        b: 0,
    };
    if last.is_null() || strcmp((*last).name, str) != 0 {
        fake.name = canoncolor(str, canon.as_mut_ptr());
        last = bsearch(
            &mut fake as *mut hsbcolor_t as *const libc::c_void,
            color_lib.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[hsbcolor_t; 652]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<hsbcolor_t>() as libc::c_ulong),
            ::std::mem::size_of::<hsbcolor_t>() as libc::c_ulong,
            Some(
                colorcmpf
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut hsbcolor_t;
    }
    if last.is_null() {
        if *(*__ctype_b_loc()).offset(canon[0 as libc::c_int as usize] as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            fprintf(
                stderr,
                b"warning: %s is not a known color\n\0" as *const u8 as *const libc::c_char,
                str,
            );
            strcpy(buf, str);
        } else {
            p = buf;
            loop {
                let fresh2 = str;
                str = str.offset(1);
                *p = *fresh2;
                if !(*p != 0) {
                    break;
                }
                if *p as libc::c_int == ',' as i32 {
                    *p = ' ' as i32 as libc::c_char;
                }
                p = p.offset(1);
            }
        }
    } else {
        sprintf(
            buf,
            b"%.3f %.3f %.3f\0" as *const u8 as *const libc::c_char,
            (*last).h as libc::c_double / 255 as libc::c_int as libc::c_double,
            (*last).s as libc::c_double / 255 as libc::c_int as libc::c_double,
            (*last).b as libc::c_double / 255 as libc::c_int as libc::c_double,
        );
    }
    return buf;
}
