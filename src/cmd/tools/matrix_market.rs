#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn SparseMatrix_from_coordinate_arrays(
        nz: libc::c_int,
        m: libc::c_int,
        n: libc::c_int,
        irn: *mut libc::c_int,
        jcn: *mut libc::c_int,
        val: *mut libc::c_void,
        type_0: libc::c_int,
        sz: size_t,
    ) -> SparseMatrix;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn mm_read_banner(f: *mut FILE, matcode: *mut MM_typecode) -> libc::c_int;
    fn mm_read_mtx_crd_size(
        f: *mut FILE,
        M: *mut libc::c_int,
        N: *mut libc::c_int,
        nz: *mut libc::c_int,
    ) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MATRIX_UNDIRECTED: C2RustUnnamed = 16;
pub const MATRIX_HERMITIAN: C2RustUnnamed = 8;
pub const MATRIX_SKEW: C2RustUnnamed = 4;
pub const MATRIX_SYMMETRIC: C2RustUnnamed = 2;
pub const MATRIX_PATTERN_SYMMETRIC: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseMatrix_struct {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nz: libc::c_int,
    pub nzmax: libc::c_int,
    pub type_0: libc::c_int,
    pub ia: *mut libc::c_int,
    pub ja: *mut libc::c_int,
    pub a: *mut libc::c_void,
    pub format: libc::c_int,
    pub property: libc::c_int,
    pub size: libc::c_int,
}
pub type SparseMatrix = *mut SparseMatrix_struct;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_0 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_0 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_0 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_0 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_0 = 1;
pub type MM_typecode = [libc::c_char; 4];
unsafe extern "C" fn mm_get_type(mut typecode: *mut libc::c_char) -> libc::c_int {
    if *typecode.offset(2 as libc::c_int as isize) as libc::c_int == 'C' as i32 {
        return MATRIX_TYPE_COMPLEX as libc::c_int
    } else {
        if *typecode.offset(2 as libc::c_int as isize) as libc::c_int == 'R' as i32 {
            return MATRIX_TYPE_REAL as libc::c_int
        } else {
            if *typecode.offset(2 as libc::c_int as isize) as libc::c_int == 'I' as i32 {
                return MATRIX_TYPE_INTEGER as libc::c_int
            } else {
                if *typecode.offset(2 as libc::c_int as isize) as libc::c_int
                    == 'P' as i32
                {
                    return MATRIX_TYPE_PATTERN as libc::c_int;
                }
            }
        }
    }
    return MATRIX_TYPE_UNKNOWN as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_import_matrix_market(
    mut f: *mut FILE,
) -> SparseMatrix {
    let mut ret_code: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut matcode: MM_typecode = [0; 4];
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vali: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut I: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut J: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut vp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut nzold: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = fgetc(f);
    if c != '%' as i32 {
        ungetc(c, f);
        return 0 as SparseMatrix;
    }
    ungetc(c, f);
    if mm_read_banner(f, &mut matcode) != 0 as libc::c_int {
        return 0 as SparseMatrix;
    }
    if !(matcode[0 as libc::c_int as usize] as libc::c_int == 'M' as i32)
        || !(matcode[1 as libc::c_int as usize] as libc::c_int == 'C' as i32)
    {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"matrix_market.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                .as_ptr(),
        );
        return 0 as SparseMatrix;
    }
    ret_code = mm_read_mtx_crd_size(f, &mut m, &mut n, &mut nz);
    if ret_code != 0 as libc::c_int {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"matrix_market.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                .as_ptr(),
        );
        return 0 as SparseMatrix;
    }
    I = gmalloc(
        (nz as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    J = gmalloc(
        (nz as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    type_0 = mm_get_type(matcode.as_mut_ptr());
    match type_0 {
        1 => {
            val = malloc(
                (nz as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < nz {
                let mut num: libc::c_int = fscanf(
                    f,
                    b"%d %d %lg\n\0" as *const u8 as *const libc::c_char,
                    &mut *I.offset(i as isize) as *mut libc::c_int,
                    &mut *J.offset(i as isize) as *mut libc::c_int,
                    &mut *val.offset(i as isize) as *mut libc::c_double,
                );
                if num == 3 as libc::c_int {} else {
                    __assert_fail(
                        b"num == 3\0" as *const u8 as *const libc::c_char,
                        b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                        89 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                            .as_ptr(),
                    );
                }
                let ref mut fresh0 = *I.offset(i as isize);
                *fresh0 -= 1;
                let ref mut fresh1 = *J.offset(i as isize);
                *fresh1 -= 1;
                i += 1;
            }
            if matcode[3 as libc::c_int as usize] as libc::c_int == 'S' as i32 {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                val = grealloc(
                    val as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_double;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {
                        *I.offset(nz as isize) = *J.offset(i as isize);
                        *J.offset(nz as isize) = *I.offset(i as isize);
                        let fresh2 = nz;
                        nz = nz + 1;
                        *val.offset(fresh2 as isize) = *val.offset(i as isize);
                    }
                    i += 1;
                }
            } else if matcode[3 as libc::c_int as usize] as libc::c_int == 'K' as i32 {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                val = grealloc(
                    val as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_double;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {} else {
                        __assert_fail(
                            b"I[i] != J[i]\0" as *const u8 as *const libc::c_char,
                            b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                            111 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *I.offset(nz as isize) = *J.offset(i as isize);
                    *J.offset(nz as isize) = *I.offset(i as isize);
                    let fresh3 = nz;
                    nz = nz + 1;
                    *val.offset(fresh3 as isize) = -*val.offset(i as isize);
                    i += 1;
                }
            } else if !(matcode[3 as libc::c_int as usize] as libc::c_int == 'H' as i32)
                {} else {
                __assert_fail(
                    b"!mm_is_hermitian(matcode)\0" as *const u8 as *const libc::c_char,
                    b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                    117 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                        .as_ptr(),
                );
            }
            vp = val as *mut libc::c_void;
        }
        4 => {
            vali = malloc(
                (nz as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < nz {
                let mut num_0: libc::c_int = fscanf(
                    f,
                    b"%d %d %d\n\0" as *const u8 as *const libc::c_char,
                    &mut *I.offset(i as isize) as *mut libc::c_int,
                    &mut *J.offset(i as isize) as *mut libc::c_int,
                    &mut *vali.offset(i as isize) as *mut libc::c_int,
                );
                if num_0 == 3 as libc::c_int {} else {
                    __assert_fail(
                        b"num == 3\0" as *const u8 as *const libc::c_char,
                        b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                        126 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                            .as_ptr(),
                    );
                }
                let ref mut fresh4 = *I.offset(i as isize);
                *fresh4 -= 1;
                let ref mut fresh5 = *J.offset(i as isize);
                *fresh5 -= 1;
                i += 1;
            }
            if matcode[3 as libc::c_int as usize] as libc::c_int == 'S' as i32 {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                vali = grealloc(
                    vali as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {
                        *I.offset(nz as isize) = *J.offset(i as isize);
                        *J.offset(nz as isize) = *I.offset(i as isize);
                        let fresh6 = nz;
                        nz = nz + 1;
                        *vali.offset(fresh6 as isize) = *vali.offset(i as isize);
                    }
                    i += 1;
                }
            } else if matcode[3 as libc::c_int as usize] as libc::c_int == 'K' as i32 {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                vali = grealloc(
                    vali as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {} else {
                        __assert_fail(
                            b"I[i] != J[i]\0" as *const u8 as *const libc::c_char,
                            b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                            148 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *I.offset(nz as isize) = *J.offset(i as isize);
                    *J.offset(nz as isize) = *I.offset(i as isize);
                    let fresh7 = nz;
                    nz = nz + 1;
                    *vali.offset(fresh7 as isize) = -*vali.offset(i as isize);
                    i += 1;
                }
            } else if !(matcode[3 as libc::c_int as usize] as libc::c_int == 'H' as i32)
                {} else {
                __assert_fail(
                    b"!mm_is_hermitian(matcode)\0" as *const u8 as *const libc::c_char,
                    b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                    154 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                        .as_ptr(),
                );
            }
            vp = vali as *mut libc::c_void;
        }
        8 => {
            i = 0 as libc::c_int;
            while i < nz {
                let mut num_1: libc::c_int = fscanf(
                    f,
                    b"%d %d\n\0" as *const u8 as *const libc::c_char,
                    &mut *I.offset(i as isize) as *mut libc::c_int,
                    &mut *J.offset(i as isize) as *mut libc::c_int,
                );
                if num_1 == 2 as libc::c_int {} else {
                    __assert_fail(
                        b"num == 2\0" as *const u8 as *const libc::c_char,
                        b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                        162 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                            .as_ptr(),
                    );
                }
                let ref mut fresh8 = *I.offset(i as isize);
                *fresh8 -= 1;
                let ref mut fresh9 = *J.offset(i as isize);
                *fresh9 -= 1;
                i += 1;
            }
            if matcode[3 as libc::c_int as usize] as libc::c_int == 'S' as i32
                || matcode[3 as libc::c_int as usize] as libc::c_int == 'K' as i32
            {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {
                        *I.offset(nz as isize) = *J.offset(i as isize);
                        let fresh10 = nz;
                        nz = nz + 1;
                        *J.offset(fresh10 as isize) = *I.offset(i as isize);
                    }
                    i += 1;
                }
            } else if !(matcode[3 as libc::c_int as usize] as libc::c_int == 'H' as i32)
                {} else {
                __assert_fail(
                    b"!mm_is_hermitian(matcode)\0" as *const u8 as *const libc::c_char,
                    b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                    177 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                        .as_ptr(),
                );
            }
        }
        2 => {
            val = malloc(
                ((2 as libc::c_int * nz) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_double;
            v = val;
            i = 0 as libc::c_int;
            while i < nz {
                let mut num_2: libc::c_int = fscanf(
                    f,
                    b"%d %d %lg %lg\n\0" as *const u8 as *const libc::c_char,
                    &mut *I.offset(i as isize) as *mut libc::c_int,
                    &mut *J.offset(i as isize) as *mut libc::c_int,
                    &mut *v.offset(0 as libc::c_int as isize) as *mut libc::c_double,
                    &mut *v.offset(1 as libc::c_int as isize) as *mut libc::c_double,
                );
                if num_2 == 4 as libc::c_int {} else {
                    __assert_fail(
                        b"num == 4\0" as *const u8 as *const libc::c_char,
                        b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                        186 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0"))
                            .as_ptr(),
                    );
                }
                v = v.offset(2 as libc::c_int as isize);
                let ref mut fresh11 = *I.offset(i as isize);
                *fresh11 -= 1;
                let ref mut fresh12 = *J.offset(i as isize);
                *fresh12 -= 1;
                i += 1;
            }
            if matcode[3 as libc::c_int as usize] as libc::c_int == 'S' as i32 {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                val = grealloc(
                    val as *mut libc::c_void,
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_double;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {
                        *I.offset(nz as isize) = *J.offset(i as isize);
                        *J.offset(nz as isize) = *I.offset(i as isize);
                        *val
                            .offset(
                                (2 as libc::c_int * nz) as isize,
                            ) = *val.offset((2 as libc::c_int * i) as isize);
                        *val
                            .offset(
                                (2 as libc::c_int * nz + 1 as libc::c_int) as isize,
                            ) = *val
                            .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
                        nz += 1;
                    }
                    i += 1;
                }
            } else if matcode[3 as libc::c_int as usize] as libc::c_int == 'K' as i32 {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                val = grealloc(
                    val as *mut libc::c_void,
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_double;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {} else {
                        __assert_fail(
                            b"I[i] != J[i]\0" as *const u8 as *const libc::c_char,
                            b"matrix_market.c\0" as *const u8 as *const libc::c_char,
                            211 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"SparseMatrix SparseMatrix_import_matrix_market(FILE *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *I.offset(nz as isize) = *J.offset(i as isize);
                    *J.offset(nz as isize) = *I.offset(i as isize);
                    *val
                        .offset(
                            (2 as libc::c_int * nz) as isize,
                        ) = -*val.offset((2 as libc::c_int * i) as isize);
                    *val
                        .offset(
                            (2 as libc::c_int * nz + 1 as libc::c_int) as isize,
                        ) = -*val
                        .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
                    nz += 1;
                    i += 1;
                }
            } else if matcode[3 as libc::c_int as usize] as libc::c_int == 'H' as i32 {
                I = grealloc(
                    I as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                J = grealloc(
                    J as *mut libc::c_void,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_int;
                val = grealloc(
                    val as *mut libc::c_void,
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        )
                        .wrapping_mul(nz as libc::c_ulong),
                ) as *mut libc::c_double;
                nzold = nz;
                i = 0 as libc::c_int;
                while i < nzold {
                    if *I.offset(i as isize) != *J.offset(i as isize) {
                        *I.offset(nz as isize) = *J.offset(i as isize);
                        *J.offset(nz as isize) = *I.offset(i as isize);
                        *val
                            .offset(
                                (2 as libc::c_int * nz) as isize,
                            ) = *val.offset((2 as libc::c_int * i) as isize);
                        *val
                            .offset(
                                (2 as libc::c_int * nz + 1 as libc::c_int) as isize,
                            ) = -*val
                            .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
                        nz += 1;
                    }
                    i += 1;
                }
            }
            vp = val as *mut libc::c_void;
        }
        _ => return 0 as SparseMatrix,
    }
    A = SparseMatrix_from_coordinate_arrays(
        nz,
        m,
        n,
        I,
        J,
        vp,
        type_0,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    free(I as *mut libc::c_void);
    free(J as *mut libc::c_void);
    free(val as *mut libc::c_void);
    if matcode[3 as libc::c_int as usize] as libc::c_int == 'S' as i32 {
        (*A).property = (*A).property | MATRIX_SYMMETRIC as libc::c_int;
        (*A).property = (*A).property | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
    } else if matcode[3 as libc::c_int as usize] as libc::c_int == 'K' as i32 {
        (*A).property = (*A).property | MATRIX_SKEW as libc::c_int;
    } else if matcode[3 as libc::c_int as usize] as libc::c_int == 'H' as i32 {
        (*A).property = (*A).property | MATRIX_HERMITIAN as libc::c_int;
    }
    return A;
}
