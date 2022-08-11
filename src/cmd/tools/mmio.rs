#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type MM_typecode = [libc::c_char; 4];
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn mm_read_banner(
    mut f: *mut FILE,
    mut matcode: *mut MM_typecode,
) -> libc::c_int {
    let mut line: [libc::c_char; 100025] = [0; 100025];
    let mut banner: [libc::c_char; 64] = [0; 64];
    let mut mtx: [libc::c_char; 64] = [0; 64];
    let mut crd: [libc::c_char; 64] = [0; 64];
    let mut data_type: [libc::c_char; 64] = [0; 64];
    let mut storage_scheme: [libc::c_char; 64] = [0; 64];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let ref mut fresh0 = (*matcode)[2 as libc::c_int as usize];
    *fresh0 = ' ' as i32 as libc::c_char;
    let ref mut fresh1 = (*matcode)[1 as libc::c_int as usize];
    *fresh1 = *fresh0;
    (*matcode)[0 as libc::c_int as usize] = *fresh1;
    (*matcode)[3 as libc::c_int as usize] = 'G' as i32 as libc::c_char;
    if (fgets(line.as_mut_ptr(), 100025 as libc::c_int, f)).is_null() {
        return 12 as libc::c_int;
    }
    if sscanf(
        line.as_mut_ptr(),
        b"%s %s %s %s %s\0" as *const u8 as *const libc::c_char,
        banner.as_mut_ptr(),
        mtx.as_mut_ptr(),
        crd.as_mut_ptr(),
        data_type.as_mut_ptr(),
        storage_scheme.as_mut_ptr(),
    ) != 5 as libc::c_int
    {
        return 12 as libc::c_int;
    }
    p = mtx.as_mut_ptr();
    while *p as libc::c_int != '\0' as i32 {
        *p = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*p as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*p as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        p = p.offset(1);
    }
    p = crd.as_mut_ptr();
    while *p as libc::c_int != '\0' as i32 {
        *p = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*p as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*p as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        p = p.offset(1);
    }
    p = data_type.as_mut_ptr();
    while *p as libc::c_int != '\0' as i32 {
        *p = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*p as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*p as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        p = p.offset(1);
    }
    p = storage_scheme.as_mut_ptr();
    while *p as libc::c_int != '\0' as i32 {
        *p = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *p as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*p as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*p as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        p = p.offset(1);
    }
    if strncmp(
        banner.as_mut_ptr(),
        b"%%MatrixMarket\0" as *const u8 as *const libc::c_char,
        strlen(b"%%MatrixMarket\0" as *const u8 as *const libc::c_char),
    ) != 0 as libc::c_int
    {
        return 14 as libc::c_int;
    }
    if strcmp(mtx.as_mut_ptr(), b"matrix\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        return 15 as libc::c_int;
    }
    (*matcode)[0 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
    if strcmp(crd.as_mut_ptr(), b"coordinate\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*matcode)[1 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    } else if strcmp(crd.as_mut_ptr(), b"array\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
        (*matcode)[1 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    } else {
        return 15 as libc::c_int
    }
    if strcmp(data_type.as_mut_ptr(), b"real\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*matcode)[2 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    } else if strcmp(
            data_type.as_mut_ptr(),
            b"complex\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
        (*matcode)[2 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    } else if strcmp(
            data_type.as_mut_ptr(),
            b"pattern\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
        (*matcode)[2 as libc::c_int as usize] = 'P' as i32 as libc::c_char;
    } else if strcmp(
            data_type.as_mut_ptr(),
            b"integer\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
        (*matcode)[2 as libc::c_int as usize] = 'I' as i32 as libc::c_char;
    } else {
        return 15 as libc::c_int
    }
    if strcmp(
        storage_scheme.as_mut_ptr(),
        b"general\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*matcode)[3 as libc::c_int as usize] = 'G' as i32 as libc::c_char;
    } else if strcmp(
            storage_scheme.as_mut_ptr(),
            b"symmetric\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
        (*matcode)[3 as libc::c_int as usize] = 'S' as i32 as libc::c_char;
    } else if strcmp(
            storage_scheme.as_mut_ptr(),
            b"hermitian\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
        (*matcode)[3 as libc::c_int as usize] = 'H' as i32 as libc::c_char;
    } else if strcmp(
            storage_scheme.as_mut_ptr(),
            b"skew-symmetric\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
        (*matcode)[3 as libc::c_int as usize] = 'K' as i32 as libc::c_char;
    } else {
        return 15 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mm_read_mtx_crd_size(
    mut f: *mut FILE,
    mut M: *mut libc::c_int,
    mut N: *mut libc::c_int,
    mut nz: *mut libc::c_int,
) -> libc::c_int {
    let mut line: [libc::c_char; 100025] = [0; 100025];
    let mut num_items_read: libc::c_int = 0;
    *nz = 0 as libc::c_int;
    *N = *nz;
    *M = *N;
    loop {
        if (fgets(line.as_mut_ptr(), 100025 as libc::c_int, f)).is_null() {
            return 12 as libc::c_int;
        }
        if !(line[0 as libc::c_int as usize] as libc::c_int == '%' as i32) {
            break;
        }
    }
    if sscanf(
        line.as_mut_ptr(),
        b"%d %d %d\0" as *const u8 as *const libc::c_char,
        M,
        N,
        nz,
    ) == 3 as libc::c_int
    {
        return 0 as libc::c_int
    } else {
        loop {
            num_items_read = fscanf(
                f,
                b"%d %d %d\0" as *const u8 as *const libc::c_char,
                M,
                N,
                nz,
            );
            if num_items_read == -(1 as libc::c_int) {
                return 12 as libc::c_int;
            }
            if !(num_items_read != 3 as libc::c_int) {
                break;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mm_typecode_to_str(
    mut matcode: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut buffer: [libc::c_char; 100025] = [0; 100025];
    let mut types: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    if *matcode.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32 {
        types[0 as libc::c_int
            as usize] = b"matrix\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        return 0 as *mut libc::c_char
    }
    if *matcode.offset(1 as libc::c_int as isize) as libc::c_int == 'C' as i32 {
        types[1 as libc::c_int
            as usize] = b"coordinate\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if *matcode.offset(1 as libc::c_int as isize) as libc::c_int == 'A' as i32 {
        types[1 as libc::c_int
            as usize] = b"array\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        return 0 as *mut libc::c_char
    }
    if *matcode.offset(2 as libc::c_int as isize) as libc::c_int == 'R' as i32 {
        types[2 as libc::c_int
            as usize] = b"real\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if *matcode.offset(2 as libc::c_int as isize) as libc::c_int == 'C' as i32 {
        types[2 as libc::c_int
            as usize] = b"complex\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if *matcode.offset(2 as libc::c_int as isize) as libc::c_int == 'P' as i32 {
        types[2 as libc::c_int
            as usize] = b"pattern\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if *matcode.offset(2 as libc::c_int as isize) as libc::c_int == 'I' as i32 {
        types[2 as libc::c_int
            as usize] = b"integer\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        return 0 as *mut libc::c_char
    }
    if *matcode.offset(3 as libc::c_int as isize) as libc::c_int == 'G' as i32 {
        types[3 as libc::c_int
            as usize] = b"general\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if *matcode.offset(3 as libc::c_int as isize) as libc::c_int == 'S' as i32 {
        types[3 as libc::c_int
            as usize] = b"symmetric\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if *matcode.offset(3 as libc::c_int as isize) as libc::c_int == 'H' as i32 {
        types[3 as libc::c_int
            as usize] = b"hermitian\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if *matcode.offset(3 as libc::c_int as isize) as libc::c_int == 'K' as i32 {
        types[3 as libc::c_int
            as usize] = b"skew-symmetric\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        return 0 as *mut libc::c_char
    }
    snprintf(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100025]>() as libc::c_ulong,
        b"%s %s %s %s\0" as *const u8 as *const libc::c_char,
        types[0 as libc::c_int as usize],
        types[1 as libc::c_int as usize],
        types[2 as libc::c_int as usize],
        types[3 as libc::c_int as usize],
    );
    return strdup(buffer.as_mut_ptr());
}
