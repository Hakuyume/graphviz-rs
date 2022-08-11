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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn grealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn gmalloc(_: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn BinaryHeap_new(
        cmp_0: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
    ) -> BinaryHeap;
    fn BinaryHeap_delete(h: BinaryHeap, del: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>);
    fn BinaryHeap_insert(h: BinaryHeap, item: *mut libc::c_void) -> libc::c_int;
    fn BinaryHeap_extract_min(h: BinaryHeap) -> *mut libc::c_void;
    fn BinaryHeap_get_item(h: BinaryHeap, id: libc::c_int) -> *mut libc::c_void;
    fn BinaryHeap_reset(h: BinaryHeap, id: libc::c_int, item: *mut libc::c_void) -> size_t;
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
pub const FORMAT_COORD: C2RustUnnamed = 2;
pub const FORMAT_CSR: C2RustUnnamed = 1;
pub const FORMAT_CSC: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_int;
pub const MASKED: C2RustUnnamed_0 = 1;
pub const UNMASKED: C2RustUnnamed_0 = -10;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MATRIX_UNDIRECTED: C2RustUnnamed_1 = 16;
pub const MATRIX_HERMITIAN: C2RustUnnamed_1 = 8;
pub const MATRIX_SKEW: C2RustUnnamed_1 = 4;
pub const MATRIX_SYMMETRIC: C2RustUnnamed_1 = 2;
pub const MATRIX_PATTERN_SYMMETRIC: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const BIPARTITE_ALWAYS: C2RustUnnamed_2 = 3;
pub const BIPARTITE_UNSYM: C2RustUnnamed_2 = 2;
pub const BIPARTITE_PATTERN_UNSYM: C2RustUnnamed_2 = 1;
pub const BIPARTITE_RECT: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MATRIX_TYPE_UNKNOWN: C2RustUnnamed_3 = 16;
pub const MATRIX_TYPE_PATTERN: C2RustUnnamed_3 = 8;
pub const MATRIX_TYPE_INTEGER: C2RustUnnamed_3 = 4;
pub const MATRIX_TYPE_COMPLEX: C2RustUnnamed_3 = 2;
pub const MATRIX_TYPE_REAL: C2RustUnnamed_3 = 1;
pub const SUM_REPEATED_ALL: C2RustUnnamed_4 = 1;
pub const SUM_REPEATED_NONE: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub type BinaryHeap = *mut BinaryHeap_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BinaryHeap_struct {
    pub max_len: size_t,
    pub len: size_t,
    pub heap: *mut *mut libc::c_void,
    pub id_to_pos: *mut size_t,
    pub pos_to_id: *mut libc::c_int,
    pub id_stack: IntStack,
    pub cmp: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
}
pub type IntStack = *mut IntStack_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntStack_struct {
    pub last: size_t,
    pub max_len: size_t,
    pub stack: *mut libc::c_int,
}
pub type nodedata = *mut nodedata_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodedata_struct {
    pub dist: libc::c_double,
    pub id: libc::c_int,
}
pub const UNVISITED: C2RustUnnamed_5 = -2;
pub const FINISHED: C2RustUnnamed_5 = -1;
pub type C2RustUnnamed_5 = libc::c_int;
unsafe extern "C" fn size_of_matrix_type(mut type_0: libc::c_int) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    match type_0 {
        1 => {
            size = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
        }
        2 => {
            size = (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong);
        }
        4 => {
            size = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        }
        8 => {
            size = 0 as libc::c_int as size_t;
        }
        16 => {
            size = 0 as libc::c_int as size_t;
        }
        _ => {
            size = 0 as libc::c_int as size_t;
        }
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_sort(mut A: SparseMatrix) -> SparseMatrix {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    B = SparseMatrix_transpose(A);
    SparseMatrix_delete(A);
    A = SparseMatrix_transpose(B);
    SparseMatrix_delete(B);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_make_undirected(mut A: SparseMatrix) -> SparseMatrix {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    B = SparseMatrix_symmetrize(A, 0 as libc::c_int != 0);
    (*B).property = (*B).property | MATRIX_UNDIRECTED as libc::c_int;
    return SparseMatrix_remove_upper(B);
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_transpose(mut A: SparseMatrix) -> SparseMatrix {
    if A.is_null() {
        return 0 as SparseMatrix;
    }
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut ib: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jb: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = (*A).nz;
    let mut m: libc::c_int = (*A).m;
    let mut n: libc::c_int = (*A).n;
    let mut type_0: libc::c_int = (*A).type_0;
    let mut format: libc::c_int = (*A).format;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"SparseMatrix SparseMatrix_transpose(SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    B = SparseMatrix_new(n, m, nz, type_0, format);
    (*B).nz = nz;
    ib = (*B).ia;
    jb = (*B).ja;
    i = 0 as libc::c_int;
    while i <= n {
        *ib.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            let ref mut fresh0 = *ib.offset((*ja.offset(j as isize) + 1 as libc::c_int) as isize);
            *fresh0 += 1;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        *ib.offset((i + 1 as libc::c_int) as isize) += *ib.offset(i as isize);
        i += 1;
    }
    match (*A).type_0 {
        1 => {
            let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
            let mut b: *mut libc::c_double = (*B).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    *jb.offset(*ib.offset(*ja.offset(j as isize) as isize) as isize) = i;
                    let ref mut fresh1 = *ib.offset(*ja.offset(j as isize) as isize);
                    let fresh2 = *fresh1;
                    *fresh1 = *fresh1 + 1;
                    *b.offset(fresh2 as isize) = *a.offset(j as isize);
                    j += 1;
                }
                i += 1;
            }
        }
        2 => {
            let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
            let mut b_0: *mut libc::c_double = (*B).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    *jb.offset(*ib.offset(*ja.offset(j as isize) as isize) as isize) = i;
                    *b_0.offset(
                        (2 as libc::c_int * *ib.offset(*ja.offset(j as isize) as isize)) as isize,
                    ) = *a_0.offset((2 as libc::c_int * j) as isize);
                    *b_0.offset(
                        (2 as libc::c_int * *ib.offset(*ja.offset(j as isize) as isize)
                            + 1 as libc::c_int) as isize,
                    ) = *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                    let ref mut fresh3 = *ib.offset(*ja.offset(j as isize) as isize);
                    *fresh3 += 1;
                    j += 1;
                }
                i += 1;
            }
        }
        4 => {
            let mut ai: *mut libc::c_int = (*A).a as *mut libc::c_int;
            let mut bi: *mut libc::c_int = (*B).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    *jb.offset(*ib.offset(*ja.offset(j as isize) as isize) as isize) = i;
                    let ref mut fresh4 = *ib.offset(*ja.offset(j as isize) as isize);
                    let fresh5 = *fresh4;
                    *fresh4 = *fresh4 + 1;
                    *bi.offset(fresh5 as isize) = *ai.offset(j as isize);
                    j += 1;
                }
                i += 1;
            }
        }
        8 => {
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    let ref mut fresh6 = *ib.offset(*ja.offset(j as isize) as isize);
                    let fresh7 = *fresh6;
                    *fresh6 = *fresh6 + 1;
                    *jb.offset(fresh7 as isize) = i;
                    j += 1;
                }
                i += 1;
            }
        }
        16 => {
            SparseMatrix_delete(B);
            return 0 as SparseMatrix;
        }
        _ => {
            SparseMatrix_delete(B);
            return 0 as SparseMatrix;
        }
    }
    i = n - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        *ib.offset((i + 1 as libc::c_int) as isize) = *ib.offset(i as isize);
        i -= 1;
    }
    *ib.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_symmetrize(
    mut A: SparseMatrix,
    mut pattern_symmetric_only: bool,
) -> SparseMatrix {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    if SparseMatrix_is_symmetric(A, pattern_symmetric_only) != 0 {
        return SparseMatrix_copy(A);
    }
    B = SparseMatrix_transpose(A);
    if B.is_null() {
        return 0 as SparseMatrix;
    }
    A = SparseMatrix_add(A, B);
    SparseMatrix_delete(B);
    (*A).property = (*A).property | MATRIX_SYMMETRIC as libc::c_int;
    (*A).property = (*A).property | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_symmetrize_nodiag(mut A: SparseMatrix) -> SparseMatrix {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    if SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
        B = SparseMatrix_copy(A);
        return SparseMatrix_remove_diagonal(B);
    }
    B = SparseMatrix_transpose(A);
    if B.is_null() {
        return 0 as SparseMatrix;
    }
    A = SparseMatrix_add(A, B);
    SparseMatrix_delete(B);
    (*A).property = (*A).property | MATRIX_SYMMETRIC as libc::c_int;
    (*A).property = (*A).property | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
    return SparseMatrix_remove_diagonal(A);
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_is_symmetric(
    mut A: SparseMatrix,
    mut test_pattern_symmetry_only: bool,
) -> libc::c_int {
    let mut current_block: u64;
    if A.is_null() {
        return 0 as libc::c_int;
    }
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ib: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jb: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut type_0: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"int SparseMatrix_is_symmetric(SparseMatrix, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).property & MATRIX_SYMMETRIC as libc::c_int != 0 {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if test_pattern_symmetry_only as libc::c_int != 0
        && (*A).property & MATRIX_PATTERN_SYMMETRIC as libc::c_int != 0
    {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if (*A).m != (*A).n {
        return 0 as libc::c_int;
    }
    B = SparseMatrix_transpose(A);
    if B.is_null() {
        return 0 as libc::c_int;
    }
    ia = (*A).ia;
    ja = (*A).ja;
    ib = (*B).ia;
    jb = (*B).ja;
    m = (*A).m;
    mask =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(m as size_t))
            as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    type_0 = (*A).type_0;
    if test_pattern_symmetry_only {
        type_0 = MATRIX_TYPE_PATTERN as libc::c_int;
    }
    match type_0 {
        1 => {
            let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
            let mut b: *mut libc::c_double = (*B).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            loop {
                if !(i <= m) {
                    current_block = 14359455889292382949;
                    break;
                }
                if *ia.offset(i as isize) != *ib.offset(i as isize) {
                    current_block = 16769872213879885304;
                    break;
                }
                i += 1;
            }
            match current_block {
                16769872213879885304 => {}
                _ => {
                    i = 0 as libc::c_int;
                    's_146: loop {
                        if !(i < m) {
                            current_block = 2543120759711851213;
                            break;
                        }
                        j = *ia.offset(i as isize);
                        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                            *mask.offset(*ja.offset(j as isize) as isize) = j;
                            j += 1;
                        }
                        j = *ib.offset(i as isize);
                        while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                            if *mask.offset(*jb.offset(j as isize) as isize)
                                < *ia.offset(i as isize)
                            {
                                current_block = 16769872213879885304;
                                break 's_146;
                            }
                            j += 1;
                        }
                        j = *ib.offset(i as isize);
                        while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                            if fabs(
                                *b.offset(j as isize)
                                    - *a.offset(
                                        *mask.offset(*jb.offset(j as isize) as isize) as isize
                                    ),
                            ) > 0.0000001f64
                            {
                                current_block = 16769872213879885304;
                                break 's_146;
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    match current_block {
                        16769872213879885304 => {}
                        _ => {
                            res = (0 as libc::c_int == 0) as libc::c_int;
                            current_block = 1765866445182206997;
                        }
                    }
                }
            }
        }
        2 => {
            let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
            let mut b_0: *mut libc::c_double = (*B).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            loop {
                if !(i <= m) {
                    current_block = 13678349939556791712;
                    break;
                }
                if *ia.offset(i as isize) != *ib.offset(i as isize) {
                    current_block = 16769872213879885304;
                    break;
                }
                i += 1;
            }
            match current_block {
                16769872213879885304 => {}
                _ => {
                    i = 0 as libc::c_int;
                    's_252: loop {
                        if !(i < m) {
                            current_block = 981995395831942902;
                            break;
                        }
                        j = *ia.offset(i as isize);
                        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                            *mask.offset(*ja.offset(j as isize) as isize) = j;
                            j += 1;
                        }
                        j = *ib.offset(i as isize);
                        while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                            if *mask.offset(*jb.offset(j as isize) as isize)
                                < *ia.offset(i as isize)
                            {
                                current_block = 16769872213879885304;
                                break 's_252;
                            }
                            j += 1;
                        }
                        j = *ib.offset(i as isize);
                        while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                            if fabs(
                                *b_0.offset((2 as libc::c_int * j) as isize)
                                    - *a_0.offset(
                                        (2 as libc::c_int
                                            * *mask.offset(*jb.offset(j as isize) as isize))
                                            as isize,
                                    ),
                            ) > 0.0000001f64
                            {
                                current_block = 16769872213879885304;
                                break 's_252;
                            }
                            if fabs(
                                *b_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize)
                                    - *a_0.offset(
                                        (2 as libc::c_int
                                            * *mask.offset(*jb.offset(j as isize) as isize)
                                            + 1 as libc::c_int)
                                            as isize,
                                    ),
                            ) > 0.0000001f64
                            {
                                current_block = 16769872213879885304;
                                break 's_252;
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                    match current_block {
                        16769872213879885304 => {}
                        _ => {
                            res = (0 as libc::c_int == 0) as libc::c_int;
                            current_block = 1765866445182206997;
                        }
                    }
                }
            }
        }
        4 => {
            let mut ai: *mut libc::c_int = (*A).a as *mut libc::c_int;
            let mut bi: *mut libc::c_int = (*B).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            's_348: loop {
                if !(i < m) {
                    current_block = 18325745679564279244;
                    break;
                }
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    *mask.offset(*ja.offset(j as isize) as isize) = j;
                    j += 1;
                }
                j = *ib.offset(i as isize);
                while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                    if *mask.offset(*jb.offset(j as isize) as isize) < *ia.offset(i as isize) {
                        current_block = 16769872213879885304;
                        break 's_348;
                    }
                    j += 1;
                }
                j = *ib.offset(i as isize);
                while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                    if *bi.offset(j as isize)
                        != *ai.offset(*mask.offset(*jb.offset(j as isize) as isize) as isize)
                    {
                        current_block = 16769872213879885304;
                        break 's_348;
                    }
                    j += 1;
                }
                i += 1;
            }
            match current_block {
                16769872213879885304 => {}
                _ => {
                    res = (0 as libc::c_int == 0) as libc::c_int;
                    current_block = 1765866445182206997;
                }
            }
        }
        8 => {
            i = 0 as libc::c_int;
            's_430: loop {
                if !(i < m) {
                    current_block = 6406431739208918833;
                    break;
                }
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    *mask.offset(*ja.offset(j as isize) as isize) = j;
                    j += 1;
                }
                j = *ib.offset(i as isize);
                while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                    if *mask.offset(*jb.offset(j as isize) as isize) < *ia.offset(i as isize) {
                        current_block = 16769872213879885304;
                        break 's_430;
                    }
                    j += 1;
                }
                i += 1;
            }
            match current_block {
                16769872213879885304 => {}
                _ => {
                    res = (0 as libc::c_int == 0) as libc::c_int;
                    current_block = 1765866445182206997;
                }
            }
        }
        16 | _ => {
            current_block = 16769872213879885304;
        }
    }
    match current_block {
        1765866445182206997 => {
            if test_pattern_symmetry_only {
                (*A).property = (*A).property | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
            } else {
                (*A).property = (*A).property | MATRIX_SYMMETRIC as libc::c_int;
                (*A).property = (*A).property | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
            }
        }
        _ => {}
    }
    free(mask as *mut libc::c_void);
    SparseMatrix_delete(B);
    return res;
}
unsafe extern "C" fn SparseMatrix_init(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut type_0: libc::c_int,
    mut sz: size_t,
    mut format: libc::c_int,
) -> SparseMatrix {
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    A = gmalloc(::std::mem::size_of::<SparseMatrix_struct>() as libc::c_ulong) as SparseMatrix;
    (*A).m = m;
    (*A).n = n;
    (*A).nz = 0 as libc::c_int;
    (*A).nzmax = 0 as libc::c_int;
    (*A).type_0 = type_0;
    (*A).size = sz as libc::c_int;
    match format {
        2 => {
            let ref mut fresh8 = (*A).ia;
            *fresh8 = 0 as *mut libc::c_int;
        }
        0 | 1 | _ => {
            let ref mut fresh9 = (*A).ia;
            *fresh9 = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul((m + 1 as libc::c_int) as size_t),
            ) as *mut libc::c_int;
        }
    }
    let ref mut fresh10 = (*A).ja;
    *fresh10 = 0 as *mut libc::c_int;
    let ref mut fresh11 = (*A).a;
    *fresh11 = 0 as *mut libc::c_void;
    (*A).format = format;
    (*A).property = 0 as libc::c_int;
    (*A).property &= !(MATRIX_PATTERN_SYMMETRIC as libc::c_int);
    (*A).property &= !(MATRIX_SYMMETRIC as libc::c_int);
    (*A).property &= !(MATRIX_SKEW as libc::c_int);
    (*A).property &= !(MATRIX_HERMITIAN as libc::c_int);
    return A;
}
unsafe extern "C" fn SparseMatrix_alloc(mut A: SparseMatrix, mut nz: libc::c_int) -> SparseMatrix {
    let mut format: libc::c_int = (*A).format;
    let mut nz_t: size_t = nz as size_t;
    let ref mut fresh12 = (*A).a;
    *fresh12 = 0 as *mut libc::c_void;
    match format {
        2 => {
            let ref mut fresh13 = (*A).ia;
            *fresh13 =
                gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz_t))
                    as *mut libc::c_int;
            let ref mut fresh14 = (*A).ja;
            *fresh14 =
                gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz_t))
                    as *mut libc::c_int;
            let ref mut fresh15 = (*A).a;
            *fresh15 = gmalloc(((*A).size as libc::c_ulong).wrapping_mul(nz_t));
        }
        1 | 0 | _ => {
            let ref mut fresh16 = (*A).ja;
            *fresh16 =
                gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz_t))
                    as *mut libc::c_int;
            if (*A).size > 0 as libc::c_int && nz_t > 0 as libc::c_int as libc::c_ulong {
                let ref mut fresh17 = (*A).a;
                *fresh17 = gmalloc(((*A).size as libc::c_ulong).wrapping_mul(nz_t));
            }
        }
    }
    (*A).nzmax = nz;
    return A;
}
unsafe extern "C" fn SparseMatrix_realloc(
    mut A: SparseMatrix,
    mut nz: libc::c_int,
) -> SparseMatrix {
    let mut format: libc::c_int = (*A).format;
    let mut nz_t: size_t = nz as size_t;
    match format {
        2 => {
            let ref mut fresh18 = (*A).ia;
            *fresh18 = grealloc(
                (*A).ia as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz_t),
            ) as *mut libc::c_int;
            let ref mut fresh19 = (*A).ja;
            *fresh19 = grealloc(
                (*A).ja as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz_t),
            ) as *mut libc::c_int;
            if (*A).size > 0 as libc::c_int {
                if !((*A).a).is_null() {
                    let ref mut fresh20 = (*A).a;
                    *fresh20 = grealloc((*A).a, ((*A).size as libc::c_ulong).wrapping_mul(nz_t));
                } else {
                    let ref mut fresh21 = (*A).a;
                    *fresh21 = gmalloc(((*A).size as libc::c_ulong).wrapping_mul(nz_t));
                }
            }
        }
        1 | 0 | _ => {
            let ref mut fresh22 = (*A).ja;
            *fresh22 = grealloc(
                (*A).ja as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz_t),
            ) as *mut libc::c_int;
            if (*A).size > 0 as libc::c_int {
                if !((*A).a).is_null() {
                    let ref mut fresh23 = (*A).a;
                    *fresh23 = grealloc((*A).a, ((*A).size as libc::c_ulong).wrapping_mul(nz_t));
                } else {
                    let ref mut fresh24 = (*A).a;
                    *fresh24 = gmalloc(((*A).size as libc::c_ulong).wrapping_mul(nz_t));
                }
            }
        }
    }
    (*A).nzmax = nz;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_new(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut nz: libc::c_int,
    mut type_0: libc::c_int,
    mut format: libc::c_int,
) -> SparseMatrix {
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut sz: size_t = 0;
    sz = size_of_matrix_type(type_0);
    A = SparseMatrix_init(m, n, type_0, sz, format);
    if nz > 0 as libc::c_int {
        A = SparseMatrix_alloc(A, nz);
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_general_new(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut nz: libc::c_int,
    mut type_0: libc::c_int,
    mut sz: size_t,
    mut format: libc::c_int,
) -> SparseMatrix {
    let mut A: SparseMatrix = 0 as *mut SparseMatrix_struct;
    A = SparseMatrix_init(m, n, type_0, sz, format);
    if nz > 0 as libc::c_int {
        A = SparseMatrix_alloc(A, nz);
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_delete(mut A: SparseMatrix) {
    if A.is_null() {
        return;
    }
    free((*A).ia as *mut libc::c_void);
    free((*A).ja as *mut libc::c_void);
    free((*A).a);
    free(A as *mut libc::c_void);
}
unsafe extern "C" fn SparseMatrix_print_csr(mut c: *mut libc::c_char, mut A: SparseMatrix) {
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ai: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    if (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void SparseMatrix_print_csr(char *, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    printf(
        b"%s\n SparseArray[{\0" as *const u8 as *const libc::c_char,
        c,
    );
    ia = (*A).ia;
    ja = (*A).ja;
    a = (*A).a as *mut libc::c_double;
    match (*A).type_0 {
        1 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    printf(
                        b"{%d, %d}->%f\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                        *a.offset(j as isize),
                    );
                    if j != *ia.offset(m as isize) - 1 as libc::c_int {
                        printf(b",\0" as *const u8 as *const libc::c_char);
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        2 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    printf(
                        b"{%d, %d}->%f + %f I\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                        *a.offset((2 as libc::c_int * j) as isize),
                        *a.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize),
                    );
                    if j != *ia.offset(m as isize) - 1 as libc::c_int {
                        printf(b",\0" as *const u8 as *const libc::c_char);
                    }
                    j += 1;
                }
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            ai = (*A).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    printf(
                        b"{%d, %d}->%d\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                        *ai.offset(j as isize),
                    );
                    if j != *ia.offset(m as isize) - 1 as libc::c_int {
                        printf(b",\0" as *const u8 as *const libc::c_char);
                    }
                    j += 1;
                }
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        8 => {
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    printf(
                        b"{%d, %d}->_\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                    );
                    if j != *ia.offset(m as isize) - 1 as libc::c_int {
                        printf(b",\0" as *const u8 as *const libc::c_char);
                    }
                    j += 1;
                }
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        16 => return,
        _ => return,
    }
    printf(
        b"},{%d, %d}]\n\0" as *const u8 as *const libc::c_char,
        m,
        (*A).n,
    );
}
unsafe extern "C" fn SparseMatrix_print_coord(mut c: *mut libc::c_char, mut A: SparseMatrix) {
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ai: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    if (*A).format == FORMAT_COORD as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_COORD\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"void SparseMatrix_print_coord(char *, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    printf(
        b"%s\n SparseArray[{\0" as *const u8 as *const libc::c_char,
        c,
    );
    ia = (*A).ia;
    ja = (*A).ja;
    a = (*A).a as *mut libc::c_double;
    match (*A).type_0 {
        1 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                printf(
                    b"{%d, %d}->%f\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                    *a.offset(i as isize),
                );
                if i != (*A).nz - 1 as libc::c_int {
                    printf(b",\0" as *const u8 as *const libc::c_char);
                }
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                printf(
                    b"{%d, %d}->%f + %f I\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                    *a.offset((2 as libc::c_int * i) as isize),
                    *a.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize),
                );
                if i != (*A).nz - 1 as libc::c_int {
                    printf(b",\0" as *const u8 as *const libc::c_char);
                }
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            ai = (*A).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                printf(
                    b"{%d, %d}->%d\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                    *ai.offset(i as isize),
                );
                if i != (*A).nz {
                    printf(b",\0" as *const u8 as *const libc::c_char);
                }
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        8 => {
            i = 0 as libc::c_int;
            while i < (*A).nz {
                printf(
                    b"{%d, %d}->_\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                );
                if i != (*A).nz - 1 as libc::c_int {
                    printf(b",\0" as *const u8 as *const libc::c_char);
                }
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        16 => return,
        _ => return,
    }
    printf(
        b"},{%d, %d}]\n\0" as *const u8 as *const libc::c_char,
        m,
        (*A).n,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_print(mut c: *mut libc::c_char, mut A: SparseMatrix) {
    match (*A).format {
        1 => {
            SparseMatrix_print_csr(c, A);
        }
        0 => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                537 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"void SparseMatrix_print(char *, SparseMatrix)\0",
                ))
                .as_ptr(),
            );
        }
        2 => {
            SparseMatrix_print_coord(c, A);
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                544 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"void SparseMatrix_print(char *, SparseMatrix)\0",
                ))
                .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn SparseMatrix_export_csr(mut f: *mut FILE, mut A: SparseMatrix) {
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ai: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    match (*A).type_0 {
        1 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        2 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate complex general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        4 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate integer general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        8 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate pattern general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        16 => return,
        _ => return,
    }
    fprintf(
        f,
        b"%d %d %d\n\0" as *const u8 as *const libc::c_char,
        (*A).m,
        (*A).n,
        (*A).nz,
    );
    ia = (*A).ia;
    ja = (*A).ja;
    a = (*A).a as *mut libc::c_double;
    match (*A).type_0 {
        1 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    fprintf(
                        f,
                        b"%d %d %16.8g\n\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                        *a.offset(j as isize),
                    );
                    j += 1;
                }
                i += 1;
            }
        }
        2 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    fprintf(
                        f,
                        b"%d %d %16.8g %16.8g\n\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                        *a.offset((2 as libc::c_int * j) as isize),
                        *a.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize),
                    );
                    j += 1;
                }
                i += 1;
            }
        }
        4 => {
            ai = (*A).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    fprintf(
                        f,
                        b"%d %d %d\n\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                        *ai.offset(j as isize),
                    );
                    j += 1;
                }
                i += 1;
            }
        }
        8 => {
            i = 0 as libc::c_int;
            while i < m {
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    fprintf(
                        f,
                        b"%d %d\n\0" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        *ja.offset(j as isize) + 1 as libc::c_int,
                    );
                    j += 1;
                }
                i += 1;
            }
        }
        16 => return,
        _ => return,
    };
}
unsafe extern "C" fn SparseMatrix_export_coord(mut f: *mut FILE, mut A: SparseMatrix) {
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ai: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    match (*A).type_0 {
        1 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate real general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        2 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate complex general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        4 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate integer general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        8 => {
            fprintf(
                f,
                b"%%%%MatrixMarket matrix coordinate pattern general\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        16 => return,
        _ => return,
    }
    fprintf(
        f,
        b"%d %d %d\n\0" as *const u8 as *const libc::c_char,
        (*A).m,
        (*A).n,
        (*A).nz,
    );
    ia = (*A).ia;
    ja = (*A).ja;
    a = (*A).a as *mut libc::c_double;
    match (*A).type_0 {
        1 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                fprintf(
                    f,
                    b"%d %d %16.8g\n\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                    *a.offset(i as isize),
                );
                i += 1;
            }
        }
        2 => {
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                fprintf(
                    f,
                    b"%d %d %16.8g %16.8g\n\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                    *a.offset((2 as libc::c_int * i) as isize),
                    *a.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize),
                );
                i += 1;
            }
        }
        4 => {
            ai = (*A).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                fprintf(
                    f,
                    b"%d %d %d\n\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                    *ai.offset(i as isize),
                );
                i += 1;
            }
        }
        8 => {
            i = 0 as libc::c_int;
            while i < (*A).nz {
                fprintf(
                    f,
                    b"%d %d\n\0" as *const u8 as *const libc::c_char,
                    *ia.offset(i as isize) + 1 as libc::c_int,
                    *ja.offset(i as isize) + 1 as libc::c_int,
                );
                i += 1;
            }
        }
        16 => return,
        _ => return,
    };
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_export(mut f: *mut FILE, mut A: SparseMatrix) {
    match (*A).format {
        1 => {
            SparseMatrix_export_csr(f, A);
        }
        0 => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                690 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void SparseMatrix_export(FILE *, SparseMatrix)\0",
                ))
                .as_ptr(),
            );
        }
        2 => {
            SparseMatrix_export_coord(f, A);
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                697 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void SparseMatrix_export(FILE *, SparseMatrix)\0",
                ))
                .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_from_coordinate_format(mut A: SparseMatrix) -> SparseMatrix {
    let mut irn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jcn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_void = (*A).a;
    if (*A).format == FORMAT_COORD as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_COORD\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            708 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"SparseMatrix SparseMatrix_from_coordinate_format(SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).format != FORMAT_COORD as libc::c_int {
        return 0 as SparseMatrix;
    }
    irn = (*A).ia;
    jcn = (*A).ja;
    return SparseMatrix_from_coordinate_arrays(
        (*A).nz,
        (*A).m,
        (*A).n,
        irn,
        jcn,
        a,
        (*A).type_0,
        (*A).size as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_from_coordinate_format_not_compacted(
    mut A: SparseMatrix,
) -> SparseMatrix {
    let mut irn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jcn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_void = (*A).a;
    if (*A).format == FORMAT_COORD as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_COORD\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            723 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 77], &[libc::c_char; 77]>(
                b"SparseMatrix SparseMatrix_from_coordinate_format_not_compacted(SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).format != FORMAT_COORD as libc::c_int {
        return 0 as SparseMatrix;
    }
    irn = (*A).ia;
    jcn = (*A).ja;
    return SparseMatrix_from_coordinate_arrays_not_compacted(
        (*A).nz,
        (*A).m,
        (*A).n,
        irn,
        jcn,
        a,
        (*A).type_0,
        (*A).size as size_t,
    );
}
unsafe extern "C" fn SparseMatrix_from_coordinate_arrays_internal(
    mut nz: libc::c_int,
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut irn: *mut libc::c_int,
    mut jcn: *mut libc::c_int,
    mut val0: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut sz: size_t,
    mut sum_repeated: libc::c_int,
) -> SparseMatrix {
    let mut A: SparseMatrix = 0 as SparseMatrix;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ai: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vali: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    if m > 0 as libc::c_int && n > 0 as libc::c_int && nz >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"m > 0 && n > 0 && nz >= 0\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            747 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    if m <= 0 as libc::c_int || n <= 0 as libc::c_int || nz < 0 as libc::c_int {
        return 0 as SparseMatrix;
    }
    A = SparseMatrix_general_new(m, n, nz, type_0, sz, FORMAT_CSR as libc::c_int);
    if !A.is_null() {
    } else {
        __assert_fail(
            b"A\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            751 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
            ))
                .as_ptr(),
        );
    }
    if A.is_null() {
        return 0 as SparseMatrix;
    }
    ia = (*A).ia;
    ja = (*A).ja;
    i = 0 as libc::c_int;
    while i <= m {
        *ia.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    match type_0 {
        1 => {
            val = val0 as *mut libc::c_double;
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < nz {
                if *irn.offset(i as isize) < 0 as libc::c_int
                    || *irn.offset(i as isize) >= m
                    || *jcn.offset(i as isize) < 0 as libc::c_int
                    || *jcn.offset(i as isize) >= n
                {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                        766 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 113],
                            &[libc::c_char; 113],
                        >(
                            b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
                        ))
                            .as_ptr(),
                    );
                    return 0 as SparseMatrix;
                }
                let ref mut fresh25 =
                    *ia.offset((*irn.offset(i as isize) + 1 as libc::c_int) as isize);
                *fresh25 += 1;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < m {
                *ia.offset((i + 1 as libc::c_int) as isize) += *ia.offset(i as isize);
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < nz {
                *a.offset(*ia.offset(*irn.offset(i as isize) as isize) as isize) =
                    *val.offset(i as isize);
                let ref mut fresh26 = *ia.offset(*irn.offset(i as isize) as isize);
                let fresh27 = *fresh26;
                *fresh26 = *fresh26 + 1;
                *ja.offset(fresh27 as isize) = *jcn.offset(i as isize);
                i += 1;
            }
            i = m;
            while i > 0 as libc::c_int {
                *ia.offset(i as isize) = *ia.offset((i - 1 as libc::c_int) as isize);
                i -= 1;
            }
            *ia.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        }
        2 => {
            val = val0 as *mut libc::c_double;
            a = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < nz {
                if *irn.offset(i as isize) < 0 as libc::c_int
                    || *irn.offset(i as isize) >= m
                    || *jcn.offset(i as isize) < 0 as libc::c_int
                    || *jcn.offset(i as isize) >= n
                {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                        784 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 113],
                            &[libc::c_char; 113],
                        >(
                            b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
                        ))
                            .as_ptr(),
                    );
                    return 0 as SparseMatrix;
                }
                let ref mut fresh28 =
                    *ia.offset((*irn.offset(i as isize) + 1 as libc::c_int) as isize);
                *fresh28 += 1;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < m {
                *ia.offset((i + 1 as libc::c_int) as isize) += *ia.offset(i as isize);
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < nz {
                let fresh29 = val;
                val = val.offset(1);
                *a.offset(
                    (2 as libc::c_int * *ia.offset(*irn.offset(i as isize) as isize)) as isize,
                ) = *fresh29;
                let fresh30 = val;
                val = val.offset(1);
                *a.offset(
                    (2 as libc::c_int * *ia.offset(*irn.offset(i as isize) as isize)
                        + 1 as libc::c_int) as isize,
                ) = *fresh30;
                let ref mut fresh31 = *ia.offset(*irn.offset(i as isize) as isize);
                let fresh32 = *fresh31;
                *fresh31 = *fresh31 + 1;
                *ja.offset(fresh32 as isize) = *jcn.offset(i as isize);
                i += 1;
            }
            i = m;
            while i > 0 as libc::c_int {
                *ia.offset(i as isize) = *ia.offset((i - 1 as libc::c_int) as isize);
                i -= 1;
            }
            *ia.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        }
        4 => {
            vali = val0 as *mut libc::c_int;
            ai = (*A).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < nz {
                if *irn.offset(i as isize) < 0 as libc::c_int
                    || *irn.offset(i as isize) >= m
                    || *jcn.offset(i as isize) < 0 as libc::c_int
                    || *jcn.offset(i as isize) >= n
                {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                        803 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 113],
                            &[libc::c_char; 113],
                        >(
                            b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
                        ))
                            .as_ptr(),
                    );
                    return 0 as SparseMatrix;
                }
                let ref mut fresh33 =
                    *ia.offset((*irn.offset(i as isize) + 1 as libc::c_int) as isize);
                *fresh33 += 1;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < m {
                *ia.offset((i + 1 as libc::c_int) as isize) += *ia.offset(i as isize);
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < nz {
                *ai.offset(*ia.offset(*irn.offset(i as isize) as isize) as isize) =
                    *vali.offset(i as isize);
                let ref mut fresh34 = *ia.offset(*irn.offset(i as isize) as isize);
                let fresh35 = *fresh34;
                *fresh34 = *fresh34 + 1;
                *ja.offset(fresh35 as isize) = *jcn.offset(i as isize);
                i += 1;
            }
            i = m;
            while i > 0 as libc::c_int {
                *ia.offset(i as isize) = *ia.offset((i - 1 as libc::c_int) as isize);
                i -= 1;
            }
            *ia.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        }
        8 => {
            i = 0 as libc::c_int;
            while i < nz {
                if *irn.offset(i as isize) < 0 as libc::c_int
                    || *irn.offset(i as isize) >= m
                    || *jcn.offset(i as isize) < 0 as libc::c_int
                    || *jcn.offset(i as isize) >= n
                {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                        819 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 113],
                            &[libc::c_char; 113],
                        >(
                            b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
                        ))
                            .as_ptr(),
                    );
                    return 0 as SparseMatrix;
                }
                let ref mut fresh36 =
                    *ia.offset((*irn.offset(i as isize) + 1 as libc::c_int) as isize);
                *fresh36 += 1;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < m {
                *ia.offset((i + 1 as libc::c_int) as isize) += *ia.offset(i as isize);
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < nz {
                let ref mut fresh37 = *ia.offset(*irn.offset(i as isize) as isize);
                let fresh38 = *fresh37;
                *fresh37 = *fresh37 + 1;
                *ja.offset(fresh38 as isize) = *jcn.offset(i as isize);
                i += 1;
            }
            i = m;
            while i > 0 as libc::c_int {
                *ia.offset(i as isize) = *ia.offset((i - 1 as libc::c_int) as isize);
                i -= 1;
            }
            *ia.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        }
        16 => {
            i = 0 as libc::c_int;
            while i < nz {
                if *irn.offset(i as isize) < 0 as libc::c_int
                    || *irn.offset(i as isize) >= m
                    || *jcn.offset(i as isize) < 0 as libc::c_int
                    || *jcn.offset(i as isize) >= n
                {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                        834 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 113],
                            &[libc::c_char; 113],
                        >(
                            b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
                        ))
                            .as_ptr(),
                    );
                    return 0 as SparseMatrix;
                }
                let ref mut fresh39 =
                    *ia.offset((*irn.offset(i as isize) + 1 as libc::c_int) as isize);
                *fresh39 += 1;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < m {
                *ia.offset((i + 1 as libc::c_int) as isize) += *ia.offset(i as isize);
                i += 1;
            }
            memcpy(
                (*A).a,
                val0,
                ((*A).size as libc::c_ulong).wrapping_mul(nz as size_t),
            );
            i = 0 as libc::c_int;
            while i < nz {
                let ref mut fresh40 = *ia.offset(*irn.offset(i as isize) as isize);
                let fresh41 = *fresh40;
                *fresh40 = *fresh40 + 1;
                *ja.offset(fresh41 as isize) = *jcn.offset(i as isize);
                i += 1;
            }
            i = m;
            while i > 0 as libc::c_int {
                *ia.offset(i as isize) = *ia.offset((i - 1 as libc::c_int) as isize);
                i -= 1;
            }
            *ia.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                848 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"SparseMatrix SparseMatrix_from_coordinate_arrays_internal(int, int, int, int *, int *, void *, int, size_t, int)\0",
                ))
                    .as_ptr(),
            );
            return 0 as SparseMatrix;
        }
    }
    (*A).nz = nz;
    if sum_repeated != 0 {
        A = SparseMatrix_sum_repeat_entries(A, sum_repeated);
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_from_coordinate_arrays(
    mut nz: libc::c_int,
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut irn: *mut libc::c_int,
    mut jcn: *mut libc::c_int,
    mut val0: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut sz: size_t,
) -> SparseMatrix {
    return SparseMatrix_from_coordinate_arrays_internal(
        nz,
        m,
        n,
        irn,
        jcn,
        val0,
        type_0,
        sz,
        SUM_REPEATED_ALL as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_from_coordinate_arrays_not_compacted(
    mut nz: libc::c_int,
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut irn: *mut libc::c_int,
    mut jcn: *mut libc::c_int,
    mut val0: *mut libc::c_void,
    mut type_0: libc::c_int,
    mut sz: size_t,
) -> SparseMatrix {
    return SparseMatrix_from_coordinate_arrays_internal(
        nz,
        m,
        n,
        irn,
        jcn,
        val0,
        type_0,
        sz,
        SUM_REPEATED_NONE as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_add(
    mut A: SparseMatrix,
    mut B: SparseMatrix,
) -> SparseMatrix {
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut C: SparseMatrix = 0 as SparseMatrix;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut ib: *mut libc::c_int = (*B).ia;
    let mut jb: *mut libc::c_int = (*B).ja;
    let mut ic: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jc: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    let mut nzmax: libc::c_int = 0;
    if !A.is_null() && !B.is_null() {
    } else {
        __assert_fail(
            b"A && B\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            877 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"SparseMatrix SparseMatrix_add(SparseMatrix, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).format == (*B).format && (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == B->format && A->format == FORMAT_CSR\0" as *const u8
                as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            878 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"SparseMatrix SparseMatrix_add(SparseMatrix, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).type_0 == (*B).type_0 {
    } else {
        __assert_fail(
            b"A->type == B->type\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            879 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"SparseMatrix SparseMatrix_add(SparseMatrix, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    m = (*A).m;
    n = (*A).n;
    if m != (*B).m || n != (*B).n {
        return 0 as SparseMatrix;
    }
    nzmax = (*A).nz + (*B).nz;
    C = SparseMatrix_new(m, n, nzmax, (*A).type_0, FORMAT_CSR as libc::c_int);
    if !C.is_null() {
        ic = (*C).ia;
        jc = (*C).ja;
        mask = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as size_t),
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < n {
            *mask.offset(i as isize) = -(1 as libc::c_int);
            i += 1;
        }
        nz = 0 as libc::c_int;
        *ic.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        match (*A).type_0 {
            1 => {
                let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
                let mut b: *mut libc::c_double = (*B).a as *mut libc::c_double;
                let mut c: *mut libc::c_double = (*C).a as *mut libc::c_double;
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *mask.offset(*ja.offset(j as isize) as isize) = nz;
                        *jc.offset(nz as isize) = *ja.offset(j as isize);
                        *c.offset(nz as isize) = *a.offset(j as isize);
                        nz += 1;
                        j += 1;
                    }
                    j = *ib.offset(i as isize);
                    while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                        if *mask.offset(*jb.offset(j as isize) as isize) < *ic.offset(i as isize) {
                            *jc.offset(nz as isize) = *jb.offset(j as isize);
                            let fresh42 = nz;
                            nz = nz + 1;
                            *c.offset(fresh42 as isize) = *b.offset(j as isize);
                        } else {
                            *c.offset(*mask.offset(*jb.offset(j as isize) as isize) as isize) +=
                                *b.offset(j as isize);
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
            }
            2 => {
                let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
                let mut b_0: *mut libc::c_double = (*B).a as *mut libc::c_double;
                let mut c_0: *mut libc::c_double = (*C).a as *mut libc::c_double;
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *mask.offset(*ja.offset(j as isize) as isize) = nz;
                        *jc.offset(nz as isize) = *ja.offset(j as isize);
                        *c_0.offset((2 as libc::c_int * nz) as isize) =
                            *a_0.offset((2 as libc::c_int * j) as isize);
                        *c_0.offset((2 as libc::c_int * nz + 1 as libc::c_int) as isize) =
                            *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                        nz += 1;
                        j += 1;
                    }
                    j = *ib.offset(i as isize);
                    while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                        if *mask.offset(*jb.offset(j as isize) as isize) < *ic.offset(i as isize) {
                            *jc.offset(nz as isize) = *jb.offset(j as isize);
                            *c_0.offset((2 as libc::c_int * nz) as isize) =
                                *b_0.offset((2 as libc::c_int * j) as isize);
                            *c_0.offset((2 as libc::c_int * nz + 1 as libc::c_int) as isize) =
                                *b_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                            nz += 1;
                        } else {
                            *c_0.offset(
                                (2 as libc::c_int * *mask.offset(*jb.offset(j as isize) as isize))
                                    as isize,
                            ) += *b_0.offset((2 as libc::c_int * j) as isize);
                            *c_0.offset(
                                (2 as libc::c_int * *mask.offset(*jb.offset(j as isize) as isize)
                                    + 1 as libc::c_int) as isize,
                            ) += *b_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
            }
            4 => {
                let mut a_1: *mut libc::c_int = (*A).a as *mut libc::c_int;
                let mut b_1: *mut libc::c_int = (*B).a as *mut libc::c_int;
                let mut c_1: *mut libc::c_int = (*C).a as *mut libc::c_int;
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *mask.offset(*ja.offset(j as isize) as isize) = nz;
                        *jc.offset(nz as isize) = *ja.offset(j as isize);
                        *c_1.offset(nz as isize) = *a_1.offset(j as isize);
                        nz += 1;
                        j += 1;
                    }
                    j = *ib.offset(i as isize);
                    while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                        if *mask.offset(*jb.offset(j as isize) as isize) < *ic.offset(i as isize) {
                            *jc.offset(nz as isize) = *jb.offset(j as isize);
                            *c_1.offset(nz as isize) = *b_1.offset(j as isize);
                            nz += 1;
                        } else {
                            *c_1.offset(*mask.offset(*jb.offset(j as isize) as isize) as isize) +=
                                *b_1.offset(j as isize);
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
            }
            8 => {
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *mask.offset(*ja.offset(j as isize) as isize) = nz;
                        *jc.offset(nz as isize) = *ja.offset(j as isize);
                        nz += 1;
                        j += 1;
                    }
                    j = *ib.offset(i as isize);
                    while j < *ib.offset((i + 1 as libc::c_int) as isize) {
                        if *mask.offset(*jb.offset(j as isize) as isize) < *ic.offset(i as isize) {
                            *jc.offset(nz as isize) = *jb.offset(j as isize);
                            nz += 1;
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
            }
            16 | _ => {}
        }
        (*C).nz = nz;
    }
    free(mask as *mut libc::c_void);
    return C;
}
unsafe extern "C" fn SparseMatrix_multiply_dense1(
    mut A: SparseMatrix,
    mut v: *mut libc::c_double,
    mut res: *mut *mut libc::c_double,
    mut dim: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    if (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1007 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"void SparseMatrix_multiply_dense1(SparseMatrix, double *, double **, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int {
    } else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1008 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"void SparseMatrix_multiply_dense1(SparseMatrix, double *, double **, int)\0",
            ))
            .as_ptr(),
        );
    }
    a = (*A).a as *mut libc::c_double;
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    u = *res;
    if u.is_null() {
        u = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(m as size_t)
                .wrapping_mul(dim as size_t),
        ) as *mut libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < m {
        k = 0 as libc::c_int;
        while k < dim {
            *u.offset((i * dim + k) as isize) = 0.0f64;
            k += 1;
        }
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            k = 0 as libc::c_int;
            while k < dim {
                *u.offset((i * dim + k) as isize) +=
                    *a.offset(j as isize) * *v.offset((*ja.offset(j as isize) * dim + k) as isize);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    *res = u;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_multiply_dense(
    mut A: SparseMatrix,
    mut v: *mut libc::c_double,
    mut res: *mut *mut libc::c_double,
    mut dim: libc::c_int,
) {
    SparseMatrix_multiply_dense1(A, v, res, dim);
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_multiply_vector(
    mut A: SparseMatrix,
    mut v: *mut libc::c_double,
    mut res: *mut *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ai: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_CSR\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1039 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                b"void SparseMatrix_multiply_vector(SparseMatrix, double *, double **)\0",
            ))
            .as_ptr(),
        );
    }
    if (*A).type_0 == MATRIX_TYPE_REAL as libc::c_int
        || (*A).type_0 == MATRIX_TYPE_INTEGER as libc::c_int
    {
    } else {
        __assert_fail(
            b"A->type == MATRIX_TYPE_REAL || A->type == MATRIX_TYPE_INTEGER\0" as *const u8
                as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1040 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                b"void SparseMatrix_multiply_vector(SparseMatrix, double *, double **)\0",
            ))
            .as_ptr(),
        );
    }
    ia = (*A).ia;
    ja = (*A).ja;
    m = (*A).m;
    u = *res;
    match (*A).type_0 {
        1 => {
            a = (*A).a as *mut libc::c_double;
            if !v.is_null() {
                if u.is_null() {
                    u = gmalloc(
                        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                            .wrapping_mul(m as size_t),
                    ) as *mut libc::c_double;
                }
                i = 0 as libc::c_int;
                while i < m {
                    *u.offset(i as isize) = 0.0f64;
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *u.offset(i as isize) +=
                            *a.offset(j as isize) * *v.offset(*ja.offset(j as isize) as isize);
                        j += 1;
                    }
                    i += 1;
                }
            } else {
                if u.is_null() {
                    u = gmalloc(
                        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                            .wrapping_mul(m as size_t),
                    ) as *mut libc::c_double;
                }
                i = 0 as libc::c_int;
                while i < m {
                    *u.offset(i as isize) = 0.0f64;
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *u.offset(i as isize) += *a.offset(j as isize);
                        j += 1;
                    }
                    i += 1;
                }
            }
        }
        4 => {
            ai = (*A).a as *mut libc::c_int;
            if !v.is_null() {
                if u.is_null() {
                    u = gmalloc(
                        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                            .wrapping_mul(m as size_t),
                    ) as *mut libc::c_double;
                }
                i = 0 as libc::c_int;
                while i < m {
                    *u.offset(i as isize) = 0.0f64;
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *u.offset(i as isize) += *ai.offset(j as isize) as libc::c_double
                            * *v.offset(*ja.offset(j as isize) as isize);
                        j += 1;
                    }
                    i += 1;
                }
            } else {
                if u.is_null() {
                    u = gmalloc(
                        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                            .wrapping_mul(m as size_t),
                    ) as *mut libc::c_double;
                }
                i = 0 as libc::c_int;
                while i < m {
                    *u.offset(i as isize) = 0.0f64;
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        *u.offset(i as isize) += *ai.offset(j as isize) as libc::c_double;
                        j += 1;
                    }
                    i += 1;
                }
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                1091 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"void SparseMatrix_multiply_vector(SparseMatrix, double *, double **)\0",
                ))
                .as_ptr(),
            );
            u = 0 as *mut libc::c_double;
        }
    }
    *res = u;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_multiply(
    mut A: SparseMatrix,
    mut B: SparseMatrix,
) -> SparseMatrix {
    let mut current_block: u64;
    let mut m: libc::c_int = 0;
    let mut C: SparseMatrix = 0 as SparseMatrix;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut ib: *mut libc::c_int = (*B).ia;
    let mut jb: *mut libc::c_int = (*B).ja;
    let mut ic: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jc: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    if (*A).format == (*B).format && (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == B->format && A->format == FORMAT_CSR\0" as *const u8
                as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1105 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"SparseMatrix SparseMatrix_multiply(SparseMatrix, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    m = (*A).m;
    if (*A).n != (*B).m {
        return 0 as SparseMatrix;
    }
    if (*A).type_0 != (*B).type_0 {
        return 0 as SparseMatrix;
    }
    type_0 = (*A).type_0;
    mask = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul((*B).n as size_t),
    ) as *mut libc::c_int;
    if mask.is_null() {
        return 0 as SparseMatrix;
    }
    i = 0 as libc::c_int;
    while i < (*B).n {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            k = *ib.offset(jj as isize);
            while k < *ib.offset((jj + 1 as libc::c_int) as isize) {
                if *mask.offset(*jb.offset(k as isize) as isize) != -i - 2 as libc::c_int {
                    if nz + 1 as libc::c_int <= nz {
                        return 0 as SparseMatrix;
                    }
                    nz += 1;
                    *mask.offset(*jb.offset(k as isize) as isize) = -i - 2 as libc::c_int;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    C = SparseMatrix_new(m, (*B).n, nz, type_0, FORMAT_CSR as libc::c_int);
    if !C.is_null() {
        ic = (*C).ia;
        jc = (*C).ja;
        nz = 0 as libc::c_int;
        match type_0 {
            1 => {
                let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
                let mut b: *mut libc::c_double = (*B).a as *mut libc::c_double;
                let mut c: *mut libc::c_double = (*C).a as *mut libc::c_double;
                *ic.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        jj = *ja.offset(j as isize);
                        k = *ib.offset(jj as isize);
                        while k < *ib.offset((jj + 1 as libc::c_int) as isize) {
                            if *mask.offset(*jb.offset(k as isize) as isize)
                                < *ic.offset(i as isize)
                            {
                                *mask.offset(*jb.offset(k as isize) as isize) = nz;
                                *jc.offset(nz as isize) = *jb.offset(k as isize);
                                *c.offset(nz as isize) =
                                    *a.offset(j as isize) * *b.offset(k as isize);
                                nz += 1;
                            } else {
                                if *jc
                                    .offset(*mask.offset(*jb.offset(k as isize) as isize) as isize)
                                    == *jb.offset(k as isize)
                                {
                                } else {
                                    __assert_fail(
                                        b"jc[mask[jb[k]]] == jb[k]\0" as *const u8
                                            as *const libc::c_char,
                                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                        1165 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 63],
                                            &[libc::c_char; 63],
                                        >(
                                            b"SparseMatrix SparseMatrix_multiply(SparseMatrix, SparseMatrix)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                *c
                                    .offset(
                                        *mask.offset(*jb.offset(k as isize) as isize) as isize,
                                    ) += *a.offset(j as isize) * *b.offset(k as isize);
                            }
                            k += 1;
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
                current_block = 7923086311623215889;
            }
            2 => {
                let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
                let mut b_0: *mut libc::c_double = (*B).a as *mut libc::c_double;
                let mut c_0: *mut libc::c_double = (*C).a as *mut libc::c_double;
                a_0 = (*A).a as *mut libc::c_double;
                b_0 = (*B).a as *mut libc::c_double;
                c_0 = (*C).a as *mut libc::c_double;
                *ic.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        jj = *ja.offset(j as isize);
                        k = *ib.offset(jj as isize);
                        while k < *ib.offset((jj + 1 as libc::c_int) as isize) {
                            if *mask.offset(*jb.offset(k as isize) as isize)
                                < *ic.offset(i as isize)
                            {
                                *mask.offset(*jb.offset(k as isize) as isize) = nz;
                                *jc.offset(nz as isize) = *jb.offset(k as isize);
                                *c_0.offset((2 as libc::c_int * nz) as isize) = *a_0
                                    .offset((2 as libc::c_int * j) as isize)
                                    * *b_0.offset((2 as libc::c_int * k) as isize)
                                    - *a_0
                                        .offset((2 as libc::c_int * j + 1 as libc::c_int) as isize)
                                        * *b_0.offset(
                                            (2 as libc::c_int * k + 1 as libc::c_int) as isize,
                                        );
                                *c_0.offset((2 as libc::c_int * nz + 1 as libc::c_int) as isize) =
                                    *a_0.offset((2 as libc::c_int * j) as isize)
                                        * *b_0.offset(
                                            (2 as libc::c_int * k + 1 as libc::c_int) as isize,
                                        )
                                        + *a_0.offset(
                                            (2 as libc::c_int * j + 1 as libc::c_int) as isize,
                                        ) * *b_0.offset((2 as libc::c_int * k) as isize);
                                nz += 1;
                            } else {
                                if *jc
                                    .offset(*mask.offset(*jb.offset(k as isize) as isize) as isize)
                                    == *jb.offset(k as isize)
                                {
                                } else {
                                    __assert_fail(
                                        b"jc[mask[jb[k]]] == jb[k]\0" as *const u8
                                            as *const libc::c_char,
                                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                        1194 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 63],
                                            &[libc::c_char; 63],
                                        >(
                                            b"SparseMatrix SparseMatrix_multiply(SparseMatrix, SparseMatrix)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                *c_0.offset(
                                    (2 as libc::c_int
                                        * *mask.offset(*jb.offset(k as isize) as isize))
                                        as isize,
                                ) += *a_0.offset((2 as libc::c_int * j) as isize)
                                    * *b_0.offset((2 as libc::c_int * k) as isize)
                                    - *a_0
                                        .offset((2 as libc::c_int * j + 1 as libc::c_int) as isize)
                                        * *b_0.offset(
                                            (2 as libc::c_int * k + 1 as libc::c_int) as isize,
                                        );
                                *c_0.offset(
                                    (2 as libc::c_int
                                        * *mask.offset(*jb.offset(k as isize) as isize)
                                        + 1 as libc::c_int)
                                        as isize,
                                ) += *a_0.offset((2 as libc::c_int * j) as isize)
                                    * *b_0
                                        .offset((2 as libc::c_int * k + 1 as libc::c_int) as isize)
                                    + *a_0
                                        .offset((2 as libc::c_int * j + 1 as libc::c_int) as isize)
                                        * *b_0.offset((2 as libc::c_int * k) as isize);
                            }
                            k += 1;
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
                current_block = 7923086311623215889;
            }
            4 => {
                let mut a_1: *mut libc::c_int = (*A).a as *mut libc::c_int;
                let mut b_1: *mut libc::c_int = (*B).a as *mut libc::c_int;
                let mut c_1: *mut libc::c_int = (*C).a as *mut libc::c_int;
                *ic.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        jj = *ja.offset(j as isize);
                        k = *ib.offset(jj as isize);
                        while k < *ib.offset((jj + 1 as libc::c_int) as isize) {
                            if *mask.offset(*jb.offset(k as isize) as isize)
                                < *ic.offset(i as isize)
                            {
                                *mask.offset(*jb.offset(k as isize) as isize) = nz;
                                *jc.offset(nz as isize) = *jb.offset(k as isize);
                                *c_1.offset(nz as isize) =
                                    *a_1.offset(j as isize) * *b_1.offset(k as isize);
                                nz += 1;
                            } else {
                                if *jc
                                    .offset(*mask.offset(*jb.offset(k as isize) as isize) as isize)
                                    == *jb.offset(k as isize)
                                {
                                } else {
                                    __assert_fail(
                                        b"jc[mask[jb[k]]] == jb[k]\0" as *const u8
                                            as *const libc::c_char,
                                        b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                        1220 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 63],
                                            &[libc::c_char; 63],
                                        >(
                                            b"SparseMatrix SparseMatrix_multiply(SparseMatrix, SparseMatrix)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                *c_1.offset(
                                    *mask.offset(*jb.offset(k as isize) as isize) as isize
                                ) += *a_1.offset(j as isize) * *b_1.offset(k as isize);
                            }
                            k += 1;
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
                current_block = 7923086311623215889;
            }
            8 => {
                *ic.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < m {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        jj = *ja.offset(j as isize);
                        k = *ib.offset(jj as isize);
                        while k < *ib.offset((jj + 1 as libc::c_int) as isize) {
                            if *mask.offset(*jb.offset(k as isize) as isize)
                                < *ic.offset(i as isize)
                            {
                                *mask.offset(*jb.offset(k as isize) as isize) = nz;
                                *jc.offset(nz as isize) = *jb.offset(k as isize);
                                nz += 1;
                            } else if *jc
                                .offset(*mask.offset(*jb.offset(k as isize) as isize) as isize)
                                == *jb.offset(k as isize)
                            {
                            } else {
                                __assert_fail(
                                    b"jc[mask[jb[k]]] == jb[k]\0" as *const u8
                                        as *const libc::c_char,
                                    b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                    1240 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 63],
                                        &[libc::c_char; 63],
                                    >(
                                        b"SparseMatrix SparseMatrix_multiply(SparseMatrix, SparseMatrix)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            k += 1;
                        }
                        j += 1;
                    }
                    *ic.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
                current_block = 7923086311623215889;
            }
            16 | _ => {
                SparseMatrix_delete(C);
                C = 0 as SparseMatrix;
                current_block = 4271686246892568934;
            }
        }
        match current_block {
            4271686246892568934 => {}
            _ => {
                (*C).nz = nz;
            }
        }
    }
    free(mask as *mut libc::c_void);
    return C;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_multiply3(
    mut A: SparseMatrix,
    mut B: SparseMatrix,
    mut C: SparseMatrix,
) -> SparseMatrix {
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut b: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut c: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut d: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut m: libc::c_int = 0;
    let mut D: SparseMatrix = 0 as SparseMatrix;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut ib: *mut libc::c_int = (*B).ia;
    let mut jb: *mut libc::c_int = (*B).ja;
    let mut ic: *mut libc::c_int = (*C).ia;
    let mut jc: *mut libc::c_int = (*C).ja;
    let mut id: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    if (*A).format == (*B).format && (*A).format == FORMAT_CSR as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == B->format && A->format == FORMAT_CSR\0" as *const u8
                as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1270 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 78], &[libc::c_char; 78]>(
                b"SparseMatrix SparseMatrix_multiply3(SparseMatrix, SparseMatrix, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    m = (*A).m;
    if (*A).n != (*B).m {
        return 0 as SparseMatrix;
    }
    if (*B).n != (*C).m {
        return 0 as SparseMatrix;
    }
    if (*A).type_0 != (*B).type_0 || (*B).type_0 != (*C).type_0 {
        return 0 as SparseMatrix;
    }
    type_0 = (*A).type_0;
    if type_0 == MATRIX_TYPE_REAL as libc::c_int {
    } else {
        __assert_fail(
            b"type == MATRIX_TYPE_REAL\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1284 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 78], &[libc::c_char; 78]>(
                b"SparseMatrix SparseMatrix_multiply3(SparseMatrix, SparseMatrix, SparseMatrix)\0",
            ))
            .as_ptr(),
        );
    }
    mask = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul((*C).n as size_t),
    ) as *mut libc::c_int;
    if mask.is_null() {
        return 0 as SparseMatrix;
    }
    i = 0 as libc::c_int;
    while i < (*C).n {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            l = *ib.offset(jj as isize);
            while l < *ib.offset((jj + 1 as libc::c_int) as isize) {
                ll = *jb.offset(l as isize);
                k = *ic.offset(ll as isize);
                while k < *ic.offset((ll + 1 as libc::c_int) as isize) {
                    if *mask.offset(*jc.offset(k as isize) as isize) != -i - 2 as libc::c_int {
                        if nz + 1 as libc::c_int <= nz {
                            return 0 as SparseMatrix;
                        }
                        nz += 1;
                        *mask.offset(*jc.offset(k as isize) as isize) = -i - 2 as libc::c_int;
                    }
                    k += 1;
                }
                l += 1;
            }
            j += 1;
        }
        i += 1;
    }
    D = SparseMatrix_new(m, (*C).n, nz, type_0, FORMAT_CSR as libc::c_int);
    if !D.is_null() {
        id = (*D).ia;
        jd = (*D).ja;
        nz = 0 as libc::c_int;
        a = (*A).a as *mut libc::c_double;
        b = (*B).a as *mut libc::c_double;
        c = (*C).a as *mut libc::c_double;
        d = (*D).a as *mut libc::c_double;
        *id.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < m {
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                jj = *ja.offset(j as isize);
                l = *ib.offset(jj as isize);
                while l < *ib.offset((jj + 1 as libc::c_int) as isize) {
                    ll = *jb.offset(l as isize);
                    k = *ic.offset(ll as isize);
                    while k < *ic.offset((ll + 1 as libc::c_int) as isize) {
                        if *mask.offset(*jc.offset(k as isize) as isize) < *id.offset(i as isize) {
                            *mask.offset(*jc.offset(k as isize) as isize) = nz;
                            *jd.offset(nz as isize) = *jc.offset(k as isize);
                            *d.offset(nz as isize) = *a.offset(j as isize)
                                * *b.offset(l as isize)
                                * *c.offset(k as isize);
                            nz += 1;
                        } else {
                            if *jd.offset(*mask.offset(*jc.offset(k as isize) as isize) as isize)
                                == *jc.offset(k as isize)
                            {
                            } else {
                                __assert_fail(
                                    b"jd[mask[jc[k]]] == jc[k]\0" as *const u8
                                        as *const libc::c_char,
                                    b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                    1337 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 78],
                                        &[libc::c_char; 78],
                                    >(
                                        b"SparseMatrix SparseMatrix_multiply3(SparseMatrix, SparseMatrix, SparseMatrix)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            *d.offset(*mask.offset(*jc.offset(k as isize) as isize) as isize) += *a
                                .offset(j as isize)
                                * *b.offset(l as isize)
                                * *c.offset(k as isize);
                        }
                        k += 1;
                    }
                    l += 1;
                }
                j += 1;
            }
            *id.offset((i + 1 as libc::c_int) as isize) = nz;
            i += 1;
        }
        (*D).nz = nz;
    }
    free(mask as *mut libc::c_void);
    return D;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_sum_repeat_entries(
    mut A: SparseMatrix,
    mut what_to_sum: libc::c_int,
) -> SparseMatrix {
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut type_0: libc::c_int = (*A).type_0;
    let mut n: libc::c_int = (*A).n;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sta: libc::c_int = 0;
    if what_to_sum == SUM_REPEATED_NONE as libc::c_int {
        return A;
    }
    mask =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as size_t))
            as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    match type_0 {
        1 => {
            let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
            nz = 0 as libc::c_int;
            sta = *ia.offset(0 as libc::c_int as isize);
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *mask.offset(*ja.offset(j as isize) as isize) < *ia.offset(i as isize) {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        *a.offset(nz as isize) = *a.offset(j as isize);
                        let fresh43 = nz;
                        nz = nz + 1;
                        *mask.offset(*ja.offset(j as isize) as isize) = fresh43;
                    } else {
                        if *ja.offset(*mask.offset(*ja.offset(j as isize) as isize) as isize)
                            == *ja.offset(j as isize)
                        {
                        } else {
                            __assert_fail(
                                b"ja[mask[ja[j]]] == ja[j]\0" as *const u8
                                    as *const libc::c_char,
                                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                1376 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 64],
                                    &[libc::c_char; 64],
                                >(
                                    b"SparseMatrix SparseMatrix_sum_repeat_entries(SparseMatrix, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        *a.offset(*mask.offset(*ja.offset(j as isize) as isize) as isize) +=
                            *a.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
        }
        2 => {
            let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
            if what_to_sum == SUM_REPEATED_ALL as libc::c_int {
                nz = 0 as libc::c_int;
                sta = *ia.offset(0 as libc::c_int as isize);
                i = 0 as libc::c_int;
                while i < (*A).m {
                    j = sta;
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        if *mask.offset(*ja.offset(j as isize) as isize) < *ia.offset(i as isize) {
                            *ja.offset(nz as isize) = *ja.offset(j as isize);
                            *a_0.offset((2 as libc::c_int * nz) as isize) =
                                *a_0.offset((2 as libc::c_int * j) as isize);
                            *a_0.offset((2 as libc::c_int * nz + 1 as libc::c_int) as isize) =
                                *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                            let fresh44 = nz;
                            nz = nz + 1;
                            *mask.offset(*ja.offset(j as isize) as isize) = fresh44;
                        } else {
                            if *ja.offset(*mask.offset(*ja.offset(j as isize) as isize) as isize)
                                == *ja.offset(j as isize)
                            {
                            } else {
                                __assert_fail(
                                    b"ja[mask[ja[j]]] == ja[j]\0" as *const u8
                                        as *const libc::c_char,
                                    b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                    1399 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 64],
                                        &[libc::c_char; 64],
                                    >(
                                        b"SparseMatrix SparseMatrix_sum_repeat_entries(SparseMatrix, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            *a_0.offset(
                                (2 as libc::c_int * *mask.offset(*ja.offset(j as isize) as isize))
                                    as isize,
                            ) += *a_0.offset((2 as libc::c_int * j) as isize);
                            *a_0.offset(
                                (2 as libc::c_int * *mask.offset(*ja.offset(j as isize) as isize)
                                    + 1 as libc::c_int) as isize,
                            ) += *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                        }
                        j += 1;
                    }
                    sta = *ia.offset((i + 1 as libc::c_int) as isize);
                    *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                    i += 1;
                }
            }
        }
        4 => {
            let mut a_1: *mut libc::c_int = (*A).a as *mut libc::c_int;
            nz = 0 as libc::c_int;
            sta = *ia.offset(0 as libc::c_int as isize);
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *mask.offset(*ja.offset(j as isize) as isize) < *ia.offset(i as isize) {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        *a_1.offset(nz as isize) = *a_1.offset(j as isize);
                        let fresh45 = nz;
                        nz = nz + 1;
                        *mask.offset(*ja.offset(j as isize) as isize) = fresh45;
                    } else {
                        if *ja.offset(*mask.offset(*ja.offset(j as isize) as isize) as isize)
                            == *ja.offset(j as isize)
                        {
                        } else {
                            __assert_fail(
                                b"ja[mask[ja[j]]] == ja[j]\0" as *const u8
                                    as *const libc::c_char,
                                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                                1422 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 64],
                                    &[libc::c_char; 64],
                                >(
                                    b"SparseMatrix SparseMatrix_sum_repeat_entries(SparseMatrix, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        *a_1.offset(*mask.offset(*ja.offset(j as isize) as isize) as isize) +=
                            *a_1.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
        }
        8 => {
            nz = 0 as libc::c_int;
            sta = *ia.offset(0 as libc::c_int as isize);
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *mask.offset(*ja.offset(j as isize) as isize) < *ia.offset(i as isize) {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        let fresh46 = nz;
                        nz = nz + 1;
                        *mask.offset(*ja.offset(j as isize) as isize) = fresh46;
                    } else if *ja.offset(*mask.offset(*ja.offset(j as isize) as isize) as isize)
                        == *ja.offset(j as isize)
                    {
                    } else {
                        __assert_fail(
                            b"ja[mask[ja[j]]] == ja[j]\0" as *const u8
                                as *const libc::c_char,
                            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                            1441 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 64],
                                &[libc::c_char; 64],
                            >(
                                b"SparseMatrix SparseMatrix_sum_repeat_entries(SparseMatrix, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
        }
        16 => return 0 as SparseMatrix,
        _ => return 0 as SparseMatrix,
    }
    (*A).nz = nz;
    free(mask as *mut libc::c_void);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_coordinate_form_add_entry(
    mut A: SparseMatrix,
    mut irn: libc::c_int,
    mut jcn: libc::c_int,
    mut val: *mut libc::c_void,
) -> SparseMatrix {
    let mut nz: libc::c_int = 0;
    let mut nzmax: libc::c_int = 0;
    static mut nentries: libc::c_int = 1 as libc::c_int;
    if (*A).format == FORMAT_COORD as libc::c_int {
    } else {
        __assert_fail(
            b"A->format == FORMAT_COORD\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1467 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"SparseMatrix SparseMatrix_coordinate_form_add_entry(SparseMatrix, int, int, void *)\0",
            ))
                .as_ptr(),
        );
    }
    nz = (*A).nz;
    if nz + nentries >= (*A).nzmax {
        nzmax = nz + nentries;
        nzmax = (if 10 as libc::c_int > 0.2f64 as libc::c_int * nzmax {
            10 as libc::c_int
        } else {
            0.2f64 as libc::c_int * nzmax
        }) + nzmax;
        A = SparseMatrix_realloc(A, nzmax);
    }
    *((*A).ia).offset(nz as isize) = irn;
    *((*A).ja).offset(nz as isize) = jcn;
    if (*A).size != 0 {
        memcpy(
            ((*A).a as *mut libc::c_char).offset(
                (nz as size_t)
                    .wrapping_mul((*A).size as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as isize,
            ) as *mut libc::c_void,
            val,
            ((*A).size as libc::c_ulong).wrapping_mul(nentries as size_t),
        );
    }
    if irn >= (*A).m {
        (*A).m = irn + 1 as libc::c_int;
    }
    if jcn >= (*A).n {
        (*A).n = jcn + 1 as libc::c_int;
    }
    (*A).nz += nentries;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_remove_diagonal(mut A: SparseMatrix) -> SparseMatrix {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut sta: libc::c_int = 0;
    if A.is_null() {
        return A;
    }
    nz = 0 as libc::c_int;
    ia = (*A).ia;
    ja = (*A).ja;
    sta = *ia.offset(0 as libc::c_int as isize);
    match (*A).type_0 {
        1 => {
            let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) != i {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        let fresh47 = nz;
                        nz = nz + 1;
                        *a.offset(fresh47 as isize) = *a.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        2 => {
            let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) != i {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        *a_0.offset((2 as libc::c_int * nz) as isize) =
                            *a_0.offset((2 as libc::c_int * j) as isize);
                        *a_0.offset((2 as libc::c_int * nz + 1 as libc::c_int) as isize) =
                            *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                        nz += 1;
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        4 => {
            let mut a_1: *mut libc::c_int = (*A).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) != i {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        let fresh48 = nz;
                        nz = nz + 1;
                        *a_1.offset(fresh48 as isize) = *a_1.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        8 => {
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) != i {
                        let fresh49 = nz;
                        nz = nz + 1;
                        *ja.offset(fresh49 as isize) = *ja.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        16 => return 0 as SparseMatrix,
        _ => return 0 as SparseMatrix,
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_remove_upper(mut A: SparseMatrix) -> SparseMatrix {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut sta: libc::c_int = 0;
    if A.is_null() {
        return A;
    }
    nz = 0 as libc::c_int;
    ia = (*A).ia;
    ja = (*A).ja;
    sta = *ia.offset(0 as libc::c_int as isize);
    match (*A).type_0 {
        1 => {
            let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) < i {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        let fresh50 = nz;
                        nz = nz + 1;
                        *a.offset(fresh50 as isize) = *a.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        2 => {
            let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) < i {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        *a_0.offset((2 as libc::c_int * nz) as isize) =
                            *a_0.offset((2 as libc::c_int * j) as isize);
                        *a_0.offset((2 as libc::c_int * nz + 1 as libc::c_int) as isize) =
                            *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                        nz += 1;
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        4 => {
            let mut a_1: *mut libc::c_int = (*A).a as *mut libc::c_int;
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) < i {
                        *ja.offset(nz as isize) = *ja.offset(j as isize);
                        let fresh51 = nz;
                        nz = nz + 1;
                        *a_1.offset(fresh51 as isize) = *a_1.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        8 => {
            i = 0 as libc::c_int;
            while i < (*A).m {
                j = sta;
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) < i {
                        let fresh52 = nz;
                        nz = nz + 1;
                        *ja.offset(fresh52 as isize) = *ja.offset(j as isize);
                    }
                    j += 1;
                }
                sta = *ia.offset((i + 1 as libc::c_int) as isize);
                *ia.offset((i + 1 as libc::c_int) as isize) = nz;
                i += 1;
            }
            (*A).nz = nz;
        }
        16 => return 0 as SparseMatrix,
        _ => return 0 as SparseMatrix,
    }
    (*A).property &= !(MATRIX_PATTERN_SYMMETRIC as libc::c_int);
    (*A).property &= !(MATRIX_SYMMETRIC as libc::c_int);
    (*A).property &= !(MATRIX_SKEW as libc::c_int);
    (*A).property &= !(MATRIX_HERMITIAN as libc::c_int);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_divide_row_by_degree(mut A: SparseMatrix) -> SparseMatrix {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut deg: libc::c_double = 0.;
    if A.is_null() {
        return A;
    }
    ia = (*A).ia;
    ja = (*A).ja;
    match (*A).type_0 {
        1 => {
            let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).m {
                deg = (*ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize))
                    as libc::c_double;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    *a.offset(j as isize) = *a.offset(j as isize) / deg;
                    j += 1;
                }
                i += 1;
            }
        }
        2 => {
            let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).m {
                deg = (*ia.offset((i + 1 as libc::c_int) as isize) - *ia.offset(i as isize))
                    as libc::c_double;
                j = *ia.offset(i as isize);
                while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                    if *ja.offset(j as isize) != i {
                        *a_0.offset((2 as libc::c_int * j) as isize) =
                            *a_0.offset((2 as libc::c_int * j) as isize) / deg;
                        *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize) =
                            *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize) / deg;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        4 => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                1684 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                    b"SparseMatrix SparseMatrix_divide_row_by_degree(SparseMatrix)\0",
                ))
                .as_ptr(),
            );
        }
        8 => {}
        16 => return 0 as SparseMatrix,
        _ => return 0 as SparseMatrix,
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_get_real_adjacency_matrix_symmetrized(
    mut A: SparseMatrix,
) -> SparseMatrix {
    let mut i: libc::c_int = 0;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    if A.is_null() {
        return A;
    }
    nz = (*A).nz;
    ia = (*A).ia;
    ja = (*A).ja;
    n = (*A).n;
    m = (*A).m;
    if n != m {
        return 0 as SparseMatrix;
    }
    B = SparseMatrix_new(
        m,
        n,
        nz,
        MATRIX_TYPE_PATTERN as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    memcpy(
        (*B).ia as *mut libc::c_void,
        ia as *const libc::c_void,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((m + 1 as libc::c_int) as size_t),
    );
    memcpy(
        (*B).ja as *mut libc::c_void,
        ja as *const libc::c_void,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
    );
    (*B).nz = (*A).nz;
    A = SparseMatrix_symmetrize(B, 1 as libc::c_int != 0);
    SparseMatrix_delete(B);
    A = SparseMatrix_remove_diagonal(A);
    let ref mut fresh53 = (*A).a;
    *fresh53 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul((*A).nz as size_t),
    );
    a = (*A).a as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < (*A).nz {
        *a.offset(i as isize) = 1.0f64;
        i += 1;
    }
    (*A).type_0 = MATRIX_TYPE_REAL as libc::c_int;
    (*A).size = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_apply_fun(
    mut A: SparseMatrix,
    mut fun: Option<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
) -> SparseMatrix {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    if A.is_null() {
        return A;
    }
    if (*A).format != FORMAT_CSR as libc::c_int && (*A).type_0 != MATRIX_TYPE_REAL as libc::c_int {
        return A;
    }
    a = (*A).a as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < (*A).m {
        j = *((*A).ia).offset(i as isize);
        while j < *((*A).ia).offset((i + 1 as libc::c_int) as isize) {
            *a.offset(j as isize) = fun.expect("non-null function pointer")(*a.offset(j as isize));
            j += 1;
        }
        i += 1;
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_copy(mut A: SparseMatrix) -> SparseMatrix {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    if A.is_null() {
        return A;
    }
    B = SparseMatrix_general_new(
        (*A).m,
        (*A).n,
        (*A).nz,
        (*A).type_0,
        (*A).size as size_t,
        (*A).format,
    );
    memcpy(
        (*B).ia as *mut libc::c_void,
        (*A).ia as *const libc::c_void,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(((*A).m + 1 as libc::c_int) as size_t),
    );
    if *((*A).ia).offset((*A).m as isize) != 0 as libc::c_int {
        memcpy(
            (*B).ja as *mut libc::c_void,
            (*A).ja as *const libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(*((*A).ia).offset((*A).m as isize) as size_t),
        );
    }
    if !((*A).a).is_null() {
        memcpy(
            (*B).a,
            (*A).a,
            ((*A).size as libc::c_ulong).wrapping_mul((*A).nz as size_t),
        );
    }
    (*B).property = (*A).property;
    (*B).nz = (*A).nz;
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_has_diagonal(mut A: SparseMatrix) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    i = 0 as libc::c_int;
    while i < m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            if i == *ja.offset(j as isize) {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
            j += 1;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn SparseMatrix_level_sets_internal(
    mut khops: libc::c_int,
    mut A: SparseMatrix,
    mut root: libc::c_int,
    mut nlevel: *mut libc::c_int,
    mut levelset_ptr: *mut *mut libc::c_int,
    mut levelset: *mut *mut libc::c_int,
    mut mask: *mut *mut libc::c_int,
    mut reinitialize_mask: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sta: libc::c_int = 0 as libc::c_int;
    let mut sto: libc::c_int = 1 as libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    if (*levelset_ptr).is_null() {
        *levelset_ptr = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((m + 2 as libc::c_int) as size_t),
        ) as *mut libc::c_int;
    }
    if (*levelset).is_null() {
        *levelset = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(m as size_t),
        ) as *mut libc::c_int;
    }
    if (*mask).is_null() {
        *mask = malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(m as size_t),
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < m {
            *(*mask).offset(i as isize) = UNMASKED as libc::c_int;
            i += 1;
        }
    }
    *nlevel = 0 as libc::c_int;
    if root >= 0 as libc::c_int && root < m {
    } else {
        __assert_fail(
            b"root >= 0 && root < m\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1803 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void SparseMatrix_level_sets_internal(int, SparseMatrix, int, int *, int **, int **, int **, int)\0",
            ))
                .as_ptr(),
        );
    }
    *(*levelset_ptr).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *(*levelset_ptr).offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    *(*levelset).offset(0 as libc::c_int as isize) = root;
    *(*mask).offset(root as isize) = 1 as libc::c_int;
    *nlevel = 1 as libc::c_int;
    nz = 1 as libc::c_int;
    sta = 0 as libc::c_int;
    sto = 1 as libc::c_int;
    while sto > sta && (khops < 0 as libc::c_int || *nlevel <= khops) {
        i = sta;
        while i < sto {
            ii = *(*levelset).offset(i as isize);
            j = *ia.offset(ii as isize);
            while j < *ia.offset((ii + 1 as libc::c_int) as isize) {
                if !(ii == *ja.offset(j as isize)) {
                    if *(*mask).offset(*ja.offset(j as isize) as isize) < 0 as libc::c_int {
                        let fresh54 = nz;
                        nz = nz + 1;
                        *(*levelset).offset(fresh54 as isize) = *ja.offset(j as isize);
                        *(*mask).offset(*ja.offset(j as isize) as isize) =
                            *nlevel + 1 as libc::c_int;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        *nlevel += 1;
        *(*levelset_ptr).offset(*nlevel as isize) = nz;
        sta = sto;
        sto = nz;
    }
    if khops < 0 as libc::c_int || *nlevel <= khops {
        *nlevel -= 1;
    }
    if reinitialize_mask != 0 {
        i = 0 as libc::c_int;
        while i < *(*levelset_ptr).offset(*nlevel as isize) {
            *(*mask).offset(*(*levelset).offset(i as isize) as isize) = UNMASKED as libc::c_int;
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_level_sets(
    mut A: SparseMatrix,
    mut root: libc::c_int,
    mut nlevel: *mut libc::c_int,
    mut levelset_ptr: *mut *mut libc::c_int,
    mut levelset: *mut *mut libc::c_int,
    mut mask: *mut *mut libc::c_int,
    mut reinitialize_mask: libc::c_int,
) {
    let mut khops: libc::c_int = -(1 as libc::c_int);
    return SparseMatrix_level_sets_internal(
        khops,
        A,
        root,
        nlevel,
        levelset_ptr,
        levelset,
        mask,
        reinitialize_mask,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_level_sets_khops(
    mut khops: libc::c_int,
    mut A: SparseMatrix,
    mut root: libc::c_int,
    mut nlevel: *mut libc::c_int,
    mut levelset_ptr: *mut *mut libc::c_int,
    mut levelset: *mut *mut libc::c_int,
    mut mask: *mut *mut libc::c_int,
    mut reinitialize_mask: libc::c_int,
) {
    return SparseMatrix_level_sets_internal(
        khops,
        A,
        root,
        nlevel,
        levelset_ptr,
        levelset,
        mask,
        reinitialize_mask,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_weakly_connected_components(
    mut A0: SparseMatrix,
    mut ncomp: *mut libc::c_int,
    mut comps: *mut *mut libc::c_int,
    mut comps_ptr: *mut *mut libc::c_int,
) {
    let mut A: SparseMatrix = A0;
    let mut levelset_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut levelset: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nlevel: libc::c_int = 0;
    let mut m: libc::c_int = (*A).m;
    let mut i: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    if SparseMatrix_is_symmetric(A, 1 as libc::c_int != 0) == 0 {
        A = SparseMatrix_symmetrize(A, 1 as libc::c_int != 0);
    }
    if (*comps_ptr).is_null() {
        *comps_ptr = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((m + 1 as libc::c_int) as size_t),
        ) as *mut libc::c_int;
    }
    *ncomp = 0 as libc::c_int;
    *(*comps_ptr).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        if i == 0 as libc::c_int || *mask.offset(i as isize) < 0 as libc::c_int {
            SparseMatrix_level_sets(
                A,
                i,
                &mut nlevel,
                &mut levelset_ptr,
                &mut levelset,
                &mut mask,
                0 as libc::c_int,
            );
            if i == 0 as libc::c_int {
                *comps = levelset;
            }
            nn = *levelset_ptr.offset(nlevel as isize);
            levelset = levelset.offset(nn as isize);
            *(*comps_ptr).offset((*ncomp + 1 as libc::c_int) as isize) =
                *(*comps_ptr).offset(*ncomp as isize) + nn;
            *ncomp += 1;
        }
        i += 1;
    }
    if A != A0 {
        SparseMatrix_delete(A);
    }
    free(levelset_ptr as *mut libc::c_void);
    free(mask as *mut libc::c_void);
}
unsafe extern "C" fn cmp(mut i: *mut libc::c_void, mut j: *mut libc::c_void) -> libc::c_int {
    let mut d1: nodedata = 0 as *mut nodedata_struct;
    let mut d2: nodedata = 0 as *mut nodedata_struct;
    d1 = i as nodedata;
    d2 = j as nodedata;
    if (*d1).dist > (*d2).dist {
        return 1 as libc::c_int;
    }
    if (*d1).dist < (*d2).dist {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Dijkstra_internal(
    mut A: SparseMatrix,
    mut root: libc::c_int,
    mut dist: *mut libc::c_double,
    mut nlist: *mut libc::c_int,
    mut list: *mut libc::c_int,
    mut dist_max: *mut libc::c_double,
    mut mask: *mut libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*A).m;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut heap_id: libc::c_int = 0;
    let mut h: BinaryHeap = 0 as *mut BinaryHeap_struct;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut aa: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ai: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ndata: nodedata = 0 as *mut nodedata_struct;
    let mut ndata_min: nodedata = 0 as *mut nodedata_struct;
    let mut heap_ids: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    if SparseMatrix_is_symmetric(A, 1 as libc::c_int != 0) != 0 {
    } else {
        __assert_fail(
            b"SparseMatrix_is_symmetric(A, true)\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1921 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"int Dijkstra_internal(SparseMatrix, int, double *, int *, int *, double *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    if m == (*A).n {
    } else {
        __assert_fail(
            b"m == A->n\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1923 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"int Dijkstra_internal(SparseMatrix, int, double *, int *, int *, double *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    match (*A).type_0 {
        2 => {
            aa = (*A).a as *mut libc::c_double;
            a = gmalloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul((*A).nz as size_t),
            ) as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                *a.offset(i as isize) = *aa.offset((i * 2 as libc::c_int) as isize);
                i += 1;
            }
        }
        1 => {
            a = (*A).a as *mut libc::c_double;
        }
        4 => {
            ai = (*A).a as *mut libc::c_int;
            a = gmalloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul((*A).nz as size_t),
            ) as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                *a.offset(i as isize) = *ai.offset(i as isize) as libc::c_double;
                i += 1;
            }
        }
        8 => {
            a = gmalloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul((*A).nz as size_t),
            ) as *mut libc::c_double;
            i = 0 as libc::c_int;
            while i < (*A).nz {
                *a.offset(i as isize) = 1.0f64;
                i += 1;
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                1944 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"int Dijkstra_internal(SparseMatrix, int, double *, int *, int *, double *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    heap_ids =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(m as size_t))
            as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *dist.offset(i as isize) = -(1 as libc::c_int) as libc::c_double;
        *heap_ids.offset(i as isize) = UNVISITED as libc::c_int;
        i += 1;
    }
    h = BinaryHeap_new(Some(
        cmp as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    ));
    if !h.is_null() {
    } else {
        __assert_fail(
            b"h\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1954 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"int Dijkstra_internal(SparseMatrix, int, double *, int *, int *, double *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    ndata = gmalloc(::std::mem::size_of::<nodedata_struct>() as libc::c_ulong) as nodedata;
    (*ndata).dist = 0 as libc::c_int as libc::c_double;
    (*ndata).id = root;
    *heap_ids.offset(root as isize) = BinaryHeap_insert(h, ndata as *mut libc::c_void);
    if *heap_ids.offset(root as isize) >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"heap_ids[root] >= 0\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            1962 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"int Dijkstra_internal(SparseMatrix, int, double *, int *, int *, double *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    loop {
        ndata_min = BinaryHeap_extract_min(h) as nodedata;
        if ndata_min.is_null() {
            break;
        }
        i = (*ndata_min).id;
        *dist.offset(i as isize) = (*ndata_min).dist;
        let fresh55 = found;
        found = found + 1;
        *list.offset(fresh55 as isize) = i;
        *heap_ids.offset(i as isize) = FINISHED as libc::c_int;
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            jj = *ja.offset(j as isize);
            heap_id = *heap_ids.offset(jj as isize);
            if !(jj == i
                || heap_id == FINISHED as libc::c_int
                || !mask.is_null() && *mask.offset(jj as isize) < 0 as libc::c_int)
            {
                if heap_id == UNVISITED as libc::c_int {
                    ndata = gmalloc(::std::mem::size_of::<nodedata_struct>() as libc::c_ulong)
                        as nodedata;
                    (*ndata).dist = fabs(*a.offset(j as isize)) + (*ndata_min).dist;
                    (*ndata).id = jj;
                    *heap_ids.offset(jj as isize) =
                        BinaryHeap_insert(h, ndata as *mut libc::c_void);
                } else {
                    ndata = BinaryHeap_get_item(h, heap_id) as nodedata;
                    (*ndata).dist =
                        if (*ndata).dist < fabs(*a.offset(j as isize)) + (*ndata_min).dist {
                            (*ndata).dist
                        } else {
                            fabs(*a.offset(j as isize)) + (*ndata_min).dist
                        };
                    if (*ndata).id == jj {
                    } else {
                        __assert_fail(
                            b"ndata->id == jj\0" as *const u8 as *const libc::c_char,
                            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                            1986 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 82],
                                &[libc::c_char; 82],
                            >(
                                b"int Dijkstra_internal(SparseMatrix, int, double *, int *, int *, double *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    BinaryHeap_reset(h, heap_id, ndata as *mut libc::c_void);
                }
            }
            j += 1;
        }
        free(ndata_min as *mut libc::c_void);
    }
    *nlist = found;
    *dist_max = *dist.offset(i as isize);
    BinaryHeap_delete(
        h,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    free(heap_ids as *mut libc::c_void);
    if !a.is_null() && a != (*A).a as *mut libc::c_double {
        free(a as *mut libc::c_void);
    }
    if found == m || !mask.is_null() {
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn Dijkstra(
    mut A: SparseMatrix,
    mut root: libc::c_int,
    mut dist: *mut libc::c_double,
    mut nlist: *mut libc::c_int,
    mut list: *mut libc::c_int,
    mut dist_max: *mut libc::c_double,
) -> libc::c_int {
    return Dijkstra_internal(A, root, dist, nlist, list, dist_max, 0 as *mut libc::c_int);
}
unsafe extern "C" fn Dijkstra_masked(
    mut A: SparseMatrix,
    mut root: libc::c_int,
    mut dist: *mut libc::c_double,
    mut nlist: *mut libc::c_int,
    mut list: *mut libc::c_int,
    mut dist_max: *mut libc::c_double,
    mut mask: *mut libc::c_int,
) -> libc::c_int {
    return Dijkstra_internal(A, root, dist, nlist, list, dist_max, mask);
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_decompose_to_supervariables(
    mut A: SparseMatrix,
    mut ncluster: *mut libc::c_int,
    mut cluster: *mut *mut libc::c_int,
    mut clusterp: *mut *mut libc::c_int,
) {
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut n: libc::c_int = (*A).n;
    let mut m: libc::c_int = (*A).m;
    let mut super_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nsuper: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut isup: libc::c_int = 0;
    let mut newmap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut isuper: libc::c_int = 0;
    super_0 =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as size_t))
            as *mut libc::c_int;
    nsuper = gmalloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((n + 1 as libc::c_int) as size_t),
    ) as *mut libc::c_int;
    mask =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as size_t))
            as *mut libc::c_int;
    newmap =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as size_t))
            as *mut libc::c_int;
    nsuper = nsuper.offset(1);
    isup = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *super_0.offset(i as isize) = isup;
        i += 1;
    }
    *nsuper.offset(0 as libc::c_int as isize) = n;
    i = 0 as libc::c_int;
    while i < n {
        *mask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    isup += 1;
    i = 0 as libc::c_int;
    while i < m {
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            isuper = *super_0.offset(*ja.offset(j as isize) as isize);
            let ref mut fresh56 = *nsuper.offset(isuper as isize);
            *fresh56 -= 1;
            j += 1;
        }
        j = *ia.offset(i as isize);
        while j < *ia.offset((i + 1 as libc::c_int) as isize) {
            isuper = *super_0.offset(*ja.offset(j as isize) as isize);
            if *mask.offset(isuper as isize) < i {
                *mask.offset(isuper as isize) = i;
                if *nsuper.offset(isuper as isize) == 0 as libc::c_int {
                    *nsuper.offset(isuper as isize) = 1 as libc::c_int;
                    *newmap.offset(isuper as isize) = isuper;
                } else {
                    *newmap.offset(isuper as isize) = isup;
                    *nsuper.offset(isup as isize) = 1 as libc::c_int;
                    let fresh57 = isup;
                    isup = isup + 1;
                    *super_0.offset(*ja.offset(j as isize) as isize) = fresh57;
                }
            } else {
                *super_0.offset(*ja.offset(j as isize) as isize) = *newmap.offset(isuper as isize);
                let ref mut fresh58 = *nsuper.offset(*newmap.offset(isuper as isize) as isize);
                *fresh58 += 1;
            }
            j += 1;
        }
        i += 1;
    }
    nsuper = nsuper.offset(-1);
    *nsuper.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < isup {
        *nsuper.offset((i + 1 as libc::c_int) as isize) += *nsuper.offset(i as isize);
        i += 1;
    }
    *cluster = newmap;
    i = 0 as libc::c_int;
    while i < n {
        isuper = *super_0.offset(i as isize);
        let ref mut fresh59 = *nsuper.offset(isuper as isize);
        let fresh60 = *fresh59;
        *fresh59 = *fresh59 + 1;
        *(*cluster).offset(fresh60 as isize) = i;
        i += 1;
    }
    i = isup;
    while i > 0 as libc::c_int {
        *nsuper.offset(i as isize) = *nsuper.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
    }
    *nsuper.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *clusterp = nsuper;
    *ncluster = isup;
    free(mask as *mut libc::c_void);
    free(super_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_get_augmented(mut A: SparseMatrix) -> SparseMatrix {
    let mut irn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jcn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nz: libc::c_int = (*A).nz;
    let mut type_0: libc::c_int = (*A).type_0;
    let mut m: libc::c_int = (*A).m;
    let mut n: libc::c_int = (*A).n;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut B: SparseMatrix = 0 as SparseMatrix;
    if A.is_null() {
        return 0 as SparseMatrix;
    }
    if nz > 0 as libc::c_int {
        irn = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nz as size_t)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        jcn = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nz as size_t)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    if !((*A).a).is_null() {
        if (*A).size != 0 as libc::c_int && nz > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"A->size != 0 && nz > 0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                2131 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"SparseMatrix SparseMatrix_get_augmented(SparseMatrix)\0",
                ))
                .as_ptr(),
            );
        }
        val = gmalloc((((*A).size * 2 as libc::c_int) as libc::c_ulong).wrapping_mul(nz as size_t));
        memcpy(
            val,
            (*A).a,
            ((*A).size as libc::c_ulong).wrapping_mul(nz as size_t),
        );
        memcpy(
            (val as *mut libc::c_char)
                .offset((nz as size_t).wrapping_mul((*A).size as libc::c_ulong) as isize)
                as *mut libc::c_void,
            (*A).a,
            ((*A).size as libc::c_ulong).wrapping_mul(nz as size_t),
        );
    }
    nz = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        j = *((*A).ia).offset(i as isize);
        while j < *((*A).ia).offset((i + 1 as libc::c_int) as isize) {
            *irn.offset(nz as isize) = i;
            let fresh61 = nz;
            nz = nz + 1;
            *jcn.offset(fresh61 as isize) = *((*A).ja).offset(j as isize) + m;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < m {
        j = *((*A).ia).offset(i as isize);
        while j < *((*A).ia).offset((i + 1 as libc::c_int) as isize) {
            *jcn.offset(nz as isize) = i;
            let fresh62 = nz;
            nz = nz + 1;
            *irn.offset(fresh62 as isize) = *((*A).ja).offset(j as isize) + m;
            j += 1;
        }
        i += 1;
    }
    B = SparseMatrix_from_coordinate_arrays(
        nz,
        m + n,
        m + n,
        irn,
        jcn,
        val,
        type_0,
        (*A).size as size_t,
    );
    (*B).property = (*B).property | MATRIX_SYMMETRIC as libc::c_int;
    (*B).property = (*B).property | MATRIX_PATTERN_SYMMETRIC as libc::c_int;
    free(irn as *mut libc::c_void);
    free(jcn as *mut libc::c_void);
    free(val);
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_to_square_matrix(
    mut A: SparseMatrix,
    mut bipartite_options: libc::c_int,
) -> SparseMatrix {
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    match bipartite_options {
        0 => {
            if (*A).m == (*A).n {
                return A;
            }
        }
        1 => {
            if (*A).m == (*A).n && SparseMatrix_is_symmetric(A, 1 as libc::c_int != 0) != 0 {
                return A;
            }
        }
        2 => {
            if (*A).m == (*A).n && SparseMatrix_is_symmetric(A, 0 as libc::c_int != 0) != 0 {
                return A;
            }
        }
        3 => {}
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                2175 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                    b"SparseMatrix SparseMatrix_to_square_matrix(SparseMatrix, int)\0",
                ))
                .as_ptr(),
            );
        }
    }
    B = SparseMatrix_get_augmented(A);
    SparseMatrix_delete(A);
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_get_submatrix(
    mut A: SparseMatrix,
    mut nrow: libc::c_int,
    mut ncol: libc::c_int,
    mut rindices: *mut libc::c_int,
    mut cindices: *mut libc::c_int,
) -> SparseMatrix {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut irn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut jcn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ia: *mut libc::c_int = (*A).ia;
    let mut ja: *mut libc::c_int = (*A).ja;
    let mut m: libc::c_int = (*A).m;
    let mut n: libc::c_int = (*A).n;
    let mut cmask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rmask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut v: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut B: SparseMatrix = 0 as SparseMatrix;
    let mut irow: libc::c_int = 0 as libc::c_int;
    let mut icol: libc::c_int = 0 as libc::c_int;
    if nrow <= 0 as libc::c_int || ncol <= 0 as libc::c_int {
        return 0 as SparseMatrix;
    }
    rmask =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(m as size_t))
            as *mut libc::c_int;
    cmask =
        gmalloc((::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(n as size_t))
            as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < m {
        *rmask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n {
        *cmask.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    if !rindices.is_null() {
        i = 0 as libc::c_int;
        while i < nrow {
            if *rindices.offset(i as isize) >= 0 as libc::c_int && *rindices.offset(i as isize) < m
            {
                let fresh63 = irow;
                irow = irow + 1;
                *rmask.offset(*rindices.offset(i as isize) as isize) = fresh63;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < nrow {
            let fresh64 = irow;
            irow = irow + 1;
            *rmask.offset(i as isize) = fresh64;
            i += 1;
        }
    }
    if !cindices.is_null() {
        i = 0 as libc::c_int;
        while i < ncol {
            if *cindices.offset(i as isize) >= 0 as libc::c_int && *cindices.offset(i as isize) < n
            {
                let fresh65 = icol;
                icol = icol + 1;
                *cmask.offset(*cindices.offset(i as isize) as isize) = fresh65;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < ncol {
            let fresh66 = icol;
            icol = icol + 1;
            *cmask.offset(i as isize) = fresh66;
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < m {
        if !(*rmask.offset(i as isize) < 0 as libc::c_int) {
            j = *ia.offset(i as isize);
            while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                if !(*cmask.offset(*ja.offset(j as isize) as isize) < 0 as libc::c_int) {
                    nz += 1;
                }
                j += 1;
            }
        }
        i += 1;
    }
    match (*A).type_0 {
        1 => {
            let mut a: *mut libc::c_double = (*A).a as *mut libc::c_double;
            let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
            irn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            jcn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            val = gmalloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(nz as size_t),
            ) as *mut libc::c_double;
            nz = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < m {
                if !(*rmask.offset(i as isize) < 0 as libc::c_int) {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        if !(*cmask.offset(*ja.offset(j as isize) as isize) < 0 as libc::c_int) {
                            *irn.offset(nz as isize) = *rmask.offset(i as isize);
                            *jcn.offset(nz as isize) =
                                *cmask.offset(*ja.offset(j as isize) as isize);
                            let fresh67 = nz;
                            nz = nz + 1;
                            *val.offset(fresh67 as isize) = *a.offset(j as isize);
                        }
                        j += 1;
                    }
                }
                i += 1;
            }
            v = val as *mut libc::c_void;
        }
        2 => {
            let mut a_0: *mut libc::c_double = (*A).a as *mut libc::c_double;
            let mut val_0: *mut libc::c_double = 0 as *mut libc::c_double;
            irn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            jcn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            val_0 = gmalloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(nz as size_t),
            ) as *mut libc::c_double;
            nz = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < m {
                if !(*rmask.offset(i as isize) < 0 as libc::c_int) {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        if !(*cmask.offset(*ja.offset(j as isize) as isize) < 0 as libc::c_int) {
                            *irn.offset(nz as isize) = *rmask.offset(i as isize);
                            *jcn.offset(nz as isize) =
                                *cmask.offset(*ja.offset(j as isize) as isize);
                            *val_0.offset((2 as libc::c_int * nz) as isize) =
                                *a_0.offset((2 as libc::c_int * j) as isize);
                            *val_0.offset((2 as libc::c_int * nz + 1 as libc::c_int) as isize) =
                                *a_0.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                            nz += 1;
                        }
                        j += 1;
                    }
                }
                i += 1;
            }
            v = val_0 as *mut libc::c_void;
        }
        4 => {
            let mut a_1: *mut libc::c_int = (*A).a as *mut libc::c_int;
            let mut val_1: *mut libc::c_int = 0 as *mut libc::c_int;
            irn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            jcn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            val_1 = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            nz = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < m {
                if !(*rmask.offset(i as isize) < 0 as libc::c_int) {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        if !(*cmask.offset(*ja.offset(j as isize) as isize) < 0 as libc::c_int) {
                            *irn.offset(nz as isize) = *rmask.offset(i as isize);
                            *jcn.offset(nz as isize) =
                                *cmask.offset(*ja.offset(j as isize) as isize);
                            *val_1.offset(nz as isize) = *a_1.offset(j as isize);
                            nz += 1;
                        }
                        j += 1;
                    }
                }
                i += 1;
            }
            v = val_1 as *mut libc::c_void;
        }
        8 => {
            irn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            jcn = gmalloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(nz as size_t),
            ) as *mut libc::c_int;
            nz = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < m {
                if !(*rmask.offset(i as isize) < 0 as libc::c_int) {
                    j = *ia.offset(i as isize);
                    while j < *ia.offset((i + 1 as libc::c_int) as isize) {
                        if !(*cmask.offset(*ja.offset(j as isize) as isize) < 0 as libc::c_int) {
                            *irn.offset(nz as isize) = *rmask.offset(i as isize);
                            let fresh68 = nz;
                            nz = nz + 1;
                            *jcn.offset(fresh68 as isize) =
                                *cmask.offset(*ja.offset(j as isize) as isize);
                        }
                        j += 1;
                    }
                }
                i += 1;
            }
        }
        16 => {
            free(rmask as *mut libc::c_void);
            free(cmask as *mut libc::c_void);
            return 0 as SparseMatrix;
        }
        _ => {
            free(rmask as *mut libc::c_void);
            free(cmask as *mut libc::c_void);
            return 0 as SparseMatrix;
        }
    }
    B = SparseMatrix_from_coordinate_arrays(
        nz,
        nrow,
        ncol,
        irn,
        jcn,
        v,
        (*A).type_0,
        (*A).size as size_t,
    );
    free(cmask as *mut libc::c_void);
    free(rmask as *mut libc::c_void);
    free(irn as *mut libc::c_void);
    free(jcn as *mut libc::c_void);
    if !v.is_null() {
        free(v);
    }
    return B;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_set_entries_to_real_one(mut A: SparseMatrix) -> SparseMatrix {
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    free((*A).a);
    let ref mut fresh69 = (*A).a;
    *fresh69 = gmalloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul((*A).nz as size_t),
    );
    a = (*A).a as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < (*A).nz {
        *a.offset(i as isize) = 1.0f64;
        i += 1;
    }
    (*A).type_0 = MATRIX_TYPE_REAL as libc::c_int;
    (*A).size = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_from_dense(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut x: *mut libc::c_double,
) -> SparseMatrix {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut A: SparseMatrix = SparseMatrix_new(
        m,
        n,
        m * n,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_CSR as libc::c_int,
    );
    *((*A).ia).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        *((*A).ia).offset(i as isize) = *((*A).ia).offset((i - 1 as libc::c_int) as isize) + n;
        i += 1;
    }
    ja = (*A).ja;
    a = (*A).a as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < m {
        j = 0 as libc::c_int;
        while j < n {
            *ja.offset(j as isize) = j;
            *a.offset(j as isize) = *x.offset((i * n + j) as isize);
            j += 1;
        }
        ja = ja.offset(n as isize);
        a = a.offset(j as isize);
        i += 1;
    }
    (*A).nz = m * n;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_distance_matrix(
    mut D0: SparseMatrix,
    mut weighted: libc::c_int,
    mut dist0: *mut *mut libc::c_double,
) -> libc::c_int {
    let mut D: SparseMatrix = D0;
    let mut m: libc::c_int = (*D).m;
    let mut n: libc::c_int = (*D).n;
    let mut levelset_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut levelset: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dist: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nlist: libc::c_int = 0;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nlevel: libc::c_int = 0;
    let mut dmax: libc::c_double = 0.;
    if SparseMatrix_is_symmetric(D, 0 as libc::c_int != 0) == 0 {
        D = SparseMatrix_symmetrize(D, 0 as libc::c_int != 0);
    }
    if m == n {
    } else {
        __assert_fail(
            b"m == n\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            2396 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"int SparseMatrix_distance_matrix(SparseMatrix, int, double **)\0",
            ))
            .as_ptr(),
        );
    }
    if (*dist0).is_null() {
        *dist0 = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < n * n {
        *(*dist0).offset(i as isize) = -(1 as libc::c_int) as libc::c_double;
        i += 1;
    }
    if weighted == 0 {
        k = 0 as libc::c_int;
        while k < n {
            SparseMatrix_level_sets(
                D,
                k,
                &mut nlevel,
                &mut levelset_ptr,
                &mut levelset,
                &mut mask,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            if *levelset_ptr.offset(nlevel as isize) == n {
            } else {
                __assert_fail(
                    b"levelset_ptr[nlevel] == n\0" as *const u8 as *const libc::c_char,
                    b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                    2404 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                        b"int SparseMatrix_distance_matrix(SparseMatrix, int, double **)\0",
                    ))
                    .as_ptr(),
                );
            }
            i = 0 as libc::c_int;
            while i < nlevel {
                j = *levelset_ptr.offset(i as isize);
                while j < *levelset_ptr.offset((i + 1 as libc::c_int) as isize) {
                    *(*dist0).offset((k * n + *levelset.offset(j as isize)) as isize) =
                        i as libc::c_double;
                    j += 1;
                }
                i += 1;
            }
            k += 1;
        }
    } else {
        list = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_int;
        k = 0 as libc::c_int;
        while k < n {
            dist = &mut *(*dist0).offset((k * n) as isize) as *mut libc::c_double;
            flag = Dijkstra(D, k, dist, &mut nlist, list, &mut dmax);
            k += 1;
        }
    }
    free(levelset_ptr as *mut libc::c_void);
    free(levelset as *mut libc::c_void);
    free(mask as *mut libc::c_void);
    if D != D0 {
        SparseMatrix_delete(D);
    }
    free(list as *mut libc::c_void);
    return flag;
}
#[no_mangle]
pub unsafe extern "C" fn SparseMatrix_distance_matrix_khops(
    mut khops: libc::c_int,
    mut D0: SparseMatrix,
    mut weighted: libc::c_int,
) -> SparseMatrix {
    let mut D: SparseMatrix = D0;
    let mut B: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut C: SparseMatrix = 0 as *mut SparseMatrix_struct;
    let mut m: libc::c_int = (*D).m;
    let mut n: libc::c_int = (*D).n;
    let mut levelset_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut levelset: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dist: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nlist: libc::c_int = 0;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut itmp: libc::c_int = 0;
    let mut nlevel: libc::c_int = 0;
    let mut dmax: libc::c_double = 0.;
    let mut dtmp: libc::c_double = 0.;
    if SparseMatrix_is_symmetric(D, 0 as libc::c_int != 0) == 0 {
        D = SparseMatrix_symmetrize(D, 0 as libc::c_int != 0);
    }
    if m == n {
    } else {
        __assert_fail(
            b"m == n\0" as *const u8 as *const libc::c_char,
            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
            2450 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 72], &[libc::c_char; 72]>(
                b"SparseMatrix SparseMatrix_distance_matrix_khops(int, SparseMatrix, int)\0",
            ))
            .as_ptr(),
        );
    }
    B = SparseMatrix_new(
        n,
        n,
        1 as libc::c_int,
        MATRIX_TYPE_REAL as libc::c_int,
        FORMAT_COORD as libc::c_int,
    );
    if weighted == 0 {
        k = 0 as libc::c_int;
        while k < n {
            SparseMatrix_level_sets_khops(
                khops,
                D,
                k,
                &mut nlevel,
                &mut levelset_ptr,
                &mut levelset,
                &mut mask,
                (0 as libc::c_int == 0) as libc::c_int,
            );
            i = 0 as libc::c_int;
            while i < nlevel {
                j = *levelset_ptr.offset(i as isize);
                while j < *levelset_ptr.offset((i + 1 as libc::c_int) as isize) {
                    itmp = *levelset.offset(j as isize);
                    dtmp = i as libc::c_double;
                    if k != itmp {
                        B = SparseMatrix_coordinate_form_add_entry(
                            B,
                            k,
                            itmp,
                            &mut dtmp as *mut libc::c_double as *mut libc::c_void,
                        );
                    }
                    j += 1;
                }
                i += 1;
            }
            k += 1;
        }
    } else {
        list = gmalloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_int;
        dist = gmalloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_double;
        k = 0 as libc::c_int;
        while k < n {
            SparseMatrix_level_sets_khops(
                khops,
                D,
                k,
                &mut nlevel,
                &mut levelset_ptr,
                &mut levelset,
                &mut mask,
                0 as libc::c_int,
            );
            if nlevel - 1 as libc::c_int <= khops {
            } else {
                __assert_fail(
                    b"nlevel-1 <= khops\0" as *const u8 as *const libc::c_char,
                    b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                    2484 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"SparseMatrix SparseMatrix_distance_matrix_khops(int, SparseMatrix, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            flag = Dijkstra_masked(D, k, dist, &mut nlist, list, &mut dmax, mask);
            if flag == 0 {
            } else {
                __assert_fail(
                    b"!flag\0" as *const u8 as *const libc::c_char,
                    b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                    2486 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"SparseMatrix SparseMatrix_distance_matrix_khops(int, SparseMatrix, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            i = 0 as libc::c_int;
            while i < nlevel {
                j = *levelset_ptr.offset(i as isize);
                while j < *levelset_ptr.offset((i + 1 as libc::c_int) as isize) {
                    if *mask.offset(*levelset.offset(j as isize) as isize) == i + 1 as libc::c_int {
                    } else {
                        __assert_fail(
                            b"mask[levelset[j]] == i+1\0" as *const u8
                                as *const libc::c_char,
                            b"SparseMatrix.c\0" as *const u8 as *const libc::c_char,
                            2489 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"SparseMatrix SparseMatrix_distance_matrix_khops(int, SparseMatrix, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *mask.offset(*levelset.offset(j as isize) as isize) = -(1 as libc::c_int);
                    j += 1;
                }
                i += 1;
            }
            j = 0 as libc::c_int;
            while j < nlist {
                itmp = *list.offset(j as isize);
                dtmp = *dist.offset(itmp as isize);
                if k != itmp {
                    B = SparseMatrix_coordinate_form_add_entry(
                        B,
                        k,
                        itmp,
                        &mut dtmp as *mut libc::c_double as *mut libc::c_void,
                    );
                }
                j += 1;
            }
            k += 1;
        }
    }
    C = SparseMatrix_from_coordinate_format(B);
    SparseMatrix_delete(B);
    free(levelset_ptr as *mut libc::c_void);
    free(levelset as *mut libc::c_void);
    free(mask as *mut libc::c_void);
    free(dist as *mut libc::c_void);
    if D != D0 {
        SparseMatrix_delete(D);
    }
    free(list as *mut libc::c_void);
    D = SparseMatrix_symmetrize(C, 0 as libc::c_int != 0);
    SparseMatrix_delete(C);
    return D;
}
